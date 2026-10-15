#!/usr/bin/env node

import fs from "node:fs";
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import http from "node:http";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath, pathToFileURL } from "node:url";

import { chromium } from "playwright-core";

import { launchBrowser } from "./browser-launch.mjs";
import { performCommand } from "./command-events.mjs";

export const WINDOWING_CONTRACT_SCHEMA_VERSION = 1;
export const WINDOWING_CONTRACT_SCENARIO = 65;
export const WINDOWING_CONTRACT_EXTERNAL_CODE = 66;
export const REQUIRED_JOURNEY_GROUPS = Object.freeze(["A", "B", "C", "D", "E"]);
export const FACTIONS = Object.freeze(["alliance", "empire"]);
export const VIEWPORTS = Object.freeze(["native", "letterboxed"]);

const FACTION_BYTES = Object.freeze({ alliance: 1, empire: 2 });
const VIEWPORT_GEOMETRY = Object.freeze({
  native: Object.freeze({ width: 640, height: 480 }),
  letterboxed: Object.freeze({ width: 1280, height: 800 }),
});
const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const site = path.join(root, ".artifacts/interface-parity/site");
const browserManifest = JSON.parse(fs.readFileSync(path.join(here, "browser.json"), "utf8"));

function fail(message) {
  throw new Error(`windowing contract: ${message}`);
}

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function sortedKeys(value) {
  return Object.keys(value).sort();
}

export function encodeFixtureRequest({ scenario, faction, viewport }) {
  if (!Number.isInteger(scenario) || scenario < 0 || scenario > 254) {
    fail(`invalid scenario ${scenario}`);
  }
  const factionByte = FACTION_BYTES[faction];
  if (factionByte === undefined) fail(`invalid faction ${faction}`);
  if (!VIEWPORTS.includes(viewport)) fail(`invalid viewport ${viewport}`);
  return {
    fixtureCode: scenario + 1 | (factionByte << 8),
    viewport,
  };
}

export function decodeFixtureRequest(encoded) {
  if (!isPlainObject(encoded) || !Number.isInteger(encoded.fixtureCode)) {
    fail("encoded fixture request must contain an integer fixtureCode");
  }
  const { fixtureCode, viewport } = encoded;
  if (fixtureCode <= 0 || fixtureCode >>> 16 !== 0) fail(`invalid fixture code ${fixtureCode}`);
  if (!VIEWPORTS.includes(viewport)) fail(`invalid viewport ${viewport}`);
  const externalScenario = fixtureCode & 0xff;
  if (externalScenario === 0) fail("fixture scenario byte must be nonzero");
  const factionByte = fixtureCode >>> 8 & 0xff;
  const faction = Object.entries(FACTION_BYTES)
    .find(([, value]) => value === factionByte)?.[0];
  if (faction === undefined) fail(`invalid faction byte ${factionByte}`);
  return { scenario: externalScenario - 1, faction, viewport };
}

function validateCounters(actionCounters) {
  if (!isPlainObject(actionCounters)
      || !isPlainObject(actionCounters.before)
      || !isPlainObject(actionCounters.after)) {
    fail("action_counters must record before and after objects");
  }
  const beforeKeys = sortedKeys(actionCounters.before);
  const afterKeys = sortedKeys(actionCounters.after);
  if (beforeKeys.length === 0 || beforeKeys.join("\0") !== afterKeys.join("\0")) {
    fail("action counter keys must be identical and nonempty before and after");
  }
  for (const key of beforeKeys) {
    const before = actionCounters.before[key];
    const after = actionCounters.after[key];
    if (!Number.isSafeInteger(before) || before < 0
        || !Number.isSafeInteger(after) || after < before) {
      fail(`invalid action counter ${key}: ${before} -> ${after}`);
    }
  }
}

