#!/usr/bin/env python3
"""Check a native run of the AI round scheduler against its bounds.

    scripts/native-checks/check-scheduler.py DIR [--battle-hold [SYSTEM]]
                                             [--save-reload] [--speed S]

DIR is a native evidence folder launched with `--telemetry` (it holds
`telemetry.jsonl`, `commands.in`, `commands.out` and `game.log`). Prints one
PASS, FAIL or SKIP line per check and exits 1 on any FAIL. SKIP means the run
cannot answer the check (the records it needs are absent), never a pass.

Always run, from the `ai_day` records (one per side per day, `details`:
faction, day, speed, rate, rounds, orders, paused):

  orders-cap       orders <= 5 at fast, <= 10 at every other speed and on a
                   day that ran more rounds than its closing speed's (a
                   mid-day speed rise)
  orders-per-round orders <= rounds (at most one order a round)
  rounds           rounds == rate+1 (5 fast, 13 medium, 61 slow, 601 very slow)
  rate-field       the record's `rate` agrees with its `speed`

`rounds` skips the first record of each side (the opening day starts carried)
and the day a speed change lands in (the effective speed is the day's last
step's, so a mid-day change leaves a mixed day), and reports how many it
skipped. A paused day (`paused: true`) is skipped too.

Without any `ai_day` record the cap falls back to counting applied `ai_action`
records per (day, faction) with `telemetry.order_counts`, graded at the
--speed given (default: the looser cap of 10), and says so. `rounds` is then
SKIP. A side the player runs has no AI, so a single-AI run shows one faction.

--battle-hold [SYSTEM]  the battle-hold run. SYSTEM, the alert's battle,
    defaults to the system of the first combat_space record from the Simulate
    click on (else the first `Battle at` line). The window is read off the
    command sequence in commands.out: the first `Battle at` (seq B), `Wait
    until battle alert` (seq W), the `BattleAlert/Simulate` click (seq S).
    The AI's first round of the day waits while the alert is open; the other
    `Battle at` systems are battles left for a later pass, which hold the
    AI's orders there. Checks:
      held-at-later-pass   every `held` record names a `Battle at` system
                           other than SYSTEM in `held_at`, at seq S or later
      no-round-while-open  with OPEN_REBELLION_TRACE=ai: no ai/round trace
                           has a seq in W+1..S-1
      no-applied-while-open (without the trace) no `applied` ai_action with
                           from/at/system equal
                           to the battle system has a seq in W+1..S-1 (the
                           alert open and held). Orders at seq W, the tick
                           that opens the alert, are counted but not judged:
                           the prologue applies a carried order before the
                           battle is known (`held_none`). Orders' `to` and
                           `target` fields are not fleets of the battle.
    A hold window in which the AI wrote no record at all is SKIP: spent
    rounds write nothing, so silence proves nothing.

--save-reload  the save-reload run. Needs a `Save:` and a later `Load:` in
    commands.out. Checks:
      reload-clean  every command after `Load:` is `done` or `skipped` (not
                    refused or timed out) and game.log holds no panic
      reload-rounds the first ai_day after the load has rounds == rate+1
                    (the saved day's rounds before and after the load add up),
                    and at least two ai_day records follow the load

Reuses `telemetry.load`, `telemetry.field` and `telemetry.order_counts`.
Standard library only.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
import telemetry  # noqa: E402

PASS, FAIL, SKIP = "PASS", "FAIL", "SKIP"

RATE = {"very_slow": 600, "slow": 60, "medium": 12, "fast": 4}


def cap_for(speed: str) -> int:
    return 5 if speed == "fast" else 10


def read_commands(folder: Path) -> list[dict]:
    """The result lines of commands.out in order (each has seq, line, status)."""
    path = folder / "commands.out"
    results: list[dict] = []
    if path.exists():
        for raw in path.read_text(encoding="utf-8", errors="replace").splitlines():
            try:
                row = json.loads(raw)
            except json.JSONDecodeError:
                continue
            if isinstance(row, dict) and "seq" in row:
                results.append(row)
    return results


def seq_of(results: list[dict], prefix: str, after: int = 0) -> int | None:
    """The seq of the first command at or after `after` whose line starts with
    `prefix` (case-insensitive)."""
    for row in results:
        line = str(row.get("line", "")).lower()
        if row["seq"] >= after and line.startswith(prefix.lower()):
            return int(row["seq"])
    return None


def ai_days(records: list[dict]) -> dict[str, list[dict]]:
    """ai_day records by faction, in file order."""
    days: dict[str, list[dict]] = {}
    for record in records:
        if record.get("event_type") == "ai_day":
            side = str(telemetry.field(record, "faction"))
            days.setdefault(side, []).append(record)
    return days


def check_caps(days: dict[str, list[dict]]) -> list[tuple[str, str, str]]:
    over, excess, bad_rate, total = [], [], [], 0
    for side, rows in days.items():
        for row in rows:
            total += 1
            speed = str(telemetry.field(row, "speed"))
            orders = telemetry.field(row, "orders")
            rounds = telemetry.field(row, "rounds")
            day = telemetry.field(row, "day")
            cap = cap_for(speed)
            if isinstance(rounds, int) and speed in RATE and rounds > RATE[speed] + 1:
                # A day whose speed rose mid-day ran rounds of the slower
                # speed (the Advice index holds Very Slow while open), so only
                # the daily cap of 10 binds (+0x428, FUN_00485b40.c:21).
                cap = cap_for("")
            if isinstance(orders, int) and orders > cap:
                over.append(f"{side} day {day}: {orders} at {speed} (cap {cap})")
            if isinstance(orders, int) and isinstance(rounds, int) and orders > rounds:
                excess.append(f"{side} day {day}: {orders} orders in {rounds} rounds")
            rate = telemetry.field(row, "rate")
            if rate is not None and speed in RATE and rate != RATE[speed]:
                bad_rate.append(f"{side} day {day}: rate {rate} at {speed}")
    def verdict(bad: list[str]) -> tuple[str, str]:
        if bad:
            return FAIL, f"{len(bad)} of {total} side-days ({'; '.join(bad[:4])})"
        return PASS, f"{total} side-days"
    return [
        ("orders-cap", *verdict(over)),
        ("orders-per-round", *verdict(excess)),
        ("rate-field", *verdict(bad_rate)),
    ]


def check_rounds(days: dict[str, list[dict]]) -> tuple[str, str, str]:
    bad, checked, skipped = [], 0, {"first": 0, "speed-change": 0, "paused": 0}
    for side, rows in days.items():
        previous = None
        for index, row in enumerate(rows):
            speed = str(telemetry.field(row, "speed"))
            if index == 0:
                skipped["first"] += 1
            elif telemetry.field(row, "paused"):
                skipped["paused"] += 1
            elif speed != previous:
                skipped["speed-change"] += 1
            elif speed in RATE:
                checked += 1
                rounds = telemetry.field(row, "rounds")
                if rounds != RATE[speed] + 1:
                    bad.append(
                        f"{side} day {telemetry.field(row, 'day')}: "
                        f"{rounds} rounds at {speed}, want {RATE[speed] + 1}"
                    )
            previous = speed
    note = f"{checked} days checked, skipped {skipped}"
    if bad:
        return "rounds", FAIL, f"{len(bad)} wrong ({'; '.join(bad[:4])}); {note}"
    if not checked:
        return "rounds", SKIP, f"no complete day to check; {note}"
    return "rounds", PASS, note


def check_fallback_cap(records: list[dict], speed: str | None) -> tuple[str, str, str]:
    counts = telemetry.order_counts(records)
    if not counts.actions:
        return "orders-cap", SKIP, "no ai_day and no ai_action records"
    if counts.unknown:
        return "orders-cap", SKIP, "ai_action records carry no faction"
    cap = cap_for(speed or "")
    over = sorted(((tick, side), n) for (tick, side), n in counts.applied.items() if n > cap)
    note = (
        f"FALLBACK: no ai_day records, counted applied ai_action per (record tick, side) "
        f"at cap {cap} ({'--speed ' + speed if speed else 'speed unknown, looser cap'}); "
        f"carried orders count on the next day's tick"
    )
    if over:
        worst = "; ".join(f"day {t} {s}: {n}" for (t, s), n in over[:4])
        return "orders-cap", FAIL, f"{len(over)} side-days over ({worst}); {note}"
    return "orders-cap", PASS, f"{len(counts.applied)} side-days; {note}"


def touches(record: dict, system: str) -> bool:
    return any(
        str(telemetry.field(record, name) or "").lower() == system.lower()
        for name in ("from", "at", "system")
    )


def check_battle_hold(
    records: list[dict], results: list[dict], folder: Path, system: str | None
) -> list[tuple[str, str, str]]:
    battle = seq_of(results, "Battle at")
    wait = seq_of(results, "Wait until battle alert", battle or 0)
    simulate = seq_of(results, "Click control: BattleAlert/Simulate", wait or 0)
    if battle is None or wait is None or simulate is None:
        return [("battle-hold", SKIP, f"commands.out lacks Battle at/Wait until battle alert/Simulate click ({battle}, {wait}, {simulate})")]
    staged = [
        str(r["line"]).split("Battle at", 1)[1].strip().rstrip(":").strip()
        for r in results
        if str(r.get("line", "")).startswith("Battle at")
    ]
    if system is None:
        # The alert's battle is the one the Simulate click resolved.
        fought = [
            str((r.get("details") or {}).get("system"))
            for r in records
            if r.get("event_type") == "combat_space" and isinstance(r.get("seq"), int) and r["seq"] >= simulate
        ]
        system = fought[0] if fought else staged[0]
    later = [name for name in staged if name.lower() != system.lower()]
    ai = [r for r in records if r.get("event_type") == "ai_action" and isinstance(r.get("seq"), int)]
    window = f"seq B={battle} W={wait} S={simulate}, alert at {system}, later pass {later or 'none'}"
    held = [r for r in ai if telemetry.field(r, "outcome") == "held"]
    stray = [
        f"line {r['_line']} held_at={telemetry.field(r, 'held_at')} seq {r['seq']}"
        for r in held
        if str(telemetry.field(r, "held_at") or "").lower() not in [name.lower() for name in later]
        or r["seq"] < simulate
    ]
    out = [(
        "held-at-later-pass",
        FAIL if stray else PASS,
        f"{len(stray)} held outside a later-pass system after the alert ({'; '.join(stray[:4])})" if stray else f"{len(held)} held records; {window}",
    )]
    rounds = [
        r for r in records
        if r.get("event_type") == "trace" and telemetry.field(r, "decision") == "round" and isinstance(r.get("seq"), int)
    ]
    open_window = [r for r in ai if wait < r["seq"] < simulate]
    at_wait = [r for r in ai if r["seq"] == wait]
    applied = [
        r for r in open_window
        if telemetry.field(r, "outcome") in (None, "applied") and touches(r, system)
    ]
    if applied:
        bad = "; ".join(f"line {r['_line']} {telemetry.field(r, 'type')} seq {r['seq']}" for r in applied[:4])
        out.append(("no-applied-while-open", FAIL, f"{len(applied)} applied orders at {system} while the alert was open ({bad})"))
    elif rounds:
        # With the ai trace on, the ai/round records show whether any round ran.
        inside = [r for r in rounds if wait < r["seq"] < simulate]
        verdict = FAIL if inside else PASS
        out.append(("no-round-while-open", verdict, f"{len(inside)} ai/round traces in W+1..S-1, {sum(1 for r in rounds if r['seq'] >= simulate)} from S on; {window}"))
    elif not open_window and not at_wait:
        out.append(("no-applied-while-open", SKIP, f"the AI wrote no ai_action in the window; spent rounds write nothing, so the hold is unproven (run with OPEN_REBELLION_TRACE=ai); {window}"))
    else:
        out.append(("no-applied-while-open", PASS, f"{len(open_window)} ai_action in the open window, none applied at {system}; {len(at_wait)} at the alert tick (not judged); {window}"))
    return out


def check_save_reload(
    days: dict[str, list[dict]], results: list[dict], folder: Path
) -> list[tuple[str, str, str]]:
    save = seq_of(results, "Save:")
    load = seq_of(results, "Load:", save or 0)
    if save is None or load is None:
        return [("save-reload", SKIP, "commands.out lacks a Save: followed by a Load:")]
    failed = [
        f"seq {r['seq']} {r.get('line')}: {r.get('status')}"
        for r in results
        if r["seq"] >= save and r.get("status") not in ("done", "skipped")
    ]
    log = folder / "game.log"
    panics = []
    if log.exists():
        panics = [l for l in log.read_text(errors="replace").splitlines() if "panicked" in l]
    bad = failed + panics[:2]
    out = [("reload-clean", FAIL if bad else PASS, "; ".join(bad[:4]) if bad else f"every command from Save (seq {save}) on ran; no panic")]
    after = [
        (side, r) for side, rows in days.items() for r in rows
        if isinstance(r.get("seq"), int) and r["seq"] > load
    ]
    if not after:
        out.append(("reload-rounds", SKIP, f"no ai_day after Load (seq {load}); the run ended before a day closed"))
        return out
    first: dict[str, dict] = {}
    count: dict[str, int] = {}
    for side, r in after:
        first.setdefault(side, r)
        count[side] = count.get(side, 0) + 1
    problems = []
    for side, r in first.items():
        speed = str(telemetry.field(r, "speed"))
        rounds = telemetry.field(r, "rounds")
        want = RATE.get(speed, -2) + 1
        if rounds != want:
            problems.append(f"{side} day {telemetry.field(r, 'day')}: {rounds} rounds at {speed}, want {want}")
        if count[side] < 2:
            problems.append(f"{side}: only {count[side]} ai_day after the load")
    detail = ", ".join(f"{s} first day {telemetry.field(r, 'day')} rounds {telemetry.field(r, 'rounds')}/{telemetry.field(r, 'speed')}" for s, r in first.items())
    out.append(("reload-rounds", FAIL if problems else PASS, "; ".join(problems) if problems else detail))
    return out


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("dir", type=Path)
    parser.add_argument("--battle-hold", nargs="?", const="", metavar="SYSTEM")
    parser.add_argument("--save-reload", action="store_true")
    parser.add_argument("--speed", choices=sorted(RATE), help="fallback cap speed when ai_day is absent")
    args = parser.parse_args()

    try:
        records = telemetry.load(args.dir)
    except OSError as error:
        print(f"{FAIL} telemetry: {error}")
        return 1
    results = read_commands(args.dir)
    days = ai_days(records)

    rows: list[tuple[str, str, str]] = []
    if days:
        rows += check_caps(days)
        rows.append(check_rounds(days))
    else:
        rows.append(check_fallback_cap(records, args.speed))
        rows.append(("rounds", SKIP, "no ai_day records (the run predates them or launched without --telemetry)"))
    if args.battle_hold is not None:
        rows += check_battle_hold(records, results, args.dir, args.battle_hold or None)
    if args.save_reload:
        rows += check_save_reload(days, results, args.dir)

    for name, verdict, detail in rows:
        print(f"{verdict} {name}: {detail}")
    return 1 if any(v == FAIL for _, v, _ in rows) else 0


if __name__ == "__main__":
    sys.exit(main())
