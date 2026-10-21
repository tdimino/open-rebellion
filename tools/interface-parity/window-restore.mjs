#!/usr/bin/env node

// A load reopens the saved strategic windows, in the browser, on the
// production build, both sides (docs/plans/2026-10-16-feat-orbital-strike-
// and-bombardment.md, Phase Q item 1; or-port-window-restore-on-load-iclp).
//
// On a staged campaign the case sets the GID mode to Uprisings (Alt+2,
// GidMode::from_accelerator_digit), opens a sector window, opens one
// System window and minimizes it to the rail, opens a second System window,
// drags it and shows its Shipyards page, then opens a Fleet window and
// drags it over the System window. Game Options saves the campaign to row
// 2 (wasm_impl::save_slot_with_interface, the interface record of
// FUN_00440ea0). A fresh page loads that row from the main menu
// (wasm_impl::load_slot_with_interface, FUN_00440f80 → FUN_00441190) and
// the case asserts:
//
// - the Fleet window and the System window at their dragged places, the
//   Fleet window in front, pixel for pixel as they were saved;
// - the System window on its first page, the Overview: the record keeps
//   no System page (SavedWindowKind::System, FUN_00458ea0);
// - the minimized System window on the rail's first slot
//   (FUN_00440ea0.c:19-24, FUN_00428b40);
// - the focus on the Fleet window saved on top (GalaxyFocusState::
//   after_load): Escape closes it, a second Escape closes the System
//   window, and a third leaves the rail as it is;
// - the GID mode: with the sector window closed the galaxy shows the
//   Uprisings map the campaign was saved with (view +0xc8, replayed as
//   WM_COMMAND 0x140).
//
//   node window-restore.mjs --saves=<dir> [--no-build] [--only=<faction>]
//
// <dir> holds coruscant-standard.reb and hoth-ds-standard.reb, the
// campaigns the endings gate stages (endings.mjs).

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
const site = path.join(root, "web");
const browserManifest = JSON.parse(fs.readFileSync(path.join(here, "browser.json"), "utf8"));
const noBuild = process.argv.includes("--no-build");
const only = process.argv.filter((arg) => arg.startsWith("--only=")).map((arg) => arg.slice(7));
const savesArg = process.argv.find((arg) => arg.startsWith("--saves="));
const runId = `${new Date().toISOString().replace(/[:.]/g, "-")}-${process.pid}`;
const runDir = path.join(root, ".artifacts/interface-parity", `window-restore-${runId}`);

// Game Options (FUN_00406b80; game-options-smoke.mjs): six rows 42 apart,
// each a Save button at x 55, a name field and a Load button at x 305; the
// load confirmation's Yes, the saved alert's checkmark and Return (0x90).
// The main menu's Load Options control 0x68 at (411, 232, 40, 37).
const options = {
  row: (index) => 90 + 42 * index,
  save: 55,
  name: 198,
  load: 305,
  savedCheckmark: { x: 318, y: 300 },
  returnIcon: { x: 183, y: 403 },
  menuLoadOptions: { x: 431, y: 250 },
};

// The System window is 226 by 304 (system_window.rs): its title buttons at
// (195, 3) minimize and (209, 3) close, 14 by 14 (STRATEGY 10253, 10108),
// the title strip 20 high, and the six tabs from (0, 20), 36 by 33, the
// Shipyards tab at x 39. The Fleet window is 235 by 304 (fleet_window.rs).
const systemWindow = { width: 226, height: 304, minimize: { x: 202, y: 10 }, title: 20, shipyards: { x: 57, y: 36 } };
const fleetWindow = { width: 235, height: 304 };

