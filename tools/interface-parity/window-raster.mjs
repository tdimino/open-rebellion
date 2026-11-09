import assert from "node:assert/strict";
import pixelmatch from "pixelmatch";

// The pixels of a 640x480 logical rect in a screenshot of `viewport`.
export function cropPng(png, viewport, rect) {
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const density = png.width / viewport.width;
  const x0 = Math.round(((viewport.width - 640 * scale) / 2 + rect.x * scale) * density);
  const y0 = Math.round(((viewport.height - 480 * scale) / 2 + rect.y * scale) * density);
  const width = Math.round(rect.width * scale * density);
  const height = Math.round(rect.height * scale * density);
  const out = Buffer.alloc(width * height * 4);
  for (let y = 0; y < height; y++) {
    const start = ((y0 + y) * png.width + x0) * 4;
    png.data.copy(out, y * width * 4, start, start + width * 4);
  }
  return out;
}

export function logicalCropDifference(
  actual, actualRect, expected, expectedRect, viewport, { threshold = 0.2 } = {},
) {
  const actualPixels = cropPng(actual, viewport, actualRect);
  const expectedPixels = cropPng(expected, viewport, expectedRect);
  assert.equal(actualPixels.length, expectedPixels.length,
    "logical window crops must have the same dimensions");
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const density = actual.width / viewport.width;
  const width = Math.round(actualRect.width * scale * density);
  const height = Math.round(actualRect.height * scale * density);
  const differentPixels = pixelmatch(
    actualPixels,
    expectedPixels,
    null,
    width,
    height,
    { threshold, includeAA: false },
  );
  return { differentPixels, differenceRatio: differentPixels / (width * height) };
}

// Dragging activates a window and legitimately changes its title strip. Keep
// the unchanged raster threshold meaningful by comparing the stable client
// body below that strip, while retaining the frame on both sides.
export function movableWindowBody({ x, y, width, height }) {
  assert.ok(width > 4 && height > 24, "movable window must contain a client body");
  return { x: x + 2, y: y + 22, width: width - 4, height: height - 24 };
}

// The 20-pixel title strip above that body, inside the frame.
export function movableWindowTitle({ x, y, width }) {
  assert.ok(width > 4, "movable window must have a title strip");
  return { x: x + 2, y: y + 2, width: width - 4, height: 20 };
}
