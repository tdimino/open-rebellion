#!/usr/bin/env node

import assert from "node:assert/strict";
import crypto from "node:crypto";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { TextDecoder } from "node:util";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020.js";
import { PNG } from "pngjs";
import { generateFixtures } from "../../tests/fixtures/encyclopedia/generators/generate.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const corpusRoot = path.join(root, "tests/fixtures/encyclopedia");
const fixtureRoot = path.join(corpusRoot, "fixtures");
const casesPath = path.join(corpusRoot, "cases.json");
const schemaRoot = path.join(root, "docs/reference/asset-library/schemas");
const cases = JSON.parse(fs.readFileSync(casesPath, "utf8"));
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const sha256 = (bytes) => crypto.createHash("sha256").update(bytes).digest("hex");
const clone = (value) => structuredClone(value);

class Diagnostic extends Error {
  constructor(code, message = code) {
    super(message);
    this.code = code;
  }
}

const ajv = new Ajv2020({ allErrors: true, strict: true });
const validators = {
  "ajv-catalog": ajv.compile(readJson(path.join(schemaRoot, "encyclopedia-catalog.schema.json"))),
  "ajv-manifest": ajv.compile(readJson(path.join(schemaRoot, "encyclopedia-manifest.schema.json"))),
  "ajv-overlay": ajv.compile(readJson(path.join(schemaRoot, "encyclopedia-overlay.schema.json"))),
};

function crc32(bytes) {
  let crc = 0xffffffff;
  for (const byte of bytes) {
    crc ^= byte;
    for (let bit = 0; bit < 8; bit += 1) {
      crc = (crc >>> 1) ^ (0xedb88320 & -(crc & 1));
    }
  }
  return (crc ^ 0xffffffff) >>> 0;
}

function detectedImageFormat(bytes) {
  if (bytes.length >= 2 && bytes[0] === 0x42 && bytes[1] === 0x4d) return "bmp";
  if (bytes.length >= 8 && bytes.subarray(0, 8).equals(Buffer.from("89504e470d0a1a0a", "hex"))) return "png";
  return null;
}

function inspectBmp(bytes) {
  if (bytes.length < 54 || bytes.toString("ascii", 0, 2) !== "BM") {
    throw new Diagnostic("invalid_image", "BMP header is missing or truncated");
  }
  const fileSize = bytes.readUInt32LE(2);
  const pixelOffset = bytes.readUInt32LE(10);
  const dibSize = bytes.readUInt32LE(14);
  const width = bytes.readInt32LE(18);
  const signedHeight = bytes.readInt32LE(22);
  const planes = bytes.readUInt16LE(26);
  const bitsPerPixel = bytes.readUInt16LE(28);
  const compression = bytes.readUInt32LE(30);
  if (fileSize !== bytes.length || dibSize !== 40 || width <= 0 || signedHeight === 0
      || planes !== 1 || bitsPerPixel !== 24 || compression !== 0 || pixelOffset < 54) {
    throw new Diagnostic("invalid_image", "synthetic BMP metadata is inconsistent");
  }
  const height = Math.abs(signedHeight);
  const pixels = BigInt(width) * BigInt(height);
  if (pixels > BigInt(cases.limits.image_pixels)) {
    throw new Diagnostic("resource_limit:image_pixels");
  }
  const stride = (width * 3 + 3) & ~3;
  const required = BigInt(pixelOffset) + BigInt(stride) * BigInt(height);
  if (required !== BigInt(bytes.length)) {
    throw new Diagnostic("invalid_image", "synthetic BMP pixel plane is truncated or has trailing bytes");
  }
  const rgba = Buffer.alloc(Number(pixels) * 4);
  for (let y = 0; y < height; y += 1) {
    const sourceY = signedHeight > 0 ? height - y - 1 : y;
    for (let x = 0; x < width; x += 1) {
      const source = pixelOffset + sourceY * stride + x * 3;
      const destination = (y * width + x) * 4;
      rgba[destination] = bytes[source + 2];
      rgba[destination + 1] = bytes[source + 1];
      rgba[destination + 2] = bytes[source];
      rgba[destination + 3] = 0xff;
    }
  }
  return { format: "bmp", width, height, rgba };
}

let pngDecodeCalls = 0;