// Per side: the galaxy aperture (CockpitState::layout_for), the rail's
// first slot (system_window.rs rail_slot_rect), the sector windows the
// staged save opens and their Close buttons, the galaxy point that opens
// the sector the case works in and that window's Close, the planet whose
// System window goes to the rail, the planet whose System window is
// dragged, the fleet icon, and where the drags put the windows: inside the
// galaxy view, which a restored window is clamped into
// (clamp_window_to_galaxy).
const factions = [
  {
    name: "alliance",
    label: "Alliance",
    save: "coruscant-standard.reb",
    galaxy: { x: 55, y: 40, width: 485, height: 350 },
    rail: { x: 544, y: 61, width: 62, height: 18 },
    savedSectorCloses: [{ x: 525, y: 43 }, { x: 285, y: 43 }],
    sector: { x: 225, y: 135 },
    sectorClose: { x: 285, y: 43 },
    minimized: { x: 105, y: 180 },
    system: { x: 163, y: 218 },
    fleet: { x: 183, y: 201 },
    systemTo: { x: 300, y: 50 },
    fleetTo: { x: 150, y: 80 },
  },
  {
    name: "empire",
    label: "Empire",
    save: "hoth-ds-standard.reb",
    galaxy: { x: 120, y: 40, width: 480, height: 355 },
    rail: { x: 21, y: 48, width: 54, height: 18 },
    savedSectorCloses: [],
    sector: { x: 175, y: 215 },
    sectorClose: { x: 345, y: 48 },
    minimized: { x: 152, y: 82 },
    system: { x: 278, y: 229 },
    fleet: { x: 241, y: 250 },
    systemTo: { x: 365, y: 50 },
    fleetTo: { x: 215, y: 80 },
  },
];

const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");

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

// A native save as the browser's two slot entries (endings.mjs,
// browserSlot): the header (save.rs, save_file_with_interface) dropped, the
// body base64 without padding, and JSON metadata with the header's
// fingerprint (wasm_impl::save_slot_with_interface).
function browserSlot(file, slot, alliance) {
  const bytes = fs.readFileSync(file);
  assert.equal(bytes.toString("latin1", 0, 8), "OPENREB\0", `${file} is not an Open Rebellion save`);
  let at = 8;
  const version = bytes.readUInt32LE(at);
  at += 4;
  const nameLength = bytes.readUInt32LE(at);
  at += 4;
  const name = bytes.toString("utf8", at, at + nameLength);
  at += nameLength + 8;
  assert.equal(bytes.readUInt32LE(at), 0, `${file} carries mods; the browser loads none`);
  at += 4 + 8;
  const fingerprintVersion = bytes.readUInt16LE(at);
  const fingerprint = bytes.readBigUInt64LE(at + 2);
  at += 10;
  return {
    [`rebellion_save_v${version}_${slot}`]: bytes.subarray(at).toString("base64").replace(/=+$/, ""),
    [`rebellion_meta_v${version}_${slot}`]: JSON.stringify({
      schema_version: 1,
      name,
      game_tick: 0,
      state_fingerprint: { version: fingerprintVersion, value: fingerprint.toString() },
      player_is_alliance: alliance,
    }),
  };
}

function mimeType(file) {
  if (file.endsWith(".html")) return "text/html; charset=utf-8";
  if (file.endsWith(".js")) return "text/javascript; charset=utf-8";
  if (file.endsWith(".wasm")) return "application/wasm";
  if (file.endsWith(".webm")) return "video/webm";
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
  await frames(page, 2);
}

// Two presses one animation frame apart, each seen by egui, well inside
// its double-click time. The pair starts half a second after any earlier
// press, past egui's 0.3 s double-click delay, so the earlier press cannot
// make the first of the pair a double click and the second a triple.
async function doubleClick(page, point) {
  const frame = () => page.evaluate(() => new Promise((resolve) => requestAnimationFrame(resolve)));
  await page.waitForTimeout(500);
  await page.mouse.move(point.x, point.y);
  await frames(page);
  const start = Date.now();
  for (let press = 1; press <= 2; press += 1) {
    await page.mouse.down({ clickCount: press });
    await frame();
    await page.mouse.up({ clickCount: press });
    if (press === 1) await frame();
  }
  const elapsed = Date.now() - start;
  await frames(page, 2);
  return elapsed;
}

