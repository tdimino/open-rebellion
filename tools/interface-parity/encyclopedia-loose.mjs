#!/usr/bin/env node

import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import fs from "node:fs";
import http from "node:http";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const fixtureRoot = path.join(
  root,
  "tests/fixtures/encyclopedia/fixtures/bundles/valid",
);
const mainSourcePath = path.join(root, "crates/rebellion-app/src/main.rs");
const looseSourcePath = path.join(root, "crates/rebellion-app/src/encyclopedia_loose.rs");
const loosePrefix = "/data/encyclopedia/";
const productionHtmlPath = path.join(root, "web/index.html");
const productionGluePath = path.join(root, "web/gl.js");
const productionLoadMarker = '    <script>load("open-rebellion.wasm");</script>';
const browserProbeMagic = 0xe1170000;
const browserProbeCases = Object.freeze([
  { id: 1, name: "success" },
  { id: 2, name: "both_missing" },
  { id: 3, name: "partial_metadata" },
  { id: 4, name: "malformed_catalog" },
  { id: 5, name: "failed_image" },
  { id: 6, name: "oversized_image" },
  { id: 7, name: "dat_mismatch" },
]);

const interfaceFixturePlugin = `    <script>
        (function registerInterfaceFixtureBridge() {
            "use strict";
            const params = new URLSearchParams(window.location.search);
            const values = params.getAll("fixture-code");
            const fixtureCode = values.length === 1 && /^\\d+$/.test(values[0])
                ? Number(values[0])
                : 0;
            function register(imports) {
                imports.env.open_rebellion_interface_fixture_code = function () {
                    return fixtureCode;
                };
                imports.env.open_rebellion_interface_fixture_emit = function (ptr, len) {
                    try {
                        const bytes = new Uint8Array(wasm_memory.buffer, ptr, len);
                        window.__openRebellionInterfaceReady = JSON.parse(
                            new TextDecoder("utf-8", { fatal: true }).decode(bytes),
                        );
                    } catch (error) {
                        window.__openRebellionInterfaceReady = {
                            schema: "open-rebellion-e17-browser-probe-v1",
                            status: "failed",
                            published: false,
                            error: String(error),
                        };
                    }
                    window.dispatchEvent(new CustomEvent(
                        "open-rebellion-interface-ready",
                        { detail: window.__openRebellionInterfaceReady },
                    ));
                };
            }
            miniquad_add_plugin({
                register_plugin: register,
                version: 1,
                name: "open_rebellion_interface_fixture",
            });
        }());
    </script>
    <script>load("open-rebellion-test.wasm");</script>`;

