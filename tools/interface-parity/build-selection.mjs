#!/usr/bin/env node

// Build Selection gate: on each side, right-click a Manufacturing window band,
// choose Build, compare the window's static STRATEGY chrome exactly, Confirm,
// see the band take the product, then Stop it.

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
const expectedRequests = ["/", "/data/runtime.orpk", "/gl.js", "/open-rebellion-test.wasm"];
// FUN_00437df0: a 210 by 261 window over background 10800.
const dimensions = { width: 210, height: 261 };
// Fixture code 55 is Scenario::BuildSelection (54) plus one.
const scenarioCode = 55;
// FUN_00438500: the side's title strip, 10801 for side 1, 10802 otherwise.
const factions = [
  { name: "alliance", byte: 1, title: 10801 },
  { name: "empire", byte: 2, title: 10802 },
];
// FUN_00438620 controls: (x, y, width, height, normal bitmap, native size).
const controls = [
  { x: 79, y: 90, width: 65, height: 18, id: 10606, native: [65, 18] },
  { x: 189, y: 196, width: 13, height: 8, id: 10610, native: [13, 8] },
  { x: 189, y: 205, width: 13, height: 8, id: 10612, native: [13, 8] },
  { x: 5, y: 224, width: 66, height: 33, id: 10592, native: [66, 33] },
  { x: 73, y: 224, width: 66, height: 33, id: 10594, native: [66, 33] },
  { x: 141, y: 224, width: 66, height: 33, id: 10596, native: [66, 33] },
  // port: the close box sits 17 pixels in from the right, as the mission
  // dialog's (hyp: the source chrome's placement is untraced).
  { x: 193, y: 3, width: 14, height: 14, id: 10108, native: [14, 14] },
];
// Dynamic content, kept as captures: the title, the selected class, the
// labels, the four values and the number to build (FUN_00437f80,
// FUN_00438500).
const masks = [
  { x: 2, y: 2, width: 190, height: 15 },
  { x: 6, y: 22, width: 195, height: 63 },
  { x: 36, y: 110, width: 64, height: 23 },
  { x: 138, y: 110, width: 64, height: 23 },
  { x: 10, y: 145, width: 125, height: 15 },
  { x: 10, y: 165, width: 125, height: 15 },
  { x: 140, y: 145, width: 60, height: 15 },
  { x: 140, y: 165, width: 60, height: 15 },
  { x: 20, y: 196, width: 118, height: 15 },
  { x: 141, y: 196, width: 45, height: 17 },
];
const runId = `${new Date().toISOString().replace(/[:.]/g, "-")}-${process.pid}`;
const runDir = path.join(root, ".artifacts/interface-parity", `build-selection-${runId}`);

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

function sourceDirectory() {
  const candidates = [
    process.env.REBELLION_STRATEGY_BMP_DIR,
    path.join(root, "data/base/ui/strategy-dll/BMP"),
  ].filter(Boolean);
  const directory = candidates.find((candidate) => fs.existsSync(path.join(candidate, "11100.bmp")));
  if (!directory) {
    throw new Error("owned STRATEGY.DLL BMP extraction is unavailable; set REBELLION_STRATEGY_BMP_DIR");
  }
  return directory;
}

