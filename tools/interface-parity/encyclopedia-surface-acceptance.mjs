#!/usr/bin/env node

import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { deflateSync, inflateSync } from "node:zlib";

import {
  runOptionalReadyJourney,
  surfaceObservationFromConsole,
} from "./encyclopedia-smoke.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));

const requiredCategoryCommands = Object.freeze([
  "0x6f",
  "0x70",
  "0x71",
  "0x72",
  "0x73",
  "0x74",
  "0x75",
]);
const requiredMatrixRows = Object.freeze([
  "categories",
  "selection",
  "index_keyboard",
  "topic_navigation",
  "scroll_text",
  "language_fallback",
  "world_change",
  "native_precedence",
]);

function uniqueStrings(values, label) {
  assert.ok(Array.isArray(values), `${label} must be an array`);
  for (const value of values) assert.equal(typeof value, "string", `${label} entry`);
  assert.equal(new Set(values).size, values.length, `${label} contains duplicates`);
  return new Set(values);
}

export function validateSurfaceScenario(scenario) {
  assert.equal(scenario.schema_version, 2, "surface scenario schema version");
  assert.equal(scenario.family, "encyclopedia-surface-acceptance", "surface scenario family");
  assert.equal(scenario.corpus_applicability?.original_no_art?.status, "not_applicable");
  assert.equal(scenario.corpus_applicability?.original_no_art?.bound_rows, 347);
  assert.equal(scenario.corpus_applicability?.original_no_art?.rows_with_art, 347);
  assert.equal(
    scenario.corpus_applicability?.alternate_art?.status,
    "deferred",
    "unproven alternate art remains deferred",
  );

  assert.ok(Array.isArray(scenario.category_commands), "category_commands must be an array");
  const commands = new Map(scenario.category_commands.map((entry) => [entry.command, entry]));
  assert.equal(commands.size, scenario.category_commands.length, "category_commands contains duplicates");
  for (const command of requiredCategoryCommands) {
    assert.ok(commands.has(command), `missing category command ${command}`);
  }
  assert.equal(commands.size, requiredCategoryCommands.length, "unexpected category command");
  for (const command of requiredCategoryCommands) {
    const entry = commands.get(command);
    assert.ok(Number.isSafeInteger(entry.topic_count) && entry.topic_count > 0, `${command} topic count`);
    assert.match(entry.visible_topic_ids_sha256, /^[0-9a-f]{64}$/, `${command} projection digest`);
  }

  assert.ok(Array.isArray(scenario.matrix), "matrix must be an array");
  const rows = new Map(scenario.matrix.map((row) => [row.id, row]));
  assert.equal(rows.size, scenario.matrix.length, "matrix contains duplicate row ids");
  for (const rowId of requiredMatrixRows) assert.ok(rows.has(rowId), `missing matrix row ${rowId}`);
  assert.equal(rows.size, requiredMatrixRows.length, "unexpected matrix row");

  let matrixCells = 0;
  for (const row of rows.values()) {
    const targets = uniqueStrings(row.targets, `${row.id}.targets`);
    const factions = uniqueStrings(row.factions, `${row.id}.factions`);
    assert.ok(targets.size > 0, `${row.id} requires a target`);
    assert.ok(factions.has("alliance") && factions.has("empire"), `${row.id} requires both factions`);
    for (const target of targets) assert.ok(["native", "browser"].includes(target), `${row.id} target ${target}`);
    const requiredFields = uniqueStrings(row.required_fields, `${row.id}.required_fields`);
    assert.ok(requiredFields.size > 0, `${row.id} requires observable fields`);
    uniqueStrings(row.actions, `${row.id}.actions`);
    uniqueStrings(row.expected, `${row.id}.expected`);
    matrixCells += targets.size * factions.size;
  }

  const runtimeFields = uniqueStrings(scenario.required_runtime_fields, "required_runtime_fields");
  assert.ok(runtimeFields.has("live_enabled_mods"), "live enabled state must be observed in-process");
  assert.ok(runtimeFields.has("world_epoch"), "world replacement must expose its epoch");
  assert.ok(runtimeFields.has("effective_language"), "whole-record fallback must expose effective language");
  assert.ok(runtimeFields.has("selected_source_kind"), "image precedence must expose selected source kind");
  assert.deepEqual(
    Object.keys(scenario.feature_controls ?? {}),
    ["F2", "F3", "F4", "F5", "F6", "F7", "F8"],
    "feature-only evidence controls must remain finite and explicit",
  );
  assert.equal(scenario.strict_a0?.required_source, "measured_original_windows");
  assert.equal(scenario.strict_a0?.matrix_rows, 21, "strict A0 row count");
  const strictRows = uniqueStrings(scenario.strict_a0?.row_ids, "strict_a0.row_ids");
  assert.equal(strictRows.size, 21, "strict A0 exact row identities");
  for (let index = 1; index <= 21; index += 1) {
    assert.ok(strictRows.has(`ENC-UI-${String(index).padStart(2, "0")}`), `strict A0 row ${index}`);
  }
  assert.ok(
    ["pending_r6_rebuild", "stale_r6_source_changed", "accepted"]
      .includes(scenario.runtime_identities?.status),
    "runtime identity status",
  );
  const worldRow = rows.get("world_change");
  assert.equal(worldRow.evidence_class, "synthetic_supplement", "world row evidence class");
  assert.ok(
    scenario.known_missing_seams.some(({ id }) => id === "actual_replacement_world_admission_publication"),
    "actual replacement-world seam remains explicit",
  );

  return {
    category_commands: commands.size,
    matrix_rows: rows.size,
    matrix_cells: matrixCells,
    runtime_fields: runtimeFields.size,
  };
}

function expectedMatrixCells(scenario) {
  return scenario.matrix.flatMap((row) => row.targets.flatMap((target) =>
    row.factions.map((faction) => ({
      key: `${row.id}:${target}:${faction}`,
      row,
      target,
      faction,
      categoryCommands: scenario.category_commands,
      categoryAnchorTopicId: scenario.canonical_probes[0].topic_id,
    }))));
}

function exactObject(value, keys, label) {
  assert.ok(value !== null && typeof value === "object" && !Array.isArray(value), `${label} object`);
  assert.deepEqual(Object.keys(value).sort(), [...keys].sort(), `${label} exact shape`);
}

function retainedFile(reference, label) {
  assert.ok(reference && typeof reference === "object", `${label} reference`);
  assert.ok(path.isAbsolute(reference.path), `${label} path must be absolute`);
  assert.match(reference.sha256, /^[0-9a-f]{64}$/, `${label} digest`);
  const bytes = fs.readFileSync(reference.path);
  assert.equal(sha256(bytes), reference.sha256, `${label} identity`);
  return bytes;
}

function crc32(bytes) {
  let crc = 0xffff_ffff;
  for (const byte of bytes) {
    crc ^= byte;
    for (let bit = 0; bit < 8; bit += 1) {
      crc = (crc >>> 1) ^ ((crc & 1) === 0 ? 0 : 0xedb8_8320);
    }
  }
  return (crc ^ 0xffff_ffff) >>> 0;
}

function pngDimensions(bytes, label) {
  assert.ok(bytes.length >= 45, `${label} PNG length`);
  assert.deepEqual([...bytes.subarray(0, 8)], [137, 80, 78, 71, 13, 10, 26, 10], `${label} PNG signature`);
  let offset = 8;
  let ihdr = null;
  let sawIend = false;
  let sawIdat = false;
  let idatSequenceEnded = false;
  let sawPlte = false;
  const imageData = [];
  while (offset < bytes.length) {
    assert.ok(offset + 12 <= bytes.length, `${label} PNG chunk header`);
    const length = bytes.readUInt32BE(offset);
    const end = offset + 12 + length;
    assert.ok(end <= bytes.length, `${label} PNG chunk length`);
    const type = bytes.subarray(offset + 4, offset + 8);
    const typeName = type.toString("ascii");
    assert.match(typeName, /^[A-Za-z]{4}$/, `${label} PNG chunk type`);
    assert.equal(type[2] & 0x20, 0, `${label} PNG reserved chunk bit`);
    if ((type[0] & 0x20) === 0) {
      assert.ok(
        ["IHDR", "PLTE", "IDAT", "IEND"].includes(typeName),
        `${label} PNG unknown critical chunk ${typeName}`,
      );
    }
    const data = bytes.subarray(offset + 8, offset + 8 + length);
    assert.equal(
      bytes.readUInt32BE(offset + 8 + length),
      crc32(Buffer.concat([type, data])),
      `${label} PNG ${typeName} CRC`,
    );
    if (offset === 8) assert.equal(typeName, "IHDR", `${label} PNG first chunk`);
    if (typeName === "IHDR") {
      assert.equal(ihdr, null, `${label} PNG duplicate IHDR`);
      assert.equal(length, 13, `${label} PNG IHDR length`);
      ihdr = {
        width: data.readUInt32BE(0),
        height: data.readUInt32BE(4),
        bitDepth: data[8],
        colorType: data[9],
        compression: data[10],
        filter: data[11],
        interlace: data[12],
      };
      assert.ok(ihdr.width > 0 && ihdr.height > 0, `${label} PNG dimensions`);
      assert.ok(ihdr.width <= 32_768 && ihdr.height <= 32_768, `${label} PNG dimensions bounded`);
      assert.equal(ihdr.bitDepth, 8, `${label} PNG bit depth`);
      assert.ok([0, 2, 4, 6].includes(ihdr.colorType), `${label} PNG color type`);
      assert.equal(ihdr.compression, 0, `${label} PNG compression`);
      assert.equal(ihdr.filter, 0, `${label} PNG filter method`);
      assert.equal(ihdr.interlace, 0, `${label} PNG interlace`);
    } else if (typeName === "PLTE") {
      assert.ok(!sawPlte, `${label} PNG duplicate PLTE`);
      assert.ok(!sawIdat, `${label} PNG PLTE must precede IDAT`);
      assert.ok(ihdr && [2, 6].includes(ihdr.colorType), `${label} PNG PLTE color type`);
      assert.ok(length > 0 && length <= 768 && length % 3 === 0, `${label} PNG PLTE length`);
      sawPlte = true;
    } else if (typeName === "IDAT") {
      assert.ok(!idatSequenceEnded, `${label} PNG IDAT chunks must be consecutive`);
      sawIdat = true;
      imageData.push(data);
    } else if (typeName === "IEND") {
      assert.equal(length, 0, `${label} PNG IEND length`);
      sawIend = true;
      assert.equal(end, bytes.length, `${label} PNG trailing data`);
    } else if (sawIdat) {
      idatSequenceEnded = true;
    }
    offset = end;
    if (sawIend) break;
  }
  assert.ok(ihdr, `${label} PNG IHDR`);
  assert.ok(imageData.length > 0, `${label} PNG IDAT`);
  assert.ok(sawIend, `${label} PNG IEND`);
  let decoded;
  const channels = new Map([[0, 1], [2, 3], [4, 2], [6, 4]]).get(ihdr.colorType);
  const rowBytes = ihdr.width * channels;
  const expectedBytes = ihdr.height * (rowBytes + 1);
  assert.ok(Number.isSafeInteger(rowBytes), `${label} PNG row byte count`);
  assert.ok(expectedBytes <= 256 * 1024 * 1024, `${label} PNG decoded bytes bounded`);
  try {
    decoded = inflateSync(Buffer.concat(imageData), { maxOutputLength: expectedBytes + 1 });
  } catch (error) {
    assert.fail(`${label} PNG decode: ${error.message}`);
  }
  assert.equal(decoded.length, expectedBytes, `${label} PNG decoded length`);
  for (let row = 0; row < ihdr.height; row += 1) {
    assert.ok(decoded[row * (rowBytes + 1)] <= 4, `${label} PNG row filter`);
  }
  return { width: ihdr.width, height: ihdr.height };
}

function selfTestPngChunk(typeName, data) {
  const type = Buffer.from(typeName, "ascii");
  const result = Buffer.alloc(12 + data.length);
  result.writeUInt32BE(data.length, 0);
  type.copy(result, 4);
  data.copy(result, 8);
  result.writeUInt32BE(crc32(Buffer.concat([type, data])), 8 + data.length);
  return result;
}

function selfTestPng(red = 0) {
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(1, 0);
  ihdr.writeUInt32BE(1, 4);
  ihdr[8] = 8;
  ihdr[9] = 6;
  const idat = deflateSync(Buffer.from([0, red, 0, 0, 255]));
  return Buffer.concat([
    Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]),
    selfTestPngChunk("IHDR", ihdr),
    selfTestPngChunk("IDAT", idat),
    selfTestPngChunk("IEND", Buffer.alloc(0)),
  ]);
}

function validIdentity(value, label) {
  assert.equal(typeof value, "string", `${label} type`);
  assert.match(value, /^[A-Za-z0-9._:-]+$/, label);
}

function validTimestamp(value, label) {
  assert.equal(typeof value, "string", `${label} type`);
  assert.match(
    value,
    /^20[0-9]{2}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}(?:\.[0-9]+)?(?:Z|[+-][0-9]{2}:[0-9]{2})$/,
    label,
  );
  assert.ok(Number.isFinite(Date.parse(value)), `${label} value`);
}

function validateAttachments(artifacts, capture, recordIds, rowId, label) {
  assert.ok(Array.isArray(artifacts), `${label}.artifacts must be an array`);
  const screenshots = [];
  let network = null;
  for (const artifact of artifacts) {
    if (artifact.kind === "screenshot") {
      exactObject(
        artifact,
        [
          "kind", "path", "sha256", "media_type", "width", "height", "row_id",
          "capture_id", "session_id", "record_id",
        ],
        `${label} screenshot`,
      );
      assert.equal(artifact.media_type, "image/png", `${label} screenshot media type`);
      assert.equal(artifact.row_id, rowId, `${label} screenshot row`);
      assert.equal(artifact.capture_id, capture.capture_id, `${label} screenshot capture`);
      assert.equal(artifact.session_id, capture.session_id, `${label} screenshot session`);
      assert.ok(recordIds.includes(artifact.record_id), `${label} screenshot record binding`);
      const bytes = retainedFile(artifact, `${label} screenshot`);
      assert.deepEqual(
        pngDimensions(bytes, `${label} screenshot`),
        { width: artifact.width, height: artifact.height },
        `${label} screenshot dimensions`,
      );
      screenshots.push(artifact);
    } else if (artifact.kind === "network_log") {
      exactObject(artifact, ["kind", "path", "sha256", "media_type"], `${label} network artifact`);
      assert.equal(artifact.media_type, "application/json", `${label} network media type`);
      assert.equal(network, null, `${label} has one network log`);
      const payload = JSON.parse(retainedFile(artifact, `${label} network log`));
      exactObject(
        payload,
        ["schema_version", "capture_id", "session_id", "offline_boundary_sequence", "requests"],
        `${label} network payload`,
      );
      assert.equal(payload.schema_version, 1, `${label} network schema`);
      assert.equal(payload.capture_id, capture.capture_id, `${label} network capture`);
      assert.equal(payload.session_id, capture.session_id, `${label} network session`);
      assert.ok(Number.isSafeInteger(payload.offline_boundary_sequence), `${label} offline boundary`);
      assert.ok(Array.isArray(payload.requests), `${label} network requests`);
      for (const request of payload.requests) {
        exactObject(request, ["sequence", "method", "url", "status", "phase"], `${label} request`);
      }
      const navigationRequests = payload.requests.filter((request) =>
        request.sequence > payload.offline_boundary_sequence || request.phase === "offline_navigation");
      assert.equal(navigationRequests.length, 0, `${label} offline navigation requests`);
      network = { artifact, payload };
    } else {
      assert.fail(`${label} unsupported artifact kind ${artifact.kind}`);
    }
  }
  assert.ok(screenshots.length > 0, `${label} requires a typed screenshot`);
  if (capture.target === "browser") assert.ok(network, `${label} browser requires a typed network log`);
  else assert.equal(network, null, `${label} native capture must not invent a network log`);
  return { screenshots, network };
}

const navigationOutcomes = new Set([
  "applied",
  "no_change",
  "rejected",
  "scroll_requested",
  "close_requested",
  "return_forwarded",
]);

function fieldIsSemanticallyValid(field, value) {
  switch (field) {
    case "target": return value === "native" || value === "browser";
    case "viewer_faction": return value === "alliance" || value === "empire";
    case "mode": return value === "index" || value === "topic";
    case "focused_control":
      return value !== null
        && typeof value === "object"
        && ["index_list", "topic_body"].includes(value.kind)
        && Number.isSafeInteger(value.target_id)
        && value.target_id > 0
        && (value.focused_id === null
          || (Number.isSafeInteger(value.focused_id) && value.focused_id > 0))
        && typeof value.owns_focus === "boolean"
        && value.owns_focus === (value.focused_id === value.target_id);
    case "selected_category_command":
      return value === null || /^0x(?:6f|7[0-5])$/.test(value);
    case "visible_topic_ids":
      return Array.isArray(value)
        && new Set(value).size === value.length
        && value.every((topicId) => /^original:[0-9]+$/.test(topicId));
    case "visible_topic_ids_sha256":
    case "asset_digest":
    case "title_sha256":
    case "body_sha256": return value === null || /^[0-9a-f]{64}$/.test(value);
    case "selected_topic_id": return value === null || /^original:[0-9]+$/.test(value);
    case "selected_topic_index": return value === null
      || (Number.isSafeInteger(value) && value >= 0);
    case "visible_topic_count": return Number.isSafeInteger(value) && value >= 0;
    case "previous_enabled":
    case "next_enabled": return typeof value === "boolean";
    case "body_scroll_offset": return Number.isFinite(value) && value >= 0;
    case "world_epoch":
    case "catalog_generation": return Number.isSafeInteger(value) && value >= 0;
    case "world_evidence_kind":
      return ["actual_replacement_world", "catalog_scoped_synthetic_admission"].includes(value);
    case "requested_language": return typeof value === "string" && /^[0-9]+$/.test(value);
    case "effective_language": return value === null
      || (typeof value === "string" && /^[0-9]+$/.test(value));
    case "asset_id": return value === null || /^(?:edata|mod):[A-Za-z0-9._:-]+$/.test(value);
    case "render_profile":
      return value === null || ["original_nearest", "faithful_hd_linear"].includes(value);
    case "selected_source_kind":
      return ["base", "approved_hd", "mod", "null", "unavailable"].includes(value);
    case "live_enabled_mods":
      return Array.isArray(value) && value.every((name) => typeof name === "string" && name.length > 0);
    case "texture_cache_event":
      return ["not_selected", "uploaded", "cache_hit", "released", "no_art", "failed"].includes(value);
    case "navigation_request_count": return value === 0;
    default: return false;
  }
}

