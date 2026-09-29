#!/usr/bin/env node

import crypto from "node:crypto";
import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import zlib from "node:zlib";
import { fileURLToPath, pathToFileURL } from "node:url";

const sha256 = (bytes) => crypto.createHash("sha256").update(bytes).digest("hex");
const jsonBytes = (value) => Buffer.from(`${JSON.stringify(value, null, 2)}\n`);
const clone = (value) => structuredClone(value);

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

function pngChunk(kind, data) {
  const kindBytes = Buffer.from(kind, "ascii");
  const chunk = Buffer.alloc(12 + data.length);
  chunk.writeUInt32BE(data.length, 0);
  kindBytes.copy(chunk, 4);
  data.copy(chunk, 8);
  chunk.writeUInt32BE(crc32(Buffer.concat([kindBytes, data])), 8 + data.length);
  return chunk;
}

function makePngRgba(width, height, pixels) {
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(width, 0);
  ihdr.writeUInt32BE(height, 4);
  ihdr[8] = 8;
  ihdr[9] = 6;
  const rows = [];
  for (let y = 0; y < height; y += 1) {
    rows.push(Buffer.from([0]));
    rows.push(Buffer.from(pixels.slice(y * width * 4, (y + 1) * width * 4)));
  }
  return Buffer.concat([
    Buffer.from("89504e470d0a1a0a", "hex"),
    pngChunk("IHDR", ihdr),
    pngChunk("IDAT", zlib.deflateSync(Buffer.concat(rows), {
      level: 9,
      strategy: zlib.constants.Z_RLE,
    })),
    pngChunk("IEND", Buffer.alloc(0)),
  ]);
}

function makePngWithDeclaredDimensions(width, height) {
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(width, 0);
  ihdr.writeUInt32BE(height, 4);
  ihdr[8] = 8;
  ihdr[9] = 6;
  return Buffer.concat([
    Buffer.from("89504e470d0a1a0a", "hex"),
    pngChunk("IHDR", ihdr),
    pngChunk("IDAT", zlib.deflateSync(Buffer.alloc(0))),
    pngChunk("IEND", Buffer.alloc(0)),
  ]);
}

function makeBmp24(width, height, pixels) {
  const rowBytes = width * 3;
  const stride = (rowBytes + 3) & ~3;
  const pixelBytes = stride * height;
  const result = Buffer.alloc(54 + pixelBytes);
  result.write("BM", 0, "ascii");
  result.writeUInt32LE(result.length, 2);
  result.writeUInt32LE(54, 10);
  result.writeUInt32LE(40, 14);
  result.writeInt32LE(width, 18);
  result.writeInt32LE(height, 22);
  result.writeUInt16LE(1, 26);
  result.writeUInt16LE(24, 28);
  result.writeUInt32LE(0, 30);
  result.writeUInt32LE(pixelBytes, 34);
  for (let outputY = 0; outputY < height; outputY += 1) {
    const sourceY = height - outputY - 1;
    for (let x = 0; x < width; x += 1) {
      const source = (sourceY * width + x) * 3;
      const destination = 54 + outputY * stride + x * 3;
      result[destination] = pixels[source + 2];
      result[destination + 1] = pixels[source + 1];
      result[destination + 2] = pixels[source];
    }
  }
  return result;
}

function makeSourceRecord(reference, ordinal) {
  const raw = Buffer.from(`E37 synthetic source record ${ordinal}`);
  return {
    source_basename: ordinal < 7 ? "SYNTHETIC-TEXT.DLL" : "SYNTHETIC-UI.DLL",
    source_sha256: sha256(Buffer.from("E37 contributor-authored source identity")),
    resource_type: { kind: "numeric", value: ordinal < 7 ? 10 : 6 },
    resource_id: { kind: "numeric", value: 60000 + ordinal },
    language_id: 1033,
    raw_length: raw.length,
    raw_sha256: sha256(raw),
    decoder: "e37-synthetic-utf8-v1",
    encoding: "utf-8",
    mapping_citations: [`synthetic:E37:${reference}`],
  };
}

const sortRule = {
  algorithm: "stable_display_title_v1",
  representable_encoding: "windows-1252-strict",
  representable_fold: "ascii-lowercase-only",
  unrepresentable: "unicode-15.1.0-scalar-lowercase-utf8-after-representable",
  tie_break: "registry-order",
};