function inspectPng(bytes) {
  const signature = Buffer.from("89504e470d0a1a0a", "hex");
  if (bytes.length < signature.length || !bytes.subarray(0, 8).equals(signature)) {
    throw new Diagnostic("invalid_image", "PNG signature is missing");
  }
  let position = 8;
  let sawHeader = false;
  let sawEnd = false;
  let headerWidth;
  let headerHeight;
  while (position < bytes.length) {
    if (position + 12 > bytes.length) throw new Diagnostic("invalid_image", "PNG chunk header is truncated");
    const length = bytes.readUInt32BE(position);
    const end = position + 12 + length;
    if (end > bytes.length) throw new Diagnostic("invalid_image", "PNG chunk payload is truncated");
    const kind = bytes.toString("ascii", position + 4, position + 8);
    const content = bytes.subarray(position + 8, position + 8 + length);
    const declaredCrc = bytes.readUInt32BE(position + 8 + length);
    const actualCrc = crc32(Buffer.concat([Buffer.from(kind, "ascii"), content]));
    if (declaredCrc !== actualCrc) throw new Diagnostic("invalid_image", `PNG ${kind} CRC is invalid`);
    if (!sawHeader && kind !== "IHDR") throw new Diagnostic("invalid_image", "PNG IHDR is not first");
    if (kind === "IHDR") {
      if (sawHeader || length !== 13) throw new Diagnostic("invalid_image", "PNG IHDR is invalid");
      headerWidth = content.readUInt32BE(0);
      headerHeight = content.readUInt32BE(4);
      if (headerWidth === 0 || headerHeight === 0) {
        throw new Diagnostic("invalid_image", "PNG IHDR dimensions must be nonzero");
      }
      const headerPixels = BigInt(headerWidth) * BigInt(headerHeight);
      if (headerPixels > BigInt(cases.limits.image_pixels)) {
        throw new Diagnostic("resource_limit:image_pixels");
      }
      sawHeader = true;
    }
    if (kind === "IEND") {
      if (length !== 0 || end !== bytes.length) throw new Diagnostic("invalid_image", "PNG IEND is invalid");
      sawEnd = true;
    }
    position = end;
  }
  if (!sawHeader || !sawEnd) throw new Diagnostic("invalid_image", "PNG stream is unterminated");
  try {
    pngDecodeCalls += 1;
    const decoded = PNG.sync.read(bytes, { checkCRC: true });
    assert.equal(decoded.width, headerWidth);
    assert.equal(decoded.height, headerHeight);
    assert.equal(decoded.data.length, decoded.width * decoded.height * 4);
    return { format: "png", width: decoded.width, height: decoded.height, rgba: decoded.data };
  } catch (error) {
    if (error instanceof Diagnostic) throw error;
    throw new Diagnostic("invalid_image", `PNG full decode failed: ${error.message}`);
  }
}

function inspectImage(bytes, declaredFormat) {
  if (bytes.length > cases.limits.image_bytes) throw new Diagnostic("resource_limit:image_bytes");
  const detected = detectedImageFormat(bytes);
  if (detected === null) throw new Diagnostic("invalid_image", "image magic is unsupported");
  if (detected !== declaredFormat) throw new Diagnostic("image_format_mismatch");
  return detected === "bmp" ? inspectBmp(bytes) : inspectPng(bytes);
}

class RawJsonParser {
  constructor(text, maxDepth = Number.POSITIVE_INFINITY) {
    this.text = text;
    this.position = 0;
    this.maxDepth = maxDepth;
  }

  parse() {
    this.skipWhitespace();
    this.parseValue(1);
    this.skipWhitespace();
    if (this.position !== this.text.length) throw new Diagnostic("invalid_json", "trailing JSON bytes");
  }

  skipWhitespace() {
    while (/[\t\n\r ]/.test(this.text[this.position] ?? "")) this.position += 1;
  }

  parseValue(depth) {
    this.skipWhitespace();
    const token = this.text[this.position];
    if (token === "{") return this.parseObject(depth);
    if (token === "[") return this.parseArray(depth);
    if (token === '"') return this.parseString();
    const remainder = this.text.slice(this.position);
    const primitive = /^(?:-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?|true|false|null)/.exec(remainder);
    if (!primitive) throw new Diagnostic("invalid_json", `unexpected token at byte ${this.position}`);
    this.position += primitive[0].length;
    return undefined;
  }

