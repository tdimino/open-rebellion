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
// `--only=alliance/load` runs the named cases alone.
const only = process.argv.filter((arg) => arg.startsWith("--only=")).map((arg) => arg.slice(7));
const expectedRequests = ["/", "/data/runtime.orpk", "/gl.js", "/open-rebellion-test.wasm"];
// Fixture codes are the Scenario index plus one (interface_test_fixture.rs).
const FLEET_LOAD = 48;
const FLEET_LOAD_FULL = 49;
const REGIMENT_UNLOAD_REFUSED = 51;
const FLEET_JOIN = 52;
const PRODUCTION_DESTINATION = 54;
// FUN_004a2630: the Fleet window, 235 by 304 (background 10770).
const windowSize = { width: 235, height: 304 };
// Side art: the Alliance's resources, the Empire's 50 higher
// (FUN_004a4b10); the title strip is the active one of the viewer's side
// (FUN_004a3340), since the viewer's fleet is listed. `sectors` are the two
// sector window columns, 235 wide (sector_window.rs,
// window_logical_position); the fixture fills both. `rail` is the galaxy
// view rail's first slot (system_window.rs, rail_slot_rect).
// `galaxy` is the galaxy view, which centers the move confirmation
// (fleet-move.mjs).
const factions = [
  { name: "alliance", byte: 1, art: 0, title: 10299, sectors: [60, 300], rail: { x: 544, y: 61, width: 62, height: 18 },
    galaxy: { x: 55, y: 40, width: 485, height: 350 } },
  { name: "empire", byte: 2, art: 50, title: 10201, sectors: [120, 365], rail: { x: 21, y: 48, width: 54, height: 18 },
    galaxy: { x: 120, y: 40, width: 480, height: 355 } },
];
// FUN_00487cc0: the move confirmation, 424 by 331, and its checkmark
// (0x14, fleet-move.mjs).
const confirmation = { width: 424, height: 331 };
const checkmark = { x: 355 + 25.5, y: 244 + 17.5 };
// Text the port draws with its own font, the tree's dotted pen, and the
// right list's GOKRES minis and names stay out of the pixel check.
const masks = {
  open: [
    { x: 18, y: 2, width: 186, height: 16, why: "title text" },
    { x: 8, y: 33, width: 60, height: 13, why: "fleet label" },
    { x: 5, y: 34, width: 5, height: 3, why: "dotted tree line" },
  ],
  selected: [
    { x: 100, y: 28, width: 130, height: 13, why: "selected fleet name" },
    { x: 101, y: 127, width: 133, height: 164, why: "right list contents" },
  ],
};
const runId = `${new Date().toISOString().replace(/[:.]/g, "-")}-${process.pid}`;
const runDir = path.join(root, ".artifacts/interface-parity", `fleet-window-${runId}`);

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

