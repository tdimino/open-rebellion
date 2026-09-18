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
import { launchBrowser } from "./browser-launch.mjs";

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
            expected: "command=0x133 destination=game_options status=pending_original_window" },
          { slug: "encyclopedia", rect: { x: 394, y: 405, width: 27, height: 16 },
            expected: "command=0x131 destination=encyclopedia status=pending_original_window" },
        ]
      : [
          { slug: "game-options", rect: { x: 79, y: 192, width: 35, height: 57 },
            expected: "command=0x133 destination=game_options status=pending_original_window" },
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
      assert.equal(sha256(released), sha256(fs.readFileSync(path.join(folder, "actual.png"))),
        `${faction}: ${command.slug} exposed a replacement destination`);
      probes.push({ type: "physical-command", command: command.slug, expected: command.expected,
        observed, hover_sha256: sha256(hover), pressed_sha256: sha256(pressed),
        released_sha256: sha256(released) });
    }

    for (const [key, expected] of [
      ["F1", "command=0x133 destination=game_options status=pending_original_window"],
      ["F7", "command=0x131 destination=encyclopedia status=pending_original_window"],
    ]) {
      const observed = await performCommand(page, expected, () => page.keyboard.press(key));
      probes.push({ type: "keyboard-command", key, expected, observed });
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
  const resumed = await capture("resumed");
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
  page, folder, stable, consoleLines, faction,
) {
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
  return [{
    type: "source-projected-tactical-interaction",
    player_object_id: playerObjectId,
    target_object_id: targetObjectId,
    deselected_sha256: sha256(deselected),
    selected_sha256: sha256(selected),
    targeted_sha256: sha256(targeted),
  }];
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
    panelProof = verifyTacticalBitmap(viewport, stable.bytes, 1302, 481, 27, [], [
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
  fs.writeFileSync(path.join(folder, "selected-damage.png"), stable.bytes);
  return [{
    type: "source-mapped-selected-capital-damage",
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
    /panel=missions event=commit order=5 tactic=0 capital_members=1 fighter_members=0/.test(line)),
  "capital mission did not commit Attack Capital Ships");
  assert.ok(orderLogs.some((line) =>
    /panel=missions event=commit order=2 tactic=0 capital_members=0 fighter_members=1/.test(line)),
  "fighter mission did not commit Recover");
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

function probeTacticalCommandProgressionPresentation(viewport, folder, stable, ready) {
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
    },
  }];
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

async function runScenario(server, executable, scenario, faction, viewport) {
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
        ? probeTacticalCommandProgressionPresentation(viewport, folder, stable, ready)
        : scenario.selected_damage_presentation
        ? probeTacticalSelectedDamagePresentation(viewport, folder, stable, ready)
        : scenario.production_participants
        ? await probeTacticalProductionParticipants(
          page, folder, stable, consoleLines, faction,
        )
        : scenario.tactical_proof
        ? [{ type: "tactical-lod-fixture", requested: ready.tactical_lod }]
        : [{ type: "tactical-3d-negative-control", proof_enabled: false }])]
      : await probeGid(page, faction, scenario, viewport, folder, consoleLines, ready);
    if (battle) {
      assert.equal(ready.schema_version, 20);
      assert.equal(ready.family, "tactical");
      assert.equal(ready.faction, faction);
      assert.equal(ready.proof_enabled, scenario.tactical_proof);
      assert.equal(ready.production_participants, Boolean(scenario.production_participants));
      assert.equal(ready.suppress_capital_fallback,
        Boolean(scenario.suppress_capital_fallback));
      assert.equal(ready.focus_player_fighter, Boolean(scenario.fighter_detail_journey));
      assert.equal(ready.group_presentation, Boolean(scenario.group_presentation));
      assert.equal(ready.effect_presentation, Boolean(scenario.effect_presentation));
      assert.equal(ready.projectile_field_presentation,
        Boolean(scenario.projectile_field_presentation));
      assert.equal(ready.selected_damage_presentation,
        Boolean(scenario.selected_damage_presentation));
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
        || scenario.subsystem_field_command_presentation
        || scenario.live_subsystem_damage_presentation
        || scenario.subsystem_repair_mobility_presentation
        || scenario.maneuver_movement_presentation
        || scenario.command_execution_presentation ? {
        ...projectileFieldLayout,
        battle_extent: 112,
        outer_positive_z: 56,
        outer_negative_z: -56,
        inner_negative_z: -36,
        inner_positive_z: 36,
      } : scenario.command_progression_presentation ? {
        first_active_objects: 5,
        second_active_objects: 5,
        battle_extent: 115,
        outer_positive_z: 57.5,
        outer_negative_z: -57.5,
        inner_negative_z: -37.5,
        inner_positive_z: 37.5,
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
            || scenario.subsystem_field_command_presentation
            || scenario.live_subsystem_damage_presentation
            || scenario.subsystem_repair_mobility_presentation
            || scenario.maneuver_movement_presentation
            || scenario.command_execution_presentation ? 8
            : scenario.command_progression_presentation ? 10 : 4);
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
        || scenario.subsystem_field_command_presentation
        || scenario.live_subsystem_damage_presentation
        || scenario.subsystem_repair_mobility_presentation
        || scenario.maneuver_movement_presentation
        || scenario.command_execution_presentation
        || scenario.command_progression_presentation) {
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
        const expectedFighters = scenario.command_progression_presentation ? 2 : 1;
        assert.equal(allianceFighters.length, expectedFighters);
        assert.equal(empireFighters.length, expectedFighters);
        if (scenario.command_progression_presentation) {
          assert.ok(allianceShips.every(({ fleet_roster_index }) =>
            [0, 1, 2].includes(fleet_roster_index)));
          assert.ok(empireShips.every(({ fleet_roster_index }) =>
            [0, 1, 2].includes(fleet_roster_index)));
        } else {
          const movedFaction = scenario.maneuver_movement_presentation ? faction : null;
          assert.ok(allianceShips.every(({ fleet_roster_index: roster, source_position: position }) =>
            position[2] === (movedFaction === "alliance" && roster === 0 ? -36.3125 : -56)));
          assert.ok(empireShips.every(({ fleet_roster_index: roster, source_position: position }) =>
            position[2] === (movedFaction === "empire" && roster === 0 ? 36.3125 : 56)));
          assert.equal(allianceFighters[0].source_position[2], -36);
          assert.equal(empireFighters[0].source_position[2], 36);
        }
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
      );
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
          assert.equal(participantFamilyLogs.length, 2,
            "production participants did not load exactly two source families");
          assert.deepEqual(participantFamilyLogs.map(({ text }) => {
            const match = text.match(/base=(\d+)/);
            assert.ok(match, "participant family log omitted its base");
            return Number(match[1]);
          }).sort((a, b) => a - b), [2010, 2510]);
          assert.equal(participantSceneLogs.length, 1,
            "production participants did not emit one scene submission");
          const expectedShipCount = ready.attacker_ships + ready.defender_ships;
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
          assert.equal(cameraLogs.length, scenario.fighter_detail_journey ? 40 : 1,
            "production participants did not use the source tactical camera");
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

