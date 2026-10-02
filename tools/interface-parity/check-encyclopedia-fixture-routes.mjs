#!/usr/bin/env node

import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const rustPath = path.join(root, "crates/rebellion-app/src/interface_test_fixture.rs");
const scenarioPaths = {
  base: path.join(here, "scenarios/encyclopedia-base.json"),
  loose: path.join(here, "scenarios/encyclopedia-loose.json"),
  surface: path.join(here, "scenarios/encyclopedia-surface.json"),
};
const targetScenarios = [
  "EncyclopediaIndexCatalog",
  "PackedEncyclopedia",
  "LooseEncyclopedia",
];
const factions = { alliance: 1, empire: 2 };

function readJson(file) {
  return JSON.parse(fs.readFileSync(file, "utf8"));
}

function parseRustRoutes(source) {
  const enumBody = source.match(/pub enum Scenario\s*\{([\s\S]*?)\n\}/)?.[1];
  assert.ok(enumBody, "Rust Scenario enum not found");
  const discriminants = new Map(
    [...enumBody.matchAll(/^\s*([A-Za-z][A-Za-z0-9_]*)\s*=\s*(\d+)\s*,\s*$/gm)]
      .map(([, name, value]) => [name, Number(value)]),
  );

  const decodeBody = source.match(/fn decode\(value: u8\) -> Option<Self>\s*\{[\s\S]*?Some\(match value\s*\{([\s\S]*?)\n\s*_ => return None,/m)?.[1];
  assert.ok(decodeBody, "Rust Scenario::decode match not found");
  const decoded = new Map(
    [...decodeBody.matchAll(/^\s*(\d+)\s*=>\s*Self::([A-Za-z][A-Za-z0-9_]*)\s*,\s*$/gm)]
      .map(([, value, name]) => [name, Number(value)]),
  );

  const decodeRequest = source.match(/pub\(crate\) fn decode_request\(code: u32\)[\s\S]*?\n\}/)?.[0];
  assert.ok(decodeRequest, "Rust decode_request function not found");
  assert.match(
    decodeRequest,
    /Scenario::decode\(\(\(code\s*&\s*0xff\)\s*as\s*u8\)\.checked_sub\(1\)\?\)\?/,
    "decode_request must subtract one from the request low byte before Scenario::decode",
  );
  assert.match(
    decodeRequest,
    /match\s*\(code\s*>>\s*8\)\s*&\s*0xff\s*\{[\s\S]*?1\s*=>\s*CockpitFaction::Alliance,[\s\S]*?2\s*=>\s*CockpitFaction::Empire,/,
    "decode_request must derive Alliance/Empire from request byte one",
  );

  for (const name of targetScenarios) {
    assert.ok(discriminants.has(name), `Rust Scenario::${name} discriminant not found`);
    assert.equal(
      decoded.get(name),
      discriminants.get(name),
      `Rust Scenario::decode mapping for ${name}`,
    );
  }
  assert.equal(
    discriminants.get("EncyclopediaIndexCatalog") + 1,
    42,
    "P65 EncyclopediaIndexCatalog must retain request low byte 42",
  );
  return Object.fromEntries(targetScenarios.map((name) => [name, discriminants.get(name)]));
}

function encodedRoutes(discriminants) {
  return Object.fromEntries(targetScenarios.map((name) => [
    name,
    Object.fromEntries(Object.entries(factions).map(([faction, factionByte]) => [
      faction,
      (factionByte << 8) | (discriminants[name] + 1),
    ])),
  ]));
}

function validateScenarioRoutes(routes, scenarios) {
  const expected = {
    "encyclopedia-base.json": [scenarios.base.fixture_codes, routes.PackedEncyclopedia],
    "encyclopedia-loose.json": [scenarios.loose.fixture_codes, routes.LooseEncyclopedia],
    "encyclopedia-surface.json packed": [scenarios.surface.fixture_codes?.packed, routes.PackedEncyclopedia],
    "encyclopedia-surface.json loose": [scenarios.surface.fixture_codes?.loose, routes.LooseEncyclopedia],
  };
  const failures = [];
  for (const [label, [actual, wanted]] of Object.entries(expected)) {
    for (const faction of Object.keys(factions)) {
      if (actual?.[faction] !== wanted[faction]) {
        failures.push(`${label} ${faction}: expected ${wanted[faction]}, found ${actual?.[faction]}`);
      }
    }
  }
  if (failures.length > 0) throw new Error(`encyclopedia fixture route mismatch:\n- ${failures.join("\n- ")}`);
}

export function checkRoutes({ adversarial = false } = {}) {
  const discriminants = parseRustRoutes(fs.readFileSync(rustPath, "utf8"));
  const routes = encodedRoutes(discriminants);
  const scenarios = Object.fromEntries(
    Object.entries(scenarioPaths).map(([name, file]) => [name, readJson(file)]),
  );
  validateScenarioRoutes(routes, scenarios);

  if (adversarial) {
    const reverted = structuredClone(scenarios);
    reverted.surface.fixture_codes.packed.alliance = routes.PackedEncyclopedia.alliance - 1;
    assert.throws(
      () => validateScenarioRoutes(routes, reverted),
      /encyclopedia-surface\.json packed alliance/,
      "reverting one surface route to the preceding Rust scenario must fail",
    );
  }
  return { adversarial_revert_rejected: adversarial, discriminants, routes };
}

if (path.resolve(process.argv[1] || "") === fileURLToPath(import.meta.url)) {
  try {
    const unknown = process.argv.slice(2).filter((value) => value !== "--self-test");
    assert.deepEqual(unknown, [], `unknown arguments: ${unknown.join(" ")}`);
    const result = checkRoutes({ adversarial: process.argv.includes("--self-test") });
    console.log(`PASS: Rust/JSON encyclopedia fixture routes ${JSON.stringify(result)}`);
  } catch (error) {
    console.error(error.stack || error.message);
    process.exitCode = 1;
  }
}
