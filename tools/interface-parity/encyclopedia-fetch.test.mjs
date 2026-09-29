import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import vm from "node:vm";
import { fileURLToPath } from "node:url";
import { selfTestEncyclopediaFetchBrowserHarness } from "./encyclopedia-fetch-browser.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "../..");
const productionHtmlPath = path.join(root, "web/index.html");
const mainSourcePath = path.join(root, "crates/rebellion-app/src/main.rs");
const prepareSitePath = path.join(here, "prepare-site.mjs");

const STATE_UNKNOWN = -1;
const STATE_PENDING = 0;
const STATE_READY = 1;
const STATE_NOT_FOUND = 2;
const STATE_HTTP_STATUS = 3;
const STATE_TRANSPORT = 4;
const STATE_RESOURCE_LIMIT = 5;
const STATE_UNSUPPORTED_STREAMING = 6;

const encoder = new TextEncoder();

function bridgeScripts(html) {
  return [...html.matchAll(/<script(?<attributes>[^>]*)>(?<body>[\s\S]*?)<\/script>/g)]
    .filter(match => match.groups.body.includes("registerEncyclopediaFetchBridge"));
}

function loadBridge(fetchImplementation, { Uint8ArrayImplementation = Uint8Array } = {}) {
  const html = fs.readFileSync(productionHtmlPath, "utf8");
  const matches = bridgeScripts(html);
  assert.equal(matches.length, 1, "production HTML must register exactly one fetch bridge");
  assert.doesNotMatch(matches[0].groups.attributes, /\bsrc\s*=/i, "fetch bridge must remain inline");

  let plugin;
  const memory = new WebAssembly.Memory({ initial: 2 });
  const context = vm.createContext({
    AbortController,
    ArrayBuffer,
    Promise,
    TextDecoder,
    Uint8Array: Uint8ArrayImplementation,
    fetch: fetchImplementation,
    miniquad_add_plugin(candidate) {
      plugin = candidate;
    },
    wasm_memory: memory,
  });
  vm.runInContext(matches[0].groups.body, context, { filename: productionHtmlPath });
  assert.equal(plugin?.name, "open_rebellion_encyclopedia_fetch");

  const imports = { env: {} };
  plugin.register_plugin(imports);
  return { api: imports.env, memory };
}

function startRequest(bridge, fetchPath, maxBytes) {
  const pathBytes = encoder.encode(fetchPath);
  new Uint8Array(bridge.memory.buffer, 0, pathBytes.length).set(pathBytes);
  return bridge.api.open_rebellion_encyclopedia_fetch_start(0, pathBytes.length, maxBytes);
}

async function waitForTerminal(api, handle) {
  for (let attempts = 0; attempts < 100; attempts += 1) {
    const state = api.open_rebellion_encyclopedia_fetch_poll(handle);
    if (state !== STATE_PENDING) {
      return state;
    }
    await new Promise(resolve => setImmediate(resolve));
  }
  throw new Error(`fetch handle ${handle} did not reach a terminal state`);
}

function controlledResponse({ status = 200, chunks = [], readError, pending = false } = {}) {
  const observations = {
    bodyCancelCalls: 0,
    readerCancelCalls: 0,
    readCalls: 0,
  };
  let chunkIndex = 0;
  const reader = {
    async read() {
      observations.readCalls += 1;
      if (readError && observations.readCalls > chunks.length) {
        throw readError;
      }
      if (chunkIndex < chunks.length) {
        const value = Uint8Array.from(chunks[chunkIndex]);
        chunkIndex += 1;
        return { done: false, value };
      }
      if (pending) {
        return new Promise(() => {});
      }
      return { done: true, value: undefined };
    },
    async cancel() {
      observations.readerCancelCalls += 1;
    },
  };
  const body = {
    async cancel() {
      observations.bodyCancelCalls += 1;
    },
    getReader() {
      return reader;
    },
  };
  return {
    observations,
    response: {
      body,
      headers: new Map(),
      ok: status >= 200 && status < 300,
      status,
    },
  };
}

function copyReadyBytes(bridge, handle) {
  const length = bridge.api.open_rebellion_encyclopedia_fetch_detail(handle);
  const destination = 4096;
  assert.equal(
    bridge.api.open_rebellion_encyclopedia_fetch_copy(handle, destination, length),
    1,
  );
  return new Uint8Array(bridge.memory.buffer, destination, length).slice();
}

