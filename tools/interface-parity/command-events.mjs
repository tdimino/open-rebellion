// Register before the action so synchronous events are captured, and only a
// fresh event can satisfy each click. The event is emitted during game update;
// wait through the following paint before inspecting or interacting with UI.
export async function performCommand(page, expected, action, timeout = 2_000) {
  let listener;
  let timer;
  const deadline = new Promise((_, reject) => {
    timer = setTimeout(() => reject(new Error(
      `Timed out after ${timeout} ms waiting for fresh interface event and paint: ${expected}`,
    )), timeout);
  });
  const observed = new Promise((resolve) => {
    listener = (message) => {
      const text = message.text();
      if (text.includes("[interface] command=") && text.includes(expected)) {
        resolve(text);
      }
    };
    page.on("console", listener);
  });
  try {
    return await Promise.race([
      (async () => {
        const [text] = await Promise.all([observed, Promise.resolve().then(action)]);
        await page.evaluate(() => new Promise((resolve) => {
          requestAnimationFrame(() => requestAnimationFrame(resolve));
        }));
        return text;
      })(),
      deadline,
    ]);
  } finally {
    clearTimeout(timer);
    page.off("console", listener);
  }
}
