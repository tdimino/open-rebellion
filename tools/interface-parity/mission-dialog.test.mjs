import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

test("Mission targeting brackets the original framed menu click with lifecycle observations", async () => {
  const runner = readFileSync(new URL("./mission-dialog.mjs", import.meta.url), "utf8");
  assert.match(runner, /chooseMissionTargetMenuRow\(/);
  assert.match(runner, /waitForMissionTargetMenuClosed/);

  const { chooseMissionTargetMenuRow } = await import("./mission-dialog-input.mjs");
  const calls = [];
  let markReady;
  const menuReady = new Promise((resolve) => { markReady = resolve; });
  const choosing = chooseMissionTargetMenuRow({
    point: { x: 411, y: 213 },
    waitForMenuReady: async () => {
      calls.push(["menu-ready-wait"]);
      await menuReady;
      calls.push(["menu-ready"]);
    },
    click: async (point) => { calls.push(["framed-click", point]); },
    waitForMenuClosed: async () => { calls.push(["menu-closed"]); },
  });

  await Promise.resolve();
  assert.deepEqual(calls, [["menu-ready-wait"]], "the row input must wait for menu publication");
  markReady();
  await choosing;

  assert.deepEqual(calls, [
    ["menu-ready-wait"],
    ["menu-ready"],
    ["framed-click", { x: 411, y: 213 }],
    ["menu-closed"],
  ]);
});

test("Mission targeting waits for the manager menu to be open and input-ready", async () => {
  const { waitForMissionTargetMenuReady } = await import("./mission-dialog-input.mjs");
  const calls = [];
  const page = {
    async waitForFunction(predicate, argument, options) {
      calls.push(["wait", argument, options]);
      assert.equal(typeof predicate, "function");
      globalThis.window = {
        __openRebellionInterfaceMenuLifecycle: { open: true, input_ready: true },
      };
      assert.equal(predicate(), true, "manager lifecycle readiness is sufficient for this path");
    },
    async evaluate(callback) {
      calls.push(["read"]);
      assert.equal(typeof callback, "function");
      return { status: "menu-lifecycle", open: true, input_ready: true };
    },
  };

  assert.deepEqual(await waitForMissionTargetMenuReady(page), {
    status: "menu-lifecycle",
    open: true,
    input_ready: true,
  });
  assert.deepEqual(calls, [
    ["wait", null, { timeout: 10_000, polling: 100 }],
    ["read"],
  ]);
});

test("Mission targeting waits on the fixture's semantic menu-close boundary", async () => {
  const { waitForMissionTargetMenuClosed } = await import("./mission-dialog-input.mjs");
  const calls = [];
  const page = {
    async waitForFunction(predicate, argument, options) {
      calls.push(["wait", argument, options]);
      assert.equal(typeof predicate, "function");
    },
    async evaluate(callback) {
      calls.push(["read"]);
      assert.equal(typeof callback, "function");
      return { status: "menu-lifecycle", open: false, input_ready: false };
    },
  };

  assert.deepEqual(await waitForMissionTargetMenuClosed(page), {
    status: "menu-lifecycle",
    open: false,
    input_ready: false,
  });
  assert.deepEqual(calls, [
    ["wait", null, { timeout: 10_000, polling: 100 }],
    ["read"],
  ]);
});