test("the production and generated fixture shells retain one inline fetch bridge", () => {
  const productionHtml = fs.readFileSync(productionHtmlPath, "utf8");
  assert.equal(bridgeScripts(productionHtml).length, 1);

  const loadMarker = "    <script>load(\"open-rebellion.wasm\");</script>";
  assert.ok(productionHtml.includes(loadMarker), "production load marker remains available");
  const fixtureHtml = productionHtml.replace(
    loadMarker,
    "    <script>load(\"open-rebellion-test.wasm\");</script>",
  );
  assert.equal(bridgeScripts(fixtureHtml).length, 1, "fixture substitution must retain the bridge");

  const prepareSite = fs.readFileSync(prepareSitePath, "utf8");
  assert.match(prepareSite, /productionHtml\.replace\(marker, fixturePlugin\)/);

  const mainSource = fs.readFileSync(mainSourcePath, "utf8");
  assert.match(
    mainSource,
    /#\[cfg\(any\(target_arch = "wasm32", test\)\)\][\s\S]*?mod encyclopedia_fetch;/,
    "native production builds must not import the browser bridge",
  );
});

test("only 404 is absence while other HTTP status codes remain distinct", async () => {
  for (const [status, expectedState] of [
    [404, STATE_NOT_FOUND],
    [403, STATE_HTTP_STATUS],
    [500, STATE_HTTP_STATUS],
  ]) {
    const controlled = controlledResponse({ status });
    let signal;
    const bridge = loadBridge(async (_path, options) => {
      signal = options.signal;
      return controlled.response;
    });
    const handle = startRequest(bridge, `/status-${status}`, 16);
    assert.equal(await waitForTerminal(bridge.api, handle), expectedState);
    assert.equal(controlled.observations.readCalls, 0, "HTTP errors must be classified before reads");
    assert.equal(controlled.observations.bodyCancelCalls, 1, "HTTP response body must be cancelled");
    assert.equal(signal.aborted, true, "terminal HTTP response must release the request");
    if (expectedState === STATE_HTTP_STATUS) {
      assert.equal(bridge.api.open_rebellion_encyclopedia_fetch_detail(handle), status);
    }
    assert.equal(bridge.api.open_rebellion_encyclopedia_fetch_release(handle), 1);
    assert.equal(bridge.api.open_rebellion_encyclopedia_fetch_poll(handle), STATE_UNKNOWN);
  }
});

test("200 HTML and misleading Content-Length remain bounded raw transport bytes", async () => {
  const html = encoder.encode("<!doctype html><title>not an image</title>");
  const controlled = controlledResponse({ chunks: [html.subarray(0, 9), html.subarray(9)] });
  controlled.response.headers = new Map([["Content-Length", "1"]]);
  const bridge = loadBridge(async () => controlled.response);
  const handle = startRequest(bridge, "/missing-route-that-serves-index", html.length);

  assert.equal(await waitForTerminal(bridge.api, handle), STATE_READY);
  assert.deepEqual(copyReadyBytes(bridge, handle), html);
  assert.equal(bridge.api.open_rebellion_encyclopedia_fetch_release(handle), 1);
  assert.equal(bridge.api.open_rebellion_encyclopedia_fetch_release(handle), 0);
});

test("actual streamed bytes accept the exact limit and cancel one byte above it", async () => {
  const exact = controlledResponse({ chunks: [[1, 2], [3, 4]] });
  exact.response.headers = new Map([["Content-Length", "999999"]]);
  const exactBridge = loadBridge(async () => exact.response);
  const exactHandle = startRequest(exactBridge, "/exact.bin", 4);
  assert.equal(await waitForTerminal(exactBridge.api, exactHandle), STATE_READY);
  assert.deepEqual([...copyReadyBytes(exactBridge, exactHandle)], [1, 2, 3, 4]);
  exactBridge.api.open_rebellion_encyclopedia_fetch_release(exactHandle);

  const above = controlledResponse({ chunks: [[1, 2], [3, 4, 5]] });
  above.response.headers = new Map();
  let signal;
  const aboveBridge = loadBridge(async (_path, options) => {
    signal = options.signal;
    return above.response;
  });
  const aboveHandle = startRequest(aboveBridge, "/above.bin", 4);
  assert.equal(await waitForTerminal(aboveBridge.api, aboveHandle), STATE_RESOURCE_LIMIT);
  assert.equal(above.observations.readerCancelCalls, 1);
  assert.equal(signal.aborted, true);
  assert.equal(aboveBridge.api.open_rebellion_encyclopedia_fetch_copy(aboveHandle, 4096, 4), 0);
  aboveBridge.api.open_rebellion_encyclopedia_fetch_release(aboveHandle);
});