export function validateObservation(value) {
  if (!isPlainObject(value)) fail("observation must be an object");
  if (value.schema_version !== WINDOWING_CONTRACT_SCHEMA_VERSION) {
    fail(`unsupported observation schema ${value.schema_version}`);
  }
  if (typeof value.case_id !== "string" || value.case_id.length === 0) {
    fail("observation case_id must be a nonempty string");
  }
  if (!FACTIONS.includes(value.faction)) fail(`invalid observation faction ${value.faction}`);
  if (!VIEWPORTS.includes(value.viewport)) fail(`invalid observation viewport ${value.viewport}`);
  if (!Array.isArray(value.events) || value.events.length === 0) {
    fail("observation must contain at least one event recipient");
  }
  const recipients = new Set();
  for (const event of value.events) {
    if (!isPlainObject(event)
        || typeof event.event_id !== "string" || event.event_id.length === 0
        || typeof event.kind !== "string" || event.kind.length === 0
        || typeof event.recipient !== "string" || event.recipient.length === 0) {
      fail("malformed event recipient");
    }
    if (recipients.has(event.event_id)) {
      fail(`duplicate recipient for event ${event.event_id}`);
    }
    recipients.add(event.event_id);
  }
  validateCounters(value.action_counters);
  return value;
}

export function defineWindowingContractCase(definition) {
  if (!isPlainObject(definition)) fail("case definition must be an object");
  if (!/^[a-z][a-z0-9-]*$/.test(definition.id ?? "")) fail(`invalid case id ${definition.id}`);
  if (!Array.isArray(definition.groups) || definition.groups.length === 0
      || definition.groups.some((group) => ![...REQUIRED_JOURNEY_GROUPS, "adapter"].includes(group))) {
    fail(`case ${definition.id} has invalid groups`);
  }
  if (!Array.isArray(definition.factions) || definition.factions.length === 0
      || definition.factions.some((faction) => !FACTIONS.includes(faction))) {
    fail(`case ${definition.id} has invalid factions`);
  }
  if (!Array.isArray(definition.viewports) || definition.viewports.length === 0
      || definition.viewports.some((viewport) => !VIEWPORTS.includes(viewport))) {
    fail(`case ${definition.id} has invalid viewports`);
  }
  for (const hook of ["setup", "actions", "assertions", "evidence"]) {
    if (typeof definition[hook] !== "function") fail(`case ${definition.id} is missing ${hook}()`);
  }
  return Object.freeze({
    ...definition,
    groups: Object.freeze([...new Set(definition.groups)]),
    factions: Object.freeze([...new Set(definition.factions)]),
    viewports: Object.freeze([...new Set(definition.viewports)]),
  });
}

const ADAPTER_SMOKE_ROUTES = Object.freeze({
  "pointer-open": Object.freeze({
    kind: "pointer",
    recipient: "root",
    tokens: Object.freeze(["command=0x132", "destination=gid_menu", "status=opened_original"]),
  }),
  "pointer-select": Object.freeze({
    kind: "pointer",
    recipient: "menu",
    tokens: Object.freeze(["destination=gid", "status=selected", "label=Display Off"]),
  }),
  "key-alt-2": Object.freeze({
    kind: "key",
    recipient: "root",
    tokens: Object.freeze(["destination=gid", "status=selected", "source=accelerator"]),
  }),
});

export function smokeEventFromCommand(eventId, observedCommand) {
  const route = ADAPTER_SMOKE_ROUTES[eventId];
  if (route === undefined) fail(`unknown adapter-smoke event ${eventId}`);
  if (typeof observedCommand !== "string"
      || !observedCommand.includes("[interface] command=")
      || route.tokens.some((token) => !observedCommand.includes(token))) {
    fail(`adapter-smoke event ${eventId} did not observe its application command`);
  }
  return {
    event_id: eventId,
    kind: route.kind,
    recipient: route.recipient,
    observed_command: observedCommand,
  };
}

