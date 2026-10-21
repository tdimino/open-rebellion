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
import { sectorDoubleClick } from "./sector-double-click.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const site = process.env.OPEN_REBELLION_INTERFACE_SITE
  ? path.resolve(process.env.OPEN_REBELLION_INTERFACE_SITE)
  : path.join(root, ".artifacts/interface-parity/site");
const browserManifest = JSON.parse(fs.readFileSync(path.join(here, "browser.json"), "utf8"));
const noBuild = process.argv.includes("--no-build");
// `--only=alliance/chrome` runs the named cases alone.
const only = process.argv.filter((arg) => arg.startsWith("--only=")).map((arg) => arg.slice(7));
const expectedRequests = ["/", "/data/runtime.orpk", "/gl.js", "/open-rebellion-test.wasm"];
// Fixture codes are the Scenario index plus one (interface_test_fixture.rs).
const FLEET_FINDER = 53;
// FUN_0042a0c0: the Fleet Finder, 470 by 330 (FUN_00461750).
const windowSize = { width: 470, height: 330 };
// FUN_00461960, by side: the base 10335/10336, the Fleet Finder panel
// 10526/10527 or the Ship Finder panel 10524/10525 at (12, 13), the rail
// 10586/10590 at (412, 0), and the Close, Display, Ship Finder and Fleet
// Finder buttons (normal, pressed) at their rectangles. The side tabs are
// the same on both sides.
const factions = [
  { name: "alliance", byte: 1, base: 10335, fleetPanel: 10526, shipPanel: 10524, rail: 10586,
    close: { x: 423, y: 25, art: [10514, 10515] }, display: { x: 423, y: 93, art: [10518, 10519] },
    shipFinder: { x: 423, y: 147, art: [10530, 10531] }, fleetFinder: { x: 423, y: 201, art: [10528, 10529] },
    button: { width: 32, height: 31 } },
  { name: "empire", byte: 2, base: 10336, fleetPanel: 10527, shipPanel: 10525, rail: 10590,
    close: { x: 426, y: 21, art: [10516, 10517] }, display: { x: 426, y: 89, art: [10520, 10521] },
    shipFinder: { x: 426, y: 143, art: [10534, 10535] }, fleetFinder: { x: 426, y: 197, art: [10532, 10533] },
    button: { width: 44, height: 41 } },
];
const tabs = [
  { x: 36, art: [10500, 10501] },
  { x: 88, art: [10502, 10503] },
  { x: 140, art: [10505, 10506] },
];
// Text the port draws with its own font, the name box and its cursor, and
// the list's rows stay out of the pixel check.
const masks = [
  { x: 36, y: 12, width: 350, height: 20, why: "title text" },
  { x: 36, y: 46, width: 105, height: 16, why: "name label" },
  { x: 143, y: 45, width: 250, height: 16, why: "name box" },
  { x: 40, y: 119, width: 283, height: 18, why: "tab label" },
  { x: 36, y: 138, width: 350, height: 165, why: "list rows" },
];
const runId = `${new Date().toISOString().replace(/[:.]/g, "-")}-${process.pid}`;
const runDir = path.join(root, ".artifacts/interface-parity", `fleet-finder-${runId}`);

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

