#!/usr/bin/env node

import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import fs from "node:fs";
import http from "node:http";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { gzipSync } from "node:zlib";

import { browserExecutable, runBrowser, verifyArtifacts } from "./encyclopedia-smoke.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const e21Root = path.resolve(
  root,
  "../agent-24-encyclopedia-base-parity-session-14-open-rebellion",
);
const defaults = Object.freeze({
  baselineScenario: path.join(here, "scenarios/encyclopedia-base.json"),
  baselineSite: path.join(e21Root, ".artifacts/e21/r2/packed-site"),
  output: path.join(root, ".artifacts/e48/controlled-http/evidence.json"),
  scenario: path.join(here, "scenarios/encyclopedia-loose.json"),
  site: path.join(root, ".artifacts/interface-parity/site"),
  sourceRoot: path.join(e21Root, ".artifacts/e21/owned-data/base"),
  uiRoot: path.join(e21Root, "web/data/ui"),
});
const loosePrefix = "/data/encyclopedia/";
const invalidRuntimePackBytes = Buffer.from("ORPK-invalid-present-pack");

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

function readJson(file) {
  return JSON.parse(fs.readFileSync(file, "utf8"));
}

function parseArguments(argv) {
  const parsed = { browser: false, selfTest: false, serve: false };
  for (let index = 0; index < argv.length; index += 1) {
    const value = argv[index];
    if (value === "--self-test") {
      parsed.selfTest = true;
      continue;
    }
    if (value === "--serve") {
      parsed.serve = true;
      continue;
    }
    if (value === "--browser") {
      parsed.browser = true;
      continue;
    }
    if (value === "--help") {
      parsed.help = true;
      continue;
    }
    if (!["--baseline-scenario", "--baseline-site", "--output", "--scenario", "--site", "--source-root", "--ui-root"].includes(value)) {
      throw new Error(`unknown argument ${value}`);
    }
    const next = argv[index + 1];
    if (!next) throw new Error(`${value} requires a path`);
    parsed[value.slice(2).replace(/-([a-z])/g, (_, letter) => letter.toUpperCase())] = path.resolve(next);
    index += 1;
  }
  if (!parsed.help) {
    assert.equal(
      [parsed.browser, parsed.selfTest, parsed.serve].filter(Boolean).length,
      1,
      "select exactly one of --browser, --self-test or --serve",
    );
  }
  return { ...defaults, ...parsed };
}

function caseMap(scenario) {
  const entries = scenario.cases.map((entry) => [entry.id, entry]);
  assert.equal(new Set(entries.map(([id]) => id)).size, entries.length, "duplicate case id");
  return new Map(entries);
}

function validateScenario(scenario) {
  assert.equal(scenario.schema_version, 1);
  assert.equal(scenario.family, "encyclopedia-loose-acceptance");
  assert.equal(scenario.canonical.owned_no_art_topic_count, 0);
  assert.deepEqual(
    scenario.synthetic_supplements,
    ["explicit_optional_art_absence"],
    "the owned catalog has no legitimate no-art topic",
  );
  assert.equal(scenario.limits.max_active_image_downloads, 4);
  assert.equal(
    scenario.invalid_runtime_pack.byte_length,
    invalidRuntimePackBytes.length,
    "invalid runtime-pack parser contract: byte length",
  );
  assert.equal(
    scenario.invalid_runtime_pack.sha256,
    sha256(invalidRuntimePackBytes),
    "invalid runtime-pack parser contract: byte digest",
  );
  assert.equal(
    scenario.invalid_runtime_pack.decoded_version,
    invalidRuntimePackBytes.readUInt16LE(4),
    "invalid runtime-pack parser contract: decoded version",
  );
  assert.equal(
    scenario.invalid_runtime_pack.parser_error,
    `unsupported runtime-pack version ${scenario.invalid_runtime_pack.decoded_version}`,
    "invalid runtime-pack parser contract: parser error",
  );
  assert.equal(
    scenario.invalid_runtime_pack.expected_diagnostic,
    `loose_fixture_unavailable: Invalid data/runtime.orpk: ${scenario.invalid_runtime_pack.parser_error}`,
    "invalid runtime-pack parser contract: fixture diagnostic",
  );
  assert.equal(
    scenario.invalid_runtime_pack.report_scenario,
    42,
    "invalid runtime-pack parser contract: fixture scenario",
  );
  const cases = caseMap(scenario);
  for (const required of [
    "ready",
    "both_missing",
    "partial_catalog_only",
    "partial_manifest_only",
    "catalog_forbidden",
    "manifest_server_error",
    "catalog_network_reset",
    "catalog_html",
    "image_overflow_absent_length",
    "image_overflow_misleading_length",
    "cancelled_image",
    "bad_present_pack",
  ]) assert.ok(cases.has(required), `missing acceptance case ${required}`);
  for (const entry of scenario.cases.filter((candidate) =>
    !["ready", "cancelled_image", "bad_present_pack"].includes(candidate.id))) {
    assert.equal(typeof entry.diagnostic_contains, "string", `${entry.id} diagnostic_contains`);
    assert.ok(entry.diagnostic_contains.length > 0, `${entry.id} diagnostic_contains is empty`);
  }
  return cases;
}

function canonicalInventory(options, scenario) {
  const catalogBytes = fs.readFileSync(path.join(options.sourceRoot, "encyclopedia/catalog.json"));
  const manifestBytes = fs.readFileSync(path.join(options.sourceRoot, "encyclopedia/manifest.json"));
  assert.equal(sha256(catalogBytes), scenario.canonical.catalog_sha256);
  assert.equal(sha256(manifestBytes), scenario.canonical.manifest_sha256);
  const catalog = JSON.parse(catalogBytes);
  const manifest = JSON.parse(manifestBytes);
  assert.equal(manifest.source_profile, scenario.canonical.source_profile);
  assert.equal(manifest.catalog_sha256, scenario.canonical.catalog_sha256);

  const images = Object.values(catalog.images)
    .map((descriptor) => ({ ...descriptor }))
    .sort((left, right) => left.path.localeCompare(right.path));
  assert.ok(images.length > 4, "canonical acceptance needs more than one image batch");
  const declaredPaths = new Set(["catalog.json", "manifest.json", ...images.map((image) => image.path)]);
  assert.equal(declaredPaths.size, scenario.canonical.encyclopedia_namespace_entries);
  assert.equal(Object.keys(manifest.files).length + 1, declaredPaths.size);
  for (const relative of declaredPaths) {
    const file = path.join(options.sourceRoot, "encyclopedia", relative);
    assert.ok(fs.statSync(file).isFile(), `canonical loose member is missing: ${relative}`);
  }
  assert.equal(
    manifest.binding_sources.length,
    scenario.canonical.binding_source_count,
    "accepted binding-source count",
  );
  for (const source of manifest.binding_sources) {
    const file = confinedFile(options.sourceRoot, source.basename);
    assert.ok(file && fs.statSync(file).isFile(), `selected DAT is missing: ${source.basename}`);
    assert.equal(
      sha256(fs.readFileSync(file)),
      source.sha256,
      `selected DAT digest ${source.basename}`,
    );
  }
  const uiManifestPath = path.join(options.uiRoot, "bmp-manifest.json");
  const uiManifestBytes = fs.readFileSync(uiManifestPath);
  assert.equal(
    sha256(uiManifestBytes),
    scenario.canonical.ui_manifest_sha256,
    "accepted UI manifest identity",
  );
  const uiManifest = JSON.parse(uiManifestBytes);
  assert.equal(
    uiManifest.length,
    scenario.canonical.ui_resource_count,
    "accepted UI resource count",
  );
  const uiResources = new Map();
  const uiInventoryHash = createHash("sha256");
  let uiResourceBytes = 0;
  for (const entry of uiManifest) {
    assert.match(entry.dll, /^[a-z0-9-]+$/, "UI manifest DLL name");
    assert.ok(Number.isSafeInteger(entry.id) && entry.id >= 0, "UI manifest resource id");
    const relative = `${entry.dll}/BMP/${entry.id}.bmp`;
    assert.equal(uiResources.has(relative), false, `duplicate UI resource ${relative}`);
    const file = confinedFile(options.uiRoot, relative);
    assert.ok(file && fs.statSync(file).isFile(), `approved UI resource is missing: ${relative}`);
    const bytes = fs.readFileSync(file);
    const digest = sha256(bytes);
    uiInventoryHash.update(relative);
    uiInventoryHash.update("\0");
    uiInventoryHash.update(String(bytes.length));
    uiInventoryHash.update("\0");
    uiInventoryHash.update(digest);
    uiInventoryHash.update("\n");
    uiResourceBytes += bytes.length;
    uiResources.set(relative, { byteLength: bytes.length, digest, file });
  }
  assert.equal(uiResourceBytes, scenario.canonical.ui_resource_bytes);
  assert.equal(uiInventoryHash.digest("hex"), scenario.canonical.ui_inventory_sha256);
  uiResources.set("bmp-manifest.json", {
    byteLength: uiManifestBytes.length,
    digest: scenario.canonical.ui_manifest_sha256,
    file: uiManifestPath,
  });
  return {
    catalog,
    catalogBytes,
    declaredPaths,
    images,
    manifest,
    manifestBytes,
    overflowPath: images[0].path,
    selectedDatBasenames: manifest.binding_sources.map((source) => source.basename).sort(),
    uiManifest,
    uiResources,
  };
}

