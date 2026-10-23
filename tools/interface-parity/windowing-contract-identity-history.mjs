import assert from "node:assert/strict";
import fs from "node:fs";
import { createHash } from "node:crypto";
import path from "node:path";

import {
  WINDOWING_CONTRACT_SCHEMA_VERSION,
  defineWindowingContractCase,
} from "./windowing-contract.mjs";

const STATE_MARKER = "[windowing-contract-state] ";

function sha256(contents) {
  return createHash("sha256").update(contents).digest("hex");
}

async function frames(page, count = 3) {
  for (let index = 0; index < count; index += 1) {
    await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(resolve)));
  }
}

function createStateMonitor(page) {
  const states = [];
  const routes = [];
  const errors = [];
  const requests = [];
  let underlay = 0;
  page.on("console", (message) => {
    const line = message.text();
    const marker = line.indexOf(STATE_MARKER);
    if (marker >= 0) {
      const state = JSON.parse(line.slice(marker + STATE_MARKER.length));
      if (state.status === "identity-history-state") states.push(state);
    }
    if (line.startsWith("[windowing-route] ")) routes.push(line);
    if (line.includes("[windowing-underlay]")) underlay += 1;
    if (message.type() === "error") errors.push(`console:${line}`);
  });
  page.on("pageerror", (error) => errors.push(`page:${error.stack || error}`));
  page.on("response", (response) => {
    const url = new URL(response.url());
    if (url.hostname === "127.0.0.1") requests.push({ path: url.pathname, status: response.status() });
  });
  return {
    states,
    routes,
    errors,
    requests,
    get underlay() { return underlay; },
    counters() {
      return {
        routed: routes.length,
        application: routes.filter((line) => !line.includes(" application=none ")).length,
        underlay,
      };
    },
    latest() {
      assert.ok(states.length > 0, "identity/history fixture emitted no state");
      return states.at(-1);
    },
    async wait(predicate, description, afterSequence = 0) {
      for (let attempt = 0; attempt < 300; attempt += 1) {
        const found = states.findLast((state) => state.sequence > afterSequence && predicate(state));
        if (found) return found;
        await frames(page, 1);
      }
      throw new Error(`timed out waiting for ${description}; last=${JSON.stringify(states.at(-1))}`);
    },
    async waitFirst(predicate, description, afterSequence = 0) {
      for (let attempt = 0; attempt < 300; attempt += 1) {
        const found = states.find((state) => state.sequence > afterSequence);
        if (found) {
          assert.ok(predicate(found), `${description} missed the first post-input state: ${JSON.stringify(found)}`);
          return found;
        }
        await frames(page, 1);
      }
      throw new Error(`timed out waiting for first-frame ${description}; last=${JSON.stringify(states.at(-1))}`);
    },
  };
}

function systemByIndex(state, index) {
  const system = state.systems[index];
  assert.ok(system, `fixture is missing required system ${index}`);
  return system;
}

function systemByDatId(state, datId) {
  const system = state.systems.find((candidate) => candidate.dat_id === datId);
  assert.ok(system, `fixture state is missing system DAT ${datId}`);
  return system;
}

function recordOf(state, id) {
  return state.snapshot.records.find((record) => record.id === id);
}

function idsOfKind(state, kind) {
  return state.snapshot.records.filter((record) => record.kind === kind).map(({ id }) => id);
}

function relativeOrder(state, ids) {
  return state.snapshot.paint_order.filter((id) => ids.includes(id));
}

function detail(state, systemDatId, kind) {
  return systemByDatId(state, systemDatId).details[kind];
}

function center(rect) {
  assert.ok(Array.isArray(rect) && rect.length === 4, `missing rectangle ${JSON.stringify(rect)}`);
  return { x: rect[0] + rect[2] / 2, y: rect[1] + rect[3] / 2 };
}

function detailButton(state, systemDatId, kind, button) {
  const rect = detail(state, systemDatId, kind).rect;
  assert.ok(rect, `${kind} detail ${systemDatId} has no visible rectangle`);
  const localX = kind === "system"
    ? (button === "minimize" ? 202 : 216)
    : (button === "minimize" ? 211 : 225);
  return { x: rect[0] + localX * state.scale, y: rect[1] + 9 * state.scale };
}

function allDetailRects(state) {
  const entries = new Map();
  for (const system of state.systems) {
    for (const kind of ["system", "fleet", "defenses", "missions"]) {
      const shown = system.details[kind];
      if (shown.id && shown.rect) entries.set(shown.id, shown.rect);
    }
  }
  return entries;
}

function contains(rect, point) {
  return point.x >= rect[0] && point.x < rect[0] + rect[2]
    && point.y >= rect[1] && point.y < rect[1] + rect[3];
}

function containsWithMargin(rect, point, margin) {
  return point.x >= rect[0] - margin && point.x < rect[0] + rect[2] + margin
    && point.y >= rect[1] - margin && point.y < rect[1] + rect[3] + margin;
}

function uncoveredPoint(rect, blockers, scale) {
  const inset = 3 * scale;
  for (let y = rect[1] + inset; y < rect[1] + rect[3] - inset; y += 2 * scale) {
    for (let x = rect[0] + inset; x < rect[0] + rect[2] - inset; x += 2 * scale) {
      const candidate = { x, y };
      if (!blockers.some((blocker) => contains(blocker, candidate))) return candidate;
    }
  }
  assert.fail(`control ${JSON.stringify(rect)} has no uncovered point`);
}

function exposedPoint(state, target) {
  const rectangles = allDetailRects(state);
  const rect = rectangles.get(target);
  assert.ok(rect, `no rectangle for detail identity ${target}`);
  const targetIndex = state.snapshot.paint_order.indexOf(target);
  const above = state.snapshot.paint_order.slice(targetIndex + 1)
    .map((id) => rectangles.get(id)).filter(Boolean);
  const candidates = [];
  const shown = state.systems.flatMap((system) => Object.values(system.details))
    .find((candidate) => candidate.id === target);
  assert.ok(shown, `detail identity ${target} has no observed content`);
  const currentTab = shown.selection?.tab ?? shown.selection?.page;
  const currentTabControls = (shown.tabs ?? [])
    .filter(([name]) => name === currentTab)
    .map(([, value]) => value);
  const controls = [shown.entry_rect, shown.cell_rect, ...(shown.tabs ?? []).map(([, value]) => value)]
    .filter(Boolean);
  // An idempotent current-tab click is the best activation probe: it produces
  // the family's native action without changing content before the queued
  // manager activation commits. Fall back to neutral background, then any live
  // control when overlap leaves no other exposed pixels.
  for (const control of currentTabControls) {
    for (let y = control[1] + state.scale; y < control[1] + control[3] - state.scale; y += 2 * state.scale) {
      for (let x = control[0] + state.scale; x < control[0] + control[2] - state.scale; x += 2 * state.scale) {
        const candidate = { x, y };
        if (!above.some((other) => contains(other, candidate))) return candidate;
      }
    }
  }
  // Skip the title-button row: its exposed pixels include minimize/close.
  for (let y = rect[1] + 24 * state.scale; y < rect[1] + rect[3] - 4 * state.scale; y += 6 * state.scale) {
    for (let x = rect[0] + 4 * state.scale; x < rect[0] + rect[2] - 4 * state.scale; x += 6 * state.scale) {
      const candidate = { x, y };
      if (!controls.some((control) => contains(control, candidate))) candidates.push(candidate);
    }
  }
  for (const control of controls) {
    for (let y = control[1] + 3 * state.scale; y < control[1] + control[3] - 2 * state.scale; y += 4 * state.scale) {
      for (let x = control[0] + 3 * state.scale; x < control[0] + control[2] - 2 * state.scale; x += 4 * state.scale) {
        candidates.push({ x, y });
      }
    }
  }
  const point = candidates.find((candidate) =>
    !above.some((other) => containsWithMargin(other, candidate, 4 * state.scale)));
  assert.ok(point, `detail identity ${target} has no exposed control point: rect=${JSON.stringify(rect)} controls=${JSON.stringify(controls)} above=${JSON.stringify(above)}`);
  return point;
}