// A press on the window's own surface drags it (system_window.rs
// TitleDrag: FUN_00606ee0, FUN_006071e0, FUN_00607010).
async function drag(page, from, to) {
  await page.mouse.move(from.x, from.y);
  await frames(page);
  await page.mouse.down();
  await frames(page);
  await page.mouse.move(to.x, to.y, { steps: 12 });
  await frames(page, 2);
  await page.mouse.up();
  await frames(page, 2);
}

async function key(page, value) {
  await page.keyboard.press(value, { delay: 100 });
  await frames(page, 2);
}

async function shot(page, directory, label) {
  await page.mouse.move(2, 2);
  await page.waitForTimeout(200);
  await frames(page, 2);
  const bytes = await page.screenshot({ animations: "disabled" });
  fs.writeFileSync(path.join(directory, `${label}.png`), bytes);
  return PNG.sync.read(bytes);
}

const inside = (rect, x, y) => x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height;

// The pixels of `rect` that differ between two captures, leaving out the
// pixels inside any of `except`.
function differing(a, b, rect, except = []) {
  let count = 0;
  for (let y = rect.y; y < rect.y + rect.height; y += 1) {
    for (let x = rect.x; x < rect.x + rect.width; x += 1) {
      if (except.some((box) => inside(box, x, y))) continue;
      const at = (y * a.width + x) * 4;
      if ([0, 1, 2].some((index) => a.data[at + index] !== b.data[at + index])) count += 1;
    }
  }
  return count;
}

// The box of the pixels that differ between two captures within `within`.
function changedBox(before, after, within) {
  let box = null;
  for (let y = within.y; y < within.y + within.height; y += 1) {
    for (let x = within.x; x < within.x + within.width; x += 1) {
      const at = (y * before.width + x) * 4;
      if ([0, 1, 2].every((index) => before.data[at + index] === after.data[at + index])) continue;
      box = box
        ? { x0: Math.min(box.x0, x), y0: Math.min(box.y0, y), x1: Math.max(box.x1, x), y1: Math.max(box.y1, y) }
        : { x0: x, y0: y, x1: x, y1: y };
    }
  }
  return box && { x: box.x0, y: box.y0, width: box.x1 - box.x0 + 1, height: box.y1 - box.y0 + 1 };
}

const screen = { x: 0, y: 0, width: 640, height: 480 };
const clip = (rect) => {
  const x = Math.max(rect.x, 0);
  const y = Math.max(rect.y, 0);
  return { x, y, width: Math.min(rect.x + rect.width, 640) - x, height: Math.min(rect.y + rect.height, 480) - y };
};

// A window that opened between two captures at the double-click point,
// clamped into the galaxy view (clamp_window_to_galaxy, FUN_0045aac0): at
// least half of its rectangle changed.
const galaxyClamp = (point, size, galaxy) => ({
  x: Math.min(Math.max(point.x, galaxy.x), galaxy.x + galaxy.width - size.width),
  y: Math.min(Math.max(point.y, galaxy.y), galaxy.y + galaxy.height - size.height),
});

function opened(before, after, point, size, galaxy, description) {
  const rect = { ...galaxyClamp(point, size, galaxy), width: size.width, height: size.height };
  const changed = differing(before, after, rect);
  assert.ok(changed >= (size.width * size.height) / 2, `${description} did not open at ${rect.x}, ${rect.y} (${changed} pixels changed)`);
  return rect;
}

// Double-click `point` and read the window that opens there; a double
// click the page missed (no pixel of the window changed) is pressed once
// more, and the attempts are kept.
async function openWindow(page, directory, label, before, point, size, galaxy, description, attempts) {
  const presses = [];
  for (let attempt = 1; ; attempt += 1) {
    presses.push(await doubleClick(page, point));
    const after = await shot(page, directory, label);
    const rect = { ...galaxyClamp(point, size, galaxy), width: size.width, height: size.height };
    if (attempt === 2 || differing(before, after, rect) > 0) {
      attempts.push({ window: description, attempts: attempt, press_ms: presses });
      return { after, rect: opened(before, after, point, size, galaxy, description) };
    }
  }
}

