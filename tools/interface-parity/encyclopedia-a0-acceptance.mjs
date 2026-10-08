#!/usr/bin/env node

import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import fs from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";
import pixelmatch from "pixelmatch";
import { PNG } from "pngjs";

const SURFACE_WIDTH = 470;
const SURFACE_HEIGHT = 330;
const origins = {
  alliance: { x: 62, y: 50 },
  empire: { x: 125, y: 52 },
};

const categoryFiles = {
  systems: "02-category-70.png",
  ships: "02-category-71.png",
  facilities: "02-category-72.png",
  missions: "02-category-73.png",
  troops: "02-category-74.png",
  personnel: "02-category-75.png",
};

const expectedCells = [
  ["OBJ-01-C001", "index"],
  ["OBJ-01-C002", "systems"],
  ["OBJ-01-C003", "ships"],
  ["OBJ-01-C004", "facilities"],
  ["OBJ-01-C005", "missions"],
  ["OBJ-01-C006", "troops"],
  ["OBJ-01-C007", "personnel"],
  ["OBJ-01-C008", "topic text"],
  ["OBJ-01-C009", "topic art"],
  ["OBJ-01-C010", "previous"],
  ["OBJ-01-C011", "next"],
  ["OBJ-01-C012", "context open"],
  ["OBJ-01-C013", "missing entry"],
];

const allowedDispositions = new Set([
  "a0_available_both_factions",
  "a0_available_endpoint",
  "a0_available_real_build_selection_context",
  "strict_a0_not_applicable",
]);

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

export function validateCrosswalk(crosswalk) {
  assert.equal(crosswalk.kind, "open-rebellion-adapted-obj01-a0-crosswalk");
  assert.ok(Array.isArray(crosswalk.cells), "crosswalk cells must be an array");
  assert.equal(crosswalk.cells.length, expectedCells.length);
  for (const [index, [expectedId, expectedRequirement]] of expectedCells.entries()) {
    const cell = crosswalk.cells[index];
    assert.equal(cell.id, expectedId, `crosswalk cell ${index} has the wrong stable ID`);
    assert.equal(
      cell.requirement,
      expectedRequirement,
      `${expectedId} has the wrong requirement`,
    );
    assert.ok(
      allowedDispositions.has(cell.disposition),
      `${expectedId} has an unknown disposition`,
    );
    assert.ok(Array.isArray(cell.captures), `${expectedId} captures must be an array`);
    if (cell.disposition === "strict_a0_not_applicable") {
      assert.equal(expectedId, "OBJ-01-C013");
      assert.equal(cell.captures.length, 0, `${expectedId} cannot carry fabricated A0`);
      continue;
    }
    assert.ok(cell.captures.length > 0, `${expectedId} must retain an A0 capture`);
    for (const capture of cell.captures) {
      assert.match(capture.sha256, /^[0-9a-f]{64}$/, `${expectedId} has an invalid digest`);
      assert.equal(path.isAbsolute(capture.path), false, `${expectedId} capture is absolute`);
      assert.equal(capture.path.includes("\\"), false, `${expectedId} capture uses backslashes`);
      assert.equal(
        path.posix.normalize(capture.path),
        capture.path,
        `${expectedId} capture path is not normalized`,
      );
      assert.ok(
        capture.path.startsWith("E51/") && !capture.path.includes("../"),
        `${expectedId} capture escapes the ignored E51 root`,
      );
    }
  }
  return crosswalk;
}

export function validateSurfaceGeometry(original, production, width, height) {
  assert.equal(original.width, width, `original width is ${original.width}, expected ${width}`);
  assert.equal(original.height, height, `original height is ${original.height}, expected ${height}`);
  assert.equal(production.width, width, `production width is ${production.width}, expected ${width}`);
  assert.equal(production.height, height, `production height is ${production.height}, expected ${height}`);
  return { width, height };
}

function cropSurface(screenshot, origin) {
  assert.equal(screenshot.width, 640, "A0 and production screenshots must use the 640px surface");
  assert.equal(screenshot.height, 480, "A0 and production screenshots must use the 480px surface");
  const surface = new PNG({ width: SURFACE_WIDTH, height: SURFACE_HEIGHT });
  for (let y = 0; y < SURFACE_HEIGHT; y += 1) {
    const start = ((origin.y + y) * screenshot.width + origin.x) * 4;
    const end = start + SURFACE_WIDTH * 4;
    surface.data.set(screenshot.data.subarray(start, end), y * SURFACE_WIDTH * 4);
  }
  return surface;
}

function cropRegion(image, region) {
  const cropped = new PNG({ width: region.width, height: region.height });
  for (let y = 0; y < region.height; y += 1) {
    const start = ((region.y + y) * image.width + region.x) * 4;
    const end = start + region.width * 4;
    cropped.data.set(image.data.subarray(start, end), y * region.width * 4);
  }
  return cropped;
}

