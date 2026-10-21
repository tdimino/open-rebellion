#!/usr/bin/env node

// The endings in the browser (docs/plans/2026-10-16-feat-orbital-strike-and-
// bombardment.md, Phase L item 6), on the production build, both sides and
// both game types:
//
// - `win`: the main menu's game-type toggle (art 10158/10159,
//   main_menu.rs) sets the type a new game logs; Game Options loads a save
//   staged one order before the win in that type; the order wins (Empire:
//   the Death Star fleet's Destroy System on Hoth, 0x223; Alliance: the
//   fleet's Planetary Assault on a stripped Coruscant, 0x234). The win's
//   message movies play (Empire 0x6f MDATA.101 and 0x73 MDATA.102; Alliance
//   Coruscant Captured 0x74 MDATA.103), the advisor stamps its ending slot
//   (84 Empire, 86 Alliance; ALSPRITE 10093/10094, EMSPRITE 11093/11094),
//   Game Options saves the won campaign to row 2, and the ending movie
//   (FUN_00425c90 code 0x12 or 0x10, FUN_00414830 states 0x10-0x13: 0x6d
//   MDATA.108 or 0x6b MDATA.105) plays to its end full screen in the
//   <video> overlay (movies::video_bounds) and returns to the main menu.
// - `replay`: in a fresh page, that row-2 save loads from the main menu and
//   plays the same ending to the same main menu.
//
//   node endings.mjs --saves=<dir> [--no-build] [--only=<faction>/<type>]
//
// <dir> holds the four native saves the cases load, staged by the
// developer commands one order before each win: hoth-ds-standard.reb,
// hoth-ds-hqonly.reb (seed 42, Empire, the Death Star fleet at Hoth, Luke
// and Mon Mothma jailed for Standard) and coruscant-standard.reb,
// coruscant-hqonly.reb (seed 3, Alliance, regiments aboard the fleet at a
// stripped Coruscant, Vader and Palpatine jailed for Standard). The ending
// movies play unskipped, so a run takes about twenty minutes.

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
const runDir = path.join(root, ".artifacts/interface-parity", `endings-${runId}`);

// Main-menu controls in the 640 by 480 menu (main_menu.rs CONTROL_RECTS):
// Load Options 0x68 at (411, 232, 40, 37), the game type at (305, 333, 42,
// 30), Empire at (153, 308, 62, 55), Alliance at (437, 307, 62, 55).
const menu = {
  loadOptions: { x: 431, y: 250 },
  gameType: { x: 305, y: 333, width: 42, height: 30 },
  loadOptionsRect: { x: 411, y: 232, width: 40, height: 37 },
};
// Game Options (FUN_00406b80; game-options-smoke.mjs): six rows 42 apart,
// each a Save button at x 55, a name field and a Load button at x 305; the
// load confirmation's Yes, the saved alert's checkmark and Return (0x90).
const options = {
  row: (index) => 90 + 42 * index,
  save: 55,
  name: 198,
  load: 305,
  yes: { x: 280, y: 301 },
  savedCheckmark: { x: 318, y: 300 },
  returnIcon: { x: 183, y: 403 },
};

const gameTypes = [
  { name: "standard", label: "Standard Game", toggle: false },
  { name: "hqonly", label: "Headquarters Only", toggle: true },
];