class Session {
  constructor(page, consoleLines) {
    this.page = page;
    this.lines = consoleLines;
  }

  count(pattern) {
    return this.lines.filter((text) => pattern.test(text)).length;
  }

  async line(pattern, { after = 0, skip = false, timeout = 120_000 } = {}) {
    const deadline = Date.now() + timeout;
    while (this.count(pattern) <= after) {
      if (Date.now() > deadline) throw new Error(`no console line ${pattern} within ${timeout} ms`);
      if (skip) {
        await this.page.evaluate(() => {
          const video = document.querySelector("video.movie");
          if (video && video.currentTime > 0.5) video.dispatchEvent(new PointerEvent("pointerdown", { button: 0, bubbles: true }));
        });
      }
      await this.page.waitForTimeout(250);
    }
    return this.lines.filter((text) => pattern.test(text)).at(-1);
  }

  async toMenu() {
    await this.line(/\[movie\] ended id=0x66 next=MainMenu/, { skip: true });
    await this.page.mouse.move(2, 2);
    await this.page.waitForTimeout(500);
  }

  async openOptions(press) {
    const opened = /destination=game_options status=opened_original/;
    const before = this.count(opened);
    await press();
    await this.line(opened, { after: before, timeout: 10_000 });
    await this.page.waitForTimeout(300);
  }

  // Load row `row` of Game Options from the main menu: no game is live, so
  // the row loads without a confirmation.
  async loadFromMenu(entries, row, faction) {
    await this.toMenu();
    await this.page.evaluate((stored) => { for (const [k, v] of Object.entries(stored)) localStorage.setItem(k, v); }, entries);
    await this.openOptions(() => click(this.page, options.menuLoadOptions));
    await click(this.page, { x: options.load, y: options.row(row) });
    const restored = await this.line(new RegExp(`\\[campaign\\] restored slot=${row} mode=Galaxy`));
    assert.match(restored, new RegExp(`faction=${faction.label}$`));
    await this.page.waitForTimeout(1_000);
    return restored;
  }
}

