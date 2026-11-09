export async function chooseMissionTargetMenuRow({
  point,
  click,
  waitForMenuReady,
  waitForMenuClosed,
}) {
  await waitForMenuReady();
  await click(point);
  await waitForMenuClosed();
}

export async function waitForMissionTargetMenuReady(page) {
  try {
    await page.waitForFunction(
      () => window.__openRebellionInterfaceMenuLifecycle?.open === true
        && window.__openRebellionInterfaceMenuLifecycle.input_ready === true,
      null,
      { timeout: 10_000, polling: 100 },
    );
  } catch (error) {
    const [menu, lifecycle] = await page.evaluate(() => [
      window.__openRebellionInterfaceObjectMenu,
      window.__openRebellionInterfaceMenuLifecycle,
    ]).catch(() => [null, null]);
    throw new Error(
      `Mission targeting menu did not become input-ready: ${JSON.stringify({ menu, lifecycle })}`,
      { cause: error },
    );
  }
  return page.evaluate(() => window.__openRebellionInterfaceMenuLifecycle);
}

export async function waitForMissionTargetMenuClosed(page) {
  try {
    await page.waitForFunction(
      () => window.__openRebellionInterfaceMenuLifecycle?.open === false,
      null,
      { timeout: 10_000, polling: 100 },
    );
  } catch (error) {
    const report = await page.evaluate(() => window.__openRebellionInterfaceMenuLifecycle)
      .catch(() => null);
    throw new Error(`Mission targeting menu did not close: ${JSON.stringify(report)}`, { cause: error });
  }
  return page.evaluate(() => window.__openRebellionInterfaceMenuLifecycle);
}
