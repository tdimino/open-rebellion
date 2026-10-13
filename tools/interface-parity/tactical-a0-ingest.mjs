import { createHash } from "node:crypto";
import fs from "node:fs";
import path from "node:path";
import { PNG } from "pngjs";
import {
  EXPECTED_ORIGINAL_EXE_SHA256,
  a0Coverage,
  collectA0Cells,
} from "./tactical-matrix.mjs";

const fail = (message) => {
  throw new Error(message);
};

const assert = (condition, message) => {
  if (!condition) fail(message);
};

const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");

// The guest stamps frames and this Mac stamps the recording's start; the
// two clocks agreed within a few seconds on 2026-10-12.
const AUDIO_CLOCK_TOLERANCE_MS = 10_000;
const AUDIO_MIN_SECONDS = 10;

// RIFF/WAVE, PCM 16-bit: the format QEMU's wavcapture writes.
function readWav(bytes, label) {
  assert(
    bytes.length >= 12 && bytes.toString("ascii", 0, 4) === "RIFF" &&
      bytes.toString("ascii", 8, 12) === "WAVE",
    `${label} is not a RIFF/WAVE file`,
  );
  let format = null;
  let data = null;
  for (let offset = 12; offset + 8 <= bytes.length;) {
    const id = bytes.toString("ascii", offset, offset + 4);
    const size = bytes.readUInt32LE(offset + 4);
    const body = offset + 8;
    if (id === "fmt ") {
      format = {
        audio_format: bytes.readUInt16LE(body),
        channels: bytes.readUInt16LE(body + 2),
        sample_rate: bytes.readUInt32LE(body + 4),
        bits_per_sample: bytes.readUInt16LE(body + 14),
      };
    } else if (id === "data") {
      data = bytes.subarray(body, Math.min(body + size, bytes.length));
    }
    offset = body + size + (size % 2);
  }
  assert(format && data, `${label} has no fmt or data chunk`);
  assert(
    format.audio_format === 1 && format.bits_per_sample === 16 &&
      format.channels >= 1 && format.sample_rate > 0,
    `${label} is not 16-bit PCM`,
  );
  const frameBytes = format.channels * 2;
  const duration = data.length / frameBytes / format.sample_rate;
  let audible = false;
  for (let index = 0; index + 1 < data.length; index += 2) {
    if (data.readInt16LE(index) !== 0) { audible = true; break; }
  }
  return { ...format, duration_s: duration, audible };
}