export function assertAdapterSmokeObservation(value) {
  const observation = validateObservation(value);
  if (observation.case_id !== "adapter-smoke") fail(`unexpected smoke case ${observation.case_id}`);
  for (const eventId of Object.keys(ADAPTER_SMOKE_ROUTES)) {
    const event = observation.events.find(({ event_id: candidate }) => candidate === eventId);
    if (event === undefined) fail(`adapter-smoke observation is missing ${eventId}`);
    const expected = smokeEventFromCommand(eventId, event.observed_command);
    if (event.kind !== expected.kind || event.recipient !== expected.recipient) {
      fail(`adapter-smoke event ${eventId} has the wrong route`);
    }
  }
  if (observation.events.length !== Object.keys(ADAPTER_SMOKE_ROUTES).length) {
    fail("adapter-smoke observation contains unexpected event recipients");
  }
  for (const [counter, expectedDelta] of [["gid_menu", 1], ["gid_selection", 2]]) {
    const before = observation.action_counters.before[counter];
    const after = observation.action_counters.after[counter];
    if (!Number.isSafeInteger(before) || !Number.isSafeInteger(after)
        || after - before !== expectedDelta) {
      fail(`adapter-smoke requires an exact ${counter} action delta of ${expectedDelta}`);
    }
  }
  return 5;
}

export const ADAPTER_SMOKE_CASE = defineWindowingContractCase({
  id: "adapter-smoke",
  groups: ["adapter"],
  factions: FACTIONS,
  viewports: VIEWPORTS,
  setup: async (context) => context.openFixture(),
  actions: async (context) => {
    await context.clickGidMenu();
    await context.pressGidAccelerator();
    return context.observation();
  },
  assertions: async (_context, observation) => assertAdapterSmokeObservation(observation),
  evidence: async (context, observation) => context.writeEvidence({
    observation,
    assertion_count: assertAdapterSmokeObservation(observation),
  }),
});

function uniqueCases(cases) {
  const byId = new Map();
  for (const value of cases) {
    const definition = defineWindowingContractCase(value);
    if (byId.has(definition.id)) fail(`duplicate case id ${definition.id}`);
    byId.set(definition.id, definition);
  }
  return byId;
}

export function selectCases(cases, requestedIds) {
  const byId = uniqueCases(cases);
  if (!Array.isArray(requestedIds)) fail("requested case selection must be an array");
  if (requestedIds.length > 0) {
    return [...new Set(requestedIds)].map((id) => {
      const selected = byId.get(id);
      if (selected === undefined) fail(`unknown case ${id}`);
      return selected;
    });
  }

  const defaults = [...byId.values()].filter(({ groups }) => !groups.includes("adapter"));
  const availableGroups = new Set(defaults.flatMap(({ groups }) => groups));
  const missing = REQUIRED_JOURNEY_GROUPS.filter((group) => !availableGroups.has(group));
  if (missing.length > 0) fail(`missing journey groups ${missing.join(", ")}; default A-E run is unavailable`);
  return defaults;
}

export function windowingContractScope(cases, requestedIds) {
  if (!Array.isArray(cases) || cases.length === 0 || !Array.isArray(requestedIds)) {
    fail("scope requires selected cases and requested case ids");
  }
  const groups = new Set(cases.flatMap(({ groups }) => groups));
  const coversRequiredGroups = REQUIRED_JOURNEY_GROUPS.every((group) => groups.has(group));
  if (requestedIds.length === 0 && coversRequiredGroups) {
    return "complete A-E fixture journey";
  }
  if (cases.every(({ groups: caseGroups }) => caseGroups.includes("adapter"))) {
    return "adapter smoke only; not A-E acceptance";
  }
  return "focused fixture selection; not complete A-E acceptance";
}