// The first page: set up the windows and save them.
async function setUp(session, faction, saves, directory, checks) {
  const { page } = session;
  checks.staged = await session.loadFromMenu(browserSlot(path.join(saves, faction.save), 0, faction.name === "alliance"), 0, faction);
  for (const point of faction.savedSectorCloses) await click(page, point);
  const plain = await shot(page, directory, "1-galaxy");
  await key(page, "Alt+Digit2");
  checks.gid = await session.line(/destination=gid status=selected label=Uprisings source=accelerator/, { timeout: 10_000 });
  const uprisings = await shot(page, directory, "2-galaxy-uprisings");
  checks.gid_galaxy_changed = differing(plain, uprisings, faction.galaxy);
  assert.ok(checks.gid_galaxy_changed > 0, "Alt+2 left the galaxy as it was");

  await click(page, faction.sector);
  const sector = await shot(page, directory, "3-sector");
  assert.ok(changedBox(uprisings, sector, faction.galaxy), "no sector window opened");

  // The System window that goes to the rail.
  checks.double_clicks = [];
  const { rect: minimizedRect } = await openWindow(page, directory, "4-system-to-minimize", sector, faction.minimized, systemWindow, faction.galaxy, "the System window to minimize", checks.double_clicks);
  await click(page, { x: minimizedRect.x + systemWindow.minimize.x, y: minimizedRect.y + systemWindow.minimize.y });
  const railed = await shot(page, directory, "5-minimized");
  checks.rail_changed = differing(sector, railed, faction.rail);
  assert.ok(checks.rail_changed > 0, "the minimized System window is not on the rail");

  // The System window that is dragged and shows its Shipyards page.
  const { rect: opening } = await openWindow(page, directory, "6-system", railed, faction.system, systemWindow, faction.galaxy, "the System window", checks.double_clicks);
  await drag(page, { x: opening.x + 100, y: opening.y + 10 }, { x: faction.systemTo.x + 100, y: faction.systemTo.y + 10 });
  const system = { ...faction.systemTo, width: systemWindow.width, height: systemWindow.height };
  const overview = await shot(page, directory, "7-system-dragged");
  await click(page, { x: system.x + systemWindow.shipyards.x, y: system.y + systemWindow.shipyards.y });
  const shipyards = await shot(page, directory, "8-system-shipyards");
  const page_ = { x: system.x, y: system.y + systemWindow.title, width: system.width, height: system.height - systemWindow.title };
  checks.page_changed = differing(overview, shipyards, page_);
  assert.ok(checks.page_changed > 0, "the Shipyards tab left the page as it was");

  // The Fleet window, dragged over the System window.
  const { after: fourth, rect: fleetOpening } = await openWindow(page, directory, "9-fleet", shipyards, faction.fleet, fleetWindow, faction.galaxy, "the Fleet window", checks.double_clicks);
  await drag(page, { x: fleetOpening.x + 100, y: fleetOpening.y + 10 }, { x: faction.fleetTo.x + 100, y: faction.fleetTo.y + 10 });
  const fleet = { ...faction.fleetTo, width: fleetWindow.width, height: fleetWindow.height };
  const saved = await shot(page, directory, "10-saved");
  assert.ok(differing(fourth, saved, fleet) > 0, "the Fleet window did not move");

  // Save to row 2.
  await session.openOptions(() => key(page, "F1"));
  await click(page, { x: options.name, y: options.row(1) });
  await key(page, "Control+a");
  await page.keyboard.type(`${faction.name} windows`);
  await click(page, { x: options.save, y: options.row(1) });
  checks.saved = await session.line(/save_state_fingerprint slot=1 /, { timeout: 10_000 });
  await click(page, options.savedCheckmark);
  await click(page, options.returnIcon);
  const returned = await shot(page, directory, "11-returned");
  checks.return_differing = differing(saved, returned, screen);
  const stored = await page.evaluate(() => Object.fromEntries(Object.entries(localStorage).filter(([k]) => /^rebellion_(save|meta)_v\d+_1$/.test(k))));
  assert.equal(Object.keys(stored).length, 2, "row 2 holds no save");
  fs.writeFileSync(path.join(directory, "row-2.json"), `${JSON.stringify(stored)}\n`);
  return { stored, captures: { uprisings, railed, overview, shipyards, saved }, rects: { system, fleet } };
}

