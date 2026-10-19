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
const expectedRequests = ["/", "/data/runtime.orpk", "/gl.js", "/open-rebellion-test.wasm"];
const origin = { x: 85, y: 55 };
const dimensions = { width: 470, height: 331 };
const controls = [
  { command: 0x79, x: 0, width: 36, shared: [10830, 10831] },
  { command: 0x7a, x: 38, width: 36, alliance: [10832, 10833], empire: [10834, 10835] },
  { command: 0x7b, x: 76, width: 36, alliance: [10840, 10841], empire: [10838, 10839] },
  { command: 0x7c, x: 114, width: 35, alliance: [10842, 10843], empire: [10844, 10845] },
  { command: 0x7d, x: 151, width: 36, shared: [10836, 10837] },
  { command: 0x7e, x: 189, width: 36, shared: [10852, 10853] },
  { command: 0x7f, x: 227, width: 34, shared: [10856, 10857] },
  { command: 0x80, x: 263, width: 36, shared: [10854, 10855] },
  { command: 0x81, x: 301, width: 35, shared: [10850, 10851] },
  { command: 0x82, x: 338, width: 37, alliance: [10846, 10847], empire: [10848, 10849] },
];
const factions = [
  { name: "alliance", byte: 1, base: 10335, rail: 10820 },
  { name: "empire", byte: 2, base: 10336, rail: 10821 },
];
const runId = `${new Date().toISOString().replace(/[:.]/g, "-")}-${process.pid}`;
const runDir = path.join(root, ".artifacts/interface-parity", `message-index-shell-${runId}`);

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

// `keyed` leaves transparent every pixel whose palette index is the first
// stored pixel's, as FUN_005fd0f0 blits (the key byte FUN_005fd170 records).
function decodeIndexedBmp(bytes, expectedWidth, expectedHeight, keyed = false) {
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
  const key = bytes[dataOffset];
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
      png.data[destination + 3] = keyed && paletteIndex === key ? 0 : 255;
    }
  }
  return png;
}

function resource(source, id, width, height, keyed = false) {
  return decodeIndexedBmp(fs.readFileSync(path.join(source, `${id}.bmp`)), width, height, keyed);
}

// Copies the source's opaque pixels; a keyed source's transparent ones
// leave the destination as it was.
function blit(destination, source, x, y) {
  for (let row = 0; row < source.height; row += 1) {
    for (let column = 0; column < source.width; column += 1) {
      const from = (row * source.width + column) * 4;
      if (source.data[from + 3] === 0) continue;
      destination.data.set(source.data.subarray(from, from + 4), ((y + row) * destination.width + x + column) * 4);
    }
  }
}

function resourcesFor(control, faction) {
  return control.shared || control[faction.name];
}

function sourceIdentity(source) {
  const resourceIds = new Set([10822]);
  for (const faction of factions) {
    resourceIds.add(faction.base);
    resourceIds.add(faction.rail);
    for (const control of controls) {
      for (const resourceId of resourcesFor(control, faction)) resourceIds.add(resourceId);
    }
  }
  const sortedIds = [...resourceIds].sort((left, right) => left - right);
  const aggregate = createHash("sha256");
  for (const resourceId of sortedIds) {
    aggregate.update(`${resourceId}\0`);
    aggregate.update(fs.readFileSync(path.join(source, `${resourceId}.bmp`)));
  }
  return {
    dll: "STRATEGY.DLL",
    resource_count: sortedIds.length,
    resource_ids: sortedIds,
    aggregate_sha256: aggregate.digest("hex"),
  };
}

// FUN_004665f0.c:97-105: a zero-filled surface (FUN_005fbda0; index 0 is
// black in the shell's palette), 10822 plain at (12, 13) (FUN_005fcc30),
// then the shell and the rail keyed over it (FUN_005fd0f0). The shell's
// bottom row is its key, so the window's last row shows the black; the
// GDI probe of the Operation Report, which keys the same shell, shows that
// row black across the window.
function composeExpected(source, faction, pressedCommand = null) {
  const expected = new PNG({ width: dimensions.width, height: dimensions.height });
  for (let offset = 0; offset < expected.data.length; offset += 4) expected.data.set([0, 0, 0, 255], offset);
  blit(expected, resource(source, 10822, 400, 306), 12, 13);
  blit(expected, resource(source, faction.base, dimensions.width, dimensions.height, true), 0, 0);
  blit(expected, resource(source, faction.rail, 58, 330, true), 412, 0);
  for (const control of controls) {
    const [normal, pressed] = resourcesFor(control, faction);
    const selected = control.command === pressedCommand ? pressed : normal;
    blit(expected, resource(source, selected, control.width, 41), 22 + control.x, 46);
  }
  return expected;
}

function cropShell(screenshotBytes) {
  const screenshot = PNG.sync.read(screenshotBytes);
  assert.deepEqual([screenshot.width, screenshot.height], [640, 480]);
  const shell = new PNG({ width: dimensions.width, height: dimensions.height });
  for (let y = 0; y < dimensions.height; y += 1) {
    const start = ((origin.y + y) * screenshot.width + origin.x) * 4;
    const end = start + dimensions.width * 4;
    shell.data.set(screenshot.data.subarray(start, end), y * dimensions.width * 4);
  }
  return shell;
}

