#!/usr/bin/env node

import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import fs from "node:fs";
import http from "node:http";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { gzipSync } from "node:zlib";
import { launchBrowser } from "./browser-launch.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const productionHtmlPath = path.join(root, "web/index.html");
const browserManifestPath = path.join(here, "browser.json");

const STATE_UNKNOWN = -1;
const STATE_PENDING = 0;
const STATE_READY = 1;
const STATE_NOT_FOUND = 2;
const STATE_HTTP_STATUS = 3;
const STATE_TRANSPORT = 4;
const STATE_RESOURCE_LIMIT = 5;

const exactPayload = Uint8Array.from([1, 2, 3, 4]);
const htmlPayload = new TextEncoder().encode("<!doctype html><title>not encyclopedia content</title>");
const compressedPayload = new TextEncoder().encode("A".repeat(128));

const glueStub = `"use strict";
globalThis.wasm_memory = new WebAssembly.Memory({ initial: 2 });
globalThis.wasm_exports = null;
globalThis.__openRebellionPlugins = [];
globalThis.miniquad_add_plugin = function (plugin) {
    globalThis.__openRebellionPlugins.push(plugin.name);
    if (plugin.name === "open_rebellion_encyclopedia_fetch") {
        const imports = { env: {} };
        plugin.register_plugin(imports);
        globalThis.__openRebellionEncyclopediaFetch = imports.env;
    }
};
globalThis.load = function () {};
`;

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

function inlineBridgeCount(html) {
  return [...html.matchAll(/<script(?<attributes>[^>]*)>(?<body>[\s\S]*?)<\/script>/g)]
    .filter(match => match.groups.body.includes("registerEncyclopediaFetchBridge")
      && !/\bsrc\s*=/i.test(match.groups.attributes)).length;
}

function writeJson(destination, value) {
  fs.writeFileSync(destination, `${JSON.stringify(value, null, 2)}\n`);
}

async function waitFor(predicate, description, timeoutMs = 5_000) {
  const deadline = performance.now() + timeoutMs;
  while (performance.now() < deadline) {
    if (predicate()) return;
    await new Promise(resolve => setTimeout(resolve, 10));
  }
  throw new Error(`timed out waiting for ${description}`);
}