  checkDepth(depth) {
    if (depth > this.maxDepth) throw new Diagnostic("resource_limit:json_depth");
  }

  parseObject(depth) {
    this.checkDepth(depth);
    this.position += 1;
    this.skipWhitespace();
    const keys = new Set();
    if (this.text[this.position] === "}") {
      this.position += 1;
      return;
    }
    while (true) {
      if (this.text[this.position] !== '"') throw new Diagnostic("invalid_json", "object key is not a string");
      const key = this.parseString();
      if (keys.has(key)) throw new Diagnostic("duplicate_key", `duplicate JSON key ${JSON.stringify(key)}`);
      keys.add(key);
      this.skipWhitespace();
      if (this.text[this.position] !== ":") throw new Diagnostic("invalid_json", "object colon is missing");
      this.position += 1;
      this.parseValue(depth + 1);
      this.skipWhitespace();
      if (this.text[this.position] === "}") {
        this.position += 1;
        return;
      }
      if (this.text[this.position] !== ",") throw new Diagnostic("invalid_json", "object comma is missing");
      this.position += 1;
      this.skipWhitespace();
    }
  }

  parseArray(depth) {
    this.checkDepth(depth);
    this.position += 1;
    this.skipWhitespace();
    if (this.text[this.position] === "]") {
      this.position += 1;
      return;
    }
    while (true) {
      this.parseValue(depth + 1);
      this.skipWhitespace();
      if (this.text[this.position] === "]") {
        this.position += 1;
        return;
      }
      if (this.text[this.position] !== ",") throw new Diagnostic("invalid_json", "array comma is missing");
      this.position += 1;
      this.skipWhitespace();
    }
  }

  parseString() {
    const start = this.position;
    this.position += 1;
    let escaped = false;
    while (this.position < this.text.length) {
      const character = this.text[this.position];
      this.position += 1;
      if (escaped) {
        escaped = false;
      } else if (character === "\\") {
        escaped = true;
      } else if (character === '"') {
        try {
          return JSON.parse(this.text.slice(start, this.position));
        } catch {
          throw new Diagnostic("invalid_json", "JSON string escape is invalid");
        }
      }
    }
    throw new Diagnostic("invalid_json", "JSON string is unterminated");
  }
}

function scanRawJson(bytes, maxDepth = Number.POSITIVE_INFINITY) {
  let text;
  try {
    text = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
  } catch {
    throw new Diagnostic("invalid_utf8");
  }
  new RawJsonParser(text, maxDepth).parse();
}

const validCatalog = readJson(path.join(fixtureRoot, "bundles/valid/catalog.json"));

function referencedImageIds(catalog) {
  const result = [];
  for (const topic of Object.values(catalog.topics)) {
    for (const localized of Object.values(topic.localized)) {
      if (typeof localized.image_id === "string") result.push(localized.image_id);
      if (localized.image_selector) {
        for (const side of ["alliance_image_id", "empire_image_id"]) {
          if (typeof localized.image_selector[side] === "string") result.push(localized.image_selector[side]);
        }
      }
    }
  }
  return result;
}