const categorySpecs = [
  ["command:0x70", "0x70", "Synthetic systems", "original:60001"],
  ["command:0x71", "0x71", "Synthetic craft", "original:60002"],
  ["command:0x72", "0x72", "Synthetic facilities", "original:60003"],
  ["command:0x73", "0x73", "Synthetic missions", "original:60004"],
  ["command:0x74", "0x74", "Synthetic troops", "original:60005"],
  ["command:0x75", "0x75", "Synthetic personnel", "original:60006"],
];

function buildCatalog(images) {
  const categories = categorySpecs.map(([id, command, label, topicId], index) => ({
    id,
    command,
    labels: index === 0 ? { "1033": label, "1036": "Systèmes synthétiques" } : { "1033": label },
    topic_ids: [topicId],
    source_ref: `fixture/category/${command}`,
  }));
  return {
    schema_version: 1,
    default_language: "1033",
    topic_sort: sortRule,
    index: {
      command: "0x6f",
      labels: { "1033": "Synthetic aggregate" },
      topic_ids: [
        "original:60001",
        "original:60002",
        "original:60003",
        "original:60004",
        "original:60005",
        "original:60006",
        "original:60007",
      ],
      source_ref: "fixture/category/0x6f",
    },
    categories,
    topics: {
      "original:60001": {
        localized: {
          "1033": { title: "Amber system", body: "Contributor-written system description.", image_id: "edata:1" },
          "1036": { title: "Système ambre", body: "Description synthétique rédigée pour ce test.", image_id: "edata:1" },
        },
        source_ref: "fixture/topic/60001",
      },
      "original:60002": {
        localized: { "1033": { title: "Blue vessel", body: "Synthetic vessel text.", image_id: "edata:2" } },
        source_ref: "fixture/topic/60002",
      },
      "original:60003": {
        localized: { "1033": { title: "Copper workshop", body: "Synthetic facility text.", image_id: "edata:2" } },
        source_ref: "fixture/topic/60003",
      },
      "original:60004": {
        localized: {
          "1033": {
            title: "Dual beacon",
            body: "Synthetic faction-sensitive art text.",
            image_selector: {
              kind: "viewer_faction",
              alliance_image_id: "edata:2",
              empire_image_id: "edata:3",
            },
          },
        },
        source_ref: "fixture/topic/60004",
      },
      "original:60005": {
        localized: { "1033": { title: "Empty canvas", body: "This synthetic topic intentionally has no image field." } },
        source_ref: "fixture/topic/60005",
      },
      "original:60006": {
        localized: { "1033": { title: "Null canvas", body: "This synthetic topic explicitly has no art.", image_id: null } },
        source_ref: "fixture/topic/60006",
      },
      "original:60007": {
        localized: { "1033": { title: "Registry-only flotilla", body: "Synthetic aggregate-only topic." } },
        source_ref: "fixture/topic/60007",
      },
    },
    images: {
      "edata:1": {
        path: "assets/EDATA.001",
        format: "bmp",
        byte_length: images.one.length,
        width: 2,
        height: 2,
        sha256: sha256(images.one),
        source_ref: "fixture/image/1",
      },
      "edata:2": {
        path: "assets/EDATA.002",
        format: "bmp",
        byte_length: images.two.length,
        width: 2,
        height: 2,
        sha256: sha256(images.two),
        source_ref: "fixture/image/2",
      },
      "edata:3": {
        path: "assets/EDATA.003",
        format: "bmp",
        byte_length: images.three.length,
        width: 2,
        height: 2,
        sha256: sha256(images.three),
        source_ref: "fixture/image/3",
      },
    },
    bindings: [
      { family: "system_locations", dat_id: 7, variant: "default", topic_id: "original:60001" },
      { family: "capital_ship_classes", dat_id: 7, variant: "default", topic_id: "original:60002" },
      { family: "facilities", dat_id: 13, variant: "default", topic_id: "original:60003" },
      { family: "missions", dat_id: 21, variant: "viewer_faction", topic_id: "original:60004" },
      { family: "troop_classes", dat_id: 5, variant: "default", topic_id: "original:60005" },
      { family: "special_forces", dat_id: 8, variant: "default", topic_id: "original:60006" },
      { family: "fixture_fleet", dat_id: 2, variant: "default", topic_id: "original:60007" },
    ],
  };
}