export async function startEncyclopediaFetchControlledServer() {
  const productionHtml = fs.readFileSync(productionHtmlPath);
  const observations = { requests: [] };
  const timers = new Set();

  function schedule(response, callback, delayMs) {
    const timer = setTimeout(() => {
      timers.delete(timer);
      callback();
    }, delayMs);
    timers.add(timer);
    response.once("close", () => {
      clearTimeout(timer);
      timers.delete(timer);
    });
  }

  const server = http.createServer((request, response) => {
    const pathname = new URL(request.url, "http://127.0.0.1").pathname;
    const observation = {
      first_chunk_written: false,
      method: request.method,
      path: pathname,
      request_aborted: false,
      response_closed: false,
      response_ended: false,
      status: null,
    };
    observations.requests.push(observation);
    request.once("aborted", () => {
      observation.request_aborted = true;
    });
    response.once("close", () => {
      observation.response_closed = true;
      observation.response_ended = response.writableEnded;
    });

    function writeHead(status, headers = {}) {
      observation.status = status;
      response.writeHead(status, {
        "cache-control": "no-store",
        ...headers,
      });
    }

    switch (pathname) {
      case "/":
        writeHead(200, {
          "content-length": productionHtml.length,
          "content-type": "text/html; charset=utf-8",
        });
        response.end(productionHtml);
        break;
      case "/gl.js":
        writeHead(200, {
          "content-length": Buffer.byteLength(glueStub),
          "content-type": "text/javascript; charset=utf-8",
        });
        response.end(glueStub);
        break;
      case "/content/404":
      case "/content/403":
      case "/content/500": {
        const status = Number(pathname.slice(-3));
        const body = Buffer.from(`controlled HTTP ${status}`);
        writeHead(status, {
          "content-length": body.length,
          "content-type": "text/plain; charset=utf-8",
        });
        response.end(body);
        break;
      }
      case "/content/html":
        writeHead(200, { "content-type": "text/html; charset=utf-8" });
        response.end(htmlPayload);
        break;
      case "/content/exact":
        writeHead(200, { "content-type": "application/octet-stream" });
        observation.first_chunk_written = true;
        response.write(exactPayload.subarray(0, 2));
        schedule(response, () => response.end(exactPayload.subarray(2)), 15);
        break;
      case "/content/compressed": {
        const compressed = gzipSync(compressedPayload);
        assert.notEqual(compressed.length, compressedPayload.length);
        writeHead(200, {
          "content-encoding": "gzip",
          "content-length": compressed.length,
          "content-type": "application/octet-stream",
        });
        response.end(compressed);
        break;
      }
      case "/content/over-limit":
        writeHead(200, { "content-type": "application/octet-stream" });
        observation.first_chunk_written = true;
        response.write(Uint8Array.from([1, 2]));
        schedule(response, () => {
          response.write(Uint8Array.from([3, 4, 5]));
          schedule(response, () => response.end(), 5_000);
        }, 15);
        break;
      case "/content/slow":
        writeHead(200, { "content-type": "application/octet-stream" });
        observation.first_chunk_written = true;
        response.write(Uint8Array.from([9]));
        schedule(response, () => response.end(Uint8Array.from([10])), 5_000);
        break;
      case "/content/stream-failure":
        writeHead(200, { "content-type": "application/octet-stream" });
        observation.first_chunk_written = true;
        response.write(Uint8Array.from([1, 2]));
        schedule(response, () => response.destroy(new Error("controlled stream failure")), 15);
        break;
      case "/content/network-failure":
        observation.response_closed = true;
        request.socket.destroy();
        break;
      default:
        writeHead(404, { "content-length": 0 });
        response.end();
    }
  });
  server.on("clientError", (_error, socket) => socket.destroy());
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
      for (const timer of timers) clearTimeout(timer);
      timers.clear();
      server.closeAllConnections();
      await new Promise(resolve => server.close(resolve));
    },
  };
}

export async function selfTestEncyclopediaFetchBrowserHarness() {
  const controlled = await startEncyclopediaFetchControlledServer();
  let result;
  try {
    const html = await (await fetch(`${controlled.origin}/`)).text();
    const statuses = [];
    for (const status of [404, 403, 500]) {
      statuses.push((await fetch(`${controlled.origin}/content/${status}`)).status);
    }
    const exactResponse = await fetch(`${controlled.origin}/content/exact`);
    assert.equal(exactResponse.headers.get("content-length"), null);
    const exactBytes = [...new Uint8Array(await exactResponse.arrayBuffer())];
    const htmlResponse = await fetch(`${controlled.origin}/content/html`);
    assert.equal(htmlResponse.status, 200);
    assert.deepEqual(new Uint8Array(await htmlResponse.arrayBuffer()), htmlPayload);
    const compressedResponse = await fetch(`${controlled.origin}/content/compressed`);
    const advertisedCompressedBytes = Number(compressedResponse.headers.get("content-length"));
    const decodedCompressedBytes = new Uint8Array(await compressedResponse.arrayBuffer());
    assert.notEqual(advertisedCompressedBytes, decodedCompressedBytes.length);

    let networkFailed = false;
    try {
      await fetch(`${controlled.origin}/content/network-failure`);
    } catch (_) {
      networkFailed = true;
    }
    let streamFailed = false;
    try {
      await (await fetch(`${controlled.origin}/content/stream-failure`)).arrayBuffer();
    } catch (_) {
      streamFailed = true;
    }

    const controller = new AbortController();
    const slowResponse = await fetch(`${controlled.origin}/content/slow`, {
      signal: controller.signal,
    });
    const reader = slowResponse.body.getReader();
    await reader.read();
    controller.abort();
    try {
      await reader.read();
    } catch (_) {
      // An aborted Fetch stream is expected to reject its pending read.
    }
    await waitFor(
      () => controlled.observations.requests.some(request =>
        request.path === "/content/slow"
        && request.response_closed
        && !request.response_ended),
      "the controlled slow response to observe cancellation",
    );
    const slow = controlled.observations.requests.find(request => request.path === "/content/slow");

    result = {
      cleanup: "pending",
      compressed_bytes: decodedCompressedBytes.length,
      exact_bytes: exactBytes,
      html_has_one_inline_bridge: inlineBridgeCount(html) === 1,
      network_failed: networkFailed,
      slow_cancelled: slow.request_aborted || (slow.response_closed && !slow.response_ended),
      statuses,
      stream_failed: streamFailed,
    };
  } finally {
    await controlled.close();
  }
  result.cleanup = "closed";
  return result;
}