function checkCatalogRelationships(catalog) {
  if (!validators["ajv-catalog"](catalog)) throw new Diagnostic("schema_structure");
  const topicIds = new Set(Object.keys(catalog.topics));
  for (const topicId of [...catalog.index.topic_ids, ...catalog.categories.flatMap((category) => category.topic_ids)]) {
    if (!topicIds.has(topicId)) throw new Diagnostic("dangling_topic_reference");
  }
  const commands = catalog.categories.map((category) => category.command);
  if (new Set(commands).size !== commands.length) throw new Diagnostic("duplicate_category_command");
  const expectedCommands = validCatalog.categories.map((category) => category.command);
  if (JSON.stringify(commands) !== JSON.stringify(expectedCommands)) throw new Diagnostic("category_order");
  if (JSON.stringify(catalog.index.topic_ids) !== JSON.stringify(validCatalog.index.topic_ids)) {
    throw new Diagnostic("potential_membership_mismatch");
  }
  const membership = catalog.categories.map((category) => category.topic_ids);
  const expectedMembership = validCatalog.categories.map((category) => category.topic_ids);
  if (JSON.stringify(membership) !== JSON.stringify(expectedMembership)) {
    throw new Diagnostic("filtered_membership_mismatch");
  }
  const bindingKeys = new Set();
  const bindingByTopic = new Map();
  for (const binding of catalog.bindings) {
    if (!topicIds.has(binding.topic_id)) throw new Diagnostic("dangling_topic_reference");
    const key = `${binding.family}\u0000${binding.dat_id}\u0000${binding.variant}`;
    if (bindingKeys.has(key)) throw new Diagnostic("ambiguous_binding");
    bindingKeys.add(key);
    bindingByTopic.set(binding.topic_id, binding);
  }
  const images = new Set(Object.keys(catalog.images));
  for (const imageId of referencedImageIds(catalog)) {
    if (!images.has(imageId)) throw new Diagnostic("dangling_image_reference");
  }
  for (const [topicId, topic] of Object.entries(catalog.topics)) {
    const binding = bindingByTopic.get(topicId);
    assert.ok(binding, `${topicId} must have a synthetic binding`);
    const selectorShapes = Object.values(topic.localized).map((localized) => Boolean(localized.image_selector));
    const hasSelector = selectorShapes.some(Boolean);
    if ((binding.variant === "viewer_faction") !== hasSelector
        || selectorShapes.some((shape) => shape !== hasSelector)) {
      throw new Diagnostic("binding_selector_mismatch");
    }
  }
}

function checkFamilyQualifiedIdentity(catalog) {
  const rows = catalog.bindings.filter((binding) => binding.dat_id === 7);
  assert.equal(rows.length, 2, "synthetic corpus must exercise equal numeric DAT IDs in two families");
  assert.deepEqual(rows.map((row) => row.family), ["system_locations", "capital_ship_classes"]);
}

function checkCatalogFeature(catalog, feature) {
  const topic = (id) => catalog.topics[id];
  switch (feature) {
    case "aggregate-only": {
      const id = "original:60007";
      assert.ok(catalog.index.topic_ids.includes(id));
      assert.equal(catalog.categories.some((category) => category.topic_ids.includes(id)), false);
      return;
    }
    case "multiple-langids":
      assert.deepEqual(Object.keys(topic("original:60001").localized), ["1033", "1036"]);
      return;
    case "shared-art":
      assert.equal(topic("original:60002").localized["1033"].image_id, "edata:2");
      assert.equal(topic("original:60003").localized["1033"].image_id, "edata:2");
      return;
    case "no-art":
      assert.equal(Object.hasOwn(topic("original:60005").localized["1033"], "image_id"), false);
      assert.equal(topic("original:60006").localized["1033"].image_id, null);
      return;
    case "viewer-faction": {
      const selector = topic("original:60004").localized["1033"].image_selector;
      assert.deepEqual(Object.keys(selector), ["kind", "alliance_image_id", "empire_image_id"]);
      assert.equal(catalog.bindings.find((binding) => binding.topic_id === "original:60004").variant, "viewer_faction");
      return;
    }
    default:
      throw new Error(`unknown catalog feature ${feature}`);
  }
}

function sortedEqual(left, right) {
  return JSON.stringify([...left].sort()) === JSON.stringify([...right].sort());
}

function catalogSourceRefs(catalog) {
  return [
    catalog.index.source_ref,
    ...catalog.categories.map((category) => category.source_ref),
    ...Object.values(catalog.topics).map((topic) => topic.source_ref),
    ...Object.values(catalog.images).map((image) => image.source_ref),
  ];
}