function browserExecutable() {
  const candidates = [
    ...(process.env.OPEN_REBELLION_CHROME_FOR_TESTING
      ? [process.env.OPEN_REBELLION_CHROME_FOR_TESTING]
      : []),
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

function decodeIndexedBmp(bytes, expectedWidth, expectedHeight) {
  assert.equal(bytes.toString("ascii", 0, 2), "BM", "source resource is not a BMP");
  const dataOffset = bytes.readUInt32LE(10);
  const headerSize = bytes.readUInt32LE(14);
  const width = bytes.readInt32LE(18);
  const signedHeight = bytes.readInt32LE(22);
  const height = Math.abs(signedHeight);
  assert.deepEqual([width, height, bytes.readUInt16LE(28), bytes.readUInt32LE(30)],
    [expectedWidth, expectedHeight, 8, 0], "source resource has unexpected dimensions or encoding");
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

function blitClipped(destination, source, x, y, width = source.width, height = source.height) {
  // FUN_005fc140 blits at the bitmap's own size; the control clips the rest.
  for (let row = 0; row < Math.min(height, source.height); row += 1) {
    if (y + row >= destination.height) break;
    const columns = Math.min(width, source.width, destination.width - x);
    const start = row * source.width * 4;
    destination.data.set(source.data.subarray(start, start + columns * 4), ((y + row) * destination.width + x) * 4);
  }
}

async function frames(page) {
  await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))));
}


function resource(source, id, width, height) {
  return decodeIndexedBmp(fs.readFileSync(path.join(source, `${id}.bmp`)), width, height);
}

function composeExpected(source, faction) {
  const expected = resource(source, 10800, dimensions.width, dimensions.height);
  blitClipped(expected, resource(source, faction.title, 240, 17), 2, 2);
  for (const control of controls) {
    blitClipped(expected, resource(source, control.id, ...control.native), control.x, control.y,
      control.width, control.height);
  }
  return expected;
}

function masked(x, y) {
  return masks.some((mask) => x >= mask.x && x < mask.x + mask.width
    && y >= mask.y && y < mask.y + mask.height);
}

function crop(screenshotBytes, origin) {
  const screenshot = PNG.sync.read(screenshotBytes);
  const cropped = new PNG({ width: dimensions.width, height: dimensions.height });
  for (let y = 0; y < dimensions.height; y += 1) {
    const start = ((origin.y + y) * screenshot.width + origin.x) * 4;
    cropped.data.set(screenshot.data.subarray(start, start + dimensions.width * 4), y * dimensions.width * 4);
  }
  return cropped;
}

function compare(actual, expected, label, directory) {
  let differentPixels = 0;
  let checked = 0;
  const diff = new PNG({ width: expected.width, height: expected.height });
  for (let y = 0; y < expected.height; y += 1) {
    for (let x = 0; x < expected.width; x += 1) {
      const offset = (y * expected.width + x) * 4;
      if (masked(x, y)) {
        diff.data.set([0, 0, 96, 255], offset);
        continue;
      }
      checked += 1;
      const matches = [0, 1, 2].every((channel) => actual.data[offset + channel] === expected.data[offset + channel]);
      if (!matches) differentPixels += 1;
      diff.data.set(matches ? [0, 0, 0, 255] : [255, 0, 80, 255], offset);
    }
  }
  fs.writeFileSync(path.join(directory, `${label}-actual.png`), PNG.sync.write(actual));
  fs.writeFileSync(path.join(directory, `${label}-expected.png`), PNG.sync.write(expected));
  fs.writeFileSync(path.join(directory, `${label}-diff.png`), PNG.sync.write(diff));
  return { label, pixels_checked: checked, different_pixels: differentPixels };
}

async function click(page, point, button = "left") {
  // egui resolves a click across frames: hover, press, then release.
  await page.mouse.move(point.x, point.y);
  await frames(page);
  await page.mouse.down({ button });
  await frames(page);
  await page.mouse.up({ button });
  await frames(page);
}

async function production(page) {
  return page.evaluate(() => window.__openRebellionInterfaceProduction);
}