function browserExecutable(manifest) {
  const candidates = [
    ...(process.env.OPEN_REBELLION_CHROME_FOR_TESTING
      ? [process.env.OPEN_REBELLION_CHROME_FOR_TESTING]
      : []),
    ...manifest.executable_candidates,
  ];
  const executable = candidates.find(candidate => fs.existsSync(candidate));
  if (!executable) {
    throw new Error(
      `pinned Chrome for Testing ${manifest.version} is missing; set `
      + "OPEN_REBELLION_CHROME_FOR_TESTING",
    );
  }
  const version = spawnSync(executable, ["--version"], { encoding: "utf8" });
  if (version.status !== 0 || !version.stdout.includes(manifest.version)) {
    throw new Error(`Chrome for Testing version mismatch: ${version.stdout || version.stderr}`);
  }
  return { executable, version: version.stdout.trim() };
}

async function invokeBridge(page, fetchPath, maxBytes) {
  return page.evaluate(async ({ fetchPath: browserPath, maxBytes: browserLimit }) => {
    const api = globalThis.__openRebellionEncyclopediaFetch;
    const pathBytes = new TextEncoder().encode(browserPath);
    new Uint8Array(globalThis.wasm_memory.buffer, 0, pathBytes.length).set(pathBytes);
    const handle = api.open_rebellion_encyclopedia_fetch_start(0, pathBytes.length, browserLimit);
    try {
      const deadline = performance.now() + 5_000;
      let state = api.open_rebellion_encyclopedia_fetch_poll(handle);
      while (state === 0 && performance.now() < deadline) {
        await new Promise(resolve => setTimeout(resolve, 10));
        state = api.open_rebellion_encyclopedia_fetch_poll(handle);
      }
      if (state === 0) throw new Error(`bridge request ${handle} did not finish`);
      const detail = api.open_rebellion_encyclopedia_fetch_detail(handle);
      let bytes = [];
      if (state === 1 && detail !== 0) {
        const destination = 4_096;
        if (api.open_rebellion_encyclopedia_fetch_copy(handle, destination, detail) !== 1) {
          throw new Error(`bridge request ${handle} failed to copy`);
        }
        bytes = [...new Uint8Array(globalThis.wasm_memory.buffer, destination, detail)];
      }
      return { bytes, detail, state };
    } finally {
      api.open_rebellion_encyclopedia_fetch_release(handle);
    }
  }, { fetchPath, maxBytes });
}

async function startBridgeRequest(page, fetchPath, maxBytes) {
  return page.evaluate(({ fetchPath: browserPath, maxBytes: browserLimit }) => {
    const pathBytes = new TextEncoder().encode(browserPath);
    new Uint8Array(globalThis.wasm_memory.buffer, 0, pathBytes.length).set(pathBytes);
    return globalThis.__openRebellionEncyclopediaFetch
      .open_rebellion_encyclopedia_fetch_start(0, pathBytes.length, browserLimit);
  }, { fetchPath, maxBytes });
}

