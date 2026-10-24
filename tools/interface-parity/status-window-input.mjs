import { clickWithRouteBarrier } from "./fleet-window.mjs";

export async function clickStatusClose({
  page,
  point,
  waitForClose,
  advanceFrame,
  settleFrames = 3,
}) {
  const nextFrame = advanceFrame ?? (() => page.evaluate(() => new Promise((resolve) => {
    requestAnimationFrame(() => requestAnimationFrame(resolve));
  })));
  await page.mouse.move(point.x, point.y);
  await nextFrame();
  // Closing removes the Status window before routed pointer receipts are
  // published. Keep egui's click in one input batch, then wait for the
  // semantic close receipt instead of requiring receipts from a dead owner.
  await page.evaluate(({ x, y }) => {
    const canvas = document.getElementById("glcanvas");
    if (!(canvas instanceof HTMLCanvasElement)) throw new Error("missing game canvas");
    const emit = (type, buttons) => canvas.dispatchEvent(new MouseEvent(type, {
      bubbles: true,
      cancelable: true,
      view: window,
      clientX: x,
      clientY: y,
      screenX: x,
      screenY: y,
      button: 0,
      buttons,
      detail: 1,
    }));
    emit("mousedown", 1);
    emit("mouseup", 0);
  }, point);
  for (let frame = 0; frame < settleFrames; frame += 1) await nextFrame();
  return waitForClose();
}

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
