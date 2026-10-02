#!/usr/bin/env node

import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import fs from "node:fs";
import http from "node:http";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const defaultScenario = path.join(here, "scenarios/encyclopedia-base.json");
const defaultSite = path.join(root, ".artifacts/e21/packed-site");
const defaultSource = path.join(
  root,
  "../agent-30-encyclopedia-build-publication-session-14-open-rebellion/data/base",
);
const expectedRequests = ["/", "/data/runtime.orpk", "/gl.js", "/open-rebellion-test.wasm"];
const artOrigin = { x: 97, y: 86 };

function parseArguments(argv) {
  const parsed = { verifyOnly: false };
  for (let index = 0; index < argv.length; index += 1) {
    const value = argv[index];
    if (value === "--verify-artifacts-only") {
      parsed.verifyOnly = true;
      continue;
    }
    if (!["--scenario", "--source-root", "--site", "--output"].includes(value)) {
      throw new Error(`unknown argument ${value}`);
    }
    const next = argv[index + 1];
    if (!next) throw new Error(`${value} requires a path`);
    parsed[value.slice(2).replace(/-([a-z])/g, (_, letter) => letter.toUpperCase())] = path.resolve(next);
    index += 1;
  }
  return {
    verifyOnly: parsed.verifyOnly,
    scenario: parsed.scenario || defaultScenario,
    sourceRoot: parsed.sourceRoot || defaultSource,
    site: parsed.site || defaultSite,
    output: parsed.output || path.join(root, ".artifacts/e21/owned-parity/verification.json"),
  };
}

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

function readJson(file) {
  return JSON.parse(fs.readFileSync(file, "utf8"));
}

function readRuntimePack(file) {
  const contents = fs.readFileSync(file);
  assert.equal(contents.subarray(0, 4).toString("ascii"), "ORPK", "runtime pack magic");
  assert.equal(contents.readUInt16LE(4), 3, "runtime pack version");
  assert.equal(contents.readUInt16LE(6), 0, "runtime pack flags");
  const count = contents.readUInt32LE(8);
  let cursor = 12;
  const entries = new Map();
  for (let index = 0; index < count; index += 1) {
    assert.ok(cursor + 7 <= contents.length, `runtime pack entry ${index} header is truncated`);
    const kind = contents.readUInt8(cursor);
    const keyLength = contents.readUInt16LE(cursor + 1);
    const byteLength = contents.readUInt32LE(cursor + 3);
    cursor += 7;
    assert.ok(cursor + keyLength + byteLength <= contents.length, `runtime pack entry ${index} is truncated`);
    const key = contents.subarray(cursor, cursor + keyLength).toString("utf8");
    cursor += keyLength;
    const bytes = contents.subarray(cursor, cursor + byteLength);
    cursor += byteLength;
    assert.ok(!entries.has(key), `runtime pack contains duplicate key ${key}`);
    entries.set(key, { kind, bytes });
  }
  assert.equal(cursor, contents.length, "runtime pack has trailing bytes");
  return { contents, entries };
}

const windows1252 = new Map([
  [0x20ac, 0x80], [0x201a, 0x82], [0x0192, 0x83], [0x201e, 0x84], [0x2026, 0x85],
  [0x2020, 0x86], [0x2021, 0x87], [0x02c6, 0x88], [0x2030, 0x89], [0x0160, 0x8a],
  [0x2039, 0x8b], [0x0152, 0x8c], [0x017d, 0x8e], [0x2018, 0x91], [0x2019, 0x92],
  [0x201c, 0x93], [0x201d, 0x94], [0x2022, 0x95], [0x2013, 0x96], [0x2014, 0x97],
  [0x02dc, 0x98], [0x2122, 0x99], [0x0161, 0x9a], [0x203a, 0x9b], [0x0153, 0x9c],
  [0x017e, 0x9e], [0x0178, 0x9f],
]);

function windows1252Bytes(value) {
  const bytes = [];
  for (const scalar of value) {
    const code = scalar.codePointAt(0);
    if (code <= 0x7f || (code >= 0xa0 && code <= 0xff)) bytes.push(code);
    else if (windows1252.has(code)) bytes.push(windows1252.get(code));
    else return null;
  }
  return Buffer.from(bytes.map((byte) => (byte >= 0x41 && byte <= 0x5a ? byte + 0x20 : byte)));
}

function titleSortKey(title) {
  const encoded = windows1252Bytes(title);
  if (encoded) return { class: 0, bytes: encoded };
  const loweredScalars = Array.from(title, (scalar) => scalar.toLowerCase()).join("");
  return { class: 1, bytes: Buffer.from(loweredScalars, "utf8") };
}

function compareSortRows(left, right) {
  if (left.key.class !== right.key.class) return left.key.class - right.key.class;
  const byteOrder = Buffer.compare(left.key.bytes, right.key.bytes);
  return byteOrder || left.registryIndex - right.registryIndex;
}

function selectedImageId(localized, faction) {
  if (Object.hasOwn(localized, "image_id")) return localized.image_id;
  if (!localized.image_selector) return null;
  return faction === "alliance"
    ? localized.image_selector.alliance_image_id
    : localized.image_selector.empire_image_id;
}

