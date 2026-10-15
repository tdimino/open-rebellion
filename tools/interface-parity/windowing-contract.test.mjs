import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";

import {
  WINDOWING_BUILD_SOURCE_FILES,
  verifyWindowingBuildManifest,
  writeWindowingBuildManifest,
} from "./windowing-build-manifest.mjs";

import {
  ADAPTER_SMOKE_CASE,
  REVIEWED_WINDOWING_CONTRACT_MANIFEST,
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
  smokeEventFromReceipts,
  smokeActionCounters,
  strategicGidDisplayOffPoint,
  strategicGidPoint,
  validateWindowingContractEvidence,
  validateWindowingContractManifest,
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
        trigger_id: "pointer-open",
        event_id: "41:2",
        kind: "pointer",
        recipient: "root",
        application_recipient: "gid_menu",
        underlay_action_count: 0,
        observed_command: "[interface] command=0x132 destination=gid_menu status=opened_original",
        observed_route: "[windowing-route] physical=41:2 kind=pointer router=root application=gid_menu underlay=0",
        observed_actions: ["[interface] command=0x132 destination=gid_menu status=opened_original"],
      },
      {
        trigger_id: "pointer-select",
        event_id: "45:1",
        kind: "pointer",
        recipient: "blocked",
        application_recipient: "gid",
        underlay_action_count: 0,
        observed_command: "[interface] command=0x80 destination=gid status=selected label=Display Off",
        observed_route: "[windowing-route] physical=45:1 kind=pointer router=blocked application=gid underlay=0",
        observed_actions: ["[interface] command=0x80 destination=gid status=selected label=Display Off"],
      },
      {
        trigger_id: "key-alt-2",
        event_id: "48:0",
        kind: "key",
        recipient: "root",
        application_recipient: "gid",
        underlay_action_count: 0,
        observed_command: "[interface] command=0x12 destination=gid status=selected label=Uprisings source=accelerator",
        observed_route: "[windowing-route] physical=48:0 kind=key router=root application=gid underlay=0",
        observed_actions: [
          "[interface] command=0x12 destination=gid status=selected label=Uprisings source=accelerator",
        ],
      },
    ],
    action_counters: {
      before: { gid_menu: 0, gid_selection: 0 },
      after: { gid_menu: 1, gid_selection: 2 },
    },
    ...overrides,
  };
}

function evidenceFixture(t) {
  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "windowing-run-evidence-"));
  t.after(() => fs.rmSync(directory, { recursive: true, force: true }));
  const artifacts = [
    ["result_json", "result.json", "result"],
    ["screenshot", "after.png", "image"],
    ["fixture_wasm", "fixture.wasm", "wasm"],
  ].map(([kind, relative, contents]) => {
    fs.writeFileSync(path.join(directory, relative), contents);
    return { kind, path: relative, sha256: createHash("sha256").update(contents).digest("hex") };
  });
  return {
    evidenceRoot: directory,
    evidence: {
      schema_version: 1,
      diagnostics: { errors: [], requests: [{ path: "/", status: 200 }] },
      artifacts,
    },
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
        { ...observation().events[0], event_id: "same-event" },
        { ...observation().events[1], event_id: "same-event" },
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

test("windowing_contract_case_selection_reports_executed_cases", async (t) => {
  const hooks = [];
  const fixture = evidenceFixture(t);
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
    evidence: async () => {
      hooks.push("evidence");
      return fixture.evidence;
    },
  })], ["case-a"]);

  const summary = await runSelectedCases(selected, async () => ({ evidenceRoot: fixture.evidenceRoot }));
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

test("windowing evidence requires clean diagnostics and verified artifact hashes", () => {
  const directory = fs.mkdtempSync(path.join(os.tmpdir(), "windowing-evidence-"));
  try {
    const artifacts = [
      ["result_json", "result.json", "result"],
      ["screenshot", "after.png", "image"],
      ["fixture_wasm", "fixture.wasm", "wasm"],
    ].map(([kind, relative, contents]) => {
      fs.writeFileSync(path.join(directory, relative), contents);
      return {
        kind,
        path: relative,
        sha256: createHash("sha256").update(contents).digest("hex"),
      };
    });
    const evidence = {
      schema_version: 1,
      diagnostics: { errors: [], requests: [{ path: "/", status: 200 }] },
      artifacts,
    };
    assert.deepEqual(validateWindowingContractEvidence(evidence, directory), evidence);
    assert.throws(() => validateWindowingContractEvidence({}, directory), /structured evidence/i);
    assert.throws(
      () => validateWindowingContractEvidence({
        ...evidence,
        diagnostics: { errors: ["page:error"], requests: [] },
      }, directory),
      /diagnostics.*error/i,
    );
    assert.throws(
      () => validateWindowingContractEvidence({
        ...evidence,
        artifacts: artifacts.map((artifact) => artifact.kind === "screenshot"
          ? { ...artifact, sha256: "0".repeat(64) }
          : artifact),
      }, directory),
      /hash mismatch.*after\.png/i,
    );
    assert.throws(
      () => validateWindowingContractEvidence({ ...evidence, artifacts: artifacts.slice(0, 2) }, directory),
      /missing artifact.*fixture_wasm/i,
    );
  } finally {
    fs.rmSync(directory, { recursive: true, force: true });
  }
});