// Right-click the band, then click the menu row the report names.
async function bandOrder(page, band, rowName, directory, label) {
  await page.evaluate(() => { window.__openRebellionInterfaceObjectMenu = undefined; });
  await click(page, { x: band.left + band.width / 2, y: band.top + band.height / 2 }, "right");
  await page.waitForFunction(() => window.__openRebellionInterfaceObjectMenu?.status === "object-menu",
    null, { timeout: 10_000 });
  const menu = await page.evaluate(() => window.__openRebellionInterfaceObjectMenu);
  fs.writeFileSync(path.join(directory, `${label}-menu.png`), await page.screenshot({ animations: "disabled" }));
  const index = menu[rowName];
  assert.notEqual(index, null, `the band's menu lists ${rowName}`);
  const row = (menu.height - 2) / menu.rows;
  await click(page, { x: menu.left + menu.width / 2, y: menu.top + 2 + row * (index + 0.5) });
  await page.mouse.move(2, 2);
  await frames(page);
  return menu;
}

async function stableWindow(page, origin, directory, label) {
  await page.waitForTimeout(150);
  await frames(page);
  const first = await page.screenshot({ animations: "disabled" });
  await frames(page);
  const second = await page.screenshot({ animations: "disabled" });
  fs.writeFileSync(path.join(directory, `${label}-screen.png`), second);
  const a = crop(first, origin);
  const b = crop(second, origin);
  assert.equal(sha256(PNG.sync.write(a)), sha256(PNG.sync.write(b)), `window did not stabilize (${label})`);
  return b;
}