function validateVisibleTextChecks(scenario, catalog) {
  const viewport = scenario.body_viewport;
  assert.ok(viewport, "body_viewport is required");
  for (const field of ["x", "y", "width", "height"]) {
    assert.ok(Number.isInteger(viewport[field]) && viewport[field] > 0, `body_viewport.${field}`);
  }
  assert.ok(viewport.x + viewport.width <= 640, "body viewport exceeds fixture width");
  assert.ok(viewport.y + viewport.height <= 480, "body viewport exceeds fixture height");

  const checks = scenario.visible_text_checks;
  assert.ok(Array.isArray(checks) && checks.length > 0, "visible_text_checks are required");
  for (const check of checks) {
    const probe = scenario.probes.find((candidate) => candidate.topic_id === check.topic_id);
    assert.ok(probe, `visible text topic ${check.topic_id} is not a retained probe`);
    const body = catalog.topics[check.topic_id]?.localized?.[catalog.default_language]?.body;
    assert.ok(body, `visible text topic ${check.topic_id} lacks a default-language body`);
    assert.equal(sha256(Buffer.from(body)), probe.body_sha256, `${check.topic_id} visible body digest`);
    assert.ok(Array.isArray(check.input_sequence), `${check.topic_id} input_sequence`);

    if (check.purpose === "long_body_scroll") {
      assert.ok(check.input_sequence.length > 0, "long body journey must scroll");
      assert.ok(
        check.input_sequence.every((key) => key === "PageDown"),
        "long body journey uses ordinary PageDown input",
      );
      assert.equal(
        check.expected_distinct_viewports,
        check.input_sequence.length + 1,
        "long body captures initial plus every scroll step",
      );
      continue;
    }

    assert.equal(check.purpose, "owned_non_ascii", `unknown visible text purpose ${check.purpose}`);
    assert.deepEqual(check.input_sequence, [], "non-ASCII initial viewport needs no synthetic input");
    assert.equal(check.expected_capture, "initial_body_viewport");
    const scalars = Array.from(body);
    const location = check.codepoint;
    const scalar = scalars[location.char_index];
    assert.ok(scalar, `${check.topic_id} codepoint index is out of range`);
    assert.equal(`U+${scalar.codePointAt(0).toString(16).toUpperCase().padStart(4, "0")}`, location.scalar);
    assert.equal(Buffer.byteLength(scalars.slice(0, location.char_index).join("")), location.utf8_byte_offset);
    assert.equal(Buffer.from(scalar).toString("hex"), location.utf8_hex);
    assert.equal(
      sha256(Buffer.from(scalars.slice(location.context_char_start, location.context_char_end_exclusive).join(""))),
      location.context_sha256,
      `${check.topic_id} codepoint context digest`,
    );
  }
  return checks;
}

export function verifyArtifacts(options) {
  const scenario = readJson(options.scenario);
  assert.equal(scenario.schema_version, 1);
  assert.equal(scenario.family, "encyclopedia-base-parity");
  const packPath = path.join(options.site, "data/runtime.orpk");
  const pack = readRuntimePack(packPath);
  assert.equal(sha256(pack.contents), scenario.artifacts.pack_sha256, "runtime pack SHA-256");
  assert.equal(pack.contents.length, scenario.artifacts.pack_byte_length, "runtime pack byte length");

  const namespace = new Map(
    [...pack.entries]
      .filter(([key]) => key.startsWith("encyclopedia/"))
      .map(([key, entry]) => [key.slice("encyclopedia/".length), entry]),
  );
  assert.equal(namespace.size, scenario.artifacts.encyclopedia_namespace_entries);
  for (const [relative, entry] of namespace) {
    assert.equal(entry.kind, 0, `encyclopedia/${relative} must be a game-data entry`);
    const loose = fs.readFileSync(path.join(options.sourceRoot, "encyclopedia", relative));
    assert.deepEqual(entry.bytes, loose, `packed encyclopedia/${relative} differs from retained loose bytes`);
  }

  const catalogBytes = namespace.get("catalog.json")?.bytes;
  const manifestBytes = namespace.get("manifest.json")?.bytes;
  assert.ok(catalogBytes, "packed catalog.json is missing");
  assert.ok(manifestBytes, "packed manifest.json is missing");
  assert.equal(sha256(catalogBytes), scenario.artifacts.catalog_sha256);
  assert.equal(sha256(manifestBytes), scenario.artifacts.manifest_sha256);
  const catalog = JSON.parse(catalogBytes.toString("utf8"));
  const manifest = JSON.parse(manifestBytes.toString("utf8"));
  const visibleTextChecks = validateVisibleTextChecks(scenario, catalog);
  assert.equal(manifest.source_profile, scenario.source_profile);
  assert.equal(manifest.catalog_sha256, scenario.artifacts.catalog_sha256);
  assert.equal(manifest.files["catalog.json"], scenario.artifacts.catalog_sha256);
  assert.equal(namespace.size, Object.keys(manifest.files).length + 1, "manifest inventory plus manifest");
  for (const [relative, digest] of Object.entries(manifest.files)) {
    const entry = namespace.get(relative);
    assert.ok(entry, `manifest file ${relative} is absent from the pack`);
    assert.equal(sha256(entry.bytes), digest, `manifest file digest ${relative}`);
  }

  for (const source of manifest.binding_sources) {
    const dat = fs.readFileSync(path.join(options.sourceRoot, source.basename));
    assert.equal(sha256(dat), source.sha256, `binding DAT ${source.basename}`);
  }
  for (const [assetId, image] of Object.entries(catalog.images)) {
    const entry = namespace.get(image.path);
    assert.ok(entry, `${assetId} path ${image.path} is absent from the pack`);
    assert.equal(entry.bytes.length, image.byte_length, `${assetId} byte length`);
    assert.equal(sha256(entry.bytes), image.sha256, `${assetId} digest`);
  }

  const registry = catalog.index.topic_ids;
  const sortedTopics = registry.map((topicId, registryIndex) => {
    const localized = catalog.topics[topicId]?.localized?.[catalog.default_language];
    assert.ok(localized, `${topicId} is missing the whole default-language record`);
    return { topicId, registryIndex, key: titleSortKey(localized.title) };
  }).sort(compareSortRows);

  for (const probe of scenario.probes) {
    const topic = catalog.topics[probe.topic_id];
    assert.ok(topic, `probe topic ${probe.topic_id} is absent`);
    const localized = topic.localized[catalog.default_language];
    assert.ok(localized, `probe topic ${probe.topic_id} lacks default language`);
    assert.equal(sha256(Buffer.from(localized.title)), probe.title_sha256, `${probe.topic_id} title`);
    assert.equal(sha256(Buffer.from(localized.body)), probe.body_sha256, `${probe.topic_id} body`);
    const bindings = catalog.bindings.filter((binding) => binding.topic_id === probe.topic_id);
    assert.equal(bindings.length, 1, `${probe.topic_id} must have one binding`);
    assert.deepEqual(
      {
        family: bindings[0].family,
        dat_id: bindings[0].dat_id,
        variant: bindings[0].variant,
      },
      probe.binding,
      `${probe.topic_id} binding`,
    );
    assert.equal(
      sortedTopics.findIndex((row) => row.topicId === probe.topic_id),
      probe.sort_index,
      `${probe.topic_id} effective title order`,
    );
    for (const faction of ["alliance", "empire"]) {
      const expected = probe.images[faction];
      const assetId = selectedImageId(localized, faction);
      assert.equal(assetId, expected?.asset_id ?? null, `${probe.topic_id} ${faction} image selection`);
      if (expected) {
        const descriptor = catalog.images[assetId];
        assert.ok(descriptor, `${probe.topic_id} selected unknown image ${assetId}`);
        assert.equal(descriptor.sha256, expected.sha256, `${probe.topic_id} ${faction} image digest`);
      }
    }
  }

  const noArtCount = Object.values(catalog.topics).filter((topic) =>
    Object.values(topic.localized).every((localized) => selectedImageId(localized, "alliance") === null
      && selectedImageId(localized, "empire") === null)).length;
  assert.equal(noArtCount, scenario.owned_no_art_topic_count, "owned no-art topic count");

  return {
    schema_version: 1,
    family: scenario.family,
    scope: "owned base-byte/package provenance; live rendering is a separate coordinator gate",
    status: "pass",
    source_profile: manifest.source_profile,
    runtime_pack_sha256: sha256(pack.contents),
    runtime_pack_byte_length: pack.contents.length,
    catalog_sha256: sha256(catalogBytes),
    manifest_sha256: sha256(manifestBytes),
    encyclopedia_namespace_entries: namespace.size,
    binding_sources_verified: manifest.binding_sources.length,
    probes_verified: scenario.probes.length,
    visible_text_checks_verified: visibleTextChecks.length,
    body_viewport: scenario.body_viewport,
    owned_no_art_topic_count: noArtCount,
  };
}

