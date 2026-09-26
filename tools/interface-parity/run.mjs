#!/usr/bin/env node

import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import fs from "node:fs";
import http from "node:http";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { execFileSync, spawnSync } from "node:child_process";
import pixelmatch from "pixelmatch";
import { chromium } from "playwright-core";
import { PNG } from "pngjs";
import { performCommand } from "./command-events.mjs";
import { launchBrowser, summarizeBrowserAttempts } from "./browser-launch.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const site = path.join(root, ".artifacts/interface-parity/site");
const browserManifest = JSON.parse(fs.readFileSync(path.join(here, "browser.json"), "utf8"));
const schemaVersion = 1;
const options = new Set(process.argv.slice(2));
const battle = options.has("--battle");
const catalog = JSON.parse(fs.readFileSync(path.join(
  here, battle ? "scenarios/tactical.catalog.json" : "scenarios/gid.catalog.json",
), "utf8"));
const all = options.has("--all");
const smoke = options.has("--smoke") || !all;
const scenarioFilter = process.argv.find((argument) => argument.startsWith("--scenario="))?.slice(11);
const updateGoldens = options.has("--update-goldens");
const noBuild = options.has("--no-build");
const runId = `${new Date().toISOString().replace(/[:.]/g, "-")}-${process.pid}`;
const runDir = path.join(root, ".artifacts/interface-parity", runId);
const baselineDir = path.join(here, "baselines/implementation/chrome-151-mac-arm64");
const originalDir = path.join(root, "docs/qa/2026-09-10-interface-parity-audit/reference-captures");
const acceptedOriginal = JSON.parse(fs.readFileSync(path.join(here, "baselines/accepted-original.json"), "utf8"));

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

function fail(message) {
  throw new Error(message);
}

function mimeType(file) {
  if (file.endsWith(".html")) return "text/html; charset=utf-8";
  if (file.endsWith(".js")) return "text/javascript; charset=utf-8";
  if (file.endsWith(".wasm")) return "application/wasm";
  if (file.endsWith(".orpk")) return "application/octet-stream";
  return "application/octet-stream";
}