function digest(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

function fixtureFiles() {
  return new Map([
    ["catalog.json", fs.readFileSync(path.join(fixtureRoot, "catalog.json"))],
    ["manifest.json", fs.readFileSync(path.join(fixtureRoot, "manifest.json"))],
    ["assets/EDATA.001", fs.readFileSync(path.join(fixtureRoot, "assets/EDATA.001"))],
    ["assets/EDATA.002", fs.readFileSync(path.join(fixtureRoot, "assets/EDATA.002"))],
    ["assets/EDATA.003", fs.readFileSync(path.join(fixtureRoot, "assets/EDATA.003"))],
  ]);
}

function browserProbeCode(probeCase) {
  return (browserProbeMagic | probeCase.id) >>> 0;
}

function browserProbeCaseFromCode(value) {
  if (!/^\d+$/.test(value || "")) return null;
  const code = Number(value);
  if (!Number.isSafeInteger(code) || (code >>> 16) !== (browserProbeMagic >>> 16)) {
    return null;
  }
  return browserProbeCases.find(probeCase => probeCase.id === (code & 0xffff)) || null;
}

function browserProbeHtml() {
  const productionHtml = fs.readFileSync(productionHtmlPath, "utf8");
  assert.ok(
    productionHtml.includes(productionLoadMarker),
    "production HTML load marker changed; probe shell cannot be generated",
  );
  assert.match(
    productionHtml,
    /open_rebellion_encyclopedia_fetch_start/,
    "probe shell must retain the actual E15 status-aware JS bridge",
  );
  return productionHtml.replace(productionLoadMarker, interfaceFixturePlugin);
}

function cookieProbeCase(request) {
  const cookie = request.headers.cookie || "";
  const match = /(?:^|;\s*)e17_probe=([a-z_]+)/.exec(cookie);
  return match
    ? browserProbeCases.find(probeCase => probeCase.name === match[1]) || null
    : null;
}

function sendBytes(response, status, bytes, contentType) {
  response.writeHead(status, {
    "cache-control": "no-store",
    "content-length": bytes.length,
    "content-type": contentType,
  });
  response.end(bytes);
}

function browserProbeFixtureResponse(probeCase, relative, files) {
  if (!probeCase) return { status: 400, bytes: Buffer.from("missing probe case") };
  if (probeCase.name === "both_missing"
      && (relative === "catalog.json" || relative === "manifest.json")) {
    return { status: 404, bytes: Buffer.alloc(0) };
  }
  if (probeCase.name === "partial_metadata" && relative === "manifest.json") {
    return { status: 404, bytes: Buffer.alloc(0) };
  }
  if (probeCase.name === "malformed_catalog" && relative === "catalog.json") {
    return { status: 200, bytes: Buffer.from("<!doctype html>") };
  }
  if (probeCase.name === "failed_image" && relative === "assets/EDATA.002") {
    return { status: 503, bytes: Buffer.from("synthetic image failure") };
  }
  const bytes = files.get(relative);
  if (!bytes) return { status: 404, bytes: Buffer.alloc(0) };
  if (probeCase.name === "oversized_image" && relative === "assets/EDATA.002") {
    return { status: 200, bytes: Buffer.concat([bytes, Buffer.from([0])]) };
  }
  return { status: 200, bytes };
}

export async function startBrowserProbeServer(wasmPath) {
  const wasmBytes = fs.readFileSync(wasmPath);
  const html = Buffer.from(browserProbeHtml());
  const glue = fs.readFileSync(productionGluePath);
  const files = fixtureFiles();
  const observations = [];
  const server = http.createServer((request, response) => {
    const url = new URL(request.url, "http://127.0.0.1");
    observations.push({
      method: request.method,
      path: url.pathname,
      probe: cookieProbeCase(request)?.name || null,
    });
    if (url.pathname === "/" || url.pathname === "/index.html") {
      const probeCase = browserProbeCaseFromCode(url.searchParams.get("fixture-code"));
      if (!probeCase) {
        sendBytes(response, 400, Buffer.from("invalid E17 fixture-code"), "text/plain");
        return;
      }
      response.setHeader("set-cookie", `e17_probe=${probeCase.name}; Path=/; SameSite=Strict`);
      sendBytes(response, 200, html, "text/html; charset=utf-8");
      return;
    }
    if (url.pathname === "/gl.js") {
      sendBytes(response, 200, glue, "text/javascript; charset=utf-8");
      return;
    }
    if (url.pathname === "/open-rebellion-test.wasm") {
      sendBytes(response, 200, wasmBytes, "application/wasm");
      return;
    }
    if (url.pathname.startsWith(loosePrefix)) {
      const relative = url.pathname.slice(loosePrefix.length);
      const probeCase = cookieProbeCase(request);
      const fixture = browserProbeFixtureResponse(probeCase, relative, files);
      sendBytes(
        response,
        fixture.status,
        fixture.bytes,
        relative.endsWith(".json") ? "application/json" : "application/octet-stream",
      );
      return;
    }
    sendBytes(response, 404, Buffer.alloc(0), "application/octet-stream");
  });
  await new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(0, "127.0.0.1", resolve);
  });

  let closed = false;
  const origin = `http://127.0.0.1:${server.address().port}`;
  return {
    artifact: {
      byte_len: wasmBytes.length,
      path: path.resolve(wasmPath),
      sha256: digest(wasmBytes),
    },
    cases: Object.fromEntries(browserProbeCases.map(probeCase => [
      probeCase.name,
      `${origin}/?fixture-code=${browserProbeCode(probeCase)}`,
    ])),
    observations,
    origin,
    async close() {
      if (closed) return;
      closed = true;
      server.closeAllConnections();
      await new Promise(resolve => server.close(resolve));
    },
  };
}