test("network failures, thrown reads, and missing streaming support fail explicitly", async () => {
  let networkSignal;
  const networkBridge = loadBridge(async (_path, options) => {
    networkSignal = options.signal;
    throw new Error("offline");
  });
  const networkHandle = startRequest(networkBridge, "/network.bin", 8);
  assert.equal(await waitForTerminal(networkBridge.api, networkHandle), STATE_TRANSPORT);
  assert.equal(networkSignal.aborted, true);
  networkBridge.api.open_rebellion_encyclopedia_fetch_release(networkHandle);

  const throwing = controlledResponse({ chunks: [[1]], readError: new Error("stream reset") });
  const throwingBridge = loadBridge(async () => throwing.response);
  const throwingHandle = startRequest(throwingBridge, "/throws.bin", 8);
  assert.equal(await waitForTerminal(throwingBridge.api, throwingHandle), STATE_TRANSPORT);
  assert.equal(throwing.observations.readerCancelCalls, 1);
  throwingBridge.api.open_rebellion_encyclopedia_fetch_release(throwingHandle);

  const unsupportedObservations = { cancelCalls: 0 };
  let unsupportedSignal;
  const unsupportedBridge = loadBridge(async (_path, options) => {
    unsupportedSignal = options.signal;
    return {
      body: {
        async cancel() {
          unsupportedObservations.cancelCalls += 1;
        },
      },
      ok: true,
      status: 200,
    };
  });
  const unsupportedHandle = startRequest(unsupportedBridge, "/unsupported.bin", 8);
  assert.equal(
    await waitForTerminal(unsupportedBridge.api, unsupportedHandle),
    STATE_UNSUPPORTED_STREAMING,
  );
  assert.equal(unsupportedObservations.cancelCalls, 1);
  assert.equal(unsupportedSignal.aborted, true);
  unsupportedBridge.api.open_rebellion_encyclopedia_fetch_release(unsupportedHandle);
});

test("releasing a pending request aborts, cancels, and drops all handle state", async () => {
  const pending = controlledResponse({ pending: true });
  let signal;
  const bridge = loadBridge(async (_path, options) => {
    signal = options.signal;
    return pending.response;
  });
  const handle = startRequest(bridge, "/slow.bin", 8);

  while (pending.observations.readCalls === 0) {
    await new Promise(resolve => setImmediate(resolve));
  }
  assert.equal(bridge.api.open_rebellion_encyclopedia_fetch_release(handle), 1);
  assert.equal(signal.aborted, true);
  assert.equal(pending.observations.readerCancelCalls, 1);
  assert.equal(bridge.api.open_rebellion_encyclopedia_fetch_poll(handle), STATE_UNKNOWN);
  assert.equal(bridge.api.open_rebellion_encyclopedia_fetch_detail(handle), 0);
  assert.equal(bridge.api.open_rebellion_encyclopedia_fetch_release(handle), 0);
});

test("a response arriving after release is cancelled without restoring handle state", async () => {
  let resolveFetch;
  let signal;
  const bridge = loadBridge((_path, options) => {
    signal = options.signal;
    return new Promise(resolve => {
      resolveFetch = resolve;
    });
  });
  const handle = startRequest(bridge, "/late.bin", 8);
  assert.equal(bridge.api.open_rebellion_encyclopedia_fetch_release(handle), 1);
  assert.equal(signal.aborted, true);

  const late = controlledResponse({ chunks: [[1, 2, 3]] });
  resolveFetch(late.response);
  await new Promise(resolve => setImmediate(resolve));

  assert.equal(late.observations.bodyCancelCalls, 1);
  assert.equal(late.observations.readCalls, 0);
  assert.equal(bridge.api.open_rebellion_encyclopedia_fetch_poll(handle), STATE_UNKNOWN);
});

test("zero-length stream chunks consume no retained-buffer objects", async () => {
  let retainedChunkCopies = 0;
  const instrumentedBytes = new Proxy(Uint8Array, {
    get(target, property) {
      if (property === "from") {
        return value => {
          retainedChunkCopies += 1;
          return Uint8Array.from(value);
        };
      }
      return Reflect.get(target, property);
    },
  });
  let remainingEmptyChunks = 10_000;
  const response = new Response(new ReadableStream({
    pull(controller) {
      if (remainingEmptyChunks > 0) {
        remainingEmptyChunks -= 1;
        controller.enqueue(new Uint8Array(0));
      } else {
        controller.close();
      }
    },
  }), { status: 200 });
  const bridge = loadBridge(async () => response, {
    Uint8ArrayImplementation: instrumentedBytes,
  });
  const handle = startRequest(bridge, "/empty-chunks.bin", 0);

  assert.equal(await waitForTerminal(bridge.api, handle), STATE_READY);
  assert.equal(bridge.api.open_rebellion_encyclopedia_fetch_detail(handle), 0);
  assert.equal(retainedChunkCopies, 0, "empty chunks must not allocate retained copies");
  assert.equal(bridge.api.open_rebellion_encyclopedia_fetch_release(handle), 1);
});

test("the browser harness serves controlled transport cases and cleans up", async () => {
  assert.deepEqual(await selfTestEncyclopediaFetchBrowserHarness(), {
    cleanup: "closed",
    compressed_bytes: 128,
    exact_bytes: [1, 2, 3, 4],
    html_has_one_inline_bridge: true,
    network_failed: true,
    slow_cancelled: true,
    statuses: [404, 403, 500],
    stream_failed: true,
  });
});
