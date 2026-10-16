#!/usr/bin/env python3
"""Check that the native game and the headless playtest agree.

    scripts/divergence.py SAVE --days N [--out DIR] [--dir REPO_COPY]
                          [--host H] [--data DIR]

Needs `scripts/launch-native.sh --load/--telemetry`, the developer commands
`Battle policy`, `Advice policy`, `Fast forward`, `Wait until day` and `Save:`, and the playtest's `--load`,
`--ticks` and `--save-out` (all present in the tree).

What it does. SAVE is a local `.reb` save. It is copied to the Mac Mini,
where the native game (from the Mini's repository copy, --dir, default
$OPEN_REBELLION_BUILD_DIR or or-native/open-rebellion) loads it with
telemetry on, reads its day D with `Status`, fast-forwards to day D+N and
saves to `<evidence>/saves/divergence-end.reb`; and the headless playtest
loads the same save, runs N ticks and saves. Both end saves and both event
logs are pulled back to OUT (default `.artifacts/divergence/<UTC stamp>`,
relative to the repository).

The verdict comes from the event streams, not the fingerprints. Only the
event types both runs emit are compared (SHARED_TYPES: combat_space,
mission_resolved, ai_action, ai_day, blockade_*, research_unlocked,
control_changed),
grouped by type, in order, with the tick, so the first difference names a
tick and a type. Trace records, wall-clock and sequence fields are ignored.

The two end saves' header fingerprints are printed for information only.
That fingerprint hashes the whole SaveState, RNG, both fogs, the clock and
the player's agent included. The runs' RNG streams desync by construction
(the playtest draws 1024 rolls a tick up front, the native game draws per
system), and the rolls decide outcomes, so the worlds themselves differ and no
state hash would match. The event-stream verdict stands until the native tick
draws rolls as `run_simulation_tick` does; until then a divergence report is
the expected result, and it shows where the runs part.

Exit 0 when the shared streams agree, 1 when they differ, 2 when a run
failed or a stream is missing.

NEVER RUN THIS WHILE ANOTHER NATIVE GAME FROM THE SAME CHECKOUT IS RUNNING.
A checkout runs at most one native game on the Mini, and launch-native.sh
stops its checkout's earlier instance, which would kill that game. The script
refuses to start when it finds the checkout's game running. The game it starts
is killed when the run ends. Standard library only.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shlex
import struct
import subprocess
import sys
import time
from collections import defaultdict
from datetime import datetime, timezone
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import telemetry  # noqa: E402

REPO = Path(__file__).resolve().parent.parent
DEFAULT_DIR = os.environ.get("OPEN_REBELLION_BUILD_DIR", "or-native/open-rebellion")
DEFAULT_HOST = os.environ.get("OPEN_REBELLION_BUILD_HOST", "mac-mini-ts")
MAGIC = b"OPENREB\0"
END_NAME = "divergence-end"
# Fields that differ between runs without meaning anything.
IGNORED_FIELDS = ("wall_ms", "seq", "source", "_line")
# Event types both runs emit; the rest are one side's bookkeeping.
SHARED_TYPES = frozenset(
    {
        "combat_space",
        "mission_resolved",
        "ai_action",
        "ai_day",
        "research_unlocked",
        "control_changed",
    }
)


def shared(record: dict) -> bool:
    kind = str(record.get("event_type", ""))
    # A held order exists only natively: headless opens no player battle.
    if kind == "ai_action" and telemetry.field(record, "outcome") == "held":
        return False
    return kind in SHARED_TYPES or kind.startswith("blockade_")


class Failure(Exception):
    pass


def fingerprint(path: Path) -> str:
    """The `state_fingerprint` of a save, read from its header.

    Layout: magic, u32 version, u32 name length + name, u64 timestamp,
    u32 mod count and each mod's two length-prefixed strings, u64 mod hash,
    u16 fingerprint version, u64 fingerprint (crates/rebellion-data/src/save.rs).
    """
    data = path.read_bytes()
    if data[:8] != MAGIC:
        raise Failure(f"{path} is not an Open Rebellion save")
    (version,) = struct.unpack_from("<I", data, 8)
    at = 12
    (name_len,) = struct.unpack_from("<I", data, at)
    at += 4 + name_len + 8  # name, timestamp
    (mods,) = struct.unpack_from("<I", data, at)
    at += 4
    for _ in range(mods * 2):  # each mod: name, version
        (length,) = struct.unpack_from("<I", data, at)
        at += 4 + length
    at += 8  # mod hash
    fp_version, value = struct.unpack_from("<HQ", data, at)
    return f"v{fp_version}:{value:016x} (save version {version})"


def ssh(host: str, command: str, check: bool = True) -> subprocess.CompletedProcess:
    done = subprocess.run(["ssh", host, command], capture_output=True, text=True)
    if check and done.returncode != 0:
        raise Failure(f"ssh {host}: {command[:120]} -> {done.returncode}: {done.stderr.strip()[:300]}")
    return done


def pull(host: str, remote: str, local: Path) -> None:
    local.parent.mkdir(parents=True, exist_ok=True)
    done = subprocess.run(["rsync", "-a", f"{host}:{shlex.quote(remote)}", str(local)], capture_output=True, text=True)
    if done.returncode != 0:
        raise Failure(f"pull {remote}: {done.stderr.strip()[:300]}")


def refuse_if_game_running(host: str, directory: str) -> None:
    """A checkout runs at most one native game; never start beside one."""
    absolute = ssh(host, f"cd {shlex.quote(directory)} && pwd").stdout.strip()
    binary = f"{absolute}/target/release/open-rebellion"
    running = ssh(host, f"pgrep -f {shlex.quote(binary)}", check=False)
    if running.returncode == 0:
        pids = " ".join(running.stdout.split())
        raise Failure(
            f"a native game from {absolute} is running (pid {pids}); launching would stop it. "
            "Close it first."
        )


def comparable(record: dict) -> str | None:
    """A shared record's identity across runs, or None when it is not one."""
    if not shared(record):
        return None
    return json.dumps(
        {k: v for k, v in record.items() if k not in IGNORED_FIELDS},
        sort_keys=True, separators=(",", ":"), ensure_ascii=False,
    )