function sourceDirectory() {
  const candidates = [
    process.env.REBELLION_STRATEGY_BMP_DIR,
    path.join(root, "data/base/ui/strategy-dll/BMP"),
  ].filter(Boolean);
  const directory = candidates.find((candidate) => fs.existsSync(path.join(candidate, "10335.bmp")));
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

function decodeIndexedBmp(bytes) {
  assert.equal(bytes.toString("ascii", 0, 2), "BM", "source resource is not a BMP");
  const dataOffset = bytes.readUInt32LE(10);
  const headerSize = bytes.readUInt32LE(14);
  const width = bytes.readInt32LE(18);
  const signedHeight = bytes.readInt32LE(22);
  const height = Math.abs(signedHeight);
  assert.deepEqual([bytes.readUInt16LE(28), bytes.readUInt32LE(30)], [8, 0],
    "source resource is not an uncompressed 8-bit BMP");
  const paletteOffset = 14 + headerSize;
  const stride = (width + 3) & ~3;
  assert.ok(bytes.length >= dataOffset + stride * height);
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

const used = new Set();

function resource(source, id) {
  used.add(id);
  return decodeIndexedBmp(fs.readFileSync(path.join(source, `${id}.bmp`)));
}

// The palette-blue matte the buttons carry (bmp_cache.rs,
// uses_blue_screen_transparency).
function blueKey(data, offset) {
  return data[offset] < 32 && data[offset + 1] < 32 && data[offset + 2] > 192;
}

// FUN_00602d30 blits at the bitmap's own size; a control and the window
// clip the rest.
function blit(destination, image, x, y, { width = image.width, height = image.height, keyed = false } = {}) {
  for (let row = 0; row < Math.min(height, image.height, destination.height - y); row += 1) {
    for (let column = 0; column < Math.min(width, image.width, destination.width - x); column += 1) {
      const from = (row * image.width + column) * 4;
      if (keyed && blueKey(image.data, from)) continue;
      const to = ((y + row) * destination.width + x + column) * 4;
      destination.data.set(image.data.subarray(from, from + 4), to);
    }
  }
}

// The window as FUN_00461960 builds it in `mode` on tab `tab`: the base,
// the mode's panel and the rail, then the buttons keyed. hyp: the current
// tab and mode show their second bitmap (fleet_finder.rs).
function composeExpected(source, faction, { mode, tab }) {
  const expected = new PNG(windowSize);
  blit(expected, resource(source, faction.base), 0, 0);
  blit(expected, resource(source, mode === "ships" ? faction.shipPanel : faction.fleetPanel), 12, 13);
  blit(expected, resource(source, faction.rail), 412, 0);
  tabs.forEach((control, index) => {
    blit(expected, resource(source, control.art[index === tab ? 1 : 0]), control.x, 78,
      { width: 49, height: 41, keyed: true });
  });
  const button = (control, down) => blit(expected, resource(source, control.art[down ? 1 : 0]),
    control.x, control.y, { ...faction.button, keyed: true });
  button(faction.shipFinder, mode === "ships");
  button(faction.fleetFinder, mode === "fleets");
  button(faction.close, false);
  button(faction.display, false);
  return expected;
}

function crop(screenshotBytes, origin) {
  const screenshot = PNG.sync.read(screenshotBytes);
  assert.deepEqual([screenshot.width, screenshot.height], [640, 480]);
  const window = new PNG(windowSize);
  for (let y = 0; y < windowSize.height; y += 1) {
    const start = ((origin.y + y) * screenshot.width + origin.x) * 4;
    window.data.set(screenshot.data.subarray(start, start + windowSize.width * 4), y * windowSize.width * 4);
  }
  return window;
}

function compare(actual, expected, label, directory) {
  let different = 0;
  let checked = 0;
  const diff = new PNG(windowSize);
  for (let y = 0; y < expected.height; y += 1) {
    for (let x = 0; x < expected.width; x += 1) {
      const offset = (y * expected.width + x) * 4;
      if (masks.some((mask) => x >= mask.x && x < mask.x + mask.width && y >= mask.y && y < mask.y + mask.height)) {
        diff.data.set([0, 0, 96, 255], offset);
        continue;
      }
      checked += 1;
      const matches = actual.data[offset] === expected.data[offset]
        && actual.data[offset + 1] === expected.data[offset + 1]
        && actual.data[offset + 2] === expected.data[offset + 2];
      if (!matches) different += 1;
      diff.data.set(matches ? [0, 0, 0, 255] : [255, 0, 80, 255], offset);
    }
  }
  fs.writeFileSync(path.join(directory, `${label}-actual.png`), PNG.sync.write(actual));
  fs.writeFileSync(path.join(directory, `${label}-expected.png`), PNG.sync.write(expected));
  fs.writeFileSync(path.join(directory, `${label}-diff.png`), PNG.sync.write(diff));
  return { label, pixels_checked: checked, different_pixels: different };
}

async function frames(page, count = 1) {
  for (let index = 0; index < count; index += 1) {
    await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  }
}

async function click(page, point) {
  // egui resolves a click across frames: hover, press, then release.
  await page.mouse.move(point.x, point.y);
  await frames(page);
  await page.mouse.down();
  await frames(page);
  await page.mouse.up();
  await frames(page);
}

async function key(page, name) {
  await page.keyboard.down(name);
  await frames(page);
  await page.keyboard.up(name);
  await frames(page);
}

function observations(page) {
  return page.evaluate(() => window.__openRebellionInterfaceFleetFinders || []);
}

async function latest(page) {
  const list = await observations(page);
  assert.ok(list.length > 0, "the fixture reported no fleet-finder observation");
  return list[list.length - 1];
}

// Wait until the latest observation meets `predicate(observation, argument)`.
async function until(page, description, predicate, argument = null, timeout = 10_000) {
  try {
    await page.waitForFunction(
      ([source, value]) => {
        const last = (window.__openRebellionInterfaceFleetFinders || []).at(-1);
        return last && new Function("o", "a", `return (${source})(o, a);`)(last, value);
      },
      [predicate.toString(), argument],
      { timeout, polling: 100 },
    );
  } catch (error) {
    throw new Error(`${description}: ${JSON.stringify((await observations(page)).slice(-3))}`, { cause: error });
  }
  return latest(page);
}

// Let an ignored action settle.
async function settle(page) {
  await page.waitForTimeout(250);
  await frames(page, 3);
  return latest(page);
}

function point([x, y]) {
  return { x: Math.round(x), y: Math.round(y) };
}

async function shot(page, directory, label) {
  const bytes = await page.screenshot({ animations: "disabled" });
  fs.writeFileSync(path.join(directory, `${label}.png`), bytes);
  return { label, sha256: sha256(bytes), bytes };
}

async function stableWindow(page, origin, directory, label) {
  // The first draw requests each newly shown bitmap from the WASM texture
  // cache; let the upload reach a later paint before hashing two frames.
  await page.mouse.move(2, 2);
  await page.waitForTimeout(150);
  await frames(page);
  const first = await page.screenshot({ animations: "disabled" });
  await frames(page);
  const second = await page.screenshot({ animations: "disabled" });
  fs.writeFileSync(path.join(directory, `${label}-screen.png`), second);
  const masked = (bytes) => {
    const window = crop(bytes, origin);
    for (const mask of masks) {
      for (let y = mask.y; y < mask.y + mask.height; y += 1) {
        window.data.fill(0, (y * windowSize.width + mask.x) * 4, (y * windowSize.width + mask.x + mask.width) * 4);
      }
    }
    return sha256(PNG.sync.write(window));
  };
  // The name box's cursor blinks inside its mask.
  assert.equal(masked(first), masked(second), `the Fleet Finder did not settle (${label})`);
  return crop(second, origin);
}

function checked(check) {
  assert.equal(check.different_pixels, 0, `${check.label}: ${check.different_pixels} of ${check.pixels_checked} pixels differ`);
  // 155,100 window pixels less the masks' 75,524.
  assert.equal(check.pixels_checked, 79_576, `${check.label}: the masks changed`);
}

// The cockpit's Fleet Finder control (0x12e, FUN_00427270) opens the Finder
// in Fleet Finder mode on its All tab (FUN_0042a0c0, FUN_00461960).
async function openFinder(page) {
  await click(page, point((await latest(page)).cockpit_button));
  const opened = await until(page, "the cockpit control opens the Fleet Finder", (o) => o.open);
  assert.equal(opened.mode, "fleets");
  assert.equal(opened.tab, "all");
  assert.equal(opened.name, "");
  assert.equal(opened.chosen, null);
  assert.ok(opened.rows.length > 1, "the Finder lists the fixture's fleets");
  return opened;
}

// FUN_0060a890 mode 2: the rows in _stricmp order.
function assertSorted(rows) {
  const names = rows.map((row) => row.name.toLowerCase());
  assert.deepEqual(names, [...names].sort((a, b) => (a < b ? -1 : a > b ? 1 : 0)), JSON.stringify(rows));
}

// FUN_00609650: the first row with the longest common prefix, ignoring case.
function prefixChoice(rows, text) {
  let best = null;
  rows.forEach((row, index) => {
    let common = 0;
    while (common < Math.min(text.length, row.name.length)
      && row.name[common].toLowerCase() === text[common].toLowerCase()) common += 1;
    if (common > 0 && (best === null || common > best.common)) best = { index, common };
  });
  return best?.index ?? null;
}

// FUN_00429440: the row's system's sector window opens, and from its fleet
// icon the Fleet window, with the row's fleet or ship selected.
function assertOpened(observed, row) {
  assert.equal(observed.open, false, "the Finder closes after it opens a window");
  assert.ok(observed.sector_open.includes(row.system_dat_id), `the sector window opens: ${JSON.stringify(observed)}`);
  const window = observed.fleet_windows.find((entry) => entry.system_dat_id === row.system_dat_id);
  assert.ok(window, `the Fleet window opens: ${JSON.stringify(observed)}`);
  assert.deepEqual([window.selected, window.fleet, window.ship], [row.kind, row.fleet, row.ship]);
  assert.equal(observed.selected_system_dat_id, row.system_dat_id);
}

// Each case starts from a fresh load and returns its checks.
const cases = [
  {
    name: "chrome",
    async run(page, faction, directory, source) {
      const opened = await openFinder(page);
      assertSorted(opened.rows);
      const origin = { x: opened.controls.origin[0], y: opened.controls.origin[1] };
      const fleets = compare(await stableWindow(page, origin, directory, "fleets"),
        composeExpected(source, faction, { mode: "fleets", tab: 0 }), "fleets", directory);
      await click(page, point(opened.controls.tabs[2]));
      const imperial = await until(page, "the Imperial tab shows", (o) => o.tab === "imperial");
      const imperialCheck = compare(await stableWindow(page, origin, directory, "imperial"),
        composeExpected(source, faction, { mode: "fleets", tab: 2 }), "imperial", directory);
      await click(page, point(opened.controls.ship_finder));
      const ships = await until(page, "the Ship Finder shows", (o) => o.mode === "ships");
      assert.equal(ships.tab, "imperial", "the mode keeps the tab (FUN_004632d0)");
      const shipsCheck = compare(await stableWindow(page, origin, directory, "ships"),
        composeExpected(source, faction, { mode: "ships", tab: 2 }), "ships", directory);
      await click(page, point(opened.controls.close));
      const closed = await until(page, "Close closes the Finder", (o) => !o.open);
      assert.deepEqual(closed.fleet_windows, []);
      for (const check of [fleets, imperialCheck, shipsCheck]) checked(check);
      return { opened, imperial, ships, closed, compares: [fleets, imperialCheck, shipsCheck] };
    },
  },
  {
    name: "tabs",
    // FUN_00462be0: tab 2 lists side bit 0x40, tab 3 side bit 0x80; a tab
    // change clears the choice.
    async run(page) {
      const opened = await openFinder(page);
      const selectedBefore = opened.selected_system_dat_id;
      const sectorBefore = opened.sector_open;
      assert.ok(opened.rows.some((row) => row.is_alliance) && opened.rows.some((row) => !row.is_alliance),
        "the All tab lists both sides");
      await click(page, point(opened.controls.rows[0]));
      const chosen = await until(page, "the row is chosen", (o) => o.chosen === 0);
      await click(page, point(opened.controls.tabs[1]));
      const alliance = await until(page, "the Alliance tab shows", (o) => o.tab === "alliance");
      assert.equal(alliance.chosen, null);
      assert.ok(alliance.rows.length > 0 && alliance.rows.every((row) => row.is_alliance), JSON.stringify(alliance.rows));
      await click(page, point(opened.controls.tabs[2]));
      const imperial = await until(page, "the Imperial tab shows", (o) => o.tab === "imperial");
      assert.ok(imperial.rows.length > 0 && imperial.rows.every((row) => !row.is_alliance), JSON.stringify(imperial.rows));
      assert.equal(alliance.rows.length + imperial.rows.length, opened.rows.length);
      // The galaxy map and authentic sector window under the Finder take none
      // of its clicks.
      assert.equal(imperial.selected_system_dat_id, selectedBefore);
      assert.deepEqual(imperial.sector_open, sectorBefore);
      return { opened, chosen, alliance, imperial };
    },
  },
  {
    name: "sector-occlusion",
    // A Finder empty-list double click must belong only to the top-level
    // Finder, even when the same pixels cover another planet in the original
    // modeless sector window.
    async run(page) {
      const opened = await openFinder(page);
      // Use the shorter side tab so the fixture always has actual empty list
      // space, even when the Alliance's All tab fills all eight visible rows.
      const shorterTab = [
        { tab: "alliance", index: 1, count: opened.rows.filter((row) => row.is_alliance).length },
        { tab: "imperial", index: 2, count: opened.rows.filter((row) => !row.is_alliance).length },
      ].sort((left, right) => left.count - right.count)[0];
      await click(page, point(opened.controls.tabs[shorterTab.index]));
      const filtered = await until(
        page,
        `the ${shorterTab.tab} tab is shown`,
        (o, tab) => o.tab === tab,
        shorterTab.tab,
      );
      const origin = { x: filtered.controls.origin[0], y: filtered.controls.origin[1] };
      const firstEmptyRowY = origin.y + 138 + filtered.rows.length * 20 + 2;
      const listBottom = origin.y + 303;
      const target = filtered.sector_planets.find((planet) => {
        const [x, y] = planet.center;
        return planet.system_dat_id !== filtered.selected_system_dat_id
          && x >= origin.x + 36 && x < origin.x + 386
          && y >= firstEmptyRowY && y < listBottom;
      });
      assert.ok(target, `no covered sector planet lies under empty Finder space: ${JSON.stringify(filtered)}`);

      await sectorDoubleClick(page, point(target.center), () => frames(page));
      await key(page, "Escape");
      const closed = await until(page, "Escape closes the Finder", (o) => !o.open);
      assert.equal(closed.selected_system_dat_id, filtered.selected_system_dat_id,
        "an empty Finder double click changed the covered sector selection");
      assert.deepEqual(closed.sector_open, filtered.sector_open,
        "an empty Finder double click changed the covered sector windows");
      return { opened, filtered, target, closed };
    },
  },
  {
    name: "fleet-overlap",
    // Opening a result creates the original Fleet window. Reopening the
    // Finder must keep its covered right-side controls above that modeless
    // Foreground window.
    async run(page) {
      const opened = await openFinder(page);
      await click(page, point(opened.controls.rows[0]));
      const chosen = await until(page, "the first fleet is chosen", (o) => o.chosen === 0);
      const row = chosen.rows[0];
      await click(page, point(chosen.controls.display));
      const fleet = await until(page, "Display opens the Fleet window", (o) => !o.open);
      assertOpened(fleet, row);

      await key(page, "F3");
      const reopened = await until(page, "F3 reopens above the Fleet window", (o) => o.open);
      assert.ok(reopened.fleet_windows.length > 0);
      // A Finder control the Fleet window covers: below the Finder, the
      // window would take this click (egui keeps the older of two raised
      // Foreground layers below, and the Finder is the older).
      const covered = (at) => reopened.fleet_windows.some(({ rect: [x, y, width, height] }) =>
        at[0] >= x && at[0] < x + width && at[1] >= y && at[1] < y + height);
      const candidates = [
        { name: "ship_finder", at: reopened.controls.ship_finder, done: (o) => o.mode === "ships" },
        { name: "alliance_tab", at: reopened.controls.tabs[1], done: (o) => o.tab === "alliance" },
        { name: "imperial_tab", at: reopened.controls.tabs[2], done: (o) => o.tab === "imperial" },
        { name: "close", at: reopened.controls.close, done: (o) => !o.open },
      ];
      const control = candidates.find((candidate) => covered(candidate.at));
      assert.ok(control, `no Finder control lies under the Fleet window: ${JSON.stringify(reopened)}`);
      await click(page, point(control.at));
      const responded = await until(page, `the covered ${control.name} control responds`, control.done);
      assert.ok(responded.fleet_windows.length > 0);
      return { opened, chosen, fleet, reopened, control: control.name, responded };
    },
  },
  {
    name: "type-and-enter",
    // F3 posts 0x12e (FUN_00422ce0); the name box has the focus; a typed
    // name chooses the row it best begins (FUN_00462a50 0x408) and Enter
    // opens it (0x407).
    async run(page) {
      // F3 is handled by the game canvas; focus it explicitly rather than
      // relying on a coordinate click or the browser's page-level focus.
      await page.locator("#glcanvas").focus();
      assert.equal(await page.evaluate(() => document.activeElement?.id), "glcanvas");
      await page.keyboard.press("F3");
      await frames(page, 3);
      const opened = await until(page, "F3 opens the Fleet Finder", (o) => o.open);
      const text = "fleet 2";
      await page.keyboard.type(text, { delay: 40 });
      const typed = await until(page, "the name box takes the text", (o, a) => o.name === a, text);
      const expected = prefixChoice(typed.rows, text);
      assert.notEqual(expected, null, JSON.stringify(typed.rows));
      assert.equal(typed.chosen, expected);
      assert.equal(typed.left_panel_open, false, "typed letters reach the name box, not the galaxy view's keys");
      const row = typed.rows[typed.chosen];
      await page.keyboard.press("Enter");
      await frames(page, 3);
      const done = await until(page, "Enter opens the chosen fleet", (o) => !o.open);
      assertOpened(done, row);
      return { opened, typed, done };
    },
  },
  {
    name: "click-and-display",
    // FUN_00462770: a click (0x29b) chooses the row and copies its name;
    // Display (0xc9) opens it. The other side's fleet at the player's
    // system is listed, since the player holds the system.
    async run(page) {
      const opened = await openFinder(page);
      await click(page, point(opened.controls.display));
      const nothing = await settle(page);
      assert.equal(nothing.open, true, "Display with nothing chosen keeps the Finder open");
      const index = opened.rows.findIndex((row) => row.is_alliance !== opened.player_is_alliance);
      assert.ok(index >= 0 && opened.controls.rows[index], JSON.stringify(opened.rows));
      const row = opened.rows[index];
      await click(page, point(opened.controls.rows[index]));
      const chosen = await until(page, "the click chooses the row", (o, a) => o.chosen === a, index);
      assert.equal(chosen.name, row.name);
      await click(page, point(opened.controls.display));
      const done = await until(page, "Display opens the fleet", (o) => !o.open);
      assertOpened(done, row);
      return { opened, nothing, chosen, done };
    },
  },
  {
    name: "double-click",
    // FUN_00462770: a double click (0x309) opens the row.
    async run(page) {
      const opened = await openFinder(page);
      const index = opened.rows.findIndex((row) => row.is_alliance === opened.player_is_alliance);
      await sectorDoubleClick(page, point(opened.controls.rows[index]), () => frames(page));
      const done = await until(page, "the double click opens the fleet", (o) => !o.open);
      assertOpened(done, opened.rows[index]);
      return { opened, done };
    },
  },
  {
    name: "ship",
    // Mode 2 lists capital ships by name; opening one selects the ship in
    // its fleet's window (FUN_00429440, slot +0x6c).
    async run(page) {
      const opened = await openFinder(page);
      await page.keyboard.type("x");
      await until(page, "the name box takes the text", (o) => o.name === "x");
      await click(page, point(opened.controls.ship_finder));
      const ships = await until(page, "the Ship Finder shows", (o) => o.mode === "ships");
      assert.equal(ships.name, "", "the mode change empties the name box (FUN_004632d0)");
      assert.ok(ships.rows.length > 0 && ships.rows.every((row) => row.kind === "ship"), JSON.stringify(ships.rows));
      assertSorted(ships.rows);
      await click(page, point(ships.controls.rows[0]));
      await until(page, "the click chooses the ship", (o) => o.chosen === 0);
      await click(page, point(ships.controls.display));
      const done = await until(page, "Display opens the ship's fleet", (o) => !o.open);
      assertOpened(done, ships.rows[0]);
      return { opened, ships, done };
    },
  },
  {
    name: "escape",
    // FUN_00463360: Escape closes the Finder, and only the Finder: the
    // cockpit control opens it again.
    async run(page) {
      const opened = await openFinder(page);
      await key(page, "Escape");
      const closed = await until(page, "Escape closes the Finder", (o) => !o.open);
      const reopened = await openFinder(page);
      return { opened, closed, reopened };
    },
  },
];

async function inspect(server, source, faction, testCase, executable) {
  const directory = path.join(runDir, `${faction.name}-${testCase.name}`);
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
    page.on("requestfailed", (request) => {
      errors.push(`requestfailed:${request.url()}:${request.failure()?.errorText}`);
    });
    page.on("pageerror", (error) => errors.push(`pageerror:${error.stack || error.message}`));
    page.on("console", (message) => {
      consoleLines.push({ type: message.type(), text: message.text() });
      if (message.type() === "error" || /\[bmp_cache\] asset unavailable/i.test(message.text())) {
        errors.push(`console:${message.type()}:${message.text()}`);
      }
    });

    const fixtureCode = FLEET_FINDER | (faction.byte << 8);
    await page.goto(`${serverOrigin}/?fixture-code=${fixtureCode}`, { waitUntil: "load", timeout: 30_000 });
    await page.waitForFunction(() => window.__openRebellionInterfaceReady?.status, null, { timeout: 30_000 });
    const ready = await page.evaluate(() => window.__openRebellionInterfaceReady);
    assert.equal(ready.status, "ready", JSON.stringify(ready));
    assert.equal(ready.code, fixtureCode);
    await page.waitForFunction(() => (window.__openRebellionInterfaceFleetFinders || []).length > 0,
      null, { timeout: 10_000 });
    const start = await latest(page);
    assert.equal(start.code, fixtureCode);
    assert.equal(start.open, false, JSON.stringify(start));
    await page.evaluate(() => document.fonts.ready);
    const { bytes: _ready, ...before } = await shot(page, directory, "ready");

    const checks = await testCase.run(page, faction, directory, source);
    await page.mouse.move(2, 2);
    await frames(page);
    const { bytes: _final, ...after } = await shot(page, directory, "final");
    assert.deepEqual(requests.map(({ url }) => url).sort(), [...expectedRequests].sort());
    assert.ok(requests.every(({ status }) => status === 200), "startup has non-200 requests");
    assert.deepEqual(errors, [], `browser diagnostics: ${errors.join("; ")}`);
    result = {
      status: "pass",
      faction: faction.name,
      case: testCase.name,
      fixture_code: fixtureCode,
      ready,
      checks,
      observations: await observations(page),
      screenshots: [before, after],
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
      case: testCase.name,
      error: String(error.stack || error),
      observations: page ? await observations(page).catch(() => []) : [],
      requests,
      errors,
      console: consoleLines,
      launch_attempts: launchAttempts,
      cleanup: "pending",
    };
    if (page) await shot(page, directory, "failure").catch(() => {});
  } finally {
    if (page) await page.close().catch((error) => errors.push(`page-close:${error}`));
    if (context) await context.close().catch((error) => errors.push(`context-close:${error}`));
    if (browser) await browser.close().catch((error) => errors.push(`browser-close:${error}`));
    result.cleanup = errors.some((error) => error.includes("-close:")) ? "failed" : "closed";
    if (result.cleanup === "failed") result.status = "fail";
  }
  return result;
}

function sourceIdentity(source) {
  const ids = [...used].sort((left, right) => left - right);
  const aggregate = createHash("sha256");
  for (const id of ids) {
    aggregate.update(`${id}\0`);
    aggregate.update(fs.readFileSync(path.join(source, `${id}.bmp`)));
  }
  return { dll: "STRATEGY.DLL", resource_ids: ids, aggregate_sha256: aggregate.digest("hex") };
}

async function main() {
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
  const source = sourceDirectory();
  const executable = browserExecutable();
  const server = await startServer();
  let results;
  try {
    results = [];
    for (const faction of factions) {
      for (const testCase of cases) {
        if (only.length && !only.includes(`${faction.name}/${testCase.name}`)) continue;
        results.push(await inspect(server, source, faction, testCase, executable));
      }
    }
  } finally {
    await new Promise((resolve) => server.close(resolve));
  }
  const passed = results.every((result) => result.status === "pass");
  const summary = {
    schema_version: 1,
    family: "fleet-finder",
    scope: "test-only Fleet Finder (window type 0x15) through its original entries, the cockpit's Fleet Finder control and F3: chrome in both modes against STRATEGY.DLL, the side tabs, a typed name and Enter, a row click and Display, a double click, the Ship Finder opening a ship in its fleet's window, Close and Escape, on both sides",
    status: passed ? "pass" : "fail",
    browser_version: browserManifest.version,
    browser_executable: executable,
    launch_arguments: browserManifest.launch_arguments,
    profile: "one-new-muted-process-per-faction-and-case",
    muted: browserManifest.launch_arguments.includes("--mute-audio"),
    viewport: { width: 640, height: 480, device_scale_factor: 1 },
    masks,
    source: sourceIdentity(source),
    results,
    wasm_sha256: sha256(fs.readFileSync(path.join(site, "open-rebellion-test.wasm"))),
    runtime_pack_sha256: sha256(fs.readFileSync(path.join(site, "data/runtime.orpk"))),
  };
  fs.writeFileSync(path.join(runDir, "result.json"), `${JSON.stringify(summary, null, 2)}\n`);
  if (!passed) {
    throw new Error(JSON.stringify(results.filter((result) => result.status !== "pass")
      .map(({ faction, case: name, error, observations: list }) => ({ faction, case: name, error, observations: (list || []).slice(-4) })), null, 2));
  }
  process.stdout.write(`${JSON.stringify({ run_dir: runDir, status: summary.status, cases: results.map((r) => `${r.faction}/${r.case}`), wasm_sha256: summary.wasm_sha256 }, null, 2)}\n`);
}

await main();