function mimeType(file) {
  if (file.endsWith(".html")) return "text/html; charset=utf-8";
  if (file.endsWith(".js")) return "text/javascript; charset=utf-8";
  if (file.endsWith(".wasm")) return "application/wasm";
  return "application/octet-stream";
}

async function startServer(site, observed) {
  const rootWithSeparator = `${path.resolve(site)}${path.sep}`;
  const server = http.createServer((request, response) => {
    const pathname = new URL(request.url, "http://localhost").pathname;
    const relative = path.posix.normalize(decodeURIComponent(pathname)).replace(/^\/+/, "") || "index.html";
    const candidate = path.resolve(site, relative);
    const permitted = candidate.startsWith(rootWithSeparator);
    const exists = permitted && fs.existsSync(candidate);
    observed.push({ url: pathname, status: exists ? 200 : permitted ? 404 : 403 });
    if (!exists) {
      response.writeHead(permitted ? 404 : 403).end();
      return;
    }
    const bytes = fs.readFileSync(candidate);
    response.writeHead(200, {
      "content-type": mimeType(candidate),
      "content-length": bytes.length,
      "cache-control": "no-store",
    }).end(bytes);
  });
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  return server;
}

export function browserExecutable(browserManifest) {
  const candidates = [
    ...(process.env.OPEN_REBELLION_CHROME_FOR_TESTING
      ? [process.env.OPEN_REBELLION_CHROME_FOR_TESTING]
      : []),
    ...browserManifest.executable_candidates,
  ];
  const executable = candidates.find((candidate) => fs.existsSync(candidate));
  if (!executable) throw new Error(`pinned Chrome for Testing ${browserManifest.version} is missing`);
  return executable;
}

function evidenceFromConsole(line, marker) {
  const offset = line.indexOf(marker);
  if (offset < 0) return null;
  return JSON.parse(line.slice(offset + marker.length));
}

