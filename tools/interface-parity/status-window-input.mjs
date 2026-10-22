import { clickWithRouteBarrier } from "./fleet-window.mjs";

export async function clickStatusDirect({ page, point, button = "left", advanceFrame, settleFrames = 3 }) {
  const nextFrame = advanceFrame ?? (() => page.evaluate(() => new Promise((resolve) => {
    requestAnimationFrame(() => requestAnimationFrame(resolve));
  })));
  await page.mouse.move(point.x, point.y);
  await nextFrame();
  await page.mouse.down({ button });
  await nextFrame();
  await page.mouse.up({ button });
  for (let frame = 0; frame < settleFrames; frame += 1) await nextFrame();
}

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