function confinedFile(rootPath, relative) {
  const absoluteRoot = path.resolve(rootPath);
  const candidate = path.resolve(absoluteRoot, relative);
  if (candidate !== absoluteRoot && !candidate.startsWith(`${absoluteRoot}${path.sep}`)) return null;
  return candidate;
}

function cookieCase(request) {
  const match = /(?:^|;\s*)e48_case=([a-z0-9_]+)/.exec(request.headers.cookie || "");
  return match?.[1] || null;
}

function responseHeaders(type, length) {
  const headers = { "cache-control": "no-store", "content-type": type };
  if (length !== null) headers["content-length"] = length;
  return headers;
}

function sendBytes(response, status, bytes, type = "application/octet-stream", length = bytes.length) {
  response.writeHead(status, responseHeaders(type, length));
  response.end(bytes);
}

function mimeType(file) {
  if (file.endsWith(".html")) return "text/html; charset=utf-8";
  if (file.endsWith(".js")) return "text/javascript; charset=utf-8";
  if (file.endsWith(".json")) return "application/json";
  if (file.endsWith(".wasm")) return "application/wasm";
  return "application/octet-stream";
}

export async function startControlledLooseServer(options) {
  const scenario = readJson(options.scenario);
  const cases = validateScenario(scenario);
  const inventory = canonicalInventory(options, scenario);
  const observations = {
    active_images: 0,
    cancelled_responses: 0,
    client_aborts: 0,
    max_active_images: 0,
    requests: [],
  };
  let requestSequence = 0;

  const server = http.createServer((request, response) => {
    const url = new URL(request.url, "http://127.0.0.1");
    let caseId = cookieCase(request) || url.searchParams.get("e48-case") || "ready";
    if (!cases.has(caseId)) caseId = "unknown";
    const acceptanceCase = cases.get(caseId);
    const record = {
      case: caseId,
      id: requestSequence,
      method: request.method,
      path: url.pathname,
      status: null,
    };
    requestSequence += 1;
    observations.requests.push(record);
    response.once("finish", () => {
      record.response_finished = true;
    });
    response.once("close", () => {
      record.response_closed = true;
    });

    if (!acceptanceCase) {
      record.status = 400;
      sendBytes(response, 400, Buffer.from("unknown E48 case"), "text/plain; charset=utf-8");
      return;
    }

    if (url.pathname === "/" || url.pathname === "/index.html") {
      const file = path.join(options.site, "index.html");
      record.status = 200;
      response.setHeader("set-cookie", `e48_case=${caseId}; Path=/; SameSite=Strict`);
      sendBytes(response, 200, fs.readFileSync(file), mimeType(file));
      return;
    }

    if (url.pathname === "/data/runtime.orpk") {
      if (acceptanceCase.runtime_pack === "invalid") {
        const bytes = invalidRuntimePackBytes;
        record.status = 200;
        record.response_bytes = bytes.length;
        record.canonical_byte_length = bytes.length;
        record.canonical_sha256 = sha256(bytes);
        sendBytes(response, 200, bytes);
      } else {
        record.status = 404;
        sendBytes(response, 404, Buffer.alloc(0));
      }
      return;
    }

    if (url.pathname.startsWith(loosePrefix)) {
      const relative = decodeURIComponent(url.pathname.slice(loosePrefix.length));
      if (!inventory.declaredPaths.has(relative)) {
        record.status = 404;
        record.unlisted = true;
        sendBytes(response, 404, Buffer.alloc(0));
        return;
      }

      const fault = acceptanceCase.fault;
      const metadataFaults = new Map([
        ["both_metadata_404", new Map([["catalog.json", 404], ["manifest.json", 404]])],
        ["manifest_404", new Map([["manifest.json", 404]])],
        ["catalog_404", new Map([["catalog.json", 404]])],
        ["catalog_403", new Map([["catalog.json", 403]])],
        ["manifest_500", new Map([["manifest.json", 500]])],
      ]);
      const forcedStatus = metadataFaults.get(fault)?.get(relative);
      if (forcedStatus) {
        record.status = forcedStatus;
        sendBytes(response, forcedStatus, Buffer.alloc(0));
        return;
      }
      if (fault === "catalog_reset" && relative === "catalog.json") {
        record.status = "connection_reset";
        request.socket.destroy();
        return;
      }
      if (fault === "catalog_html_200" && relative === "catalog.json") {
        const bytes = Buffer.from("<!doctype html><title>not catalog json</title>");
        record.status = 200;
        sendBytes(response, 200, bytes, "text/html; charset=utf-8");
        return;
      }

      const candidate = confinedFile(path.join(options.sourceRoot, "encyclopedia"), relative);
      assert.ok(candidate, `server rejected canonical relative path ${relative}`);
      const bytes = fs.readFileSync(candidate);
      record.canonical_byte_length = bytes.length;
      record.canonical_sha256 = sha256(bytes);
      const isImage = relative.startsWith("assets/");
      let finished = false;
      const finishImage = (cancelled = false) => {
        if (!isImage || finished) return;
        finished = true;
        observations.active_images -= 1;
        if (cancelled) observations.cancelled_responses += 1;
      };
      if (isImage) {
        observations.active_images += 1;
        observations.max_active_images = Math.max(observations.max_active_images, observations.active_images);
        request.once("aborted", () => finishImage(true));
        response.once("finish", () => finishImage(fault === "image_hold"));
        response.once("close", () => finishImage(fault === "image_hold" || !response.writableFinished));
      }

      if (relative === inventory.overflowPath && fault === "image_overflow_no_length") {
        const overflow = Buffer.concat([bytes, Buffer.from([0])]);
        record.status = 200;
        record.response_bytes = overflow.length;
        response.writeHead(200, responseHeaders("application/octet-stream", null));
        response.write(overflow.subarray(0, bytes.length));
        setTimeout(() => response.end(overflow.subarray(bytes.length)), 5);
        return;
      }
      if (relative === inventory.overflowPath && fault === "image_overflow_small_length") {
        const overflow = Buffer.concat([bytes, Buffer.from([0])]);
        const compressed = gzipSync(overflow);
        record.status = 200;
        record.response_bytes = overflow.length;
        record.wire_bytes = compressed.length;
        response.writeHead(200, {
          ...responseHeaders("application/octet-stream", compressed.length),
          "content-encoding": "gzip",
        });
        response.end(compressed);
        return;
      }
      if (relative === inventory.overflowPath && fault === "image_hold") {
        record.status = 200;
        record.held = true;
        response.writeHead(200, responseHeaders("application/octet-stream", null));
        response.write(bytes.subarray(0, Math.min(32, bytes.length)));
        return;
      }

      record.status = 200;
      record.response_bytes = bytes.length;
      const complete = () => sendBytes(response, 200, bytes, mimeType(candidate));
      if (isImage) setTimeout(complete, 8);
      else complete();
      return;
    }

    if (url.pathname.startsWith("/data/ui/")) {
      const relative = decodeURIComponent(url.pathname.slice("/data/ui/".length));
      const approved = inventory.uiResources.get(relative);
      if (!approved) {
        record.status = 404;
        record.unlisted = true;
        sendBytes(response, 404, Buffer.alloc(0));
        return;
      }
      const bytes = fs.readFileSync(approved.file);
      assert.equal(bytes.length, approved.byteLength, `approved UI length changed: ${relative}`);
      assert.equal(sha256(bytes), approved.digest, `approved UI digest changed: ${relative}`);
      record.status = 200;
      record.canonical_byte_length = approved.byteLength;
      record.canonical_sha256 = approved.digest;
      record.response_bytes = bytes.length;
      sendBytes(response, 200, bytes, mimeType(approved.file));
      return;
    }

    const staticRoots = [
      ["/data/base/", options.sourceRoot],
      ["/", options.site],
    ];
    for (const [prefix, rootPath] of staticRoots) {
      if (!url.pathname.startsWith(prefix)) continue;
      const relative = decodeURIComponent(url.pathname.slice(prefix.length));
      const candidate = confinedFile(rootPath, relative);
      if (candidate && fs.existsSync(candidate) && fs.statSync(candidate).isFile()) {
        const bytes = fs.readFileSync(candidate);
        record.status = 200;
        record.response_bytes = bytes.length;
        sendBytes(response, 200, bytes, mimeType(candidate));
        return;
      }
      break;
    }
    record.status = 404;
    record.unlisted = true;
    sendBytes(response, 404, Buffer.alloc(0));
  });

  await new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(0, "127.0.0.1", resolve);
  });
  let closed = false;
  const origin = `http://127.0.0.1:${server.address().port}`;
  return {
    cases: Object.fromEntries([...cases.keys()].map((id) => [id, `${origin}/?e48-case=${id}`])),
    inventory,
    observations,
    origin,
    scenario,
    async close() {
      if (closed) return;
      closed = true;
      server.closeAllConnections();
      await new Promise((resolve) => server.close(resolve));
    },
  };
}

