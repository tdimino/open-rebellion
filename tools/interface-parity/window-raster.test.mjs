import assert from "node:assert/strict";
import test from "node:test";

import { PNG } from "pngjs";

import { logicalCropDifference, movableWindowBody } from "./window-raster.mjs";

const VIEWPORT = { width: 640, height: 480 };
const ORIGINAL = { x: 225, y: 76, width: 226, height: 304 };
const TITLE_MOVED = { ...ORIGINAL, x: 185, y: 56 };
const BODY_MOVED = { ...TITLE_MOVED, x: 215, y: 66 };
const MAX_DRAG_RASTER_DIFFERENCE = 0.055;

function fixtureImage() {
  const png = new PNG(VIEWPORT);
  png.data.fill(255);
  for (let offset = 0; offset < png.data.length; offset += 4) {
    png.data[offset] = 7;
    png.data[offset + 1] = 11;
    png.data[offset + 2] = 13;
  }
  return png;
}

function paintWindow(png, rect, titleColor) {
  for (let y = 0; y < rect.height; y += 1) {
    for (let x = 0; x < rect.width; x += 1) {
      const offset = ((rect.y + y) * png.width + rect.x + x) * 4;
      const body = y >= 22 && y < rect.height - 2 && x >= 2 && x < rect.width - 2;
      const color = body
        ? [(x * 17 + y * 3) % 251, (x * 5 + y * 19) % 251, (x * 11 + y * 7) % 251]
        : titleColor;
      png.data[offset] = color[0];
      png.data[offset + 1] = color[1];
      png.data[offset + 2] = color[2];
      png.data[offset + 3] = 255;
    }
  }
}

function windowContent({ x, y, width, height }) {
  return { x: x + 2, y: y + 2, width: width - 4, height: height - 4 };
}

test("System drag comparison ignores activation paint but rejects an unmoved body", () => {
  const before = fixtureImage();
  paintWindow(before, ORIGINAL, [70, 130, 70]);

  const afterTitleDrag = fixtureImage();
  paintWindow(afterTitleDrag, TITLE_MOVED, [0, 255, 0]);
  const afterBodyDrag = fixtureImage();
  paintWindow(afterBodyDrag, BODY_MOVED, [0, 255, 0]);

  const activationPaint = logicalCropDifference(
    afterTitleDrag,
    windowContent(TITLE_MOVED),
    before,
    windowContent(ORIGINAL),
    VIEWPORT,
  );
  assert.ok(activationPaint.differenceRatio > MAX_DRAG_RASTER_DIFFERENCE,
    "whole-window comparison must expose the activation-paint false failure");

  const expected = movableWindowBody(ORIGINAL);
  for (const [label, png, moved] of [
    ["title", afterTitleDrag, TITLE_MOVED],
    ["bare surface", afterBodyDrag, BODY_MOVED],
  ]) {
    const correct = logicalCropDifference(
      png,
      movableWindowBody(moved),
      before,
      expected,
      VIEWPORT,
    );
    assert.equal(correct.differenceRatio, 0, `${label} drag changed stable client pixels`);

    const unmoved = logicalCropDifference(png, expected, before, expected, VIEWPORT);
    assert.ok(unmoved.differenceRatio > MAX_DRAG_RASTER_DIFFERENCE,
      `${label} regression would accept an unmoved window`);
  }
});