function checkBundle(bundleRoot) {
  const catalogBytes = fs.readFileSync(path.join(bundleRoot, "catalog.json"));
  const manifestBytes = fs.readFileSync(path.join(bundleRoot, "manifest.json"));
  const catalog = JSON.parse(catalogBytes);
  const manifest = JSON.parse(manifestBytes);
  if (!validators["ajv-catalog"](catalog) || !validators["ajv-manifest"](manifest)) {
    throw new Diagnostic("schema_structure");
  }
  const catalogDigest = sha256(catalogBytes);
  if (manifest.catalog_sha256 !== catalogDigest || manifest.files["catalog.json"] !== catalogDigest) {
    throw new Diagnostic("catalog_digest_mismatch");
  }
  const expectedFiles = ["catalog.json", ...Object.values(catalog.images).map((image) => image.path)];
  if (!sortedEqual(Object.keys(manifest.files), expectedFiles)) {
    throw new Diagnostic("manifest_file_set_mismatch");
  }
  for (const [relative, digest] of Object.entries(manifest.files)) {
    const bytes = fs.readFileSync(path.join(bundleRoot, relative));
    if (sha256(bytes) !== digest) throw new Diagnostic("file_digest_mismatch");
  }
  for (const source of manifest.binding_sources) {
    const sourceBytes = fs.readFileSync(path.join(bundleRoot, "sources", source.basename));
    if (sha256(sourceBytes) !== source.sha256) throw new Diagnostic("binding_source_mismatch");
  }
  for (const sourceRef of catalogSourceRefs(catalog)) {
    if (!Object.hasOwn(manifest.source_records, sourceRef)) throw new Diagnostic("dangling_source_ref");
  }
  for (const image of Object.values(catalog.images)) {
    if (image.format !== "bmp") throw new Diagnostic("unsupported_base_image_format");
    const bytes = fs.readFileSync(path.join(bundleRoot, image.path));
    const inspected = inspectImage(bytes, image.format);
    if (image.sha256 !== sha256(bytes)) throw new Diagnostic("image_digest_mismatch");
    if (image.byte_length !== bytes.length || image.width !== inspected.width || image.height !== inspected.height) {
      throw new Diagnostic("image_facts_mismatch");
    }
  }
  checkCatalogRelationships(catalog);
}

let maximumBoundaryBuffer;
function boundaryResult(boundary) {
  const acceptLimit = (value, limit, code) => {
    if (BigInt(value) > BigInt(limit)) throw new Diagnostic(code);
  };
  switch (boundary.kind) {
    case "body_ascii_bytes": {
      const body = "a".repeat(boundary.value);
      acceptLimit(Buffer.byteLength(body, "utf8"), cases.limits.localized_body_utf8_bytes, "resource_limit:body_bytes");
      const catalog = clone(validCatalog);
      catalog.topics["original:60001"].localized["1033"].body = body;
      if (!validators["ajv-catalog"](catalog)) throw new Diagnostic("resource_limit:body_bytes");
      return;
    }
    case "body_utf8_bytes": {
      assert.equal(boundary.value % 2, 0);
      const body = "é".repeat(boundary.value / 2);
      assert.equal(Buffer.byteLength(body, "utf8"), boundary.value);
      acceptLimit(boundary.value, cases.limits.localized_body_utf8_bytes, "resource_limit:body_bytes");
      return;
    }
    case "title_ascii_bytes": {
      const title = "t".repeat(boundary.value);
      acceptLimit(Buffer.byteLength(title, "utf8"), cases.limits.title_or_label_utf8_bytes, "resource_limit:title_bytes");
      const catalog = clone(validCatalog);
      catalog.topics["original:60001"].localized["1033"].title = title;
      if (!validators["ajv-catalog"](catalog)) throw new Diagnostic("resource_limit:title_bytes");
      return;
    }
    case "label_ascii_bytes": {
      const label = "l".repeat(boundary.value);
      acceptLimit(Buffer.byteLength(label, "utf8"), cases.limits.title_or_label_utf8_bytes, "resource_limit:label_bytes");
      const catalog = clone(validCatalog);
      catalog.index.labels["1033"] = label;
      if (!validators["ajv-catalog"](catalog)) throw new Diagnostic("resource_limit:label_bytes");
      return;
    }
    case "topic_count": {
      const catalog = clone(validCatalog);
      const template = catalog.topics["original:60001"];
      catalog.topics = {};
      for (let index = 0; index < boundary.value; index += 1) catalog.topics[`original:${index + 1}`] = template;
      if (!validators["ajv-catalog"](catalog)) throw new Diagnostic("resource_limit:topics");
      return;
    }
    case "overlay_patch_count": {
      const patch = { id: "original:60001", localized: { "1033": { body: "Synthetic." } } };
      const overlay = Array(boundary.value).fill(patch);
      if (!validators["ajv-overlay"](overlay)) throw new Diagnostic("resource_limit:overlay_patches");
      return;
    }
    case "image_bytes":
      acceptLimit(boundary.value, cases.limits.image_bytes, "resource_limit:image_bytes");
      return;
    case "image_pixels":
      acceptLimit(BigInt(boundary.width) * BigInt(boundary.height), cases.limits.image_pixels, "resource_limit:image_pixels");
      return;
    case "effective_image_bytes":
      acceptLimit(boundary.values.reduce((sum, value) => sum + BigInt(value), 0n), cases.limits.effective_image_bytes, "resource_limit:effective_image_bytes");
      return;
    case "retained_bytes":
      acceptLimit(boundary.values.reduce((sum, value) => sum + BigInt(value), 0n), cases.limits.retained_bytes, "resource_limit:retained_bytes");
      return;
    case "json_bytes": {
      if (!maximumBoundaryBuffer) maximumBoundaryBuffer = Buffer.alloc(cases.limits.catalog_json_bytes + 1);
      const limits = {
        catalog: cases.limits.catalog_json_bytes,
        manifest: cases.limits.manifest_json_bytes,
        overlay: cases.limits.overlay_json_bytes,
      };
      const bytes = maximumBoundaryBuffer.subarray(0, boundary.value);
      acceptLimit(bytes.length, limits[boundary.document], "resource_limit:json_bytes");
      return;
    }
    case "json_depth": {
      const limits = {
        catalog: cases.limits.catalog_json_depth,
        manifest: cases.limits.manifest_json_depth,
        overlay: cases.limits.overlay_json_depth,
      };
      scanRawJson(Buffer.from(`${"[".repeat(boundary.value)}0${"]".repeat(boundary.value)}`), limits[boundary.document]);
      return;
    }
    default:
      throw new Error(`unknown boundary kind ${boundary.kind}`);
  }
}

