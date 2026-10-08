#!/usr/bin/env node

// The Status window (window type 0x1a, FUN_00442d70; ghidra/notes/status-window.md)
// opened from each object family's pop-up menu (Status, 0x103): a character
// (manual p. 101: the System Defenses window's personnel page), a regiment
// (its regiment page), a fleet and a capital ship (the Fleet window) and a
// production manager (a Manufacturing window band). Its STRATEGY background
// and buttons are compared exactly; the title, list, name and picture are
// masked and checked for content. Close closes each; for the character,
// Escape and the Encyclopedia button close it too. Both sides.

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
const dimensions = { width: 379, height: 272 };
// Fixture 45 (Scenario::MissionTargeting + 1) opens a Defenses window with the
// player's agent first on its personnel page; `galaxy` is the galaxy view's top-left and width at 640 by
// 480. FUN_00443130: an own-side character's background is 0x2d22 + its side,
// plus 3 for a player not of side 1.
const factions = [
  { name: "alliance", byte: 1, galaxy: { x: 55, y: 40, width: 485 }, background: 11554 },
  { name: "empire", byte: 2, galaxy: { x: 120, y: 40, width: 480 }, background: 11558 },
];
// Fixture codes are the Scenario index plus one (interface_test_fixture.rs):
// 45 MissionTargeting, 48 FleetLoad, 55 BuildSelection.
const MISSION_TARGETING = 45;
const FLEET_LOAD = 48;
const BUILD_SELECTION = 55;
// FUN_00443020: 0x66 Encyclopedia and 0x65 Close, 32 by 31.
const buttons = {
  encyclopedia: { x: 258, y: 218, id: 11552 },
  close: { x: 324, y: 218, id: 10370 },
};
// Dynamic content: the title, the list, the name and the picture frame's
// interior (x 242..371, y 15..112 of the owned backgrounds), which a fleet's
// or a production manager's picture fills.
const masks = {
  title: { x: 15, y: 18, width: 211, height: 18 },
  list: { x: 18, y: 47, width: 208, height: 204 },
  name: { x: 242, y: 137, width: 130, height: 44 },
  picture: { x: 242, y: 15, width: 130, height: 98 },
};
const runId = `${new Date().toISOString().replace(/[:.]/g, "-")}-${process.pid}`;
const runDir = path.join(root, ".artifacts/interface-parity", `status-window-${runId}`);

const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");

function sourceDirectory() {
  const candidates = [
    process.env.REBELLION_STRATEGY_BMP_DIR,
    path.join(root, "data/base/ui/strategy-dll/BMP"),
  ].filter(Boolean);
  const directory = candidates.find((candidate) => fs.existsSync(path.join(candidate, "11554.bmp")));
  if (!directory) throw new Error("owned STRATEGY.DLL BMP extraction is unavailable; set REBELLION_STRATEGY_BMP_DIR");
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
      response.writeHead(200, { "content-type": mimeType(candidate), "content-length": bytes.length, "cache-control": "no-store" }).end(bytes);
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
  const png = new PNG({ width, height });
  for (let y = 0; y < height; y += 1) {
    const sourceY = signedHeight > 0 ? height - 1 - y : y;
    for (let x = 0; x < width; x += 1) {
      const palette = paletteOffset + bytes[dataOffset + sourceY * stride + x] * 4;
      const destination = (y * width + x) * 4;
      png.data[destination] = bytes[palette + 2];
      png.data[destination + 1] = bytes[palette + 1];
      png.data[destination + 2] = bytes[palette];
      png.data[destination + 3] = 255;
    }
  }
  return png;
}

const resource = (source, id, width, height) => decodeIndexedBmp(fs.readFileSync(path.join(source, `${id}.bmp`)), width, height);

function blit(destination, image, x, y) {
  for (let row = 0; row < image.height; row += 1) {
    const start = row * image.width * 4;
    destination.data.set(image.data.subarray(start, start + image.width * 4), ((y + row) * destination.width + x) * 4);
  }
}

function composeExpected(source, faction) {
  const expected = resource(source, faction.background, dimensions.width, dimensions.height);
  for (const button of Object.values(buttons)) blit(expected, resource(source, button.id, 32, 31), button.x, button.y);
  return expected;
}

const inside = (box, x, y) => x >= box.x && x < box.x + box.width && y >= box.y && y < box.y + box.height;

