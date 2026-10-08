#!/usr/bin/env node

import assert from "node:assert/strict";
import { execFileSync, spawnSync } from "node:child_process";
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
const browserManifest = JSON.parse(fs.readFileSync(path.join(here, "browser.json"), "utf8"));
const noBuild = process.argv.includes("--no-build");
const catalogMode = process.argv.includes("--catalog");
const expectedRequests = ["/", "/data/runtime.orpk", "/gl.js", "/open-rebellion-test.wasm"];
const origin = { x: 85, y: 55 };
const dimensions = { width: 470, height: 330 };
const categoryControls = [
  { command: 0x6f, x: 36, y: 78, width: 49, height: 41, shared: [10340, 10339] },
  { command: 0x70, x: 88, y: 78, width: 49, height: 41, shared: [10350, 10349] },
  { command: 0x71, x: 140, y: 78, width: 49, height: 41, alliance: [10348, 10347], empire: [10360, 10359] },
  { command: 0x72, x: 192, y: 78, width: 49, height: 41, alliance: [10344, 10343], empire: [10356, 10355] },
  { command: 0x73, x: 244, y: 78, width: 49, height: 41, alliance: [11616, 11615], empire: [11618, 11617] },
  { command: 0x74, x: 296, y: 78, width: 49, height: 41, alliance: [10352, 10351], empire: [10362, 10361] },
  { command: 0x75, x: 348, y: 78, width: 49, height: 41, alliance: [10346, 10345], empire: [10358, 10357] },
];
const factions = [
  {
    name: "alliance",
    byte: 1,
    base: 10335,
    rail: 10585,
    underlyingEncyclopedia: { x: 408, y: 413 },
    railControls: [
      { command: 0xfb, x: 423, y: 25, width: 32, height: 31, resources: [10370, 10371] },
      { command: 0x67, x: 423, y: 93, width: 32, height: 31, resources: [10374, 10375] },
      { command: 0x68, x: 423, y: 147, width: 32, height: 31, resources: [10372, 10373], selected: true },
    ],
  },
  {
    name: "empire",
    byte: 2,
    base: 10336,
    rail: 10589,
    underlyingEncyclopedia: { x: 483, y: 446 },
    railControls: [
      { command: 0xfb, x: 426, y: 21, width: 44, height: 41, resources: [10376, 10377] },
      { command: 0x67, x: 426, y: 89, width: 44, height: 41, resources: [10380, 10381] },
      { command: 0x68, x: 426, y: 143, width: 44, height: 41, resources: [10378, 10379], selected: true },
    ],
  },
];
const runId = `${new Date().toISOString().replace(/[:.]/g, "-")}-${process.pid}`;
const runDir = path.join(root, ".artifacts/interface-parity", `encyclopedia-index-shell-${runId}`);

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

function sourceDirectory() {
  const candidates = [
    process.env.REBELLION_STRATEGY_BMP_DIR,
    path.join(root, "data/base/ui/strategy-dll/BMP"),
  ].filter(Boolean);
  const directory = candidates.find((candidate) => fs.existsSync(path.join(candidate, "10335.bmp")));
  if (!directory) throw new Error("owned STRATEGY.DLL BMP extraction is unavailable");
  return directory;
}

function browserExecutable() {
  const candidates = [
    ...(process.env.OPEN_REBELLION_CHROME_FOR_TESTING ? [process.env.OPEN_REBELLION_CHROME_FOR_TESTING] : []),
    ...browserManifest.executable_candidates,
  ];
  const executable = candidates.find((candidate) => fs.existsSync(candidate));
  if (!executable) throw new Error(`pinned Chrome for Testing ${browserManifest.version} is missing`);
  const version = spawnSync(executable, ["--version"], { encoding: "utf8" });
  if (version.status !== 0 || !version.stdout.includes(browserManifest.version)) {
    throw new Error(`Chrome for Testing version mismatch: ${version.stdout || version.stderr}`);
  }
  return executable;
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
    const relative = path.posix.normalize(decodeURIComponent(pathname)).replace(/^\/+/, "") || "index.html";
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
  const height = Math.abs(signedHeight);
  assert.equal(bytes.readUInt16LE(28), 8);
  assert.equal(bytes.readUInt32LE(30), 0);
  const paletteOffset = 14 + headerSize;
  const stride = (width + 3) & ~3;
  assert.ok(bytes.length >= dataOffset + stride * height);
  const png = new PNG({ width, height });
  for (let y = 0; y < height; y += 1) {
    const sourceY = signedHeight > 0 ? height - 1 - y : y;
    for (let x = 0; x < width; x += 1) {
      const paletteIndex = bytes[dataOffset + sourceY * stride + x];
      const palette = paletteOffset + paletteIndex * 4;
      const destination = (y * width + x) * 4;
      png.data[destination] = bytes[palette + 2];
      png.data[destination + 1] = bytes[palette + 1];
      png.data[destination + 2] = bytes[palette];
      png.data[destination + 3] = 255;
    }
  }
  return png;
}

