import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020.js";
import {
  EXPECTED_ORIGINAL_EXE_SHA256,
  TACTICAL_CELL_TOTAL,
  a0Coverage,
  buildTacticalMatrix,
  catalogCoverage,
  collectTacticalCells,
  forbiddenTrackedA0Paths,
} from "./tactical-matrix.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const readJson = (relative) => JSON.parse(fs.readFileSync(path.join(root, relative), "utf8"));
const ledger = readJson("docs/qa/2026-09-10-interface-parity-audit/surface-ledger.json");
const catalog = readJson("tools/interface-parity/scenarios/tactical.catalog.json");
const example = readJson("tools/interface-parity/tactical-a0-manifest.example.json");

function clone(value) {
  return structuredClone(value);
}

function completeCatalog(cells) {
  return {
    scenarios: cells.map((cell, index) => ({
      slug: `cell-${index + 1}`,
      execution_kind: index % 2 === 0 ? "journey" : "snapshot",
      audit_cells: [cell.id],
    })),
  };
}

function completeManifest(cells) {
  const manifest = clone(example);
  manifest.captures = cells.map((cell) => ({
    cell_id: cell.id,
    requirement: cell.requirement,
    captured_at: "2026-09-22T00:00:00Z",
    state_setup: { kind: "live-journey", identifier: cell.id, notes: "test" },
    input_trace: ["launch original executable", `reach ${cell.id}`],
    artifacts: [{
      kind: "visual_png",
      path: `.artifacts/interface-parity/a0/${cell.id}/original.png`,
      sha256: "a".repeat(64),
      mime_type: "image/png",
      width: 640,
      height: 480,
    }],
  }));
  return manifest;
}

test("canonical tactical denominator is exactly 106 with the expected family distribution", () => {
  const cells = collectTacticalCells(ledger);
  assert.equal(cells.length, TACTICAL_CELL_TOTAL);
  assert.deepEqual(
    Object.fromEntries([...new Set(cells.map(({ family }) => family))].map((family) => [
      family,
      cells.filter((cell) => cell.family === family).length,
    ])),
    { "TAC-01": 13, "TAC-02": 13, "TAC-03": 20, "TAC-04": 14,
      "TAC-05": 23, "TAC-06": 9, "TAC-07": 14 },
  );
});

test("denominator drift fails closed", () => {
  const changed = clone(ledger);
  changed.surfaces.find(({ id }) => id === "TAC-04").acceptance_cells.pop();
  assert.throws(() => collectTacticalCells(changed), /TAC-04 denominator drift/);
});

test("unknown and duplicate catalog mappings fail closed", () => {
  const cells = collectTacticalCells(ledger);
  const unknown = clone(catalog);
  unknown.scenarios[0].audit_cells = ["TAC-07-C999"];
  assert.throws(() => catalogCoverage(unknown, cells), /unknown tactical cell/);

  const duplicate = clone(catalog);
  duplicate.scenarios[0].audit_cells = [cells[0].id];
  duplicate.scenarios[2].audit_cells = [cells[0].id];
  assert.throws(() => catalogCoverage(duplicate, cells), /mapped by both/);
});

test("development matrix reports partial mapping separately from original evidence", () => {
  const result = buildTacticalMatrix({ surfaceLedger: ledger, catalog });
  assert.deepEqual(result.summary, {
    denominator: 106,
    matrix_coverage: 104,
    journey_cells: 85,
    snapshot_cells: 19,
    a0_coverage: 0,
    parity_acceptance: 0,
    catalog_cells_missing: 2,
    a0_cells_missing: 106,
    status: "incomplete",
  });
});

test("strict coverage requires all 106 catalog and A0 records", () => {
  const cells = collectTacticalCells(ledger);
  assert.throws(
    () => buildTacticalMatrix({ surfaceLedger: ledger, catalog, strict: true }),
    /tactical catalog is missing 2 cells/,
  );
  const result = buildTacticalMatrix({
    surfaceLedger: ledger,
    catalog: completeCatalog(cells),
    a0Manifest: completeManifest(cells),
    strict: true,
  });
  assert.equal(result.summary.matrix_coverage, 106);
  assert.equal(result.summary.journey_cells, 53);
  assert.equal(result.summary.snapshot_cells, 53);
  assert.equal(result.summary.a0_coverage, 106);
  assert.equal(result.summary.status, "runnable-not-accepted");
});

test("release acceptance always implies strict catalog and A0 coverage", () => {
  const acceptedLedger = clone(ledger);
  for (const surface of acceptedLedger.surfaces.filter(({ id }) => /^TAC-0[1-7]$/.test(id))) {
    for (const cell of surface.acceptance_cells) cell.status = "passed";
  }
  assert.throws(
    () => buildTacticalMatrix({
      surfaceLedger: acceptedLedger,
      catalog,
      requireAccepted: true,
    }),
    /tactical catalog is missing 2 cells/,
  );
});

test("scenario kinds are mandatory and negative controls cannot claim cells", () => {
  const cells = collectTacticalCells(ledger);
  const missingKind = clone(catalog);
  delete missingKind.scenarios[0].execution_kind;
  assert.throws(() => catalogCoverage(missingKind, cells), /invalid execution_kind/);

  const negativeClaim = clone(catalog);
  negativeClaim.scenarios.find(({ execution_kind: kind }) => kind === "negative-control")
    .audit_cells = [cells[0].id];
  assert.throws(() => catalogCoverage(negativeClaim, cells), /may not map acceptance cells/);
});

test("A0 manifest schema requires capture provenance", () => {
  const schema = readJson("tools/interface-parity/schemas/tactical-a0-manifest.schema.json");
  const validate = new Ajv2020({ allErrors: true, formats: { "date-time": true } }).compile(schema);
  assert.equal(validate(example), true);
  const missing = completeManifest(collectTacticalCells(ledger));
  delete missing.source.vm.scale_percent;
  assert.equal(validate(missing), false);
  assert.match(JSON.stringify(validate.errors), /scale_percent/);
});

test("A0 semantic validation rejects modified sources and unsafe paths", () => {
  const cells = collectTacticalCells(ledger);
  const manifest = completeManifest(cells);
  assert.equal(manifest.source.executable_sha256, EXPECTED_ORIGINAL_EXE_SHA256);
  manifest.source.unmodified = false;
  assert.throws(() => a0Coverage(manifest, cells), /must be unmodified/);

  const unsafe = completeManifest(cells);
  unsafe.captures[0].artifacts[0].path = "data/base/original.png";
  assert.throws(() => a0Coverage(unsafe, cells), /ignored A0 directory/);

  const resized = completeManifest(cells);
  resized.captures[0].artifacts[0].width = 1280;
  assert.throws(() => a0Coverage(resized, cells), /visual width must be 640/);
});

test("tracked proprietary A0 locations fail exclusion", () => {
  assert.deepEqual(forbiddenTrackedA0Paths([
    "README.md",
    ".artifacts/interface-parity/a0/TAC-01-C001/original.png",
    "tools/interface-parity/tactical-a0-manifest.local.json",
    "tools/interface-parity/baselines/original/tactical/TAC-01-C001.png",
  ]), [
    ".artifacts/interface-parity/a0/TAC-01-C001/original.png",
    "tools/interface-parity/tactical-a0-manifest.local.json",
    "tools/interface-parity/baselines/original/tactical/TAC-01-C001.png",
  ]);
});
