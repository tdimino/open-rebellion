#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020.js";
import { buildTacticalMatrix } from "./tactical-matrix.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const argumentsByName = new Map(process.argv.slice(2).map((argument) => {
  const index = argument.indexOf("=");
  return index < 0 ? [argument, true] : [argument.slice(0, index), argument.slice(index + 1)];
}));
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const resolveArgument = (name, fallback) => path.resolve(root, argumentsByName.get(name) ?? fallback);

const surfaceLedger = readJson(resolveArgument(
  "--surface-ledger",
  "docs/qa/2026-09-10-interface-parity-audit/surface-ledger.json",
));
const catalog = readJson(resolveArgument(
  "--catalog",
  "tools/interface-parity/scenarios/tactical.catalog.json",
));
let a0Manifest = null;
if (argumentsByName.has("--a0-manifest")) {
  const manifestPath = resolveArgument("--a0-manifest", "");
  a0Manifest = readJson(manifestPath);
  const schema = readJson(path.join(here, "schemas/tactical-a0-manifest.schema.json"));
  const validate = new Ajv2020({
    allErrors: true,
    formats: { "date-time": true },
  }).compile(schema);
  if (!validate(a0Manifest)) {
    throw new Error(`invalid A0 manifest:\n${JSON.stringify(validate.errors, null, 2)}`);
  }
}

const result = buildTacticalMatrix({
  surfaceLedger,
  catalog,
  a0Manifest,
  strict: argumentsByName.has("--strict"),
  requireAccepted: argumentsByName.has("--require-accepted"),
});
const reported = argumentsByName.has("--rows") || argumentsByName.has("--write")
  ? result
  : {
      schema_version: result.schema_version,
      family: result.family,
      expected_distribution: result.expected_distribution,
      summary: result.summary,
    };
const serialized = `${JSON.stringify(reported, null, 2)}\n`;
if (argumentsByName.has("--write")) {
  const output = resolveArgument("--write", "");
  fs.mkdirSync(path.dirname(output), { recursive: true });
  fs.writeFileSync(output, serialized);
} else {
  process.stdout.write(serialized);
}