function decodeIndexedHitMask(bytes) {
  assert.equal(bytes.toString("ascii", 0, 2), "BM");
  const dataOffset = bytes.readUInt32LE(10);
  const width = bytes.readInt32LE(18);
  const signedHeight = bytes.readInt32LE(22);
  const height = Math.abs(signedHeight);
  assert.ok(width > 0 && height > 0);
  assert.equal(bytes.readUInt16LE(28), 8);
  assert.equal(bytes.readUInt32LE(30), 0);
  const stride = (width + 3) & ~3;
  const transparentIndex = bytes[dataOffset];
  const opaque = new Uint8Array(width * height);
  for (let y = 0; y < height; y += 1) {
    const sourceY = signedHeight > 0 ? height - 1 - y : y;
    for (let x = 0; x < width; x += 1) {
      opaque[y * width + x] = bytes[dataOffset + sourceY * stride + x] === transparentIndex ? 0 : 1;
    }
  }
  return { width, height, opaque };
}

function nearestMaskPoint(source, resourceId, control, wantedOpaque) {
  const mask = decodeIndexedHitMask(fs.readFileSync(path.join(source, `${resourceId}.bmp`)));
  const candidates = [];
  const width = Math.min(mask.width, control.width);
  const height = Math.min(mask.height, control.height);
  for (let y = 1; y < height; y += 1) {
    for (let x = 1; x < width; x += 1) {
      if (Boolean(mask.opaque[y * mask.width + x]) === wantedOpaque) {
        const distance = Math.abs(x - width / 2) + Math.abs(y - height / 2);
        candidates.push({ x, y, distance });
      }
    }
  }
  candidates.sort((left, right) => left.distance - right.distance);
  assert.ok(candidates.length > 0, `${resourceId} has no ${wantedOpaque ? "opaque" : "transparent"} probe pixel`);
  return candidates[0];
}

function resource(source, id) {
  return decodeIndexedBmp(fs.readFileSync(path.join(source, `${id}.bmp`)));
}

function resourcesFor(control, faction) {
  return control.shared || control[faction.name];
}

function blitClipped(destination, source, x, y, width = source.width, height = source.height) {
  const rows = Math.min(height, source.height, destination.height - y);
  const columns = Math.min(width, source.width, destination.width - x);
  for (let row = 0; row < rows; row += 1) {
    const start = row * source.width * 4;
    const end = start + columns * 4;
    destination.data.set(source.data.subarray(start, end), ((y + row) * destination.width + x) * 4);
  }
}

function sourceIdentity(source) {
  const resourceIds = new Set([10338]);
  for (const faction of factions) {
    resourceIds.add(faction.base);
    resourceIds.add(faction.rail);
    for (const control of categoryControls) {
      for (const resourceId of resourcesFor(control, faction)) resourceIds.add(resourceId);
    }
    for (const control of faction.railControls) {
      for (const resourceId of control.resources) resourceIds.add(resourceId);
    }
  }
  const sortedIds = [...resourceIds].sort((left, right) => left - right);
  const aggregate = createHash("sha256");
  for (const resourceId of sortedIds) {
    aggregate.update(`${resourceId}\0`);
    aggregate.update(fs.readFileSync(path.join(source, `${resourceId}.bmp`)));
  }
  return { dll: "STRATEGY.DLL", resource_count: sortedIds.length, resource_ids: sortedIds, aggregate_sha256: aggregate.digest("hex") };
}

