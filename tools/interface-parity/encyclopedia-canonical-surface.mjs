#!/usr/bin/env node

import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import fs from "node:fs";
import http from "node:http";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright-core";
import { PNG } from "pngjs";
import { launchBrowser } from "./browser-launch.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const site = path.join(root, ".artifacts/interface-parity/site");
const chromeSource = path.join(root, "data/base/ui/strategy-dll/BMP");
const sourcePath = process.env.REBELLION_ENCYCLOPEDIA_TEST_SOURCE
  || path.join(root, "data/base/encyclopedia/source.json");
const edata = process.env.REBELLION_EDATA_DIR;
const executable = process.env.OPEN_REBELLION_CHROME_FOR_TESTING || chromium.executablePath();
const launchArguments = ["--mute-audio", "--disable-background-networking", "--no-first-run"];
const expectedRequests = ["/", "/data/runtime.orpk", "/gl.js", "/open-rebellion-test.wasm"];
const expectedFingerprint = "20c342868cee50e80ef3b94b9f81a898c48f4b593ddd0d83ab69b67d755ae9ea";
const expectedCategories = [
  { command_id: 0x6f, label_resource_id: 0x1850 },
  { command_id: 0x70, label_resource_id: 0x1855 },
  { command_id: 0x71, label_resource_id: 0x1854 },
  { command_id: 0x72, label_resource_id: 0x1852 },
  { command_id: 0x73, label_resource_id: 0x1851 },
  { command_id: 0x74, label_resource_id: 0x1856 },
  { command_id: 0x75, label_resource_id: 0x1853 },
];
const starts = {
  index: 59,
  first: 60,
  last: 61,
  longest: 62,
  contextual: 64,
};
const factions = [
  {
    name: "alliance",
    byte: 1,
    nativeOrigin: { x: 62, y: 50 },
    indexResource: 10372,
    indexOrigin: { x: 423, y: 147 },
    closeResource: 10370,
    closeOrigin: { x: 423, y: 25 },
  },
  {
    name: "empire",
    byte: 2,
    nativeOrigin: { x: 125, y: 52 },
    indexResource: 10378,
    indexOrigin: { x: 426, y: 143 },
    closeResource: 10376,
    closeOrigin: { x: 426, y: 21 },
  },
];
const runId = `${new Date().toISOString().replace(/[:.]/g, "-")}-${process.pid}`;
const runDir = path.join(root, ".artifacts/interface-parity", `encyclopedia-canonical-surface-${runId}`);

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

function fixtureCode(scenario, factionByte) {
  return (scenario + 1) | (factionByte << 8);
}

function mimeType(file) {
  if (file.endsWith(".html")) return "text/html; charset=utf-8";
  if (file.endsWith(".js")) return "text/javascript; charset=utf-8";
  if (file.endsWith(".wasm")) return "application/wasm";
  return "application/octet-stream";
}

