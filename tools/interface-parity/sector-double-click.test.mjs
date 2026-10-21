import assert from "node:assert/strict";
import test from "node:test";

import { sectorDoubleClick } from "./sector-double-click.mjs";

test("sector double click reaches one input batch with native click details", async () => {
  const timeline = [];
  const dispatched = [];
  const page = {
    mouse: {
      async move(x, y) { timeline.push(["move", x, y]); },
    },
    async evaluate(callback, point) {
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
          dispatched.push(event);
          timeline.push([event.type]);
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
  const advanceFrame = async () => { timeline.push(["frame"]); };

  await sectorDoubleClick(page, { x: 104, y: 221 }, advanceFrame);

  assert.deepEqual(timeline, [
    ["move", 104, 221],
    ["frame"],
    ["mousedown"],
    ["mouseup"],
    ["mousedown"],
    ["mouseup"],
    ["frame"],
  ]);
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
});
