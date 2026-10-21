#!/usr/bin/env node

// C6: compare the port's tactical frames (A1) with the original's (A0), cell
// by cell, region by region, against the audit's numerical gates.
//
//   node tactical-a0-compare.mjs [--a1-run <run dir>] [--negative-controls] [--strict]
//
// The spec (scenarios/tactical-a0-compare.json) names each cell's A1 frame and
// the regions the cell is about. A `state` region is compared only when the
// A1 scenario reproduces the A0 capture's save (`state_matches`), or when the
// pair names the state its A1 frame shares with the A0 frame (`state_match`,
// a written justification); otherwise it is not-run, never a fail and never a
// pass.

import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import pixelmatch from "pixelmatch";
import { PNG } from "pngjs";
import { collectTacticalCells, TACTICAL_CELL_TOTAL } from "./tactical-matrix.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const specPath = path.join(here, "scenarios/tactical-a0-compare.json");
const ledgerPath = path.join(
  root, "docs/qa/2026-09-10-interface-parity-audit/2026-10-06-surface-ledger.json",
);
const auditPath = path.join(root, "docs/qa/2026-09-10-interface-parity-audit/audit-report.json");
const manifestPath = path.join(root, ".artifacts/interface-parity/a0/manifest.json");

const WIDTH = 640;
const HEIGHT = 480;
const DEFAULT_SEARCH_RADIUS = 2;
// Sobel magnitude on 0-255 luma above which a pixel is an edge.
const EDGE_MAGNITUDE = 64;
// The rows a split pair defers: both read exact pixel structure, which needs
// the original's 8-bit rasterizer rather than the frame's positions.
const SPLIT_DEFERRED_ROWS = ["ssim", "edge_iou"];

const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");

export function readThresholds(audit) {
  const t = audit.thresholds;
  return {
    ssim_min: t.ssim_lossless_min,
    mae_max: t.normalized_mae_max,
    changed_ratio_max: t.changed_pixel_ratio_max,
    changed_channel_delta: t.changed_pixel_channel_delta,
    edge_iou_min: t.edge_iou_lossless_min,
    displacement_max: t.layout_error_logical_px_max,
  };
}

export function validateSpec(spec, cells) {
  assert.equal(spec.schema_version, 1);
  assert.deepEqual(spec.canvas, { width: WIDTH, height: HEIGHT });
  for (const [name, region] of Object.entries(spec.regions)) {
    const [x, y, w, h] = region.rect;
    assert.ok(w > 0 && h > 0 && x >= 0 && y >= 0 && x + w <= WIDTH && y + h <= HEIGHT,
      `region ${name} leaves the canvas`);
    assert.ok(["chrome", "state"].includes(region.class), `region ${name} has no class`);
    assert.ok(typeof region.source === "string" && region.source.length > 0,
      `region ${name} cites no source`);
    for (const mask of region.masks ?? []) {
      assert.ok(spec.masks[mask], `region ${name} names unknown mask ${mask}`);
    }
  }
  for (const [name, mask] of Object.entries(spec.masks)) {
    assert.ok(typeof mask.justification === "string" && mask.justification.length > 20,
      `mask ${name} has no written justification`);
  }
  const ids = cells.map(({ id }) => id);
  assert.deepEqual(Object.keys(spec.cells).sort(), [...ids].sort(),
    "the compare spec must name exactly the tactical cells");
  assert.equal(ids.length, TACTICAL_CELL_TOTAL);
  for (const [id, cell] of Object.entries(spec.cells)) {
    assert.ok(Array.isArray(cell.pairs), `${id} has no pairs`);
    for (const pair of cell.pairs) {
      assert.match(pair.a1, /^[a-z0-9-]+\/[a-z0-9-]+$/, `${id} A1 frame must be scenario/frame`);
      assert.ok(["original", "before", undefined].includes(pair.a0), `${id} has an unknown A0 frame`);
      assert.ok(pair.state_match === undefined
        || (typeof pair.state_match === "string" && pair.state_match.length > 20),
      `${id} state_match needs a written justification`);
      assert.ok(pair.regions.length > 0, `${id} pair compares no region`);
      for (const mask of pair.masks ?? []) {
        assert.ok(spec.masks[mask], `${id} names unknown mask ${mask}`);
      }
      for (const region of pair.regions) {
        assert.ok(spec.regions[region], `${id} names unknown region ${region}`);
      }
      if (pair.split) {
        assert.ok(pair.split.regions?.length > 0
          && pair.split.regions.every((region) => pair.regions.includes(region)),
        `${id} split names a region the pair does not compare`);
        assert.match(pair.split.node ?? "", /^or-[a-z0-9-]+$/, `${id} split names no br node`);
        assert.ok(typeof pair.split.reason === "string" && pair.split.reason.length > 20,
          `${id} split needs a written reason`);
      }
    }
  }
  return spec;
}

