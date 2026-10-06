#!/usr/bin/env node

import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import fs from "node:fs";
import http from "node:http";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright-core";
import { launchBrowser } from "./browser-launch.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const site = path.join(root, "web");
const sourcePath = process.env.REBELLION_ENCYCLOPEDIA_TEST_SOURCE
  || path.join(root, "data/base/encyclopedia/source.json");
const edata = process.env.REBELLION_EDATA_DIR;
const executable = process.env.OPEN_REBELLION_CHROME_FOR_TESTING || chromium.executablePath();
const expectedRequests = ["/", "/data/runtime.orpk", "/gl.js", "/open-rebellion.wasm"];
const runId = `${new Date().toISOString().replace(/[:.]/g, "-")}-${process.pid}`;
const runDir = path.join(root, ".artifacts/interface-parity", `encyclopedia-publication-${runId}`);

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

function parsePack(bytes) {
  assert.equal(bytes.toString("ascii", 0, 4), "ORPK");
  assert.equal(bytes.readUInt16LE(4), 3);
  assert.equal(bytes.readUInt16LE(6), 0);
  const count = bytes.readUInt32LE(8);
  const entries = new Map();
  let cursor = 12;
  for (let index = 0; index < count; index += 1) {
    assert.ok(cursor + 7 <= bytes.length, "truncated ORPK entry header");
    const kind = bytes[cursor];
    const keyLength = bytes.readUInt16LE(cursor + 1);
    const dataLength = bytes.readUInt32LE(cursor + 3);
    cursor += 7;
    assert.ok(cursor + keyLength + dataLength <= bytes.length, "truncated ORPK entry");
    const key = bytes.toString("utf8", cursor, cursor + keyLength);
    cursor += keyLength;
    const data = bytes.subarray(cursor, cursor + dataLength);
    cursor += dataLength;
    assert.ok(!entries.has(`${kind}:${key}`), `duplicate ORPK entry ${kind}:${key}`);
    entries.set(`${kind}:${key}`, data);
  }
  assert.equal(cursor, bytes.length, "ORPK has trailing bytes");
  return entries;
}

function inspectCanonicalNamespace() {
  assert.ok(fs.existsSync(sourcePath), `owned ignored source is missing: ${sourcePath}`);
  assert.ok(edata && fs.existsSync(edata), "set REBELLION_EDATA_DIR to the owned EData directory");
  const sourceBytes = fs.readFileSync(sourcePath);
  const manifestBytes = fs.readFileSync(`${sourcePath}.manifest.json`);
  const source = JSON.parse(sourceBytes);
  const expectedAssets = [...new Set(Object.values(source.artwork))].sort();
  const packBytes = fs.readFileSync(path.join(site, "data/runtime.orpk"));
  const entries = parsePack(packBytes);
  const encyclopediaKeys = [...entries.keys()]
    .filter((key) => key.startsWith("0:encyclopedia/"))
    .map((key) => key.slice(2))
    .sort();
  assert.deepEqual(encyclopediaKeys, [
    ...expectedAssets.map((filename) => `encyclopedia/assets/${filename}`),
    "encyclopedia/catalog.json",
    "encyclopedia/manifest.json",
  ].sort());
  assert.deepEqual(entries.get("0:encyclopedia/catalog.json"), sourceBytes);
  assert.deepEqual(entries.get("0:encyclopedia/manifest.json"), manifestBytes);
  for (const filename of expectedAssets) {
    assert.deepEqual(
      entries.get(`0:encyclopedia/assets/${filename}`),
      fs.readFileSync(path.join(edata, filename)),
      `packed artwork differs from owned ${filename}`,
    );
  }
  return {
    runtime_pack_sha256: sha256(packBytes),
    runtime_pack_bytes: packBytes.length,
    catalog_sha256: sha256(sourceBytes),
    manifest_sha256: sha256(manifestBytes),
    artwork_count: expectedAssets.length,
  };
}

function mimeType(file) {
  if (file.endsWith(".html")) return "text/html; charset=utf-8";
  if (file.endsWith(".js")) return "text/javascript; charset=utf-8";
  if (file.endsWith(".wasm")) return "application/wasm";
  return "application/octet-stream";
}

