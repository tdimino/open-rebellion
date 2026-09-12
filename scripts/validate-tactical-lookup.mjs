#!/usr/bin/env node
// Check the documented native ordinal table and its explicitly candidate DAT labels.

import assert from "node:assert/strict";
import crypto from "node:crypto";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const read = (relative) => JSON.parse(fs.readFileSync(path.join(root, relative), "utf8"));
const lookup = read("docs/reference/asset-library/tactical-lookup.json");
const names = read("web/data/base/textstra.json");
const ships = read("data/base/json/CAPSHPSD.json").ships;
const fighters = read("data/base/json/FIGHTSD.json").fighters;

assert.equal(lookup.ordinal_to_resource_grade, "source-proven");
assert.equal(lookup.candidate_dat_identity_grade, "candidate");
assert.equal(lookup.capital_ships.length, 29);
assert.equal(lookup.fighters.length, 8);
assert.deepEqual(lookup.capital_ship_columns, ["tactical_ordinal", "type301_resource_base", "candidate_capshpsd_id", "candidate_name"]);
assert.deepEqual(lookup.fighter_columns, ["tactical_ordinal", "first_side_type303_base", "opposing_side_type303_base", "candidate_fightsd_id", "candidate_name"]);

const expectedShipBases = [
  ...Array.from({ length: 15 }, (_, index) => 2010 + index * 10),
  ...Array.from({ length: 14 }, (_, index) => 2510 + index * 10),
];
for (const [index, row] of lookup.capital_ships.entries()) {
  const [ordinal, base, datID, name] = row;
  assert.equal(ordinal, index);
  assert.equal(base, expectedShipBases[index]);
  const record = ships.find((ship) => ship.id === datID);
  assert.ok(record, `unknown candidate ship DAT ID ${datID}`);
  assert.equal(names[String(record.text_stra_dll_id)], name);
}
assert.equal(new Set(lookup.capital_ships.map((row) => row[2])).size, 29);
assert.equal(lookup.death_star.candidate_capshpsd_id, 136);
assert.equal(lookup.death_star.resource_base_without_flag, 5010);
assert.equal(lookup.death_star.resource_base_with_flag, 5020);

const expectedFighterBases = [4000, 4010, 4020, 4030, 4100, 4110, 4120, 4130];
for (const [index, row] of lookup.fighters.entries()) {
  const [ordinal, firstBase, otherBase, datID, name] = row;
  assert.equal(ordinal, index + 29);
  assert.equal(firstBase, expectedFighterBases[index]);
  assert.equal(otherBase, firstBase + 4);
  const record = fighters.find((fighter) => fighter.id === datID);
  assert.ok(record, `unknown candidate fighter DAT ID ${datID}`);
  assert.equal(names[String(record.text_stra_dll_id)], name);
}
assert.equal(new Set(lookup.fighters.map((row) => row[3])).size, 8);

const sourceArg = process.argv.find((arg) => arg.startsWith("--source="));
if (sourceArg) {
  const sourcePath = sourceArg.slice("--source=".length);
  const digest = crypto.createHash("sha256").update(fs.readFileSync(sourcePath)).digest("hex");
  assert.equal(digest, lookup.source_sha256, "original executable hash mismatch");
}

process.stdout.write("Validated 29 native ship ordinals, eight fighter ordinals, and candidate DAT labels\n");