function crop(screenshotBytes, rect) {
  const screenshot = PNG.sync.read(screenshotBytes);
  assert.deepEqual([screenshot.width, screenshot.height], [640, 480]);
  const window = new PNG({ width: rect.width, height: rect.height });
  for (let y = 0; y < rect.height; y += 1) {
    const start = ((rect.y + y) * screenshot.width + rect.x) * 4;
    window.data.set(screenshot.data.subarray(start, start + rect.width * 4), y * rect.width * 4);
  }
  return window;
}

function compare(actual, expected, directory, label) {
  let different = 0;
  let checked = 0;
  const content = Object.fromEntries(Object.keys(masks).map((name) => [name, 0]));
  let blue = 0;
  for (let y = 0; y < expected.height; y += 1) {
    for (let x = 0; x < expected.width; x += 1) {
      const offset = (y * expected.width + x) * 4;
      const same = [0, 1, 2].every((index) => actual.data[offset + index] === expected.data[offset + index]);
      const mask = Object.entries(masks).find(([, box]) => inside(box, x, y));
      if (mask) {
        if (!same) content[mask[0]] += 1;
        if (mask[0] === "picture" && actual.data[offset] === 0 && actual.data[offset + 1] === 0 && actual.data[offset + 2] === 255) blue += 1;
        continue;
      }
      checked += 1;
      if (!same) different += 1;
    }
  }
  fs.writeFileSync(path.join(directory, `${label}-actual.png`), PNG.sync.write(actual));
  fs.writeFileSync(path.join(directory, `${label}-expected.png`), PNG.sync.write(expected));
  return { pixels_checked: checked, different_pixels: different, changed_in_masks: content, picture_blue_pixels: blue };
}

async function frames(page) {
  await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))));
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

// Right-click `at`, wait for the pop-up menu and choose its Status row.
// Rows below a 2-pixel border share one height (object_menu.rs).
async function chooseStatus(page, at) {
  await page.evaluate(() => { window.__openRebellionInterfaceObjectMenu = undefined; });
  await click(page, at, "right");
  await page.waitForFunction(() => window.__openRebellionInterfaceObjectMenu?.status === "object-menu", null, { timeout: 10_000 });
  const menu = await page.evaluate(() => window.__openRebellionInterfaceObjectMenu);
  assert.notEqual(menu.status_window_row, null, "the menu lists Status");
  const row = (menu.height - 2) / menu.rows;
  await click(page, { x: menu.left + menu.width / 2, y: menu.top + 2 + row * (menu.status_window_row + 0.5) });
  return menu;
}

const point = ([x, y]) => ({ x: Math.round(x), y: Math.round(y) });

async function doubleClick(page, at) {
  await page.mouse.move(at.x, at.y);
  await frames(page);
  await page.mouse.dblclick(at.x, at.y);
  await frames(page);
}

async function fleetLoadSetup(page) {
  await page.waitForFunction(() => window.__openRebellionInterfaceFleetLoadSetup
    && (window.__openRebellionInterfaceFleetLoads || []).length > 0, null, { timeout: 10_000 });
  return page.evaluate(() => window.__openRebellionInterfaceFleetLoadSetup);
}

async function fleetLoad(page, predicate) {
  await page.waitForFunction(
    (source) => {
      const last = (window.__openRebellionInterfaceFleetLoads || []).at(-1);
      return last && new Function("o", `return (${source})(o);`)(last);
    },
    predicate.toString(),
    { timeout: 10_000, polling: 100 },
  );
  return page.evaluate(() => window.__openRebellionInterfaceFleetLoads.at(-1));
}

// The sector window's fleet icon opens the Fleet window (fleet-window.mjs).
async function openFleetWindow(page) {
  const setup = await fleetLoadSetup(page);
  await doubleClick(page, point(setup.icon));
  return fleetLoad(page, (o) => o.window_open && o.fleet_entry);
}

