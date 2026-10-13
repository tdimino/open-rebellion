import assert from "node:assert/strict";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";

import {
  ADAPTER_SMOKE_CASE,
  acquireBrowserPage,
  assertAdapterSmokeObservation,
  createBrowserRuntime,
  decodeFixtureRequest,
  defineWindowingContractCase,
  discoverCaseModules,
  encodeFixtureRequest,
  isCaseModuleFilename,
  parseWindowingContractArgs,
  runSelectedCases,
  selectCases,
  smokeActionCounters,
  strategicGidDisplayOffPoint,
  strategicGidPoint,
  validateObservation,
  windowingContractScope,
} from "./windowing-contract.mjs";

function observation(overrides = {}) {
  return {
    schema_version: 1,
    case_id: "adapter-smoke",
    faction: "alliance",
    viewport: "native",
    events: [
      {
        event_id: "pointer-open",
        kind: "pointer",
        recipient: "root",
        observed_command: "[interface] command=0x132 destination=gid_menu status=opened_original",
      },
      {
        event_id: "pointer-select",
        kind: "pointer",
        recipient: "menu",
        observed_command: "[interface] command=0x80 destination=gid status=selected label=Display Off",
      },
      {
        event_id: "key-alt-2",
        kind: "key",
        recipient: "root",
        observed_command: "[interface] command=0x12 destination=gid status=selected label=Uprisings source=accelerator",
      },
    ],
    action_counters: {
      before: { gid_menu: 0, gid_selection: 0 },
      after: { gid_menu: 1, gid_selection: 2 },
    },
    ...overrides,
  };
}

test("windowing fixture requests preserve scenario, faction and viewport", () => {
  for (const faction of ["alliance", "empire"]) {
    for (const viewport of ["native", "letterboxed"]) {
      const encoded = encodeFixtureRequest({ scenario: 65, faction, viewport });
      assert.equal(encoded.fixtureCode & 0xff, 66);
      assert.deepEqual(decodeFixtureRequest(encoded), {
        scenario: 65,
        faction,
        viewport,
      });
    }
  }
});

test("windowing_contract_observations_reject_duplicate_recipients", () => {
  assert.deepEqual(validateObservation(observation()), observation());
  assert.throws(
    () => validateObservation(observation({
      events: [
        { event_id: "same-event", kind: "pointer", recipient: "root" },
        { event_id: "same-event", kind: "pointer", recipient: "window:7" },
      ],
    })),
    /duplicate recipient.*same-event/i,
  );
  assert.throws(
    () => validateObservation(observation({
      action_counters: {
        before: { gid_menu: 1 },
        after: { gid_selection: 1 },
      },
    })),
    /counter keys/i,
  );
});

test("windowing_contract_case_selection_reports_executed_cases", async () => {
  const hooks = [];
  const selected = selectCases([defineWindowingContractCase({
    id: "case-a",
    groups: ["A"],
    factions: ["alliance"],
    viewports: ["native"],
    setup: async () => hooks.push("setup"),
    actions: async () => {
      hooks.push("actions");
      return observation({ case_id: "case-a" });
    },
    assertions: async () => {
      hooks.push("assertions");
      return 3;
    },
    evidence: async () => hooks.push("evidence"),
  })], ["case-a"]);

  const summary = await runSelectedCases(selected, async () => ({}));
  assert.deepEqual(hooks, ["setup", "actions", "assertions", "evidence"]);
  assert.equal(summary.executed_cases, 1);
  assert.equal(summary.assertion_count, 3);
});

test("windowing_contract_runner_validates_observations_and_execution_identity", async () => {
  const candidate = (id, actions) => defineWindowingContractCase({
    id,
    groups: ["A"],
    factions: ["alliance"],
    viewports: ["native"],
    setup: async () => {},
    actions,
    assertions: async () => 1,
    evidence: async () => ({ result_json: "result.json" }),
  });

  await assert.rejects(
    () => runSelectedCases([candidate("malformed", async () => ({ malformed: true }))], async () => ({})),
    /observation|schema/i,
  );
  await assert.rejects(
    () => runSelectedCases([
      candidate("expected-case", async () => observation({ case_id: "another-case" })),
    ], async () => ({})),
    /observation identity.*expected-case\/alliance\/native/i,
  );
});