function expectedBuildIdentity(scenario, target) {
  return target === "native"
    ? scenario.runtime_identities.native_feature_binary
    : scenario.runtime_identities.browser_feature_wasm;
}

function loadRawCapture(scenario, reference, cell, semanticFailures) {
  exactObject(reference, ["path", "sha256"], `${cell.key} capture reference`);
  const payload = JSON.parse(retainedFile(reference, `${cell.key} raw console capture`));
  exactObject(payload, [
    "schema_version", "family", "capture_id", "session_id", "target", "faction",
    "fixture_code", "source_profile", "runtime_pack_sha256", "build", "console_lines",
  ], `${cell.key} raw console capture`);
  assert.equal(payload.schema_version, 1, `${cell.key} capture schema`);
  assert.equal(payload.family, scenario.family, `${cell.key} capture family`);
  assert.match(payload.capture_id, /^[A-Za-z0-9._:-]+$/, `${cell.key} capture id`);
  assert.match(payload.session_id, /^[A-Za-z0-9._:-]+$/, `${cell.key} session id`);
  assert.equal(payload.target, cell.target, `${cell.key} capture target`);
  assert.equal(payload.faction, cell.faction, `${cell.key} capture faction`);
  const expectedCode = scenario.fixture_codes.packed[cell.faction];
  assert.equal(payload.fixture_code, expectedCode, `${cell.key} fixture code`);
  assert.equal(payload.source_profile, scenario.canonical.source_profile, `${cell.key} source profile`);
  assert.equal(payload.runtime_pack_sha256, scenario.canonical.runtime_pack_sha256, `${cell.key} pack`);
  exactObject(payload.build, ["kind", "sha256", "byte_length"], `${cell.key} build identity`);
  const expectedBuild = expectedBuildIdentity(scenario, cell.target);
  const expectedKind = cell.target === "native" ? "native_feature_binary" : "browser_feature_wasm";
  if (scenario.runtime_identities.status !== "accepted"
    || expectedBuild.sha256 === null || expectedBuild.byte_length === null) {
    semanticFailures.push({ cell: cell.key, field: "build", detail: "reviewed current-source build identity is pending" });
  } else if (payload.build.kind !== expectedKind
    || payload.build.sha256 !== expectedBuild.sha256
    || payload.build.byte_length !== expectedBuild.byte_length) {
    semanticFailures.push({ cell: cell.key, field: "build", detail: "capture build identity differs" });
  }
  assert.ok(Array.isArray(payload.console_lines), `${cell.key} console lines`);
  const records = [];
  for (const [index, line] of payload.console_lines.entries()) {
    exactObject(line, ["sequence", "text"], `${cell.key} console line`);
    assert.equal(line.sequence, index + 1, `${cell.key} console sequence`);
    assert.equal(typeof line.text, "string", `${cell.key} console text`);
    const record = surfaceObservationFromConsole(line.text);
    if (record) records.push(record);
  }
  const recordIds = new Set();
  for (const [index, record] of records.entries()) {
    assert.ok(!recordIds.has(record.record_id), `${cell.key} duplicate runtime record`);
    recordIds.add(record.record_id);
    if (index === 0) {
      assert.equal(record.transition.from_record_id, null, `${cell.key} initial transition origin`);
    } else {
      assert.equal(
        record.transition.from_record_id,
        records[index - 1].record_id,
        `${cell.key} consecutive transition origin`,
      );
      assert.equal(record.sequence, records[index - 1].sequence + 1, `${cell.key} record sequence`);
    }
  }
  return { payload, records };
}

function selectedConsecutiveRecords(capture, recordIds, cell) {
  uniqueStrings(recordIds, `${cell.key}.record_ids`);
  assert.ok(recordIds.length > 0, `${cell.key} requires runtime record ids`);
  const positions = recordIds.map((recordId) =>
    capture.records.findIndex((record) => record.record_id === recordId));
  assert.ok(positions.every((position) => position >= 0), `${cell.key} record id missing from raw console`);
  for (let index = 1; index < positions.length; index += 1) {
    assert.equal(positions[index], positions[index - 1] + 1, `${cell.key} records must be consecutive`);
  }
  return positions.map((position) => capture.records[position]);
}

function controllerStateFromValues(values) {
  return {
    mode: values.mode,
    selected_category_command: values.selected_category_command,
    selected_topic_id: values.selected_topic_id,
    selected_topic_index: values.selected_topic_index,
    visible_topic_count: values.visible_topic_count,
    visible_topic_ids: values.visible_topic_ids,
    visible_topic_ids_sha256: values.visible_topic_ids_sha256,
    previous_enabled: values.previous_enabled,
    next_enabled: values.next_enabled,
    world_epoch: values.world_epoch,
  };
}

function sameValue(left, right) {
  return JSON.stringify(left) === JSON.stringify(right);
}

function transitionFailure(semanticFailures, cell, field, detail) {
  semanticFailures.push({ cell: cell.key, field, detail });
}

function categoryProjection(cell, command) {
  return cell.categoryCommands.find((entry) => entry.command === command) ?? null;
}

function validateCategoryProjection(cell, value, semanticFailures) {
  const expected = categoryProjection(cell, value.selected_category_command);
  if (!expected
    || value.visible_topic_count !== expected.topic_count
    || value.visible_topic_ids.length !== expected.topic_count
    || value.visible_topic_ids_sha256 !== expected.visible_topic_ids_sha256) {
    transitionFailure(
      semanticFailures,
      cell,
      "category_projection",
      `category ${value.selected_category_command} differs from the accepted ordered projection`,
    );
    return false;
  }
  return true;
}

function expectedIndexAfterSourceKey(step) {
  const before = step.before;
  if (before.mode !== "index") return undefined;
  const rowActions = [
    "SourceKey(Up)", "SourceKey(Down)",
    "SourceKey(PageUp { visible_rows: 8 })", "SourceKey(PageDown { visible_rows: 8 })",
    "SourceKey(Home)", "SourceKey(End)",
  ];
  if (!rowActions.includes(step.action)) return undefined;
  const last = before.visible_topic_count - 1;
  if (last < 0) return null;
  if (before.selected_topic_index === null) {
    if (step.action === "SourceKey(Home)") return 0;
    if (step.action === "SourceKey(End)") return last;
    return null;
  }
  if (step.action === "SourceKey(Up)") return Math.max(0, before.selected_topic_index - 1);
  if (step.action === "SourceKey(Down)") return Math.min(last, before.selected_topic_index + 1);
  if (step.action === "SourceKey(PageUp { visible_rows: 8 })") {
    return Math.max(0, before.selected_topic_index - 8);
  }
  if (step.action === "SourceKey(PageDown { visible_rows: 8 })") {
    return Math.min(last, before.selected_topic_index + 8);
  }
  if (step.action === "SourceKey(Home)") return 0;
  if (step.action === "SourceKey(End)") return last;
  return undefined;
}

function expectedCategoryAfterSourceKey(step) {
  if (step.before.mode !== "index"
    || !["SourceKey(Left)", "SourceKey(Right)"].includes(step.action)) return undefined;
  const index = requiredCategoryCommands.indexOf(step.before.selected_category_command);
  if (index < 0) return null;
  if (step.action === "SourceKey(Left)") {
    return requiredCategoryCommands[Math.max(0, index - 1)];
  }
  return requiredCategoryCommands[index + 1] ?? requiredCategoryCommands[0];
}

