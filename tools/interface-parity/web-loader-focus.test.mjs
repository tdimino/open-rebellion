import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";
import vm from "node:vm";

const loader = readFileSync(new URL("../../web/gl.js", import.meta.url), "utf8");

class BrowserEvents {
  listeners = new Map();

  addEventListener(type, listener) {
    const listeners = this.listeners.get(type) ?? [];
    listeners.push(listener);
    this.listeners.set(type, listeners);
  }

  dispatch(type) {
    for (const listener of this.listeners.get(type) ?? []) listener({});
  }
}

function focusRuntime() {
  const canvas = new BrowserEvents();
  canvas.focus = () => {};
  canvas.style = {};

  const document = new BrowserEvents();
  document.focused = true;
  document.visibilityState = "visible";
  document.hasFocus = () => document.focused;
  document.querySelector = () => canvas;
  document.exitPointerLock = () => {};

  const window = new BrowserEvents();
  window.requestAnimationFrame = () => 1;

  const sandbox = { alert: () => {}, canvas, console, document, window };
  vm.runInNewContext(loader, sandbox, { filename: "web/gl.js" });

  const notifications = [];
  sandbox.wasm_exports = { focus: (focused) => notifications.push(focused) };
  sandbox.importObject.env.run_animation_loop(true);

  return { document, notifications, window };
}

test("browser blur and refocus notify the WASM runtime once per transition", () => {
  const runtime = focusRuntime();

  runtime.document.focused = false;
  runtime.window.dispatch("blur");
  runtime.window.dispatch("blur");
  runtime.document.focused = true;
  runtime.window.dispatch("focus");

  assert.deepEqual(runtime.notifications, [false, true]);
});

test("a hidden focused tab is blurred until it becomes visible again", () => {
  const runtime = focusRuntime();

  runtime.document.visibilityState = "hidden";
  runtime.document.dispatch("visibilitychange");
  runtime.document.visibilityState = "visible";
  runtime.document.dispatch("visibilitychange");

  assert.deepEqual(runtime.notifications, [false, true]);
});