async function runBrowserHarness() {
  const manifest = JSON.parse(fs.readFileSync(browserManifestPath, "utf8"));
  assert.ok(manifest.launch_arguments.includes("--mute-audio"), "browser launch must mute audio");
  const browserBinary = browserExecutable(manifest);
  const { chromium } = await import("playwright-core");
  const runId = `${new Date().toISOString().replace(/[:.]/g, "-")}-${process.pid}`;
  const runDirectory = path.join(
    root,
    ".artifacts/interface-parity",
    `encyclopedia-fetch-${runId}`,
  );
  fs.mkdirSync(runDirectory, { recursive: true });

  const controlled = await startEncyclopediaFetchControlledServer();
  const browserRequests = [];
  const consoleLines = [];
  const browserErrors = [];
  const launchAttempts = [];
  let browser;
  let context;
  let page;
  let failure;
  let result = { status: "failed" };
  const cleanup = { browser: "not-started", context: "not-started", server: "open" };
  try {
    browser = await launchBrowser(chromium, {
      args: manifest.launch_arguments,
      executablePath: browserBinary.executable,
      headless: true,
      timeout: 30_000,
    }, launchAttempts);
    cleanup.browser = "open";
    context = await browser.newContext({
      colorScheme: "dark",
      deviceScaleFactor: 1,
      locale: "en-US",
      reducedMotion: "reduce",
      serviceWorkers: "block",
      timezoneId: "America/New_York",
      viewport: { height: 480, width: 640 },
    });
    cleanup.context = "open";
    page = await context.newPage();
    page.on("request", request => browserRequests.push({
      event: "request",
      method: request.method(),
      url: request.url(),
    }));
    page.on("response", response => browserRequests.push({
      event: "response",
      status: response.status(),
      url: response.url(),
    }));
    page.on("requestfailed", request => {
      browserRequests.push({
        error: request.failure()?.errorText,
        event: "requestfailed",
        url: request.url(),
      });
    });
    page.on("console", message => {
      consoleLines.push({ text: message.text(), type: message.type() });
    });
    page.on("pageerror", error => browserErrors.push(String(error.stack || error)));

    await page.goto(`${controlled.origin}/`, { waitUntil: "load", timeout: 30_000 });
    await page.waitForFunction(() => globalThis.__openRebellionEncyclopediaFetch, null, {
      timeout: 5_000,
    });

    const cases = {
      compressed: await invokeBridge(page, `${controlled.origin}/content/compressed`, 128),
      exact: await invokeBridge(page, `${controlled.origin}/content/exact`, 4),
      forbidden: await invokeBridge(page, `${controlled.origin}/content/403`, 32),
      html: await invokeBridge(page, `${controlled.origin}/content/html`, htmlPayload.length),
      network: await invokeBridge(page, `${controlled.origin}/content/network-failure`, 32),
      not_found: await invokeBridge(page, `${controlled.origin}/content/404`, 32),
      over_limit: await invokeBridge(page, `${controlled.origin}/content/over-limit`, 4),
      server_error: await invokeBridge(page, `${controlled.origin}/content/500`, 32),
      stream_failure: await invokeBridge(page, `${controlled.origin}/content/stream-failure`, 32),
    };
    assert.deepEqual(cases.exact, { bytes: [...exactPayload], detail: 4, state: STATE_READY });
    assert.deepEqual(cases.compressed.bytes, [...compressedPayload]);
    assert.deepEqual(cases.html.bytes, [...htmlPayload]);
    assert.deepEqual(cases.not_found, { bytes: [], detail: 404, state: STATE_NOT_FOUND });
    assert.deepEqual(cases.forbidden, { bytes: [], detail: 403, state: STATE_HTTP_STATUS });
    assert.deepEqual(cases.server_error, { bytes: [], detail: 500, state: STATE_HTTP_STATUS });
    assert.equal(cases.network.state, STATE_TRANSPORT);
    assert.equal(cases.stream_failure.state, STATE_TRANSPORT);
    assert.equal(cases.over_limit.state, STATE_RESOURCE_LIMIT);
    assert.deepEqual(browserErrors, [], "controlled browser run emitted uncaught page errors");

    const slowHandle = await startBridgeRequest(page, `${controlled.origin}/content/slow`, 32);
    await waitFor(
      () => controlled.observations.requests.some(request =>
        request.path === "/content/slow" && request.first_chunk_written),
      "the browser slow request to receive its first chunk",
    );
    const released = await page.evaluate(handle => {
      const api = globalThis.__openRebellionEncyclopediaFetch;
      return {
        release: api.open_rebellion_encyclopedia_fetch_release(handle),
        state: api.open_rebellion_encyclopedia_fetch_poll(handle),
      };
    }, slowHandle);
    assert.deepEqual(released, { release: 1, state: STATE_UNKNOWN });
    await waitFor(
      () => controlled.observations.requests.some(request =>
        request.path === "/content/slow"
        && request.response_closed
        && !request.response_ended),
      "the browser slow request cancellation",
    );
    await waitFor(
      () => controlled.observations.requests.some(request =>
        request.path === "/content/over-limit"
        && request.response_closed
        && !request.response_ended),
      "the over-limit browser request cancellation",
    );

    await page.screenshot({ path: path.join(runDirectory, "harness.png") });
    result = {
      bridge_scope: "production inline JavaScript transport; no invented Rust/downstream consumer",
      browser: {
        flags: manifest.launch_arguments,
        manifest_version: manifest.version,
        runtime_version: browser.version(),
        version_command: browserBinary.version,
      },
      cases,
      launch_attempts: launchAttempts,
      production_html_sha256: sha256(fs.readFileSync(productionHtmlPath)),
      status: "passed",
    };
  } catch (error) {
    failure = error;
    result = {
      ...result,
      error: String(error.stack || error),
      launch_attempts: launchAttempts,
      status: "failed",
    };
  } finally {
    if (page) await page.close().catch(() => {});
    if (context) {
      await context.close().catch(() => {});
      cleanup.context = "closed";
    }
    if (browser) {
      await browser.close().catch(() => {});
      cleanup.browser = "closed";
    }
    await controlled.close();
    cleanup.server = "closed";
  }

  result.cleanup = cleanup;
  result.browser_errors = browserErrors;
  writeJson(path.join(runDirectory, "result.json"), result);
  writeJson(path.join(runDirectory, "browser-requests.json"), browserRequests);
  writeJson(path.join(runDirectory, "console.json"), consoleLines);
  writeJson(path.join(runDirectory, "server-observations.json"), controlled.observations);
  const artifactHashes = {};
  for (const name of [
    "result.json",
    "browser-requests.json",
    "console.json",
    "server-observations.json",
    ...(fs.existsSync(path.join(runDirectory, "harness.png")) ? ["harness.png"] : []),
  ]) {
    artifactHashes[name] = sha256(fs.readFileSync(path.join(runDirectory, name)));
  }
  writeJson(path.join(runDirectory, "artifact-hashes.json"), artifactHashes);
  console.log(JSON.stringify({ artifact_hashes: artifactHashes, result, run_directory: runDirectory }));
  if (failure) throw failure;
}

async function main() {
  if (process.argv.includes("--self-test")) {
    console.log(JSON.stringify(await selfTestEncyclopediaFetchBrowserHarness()));
    return;
  }
  if (process.argv.includes("--help")) {
    console.log(
      "Usage: node tools/interface-parity/encyclopedia-fetch-browser.mjs [--self-test]\n"
      + "Set OPEN_REBELLION_CHROME_FOR_TESTING to the pinned browser executable.\n"
      + "This source-free harness tests only the production inline transport bridge.\n"
      + "The separate four-request gate needs an authorized ignored runtime pack, built "
      + "interface fixture site, REBELLION_EDATA_DIR, and then:\n"
      + "  npm --prefix tools/interface-parity run test:encyclopedia-art:no-build",
    );
    return;
  }
  await runBrowserHarness();
}

if (path.resolve(process.argv[1] || "") === fileURLToPath(import.meta.url)) {
  main().catch(error => {
    console.error(error.stack || error);
    process.exitCode = 1;
  });
}
