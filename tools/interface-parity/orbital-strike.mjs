#!/usr/bin/env node

// The orbital strike's browser journeys (docs/plans/2026-10-16-feat-orbital-
// strike-and-bombardment.md, Phases E, L and M), both sides:
//
// - `report-window` (EVT-03): the Operation Report window (window type 0x1c,
//   FUN_0044c630; operation_report.rs) as the fixture opens it, the player's
//   own copy of a run that killed a regiment and a shipyard. Its Summary
//   shows picture 0x2b9b through the shell's blue aperture; the shell, the
//   rail art and the rail of five (Close and four views) are compared
//   exactly; the two Forces views show the six tabs, each tab its layer
//   (Personnel the three-column 0x2a0a with Survivors, Captured and Killed),
//   Display its two buttons; Close closes it.
// - `live-strike` (OBJ-14, EVT-03): the fleet's pop-up menu lists Planetary
//   Bombardment (0x120) enabled with its four rows, three enabled and
//   Destroy System grey (no Death Star, FUN_00581e80); a click on the grey
//   row does nothing; General Bombardment opens the report at once for the
//   bombarder (class 7, FUN_0048a060); the Conflict rail light is lit, opens
//   the Message Index on Conflict, whose report row reopens the report, and
//   its Close reopens the Index (FUN_00469de0, FUN_00469d60).
// - `death-star-strike` (OBJ-14, L7): with the Death Star aboard, all four
//   rows are enabled; Destroy System plays movie 0x6f (MDATA.101) in the
//   <video> overlay over the picture alone, the caption drawn below it on
//   the canvas; after it, the destroyed system greys the parent row
//   (0x224 refuses 1/0x22).
// - The EVT-03 cells, each through the fleet's menu and the live report
//   (interface_test_fixture.rs, OperationReportWatch::observe_live):
//   `military-strike` and `civilian-strike` (C001, C002); `display-goto`
//   (C014, C015), Display's buttons closing the report on the system's
//   sector and Fleet windows; `shielded-strike` (C004), 0 hits under shield
//   generators; `assault-captures` and `assault-repelled` (C007-C009);
//   `damaged-survivor` and `damaged-loss` (C012), the damage art under
//   Operational and under Destroyed.
//
//   node orbital-strike.mjs [--no-build] [--only=<faction>/<case>]
//
// The fixture site carries no movies; this server maps /data/movies/ to the
// staged web/data/movies/ (scripts/stage-web-movies.sh).

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
const movies = path.join(root, "web/data/movies");
const browserManifest = JSON.parse(fs.readFileSync(path.join(here, "browser.json"), "utf8"));
const noBuild = process.argv.includes("--no-build");
const only = process.argv.filter((arg) => arg.startsWith("--only=")).map((arg) => arg.slice(7));
const outRoot = process.env.ORBITAL_STRIKE_OUT
  ? path.resolve(process.env.ORBITAL_STRIKE_OUT)
  : path.join(root, ".artifacts/interface-parity");
const runId = `${new Date().toISOString().replace(/[:.]/g, "-")}-${process.pid}`;
const runDir = path.join(outRoot, `orbital-strike-${runId}`);

// Fixture codes are the Scenario index plus one (interface_test_fixture.rs).
const FLEET_MOVE_BLOCKADE = 47;
const OPERATION_REPORT = 67;
const STRIKE_DEATH_STAR = 68;
const STRIKE_SHIELDED = 71;
const ASSAULT_CAPTURES = 72;
const ASSAULT_REPELLED = 73;
const DAMAGED_SURVIVOR = 74;
const DAMAGED_LOSS = 75;

// The galaxy view at 640 by 480 (CockpitState::layout_for). `side` indexes
// the report's per-side tables; `conflict` is rail light 0x13c (cockpit.rs,
// ALLIANCE/EMPIRE_MESSAGE_INDEX_CONTROLS) and its lit art.
const factions = [
  { name: "alliance", byte: 1, side: 0, galaxy: { x: 55, y: 40, width: 485, height: 350 },
    conflict: { x: 3, y: 259, width: 27, height: 22, lit: 10066, resting: 10056 }, highlight: [255, 0, 0] },
  { name: "empire", byte: 2, side: 1, galaxy: { x: 120, y: 40, width: 480, height: 355 },
    conflict: { x: 611, y: 260, width: 27, height: 22, lit: 10046, resting: 10036 }, highlight: [0, 255, 0] },
];

// operation_report.rs (FUN_0044c630), in window coordinates.
const report = {
  size: { width: 470, height: 331 },
  shell: [0x285f, 0x2860],
  railArt: [0x2a44, 0x2a45],
  railX: [412, 415],
  pictureAt: { x: 12, y: 13 },
  close: [{ rect: [423, 25, 32, 31], art: [0x2882, 0x2883] }, { rect: [426, 21, 44, 41], art: [0x2888, 0x2889] }],
  railOrigin: [[418, 88], [426, 89]],
  railButton: [[41, 41], [44, 41]],
  viewY: [[0, 53, 108, 162], [0, 59, 118, 177]],
  views: ["summary", "fleet", "system", "display"],
  viewCommand: [0x65, 0x66, 0x67, 0x68],
  tabsAt: [36, 59],
  tabSize: [49, 41],
  tabX: [0, 62, 124, 186, 248, 312],
  tabCommand: [0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c],
  tabArt: [
    [[0x286c, 0x286b], [0x2878, 0x2877]],
    [[0x2872, 0x2871], [0x287c, 0x287b]],
    [[0x2868, 0x2867], [0x2874, 0x2873]],
    [[0x2866, 0x2865], [0x2866, 0x2865]],
    [[0x2870, 0x286f], [0x287a, 0x2879]],
    [[0x286a, 0x2869], [0x2876, 0x2875]],
  ],
  twoColumn: 0x2a09,
  threeColumn: 0x2a0a,
  displayLayer: 0x2a0b,
  gotoSystem: { rect: [29, 182, 169, 96], art: 0x2a0c },
  gotoFleet: { rect: [226, 182, 169, 96], art: [0x2a0d, 0x2a0e] },
  pictures: { 0x2b9a: "intact", 0x2b9b: "ruined", 0x2b9c: "ship-lost" },
  indexContent: 10822,
};

// A view button's normal and pressed art (ReportView::art): Forces by the
// fleet owner, System Forces by the holder. In every journey here the
// player bombarded the other side's system.
function viewArt(view, side) {
  const owner = side === 0 ? "alliance" : "empire";
  const holder = side === 0 ? "empire" : "alliance";
  const by = (faction) => (faction === "alliance"
    ? [[0x29ea, 0x29eb], [0x29f4, 0x29f5]] : [[0x29ec, 0x29ed], [0x29f6, 0x29f7]])[side];
  return {
    summary: [[0x29e8, 0x29e9], [0x29f2, 0x29f3]][side],
    fleet: by(owner),
    system: by(holder),
    display: [[0x2a27, 0x2a28], [0x2918, 0x2919]][side],
  }[view];
}

// A pixel comparison that fails records its finding and lets the journey
// go on, so one run reports every surface; the case still fails.
let failures = [];
function soft(condition, message) {
  if (!condition) failures.push(message);
}

const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");