function composeExpected(source, faction, selectedCategory = 0x6f, heldCommand = null) {
  const base = resource(source, faction.base);
  const expected = new PNG({ width: dimensions.width, height: dimensions.height });
  blitClipped(expected, base, 0, 0, dimensions.width, dimensions.height);
  blitClipped(expected, resource(source, faction.rail), 412, 0, 58, 330);
  blitClipped(expected, resource(source, 10338), 12, 13, 400, 306);
  for (const control of categoryControls) {
    const [normal, pressed] = resourcesFor(control, faction);
    const resourceId = control.command === selectedCategory || control.command === heldCommand ? pressed : normal;
    blitClipped(expected, resource(source, resourceId), control.x, control.y, control.width, control.height);
  }
  for (const control of faction.railControls) {
    const [normal, pressed] = control.resources;
    const resourceId = control.selected || control.command === heldCommand ? pressed : normal;
    blitClipped(expected, resource(source, resourceId), control.x, control.y, control.width, control.height);
  }
  return expected;
}

function cropShell(screenshotBytes) {
  const screenshot = PNG.sync.read(screenshotBytes);
  assert.deepEqual([screenshot.width, screenshot.height], [640, 480]);
  const shell = new PNG(dimensions);
  for (let y = 0; y < dimensions.height; y += 1) {
    const start = ((origin.y + y) * screenshot.width + origin.x) * 4;
    const end = start + dimensions.width * 4;
    shell.data.set(screenshot.data.subarray(start, end), y * dimensions.width * 4);
  }
  return shell;
}

function compare(actual, expected, label, directory, ignoredRects = []) {
  let differentPixels = 0;
  let ignoredPixels = 0;
  let ignoredDifferencePixels = 0;
  const ignoredRegionDifferencePixels = Object.fromEntries(
    ignoredRects.map((rect, index) => [rect.label || `region-${index}`, 0]),
  );
  const diff = new PNG(dimensions);
  for (let offset = 0; offset < expected.data.length; offset += 4) {
    const pixel = offset / 4;
    const x = pixel % expected.width;
    const y = Math.floor(pixel / expected.width);
    const ignored = ignoredRects.some((rect) => x >= rect.x && x < rect.x + rect.width
      && y >= rect.y && y < rect.y + rect.height);
    const matches = actual.data[offset] === expected.data[offset]
      && actual.data[offset + 1] === expected.data[offset + 1]
      && actual.data[offset + 2] === expected.data[offset + 2];
    if (ignored) {
      ignoredPixels += 1;
      if (!matches) {
        ignoredDifferencePixels += 1;
        const region = ignoredRects.find((rect) => x >= rect.x && x < rect.x + rect.width
          && y >= rect.y && y < rect.y + rect.height);
        if (region) ignoredRegionDifferencePixels[region.label] += 1;
      }
      diff.data.set(matches ? [32, 32, 32, 255] : [0, 120, 255, 255], offset);
    } else {
      if (!matches) differentPixels += 1;
      diff.data.set(matches ? [0, 0, 0, 255] : [255, 0, 80, 255], offset);
    }
  }
  fs.writeFileSync(path.join(directory, `${label}-actual.png`), PNG.sync.write(actual));
  fs.writeFileSync(path.join(directory, `${label}-expected.png`), PNG.sync.write(expected));
  fs.writeFileSync(path.join(directory, `${label}-diff.png`), PNG.sync.write(diff));
  return {
    label,
    pixels_checked: expected.width * expected.height - ignoredPixels,
    ignored_pixels: ignoredPixels,
    ignored_difference_pixels: ignoredDifferencePixels,
    ignored_region_difference_pixels: ignoredRegionDifferencePixels,
    different_pixels: differentPixels,
  };
}

async function stableShellScreenshot(page, directory, label) {
  await page.waitForTimeout(100);
  await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  const first = cropShell(await page.screenshot({ animations: "disabled" }));
  await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  const second = cropShell(await page.screenshot({ animations: "disabled" }));
  fs.writeFileSync(path.join(directory, `${label}-frame-1.png`), PNG.sync.write(first));
  fs.writeFileSync(path.join(directory, `${label}-frame-2.png`), PNG.sync.write(second));
  assert.equal(sha256(PNG.sync.write(first)), sha256(PNG.sync.write(second)), "Encyclopedia shell did not stabilize");
  return second;
}

