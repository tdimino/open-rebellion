import assert from "node:assert/strict";
import test from "node:test";
import { PNG } from "pngjs";
import { compareCell } from "./tactical-a0-compare.mjs";

const THRESHOLDS = {
  ssim_min: 0.995,
  mae_max: 0.0039215686,
  changed_ratio_max: 0.005,
  changed_channel_delta: 2,
  edge_iou_min: 0.98,
  displacement_max: 1,
};

function frame() {
  const png = new PNG({ width: 640, height: 480 });
  for (let index = 0; index < 640 * 480; index += 1) {
    png.data[index * 4] = (index * 7) % 256;
    png.data[index * 4 + 1] = (index * 13) % 256;
    png.data[index * 4 + 2] = (index * 29) % 256;
    png.data[index * 4 + 3] = 255;
  }
  return PNG.sync.write(png);
}

function spec(pair) {
  return {
    regions: {
      gauge: { rect: [508, 9, 95, 6], class: "state", source: "test" },
    },
    masks: {},
    cells: { "TAC-05-C018": { pairs: [pair] } },
    state_matches: {},
  };
}

const capture = {
  cell_id: "TAC-05-C018",
  state_setup: { identifier: "empire-death-star-pre-alert" },
  artifacts: [{ path: "a0/TAC-05-C018/original.png", sha256: "unused" }],
};

test("a state region stays unrun when its scenario does not reproduce the save", () => {
  const bytes = frame();
  const result = compareCell({
    id: "TAC-05-C018",
    spec: spec({ a1: "laser/committed", regions: ["gauge"] }),
    capture,
    a1Run: "",
    thresholds: THRESHOLDS,
    frames: { a0: bytes, a1: bytes },
  });
  assert.equal(result.status, "not-run");
});

test("a pair whose frame holds the save's state compares its state regions", () => {
  const bytes = frame();
  const result = compareCell({
    id: "TAC-05-C018",
    spec: spec({
      a1: "laser/committed",
      regions: ["gauge"],
      state_match: "both frames hold the charge at 0 just after the shot",
    }),
    capture,
    a1Run: "",
    thresholds: THRESHOLDS,
    frames: { a0: bytes, a1: bytes },
  });
  assert.equal(result.status, "pass");
  assert.equal(result.pairs[0].regions[0].status, "pass");
});

test("a pair's mask leaves the A0 pointer out of its regions", () => {
  const bytes = frame();
  const pointed = PNG.sync.read(bytes);
  for (let y = 10; y < 14; y += 1) {
    for (let x = 520; x < 524; x += 1) pointed.data.fill(255, (y * 640 + x) * 4, (y * 640 + x) * 4 + 3);
  }
  const masked = spec({
    a1: "laser/committed",
    regions: ["gauge"],
    masks: ["pointer"],
    state_match: "both frames hold the charge at 0 just after the shot",
  });
  masked.masks.pointer = { rect: [520, 10, 4, 4], justification: "the A0 capture's mouse pointer" };
  const run = (pairSpec) => compareCell({
    id: "TAC-05-C018",
    spec: pairSpec,
    capture,
    a1Run: "",
    thresholds: THRESHOLDS,
    frames: { a0: PNG.sync.write(pointed), a1: bytes },
  });
  assert.equal(run(masked).status, "pass");
  delete masked.cells["TAC-05-C018"].pairs[0].masks;
  assert.equal(run(masked).status, "fail");
});

test("an edge read across a mask's border is not compared", () => {
  // The Sobel kernel at a kept pixel next to the mask reads the pointer's
  // pixels; over a flat button those are the only edges in either frame.
  const flat = new PNG({ width: 640, height: 480 });
  flat.data.fill(128);
  const pointed = new PNG({ width: 640, height: 480 });
  pointed.data.fill(128);
  for (let y = 10; y < 14; y += 1) {
    for (let x = 520; x < 524; x += 1) pointed.data.fill(255, (y * 640 + x) * 4, (y * 640 + x) * 4 + 3);
  }
  const masked = spec({
    a1: "laser/committed",
    regions: ["gauge"],
    masks: ["pointer"],
    state_match: "both frames hold the charge at 0 just after the shot",
  });
  masked.masks.pointer = { rect: [520, 10, 4, 4], justification: "the A0 capture's mouse pointer" };
  const result = compareCell({
    id: "TAC-05-C018",
    spec: masked,
    capture,
    a1Run: "",
    thresholds: THRESHOLDS,
    frames: { a0: PNG.sync.write(pointed), a1: PNG.sync.write(flat) },
  });
  assert.equal(result.pairs[0].regions[0].metrics.edge_iou, 1);
  assert.equal(result.status, "pass");
});

test("a split region passes on position alone and still reports its deferred rows", () => {
  const bytes = frame();
  const shifted = PNG.sync.read(bytes);
  // One pixel's value changes: positions hold, the structure rows do not.
  for (let x = 508; x < 603; x += 2) shifted.data.fill(0, (11 * 640 + x) * 4, (11 * 640 + x) * 4 + 3);
  const split = spec({
    a1: "laser/committed",
    regions: ["gauge"],
    state_match: "both frames hold the charge at 0 just after the shot",
    split: { regions: ["gauge"], node: "or-p-test-node", reason: "the structure rows need the original rasterizer" },
  });
  const run = (pairSpec) => compareCell({
    id: "TAC-05-C018",
    spec: pairSpec,
    capture,
    a1Run: "",
    thresholds: { ...THRESHOLDS, changed_ratio_max: 1, mae_max: 1 },
    frames: { a0: bytes, a1: PNG.sync.write(shifted) },
  });
  const region = run(split).pairs[0].regions[0];
  assert.equal(region.status, "pass");
  assert.ok(region.split.deferred_rows.length > 0);
  delete split.cells["TAC-05-C018"].pairs[0].split;
  assert.equal(run(split).status, "fail");
});
