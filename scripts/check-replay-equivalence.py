#!/usr/bin/env python3
"""Fail-closed native/WASM replay checkpoint equivalence gate."""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
ARTIFACT = ROOT / "crates/rebellion-data/tests/fixtures/replay_seed42_v1.json"
EXPECTED_REQUEST_SUFFIXES = ("/", "/gl.js", "/open-rebellion.wasm", "/data/runtime.orpk")
# Ordinary startup also plays the intro movies 0x65 and 0x66 before the menu.
INTRO_MOVIES = ("/data/movies/000.webm", "/data/movies/001.webm")
EXPECTED_NORMAL_REQUESTS = (*EXPECTED_REQUEST_SUFFIXES, *INTRO_MOVIES)
EQUIVALENCE_FIELDS = (
    "schema_version",
    "fixture_id",
    "status",
    "artifact_text",
    "engine_version",
    "seed",
    "data_input_count",
    "data_bytes",
    "data_fingerprint",
    "initial_fingerprint",
    "observed_checkpoints",
    "final_tick",
    "final_fingerprint",
    "failure_phase",
    "error",
)


def run(command: list[str], *, timeout: int = 300) -> subprocess.CompletedProcess[str]:
    environment = dict(os.environ)
    environment.update(
        {
            "CC": "/usr/bin/cc",
            "CXX": "/usr/bin/c++",
            "CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER": "/usr/bin/cc",
        }
    )
    return subprocess.run(
        command,
        cwd=ROOT,
        env=environment,
        capture_output=True,
        text=True,
        timeout=timeout,
        check=False,
    )


def parse_json_line(output: str) -> dict:
    for line in reversed(output.splitlines()):
        if line.startswith("{"):
            return json.loads(line)
    raise ValueError(f"command emitted no JSON object: {output[-500:]}")


def native_report() -> dict:
    result = run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "rebellion-data",
            "--bin",
            "replay-gate",
            "--",
            "data/base",
        ]
    )
    if result.returncode != 0:
        raise RuntimeError(f"native replay gate failed:\n{result.stderr}\n{result.stdout}")
    return parse_json_line(result.stdout)


