#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020.js";
import { ingestCapture } from "./tactical-a0-ingest.mjs";
import { a0Coverage, collectTacticalCells } from "./tactical-matrix.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const argumentsByName = new Map(process.argv.slice(2).map((argument) => {
  const index = argument.indexOf("=");
  return index < 0 ? [argument, true] : [argument.slice(0, index), argument.slice(index + 1)];
}));
const requiredPath = (name) => {
  const value = argumentsByName.get(name);
  if (typeof value !== "string" || value.length === 0) {
    throw new Error(`missing ${name}=<path>`);
  }
  return path.resolve(root, value);
};
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));

const manifestPath = requiredPath("--manifest");
const pngPath = requiredPath("--png");
const metadataPath = requiredPath("--metadata");
const allowedManifestRoot = path.resolve(root, ".artifacts/interface-parity/a0");
if (!manifestPath.startsWith(`${allowedManifestRoot}${path.sep}`)) {
  throw new Error("manifest must remain inside .artifacts/interface-parity/a0/");
}

const surfaceLedger = readJson(path.join(
  root,
  "docs/qa/2026-09-10-interface-parity-audit/surface-ledger.json",
));
const manifest = readJson(manifestPath);
const metadata = readJson(metadataPath);
const schema = readJson(path.join(here, "schemas/tactical-a0-manifest.schema.json"));
const validate = new Ajv2020({
  allErrors: true,
  formats: { "date-time": true },
}).compile(schema);
if (!validate(manifest)) {
  throw new Error(`input A0 manifest is invalid:\n${JSON.stringify(validate.errors, null, 2)}`);
}
const result = ingestCapture({ root, surfaceLedger, manifest, metadata, pngPath });
if (!validate(result.manifest)) {
  throw new Error(`updated A0 manifest is invalid:\n${JSON.stringify(validate.errors, null, 2)}`);
}
const cells = collectTacticalCells(surfaceLedger);
const coverage = a0Coverage(result.manifest, cells);
const temporary = `${manifestPath}.tmp-${process.pid}`;
fs.writeFileSync(temporary, `${JSON.stringify(result.manifest, null, 2)}\n`);
fs.renameSync(temporary, manifestPath);
process.stdout.write(`${JSON.stringify({
  status: "pass",
  cell_id: result.capture.cell_id,
  sha256: result.capture.artifacts[0].sha256,
  destination: path.relative(root, result.destination),
  a0_coverage: coverage.captures.size,
  a0_missing: coverage.missing.length,
}, null, 2)}\n`);
