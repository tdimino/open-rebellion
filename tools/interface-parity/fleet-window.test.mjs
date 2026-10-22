import assert from "node:assert/strict";
import test from "node:test";

test("Fleet clicks keep press and release in one manager-safe input batch", async () => {
  const { clickWithRouteBarrier } = await import("./fleet-window.mjs");
  const routes = [
    "[windowing-route] physical=7:0 kind=pointer router=root application=none underlay=0",
  ];
  const calls = [];
  let frame = 0;
  const page = {
    mouse: {
      async move(x, y) { calls.push(["move", frame, x, y]); },
      async down(options) { calls.push(["down", frame, options]); },
      async up(options) { calls.push(["up", frame, options]); },
    },
    async evaluate(_callback, argument) {
      calls.push(["batch", frame, argument]);
    },
  };
  const advanceFrame = async () => {
    frame += 1;
    if (frame === 1) {
      routes.push("[windowing-route] physical=11:0 kind=pointer router=window:4 application=pointer_ready underlay=0");
    } else if (frame === 2) {
      routes.push("[windowing-route] physical=13:0 kind=pointer router=window:4 application=fleet_window underlay=0");
      routes.push("[windowing-route] physical=13:1 kind=pointer router=window:4 application=fleet_window underlay=0");
    }
  };

  const receipts = await clickWithRouteBarrier({
    page,
    routes,
    point: { x: 104, y: 221 },
    label: "Fleet entry",
    expectedDown: { application: "fleet_window" },
    expectedRelease: { application: "fleet_window" },
    advanceFrame,
    settleFrames: 0,
  });

  assert.deepEqual(calls, [
    ["move", 0, 104, 221],
    ["batch", 1, { x: 104, y: 221, button: "left" }],
  ]);
  assert.deepEqual(
    [receipts.move.physical, receipts.down.physical, receipts.release.physical],
    ["11:0", "13:0", "13:1"],
  );
});

test("a delayed prior receipt cannot satisfy the next Fleet pointer phase", async () => {
  const { clickWithRouteBarrier } = await import("./fleet-window.mjs");
  const routes = [];
  const cursor = { physical: null };
  const calls = [];
  let frame = 0;
  const page = {
    mouse: {
      async move() { calls.push("move"); },
      async down() { calls.push("down"); },
      async up() { calls.push("up"); },
    },
    async evaluate() { calls.push("batch"); },
  };
  const advanceFrame = async () => {
    frame += 1;
    if (frame === 1) {
      routes.push("[windowing-route] physical=41:0 kind=pointer router=window:4 application=pointer_ready underlay=0");
    } else if (frame === 2) {
      // Console delivery from the already-consumed hover arrives after the
      // atomic click. It cannot satisfy either action receipt.
      routes.push("[windowing-route] physical=41:0 kind=pointer router=window:4 application=fleet_window underlay=0");
    } else if (frame === 3) {
      routes.push("[windowing-route] physical=43:0 kind=pointer router=window:4 application=fleet_window underlay=0");
      routes.push("[windowing-route] physical=43:1 kind=pointer router=window:4 application=fleet_window underlay=0");
    }
  };

  const receipts = await clickWithRouteBarrier({
    page,
    routes,
    cursor,
    point: { x: 104, y: 221 },
    label: "Fleet entry",
    expectedDown: { application: "fleet_window" },
    expectedRelease: { application: "fleet_window" },
    advanceFrame,
    settleFrames: 0,
  });

  assert.deepEqual(calls, ["move", "batch"]);
  assert.deepEqual(
    [receipts.move.physical, receipts.down.physical, receipts.release.physical],
    ["41:0", "43:0", "43:1"],
  );
  assert.equal(cursor.physical, "43:1");
});

test("a context click keeps press and release in one browser input batch", async () => {
  const { clickWithRouteBarrier } = await import("./fleet-window.mjs");
  const routes = [];
  const cursor = { physical: null };
  const calls = [];
  let frame = 0;
  const page = {
    mouse: {
      async move() { calls.push("move"); },
      async down() { calls.push("split-down"); },
      async up() { calls.push("split-up"); },
    },
    async evaluate(_callback, argument) {
      calls.push(["batch", argument]);
    },
  };
  const advanceFrame = async () => {
    frame += 1;
    if (frame === 1) {
      routes.push("[windowing-route] physical=31:0 kind=pointer router=window:1 application=pointer_ready underlay=0");
    } else if (frame === 2) {
      // Application delivery may precede the earlier physical event on the console.
      routes.push("[windowing-route] physical=35:1 kind=pointer router=window:1 application=defenses_window underlay=0");
      routes.push("[windowing-route] physical=35:0 kind=pointer router=window:1 application=none underlay=0");
    }
  };

  const receipts = await clickWithRouteBarrier({
    page,
    routes,
    cursor,
    point: { x: 112, y: 249 },
    label: "Defenses context menu",
    button: "right",
    batchActions: true,
    expectedDown: { application: "none" },
    expectedRelease: { application: "defenses_window" },
    advanceFrame,
    settleFrames: 0,
  });

  assert.deepEqual(calls, ["move", ["batch", { x: 112, y: 249, button: "right" }]]);
  assert.deepEqual([receipts.down.physical, receipts.release.physical], ["35:0", "35:1"]);
  assert.equal(cursor.physical, "35:1");
});

