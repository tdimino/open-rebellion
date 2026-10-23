import assert from "node:assert/strict";
import fs from "node:fs";
import { createHash } from "node:crypto";
import path from "node:path";

import {
  WINDOWING_CONTRACT_SCHEMA_VERSION,
  defineWindowingContractCase,
} from "./windowing-contract.mjs";
import { clickWithRouteBarrier } from "./fleet-window.mjs";

const FACTIONS = Object.freeze(["alliance", "empire"]);
const VIEWPORTS = Object.freeze(["native", "letterboxed"]);
const D_STEPS = Object.freeze(Array.from(
  { length: 10 },
  (_, index) => `D${String(index + 1).padStart(2, "0")}`,
));
const E_STEPS = Object.freeze(Array.from(
  { length: 10 },
  (_, index) => `E${String(index + 1).padStart(2, "0")}`,
));

function sameIds(actual, expected, description) {
  assert.deepEqual(actual, expected, description);
  return 1;
}

function nonOverlapping(first, second) {
  return first[0] + first[2] <= second[0]
    || second[0] + second[2] <= first[0]
    || first[1] + first[3] <= second[1]
    || second[1] + second[3] <= first[1];
}

export function assertChildConsumptionAndGestureCancellation(value) {
  const steps = {};
  const { search } = value;
  assert.ok(search, "journey D omitted focused search evidence");
  assert.equal(search.kind, "key");
  assert.equal(search.recipient, `window:${search.after.focused.window}`);
  steps.D01 = 3;
  sameIds(search.after.registered, search.before.registered, "search changed manager IDs");
  assert.equal(search.after.record_count, search.before.record_count, "search changed record count");
  assert.equal(search.after.root_actions, search.before.root_actions, "search ran a root action");
  assert.notDeepEqual(search.after.focused, search.before.focused, "search navigation did not change child focus");
  steps.D02 = 4;

  const { renameCommit, renameCancel } = value;
  assert.ok(renameCommit && renameCancel, "journey D omitted Rename commit/cancel evidence");
  assert.equal(renameCommit.kind, "key");
  assert.equal(renameCommit.recipient, `window:${renameCommit.window}`);
  assert.equal(renameCommit.after.rename_commits, 1, "Rename Enter did not commit exactly once");
  assert.equal(renameCommit.after.rename_open, false, "Rename Enter left the editor open");
  assert.equal(renameCommit.after.record_count, renameCommit.before.record_count, "Rename created a top-level record");
  assert.equal(renameCancel.kind, "key");
  assert.equal(renameCancel.recipient, `window:${renameCancel.window}`);
  assert.equal(renameCancel.after.rename_commits, renameCancel.before.rename_commits, "Rename Escape committed text");
  assert.equal(renameCancel.after.rename_open, false, "Rename Escape left the editor open");
  assert.equal(renameCancel.after.record_count, renameCancel.before.record_count, "Rename Escape changed record count");
  steps.D03 = 11;

  const detail = value.detailReturn;
  assert.ok(detail, "journey D omitted detail raise/return evidence");
  assert.notEqual(detail.raised_id, detail.returned_id, "another detail was not raised");
  assert.equal(detail.returned_id, detail.before.id, "return allocated a replacement detail identity");
  assert.deepEqual(detail.after.selection, detail.before.selection, "return changed detail content state");
  assert.ok(detail.after.registered.includes(detail.returned_id), "returned detail is no longer registered");
  steps.D04 = 5;

  const { menu } = value;
  assert.ok(menu, "journey D omitted Menu hit-union evidence");
  assert.equal(menu.rects.length, 2, "Menu did not expose root and submenu rectangles");
  assert.ok(nonOverlapping(menu.rects[0], menu.rects[1]), "Menu root and submenu overlap");
  assert.deepEqual(menu.hit_recipients, [menu.id, menu.id], "Menu hit regions used different IDs");
  assert.equal(menu.owner, menu.expected_owner, "Menu owner changed unexpectedly");
  steps.D05 = 5;
  assert.equal(menu.outside_recipient, "blocked");
  assert.equal(menu.dismissed_registered_after, false, "outside press retained the blocking Menu identity");
  assert.equal(menu.root_actions_after, menu.root_actions_before, "outside Menu dismissal ran cockpit input");
  steps.D06 = 3;

  const replacement = value.menuReplacement;
  assert.ok(replacement, "journey D omitted Menu replacement/owner-close evidence");
  assert.equal(replacement.injected, true, "Menu replacement injection was not labelled");
  assert.ok(replacement.new_id > replacement.old_id, "Menu replacement reused its old identity");
  assert.equal(replacement.old_owner, replacement.expected_owner, "old Menu had the wrong owner");
  assert.equal(replacement.new_owner, replacement.expected_owner, "replacement Menu changed owner");
  assert.equal(replacement.old_registered_after_replace, false, "Menu replacement retained the old identity");
  assert.equal(replacement.new_registered_after_replace, true, "Menu replacement did not publish the new identity");
  assert.equal(replacement.child_registered_after_dismiss, false, "owner press retained the child Menu");
  assert.equal(replacement.owner_registered_after_close, false, "second press did not close the owner");
  assert.equal(replacement.focus_after?.window ?? null, replacement.expected_focus,
    "owner close did not restore the exact prior focus fallback");
  steps.D07 = 10;

  const { capturedRelease } = value;
  assert.ok(capturedRelease, "journey D omitted captured-release evidence");
  assert.equal(capturedRelease.release_recipient, `window:${capturedRelease.owner}`);
  assert.equal(capturedRelease.capture_after, null);
  assert.equal(capturedRelease.payload_after, false);
  assert.equal(capturedRelease.drop_delta, 0, "release outside a destination emitted a drop action");
  assert.equal(capturedRelease.underlay_delta, 0);
  steps.D08 = 6;

  const expectedInterruptions = ["modal", "close", "minimize", "scene", "focus-loss"];
  assert.deepEqual(value.interruptions.map(({ kind }) => kind), expectedInterruptions);
  for (const interruption of value.interruptions) {
    assert.equal(interruption.captured_owner_id, interruption.capture_before,
      `${interruption.kind} cancellation identity changed`);
    assert.equal(interruption.capture_after, null, `${interruption.kind} retained capture`);
    assert.equal(interruption.payload_after, false, `${interruption.kind} retained drag payload`);
    assert.equal(interruption.release_recipient, "blocked", `${interruption.kind} orphan release escaped`);
    assert.equal(interruption.drop_delta, 0, `${interruption.kind} delivered an orphan drop`);
    assert.equal(interruption.underlay_delta, 0, `${interruption.kind} activated the underlay`);
    if (interruption.kind !== "modal") {
      assert.equal(interruption.injected, true, `${interruption.kind} injection was not labelled`);
    }
  }
  assert.equal(value.interruptions[0].injected, false, "legitimate modal interruption was labelled as injected");
  steps.D09 = 6;
  steps.D10 = 30;
  assert.deepEqual(Object.keys(steps), D_STEPS);
  return steps;
}

