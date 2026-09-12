#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020.js";

const here = path.dirname(fileURLToPath(import.meta.url));
const catalogPath = path.join(here, "scenarios/gid.catalog.json");
const schemaPath = path.join(here, "schemas/scenario-catalog.schema.json");
const catalog = JSON.parse(fs.readFileSync(catalogPath, "utf8"));
const schema = JSON.parse(fs.readFileSync(schemaPath, "utf8"));
const validate = new Ajv2020({ allErrors: true }).compile(schema);

if (!validate(catalog)) {
  process.stderr.write(`${JSON.stringify(validate.errors, null, 2)}\n`);
  process.exit(1);
}

const indices = catalog.scenarios.map(({ index }) => index);
const slugs = catalog.scenarios.map(({ slug }) => slug);
const requirements = catalog.scenarios.map(({ requirement }) => requirement);
const viewportIds = catalog.viewports.map(({ id }) => id);
for (const [name, values] of Object.entries({ indices, slugs, requirements, viewportIds })) {
  if (new Set(values).size !== values.length) {
    throw new Error(`duplicate ${name} in ${catalogPath}`);
  }
}
if (indices.some((value, index) => value !== index)) {
  throw new Error("scenario indices must remain contiguous and stable from 0 through 37");
}

const expected = [
  "galaxy", "sector", "system", "display off", "support", "uprising",
  "fleets", "personnel", "energy", "raw materials", "mines", "refineries",
  "shipyards", "training", "construction", "defenses",
  "matching legend", "known", "unknown", "uninhabited", "HQ",
  "blockade", "mission", "fleet", "Death Star intelligence", "hover",
  "selection", "pan", "zoom",
];
if (JSON.stringify(requirements.slice(0, 29)) !== JSON.stringify(expected)) {
  throw new Error("catalog no longer matches the 29 canonical CMD-02 requirements");
}
const extended = [
  "fleets enroute filter", "active personnel filter", "idle shipyards filter",
  "idle training facilities filter", "idle construction yards filter",
  "troopers filter", "fighter squadrons filter", "Death Star shields filter",
  "planetary shield generators filter",
];
if (JSON.stringify(requirements.slice(29)) !== JSON.stringify(extended)) {
  throw new Error("catalog no longer covers every recovered native GID filter");
}

process.stdout.write(`${JSON.stringify({
  status: "pass",
  family: catalog.family,
  scenarios: catalog.scenarios.length,
  executions: catalog.scenarios.length * catalog.factions.length * catalog.viewports.length,
}, null, 2)}\n`);