async function inspectFaction(server, source, faction, executable) {
  const directory = path.join(runDir, faction.name);
  fs.mkdirSync(directory, { recursive: true });
  const requests = [];
  const errors = [];
  const consoleLines = [];
  const launchAttempts = [];
  let browser;
  let context;
  let page;
  let result;
  try {
    browser = await launchBrowser(chromium, {
      executablePath: executable,
      headless: true,
      args: browserManifest.launch_arguments,
      timeout: 30_000,
    }, launchAttempts);
    context = await browser.newContext({
      viewport: { width: 640, height: 480 },
      deviceScaleFactor: 1,
      locale: "en-US",
      timezoneId: "America/New_York",
      colorScheme: "dark",
      reducedMotion: "reduce",
      serviceWorkers: "block",
    });
    page = await context.newPage();
    const serverOrigin = `http://127.0.0.1:${server.address().port}`;
    page.on("response", (response) => {
      if (new URL(response.url()).origin === serverOrigin) requests.push({ url: new URL(response.url()).pathname, status: response.status() });
    });
    page.on("requestfailed", (request) => errors.push(`requestfailed:${request.url()}:${request.failure()?.errorText}`));
    page.on("pageerror", (error) => errors.push(`pageerror:${error.stack || error.message}`));
    page.on("console", (message) => {
      consoleLines.push({ type: message.type(), text: message.text() });
      if (message.type() === "error" || /\[bmp_cache\] asset unavailable/i.test(message.text())) {
        errors.push(`console:${message.type()}:${message.text()}`);
      }
    });

    const fixtureCode = (catalogMode ? 42 : 41) | (faction.byte << 8);
    await page.goto(`${serverOrigin}/?fixture-code=${fixtureCode}`, { waitUntil: "load", timeout: 30_000 });
    await page.waitForFunction(() => window.__openRebellionInterfaceReady?.status, null, { timeout: 30_000 });
    const ready = await page.evaluate(() => window.__openRebellionInterfaceReady);
    assert.equal(ready.status, "ready", JSON.stringify(ready));
    assert.equal(ready.code, fixtureCode);
    await page.evaluate(() => document.fonts.ready);
    if (catalogMode) {
      assert.ok(
        consoleLines.some(({ text }) => text.includes("[encyclopedia] source_catalog loaded entries=346 categories=all:346,systems:200,ships:38,facilities:14,missions:15,troops:10,personnel:69")),
        "browser did not load the complete source-derived Encyclopedia catalog",
      );
    }

    const comparisons = [];
    const ignoredRects = catalogMode ? [
      { label: "title", x: 20, y: 4, width: 380, height: 32 },
      { label: "topic-and-current-object", x: 30, y: 40, width: 365, height: 32 },
      { label: "category-label", x: 40, y: 119, width: 330, height: 18 },
      { label: "list", x: 36, y: 137, width: 338, height: 160 },
    ] : [];
    const compareState = async (label, category, heldCommand = null) => compare(
      await stableShellScreenshot(page, directory, label),
      composeExpected(source, faction, category, heldCommand),
      label,
      directory,
      ignoredRects,
    );
    let selectedCategory = 0x6f;
    comparisons.push(await compareState("selected-6f", selectedCategory));
    for (const control of categoryControls.slice(1)) {
      await page.mouse.click(origin.x + control.x + Math.floor(control.width / 2), origin.y + control.y + Math.floor(control.height / 2));
      selectedCategory = control.command;
      const label = `selected-${control.command.toString(16)}`;
      comparisons.push(await compareState(label, selectedCategory));
    }
    if (catalogMode) {
      const beforeScroll = fs.readFileSync(path.join(directory, "selected-75-actual.png"));
      await page.mouse.click(origin.x + 380, origin.y + 290);
      comparisons.push(await compareState("list-scroll-down", selectedCategory));
      const afterScroll = fs.readFileSync(path.join(directory, "list-scroll-down-actual.png"));
      assert.notEqual(sha256(beforeScroll), sha256(afterScroll), "the native list down control did not scroll the catalog");
      const activeCategory = categoryControls.find(({ command }) => command === selectedCategory);
      await page.mouse.click(
        origin.x + activeCategory.x + Math.floor(activeCategory.width / 2),
        origin.y + activeCategory.y + Math.floor(activeCategory.height / 2),
      );
      comparisons.push(await compareState("selected-category-reclick", selectedCategory));
      const afterCategoryReclick = fs.readFileSync(path.join(directory, "selected-category-reclick-actual.png"));
      assert.equal(
        sha256(afterScroll),
        sha256(afterCategoryReclick),
        "re-clicking the selected category reset the native list scroll position",
      );
      const beforeSelection = PNG.sync.write(await stableShellScreenshot(page, directory, "row-selection-before"));
      await page.mouse.click(origin.x + 100, origin.y + 137 + 1 * 18 + 9);
      comparisons.push(await compareState("row-selection-after", selectedCategory));
      const afterSelection = fs.readFileSync(path.join(directory, "row-selection-after-actual.png"));
      assert.notEqual(sha256(beforeSelection), sha256(afterSelection), "selecting another catalog row did not alter the rendered state");
      assert.ok(
        comparisons.every(({ ignored_region_difference_pixels: regions }) =>
          Object.keys(regions).length === ignoredRects.length
          && Object.values(regions).every((count) => count > 100)),
        "localized title, topic/current-object, category-label, or list content was not visible in every catalog state",
      );
    } else {
      for (const control of faction.railControls) {
        await page.mouse.move(origin.x + control.x + Math.floor(control.width / 2), origin.y + control.y + Math.floor(control.height / 2));
        await page.mouse.down();
        const label = `held-${control.command.toString(16)}`;
        comparisons.push(await compareState(label, selectedCategory, control.command));
        await page.mouse.up();
      }
      const edgeControl = categoryControls.at(-1);
      await page.mouse.move(origin.x + edgeControl.x + edgeControl.width, origin.y + edgeControl.y + 20);
      await page.mouse.down();
      comparisons.push(await compareState("outside-category-right-edge", selectedCategory));
      await page.mouse.up();
      const close = faction.railControls[0];
      await page.mouse.move(origin.x + close.x + close.width, origin.y + close.y + Math.floor(close.height / 2));
      await page.mouse.down();
      comparisons.push(await compareState("outside-close-right-edge", selectedCategory));
      await page.mouse.up();

      const retainedControl = categoryControls.at(-2);
      await page.mouse.click(
        origin.x + retainedControl.x + Math.floor(retainedControl.width / 2),
        origin.y + retainedControl.y + Math.floor(retainedControl.height / 2),
      );
      await page.waitForTimeout(100);
      await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(resolve)));
      selectedCategory = retainedControl.command;
      const probeControl = categoryControls.at(-1);
      const [probeResource] = resourcesFor(probeControl, faction);
      const transparentPoint = nearestMaskPoint(source, probeResource, probeControl, false);
      await page.mouse.move(
        origin.x + probeControl.x + transparentPoint.x,
        origin.y + probeControl.y + transparentPoint.y,
      );
      await page.mouse.down();
      comparisons.push(await compareState("transparent-category-hit-miss", selectedCategory));
      await page.mouse.up();

      const opaquePoint = nearestMaskPoint(source, probeResource, probeControl, true);
      const probeInside = {
        x: origin.x + probeControl.x + opaquePoint.x,
        y: origin.y + probeControl.y + opaquePoint.y,
      };
      const probeOutside = {
        x: origin.x + probeControl.x + probeControl.width,
        y: probeInside.y,
      };
      await page.mouse.move(probeOutside.x, probeOutside.y);
      await page.mouse.down();
      await page.mouse.move(probeInside.x, probeInside.y);
      comparisons.push(await compareState("press-origin-outside-drag-inside", selectedCategory));
      await page.mouse.up();

      await page.mouse.move(probeInside.x, probeInside.y);
      await page.mouse.down();
      await page.mouse.move(probeOutside.x, probeOutside.y);
      comparisons.push(await compareState("press-origin-inside-drag-outside", selectedCategory));
      await page.mouse.up();
    }

    await page.mouse.click(faction.underlyingEncyclopedia.x, faction.underlyingEncyclopedia.y);
    comparisons.push(await compareState("underlying-cockpit-encyclopedia-blocked", selectedCategory));
    assert.ok(
      consoleLines.every(({ text }) => !/\[interface\] command=0x131\b/.test(text)),
      "modal Encyclopedia fixture leaked input to the underlying cockpit control",
    );

    assert.ok(comparisons.every(({ different_pixels: differentPixels }) => differentPixels === 0),
      JSON.stringify(comparisons.filter(({ different_pixels: differentPixels }) => differentPixels !== 0)));
    assert.deepEqual(requests.map(({ url }) => url).sort(), [...expectedRequests].sort());
    assert.ok(requests.every(({ status }) => status === 200));
    assert.deepEqual(errors, []);
    result = { status: "pass", faction: faction.name, fixture_code: fixtureCode, ready, comparisons, requests, errors, console: consoleLines, launch_attempts: launchAttempts, cleanup: "pending" };
  } catch (error) {
    result = { status: "fail", faction: faction.name, error: String(error.stack || error), requests, errors, console: consoleLines, launch_attempts: launchAttempts, cleanup: "pending" };
  } finally {
    if (page) await page.close().catch((error) => errors.push(`page-close:${error}`));
    if (context) await context.close().catch((error) => errors.push(`context-close:${error}`));
    if (browser) await browser.close().catch((error) => errors.push(`browser-close:${error}`));
    result.cleanup = errors.some((error) => error.includes("-close:")) ? "failed" : "closed";
    if (result.cleanup === "failed") result.status = "fail";
  }
  return result;
}