function readOwnedSource() {
  assert.ok(fs.existsSync(sourcePath), `owned ignored source is missing: ${sourcePath}`);
  assert.ok(edata && fs.existsSync(edata), "set REBELLION_EDATA_DIR to the owned EData directory");
  const bytes = fs.readFileSync(sourcePath);
  return { bytes, source: JSON.parse(bytes) };
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

function decodeIndexedBmp(bytes) {
  assert.equal(bytes.toString("ascii", 0, 2), "BM");
  const dataOffset = bytes.readUInt32LE(10);
  const headerSize = bytes.readUInt32LE(14);
  const width = bytes.readInt32LE(18);
  const signedHeight = bytes.readInt32LE(22);
  const bitsPerPixel = bytes.readUInt16LE(28);
  assert.equal(bitsPerPixel, 8);
  const height = Math.abs(signedHeight);
  const paletteOffset = 14 + headerSize;
  const stride = (width + 3) & ~3;
  const png = new PNG({ width, height });
  for (let y = 0; y < height; y += 1) {
    const sourceY = signedHeight > 0 ? height - 1 - y : y;
    for (let x = 0; x < width; x += 1) {
      const palette = paletteOffset + bytes[dataOffset + sourceY * stride + x] * 4;
      const destination = (y * width + x) * 4;
      png.data.set([bytes[palette + 2], bytes[palette + 1], bytes[palette], 255], destination);
    }
  }
  return { png, bytes, dataOffset, stride, signedHeight };
}

function opaqueProbe(resourceId) {
  const decoded = decodeIndexedBmp(fs.readFileSync(path.join(chromeSource, `${resourceId}.bmp`)));
  const { png, bytes, dataOffset, stride, signedHeight } = decoded;
  const transparent = bytes[dataOffset];
  const candidates = [];
  for (let y = 1; y < png.height - 1; y += 1) {
    const sourceY = signedHeight > 0 ? png.height - 1 - y : y;
    for (let x = 1; x < png.width - 1; x += 1) {
      if (bytes[dataOffset + sourceY * stride + x] !== transparent) {
        candidates.push({
          x,
          y,
          distance: Math.abs(x - png.width / 2) + Math.abs(y - png.height / 2),
        });
      }
    }
  }
  candidates.sort((left, right) => left.distance - right.distance);
  assert.ok(candidates.length > 0, `${resourceId} has no opaque probe`);
  return candidates[0];
}

function compareResource(screenshot, resourceId, x, y, width, height) {
  const decoded = decodeIndexedBmp(fs.readFileSync(path.join(chromeSource, `${resourceId}.bmp`)));
  const { png: expected, bytes, dataOffset, stride, signedHeight } = decoded;
  const transparent = bytes[dataOffset];
  let different = 0;
  for (let row = 0; row < height; row += 1) {
    const sourceRow = signedHeight > 0 ? expected.height - 1 - row : row;
    for (let column = 0; column < width; column += 1) {
      if (bytes[dataOffset + sourceRow * stride + column] === transparent) continue;
      const actualOffset = ((y + row) * screenshot.width + x + column) * 4;
      const expectedOffset = (row * expected.width + column) * 4;
      if (screenshot.data[actualOffset] !== expected.data[expectedOffset]
        || screenshot.data[actualOffset + 1] !== expected.data[expectedOffset + 1]
        || screenshot.data[actualOffset + 2] !== expected.data[expectedOffset + 2]) different += 1;
    }
  }
  return different;
}

function compareArtwork(screenshot, observation, origin, scale, owned) {
  if (!observation.artwork || scale !== 1) return null;
  const filename = observation.artwork.filename;
  assert.equal(owned.source.artwork[String(observation.artwork.resource_id)], filename);
  const bytes = fs.readFileSync(path.join(edata, filename));
  assert.equal(sha256(bytes), observation.artwork.sha256);
  const expected = decodeIndexedBmp(bytes).png;
  assert.equal(expected.width, observation.artwork.width);
  assert.equal(expected.height, observation.artwork.height);
  let different = 0;
  const startX = origin.x + 12;
  const startY = origin.y + 31;
  for (let y = 0; y < expected.height; y += 1) {
    for (let x = 0; x < expected.width; x += 1) {
      const actualOffset = ((startY + y) * screenshot.width + startX + x) * 4;
      const expectedOffset = (y * expected.width + x) * 4;
      if (screenshot.data[actualOffset] !== expected.data[expectedOffset]
        || screenshot.data[actualOffset + 1] !== expected.data[expectedOffset + 1]
        || screenshot.data[actualOffset + 2] !== expected.data[expectedOffset + 2]) different += 1;
    }
  }
  assert.equal(different, 0, `${filename} pixels differ from the owned source bitmap`);
  return { filename, sha256: observation.artwork.sha256, different_pixels: different };
}

async function stableScreenshot(page, directory, label) {
  await page.waitForTimeout(80);
  await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  const first = await page.screenshot({ animations: "disabled" });
  await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  const second = await page.screenshot({ animations: "disabled" });
  assert.equal(sha256(first), sha256(second), `${label} did not stabilize`);
  fs.writeFileSync(path.join(directory, `${label}.png`), second);
  return { sha256: sha256(second), png: PNG.sync.read(second) };
}

async function observations(page) {
  return page.evaluate(() => window.__openRebellionEncyclopediaSurfaces || []);
}

async function waitForObservation(page, expected, after = 0) {
  await page.waitForFunction(
    ({ wanted, minimum }) => {
      const reports = window.__openRebellionEncyclopediaSurfaces || [];
      if (reports.length <= minimum) return false;
      const latest = reports.at(-1);
      return Object.entries(wanted).every(([key, value]) => latest[key] === value);
    },
    { wanted: expected, minimum: after },
    { timeout: 30_000 },
  );
  return (await observations(page)).at(-1);
}

function assertCanonicalObservation(observation, code, faction) {
  assert.equal(observation.schema_version, 1);
  assert.equal(observation.status, "encyclopedia-surface");
  assert.equal(observation.code, code);
  assert.equal(observation.audience, faction.name);
  assert.equal(observation.logical_fingerprint, expectedFingerprint);
  assert.ok(observation.texture_generation > 0);
  assert.deepEqual(
    observation.categories.map(({ command_id, label_resource_id }) => ({ command_id, label_resource_id })),
    expectedCategories,
  );
  assert.equal(observation.categories[0].topic_count, 346);
  assert.equal(observation.resolved_count + observation.source_unavailable_count, observation.topic_count);
  assert.equal(observation.source_unavailable_count, 0);
  assert.ok(!Object.hasOwn(observation, "title"));
  assert.ok(!Object.hasOwn(observation, "description"));
}

async function pressAndWait(page, key, expected) {
  const before = (await observations(page)).length;
  await page.keyboard.press(key);
  return waitForObservation(page, expected, before);
}

async function clickIndexControl(page, faction, origin, scale) {
  const probe = opaqueProbe(faction.indexResource);
  const before = (await observations(page)).length;
  await page.mouse.click(
    origin.x + (faction.indexOrigin.x + probe.x) * scale,
    origin.y + (faction.indexOrigin.y + probe.y) * scale,
  );
  return waitForObservation(page, { mode: "index" }, before);
}

async function clickCloseControl(page, faction, origin, scale) {
  const probe = opaqueProbe(faction.closeResource);
  const before = (await observations(page)).length;
  await page.mouse.click(
    origin.x + (faction.closeOrigin.x + probe.x) * scale,
    origin.y + (faction.closeOrigin.y + probe.y) * scale,
  );
  return waitForObservation(page, { open: false }, before);
}

async function runIndexJourney(page, directory, faction, origin, scale, initial, owned) {
  const screenshots = [];
  const categories = [];
  let observation = initial;
  for (let index = 0; index < expectedCategories.length; index += 1) {
    if (index > 0) {
      observation = await pressAndWait(page, "ArrowRight", {
        mode: "index",
        category_command: expectedCategories[index].command_id,
      });
    }
    assert.equal(observation.topic_count, observation.categories[index].topic_count);
    const shot = await stableScreenshot(page, directory, `category-${observation.category_command.toString(16)}`);
    screenshots.push(shot.sha256);
    categories.push({
      command_id: observation.category_command,
      label_resource_id: observation.category_label_resource_id,
      topic_count: observation.topic_count,
      screenshot_sha256: shot.sha256,
    });
  }
  assert.equal(categories.slice(1).reduce((sum, category) => sum + category.topic_count, 0), 346);
  observation = await pressAndWait(page, "ArrowRight", { mode: "index", category_command: 0x6f });

  const last = await pressAndWait(page, "End", { mode: "index" });
  assert.ok(last.selected_object_id !== null);
  assert.ok(last.index_scroll_row > 0);
  const lastShot = await stableScreenshot(page, directory, "selection-last");
  const first = await pressAndWait(page, "Home", { mode: "index", index_scroll_row: 0 });
  assert.ok(first.selected_object_id !== null);
  assert.notEqual(first.selected_object_id, last.selected_object_id);
  const firstShot = await stableScreenshot(page, directory, "selection-first");

  const topic = await pressAndWait(page, "Enter", { mode: "topic", open: true });
  assert.equal(topic.active_object_id, first.selected_object_id);
  assert.equal(topic.availability, "resolved");
  const topicShot = await stableScreenshot(page, directory, "selection-opened");
  const artwork = compareArtwork(topicShot.png, topic, origin, scale, owned);
  const indexed = await clickIndexControl(page, faction, origin, scale);
  assert.equal(indexed.selected_object_id, topic.selected_object_id);
  const indexShot = await stableScreenshot(page, directory, "topic-returned-to-index");
  const closed = await clickCloseControl(page, faction, origin, scale);
  const closedShot = await stableScreenshot(page, directory, "closed-returned-to-cockpit");
  assert.notEqual(closedShot.sha256, indexShot.sha256);
  return {
    categories,
    selection: {
      first_object_id: first.selected_object_id,
      last_object_id: last.selected_object_id,
      first_screenshot_sha256: firstShot.sha256,
      last_screenshot_sha256: lastShot.sha256,
    },
    opened_artwork: artwork,
    returned_selected_object_id: indexed.selected_object_id,
    close_observation: closed,
    screenshot_hashes: screenshots,
  };
}

async function runEndpointJourney(page, directory, startName, origin, scale, initial, owned) {
  const initialShot = await stableScreenshot(page, directory, "initial");
  const artwork = compareArtwork(initialShot.png, initial, origin, scale, owned);
  const before = (await observations(page)).length;
  const key = startName === "first" ? "ArrowLeft" : "ArrowRight";
  if (startName === "first") assert.equal(initial.previous_object_id, null);
  else assert.equal(initial.next_object_id, null);
  await page.keyboard.press(key);
  await page.waitForTimeout(150);
  assert.equal((await observations(page)).length, before, `${startName} endpoint emitted a wrapped state`);
  const retained = await stableScreenshot(page, directory, `${startName}-endpoint-retained`);
  assert.equal(retained.sha256, initialShot.sha256, `${startName} endpoint wrapped visually`);
  if (scale === 1) {
    const resource = startName === "first" ? 10387 : 10384;
    const x = origin.x + (startName === "first" ? 28 : 380);
    assert.equal(compareResource(initialShot.png, resource, x, origin.y + 14, 21, 17), 0);
  }
  return { endpoint_screenshot_sha256: retained.sha256, artwork };
}

async function runLongestJourney(page, directory, faction, origin, scale, initial, owned) {
  assert.equal(initial.availability, "resolved");
  assert.ok(initial.body_utf8_bytes > 200);
  const initialShot = await stableScreenshot(page, directory, "initial");
  const artwork = compareArtwork(initialShot.png, initial, origin, scale, owned);
  await page.keyboard.press("PageDown");
  const scrolled = await stableScreenshot(page, directory, "body-scrolled");
  assert.notEqual(scrolled.sha256, initialShot.sha256, "longest canonical body did not scroll");

  const forward = initial.next_object_id !== null;
  const destination = forward ? initial.next_object_id : initial.previous_object_id;
  assert.ok(destination !== null);
  const adjacent = await pressAndWait(page, forward ? "ArrowRight" : "ArrowLeft", {
    mode: "topic",
    active_object_id: destination,
  });
  const adjacentShot = await stableScreenshot(page, directory, "adjacent-topic");
  const adjacentArtwork = compareArtwork(adjacentShot.png, adjacent, origin, scale, owned);
  const restored = await pressAndWait(page, forward ? "ArrowLeft" : "ArrowRight", {
    mode: "topic",
    active_object_id: initial.active_object_id,
  });
  const restoredShot = await stableScreenshot(page, directory, "longest-restored");
  assert.equal(restored.body_utf8_bytes, initial.body_utf8_bytes);
  assert.equal(restoredShot.sha256, initialShot.sha256, "returning to the topic did not reset body scroll");
  const indexed = await clickIndexControl(page, faction, origin, scale);
  assert.equal(indexed.selected_object_id, initial.selected_object_id);
  return {
    body_utf8_bytes: initial.body_utf8_bytes,
    initial_screenshot_sha256: initialShot.sha256,
    scrolled_screenshot_sha256: scrolled.sha256,
    restored_screenshot_sha256: restoredShot.sha256,
    artwork,
    adjacent_artwork: adjacentArtwork,
  };
}

async function runContextualJourney(page, directory, faction, origin, scale, initial, owned) {
  assert.equal(initial.return_route.kind, "contextual");
  assert.equal(initial.return_route.audience, faction.name);
  assert.equal(initial.return_route.requested_object_id, initial.active_object_id);
  assert.equal(initial.return_route.caller, "handler-00438800-command-67");
  const initialShot = await stableScreenshot(page, directory, "contextual-initial");
  const artwork = compareArtwork(initialShot.png, initial, origin, scale, owned);
  const adjacent = await pressAndWait(page, "ArrowRight", {
    mode: "topic",
    active_object_id: initial.next_object_id,
  });
  assert.equal(adjacent.return_route.requested_object_id, initial.active_object_id);
  const adjacentShot = await stableScreenshot(page, directory, "contextual-adjacent");
  const adjacentArtwork = compareArtwork(adjacentShot.png, adjacent, origin, scale, owned);
  const restored = await pressAndWait(page, "ArrowLeft", {
    mode: "topic",
    active_object_id: initial.active_object_id,
  });
  assert.deepEqual(restored.return_route, initial.return_route);
  const restoredShot = await stableScreenshot(page, directory, "contextual-restored");
  assert.equal(restoredShot.sha256, initialShot.sha256);
  const closed = await clickCloseControl(page, faction, origin, scale);
  assert.deepEqual(closed.return_route, initial.return_route);
  const closedShot = await stableScreenshot(page, directory, "contextual-closed");
  assert.notEqual(closedShot.sha256, restoredShot.sha256);
  return {
    requested_object_id: initial.return_route.requested_object_id,
    caller: initial.return_route.caller,
    artwork,
    adjacent_artwork: adjacentArtwork,
    restored_screenshot_sha256: restoredShot.sha256,
    closed_screenshot_sha256: closedShot.sha256,
  };
}

async function runCase(browser, server, faction, startName, viewport, owned) {
  const directory = path.join(runDir, faction.name, startName, `${viewport.width}x${viewport.height}`);
  fs.mkdirSync(directory, { recursive: true });
  const code = fixtureCode(starts[startName], faction.byte);
  const requests = [];
  const errors = [];
  const consoleLines = [];
  const context = await browser.newContext({ viewport, deviceScaleFactor: 1, reducedMotion: "reduce" });
  const page = await context.newPage();
  const originUrl = `http://127.0.0.1:${server.address().port}`;
  page.on("response", (response) => {
    if (new URL(response.url()).origin === originUrl) {
      requests.push({ path: new URL(response.url()).pathname, status: response.status() });
    }
  });
  page.on("requestfailed", (request) => errors.push(`request:${request.url()}:${request.failure()?.errorText}`));
  page.on("pageerror", (error) => errors.push(`page:${error.stack || error.message}`));
  page.on("console", (message) => {
    consoleLines.push({ type: message.type(), text: message.text() });
    if (message.type() === "error") errors.push(`console:${message.text()}`);
  });
  try {
    await page.goto(`${originUrl}/?fixture-code=${code}`, { waitUntil: "load", timeout: 30_000 });
    await page.waitForFunction(() => window.__openRebellionInterfaceReady?.status, null, { timeout: 60_000 });
    const ready = await page.evaluate(() => window.__openRebellionInterfaceReady);
    assert.equal(ready.status, "ready", JSON.stringify(ready));
    assert.equal(ready.code, code);
    const initial = await waitForObservation(page, {
      code,
      audience: faction.name,
      mode: startName === "index" ? "index" : "topic",
      open: true,
    });
    assertCanonicalObservation(initial, code, faction);
    assert.equal(initial.category_command, 0x6f);
    assert.equal(initial.topic_count, 346);
    assert.equal(initial.resolved_count, 346);
    assert.equal(initial.source_unavailable_count, 0);
    assert.deepEqual([...new Set(requests.map(({ path: value }) => value))].sort(), [...expectedRequests].sort());
    assert.equal(requests.length, expectedRequests.length, "unexpected duplicate startup request");
    assert.ok(requests.every(({ status }) => status === 200));
    const startupRequests = requests.length;
    const scale = Math.min(viewport.width / 640, viewport.height / 480);
    const origin = {
      x: Math.round((viewport.width - 640 * scale) / 2 + faction.nativeOrigin.x * scale),
      y: Math.round((viewport.height - 480 * scale) / 2 + faction.nativeOrigin.y * scale),
    };
    let journey;
    if (startName === "index") {
      journey = await runIndexJourney(page, directory, faction, origin, scale, initial, owned);
    } else if (startName === "first" || startName === "last") {
      journey = await runEndpointJourney(page, directory, startName, origin, scale, initial, owned);
    } else if (startName === "longest") {
      journey = await runLongestJourney(page, directory, faction, origin, scale, initial, owned);
    } else {
      journey = await runContextualJourney(page, directory, faction, origin, scale, initial, owned);
    }
    assert.equal(requests.length, startupRequests, "Encyclopedia navigation performed network I/O");
    assert.ok(consoleLines.some(({ text }) => text.includes(
      `[encyclopedia] content_session installed disposition=Installed fingerprint=${expectedFingerprint}`,
    )));
    assert.ok(!consoleLines.some(({ text }) => text.includes("topic artwork unavailable")));
    assert.deepEqual(errors, []);
    return {
      faction: faction.name,
      start: startName,
      viewport,
      code,
      ready,
      initial,
      journey,
      requests,
      navigation_request_delta: requests.length - startupRequests,
      console: consoleLines,
      errors,
    };
  } finally {
    await context.close();
  }
}

async function main() {
  for (const required of ["index.html", "gl.js", "open-rebellion-test.wasm", "data/runtime.orpk"]) {
    assert.ok(fs.existsSync(path.join(site, required)), `fixture site is missing ${required}`);
  }
  assert.ok(fs.existsSync(executable), `browser is missing: ${executable}`);
  const owned = readOwnedSource();
  fs.mkdirSync(runDir, { recursive: true });
  const server = await startServer();
  const launchAttempts = [];
  let browser;
  let cleanup = { browser: "not-started", server: "open" };
  try {
    browser = await launchBrowser(chromium, {
      executablePath: executable,
      headless: true,
      args: launchArguments,
    }, launchAttempts);
    cleanup.browser = "open";
    const browserVersion = browser.version();
    const results = [];
    for (const faction of factions) {
      for (const startName of Object.keys(starts)) {
        results.push(await runCase(browser, server, faction, startName, { width: 640, height: 480 }, owned));
      }
      results.push(await runCase(browser, server, faction, "longest", { width: 800, height: 600 }, owned));
    }
    await browser.close();
    browser = null;
    cleanup.browser = "closed";
    await new Promise((resolve) => server.close(resolve));
    cleanup.server = "closed";
    const summary = {
      schema_version: 1,
      family: "encyclopedia-canonical-surface-a1",
      status: "pass",
      scope: "owned ignored P66A canonical base profile through the packed-browser W2 session and W4 surface; strict original Windows A0, HD, and native mod-overlay precedence are not claimed",
      browser: executable,
      browser_version: browserVersion,
      wasm_sha256: sha256(fs.readFileSync(path.join(site, "open-rebellion-test.wasm"))),
      runtime_pack_sha256: sha256(fs.readFileSync(path.join(site, "data/runtime.orpk"))),
      source_catalog_sha256: sha256(owned.bytes),
      logical_fingerprint: expectedFingerprint,
      cases: results.length,
      category_cells: results
        .filter(({ start, viewport }) => start === "index" && viewport.width === 640)
        .reduce((sum, result) => sum + result.journey.categories.length, 0),
      cleanup,
      launch_attempts: launchAttempts,
      results,
    };
    const summaryPath = path.join(runDir, "summary.json");
    fs.writeFileSync(summaryPath, `${JSON.stringify(summary, null, 2)}\n`);
    console.log(JSON.stringify({
      ...summary,
      summary_sha256: sha256(fs.readFileSync(summaryPath)),
      run_directory: runDir,
    }, null, 2));
  } finally {
    if (browser) {
      await browser.close();
      cleanup.browser = "closed-after-failure";
    }
    if (server.listening) {
      await new Promise((resolve) => server.close(resolve));
      cleanup.server = "closed-after-failure";
    }
  }
}

await main();