// Each case reaches one family's object and returns the point to right-click.
// The fixture's 235-pixel Defenses window sits 5 pixels in from the galaxy
// view's top-right corner; the agent's cell is centred 42 by 116 into it
// (mission-dialog.mjs).
const cases = [
  {
    name: "character",
    code: MISSION_TARGETING,
    family: "Character",
    closes: true,
    target: async (page, faction) => ({ x: faction.galaxy.x + faction.galaxy.width - 235 - 5 + 42, y: faction.galaxy.y + 5 + 116 }),
  },
  {
    name: "regiment",
    code: FLEET_LOAD,
    family: "Troop",
    target: async (page) => point((await fleetLoadSetup(page)).troop_item),
  },
  {
    name: "fleet",
    code: FLEET_LOAD,
    family: "Fleet",
    target: async (page) => point((await openFleetWindow(page)).fleet_entry),
  },
  {
    name: "capital-ship",
    code: FLEET_LOAD,
    family: "Ship",
    target: async (page) => {
      const opened = await openFleetWindow(page);
      await click(page, point(opened.fleet_entry));
      return point((await fleetLoad(page, (o) => o.selected === "fleet" && o.first_item)).first_item);
    },
  },
  {
    name: "production-manager",
    code: BUILD_SELECTION,
    family: "Producer",
    target: async (page) => {
      await page.waitForFunction(() => window.__openRebellionInterfaceProduction?.bands?.length === 3, null, { timeout: 10_000 });
      const [band] = (await page.evaluate(() => window.__openRebellionInterfaceProduction)).bands;
      return { x: band.left + band.width / 2, y: band.top + band.height / 2 };
    },
  },
];

async function inspect(server, source, faction, testCase, executable) {
  const directory = path.join(runDir, faction.name, testCase.name);
  fs.mkdirSync(directory, { recursive: true });
  const errors = [];
  const consoleLines = [];
  const launchAttempts = [];
  let browser;
  let context;
  let page;
  let result;
  const logged = async (description, pattern, count = 1, timeout = 10_000) => {
    const deadline = Date.now() + timeout;
    while (Date.now() < deadline) {
      const lines = consoleLines.filter((line) => pattern.test(line));
      if (lines.length >= count) return lines[count - 1];
      await new Promise((resolve) => setTimeout(resolve, 50));
    }
    throw new Error(`${description}: ${pattern} x${count} not logged; last lines ${JSON.stringify(consoleLines.slice(-6))}`);
  };
  try {
    browser = await launchBrowser(chromium, { executablePath: executable, headless: true, args: browserManifest.launch_arguments, timeout: 30_000 }, launchAttempts);
    context = await browser.newContext({ viewport: { width: 640, height: 480 }, deviceScaleFactor: 1, locale: "en-US", timezoneId: "America/New_York", colorScheme: "dark", reducedMotion: "reduce", serviceWorkers: "block" });
    page = await context.newPage();
    page.on("pageerror", (error) => errors.push(`pageerror:${error.stack || error.message}`));
    page.on("console", (message) => {
      consoleLines.push(message.text());
      if (message.type() === "error" || /\[bmp_cache\] asset unavailable/i.test(message.text())) errors.push(`console:${message.type()}:${message.text()}`);
    });
    const fixtureCode = testCase.code | (faction.byte << 8);
    await page.goto(`http://127.0.0.1:${server.address().port}/?fixture-code=${fixtureCode}`, { waitUntil: "load", timeout: 30_000 });
    await page.waitForFunction(() => window.__openRebellionInterfaceReady?.status, null, { timeout: 30_000 });
    const ready = await page.evaluate(() => window.__openRebellionInterfaceReady);
    assert.equal(ready.status, "ready", JSON.stringify(ready));
    await page.evaluate(() => document.fonts.ready);

    const target = await testCase.target(page, faction);
    const openStatus = () => chooseStatus(page, target);
    const opened = new RegExp(`command=0x103 destination=status_window status=opened object=${testCase.family}[ (].+ rect=(\\d+),(\\d+),(\\d+),(\\d+)`);
    const closed = /destination=status_window status=closed action=(\w+)/;

    const menu = await openStatus();
    const line = await logged("Status opens the window", opened);
    const [, x, y, width, height] = line.match(opened).map(Number);
    const rect = { x, y, width, height };
    assert.deepEqual([width, height], [dimensions.width, dimensions.height]);
    await page.mouse.move(2, 2);
    await page.waitForTimeout(200);
    await frames(page);
    const first = await page.screenshot({ animations: "disabled" });
    await frames(page);
    const second = await page.screenshot({ animations: "disabled" });
    fs.writeFileSync(path.join(directory, "status-screen.png"), second);
    assert.equal(sha256(PNG.sync.write(crop(first, rect))), sha256(PNG.sync.write(crop(second, rect))), "the window did not stabilize");
    // FUN_00443130: every case's object is the player's, so its background
    // is the player's own-side one.
    const comparison = compare(crop(second, rect), composeExpected(source, faction), directory, "window");
    assert.equal(comparison.different_pixels, 0, JSON.stringify(comparison));
    for (const name of Object.keys(masks)) assert.ok(comparison.changed_in_masks[name] > 0, `${name} shows nothing`);
    assert.equal(comparison.picture_blue_pixels, 0, "the picture's blue matte is not keyed");

    const closes = {};
    // 0x65 closes.
    await click(page, { x: rect.x + buttons.close.x + 16, y: rect.y + buttons.close.y + 15 });
    closes.close = (await logged("Close closes it", closed, 1)).match(closed)[1];
    if (testCase.closes) {
      // Escape closes (port: no key slot is traced).
      await openStatus();
      await logged("Status opens it again", opened, 2);
      await page.keyboard.press("Escape");
      closes.escape = (await logged("Escape closes it", closed, 2)).match(closed)[1];
      // 0x66 closes and opens the Encyclopedia.
      await openStatus();
      await logged("Status opens it a third time", opened, 3);
      await click(page, { x: rect.x + buttons.encyclopedia.x + 16, y: rect.y + buttons.encyclopedia.y + 15 });
      closes.encyclopedia = (await logged("Encyclopedia closes it", closed, 3)).match(closed)[1];
      assert.deepEqual(closes, { close: "close", escape: "close", encyclopedia: "encyclopedia" });
      await page.waitForTimeout(200);
      fs.writeFileSync(path.join(directory, "encyclopedia-screen.png"), await page.screenshot({ animations: "disabled" }));
    } else {
      assert.deepEqual(closes, { close: "close" });
    }
    assert.deepEqual(errors, [], `browser diagnostics: ${errors.join("; ")}`);
    result = { status: "pass", faction: faction.name, case: testCase.name, fixture_code: fixtureCode, object_menu: menu, opened: line, rect, comparison, closes, console: consoleLines.slice(-20), launch_attempts: launchAttempts };
  } catch (error) {
    result = { status: "fail", faction: faction.name, case: testCase.name, error: String(error.stack || error), errors, console: consoleLines.slice(-20), launch_attempts: launchAttempts };
  } finally {
    if (page) await page.close().catch(() => {});
    if (context) await context.close().catch(() => {});
    if (browser) await browser.close().catch(() => {});
  }
  return result;
}