def stream_by_type(records: list[dict]) -> dict[str, list[tuple[str, dict]]]:
    grouped: dict[str, list[tuple[str, dict]]] = defaultdict(list)
    for record in records:
        key = comparable(record)
        if key is not None:
            grouped[str(record.get("event_type"))].append((key, record))
    return grouped


def first_divergence(native: list[dict], headless: list[dict]) -> tuple[list[str], dict | None]:
    """Per-type counts, and the earliest-tick disagreement across types."""
    a, b = stream_by_type(native), stream_by_type(headless)
    lines: list[str] = []
    worst: dict | None = None
    for kind in sorted(set(a) | set(b)):
        left, right = a.get(kind, []), b.get(kind, [])
        index = next((i for i, (x, y) in enumerate(zip(left, right)) if x[0] != y[0]), None)
        if index is None and len(left) != len(right):
            index = min(len(left), len(right))
        lines.append(f"| {kind} | {len(left)} | {len(right)} | {'agree' if index is None else f'differ at record {index}'} |")
        if index is None:
            continue
        mine = left[index][1] if index < len(left) else None
        theirs = right[index][1] if index < len(right) else None
        tick = min(r.get("tick", 0) for r in (mine, theirs) if r is not None)
        if worst is None or tick < worst["tick"]:
            worst = {"tick": tick, "type": kind, "index": index, "native": mine, "headless": theirs}
    return lines, worst