function luma(data, offset) {
  return 0.299 * data[offset] + 0.587 * data[offset + 1] + 0.114 * data[offset + 2];
}

function maskedAt(masks, x, y) {
  return masks.some(([mx, my, mw, mh]) => x >= mx && y >= my && x < mx + mw && y < my + mh);
}

// Lumas of a w×h block of `image` whose top-left is (x, y).
function lumaBlock(image, x, y, w, h) {
  const out = new Float64Array(w * h);
  for (let row = 0; row < h; row++) {
    for (let col = 0; col < w; col++) {
      out[row * w + col] = luma(image.data, ((y + row) * image.width + x + col) * 4);
    }
  }
  return out;
}

// Mean SSIM of 8×8 windows at stride 4 (C1, C2 from Wang et al. 2004), over
// windows holding no masked pixel.
function ssim(a, b, w, h, keep) {
  const size = Math.min(8, w, h);
  const step = Math.max(1, size >> 1);
  const c1 = (0.01 * 255) ** 2;
  const c2 = (0.03 * 255) ** 2;
  let total = 0;
  let windows = 0;
  for (let y = 0; y + size <= h; y += step) {
    for (let x = 0; x + size <= w; x += step) {
      let ma = 0, mb = 0, n = 0, skip = false;
      for (let j = 0; j < size && !skip; j++) {
        for (let i = 0; i < size; i++) {
          const k = (y + j) * w + x + i;
          if (!keep[k]) { skip = true; break; }
          ma += a[k]; mb += b[k]; n++;
        }
      }
      if (skip) continue;
      ma /= n; mb /= n;
      let va = 0, vb = 0, cov = 0;
      for (let j = 0; j < size; j++) {
        for (let i = 0; i < size; i++) {
          const k = (y + j) * w + x + i;
          va += (a[k] - ma) ** 2; vb += (b[k] - mb) ** 2; cov += (a[k] - ma) * (b[k] - mb);
        }
      }
      va /= n - 1; vb /= n - 1; cov /= n - 1;
      total += ((2 * ma * mb + c1) * (2 * cov + c2)) / ((ma * ma + mb * mb + c1) * (va + vb + c2));
      windows++;
    }
  }
  return windows ? total / windows : null;
}

function edgeMask(l, w, h) {
  const edges = new Uint8Array(w * h);
  for (let y = 1; y < h - 1; y++) {
    for (let x = 1; x < w - 1; x++) {
      const p = (dx, dy) => l[(y + dy) * w + x + dx];
      const gx = p(1, -1) + 2 * p(1, 0) + p(1, 1) - p(-1, -1) - 2 * p(-1, 0) - p(-1, 1);
      const gy = p(-1, 1) + 2 * p(0, 1) + p(1, 1) - p(-1, -1) - 2 * p(0, -1) - p(1, -1);
      edges[y * w + x] = Math.hypot(gx, gy) > EDGE_MAGNITUDE ? 1 : 0;
    }
  }
  return edges;
}

// Mean absolute RGB difference between A1's rect and A0's rect moved by
// (dx, dy), sampling every `stride` pixels.
function offsetMae(a0, a1, [x, y, w, h], dx, dy, masks, stride) {
  let sum = 0, n = 0;
  for (let row = 0; row < h; row += stride) {
    for (let col = 0; col < w; col += stride) {
      if (maskedAt(masks, x + col, y + row)) continue;
      const i1 = ((y + row) * WIDTH + x + col) * 4;
      const i0 = ((y + row + dy) * WIDTH + x + col + dx) * 4;
      sum += Math.abs(a0.data[i0] - a1.data[i1]) + Math.abs(a0.data[i0 + 1] - a1.data[i1 + 1])
        + Math.abs(a0.data[i0 + 2] - a1.data[i1 + 2]);
      n += 3;
    }
  }
  return n ? sum / n / 255 : 0;
}