function executeProbe(testCase) {
  try {
    if (testCase.executed_by in validators) {
      const value = readJson(path.join(fixtureRoot, testCase.fixture));
      return { accepted: Boolean(validators[testCase.executed_by](value)), structural: true };
    }
    switch (testCase.executed_by) {
      case "raw-json-key-scanner":
        scanRawJson(fs.readFileSync(path.join(fixtureRoot, testCase.fixture)));
        break;
      case "catalog-relationship-probe":
        checkCatalogRelationships(readJson(path.join(fixtureRoot, testCase.fixture)));
        break;
      case "family-qualified-identity-probe":
        checkFamilyQualifiedIdentity(readJson(path.join(fixtureRoot, testCase.fixture)));
        break;
      case "catalog-feature-probe":
        checkCatalogFeature(readJson(path.join(fixtureRoot, testCase.fixture)), testCase.feature);
        break;
      case "bundle-integrity-probe":
        checkBundle(path.join(fixtureRoot, testCase.fixture));
        break;
      case "image-decode-probe":
        if (testCase.assert_decoder_not_called) {
          const callsBefore = pngDecodeCalls;
          try {
            inspectImage(fs.readFileSync(path.join(fixtureRoot, testCase.fixture)), testCase.declared_format);
          } finally {
            assert.equal(
              pngDecodeCalls,
              callsBefore,
              `${testCase.id} entered the PNG decoder before rejecting the IHDR dimensions`,
            );
          }
        } else {
          inspectImage(fs.readFileSync(path.join(fixtureRoot, testCase.fixture)), testCase.declared_format);
        }
        break;
      case "overlay-image-probe": {
        const overlay = readJson(path.join(fixtureRoot, testCase.fixture));
        if (!validators["ajv-overlay"](overlay)) throw new Diagnostic("schema_structure");
        const relativePath = overlay[0].localized["1033"].image.path;
        assert.equal(relativePath, "encyclopedia/assets/test.png");
        inspectImage(fs.readFileSync(path.join(fixtureRoot, testCase.asset)), path.extname(relativePath).slice(1));
        break;
      }
      case "boundary-probe":
        boundaryResult(testCase.boundary);
        break;
      default:
        throw new Error(`unknown fixture executor ${testCase.executed_by}`);
    }
    return { accepted: true, code: "ok", structural: false };
  } catch (error) {
    if (error instanceof Diagnostic) return { accepted: false, code: error.code, structural: false };
    throw error;
  }
}

