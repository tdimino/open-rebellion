#!/usr/bin/env node

import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import fs from "node:fs";
import http from "node:http";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const defaultScenario = path.join(here, "scenarios/encyclopedia-base.json");
const defaultSite = path.join(root, ".artifacts/e21/packed-site");
const defaultSource = path.join(
  root,
  "../agent-30-encyclopedia-build-publication-session-14-open-rebellion/data/base",
);
const expectedRequests = ["/", "/data/runtime.orpk", "/gl.js", "/open-rebellion-test.wasm"];
const artOrigin = { x: 97, y: 86 };

function parseArguments(argv) {
  const parsed = { verifyOnly: false };
  for (let index = 0; index < argv.length; index += 1) {
    const value = argv[index];
    if (value === "--verify-artifacts-only") {
      parsed.verifyOnly = true;
      continue;
    }
    if (!["--scenario", "--source-root", "--site", "--output"].includes(value)) {
      throw new Error(`unknown argument ${value}`);
    }
    const next = argv[index + 1];
    if (!next) throw new Error(`${value} requires a path`);
    parsed[value.slice(2).replace(/-([a-z])/g, (_, letter) => letter.toUpperCase())] = path.resolve(next);
    index += 1;
  }
  return {
    verifyOnly: parsed.verifyOnly,
    scenario: parsed.scenario || defaultScenario,
    sourceRoot: parsed.sourceRoot || defaultSource,
    site: parsed.site || defaultSite,
    output: parsed.output || path.join(root, ".artifacts/e21/owned-parity/verification.json"),
  };
}

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

function readJson(file) {
  return JSON.parse(fs.readFileSync(file, "utf8"));
}

function readRuntimePack(file) {
  const contents = fs.readFileSync(file);
  assert.equal(contents.subarray(0, 4).toString("ascii"), "ORPK", "runtime pack magic");
  assert.equal(contents.readUInt16LE(4), 3, "runtime pack version");
  assert.equal(contents.readUInt16LE(6), 0, "runtime pack flags");
  const count = contents.readUInt32LE(8);
  let cursor = 12;
  const entries = new Map();
  for (let index = 0; index < count; index += 1) {
    assert.ok(cursor + 7 <= contents.length, `runtime pack entry ${index} header is truncated`);
    const kind = contents.readUInt8(cursor);
    const keyLength = contents.readUInt16LE(cursor + 1);
    const byteLength = contents.readUInt32LE(cursor + 3);
    cursor += 7;
    assert.ok(cursor + keyLength + byteLength <= contents.length, `runtime pack entry ${index} is truncated`);
    const key = contents.subarray(cursor, cursor + keyLength).toString("utf8");
    cursor += keyLength;
    const bytes = contents.subarray(cursor, cursor + byteLength);
    cursor += byteLength;
    assert.ok(!entries.has(key), `runtime pack contains duplicate key ${key}`);
    entries.set(key, { kind, bytes });
  }
  assert.equal(cursor, contents.length, "runtime pack has trailing bytes");
  return { contents, entries };
}

const windows1252 = new Map([
  [0x20ac, 0x80], [0x201a, 0x82], [0x0192, 0x83], [0x201e, 0x84], [0x2026, 0x85],
  [0x2020, 0x86], [0x2021, 0x87], [0x02c6, 0x88], [0x2030, 0x89], [0x0160, 0x8a],
  [0x2039, 0x8b], [0x0152, 0x8c], [0x017d, 0x8e], [0x2018, 0x91], [0x2019, 0x92],
  [0x201c, 0x93], [0x201d, 0x94], [0x2022, 0x95], [0x2013, 0x96], [0x2014, 0x97],
  [0x02dc, 0x98], [0x2122, 0x99], [0x0161, 0x9a], [0x203a, 0x9b], [0x0153, 0x9c],
  [0x017e, 0x9e], [0x0178, 0x9f],
]);

function windows1252Bytes(value) {
  const bytes = [];
  for (const scalar of value) {
    const code = scalar.codePointAt(0);
    if (code <= 0x7f || (code >= 0xa0 && code <= 0xff)) bytes.push(code);
    else if (windows1252.has(code)) bytes.push(windows1252.get(code));
    else return null;
  }
  return Buffer.from(bytes.map((byte) => (byte >= 0x41 && byte <= 0x5a ? byte + 0x20 : byte)));
}