/// Parse one feature-only typed surface observation without treating ordinary
/// console output as evidence. Callers retain the complete console artifact;
/// this helper only extracts records carrying the explicit schema/status.
export function surfaceObservationFromConsole(line) {
  const observation = evidenceFromConsole(line, "[encyclopedia_surface_observation] ");
  if (observation === null) return null;
  const exactKeys = (value, expected, label) => {
    assert.ok(value !== null && typeof value === "object" && !Array.isArray(value), `${label} object`);
    assert.deepEqual(Object.keys(value).sort(), [...expected].sort(), `${label} exact shape`);
  };
  exactKeys(
    observation,
    ["schema_version", "status", "record_id", "sequence", "values", "transition"],
    "surface observation",
  );
  assert.equal(observation.schema_version, 5, "surface observation schema");
  assert.equal(observation.status, "surface_observation", "surface observation status");
  assert.match(observation.record_id, /^surface-[0-9]{8}$/, "surface observation record id");
  assert.ok(Number.isSafeInteger(observation.sequence) && observation.sequence > 0, "surface sequence");
  assert.equal(
    observation.record_id,
    `surface-${String(observation.sequence).padStart(8, "0")}`,
    "surface record id matches sequence",
  );
  exactKeys(observation.values, [
    "target", "viewer_faction", "mode", "focused_control", "selected_category_command",
    "visible_topic_ids", "visible_topic_ids_sha256", "selected_topic_id", "selected_topic_index",
    "visible_topic_count", "previous_enabled", "next_enabled",
    "body_scroll_offset", "world_epoch", "world_evidence_kind", "catalog_generation",
    "requested_language", "effective_language", "title_sha256", "body_sha256",
    "asset_id", "asset_digest", "render_profile",
    "selected_source_kind", "live_enabled_mods", "texture_cache_event",
  ], "surface values");
  assert.ok(["native", "browser"].includes(observation.values.target), "surface target");
  assert.ok(
    ["alliance", "empire"].includes(observation.values.viewer_faction),
    "surface viewer faction",
  );
  assert.ok(["index", "topic"].includes(observation.values.mode), "surface mode");
  if (observation.values.focused_control !== null) {
    exactKeys(
      observation.values.focused_control,
      ["kind", "target_id", "focused_id", "owns_focus"],
      "surface focused control",
    );
    assert.ok(
      ["index_list", "topic_body"].includes(observation.values.focused_control.kind),
      "surface focused control kind",
    );
    assert.ok(
      Number.isSafeInteger(observation.values.focused_control.target_id)
        && observation.values.focused_control.target_id > 0,
      "surface focused target id",
    );
    assert.ok(
      observation.values.focused_control.focused_id === null
        || (Number.isSafeInteger(observation.values.focused_control.focused_id)
          && observation.values.focused_control.focused_id > 0),
      "surface focused id",
    );
    assert.equal(
      observation.values.focused_control.owns_focus,
      observation.values.focused_control.focused_id
        === observation.values.focused_control.target_id,
      "surface focus ownership",
    );
  }
  assert.ok(
    observation.values.selected_category_command === null
      || /^0x(?:6f|7[0-5])$/.test(observation.values.selected_category_command),
    "surface category command",
  );
  assert.ok(Array.isArray(observation.values.visible_topic_ids), "surface ordered topic ids");
  assert.equal(
    new Set(observation.values.visible_topic_ids).size,
    observation.values.visible_topic_ids.length,
    "surface ordered topic ids unique",
  );
  for (const topicId of observation.values.visible_topic_ids) {
    assert.match(topicId, /^original:[0-9]+$/, "surface ordered topic id");
  }
  assert.equal(
    observation.values.visible_topic_ids.length,
    observation.values.visible_topic_count,
    "surface ordered topic count",
  );
  assert.match(observation.values.visible_topic_ids_sha256, /^[0-9a-f]{64}$/, "surface membership");
  assert.equal(
    observation.values.visible_topic_ids_sha256,
    sha256(Buffer.from(JSON.stringify(observation.values.visible_topic_ids))),
    "surface ordered topic digest",
  );
  assert.ok(
    observation.values.selected_topic_id === null
      || /^original:[0-9]+$/.test(observation.values.selected_topic_id),
    "surface selected topic",
  );
  assert.ok(
    observation.values.selected_topic_index === null
      || (Number.isSafeInteger(observation.values.selected_topic_index)
        && observation.values.selected_topic_index >= 0),
    "surface selected topic index",
  );
  assert.ok(
    Number.isSafeInteger(observation.values.visible_topic_count)
      && observation.values.visible_topic_count >= 0,
    "surface visible topic count",
  );
  assert.equal(typeof observation.values.previous_enabled, "boolean", "surface previous enabled");
  assert.equal(typeof observation.values.next_enabled, "boolean", "surface next enabled");
  assert.ok(
    Number.isFinite(observation.values.body_scroll_offset)
      && observation.values.body_scroll_offset >= 0,
    "surface body scroll offset",
  );
  assert.ok(
    Number.isSafeInteger(observation.values.world_epoch) && observation.values.world_epoch >= 0,
    "surface world epoch",
  );
  assert.ok(
    ["actual_replacement_world", "catalog_scoped_synthetic_admission"]
      .includes(observation.values.world_evidence_kind),
    "surface world evidence kind",
  );
  assert.ok(
    Number.isSafeInteger(observation.values.catalog_generation)
      && observation.values.catalog_generation >= 0,
    "surface catalog generation",
  );
  assert.match(observation.values.requested_language, /^[0-9]+$/, "surface requested language");
  assert.ok(
    observation.values.effective_language === null
      || /^[0-9]+$/.test(observation.values.effective_language),
    "surface effective language",
  );
  for (const field of ["title_sha256", "body_sha256", "asset_digest"]) {
    assert.ok(
      observation.values[field] === null || /^[0-9a-f]{64}$/.test(observation.values[field]),
      `surface ${field}`,
    );
  }
  assert.ok(
    observation.values.asset_id === null
      || /^(?:edata|mod):[A-Za-z0-9._:-]+$/.test(observation.values.asset_id),
    "surface asset id",
  );
  assert.ok(
    observation.values.render_profile === null
      || ["original_nearest", "faithful_hd_linear"].includes(observation.values.render_profile),
    "surface render profile",
  );
  assert.ok(
    ["base", "approved_hd", "mod", "null", "unavailable"]
      .includes(observation.values.selected_source_kind),
    "surface selected source kind",
  );
  assert.ok(
    Array.isArray(observation.values.live_enabled_mods)
      && observation.values.live_enabled_mods.every(
        (name) => typeof name === "string" && name.length > 0,
      ),
    "surface live enabled mods",
  );
  assert.ok(
    ["not_selected", "uploaded", "cache_hit", "released", "no_art", "failed"]
      .includes(observation.values.texture_cache_event),
    "surface texture cache event",
  );
  exactKeys(
    observation.transition,
    ["from_record_id", "controller_steps", "fixture_controls", "input_attempts", "texture_events"],
    "surface transition",
  );
  assert.ok(Array.isArray(observation.transition.controller_steps), "surface controller steps");
  for (const step of observation.transition.controller_steps) {
    exactKeys(
      step,
      ["input_record_id", "action", "outcome", "before", "after"],
      "surface controller step",
    );
    assert.match(step.input_record_id, /^surface-[0-9]{8}$/, "surface controller input record");
    assert.equal(typeof step.action, "string", "surface controller action");
    assert.equal(typeof step.outcome, "string", "surface controller outcome");
    for (const [phase, state] of [["before", step.before], ["after", step.after]]) {
      exactKeys(state, [
        "mode", "selected_category_command", "selected_topic_id", "visible_topic_ids",
        "visible_topic_ids_sha256",
        "selected_topic_index", "visible_topic_count", "previous_enabled", "next_enabled",
        "world_epoch",
      ], `surface controller ${phase}`);
      assert.ok(["index", "topic"].includes(state.mode), `surface controller ${phase} mode`);
      assert.ok(
        state.selected_category_command === null
          || /^0x(?:6f|7[0-5])$/.test(state.selected_category_command),
        `surface controller ${phase} category`,
      );
      assert.ok(
        state.selected_topic_id === null || /^original:[0-9]+$/.test(state.selected_topic_id),
        `surface controller ${phase} selected topic`,
      );
      assert.ok(
        state.selected_topic_index === null
          || (Number.isSafeInteger(state.selected_topic_index) && state.selected_topic_index >= 0),
        `surface controller ${phase} selected index`,
      );
      assert.ok(
        Number.isSafeInteger(state.visible_topic_count) && state.visible_topic_count >= 0,
        `surface controller ${phase} visible count`,
      );
      assert.ok(Array.isArray(state.visible_topic_ids), `surface controller ${phase} ordered topics`);
      assert.equal(
        state.visible_topic_ids.length,
        state.visible_topic_count,
        `surface controller ${phase} ordered topic count`,
      );
      assert.equal(
        new Set(state.visible_topic_ids).size,
        state.visible_topic_ids.length,
        `surface controller ${phase} ordered topic uniqueness`,
      );
      for (const topicId of state.visible_topic_ids) {
        assert.match(topicId, /^original:[0-9]+$/, `surface controller ${phase} ordered topic id`);
      }
      assert.match(
        state.visible_topic_ids_sha256,
        /^[0-9a-f]{64}$/,
        `surface controller ${phase} membership`,
      );
      assert.equal(
        state.visible_topic_ids_sha256,
        sha256(Buffer.from(JSON.stringify(state.visible_topic_ids))),
        `surface controller ${phase} ordered topic digest`,
      );
      assert.equal(typeof state.previous_enabled, "boolean", `surface controller ${phase} previous`);
      assert.equal(typeof state.next_enabled, "boolean", `surface controller ${phase} next`);
      assert.ok(Number.isSafeInteger(state.world_epoch), `surface controller ${phase} epoch`);
      assert.equal(
        state.selected_topic_id === null,
        state.selected_topic_index === null,
        `surface controller ${phase} topic/index presence`,
      );
      if (state.selected_topic_index !== null) {
        assert.ok(
          state.selected_topic_index < state.visible_topic_count,
          `surface controller ${phase} selected index bound`,
        );
        assert.equal(
          state.selected_topic_id,
          state.visible_topic_ids[state.selected_topic_index],
          `surface controller ${phase} selected topic/index binding`,
        );
      }
    }
  }
  assert.ok(Array.isArray(observation.transition.fixture_controls), "surface fixture controls");
  for (const control of observation.transition.fixture_controls) {
    assert.equal(typeof control, "string", "surface fixture control");
  }
  assert.ok(Array.isArray(observation.transition.input_attempts), "surface input attempts");
  for (const attempt of observation.transition.input_attempts) {
    exactKeys(attempt, ["control", "enabled"], "surface input attempt");
    assert.ok(
      ["previous_topic", "next_topic"].includes(attempt.control),
      "surface input attempt control",
    );
    assert.equal(typeof attempt.enabled, "boolean", "surface input attempt enabled");
  }
  assert.ok(Array.isArray(observation.transition.texture_events), "surface texture events");
  for (const event of observation.transition.texture_events) {
    exactKeys(
      event,
      ["kind", "asset_id", "digest", "render_profile", "cache_hit", "diagnostic"],
      "surface texture event",
    );
    assert.ok(["selected", "released", "failed"].includes(event.kind), "surface texture event kind");
    assert.equal(typeof event.asset_id, "string", "surface texture event asset id");
    assert.ok(event.asset_id.length > 0, "surface texture event nonempty asset id");
    if (event.kind === "selected") {
      assert.match(event.digest, /^[0-9a-f]{64}$/, "selected texture digest");
      assert.ok(
        ["original_nearest", "faithful_hd_linear"].includes(event.render_profile),
        "selected texture profile",
      );
      assert.equal(typeof event.cache_hit, "boolean", "selected texture cache flag");
      assert.equal(event.diagnostic, null, "selected texture diagnostic");
    } else if (event.kind === "released") {
      assert.match(event.digest, /^[0-9a-f]{64}$/, "released texture digest");
      assert.ok(
        ["original_nearest", "faithful_hd_linear"].includes(event.render_profile),
        "released texture profile",
      );
      assert.equal(event.cache_hit, null, "released texture cache flag");
      assert.equal(event.diagnostic, null, "released texture diagnostic");
    } else {
      assert.equal(event.digest, null, "failed texture digest");
      assert.equal(event.render_profile, null, "failed texture profile");
      assert.equal(event.cache_hit, null, "failed texture cache flag");
      assert.equal(typeof event.diagnostic, "string", "failed texture diagnostic");
      assert.ok(event.diagnostic.includes(event.asset_id), "failed texture diagnostic names asset");
    }
  }
  return observation;
}

