import { clickWithRouteBarrier } from "./fleet-window.mjs";

export function appendStatusConsoleLine({ consoleLines, routes, line }) {
  consoleLines.push(line);
  if (line.startsWith("[windowing-route]")) routes.push(line);
}

export async function clickStatusWithRouteBarrier({
  page,
  routes,
  cursor = { physical: null },
  point,
  label,
  button = "left",
  expectedMove = { application: "pointer_ready" },
  expectedDown,
  expectedRelease,
  skipMoveReceipt = false,
  advanceFrame,
  settleFrames = 3,
}) {
  return clickWithRouteBarrier({
    page,
    routes,
    cursor,
    point,
    label,
    button,
    expectedMove,
    expectedDown,
    expectedRelease,
    skipMoveReceipt,
    advanceFrame,
    settleFrames,
  });
}