function titleSortKey(title) {
  const encoded = windows1252Bytes(title);
  if (encoded) return { class: 0, bytes: encoded };
  const loweredScalars = Array.from(title, (scalar) => scalar.toLowerCase()).join("");
  return { class: 1, bytes: Buffer.from(loweredScalars, "utf8") };
}

function compareSortRows(left, right) {
  if (left.key.class !== right.key.class) return left.key.class - right.key.class;
  const byteOrder = Buffer.compare(left.key.bytes, right.key.bytes);
  return byteOrder || left.registryIndex - right.registryIndex;
}

function selectedImageId(localized, faction) {
  if (Object.hasOwn(localized, "image_id")) return localized.image_id;
  if (!localized.image_selector) return null;
  return faction === "alliance"
    ? localized.image_selector.alliance_image_id
    : localized.image_selector.empire_image_id;
}

function validateVisibleTextChecks(scenario, catalog) {
  const viewport = scenario.body_viewport;
  assert.ok(viewport, "body_viewport is required");
  for (const field of ["x", "y", "width", "height"]) {
    assert.ok(Number.isInteger(viewport[field]) && viewport[field] > 0, `body_viewport.${field}`);
  }
  assert.ok(viewport.x + viewport.width <= 640, "body viewport exceeds fixture width");
  assert.ok(viewport.y + viewport.height <= 480, "body viewport exceeds fixture height");

  const checks = scenario.visible_text_checks;
  assert.ok(Array.isArray(checks) && checks.length > 0, "visible_text_checks are required");
  for (const check of checks) {
    const probe = scenario.probes.find((candidate) => candidate.topic_id === check.topic_id);
    assert.ok(probe, `visible text topic ${check.topic_id} is not a retained probe`);
    const body = catalog.topics[check.topic_id]?.localized?.[catalog.default_language]?.body;
    assert.ok(body, `visible text topic ${check.topic_id} lacks a default-language body`);
    assert.equal(sha256(Buffer.from(body)), probe.body_sha256, `${check.topic_id} visible body digest`);
    assert.ok(Array.isArray(check.input_sequence), `${check.topic_id} input_sequence`);

    if (check.purpose === "long_body_scroll") {
      assert.ok(check.input_sequence.length > 0, "long body journey must scroll");
      assert.ok(
        check.input_sequence.every((key) => key === "PageDown"),
        "long body journey uses ordinary PageDown input",
      );
      assert.equal(
        check.expected_distinct_viewports,
        check.input_sequence.length + 1,
        "long body captures initial plus every scroll step",
      );
      continue;
    }

    assert.equal(check.purpose, "owned_non_ascii", `unknown visible text purpose ${check.purpose}`);
    assert.deepEqual(check.input_sequence, [], "non-ASCII initial viewport needs no synthetic input");
    assert.equal(check.expected_capture, "initial_body_viewport");
    const scalars = Array.from(body);
    const location = check.codepoint;
    const scalar = scalars[location.char_index];
    assert.ok(scalar, `${check.topic_id} codepoint index is out of range`);
    assert.equal(`U+${scalar.codePointAt(0).toString(16).toUpperCase().padStart(4, "0")}`, location.scalar);
    assert.equal(Buffer.byteLength(scalars.slice(0, location.char_index).join("")), location.utf8_byte_offset);
    assert.equal(Buffer.from(scalar).toString("hex"), location.utf8_hex);
    assert.equal(
      sha256(Buffer.from(scalars.slice(location.context_char_start, location.context_char_end_exclusive).join(""))),
      location.context_sha256,
      `${check.topic_id} codepoint context digest`,
    );
  }
  return checks;
}