// The offset of A0's content relative to A1's within the radius: the A1 rect
// matches A0 best at rect + (dx, dy). Coarse sampled search, then exact.
function bestOffset(a0, a1, rect, masks, radius) {
  const [x, y, w, h] = rect;
  const inside = (dx, dy) => x + dx >= 0 && y + dy >= 0 && x + w + dx <= WIDTH && y + h + dy <= HEIGHT;
  const search = (centre, reach, stride) => {
    let best = null;
    for (let dy = centre.dy - reach; dy <= centre.dy + reach; dy++) {
      for (let dx = centre.dx - reach; dx <= centre.dx + reach; dx++) {
        if (!inside(dx, dy)) continue;
        const mae = offsetMae(a0, a1, rect, dx, dy, masks, stride);
        if (!best || mae < best.mae - 1e-12
          || (Math.abs(mae - best.mae) <= 1e-12 && Math.hypot(dx, dy) < Math.hypot(best.dx, best.dy))) {
          best = { dx, dy, mae };
        }
      }
    }
    return best;
  };
  const coarseStride = radius > DEFAULT_SEARCH_RADIUS ? 4 : 1;
  const coarse = search({ dx: 0, dy: 0 }, radius, coarseStride);
  return coarseStride === 1 ? coarse : search(coarse, 3, 1);
}

export function compareRegion(a0, a1, rect, masks, thresholds, radius = DEFAULT_SEARCH_RADIUS) {
  const [x, y, w, h] = rect;
  const keep = new Uint8Array(w * h);
  let kept = 0, changed = 0, absolute = 0;
  for (let row = 0; row < h; row++) {
    for (let col = 0; col < w; col++) {
      if (maskedAt(masks, x + col, y + row)) continue;
      keep[row * w + col] = 1;
      kept++;
      const i = ((y + row) * WIDTH + x + col) * 4;
      let maxDelta = 0;
      for (let c = 0; c < 3; c++) {
        const delta = Math.abs(a0.data[i + c] - a1.data[i + c]);
        absolute += delta;
        maxDelta = Math.max(maxDelta, delta);
      }
      if (maxDelta > thresholds.changed_channel_delta) changed++;
    }
  }
  const l0 = lumaBlock(a0, x, y, w, h);
  const l1 = lumaBlock(a1, x, y, w, h);
  const e0 = edgeMask(l0, w, h);
  const e1 = edgeMask(l1, w, h);
  // The Sobel kernel reads a pixel's eight neighbours, so an edge beside a
  // masked pixel is the mask's content and is not compared either.
  const edgeKept = (k) => {
    const row = Math.floor(k / w), col = k % w;
    for (let dy = -1; dy <= 1; dy++) {
      for (let dx = -1; dx <= 1; dx++) {
        const r = row + dy, c = col + dx;
        if (r >= 0 && c >= 0 && r < h && c < w && !keep[r * w + c]) return false;
      }
    }
    return true;
  };
  let both = 0, either = 0;
  for (let k = 0; k < w * h; k++) {
    if (!keep[k] || !edgeKept(k)) continue;
    if (e0[k] && e1[k]) both++;
    if (e0[k] || e1[k]) either++;
  }
  const offset = bestOffset(a0, a1, rect, masks, radius);
  const metrics = {
    ssim: ssim(l0, l1, w, h, keep),
    normalized_mae: kept ? absolute / (kept * 3) / 255 : 0,
    changed_pixel_ratio: kept ? changed / kept : 0,
    edge_iou: either ? both / either : 1,
    displacement_px: Math.max(Math.abs(offset.dx), Math.abs(offset.dy)),
  };
  const rows = {
    ssim: metrics.ssim === null || metrics.ssim >= thresholds.ssim_min,
    normalized_mae: metrics.normalized_mae <= thresholds.mae_max,
    changed_pixel_ratio: metrics.changed_pixel_ratio <= thresholds.changed_ratio_max,
    edge_iou: metrics.edge_iou >= thresholds.edge_iou_min,
    displacement_px: metrics.displacement_px <= thresholds.displacement_max,
  };
  return {
    status: Object.values(rows).every(Boolean) ? "pass" : "fail",
    failed_rows: Object.keys(rows).filter((row) => !rows[row]),
    metrics,
    best_offset: offset,
    compared_pixels: kept,
  };
}

function faction(capture) {
  return capture.state_setup.identifier.startsWith("alliance-") ? "alliance" : "empire";
}

function a0FramePath(capture, which) {
  const name = which === "before" ? "before.png" : "original.png";
  const artifact = capture.artifacts.find(({ path: p }) => p.endsWith(`/${name}`));
  return artifact ? { path: path.join(root, artifact.path), sha256: artifact.sha256 } : null;
}