async function waitForSelection(consoleLines, topicId, fromIndex, timeoutMs = 15_000) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    for (let index = fromIndex; index < consoleLines.length; index += 1) {
      const evidence = evidenceFromConsole(consoleLines[index].text, "selection_evidence=");
      if (evidence?.topic_id === topicId) return { index, evidence };
    }
    await new Promise((resolve) => setTimeout(resolve, 25));
  }
  throw new Error(`timed out waiting for selected topic ${topicId}`);
}

async function waitForViewport(consoleLines, topicId, fromIndex, expectedIntent, timeoutMs = 15_000) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    for (let index = fromIndex; index < consoleLines.length; index += 1) {
      const evidence = evidenceFromConsole(consoleLines[index].text, "viewport_evidence=");
      if (evidence?.topic_id === topicId
        && evidence.consumed_scroll_intents?.includes(expectedIntent)) return { index, evidence };
    }
    await new Promise((resolve) => setTimeout(resolve, 25));
  }
  throw new Error(`timed out waiting for ${expectedIntent} viewport evidence for ${topicId}`);
}

function decodeIndexedBmp(bytes, PNG) {
  assert.equal(bytes.subarray(0, 2).toString("ascii"), "BM");
  const dataOffset = bytes.readUInt32LE(10);
  const headerSize = bytes.readUInt32LE(14);
  const width = bytes.readInt32LE(18);
  const signedHeight = bytes.readInt32LE(22);
  const height = Math.abs(signedHeight);
  assert.equal(bytes.readUInt16LE(28), 8);
  assert.equal(bytes.readUInt32LE(30), 0);
  const paletteOffset = 14 + headerSize;
  const stride = (width + 3) & ~3;
  const image = new PNG({ width, height });
  for (let y = 0; y < height; y += 1) {
    const sourceY = signedHeight > 0 ? height - 1 - y : y;
    for (let x = 0; x < width; x += 1) {
      const paletteIndex = bytes[dataOffset + sourceY * stride + x];
      const palette = paletteOffset + paletteIndex * 4;
      const destination = (y * width + x) * 4;
      image.data[destination] = bytes[palette + 2];
      image.data[destination + 1] = bytes[palette + 1];
      image.data[destination + 2] = bytes[palette];
      image.data[destination + 3] = 255;
    }
  }
  return image;
}