function verifyArtifacts(options) {
  const scenario = readJson(options.scenario);
  assert.equal(scenario.schema_version, 1);
  assert.equal(scenario.family, "encyclopedia-base-parity");
  const packPath = path.join(options.site, "data/runtime.orpk");
  const pack = readRuntimePack(packPath);
  assert.equal(sha256(pack.contents), scenario.artifacts.pack_sha256, "runtime pack SHA-256");
  assert.equal(pack.contents.length, scenario.artifacts.pack_byte_length, "runtime pack byte length");

  const namespace = new Map(
    [...pack.entries]
      .filter(([key]) => key.startsWith("encyclopedia/"))
      .map(([key, entry]) => [key.slice("encyclopedia/".length), entry]),
  );
  assert.equal(namespace.size, scenario.artifacts.encyclopedia_namespace_entries);
  for (const [relative, entry] of namespace) {
    assert.equal(entry.kind, 0, `encyclopedia/${relative} must be a game-data entry`);
    const loose = fs.readFileSync(path.join(options.sourceRoot, "encyclopedia", relative));
    assert.deepEqual(entry.bytes, loose, `packed encyclopedia/${relative} differs from retained loose bytes`);
  }

  const catalogBytes = namespace.get("catalog.json")?.bytes;
  const manifestBytes = namespace.get("manifest.json")?.bytes;
  assert.ok(catalogBytes, "packed catalog.json is missing");
  assert.ok(manifestBytes, "packed manifest.json is missing");
  assert.equal(sha256(catalogBytes), scenario.artifacts.catalog_sha256);
  assert.equal(sha256(manifestBytes), scenario.artifacts.manifest_sha256);
  const catalog = JSON.parse(catalogBytes.toString("utf8"));
  const manifest = JSON.parse(manifestBytes.toString("utf8"));
  const visibleTextChecks = validateVisibleTextChecks(scenario, catalog);
  assert.equal(manifest.source_profile, scenario.source_profile);
  assert.equal(manifest.catalog_sha256, scenario.artifacts.catalog_sha256);
  assert.equal(manifest.files["catalog.json"], scenario.artifacts.catalog_sha256);
  assert.equal(namespace.size, Object.keys(manifest.files).length + 1, "manifest inventory plus manifest");
  for (const [relative, digest] of Object.entries(manifest.files)) {
    const entry = namespace.get(relative);
    assert.ok(entry, `manifest file ${relative} is absent from the pack`);
    assert.equal(sha256(entry.bytes), digest, `manifest file digest ${relative}`);
  }

  for (const source of manifest.binding_sources) {
    const dat = fs.readFileSync(path.join(options.sourceRoot, source.basename));
    assert.equal(sha256(dat), source.sha256, `binding DAT ${source.basename}`);
  }
  for (const [assetId, image] of Object.entries(catalog.images)) {
    const entry = namespace.get(image.path);
    assert.ok(entry, `${assetId} path ${image.path} is absent from the pack`);
    assert.equal(entry.bytes.length, image.byte_length, `${assetId} byte length`);
    assert.equal(sha256(entry.bytes), image.sha256, `${assetId} digest`);
  }

  const registry = catalog.index.topic_ids;
  const sortedTopics = registry.map((topicId, registryIndex) => {
    const localized = catalog.topics[topicId]?.localized?.[catalog.default_language];
    assert.ok(localized, `${topicId} is missing the whole default-language record`);
    return { topicId, registryIndex, key: titleSortKey(localized.title) };
  }).sort(compareSortRows);

  for (const probe of scenario.probes) {
    const topic = catalog.topics[probe.topic_id];
    assert.ok(topic, `probe topic ${probe.topic_id} is absent`);
    const localized = topic.localized[catalog.default_language];
    assert.ok(localized, `probe topic ${probe.topic_id} lacks default language`);
    assert.equal(sha256(Buffer.from(localized.title)), probe.title_sha256, `${probe.topic_id} title`);
    assert.equal(sha256(Buffer.from(localized.body)), probe.body_sha256, `${probe.topic_id} body`);
    const bindings = catalog.bindings.filter((binding) => binding.topic_id === probe.topic_id);
    assert.equal(bindings.length, 1, `${probe.topic_id} must have one binding`);
    assert.deepEqual(
      {
        family: bindings[0].family,
        dat_id: bindings[0].dat_id,
        variant: bindings[0].variant,
      },
      probe.binding,
      `${probe.topic_id} binding`,
    );
    assert.equal(
      sortedTopics.findIndex((row) => row.topicId === probe.topic_id),
      probe.sort_index,
      `${probe.topic_id} effective title order`,
    );
    for (const faction of ["alliance", "empire"]) {
      const expected = probe.images[faction];
      const assetId = selectedImageId(localized, faction);
      assert.equal(assetId, expected?.asset_id ?? null, `${probe.topic_id} ${faction} image selection`);
      if (expected) {
        const descriptor = catalog.images[assetId];
        assert.ok(descriptor, `${probe.topic_id} selected unknown image ${assetId}`);
        assert.equal(descriptor.sha256, expected.sha256, `${probe.topic_id} ${faction} image digest`);
      }
    }
  }

  const noArtCount = Object.values(catalog.topics).filter((topic) =>
    Object.values(topic.localized).every((localized) => selectedImageId(localized, "alliance") === null
      && selectedImageId(localized, "empire") === null)).length;
  assert.equal(noArtCount, scenario.owned_no_art_topic_count, "owned no-art topic count");

  return {
    schema_version: 1,
    family: scenario.family,
    scope: "owned base-byte/package provenance; live rendering is a separate coordinator gate",
    status: "pass",
    source_profile: manifest.source_profile,
    runtime_pack_sha256: sha256(pack.contents),
    runtime_pack_byte_length: pack.contents.length,
    catalog_sha256: sha256(catalogBytes),
    manifest_sha256: sha256(manifestBytes),
    encyclopedia_namespace_entries: namespace.size,
    binding_sources_verified: manifest.binding_sources.length,
    probes_verified: scenario.probes.length,
    visible_text_checks_verified: visibleTextChecks.length,
    body_viewport: scenario.body_viewport,
    owned_no_art_topic_count: noArtCount,
  };
}

