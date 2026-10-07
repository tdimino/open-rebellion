#!/usr/bin/env node

// The 2026-10-06 audit batch through its original entries, on the Fleet
// Finder fixture's campaign: F4 and F5 open the Troop Finder (0x130) and
// the Personnel Finder (0x12f) with their STRATEGY.DLL rails; F6 (0x75) and
// a rail light (0x136) open the Message Index, which opens only while none
// is open (FUN_0042a240) and closes from its Close button (0x28); Alt+1..9
// select a GID mode (0xbca..0xbd2); Alt+G toggles Manage Garrisons
// (0xbc5 -> 0x115); and a right click on the agent's droid opens the Agent
// menu (list 0xdead, FUN_004420b0). The viewport is the cockpit's own
// 640x480, so every point below is a logical cockpit point.

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
// Fixture codes are the Scenario index plus one (interface_test_fixture.rs).
const FLEET_FINDER = 53;
// The rails at (412, 0) by side: Troop Finder 10584/10588 (FUN_0046ce40),
// Personnel Finder 10586/10590 (FUN_00463500).
const factions = [
  { name: "alliance", byte: 1, troopRail: 10584, personnelRail: 10586,
    // Rail light 0x136 at (3, 109), the droid aperture (541, 337, 67, 116),
    // Close at (423, 25) 32x31: their centers.
    railLight: { x: 16, y: 120 }, droid: { x: 574, y: 395 }, close: { x: 439, y: 40 } },
  { name: "empire", byte: 2, troopRail: 10588, personnelRail: 10590,
    // Rail light 0x136 at (611, 110), the droid (0, 347, 107, 133), Close at
    // (426, 21) 44x41.
    railLight: { x: 624, y: 121 }, droid: { x: 53, y: 413 }, close: { x: 448, y: 41 } },
];
const RAIL = { x: 412, y: 0, width: 58, height: 330 };
// Buttons the windows draw over their rails, in rail coordinates: Close and
// Display (FUN_0046ce40, FUN_004637a0), then the Personnel Finder's two
// view buttons below them.
const RAIL_BUTTONS = {
  // The Alliance art draws a few pixels past its 32-pixel control.
  alliance: [[7, 25, 40, 31], [7, 93, 40, 31], [7, 147, 40, 31], [7, 201, 40, 31]],
  empire: [[14, 21, 44, 41], [14, 89, 44, 41], [14, 143, 44, 41], [14, 197, 44, 41]],
};
const runId = `${new Date().toISOString().replace(/[:.]/g, "-")}-${process.pid}`;
const runDir = path.join(root, ".artifacts/interface-parity", `audit-batch-${runId}`);

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

function crop(bytes, rect) {
  const screenshot = PNG.sync.read(bytes);
  assert.deepEqual([screenshot.width, screenshot.height], [640, 480]);
  const out = new PNG({ width: rect.width, height: rect.height });
  for (let y = 0; y < rect.height; y += 1) {
    const start = ((rect.y + y) * screenshot.width + rect.x) * 4;
    out.data.set(screenshot.data.subarray(start, start + rect.width * 4), y * rect.width * 4);
  }
  return out;
}

// The rail bitmap against the screen at the window's rail rectangle.
function compareRail(bytes, origin, image, label, directory, buttons) {
  const rect = { x: origin.x + RAIL.x, y: origin.y + RAIL.y, width: RAIL.width, height: RAIL.height };
  const actual = crop(bytes, rect);
  let different = 0;
  let checked = 0;
  for (let y = 0; y < Math.min(image.height, rect.height); y += 1) {
    for (let x = 0; x < Math.min(image.width, rect.width); x += 1) {
      if (buttons.some(([bx, by, bw, bh]) => x >= bx && x < bx + bw && y >= by && y < by + bh)) continue;
      const a = (y * rect.width + x) * 4;
      const e = (y * image.width + x) * 4;
      checked += 1;
      if (actual.data[a] !== image.data[e] || actual.data[a + 1] !== image.data[e + 1]
        || actual.data[a + 2] !== image.data[e + 2]) different += 1;
    }
  }
  fs.writeFileSync(path.join(directory, `${label}-rail.png`), PNG.sync.write(actual));
  return { label, pixels_checked: checked, different_pixels: different };
}

async function frames(page, count = 1) {
  for (let index = 0; index < count; index += 1) {
    await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  }
}

