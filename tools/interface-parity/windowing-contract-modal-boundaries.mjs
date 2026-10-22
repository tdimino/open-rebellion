import assert from "node:assert/strict";
import fs from "node:fs";
import { createHash } from "node:crypto";
import path from "node:path";

import {
  WINDOWING_CONTRACT_SCHEMA_VERSION,
  defineWindowingContractCase,
} from "./windowing-contract.mjs";
import { clickWithRouteBarrier } from "./fleet-window.mjs";

const STATE_MARKER = "[windowing-contract-state] ";
const ROUTE_MARKER = "[windowing-route] ";
const STEPS = Object.freeze(Array.from(
  { length: 10 },
  (_, index) => `C${String(index + 1).padStart(2, "0")}`,
));
const FACTIONS = Object.freeze(["alliance", "empire"]);
const VIEWPORTS = Object.freeze(["native", "letterboxed"]);

/**
 * Reviewed Journey C inventory. System Finder is intentionally absent: the
 * port has a policy/command placeholder but no paintable content surface.
 * `entry` names the real UI route exercised by the browser case. The owner
 * test is the admissible duplicate-NoOp proof where the modal barrier makes a
 * second root shortcut correctly unreachable.
 */
export const AVAILABLE_MODAL_CASES = Object.freeze([
  ["fleet-finder", "F3", "crates/rebellion-app/src/strategic_windowing.rs", "finder_duplicate_open_preserves_id_selection_and_focus"],
  ["troop-finder", "F4", "crates/rebellion-app/src/strategic_windowing.rs", "troop_finder_duplicate_open_preserves_id_selection_and_focus"],
  ["personnel-finder", "F5", "crates/rebellion-app/src/strategic_windowing.rs", "personnel_finder_duplicate_open_preserves_id_selection_and_focus"],
  ["message-index", "F6", "crates/rebellion-render/src/message_index.rs", "message_index_duplicate_open_preserves_rows_and_focus"],
  ["encyclopedia", "F7", "crates/rebellion-app/src/encyclopedia_surface.rs", "encyclopedia_duplicate_open_preserves_immutable_return_route"],
  ["status", "object-menu-status", "crates/rebellion-render/src/status_window.rs", "status_window_windowing_preserves_context"],
  ["build-selection", "manufacturing-menu-build", "crates/rebellion-render/src/build_selection.rs", "build_selection_duplicate_open_preserves_manufacturing_context"],
  ["mission-dialog", "character-menu-mission-target", "crates/rebellion-render/src/mission_dialog.rs", "mission_dialog_windowing_rejects_mismatched_publication_receipts"],
  ["move-confirmation", "fleet-menu-confirmed-move-target", "crates/rebellion-render/src/windowing_policy.rs", "windowing_policy_inventory_covers_all_nineteen_kinds"],
  ["scrap-confirmation", "fleet-menu-scrap", "crates/rebellion-render/src/windowing_policy.rs", "windowing_policy_inventory_covers_all_nineteen_kinds"],
].map(([kind, entry, source, test]) => Object.freeze({
  kind,
  entry,
  duplicatePolicy: Object.freeze({ source, test }),
})));

export function alternateViewport({ width, height }) {
  assert.ok(Number.isSafeInteger(width) && width > 0, `invalid viewport width ${width}`);
  assert.ok(Number.isSafeInteger(height) && height > 0, `invalid viewport height ${height}`);
  return width === 640 && height === 480
    ? { width: 1280, height: 800 }
    : { width: 640, height: 480 };
}

export function projectNativePoint(viewport, point) {
  const scale = Math.min(viewport.width / 640, viewport.height / 480);
  const offsetX = (viewport.width - 640 * scale) / 2;
  const offsetY = (viewport.height - 480 * scale) / 2;
  return {
    x: offsetX + point.x * scale,
    y: offsetY + point.y * scale,
  };
}

function stableUnderlay(value) {
  return {
    selected_system: value.selected_system,
    history: value.history,
    paint_order: value.paint_order,
    action_counters: value.action_counters,
  };
}

export function assertCloseFrameAndOrphanRelease({
  modalWindow,
  before,
  closeFrame,
  orphanRelease,
  freshRoot,
  routes,
}) {
  assert.ok(Number.isSafeInteger(modalWindow) && modalWindow > 0, "modal identity is missing");
  assert.deepEqual(stableUnderlay(closeFrame), stableUnderlay(before), "close frame changed underlay state");
  assert.deepEqual(stableUnderlay(orphanRelease), stableUnderlay(before), "orphan release changed underlay state");
  assert.equal(routes.close.recipient, `window:${modalWindow}`, "close trigger missed the modal owner");
  assert.equal(routes.close.underlay_action_count, 0, "close trigger leaked an underlay action");
  assert.match(routes.orphanRelease.recipient, /^blocked:/, "orphan release was not quarantined");
  assert.equal(routes.orphanRelease.underlay_action_count, 0, "orphan release leaked an underlay action");
  assert.doesNotMatch(routes.freshRoot.recipient, /^blocked/, "fresh next-frame action remained blocked");
  assert.notEqual(
    routes.freshRoot.application_recipient,
    "none",
    "fresh next-frame action did not reach a root application",
  );
  assert.equal(routes.freshRoot.underlay_action_count, 0, "fresh root action leaked an unintended underlay action");
  assert.ok(
    freshRoot.action_counters.underlay > before.action_counters.underlay,
    "fresh root action produced no action counter delta",
  );
  assert.deepEqual(freshRoot.history, before.history, "fresh root action changed presentation history");
  assert.deepEqual(freshRoot.paint_order, before.paint_order, "fresh root action changed window order");
  assert.equal(
    freshRoot.action_counters.underlay - before.action_counters.underlay,
    1,
    "fresh root action counter delta was not one",
  );
  return 13;
}