export function buildCaptureRecord({
  metadata, pngBytes, cells, before = null, burstFrames = null, cellFrame = null, audio = null,
}) {
  const cell = cells.find(({ id }) => id === metadata.cell_id);
  assert(cell, `capture maps unknown tactical cell ${metadata.cell_id}`);
  assert(
    metadata.requirement === cell.requirement,
    `${metadata.cell_id} requirement does not match the surface ledger`,
  );
  assert(metadata.process === "REBEXE", `${metadata.cell_id} was not captured from REBEXE`);
  assert(
    metadata.executable_sha256 === EXPECTED_ORIGINAL_EXE_SHA256,
    `${metadata.cell_id} executable hash does not match the audited original`,
  );
  assert(metadata.width === 640 && metadata.height === 480, `${metadata.cell_id} is not 640x480`);
  assert(
    typeof metadata.windows_build === "string" && metadata.windows_build.length > 0,
    `${metadata.cell_id} has no Windows build provenance`,
  );
  assert(
    typeof metadata.method === "string" && metadata.method.length > 0,
    `${metadata.cell_id} has no capture method`,
  );
  assert(
    Number.isFinite(Date.parse(metadata.captured_at_utc)),
    `${metadata.cell_id} has an invalid capture timestamp`,
  );
  assert(
    metadata.state_setup && typeof metadata.state_setup === "object",
    `${metadata.cell_id} has no state setup`,
  );
  assert(
    ["new-game", "original-save", "editor-assisted-save", "live-journey"]
      .includes(metadata.state_setup.kind),
    `${metadata.cell_id} has an invalid state setup kind`,
  );
  assert(
    typeof metadata.state_setup.identifier === "string" &&
      metadata.state_setup.identifier.length > 0,
    `${metadata.cell_id} has no state setup identifier`,
  );
  assert(
    typeof metadata.state_setup.notes === "string",
    `${metadata.cell_id} has invalid state setup notes`,
  );
  assert(
    Array.isArray(metadata.input_trace) &&
      metadata.input_trace.length > 0 &&
      metadata.input_trace.every((step) => typeof step === "string" && step.length > 0),
    `${metadata.cell_id} has no ordered input trace`,
  );

  const decoded = PNG.sync.read(pngBytes);
  assert(
    decoded.width === 640 && decoded.height === 480,
    `${metadata.cell_id} PNG is ${decoded.width}x${decoded.height}, expected 640x480`,
  );
  const digest = sha256(pngBytes);
  assert(digest === metadata.sha256, `${metadata.cell_id} PNG hash disagrees with its sidecar`);

  const artifacts = [{
    kind: "visual_png",
    path: `.artifacts/interface-parity/a0/${cell.id}/original.png`,
    sha256: digest,
    mime_type: "image/png",
    width: decoded.width,
    height: decoded.height,
  }];
  // A cell about a transition (entry from the Battle Alert, a click's
  // effect) pairs its frame with the one before it, from the same setup.
  if (before) {
    const label = `${metadata.cell_id} before-frame`;
    const prior = before.metadata;
    assert(prior.process === "REBEXE", `${label} was not captured from REBEXE`);
    assert(
      prior.executable_sha256 === EXPECTED_ORIGINAL_EXE_SHA256,
      `${label} executable hash does not match the audited original`,
    );
    assert(
      prior.state_setup?.identifier === metadata.state_setup.identifier,
      `${label} comes from a different state setup`,
    );
    assert(
      Date.parse(prior.captured_at_utc) < Date.parse(metadata.captured_at_utc),
      `${label} was not captured before the cell's frame`,
    );
    const priorPng = PNG.sync.read(before.pngBytes);
    assert(priorPng.width === 640 && priorPng.height === 480, `${label} is not 640x480`);
    const priorDigest = sha256(before.pngBytes);
    assert(priorDigest === prior.sha256, `${label} PNG hash disagrees with its sidecar`);
    artifacts.push({
      kind: "visual_png",
      path: `.artifacts/interface-parity/a0/${cell.id}/before.png`,
      sha256: priorDigest,
      mime_type: "image/png",
      width: 640,
      height: 480,
    });
  }
  // A cell about motion carries a burst: frames at a fixed cadence, each
  // rebuilt exactly but none held still, with their offsets in burst.json.
  // The cell's own frame is the burst's last unless cellFrame names another:
  // an event the original holds for only a few seconds (a kill, before the
  // results screen) lands at a frame known only once the burst is seen.
  const sources = artifacts.map(() => null);
  assert(
    Boolean(metadata.burst) === (burstFrames !== null),
    `${metadata.cell_id} burst metadata and frames must come together`,
  );
  assert(
    cellFrame === null || metadata.burst,
    `${metadata.cell_id} names a cell frame but carries no burst`,
  );
  if (metadata.burst) {
    const { interval_ms: intervalMs, frames } = metadata.burst;
    assert(Number.isInteger(intervalMs) && intervalMs >= 100, `${metadata.cell_id} burst interval is invalid`);
    assert(
      Array.isArray(frames) && frames.length >= 2 && frames.length <= 60 &&
        frames.length === burstFrames.length,
      `${metadata.cell_id} burst has a wrong frame count`,
    );
    const timing = [];
    frames.forEach((frame, index) => {
      const label = `${metadata.cell_id} burst frame ${index}`;
      assert(frame.index === index, `${label} is out of order`);
      assert(
        Number.isInteger(frame.t_ms) && frame.t_ms >= 0 &&
          (index === 0 || frame.t_ms > frames[index - 1].t_ms),
        `${label} offset does not rise`,
      );
      assert(Number.isFinite(Date.parse(frame.captured_at_utc)), `${label} has an invalid timestamp`);
      const decodedFrame = PNG.sync.read(burstFrames[index]);
      assert(decodedFrame.width === 640 && decodedFrame.height === 480, `${label} is not 640x480`);
      const frameDigest = sha256(burstFrames[index]);
      assert(frameDigest === frame.sha256, `${label} PNG hash disagrees with its sidecar`);
      const framePath = `.artifacts/interface-parity/a0/${cell.id}/frame-${String(index).padStart(2, "0")}.png`;
      artifacts.push({
        kind: "visual_png", path: framePath, sha256: frameDigest,
        mime_type: "image/png", width: 640, height: 480,
      });
      sources.push(burstFrames[index]);
      timing.push({
        index, t_ms: frame.t_ms, captured_at_utc: frame.captured_at_utc,
        path: framePath, sha256: frameDigest, screen_sha256: frame.screen_sha256,
      });
    });
    assert(
      frames.at(-1).sha256 === digest,
      `${metadata.cell_id} frame is not the burst's last frame`,
    );
    const chosen = cellFrame ?? frames.length - 1;
    assert(
      Number.isInteger(chosen) && chosen >= 0 && chosen < frames.length,
      `${metadata.cell_id} cell frame ${cellFrame} is not in the burst`,
    );
    // The guest hashed every frame, so the chosen one keeps its provenance.
    artifacts[0].sha256 = frames[chosen].sha256;
    sources[0] = burstFrames[chosen];
    const timingBytes = Buffer.from(`${JSON.stringify({
      interval_ms: intervalMs, ...(cellFrame === null ? {} : { cell_frame: chosen }), frames: timing,
    }, null, 2)}\n`);
    artifacts.push({
      kind: "interaction_trace",
      path: `.artifacts/interface-parity/a0/${cell.id}/burst.json`,
      sha256: sha256(timingBytes),
      mime_type: "application/json",
    });
    sources.push(timingBytes);
  }
  // A cell about sound carries the guest's audio, recorded on the host by
  // QEMU's wavcapture across the moment of the cell's frame.
  if (audio) {
    const label = `${metadata.cell_id} audio`;
    const meta = audio.metadata;
    const audioDigest = sha256(audio.bytes);
    assert(audioDigest === meta?.sha256, `${label} hash disagrees with its sidecar`);
    assert(typeof meta.source === "string" && meta.source.length > 0, `${label} has no source`);
    const started = Date.parse(meta.started_at_utc);
    const stopped = Date.parse(meta.stopped_at_utc);
    assert(
      Number.isFinite(started) && Number.isFinite(stopped) && stopped > started,
      `${label} has invalid start and stop timestamps`,
    );
    const wav = readWav(audio.bytes, label);
    assert(wav.duration_s >= AUDIO_MIN_SECONDS, `${label} is shorter than ${AUDIO_MIN_SECONDS} s`);
    assert(wav.audible, `${label} is silent`);
    // QEMU writes only while the guest plays, so a paused battle adds wall
    // time but no samples: the WAV may be shorter than the recording's span.
    assert(
      wav.duration_s * 1000 <= stopped - started + AUDIO_CLOCK_TOLERANCE_MS,
      `${label} is longer than the time it was recording`,
    );
    const frameAt = Date.parse(metadata.captured_at_utc);
    assert(
      frameAt >= started - AUDIO_CLOCK_TOLERANCE_MS && frameAt <= stopped + AUDIO_CLOCK_TOLERANCE_MS,
      `${label} was not recording when the cell's frame was captured`,
    );
    artifacts.push({
      kind: "audio_capture",
      path: `.artifacts/interface-parity/a0/${cell.id}/original.wav`,
      sha256: audioDigest,
      mime_type: "audio/wav",
    });
    sources.push(audio.bytes);
    const audioBytes = Buffer.from(`${JSON.stringify({
      source: meta.source,
      started_at_utc: meta.started_at_utc,
      stopped_at_utc: meta.stopped_at_utc,
      duration_s: Number(wav.duration_s.toFixed(3)),
      sample_rate: wav.sample_rate,
      channels: wav.channels,
      bits_per_sample: wav.bits_per_sample,
      // Wall-clock seconds from the start; not a position in the WAV, which
      // holds no samples for the time the guest was silent.
      frame_wall_offset_s: Number(((frameAt - started) / 1000).toFixed(3)),
    }, null, 2)}\n`);
    artifacts.push({
      kind: "interaction_trace",
      path: `.artifacts/interface-parity/a0/${cell.id}/audio.json`,
      sha256: sha256(audioBytes),
      mime_type: "application/json",
    });
    sources.push(audioBytes);
  }

  const record = {
    cell_id: cell.id,
    requirement: cell.requirement,
    captured_at: metadata.captured_at_utc,
    state_setup: metadata.state_setup,
    input_trace: metadata.input_trace,
    artifacts,
  };
  // The bytes of each burst artifact, by index. Not enumerable, so the
  // record stays the manifest's shape wherever it is cloned or serialized.
  Object.defineProperty(record, "sources", { value: sources });
  return record;
}

