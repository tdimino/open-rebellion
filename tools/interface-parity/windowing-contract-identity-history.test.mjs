import assert from "node:assert/strict";
import test from "node:test";

import {
  applicationEvents,
  assertExactHistoryVictim,
  assertIdentityAndRecipientDeltas,
  routedReceipt,
  windowingContractCases,
} from "./windowing-contract-identity-history.mjs";

async function stateMonitorSetup(documents) {
  const listeners = new Map();
  const page = {
    on(event, listener) {
      listeners.set(event, listener);
    },
    async evaluate() {},
  };
  const emitState = (document) => listeners.get("console")({
    text: () => `[windowing-contract-state] ${JSON.stringify(document)}`,
    type: () => "log",
  });
  const journeyA = windowingContractCases.find(({ id }) => id === "journey-a");
  const setup = await journeyA.setup({
    page,
    async openFixture() {
      for (const document of documents) emitState(document);
      return { fixture: "ready" };
    },
  });
  return { setup, emitState };
}

test("identity_history_state_monitor_ignores_other_state_families_without_hiding_malformed_identity_documents", async () => {
  const validIdentity = {
    status: "identity-history-state",
    sequence: 1,
    available: true,
    required_systems: 1,
    systems: [{ name: "Corellia" }],
  };
  const { setup } = await stateMonitorSetup([
    validIdentity,
    { status: "modal-boundaries-state", sequence: 2 },
  ]);
  assert.deepEqual(setup.monitor.states, [validIdentity]);
  assert.deepEqual(setup.monitor.latest(), validIdentity);

  const malformedIdentity = {
    status: "identity-history-state",
    sequence: 3,
    available: true,
  };
  const { setup: malformedSetup, emitState } = await stateMonitorSetup([validIdentity]);
  emitState(malformedIdentity);
  emitState({ status: "input-scenes-state", sequence: 4 });
  assert.deepEqual(malformedSetup.monitor.states, [validIdentity, malformedIdentity]);
  assert.deepEqual(malformedSetup.monitor.latest(), malformedIdentity);
});

test("windowing_journey_ab_checks_identity_and_recipient_deltas", () => {
  const before = {
    paint_order: [11, 12, 21],
    focused: 21,
    records: [
      { id: 11, kind: "sector", subject: "sector:101" },
      { id: 12, kind: "sector", subject: "sector:202" },
      { id: 21, kind: "fleet", subject: "system:301" },
    ],
    counters: { routed: 7, application: 5, underlay: 0 },
  };
  const duplicate = {
    ...before,
    counters: { routed: 8, application: 6, underlay: 0 },
  };

  assert.equal(assertIdentityAndRecipientDeltas({
    before,
    after: duplicate,
    expected: {
      preserved_ids: [11, 12, 21],
      paint_order: [11, 12, 21],
      focused: 21,
      recipient: "root",
      application_recipient: "sector-opener",
      action_delta: 1,
      routed_delta: 1,
      underlay_delta: 0,
    },
    event: {
      recipient: "root",
      application_recipient: "sector-opener",
      observed_actions: ["open-sector:101"],
      underlay_action_count: 0,
    },
  }), 10);

  assert.throws(() => assertIdentityAndRecipientDeltas({
    before,
    after: {
      ...duplicate,
      counters: { routed: 8, application: 7, underlay: 0 },
    },
    expected: {
      preserved_ids: [11, 12, 21],
      paint_order: [11, 12, 21],
      focused: 21,
      recipient: "root",
      application_recipient: "sector-opener",
      action_delta: 1,
      routed_delta: 1,
      underlay_delta: 0,
    },
    event: {
      recipient: "root",
      application_recipient: "sector-opener",
      observed_actions: ["open-sector:101", "second-application"],
      underlay_action_count: 0,
    },
  }), /application receipt count|application action delta/i);

  const applied = { trigger_id: "applied", observed_actions: ["application-receipt"] };
  const blocked = { trigger_id: "blocked", observed_actions: [] };
  assert.deepEqual(applicationEvents([applied, blocked]), [applied]);
  assert.deepEqual(blocked.observed_actions, [], "blocked routing evidence gained a synthetic action");

  assert.throws(() => routedReceipt(
    "wrong-press-recipient",
    0,
    [
      "[windowing-route] physical=77:1 kind=pointer router=root application=none underlay=0",
      "[windowing-route] physical=77:2 kind=pointer router=window:6 application=defenses_window underlay=0",
    ],
    "window:6",
    "defenses_window",
    2,
  ), /raw route.*recipient|reached root/i);
});

test("windowing_journey_ab_underlay_failures_include_the_offending_route_receipts", () => {
  const offendingRoute = "[windowing-route] physical=54:1 kind=pointer router=root application=sector-opener underlay=1";
  assert.throws(() => routedReceipt(
    "underlay-diagnostic",
    0,
    [offendingRoute, "[windowing-route] physical=54:2 kind=pointer router=root application=none underlay=0"],
    "root",
    "sector-opener",
    2,
  ), (error) => error.message.includes(offendingRoute));
});

test("windowing_journey_ab_checks_exact_history_victim", () => {
  const before = {
    history: Array.from({ length: 12 }, (_, index) => index + 101),
    registered: Array.from({ length: 13 }, (_, index) => index + 101),
  };
  const after = {
    history: Array.from({ length: 12 }, (_, index) => index + 102),
    registered: Array.from({ length: 12 }, (_, index) => index + 102),
  };

  assert.deepEqual(assertExactHistoryVictim(before, after, 113), {
    victim: 101,
    retained: after.history,
    assertions: 8,
  });

  assert.throws(
    () => assertExactHistoryVictim(before, {
      history: [101, ...after.history.slice(1)],
      registered: [101, ...after.registered.slice(1)],
    }, 113),
    /oldest|victim/i,
  );
  assert.throws(
    () => assertExactHistoryVictim(before, {
      history: [...after.history.slice(0, -1), 112],
      registered: after.registered,
    }, 113),
    /duplicate/i,
  );
});