// The second page: load the save and read the windows back.
async function restore(session, faction, setup, directory, checks) {
  const { page } = session;
  const { captures, rects } = setup;
  checks.restored = await session.loadFromMenu(setup.stored, 1, faction);
  const loaded = await shot(page, directory, "1-loaded");
  const fleetClip = clip(rects.fleet);
  const systemClip = clip(rects.system);
  const titleStrip = { ...systemClip, height: systemWindow.title };
  const page_ = { x: systemClip.x, y: systemClip.y + systemWindow.title, width: systemClip.width, height: systemClip.height - systemWindow.title };

  // The Fleet window in front, where it was dragged.
  checks.fleet_differing = differing(captures.saved, loaded, fleetClip);
  checks.fleet_differing_box = changedBox(captures.saved, loaded, fleetClip);
  assert.equal(checks.fleet_differing, 0, "the Fleet window is not back in front where it was saved");
  // The System window where it was dragged, on its Overview page.
  checks.system_title_differing = differing(captures.saved, loaded, titleStrip, [fleetClip]);
  assert.equal(checks.system_title_differing, 0, "the System window is not back where it was saved");
  checks.system_overview_differing = differing(captures.overview, loaded, page_, [fleetClip]);
  checks.system_shipyards_differing = differing(captures.shipyards, loaded, page_, [fleetClip]);
  assert.equal(checks.system_overview_differing, 0, "the System window did not reopen on its Overview page");
  assert.ok(checks.system_shipyards_differing > 0, "the System window kept its Shipyards page");
  // The minimized System window on the rail.
  checks.rail_differing = differing(captures.railed, loaded, faction.rail);
  assert.equal(checks.rail_differing, 0, "the rail is not as it was saved");

  // The focus: Escape closes the Fleet window, then the System window,
  // then nothing on the rail.
  await key(page, "Escape");
  const first = await shot(page, directory, "2-escape-fleet");
  checks.escape_fleet = {
    fleet: differing(loaded, first, fleetClip),
    system_page: differing(loaded, first, page_, [fleetClip]),
    system_title: differing(loaded, first, titleStrip, [fleetClip]),
  };
  assert.ok(checks.escape_fleet.fleet > 0, "the first Escape did not close the Fleet window");
  assert.equal(checks.escape_fleet.system_page, 0, "the first Escape moved or closed the System window");
  // The System window under it takes the focus (0x467): its title strip
  // draws active.
  assert.ok(checks.escape_fleet.system_title > 0, "the System window did not take the focus");
  await key(page, "Escape");
  const second = await shot(page, directory, "3-escape-system");
  checks.escape_system = differing(first, second, systemClip);
  assert.ok(checks.escape_system > 0, "the second Escape did not close the System window");
  await key(page, "Escape");
  const third = await shot(page, directory, "4-escape-rail");
  checks.escape_rail = differing(second, third, faction.rail);
  assert.equal(checks.escape_rail, 0, "the third Escape changed the rail");

  // The GID mode: the galaxy with the sector window closed shows the
  // Uprisings map it was saved with.
  await click(page, faction.sectorClose);
  const galaxy = await shot(page, directory, "5-galaxy");
  checks.gid_galaxy_differing = differing(captures.uprisings, galaxy, faction.galaxy);
  assert.equal(checks.gid_galaxy_differing, 0, "the galaxy does not show the saved GID mode");
}

async function inspect(server, executable, faction, saves) {
  const directory = path.join(runDir, faction.name);
  const pages = { setup: path.join(directory, "save"), restore: path.join(directory, "load") };
  for (const folder of Object.values(pages)) fs.mkdirSync(folder, { recursive: true });
  const requests = [];
  const errors = [];
  const consoleLines = [];
  const launchAttempts = [];
  const checks = {};
  let browser;
  let result;
  try {
    browser = await launchBrowser(chromium, { executablePath: executable, headless: true, args: browserManifest.launch_arguments, timeout: 30_000 }, launchAttempts);
    const serverOrigin = `http://127.0.0.1:${server.address().port}`;
    // Each page is a fresh context: the second sees only the row the first
    // saved.
    const open = async () => {
      const context = await browser.newContext({ viewport: { width: 640, height: 480 }, deviceScaleFactor: 1, locale: "en-US", timezoneId: "America/New_York", colorScheme: "dark", serviceWorkers: "block" });
      const page = await context.newPage();
      const lines = [];
      page.on("response", (response) => {
        if (new URL(response.url()).origin === serverOrigin) requests.push({ url: new URL(response.url()).pathname, status: response.status() });
      });
      page.on("requestfailed", (request) => {
        if (!(request.url().endsWith(".webm") && request.failure()?.errorText === "net::ERR_ABORTED")) errors.push(`requestfailed:${request.url()}:${request.failure()?.errorText}`);
      });
      page.on("pageerror", (error) => errors.push(`pageerror:${error.stack || error.message}`));
      page.on("console", (message) => {
        lines.push(message.text());
        consoleLines.push(message.text());
        if (message.type() === "error") errors.push(`console:error:${message.text()}`);
      });
      await page.goto(`${serverOrigin}/`, { waitUntil: "load", timeout: 30_000 });
      return { context, page, lines };
    };
    const first = await open();
    let setup;
    try {
      checks.save = {};
      setup = await setUp(new Session(first.page, first.lines), faction, saves, pages.setup, checks.save);
    } catch (error) {
      await first.page.screenshot({ path: path.join(pages.setup, "failure.png") }).catch(() => {});
      throw error;
    } finally {
      await first.context.close().catch(() => {});
    }
    const second = await open();
    try {
      checks.load = {};
      await restore(new Session(second.page, second.lines), faction, setup, pages.restore, checks.load);
    } catch (error) {
      await second.page.screenshot({ path: path.join(pages.restore, "failure.png") }).catch(() => {});
      throw error;
    } finally {
      await second.context.close().catch(() => {});
    }
    assert.ok(requests.every(({ status }) => status === 200 || status === 206), `non-200 requests: ${JSON.stringify(requests.filter(({ status }) => status !== 200 && status !== 206))}`);
    assert.deepEqual(errors, [], `browser diagnostics: ${errors.join("; ")}`);
    result = { status: "pass", faction: faction.name, checks, launch_attempts: launchAttempts };
  } catch (error) {
    result = { status: "fail", faction: faction.name, error: String(error.stack || error), checks, errors, launch_attempts: launchAttempts };
  } finally {
    if (browser) await browser.close().catch(() => {});
  }
  const kept = consoleLines.filter((text) => /\[(campaign|interface)\]|state_fingerprint/.test(text));
  fs.writeFileSync(path.join(directory, "console.json"), `${JSON.stringify(kept, null, 2)}\n`);
  fs.writeFileSync(path.join(directory, "result.json"), `${JSON.stringify({ ...result, requests }, null, 2)}\n`);
  return result;
}