function compareArtwork(screenshotBytes, sourceBytes, PNG) {
  const screenshot = PNG.sync.read(screenshotBytes);
  const source = decodeIndexedBmp(sourceBytes, PNG);
  assert.deepEqual([source.width, source.height], [400, 200]);
  let differentPixels = 0;
  for (let y = 0; y < source.height; y += 1) {
    for (let x = 0; x < source.width; x += 1) {
      const expected = (y * source.width + x) * 4;
      const actual = ((artOrigin.y + y) * screenshot.width + artOrigin.x + x) * 4;
      if (screenshot.data[actual] !== source.data[expected]
        || screenshot.data[actual + 1] !== source.data[expected + 1]
        || screenshot.data[actual + 2] !== source.data[expected + 2]) differentPixels += 1;
    }
  }
  return { width: source.width, height: source.height, different_pixels: differentPixels };
}

function cropViewport(screenshotBytes, viewport, PNG) {
  const screenshot = PNG.sync.read(screenshotBytes);
  assert.ok(viewport.x + viewport.width <= screenshot.width, "body viewport exceeds screenshot width");
  assert.ok(viewport.y + viewport.height <= screenshot.height, "body viewport exceeds screenshot height");
  const crop = new PNG({ width: viewport.width, height: viewport.height });
  for (let row = 0; row < viewport.height; row += 1) {
    const source = ((viewport.y + row) * screenshot.width + viewport.x) * 4;
    const destination = row * viewport.width * 4;
    screenshot.data.copy(crop.data, destination, source, source + viewport.width * 4);
  }
  return PNG.sync.write(crop);
}

function differentPixelCount(leftBytes, rightBytes, PNG) {
  const left = PNG.sync.read(leftBytes);
  const right = PNG.sync.read(rightBytes);
  assert.deepEqual([left.width, left.height], [right.width, right.height]);
  let different = 0;
  for (let offset = 0; offset < left.data.length; offset += 4) {
    if (left.data[offset] !== right.data[offset]
      || left.data[offset + 1] !== right.data[offset + 1]
      || left.data[offset + 2] !== right.data[offset + 2]
      || left.data[offset + 3] !== right.data[offset + 3]) different += 1;
  }
  return different;
}

/// Run an optional bounded surface journey after Ready and offline transition.
/// Existing E21/E48 callers omit the hook and retain byte-identical control
/// flow. A supplemental journey must restore the accepted initial topic before
/// the shared canonical probes resume.
export async function runOptionalReadyJourney(hooks, context) {
  if (typeof hooks.runReadyJourney !== "function") return null;
  return hooks.runReadyJourney(context);
}