def run(args: argparse.Namespace) -> int:
    if not args.save.is_file():
        raise Failure(f"no save {args.save}")
    start_fp = fingerprint(args.save)
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    out_rel = args.out or Path(".artifacts/divergence") / stamp
    if out_rel.is_absolute() or ".." in out_rel.parts:
        raise Failure("--out must be relative to the repository, without '..'")
    out = REPO / out_rel
    out.mkdir(parents=True, exist_ok=True)
    host, directory = args.host, args.dir
    work = f"{out_rel}"  # the same relative path on the Mini
    native_rel = f"{work}/native"
    headless_rel = f"{work}/headless"

    refuse_if_game_running(host, directory)
    q = shlex.quote
    ssh(host, f"cd {q(directory)} && mkdir -p {q(native_rel)} {q(headless_rel)} && "
              f"rm -f {q(native_rel)}/commands.out {q(native_rel)}/commands.in {q(native_rel)}/native-results.jsonl")
    push_save = subprocess.run(["rsync", "-a", str(args.save), f"{host}:{q(directory)}/{q(work)}/start.reb"], capture_output=True, text=True)
    if push_save.returncode != 0:
        raise Failure(f"copying the save: {push_save.stderr.strip()[:300]}")

    # Native: load, read the day, fast-forward, save.
    ssh(host, f"cd {q(directory)} && (OPEN_REBELLION_DEV=1 scripts/launch-native.sh --live --telemetry "
              f"--evidence {q(native_rel)} --load {q(work + '/start.reb')} >/dev/null 2>&1 &)")
    try:
        deadline = time.time() + 180
        outfile = q(directory + "/" + native_rel + "/commands.out")
        while True:
            # The launcher writes the `Load:` line itself, so the first
            # result (seq 1) is what shows the game is up and the load ran.
            first = ssh(host, f"grep -F '{{\"seq\":1,' {outfile}", check=False).stdout.strip()
            if first:
                break
            if time.time() > deadline:
                raise Failure("the native game never answered the Load")
            time.sleep(1)
        if '"status":"done"' not in first:
            raise Failure(f"the native Load failed: {first[:300]}")
        status = ssh(host, f"cd {q(directory)} && scripts/dev-send.sh {q(native_rel)} Status --timeout 60", check=False)
        match = re.search(r"\bday=(\d+)", status.stdout)
        if not match:
            raise Failure(f"Status gave no day after the load: {status.stdout.strip()[:300]}")
        day = int(match.group(1))
        script = REPO / out_rel / "native-steps.txt"
        script.write_text(
            "Battle policy: simulate\nAdvice policy: close\n"
            f"Fast forward: on\nWait until day {day + args.days}\nFast forward: off\nSave: {END_NAME}\n",
            encoding="utf-8",
        )
        ran = subprocess.run(
            [str(REPO / "scripts" / "dev-run.sh"), native_rel, str(script), "--host", host,
             "--timeout", str(args.timeout)],
            capture_output=True, text=True, env=dict(os.environ, OPEN_REBELLION_BUILD_DIR=directory),
        )
        (out / "native-results.jsonl").write_text(ran.stdout, encoding="utf-8")
        if ran.returncode != 0:
            raise Failure(f"native run failed ({ran.returncode}); see {out_rel}/native-results.jsonl")
    finally:
        # The game this script started goes with it.
        ssh(host, f"pkill -f {q(directory + '/target/release/open-rebellion')} || true", check=False)

    # Headless: same save, same number of days.
    play = [
        "target/release/rebellion-playtest", args.data, "--load", f"{work}/start.reb",
        "--ticks", str(args.days), "--save-out", f"{headless_rel}/{END_NAME}.reb",
        "--output", f"{headless_rel}/telemetry.jsonl",
    ]
    ssh(host, f"cd {q(directory)} && {shlex.join(play)} >{q(headless_rel)}/summary.txt 2>{q(headless_rel)}/log.txt")
    pull(host, f"{directory}/{headless_rel}/", out / "headless")

    native_save = out / "native" / "saves" / f"{END_NAME}.reb"
    if not native_save.exists():
        reported = re.findall(r'"path":"([^"]+\.reb)"', ran.stdout)
        if reported:
            where = reported[-1] if reported[-1].startswith("/") else f"{directory}/{reported[-1]}"
            pull(host, where, native_save)
    headless_save = out / "headless" / f"{END_NAME}.reb"
    for path in (native_save, headless_save):
        if not path.exists():
            raise Failure(f"missing end save {path}")

    native_fp, headless_fp = fingerprint(native_save), fingerprint(headless_save)
    lines = [
        "# Native vs headless divergence", "",
        f"- save: {args.save} ({start_fp}), loaded at day {day}, run {args.days} days to day {day + args.days}",
        f"- native end fingerprint:   {native_fp}",
        f"- headless end fingerprint: {headless_fp}",
        "- the fingerprints cover the RNG, both fogs, the clock and the player's agent, which differ between the runs "
        "by construction: shown for information, not compared.", "",
    ]
    native_log, headless_log = out / "native" / "telemetry.jsonl", out / "headless" / "telemetry.jsonl"
    code = 2
    if native_log.exists() and headless_log.exists():
        rows, worst = first_divergence(telemetry.load(native_log), telemetry.load(headless_log))
        lines += ["## Shared event streams", "", "| event type | native | headless | |", "|---|---|---|---|", *rows, ""]
        if worst:
            code = 1
            lines += [
                f"verdict: DIVERGED. First difference: tick {worst['tick']}, type {worst['type']}, record {worst['index']}", "",
                f"- native:   {json.dumps(worst['native'], ensure_ascii=False) if worst['native'] else 'no record'}",
                f"- headless: {json.dumps(worst['headless'], ensure_ascii=False) if worst['headless'] else 'no record'}", "",
            ]
        else:
            code = 0
            lines += ["verdict: the shared event streams agree record for record.", ""]
    else:
        lines += ["## Shared event streams", "", "verdict: UNKNOWN. Both runs must have written telemetry "
                  f"(native {'found' if native_log.exists() else 'missing'}, "
                  f"headless {'found' if headless_log.exists() else 'missing'}).", ""]
    report = out / "report.md"
    report.write_text("\n".join(lines), encoding="utf-8")
    print("\n".join(lines))
    print(report)
    return code


def main() -> int:
    parser = argparse.ArgumentParser(
        description=__doc__.split("\n\n")[0], epilog="Depends on launch-native.sh --load, the developer commands "
        "Fast forward / Wait until day / Save, and playtest --load/--save-out. The native run sets "
        "`Battle policy: simulate` and `Advice policy: close`. Never run while another native "
        "game from the same checkout is running.",
    )
    parser.add_argument("save", type=Path, help="local .reb save to start both runs from")
    parser.add_argument("--days", type=int, required=True, help="game days to run (headless ticks)")
    parser.add_argument("--out", type=Path, help="output folder, relative to the repository")
    parser.add_argument("--dir", default=DEFAULT_DIR, help="the Mini's repository copy")
    parser.add_argument("--host", default=DEFAULT_HOST)
    parser.add_argument("--data", default="data/base", help="data dir on the Mini, relative to --dir")
    parser.add_argument("--timeout", type=int, default=600, help="seconds the native wait may take")
    args = parser.parse_args()
    if args.days < 1:
        parser.error("--days must be at least 1")
    try:
        return run(args)
    except Failure as error:
        print(f"divergence: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