function regionVerdictForState(spec, regionName, capture, scenario) {
  const region = spec.regions[regionName];
  if (region.class === "chrome") return null;
  const matched = spec.state_matches[scenario] ?? [];
  if (matched.includes(capture.state_setup.identifier)) return null;
  return `state region; the A1 scenario ${scenario} does not reproduce save ${capture.state_setup.identifier}`;
}

function cellVerdict(statuses) {
  if (statuses.includes("fail")) return "fail";
  if (statuses.length === 0 || statuses.includes("not-run")) return "not-run";
  return "pass";
}

export function compareCell({ id, spec, capture, a1Run, thresholds, frames = {}, outDir = null }) {
  const cell = spec.cells[id];
  // A cell the original has no state for is settled, not awaiting a reference.
  if (cell.settled) return { cell_id: id, status: "settled", reason: cell.settled, pairs: [] };
  if (!capture) return { cell_id: id, status: "reference-needed", reason: "no A0 capture", pairs: [] };
  if (cell.pairs.length === 0) {
    return { cell_id: id, status: "not-run", reason: cell.not_run_reason ?? "no A1 frame mapped", pairs: [] };
  }
  const side = faction(capture);
  const pairs = cell.pairs.map((pair, index) => {
    const which = pair.a0 ?? "original";
    const [scenario, frame] = pair.a1.split("/");
    const a0Ref = a0FramePath(capture, which);
    const a1File = path.join(a1Run, "tactical", side, scenario, "640x480", `${frame}.png`);
    const base = { a0: which, a1: `${side}/${pair.a1}` };
    if (!a0Ref) return { ...base, status: "not-run", reason: `no A0 ${which} frame`, regions: [] };
    const a1Bytes = frames.a1 ?? (fs.existsSync(a1File) ? fs.readFileSync(a1File) : null);
    if (!a1Bytes) return { ...base, status: "not-run", reason: "A1 frame missing from the run", regions: [] };
    const a0Bytes = frames.a0 ?? fs.readFileSync(a0Ref.path);
    if (!frames.a0) assert.equal(sha256(a0Bytes), a0Ref.sha256, `${id} A0 ${which} frame hash changed`);
    const a0 = PNG.sync.read(a0Bytes);
    const a1 = PNG.sync.read(a1Bytes);
    if (a1.width !== WIDTH || a1.height !== HEIGHT) {
      return { ...base, status: "fail", reason: `A1 frame is ${a1.width}x${a1.height}`, regions: [] };
    }
    const regions = pair.regions.map((name) => {
      const region = spec.regions[name];
      const unmatched = pair.state_match ? null : regionVerdictForState(spec, name, capture, scenario);
      if (unmatched) return { region: name, class: region.class, status: "not-run", reason: unmatched };
      // A region's masks, then the pair's own (an A0 frame's pointer).
      const names = [...(region.masks ?? []), ...(pair.masks ?? [])];
      const masks = names.map((mask) => spec.masks[mask].rect);
      const result = compareRegion(a0, a1, region.rect, masks, thresholds,
        region.search_radius ?? DEFAULT_SEARCH_RADIUS);
      // A split region is judged on position alone (displacement, MAE and
      // changed pixels); its SSIM and edge rows wait on the named node and
      // are reported, never dropped.
      if (pair.split?.regions.includes(name)) {
        const deferred = result.failed_rows.filter((row) => SPLIT_DEFERRED_ROWS.includes(row));
        result.failed_rows = result.failed_rows.filter((row) => !SPLIT_DEFERRED_ROWS.includes(row));
        result.status = result.failed_rows.length === 0 ? "pass" : "fail";
        result.split = { node: pair.split.node, deferred_rows: deferred };
      }
      if (outDir && result.status === "fail") {
        const [x, y, w, h] = region.rect;
        const crop = (image) => {
          const out = new PNG({ width: w, height: h });
          PNG.bitblt(image, out, x, y, w, h, 0, 0);
          return out;
        };
        const left = crop(a0), right = crop(a1);
        const diff = new PNG({ width: w, height: h });
        pixelmatch(left.data, right.data, diff.data, w, h, { threshold: 0 });
        const sheet = new PNG({ width: w * 3, height: h });
        PNG.bitblt(left, sheet, 0, 0, w, h, 0, 0);
        PNG.bitblt(right, sheet, 0, 0, w, h, w, 0);
        PNG.bitblt(diff, sheet, 0, 0, w, h, 2 * w, 0);
        fs.writeFileSync(path.join(outDir, `${id}-${index}-${name}.png`), PNG.sync.write(sheet));
      }
      return { region: name, class: region.class, masks: names, ...result };
    });
    return {
      ...base,
      a0_sha256: sha256(a0Bytes),
      a1_sha256: sha256(a1Bytes),
      status: cellVerdict(regions.map(({ status }) => status)),
      regions,
    };
  });
  const status = cellVerdict(pairs.map(({ status }) => status));
  return { cell_id: id, status, ...(cell.audio ? { audio: "not compared" } : {}), pairs };
}

