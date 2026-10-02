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
  "fixture-code",
  "__openRebellionInterfaceReady",
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
  "packed-encyclopedia-fixture",
  "Synthetic encyclopedia index",
  "REBELLION_ENCYCLOPEDIA_INSPECTOR",
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
  for (const required of [
    "open_rebellion_interface_fixture",
    "packed-encyclopedia-fixture",
  ]) {
    if (!bytes.includes(Buffer.from(required))) {
      throw new Error(`test WASM does not contain required fixture token ${required}`);
    }
  }
}

process.stdout.write(`${JSON.stringify({
  status: "pass",
  production_fixture_tokens: 0,
  test_fixture_bridge: fs.existsSync(testWasm),
}, null, 2)}\n`);