test("windowing_contract_default_rejects_missing_journey_groups", () => {
  assert.throws(() => selectCases([ADAPTER_SMOKE_CASE], []), /missing journey groups.*A.*B.*C.*D.*E/i);
});

test("windowing contract scope distinguishes full default and focused selections", () => {
  const journeys = [{ groups: ["A", "B", "C", "D", "E"] }];
  assert.equal(windowingContractScope(journeys, []), "complete A-E fixture journey");
  assert.equal(
    windowingContractScope(journeys, ["case-a"]),
    "focused fixture selection; not complete A-E acceptance",
  );
  assert.equal(
    windowingContractScope([ADAPTER_SMOKE_CASE], ["adapter-smoke"]),
    "adapter smoke only; not A-E acceptance",
  );
});

test("windowing_contract_unknown_case_and_zero_assertions_fail", async () => {
  assert.throws(() => selectCases([ADAPTER_SMOKE_CASE], ["unknown"]), /unknown case.*unknown/i);

  const zero = defineWindowingContractCase({
    id: "zero",
    groups: ["A", "B", "C", "D", "E"],
    factions: ["alliance"],
    viewports: ["native"],
    setup: async () => {},
    actions: async () => observation({ case_id: "zero" }),
    assertions: async () => 0,
    evidence: async () => {},
  });
  await assert.rejects(() => runSelectedCases([zero], async () => ({})), /zero assertions/i);
  await assert.rejects(() => runSelectedCases([], async () => ({})), /zero executed cases/i);
});

test("future journey modules are discovered and imported without editing the runner registry", async (t) => {
  assert.equal(isCaseModuleFilename("windowing-contract-identity-history.mjs"), true);
  assert.equal(isCaseModuleFilename("windowing-contract-modal-boundaries.mjs"), true);
  assert.equal(isCaseModuleFilename("windowing-contract-input-scenes.mjs"), true);
  assert.equal(isCaseModuleFilename("windowing-contract.test.mjs"), false);
  assert.equal(isCaseModuleFilename("windowing-contract.mjs"), false);

  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "windowing-contract-discovery-"));
  t.after(() => fs.rmSync(directory, { recursive: true, force: true }));
  const contractUrl = new URL("./windowing-contract.mjs", import.meta.url).href;
  fs.writeFileSync(path.join(directory, "windowing-contract-valid.mjs"), `
    import { defineWindowingContractCase } from ${JSON.stringify(contractUrl)};
    export const windowingContractCase = defineWindowingContractCase({
      id: "discovered-case",
      groups: ["A"],
      factions: ["alliance"],
      viewports: ["native"],
      setup: async () => {},
      actions: async () => {},
      assertions: async () => 1,
      evidence: async () => {},
    });
  `);
  assert.deepEqual((await discoverCaseModules(directory)).map(({ id }) => id), ["discovered-case"]);

  fs.writeFileSync(
    path.join(directory, "windowing-contract-empty.mjs"),
    "export const windowingContractCases = [];\n",
  );
  await assert.rejects(() => discoverCaseModules(directory), /must export at least one case/i);
});

test("adapter smoke drives pointer and key input through the case interface", async () => {
  const calls = [];
  const expected = observation();
  const context = {
    openFixture: async () => {
      calls.push("open");
      return { status: "ready", code: 322 };
    },
    clickGidMenu: async () => {
      calls.push("pointer");
      return { event_id: "pointer-1", kind: "pointer", recipient: "root" };
    },
    pressGidAccelerator: async () => {
      calls.push("key");
      return { event_id: "key-1", kind: "key", recipient: "root" };
    },
    observation: () => expected,
    writeEvidence: async (evidence) => {
      calls.push("evidence");
      assert.equal(evidence.observation, expected);
      assert.ok(evidence.assertion_count > 0);
      return { result_json: "result.json" };
    },
  };

  const setup = await ADAPTER_SMOKE_CASE.setup(context);
  const actions = await ADAPTER_SMOKE_CASE.actions(context, setup);
  const assertions = await ADAPTER_SMOKE_CASE.assertions(context, actions, setup);
  const evidence = await ADAPTER_SMOKE_CASE.evidence(context, actions, setup);

  assert.deepEqual(calls, ["open", "pointer", "key", "evidence"]);
  assert.ok(assertions > 0);
  assert.equal(evidence.result_json, "result.json");
});

