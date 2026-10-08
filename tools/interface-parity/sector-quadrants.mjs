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
// `--only=alliance/icons` runs the named cases alone.
const only = process.argv.filter((arg) => arg.startsWith("--only=")).map((arg) => arg.slice(7));
const expectedRequests = ["/", "/data/runtime.orpk", "/gl.js", "/open-rebellion-test.wasm"];
// Fixture codes are the Scenario index plus one (interface_test_fixture.rs).
const QUADRANTS = 50;
const factions = [
  { name: "alliance", byte: 1, side: 1, other: 2 },
  { name: "empire", byte: 2, side: 2, other: 1 },
];
// FUN_0045ca80(kind, side, 0): the first bitmap of each kind for sides
// 0 (and 3), 1 and 2; the fleet and mission kinds have none for side 0.
const art = {
  system: [10787, 10771, 10779],
  defenses: [10789, 10773, 10781],
  fleets: [null, 10775, 10783],
  missions: [null, 10777, 10785],
};
// What the fixture stocks (interface_test_fixture.rs,
// place_quadrant_contents): the side each checked icon shows, or null where
// nothing at the system lights it.
function expectedSides(faction) {
  return {
    primary: {
      system: faction.side,
      defenses: faction.side,
      fleets: faction.side,
      // FUN_004a1f60: the other side's mission wins.
      missions: faction.other,
    },
    second: {
      system: null,
      // FUN_0045ce80: the system's side, held by the other side.
      defenses: faction.other,
      fleets: null,
      missions: faction.side,
    },
  };
}
const runId = `${new Date().toISOString().replace(/[:.]/g, "-")}-${process.pid}`;
const runDir = path.join(root, ".artifacts/interface-parity", `sector-quadrants-${runId}`);

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

function gokresDirectory() {
  const candidates = [
    process.env.REBELLION_GOKRES_BMP_DIR,
    path.join(root, "data/base/ui/gokres-dll/BMP"),
  ].filter(Boolean);
  const directory = candidates.find((candidate) => fs.existsSync(path.join(candidate, "17472.bmp")));
  if (!directory) {
    throw new Error("owned GOKRES.DLL BMP extraction is unavailable; set REBELLION_GOKRES_BMP_DIR");
  }
  return directory;
}