function sourceDirectory() {
  const candidates = [
    process.env.REBELLION_STRATEGY_BMP_DIR,
    path.join(root, "data/base/ui/strategy-dll/BMP"),
  ].filter(Boolean);
  const directory = candidates.find((candidate) => fs.existsSync(path.join(candidate, "10770.bmp")));
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

// The palette-blue matte the overlays carry (bmp_cache.rs,
// uses_blue_screen_transparency).
function blueKey(data, offset) {
  return data[offset] < 32 && data[offset + 1] < 32 && data[offset + 2] > 192;
}

// FUN_00602d30 blits at the bitmap's own size; a control clips the rest.
function blit(destination, image, x, y, { width = image.width, height = image.height, keyed = false } = {}) {
  for (let row = 0; row < Math.min(height, image.height); row += 1) {
    for (let column = 0; column < Math.min(width, image.width); column += 1) {
      const from = (row * image.width + column) * 4;
      if (keyed && blueKey(image.data, from)) continue;
      const to = ((y + row) * destination.width + x + column) * 4;
      destination.data.set(image.data.subarray(from, from + 4), to);
    }
  }
}

// The window as FUN_004a4b10 builds it with nothing selected, and with the
// fleet selected on its Capital Ships tab: pane art 10407 at (97, 29), the
// selection 10401 behind frame 10400 at the list item's (0, 0) and (5, 5),
// the picture 10425 centred in the 125-wide panel at (100, 42), and the tab
// strip from base 10409 (normal b+k, disabled b+3+k, selected b+7+k), each
// tab clipped to its 31 by 28 control at (99 + 1/33/65/97, 96).
function composeExpected(source, faction, state) {
  const expected = resource(source, 10770);
  assert.deepEqual([expected.width, expected.height], [windowSize.width, windowSize.height]);
  blit(expected, resource(source, faction.title), 2, 2);
  blit(expected, resource(source, 10209), 3, 3);
  blit(expected, resource(source, 10253), 204, 3);
  blit(expected, resource(source, 10108), 218, 3);
  if (state.selected) {
    blit(expected, resource(source, 10407 + faction.art), 97, 29);
    blit(expected, resource(source, 10401 + faction.art), 4, 29, { keyed: true });
  }
  blit(expected, resource(source, 10400 + faction.art), 9, 34, { keyed: true });
  if (state.selected) {
    const picture = resource(source, 10425 + faction.art);
    blit(expected, picture, 100 + Math.floor((125 - picture.width) / 2), 42, { keyed: true });
    const base = 10409 + faction.art;
    [1, 33, 65, 97].forEach((x, k) => {
      const id = !state.enabled[k] ? base + 3 + k : k === state.tab ? base + 7 + k : base + k;
      blit(expected, resource(source, id), 99 + x, 96, { width: 31, height: 28 });
    });
  }
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

function compare(actual, expected, regions, label, directory) {
  let different = 0;
  let checked = 0;
  const diff = new PNG(windowSize);
  for (let y = 0; y < expected.height; y += 1) {
    for (let x = 0; x < expected.width; x += 1) {
      const offset = (y * expected.width + x) * 4;
      if (regions.some((mask) => x >= mask.x && x < mask.x + mask.width && y >= mask.y && y < mask.y + mask.height)) {
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

async function click(page, point, button = "left") {
  // egui resolves a click across frames: hover, press, then release.
  await page.mouse.move(point.x, point.y);
  await frames(page);
  await page.mouse.down({ button });
  await frames(page);
  await page.mouse.up({ button });
  await frames(page);
}

async function press(page, name) {
  await page.keyboard.down(name);
  await frames(page);
  await page.keyboard.up(name);
  await frames(page);
}

function escapeRegExp(text) {
  return text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

// Two clicks inside egui's double-click time.
async function doubleClick(page, point) {
  await page.mouse.move(point.x, point.y);
  await frames(page);
  for (let index = 0; index < 2; index += 1) {
    await page.mouse.down();
    await frames(page);
    await page.mouse.up();
    await frames(page);
  }
}

function observations(page) {
  return page.evaluate(() => window.__openRebellionInterfaceFleetLoads || []);
}

async function latest(page) {
  const list = await observations(page);
  assert.ok(list.length > 0, "the fixture reported no fleet-load observation");
  return list[list.length - 1];
}

// Wait until the latest observation meets `predicate(observation, argument)`.
async function until(page, description, predicate, argument = null, timeout = 10_000) {
  try {
    await page.waitForFunction(
      ([source, value]) => {
        const last = (window.__openRebellionInterfaceFleetLoads || []).at(-1);
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

// Let a refused or ignored action settle.
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
  const a = crop(first, origin);
  const b = crop(second, origin);
  assert.equal(sha256(PNG.sync.write(a)), sha256(PNG.sync.write(b)), `the Fleet window did not settle (${label})`);
  return b;
}

// The sector window's fleet icon opens the Fleet window (FUN_0045ccc0,
// FUN_0045aac0, kind 0x10) at the double-click.
async function openFleetWindow(page, setup, entries = 1) {
  await doubleClick(page, point(setup.icon));
  const observed = await until(page, "the fleet icon opens the Fleet window", (o) => o.window_open);
  assert.equal(observed.entries, entries, "the window lists the system's fleets");
  assert.equal(observed.selected, null);
  return observed;
}

async function selectFleet(page, observed) {
  await click(page, point(observed.fleet_entry));
  return until(page, "a click selects the fleet", (o) => o.selected === "fleet");
}

async function openMenu(page, at, directory, label) {
  await page.evaluate(() => { window.__openRebellionInterfaceObjectMenu = undefined; });
  await click(page, at, "right");
  await page.waitForFunction(() => window.__openRebellionInterfaceObjectMenu?.status === "object-menu",
    null, { timeout: 10_000 });
  const menu = await page.evaluate(() => window.__openRebellionInterfaceObjectMenu);
  fs.writeFileSync(path.join(directory, `${label}-menu.png`), await page.screenshot({ animations: "disabled" }));
  return menu;
}

// Rows below a 2-pixel border share one height: object menu entries carry
// no icon (object_menu.rs).
async function choose(page, menu, row, name) {
  assert.notEqual(row, null, `the menu lists ${name}`);
  const height = (menu.height - 2) / menu.rows;
  await click(page, { x: menu.left + menu.width / 2, y: menu.top + 2 + height * (row + 0.5) });
}

// The regiment's Move (FUN_00504b30) released on the Fleet window's fleet
// (+0x70, FUN_004a3130).
async function moveRegimentOntoFleet(page, setup, observed, directory) {
  const menu = await openMenu(page, point(setup.troop_item), directory, "regiment");
  // FUN_00504b30 lists Move first and Confirmed Move second.
  assert.deepEqual([menu.move_row, menu.confirmed_move_row], [0, 1], JSON.stringify(menu));
  await choose(page, menu, menu.move_row, "Move");
  await click(page, point(observed.fleet_entry));
  return menu;
}

// The browser has no keypad +, so the clock starts through the Game Speed
// menu on the day readout: Fast, its fifth row (FUN_0042d190).
async function runFast(page, setup) {
  await page.evaluate(() => { window.__openRebellionInterfaceSpeedMenu = undefined; });
  await click(page, point(setup.day_readout), "right");
  await page.waitForFunction(() => window.__openRebellionInterfaceSpeedMenu?.status === "speed-menu",
    null, { timeout: 10_000 });
  const menu = await page.evaluate(() => window.__openRebellionInterfaceSpeedMenu);
  const height = (menu.height - 2) / menu.rows;
  await click(page, { x: menu.left + menu.width / 2, y: menu.top + 2 + height * 4.5 });
  return menu;
}

async function loadRegiment(page, setup, directory) {
  const opened = await openFleetWindow(page, setup);
  await selectFleet(page, opened);
  const menu = await moveRegimentOntoFleet(page, setup, opened, directory);
  const loaded = await until(page, "the regiment boards the fleet", (o) => o.aboard && o.held);
  assert.equal(loaded.cargo, 1);
  assert.deepEqual(loaded.enabled, [true, false, true, false], "the Troops tab lights with a regiment aboard");
  return { opened, menu, loaded };
}

// A left press on a right-list item, held while the mouse moves away, then
// released at `to` (CoolDragList, FUN_006083c0, posts 0x29a).
async function dragItem(page, from, to) {
  await page.mouse.move(from.x, from.y);
  await frames(page);
  await page.mouse.down();
  await frames(page);
  await page.mouse.move(to.x, to.y, { steps: 4 });
  await frames(page);
  await page.mouse.up();
  await frames(page);
}

async function openTroopsTab(page, loaded) {
  await click(page, point(loaded.troops_tab));
  const troops = await until(page, "the Troops tab opens", (o) => o.tab === "Troops" && o.first_item);
  assert.equal(troops.items.length, 1, "the Troops tab lists the regiment");
  return troops;
}

function ships(observed) {
  return observed.fleets.map((fleet) => fleet.ships);
}

// The joining scenario's Fleet window: two fleets of the player's, of two
// ships and one, the first selected on its Capital Ships tab.
async function openJoining(page, setup) {
  const opened = await openFleetWindow(page, setup, 2);
  assert.deepEqual(ships(opened), [2, 1], JSON.stringify(opened));
  await click(page, point(opened.fleets[0].entry));
  const selected = await until(page, "a click selects the first fleet",
    (o) => o.selected === "fleet" && o.tab === "CapitalShips" && o.first_item);
  assert.equal(selected.items.length, 2, "the Capital Ships tab lists the fleet's two ships");
  return selected;
}

function confirmationCheckmark(faction) {
  const { galaxy } = faction;
  return {
    x: Math.round(galaxy.x + galaxy.width / 2 - confirmation.width / 2 + checkmark.x),
    y: Math.round(galaxy.y + galaxy.height / 2 - confirmation.height / 2 + checkmark.y),
  };
}

// Each case starts from a fresh load and returns its checks.
const cases = [
  {
    name: "chrome",
    code: FLEET_LOAD,
    async run(page, faction, setup, directory, source) {
      const opened = await openFleetWindow(page, setup);
      const origin = { x: opened.origin[0], y: opened.origin[1] };
      const openCheck = compare(await stableWindow(page, origin, directory, "open"),
        composeExpected(source, faction, { selected: false }), masks.open, "open", directory);
      const selected = await selectFleet(page, opened);
      assert.equal(selected.tab, "CapitalShips");
      assert.deepEqual(selected.enabled, [true, false, false, false]);
      assert.equal(selected.items.length, 1, "the fleet's one capital ship is listed");
      const selectedCheck = compare(await stableWindow(page, origin, directory, "selected"),
        composeExpected(source, faction, { selected: true, enabled: selected.enabled, tab: 0 }),
        [...masks.open, ...masks.selected], "selected", directory);
      // 0x280d minimizes the window to the rail (FUN_004a76e0); the rail
      // entry restores it where it was.
      await click(page, { x: origin.x + 204 + 7, y: origin.y + 3 + 7 });
      const minimized = await until(page, "the minimize button hides the window", (o) => !o.window_open);
      const { rail } = faction;
      await click(page, { x: rail.x + rail.width / 2, y: rail.y + rail.height / 2 });
      const restored = await until(page, "the rail restores the window", (o) => o.window_open);
      assert.deepEqual(restored.origin, opened.origin);
      for (const check of [openCheck, selectedCheck]) {
        assert.equal(check.different_pixels, 0, `${check.label}: ${check.different_pixels} of ${check.pixels_checked} pixels differ`);
      }
      return { opened, selected, compares: [openCheck, selectedCheck], minimized, restored };
    },
  },
  {
    name: "load",
    code: FLEET_LOAD,
    // The regiment stays aboard while the fleet orbits the system it loaded
    // at, though the clock runs (port: the hold, ghidra/notes/fleet-window.md).
    async run(page, faction, setup, directory) {
      const { loaded, menu } = await loadRegiment(page, setup, directory);
      await click(page, point(loaded.troops_tab));
      const troops = await until(page, "the Troops tab opens", (o) => o.tab === "Troops");
      assert.equal(troops.items.length, 1, "the Troops tab lists the regiment");
      assert.deepEqual(troops.counts, [1, setup.capacity]);
      await shot(page, directory, "troops-tab");
      const speed = await runFast(page, setup);
      await page.waitForTimeout(3_000);
      const held = await settle(page);
      assert.ok(held.aboard && held.held, `the regiment landed where it loaded: ${JSON.stringify(held)}`);
      assert.equal(held.in_transit, false);
      return { menu, loaded, troops, speed, held };
    },
  },
  {
    name: "move-and-land",
    code: FLEET_LOAD,
    // The fleet's Move to the target; on arrival the regiment lands on the
    // player's empty planet (apply_fleet_arrival releases the hold).
    async run(page, faction, setup, directory) {
      const { loaded } = await loadRegiment(page, setup, directory);
      const origin = { x: loaded.origin[0], y: loaded.origin[1] };
      await click(page, { x: origin.x + 218 + 7, y: origin.y + 3 + 7 });
      await until(page, "the close button closes the Fleet window", (o) => !o.window_open);
      // The sector window's fleet icon offers the system's fleets' Move
      // (FUN_00507290 kind 0x10, sector-icon-menus.md).
      const menu = await openMenu(page, point(setup.icon), directory, "fleet");
      await choose(page, menu, menu.move_row, "Move");
      await click(page, point(setup.target_planet));
      const departed = await until(page, "the fleet departs", (o) => o.in_transit);
      assert.ok(departed.aboard, "the regiment left the fleet before it departed");
      const speed = await runFast(page, setup);
      const landed = await until(page, "the regiment lands at the target",
        (o, target) => !o.aboard && !o.held && !o.in_transit && o.troop_system_dat_id === target,
        setup.target_dat_id, 90_000);
      assert.equal(landed.cargo, 0);
      return { loaded, menu, departed, speed, landed };
    },
  },
  {
    name: "unload",
    code: FLEET_LOAD,
    // A drag out of the Troops tab released on the Defenses window (type
    // 10, +0x70: its subject, FUN_004aa470) issues 0x201; in its own system the regiment
    // changes container at once (FUN_00556390, regiment-unload.md).
    async run(page, faction, setup, directory) {
      const { loaded } = await loadRegiment(page, setup, directory);
      const troops = await openTroopsTab(page, loaded);
      await dragItem(page, point(troops.first_item), point(setup.defenses_window));
      const unloaded = await until(page, "the regiment lands on its planet",
        (o, primary) => !o.aboard && !o.held && !o.regiment_travelling && o.troop_system_dat_id === primary,
        setup.primary_dat_id);
      assert.equal(unloaded.cargo, 0);
      assert.equal(unloaded.in_transit, false);
      await shot(page, directory, "unloaded");
      return { loaded, troops, unloaded };
    },
  },
  {
    name: "drag-wheel",
    code: FLEET_LOAD,
    // While a regiment drags out of the Troops tab the list holds the mouse
    // (CoolDragList, FUN_006083c0): a wheel over the bare map mid-drag
    // changes nothing (the map never zooms, FUN_00422ce0), and a release back
    // inside the list drops nothing.
    async run(page, faction, setup, directory) {
      const { loaded } = await loadRegiment(page, setup, directory);
      const troops = await openTroopsTab(page, loaded);
      const gap = { x: faction.sectors[0] + 235 + 2, y: troops.origin[1] + 150 };
      const from = point(troops.first_item);
      await page.mouse.move(from.x, from.y);
      await frames(page);
      await page.mouse.down();
      await frames(page);
      await page.mouse.move(gap.x, gap.y, { steps: 4 });
      await frames(page);
      for (let notch = 0; notch < 3; notch += 1) {
        await page.mouse.wheel(0, -100);
        await frames(page);
      }
      await page.mouse.move(from.x, from.y, { steps: 4 });
      await frames(page);
      await page.mouse.up();
      await frames(page);
      const held = await settle(page);
      assert.ok(held.aboard && held.held, JSON.stringify(held));
      return { loaded, gap, held };
    },
  },
  {
    name: "unload-refused",
    code: REGIMENT_UNLOAD_REFUSED,
    // FUN_0053d430: a regiment group's destination of another side, other
    // than an existing unpopulated system, is refused 1/0x28. The regiment
    // starts aboard and held, and the Defenses window shows the other side's
    // populated target (the Fleet window covers its planet).
    async run(page, faction, setup, directory) {
      const opened = await openFleetWindow(page, setup);
      const selected = await selectFleet(page, opened);
      assert.ok(selected.aboard && selected.held, JSON.stringify(selected));
      assert.deepEqual(selected.enabled, [true, false, true, false], "the Troops tab lights with a regiment aboard");
      const troops = await openTroopsTab(page, selected);
      await dragItem(page, point(troops.first_item), point(setup.defenses_window));
      const refusal = "Regiment move rejected: the destination belongs to another side";
      const refused = await until(page, "the other side's planet refuses the regiment",
        (o, text) => o.last_message === text, refusal);
      assert.ok(refused.aboard && refused.held, JSON.stringify(refused));
      assert.equal(refused.regiment_travelling, false);
      return { opened, selected, troops, refused };
    },
  },
  {
    name: "travel",
    code: FLEET_LOAD,
    // A regiment's speed is GNPRTB 1 (FUN_004f63f0), so its Move to another
    // planet of its side travels on its own (FUN_00556430) and arrives.
    async run(page, faction, setup, directory) {
      const menu = await openMenu(page, point(setup.troop_item), directory, "regiment");
      await choose(page, menu, menu.move_row, "Move");
      await click(page, point(setup.target_planet));
      const departed = await until(page, "the regiment departs on its own",
        (o) => o.regiment_travelling && o.troop_system_dat_id === null);
      assert.equal(departed.aboard, false);
      const speed = await runFast(page, setup);
      const arrived = await until(page, "the regiment arrives at the target",
        (o, target) => !o.regiment_travelling && o.troop_system_dat_id === target,
        setup.target_dat_id, 90_000);
      // Notification 0xd, Unit Arrival (main.rs).
      assert.equal(arrived.last_regiment_message, `Regiment arrived at ${setup.target_name}`);
      return { menu, departed, speed, arrived };
    },
  },
  {
    name: "full",
    code: FLEET_LOAD_FULL,
    // FUN_00500b40: no room is left, so the move is refused and nothing
    // boards.
    async run(page, faction, setup, directory) {
      const opened = await openFleetWindow(page, setup);
      const before = await settle(page);
      assert.equal(before.cargo, setup.capacity);
      const menu = await moveRegimentOntoFleet(page, setup, opened, directory);
      const refusal = `Regiment move rejected: troop capacity exceeded: capacity ${setup.capacity}, requested ${setup.capacity + 1}`;
      const refused = await until(page, "the full fleet refuses the regiment",
        (o, text) => o.last_message === text, refusal);
      assert.equal(refused.aboard, false);
      assert.equal(refused.cargo, setup.capacity);
      assert.equal(refused.troop_system_dat_id, setup.primary_dat_id);
      return { opened, menu, refused };
    },
  },
  {
    name: "join-ship",
    code: FLEET_JOIN,
    // A ship dragged onto another fleet's entry moves into it (0x201 with a
    // fleet destination, FUN_004feca0; manual p. 120).
    async run(page, faction, setup, directory) {
      const selected = await openJoining(page, setup);
      await dragItem(page, point(selected.first_item), point(selected.fleets[1].entry));
      const joined = await until(page, "the ship joins the second fleet",
        (o) => JSON.stringify(o.fleets.map((f) => f.ships)) === "[1,2]");
      assert.equal(joined.entries, 2);
      assert.equal(joined.in_transit, false);
      await shot(page, directory, "joined");
      return { selected, joined };
    },
  },
  {
    name: "join-refused",
    code: FLEET_JOIN,
    // A ship dropped on its own fleet goes nowhere (FUN_00553aa0 makes the
    // move into the mover's own fleet 0x26).
    async run(page, faction, setup, directory) {
      const selected = await openJoining(page, setup);
      await dragItem(page, point(selected.first_item), point(selected.fleets[0].entry));
      const refusal = "Fleet move rejected: a fleet cannot move into itself";
      const refused = await until(page, "the ship's own fleet refuses it",
        (o, text) => o.last_message === text, refusal);
      assert.deepEqual(ships(refused), [2, 1]);
      return { selected, refused };
    },
  },
  {
    name: "create-fleet",
    code: FLEET_JOIN,
    // A ship's Create Fleet (0x270, TEXTSTRA 12319) makes a fleet of it in
    // its system (FUN_005809c0, FUN_00509b40; manual p. 120).
    async run(page, faction, setup, directory) {
      const selected = await openJoining(page, setup);
      const menu = await openMenu(page, point(selected.first_item), directory, "ship");
      // FUN_00502bd0: Move, Confirmed Move, then Create Fleet (sort 50).
      assert.deepEqual([menu.move_row, menu.confirmed_move_row, menu.create_fleet_row], [0, 1, 2],
        JSON.stringify(menu));
      await choose(page, menu, menu.create_fleet_row, "Create Fleet");
      const created = await until(page, "Create Fleet makes a third fleet",
        (o) => JSON.stringify(o.fleets.map((f) => f.ships)) === "[1,1,1]");
      assert.equal(created.entries, 3);
      await shot(page, directory, "created");
      return { selected, menu, created };
    },
  },
  {
    name: "ship-to-system",
    code: FLEET_JOIN,
    // A ship dragged onto its own system's Defenses window: a system holds no
    // capital ships (FUN_00507750), so the ship forms a fleet of its own
    // there (FUN_005097d0).
    async run(page, faction, setup, directory) {
      const selected = await openJoining(page, setup);
      await dragItem(page, point(selected.first_item), point(setup.defenses_window));
      const created = await until(page, "the ship forms a fleet of its own",
        (o) => JSON.stringify(o.fleets.map((f) => f.ships)) === "[1,1,1]");
      assert.equal(created.entries, 3);
      return { selected, created };
    },
  },
  {
    name: "ship-move-join",
    code: FLEET_JOIN,
    // A ship's pop-up Move (0x201, FUN_00502bd0) released on another
    // fleet's entry: the ship joins it (FUN_004feca0).
    async run(page, faction, setup, directory) {
      const selected = await openJoining(page, setup);
      const menu = await openMenu(page, point(selected.first_item), directory, "ship");
      await choose(page, menu, menu.move_row, "Move");
      await click(page, point(selected.fleets[1].entry));
      const joined = await until(page, "the ship's Move joins the second fleet",
        (o) => JSON.stringify(o.fleets.map((f) => f.ships)) === "[1,2]");
      return { selected, menu, joined };
    },
  },
  {
    name: "ship-move-system",
    code: FLEET_JOIN,
    // A ship's pop-up Move released on its own system's Defenses window forms a
    // fleet of its own there (FUN_00507750, FUN_005097d0).
    async run(page, faction, setup, directory) {
      const selected = await openJoining(page, setup);
      const menu = await openMenu(page, point(selected.first_item), directory, "ship");
      await choose(page, menu, menu.move_row, "Move");
      await click(page, point(setup.defenses_window));
      const created = await until(page, "the ship's Move forms a fleet of its own",
        (o) => JSON.stringify(o.fleets.map((f) => f.ships)) === "[1,1,1]");
      return { selected, menu, created };
    },
  },
  {
    name: "join-fleet",
    code: FLEET_JOIN,
    // A fleet's Move released on another fleet's entry: its ships join that
    // fleet (FUN_004ffc90, FUN_004feca0), and the emptied fleet disbands
    // (FUN_004fe630).
    async run(page, faction, setup, directory) {
      const opened = await openFleetWindow(page, setup, 2);
      const menu = await openMenu(page, point(opened.fleets[1].entry), directory, "fleet");
      await choose(page, menu, menu.move_row, "Move");
      await click(page, point(opened.fleets[0].entry));
      const joined = await until(page, "the second fleet joins the first",
        (o) => JSON.stringify(o.fleets.map((f) => f.ships)) === "[3]");
      assert.equal(joined.entries, 1);
      assert.equal(joined.confirmation_open, false);
      await shot(page, directory, "joined");
      return { opened, menu, joined };
    },
  },
  {
    name: "confirmed-join-refused",
    code: FLEET_JOIN,
    // The order is checked before it asks (FUN_00487740): a Confirmed Move
    // onto the fleet itself is refused and opens no window (FUN_00553aa0).
    async run(page, faction, setup, directory) {
      const opened = await openFleetWindow(page, setup, 2);
      const menu = await openMenu(page, point(opened.fleets[0].entry), directory, "fleet");
      await choose(page, menu, menu.confirmed_move_row, "Confirmed Move");
      await click(page, point(opened.fleets[0].entry));
      const refusal = "Fleet move rejected: a fleet cannot move into itself";
      const refused = await until(page, "the fleet refuses itself",
        (o, text) => o.last_message === text, refusal);
      const settled = await settle(page);
      assert.equal(settled.confirmation_open, false, "the confirmation opened for a refused order");
      assert.deepEqual(ships(settled), [2, 1]);
      return { opened, menu, refused, settled };
    },
  },
  {
    name: "confirmed-join",
    code: FLEET_JOIN,
    // Confirmed Move asks first (FUN_00487cc0); the checkmark joins.
    async run(page, faction, setup, directory) {
      const opened = await openFleetWindow(page, setup, 2);
      const menu = await openMenu(page, point(opened.fleets[1].entry), directory, "fleet");
      await choose(page, menu, menu.confirmed_move_row, "Confirmed Move");
      await click(page, point(opened.fleets[0].entry));
      const open = await until(page, "the confirmation opens", (o) => o.confirmation_open);
      assert.deepEqual(ships(open), [2, 1], "the fleets joined before the checkmark");
      await shot(page, directory, "confirmation");
      await click(page, confirmationCheckmark(faction));
      const joined = await until(page, "the checkmark joins the fleets",
        (o) => !o.confirmation_open && JSON.stringify(o.fleets.map((f) => f.ships)) === "[3]");
      return { opened, menu, open, joined };
    },
  },
  {
    name: "rename",
    code: FLEET_LOAD,
    // Rename (0x203, FUN_00486fb0 -> FUN_0041d600 -> FUN_00429350) puts an
    // edit over the fleet's name with the name selected. An empty name
    // keeps it open (FUN_004ac950), so the name typed after it is the one
    // Enter submits.
    async run(page, faction, setup, directory, source, log) {
      const opened = await openFleetWindow(page, setup);
      const menu = await openMenu(page, point(opened.fleet_entry), directory, "fleet");
      await choose(page, menu, menu.rename_row, "Rename");
      await frames(page, 2);
      await press(page, "Backspace");
      await press(page, "Enter");
      await page.waitForTimeout(300);
      assert.equal(log.countOf(/command=0x203 destination=rename/), 0, "an empty name submits nothing");
      await shot(page, directory, "rename-empty");
      await page.keyboard.type("Rogue");
      await frames(page, 2);
      await press(page, "Enter");
      const applied = await log.logged("Enter submits the typed name",
        /command=0x203 destination=rename status=applied name=Rogue$/);
      await shot(page, directory, "renamed");
      return { opened, menu, applied };
    },
  },
  {
    name: "destination",
    code: PRODUCTION_DESTINATION,
    // The facility icon's Destination (0x214, TEXTSTRA 12290) released on a
    // planet sends the system's production areas' output there
    // (FUN_00512700 kind 4).
    async run(page, faction, setup, directory, source, log) {
      assert.ok(setup.system_icon, `the facility icon is shown: ${JSON.stringify(setup)}`);
      const menu = await openMenu(page, point(setup.system_icon), directory, "facility");
      await choose(page, menu, menu.destination_row, "Destination");
      await click(page, point(setup.target_planet));
      const set = await log.logged("the release sets the destination", new RegExp(
        `command=0x214 destination=production_destination status=set areas=\\d+ to=${escapeRegExp(setup.target_name)}$`));
      await shot(page, directory, "destination-set");
      return { menu, set };
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

    const fixtureCode = testCase.code | (faction.byte << 8);
    await page.goto(`${serverOrigin}/?fixture-code=${fixtureCode}`, { waitUntil: "load", timeout: 30_000 });
    await page.waitForFunction(() => window.__openRebellionInterfaceReady?.status, null, { timeout: 30_000 });
    const ready = await page.evaluate(() => window.__openRebellionInterfaceReady);
    assert.equal(ready.status, "ready", JSON.stringify(ready));
    assert.equal(ready.code, fixtureCode);
    await page.waitForFunction(() => window.__openRebellionInterfaceFleetLoadSetup
      && (window.__openRebellionInterfaceFleetLoads || []).length > 0, null, { timeout: 10_000 });
    const setup = await page.evaluate(() => window.__openRebellionInterfaceFleetLoadSetup);
    assert.equal(setup.code, fixtureCode);
    assert.ok(setup.capacity > 0, "the fleet carries regiments");
    const start = await latest(page);
    assert.equal(start.window_open, false, JSON.stringify(start));
    // The refused variant's regiment starts aboard (interface_test_fixture.rs).
    const startsAboard = testCase.code === REGIMENT_UNLOAD_REFUSED;
    assert.equal(start.aboard, startsAboard, JSON.stringify(start));
    assert.equal(start.troop_system_dat_id, startsAboard ? null : setup.primary_dat_id, JSON.stringify(start));
    await page.evaluate(() => document.fonts.ready);
    const { bytes: _ready, ...before } = await shot(page, directory, "ready");

    // Wait for the `count`th console line matching `pattern`.
    const log = {
      countOf: (pattern) => consoleLines.filter((line) => pattern.test(line.text)).length,
      async logged(description, pattern, count = 1, timeout = 10_000) {
        const started = Date.now();
        while (Date.now() - started < timeout) {
          const lines = consoleLines.filter((line) => pattern.test(line.text));
          if (lines.length >= count) return lines[count - 1].text;
          await page.waitForTimeout(100);
        }
        throw new Error(`${description}: ${pattern} x${count} not logged; last lines ${JSON.stringify(consoleLines.slice(-6).map((line) => line.text))}`);
      },
    };
    const checks = await testCase.run(page, faction, setup, directory, source, log);
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
      setup,
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
    family: "fleet-window",
    scope: "test-only Fleet window (type 4) through its original entry, the sector window's fleet icon: chrome against STRATEGY.DLL, a regiment's Move onto a fleet, the hold, the fleet's move and the landing, a full fleet's refusal, a regiment dragged out of the Troops tab (a wheel mid-drag does not zoom the map) onto its own system's window and onto another side's populated system's window (refused), a regiment travelling on its own, and joining and splitting fleets (a ship dragged or moved onto another fleet or its own system, a ship refused by its own fleet, Create Fleet, a fleet's Move and Confirmed Move onto another fleet, and a Confirmed Move onto itself refused before it asks), Rename (0x203: an emptied name keeps the edit open, Enter submits the typed one), and the facility icon's Destination (0x214) released on a planet, on both sides",
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
