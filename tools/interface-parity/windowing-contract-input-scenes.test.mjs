import assert from "node:assert/strict";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

import { discoverCaseModules } from "./windowing-contract.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));

test("windowing_journey_de_checks_child_consumption_and_gesture_cancellation", async () => {
  const cases = await discoverCaseModules(here);
  const journey = cases.find(({ id }) => id === "journey-d");
  assert.ok(journey, "the discovery interface omitted journey-d");
  const module = await import("./windowing-contract-input-scenes.mjs");
  assert.deepEqual(journey.groups, ["D"]);
  assert.deepEqual(journey.steps, Array.from(
    { length: 10 },
    (_, index) => `D${String(index + 1).padStart(2, "0")}`,
  ));

  const stable = {
    registered: [7, 9],
    record_count: 2,
    focused: { window: 7, control: 101 },
    root_actions: 4,
    underlay_actions: 0,
  };
  const assertions = module.assertChildConsumptionAndGestureCancellation({
    search: {
      before: stable,
      after: { ...stable, focused: { window: 7, control: 102 } },
      kind: "key",
      recipient: "window:7",
    },
    renameCommit: {
      before: stable,
      after: { ...stable, rename_open: false, rename_commits: 1 },
      window: 7,
      kind: "key",
      recipient: "window:7",
    },
    renameCancel: {
      before: { ...stable, rename_commits: 1, rename_open: true },
      after: { ...stable, rename_commits: 1, rename_open: false },
      window: 7,
      kind: "key",
      recipient: "window:7",
    },
    detailReturn: {
      raised_id: 9,
      returned_id: 7,
      before: { id: 7, selection: { tab: "ships", entry: 0 } },
      after: { selection: { tab: "ships", entry: 0 }, registered: [7, 9] },
    },
    menu: {
      id: 12,
      dismissed_id: 13,
      owner: "root",
      expected_owner: "root",
      rects: [[400, 200, 100, 100], [200, 200, 100, 100]],
      hit_recipients: [12, 12],
      outside_recipient: "blocked",
      dismissed_registered_after: false,
      root_actions_before: 4,
      root_actions_after: 4,
    },
    menuReplacement: {
      old_id: 14,
      new_id: 15,
      old_owner: 7,
      new_owner: 7,
      expected_owner: 7,
      injected: true,
      old_registered_after_replace: false,
      new_registered_after_replace: true,
      child_registered_after_dismiss: false,
      owner_registered_after_close: false,
      expected_focus: 9,
      focus_after: { window: 9, control: null },
    },
    capturedRelease: {
      owner: 7,
      release_recipient: "window:7",
      capture_after: null,
      payload_after: false,
      drop_delta: 0,
      underlay_delta: 0,
    },
    interruptions: ["modal", "close", "minimize", "scene", "focus-loss"].map((kind) => ({
      kind,
      injected: kind !== "modal",
      captured_owner_id: 7,
      capture_before: 7,
      capture_after: null,
      payload_after: false,
      release_recipient: "blocked",
      drop_delta: 0,
      underlay_delta: 0,
    })),
  });
  assert.deepEqual(Object.keys(assertions), journey.steps);

  assert.throws(() => module.assertChildConsumptionAndGestureCancellation({
    search: {
      before: stable,
      after: { ...stable, root_actions: 5 },
      kind: "key",
      recipient: "root",
    },
  }), /search|root|recipient/i);
});