function listFiles(directory, prefix = "") {
  const result = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
    const relative = prefix ? `${prefix}/${entry.name}` : entry.name;
    if (entry.isDirectory()) result.push(...listFiles(path.join(directory, entry.name), relative));
    else if (entry.isFile()) result.push(relative);
    else throw new Error(`fixture tree contains unsupported entry ${relative}`);
  }
  return result;
}

function compareTrees(left, right) {
  const leftFiles = listFiles(left);
  const rightFiles = listFiles(right);
  assert.deepEqual(leftFiles, rightFiles, "fixture generator file inventory is not deterministic");
  for (const relative of leftFiles) {
    assert.ok(
      fs.readFileSync(path.join(left, relative)).equals(fs.readFileSync(path.join(right, relative))),
      `fixture generator changed bytes for ${relative}`,
    );
  }
  return leftFiles;
}

function referencedFixtureFiles() {
  const referenced = new Set(cases.artifacts.map((artifact) => artifact.path));
  const include = (relative) => {
    if (!relative) return;
    const absolute = path.join(fixtureRoot, relative);
    if (fs.statSync(absolute).isDirectory()) {
      for (const child of listFiles(absolute)) referenced.add(`${relative}/${child}`);
    } else {
      referenced.add(relative);
    }
  };
  for (const testCase of cases.cases) {
    include(testCase.fixture);
    include(testCase.asset);
  }
  for (const delegated of cases.delegated_inventory) include(delegated.fixture);
  return referenced;
}

function validateInventory() {
  assert.equal(cases.schema_version, 1);
  assert.equal(cases.wire_contract, "encyclopedia-v1-coordinator-approved-2026-09-29");
  assert.match(cases.authorship, /Contributor-authored synthetic/);
  assert.equal(cases.limits.retained_bytes, 536870912, "v1 uses one global retained-byte cap");
  const ids = cases.cases.map((testCase) => testCase.id);
  assert.equal(new Set(ids).size, ids.length, "case IDs must be unique and stable");
  const delegatedIds = cases.delegated_inventory.map((entry) => entry.id);
  assert.equal(new Set(delegatedIds).size, delegatedIds.length, "delegated case IDs must be unique");
  assert.equal(ids.includes("catalog-mixed-language"), false, "E44 presentation case leaked into file corpus");
  assert.equal(ids.includes("catalog-internal-key-as-label"), false, "E44 label case leaked into file corpus");
  for (const testCase of cases.cases) {
    assert.match(testCase.id, /^[a-z0-9][a-z0-9-]*$/);
    assert.ok(typeof testCase.consuming_validator === "string" && testCase.consuming_validator.length > 0);
    assert.equal(typeof testCase.expect?.accepted, "boolean");
    assert.ok(typeof testCase.expect?.code === "string" && testCase.expect.code.length > 0);
    if (testCase.fixture) {
      assert.equal(path.isAbsolute(testCase.fixture), false);
      assert.equal(testCase.fixture.split("/").includes(".."), false);
      assert.ok(fs.existsSync(path.join(fixtureRoot, testCase.fixture)), `${testCase.id} fixture is missing`);
    }
    if (testCase.asset) {
      assert.equal(path.isAbsolute(testCase.asset), false);
      assert.equal(testCase.asset.split("/").includes(".."), false);
      assert.ok(fs.existsSync(path.join(fixtureRoot, testCase.asset)), `${testCase.id} asset is missing`);
    }
    if (testCase.assert_decoder_not_called !== undefined) {
      assert.equal(testCase.assert_decoder_not_called, true, `${testCase.id} has an invalid decoder-entry assertion`);
      assert.equal(testCase.executed_by, "image-decode-probe");
      assert.equal(testCase.declared_format, "png");
    }
  }
  for (const id of [
    "relationship-frozen-admitted-subset-forbidden", "catalog-aliases-unknown-v1",
    "overlay-valid-faction-pair-shape", "integrity-base-png-forbidden-v1",
    "image-valid-mod-png", "body-ascii-exact-limit", "body-ascii-one-above-limit",
    "retained-bytes-exact-limit", "retained-bytes-one-above-limit",
    "catalog-duplicate-index-topic-id", "catalog-duplicate-category-topic-id",
    "raw-valid-nested-values", "raw-valid-repeated-keys-distinct-scopes",
    "raw-catalog-escaped-equivalent-duplicate-key", "image-png-oversized-ihdr-predecode-limit",
  ]) assert.ok(ids.includes(id), `required E09 case ${id} is missing`);
  const delegated = new Map(cases.delegated_inventory.map((entry) => [entry.id, entry]));
  assert.equal(delegated.get("overlay-viewer-faction-pair-capability")?.expected_code, "ok");
  assert.equal(
    delegated.get("overlay-static-topic-faction-pair-capability")?.expected_code,
    "image_override_capability",
  );
}

