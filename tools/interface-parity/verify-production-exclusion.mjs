#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const productionWasm = path.join(root, "web/open-rebellion.wasm");
const productionHtml = path.join(root, "web/index.html");
const forbidden = [
  "open_rebellion_interface_fixture",
  "open_rebellion_windowing_fixture_command",
  "fixture-code",
  "__openRebellionInterfaceReady",
  "__openRebellionWindowingFixtureCommand",
  "[windowing-input-scenes-action]",
  "[windowing-fixture-command]",
  "gid/alliance/popular-support",
  "tactical_test_fixture",
  "enable_resource_2560_proof",
  "set_tactical_lod_fixture",
  "enable_tactical_lod_journey",
  "[tactical_3d] rendered",
  "[tactical_3d] family_loaded",
  "[tactical_3d] lod_selection",
  "battle-ready",
  "Tactical test launcher",
];

if (!fs.existsSync(productionWasm)) {
  throw new Error("production WASM is missing; run npm run build first");
}
for (const file of [productionWasm, productionHtml]) {
  const bytes = fs.readFileSync(file);
  for (const needle of forbidden) {
    if (bytes.includes(Buffer.from(needle))) {
      throw new Error(`${path.relative(root, file)} contains forbidden fixture token ${needle}`);
    }
  }
}

const testWasm = path.join(root, ".artifacts/interface-parity/site/open-rebellion-test.wasm");
if (fs.existsSync(testWasm)) {
  const bytes = fs.readFileSync(testWasm);
  for (const needle of [
    "open_rebellion_interface_fixture",
    "open_rebellion_windowing_fixture_command",
  ]) {
    if (!bytes.includes(Buffer.from(needle))) {
      throw new Error(`test WASM does not expose fixture bridge ${needle}`);
    }
  }
}

process.stdout.write(`${JSON.stringify({
  status: "pass",
  production_fixture_tokens: 0,
  test_fixture_bridge: fs.existsSync(testWasm),
}, null, 2)}\n`);