test("windowing_journey_de_checks_fail_closed_scene_and_reset", async () => {
  const cases = await discoverCaseModules(here);
  const journey = cases.find(({ id }) => id === "journey-e");
  assert.ok(journey, "the discovery interface omitted journey-e");
  const module = await import("./windowing-contract-input-scenes.mjs");
  assert.deepEqual(journey.groups, ["E"]);
  assert.deepEqual(journey.steps, Array.from(
    { length: 10 },
    (_, index) => `E${String(index + 1).padStart(2, "0")}`,
  ));

  const assertions = module.assertFailClosedSceneAndReset({
    unpaintableModal: {
      id: 21,
      registered: [7, 21],
      active_modal: 21,
      blocking_unpainted: [21],
      recovery_target: 21,
      forced_status: "unpaintable",
      outside_recipient: "blocked",
    },
    unpaintableModeless: { id: 7, registered: [7], removed: [] },
    absentModeless: { id: 7, registered: [], removed: [7], removal_count: 1 },
    sectorClose: {
      id: 33,
      down: { recipient: "window:33", application: "none", physical: "401:0" },
      release: { recipient: "window:33", application: "sector_window", physical: "403:0" },
      after_id: null,
    },
    tactical: {
      dormant_galaxy_modal: 21,
      dormant_galaxy_registered: true,
      combat_recipient: "tactical",
      combat_action: "toggle_pause",
      paused_before: false,
      paused_after: true,
      alert_modal: 31,
      alert_outside_recipient: "blocked",
      results_modal: 32,
      results_outside_recipient: "blocked",
      return_release_recipient: "blocked",
      dismissal_underlay_delta: 0,
    },
    reset: {
      queued_target: 21,
      old_ids: [7, 21, 32],
      new_ids: [33],
      stale_action_discarded: true,
      registered_after_apply: [33],
    },
    geometry: [
      { phase: "cold", id: 33, painted: [20, 30, 200, 140], content_status: "ready", hit_recipient: "window:33" },
      { phase: "warm", id: 34, painted: [40, 50, 300, 180], content_status: "ready", hit_recipient: "window:34" },
      { phase: "restored", id: 35, painted: [20, 30, 200, 140], content_status: "ready", hit_recipient: "window:35" },
    ],
  });
  assert.deepEqual(Object.keys(assertions), journey.steps);

  assert.throws(() => module.assertFailClosedSceneAndReset({
    unpaintableModal: {
      id: 21,
      registered: [],
      active_modal: null,
      blocking_unpainted: [],
      recovery_target: null,
      forced_status: "unpaintable",
      outside_recipient: "root",
    },
  }), /unpaintable|modal|block/i);
});

test("pointer release waits for a fresh down route receipt", async () => {
  const module = await import("./windowing-contract-input-scenes.mjs");
  const routes = [];
  const calls = [];
  const downWait = deferred();
  const allowDownReceipt = deferred();
  const releaseWait = deferred();
  const allowReleaseReceipt = deferred();
  let frame = 0;
  let completed = false;
  const page = {
    mouse: {
      async move(x, y) { calls.push(["move", x, y]); },
      async down(options) { calls.push(["down", options]); },
      async up(options) { calls.push(["up", options]); },
    },
  };
  const advanceFrame = async () => {
    frame += 1;
    if (frame === 2) {
      downWait.resolve();
      await allowDownReceipt.promise;
    } else if (frame === 3) {
      releaseWait.resolve();
      await allowReleaseReceipt.promise;
    }
  };

  const pressing = module.pressWithRouteReceipts({
    page,
    routes,
    point: { x: 1090, y: 95 },
    label: "owner close",
    expectedDown: { recipient: "window:3", application: "none" },
    expectedRelease: { recipient: "window:3", application: "fleet_window" },
    advanceFrame,
  }).then((receipts) => {
    completed = true;
    return receipts;
  });

  await downWait.promise;
  assert.deepEqual(calls.map(([kind]) => kind), ["move", "down"]);
  assert.equal(completed, false, "requestAnimationFrame released the press without a route receipt");
  routes.push("[windowing-route] physical=309:0 kind=pointer router=window:3 application=none underlay=0");
  allowDownReceipt.resolve();

  await releaseWait.promise;
  assert.deepEqual(calls.map(([kind]) => kind), ["move", "down", "up"]);
  assert.equal(completed, false, "the press completed without a distinct release receipt");
  routes.push("[windowing-route] physical=311:0 kind=pointer router=window:3 application=fleet_window underlay=0");
  allowReleaseReceipt.resolve();

  const receipts = await pressing;
  assert.equal(completed, true);
  assert.equal(receipts.down.physical, "309:0");
  assert.equal(receipts.release.physical, "311:0");
});