export function assertFailClosedSceneAndReset(value) {
  const steps = {};
  const modal = value.unpaintableModal;
  assert.ok(modal, "journey E omitted unpaintable modal evidence");
  assert.equal(modal.forced_status, "unpaintable");
  steps.E01 = 2;
  assert.ok(modal.registered.includes(modal.id), "unpaintable modal was destroyed");
  assert.equal(modal.active_modal, modal.id, "unpaintable modal stopped blocking");
  assert.deepEqual(modal.blocking_unpainted, [modal.id]);
  assert.equal(modal.recovery_target, modal.id, "unpaintable modal lost its close/recovery route");
  assert.equal(modal.outside_recipient, "blocked");
  steps.E02 = 5;

  const modeless = value.unpaintableModeless;
  assert.ok(modeless?.registered.includes(modeless.id), "unpaintable modeless content was destroyed");
  assert.deepEqual(modeless.removed, []);
  steps.E03 = 2;
  const absent = value.absentModeless;
  assert.ok(!absent?.registered.includes(absent.id), "explicitly absent modeless content remained registered");
  assert.deepEqual(absent.removed, [absent.id]);
  assert.equal(absent.removal_count, 1, "explicit absence did not tear down exactly once");
  steps.E04 = 3;

  const tactical = value.tactical;
  assert.ok(tactical, "journey E omitted tactical scene evidence");
  assert.equal(tactical.dormant_galaxy_registered, true, "Galaxy modal did not remain dormant and registered");
  steps.E05 = 2;
  assert.equal(tactical.combat_recipient, "tactical", "dormant Galaxy modal blocked battle input");
  assert.equal(tactical.combat_action, "toggle_pause");
  assert.notEqual(tactical.paused_before, tactical.paused_after, "battle input produced no tactical state delta");
  steps.E06 = 3;
  assert.equal(tactical.alert_outside_recipient, "blocked");
  assert.equal(tactical.results_outside_recipient, "blocked");
  steps.E07 = 2;
  assert.equal(tactical.dismissal_underlay_delta, 0, "tactical dismissal leaked into the exposed scene");
  assert.equal(tactical.return_release_recipient, "blocked", "tactical return released into the exposed scene");
  steps.E08 = 2;

  const reset = value.reset;
  assert.ok(reset.old_ids.includes(reset.queued_target), "reset evidence omitted the stale target");
  assert.ok(reset.new_ids.every((id) => reset.old_ids.every((old) => id > old)), "reset reused a stale ID");
  assert.equal(reset.stale_action_discarded, true, "reset retained its frozen stale command");
  assert.ok(reset.registered_after_apply.includes(reset.new_ids[0]), "stale command removed the reopened window");
  steps.E09 = 4;

  assert.deepEqual(value.geometry.map(({ phase }) => phase), ["cold", "warm", "restored"]);
  for (const geometry of value.geometry) {
    assert.ok(Array.isArray(geometry.painted) && geometry.painted.length === 4,
      `${geometry.phase} emitted no live paint rectangle`);
    assert.equal(geometry.content_status, "ready", `${geometry.phase} content was not paintable`);
    assert.equal(geometry.hit_recipient, `window:${geometry.id}`,
      `${geometry.phase} hit geometry did not route to the painted window`);
  }
  assert.deepEqual(value.geometry[2].painted, value.geometry[0].painted,
    "restored viewport did not restore Encyclopedia geometry");
  steps.E10 = 11;
  assert.deepEqual(Object.keys(steps), E_STEPS);
  return steps;
}

const STATE_MARKER = "[windowing-contract-state] ";
const ROUTE_MARKER = "[windowing-route] ";
const RENAME_CONTROL = 0x203;
const COMMAND = Object.freeze({
  modalUnpaintable: 1,
  recoverContent: 2,
  modelessUnpaintable: 3,
  modelessAbsent: 4,
  tacticalAlert: 5,
  tacticalCombat: 6,
  tacticalResults: 7,
  galaxyReturn: 8,
  queueStale: 9,
  resetSession: 10,
  closeCapture: 11,
  minimizeCapture: 12,
  focusLoss: 13,
  replaceMenu: 14,
});

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

async function frames(page, count = 3) {
  await page.evaluate((remaining) => new Promise((resolve) => {
    function next(value) {
      if (value === 0) resolve();
      else requestAnimationFrame(() => next(value - 1));
    }
    next(remaining);
  }), count);
}

function center(rect) {
  assert.ok(Array.isArray(rect) && rect.length === 4, `missing rectangle ${JSON.stringify(rect)}`);
  return { x: rect[0] + rect[2] / 2, y: rect[1] + rect[3] / 2 };
}

function tupleMap(entries) {
  return Object.fromEntries(entries ?? []);
}

function screenPoint(context, x, y) {
  const viewport = context.page.viewportSize();
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  return {
    x: (viewport.width - 640 * scale) / 2 + x * scale,
    y: (viewport.height - 480 * scale) / 2 + y * scale,
  };
}

function parseRoute(line) {
  const match = line.match(
    /^\[windowing-route\] physical=(\d+:\d+) kind=(pointer|key) router=(root|blocked|window:\d+) application=([a-z0-9_:-]+) underlay=(\d+)$/,
  );
  assert.ok(match, `malformed route receipt ${line}`);
  return {
    physical: match[1],
    kind: match[2],
    recipient: match[3],
    application: match[4],
    underlay: Number(match[5]),
    line,
  };
}

export async function waitForRouteReceipt({
  routes,
  after,
  expected,
  description,
  advanceFrame,
  attempts = 480,
}) {
  for (let attempt = 0; attempt < attempts; attempt += 1) {
    const route = routes.slice(after)
      .map(parseRoute)
      .find((candidate) => candidate.kind === "pointer"
        && candidate.recipient === expected.recipient
        && candidate.application === expected.application);
    if (route) return route;
    await advanceFrame();
  }
  throw new Error(
    `${description}; last=${JSON.stringify(routes.slice(after).map(parseRoute).at(-1) ?? null)}`,
  );
}

export async function pressWithRouteReceipts({
  page,
  routes,
  point,
  label,
  expectedDown,
  expectedRelease,
  button = "left",
  batchActions = false,
  skipMoveReceipt = false,
  cursor = { physical: null },
  settleFrames = 3,
  advanceFrame,
}) {
  if (batchActions) {
    return clickWithRouteBarrier({
      page,
      routes,
      point,
      label,
      button,
      batchActions: true,
      skipMoveReceipt,
      cursor,
      expectedDown,
      expectedRelease,
      advanceFrame,
      settleFrames,
    });
  }

  const nextFrame = advanceFrame ?? (() => frames(page, 1));
  await page.mouse.move(point.x, point.y);
  await nextFrame();
  const downStart = routes.length;
  await page.mouse.down({ button });
  const down = await waitForRouteReceipt({
    routes,
    after: downStart,
    expected: expectedDown,
    description: `${label} down emitted no expected fresh route receipt`,
    advanceFrame: nextFrame,
  });
  const releaseStart = routes.length;
  await page.mouse.up({ button });
  const release = await waitForRouteReceipt({
    routes,
    after: releaseStart,
    expected: expectedRelease,
    description: `${label} release emitted no expected fresh route receipt`,
    advanceFrame: nextFrame,
  });
  assert.notEqual(release.physical, down.physical, `${label} reused its down route receipt`);
  await nextFrame();
  await nextFrame();
  await nextFrame();
  return { down, release };
}