function compact(state) {
  return {
    sequence: state.sequence,
    scale: state.scale,
    snapshot: state.snapshot,
    rail: state.rail,
    systems: state.systems.map((system) => ({
      dat_id: system.dat_id,
      name: system.name,
      sector_id: system.sector_id,
      map_point: system.map_point,
      sector_title_point: system.sector_title_point,
      sector_close_point: system.sector_close_point,
      planet_rect: system.planet_rect,
      quadrants: system.quadrants,
      details: Object.fromEntries(Object.entries(system.details).map(([kind, value]) => [kind, {
        id: value.id,
        rect: value.rect,
        selection: value.selection,
        entry_rect: value.entry_rect,
        cell_rect: value.cell_rect,
        tabs: value.tabs,
      }])),
    })),
  };
}

export function routedReceipt(
  triggerId,
  index,
  lines,
  expectedRecipient,
  expectedApplicationRecipient,
  expectedRouteCount,
) {
  assert.equal(
    lines.length,
    expectedRouteCount,
    `${triggerId} emitted ${lines.length}/${expectedRouteCount} raw route receipts: ${JSON.stringify(lines)}`,
  );
  const parsed = lines.map((line) => {
    const match = line.match(
      /^\[windowing-route\] physical=(\d+:\d+) kind=(pointer|key) router=(root|blocked|window:\d+) application=([a-z0-9_:-]+) underlay=(\d+)$/,
    );
    assert.ok(match, `${triggerId} has malformed route receipt ${line}`);
    const [, eventId, kind, recipient, applicationRecipient, underlay] = match;
    return { line, eventId, kind, recipient, applicationRecipient, underlay: Number(underlay) };
  });
  const applicationReceipts = parsed.filter(({ applicationRecipient }) => applicationRecipient !== "none");
  const expectedApplicationCount = expectedApplicationRecipient === "none" ? 0 : 1;
  assert.equal(
    applicationReceipts.length,
    expectedApplicationCount,
    `${triggerId} emitted ${applicationReceipts.length}/${expectedApplicationCount} application receipts: ${JSON.stringify(lines)}`,
  );
  const selected = expectedApplicationRecipient === "none"
    ? parsed.at(-1)
    : applicationReceipts[0];
  assert.ok(selected, `${triggerId} emitted no route receipt`);
  const { line, eventId, kind, recipient, applicationRecipient, underlay } = selected;
  const normalizedExpected = expectedRecipient.startsWith("blocked:") ? "blocked" : expectedRecipient;
  assert.ok(
    parsed.every((route) => route.recipient === normalizedExpected),
    `${triggerId} raw route recipient mismatch: ${JSON.stringify(lines)}`,
  );
  assert.equal(recipient, normalizedExpected, `${triggerId} reached ${recipient}, expected ${normalizedExpected}`);
  assert.equal(
    applicationRecipient,
    expectedApplicationRecipient,
    `${triggerId} reached application ${applicationRecipient}, expected ${expectedApplicationRecipient}`,
  );
  assert.ok(
    parsed.every((route) => route.underlay === 0),
    `${triggerId} leaked an underlay action: ${JSON.stringify(lines)}`,
  );
  return {
    trigger_id: triggerId,
    event_id: eventId,
    kind,
    recipient,
    application_recipient: applicationRecipient,
    underlay_action_count: underlay,
    observed_route: line,
    observed_routes: lines,
    observed_actions: applicationReceipts.map((route) => route.line),
    receipt_index: index,
  };
}

function assertReceiptCounterDeltas(triggerId, before, after, event) {
  assert.equal(
    after.routed - before.routed,
    event.observed_routes.length,
    `${triggerId} raw route counter disagrees with observed receipts`,
  );
  assert.equal(
    after.application - before.application,
    event.observed_actions.length,
    `${triggerId} application counter disagrees with observed receipts`,
  );
}

async function setupJourney(context) {
  const monitor = createStateMonitor(context.page);
  const ready = await context.openFixture();
  const initial = await monitor.wait(
    (state) => state.status === "identity-history-state",
    "initial identity/history state",
  );
  assert.equal(initial.available, true, `fixture has ${initial.systems.length}/${initial.required_systems} required systems`);
  assert.ok(initial.systems.every(({ name }) => typeof name === "string" && name.length > 0));
  return {
    monitor,
    ready,
    initial,
    events: [],
    timeline: [],
    screenshotHashes: {},
    evidenceScreenshots: [],
    initialCounters: monitor.counters(),
  };
}

async function clickTransition(context, setup, pointValue, predicate, {
  triggerId,
  recipient,
  applicationRecipient,
  button = "left",
  firstFrame = false,
  doubleClick = false,
}) {
  const before = setup.monitor.latest();
  const beforeCounters = setup.monitor.counters();
  const routeStart = setup.monitor.routes.length;
  if (doubleClick) {
    await context.page.mouse.dblclick(pointValue.x, pointValue.y, { button });
  } else {
    await context.page.mouse.click(pointValue.x, pointValue.y, { button });
  }
  await frames(context.page, 2);
  let after;
  try {
    after = firstFrame
      ? await setup.monitor.waitFirst(predicate, triggerId, before.sequence)
      : await setup.monitor.wait(predicate, triggerId, before.sequence);
  } catch (error) {
    throw new Error(
      `${error.message}; routes=${JSON.stringify(setup.monitor.routes.slice(routeStart))}`,
      { cause: error },
    );
  }
  await frames(context.page, 2);
  const observedRoutes = setup.monitor.routes.slice(routeStart)
    .filter((line) => line.includes(" kind=pointer "));
  const event = routedReceipt(
    triggerId,
    setup.events.length,
    observedRoutes,
    recipient,
    applicationRecipient,
    doubleClick ? 4 : 2,
  );
  const afterCounters = setup.monitor.counters();
  assertReceiptCounterDeltas(triggerId, beforeCounters, afterCounters, event);
  setup.events.push(event);
  return { before, after, event, beforeCounters, afterCounters };
}

async function unchangedInput(context, setup, input, {
  triggerId,
  recipient,
  applicationRecipient,
  kind = "pointer",
}) {
  const before = setup.monitor.latest();
  const beforeCounters = setup.monitor.counters();
  const serialized = JSON.stringify(compact(before));
  const routeStart = setup.monitor.routes.length;
  await input();
  await frames(context.page, 5);
  const after = setup.monitor.latest();
  assert.equal(JSON.stringify(compact(after)), serialized, `${triggerId} unexpectedly changed state`);
  const observedRoutes = setup.monitor.routes.slice(routeStart)
    .filter((line) => line.includes(` kind=${kind} `));
  const event = routedReceipt(
    triggerId,
    setup.events.length,
    observedRoutes,
    recipient,
    applicationRecipient,
    kind === "pointer" ? 2 : 1,
  );
  const afterCounters = setup.monitor.counters();
  assertReceiptCounterDeltas(triggerId, beforeCounters, afterCounters, event);
  setup.events.push(event);
  return { before, after, event, beforeCounters, afterCounters };
}

function pushStep(setup, id, before, after, event, assertions, extra = {}) {
  setup.timeline.push({
    id,
    before: compact(before),
    after: compact(after),
    event,
    assertions,
    ...extra,
  });
}

export function applicationEvents(events) {
  return events.filter(({ observed_actions: observedActions }) => observedActions.length === 1);
}