async function main() {
  const source = sourceDirectory();
  if (!noBuild) execFileSync("bash", [path.join(root, "scripts/build-interface-test-wasm.sh")], { cwd: root, env: { ...process.env }, stdio: "inherit" });
  for (const required of ["index.html", "gl.js", "open-rebellion-test.wasm", "data/runtime.orpk"]) {
    assert.ok(fs.existsSync(path.join(site, required)), `fixture site is missing ${required}`);
  }
  fs.mkdirSync(runDir, { recursive: true });
  const executable = browserExecutable();
  const server = await startServer();
  const results = [];
  try {
    for (const faction of factions) {
      for (const testCase of cases) results.push(await inspect(server, source, faction, testCase, executable));
    }
  } finally {
    await new Promise((resolve) => server.close(resolve));
  }
  const passed = results.every((result) => result.status === "pass");
  const summary = {
    schema_version: 1,
    family: "status-window",
    scope: "test-only Status window (FUN_00442d70) for a character, a regiment, a fleet, a capital ship and a production manager, each opened from its pop-up menu: STRATEGY background and buttons compared exactly; title, list, name and picture masked and checked for content and keying; Close closes each, and Escape and Encyclopedia close the character's; on both sides",
    status: passed ? "pass" : "fail",
    browser_version: browserManifest.version,
    viewport: { width: 640, height: 480, device_scale_factor: 1 },
    window: dimensions,
    masks,
    factions: results,
    wasm_sha256: sha256(fs.readFileSync(path.join(site, "open-rebellion-test.wasm"))),
  };
  fs.writeFileSync(path.join(runDir, "result.json"), `${JSON.stringify(summary, null, 2)}\n`);
  if (!passed) throw new Error(JSON.stringify(results.filter((result) => result.status !== "pass"), null, 2));
  process.stdout.write(`${JSON.stringify({ run_dir: runDir, status: summary.status, cases: results.map(({ faction, case: name, rect, comparison, closes }) => ({ faction, case: name, rect, comparison, closes })) }, null, 2)}\n`);
}

await main();
