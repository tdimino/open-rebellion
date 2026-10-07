#!/usr/bin/env node

import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import fs from "node:fs";
import http from "node:http";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright-core";
import { PNG } from "pngjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const site = path.join(root, ".artifacts/interface-parity/site");
const source = path.join(root, "data/base/ui/strategy-dll/BMP");
const executable = process.env.OPEN_REBELLION_CHROME_FOR_TESTING || chromium.executablePath();
const launchArguments = ["--mute-audio", "--disable-background-networking", "--no-first-run"];
const expectedRequests = ["/", "/data/runtime.orpk", "/gl.js", "/open-rebellion-test.wasm"];
const runId = `${new Date().toISOString().replace(/[:.]/g, "-")}-${process.pid}`;
const runDir = path.join(root, ".artifacts/interface-parity", `encyclopedia-surface-${runId}`);
const starts = [
  { name: "middle", scenario: 55 },
  { name: "first", scenario: 56 },
  { name: "unavailable", scenario: 57 },
  { name: "index", scenario: 58 },
];
const factions = [
  { name: "alliance", byte: 1, middleColor: [84, 52, 102], indexResource: 10372 },
  { name: "empire", byte: 2, middleColor: [96, 32, 28], indexResource: 10378 },
];

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

function fixtureCode(scenario, factionByte) {
  return (scenario + 1) | (factionByte << 8);
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
  assert.equal(bytes.toString("ascii", 0, 2), "BM");
  const dataOffset = bytes.readUInt32LE(10);
  const headerSize = bytes.readUInt32LE(14);
  const width = bytes.readInt32LE(18);
  const signedHeight = bytes.readInt32LE(22);
  const height = Math.abs(signedHeight);
  const paletteOffset = 14 + headerSize;
  const stride = (width + 3) & ~3;
  const png = new PNG({ width, height });
  for (let y = 0; y < height; y += 1) {
    const sourceY = signedHeight > 0 ? height - 1 - y : y;
    for (let x = 0; x < width; x += 1) {
      const palette = paletteOffset + bytes[dataOffset + sourceY * stride + x] * 4;
      const destination = (y * width + x) * 4;
      png.data.set([bytes[palette + 2], bytes[palette + 1], bytes[palette], 255], destination);
    }
  }
  return { png, bytes, dataOffset, stride, signedHeight };
}

function opaqueProbe(resourceId) {
  const decoded = decodeIndexedBmp(fs.readFileSync(path.join(source, `${resourceId}.bmp`)));
  const { png, bytes, dataOffset, stride, signedHeight } = decoded;
  const transparent = bytes[dataOffset];
  const candidates = [];
  for (let y = 1; y < png.height - 1; y += 1) {
    const sourceY = signedHeight > 0 ? png.height - 1 - y : y;
    for (let x = 1; x < png.width - 1; x += 1) {
      if (bytes[dataOffset + sourceY * stride + x] !== transparent) {
        candidates.push({ x, y, distance: Math.abs(x - png.width / 2) + Math.abs(y - png.height / 2) });
      }
    }
  }
  candidates.sort((left, right) => left.distance - right.distance);
  assert.ok(candidates.length > 0, `${resourceId} has no opaque probe`);
  return candidates[0];
}

function countColor(screenshot, rect, color) {
  let count = 0;
  for (let y = rect.y; y < rect.y + rect.height; y += 1) {
    for (let x = rect.x; x < rect.x + rect.width; x += 1) {
      const offset = (y * screenshot.width + x) * 4;
      if (screenshot.data[offset] === color[0]
        && screenshot.data[offset + 1] === color[1]
        && screenshot.data[offset + 2] === color[2]) count += 1;
    }
  }
  return count;
}

function compareResource(screenshot, resourceId, x, y, width, height) {
  const expected = decodeIndexedBmp(fs.readFileSync(path.join(source, `${resourceId}.bmp`))).png;
  let different = 0;
  for (let row = 0; row < height; row += 1) {
    for (let column = 0; column < width; column += 1) {
      const actualOffset = ((y + row) * screenshot.width + x + column) * 4;
      const expectedOffset = (row * expected.width + column) * 4;
      if (screenshot.data[actualOffset] !== expected.data[expectedOffset]
        || screenshot.data[actualOffset + 1] !== expected.data[expectedOffset + 1]
        || screenshot.data[actualOffset + 2] !== expected.data[expectedOffset + 2]) different += 1;
    }
  }
  return different;
}

