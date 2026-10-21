import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import test from "node:test";

test("the sector quadrant runner is inert when imported", () => {
  const runner = new URL("./sector-quadrants.mjs", import.meta.url).href;
  const imported = spawnSync(
    process.execPath,
    [
      "--input-type=module",
      "--eval",
      `process.argv.push("--no-build", "--only=none"); await import(${JSON.stringify(runner)});`,
    ],
    {
      encoding: "utf8",
      env: { ...process.env, OPEN_REBELLION_CHROME_FOR_TESTING: "/bin/false" },
    },
  );

  assert.equal(imported.status, 0, imported.stderr);
  assert.equal(imported.stdout, "");
});

test("sector pointer release waits for a fresh post-down route receipt", async () => {
  const { pressWithRouteReceipts } = await import("./sector-quadrants.mjs");
  const routes = [];
  const calls = [];
  const moveReceiptWait = deferred();
  const allowMoveReceipt = deferred();
  const downWait = deferred();
  const allowDownReceipt = deferred();
  const rootReleaseWait = deferred();
  const allowRootReleaseReceipt = deferred();
  const managedReleaseWait = deferred();
  const allowManagedReleaseReceipt = deferred();
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
      moveReceiptWait.resolve();
      await allowMoveReceipt.promise;
    } else if (frame === 3) {
      downWait.resolve();
      await allowDownReceipt.promise;
    } else if (frame === 4) {
      rootReleaseWait.resolve();
      await allowRootReleaseReceipt.promise;
    } else if (frame === 5) {
      managedReleaseWait.resolve();
      await allowManagedReleaseReceipt.promise;
    }
  };

  const pressing = pressWithRouteReceipts({
    page,
    routes,
    point: { x: 104, y: 221 },
    label: "sector icon",
    expectedMove: { recipient: "window:7", application: "pointer_ready" },
    expectedDown: { recipient: "window:7", application: "sector_window" },
    expectedRelease: { recipient: "window:7", application: "sector_window" },
    advanceFrame,
  }).then((receipts) => {
    completed = true;
    return receipts;
  });

  await moveReceiptWait.promise;
  assert.deepEqual(calls.map(([kind]) => kind), ["move"]);
  assert.equal(completed, false, "the press began before the move receipt");
  routes.push("[windowing-route] physical=29:0 kind=pointer router=window:7 application=pointer_ready underlay=0");
  allowMoveReceipt.resolve();

  await downWait.promise;
  assert.deepEqual(calls.map(([kind]) => kind), ["move", "down"]);
  assert.equal(completed, false, "requestAnimationFrame released the press without a route receipt");
  routes.push("[windowing-route] physical=31:0 kind=pointer router=window:7 application=sector_window underlay=0");
  allowDownReceipt.resolve();

  await rootReleaseWait.promise;
  assert.deepEqual(calls.map(([kind]) => kind), ["move", "down", "up"]);
  assert.equal(completed, false, "the press completed without a fresh release receipt");
  routes.push("[windowing-route] physical=33:0 kind=pointer router=root application=sector_window underlay=0");
  allowRootReleaseReceipt.resolve();

  await managedReleaseWait.promise;
  assert.equal(completed, false, "a root-routed receipt satisfied the managed Sector release");
  routes.push("[windowing-route] physical=35:0 kind=pointer router=window:7 application=sector_window underlay=0");
  allowManagedReleaseReceipt.resolve();

  const receipts = await pressing;
  assert.equal(completed, true);
  assert.equal(receipts.down.physical, "31:0");
  assert.equal(receipts.release.physical, "35:0");
  assert.deepEqual(
    routes.map((line) => line.match(/router=([^ ]+)/)[1]),
    ["window:7", "window:7", "root", "window:7"],
  );
});

test("click routes Sector capture presses and non-capture window presses", async () => {
  const { click } = await import("./sector-quadrants.mjs");
  for (const [application, downApplication] of [
    ["sector_window", "sector_window"],
    ["defenses_window", "none"],
  ]) {
    const routes = [];
    const calls = [];
    let observedCalls = 0;
    let physical = 50;
    const page = {
      mouse: {
        async move(x, y) { calls.push(["move", x, y]); },
        async down(options) { calls.push(["down", options]); },
        async up(options) { calls.push(["up", options]); },
      },
    };
    const advanceFrame = async () => {
      if (observedCalls === calls.length) return;
      observedCalls = calls.length;
      physical += 1;
      const kind = calls.at(-1)[0];
      const routed = kind === "move" ? "pointer_ready"
        : kind === "down" ? downApplication : application;
      routes.push(`[windowing-route] physical=${physical}:0 kind=pointer router=root application=${routed} underlay=0`);
    };

    const receipts = await click(
      page,
      routes,
      { x: 104, y: 221 },
      application,
      { advanceFrame },
    );

    assert.deepEqual(calls.map(([kind]) => kind), ["move", "down", "up"]);
    assert.equal(receipts.down.application, downApplication);
    assert.equal(receipts.release.application, application);
    assert.notEqual(receipts.down.physical, receipts.release.physical);
  }
});