class CandidateError extends Error {
  constructor(code, detail) {
    super(detail);
    this.code = code;
  }
}

async function boundedFetch(url, maxBytes, options = {}) {
  let response;
  try {
    response = await fetch(url, options);
  } catch (error) {
    throw new CandidateError("transport_error", String(error));
  }
  if (response.status === 404) return { kind: "not_found" };
  if (!response.ok) {
    await response.body?.cancel().catch(() => {});
    throw new CandidateError("transport_error", `HTTP ${response.status}`);
  }
  if (!response.body) throw new CandidateError("transport_error", "streaming body unavailable");
  const reader = response.body.getReader();
  const chunks = [];
  let total = 0;
  try {
    for (;;) {
      const { done, value } = await reader.read();
      if (done) break;
      if (!value.byteLength) continue;
      total += value.byteLength;
      if (total > maxBytes) {
        await reader.cancel();
        throw new CandidateError("resource_limit", `${url} exceeded ${maxBytes} bytes`);
      }
      chunks.push(value);
    }
  } catch (error) {
    await reader.cancel().catch(() => {});
    if (error instanceof CandidateError) throw error;
    throw new CandidateError("transport_error", String(error));
  }
  return { bytes: Buffer.concat(chunks.map((chunk) => Buffer.from(chunk)), total), kind: "ready" };
}

async function fetchCandidate(controlled, caseId) {
  const { limits } = controlled.scenario;
  const headers = { "x-e48-case": caseId, cookie: `e48_case=${caseId}` };
  const [manifestResult, catalogResult] = await Promise.all([
    boundedFetch(`${controlled.origin}${loosePrefix}manifest.json`, limits.manifest_bytes, { headers }),
    boundedFetch(`${controlled.origin}${loosePrefix}catalog.json`, limits.catalog_bytes, { headers }),
  ]);
  if (manifestResult.kind === "not_found" && catalogResult.kind === "not_found") {
    return { kind: "unavailable" };
  }
  if (manifestResult.kind !== "ready" || catalogResult.kind !== "ready") {
    throw new CandidateError("integrity_error", "partial metadata namespace");
  }
  let manifest;
  let catalog;
  try {
    manifest = JSON.parse(manifestResult.bytes);
    catalog = JSON.parse(catalogResult.bytes);
  } catch (error) {
    throw new CandidateError("integrity_error", String(error));
  }
  if (sha256(catalogResult.bytes) !== manifest.catalog_sha256) {
    throw new CandidateError("integrity_error", "catalog digest mismatch");
  }
  const images = Object.values(catalog.images).sort((left, right) => left.path.localeCompare(right.path));
  let aggregateBytes = 0;
  let imageCount = 0;
  for (let offset = 0; offset < images.length; offset += limits.max_active_image_downloads) {
    const batch = images.slice(offset, offset + limits.max_active_image_downloads);
    const results = await Promise.all(batch.map((image) => boundedFetch(
      `${controlled.origin}${loosePrefix}${image.path}`,
      image.byte_length,
      { headers },
    )));
    for (let index = 0; index < batch.length; index += 1) {
      const image = batch[index];
      const result = results[index];
      if (result.kind !== "ready") throw new CandidateError("integrity_error", `missing ${image.path}`);
      if (result.bytes.length !== image.byte_length || sha256(result.bytes) !== image.sha256) {
        throw new CandidateError("integrity_error", `invalid ${image.path}`);
      }
      aggregateBytes += result.bytes.length;
      if (aggregateBytes > limits.aggregate_image_bytes) {
        throw new CandidateError("resource_limit", "aggregate image bytes exceeded");
      }
      imageCount += 1;
    }
  }
  return {
    catalog_sha256: sha256(catalogResult.bytes),
    image_count: imageCount,
    kind: "ready",
    manifest_sha256: sha256(manifestResult.bytes),
    source_profile: manifest.source_profile,
  };
}

async function expectCandidateFailure(controlled, caseId, expectedCode) {
  await assert.rejects(
    fetchCandidate(controlled, caseId),
    (error) => error instanceof CandidateError && error.code === expectedCode,
    `${caseId} should fail as ${expectedCode}`,
  );
}

async function waitUntil(predicate, detail, timeoutMs = 1_000) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() <= deadline) {
    if (predicate()) return;
    await new Promise((resolve) => setTimeout(resolve, 5));
  }
  throw new Error(`timed out waiting for ${detail}`);
}

function requireInvalidPackDiagnostic(diagnostics, report, scenario) {
  const diagnostic = report?.diagnostic;
  const nullFields = [
    "source_profile",
    "topic_id",
    "title",
    "title_sha256",
    "body_sha256",
    "body_chars",
    "binding_family",
    "binding_dat_id",
    "binding_variant",
    "asset_id",
    "digest",
    "navigation_requests",
  ];
  if (
    report?.schema_version !== 1
    || report?.status !== "unavailable"
    || report.code !== scenario.fixture_codes.alliance
    || report.scenario !== scenario.invalid_runtime_pack.report_scenario
    || report.faction !== "alliance"
    || report.surface !== "loose-encyclopedia-fixture"
    || diagnostic !== scenario.invalid_runtime_pack.expected_diagnostic
    || report.cache_status !== "not selected"
    || nullFields.some((field) => report[field] !== null)
  ) {
    throw new Error(
      `expected invalid runtime-pack parse failure, observed report=${JSON.stringify(report)} diagnostics=${JSON.stringify(diagnostics)}`,
    );
  }
  if (diagnostics.length > 0) {
    throw new Error(`unexpected diagnostics during invalid-pack rejection: ${JSON.stringify(diagnostics)}`);
  }
  return diagnostic;
}