test("a click can settle exact-point movement without requiring a move receipt", async () => {
  const { clickWithRouteBarrier } = await import("./fleet-window.mjs");
  const routes = [
    "[windowing-route] physical=37:4 kind=pointer router=window:1 application=none underlay=0",
  ];
  const cursor = { physical: null };
  const calls = [];
  let frame = 0;
  const page = {
    mouse: {
      async move(x, y) { calls.push(["move", frame, x, y]); },
      async down() { calls.push(["split-down", frame]); },
      async up() { calls.push(["split-up", frame]); },
    },
    async evaluate(_callback, argument) {
      calls.push(["batch", frame, argument]);
    },
  };
  const advanceFrame = async () => {
    frame += 1;
    calls.push(["frame", frame]);
    if (frame === 2) {
      routes.push("[windowing-route] physical=39:0 kind=pointer router=window:1 application=none underlay=0");
      routes.push("[windowing-route] physical=39:1 kind=pointer router=window:1 application=defenses_window underlay=0");
    }
  };

  const receipts = await clickWithRouteBarrier({
    page,
    routes,
    cursor,
    point: { x: 112, y: 249 },
    label: "modal object-menu opener",
    button: "right",
    skipMoveReceipt: true,
    batchActions: true,
    expectedDown: { application: "none" },
    expectedRelease: { application: "defenses_window" },
    advanceFrame,
    settleFrames: 0,
  });

  assert.deepEqual(calls, [
    ["move", 0, 112, 249],
    ["frame", 1],
    ["batch", 1, { x: 112, y: 249, button: "right" }],
    ["frame", 2],
  ]);
  assert.equal(receipts.move.skipped, true);
  assert.deepEqual(
    [receipts.down.physical, receipts.release.physical],
    ["39:0", "39:1"],
  );
  assert.equal(cursor.physical, "39:1");
});

test("a managed menu choice consumes its ordered atomic press and release", async () => {
  const { clickWithRouteBarrier } = await import("./fleet-window.mjs");
  const routes = [
    "[windowing-route] physical=35:1 kind=pointer router=window:1 application=defenses_window underlay=0",
  ];
  const cursor = { physical: "35:1" };
  const calls = [];
  let frame = 0;
  const page = {
    mouse: {
      async move() { calls.push("move"); },
      async down() { calls.push("split-down"); },
      async up() { calls.push("split-up"); },
    },
    async evaluate(_callback, argument) {
      calls.push(["batch", argument]);
    },
  };
  const advanceFrame = async () => {
    frame += 1;
    if (frame === 1) {
      routes.push("[windowing-route] physical=51:0 kind=pointer router=window:3 application=pointer_ready underlay=0");
    } else if (frame === 2) {
      routes.push("[windowing-route] physical=54:1 kind=pointer router=window:3 application=none underlay=0");
      routes.push("[windowing-route] physical=54:0 kind=pointer router=window:3 application=none underlay=0");
    }
  };

  const receipts = await clickWithRouteBarrier({
    page,
    routes,
    cursor,
    point: { x: 211, y: 187 },
    label: "managed menu Move choice",
    batchActions: true,
    expectedDown: { application: "none" },
    expectedRelease: { application: "none" },
    advanceFrame,
    settleFrames: 0,
  });

  assert.deepEqual(calls, ["move", ["batch", { x: 211, y: 187, button: "left" }]]);
  assert.deepEqual([receipts.down.physical, receipts.release.physical], ["54:0", "54:1"]);
  assert.equal(cursor.physical, "54:1");
});

test("Fleet drags observe routed capture motion and release before settling", async () => {
  const { dragWithRouteBarrier } = await import("./fleet-window.mjs");
  const routes = [];
  const calls = [];
  let observedCalls = 0;
  let physical = 20;
  let settled = 0;
  const page = {
    mouse: {
      async move(x, y, options) { calls.push(["move", x, y, options]); },
      async down(options) { calls.push(["down", options]); },
      async up(options) { calls.push(["up", options]); },
    },
  };
  const advanceFrame = async () => {
    if (observedCalls === calls.length) {
      settled += 1;
      return;
    }
    observedCalls = calls.length;
    physical += 1;
    const phase = calls.at(-1)[0];
    if (phase === "move" && calls.length === 3) {
      for (let step = 0; step < 4; step += 1) {
        routes.push(`[windowing-route] physical=${physical + step}:0 kind=pointer router=window:9 application=pointer_ready underlay=0`);
      }
      physical += 3;
    } else {
      const application = phase === "move" ? "pointer_ready" : "fleet_window";
      routes.push(`[windowing-route] physical=${physical}:0 kind=pointer router=window:9 application=${application} underlay=0`);
    }
  };

  const receipts = await dragWithRouteBarrier({
    page,
    routes,
    from: { x: 120, y: 160 },
    to: { x: 420, y: 260 },
    label: "Fleet cargo",
    advanceFrame,
  });

  assert.deepEqual(calls.map(([kind]) => kind), ["move", "down", "move", "up"]);
  assert.deepEqual(
    [receipts.move.physical, receipts.down.physical, receipts.drag.physical, receipts.release.physical],
    ["21:0", "22:0", "23:0", "27:0"],
  );
  assert.equal(settled, 3, "the completed drag did not cross the manager publication boundary");
});

test("managed fixture surfaces wait until their manager admits input", async () => {
  const { waitForFixtureInputReady } = await import("./fleet-window.mjs");
  const previousWindow = globalThis.window;
  const observed = [];
  globalThis.window = {
    __openRebellionInterfaceObjectMenu: { status: "object-menu", input_ready: false },
  };
  const page = {
    async waitForFunction(predicate, property) {
      observed.push(predicate(property));
      globalThis.window[property].input_ready = true;
      observed.push(predicate(property));
    },
    async evaluate(_callback, property) { return globalThis.window[property]; },
  };
  try {
    const report = await waitForFixtureInputReady(
      page,
      "__openRebellionInterfaceObjectMenu",
      "the object menu becomes input-ready",
    );
    assert.deepEqual(observed, [false, true]);
    assert.equal(report.input_ready, true);
  } finally {
    globalThis.window = previousWindow;
  }
});
