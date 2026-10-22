import assert from "node:assert/strict";
import test from "node:test";

test("capital-ship Status selection accepts the Fleet batch down/release application pair", async () => {
  const { clickStatusWithRouteBarrier } = await import("./status-window-input.mjs");
  const routes = [];
  const calls = [];
  let frame = 0;
  const page = {
    mouse: {
      async move(x, y) { calls.push(["move", x, y]); },
      async down(options) { calls.push(["down", options]); },
      async up(options) { calls.push(["up", options]); },
    },
    async evaluate(callback, argument) {
      calls.push(["batch", argument]);
      assert.equal(typeof callback, "function");
    },
  };
  const advanceFrame = async () => {
    frame += 1;
    if (frame === 1) {
      routes.push("[windowing-route] physical=37:0 kind=pointer router=window:4 application=pointer_ready underlay=0");
    } else if (frame === 2) {
      routes.push("[windowing-route] physical=40:0 kind=pointer router=window:4 application=none underlay=0");
      routes.push("[windowing-route] physical=40:1 kind=pointer router=window:4 application=fleet_window underlay=0");
    }
  };

  const receipts = await clickStatusWithRouteBarrier({
    page,
    routes,
    point: { x: 290, y: 230 },
    label: "capital ship Fleet selection",
    expectedDown: { application: "none" },
    expectedRelease: { application: "fleet_window" },
    advanceFrame,
    settleFrames: 0,
  });

  assert.deepEqual(calls, [
    ["move", 290, 230],
    ["batch", { x: 290, y: 230, button: "left" }],
  ]);
  assert.deepEqual(
    [receipts.down.physical, receipts.release.physical],
    ["40:0", "40:1"],
  );
});

test("ordinary Status actions use frame-separated input without requiring route receipts", async () => {
  const { clickStatusDirect } = await import("./status-window-input.mjs");
  const events = [];
  let frame = 0;
  const page = {
    mouse: {
      async move(x, y) { events.push(["move", x, y, frame]); },
      async down({ button }) { events.push(["down", button, frame]); },
      async up({ button }) { events.push(["up", button, frame]); },
    },
  };
  const advanceFrame = async () => { frame += 1; };

  await clickStatusDirect({
    page,
    point: { x: 340, y: 233 },
    advanceFrame,
    settleFrames: 0,
  });

  assert.deepEqual(events, [
    ["move", 340, 233, 0],
    ["down", "left", 1],
    ["up", "left", 2],
  ]);
});

test("Status object-menu context clicks keep press and release on separate frames", async () => {
  const { clickStatusDirect } = await import("./status-window-input.mjs");
  const events = [];
  let frame = 0;
  const page = {
    mouse: {
      async move(x, y) { events.push(["move", x, y, frame]); },
      async down({ button }) { events.push(["down", button, frame]); },
      async up({ button }) { events.push(["up", button, frame]); },
    },
  };

  await clickStatusDirect({
    page,
    point: { x: 160, y: 120 },
    button: "right",
    advanceFrame: async () => { frame += 1; },
    settleFrames: 0,
  });

  assert.deepEqual(events, [
    ["move", 160, 120, 0],
    ["down", "right", 1],
    ["up", "right", 2],
  ]);
});

test("Status console capture keeps diagnostics out of the live route receipt stream", async () => {
  const { appendStatusConsoleLine, clickStatusWithRouteBarrier } = await import("./status-window-input.mjs");
  const consoleLines = [];
  const routes = [];
  const calls = [];
  let frame = 0;
  const page = {
    mouse: {
      async move(x, y) { calls.push(["move", x, y]); },
      async down(options) { calls.push(["down", options]); },
      async up(options) { calls.push(["up", options]); },
    },
    async evaluate(_callback, argument) { calls.push(["batch", argument]); },
  };
  const append = (line) => appendStatusConsoleLine({ consoleLines, routes, line });
  const advanceFrame = async () => {
    frame += 1;
    if (frame === 1) {
      append("Plugin open_rebellion_replay is present in JS bundle, but is not used in the rust code.");
      append("[windowing-route] physical=37:0 kind=pointer router=window:4 application=pointer_ready underlay=0");
    } else if (frame === 2) {
      append("[windowing-route] physical=40:0 kind=pointer router=window:4 application=none underlay=0");
      append("[windowing-route] physical=40:1 kind=pointer router=window:4 application=fleet_window underlay=0");
    }
  };

  const receipts = await clickStatusWithRouteBarrier({
    page,
    routes,
    point: { x: 290, y: 230 },
    label: "capital ship Fleet selection",
    expectedDown: { application: "none" },
    expectedRelease: { application: "fleet_window" },
    advanceFrame,
    settleFrames: 0,
  });

  assert.equal(consoleLines[0].startsWith("Plugin open_rebellion_replay"), true);
  assert.equal(routes.length, 3);
  assert.deepEqual(
    [receipts.move.physical, receipts.down.physical, receipts.release.physical],
    ["37:0", "40:0", "40:1"],
  );
  assert.deepEqual(calls, [
    ["move", 290, 230],
    ["batch", { x: 290, y: 230, button: "left" }],
  ]);
});
