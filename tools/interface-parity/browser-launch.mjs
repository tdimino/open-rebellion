// Retry only a browser-startup timeout, before a page or application exists.
// A second timeout fails the case. Retain every attempt in the case evidence.
export async function launchBrowser(chromium, options, attempts) {
  for (let attempt = 1; attempt <= 2; attempt += 1) {
    const start = performance.now();
    try {
      const browser = await chromium.launch(options);
      attempts.push({ attempt, status: "pass", duration_ms: performance.now() - start });
      return browser;
    } catch (error) {
      attempts.push({ attempt, status: "fail", duration_ms: performance.now() - start,
        error_name: error.name, error: String(error.stack || error) });
      if (error.name !== "TimeoutError" || attempt === 2) throw error;
      await new Promise((resolve) => setTimeout(resolve, 1_000));
    }
  }
}