function requireInvalidPackTransport(browserRequests, observations, scenario) {
  const expectedRequests = [
    "/",
    "/gl.js",
    "/open-rebellion-test.wasm",
    "/data/runtime.orpk",
  ];
  assert.deepEqual(
    browserRequests,
    expectedRequests,
    "invalid runtime-pack transport: browser requests",
  );
  assert.deepEqual(
    observations.map((entry) => entry.path),
    expectedRequests,
    "invalid runtime-pack transport: server requests",
  );
  assert.equal(
    observations.every((entry) => entry.status === 200),
    true,
    "invalid runtime-pack transport: non-success response",
  );
  assert.equal(
    observations.some((entry) => entry.path.startsWith(loosePrefix)),
    false,
    "invalid runtime-pack transport: loose fallback request",
  );
  const packResponses = observations.filter((entry) => entry.path === "/data/runtime.orpk");
  assert.equal(
    packResponses.length,
    1,
    "invalid runtime-pack transport: expected exactly one pack response",
  );
  const [pack] = packResponses;
  assert.equal(pack.response_finished, true, "invalid runtime-pack transport: unfinished pack response");
  assert.equal(pack.response_closed, true, "invalid runtime-pack transport: unclosed pack response");
  assert.equal(
    pack.response_bytes,
    scenario.invalid_runtime_pack.byte_length,
    "invalid runtime-pack transport: pack response length",
  );
  assert.equal(
    pack.canonical_byte_length,
    scenario.invalid_runtime_pack.byte_length,
    "invalid runtime-pack transport: canonical pack length",
  );
  assert.equal(
    pack.canonical_sha256,
    scenario.invalid_runtime_pack.sha256,
    "invalid runtime-pack transport: canonical pack digest",
  );
  return pack;
}

function persistBrowserFailure(output, failure) {
  const error = failure.error instanceof Error
    ? failure.error.message
    : String(failure.error);
  const payload = {
    schema_version: 1,
    status: "failed",
    failure: { ...failure, error },
  };
  fs.mkdirSync(path.dirname(output), { recursive: true });
  fs.writeFileSync(output, `${JSON.stringify(payload, null, 2)}\n`);
  return payload;
}

function snapshotJson(value) {
  return JSON.parse(JSON.stringify(value));
}

function createCompletedReadyLedger() {
  const completedReadyCases = [];
  return {
    failure(failure) {
      const { error, ...evidence } = failure;
      return {
        ...snapshotJson(evidence),
        error,
        completed_ready_cases: snapshotJson(completedReadyCases),
      };
    },
    record(completedReadyCase) {
      completedReadyCases.push(snapshotJson(completedReadyCase));
    },
  };
}

function finalizeBrowserFailure(failure, allObservations) {
  const { error, ...evidence } = failure;
  return {
    ...snapshotJson(evidence),
    all_observations: snapshotJson(allObservations),
    cleanup: "closed",
    error,
  };
}

function classifyLooseBrowserDiagnostics({ diagnostics, inventory, observations, ready, scenario }) {
  const expected = [];
  const fatal = [];
  const absentPackDiagnostic =
    "console:Failed to load resource: the server responded with a status of 404 (Not Found)";
  const absentPackDiagnostics = diagnostics.filter((entry) => entry === absentPackDiagnostic);
  const absentPackResponses = observations.filter((entry) =>
    entry.path === "/data/runtime.orpk"
    && entry.status === 404
    && entry.response_finished === true);
  const readyMatches = ready?.status === "ready"
    && ready.source_profile === scenario.canonical.source_profile;
  const imagesByPath = new Map(inventory.images.map((image) => [
    `${loosePrefix}${image.path}`,
    image,
  ]));
  const metadataByPath = new Map([
    [`${loosePrefix}manifest.json`, {
      byte_length: inventory.manifestBytes.length,
      kind: "manifest",
      sha256: scenario.canonical.manifest_sha256,
    }],
    [`${loosePrefix}catalog.json`, {
      byte_length: inventory.catalogBytes.length,
      kind: "catalog",
      sha256: scenario.canonical.catalog_sha256,
    }],
  ]);

  for (const diagnostic of diagnostics) {
    if (
      diagnostic === absentPackDiagnostic
      && absentPackDiagnostics.length === 1
      && absentPackResponses.length === 1
    ) {
      expected.push({
        classification: "expected_absent_runtime_pack",
        diagnostic,
        path: "/data/runtime.orpk",
        status: 404,
      });
      continue;
    }

    const prefix = "request:";
    const suffix = ":net::ERR_ABORTED";
    if (diagnostic.startsWith(prefix) && diagnostic.endsWith(suffix) && readyMatches) {
      const urlText = diagnostic.slice(prefix.length, -suffix.length);
      let requestPath = null;
      try {
        requestPath = new URL(urlText).pathname;
      } catch (_) {
        // A malformed diagnostic is never accepted as release bookkeeping.
      }
      const image = imagesByPath.get(requestPath);
      const metadata = metadataByPath.get(requestPath);
      const retained = image || metadata;
      const matches = observations.filter((entry) => entry.path === requestPath);
      const observation = matches.length === 1 ? matches[0] : null;
      if (
        retained
        && observation?.status === 200
        && observation.response_finished === true
        && observation.response_bytes === retained.byte_length
        && observation.canonical_byte_length === retained.byte_length
        && observation.canonical_sha256 === retained.sha256
      ) {
        expected.push({
          classification: image
            ? "completed_bridge_release"
            : "completed_metadata_bridge_release",
          diagnostic,
          path: requestPath,
          byte_length: retained.byte_length,
          sha256: retained.sha256,
          ...(metadata ? { metadata_kind: metadata.kind } : {}),
        });
        continue;
      }
    }
    fatal.push(diagnostic);
  }

  return { expected, fatal, raw: [...diagnostics] };
}

function compareWithE21(options, scenario) {
  assert.equal(
    sha256(fs.readFileSync(options.baselineScenario)),
    scenario.canonical.baseline_scenario_sha256,
    "accepted E21 scenario identity",
  );
  const baselineScenario = readJson(options.baselineScenario);
  assert.deepEqual(
    baselineScenario.probes.map((probe) => probe.topic_id),
    scenario.canonical.probe_topic_ids,
    "loose acceptance must retain the exact accepted E21 probe set",
  );
  const result = verifyArtifacts({
    scenario: options.baselineScenario,
    site: options.baselineSite,
    sourceRoot: options.sourceRoot,
  });
  assert.equal(result.runtime_pack_sha256, scenario.canonical.runtime_pack_sha256);
  assert.equal(result.runtime_pack_byte_length, scenario.canonical.runtime_pack_byte_length);
  assert.equal(result.catalog_sha256, scenario.canonical.catalog_sha256);
  assert.equal(result.manifest_sha256, scenario.canonical.manifest_sha256);
  assert.equal(result.encyclopedia_namespace_entries, scenario.canonical.encyclopedia_namespace_entries);
  assert.equal(result.source_profile, scenario.canonical.source_profile);
  assert.equal(result.owned_no_art_topic_count, 0);
  return result;
}