async function main() {
  fs.mkdirSync(runDir, { recursive: true });
  if (battle) {
    assert.equal(catalog.family, "TAC-01");
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
        "production-participants-3d-off"]);
    assert.deepEqual(catalog.scenarios.map(({ tactical_proof }) => tactical_proof),
      [true, false, true, true, true, true, true, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false]);
    assert.deepEqual(catalog.scenarios.map(({ expected_lod_resource }) => expected_lod_resource),
      [2560, undefined, 2560, 2561, 2562, 2560, 2560, undefined, undefined,
        undefined, undefined, undefined, undefined, undefined, undefined, undefined, undefined,
        undefined, undefined, undefined, undefined]);
    assert.deepEqual(catalog.scenarios.map(({ lod_journey }) => Boolean(lod_journey)),
      [false, false, false, false, false, true, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false]);
    assert.deepEqual(catalog.scenarios.map(({ camera_journey }) => Boolean(camera_journey)),
      [false, false, false, false, false, false, true, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false]);
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
  const summary = {
    schema_version: schemaVersion,
    family: catalog.family,
    status: results.every(({ status }) => status === "pass") ? "pass" : "fail",
    run_id: runId,
    executions: results.length,
    passed: results.filter(({ status }) => status === "pass").length,
    failed: results.filter(({ status }) => status === "fail").length,
    screenshots_with_matching_goldens: results.filter(({ comparison }) => comparison?.status === "match").length,
    unbaselined: results.filter(({ comparison }) => comparison?.status === "unbaselined").length,
    unstable_screenshots: results.filter(({ two_frame_hashes }) => two_frame_hashes && !two_frame_hashes.equal).length,
    four_request_startups: results.filter(({ requests }) => requests?.length === 4).length,
    muted_launches: results.reduce((total, result) => total + result.launch_attempts.length, 0),
    launch_timeouts: results.reduce((total, result) => total
      + result.launch_attempts.filter(({ error_name }) => error_name === "TimeoutError").length, 0),
    recovered_launch_timeouts: results.filter((result) => result.launch_attempts.length > 1
      && result.launch_attempts.at(-1).status === "pass").length,
    browser_processes_closed: results.filter(({ cleanup }) => cleanup === "closed").length,
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
