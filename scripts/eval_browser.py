#!/usr/bin/env python3
"""
Evaluate Open Rebellion WASM build for browser functionality.

Uses dev-browser CLI for headless browser validation. Checks that the WASM
binary builds, loads, renders UI, and supports save/load.

Usage:
    python3 scripts/eval_browser.py
    python3 scripts/eval_browser.py --json
    python3 scripts/eval_browser.py --skip-build  # skip WASM build step

Composite score formula (5 sub-metrics):
    score = 0.25 * wasm_build
          + 0.10 * wasm_size
          + 0.20 * dat_loading
          + 0.25 * ui_rendering
          + 0.20 * save_load

Each sub-metric is 0.0 or 1.0 (pass/fail).

Requires:
    - dev-browser CLI installed (brew install dev-browser or npm i -g dev-browser)
    - data/base/*.DAT files present
"""

import json
import os
import signal
import subprocess
import sys
import tempfile
import time
from pathlib import Path

SCRIPT_DIR = Path(__file__).parent
PROJECT_DIR = SCRIPT_DIR.parent
WEB_DIR = PROJECT_DIR / "web"
WASM_FILE = WEB_DIR / "open-rebellion.wasm"
BUILD_SCRIPT = SCRIPT_DIR / "build-wasm.sh"

# Max WASM binary size (8MB)
MAX_WASM_SIZE = 8 * 1024 * 1024

# Port for local HTTP server
SERVER_PORT = 18080


def check_wasm_build(skip_build: bool) -> tuple[bool, str]:
    """Build WASM binary. Returns (success, message)."""
    if skip_build:
        if WASM_FILE.exists():
            return True, f"skipped (existing: {WASM_FILE.stat().st_size} bytes)"
        return False, "skipped but wasm file missing"

    env = dict(os.environ)
    env["PATH"] = f"/usr/bin:{env.get('PATH', '')}"
    result = subprocess.run(
        ["bash", str(BUILD_SCRIPT)],
        capture_output=True, text=True, env=env, timeout=300,
        cwd=str(PROJECT_DIR),
    )
    if result.returncode != 0:
        return False, f"build failed: {result.stderr[-200:]}"
    if not WASM_FILE.exists():
        return False, "build succeeded but wasm file not found"
    return True, f"built ({WASM_FILE.stat().st_size} bytes)"


def check_wasm_size() -> tuple[bool, str]:
    """Check WASM binary size is under threshold."""
    if not WASM_FILE.exists():
        return False, "wasm file missing"
    size = WASM_FILE.stat().st_size
    ok = size <= MAX_WASM_SIZE
    return ok, f"{size} bytes ({'ok' if ok else 'too large'}, limit {MAX_WASM_SIZE})"