function validateStepSemantics(cell, entry, semanticFailures) {
  const { step, inputRecord } = entry;
  if (!navigationOutcomes.has(step.outcome) || typeof step.action !== "string" || step.action.length === 0) {
    transitionFailure(semanticFailures, cell, "transition", "invalid controller step");
    return;
  }
  const unchangedOutcome = [
    "no_change", "rejected", "scroll_requested", "close_requested", "return_forwarded",
  ].includes(step.outcome);
  const stateChanged = !sameValue(step.before, step.after);
  if (step.outcome === "applied" && !stateChanged) {
    transitionFailure(
      semanticFailures,
      cell,
      "transition",
      "applied step did not change controller state",
    );
  } else if (unchangedOutcome && stateChanged) {
    transitionFailure(semanticFailures, cell, "transition", "non-applied step changed controller state");
  }
  if (step.action.startsWith("SourceKey(")) {
    const expectedKind = step.before.mode === "index" ? "index_list" : "topic_body";
    if (!inputRecord || inputRecord.values.focused_control?.owns_focus !== true
      || inputRecord.values.focused_control.kind !== expectedKind) {
      transitionFailure(
        semanticFailures,
        cell,
        "focused_control",
        `keyboard step ${step.action} lacks renderer-owned ${expectedKind} focus on its input record`,
      );
    }
  }
  const expectedIndex = expectedIndexAfterSourceKey(step);
  if (expectedIndex !== undefined) {
    const missingSelectionRejected = step.before.selected_topic_index === null
      && !["SourceKey(Home)", "SourceKey(End)"].includes(step.action);
    const expectedOutcome = missingSelectionRejected
      ? "rejected"
      : expectedIndex === step.before.selected_topic_index ? "no_change" : "applied";
    if (step.before.mode !== "index" || step.after.mode !== "index"
      || step.after.selected_topic_index !== expectedIndex || step.outcome !== expectedOutcome) {
      transitionFailure(semanticFailures, cell, "index_keyboard", `incorrect ${step.action} row transition`);
    }
  }
  const expectedCategory = expectedCategoryAfterSourceKey(step);
  if (expectedCategory !== undefined) {
    const expectedOutcome = expectedCategory === step.before.selected_category_command
      ? "no_change"
      : "applied";
    const projectionValid = expectedCategory !== null
      && validateCategoryProjection(cell, step.after, semanticFailures);
    const retainedTopicIndex = step.before.selected_topic_id === null
      ? -1
      : step.after.visible_topic_ids.indexOf(step.before.selected_topic_id);
    const selectionValid = retainedTopicIndex < 0
      ? step.after.selected_topic_id === null && step.after.selected_topic_index === null
      : step.after.selected_topic_id === step.before.selected_topic_id
        && step.after.selected_topic_index === retainedTopicIndex;
    if (expectedCategory === null || step.after.mode !== "index"
      || step.after.selected_category_command !== expectedCategory
      || step.outcome !== expectedOutcome || !projectionValid || !selectionValid) {
      transitionFailure(
        semanticFailures,
        cell,
        "index_keyboard",
        `incorrect ${step.action} category transition`,
      );
    }
  }
  const selectedTopicMatch = /^SelectTopic\(\"([^\"]+)\"\)$/.exec(step.action);
  if (selectedTopicMatch) {
    const topicId = selectedTopicMatch[1];
    const expectedIndex = step.before.visible_topic_ids.indexOf(topicId);
    if (expectedIndex < 0) {
      if (step.outcome !== "rejected" || stateChanged) {
        transitionFailure(
          semanticFailures,
          cell,
          "selected_topic_id",
          `unavailable stable topic ${topicId} was not rejected unchanged`,
        );
      }
    } else {
      const expectedOutcome = step.before.selected_topic_id === topicId ? "no_change" : "applied";
      if (step.outcome !== expectedOutcome
        || step.after.selected_topic_id !== topicId
        || step.after.selected_topic_index !== expectedIndex) {
        transitionFailure(
          semanticFailures,
          cell,
          "selected_topic_id",
          `stable topic ${topicId} does not match the ordered visible membership`,
        );
      }
    }
  }
  const categoryMatch = /^SelectCategory \{ category_id: (None|Some\(\"([^\"]+)\"\)), force: Normal \}$/.exec(step.action);
  if (categoryMatch) {
    const expectedCommand = categoryMatch[1] === "None"
      ? "0x6f"
      : categoryMatch[2]?.replace("command:", "");
    const projectionValid = validateCategoryProjection(cell, step.after, semanticFailures);
    const retainedTopicIndex = step.before.selected_topic_id === null
      ? -1
      : step.after.visible_topic_ids.indexOf(step.before.selected_topic_id);
    const selectionValid = retainedTopicIndex < 0
      ? step.after.selected_topic_id === null && step.after.selected_topic_index === null
      : step.after.selected_topic_id === step.before.selected_topic_id
        && step.after.selected_topic_index === retainedTopicIndex;
    const expectedOutcome = step.before.selected_category_command === expectedCommand
      ? "no_change"
      : "applied";
    if (!projectionValid
      || expectedCommand === undefined
      || step.after.selected_category_command !== expectedCommand
      || step.outcome !== expectedOutcome
      || !selectionValid) {
      transitionFailure(
        semanticFailures,
        cell,
        "category_projection",
        "category action does not apply the accepted projection and selection reconciliation",
      );
    }
  }
  if (step.action === "PreviousTopic" || step.action === "SourceKey(Left)"
    || step.action === "NextTopic" || step.action === "SourceKey(Right)") {
    if (step.before.mode === "topic") {
      if (step.before.selected_topic_index === null || step.before.visible_topic_count === 0) {
        transitionFailure(
          semanticFailures,
          cell,
          "topic_navigation",
          `${step.action} lacks a selected topic index`,
        );
        return;
      }
      const delta = step.action === "PreviousTopic" || step.action === "SourceKey(Left)" ? -1 : 1;
      const candidate = step.before.selected_topic_index + delta;
      const expected = candidate < 0 || candidate >= step.before.visible_topic_count
        ? step.before.selected_topic_index : candidate;
      const expectedOutcome = expected === step.before.selected_topic_index ? "no_change" : "applied";
      if (step.after.selected_topic_index !== expected || step.outcome !== expectedOutcome) {
        transitionFailure(semanticFailures, cell, "topic_navigation", `incorrect ${step.action} endpoint transition`);
      }
    }
  }
}

function transitionEntries(cell, records, semanticFailures) {
  const byId = new Map(records.map((record) => [record.record_id, record]));
  const entries = [];
  for (let index = 0; index < records.length; index += 1) {
    const record = records[index];
    if (index > 0 && record.transition.from_record_id !== records[index - 1].record_id) {
      transitionFailure(semanticFailures, cell, "transition", "discontinuous record sequence");
    }
    const inputRecord = byId.get(record.transition.from_record_id) ?? null;
    let chained = inputRecord && controllerStateFromValues(inputRecord.values);
    for (const step of record.transition.controller_steps) {
      if (step.input_record_id !== record.transition.from_record_id) {
        transitionFailure(semanticFailures, cell, "transition", "controller input record does not match transition origin");
      }
      if (!chained || !sameValue(step.before, chained)) {
        transitionFailure(semanticFailures, cell, "transition", "controller before-state is not the preceding emitted state");
      }
      const entry = { record, inputRecord, step };
      validateStepSemantics(cell, entry, semanticFailures);
      entries.push(entry);
      chained = step.after;
    }
    if (record.transition.controller_steps.length > 0
      && !sameValue(chained, controllerStateFromValues(record.values))) {
      transitionFailure(semanticFailures, cell, "transition", "controller after-state is not the emitted post-state");
    }
  }
  return entries;
}

function validateTransitions(cell, records, semanticFailures, screenshotRecordIds = []) {
  const entries = transitionEntries(cell, records, semanticFailures);
  const steps = entries.map(({ step }) => step);
  const values = records.map((record) => record.values);
  const first = values[0];
  const last = values.at(-1);
  switch (cell.row.id) {
    case "categories": {
      const commands = values.map((value) => value.selected_category_command)
        .filter((command, index, all) => index === 0 || command !== all[index - 1]);
      const categorySteps = steps.filter((step) => step.action.startsWith("SelectCategory"));
      const clearsAnchor = categorySteps.some((step) =>
        step.before.selected_topic_id === cell.categoryAnchorTopicId
        && step.after.selected_topic_id === null
        && !step.after.visible_topic_ids.includes(cell.categoryAnchorTopicId));
      if (values.some((value) => value.mode !== "index")
        || values.some((value) => value.catalog_generation !== first.catalog_generation)
        || values.some((value) => !validateCategoryProjection(cell, value, semanticFailures))
        || !sameValue(commands, requiredCategoryCommands)
        || steps.length !== categorySteps.length
        || categorySteps.length !== requiredCategoryCommands.length - 1
        || categorySteps.some((step) => step.outcome !== "applied")
        || first.selected_topic_id !== cell.categoryAnchorTopicId
        || !clearsAnchor) {
        transitionFailure(semanticFailures, cell, "category_commands", "exact applied command traversal absent");
      }
      break;
    }
    case "selection": {
      const firstTopicId = first.visible_topic_ids[0];
      const [single, keyboardOpen, returnToIndex, doubleSelect, doubleOpen] = entries;
      const ordered = entries.length === 5 && firstTopicId !== undefined
        && single.step.action === `SelectTopic(\"${firstTopicId}\")`
        && ["applied", "no_change"].includes(single.step.outcome)
        && single.step.before.mode === "index" && single.step.after.mode === "index"
        && single.step.after.selected_topic_id === firstTopicId
        && single.step.after.selected_topic_index === 0
        && keyboardOpen.step.action === "SourceKey(Enter)"
        && keyboardOpen.step.outcome === "applied"
        && keyboardOpen.step.before.mode === "index" && keyboardOpen.step.after.mode === "topic"
        && keyboardOpen.step.before.selected_topic_id === firstTopicId
        && keyboardOpen.step.after.selected_topic_id === firstTopicId
        && returnToIndex.step.action === "SetMode(Index)"
        && returnToIndex.step.outcome === "applied"
        && returnToIndex.step.before.mode === "topic" && returnToIndex.step.after.mode === "index"
        && doubleSelect.record === doubleOpen.record
        && doubleSelect.step.action === `SelectTopic(\"${firstTopicId}\")`
        && doubleSelect.step.outcome === "no_change"
        && doubleOpen.step.action === "SetMode(Topic)"
        && doubleOpen.step.outcome === "applied"
        && doubleOpen.step.after.mode === "topic"
        && doubleOpen.step.after.selected_topic_id === firstTopicId;
      if (!ordered) {
        transitionFailure(semanticFailures, cell, "selection_transition", "single, keyboard-open, return, and double activation trace absent");
      }
      break;
    }
    case "index_keyboard": {
      const expected = [
        "SourceKey(Left)", "SourceKey(Right)",
        null,
        "SourceKey(Right)", null, "SourceKey(Up)", "SourceKey(Down)",
        "SourceKey(PageUp { visible_rows: 8 })", "SourceKey(PageDown { visible_rows: 8 })",
        "SourceKey(Home)", "SourceKey(End)", "SourceKey(Down)",
      ];
      if (values.some((value) => value.mode !== "index")
        || steps.length !== expected.length
        || steps.some((step, index) => {
          if (index === 2) {
            return !/^SelectCategory \{ category_id: Some\(\"[^\"]+\"\), force: Normal \}$/.test(step.action);
          }
          if (index === 4) {
            return step.action !== `SelectTopic(\"${step.before.visible_topic_ids[0]}\")`;
          }
          return step.action !== expected[index];
        })
        || steps.some((step) => step.outcome === "rejected")
        || steps[0]?.before.selected_category_command !== "0x6f"
        || steps[0]?.after.selected_category_command !== "0x6f"
        || steps[1]?.after.selected_category_command !== "0x70"
        || steps[2]?.after.selected_category_command !== "0x75"
        || steps[3]?.after.selected_category_command !== "0x6f"
        || steps[5]?.before.selected_topic_index !== 0 || steps[5]?.after.selected_topic_index !== 0
        || steps[11]?.before.selected_topic_index !== steps[11]?.before.visible_topic_count - 1
        || steps[11]?.after.selected_topic_index !== steps[11]?.before.visible_topic_count - 1) {
        transitionFailure(semanticFailures, cell, "index_keyboard", "exact asymmetric category and row-key state machine absent");
      }
      break;
    }
    case "topic_navigation": {
      const expectedActions = [
        "SourceKey(Left)", "NextTopic", "PreviousTopic",
        "SourceKey(Right)", "SourceKey(Left)", null, "SourceKey(Right)",
      ];
      const lastIndex = first.visible_topic_ids.length - 1;
      const expectedIndexes = [
        [0, 0, "no_change"],
        [0, 1, "applied"],
        [1, 0, "applied"],
        [0, 1, "applied"],
        [1, 0, "applied"],
        [0, lastIndex, "applied"],
        [lastIndex, lastIndex, "no_change"],
      ];
      const previousAttempts = records.filter((record) =>
        record.transition.input_attempts.some((attempt) =>
          attempt.control === "previous_topic" && attempt.enabled === false));
      const nextAttempts = records.filter((record) =>
        record.transition.input_attempts.some((attempt) =>
          attempt.control === "next_topic" && attempt.enabled === false));
      const orderedActions = entries.length === expectedActions.length
        && entries.every(({ step }, index) => {
          const [beforeIndex, afterIndex, outcome] = expectedIndexes[index];
          const expectedAction = expectedActions[index];
          const actionMatches = expectedAction === null
            ? step.action === `SelectTopic(\"${first.visible_topic_ids[lastIndex]}\")`
            : step.action === expectedAction;
          return actionMatches
            && step.before.selected_topic_index === beforeIndex
            && step.after.selected_topic_index === afterIndex
            && step.outcome === outcome;
        });
      if (values.some((value) => value.mode !== "topic"
          || !sameValue(value.visible_topic_ids, first.visible_topic_ids))
        || first.visible_topic_ids.length < 2 || !orderedActions
        || previousAttempts.length !== 1
        || previousAttempts[0].values.selected_topic_index !== 0
        || previousAttempts[0].values.previous_enabled !== false
        || !screenshotRecordIds.includes(previousAttempts[0].record_id)
        || nextAttempts.length !== 1
        || nextAttempts[0].values.selected_topic_index !== lastIndex
        || nextAttempts[0].values.next_enabled !== false
        || !screenshotRecordIds.includes(nextAttempts[0].record_id)
        || steps.some((step) => (step.action === "PreviousTopic"
            && step.before.selected_topic_index === 0)
          || (step.action === "NextTopic"
            && step.before.selected_topic_index === lastIndex))) {
        transitionFailure(semanticFailures, cell, "topic_endpoints", "first/middle/last no-wrap action trace absent");
      }
      break;
    }
    case "scroll_text": {
      const offsets = values.map((value) => value.body_scroll_offset);
      const scrollEntries = entries.filter(({ step }) => step.action.startsWith("SourceKey("));
      const pageDown = scrollEntries.filter(({ step }) => step.action.startsWith("SourceKey(PageDown"));
      const pageUp = scrollEntries.filter(({ step }) => step.action.startsWith("SourceKey(PageUp"));
      const stableBottom = pageDown.length >= 2
        && pageDown.at(-1).record.values.body_scroll_offset === pageDown.at(-2).record.values.body_scroll_offset
        && pageDown.at(-1).record.values.body_scroll_offset > 0;
      const stableTop = pageUp.length >= 2
        && pageUp.at(-1).record.values.body_scroll_offset === 0
        && pageUp.at(-2).record.values.body_scroll_offset === 0;
      const actions = scrollEntries.map(({ step }) => step.action);
      const firstUp = actions.indexOf("SourceKey(Up)");
      const firstDownAfterUp = actions.indexOf("SourceKey(Down)", firstUp + 1);
      const firstPageUp = actions.findIndex((action, index) =>
        index > firstDownAfterUp && action.startsWith("SourceKey(PageUp"));
      const orderedKinds = firstUp >= 2 && firstDownAfterUp === firstUp + 1
        && firstPageUp === firstDownAfterUp + 1
        && actions.slice(0, firstUp).every((action) => action.startsWith("SourceKey(PageDown"))
        && actions.slice(firstPageUp).every((action) => action.startsWith("SourceKey(PageUp"))
        && scrollEntries.every(({ step }) => step.outcome === "scroll_requested");
      if (entries.length !== scrollEntries.length
        || first.body_scroll_offset !== 0 || last.body_scroll_offset !== 0
        || Math.max(...offsets) <= 0 || !stableBottom || !stableTop || !orderedKinds
        || values.some((value) => value.selected_topic_id !== first.selected_topic_id
          || value.body_sha256 !== first.body_sha256)) {
        transitionFailure(semanticFailures, cell, "body_scroll_offset", "ordered bounded scroll endpoint state machine absent");
      }
      break;
    }
    case "language_fallback": {
      const controls = records.flatMap((record) => record.transition.fixture_controls);
      const fallback = values.find((value) => value.requested_language === "1041");
      if (entries.length !== 0
        || !sameValue(controls, ["request_language_1041", "request_language_1033"])
        || first.requested_language !== "1033" || first.effective_language !== "1033"
        || !fallback || fallback.effective_language !== "1033"
        || last.requested_language !== "1033" || last.effective_language !== "1033"
        || first.selected_topic_id === null
        || values.some((value) => value.selected_topic_id !== first.selected_topic_id
          || value.catalog_generation !== first.catalog_generation
          || !sameValue(value.visible_topic_ids, first.visible_topic_ids)
          || value.visible_topic_ids_sha256 !== first.visible_topic_ids_sha256
          || value.title_sha256 !== first.title_sha256 || value.body_sha256 !== first.body_sha256
          || value.asset_id !== first.asset_id || value.asset_digest !== first.asset_digest)) {
        transitionFailure(semanticFailures, cell, "effective_language", "exact whole-record fallback and restoration trace absent");
      }
      break;
    }
    case "world_change": {
      const [initialRecord, removedRecord, rejectedRecord, restoredRecord, reselectedRecord] = records;
      const removed = removedRecord?.values;
      const restored = restoredRecord?.values;
      const reselected = reselectedRecord?.values;
      const removedIds = first.visible_topic_ids.filter((topicId) => topicId !== first.selected_topic_id);
      const rejected = rejectedRecord?.transition.controller_steps ?? [];
      const recovered = reselectedRecord?.transition.controller_steps ?? [];
      if (records.length !== 5
        || values.some((value) => value.world_evidence_kind !== "catalog_scoped_synthetic_admission")
        || !initialRecord || !removed || !restored || !reselected
        || !sameValue(removedRecord.transition.fixture_controls,
          [`remove_selected_admission:${first.selected_topic_id}`])
        || rejected.length !== 1
        || !sameValue(rejectedRecord.transition.fixture_controls,
          [`attempt_removed_topic:${first.selected_topic_id}`])
        || rejected[0].action !== `SelectTopic(\"${first.selected_topic_id}\")`
        || rejected[0].outcome !== "rejected"
        || !sameValue(rejected[0].before, rejected[0].after)
        || !sameValue(restoredRecord.transition.fixture_controls,
          ["restore_admission_snapshot:applied"])
        || recovered.length !== 1
        || recovered[0].action !== `SelectTopic(\"${first.selected_topic_id}\")`
        || recovered[0].outcome !== "applied"
        || removed.world_epoch !== first.world_epoch + 1
        || restored.world_epoch !== removed.world_epoch + 1
        || values.some((value) => value.catalog_generation !== first.catalog_generation)
        || !sameValue(removed.visible_topic_ids, removedIds)
        || !sameValue(restored.visible_topic_ids, first.visible_topic_ids)
        || removed.selected_topic_id !== null || removed.effective_language !== null
        || removed.title_sha256 !== null || removed.body_sha256 !== null
        || removed.asset_id !== null || removed.asset_digest !== null
        || restored.selected_topic_id !== null || restored.asset_id !== null
        || reselected.selected_topic_id !== first.selected_topic_id
        || reselected.asset_id !== first.asset_id || reselected.asset_digest !== first.asset_digest) {
        transitionFailure(semanticFailures, cell, "world_epoch", "exact synthetic remove/reject/restore/reselect trace absent");
      }
      break;
    }
    case "native_precedence": {
      const tupleFor = ({ values: value }) => ({
        source: value.selected_source_kind,
        asset: value.asset_id,
        digest: value.asset_digest,
        profile: value.render_profile,
        mods: value.live_enabled_mods,
      });
      const expectedSelected = (tuple, cacheHit) => ({
        kind: "selected", asset_id: tuple.asset, digest: tuple.digest,
        render_profile: tuple.profile, cache_hit: cacheHit, diagnostic: null,
      });
      const expectedReleased = (tuple) => ({
        kind: "released", asset_id: tuple.asset, digest: tuple.digest,
        render_profile: tuple.profile, cache_hit: null, diagnostic: null,
      });
      let lifecycleValid = records.length > 0;
      let previousTuple = null;
      let previousCacheEvent = null;
      let stableCacheHitSeen = false;
      const stageIndexes = [];
      for (const [recordIndex, record] of records.entries()) {
        const tuple = tupleFor(record);
        const keyChanged = previousTuple === null || !sameValue(tuple, previousTuple);
        if (keyChanged) {
          stageIndexes.push(recordIndex);
          stableCacheHitSeen = false;
          const expectedEvents = [];
          if (previousTuple?.asset !== null && previousTuple !== null) {
            expectedEvents.push(expectedReleased(previousTuple));
          }
          if (tuple.asset !== null) expectedEvents.push(expectedSelected(tuple, false));
          const expectedResult = tuple.asset === null ? "no_art" : "uploaded";
          lifecycleValid = lifecycleValid
            && sameValue(record.transition.texture_events, expectedEvents)
            && record.values.texture_cache_event === expectedResult;
        } else {
          const events = record.transition.texture_events;
          if (events.length === 1
            && sameValue(events[0], expectedSelected(tuple, true))
            && !stableCacheHitSeen) {
            stableCacheHitSeen = true;
            lifecycleValid = lifecycleValid
              && record.values.texture_cache_event === "cache_hit";
          } else if (events.length === 0) {
            lifecycleValid = lifecycleValid
              && record.values.texture_cache_event === previousCacheEvent;
          } else {
            lifecycleValid = false;
          }
        }
        previousTuple = tuple;
        previousCacheEvent = record.values.texture_cache_event;
      }
      const stages = stageIndexes.map((index) => records[index]);
      const sources = stages.map((record) => record.values.selected_source_kind);
      const expected = ["base", "approved_hd", "mod", "null", "base"];
      let valid = cell.target === "native" && lifecycleValid && sameValue(sources, expected);
      if (valid) {
        const tuples = stages.map(tupleFor);
        valid = tuples[0].profile === "original_nearest" && tuples[0].mods.length === 0
          && tuples[1].asset === tuples[0].asset
          && tuples[1].digest !== tuples[0].digest
          && tuples[1].profile === "faithful_hd_linear" && tuples[1].mods.length === 0
          && tuples[2].asset !== tuples[1].asset
          && tuples[2].digest !== tuples[1].digest
          && tuples[2].profile === "original_nearest" && tuples[2].mods.length > 0
          && tuples[3].asset === null && tuples[3].digest === null && tuples[3].profile === null
          && sameValue(tuples[3].mods, tuples[2].mods)
          && sameValue(tuples[4], tuples[0]);
        const profileControls = records.flatMap((record, recordIndex) =>
          record.transition.fixture_controls
            .filter((control) => control.startsWith("profile_"))
            .map((control) => ({ control, recordIndex })));
        const originalControlIndex = profileControls[1]?.recordIndex;
        const originalAtRestoration = originalControlIndex === stageIndexes[4];
        const originalWhileNull = Number.isSafeInteger(originalControlIndex)
          && originalControlIndex >= stageIndexes[3]
          && originalControlIndex < stageIndexes[4]
          && records.slice(originalControlIndex, stageIndexes[4]).every((record) => {
            const tuple = tupleFor(record);
            return tuple.source === "null" && tuple.asset === null
              && tuple.digest === null && tuple.profile === null;
          })
          && records[originalControlIndex].transition.texture_events.length === 0;
        valid = valid && profileControls.length === 2
          && profileControls[0].control === "profile_faithful_hd:applied"
          && profileControls[0].recordIndex === stageIndexes[1]
          && profileControls[1].control === "profile_original:applied"
          && (originalAtRestoration || originalWhileNull);
      }
      if (!valid) {
        transitionFailure(semanticFailures, cell, "selected_source_kind", "exact native precedence and texture lifecycle differs");
      }
      break;
    }
    default: throw new Error(`unhandled matrix row ${cell.row.id}`);
  }
}

function validateAdjudication(adjudication, capture, attachments, label) {
  exactObject(adjudication, [
    "status", "coordinator", "reviewer", "capture_id", "session_id",
    "screenshot_sha256s", "network_sha256", "reviewed_at",
  ], `${label} adjudication`);
  assert.equal(adjudication.status, "pass", `${label} adjudication status`);
  assert.equal(adjudication.coordinator, "IndigoCompass", `${label} coordinator`);
  assert.equal(adjudication.reviewer, "Astra", `${label} reviewer`);
  assert.equal(adjudication.capture_id, capture.capture_id, `${label} adjudication capture`);
  assert.equal(adjudication.session_id, capture.session_id, `${label} adjudication session`);
  assert.deepEqual(
    adjudication.screenshot_sha256s,
    attachments.screenshots.map((entry) => entry.sha256),
    `${label} adjudicated screenshots`,
  );
  assert.equal(
    adjudication.network_sha256,
    attachments.network?.artifact.sha256 ?? null,
    `${label} adjudicated network log`,
  );
  validTimestamp(adjudication.reviewed_at, `${label} review time`);
}

export function gradeSurfaceEvidence(scenario, evidence) {
  const contract = validateSurfaceScenario(scenario);
  const observations = evidence.observations ?? [];
  assert.ok(Array.isArray(observations), "evidence.observations must be an array");
  const observationMap = new Map();
  for (const observation of observations) {
    const key = `${observation.row_id}:${observation.target}:${observation.faction}`;
    assert.ok(!observationMap.has(key), `duplicate evidence observation ${key}`);
    observationMap.set(key, observation);
  }
  const expectedCells = expectedMatrixCells(scenario);
  const missingMatrixCells = [];
  const missingPerCellFields = [];
  const missingPerCellValues = [];
  const missingPerCellArtifacts = [];
  const missingRuntimeRecords = [];
  const semanticFailures = [];
  const blockedLiveRows = [];
  const observedFields = new Set();
  for (const cell of expectedCells) {
    const observation = observationMap.get(cell.key);
    if (!observation || observation.status !== "pass") {
      missingMatrixCells.push(cell.key);
      continue;
    }
    const expectedEvidenceClass = cell.row.evidence_class ?? "live_port_runtime";
    assert.equal(observation.evidence_class, expectedEvidenceClass, `${cell.key} evidence class`);
    if (expectedEvidenceClass !== "live_port_runtime") blockedLiveRows.push(cell.key);
    const fields = uniqueStrings(observation.runtime_fields ?? [], `${cell.key}.runtime_fields`);
    const missing = cell.row.required_fields.filter((field) => !fields.has(field));
    if (missing.length > 0) missingPerCellFields.push({ cell: cell.key, fields: missing });
    if (!observation.capture || !Array.isArray(observation.record_ids)) {
      missingRuntimeRecords.push(cell.key);
    } else {
      const capture = loadRawCapture(scenario, observation.capture, cell, semanticFailures);
      const records = selectedConsecutiveRecords(capture, observation.record_ids, cell);
      const requiredValues = [...new Set(["target", "viewer_faction", ...fields])];
      const missingValues = requiredValues.filter((field) =>
        field !== "navigation_request_count"
        && records.every((record) => !Object.hasOwn(record.values, field)));
      if (missingValues.length > 0) {
        missingPerCellValues.push({ cell: cell.key, fields: missingValues });
      }
      for (const record of records) {
        assert.equal(record.values.target, cell.target, `${cell.key} target value`);
        assert.equal(record.values.viewer_faction, cell.faction, `${cell.key} faction value`);
        for (const field of fields) {
          if (field === "navigation_request_count") continue;
          if (fieldIsSemanticallyValid(field, record.values[field])) observedFields.add(field);
          else semanticFailures.push({ cell: cell.key, field, detail: "invalid typed runtime value" });
        }
        const value = record.values;
        const selectionShape = value.selected_topic_id === null
          ? value.selected_topic_index === null
          : Number.isSafeInteger(value.selected_topic_index)
            && value.selected_topic_index >= 0
            && value.selected_topic_index < value.visible_topic_count;
        if (!selectionShape) {
          semanticFailures.push({ cell: cell.key, field: "selected_topic_index", detail: "selection/index/count mismatch" });
        }
        const noTopicShape = value.selected_topic_id !== null
          || (value.effective_language === null && value.title_sha256 === null
            && value.body_sha256 === null && value.asset_id === null
            && value.asset_digest === null && value.render_profile === null
            && value.selected_source_kind === "unavailable");
        if (!noTopicShape) {
          semanticFailures.push({ cell: cell.key, field: "selected_topic_id", detail: "no-topic observation retained stale localized or image state" });
        }
      }
      let attachments;
      try {
        attachments = validateAttachments(
          observation.artifacts,
          capture.payload,
          observation.record_ids,
          cell.row.id,
          cell.key,
        );
        validateAdjudication(observation.adjudication, capture.payload, attachments, cell.key);
        observedFields.add("navigation_request_count");
      } catch (error) {
        missingPerCellArtifacts.push({ cell: cell.key, detail: error.message });
      }
      validateTransitions(
        cell,
        records,
        semanticFailures,
        attachments?.screenshots.map((entry) => entry.record_id) ?? [],
      );
    }
  }
  const missingRuntimeFields = scenario.required_runtime_fields.filter(
    (field) => !observedFields.has(field),
  );
  const strictRows = evidence.strict_a0?.rows ?? [];
  const acceptedStrictRows = new Set();
  const strictArtifactIdentities = new Set();
  const strictCaptureSessions = new Set();
  const strictFailures = [];
  if (Array.isArray(strictRows)) {
    for (const row of strictRows) {
      try {
        exactObject(row, ["row_id", "source", "outcome", "measurement", "artifacts", "adjudication"], "strict A0 row");
        assert.ok(scenario.strict_a0.row_ids.includes(row.row_id), "strict A0 row id");
        assert.ok(!acceptedStrictRows.has(row.row_id), "strict A0 duplicate row");
        assert.equal(row.source, scenario.strict_a0.required_source, "strict A0 source");
        assert.equal(row.outcome, "pass", "strict A0 outcome");
        exactObject(row.measurement, ["capture_id", "session_id", "platform", "observed_at", "contract_row"], "strict A0 measurement");
        validIdentity(row.measurement.capture_id, "strict A0 capture id");
        validIdentity(row.measurement.session_id, "strict A0 session id");
        validTimestamp(row.measurement.observed_at, "strict A0 observed time");
        assert.equal(row.measurement.platform, "original_windows", "strict A0 platform");
        assert.equal(row.measurement.contract_row, row.row_id, "strict A0 contract row");
        const screenshots = row.artifacts.map((artifact) => {
          exactObject(
            artifact,
            [
              "kind", "path", "sha256", "media_type", "width", "height", "row_id",
              "capture_id", "session_id",
            ],
            `strict A0 ${row.row_id} screenshot`,
          );
          assert.equal(artifact.kind, "screenshot", "strict A0 screenshot kind");
          assert.equal(artifact.media_type, "image/png", "strict A0 screenshot media type");
          assert.equal(artifact.row_id, row.row_id, "strict A0 screenshot row");
          assert.equal(artifact.capture_id, row.measurement.capture_id, "strict A0 screenshot capture");
          assert.equal(artifact.session_id, row.measurement.session_id, "strict A0 screenshot session");
          const bytes = retainedFile(artifact, `strict A0 ${row.row_id}`);
          assert.deepEqual(
            pngDimensions(bytes, `strict A0 ${row.row_id}`),
            { width: artifact.width, height: artifact.height },
            `strict A0 ${row.row_id} screenshot dimensions`,
          );
          return artifact;
        });
        assert.ok(screenshots.length > 0, "strict A0 screenshot required");
        exactObject(row.adjudication, [
          "status", "coordinator", "reviewer", "capture_id", "session_id",
          "screenshot_sha256s", "reviewed_at",
        ], "strict A0 adjudication");
        assert.equal(row.adjudication.status, "pass", "strict A0 adjudication");
        assert.equal(row.adjudication.coordinator, "IndigoCompass", "strict A0 coordinator");
        assert.equal(row.adjudication.reviewer, "Astra", "strict A0 reviewer");
        assert.equal(row.adjudication.capture_id, row.measurement.capture_id, "strict A0 capture");
        assert.equal(row.adjudication.session_id, row.measurement.session_id, "strict A0 session");
        assert.deepEqual(
          row.adjudication.screenshot_sha256s,
          screenshots.map((artifact) => artifact.sha256),
          "strict A0 screenshot adjudication",
        );
        validTimestamp(row.adjudication.reviewed_at, "strict A0 review time");
        assert.ok(
          Date.parse(row.adjudication.reviewed_at) >= Date.parse(row.measurement.observed_at),
          "strict A0 review time precedes observation",
        );
        const rowArtifactIdentities = screenshots.map((artifact) => artifact.sha256);
        assert.equal(
          new Set(rowArtifactIdentities).size,
          rowArtifactIdentities.length,
          "strict A0 duplicate screenshot identity within row",
        );
        for (const identity of rowArtifactIdentities) {
          assert.ok(!strictArtifactIdentities.has(identity), "strict A0 screenshot reused across rows");
        }
        const captureSession = `${row.measurement.capture_id}\0${row.measurement.session_id}`;
        assert.ok(!strictCaptureSessions.has(captureSession), "strict A0 capture/session reused across rows");
        for (const identity of rowArtifactIdentities) strictArtifactIdentities.add(identity);
        strictCaptureSessions.add(captureSession);
        acceptedStrictRows.add(row.row_id);
      } catch (error) {
        strictFailures.push({ row: row?.row_id ?? null, detail: error.message });
      }
    }
  }
  const strictA0 = {
    required_source: scenario.strict_a0.required_source,
    required_row_ids: scenario.strict_a0.row_ids,
    accepted_row_ids: [...acceptedStrictRows].sort(),
    failures: strictFailures,
    status: acceptedStrictRows.size === scenario.strict_a0.row_ids.length
      && strictFailures.length === 0 ? "accepted" : "blocked",
  };
  return {
    status: missingRuntimeFields.length === 0
      && missingMatrixCells.length === 0
      && missingPerCellFields.length === 0
      && missingPerCellValues.length === 0
      && missingPerCellArtifacts.length === 0
      && missingRuntimeRecords.length === 0
      && blockedLiveRows.length === 0
      && semanticFailures.length === 0
      ? "review_ready"
      : "blocked",
    contract,
    missing_runtime_fields: missingRuntimeFields,
    missing_matrix_cells: missingMatrixCells,
    missing_per_cell_fields: missingPerCellFields,
    missing_per_cell_values: missingPerCellValues,
    missing_per_cell_artifacts: missingPerCellArtifacts,
    missing_runtime_records: missingRuntimeRecords,
    semantic_failures: semanticFailures,
    blocked_live_rows: blockedLiveRows,
    strict_a0: strictA0,
  };
}

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

function effectiveImageIds(localized) {
  if (typeof localized.image_id === "string") return [localized.image_id];
  if (localized.image_selector?.kind === "viewer_faction") {
    return [
      localized.image_selector.alliance_image_id,
      localized.image_selector.empire_image_id,
    ];
  }
  return [];
}

const windows1252SpecialBytes = new Map([
  [0x20ac, 0x80], [0x201a, 0x82], [0x0192, 0x83], [0x201e, 0x84],
  [0x2026, 0x85], [0x2020, 0x86], [0x2021, 0x87], [0x02c6, 0x88],
  [0x2030, 0x89], [0x0160, 0x8a], [0x2039, 0x8b], [0x0152, 0x8c],
  [0x017d, 0x8e], [0x2018, 0x91], [0x2019, 0x92], [0x201c, 0x93],
  [0x201d, 0x94], [0x2022, 0x95], [0x2013, 0x96], [0x2014, 0x97],
  [0x02dc, 0x98], [0x2122, 0x99], [0x0161, 0x9a], [0x203a, 0x9b],
  [0x0153, 0x9c], [0x017e, 0x9e], [0x0178, 0x9f],
]);

function windows1252TitleKey(title) {
  const bytes = [];
  for (const scalar of title) {
    const value = scalar.codePointAt(0);
    let byte = value <= 0x7f || (value >= 0xa0 && value <= 0xff)
      ? value
      : windows1252SpecialBytes.get(value);
    assert.notEqual(byte, undefined, "canonical title uses the accepted Windows-1252 sort class");
    if (byte >= 0x41 && byte <= 0x5a) byte += 0x20;
    bytes.push(byte);
  }
  return Buffer.from(bytes);
}

function canonicalProjection(catalog, topicIds) {
  return topicIds.map((topicId, registryIndex) => {
    const localized = catalog.topics[topicId]?.localized?.[catalog.default_language];
    assert.ok(localized, `canonical projection topic ${topicId}`);
    return { topicId, registryIndex, key: windows1252TitleKey(localized.title) };
  }).sort((left, right) => Buffer.compare(left.key, right.key)
      || left.registryIndex - right.registryIndex)
    .map(({ topicId }) => topicId);
}

export function verifyCanonicalInventory(scenario, options) {
  const baselineBytes = fs.readFileSync(options.baselineScenario);
  assert.equal(sha256(baselineBytes), scenario.canonical.base_scenario_sha256, "base scenario identity");
  const catalogBytes = fs.readFileSync(path.join(options.sourceRoot, "encyclopedia/catalog.json"));
  const manifestBytes = fs.readFileSync(path.join(options.sourceRoot, "encyclopedia/manifest.json"));
  assert.equal(sha256(catalogBytes), scenario.canonical.catalog_sha256, "catalog identity");
  assert.equal(sha256(manifestBytes), scenario.canonical.manifest_sha256, "manifest identity");
  const catalog = JSON.parse(catalogBytes);
  const manifest = JSON.parse(manifestBytes);
  assert.equal(manifest.source_profile, scenario.canonical.source_profile, "source profile");
  assert.equal(Object.keys(catalog.topics).length, scenario.canonical.topic_count, "topic count");
  assert.equal(catalog.bindings.length, scenario.canonical.binding_count, "binding count");
  assert.equal(catalog.categories.length, scenario.canonical.category_count, "category count");
  assert.equal(Object.keys(catalog.images).length, scenario.canonical.image_count, "image count");

  const expectedCategories = new Map(scenario.category_commands.map((entry) => [entry.command, entry]));
  assert.equal(catalog.index.command, "0x6f");
  assert.equal(catalog.index.topic_ids.length, expectedCategories.get("0x6f").topic_count);
  assert.equal(
    sha256(Buffer.from(JSON.stringify(canonicalProjection(catalog, catalog.index.topic_ids)))),
    expectedCategories.get("0x6f").visible_topic_ids_sha256,
    "0x6f accepted ordered projection",
  );
  for (const category of catalog.categories) {
    const expected = expectedCategories.get(category.command);
    assert.ok(expected, `unexpected catalog category ${category.command}`);
    assert.equal(category.id, expected.catalog_id, `${category.command} stable category id`);
    assert.equal(category.topic_ids.length, expected.topic_count, `${category.command} topic count`);
    assert.equal(
      sha256(Buffer.from(JSON.stringify(canonicalProjection(catalog, category.topic_ids)))),
      expected.visible_topic_ids_sha256,
      `${category.command} accepted ordered projection`,
    );
  }

  let boundRowsWithEffectiveArt = 0;
  for (const binding of catalog.bindings) {
    const topic = catalog.topics[binding.topic_id];
    assert.ok(topic, `binding topic ${binding.topic_id}`);
    let allRecordsHaveArt = true;
    for (const localized of Object.values(topic.localized)) {
      const imageIds = effectiveImageIds(localized);
      allRecordsHaveArt &&= imageIds.length > 0;
      for (const imageId of imageIds) assert.ok(catalog.images[imageId], `${binding.topic_id} image ${imageId}`);
    }
    if (allRecordsHaveArt) boundRowsWithEffectiveArt += 1;
  }
  assert.equal(
    boundRowsWithEffectiveArt,
    scenario.canonical.bound_rows_with_effective_art,
    "bound rows with effective art",
  );

  for (const probe of scenario.canonical_probes) {
    const matching = catalog.bindings.filter((binding) =>
      binding.topic_id === probe.topic_id
      && binding.family === probe.binding.family
      && binding.dat_id === probe.binding.dat_id
      && binding.variant === probe.binding.variant);
    assert.equal(matching.length, 1, `${probe.topic_id} exact binding`);
    const localized = catalog.topics[probe.topic_id].localized[catalog.default_language];
    assert.equal(sha256(Buffer.from(localized.title)), probe.title_sha256, `${probe.topic_id} title`);
    assert.equal(sha256(Buffer.from(localized.body)), probe.body_sha256, `${probe.topic_id} body`);
    for (const image of [probe.alliance_asset, probe.empire_asset]) {
      assert.equal(catalog.images[image.id].sha256, image.sha256, `${probe.topic_id} ${image.id}`);
      const bytes = fs.readFileSync(path.join(options.sourceRoot, "encyclopedia", catalog.images[image.id].path));
      assert.equal(sha256(bytes), image.sha256, `${probe.topic_id} ${image.id} bytes`);
    }
  }

  const uiManifestBytes = fs.readFileSync(path.join(options.uiRoot, "bmp-manifest.json"));
  assert.equal(sha256(uiManifestBytes), scenario.canonical.ui_manifest_sha256, "UI manifest identity");
  const uiManifest = JSON.parse(uiManifestBytes);
  assert.equal(uiManifest.length, scenario.canonical.ui_resource_count, "UI resource count");
  const uiRoot = path.resolve(options.uiRoot);
  const uiRootPrefix = `${uiRoot}${path.sep}`;
  const uiInventory = createHash("sha256");
  const seenUiResources = new Set();
  let uiResourceBytes = 0;
  for (const entry of uiManifest) {
    assert.match(entry.dll, /^[a-z0-9-]+$/, "UI manifest DLL name");
    assert.ok(Number.isSafeInteger(entry.id) && entry.id >= 0, "UI manifest resource id");
    const relative = `${entry.dll}/BMP/${entry.id}.bmp`;
    assert.ok(!seenUiResources.has(relative), `duplicate UI resource ${relative}`);
    seenUiResources.add(relative);
    const file = path.resolve(uiRoot, relative);
    assert.ok(file.startsWith(uiRootPrefix), `UI resource escapes root: ${relative}`);
    assert.ok(fs.statSync(file).isFile(), `UI resource is missing: ${relative}`);
    const bytes = fs.readFileSync(file);
    const digest = sha256(bytes);
    uiInventory.update(relative);
    uiInventory.update("\0");
    uiInventory.update(String(bytes.length));
    uiInventory.update("\0");
    uiInventory.update(digest);
    uiInventory.update("\n");
    uiResourceBytes += bytes.length;
  }
  const uiInventorySha256 = uiInventory.digest("hex");
  assert.equal(uiResourceBytes, scenario.canonical.ui_resource_bytes, "UI resource bytes");
  assert.equal(uiInventorySha256, scenario.canonical.ui_inventory_sha256, "UI inventory identity");

  return {
    source_profile: manifest.source_profile,
    topics: Object.keys(catalog.topics).length,
    bindings: catalog.bindings.length,
    images: Object.keys(catalog.images).length,
    category_commands: scenario.category_commands.length,
    bound_rows_with_effective_art: boundRowsWithEffectiveArt,
    ui_resources: uiManifest.length,
    ui_resource_bytes: uiResourceBytes,
    ui_manifest_sha256: sha256(uiManifestBytes),
    ui_inventory_sha256: uiInventorySha256,
    catalog_sha256: sha256(catalogBytes),
    manifest_sha256: sha256(manifestBytes),
  };
}

export function verifyRetainedProgress(scenario, options) {
  const browserBytes = fs.readFileSync(options.browserEvidence);
  const reviewBytes = fs.readFileSync(options.coordinatorReview);
  assert.equal(
    sha256(browserBytes),
    scenario.retained_progress.e48_browser_r9_sha256,
    "retained E48 browser evidence identity",
  );
  assert.equal(
    sha256(reviewBytes),
    scenario.retained_progress.e48_review_sha256,
    "retained E48 review identity",
  );
  const browser = JSON.parse(browserBytes);
  assert.equal(browser.status, "pass", "retained E48 browser status");
  assert.equal(browser.source_profile, scenario.canonical.source_profile);
  assert.equal(browser.catalog_sha256, scenario.canonical.catalog_sha256);
  assert.equal(browser.manifest_sha256, scenario.canonical.manifest_sha256);
  assert.equal(browser.runtime_pack_sha256, scenario.canonical.runtime_pack_sha256);
  assert.equal(browser.cases.length, 2, "retained E48 faction cases");
  const factions = new Set();
  let probes = 0;
  let navigationRequests = 0;
  for (const entry of browser.cases) {
    assert.equal(entry.ready.status, "ready", `${entry.faction} retained Ready`);
    assert.equal(entry.ready.faction, entry.faction);
    assert.equal(entry.ready.source_profile, scenario.canonical.source_profile);
    assert.equal(entry.navigation_requests, 0, `${entry.faction} retained offline navigation`);
    assert.deepEqual(entry.browser_diagnostics.fatal, [], `${entry.faction} retained diagnostics`);
    factions.add(entry.faction);
    probes += entry.probes.length;
    navigationRequests += entry.navigation_requests;
  }
  assert.deepEqual([...factions].sort(), ["alliance", "empire"]);
  assert.equal(probes, scenario.canonical_probes.length * 2, "retained E48 probe count");
  return {
    status: "retained_progress_only",
    browser_evidence_sha256: sha256(browserBytes),
    coordinator_review_sha256: sha256(reviewBytes),
    ready_cases: browser.cases.length,
    probes,
    navigation_requests: navigationRequests,
    strict_surface_acceptance: false,
  };
}

export function buildPreflightReport(scenario, inventory, retained, evidence) {
  const grade = gradeSurfaceEvidence(scenario, evidence);
  const functionalBlocked = grade.missing_matrix_cells.length > 0
    || grade.missing_per_cell_fields.length > 0
    || grade.missing_per_cell_values.length > 0
    || grade.missing_per_cell_artifacts.length > 0
    || grade.missing_runtime_records.length > 0
    || grade.blocked_live_rows.length > 0
    || grade.semantic_failures.length > 0
    || grade.missing_runtime_fields.length > 0;
  return {
    schema_version: 2,
    family: scenario.family,
    status: grade.status,
    functional_status: functionalBlocked ? "blocked" : "accepted",
    corpus_applicability: scenario.corpus_applicability,
    inventory,
    retained_progress: retained,
    grade,
    missing_seams: functionalBlocked ? scenario.known_missing_seams : [],
    strict_a0: grade.strict_a0,
    limitations: [
      "Inherited E48 transport probes do not accept E30 surface rows.",
      "E26 is accepted and integrated; its retained native run is qualified E26 evidence until the E30 matrix executes.",
      "Disk mod configuration is not evidence of the running enabled set.",
      "Live browser/computer inspection is coordinator Astra-only.",
    ],
  };
}

export function renderCoordinatorRunbook(scenario, preflight, options) {
  const missing = preflight.missing_seams
    .map((seam) => `- ${seam.id}: ${seam.needed.join(", ")}`)
    .join("\n");
  const matrix = scenario.matrix.map((row) => [
    `### ${row.id}`,
    `Targets: ${row.targets.join(", ")}; factions: ${row.factions.join(", ")}.`,
    `Actions: ${row.actions.join(" → ")}.`,
    `Expected: ${row.expected.join(", ")}.`,
    `Required telemetry: ${row.required_fields.join(", ")}.`,
  ].join("\n")).join("\n\n");
  return `# E30 coordinator-only live acceptance runbook

Worktree: \`${options.worktree}\`

Preflight status: **${preflight.status}**. E26 is accepted and integrated in this worktree. Its feature-only path exposes the running \`live_enabled_mods\`; writing \`mods/config.toml\` alone is still not proof that the in-process \`ModRuntime\` changed.

## Current missing feature seams

${missing || "- none"}

Rows named by a missing seam remain blocked until those structured fields are captured in bound runtime records. Other functional rows may proceed independently. Screenshot inference, an unstructured debug line, Wine, decoding and inherited E48 transport evidence do not fill a blocked seam.

## Immutable inputs

- Catalog SHA-256: \`${scenario.canonical.catalog_sha256}\`
- Manifest SHA-256: \`${scenario.canonical.manifest_sha256}\`
- Runtime pack SHA-256: \`${scenario.canonical.runtime_pack_sha256}\`
- Retained native r6 feature binary SHA-256 (stale after schema-v5 source changes): \`${scenario.runtime_identities.native_feature_binary.sha256}\`
- Retained browser r6 feature WASM SHA-256 (stale after schema-v5 source changes): \`${scenario.runtime_identities.browser_feature_wasm.sha256}\`
- E48 r9 browser evidence SHA-256: \`${scenario.retained_progress.e48_browser_r9_sha256}\`
- Owned original no-art: N/A (${scenario.canonical.bound_rows_with_effective_art}/${scenario.canonical.binding_count} bound rows have effective art).
- Alternate art, including EDATA.192: deferred.

## Reproducible integrated build/check commands

\`\`\`bash
env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/home/will/.local/bin \\
  CARGO_TARGET_DIR=/data/projects/open-rebellion/E48-baseline-target \\
  CARGO_INCREMENTAL=0 \\
  RUSTFLAGS='-L native=${options.worktree}/.artifacts/lib' \\
  rch exec -- cargo build -p rebellion-app --features interface-test-fixtures

env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/home/will/.local/bin \\
  CARGO_TARGET_DIR=/data/projects/open-rebellion/E48-baseline-target \\
  CARGO_INCREMENTAL=0 \\
  rch exec -- cargo build -p rebellion-app --target wasm32-unknown-unknown --features interface-test-fixtures
\`\`\`

The pinned r6 files above remain immutable historical evidence but cannot be launched for current acceptance. After source review, rebuild into a separate r7 directory, update \`runtime_identities\` to the exact reviewed byte lengths and hashes, and only then run the artifact verifier. The E30 site combines that WASM with the accepted E21 runtime pack and E48's already-inspected UI inventory. A release/wasm-opt copy is a distinct identity requiring its own scenario update and live review.

## Native launch (coordinator/Astra only)

After the r7 artifacts are rebuilt and reviewed, reserve an unused private X display, launch it with \`-nolisten tcp\`, record exact Xvfb/app PIDs, then run each faction with that feature binary and the immutable owned base:

\`\`\`bash
env DISPLAY=:<reserved> REBELLION_ENCYCLOPEDIA_INSPECTOR=1 \\
  REBELLION_ENCYCLOPEDIA_VIEWER_FACTION=<alliance|empire> \\
  .artifacts/e30/r7/final/native-bin/open-rebellion \\
  /data/projects/open-rebellion/agent-24-encyclopedia-base-parity-session-14-open-rebellion/.artifacts/e21/owned-data/base
\`\`\`

The native precedence journey must observe the in-process enabled set and exact actual selected bytes through original → approved HD → mod replacement → explicit mod null → restored base. It must not infer the enabled set from disk configuration.

Feature-only surface controls are F2=request missing whole language 1041, F3=restore 1033, F4=synthetic catalog-admission removal, F5=synthetic catalog-admission restoration, F6=original profile, F7=faithful HD, and F8=attempt the stable topic removed by F4 through the shared reducer. F4/F5/F8 are a synthetic supplement only and cannot fill the live replacement-world row. Press F4, retain its schema-v5 observation, then press F8 and retain the typed control plus rejected reducer step before F5 restores membership. Each control must be followed by a new \`[encyclopedia_surface_observation]\` record before the next action.

Retain the raw console lines in order, assigning monotonically increasing line sequence numbers without rewriting the JSON payload. The evidence file references consecutive emitted record IDs; this runner reparses those raw lines through \`surfaceObservationFromConsole\` and rejects caller-authored value summaries. Every controller step names its input record and exact before/after state; every keyboard step must bind to renderer-owned focus on that input record. Record the exact capture/session IDs, fixture code, source profile, pack digest, binary/WASM digest and byte length in the capture envelope.

## Packed browser launch (coordinator/Astra only)

Keep the immutable E48 r7 site as historical transport evidence. First run the accepted packed baseline journey against the separately rebuilt E30 site with the pinned muted browser:

\`\`\`bash
env OPEN_REBELLION_CHROME_FOR_TESTING=/home/will/.cache/ms-playwright/chromium-1234/chrome-linux64/chrome \\
  OPEN_REBELLION_INTERFACE_NODE_MODULES=/data/projects/open-rebellion/agent-27-encyclopedia-packed-fixture-session-14-open-rebellion/tools/interface-parity/node_modules \\
  node tools/interface-parity/encyclopedia-smoke.mjs \\
    --scenario tools/interface-parity/scenarios/encyclopedia-base.json \\
    --source-root /data/projects/open-rebellion/agent-24-encyclopedia-base-parity-session-14-open-rebellion/.artifacts/e21/owned-data/base \\
    --site ${options.worktree}/.artifacts/e30/r7/final/site \\
    --output .artifacts/e30/coordinator-browser/packed-baseline.json
\`\`\`

For the full finite matrix, reserve an unused loopback port and keep a separately owned static server in the foreground so its exit is recorded independently from the browser:

\`\`\`bash
python3 -m http.server <reserved-port> \\
  --bind 127.0.0.1 \\
  --directory ${options.worktree}/.artifacts/e30/r7/final/site
\`\`\`

The coordinator/Astra browser opens \`http://127.0.0.1:<reserved-port>/?fixture-code=${scenario.fixture_codes.packed.alliance}\` and then \`...?fixture-code=${scenario.fixture_codes.packed.empire}\`. Use only the packed fixture codes; the loose codes belong to E48 and are not an E30 surface input. Disable networking after Ready and require zero subsequent requests. Retain typed, fully decodable PNG screenshots carrying the exact row/capture/session/record IDs, the raw console-line capture, a typed request log bound to the same capture/session, exact action outcomes, selected topic/binding/asset IDs and digests, all texture select/release events, process exits and cleanup. The static server must be stopped after both contexts close. The evidence row additionally requires an explicit IndigoCompass/Astra adjudication that names those exact attachment hashes.

The Rust observation intentionally does not assert \`navigation_request_count\`. The coordinator runner must derive that field from the retained browser request log after the offline boundary; a constant fixture value is not evidence.

## Finite action matrix

${matrix}

## Strict A0 boundary

The port findings \`${scenario.strict_a0.known_port_findings.join("\`, \`")}\` remain findings until adjudicated. Source-static and Wine evidence are qualified context only; strict A0 requires independent measured original Windows evidence for the exact rows \`${scenario.strict_a0.row_ids.join("\`, \`")}\`, each with its own fully decodable PNG bound to that row/capture/session and coordinator/Astra adjudication. Screenshot digests and capture/session pairs may not be reused across rows. A row count, source string, or grader source file cannot accept A0.
`;
}

function writeJson(file, value) {
  const temporary = `${file}.tmp-${process.pid}`;
  fs.writeFileSync(temporary, `${JSON.stringify(value, null, 2)}\n`);
  fs.renameSync(temporary, file);
}

export function prepareArtifacts(options) {
  const scenarioBytes = fs.readFileSync(options.scenario);
  const scenario = JSON.parse(scenarioBytes);
  validateSurfaceScenario(scenario);
  const inventory = verifyCanonicalInventory(scenario, options);
  const retained = verifyRetainedProgress(scenario, options);
  const evidence = options.evidence
    ? JSON.parse(fs.readFileSync(options.evidence))
    : {
      schema_version: 2,
      family: scenario.family,
      observations: [],
      strict_a0: { rows: [] },
    };
  const report = buildPreflightReport(scenario, inventory, retained, evidence);
  report.inputs = {
    scenario_sha256: sha256(scenarioBytes),
    baseline_scenario_sha256: sha256(fs.readFileSync(options.baselineScenario)),
    evidence_sha256: options.evidence ? sha256(fs.readFileSync(options.evidence)) : null,
  };
  report.source_files = {
    acceptance_runner_sha256: sha256(fs.readFileSync(fileURLToPath(import.meta.url))),
    shared_smoke_sha256: sha256(fs.readFileSync(path.join(here, "encyclopedia-smoke.mjs"))),
  };

  fs.mkdirSync(options.outputDirectory, { recursive: true });
  const reportPath = path.join(options.outputDirectory, "preflight.json");
  const evidenceTemplatePath = path.join(options.outputDirectory, "evidence-template.json");
  const runbookPath = path.join(options.outputDirectory, "RUNBOOK.md");
  writeJson(reportPath, report);
  writeJson(evidenceTemplatePath, {
    schema_version: 2,
    family: scenario.family,
    note: "Populate only from retained coordinator/Astra live observations. Synthetic self-tests and inherited E48 probes cannot fill E30 matrix cells.",
    observation_contract: {
      required_identity_fields: [
        "row_id",
        "target",
        "faction",
        "status",
        "evidence_class",
        "runtime_fields",
        "capture",
        "record_ids",
        "artifacts",
        "adjudication",
      ],
      evidence_class: "live_port_runtime",
      capture: "Exact retained raw console-line capture; the grader reconstructs schema-v5 records with the shared parser.",
      record_ids: "Consecutive emitted record IDs that bind observed before/after states and actual controller outcomes.",
      artifacts: "Fully decodable PNG screenshots bound to row/capture/session/record and, for browser rows, a typed request log bound to the same capture/session.",
      adjudication: "Explicit IndigoCompass/Astra review of the listed screenshot and network hashes.",
      measured_network_field: "navigation_request_count is supplied by the retained browser request log, never by a constant Rust fixture value.",
    },
    observations: [],
    strict_a0: { rows: [] },
  });
  fs.writeFileSync(runbookPath, renderCoordinatorRunbook(scenario, report, options));
  return {
    report,
    reportPath,
    evidenceTemplatePath,
    runbookPath,
    artifact_sha256: {
      report: sha256(fs.readFileSync(reportPath)),
      evidence_template: sha256(fs.readFileSync(evidenceTemplatePath)),
      runbook: sha256(fs.readFileSync(runbookPath)),
    },
  };
}

function defaultOptions() {
  const root = path.resolve(here, "../..");
  const projectParent = path.resolve(root, "..");
  return {
    baselineScenario: path.join(here, "scenarios/encyclopedia-base.json"),
    browserEvidence: path.join(
      projectParent,
      "agent-29-encyclopedia-loose-acceptance-session-25-session-14-open-rebellion/.artifacts/e48/coordinator-browser-r9/acceptance.json",
    ),
    coordinatorReview: path.join(projectParent, "agent-work/encyclopedia-dispatch/E48-r9-coordinator-review.json"),
    outputDirectory: path.join(root, ".artifacts/e30/prepared"),
    scenario: path.join(here, "scenarios/encyclopedia-surface.json"),
    sourceRoot: path.join(
      projectParent,
      "agent-24-encyclopedia-base-parity-session-14-open-rebellion/.artifacts/e21/owned-data/base",
    ),
    uiRoot: path.join(
      projectParent,
      "agent-29-encyclopedia-loose-acceptance-session-25-session-14-open-rebellion/.artifacts/e48/r7/site-final/data/ui",
    ),
    worktree: root,
  };
}

function parseArguments(argv) {
  const parsed = { prepare: false, selfTest: false };
  const pathArguments = new Map([
    ["--baseline-scenario", "baselineScenario"],
    ["--browser-evidence", "browserEvidence"],
    ["--coordinator-review", "coordinatorReview"],
    ["--evidence", "evidence"],
    ["--output-directory", "outputDirectory"],
    ["--scenario", "scenario"],
    ["--source-root", "sourceRoot"],
    ["--ui-root", "uiRoot"],
  ]);
  for (let index = 0; index < argv.length; index += 1) {
    const value = argv[index];
    if (value === "--self-test") {
      parsed.selfTest = true;
      continue;
    }
    if (value === "--prepare") {
      parsed.prepare = true;
      continue;
    }
    if (value === "--help") {
      parsed.help = true;
      continue;
    }
    const name = pathArguments.get(value);
    if (!name) throw new Error(`unknown argument ${value}`);
    const next = argv[index + 1];
    if (!next) throw new Error(`${value} requires a path`);
    parsed[name] = path.resolve(next);
    index += 1;
  }
  if (!parsed.help) {
    assert.equal(
      Number(parsed.selfTest) + Number(parsed.prepare),
      1,
      "select exactly one of --self-test or --prepare",
    );
  }
  return { ...defaultOptions(), ...parsed };
}

async function runSelfTest() {
  const canonicalScenario = JSON.parse(fs.readFileSync(
    path.join(here, "scenarios/encyclopedia-surface.json"),
    "utf8",
  ));
  const canonicalContract = validateSurfaceScenario(canonicalScenario);
  assert.equal(canonicalContract.category_commands, 7);
  assert.equal(canonicalContract.matrix_rows, 8);
  assert.equal(canonicalContract.matrix_cells, 30);

  const scenario = canonicalScenario;
  assert.equal(validateSurfaceScenario(scenario).matrix_rows, 8);

  assert.throws(
    () => surfaceObservationFromConsole(
      "[encyclopedia_surface_observation] "
        + JSON.stringify({
          schema_version: 1,
          status: "surface_observation",
          record_id: "surface-00000001",
          values: { target: "browser" },
          controller_steps: [],
          fixture_controls: [],
        }),
    ),
    /surface observation (?:exact shape|schema)/,
    "the pre-r4 self-authored observation schema must fail closed",
  );

  const missingCategory = structuredClone(scenario);
  missingCategory.category_commands.pop();
  assert.throws(
    () => validateSurfaceScenario(missingCategory),
    /category command 0x75/,
  );

  const blocked = gradeSurfaceEvidence(scenario, { observations: [], strict_a0: { rows: [] } });
  assert.equal(blocked.status, "blocked");
  assert.equal(blocked.missing_matrix_cells.length, 30);
  assert.equal(blocked.strict_a0.status, "blocked");

  const boundDirectory = fs.mkdtempSync(path.join(os.tmpdir(), "e30-raw-console-"));
  try {
    const conformanceScenario = structuredClone(scenario);
    conformanceScenario.runtime_identities = {
      status: "accepted",
      native_feature_binary: { sha256: "a".repeat(64), byte_length: 1234 },
      browser_feature_wasm: { sha256: "b".repeat(64), byte_length: 5678 },
    };
    const baseTopicIds = ["original:5696", ...Array.from(
      { length: 346 },
      (_, index) => `original:${60_000 + index}`,
    )];
    const categoryIds = new Map([
      ["0x6f", baseTopicIds],
      ["0x70", baseTopicIds.slice(1, 201)],
      ["0x71", baseTopicIds.slice(0, 38)],
      ["0x72", baseTopicIds.slice(20, 34)],
      ["0x73", baseTopicIds.slice(40, 55)],
      ["0x74", baseTopicIds.slice(10, 20)],
      ["0x75", baseTopicIds.slice(30, 99)],
    ]);
    for (const command of conformanceScenario.category_commands) {
      const ids = categoryIds.get(command.command);
      command.visible_topic_ids_sha256 = sha256(Buffer.from(JSON.stringify(ids)));
    }
    const values = (command, focusedId = 42, index = 0, ids = categoryIds.get(command)) => ({
      target: "native",
      viewer_faction: "alliance",
      mode: "index",
      focused_control: {
        kind: "index_list",
        target_id: 42,
        focused_id: focusedId,
        owns_focus: focusedId === 42,
      },
      selected_category_command: command,
      visible_topic_ids: ids,
      visible_topic_ids_sha256: sha256(Buffer.from(JSON.stringify(ids))),
      selected_topic_id: index === null ? null : ids[index],
      selected_topic_index: index,
      visible_topic_count: ids.length,
      previous_enabled: index !== null && index > 0,
      next_enabled: index !== null && index + 1 < ids.length,
      body_scroll_offset: 0,
      world_epoch: 1,
      world_evidence_kind: "catalog_scoped_synthetic_admission",
      catalog_generation: 1,
      requested_language: "1033",
      effective_language: index === null ? null : "1033",
      title_sha256: index === null ? null : "2".repeat(64),
      body_sha256: index === null ? null : "3".repeat(64),
      asset_id: index === null ? null : "edata:34",
      asset_digest: index === null
        ? null
        : "14695e60336dbf2cf76f53525d0efa1100ae0b0c97cb33030c1873dcf5da7d5f",
      render_profile: index === null ? null : "original_nearest",
      selected_source_kind: index === null ? "unavailable" : "base",
      live_enabled_mods: [],
      texture_cache_event: index === null ? "no_art" : "cache_hit",
    });
    const controllerState = (command, index = 0, ids = categoryIds.get(command)) => ({
      mode: "index",
      selected_category_command: command,
      selected_topic_id: index === null ? null : ids[index],
      selected_topic_index: index,
      visible_topic_count: ids.length,
      visible_topic_ids: ids,
      visible_topic_ids_sha256: sha256(Buffer.from(JSON.stringify(ids))),
      previous_enabled: index !== null && index > 0,
      next_enabled: index !== null && index + 1 < ids.length,
      world_epoch: 1,
    });
    const traceState = ({
      mode = "index",
      command = "0x6f",
      index = 0,
      ids = categoryIds.get(command) ?? baseTopicIds,
      epoch = 1,
    } = {}) => ({
      mode,
      selected_category_command: command,
      selected_topic_id: index === null ? null : ids[index],
      selected_topic_index: index,
      visible_topic_count: ids.length,
      visible_topic_ids: ids,
      visible_topic_ids_sha256: sha256(Buffer.from(JSON.stringify(ids))),
      previous_enabled: index !== null && index > 0,
      next_enabled: index !== null && index + 1 < ids.length,
      world_epoch: epoch,
    });
    const traceRecord = (sequence, state, transition = {}) => ({
      schema_version: 5,
      status: "surface_observation",
      record_id: `surface-${String(sequence).padStart(8, "0")}`,
      sequence,
      values: {
        ...values(
          state.selected_category_command,
          42,
          state.selected_topic_index,
          state.visible_topic_ids,
        ),
        ...state,
        focused_control: {
          kind: state.mode === "index" ? "index_list" : "topic_body",
          target_id: state.mode === "index" ? 42 : 43,
          focused_id: state.mode === "index" ? 42 : 43,
          owns_focus: true,
        },
        ...transition.valueOverrides,
      },
      transition: {
        from_record_id: sequence === 1
          ? null
          : `surface-${String(sequence - 1).padStart(8, "0")}`,
        controller_steps: transition.action ? [{
          input_record_id: `surface-${String(sequence - 1).padStart(8, "0")}`,
          action: transition.action,
          outcome: transition.outcome,
          before: transition.before,
          after: state,
        }] : [],
        fixture_controls: transition.fixtureControls ?? [],
        input_attempts: transition.inputAttempts ?? [],
        texture_events: transition.textureEvents ?? [],
      },
    });
    const record = (sequence, command, focusedId = 42, index = 0) => ({
      schema_version: 5,
      status: "surface_observation",
      record_id: `surface-${String(sequence).padStart(8, "0")}`,
      sequence,
      values: values(command, focusedId, index),
      transition: {
        from_record_id: sequence === 1 ? null : `surface-${String(sequence - 1).padStart(8, "0")}`,
        controller_steps: sequence === 1 ? [] : [{
          input_record_id: `surface-${String(sequence - 1).padStart(8, "0")}`,
          action: `SelectCategory { category_id: ${command === "0x6f" ? "None" : `Some(\"command:${command}\")`}, force: Normal }`,
          outcome: "applied",
          before: controllerState(requiredCategoryCommands[sequence - 2], sequence === 2 ? 0 : null),
          after: controllerState(command, index),
        }],
        fixture_controls: [],
        input_attempts: [],
        texture_events: [],
      },
    });
    const records = requiredCategoryCommands.map((command, index) =>
      record(index + 1, command, 42, index === 0 ? 0 : null));
    const capturePayload = {
      schema_version: 1,
      family: conformanceScenario.family,
      capture_id: "synthetic-parser-conformance",
      session_id: "synthetic-session",
      target: "native",
      faction: "alliance",
      fixture_code: conformanceScenario.fixture_codes.packed.alliance,
      source_profile: conformanceScenario.canonical.source_profile,
      runtime_pack_sha256: conformanceScenario.canonical.runtime_pack_sha256,
      build: { kind: "native_feature_binary", sha256: "a".repeat(64), byte_length: 1234 },
      console_lines: records.map((entry, index) => ({
        sequence: index + 1,
        text: `[encyclopedia_surface_observation] ${JSON.stringify(entry)}`,
      })),
    };
    const capturePath = path.join(boundDirectory, "raw-console.json");
    fs.writeFileSync(capturePath, `${JSON.stringify(capturePayload, null, 2)}\n`);
    const truncatedPng = Buffer.alloc(24);
    Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]).copy(truncatedPng, 0);
    Buffer.from("IHDR").copy(truncatedPng, 12);
    truncatedPng.writeUInt32BE(1, 16);
    truncatedPng.writeUInt32BE(1, 20);
    assert.throws(
      () => pngDimensions(truncatedPng, "truncated self-test PNG"),
      /PNG (?:chunk|IEND|decode|data|length)/,
      "an IHDR prefix without complete decodable PNG data must fail closed",
    );
    const png = selfTestPng();
    const idatOffset = 8 + 12 + 13;
    const idatLength = png.readUInt32BE(idatOffset);
    const idatData = png.subarray(idatOffset + 8, idatOffset + 8 + idatLength);
    const split = Math.max(1, Math.floor(idatData.length / 2));
    const nonconsecutiveIdat = Buffer.concat([
      png.subarray(0, idatOffset),
      selfTestPngChunk("IDAT", idatData.subarray(0, split)),
      selfTestPngChunk("tEXt", Buffer.from("fixture\0ordering")),
      selfTestPngChunk("IDAT", idatData.subarray(split)),
      selfTestPngChunk("IEND", Buffer.alloc(0)),
    ]);
    assert.throws(
      () => pngDimensions(nonconsecutiveIdat, "nonconsecutive-IDAT self-test PNG"),
      /PNG IDAT chunks must be consecutive/,
      "critical IDAT ordering must fail closed",
    );
    const unknownCritical = Buffer.concat([
      png.subarray(0, idatOffset),
      selfTestPngChunk("ABCD", Buffer.from([1, 2, 3])),
      png.subarray(idatOffset),
    ]);
    assert.throws(
      () => pngDimensions(unknownCritical, "unknown-critical self-test PNG"),
      /PNG unknown critical chunk ABCD/,
      "unknown critical PNG chunks must fail closed",
    );
    assert.throws(
      () => pngDimensions(png.subarray(0, png.length - 12), "missing-IEND self-test PNG"),
      /PNG IEND/,
      "an otherwise decodable PNG without IEND must fail closed",
    );
    const screenshotPath = path.join(boundDirectory, "frame.png");
    fs.writeFileSync(screenshotPath, png);
    const screenshot = {
      kind: "screenshot",
      path: screenshotPath,
      sha256: sha256(png),
      media_type: "image/png",
      width: 1,
      height: 1,
      row_id: "categories",
      capture_id: capturePayload.capture_id,
      session_id: capturePayload.session_id,
      record_id: records.at(-1).record_id,
    };
    const observation = {
      row_id: "categories",
      target: "native",
      faction: "alliance",
      status: "pass",
      evidence_class: "live_port_runtime",
      runtime_fields: ["mode", "focused_control", "selected_category_command", "visible_topic_ids_sha256"],
      capture: { path: capturePath, sha256: sha256(fs.readFileSync(capturePath)) },
      record_ids: records.map((entry) => entry.record_id),
      artifacts: [screenshot],
      adjudication: {
        status: "pass",
        coordinator: "IndigoCompass",
        reviewer: "Astra",
        capture_id: capturePayload.capture_id,
        session_id: capturePayload.session_id,
        screenshot_sha256s: [screenshot.sha256],
        network_sha256: null,
        reviewed_at: "2026-10-01T00:00:00Z",
      },
    };
    const conformance = gradeSurfaceEvidence(conformanceScenario, {
      observations: [observation],
      strict_a0: { rows: [] },
    });
    assert.equal(conformance.status, "blocked", "synthetic parser conformance is never final live acceptance");
    assert.equal(conformance.semantic_failures.length, 0);

    const wrongBuild = structuredClone(capturePayload);
    wrongBuild.build.sha256 = "c".repeat(64);
    const wrongBuildPath = path.join(boundDirectory, "wrong-build.json");
    fs.writeFileSync(wrongBuildPath, `${JSON.stringify(wrongBuild, null, 2)}\n`);
    const wrongBuildEvidence = structuredClone(observation);
    wrongBuildEvidence.capture = {
      path: wrongBuildPath,
      sha256: sha256(fs.readFileSync(wrongBuildPath)),
    };
    const wrongBuildGrade = gradeSurfaceEvidence(conformanceScenario, {
      observations: [wrongBuildEvidence], strict_a0: { rows: [] },
    });
    assert.ok(wrongBuildGrade.semantic_failures.some(({ field }) => field === "build"));

    for (const [name, mutate, expected] of [
      ["wrong-fixture", (payload) => {
        payload.fixture_code = conformanceScenario.fixture_codes.packed.empire;
      }, /fixture code/],
      ["wrong-source-profile", (payload) => { payload.source_profile = "wrong-profile"; }, /source profile/],
      ["wrong-pack", (payload) => { payload.runtime_pack_sha256 = "d".repeat(64); }, /pack/],
    ]) {
      const wrongIdentity = structuredClone(capturePayload);
      mutate(wrongIdentity);
      const wrongIdentityPath = path.join(boundDirectory, `${name}.json`);
      fs.writeFileSync(wrongIdentityPath, `${JSON.stringify(wrongIdentity, null, 2)}\n`);
      const wrongIdentityEvidence = structuredClone(observation);
      wrongIdentityEvidence.capture = {
        path: wrongIdentityPath,
        sha256: sha256(fs.readFileSync(wrongIdentityPath)),
      };
      assert.throws(
        () => gradeSurfaceEvidence(conformanceScenario, {
          observations: [wrongIdentityEvidence], strict_a0: { rows: [] },
        }),
        expected,
        `${name} must fail before runtime evidence is graded`,
      );
    }

    const unrelatedFocus = structuredClone(capturePayload);
    unrelatedFocus.console_lines = records.map((entry, index) => {
      const changed = {
        ...structuredClone(entry),
        values: values(requiredCategoryCommands[index], 77),
      };
      if (index > 0) changed.transition.controller_steps[0].action = "SourceKey(Right)";
      return {
        sequence: index + 1,
        text: `[encyclopedia_surface_observation] ${JSON.stringify(changed)}`,
      };
    });
    const unrelatedPath = path.join(boundDirectory, "unrelated-focus.json");
    fs.writeFileSync(unrelatedPath, `${JSON.stringify(unrelatedFocus, null, 2)}\n`);
    const unrelatedEvidence = structuredClone(observation);
    unrelatedEvidence.capture = { path: unrelatedPath, sha256: sha256(fs.readFileSync(unrelatedPath)) };
    const unrelatedGrade = gradeSurfaceEvidence(conformanceScenario, {
      observations: [unrelatedEvidence], strict_a0: { rows: [] },
    });
    assert.ok(unrelatedGrade.semantic_failures.some(({ field }) => field === "focused_control"));

    const discontinuous = structuredClone(capturePayload);
    const second = structuredClone(records[1]);
    second.transition.from_record_id = "surface-99999999";
    discontinuous.console_lines[1].text = `[encyclopedia_surface_observation] ${JSON.stringify(second)}`;
    const discontinuousPath = path.join(boundDirectory, "discontinuous.json");
    fs.writeFileSync(discontinuousPath, `${JSON.stringify(discontinuous, null, 2)}\n`);
    const discontinuousEvidence = structuredClone(observation);
    discontinuousEvidence.capture = {
      path: discontinuousPath,
      sha256: sha256(fs.readFileSync(discontinuousPath)),
    };
    assert.throws(
      () => gradeSurfaceEvidence(conformanceScenario, {
        observations: [discontinuousEvidence], strict_a0: { rows: [] },
      }),
      /consecutive transition origin/,
    );

    const fabricatedRecord = structuredClone(observation);
    fabricatedRecord.record_ids[0] = "surface-99999999";
    assert.throws(
      () => gradeSurfaceEvidence(conformanceScenario, {
        observations: [fabricatedRecord], strict_a0: { rows: [] },
      }),
      /record id missing from raw console/,
    );

    const worldAsLive = {
      ...structuredClone(observation),
      row_id: "world_change",
      evidence_class: "live_port_runtime",
    };
    assert.throws(
      () => gradeSurfaceEvidence(conformanceScenario, {
        observations: [worldAsLive], strict_a0: { rows: [] },
      }),
      /world_change:native:alliance evidence class/,
    );

    const precedenceFailures = [];
    const precedenceCell = expectedMatrixCells(conformanceScenario)
      .find(({ key }) => key === "native_precedence:native:alliance");
    const precedenceTuples = [
      { source: "base", asset: "edata:1", digest: "1".repeat(64), profile: "original_nearest", mods: [] },
      { source: "approved_hd", asset: "edata:1", digest: "2".repeat(64), profile: "faithful_hd_linear", mods: [] },
      { source: "mod", asset: "mod:replacement", digest: "3".repeat(64), profile: "original_nearest", mods: ["synthetic@1"] },
      { source: "null", asset: null, digest: null, profile: null, mods: ["synthetic@1"] },
      { source: "base", asset: "edata:1", digest: "1".repeat(64), profile: "original_nearest", mods: [] },
    ];
    const precedenceRecords = precedenceTuples.map((tuple, index) => ({
        ...record(index + 1, "0x6f"),
        values: {
          ...values("0x6f"),
          mode: "topic",
          selected_source_kind: tuple.source,
          live_enabled_mods: tuple.mods,
          asset_id: tuple.asset,
          asset_digest: tuple.digest,
          render_profile: tuple.profile,
        },
      }));
    validateTransitions(precedenceCell, precedenceRecords, precedenceFailures);
    assert.ok(
      precedenceFailures.some(({ detail }) =>
        detail === "exact native precedence and texture lifecycle differs"),
      "omitting superseded texture releases must fail",
    );
    const selectedTexture = (tuple, cacheHit = false) => ({
      kind: "selected",
      asset_id: tuple.asset,
      digest: tuple.digest,
      render_profile: tuple.profile,
      cache_hit: cacheHit,
      diagnostic: null,
    });
    const releasedTexture = (tuple) => ({
      kind: "released",
      asset_id: tuple.asset,
      digest: tuple.digest,
      render_profile: tuple.profile,
      cache_hit: null,
      diagnostic: null,
    });
    const precedenceTrace = precedenceTuples.map((tuple, index) => {
      const events = [];
      if (index > 0 && precedenceTuples[index - 1].asset !== null) {
        events.push(releasedTexture(precedenceTuples[index - 1]));
      }
      if (tuple.asset !== null) events.push(selectedTexture(tuple));
      const controls = index === 1
        ? ["profile_faithful_hd:applied"]
        : index === 4 ? ["profile_original:applied"] : [];
      return traceRecord(index + 1, traceState({ mode: "topic", index: 0 }), {
        fixtureControls: controls,
        textureEvents: events,
        valueOverrides: {
          selected_source_kind: tuple.source,
          live_enabled_mods: tuple.mods,
          asset_id: tuple.asset,
          asset_digest: tuple.digest,
          render_profile: tuple.profile,
          texture_cache_event: tuple.asset === null ? "no_art" : "uploaded",
        },
      });
    });
    const positivePrecedenceFailures = [];
    validateTransitions(precedenceCell, precedenceTrace, positivePrecedenceFailures);
    assert.deepEqual(positivePrecedenceFailures, [], "the exact native precedence trace is valid");
    const expectValidRow = (cell, trace, label) => {
      const failures = [];
      validateTransitions(cell, trace, failures);
      assert.deepEqual(failures, [], `${label} synthetic conformance trace is valid`);
    };
    const renumberTrace = (trace) => trace.map((entry, index) => {
      const next = structuredClone(entry);
      next.sequence = index + 1;
      next.record_id = `surface-${String(index + 1).padStart(8, "0")}`;
      next.transition.from_record_id = index === 0
        ? null
        : `surface-${String(index).padStart(8, "0")}`;
      for (const step of next.transition.controller_steps) {
        step.input_record_id = next.transition.from_record_id;
      }
      return next;
    });
    const stableCacheRecord = traceRecord(2, traceState({ mode: "topic", index: 0 }), {
      textureEvents: [selectedTexture(precedenceTuples[0], true)],
      valueOverrides: {
        selected_source_kind: precedenceTuples[0].source,
        live_enabled_mods: precedenceTuples[0].mods,
        asset_id: precedenceTuples[0].asset,
        asset_digest: precedenceTuples[0].digest,
        render_profile: precedenceTuples[0].profile,
        texture_cache_event: "cache_hit",
      },
    });
    const precedenceWithCacheHit = renumberTrace([
      precedenceTrace[0], stableCacheRecord, ...precedenceTrace.slice(1),
    ]);
    expectValidRow(precedenceCell, precedenceWithCacheHit, "native precedence cache hit");
    const detachedF7 = structuredClone(precedenceWithCacheHit);
    for (const entry of detachedF7) {
      entry.transition.fixture_controls = entry.transition.fixture_controls
        .filter((control) => control !== "profile_faithful_hd:applied");
    }
    detachedF7[1].transition.fixture_controls.push("profile_faithful_hd:applied");
    const detachedF7Failures = [];
    validateTransitions(precedenceCell, detachedF7, detachedF7Failures);
    assert.ok(
      detachedF7Failures.some(({ field }) => field === "selected_source_kind"),
      "F7 must share the exact HD release/select transition record",
    );
    const stableNullOriginal = traceRecord(5, traceState({ mode: "topic", index: 0 }), {
      fixtureControls: ["profile_original:applied"],
      valueOverrides: {
        selected_source_kind: "null",
        live_enabled_mods: precedenceTuples[3].mods,
        asset_id: null,
        asset_digest: null,
        render_profile: null,
        texture_cache_event: "no_art",
      },
    });
    const stableNullProfileTrace = structuredClone(precedenceTrace);
    stableNullProfileTrace[4].transition.fixture_controls = [];
    const stableNullProfile = renumberTrace([
      ...stableNullProfileTrace.slice(0, 4),
      stableNullOriginal,
      stableNullProfileTrace[4],
    ]);
    expectValidRow(
      precedenceCell,
      stableNullProfile,
      "native precedence with F6 while null remains authoritative",
    );
    const duplicateCacheFailures = [];
    validateTransitions(
      precedenceCell,
      renumberTrace([
        precedenceTrace[0], stableCacheRecord, stableCacheRecord, ...precedenceTrace.slice(1),
      ]),
      duplicateCacheFailures,
    );
    assert.ok(
      duplicateCacheFailures.some(({ field }) => field === "selected_source_kind"),
      "duplicate same-key cache-hit events must fail the texture lifecycle automaton",
    );
    const selectionCell = expectedMatrixCells(conformanceScenario)
      .find(({ key }) => key === "selection:native:alliance");
    const selectionIndex = traceState({ index: 0 });
    const selectionTopic = traceState({ mode: "topic", index: 0 });
    const selectionTrace = [
      traceRecord(1, selectionIndex),
      traceRecord(2, selectionIndex, {
        action: `SelectTopic(\"${baseTopicIds[0]}\")`, outcome: "no_change", before: selectionIndex,
      }),
      traceRecord(3, selectionTopic, {
        action: "SourceKey(Enter)", outcome: "applied", before: selectionIndex,
      }),
      traceRecord(4, selectionIndex, {
        action: "SetMode(Index)", outcome: "applied", before: selectionTopic,
      }),
      traceRecord(5, selectionTopic),
    ];
    selectionTrace[4].transition.controller_steps = [{
      input_record_id: selectionTrace[3].record_id,
      action: `SelectTopic(\"${baseTopicIds[0]}\")`,
      outcome: "no_change",
      before: selectionIndex,
      after: selectionIndex,
    }, {
      input_record_id: selectionTrace[3].record_id,
      action: "SetMode(Topic)",
      outcome: "applied",
      before: selectionIndex,
      after: selectionTopic,
    }];
    expectValidRow(selectionCell, selectionTrace, "selection");

    const indexCell = expectedMatrixCells(conformanceScenario)
      .find(({ key }) => key === "index_keyboard:native:alliance");
    const indexStates = [
      traceState({ command: "0x6f", index: 0 }),
      traceState({ command: "0x6f", index: 0 }),
      traceState({ command: "0x70", index: null }),
      traceState({ command: "0x75", index: null }),
      traceState({ command: "0x6f", index: null }),
      traceState({ command: "0x6f", index: 0 }),
      traceState({ command: "0x6f", index: 0 }),
      traceState({ command: "0x6f", index: 1 }),
      traceState({ command: "0x6f", index: 0 }),
      traceState({ command: "0x6f", index: 8 }),
      traceState({ command: "0x6f", index: 0 }),
      traceState({ command: "0x6f", index: baseTopicIds.length - 1 }),
      traceState({ command: "0x6f", index: baseTopicIds.length - 1 }),
    ];
    const indexActions = [
      ["SourceKey(Left)", "no_change"],
      ["SourceKey(Right)", "applied"],
      ["SelectCategory { category_id: Some(\"command:0x75\"), force: Normal }", "applied"],
      ["SourceKey(Right)", "applied"],
      [`SelectTopic(\"${baseTopicIds[0]}\")`, "applied"],
      ["SourceKey(Up)", "no_change"],
      ["SourceKey(Down)", "applied"],
      ["SourceKey(PageUp { visible_rows: 8 })", "applied"],
      ["SourceKey(PageDown { visible_rows: 8 })", "applied"],
      ["SourceKey(Home)", "applied"],
      ["SourceKey(End)", "applied"],
      ["SourceKey(Down)", "no_change"],
    ];
    const indexTrace = [traceRecord(1, indexStates[0])];
    for (const [index, [action, outcome]] of indexActions.entries()) {
      indexTrace.push(traceRecord(index + 2, indexStates[index + 1], {
        action, outcome, before: indexStates[index],
      }));
    }
    expectValidRow(indexCell, indexTrace, "index keyboard");

    const topicCell = expectedMatrixCells(conformanceScenario)
      .find(({ key }) => key === "topic_navigation:native:alliance");
    const topicIndexes = [0, 0, 0, 1, 0, 1, 0, baseTopicIds.length - 1,
      baseTopicIds.length - 1, baseTopicIds.length - 1];
    const topicActions = [
      [null, null], ["SourceKey(Left)", "no_change"],
      ["NextTopic", "applied"], ["PreviousTopic", "applied"],
      ["SourceKey(Right)", "applied"], ["SourceKey(Left)", "applied"],
      [`SelectTopic(\"${baseTopicIds.at(-1)}\")`, "applied"],
      [null, null], ["SourceKey(Right)", "no_change"],
    ];
    const topicStates = topicIndexes.map((index) => traceState({ mode: "topic", index }));
    const topicTrace = [traceRecord(1, topicStates[0])];
    for (const [index, [action, outcome]] of topicActions.entries()) {
      topicTrace.push(traceRecord(index + 2, topicStates[index + 1], {
        action, outcome, before: topicStates[index],
        inputAttempts: index === 0
          ? [{ control: "previous_topic", enabled: false }]
          : index === 7 ? [{ control: "next_topic", enabled: false }] : [],
      }));
    }
    const topicFailures = [];
    validateTransitions(
      topicCell,
      topicTrace,
      topicFailures,
      [topicTrace[1].record_id, topicTrace[8].record_id],
    );
    assert.deepEqual(topicFailures, [], "topic navigation producer-shaped trace is valid");
    const missingDisabledAttempts = structuredClone(topicTrace);
    missingDisabledAttempts[1].transition.input_attempts = [];
    missingDisabledAttempts[8].transition.input_attempts = [];
    const missingDisabledAttemptFailures = [];
    validateTransitions(
      topicCell,
      missingDisabledAttempts,
      missingDisabledAttemptFailures,
      [missingDisabledAttempts[1].record_id, missingDisabledAttempts[8].record_id],
    );
    assert.ok(
      missingDisabledAttemptFailures.some(({ field }) => field === "topic_endpoints"),
      "disabled endpoint evidence must be an inspected input attempt, not a reducer action",
    );

    const scrollCell = expectedMatrixCells(conformanceScenario)
      .find(({ key }) => key === "scroll_text:native:alliance");
    const scrollState = traceState({ mode: "topic", index: 0 });
    const scrollActions = [
      ["SourceKey(PageDown { visible_rows: 8 })", 100],
      ["SourceKey(PageDown { visible_rows: 8 })", 100],
      ["SourceKey(Up)", 80], ["SourceKey(Down)", 100],
      ["SourceKey(PageUp { visible_rows: 8 })", 0],
      ["SourceKey(PageUp { visible_rows: 8 })", 0],
    ];
    const scrollTrace = [traceRecord(1, scrollState, {
      valueOverrides: { body_scroll_offset: 0 },
    })];
    for (const [index, [action, offset]] of scrollActions.entries()) {
      scrollTrace.push(traceRecord(index + 2, scrollState, {
        action, outcome: "scroll_requested", before: scrollState,
        valueOverrides: { body_scroll_offset: offset },
      }));
    }
    expectValidRow(scrollCell, scrollTrace, "topic scroll");
    const scrollWithUnrelatedAction = structuredClone(scrollTrace);
    scrollWithUnrelatedAction[1].transition.controller_steps.push({
      input_record_id: scrollWithUnrelatedAction[0].record_id,
      action: "Close",
      outcome: "close_requested",
      before: scrollState,
      after: scrollState,
    });
    const unrelatedScrollFailures = [];
    validateTransitions(scrollCell, scrollWithUnrelatedAction, unrelatedScrollFailures);
    assert.ok(
      unrelatedScrollFailures.some(({ field }) => field === "body_scroll_offset"),
      "scroll evidence rejects unrelated controller activity",
    );

    const languageCell = expectedMatrixCells(conformanceScenario)
      .find(({ key }) => key === "language_fallback:native:alliance");
    const fallbackState = traceState({ mode: "topic", index: 0 });
    const languageTrace = [
      traceRecord(1, fallbackState),
      traceRecord(2, fallbackState, {
        fixtureControls: ["request_language_1041"],
        valueOverrides: { requested_language: "1041", effective_language: "1033" },
      }),
      traceRecord(3, fallbackState, {
        fixtureControls: ["request_language_1033"],
      }),
    ];
    expectValidRow(languageCell, languageTrace, "whole-record language fallback");
    const changedFallbackMembership = structuredClone(languageTrace);
    changedFallbackMembership[1].values.visible_topic_ids = [
      ...baseTopicIds.slice(0, -1),
      "original:999999",
    ];
    changedFallbackMembership[1].values.visible_topic_ids_sha256 = sha256(Buffer.from(
      JSON.stringify(changedFallbackMembership[1].values.visible_topic_ids),
    ));
    const changedFallbackMembershipFailures = [];
    validateTransitions(languageCell, changedFallbackMembership, changedFallbackMembershipFailures);
    assert.ok(
      changedFallbackMembershipFailures.some(({ field }) => field === "effective_language"),
      "same-generation whole-language fallback must retain the exact ordered membership",
    );
    const languageWithUnrelatedAction = structuredClone(languageTrace);
    languageWithUnrelatedAction[1].transition.controller_steps = [{
      input_record_id: languageWithUnrelatedAction[0].record_id,
      action: "Close",
      outcome: "close_requested",
      before: fallbackState,
      after: fallbackState,
    }];
    const unrelatedLanguageFailures = [];
    validateTransitions(languageCell, languageWithUnrelatedAction, unrelatedLanguageFailures);
    assert.ok(
      unrelatedLanguageFailures.some(({ field }) => field === "effective_language"),
      "whole-language evidence rejects unrelated controller activity",
    );

    const worldCell = expectedMatrixCells(conformanceScenario)
      .find(({ key }) => key === "world_change:native:alliance");
    const worldInitial = traceState({ mode: "topic", index: 0, epoch: 1 });
    const removedIds = baseTopicIds.slice(1);
    const worldRemoved = traceState({ mode: "index", index: null, ids: removedIds, epoch: 2 });
    const worldRestored = traceState({ mode: "index", index: null, epoch: 3 });
    const worldReselected = traceState({ mode: "index", index: 0, epoch: 3 });
    const unavailableValues = {
      effective_language: null, title_sha256: null, body_sha256: null,
      asset_id: null, asset_digest: null, render_profile: null,
      selected_source_kind: "unavailable", texture_cache_event: "no_art",
    };
    const worldTrace = [
      traceRecord(1, worldInitial),
      traceRecord(2, worldRemoved, {
        fixtureControls: [`remove_selected_admission:${baseTopicIds[0]}`],
        valueOverrides: unavailableValues,
      }),
      traceRecord(3, worldRemoved, {
        action: `SelectTopic(\"${baseTopicIds[0]}\")`, outcome: "rejected", before: worldRemoved,
        fixtureControls: [`attempt_removed_topic:${baseTopicIds[0]}`],
        valueOverrides: unavailableValues,
      }),
      traceRecord(4, worldRestored, {
        fixtureControls: ["restore_admission_snapshot:applied"],
        valueOverrides: unavailableValues,
      }),
      traceRecord(5, worldReselected, {
        action: `SelectTopic(\"${baseTopicIds[0]}\")`, outcome: "applied", before: worldRestored,
      }),
    ];
    expectValidRow(worldCell, worldTrace, "synthetic admission replacement");
    const fabricatedStaleTarget = structuredClone(worldTrace);
    fabricatedStaleTarget[2].transition.fixture_controls = [];
    const fabricatedStaleTargetFailures = [];
    validateTransitions(worldCell, fabricatedStaleTarget, fabricatedStaleTargetFailures);
    assert.ok(
      fabricatedStaleTargetFailures.some(({ field }) => field === "world_epoch"),
      "a caller-authored rejected step without the typed F8 input is not reachable evidence",
    );

    const impossibleCategoryMembership = structuredClone(records);
    for (const entry of impossibleCategoryMembership) {
      entry.values.visible_topic_ids = baseTopicIds;
      entry.values.visible_topic_count = baseTopicIds.length;
      entry.values.visible_topic_ids_sha256 = sha256(Buffer.from(JSON.stringify(baseTopicIds)));
      for (const step of entry.transition.controller_steps) {
        for (const state of [step.before, step.after]) {
          state.visible_topic_ids = baseTopicIds;
          state.visible_topic_count = baseTopicIds.length;
          state.visible_topic_ids_sha256 = sha256(Buffer.from(JSON.stringify(baseTopicIds)));
        }
      }
    }
    const impossibleCategoryFailures = [];
    validateTransitions(
      expectedMatrixCells(conformanceScenario)
        .find(({ key }) => key === "categories:native:alliance"),
      impossibleCategoryMembership,
      impossibleCategoryFailures,
    );
    assert.ok(
      impossibleCategoryFailures.some(({ field }) => field === "category_projection"),
      "all-347 membership cannot stand in for every accepted category projection",
    );

    const extraFailureTupleRecords = [
      precedenceTrace[0],
      traceRecord(2, traceState({ mode: "topic", index: 0 }), {
        textureEvents: [{
          kind: "failed",
          asset_id: "edata:999",
          digest: null,
          render_profile: null,
          cache_hit: null,
          diagnostic: "edata:999 failed synthetic upload",
        }],
        valueOverrides: {
          selected_source_kind: precedenceTuples[0].source,
          live_enabled_mods: precedenceTuples[0].mods,
          asset_id: precedenceTuples[0].asset,
          asset_digest: precedenceTuples[0].digest,
          render_profile: precedenceTuples[0].profile,
          texture_cache_event: "failed",
        },
      }),
      ...precedenceTrace.slice(1).map((entry, index) => ({
        ...structuredClone(entry),
        record_id: `surface-${String(index + 3).padStart(8, "0")}`,
        sequence: index + 3,
        transition: {
          ...structuredClone(entry.transition),
          from_record_id: `surface-${String(index + 2).padStart(8, "0")}`,
        },
      })),
    ];
    const extraFailureEvents = [];
    validateTransitions(precedenceCell, extraFailureTupleRecords, extraFailureEvents);
    assert.ok(
      extraFailureEvents.some(({ field }) => field === "selected_source_kind"),
      "same-source failed texture events must remain visible to the lifecycle automaton",
    );

    const selectionFailures = [];
    const arbitrarySelection = [record(1, "0x6f"), record(2, "0x6f")];
    arbitrarySelection[1].transition.controller_steps = [{
      input_record_id: arbitrarySelection[0].record_id,
      action: "Close",
      outcome: "close_requested",
      before: controllerState("0x6f"),
      after: controllerState("0x6f"),
    }];
    validateTransitions(selectionCell, arbitrarySelection, selectionFailures);
    assert.ok(selectionFailures.some(({ field }) => field === "selection_transition"));

    const appliedWithoutChangeFailures = [];
    const appliedWithoutChange = [record(1, "0x6f"), record(2, "0x6f")];
    appliedWithoutChange[1].transition.controller_steps = [{
      input_record_id: appliedWithoutChange[0].record_id,
      action: "SelectCategory { category_id: None, force: Aggregate }",
      outcome: "applied",
      before: controllerState("0x6f"),
      after: controllerState("0x6f"),
    }];
    validateTransitions(selectionCell, appliedWithoutChange, appliedWithoutChangeFailures);
    assert.ok(appliedWithoutChangeFailures.some(({ detail }) =>
      detail === "applied step did not change controller state"));

    const topicScrollFailures = [];
    const topicScroll = [record(1, "0x6f"), record(2, "0x6f")];
    for (const entry of topicScroll) {
      entry.values.mode = "topic";
      entry.values.focused_control.kind = "topic_body";
    }
    topicScroll[1].values.body_scroll_offset = 120;
    const topicState = { ...controllerState("0x6f"), mode: "topic" };
    topicScroll[1].transition.controller_steps = [{
      input_record_id: topicScroll[0].record_id,
      action: "SourceKey(PageDown { visible_rows: 8 })",
      outcome: "scroll_requested",
      before: topicState,
      after: topicState,
    }];
    validateTransitions(scrollCell, topicScroll, topicScrollFailures);
    assert.ok(
      !topicScrollFailures.some(({ field }) => field === "index_keyboard"),
      "topic body scrolling must not be graded as index-row navigation",
    );

    const indexCategoryFailures = [];
    const unchangedCategoryStates = [
      traceState({ index: 9 }),
      traceState({ index: 9 }),
      traceState({ index: 9 }),
      traceState({ index: 8 }),
      traceState({ index: 9 }),
      traceState({ index: 1 }),
      traceState({ index: 9 }),
      traceState({ index: 0 }),
      traceState({ index: baseTopicIds.length - 1 }),
    ];
    const unchangedCategoryActions = [
      ["SourceKey(Left)", "no_change"],
      ["SourceKey(Right)", "no_change"],
      ["SourceKey(Up)", "applied"],
      ["SourceKey(Down)", "applied"],
      ["SourceKey(PageUp { visible_rows: 8 })", "applied"],
      ["SourceKey(PageDown { visible_rows: 8 })", "applied"],
      ["SourceKey(Home)", "applied"],
      ["SourceKey(End)", "applied"],
    ];
    const unchangedCategoryTrace = [traceRecord(1, unchangedCategoryStates[0])];
    for (const [index, [action, outcome]] of unchangedCategoryActions.entries()) {
      unchangedCategoryTrace.push(traceRecord(index + 2, unchangedCategoryStates[index + 1], {
        action,
        outcome,
        before: unchangedCategoryStates[index],
      }));
    }
    validateTransitions(indexCell, unchangedCategoryTrace, indexCategoryFailures);
    assert.ok(
      indexCategoryFailures.some(({ field }) => field === "index_keyboard"),
      "index Left/Right must prove the source-asymmetric category transition",
    );

    const unorderedTopicFailures = [];
    const unorderedTopicStates = [
      traceState({ mode: "topic", index: baseTopicIds.length - 1 }),
      traceState({ mode: "topic", index: baseTopicIds.length - 1 }),
      traceState({ mode: "topic", index: baseTopicIds.length - 2 }),
      traceState({ mode: "topic", index: baseTopicIds.length - 1 }),
      traceState({ mode: "topic", index: 0 }),
      traceState({ mode: "topic", index: 0 }),
    ];
    const unorderedTopicActions = [
      ["NextTopic", "no_change"],
      ["PreviousTopic", "applied"],
      ["NextTopic", "applied"],
      [`SelectTopic(\"${baseTopicIds[0]}\")`, "applied"],
      ["PreviousTopic", "no_change"],
    ];
    const unorderedTopicTrace = [traceRecord(1, unorderedTopicStates[0])];
    for (const [index, [action, outcome]] of unorderedTopicActions.entries()) {
      unorderedTopicTrace.push(traceRecord(index + 2, unorderedTopicStates[index + 1], {
        action,
        outcome,
        before: unorderedTopicStates[index],
      }));
    }
    validateTransitions(topicCell, unorderedTopicTrace, unorderedTopicFailures);
    assert.ok(
      unorderedTopicFailures.some(({ field }) => field === "topic_endpoints"),
      "topic navigation must prove ordered mouse and keyboard first/middle/last transitions",
    );

    const nonFirstSelectionFailures = [];
    const nonFirstSelectionStates = [
      traceState({ index: 0 }),
      traceState({ index: 1 }),
      traceState({ mode: "topic", index: 1 }),
      traceState({ index: 1 }),
      traceState({ mode: "topic", index: 1 }),
    ];
    const nonFirstSelectionTrace = [
      traceRecord(1, nonFirstSelectionStates[0]),
      traceRecord(2, nonFirstSelectionStates[1], {
        action: `SelectTopic(\"${baseTopicIds[1]}\")`,
        outcome: "applied",
        before: nonFirstSelectionStates[0],
      }),
      traceRecord(3, nonFirstSelectionStates[2], {
        action: "SourceKey(Enter)",
        outcome: "applied",
        before: nonFirstSelectionStates[1],
      }),
      traceRecord(4, nonFirstSelectionStates[3], {
        action: "SetMode(Index)",
        outcome: "applied",
        before: nonFirstSelectionStates[2],
      }),
      traceRecord(5, nonFirstSelectionStates[4]),
    ];
    nonFirstSelectionTrace[4].transition.controller_steps = [{
      input_record_id: nonFirstSelectionTrace[3].record_id,
      action: `SelectTopic(\"${baseTopicIds[1]}\")`,
      outcome: "no_change",
      before: nonFirstSelectionStates[3],
      after: nonFirstSelectionStates[3],
    }, {
      input_record_id: nonFirstSelectionTrace[3].record_id,
      action: "SetMode(Topic)",
      outcome: "applied",
      before: nonFirstSelectionStates[3],
      after: nonFirstSelectionStates[4],
    }];
    validateTransitions(selectionCell, nonFirstSelectionTrace, nonFirstSelectionFailures);
    assert.ok(
      nonFirstSelectionFailures.some(({ field }) => field === "selection_transition"),
      "selection evidence must select and open the actual first visible stable ID",
    );

    const languageFailures = [];
    const languageRecords = [record(1, "0x6f"), record(2, "0x6f"), record(3, "0x6f")];
    languageRecords[1].values = {
      ...languageRecords[1].values,
      requested_language: "1041",
      effective_language: "1033",
      title_sha256: "4".repeat(64),
    };
    languageRecords[1].transition = {
      from_record_id: languageRecords[0].record_id,
      controller_steps: [],
      fixture_controls: ["request_language_1041"],
      input_attempts: [],
      texture_events: [],
    };
    languageRecords[2].transition = {
      from_record_id: languageRecords[1].record_id,
      controller_steps: [],
      fixture_controls: ["request_language_1033"],
      input_attempts: [],
      texture_events: [],
    };
    validateTransitions(languageCell, languageRecords, languageFailures);
    assert.ok(languageFailures.some(({ field }) => field === "effective_language"));

    const changedGenerationFailures = [];
    const changedGeneration = [
      traceRecord(1, fallbackState),
      traceRecord(2, fallbackState, {
        fixtureControls: ["request_language_1041"],
        valueOverrides: {
          requested_language: "1041",
          effective_language: "1033",
          catalog_generation: 2,
        },
      }),
      traceRecord(3, fallbackState, {
        fixtureControls: ["request_language_1033"],
      }),
    ];
    validateTransitions(languageCell, changedGeneration, changedGenerationFailures);
    assert.ok(
      changedGenerationFailures.some(({ field }) => field === "effective_language"),
      "whole-record language fallback must retain the exact catalog generation",
    );

    const worldFailures = [];
    const malformedWorld = [record(1, "0x6f"), record(2, "0x6f"), record(3, "0x6f")];
    malformedWorld[1].values = {
      ...malformedWorld[1].values,
      world_epoch: 2,
      visible_topic_ids_sha256: "4".repeat(64),
    };
    malformedWorld[1].transition = {
      from_record_id: malformedWorld[0].record_id,
      controller_steps: [],
      fixture_controls: [`remove_selected_admission:${malformedWorld[0].values.selected_topic_id}`],
      input_attempts: [],
      texture_events: [],
    };
    malformedWorld[2].values = {
      ...malformedWorld[2].values,
      world_epoch: 3,
    };
    malformedWorld[2].transition = {
      from_record_id: malformedWorld[1].record_id,
      controller_steps: [],
      fixture_controls: ["restore_admission_snapshot:applied"],
      input_attempts: [],
      texture_events: [],
    };
    validateTransitions(worldCell, malformedWorld, worldFailures);
    assert.ok(worldFailures.some(({ detail }) =>
      detail === "exact synthetic remove/reject/restore/reselect trace absent"));

    const malformedTexture = structuredClone(records[1]);
    malformedTexture.transition.texture_events = [{
      kind: "released",
      asset_id: "edata:34",
      digest: null,
      render_profile: null,
      cache_hit: true,
      diagnostic: null,
    }];
    assert.throws(
      () => surfaceObservationFromConsole(
        `[encyclopedia_surface_observation] ${JSON.stringify(malformedTexture)}`,
      ),
      /released texture digest/,
      "an omitted texture release identity must fail at the raw parser boundary",
    );

    const sourceFileA0 = gradeSurfaceEvidence(scenario, {
      observations: [],
      strict_a0: {
        rows: [{
          row_id: "ENC-UI-01",
          source: "measured_original_windows",
          outcome: "pass",
          measurement: {
            capture_id: "fabricated",
            session_id: "fabricated",
            platform: "original_windows",
            observed_at: "2026-10-01T00:00:00Z",
            contract_row: "ENC-UI-01",
          },
          artifacts: [{
            kind: "screenshot",
            path: fileURLToPath(import.meta.url),
            sha256: sha256(fs.readFileSync(fileURLToPath(import.meta.url))),
          }],
          adjudication: { status: "pass", coordinator: "IndigoCompass", reviewer: "Astra" },
        }],
      },
    });
    assert.equal(sourceFileA0.strict_a0.status, "blocked");
    assert.ok(sourceFileA0.strict_a0.failures.length > 0, "a source file cannot attest strict A0");

    const strictRows = scenario.strict_a0.row_ids.map((rowId, index) => {
      const captureId = `original-capture-${index + 1}`;
      const sessionId = `original-session-${index + 1}`;
      const bytes = selfTestPng(index + 1);
      const screenshotPath = path.join(boundDirectory, `${rowId}.png`);
      fs.writeFileSync(screenshotPath, bytes);
      const screenshot = {
        kind: "screenshot",
        path: screenshotPath,
        sha256: sha256(bytes),
        media_type: "image/png",
        width: 1,
        height: 1,
        row_id: rowId,
        capture_id: captureId,
        session_id: sessionId,
      };
      return {
        row_id: rowId,
        source: "measured_original_windows",
        outcome: "pass",
        measurement: {
          capture_id: captureId,
          session_id: sessionId,
          platform: "original_windows",
          observed_at: "2026-10-01T00:00:00Z",
          contract_row: rowId,
        },
        artifacts: [screenshot],
        adjudication: {
          status: "pass",
          coordinator: "IndigoCompass",
          reviewer: "Astra",
          capture_id: captureId,
          session_id: sessionId,
          screenshot_sha256s: [screenshot.sha256],
          reviewed_at: "2026-10-01T00:05:00Z",
        },
      };
    });
    const strictConformance = gradeSurfaceEvidence(scenario, {
      observations: [], strict_a0: { rows: strictRows },
    });
    assert.equal(strictConformance.strict_a0.status, "accepted");

    const reviewBeforeObservation = structuredClone(strictRows);
    reviewBeforeObservation[0].measurement.observed_at = "2026-10-01T00:05:00Z";
    reviewBeforeObservation[0].adjudication.reviewed_at = "2026-10-01T00:00:00Z";
    const reviewBeforeObservationGrade = gradeSurfaceEvidence(scenario, {
      observations: [], strict_a0: { rows: reviewBeforeObservation },
    });
    assert.equal(reviewBeforeObservationGrade.strict_a0.status, "blocked");
    assert.ok(reviewBeforeObservationGrade.strict_a0.failures.some(({ detail }) =>
      /review time precedes observation/.test(detail)));

    const reusedArtifact = structuredClone(strictRows);
    reusedArtifact[1].artifacts[0].path = strictRows[0].artifacts[0].path;
    reusedArtifact[1].artifacts[0].sha256 = strictRows[0].artifacts[0].sha256;
    reusedArtifact[1].adjudication.screenshot_sha256s = [strictRows[0].artifacts[0].sha256];
    const reusedArtifactGrade = gradeSurfaceEvidence(scenario, {
      observations: [], strict_a0: { rows: reusedArtifact },
    });
    assert.equal(reusedArtifactGrade.strict_a0.status, "blocked");
    assert.ok(reusedArtifactGrade.strict_a0.failures.some(({ detail }) =>
      /screenshot reused across rows/.test(detail)));

    const reusedCapture = structuredClone(strictRows);
    reusedCapture[1].measurement.capture_id = reusedCapture[0].measurement.capture_id;
    reusedCapture[1].measurement.session_id = reusedCapture[0].measurement.session_id;
    reusedCapture[1].artifacts[0].capture_id = reusedCapture[0].measurement.capture_id;
    reusedCapture[1].artifacts[0].session_id = reusedCapture[0].measurement.session_id;
    reusedCapture[1].adjudication.capture_id = reusedCapture[0].measurement.capture_id;
    reusedCapture[1].adjudication.session_id = reusedCapture[0].measurement.session_id;
    const reusedCaptureGrade = gradeSurfaceEvidence(scenario, {
      observations: [], strict_a0: { rows: reusedCapture },
    });
    assert.equal(reusedCaptureGrade.strict_a0.status, "blocked");
    assert.ok(reusedCaptureGrade.strict_a0.failures.some(({ detail }) =>
      /capture\/session reused across rows/.test(detail)));

    const missingTimestamp = structuredClone(strictRows);
    missingTimestamp[0].measurement.observed_at = null;
    const missingTimestampGrade = gradeSurfaceEvidence(scenario, {
      observations: [], strict_a0: { rows: missingTimestamp },
    });
    assert.equal(missingTimestampGrade.strict_a0.status, "blocked");
    assert.ok(missingTimestampGrade.strict_a0.failures.some(({ detail }) =>
      /observed time type/.test(detail)));
  } finally {
    fs.rmSync(boundDirectory, { recursive: true });
  }

  const hookContext = { faction: "alliance", ready: { status: "ready" } };
  assert.equal(await runOptionalReadyJourney({}, hookContext), null);
  let calls = 0;
  const hookResult = await runOptionalReadyJourney({
    runReadyJourney: async (observed) => {
      calls += 1;
      assert.equal(observed, hookContext);
      return { reset_to_initial_topic: true };
    },
  }, hookContext);
  assert.equal(calls, 1);
  assert.deepEqual(hookResult, { reset_to_initial_topic: true });
  assert.equal(surfaceObservationFromConsole("ordinary browser log"), null);

  const inventory = verifyCanonicalInventory(canonicalScenario, {
    baselineScenario: path.join(here, "scenarios/encyclopedia-base.json"),
    sourceRoot: path.resolve(
      here,
      "../../../agent-24-encyclopedia-base-parity-session-14-open-rebellion/.artifacts/e21/owned-data/base",
    ),
    uiRoot: path.resolve(
      here,
      "../../../agent-29-encyclopedia-loose-acceptance-session-25-session-14-open-rebellion/.artifacts/e48/r7/site-final/data/ui",
    ),
  });
  assert.equal(inventory.bound_rows_with_effective_art, 347);
  assert.equal(inventory.category_commands, 7);
  assert.equal(inventory.ui_resources, 2326);
  assert.equal(inventory.ui_inventory_sha256, canonicalScenario.canonical.ui_inventory_sha256);

  const retained = verifyRetainedProgress(canonicalScenario, {
    browserEvidence: path.resolve(
      here,
      "../../../agent-29-encyclopedia-loose-acceptance-session-25-session-14-open-rebellion/.artifacts/e48/coordinator-browser-r9/acceptance.json",
    ),
    coordinatorReview: path.resolve(
      here,
      "../../../agent-work/encyclopedia-dispatch/E48-r9-coordinator-review.json",
    ),
  });
  assert.equal(retained.ready_cases, 2);
  assert.equal(retained.probes, 8);
  assert.equal(retained.navigation_requests, 0);
  const wrongRetainedIdentity = structuredClone(canonicalScenario);
  wrongRetainedIdentity.retained_progress.e48_browser_r9_sha256 = "0".repeat(64);
  assert.throws(
    () => verifyRetainedProgress(wrongRetainedIdentity, {
      browserEvidence: path.resolve(
        here,
        "../../../agent-29-encyclopedia-loose-acceptance-session-25-session-14-open-rebellion/.artifacts/e48/coordinator-browser-r9/acceptance.json",
      ),
      coordinatorReview: path.resolve(
        here,
        "../../../agent-work/encyclopedia-dispatch/E48-r9-coordinator-review.json",
      ),
    }),
    /retained E48 browser evidence identity/,
  );
  const wrongUiInventory = structuredClone(canonicalScenario);
  wrongUiInventory.canonical.ui_resource_count -= 1;
  assert.throws(
    () => verifyCanonicalInventory(wrongUiInventory, {
      baselineScenario: path.join(here, "scenarios/encyclopedia-base.json"),
      sourceRoot: path.resolve(
        here,
        "../../../agent-24-encyclopedia-base-parity-session-14-open-rebellion/.artifacts/e21/owned-data/base",
      ),
      uiRoot: path.resolve(
        here,
        "../../../agent-29-encyclopedia-loose-acceptance-session-25-session-14-open-rebellion/.artifacts/e48/r7/site-final/data/ui",
      ),
    }),
    /UI resource count/,
  );

  const preflight = buildPreflightReport(canonicalScenario, inventory, retained, {
    observations: [],
    strict_a0: { rows: [] },
  });
  assert.equal(preflight.status, "blocked");
  assert.equal(preflight.grade.missing_matrix_cells.length, 30);
  assert.deepEqual(
    preflight.missing_seams.map((seam) => seam.id),
    ["actual_replacement_world_admission_publication"],
  );
  const runbook = renderCoordinatorRunbook(canonicalScenario, preflight, {
    worktree: path.resolve(here, "../.."),
  });
  assert.doesNotMatch(runbook, /E26 r4 remains unaccepted/);
  assert.match(runbook, /E26 is accepted and integrated/);
  assert.match(runbook, /OPEN_REBELLION_CHROME_FOR_TESTING/);
  assert.match(runbook, /live_enabled_mods/);

  const temporary = fs.mkdtempSync(path.join(os.tmpdir(), "e30-surface-self-test-"));
  try {
    const prepared = prepareArtifacts({
      baselineScenario: path.join(here, "scenarios/encyclopedia-base.json"),
      browserEvidence: path.resolve(
        here,
        "../../../agent-29-encyclopedia-loose-acceptance-session-25-session-14-open-rebellion/.artifacts/e48/coordinator-browser-r9/acceptance.json",
      ),
      coordinatorReview: path.resolve(
        here,
        "../../../agent-work/encyclopedia-dispatch/E48-r9-coordinator-review.json",
      ),
      outputDirectory: temporary,
      scenario: path.join(here, "scenarios/encyclopedia-surface.json"),
      sourceRoot: path.resolve(
        here,
        "../../../agent-24-encyclopedia-base-parity-session-14-open-rebellion/.artifacts/e21/owned-data/base",
      ),
      uiRoot: path.resolve(
        here,
        "../../../agent-29-encyclopedia-loose-acceptance-session-25-session-14-open-rebellion/.artifacts/e48/r7/site-final/data/ui",
      ),
      worktree: path.resolve(here, "../.."),
    });
    assert.equal(prepared.report.status, "blocked");
    assert.ok(fs.statSync(prepared.reportPath).isFile());
    assert.ok(fs.statSync(prepared.evidenceTemplatePath).isFile());
    assert.ok(fs.statSync(prepared.runbookPath).isFile());
    assert.equal(JSON.parse(fs.readFileSync(prepared.reportPath)).status, "blocked");
    const template = JSON.parse(fs.readFileSync(prepared.evidenceTemplatePath));
    assert.deepEqual(
      template.observation_contract.required_identity_fields,
      [
        "row_id",
        "target",
        "faction",
        "status",
        "evidence_class",
        "runtime_fields",
        "capture",
        "record_ids",
        "artifacts",
        "adjudication",
      ],
    );
  } finally {
    fs.rmSync(temporary, { recursive: true });
  }

  console.log("PASS: surface scenario/evidence contract, optional journey hook and canonical inventory");
}

async function main() {
  const options = parseArguments(process.argv.slice(2));
  if (options.help) {
    console.log("usage: encyclopedia-surface-acceptance.mjs --self-test | --prepare [--evidence FILE] [path overrides]");
    return;
  }
  if (options.selfTest) {
    await runSelfTest();
    return;
  }
  const prepared = prepareArtifacts(options);
  console.log(JSON.stringify({
    status: prepared.report.status,
    report: prepared.reportPath,
    evidence_template: prepared.evidenceTemplatePath,
    runbook: prepared.runbookPath,
    artifact_sha256: prepared.artifact_sha256,
  }));
}

if (path.resolve(process.argv[1] || "") === fileURLToPath(import.meta.url)) {
  main().catch((error) => {
    console.error(error.stack || error.message);
    process.exitCode = 1;
  });
}
