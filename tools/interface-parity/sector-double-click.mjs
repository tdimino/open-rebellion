// Deliver the original Sector icon double click as one browser input batch.
// Separate Playwright down/up awaits can paint between events and lose egui's
// aggregate before legacy fixture-only Sector publications consume it.
export async function sectorDoubleClick(page, point, advanceFrame) {
  await page.mouse.move(point.x, point.y);
  await advanceFrame();
  await page.evaluate(({ x, y }) => {
    const canvas = document.getElementById("glcanvas");
    if (!(canvas instanceof HTMLCanvasElement)) throw new Error("missing game canvas");
    const emit = (type, buttons, detail) => canvas.dispatchEvent(new MouseEvent(type, {
      bubbles: true,
      cancelable: true,
      view: window,
      clientX: x,
      clientY: y,
      screenX: x,
      screenY: y,
      button: 0,
      buttons,
      detail,
    }));
    emit("mousedown", 1, 1);
    emit("mouseup", 0, 1);
    emit("mousedown", 1, 2);
    emit("mouseup", 0, 2);
  }, point);
  await advanceFrame();
}