function compare(actual, expected, label, directory) {
  assert.deepEqual([actual.width, actual.height], [expected.width, expected.height]);
  let differentPixels = 0;
  const diff = new PNG({ width: expected.width, height: expected.height });
  for (let offset = 0; offset < expected.data.length; offset += 4) {
    const matches = actual.data[offset] === expected.data[offset]
      && actual.data[offset + 1] === expected.data[offset + 1]
      && actual.data[offset + 2] === expected.data[offset + 2];
    if (!matches) differentPixels += 1;
    diff.data.set(matches ? [0, 0, 0, 255] : [255, 0, 80, 255], offset);
  }
  fs.writeFileSync(path.join(directory, `${label}-actual.png`), PNG.sync.write(actual));
  fs.writeFileSync(path.join(directory, `${label}-expected.png`), PNG.sync.write(expected));
  fs.writeFileSync(path.join(directory, `${label}-diff.png`), PNG.sync.write(diff));
  return { label, pixels_checked: expected.width * expected.height, different_pixels: differentPixels };
}

async function stableShellScreenshot(page, directory, label) {
  // The first draw requests any newly selected BMP from the WASM texture
  // cache. Allow that upload to reach the following paint before establishing
  // the two-frame stability hash.
  await page.waitForTimeout(100);
  await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  const first = await page.screenshot({ animations: "disabled" });
  await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  const second = await page.screenshot({ animations: "disabled" });
  const firstShell = cropShell(first);
  const secondShell = cropShell(second);
  fs.writeFileSync(path.join(directory, `${label}-frame-1.png`), PNG.sync.write(firstShell));
  fs.writeFileSync(path.join(directory, `${label}-frame-2.png`), PNG.sync.write(secondShell));
  assert.equal(
    sha256(PNG.sync.write(firstShell)),
    sha256(PNG.sync.write(secondShell)),
    "Message Index shell did not stabilize",
  );
  return secondShell;
}

async function inspectFaction(server, source, faction, executable) {
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

    const fixtureCode = 40 | (faction.byte << 8);
    await page.goto(`${serverOrigin}/?fixture-code=${fixtureCode}`, {
      waitUntil: "load",
      timeout: 30_000,
    });
    await page.waitForFunction(() => window.__openRebellionInterfaceReady?.status, null, {
      timeout: 30_000,
    });
    const ready = await page.evaluate(() => window.__openRebellionInterfaceReady);
    assert.equal(ready.status, "ready", JSON.stringify(ready));
    assert.equal(ready.code, fixtureCode);
    await page.evaluate(() => document.fonts.ready);

    const comparisons = [];
    const normal = await stableShellScreenshot(page, directory, "normal");
    comparisons.push(compare(normal, composeExpected(source, faction), "normal", directory));
    for (const control of controls) {
      const x = origin.x + 22 + control.x + Math.floor(control.width / 2);
      const y = origin.y + 46 + 20;
      await page.mouse.move(x, y);
      await page.mouse.down();
      const label = `pressed-${control.command.toString(16)}`;
      const held = await stableShellScreenshot(page, directory, label);
      comparisons.push(compare(
        held,
        composeExpected(source, faction, control.command),
        label,
        directory,
      ));
      await page.mouse.up();
      await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))));
    }
    await page.mouse.move(origin.x + 22 + controls[0].width, origin.y + 46 + 20);
    await page.mouse.down();
    const outside = await stableShellScreenshot(page, directory, "outside-right-edge");
    comparisons.push(compare(outside, composeExpected(source, faction), "outside-right-edge", directory));
    await page.mouse.up();
    await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))));

    assert.ok(comparisons.every(({ different_pixels: differentPixels }) => differentPixels === 0),
      JSON.stringify(comparisons.filter(({ different_pixels: differentPixels }) => differentPixels !== 0)));
    assert.deepEqual(requests.map(({ url }) => url).sort(), [...expectedRequests].sort());
    assert.ok(requests.every(({ status }) => status === 200), "startup has non-200 requests");
    assert.deepEqual(errors, [], `browser diagnostics: ${errors.join("; ")}`);
    result = {
      status: "pass",
      faction: faction.name,
      fixture_code: fixtureCode,
      ready,
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
    for (const faction of factions) results.push(await inspectFaction(server, source, faction, executable));
  } finally {
    await new Promise((resolve) => server.close(resolve));
  }
  const passed = results.every((result) => result.status === "pass");
  const summary = {
    schema_version: 1,
    family: "message-index-shell",
    scope: "test-only source-exact Message Index shell and category-control states; populated rows and production routing remain open",
    status: passed ? "pass" : "fail",
    browser_version: browserManifest.version,
    browser_executable: executable,
    launch_arguments: browserManifest.launch_arguments,
    profile: "one-new-muted-process-per-faction",
    muted: browserManifest.launch_arguments.includes("--mute-audio"),
    viewport: { width: 640, height: 480, device_scale_factor: 1 },
    shell: dimensions,
    source: sourceIdentity(source),
    factions: results,
    wasm_sha256: sha256(fs.readFileSync(path.join(site, "open-rebellion-test.wasm"))),
    runtime_pack_sha256: sha256(fs.readFileSync(path.join(site, "data/runtime.orpk"))),
  };
  fs.writeFileSync(path.join(runDir, "result.json"), `${JSON.stringify(summary, null, 2)}\n`);
  if (!passed) throw new Error(JSON.stringify(results.filter((result) => result.status !== "pass"), null, 2));
  process.stdout.write(`${JSON.stringify({ run_dir: runDir, ...summary }, null, 2)}\n`);
}

await main();
