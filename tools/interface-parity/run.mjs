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
  const control = faction === "alliance"
    ? { x: 3, y: 355, width: 27, height: 41 }
    : { x: 79, y: 192, width: 35, height: 57 };
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
  await page.screenshot({ path: path.join(folder, "control-pressed.png"), animations: "disabled" });
  await page.mouse.up();
  await page.waitForTimeout(80);
  assert.ok(commandLines().some((line) => line.includes("destination=gid_menu status=opened")),
    `${faction}: interior did not open the GID menu`);
  const rootCapture = await page.screenshot({ path: path.join(folder, "menu-root.png"), animations: "disabled" });
  const rootFrame = verifyGidRootFrame(viewport, rootCapture, scenario.slug === "system");
  probes.push({ type: "control-click", x: interior.x, y: interior.y, opened: true,
    screenshot_sha256: sha256(rootCapture), root_frame: rootFrame });

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
    await page.mouse.click(alternateLeaf.x, alternateLeaf.y);
    await page.waitForFunction((expected) => window.__openRebellionInterfaceSelection?.mode === expected,
      alternate, { timeout: 2_000 });
    probes.push({ type: "alternate-mode", mode: alternate });

    await page.mouse.click(interior.x, interior.y);
    await page.waitForTimeout(80);
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
    await page.mouse.click(leafPoint.x, leafPoint.y);
    await page.waitForTimeout(80);
    const expected = `destination=gid status=selected label=${scenario.mode}`;
    assert.ok(commandLines().some((line) => line.includes(expected)),
      `GID ${scenario.mode} did not route from category ${scenario.category} leaf ${scenario.leaf}: ${commandLines()}`);
    await page.waitForFunction((mode) => window.__openRebellionInterfaceSelection?.mode === mode,
      scenario.mode, { timeout: 2_000 });
    const selected = await page.evaluate(() => window.__openRebellionInterfaceSelection);
    assert.equal(selected.code, fixtureCode(scenario, faction));
    const selectedCapture = await page.screenshot({ path: path.join(folder, "menu-selected.png"), animations: "disabled" });
    probes.push({ type: "submenu-selection", mode: scenario.mode,
      screenshot_sha256: sha256(selectedCapture) });
  } else if (scenario.slug === "display-off") {
    const displayOff = point(477, 372);
    await page.mouse.click(displayOff.x, displayOff.y);
    await page.waitForTimeout(80);
    assert.ok(commandLines().some((line) => line.includes("destination=gid status=selected label=Display Off")),
      `${faction}: root Display Off did not route`);
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

function verifyTacticalBitmap(viewport, screenshotBytes, id, x0, y0) {
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
      if (color[0] < 32 && color[1] < 32 && color[2] > 192) continue;
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

function verifyTacticalApertureIsolation(viewport, beforeBytes, afterBytes) {
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
  let pixelsChecked = 0;
  for (let y = 0; y < before.height; y++) {
    for (let x = 0; x < before.width; x++) {
      if (x >= aperture.x0 && x < aperture.x1 && y >= aperture.y0 && y < aperture.y1) continue;
      const offset = (y * before.width + x) * 4;
      assert.deepEqual(
        Array.from(after.data.subarray(offset, offset + 4)),
        Array.from(before.data.subarray(offset, offset + 4)),
        `battlefield redraw escaped tactical aperture at (${x}, ${y})`,
      );
      pixelsChecked++;
    }
  }
  return { status: "isolated", pixels_checked: pixelsChecked };
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
  const capture = async (name) => page.screenshot({
    path: path.join(folder, `${name}.png`), animations: "disabled",
  });
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

  const zoomIn = point(498, 355);
  await page.mouse.click(zoomIn.x, zoomIn.y);
  await page.waitForTimeout(80);
  const zoomed = await capture("zoomed-in");
  assert.notEqual(sha256(highlightsRestored), sha256(zoomed), "zoom-in control did not redraw battle");
  probes.push({ type: "zoom-in", ...verifyTacticalBitmap(viewport, zoomed, 1044, 486, 343),
    aperture_isolation: verifyTacticalApertureIsolation(viewport, highlightsRestored, zoomed) });
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
  let browser;
  let context;
  let page;
  let result;
  try {
    browser = await chromium.launch({
      executablePath: executable,
      headless: true,
      args: browserManifest.launch_arguments,
      timeout: 30_000,
    });
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
      if (message.type() === "error" || /missing.asset|\[bmp_cache\].*not found/i.test(message.text())) {
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
        fighters: ready.fighters }, ...await probeTactical(page, viewport, folder, stable)]
      : await probeGid(page, faction, scenario, viewport, folder, consoleLines, ready);
    if (battle) {
      assert.equal(ready.family, "tactical");
      assert.equal(ready.faction, faction);
      assert.ok(ready.attacker_ships > 0 && ready.defender_ships > 0);
      assert.ok(ready.fighters > 0);
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
    assert.deepEqual(catalog.scenarios.map(({ slug }) => slug), ["battle-entry"]);
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
    const factions = smoke ? ["alliance", "empire"] : catalog.factions;
    const viewports = smoke ? [catalog.viewports[0]] : catalog.viewports;
    for (const scenario of scenarios) {
      for (const faction of factions) {
        for (const viewport of viewports) {
          const result = await runScenario(server, executable, scenario, faction, viewport);
          results.push(result);
          process.stdout.write(`${result.status === "pass" ? "PASS" : "FAIL"} ${result.id}\n`);
        }
      }
    }
  } finally {
    if (server) await new Promise((resolve) => server.close(resolve));
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
    muted_launches: results.length,
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
