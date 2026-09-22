import { createHash } from "node:crypto";
import fs from "node:fs";
import path from "node:path";
import { PNG } from "pngjs";
import {
  EXPECTED_ORIGINAL_EXE_SHA256,
  a0Coverage,
  collectTacticalCells,
} from "./tactical-matrix.mjs";

const fail = (message) => {
  throw new Error(message);
};

const assert = (condition, message) => {
  if (!condition) fail(message);
};

const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");

export function buildCaptureRecord({ metadata, pngBytes, cells }) {
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

  return {
    cell_id: cell.id,
    requirement: cell.requirement,
    captured_at: metadata.captured_at_utc,
    state_setup: metadata.state_setup,
    input_trace: metadata.input_trace,
    artifacts: [{
      kind: "visual_png",
      path: `.artifacts/interface-parity/a0/${cell.id}/original.png`,
      sha256: digest,
      mime_type: "image/png",
      width: decoded.width,
      height: decoded.height,
    }],
  };
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

export function ingestCapture({ root, surfaceLedger, manifest, metadata, pngPath }) {
  const cells = collectTacticalCells(surfaceLedger);
  const pngBytes = fs.readFileSync(pngPath);
  const capture = buildCaptureRecord({ metadata, pngBytes, cells });
  const nextManifest = mergeCapture({ manifest, capture, cells });
  const destination = path.resolve(root, capture.artifacts[0].path);
  const allowedRoot = path.resolve(root, ".artifacts/interface-parity/a0");
  assert(
    destination.startsWith(`${allowedRoot}${path.sep}`),
    `${capture.cell_id} destination escapes the ignored A0 store`,
  );
  if (fs.existsSync(destination)) {
    assert(
      sha256(fs.readFileSync(destination)) === capture.artifacts[0].sha256,
      `${capture.cell_id} destination already contains different bytes`,
    );
  } else {
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    fs.copyFileSync(pngPath, destination);
  }
  return { manifest: nextManifest, capture, destination };
}