function sourceDirectory() {
  const candidates = [process.env.REBELLION_STRATEGY_BMP_DIR, path.join(root, "data/base/ui/strategy-dll/BMP")].filter(Boolean);
  const directory = candidates.find((candidate) => fs.existsSync(path.join(candidate, "10335.bmp")));
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
  if (file.endsWith(".webm")) return "video/webm";
  return "application/octet-stream";
}

async function startServer() {
  const server = http.createServer((request, response) => {
    const pathname = new URL(request.url, "http://localhost").pathname;
    const relative = path.posix.normalize(decodeURIComponent(pathname)).replace(/^\/+/, "") || "index.html";
    const [base, rest] = relative.startsWith("data/movies/") ? [movies, relative.slice(12)] : [site, relative];
    const candidate = path.resolve(base, rest);
    if (!candidate.startsWith(`${base}${path.sep}`)) {
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

// An 8-bit indexed BMP as RGBA; a pixel of the palette-blue matte
// (bmp_cache.rs: r < 32, g < 32, b > 192) gets alpha 0, and one of the
// palette index the first stored pixel holds (bmp_cache.rs's palette-key
// mask) alpha 254.
function decodeBmp(bytes) {
  assert.equal(bytes.toString("ascii", 0, 2), "BM", "source resource is not a BMP");
  const dataOffset = bytes.readUInt32LE(10);
  const headerSize = bytes.readUInt32LE(14);
  const width = bytes.readInt32LE(18);
  const signedHeight = bytes.readInt32LE(22);
  const height = Math.abs(signedHeight);
  assert.deepEqual([bytes.readUInt16LE(28), bytes.readUInt32LE(30)], [8, 0], "source resource is not 8-bit uncompressed");
  const paletteOffset = 14 + headerSize;
  const stride = (width + 3) & ~3;
  const png = new PNG({ width, height });
  const keyIndex = bytes[dataOffset];
  for (let y = 0; y < height; y += 1) {
    const sourceY = signedHeight > 0 ? height - 1 - y : y;
    for (let x = 0; x < width; x += 1) {
      const index = bytes[dataOffset + sourceY * stride + x];
      const palette = paletteOffset + index * 4;
      const at = (y * width + x) * 4;
      const [r, g, b] = [bytes[palette + 2], bytes[palette + 1], bytes[palette]];
      png.data[at] = r;
      png.data[at + 1] = g;
      png.data[at + 2] = b;
      png.data[at + 3] = r < 32 && g < 32 && b > 192 ? 0 : index === keyIndex ? 254 : 255;
    }
  }
  return png;
}

const resourceCache = new Map();
function resource(source, id) {
  if (!resourceCache.has(id)) resourceCache.set(id, decodeBmp(fs.readFileSync(path.join(source, `${id}.bmp`))));
  return resourceCache.get(id);
}

function blank(width, height) {
  const png = new PNG({ width, height });
  for (let index = 0; index < width * height; index += 1) png.data[index * 4 + 3] = 255;
  return png;
}

// Paint `image` keyed: its matte-blue pixels leave what is under them. A
// button's bitmap is also keyed on its first stored pixel's palette index:
// CoolStrobeButton (FUN_00602d30) blits through FUN_005fd0f0 on that index
// unless its +0xc4 bit 0x80 is set, and FUN_00602150 leaves the bit clear.
// A button's control clips its bitmap to `clip` (FUN_005fc140 at native
// size). An `opaque` bitmap is copied whole, its matte included.
function blit(destination, image, x, y, button = false, clip = null, opaque = false) {
  const [width, height] = clip ? [Math.min(clip[0], image.width), Math.min(clip[1], image.height)] : [image.width, image.height];
  for (let row = 0; row < height; row += 1) {
    for (let col = 0; col < width; col += 1) {
      const dx = x + col;
      const dy = y + row;
      if (dx < 0 || dy < 0 || dx >= destination.width || dy >= destination.height) continue;
      const from = (row * image.width + col) * 4;
      const alpha = image.data[from + 3];
      if (!opaque && (alpha === 0 || (button && alpha === 254))) continue;
      const to = (dy * destination.width + dx) * 4;
      destination.data[to] = image.data[from];
      destination.data[to + 1] = image.data[from + 1];
      destination.data[to + 2] = image.data[from + 2];
      destination.data[to + 3] = 255;
    }
  }
}

function crop(screenshot, rect) {
  const out = new PNG({ width: rect.width, height: rect.height });
  for (let y = 0; y < rect.height; y += 1) {
    const start = ((rect.y + y) * screenshot.width + rect.x) * 4;
    out.data.set(screenshot.data.subarray(start, start + rect.width * 4), y * rect.width * 4);
  }
  return out;
}

const within = (box, x, y) => x >= box[0] && x < box[0] + box[2] && y >= box[1] && y < box[1] + box[3];

// Pixel-exact compare outside `masks`; each mask's changed pixels are
// counted as its content.
function compare(actual, expected, masks, directory, label) {
  let different = 0;
  let checked = 0;
  const content = Object.fromEntries(Object.keys(masks).map((name) => [name, 0]));
  const diff = new PNG({ width: expected.width, height: expected.height });
  for (let y = 0; y < expected.height; y += 1) {
    for (let x = 0; x < expected.width; x += 1) {
      const at = (y * expected.width + x) * 4;
      const same = [0, 1, 2].every((index) => actual.data[at + index] === expected.data[at + index]);
      const mask = Object.entries(masks).find(([, box]) => within(box, x, y));
      diff.data[at + 3] = 255;
      if (mask) {
        if (!same) content[mask[0]] += 1;
        diff.data[at + 2] = 96;
        continue;
      }
      checked += 1;
      if (!same) {
        different += 1;
        diff.data[at] = 255;
      }
    }
  }
  fs.writeFileSync(path.join(directory, `${label}-actual.png`), PNG.sync.write(actual));
  fs.writeFileSync(path.join(directory, `${label}-expected.png`), PNG.sync.write(expected));
  fs.writeFileSync(path.join(directory, `${label}-diff.png`), PNG.sync.write(diff));
  return { pixels_checked: checked, different_pixels: different, changed_in_masks: content };
}

// The report window as FUN_0044c630 composes it: black, the layer's picture
// at (12, 13) unkeyed, the shell and the rail art keyed over it, then Close and the
// rail of four views (the shown view pressed), keyed as buttons (blit).
// Over a list the six
// tabs follow (the shown tab pressed); Display adds its two buttons.
function composeReport(source, side, { picture, view, tab = 0 }) {
  const image = blank(report.size.width, report.size.height);
  // The layer's picture is copied whole: in the VM A0 capture of the Orto
  // bombardment's 0x2b9c (sha256 2b9ae014...) none of the bitmap's 386
  // 0x0000c4 pixels shows black; 204 show that blue.
  blit(image, resource(source, picture), report.pictureAt.x, report.pictureAt.y, false, null, true);
  blit(image, resource(source, report.shell[side]), 0, 0);
  blit(image, resource(source, report.railArt[side]), report.railX[side], 0);
  const close = report.close[side];
  blit(image, resource(source, close.art[0]), close.rect[0], close.rect[1], true, close.rect.slice(2));
  const [railX, railY] = report.railOrigin[side];
  report.views.forEach((name, index) => {
    const art = viewArt(name, side);
    blit(image, resource(source, art[name === view ? 1 : 0]), railX, railY + report.viewY[side][index], true, report.railButton[side]);
  });
  if (view === "fleet" || view === "system") {
    report.tabX.forEach((x, index) => {
      const art = report.tabArt[index][side];
      blit(image, resource(source, art[index === tab ? 1 : 0]), report.tabsAt[0] + x, report.tabsAt[1], true, report.tabSize);
    });
  }
  if (view === "display") {
    blit(image, resource(source, report.gotoSystem.art), report.gotoSystem.rect[0], report.gotoSystem.rect[1], true, report.gotoSystem.rect.slice(2));
    blit(image, resource(source, report.gotoFleet.art[side]), report.gotoFleet.rect[0], report.gotoFleet.rect[1], true, report.gotoFleet.rect.slice(2));
  }
  return image;
}

// The text the window draws over its art (operation_report.rs): the title
// (font 0xb, centred in 400 at (12, 13)), the Summary's body at (25, 225),
// the view and tab names, the column headers, the list and its bar, and
// Display's text and button captions.
function reportMasks(view) {
  const masks = { title: [12, 13, 400, 24] };
  if (view === "summary") masks.body = [24, 224, 362, 96];
  if (view === "fleet" || view === "system") {
    masks.subtitle = [12, 36, 347, 20];
    masks.tab_name = [38, 100, 300, 16];
    masks.headers = [37, 117, 347, 18];
    masks.list = [37, 136, 375, 166];
  }
  if (view === "display") {
    masks.display_text = [29, 120, 350, 58];
    // A caption's line: FUN_006026e0 centres its 16-pixel font-4 line in
    // the 96-pixel button and FUN_006027e0 moves it down 36
    // (FUN_0044c630.c:851, :909), so its top is 182 + 40 + 36. The VM A0
    // capture of the Selonia Display draws "Selonia" on window rows 261-270.
    masks.system_caption = [29, 258, 169, 16];
    masks.fleet_caption = [226, 258, 169, 16];
  }
  return masks;
}

// What fills a 470 by 331 window at `origin`: the fraction of the shell's
// 400 by 310 aperture whose pixels equal each candidate's. Text, rows and
// controls drawn over it lower every score alike, so the best must reach
// 0.6 and lead the next by 0.3.
function classifyWindow(screenshot, origin, source, side) {
  const candidates = {
    ...Object.fromEntries(Object.entries(report.pictures).map(([id, name]) => [`report:${name}`, Number(id)])),
    "index-content": report.indexContent,
    "layer:two-column": report.twoColumn,
    "layer:three-column": report.threeColumn,
    "layer:display": report.displayLayer,
  };
  const shell = resource(source, report.shell[side]);
  const scores = {};
  for (const [name, id] of Object.entries(candidates)) {
    const picture = resource(source, id);
    let checked = 0;
    let equal = 0;
    for (let y = 13; y < 323; y += 1) {
      for (let x = 12; x < 412; x += 1) {
        if (shell.data[(y * shell.width + x) * 4 + 3] !== 0) continue;
        const px = x - report.pictureAt.x;
        const py = y - report.pictureAt.y;
        if (px >= picture.width || py >= picture.height) continue;
        const from = (py * picture.width + px) * 4;
        const sx = origin.x + x;
        const sy = origin.y + y;
        if (sx < 0 || sy < 0 || sx >= screenshot.width || sy >= screenshot.height) continue;
        const at = (sy * screenshot.width + sx) * 4;
        checked += 1;
        if ([0, 1, 2].every((index) => screenshot.data[at + index] === picture.data[from + index])) equal += 1;
      }
    }
    scores[name] = checked ? Number((equal / checked).toFixed(4)) : 0;
  }
  const [[best, score], [, next]] = Object.entries(scores).sort((a, b) => b[1] - a[1]);
  return { shown: score >= 0.6 && score - next >= 0.3 ? best : "none", scores };
}

// The galaxy-centred window rect (mission_dialog::galaxy_centered_rect).
function centred(faction, size) {
  const { galaxy } = faction;
  return {
    x: Math.round(galaxy.x + galaxy.width / 2 - size.width / 2),
    y: Math.round(galaxy.y + galaxy.height / 2 - size.height / 2),
    width: size.width,
    height: size.height,
  };
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

// Two presses a frame apart, each seen by egui, inside its double-click
// time.
async function doubleClick(page, point) {
  await page.mouse.move(point.x, point.y);
  await frames(page);
  for (let press = 1; press <= 2; press += 1) {
    await page.mouse.down({ clickCount: press });
    await frames(page);
    await page.mouse.up({ clickCount: press });
    await frames(page);
  }
  await frames(page, 2);
}

async function stableShot(page, directory, label) {
  await page.waitForTimeout(150);
  await frames(page, 2);
  const first = await page.screenshot({ animations: "disabled" });
  await frames(page);
  const bytes = await page.screenshot({ animations: "disabled" });
  fs.writeFileSync(path.join(directory, `${label}.png`), bytes);
  return { bytes, png: PNG.sync.read(bytes), stable: sha256(first) === sha256(bytes), sha256: sha256(bytes), label };
}

// The text pixels of one menu row: white (enabled), 0x808080 (disabled,
// game_menu.rs DISABLED_COLOR) or the side's highlight.
function rowInk(png, rect, highlight) {
  const ink = { white: 0, grey: 0, highlight: 0 };
  for (let y = Math.ceil(rect.y); y < Math.floor(rect.y + rect.height); y += 1) {
    for (let x = Math.ceil(rect.x); x < Math.floor(rect.x + rect.width); x += 1) {
      const at = (y * png.width + x) * 4;
      const [r, g, b] = [png.data[at], png.data[at + 1], png.data[at + 2]];
      if (r === 255 && g === 255 && b === 255) ink.white += 1;
      else if (r === 128 && g === 128 && b === 128) ink.grey += 1;
      else if (r === highlight[0] && g === highlight[1] && b === highlight[2]) ink.highlight += 1;
    }
  }
  const state = ink.grey > 0 && ink.white === 0 && ink.highlight === 0 ? "disabled"
    : ink.grey === 0 && (ink.white > 0 || ink.highlight > 0) ? "enabled" : "unclear";
  return { ...ink, state };
}

// The bounding box of pixels that differ between two captures, outside
// `exclude`.
function changedBox(before, after, exclude) {
  let box = null;
  for (let y = 0; y < before.height; y += 1) {
    for (let x = 0; x < before.width; x += 1) {
      if (exclude && x >= exclude.x && x < exclude.x + exclude.width && y >= exclude.y && y < exclude.y + exclude.height) continue;
      const at = (y * before.width + x) * 4;
      if ([0, 1, 2].every((index) => before.data[at + index] === after.data[at + index])) continue;
      box = box
        ? { x0: Math.min(box.x0, x), y0: Math.min(box.y0, y), x1: Math.max(box.x1, x), y1: Math.max(box.y1, y) }
        : { x0: x, y0: y, x1: x, y1: y };
    }
  }
  return box && { x: box.x0, y: box.y0, width: box.x1 - box.x0 + 1, height: box.y1 - box.y0 + 1 };
}

const menuRect = (menu) => ({ x: Math.floor(menu.left), y: Math.floor(menu.top), width: Math.ceil(menu.width), height: Math.ceil(menu.height) });

// Rows below a 2-pixel border share one height (object_menu.rs). The frame
// (cockpit.rs paint_gid_frame_border, STRATEGY 10100-10107: white and
// 0xc0c0c0 dots) is drawn 2 pixels deep over every edge last, so the last
// row ends above the bottom edge.
function rowRect(rect, rows, row) {
  const height = (rect.height - 2) / rows;
  const y = rect.y + 2 + height * row;
  return { x: rect.x + 2, y, width: rect.width - 4, height: Math.min(height, rect.y + rect.height - 2 - y) };
}

const centre = (rect) => ({ x: Math.round(rect.x + rect.width / 2), y: Math.round(rect.y + rect.height / 2) });

async function fleetSetup(page) {
  await page.waitForFunction(() => window.__openRebellionInterfaceFleetMoveSetup
    && (window.__openRebellionInterfaceFleetMoves || []).length > 0, null, { timeout: 10_000 });
  return page.evaluate(() => window.__openRebellionInterfaceFleetMoveSetup);
}

async function openFleetMenu(page, setup) {
  await page.evaluate(() => { window.__openRebellionInterfaceObjectMenu = undefined; });
  await click(page, { x: Math.round(setup.fleet_item_x), y: Math.round(setup.fleet_item_y) }, "right");
  await page.waitForFunction(() => window.__openRebellionInterfaceObjectMenu?.status === "object-menu", null, { timeout: 10_000 });
  return page.evaluate(() => window.__openRebellionInterfaceObjectMenu);
}

// Open the fleet's menu, read its Planetary Bombardment row, hover it past
// the 500 ms submenu timer and read the four rows of the submenu that shows.
async function bombardmentMenu(page, faction, setup, directory, label) {
  const menu = await openFleetMenu(page, setup);
  assert.notEqual(menu.bombardment_row, null, "the fleet's menu lists Planetary Bombardment");
  const root = menuRect(menu);
  const parent = rowRect(root, menu.rows, menu.bombardment_row);
  await page.mouse.move(root.x + root.width - 3, root.y + root.height + 20);
  const before = await stableShot(page, directory, `${label}-menu`);
  const parentInk = rowInk(before.png, parent, faction.highlight);
  await page.mouse.move(Math.round(parent.x + parent.width / 2), Math.round(parent.y + parent.height / 2));
  await page.waitForTimeout(900);
  const hovered = await stableShot(page, directory, `${label}-submenu`);
  const child = changedBox(before.png, hovered.png, root);
  const rows = child ? [0, 1, 2, 3].map((row) => rowInk(hovered.png, rowRect(child, 4, row), faction.highlight)) : [];
  return { menu, root, parent, parent_ink: parentInk, child, rows, screenshots: [before.sha256, hovered.sha256] };
}


// The GOKRES bitmaps a list cell stacks (operation_report.rs cell_layers).
const gokresDirectory = path.join(root, "data/base/ui/gokres-dll/BMP");
const gokresCache = new Map();
function gokres(id) {
  if (!gokresCache.has(id)) gokresCache.set(id, decodeBmp(fs.readFileSync(path.join(gokresDirectory, `${id}.bmp`))));
  return gokresCache.get(id);
}

// The live report the fleet scenarios emit as it changes
// (OperationReportWatch::observe_live): its view, text, picture, lists with
// each cell's bitmaps, and the z serials of its system's sector and Fleet
// windows.
async function liveReports(page) {
  return page.evaluate(() => (window.__openRebellionInterfaceOperationReports || []).filter((state) => state.live));
}

async function untilLive(page, description, test) {
  try {
    await page.waitForFunction((source) => {
      const last = (window.__openRebellionInterfaceOperationReports || []).filter((state) => state.live).at(-1);
      return Boolean(last) && new Function("state", `return (${source})(state);`)(last);
    }, test.toString(), { timeout: 10_000 });
  } catch (error) {
    throw new Error(`${description}: ${JSON.stringify((await liveReports(page)).slice(-2))}`, { cause: error });
  }
  return (await liveReports(page)).at(-1);
}

async function consoleLine(consoleLines, pattern, description) {
  const deadline = Date.now() + 5_000;
  while (Date.now() < deadline) {
    const line = consoleLines.find(({ text }) => pattern.test(text));
    if (line) return line.text;
    await new Promise((resolve) => setTimeout(resolve, 50));
  }
  throw new Error(`${description}: no console line ${pattern}`);
}

// A Planetary Bombardment row's order from the fleet's menu (0 Target
// Military Facilities, 1 Target Civilian Facilities, 2 General
// Bombardment), as the live-strike journey orders General Bombardment.
async function orderStrike(page, faction, directory, row) {
  const setup = await fleetSetup(page);
  const opened = await bombardmentMenu(page, faction, setup, directory, "fleet");
  assert.ok(opened.child, "no submenu showed under Planetary Bombardment");
  assert.equal(opened.rows[row].state, "enabled", JSON.stringify(opened.rows));
  const point = centre(rowRect(opened.child, 4, row));
  await page.mouse.move(point.x, point.y);
  await frames(page, 2);
  await click(page, point);
  await page.mouse.move(2, 2);
  return opened;
}

// The fleet menu's Planetary Assault row (order 0x234, FUN_005814d0).
async function orderAssault(page) {
  const setup = await fleetSetup(page);
  const menu = await openFleetMenu(page, setup);
  assert.notEqual(menu.assault_row, null, "the fleet's menu lists Planetary Assault");
  const row = rowRect(menuRect(menu), menu.rows, menu.assault_row);
  const point = centre(row);
  await page.mouse.move(point.x, point.y);
  await frames(page, 2);
  await click(page, point);
  await page.mouse.move(2, 2);
  return menu;
}

// The report's Summary as the order opened it at once: the live report's
// own picture through the shell, compared exactly outside its text.
async function liveSummary(page, faction, source, directory, label) {
  const state = await untilLive(page, "the order opened its report at once", (live) => live.open && live.view === 0x65);
  const window = centred(faction, report.size);
  const shot = await stableShot(page, directory, label);
  const checked = compare(crop(shot.png, window), composeReport(source, faction.side, { picture: state.picture, view: "summary" }), reportMasks("summary"), directory, label);
  soft(checked.different_pixels === 0, `${label}: ${JSON.stringify(checked)}`);
  assert.ok(checked.changed_in_masks.title > 0 && checked.changed_in_masks.body > 0, `${label} shows no title or text`);
  return { state, window, summary: checked };
}

// A damaged object's list cell (FUN_0042c3b0.c:350-410): its GOKRES damage
// art, portrait + 0x2000, under the portrait, which is keyed on its matte,
// so the burns show where the portrait leaves the art uncovered (the Wine
// capture of alliance-day927-orto-orbit.sav's Corellian Corvette 1 under
// Destroyed matches this stack pixel for pixel). The cell is the tab's
// 165 by 70 cell in `column` of the first row, the picture centred
// (operation_report.rs, LIST and ReportTab::cell).
function damagedCell(png, window, column, cell) {
  assert.equal(cell.layers.length, 2, `the damaged cell's bitmaps: ${JSON.stringify(cell)}`);
  const [[burnSource, burnId], [portraitSource, portraitId]] = cell.layers;
  assert.deepEqual([burnSource, portraitSource], ["Gokres", "Gokres"], JSON.stringify(cell));
  assert.equal(burnId, portraitId + 0x2000, `the damage art of ${cell.name}`);
  const burn = gokres(burnId);
  const portrait = gokres(portraitId);
  const x0 = window.x + 37 + 165 * column + Math.floor((165 - burn.width) / 2);
  const y0 = window.y + 136 + Math.floor((70 - burn.height) / 2);
  const counts = { burn_only: 0, matched: 0, mismatched: 0 };
  for (let y = 0; y < burn.height; y += 1) {
    for (let x = 0; x < burn.width; x += 1) {
      const from = (y * burn.width + x) * 4;
      const covered = x < portrait.width && y < portrait.height && portrait.data[(y * portrait.width + x) * 4 + 3] !== 0;
      const layer = covered ? portrait : burn;
      const at = covered ? (y * portrait.width + x) * 4 : from;
      if (layer.data[at + 3] === 0) continue;
      if (!covered) counts.burn_only += 1;
      const to = ((y0 + y) * png.width + x0 + x) * 4;
      const same = [0, 1, 2].every((index) => Math.abs(png.data[to + index] - layer.data[at + index]) <= 2);
      counts[same ? "matched" : "mismatched"] += 1;
    }
  }
  return { name: cell.name, burn: burnId, portrait: portraitId, at: [x0, y0], ...counts };
}

// The strike scenarios' journey to a list: the rail's Forces view of the
// fleet's side (1) or the system's (2), on its first tab.
async function forcesList(page, faction, window, view, directory, label) {
  const side = faction.side;
  const [railX, railY] = report.railOrigin[side];
  const [buttonW, buttonH] = report.railButton[side];
  await click(page, { x: window.x + railX + buttonW / 2, y: window.y + railY + report.viewY[side][view] + buttonH / 2 });
  await page.mouse.move(2, 2);
  await untilLive(page, `Forces view ${view}`, (live) => live.open && live.view !== 0x65);
  return stableShot(page, directory, label);
}

async function strikeCell(page, faction, source, directory, consoleLines, row, order) {
  await orderStrike(page, faction, directory, row);
  const checks = {};
  checks.log = await consoleLine(consoleLines, new RegExp(`command=${order} destination=bombardment status=issued runs=1 hits=\\d+`), `order ${order}`);
  Object.assign(checks, await liveSummary(page, faction, source, directory, "report-alert"));
  assert.match(checks.state.title, /^Orbital bombardment of /, JSON.stringify(checks.state));
  assert.equal(checks.state.assault, false);
  return checks;
}

const cases = [
  {
    name: "report-window",
    code: OPERATION_REPORT,
    async run(page, faction, source, directory) {
      const side = faction.side;
      const origin = { x: 85, y: 55 };
      const rect = { ...origin, ...report.size };
      const states = () => page.evaluate(() => window.__openRebellionInterfaceOperationReports || []);
      const until = async (description, view, tab, open = true) => {
        try {
          await page.waitForFunction(([v, t, o]) => {
            const last = (window.__openRebellionInterfaceOperationReports || []).at(-1);
            return last && last.open === o && (!o || (last.view === v && last.tab === t));
          }, [view, tab, open], { timeout: 10_000 });
        } catch (error) {
          throw new Error(`${description}: ${JSON.stringify(await states(page))}`, { cause: error });
        }
      };
      const checks = {};
      await until("the fixture opens the report on Summary and tab 0x97", 0x65, 0x97);
      await page.mouse.move(2, 2);
      // Summary: picture 0x2b9b (the run killed a regiment and a shipyard,
      // OperationReport::picture) through the shell's blue aperture.
      let shot = await stableShot(page, directory, "summary");
      checks.summary = {
        stable: shot.stable,
        classified: classifyWindow(shot.png, origin, source, side),
        ...compare(crop(shot.png, rect), composeReport(source, side, { picture: 0x2b9b, view: "summary" }), reportMasks("summary"), directory, "summary"),
      };
      assert.equal(checks.summary.classified.shown, "report:ruined", JSON.stringify(checks.summary.classified));
      soft(checks.summary.different_pixels === 0, `Summary: ${JSON.stringify(checks.summary)}`);
      assert.ok(checks.summary.changed_in_masks.title > 0 && checks.summary.changed_in_masks.body > 0, "Summary shows no title or text");

      const [railX, railY] = report.railOrigin[side];
      const [buttonW, buttonH] = report.railButton[side];
      const viewPoint = (index) => ({ x: origin.x + railX + buttonW / 2, y: origin.y + railY + report.viewY[side][index] + buttonH / 2 });
      const tabPoint = (index) => ({ x: origin.x + report.tabsAt[0] + report.tabX[index] + report.tabSize[0] / 2, y: origin.y + report.tabsAt[1] + report.tabSize[1] / 2 });

      // Both Forces views: the six tabs, each its layer.
      checks.tabs = [];
      for (const [viewIndex, view] of [[1, "fleet"], [2, "system"]]) {
        await click(page, viewPoint(viewIndex));
        for (let tab = 0; tab < 6; tab += 1) {
          if (tab > 0 || viewIndex === 2) await click(page, tabPoint(tab));
          await until(`view ${view} tab ${tab}`, report.viewCommand[viewIndex], report.tabCommand[tab]);
          await page.mouse.move(2, 2);
          shot = await stableShot(page, directory, `${view}-tab-${tab}`);
          const picture = tab === 5 ? report.threeColumn : report.twoColumn;
          const result = compare(crop(shot.png, rect), composeReport(source, side, { picture, view, tab }), reportMasks(view), directory, `${view}-tab-${tab}`);
          checks.tabs.push({ view, tab: report.tabCommand[tab], stable: shot.stable, ...result });
          soft(result.different_pixels === 0, `${view} tab ${tab}: ${JSON.stringify(result)}`);
          assert.ok(result.changed_in_masks.headers > 0, `${view} tab ${tab} draws no headers`);
          assert.ok(result.changed_in_masks.list > 0, `${view} tab ${tab} draws no list`);
        }
      }
      // Personnel's headers: Survivors, Captured, Killed over its three
      // 110-wide columns at x 38, 165 and 275 (FUN_0044c630.c:264-328).
      shot = PNG.sync.read(fs.readFileSync(path.join(directory, "system-tab-5-actual.png")));
      const layer = resource(source, report.threeColumn);
      checks.personnel_headers = [38, 165, 275].map((x) => {
        let ink = 0;
        for (let y = 117; y < 133; y += 1) {
          for (let dx = 0; dx < 110; dx += 1) {
            const at = (y * shot.width + x + dx) * 4;
            const from = ((y - 13) * layer.width + x + dx - 12) * 4;
            if (![0, 1, 2].every((index) => shot.data[at + index] === layer.data[from + index])) ink += 1;
          }
        }
        return ink;
      });
      assert.ok(checks.personnel_headers.every((ink) => ink > 0), `a Personnel column has no header: ${checks.personnel_headers}`);

      // Display: its layer and the two buttons.
      await click(page, viewPoint(3));
      // The list keeps its tab (FUN_0044e3d0), Personnel from the last click.
      await until("Display", 0x68, 0x9c);
      await page.mouse.move(2, 2);
      shot = await stableShot(page, directory, "display");
      checks.display = compare(crop(shot.png, rect), composeReport(source, side, { picture: report.displayLayer, view: "display" }), reportMasks("display"), directory, "display");
      soft(checks.display.different_pixels === 0, `Display: ${JSON.stringify(checks.display)}`);
      assert.ok(checks.display.changed_in_masks.system_caption > 0 && checks.display.changed_in_masks.fleet_caption > 0,
        `Display draws no button caption: ${JSON.stringify(checks.display.changed_in_masks)}`);

      // Close (0x0a) closes it.
      const close = report.close[side].rect;
      await click(page, { x: origin.x + close[0] + close[2] / 2, y: origin.y + close[1] + close[3] / 2 });
      await until("Close closes the report", 0, 0, false);
      shot = await stableShot(page, directory, "closed");
      checks.closed = classifyWindow(shot.png, origin, source, side);
      assert.equal(checks.closed.shown, "none", JSON.stringify(checks.closed));
      checks.states = await states();
      return checks;
    },
  },
  {
    name: "live-strike",
    code: FLEET_MOVE_BLOCKADE,
    async run(page, faction, source, directory, consoleLines) {
      const side = faction.side;
      const window = centred(faction, report.size);
      const setup = await fleetSetup(page);
      const checks = { setup: { primary_dat_id: setup.primary_dat_id, fleet_item: [setup.fleet_item_x, setup.fleet_item_y] } };
      let shot = await stableShot(page, directory, "ready");
      checks.conflict_light_before = compare(crop(shot.png, faction.conflict), resource(source, faction.conflict.resting), {}, directory, "conflict-before");

      const opened = await bombardmentMenu(page, faction, setup, directory, "fleet");
      checks.menu = opened;
      assert.equal(opened.parent_ink.state, "enabled", `the Planetary Bombardment row: ${JSON.stringify(opened.parent_ink)}`);
      assert.ok(opened.child, "no submenu showed under Planetary Bombardment");
      assert.deepEqual(opened.rows.map(({ state }) => state), ["enabled", "enabled", "enabled", "disabled"],
        JSON.stringify(opened.rows));

      // The grey Destroy System row takes no click: the menu stays and no
      // report opens.
      const destroy = centre(rowRect(opened.child, 4, 3));
      await click(page, destroy);
      shot = await stableShot(page, directory, "destroy-disabled-clicked");
      checks.disabled_click = {
        report: classifyWindow(shot.png, window, source, side),
        submenu_still_shown: changedBox(PNG.sync.read(fs.readFileSync(path.join(directory, "fleet-menu.png"))), shot.png, opened.root) !== null,
      };
      assert.equal(checks.disabled_click.report.shown, "none", "a disabled row issued an order");
      assert.ok(checks.disabled_click.submenu_still_shown, "the menu closed on a disabled row");

      // General Bombardment opens the report at once.
      const general = centre(rowRect(opened.child, 4, 2));
      await page.mouse.move(general.x, general.y);
      await frames(page, 2);
      await click(page, general);
      await page.mouse.move(2, 2);
      shot = await stableShot(page, directory, "report-alert");
      checks.alert = classifyWindow(shot.png, window, source, side);
      assert.match(checks.alert.shown, /^report:/, `the report did not open at once: ${JSON.stringify(checks.alert)}`);
      const picture = Number(Object.entries(report.pictures).find(([, name]) => `report:${name}` === checks.alert.shown)[0]);
      checks.alert_compare = compare(crop(shot.png, window), composeReport(source, side, { picture, view: "summary" }), reportMasks("summary"), directory, "report-alert");
      soft(checks.alert_compare.different_pixels === 0, `the live report: ${JSON.stringify(checks.alert_compare)}`);

      const close = report.close[side].rect;
      const closePoint = { x: window.x + close[0] + close[2] / 2, y: window.y + close[1] + close[3] / 2 };
      await click(page, closePoint);
      await page.mouse.move(2, 2);
      shot = await stableShot(page, directory, "report-closed");
      checks.closed = classifyWindow(shot.png, window, source, side);
      assert.equal(checks.closed.shown, "none", JSON.stringify(checks.closed));

      // The Conflict rail light is lit and opens the Index on Conflict.
      checks.conflict_light_after = compare(crop(shot.png, faction.conflict), resource(source, faction.conflict.lit), {}, directory, "conflict-lit");
      soft(checks.conflict_light_after.different_pixels === 0, `the Conflict light is not lit: ${JSON.stringify(checks.conflict_light_after)}`);
      await click(page, centre(faction.conflict));
      const pattern = /command=0x13c destination=message_index category=0x80 status=opened_original/;
      const deadline = Date.now() + 5_000;
      while (Date.now() < deadline && !consoleLines.some(({ text }) => pattern.test(text))) await page.waitForTimeout(50);
      checks.index_log = consoleLines.find(({ text }) => pattern.test(text))?.text ?? null;
      assert.ok(checks.index_log, "the Conflict light did not open the Index on Conflict");
      await page.mouse.move(2, 2);
      shot = await stableShot(page, directory, "index-conflict");
      checks.index = classifyWindow(shot.png, window, source, side);
      assert.equal(checks.index.shown, "index-content", JSON.stringify(checks.index));

      // A double click on the report's row reopens the report.
      await doubleClick(page, { x: window.x + 25 + 173, y: window.y + 108 + 10 });
      await page.mouse.move(2, 2);
      shot = await stableShot(page, directory, "report-reopened");
      checks.reopened = classifyWindow(shot.png, window, source, side);
      assert.equal(checks.reopened.shown, checks.alert.shown, `the row did not reopen the report: ${JSON.stringify(checks.reopened)}`);

      // Its Close returns to the Index (FUN_00469d60).
      await click(page, closePoint);
      await page.mouse.move(2, 2);
      shot = await stableShot(page, directory, "index-returned");
      checks.index_returned = classifyWindow(shot.png, window, source, side);
      assert.equal(checks.index_returned.shown, "index-content", JSON.stringify(checks.index_returned));
      return checks;
    },
  },
  {
    name: "death-star-strike",
    code: STRIKE_DEATH_STAR,
    async run(page, faction, source, directory) {
      const setup = await fleetSetup(page);
      const checks = {};
      const opened = await bombardmentMenu(page, faction, setup, directory, "fleet");
      checks.menu = opened;
      assert.equal(opened.parent_ink.state, "enabled", JSON.stringify(opened.parent_ink));
      assert.ok(opened.child, "no submenu showed under Planetary Bombardment");
      assert.deepEqual(opened.rows.map(({ state }) => state), ["enabled", "enabled", "enabled", "enabled"],
        JSON.stringify(opened.rows));

      const destroy = centre(rowRect(opened.child, 4, 3));
      await page.mouse.move(destroy.x, destroy.y);
      await frames(page, 2);
      await click(page, destroy);
      await page.mouse.move(2, 2);
      // Movie 0x6f (MDATA.101) plays over the picture alone: 640 by 324,
      // centred (movies::video_bounds), the caption field at (4, 404).
      await page.waitForFunction(() => {
        const video = document.querySelector("video.movie");
        return video && video.currentTime > 0.5;
      }, null, { timeout: 15_000 });
      const video = await page.evaluate(() => {
        const element = document.querySelector("video.movie");
        const box = element.getBoundingClientRect();
        return { src: element.getAttribute("src"), rect: [box.x, box.y, box.width, box.height], video: [element.videoWidth, element.videoHeight], muted: element.muted, time: element.currentTime };
      });
      checks.video = video;
      const shot = await stableShot(page, directory, "movie");
      checks.movie_screen_sha256 = shot.sha256;
      assert.equal(video.src, "data/movies/101.webm");
      video.rect.forEach((value, index) => assert.ok(Math.abs(value - [0, 78, 640, 324][index]) < 0.5, `the video is not over the picture: ${video.rect}`));
      // The caption: canvas pixels lit in the field below the video.
      let ink = 0;
      for (let y = 404; y < 474; y += 1) {
        for (let x = 4; x < 584; x += 1) {
          const at = (y * shot.png.width + x) * 4;
          if (shot.png.data[at] + shot.png.data[at + 1] + shot.png.data[at + 2] > 300) ink += 1;
        }
      }
      checks.caption_ink = ink;
      assert.ok(ink > 50, `no caption below the movie (${ink} lit pixels)`);
      await page.keyboard.press("Escape");
      await page.waitForFunction(() => !document.querySelector("video.movie"), null, { timeout: 5_000 });
      await frames(page, 3);

      // The destroyed system: the parent row is grey and shows no submenu.
      const after = await bombardmentMenu(page, faction, setup, directory, "after");
      checks.after = after;
      assert.equal(after.parent_ink.state, "disabled", `after Destroy System: ${JSON.stringify(after.parent_ink)}`);
      assert.equal(after.child, null, "a disabled parent showed its submenu");
      return checks;
    },
  },
  {
    // EVT-03 C001: Target Military Facilities (0x220) from the fleet's menu
    // opens its report at once (FUN_0048a060).
    name: "military-strike",
    code: FLEET_MOVE_BLOCKADE,
    async run(page, faction, source, directory, consoleLines) {
      return strikeCell(page, faction, source, directory, consoleLines, 0, "0x220");
    },
  },
  {
    // EVT-03 C002: Target Civilian Facilities (0x221).
    name: "civilian-strike",
    code: FLEET_MOVE_BLOCKADE,
    async run(page, faction, source, directory, consoleLines) {
      return strikeCell(page, faction, source, directory, consoleLines, 1, "0x221");
    },
  },
  {
    // EVT-03 C014, C015: Display's buttons close the report and open the
    // system's sector window (0x0b, FUN_00429ce0) or, through the sector
    // window's Fleet icon, its Fleet window (0x0c, FUN_0045c8e0 kind 4,
    // FUN_0045aac0); FUN_0044df90.c:20-35.
    name: "display-goto",
    code: FLEET_MOVE_BLOCKADE,
    async run(page, faction, source, directory, consoleLines) {
      const side = faction.side;
      const checks = await strikeCell(page, faction, source, directory, consoleLines, 2, "0x222");
      const window = checks.window;
      const system = checks.state.title.replace(/^Orbital bombardment of /, "");
      const [railX, railY] = report.railOrigin[side];
      const [buttonW, buttonH] = report.railButton[side];
      const display = { x: window.x + railX + buttonW / 2, y: window.y + railY + report.viewY[side][3] + buttonH / 2 };
      const button = (rect) => ({ x: window.x + rect[0] + rect[2] / 2, y: window.y + rect[1] + rect[3] / 2 });

      await click(page, display);
      await untilLive(page, "Display", (live) => live.open && live.view === 0x68);
      await click(page, button(report.gotoSystem.rect));
      await page.mouse.move(2, 2);
      checks.goto_system = await untilLive(page, "Goto System closes the report on the system's sector window",
        (live) => !live.open && live.destination !== null && live.destination.startsWith("sector:") && live.sector_open);
      assert.equal(checks.goto_system.destination, `sector:${system}`);
      checks.goto_system_shot = (await stableShot(page, directory, "goto-system")).sha256;

      // The Index row reopens the report (FUN_00469de0).
      await click(page, centre(faction.conflict));
      await page.mouse.move(2, 2);
      await stableShot(page, directory, "index-conflict");
      await doubleClick(page, { x: window.x + 25 + 173, y: window.y + 108 + 10 });
      await page.mouse.move(2, 2);
      await untilLive(page, "the Index reopens the report", (live) => live.open);
      await click(page, display);
      await untilLive(page, "Display again", (live) => live.open && live.view === 0x68);
      await click(page, button(report.gotoFleet.rect));
      await page.mouse.move(2, 2);
      checks.goto_fleet = await untilLive(page, "Goto Fleet closes the report on the system's Fleet window",
        (live) => !live.open && live.destination !== null && live.destination.startsWith("fleet:") && live.fleet_window_open);
      assert.equal(checks.goto_fleet.destination, `fleet:${system}`);
      checks.goto_fleet_shot = (await stableShot(page, directory, "goto-fleet")).sha256;
      return checks;
    },
  },
  {
    // EVT-03 C004: shield generators (family 0x24) whose shields reach the
    // fleet's strength take every hit (FUN_0058df60): no hit lands, nothing
    // at the system is destroyed, so the picture is 0x2b9a and System
    // Forces lists the generators operational.
    name: "shielded-strike",
    code: STRIKE_SHIELDED,
    async run(page, faction, source, directory, consoleLines) {
      const checks = await strikeCell(page, faction, source, directory, consoleLines, 0, "0x220");
      assert.match(checks.log, /hits=0$/, checks.log);
      assert.equal(checks.state.picture, 0x2b9a, JSON.stringify(checks.state));
      assert.deepEqual(checks.state.system_destroyed, [], JSON.stringify(checks.state));
      assert.ok(checks.state.system_operational.some((cell) => cell.family === 0x24), JSON.stringify(checks.state.system_operational));
      return checks;
    },
  },
  {
    // EVT-03 C007, C008: Planetary Assault (0x234) on an empty garrison opens
    // the assault's Summary at once, the side's picture (0x2b98 Alliance,
    // 0x2b99 Empire) and the capture text 0x7091 (FUN_00496770 code 2).
    name: "assault-captures",
    code: ASSAULT_CAPTURES,
    async run(page, faction, source, directory, consoleLines) {
      await orderAssault(page);
      const checks = {};
      checks.log = await consoleLine(consoleLines, /command=0x234 destination=assault status=issued runs=1/, "order 0x234");
      Object.assign(checks, await liveSummary(page, faction, source, directory, "assault-summary"));
      assert.equal(checks.state.assault, true);
      assert.equal(checks.state.picture, faction.name === "alliance" ? 0x2b98 : 0x2b99);
      assert.match(checks.state.title, /^Assault on /, JSON.stringify(checks.state));
      assert.match(checks.state.body, /troops have taken control of the /, JSON.stringify(checks.state));
      return checks;
    },
  },
  {
    // EVT-03 C009: one regiment against six; the holder keeps the system and
    // the report reads 0x7092, "... Troops have defended ... assault."
    name: "assault-repelled",
    code: ASSAULT_REPELLED,
    async run(page, faction, source, directory, consoleLines) {
      await orderAssault(page);
      const checks = {};
      checks.log = await consoleLine(consoleLines, /command=0x234 destination=assault status=issued runs=1/, "order 0x234");
      Object.assign(checks, await liveSummary(page, faction, source, directory, "assault-summary"));
      assert.equal(checks.state.assault, true);
      assert.equal(checks.state.picture, faction.name === "alliance" ? 0x2b98 : 0x2b99);
      assert.match(checks.state.body, / Troops have defended .* from an? .* assault\./, JSON.stringify(checks.state));
      return checks;
    },
  },
  {
    // EVT-03 C012, a damaged survivor: listed Operational with its damage
    // art (+0x50 bit 9, FUN_00501c00; FUN_0042c3b0.c:350-410).
    name: "damaged-survivor",
    code: DAMAGED_SURVIVOR,
    async run(page, faction, source, directory, consoleLines) {
      const checks = await strikeCell(page, faction, source, directory, consoleLines, 2, "0x222");
      const ship = checks.state.fleet_operational.find((cell) => cell.family >= 0x14 && cell.family <= 0x1b);
      assert.ok(ship, JSON.stringify(checks.state.fleet_operational));
      const shot = await forcesList(page, faction, checks.window, 1, directory, "fleet-ships");
      checks.cell = damagedCell(shot.png, checks.window, 0, ship);
      assert.ok(checks.cell.burn_only > 0 && checks.cell.mismatched === 0, JSON.stringify(checks.cell));
      return checks;
    },
  },
  {
    // EVT-03 C012, a damaged loss: the batteries' only target dies with its
    // bit and is listed Destroyed with the damage art.
    name: "damaged-loss",
    code: DAMAGED_LOSS,
    async run(page, faction, source, directory, consoleLines) {
      const checks = await strikeCell(page, faction, source, directory, consoleLines, 0, "0x220");
      const ship = checks.state.fleet_destroyed.find((cell) => cell.family >= 0x14 && cell.family <= 0x1b);
      assert.ok(ship, JSON.stringify(checks.state.fleet_destroyed));
      assert.ok(!checks.state.fleet_operational.some((cell) => cell.family >= 0x14 && cell.family <= 0x1b), JSON.stringify(checks.state.fleet_operational));
      const shot = await forcesList(page, faction, checks.window, 1, directory, "fleet-ships");
      checks.cell = damagedCell(shot.png, checks.window, 1, ship);
      assert.ok(checks.cell.burn_only > 0 && checks.cell.mismatched === 0, JSON.stringify(checks.cell));
      return checks;
    },
  },
];

async function inspect(server, source, faction, testCase, executable) {
  const directory = path.join(runDir, faction.name, testCase.name);
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
    browser = await launchBrowser(chromium, { executablePath: executable, headless: true, args: browserManifest.launch_arguments, timeout: 30_000 }, launchAttempts);
    context = await browser.newContext({ viewport: { width: 640, height: 480 }, deviceScaleFactor: 1, locale: "en-US", timezoneId: "America/New_York", colorScheme: "dark", reducedMotion: "reduce", serviceWorkers: "block" });
    page = await context.newPage();
    const serverOrigin = `http://127.0.0.1:${server.address().port}`;
    page.on("response", (response) => {
      if (new URL(response.url()).origin === serverOrigin) requests.push({ url: new URL(response.url()).pathname, status: response.status() });
    });
    page.on("requestfailed", (request) => errors.push(`requestfailed:${request.url()}:${request.failure()?.errorText}`));
    page.on("pageerror", (error) => errors.push(`pageerror:${error.stack || error.message}`));
    page.on("console", (message) => {
      consoleLines.push({ type: message.type(), text: message.text() });
      if (message.type() === "error" || /\[bmp_cache\] asset unavailable/i.test(message.text())) errors.push(`console:${message.type()}:${message.text()}`);
    });
    const fixtureCode = testCase.code | (faction.byte << 8);
    await page.goto(`${serverOrigin}/?fixture-code=${fixtureCode}`, { waitUntil: "load", timeout: 30_000 });
    await page.waitForFunction(() => window.__openRebellionInterfaceReady?.status, null, { timeout: 30_000 });
    const ready = await page.evaluate(() => window.__openRebellionInterfaceReady);
    assert.equal(ready.status, "ready", JSON.stringify(ready));
    assert.equal(ready.code, fixtureCode);
    await page.evaluate(() => document.fonts.ready);
    failures = [];
    const checks = await testCase.run(page, faction, source, directory, consoleLines);
    checks.pixel_failures = failures;
    assert.deepEqual(failures, [], `pixel comparisons failed: ${failures.join(" | ")}`);
    assert.ok(requests.every(({ status }) => status === 200 || status === 206), `non-200 requests: ${JSON.stringify(requests)}`);
    assert.deepEqual(errors, [], `browser diagnostics: ${errors.join("; ")}`);
    result = { status: "pass", faction: faction.name, case: testCase.name, fixture_code: fixtureCode, checks, requests, console: consoleLines, launch_attempts: launchAttempts };
  } catch (error) {
    if (page) await page.screenshot({ path: path.join(directory, "failure.png") }).catch(() => {});
    result = { status: "fail", faction: faction.name, case: testCase.name, error: String(error.stack || error), pixel_failures: failures, requests, errors, console: consoleLines, launch_attempts: launchAttempts };
  } finally {
    if (page) await page.close().catch(() => {});
    if (context) await context.close().catch(() => {});
    if (browser) await browser.close().catch(() => {});
  }
  fs.writeFileSync(path.join(directory, "result.json"), `${JSON.stringify(result, null, 2)}\n`);
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
      for (const testCase of cases) {
        if (only.length && !only.includes(`${faction.name}/${testCase.name}`)) continue;
        const result = await inspect(server, source, faction, testCase, executable);
        results.push(result);
        process.stdout.write(`${result.status === "pass" ? "PASS" : "FAIL"} ${faction.name}/${testCase.name}${result.error ? `: ${result.error.split("\n")[0]}` : ""}\n`);
      }
    }
  } finally {
    await new Promise((resolve) => server.close(resolve));
  }
  const passed = results.length > 0 && results.every((result) => result.status === "pass");
  const wasm = fs.readFileSync(path.join(site, "open-rebellion-test.wasm"));
  const movieHashes = fs.existsSync(movies)
    ? Object.fromEntries(fs.readdirSync(movies).filter((name) => name.endsWith(".webm")).map((name) => [name, sha256(fs.readFileSync(path.join(movies, name)))]))
    : {};
  const summary = {
    schema_version: 1,
    family: "orbital-strike",
    scope: "test-only: EVT-03 Operation Report window (Summary picture through the shell, rail of five, six tabs per Forces view, Personnel's three columns, Display, Close); OBJ-14 fleet Planetary Bombardment row and its four-row submenu enabled and grey, a grey row taking no click, General Bombardment opening the report at once, the Conflict light and the Index reopen; Destroy System's movie 0x6f in the <video> overlay with its canvas caption, and the destroyed system greying the row; EVT-03 C001, C002, C004, C007-C009, C012, C014 and C015 through the fleet's menu and the live report; both sides",
    status: passed ? "pass" : "fail",
    browser_version: browserManifest.version,
    viewport: { width: 640, height: 480, device_scale_factor: 1 },
    wasm_sha256: sha256(wasm),
    index_html_sha256: sha256(fs.readFileSync(path.join(site, "index.html"))),
    movie_sha256: movieHashes,
    results: results.map(({ console: _, ...rest }) => rest),
  };
  fs.writeFileSync(path.join(runDir, "result.json"), `${JSON.stringify(summary, null, 2)}\n`);
  process.stdout.write(`${JSON.stringify({ run_dir: runDir, status: summary.status, cases: results.map(({ faction, case: name, status }) => `${status} ${faction}/${name}`) }, null, 2)}\n`);
  if (!passed) process.exitCode = 1;
}

await main();