test("sector double click reuses one ready pointer without inter-click settling", async () => {
  const { doubleClick } = await import("./sector-quadrants.mjs");
  const routes = [];
  const calls = [];
  const timeline = [];
  const dispatched = [];
  let evaluations = 0;
  let observedCalls = 0;
  let physical = 40;
  const page = {
    mouse: {
      async move(x, y) { calls.push(["move", x, y]); timeline.push("move"); },
      async down(options) { calls.push(["down", options]); timeline.push("down"); },
      async up(options) { calls.push(["up", options]); timeline.push("up"); },
    },
    async evaluate(callback, point) {
      evaluations += 1;
      const previous = {
        document: globalThis.document,
        HTMLCanvasElement: globalThis.HTMLCanvasElement,
        MouseEvent: globalThis.MouseEvent,
        window: globalThis.window,
      };
      class FakeMouseEvent {
        constructor(type, init) {
          this.type = type;
          Object.assign(this, init);
        }
      }
      class FakeCanvas {
        dispatchEvent(event) {
          const kind = event.type === "mousedown" ? "down" : "up";
          dispatched.push(event);
          calls.push([kind, event]);
          timeline.push(kind);
          return true;
        }
      }
      const canvas = new FakeCanvas();
      globalThis.document = { getElementById: (id) => id === "glcanvas" ? canvas : null };
      globalThis.HTMLCanvasElement = FakeCanvas;
      globalThis.MouseEvent = FakeMouseEvent;
      globalThis.window = { name: "fixture-window" };
      try {
        await callback(point);
      } finally {
        globalThis.document = previous.document;
        globalThis.HTMLCanvasElement = previous.HTMLCanvasElement;
        globalThis.MouseEvent = previous.MouseEvent;
        globalThis.window = previous.window;
      }
    },
  };
  const advanceFrame = async () => {
    timeline.push("frame");
    if (observedCalls === calls.length) return;
    const freshCalls = calls.slice(observedCalls);
    observedCalls = calls.length;
    if (freshCalls.map(([kind]) => kind).join(",") === "down,up,down,up") {
      const firstPhysical = physical + 1;
      physical += 4;
      routes.push(`[windowing-route] physical=${firstPhysical + 1}:0 kind=pointer router=root application=sector_window underlay=0`);
      routes.push(`[windowing-route] physical=${physical}:0 kind=pointer router=root application=sector_window underlay=0`);
      routes.push(`[windowing-route] physical=${firstPhysical}:0 kind=pointer router=root application=none underlay=0`);
      routes.push(`[windowing-route] physical=${firstPhysical + 2}:0 kind=pointer router=root application=none underlay=0`);
      return;
    }
    for (const [kind] of freshCalls) {
      physical += 1;
      const application = kind === "move" ? "pointer_ready" : "sector_window";
      routes.push(`[windowing-route] physical=${physical}:0 kind=pointer router=root application=${application} underlay=0`);
    }
  };

  const [first, second] = await doubleClick(
    page,
    routes,
    { x: 104, y: 221 },
    "sector_window",
    { advanceFrame },
  );

  assert.deepEqual(calls.map(([kind]) => kind), ["move", "down", "up", "down", "up"]);
  assert.equal(evaluations, 1);
  assert.deepEqual(
    dispatched.map(({ type, button, buttons, detail, clientX, clientY, bubbles, cancelable }) => ({
      type, button, buttons, detail, clientX, clientY, bubbles, cancelable,
    })),
    [
      { type: "mousedown", button: 0, buttons: 1, detail: 1, clientX: 104, clientY: 221, bubbles: true, cancelable: true },
      { type: "mouseup", button: 0, buttons: 0, detail: 1, clientX: 104, clientY: 221, bubbles: true, cancelable: true },
      { type: "mousedown", button: 0, buttons: 1, detail: 2, clientX: 104, clientY: 221, bubbles: true, cancelable: true },
      { type: "mouseup", button: 0, buttons: 0, detail: 2, clientX: 104, clientY: 221, bubbles: true, cancelable: true },
    ],
  );
  assert.deepEqual(timeline, [
    "move", "frame",
    "down", "up", "down", "up", "frame",
    "frame", "frame", "frame",
  ]);
  assert.equal(first.down.physical, "42:0");
  assert.equal(first.down.application, "none");
  assert.equal(first.release.physical, "43:0");
  assert.equal(first.release.application, "sector_window");
  assert.equal(second.down.physical, "44:0");
  assert.equal(second.down.application, "none");
  assert.equal(second.release.physical, "45:0");
  assert.equal(second.release.application, "sector_window");
  assert.notEqual(second.down.physical, first.release.physical);
  assert.notEqual(second.release.physical, first.release.physical);
});

function deferred() {
  let resolve;
  const promise = new Promise((finish) => { resolve = finish; });
  return { promise, resolve };
}