export async function selfTest(options = defaults) {
  assert.equal(typeof verifyArtifacts, "function");
  const scenario = readJson(options.scenario);
  validateScenario(scenario);
  assert.throws(
    () => validateScenario({
      ...scenario,
      invalid_runtime_pack: {
        ...scenario.invalid_runtime_pack,
        expected_diagnostic: "loose_fixture_unavailable: Invalid data/runtime.orpk: invented parser error",
      },
    }),
    /invalid runtime-pack parser contract/,
  );
  assert.throws(
    () => requireInvalidPackDiagnostic(
      ["page:RuntimeError: unreachable"],
      null,
      scenario,
    ),
    /expected invalid runtime-pack parse failure/,
  );
  const invalidPackReport = {
    schema_version: 1,
    status: "unavailable",
    code: scenario.fixture_codes.alliance,
    scenario: scenario.invalid_runtime_pack.report_scenario,
    faction: "alliance",
    surface: "loose-encyclopedia-fixture",
    source_profile: null,
    topic_id: null,
    title: null,
    title_sha256: null,
    body_sha256: null,
    body_chars: null,
    binding_family: null,
    binding_dat_id: null,
    binding_variant: null,
    asset_id: null,
    digest: null,
    cache_status: "not selected",
    navigation_requests: null,
    diagnostic: scenario.invalid_runtime_pack.expected_diagnostic,
    stable_frames: 3,
  };
  assert.equal(
    requireInvalidPackDiagnostic([], invalidPackReport, scenario),
    invalidPackReport.diagnostic,
  );
  assert.throws(
    () => requireInvalidPackDiagnostic(
      [],
      {
        ...invalidPackReport,
        diagnostic: "loose_fixture_unavailable: Invalid data/runtime.orpk: unsupported runtime-pack version 26924",
      },
      scenario,
    ),
    /expected invalid runtime-pack parse failure/,
  );
  assert.throws(
    () => requireInvalidPackDiagnostic(
      ["page:RuntimeError: unreachable"],
      invalidPackReport,
      scenario,
    ),
    /unexpected diagnostics during invalid-pack rejection/,
  );
  assert.throws(
    () => requireInvalidPackDiagnostic(
      [],
      { ...invalidPackReport, topic_id: "stale:topic" },
      scenario,
    ),
    /expected invalid runtime-pack parse failure/,
  );
  const partialPath = `${options.output}.partial-browser-self-test`;
  persistBrowserFailure(partialPath, {
    case: "ready",
    error: new Error("synthetic readiness failure"),
    observations: [{ path: "/data/runtime.orpk", status: 404 }],
    report: { status: "unavailable" },
  });
  const partial = readJson(partialPath);
  assert.equal(partial.status, "failed");
  assert.equal(partial.failure.case, "ready");
  assert.equal(partial.failure.error, "synthetic readiness failure");
  assert.deepEqual(partial.failure.report, { status: "unavailable" });
  assert.deepEqual(partial.failure.observations, [
    { path: "/data/runtime.orpk", status: 404 },
  ]);
  fs.unlinkSync(partialPath);

  const matrixFailurePath = `${options.output}.ready-before-matrix-failure-self-test`;
  const matrixLedger = createCompletedReadyLedger();
  const completedAlliance = {
    faction: "alliance",
    ready: { status: "ready", topic_id: "systems:death-star" },
    browser_diagnostics: {
      expected: [{ classification: "completed_metadata_bridge_release" }],
      fatal: [],
      raw: ["request:manifest:net::ERR_ABORTED"],
    },
    probes: [{
      selection: { topic_id: "systems:death-star", viewer: "alliance" },
      pixels: { different_pixels: 0, width: 84, height: 60 },
      visible_text: [{
        purpose: "long_body_scroll",
        captures: [{ stage: "after_PageDown_1", scroll_offset: 133 }],
      }],
    }],
  };
  matrixLedger.record(completedAlliance);
  completedAlliance.probes[0].selection.topic_id = "mutated-after-record";
  const matrixCaseObservations = [{ path: `${loosePrefix}manifest.json`, status: 500 }];
  const fullMatrixObservations = [
    { path: "/data/runtime.orpk", status: 404 },
    ...matrixCaseObservations,
  ];
  persistBrowserFailure(matrixFailurePath, finalizeBrowserFailure(matrixLedger.failure({
    case: "http_500",
    completed_cases: [{ case: "both_404", status: "pass" }],
    diagnostics: ["console:HTTP 500"],
    error: new Error("later matrix failure"),
    observations: matrixCaseObservations,
  }), fullMatrixObservations));
  const matrixFailure = readJson(matrixFailurePath);
  assert.equal(matrixFailure.failure.case, "http_500");
  assert.equal(matrixFailure.failure.error, "later matrix failure");
  assert.equal(matrixFailure.failure.cleanup, "closed");
  assert.deepEqual(matrixFailure.failure.observations, matrixCaseObservations);
  assert.deepEqual(matrixFailure.failure.all_observations, fullMatrixObservations);
  assert.deepEqual(matrixFailure.failure.completed_cases, [
    { case: "both_404", status: "pass" },
  ]);
  assert.equal(matrixFailure.failure.completed_ready_cases.length, 1);
  assert.equal(
    matrixFailure.failure.completed_ready_cases[0].probes[0].selection.topic_id,
    "systems:death-star",
  );
  assert.deepEqual(
    matrixFailure.failure.completed_ready_cases[0].browser_diagnostics,
    {
      expected: [{ classification: "completed_metadata_bridge_release" }],
      fatal: [],
      raw: ["request:manifest:net::ERR_ABORTED"],
    },
  );
  assert.deepEqual(
    matrixFailure.failure.completed_ready_cases[0].probes[0].pixels,
    { different_pixels: 0, width: 84, height: 60 },
  );
  assert.equal(
    matrixFailure.failure.completed_ready_cases[0].probes[0].visible_text[0].captures[0].scroll_offset,
    133,
  );
  fs.unlinkSync(matrixFailurePath);

  const secondFactionFailurePath = `${options.output}.first-faction-before-second-failure-self-test`;
  const factionLedger = createCompletedReadyLedger();
  factionLedger.record({ faction: "alliance", ready: { status: "ready" }, probes: [] });
  persistBrowserFailure(secondFactionFailurePath, finalizeBrowserFailure(factionLedger.failure({
    case: "ready_empire",
    diagnostics: ["console:empire failed"],
    error: new Error("second faction failure"),
    observations: [{ path: "/data/runtime.orpk", status: 404 }],
  }), [{ path: "/data/runtime.orpk", status: 404 }]));
  const secondFactionFailure = readJson(secondFactionFailurePath);
  assert.equal(secondFactionFailure.failure.case, "ready_empire");
  assert.equal(secondFactionFailure.failure.error, "second faction failure");
  assert.deepEqual(
    secondFactionFailure.failure.completed_ready_cases.map((entry) => entry.faction),
    ["alliance"],
  );
  fs.unlinkSync(secondFactionFailurePath);
  const e21 = compareWithE21(options, scenario);
  const controlled = await startControlledLooseServer(options);
  try {
    const manifestPath = `${loosePrefix}manifest.json`;
    const catalogPath = `${loosePrefix}catalog.json`;
    const completedMetadataObservation = (metadataPath, bytes, digest) => ({
      path: metadataPath,
      status: 200,
      canonical_byte_length: bytes.length,
      canonical_sha256: digest,
      response_bytes: bytes.length,
      response_finished: true,
    });
    const classifiedDiagnostics = classifyLooseBrowserDiagnostics({
      diagnostics: [
        "console:Failed to load resource: the server responded with a status of 404 (Not Found)",
        `request:${controlled.origin}${loosePrefix}${controlled.inventory.images[0].path}:net::ERR_ABORTED`,
        `request:${controlled.origin}${loosePrefix}${controlled.inventory.images[1].path}:net::ERR_ABORTED`,
        `request:${controlled.origin}${manifestPath}:net::ERR_ABORTED`,
        `request:${controlled.origin}${catalogPath}:net::ERR_ABORTED`,
        "console:unexpected synthetic browser error",
      ],
      inventory: controlled.inventory,
      observations: [
        { path: "/data/runtime.orpk", status: 404, response_finished: true },
        {
          path: `${loosePrefix}${controlled.inventory.images[0].path}`,
          status: 200,
          canonical_byte_length: controlled.inventory.images[0].byte_length,
          canonical_sha256: controlled.inventory.images[0].sha256,
          response_bytes: controlled.inventory.images[0].byte_length,
          response_finished: true,
        },
        {
          path: `${loosePrefix}${controlled.inventory.images[1].path}`,
          status: 200,
          canonical_byte_length: controlled.inventory.images[1].byte_length,
          canonical_sha256: controlled.inventory.images[1].sha256,
          response_bytes: controlled.inventory.images[1].byte_length,
          response_finished: false,
        },
        completedMetadataObservation(
          manifestPath,
          controlled.inventory.manifestBytes,
          scenario.canonical.manifest_sha256,
        ),
        completedMetadataObservation(
          catalogPath,
          controlled.inventory.catalogBytes,
          scenario.canonical.catalog_sha256,
        ),
      ],
      ready: { status: "ready", source_profile: scenario.canonical.source_profile },
      scenario,
    });
    assert.deepEqual(classifiedDiagnostics.expected.map((entry) => entry.classification), [
      "expected_absent_runtime_pack",
      "completed_bridge_release",
      "completed_metadata_bridge_release",
      "completed_metadata_bridge_release",
    ]);
    assert.deepEqual(classifiedDiagnostics.fatal, [
      `request:${controlled.origin}${loosePrefix}${controlled.inventory.images[1].path}:net::ERR_ABORTED`,
      "console:unexpected synthetic browser error",
    ]);

    const metadataAbort = `request:${controlled.origin}${manifestPath}:net::ERR_ABORTED`;
    const validManifestObservation = completedMetadataObservation(
      manifestPath,
      controlled.inventory.manifestBytes,
      scenario.canonical.manifest_sha256,
    );
    for (const [name, observations, ready] of [
      ["duplicate", [validManifestObservation, { ...validManifestObservation }],
        { status: "ready", source_profile: scenario.canonical.source_profile }],
      ["mismatched digest", [{ ...validManifestObservation, canonical_sha256: "0".repeat(64) }],
        { status: "ready", source_profile: scenario.canonical.source_profile }],
      ["mismatched length", [{ ...validManifestObservation, response_bytes: validManifestObservation.response_bytes - 1 }],
        { status: "ready", source_profile: scenario.canonical.source_profile }],
      ["unfinished", [{ ...validManifestObservation, response_finished: false }],
        { status: "ready", source_profile: scenario.canonical.source_profile }],
      ["pre-Ready", [validManifestObservation],
        { status: "unavailable", source_profile: scenario.canonical.source_profile }],
      ["wrong profile", [validManifestObservation],
        { status: "ready", source_profile: "unaccepted-profile" }],
    ]) {
      const rejected = classifyLooseBrowserDiagnostics({
        diagnostics: [metadataAbort],
        inventory: controlled.inventory,
        observations,
        ready,
        scenario,
      });
      assert.deepEqual(rejected.expected, [], `${name} metadata abort must not be expected`);
      assert.deepEqual(rejected.fatal, [metadataAbort], `${name} metadata abort must stay fatal`);
    }
    const unknownMetadataAbort =
      `request:${controlled.origin}${loosePrefix}unknown.json:net::ERR_ABORTED`;
    const unknownMetadata = classifyLooseBrowserDiagnostics({
      diagnostics: [unknownMetadataAbort],
      inventory: controlled.inventory,
      observations: [{ ...validManifestObservation, path: `${loosePrefix}unknown.json` }],
      ready: { status: "ready", source_profile: scenario.canonical.source_profile },
      scenario,
    });
    assert.deepEqual(unknownMetadata.expected, []);
    assert.deepEqual(unknownMetadata.fatal, [unknownMetadataAbort]);

    const uiManifestResponse = await fetch(`${controlled.origin}/data/ui/bmp-manifest.json`, {
      headers: { cookie: "e48_case=ready" },
    });
    assert.equal(uiManifestResponse.status, 200, "approved UI manifest must be served");
    const uiManifest = await uiManifestResponse.json();
    assert.ok(uiManifest.length > 0, "approved UI manifest must list resources");
    const firstUiResource = uiManifest[0];
    const firstUiPath = `/data/ui/${firstUiResource.dll}/BMP/${firstUiResource.id}.bmp`;
    const firstUiResponse = await fetch(`${controlled.origin}${firstUiPath}`, {
      headers: { cookie: "e48_case=ready" },
    });
    assert.equal(firstUiResponse.status, 200, "manifest-listed UI resource must be served");
    await firstUiResponse.arrayBuffer();
    const unlistedUiResponse = await fetch(`${controlled.origin}/data/ui/not-approved.bmp`, {
      headers: { cookie: "e48_case=ready" },
    });
    assert.equal(unlistedUiResponse.status, 404, "unlisted UI resource must be rejected");

    const ready = await fetchCandidate(controlled, "ready");
    assert.equal(ready.kind, "ready");
    assert.equal(ready.catalog_sha256, scenario.canonical.catalog_sha256);
    assert.equal(ready.manifest_sha256, scenario.canonical.manifest_sha256);
    assert.equal(ready.source_profile, scenario.canonical.source_profile);
    assert.equal(ready.image_count, controlled.inventory.images.length);
    assert.equal(controlled.observations.max_active_images, 4);
    assert.equal(controlled.observations.active_images, 0);
    const readyRequests = controlled.observations.requests.filter((request) =>
      request.case === "ready" && request.path.startsWith(loosePrefix));
    assert.equal(readyRequests.filter((request) => request.unlisted).length, 0);
    assert.equal(readyRequests.length, controlled.inventory.declaredPaths.size);
    assert.deepEqual(
      new Set(readyRequests.map((request) => request.path)),
      new Set([
        `${loosePrefix}manifest.json`,
        `${loosePrefix}catalog.json`,
        ...controlled.inventory.images.map((image) => `${loosePrefix}${image.path}`),
      ]),
    );

    assert.deepEqual(await fetchCandidate(controlled, "both_missing"), { kind: "unavailable" });
    await expectCandidateFailure(controlled, "partial_catalog_only", "integrity_error");
    await expectCandidateFailure(controlled, "partial_manifest_only", "integrity_error");
    await expectCandidateFailure(controlled, "catalog_forbidden", "transport_error");
    await expectCandidateFailure(controlled, "manifest_server_error", "transport_error");
    await expectCandidateFailure(controlled, "catalog_network_reset", "transport_error");
    await expectCandidateFailure(controlled, "catalog_html", "integrity_error");
    await expectCandidateFailure(controlled, "image_overflow_absent_length", "resource_limit");
    await expectCandidateFailure(controlled, "image_overflow_misleading_length", "resource_limit");

    const heldHeaders = {
      cookie: "e48_case=cancelled_image",
      "x-e48-case": "cancelled_image",
    };
    const controller = new AbortController();
    const held = fetch(
      `${controlled.origin}${loosePrefix}${controlled.inventory.overflowPath}`,
      { headers: heldHeaders, signal: controller.signal },
    ).then((response) => response.arrayBuffer());
    await waitUntil(() => controlled.observations.active_images === 1, "held image response");
    controller.abort();
    controlled.observations.client_aborts += 1;
    await assert.rejects(held, /AbortError|aborted|abort/i);
    assert.equal(controller.signal.aborted, true);
    await waitUntil(() => controlled.observations.active_images === 0, "cancelled response cleanup");
    assert.equal(controlled.observations.client_aborts, 1);

    const beforeBadPack = controlled.observations.requests.length;
    const badPackIndex = await fetch(controlled.cases.bad_present_pack);
    assert.equal(badPackIndex.status, 200);
    const badPackHeaders = { cookie: badPackIndex.headers.get("set-cookie") };
    for (const resource of ["/gl.js", "/open-rebellion-test.wasm"]) {
      const response = await fetch(`${controlled.origin}${resource}`, { headers: badPackHeaders });
      assert.equal(response.status, 200);
      await response.arrayBuffer();
    }
    const badPack = await fetch(`${controlled.origin}/data/runtime.orpk`, {
      headers: badPackHeaders,
    });
    assert.equal(badPack.status, 200);
    const badPackBytes = Buffer.from(await badPack.arrayBuffer());
    assert.equal(badPackBytes.toString(), "ORPK-invalid-present-pack");
    assert.equal(badPackBytes.length, scenario.invalid_runtime_pack.byte_length);
    assert.equal(sha256(badPackBytes), scenario.invalid_runtime_pack.sha256);
    assert.equal(badPackBytes.readUInt16LE(4), scenario.invalid_runtime_pack.decoded_version);
    assert.equal(
      `unsupported runtime-pack version ${badPackBytes.readUInt16LE(4)}`,
      scenario.invalid_runtime_pack.parser_error,
    );
    const badPackRequests = controlled.observations.requests.slice(beforeBadPack);
    assert.equal(badPackRequests.some((request) => request.path.startsWith(loosePrefix)), false);
    const badPackResponses = badPackRequests.filter((request) =>
      request.path === "/data/runtime.orpk" && request.status === 200);
    assert.equal(badPackResponses.length, 1);
    assert.equal(badPackResponses[0].response_bytes, scenario.invalid_runtime_pack.byte_length);
    assert.equal(
      badPackResponses[0].canonical_byte_length,
      scenario.invalid_runtime_pack.byte_length,
    );
    assert.equal(badPackResponses[0].canonical_sha256, scenario.invalid_runtime_pack.sha256);
    const badPackBrowserRequests = [
      "/",
      "/gl.js",
      "/open-rebellion-test.wasm",
      "/data/runtime.orpk",
    ];
    requireInvalidPackTransport(badPackBrowserRequests, badPackRequests, scenario);
    assert.throws(
      () => requireInvalidPackTransport(
        [...badPackBrowserRequests, `${loosePrefix}catalog.json`],
        badPackRequests,
        scenario,
      ),
      /invalid runtime-pack transport/,
    );
    assert.throws(
      () => requireInvalidPackTransport(
        badPackBrowserRequests,
        [...badPackRequests, badPackResponses[0]],
        scenario,
      ),
      /invalid runtime-pack transport/,
    );
    assert.throws(
      () => requireInvalidPackTransport(
        badPackBrowserRequests,
        badPackRequests.map((request) => request.path === "/data/runtime.orpk"
          ? { ...request, canonical_sha256: "0".repeat(64) }
          : request),
        scenario,
      ),
      /invalid runtime-pack transport/,
    );

    const unlisted = await fetch(`${controlled.origin}${loosePrefix}assets/UNLISTED.999`, {
      headers: { cookie: "e48_case=ready" },
    });
    assert.equal(unlisted.status, 404);

    return {
      canonical: {
        catalog_sha256: ready.catalog_sha256,
        manifest_sha256: ready.manifest_sha256,
        runtime_pack_sha256: e21.runtime_pack_sha256,
        source_profile: ready.source_profile,
      },
      cases_verified: scenario.cases.map((entry) => entry.id),
      cleanup: "pending",
      image_count: ready.image_count,
      max_active_image_downloads: controlled.observations.max_active_images,
      owned_no_art_topic_count: 0,
      ready_request_count: readyRequests.length,
      selected_dat_basenames: controlled.inventory.selectedDatBasenames,
      ui_inventory_sha256: scenario.canonical.ui_inventory_sha256,
      ui_manifest_sha256: scenario.canonical.ui_manifest_sha256,
      ui_resource_count: controlled.inventory.uiManifest.length,
      status: "pass",
    };
  } finally {
    await controlled.close();
  }
}