export async function runSelectedCases(cases, createContext) {
  if (!Array.isArray(cases) || cases.length === 0) fail("zero executed cases");
  if (typeof createContext !== "function") fail("createContext must be a function");
  let executedCases = 0;
  let assertionCount = 0;
  const executions = [];
  for (const definition of cases) {
    for (const faction of definition.factions) {
      for (const viewport of definition.viewports) {
        const context = await createContext({ definition, faction, viewport });
        try {
          const setup = await definition.setup(context);
          const observation = validateObservation(await definition.actions(context, setup));
          if (observation.case_id !== definition.id
              || observation.faction !== faction
              || observation.viewport !== viewport) {
            fail(`observation identity does not match ${definition.id}/${faction}/${viewport}`);
          }
          const assertions = await definition.assertions(context, observation, setup);
          if (!Number.isSafeInteger(assertions) || assertions <= 0) {
            fail(`case ${definition.id}/${faction}/${viewport} reported zero assertions`);
          }
          const evidence = await definition.evidence(context, observation, setup);
          executedCases += 1;
          assertionCount += assertions;
          executions.push({ case_id: definition.id, faction, viewport, assertions, evidence });
        } finally {
          await context?.cleanup?.();
        }
      }
    }
  }
  if (executedCases === 0) fail("zero executed cases");
  if (assertionCount === 0) fail("selected cases reported zero assertions");
  return { executed_cases: executedCases, assertion_count: assertionCount, executions };
}

export function isCaseModuleFilename(filename) {
  return /^windowing-contract-[a-z0-9-]+\.mjs$/.test(filename)
    && !filename.endsWith(".test.mjs");
}

export async function discoverCaseModules(directory) {
  const discovered = [];
  for (const filename of fs.readdirSync(directory).filter(isCaseModuleFilename).sort()) {
    const module = await import(pathToFileURL(path.join(directory, filename)));
    const exported = module.windowingContractCases ?? [module.windowingContractCase];
    if (!Array.isArray(exported) || exported.length === 0
        || exported.some((value) => value === undefined)) {
      fail(`${filename} must export at least one case via windowingContractCase or windowingContractCases`);
    }
    discovered.push(...exported.map(defineWindowingContractCase));
  }
  return discovered;
}

export function parseWindowingContractArgs(argv) {
  let noBuild = false;
  const caseIds = [];
  for (let index = 0; index < argv.length; index += 1) {
    const argument = argv[index];
    if (argument === "--no-build") {
      noBuild = true;
    } else if (argument === "--case") {
      const value = argv[index + 1];
      if (!value || value.startsWith("--")) fail("--case requires a case id");
      caseIds.push(...value.split(",").filter(Boolean));
      index += 1;
    } else if (argument.startsWith("--case=")) {
      const value = argument.slice("--case=".length);
      if (!value) fail("--case requires a case id");
      caseIds.push(...value.split(",").filter(Boolean));
    } else {
      fail(`unknown argument ${argument}`);
    }
  }
  return { noBuild, caseIds };
}

export function strategicGidPoint(faction, viewport) {
  const geometry = VIEWPORT_GEOMETRY[viewport];
  if (geometry === undefined) fail(`invalid viewport ${viewport}`);
  const control = faction === "alliance"
    ? { x: 446, y: 406, width: 27, height: 16 }
    : faction === "empire"
      ? { x: 519, y: 434, width: 37, height: 25 }
      : fail(`invalid faction ${faction}`);
  const scale = Math.min(geometry.width / 640, geometry.height / 480);
  const offsetX = (geometry.width - 640 * scale) / 2;
  const offsetY = (geometry.height - 480 * scale) / 2;
  return {
    x: offsetX + (control.x + control.width / 2) * scale,
    y: offsetY + (control.y + control.height / 2) * scale,
  };
}

export function strategicGidDisplayOffPoint(viewport) {
  const geometry = VIEWPORT_GEOMETRY[viewport];
  if (geometry === undefined) fail(`invalid viewport ${viewport}`);
  const scale = Math.min(geometry.width / 640, geometry.height / 480);
  const offsetX = (geometry.width - 640 * scale) / 2;
  const offsetY = (geometry.height - 480 * scale) / 2;
  return {
    x: offsetX + 477 * scale,
    y: offsetY + 372 * scale,
  };
}