function mimeType(file) {
  if (file.endsWith(".html")) return "text/html; charset=utf-8";
  if (file.endsWith(".js")) return "text/javascript; charset=utf-8";
  if (file.endsWith(".wasm")) return "application/wasm";
  return "application/octet-stream";
}

async function startServer(site, observed) {
  const rootWithSeparator = `${path.resolve(site)}${path.sep}`;
  const server = http.createServer((request, response) => {
    const pathname = new URL(request.url, "http://localhost").pathname;
    const relative = path.posix.normalize(decodeURIComponent(pathname)).replace(/^\/+/, "") || "index.html";
    const candidate = path.resolve(site, relative);
    const permitted = candidate.startsWith(rootWithSeparator);
    const exists = permitted && fs.existsSync(candidate);
    observed.push({ url: pathname, status: exists ? 200 : permitted ? 404 : 403 });
    if (!exists) {
      response.writeHead(permitted ? 404 : 403).end();
      return;
    }
    const bytes = fs.readFileSync(candidate);
    response.writeHead(200, {
      "content-type": mimeType(candidate),
      "content-length": bytes.length,
      "cache-control": "no-store",
    }).end(bytes);
  });
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  return server;
}

function browserExecutable(browserManifest) {
  const candidates = [
    ...(process.env.OPEN_REBELLION_CHROME_FOR_TESTING
      ? [process.env.OPEN_REBELLION_CHROME_FOR_TESTING]
      : []),
    ...browserManifest.executable_candidates,
  ];
  const executable = candidates.find((candidate) => fs.existsSync(candidate));
  if (!executable) throw new Error(`pinned Chrome for Testing ${browserManifest.version} is missing`);
  return executable;
}

function evidenceFromConsole(line, marker) {
  const offset = line.indexOf(marker);
  if (offset < 0) return null;
  return JSON.parse(line.slice(offset + marker.length));
}

async function waitForSelection(consoleLines, topicId, fromIndex, timeoutMs = 15_000) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    for (let index = fromIndex; index < consoleLines.length; index += 1) {
      const evidence = evidenceFromConsole(consoleLines[index].text, "selection_evidence=");
      if (evidence?.topic_id === topicId) return { index, evidence };
    }
    await new Promise((resolve) => setTimeout(resolve, 25));
  }
  throw new Error(`timed out waiting for selected topic ${topicId}`);
}

async function waitForViewport(consoleLines, topicId, fromIndex, expectedIntent, timeoutMs = 15_000) {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    for (let index = fromIndex; index < consoleLines.length; index += 1) {
      const evidence = evidenceFromConsole(consoleLines[index].text, "viewport_evidence=");
      if (evidence?.topic_id === topicId
        && evidence.consumed_scroll_intents?.includes(expectedIntent)) return { index, evidence };
    }
    await new Promise((resolve) => setTimeout(resolve, 25));
  }
  throw new Error(`timed out waiting for ${expectedIntent} viewport evidence for ${topicId}`);
}