async function click(page, point, button = "left") {
  await page.mouse.move(point.x, point.y);
  await frames(page);
  await page.mouse.down({ button });
  await frames(page);
  await page.mouse.up({ button });
  await frames(page);
}

async function key(page, name) {
  await page.keyboard.down(name);
  await frames(page);
  await page.keyboard.up(name);
  await frames(page);
}

async function altKey(page, name) {
  await page.keyboard.down("Alt");
  await frames(page);
  await key(page, name);
  await page.keyboard.up("Alt");
  await frames(page);
}

async function stableShot(page, directory, label) {
  await page.mouse.move(2, 2);
  await page.waitForTimeout(150);
  await frames(page, 2);
  const bytes = await page.screenshot({ animations: "disabled" });
  fs.writeFileSync(path.join(directory, `${label}.png`), bytes);
  return bytes;
}

function finderObservation(page) {
  return page.evaluate(() => (window.__openRebellionInterfaceFleetFinders || []).at(-1));
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
  // Wait for the `count`th console line matching `pattern`.
  const logged = async (description, pattern, count = 1, timeout = 10_000) => {
    const started = Date.now();
    while (Date.now() - started < timeout) {
      const lines = consoleLines.filter((line) => pattern.test(line.text));
      if (lines.length >= count) return lines[count - 1].text;
      await page.waitForTimeout(100);
    }
    throw new Error(`${description}: ${pattern} x${count} not logged; last lines ${JSON.stringify(consoleLines.slice(-6).map((line) => line.text))}`);
  };
  const countOf = (pattern) => consoleLines.filter((line) => pattern.test(line.text)).length;
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
    await page.waitForFunction(() => (window.__openRebellionInterfaceFleetFinders || []).length > 0,
      null, { timeout: 10_000 });
    await page.evaluate(() => document.fonts.ready);
    const checks = {};

    // The Finders share the Fleet Finder's 470 by 330 centered rectangle:
    // F3 reports its origin.
    await key(page, "F3");
    await page.waitForFunction(() => (window.__openRebellionInterfaceFleetFinders || []).at(-1)?.open,
      null, { timeout: 10_000 });
    const fleetFinder = await finderObservation(page);
    const origin = { x: Math.round(fleetFinder.controls.origin[0]), y: Math.round(fleetFinder.controls.origin[1]) };
    await key(page, "Escape");
    await page.waitForFunction(() => !(window.__openRebellionInterfaceFleetFinders || []).at(-1)?.open,
      null, { timeout: 10_000 });
    const empty = await stableShot(page, directory, "galaxy");

    // F4: FUN_00422ce0 case 0x73 -> FUN_0042a4d0, the Troop Finder.
    await key(page, "F4");
    checks.troop_log = await logged("F4 opens the Troop Finder", /command=0x130 destination=troop_finder status=opened_original/);
    const troop = await stableShot(page, directory, "troop-finder");
    checks.troop_rail = compareRail(troop, origin, resource(source, faction.troopRail), "troop", directory,
      RAIL_BUTTONS[faction.name].slice(0, 2));
    assert.equal(checks.troop_rail.different_pixels, 0, JSON.stringify(checks.troop_rail));
    await key(page, "Escape");
    const troopClosed = await stableShot(page, directory, "troop-closed");
    assert.equal(sha256(crop(troopClosed, { ...RAIL, x: origin.x + RAIL.x, y: origin.y }).data),
      sha256(crop(empty, { ...RAIL, x: origin.x + RAIL.x, y: origin.y }).data),
      "Escape closes the Troop Finder");

    // F5: case 0x74 -> FUN_0042a180, the Personnel Finder.
    await key(page, "F5");
    checks.personnel_log = await logged("F5 opens the Personnel Finder", /command=0x12f destination=personnel_finder status=opened_original/);
    const personnel = await stableShot(page, directory, "personnel-finder");
    checks.personnel_rail = compareRail(personnel, origin, resource(source, faction.personnelRail), "personnel", directory,
      RAIL_BUTTONS[faction.name]);
    assert.equal(checks.personnel_rail.different_pixels, 0, JSON.stringify(checks.personnel_rail));
    await key(page, "Escape");
    await stableShot(page, directory, "personnel-closed");

    // Alt+2 then Alt+1: 0xbcb/0xbca select GID modes 0x12 and 0x11; the
    // current mode returns early, so at least one selects.
    await altKey(page, "2");
    await altKey(page, "1");
    checks.gid_log = await logged("Alt+digit selects a GID mode", /destination=gid status=selected .* source=accelerator/);

    // Alt+G: 0xbc5 -> 0x115 toggles Manage Garrisons on, then off.
    await altKey(page, "g");
    checks.garrisons_on = await logged("Alt+G turns Manage Garrisons on",
      /\[agent\] command=manage_garrisons status=on source=accelerator/);
    await altKey(page, "g");
    checks.garrisons_off = await logged("Alt+G turns it off again",
      /\[agent\] command=manage_garrisons status=off source=accelerator/);

    // F6: case 0x75 -> FUN_0042a240(.., 0x79), All. A second F6 while it
    // is open does nothing; Close (0x28) closes it, and F6 opens it again.
    const opens = /destination=message_index category=0x79 status=opened_original/;
    await key(page, "F6");
    checks.index_log = await logged("F6 opens the Message Index on All", opens);
    await stableShot(page, directory, "message-index");
    await key(page, "F6");
    await page.waitForTimeout(300);
    assert.equal(countOf(opens), 1, "F6 while the Message Index is open opens nothing");
    await click(page, { x: origin.x + faction.close.x, y: origin.y + faction.close.y });
    await stableShot(page, directory, "message-index-closed");
    await key(page, "F6");
    await logged("Close closed the Message Index, so F6 opens it again", opens, 2);
    await key(page, "Escape");
    // A rail light: 0x136 opens Popular Support (0x7a).
    await click(page, faction.railLight);
    checks.rail_log = await logged("the rail light opens its category",
      /command=0x136 destination=message_index category=0x7a status=opened_original/);
    await stableShot(page, directory, "message-index-rail");
    await key(page, "Escape");

    // A right click on the agent's droid opens the Agent menu.
    const beforeMenu = await stableShot(page, directory, "before-agent-menu");
    await click(page, faction.droid, "right");
    const menu = await stableShot(page, directory, "agent-menu");
    assert.notEqual(sha256(menu), sha256(beforeMenu), "the right click drew the Agent menu");
    await key(page, "Escape");
    const afterMenu = await stableShot(page, directory, "agent-menu-closed");
    checks.agent_menu = { opened_sha256: sha256(menu), closed_sha256: sha256(afterMenu) };

    assert.deepEqual(requests.map(({ url }) => url).sort(), [...expectedRequests].sort());
    assert.ok(requests.every(({ status }) => status === 200), "startup has non-200 requests");
    assert.deepEqual(errors, [], `browser diagnostics: ${errors.join("; ")}`);
    result = {
      status: "pass",
      faction: faction.name,
      fixture_code: fixtureCode,
      origin,
      checks,
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
    if (page) await page.screenshot().then((bytes) => fs.writeFileSync(path.join(directory, "failure.png"), bytes)).catch(() => {});
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
  const source = sourceDirectory();
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
  const summary = {
    schema_version: 1,
    family: "audit-batch",
    scope: "test-only: F4 Troop Finder and F5 Personnel Finder rails against STRATEGY.DLL and Escape; F6 and a rail light open the Message Index once, Close closes it; Alt+digit GID; Alt+G Manage Garrisons on and off; right click on the agent droid opens the Agent menu; on both sides",
    status: passed ? "pass" : "fail",
    browser_version: browserManifest.version,
    browser_executable: executable,
    launch_arguments: browserManifest.launch_arguments,
    muted: browserManifest.launch_arguments.includes("--mute-audio"),
    viewport: { width: 640, height: 480, device_scale_factor: 1 },
    source_resources: [...used].sort((left, right) => left - right),
    results,
    wasm_sha256: sha256(fs.readFileSync(path.join(site, "open-rebellion-test.wasm"))),
    runtime_pack_sha256: sha256(fs.readFileSync(path.join(site, "data/runtime.orpk"))),
  };
  fs.writeFileSync(path.join(runDir, "result.json"), `${JSON.stringify(summary, null, 2)}\n`);
  if (!passed) {
    throw new Error(JSON.stringify(results.filter((result) => result.status !== "pass")
      .map(({ faction, error, console: lines }) => ({ faction, error, console: (lines || []).slice(-8) })), null, 2));
  }
  process.stdout.write(`${JSON.stringify({ run_dir: runDir, status: summary.status, wasm_sha256: summary.wasm_sha256 }, null, 2)}\n`);
}

await main();