export function compareRegions(original, production, regions) {
  validateSurfaceGeometry(original, production, original.width, original.height);
  return regions.map((region) => {
    const left = cropRegion(original, region);
    const right = cropRegion(production, region);
    const differentPixels = pixelmatch(
      left.data,
      right.data,
      null,
      region.width,
      region.height,
      { threshold: region.threshold ?? 0.1 },
    );
    const pixels = region.width * region.height;
    const ratio = differentPixels / pixels;
    return {
      name: region.name,
      pixels,
      different_pixels: differentPixels,
      ratio,
      maximum_ratio: region.maximum_ratio,
      status: ratio <= region.maximum_ratio ? "pass" : "fail",
    };
  });
}

export function productionScreenshotPath(runDirectory, faction, requirement) {
  let filename;
  if (requirement === "index") filename = "02-index.png";
  else if (categoryFiles[requirement]) filename = categoryFiles[requirement];
  else if (["topic text", "topic art", "previous"].includes(requirement)) filename = "03-topic.png";
  else if (requirement === "next") filename = "05-last-ship-endpoint.png";
  else if (requirement === "context open") filename = "08-contextual-topic.png";
  else return null;
  return path.join(runDirectory, faction, filename);
}

function regionsFor(requirement) {
  const topicStructural = [
    { name: "top-chrome", x: 0, y: 0, width: 470, height: 78, maximum_ratio: 0.05 },
    { name: "right-rail", x: 412, y: 0, width: 58, height: 330, maximum_ratio: 0.01 },
    { name: "bottom-chrome", x: 0, y: 311, width: 412, height: 19, maximum_ratio: 0.06 },
  ];
  const indexStructural = [
    { name: "top-chrome", x: 0, y: 0, width: 470, height: 78, maximum_ratio: 0.05 },
    { name: "right-rail-above-tooltip", x: 412, y: 0, width: 58, height: 105, maximum_ratio: 0.01 },
    { name: "right-rail-below-tooltip", x: 412, y: 140, width: 58, height: 190, maximum_ratio: 0.01 },
    { name: "bottom-chrome", x: 0, y: 311, width: 412, height: 19, maximum_ratio: 0.06 },
  ];
  if (requirement === "context open") return [topicStructural[1]];
  if (requirement === "next") {
    return [
      { name: "left-frame", x: 0, y: 0, width: 28, height: 311, maximum_ratio: 0.03 },
      { name: "right-rail-below-tooltip", x: 412, y: 70, width: 58, height: 241, maximum_ratio: 0.01 },
      topicStructural[2],
    ];
  }
  if (["topic text", "topic art", "previous", "next"].includes(requirement)) {
    return [
      { name: "full-topic", x: 0, y: 0, width: 470, height: 330, maximum_ratio: 0.05 },
      ...topicStructural,
      { name: "topic-art", x: 12, y: 31, width: 400, height: 200, maximum_ratio: 0 },
      { name: "topic-body", x: 17, y: 231, width: 395, height: 80, maximum_ratio: 0.15 },
    ];
  }
  return [
    { name: "full-index", x: 0, y: 0, width: 470, height: 330, maximum_ratio: 0.12 },
    ...indexStructural,
    { name: "index-list", x: 36, y: 137, width: 350, height: 160, maximum_ratio: 0.24 },
  ];
}

function factionForCapture(capturePath, requirement) {
  const name = path.basename(capturePath);
  if (name.startsWith("alliance-")) return "alliance";
  if (name.startsWith("empire-")) return "empire";
  if (requirement === "context open") return "alliance";
  throw new Error(`cannot identify capture faction: ${capturePath}`);
}

function comparisonDiff(original, production) {
  const diff = new PNG({ width: SURFACE_WIDTH, height: SURFACE_HEIGHT });
  pixelmatch(
    original.data,
    production.data,
    diff.data,
    SURFACE_WIDTH,
    SURFACE_HEIGHT,
    { threshold: 0.1 },
  );
  return diff;
}

function safeName(value) {
  return value.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "");
}

