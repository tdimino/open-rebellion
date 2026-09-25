#!/usr/bin/env node
// Check the documented native ordinal table and its source-proven DAT identities.

import assert from "node:assert/strict";
import crypto from "node:crypto";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const read = (relative) => JSON.parse(fs.readFileSync(path.join(root, relative), "utf8"));
const lookup = read("docs/reference/asset-library/tactical-lookup.json");
const voices = read("docs/reference/asset-library/tactical-voice-map.json");
const names = read("web/data/base/textstra.json");
const ships = read("data/base/json/CAPSHPSD.json").ships;
const fighters = read("data/base/json/FIGHTSD.json").fighters;

assert.equal(lookup.ordinal_to_resource_grade, "source-proven");
assert.equal(lookup.dat_identity_grade, "source-proven");
assert.equal(lookup.capital_ships.length, 29);
assert.equal(lookup.fighters.length, 8);
assert.deepEqual(lookup.capital_ship_columns, ["tactical_ordinal", "type301_resource_base", "capshpsd_id", "name"]);
assert.deepEqual(lookup.fighter_columns, ["tactical_ordinal", "first_side_type303_base", "opposing_side_type303_base", "fightsd_id", "name"]);

const expectedShipBases = [
  ...Array.from({ length: 15 }, (_, index) => 2010 + index * 10),
  ...Array.from({ length: 14 }, (_, index) => 2510 + index * 10),
];
for (const [index, row] of lookup.capital_ships.entries()) {
  const [ordinal, base, datID, name] = row;
  assert.equal(ordinal, index);
  assert.equal(base, expectedShipBases[index]);
  const record = ships.find((ship) => ship.id === datID);
  assert.ok(record, `unknown ship DAT ID ${datID}`);
  assert.equal(names[String(record.text_stra_dll_id)], name);
}
assert.equal(new Set(lookup.capital_ships.map((row) => row[2])).size, 29);
assert.equal(lookup.death_star.capshpsd_id, 136);
assert.equal(lookup.death_star.dat_identity_grade, "source-proven");
assert.equal(lookup.death_star.resource_base_without_flag, 5010);
assert.equal(lookup.death_star.resource_base_with_flag, 5020);

assert.equal(voices.schema_version, 1);
assert.equal(voices.source_binary, "REBEXE.EXE");
assert.equal(voices.source_sha256, lookup.source_sha256);
assert.equal(voices.resource_type, "WAVE");
assert.deepEqual(voices.resource_formulae, [
  { faction: "alliance", dll: "VOICEFXA.DLL", event_range: [32, 153], resource_base: 14001, resource_count: 122 },
  { faction: "empire", dll: "VOICEFXE.DLL", event_range: [154, 285], resource_base: 15001, resource_count: 132 },
  { faction: "alliance", dll: "VOICEFXA.DLL", event_range: [286, 316], resource_base: 15133, resource_count: 31 },
]);

const voiceEvents = [];
const voiceResources = [];
for (const [faction, family, eventFirst, eventLast, resourceFirst, resourceLast] of voices.families) {
  assert.ok(["alliance", "empire"].includes(faction), `invalid voice faction for ${family}`);
  assert.equal(eventLast - eventFirst, resourceLast - resourceFirst, `voice span mismatch for ${family}`);
  const formula = voices.resource_formulae.find(({ event_range: [first, last] }) =>
    eventFirst >= first && eventLast <= last);
  assert.ok(formula, `missing voice formula for ${family}`);
  assert.equal(resourceFirst, formula.resource_base + eventFirst - formula.event_range[0]);
  assert.equal(resourceLast, formula.resource_base + eventLast - formula.event_range[0]);
  voiceEvents.push(...Array.from({ length: eventLast - eventFirst + 1 }, (_, index) => eventFirst + index));
  voiceResources.push(...Array.from({ length: resourceLast - resourceFirst + 1 }, (_, index) => resourceFirst + index));
}
assert.deepEqual(voiceEvents, Array.from({ length: 285 }, (_, index) => index + 32));
assert.equal(new Set(voiceEvents).size, 285);
assert.equal(new Set(voiceResources).size, 285);
assert.deepEqual([...new Set(voiceResources)].sort((a, b) => a - b), [
  ...Array.from({ length: 122 }, (_, index) => 14001 + index),
  ...Array.from({ length: 163 }, (_, index) => 15001 + index),
]);
assert.equal(voices.coverage.resource_count, 285);
assert.equal(voices.coverage.runtime_transport, "implemented");
assert.equal(lookup.command_voice_contract.complete_bank_map, "tactical-voice-map.json");
assert.equal(lookup.command_voice_contract.full_bank_transport, "285 source resources implemented");

const expectedFighterBases = [4000, 4010, 4020, 4030, 4100, 4110, 4120, 4130];
for (const [index, row] of lookup.fighters.entries()) {
  const [ordinal, firstBase, otherBase, datID, name] = row;
  assert.equal(ordinal, index + 29);
  assert.equal(firstBase, expectedFighterBases[index]);
  assert.equal(otherBase, firstBase + 4);
  const record = fighters.find((fighter) => fighter.id === datID);
  assert.ok(record, `unknown fighter DAT ID ${datID}`);
  assert.equal(names[String(record.text_stra_dll_id)], name);
}
assert.equal(new Set(lookup.fighters.map((row) => row[3])).size, 8);

const sourceArg = process.argv.find((arg) => arg.startsWith("--source="));
if (sourceArg) {
  const sourcePath = sourceArg.slice("--source=".length);
  const digest = crypto.createHash("sha256").update(fs.readFileSync(sourcePath)).digest("hex");
  assert.equal(digest, lookup.source_sha256, "original executable hash mismatch");
}

process.stdout.write("Validated 29 ship ordinals, eight fighter ordinals, source-proven DAT identities, and 285 tactical voices\n");