async function startServer() {
  const server = http.createServer((request, response) => {
    const pathname = new URL(request.url, "http://localhost").pathname;
    const relative = path.posix.normalize(decodeURIComponent(pathname))
      .replace(/^\/+/, "") || "index.html";
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

function browserExecutable() {
  const candidates = [
    ...(process.env.OPEN_REBELLION_CHROME_FOR_TESTING
      ? [process.env.OPEN_REBELLION_CHROME_FOR_TESTING]
      : []),
    ...browserManifest.executable_candidates,
  ];
  const executable = candidates.find((candidate) => fs.existsSync(candidate));
  if (!executable) {
    fail(`pinned Chrome for Testing ${browserManifest.version} is missing; see README.md`);
  }
  const result = spawnSync(executable, ["--version"], { encoding: "utf8" });
  if (result.status !== 0 || !result.stdout.includes(browserManifest.version)) {
    fail(`Chrome for Testing version mismatch: ${result.stdout || result.stderr}`);
  }
  return executable;
}

function scenarioId(scenario, faction, viewport) {
  return `${battle ? "tactical" : "gid"}/${faction}/${scenario.slug}/${viewport.id}`;
}

function writeContactSheet(results) {
  const cards = results.map(({ id, status }) => {
    const safe = encodeURI(id);
    return `<figure><a href="${safe}/actual.png"><img src="${safe}/actual.png" alt="${id}"></a><figcaption>${status}: ${id}</figcaption></figure>`;
  }).join("\n");
  fs.writeFileSync(path.join(runDir, "contact-sheet.html"), `<!doctype html>
<html lang="en"><meta charset="utf-8"><title>${catalog.family} interface contact sheet</title>
<style>body{background:#141822;color:#e7ebf5;font:14px system-ui;margin:16px}
main{display:grid;grid-template-columns:repeat(auto-fit,minmax(310px,1fr));gap:14px}
figure{margin:0;background:#202638;padding:8px}img{width:100%;height:240px;object-fit:contain}
figcaption{overflow-wrap:anywhere;margin-top:7px}</style>
<h1>${catalog.family} interface contact sheet</h1><p>Previews link to original lossless PNGs.
Passing harness probes are not original-game parity acceptance.</p><main>${cards}</main></html>\n`);
}

function fixtureCode(scenario, faction) {
  if (battle) return scenario.fixture_codes[faction];
  return (scenario.index + 1) | ((faction === "alliance" ? 1 : 2) << 8);
}

async function measureBrowserRuntime(page) {
  return page.evaluate(async () => {
    const intervals = [];
    await new Promise((resolve) => {
      let previous = null;
      function sample(now) {
        if (previous !== null) intervals.push(now - previous);
        previous = now;
        if (intervals.length >= 60) resolve();
        else requestAnimationFrame(sample);
      }
      requestAnimationFrame(sample);
    });
    const sorted = [...intervals].sort((a, b) => a - b);
    const percentile = (ratio) => sorted[Math.min(
      sorted.length - 1,
      Math.ceil(sorted.length * ratio) - 1,
    )];
    const navigation = performance.getEntriesByType("navigation")[0];
    const wasmMemoryBytes = typeof wasm_memory !== "undefined"
      ? wasm_memory.buffer.byteLength
      : null;
    const jsHeap = performance.memory || null;
    return {
      sample: "60 paused requestAnimationFrame intervals after cold fixture load",
      frame_interval_ms: {
        median: percentile(0.5),
        p95: percentile(0.95),
        max: sorted[sorted.length - 1],
      },
      navigation_duration_ms: navigation?.duration ?? null,
      wasm_memory_bytes: wasmMemoryBytes,
      js_heap_used_bytes: jsHeap?.usedJSHeapSize ?? null,
      js_heap_total_bytes: jsHeap?.totalJSHeapSize ?? null,
    };
  });
}

function compareScreenshot(id, bytes, folder, capture = "actual") {
  const baselineId = capture === "actual" ? id : `${id}/${capture}`;
  const baseline = path.join(baselineDir, `${baselineId}.png`);
  const comparison = {
    original_reference_source: "owned-original-or-curated-reference-only",
    original_reference_root: path.relative(root, originalDir),
    implementation_baseline: fs.existsSync(baseline)
      ? path.relative(root, baseline)
      : null,
    status: "unbaselined",
    different_pixels: null,
    difference_ratio: null,
    tolerance: 0,
    masks: [],
  };
  if (!fs.existsSync(baseline)) {
    if (updateGoldens) {
      const authority = acceptedOriginal.accepted[baselineId];
      if (!authority || authority.disposition !== "accepted" || !/^[a-f0-9]{64}$/.test(authority.sha256)) {
        fail(`cannot create implementation golden for ${baselineId} without a reviewed original reference`);
      }
      const original = path.resolve(root, authority.path);
      if (!original.startsWith(`${originalDir}${path.sep}`) || !fs.existsSync(original)
        || sha256(fs.readFileSync(original)) !== authority.sha256) {
        fail(`original-reference integrity check failed for ${baselineId}`);
      }
      fs.mkdirSync(path.dirname(baseline), { recursive: true });
      fs.copyFileSync(path.join(folder, `${capture}.png`), baseline);
      comparison.implementation_baseline = path.relative(root, baseline);
      comparison.status = "new_regression_baseline_from_accepted_reference";
    }
    return comparison;
  }

  const actual = PNG.sync.read(bytes);
  const expectedBytes = fs.readFileSync(baseline);
  const expected = PNG.sync.read(expectedBytes);
  fs.writeFileSync(path.join(folder, `expected-${capture}.png`), expectedBytes);
  if (actual.width !== expected.width || actual.height !== expected.height) {
    return { ...comparison, status: "dimension_mismatch" };
  }
  const diff = new PNG({ width: actual.width, height: actual.height });
  const differentPixels = pixelmatch(
    actual.data,
    expected.data,
    diff.data,
    actual.width,
    actual.height,
    { threshold: 0, includeAA: true, diffColor: [255, 0, 80] },
  );
  fs.writeFileSync(path.join(folder, `diff-${capture}.png`), PNG.sync.write(diff));
  comparison.different_pixels = differentPixels;
  comparison.difference_ratio = differentPixels / (actual.width * actual.height);
  comparison.status = differentPixels === 0 ? "match" : "mismatch";
  if (updateGoldens && differentPixels) {
    comparison.status = "mismatch_not_updated";
  }
  return comparison;
}

async function stableFrame(page, folder) {
  const first = await page.screenshot({ path: path.join(folder, "frame-a.png"), animations: "disabled" });
  await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  const second = await page.screenshot({ path: path.join(folder, "actual.png"), animations: "disabled" });
  return { first: sha256(first), second: sha256(second), bytes: second };
}

async function stableInteractionFrame(page, folder, name) {
  let previous = null;
  for (let attempt = 0; attempt < 8; attempt++) {
    await page.evaluate(() => new Promise((resolve) =>
      requestAnimationFrame(() => requestAnimationFrame(resolve))));
    const bytes = await page.screenshot({ animations: "disabled" });
    if (previous && sha256(previous) === sha256(bytes)) {
      fs.writeFileSync(path.join(folder, `${name}.png`), bytes);
      return bytes;
    }
    previous = bytes;
  }
  throw new Error(`tactical interaction capture did not stabilize: ${name}`);
}

async function dynamicInteractionFrame(page, folder, name) {
  await page.evaluate(() => new Promise((resolve) =>
    requestAnimationFrame(() => requestAnimationFrame(resolve))));
  return page.screenshot({
    path: path.join(folder, `${name}.png`),
    animations: "disabled",
  });
}

function decodeIndexedBmp(bytes) {
  assert.equal(bytes.toString("ascii", 0, 2), "BM", "source resource is not a BMP");
  const dataOffset = bytes.readUInt32LE(10);
  const headerSize = bytes.readUInt32LE(14);
  const width = bytes.readInt32LE(18);
  const height = bytes.readInt32LE(22);
  assert.equal(bytes.readUInt16LE(28), 8, "source resource is not indexed 8-bit BMP");
  assert.equal(bytes.readUInt32LE(30), 0, "source resource has unsupported compression");
  assert.ok(width > 0 && height > 0 && headerSize >= 40);
  const paletteOffset = 14 + headerSize;
  const stride = (width + 3) & ~3;
  assert.ok(bytes.length >= dataOffset + stride * height);
  return {
    width,
    height,
    pixel(x, y) {
      const index = bytes[dataOffset + (height - 1 - y) * stride + x];
      const entry = paletteOffset + index * 4;
      return [bytes[entry + 2], bytes[entry + 1], bytes[entry]];
    },
  };
}

function verifyGidRootFrame(viewport, screenshotBytes, systemWindowOpen = false) {
  if (viewport.width !== 640 || viewport.height !== 480 || viewport.device_scale_factor !== 1) {
    return { status: "non-native-scale", pixels_checked: 0 };
  }
  const screenshot = PNG.sync.read(screenshotBytes);
  const resources = new Map();
  for (let id = 10100; id <= 10107; id++) {
    resources.set(id, decodeIndexedBmp(fs.readFileSync(path.join(
      root, `data/base/ui/strategy-dll/BMP/${id}.bmp`,
    ))));
  }
  let pixelsChecked = 0;
  let pixelsOccluded = 0;
  if (systemWindowOpen) {
    const covered = (230 * screenshot.width + 425) * 4;
    assert.notDeepEqual(
      Array.from(screenshot.data.subarray(covered, covered + 3)),
      resources.get(10100).pixel(0, 0),
      "system fixture did not occlude the GID root at its known overlap",
    );
  }
  const check = (sx, sy, id, bx, by) => {
    // The detailed system window and its edge span x=226..455, y=75..379. It is in front
    // of the GID popup, so compare the authored frame everywhere else.
    if (systemWindowOpen && sx <= 455 && sy <= 379) {
      pixelsOccluded++;
      return;
    }
    const source = resources.get(id);
    const offset = (sy * screenshot.width + sx) * 4;
    assert.deepEqual(
      Array.from(screenshot.data.subarray(offset, offset + 3)),
      source.pixel(bx, by),
      `GID root border (${sx}, ${sy}) differs from STRATEGY ${id} (${bx}, ${by})`,
    );
    pixelsChecked++;
  };
  // The root rectangle is 158x159 at native 640x480, from the recovered
  // GID anchor and seven rows. Check every visible painted border pixel.
  for (const [id, x, y] of [
    [10100, 425, 230], [10101, 581, 230],
    [10102, 425, 387], [10103, 581, 387],
  ]) {
    for (let by = 0; by < 2; by++) {
      for (let bx = 0; bx < 2; bx++) check(x + bx, y + by, id, bx, by);
    }
  }
  for (let x = 427; x <= 580; x++) {
    check(x, 230, 10104, (x - 427) % 2, 0);
    check(x, 388, 10107, (x - 427) % 2, 0);
  }
  for (let y = 232; y <= 386; y++) {
    check(425, y, 10105, 0, (y - 232) % 2);
    check(582, y, 10106, 0, (y - 232) % 2);
  }
  return {
    status: systemWindowOpen ? "source-bitmap-visible-partial" : "source-bitmap-exact",
    source_bitmaps_checked: 8,
    pixels_checked: pixelsChecked,
    pixels_occluded: pixelsOccluded,
  };
}

function verifyMessageIndexRail(faction, viewport, screenshotBytes) {
  if (viewport.width !== 640 || viewport.height !== 480 || viewport.device_scale_factor !== 1) {
    return { status: "non-native-scale", source_bitmaps_checked: 0 };
  }
  const screenshot = PNG.sync.read(screenshotBytes);
  assert.equal(screenshot.width, 640);
  assert.equal(screenshot.height, 480);
  const x0 = faction === "alliance" ? 3 : 611;
  const y0 = faction === "alliance" ? 109 : 110;
  const firstResource = faction === "alliance" ? 10050 : 10030;
  for (let control = 0; control < 9; control++) {
    const resource = firstResource + control;
    const source = decodeIndexedBmp(fs.readFileSync(path.join(
      root, `data/base/ui/strategy-dll/BMP/${resource}.bmp`,
    )));
    assert.equal(source.width, 27);
    assert.equal(source.height, 22);
    for (let y = 0; y < source.height; y++) {
      for (let x = 0; x < source.width; x++) {
        const actualOffset = ((y0 + control * 25 + y) * screenshot.width + x0 + x) * 4;
        assert.deepEqual(
          Array.from(screenshot.data.subarray(actualOffset, actualOffset + 3)),
          source.pixel(x, y),
          `${faction} rail ${resource} pixel (${x}, ${y}) differs from its original BMP`,
        );
      }
    }
  }
  return { status: "source-bitmap-exact", source_bitmaps_checked: 9, pixels_checked: 9 * 27 * 22 };
}

async function probeGid(page, faction, scenario, viewport, folder, consoleLines, ready) {
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  // Command 0x132: rightmost bottom control in cockpit.rs for each faction.
  const control = faction === "alliance"
    ? { x: 446, y: 406, width: 27, height: 16 }
    : { x: 519, y: 434, width: 37, height: 25 };
  const point = (x, y) => ({ x: offsetX + x * scale, y: offsetY + y * scale });
  const interior = point(control.x + control.width / 2, control.y + control.height / 2);
  const outside = point(control.x - 2, control.y + control.height / 2);
  const probes = [];

  const commandLines = () => consoleLines.map(({ text }) => text)
    .filter((line) => line.includes("[interface] command="));
  await page.mouse.click(outside.x, outside.y);
  await page.waitForTimeout(70);
  assert.ok(!commandLines().some((line) => line.includes("destination=gid_menu status=opened")),
    `${faction}: strict outside-edge opened the GID menu`);
  probes.push({ type: "outside-control", x: outside.x, y: outside.y, opened: false });

  await page.mouse.move(interior.x, interior.y);
  await page.screenshot({ path: path.join(folder, "control-hover.png"), animations: "disabled" });
  await page.mouse.down();
  await page.evaluate(() => new Promise((resolve) => {
    requestAnimationFrame(() => requestAnimationFrame(resolve));
  }));
  await page.screenshot({ path: path.join(folder, "control-pressed.png"), animations: "disabled" });
  const opened = "command=0x132 destination=gid_menu status=opened_original";
  await performCommand(page, opened, () => page.mouse.up());
  const rootCapture = await page.screenshot({ path: path.join(folder, "menu-root.png"), animations: "disabled" });
  const rootFrame = verifyGidRootFrame(viewport, rootCapture, scenario.slug === "system");
  probes.push({ type: "control-click", x: interior.x, y: interior.y, opened: true,
    screenshot_sha256: sha256(rootCapture), root_frame: rootFrame });

  const closed = "command=0x132 destination=gid_menu status=closed";
  await performCommand(page, closed, () => page.mouse.click(interior.x, interior.y));
  const closedCapture = await page.screenshot({ path: path.join(folder, "control-closed.png"),
    animations: "disabled" });
  assert.equal(sha256(closedCapture), sha256(fs.readFileSync(path.join(folder, "actual.png"))),
    `${faction}: the GID control did not close its own menu cleanly`);
  probes.push({ type: "control-click-close", x: interior.x, y: interior.y, opened: false,
    screenshot_sha256: sha256(closedCapture) });
  await performCommand(page, opened, () => page.mouse.click(interior.x, interior.y));

  if (scenario.category !== null) {
    const alternate = scenario.mode === "Uprisings" ? "Popular Support" : "Uprisings";
    const alternatePoint = point(470, 246);
    await page.mouse.move(alternatePoint.x, alternatePoint.y);
    await page.waitForTimeout(50);
    const hoveredRoot = await page.screenshot({ path: path.join(folder, "menu-root-hover.png"), animations: "disabled" });
    if (viewport.width === 640 && viewport.height === 480 && viewport.device_scale_factor === 1
        && scenario.slug !== "system") {
      const before = PNG.sync.read(rootCapture);
      const after = PNG.sync.read(hoveredRoot);
      const pixel = (png, x, y) => Array.from(png.data.subarray((y * png.width + x) * 4, (y * png.width + x) * 4 + 3));
      assert.deepEqual(pixel(after, 450, 245), pixel(before, 450, 245),
        "GID root added an unproven synthetic row hover wash");
    }
    const alternateLeaf = point(222, 246 + (alternate === "Uprisings" ? 21 : 0));
    await performCommand(page, `destination=gid status=selected label=${alternate}`,
      () => page.mouse.click(alternateLeaf.x, alternateLeaf.y));
    await page.waitForFunction((expected) => window.__openRebellionInterfaceSelection?.mode === expected,
      alternate, { timeout: 2_000 });
    probes.push({ type: "alternate-mode", mode: alternate });

    await performCommand(page, opened, () => page.mouse.click(interior.x, interior.y));
    const rootPoint = point(470, 246 + 21 * scenario.category);
    await page.mouse.move(rootPoint.x, rootPoint.y);
    await page.waitForTimeout(80);
    const submenuCapture = await page.screenshot({ path: path.join(folder, "menu-submenu.png"), animations: "disabled" });
    probes.push({ type: "category-hover", category: scenario.category,
      screenshot_sha256: sha256(submenuCapture) });
    assert.notEqual(sha256(rootCapture), sha256(submenuCapture),
      `${scenario.slug}: submenu did not change the root view`);
    const submenuTop = Math.min(230 + 21 * scenario.category,
      470 - 21 * [2, 2, 2, 4, 6, 5][scenario.category]);
    const leafPoint = point(222, submenuTop + 16 + 21 * scenario.leaf);
    const expected = `destination=gid status=selected label=${scenario.mode}`;
    await performCommand(page, expected, () => page.mouse.click(leafPoint.x, leafPoint.y));
    await page.waitForFunction((mode) => window.__openRebellionInterfaceSelection?.mode === mode,
      scenario.mode, { timeout: 2_000 });
    const selected = await page.evaluate(() => window.__openRebellionInterfaceSelection);
    assert.equal(selected.code, fixtureCode(scenario, faction));
    const selectedCapture = await page.screenshot({ path: path.join(folder, "menu-selected.png"), animations: "disabled" });
    probes.push({ type: "submenu-selection", mode: scenario.mode,
      screenshot_sha256: sha256(selectedCapture) });
  } else if (scenario.slug === "display-off") {
    const displayOff = point(477, 372);
    await performCommand(page, "destination=gid status=selected label=Display Off",
      () => page.mouse.click(displayOff.x, displayOff.y));
    await page.waitForFunction(() => window.__openRebellionInterfaceSelection?.mode === "Display Off",
      null, { timeout: 2_000 });
    probes.push({ type: "root-selection", mode: "Display Off" });
  } else {
    await page.keyboard.press("Escape");
    await page.waitForTimeout(80);
    const dismissed = await page.screenshot({ path: path.join(folder, "menu-dismissed.png"), animations: "disabled" });
    assert.equal(sha256(dismissed), sha256(fs.readFileSync(path.join(folder, "actual.png"))),
      `${scenario.slug}: Escape left the original galaxy surface`);
    probes.push({ type: "menu-escape", screenshot_sha256: sha256(dismissed) });
  }

  if (scenario.slug === "popular-support") {
    const commandControls = faction === "alliance"
      ? [
          { slug: "game-options", rect: { x: 3, y: 355, width: 27, height: 41 },
            expected: "command=0x133 destination=game_options status=opened_original" },
          { slug: "encyclopedia", rect: { x: 394, y: 405, width: 27, height: 16 },
            expected: "command=0x131 destination=encyclopedia status=pending_original_window" },
        ]
      : [
          { slug: "game-options", rect: { x: 79, y: 192, width: 35, height: 57 },
            expected: "command=0x133 destination=game_options status=opened_original" },
          { slug: "encyclopedia", rect: { x: 465, y: 434, width: 35, height: 24 },
            expected: "command=0x131 destination=encyclopedia status=pending_original_window" },
        ];
    for (const command of commandControls) {
      const center = point(command.rect.x + command.rect.width / 2,
        command.rect.y + command.rect.height / 2);
      await page.mouse.move(center.x, center.y);
      const hover = await page.screenshot({ path: path.join(folder, `${command.slug}-hover.png`),
        animations: "disabled" });
      await page.mouse.down();
      await page.evaluate(() => new Promise((resolve) => {
        requestAnimationFrame(() => requestAnimationFrame(resolve));
      }));
      const pressed = await page.screenshot({
        path: path.join(folder, `${command.slug}-pressed.png`), animations: "disabled",
      });
      const observed = await performCommand(page, command.expected, () => page.mouse.up());
      const released = await page.screenshot({
        path: path.join(folder, `${command.slug}-released.png`), animations: "disabled",
      });
      assert.notEqual(sha256(pressed), sha256(hover),
        `${faction}: ${command.slug} did not show its original pressed resource`);
      if (command.slug === "game-options") {
        assert.notEqual(sha256(released), sha256(fs.readFileSync(path.join(folder, "actual.png"))),
          `${faction}: Game Options did not open its original surface`);
        probes.push({
          type: "game-options-command-surface",
          ...verifyCommonBitmap(viewport, released, 20002, 0, 0, [
            { x: 0, y: 0, width: 640, height: 2 },
            { x: 0, y: 478, width: 640, height: 2 },
            { x: 0, y: 2, width: 2, height: 476 },
            { x: 638, y: 2, width: 2, height: 476 },
            { x: 22, y: 26, width: 596, height: 440 },
          ]),
        });
        await page.keyboard.press("Escape");
        const returned = await stableInteractionFrame(page, folder,
          `${command.slug}-returned-to-command-center`);
        assert.equal(sha256(returned), sha256(fs.readFileSync(path.join(folder, "actual.png"))),
          `${faction}: Game Options did not return to the authentic command center`);
      } else {
        assert.equal(sha256(released), sha256(fs.readFileSync(path.join(folder, "actual.png"))),
          `${faction}: ${command.slug} exposed a replacement destination`);
      }
      probes.push({ type: "physical-command", command: command.slug, expected: command.expected,
        observed, hover_sha256: sha256(hover), pressed_sha256: sha256(pressed),
        released_sha256: sha256(released) });
    }

    for (const [key, expected] of [
      ["F1", "command=0x133 destination=game_options status=opened_original"],
      ["F7", "command=0x131 destination=encyclopedia status=pending_original_window"],
    ]) {
      const observed = await performCommand(page, expected, () => page.keyboard.press(key));
      probes.push({ type: "keyboard-command", key, expected, observed });
      if (key === "F1") {
        const options = await stableInteractionFrame(page, folder, "keyboard-game-options-open");
        assert.notEqual(sha256(options), sha256(fs.readFileSync(path.join(folder, "actual.png"))),
          `${faction}: F1 did not open the original Game Options surface`);
        await page.keyboard.press("Escape");
        const returned = await stableInteractionFrame(page, folder,
          "keyboard-game-options-returned");
        assert.equal(sha256(returned), sha256(fs.readFileSync(path.join(folder, "actual.png"))),
          `${faction}: F1 Game Options did not return to the command center`);
      }
    }

    await page.keyboard.press("E");
    await page.evaluate(() => new Promise((resolve) => {
      requestAnimationFrame(() => requestAnimationFrame(resolve));
    }));
    const legacyEncyclopedia = await page.screenshot({
      path: path.join(folder, "legacy-encyclopedia-shortcut.png"), animations: "disabled",
    });
    assert.equal(sha256(legacyEncyclopedia), sha256(fs.readFileSync(path.join(folder, "actual.png"))),
      `${faction}: legacy E shortcut exposed a replacement Encyclopedia`);
    probes.push({ type: "legacy-encyclopedia-shortcut", key: "E", status: "fails-closed",
      screenshot_sha256: sha256(legacyEncyclopedia) });
  }

  if (scenario.mode !== null && scenario.mode !== "Display Off") {
    const legendPoint = point(faction === "alliance" ? 55 : 113, 50);
    const compact = await page.screenshot({ path: path.join(folder, "legend-compact.png"),
      clip: { x: legendPoint.x, y: legendPoint.y, width: 47 * scale, height: 25 * scale },
      animations: "disabled" });
    probes.push({ type: "compact-legend-bitmap", x: legendPoint.x, y: legendPoint.y,
      screenshot_sha256: sha256(compact) });
  }

  if (scenario.slug === "hover") {
    const system = point(ready.probe_screen_x, ready.probe_screen_y);
    await page.mouse.move(system.x, system.y);
    await page.waitForFunction((datId) => window.__openRebellionInterfaceHover?.system_dat_id === datId,
      ready.probe_system_dat_id, { timeout: 2_000 });
    const hovered = await page.evaluate(() => window.__openRebellionInterfaceHover);
    assert.equal(hovered.system_name, ready.probe_system_name);
    const capture = await page.screenshot({ path: path.join(folder, "system-hover.png"), animations: "disabled" });
    probes.push({ type: "system-hover", dat_id: hovered.system_dat_id,
      name: hovered.system_name, screenshot_sha256: sha256(capture) });
  }

  if (["pan", "zoom"].includes(scenario.slug)) {
    const start = point(320, 170);
    const before = await page.screenshot({ animations: "disabled" });
    await page.mouse.move(start.x, start.y);
    if (scenario.slug === "pan") {
      await page.mouse.down({ button: "right" });
      await page.mouse.move(start.x + 35 * scale, start.y + 15 * scale, { steps: 5 });
      await page.mouse.up({ button: "right" });
    } else {
      await page.mouse.wheel(0, -120);
    }
    await page.waitForTimeout(100);
    const after = await page.screenshot({ path: path.join(folder, `${scenario.slug}-interaction.png`), animations: "disabled" });
    assert.notEqual(sha256(before), sha256(after), `${scenario.slug} did not redraw`);
    probes.push({ type: scenario.slug, before_sha256: sha256(before), after_sha256: sha256(after) });
  }

  return probes;
}

function verifyTacticalBitmap(
  viewport, screenshotBytes, id, x0, y0, occluders = [], ignoredRects = [],
) {
  if (viewport.width !== 640 || viewport.height !== 480 || viewport.device_scale_factor !== 1) {
    return { status: "non-native-scale", pixels_checked: 0 };
  }
  const screenshot = PNG.sync.read(screenshotBytes);
  const source = decodeIndexedBmp(fs.readFileSync(path.join(
    root, `data/base/ui/tactical-dll/BMP/${id}.bmp`,
  )));
  const decodedOccluders = occluders.map(({ id: overlayId, x, y }) => ({
    x,
    y,
    source: decodeIndexedBmp(fs.readFileSync(path.join(
      root, `data/base/ui/tactical-dll/BMP/${overlayId}.bmp`,
    ))),
  }));
  let pixelsChecked = 0;
  for (let y = 0; y < source.height; y++) {
    for (let x = 0; x < source.width; x++) {
      if (ignoredRects.some((rect) => x >= rect.x && y >= rect.y
        && x < rect.x + rect.width && y < rect.y + rect.height)) continue;
      const color = source.pixel(x, y);
      if (color[0] < 32 && color[1] < 32 && color[2] > 192) continue;
      const screenX = x0 + x;
      const screenY = y0 + y;
      const occluded = decodedOccluders.some(({ x: overlayX, y: overlayY, source: overlay }) => {
        const localX = screenX - overlayX;
        const localY = screenY - overlayY;
        if (localX < 0 || localY < 0 || localX >= overlay.width || localY >= overlay.height) {
          return false;
        }
        const overlayColor = overlay.pixel(localX, localY);
        return !(overlayColor[0] < 32 && overlayColor[1] < 32 && overlayColor[2] > 192);
      });
      if (occluded) continue;
      const actualOffset = ((y0 + y) * screenshot.width + x0 + x) * 4;
      assert.deepEqual(
        Array.from(screenshot.data.subarray(actualOffset, actualOffset + 3)), color,
        `tactical control ${id} pixel (${x}, ${y}) differs from its original BMP`,
      );
      pixelsChecked++;
    }
  }
  return { status: "source-bitmap-exact", resource_id: id, pixels_checked: pixelsChecked };
}

function verifyGokresBitmap(viewport, screenshotBytes, id, x0, y0) {
  if (viewport.width !== 640 || viewport.height !== 480 || viewport.device_scale_factor !== 1) {
    return { status: "non-native-scale", pixels_checked: 0 };
  }
  const screenshot = PNG.sync.read(screenshotBytes);
  const source = decodeIndexedBmp(fs.readFileSync(path.join(
    root, `data/base/ui/gokres-dll/BMP/${id}.bmp`,
  )));
  let pixelsChecked = 0;
  for (let y = 0; y < source.height; y++) {
    for (let x = 0; x < source.width; x++) {
      const color = source.pixel(x, y);
      if (color[0] < 32 && color[1] < 32 && color[2] > 192) continue;
      const actualOffset = ((y0 + y) * screenshot.width + x0 + x) * 4;
      assert.deepEqual(
        Array.from(screenshot.data.subarray(actualOffset, actualOffset + 3)), color,
        `GOKRES content ${id} pixel (${x}, ${y}) differs from its original BMP`,
      );
      pixelsChecked++;
    }
  }
  assert.ok(pixelsChecked > 0, `GOKRES content ${id} contained no opaque proof pixels`);
  return { status: "source-bitmap-exact", resource_id: id, pixels_checked: pixelsChecked };
}

function verifyTacticalBackdrop(viewport, screenshotBytes, paletteResourceId) {
  if (viewport.width !== 640 || viewport.height !== 480 || viewport.device_scale_factor !== 1) {
    return { status: "non-native-scale", bright_pixels_checked: 0 };
  }
  const screenshot = PNG.sync.read(screenshotBytes);
  const runtimeRoot = path.join(root, "data/base/ui/tactical-dll/TACTICAL3D/runtime");
  const manifest = JSON.parse(fs.readFileSync(path.join(runtimeRoot, "manifest.json"), "utf8"));
  const textureRecord = manifest.textures.find(({ identifier_kind: kind, id }) =>
    kind === "id" && id === 5030);
  const paletteRecord = manifest.textures.find(({ identifier_kind: kind, id }) =>
    kind === "id" && id === paletteResourceId);
  assert.ok(textureRecord && paletteRecord, "resource 5030 or its active palette is missing");
  const texture = fs.readFileSync(path.join(runtimeRoot, textureRecord.object));
  const palette = fs.readFileSync(path.join(runtimeRoot, paletteRecord.object));
  assert.equal(texture.subarray(0, 8).toString("ascii"), "ORTINDEX");
  assert.equal(palette.subarray(0, 8).toString("ascii"), "ORTPAL00");
  assert.equal(texture.readUInt32LE(8), 1);
  assert.equal(palette.readUInt32LE(8), 1);
  assert.equal(palette.readUInt32LE(12), paletteResourceId);
  const width = texture.readUInt32LE(12);
  const height = texture.readUInt32LE(16);
  const pixelCount = texture.readUInt32LE(28);
  assert.deepEqual([width, height, pixelCount], [440, 438, 440 * 438]);
  const indices = texture.subarray(36, 36 + pixelCount);
  const rgb = palette.subarray(16, 16 + 256 * 3);
  assert.equal(indices.length, pixelCount);
  assert.equal(rgb.length, 256 * 3);
  let brightPixels = 0;
  let matchingBrightPixels = 0;
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const index = indices[y * width + x];
      const color = Array.from(rgb.subarray(index * 3, index * 3 + 3));
      if (Math.max(...color) < 16) continue;
      brightPixels++;
      const offset = ((29 + y) * screenshot.width + 18 + x) * 4;
      const actual = Array.from(screenshot.data.subarray(offset, offset + 3));
      if (actual.every((channel, index) => channel === color[index])) matchingBrightPixels++;
    }
  }
  assert.ok(brightPixels > 100, "resource 5030 lacks a meaningful bright-star sample");
  const matchRatio = matchingBrightPixels / brightPixels;
  assert.ok(matchRatio > 0.6,
    `resource 5030 bright-star match ratio was ${matchRatio.toFixed(3)}`);
  return {
    status: "source-bitmap-visible",
    resource_id: 5030,
    bright_pixels_checked: brightPixels,
    matching_bright_pixels: matchingBrightPixels,
    match_ratio: matchRatio,
  };
}

function verifyTacticalGreenBeam(viewport, beforeBytes, beamBytes) {
  const before = PNG.sync.read(beforeBytes);
  const beam = PNG.sync.read(beamBytes);
  assert.deepEqual([beam.width, beam.height], [before.width, before.height]);
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const minX = Math.floor(offsetX + 16 * scale);
  const maxX = Math.ceil(offsetX + 460 * scale);
  const minY = Math.floor(offsetY + 28 * scale);
  const maxY = Math.ceil(offsetY + 467 * scale);
  let changedGreenPixels = 0;
  for (let y = minY; y < maxY; y++) {
    for (let x = minX; x < maxX; x++) {
      const offset = (y * beam.width + x) * 4;
      const red = beam.data[offset];
      const green = beam.data[offset + 1];
      const blue = beam.data[offset + 2];
      const changed = red !== before.data[offset]
        || green !== before.data[offset + 1]
        || blue !== before.data[offset + 2];
      if (changed && green > 96 && green > red * 1.5 && green > blue * 1.25) {
        changedGreenPixels++;
      }
    }
  }
  assert.ok(changedGreenPixels > 0, "Death Star beam produced no visible green aperture pixels");
  return { status: "visible", changed_green_pixels: changedGreenPixels };
}

function verifyStrategyBitmap(viewport, screenshotBytes, id, x0, y0, ignoredRects = []) {
  if (viewport.width !== 640 || viewport.height !== 480 || viewport.device_scale_factor !== 1) {
    return { status: "non-native-scale", pixels_checked: 0 };
  }
  const screenshot = PNG.sync.read(screenshotBytes);
  const source = decodeIndexedBmp(fs.readFileSync(path.join(
    root, `data/base/ui/strategy-dll/BMP/${id}.bmp`,
  )));
  let pixelsChecked = 0;
  for (let y = 0; y < source.height; y++) {
    for (let x = 0; x < source.width; x++) {
      if (ignoredRects.some((rect) => x >= rect.x && y >= rect.y
        && x < rect.x + rect.width && y < rect.y + rect.height)) continue;
      const color = source.pixel(x, y);
      if (color[0] < 32 && color[1] < 32 && color[2] > 192) continue;
      const actualOffset = ((y0 + y) * screenshot.width + x0 + x) * 4;
      assert.deepEqual(
        Array.from(screenshot.data.subarray(actualOffset, actualOffset + 3)), color,
        `strategy control ${id} pixel (${x}, ${y}) differs from its original BMP`,
      );
      pixelsChecked++;
    }
  }
  return { status: "source-bitmap-exact", resource_id: id, pixels_checked: pixelsChecked };
}

function verifyCommonBitmap(viewport, screenshotBytes, id, x0, y0, ignoredRects = []) {
  if (viewport.width !== 640 || viewport.height !== 480 || viewport.device_scale_factor !== 1) {
    return { status: "non-native-scale", pixels_checked: 0 };
  }
  const screenshot = PNG.sync.read(screenshotBytes);
  const source = decodeIndexedBmp(fs.readFileSync(path.join(
    root, `data/base/ui/common-dll/BMP/${id}.bmp`,
  )));
  let pixelsChecked = 0;
  for (let y = 0; y < source.height; y++) {
    for (let x = 0; x < source.width; x++) {
      if (ignoredRects.some((rect) => x >= rect.x && y >= rect.y
        && x < rect.x + rect.width && y < rect.y + rect.height)) continue;
      const color = source.pixel(x, y);
      const actualOffset = ((y0 + y) * screenshot.width + x0 + x) * 4;
      assert.deepEqual(
        Array.from(screenshot.data.subarray(actualOffset, actualOffset + 3)), color,
        `common control ${id} pixel (${x}, ${y}) differs from its original BMP`,
      );
      pixelsChecked++;
    }
  }
  return { status: "source-bitmap-exact", resource_id: id, pixels_checked: pixelsChecked };
}

function verifyTacticalTransparentOverlay(viewport, screenshotBytes, id, x0, y0) {
  if (viewport.width !== 640 || viewport.height !== 480 || viewport.device_scale_factor !== 1) {
    return { status: "non-native-scale", pixels_checked: 0 };
  }
  const screenshot = PNG.sync.read(screenshotBytes);
  const source = decodeIndexedBmp(fs.readFileSync(path.join(
    root, `data/base/ui/tactical-dll/BMP/${id}.bmp`,
  )));
  let pixelsChecked = 0;
  for (let y = 0; y < source.height; y++) {
    for (let x = 0; x < source.width; x++) {
      const color = source.pixel(x, y);
      const blueMatte = color[0] < 32 && color[1] < 32 && color[2] > 192;
      const limeMatte = color[0] < 32 && color[1] > 192 && color[2] < 32;
      if (blueMatte || limeMatte) continue;
      const actualOffset = ((y0 + y) * screenshot.width + x0 + x) * 4;
      assert.deepEqual(
        Array.from(screenshot.data.subarray(actualOffset, actualOffset + 3)), color,
        `tactical overlay ${id} pixel (${x}, ${y}) differs from its original BMP`,
      );
      pixelsChecked++;
    }
  }
  assert.ok(pixelsChecked > 0, `tactical overlay ${id} has no inspectable pixels`);
  return { status: "source-bitmap-exact", resource_id: id, pixels_checked: pixelsChecked };
}

function verifyTacticalShell(viewport, screenshotBytes) {
  if (viewport.width !== 640 || viewport.height !== 480 || viewport.device_scale_factor !== 1) {
    return { status: "non-native-scale", pixels_checked: 0 };
  }
  const screenshot = PNG.sync.read(screenshotBytes);
  const source = decodeIndexedBmp(fs.readFileSync(path.join(
    root, "data/base/ui/tactical-dll/BMP/1000.bmp",
  )));
  assert.equal(source.width, 640);
  assert.equal(source.height, 480);
  let pixelsChecked = 0;
  // Uncovered corner and bottom-edge pixels must be the original shell. The
  // aperture, group headers, selected panel, and controls are layered above it.
  const points = [];
  // The outermost two WebGL edge pixels are antialiased against the canvas
  // backing color; compare the first fully covered source pixels instead.
  for (let y = 2; y < 8; y++) {
    for (let x = 2; x < 8; x++) {
      points.push([x, y], [628 + x, y]);
    }
  }
  for (let x = 2; x < 638; x++) points.push([x, 476]);
  for (const [x, y] of points) {
    const offset = (y * 640 + x) * 4;
    assert.deepEqual(Array.from(screenshot.data.subarray(offset, offset + 3)), source.pixel(x, y),
      `tactical shell pixel (${x}, ${y}) differs from TACTICAL 1000`);
    pixelsChecked++;
  }
  return { status: "source-bitmap-exact", resource_id: 1000, pixels_checked: pixelsChecked };
}

function verifyTacticalApertureIsolation(viewport, beforeBytes, afterBytes, ignoreNativeRects = []) {
  const before = PNG.sync.read(beforeBytes);
  const after = PNG.sync.read(afterBytes);
  assert.equal(before.width, after.width);
  assert.equal(before.height, after.height);
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const aperture = {
    x0: Math.round(offsetX + 16 * scale),
    y0: Math.round(offsetY + 28 * scale),
    x1: Math.round(offsetX + 460 * scale),
    y1: Math.round(offsetY + 467 * scale),
  };
  const ignored = ignoreNativeRects.map(({ x, y, width, height, reason }) => ({
    x0: Math.floor(offsetX + x * scale),
    y0: Math.floor(offsetY + y * scale),
    x1: Math.ceil(offsetX + (x + width) * scale),
    y1: Math.ceil(offsetY + (y + height) * scale),
    reason,
  }));
  let pixelsChecked = 0;
  for (let y = 0; y < before.height; y++) {
    for (let x = 0; x < before.width; x++) {
      if (x >= aperture.x0 && x < aperture.x1 && y >= aperture.y0 && y < aperture.y1) continue;
      if (ignored.some((rect) =>
        x >= rect.x0 && x < rect.x1 && y >= rect.y0 && y < rect.y1)) continue;
      const offset = (y * before.width + x) * 4;
      assert.deepEqual(
        Array.from(after.data.subarray(offset, offset + 4)),
        Array.from(before.data.subarray(offset, offset + 4)),
        `battlefield redraw escaped tactical aperture at (${x}, ${y})`,
      );
      pixelsChecked++;
    }
  }
  return {
    status: "isolated",
    pixels_checked: pixelsChecked,
    ...(ignoreNativeRects.length ? { ignored_native_rects: ignoreNativeRects } : {}),
  };
}

function tacticalApertureHash(viewport, screenshotBytes) {
  const screenshot = PNG.sync.read(screenshotBytes);
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const x0 = Math.round(offsetX + 16 * scale);
  const y0 = Math.round(offsetY + 28 * scale);
  const x1 = Math.round(offsetX + 460 * scale);
  const y1 = Math.round(offsetY + 467 * scale);
  const hash = createHash("sha256");
  for (let y = y0; y < y1; y++) {
    hash.update(screenshot.data.subarray((y * screenshot.width + x0) * 4,
      (y * screenshot.width + x1) * 4));
  }
  return hash.digest("hex");
}

function tacticalProofModelHash(viewport, screenshotBytes) {
  const screenshot = PNG.sync.read(screenshotBytes);
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const x0 = Math.floor(offsetX + 180 * scale);
  const y0 = Math.floor(offsetY + 80 * scale);
  const x1 = Math.ceil(offsetX + 440 * scale);
  const y1 = Math.ceil(offsetY + 320 * scale);
  const hash = createHash("sha256");
  for (let y = y0; y < y1; y++) {
    hash.update(screenshot.data.subarray((y * screenshot.width + x0) * 4,
      (y * screenshot.width + x1) * 4));
  }
  return hash.digest("hex");
}

function tacticalApertureChangedPixels(viewport, proofBytes, controlBytes) {
  const proof = PNG.sync.read(proofBytes);
  const control = PNG.sync.read(controlBytes);
  assert.equal(proof.width, control.width);
  assert.equal(proof.height, control.height);
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const x0 = Math.floor(offsetX + 16 * scale);
  const y0 = Math.floor(offsetY + 28 * scale);
  const x1 = Math.ceil(offsetX + 460 * scale);
  const y1 = Math.ceil(offsetY + 476 * scale);
  let changed = 0;
  for (let y = y0; y < y1; y++) {
    for (let x = x0; x < x1; x++) {
      const offset = (y * proof.width + x) * 4;
      if (Math.abs(proof.data[offset] - control.data[offset]) > 2
        || Math.abs(proof.data[offset + 1] - control.data[offset + 1]) > 2
        || Math.abs(proof.data[offset + 2] - control.data[offset + 2]) > 2) {
        changed++;
      }
    }
  }
  return changed;
}

function tacticalModelRegionChangedPixels(
  viewport, proofBytes, controlBytes, center, logicalRadius = 12,
) {
  const proof = PNG.sync.read(proofBytes);
  const control = PNG.sync.read(controlBytes);
  assert.equal(proof.width, control.width);
  assert.equal(proof.height, control.height);
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const radius = Math.ceil(logicalRadius * scale);
  const x0 = Math.max(0, Math.floor(center.x - radius));
  const y0 = Math.max(0, Math.floor(center.y - radius));
  const x1 = Math.min(proof.width, Math.ceil(center.x + radius));
  const y1 = Math.min(proof.height, Math.ceil(center.y + radius));
  let changed = 0;
  for (let y = y0; y < y1; y++) {
    for (let x = x0; x < x1; x++) {
      const offset = (y * proof.width + x) * 4;
      if (Math.abs(proof.data[offset] - control.data[offset]) > 2
        || Math.abs(proof.data[offset + 1] - control.data[offset + 1]) > 2
        || Math.abs(proof.data[offset + 2] - control.data[offset + 2]) > 2) {
        changed++;
      }
    }
  }
  return changed;
}

function verifyTacticalProofNegativeControls(results, factions, viewports) {
  for (const faction of factions) {
    for (const viewport of viewports) {
      const proof = results.find(({ id }) =>
        id === `tactical/${faction}/battle-entry/${viewport.id}`);
      const control = results.find(({ id }) =>
        id === `tactical/${faction}/battle-entry-proof-off/${viewport.id}`);
      assert.ok(proof && control, `missing tactical proof pair for ${faction}/${viewport.id}`);
      if (proof.status !== "pass" || control.status !== "pass") continue;
      const proofImage = fs.readFileSync(path.join(runDir, proof.id, "actual.png"));
      const controlImage = fs.readFileSync(path.join(runDir, control.id, "actual.png"));
      const changedPixels = tacticalApertureChangedPixels(viewport, proofImage, controlImage);
      const scale = Math.min(viewport.width / 640, viewport.height / 480);
      const minimumChanged = Math.ceil(1000 * scale * scale);
      const probe = {
        type: "tactical-3d-pixel-submission-negative-control",
        proof_fixture_code: proof.fixture_code,
        control_fixture_code: control.fixture_code,
        changed_aperture_pixels: changedPixels,
        minimum_changed_pixels: minimumChanged,
      };
      for (const result of [proof, control]) {
        result.probes.push(probe);
        if (changedPixels < minimumChanged) {
          result.status = "fail";
          result.error = `tactical proof/control aperture differs by only ${changedPixels} pixels`;
        }
        fs.writeFileSync(path.join(runDir, result.id, "result.json"),
          `${JSON.stringify(result, null, 2)}\n`);
      }
    }
  }
}

function verifyTacticalProductionParticipants(results, factions, viewports) {
  for (const faction of factions) {
    for (const viewport of viewports) {
      const production = results.find(({ id }) =>
        id === `tactical/${faction}/production-participants/${viewport.id}`);
      const control = results.find(({ id }) =>
        id === `tactical/${faction}/production-participants-3d-off/${viewport.id}`);
      assert.ok(production && control,
        `missing tactical production participant pair for ${faction}/${viewport.id}`);
      if (production.status !== "pass" || control.status !== "pass") continue;
      const productionImage = fs.readFileSync(path.join(runDir, production.id, "actual.png"));
      const controlImage = fs.readFileSync(path.join(runDir, control.id, "actual.png"));
      const changedPixels = tacticalApertureChangedPixels(
        viewport, productionImage, controlImage,
      );
      const scale = Math.min(viewport.width / 640, viewport.height / 480);
      const sceneLog = production.console.find(({ text }) =>
        text.includes("[tactical_3d] participant_scene"));
      const positionMatch = sceneLog?.text.match(/screen_positions=([^ ]+)/);
      assert.ok(positionMatch, "production participant scene omitted projected positions");
      const modelRegions = positionMatch[1].split(";").map((entry) => {
        const match = entry.match(/^(\d+):(-?[\d.]+),(-?[\d.]+)$/);
        assert.ok(match, `invalid projected participant position ${entry}`);
        const center = { x: Number(match[2]), y: Number(match[3]) };
        return {
          object_id: Number(match[1]),
          center,
          changed_pixels: tacticalModelRegionChangedPixels(
            viewport, productionImage, controlImage, center,
          ),
          minimum_changed_pixels: Math.ceil(2 * scale * scale),
        };
      });
      assert.deepEqual(modelRegions.map(({ object_id }) => object_id), [1, 2]);
      const fighterSceneLog = production.console.find(({ text }) =>
        text.includes("[tactical_3d] fighter_scene"));
      const fighterPositionMatch = fighterSceneLog?.text.match(/screen_positions=([^ ]+)/);
      assert.ok(fighterPositionMatch, "production fighter scene omitted projected positions");
      const fighterRegions = fighterPositionMatch[1].split(";").map((entry) => {
        const match = entry.match(/^(\d+):(-?[\d.]+),(-?[\d.]+)$/);
        assert.ok(match, `invalid projected fighter position ${entry}`);
        const center = { x: Number(match[2]), y: Number(match[3]) };
        return {
          object_id: Number(match[1]),
          center,
          changed_pixels: tacticalModelRegionChangedPixels(
            viewport, productionImage, controlImage, center, 4,
          ),
          minimum_changed_pixels: Math.ceil(scale * scale),
        };
      });
      assert.deepEqual(fighterRegions.map(({ object_id }) => object_id), [1001, 1002]);
      const minimumChanged = [...modelRegions, ...fighterRegions].reduce(
        (total, region) => total + region.minimum_changed_pixels, 0,
      );
      const probe = {
        type: "production-tactical-participant-rendering",
        resource_families: [2010, 2510],
        selected_lod_resources: [2012, 2512],
        changed_aperture_pixels: changedPixels,
        minimum_changed_pixels: minimumChanged,
        model_regions: modelRegions,
        fighter_regions: fighterRegions,
      };
      for (const result of [production, control]) result.probes.push(probe);
      if (changedPixels < minimumChanged
        || [...modelRegions, ...fighterRegions].some((region) =>
          region.changed_pixels < region.minimum_changed_pixels)) {
        production.status = "fail";
        control.status = "fail";
        production.error = "production participant framebuffer lacks visible source pixels";
        control.error = production.error;
      }
      for (const result of [production, control]) {
        fs.writeFileSync(path.join(runDir, result.id, "result.json"),
          `${JSON.stringify(result, null, 2)}\n`);
      }
    }
  }
}

function verifyTacticalLodFamily(results, factions, viewports) {
  const slugs = ["lod-close", "lod-medium", "lod-far"];
  for (const faction of factions) {
    for (const viewport of viewports) {
      const family = slugs.map((slug) => results.find(({ id }) =>
        id === `tactical/${faction}/${slug}/${viewport.id}`));
      assert.ok(family.every(Boolean), `missing tactical LOD family for ${faction}/${viewport.id}`);
      if (family.some(({ status }) => status !== "pass")) continue;
      const images = family.map(({ id }) =>
        fs.readFileSync(path.join(runDir, id, "actual.png")));
      const hashes = images.map((bytes) => tacticalApertureHash(viewport, bytes));
      assert.equal(new Set(hashes).size, 3,
        `tactical LOD views are not distinct for ${faction}/${viewport.id}`);
      const closeMediumChanged = tacticalApertureChangedPixels(viewport, images[0], images[1]);
      const mediumFarChanged = tacticalApertureChangedPixels(viewport, images[1], images[2]);
      assert.ok(closeMediumChanged > 100 && mediumFarChanged > 100,
        `tactical LOD selections changed too few aperture pixels for ${faction}/${viewport.id}`);
      const closeMediumIsolation = verifyTacticalApertureIsolation(viewport, images[0], images[1]);
      const mediumFarIsolation = verifyTacticalApertureIsolation(viewport, images[1], images[2]);
      const probe = {
        type: "tactical-3d-three-lod-selection-matrix",
        resources: [2560, 2561, 2562],
        aperture_hashes: hashes,
        close_medium_changed_pixels: closeMediumChanged,
        medium_far_changed_pixels: mediumFarChanged,
        close_medium_isolation: closeMediumIsolation,
        medium_far_isolation: mediumFarIsolation,
      };
      for (const result of family) {
        result.probes.push(probe);
        fs.writeFileSync(path.join(runDir, result.id, "result.json"),
          `${JSON.stringify(result, null, 2)}\n`);
      }
    }
  }
}

function verifyWireframeColor(viewport, onBytes, offBytes, faction) {
  const on = PNG.sync.read(onBytes);
  const off = PNG.sync.read(offBytes);
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const x0 = Math.round(offsetX + 16 * scale);
  const y0 = Math.round(offsetY + 28 * scale);
  const x1 = Math.round(offsetX + 460 * scale);
  const y1 = Math.round(offsetY + 467 * scale);
  let changedPixels = 0;
  let factionColorPixels = 0;
  for (let y = y0; y < y1; y++) {
    for (let x = x0; x < x1; x++) {
      const offset = (y * on.width + x) * 4;
      const onPixel = Array.from(on.data.subarray(offset, offset + 3));
      const offPixel = Array.from(off.data.subarray(offset, offset + 3));
      if (onPixel.every((channel, index) => channel === offPixel[index])) continue;
      changedPixels++;
      const [red, green, blue] = onPixel;
      const factionColor = faction === "alliance"
        ? red > 80 && red > green * 2 && red > blue * 2
        : green > 80 && green > red * 2 && green > blue * 1.5;
      if (factionColor) factionColorPixels++;
    }
  }
  assert.ok(changedPixels > 0, `${faction} highlight changed no aperture pixels`);
  assert.ok(factionColorPixels > 0, `${faction} highlight used the wrong wireframe color`);
  return { status: "verified", changed_pixels: changedPixels,
    faction_color_pixels: factionColorPixels };
}

async function probeTactical(page, viewport, folder, stable) {
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const point = (x, y) => ({ x: offsetX + x * scale, y: offsetY + y * scale });
  const capture = async (name) => stableInteractionFrame(page, folder, name);
  const probes = [{ type: "tactical-shell", ...verifyTacticalShell(viewport, stable.bytes) }];
  probes.push({ type: "paused-control-bitmap",
    ...verifyTacticalBitmap(viewport, stable.bytes, 1061, 560, 307) });
  probes.push({ type: "alliance-highlight-on",
    ...verifyTacticalBitmap(viewport, stable.bytes, 1035, 517, 304) });
  probes.push({ type: "empire-highlight-on",
    ...verifyTacticalBitmap(viewport, stable.bytes, 1037, 482, 304) });

  const outside = point(558, 318);
  await page.mouse.click(outside.x, outside.y);
  await page.waitForTimeout(80);
  const outsideFrame = await capture("pause-outside-edge");
  assert.equal(sha256(stable.bytes), sha256(outsideFrame), "outside-edge changed paused battle");
  probes.push({ type: "pause-outside-edge", unchanged: true });

  const pauseMatte = point(561.5, 308.5);
  await page.mouse.click(pauseMatte.x, pauseMatte.y);
  await page.waitForTimeout(80);
  const pauseMatteFrame = await capture("pause-transparent-matte");
  assert.equal(sha256(stable.bytes), sha256(pauseMatteFrame),
    "transparent pause-control matte pixel changed paused battle");
  probes.push({ type: "pause-transparent-matte", source_pixel: [1, 1], unchanged: true });

  const pause = point(574, 318);
  await page.mouse.click(pause.x, pause.y);
  await page.waitForTimeout(80);
  const resumed = await dynamicInteractionFrame(page, folder, "resumed");
  assert.notEqual(sha256(stable.bytes), sha256(resumed), "pause control did not resume battle");
  probes.push({ type: "resume", ...verifyTacticalBitmap(viewport, resumed, 1060, 560, 307) });
  await page.mouse.click(pause.x, pause.y);
  await page.waitForTimeout(80);
  const pausedAgain = await capture("paused-again");
  probes.push({ type: "pause", ...verifyTacticalBitmap(viewport, pausedAgain, 1061, 560, 307) });
  await page.waitForTimeout(250);
  const pauseStable = await capture("pause-stable");
  assert.equal(sha256(pausedAgain), sha256(pauseStable), "battle continued after re-pause");
  probes.push({ type: "paused-stability", stable: true,
    screenshot_sha256: sha256(pauseStable) });

  const allianceHighlight = point(532, 317);
  await page.mouse.click(allianceHighlight.x, allianceHighlight.y);
  await page.waitForTimeout(80);
  const highlightOff = await capture("alliance-highlight-off");
  assert.notEqual(sha256(pausedAgain), sha256(highlightOff), "Alliance highlight did not toggle");
  assert.notEqual(tacticalApertureHash(viewport, pauseStable),
    tacticalApertureHash(viewport, highlightOff), "Alliance battle wireframe did not dim");
  probes.push({ type: "alliance-highlight-off",
    ...verifyTacticalBitmap(viewport, highlightOff, 1034, 517, 304),
    wireframe: verifyWireframeColor(viewport, pauseStable, highlightOff, "alliance") });
  await page.mouse.click(allianceHighlight.x, allianceHighlight.y);
  await page.waitForTimeout(80);
  const allianceRestored = await capture("alliance-highlight-restored");
  assert.equal(sha256(pauseStable), sha256(allianceRestored),
    "Alliance highlight did not restore the paused display");
  probes.push({ type: "alliance-highlight-restored",
    ...verifyTacticalBitmap(viewport, allianceRestored, 1035, 517, 304) });

  const empireHighlight = point(497, 317);
  await page.mouse.click(empireHighlight.x, empireHighlight.y);
  await page.waitForTimeout(80);
  const empireOff = await capture("empire-highlight-off");
  assert.notEqual(sha256(allianceRestored), sha256(empireOff), "Empire highlight did not toggle");
  assert.notEqual(tacticalApertureHash(viewport, allianceRestored),
    tacticalApertureHash(viewport, empireOff), "Empire battle wireframe did not dim");
  probes.push({ type: "empire-highlight-off",
    ...verifyTacticalBitmap(viewport, empireOff, 1036, 482, 304),
    wireframe: verifyWireframeColor(viewport, allianceRestored, empireOff, "empire") });
  await page.mouse.click(empireHighlight.x, empireHighlight.y);
  await page.waitForTimeout(80);
  const highlightsRestored = await capture("highlights-restored");
  assert.equal(sha256(pauseStable), sha256(highlightsRestored),
    "Empire highlight did not restore the paused display");
  probes.push({ type: "empire-highlight-restored",
    ...verifyTacticalBitmap(viewport, highlightsRestored, 1037, 482, 304) });

  const zoomInMatte = point(487.5, 344.5);
  await page.mouse.click(zoomInMatte.x, zoomInMatte.y);
  await page.waitForTimeout(80);
  const zoomInMatteFrame = await capture("zoom-in-transparent-matte");
  assert.equal(sha256(highlightsRestored), sha256(zoomInMatteFrame),
    "transparent zoom-in matte pixel changed paused battle");
  probes.push({ type: "zoom-in-transparent-matte", source_pixel: [1, 1], unchanged: true });

  const zoomIn = point(498, 355);
  await page.mouse.move(zoomIn.x, zoomIn.y);
  await page.mouse.down({ button: "left" });
  await page.waitForTimeout(80);
  const zoomInPressed = await capture("zoom-in-pressed");
  probes.push({ type: "zoom-in-pressed",
    ...verifyTacticalBitmap(viewport, zoomInPressed, 1045, 486, 343) });
  await page.mouse.up({ button: "left" });
  await page.waitForTimeout(80);
  const zoomed = await capture("zoomed-in");
  assert.notEqual(sha256(highlightsRestored), sha256(zoomed), "zoom-in control did not redraw battle");
  probes.push({ type: "zoom-in", ...verifyTacticalBitmap(viewport, zoomed, 1044, 486, 343),
    aperture_isolation: verifyTacticalApertureIsolation(viewport, highlightsRestored, zoomed) });

  const zoomOutMatte = point(604.5, 344.5);
  await page.mouse.click(zoomOutMatte.x, zoomOutMatte.y);
  await page.waitForTimeout(80);
  const zoomOutMatteFrame = await capture("zoom-out-transparent-matte");
  assert.equal(sha256(zoomed), sha256(zoomOutMatteFrame),
    "transparent zoom-out matte pixel changed zoomed battle");
  probes.push({ type: "zoom-out-transparent-matte", source_pixel: [1, 1], unchanged: true });

  const zoomOut = point(615, 355);
  await page.mouse.move(zoomOut.x, zoomOut.y);
  await page.mouse.down({ button: "left" });
  await page.waitForTimeout(80);
  const zoomOutPressed = await capture("zoom-out-pressed");
  probes.push({ type: "zoom-out-pressed",
    ...verifyTacticalBitmap(viewport, zoomOutPressed, 1047, 603, 343) });
  await page.mouse.up({ button: "left" });
  await page.waitForTimeout(80);
  const zoomRestored = await capture("zoom-restored");
  assert.equal(sha256(highlightsRestored), sha256(zoomRestored),
    "zoom-out did not restore the original paused battlefield");
  probes.push({ type: "zoom-out", ...verifyTacticalBitmap(viewport, zoomRestored, 1046, 603, 343),
    aperture_isolation: verifyTacticalApertureIsolation(viewport, zoomed, zoomRestored) });
  return probes;
}

async function probeTacticalLodJourney(page, viewport, folder, stable) {
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const point = (x, y) => ({ x: offsetX + x * scale, y: offsetY + y * scale });
  const capture = async (name) => stableInteractionFrame(page, folder, name);
  const clickRepeatedly = async (location, count) => {
    for (let index = 0; index < count; index++) {
      await page.mouse.click(location.x, location.y);
      await page.waitForTimeout(80);
    }
  };
  const zoomOut = point(615, 355);
  const zoomIn = point(498, 355);

  await clickRepeatedly(zoomOut, 1);
  const medium = await capture("lod-journey-medium");
  await clickRepeatedly(zoomOut, 5);
  const far = await capture("lod-journey-far");
  await clickRepeatedly(zoomIn, 1);
  const returnMedium = await capture("lod-journey-return-medium");
  // One additional authentic click reaches the existing 2.0 clamp exactly,
  // avoiding reciprocal-f32 drift that is visible at letterboxed resolution.
  await clickRepeatedly(zoomIn, 6);
  const restoredClose = await capture("lod-journey-restored-close");

  const apertureHashes = [stable.bytes, medium, far, returnMedium, restoredClose]
    .map((bytes) => tacticalApertureHash(viewport, bytes));
  const modelHashes = [stable.bytes, medium, far, returnMedium, restoredClose]
    .map((bytes) => tacticalProofModelHash(viewport, bytes));
  assert.equal(new Set(modelHashes.slice(0, 3)).size, 3,
    "same-renderer close, medium, and far journey views are not distinct");
  assert.equal(modelHashes[4], modelHashes[0],
    "same-renderer LOD journey did not restore its exact initial model view");
  const transitions = [
    [stable.bytes, medium],
    [medium, far],
    [far, returnMedium],
    [returnMedium, restoredClose],
  ];
  const provisionalTextMasks = [
    { x: 65, y: 4, width: 18, height: 18, reason: "provisional task-force number" },
    { x: 490, y: 35, width: 145, height: 20, reason: "provisional selected-ship label" },
    { x: 490, y: 208, width: 145, height: 22, reason: "provisional orders label" },
  ];
  const exactNativeScale = Number.isInteger(scale);
  const isolationMasks = exactNativeScale ? [] : provisionalTextMasks;
  const isolation = transitions.map(([before, after]) =>
    verifyTacticalApertureIsolation(viewport, before, after, isolationMasks));
  assert.ok(isolation.every(({ status }) => status === "isolated"),
    "same-renderer LOD journey changed pixels outside the tactical aperture");
  const restoredFullFrameExact = sha256(restoredClose) === sha256(stable.bytes);
  if (exactNativeScale) {
    assert.equal(restoredFullFrameExact, true,
      "native same-renderer LOD journey did not restore its exact initial frame");
  }

  return [{
    type: "tactical-3d-same-renderer-lod-journey",
    controls: ["zoom-out", "zoom-in"],
    expected_resources: [2560, 2561, 2562, 2561, 2560],
    aperture_hashes: apertureHashes,
    model_crop_native_rect: { x: 180, y: 80, width: 260, height: 240 },
    model_crop_hashes: modelHashes,
    aperture_isolation: isolation,
    restored_model_exact: true,
    restored_full_frame_exact: restoredFullFrameExact,
    native_full_frame_asserted: exactNativeScale,
  }];
}

async function probeTacticalCameraJourney(page, viewport, folder, stable) {
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const point = (x, y) => ({ x: offsetX + x * scale, y: offsetY + y * scale });
  const capture = async (name) => stableInteractionFrame(page, folder, name);
  const press = async (name, location, resource, resourceX, resourceY, occluders = []) => {
    await page.mouse.move(location.x, location.y);
    await page.mouse.down({ button: "left" });
    await page.waitForTimeout(80);
    const pressed = await capture(`${name}-pressed`);
    const bitmap = verifyTacticalBitmap(
      viewport, pressed, resource, resourceX, resourceY, occluders,
    );
    await page.mouse.up({ button: "left" });
    await page.waitForTimeout(80);
    const released = await capture(name);
    return { name, bitmap, released };
  };

  const initialModel = tacticalProofModelHash(viewport, stable.bytes);
  const transitions = [];
  transitions.push(await press("camera-zoom-in", point(498, 355), 1045, 486, 343));
  transitions.push(await press("camera-zoom-out", point(615, 355), 1047, 603, 343));
  const up = { id: 1052, x: 537, y: 344 };
  const down = { id: 1055, x: 537, y: 412 };
  const target = { id: 1058, x: 538, y: 379 };
  transitions.push(await press(
    "camera-left", point(520, 397), 1049, 511, 376, [up, down, target],
  ));
  transitions.push(await press(
    "camera-right", point(590, 397), 1051, 557, 376, [up, down, target],
  ));
  transitions.push(await press(
    "camera-up", point(558, 354), 1053, 537, 344, [target],
  ));
  transitions.push(await press("camera-down", point(558, 444), 1056, 537, 412));
  transitions.push(await press("camera-target", point(549, 390), 1059, 538, 379));

  const modelHashes = [initialModel, ...transitions.map(({ released }) =>
    tacticalProofModelHash(viewport, released))];
  assert.ok(new Set(modelHashes).size >= 4,
    "source camera journey did not produce distinct rendered model views");
  return transitions.map(({ name, bitmap }, index) => ({
    type: "source-camera-control",
    control: name,
    pressed_bitmap: bitmap,
    model_crop_sha256: modelHashes[index + 1],
  }));
}

function parseTacticalScenePositions(line, label) {
  const match = line?.text.match(/screen_positions=([^ ]+)/);
  assert.ok(match, `${label} omitted projected positions`);
  return new Map(match[1].split(";").map((entry) => {
    const position = entry.match(/^(\d+):(-?[\d.]+),(-?[\d.]+)$/);
    assert.ok(position, `invalid ${label} projected position ${entry}`);
    return [Number(position[1]), { x: Number(position[2]), y: Number(position[3]) }];
  }));
}

async function probeTacticalProductionParticipants(
  page, viewport, folder, stable, consoleLines, faction, ready,
) {
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const point = (x, y) => ({ x: offsetX + x * scale, y: offsetY + y * scale });
  const probes = probeTacticalSelectedDamagePresentation(
    viewport,
    folder,
    stable,
    ready,
    { hull: ready.selected_ship?.hull_max, shield: ready.selected_ship?.shield_max },
    { filename: "selected-healthy.png", type: "source-mapped-selected-capital-healthy" },
  );

  const assignedTaskForces = new Set(ready.participants
    .filter(({ kind, faction: owner, active, task_force }) =>
      kind === "capital-ship" && owner === faction && active && Number.isInteger(task_force))
    .map(({ task_force }) => task_force));
  const assignedFighterGroups = new Set(ready.participants
    .filter(({ kind, faction: owner, active, fighter_group }) =>
      kind === "fighter-group" && owner === faction && active && Number.isInteger(fighter_group))
    .map(({ fighter_group }) => fighter_group));
  const unassignedTaskForces = [];
  for (let index = 0; index < 8; index++) {
    if (assignedTaskForces.has(index)) continue;
    unassignedTaskForces.push({
      slot: index + 1,
      ...verifyTacticalBitmap(viewport, stable.bytes, 1007, 60 + index * 26, 2),
    });
  }
  const unassignedFighterGroups = [];
  for (let index = 0; index < 4; index++) {
    if (assignedFighterGroups.has(index)) continue;
    unassignedFighterGroups.push({
      slot: index + 1,
      ...verifyTacticalBitmap(viewport, stable.bytes, 1020, 330 + index * 26, 2),
    });
  }
  assert.ok(unassignedTaskForces.length > 0 && unassignedFighterGroups.length > 0,
    "production participant fixture omitted unassigned group states");
  const unassignedTaskIndex = unassignedTaskForces[0].slot - 1;
  const unassignedTask = point(72 + unassignedTaskIndex * 26, 10);
  await page.mouse.move(unassignedTask.x, unassignedTask.y);
  await page.waitForTimeout(80);
  const hovered = await stableInteractionFrame(page, folder, "task-force-unassigned-hover");
  assert.equal(sha256(hovered), sha256(stable.bytes),
    "unassigned task-force hover altered the original disabled bitmap state");
  const transitionLogCount = consoleLines.filter(({ text }) =>
    text.includes("[tactical_groups]")).length;
  await page.mouse.down({ button: "left" });
  await page.waitForTimeout(80);
  const held = await stableInteractionFrame(page, folder, "task-force-unassigned-held");
  await page.mouse.up({ button: "left" });
  await page.waitForTimeout(80);
  const released = await stableInteractionFrame(page, folder, "task-force-unassigned-released");
  assert.equal(sha256(held), sha256(stable.bytes),
    "unassigned task-force press escaped the disabled bitmap state");
  assert.equal(sha256(released), sha256(stable.bytes),
    "unassigned task-force release changed tactical selection");
  assert.equal(consoleLines.filter(({ text }) => text.includes("[tactical_groups]")).length,
    transitionLogCount, "unassigned task-force input emitted a selection transition");
  probes.push({
    type: "task-force-unassigned-hover-disabled",
    task_force_resource: 1007,
    fighter_group_resource: 1020,
    assigned_task_forces: [...assignedTaskForces].sort(),
    assigned_fighter_groups: [...assignedFighterGroups].sort(),
    task_forces: unassignedTaskForces,
    fighter_groups: unassignedFighterGroups,
    hover_sha256: sha256(hovered),
    held_sha256: sha256(held),
    released_sha256: sha256(released),
    state_unchanged: true,
    transition_log_count: transitionLogCount,
  });

  const participantLine = consoleLines.find(({ text }) =>
    text.includes("[tactical_3d] participant_scene"));
  const positions = parseTacticalScenePositions(participantLine, "capital participant scene");
  const playerObjectId = faction === "alliance" ? 1 : 2;
  const targetObjectId = faction === "alliance" ? 2 : 1;
  const player = positions.get(playerObjectId);
  const target = positions.get(targetObjectId);
  assert.ok(player && target, "production participant interaction positions are incomplete");

  await page.mouse.click(player.x - 80, player.y, { button: "left" });
  await page.waitForTimeout(80);
  const deselected = await stableInteractionFrame(page, folder, "production-deselection");
  await page.mouse.click(player.x, player.y, { button: "left" });
  await page.waitForTimeout(80);
  const selected = await stableInteractionFrame(page, folder, "production-selection");
  await page.mouse.click(target.x, target.y, { button: "right" });
  await page.waitForTimeout(80);
  const targeted = await stableInteractionFrame(page, folder, "production-target");
  assert.notEqual(sha256(deselected), sha256(stable.bytes),
    "projected capital deselection produced no stable framebuffer change");
  assert.equal(sha256(selected), sha256(stable.bytes),
    "projected capital reselection did not restore the deterministic selection frame");
  assert.notEqual(sha256(targeted), sha256(selected),
    "projected capital targeting produced no stable framebuffer change");

  const selectionLog = consoleLines.find(({ text }) =>
    text.includes(`[tactical_3d] selection object_id=${playerObjectId}`));
  const focusLog = consoleLines.find(({ text }) =>
    text.includes(`[tactical_3d] focus source_object_ids=${playerObjectId} `)
      && text.includes(`target_object_id=${targetObjectId}`));
  assert.match(selectionLog?.text || "", /source_projection=true/,
    "selection did not use the source projection");
  assert.match(focusLog?.text || "", /source_projection=true/,
    "focus targeting did not use the source projection");
  probes.push({
    type: "source-projected-tactical-interaction",
    player_object_id: playerObjectId,
    target_object_id: targetObjectId,
    deselected_sha256: sha256(deselected),
    selected_sha256: sha256(selected),
    targeted_sha256: sha256(targeted),
  });
  return probes;
}

function parseFighterSceneResources(line) {
  const match = line?.text.match(/resources=([^ ]+)/);
  assert.ok(match, "fighter scene omitted selected resources");
  return match[1].split(",").map((entry) => {
    const fields = entry.match(/^(\d+):(\d+):(Close|Far|Indicator):([\d.]+)$/);
    assert.ok(fields, `invalid fighter scene resource ${entry}`);
    return {
      object_id: Number(fields[1]),
      resource_id: Number(fields[2]),
      detail: fields[3].toLowerCase(),
      view_span: Number(fields[4]),
    };
  });
}

async function probeTacticalFighterDetailJourney(
  page, viewport, folder, stable, consoleLines, ready,
) {
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const point = (x, y) => ({ x: offsetX + x * scale, y: offsetY + y * scale });
  const zoomIn = point(498, 355);
  const zoomOut = point(615, 355);
  const capture = async (name) => stableInteractionFrame(page, folder, name);
  const clickRepeatedly = async (location, count) => {
    for (let index = 0; index < count; index++) {
      await page.mouse.click(location.x, location.y);
      await page.waitForTimeout(40);
    }
  };

  await clickRepeatedly(zoomIn, 14);
  const far = await capture("fighter-detail-far");
  await clickRepeatedly(zoomIn, 7);
  const close = await capture("fighter-detail-close");
  await clickRepeatedly(zoomOut, 7);
  const returnFar = await capture("fighter-detail-return-far");
  await clickRepeatedly(zoomOut, 11);
  const returnIndicator = await capture("fighter-detail-return-indicator");

  const sceneLines = consoleLines.filter(({ text }) =>
    text.includes("[tactical_3d] fighter_scene"));
  assert.equal(sceneLines.length, 9,
    "fighter detail journey emitted the wrong number of state transitions");
  const states = sceneLines.map(parseFighterSceneResources);
  const details = states.map((state) => state.map(({ detail }) => detail));
  assert.deepEqual(details, [
    ["indicator", "indicator"],
    ["far", "indicator"],
    ["far", "far"],
    ["close", "far"],
    ["close", "close"],
    ["close", "far"],
    ["far", "far"],
    ["far", "indicator"],
    ["indicator", "indicator"],
  ]);
  const fighterParticipants = ready.participants.filter(({ kind }) => kind === "fighter-group");
  const activeResources = new Map(fighterParticipants.map((participant, index) => [1001 + index, {
    close: participant.active_close_resource,
    far: participant.active_far_resource,
    indicator: participant.active_indicator_resource,
  }]));
  for (const state of states) {
    for (const entry of state) {
      assert.equal(entry.resource_id, activeResources.get(entry.object_id)?.[entry.detail],
        `fighter ${entry.object_id} used the wrong ${entry.detail} group resource`);
    }
  }
  for (const state of states) {
    for (const { detail, view_span: viewSpan } of state) {
      if (detail === "close") assert.ok(viewSpan < 5);
      if (detail === "far") assert.ok(viewSpan >= 5 && viewSpan < 10);
      if (detail === "indicator") assert.ok(viewSpan >= 10);
    }
  }

  const frames = [stable.bytes, far, close, returnFar, returnIndicator];
  const apertureHashes = frames.map((bytes) => tacticalApertureHash(viewport, bytes));
  assert.equal(new Set(apertureHashes.slice(0, 3)).size, 3,
    "fighter indicator, far, and close views did not produce distinct framebuffers");
  const transitions = frames.slice(1).map((after, index) =>
    verifyTacticalApertureIsolation(viewport, frames[index], after));
  assert.ok(transitions.every(({ status }) => status === "isolated"),
    "fighter detail journey changed pixels outside the tactical aperture");
  return [{
    type: "production-tactical-fighter-detail-journey",
    controls: ["zoom-in", "zoom-out"],
    executable_functions: ["FUN_005c7150", "FUN_005c63f0", "0x005d4af0"],
    thresholds: { close_below: 5, far_below: 10, indicator_at_or_above: 10 },
    states,
    aperture_hashes: apertureHashes,
    aperture_isolation: transitions,
    family_loads: 2,
  }];
}

async function probeTacticalEffectPresentation(folder, stable, consoleLines, ready) {
  const firstTarget = ready.faction === "alliance" ? 0 : 1;
  const expected = [
    { target: firstTarget, resource_base: 3060, resource_id: 3062, frame: 2,
      frame_count: 6, source_size: [32, 32] },
    { target: firstTarget + 1, resource_base: 3180, resource_id: 3182, frame: 2,
      frame_count: 6, source_size: [32, 32] },
    { target: firstTarget + 2, resource_base: 3120, resource_id: 3123, frame: 3,
      frame_count: 7, source_size: [32, 32] },
    { target: firstTarget + 3, resource_base: 3240, resource_id: 3247, frame: 7,
      frame_count: 16, source_size: [64, 32] },
    { target: firstTarget + 4, resource_base: 3300, resource_id: 3302, frame: 2,
      frame_count: 7, source_size: [32, 32] },
    { target: firstTarget + 5, resource_base: 3360, resource_id: 3368, frame: 8,
      frame_count: 16, source_size: [64, 64] },
  ];
  assert.deepEqual(ready.effects, expected,
    "effect fixture did not expose the six exact source-selected frames");

  const loadLines = consoleLines.filter(({ text }) =>
    text.includes("[tactical_3d] effect_loaded"));
  assert.equal(loadLines.length, expected.length,
    "effect fixture did not load exactly six source frames");
  const loadedResources = loadLines.map(({ text }) => {
    const match = text.match(/resource=(\d+)/);
    assert.ok(match, "effect load omitted its resource identity");
    assert.match(text,
      /palette_selector=\d+ palette_resource_id=\d+ texture_filter=nearest transparent_index=\d+ source=FUN_005d38d0/,
      "effect load omitted its palette, transparency, filter, or source contract");
    return Number(match[1]);
  }).sort((left, right) => left - right);
  assert.deepEqual(loadedResources,
    expected.map(({ resource_id: resourceId }) => resourceId).sort((left, right) => left - right));

  const sceneLines = consoleLines.filter(({ text }) =>
    text.includes("[tactical_3d] effect_scene"));
  assert.equal(sceneLines.length, 1,
    "paused effect fixture emitted the wrong number of scene states");
  assert.match(sceneLines[0].text,
    /requested=6 rendered=6 .* frame_seconds=0\.1 source_positions=true source=FUN_005d39a0,FUN_005d3e90,FUN_005d41a0/,
    "effect scene omitted its target, timing, or executable contract");
  for (const { target, resource_id: resourceId } of expected) {
    assert.match(sceneLines[0].text, new RegExp(`${target + 1}:${resourceId}:\\d+:`),
      `effect ${resourceId} was not attached to source object ${target + 1}`);
  }
  fs.writeFileSync(path.join(folder, "effect-families.png"), stable.bytes);
  return [{
    type: "source-traced-tactical-impact-effects",
    executable_functions: [
      "FUN_005a7500", "FUN_005d39a0", "FUN_005d3e90", "FUN_005d41a0",
    ],
    frame_seconds: 0.1,
    transparency: "type-303-border-index",
    effects: expected,
  }];
}

async function probeTacticalDetailEscortPresentation(
  page, viewport, folder, stable, consoleLines, ready,
) {
  const selected = ready.selected_ship;
  assert.ok(selected, "detail fixture omitted its selected command hull");
  assert.ok(selected.contents.length > 0,
    "selected command hull omitted its assigned personnel contents");
  const content = selected.contents[0];
  const contentProof = verifyGokresBitmap(
    viewport, stable.bytes, content.resource_id, 491, 155,
  );

  const destroyed = ready.effects.find(({ resource_base: resourceBase }) => resourceBase === 3360);
  assert.ok(destroyed, "detail fixture omitted the source destroyed sequence");
  assert.deepEqual(destroyed, {
    target: destroyed.target,
    resource_base: 3360,
    resource_id: 3368,
    frame: 8,
    frame_count: 16,
    source_size: [64, 64],
  });
  assert.equal(ready.participants[destroyed.target]?.active, false,
    "destroyed sequence remained attached to an active hull");
  const effectLine = consoleLines.find(({ text }) => text.includes("[tactical_3d] effect_scene"));
  assert.match(effectLine?.text || "",
    new RegExp(`requested=1 rendered=1 .*${destroyed.target + 1}:3368:`),
    "destroyed source frame was not submitted at its retired hull position");

  const participantLine = consoleLines.find(({ text }) =>
    text.includes("[tactical_3d] participant_scene"));
  const positions = parseTacticalScenePositions(participantLine, "escort participant scene");
  const friendlyObjectIds = ready.participants
    .map((participant, index) => ({ participant, objectId: index + 1 }))
    .filter(({ participant }) => participant.kind === "capital-ship"
      && participant.faction === ready.faction && participant.active)
    .map(({ objectId }) => objectId);
  assert.ok(friendlyObjectIds.length >= 2, "escort fixture omitted its friendly target hull");
  const escortTargetId = friendlyObjectIds[1];
  const escortTarget = positions.get(escortTargetId);
  assert.ok(escortTarget, "escort target omitted its source-projected screen position");
  await page.mouse.click(escortTarget.x, escortTarget.y, { button: "right" });
  await page.waitForTimeout(80);
  const assigned = await stableInteractionFrame(page, folder, "escort-assigned");
  const escortLine = consoleLines.find(({ text }) =>
    text.includes("[tactical_escort] event=assigned order=1")
      && text.includes(`target_object_id=${escortTargetId}`));
  assert.match(escortLine?.text || "", /capital_members=1 .*source_projection=true/,
    "direct friendly right-click did not enter source order code 1");
  assert.notEqual(sha256(assigned), sha256(stable.bytes),
    "Escort assignment produced no stable framebuffer transition");

  const before = PNG.sync.read(stable.bytes);
  const after = PNG.sync.read(assigned);
  const radius = 72 * Math.min(viewport.width / 640, viewport.height / 480);
  let newWhitePixels = 0;
  for (let y = Math.max(0, Math.floor(escortTarget.y - radius));
    y <= Math.min(after.height - 1, Math.ceil(escortTarget.y + radius)); y++) {
    for (let x = Math.max(0, Math.floor(escortTarget.x - radius));
      x <= Math.min(after.width - 1, Math.ceil(escortTarget.x + radius)); x++) {
      const offset = (y * after.width + x) * 4;
      const afterColor = after.data.subarray(offset, offset + 3);
      const beforeColor = before.data.subarray(offset, offset + 3);
      const isWhite = afterColor[0] >= 248 && afterColor[1] >= 248 && afterColor[2] >= 248;
      const wasWhite = beforeColor[0] >= 248 && beforeColor[1] >= 248 && beforeColor[2] >= 248;
      if (isWhite && !wasWhite) newWhitePixels++;
    }
  }
  assert.ok(newWhitePixels >= 8,
    "Escort target did not receive the manual-defined white selection box");

  fs.writeFileSync(path.join(folder, "destroyed-and-contents.png"), stable.bytes);
  return [{
    type: "source-traced-destroyed-capital",
    executable_functions: ["FUN_005d3e90", "FUN_005d41a0"],
    effect: destroyed,
    screenshot_sha256: sha256(stable.bytes),
  }, {
    type: "source-bitmap-selected-hull-contents",
    panel_resource: 1302,
    label: content.label,
    bitmap: contentProof,
  }, {
    type: "source-order-escort",
    executable_functions: ["FUN_005ca6d0", "FUN_005d0af0", "FUN_005cf910"],
    order_code: 1,
    target_object_id: escortTargetId,
    white_box_new_pixels: newWhitePixels,
    screenshot_sha256: sha256(assigned),
  }];
}

async function probeTacticalProjectileFieldPresentation(folder, stable, consoleLines, ready) {
  const expectedProjectiles = [
    { source: 0, target: 3, kind: "laser", longitudinal_scale: 0.5,
      shape_variant: 2, color_selector: 0, elapsed: 0.5, duration: 1, progress: 0.5 },
    { source: 4, target: 1, kind: "turbolaser", longitudinal_scale: 0.65,
      shape_variant: 1, color_selector: 1, elapsed: 0.5, duration: 1, progress: 0.5 },
    { source: 2, target: 5, kind: "ion", longitudinal_scale: 1,
      shape_variant: 3, color_selector: 2, elapsed: 0.5, duration: 2, progress: 0.25 },
  ];
  const expectedFields = [
    {
      target: 1,
      resource_id: 3623,
      frame: 3,
      tractor_sources: 1,
      gravity_sources: 0,
      tractor_source_ids: [3],
      gravity_source_ids: [],
    },
    {
      target: 4,
      resource_id: 3525,
      frame: 5,
      tractor_sources: 1,
      gravity_sources: 1,
      tractor_source_ids: [0],
      gravity_source_ids: [2],
    },
  ];
  assert.deepEqual(ready.projectiles, expectedProjectiles,
    "projectile fixture did not expose the exact source profiles");
  assert.deepEqual(ready.fields, expectedFields,
    "field fixture did not preserve tractor/gravity priority and resources");
  assert.deepEqual(ready.effects, [],
    "projectile/field fixture unexpectedly installed an impact sequence");

  const loadLines = consoleLines.filter(({ text }) =>
    text.includes("[tactical_3d] effect_loaded"));
  assert.equal(loadLines.length, 2,
    "field fixture did not load exactly two source frames");
  const loaded = loadLines.map(({ text }) => {
    const match = text.match(/resource=(\d+)/);
    assert.ok(match, "field load omitted its resource identity");
    assert.match(text,
      /texture_filter=nearest transparent_index=\d+ source=FUN_005d38d0/,
      "field load omitted its transparency, filter, or decoder contract");
    return Number(match[1]);
  }).sort((left, right) => left - right);
  assert.deepEqual(loaded, [3525, 3623]);

  const fieldLines = consoleLines.filter(({ text }) =>
    text.includes("[tactical_3d] field_scene"));
  assert.equal(fieldLines.length, 1,
    "field fixture emitted the wrong number of stable scene states");
  assert.match(fieldLines[0].text,
    /requested=2 rendered=2 .*frame_seconds=0\.1 source_size=128x128 priority=gravity_over_tractor .*source=FUN_005b23e0,FUN_005b24d0,FUN_005b2440,FUN_005b2480,FUN_005d3ac0,FUN_005d3cc0/,
    "field scene omitted its dimensions, timing, priority, or source contract");
  for (const { target, resource_id: resourceId } of expectedFields) {
    assert.match(fieldLines[0].text, new RegExp(`${target + 1}:${resourceId}:\\d+:`),
      `field ${resourceId} was not attached to source object ${target + 1}`);
  }

  const projectileLines = consoleLines.filter(({ text }) =>
    text.includes("[tactical_3d] projectile_scene"));
  assert.equal(projectileLines.length, 1,
    "projectile fixture emitted the wrong number of stable scene states");
  assert.match(projectileLines[0].text,
    /requested=3 rendered=3 .*selectors=0,1,2 durations=1,1,2 interpolation=source_to_live_target geometry=source_mesh raster_compat=device_pixel_floor .*FUN_005ee590,LAB_005eeb90/,
    "projectile scene omitted its shapes, colors, lifecycle, geometry, or interpolation source");
  for (const [index, projectile] of expectedProjectiles.entries()) {
    assert.match(projectileLines[0].text,
      new RegExp(`${index + 2001}:${projectile.shape_variant}:`
        + `${projectile.longitudinal_scale.toFixed(3)}:${projectile.progress.toFixed(3)}:`),
      `projectile ${index} omitted its exact shape, scale, or progress`);
  }
  const projectileCenters = [...projectileLines[0].text.matchAll(
    /(20\d\d):\d+:[\d.]+:[\d.]+:([\d.]+),([\d.]+)/g,
  )].map((match) => ({ object_id: Number(match[1]), x: Number(match[2]), y: Number(match[3]) }));
  assert.equal(projectileCenters.length, expectedProjectiles.length,
    "projectile scene did not expose all framebuffer probe centers");
  const screenshot = PNG.sync.read(stable.bytes);
  const selectorPixelCounts = projectileCenters.map((center, index) => {
    const selector = expectedProjectiles[index].color_selector;
    let matchingPixels = 0;
    for (let y = Math.max(0, Math.round(center.y) - 6);
      y <= Math.min(screenshot.height - 1, Math.round(center.y) + 6); y++) {
      for (let x = Math.max(0, Math.round(center.x) - 6);
        x <= Math.min(screenshot.width - 1, Math.round(center.x) + 6); x++) {
        const offset = (y * screenshot.width + x) * 4;
        const [red, green, blue] = screenshot.data.subarray(offset, offset + 3);
        const matches = selector === 0
          ? red >= 48 && green <= 24 && blue <= 24
          : selector === 1
            ? green >= 48 && red <= 24 && blue <= 24
            : blue >= 48 && red <= 24 && green <= 24;
        if (matches) matchingPixels++;
      }
    }
    assert.ok(matchingPixels > 0,
      `projectile ${center.object_id} produced no selector ${selector} framebuffer pixels`);
    return { object_id: center.object_id, color_selector: selector, matching_pixels: matchingPixels };
  });
  fs.writeFileSync(path.join(folder, "projectiles-and-fields.png"), stable.bytes);
  return [
    {
      type: "source-traced-tactical-projectiles",
      executable_functions: [
        "FUN_005b1ea0", "FUN_005b1f60", "FUN_005b2080", "FUN_005b2c70",
        "FUN_005d3de0", "FUN_005ee590", "LAB_005eeb90",
      ],
      thresholds: { laser: 28.8, turbolaser: 34.666668, ion: 32, torpedo: 12.8 },
      source_vertices: 12,
      shape_variants: [1, 2, 3],
      material_selectors: { alliance: 0, empire: 1, ion: 2 },
      durations: [1, 1, 2],
      raster_compatibility: "one-device-pixel floor for source 0.001-unit faces",
      framebuffer_color_pixels: selectorPixelCounts,
      projectiles: expectedProjectiles,
    },
    {
      type: "source-traced-tactical-fields",
      executable_functions: [
        "FUN_005b23e0", "FUN_005b24d0", "FUN_005b2440", "FUN_005b2480",
        "FUN_005d3ac0", "FUN_005d3cc0",
      ],
      families: { tractor: [3620, 3627], gravity: [3520, 3527] },
      source_size: [128, 128],
      frame_seconds: 0.1,
      priority: "gravity-over-tractor-with-tractor-restoration",
      fields: expectedFields,
    },
  ];
}

function nativeRectPixelCount(viewport, screenshot, rect, matches) {
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const x0 = Math.ceil(offsetX + rect.x * scale);
  const y0 = Math.ceil(offsetY + rect.y * scale);
  const x1 = Math.floor(offsetX + (rect.x + rect.width) * scale);
  const y1 = Math.floor(offsetY + (rect.y + rect.height) * scale);
  let count = 0;
  for (let y = y0; y < y1; y++) {
    for (let x = x0; x < x1; x++) {
      const offset = (y * screenshot.width + x) * 4;
      if (matches(...screenshot.data.subarray(offset, offset + 3))) count++;
    }
  }
  return count;
}

function probeTacticalSelectedDamagePresentation(
  viewport, folder, stable, ready, expectedDamage = null,
  evidence = { filename: "selected-damage.png", type: "source-mapped-selected-capital-damage" },
) {
  const selected = ready.selected_ship;
  assert.ok(selected, "selected-damage fixture omitted its selected ship");
  const expected = ready.faction === "alliance"
    ? { class_dat_id: 64, hud_resource: 2001 }
    : { class_dat_id: 128, hud_resource: 2016 };
  assert.equal(selected.faction, ready.faction);
  assert.equal(selected.class_dat_id, expected.class_dat_id);
  assert.equal(selected.hud_resource, expected.hud_resource);
  assert.ok(selected.hull_max > 0 && selected.shield_max > 0);
  assert.equal(selected.hull_current, expectedDamage?.hull
    ?? Math.max(1, Math.trunc(selected.hull_max * 2 / 5)));
  assert.equal(selected.shield_current, expectedDamage?.shield
    ?? Math.max(0, Math.trunc(selected.shield_max / 4)));

  const screenshot = PNG.sync.read(stable.bytes);
  const shieldPixels = nativeRectPixelCount(
    viewport,
    screenshot,
    { x: 514, y: 107, width: selected.shield_current > 0
      ? 40 * selected.shield_current / selected.shield_max : 40, height: 8 },
    (red, green, blue) => red === 0 && green === 0 && blue === 255,
  );
  const hullPixels = nativeRectPixelCount(
    viewport,
    screenshot,
    { x: 585, y: 107, width: 39 * selected.hull_current / selected.hull_max, height: 8 },
    ready.faction === "alliance"
      ? (red, green, blue) => red === 0 && green >= 48 && blue === 255
      : (red, green, blue) => red === 255 && green === 0 && blue === 0,
  );
  if (selected.shield_current > 0) {
    assert.ok(shieldPixels > 0, "selected ship shield meter produced no exact blue pixels");
  } else {
    assert.equal(shieldPixels, 0, "empty selected ship shield meter retained blue pixels");
  }
  assert.ok(hullPixels > 0, "selected ship hull meter produced no exact faction pixels");

  let panelProof = { status: "non-native-scale", pixels_checked: 0 };
  let portraitProof = { status: "non-native-scale", opaque_pixels_checked: 0 };
  if (viewport.width === 640 && viewport.height === 480 && viewport.device_scale_factor === 1) {
    panelProof = verifyTacticalBitmap(viewport, stable.bytes, 1302, 481, 27, [
      { id: 1101, x: 488, y: 37 },
      { id: 1103, x: 613, y: 37 },
    ], [
      { x: 10, y: 8, width: 130, height: 42 },
      { x: 33, y: 80, width: 40, height: 8 },
      { x: 104, y: 80, width: 39, height: 8 },
      { x: 10, y: 103, width: 130, height: 15 },
      { x: 12, y: 183, width: 130, height: 24 },
      { x: 12, y: 210, width: 126, height: 22 },
    ]);
    const portrait = decodeIndexedBmp(fs.readFileSync(path.join(
      root, `data/base/ui/tactical-dll/BMP/${selected.hud_resource}.bmp`,
    )));
    const panel = decodeIndexedBmp(fs.readFileSync(path.join(
      root, "data/base/ui/tactical-dll/BMP/1302.bmp",
    )));
    let opaquePixels = 0;
    let exactOpaquePixels = 0;
    for (let y = 0; y < portrait.height; y++) {
      for (let x = 0; x < portrait.width; x++) {
        const color = portrait.pixel(x, y);
        if (color[0] < 32 && color[1] > 192 && color[2] < 32) continue;
        opaquePixels++;
        const offset = ((37 + y) * screenshot.width + 507 + x) * 4;
        if (Array.from(screenshot.data.subarray(offset, offset + 3))
          .every((channel, index) => channel === color[index])) exactOpaquePixels++;
      }
    }
    assert.ok(opaquePixels > 0 && exactOpaquePixels > opaquePixels / 2,
      "selected ship portrait did not retain a majority of exact source pixels");
    const matteOffset = (37 * screenshot.width + 507) * 4;
    assert.deepEqual(
      Array.from(screenshot.data.subarray(matteOffset, matteOffset + 3)),
      panel.pixel(26, 10),
      "selected ship lime matte erased the underlying 1302 display grid",
    );
    portraitProof = {
      status: "source-bitmap-composited",
      resource_id: selected.hud_resource,
      opaque_pixels_checked: opaquePixels,
      exact_opaque_pixels: exactOpaquePixels,
      lime_matte_reveals_panel: true,
    };
  }
  fs.writeFileSync(path.join(folder, evidence.filename), stable.bytes);
  return [{
    type: evidence.type,
    source_registry_function: "FUN_00597610",
    panel_resource: 1302,
    portrait_resource: selected.hud_resource,
    shield: { current: selected.shield_current, maximum: selected.shield_max,
      exact_color_pixels: shieldPixels },
    hull: { current: selected.hull_current, maximum: selected.hull_max,
      exact_color_pixels: hullPixels, faction_color: ready.faction },
    panel_proof: panelProof,
    portrait_proof: portraitProof,
  }];
}

function probeTacticalLiveSubsystemDamagePresentation(viewport, folder, stable, ready) {
  const selected = ready.selected_ship;
  assert.ok(selected, "live subsystem fixture omitted its selected ship");
  const probes = probeTacticalSelectedDamagePresentation(
    viewport,
    folder,
    stable,
    ready,
    { hull: selected.hull_max - 11, shield: 0 },
  );
  const expectedLimits = [4, 4, 4, 4, 2];
  const expectedHits = [1, 2, 3, 4, 2];
  const expectedPercentages = [74, 49, 24, 0, 0];
  const expectedResources = [1204, 1208, 1212, 1216, 1221];
  assert.deepEqual(selected.subsystem_hit_limits, expectedLimits);
  assert.deepEqual(selected.subsystem_damage_hits, expectedHits);
  assert.deepEqual(selected.subsystem_percentages, expectedPercentages);
  assert.deepEqual(selected.subsystem_resources, expectedResources);
  const subsystemProofs = expectedResources.map((resourceId, index) =>
    verifyTacticalBitmap(viewport, stable.bytes, resourceId, 491 + index * 27, 130));
  fs.writeFileSync(path.join(folder, "live-subsystem-damage.png"), stable.bytes);
  probes.push({
    type: "source-traced-live-subsystem-damage",
    executable_functions: [
      "FUN_005b54d0", "FUN_005b1970", "FUN_005b1770", "FUN_005b0400",
      "FUN_005b0460", "FUN_005b1790", "FUN_005b17f0", "FUN_005b1bc0",
      "FUN_005b1680", "FUN_005b16b0", "FUN_005b16e0", "FUN_005b1710",
      "FUN_005b1740",
    ],
    hit_limits: expectedLimits,
    damage_hits: expectedHits,
    percentages: expectedPercentages,
    resources: expectedResources,
    subsystem_proofs: subsystemProofs,
    rng_contract: "inclusive-0-through-100-range-with-deterministic-session-bridge",
  });
  return probes;
}

function probeTacticalSubsystemRepairMobilityPresentation(
  viewport, folder, stable, ready, consoleLines,
) {
  const selected = ready.selected_ship;
  assert.ok(selected, "subsystem repair fixture omitted its selected ship");
  const probes = probeTacticalSelectedDamagePresentation(
    viewport,
    folder,
    stable,
    ready,
    { hull: selected.hull_max, shield: selected.shield_current },
  );
  const expectedHits = [1, 2, 1, 1, 1];
  const expectedPercentages = [75, 50, 75, 50, 50];
  const expectedResources = [1205, 1209, 1215, 1219, 1224];
  assert.deepEqual(selected.subsystem_damage_hits, expectedHits);
  assert.deepEqual(selected.subsystem_percentages, expectedPercentages);
  assert.deepEqual(selected.subsystem_resources, expectedResources);
  assert.equal(selected.base_engine_power, 100);
  assert.equal(selected.engine_mode_bonus, 1);
  assert.equal(selected.active_tractor_power, 25);
  assert.equal(selected.effective_engine_power, 50.75);
  assert.equal(selected.damage_control, 100);
  assert.equal(ready.subsystem_repairs.length, 1);
  assert.deepEqual(ready.subsystem_repairs[0], {
    ship: ready.fields[0].target,
    kind: "engines",
    hits_before: 2,
    hits_after: 1,
  });
  assert.equal(ready.fields.length, 1);
  assert.equal(ready.fields[0].tractor_sources, 1);
  assert.equal(ready.fields[0].gravity_sources, 0);
  const fieldLine = consoleLines.find(({ text }) =>
    text.includes("[tactical_3d] field_scene"));
  const fieldPosition = fieldLine?.text.match(
    /resources=\d+:\d+:\d+:(-?[\d.]+),(-?[\d.]+)/,
  );
  assert.ok(fieldPosition, "subsystem repair fixture omitted its projected tractor field");
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const halfField = 64 * scale;
  const fieldCenter = { x: Number(fieldPosition[1]), y: Number(fieldPosition[2]) };
  const aperture = {
    x0: offsetX + 16 * scale,
    y0: offsetY + 28 * scale,
    x1: offsetX + 460 * scale,
    y1: offsetY + 467 * scale,
  };
  assert.ok(fieldCenter.x - halfField >= aperture.x0
    && fieldCenter.x + halfField <= aperture.x1
    && fieldCenter.y - halfField >= aperture.y0
    && fieldCenter.y + halfField <= aperture.y1,
  "tractor field source bounds escape the tactical aperture");
  const subsystemProofs = expectedResources.map((resourceId, index) =>
    verifyTacticalBitmap(viewport, stable.bytes, resourceId, 491 + index * 27, 130));
  fs.writeFileSync(path.join(folder, "subsystem-repair-mobility.png"), stable.bytes);
  probes.push({
    type: "source-traced-subsystem-repair-mobility",
    executable_functions: [
      "FUN_005b0330", "FUN_005b1490", "FUN_005b1ab0", "FUN_005b17f0",
      "FUN_005b16b0", "FUN_005b1790",
    ],
    repair_interval_ms: 50000,
    repair: ready.subsystem_repairs[0],
    damage_hits: expectedHits,
    percentages: expectedPercentages,
    resources: expectedResources,
    base_engine_power: selected.base_engine_power,
    engine_mode_bonus: selected.engine_mode_bonus,
    active_tractor_power: selected.active_tractor_power,
    effective_engine_power: selected.effective_engine_power,
    field_source_bounds_contained: true,
    subsystem_proofs: subsystemProofs,
  });
  return probes;
}

function probeTacticalManeuverMovementPresentation(viewport, folder, stable, ready) {
  const selected = ready.selected_ship;
  assert.ok(selected, "maneuver movement fixture omitted its selected ship");
  const probes = probeTacticalSelectedDamagePresentation(
    viewport,
    folder,
    stable,
    ready,
    { hull: selected.hull_max, shield: selected.shield_max },
  );
  const alliance = ready.faction === "alliance";
  const initialZ = alliance ? -56 : 56;
  const directionZ = alliance ? 1 : -1;
  const expectedZ = initialZ + directionZ * 78.75 * 0.25;
  assert.equal(selected.base_engine_power, 100);
  assert.equal(selected.maneuver_state_value, 4);
  assert.equal(selected.engine_mode_bonus, 5);
  assert.ok(Math.abs(selected.active_tractor_power) === 0);
  assert.equal(selected.effective_engine_power, 78.75);
  assert.equal(selected.movement_alignment, 1);
  assert.ok(Math.abs(selected.source_forward[0]) === 0
    && Math.abs(selected.source_forward[1]) === 0
    && selected.source_forward[2] === directionZ);
  assert.ok(Math.abs(selected.source_desired_forward[0]) === 0
    && Math.abs(selected.source_desired_forward[1]) === 0
    && selected.source_desired_forward[2] === directionZ);
  assert.ok(Math.abs(selected.source_velocity[0]) === 0
    && Math.abs(selected.source_velocity[1]) === 0
    && selected.source_velocity[2] === directionZ * 78.75);
  assert.ok(Math.abs(selected.source_position[0]) === 0
    && selected.source_position[1] === 0
    && selected.source_position[2] === expectedZ);
  const selectedParticipant = ready.participants.find((participant) =>
    participant.kind === "capital-ship"
      && participant.faction === ready.faction
      && participant.fleet_roster_index === 0);
  assert.ok(selectedParticipant, "maneuver movement fixture omitted its selected participant");
  assert.deepEqual(selectedParticipant.source_position, selected.source_position,
    "render participant did not consume the integrated source position");
  fs.writeFileSync(path.join(folder, "maneuver-movement.png"), stable.bytes);
  probes.push({
    type: "source-traced-tactical-maneuver-movement",
    executable_functions: [
      "FUN_005ad750", "FUN_005afb70", "FUN_005b17f0", "FUN_005b2f30", "FUN_005cd640",
    ],
    maneuver_state_value: selected.maneuver_state_value,
    engine_mode_bonus: selected.engine_mode_bonus,
    effective_engine_power: selected.effective_engine_power,
    alignment: selected.movement_alignment,
    delta_milliseconds: 250,
    initial_position: [-0, 0, initialZ],
    velocity: selected.source_velocity,
    integrated_position: selected.source_position,
    rendered_participant_position: selectedParticipant.source_position,
  });
  return probes;
}

async function probeTacticalCommandAssignmentPresentation(
  page, viewport, folder, stable, consoleLines, ready,
) {
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const point = (x, y) => ({ x: offsetX + x * scale, y: offsetY + y * scale });
  const click = async (x, y) => {
    const target = point(x, y);
    await page.mouse.click(target.x, target.y);
    await page.waitForTimeout(80);
  };
  const capture = async (name) => stableInteractionFrame(page, folder, name);
  const probes = [
    {
      type: "missions-button-initial",
      ...verifyTacticalBitmap(viewport, stable.bytes, 1107, 493, 237),
    },
    {
      type: "maneuvers-button-initial",
      ...verifyTacticalBitmap(viewport, stable.bytes, 1105, 561, 237),
    },
  ];

  await click(590, 248);
  const maneuversOpen = await capture("maneuvers-open");
  probes.push({ type: "maneuvers-panel", ...verifyTacticalBitmap(
    viewport, maneuversOpen, 1309, 481, 27, [], [
      { x: 14, y: 14, width: 126, height: 108 },
      { x: 13, y: 129, width: 58, height: 58 },
      { x: 75, y: 144, width: 28, height: 28 },
      { x: 111, y: 132, width: 24, height: 52 },
      { x: 66, y: 202, width: 67, height: 25 },
    ],
  ) });
  probes.push({ type: "stand-off-default", ...verifyTacticalBitmap(
    viewport, maneuversOpen, 1117, 592, 159,
  ) });

  await click(508, 170);
  await click(604, 185);
  const hammerSurround = await capture("maneuvers-hammer-surround");
  probes.push({ type: "hammer-selected", ...verifyTacticalBitmap(
    viewport, hammerSurround, 1138, 494, 156,
  ) });
  probes.push({ type: "surround-selected", ...verifyTacticalBitmap(
    viewport, hammerSurround, 1118, 592, 159,
  ) });
  probes.push({ type: "hammer-graphic", ...verifyTacticalTransparentOverlay(
    viewport, hammerSurround, 2103, 495, 41,
  ) });
  await click(600, 241);
  const afterCancel = await capture("maneuvers-cancelled");
  probes.push({ type: "cancel-restores-display", ...verifyTacticalBitmap(
    viewport, afterCancel, 1105, 561, 237,
  ) });

  await click(590, 248);
  await click(508, 170);
  await click(604, 185);
  await click(560, 241);
  const maneuverCommitted = await capture("maneuvers-committed");
  probes.push({ type: "maneuver-commit-restores-display", ...verifyTacticalBitmap(
    viewport, maneuverCommitted, 1105, 561, 237,
  ) });

  await click(590, 248);
  await click(538, 170);
  await click(604, 185);
  const anvilStandOff = await capture("maneuvers-anvil-stand-off");
  probes.push({ type: "anvil-selected", ...verifyTacticalBitmap(
    viewport, anvilStandOff, 1140, 524, 156,
  ) });
  probes.push({ type: "stand-off-selected", ...verifyTacticalBitmap(
    viewport, anvilStandOff, 1117, 592, 159,
  ) });
  probes.push({ type: "anvil-graphic", ...verifyTacticalTransparentOverlay(
    viewport, anvilStandOff, 2104, 495, 41,
  ) });
  await click(560, 241);
  await capture("maneuvers-anvil-stand-off-committed");

  await click(520, 248);
  const missionsOpen = await capture("missions-open-capital");
  probes.push({ type: "missions-panel", ...verifyTacticalBitmap(
    viewport, missionsOpen, 1308, 481, 27, [], [
      { x: 14, y: 14, width: 124, height: 106 },
      { x: 18, y: 129, width: 114, height: 56 },
      { x: 66, y: 202, width: 67, height: 25 },
    ],
  ) });
  probes.push({ type: "capital-recover-disabled", ...verifyTacticalBitmap(
    viewport, missionsOpen, ready.faction === "alliance" ? 1175 : 1172, 567, 156,
  ) });
  probes.push({ type: "death-star-disabled", ...verifyTacticalBitmap(
    viewport, missionsOpen, 1178, 567, 186,
  ) });
  await click(522, 169);
  const capitalMission = await capture("missions-attack-capital-selected");
  const capitalButton = ready.faction === "alliance" ? 1180 : 1183;
  const capitalGraphic = ready.faction === "alliance" ? 2151 : 2155;
  probes.push({ type: "attack-capital-selected", ...verifyTacticalBitmap(
    viewport, capitalMission, capitalButton, 499, 156,
  ) });
  probes.push({ type: "attack-capital-graphic", ...verifyTacticalTransparentOverlay(
    viewport, capitalMission, capitalGraphic, 495, 41,
  ) });
  await click(560, 241);

  await click(342, 10);
  const fighterSelected = await capture("fighter-group-selected");
  probes.push({ type: "fighter-group-selected", ...verifyTacticalBitmap(
    viewport, fighterSelected, 1016, 330, 2,
  ) });
  await click(520, 248);
  const fighterMissions = await capture("missions-open-fighter");
  probes.push({ type: "fighter-recover-enabled", ...verifyTacticalBitmap(
    viewport, fighterMissions, ready.faction === "alliance" ? 1173 : 1170, 567, 156,
  ) });
  await click(590, 169);
  const recoverSelected = await capture("missions-recover-selected");
  probes.push({ type: "recover-selected", ...verifyTacticalBitmap(
    viewport, recoverSelected, ready.faction === "alliance" ? 1174 : 1171, 567, 156,
  ) });
  probes.push({ type: "recover-graphic", ...verifyTacticalTransparentOverlay(
    viewport, recoverSelected, ready.faction === "alliance" ? 2153 : 2157, 495, 41,
  ) });
  await click(560, 241);
  await capture("fighter-recover-committed");

  const orderLogs = consoleLines
    .filter(({ text }) => text.includes("[tactical_orders]"))
    .map(({ text }) => text);
  assert.ok(orderLogs.some((line) => line.includes("event=cancel")),
    "maneuver cancel did not emit its deterministic transition");
  assert.ok(orderLogs.some((line) =>
    /panel=maneuvers event=commit order=9 tactic=1 capital_members=1 fighter_members=0/.test(line)),
  "maneuver assignment did not commit Hammer and Surround to the selected capital");
  assert.ok(orderLogs.some((line) =>
    /panel=maneuvers event=commit order=10 tactic=2 capital_members=1 fighter_members=0/.test(line)),
  "maneuver assignment did not commit Anvil and Stand Off to the selected capital");
  assert.ok(orderLogs.some((line) =>
    /panel=missions event=commit order=5 tactic=0 capital_members=1 fighter_members=0/.test(line)),
  "capital mission did not commit Attack Capital Ships");
  assert.ok(orderLogs.some((line) =>
    /panel=missions event=commit order=2 tactic=0 capital_members=0 fighter_members=2/.test(line)),
  "fighter mission did not commit Recover");
  const expectedCapacityVoice = ready.faction === "alliance"
    ? { event: "0x8b", wave: 14108, owner: "Alliance" }
    : { event: "0x109", wave: 15112, owner: "Empire" };
  const capacityVoice = consoleLines.find(({ text }) =>
    text.includes("voice_event=FighterRecoveryCapacity")
      && text.includes(`source_event=${expectedCapacityVoice.event}`));
  assert.match(capacityVoice?.text || "",
    new RegExp(`wave=${expectedCapacityVoice.wave} faction=${expectedCapacityVoice.owner} routed=true loaded=true muted=true`),
    "fighter Recover did not route the exact muted no-capacity warning");
  probes.push({
    type: "source-traced-tactical-command-assignment",
    executable_functions: [
      "FUN_005dcc70", "FUN_005dd080", "FUN_005dd140", "FUN_005dd290", "FUN_005dd3a0",
      "FUN_005def00", "FUN_005df110", "FUN_005df4f0", "FUN_005df5f0", "FUN_005df660",
    ],
    order_codes: {
      recover: 2,
      attack_fighters: 4,
      attack_capital_ships: 5,
      attack_death_star: 6,
      left_hook: 7,
      right_hook: 8,
      hammer: 9,
      anvil: 10,
      hold_position: 11,
    },
    tactic_codes: { surround: 1, stand_off: 2 },
    logs: orderLogs,
    capacity_voice_log: capacityVoice.text,
  });
  return probes;
}

function probeTacticalCommandExecutionPresentation(viewport, folder, stable, ready) {
  const selected = ready.selected_ship;
  assert.ok(selected, "command execution fixture omitted its selected capital");
  assert.equal(selected.order_code, 7, "selected capital did not retain Left Hook");
  assert.equal(selected.tactic_code, 1, "selected capital did not retain Surround");
  assert.ok(Array.isArray(selected.source_waypoint)
    && selected.source_waypoint.length === 3
    && selected.source_waypoint.every(Number.isFinite),
  "Left Hook did not create a finite source waypoint");
  const desiredMagnitude = Math.hypot(...selected.source_desired_forward);
  assert.ok(Math.abs(desiredMagnitude - 1) < 0.00001,
    "Left Hook desired direction is not normalized");

  const selectedParticipant = ready.participants.find((participant) =>
    participant.kind === "capital-ship"
      && participant.faction === ready.faction
      && participant.fleet_roster_index === 0);
  assert.ok(selectedParticipant, "command execution fixture omitted its selected participant");
  assert.equal(selectedParticipant.order_code, 7);
  assert.equal(selectedParticipant.tactic_code, 1);
  assert.deepEqual(selectedParticipant.source_waypoint, selected.source_waypoint);
  assert.deepEqual(selectedParticipant.source_desired_forward,
    selected.source_desired_forward);

  const held = ready.participants.find((participant) =>
    participant.kind === "capital-ship"
      && participant.faction === ready.faction
      && participant.order_code === 11);
  assert.ok(held, "command execution fixture omitted its held capital");
  assert.equal(held.source_waypoint, null);
  assert.deepEqual(held.source_desired_forward, [0, 0, 0]);

  const returning = ready.participants.find((participant) =>
    participant.kind === "fighter-group"
      && participant.faction === ready.faction
      && participant.order_code === 2);
  assert.ok(returning, "command execution fixture omitted its recovering fighter group");
  assert.equal(returning.recovery_state_code, 2);
  assert.ok(Number.isSafeInteger(returning.recovery_target)
    && returning.recovery_target >= 0,
  "Recover did not reserve a carrier");
  const carrier = ready.participants[returning.recovery_target];
  assert.ok(carrier?.kind === "capital-ship" && carrier.faction === ready.faction,
    "Recover reserved an incompatible carrier");

  fs.writeFileSync(path.join(folder, "command-execution.png"), stable.bytes);
  return [{
    type: "source-traced-tactical-command-execution",
    executable_functions: [
      "FUN_005ca6d0", "FUN_005cef00", "FUN_005cf1a0", "FUN_005cf410",
      "FUN_005cf680", "FUN_005cf930", "FUN_005cf940", "FUN_005cf980",
      "FUN_005d21b0", "FUN_005cda40", "FUN_005a9530", "FUN_005a8f70",
    ],
    maneuver: {
      order_code: selected.order_code,
      tactic_code: selected.tactic_code,
      waypoint_scale: 0.75,
      angle_radians: Math.PI / 8,
      waypoint: selected.source_waypoint,
      desired_forward: selected.source_desired_forward,
    },
    hold: {
      order_code: held.order_code,
      waypoint: held.source_waypoint,
      desired_forward: held.source_desired_forward,
    },
    recover: {
      order_code: returning.order_code,
      state_code: returning.recovery_state_code,
      carrier: returning.recovery_target,
    },
  }];
}

function probeTacticalCommandProgressionPresentation(
  viewport, folder, stable, consoleLines, ready,
) {
  const selected = ready.selected_ship;
  assert.ok(selected, "command progression fixture omitted its turning capital");
  assert.equal(selected.order_code, 7);
  assert.equal(selected.maneuverability, 4);
  const expectedTurn = 5 * Math.PI / 72;
  const forwardSign = ready.faction === "alliance" ? 1 : -1;
  assert.ok(Math.abs(selected.source_forward[0] - Math.sin(expectedTurn)) < 0.00001,
    "turning capital did not apply the executable-derived X component");
  assert.ok(Math.abs(selected.source_forward[1]) < 0.00001);
  assert.ok(Math.abs(selected.source_forward[2] - forwardSign * Math.cos(expectedTurn)) < 0.00001,
    "turning capital did not apply the executable-derived Z component");
  assert.ok(Math.abs(Math.hypot(...selected.source_forward) - 1) < 0.00001,
    "turning capital direction is not normalized");

  const playerCapitals = ready.participants.filter((participant) =>
    participant.kind === "capital-ship" && participant.faction === ready.faction);
  const arrived = playerCapitals.find((participant) =>
    participant.fleet_roster_index === 1);
  assert.ok(arrived, "command progression fixture omitted its arriving capital");
  assert.equal(arrived.order_code, 0);
  assert.equal(arrived.source_waypoint, null);
  assert.deepEqual(arrived.source_desired_forward, [0, 0, 0]);
  const expectedArrivalZ = (ready.faction === "alliance"
    ? ready.source_layout.outer_negative_z
    : ready.source_layout.outer_positive_z) + forwardSign;
  assert.equal(arrived.source_position[2], expectedArrivalZ);

  const playerFighters = ready.participants.filter((participant) =>
    participant.kind === "fighter-group" && participant.faction === ready.faction);
  assert.equal(playerFighters.length, 2);
  const docking = playerFighters.find((participant) => participant.recovery_state_code === 3);
  const recovered = playerFighters.find((participant) => participant.recovery_state_code === 4);
  assert.ok(docking, "returning fighter did not enter the source docking state");
  assert.equal(docking.active, true);
  assert.equal(docking.order_code, 2);
  assert.equal(docking.recovery_target, ready.faction === "alliance" ? 1 : 4);
  assert.equal(Math.abs(docking.source_position[2] - arrived.source_position[2]), 1.5);
  assert.ok(recovered, "docking callback did not enter the source recovered state");
  assert.equal(recovered.active, false);
  assert.equal(recovered.order_code, 0);
  assert.equal(recovered.recovery_target, docking.recovery_target);
  assert.deepEqual(recovered.source_position, arrived.source_position);
  assert.equal(recovered.fighter_group, 1);
  const expectedRecoveryVoice = ready.faction === "alliance"
    ? { event: "0x88", wave: 14105, owner: "Alliance" }
    : { event: "0x106", wave: 15109, owner: "Empire" };
  const recoveryVoice = consoleLines.find(({ text }) =>
    text.includes("voice_event=FighterRecoveryComplete")
      && text.includes(`source_event=${expectedRecoveryVoice.event}`));
  assert.match(recoveryVoice?.text || "",
    new RegExp(`wave=${expectedRecoveryVoice.wave} faction=${expectedRecoveryVoice.owner} routed=true loaded=true muted=true`),
    "fighter recovery did not route the exact muted group-completion voice");

  fs.writeFileSync(path.join(folder, "command-progression.png"), stable.bytes);
  return [{
    type: "source-traced-tactical-command-progression",
    executable_functions: [
      "FUN_005b9c60", "FUN_005ba270", "FUN_005cd2d0", "FUN_005cd460",
      "FUN_005cd640", "FUN_005cf190", "FUN_005cf8f0", "FUN_005cf980",
      "FUN_005b1e10", "FUN_005b84e0", "FUN_005b8630",
    ],
    turn: {
      raw_maneuverability: selected.maneuverability,
      radians: expectedTurn,
      source_forward: selected.source_forward,
    },
    arrival: {
      roster: arrived.fleet_roster_index,
      position: arrived.source_position,
      order_code: arrived.order_code,
      waypoint: arrived.source_waypoint,
    },
    recover: {
      docking_state_code: docking.recovery_state_code,
      recovered_state_code: recovered.recovery_state_code,
      carrier: docking.recovery_target,
      docking_distance: 2,
      strategic_count_preserved: true,
      completion_voice_log: recoveryVoice.text,
    },
  }];
}

function probeTacticalAttackTargetingPresentation(viewport, folder, stable, ready) {
  const playerCapitals = ready.participants.filter((participant) =>
    participant.kind === "capital-ship" && participant.faction === ready.faction);
  const playerFighters = ready.participants.filter((participant) =>
    participant.kind === "fighter-group" && participant.faction === ready.faction);
  const enemyCapitalIndex = ready.participants.findIndex((participant) =>
    participant.kind === "capital-ship" && participant.faction !== ready.faction);
  const enemyFighterIndex = ready.participants
    .filter((participant) => participant.kind === "fighter-group")
    .findIndex((participant) => participant.faction !== ready.faction);
  assert.ok(enemyCapitalIndex >= 0, "attack fixture omitted its hostile capital target");
  assert.ok(enemyFighterIndex >= 0, "attack fixture omitted its hostile fighter target");

  const capitalToCapital = playerCapitals.find((participant) =>
    participant.fleet_roster_index === 0);
  const capitalToFighter = playerCapitals.find((participant) =>
    participant.fleet_roster_index === 1);
  const fighterToCapital = playerFighters.find((participant) =>
    participant.fleet_roster_index === 0);
  const fighterToFighter = playerFighters.find((participant) =>
    participant.fleet_roster_index === 1);
  assert.deepEqual(
    [capitalToCapital?.order_code, capitalToCapital?.attack_target_kind,
      capitalToCapital?.attack_target_index],
    [5, "capital-ship", enemyCapitalIndex],
  );
  assert.deepEqual(
    [capitalToFighter?.order_code, capitalToFighter?.attack_target_kind,
      capitalToFighter?.attack_target_index],
    [4, "fighter-group", enemyFighterIndex],
  );
  assert.deepEqual(
    [fighterToCapital?.order_code, fighterToCapital?.attack_target_kind,
      fighterToCapital?.attack_target_index],
    [5, "capital-ship", enemyCapitalIndex],
  );
  assert.deepEqual(
    [fighterToFighter?.order_code, fighterToFighter?.attack_target_kind,
      fighterToFighter?.attack_target_index],
    [4, "fighter-group", enemyFighterIndex],
  );
  assert.equal(ready.selected_ship?.order_code, 4);

  fs.writeFileSync(path.join(folder, "attack-targeting.png"), stable.bytes);
  return [{
    type: "source-traced-tactical-attack-targeting",
    executable_functions: [
      "FUN_005ca6d0", "FUN_005d0b10", "FUN_005d0bb0", "FUN_005c91e0",
      "FUN_005c91c0", "FUN_005f5060", "FUN_005a8fc0",
    ],
    object_type_codes: { "capital-ship": 0, "fighter-group": 1 },
    source_event: "0x36",
    assignments: [capitalToCapital, capitalToFighter, fighterToCapital, fighterToFighter]
      .map((participant) => ({
        owner_kind: participant.kind,
        owner_roster: participant.fleet_roster_index,
        order_code: participant.order_code,
        target_kind: participant.attack_target_kind,
        target_index: participant.attack_target_index,
      })),
  }];
}

function probeTacticalAttackTargetLifecyclePresentation(viewport, folder, stable, ready) {
  const playerCapitals = ready.participants.filter((participant) =>
    participant.kind === "capital-ship" && participant.faction === ready.faction);
  const playerFighters = ready.participants.filter((participant) =>
    participant.kind === "fighter-group" && participant.faction === ready.faction);
  const enemyCapitalIndices = ready.participants
    .map((participant, index) => ({ participant, index }))
    .filter(({ participant }) =>
      participant.kind === "capital-ship" && participant.faction !== ready.faction);
  const allFighters = ready.participants.filter((participant) =>
    participant.kind === "fighter-group");
  const enemyFighterIndices = allFighters
    .map((participant, index) => ({ participant, index }))
    .filter(({ participant }) => participant.faction !== ready.faction);
  assert.ok(enemyCapitalIndices.length >= 2,
    "target lifecycle fixture omitted its capital replacement");
  assert.ok(enemyFighterIndices.length >= 2,
    "target lifecycle fixture omitted its fighter replacement");

  const retiredCapital = enemyCapitalIndices[0];
  const replacementCapital = enemyCapitalIndices[1];
  const retiredFighter = enemyFighterIndices[0];
  const replacementFighter = enemyFighterIndices[1];
  assert.equal(retiredCapital.participant.retreating, true);
  assert.equal(replacementCapital.participant.retreating, false);
  assert.equal(retiredFighter.participant.recovery_state_code, 4);
  assert.notEqual(replacementFighter.participant.recovery_state_code, 4);

  const capitalToCapital = playerCapitals.find((participant) =>
    participant.fleet_roster_index === 0);
  const capitalToFighter = playerCapitals.find((participant) =>
    participant.fleet_roster_index === 1);
  const fighterToCapital = playerFighters.find((participant) =>
    participant.fleet_roster_index === 0);
  const fighterToFighter = playerFighters.find((participant) =>
    participant.fleet_roster_index === 1);
  assert.deepEqual(
    [capitalToCapital?.order_code, capitalToCapital?.attack_target_kind,
      capitalToCapital?.attack_target_index],
    [5, "capital-ship", replacementCapital.index],
  );
  assert.deepEqual(
    [capitalToFighter?.order_code, capitalToFighter?.attack_target_kind,
      capitalToFighter?.attack_target_index],
    [4, "fighter-group", replacementFighter.index],
  );
  assert.deepEqual(
    [fighterToCapital?.order_code, fighterToCapital?.attack_target_kind,
      fighterToCapital?.attack_target_index],
    [5, "capital-ship", replacementCapital.index],
  );
  assert.deepEqual(
    [fighterToFighter?.order_code, fighterToFighter?.attack_target_kind,
      fighterToFighter?.attack_target_index],
    [4, "fighter-group", replacementFighter.index],
  );

  fs.writeFileSync(path.join(folder, "attack-target-lifecycle.png"), stable.bytes);
  return [{
    type: "source-traced-tactical-attack-target-lifecycle",
    executable_functions: [
      "FUN_005a8c50", "FUN_005a8cc0", "FUN_005a8d30", "FUN_005a8e30",
      "FUN_005a8e80", "FUN_005a8fc0", "FUN_005d0b00", "FUN_005cf910",
      "FUN_005cf920",
    ],
    invalidated: {
      capital_index: retiredCapital.index,
      fighter_index: retiredFighter.index,
    },
    replacements: [capitalToCapital, capitalToFighter, fighterToCapital, fighterToFighter]
      .map((participant) => ({
        owner_kind: participant.kind,
        owner_roster: participant.fleet_roster_index,
        order_code: participant.order_code,
        target_kind: participant.attack_target_kind,
        target_index: participant.attack_target_index,
      })),
  }];
}

async function probeTacticalDeathStarPresentation(
  page, viewport, folder, stable, consoleLines, ready,
) {
  const deathStar = ready.death_star;
  assert.ok(deathStar, "Death Star fixture omitted its manager-owned object");
  assert.deepEqual({
    faction: deathStar.faction,
    is_attacker: deathStar.is_attacker,
    tactical_ordinal: deathStar.tactical_ordinal,
    resource_id: deathStar.resource_id,
    opposing_resource_id: deathStar.opposing_resource_id,
    operational: deathStar.operational,
    hull: deathStar.hull,
    laser_charge: deathStar.laser_charge,
    destroyed: deathStar.destroyed,
    action_committed: deathStar.action_committed,
  }, {
    faction: "empire",
    is_attacker: false,
    tactical_ordinal: 37,
    resource_id: 5010,
    opposing_resource_id: 5020,
    operational: true,
    hull: 100,
    laser_charge: 0,
    destroyed: false,
    action_committed: false,
  });
  assert.deepEqual(deathStar.source_position,
    [0, 0, ready.source_layout.outer_positive_z]);

  const effectLoad = consoleLines.find(({ text }) =>
    text.includes("[tactical_3d] effect_loaded resource=5010"));
  assert.ok(effectLoad, "Death Star resource 5010 was not decoded into the tactical renderer");
  const effectScene = consoleLines.find(({ text }) =>
    text.includes("[tactical_3d] effect_scene") && text.includes("3001:5010:"));
  assert.ok(effectScene, "Death Star resource 5010 was not submitted as object 3001");
  fs.writeFileSync(path.join(folder, "death-star-overview.png"), stable.bytes);

  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const click = async (x, y) => {
    await page.mouse.click(offsetX + x * scale, offsetY + y * scale);
    await page.waitForTimeout(80);
  };
  const capture = async (name) => stableInteractionFrame(page, folder, name);

  await click(342, 10);
  await click(520, 248);
  const missions = await capture("death-star-missions-open");
  const enabled = ready.faction === "alliance";
  const probes = [{
    type: enabled ? "attack-death-star-enabled" : "attack-death-star-disabled",
    ...verifyTacticalBitmap(viewport, missions, enabled ? 1176 : 1178, 567, 186),
  }];

  await click(590, 199);
  const afterAttack = await capture(enabled
    ? "death-star-mission-selected"
    : "death-star-disabled-probe");
  if (enabled) {
    probes.push({
      type: "attack-death-star-selected",
      ...verifyTacticalBitmap(viewport, afterAttack, 1177, 567, 186),
    });
    probes.push({
      type: "attack-death-star-graphic",
      ...verifyTacticalTransparentOverlay(viewport, afterAttack, 2154, 495, 41),
    });
    await click(560, 241);
  } else {
    assert.equal(sha256(afterAttack), sha256(missions),
      "friendly Death Star accepted the disabled attack control");
  }

  const orderLogs = consoleLines
    .filter(({ text }) => text.includes("[tactical_orders]"))
    .map(({ text }) => text);
  if (enabled) {
    assert.ok(orderLogs.some((line) =>
      /panel=missions event=commit order=6 tactic=0 capital_members=0 fighter_members=1/.test(line)),
    "Alliance fighter group did not commit Attack Death Star");
    assert.ok(consoleLines.some(({ text }) =>
      text.includes("[tactical_death_star] trench_run_launch status=launched fighter_members=1")),
    "Alliance fighter group did not enter the production trench-run lifecycle");
  } else {
    assert.ok(!orderLogs.some((line) => /order=6/.test(line)),
      "Imperial fighter group committed an attack against its friendly Death Star");
  }
  probes.push({
    type: "source-traced-tactical-death-star-manager",
    executable_functions: [
      "FUN_005ab0e0", "FUN_005ab650", "FUN_005ba420", "FUN_005ba7f0", "FUN_005df110",
    ],
    object_id: 3001,
    manager: deathStar,
    attack_enabled: enabled,
    order_logs: orderLogs,
  });
  return probes;
}

async function probeTacticalDeathStarLaserJourney(
  page, viewport, folder, stable, consoleLines, ready,
) {
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const point = (x, y) => ({ x: offsetX + x * scale, y: offsetY + y * scale });
  const probes = [{
    type: "source-tactical-star-surface",
    ...verifyTacticalBackdrop(viewport, stable.bytes, ready.palette_resource_id),
  }];
  assert.ok(consoleLines.some(({ text }) =>
    text.includes("[tactical_3d] backdrop_loaded resource=5030 dimensions=440x438")),
  "resource 5030 was not decoded as the tactical star surface");
  assert.ok(consoleLines.some(({ text }) =>
    text.includes("[tactical_3d] backdrop_scene resource=5030 dimensions=440x438")),
  "resource 5030 was not submitted behind the tactical scene");

  const button = point(623, 12);
  if (ready.faction === "alliance") {
    await page.mouse.click(button.x, button.y);
    await page.waitForTimeout(80);
    const rejected = await stableInteractionFrame(page, folder, "enemy-death-star-control-absent");
    assert.equal(sha256(rejected), sha256(stable.bytes),
      "Alliance player activated the enemy Death Star control region");
    assert.ok(!consoleLines.some(({ text }) => text.includes("superlaser_targeting status=armed")),
      "Alliance player armed the enemy Death Star superlaser");
    probes.push({
      type: "enemy-death-star-control-absent",
      faction_gate: "empire-owner-only",
      unchanged_sha256: sha256(rejected),
    });
    return probes;
  }

  probes.push({
    type: "death-star-laser-gauge-ready",
    ...verifyTacticalBitmap(viewport, stable.bytes, 1024, 474, 0, [], [
      { x: 0, y: 0, width: 166, height: 1 },
      { x: 0, y: 0, width: 1, height: 25 },
      { x: 163, y: 0, width: 3, height: 25 },
      { x: 34, y: 9, width: 95, height: 6 },
      { x: 139, y: 2, width: 21, height: 20 },
    ]),
  }, {
    type: "death-star-laser-ready",
    ...verifyTacticalBitmap(viewport, stable.bytes, 1021, 613, 2),
  });

  await page.mouse.move(button.x, button.y);
  await page.mouse.down({ button: "left" });
  await page.waitForTimeout(80);
  const held = await stableInteractionFrame(page, folder, "death-star-laser-held");
  probes.push({
    type: "death-star-laser-held",
    ...verifyTacticalBitmap(viewport, held, 1022, 613, 2),
  });
  await page.mouse.up({ button: "left" });
  await page.waitForTimeout(80);
  const armed = await stableInteractionFrame(page, folder, "death-star-laser-armed");
  probes.push({
    type: "death-star-laser-armed",
    ...verifyTacticalBitmap(viewport, armed, 1022, 613, 2),
  });
  assert.ok(consoleLines.some(({ text }) =>
    text.includes("[tactical_death_star] superlaser_targeting status=armed input=right_click")),
  "Death Star fire control did not arm right-click targeting");

  const participantLine = consoleLines.find(({ text }) =>
    text.includes("[tactical_3d] participant_scene"));
  const positions = parseTacticalScenePositions(participantLine, "Death Star target scene");
  const enemyIndex = ready.participants.findIndex(({ kind, faction }) =>
    kind === "capital-ship" && faction === "alliance");
  assert.ok(enemyIndex >= 0, "Death Star journey lacks an Alliance capital target");
  const targetObjectId = enemyIndex + 1;
  const target = positions.get(targetObjectId);
  assert.ok(target, `Death Star target object ${targetObjectId} lacks a projected position`);
  await page.mouse.click(target.x, target.y, { button: "right" });
  await page.waitForTimeout(80);
  const committed = await stableInteractionFrame(page, folder, "death-star-laser-committed");
  probes.push({
    type: "death-star-laser-committed",
    target_object_id: targetObjectId,
    ...verifyTacticalBitmap(viewport, committed, 1022, 613, 2),
  });
  assert.ok(consoleLines.some(({ text }) =>
    text.includes(`[tactical_death_star] superlaser_committed target_object_id=${targetObjectId} charge=0`)),
  "Death Star right-click target did not commit the superlaser shot");

  const pause = point(574, 318);
  await page.mouse.click(pause.x, pause.y);
  await page.waitForTimeout(1100);
  const beam = await dynamicInteractionFrame(page, folder, "death-star-superlaser-beam");
  const beamVisibility = verifyTacticalGreenBeam(viewport, committed, beam);
  assert.ok(consoleLines.some(({ text }) =>
    text.includes("[tactical_3d] projectile_scene") && text.includes("3999:3:12.000")),
  "Death Star beam was not submitted to the production tactical renderer");
  await page.waitForTimeout(1100);
  assert.ok(consoleLines.some(({ text }) =>
    text.includes(`[tactical_death_star] superlaser_resolved target_object_id=${targetObjectId}`)),
  "Death Star beam did not resolve against its assigned target");
  await page.mouse.click(pause.x, pause.y);
  await page.waitForTimeout(80);
  const resolved = await stableInteractionFrame(page, folder, "death-star-superlaser-resolved");
  probes.push({
    type: "death-star-superlaser-resolution",
    target_object_id: targetObjectId,
    beam_frame_sha256: sha256(beam),
    beam_visibility: beamVisibility,
    resolved_frame_sha256: sha256(resolved),
    control: verifyTacticalBitmap(viewport, resolved, 1023, 613, 2),
    executable_functions: ["FUN_005ba5e0", "FUN_005ba7f0", "FUN_005c4ed0", "FUN_005d4d10"],
  });
  return probes;
}

function probeTacticalTrenchRunOutcome(folder, stable, consoleLines, ready, expected) {
  const movie = expected === "success" ? "201.webm" : "202.webm";
  assert.equal(ready.trench_run_outcome, expected);
  const route = consoleLines.find(({ text }) =>
    text.includes(`[cutscene] opened path=assets/references/ref-videos/${movie}`));
  assert.ok(route, `trench-run ${expected} did not route through ${movie}`);
  fs.writeFileSync(path.join(folder, `trench-run-${expected}-return.png`), stable.bytes);
  return [{
    type: `trench-run-${expected}-route`,
    result_state: expected === "success" ? 6 : 7,
    movie,
    returned_to: "tactical-combat",
    log: route.text,
  }];
}

async function probeTacticalBattleResultsPresentation(
  page, viewport, folder, stable, consoleLines, faction,
) {
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const point = (x, y) => ({ x: offsetX + x * scale, y: offsetY + y * scale });
  const alliance = faction === "alliance";
  const resources = alliance ? {
    frame: 10335, close: 10370, tabs: [10729, 10731, 10733, 10792],
    summary: 10759, detail: 10762, fleet: 10765,
  } : {
    frame: 10336, close: 10376, tabs: [10739, 10741, 10743, 10521],
    summary: 10760, detail: 10762, fleet: 10766,
  };
  const tabX = alliance ? 523.5 : 533;
  const tabY = alliance ? [182.5, 236.5, 290.5, 344.5] : [183.5, 242.5, 301.5, 360.5];
  const frameIgnored = [
    { x: 12, y: 13, width: 400, height: 310 },
    { x: 412, y: 0, width: 58, height: 330 },
  ];
  const probes = [{
    type: "battle-results-frame",
    ...verifyStrategyBitmap(viewport, stable.bytes, resources.frame, 85, 74, frameIgnored),
  }, {
    type: "battle-results-summary",
    ...verifyStrategyBitmap(viewport, stable.bytes, resources.summary, 97, 87, [
      { x: 0, y: 4, width: 400, height: 43 },
      { x: 0, y: 202, width: 400, height: 94 },
    ]),
  }, {
    type: "battle-results-close-rest",
    ...verifyStrategyBitmap(
      viewport, stable.bytes, resources.close, alliance ? 508 : 511, alliance ? 99 : 91,
    ),
  }, {
    type: "battle-results-summary-selected",
    ...verifyStrategyBitmap(
      viewport, stable.bytes, resources.tabs[0], alliance ? 503 : 511, alliance ? 162 : 163,
    ),
  }];

  const clickTab = async (index, name) => {
    const location = point(tabX, tabY[index]);
    await page.mouse.click(location.x, location.y);
    return stableInteractionFrame(page, folder, name);
  };

  let capture = await clickTab(1, "results-alliance-forces");
  probes.push({
    type: "battle-results-alliance-tab",
    ...verifyStrategyBitmap(
      viewport, capture, resources.tabs[1], alliance ? 503 : 511, alliance ? 216 : 222,
    ),
  }, {
    type: "battle-results-alliance-detail",
    ...verifyStrategyBitmap(viewport, capture, resources.detail, 97, 87, [
      { x: 0, y: 0, width: 400, height: 100 },
      { x: 0, y: 100, width: 340, height: 196 },
    ]),
  });

  for (let category = 0; category < 4; category++) {
    const location = point(85 + 62 + category * 96, 74 + 70);
    await page.mouse.click(location.x, location.y);
    capture = await stableInteractionFrame(page, folder, `results-category-${category + 1}`);
    probes.push({
      type: "battle-results-force-category",
      category: ["capital-ships", "fighters", "troops", "personnel"][category],
      screenshot_sha256: sha256(capture),
    });
  }

  capture = await clickTab(2, "results-imperial-forces");
  probes.push({
    type: "battle-results-imperial-tab",
    ...verifyStrategyBitmap(
      viewport, capture, resources.tabs[2], alliance ? 503 : 511, alliance ? 270 : 281,
    ),
  });

  capture = await clickTab(3, "results-go-directly-to");
  probes.push({
    type: "battle-results-destination-tab",
    ...verifyStrategyBitmap(
      viewport, capture, resources.tabs[3], alliance ? 503 : 511, alliance ? 324 : 340,
    ),
  }, {
    type: "battle-results-destination-background",
    ...verifyStrategyBitmap(viewport, capture, 10763, 97, 87, [
      { x: 0, y: 20, width: 400, height: 45 },
      { x: 0, y: 142, width: 400, height: 40 },
      { x: 17, y: 169, width: 169, height: 96 },
      { x: 217, y: 169, width: 169, height: 96 },
    ]),
  }, {
    type: "battle-results-system-destination",
    ...verifyStrategyBitmap(viewport, capture, 10764, 114, 256),
  }, {
    type: "battle-results-fleet-destination",
    ...verifyStrategyBitmap(viewport, capture, resources.fleet, 314, 256),
  });

  const destination = alliance ? point(198.5, 304) : point(398.5, 304);
  await page.mouse.click(destination.x, destination.y);
  await page.waitForTimeout(200);
  const routed = await stableInteractionFrame(
    page, folder, alliance ? "results-route-system" : "results-route-fleet",
  );
  assert.notEqual(sha256(capture), sha256(routed), `${faction}: result destination did not route`);
  const route = alliance ? "system" : "fleet";
  assert.ok(consoleLines.some(({ text }) =>
    text.includes(`[tactical-results] destination=${route}`)), `${faction}: missing route log`);
  const persistence = consoleLines.find(({ text }) =>
    text.includes("[tactical_results] strategic_persistence applied=true"));
  assert.ok(persistence, `${faction}: strategic result was not persisted before routing`);
  assert.match(persistence.text, /attacker_capitals=3->2/,
    `${faction}: destroyed attacker capital was not removed from strategic state`);
  assert.match(persistence.text, /defender_capitals=3->2/,
    `${faction}: destroyed defender capital was not removed from strategic state`);
  assert.match(persistence.text, /attacker_fighters=3->2/,
    `${faction}: destroyed attacker fighter was not removed from strategic state`);
  assert.match(persistence.text, /defender_fighters=3->2/,
    `${faction}: destroyed defender fighter was not removed from strategic state`);
  probes.push({ type: "battle-results-route", destination: route,
    screenshot_sha256: sha256(routed) });
  probes.push({ type: "strategic-result-persistence", log: persistence.text });

  for (const category of ["Capital Ships", "Fighters", "Troops", "Personnel"]) {
    assert.ok(consoleLines.some(({ text }) => text.includes(`category=${category}`)),
      `${faction}: missing ${category} result category transition`);
  }
  return probes;
}

async function probeTacticalBattleAlertEntry(
  page, viewport, folder, stable, consoleLines, ready, faction,
) {
  assert.equal(ready.battle_alert_open, true, `${faction}: Battle Alert was bypassed`);
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const point = (x, y) => ({ x: offsetX + x * scale, y: offsetY + y * scale });
  const alliance = faction === "alliance";
  const resources = alliance ? {
    frame: 10710,
    scene: 10712,
    rail: 10820,
    tabs: [10729, 10731, 10733, 10735],
    retreat: [10971, 10972],
    simulate: [10716, 10717],
    command: [10719, 10720],
  } : {
    frame: 10711,
    scene: 10713,
    rail: 10821,
    tabs: [10739, 10741, 10743, 10745],
    retreat: [10974, 10975],
    simulate: [10722, 10723],
    command: [10725, 10726],
  };
  const probes = [{
    type: "battle-alert-frame",
    ...verifyStrategyBitmap(viewport, stable.bytes, resources.frame, 65, 54, [
      { x: 12, y: 13, width: 400, height: 310 },
      { x: 412, y: 0, width: 58, height: 331 },
    ]),
  }, {
    type: "battle-alert-scene",
    ...verifyStrategyBitmap(viewport, stable.bytes, resources.scene, 77, 67, [
      { x: 0, y: 0, width: 400, height: 48 },
      { x: 20, y: 145, width: 360, height: 100 },
      { x: 0, y: 280, width: 400, height: 30 },
    ]),
  }, {
    type: "battle-alert-rail",
    ...verifyStrategyBitmap(viewport, stable.bytes, resources.rail, 477, 54,
      (alliance ? [21, 81, 143, 205] : [17, 80, 143, 206]).map((y) => ({
        x: alliance ? 6 : 14,
        y,
        width: alliance ? 41 : 44,
        height: 41,
      })).concat([{ x: 0, y: 296, width: 2, height: 27 }])),
  }, {
    type: "battle-alert-summary-selected",
    ...verifyStrategyBitmap(
      viewport,
      stable.bytes,
      resources.tabs[0],
      alliance ? 483 : 491,
      alliance ? 75 : 71,
    ),
  }, {
    type: "battle-alert-retreat-rest",
    ...verifyStrategyBitmap(viewport, stable.bytes, resources.retreat[0], 77, 350),
  }, {
    type: "battle-alert-simulate-rest",
    ...verifyStrategyBitmap(viewport, stable.bytes, resources.simulate[0], 211, 350),
  }, {
    type: "battle-alert-command-rest",
    ...verifyStrategyBitmap(viewport, stable.bytes, resources.command[0], 345, 350),
  }];

  const tabX = alliance ? 503.5 : 513;
  const tabY = alliance ? [95.5, 155.5, 217.5, 279.5] : [91.5, 154.5, 217.5, 280.5];
  for (let index = 1; index < 4; index++) {
    const tab = point(tabX, tabY[index]);
    await page.mouse.click(tab.x, tab.y);
    const capture = await stableInteractionFrame(page, folder, `battle-alert-tab-${index}`);
    probes.push({
      type: "battle-alert-tab",
      tab: ["alliance-forces", "imperial-forces", "system-summary"][index - 1],
      ...verifyStrategyBitmap(
        viewport,
        capture,
        resources.tabs[index],
        alliance ? 483 : 491,
        alliance ? [75, 135, 197, 259][index] : [71, 134, 197, 260][index],
      ),
    });
  }

  const command = point(412, 363.5);
  await page.mouse.move(command.x, command.y);
  await page.mouse.down({ button: "left" });
  const held = await dynamicInteractionFrame(page, folder, "battle-alert-command-held");
  probes.push({
    type: "battle-alert-command-held",
    ...verifyStrategyBitmap(viewport, held, resources.command[1], 345, 350),
  });
  await page.mouse.up({ button: "left" });
  await page.waitForTimeout(100);
  const tactical = await stableInteractionFrame(page, folder, "battle-alert-take-command");
  probes.push({
    type: "battle-alert-take-command",
    ...verifyTacticalShell(viewport, tactical),
    screenshot_sha256: sha256(tactical),
  });
  assert.ok(consoleLines.some(({ text }) =>
    text.includes("[battle_alert] command=take_command")
      && text.includes("status=paused_tactical")),
  `${faction}: Take Command did not enter paused tactical combat`);
  return probes;
}

function probeTacticalAudioRouting(folder, stable, consoleLines, ready) {
  assert.equal(ready.tactical_music_mdata_id, 307);
  assert.equal(ready.tactical_weapon_audio_wave_first, 13033);
  assert.equal(ready.tactical_weapon_audio_wave_last, 13054);
  assert.equal(ready.tactical_weapon_audio_variant_count, 22);
  assert.equal(ready.tactical_voice_variant_count, 285);
  assert.equal(ready.battle_alert_open, false);
  const music = consoleLines.find(({ text }) =>
    text.includes("[audio] context=combat track=Battle mdata=307"));
  const cueLogs = consoleLines.filter(({ text }) =>
    text.includes("[audio] context=combat event=0x") && text.includes("routed=true"));
  assert.ok(music, "tactical music context was not routed");
  assert.match(music.text, /loaded=true/);
  assert.match(music.text, /muted=true/);
  const routed = cueLogs.map(({ text }) => {
    const match = text.match(/event=0x([0-9a-f]{2}) wave=(\d+)/);
    assert.ok(match, `malformed tactical cue log: ${text}`);
    assert.match(text, /loaded=true/);
    assert.match(text, /muted=true/);
    return { event: Number.parseInt(match[1], 16), wave: Number(match[2]) };
  });
  assert.deepEqual(routed.map(({ wave }) => wave),
    Array.from({ length: 22 }, (_, index) => 13033 + index));
  assert.deepEqual([...new Set(routed.map(({ event }) => event))],
    [0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14]);
  const voiceLogs = consoleLines.filter(({ text }) =>
    text.includes("[audio] context=combat voice_event=") && text.includes("routed=true"));
  const voices = voiceLogs.map(({ text }) => {
    const match = text.match(/source_event=0x([0-9a-f]+) wave=(\d+) faction=(Alliance|Empire)/);
    assert.ok(match, `malformed tactical voice log: ${text}`);
    assert.match(text, /loaded=true/);
    assert.match(text, /muted=true/);
    return { sourceEvent: Number.parseInt(match[1], 16), wave: Number(match[2]), faction: match[3] };
  });
  const resourceRanges = [
    ["Alliance", 14001, 14122], ["Alliance", 15133, 15163],
    ["Empire", 15001, 15132],
  ];
  const expectedVoices = resourceRanges.flatMap(([faction, first, last]) =>
    Array.from({ length: last - first + 1 }, (_, index) => ({ faction, wave: first + index })));
  assert.deepEqual(voices.map(({ faction, wave }) => ({ faction, wave })), expectedVoices);
  assert.equal(new Set(voices.map(({ faction, wave }) => `${faction}:${wave}`)).size, 285);
  assert.deepEqual(voices.slice(0, 2).map(({ sourceEvent }) => sourceEvent), [0x20, 0x21]);
  assert.equal(voices.find(({ faction, wave }) => faction === "Alliance" && wave === 15133).sourceEvent, 0x11e);
  assert.equal(voices.find(({ faction, wave }) => faction === "Empire" && wave === 15001).sourceEvent, 0x9a);
  fs.writeFileSync(path.join(folder, "tactical-audio-muted.png"), stable.bytes);
  return [{
    type: "tactical-audio-routing",
    music_mdata_id: 307,
    tactical_wave_first: 13033,
    tactical_wave_last: 13054,
    tactical_wave_variants: 22,
    tactical_voice_variants: 285,
    music_log: music.text,
    cue_logs: cueLogs.map(({ text }) => text),
    voice_logs: voiceLogs.map(({ text }) => text),
    browser_muted: true,
    screenshot_sha256: sha256(stable.bytes),
  }];
}

async function probeTacticalBattleOptionsPresentation(
  page, viewport, folder, stable, consoleLines, faction, withdrawalJourney,
) {
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const point = (x, y) => ({ x: offsetX + x * scale, y: offsetY + y * scale });
  const panelResource = faction === "alliance" ? 1303 : 1304;
  const controls = [
    { name: "withdraw", x: 497, y: 47, width: 48, height: 34, normal: 1149, pressed: 1150 },
    { name: "simulate", x: 497, y: 114, width: 44, height: 25, normal: 1151, pressed: 1152 },
    { name: "observe", x: 561, y: 114, width: 45, height: 25, normal: 1153, pressed: 1154 },
    { name: "game-options", x: 494, y: 203, width: 52, height: 33, normal: 1155, pressed: 1156 },
    { name: "close", x: 565, y: 203, width: 29, height: 33, normal: 1157, pressed: 1158 },
  ];
  const ignored = (included = controls) => included.map(({ x, y, width, height }) => ({
    x: x - 481, y: y - 27, width, height,
  }));
  const captureHeld = async (control, name) => {
    const location = point(control.x + control.width / 2, control.y + control.height / 2);
    await page.mouse.move(location.x, location.y);
    await page.mouse.down();
    const capture = await stableInteractionFrame(page, folder, name);
    await page.mouse.up();
    return capture;
  };
  const openLocation = point(616, 318);
  await page.mouse.move(openLocation.x, openLocation.y);
  await page.mouse.down();
  const openHeld = await stableInteractionFrame(page, folder, "battle-options-open-held");
  const probes = [{
    type: "battle-options-hud-pressed",
    ...verifyTacticalBitmap(viewport, openHeld, 1039, 606, 308),
  }];
  await page.mouse.up();
  let panel = await stableInteractionFrame(page, folder, "battle-options-open");
  probes.push({
    type: "battle-options-panel",
    faction,
    ...verifyTacticalBitmap(viewport, panel, panelResource, 481, 27, [], ignored()),
  });
  for (const control of controls) {
    probes.push({
      type: "battle-options-control",
      control: control.name,
      ...verifyTacticalBitmap(viewport, panel, control.normal, control.x, control.y),
    });
  }

  if (withdrawalJourney) {
    const withdraw = controls[0];
    const held = await captureHeld(withdraw, "battle-options-withdraw-held");
    probes.push({
      type: "battle-options-withdraw-pressed",
      ...verifyTacticalBitmap(viewport, held, withdraw.pressed, withdraw.x, withdraw.y),
    });
    let confirmation = await stableInteractionFrame(
      page, folder, "battle-options-withdraw-confirmation",
    );
    const confirmationIgnored = [
      { x: 20, y: 28, width: 125, height: 14 },
      { x: 14, y: 92, width: 132, height: 28 },
      { x: 66, y: 202, width: 27, height: 25 },
      { x: 106, y: 202, width: 27, height: 25 },
    ];
    probes.push({
      type: "withdraw-confirmation-panel",
      ...verifyTacticalBitmap(
        viewport, confirmation, 1310, 481, 27, [], confirmationIgnored,
      ),
    });
    for (const control of [
      { name: "confirm", x: 547, y: 229, width: 27, height: 25, normal: 1113, pressed: 1114 },
      { name: "cancel", x: 587, y: 229, width: 27, height: 25, normal: 1115, pressed: 1116 },
    ]) {
      probes.push({
        type: "withdraw-confirmation-control",
        control: control.name,
        ...verifyTacticalBitmap(viewport, confirmation, control.normal, control.x, control.y),
      });
    }
    const cancel = { x: 587, y: 229, width: 27, height: 25, pressed: 1116 };
    const cancelHeld = await captureHeld(cancel, "battle-options-withdraw-cancel-held");
    probes.push({
      type: "withdraw-confirmation-cancel-pressed",
      ...verifyTacticalBitmap(viewport, cancelHeld, cancel.pressed, cancel.x, cancel.y),
    });
    await stableInteractionFrame(page, folder, "battle-options-withdraw-cancelled");

    await page.mouse.click(openLocation.x, openLocation.y);
    await stableInteractionFrame(page, folder, "battle-options-withdraw-reopened");
    await page.mouse.click(point(521, 64).x, point(521, 64).y);
    confirmation = await stableInteractionFrame(
      page, folder, "battle-options-withdraw-confirmation-reopened",
    );
    const confirm = { x: 547, y: 229, width: 27, height: 25, pressed: 1114 };
    const confirmHeld = await captureHeld(confirm, "battle-options-withdraw-confirm-held");
    probes.push({
      type: "withdraw-confirmation-confirm-pressed",
      ...verifyTacticalBitmap(viewport, confirmHeld, confirm.pressed, confirm.x, confirm.y),
    });
    await stableInteractionFrame(page, folder, "battle-options-withdraw-started");
    await page.mouse.click(openLocation.x, openLocation.y);
    panel = await stableInteractionFrame(page, folder, "battle-options-withdraw-disabled");
    probes.push({
      type: "battle-options-withdraw-disabled",
      ...verifyTacticalBitmap(
        viewport, panel, panelResource, 481, 27, [], ignored(controls.slice(1)),
      ),
    });
    const disabledWithdraw = point(521, 64);
    await page.mouse.click(disabledWithdraw.x, disabledWithdraw.y);
    await stableInteractionFrame(page, folder, "battle-options-disabled-click-rejected");
    const withdrawLogs = consoleLines.filter(({ text }) =>
      text.includes("[tactical_options] command=withdraw"));
    assert.equal(withdrawLogs.filter(({ text }) => text.includes("status=confirmation_open")).length,
      2, "withdrawal confirmation did not open exactly twice");
    assert.equal(withdrawLogs.filter(({ text }) =>
      text.includes("status=confirmation_cancelled")).length, 1,
    "withdrawal confirmation cancel did not route exactly once");
    assert.equal(withdrawLogs.filter(({ text }) =>
      text.includes("status=withdrawal_started")).length, 1,
    "disabled withdrawal accepted another command or confirmation did not dispatch");
    assert.match(withdrawLogs.find(({ text }) =>
      text.includes("status=withdrawal_started"))?.text || "", /hyperdrive_warning=true/,
    "withdrawal confirmation did not detect the stranded hyperdrive-disabled ship");
    const expectedVoices = faction === "alliance" ? [
      { name: "WithdrawalStarted", event: "0x8f", wave: 14112, owner: "Alliance" },
      { name: "WithdrawalHyperdriveWarning", event: "0x95", wave: 14118, owner: "Alliance" },
    ] : [
      { name: "WithdrawalStarted", event: "0x10d", wave: 15116, owner: "Empire" },
      { name: "WithdrawalHyperdriveWarning", event: "0x10e", wave: 15117, owner: "Empire" },
    ];
    const voiceLogs = expectedVoices.map((expected) => {
      const voice = consoleLines.find(({ text }) =>
        text.includes(`voice_event=${expected.name}`)
          && text.includes(`source_event=${expected.event}`));
      assert.match(voice?.text || "",
        new RegExp(`wave=${expected.wave} faction=${expected.owner} routed=true loaded=true muted=true`),
        `withdrawal did not route exact muted ${expected.name} voice`);
      return voice.text;
    });
    probes.push({
      type: "battle-options-withdraw-route",
      logs: withdrawLogs.map(({ text }) => text),
      voice_logs: voiceLogs,
    });
    return probes;
  }

  const observe = controls[2];
  let held = await captureHeld(observe, "battle-options-observe-held");
  probes.push({
    type: "battle-options-observe-pressed",
    ...verifyTacticalBitmap(viewport, held, observe.pressed, observe.x, observe.y),
  });
  panel = await stableInteractionFrame(page, folder, "battle-options-observing");
  probes.push({
    type: "battle-options-observe-selected",
    ...verifyTacticalBitmap(viewport, panel, observe.pressed, observe.x, observe.y),
  });
  await captureHeld(observe, "battle-options-take-command-held");
  panel = await stableInteractionFrame(page, folder, "battle-options-take-command");
  probes.push({
    type: "battle-options-take-command",
    ...verifyTacticalBitmap(viewport, panel, observe.normal, observe.x, observe.y),
  });

  const gameOptions = controls[3];
  held = await captureHeld(gameOptions, "battle-options-game-options-held");
  probes.push({
    type: "battle-options-game-options-pressed",
    ...verifyTacticalBitmap(viewport, held, gameOptions.pressed, gameOptions.x, gameOptions.y),
  });
  await stableInteractionFrame(page, folder, "battle-options-game-options-route");
  assert.ok(consoleLines.some(({ text }) =>
    text.includes("[tactical_options] command=game_options status=routed")),
  "Battle Options did not route Game Options");
  const gameOptionsScreen = await stableInteractionFrame(page, folder, "game-options-open");
  probes.push({
    type: "game-options-background",
    ...verifyCommonBitmap(viewport, gameOptionsScreen, 20002, 0, 0, [
      // The full-screen quad's outer two pixels blend with the cleared WebGL
      // framebuffer. Compare the stable source-exact frame around the dynamic
      // labels and controls instead of treating raster coverage as asset drift.
      { x: 0, y: 0, width: 640, height: 2 },
      { x: 0, y: 478, width: 640, height: 2 },
      { x: 0, y: 2, width: 2, height: 476 },
      { x: 638, y: 2, width: 2, height: 476 },
      { x: 22, y: 26, width: 596, height: 440 },
    ]),
  }, {
    type: "game-options-tactical-toggle-disabled",
    ...verifyCommonBitmap(viewport, gameOptionsScreen, 10045, 359, 310),
  });
  assert.ok(consoleLines.some(({ text }) =>
    text.includes("destination=game_options status=opened_original")),
  "Battle Options did not open the original unified Game Options screen");
  const returnToBattle = point(185, 404);
  await page.mouse.click(returnToBattle.x, returnToBattle.y);
  panel = await stableInteractionFrame(page, folder, "game-options-return-to-battle");
  assert.ok(consoleLines.some(({ text }) =>
    text.includes("[game_options] command=return origin=TacticalBattle")),
  "Game Options did not return to the tactical battle");

  const close = controls[4];
  held = await captureHeld(close, "battle-options-close-held");
  probes.push({
    type: "battle-options-close-pressed",
    ...verifyTacticalBitmap(viewport, held, close.pressed, close.x, close.y),
  });
  const closed = await stableInteractionFrame(page, folder, "battle-options-closed");
  assert.notEqual(sha256(panel), sha256(closed), "Battle Options close did not restore the display panel");

  await page.mouse.click(openLocation.x, openLocation.y);
  await stableInteractionFrame(page, folder, "battle-options-reopened");
  const simulate = controls[1];
  held = await captureHeld(simulate, "battle-options-simulate-held");
  probes.push({
    type: "battle-options-simulate-pressed",
    ...verifyTacticalBitmap(viewport, held, simulate.pressed, simulate.x, simulate.y),
  });
  const results = await stableInteractionFrame(page, folder, "battle-options-simulated-results");
  probes.push({
    type: "battle-options-simulate-route",
    ...verifyStrategyBitmap(viewport, results, faction === "alliance" ? 10335 : 10336, 85, 74, [
      { x: 12, y: 13, width: 400, height: 310 },
      { x: 412, y: 0, width: 58, height: 330 },
    ]),
  });
  const optionLogs = consoleLines
    .filter(({ text }) => text.includes("[tactical_options]"))
    .map(({ text }) => text);
  for (const expected of ["command=observe", "command=take_command", "command=game_options",
    "command=close", "command=simulate_remainder"]) {
    assert.ok(optionLogs.some((line) => line.includes(expected)),
      `missing Battle Options transition ${expected}`);
  }
  probes.push({ type: "battle-options-routes", logs: optionLogs });
  return probes;
}

function probeTacticalSubsystemFieldCommandPresentation(viewport, folder, stable, ready) {
  const probes = probeTacticalSelectedDamagePresentation(viewport, folder, stable, ready);
  const selected = ready.selected_ship;
  const expectedPercentages = [0, 24, 25, 50, 75];
  const expectedResources = [1201, 1207, 1213, 1219, 1225];
  assert.deepEqual(selected.subsystem_percentages, expectedPercentages);
  assert.deepEqual(selected.subsystem_resources, expectedResources);

  const subsystemProofs = expectedResources.map((resourceId, index) =>
    verifyTacticalBitmap(viewport, stable.bytes, resourceId, 491 + index * 27, 130));
  const tractor = ready.fields.find((field) => field.target === 1);
  const gravity = ready.fields.find((field) => field.target === 4);
  assert.deepEqual(tractor, {
    target: 1,
    resource_id: 3624,
    frame: 4,
    tractor_sources: 2,
    gravity_sources: 0,
    tractor_source_ids: [3, 5],
    gravity_source_ids: [],
  });
  assert.deepEqual(gravity, {
    target: 4,
    resource_id: 3524,
    frame: 4,
    tractor_sources: 1,
    gravity_sources: 1,
    tractor_source_ids: [0],
    gravity_source_ids: [2],
  });
  fs.writeFileSync(path.join(folder, "subsystem-field-command.png"), stable.bytes);
  probes.push({
    type: "source-traced-subsystem-field-command-state",
    executable_functions: [
      "FUN_005e45f0", "FUN_005e7540", "FUN_005e77c0",
      "FUN_005b23e0", "FUN_005b2440", "FUN_005b24d0", "FUN_005b2480",
      "FUN_005b2520", "FUN_005b2550", "FUN_005b2570", "FUN_005b25d0",
    ],
    subsystem_order: ["shields", "weapons", "tractor", "engines", "hyperdrive"],
    percentages: expectedPercentages,
    resources: expectedResources,
    subsystem_proofs: subsystemProofs,
    tractor_source_limit: 1,
    gravity_target_limit_per_source: 4,
    fields: [tractor, gravity],
  });
  return probes;
}

async function probeTacticalGroupPresentation(
  page, viewport, folder, stable, consoleLines, ready, faction,
) {
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const point = (x, y) => ({ x: offsetX + x * scale, y: offsetY + y * scale });
  const capture = async (name) => stableInteractionFrame(page, folder, name);
  const labelMask = [{ x: 4, y: 2, width: 17, height: 14 }];
  const playerFighter = ready.participants.find((participant) =>
    participant.kind === "fighter-group" && participant.faction === faction
      && participant.fighter_group === 3);
  assert.ok(playerFighter, "group fixture omitted the player's gold fighter group");
  const portraitResource = 2030 + playerFighter.tactical_ordinal - 29;
  const taskButtons = Array.from({ length: 8 }, (_, index) => ({
    id: index === 0 ? 1006 : 1005,
    x: 60 + index * 26,
    y: 2,
  }));
  const fighterButtons = [1012, 1013, 1014, 1015].map((id, index) => ({
    id, x: 330 + index * 26, y: 2,
  }));
  const probes = [];
  probes.push({
    type: "task-force-header",
    ...verifyTacticalBitmap(viewport, stable.bytes, faction === "alliance" ? 1001 : 1004,
      10, 1, taskButtons),
  });
  probes.push({
    type: "fighter-group-header",
    ...verifyTacticalBitmap(viewport, stable.bytes, faction === "alliance" ? 1008 : 1010,
      275, 1, fighterButtons),
  });
  for (const button of taskButtons) {
    probes.push({
      type: "task-force-initial",
      slot: (button.x - 60) / 26 + 1,
      ...verifyTacticalBitmap(
        viewport, stable.bytes, button.id, button.x, button.y, [], labelMask,
      ),
    });
  }
  for (const [index, button] of fighterButtons.entries()) {
    probes.push({
      type: "fighter-group-initial",
      slot: index + 1,
      ...verifyTacticalBitmap(viewport, stable.bytes, button.id, button.x, button.y),
    });
  }

  const matte = point(330.1, 2.1);
  await page.mouse.click(matte.x, matte.y);
  await page.waitForTimeout(80);
  const matteFrame = await capture("group-transparent-matte");
  assert.equal(sha256(matteFrame), sha256(stable.bytes),
    "transparent fighter-group matte changed the paused battle");

  const taskEight = point(254, 10);
  await page.mouse.move(taskEight.x, taskEight.y);
  await page.mouse.down({ button: "left" });
  await page.waitForTimeout(80);
  const taskEightHeld = await capture("task-force-8-held");
  probes.push({ type: "task-force-8-held", ...verifyTacticalBitmap(
    viewport, taskEightHeld, 1006, 242, 2, [], labelMask,
  ) });
  await page.mouse.up({ button: "left" });
  await page.waitForTimeout(80);
  const taskEightSelected = await capture("task-force-8-selected");
  probes.push({ type: "task-force-8-selected", ...verifyTacticalBitmap(
    viewport, taskEightSelected, 1006, 242, 2, [], labelMask,
  ) });
  probes.push({ type: "task-force-1-released", ...verifyTacticalBitmap(
    viewport, taskEightSelected, 1005, 60, 2, [], labelMask,
  ) });

  const gold = point(420, 10);
  await page.mouse.move(gold.x, gold.y);
  await page.mouse.down({ button: "left" });
  await page.waitForTimeout(80);
  const goldHeld = await capture("gold-group-held");
  probes.push({ type: "gold-group-held", ...verifyTacticalBitmap(
    viewport, goldHeld, 1019, 408, 2,
  ) });
  await page.mouse.up({ button: "left" });
  await page.waitForTimeout(80);
  const goldSelected = await capture("gold-group-selected");
  probes.push({ type: "gold-group-selected", ...verifyTacticalBitmap(
    viewport, goldSelected, 1019, 408, 2,
  ) });
  probes.push({ type: "selected-fighter-panel", ...verifyTacticalBitmap(
    viewport, goldSelected, 1307, 481, 27,
    [{ id: portraitResource, x: 505, y: 76 }],
    [
      { x: 8, y: 7, width: 145, height: 40 },
      { x: 12, y: 210, width: 126, height: 22 },
    ],
  ) });
  probes.push({ type: "selected-fighter-portrait", ...verifyTacticalBitmap(
    viewport, goldSelected, portraitResource, 505, 76,
  ) });

  const participantLine = consoleLines.find(({ text }) =>
    text.includes("[tactical_3d] participant_scene"));
  const participantPositions = parseTacticalScenePositions(
    participantLine, "mixed-task-force participant scene",
  );
  const playerCapitals = ready.participants
    .map((participant, index) => ({ participant, objectId: index + 1 }))
    .filter(({ participant }) => participant.kind === "capital-ship"
      && participant.faction === faction && participant.active);
  const addressablePlayerCapitals = playerCapitals
    .filter(({ objectId }) => {
      const position = participantPositions.get(objectId);
      return position
        && position.x >= offsetX + 40 * scale && position.x < offsetX + 430 * scale
        && position.y >= offsetY + 40 * scale && position.y < offsetY + 420 * scale;
    })
    .sort((left, right) => {
      const leftPosition = participantPositions.get(left.objectId);
      const rightPosition = participantPositions.get(right.objectId);
      const centerX = offsetX + 238 * scale;
      const centerY = offsetY + 247 * scale;
      const distance = (position) => (position.x - centerX) ** 2 + (position.y - centerY) ** 2;
      return distance(leftPosition) - distance(rightPosition);
    });
  const firstCapital = addressablePlayerCapitals[0];
  const secondCapital = addressablePlayerCapitals.find(({ participant }) =>
    participant.task_force !== firstCapital?.participant.task_force);
  const hostileCapital = ready.participants
    .map((participant, index) => ({ participant, objectId: index + 1 }))
    .find(({ participant }) => participant.kind === "capital-ship"
      && participant.faction !== faction && participant.active);
  assert.ok(firstCapital && secondCapital && hostileCapital,
    "group fixture omitted mixed player task forces or a hostile focus target");
  const firstPosition = participantPositions.get(firstCapital.objectId);
  const secondPosition = participantPositions.get(secondCapital.objectId);
  const hostilePosition = participantPositions.get(hostileCapital.objectId);
  assert.ok(firstPosition && secondPosition && hostilePosition,
    "mixed-task-force journey omitted a required projected position");

  await page.mouse.click(firstPosition.x, firstPosition.y);
  await page.waitForTimeout(80);
  await page.keyboard.down("ControlLeft");
  await page.waitForTimeout(80);
  await page.mouse.click(secondPosition.x, secondPosition.y);
  await page.waitForTimeout(80);
  await page.keyboard.up("ControlLeft");
  await page.waitForTimeout(80);
  const selectionLogs = consoleLines
    .filter(({ text }) => text.includes("[tactical_3d] selection object_id="))
    .map(({ text }) => text);
  assert.ok(selectionLogs.some((line) => line.includes(`object_id=${firstCapital.objectId} `))
      && selectionLogs.some((line) => line.includes(`object_id=${secondCapital.objectId} `)),
  "mixed-task-force journey did not select both projected capitals");
  await capture("mixed-task-force-selection");
  const assignedFocusCount = consoleLines.filter(({ text }) =>
    text.includes("[tactical_3d] focus source_object_ids=")).length;
  const assignedTargetCount = consoleLines.filter(({ text }) =>
    text.includes("[tactical_navigation] event=target_assign status=assigned")).length;
  await page.mouse.click(hostilePosition.x, hostilePosition.y, { button: "right" });
  await page.waitForTimeout(120);
  await page.mouse.move(secondPosition.x, secondPosition.y);
  await page.waitForTimeout(80);
  const rejectedTarget = await capture("mixed-task-force-target-rejected");
  assert.equal(consoleLines.filter(({ text }) =>
    text.includes("[tactical_3d] focus source_object_ids=")).length, assignedFocusCount,
  "rejected mixed-task-force target entered the focus-assignment path");
  assert.equal(consoleLines.filter(({ text }) =>
    text.includes("[tactical_navigation] event=target_assign status=assigned")).length,
  assignedTargetCount, "rejected mixed-task-force target emitted an assigned order");
  const rejectionLog = consoleLines.find(({ text }) =>
    text.includes("[tactical_navigation] event=target_assign")
      && text.includes("status=rejected_mixed_task_forces"));
  assert.match(rejectionLog?.text || "", /members=2/,
    "mixed-task-force target did not report the exact rejected selection");
  const expectedVoice = faction === "alliance"
    ? { event: "0x84", wave: 14101, owner: "Alliance" }
    : { event: "0x102", wave: 15105, owner: "Empire" };
  const rejectionVoice = consoleLines.find(({ text }) =>
    text.includes("voice_event=TargetTaskForceRejected")
      && text.includes(`source_event=${expectedVoice.event}`));
  assert.match(rejectionVoice?.text || "",
    new RegExp(`wave=${expectedVoice.wave} faction=${expectedVoice.owner} routed=true loaded=true muted=true`),
    "mixed-task-force rejection did not route the exact muted faction voice");
  probes.push({
    type: "source-rejected-mixed-task-force-target",
    executable_function: "FUN_005a24d0",
    player_task_forces: [firstCapital.participant.task_force, secondCapital.participant.task_force],
    hostile_object_id: hostileCapital.objectId,
    rejection_log: rejectionLog.text,
    voice_log: rejectionVoice.text,
    source_event: expectedVoice.event,
    resource_id: expectedVoice.wave,
    assignment_state_unchanged: true,
    screenshot_sha256: sha256(rejectedTarget),
  });

  await page.keyboard.press("F1");
  await page.waitForTimeout(80);
  const taskOneHotkey = await capture("task-force-1-hotkey");
  probes.push({ type: "task-force-1-hotkey", ...verifyTacticalBitmap(
    viewport, taskOneHotkey, 1006, 60, 2, [], labelMask,
  ) });
  await page.keyboard.press("F12");
  await page.waitForTimeout(80);
  const goldHotkey = await capture("gold-group-hotkey");
  probes.push({ type: "gold-group-f12-hotkey", ...verifyTacticalBitmap(
    viewport, goldHotkey, 1019, 408, 2,
  ) });

  assert.ok(consoleLines.some(({ text }) =>
    text.includes("[tactical_groups] task_force_selected slot=8 members=1")),
  "task-force selection did not emit its deterministic state transition");
  assert.ok(consoleLines.some(({ text }) =>
    text.includes("[tactical_groups] fighter_group_selected slot=4 members=1")),
  "fighter-group selection did not emit its deterministic state transition");
  probes.push({
    type: "source-task-force-fighter-group-presentation",
    executable_functions: ["FUN_005c7150", "FUN_0059f680", "FUN_005ab650"],
    task_force_resources: { normal: 1005, pressed: 1006, unassigned: 1007 },
    fighter_group_resources: { normal: [1012, 1013, 1014, 1015], pressed: [1016, 1017, 1018, 1019] },
    fighter_texture_resources: ready.participants
      .filter(({ kind }) => kind === "fighter-group")
      .map(({ faction: owner, fighter_group, active_close_resource,
        active_far_resource, active_indicator_resource }) => ({
        faction: owner,
        fighter_group,
        close: active_close_resource,
        far: active_far_resource,
        indicator: active_indicator_resource,
      })),
    keyboard: { task_forces: "F1-F8", fighter_groups: "F9-F12" },
  });
  return probes;
}

async function probeTacticalSelectedNavigationPresentation(
  page, viewport, folder, stable, consoleLines, ready, faction,
) {
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const point = (x, y) => ({ x: offsetX + x * scale, y: offsetY + y * scale });
  const capture = async (name) => stableInteractionFrame(page, folder, name);
  const playerCapitals = ready.participants.filter(({ kind, faction: owner, active, task_force }) =>
    kind === "capital-ship" && owner === faction && active && task_force === 0);
  assert.equal(playerCapitals.length, 3,
    "selected-navigation fixture must expose three live capitals in task force 1");

  const probes = [
    {
      type: "previous-capital-rest",
      ...verifyTacticalBitmap(viewport, stable.bytes, 1101, 488, 37),
    },
    {
      type: "next-capital-rest",
      ...verifyTacticalBitmap(viewport, stable.bytes, 1103, 613, 37),
    },
  ];
  const press = async (name, location, pressedResource, pressedX) => {
    await page.mouse.move(location.x, location.y);
    await page.mouse.down({ button: "left" });
    await page.waitForTimeout(80);
    const held = await capture(`${name}-held`);
    probes.push({
      type: `${name}-held`,
      ...verifyTacticalBitmap(viewport, held, pressedResource, pressedX, 37),
    });
    await page.mouse.up({ button: "left" });
    await page.waitForTimeout(80);
    const released = await capture(`${name}-released`);
    probes.push({
      type: `${name}-released`,
      framebuffer_sha256: sha256(released),
    });
  };

  await press("next-capital", point(618, 60), 1104, 613);
  await press("previous-capital", point(493, 60), 1102, 488);
  await press("previous-capital-wrap", point(493, 60), 1102, 488);
  const transitions = consoleLines
    .filter(({ text }) => text.includes("[tactical_selection]"))
    .map(({ text }) => text);
  assert.equal(transitions.length, 3,
    "selected-capital navigation did not emit all three transitions");
  assert.match(transitions[0], /direction=next task_force=1 from=(\d+) to=(\d+)/);
  assert.match(transitions[1], /direction=previous task_force=1 from=(\d+) to=(\d+)/);
  assert.match(transitions[2], /direction=previous task_force=1 from=(\d+) to=(\d+)/);
  const parse = (line) => line.match(/from=(\d+) to=(\d+)/).slice(1).map(Number);
  const [nextFrom, nextTo] = parse(transitions[0]);
  const [returnFrom, returnTo] = parse(transitions[1]);
  const [wrapFrom, wrapTo] = parse(transitions[2]);
  assert.deepEqual([returnFrom, returnTo], [nextTo, nextFrom],
    "previous control did not reverse the next-capital transition");
  assert.equal(wrapFrom, nextFrom);
  assert.notEqual(wrapTo, wrapFrom, "previous control did not wrap to another task-force member");
  probes.push({
    type: "source-task-force-selected-capital-navigation",
    constructor_functions: ["FUN_005e4110", "FUN_005e7030"],
    task_force: 1,
    members: playerCapitals.map(({ fleet_roster_index, class_dat_id }) => ({
      fleet_roster_index,
      class_dat_id,
    })),
    controls: {
      previous: { rect: [488, 37, 11, 53], resources: [1101, 1102] },
      next: { rect: [613, 37, 11, 53], resources: [1103, 1104] },
    },
    transitions,
    wraps_within_task_force: true,
  });
  return probes;
}

function parseTacticalNavigationScene(consoleLines) {
  const line = [...consoleLines].reverse().find(({ text }) =>
    text.includes("[tactical_navigation] event=scene")
      && /screen_positions=\S+/.test(text));
  const match = line?.text.match(/screen_positions=([^ ]+)/);
  assert.ok(match, "navigation scene omitted projected point positions");
  return match[1].split(";").map((entry) => {
    const fields = entry.match(/^(\d+):(\d+):(-?[\d.]+),(-?[\d.]+)$/);
    assert.ok(fields, `invalid navigation point projection ${entry}`);
    return {
      set: Number(fields[1]),
      point: Number(fields[2]),
      x: Number(fields[3]),
      y: Number(fields[4]),
    };
  });
}

async function probeTacticalNavigationCameraPresentation(
  page, viewport, folder, stable, consoleLines, ready,
) {
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const point = (x, y) => ({ x: offsetX + x * scale, y: offsetY + y * scale });
  const capture = async (name) => stableInteractionFrame(page, folder, name);
  const press = async (name, location, resource, resourceX, resourceY) => {
    await page.mouse.move(location.x, location.y);
    await page.mouse.down({ button: "left" });
    await page.waitForTimeout(80);
    const held = await capture(`${name}-held`);
    const bitmap = verifyTacticalBitmap(viewport, held, resource, resourceX, resourceY);
    await page.mouse.up({ button: "left" });
    await page.waitForTimeout(80);
    return { bitmap, released: await capture(`${name}-released`) };
  };
  const probes = [];

  const navControls = [
    { set: 1, normal: 1026, selected: 1030, x: 485 },
    { set: 2, normal: 1027, selected: 1031, x: 521 },
    { set: 3, normal: 1028, selected: 1032, x: 560 },
    { set: 4, normal: 1029, selected: 1033, x: 601 },
  ];
  for (const control of navControls) {
    probes.push({
      type: `navigation-set-${control.set}-rest`,
      ...verifyTacticalBitmap(viewport, stable.bytes, control.normal, control.x, 272),
    });
    await page.mouse.click(point(control.x + 13, 285).x, point(control.x + 13, 285).y);
    await page.waitForTimeout(80);
    const selected = await capture(`navigation-set-${control.set}-selected`);
    probes.push({
      type: `navigation-set-${control.set}-selected`,
      ...verifyTacticalBitmap(viewport, selected, control.selected, control.x, 272),
    });
  }

  const navigationPoints = parseTacticalNavigationScene(consoleLines)
    .filter(({ x, y }) => x >= offsetX + 16 * scale && x < offsetX + 460 * scale
      && y >= offsetY + 28 * scale && y < offsetY + 467 * scale);
  assert.ok(navigationPoints.length >= 2,
    "navigation journey did not project two addressable points inside the aperture");
  await page.mouse.click(navigationPoints[0].x, navigationPoints[0].y, { button: "right" });
  await page.waitForTimeout(80);
  await page.keyboard.down("ControlLeft");
  await page.waitForTimeout(80);
  await page.mouse.click(navigationPoints[1].x, navigationPoints[1].y, { button: "right" });
  await page.waitForTimeout(80);
  await page.keyboard.up("ControlLeft");
  await page.waitForTimeout(80);
  const routeFrame = await capture("navigation-route-two-points");
  const routeLogs = consoleLines
    .filter(({ text }) => text.includes("[tactical_navigation] event=route_assign"))
    .map(({ text }) => text);
  assert.ok(routeLogs.some((line) => /append=false .*route_len=1/.test(line)),
    "navigation journey did not replace the route with its first point");
  assert.ok(routeLogs.some((line) => /append=true .*route_len=2/.test(line)),
    "navigation journey did not append its second point");
  assert.notEqual(sha256(routeFrame), sha256(stable.bytes),
    "ordered navigation route produced no framebuffer evidence");

  const participantLine = consoleLines.find(({ text }) =>
    text.includes("[tactical_3d] participant_scene"));
  const participantPositions = parseTacticalScenePositions(
    participantLine, "navigation participant scene",
  );
  const hostileObjectIds = ready.participants
    .map((participant, index) => ({ participant, objectId: index + 1 }))
    .filter(({ participant }) => participant.kind === "capital-ship"
      && participant.faction !== ready.faction)
    .map(({ objectId }) => objectId);
  const enemyObjectIds = hostileObjectIds.length >= 2
    ? [hostileObjectIds[0], hostileObjectIds[hostileObjectIds.length - 1]]
    : hostileObjectIds;
  assert.equal(enemyObjectIds.length, 2,
    "navigation fixture omitted two hostile capital targets");
  const enemyTargets = enemyObjectIds.map((objectId) => participantPositions.get(objectId));
  assert.ok(enemyTargets.every(Boolean), "hostile target projections are incomplete");
  await page.mouse.click(enemyTargets[0].x, enemyTargets[0].y, { button: "right" });
  await page.waitForTimeout(80);
  await page.keyboard.down("ControlLeft");
  await page.waitForTimeout(80);
  await page.mouse.click(enemyTargets[1].x, enemyTargets[1].y, { button: "right" });
  await page.waitForTimeout(80);
  await page.keyboard.up("ControlLeft");
  await page.waitForTimeout(80);
  const multipleTargets = await capture("multiple-targets");
  const targetLogs = consoleLines
    .filter(({ text }) => text.includes("[tactical_navigation] event=target_assign"))
    .map(({ text }) => text);
  assert.ok(targetLogs.some((line) => /append=true .*target_count=2/.test(line)),
    "navigation journey did not retain two ordered hostile targets");
  assert.notEqual(sha256(multipleTargets), sha256(routeFrame),
    "multiple targets produced no additional framebuffer evidence");

  await page.mouse.click(point(498, 355).x, point(498, 355).y);
  await page.waitForTimeout(80);
  const memorizedView = await capture("camera-view-to-memorize");
  const memorized = await press("camera-memorize", point(606, 442), 1043, 585, 430);
  await page.mouse.click(point(615, 355).x, point(615, 355).y);
  await page.mouse.click(point(520, 397).x, point(520, 397).y);
  await page.waitForTimeout(80);
  const changedView = await capture("camera-view-changed");
  assert.notEqual(tacticalApertureHash(viewport, changedView),
    tacticalApertureHash(viewport, memorizedView),
    "camera changes did not alter the tactical aperture");
  const recalled = await press("camera-recall", point(505, 442), 1041, 484, 430);
  assert.equal(tacticalApertureHash(viewport, recalled.released),
    tacticalApertureHash(viewport, memorized.released),
    "camera recall did not restore the memorized retained-mode view");

  await page.keyboard.press("F1");
  await page.waitForTimeout(100);
  const chase = await capture("camera-chase-task-force-1");
  await page.mouse.click(point(520, 397).x, point(520, 397).y);
  await page.waitForTimeout(80);
  const chaseExited = await capture("camera-chase-exited");
  assert.notEqual(tacticalApertureHash(viewport, chase),
    tacticalApertureHash(viewport, recalled.released),
    "task-force chase did not change the retained-mode target");
  assert.notEqual(tacticalApertureHash(viewport, chaseExited),
    tacticalApertureHash(viewport, chase),
    "manual camera control did not exit the chase view");

  const cameraLogs = consoleLines
    .filter(({ text }) => text.includes("[tactical_camera]"))
    .map(({ text }) => text);
  assert.ok(cameraLogs.some((line) => /command=memorize stored=true/.test(line)));
  assert.ok(cameraLogs.some((line) => /command=recall source=memorized/.test(line)));
  assert.ok(cameraLogs.some((line) => /command=follow kind=task_force group=1/.test(line)));
  assert.ok(cameraLogs.some((line) => /command=follow_exit reason=manual/.test(line)));
  probes.push({
    type: "tactical-navigation-camera-journey",
    navigation_resources: navControls.flatMap(({ normal, selected }) => [normal, selected]),
    camera_resources: [1040, 1041, 1042, 1043],
    visible_sets: [1, 2, 3, 4],
    route_points: navigationPoints.slice(0, 2),
    route_logs: routeLogs,
    target_object_ids: enemyObjectIds,
    target_logs: targetLogs,
    camera_logs: cameraLogs,
    provisional_point_coordinates: true,
  });
  return probes;
}

async function runScenarioOnce(server, executable, scenario, faction, viewport) {
  const id = scenarioId(scenario, faction, viewport);
  const folder = path.resolve(runDir, id);
  assert.ok(folder.startsWith(`${runDir}${path.sep}`), `scenario path escaped run directory: ${id}`);
  fs.mkdirSync(folder, { recursive: true });
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
      viewport: { width: viewport.width, height: viewport.height },
      deviceScaleFactor: viewport.device_scale_factor,
      locale: "en-US",
      timezoneId: "America/New_York",
      colorScheme: "dark",
      reducedMotion: "reduce",
      serviceWorkers: "block",
    });
    page = await context.newPage();
    page.on("response", (response) => {
      if (new URL(response.url()).origin === `http://127.0.0.1:${server.address().port}`) {
        requests.push({ url: new URL(response.url()).pathname, status: response.status() });
      }
    });
    page.on("requestfailed", (request) => errors.push(`requestfailed:${request.url()}:${request.failure()?.errorText}`));
    page.on("pageerror", (error) => errors.push(`pageerror:${error.stack || error.message}`));
    page.on("console", (message) => {
      consoleLines.push({ type: message.type(), text: message.text() });
      if (message.type() === "error"
        || /missing.asset|\[bmp_cache\].*not found|\[tactical_3d\].*(unavailable|rejected)/i.test(message.text())) {
        errors.push(`console:${message.type()}:${message.text()}`);
      }
    });

    const code = fixtureCode(scenario, faction);
    const url = `http://127.0.0.1:${server.address().port}/?fixture-code=${code}`;
    await page.goto(url, { waitUntil: "load", timeout: 30_000 });
    await page.waitForFunction(() => window.__openRebellionInterfaceReady?.status, null, {
      timeout: 30_000,
    });
    const ready = await page.evaluate(() => window.__openRebellionInterfaceReady);
    assert.equal(ready.status, battle ? "battle-ready" : "ready", JSON.stringify(ready));
    assert.equal(battle ? ready.fixture_code : ready.code, code);
    await page.evaluate(() => document.fonts.ready);
    await page.waitForTimeout(250);
    const stable = await stableFrame(page, folder);
    assert.equal(stable.first, stable.second, `unstable screenshot for ${id}`);
    const messageIndexRail = battle ? null : verifyMessageIndexRail(faction, viewport, stable.bytes);
    const probes = battle
      ? [{ type: "production-tactical-entry", system: ready.system,
        attacker_ships: ready.attacker_ships, defender_ships: ready.defender_ships,
        fighters: ready.fighters }, ...(scenario.camera_journey
        ? await probeTacticalCameraJourney(page, viewport, folder, stable)
        : scenario.lod_journey
        ? await probeTacticalLodJourney(page, viewport, folder, stable)
        : scenario.slug === "battle-entry"
        ? await probeTactical(page, viewport, folder, stable)
        : scenario.fighter_detail_journey
        ? await probeTacticalFighterDetailJourney(
          page, viewport, folder, stable, consoleLines, ready,
        )
        : scenario.group_presentation
        ? await probeTacticalGroupPresentation(
          page, viewport, folder, stable, consoleLines, ready, faction,
        )
        : scenario.selected_navigation_presentation
        ? await probeTacticalSelectedNavigationPresentation(
          page, viewport, folder, stable, consoleLines, ready, faction,
        )
        : scenario.navigation_camera_presentation
        ? await probeTacticalNavigationCameraPresentation(
          page, viewport, folder, stable, consoleLines, ready,
        )
        : scenario.battle_alert_presentation
        ? await probeTacticalBattleAlertEntry(
          page, viewport, folder, stable, consoleLines, ready, faction,
        )
        : scenario.tactical_audio_presentation
        ? probeTacticalAudioRouting(folder, stable, consoleLines, ready)
        : scenario.detail_escort_presentation
        ? await probeTacticalDetailEscortPresentation(
          page, viewport, folder, stable, consoleLines, ready,
        )
        : scenario.effect_presentation
        ? await probeTacticalEffectPresentation(folder, stable, consoleLines, ready)
        : scenario.projectile_field_presentation
        ? await probeTacticalProjectileFieldPresentation(folder, stable, consoleLines, ready)
        : scenario.subsystem_field_command_presentation
        ? probeTacticalSubsystemFieldCommandPresentation(viewport, folder, stable, ready)
        : scenario.live_subsystem_damage_presentation
        ? probeTacticalLiveSubsystemDamagePresentation(viewport, folder, stable, ready)
        : scenario.subsystem_repair_mobility_presentation
        ? probeTacticalSubsystemRepairMobilityPresentation(
          viewport, folder, stable, ready, consoleLines,
        )
        : scenario.maneuver_movement_presentation
        ? probeTacticalManeuverMovementPresentation(viewport, folder, stable, ready)
        : scenario.command_assignment_presentation
        ? await probeTacticalCommandAssignmentPresentation(
          page, viewport, folder, stable, consoleLines, ready,
        )
        : scenario.command_execution_presentation
        ? probeTacticalCommandExecutionPresentation(viewport, folder, stable, ready)
        : scenario.command_progression_presentation
        ? probeTacticalCommandProgressionPresentation(
          viewport, folder, stable, consoleLines, ready,
        )
        : scenario.attack_targeting_presentation
        ? probeTacticalAttackTargetingPresentation(viewport, folder, stable, ready)
        : scenario.attack_target_lifecycle_presentation
        ? probeTacticalAttackTargetLifecyclePresentation(viewport, folder, stable, ready)
        : scenario.trench_run_outcome
        ? probeTacticalTrenchRunOutcome(
          folder, stable, consoleLines, ready, scenario.trench_run_outcome,
        )
        : scenario.death_star_laser_journey
        ? await probeTacticalDeathStarLaserJourney(
          page, viewport, folder, stable, consoleLines, ready,
        )
        : scenario.death_star_presentation
        ? await probeTacticalDeathStarPresentation(
          page, viewport, folder, stable, consoleLines, ready,
        )
        : scenario.battle_results_presentation
        ? await probeTacticalBattleResultsPresentation(
          page, viewport, folder, stable, consoleLines, faction,
        )
        : scenario.battle_options_presentation
        ? await probeTacticalBattleOptionsPresentation(
          page, viewport, folder, stable, consoleLines, faction,
          Boolean(scenario.battle_options_withdrawal),
        )
        : scenario.selected_damage_presentation
        ? probeTacticalSelectedDamagePresentation(viewport, folder, stable, ready)
        : scenario.empty_space_presentation
        ? [{
            type: "empty-space-tactical-viewport",
            backdrop: verifyTacticalBackdrop(viewport, stable.bytes, ready.palette_resource_id),
            planet_loaded: false,
            planet_submitted: false,
            screenshot_sha256: sha256(stable.bytes),
          }]
        : scenario.production_participants
        ? await probeTacticalProductionParticipants(
          page, viewport, folder, stable, consoleLines, faction, ready,
        )
        : scenario.tactical_proof
        ? [{ type: "tactical-lod-fixture", requested: ready.tactical_lod }]
        : [{ type: "tactical-3d-negative-control", proof_enabled: false }])]
      : await probeGid(page, faction, scenario, viewport, folder, consoleLines, ready);
    if (battle) {
      assert.equal(ready.schema_version, 35);
      assert.equal(ready.family, "tactical");
      assert.equal(ready.faction, faction);
      assert.equal(ready.proof_enabled, scenario.tactical_proof);
      assert.equal(ready.production_participants, Boolean(scenario.production_participants));
      assert.equal(ready.suppress_capital_fallback,
        Boolean(scenario.suppress_capital_fallback));
      assert.equal(ready.focus_player_fighter,
        Boolean(scenario.fighter_detail_journey || scenario.death_star_presentation));
      assert.equal(ready.group_presentation, Boolean(scenario.group_presentation));
      assert.equal(ready.effect_presentation, Boolean(scenario.effect_presentation));
      assert.equal(ready.projectile_field_presentation,
        Boolean(scenario.projectile_field_presentation));
      assert.equal(ready.selected_damage_presentation,
        Boolean(scenario.selected_damage_presentation));
      assert.equal(ready.selected_navigation_presentation,
        Boolean(scenario.selected_navigation_presentation));
      assert.equal(ready.navigation_camera_presentation,
        Boolean(scenario.navigation_camera_presentation));
      assert.equal(ready.empty_space_presentation,
        Boolean(scenario.empty_space_presentation));
      assert.equal(ready.detail_escort_presentation,
        Boolean(scenario.detail_escort_presentation));
      assert.equal(ready.battle_alert_presentation,
        Boolean(scenario.battle_alert_presentation));
      assert.equal(ready.tactical_audio_presentation,
        Boolean(scenario.tactical_audio_presentation));
      assert.equal(ready.subsystem_field_command_presentation,
        Boolean(scenario.subsystem_field_command_presentation));
      assert.equal(ready.live_subsystem_damage_presentation,
        Boolean(scenario.live_subsystem_damage_presentation));
      assert.equal(ready.subsystem_repair_mobility_presentation,
        Boolean(scenario.subsystem_repair_mobility_presentation));
      assert.equal(ready.maneuver_movement_presentation,
        Boolean(scenario.maneuver_movement_presentation));
      assert.equal(ready.command_assignment_presentation,
        Boolean(scenario.command_assignment_presentation));
      assert.equal(ready.command_execution_presentation,
        Boolean(scenario.command_execution_presentation));
      assert.equal(ready.command_progression_presentation,
        Boolean(scenario.command_progression_presentation));
      assert.equal(ready.attack_targeting_presentation,
        Boolean(scenario.attack_targeting_presentation));
      assert.equal(ready.attack_target_lifecycle_presentation,
        Boolean(scenario.attack_target_lifecycle_presentation));
      assert.equal(ready.death_star_presentation,
        Boolean(scenario.death_star_presentation));
      assert.equal(ready.battle_results_presentation,
        Boolean(scenario.battle_results_presentation));
      assert.equal(ready.battle_options_presentation,
        Boolean(scenario.battle_options_presentation));
      assert.equal(ready.battle_options_withdrawal,
        Boolean(scenario.battle_options_withdrawal));
      assert.equal(ready.death_star_laser_journey,
        Boolean(scenario.death_star_laser_journey));
      assert.equal(ready.trench_run_outcome, scenario.trench_run_outcome ?? null);
      assert.equal(ready.death_star === null, !scenario.death_star_presentation);
      assert.ok(Number.isSafeInteger(ready.system_picture_id)
        && ready.system_picture_id >= 1 && ready.system_picture_id <= 27,
      "tactical fixture lacks its SYSTEMSD picture identity");
      assert.equal(ready.planet_resource_id, ready.system_picture_id + 5500);
      assert.equal(ready.palette_resource_id, ready.system_picture_id + 5530);
      assert.ok(ready.attacker_ships > 0 && ready.defender_ships > 0);
      assert.ok(ready.fighters > 0);
      const groupLayout = faction === "alliance"
        ? { first_active_objects: 12, second_active_objects: 2 }
        : { first_active_objects: 2, second_active_objects: 12 };
      const effectLayout = faction === "alliance"
        ? { first_active_objects: 7, second_active_objects: 2 }
        : { first_active_objects: 2, second_active_objects: 7 };
      const projectileFieldLayout = { first_active_objects: 4, second_active_objects: 4 };
      assert.deepEqual(ready.source_layout, scenario.group_presentation ? {
        ...groupLayout,
        battle_extent: 136,
        outer_positive_z: 68,
        outer_negative_z: -68,
        inner_negative_z: -48,
        inner_positive_z: 48,
      } : scenario.effect_presentation ? {
        ...effectLayout,
        battle_extent: 121,
        outer_positive_z: 60.5,
        outer_negative_z: -60.5,
        inner_negative_z: -40.5,
        inner_positive_z: 40.5,
      } : scenario.projectile_field_presentation
        || scenario.detail_escort_presentation
        || scenario.subsystem_field_command_presentation
        || scenario.live_subsystem_damage_presentation
        || scenario.subsystem_repair_mobility_presentation
        || scenario.maneuver_movement_presentation
        || scenario.command_execution_presentation
        || scenario.selected_navigation_presentation
        || scenario.navigation_camera_presentation ? {
        ...projectileFieldLayout,
        battle_extent: 112,
        outer_positive_z: 56,
        outer_negative_z: -56,
        inner_negative_z: -36,
        inner_positive_z: 36,
      } : scenario.command_progression_presentation
        || scenario.attack_targeting_presentation
        || scenario.attack_target_lifecycle_presentation ? {
        first_active_objects: 5,
        second_active_objects: 5,
        battle_extent: 115,
        outer_positive_z: 57.5,
        outer_negative_z: -57.5,
        inner_negative_z: -37.5,
        inner_positive_z: 37.5,
      } : scenario.command_assignment_presentation
        || scenario.battle_options_withdrawal ? {
        first_active_objects: 3,
        second_active_objects: 3,
        battle_extent: 109,
        outer_positive_z: 54.5,
        outer_negative_z: -54.5,
        inner_negative_z: -34.5,
        inner_positive_z: 34.5,
      } : scenario.battle_results_presentation ? {
        first_active_objects: 6,
        second_active_objects: 6,
        battle_extent: 118,
        outer_positive_z: 59,
        outer_negative_z: -59,
        inner_negative_z: -39,
        inner_positive_z: 39,
      } : {
        first_active_objects: 2,
        second_active_objects: 2,
        battle_extent: 106,
        outer_positive_z: 53,
        outer_negative_z: -53,
        inner_negative_z: -33,
        inner_positive_z: 33,
      });
      assert.equal(ready.participants.length,
        scenario.group_presentation ? 14
          : scenario.effect_presentation ? 9
          : scenario.projectile_field_presentation
            || scenario.detail_escort_presentation
            || scenario.subsystem_field_command_presentation
            || scenario.live_subsystem_damage_presentation
            || scenario.subsystem_repair_mobility_presentation
            || scenario.maneuver_movement_presentation
            || scenario.command_execution_presentation
            || scenario.selected_navigation_presentation
            || scenario.navigation_camera_presentation ? 8
            : scenario.command_progression_presentation
              || scenario.attack_targeting_presentation
              || scenario.attack_target_lifecycle_presentation ? 10
              : scenario.command_assignment_presentation
                || scenario.battle_options_withdrawal ? 6
                : scenario.battle_results_presentation ? 12 : 4);
      const expectedParticipantLanes = new Map([
        ["capital-ship:alliance", -53],
        ["capital-ship:empire", 53],
        ["fighter-group:alliance", -33],
        ["fighter-group:empire", 33],
      ]);
      const expectedParticipantResources = new Map([
        ["capital-ship:alliance", {
          datId: 64, ordinal: 0, base: 2010, opposingBase: null,
          close: null, far: null, indicator: null,
        }],
        ["capital-ship:empire", {
          datId: 128, ordinal: 15, base: 2510, opposingBase: null,
          close: null, far: null, indicator: null,
        }],
        ["fighter-group:alliance", {
          datId: 1, ordinal: 31, base: 4020, opposingBase: 4024,
          close: 4024, far: 4029, indicator: 4204,
        }],
        ["fighter-group:empire", {
          datId: 5, ordinal: 33, base: 4100, opposingBase: 4104,
          close: 4104, far: 4109, indicator: 4204,
        }],
      ]);
      if (scenario.group_presentation) {
        const playerShips = ready.participants.filter((participant) =>
          participant.kind === "capital-ship" && participant.faction === faction);
        const opponentShips = ready.participants.filter((participant) =>
          participant.kind === "capital-ship" && participant.faction !== faction);
        const playerFighters = ready.participants.filter((participant) =>
          participant.kind === "fighter-group" && participant.faction === faction);
        const opponentFighters = ready.participants.filter((participant) =>
          participant.kind === "fighter-group" && participant.faction !== faction);
        assert.deepEqual(playerShips.map(({ fleet_roster_index }) => fleet_roster_index),
          [0, 1, 2, 3, 4, 5, 6, 7]);
        assert.deepEqual(playerShips.map(({ task_force }) => task_force),
          [0, 1, 2, 3, 4, 5, 6, 7]);
        assert.equal(opponentShips.length, 1);
        assert.deepEqual(playerFighters.map(({ fleet_roster_index }) => fleet_roster_index),
          [0, 1, 2, 3]);
        assert.deepEqual(playerFighters.map(({ fighter_group }) => fighter_group), [0, 1, 2, 3]);
        assert.equal(opponentFighters.length, 1);
        const playerBase = faction === "alliance" ? 4020 : 4100;
        for (const fighter of playerFighters) {
          assert.equal(fighter.active_close_resource, playerBase + fighter.fighter_group);
          assert.equal(fighter.active_far_resource, playerBase + fighter.fighter_group + 5);
          assert.equal(fighter.active_indicator_resource, 4200 + fighter.fighter_group);
        }
        assert.equal(opponentFighters[0].active_close_resource,
          faction === "alliance" ? 4104 : 4024);
        assert.equal(opponentFighters[0].active_far_resource,
          faction === "alliance" ? 4109 : 4029);
        assert.equal(opponentFighters[0].active_indicator_resource, 4204);
        const playerShipZ = faction === "alliance" ? -68 : 68;
        const playerFighterZ = faction === "alliance" ? -48 : 48;
        assert.ok(playerShips.every(({ source_position }) => source_position[2] === playerShipZ));
        assert.ok(playerFighters.every(({ source_position }) =>
          source_position[2] === playerFighterZ));
      } else if (scenario.effect_presentation) {
        const playerShips = ready.participants.filter((participant) =>
          participant.kind === "capital-ship" && participant.faction === faction);
        const opponentShips = ready.participants.filter((participant) =>
          participant.kind === "capital-ship" && participant.faction !== faction);
        const playerFighters = ready.participants.filter((participant) =>
          participant.kind === "fighter-group" && participant.faction === faction);
        const opponentFighters = ready.participants.filter((participant) =>
          participant.kind === "fighter-group" && participant.faction !== faction);
        assert.deepEqual(playerShips.map(({ fleet_roster_index }) => fleet_roster_index),
          [0, 1, 2, 3, 4, 5]);
        assert.equal(opponentShips.length, 1);
        assert.equal(playerFighters.length, 1);
        assert.equal(opponentFighters.length, 1);
        const playerShipZ = faction === "alliance" ? -60.5 : 60.5;
        const playerFighterZ = faction === "alliance" ? -40.5 : 40.5;
        assert.ok(playerShips.every(({ source_position: position }) => position[2] === playerShipZ));
        assert.ok(playerFighters.every(({ source_position: position }) =>
          position[2] === playerFighterZ));
      } else if (scenario.projectile_field_presentation
        || scenario.detail_escort_presentation
        || scenario.subsystem_field_command_presentation
        || scenario.live_subsystem_damage_presentation
        || scenario.subsystem_repair_mobility_presentation
        || scenario.maneuver_movement_presentation
        || scenario.command_execution_presentation
        || scenario.selected_navigation_presentation
        || scenario.navigation_camera_presentation
        || scenario.command_progression_presentation
        || scenario.attack_targeting_presentation
        || scenario.attack_target_lifecycle_presentation) {
        const allianceShips = ready.participants.filter((participant) =>
          participant.kind === "capital-ship" && participant.faction === "alliance");
        const empireShips = ready.participants.filter((participant) =>
          participant.kind === "capital-ship" && participant.faction === "empire");
        const allianceFighters = ready.participants.filter((participant) =>
          participant.kind === "fighter-group" && participant.faction === "alliance");
        const empireFighters = ready.participants.filter((participant) =>
          participant.kind === "fighter-group" && participant.faction === "empire");
        assert.deepEqual(allianceShips.map(({ fleet_roster_index }) => fleet_roster_index),
          [0, 1, 2]);
        assert.deepEqual(empireShips.map(({ fleet_roster_index }) => fleet_roster_index),
          [0, 1, 2]);
        const expectedFighters = scenario.command_progression_presentation
          || scenario.attack_targeting_presentation
          || scenario.attack_target_lifecycle_presentation ? 2 : 1;
        assert.equal(allianceFighters.length, expectedFighters);
        assert.equal(empireFighters.length, expectedFighters);
        if (scenario.command_progression_presentation
          || scenario.attack_targeting_presentation
          || scenario.attack_target_lifecycle_presentation) {
          assert.ok(allianceShips.every(({ fleet_roster_index }) =>
            [0, 1, 2].includes(fleet_roster_index)));
          assert.ok(empireShips.every(({ fleet_roster_index }) =>
            [0, 1, 2].includes(fleet_roster_index)));
        } else {
          const movedFaction = scenario.maneuver_movement_presentation ? faction : null;
          assert.ok(allianceShips.filter(({ active }) => active).every(({ source_position: position }) =>
            position[2] === (movedFaction === "alliance" ? -36.3125 : -56)),
          "Alliance retained formation did not follow its integrated leader");
          assert.ok(empireShips.filter(({ active }) => active).every(({ source_position: position }) =>
            position[2] === (movedFaction === "empire" ? 36.3125 : 56)),
          "Imperial retained formation did not follow its integrated leader");
          assert.equal(allianceFighters[0].source_position[2], -36);
          assert.equal(empireFighters[0].source_position[2], 36);
        }
      } else if (scenario.death_star_presentation
        || scenario.battle_options_presentation
        || scenario.command_assignment_presentation) {
        const allianceShips = ready.participants.filter((participant) =>
          participant.kind === "capital-ship" && participant.faction === "alliance");
        const empireShips = ready.participants.filter((participant) =>
          participant.kind === "capital-ship" && participant.faction === "empire");
        const allianceFighters = ready.participants.filter((participant) =>
          participant.kind === "fighter-group" && participant.faction === "alliance");
        const empireFighters = ready.participants.filter((participant) =>
          participant.kind === "fighter-group" && participant.faction === "empire");
        const expectedShips = scenario.battle_options_withdrawal ? 2 : 1;
        assert.equal(allianceShips.length, expectedShips);
        assert.equal(empireShips.length, expectedShips);
        const expectedFighters = scenario.command_assignment_presentation ? 2 : 1;
        assert.equal(allianceFighters.length, expectedFighters);
        assert.equal(empireFighters.length, expectedFighters);
        const allianceShipZ = scenario.death_star_laser_journey && faction === "empire"
          ? ready.source_layout.outer_positive_z
          : ready.source_layout.outer_negative_z;
        assert.ok(allianceShips.every(({ source_position: position }) =>
          position[2] === allianceShipZ));
        assert.ok(empireShips.every(({ source_position: position }) =>
          position[2] === ready.source_layout.outer_positive_z));
        assert.ok(allianceFighters.every(({ source_position: position }) =>
          position[2] === ready.source_layout.inner_negative_z));
        assert.ok(empireFighters.every(({ source_position: position }) =>
          position[2] === ready.source_layout.inner_positive_z));
      } else if (scenario.battle_results_presentation) {
        const allianceShips = ready.participants.filter((participant) =>
          participant.kind === "capital-ship" && participant.faction === "alliance");
        const empireShips = ready.participants.filter((participant) =>
          participant.kind === "capital-ship" && participant.faction === "empire");
        const allianceFighters = ready.participants.filter((participant) =>
          participant.kind === "fighter-group" && participant.faction === "alliance");
        const empireFighters = ready.participants.filter((participant) =>
          participant.kind === "fighter-group" && participant.faction === "empire");
        assert.equal(allianceShips.length, 3);
        assert.equal(empireShips.length, 3);
        assert.equal(allianceFighters.length, 3);
        assert.equal(empireFighters.length, 3);
        assert.ok(allianceShips.every(({ source_position: position }) =>
          position[2] === ready.source_layout.outer_negative_z));
        assert.ok(empireShips.every(({ source_position: position }) =>
          position[2] === ready.source_layout.outer_positive_z));
        assert.ok(allianceFighters.every(({ source_position: position }) =>
          position[2] === ready.source_layout.inner_negative_z));
        assert.ok(empireFighters.every(({ source_position: position }) =>
          position[2] === ready.source_layout.inner_positive_z));
      } else {
        for (const participant of ready.participants) {
          const key = `${participant.kind}:${participant.faction}`;
          assert.ok(expectedParticipantLanes.has(key), `unexpected participant ${key}`);
          const expectedResource = expectedParticipantResources.get(key);
          assert.equal(participant.class_dat_id, expectedResource.datId,
            `${key} has the wrong stable DAT identity`);
          assert.equal(participant.fleet_roster_index, 0);
          assert.equal(participant.tactical_ordinal, expectedResource.ordinal,
            `${key} has the wrong tactical ordinal`);
          assert.equal(participant.resource_base, expectedResource.base,
            `${key} has the wrong tactical resource base`);
          assert.equal(participant.opposing_resource_base, expectedResource.opposingBase,
            `${key} has the wrong opposing tactical resource base`);
          assert.equal(participant.initial_close_resource, expectedResource.close,
            `${key} has the wrong initial close resource`);
          assert.equal(participant.initial_far_resource, expectedResource.far,
            `${key} has the wrong initial far resource`);
          assert.equal(participant.initial_indicator_resource, expectedResource.indicator,
            `${key} has the wrong initial indicator resource`);
          assert.deepEqual(participant.source_position,
            [-0, 0, expectedParticipantLanes.get(key)], `${key} has the wrong source position`);
        }
      }
      probes.push({
        type: "source-bound-tactical-participants",
        executable_functions: ["FUN_005ab650", "FUN_005a9030"],
        layout: ready.source_layout,
        participants: ready.participants,
      });
      const packLog = consoleLines.find(({ text }) => text.includes("runtime_asset_pack loaded"));
      assert.match(packLog?.text || "", /tactical_meshes=87 tactical_textures=397/,
        "runtime pack did not install the complete tactical object corpus");
      const familyLogs = consoleLines.filter(({ text }) =>
        text.includes("[tactical_3d] family_loaded base=2560"));
      const lodLogs = consoleLines.filter(({ text }) =>
        text.includes("[tactical_3d] lod_selection"));
      const cameraLogs = consoleLines.filter(({ text }) =>
        text.includes("[tactical_3d] camera_source"));
      const layoutLogs = consoleLines.filter(({ text }) =>
        text.includes("[tactical_3d] layout_source"));
      const participantFamilyLogs = consoleLines.filter(({ text }) =>
        text.includes("[tactical_3d] participant_family_loaded"));
      const participantSceneLogs = consoleLines.filter(({ text }) =>
        text.includes("[tactical_3d] participant_scene"));
      const participantBoundsLogs = consoleLines.filter(({ text }) =>
        text.includes("[tactical_3d] participant_bounds"));
      const fighterFamilyLogs = consoleLines.filter(({ text }) =>
        text.includes("[tactical_3d] fighter_family_loaded"));
      const fighterSceneLogs = consoleLines.filter(({ text }) =>
        text.includes("[tactical_3d] fighter_scene"));
      const planetLoadLogs = consoleLines.filter(({ text }) =>
        text.includes("[tactical_3d] planet_loaded"));
      const planetSceneLogs = consoleLines.filter(({ text }) =>
        text.includes("[tactical_3d] planet_scene"));
      const expectsPlanet = Boolean(
        scenario.production_participants || scenario.suppress_capital_fallback,
      ) && !scenario.battle_results_presentation && !scenario.empty_space_presentation;
      assert.equal(planetLoadLogs.length, expectsPlanet ? 1 : 0,
        "tactical scene loaded the wrong number of system-selected planets");
      assert.equal(planetSceneLogs.length, expectsPlanet ? 1 : 0,
        "tactical scene submitted the wrong number of system-selected planets");
      if (expectsPlanet) {
        assert.match(planetLoadLogs[0]?.text || "",
          new RegExp(`resource=${ready.planet_resource_id} `
            + `palette_resource_id=${ready.palette_resource_id} dimensions=256x256 `
            + "texture_filter=nearest alpha=opaque "
            + "source=FUN_0059a850,FUN_00596ad0,FUN_005c2e60"),
        "tactical planet load used the wrong resource, palette, or source state");
        assert.match(planetSceneLogs[0]?.text || "",
          new RegExp(`resource=${ready.planet_resource_id} `
            + `palette_resource_id=${ready.palette_resource_id} dimensions=256x256 `
            + "source_position=left_edge_provisional "),
        "tactical planet scene used the wrong resource or provisional placement");
        probes.push({
          type: "system-selected-tactical-planet",
          executable_functions: [
            "FUN_0040b0e0", "FUN_00509610", "FUN_00595d60", "FUN_0059a850",
            "FUN_00596ad0", "FUN_005c2e60",
          ],
          system_picture_id: ready.system_picture_id,
          planet_resource_id: ready.planet_resource_id,
          palette_resource_id: ready.palette_resource_id,
          dimensions: [256, 256],
          texture_filter: "nearest",
          placement_acceptance: "provisional-A1-only",
        });
      }
      if (scenario.tactical_proof) {
        assert.equal(familyLogs.length, 1,
          "source-bound tactical LOD family did not emit exactly one load event");
        assert.match(familyLogs[0]?.text || "",
          new RegExp(`resources=2560,2561,2562 textures=SDESTI52\\.BMP,SDESTI_M\\.BMP `
            + `palette_selector=${ready.system_picture_id} `
            + `palette_resource_id=${ready.palette_resource_id} palette_flags=68 `
            + `transform=authored_xyz_z_reflection .*family_loads=1`),
          "source-bound tactical LOD family did not load exactly once");
        assert.match(familyLogs[0]?.text || "",
          /light_directional_rgb=0\.8 light_ambient_rgb=0\.5 light_frame_source=5,5,-1 surface_to_light_rh=0\.70014006,0\.70014006,0\.14002801 light_constraint=z/,
          "source-traced tactical light rig did not preserve its retained-mode contract");
        assert.match(familyLogs[0]?.text || "",
          /render_quality=gouraud device_dither=false texture_filter=nearest mip_filter=none source_cull=d3dcull_ccw target_cull=back_cw depth_test=less_equal depth_write=true specular=false material=diffuse_plus_emissive/,
          "source-traced tactical renderer did not preserve its retained-mode device state");
        const expectedResources = scenario.expected_lod_sequence
          || [scenario.expected_lod_resource];
        assert.equal(lodLogs.length, expectedResources.length,
          "source-derived tactical LOD fixture emitted the wrong selection-event count");
        const selectedResources = lodLogs.map(({ text }) => {
          assert.match(text, /family_loads=1/,
            "tactical LOD selection observed a reloaded family");
          const match = text.match(/resource_id=(\d+)/);
          assert.ok(match, "tactical LOD selection omitted its resource identity");
          return Number(match[1]);
        });
        assert.deepEqual(selectedResources, expectedResources,
          "source-derived tactical LOD selection chose the wrong resource sequence");
        probes.push({
          type: "source-bound-tactical-3d-lod-family",
          meshes: ["2560/1033", "2561/1033", "2562/1033"],
          textures: ["SDESTI52.BMP/1033", "SDESTI_M.BMP/1033"],
          selected_resources: selectedResources,
          palette: {
            system_picture_id: ready.system_picture_id,
            resource_id: ready.palette_resource_id,
            flags: 68,
          },
          transform: "authored_xyz_z_reflection",
          family_loads: 1,
        });
        probes.push({
          type: "source-traced-tactical-light-rig",
          executable_function: "FUN_005d4d10",
          direct3drm: {
            directional: {
              type: 3,
              rgb: [0.8, 0.8, 0.8],
              frame_source: [5, 5, -1],
              target_source: [0, 0, 0],
              constraint: "z",
            },
            ambient: {
              type: 0,
              rgb: [0.5, 0.5, 0.5],
            },
          },
          surface_to_light_rh: [0.70014006, 0.70014006, 0.14002801],
        });
        probes.push({
          type: "source-traced-tactical-render-state",
          executable_functions: ["FUN_005c1c10", "FUN_005d6e10"],
          direct3drm: {
            dither: false,
            render_quality: "gouraud",
            texture_filter: "nearest",
            mip_filter: "none",
            source_cull: "d3dcull_ccw",
            target_cull: "back_cw",
            depth_test: "less_equal",
            depth_write: true,
            specular: false,
            material: "diffuse_plus_emissive",
          },
        });
        if (scenario.camera_journey) {
          assert.equal(layoutLogs.length, 1,
            "source camera journey emitted the wrong layout-event count");
          assert.match(layoutLogs[0]?.text || "",
            /first_active_objects=2 second_active_objects=2 battle_extent=106 outer_positive_z=53 outer_negative_z=-53 inner_negative_z=-33 inner_positive_z=33 source=FUN_005ab650/,
            "source camera journey did not use the fixture force composition");
          assert.equal(cameraLogs.length, 8,
            "source camera journey emitted the wrong state count");
          const states = cameraLogs.map(({ text }) => {
            const value = (label) => {
              const match = text.match(new RegExp(`${label}=(-?[0-9.]+)`));
              assert.ok(match, `source camera log omitted ${label}`);
              return Number(match[1]);
            };
            assert.match(text, /handedness=lh_y_up_to_rh_y_up/);
            return {
              pitch: value("pitch"),
              yaw: value("yaw"),
              field: value("field"),
              zoom_step: value("zoom_step"),
              orbit_step: value("orbit_step"),
              distance: value("distance"),
              near: value("near"),
              far: value("far"),
              fovy_radians: value("fovy_radians"),
              target_object_id: value("target_object_id"),
              target_x: value("target_x"),
              target_y: value("target_y"),
              target_z: value("target_z"),
            };
          });
          const initialYaw = faction === "alliance" ? -30 : 150;
          assert.deepEqual(states.map(({ pitch }) => pitch), [30, 30, 30, 30, 30, 35, 30, 30]);
          assert.deepEqual(states.map(({ yaw }) => yaw),
            [initialYaw, initialYaw, initialYaw, initialYaw - 5, initialYaw, initialYaw, initialYaw,
              initialYaw]);
          assert.deepEqual(states.map(({ zoom_step }) => zoom_step), [5, 4, 5, 5, 5, 5, 5, 5]);
          assert.deepEqual(states.map(({ orbit_step }) => orbit_step), [5, 4, 5, 5, 5, 5, 5, 5]);
          assert.ok(Math.abs(states[0].field - 0.2) < 1e-6);
          assert.ok(Math.abs(states[1].field - 0.18) < 1e-6);
          assert.ok(Math.abs(states[2].field - 0.198) < 1e-6);
          assert.ok(states.every(({ distance, near, far }) =>
            Math.abs(distance - 180.2) < 1e-4 && near === 1 && Math.abs(far - 450.5) < 1e-4));
          const selectedObjectId = faction === "alliance" ? 1 : 2;
          assert.deepEqual(states.map(({ target_object_id }) => target_object_id),
            [0, 0, 0, 0, 0, 0, 0, selectedObjectId]);
          assert.ok(states.slice(0, -1).every(({ target_x, target_y, target_z }) =>
            target_x === 0 && target_y === 0 && target_z === 0));
          assert.equal(states.at(-1).target_x, -0);
          assert.equal(states.at(-1).target_y, 0);
          assert.equal(states.at(-1).target_z, faction === "alliance" ? 53 : -53);
          probes.push({
            type: "source-traced-tactical-camera-journey",
            executable_functions: [
              "FUN_005d9490", "FUN_005d9620", "FUN_005d9640", "FUN_00595be0",
              "FUN_005c1080", "0x005d97c0",
            ],
            states,
          });
          probes.push({
            type: "source-traced-tactical-layout",
            executable_function: "FUN_005ab650",
            active_objects: { first_force: 2, second_force: 2 },
            battle_extent: 106,
            z_lanes: [53, -53, -33, 33],
          });
        } else {
          assert.equal(cameraLogs.length, 0,
            "non-camera tactical proof unexpectedly enabled the source camera");
          assert.equal(layoutLogs.length, 0,
            "non-camera tactical proof unexpectedly enabled the source layout");
        }
      } else {
        assert.equal(familyLogs.length, 0, "negative control loaded the tactical LOD family");
        assert.equal(lodLogs.length, 0, "negative control submitted a tactical LOD mesh");
        if (scenario.production_participants) {
          if (!scenario.battle_results_presentation && !scenario.tactical_audio_presentation) {
          assert.equal(participantFamilyLogs.length, 2,
            "production participants did not load exactly two source families");
          assert.deepEqual(participantFamilyLogs.map(({ text }) => {
            const match = text.match(/base=(\d+)/);
            assert.ok(match, "participant family log omitted its base");
            return Number(match[1]);
          }).sort((a, b) => a - b), [2010, 2510]);
          assert.equal(participantSceneLogs.length, 1,
            "production participants did not emit one scene submission");
          const expectedShipCount = scenario.detail_escort_presentation
            ? ready.participants.filter(({ kind, active }) =>
              kind === "capital-ship" && active).length
            : ready.attacker_ships + ready.defender_ships;
          assert.match(participantSceneLogs[0].text,
            new RegExp(`requested=${expectedShipCount} rendered=${expectedShipCount} `
              + "families=2010,2510 .*source_positions=true"),
          "production participant scene used the wrong resource families or positions");
          assert.equal(participantBoundsLogs.length, 1,
            "production participants did not emit projected mesh bounds");
          assert.match(participantBoundsLogs[0].text, /source_projection=true/,
            "production participant bounds omitted source-projected ships");
          const fighterParticipants = ready.participants
            .filter(({ kind, active }) => kind === "fighter-group" && active);
          const expectedFighterFamilies = [...new Map(fighterParticipants.map((participant) => [
            participant.active_close_resource,
            [participant.active_close_resource, participant.active_far_resource,
              participant.active_indicator_resource],
          ])).values()].sort((left, right) => left[0] - right[0]);
          assert.equal(fighterFamilyLogs.length, expectedFighterFamilies.length,
            "production participants loaded the wrong number of fighter families");
          assert.deepEqual(fighterFamilyLogs.map(({ text }) => {
            const match = text.match(/close=(\d+) far=(\d+) indicator=(\d+)/);
            assert.ok(match, "fighter family log omitted its exact resources");
            return match.slice(1).map(Number);
          }).sort((a, b) => a[0] - b[0]), expectedFighterFamilies);
          assert.ok(fighterFamilyLogs.every(({ text }) =>
            /dimensions=32x32,16x16,2x2 .*texture_filter=nearest alpha=opaque source=FUN_005c63f0/.test(text)),
          "fighter family load did not preserve dimensions or source texture state");
          assert.equal(fighterSceneLogs.length, scenario.fighter_detail_journey ? 9 : 1,
            "production participants emitted the wrong fighter scene count");
          assert.match(fighterSceneLogs[0].text,
            new RegExp(`requested=${fighterParticipants.length} rendered=${fighterParticipants.length} `
              + ".*source_positions=true source=FUN_005ab650,FUN_005c63f0,0x005d4af0"),
            "production fighter scene used the wrong resource families, detail state, or positions");
          const initialFighterResources = parseFighterSceneResources(fighterSceneLogs[0]);
          assert.deepEqual(initialFighterResources.map(({ resource_id }) => resource_id),
            fighterParticipants.map(({ active_indicator_resource }) => active_indicator_resource));
          if (scenario.navigation_camera_presentation) {
            assert.ok(cameraLogs.length >= 6,
              "navigation journey did not exercise the source tactical camera");
          } else if (scenario.group_presentation) {
            assert.equal(cameraLogs.length, 3,
              "group journey did not preserve its initial and two hotkey camera states");
          } else {
            assert.equal(cameraLogs.length, scenario.fighter_detail_journey ? 40 : 1,
              "production participants did not use the source tactical camera");
          }
          assert.equal(layoutLogs.length, 1,
            "production participants did not use the source tactical layout");
          probes.push({
            type: "production-tactical-resource-families",
            families: [2010, 2510],
            selected_resources: [2012, 2512],
            source_positions: true,
          });
          probes.push({
            type: "production-tactical-fighter-families",
            close_resources: fighterParticipants.map(({ active_close_resource }) =>
              active_close_resource),
            far_resources: fighterParticipants.map(({ active_far_resource }) =>
              active_far_resource),
            selected_resources: fighterParticipants.map(({ active_indicator_resource }) =>
              active_indicator_resource),
            source_thresholds: [5, 10],
            source_positions: true,
          });
          }
        } else {
          assert.equal(participantFamilyLogs.length, 0,
            "negative control loaded a production participant family");
          assert.equal(participantSceneLogs.length, 0,
            "negative control submitted production participants");
          assert.equal(participantBoundsLogs.length, 0,
            "negative control projected production participant bounds");
          assert.equal(fighterFamilyLogs.length, 0,
            "negative control loaded production fighter families");
          assert.equal(fighterSceneLogs.length, 0,
            "negative control submitted production fighter groups");
        }
      }
    }
    const runtimeMeasurements = battle ? await measureBrowserRuntime(page) : null;
    if (battle) {
      assert.ok(Number.isFinite(runtimeMeasurements.frame_interval_ms.p95)
        && runtimeMeasurements.frame_interval_ms.p95 < 100,
      "paused tactical render stalled during the 60-frame timing sample");
      assert.ok(Number.isSafeInteger(runtimeMeasurements.wasm_memory_bytes)
        && runtimeMeasurements.wasm_memory_bytes > 0,
      "WASM cold-load memory was not observable");
      probes.push({ type: "browser-runtime-measurements", ...runtimeMeasurements });
    }
    const comparison = compareScreenshot(id, stable.bytes, folder);
    const interactionComparisons = {};
    for (const capture of fs.readdirSync(folder)
      .filter((name) => name.endsWith(".png")
        && !["frame-a.png", "actual.png"].includes(name)
        && !name.startsWith("expected-") && !name.startsWith("diff-"))) {
      const name = capture.slice(0, -4);
      interactionComparisons[name] = compareScreenshot(
        id, fs.readFileSync(path.join(folder, capture)), folder, name,
      );
    }

    const expectedRequests = ["/", "/gl.js", "/open-rebellion-test.wasm", "/data/runtime.orpk"];
    assert.deepEqual(requests.map(({ url: pathname }) => pathname).sort(), expectedRequests.sort());
    assert.ok(requests.every(({ status }) => status === 200), "startup has non-200 requests");
    assert.deepEqual(errors, [], `browser diagnostics for ${id}: ${errors.join("; ")}`);
    if (["mismatch", "mismatch_not_updated", "dimension_mismatch"].includes(comparison.status)) {
      fail(`pixel comparison failed for ${id}: ${comparison.different_pixels} pixels`);
    }
    for (const [capture, stateComparison] of Object.entries(interactionComparisons)) {
      if (["mismatch", "mismatch_not_updated", "dimension_mismatch"].includes(stateComparison.status)) {
        fail(`interaction pixel comparison failed for ${id}/${capture}: ${stateComparison.different_pixels} pixels`);
      }
    }

    result = {
      schema_version: schemaVersion,
      id,
      launch_attempts: launchAttempts,
      status: "pass",
      fixture_code: code,
      browser_version: browser.version(),
      browser_executable: executable,
      launch_arguments: browserManifest.launch_arguments,
      profile: "new-process-and-temporary-profile-per-scenario",
      muted: true,
      viewport,
      ready,
      screenshot_sha256: stable.second,
      two_frame_hashes: { first: stable.first, second: stable.second, equal: stable.first === stable.second },
      message_index_rail: messageIndexRail,
      runtime_measurements: runtimeMeasurements,
      wasm_sha256: sha256(fs.readFileSync(path.join(site, "open-rebellion-test.wasm"))),
      runtime_pack_sha256: sha256(fs.readFileSync(path.join(site, "data/runtime.orpk"))),
      requests,
      errors,
      console: consoleLines,
      interaction_logs: consoleLines.filter(({ text }) => text.includes("[interface] command=")),
      probes,
      comparison,
      interaction_comparisons: interactionComparisons,
      cleanup: "pending",
    };
  } catch (error) {
    result = {
      schema_version: schemaVersion,
      id,
      launch_attempts: launchAttempts,
      status: "fail",
      error: String(error.stack || error),
      requests,
      errors,
      console: consoleLines,
      cleanup: "pending",
    };
  } finally {
    if (page) await page.close().catch((error) => errors.push(`page-close:${error}`));
    if (context) await context.close().catch((error) => errors.push(`context-close:${error}`));
    if (browser) await browser.close().catch((error) => errors.push(`browser-close:${error}`));
    result.cleanup = errors.some((error) => error.includes("-close:")) ? "failed" : "closed";
    if (result.cleanup === "failed") {
      result.status = "fail";
      result.error = `${result.error || ""}\nBrowser cleanup failed: ${errors.filter((error) => error.includes("-close:")).join("; ")}`.trim();
    }
    fs.writeFileSync(path.join(folder, "result.json"), `${JSON.stringify(result, null, 2)}\n`);
  }
  return result;
}

