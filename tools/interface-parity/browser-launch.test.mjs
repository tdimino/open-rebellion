import assert from "node:assert/strict";
import test from "node:test";
import { launchBrowser, summarizeBrowserAttempts } from "./browser-launch.mjs";

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

test("summary retains launches from a recovered whole-scenario retry", () => {
  const results = [{
    status: "pass",
    cleanup: "closed",
    launch_attempts: [
      { attempt: 1, status: "fail", error_name: "TimeoutError" },
      { attempt: 2, status: "pass" },
    ],
    execution_attempts: [
      {
        attempt: 1,
        status: "fail",
        cleanup: "closed",
        retryable_timeout: true,
        launch_attempts: [{ attempt: 1, status: "pass" }],
      },
      {
        attempt: 2,
        status: "pass",
        cleanup: "closed",
        retryable_timeout: false,
        launch_attempts: [
          { attempt: 1, status: "fail", error_name: "TimeoutError" },
          { attempt: 2, status: "pass" },
        ],
      },
    ],
  }];

  assert.deepEqual(summarizeBrowserAttempts(results), {
    browser_executions: 2,
    muted_launches: 3,
    launch_timeouts: 1,
    recovered_launch_timeouts: 1,
    execution_timeouts: 1,
    recovered_execution_timeouts: 1,
    browser_processes_closed: 2,
    browser_executions_closed: 2,
  });
});
