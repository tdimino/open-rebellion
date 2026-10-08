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
const site = path.resolve(
  process.env.OPEN_REBELLION_ENCYCLOPEDIA_SITE || path.join(root, "web"),
);
const sourcePath = process.env.REBELLION_ENCYCLOPEDIA_TEST_SOURCE
  || path.join(root, "data/base/encyclopedia/source.json");
const edata = process.env.REBELLION_EDATA_DIR;
const executable = process.env.OPEN_REBELLION_CHROME_FOR_TESTING || chromium.executablePath();
const expectedRequests = ["/", "/data/runtime.orpk", "/gl.js", "/open-rebellion.wasm"];
const categoryControls = [
  { name: "all", command: 0x6f, x: 36 },
  { name: "systems", command: 0x70, x: 88 },
  { name: "ships", command: 0x71, x: 140 },
  { name: "facilities", command: 0x72, x: 192 },
  { name: "missions", command: 0x73, x: 244 },
  { name: "troops", command: 0x74, x: 296 },
  { name: "personnel", command: 0x75, x: 348 },
];
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

function withoutEncyclopediaNamespace(bytes) {
  assert.equal(bytes.toString("ascii", 0, 4), "ORPK");
  const retained = [];
  let removed = 0;
  let cursor = 12;
  for (let index = 0; index < bytes.readUInt32LE(8); index += 1) {
    const start = cursor;
    const kind = bytes[cursor];
    const keyLength = bytes.readUInt16LE(cursor + 1);
    const dataLength = bytes.readUInt32LE(cursor + 3);
    cursor += 7;
    const key = bytes.toString("utf8", cursor, cursor + keyLength);
    cursor += keyLength + dataLength;
    if (kind === 0 && key.startsWith("encyclopedia/")) removed += 1;
    else retained.push(bytes.subarray(start, cursor));
  }
  assert.ok(removed > 2, "old-pack fixture removes the complete Encyclopedia namespace");
  const header = Buffer.from(bytes.subarray(0, 12));
  header.writeUInt32LE(retained.length, 8);
  return Buffer.concat([header, ...retained]);
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

async function startServer({ oldPack = false } = {}) {
  const server = http.createServer((request, response) => {
    const pathname = new URL(request.url, "http://localhost").pathname;
    const relative = path.posix.normalize(decodeURIComponent(pathname)).replace(/^\/+/, "")
      || "index.html";
    const candidate = path.resolve(site, relative);
    if (!candidate.startsWith(`${site}${path.sep}`)) {
      response.writeHead(403).end();
      return;
    }
    fs.readFile(candidate, (error, sourceBytes) => {
      if (error) {
        response.writeHead(404).end();
        return;
      }
      const bytes = oldPack && relative === "data/runtime.orpk"
        ? withoutEncyclopediaNamespace(sourceBytes)
        : sourceBytes;
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

async function frames(page, count = 6) {
  for (let index = 0; index < count; index += 1) {
    await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(resolve)));
  }
}

async function screenshot(page, directory, name) {
  const file = path.join(directory, `${name}.png`);
  const bytes = await page.screenshot({ path: file, animations: "disabled" });
  return { file: path.relative(root, file), sha256: sha256(bytes) };
}

function observePage(page, origin) {
  const requests = [];
  const failures = [];
  const consoleLines = [];
  page.on("response", (response) => {
    if (new URL(response.url()).origin === origin) {
      requests.push({ path: new URL(response.url()).pathname, status: response.status() });
    }
  });
  page.on("requestfailed", (request) => failures.push(
    `request:${request.url()}:${request.failure()?.errorText}`,
  ));
  page.on("pageerror", (error) => failures.push(`page:${error.stack || error.message}`));
  page.on("console", (message) => {
    consoleLines.push({ type: message.type(), text: message.text() });
  });
  return { requests, failures, consoleLines };
}

function assertFourRequestStartup(requests) {
  assert.equal(requests.length, expectedRequests.length, "unexpected duplicate startup request");
  assert.deepEqual(
    [...new Set(requests.map(({ path: value }) => value))].sort(),
    [...expectedRequests].sort(),
  );
  assert.ok(requests.every(({ status }) => status === 200));
}

async function runFactionJourney(browser, origin, faction, namespace) {
  const directory = path.join(runDir, faction.name);
  fs.mkdirSync(directory, { recursive: true });
  const context = await browser.newContext({
    viewport: { width: 640, height: 480 },
    deviceScaleFactor: 1,
    reducedMotion: "reduce",
  });
  const page = await context.newPage();
  const observed = observePage(page, origin);
  try {
    await page.goto(`${origin}/`, { waitUntil: "load", timeout: 30_000 });
    await waitForLog(observed.consoleLines, "[encyclopedia] content_session installed");
    await waitForLog(observed.consoleLines, "alliance_topics=346 empire_topics=346");
    assert.ok(observed.consoleLines.some(({ text }) => text.includes(
      `encyclopedia_assets=${namespace.artwork_count}`,
    )));
    assert.ok(observed.consoleLines.some(({ text }) => text.includes(
      "fingerprint=5c4b64bfd739508e63a87118fd7cac8503ea2d34999144838074a52736b00fe3",
    )));
    assertFourRequestStartup(observed.requests);
    assert.equal(await page.evaluate(() => typeof window.__openRebellionInterfaceReady), "undefined");
    await page.waitForTimeout(12_000);
    await page.mouse.click(faction.menu.x, faction.menu.y, { delay: 250 });
    await waitForLog(observed.consoleLines, `[campaign] faction=${faction.campaign}`);
    await frames(page, 8);
    const campaign = await screenshot(page, directory, "01-campaign");

    await page.keyboard.press("F7");
    await waitForLog(
      observed.consoleLines,
      "command=0x131 destination=encyclopedia status=opened_original",
    );
    await frames(page, 8);
    const index = await screenshot(page, directory, "02-index");
    assert.notEqual(index.sha256, campaign.sha256, "F7 replaces the command center with the index");

    const categories = { all: index };
    const encyclopediaOrigin = faction.name === "alliance"
      ? { x: 62, y: 50 }
      : { x: 125, y: 52 };
    for (const category of categoryControls.slice(1)) {
      await page.mouse.click(
        encyclopediaOrigin.x + category.x + 24,
        encyclopediaOrigin.y + 78 + 20,
        { delay: 150 },
      );
      await frames(page, 6);
      categories[category.name] = await screenshot(
        page,
        directory,
        `02-category-${category.command.toString(16)}`,
      );
      assert.notEqual(
        categories[category.name].sha256,
        index.sha256,
        `${category.name} category changes the production index`,
      );
    }
    const all = categoryControls[0];
    await page.mouse.click(
      encyclopediaOrigin.x + all.x + 24,
      encyclopediaOrigin.y + 78 + 20,
      { delay: 150 },
    );
    await page.mouse.move(8, 8);
    await page.keyboard.press("Home");
    await frames(page, 6);
    await page.keyboard.press("Enter");
    await frames(page, 8);
    const topic = await screenshot(page, directory, "03-topic");
    assert.notEqual(topic.sha256, index.sha256, "Enter opens the selected topic");

    await page.keyboard.press("ArrowRight");
    await frames(page, 8);
    const next = await screenshot(page, directory, "04-next-topic");
    assert.notEqual(next.sha256, topic.sha256, "Right opens the adjacent topic");

    await page.keyboard.press("Escape");
    await waitForLog(observed.consoleLines, "production_route status=closed return=Cockpit");
    await frames(page, 8);
    const returned = await screenshot(page, directory, "05-returned-campaign");
    assert.notEqual(returned.sha256, next.sha256, "Escape returns to the command center");

    const opens = observed.consoleLines.filter(({ text }) => text.includes(
      "command=0x131 destination=encyclopedia status=opened_original",
    )).length;
    await page.keyboard.press("F7");
    await waitForLog(observed.consoleLines, "command=0x131 destination=encyclopedia status=opened_original");
    await frames(page, 4);
    assert.equal(observed.consoleLines.filter(({ text }) => text.includes(
      "command=0x131 destination=encyclopedia status=opened_original",
    )).length, opens + 1, "returned campaign accepts a second production entry");
    const ships = categoryControls.find(({ command }) => command === 0x71);
    assert.ok(ships);
    await page.mouse.click(
      encyclopediaOrigin.x + ships.x + 24,
      encyclopediaOrigin.y + 78 + 20,
      { delay: 150 },
    );
    await page.mouse.move(8, 8);
    await page.keyboard.press("End");
    await page.keyboard.press("Enter");
    await frames(page, 8);
    const lastEndpoint = await screenshot(page, directory, "05-last-ship-endpoint");
    assert.notEqual(
      lastEndpoint.sha256,
      index.sha256,
      "the last ship endpoint opens through the production index",
    );
    await page.keyboard.press("Escape");
    await frames(page, 4);

    await page.keyboard.press("F3");
    await waitForLog(
      observed.consoleLines,
      "command=0x12e destination=fleet_finder status=opened_original",
    );
    await frames(page, 6);
    const finderOrigin = faction.name === "alliance" ? { x: 62.5, y: 50 } : { x: 125, y: 52.5 };
    await page.mouse.click(finderOrigin.x + 200, finderOrigin.y + 148, { delay: 150 });
    const display = faction.name === "alliance"
      ? { x: finderOrigin.x + 439, y: finderOrigin.y + 108 }
      : { x: finderOrigin.x + 448, y: finderOrigin.y + 109.5 };
    await page.mouse.click(display.x, display.y, { delay: 150 });
    await frames(page, 8);
    const fleetWindow = await screenshot(page, directory, "06-fleet-window");
    // The recovered sector placement opens a first right-half sector in the
    // secondary column. Sumitra therefore clamps Yavin's Fleet window against
    // the galaxy view's right edge; the Empire fixture remains in its original
    // column.
    const ship = faction.name === "alliance" ? { x: 444, y: 235 } : { x: 379, y: 241 };
    await page.mouse.click(ship.x, ship.y, { button: "right", delay: 150 });
    await frames(page, 6);
    const objectMenu = await screenshot(page, directory, "07-object-menu");
    const encyclopediaRow = faction.name === "alliance"
      // The menu opens left of the click when the right-column window leaves
      // too little room to its right.
      ? { x: 385, y: 327 }
      : { x: 434, y: 333 };
    await page.mouse.click(encyclopediaRow.x, encyclopediaRow.y, { delay: 150 });
    await waitForLog(
      observed.consoleLines,
      "production_route status=opened origin=contextual caller=FUN_00486fb0_event_0x100",
    );
    await frames(page, 8);
    const contextualTopic = await screenshot(page, directory, "08-contextual-topic");
    assert.notEqual(
      contextualTopic.sha256,
      objectMenu.sha256,
      "the object popup enters the matched Encyclopedia topic",
    );
    await page.keyboard.press("ArrowRight");
    await frames(page, 8);
    const contextualNext = await screenshot(page, directory, "09-contextual-next-topic");
    assert.notEqual(
      contextualNext.sha256,
      contextualTopic.sha256,
      "the contextual journey retains adjacent-topic navigation",
    );
    await page.keyboard.press("Escape");
    await waitForLog(observed.consoleLines, "production_route status=closed return=Contextual");
    await frames(page, 8);
    const contextualReturn = await screenshot(page, directory, "10-contextual-return");
    assert.notEqual(
      contextualReturn.sha256,
      contextualNext.sha256,
      "the contextual return restores the fleet window",
    );

    assert.deepEqual(observed.failures, []);
    assert.deepEqual(
      observed.consoleLines.filter(({ type }) => type === "error"),
      [],
      "canonical production journey has no console errors",
    );
    return {
      faction: faction.name,
      status: "pass",
      screenshots: {
        campaign,
        index,
        categories,
        topic,
        next,
        returned,
        fleetWindow,
        objectMenu,
        contextualTopic,
        contextualNext,
        contextualReturn,
        lastEndpoint,
      },
      requests: observed.requests,
      console: observed.consoleLines,
      failures: observed.failures,
    };
  } finally {
    await context.close();
  }
}

async function runOldPackJourney(browser) {
  const server = await startServer({ oldPack: true });
  const origin = `http://127.0.0.1:${server.address().port}`;
  const context = await browser.newContext({
    viewport: { width: 640, height: 480 },
    deviceScaleFactor: 1,
    reducedMotion: "reduce",
  });
  const page = await context.newPage();
  const observed = observePage(page, origin);
  try {
    await page.goto(`${origin}/`, { waitUntil: "load", timeout: 30_000 });
    await waitForLog(
      observed.consoleLines,
      "content_session unavailable; production route remains disabled",
    );
    assertFourRequestStartup(observed.requests);
    assert.equal(await page.evaluate(() => typeof window.__openRebellionInterfaceReady), "undefined");
    await page.waitForTimeout(12_000);
    await page.mouse.click(472, 333, { delay: 250 });
    await waitForLog(observed.consoleLines, "[campaign] faction=Rebel Alliance");
    await page.keyboard.press("F7");
    await waitForLog(
      observed.consoleLines,
      "command=0x131 destination=encyclopedia status=unavailable",
    );
    const unavailable = await screenshot(page, runDir, "old-pack-unavailable");
    assert.deepEqual(observed.failures, []);
    assert.deepEqual(
      observed.consoleLines.filter(({ type }) => type === "error"),
      [],
      "an old pack without the namespace is unavailable, not corrupt",
    );
    return {
      status: "pass",
      screenshot: unavailable,
      requests: observed.requests,
      console: observed.consoleLines,
      failures: observed.failures,
    };
  } finally {
    await context.close();
    await new Promise((resolve) => server.close(resolve));
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
    const origin = `http://127.0.0.1:${server.address().port}`;
    const journeys = [];
    const requestedFaction = process.env.OPEN_REBELLION_ENCYCLOPEDIA_FACTION;
    for (const faction of [
      { name: "alliance", campaign: "Rebel Alliance", menu: { x: 472, y: 333 } },
      { name: "empire", campaign: "Galactic Empire", menu: { x: 180, y: 333 } },
    ].filter((faction) => !requestedFaction || faction.name === requestedFaction)) {
      journeys.push(await runFactionJourney(browser, origin, faction, namespace));
    }
    const oldPack = requestedFaction ? { status: "skipped-filtered-run" } : await runOldPackJourney(browser);
    const summary = {
      schema_version: 1,
      family: "encyclopedia-canonical-publication",
      status: "pass",
      scope: "owned canonical namespace, strict production startup, both-faction command 0x131 journeys, exact cockpit return and old-pack unavailable compatibility",
      browser: executable,
      browser_version: browser.version(),
      wasm_sha256: sha256(fs.readFileSync(path.join(site, "open-rebellion.wasm"))),
      ...namespace,
      journeys,
      old_pack: oldPack,
      launch_attempts: launchAttempts,
    };
    fs.writeFileSync(path.join(runDir, "summary.json"), `${JSON.stringify(summary, null, 2)}\n`);
    console.log(JSON.stringify({ ...summary, run_directory: runDir }, null, 2));
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
