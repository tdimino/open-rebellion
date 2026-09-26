import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { PNG } from "pngjs";
import {
  buildCaptureRecord,
  ingestCapture,
  mergeCapture,
} from "./tactical-a0-ingest.mjs";
import {
  EXPECTED_ORIGINAL_EXE_SHA256,
  collectTacticalCells,
} from "./tactical-matrix.mjs";

const repository = path.resolve(import.meta.dirname, "../..");
const readJson = (relative) => JSON.parse(fs.readFileSync(path.join(repository, relative), "utf8"));
const ledger = readJson("docs/qa/2026-09-10-interface-parity-audit/surface-ledger.json");
const example = readJson("tools/interface-parity/tactical-a0-manifest.example.json");
const cells = collectTacticalCells(ledger);
const cell = cells[0];

function pngBytes(width = 640, height = 480) {
  const png = new PNG({ width, height });
  png.data.fill(0);
  return PNG.sync.write(png);
}

function metadata(bytes) {
  return {
    name: cell.id,
    cell_id: cell.id,
    requirement: cell.requirement,
    sha256: createHash("sha256").update(bytes).digest("hex"),
    width: 640,
    height: 480,
    process: "REBEXE",
    executable_sha256: EXPECTED_ORIGINAL_EXE_SHA256,
    windows_build: "test-build",
    captured_at_utc: "2026-09-22T12:00:00Z",
    method: "GDI client-area capture",
    state_setup: {
      kind: "live-journey",
      identifier: "test-journey",
      notes: "test",
    },
    input_trace: ["launch REBEXE.exe -w", "enter tactical battle"],
  };
}

test("builds a provenance-complete 640x480 capture record", () => {
  const bytes = pngBytes();
  const capture = buildCaptureRecord({ metadata: metadata(bytes), pngBytes: bytes, cells });
  assert.equal(capture.cell_id, cell.id);
  assert.equal(capture.artifacts[0].width, 640);
  assert.equal(capture.artifacts[0].height, 480);
  assert.match(capture.artifacts[0].path, new RegExp(`${cell.id}/original\\.png$`));
});

test("rejects resized, modified, and mismatched captures", () => {
  const bytes = pngBytes();
  const resized = pngBytes(1280, 960);
  assert.throws(
    () => buildCaptureRecord({ metadata: metadata(bytes), pngBytes: resized, cells }),
    /PNG is 1280x960/,
  );
  const wrongHash = metadata(bytes);
  wrongHash.sha256 = "0".repeat(64);
  assert.throws(
    () => buildCaptureRecord({ metadata: wrongHash, pngBytes: bytes, cells }),
    /hash disagrees/,
  );
  const wrongExe = metadata(bytes);
  wrongExe.executable_sha256 = "1".repeat(64);
  assert.throws(
    () => buildCaptureRecord({ metadata: wrongExe, pngBytes: bytes, cells }),
    /executable hash/,
  );
  const wrongRequirement = metadata(bytes);
  wrongRequirement.requirement = "different state";
  assert.throws(
    () => buildCaptureRecord({ metadata: wrongRequirement, pngBytes: bytes, cells }),
    /requirement does not match/,
  );
  const missingSetup = metadata(bytes);
  missingSetup.state_setup.identifier = "";
  assert.throws(
    () => buildCaptureRecord({ metadata: missingSetup, pngBytes: bytes, cells }),
    /state setup identifier/,
  );
  const invalidTrace = metadata(bytes);
  invalidTrace.input_trace = [""];
  assert.throws(
    () => buildCaptureRecord({ metadata: invalidTrace, pngBytes: bytes, cells }),
    /ordered input trace/,
  );
});

test("merge is idempotent and rejects replacement bytes", () => {
  const bytes = pngBytes();
  const capture = buildCaptureRecord({ metadata: metadata(bytes), pngBytes: bytes, cells });
  const once = mergeCapture({ manifest: example, capture, cells });
  const twice = mergeCapture({ manifest: once, capture, cells });
  assert.deepEqual(twice, once);
  const changed = structuredClone(capture);
  changed.artifacts[0].sha256 = "f".repeat(64);
  assert.throws(() => mergeCapture({ manifest: once, capture: changed, cells }), /different/);
});

test("ingests only into the ignored per-cell A0 store", () => {
  const temporaryRoot = fs.mkdtempSync(path.join(os.tmpdir(), "rebellion-a0-ingest-"));
  const bytes = pngBytes();
  const source = path.join(temporaryRoot, "source.png");
  fs.writeFileSync(source, bytes);
  const result = ingestCapture({
    root: temporaryRoot,
    surfaceLedger: ledger,
    manifest: example,
    metadata: metadata(bytes),
    pngPath: source,
  });
  assert.equal(
    path.relative(temporaryRoot, result.destination),
    `.artifacts/interface-parity/a0/${cell.id}/original.png`,
  );
  assert.deepEqual(fs.readFileSync(result.destination), bytes);
});