def browser_report(
    *,
    query: str,
    expect_report: bool,
    abort_runtime_pack: bool = False,
    dismiss_intro: bool = False,
) -> dict:
    """One page load of web/ in the interface-parity gates' pinned, muted Chrome."""
    command = [
        "node",
        str(ROOT / "tools/interface-parity/replay-browser.mjs"),
        query,
    ]
    if expect_report:
        command.append("--expect-report")
    if abort_runtime_pack:
        command.append("--abort-runtime-pack")
    if dismiss_intro:
        command.append("--dismiss-intro")
    result = subprocess.run(
        command,
        cwd=ROOT,
        capture_output=True,
        text=True,
        timeout=120,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(f"browser leg failed:\n{result.stderr}\n{result.stdout}")
    browser = parse_json_line(result.stdout)
    if expect_report and browser.get("report") is None:
        raise RuntimeError(
            "browser replay gate did not report within 20 seconds: "
            f"{json.dumps(browser.get('page_state'), sort_keys=True)}"
        )
    return browser


def verify(
    native: dict,
    browser: dict,
    invalid_query: dict,
    missing_pack: dict,
    normal_startup: dict,
) -> dict:
    failures: list[str] = []
    browser_gate = browser.get("report")
    if not isinstance(browser_gate, dict):
        failures.append("browser did not publish window.__openRebellionReplay")
        browser_gate = {}

    artifact_text = ARTIFACT.read_text(encoding="utf-8")
    if native.get("artifact_text") != artifact_text:
        failures.append("native runner did not execute the exact committed artifact text")
    if browser_gate.get("artifact_text") != artifact_text:
        failures.append("WASM runner did not execute the exact committed artifact text")
    for field in EQUIVALENCE_FIELDS:
        if native.get(field) != browser_gate.get(field):
            failures.append(f"native/WASM mismatch in {field}")

    errors = browser.get("errors", [])
    if errors:
        failures.append(f"browser emitted errors: {errors}")

    requests = browser.get("requests", [])
    replay_paths = browser.get("requests", [])
    for name, page in (
        ("replay", browser),
        ("invalid query", invalid_query),
        ("missing pack", missing_pack),
        ("normal startup", normal_startup),
    ):
        if page.get("foreign_requests"):
            failures.append(f"{name} requested other origins: {page['foreign_requests']}")
    if tuple(replay_paths) != EXPECTED_REQUEST_SUFFIXES:
        failures.append(f"expected four replay-gate requests, saw {replay_paths}")
    failed_responses = [
        response for response in browser.get("responses", []) if response.get("status", 0) >= 400
    ]
    if failed_responses:
        failures.append(f"browser responses failed: {failed_responses}")

    invalid_report = invalid_query.get("report") or {}
    if invalid_report.get("status") != "failed" or invalid_report.get("failure_phase") != "query":
        failures.append("invalid replay query did not fail closed in the query phase")
    if invalid_query.get("requests", []) != ["/", "/gl.js", "/open-rebellion.wasm"]:
        failures.append("invalid replay query fell through to runtime-pack or production loading")
    if (
        invalid_report.get("initial_fingerprint") is not None
        or invalid_report.get("observed_checkpoints")
        or invalid_report.get("final_fingerprint") is not None
    ):
        failures.append("invalid replay query retained partial replay state")
    if invalid_query.get("errors"):
        failures.append(f"invalid replay query emitted browser errors: {invalid_query['errors']}")

    missing_report = missing_pack.get("report") or {}
    if (
        missing_report.get("status") != "failed"
        or missing_report.get("failure_phase") != "runtime_pack_fetch"
    ):
        failures.append("missing runtime pack did not produce a structured fetch failure")
    if (
        missing_report.get("initial_fingerprint") is not None
        or missing_report.get("observed_checkpoints")
        or missing_report.get("final_fingerprint") is not None
    ):
        failures.append("missing runtime pack retained partial replay state")
    if missing_pack.get("errors"):
        failures.append(f"missing runtime pack emitted browser errors: {missing_pack['errors']}")

    normal_state = normal_startup.get("page_state", {})
    if normal_state.get("report") is not None:
        failures.append("normal startup unexpectedly executed the replay gate")
    if normal_state.get("menuActive") != "true":
        failures.append("normal startup did not reach the semantic main menu")
    if normal_state.get("canvasWidth", 0) <= 0 or normal_state.get("canvasHeight", 0) <= 0:
        failures.append("normal startup canvas is blank-sized")
    normal_paths = normal_startup.get("requests", [])
    if tuple(normal_paths) != EXPECTED_NORMAL_REQUESTS:
        failures.append(f"expected six normal-startup requests, saw {normal_paths}")
    if tuple(normal_state.get("introDismissed") or ()) != INTRO_MOVIES:
        failures.append("normal startup did not play both intro movies")
    if normal_startup.get("errors"):
        failures.append(f"normal startup emitted browser errors: {normal_startup['errors']}")

    return {
        "status": "passed" if not failures else "failed",
        "native_platform": native.get("platform"),
        "browser_platform": browser_gate.get("platform"),
        "artifact_bytes": len(artifact_text.encode("utf-8")),
        "checkpoint_count": len(native.get("observed_checkpoints", [])),
        "initial_fingerprint": native.get("initial_fingerprint"),
        "final_tick": native.get("final_tick"),
        "final_fingerprint": native.get("final_fingerprint"),
        "request_count": len(requests),
        "browser_errors": len(errors),
        "invalid_query": invalid_report.get("failure_phase"),
        "missing_pack": missing_report.get("failure_phase"),
        "normal_startup_requests": len(normal_startup.get("requests", [])),
        "normal_menu_active": normal_state.get("menuActive"),
        "failures": failures,
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--skip-build", action="store_true")
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args()

    if not args.skip_build:
        built = run(["bash", "scripts/build-wasm.sh"])
        if built.returncode != 0:
            print(built.stderr, file=sys.stderr)
            return 1

    native = native_report()
    browser = browser_report(
        query=f"?replay-check=seed42-v1&gate-run={os.getpid()}",
        expect_report=True,
    )
    invalid_query = browser_report(query="?replay-check=unknown", expect_report=True)
    missing_pack = browser_report(
        query="?replay-check=seed42-v1",
        expect_report=True,
        abort_runtime_pack=True,
    )
    normal_startup = browser_report(query="", expect_report=False, dismiss_intro=True)
    result = verify(native, browser, invalid_query, missing_pack, normal_startup)

    if args.json:
        print(json.dumps(result, indent=2))
    else:
        print(
            f"Replay equivalence: {result['status'].upper()} | "
            f"{result['checkpoint_count']} checkpoints | "
            f"{result['request_count']} requests | "
            f"{result['browser_errors']} browser errors"
        )
        for failure in result["failures"]:
            print(f"- {failure}")
    return 0 if result["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