function isRetryableBrowserTimeout(result) {
  return result.status === "fail"
    && /(?:TimeoutError|Timeout \d+ms exceeded)/.test(result.error || "");
}

async function runScenario(server, executable, scenario, faction, viewport) {
  const executionAttempts = [];
  let result;
  for (let attempt = 1; attempt <= 2; attempt++) {
    result = await runScenarioOnce(server, executable, scenario, faction, viewport);
    const retryableTimeout = isRetryableBrowserTimeout(result);
    executionAttempts.push({
      attempt,
      status: result.status,
      cleanup: result.cleanup,
      retryable_timeout: retryableTimeout,
      launch_attempts: result.launch_attempts,
      ...(result.error ? { error: result.error } : {}),
    });
    if (result.status === "pass" || !retryableTimeout || attempt === 2) break;
    await new Promise((resolve) => setTimeout(resolve, 1_000));
  }

  result.execution_attempts = executionAttempts;
  const folder = path.resolve(runDir, result.id);
  fs.writeFileSync(path.join(folder, "result.json"), `${JSON.stringify(result, null, 2)}\n`);
  return result;
}

async function main() {
  fs.mkdirSync(runDir, { recursive: true });
  if (battle) {
    assert.equal(catalog.family, "TAC-01..TAC-07");
    assert.equal(catalog.fixture_namespace, 1);
    assert.deepEqual(catalog.scenarios.map(({ slug }) => slug),
      ["battle-entry", "battle-entry-proof-off", "lod-close", "lod-medium", "lod-far",
        "lod-journey", "camera-journey", "production-participants",
        "production-fighter-detail-journey",
        "production-group-presentation",
        "production-effect-presentation",
        "production-projectile-field-presentation",
        "production-selected-damage-presentation",
        "production-subsystem-field-command-presentation",
        "production-live-subsystem-damage-presentation",
        "production-subsystem-repair-mobility-presentation",
        "production-maneuver-movement-presentation",
        "production-command-assignment-presentation",
        "production-command-execution-presentation",
        "production-command-progression-presentation",
        "production-attack-targeting-presentation",
        "production-attack-target-lifecycle-presentation",
        "production-death-star-presentation",
        "production-battle-results-presentation",
        "production-battle-options-presentation",
        "production-battle-options-withdrawal",
        "production-death-star-laser-journey",
        "production-trench-run-success",
        "production-trench-run-failure",
        "production-selected-navigation-presentation",
        "production-navigation-camera-presentation",
        "production-empty-space-presentation",
        "production-detail-escort-presentation",
        "production-battle-alert-entry",
        "production-tactical-audio-routing",
        "production-participants-3d-off"]);
    assert.deepEqual(catalog.scenarios.map(({ tactical_proof }) => tactical_proof),
      [true, false, true, true, true, true, true, ...Array(29).fill(false)]);
    assert.deepEqual(catalog.scenarios.map(({ expected_lod_resource }) => expected_lod_resource),
      [2560, undefined, 2560, 2561, 2562, 2560, 2560, undefined, undefined,
        undefined, undefined, undefined, undefined, undefined, undefined, undefined, undefined,
        undefined, undefined, undefined, undefined, undefined, undefined, undefined, undefined,
        undefined, undefined, undefined, undefined, undefined, undefined, undefined, undefined,
        undefined, undefined, undefined]);
    assert.deepEqual(catalog.scenarios.map(({ lod_journey }) => Boolean(lod_journey)),
      Array.from({ length: 36 }, (_, index) => index === 5));
    assert.deepEqual(catalog.scenarios.map(({ camera_journey }) => Boolean(camera_journey)),
      Array.from({ length: 36 }, (_, index) => index === 6));
    assert.deepEqual(catalog.factions, ["alliance", "empire"]);
  } else {
    execFileSync(process.execPath, [path.join(here, "validate-catalog.mjs")], { stdio: "inherit" });
  }
  if (!noBuild) {
    const build = spawnSync("bash", [path.join(root, "scripts/build-interface-test-wasm.sh")], {
      cwd: root,
      stdio: "inherit",
      env: {
        ...process.env,
        PATH: `/opt/homebrew/bin:/usr/bin:/bin:/usr/sbin:/sbin:${path.join(os.homedir(), ".cargo/bin")}`,
      },
    });
    if (build.status !== 0) fail(`interface-test build failed: exit ${build.status}`);
  }
  const executable = browserExecutable();
  let server;
  const results = [];
  let selectedFactions = [];
  let selectedViewports = [];
  try {
    server = await startServer();
    const scenarios = scenarioFilter
      ? catalog.scenarios.filter(({ slug }) => slug === scenarioFilter)
      : battle
      ? catalog.scenarios
      : smoke
      ? catalog.scenarios.filter(({ slug }) => ["popular-support", "galaxy"].includes(slug))
      : catalog.scenarios;
    if (!scenarios.length) fail(`unknown ${battle ? "tactical" : "GID"} scenario: ${scenarioFilter}`);
    selectedFactions = smoke ? ["alliance", "empire"] : catalog.factions;
    selectedViewports = smoke ? [catalog.viewports[0]] : catalog.viewports;
    for (const scenario of scenarios) {
      for (const faction of selectedFactions) {
        for (const viewport of selectedViewports) {
          const result = await runScenario(server, executable, scenario, faction, viewport);
          results.push(result);
          process.stdout.write(`${result.status === "pass" ? "PASS" : "FAIL"} ${result.id}\n`);
        }
      }
    }
  } finally {
    if (server) await new Promise((resolve) => server.close(resolve));
  }

  if (battle && !scenarioFilter) {
    verifyTacticalProofNegativeControls(results, selectedFactions, selectedViewports);
    verifyTacticalLodFamily(results, selectedFactions, selectedViewports);
    verifyTacticalProductionParticipants(results, selectedFactions, selectedViewports);
  }

  writeContactSheet(results);
  const browserAttempts = summarizeBrowserAttempts(results);
  const summary = {
    schema_version: schemaVersion,
    family: catalog.family,
    status: results.every(({ status }) => status === "pass") ? "pass" : "fail",
    run_id: runId,
    cases: results.length,
    executions: browserAttempts.browser_executions,
    passed: results.filter(({ status }) => status === "pass").length,
    failed: results.filter(({ status }) => status === "fail").length,
    screenshots_with_matching_goldens: results.filter(({ comparison }) => comparison?.status === "match").length,
    unbaselined: results.filter(({ comparison }) => comparison?.status === "unbaselined").length,
    unstable_screenshots: results.filter(({ two_frame_hashes }) => two_frame_hashes && !two_frame_hashes.equal).length,
    four_request_startups: results.filter(({ requests }) => requests?.length === 4).length,
    ...browserAttempts,
    raw_artifacts: path.relative(root, runDir),
  };
  fs.writeFileSync(path.join(runDir, "summary.json"), `${JSON.stringify(summary, null, 2)}\n`);
  process.stdout.write(`${JSON.stringify(summary, null, 2)}\n`);
  if (summary.status !== "pass") process.exitCode = 1;
}

main().catch((error) => {
  process.stderr.write(`${error.stack || error}\n`);
  process.exitCode = 1;
});
