import assert from "node:assert/strict";
import test from "node:test";
import { launchBrowser } from "./browser-launch.mjs";

const timeoutError = () => Object.assign(new Error("browserType.launch: Timeout 30000ms exceeded."), {
  name: "TimeoutError",
});

test("records a successful fresh browser launch", async () => {
  const browser = {};
  const attempts = [];
  assert.equal(await launchBrowser({ launch: async () => browser }, {}, attempts), browser);
  assert.equal(attempts.length, 1);
  assert.equal(attempts[0].status, "pass");
});

test("one launch timeout may recover, with both attempts retained", async () => {
  let calls = 0;
  const browser = {};
  const attempts = [];
  const launcher = { launch: async () => {
    if (++calls === 1) throw timeoutError();
    return browser;
  } };
  assert.equal(await launchBrowser(launcher, {}, attempts), browser);
  assert.equal(calls, 2);
  assert.deepEqual(attempts.map(({ status }) => status), ["fail", "pass"]);
  assert.match(attempts[0].error, /Timeout 30000ms/);
});

test("a second launch timeout fails the case and retains both errors", async () => {
  const attempts = [];
  await assert.rejects(launchBrowser({ launch: async () => { throw timeoutError(); } }, {}, attempts),
    /Timeout 30000ms/);
  assert.deepEqual(attempts.map(({ status }) => status), ["fail", "fail"]);
});

test("non-timeout launch errors fail immediately", async () => {
  const attempts = [];
  await assert.rejects(launchBrowser({ launch: async () => {
    throw new Error("executable missing");
  } }, {}, attempts), /executable missing/);
  assert.equal(attempts.length, 1);
});