async function keyTransition(context, setup, key, predicate, {
  triggerId,
  recipient,
  applicationRecipient,
  firstFrame = false,
}) {
  const before = setup.monitor.latest();
  const beforeCounters = setup.monitor.counters();
  const routeStart = setup.monitor.routes.length;
  await context.page.keyboard.press(key);
  await frames(context.page, 2);
  const after = firstFrame
    ? await setup.monitor.waitFirst(predicate, triggerId, before.sequence)
    : await setup.monitor.wait(predicate, triggerId, before.sequence);
  const observedRoutes = setup.monitor.routes.slice(routeStart)
    .filter((line) => line.includes(" kind=key "));
  const event = routedReceipt(
    triggerId,
    setup.events.length,
    observedRoutes,
    recipient,
    applicationRecipient,
    1,
  );
  const afterCounters = setup.monitor.counters();
  assertReceiptCounterDeltas(triggerId, beforeCounters, afterCounters, event);
  setup.events.push(event);
  return { before, after, event, beforeCounters, afterCounters };
}

async function captureEvidenceScreenshot(context, setup, name, clip = undefined) {
  const bytes = await context.page.screenshot({ clip, animations: "disabled" });
  const file = path.join(context.directory, `${name}.png`);
  fs.writeFileSync(file, bytes);
  const digest = sha256(bytes);
  setup.screenshotHashes[name] = digest;
  setup.evidenceScreenshots.push([name, file]);
  return { bytes, digest, file };
}

async function clickReceipt(context, setup, pointValue, {
  triggerId,
  recipient,
  applicationRecipient,
  button = "left",
}) {
  const before = setup.monitor.latest();
  const beforeCounters = setup.monitor.counters();
  const routeStart = setup.monitor.routes.length;
  await context.page.mouse.click(pointValue.x, pointValue.y, { button });
  await frames(context.page, 3);
  const after = setup.monitor.latest();
  const observedRoutes = setup.monitor.routes.slice(routeStart);
  const pointerRoutes = observedRoutes.filter((line) => line.includes(" kind=pointer "));
  const event = routedReceipt(
    triggerId,
    setup.events.length,
    pointerRoutes,
    recipient,
    applicationRecipient,
    2,
  );
  const afterCounters = setup.monitor.counters();
  assertReceiptCounterDeltas(triggerId, beforeCounters, afterCounters, event);
  setup.events.push(event);
  return { before, after, event, beforeCounters, afterCounters };
}

async function openSector(context, setup, systemIndex, triggerId) {
  const current = setup.monitor.latest();
  const system = systemByIndex(current, systemIndex);
  assert.ok(system.map_point, `system ${system.dat_id} has no real map opener`);
  const transition = await clickTransition(
    context,
    setup,
    { x: system.map_point[0], y: system.map_point[1] },
    (state) => systemByDatId(state, system.dat_id).sector_id !== null,
    {
      triggerId,
      recipient: "root",
      applicationRecipient: "sector_opener",
      firstFrame: true,
    },
  );
  return {
    ...transition,
    systemDatId: system.dat_id,
    id: systemByDatId(transition.after, system.dat_id).sector_id,
  };
}

async function openDetail(context, setup, kind, systemIndex, triggerId) {
  let state = setup.monitor.latest();
  const system = systemByIndex(state, systemIndex);
  if (system.sector_id === null) {
    await openSector(context, setup, systemIndex, `${triggerId}-sector`);
    state = setup.monitor.latest();
  }
  const visible = systemByDatId(state, system.dat_id);
  const existingId = detail(state, system.dat_id, kind).id;
  const rect = kind === "system"
    ? visible.planet_rect
    : namedRect(visible.quadrants, kind === "fleet" ? "fleets" : kind);
  assert.ok(rect, `${kind} detail ${system.dat_id} has no real Sector opener`);
  const point = existingId === null
    ? center(rect)
    : uncoveredPoint(rect, [...allDetailRects(state).values()], state.scale);
  const transition = await clickTransition(
    context,
    setup,
    point,
    (candidate) => {
      const opened = detail(candidate, system.dat_id, kind).id;
      return existingId === null
        ? opened !== null
        : opened === existingId && candidate.snapshot.focused?.window === existingId;
    },
    {
      triggerId,
      recipient: `window:${visible.sector_id}`,
      applicationRecipient: "sector_window",
      firstFrame: true,
      doubleClick: true,
    },
  );
  return { ...transition, systemDatId: system.dat_id, id: detail(transition.after, system.dat_id, kind).id };
}

async function minimizeDetail(context, setup, kind, systemIndex, triggerId) {
  let state = setup.monitor.latest();
  const system = systemByIndex(state, systemIndex);
  let shown = detail(state, system.dat_id, kind);
  if (!shown.id || !state.snapshot.paint_order.includes(shown.id)) {
    await openDetail(context, setup, kind, systemIndex, `${triggerId}-open`);
    state = setup.monitor.latest();
    shown = detail(state, system.dat_id, kind);
  }
  const id = shown.id;
  const transition = await clickTransition(
    context,
    setup,
    detailButton(state, system.dat_id, kind, "minimize"),
    (candidate) => candidate.snapshot.history.includes(id) && !candidate.snapshot.paint_order.includes(id),
    {
      triggerId,
      recipient: `window:${id}`,
      applicationRecipient: `${kind}_window`,
    },
  );
  return { ...transition, id, systemDatId: system.dat_id };
}

async function restoreHistoryId(context, setup, id, triggerId) {
  const state = setup.monitor.latest();
  const slot = state.rail.find((candidate) => candidate.id === id);
  assert.ok(slot, `history has no slot for identity ${id}`);
  return clickTransition(
    context,
    setup,
    { x: slot.point[0], y: slot.point[1] },
    (candidate) => candidate.snapshot.paint_order.includes(id)
      && !candidate.snapshot.history.includes(id),
    {
      triggerId,
      recipient: "root",
      applicationRecipient: "reference_rail",
      firstFrame: true,
    },
  );
}

async function writeEvidence(context, setup, observation, assertionCount, scope) {
  await frames(context.page, 3);
  const screenshot = await context.page.screenshot({ animations: "disabled" });
  const screenshotPath = path.join(context.directory, "after.png");
  fs.writeFileSync(screenshotPath, screenshot);
  const wasm = fs.readFileSync(context.fixtureWasm);
  assert.ok(setup.monitor.requests.length > 0, "journey recorded no fixture requests");
  assert.ok(setup.monitor.requests.every(({ status }) => status === 200), JSON.stringify(setup.monitor.requests));
  assert.deepEqual(setup.monitor.errors, []);
  const result = {
    schema_version: WINDOWING_CONTRACT_SCHEMA_VERSION,
    family: "strategic-windowing-contract",
    scope,
    status: "pass",
    run_id: context.runId,
    assertion_count: assertionCount,
    observation,
    ready: setup.ready,
    browser_version: context.page.context().browser().version(),
    viewport: context.viewport,
    faction: context.faction,
    muted: true,
    requests: setup.monitor.requests,
    errors: setup.monitor.errors,
    wasm_sha256: sha256(wasm),
    screenshot_sha256: sha256(screenshot),
  };
  const resultPath = path.join(context.directory, "result.json");
  fs.writeFileSync(resultPath, `${JSON.stringify(result, null, 2)}\n`);
  return {
    schema_version: WINDOWING_CONTRACT_SCHEMA_VERSION,
    diagnostics: { errors: setup.monitor.errors, requests: setup.monitor.requests },
    artifacts: [
      ["result_json", resultPath],
      ["screenshot", screenshotPath],
      ["fixture_wasm", context.fixtureWasm],
      ...setup.evidenceScreenshots.map(([name, file]) => [`screenshot:${name}`, file]),
    ].map(([kind, file]) => ({
      kind,
      path: path.relative(context.evidenceRoot, file),
      sha256: sha256(fs.readFileSync(file)),
    })),
  };
}

