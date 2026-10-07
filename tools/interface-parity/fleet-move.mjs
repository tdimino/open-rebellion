#!/usr/bin/env node

import assert from "node:assert/strict";
import { execFileSync, spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import fs from "node:fs";
import http from "node:http";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright-core";
import { launchBrowser } from "./browser-launch.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const site = path.join(root, ".artifacts/interface-parity/site");
const browserManifest = JSON.parse(fs.readFileSync(path.join(here, "browser.json"), "utf8"));
const noBuild = process.argv.includes("--no-build");
// `--only=alliance/drag` runs the named cases alone.
const only = process.argv.filter((arg) => arg.startsWith("--only=")).map((arg) => arg.slice(7));
const expectedRequests = ["/", "/data/runtime.orpk", "/gl.js", "/open-rebellion-test.wasm"];
// The galaxy view at 640 by 480 (CockpitState::layout_for, FUN_00421c70).
// `sector` is the first sector window column (sector_window.rs,
// window_logical_position), 235 by 360.
const factions = [
  { name: "alliance", byte: 1, galaxy: { x: 55, y: 40, width: 485, height: 350 }, sector: { x: 60, y: 35 } },
  { name: "empire", byte: 2, galaxy: { x: 120, y: 40, width: 480, height: 355 }, sector: { x: 120, y: 40 } },
];
const sectorWindow = { width: 235, height: 360 };
// The map picks the system nearest the pointer within 8.5 pixels
// (draw_galaxy_map's hover radius; the map never zooms).
const PICK_RADIUS = 8;
// Fixture codes are the Scenario index plus one (interface_test_fixture.rs).
const FLEET_MOVE = 46;
const FLEET_MOVE_BLOCKADE = 47;
// FUN_0044f180: the 424 by 331 window, centred on whole pixels in the galaxy
// view as the mission dialog is (hyp: FUN_00606980); checkmark 0x14 and X
// 0x15, 51 by 35, at (355, 244) and (355, 281).
const confirmation = { width: 424, height: 331 };
const checkmark = { x: 355 + 25.5, y: 244 + 17.5 };
const cross = { x: 355 + 25.5, y: 281 + 17.5 };
// The Fleet window the fixture opens 5 pixels in from the galaxy view's
// top-right corner.
const fleetWindow = { width: 235, height: 304 };
const runId = `${new Date().toISOString().replace(/[:.]/g, "-")}-${process.pid}`;
const runDir = path.join(root, ".artifacts/interface-parity", `fleet-move-${runId}`);

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
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

// A left press on the fleet's cell, held across frames while the pointer
// travels, then released (CoolDragList, FUN_006083c0). Returns the last
// observation before the release. `wheel` scrolls that many notches, one a
// frame, while the button is still held.
async function drag(page, from, to, { wheel = 0 } = {}) {
  await page.mouse.move(from.x, from.y);
  await frames(page);
  await page.mouse.down();
  await frames(page);
  await page.mouse.move(to.x, to.y, { steps: 12 });
  await frames(page, 2);
  for (let notch = 0; notch < wheel; notch += 1) {
    await page.mouse.wheel(0, -100);
    await frames(page);
  }
  const held = await latest(page);
  await page.mouse.up();
  await frames(page, 2);
  return held;
}

function observations(page) {
  return page.evaluate(() => window.__openRebellionInterfaceFleetMoves || []);
}

async function latest(page) {
  const list = await observations(page);
  assert.ok(list.length > 0, "the fixture reported no fleet-move observation");
  return list[list.length - 1];
}

async function until(page, description, predicate, argument = null) {
  try {
    await page.waitForFunction(predicate, argument, { timeout: 10_000 });
  } catch (error) {
    throw new Error(`${description}: ${JSON.stringify(await observations(page))}`, { cause: error });
  }
  return latest(page);
}

// Let a refused or ignored action settle: no observation may follow.
async function settle(page) {
  await page.waitForTimeout(250);
  await frames(page, 3);
  return latest(page);
}

function confirmationOrigin(faction) {
  const { galaxy } = faction;
  return {
    x: Math.round(galaxy.x + galaxy.width / 2 - confirmation.width / 2),
    y: Math.round(galaxy.y + galaxy.height / 2 - confirmation.height / 2),
  };
}

function inside(point, rect) {
  return point.x >= rect.x && point.x < rect.x + rect.width
    && point.y >= rect.y && point.y < rect.y + rect.height;
}

// The galaxy view's points that no window covers: outside the sector and
// Fleet windows the fixture opens, and inside `area` when given.
function grown(rect, margin) {
  return { x: rect.x - margin, y: rect.y - margin, width: rect.width + 2 * margin, height: rect.height + 2 * margin };
}

function bare(faction, point, area) {
  const sector = grown({ ...faction.sector, ...sectorWindow }, PICK_RADIUS);
  const fleets = grown({
    x: faction.galaxy.x + faction.galaxy.width - fleetWindow.width - 5,
    y: faction.galaxy.y + 5,
    ...fleetWindow,
  }, PICK_RADIUS);
  // Keep clear of the cockpit frame at the view's edges.
  const view = {
    x: faction.galaxy.x + 6,
    y: faction.galaxy.y + 6,
    width: faction.galaxy.width - 12,
    height: faction.galaxy.height - 12,
  };
  return inside(point, view) && !inside(point, sector) && !inside(point, fleets)
    && (!area || inside(point, area));
}

// The confirmation window's left part, clear of its buttons (x 355 and on).
function confirmationArea(faction) {
  const origin = confirmationOrigin(faction);
  return { x: origin.x + 8, y: origin.y + 8, width: 340, height: confirmation.height - 16 };
}

// For each system other than the fleet's and the target's, the bare point
// nearest it within the pick radius, nearest first.
function probeCandidates(faction, setup, area) {
  const candidates = [];
  const avoided = setup.systems.filter((system) => system.dat_id === setup.primary_dat_id
    || system.dat_id === setup.target_dat_id);
  for (const system of setup.systems) {
    if (avoided.includes(system)) continue;
    // The map picks the nearest system, so stay out of reach of the two it
    // must not pick.
    if (avoided.some((other) => Math.hypot(other.x - system.x, other.y - system.y) < 3 * PICK_RADIUS)) continue;
    let best = null;
    for (let dy = -PICK_RADIUS; dy <= PICK_RADIUS; dy += 1) {
      for (let dx = -PICK_RADIUS; dx <= PICK_RADIUS; dx += 1) {
        const distance = Math.hypot(dx, dy);
        if (distance >= PICK_RADIUS) continue;
        const point = { x: Math.round(system.x) + dx, y: Math.round(system.y) + dy };
        if (bare(faction, point, area) && (!best || distance < best.distance)) best = { ...point, distance };
      }
    }
    if (best) candidates.push({ dat_id: system.dat_id, x: best.x, y: best.y, distance: best.distance });
  }
  return candidates.sort((a, b) => a.distance - b.distance);
}

function probe(faction, setup, area = null) {
  const [first] = probeCandidates(faction, setup, area);
  assert.ok(first, "no bare map point lies within the pick radius of a system");
  return first;
}

// The control, run after the check it backs: a click on the probe must now
// select a system, so the point was bare map. A map click also activates the
// system, which opens its sector window, so it cannot come first.
async function provenBare(page, point) {
  await page.waitForTimeout(600);
  const before = (await settle(page)).selected_system_dat_id;
  await click(page, point);
  const after = await settle(page);
  assert.equal(after.selected_system_dat_id, point.dat_id,
    `the probe ${JSON.stringify(point)} is not bare map (${before} -> ${after.selected_system_dat_id})`);
  return { ...point, selected: after.selected_system_dat_id };
}

async function openMenu(page, setup, directory) {
  await page.evaluate(() => { window.__openRebellionInterfaceObjectMenu = undefined; });
  await click(page, fleetPoint(setup), "right");
  await page.waitForFunction(() => window.__openRebellionInterfaceObjectMenu?.status === "object-menu",
    null, { timeout: 10_000 });
  const menu = await page.evaluate(() => window.__openRebellionInterfaceObjectMenu);
  fs.writeFileSync(path.join(directory, "menu-screen.png"), await page.screenshot({ animations: "disabled" }));
  return menu;
}

// Rows below a 2-pixel border share one height: object menu entries carry
// no icon (object_menu.rs).
async function choose(page, menu, row, name) {
  assert.notEqual(row, null, `the fleet's menu lists ${name}`);
  const height = (menu.height - 2) / menu.rows;
  await click(page, { x: menu.left + menu.width / 2, y: menu.top + 2 + height * (row + 0.5) });
}

function fleetPoint(setup) {
  return { x: Math.round(setup.fleet_item_x), y: Math.round(setup.fleet_item_y) };
}

function targetPoint(setup) {
  return { x: Math.round(setup.target_screen_x), y: Math.round(setup.target_screen_y) };
}

async function shot(page, directory, label) {
  const bytes = await page.screenshot({ animations: "disabled" });
  fs.writeFileSync(path.join(directory, `${label}.png`), bytes);
  return { label, sha256: sha256(bytes) };
}

function inTransitTo(setup) {
  return (observed) => observed.in_transit && observed.destination_dat_id === setup.target_dat_id;
}

// Each case starts from a fresh load and returns its checks.
const cases = [
  {
    name: "move",
    code: FLEET_MOVE,
    // Right-click the fleet, Move, click the target (manual p. 121). Move
    // asks nothing outside a blockade (FUN_00487cc0).
    async run(page, faction, setup, directory) {
      const menu = await openMenu(page, setup, directory);
      await choose(page, menu, menu.move_row, "Move");
      await click(page, targetPoint(setup));
      const observed = await until(page, "the fleet departs for the target",
        (target) => (window.__openRebellionInterfaceFleetMoves || []).some(
          (o) => o.in_transit && o.destination_dat_id === target), setup.target_dat_id);
      assert.ok(inTransitTo(setup)(observed));
      assert.ok((await observations(page)).every((o) => !o.confirmation_open), "Move opened the window");
      return { menu, observed };
    },
  },
  {
    name: "confirmed-move",
    code: FLEET_MOVE,
    // Confirmed Move always asks (0x202); the window takes the pointer, and
    // its checkmark submits the order (FUN_0041ce20).
    async run(page, faction, setup, directory) {
      const point = probe(faction, setup, confirmationArea(faction));
      const menu = await openMenu(page, setup, directory);
      await choose(page, menu, menu.confirmed_move_row, "Confirmed Move");
      await click(page, targetPoint(setup));
      const open = await until(page, "the confirmation window opens",
        () => (window.__openRebellionInterfaceFleetMoves || []).at(-1)?.confirmation_open);
      assert.equal(open.in_transit, false, "the fleet left before the checkmark");
      await page.mouse.move(2, 2);
      await page.waitForTimeout(150);
      await frames(page);
      const opened = await shot(page, directory, "confirmation-open");
      await click(page, point);
      const blocked = await settle(page);
      assert.equal(blocked.selected_system_dat_id, open.selected_system_dat_id,
        "a click inside the window reached the map");
      assert.equal(blocked.confirmation_open, true);
      const origin = confirmationOrigin(faction);
      await click(page, { x: Math.round(origin.x + checkmark.x), y: Math.round(origin.y + checkmark.y) });
      const observed = await until(page, "the checkmark sends the fleet",
        () => {
          const o = (window.__openRebellionInterfaceFleetMoves || []).at(-1);
          return o?.in_transit && !o.confirmation_open;
        });
      assert.ok(inTransitTo(setup)(observed));
      const control = await provenBare(page, point);
      return { menu, probe: control, opened, observed };
    },
  },
  {
    name: "confirmed-move-cancelled",
    code: FLEET_MOVE,
    // The X destroys the order (FUN_0044f640).
    async run(page, faction, setup, directory) {
      const menu = await openMenu(page, setup, directory);
      await choose(page, menu, menu.confirmed_move_row, "Confirmed Move");
      await click(page, targetPoint(setup));
      await until(page, "the confirmation window opens",
        () => (window.__openRebellionInterfaceFleetMoves || []).at(-1)?.confirmation_open);
      const origin = confirmationOrigin(faction);
      await click(page, { x: Math.round(origin.x + cross.x), y: Math.round(origin.y + cross.y) });
      await until(page, "the X closes the window",
        () => (window.__openRebellionInterfaceFleetMoves || []).at(-1)?.confirmation_open === false);
      const observed = await settle(page);
      assert.equal(observed.in_transit, false, "the X sent the fleet");
      assert.equal(observed.destination_dat_id, null);
      return { menu, observed };
    },
  },
  {
    name: "drag",
    code: FLEET_MOVE,
    // A drag out of the Fleet window's list issues 0x201 (FUN_00422ce0,
    // window type 4), which asks nothing outside a blockade and departs at
    // once (FUN_00487cc0).
    async run(page, faction, setup) {
      await drag(page, fleetPoint(setup), targetPoint(setup));
      const observed = await until(page, "the dragged fleet departs",
        (target) => (window.__openRebellionInterfaceFleetMoves || []).some(
          (o) => o.in_transit && o.destination_dat_id === target), setup.target_dat_id);
      assert.ok(inTransitTo(setup)(observed));
      assert.ok((await observations(page)).every((o) => !o.confirmation_open), "a drag opened the window");
      return { observed };
    },
  },
  {
    name: "drag-to-bare-map",
    code: FLEET_MOVE,
    // A drop on no window has no destination; the list held the mouse, so the
    // map under the release selects nothing, and the wheel turned mid-drag
    // changes nothing (the map never zooms, FUN_00422ce0).
    async run(page, faction, setup) {
      const point = probe(faction, setup);
      const held = await drag(page, fleetPoint(setup), point, { wheel: 6 });
      const observed = await settle(page);
      assert.equal(observed.in_transit, false, "a drop on the bare map moved the fleet");
      assert.equal(observed.selected_system_dat_id, held.selected_system_dat_id, "the release reached the map");
      const control = await provenBare(page, point);
      return { probe: control, held, observed };
    },
  },
  {
    name: "blockade-drag",
    code: FLEET_MOVE_BLOCKADE,
    // The drag's 0x201 asks only when the blockaded system is the fleet's
    // own side (FUN_00487cc0); here the other side holds it, so the fleet
    // departs without the window.
    async run(page, faction, setup) {
      await drag(page, fleetPoint(setup), targetPoint(setup));
      const observed = await until(page, "the blockaded fleet departs",
        (target) => (window.__openRebellionInterfaceFleetMoves || []).some(
          (o) => o.in_transit && o.destination_dat_id === target), setup.target_dat_id);
      assert.ok(inTransitTo(setup)(observed));
      assert.ok((await observations(page)).every((o) => !o.confirmation_open), "the drag opened the window");
      return { observed };
    },
  },
];

async function inspect(server, faction, testCase, executable) {
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
    await page.waitForFunction(() => window.__openRebellionInterfaceFleetMoveSetup
      && (window.__openRebellionInterfaceFleetMoves || []).length > 0, null, { timeout: 10_000 });
    const setup = await page.evaluate(() => window.__openRebellionInterfaceFleetMoveSetup);
    assert.equal(setup.code, fixtureCode);
    const start = await latest(page);
    assert.deepEqual([start.in_transit, start.confirmation_open], [false, false], JSON.stringify(start));
    await page.evaluate(() => document.fonts.ready);
    const before = await shot(page, directory, "ready");

    const checks = await testCase.run(page, faction, setup, directory);
    await page.mouse.move(2, 2);
    await frames(page);
    const after = await shot(page, directory, "final");
    assert.deepEqual(requests.map(({ url }) => url).sort(), [...expectedRequests].sort());
    assert.ok(requests.every(({ status }) => status === 200), "startup has non-200 requests");
    assert.deepEqual(errors, [], `browser diagnostics: ${errors.join("; ")}`);
    result = {
      status: "pass",
      faction: faction.name,
      case: testCase.name,
      fixture_code: fixtureCode,
      ready,
      setup: { ...setup, systems: setup.systems.length },
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
  const executable = browserExecutable();
  const server = await startServer();
  let results;
  try {
    results = [];
    for (const faction of factions) {
      for (const testCase of cases) {
        if (only.length && !only.includes(`${faction.name}/${testCase.name}`)) continue;
        results.push(await inspect(server, faction, testCase, executable));
      }
    }
  } finally {
    await new Promise((resolve) => server.close(resolve));
  }
  const passed = results.every((result) => result.status === "pass");
  const summary = {
    schema_version: 1,
    family: "fleet-move",
    scope: "test-only fleet Move and Confirmed Move through the original entries: the pop-up menu and targeting (0x201, 0x202 with FUN_0044f060's window), and the Fleet window drag (0x201), on both sides",
    status: passed ? "pass" : "fail",
    browser_version: browserManifest.version,
    browser_executable: executable,
    launch_arguments: browserManifest.launch_arguments,
    profile: "one-new-muted-process-per-faction-and-case",
    muted: browserManifest.launch_arguments.includes("--mute-audio"),
    viewport: { width: 640, height: 480, device_scale_factor: 1 },
    confirmation: { ...confirmation, origins: Object.fromEntries(factions.map((f) => [f.name, confirmationOrigin(f)])) },
    results,
    wasm_sha256: sha256(fs.readFileSync(path.join(site, "open-rebellion-test.wasm"))),
    runtime_pack_sha256: sha256(fs.readFileSync(path.join(site, "data/runtime.orpk"))),
  };
  fs.writeFileSync(path.join(runDir, "result.json"), `${JSON.stringify(summary, null, 2)}\n`);
  if (!passed) {
    throw new Error(JSON.stringify(results.filter((result) => result.status !== "pass")
      .map(({ faction, case: name, error, observations: list }) => ({ faction, case: name, error, observations: list })), null, 2));
  }
  process.stdout.write(`${JSON.stringify({ run_dir: runDir, status: summary.status, cases: results.map((r) => `${r.faction}/${r.case}`), wasm_sha256: summary.wasm_sha256 }, null, 2)}\n`);
}

await main();
