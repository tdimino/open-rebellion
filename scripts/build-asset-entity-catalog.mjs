#!/usr/bin/env node
// Inventory game entities and the graphic relations actually used by this port.

import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const output = path.join(root, "docs/reference/asset-library/entity-catalog.json");
const check = process.argv.includes("--check");
const read = (relative) => JSON.parse(fs.readFileSync(path.join(root, relative), "utf8"));
const names = read("web/data/base/textstra.json");
const resourceInventory = read("docs/reference/asset-library/resource-inventory.json");
const resourceKeys = new Set(resourceInventory.families.flatMap((family) =>
  family.bitmaps.map((bitmap) => `${family.module}/BMP/${bitmap.id}`)));

const families = [
  { file: "CAPSHPSD", key: "ships", family: "capital_ship", roles: ["strategic_mini_icon", "status_view", "encyclopedia_image", "tactical_bitmap", "tactical_mesh", "tactical_texture", "damage_view", "weapon_arc"] },
  { file: "FIGHTSD", key: "fighters", family: "fighter", roles: ["strategic_mini_icon", "encyclopedia_image", "tactical_bitmap", "tactical_mesh", "tactical_texture", "squadron_state"] },
  { file: "TROOPSD", key: "troops", family: "troop", roles: ["strategic_mini_icon", "status_view", "encyclopedia_image", "ground_report_art"] },
  { file: "SPECFCSD", key: "units", family: "special_force", roles: ["strategic_mini_icon", "status_view", "encyclopedia_image", "mission_art"] },
  { file: "PROFACSD", key: "facilities", family: "production_facility", roles: ["system_mini_icon", "status_view", "encyclopedia_image", "construction_state"] },
  { file: "MANFACSD", key: "facilities", family: "manufacturing_facility", roles: ["system_mini_icon", "status_view", "encyclopedia_image", "construction_state"] },
  { file: "DEFFACSD", key: "facilities", family: "defense_facility", roles: ["system_mini_icon", "status_view", "encyclopedia_image", "construction_state"] },
  { file: "MJCHARSD", key: "characters", family: "major_character", roles: ["portrait", "strategic_mini_icon", "encyclopedia_image", "event_variant"] },
  { file: "MNCHARSD", key: "characters", family: "minor_character", roles: ["portrait", "strategic_mini_icon", "encyclopedia_image", "event_variant"] },
  { file: "SYSTEMSD", key: "systems", family: "star_system", roles: ["galaxy_marker", "system_window_art", "encyclopedia_image", "tactical_planet", "event_variant"] },
];

function relation(role, resourceKey, evidence, grade = "source-derived") {
  assert.ok(resourceKeys.has(resourceKey), `staged BMP absent: ${resourceKey}`);
  return { role, resource_key: resourceKey, grade, evidence };
}

const classBitmapEvidence = "ghidra/notes/FUN_0042c3b0.c; ghidra/notes/FUN_00437880.c";
const encyclopediaKeyEvidence = "ghidra/notes/FUN_0045d400_encyclopedia_loader.c";

function sourceRelations(classKey) {
  return [
    relation("class_bitmap", `gokres-dll/BMP/${classKey}`, classBitmapEvidence),
    relation("class_miniature", `gokres-dll/BMP/${classKey + 0x4000}`, classBitmapEvidence),
  ];
}

const entities = families.flatMap(({ file, key, family, roles }) => {
  const data = read(`data/base/json/${file}.json`);
  assert.equal(data[key].length, data.count, `${file} count mismatch`);
  return data[key].map((record) => {
    const classKey = family === "star_system" ? null : record.text_stra_dll_id & 0x0fff;
    const graphics = classKey === null ? [] : sourceRelations(classKey);
    const expectedRoles = classKey === null ? roles : [...roles, "class_bitmap", "class_miniature"];
    const result = {
      family,
      dat_id: record.id,
      name: names[String(record.text_stra_dll_id)] ?? null,
      faction: family === "star_system" ? "neutral" : record.is_alliance && record.is_empire ? "both" : record.is_alliance ? "alliance" : record.is_empire ? "empire" : "unspecified",
      source: `data/base/json/${file}.json`,
      text_resource_id: record.text_stra_dll_id,
      class_resource_key: classKey,
      encyclopedia_lookup_key: classKey === null ? null : classKey + 0x1000,
      encyclopedia_lookup_evidence: classKey === null ? null : encyclopediaKeyEvidence,
      graphics,
      unresolved_roles: expectedRoles.filter((role) => !graphics.some((graphic) => graphic.role === role)),
    };
    if (family === "star_system") result.picture_id = record.picture_id;
    if (family !== "star_system") result.research = { order: record.research_order ?? 0, difficulty: record.research_difficulty ?? 0 };
    return result;
  });
});

const counts = Object.fromEntries(families.map(({ family }) => [family, entities.filter((entity) => entity.family === family).length]));
const document = {
  schema_version: 1,
  scope: "All decoded base-game entity records; GOKRES class and miniature IDs are source-derived and whole-family validated, while the DAT-to-runtime-field initialization edge and tactical/state/encyclopedia pixels remain unresolved",
  evidence_grades: ["source-proven", "source-derived", "runtime-corroborated", "implementation-only", "unmapped"],
  missing_source_join: "DAT text_stra_dll_id to native runtime class resource field read by FUN_0042c3b0 and FUN_00437880",
  entity_count: entities.length,
  family_counts: counts,
  researchable_entity_count: entities.filter((entity) => entity.research?.order > 0).length,
  source_derived_graphic_relation_count: entities.reduce((count, entity) => count + entity.graphics.filter((graphic) => graphic.grade === "source-derived").length, 0),
  entities,
};
assert.equal(entities.filter((entity) => entity.class_resource_key !== null).length, 131);
assert.equal(new Set(entities.map((entity) => entity.class_resource_key).filter((key) => key !== null)).size, 131);
assert.equal(document.source_derived_graphic_relation_count, 262);
const contents = `${JSON.stringify(document, null, 2)}\n`;
if (check) {
  assert.equal(fs.readFileSync(output, "utf8"), contents, "entity-catalog.json is stale");
  process.stdout.write("Checked entity-catalog.json\n");
} else {
  fs.mkdirSync(path.dirname(output), { recursive: true });
  fs.writeFileSync(output, contents);
  process.stdout.write(`Wrote entity-catalog.json: ${entities.length} entities\n`);
}