test("Fleet context-menu press is atomic and waits for its owner route receipts", async () => {
  const module = await import("./windowing-contract-input-scenes.mjs");
  const routes = [
    "[windowing-route] physical=101:0 kind=pointer router=window:7 application=pointer_ready underlay=0",
  ];
  const calls = [];
  let frame = 0;
  let batched = false;
  const page = {
    mouse: {
      async move(x, y) { calls.push(["move", x, y]); },
      async down({ button }) { calls.push(["down", button]); },
      async up({ button }) { calls.push(["up", button]); },
    },
    async evaluate(_callback, argument) {
      if (argument && typeof argument === "object" && "button" in argument) {
        batched = true;
        calls.push(["batch", argument]);
      }
    },
  };
  const advanceFrame = async () => {
    frame += 1;
    if (batched && frame === 2) {
      // Console delivery is allowed to publish the same-frame pair out of order.
      routes.push("[windowing-route] physical=108:1 kind=pointer router=window:7 application=fleet_window underlay=0");
      routes.push("[windowing-route] physical=108:0 kind=pointer router=window:7 application=none underlay=0");
    } else if (!batched && frame === 2) {
      routes.push("[windowing-route] physical=108:0 kind=pointer router=window:7 application=none underlay=0");
    } else if (!batched && frame === 3) {
      routes.push("[windowing-route] physical=109:0 kind=pointer router=window:7 application=fleet_window underlay=0");
    }
  };

  const receipts = await module.pressWithRouteReceipts({
    page,
    routes,
    point: { x: 421, y: 166 },
    label: "Fleet object menu",
    button: "right",
    batchActions: true,
    skipMoveReceipt: true,
    expectedDown: { recipient: "window:7", application: "none" },
    expectedRelease: { recipient: "window:7", application: "fleet_window" },
    advanceFrame,
    settleFrames: 0,
  });

  assert.deepEqual(calls, [
    ["move", 421, 166],
    ["batch", { x: 421, y: 166, button: "right" }],
  ]);
  assert.deepEqual([receipts.down.physical, receipts.release.physical], ["108:0", "108:1"]);
});

test("Sector close after modeless absence waits for fresh owner route receipts", async () => {
  const module = await import("./windowing-contract-input-scenes.mjs");
  const routes = [
    "[windowing-route] physical=390:0 kind=pointer router=window:3 application=fleet_window underlay=0",
  ];
  const calls = [];
  let frame = 0;
  let upBeforeDownReceipt = false;
  let closureObserved = false;
  const page = {
    mouse: {
      async move(x, y) { calls.push(["move", x, y]); },
      async down({ button }) { calls.push(["down", button]); },
      async up({ button }) { calls.push(["up", button]); },
    },
  };
  const advanceFrame = async () => {
    frame += 1;
    if (frame === 2) {
      upBeforeDownReceipt = calls.some(([kind]) => kind === "up");
      routes.push("[windowing-route] physical=401:0 kind=pointer router=window:33 application=none underlay=0");
    } else if (frame === 3) {
      routes.push("[windowing-route] physical=403:0 kind=pointer router=window:33 application=sector_window underlay=0");
    }
  };

  const closed = await module.closeSectorAfterModelessAbsence({
    page,
    routes,
    sectorId: 33,
    point: { x: 1090, y: 81 },
    advanceFrame,
    waitForClosed: async () => {
      closureObserved = true;
      assert.equal(calls.at(-1)?.[0], "up", "Sector closure was awaited before pointer release");
      assert.ok(routes.some((line) => line.includes("physical=403:0")
        && line.includes("application=sector_window")), "Sector closure was awaited before its release receipt");
      return null;
    },
  });

  assert.equal(closureObserved, true);
  assert.equal(upBeforeDownReceipt, false, "Sector release occurred before its fresh down route receipt");
  assert.deepEqual(calls, [["move", 1090, 81], ["down", "left"], ["up", "left"]]);
  assert.equal(closed.id, 33);
  assert.equal(closed.down.recipient, "window:33");
  assert.equal(closed.down.application, "none");
  assert.equal(closed.release.recipient, "window:33");
  assert.equal(closed.release.application, "sector_window");
  assert.notEqual(closed.down.physical, closed.release.physical);
  assert.equal(closed.after_id, null);
});

function deferred() {
  let resolve;
  const promise = new Promise((finish) => { resolve = finish; });
  return { promise, resolve };
}