async function main() {
  assert.ok(savesArg, "pass --saves=<dir> holding coruscant-standard.reb and hoth-ds-standard.reb");
  const saves = path.resolve(savesArg.slice(8));
  if (!noBuild) execFileSync("bash", [path.join(root, "scripts/build-wasm.sh")], { cwd: root, env: { ...process.env }, stdio: "inherit" });
  for (const required of ["index.html", "gl.js", "open-rebellion.wasm", "data/runtime.orpk"]) {
    assert.ok(fs.existsSync(path.join(site, required)), `the production site is missing ${required}`);
  }
  fs.mkdirSync(runDir, { recursive: true });
  const executable = browserExecutable();
  const server = await startServer();
  const results = [];
  try {
    for (const faction of factions) {
      if (only.length && !only.includes(faction.name)) continue;
      const result = await inspect(server, executable, faction, saves);
      results.push(result);
      process.stdout.write(`${result.status === "pass" ? "PASS" : "FAIL"} ${faction.name}${result.error ? `: ${result.error.split("\n")[0]}` : ""}\n`);
    }
  } finally {
    await new Promise((resolve) => server.close(resolve));
  }
  const passed = results.filter(({ status }) => status === "pass").length;
  const summary = {
    schema_version: 1,
    family: "window-restore",
    scope: "production build: Game Options saves a campaign with a dragged System window on its Shipyards page, a minimized System window, a Fleet window dragged over the first, and the Uprisings GID mode; a fresh page loads it and finds the windows where they were saved, the Fleet window in front, the System window on its Overview page, the rail as saved, the focus on the Fleet window (Escape closes it, then the System window) and the Uprisings map; both sides",
    status: results.length > 0 && passed === results.length ? "pass" : "fail",
    cases: results.length,
    passed,
    failed: results.length - passed,
    browser_version: browserManifest.version,
    launch_arguments: browserManifest.launch_arguments,
    viewport: { width: 640, height: 480, device_scale_factor: 1 },
    wasm_sha256: sha256(fs.readFileSync(path.join(site, "open-rebellion.wasm"))),
    index_html_sha256: sha256(fs.readFileSync(path.join(site, "index.html"))),
    save_sha256: Object.fromEntries(factions.map(({ save }) => [save, sha256(fs.readFileSync(path.join(saves, save)))])),
    results,
  };
  fs.writeFileSync(path.join(runDir, "summary.json"), `${JSON.stringify(summary, null, 2)}\n`);
  process.stdout.write(`${JSON.stringify({ run_dir: runDir, status: summary.status, cases: summary.cases, passed, failed: summary.failed }, null, 2)}\n`);
  if (summary.status !== "pass") process.exitCode = 1;
}

await main();
