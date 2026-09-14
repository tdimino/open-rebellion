import assert from "node:assert/strict";
import { EventEmitter } from "node:events";
import test from "node:test";
import vm from "node:vm";
import { performCommand } from "./command-events.mjs";

class PageEvents extends EventEmitter {
  paints = 0;
  async evaluate(fn) {
    return vm.runInNewContext(`(${fn.toString()})()`, {
      requestAnimationFrame: (callback) => setTimeout(() => {
        this.paints += 1;
        callback();
      }, 0),
    });
  }
}

const expected = "command=0x132 destination=gid_menu status=opened_original";
const emit = (page, text = expected) => page.emit("console", {
  text: () => `[interface] ${text}`,
});

test("waits for a command delivered later than the old 80 ms delay", async () => {
  const page = new PageEvents();
  let timer;
  try {
    await performCommand(page, expected, async () => {
      timer = setTimeout(() => emit(page), 150);
    }, 1000);
    assert.equal(page.listenerCount("console"), 0);
  } finally {
    clearTimeout(timer);
  }
});

test("observes an event emitted synchronously by the action", async () => {
  const page = new PageEvents();
  await performCommand(page, expected, () => emit(page));
  assert.equal(page.listenerCount("console"), 0);
});

test("a prior opening cannot satisfy a new click", async () => {
  const page = new PageEvents();
  emit(page);
  await assert.rejects(performCommand(page, expected, () => {}, 30), /0x132/);
  assert.equal(page.listenerCount("console"), 0);
});

test("a different command cannot satisfy the expected event", async () => {
  const page = new PageEvents();
  await assert.rejects(performCommand(page, expected, () => {
    emit(page, "command=0x133 destination=game_options status=opened");
  }, 30), /0x132/);
  assert.equal(page.listenerCount("console"), 0);
});

test("an action failure propagates and removes the listener", async () => {
  const page = new PageEvents();
  await assert.rejects(performCommand(page, expected, () => {
    throw new Error("click failed");
  }), /click failed/);
  assert.equal(page.listenerCount("console"), 0);
});

test("does not continue before the command has been painted", async () => {
  const page = new PageEvents();
  await performCommand(page, expected, () => emit(page));
  assert.ok(page.paints >= 2, "event delivery alone does not prove next-frame rendering");
});

test("a stalled paint remains bounded and cleans up the listener", async () => {
  const page = new PageEvents();
  page.evaluate = () => new Promise(() => {});
  await assert.rejects(performCommand(page, expected, () => emit(page), 30), /event and paint/);
  assert.equal(page.listenerCount("console"), 0);
});