// Galaxy points on the saves above (Small galaxy): the Empire opens the
// Churba sector, right-clicks the Death Star fleet at Hoth, holds Planetary
// Bombardment past the 500 ms submenu timer and clicks Destroy System; the
// Alliance's Churba and Sesswenna windows are open in its save, it
// right-clicks the fleet at Coruscant, clicks Planetary Assault and closes
// the Operation Report (FUN_0044c630's Close at (423, 25) in the window).
const factions = [
  {
    name: "alliance",
    label: "Alliance",
    side: "Rebel Alliance",
    save: (type) => `coruscant-${type}.reb`,
    start: { x: 468, y: 334 },
    order: /\[interface\] command=0x234 destination=assault status=issued/,
    async act(page) {
      await click(page, { x: 423, y: 201 }, "right");
      await click(page, { x: 316, y: 272 });
    },
    // The assault opens its Operation Report for the attacker; a waiting
    // movie does not close it (movies::clear_for_movie), so Coruscant
    // Captured plays once the player shuts it with its Close (FUN_0044c630,
    // at (423, 25) in the window).
    report: { close: { x: 485, y: 80, width: 32, height: 31 }, point: { x: 501, y: 95 } },
    messageMovies: [{ id: 0x74, file: 103 }],
    slot: 86,
    ending: { id: 0x6b, file: 105 },
  },
  {
    name: "empire",
    label: "Empire",
    side: "Galactic Empire",
    save: (type) => `hoth-ds-${type}.reb`,
    start: { x: 184, y: 335 },
    order: /\[interface\] command=0x223 destination=bombardment status=issued/,
    async act(page) {
      await click(page, { x: 175, y: 215 });
      await page.waitForTimeout(500);
      await click(page, { x: 241, y: 250 }, "right");
      await page.mouse.move(340, 302);
      await page.waitForTimeout(1_000);
      await click(page, { x: 140, y: 361 });
    },
    report: null,
    messageMovies: [{ id: 0x6f, file: 101 }, { id: 0x73, file: 102 }],
    slot: 84,
    ending: { id: 0x6d, file: 108 },
  },
];

const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");
const hex = (value) => `0x${value.toString(16)}`;
const escape = (text) => text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");

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

// A native save as the browser's two slot entries. The file is the header
// (SAVE_MAGIC, SAVE_VERSION, the name, a timestamp, the mods, the mod hash,
// the state fingerprint) and then the body (save.rs,
// save_file_with_interface); the browser keeps the same body base64 without
// padding under rebellion_save_v{V}_{slot} and JSON metadata under
// rebellion_meta_v{V}_{slot} (wasm_impl::save_slot_with_interface), and its
// loader refuses a fingerprint that is not the body's. game_tick is shown
// in the list only.
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

async function key(page, value) {
  await page.keyboard.press(value, { delay: 100 });
  await frames(page, 2);
}

async function shot(page, directory, label) {
  await frames(page, 2);
  const bytes = await page.screenshot({ animations: "disabled" });
  fs.writeFileSync(path.join(directory, `${label}.png`), bytes);
  return { png: PNG.sync.read(bytes), sha256: sha256(bytes) };
}

function differing(a, b, rect) {
  let count = 0;
  for (let y = rect.y; y < rect.y + rect.height; y += 1) {
    for (let x = rect.x; x < rect.x + rect.width; x += 1) {
      const at = (y * a.width + x) * 4;
      if ([0, 1, 2].some((index) => a.data[at + index] !== b.data[at + index])) count += 1;
    }
  }
  return count;
}

// Every media event of the overlay's <video> elements. They do not bubble,
// so the listener captures them on the document.
function recordMedia() {
  window.__endingsMedia = [];
  for (const type of ["playing", "ended", "error"]) {
    document.addEventListener(type, (event) => {
      const video = event.target;
      if (!(video instanceof HTMLVideoElement)) return;
      window.__endingsMedia.push({
        type,
        src: video.getAttribute("src"),
        time: video.currentTime,
        duration: video.duration,
        error: video.error ? `${video.error.code} ${video.error.message}` : null,
      });
    }, true);
  }
}

class Session {
  constructor(page, consoleLines) {
    this.page = page;
    this.lines = consoleLines;
  }

  count(pattern) {
    return this.lines.filter(({ text }) => pattern.test(text)).length;
  }

  find(pattern) {
    return this.lines.find(({ text }) => pattern.test(text))?.text;
  }

  // Wait for the console to log `pattern` more than `after` times. With
  // `skip`, a movie on screen is ended by a left press on its <video>, as a
  // player's click ends one (FUN_005f7240; web/index.html), once it has
  // played half a second.
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
    return this.lines.filter(({ text }) => pattern.test(text)).at(-1).text;
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