export async function startLooseEncyclopediaServer(files = fixtureFiles()) {
  const observations = {
    active_images: 0,
    max_active_images: 0,
    requests: [],
  };
  const server = http.createServer((request, response) => {
    const pathname = new URL(request.url, "http://127.0.0.1").pathname;
    const relative = pathname.startsWith(loosePrefix)
      ? pathname.slice(loosePrefix.length)
      : null;
    const bytes = relative === null ? undefined : files.get(relative);
    observations.requests.push({ method: request.method, path: pathname });
    if (!bytes) {
      response.writeHead(404, { "cache-control": "no-store", "content-length": 0 });
      response.end();
      return;
    }

    const isImage = relative.startsWith("assets/");
    if (isImage) {
      observations.active_images += 1;
      observations.max_active_images = Math.max(
        observations.max_active_images,
        observations.active_images,
      );
    }
    response.writeHead(200, {
      "cache-control": "no-store",
      "content-length": bytes.length,
      "content-type": relative.endsWith(".json")
        ? "application/json"
        : "application/octet-stream",
    });
    const finish = () => {
      response.end(bytes);
      if (isImage) observations.active_images -= 1;
    };
    if (isImage) setTimeout(finish, 20);
    else finish();
  });
  await new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(0, "127.0.0.1", resolve);
  });

  let closed = false;
  return {
    observations,
    origin: `http://127.0.0.1:${server.address().port}`,
    async close() {
      if (closed) return;
      closed = true;
      server.closeAllConnections();
      await new Promise(resolve => server.close(resolve));
    },
  };
}

async function boundedFetch(url, maxBytes) {
  const response = await fetch(url);
  if (response.status === 404) return { kind: "not_found" };
  assert.equal(response.status, 200, `${url} returned HTTP ${response.status}`);
  assert.ok(response.body, `${url} lacks a streaming response body`);
  const reader = response.body.getReader();
  const chunks = [];
  let total = 0;
  try {
    for (;;) {
      const { done, value } = await reader.read();
      if (done) break;
      if (value.byteLength === 0) continue;
      total += value.byteLength;
      assert.ok(total <= maxBytes, `${url} exceeded ${maxBytes} bytes`);
      chunks.push(value);
    }
  } catch (error) {
    await reader.cancel().catch(() => {});
    throw error;
  }
  const bytes = Buffer.allocUnsafe(total);
  let offset = 0;
  for (const chunk of chunks) {
    bytes.set(chunk, offset);
    offset += chunk.byteLength;
  }
  return { bytes, kind: "ready" };
}

async function fetchCandidate(origin) {
  const [manifestResult, catalogResult] = await Promise.all([
    boundedFetch(`${origin}${loosePrefix}manifest.json`, 32 * 1024 * 1024),
    boundedFetch(`${origin}${loosePrefix}catalog.json`, 64 * 1024 * 1024),
  ]);
  assert.equal(manifestResult.kind, "ready");
  assert.equal(catalogResult.kind, "ready");
  const manifest = JSON.parse(manifestResult.bytes);
  const catalog = JSON.parse(catalogResult.bytes);
  const imageDescriptors = Object.values(catalog.images).sort((left, right) =>
    left.path.localeCompare(right.path),
  );
  const expectedFiles = ["catalog.json", ...imageDescriptors.map(image => image.path)].sort();
  assert.deepEqual(Object.keys(manifest.files).sort(), expectedFiles);
  assert.equal(digest(catalogResult.bytes), manifest.catalog_sha256);

  const retained = new Map([
    ["catalog.json", catalogResult.bytes],
    ["manifest.json", manifestResult.bytes],
  ]);
  for (let offset = 0; offset < imageDescriptors.length; offset += 4) {
    const batch = imageDescriptors.slice(offset, offset + 4);
    const results = await Promise.all(batch.map(image =>
      boundedFetch(`${origin}${loosePrefix}${image.path}`, image.byte_length),
    ));
    for (let index = 0; index < batch.length; index += 1) {
      const image = batch[index];
      const result = results[index];
      assert.equal(result.kind, "ready");
      assert.equal(result.bytes.length, image.byte_length);
      assert.equal(digest(result.bytes), image.sha256);
      assert.equal(digest(result.bytes), manifest.files[image.path]);
      retained.set(image.path, result.bytes);
    }
  }
  return { catalog, manifest, retained };
}

