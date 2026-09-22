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

/**
 * Aggregate launch and execution evidence without discarding attempts from a
 * recovered whole-scenario retry. Each logical result retains the final
 * `launch_attempts` for compatibility, while `execution_attempts` is the
 * canonical record of every browser execution.
 */
export function summarizeBrowserAttempts(results) {
  const executions = results.flatMap((result) => result.execution_attempts ?? [{
    cleanup: result.cleanup,
    retryable_timeout: false,
    launch_attempts: result.launch_attempts ?? [],
  }]);
  const launches = executions.flatMap(({ launch_attempts }) => launch_attempts ?? []);

  return {
    browser_executions: executions.length,
    muted_launches: launches.length,
    launch_timeouts: launches.filter(({ error_name }) => error_name === "TimeoutError").length,
    recovered_launch_timeouts: executions.filter(({ launch_attempts = [] }) =>
      launch_attempts.length > 1 && launch_attempts.at(-1).status === "pass").length,
    execution_timeouts: executions.filter(({ retryable_timeout }) => retryable_timeout).length,
    recovered_execution_timeouts: results.filter((result) =>
      (result.execution_attempts?.length ?? 0) > 1 && result.status === "pass").length,
    browser_processes_closed: executions.filter(({ cleanup, launch_attempts = [] }) =>
      cleanup === "closed" && launch_attempts.some(({ status }) => status === "pass")).length,
    browser_executions_closed: executions.filter(({ cleanup }) => cleanup === "closed").length,
  };
}