async function runRealLoaderFailureMatrix(controlled, onFailure = () => {}) {
  const dependencyRoot = process.env.OPEN_REBELLION_INTERFACE_NODE_MODULES;
  const playwrightModule = dependencyRoot
    ? pathToFileURL(path.join(dependencyRoot, "playwright-core/index.mjs")).href
    : "playwright-core";
  const [{ chromium }, { launchBrowser }] = await Promise.all([
    import(playwrightModule),
    import("./browser-launch.mjs"),
  ]);
  const browserManifest = readJson(path.join(here, "browser.json"));
  const launchAttempts = [];
  const browser = await launchBrowser(chromium, {
    executablePath: browserExecutable(browserManifest),
    headless: true,
    args: browserManifest.launch_arguments,
    timeout: 30_000,
  }, launchAttempts);
  const browserVersion = browser.version();
  const results = [];

  const openCase = async (caseId) => {
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
    const diagnostics = [];
    page.on("request", (request) => browserRequests.push(new URL(request.url()).pathname));
    page.on("requestfailed", (request) => diagnostics.push(`request:${request.url()}:${request.failure()?.errorText}`));
    page.on("pageerror", (error) => diagnostics.push(`page:${error.stack || error.message}`));
    page.on("console", (message) => {
      if (message.type() === "error") diagnostics.push(`console:${message.text()}`);
    });
    const observationStart = controlled.observations.requests.length;
    try {
      await page.goto(
        `${controlled.cases[caseId]}&fixture-code=${controlled.scenario.fixture_codes.alliance}`,
        { waitUntil: "load", timeout: 30_000 },
      );
      return { browserRequests, context, diagnostics, observationStart, page };
    } catch (error) {
      onFailure({
        browser_requests: browserRequests,
        case: caseId,
        completed_cases: [...results],
        diagnostics,
        error,
        observations: controlled.observations.requests.slice(observationStart),
        report: null,
      });
      await context.close();
      throw error;
    }
  };

  try {
    for (const acceptanceCase of controlled.scenario.cases.filter((entry) =>
      !["ready", "cancelled_image", "bad_present_pack"].includes(entry.id))) {
      const opened = await openCase(acceptanceCase.id);
      try {
        await opened.page.waitForFunction(
          () => window.__openRebellionInterfaceReady?.status,
          null,
          { timeout: 180_000 },
        );
        const report = await opened.page.evaluate(() => window.__openRebellionInterfaceReady);
        assert.equal(report.status, "unavailable", `${acceptanceCase.id} report`);
        assert.equal(report.surface, "loose-encyclopedia-fixture");
        assert.equal(report.faction, "alliance");
        assert.equal(report.code, controlled.scenario.fixture_codes.alliance);
        assert.ok(
          report.diagnostic?.includes(acceptanceCase.diagnostic_contains),
          `${acceptanceCase.id} diagnostic ${JSON.stringify(report.diagnostic)}`,
        );
        const observed = controlled.observations.requests.slice(opened.observationStart);
        assert.equal(observed.some((request) => request.unlisted), false);
        assert.equal(
          observed.filter((request) => request.path === "/data/runtime.orpk" && request.status === 404).length,
          1,
          `${acceptanceCase.id} absent runtime pack`,
        );
        assert.ok(
          observed.some((request) => request.path.startsWith(loosePrefix)),
          `${acceptanceCase.id} must reach the real loose bridge`,
        );
        results.push({
          case: acceptanceCase.id,
          status: "pass",
          report,
          browser_requests: opened.browserRequests,
          diagnostics: opened.diagnostics,
          observations: observed,
        });
      } catch (error) {
        onFailure({
          browser_requests: opened.browserRequests,
          case: acceptanceCase.id,
          completed_cases: [...results],
          diagnostics: opened.diagnostics,
          error,
          observations: controlled.observations.requests.slice(opened.observationStart),
          report: await opened.page.evaluate(
            () => window.__openRebellionInterfaceReady ?? null,
          ).catch(() => null),
        });
        throw error;
      } finally {
        await opened.context.close();
      }
    }

    const cancelled = await openCase("cancelled_image");
    const cancelledBefore = controlled.observations.cancelled_responses;
    try {
      await waitUntil(
        () => controlled.observations.active_images > 0,
        "real loader held image response",
        180_000,
      );
      await cancelled.context.close();
      await waitUntil(
        () => controlled.observations.active_images === 0,
        "real loader cancellation cleanup",
        30_000,
      );
      const cancelledObserved = controlled.observations.requests.slice(cancelled.observationStart);
      assert.equal(cancelledObserved.some((request) => request.unlisted), false);
      assert.ok(cancelledObserved.some((request) => request.held));
      assert.ok(
        controlled.observations.cancelled_responses > cancelledBefore,
        "closing the browser context must cancel the held image response",
      );
      results.push({
        case: "cancelled_image",
        status: "pass",
        active_images_after_close: controlled.observations.active_images,
        browser_requests: cancelled.browserRequests,
        cancelled_responses: controlled.observations.cancelled_responses - cancelledBefore,
        diagnostics: cancelled.diagnostics,
        observations: cancelledObserved,
      });
    } catch (error) {
      onFailure({
        browser_requests: cancelled.browserRequests,
        case: "cancelled_image",
        completed_cases: [...results],
        diagnostics: cancelled.diagnostics,
        error,
        observations: controlled.observations.requests.slice(cancelled.observationStart),
        report: null,
      });
      throw error;
    } finally {
      await cancelled.context.close();
    }

    const badPack = await openCase("bad_present_pack");
    try {
      await badPack.page.waitForFunction(
        () => window.__openRebellionInterfaceReady?.status,
        null,
        { timeout: 30_000 },
      );
      const report = await badPack.page.evaluate(() => window.__openRebellionInterfaceReady);
      const invalidPackDiagnostic = requireInvalidPackDiagnostic(
        badPack.diagnostics,
        report,
        controlled.scenario,
      );
      assert.equal(report.status, "unavailable");
      assert.equal(report.source_profile, null);
      assert.equal(report.topic_id, null);
      assert.equal(report.asset_id, null);
      const observed = controlled.observations.requests.slice(badPack.observationStart);
      requireInvalidPackTransport(badPack.browserRequests, observed, controlled.scenario);
      results.push({
        case: "bad_present_pack",
        status: "pass",
        report,
        browser_requests: badPack.browserRequests,
        diagnostics: badPack.diagnostics,
        invalid_pack_diagnostic: invalidPackDiagnostic,
        observations: observed,
      });
    } catch (error) {
      onFailure({
        browser_requests: badPack.browserRequests,
        case: "bad_present_pack",
        completed_cases: [...results],
        diagnostics: badPack.diagnostics,
        error,
        observations: controlled.observations.requests.slice(badPack.observationStart),
        report: await badPack.page.evaluate(
          () => window.__openRebellionInterfaceReady ?? null,
        ).catch(() => null),
      });
      throw error;
    } finally {
      await badPack.context.close();
    }
  } finally {
    await browser.close();
  }

  return {
    browser_version: browserVersion,
    launch_attempts: launchAttempts,
    muted: browserManifest.launch_arguments.includes("--mute-audio"),
    cases: results,
  };
}

