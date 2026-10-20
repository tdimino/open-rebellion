import assert from "node:assert/strict";
import test from "node:test";

import {
  applicationEvents,
  assertExactHistoryVictim,
  assertIdentityAndRecipientDeltas,
  routedReceipt,
} from "./windowing-contract-identity-history.mjs";

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