test("adapter smoke assertions require observed routes and exact action deltas", () => {
  assert.ok(assertAdapterSmokeObservation(observation()) > 0);
  assert.throws(
    () => assertAdapterSmokeObservation(observation({
      events: [{ event_id: "key-1", kind: "key", recipient: "root" }],
    })),
    /pointer-open/i,
  );
  assert.throws(
    () => assertAdapterSmokeObservation(observation({
      action_counters: {
        before: { gid_menu: 0, gid_selection: 0 },
        after: { gid_menu: 0, gid_selection: 0 },
      },
    })),
    /exact.*gid_menu/i,
  );
  assert.throws(
    () => assertAdapterSmokeObservation(observation({
      action_counters: {
        before: { gid_menu: 0, gid_selection: 0 },
        after: { gid_menu: 2, gid_selection: 3 },
      },
    })),
    /exact.*gid_menu/i,
  );
});

test("adapter smoke CLI and GID geometry are deterministic", () => {
  assert.deepEqual(
    parseWindowingContractArgs(["--no-build", "--case", "adapter-smoke"]),
    { noBuild: true, caseIds: ["adapter-smoke"] },
  );
  assert.deepEqual(
    parseWindowingContractArgs(["--case=adapter-smoke"]),
    { noBuild: false, caseIds: ["adapter-smoke"] },
  );
  assert.throws(() => parseWindowingContractArgs(["--wat"]), /unknown argument/i);

  assert.deepEqual(strategicGidPoint("alliance", "native"), { x: 459.5, y: 414 });
  assert.deepEqual(strategicGidPoint("empire", "native"), { x: 537.5, y: 446.5 });
  assert.deepEqual(strategicGidDisplayOffPoint("native"), { x: 477, y: 372 });
  const scaled = strategicGidPoint("alliance", "letterboxed");
  assert.ok(scaled.x > 870 && scaled.x < 880);
  assert.equal(scaled.y, 690);
  const scaledDisplayOff = strategicGidDisplayOffPoint("letterboxed");
  assert.ok(scaledDisplayOff.x > 901 && scaledDisplayOff.x < 902);
  assert.equal(scaledDisplayOff.y, 620);
});

test("adapter smoke counters distinguish menu and keyboard actions", () => {
  assert.deepEqual(smokeActionCounters([]), { gid_menu: 0, gid_selection: 0 });
  assert.deepEqual(smokeActionCounters([
    "[interface] command=0x132 destination=gid_menu status=opened_original",
    "[interface] command=0x80 destination=gid status=selected label=Display Off",
    "[interface] command=0x12 destination=gid status=selected label=Uprisings source=accelerator",
  ]), { gid_menu: 1, gid_selection: 2 });
});

test("browser runtime acquisition cleans partial resources and validates before serving", async () => {
  let serverStarts = 0;
  await assert.rejects(
    () => createBrowserRuntime("/unused", {
      assertSite: () => {},
      resolveBrowser: () => { throw new Error("missing browser"); },
      serve: async () => { serverStarts += 1; },
    }),
    /missing browser/i,
  );
  assert.equal(serverStarts, 0);

  let serverCloses = 0;
  await assert.rejects(
    () => createBrowserRuntime("/unused", {
      assertSite: () => {},
      resolveBrowser: () => "/browser",
      serve: async () => ({
        address: () => null,
        close: (callback) => { serverCloses += 1; callback(); },
      }),
    }),
    /listening port/i,
  );
  assert.equal(serverCloses, 1);

  const closed = [];
  const browser = {
    newContext: async () => ({
      newPage: async () => { throw new Error("new page failed"); },
      close: async () => { closed.push("context"); },
    }),
    close: async () => { closed.push("browser"); },
  };
  await assert.rejects(
    () => acquireBrowserPage(async () => browser, {}),
    /new page failed/i,
  );
  assert.deepEqual(closed, ["context", "browser"]);
});