test("case execution rejects unstructured evidence", async () => {
  const candidate = defineWindowingContractCase({
    id: "unstructured-evidence",
    groups: ["A"],
    factions: ["alliance"],
    viewports: ["native"],
    setup: async () => {},
    actions: async () => observation({ case_id: "unstructured-evidence" }),
    assertions: async () => 1,
    evidence: async () => ({}),
  });
  await assert.rejects(
    () => runSelectedCases([candidate], async () => ({ evidenceRoot: os.tmpdir() })),
    /structured evidence/i,
  );
});

test("windowing_contract_default_rejects_missing_reviewed_cases", () => {
  assert.throws(
    () => selectCases([ADAPTER_SMOKE_CASE], []),
    /missing reviewed cases.*journey-a.*journey-b.*journey-c.*journey-d.*journey-e/i,
  );
});

test("default selection is authorized by the reviewed case and step manifest", () => {
  const manifest = validateWindowingContractManifest({
    schema_version: 1,
    kind: "open-rebellion-windowing-contract-manifest",
    source_plan: "docs/plans/2026-10-10-strategic-windowing-toolkit-implementation.md",
    cases: ["A", "B", "C", "D", "E"].map((group) => ({
      id: `journey-${group.toLowerCase()}`,
      group,
      required_steps: [`${group}01`],
      applicability: { factions: ["alliance", "empire"], viewports: ["native", "letterboxed"] },
    })),
  });
  const candidate = (
    id,
    groups,
    factions = ["alliance", "empire"],
    viewports = ["native", "letterboxed"],
    steps = groups.map((group) => `${group}01`),
  ) =>
    defineWindowingContractCase({
      id,
      groups,
      steps,
      factions,
      viewports,
      setup: async () => {},
      actions: async () => observation({ case_id: id }),
      assertions: async () => 1,
      evidence: async () => ({}),
    });

  assert.throws(
    () => selectCases([candidate("omnibus", ["A", "B", "C", "D", "E"])], [], manifest),
    /reviewed cases.*journey-a.*journey-b.*journey-c.*journey-d.*journey-e/i,
  );
  const reviewed = ["A", "B", "C", "D", "E"].map((group) =>
    candidate(`journey-${group.toLowerCase()}`, [group]));
  assert.deepEqual(selectCases(reviewed, [], manifest).map(({ id }) => id), [
    "journey-a", "journey-b", "journey-c", "journey-d", "journey-e",
  ]);
  reviewed[0] = candidate(
    "journey-a",
    ["A"],
    ["alliance", "empire"],
    ["native", "letterboxed"],
    [],
  );
  assert.throws(
    () => selectCases(reviewed, [], manifest),
    /journey-a.*required steps.*A01/i,
  );
  reviewed[0] = candidate("journey-a", ["A"], ["alliance"], ["native"]);
  assert.throws(
    () => selectCases(reviewed, [], manifest),
    /journey-a.*applicability.*empire.*letterboxed/i,
  );
});

test("tracked manifest contains the reviewed A01 through E10 inventory", () => {
  const manifest = validateWindowingContractManifest(JSON.parse(fs.readFileSync(
    new URL("./windowing-contract-manifest.json", import.meta.url),
    "utf8",
  )));
  assert.deepEqual(manifest.cases.map(({ id, group, required_steps, applicability }) => ({
    id,
    group,
    required_steps,
    applicability,
  })), ["A", "B", "C", "D", "E"].map((group) => ({
    id: `journey-${group.toLowerCase()}`,
    group,
    required_steps: Array.from({ length: 10 }, (_, index) => `${group}${String(index + 1).padStart(2, "0")}`),
    applicability: { factions: ["alliance", "empire"], viewports: ["native", "letterboxed"] },
  })));
});

