import assert from "node:assert/strict";
import test from "node:test";
import { encyclopediaSurfaceGeometry } from "./encyclopedia-surface-geometry.mjs";

test("BMP resource comparison ignores only transparent pixels", async () => {
  const { countMaskedRgbDifferences } = await import("./encyclopedia-surface-geometry.mjs");
  assert.equal(typeof countMaskedRgbDifferences, "function", "resource oracle must expose masked RGB comparison");

  const width = 21;
  const height = 17;
  const pixels = width * height;
  const expected = new Uint8Array(pixels * 4);
  const actual = new Uint8Array(pixels * 4);
  const opaque = new Uint8Array(pixels);
  expected.fill(255);
  actual.fill(255);
  opaque.fill(1);
  for (let index = 0; index < 123; index += 1) {
    opaque[index] = 0;
    actual[index * 4] = 0;
  }

  assert.equal(opaque.reduce((count, value) => count + value, 0), 234);
  assert.equal(countMaskedRgbDifferences(actual, expected, opaque), 0);

  actual[123 * 4] = 0;
  assert.equal(countMaskedRgbDifferences(actual, expected, opaque), 1);
});

test("Encyclopedia art geometry uses each faction's native 640x480 origin", () => {
  assert.deepEqual(
    encyclopediaSurfaceGeometry("alliance", { width: 640, height: 480 }, {
      forward: { x: 10, y: 8 }, index: { x: 2, y: 3 },
    }),
    {
      scale: 1,
      origin: { x: 62, y: 50 },
      artRect: { x: 74, y: 81, width: 400, height: 200 },
      forwardPoint: { x: 452, y: 72 },
      indexPoint: { x: 487, y: 200 },
      disabledBackRect: { x: 90, y: 64, width: 21, height: 17 },
    },
  );
  assert.deepEqual(
    encyclopediaSurfaceGeometry("empire", { width: 640, height: 480 }, {
      forward: { x: 10, y: 8 }, index: { x: 2, y: 3 },
    }),
    {
      scale: 1,
      origin: { x: 125, y: 52 },
      artRect: { x: 137, y: 83, width: 400, height: 200 },
      forwardPoint: { x: 515, y: 74 },
      indexPoint: { x: 553, y: 198 },
      disabledBackRect: { x: 153, y: 66, width: 21, height: 17 },
    },
  );
});

test("Encyclopedia art geometry preserves faction origins at a 1.25x viewport scale", () => {
  assert.deepEqual(
    encyclopediaSurfaceGeometry("alliance", { width: 800, height: 600 }, {
      forward: { x: 10, y: 8 }, index: { x: 2, y: 3 },
    }),
    {
      scale: 1.25,
      origin: { x: 78, y: 63 },
      artRect: { x: 93, y: 102, width: 500, height: 250 },
      forwardPoint: { x: 565.5, y: 90.5 },
      indexPoint: { x: 609.25, y: 250.5 },
      disabledBackRect: { x: 113, y: 81, width: 26, height: 21 },
    },
  );
  assert.deepEqual(
    encyclopediaSurfaceGeometry("empire", { width: 800, height: 600 }, {
      forward: { x: 10, y: 8 }, index: { x: 2, y: 3 },
    }),
    {
      scale: 1.25,
      origin: { x: 156, y: 65 },
      artRect: { x: 171, y: 104, width: 500, height: 250 },
      forwardPoint: { x: 643.5, y: 92.5 },
      indexPoint: { x: 691, y: 247.5 },
      disabledBackRect: { x: 191, y: 83, width: 26, height: 21 },
    },
  );
});