export function mergeCapture({ manifest, capture, cells }) {
  const existing = manifest.captures.find(({ cell_id: cellId }) => cellId === capture.cell_id);
  if (existing) {
    const existingHash = existing.artifacts.find(({ kind }) => kind === "visual_png")?.sha256;
    const captureHash = capture.artifacts[0].sha256;
    assert(
      existingHash === captureHash && JSON.stringify(existing) === JSON.stringify(capture),
      `${capture.cell_id} already has a different registered capture`,
    );
    return structuredClone(manifest);
  }

  const next = structuredClone(manifest);
  next.captures.push(capture);
  const order = new Map(cells.map(({ id }, index) => [id, index]));
  next.captures.sort((left, right) => order.get(left.cell_id) - order.get(right.cell_id));
  a0Coverage(next, cells);
  return next;
}

export function ingestCapture({
  root, surfaceLedger, manifest, metadata, pngPath, beforePath = null, beforeMetadata = null,
  burstDir = null, cellFrame = null, audioPath = null, audioMetadata = null, validateManifest = null,
}) {
  const cells = collectA0Cells(surfaceLedger);
  const pngBytes = fs.readFileSync(pngPath);
  assert((beforePath === null) === (beforeMetadata === null), "a before-frame needs both PNG and metadata");
  const before = beforePath ? { metadata: beforeMetadata, pngBytes: fs.readFileSync(beforePath) } : null;
  // Frames are read by position; the record checks each one's index.
  const frameCount = Array.isArray(metadata.burst?.frames) ? metadata.burst.frames.length : 0;
  const burstFrames = burstDir
    ? Array.from({ length: Math.min(frameCount, 60) }, (_, index) =>
      fs.readFileSync(path.join(burstDir, `f${String(index).padStart(2, "0")}.png`)))
    : null;
  assert((audioPath === null) === (audioMetadata === null), "audio needs both WAV and metadata");
  const audio = audioPath ? { metadata: audioMetadata, bytes: fs.readFileSync(audioPath) } : null;
  const capture = buildCaptureRecord({
    metadata, pngBytes, cells, before, burstFrames, cellFrame, audio,
  });
  const nextManifest = mergeCapture({ manifest, capture, cells });
  // Validate before any write, so a refused capture leaves no files behind.
  if (validateManifest) validateManifest(nextManifest);
  const destination = path.resolve(root, capture.artifacts[0].path);
  const allowedRoot = path.resolve(root, ".artifacts/interface-parity/a0");
  assert(
    destination.startsWith(`${allowedRoot}${path.sep}`),
    `${capture.cell_id} destination escapes the ignored A0 store`,
  );
  // Every file the record names, checked before any is written: one that
  // exists must already hold the same bytes.
  const writes = capture.artifacts.map((artifact, index) => ({
    target: path.resolve(root, artifact.path),
    sha256: artifact.sha256,
    bytes: capture.sources[index] ?? fs.readFileSync(index === 0 ? pngPath : beforePath),
  }));
  for (const { target, sha256: expected } of writes) {
    if (fs.existsSync(target)) {
      assert(
        sha256(fs.readFileSync(target)) === expected,
        `${capture.cell_id} ${path.basename(target)} already contains different bytes`,
      );
    }
  }
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  for (const { target, bytes } of writes) {
    if (fs.existsSync(target)) continue;
    // Write then rename, so a killed run never leaves a torn file.
    const temporary = `${target}.tmp-${process.pid}`;
    fs.writeFileSync(temporary, bytes);
    fs.renameSync(temporary, target);
  }
  return { manifest: nextManifest, capture, destination };
}