async function inspect(server, source, faction, executable) {
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
      if (new URL(response.url()).origin === serverOrigin) {
        requests.push({ url: new URL(response.url()).pathname, status: response.status() });
      }
    });
    page.on("requestfailed", (request) => errors.push(`requestfailed:${request.url()}:${request.failure()?.errorText}`));
    page.on("pageerror", (error) => errors.push(`pageerror:${error.stack || error.message}`));
    page.on("console", (message) => {
      consoleLines.push({ type: message.type(), text: message.text() });
      if (message.type() === "error" || /\[bmp_cache\] asset unavailable/i.test(message.text())) {
        errors.push(`console:${message.type()}:${message.text()}`);
      }
    });

    const fixtureCode = scenarioCode | (faction.byte << 8);
    await page.goto(`${serverOrigin}/?fixture-code=${fixtureCode}`, { waitUntil: "load", timeout: 30_000 });
    await page.waitForFunction(() => window.__openRebellionInterfaceReady?.status, null, { timeout: 30_000 });
    const ready = await page.evaluate(() => window.__openRebellionInterfaceReady);
    assert.equal(ready.status, "ready", JSON.stringify(ready));
    await page.evaluate(() => document.fonts.ready);
    await page.waitForFunction(() => window.__openRebellionInterfaceProduction?.bands?.length === 3,
      null, { timeout: 10_000 });
    const before = await production(page);
    const troops = before.bands.find(({ area }) => area === "troops");
    assert.equal(troops.queued, 0, "the training facility starts idle");
    fs.writeFileSync(path.join(directory, "overview-screen.png"), await page.screenshot({ animations: "disabled" }));

    // Build (0x212) opens Build Selection (FUN_0041d640).
    const buildMenu = await bandOrder(page, troops, "build_row", directory, "build");
    await page.waitForFunction(() => window.__openRebellionInterfaceProduction?.build_selection_open === true,
      null, { timeout: 10_000 });
    const opened = await production(page);
    const origin = { x: Math.round(opened.build_selection_left), y: Math.round(opened.build_selection_top) };
    const captured = await stableWindow(page, origin, directory, "opened");
    const comparisons = [compare(captured, composeExpected(source, faction), "opened", directory)];

    // Confirm (0x65, FUN_00438980) builds one unit of the selected class.
    await click(page, { x: origin.x + 73 + 33, y: origin.y + 224 + 16 });
    await page.mouse.move(2, 2);
    await page.waitForFunction(() => {
      const report = window.__openRebellionInterfaceProduction;
      const band = report?.bands?.find(({ area }) => area === "troops");
      return report && !report.build_selection_open && band?.queued > 0;
    }, null, { timeout: 10_000 });
    const built = (await production(page)).bands.find(({ area }) => area === "troops");
    assert.ok(built.product.length > 0, "the band names its product");
    fs.writeFileSync(path.join(directory, "confirmed-screen.png"), await page.screenshot({ animations: "disabled" }));

    // Stop (0x213) clears the band (manual p. 84).
    const stopMenu = await bandOrder(page, built, "stop_row", directory, "stop");
    await page.waitForFunction(() => window.__openRebellionInterfaceProduction?.bands
      ?.find(({ area }) => area === "troops")?.queued === 0, null, { timeout: 10_000 });
    fs.writeFileSync(path.join(directory, "stopped-screen.png"), await page.screenshot({ animations: "disabled" }));

    assert.ok(comparisons.every(({ different_pixels: d }) => d === 0),
      JSON.stringify(comparisons.filter(({ different_pixels: d }) => d !== 0)));
    assert.deepEqual(requests.map(({ url }) => url).sort(), [...expectedRequests].sort());
    assert.ok(requests.every(({ status }) => status === 200), "startup has non-200 requests");
    assert.deepEqual(errors, [], `browser diagnostics: ${errors.join("; ")}`);
    result = {
      status: "pass",
      faction: faction.name,
      fixture_code: fixtureCode,
      origin,
      build_menu: buildMenu,
      stop_menu: stopMenu,
      built: { product: built.product, queued: built.queued },
      comparisons,
      requests,
      errors,
      console: consoleLines,
      launch_attempts: launchAttempts,
      cleanup: "pending",
    };
  } catch (error) {
    result = {
      status: "fail",
      faction: faction.name,
      error: String(error.stack || error),
      requests,
      errors,
      console: consoleLines,
      launch_attempts: launchAttempts,
      cleanup: "pending",
    };
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
    execFileSync("bash", [path.join(root, "scripts/build-interface-test-wasm.sh")], {
      cwd: root,
      env: { ...process.env },
      stdio: "inherit",
    });
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
    for (const faction of factions) results.push(await inspect(server, source, faction, executable));
  } finally {
    await new Promise((resolve) => server.close(resolve));
  }
  const passed = results.every((result) => result.status === "pass");
  const ids = [10800, 10108, ...new Set([...controls.map(({ id }) => id), ...factions.map(({ title }) => title)])]
    .filter((id, index, all) => all.indexOf(id) === index).sort((a, b) => a - b);
  const aggregate = createHash("sha256");
  for (const id of ids) {
    aggregate.update(`${id}\0`);
    aggregate.update(fs.readFileSync(path.join(source, `${id}.bmp`)));
  }
  const summary = {
    schema_version: 1,
    family: "build-selection",
    scope: "test-only Build Selection window (FUN_00437df0) reached from a Manufacturing window band's Build: static STRATEGY chrome compared exactly; title, class item, labels, values and number masked and kept as captures; Confirm builds and Stop clears the band",
    status: passed ? "pass" : "fail",
    browser_version: browserManifest.version,
    browser_executable: executable,
    launch_arguments: browserManifest.launch_arguments,
    viewport: { width: 640, height: 480, device_scale_factor: 1 },
    window: dimensions,
    masks,
    source: { dll: "STRATEGY.DLL", resource_ids: ids, aggregate_sha256: aggregate.digest("hex") },
    factions: results,
    wasm_sha256: sha256(fs.readFileSync(path.join(site, "open-rebellion-test.wasm"))),
    runtime_pack_sha256: sha256(fs.readFileSync(path.join(site, "data/runtime.orpk"))),
  };
  fs.writeFileSync(path.join(runDir, "result.json"), `${JSON.stringify(summary, null, 2)}\n`);
  if (!passed) throw new Error(JSON.stringify(results.filter((result) => result.status !== "pass"), null, 2));
  process.stdout.write(`${JSON.stringify({ run_dir: runDir, status: summary.status, factions: results.map(({ faction, status, comparisons, built }) => ({ faction, status, comparisons, built })) }, null, 2)}\n`);
}

await main();