async function main() {
  const source = sourceDirectory();
  if (!noBuild) {
    execFileSync("bash", [path.join(root, "scripts/build-interface-test-wasm.sh")], { cwd: root, env: { ...process.env }, stdio: "inherit" });
  }
  for (const required of ["index.html", "gl.js", "open-rebellion-test.wasm", "data/runtime.orpk"]) {
    assert.ok(fs.existsSync(path.join(site, required)), `fixture site is missing ${required}`);
  }
  fs.mkdirSync(runDir, { recursive: true });
  const executable = browserExecutable();
  const server = await startServer();
  let results;
  try {
    results = [];
    for (const faction of factions) results.push(await inspectFaction(server, source, faction, executable));
  } finally {
    await new Promise((resolve) => server.close(resolve));
  }
  const passed = results.every((result) => result.status === "pass");
  const summary = {
    schema_version: 1,
    family: catalogMode ? "encyclopedia-index-catalog" : "encyclopedia-index-shell",
    scope: catalogMode
      ? "production-dormant source-derived Galactic Encyclopedia localized catalog, seven family filters, alphabetical ordering, stable row identity, exact bitmap pixels outside dynamic text, and modal input blocking; topics and production routing remain open"
      : "test-only source-exact Galactic Encyclopedia index bitmap shell, category selection, rail states, clipping, edge, palette-key, press-capture, cancel, and modal input probes; labels, object rows, topics, and production routing remain open",
    status: passed ? "pass" : "fail",
    browser_version: browserManifest.version,
    browser_executable: executable,
    launch_arguments: browserManifest.launch_arguments,
    profile: "one-new-muted-process-per-faction",
    muted: browserManifest.launch_arguments.includes("--mute-audio"),
    viewport: { width: 640, height: 480, device_scale_factor: 1 },
    shell: { width: dimensions.width, height: dimensions.height },
    source: sourceIdentity(source),
    factions: results,
    wasm_sha256: sha256(fs.readFileSync(path.join(site, "open-rebellion-test.wasm"))),
    runtime_pack_sha256: sha256(fs.readFileSync(path.join(site, "data/runtime.orpk"))),
  };
  fs.writeFileSync(path.join(runDir, "result.json"), `${JSON.stringify(summary, null, 2)}\n`);
  if (!passed) throw new Error(JSON.stringify(results.filter((result) => result.status !== "pass"), null, 2));
  process.stdout.write(`${JSON.stringify({ run_dir: runDir, ...summary }, null, 2)}\n`);
}

await main();