function ids(records) {
  return new Set(records.map(({ id }) => id));
}

export function assertIdentityAndRecipientDeltas({ before, after, expected, event }) {
  const beforeIds = ids(before.records);
  const afterIds = ids(after.records);
  for (const id of expected.preserved_ids) {
    assert.ok(beforeIds.has(id), `preserved identity ${id} was absent before the action`);
    assert.ok(afterIds.has(id), `preserved identity ${id} was replaced or removed`);
  }
  assert.deepEqual(after.paint_order, expected.paint_order, "unexpected paint order");
  assert.equal(after.focused, expected.focused, "unexpected focused identity");
  const expectedRecipient = expected.recipient.startsWith("blocked:")
    ? "blocked"
    : expected.recipient;
  assert.equal(event.recipient, expectedRecipient, "unexpected routed recipient");
  assert.equal(
    event.application_recipient,
    expected.application_recipient,
    "unexpected application recipient",
  );
  assert.equal(
    event.observed_actions.length,
    expected.action_delta,
    "unexpected observed application receipt count",
  );
  assert.equal(
    after.counters.routed - before.counters.routed,
    expected.routed_delta,
    "unexpected raw route receipt delta",
  );
  assert.equal(
    after.counters.application - before.counters.application,
    expected.action_delta,
    "unexpected application action delta",
  );
  assert.equal(
    after.counters.underlay - before.counters.underlay,
    expected.underlay_delta,
    "unexpected underlay counter delta",
  );
  assert.equal(event.underlay_action_count, 0, "underlay action leaked from routed event");
  return 10;
}

function assertObservedDelta(transition, {
  preservedIds = [],
  recipient,
  applicationRecipient,
  actionDelta = 1,
  routedDelta = 2,
  underlayDelta = 0,
}) {
  return assertIdentityAndRecipientDeltas({
    before: { ...transition.before.snapshot, counters: transition.beforeCounters },
    after: { ...transition.after.snapshot, counters: transition.afterCounters },
    expected: {
      preserved_ids: preservedIds,
      paint_order: transition.after.snapshot.paint_order,
      focused: transition.after.snapshot.focused,
      recipient,
      application_recipient: applicationRecipient,
      action_delta: actionDelta,
      routed_delta: routedDelta,
      underlay_delta: underlayDelta,
    },
    event: transition.event,
  });
}

export function assertExactHistoryVictim(before, after, inserted) {
  assert.equal(before.history.length, 12, "overflow precondition must contain twelve entries");
  assert.equal(new Set(before.history).size, 12, "pre-overflow history contains a duplicate");
  assert.equal(after.history.length, 12, "history capacity changed after overflow");
  assert.equal(new Set(after.history).size, 12, "post-overflow history contains a duplicate");
  assert.equal(after.history.at(-1), inserted, "the thirteenth entry was not appended");
  const victim = before.history[0];
  assert.ok(!after.history.includes(victim), `oldest victim ${victim} remained in history`);
  assert.ok(!after.registered.includes(victim), `oldest victim ${victim} remained registered`);
  assert.deepEqual(
    after.history,
    [...before.history.slice(1), inserted],
    "overflow removed a history entry other than the exact oldest victim",
  );
  return { victim, retained: after.history, assertions: 8 };
}