test("windowing contract scope distinguishes full default and focused selections", () => {
  const journeys = REVIEWED_WINDOWING_CONTRACT_MANIFEST.cases.map((required) => ({
    id: required.id,
    groups: [required.group],
    steps: required.required_steps,
  }));
  assert.equal(windowingContractScope(journeys, []), "complete A-E fixture journey");
  assert.equal(
    windowingContractScope([{ id: "omnibus", groups: ["A", "B", "C", "D", "E"], steps: [] }], []),
    "focused fixture selection; not complete A-E acceptance",
  );
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
      events: [observation().events[2]],
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

test("adapter smoke derives the physical event and recipient from the application route receipt", () => {
  const event = smokeEventFromReceipts(
    "pointer-select",
    "[interface] command=0x80 destination=gid status=selected label=Display Off",
    "[windowing-route] physical=77:3 kind=pointer router=blocked application=gid underlay=0",
    ["[interface] command=0x80 destination=gid status=selected label=Display Off"],
  );
  assert.deepEqual(event, {
    trigger_id: "pointer-select",
    event_id: "77:3",
    kind: "pointer",
    recipient: "blocked",
    application_recipient: "gid",
    underlay_action_count: 0,
    observed_command: "[interface] command=0x80 destination=gid status=selected label=Display Off",
    observed_route: "[windowing-route] physical=77:3 kind=pointer router=blocked application=gid underlay=0",
    observed_actions: ["[interface] command=0x80 destination=gid status=selected label=Display Off"],
  });
  assert.throws(
    () => smokeEventFromReceipts(
      "pointer-select",
      "[interface] command=0x80 destination=gid status=selected label=Display Off",
      "[windowing-route] physical=77:3 kind=pointer router=blocked application=gid underlay=1",
      ["[interface] command=0x80 destination=gid status=selected label=Display Off"],
    ),
    /underlay.*zero/i,
  );
  assert.throws(
    () => smokeEventFromReceipts(
      "pointer-select",
      "[interface] command=0x80 destination=gid status=selected label=Display Off",
      "[windowing-route] physical=77:3 kind=pointer router=blocked application=gid underlay=0",
      [
        "[interface] command=0x80 destination=gid status=selected label=Display Off",
        "[interface] command=0x201 destination=galaxy_map status=applied",
      ],
    ),
    /underlay.*zero/i,
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

test("no-build verification rejects stale reviewed sources and fixture artifacts", () => {
  const repository = fs.mkdtempSync(path.join(os.tmpdir(), "windowing-build-manifest-"));
  try {
    const fixtureSite = path.join(repository, "site");
    fs.mkdirSync(fixtureSite);
    fs.writeFileSync(path.join(repository, "reviewed-source.txt"), "source-v1");
    fs.writeFileSync(path.join(fixtureSite, "fixture.bin"), "fixture-v1");
    const options = {
      repoRoot: repository,
      siteRoot: fixtureSite,
      sourceFiles: ["reviewed-source.txt"],
      artifactFiles: ["fixture.bin"],
    };
    writeWindowingBuildManifest(options);
    assert.equal(verifyWindowingBuildManifest(options).status, "verified");

    fs.writeFileSync(path.join(repository, "reviewed-source.txt"), "source-v2");
    assert.throws(() => verifyWindowingBuildManifest(options), /source hash mismatch.*reviewed-source/i);
    fs.writeFileSync(path.join(repository, "reviewed-source.txt"), "source-v1");
    fs.writeFileSync(path.join(fixtureSite, "fixture.bin"), "fixture-v2");
    assert.throws(() => verifyWindowingBuildManifest(options), /artifact hash mismatch.*fixture\.bin/i);
  } finally {
    fs.rmSync(repository, { recursive: true, force: true });
  }
});

test("build manifest binds the fixture, router, runner, and build-tool inputs", () => {
  for (const required of [
    "Cargo.toml",
    "Cargo.lock",
    ".cargo/config.toml",
    "scripts/build-wasm.sh",
    "scripts/build-runtime-pack.py",
    "assets/fonts/LiberationSans-Regular.ttf",
    "assets/fonts/LiberationSans-Bold.ttf",
    "tools/interface-parity/prepare-site.mjs",
    "tools/interface-parity/browser-launch.mjs",
    "tools/interface-parity/command-events.mjs",
    "tools/interface-parity/browser.json",
    "tools/interface-parity/package.json",
    "tools/interface-parity/package-lock.json",
    "web/index.html",
    "web/gl.js",
    "crates/rebellion-windowing/src/manager.rs",
    "crates/rebellion-windowing/src/snapshot.rs",
    "crates/rebellion-windowing/src/policy.rs",
  ]) {
    assert.ok(WINDOWING_BUILD_SOURCE_FILES.includes(required), required);
  }
  assert.equal(
    WINDOWING_BUILD_SOURCE_FILES.some((filename) => filename.includes("/node_modules/")),
    false,
    "dependency installation must not affect the reviewed source inventory",
  );
});
