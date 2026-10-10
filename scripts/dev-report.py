#!/usr/bin/env python3
"""Show a native run as one timeline: each developer command, its result,
and what the game did while it ran.

    scripts/dev-report.py DIR [--all] [--no-status]

DIR is a `scripts/launch-native.sh --evidence` folder. The report reads
DIR/commands.out (each command's JSON result) and DIR/game.log, which it
splits at the `[dev-command] run #N: <line>` marker each command logs as it
starts. Under each command it lists the game's tagged lines up to the next
command. Routine lines (droid sound and slot traces, `sent`/`done`
bookkeeping) are hidden unless --all is given.

If the game is still running with its live channel open, the report ends
with a fresh `Status` (mode, day, speed, open dialogs), so a run stuck behind
a dialog says so. --no-status skips it. agent_docs/dev-commands.md documents
the channel. Standard library only.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
from pathlib import Path

RUN = re.compile(r"^\[dev-command\] run #(\S+): (.*)$")
# Bookkeeping and per-frame droid traces: shown only with --all.
ROUTINE = (
    re.compile(r"^\[dev-command\] (sent|done)\b"),
    re.compile(r"^\[advisor\] (sound|slot|cockpit step)\b"),
)
# Lines worth flagging in the margin.
MARKS = (
    (re.compile(r"refused|unknown|cannot|error|panic", re.I), "!"),
    (re.compile(r"^\[capture\]"), "@"),
)


def results(path: Path) -> dict[str, dict]:
    """Each inbox command's result by sequence number."""
    out: dict[str, dict] = {}
    if not path.exists():
        return out
    for raw in path.read_text(encoding="utf-8", errors="replace").splitlines():
        try:
            entry = json.loads(raw)
        except json.JSONDecodeError:
            continue
        if "seq" in entry:
            out[str(entry["seq"])] = entry
    return out


def sections(path: Path) -> tuple[list[str], list[tuple[str, str, list[str]]]]:
    """The log lines before the first command, then each command's lines."""
    prelude: list[str] = []
    runs: list[tuple[str, str, list[str]]] = []
    if not path.exists():
        return prelude, runs
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        match = RUN.match(line)
        if match:
            runs.append((match.group(1), match.group(2), []))
        elif runs:
            runs[-1][2].append(line)
        else:
            prelude.append(line)
    return prelude, runs


def shown(line: str, everything: bool) -> bool:
    if not line.strip():
        return False
    return everything or not any(p.search(line) for p in ROUTINE)


def mark(line: str) -> str:
    for pattern, symbol in MARKS:
        if pattern.search(line):
            return symbol
    return " "


def live_pid(directory: Path) -> int | None:
    """The run's pid from binary.txt, if that process is still alive."""
    summary = directory / "binary.txt"
    if not summary.exists():
        return None
    match = re.search(r"\bpid=(\d+)", summary.read_text())
    if not match:
        return None
    pid = int(match.group(1))
    try:
        os.kill(pid, 0)
    except OSError:
        return None
    return pid


def status(directory: Path) -> str | None:
    sender = Path(__file__).with_name("dev-send.sh")
    try:
        done = subprocess.run(
            [str(sender), str(directory), "Status", "--timeout", "10"],
            capture_output=True,
            text=True,
            timeout=20,
        )
    except (OSError, subprocess.TimeoutExpired):
        return None
    try:
        return json.loads(done.stdout.strip().splitlines()[-1]).get("detail")
    except (IndexError, json.JSONDecodeError):
        return done.stdout.strip() or None


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("dir", type=Path, help="the run's evidence folder")
    parser.add_argument("--all", action="store_true", help="show routine lines too")
    parser.add_argument("--no-status", action="store_true", help="skip the live Status")
    args = parser.parse_args()
    directory: Path = args.dir
    if not directory.is_dir():
        print(f"no folder {directory}", file=sys.stderr)
        return 2

    summary = directory / "binary.txt"
    if summary.exists():
        print(summary.read_text().strip())
    answers = results(directory / "commands.out")
    prelude, runs = sections(directory / "game.log")
    startup = [line for line in prelude if shown(line, args.all) and mark(line) == "!"]
    if startup:
        print("\nbefore the first command:")
        for line in startup:
            print(f"  ! {line}")

    for seq, line, lines in runs:
        answer = answers.get(seq, {})
        state = answer.get("status", "script" if seq == "-" else "pending")
        detail = answer.get("detail", "")
        flag = "!" if state in ("refused", "pending") else " "
        print(f"\n{flag}#{seq} {line}  -> {state}{': ' + detail if detail else ''}")
        if answer.get("path"):
            print(f"  @ {answer['path']}")
        for event in lines:
            # The result above already names this command's capture.
            if answer.get("path") and event.startswith("[capture]"):
                continue
            if shown(event, args.all):
                print(f"  {mark(event)} {event}")

    if not args.no_status and (directory / "commands.in").exists() and live_pid(directory):
        now = status(directory)
        print(f"\nnow: {now}" if now else "\nnow: no answer to Status (busy or not live)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