function collectSourceRefs(catalog) {
  return [
    catalog.index.source_ref,
    ...catalog.categories.map((category) => category.source_ref),
    ...Object.values(catalog.topics).map((topic) => topic.source_ref),
    ...Object.values(catalog.images).map((image) => image.source_ref),
  ];
}

function buildManifest(catalogBytes, catalog, images, sourceDat) {
  const sourceRecords = Object.fromEntries(
    collectSourceRefs(catalog).map((reference, index) => [reference, makeSourceRecord(reference, index + 1)]),
  );
  return {
    schema_version: 1,
    source_profile: "e37-synthetic-v1",
    extractor_version: "fixture-generator-v1",
    catalog_sha256: sha256(catalogBytes),
    files: {
      "catalog.json": sha256(catalogBytes),
      "assets/EDATA.001": sha256(images.one),
      "assets/EDATA.002": sha256(images.two),
      "assets/EDATA.003": sha256(images.three),
    },
    binding_sources: [{ basename: "SYNTHETIC.DAT", sha256: sha256(sourceDat) }],
    source_records: sourceRecords,
  };
}

function writeFile(root, relative, bytes) {
  const destination = path.join(root, relative);
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.writeFileSync(destination, bytes);
}

function writeJson(root, relative, value) {
  writeFile(root, relative, jsonBytes(value));
}

function writeBundle(root, name, catalog, images, sourceDat, mutateManifest = (manifest) => manifest) {
  const bundleRoot = path.join(root, "bundles", name);
  const catalogBytes = jsonBytes(catalog);
  const manifest = mutateManifest(buildManifest(catalogBytes, catalog, images, sourceDat));
  writeFile(bundleRoot, "catalog.json", catalogBytes);
  writeJson(bundleRoot, "manifest.json", manifest);
  writeFile(bundleRoot, "assets/EDATA.001", images.one);
  writeFile(bundleRoot, "assets/EDATA.002", images.two);
  writeFile(bundleRoot, "assets/EDATA.003", images.three);
  writeFile(bundleRoot, "sources/SYNTHETIC.DAT", sourceDat);
}