export function smokeActionCounters(lines) {
  return {
    gid_menu: lines.filter((line) => line.includes("destination=gid_menu status=")).length,
    gid_selection: lines.filter((line) => line.includes("destination=gid status=selected")).length,
  };
}

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

function frames(page, count = 2) {
  return page.evaluate((frameCount) => new Promise((resolve) => {
    function next(remaining) {
      if (remaining === 0) resolve();
      else requestAnimationFrame(() => next(remaining - 1));
    }
    next(frameCount);
  }), count);
}

function browserExecutable() {
  const candidates = [
    ...(process.env.OPEN_REBELLION_CHROME_FOR_TESTING
      ? [process.env.OPEN_REBELLION_CHROME_FOR_TESTING]
      : []),
    ...browserManifest.executable_candidates,
  ];
  const executable = candidates.find((candidate) => fs.existsSync(candidate));
  if (!executable) fail(`pinned Chrome for Testing ${browserManifest.version} is missing; see README.md`);
  const version = spawnSync(executable, ["--version"], { encoding: "utf8" });
  if (version.status !== 0 || !version.stdout.includes(browserManifest.version)) {
    fail(`Chrome for Testing version mismatch: ${version.stdout || version.stderr}`);
  }
  return executable;
}

function mimeType(file) {
  if (file.endsWith(".html")) return "text/html; charset=utf-8";
  if (file.endsWith(".js")) return "text/javascript; charset=utf-8";
  if (file.endsWith(".wasm")) return "application/wasm";
  return "application/octet-stream";
}

async function startServer() {
  const server = http.createServer((request, response) => {
    const pathname = new URL(request.url, "http://localhost").pathname;
    const relative = path.posix.normalize(decodeURIComponent(pathname)).replace(/^\/+/, "")
      || "index.html";
    const candidate = path.resolve(site, relative);
    if (!candidate.startsWith(`${site}${path.sep}`)) {
      response.writeHead(403).end();
      return;
    }
    fs.readFile(candidate, (error, bytes) => {
      if (error) {
        response.writeHead(404).end();
        return;
      }
      response.writeHead(200, {
        "content-type": mimeType(candidate),
        "content-length": bytes.length,
        "cache-control": "no-store",
      }).end(bytes);
    });
  });
  await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
  return server;
}

function buildFixtureSite() {
  const built = spawnSync("npm", ["run", "build"], { cwd: here, stdio: "inherit" });
  if (built.status !== 0) fail(`fixture build failed with exit ${built.status}`);
}

function assertFixtureSite() {
  for (const file of ["index.html", "gl.js", "open-rebellion-test.wasm", "data/runtime.orpk"]) {
    if (!fs.existsSync(path.join(site, file))) fail(`fixture site is missing ${file}; run npm run build`);
  }
}

function createRunId() {
  return `${new Date().toISOString().replace(/[:.]/g, "-")}-${process.pid}`;
}

async function closeHttpServer(server) {
  if (server === undefined) return;
  server.closeIdleConnections?.();
  server.closeAllConnections?.();
  if (server.listening === false) return;
  await new Promise((resolve, reject) => server.close((error) => {
    if (error && error.code !== "ERR_SERVER_NOT_RUNNING") reject(error);
    else resolve();
  }));
}

export async function acquireBrowserPage(launch, contextOptions) {
  let browser;
  let browserContext;
  try {
    browser = await launch();
    browserContext = await browser.newContext(contextOptions);
    const page = await browserContext.newPage();
    return { browser, browserContext, page };
  } catch (error) {
    await browserContext?.close().catch(() => {});
    await browser?.close().catch(() => {});
    throw error;
  }
}