async function runJourneyA(context, setup) {
  const initial = setup.monitor.latest();
  const sectorA = systemByIndex(initial, 0);
  const sectorB = systemByIndex(initial, 1);
  const sectorC = systemByIndex(initial, 2);

  const openedA = await openSector(context, setup, 0, "pointer-A01-open-a");
  const idA = systemByDatId(openedA.after, sectorA.dat_id).sector_id;
  const openedB = await openSector(context, setup, 1, "pointer-A01-open-b");
  const idB = systemByDatId(openedB.after, sectorB.dat_id).sector_id;
  assert.deepEqual(relativeOrder(openedB.after, [idA, idB]), [idB, idA]);
  pushStep(setup, "A01", openedA.before, openedB.after, openedB.event, 6, {
    sector_ids: { a: idA, b: idB },
  });

  const duplicate = await unchangedInput(context, setup, () => context.page.mouse.click(
    sectorA.map_point[0],
    sectorA.map_point[1],
  ), {
    triggerId: "pointer-A02-duplicate-a",
    recipient: "root",
    applicationRecipient: "sector_opener",
  });
  assert.equal(systemByDatId(duplicate.after, sectorA.dat_id).sector_id, idA);
  assert.deepEqual(duplicate.after.snapshot.paint_order, duplicate.before.snapshot.paint_order);
  assert.deepEqual(duplicate.after.snapshot.focused, duplicate.before.snapshot.focused);
  assertObservedDelta(duplicate, {
    preservedIds: duplicate.before.snapshot.registered,
    recipient: "root",
    applicationRecipient: "sector_opener",
    underlayDelta: 1,
  });
  pushStep(setup, "A02", duplicate.before, duplicate.after, duplicate.event, 9);

  const activatePoint = systemByDatId(duplicate.after, sectorA.dat_id).sector_title_point;
  assert.ok(activatePoint, "Sector A has no exposed title point");
  const underlayBefore = setup.monitor.underlay;
  const activated = await clickTransition(
    context,
    setup,
    { x: activatePoint[0], y: activatePoint[1] },
    (state) => state.snapshot.focused?.window === idA,
    {
      triggerId: "pointer-A03-activate-a",
      recipient: `window:${idA}`,
      applicationRecipient: "sector_window",
    },
  );
  assert.equal(setup.monitor.underlay, underlayBefore, "Sector activation leaked to the map");
  assert.deepEqual(relativeOrder(activated.after, [idA, idB]), [idB, idA]);
  pushStep(setup, "A03", activated.before, activated.after, activated.event, 5);

  const stateCount = setup.monitor.states.length;
  const openedC = await openSector(context, setup, 2, "pointer-A04-replace-a-with-c");
  assert.equal(systemByDatId(openedC.after, sectorA.dat_id).sector_id, null);
  const idC = systemByDatId(openedC.after, sectorC.dat_id).sector_id;
  assert.notEqual(idC, idA);
  assert.equal(idsOfKind(openedC.after, "sector").length, 2);
  assert.ok(idsOfKind(openedC.after, "sector").includes(idB));
  assert.ok(setup.monitor.states.slice(stateCount).every((state) => idsOfKind(state, "sector").length <= 2));
  pushStep(setup, "A04", openedC.before, openedC.after, openedC.event, 7, {
    replaced: idA,
    replacement: idC,
  });

  // Minimize each opened detail before using the next real Sector opener so a
  // higher detail band cannot cover that opener. Restore the exact identities
  // through the shared rail, then raise Missions to establish the requested
  // initial four-family order.
  const systemOpen = await openDetail(context, setup, "system", 15, "pointer-A05-system");
  await minimizeDetail(context, setup, "system", 15, "pointer-A05-minimize-system");
  const fleetOpen = await openDetail(context, setup, "fleet", 2, "pointer-A05-fleet");
  await minimizeDetail(context, setup, "fleet", 2, "pointer-A05-minimize-fleet");
  const defensesOpen = await openDetail(context, setup, "defenses", 12, "pointer-A05-defenses");
  await minimizeDetail(context, setup, "defenses", 12, "pointer-A05-minimize-defenses");
  const missionsOpen = await openDetail(context, setup, "missions", 13, "pointer-A05-missions");
  await minimizeDetail(context, setup, "missions", 13, "pointer-A05-minimize-missions");
  await restoreHistoryId(context, setup, systemOpen.id, "pointer-A05-restore-system");
  await restoreHistoryId(context, setup, fleetOpen.id, "pointer-A05-restore-fleet");
  await restoreHistoryId(context, setup, defensesOpen.id, "pointer-A05-restore-defenses");
  const detailIds = [systemOpen.id, fleetOpen.id, defensesOpen.id, missionsOpen.id];
  const initialMissionsRaised = await restoreHistoryId(
    context,
    setup,
    missionsOpen.id,
    "pointer-A05-restore-missions",
  );
  assert.equal(new Set(detailIds).size, 4);
  assert.deepEqual(relativeOrder(initialMissionsRaised.after, detailIds), detailIds);
  await captureEvidenceScreenshot(context, setup, "A05-four-family-overlap");
  pushStep(setup, "A05", systemOpen.before, initialMissionsRaised.after, initialMissionsRaised.event, 8, {
    detail_ids: detailIds,
  });

  const missionsDatId = missionsOpen.systemDatId;
  const missionsClose = await clickTransition(
    context,
    setup,
    detailButton(initialMissionsRaised.after, missionsDatId, "missions", "close"),
    (state) => !state.snapshot.registered.includes(missionsOpen.id),
    {
      triggerId: "pointer-A06-close-fourth-detail",
      recipient: `window:${missionsOpen.id}`,
      applicationRecipient: "missions_window",
      firstFrame: true,
    },
  );
  const overlapIds = [systemOpen.id, fleetOpen.id, defensesOpen.id];
  const overlapBefore = missionsClose.after;
  await captureEvidenceScreenshot(context, setup, "A06-overlap-before");
  const overlapEvents = [missionsClose.event];
  for (const [index, target] of [defensesOpen.id, systemOpen.id, fleetOpen.id].entries()) {
    const state = setup.monitor.latest();
    const underlay = setup.monitor.underlay;
    const point = exposedPoint(state, target);
    const family = recordOf(state, target).kind;
    const input = {
        triggerId: `pointer-A06-overlap-${index}`,
        recipient: `window:${target}`,
        applicationRecipient: `${family}_window`,
    };
    const transition = index === 0
      ? await clickReceipt(context, setup, point, input)
      : await clickTransition(
        context,
        setup,
        point,
        (candidate) => relativeOrder(candidate, overlapIds).at(-1) === target,
        { ...input, firstFrame: true },
      );
    if (index === 0) {
      assert.equal(
        JSON.stringify(compact(transition.after)),
        JSON.stringify(compact(transition.before)),
        "clicking the already-focused Defenses tab changed its content",
      );
    }
    assert.equal(
      setup.monitor.underlay,
      underlay,
      `overlap activation ${index} for ${target} at ${JSON.stringify(point)} leaked to the map`,
    );
    assertObservedDelta(transition, {
      preservedIds: overlapIds,
      recipient: `window:${target}`,
      applicationRecipient: `${family}_window`,
    });
    overlapEvents.push(transition.event);
  }
  const overlapAfter = setup.monitor.latest();
  await captureEvidenceScreenshot(context, setup, "A06-overlap-after");
  assert.deepEqual(relativeOrder(overlapAfter, overlapIds), [
    defensesOpen.id,
    systemOpen.id,
    fleetOpen.id,
  ]);
  pushStep(setup, "A06", overlapBefore, overlapAfter, overlapEvents.at(-1), 6, {
    overlap_events: overlapEvents,
  });

  const permutationIds = [systemOpen.id, fleetOpen.id, defensesOpen.id];
  const permutations = [
    permutationIds,
    [defensesOpen.id, systemOpen.id, fleetOpen.id],
    [fleetOpen.id, defensesOpen.id, systemOpen.id],
  ];
  const permutationClicks = [
    permutationIds,
    [systemOpen.id, fleetOpen.id],
    [defensesOpen.id, systemOpen.id],
  ];
  const originalViewport = context.page.viewportSize();
  assert.ok(originalViewport, "browser context has no viewport");
  await context.page.setViewportSize({
    width: originalViewport.width + (context.viewport === "native" ? 160 : -160),
    height: originalViewport.height + (context.viewport === "native" ? 120 : -80),
  });
  await frames(context.page, 5);
  const resizedState = setup.monitor.latest();
  const permutationEvents = [];
  for (const [permutationIndex, permutation] of permutations.entries()) {
    for (const target of permutationClicks[permutationIndex]) {
      const state = setup.monitor.latest();
      const family = recordOf(state, target).kind;
      const transition = await clickTransition(
        context,
        setup,
        exposedPoint(state, target),
        (candidate) => relativeOrder(candidate, permutationIds).at(-1) === target,
        {
          triggerId: `pointer-A07-${permutationIndex}-${target}`,
          recipient: `window:${target}`,
          applicationRecipient: `${family}_window`,
          firstFrame: true,
        },
      );
      assertObservedDelta(transition, {
        preservedIds: permutationIds,
        recipient: `window:${target}`,
        applicationRecipient: `${family}_window`,
      });
      permutationEvents.push(transition.event);
    }
    assert.deepEqual(relativeOrder(setup.monitor.latest(), permutationIds), permutation);
    await captureEvidenceScreenshot(context, setup, `A07-permutation-${permutationIndex + 1}`);
  }
  await context.page.setViewportSize(originalViewport);
  await frames(context.page, 5);
  pushStep(setup, "A07", overlapAfter, setup.monitor.latest(), permutationEvents.at(-1), 9, {
    permutations,
    resized_scale: resizedState.scale,
    resized_viewport: context.viewport,
    permutation_events: permutationEvents,
  });

  const beforeReopen = setup.monitor.latest();
  const fleetContent = JSON.stringify(detail(beforeReopen, fleetOpen.systemDatId, "fleet").selection);
  const fleetSystemIndex = beforeReopen.systems.findIndex(
    ({ dat_id: datId }) => datId === fleetOpen.systemDatId,
  );
  const reopened = await openDetail(
    context,
    setup,
    "fleet",
    fleetSystemIndex,
    "pointer-A08-reopen-fleet",
  );
  assert.equal(detail(reopened.after, fleetOpen.systemDatId, "fleet").id, fleetOpen.id);
  assert.equal(JSON.stringify(detail(reopened.after, fleetOpen.systemDatId, "fleet").selection), fleetContent);
  pushStep(setup, "A08", beforeReopen, reopened.after, reopened.event, 6);

  const sectorOrigin = systemByDatId(reopened.after, systemOpen.systemDatId);
  const sectorOriginId = sectorOrigin.sector_id;
  assert.ok(sectorOriginId, "Fleet navigation origin Sector is not open");
  const rootOwnedOriginDetails = reopened.after.systems
    .filter(({ sector_id: sectorId }) => sectorId === sectorOriginId)
    .flatMap(({ details }) => Object.values(details).map(({ id }) => id).filter(Boolean))
    .filter((id) => {
      const record = recordOf(reopened.after, id);
      return record?.owner === "root" && record.visible;
    });
  assert.ok(rootOwnedOriginDetails.length >= 2, "Sector origin has fewer than two visible root-owned details");
  assert.ok(rootOwnedOriginDetails.includes(systemOpen.id), "origin System is not root-owned");
  assert.ok(rootOwnedOriginDetails.includes(defensesOpen.id), "origin Defenses is not root-owned");
  const sectorClosePoint = sectorOrigin.sector_close_point;
  const closedSector = await clickTransition(
    context,
    setup,
    { x: sectorClosePoint[0] + 6 * reopened.after.scale, y: sectorClosePoint[1] },
    (state) => !state.snapshot.registered.includes(sectorOriginId),
    {
      triggerId: "pointer-A09-close-sector-origin",
      recipient: `window:${sectorOriginId}`,
      applicationRecipient: "sector_window",
    },
  );
  for (const detailId of rootOwnedOriginDetails) {
    assert.ok(closedSector.after.snapshot.registered.includes(detailId), `root-owned detail ${detailId} was torn down`);
    assert.ok(closedSector.after.snapshot.paint_order.includes(detailId), `root-owned detail ${detailId} was hidden`);
    assert.equal(recordOf(closedSector.after, detailId).owner, "root", `detail ${detailId} owner changed`);
  }
  pushStep(setup, "A09", closedSector.before, closedSector.after, closedSector.event, 5, {
    owner_contract: "strategic-root",
    preserved_root_details: rootOwnedOriginDetails,
  });

  const fleetEntry = detail(closedSector.after, fleetOpen.systemDatId, "fleet").entry_rect;
  assert.ok(fleetEntry, "Fleet detail has no real row for its owned menu");
  const menuOpened = await clickTransition(
    context,
    setup,
    center(fleetEntry),
    (state) => state.snapshot.records.some((record) => record.kind === "menu" && record.owner === fleetOpen.id),
    {
      triggerId: "pointer-A10-open-owned-menu",
      recipient: `window:${fleetOpen.id}`,
      applicationRecipient: "fleet_window",
      button: "right",
    },
  );
  const menuId = menuOpened.after.snapshot.records.find(
    (record) => record.kind === "menu" && record.owner === fleetOpen.id,
  ).id;
  const fleetClosePoint = detailButton(menuOpened.after, fleetOpen.systemDatId, "fleet", "close");
  const childClosed = await clickTransition(
    context,
    setup,
    fleetClosePoint,
    (state) => !state.snapshot.registered.includes(menuId),
    {
      triggerId: "pointer-A10-dismiss-child-first",
      recipient: `blocked:${menuId}`,
      applicationRecipient: "managed_menu",
    },
  );
  assert.ok(childClosed.after.snapshot.registered.includes(fleetOpen.id));
  const ownerClosed = await clickTransition(
    context,
    setup,
    fleetClosePoint,
    (state) => !state.snapshot.registered.includes(fleetOpen.id),
    {
      triggerId: "pointer-A10-close-owner",
      recipient: `window:${fleetOpen.id}`,
      applicationRecipient: "fleet_window",
    },
  );
  assert.ok(ownerClosed.after.snapshot.focused === null
    || ownerClosed.after.snapshot.registered.includes(ownerClosed.after.snapshot.focused.window));
  pushStep(setup, "A10", menuOpened.before, ownerClosed.after, ownerClosed.event, 8, {
    child_id: menuId,
    child_close: compact(childClosed.after),
  });

  return {
    schema_version: WINDOWING_CONTRACT_SCHEMA_VERSION,
    case_id: "journey-a",
    faction: context.faction,
    viewport: context.viewport,
    events: applicationEvents(setup.events),
    action_counters: {
      before: setup.initialCounters,
      after: setup.monitor.counters(),
    },
    conditions: ["cold", "warm", "resize"],
    timeline: setup.timeline,
  };
}