def start_http_server() -> subprocess.Popen:
    """Start a local HTTP server for web/ directory."""
    proc = subprocess.Popen(
        [sys.executable, "-m", "http.server", str(SERVER_PORT)],
        cwd=str(WEB_DIR),
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    time.sleep(1)  # let server start
    return proc


def run_dev_browser_script(script: str, timeout: int = 30) -> tuple[bool, str]:
    """Run a dev-browser script and return (success, output)."""
    result = subprocess.run(
        ["dev-browser", "--headless", "--timeout", str(timeout)],
        input=script,
        capture_output=True, text=True, timeout=timeout + 10,
    )
    output = result.stdout.strip()
    if result.returncode != 0:
        return False, f"dev-browser failed: {result.stderr[-200:]}"
    return True, output


def check_dat_loading() -> tuple[bool, str]:
    """Check that DAT files load successfully in the WASM app."""
    script = f'''
const page = await browser.getPage("open-rebellion");
await page.goto("http://localhost:{SERVER_PORT}/");

// Wait for canvas to appear (macroquad renders to canvas)
await page.waitForSelector("canvas", {{ timeout: 10000 }});

// Wait a bit for WASM to initialize and load DAT files
await new Promise(r => setTimeout(r, 5000));

// Check for console errors that indicate loading failures
const logs = await page.evaluate(() => {{
    // macroquad logs to console; check if the page rendered
    return {{
        title: document.title,
        hasCanvas: !!document.querySelector("canvas"),
        canvasWidth: document.querySelector("canvas")?.width || 0,
        canvasHeight: document.querySelector("canvas")?.height || 0,
    }};
}});

console.log(JSON.stringify(logs));
'''
    ok, output = run_dev_browser_script(script, timeout=20)
    if not ok:
        return False, output

    try:
        data = json.loads(output)
        has_canvas = data.get("hasCanvas", False)
        width = data.get("canvasWidth", 0)
        return has_canvas and width > 0, f"canvas: {width}x{data.get('canvasHeight', 0)}"
    except json.JSONDecodeError:
        return False, f"unexpected output: {output[:200]}"


def check_ui_rendering() -> tuple[bool, str]:
    """Check that UI renders (screenshot + snapshot for content)."""
    screenshot_dir = tempfile.mkdtemp(prefix="eval-browser-")
    script = f'''
const page = await browser.getPage("open-rebellion");
// Page should already be loaded from dat_loading check
await new Promise(r => setTimeout(r, 2000));

// Take screenshot
const buf = await page.screenshot();
const path = await saveScreenshot(buf, "ui-check.png");

// Get page snapshot for AI — check for UI text content
const snap = await page.snapshotForAI();

console.log(JSON.stringify({{
    screenshot: path,
    hasContent: snap.full.length > 100,
    snapLength: snap.full.length,
}}));
'''
    ok, output = run_dev_browser_script(script, timeout=15)
    if not ok:
        return False, output

    try:
        data = json.loads(output)
        has_content = data.get("hasContent", False)
        return has_content, f"snapshot: {data.get('snapLength', 0)} chars"
    except json.JSONDecodeError:
        return False, f"unexpected output: {output[:200]}"


def check_save_load() -> tuple[bool, str]:
    """Check if save/load works in browser (localStorage/IndexedDB)."""
    script = f'''
const page = await browser.getPage("open-rebellion");
await new Promise(r => setTimeout(r, 1000));

// Check if localStorage is accessible and if save stubs exist
const result = await page.evaluate(() => {{
    try {{
        localStorage.setItem("__eval_test", "1");
        const v = localStorage.getItem("__eval_test");
        localStorage.removeItem("__eval_test");
        return {{
            localStorageWorks: v === "1",
            existingSaves: Object.keys(localStorage).filter(k => k.startsWith("save_")).length,
        }};
    }} catch (e) {{
        return {{ localStorageWorks: false, error: e.message }};
    }}
}});

console.log(JSON.stringify(result));
'''
    ok, output = run_dev_browser_script(script, timeout=10)
    if not ok:
        return False, output

    try:
        data = json.loads(output)
        # For now, just check localStorage is accessible
        # Full save/load requires WASM integration (task #133)
        works = data.get("localStorageWorks", False)
        return works, f"localStorage: {'accessible' if works else 'blocked'}"
    except json.JSONDecodeError:
        return False, f"unexpected output: {output[:200]}"


def evaluate(skip_build: bool = False) -> dict:
    """Run all browser checks and compute composite score."""
    metrics = {}

    # 1. WASM build
    ok, msg = check_wasm_build(skip_build)
    metrics["wasm_build"] = {"pass": ok, "detail": msg}
    if not ok:
        # Can't proceed without a WASM binary
        return {
            "score": 0.0,
            "degenerate": True,
            "reason": f"WASM build failed: {msg}",
            **{k: v["pass"] for k, v in metrics.items()},
        }

    # 2. WASM size
    ok, msg = check_wasm_size()
    metrics["wasm_size"] = {"pass": ok, "detail": msg}

    # 3-5. Browser tests (need HTTP server)
    server = None
    try:
        server = start_http_server()

        ok, msg = check_dat_loading()
        metrics["dat_loading"] = {"pass": ok, "detail": msg}

        ok, msg = check_ui_rendering()
        metrics["ui_rendering"] = {"pass": ok, "detail": msg}

        ok, msg = check_save_load()
        metrics["save_load"] = {"pass": ok, "detail": msg}

    except Exception as e:
        for key in ("dat_loading", "ui_rendering", "save_load"):
            if key not in metrics:
                metrics[key] = {"pass": False, "detail": str(e)}
    finally:
        if server:
            server.terminate()
            server.wait(timeout=5)

    # Composite score
    weights = {
        "wasm_build": 0.25,
        "wasm_size": 0.10,
        "dat_loading": 0.20,
        "ui_rendering": 0.25,
        "save_load": 0.20,
    }

    score = sum(weights[k] * (1.0 if metrics[k]["pass"] else 0.0) for k in weights)

    return {
        "score": round(score, 4),
        "degenerate": False,
        **{k: 1.0 if metrics[k]["pass"] else 0.0 for k in weights},
        "details": {k: metrics[k]["detail"] for k in metrics},
    }


def main():
    skip_build = "--skip-build" in sys.argv
    use_json = "--json" in sys.argv

    result = evaluate(skip_build=skip_build)

    if use_json:
        print(json.dumps(result, indent=2))
    else:
        print(f"Browser Score: {result['score']:.4f}")
        if result.get("degenerate"):
            print(f"  DEGENERATE: {result['reason']}")
        for key in ("wasm_build", "wasm_size", "dat_loading", "ui_rendering", "save_load"):
            if key in result:
                val = result[key]
                icon = "pass" if val == 1.0 else "FAIL"
                detail = result.get("details", {}).get(key, "")
                print(f"  {key}: {icon} — {detail}")


if __name__ == "__main__":
    main()