export async function selfTestLooseEncyclopediaHarness() {
  const controlled = await startLooseEncyclopediaServer();
  try {
    const candidate = await fetchCandidate(controlled.origin);
    const expectedRequests = [
      "/data/encyclopedia/manifest.json",
      "/data/encyclopedia/catalog.json",
      "/data/encyclopedia/assets/EDATA.001",
      "/data/encyclopedia/assets/EDATA.002",
      "/data/encyclopedia/assets/EDATA.003",
    ];
    assert.deepEqual(
      controlled.observations.requests.map(request => request.path),
      expectedRequests,
    );
    assert.ok(controlled.observations.max_active_images <= 4);
    assert.equal(candidate.retained.size, expectedRequests.length);
    const mainSource = fs.readFileSync(mainSourcePath, "utf8");
    const legacyStart = mainSource.indexOf("async fn load_legacy_wasm_assets");
    const legacyEnd = mainSource.indexOf("async fn load_wasm_assets", legacyStart);
    const legacySource = mainSource.slice(legacyStart, legacyEnd);
    const preparation = legacySource.indexOf("prepare_browser_loose_encyclopedia");
    assert.ok(preparation >= 0, "legacy startup must invoke the bounded loose loader");
    assert.equal(
      (mainSource.match(/prepare_browser_loose_encyclopedia/g) || []).length,
      1,
      "startup is the sole loose-loader call site; navigation must not fetch",
    );
    for (const globalSetter of [
      "rebellion_data::set_string_table(",
      "rebellion_data::set_file_cache(",
      "rebellion_render::set_encyclopedia_asset_cache(",
    ]) {
      assert.ok(
        legacySource.indexOf(globalSetter) > preparation,
        `${globalSetter} must run only after complete loose preparation`,
      );
    }
    const looseSource = fs.readFileSync(looseSourcePath, "utf8");
    assert.doesNotMatch(
      looseSource,
      /macroquad::file::load_file/,
      "status-sensitive encyclopedia requests must use the E15 bridge",
    );
    const packedStart = mainSource.indexOf("async fn load_wasm_assets");
    const packedSource = mainSource.slice(packedStart, mainSource.indexOf("/// Cache", packedStart));
    assert.match(
      packedSource,
      /Ok\(bytes\)[\s\S]*?install_runtime_pack\(&bytes\)[\s\S]*?panic!\("Invalid data\/runtime\.orpk/,
      "a present invalid ORPK must panic rather than enter loose fallback",
    );
    return {
      catalog_sha256: digest(candidate.retained.get("catalog.json")),
      cleanup: "pending",
      scope: "independent-js-transport-model-plus-static-production-wiring-check",
      image_requests: candidate.catalog.images
        ? Object.keys(candidate.catalog.images).length
        : 0,
      max_active_images: controlled.observations.max_active_images,
      requests: controlled.observations.requests,
      retained_paths: [...candidate.retained.keys()],
    };
  } finally {
    await controlled.close();
  }
}

export async function selfTestBrowserProbeServer(wasmPath) {
  const controlled = await startBrowserProbeServer(wasmPath);
  try {
    const successIndex = await fetch(controlled.cases.success);
    assert.equal(successIndex.status, 200);
    const cookie = successIndex.headers.get("set-cookie");
    assert.match(cookie, /^e17_probe=success;/);
    const html = await successIndex.text();
    assert.match(html, /open_rebellion_encyclopedia_fetch_start/);
    assert.match(html, /open_rebellion_interface_fixture_emit/);
    assert.match(html, /load\("open-rebellion-test\.wasm"\)/);

    const wasm = await fetch(`${controlled.origin}/open-rebellion-test.wasm`);
    assert.equal(wasm.status, 200);
    assert.equal(digest(Buffer.from(await wasm.arrayBuffer())), controlled.artifact.sha256);
    assert.equal(wasm.headers.get("content-type"), "application/wasm");

    const successHeaders = { cookie };
    for (const relative of fixtureFiles().keys()) {
      const response = await fetch(`${controlled.origin}${loosePrefix}${relative}`, {
        headers: successHeaders,
      });
      assert.equal(response.status, 200, relative);
      assert.deepEqual(Buffer.from(await response.arrayBuffer()), fixtureFiles().get(relative));
    }

    for (const [caseName, expected] of [
      ["both_missing", { catalog: 404, manifest: 404 }],
      ["partial_metadata", { catalog: 200, manifest: 404 }],
      ["malformed_catalog", { catalog: 200, manifest: 200 }],
    ]) {
      const index = await fetch(controlled.cases[caseName]);
      const caseCookie = index.headers.get("set-cookie");
      assert.equal(index.status, 200);
      const catalog = await fetch(`${controlled.origin}${loosePrefix}catalog.json`, {
        headers: { cookie: caseCookie },
      });
      const manifest = await fetch(`${controlled.origin}${loosePrefix}manifest.json`, {
        headers: { cookie: caseCookie },
      });
      assert.equal(catalog.status, expected.catalog, `${caseName} catalog status`);
      assert.equal(manifest.status, expected.manifest, `${caseName} manifest status`);
      if (caseName === "malformed_catalog") {
        assert.equal(await catalog.text(), "<!doctype html>");
      }
    }

    for (const [caseName, expectedStatus, expectedLength] of [
      ["failed_image", 503, Buffer.byteLength("synthetic image failure")],
      ["oversized_image", 200, fixtureFiles().get("assets/EDATA.002").length + 1],
      ["dat_mismatch", 200, fixtureFiles().get("assets/EDATA.002").length],
    ]) {
      const index = await fetch(controlled.cases[caseName]);
      const response = await fetch(
        `${controlled.origin}${loosePrefix}assets/EDATA.002`,
        { headers: { cookie: index.headers.get("set-cookie") } },
      );
      assert.equal(response.status, expectedStatus, `${caseName} art status`);
      assert.equal((await response.arrayBuffer()).byteLength, expectedLength);
    }

    const mainSource = fs.readFileSync(mainSourcePath, "utf8");
    const probeHook = mainSource.indexOf("browser_probe_requested()");
    const assetLoad = mainSource.indexOf("let (mut world, mut browser_audio_files");
    assert.ok(probeHook >= 0 && probeHook < assetLoad);
    const looseSource = fs.readFileSync(looseSourcePath, "utf8");
    assert.match(
      looseSource,
      /run_browser_probe[\s\S]*prepare_browser_loose_encyclopedia\(&selected_dats\)\.await/,
      "feature probe must invoke the actual production browser wrapper",
    );
    assert.match(looseSource, /"published": false/);

    return {
      artifact: controlled.artifact,
      cases: controlled.cases,
      cleanup: "pending",
      live_wasm_execution: "coordinator-browser-gate-required",
      observations: controlled.observations,
      scope: "controlled-probe-server-and-artifact-routing-only",
    };
  } finally {
    await controlled.close();
  }
}

async function waitForTermination() {
  await new Promise(resolve => {
    process.once("SIGINT", resolve);
    process.once("SIGTERM", resolve);
  });
}

async function main() {
  if (process.argv.includes("--help")) {
    console.log(
      "Usage:\n"
      + "  node tools/interface-parity/encyclopedia-loose.mjs --self-test\n"
      + "  node tools/interface-parity/encyclopedia-loose.mjs --self-test-probe-server <wasm>\n"
      + "  node tools/interface-parity/encyclopedia-loose.mjs --serve-wasm-probe <wasm>\n"
      + "The first self-test is an independent JS transport model plus static wiring check; "
      + "it is not Rust WASM execution. The probe server retains the production E15 bridge "
      + "and serves the feature-only Rust artifact for the coordinator browser gate.",
    );
    return;
  }
  const probeSelfTest = process.argv.indexOf("--self-test-probe-server");
  if (probeSelfTest >= 0) {
    const wasmPath = process.argv[probeSelfTest + 1];
    assert.ok(wasmPath, "--self-test-probe-server requires a WASM artifact path");
    const result = await selfTestBrowserProbeServer(wasmPath);
    result.cleanup = "closed";
    console.log(JSON.stringify(result));
    return;
  }
  const serveProbe = process.argv.indexOf("--serve-wasm-probe");
  if (serveProbe >= 0) {
    const wasmPath = process.argv[serveProbe + 1];
    assert.ok(wasmPath, "--serve-wasm-probe requires a WASM artifact path");
    const controlled = await startBrowserProbeServer(wasmPath);
    console.log(JSON.stringify({
      artifact: controlled.artifact,
      cases: controlled.cases,
      live_wasm_execution: "ready-for-coordinator-browser-gate",
      origin: controlled.origin,
    }));
    await waitForTermination();
    await controlled.close();
    return;
  }
  const result = await selfTestLooseEncyclopediaHarness();
  result.cleanup = "closed";
  console.log(JSON.stringify(result));
}

if (path.resolve(process.argv[1] || "") === fileURLToPath(import.meta.url)) {
  main().catch(error => {
    console.error(error.stack || error);
    process.exitCode = 1;
  });
}