function namedRect(entries, name) {
  const entry = entries.find(([candidate]) => candidate === name);
  assert.ok(entry, `missing ${name} control rectangle`);
  return entry[1];
}

async function runJourneyB(context, setup) {
  const initial = setup.monitor.latest();
  const primary = systemByIndex(initial, 0);
  const defensesOpen = await openDetail(context, setup, "defenses", 0, "pointer-B01-open-defenses");
  let state = defensesOpen.after;
  const regiments = namedRect(detail(state, primary.dat_id, "defenses").tabs, "regiments");
  const changedTab = await clickTransition(
    context,
    setup,
    center(regiments),
    (candidate) => detail(candidate, primary.dat_id, "defenses").selection?.page === "regiments",
    {
      triggerId: "pointer-B01-regiments-tab",
      recipient: `window:${defensesOpen.id}`,
      applicationRecipient: "defenses_window",
    },
  );
  state = changedTab.after;
  const cellRect = detail(state, primary.dat_id, "defenses").cell_rect;
  assert.ok(cellRect, "stocked Defenses fixture has no regiment row");
  const selected = await clickTransition(
    context,
    setup,
    center(cellRect),
    (candidate) => detail(candidate, primary.dat_id, "defenses").selection?.selected === 0,
    {
      triggerId: "pointer-B01-select-regiment",
      recipient: `window:${defensesOpen.id}`,
      applicationRecipient: "defenses_window",
    },
  );
  const windowRect = detail(selected.after, primary.dat_id, "defenses").rect;
  const clip = { x: windowRect[0], y: windowRect[1], width: windowRect[2], height: windowRect[3] };
  const beforeScrollEvidence = await captureEvidenceScreenshot(
    context,
    setup,
    "B01-defenses-before-scroll",
    clip,
  );
  const beforeScroll = beforeScrollEvidence.bytes;
  const scrollTransition = await clickTransition(
    context,
    setup,
    {
      x: windowRect[0] + 224 * selected.after.scale,
      y: windowRect[1] + 286 * selected.after.scale,
    },
    (candidate) => detail(candidate, primary.dat_id, "defenses").selection?.first_row === 1,
    {
      triggerId: "pointer-B01-scroll-down",
      recipient: `window:${defensesOpen.id}`,
      applicationRecipient: "defenses_window",
    },
  );
  const afterScrollEvidence = await captureEvidenceScreenshot(
    context,
    setup,
    "B01-defenses-after-scroll",
    clip,
  );
  const afterScroll = afterScrollEvidence.bytes;
  assert.notEqual(
    sha256(afterScroll),
    sha256(beforeScroll),
    `real wheel input did not change Defenses pixels: ${JSON.stringify(detail(setup.monitor.latest(), primary.dat_id, "defenses").selection)}`,
  );
  setup.screenshotHashes.defenses_scrolled = sha256(afterScroll);
  const stableDefensesContent = JSON.stringify(
    detail(setup.monitor.latest(), primary.dat_id, "defenses").selection,
  );
  pushStep(setup, "B01", defensesOpen.before, setup.monitor.latest(), scrollTransition.event, 8, {
    before_scroll_sha256: sha256(beforeScroll),
    after_scroll_sha256: sha256(afterScroll),
    stable_content: detail(setup.monitor.latest(), primary.dat_id, "defenses").selection,
  });

  const minimizedDefenses = await minimizeDetail(context, setup, "defenses", 0, "pointer-B02-minimize-defenses");
  assert.deepEqual(minimizedDefenses.after.snapshot.history, [defensesOpen.id]);
  assert.ok(!minimizedDefenses.after.snapshot.paint_order.includes(defensesOpen.id));
  const defensesSlot = minimizedDefenses.after.rail.find(({ id }) => id === defensesOpen.id);
  assert.ok(defensesSlot, "modified Defenses history entry has no rail slot");
  const restoredDefenses = await clickTransition(
    context,
    setup,
    { x: defensesSlot.point[0], y: defensesSlot.point[1] },
    (candidate) => candidate.snapshot.paint_order.includes(defensesOpen.id)
      && !candidate.snapshot.history.includes(defensesOpen.id),
    {
      triggerId: "pointer-B02-restore-defenses",
      recipient: "root",
      applicationRecipient: "reference_rail",
      firstFrame: true,
    },
  );
  assert.equal(detail(restoredDefenses.after, primary.dat_id, "defenses").id, defensesOpen.id);
  assert.equal(
    JSON.stringify(detail(restoredDefenses.after, primary.dat_id, "defenses").selection),
    stableDefensesContent,
    "Defenses tab, scroll, or selected row changed across minimize/restore",
  );
  const reminimizedDefenses = await minimizeDetail(
    context,
    setup,
    "defenses",
    0,
    "pointer-B02-reminimize-defenses",
  );
  assert.deepEqual(reminimizedDefenses.after.snapshot.history, [defensesOpen.id]);
  pushStep(setup, "B02", minimizedDefenses.before, reminimizedDefenses.after, restoredDefenses.event, 10, {
    restored_content: JSON.parse(stableDefensesContent),
  });

  const minimizedFleet = await minimizeDetail(context, setup, "fleet", 1, "pointer-B03-minimize-fleet");
  const stableFleetContent = JSON.stringify(
    detail(minimizedFleet.before, minimizedFleet.systemDatId, "fleet").selection,
  );
  const minimizedSystem = await minimizeDetail(context, setup, "system", 2, "pointer-B03-minimize-system");
  const stableSystemContent = JSON.stringify(
    detail(minimizedSystem.before, minimizedSystem.systemDatId, "system").selection,
  );
  const minimizedMissions = await minimizeDetail(context, setup, "missions", 0, "pointer-B03-minimize-missions");
  assert.deepEqual(minimizedMissions.after.snapshot.history, [
    defensesOpen.id,
    minimizedFleet.id,
    minimizedSystem.id,
    minimizedMissions.id,
  ]);
  pushStep(setup, "B03", minimizedFleet.before, minimizedMissions.after, minimizedMissions.event, 7, {
    shared_history: minimizedMissions.after.snapshot.history,
  });

  const restoreState = setup.monitor.latest();
  const middleSlot = restoreState.rail.find(({ id }) => id === minimizedFleet.id);
  assert.ok(middleSlot, "middle Fleet history entry has no rail slot");
  const restoredMiddle = await clickTransition(
    context,
    setup,
    { x: middleSlot.point[0], y: middleSlot.point[1] },
    (candidate) => candidate.snapshot.paint_order.includes(minimizedFleet.id)
      && !candidate.snapshot.history.includes(minimizedFleet.id),
    {
      triggerId: "pointer-B04-restore-middle",
      recipient: "root",
      applicationRecipient: "reference_rail",
      firstFrame: true,
    },
  );
  assert.equal(detail(restoredMiddle.after, minimizedFleet.systemDatId, "fleet").id, minimizedFleet.id);
  assert.equal(restoredMiddle.after.snapshot.focused?.window, minimizedFleet.id);
  assert.equal(
    JSON.stringify(detail(restoredMiddle.after, minimizedFleet.systemDatId, "fleet").selection),
    stableFleetContent,
    "Fleet content changed across middle-history restore",
  );
  assert.deepEqual(restoredMiddle.after.snapshot.history, [
    defensesOpen.id,
    minimizedSystem.id,
    minimizedMissions.id,
  ]);
  pushStep(setup, "B04", restoredMiddle.before, restoredMiddle.after, restoredMiddle.event, 8);

  const systemIndex = restoredMiddle.after.systems.findIndex(
    ({ dat_id: datId }) => datId === minimizedSystem.systemDatId,
  );
  const reopenedSystem = await openDetail(
    context,
    setup,
    "system",
    systemIndex,
    "pointer-B05-reopen-minimized-system",
  );
  assert.equal(detail(reopenedSystem.after, minimizedSystem.systemDatId, "system").id, minimizedSystem.id);
  assert.equal(reopenedSystem.after.snapshot.focused?.window, minimizedSystem.id);
  assert.ok(
    !reopenedSystem.after.snapshot.history.includes(minimizedSystem.id),
    "normal opener left restored System in presentation history",
  );
  assert.equal(
    JSON.stringify(detail(reopenedSystem.after, minimizedSystem.systemDatId, "system").selection),
    stableSystemContent,
    "System content changed across normal-opener restore",
  );
  pushStep(setup, "B05", reopenedSystem.before, reopenedSystem.after, reopenedSystem.event, 6);

  await minimizeDetail(context, setup, "system", 2, "pointer-B06-reminimize-system");
  await minimizeDetail(context, setup, "fleet", 1, "pointer-B06-reminimize-fleet");
  await openSector(context, setup, 0, "pointer-B06-open-history-sector");
  for (let systemIndex = 4; systemIndex <= 11; systemIndex += 1) {
    await minimizeDetail(context, setup, "system", systemIndex, `pointer-B06-fill-${systemIndex}`);
  }
  const beforeOverflow = setup.monitor.latest();
  assert.equal(beforeOverflow.snapshot.history.length, 12);
  assert.equal(new Set(beforeOverflow.snapshot.history).size, 12);
  const thirteenth = await minimizeDetail(context, setup, "system", 12, "pointer-B06-thirteenth");
  assert.equal(thirteenth.after.snapshot.history.length, 12);
  pushStep(setup, "B06", beforeOverflow, thirteenth.after, thirteenth.event, 6, {
    inserted: thirteenth.id,
  });

  const eviction = assertExactHistoryVictim(
    {
      history: beforeOverflow.snapshot.history,
      registered: beforeOverflow.snapshot.registered,
    },
    {
      history: thirteenth.after.snapshot.history,
      registered: thirteenth.after.snapshot.registered,
    },
    thirteenth.id,
  );
  assert.equal(eviction.victim, defensesOpen.id);
  assert.equal(detail(thirteenth.after, primary.dat_id, "defenses").id, null);
  pushStep(setup, "B07", beforeOverflow, thirteenth.after, thirteenth.event, eviction.assertions, {
    eviction,
  });

  const beforeModal = setup.monitor.latest();
  assert.ok(beforeModal.fleet_finder_point, "fixture omitted the Fleet Finder cockpit control");
  const openedModal = await clickTransition(
    context,
    setup,
    { x: beforeModal.fleet_finder_point[0], y: beforeModal.fleet_finder_point[1] },
    (candidate) => candidate.snapshot.active_modal !== null
      && recordOf(candidate, candidate.snapshot.active_modal)?.kind === "fleet-finder",
    {
      triggerId: "pointer-B08-open-fleet-finder",
      recipient: "root",
      applicationRecipient: "fleet_finder",
      firstFrame: true,
    },
  );
  const modalState = openedModal.after;
  const modalId = modalState.snapshot.active_modal;
  const modalEvent = openedModal.event;
  const blockedSlot = modalState.rail[0];
  assert.ok(blockedSlot, "full history has no rail slot under modal");
  const blocked = await unchangedInput(context, setup, () => context.page.mouse.click(
    blockedSlot.point[0],
    blockedSlot.point[1],
  ), {
    triggerId: "pointer-B08-blocked-rail",
    recipient: `blocked:${modalId}`,
    applicationRecipient: "none",
  });
  assert.deepEqual(blocked.after.snapshot.history, modalState.snapshot.history);
  assert.equal(blocked.after.snapshot.active_modal, modalId);
  assertObservedDelta(blocked, {
    preservedIds: modalState.snapshot.registered,
    recipient: `blocked:${modalId}`,
    applicationRecipient: "none",
    actionDelta: 0,
  });
  pushStep(setup, "B08", beforeModal, blocked.after, blocked.event, 8, {
    modal_open_event: modalEvent,
  });

  const dismissedTransition = await keyTransition(
    context,
    setup,
    "Escape",
    (candidate) => candidate.snapshot.active_modal === null,
    {
      triggerId: "key-B09-dismiss-fleet-finder",
      recipient: `window:${modalId}`,
      applicationRecipient: "fleet_finder",
      firstFrame: true,
    },
  );
  const beforeDismiss = dismissedTransition.before;
  const dismissed = dismissedTransition.after;
  const dismissEvent = dismissedTransition.event;
  const restoreSlot = dismissed.rail.find(({ id }) => id === blockedSlot.id);
  assert.ok(restoreSlot, "blocked history entry disappeared with modal dismissal");
  const freshRestore = await clickTransition(
    context,
    setup,
    { x: restoreSlot.point[0], y: restoreSlot.point[1] },
    (candidate) => candidate.snapshot.paint_order.includes(restoreSlot.id)
      && !candidate.snapshot.history.includes(restoreSlot.id),
    {
      triggerId: "pointer-B09-fresh-rail-restore",
      recipient: "root",
      applicationRecipient: "reference_rail",
      firstFrame: true,
    },
  );
  assert.equal(freshRestore.after.snapshot.focused?.window, restoreSlot.id);
  assert.equal(
    freshRestore.before.snapshot.history.length - freshRestore.after.snapshot.history.length,
    1,
  );
  pushStep(setup, "B09", beforeDismiss, freshRestore.after, freshRestore.event, 8, {
    dismiss_event: dismissEvent,
  });

  const beforeResize = setup.monitor.latest();
  const viewport = context.page.viewportSize();
  assert.ok(viewport);
  await context.page.setViewportSize({ width: viewport.width + 96, height: viewport.height + 64 });
  await frames(context.page, 5);
  const resized = setup.monitor.latest();
  assert.deepEqual(resized.snapshot.registered, beforeResize.snapshot.registered);
  assert.deepEqual(resized.snapshot.history, beforeResize.snapshot.history);
  await context.page.setViewportSize(viewport);
  await frames(context.page, 4);
  pushStep(setup, "B10", beforeResize, setup.monitor.latest(), freshRestore.event, 6, {
    conditions: ["cold", "warm", "resize"],
    resized_scale: resized.scale,
  });

  return {
    schema_version: WINDOWING_CONTRACT_SCHEMA_VERSION,
    case_id: "journey-b",
    faction: context.faction,
    viewport: context.viewport,
    events: applicationEvents(setup.events),
    action_counters: {
      before: setup.initialCounters,
      after: setup.monitor.counters(),
    },
    conditions: ["cold", "warm", "resize"],
    screenshot_hashes: setup.screenshotHashes,
    timeline: setup.timeline,
  };
}