export async function createBrowserRuntime(runDirectory, dependencies = {}) {
  const assertSite = dependencies.assertSite ?? assertFixtureSite;
  const resolveBrowser = dependencies.resolveBrowser ?? browserExecutable;
  const serve = dependencies.serve ?? startServer;
  const launcher = dependencies.launcher ?? launchBrowser;
  assertSite();
  const executablePath = await resolveBrowser();
  let server;
  try {
    server = await serve();
    const address = server.address();
    if (!isPlainObject(address) || !Number.isInteger(address.port)) {
      fail("fixture server did not report a listening port");
    }
    const origin = `http://127.0.0.1:${address.port}`;

    return {
      async createContext({ definition, faction, viewport }) {
        const directory = path.join(runDirectory, definition.id, faction, viewport);
        fs.mkdirSync(directory, { recursive: true });
        const launchAttempts = [];
        const geometry = VIEWPORT_GEOMETRY[viewport];
        const { browser, browserContext, page } = await acquireBrowserPage(
          () => launcher(chromium, {
            executablePath,
            headless: true,
            args: browserManifest.launch_arguments,
            timeout: 30_000,
          }, launchAttempts),
          {
            viewport: geometry,
            deviceScaleFactor: 1,
            locale: "en-US",
            timezoneId: "America/New_York",
            colorScheme: "dark",
            reducedMotion: "reduce",
          },
        );
        const consoleLines = [];
        const errors = [];
        const requests = [];
        const events = [];
        let before = null;
        let ready = null;
        page.on("console", (message) => {
          const line = message.text();
          consoleLines.push(line);
          if (message.type() === "error") errors.push(`console:${line}`);
        });
        page.on("pageerror", (error) => errors.push(`page:${error.stack || error}`));
        page.on("response", (response) => {
          if (new URL(response.url()).origin === origin) {
            requests.push({ path: new URL(response.url()).pathname, status: response.status() });
          }
        });

        return {
          page,
          directory,
          faction,
          viewport,
          async openFixture() {
            const encoded = encodeFixtureRequest({
              scenario: WINDOWING_CONTRACT_SCENARIO,
              faction,
              viewport,
            });
            await page.goto(`${origin}/?fixture-code=${encoded.fixtureCode}`, {
              waitUntil: "load",
              timeout: 30_000,
            });
            await page.waitForFunction(() => window.__openRebellionInterfaceReady?.status, null, {
              timeout: 60_000,
            });
            ready = await page.evaluate(() => window.__openRebellionInterfaceReady);
            assert.equal(ready.status, "ready", JSON.stringify(ready));
            assert.equal(ready.code, encoded.fixtureCode);
            assert.equal(ready.scenario, WINDOWING_CONTRACT_SCENARIO);
            await page.evaluate(() => document.fonts.ready);
            await frames(page);
            before = smokeActionCounters(consoleLines);
            const screenshot = await page.screenshot({ animations: "disabled" });
            fs.writeFileSync(path.join(directory, "before.png"), screenshot);
            return ready;
          },
          async clickGidMenu() {
            const point = strategicGidPoint(faction, viewport);
            await page.mouse.move(point.x, point.y);
            await frames(page);
            await page.mouse.down();
            await frames(page);
            const opened = await performCommand(
              page,
              "command=0x132 destination=gid_menu status=opened_original",
              () => page.mouse.up(),
            );
            events.push(smokeEventFromCommand("pointer-open", opened));
            const displayOff = strategicGidDisplayOffPoint(viewport);
            const selected = await performCommand(
              page,
              "destination=gid status=selected label=Display Off",
              () => page.mouse.click(displayOff.x, displayOff.y),
            );
            events.push(smokeEventFromCommand("pointer-select", selected));
          },
          async pressGidAccelerator() {
            await page.keyboard.down("Alt");
            await frames(page);
            let selected;
            try {
              selected = await performCommand(
                page,
                "destination=gid status=selected",
                () => page.keyboard.down("2"),
              );
            } finally {
              await page.keyboard.up("2").catch(() => {});
              await frames(page);
              await page.keyboard.up("Alt").catch(() => {});
              await frames(page);
            }
            events.push(smokeEventFromCommand("key-alt-2", selected));
          },
          observation() {
            if (before === null || ready === null) fail("adapter-smoke observed before fixture setup");
            return validateObservation({
              schema_version: WINDOWING_CONTRACT_SCHEMA_VERSION,
              case_id: definition.id,
              faction,
              viewport,
              events,
              action_counters: { before, after: smokeActionCounters(consoleLines) },
            });
          },
          async writeEvidence({ observation, assertion_count: assertionCount }) {
            await frames(page);
            const screenshot = await page.screenshot({ animations: "disabled" });
            const screenshotPath = path.join(directory, "after.png");
            fs.writeFileSync(screenshotPath, screenshot);
            const result = {
              schema_version: WINDOWING_CONTRACT_SCHEMA_VERSION,
              family: "strategic-windowing-contract",
              scope: "adapter smoke only; not A-E acceptance",
              status: "pass",
              assertion_count: assertionCount,
              observation,
              ready,
              browser_version: browser.version(),
              browser_executable: executablePath,
              launch_arguments: browserManifest.launch_arguments,
              launch_attempts: launchAttempts,
              viewport: { ...VIEWPORT_GEOMETRY[viewport], device_scale_factor: 1 },
              muted: browserManifest.launch_arguments.includes("--mute-audio"),
              requests,
              errors,
              wasm_sha256: sha256(fs.readFileSync(path.join(site, "open-rebellion-test.wasm"))),
              screenshot_sha256: sha256(screenshot),
            };
            assert.ok(requests.length > 0, "adapter smoke recorded no fixture requests");
            assert.ok(requests.every(({ status }) => status === 200), JSON.stringify(requests));
            assert.deepEqual(errors, []);
            fs.writeFileSync(path.join(directory, "result.json"), `${JSON.stringify(result, null, 2)}\n`);
            return {
              result_json: path.relative(runDirectory, path.join(directory, "result.json")),
              screenshot_sha256: result.screenshot_sha256,
              wasm_sha256: result.wasm_sha256,
            };
          },
          async cleanup() {
            await browserContext.close().catch(() => {});
            await browser.close().catch(() => {});
          },
        };
      },
      async close() {
        await closeHttpServer(server);
      },
    };
  } catch (error) {
    await closeHttpServer(server).catch(() => {});
    throw error;
  }
}