function createMonitor(page) {
  const states = [];
  const routes = [];
  const lines = [];
  const errors = [];
  const requests = [];
  page.on("console", (message) => {
    const line = message.text();
    lines.push(line);
    const marker = line.indexOf(STATE_MARKER);
    if (marker >= 0) states.push(JSON.parse(line.slice(marker + STATE_MARKER.length)));
    if (line.startsWith(ROUTE_MARKER)) routes.push(line);
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
    lines,
    errors,
    requests,
    latest(status) {
      const state = states.findLast((candidate) => candidate.status === status);
      assert.ok(state, `fixture emitted no ${status}`);
      return state;
    },
    async wait(status, predicate, description, after = 0, attempts = 480) {
      for (let attempt = 0; attempt < attempts; attempt += 1) {
        const state = states.slice(after).findLast((candidate) =>
          candidate.status === status && predicate(candidate));
        if (state) return state;
        await frames(page, 1);
      }
      throw new Error(`${description}; last=${JSON.stringify(states.findLast((state) => state.status === status))}`);
    },
    counters() {
      const parsed = routes.map(parseRoute);
      return {
        application: parsed.filter(({ application }) => application !== "none").length,
        routed: parsed.length,
        underlay: parsed.reduce((sum, route) => sum + route.underlay, 0),
      };
    },
    rootActions() {
      return routes.map(parseRoute)
        .filter(({ recipient, application }) => recipient === "root" && application !== "none")
        .length;
    },
    renameCommits() {
      return lines.filter((line) =>
        line.includes("destination=rename status=applied")).length;
    },
    moveActions() {
      return lines.filter((line) =>
        line.includes("destination=move status=")
          || line.includes("destination=confirmed_move status=")).length;
    },
  };
}

function records(state) {
  return state.snapshot?.records ?? [];
}

function recordOfKind(state, kind) {
  return records(state).findLast((record) => record.kind === kind);
}

function compactState(monitor) {
  const state = monitor.latest("input-scenes-state");
  return {
    registered: state.registered,
    record_count: state.record_count,
    focused: state.focused,
    root_actions: monitor.rootActions(),
    underlay_actions: monitor.counters().underlay,
  };
}

function routeEvent(setup, triggerId, routeLines, expectedKind, applicationOverride = undefined) {
  const routes = routeLines.map(parseRoute).filter(({ kind }) => kind === expectedKind);
  assert.ok(routes.length > 0, `${triggerId} emitted no ${expectedKind} route receipt`);
  const route = routes.at(-1);
  assert.equal(route.underlay, 0, `${triggerId} leaked an underlay action`);
  const event = {
    trigger_id: triggerId,
    event_id: `${setup.events.length}:${route.physical}`,
    kind: route.kind,
    recipient: route.recipient,
    application_recipient: applicationOverride ?? route.application,
    underlay_action_count: route.underlay,
    observed_route: route.line,
    observed_actions: [route.line],
  };
  setup.events.push(event);
  return event;
}

function actionEvent(setup, triggerId, receipt, recipient) {
  assert.ok(typeof receipt === "string" && receipt.length > 0, `${triggerId} emitted no action receipt`);
  const match = receipt.match(/recipient=([a-z-]+)(?:\s|$)/);
  assert.ok(match, `${triggerId} emitted a malformed action receipt: ${receipt}`);
  assert.equal(match[1], recipient, `${triggerId} action reached ${match[1]} instead of ${recipient}`);
  const observedRecipient = match[1];
  const event = {
    trigger_id: triggerId,
    event_id: `${setup.events.length}:action`,
    kind: "pointer",
    recipient: observedRecipient,
    application_recipient: observedRecipient,
    underlay_action_count: 0,
    observed_route: receipt,
    observed_actions: [receipt],
  };
  setup.events.push(event);
  return event;
}

async function key(page, name) {
  await page.keyboard.down(name);
  await frames(page, 1);
  await page.keyboard.up(name);
  await frames(page, 3);
}

async function click(page, point, button = "left") {
  await page.mouse.move(point.x, point.y);
  await frames(page, 1);
  await page.mouse.down({ button });
  await frames(page, 1);
  await page.mouse.up({ button });
  await frames(page, 3);
}

async function issueCommand(setup, code, label, predicate = () => true) {
  const before = setup.monitor.latest("input-scenes-state").command_sequence;
  const start = setup.monitor.states.length;
  await setup.page.evaluate((value) => {
    window.__openRebellionWindowingFixtureCommand = value;
  }, code);
  const state = await setup.monitor.wait(
    "input-scenes-state",
    (candidate) => candidate.command_sequence > before && predicate(candidate),
    `${label} was not applied`,
    start,
  );
  setup.injections.push({ label, code, sequence: state.command_sequence });
  return state;
}

async function openEncyclopedia(setup, triggerId = "open-encyclopedia") {
  const routeStart = setup.monitor.routes.length;
  const stateStart = setup.monitor.states.length;
  await key(setup.page, "F7");
  const state = await setup.monitor.wait(
    "identity-history-state",
    (candidate) => recordOfKind(candidate, "encyclopedia")?.id === candidate.snapshot.active_modal,
    "F7 did not open Encyclopedia",
    stateStart,
  );
  const id = recordOfKind(state, "encyclopedia").id;
  const event = routeEvent(setup, triggerId, setup.monitor.routes.slice(routeStart), "key");
  return { state, id, event };
}

async function closeActiveModal(setup) {
  for (let attempt = 0; attempt < 3; attempt += 1) {
    const before = setup.monitor.states.length;
    await key(setup.page, "Escape");
    try {
      return await setup.monitor.wait(
        "identity-history-state",
        (state) => state.snapshot.active_modal === null,
        "Escape did not close the active modal",
        before,
        120,
      );
    } catch (error) {
      if (attempt === 2) throw error;
    }
  }
  return null;
}

async function openSector(setup, index = 0) {
  const current = setup.monitor.latest("identity-history-state");
  const system = current.systems[index];
  assert.ok(system, `fixture is missing system ${index}`);
  if (system.sector_id) return current;
  const start = setup.monitor.states.length;
  await click(setup.page, { x: system.map_point[0], y: system.map_point[1] });
  return setup.monitor.wait(
    "identity-history-state",
    (state) => state.systems[index]?.sector_id,
    `system ${index} Sector did not open`,
    start,
  );
}

async function openFleet(setup, index = 0) {
  let state = await openSector(setup, index);
  const system = state.systems[index];
  const fleetRect = tupleMap(system.quadrants).fleets;
  assert.ok(fleetRect, "Sector exposes no Fleet quadrant");
  const existing = system.details.fleet.id;
  if (existing && state.snapshot.paint_order.includes(existing)) return state;
  const start = setup.monitor.states.length;
  await setup.page.mouse.dblclick(center(fleetRect).x, center(fleetRect).y);
  await frames(setup.page, 4);
  return setup.monitor.wait(
    "identity-history-state",
    (candidate) => {
      const id = candidate.systems[index]?.details.fleet.id;
      return id && candidate.snapshot.paint_order.includes(id);
    },
    "Fleet detail did not open",
    start,
  );
}

async function selectFleetEntry(setup, index = 0) {
  let state = await openFleet(setup, index);
  let entry = state.systems[index].details.fleet.entry_rect;
  assert.ok(entry, "Fleet detail exposes no fleet entry");
  await click(setup.page, center(entry));
  await frames(setup.page, 2);
  state = setup.monitor.latest("identity-history-state");
  entry = state.systems[index].details.fleet.entry_rect;
  return { state, entry, id: state.systems[index].details.fleet.id };
}

async function openObjectMenu(setup, point, ownerId) {
  assert.ok(Number.isSafeInteger(ownerId), "Fleet object menu has no stable owner identity");
  await setup.page.evaluate(() => { window.__openRebellionInterfaceObjectMenu = undefined; });
  const stateStart = setup.monitor.states.length;
  await pressWithRouteReceipts({
    page: setup.page,
    routes: setup.monitor.routes,
    point,
    label: "Fleet object menu",
    button: "right",
    batchActions: true,
    skipMoveReceipt: true,
    expectedDown: { recipient: `window:${ownerId}`, application: "none" },
    expectedRelease: { recipient: `window:${ownerId}`, application: "fleet_window" },
  });
  await setup.page.waitForFunction(
    () => window.__openRebellionInterfaceObjectMenu?.status === "object-menu",
    null,
    { timeout: 5_000 },
  );
  const menu = await setup.page.evaluate(() => window.__openRebellionInterfaceObjectMenu);
  const latest = setup.monitor.latest("identity-history-state");
  const state = recordOfKind(latest, "menu")
    ? latest
    : await setup.monitor.wait(
      "identity-history-state",
      (candidate) => recordOfKind(candidate, "menu") !== undefined,
      "object menu did not register",
      stateStart,
    );
  return { menu, state, record: recordOfKind(state, "menu") };
}

function menuRowPoint(menu, row) {
  assert.ok(Number.isInteger(row), `menu row is unavailable: ${JSON.stringify(menu)}`);
  const height = (menu.height - 2) / menu.rows;
  return {
    x: menu.left + menu.width / 2,
    y: menu.top + 2 + height * (row + 0.5),
  };
}

async function beginRename(setup, index = 0) {
  const selected = await selectFleetEntry(setup, index);
  const { menu } = await openObjectMenu(setup, center(selected.entry), selected.id);
  const start = setup.monitor.states.length;
  await click(setup.page, menuRowPoint(menu, menu.rename_row));
  const focused = await setup.monitor.wait(
    "input-scenes-state",
    (state) => state.focused?.window === selected.id && state.focused?.control === RENAME_CONTROL,
    "Rename did not acquire child focus",
    start,
  );
  return { selected, focused };
}

async function beginFleetDrag(setup, index = 0) {
  const selected = await selectFleetEntry(setup, index);
  const point = center(selected.entry);
  await setup.page.mouse.move(point.x, point.y);
  await frames(setup.page, 1);
  const stateStart = setup.monitor.states.length;
  await setup.page.mouse.down();
  await frames(setup.page, 1);
  await setup.page.mouse.move(point.x + selected.entry[2] + 36, point.y + 18, { steps: 4 });
  const started = await setup.monitor.wait(
    "input-scenes-state",
    (state) => state.capture === selected.id && state.fleet_dragging === true,
    "Fleet drag did not acquire capture",
    stateStart,
  );
  return { owner: selected.id, started, source: point };
}

function orphanRecipient(routeLines) {
  const route = routeLines.map(parseRoute).findLast(({ kind }) => kind === "pointer");
  assert.ok(route, `orphan release emitted no route receipt: ${JSON.stringify(routeLines)}`);
  return route.recipient;
}

async function captureScreenshot(context, setup, name) {
  const screenshot = await context.page.screenshot({ animations: "disabled" });
  const file = path.join(context.directory, `${name}.png`);
  fs.writeFileSync(file, screenshot);
  setup.screenshots.push(file);
  return { file, sha256: sha256(screenshot) };
}

async function setupInputScenes(context) {
  const monitor = createMonitor(context.page);
  const ready = await context.openFixture();
  await monitor.wait("identity-history-state", () => true, "identity fixture did not start");
  await monitor.wait("input-scenes-state", () => true, "input-scenes fixture did not start");
  return {
    page: context.page,
    monitor,
    ready,
    events: [],
    injections: [],
    screenshots: [],
    initialCounters: monitor.counters(),
  };
}

async function runJourneyD(context, setup, caseId = "journey-d") {
  const encyclopedia = await openEncyclopedia(setup);
  const beforeSearch = compactState(setup.monitor);
  const scale = encyclopedia.state.scale;
  const nativeOrigin = context.faction === "alliance" ? [62, 50] : [125, 52];
  const viewport = context.page.viewportSize();
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  const searchPoint = {
    x: offsetX + (nativeOrigin[0] + 143 + 122.5) * scale,
    y: offsetY + (nativeOrigin[1] + 45 + 9) * scale,
  };
  const searchRoutes = setup.monitor.routes.length;
  await click(setup.page, searchPoint);
  await setup.page.keyboard.type("Mon");
  await key(setup.page, "ArrowDown");
  const searchState = await setup.monitor.wait(
    "input-scenes-state",
    (state) => state.focused?.window === encyclopedia.id && state.focused?.control === 2,
    "Encyclopedia search did not retain child focus",
  );
  const searchEvent = routeEvent(setup, "focused-search", setup.monitor.routes.slice(searchRoutes), "key");
  const search = {
    before: beforeSearch,
    after: { ...compactState(setup.monitor), focused: searchState.focused },
    kind: searchEvent.kind,
    recipient: searchEvent.recipient,
  };
  await closeActiveModal(setup);

  await openFleet(setup);
  const beforeCommit = compactState(setup.monitor);
  const commitCount = setup.monitor.renameCommits();
  const committing = await beginRename(setup);
  await setup.page.keyboard.press("Control+A");
  await setup.page.keyboard.type("Journey D Fleet");
  const commitRoutes = setup.monitor.routes.length;
  const commitStateStart = setup.monitor.states.length;
  await key(setup.page, "Enter");
  const committed = await setup.monitor.wait(
    "input-scenes-state",
    (state) => state.focused?.control !== RENAME_CONTROL
      && setup.monitor.renameCommits() === commitCount + 1,
    "Rename Enter did not commit exactly once",
    commitStateStart,
  );
  const commitEvent = routeEvent(setup, "rename-enter", setup.monitor.routes.slice(commitRoutes), "key");
  const renameCommit = {
    before: beforeCommit,
    after: {
      ...compactState(setup.monitor),
      rename_open: committed.focused?.control === RENAME_CONTROL,
      rename_commits: setup.monitor.renameCommits() - commitCount,
    },
    window: committing.selected.id,
    kind: commitEvent.kind,
    recipient: commitEvent.recipient,
  };

  const cancelBefore = {
    ...compactState(setup.monitor),
    rename_commits: setup.monitor.renameCommits(),
    rename_open: true,
  };
  const cancelling = await beginRename(setup);
  await setup.page.keyboard.type(" Cancelled");
  const cancelRoutes = setup.monitor.routes.length;
  const cancelStateStart = setup.monitor.states.length;
  await key(setup.page, "Escape");
  const cancelled = await setup.monitor.wait(
    "input-scenes-state",
    (state) => state.focused?.control !== RENAME_CONTROL,
    "Rename Escape did not cancel the editor",
    cancelStateStart,
  );
  const cancelEvent = routeEvent(setup, "rename-escape", setup.monitor.routes.slice(cancelRoutes), "key");
  const renameCancel = {
    before: cancelBefore,
    after: {
      ...compactState(setup.monitor),
      rename_open: cancelled.focused?.control === RENAME_CONTROL,
      rename_commits: setup.monitor.renameCommits(),
    },
    window: cancelling.selected.id,
    kind: cancelEvent.kind,
    recipient: cancelEvent.recipient,
  };

  const beforeDetailState = await openFleet(setup);
  const fleetBefore = beforeDetailState.systems[0].details.fleet;
  const minimizePoint = {
    x: fleetBefore.rect[0] + 211 * beforeDetailState.scale,
    y: fleetBefore.rect[1] + 9 * beforeDetailState.scale,
  };
  const minimizeStart = setup.monitor.states.length;
  await click(setup.page, minimizePoint);
  const minimizedState = await setup.monitor.wait(
    "identity-history-state",
    (state) => state.snapshot.history.includes(fleetBefore.id)
      && !state.snapshot.paint_order.includes(fleetBefore.id),
    "Fleet detail did not minimize before the raise/return probe",
    minimizeStart,
  );
  const defensesRect = tupleMap(minimizedState.systems[0].quadrants).defenses;
  assert.ok(defensesRect, "Sector exposes no Defenses quadrant");
  const raiseStart = setup.monitor.states.length;
  await setup.page.mouse.dblclick(center(defensesRect).x, center(defensesRect).y);
  await frames(setup.page, 4);
  const raisedState = await setup.monitor.wait(
    "identity-history-state",
    (state) => state.systems[0].details.defenses.id
      && state.snapshot.paint_order.at(-1) === state.systems[0].details.defenses.id,
    "Defenses detail did not raise over Fleet",
    raiseStart,
  );
  const raisedId = raisedState.systems[0].details.defenses.id;
  const fleetSlot = raisedState.rail.find(({ id }) => id === fleetBefore.id);
  assert.ok(fleetSlot, "minimized Fleet did not retain a return rail slot");
  await frames(setup.page, 6);
  const returnRoutes = setup.monitor.routes.length;
  const returnStart = setup.monitor.states.length;
  await click(setup.page, { x: fleetSlot.point[0], y: fleetSlot.point[1] });
  const returnEvent = routeEvent(
    setup,
    "detail-return",
    setup.monitor.routes.slice(returnRoutes),
    "pointer",
  );
  assert.equal(returnEvent.recipient, "root", "rail return did not route through the strategic root");
  const returnedState = await setup.monitor.wait(
    "identity-history-state",
    (state) => state.snapshot.paint_order.at(-1) === fleetBefore.id
      && !state.snapshot.history.includes(fleetBefore.id),
    "Fleet detail did not return to front",
    returnStart,
  );
  const fleetAfter = returnedState.systems[0].details.fleet;
  const detailReturn = {
    raised_id: raisedId,
    returned_id: fleetAfter.id,
    before: { id: fleetBefore.id, selection: fleetBefore.selection },
    after: {
      selection: fleetAfter.selection,
      registered: returnedState.snapshot.registered,
    },
  };

  const gidControl = context.faction === "alliance"
    ? { x: 459.5, y: 414 }
    : { x: 537.5, y: 446.5 };
  const gidStart = setup.monitor.states.length;
  await click(setup.page, screenPoint(context, gidControl.x, gidControl.y));
  const gidOpen = await setup.monitor.wait(
    "identity-history-state",
    (state) => recordOfKind(state, "menu")?.owner === "root",
    "GID owner Menu did not open",
    gidStart,
  );
  const menuRecord = recordOfKind(gidOpen, "menu");
  const rootPoint = screenPoint(context, 470, 246);
  await setup.page.mouse.move(rootPoint.x, rootPoint.y);
  const submenuStart = setup.monitor.states.length;
  await frames(setup.page, 5);
  const submenuState = await setup.monitor.wait(
    "input-scenes-state",
    (state) => state.menu_rects.length === 2,
    "GID submenu did not publish both hit rectangles",
    submenuStart,
  );
  const rootRoutes = setup.monitor.routes.length;
  await click(setup.page, rootPoint);
  const rootHit = routeEvent(setup, "menu-root-hit", setup.monitor.routes.slice(rootRoutes), "pointer");
  const childPoint = screenPoint(context, 222, 246);
  const childRoutes = setup.monitor.routes.length;
  await click(setup.page, childPoint);
  const childHit = routeEvent(setup, "menu-submenu-hit", setup.monitor.routes.slice(childRoutes), "pointer");

  const selected = await selectFleetEntry(setup);
  const outsideMenu = await openObjectMenu(setup, center(selected.entry), selected.id);
  const rootBefore = setup.monitor.rootActions();
  const outsideRoutes = setup.monitor.routes.length;
  const outsideStateStart = setup.monitor.states.length;
  const cockpit = setup.monitor.latest("identity-history-state").fleet_finder_point;
  await click(setup.page, { x: cockpit[0], y: cockpit[1] });
  const outsideEvent = routeEvent(
    setup,
    "menu-outside-dismiss",
    setup.monitor.routes.slice(outsideRoutes),
    "pointer",
  );
  const outsideDismissed = await setup.monitor.wait(
    "identity-history-state",
    (state) => !state.snapshot.registered.includes(outsideMenu.record.id),
    "outside Menu press did not dismiss the exact blocking identity",
    outsideStateStart,
  );
  const menu = {
    id: menuRecord.id,
    dismissed_id: outsideMenu.record.id,
    owner: menuRecord.owner,
    expected_owner: "root",
    rects: submenuState.menu_rects,
    hit_recipients: [rootHit, childHit].map(({ recipient }) =>
      Number(recipient.slice("window:".length))),
    outside_recipient: outsideEvent.recipient,
    dismissed_registered_after: outsideDismissed.snapshot.registered.includes(outsideMenu.record.id),
    root_actions_before: rootBefore,
    root_actions_after: setup.monitor.rootActions(),
  };

  const replacementSelected = await selectFleetEntry(setup);
  const ownerId = replacementSelected.id;
  const expectedFocus = replacementSelected.state.snapshot.paint_order
    .filter((id) => id !== ownerId)
    .at(-1) ?? null;
  const replacementMenu = await openObjectMenu(
    setup,
    center(replacementSelected.entry),
    replacementSelected.id,
  );
  const oldMenuId = replacementMenu.record.id;
  const ownerRect = replacementMenu.state.systems[0].details.fleet.rect;
  const replacementStart = setup.monitor.states.length;
  await issueCommand(setup, COMMAND.replaceMenu, "inject-menu-replacement");
  const replacedState = await setup.monitor.wait(
    "identity-history-state",
    (state) => {
      const active = recordOfKind(state, "menu");
      return active?.id > oldMenuId
        && active.owner === ownerId
        && !state.snapshot.registered.includes(oldMenuId);
    },
    "managed Menu replacement did not publish a new identity",
    replacementStart,
  );
  const replacementId = recordOfKind(replacedState, "menu").id;
  const replacementOwner = recordOfKind(replacedState, "menu").owner;
  const closePoint = {
    x: ownerRect[0] + 225 * replacementMenu.state.scale,
    y: ownerRect[1] + 9 * replacementMenu.state.scale,
  };
  const dismissStart = setup.monitor.states.length;
  await pressWithRouteReceipts({
    page: setup.page,
    routes: setup.monitor.routes,
    point: closePoint,
    label: "replacement child dismissal",
    expectedDown: { recipient: "blocked", application: "managed_menu" },
    expectedRelease: { recipient: "blocked", application: "none" },
  });
  const childDismissed = await setup.monitor.wait(
    "identity-history-state",
    (state) => !state.snapshot.registered.includes(replacementId)
      && state.snapshot.registered.includes(ownerId),
    "owner press did not dismiss its Menu first",
    dismissStart,
  );
  const ownerCloseStart = setup.monitor.states.length;
  await pressWithRouteReceipts({
    page: setup.page,
    routes: setup.monitor.routes,
    point: closePoint,
    label: "replacement owner close",
    expectedDown: { recipient: `window:${ownerId}`, application: "none" },
    expectedRelease: { recipient: `window:${ownerId}`, application: "fleet_window" },
  });
  const ownerClosed = await setup.monitor.wait(
    "identity-history-state",
    (state) => !state.snapshot.registered.includes(ownerId),
    "second owner press did not close Fleet",
    ownerCloseStart,
  );
  const menuReplacement = {
    old_id: oldMenuId,
    new_id: replacementId,
    old_owner: replacementMenu.record.owner,
    new_owner: replacementOwner,
    expected_owner: ownerId,
    injected: setup.injections.some(({ label }) => label === "inject-menu-replacement"),
    old_registered_after_replace: replacedState.snapshot.registered.includes(oldMenuId),
    new_registered_after_replace: replacedState.snapshot.registered.includes(replacementId),
    child_registered_after_dismiss: childDismissed.snapshot.registered.includes(replacementId),
    owner_registered_after_close: ownerClosed.snapshot.registered.includes(ownerId),
    expected_focus: expectedFocus,
    focus_after: ownerClosed.snapshot.focused,
  };

  await openFleet(setup);
  const drag = await beginFleetDrag(setup);
  const releaseRoutes = setup.monitor.routes.length;
  const dropBefore = setup.monitor.moveActions();
  const releaseStateStart = setup.monitor.states.length;
  await setup.page.mouse.move(100, 300, { steps: 4 });
  await setup.page.mouse.up();
  const released = await setup.monitor.wait(
    "input-scenes-state",
    (state) => state.capture === null && state.fleet_dragging === false,
    "captured Fleet release did not clear its payload",
    releaseStateStart,
  );
  const releaseEvent = routeEvent(setup, "captured-fleet-release", setup.monitor.routes.slice(releaseRoutes), "pointer");
  const capturedRelease = {
    owner: drag.owner,
    release_recipient: releaseEvent.recipient,
    capture_after: released.capture,
    payload_after: released.fleet_dragging,
    drop_delta: setup.monitor.moveActions() - dropBefore,
    underlay_delta: releaseEvent.underlay_action_count,
  };

  const interruptions = [];
  const interrupt = async (kind, action, cleanup = async () => {}) => {
    await openFleet(setup);
    const activeDrag = await beginFleetDrag(setup);
    const moves = setup.monitor.moveActions();
    const cancelStateStart = setup.monitor.states.length;
    await action(activeDrag);
    const cancelledState = await setup.monitor.wait(
      "input-scenes-state",
      (state) => state.capture === null && state.fleet_dragging === false,
      `${kind} did not cancel Fleet capture`,
      cancelStateStart,
    );
    const routeStart = setup.monitor.routes.length;
    await setup.page.mouse.up();
    await frames(setup.page, 4);
    const releaseRecipient = orphanRecipient(setup.monitor.routes.slice(routeStart));
    const orphanEvent = routeEvent(
      setup,
      `orphan-release-${kind}`,
      setup.monitor.routes.slice(routeStart),
      "pointer",
    );
    const injectionLabels = {
      close: "inject-close-capture-owner",
      minimize: "inject-minimize-capture-owner",
      scene: "inject-tactical-alert",
      "focus-loss": "inject-focus-loss",
    };
    interruptions.push({
      kind,
      injected: injectionLabels[kind]
        ? setup.injections.some(({ label }) => label === injectionLabels[kind])
        : false,
      captured_owner_id: activeDrag.owner,
      capture_before: activeDrag.started.capture,
      capture_after: cancelledState.capture,
      payload_after: cancelledState.fleet_dragging,
      release_recipient: releaseRecipient,
      drop_delta: setup.monitor.moveActions() - moves,
      underlay_delta: orphanEvent.underlay_action_count,
    });
    await cleanup();
  };
  await interrupt("modal", async () => {
    await key(setup.page, "F3");
  }, async () => closeActiveModal(setup));
  await interrupt("close", () => issueCommand(setup, COMMAND.closeCapture, "inject-close-capture-owner"));
  await interrupt("minimize", () => issueCommand(setup, COMMAND.minimizeCapture, "inject-minimize-capture-owner"));
  await interrupt("scene", () => issueCommand(
    setup,
    COMMAND.tacticalAlert,
    "inject-tactical-alert",
    (state) => state.scene === "tactical",
  ), async () => issueCommand(
    setup,
    COMMAND.galaxyReturn,
    "inject-galaxy-return",
    (state) => state.scene === "galaxy",
  ));
  await interrupt("focus-loss", () => issueCommand(setup, COMMAND.focusLoss, "inject-focus-loss"));

  await captureScreenshot(context, setup, "journey-d-final");
  return {
    schema_version: WINDOWING_CONTRACT_SCHEMA_VERSION,
    case_id: caseId,
    faction: context.faction,
    viewport: context.viewport,
    events: setup.events,
    action_counters: { before: setup.initialCounters, after: setup.monitor.counters() },
    search,
    renameCommit,
    renameCancel,
    detailReturn,
    menu,
    menuReplacement,
    capturedRelease,
    interruptions,
    timeline: D_STEPS.map((id) => ({ id, assertions: 0 })),
  };
}

async function probeEncyclopediaGeometry(context, setup, opened, phase) {
  const state = await setup.monitor.wait(
    "input-scenes-state",
    (candidate) => candidate.active_modal === opened.id
      && Array.isArray(candidate.encyclopedia_rect)
      && candidate.encyclopedia_content_status === "ready",
    `${phase} Encyclopedia did not publish live ready geometry`,
  );
  const routes = setup.monitor.routes.length;
  await click(setup.page, center(state.encyclopedia_rect));
  const hit = routeEvent(
    setup,
    `encyclopedia-${phase}-hit`,
    setup.monitor.routes.slice(routes),
    "pointer",
  );
  await captureScreenshot(context, setup, `journey-e-${phase}`);
  return {
    phase,
    id: opened.id,
    painted: state.encyclopedia_rect,
    content_status: state.encyclopedia_content_status,
    hit_recipient: hit.recipient,
  };
}

async function runJourneyE(context, setup, caseId = "journey-e") {
  const firstModal = await openEncyclopedia(setup);
  const failedModal = await issueCommand(
    setup,
    COMMAND.modalUnpaintable,
    "inject-modal-unpaintable",
    (state) => state.blocking_unpainted.includes(firstModal.id),
  );
  const outsideStart = setup.monitor.routes.length;
  await click(setup.page, { x: 3, y: 3 });
  const outsideEvent = routeEvent(
    setup,
    "unpaintable-modal-outside",
    setup.monitor.routes.slice(outsideStart),
    "pointer",
  );
  const unpaintableModal = {
    id: firstModal.id,
    registered: failedModal.registered,
    active_modal: failedModal.active_modal,
    blocking_unpainted: failedModal.blocking_unpainted,
    recovery_target: failedModal.failure_target,
    forced_status: failedModal.forced_status,
    outside_recipient: outsideEvent.recipient,
  };
  await issueCommand(setup, COMMAND.recoverContent, "recover-content");
  await closeActiveModal(setup);

  const fleet = await openFleet(setup);
  const fleetId = fleet.systems[0].details.fleet.id;
  const failedModeless = await issueCommand(
    setup,
    COMMAND.modelessUnpaintable,
    "inject-modeless-unpaintable",
    (state) => state.failure_target === fleetId && state.forced_status === "unpaintable",
  );
  const unpaintableModeless = {
    id: fleetId,
    registered: failedModeless.registered,
    removed: failedModeless.removed,
  };
  const absent = await issueCommand(
    setup,
    COMMAND.modelessAbsent,
    "inject-modeless-absent",
    (state) => !state.registered.includes(fleetId) && state.removal_count === 1
      && state.removed.includes(fleetId),
  );
  const absentModeless = {
    id: fleetId,
    registered: absent.registered,
    removed: absent.removed,
    removal_count: absent.removal_count,
  };
  await issueCommand(setup, COMMAND.recoverContent, "recover-after-modeless-absence");
  await frames(setup.page, 4);
  const sectorState = setup.monitor.latest("identity-history-state");
  const sectorClose = sectorState.systems[0].sector_close_point;
  if (sectorState.systems[0].sector_id && sectorClose) {
    const closeStart = setup.monitor.states.length;
    await click(setup.page, { x: sectorClose[0], y: sectorClose[1] });
    await setup.monitor.wait(
      "identity-history-state",
      (state) => state.systems[0].sector_id === null,
      "Sector owner did not close after modeless absence",
      closeStart,
    );
  }

  const dormant = await openEncyclopedia(setup, "open-dormant-galaxy-modal");
  const alert = await issueCommand(
    setup,
    COMMAND.tacticalAlert,
    "inject-tactical-alert",
    (state) => state.scene === "tactical" && state.tactical_phase === "alert"
      && state.dormant_galaxy_registered === true,
  );
  const alertId = alert.active_modal;
  const alertOutsideStart = setup.monitor.routes.length;
  await click(setup.page, { x: 4, y: 4 });
  const alertOutsideEvent = routeEvent(
    setup,
    "battle-alert-outside",
    setup.monitor.routes.slice(alertOutsideStart),
    "pointer",
  );

  await issueCommand(
    setup,
    COMMAND.tacticalCombat,
    "inject-tactical-combat",
    (state) => state.tactical_phase === "combat" && state.active_modal === null,
  );
  const combatLineStart = setup.monitor.lines.length;
  const combatBefore = setup.monitor.latest("input-scenes-state");
  const combatInputBefore = combatBefore.tactical_inputs;
  await click(setup.page, screenPoint(context, 574, 317));
  const combatAfter = await setup.monitor.wait(
    "input-scenes-state",
    (state) => state.tactical_inputs > combatInputBefore
      && state.tactical_action === "toggle_pause"
      && state.tactical_paused !== combatBefore.tactical_paused,
    "Tactical combat probe did not reach the tactical scene",
  );
  const combatReceipt = setup.monitor.lines.slice(combatLineStart)
    .find((line) => line.includes(
      "[windowing-input-scenes-action] recipient=tactical action=toggle_pause",
    ));
  const combatEvent = actionEvent(setup, "tactical-combat-input", combatReceipt, "tactical");

  const results = await issueCommand(
    setup,
    COMMAND.tacticalResults,
    "inject-tactical-results",
    (state) => state.tactical_phase === "results" && state.active_modal !== null,
  );
  const resultsId = results.active_modal;
  const resultsOutsideStart = setup.monitor.routes.length;
  await click(setup.page, { x: 4, y: 4 });
  const resultsOutsideEvent = routeEvent(
    setup,
    "battle-results-outside",
    setup.monitor.routes.slice(resultsOutsideStart),
    "pointer",
  );

  await setup.page.mouse.move(4, 4);
  await setup.page.mouse.down();
  await frames(setup.page, 1);
  await issueCommand(
    setup,
    COMMAND.galaxyReturn,
    "inject-galaxy-return",
    (state) => state.scene === "galaxy",
  );
  const returnRelease = setup.monitor.routes.length;
  await setup.page.mouse.up();
  await frames(setup.page, 4);
  const returnEvent = routeEvent(
    setup,
    "results-return-orphan-release",
    setup.monitor.routes.slice(returnRelease),
    "pointer",
  );
  const tactical = {
    dormant_galaxy_modal: dormant.id,
    dormant_galaxy_registered: alert.dormant_galaxy_registered,
    combat_recipient: combatEvent.recipient,
    combat_action: combatAfter.tactical_action,
    paused_before: combatBefore.tactical_paused,
    paused_after: combatAfter.tactical_paused,
    alert_modal: alertId,
    alert_outside_recipient: alertOutsideEvent.recipient,
    results_modal: resultsId,
    results_outside_recipient: resultsOutsideEvent.recipient,
    return_release_recipient: orphanRecipient(setup.monitor.routes.slice(returnRelease)),
    dismissal_underlay_delta: returnEvent.underlay_action_count,
  };

  const beforeReset = setup.monitor.latest("input-scenes-state");
  await issueCommand(setup, COMMAND.queueStale, "inject-stale-action", (state) =>
    state.queued_target !== null);
  const resetState = await issueCommand(
    setup,
    COMMAND.resetSession,
    "inject-session-reset",
    (state) => state.reset_count === 1 && state.registered.length === 0,
  );
  const reopened = await openEncyclopedia(setup, "open-after-reset");
  const oldIds = [dormant.id, alertId, resultsId, ...beforeReset.registered]
    .filter((value, index, values) => Number.isSafeInteger(value) && values.indexOf(value) === index);
  const reset = {
    queued_target: resetState.queued_target,
    old_ids: oldIds,
    new_ids: [reopened.id],
    stale_action_discarded: resetState.stale_action_discarded,
    registered_after_apply: [],
  };

  const originalViewport = context.page.viewportSize();
  const cold = await probeEncyclopediaGeometry(context, setup, reopened, "cold");
  await frames(setup.page, 4);
  reset.registered_after_apply = setup.monitor.latest("input-scenes-state").registered;
  await closeActiveModal(setup);
  const alternate = originalViewport.width === 640 && originalViewport.height === 480
    ? { width: 1280, height: 800 }
    : { width: 640, height: 480 };
  await context.page.setViewportSize(alternate);
  await frames(context.page, 5);
  const warm = await openEncyclopedia(setup, "open-after-resize");
  const warmGeometry = await probeEncyclopediaGeometry(context, setup, warm, "warm");
  assert.ok(warm.id > reopened.id, "resize reopen reused a stale identity");
  await closeActiveModal(setup);
  await context.page.setViewportSize(originalViewport);
  await frames(context.page, 4);
  const restored = await openEncyclopedia(setup, "open-after-restore");
  const restoredGeometry = await probeEncyclopediaGeometry(context, setup, restored, "restored");
  assert.ok(restored.id > warm.id, "restore reopen reused a stale identity");
  await closeActiveModal(setup);
  const geometry = [cold, warmGeometry, restoredGeometry];

  return {
    schema_version: WINDOWING_CONTRACT_SCHEMA_VERSION,
    case_id: caseId,
    faction: context.faction,
    viewport: context.viewport,
    events: setup.events,
    action_counters: { before: setup.initialCounters, after: setup.monitor.counters() },
    unpaintableModal,
    unpaintableModeless,
    absentModeless,
    tactical,
    reset,
    geometry,
    timeline: E_STEPS.map((id) => ({ id, assertions: 0 })),
  };
}

function finishAssertions(context, observation, stepAssertions) {
  assert.deepEqual(Object.keys(stepAssertions), observation.timeline.map(({ id }) => id),
    "assertions did not bind exactly to the declared journey steps");
  let count = 0;
  for (const step of observation.timeline) {
    step.assertions = stepAssertions[step.id];
    assert.ok(Number.isSafeInteger(step.assertions) && step.assertions > 0,
      `${step.id} completed without a bound assertion`);
    context.completeStep(step.id, step.assertions);
    count += step.assertions;
  }
  observation.assertion_count = count;
  return count;
}

async function writeEvidence(context, observation, setups, scope) {
  const setupList = Array.isArray(setups) ? setups : [setups];
  await frames(context.page, 3);
  const screenshot = await context.page.screenshot({ animations: "disabled" });
  const screenshotPath = path.join(context.directory, "after.png");
  fs.writeFileSync(screenshotPath, screenshot);
  const errors = setupList.flatMap((setup) => setup.monitor.errors);
  const requests = setupList.flatMap((setup) => setup.monitor.requests);
  const consoleNetwork = {
    console: setupList.flatMap((setup) => setup.monitor.lines),
    requests,
    injections: setupList.flatMap((setup) => setup.injections),
  };
  const diagnosticsPath = path.join(context.directory, "console-network.json");
  fs.writeFileSync(diagnosticsPath, `${JSON.stringify(consoleNetwork, null, 2)}\n`);
  const wasm = fs.readFileSync(context.fixtureWasm);
  assert.ok(requests.length > 0, "journey recorded no fixture requests");
  assert.ok(requests.every(({ status }) => status >= 200 && status < 400), JSON.stringify(requests));
  assert.deepEqual(errors, []);
  const result = {
    schema_version: WINDOWING_CONTRACT_SCHEMA_VERSION,
    family: "strategic-windowing-contract",
    scope,
    status: "pass",
    run_id: context.runId,
    assertion_count: observation.assertion_count,
    observation,
    browser_version: context.page.context().browser().version(),
    viewport: context.viewport,
    faction: context.faction,
    muted: true,
    requests,
    errors,
    wasm_sha256: sha256(wasm),
    screenshot_sha256: sha256(screenshot),
  };
  const resultPath = path.join(context.directory, "result.json");
  fs.writeFileSync(resultPath, `${JSON.stringify(result, null, 2)}\n`);
  const files = [
    ["result_json", resultPath],
    ["screenshot", screenshotPath],
    ["fixture_wasm", context.fixtureWasm],
    ["console_network", diagnosticsPath],
    ...setupList.flatMap((setup) => setup.screenshots)
      .map((file, index) => [`journey_screenshot_${index + 1}`, file]),
  ];
  return {
    schema_version: WINDOWING_CONTRACT_SCHEMA_VERSION,
    diagnostics: { errors, requests },
    artifacts: files.map(([kind, file]) => ({
      kind,
      path: path.relative(context.evidenceRoot, file),
      sha256: sha256(fs.readFileSync(file)),
    })),
  };
}

const journeyD = defineWindowingContractCase({
  id: "journey-d",
  groups: ["D"],
  steps: D_STEPS,
  factions: FACTIONS,
  viewports: VIEWPORTS,
  setup: setupInputScenes,
  actions: runJourneyD,
  assertions: (context, observation) => finishAssertions(
    context,
    observation,
    assertChildConsumptionAndGestureCancellation(observation),
  ),
  evidence: (context, observation, setup) => writeEvidence(
    context,
    observation,
    setup,
    "focused fixture journey D; not complete A-E acceptance",
  ),
});

const journeyE = defineWindowingContractCase({
  id: "journey-e",
  groups: ["E"],
  steps: E_STEPS,
  factions: FACTIONS,
  viewports: VIEWPORTS,
  setup: setupInputScenes,
  actions: runJourneyE,
  assertions: (context, observation) => finishAssertions(
    context,
    observation,
    assertFailClosedSceneAndReset(observation),
  ),
  evidence: (context, observation, setup) => writeEvidence(
    context,
    observation,
    setup,
    "focused fixture journey E; not complete A-E acceptance",
  ),
});

const inputScenes = defineWindowingContractCase({
  id: "input-scenes",
  groups: ["D", "E"],
  steps: [...D_STEPS, ...E_STEPS],
  factions: FACTIONS,
  viewports: VIEWPORTS,
  setup: async () => ({ journeyD: null, journeyE: null }),
  actions: async (context, setup) => {
    setup.journeyD = await setupInputScenes(context);
    const d = await runJourneyD(context, setup.journeyD, "journey-d");
    setup.journeyE = await setupInputScenes(context);
    const e = await runJourneyE(context, setup.journeyE, "journey-e");
    return {
      schema_version: WINDOWING_CONTRACT_SCHEMA_VERSION,
      case_id: "input-scenes",
      faction: context.faction,
      viewport: context.viewport,
      events: [
        ...d.events.map((event) => ({ ...event, event_id: `d:${event.event_id}` })),
        ...e.events.map((event) => ({ ...event, event_id: `e:${event.event_id}` })),
      ],
      action_counters: {
        before: { application: 0, routed: 0, underlay: 0 },
        after: {
          application: d.action_counters.after.application + e.action_counters.after.application,
          routed: d.action_counters.after.routed + e.action_counters.after.routed,
          underlay: d.action_counters.after.underlay + e.action_counters.after.underlay,
        },
      },
      journey_d: d,
      journey_e: e,
      timeline: [...d.timeline, ...e.timeline],
    };
  },
  assertions: (context, observation) => finishAssertions(
    context,
    observation,
    {
      ...assertChildConsumptionAndGestureCancellation(observation.journey_d),
      ...assertFailClosedSceneAndReset(observation.journey_e),
    },
  ),
  evidence: (context, observation, setup) => writeEvidence(
    context,
    observation,
    [setup.journeyD, setup.journeyE],
    "focused fixture journeys D-E; complete D-E slice only",
  ),
});

export const windowingContractCases = Object.freeze([journeyD, journeyE, inputScenes]);