function decodeIndexedBmp(bytes, PNG) {
  assert.equal(bytes.subarray(0, 2).toString("ascii"), "BM");
  const dataOffset = bytes.readUInt32LE(10);
  const headerSize = bytes.readUInt32LE(14);
  const width = bytes.readInt32LE(18);
  const signedHeight = bytes.readInt32LE(22);
  const height = Math.abs(signedHeight);
  assert.equal(bytes.readUInt16LE(28), 8);
  assert.equal(bytes.readUInt32LE(30), 0);
  const paletteOffset = 14 + headerSize;
  const stride = (width + 3) & ~3;
  const image = new PNG({ width, height });
  for (let y = 0; y < height; y += 1) {
    const sourceY = signedHeight > 0 ? height - 1 - y : y;
    for (let x = 0; x < width; x += 1) {
      const paletteIndex = bytes[dataOffset + sourceY * stride + x];
      const palette = paletteOffset + paletteIndex * 4;
      const destination = (y * width + x) * 4;
      image.data[destination] = bytes[palette + 2];
      image.data[destination + 1] = bytes[palette + 1];
      image.data[destination + 2] = bytes[palette];
      image.data[destination + 3] = 255;
    }
  }
  return image;
}

function compareArtwork(screenshotBytes, sourceBytes, PNG) {
  const screenshot = PNG.sync.read(screenshotBytes);
  const source = decodeIndexedBmp(sourceBytes, PNG);
  assert.deepEqual([source.width, source.height], [400, 200]);
  let differentPixels = 0;
  for (let y = 0; y < source.height; y += 1) {
    for (let x = 0; x < source.width; x += 1) {
      const expected = (y * source.width + x) * 4;
      const actual = ((artOrigin.y + y) * screenshot.width + artOrigin.x + x) * 4;
      if (screenshot.data[actual] !== source.data[expected]
        || screenshot.data[actual + 1] !== source.data[expected + 1]
        || screenshot.data[actual + 2] !== source.data[expected + 2]) differentPixels += 1;
    }
  }
  return { width: source.width, height: source.height, different_pixels: differentPixels };
}

function cropViewport(screenshotBytes, viewport, PNG) {
  const screenshot = PNG.sync.read(screenshotBytes);
  assert.ok(viewport.x + viewport.width <= screenshot.width, "body viewport exceeds screenshot width");
  assert.ok(viewport.y + viewport.height <= screenshot.height, "body viewport exceeds screenshot height");
  const crop = new PNG({ width: viewport.width, height: viewport.height });
  for (let row = 0; row < viewport.height; row += 1) {
    const source = ((viewport.y + row) * screenshot.width + viewport.x) * 4;
    const destination = row * viewport.width * 4;
    screenshot.data.copy(crop.data, destination, source, source + viewport.width * 4);
  }
  return PNG.sync.write(crop);
}

function differentPixelCount(leftBytes, rightBytes, PNG) {
  const left = PNG.sync.read(leftBytes);
  const right = PNG.sync.read(rightBytes);
  assert.deepEqual([left.width, left.height], [right.width, right.height]);
  let different = 0;
  for (let offset = 0; offset < left.data.length; offset += 4) {
    if (left.data[offset] !== right.data[offset]
      || left.data[offset + 1] !== right.data[offset + 1]
      || left.data[offset + 2] !== right.data[offset + 2]
      || left.data[offset + 3] !== right.data[offset + 3]) different += 1;
  }
  return different;
}