  // The ending: the code's movie opens full screen, plays to its own end
  // unskipped, and the main menu follows (FUN_00414830 states 0x10-0x13).
  async ending(faction, directory, opening) {
    const checks = {};
    checks.campaign_ended = await this.line(new RegExp(`\\[main_menu\\] campaign_ended generation=\\d+ movie=${hex(faction.ending.id)}$`));
    const file = String(faction.ending.file).padStart(3, "0");
    await this.line(new RegExp(`\\[cutscene\\] opened path=assets/references/ref-videos/${file}\\.webm`));
    await this.page.waitForFunction(() => {
      const video = document.querySelector("video.movie");
      return video && video.currentTime > 1;
    }, null, { timeout: 20_000 });
    checks.video = await this.page.evaluate(() => {
      const video = document.querySelector("video.movie");
      const box = video.getBoundingClientRect();
      return { src: video.getAttribute("src"), rect: [box.x, box.y, box.width, box.height], size: [video.videoWidth, video.videoHeight], duration: video.duration };
    });
    checks.movie_screen_sha256 = (await shot(this.page, directory, "ending-movie")).sha256;
    assert.equal(checks.video.src, `data/movies/${file}.webm`);
    // An ending movie is no message movie: the whole screen.
    checks.video.rect.forEach((value, index) => assert.ok(Math.abs(value - [0, 0, 640, 480][index]) < 0.5, `the ending is not full screen: ${checks.video.rect}`));
    checks.movie_ended = await this.line(new RegExp(`\\[movie\\] ended id=${hex(faction.ending.id)} next=MainMenu`), { timeout: 300_000 });
    const media = (await this.page.evaluate(() => window.__endingsMedia)).filter(({ src }) => src === `data/movies/${file}.webm`);
    checks.media = media;
    assert.deepEqual(media.filter(({ type }) => type === "error"), [], "the ending movie raised an error");
    const ended = media.find(({ type }) => type === "ended");
    assert.ok(ended, "the ending movie did not play to its end");
    assert.ok(Math.abs(ended.time - ended.duration) < 0.5, `the ending stopped at ${ended.time} of ${ended.duration}`);
    await this.page.waitForFunction(() => !document.querySelector("video.movie"), null, { timeout: 5_000 });
    await this.page.mouse.move(2, 2);
    await this.page.waitForTimeout(500);
    const menuShot = await shot(this.page, directory, "main-menu-returned");
    checks.menu_load_options_differing = differing(menuShot.png, opening.png, menu.loadOptionsRect);
    assert.equal(checks.menu_load_options_differing, 0, "the main menu's Load Options control is not where the menu left it");
    return checks;
  }
}