export async function main(argv = process.argv.slice(2)) {
  const options = parseWindowingContractArgs(argv);
  if (!options.noBuild) buildFixtureSite();
  const discovered = await discoverCaseModules(here);
  const selected = selectCases([ADAPTER_SMOKE_CASE, ...discovered], options.caseIds);
  const runDirectory = path.join(root, ".artifacts/interface-parity", `windowing-contract-${createRunId()}`);
  fs.mkdirSync(runDirectory, { recursive: true });
  const runtime = await createBrowserRuntime(runDirectory);
  try {
    const summary = await runSelectedCases(selected, (selection) => runtime.createContext(selection));
    const result = {
      schema_version: WINDOWING_CONTRACT_SCHEMA_VERSION,
      family: "strategic-windowing-contract",
      scope: windowingContractScope(selected, options.caseIds),
      status: "pass",
      ...summary,
    };
    fs.writeFileSync(path.join(runDirectory, "result.json"), `${JSON.stringify(result, null, 2)}\n`);
    process.stdout.write(`${JSON.stringify({ ...result, run_directory: runDirectory }, null, 2)}\n`);
    return result;
  } finally {
    await runtime.close();
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href) {
  main().catch((error) => {
    process.stderr.write(`${error.stack || error}\n`);
    process.exitCode = 1;
  });
}