export function runAcceptance({ crosswalkPath, a0Root, publicationRun }) {
  assert.ok(crosswalkPath && fs.existsSync(crosswalkPath), "set an existing A0 crosswalk path");
  assert.ok(a0Root && fs.existsSync(a0Root), "set an existing A0 capture root");
  assert.ok(publicationRun && fs.existsSync(publicationRun), "set an existing publication run");
  const publicationSummaryPath = path.join(publicationRun, "summary.json");
  assert.ok(fs.existsSync(publicationSummaryPath), "publication run is missing summary.json");
  const publicationSummaryBytes = fs.readFileSync(publicationSummaryPath);
  const publicationSummary = JSON.parse(publicationSummaryBytes);
  assert.equal(publicationSummary.status, "pass");

  const crosswalkBytes = fs.readFileSync(crosswalkPath);
  const crosswalk = validateCrosswalk(JSON.parse(crosswalkBytes));
  const outputDirectory = path.join(publicationRun, "a0-comparison");
  fs.mkdirSync(outputDirectory, { recursive: true });
  const results = [];
  for (const cell of crosswalk.cells) {
    if (cell.disposition === "strict_a0_not_applicable") {
      results.push({
        cell_id: cell.id,
        requirement: cell.requirement,
        status: "not-applicable",
        reason: cell.reason,
        comparisons: [],
      });
      continue;
    }
    const comparisons = [];
    for (const capture of cell.captures) {
      const faction = factionForCapture(capture.path, cell.requirement);
      const originalPath = path.join(a0Root, capture.path);
      const productionPath = productionScreenshotPath(publicationRun, faction, cell.requirement);
      assert.ok(productionPath, `no production mapping for ${cell.requirement}`);
      assert.ok(fs.existsSync(originalPath), `A0 capture is missing: ${originalPath}`);
      assert.ok(fs.existsSync(productionPath), `production screenshot is missing: ${productionPath}`);
      const originalBytes = fs.readFileSync(originalPath);
      assert.equal(sha256(originalBytes), capture.sha256, `${cell.id} A0 capture hash changed`);
      const productionBytes = fs.readFileSync(productionPath);
      const original = cropSurface(PNG.sync.read(originalBytes), origins[faction]);
      const production = cropSurface(PNG.sync.read(productionBytes), origins[faction]);
      validateSurfaceGeometry(original, production, SURFACE_WIDTH, SURFACE_HEIGHT);
      const regions = compareRegions(original, production, regionsFor(cell.requirement));
      const label = `${cell.id}-${faction}-${safeName(cell.requirement)}`;
      fs.writeFileSync(path.join(outputDirectory, `${label}-a0.png`), PNG.sync.write(original));
      fs.writeFileSync(path.join(outputDirectory, `${label}-production.png`), PNG.sync.write(production));
      fs.writeFileSync(
        path.join(outputDirectory, `${label}-diff.png`),
        PNG.sync.write(comparisonDiff(original, production)),
      );
      comparisons.push({
        faction,
        original: { path: capture.path, sha256: capture.sha256 },
        production: {
          path: path.relative(publicationRun, productionPath),
          sha256: sha256(productionBytes),
        },
        regions,
        status: regions.every(({ status }) => status === "pass") ? "pass" : "fail",
      });
    }
    results.push({
      cell_id: cell.id,
      requirement: cell.requirement,
      status: comparisons.every(({ status }) => status === "pass") ? "pass" : "fail",
      comparisons,
    });
  }
  const summary = {
    schema_version: 1,
    family: "encyclopedia-original-executable-wine-compatibility",
    status: results.every(({ status }) => status === "pass" || status === "not-applicable")
      ? "pass"
      : "fail",
    comparison_profile: "lossless 640x480 Wine A0 versus normal-route production browser; pixelmatch threshold 0.1 with explicit per-region maxima",
    limitations: [
      "A0 captures are Wine compatibility observations, not original-Windows rendering proof.",
      "The gameplay-only mission exclusion has no visible surface and therefore no A0 fixture.",
      "The contextual A0 and production captures use different source-proven callers, so only the caller-independent opaque rail is compared.",
      "The retained final-topic A0 has a documented Wine repaint lag and a different visible topic from its traced endpoint identity, so endpoint comparison is limited to caller-independent frame regions.",
    ],
    crosswalk: { sha256: sha256(crosswalkBytes), source_profile: crosswalk.source_profile },
    publication: {
      summary_sha256: sha256(publicationSummaryBytes),
      wasm_sha256: publicationSummary.wasm_sha256,
      runtime_pack_sha256: publicationSummary.runtime_pack_sha256,
    },
    cells: results,
  };
  fs.writeFileSync(
    path.join(outputDirectory, "summary.json"),
    `${JSON.stringify(summary, null, 2)}\n`,
  );
  return summary;
}

function main() {
  const summary = runAcceptance({
    crosswalkPath: process.env.OPEN_REBELLION_ENCYCLOPEDIA_A0_CROSSWALK,
    a0Root: process.env.OPEN_REBELLION_ENCYCLOPEDIA_A0_ROOT,
    publicationRun: process.env.OPEN_REBELLION_ENCYCLOPEDIA_PUBLICATION_RUN,
  });
  console.log(JSON.stringify(summary, null, 2));
  assert.equal(summary.status, "pass", "adapted A0 compatibility comparison contains failed regions");
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) main();