export async function runBrowser(options, artifactResult, hooks = {}) {
  const scenario = readJson(options.scenario);
  for (const required of ["index.html", "gl.js", "open-rebellion-test.wasm", "data/runtime.orpk"]) {
    assert.ok(fs.existsSync(path.join(options.site, required)), `packed site is missing ${required}`);
  }
  const dependencyRoot = process.env.OPEN_REBELLION_INTERFACE_NODE_MODULES;
  const playwrightModule = dependencyRoot
    ? pathToFileURL(path.join(dependencyRoot, "playwright-core/index.mjs")).href
    : "playwright-core";
  const pngModule = dependencyRoot
    ? pathToFileURL(path.join(dependencyRoot, "pngjs/lib/png.js")).href
    : "pngjs";
  const [{ chromium }, { PNG }, { launchBrowser }] = await Promise.all([
    import(playwrightModule),
    import(pngModule),
    import("./browser-launch.mjs"),
  ]);
  const browserManifest = readJson(path.join(here, "browser.json"));
  const serverRequests = [];
  const ownedServer = hooks.origin ? null : await startServer(options.site, serverRequests);
  const origin = hooks.origin || `http://127.0.0.1:${ownedServer.address().port}`;
  const result = {
    ...artifactResult,
    scope: hooks.scope || "owned base-byte native/packed transport and rendering checkpoint",
    cases: [],
  };
  let browser;
  try {
    browser = await launchBrowser(chromium, {
      executablePath: browserExecutable(browserManifest),
      headless: true,
      args: browserManifest.launch_arguments,
      timeout: 30_000,
    }, []);
    result.browser_version = browser.version();
    result.muted = browserManifest.launch_arguments.includes("--mute-audio");
    for (const faction of ["alliance", "empire"]) {
      const context = await browser.newContext({
        viewport: { width: 640, height: 480 },
        deviceScaleFactor: 1,
        locale: "en-US",
        timezoneId: "America/New_York",
        reducedMotion: "reduce",
        serviceWorkers: "block",
      });
      const page = await context.newPage();
      const browserRequests = [];
      const consoleLines = [];
      const errors = [];
      page.on("request", (request) => browserRequests.push(new URL(request.url()).pathname));
      page.on("requestfailed", (request) => errors.push(`request:${request.url()}:${request.failure()?.errorText}`));
      page.on("pageerror", (error) => errors.push(`page:${error.stack || error.message}`));
      page.on("console", (message) => {
        consoleLines.push({ type: message.type(), text: message.text() });
        if (message.type() === "error") errors.push(`console:${message.text()}`);
      });
      const code = hooks.fixtureCodes?.[faction] ?? scenario.fixture_codes[faction];
      const serverStart = serverRequests.length;
      const caseToken = hooks.onCaseStart?.({ faction });
      let ready = null;
      try {
        await page.goto(`${origin}/?fixture-code=${code}`, { waitUntil: "load", timeout: 30_000 });
        await page.waitForFunction(
          () => window.__openRebellionInterfaceReady?.status,
          null,
          { timeout: hooks.reportTimeoutMs ?? 30_000 },
        );
        ready = await page.evaluate(() => window.__openRebellionInterfaceReady);
      assert.equal(ready.status, "ready", JSON.stringify(ready));
      assert.equal(ready.faction, faction);
      assert.equal(ready.source_profile, scenario.source_profile);
      hooks.assertReady?.({ faction, ready });
      await context.setOffline(true);
      const navigationRequestStart = browserRequests.length;
      const supplementalJourney = await runOptionalReadyJourney(hooks, {
        browserRequests,
        consoleLines,
        context,
        faction,
        outputDirectory: path.dirname(options.output),
        page,
        ready,
        scenario,
      });
      if (supplementalJourney !== null) {
        assert.equal(
          supplementalJourney.reset_to_initial_topic,
          true,
          "supplemental journey must restore the accepted initial topic",
        );
      }
      let currentIndex = 0;
      let consoleIndex = 0;
      const probes = [];
      for (const probe of [...scenario.probes].sort((left, right) => left.sort_index - right.sort_index)) {
        while (currentIndex < probe.sort_index) {
          await page.keyboard.press("ArrowRight");
          currentIndex += 1;
          await page.waitForTimeout(20);
        }
        const selected = await waitForSelection(consoleLines, probe.topic_id, consoleIndex);
        consoleIndex = selected.index + 1;
        assert.equal(selected.evidence.viewer, faction);
        assert.equal(selected.evidence.binding_family, probe.binding.family);
        assert.equal(selected.evidence.binding_dat_id, probe.binding.dat_id);
        assert.equal(selected.evidence.binding_variant, probe.binding.variant);
        assert.equal(selected.evidence.title_sha256, probe.title_sha256);
        assert.equal(selected.evidence.body_sha256, probe.body_sha256);
        const expectedImage = probe.images[faction];
        assert.equal(selected.evidence.asset_id, expectedImage?.asset_id ?? null);
        assert.equal(selected.evidence.digest, expectedImage?.sha256 ?? null);
        await page.waitForTimeout(80);
        const screenshot = await page.screenshot({ animations: "disabled" });
        let pixels = null;
        if (expectedImage) {
          const catalog = readJson(path.join(options.sourceRoot, "encyclopedia/catalog.json"));
          const sourceBytes = fs.readFileSync(path.join(
            options.sourceRoot,
            "encyclopedia",
            catalog.images[expectedImage.asset_id].path,
          ));
          pixels = compareArtwork(screenshot, sourceBytes, PNG);
          assert.equal(pixels.different_pixels, 0, `${faction} ${probe.topic_id} artwork pixels`);
        }
        const cacheHit = consoleLines.some((line) =>
          line.text.includes("texture_event=Selected")
          && line.text.includes(`asset_id: \"${expectedImage?.asset_id}\"`)
          && line.text.includes("cache_hit: true"));
        assert.equal(cacheHit, Boolean(expectedImage), `${faction} ${probe.topic_id} cache-hit event`);
        const screenshotPath = path.join(path.dirname(options.output), `${faction}-${probe.topic_id.replace(":", "-")}.png`);
        fs.mkdirSync(path.dirname(screenshotPath), { recursive: true });
        fs.writeFileSync(screenshotPath, screenshot);
        const visibleText = [];
        for (const check of scenario.visible_text_checks.filter((candidate) => candidate.topic_id === probe.topic_id)) {
          const prefix = `${faction}-${probe.topic_id.replace(":", "-")}-${check.purpose}`;
          let viewportBytes = cropViewport(screenshot, scenario.body_viewport, PNG);
          const initialViewportPath = path.join(path.dirname(options.output), `${prefix}-initial.png`);
          fs.writeFileSync(initialViewportPath, viewportBytes);
          const captures = [{
            stage: "initial",
            scroll_offset: 0,
            screenshot: path.resolve(initialViewportPath),
            screenshot_sha256: sha256(viewportBytes),
          }];

          if (check.purpose === "long_body_scroll") {
            let previousOffset = 0;
            for (let step = 0; step < check.input_sequence.length; step += 1) {
              const key = check.input_sequence[step];
              const evidenceStart = consoleLines.length;
              await page.keyboard.press(key);
              const observed = await waitForViewport(
                consoleLines,
                probe.topic_id,
                evidenceStart,
                "page_down",
              );
              assert.equal(observed.evidence.source_profile, scenario.source_profile);
              assert.equal(observed.evidence.viewer, faction);
              assert.equal(observed.evidence.body_sha256, probe.body_sha256);
              assert.deepEqual(observed.evidence.consumed_scroll_intents, ["page_down"]);
              assert.ok(
                observed.evidence.scroll_offset > previousOffset,
                `${faction} ${probe.topic_id} PageDown ${step + 1} did not advance`,
              );
              await page.waitForTimeout(80);
              const full = await page.screenshot({ animations: "disabled" });
              const nextViewport = cropViewport(full, scenario.body_viewport, PNG);
              const changedPixels = differentPixelCount(viewportBytes, nextViewport, PNG);
              assert.ok(changedPixels > 0, `${faction} ${probe.topic_id} PageDown ${step + 1} did not change the viewport`);
              const capturePath = path.join(path.dirname(options.output), `${prefix}-page-${step + 1}.png`);
              fs.writeFileSync(capturePath, nextViewport);
              captures.push({
                stage: `after_${key}_${step + 1}`,
                scroll_offset: observed.evidence.scroll_offset,
                consumed_scroll_intents: observed.evidence.consumed_scroll_intents,
                changed_pixels_from_previous: changedPixels,
                screenshot: path.resolve(capturePath),
                screenshot_sha256: sha256(nextViewport),
              });
              previousOffset = observed.evidence.scroll_offset;
              viewportBytes = nextViewport;
            }
            assert.equal(captures.length, check.expected_distinct_viewports);
          }

          visibleText.push({
            purpose: check.purpose,
            input_sequence: check.input_sequence,
            body_viewport: scenario.body_viewport,
            codepoint: check.codepoint ?? null,
            expected_capture: check.expected_capture ?? null,
            captures,
          });
        }
        probes.push({
          topic_id: probe.topic_id,
          selection: selected.evidence,
          pixels,
          screenshot: path.resolve(screenshotPath),
          screenshot_sha256: sha256(screenshot),
          cache_hit_observed: cacheHit,
          visible_text: visibleText,
        });
      }
      assert.equal(browserRequests.length, navigationRequestStart, `${faction} offline navigation requests`);
      if (hooks.assertStartupRequests) {
        await hooks.assertStartupRequests({
          browserRequests,
          caseToken,
          faction,
          navigationRequestStart,
        });
      } else {
        assert.deepEqual([...browserRequests].sort(), [...expectedRequests].sort(), `${faction} startup requests`);
      }
      const diagnosticClassification = hooks.classifyDiagnostics
        ? await hooks.classifyDiagnostics({
          caseToken,
          diagnostics: [...errors],
          faction,
          ready,
        })
        : { expected: [], fatal: [...errors], raw: [...errors] };
      assert.deepEqual(diagnosticClassification.fatal, [], `${faction} browser diagnostics`);
      const caseResult = {
        faction,
        fixture_code: code,
        ready,
        requests: browserRequests,
        navigation_requests: browserRequests.length - navigationRequestStart,
        browser_diagnostics: diagnosticClassification,
        probes,
      };
      if (supplementalJourney !== null) caseResult.supplemental_journey = supplementalJourney;
      result.cases.push(caseResult);
      if (ownedServer) {
        assert.equal(serverRequests.length - serverStart, expectedRequests.length, `${faction} server request count`);
      }
      await hooks.onCaseComplete?.({ caseResult, faction });
      } catch (error) {
        await hooks.onCaseFailure?.({
          browserRequests: [...browserRequests],
          caseToken,
          consoleLines: [...consoleLines],
          errors: [...errors],
          error,
          faction,
          fixtureCode: code,
          ready,
        });
        throw error;
      } finally {
        await context.close();
      }
    }
  } finally {
    if (browser) await browser.close();
    if (ownedServer) await new Promise((resolve) => ownedServer.close(resolve));
  }
  return result;
}

async function main() {
  const options = parseArguments(process.argv.slice(2));
  const artifactResult = verifyArtifacts(options);
  const result = options.verifyOnly ? artifactResult : await runBrowser(options, artifactResult);
  fs.mkdirSync(path.dirname(options.output), { recursive: true });
  fs.writeFileSync(options.output, `${JSON.stringify(result, null, 2)}\n`);
  console.log(JSON.stringify({ status: result.status, output: options.output }));
}

if (path.resolve(process.argv[1] || "") === fileURLToPath(import.meta.url)) {
  main().catch((error) => {
    console.error(error.stack || error);
    process.exitCode = 1;
  });
}