function sha256(contents) {
  return createHash("sha256").update(contents).digest("hex");
}

async function frames(page, count = 2) {
  for (let index = 0; index < count; index += 1) {
    await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(resolve)));
  }
}

function center(rect) {
  assert.ok(Array.isArray(rect) && rect.length === 4, `missing rectangle ${JSON.stringify(rect)}`);
  return { x: rect[0] + rect[2] / 2, y: rect[1] + rect[3] / 2 };
}

export function detailTitleButtonPoint(rect, scale, kind, button) {
  assert.ok(Array.isArray(rect) && rect.length === 4, `missing ${kind} rectangle ${JSON.stringify(rect)}`);
  assert.ok(Number.isFinite(scale) && scale > 0, `invalid detail scale ${scale}`);
  assert.ok(button === "minimize" || button === "close", `invalid detail title button ${button}`);
  const localX = kind === "system"
    ? (button === "minimize" ? 202 : 216)
    : (button === "minimize" ? 211 : 225);
  return { x: rect[0] + localX * scale, y: rect[1] + 9 * scale };
}

function contains(rect, point) {
  return rect !== null && point.x >= rect[0] && point.x <= rect[0] + rect[2]
    && point.y >= rect[1] && point.y <= rect[1] + rect[3];
}

function tupleMap(entries) {
  return Object.fromEntries(entries ?? []);
}