async function win(session, faction, type, saves, directory) {
  const { page } = session;
  const checks = {};
  await session.toMenu();
  const opening = await shot(page, directory, "main-menu");
  await page.evaluate((entries) => { for (const [k, v] of Object.entries(entries)) localStorage.setItem(k, v); },
    browserSlot(path.join(saves, faction.save(type.name)), 0, faction.name === "alliance"));
  if (type.toggle) {
    await click(page, { x: menu.gameType.x + 21, y: menu.gameType.y + 15 });
    await page.mouse.move(2, 2);
    const toggled = await shot(page, directory, "main-menu-toggled");
    checks.toggle_differing = differing(opening.png, toggled.png, menu.gameType);
    assert.ok(checks.toggle_differing > 0, "the game-type control did not change its art");
  }
  // A new game in the menu's type.
  await click(page, faction.start);
  checks.started = await session.line(new RegExp(`\\[campaign\\] faction=${faction.side} configuration=`));
  assert.ok(checks.started.endsWith(type.label), `the new game is not ${type.label}: ${checks.started}`);
  await session.line(/\[movie\] ended id=0x6[89] next=Galaxy/, { skip: true });
  await page.waitForTimeout(500);

  // Load the staged campaign of the same type over it (row 1, Yes).
  await session.openOptions(() => key(page, "F1"));
  await click(page, { x: options.load, y: options.row(0) });
  await click(page, options.yes);
  checks.restored = await session.line(/\[campaign\] restored slot=0 mode=Galaxy/);
  assert.match(checks.restored, new RegExp(`faction=${faction.label}$`));
  checks.loaded = session.find(/\[campaign\] loaded configuration=/);
  assert.ok(checks.loaded?.endsWith(type.label), `the loaded campaign is not ${type.label}: ${checks.loaded}`);
  await page.waitForTimeout(1_000);
  await shot(page, directory, "loaded");
  // Alt+NumPad+ steps the speed up (FUN_00422ce0) so the win's day comes.
  for (let step = 0; step < 4; step += 1) await key(page, "Alt+NumpadAdd");

  const ordering = await shot(page, directory, "before-order");
  await faction.act(page);
  checks.order = await session.line(faction.order, { timeout: 10_000 });
  await page.mouse.move(2, 2);
  // Each message movie plays and is ended by a click once it has played;
  // an Operation Report is closed once it shows.
  checks.message_movies = [];
  const pending = [...faction.messageMovies];
  let report = faction.report;
  const deadline = Date.now() + 180_000;
  while (pending.length > 0 || report) {
    assert.ok(Date.now() < deadline, `the win did not finish: movies ${JSON.stringify(pending)}, report ${report ? "open" : "closed"}`);
    const next = pending[0];
    const ended = next && session.find(new RegExp(`\\[movie\\] ended id=${hex(next.id)} next=Galaxy`));
    if (ended) {
      const file = String(next.file).padStart(3, "0");
      assert.ok(session.find(new RegExp(`\\[cutscene\\] opened path=assets/references/ref-videos/${file}\\.webm`)), `movie ${hex(next.id)} ended without opening ${file}.webm`);
      checks.message_movies.push(ended);
      pending.shift();
      continue;
    }
    const playing = await page.evaluate(() => {
      const video = document.querySelector("video.movie");
      if (video && video.currentTime > 0.5) video.dispatchEvent(new PointerEvent("pointerdown", { button: 0, bubbles: true }));
      return Boolean(video);
    });
    if (!playing && report) {
      const now = PNG.sync.read(await page.screenshot({ animations: "disabled" }));
      const opened = differing(ordering.png, now, report.close);
      if (opened > 200) {
        fs.writeFileSync(path.join(directory, "report-open.png"), PNG.sync.write(now));
        await click(page, report.point);
        await page.mouse.move(2, 2);
        const shut = await shot(page, directory, "report-closed");
        const closed = differing(now, shut.png, report.close);
        assert.ok(closed > 200, `the Operation Report did not close (${closed} pixels changed at its Close)`);
        checks.report_close_changed = [opened, closed];
        report = null;
      }
    }
    await page.waitForTimeout(250);
  }
  checks.slot = await session.line(/\[advisor\] ending slot \d+ stamped/);
  assert.match(checks.slot, new RegExp(`ending slot ${faction.slot} stamped`));
  await page.waitForTimeout(500);
  assert.equal(session.count(/\[main_menu\] campaign_ended/), 0, "the ending began before the won campaign could be saved");

  // Save the won campaign to row 2 (no confirmation on an empty row; the
  // 0x474 alert's checkmark; Return).
  await session.openOptions(() => key(page, "F1"));
  await click(page, { x: options.name, y: options.row(1) });
  await key(page, "Control+a");
  await page.keyboard.type(`${faction.name} ${type.name} won`);
  await click(page, { x: options.save, y: options.row(1) });
  checks.saved = await session.line(/save_state_fingerprint slot=1 /, { timeout: 10_000 });
  await click(page, options.savedCheckmark);
  await click(page, options.returnIcon);
  const stored = await page.evaluate(() => Object.fromEntries(Object.entries(localStorage).filter(([k]) => /^rebellion_(save|meta)_v\d+_1$/.test(k))));
  assert.equal(Object.keys(stored).length, 2, "row 2 holds no save");

  Object.assign(checks, await session.ending(faction, directory, opening));
  return { checks, stored };
}