// Each control perturbs a pair of identical A0 frames, so the comparator
// passes the unperturbed pair and must fail the perturbed one on the named
// row. Returns one row per control.
export function negativeControls({ spec, captures, thresholds }) {
  const read = (cellId, which = "original") =>
    PNG.sync.read(fs.readFileSync(a0FramePath(captures.get(cellId), which).path));
  const run = (a0, a1, regionName) => {
    const region = spec.regions[regionName];
    const masks = (region.masks ?? []).map((mask) => spec.masks[mask].rect);
    return compareRegion(a0, a1, region.rect, masks, thresholds,
      region.search_radius ?? DEFAULT_SEARCH_RADIUS);
  };
  const copy = (image) => {
    const out = new PNG({ width: WIDTH, height: HEIGHT });
    image.data.copy(out.data);
    return out;
  };
  const results = [];
  const expect = (control, result, wanted) => {
    const ok = wanted === "pass"
      ? result.status === "pass"
      : result.status === "fail" && result.failed_rows.includes(wanted);
    results.push({ control, expected: wanted, status: result.status, failed_rows: result.failed_rows, ok });
  };

  // Identity: the maneuver panel against itself passes.
  const anvil = read("TAC-05-C003");
  expect("identity", run(anvil, copy(anvil), "maneuver-anvil"), "pass");

  // One-pixel geometry: the Anvil button drawn one pixel right. The audit
  // allows displacement up to 1 px, so that row cannot fail here; the edge
  // IoU row must, and the offset search must name the 1 px shift.
  const shifted = copy(anvil);
  const [ax, ay, aw, ah] = spec.regions["maneuver-anvil"].rect;
  PNG.bitblt(anvil, shifted, ax, ay, aw, ah, ax + 1, ay);
  const geometry = run(anvil, shifted, "maneuver-anvil");
  expect("one-pixel geometry", geometry, "edge_iou");
  results.at(-1).ok &&= geometry.metrics.displacement_px === 1;
  results.at(-1).measured_displacement_px = geometry.metrics.displacement_px;

  // Wrong bitmap: the Hammer button's art drawn in the Anvil slot.
  const wrong = copy(anvil);
  const [hx, hy] = spec.regions["maneuver-hammer"].rect;
  PNG.bitblt(anvil, wrong, hx, hy, aw, ah, ax, ay);
  expect("wrong bitmap", run(anvil, wrong, "maneuver-anvil"), "changed_pixel_ratio");

  // Wrong action: Hammer chosen (TAC-05-C002) where Anvil was chosen.
  expect("wrong action", run(anvil, read("TAC-05-C002"), "maneuver-anvil"), "changed_pixel_ratio");

  // Stale state: task force 1 held down (TAC-02-C010) against the frame
  // before the press, the pointer resting on it (TAC-02-C009).
  expect("stale state",
    run(read("TAC-02-C010"), read("TAC-02-C009"), "task-force-1"), "changed_pixel_ratio");

  // Missing audio: the sound cell may not pass without an audio comparison.
  const audio = compareCell({
    id: "TAC-01-C012", spec, capture: captures.get("TAC-01-C012"), a1Run: "", thresholds,
  });
  results.push({
    control: "missing audio", expected: "not pass", status: audio.status,
    failed_rows: [], ok: audio.status !== "pass",
  });
  return results;
}