function assertJourney(context, observation, setup) {
  assert.equal(observation.timeline.length, 10);
  assert.deepEqual(observation.timeline.map(({ id }) => id), setup.timeline.map(({ id }) => id));
  assert.ok(observation.events.length > 0);
  assert.ok(observation.events.every(({ observed_actions }) => observed_actions.length === 1));
  assert.deepEqual(observation.conditions, ["cold", "warm", "resize"]);
  let assertions = 0;
  for (const step of observation.timeline) {
    context.completeStep(step.id, step.assertions);
    assertions += step.assertions;
  }
  return assertions;
}

function namespacedEvents(events, journey) {
  return events.map((event) => ({
    ...event,
    event_id: `${journey}:${event.event_id}`,
  }));
}

async function setupIdentityHistory(context) {
  return {
    journeyA: await setupJourney(context),
    journeyB: null,
  };
}

async function runIdentityHistory(context, setup) {
  const journeyAObservation = await runJourneyA(context, setup.journeyA);
  setup.journeyB = await setupJourney(context);
  const journeyBObservation = await runJourneyB(context, setup.journeyB);
  const events = [
    ...namespacedEvents(journeyAObservation.events, "journey-a"),
    ...namespacedEvents(journeyBObservation.events, "journey-b"),
  ];
  return {
    schema_version: WINDOWING_CONTRACT_SCHEMA_VERSION,
    case_id: "identity-history",
    faction: context.faction,
    viewport: context.viewport,
    events,
    action_counters: {
      before: { application: 0, routed: 0, underlay: 0 },
      after: {
        application: journeyAObservation.action_counters.after.application
          + journeyBObservation.action_counters.after.application,
        routed: journeyAObservation.action_counters.after.routed
          + journeyBObservation.action_counters.after.routed,
        underlay: setup.journeyA.monitor.underlay + setup.journeyB.monitor.underlay,
      },
    },
    conditions: ["cold", "warm", "resize"],
    screenshot_hashes: {
      journey_a: journeyAObservation.screenshot_hashes,
      journey_b: journeyBObservation.screenshot_hashes,
    },
    timeline: [...journeyAObservation.timeline, ...journeyBObservation.timeline],
  };
}

