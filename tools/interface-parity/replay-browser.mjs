#!/usr/bin/env node
// The browser leg of scripts/check-replay-equivalence.py: serve web/ on a
// loopback port, load one page of it in the gates' pinned, muted Chrome for
// Testing and print one JSON line with the replay report, the page state,
// page errors, the same-origin request paths (in request order) and
// responses, and every request to another origin.
//
//   node tools/interface-parity/replay-browser.mjs QUERY [--expect-report]
//     [--abort-runtime-pack] [--dismiss-intro]
//
// --dismiss-intro: ordinary startup plays the intro movies 0x65 (000.webm)
// and 0x66 (001.webm) before the menu (FUN_00414830); press Escape on each
// once its request appears, which ends it (FUN_00421790 returns on
// WM_KEYDOWN for every movie but 0x70 and 0x71).

import fs from "node:fs";
import { spawnSync } from "node:child_process";
import http from "node:http";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright-core";
import { launchBrowser } from "./browser-launch.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const site = path.resolve(here, "../../web");
const browserManifest = JSON.parse(fs.readFileSync(path.join(here, "browser.json"), "utf8"));
const query = process.argv[2] ?? "";
const expectReport = process.argv.includes("--expect-report");
const abortRuntimePack = process.argv.includes("--abort-runtime-pack");
const dismissIntro = process.argv.includes("--dismiss-intro");
const introMovies = ["/data/movies/000.webm", "/data/movies/001.webm"];

function browserExecutable() {
  const candidates = [
    ...(process.env.OPEN_REBELLION_CHROME_FOR_TESTING
      ? [process.env.OPEN_REBELLION_CHROME_FOR_TESTING]
      : []),
    ...browserManifest.executable_candidates,
  ];
  const executable = candidates.find((candidate) => fs.existsSync(candidate));
  if (!executable) throw new Error(`pinned Chrome for Testing ${browserManifest.version} is missing`);
  const version = spawnSync(executable, ["--version"], { encoding: "utf8" });
  if (version.status !== 0 || !version.stdout.includes(browserManifest.version)) {
    throw new Error(`Chrome for Testing version mismatch: ${version.stdout || version.stderr}`);
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

// Pump macroquad's exported frame only until the replay report or the menu
// is ready: headless Chrome may instantiate the WASM without running
// requestAnimationFrame.
function pageState() {
  const menu = () => document.getElementById("main-menu-semantics")?.dataset.active || null;
  let framePumped = false;
  if (window.__openRebellionReplay == null && menu() !== "true"
    && typeof wasm_exports !== "undefined" && typeof wasm_exports.frame === "function") {
    wasm_exports.frame();
    framePumped = true;
  }
  return {
    search: window.location.search,
    report: window.__openRebellionReplay || null,
    menuActive: menu(),
    canvasWidth: document.getElementById("glcanvas")?.width || 0,
    canvasHeight: document.getElementById("glcanvas")?.height || 0,
    framePumped,
  };
}

for (const flag of ["--mute-audio", "--disable-audio-output"]) {
  if (!browserManifest.launch_arguments.includes(flag)) {
    throw new Error(`browser.json launch_arguments lack ${flag}`);
  }
}

const server = await startServer();
const origin = `http://127.0.0.1:${server.address().port}`;
const errors = [];
const requests = [];
const foreignRequests = [];
const responses = [];
const launchAttempts = [];
let browser;
let state = {};
try {
  browser = await launchBrowser(chromium, {
    executablePath: browserExecutable(),
    headless: true,
    args: browserManifest.launch_arguments,
    timeout: 30_000,
  }, launchAttempts);
  const context = await browser.newContext({ serviceWorkers: "block" });
  const page = await context.newPage();
  page.on("request", (request) => {
    const url = new URL(request.url());
    if (url.origin === origin) requests.push(url.pathname);
    else foreignRequests.push(request.url());
  });
  page.on("response", (response) => {
    const url = new URL(response.url());
    if (url.origin === origin) responses.push({ url: url.pathname, status: response.status() });
  });
  page.on("pageerror", (error) => errors.push(error.stack || error.message));
  if (abortRuntimePack) {
    await page.route("**/data/runtime.orpk", (route) => route.abort());
  }
  await page.goto(`${origin}/${query}`, { waitUntil: "load", timeout: 30_000 });
  const dismissed = [];
  const attempts = expectReport ? 200 : dismissIntro ? 100 : 30;
  for (let attempt = 0; attempt < attempts; attempt += 1) {
    state = await page.evaluate(pageState);
    if (expectReport && state.report !== null) break;
    const playing = introMovies.find((movie) =>
      requests.includes(movie) && !dismissed.includes(movie));
    if (dismissIntro && playing) {
      await page.focus("#glcanvas");
      await page.keyboard.press("Escape");
      dismissed.push(playing);
    }
    if (dismissIntro && state.menuActive === "true") break;
    await new Promise((resolve) => setTimeout(resolve, 100));
  }
  state.introDismissed = dismissed;
} finally {
  await browser?.close();
  server.close();
}

process.stdout.write(`${JSON.stringify({
  report: state.report ?? null,
  page_state: state,
  errors,
  requests,
  foreign_requests: foreignRequests,
  responses,
  launch_attempts: launchAttempts,
})}\n`);