export function generateFixtures(outputRoot) {
  fs.mkdirSync(outputRoot, { recursive: true });
  const images = {
    one: makeBmp24(2, 2, [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xad, 0xbe, 0xef, 0x44, 0x22]),
    two: makeBmp24(2, 2, [0x90, 0x20, 0x10, 0x10, 0x90, 0x20, 0x20, 0x10, 0x90, 0xaa, 0xbb, 0xcc]),
    three: makeBmp24(2, 2, [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x11, 0x22, 0x33]),
  };
  const modPng = makePngRgba(1, 1, [0x12, 0x34, 0x56, 0xff]);
  const sourceDat = Buffer.from("E37 contributor-authored synthetic DAT bytes\n");
  const catalog = buildCatalog(images);
  writeBundle(outputRoot, "valid", catalog, images, sourceDat);

  const catalogMutations = {
    "unknown-aliases": (value) => { value.aliases = {}; },
    "missing-topic-sort": (value) => { delete value.topic_sort; },
    "wrong-topic-sort": (value) => { value.topic_sort.tie_break = "title"; },
    "mixed-static-selector": (value) => { value.topics["original:60001"].localized["1033"].image_selector = clone(value.topics["original:60004"].localized["1033"].image_selector); },
    "selector-missing-side": (value) => { delete value.topics["original:60004"].localized["1033"].image_selector.empire_image_id; },
    "selector-unknown-kind": (value) => { value.topics["original:60004"].localized["1033"].image_selector.kind = "campaign_state"; },
    "localized-unknown-field": (value) => { value.topics["original:60001"].localized["1033"].markup = true; },
    "bad-langid": (value) => { value.topics["original:60001"].localized.en = value.topics["original:60001"].localized["1033"]; },
    "unknown-version": (value) => { value.schema_version = 2; },
    "image-byte-above": (value) => { value.images["edata:1"].byte_length = 33554433; },
    "bad-base-image-id": (value) => { value.images["mod:demo:test"] = value.images["edata:1"]; delete value.images["edata:1"]; },
    "unsupported-image-format": (value) => { value.images["edata:1"].format = "gif"; },
    "unsafe-asset-path": (value) => { value.images["edata:1"].path = "assets/../escape.bmp"; },
  };
  for (const [name, mutate] of Object.entries(catalogMutations)) {
    const value = clone(catalog);
    mutate(value);
    writeJson(outputRoot, `schema/catalog-${name}.json`, value);
  }

  const relationshipMutations = {
    "frozen-admitted-subset": (value) => { value.index.topic_ids = value.index.topic_ids.filter((id) => id !== "original:60001"); },
    "invented-filtered-membership": (value) => { value.categories[0].topic_ids.push("original:60007"); },
    "dangling-index-topic": (value) => { value.index.topic_ids.push("original:69999"); },
    "duplicate-index-topic": (value) => { value.index.topic_ids.push(value.index.topic_ids[0]); },
    "duplicate-category-topic": (value) => { value.categories[0].topic_ids.push(value.categories[0].topic_ids[0]); },
    "duplicate-category-command": (value) => { value.categories[1].command = "0x70"; },
    "wrong-category-order": (value) => { [value.categories[0], value.categories[1]] = [value.categories[1], value.categories[0]]; },
    "duplicate-binding-tuple": (value) => { value.bindings.push({ ...value.bindings[0], topic_id: "original:60002" }); },
    "dangling-binding-topic": (value) => { value.bindings[0].topic_id = "original:69999"; },
    "dangling-image-reference": (value) => { value.topics["original:60001"].localized["1033"].image_id = "edata:999"; },
    "default-binding-faction-selector": (value) => { value.bindings.find((binding) => binding.topic_id === "original:60004").variant = "default"; },
    "faction-binding-static-selector": (value) => { value.bindings.find((binding) => binding.topic_id === "original:60001").variant = "viewer_faction"; },
  };
  for (const [name, mutate] of Object.entries(relationshipMutations)) {
    const value = clone(catalog);
    mutate(value);
    writeJson(outputRoot, `relationships/catalog-${name}.json`, value);
  }

  const validManifest = buildManifest(jsonBytes(catalog), catalog, images, sourceDat);
  const mandatoryManifestFields = [
    "schema_version", "source_profile", "extractor_version", "catalog_sha256",
    "files", "binding_sources", "source_records",
  ];
  for (const field of mandatoryManifestFields) {
    const value = clone(validManifest);
    delete value[field];
    writeJson(outputRoot, `schema/manifest-missing-${field.replaceAll("_", "-")}.json`, value);
  }
  const manifestSelfHash = clone(validManifest);
  manifestSelfHash.files["manifest.json"] = "0".repeat(64);
  writeJson(outputRoot, "schema/manifest-self-hash.json", manifestSelfHash);
  const manifestUnknown = clone(validManifest);
  manifestUnknown.generated_at = "not-deterministic";
  writeJson(outputRoot, "schema/manifest-unknown-field.json", manifestUnknown);
  const manifestUnsafe = clone(validManifest);
  manifestUnsafe.files["assets/../escape.bmp"] = "0".repeat(64);
  writeJson(outputRoot, "schema/manifest-unsafe-path.json", manifestUnsafe);

  const overlays = {
    "valid-empty": [],
    "valid-omitted-image": [{ id: "original:60001", localized: { "1033": { body: "Synthetic body replacement." } } }],
    "valid-null-image": [{ id: "original:60001", localized: { "1033": { image: null } } }],
    "valid-empty-body": [{ id: "original:60001", localized: { "1033": { body: "" } } }],
    "valid-language-delete": [{ id: "original:60001", localized: { "1036": null } }],
    "valid-static-bmp": [{ id: "original:60001", localized: { "1033": { image: { path: "encyclopedia/assets/test.bmp" } } } }],
    "valid-static-png": [{ id: "original:60001", localized: { "1033": { image: { path: "encyclopedia/assets/test.png" } } } }],
    "valid-faction-pair": [{
      id: "original:60004",
      localized: {
        "1041": {
          title: "Synthetic translated title",
          body: "Synthetic translated body.",
          image: {
            alliance: { path: "encyclopedia/assets/alliance.png" },
            empire: { path: "encyclopedia/assets/empire.png" },
          },
        },
      },
    }],
    "semantic-static-topic-faction-pair": [{
      id: "original:60001",
      localized: {
        "1033": {
          image: {
            alliance: { path: "encyclopedia/assets/alliance.png" },
            empire: { path: "encyclopedia/assets/empire.png" },
          },
        },
      },
    }],
    "invalid-faction-pair-missing-side": [{ id: "original:60004", localized: { "1033": { image: { alliance: { path: "encyclopedia/assets/alliance.png" } } } } }],
    "invalid-unsafe-path": [{ id: "original:60001", localized: { "1033": { image: { path: "encyclopedia/assets/../escape.png" } } } }],
    "invalid-generated-id": [{ id: "original:60001", localized: { "1033": { image: { image_id: "mod:v1:00:encyclopedia/assets/test.png" } } } }],
    "invalid-binding-edit": [{ id: "original:60001", localized: { "1033": { body: "Synthetic." } }, bindings: [] }],
    "invalid-membership-edit": [{ id: "original:60001", localized: { "1033": { body: "Synthetic." } }, topic_ids: [] }],
    "invalid-provenance-edit": [{ id: "original:60001", localized: { "1033": { body: "Synthetic." } }, source_ref: "fixture/forbidden" }],
    "invalid-aliases": [{ id: "original:60001", localized: { "1033": { body: "Synthetic." } }, aliases: ["original:60002"] }],
    "semantic-unknown-topic": [{ id: "original:69999", localized: { "1033": { body: "Synthetic." } } }],
  };
  for (const [name, value] of Object.entries(overlays)) {
    writeJson(outputRoot, `overlays/${name}.json`, value);
  }

  const rawFixtures = {
    "valid-nested-values.json": "{\"outer\":{\"items\":[{\"value\":1},{\"value\":2}]},\"matrix\":[[true,false],[null,\"x\"]]}",
    "valid-repeated-keys-distinct-scopes.json": "{\"left\":{\"name\":\"a\"},\"right\":{\"name\":\"b\"},\"items\":[{\"name\":\"c\"},{\"name\":\"d\"}]}",
    "catalog-duplicate-root-key.json": "{\"schema_version\":1,\"schema_version\":1}",
    "catalog-escaped-equivalent-duplicate-key.json": "{\"title\":1,\"\\u0074itle\":2}",
    "catalog-duplicate-topic-key.json": "{\"topics\":{\"original:60001\":{},\"original:60001\":{}}}",
    "catalog-duplicate-langid-key.json": "{\"localized\":{\"1033\":{},\"1033\":{}}}",
    "catalog-duplicate-localized-field.json": "{\"localized\":{\"1033\":{\"body\":\"one\",\"body\":\"two\"}}}",
    "manifest-duplicate-file-key.json": "{\"files\":{\"catalog.json\":\"a\",\"catalog.json\":\"b\"}}",
    "manifest-duplicate-source-record-key.json": "{\"source_records\":{\"fixture/topic/1\":{},\"fixture/topic/1\":{}}}",
    "overlay-duplicate-selector-key.json": "[{\"id\":\"original:60001\",\"id\":\"original:60002\",\"localized\":{}}]",
    "overlay-duplicate-langid-key.json": "[{\"id\":\"original:60001\",\"localized\":{\"1033\":{},\"1033\":{}}}]",
    "overlay-duplicate-patch-field.json": "[{\"id\":\"original:60001\",\"localized\":{\"1033\":{\"body\":\"one\",\"body\":\"two\"}}}]",
  };
  for (const [name, value] of Object.entries(rawFixtures)) {
    writeFile(outputRoot, `raw/${name}`, Buffer.from(value));
  }
  writeFile(outputRoot, "raw/catalog-invalid-utf8.json", Buffer.from([0x7b, 0x22, 0x78, 0x22, 0x3a, 0x22, 0xc3, 0x28, 0x22, 0x7d]));

  writeFile(outputRoot, "images/base-valid.bmp", images.one);
  writeFile(outputRoot, "images/mod-valid.png", modPng);
  writeFile(outputRoot, "images/mod-plain-text.png", Buffer.from("synthetic mod image bytes"));
  writeFile(outputRoot, "images/mod-png-with-bmp-extension.bmp", modPng);
  writeFile(outputRoot, "images/corrupt-bmp.bmp", images.one.subarray(0, images.one.length - 3));
  writeFile(outputRoot, "images/corrupt-png.png", modPng.subarray(0, modPng.length - 4));
  const corruptPngCrc = Buffer.from(modPng);
  corruptPngCrc[corruptPngCrc.length - 1] ^= 1;
  writeFile(outputRoot, "images/corrupt-png-crc.png", corruptPngCrc);
  writeFile(outputRoot, "images/oversized-ihdr.png", makePngWithDeclaredDimensions(4001, 4000));
  writeFile(outputRoot, "images/zero-width-ihdr.png", makePngWithDeclaredDimensions(0, 1));

  const pngImages = { ...images, one: modPng };
  const pngCatalog = clone(catalog);
  pngCatalog.images["edata:1"] = {
    ...pngCatalog.images["edata:1"],
    path: "assets/EDATA.001",
    format: "png",
    byte_length: modPng.length,
    width: 1,
    height: 1,
    sha256: sha256(modPng),
  };
  writeBundle(outputRoot, "invalid-base-png", pngCatalog, pngImages, sourceDat);

  const mislabeledCatalog = clone(pngCatalog);
  mislabeledCatalog.images["edata:1"].format = "bmp";
  writeBundle(outputRoot, "invalid-base-format-mismatch", mislabeledCatalog, pngImages, sourceDat);

  writeBundle(outputRoot, "invalid-catalog-hash", catalog, images, sourceDat, (manifest) => {
    manifest.catalog_sha256 = "0".repeat(64);
    return manifest;
  });
  writeBundle(outputRoot, "invalid-files-catalog-hash", catalog, images, sourceDat, (manifest) => {
    manifest.files["catalog.json"] = "0".repeat(64);
    return manifest;
  });
  writeBundle(outputRoot, "invalid-missing-image-file-entry", catalog, images, sourceDat, (manifest) => {
    delete manifest.files["assets/EDATA.003"];
    return manifest;
  });
  writeBundle(outputRoot, "invalid-extra-file-entry", catalog, images, sourceDat, (manifest) => {
    manifest.files["assets/EXTRA.BMP"] = "0".repeat(64);
    return manifest;
  });
  writeBundle(outputRoot, "invalid-image-file-hash", catalog, images, sourceDat, (manifest) => {
    manifest.files["assets/EDATA.001"] = "0".repeat(64);
    return manifest;
  });
  writeBundle(outputRoot, "invalid-binding-source-hash", catalog, images, sourceDat, (manifest) => {
    manifest.binding_sources[0].sha256 = "0".repeat(64);
    return manifest;
  });

  const descriptorMismatch = clone(catalog);
  descriptorMismatch.images["edata:1"].sha256 = "0".repeat(64);
  writeBundle(outputRoot, "invalid-image-descriptor-hash", descriptorMismatch, images, sourceDat);

  const danglingSource = clone(catalog);
  danglingSource.topics["original:60001"].source_ref = "fixture/topic/missing";
  writeBundle(outputRoot, "invalid-dangling-source-ref", danglingSource, images, sourceDat, (manifest) => {
    delete manifest.source_records["fixture/topic/missing"];
    return manifest;
  });

  return {
    png: { byte_length: modPng.length, sha256: sha256(modPng), width: 1, height: 1 },
    bmp: { byte_length: images.one.length, sha256: sha256(images.one), width: 2, height: 2 },
    catalog_sha256: sha256(jsonBytes(catalog)),
    source_dat_sha256: sha256(sourceDat),
  };
}

const modulePath = fileURLToPath(import.meta.url);
if (process.argv[1] && pathToFileURL(path.resolve(process.argv[1])).href === import.meta.url) {
  const output = process.argv[2];
  if (!output) {
    throw new Error("usage: node generate.mjs <output-directory>");
  }
  const result = generateFixtures(path.resolve(output));
  process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
}