async function runBrowser(options, artifactResult) {
  const scenario = readJson(options.scenario);
  for (const required of ["index.html", "gl.js", "open-rebellion-test.wasm", "data/runtime.orpk"]) {
    assert.ok(fs.existsSync(path.join(options.site, required)), `packed site is missing ${required}`);
  }
  const dependencyRoot = process.env.OPEN_REBELLION_INTERFACE_NODE_MODULES;
  const playwrightModule = dependencyRoot
    ? pathToFileURL(path.join(dependencyRoot, "playwright-core/index.mjs")).href
    : "playwright-core";
  const pngModule = dependencyRoot
    ? pathToFileURL(path.join(dependencyRoot, "pngjs/lib/png.js")).href
    : "pngjs";
  const [{ chromium }, { PNG }, { launchBrowser }] = await Promise.all([
    import(playwrightModule),
    import(pngModule),
    import("./browser-launch.mjs"),
  ]);
  const browserManifest = readJson(path.join(here, "browser.json"));
  const serverRequests = [];
  const server = await startServer(options.site, serverRequests);
  const result = { ...artifactResult, scope: "owned base-byte native/packed transport and rendering checkpoint", cases: [] };
  let browser;
  try {
    browser = await launchBrowser(chromium, {
      executablePath: browserExecutable(browserManifest),
      headless: true,
      args: browserManifest.launch_arguments,
      timeout: 30_000,
    }, []);
    result.browser_version = browser.version();
    result.muted = browserManifest.launch_arguments.includes("--mute-audio");
    for (const faction of ["alliance", "empire"]) {
      const context = await browser.newContext({
        viewport: { width: 640, height: 480 },
        deviceScaleFactor: 1,
        locale: "en-US",
        timezoneId: "America/New_York",
        reducedMotion: "reduce",
        serviceWorkers: "block",
      });
      const page = await context.newPage();
      const browserRequests = [];
      const consoleLines = [];
      const errors = [];
      page.on("request", (request) => browserRequests.push(new URL(request.url()).pathname));
      page.on("requestfailed", (request) => errors.push(`request:${request.url()}:${request.failure()?.errorText}`));
      page.on("pageerror", (error) => errors.push(`page:${error.stack || error.message}`));
      page.on("console", (message) => {
        consoleLines.push({ type: message.type(), text: message.text() });
        if (message.type() === "error") errors.push(`console:${message.text()}`);
      });
      const code = scenario.fixture_codes[faction];
      const origin = `http://127.0.0.1:${server.address().port}`;
      const serverStart = serverRequests.length;
      await page.goto(`${origin}/?fixture-code=${code}`, { waitUntil: "load", timeout: 30_000 });
      await page.waitForFunction(() => window.__openRebellionInterfaceReady?.status, null, { timeout: 30_000 });
      const ready = await page.evaluate(() => window.__openRebellionInterfaceReady);
      assert.equal(ready.status, "ready", JSON.stringify(ready));
      assert.equal(ready.faction, faction);
      assert.equal(ready.source_profile, scenario.source_profile);
      await context.setOffline(true);
      const navigationRequestStart = browserRequests.length;
      let currentIndex = 0;
      let consoleIndex = 0;
      const probes = [];
      for (const probe of [...scenario.probes].sort((left, right) => left.sort_index - right.sort_index)) {
        while (currentIndex < probe.sort_index) {
          await page.keyboard.press("ArrowRight");
          currentIndex += 1;
          await page.waitForTimeout(20);
        }
        const selected = await waitForSelection(consoleLines, probe.topic_id, consoleIndex);
        consoleIndex = selected.index + 1;
        assert.equal(selected.evidence.viewer, faction);
        assert.equal(selected.evidence.binding_family, probe.binding.family);
        assert.equal(selected.evidence.binding_dat_id, probe.binding.dat_id);
        assert.equal(selected.evidence.binding_variant, probe.binding.variant);
        assert.equal(selected.evidence.title_sha256, probe.title_sha256);
        assert.equal(selected.evidence.body_sha256, probe.body_sha256);
        const expectedImage = probe.images[faction];
        assert.equal(selected.evidence.asset_id, expectedImage?.asset_id ?? null);
        assert.equal(selected.evidence.digest, expectedImage?.sha256 ?? null);
        await page.waitForTimeout(80);
        const screenshot = await page.screenshot({ animations: "disabled" });
        let pixels = null;
        if (expectedImage) {
          const catalog = readJson(path.join(options.sourceRoot, "encyclopedia/catalog.json"));
          const sourceBytes = fs.readFileSync(path.join(
            options.sourceRoot,
            "encyclopedia",
            catalog.images[expectedImage.asset_id].path,
          ));
          pixels = compareArtwork(screenshot, sourceBytes, PNG);
          assert.equal(pixels.different_pixels, 0, `${faction} ${probe.topic_id} artwork pixels`);
        }
        const cacheHit = consoleLines.some((line) =>
          line.text.includes("texture_event=Selected")
          && line.text.includes(`asset_id: \"${expectedImage?.asset_id}\"`)
          && line.text.includes("cache_hit: true"));
        assert.equal(cacheHit, Boolean(expectedImage), `${faction} ${probe.topic_id} cache-hit event`);
        const screenshotPath = path.join(path.dirname(options.output), `${faction}-${probe.topic_id.replace(":", "-")}.png`);
        fs.mkdirSync(path.dirname(screenshotPath), { recursive: true });
        fs.writeFileSync(screenshotPath, screenshot);
        const visibleText = [];
        for (const check of scenario.visible_text_checks.filter((candidate) => candidate.topic_id === probe.topic_id)) {
          const prefix = `${faction}-${probe.topic_id.replace(":", "-")}-${check.purpose}`;
          let viewportBytes = cropViewport(screenshot, scenario.body_viewport, PNG);
          const initialViewportPath = path.join(path.dirname(options.output), `${prefix}-initial.png`);
          fs.writeFileSync(initialViewportPath, viewportBytes);
          const captures = [{
            stage: "initial",
            scroll_offset: 0,
            screenshot: path.resolve(initialViewportPath),
            screenshot_sha256: sha256(viewportBytes),
          }];

          if (check.purpose === "long_body_scroll") {
            let previousOffset = 0;
            for (let step = 0; step < check.input_sequence.length; step += 1) {
              const key = check.input_sequence[step];
              const evidenceStart = consoleLines.length;
              await page.keyboard.press(key);
              const observed = await waitForViewport(
                consoleLines,
                probe.topic_id,
                evidenceStart,
                "page_down",
              );
              assert.equal(observed.evidence.source_profile, scenario.source_profile);
              assert.equal(observed.evidence.viewer, faction);
              assert.equal(observed.evidence.body_sha256, probe.body_sha256);
              assert.deepEqual(observed.evidence.consumed_scroll_intents, ["page_down"]);
              assert.ok(
                observed.evidence.scroll_offset > previousOffset,
                `${faction} ${probe.topic_id} PageDown ${step + 1} did not advance`,
              );
              await page.waitForTimeout(80);
              const full = await page.screenshot({ animations: "disabled" });
              const nextViewport = cropViewport(full, scenario.body_viewport, PNG);
              const changedPixels = differentPixelCount(viewportBytes, nextViewport, PNG);
              assert.ok(changedPixels > 0, `${faction} ${probe.topic_id} PageDown ${step + 1} did not change the viewport`);
              const capturePath = path.join(path.dirname(options.output), `${prefix}-page-${step + 1}.png`);
              fs.writeFileSync(capturePath, nextViewport);
              captures.push({
                stage: `after_${key}_${step + 1}`,
                scroll_offset: observed.evidence.scroll_offset,
                consumed_scroll_intents: observed.evidence.consumed_scroll_intents,
                changed_pixels_from_previous: changedPixels,
                screenshot: path.resolve(capturePath),
                screenshot_sha256: sha256(nextViewport),
              });
              previousOffset = observed.evidence.scroll_offset;
              viewportBytes = nextViewport;
            }
            assert.equal(captures.length, check.expected_distinct_viewports);
          }

          visibleText.push({
            purpose: check.purpose,
            input_sequence: check.input_sequence,
            body_viewport: scenario.body_viewport,
            codepoint: check.codepoint ?? null,
            expected_capture: check.expected_capture ?? null,
            captures,
          });
        }
        probes.push({
          topic_id: probe.topic_id,
          selection: selected.evidence,
          pixels,
          screenshot: path.resolve(screenshotPath),
          screenshot_sha256: sha256(screenshot),
          cache_hit_observed: cacheHit,
          visible_text: visibleText,
        });
      }
      assert.equal(browserRequests.length, navigationRequestStart, `${faction} offline navigation requests`);
      assert.deepEqual([...browserRequests].sort(), [...expectedRequests].sort(), `${faction} startup requests`);
      assert.deepEqual(errors, [], `${faction} browser diagnostics`);
      result.cases.push({
        faction,
        fixture_code: code,
        ready,
        requests: browserRequests,
        navigation_requests: browserRequests.length - navigationRequestStart,
        probes,
      });
      await context.close();
      assert.equal(serverRequests.length - serverStart, expectedRequests.length, `${faction} server request count`);
    }
  } finally {
    if (browser) await browser.close();
    await new Promise((resolve) => server.close(resolve));
  }
  return result;
}

async function main() {
  const options = parseArguments(process.argv.slice(2));
  const artifactResult = verifyArtifacts(options);
  const result = options.verifyOnly ? artifactResult : await runBrowser(options, artifactResult);
  fs.mkdirSync(path.dirname(options.output), { recursive: true });
  fs.writeFileSync(options.output, `${JSON.stringify(result, null, 2)}\n`);
  console.log(JSON.stringify({ status: result.status, output: options.output }));
}

main().catch((error) => {
  console.error(error.stack || error);
  process.exitCode = 1;
});
