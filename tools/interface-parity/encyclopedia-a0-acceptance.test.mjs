import assert from "node:assert/strict";
import test from "node:test";
import { PNG } from "pngjs";

import {
  compareRegions,
  productionScreenshotPath,
  validateCrosswalk,
  validateSurfaceGeometry,
} from "./encyclopedia-a0-acceptance.mjs";

function solid(width, height, value) {
  const png = new PNG({ width, height });
  for (let offset = 0; offset < png.data.length; offset += 4) {
    png.data.set([value, value, value, 255], offset);
  }
  return png;
}

test("A0 comparison keeps geometry and region thresholds explicit", () => {
  const original = solid(4, 4, 0);
  const production = solid(4, 4, 0);
  production.data.set([255, 255, 255, 255], 0);

  assert.deepEqual(validateSurfaceGeometry(original, production, 4, 4), {
    width: 4,
    height: 4,
  });
  const [result] = compareRegions(original, production, [
    { name: "all", x: 0, y: 0, width: 4, height: 4, maximum_ratio: 0.1 },
  ]);
  assert.equal(result.different_pixels, 1);
  assert.equal(result.ratio, 1 / 16);
  assert.equal(result.status, "pass");
});

test("A0 cells map to normal-route publication artifacts", () => {
  assert.equal(
    productionScreenshotPath("/run", "alliance", "systems"),
    "/run/alliance/02-category-70.png",
  );
  assert.equal(
    productionScreenshotPath("/run", "empire", "topic text"),
    "/run/empire/03-topic.png",
  );
  assert.equal(
    productionScreenshotPath("/run", "alliance", "next"),
    "/run/alliance/05-last-ship-endpoint.png",
  );
});

function validCrosswalk() {
  const requirements = [
    "index",
    "systems",
    "ships",
    "facilities",
    "missions",
    "troops",
    "personnel",
    "topic text",
    "topic art",
    "previous",
    "next",
    "context open",
    "missing entry",
  ];
  return {
    kind: "open-rebellion-adapted-obj01-a0-crosswalk",
    cells: requirements.map((requirement, index) => ({
      id: `OBJ-01-C${String(index + 1).padStart(3, "0")}`,
      requirement,
      disposition: index === 12
        ? "strict_a0_not_applicable"
        : "a0_available_endpoint",
      captures: index === 12
        ? []
        : [{ path: `E51/cell-${index + 1}.client.png`, sha256: "0".repeat(64) }],
    })),
  };
}

test("A0 crosswalk validation rejects denominator drift and unsafe paths", () => {
  assert.equal(validateCrosswalk(validCrosswalk()).cells.length, 13);

  const duplicate = validCrosswalk();
  duplicate.cells[1].id = duplicate.cells[0].id;
  assert.throws(() => validateCrosswalk(duplicate), /wrong stable ID/);

  const traversal = validCrosswalk();
  traversal.cells[0].captures[0].path = "E51/../outside.png";
  assert.throws(() => validateCrosswalk(traversal), /not normalized|escapes/);
});
