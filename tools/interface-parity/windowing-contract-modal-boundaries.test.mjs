import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

import {
  AVAILABLE_MODAL_CASES,
  assertCloseFrameAndOrphanRelease,
  alternateViewport,
  detailTitleButtonPoint,
  projectNativePoint,
  windowingContractCases,
} from "./windowing-contract-modal-boundaries.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");

test("windowing_journey_c_requires_every_available_modal", () => {
  const expected = [
    "fleet-finder",
    "troop-finder",
    "personnel-finder",
    "message-index",
    "encyclopedia",
    "status",
    "build-selection",
    "mission-dialog",
    "move-confirmation",
    "scrap-confirmation",
  ];

  assert.deepEqual(AVAILABLE_MODAL_CASES.map(({ kind }) => kind), expected);
  assert.ok(AVAILABLE_MODAL_CASES.every(({ entry }) => entry !== "fixture-open"));
  for (const { duplicatePolicy } of AVAILABLE_MODAL_CASES) {
    assert.ok(duplicatePolicy.source.length > 0);
    assert.ok(duplicatePolicy.test.length > 0);
    const ownerSource = fs.readFileSync(path.join(root, duplicatePolicy.source), "utf8");
    assert.match(ownerSource, new RegExp(`fn\\s+${duplicatePolicy.test}\\s*\\(`));
  }
  assert.ok(!AVAILABLE_MODAL_CASES.some(({ kind }) => kind === "system-finder"));
  assert.ok(windowingContractCases.some(({ id }) => id === "journey-c"));
  assert.ok(windowingContractCases.some(({ id }) => id === "modal-boundaries"));
  for (const kind of expected) {
    assert.ok(
      windowingContractCases.some(({ id }) => id === `modal-boundary-${kind}`),
      `${kind} has no independently selectable browser case`,
    );
  }
});

test("windowing_journey_c_exercises_a_real_resize_round_trip", () => {
  assert.deepEqual(alternateViewport({ width: 640, height: 480 }), { width: 1280, height: 800 });
  assert.deepEqual(alternateViewport({ width: 1280, height: 800 }), { width: 640, height: 480 });
  assert.deepEqual(
    projectNativePoint({ width: 640, height: 480 }, { x: 452, y: 149.5 }),
    { x: 452, y: 149.5 },
  );
  assert.deepEqual(
    projectNativePoint({ width: 1280, height: 800 }, { x: 452, y: 149.5 }),
    { x: 860, y: 249.16666666666669 },
  );
});

test("windowing_journey_c_uses_the_live_detail_title_button_geometry", () => {
  const rect = [100, 40, 235, 304];
  assert.deepEqual(detailTitleButtonPoint(rect, 1.5, "system", "minimize"), {
    x: 403,
    y: 53.5,
  });
  assert.deepEqual(detailTitleButtonPoint(rect, 1.5, "fleet", "minimize"), {
    x: 416.5,
    y: 53.5,
  });
  assert.deepEqual(detailTitleButtonPoint(rect, 1.5, "defenses", "close"), {
    x: 437.5,
    y: 53.5,
  });
});

test("windowing_journey_c_checks_close_frame_and_orphan_release", () => {
  const underlay = {
    selected_system: 101,
    history: [7, 9],
    paint_order: [7, 9, 11],
    action_counters: { underlay: 4 },
  };
  const result = assertCloseFrameAndOrphanRelease({
    modalWindow: 11,
    before: underlay,
    closeFrame: underlay,
    orphanRelease: underlay,
    freshRoot: {
      ...underlay,
      action_counters: { underlay: 5 },
    },
    routes: {
      close: { recipient: "window:11", underlay_action_count: 0 },
      orphanRelease: { recipient: "blocked:modal-transition", underlay_action_count: 0 },
      freshRoot: { recipient: "root", application_recipient: "fleet_finder", underlay_action_count: 0 },
    },
  });
  assert.equal(result, 13);

  assert.throws(() => assertCloseFrameAndOrphanRelease({
    modalWindow: 11,
    before: underlay,
    closeFrame: { ...underlay, selected_system: 202 },
    orphanRelease: underlay,
    freshRoot: { ...underlay, action_counters: { underlay: 5 } },
    routes: {
      close: { recipient: "window:11", underlay_action_count: 1 },
      orphanRelease: { recipient: "root", underlay_action_count: 1 },
      freshRoot: { recipient: "root", application_recipient: "fleet_finder", underlay_action_count: 0 },
    },
  }), /close frame|underlay/i);
});