async function replay(session, faction, type, stored, directory) {
  const { page } = session;
  const checks = {};
  await session.toMenu();
  const opening = await shot(page, directory, "main-menu");
  await page.evaluate((entries) => { for (const [k, v] of Object.entries(entries)) localStorage.setItem(k, v); }, stored);
  await session.openOptions(() => click(page, menu.loadOptions));
  await click(page, { x: options.load, y: options.row(1) });
  checks.restored = await session.line(/\[campaign\] restored slot=1 mode=Galaxy/);
  assert.match(checks.restored, new RegExp(`faction=${faction.label}$`));
  checks.loaded = session.find(/\[campaign\] loaded configuration=/);
  assert.ok(checks.loaded?.endsWith(type.label), `the loaded campaign is not ${type.label}: ${checks.loaded}`);
  checks.slot = await session.line(/\[advisor\] ending slot \d+ stamped/);
  assert.match(checks.slot, new RegExp(`ending slot ${faction.slot} stamped`));
  Object.assign(checks, await session.ending(faction, directory, opening));
  return { checks };
}

async function inspect(server, executable, faction, type, name, run) {
  const directory = path.join(runDir, faction.name, type.name, name);
  fs.mkdirSync(directory, { recursive: true });
  const requests = [];
  const errors = [];
  const consoleLines = [];
  const launchAttempts = [];
  let browser;
  let context;
  let page;
  let result;
  let outcome = {};
  try {
    browser = await launchBrowser(chromium, { executablePath: executable, headless: true, args: browserManifest.launch_arguments, timeout: 30_000 }, launchAttempts);
    context = await browser.newContext({ viewport: { width: 640, height: 480 }, deviceScaleFactor: 1, locale: "en-US", timezoneId: "America/New_York", colorScheme: "dark", serviceWorkers: "block" });
    await context.addInitScript(recordMedia);
    page = await context.newPage();
    const serverOrigin = `http://127.0.0.1:${server.address().port}`;
    page.on("response", (response) => {
      if (new URL(response.url()).origin === serverOrigin) requests.push({ url: new URL(response.url()).pathname, status: response.status() });
    });
    page.on("requestfailed", (request) => {
      // The shell drops a finished movie's src, which aborts its range request.
      if (!(request.url().endsWith(".webm") && request.failure()?.errorText === "net::ERR_ABORTED")) errors.push(`requestfailed:${request.url()}:${request.failure()?.errorText}`);
    });
    page.on("pageerror", (error) => errors.push(`pageerror:${error.stack || error.message}`));
    page.on("console", (message) => {
      consoleLines.push({ type: message.type(), text: message.text() });
      if (message.type() === "error") errors.push(`console:error:${message.text()}`);
    });
    await page.goto(`${serverOrigin}/`, { waitUntil: "load", timeout: 30_000 });
    outcome = await run(new Session(page, consoleLines), directory);
    assert.ok(requests.every(({ status }) => status === 200 || status === 206), `non-200 requests: ${JSON.stringify(requests.filter(({ status }) => status !== 200 && status !== 206))}`);
    assert.deepEqual(errors, [], `browser diagnostics: ${errors.join("; ")}`);
    result = { status: "pass", faction: faction.name, game_type: type.name, case: name, checks: outcome.checks, launch_attempts: launchAttempts };
  } catch (error) {
    if (page) await page.screenshot({ path: path.join(directory, "failure.png") }).catch(() => {});
    result = { status: "fail", faction: faction.name, game_type: type.name, case: name, error: String(error.stack || error), errors, launch_attempts: launchAttempts };
  } finally {
    if (page) await page.close().catch(() => {});
    if (context) await context.close().catch(() => {});
    if (browser) await browser.close().catch(() => {});
  }
  const kept = consoleLines.filter(({ text }) => /\[(campaign|cutscene|movie|main_menu|interface|advisor\] ending)|state_fingerprint/.test(text));
  fs.writeFileSync(path.join(directory, "console.json"), `${JSON.stringify(kept, null, 2)}\n`);
  fs.writeFileSync(path.join(directory, "result.json"), `${JSON.stringify({ ...result, requests }, null, 2)}\n`);
  return { result, stored: outcome.stored };
}