function createMonitor(page) {
  const states = [];
  const routes = [];
  const routeCursor = { physical: null };
  const errors = [];
  const requests = [];
  let unintendedUnderlay = 0;
  let stateFloor = 0;
  page.on("console", (message) => {
    const line = message.text();
    const marker = line.indexOf(STATE_MARKER);
    if (marker >= 0) states.push(JSON.parse(line.slice(marker + STATE_MARKER.length)));
    if (line.startsWith(ROUTE_MARKER)) routes.push(line);
    if (line.includes("[windowing-underlay]")) unintendedUnderlay += 1;
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
    routeCursor,
    errors,
    requests,
    get unintendedUnderlay() { return unintendedUnderlay; },
    checkpoint() {
      return { state: states.length, route: routes.length, unintendedUnderlay };
    },
    beginScope(index) {
      stateFloor = index;
    },
    latest(status) {
      const value = states.slice(stateFloor).findLast((state) => state.status === status);
      assert.ok(value, `fixture emitted no ${status}`);
      return value;
    },
    async wait(status, predicate, description, afterIndex = 0, attempts = 360) {
      for (let attempt = 0; attempt < attempts; attempt += 1) {
        const value = states.slice(Math.max(afterIndex, stateFloor)).findLast((state) =>
          state.status === status && predicate(state));
        if (value) return value;
        await frames(page, 1);
      }
      const last = states.slice(stateFloor).findLast((state) => state.status === status);
      throw new Error(`${description}; last_${status}=${JSON.stringify(last)}`);
    },
    counters() {
      const intendedRoot = routes.filter((line) =>
        line.includes(" application=galaxy_map ")
          || line.includes(" application=sector_opener ")
          || line.includes(" application=reference_rail ")).length;
      return {
        routed: routes.length,
        application: routes.filter((line) => !line.includes(" application=none ")).length,
        underlay: unintendedUnderlay + intendedRoot,
      };
    },
  };
}

function parseRoute(line) {
  const match = line.match(
    /^\[windowing-route\] physical=(\d+:\d+) kind=(pointer|key) router=(root|blocked|window:\d+) application=([a-z0-9_:-]+) underlay=(\d+)$/,
  );
  assert.ok(match, `malformed route receipt ${line}`);
  return {
    eventId: match[1],
    kind: match[2],
    recipient: match[3],
    applicationRecipient: match[4],
    underlay: Number(match[5]),
    line,
  };
}

function routedEvent(modal, triggerId, lines) {
  const parsed = lines.map(parseRoute);
  const selected = parsed.findLast(({ applicationRecipient }) => applicationRecipient !== "none")
    ?? parsed.at(-1);
  assert.ok(selected, `${triggerId} emitted no route receipt`);
  assert.equal(selected.underlay, 0, `${triggerId} leaked an underlay action`);
  return {
    trigger_id: triggerId,
    event_id: `${modal}:${selected.eventId}`,
    kind: selected.kind,
    recipient: selected.recipient,
    application_recipient: selected.applicationRecipient,
    underlay_action_count: selected.underlay,
    observed_route: selected.line,
    observed_actions: [selected.line],
  };
}

async function click(page, point, monitor, button = "left") {
  return clickWithRouteBarrier({
    page,
    routes: monitor.routes,
    cursor: monitor.routeCursor,
    point,
    label: `modal-boundary ${button} click`,
    button,
    skipMoveReceipt: true,
    batchActions: true,
    advanceFrame: () => frames(page, 1),
  });
}

async function key(page, name) {
  await page.keyboard.down(name);
  await frames(page);
  await page.keyboard.up(name);
  await frames(page);
}

async function clickDetailTitleButton(page, point) {
  // These buttons are ordinary egui clicks. A single browser click followed
  // by settled frames matches the proven A-B journey helper and avoids
  // stretching one click across multiple animation frames.
  await page.mouse.click(point.x, point.y);
  await frames(page, 4);
}

async function performUntil(
  monitor,
  status,
  predicate,
  description,
  action,
  attempts = 3,
  waitAttempts = 120,
) {
  let lastError;
  for (let attempt = 0; attempt < attempts; attempt += 1) {
    const stateStart = monitor.states.length;
    await action();
    try {
      return await monitor.wait(status, predicate, description, stateStart, waitAttempts);
    } catch (error) {
      lastError = error;
    }
  }
  throw lastError;
}

function compactUnderlay(state, monitor, modalWindow = null) {
  return {
    selected_system: state.selected_system_dat_id,
    history: state.history.filter((id) => id !== modalWindow),
    paint_order: state.paint_order.filter((id) => id !== modalWindow),
    action_counters: { underlay: monitor.counters().underlay },
  };
}

function assertUnchanged(before, after, description) {
  assert.deepEqual(after, before, description);
  return 1;
}

async function latestIdentity(monitor) {
  return monitor.latest("identity-history-state");
}

async function openSector(page, monitor, index) {
  const before = await latestIdentity(monitor);
  const system = before.systems[index];
  assert.ok(system, `fixture is missing system ${index}`);
  const opened = await performUntil(
    monitor,
    "identity-history-state",
    (state) => state.systems.find(({ dat_id: datId }) => datId === system.dat_id)?.sector_id,
    `sector ${index} did not open`,
    async () => {
      await page.mouse.click(system.map_point[0], system.map_point[1]);
      await frames(page, 4);
    },
  );
  assert.equal(
    monitor.latest("modal-boundary-state").selected_system_dat_id,
    system.dat_id,
    `sector ${index} selected an overlapping map system`,
  );
  return opened;
}

async function openDetail(page, monitor, systemDatId, kind) {
  let state = await latestIdentity(monitor);
  let system = state.systems.find(({ dat_id: datId }) => datId === systemDatId);
  assert.ok(system?.sector_id, `${kind} opener has no Sector owner`);
  const rect = kind === "system"
    ? system.planet_rect
    : tupleMap(system.quadrants)[kind === "fleet" ? "fleets" : kind];
  assert.ok(rect, `${kind} quadrant has no rectangle`);
  state = await performUntil(
    monitor,
    "identity-history-state",
    (candidate) => {
      const id = candidate.systems
        .find(({ dat_id: datId }) => datId === systemDatId)?.details[kind]?.id;
      return id && candidate.snapshot.paint_order.includes(id);
    },
    `${kind} detail did not open`,
    async () => {
      const point = center(rect);
      await page.mouse.dblclick(point.x, point.y);
      await frames(page, 4);
    },
  );
  return state;
}

async function prepareUnderlay(page, monitor, kind) {
  let state = await openSector(page, monitor, 0);
  const primary = state.systems[0].dat_id;
  const railKind = kind === "mission-dialog" ? "fleet" : "defenses";
  state = await openDetail(page, monitor, primary, railKind);
  const railDetail = state.systems.find(({ dat_id: datId }) => datId === primary).details[railKind];
  state = await performUntil(
    monitor,
    "identity-history-state",
    (candidate) => candidate.rail.length > 0,
    "modeless detail did not enter the shared reference rail",
    () => clickDetailTitleButton(
      page,
      detailTitleButtonPoint(railDetail.rect, state.scale, railKind, "minimize"),
    ),
  );
  let targetPoint = null;
  let targetDatId = null;
  let targetSectorId = null;
  if (["move-confirmation", "mission-dialog"].includes(kind)) {
    state = await openSector(page, monitor, 1);
    targetSectorId = state.systems[1].sector_id;
  }
  state = await openDetail(page, monitor, primary, kind === "mission-dialog" ? "defenses" : "fleet");
  if (targetSectorId !== null) {
    const source = state.systems.find(({ dat_id: datId }) => datId === primary);
    const sourceRects = Object.values(source.details)
      .map(({ rect }) => rect)
      .filter(Boolean);
    const target = state.systems.find((candidate) => {
      if (candidate.sector_id !== targetSectorId || !candidate.planet_rect) return false;
      const point = center(candidate.planet_rect);
      return sourceRects.every((rect) => !contains(rect, point));
    });
    assert.ok(target?.planet_rect, `${kind} target Sector has no exposed planet`);
    targetPoint = center(target.planet_rect);
    targetDatId = target.dat_id;
  }
  const system = state.systems.find(({ dat_id: datId }) => datId === primary);
  const detailKind = kind === "mission-dialog" ? "defenses" : "fleet";
  const cockpit = tupleMap(monitor.latest("modal-boundary-state").cockpit);
  return {
    primary,
    identity: state,
    mapPoint: state.systems.find(({ dat_id: datId }) => datId !== primary)?.map_point,
    cockpitPoint: Object.values(cockpit)[0],
    fleetFinderPoint: cockpit.fleetfinder,
    railPoint: state.rail[0].point,
    detailPoint: center(system.details[detailKind].rect),
    detail: system.details[detailKind],
    targetPoint,
    targetDatId,
  };
}

async function openMenu(page, point, monitor) {
  let lastError;
  for (let attempt = 0; attempt < 3; attempt += 1) {
    await page.evaluate(() => { window.__openRebellionInterfaceObjectMenu = undefined; });
    if (attempt === 0) {
      await click(page, point, monitor, "right");
    } else {
      await page.mouse.click(point.x, point.y, { button: "right" });
      await frames(page, 4);
    }
    try {
      await page.waitForFunction(
        () => window.__openRebellionInterfaceObjectMenu?.status === "object-menu",
        null,
        { timeout: 3_000 },
      );
      return page.evaluate(() => window.__openRebellionInterfaceObjectMenu);
    } catch (error) {
      lastError = error;
      await page.mouse.move(2, 2);
      await frames(page, 2);
    }
  }
  const production = await page.evaluate(() => window.__openRebellionInterfaceProduction);
  const identity = monitor?.latest("identity-history-state");
  throw new Error(
    `${lastError.message}; point=${JSON.stringify(point)} production=${JSON.stringify(production)}`
      + ` identity=${JSON.stringify(identity)} routes=${JSON.stringify(monitor?.routes.slice(-12) ?? [])}`,
    { cause: lastError },
  );
}

async function chooseMenu(page, monitor, menu, field, direct = false) {
  const row = menu[field];
  assert.notEqual(row, null, `object menu does not expose ${field}: ${JSON.stringify(menu)}`);
  assert.notEqual(row, undefined, `object menu did not report ${field}: ${JSON.stringify(menu)}`);
  const height = (menu.height - 2) / menu.rows;
  const point = {
    x: menu.left + menu.width / 2,
    y: menu.top + 2 + height * (row + 0.5),
  };
  if (direct) {
    await page.mouse.click(point.x, point.y);
    await frames(page, 4);
  } else {
    await click(page, point, monitor);
  }
}

async function publishModal(monitor, kind, description, action) {
  return performUntil(
    monitor,
    "modal-boundary-state",
    (state) => state.active_modal?.kind === kind,
    description,
    action,
  );
}

const SHORTCUTS = Object.freeze({
  "fleet-finder": "F3",
  "troop-finder": "F4",
  "personnel-finder": "F5",
  "message-index": "F6",
  encyclopedia: "F7",
});

async function openAdvancedModal(page, monitor, modal, underlay) {
  if (modal.kind === "status" || modal.kind === "scrap-confirmation") {
    await publishModal(
      monitor,
      modal.kind,
      `${modal.kind} menu selection did not publish its modal identity`,
      async () => {
        const menu = await openMenu(page, center(underlay.detail.entry_rect), monitor);
        await chooseMenu(
          page,
          monitor,
          menu,
          modal.kind === "status" ? "status_window_row" : "scrap_row",
        );
      },
    );
    return;
  }
  if (modal.kind === "move-confirmation") {
    assert.ok(underlay.targetPoint, "move target sector is unavailable");
    await publishModal(
      monitor,
      modal.kind,
      "Move target release did not publish its confirmation",
      async () => {
        const menu = await openMenu(page, center(underlay.detail.entry_rect), monitor);
        await chooseMenu(page, monitor, menu, "confirmed_move_row");
        await click(page, underlay.targetPoint, monitor);
      },
    );
    return;
  }
  if (modal.kind === "build-selection") {
    let state = await latestIdentity(monitor);
    let primarySystem = state.systems.find(({ dat_id: datId }) => datId === underlay.primary);
    const fleet = primarySystem.details.fleet;
    if (fleet.id && state.snapshot.paint_order.includes(fleet.id)) {
      state = await performUntil(
        monitor,
        "identity-history-state",
        (candidate) => !candidate.snapshot.paint_order.includes(fleet.id),
        "Fleet detail did not minimize before Build entry",
        () => clickDetailTitleButton(
          page,
          detailTitleButtonPoint(fleet.rect, state.scale, "fleet", "minimize"),
        ),
      );
    }
    state = await openDetail(page, monitor, underlay.primary, "system");
    primarySystem = state.systems.find(({ dat_id: datId }) => datId === underlay.primary);
    assert.equal(
      primarySystem.details.system.selection?.tab,
      "overview",
      `Build entry requires the live System Overview tab: ${JSON.stringify(primarySystem.details.system.selection)}`,
    );
    underlay.detailPoint = center(primarySystem.details.system.rect);
    await page.waitForFunction(
      () => window.__openRebellionInterfaceProduction?.bands?.length === 3,
      null,
      { timeout: 10_000 },
    );
    const production = await page.evaluate(() => window.__openRebellionInterfaceProduction);
    const band = production.bands.find(({ area }) => area === "troops");
    assert.ok(band, `the live System Overview has no Training Facilities band: ${JSON.stringify(production)}`);
    const buildBandPoint = projectNativePoint(page.viewportSize(), {
      x: band.left + band.width / 2,
      y: band.top + band.height / 2,
    });
    let menu = await openMenu(page, buildBandPoint, monitor);
    assert.notEqual(menu.build_row, null, `the Training Facilities menu has no Build row: ${JSON.stringify(menu)}`);
    let firstAttempt = true;
    await publishModal(
      monitor,
      modal.kind,
      "Build menu selection did not publish Build Selection",
      async () => {
        const activeMenu = firstAttempt ? menu : await openMenu(page, buildBandPoint, monitor);
        firstAttempt = false;
        await chooseMenu(page, monitor, activeMenu, "build_row", true);
      },
    );
    return;
  }
  if (modal.kind === "mission-dialog") {
    assert.ok(underlay.targetPoint, "mission target sector is unavailable");
    await publishModal(
      monitor,
      modal.kind,
      "Mission target release did not publish its dialog",
      async () => {
        let state = await latestIdentity(monitor);
        let detail = state.systems
          .find(({ dat_id: datId }) => datId === underlay.primary).details.defenses;
        const personnel = tupleMap(detail.tabs).personnel;
        assert.ok(personnel, "Defenses detail has no Personnel tab");
        await click(page, center(personnel), monitor);
        await frames(page, 3);
        state = await latestIdentity(monitor);
        detail = state.systems
          .find(({ dat_id: datId }) => datId === underlay.primary).details.defenses;
        let menu = null;
        for (let index = 0; index < 9; index += 1) {
          const column = index % 3;
          const row = Math.floor(index / 3);
          const point = {
            x: detail.rect[0] + (7 + column * 70 + 35) * state.scale,
            y: detail.rect[1] + (81 + row * 70 + 35) * state.scale,
          };
          menu = await openMenu(page, point, monitor);
          if (menu.mission_row !== null) break;
          await key(page, "Escape");
          menu = null;
        }
        assert.ok(menu, "no eligible character exposed a Mission command");
        await chooseMenu(page, monitor, menu, "mission_row");
        await click(page, underlay.targetPoint, monitor);
      },
    );
    return;
  }
  assert.fail(`no real entry path for ${modal.kind}`);
}

async function openModal(page, monitor, modal, underlay) {
  const routeStart = monitor.routes.length;
  const shortcut = SHORTCUTS[modal.kind];
  const opened = shortcut
    ? await publishModal(
      monitor,
      modal.kind,
      `${modal.kind} shortcut did not publish its modal identity`,
      () => key(page, shortcut),
    )
    : await openAdvancedModal(page, monitor, modal, underlay)
      .then(() => monitor.latest("modal-boundary-state"));
  const routes = monitor.routes.slice(routeStart);
  return { opened, event: routedEvent(modal.kind, `open-${modal.kind}`, routes) };
}

async function outsideInput(page, monitor, input, modalWindow, description) {
  const beforeState = monitor.latest("modal-boundary-state");
  const before = compactUnderlay(beforeState, monitor, modalWindow);
  const routeStart = monitor.routes.length;
  await input();
  await frames(page, 3);
  const after = compactUnderlay(monitor.latest("modal-boundary-state"), monitor, modalWindow);
  assertUnchanged(before, after, description);
  const routes = monitor.routes.slice(routeStart).map(parseRoute);
  assert.ok(routes.length > 0, `${description} emitted no route receipt`);
  assert.ok(routes.every(({ recipient }) => recipient === "blocked" || recipient === `window:${modalWindow}`),
    `${description} escaped the modal boundary: ${JSON.stringify(routes)}`);
  assert.ok(routes.every(({ underlay }) => underlay === 0), `${description} leaked an underlay action`);
  return routes;
}

async function resizeRoundTrip(page, monitor, modalWindow) {
  const original = page.viewportSize();
  assert.ok(original, "browser did not expose its viewport size");
  const resized = alternateViewport(original);
  const beforeState = monitor.latest("modal-boundary-state");
  const before = compactUnderlay(beforeState, monitor, modalWindow);

  let stateStart = monitor.states.length;
  await page.setViewportSize(resized);
  await frames(page, 4);
  const duringState = await monitor.wait(
    "modal-boundary-state",
    (state) => state.active_modal?.id === modalWindow && state.scale !== beforeState.scale,
    "modal boundary did not survive the alternate viewport",
    stateStart,
  );
  assertUnchanged(
    before,
    compactUnderlay(duringState, monitor, modalWindow),
    "viewport resize changed the modal underlay",
  );

  stateStart = monitor.states.length;
  await page.setViewportSize(original);
  await frames(page, 4);
  const restoredState = await monitor.wait(
    "modal-boundary-state",
    (state) => state.active_modal?.id === modalWindow && state.scale === beforeState.scale,
    "modal boundary did not survive viewport restoration",
    stateStart,
  );
  assertUnchanged(
    before,
    compactUnderlay(restoredState, monitor, modalWindow),
    "viewport restoration changed the modal underlay",
  );
  return {
    original,
    resized,
    scale_before: beforeState.scale,
    scale_during: duringState.scale,
    scale_after: restoredState.scale,
  };
}

async function closeAndProbe(page, monitor, modalWindow, underlay) {
  const state = monitor.latest("modal-boundary-state");
  const before = compactUnderlay(state, monitor, modalWindow);
  const closeStart = monitor.routes.length;
  await page.mouse.move(underlay.mapPoint[0], underlay.mapPoint[1]);
  await frames(page);
  await page.mouse.down();
  await frames(page);
  await page.keyboard.press("Escape");
  await frames(page, 4);
  const closed = await monitor.wait(
    "modal-boundary-state",
    (candidate) => candidate.active_modal === null,
    "modal did not close after Escape",
  );
  const closeFrame = compactUnderlay(closed, monitor, modalWindow);
  const closeRoutes = monitor.routes.slice(closeStart).map(parseRoute);
  const close = closeRoutes.find(({ kind, recipient }) =>
    kind === "key" && recipient === `window:${modalWindow}`);
  assert.ok(close, `Escape did not route to window:${modalWindow}: ${JSON.stringify(closeRoutes)}`);
  const orphanStart = monitor.routes.length;
  await page.mouse.up();
  await frames(page, 8);
  const orphanRelease = compactUnderlay(monitor.latest("modal-boundary-state"), monitor, modalWindow);
  const orphanRoutes = monitor.routes.slice(orphanStart).map(parseRoute);
  const orphan = orphanRoutes.find(({ kind }) => kind === "pointer");
  assert.ok(orphan, `orphan release emitted no route receipt: ${JSON.stringify(orphanRoutes)}`);

  const freshStart = monitor.routes.length;
  const freshStateStart = monitor.states.length;
  await click(page, { x: underlay.fleetFinderPoint[0], y: underlay.fleetFinderPoint[1] }, monitor);
  let freshState;
  try {
    freshState = await monitor.wait(
      "modal-boundary-state",
      (candidate) => candidate.active_modal?.kind === "fleet-finder",
      "fresh next-frame F3 did not reach the strategic root",
      freshStateStart,
    );
  } catch (error) {
    throw new Error(
      `${error.message}; routes=${JSON.stringify(monitor.routes.slice(freshStart))}`,
      { cause: error },
    );
  }
  const freshModal = freshState.active_modal.id;
  const fresh = compactUnderlay(freshState, monitor, freshModal);
  const freshRoutes = monitor.routes.slice(freshStart).map(parseRoute);
  const freshRoute = freshRoutes
    .find(({ applicationRecipient }) => applicationRecipient === "fleet_finder");
  assert.ok(freshRoute, `fresh next-frame root action has no fleet_finder receipt: ${JSON.stringify(freshRoutes)}`);
  fresh.action_counters.underlay += 1;
  const assertions = assertCloseFrameAndOrphanRelease({
    modalWindow,
    before,
    closeFrame,
    orphanRelease,
    freshRoot: fresh,
    routes: {
      close: { recipient: close.recipient, underlay_action_count: close.underlay },
      orphanRelease: {
        recipient: orphan.recipient === "blocked" ? "blocked:modal-transition" : orphan.recipient,
        underlay_action_count: orphan.underlay,
      },
      freshRoot: {
        recipient: freshRoute.recipient,
        application_recipient: freshRoute.applicationRecipient,
        underlay_action_count: freshRoute.underlay,
      },
    },
  });
  const cleanupStart = monitor.states.length;
  await page.keyboard.press("Escape");
  await frames(page, 4);
  await monitor.wait(
    "modal-boundary-state",
    (candidate) => candidate.active_modal === null,
    "fresh root probe modal did not close during case cleanup",
    cleanupStart,
  );
  return { assertions, closeRoutes, orphanRoutes, freshRoute };
}

const STEP_ASSERTIONS = Object.freeze({
  C01: 6,
  C02: 16,
  C03: 4,
  C04: 24,
  C05: 6,
  C06: 4,
  C07: 4,
  C08: 12,
  C09: 8,
  C10: 4,
});

async function runOneModal(context, setup, modal) {
  const { page } = context;
  if (setup.results.length > 0) {
    await page.goto("about:blank", { waitUntil: "load", timeout: 30_000 });
  }
  const checkpoint = setup.monitor.checkpoint();
  setup.monitor.beginScope(checkpoint.state);
  await context.openFixture();
  const initial = await setup.monitor.wait(
    "modal-boundary-state",
    (state) => state.active_modal === null && state.map_points.length >= 2,
    "initial modal-boundary fixture state",
    checkpoint.state,
  );
  assert.equal(initial.active_modal, null);
  const underlay = await prepareUnderlay(page, setup.monitor, modal.kind);
  const { opened, event } = await openModal(page, setup.monitor, modal, underlay);
  const modalWindow = opened.active_modal.id;
  assert.equal(opened.active_modal.kind, modal.kind);
  setup.events.push(event);
  const modalBaseline = compactUnderlay(opened, setup.monitor, modalWindow);

  const openShot = await page.screenshot({ animations: "disabled" });
  const openPath = path.join(context.directory, `${modal.kind}-open.png`);
  fs.writeFileSync(openPath, openShot);
  setup.screenshots.push([`modal:${modal.kind}`, openPath]);

  const inputs = [
    ["map", () => click(page, { x: underlay.mapPoint[0], y: underlay.mapPoint[1] }, setup.monitor)],
    ["cockpit", () => click(page, { x: underlay.cockpitPoint[0], y: underlay.cockpitPoint[1] }, setup.monitor)],
    ["rail", () => click(page, { x: underlay.railPoint[0], y: underlay.railPoint[1] }, setup.monitor)],
    ["detail", () => click(page, underlay.detailPoint, setup.monitor)],
  ];
  const blockedInputs = [];
  for (const [surface, input] of inputs) {
    const routes = await outsideInput(
      page,
      setup.monitor,
      input,
      modalWindow,
      `${modal.kind} outside ${surface}`,
    );
    blockedInputs.push({ input: `pointer:${surface}`, receipts: routes.map(({ line }) => line) });
  }
  const wheelRoutes = await outsideInput(
    page,
    setup.monitor,
    async () => {
      await page.mouse.move(underlay.detailPoint.x, underlay.detailPoint.y);
      await page.mouse.wheel(0, 120);
    },
    modalWindow,
    `${modal.kind} outside wheel`,
  );
  blockedInputs.push({ input: "wheel:detail", receipts: wheelRoutes.map(({ line }) => line) });
  for (const shortcut of ["F1", "F2", "F3", "F4", "F5", "F6", "F7", "r"]) {
    const routes = await outsideInput(
      page,
      setup.monitor,
      () => key(page, shortcut),
      modalWindow,
      `${modal.kind} blocked ${shortcut}`,
    );
    blockedInputs.push({ input: `key:${shortcut}`, receipts: routes.map(({ line }) => line) });
  }
  assert.deepEqual(
    compactUnderlay(setup.monitor.latest("modal-boundary-state"), setup.monitor, modalWindow),
    modalBaseline,
    `${modal.kind} modal attempts changed underlay state`,
  );
  const resize = await resizeRoundTrip(page, setup.monitor, modalWindow);
  assert.ok(modal.duplicatePolicy.source.length > 0, `${modal.kind} lacks duplicate-policy source`);
  assert.ok(modal.duplicatePolicy.test.length > 0, `${modal.kind} lacks owning duplicate-NoOp test`);
  const close = await closeAndProbe(page, setup.monitor, modalWindow, underlay);
  const closeRoute = close.closeRoutes.find(({ kind, recipient }) =>
    kind === "key" && recipient === `window:${modalWindow}`);
  const orphanRoute = close.orphanRoutes.find(({ kind }) => kind === "pointer");
  return {
    kind: modal.kind,
    entry: modal.entry,
    duplicate_policy: modal.duplicatePolicy,
    resize,
    blocked_inputs: blockedInputs,
    blocked_underlay_action_delta: 0,
    allowed_child_action: {
      input: "Escape",
      recipient: closeRoute.recipient,
      effect: "closed-modal",
      underlay_action_delta: closeRoute.underlay,
    },
    close_transition: {
      same_frame_receipt: closeRoute.line,
      orphan_release_receipt: orphanRoute.line,
      fresh_root_receipt: close.freshRoute.line,
    },
    modal_window: modalWindow,
    open_event: event.event_id,
    close_frame_assertions: close.assertions,
  };
}

async function setupModalBoundaries(context) {
  return {
    monitor: createMonitor(context.page),
    events: [],
    screenshots: [],
    initialCounters: null,
    results: [],
  };
}

async function runModalInventory(context, setup, inventory, caseId) {
  setup.initialCounters = setup.monitor.counters();
  for (const modal of inventory) setup.results.push(await runOneModal(context, setup, modal));
  return {
    schema_version: WINDOWING_CONTRACT_SCHEMA_VERSION,
    case_id: caseId,
    faction: context.faction,
    viewport: context.viewport,
    events: setup.events,
    action_counters: { before: setup.initialCounters, after: setup.monitor.counters() },
    conditions: ["cold", "warm", "resize"],
    system_finder: { status: "excluded", reason: "no paintable port content" },
    modals: setup.results,
    timeline: STEPS.map((id) => ({ id, assertions: STEP_ASSERTIONS[id] * inventory.length })),
  };
}

function assertModalInventory(context, observation, setup, inventory) {
  assert.deepEqual(observation.modals.map(({ kind }) => kind), inventory.map(({ kind }) => kind));
  assert.deepEqual(observation.conditions, ["cold", "warm", "resize"]);
  assert.deepEqual(observation.system_finder, {
    status: "excluded",
    reason: "no paintable port content",
  });
  assert.equal(observation.events.length, inventory.length);
  assert.ok(observation.events.every(({ observed_actions }) => observed_actions.length === 1));
  for (const modal of observation.modals) {
    assert.equal(modal.blocked_inputs.length, 13, `${modal.kind} omitted a blocked input family`);
    assert.ok(modal.blocked_inputs.every(({ receipts }) => receipts.length > 0));
    assert.equal(modal.blocked_underlay_action_delta, 0);
    assert.equal(modal.allowed_child_action.input, "Escape");
    assert.equal(modal.allowed_child_action.recipient, `window:${modal.modal_window}`);
    assert.equal(modal.allowed_child_action.effect, "closed-modal");
    assert.equal(modal.allowed_child_action.underlay_action_delta, 0);
    assert.match(modal.close_transition.same_frame_receipt, /kind=key/);
    assert.match(modal.close_transition.orphan_release_receipt, /kind=pointer/);
    assert.match(modal.close_transition.fresh_root_receipt, /application=fleet_finder/);
  }
  assert.deepEqual(setup.monitor.errors, []);
  let assertions = 0;
  for (const step of observation.timeline) {
    context.completeStep(step.id, step.assertions);
    assertions += step.assertions;
  }
  return assertions;
}

async function writeEvidence(context, observation, setup, scope) {
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
    assertion_count: observation.timeline.reduce((sum, step) => sum + step.assertions, 0),
    observation,
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
      ...setup.screenshots,
    ].map(([kind, file]) => ({
      kind,
      path: path.relative(context.evidenceRoot, file),
      sha256: sha256(fs.readFileSync(file)),
    })),
  };
}