async function startServer() {
  const server = http.createServer((request, response) => {
    const pathname = new URL(request.url, "http://localhost").pathname;
    const relative = path.posix.normalize(decodeURIComponent(pathname)).replace(/^\/+/, "")
      || "index.html";
    const candidate = path.resolve(site, relative);
    if (!candidate.startsWith(`${site}${path.sep}`)) {
      response.writeHead(403).end();
      return;
    }
    fs.readFile(candidate, (error, bytes) => {
      if (error) {
        response.writeHead(404).end();
        return;
      }
      response.writeHead(200, {
        "content-type": mimeType(candidate),
        "content-length": bytes.length,
        "cache-control": "no-store",
      }).end(bytes);
    });
  });
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  return server;
}

async function waitForLog(consoleLines, fragment, timeoutMs = 60_000) {
  const deadline = Date.now() + timeoutMs;
  while (!consoleLines.some((line) => line.text.includes(fragment))) {
    if (Date.now() >= deadline) throw new Error(`timed out waiting for console log: ${fragment}`);
    await new Promise((resolve) => setTimeout(resolve, 100));
  }
}

async function main() {
  for (const required of ["index.html", "gl.js", "open-rebellion.wasm", "data/runtime.orpk"]) {
    assert.ok(fs.existsSync(path.join(site, required)), `production site is missing ${required}`);
  }
  const namespace = inspectCanonicalNamespace();
  fs.mkdirSync(runDir, { recursive: true });
  const server = await startServer();
  const launchAttempts = [];
  let browser;
  let cleanup = "not-started";
  try {
    browser = await launchBrowser(chromium, {
      executablePath: executable,
      headless: true,
      args: ["--mute-audio", "--disable-background-networking", "--no-first-run"],
    }, launchAttempts);
    cleanup = "open";
    const context = await browser.newContext({
      viewport: { width: 640, height: 480 },
      deviceScaleFactor: 1,
      reducedMotion: "reduce",
    });
    const page = await context.newPage();
    const requests = [];
    const errors = [];
    const consoleLines = [];
    const origin = `http://127.0.0.1:${server.address().port}`;
    page.on("response", (response) => {
      if (new URL(response.url()).origin === origin) {
        requests.push({ path: new URL(response.url()).pathname, status: response.status() });
      }
    });
    page.on("requestfailed", (request) => errors.push(`request:${request.url()}:${request.failure()?.errorText}`));
    page.on("pageerror", (error) => errors.push(`page:${error.stack || error.message}`));
    page.on("console", (message) => {
      consoleLines.push({ type: message.type(), text: message.text() });
      if (message.type() === "error") errors.push(`console:${message.text()}`);
    });
    await page.goto(`${origin}/`, { waitUntil: "load", timeout: 30_000 });
    await waitForLog(consoleLines, "[encyclopedia] content_session installed");
    await waitForLog(consoleLines, "alliance_topics=356 empire_topics=356");
    assert.ok(consoleLines.some(({ text }) => text.includes(namespace.artwork_count > 0
      ? `encyclopedia_assets=${namespace.artwork_count}`
      : "encyclopedia_assets=0")));
    assert.ok(consoleLines.some(({ text }) => text.includes(
      "fingerprint=5c4b64bfd739508e63a87118fd7cac8503ea2d34999144838074a52736b00fe3",
    )));
    assert.equal(requests.length, expectedRequests.length, "unexpected duplicate startup request");
    assert.deepEqual([...new Set(requests.map(({ path: value }) => value))].sort(),
      [...expectedRequests].sort());
    assert.ok(requests.every(({ status }) => status === 200));
    assert.deepEqual(errors, []);
    assert.equal(await page.evaluate(() => typeof window.__openRebellionInterfaceReady), "undefined");
    await page.screenshot({ path: path.join(runDir, "production-startup.png"), animations: "disabled" });
    const summary = {
      schema_version: 1,
      family: "encyclopedia-canonical-publication",
      status: "pass",
      scope: "owned ignored P66A native/ORPK byte parity and packaged production startup; command 0x131 remains gated",
      browser: executable,
      browser_version: browser.version(),
      wasm_sha256: sha256(fs.readFileSync(path.join(site, "open-rebellion.wasm"))),
      ...namespace,
      requests,
      console: consoleLines,
      errors,
      launch_attempts: launchAttempts,
    };
    fs.writeFileSync(path.join(runDir, "summary.json"), `${JSON.stringify(summary, null, 2)}\n`);
    console.log(JSON.stringify({ ...summary, run_directory: runDir }, null, 2));
    await context.close();
  } finally {
    if (browser) {
      await browser.close();
      cleanup = "closed";
    }
    await new Promise((resolve) => server.close(resolve));
  }
  assert.equal(cleanup, "closed");
}

await main();