function markdown(report) {
  const lines = [
    "# Tactical A0 comparison",
    "",
    `Generated by \`tools/interface-parity/tactical-a0-compare.mjs\` from A1 run \`${report.a1_run}\`.`,
    "",
    `Pass ${report.summary.pass}, fail ${report.summary.fail}, not-run ${report.summary["not-run"]}, ` +
      `reference-needed ${report.summary["reference-needed"]}, settled ${report.summary.settled} (of ${report.summary.total}).`,
    "",
    "| Cell | Status | Detail |",
    "|---|---|---|",
  ];
  for (const cell of report.cells) {
    let detail = cell.reason ?? "";
    if (!detail) {
      detail = cell.pairs.flatMap((pair) => (pair.reason ? [`${pair.a0}: ${pair.reason}`] : pair.regions
        .filter(({ status, split }) => status !== "pass" || split?.deferred_rows.length)
        .map((r) => {
          if (r.status === "pass") return `${r.region}: split, ${r.split.deferred_rows.join(", ")} deferred to ${r.split.node}`;
          return r.status === "fail"
            ? `${r.region}: ${r.failed_rows.join(", ")}`
            : `${r.region}: not-run (${r.class})`;
        }))).join("; ");
    }
    lines.push(`| ${cell.cell_id} | ${cell.status} | ${detail} |`);
  }
  if (report.negative_controls) {
    lines.push("", "## Negative controls", "", "| Control | Expected | Result |", "|---|---|---|");
    for (const row of report.negative_controls) {
      lines.push(`| ${row.control} | ${row.expected} | ${row.ok ? "ok" : "WRONG"}: ${row.status} ${row.failed_rows.join(", ")} |`);
    }
  }
  return `${lines.join("\n")}\n`;
}

function latestA1Run() {
  const base = path.join(root, ".artifacts/interface-parity");
  const runs = fs.readdirSync(base)
    .filter((name) => /^\d{4}-\d{2}-\d{2}T/.test(name) && fs.existsSync(path.join(base, name, "tactical")))
    .sort();
  assert.ok(runs.length, "no tactical A1 run under .artifacts/interface-parity");
  return path.join(base, runs.at(-1));
}

function main() {
  const args = process.argv.slice(2);
  const runArg = args.indexOf("--a1-run");
  const a1Run = path.resolve(runArg >= 0 ? args[runArg + 1] : latestA1Run());
  const cells = collectTacticalCells(JSON.parse(fs.readFileSync(ledgerPath, "utf8")));
  const spec = validateSpec(JSON.parse(fs.readFileSync(specPath, "utf8")), cells);
  const thresholds = readThresholds(JSON.parse(fs.readFileSync(auditPath, "utf8")));
  const manifest = JSON.parse(fs.readFileSync(manifestPath, "utf8"));
  const captures = new Map(manifest.captures.map((capture) => [capture.cell_id, capture]));

  const stamp = new Date().toISOString().replace(/[:.]/g, "-");
  const outDir = path.join(root, ".artifacts/interface-parity/c6", stamp);
  const regionDir = path.join(outDir, "regions");
  fs.mkdirSync(path.join(outDir, "cells"), { recursive: true });
  fs.mkdirSync(regionDir, { recursive: true });

  const results = cells.map(({ id }) => {
    const result = compareCell({
      id, spec, capture: captures.get(id), a1Run, thresholds, outDir: regionDir,
    });
    fs.writeFileSync(path.join(outDir, "cells", `${id}.json`), `${JSON.stringify(result, null, 2)}\n`);
    return result;
  });
  const count = (status) => results.filter((cell) => cell.status === status).length;
  const report = {
    schema_version: 1,
    family: spec.family,
    a1_run: path.relative(root, a1Run),
    spec_sha256: sha256(fs.readFileSync(specPath)),
    manifest_sha256: sha256(fs.readFileSync(manifestPath)),
    thresholds,
    summary: {
      total: results.length,
      pass: count("pass"),
      fail: count("fail"),
      "not-run": count("not-run"),
      "reference-needed": count("reference-needed"),
      settled: count("settled"),
    },
    ...(args.includes("--negative-controls")
      ? { negative_controls: negativeControls({ spec, captures, thresholds }) }
      : {}),
    cells: results,
  };
  fs.writeFileSync(path.join(outDir, "report.json"), `${JSON.stringify(report, null, 2)}\n`);
  fs.writeFileSync(path.join(outDir, "report.md"), markdown(report));
  process.stdout.write(`${JSON.stringify({ ...report.summary, report: path.relative(root, outDir) })}\n`);
  if (report.negative_controls?.some(({ ok }) => !ok)) {
    process.stderr.write("a negative control did not fail its row\n");
    process.exitCode = 1;
  }
  if (args.includes("--strict") && report.summary.pass !== TACTICAL_CELL_TOTAL) process.exitCode = 1;
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) main();