function sourceDirectory() {
  const candidates = [
    process.env.REBELLION_STRATEGY_BMP_DIR,
    path.join(root, "data/base/ui/strategy-dll/BMP"),
  ].filter(Boolean);
  const directory = candidates.find((candidate) => fs.existsSync(path.join(candidate, "10771.bmp")));
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
const usedGokres = new Set();

function resource(source, id) {
  used.add(id);
  return decodeIndexedBmp(fs.readFileSync(path.join(source, `${id}.bmp`)));
}

function gokres(id) {
  usedGokres.add(id);
  return decodeIndexedBmp(fs.readFileSync(path.join(gokresDirectory(), `${id}.bmp`)));
}

// A window-sized bitmap's region (x, y, w, h) against the screenshot at the
// window's screen corner, every pixel opaque.
function compareRegion(screenshot, corner, image, region, label, directory) {
  const [x0, y0, width, height] = region;
  const crop = new PNG({ width, height });
  for (let y = 0; y < height; y += 1) {
    const from = ((y0 + y) * image.width + x0) * 4;
    crop.data.set(image.data.subarray(from, from + width * 4), y * width * 4);
  }
  return compareIcon(screenshot, [corner[0] + x0, corner[1] + y0], crop, label, directory, false);
}

// The palette-blue matte every quadrant icon carries (bmp_cache.rs,
// uses_blue_screen_transparency, 10771..=10790).
function blueKey(data, offset) {
  return data[offset] < 32 && data[offset + 1] < 32 && data[offset + 2] > 192;
}

// The icon's opaque pixels against the screenshot at the overlay's top-left
// (port: paint_native draws the bitmap at its own size there). The matte
// shows whatever lies behind, so it stays out of the check.
function compareIcon(screenshot, rect, icon, label, directory, keyed = true) {
  const [left, top] = rect;
  const diff = new PNG({ width: icon.width, height: icon.height });
  const actual = new PNG({ width: icon.width, height: icon.height });
  let checked = 0;
  let different = 0;
  for (let y = 0; y < icon.height; y += 1) {
    for (let x = 0; x < icon.width; x += 1) {
      const from = (y * icon.width + x) * 4;
      const at = ((Math.round(top) + y) * screenshot.width + Math.round(left) + x) * 4;
      actual.data.set(screenshot.data.subarray(at, at + 4), from);
      if (keyed && blueKey(icon.data, from)) {
        diff.data.set([0, 0, 96, 255], from);
        continue;
      }
      checked += 1;
      const matches = screenshot.data[at] === icon.data[from]
        && screenshot.data[at + 1] === icon.data[from + 1]
        && screenshot.data[at + 2] === icon.data[from + 2];
      if (!matches) different += 1;
      diff.data.set(matches ? [0, 0, 0, 255] : [255, 0, 80, 255], from);
    }
  }
  fs.writeFileSync(path.join(directory, `${label}-actual.png`), PNG.sync.write(actual));
  fs.writeFileSync(path.join(directory, `${label}-expected.png`), PNG.sync.write(icon));
  fs.writeFileSync(path.join(directory, `${label}-diff.png`), PNG.sync.write(diff));
  return { label, pixels_checked: checked, different_pixels: different };
}

async function click(page, point) {
  await page.mouse.move(point.x, point.y);
  await frames(page);
  await page.mouse.down();
  await frames(page);
  await page.mouse.up();
  await frames(page);
}

async function frames(page, count = 1) {
  for (let index = 0; index < count; index += 1) {
    await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  }
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
  return page.evaluate(() => window.__openRebellionInterfaceQuadrants || []);
}

async function latest(page) {
  const list = await observations(page);
  assert.ok(list.length > 0, "the fixture reported no quadrant observation");
  return list[list.length - 1];
}

// Wait until the latest observation meets `predicate(observation, argument)`.
async function until(page, description, predicate, argument = null, timeout = 10_000) {
  try {
    await page.waitForFunction(
      ([source, value]) => {
        const last = (window.__openRebellionInterfaceQuadrants || []).at(-1);
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

async function shot(page, directory, label) {
  const bytes = await page.screenshot({ animations: "disabled" });
  fs.writeFileSync(path.join(directory, `${label}.png`), bytes);
  return { label, sha256: sha256(bytes), bytes };
}

// The first draw requests each newly shown bitmap from the WASM texture
// cache; let the upload reach a later paint, then require two equal frames.
async function stableScreen(page, directory, label) {
  await page.mouse.move(2, 2);
  await page.waitForTimeout(150);
  await frames(page);
  const first = await page.screenshot({ animations: "disabled" });
  await frames(page);
  const second = await page.screenshot({ animations: "disabled" });
  fs.writeFileSync(path.join(directory, `${label}-screen.png`), second);
  assert.equal(sha256(first), sha256(second), `the screen did not settle (${label})`);
  const screenshot = PNG.sync.read(second);
  assert.deepEqual([screenshot.width, screenshot.height], [640, 480]);
  return screenshot;
}

function quadrant(setup, system, name) {
  const dat = system === "primary" ? setup.primary_dat_id : setup.second_dat_id;
  const found = setup.quadrants.find((entry) => entry.system_dat_id === dat && entry.quadrant === name);
  assert.ok(found, `the setup reports ${system}/${name}`);
  return found.rect;
}


// The System Defenses window (type 10, defenses_window.rs) the bottom-left
// icon opens: FUN_004a8790's tab art (normal, selected = +1, empty = +2;
// one gray id for side 0/3 on the side-art tabs), and the pages the
// fixture stocks.
const TAB_BASE = {
  batteries: [10550, 10550, 10550],
  shields: [10553, 10553, 10553],
  squadrons: [10562, 10556, 10559],
  regiments: [10569, 10563, 10566],
  personnel: [10576, 10570, 10573],
};
function tabArt(name, side, selected, empty) {
  const base = TAB_BASE[name][side === 1 || side === 2 ? side : 0];
  if ((side !== 1 && side !== 2) && !["batteries", "shields"].includes(name)) return base;
  return base + (selected ? 1 : empty ? 2 : 0);
}
const PAGES = ["personnel", "regiments", "squadrons", "shields", "batteries"];
// troop_mini: the Alliance Fleet Regiment and Stormtrooper Regiment minis.
const REGIMENT_MINI = { 1: 17472, 2: 17536 };

function defensesWindow(observation, dat) {
  return observation.defenses_windows.find((window) => window.system_dat_id === dat);
}

// The window's screen corner from its personnel tab at (28, 20).
function corner(window) {
  const [left, top] = window.tabs.find(([name]) => name === "personnel")[1];
  return [left - 28, top - 20];
}

function compareTabs(screenshot, source, window, directory, prefix) {
  const checks = [];
  for (const [name, rect] of window.tabs) {
    const index = PAGES.indexOf(name);
    const id = tabArt(name, window.side, window.page === name, window.counts[index] === 0);
    checks.push({ id, ...compareIcon(screenshot, rect, resource(source, id), `${prefix}-tab-${name}`, directory) });
  }
  return checks;
}

function assertExact(checks) {
  for (const check of checks) {
    assert.ok(check.pixels_checked > 0, `${check.label} checked nothing`);
    assert.equal(check.different_pixels, 0,
      `${check.label} (${check.id}): ${check.different_pixels} of ${check.pixels_checked} pixels differ`);
  }
}

async function openDefenses(page, setup, system) {
  const [left, top] = quadrant(setup, system, "defenses");
  const at = { x: Math.round(left) + 4, y: Math.round(top) + 9 };
  await doubleClick(page, at);
  const dat = system === "primary" ? setup.primary_dat_id : setup.second_dat_id;
  const opened = await until(page, `the ${system} defenses icon opens the Defenses window`,
    (o, value) => o.defenses_windows.some((window) => window.system_dat_id === value), dat);
  return { at, dat, opened, window: defensesWindow(opened, dat) };
}

// The Missions window (type 11, missions_window.rs) the bottom-right icon
// opens: a mission's GOKRES mini is 0x4000 + (TEXTSTRA id & 0xfff), 0x5000 +
// for side 2 (FUN_004a1590; Diplomacy 0x2c10); the row frame by the player's
// side; the tabs by the selected mission's side, pressed while selected
// (FUN_004a0ca0); the rail icon by FUN_004a1f60's side (FUN_004a21c0).
const DIPLOMACY_MINI = { 1: 0x4c10, 2: 0x5c10 };
const MISSION_FRAME = { 1: 11127, 2: 11128 };
const MISSION_TABS = { 1: { agents: 11560, decoys: 11562 }, 2: { agents: 11565, decoys: 11567 } };
const MISSIONS_RAIL = { 1: 11539, 2: 11540 };

function missionsWindow(observation, dat) {
  return observation.missions_windows.find((window) => window.system_dat_id === dat);
}

// A mission row's mini below its name: the list draws the name over the
// picture from (1, 0) (FUN_006083c0's DrawTextA after FUN_00609960), so the
// first rows hold text in the original as in the port.
const ROW_TEXT_BAND = 12;
function compareRowMini(screenshot, rect, image, label, directory) {
  const height = image.height - ROW_TEXT_BAND;
  const crop = new PNG({ width: image.width, height });
  image.data.copy(crop.data, 0, ROW_TEXT_BAND * image.width * 4);
  return compareIcon(screenshot, [rect[0], rect[1] + ROW_TEXT_BAND], crop, label, directory);
}

function compareMissionTabs(screenshot, source, window, directory, prefix) {
  return window.tabs.map(([name, rect]) => {
    const id = MISSION_TABS[window.tab_side][name] + (window.tab === name ? 1 : 0);
    return { id, ...compareIcon(screenshot, rect, resource(source, id), `${prefix}-tab-${name}`, directory) };
  });
}

const cases = [
  {
    name: "icons",
    // FUN_0045d140 draws each enabled overlay with FUN_0045ca80's art for
    // its side, and nothing for a disabled one.
    async run(page, faction, setup, directory, source) {
      const screenshot = await stableScreen(page, directory, "icons");
      const compares = [];
      const absent = [];
      for (const [system, sides] of Object.entries(expectedSides(faction))) {
        for (const [name, side] of Object.entries(sides)) {
          const rect = quadrant(setup, system, name);
          assert.deepEqual(rect.slice(2), [28, 19], `${system}/${name} is 28 by 19`);
          if (side !== null) {
            const id = art[name][side];
            compares.push({ id, ...compareIcon(screenshot, rect, resource(source, id), `${system}-${name}`, directory) });
            continue;
          }
          // Hidden: no art of the kind matches there.
          for (const id of art[name].filter((value) => value !== null)) {
            const check = compareIcon(screenshot, rect, resource(source, id), `${system}-${name}-not-${id}`, directory);
            absent.push({ id, ...check });
            assert.ok(check.different_pixels > check.pixels_checked / 2,
              `${system}/${name} shows ${id}: ${check.different_pixels} of ${check.pixels_checked} differ`);
          }
        }
      }
      for (const check of compares) {
        assert.equal(check.different_pixels, 0,
          `${check.label} (${check.id}): ${check.different_pixels} of ${check.pixels_checked} pixels differ`);
      }
      return { compares, absent };
    },
  },
  {
    name: "selected",
    // FUN_004593e0 WM_PAINT draws a selected overlay's second bitmap, the
    // next id in FUN_0045ca80's pair; a press selects the overlay under it
    // (FUN_0045b1b0) and deselects the rest (FUN_0045afc0).
    async run(page, faction, setup, directory, source) {
      const shown = Object.entries(expectedSides(faction).primary)
        .filter(([, side]) => side !== null);
      assert.ok(shown.length >= 2, "the primary system shows two icons");
      const compares = [];
      for (const [pressed] of shown) {
        const [left, top] = quadrant(setup, "primary", pressed);
        await click(page, { x: Math.round(left) + 4, y: Math.round(top) + 9 });
        const screenshot = await stableScreen(page, directory, `selected-${pressed}`);
        for (const [name, side] of shown) {
          const id = art[name][side] + (name === pressed ? 1 : 0);
          compares.push({
            id,
            ...compareIcon(screenshot, quadrant(setup, "primary", name), resource(source, id),
              `selected-${pressed}-${name}`, directory),
          });
        }
      }
      assertExact(compares);
      return { compares };
    },
  },
  {
    name: "open-system",
    // FUN_004593e0 case 0x203 finds the shown overlay under the point;
    // FUN_0045aac0 maps kind 4 to the System window (type 9). The point lies
    // on the top-left icon, left of the planet's picture.
    async run(page, faction, setup) {
      const [left, top] = quadrant(setup, "primary", "system");
      const at = { x: Math.round(left) + 4, y: Math.round(top) + 9 };
      await doubleClick(page, at);
      const opened = await until(page, "the system icon opens the System window",
        (o, dat) => o.system_windows.some(([system]) => system === dat), setup.primary_dat_id);
      assert.equal(opened.system_windows.length, 1, JSON.stringify(opened));
      return { at, opened };
    },
  },
  {
    name: "defenses",
    // FUN_0045aac0 maps kind 8 to type 10 (FUN_004a7790), which opens on
    // the personnel page (FUN_0060d7e0(strip, 1)). The player's own system:
    // its regiment and KDY-150, the garrison line on the regiment page
    // (FUN_004a90d0's tail), the row frame keyed over the selected mini
    // (FUN_004a9ab0), and minimize to the rail and back (0x466,
    // FUN_004aa4a0).
    async run(page, faction, setup, directory, source) {
      const { at, dat, window } = await openDefenses(page, setup, "primary");
      assert.equal(window.side, faction.side, JSON.stringify(window));
      assert.equal(window.page, "personnel");
      assert.deepEqual(window.counts, [0, 1, 0, 0, 1]);
      assert.deepEqual([window.rows, window.selected, window.garrison], [[], null, null]);
      const origin = corner(window);
      const background = resource(source, 10577);
      let screenshot = await stableScreen(page, directory, "personnel");
      const opening = [
        ...compareTabs(screenshot, source, window, directory, "personnel"),
        // The empty list and the strip below it show the background.
        { id: 10577, ...compareRegion(screenshot, origin, background, [7, 81, 222, 210], "personnel-list", directory) },
        { id: 10577, ...compareRegion(screenshot, origin, background, [0, 292, 235, 12], "personnel-bottom", directory) },
      ];
      assertExact(opening);

      const regimentsTab = window.tabs.find(([name]) => name === "regiments")[1];
      await click(page, { x: regimentsTab[0] + 18, y: regimentsTab[1] + 16 });
      const regiments = defensesWindow(await until(page, "the regiment tab opens its page",
        (o, value) => o.defenses_windows.some((w) => w.system_dat_id === value && w.page === "regiments"), dat), dat);
      assert.equal(regiments.rows.length, 1, JSON.stringify(regiments));
      assert.match(regiments.garrison, /^Garrison Requirement: \d+$/);
      screenshot = await stableScreen(page, directory, "regiments");
      const mini = REGIMENT_MINI[faction.side];
      const regimentChecks = [
        ...compareTabs(screenshot, source, regiments, directory, "regiments"),
        { id: mini, ...compareIcon(screenshot, regiments.cells[0], gokres(mini), "regiments-mini", directory) },
      ];
      assertExact(regimentChecks);

      const cell = regiments.cells[0];
      await click(page, { x: cell[0] + 30, y: cell[1] + 12 });
      const selected = defensesWindow(await until(page, "a click selects the row",
        (o, value) => o.defenses_windows.some((w) => w.system_dat_id === value && w.selected === 0), dat), dat);
      screenshot = await stableScreen(page, directory, "selected");
      const frame = faction.side === 1 ? 10578 : 10579;
      const selectedChecks = [
        { id: frame, ...compareIcon(screenshot, selected.cells[0], resource(source, frame), "selected-frame", directory) },
      ];
      assertExact(selectedChecks);

      await click(page, { x: origin[0] + 210, y: origin[1] + 9 });
      const railed = await until(page, "minimize sends the window to the rail",
        (o, value) => !o.defenses_windows.some((w) => w.system_dat_id === value)
          && o.rail.some(([system, kind]) => system === value && kind === "defenses"), dat);
      const slot = railed.rail.find(([system, kind]) => system === dat && kind === "defenses")[2];
      screenshot = await stableScreen(page, directory, "railed");
      const icon = faction.side === 1 ? 11533 : 11534;
      const railChecks = [{ id: icon, ...compareIcon(screenshot, slot, resource(source, icon), "rail-icon", directory) }];
      assertExact(railChecks);

      await click(page, { x: slot[0] + slot[2] / 2, y: slot[1] + slot[3] / 2 });
      const restored = defensesWindow(await until(page, "the rail slot restores the window",
        (o, value) => o.defenses_windows.some((w) => w.system_dat_id === value)
          && !o.rail.some(([system, kind]) => system === value && kind === "defenses"), dat), dat);
      assert.deepEqual(restored.origin, window.origin);
      return { at, window, opening, regiments, regimentChecks, selectedChecks, railChecks, restored };
    },
  },
  {
    name: "defenses-other-side",
    // +0x148 is the system's side: the other side's system lists only its
    // objects, of which the fixture leaves none, so every tab shows that
    // side's empty art and the player's regiment there is not listed.
    async run(page, faction, setup, directory, source) {
      const { at, window } = await openDefenses(page, setup, "second");
      assert.equal(window.side, faction.other, JSON.stringify(window));
      assert.deepEqual(window.counts, [0, 0, 0, 0, 0]);
      assert.equal(window.garrison, null);
      const screenshot = await stableScreen(page, directory, "other-side");
      const checks = compareTabs(screenshot, source, window, directory, "other-side");
      assertExact(checks);
      return { at, window, checks };
    },
  },
  {
    name: "missions",
    // FUN_0045aac0 maps kind 0x40 to type 11 (FUN_0049f130): the primary
    // system's two Diplomacy missions, the player's first and selected
    // (FUN_0049f540, FUN_00609500), its agent listed; another row's click
    // reselects (FUN_004a0c60), the Decoys tab refills (id 0x16), and
    // minimize goes to the rail and back (0x466, FUN_004a21c0).
    async run(page, faction, setup, directory, source) {
      const [left, top] = quadrant(setup, "primary", "missions");
      const at = { x: Math.round(left) + 24, y: Math.round(top) + 9 };
      await doubleClick(page, at);
      const dat = setup.primary_dat_id;
      const opened = await until(page, "the missions icon opens the Missions window",
        (o, value) => o.missions_windows.some((window) => window.system_dat_id === value), dat);
      const window = missionsWindow(opened, dat);
      assert.deepEqual(window.rows, [
        ["Diplomacy", faction.side, DIPLOMACY_MINI[faction.side]],
        ["Diplomacy", faction.other, DIPLOMACY_MINI[faction.other]],
      ], JSON.stringify(window));
      assert.equal(window.side, faction.other);
      assert.deepEqual([window.selected, window.tab, window.tab_side], [0, "agents", faction.side]);
      assert.equal(window.members.length, 1, JSON.stringify(window));
      assert.ok(window.target && window.target !== "Target Unknown", JSON.stringify(window));
      const [rowLeft, rowTop] = window.row_rects[0];
      const origin = [rowLeft - 5, rowTop - 24];
      const background = resource(source, 11165);
      let screenshot = await stableScreen(page, directory, "missions");
      const other = DIPLOMACY_MINI[faction.other];
      const opening = [
        // Below the two rows the list shows the background, as does the
        // foot right of the list.
        { id: 11165, ...compareRegion(screenshot, origin, background, [5, 124, 94, 175], "missions-list", directory) },
        { id: 11165, ...compareRegion(screenshot, origin, background, [100, 292, 135, 12], "missions-foot", directory) },
        { id: other, ...compareRowMini(screenshot, window.row_rects[1], gokres(other), "missions-second-mini", directory) },
        { id: MISSION_FRAME[faction.side], ...compareIcon(screenshot, window.row_rects[0],
          resource(source, MISSION_FRAME[faction.side]), "missions-selected-frame", directory) },
        ...compareMissionTabs(screenshot, source, window, directory, "missions"),
      ];
      assertExact(opening);

      const second = window.row_rects[1];
      await click(page, { x: second[0] + 40, y: second[1] + 20 });
      const reselected = missionsWindow(await until(page, "a click selects the other mission",
        (o, value) => o.missions_windows.some((w) => w.system_dat_id === value && w.selected === 1), dat), dat);
      assert.deepEqual([reselected.tab, reselected.tab_side], ["agents", faction.other]);
      assert.equal(reselected.members.length, 1, JSON.stringify(reselected));
      screenshot = await stableScreen(page, directory, "reselected");
      const own = DIPLOMACY_MINI[faction.side];
      const reselectedChecks = [
        { id: own, ...compareRowMini(screenshot, reselected.row_rects[0], gokres(own), "reselected-first-mini", directory) },
        { id: MISSION_FRAME[faction.side], ...compareIcon(screenshot, reselected.row_rects[1],
          resource(source, MISSION_FRAME[faction.side]), "reselected-frame", directory) },
        ...compareMissionTabs(screenshot, source, reselected, directory, "reselected"),
      ];
      assertExact(reselectedChecks);

      const decoysTab = reselected.tabs.find(([name]) => name === "decoys")[1];
      await click(page, { x: decoysTab[0] + 30, y: decoysTab[1] + 8 });
      const decoys = missionsWindow(await until(page, "the Decoys tab refills the member list",
        (o, value) => o.missions_windows.some((w) => w.system_dat_id === value && w.tab === "decoys"), dat), dat);
      assert.deepEqual(decoys.members, []);
      screenshot = await stableScreen(page, directory, "decoys");
      const decoyChecks = compareMissionTabs(screenshot, source, decoys, directory, "decoys");
      assertExact(decoyChecks);

      await click(page, { x: origin[0] + 210, y: origin[1] + 9 });
      const railed = await until(page, "minimize sends the window to the rail",
        (o, value) => !o.missions_windows.some((w) => w.system_dat_id === value)
          && o.rail.some(([system, kind]) => system === value && kind === "missions"), dat);
      const slot = railed.rail.find(([system, kind]) => system === dat && kind === "missions")[2];
      screenshot = await stableScreen(page, directory, "railed");
      const icon = MISSIONS_RAIL[faction.other];
      const railChecks = [{ id: icon, ...compareIcon(screenshot, slot, resource(source, icon), "rail-icon", directory) }];
      assertExact(railChecks);

      await click(page, { x: slot[0] + slot[2] / 2, y: slot[1] + slot[3] / 2 });
      const restored = missionsWindow(await until(page, "the rail slot restores the window",
        (o, value) => o.missions_windows.some((w) => w.system_dat_id === value)
          && !o.rail.some(([system, kind]) => system === value && kind === "missions"), dat), dat);
      assert.deepEqual(restored.origin, window.origin);
      assert.equal(restored.selected, 0);
      return { at, window, opening, reselected, reselectedChecks, decoyChecks, railChecks, restored };
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

    const fixtureCode = QUADRANTS | (faction.byte << 8);
    await page.goto(`${serverOrigin}/?fixture-code=${fixtureCode}`, { waitUntil: "load", timeout: 30_000 });
    await page.waitForFunction(() => window.__openRebellionInterfaceReady?.status, null, { timeout: 30_000 });
    const ready = await page.evaluate(() => window.__openRebellionInterfaceReady);
    assert.equal(ready.status, "ready", JSON.stringify(ready));
    assert.equal(ready.code, fixtureCode);
    await page.waitForFunction(() => window.__openRebellionInterfaceQuadrantSetup
      && (window.__openRebellionInterfaceQuadrants || []).length > 0, null, { timeout: 10_000 });
    const setup = await page.evaluate(() => window.__openRebellionInterfaceQuadrantSetup);
    assert.equal(setup.code, fixtureCode);
    assert.equal(setup.scale, 1, "the gate compares at the original's scale");
    assert.equal(setup.quadrants.length, 8, JSON.stringify(setup));
    const start = await latest(page);
    assert.deepEqual(
      [start.system_windows, start.defenses_windows, start.missions_windows, start.rail],
      [[], [], [], []],
      JSON.stringify(start));
    await page.evaluate(() => document.fonts.ready);
    const { bytes: _ready, ...before } = await shot(page, directory, "ready");

    const checks = await testCase.run(page, faction, setup, directory, source);
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
  const gokresIds = [...usedGokres].sort((left, right) => left - right);
  const gokresAggregate = createHash("sha256");
  for (const id of gokresIds) {
    gokresAggregate.update(`${id}\0`);
    gokresAggregate.update(fs.readFileSync(path.join(gokresDirectory(), `${id}.bmp`)));
  }
  return [
    { dll: "STRATEGY.DLL", resource_ids: ids, aggregate_sha256: aggregate.digest("hex") },
    { dll: "GOKRES.DLL", resource_ids: gokresIds, aggregate_sha256: gokresAggregate.digest("hex") },
  ];
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
    family: "sector-quadrants",
    scope: "test-only sector window quadrant icons (FUN_00459e30): each shown icon's art against STRATEGY.DLL, a pressed icon's selected art, hidden icons absent, the system icon opening the System window, and the defenses icon opening the System Defenses window (tabs, rows, selection frame, rail), and the missions icon opening the Missions window (rows, selection frame, tabs, rail) against STRATEGY.DLL and GOKRES.DLL, on both sides",
    status: passed ? "pass" : "fail",
    browser_version: browserManifest.version,
    browser_executable: executable,
    launch_arguments: browserManifest.launch_arguments,
    profile: "one-new-muted-process-per-faction-and-case",
    muted: browserManifest.launch_arguments.includes("--mute-audio"),
    viewport: { width: 640, height: 480, device_scale_factor: 1 },
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