function assertIdentityHistory(context, observation, setup) {
  const expectedTimeline = [...setup.journeyA.timeline, ...setup.journeyB.timeline];
  assert.equal(observation.timeline.length, 20);
  assert.deepEqual(
    observation.timeline.map(({ id }) => id),
    expectedTimeline.map(({ id }) => id),
  );
  assert.ok(observation.events.length > 0);
  assert.equal(new Set(observation.events.map(({ event_id }) => event_id)).size, observation.events.length);
  assert.ok(observation.events.every(({ observed_actions }) => observed_actions.length === 1));
  let assertions = 0;
  for (const step of observation.timeline) {
    context.completeStep(step.id, step.assertions);
    assertions += step.assertions;
  }
  return assertions;
}

function identityHistoryEvidence(context, observation, setup) {
  return writeEvidence(
    context,
    {
      monitor: {
        requests: [...setup.journeyA.monitor.requests, ...setup.journeyB.monitor.requests],
        errors: [...setup.journeyA.monitor.errors, ...setup.journeyB.monitor.errors],
      },
      ready: {
        journey_a: setup.journeyA.ready,
        journey_b: setup.journeyB.ready,
      },
      screenshotHashes: observation.screenshot_hashes,
      evidenceScreenshots: [
        ...setup.journeyA.evidenceScreenshots,
        ...setup.journeyB.evidenceScreenshots,
      ],
    },
    observation,
    observation.timeline.reduce((sum, step) => sum + step.assertions, 0),
    "focused fixture journeys A-B; not complete A-E acceptance",
  );
}

const APPLICABLE_FACTIONS = ["alliance", "empire"];
const APPLICABLE_VIEWPORTS = ["native", "letterboxed"];

const journeyA = defineWindowingContractCase({
  id: "journey-a",
  groups: ["A"],
  steps: Array.from({ length: 10 }, (_, index) => `A${String(index + 1).padStart(2, "0")}`),
  factions: APPLICABLE_FACTIONS,
  viewports: APPLICABLE_VIEWPORTS,
  setup: setupJourney,
  actions: runJourneyA,
  assertions: assertJourney,
  evidence: (context, observation, setup) => writeEvidence(
    context,
    setup,
    observation,
    observation.timeline.reduce((sum, step) => sum + step.assertions, 0),
    "focused fixture journey A; not complete A-E acceptance",
  ),
});

const journeyB = defineWindowingContractCase({
  id: "journey-b",
  groups: ["B"],
  steps: Array.from({ length: 10 }, (_, index) => `B${String(index + 1).padStart(2, "0")}`),
  factions: APPLICABLE_FACTIONS,
  viewports: APPLICABLE_VIEWPORTS,
  setup: setupJourney,
  actions: runJourneyB,
  assertions: assertJourney,
  evidence: (context, observation, setup) => writeEvidence(
    context,
    setup,
    observation,
    observation.timeline.reduce((sum, step) => sum + step.assertions, 0),
    "focused fixture journey B; not complete A-E acceptance",
  ),
});

const identityHistory = defineWindowingContractCase({
  id: "identity-history",
  groups: ["A", "B"],
  steps: [
    ...Array.from({ length: 10 }, (_, index) => `A${String(index + 1).padStart(2, "0")}`),
    ...Array.from({ length: 10 }, (_, index) => `B${String(index + 1).padStart(2, "0")}`),
  ],
  factions: APPLICABLE_FACTIONS,
  viewports: APPLICABLE_VIEWPORTS,
  setup: setupIdentityHistory,
  actions: runIdentityHistory,
  assertions: assertIdentityHistory,
  evidence: identityHistoryEvidence,
});

export const windowingContractCases = Object.freeze([journeyA, journeyB, identityHistory]);