async function main() {
  assert.ok(savesArg, "pass --saves=<dir> holding the four staged saves");
  const saves = path.resolve(savesArg.slice(8));
  if (!noBuild) execFileSync("bash", [path.join(root, "scripts/build-wasm.sh")], { cwd: root, env: { ...process.env }, stdio: "inherit" });
  for (const required of ["index.html", "gl.js", "open-rebellion.wasm", "data/runtime.orpk"]) {
    assert.ok(fs.existsSync(path.join(site, required)), `the production site is missing ${required}`);
  }
  const movies = [...new Set(factions.flatMap(({ messageMovies, ending }) => [...messageMovies, ending].map(({ file }) => `${String(file).padStart(3, "0")}.webm`)))];
  for (const movie of movies) assert.ok(fs.existsSync(path.join(site, "data/movies", movie)), `web/data/movies is missing ${movie}`);
  fs.mkdirSync(runDir, { recursive: true });
  const executable = browserExecutable();
  const server = await startServer();
  const results = [];
  try {
    for (const faction of factions) {
      for (const type of gameTypes) {
        if (only.length && !only.includes(`${faction.name}/${type.name}`)) continue;
        const won = await inspect(server, executable, faction, type, "win", (session, directory) => win(session, faction, type, saves, directory));
        results.push(won.result);
        const replayed = won.stored
          ? await inspect(server, executable, faction, type, "replay", (session, directory) => replay(session, faction, type, won.stored, directory))
          : { result: { status: "fail", faction: faction.name, game_type: type.name, case: "replay", error: "the win case saved no won campaign" } };
        results.push(replayed.result);
        for (const { result } of [won, replayed]) {
          process.stdout.write(`${result.status === "pass" ? "PASS" : "FAIL"} ${faction.name}/${type.name}/${result.case}${result.error ? `: ${result.error.split("\n")[0]}` : ""}\n`);
        }
      }
    }
  } finally {
    await new Promise((resolve) => server.close(resolve));
  }
  const passed = results.filter(({ status }) => status === "pass").length;
  const summary = {
    schema_version: 1,
    family: "endings",
    scope: "production build: the main menu's game-type toggle setting a new game's type; a staged campaign of that type won by one order (Empire Destroy System on Hoth, Alliance Planetary Assault on Coruscant); the win's message movies; the advisor's ending slot; the won campaign saved from Game Options; the ending movie played to its end full screen in the <video> overlay and the main menu after it; the save replaying the ending from the main menu; both sides, Standard and Headquarters Only",
    status: results.length > 0 && passed === results.length ? "pass" : "fail",
    cases: results.length,
    passed,
    failed: results.length - passed,
    browser_version: browserManifest.version,
    launch_arguments: browserManifest.launch_arguments,
    viewport: { width: 640, height: 480, device_scale_factor: 1 },
    wasm_sha256: sha256(fs.readFileSync(path.join(site, "open-rebellion.wasm"))),
    index_html_sha256: sha256(fs.readFileSync(path.join(site, "index.html"))),
    movie_sha256: Object.fromEntries(movies.map((movie) => [movie, sha256(fs.readFileSync(path.join(site, "data/movies", movie)))])),
    save_sha256: Object.fromEntries(fs.readdirSync(saves).filter((file) => file.endsWith(".reb")).map((file) => [file, sha256(fs.readFileSync(path.join(saves, file)))])),
    results,
  };
  fs.writeFileSync(path.join(runDir, "summary.json"), `${JSON.stringify(summary, null, 2)}\n`);
  process.stdout.write(`${JSON.stringify({ run_dir: runDir, status: summary.status, cases: summary.cases, passed, failed: summary.failed }, null, 2)}\n`);
  if (summary.status !== "pass") process.exitCode = 1;
}

await main();
