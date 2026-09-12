#!/usr/bin/env node
// Generate metadata-only resource indexes from a contributor-owned installation.

import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const outputDir = path.join(root, "docs/reference/asset-library");
const args = process.argv.slice(2);
const check = args.includes("--check");
const option = (name, fallback) => {
  const value = args.find((arg) => arg.startsWith(`${name}=`));
  return path.resolve(root, value ? value.slice(name.length + 1) : fallback);
};
const uiDir = option("--ui", "data/base/ui");
const edataArg = args.find((arg) => arg.startsWith("--edata="));
const edataDir = edataArg ? option("--edata", "") : null;
const expected = new Map([
  ["common-dll", [321, 0]],
  ["gokres-dll", [580, 0]],
  ["strategy-dll", [1042, 0]],
  ["tactical-dll", [288, 0]],
  ["alsprite-dll", [38, 1640]],
  ["emsprite-dll", [34, 2348]],
]);

function bitmap(pathname, id) {
  const bytes = fs.readFileSync(pathname);
  assert.ok(bytes.length >= 54 && bytes.toString("ascii", 0, 2) === "BM", `invalid BMP: ${pathname}`);
  const width = bytes.readInt32LE(18);
  const height = Math.abs(bytes.readInt32LE(22));
  const bitsPerPixel = bytes.readUInt16LE(28);
  assert.ok(width > 0 && height > 0, `invalid BMP dimensions: ${pathname}`);
  return { id, width, height, bits_per_pixel: bitsPerPixel, bytes: bytes.length };
}

function numberedFiles(dir, expression) {
  return fs.readdirSync(dir)
    .map((name) => ({ name, match: expression.exec(name) }))
    .filter(({ match }) => match)
    .map(({ name, match }) => ({ name, id: Number(match[1]) }))
    .sort((a, b) => a.id - b.id);
}

function resourceFamily(name) {
  const [bmpExpected, framesExpected] = expected.get(name);
  const bmpDir = path.join(uiDir, name, "BMP");
  const bmps = numberedFiles(bmpDir, /^(\d+)\.bmp$/);
  assert.equal(bmps.length, bmpExpected, `${name} BMP count`);
  const bitmaps = bmps.map(({ name: file, id }) => bitmap(path.join(bmpDir, file), id));
  const frameDir = path.join(uiDir, name, "TYPE302");
  const frames = framesExpected === 0 ? [] : numberedFiles(frameDir, /^(\d+)\.bin$/)
    .map(({ name: file, id }) => ({ id, bytes: fs.statSync(path.join(frameDir, file)).size }));
  assert.equal(frames.length, framesExpected, `${name} type-302 count`);
  return { module: name, bitmap_count: bitmaps.length, type302_count: frames.length, bitmaps, type302_frames: frames };
}

function writeOrCheck(name, value) {
  const target = path.join(outputDir, name);
  const contents = `${JSON.stringify(value, null, 2)}\n`;
  if (check) {
    assert.equal(fs.readFileSync(target, "utf8"), contents, `${name} is stale`);
  } else {
    fs.mkdirSync(outputDir, { recursive: true });
    fs.writeFileSync(target, contents);
  }
  process.stdout.write(`${check ? "Checked" : "Wrote"} ${name}\n`);
}

writeOrCheck("resource-inventory.json", {
  schema_version: 1,
  scope: "metadata for six locally staged original DLL resource families; IDs are not semantic mappings",
  key_format: "module/resource_type/id",
  families: [...expected.keys()].map(resourceFamily),
});

if (edataDir) {
  const files = numberedFiles(edataDir, /^EDATA\.(\d+)$/);
  writeOrCheck("edata-inventory.json", {
    schema_version: 1,
    scope: "metadata for EData files in the contributor-owned GOG installation; IDs are not semantic mappings",
    key_format: "EData/EDATA.NNN",
    count: files.length,
    bitmaps: files.map(({ name, id }) => bitmap(path.join(edataDir, name), id)),
  });
}