function verifyAuthoredArtifacts() {
  for (const artifact of cases.artifacts) {
    const bytes = fs.readFileSync(path.join(fixtureRoot, artifact.path));
    assert.equal(bytes.length, artifact.byte_length, `${artifact.path} length changed`);
    assert.equal(sha256(bytes), artifact.sha256, `${artifact.path} digest changed`);
    if (artifact.kind === "bmp" || artifact.kind === "png") {
      const inspected = inspectImage(bytes, artifact.kind);
      assert.equal(inspected.width, artifact.width);
      assert.equal(inspected.height, artifact.height);
    }
  }
  for (const relative of listFiles(fixtureRoot).filter((file) => file.endsWith(".json"))) {
    if (relative.includes("raw/")) continue;
    const text = fs.readFileSync(path.join(fixtureRoot, relative), "utf8");
    assert.equal(text.includes("/home/"), false, `${relative} contains an installation path`);
    assert.equal(text.includes("Star Wars - Rebellion"), false, `${relative} contains an installation name`);
    assert.equal(text.includes("EDATA.192"), false, `${relative} introduced deferred alternate art`);
  }
}

validateInventory();

const temporaryRoot = fs.mkdtempSync(path.join(os.tmpdir(), "rebellion-encyclopedia-fixtures-"));
try {
  const first = path.join(temporaryRoot, "first");
  const second = path.join(temporaryRoot, "second");
  const firstIdentity = generateFixtures(first);
  const secondIdentity = generateFixtures(second);
  assert.deepEqual(firstIdentity, secondIdentity, "generator-reported identities changed between runs");
  const generatedFiles = compareTrees(first, second);
  compareTrees(first, fixtureRoot);
  const referencedFiles = referencedFixtureFiles();
  assert.deepEqual(
    generatedFiles.filter((relative) => !referencedFiles.has(relative)),
    [],
    "generated fixture files must be named by cases.json or its artifact inventory",
  );
  assert.equal(firstIdentity.png.sha256, "5b8ce344a9d7fe4bdf8780725fc2fc36dce3f297688e62f18c8848fce6fecb8b");
  assert.equal(firstIdentity.catalog_sha256, "f1087ebf39faef3aef5b702e6ad957a910e7fa5924995d668be42b6eddbc39c9");
  verifyAuthoredArtifacts();

  const counts = { positive: 0, negative: 0, structural: 0, semantic_probe: 0 };
  for (const testCase of cases.cases) {
    const actual = executeProbe(testCase);
    assert.equal(actual.accepted, testCase.expect.accepted, `${testCase.id} acceptance disagrees with cases.json`);
    if (!actual.structural) {
      assert.equal(actual.code, testCase.expect.code, `${testCase.id} diagnostic disagrees with cases.json`);
      counts.semantic_probe += 1;
    } else {
      counts.structural += 1;
    }
    counts[testCase.expect.accepted ? "positive" : "negative"] += 1;
  }

  process.stdout.write(`${JSON.stringify({
    status: "pass",
    wire_contract: cases.wire_contract,
    cases: cases.cases.length,
    positive: counts.positive,
    negative: counts.negative,
    structural: counts.structural,
    semantic_probes: counts.semantic_probe,
    delegated: cases.delegated_inventory.length,
    generated_files: generatedFiles.length,
    generator_runs_equal: true,
    committed_tree_equal: true,
    catalog_sha256: firstIdentity.catalog_sha256,
    bmp_sha256: firstIdentity.bmp.sha256,
    png_sha256: firstIdentity.png.sha256,
  }, null, 2)}\n`);
} finally {
  fs.rmSync(temporaryRoot, { recursive: true, force: true });
}