async function main() {
  const options = parseArguments(process.argv.slice(2));
  if (options.help) {
    console.log(
      "Usage:\n"
      + "  node tools/interface-parity/encyclopedia-loose-acceptance.mjs --self-test [paths]\n"
      + "  node tools/interface-parity/encyclopedia-loose-acceptance.mjs --serve [paths]\n"
      + "  node tools/interface-parity/encyclopedia-loose-acceptance.mjs --browser [paths]\n"
      + "Options: --scenario --baseline-scenario --baseline-site --site --source-root --ui-root --output\n"
      + "The self-test exercises real loopback HTTP but is not live viewer acceptance. "
      + "Serve prepares URLs; --browser is reserved for the coordinator-owned muted browser gate.",
    );
    return;
  }
  if (options.selfTest) {
    const result = await selfTest(options);
    result.cleanup = "closed";
    fs.mkdirSync(path.dirname(options.output), { recursive: true });
    fs.writeFileSync(options.output, `${JSON.stringify(result, null, 2)}\n`);
    console.log(JSON.stringify({ ...result, output: path.resolve(options.output) }));
    return;
  }
  const scenario = readJson(options.scenario);
  const e21 = compareWithE21(options, scenario);
  const controlled = await startControlledLooseServer(options);
  if (options.browser) {
    let browserError = null;
    let latestFailure = null;
    const readyLedger = createCompletedReadyLedger();
    const recordFailure = (failure) => {
      latestFailure = readyLedger.failure({
        ...failure,
        cleanup: "pending",
        observations: failure.observations ?? controlled.observations.requests,
      });
      persistBrowserFailure(options.output, latestFailure);
    };
    try {
      const result = await runBrowser({
        output: options.output,
        scenario: options.baselineScenario,
        site: options.site,
        sourceRoot: options.sourceRoot,
      }, e21, {
        origin: controlled.origin,
        fixtureCodes: scenario.fixture_codes,
        reportTimeoutMs: 180_000,
        scope: "owned canonical loose HTTP transport and shared feature-surface rendering checkpoint",
        onCaseStart: () => controlled.observations.requests.length,
        onCaseFailure: (failure) => recordFailure({
          browser_requests: failure.browserRequests,
          case: `ready_${failure.faction}`,
          console_lines: failure.consoleLines,
          diagnostics: failure.errors,
          error: failure.error,
          fixture_code: failure.fixtureCode,
          observations: controlled.observations.requests.slice(failure.caseToken),
          report: failure.ready,
        }),
        onCaseComplete: ({ caseResult }) => readyLedger.record(caseResult),
        assertReady: ({ faction, ready }) => {
          assert.equal(ready.surface, "loose-encyclopedia-fixture", `${faction} transport label`);
          assert.equal(ready.code, scenario.fixture_codes[faction], `${faction} fixture code`);
        },
        assertStartupRequests: ({ browserRequests, caseToken, faction, navigationRequestStart }) => {
          assert.equal(
            browserRequests.length,
            navigationRequestStart,
            `${faction} must remain network-quiet during offline navigation`,
          );
          const observed = controlled.observations.requests.slice(caseToken);
          assert.equal(observed.some((request) => request.unlisted), false, `${faction} unlisted request`);
          assert.deepEqual(
            observed
              .filter((request) => request.status !== 200 && request.path !== "/data/runtime.orpk")
              .map((request) => ({ path: request.path, status: request.status })),
            [],
            `${faction} unexpected non-success response`,
          );
          assert.equal(
            observed.filter((request) => request.path === "/data/runtime.orpk" && request.status === 404).length,
            1,
            `${faction} must exercise the loose fallback after one absent pack request`,
          );
          const loose = observed.filter((request) => request.path.startsWith(loosePrefix));
          assert.equal(loose.length, controlled.inventory.declaredPaths.size);
          assert.deepEqual(
            new Set(loose.map((request) => request.path)),
            new Set([...controlled.inventory.declaredPaths].map((relative) => `${loosePrefix}${relative}`)),
          );
          const ui = observed.filter((request) => request.path.startsWith("/data/ui/"));
          assert.equal(ui.length, controlled.inventory.uiResources.size);
          assert.deepEqual(
            new Set(ui.map((request) => request.path)),
            new Set([...controlled.inventory.uiResources.keys()].map((relative) => `/data/ui/${relative}`)),
          );
          for (const request of ui) {
            const relative = request.path.slice("/data/ui/".length);
            const approved = controlled.inventory.uiResources.get(relative);
            assert.ok(approved, `${faction} UI request is outside the approved inventory: ${request.path}`);
            assert.equal(request.status, 200, `${faction} UI response status: ${request.path}`);
            assert.equal(request.response_finished, true, `${faction} UI response completion: ${request.path}`);
            assert.equal(request.response_bytes, approved.byteLength, `${faction} UI response length: ${request.path}`);
            assert.equal(request.canonical_byte_length, approved.byteLength, `${faction} UI canonical length: ${request.path}`);
            assert.equal(request.canonical_sha256, approved.digest, `${faction} UI canonical digest: ${request.path}`);
          }
        },
        classifyDiagnostics: ({ caseToken, diagnostics, ready }) =>
          classifyLooseBrowserDiagnostics({
            diagnostics,
            inventory: controlled.inventory,
            observations: controlled.observations.requests.slice(caseToken),
            ready,
            scenario,
          }),
      });
      assert.equal(controlled.observations.active_images, 0);
      assert.equal(
        controlled.observations.max_active_images,
        scenario.limits.max_active_image_downloads,
      );
      const failureMatrix = await runRealLoaderFailureMatrix(controlled, recordFailure);
      result.transport = {
        active_images_after_run: controlled.observations.active_images,
        failure_matrix: failureMatrix,
        max_active_images: controlled.observations.max_active_images,
        observations: controlled.observations.requests,
      };
      fs.mkdirSync(path.dirname(options.output), { recursive: true });
      fs.writeFileSync(options.output, `${JSON.stringify(result, null, 2)}\n`);
      console.log(JSON.stringify({ status: result.status, output: path.resolve(options.output) }));
    } catch (error) {
      browserError = error;
      if (!latestFailure) {
        recordFailure({
          case: "browser_startup",
          error,
          observations: controlled.observations.requests,
          report: null,
        });
      }
    } finally {
      await controlled.close();
      if (browserError) {
        persistBrowserFailure(
          options.output,
          finalizeBrowserFailure(latestFailure, controlled.observations.requests),
        );
      }
    }
    if (browserError) throw browserError;
    return;
  }
  const urls = Object.fromEntries(Object.entries(controlled.cases).flatMap(([caseId, baseUrl]) =>
    Object.entries(scenario.fixture_codes).map(([faction, fixtureCode]) => [
      `${caseId}_${faction}`,
      `${baseUrl}&fixture-code=${fixtureCode}`,
    ])));
  console.log(JSON.stringify({
    browser_audio: "muted",
    canonical: scenario.canonical,
    evidence_output: path.resolve(options.output),
    live_acceptance: "coordinator-owned; not passed by this server",
    origin: controlled.origin,
    pid: process.pid,
    urls,
  }));
  await new Promise((resolve) => {
    process.once("SIGINT", resolve);
    process.once("SIGTERM", resolve);
  });
  await controlled.close();
  const result = {
    cleanup: "closed",
    observations: controlled.observations,
    scenario: path.resolve(options.scenario),
  };
  fs.mkdirSync(path.dirname(options.output), { recursive: true });
  fs.writeFileSync(options.output, `${JSON.stringify(result, null, 2)}\n`);
}

if (fileURLToPath(import.meta.url) === process.argv[1]) {
  main().catch((error) => {
    console.error(error.stack || error);
    process.exitCode = 1;
  });
}