async function stableScreenshot(page, directory, label) {
  await page.waitForTimeout(80);
  await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  const first = await page.screenshot({ animations: "disabled" });
  await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  const second = await page.screenshot({ animations: "disabled" });
  assert.equal(sha256(first), sha256(second), `${label} did not stabilize`);
  fs.writeFileSync(path.join(directory, `${label}.png`), second);
  return { bytes: second, png: PNG.sync.read(second), sha256: sha256(second) };
}

async function runCase(server, faction, start, viewport) {
  const directory = path.join(runDir, faction.name, start.name, `${viewport.width}x${viewport.height}`);
  fs.mkdirSync(directory, { recursive: true });
  const code = fixtureCode(start.scenario, faction.byte);
  const requests = [];
  const errors = [];
  const browser = await chromium.launch({ executablePath: executable, headless: true, args: launchArguments });
  const context = await browser.newContext({ viewport, deviceScaleFactor: 1, reducedMotion: "reduce" });
  const page = await context.newPage();
  const originUrl = `http://127.0.0.1:${server.address().port}`;
  page.on("response", (response) => {
    if (new URL(response.url()).origin === originUrl) {
      requests.push({ path: new URL(response.url()).pathname, status: response.status() });
    }
  });
  page.on("requestfailed", (request) => errors.push(`request:${request.url()}:${request.failure()?.errorText}`));
  page.on("pageerror", (error) => errors.push(`page:${error.stack || error.message}`));
  page.on("console", (message) => {
    if (message.type() === "error") errors.push(`console:${message.text()}`);
  });
  try {
    await page.goto(`${originUrl}/?fixture-code=${code}`, { waitUntil: "load", timeout: 30_000 });
    await page.waitForFunction(() => window.__openRebellionInterfaceReady?.status, null, { timeout: 30_000 });
    const ready = await page.evaluate(() => window.__openRebellionInterfaceReady);
    assert.equal(ready.status, "ready", JSON.stringify(ready));
    assert.equal(ready.code, code);
    const initial = await stableScreenshot(page, directory, "initial");
    const scale = Math.min(viewport.width / 640, viewport.height / 480);
    const origin = {
      x: Math.round((viewport.width - 640 * scale) / 2 + 85 * scale),
      y: Math.round((viewport.height - 480 * scale) / 2 + 55 * scale),
    };
    const artRect = {
      x: Math.round(origin.x + 12 * scale),
      y: Math.round(origin.y + 31 * scale),
      width: Math.round(400 * scale),
      height: Math.round(200 * scale),
    };
    const checks = {};
    if (start.name === "middle") {
      checks.art_pixels = countColor(initial.png, artRect, faction.middleColor);
      assert.ok(checks.art_pixels >= artRect.width * artRect.height * 0.99);

      await page.keyboard.press("PageDown");
      const scrolled = await stableScreenshot(page, directory, "body-scrolled");
      assert.notEqual(scrolled.sha256, initial.sha256, "long body did not scroll");

      await page.reload({ waitUntil: "load" });
      await page.waitForFunction(() => window.__openRebellionInterfaceReady?.status);
      const reset = await stableScreenshot(page, directory, "reset");
      const forward = opaqueProbe(10382);
      const forwardPoint = {
        x: origin.x + (380 + forward.x) * scale,
        y: origin.y + (14 + forward.y) * scale,
      };
      await page.mouse.move(forwardPoint.x, forwardPoint.y);
      const hovered = await stableScreenshot(page, directory, "forward-hover");
      assert.equal(hovered.sha256, reset.sha256, "hover incorrectly changed the bitmap");
      await page.mouse.down();
      const pressed = await stableScreenshot(page, directory, "forward-pressed");
      assert.notEqual(pressed.sha256, reset.sha256, "captured press did not change the bitmap");
      await page.mouse.up();
      const next = await stableScreenshot(page, directory, "forward-next-topic");
      assert.notEqual(next.sha256, reset.sha256, "forward did not open the next topic");

      await page.reload({ waitUntil: "load" });
      await page.waitForFunction(() => window.__openRebellionInterfaceReady?.status);
      const indexProbe = opaqueProbe(faction.indexResource);
      const indexOrigin = faction.name === "alliance" ? { x: 423, y: 147 } : { x: 426, y: 143 };
      await page.mouse.click(
        origin.x + (indexOrigin.x + indexProbe.x) * scale,
        origin.y + (indexOrigin.y + indexProbe.y) * scale,
      );
      const indexed = await stableScreenshot(page, directory, "mode-index");
      assert.notEqual(indexed.sha256, reset.sha256, "index mode control did not switch surfaces");

      await page.reload({ waitUntil: "load" });
      await page.waitForFunction(() => window.__openRebellionInterfaceReady?.status);
      await page.keyboard.press("Escape");
      const closed = await stableScreenshot(page, directory, "closed");
      assert.equal(countColor(closed.png, artRect, faction.middleColor), 0, "Escape did not close the surface");
    } else if (start.name === "first") {
      checks.art_pixels = countColor(initial.png, artRect, [26, 48, 82]);
      assert.ok(checks.art_pixels >= artRect.width * artRect.height * 0.99);
      if (scale === 1) {
        checks.disabled_back_differences = compareResource(
          initial.png,
          10387,
          origin.x + 28,
          origin.y + 14,
          21,
          17,
        );
        assert.equal(checks.disabled_back_differences, 0);
      }
      await page.keyboard.press("ArrowLeft");
      const retained = await stableScreenshot(page, directory, "left-endpoint-retained");
      assert.equal(retained.sha256, initial.sha256, "first topic wrapped backward");
    } else if (start.name === "unavailable") {
      checks.middle_color_pixels = countColor(initial.png, artRect, faction.middleColor);
      assert.equal(checks.middle_color_pixels, 0, "source-unavailable topic retained stale artwork");
      if (scale === 1) {
        checks.disabled_forward_differences = compareResource(
          initial.png,
          10384,
          origin.x + 380,
          origin.y + 14,
          21,
          17,
        );
        assert.equal(checks.disabled_forward_differences, 0);
      }
      await page.keyboard.press("ArrowRight");
      const retained = await stableScreenshot(page, directory, "right-endpoint-retained");
      assert.equal(retained.sha256, initial.sha256, "last topic wrapped forward");
    } else {
      await page.keyboard.press("ArrowRight");
      const category = await stableScreenshot(page, directory, "next-category");
      assert.notEqual(category.sha256, initial.sha256, "index Right did not select the next category");
      await page.keyboard.press("Enter");
      const topic = await stableScreenshot(page, directory, "enter-topic");
      assert.notEqual(topic.sha256, category.sha256, "index Enter did not open its selected topic");
      checks.topic_art_pixels = countColor(topic.png, artRect, [26, 48, 82]);
      assert.ok(checks.topic_art_pixels >= artRect.width * artRect.height * 0.99);
    }

    assert.deepEqual([...new Set(requests.map(({ path: value }) => value))].sort(), [...expectedRequests].sort());
    assert.ok(requests.every(({ status }) => status === 200));
    assert.deepEqual(errors, []);
    return { faction: faction.name, start: start.name, viewport, code, ready, checks, errors };
  } finally {
    await context.close();
    await browser.close();
  }
}