function modalCase(kind) {
  const modal = AVAILABLE_MODAL_CASES.find((candidate) => candidate.kind === kind);
  return defineWindowingContractCase({
    id: `modal-boundary-${kind}`,
    groups: ["C"],
    steps: STEPS,
    factions: FACTIONS,
    viewports: VIEWPORTS,
    setup: setupModalBoundaries,
    actions: (context, setup) => runModalInventory(context, setup, [modal], `modal-boundary-${kind}`),
    assertions: (context, observation, setup) => assertModalInventory(context, observation, setup, [modal]),
    evidence: (context, observation, setup) => writeEvidence(
      context,
      observation,
      setup,
      `focused ${kind} modal-boundary case; not complete A-E acceptance`,
    ),
  });
}

const journeyC = defineWindowingContractCase({
  id: "journey-c",
  groups: ["C"],
  steps: STEPS,
  factions: FACTIONS,
  viewports: VIEWPORTS,
  setup: setupModalBoundaries,
  actions: (context, setup) => runModalInventory(context, setup, AVAILABLE_MODAL_CASES, "journey-c"),
  assertions: (context, observation, setup) => assertModalInventory(
    context,
    observation,
    setup,
    AVAILABLE_MODAL_CASES,
  ),
  evidence: (context, observation, setup) => writeEvidence(
    context,
    observation,
    setup,
    "focused fixture journey C; not complete A-E acceptance",
  ),
});

const modalBoundaries = defineWindowingContractCase({
  id: "modal-boundaries",
  groups: ["C"],
  steps: STEPS,
  factions: FACTIONS,
  viewports: VIEWPORTS,
  setup: setupModalBoundaries,
  actions: (context, setup) => runModalInventory(
    context,
    setup,
    AVAILABLE_MODAL_CASES,
    "modal-boundaries",
  ),
  assertions: (context, observation, setup) => assertModalInventory(
    context,
    observation,
    setup,
    AVAILABLE_MODAL_CASES,
  ),
  evidence: (context, observation, setup) => writeEvidence(
    context,
    observation,
    setup,
    "focused all-available modal-boundary cases; not complete A-E acceptance",
  ),
});

export const windowingContractCases = Object.freeze([
  ...AVAILABLE_MODAL_CASES.map(({ kind }) => modalCase(kind)),
  journeyC,
  modalBoundaries,
]);