async function main() {
  for (const required of ["index.html", "gl.js", "open-rebellion-test.wasm", "data/runtime.orpk"]) {
    assert.ok(fs.existsSync(path.join(site, required)), `fixture site is missing ${required}`);
  }
  assert.ok(fs.existsSync(executable), `browser is missing: ${executable}`);
  fs.mkdirSync(runDir, { recursive: true });
  const server = await startServer();
  const results = [];
  try {
    for (const faction of factions) {
      for (const start of starts) results.push(await runCase(server, faction, start, { width: 640, height: 480 }));
    }
    for (const faction of factions) {
      results.push(await runCase(server, faction, starts[0], { width: 800, height: 600 }));
    }
  } finally {
    await new Promise((resolve) => server.close(resolve));
  }
  const summary = {
    schema_version: 1,
    family: "encyclopedia-authentic-surface-a1",
    status: "pass",
    scope: "synthetic validated topic bytes with authentic STRATEGY chrome; not A0 or canonical packaged-reader proof",
    browser: executable,
    browser_version: await chromium.launch({ executablePath: executable, headless: true }).then(async (browser) => {
      const version = browser.version();
      await browser.close();
      return version;
    }),
    cases: results.length,
    wasm_sha256: sha256(fs.readFileSync(path.join(site, "open-rebellion-test.wasm"))),
    runtime_pack_sha256: sha256(fs.readFileSync(path.join(site, "data/runtime.orpk"))),
    results,
  };
  fs.writeFileSync(path.join(runDir, "summary.json"), `${JSON.stringify(summary, null, 2)}\n`);
  console.log(JSON.stringify({ ...summary, run_directory: runDir }, null, 2));
}

await main();
