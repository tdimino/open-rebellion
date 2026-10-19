#!/usr/bin/env python3
"""Calibrate the simulation: run the headless playtest over many seeds and
report the distributions it produces.

    scripts/calibrate.py --seeds 1..200 --ticks 400 [--dual-ai] [--jobs N]
                         [--out DIR] [--dir REPO_COPY] [--host H]
                         [--trace SPEC] [--no-build] [--reuse] [--from DIR]
                         [--reference FILE] [--speed SPEED]

The playtest builds once on the Mac Mini (scripts/mini-cargo.sh --keep, so
the Mini's other build units stay), then the built binary runs once per seed
over ssh, --jobs seeds at a time. Each seed's JSONL comes back to
`OUT/seeds/seed-S.jsonl` and is deleted on the Mini. --seeds takes `A..B` or a
comma list. --out defaults to `.artifacts/calibration/<UTC stamp>`. --dir is
the Mini's repository copy (default $OPEN_REBELLION_BUILD_DIR or
or-native/open-rebellion, the shared copy). --reuse keeps a seed file already
in OUT/seeds; --from DIR analyzes the seed-*.jsonl files in DIR and runs
nothing.

The report (OUT/report.md, with one CSV per table) holds:

- mission success rate by kind (95% Wilson interval), and, from the missions
  traces the playtest records (--trace, default `missions`; decision
  member_roll with kind, chance, skill, succeeded), the logged odds against the
  realized frequency by odds bucket, and by skill bucket; missions/table_roll
  traces get their own table (logged chance against realized pass rate);
- battle outcomes, and the Alliance win rate by strength ratio. Strength is
  the hull each side brought (`combat_space` alliance_before / empire_before; the
  combat/pre_battle trace holds the same figures but no winner), with the Alliance as the attacker as the log names it;
- orders per side per day from `ai_action` events (telemetry.order_counts), graded
  against the original's bound for the run's speed. Each `ai_day` record
  carries a side-day's speed and counted orders, and each is graded against
  its own speed's bound; --speed (fast, medium, slow, very_slow; default fast,
  the playtest's own setting) names the speed only for logs without them;
- days (ticks) to reach each research level per side and tree, across the
  seeds that reached it;
- control share over time: the campaign snapshots every 250 ticks, and
  systems changing hands per 100 ticks.

Reference values live in scripts/calibration-reference.json. Each carries a
`src` (a Ghidra function, GNPRTB parameter, manual page or VM capture); a
metric with no cited value stays `TODO` and the report lists it as such. The
script never fills a reference in from its own runs. Standard library only.
"""

from __future__ import annotations

import argparse
import csv
import json
import math
import os
import shlex
import statistics
import subprocess
import sys
from collections import Counter, defaultdict
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime, timezone
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import telemetry  # noqa: E402

REPO = Path(__file__).resolve().parent.parent
DEFAULT_REFERENCE = Path(__file__).with_name("calibration-reference.json")
DEFAULT_DIR = os.environ.get("OPEN_REBELLION_BUILD_DIR", "or-native/open-rebellion")
DEFAULT_HOST = os.environ.get("OPEN_REBELLION_BUILD_HOST", "mac-mini-ts")

RATIO_BINS = (0.0, 0.5, 0.8, 1.25, 2.0, math.inf)


# ── running seeds ──────────────────────────────────────────────────────────


def parse_seeds(text: str) -> list[int]:
    seeds: list[int] = []
    for part in text.split(","):
        if ".." in part:
            low, high = part.split("..", 1)
            seeds.extend(range(int(low), int(high) + 1))
        else:
            seeds.append(int(part))
    return seeds


def build(host: str, directory: str) -> None:
    """Build the playtest once on the Mini, into `directory`'s copy."""
    env = dict(os.environ, OPEN_REBELLION_BUILD_HOST=host, OPEN_REBELLION_BUILD_DIR=directory, CARGO_TERM_COLOR="never")
    command = [
        str(REPO / "scripts" / "mini-cargo.sh"), "--keep", "--",
        "build", "--release", "-p", "rebellion-playtest", "--color", "never",
    ]
    print("building rebellion-playtest on", host, directory, file=sys.stderr)
    done = subprocess.run(command, env=env, cwd=REPO)
    if done.returncode != 0:
        raise SystemExit(f"build failed ({done.returncode})")


def run_seed(seed: int, args: argparse.Namespace, stamp: str, seeds_dir: Path) -> Path | None:
    """Run one seed on the Mini and pull its JSONL. None when it failed."""
    rel = f".artifacts/calibration/{stamp}"  # relative to the repository copy
    remote = f"{args.dir}/{rel}"
    name = f"seed-{seed}"
    play = [
        "target/release/rebellion-playtest", args.data, "--seed", str(seed),
        "--ticks", str(args.ticks), "--output", f"{rel}/{name}.jsonl", "--summary",
    ]
    if args.dual_ai:
        play.append("--dual-ai")
    if args.trace:
        play += ["--trace", args.trace]
    script = (
        f"cd {shlex.quote(args.dir)} && mkdir -p {shlex.quote(rel)} && "
        f"{shlex.join(play)} >{shlex.quote(rel)}/{name}.summary.txt "
        f"2>{shlex.quote(rel)}/{name}.log"
    )
    ran = subprocess.run(["ssh", args.host, script], capture_output=True, text=True)
    if ran.returncode != 0:
        print(f"seed {seed}: playtest failed ({ran.returncode}) {ran.stderr.strip()[:200]}", file=sys.stderr)
        return None
    pulled = subprocess.run(
        ["rsync", "-az", *(f"{args.host}:{shlex.quote(remote)}/{name}.{ext}" for ext in ("jsonl", "summary.txt", "log")), f"{seeds_dir}/"],
        capture_output=True, text=True,
    )
    if pulled.returncode != 0:
        print(f"seed {seed}: pull failed {pulled.stderr.strip()[:200]}", file=sys.stderr)
        return None
    subprocess.run(["ssh", args.host, f"cd {shlex.quote(args.dir)} && rm -f {shlex.quote(rel)}/{name}.jsonl {shlex.quote(rel)}/{name}.summary.txt {shlex.quote(rel)}/{name}.log"], capture_output=True)
    return seeds_dir / f"{name}.jsonl"


# ── accumulating ───────────────────────────────────────────────────────────


class Acc:
    """Everything the report needs, summed over seeds."""

    def __init__(self) -> None:
        self.seeds = 0
        self.events = 0
        self.final_ticks: list[int] = []
        self.missions: Counter[tuple[str, bool]] = Counter()
        # (kind, odds, skill, success) from trace records
        self.mission_traces: list[tuple[str, float, float | None, bool]] = []
        # (table, chance, passed) from missions table_roll traces
        self.table_rolls: list[tuple[str, float, bool]] = []
        # (ratio, attacker_won, source "hull"); draws are counted apart
        self.battles: list[tuple[float, bool, str]] = []
        self.battle_outcomes: Counter[str] = Counter()
        # (side, tree) -> level -> ticks across seeds
        self.research: dict[tuple[str, str], dict[int, list[int]]] = defaultdict(lambda: defaultdict(list))
        self.snapshots: dict[int, list[tuple[int, int, int]]] = defaultdict(list)
        self.control_changes: Counter[tuple[int, str]] = Counter()
        self.speed = "fast"
        # (speed, counted orders) of each ai_day record, across seeds
        self.ai_days: list[tuple[str, int]] = []
        # side -> orders in each side-day, across seeds
        self.order_days: dict[str, list[int]] = defaultdict(list)


def as_float(value) -> float | None:
    if isinstance(value, bool) or value is None:
        return None
    try:
        return float(value)
    except (TypeError, ValueError):
        return None


def attacker_won(record: dict) -> bool | None:
    winner = telemetry.field(record, "winner")
    if not isinstance(winner, str):
        return None
    winner = winner.lower()
    if winner in ("attacker", "alliance"):
        return True
    if winner in ("defender", "empire"):
        return False
    return None


def hull(side: object) -> float | None:
    return as_float(side.get("hull")) if isinstance(side, dict) else None


def holder(record: dict) -> str | None:
    text = str(telemetry.field(record, "new_control", "to") or "")
    for name in ("Alliance", "Empire"):
        if name in text:
            return name.lower()
    return None


def accumulate(acc: Acc, records: list[dict]) -> None:
    orders = telemetry.order_counts(records)
    for side in orders.sides():
        acc.order_days[side].extend(orders.per_day(side))
    for record in records:
        if record.get("event_type") == "ai_day":
            speed = telemetry.field(record, "speed")
            counted = as_float(telemetry.field(record, "orders"))
            if isinstance(speed, str) and counted is not None:
                acc.ai_days.append((speed, int(counted)))
    acc.seeds += 1
    acc.events += len(records)
    acc.final_ticks.append(max((r.get("tick", 0) for r in records if isinstance(r.get("tick"), int)), default=0))
    for record in records:
        kind = record.get("event_type")
        tick = record.get("tick") if isinstance(record.get("tick"), int) else 0
        body = telemetry.details(record)
        sub = telemetry.trace_subsystem(record)
        if sub is not None:
            trace_record(acc, record)
            continue
        if kind == "mission_resolved" and "parent_kind" not in body:
            acc.missions[(str(body.get("kind", "?")), body.get("outcome") == "Success")] += 1
        elif kind == "combat_space":
            space_battle(acc, record)
        elif kind == "research_unlocked":
            side = "alliance" if body.get("faction_is_alliance") else "empire"
            tree = str(body.get("tech_type", "?"))
            level = body.get("new_level")
            if isinstance(level, int):
                # The level the unlock names, at its tick; levels held from
                # the start, or skipped past, are not credited.
                acc.research[(side, tree)][level].append(tick)
        elif kind == "campaign_snapshot":
            counts = tuple(body.get(k) for k in ("alliance_systems", "empire_systems", "neutral_systems"))
            if all(isinstance(c, int) for c in counts):
                acc.snapshots[tick].append(counts)  # type: ignore[arg-type]
        elif kind == "control_changed":
            who = holder(record)
            if who:
                acc.control_changes[(tick // 100 * 100, who)] += 1


def trace_record(acc: Acc, record: dict) -> None:
    """missions traces by `details.decision`: member_roll (chance percent,
    skill, succeeded, `kind` the mission kind) and table_roll (table, chance,
    passed), kept apart."""
    if record.get("system") != "missions":
        return
    body = telemetry.details(record)
    decision = body.get("decision")
    odds = as_float(body.get("chance"))
    if odds is None:
        return
    if decision == "member_roll" and isinstance(body.get("succeeded"), bool):
        acc.mission_traces.append((str(body.get("kind") or "?"), odds / 100, as_float(body.get("skill")), body["succeeded"]))
    elif decision == "table_roll" and isinstance(body.get("passed"), bool):
        acc.table_rolls.append((str(body.get("table", "?")), odds / 100, body["passed"]))


def space_battle(acc: Acc, record: dict) -> None:
    body = telemetry.details(record)
    winner = str(body.get("winner", "?"))
    acc.battle_outcomes["stalemate" if body.get("stalemate") else winner] += 1
    if winner == "draw" or body.get("stalemate"):
        return
    ours, theirs = hull(body.get("alliance_before")), hull(body.get("empire_before"))
    won = attacker_won(record)
    if ours is not None and theirs and won is not None:
        acc.battles.append((ours / theirs, won, "hull"))


# ── statistics ─────────────────────────────────────────────────────────────


def wilson(wins: int, total: int) -> tuple[float, float]:
    """95% Wilson score interval for a rate."""
    if total == 0:
        return (0.0, 0.0)
    z = 1.96
    p = wins / total
    denom = 1 + z * z / total
    centre = (p + z * z / (2 * total)) / denom
    half = z * math.sqrt(p * (1 - p) / total + z * z / (4 * total * total)) / denom
    return (max(0.0, centre - half), min(1.0, centre + half))


def pct(value: float) -> str:
    return f"{100 * value:.1f}%"


def bin_label(edges: tuple[float, ...], index: int) -> str:
    low, high = edges[index], edges[index + 1]
    return f"{low:g}+" if math.isinf(high) else f"{low:g}-{high:g}"


def bucket(value: float, edges: tuple[float, ...]) -> int:
    for index in range(len(edges) - 1):
        if edges[index] <= value < edges[index + 1]:
            return index
    return len(edges) - 2


# ── tables ─────────────────────────────────────────────────────────────────

Table = tuple[list[str], list[list]]


def table_missions(acc: Acc) -> Table:
    rows = []
    for kind in sorted({k for k, _ in acc.missions}):
        wins, losses = acc.missions[(kind, True)], acc.missions[(kind, False)]
        total = wins + losses
        low, high = wilson(wins, total)
        rows.append([kind, total, wins, f"{wins / total:.4f}", f"{low:.4f}", f"{high:.4f}"])
    return (["kind", "resolved", "successes", "rate", "ci95_low", "ci95_high"], rows)


def table_odds(acc: Acc) -> Table:
    groups: dict[tuple[str, int], list[tuple[float, bool]]] = defaultdict(list)
    for kind, odds, _, win in acc.mission_traces:
        chance = odds
        groups[(kind, min(int(chance * 10), 9))].append((chance, win))
    rows = []
    for (kind, index), items in sorted(groups.items()):
        wins = sum(1 for _, w in items if w)
        low, high = wilson(wins, len(items))
        rows.append([
            kind, f"{index / 10:.1f}-{(index + 1) / 10:.1f}", len(items),
            f"{statistics.fmean(o for o, _ in items):.4f}", f"{wins / len(items):.4f}",
            f"{low:.4f}", f"{high:.4f}",
        ])
    return (["kind", "odds_bucket", "n", "mean_logged_odds", "realized", "ci95_low", "ci95_high"], rows)


def table_tables(acc: Acc) -> Table:
    groups: dict[str, list[tuple[float, bool]]] = defaultdict(list)
    for name, chance, passed in acc.table_rolls:
        groups[name].append((chance, passed))
    rows = []
    for name, items in sorted(groups.items()):
        wins = sum(1 for _, p in items if p)
        low, high = wilson(wins, len(items))
        rows.append([name, len(items), f"{statistics.fmean(c for c, _ in items):.4f}", f"{wins / len(items):.4f}", f"{low:.4f}", f"{high:.4f}"])
    return (["table", "n", "mean_logged_chance", "realized_pass", "ci95_low", "ci95_high"], rows)


def table_skill(acc: Acc) -> Table:
    groups: dict[tuple[str, int], list[bool]] = defaultdict(list)
    for kind, _, skill, win in acc.mission_traces:
        if skill is not None:
            groups[(kind, min(int(skill // 20), 4))].append(win)
    rows = []
    for (kind, index), items in sorted(groups.items()):
        rows.append([kind, f"{index * 20}-{index * 20 + 19}", len(items), f"{sum(items) / len(items):.4f}"])
    return (["kind", "skill_bucket", "n", "realized"], rows)


def table_battles(acc: Acc) -> Table:
    rows = []
    for source in ("hull",):
        items = [(r, w) for r, w, s in acc.battles if s == source]
        if not items:
            continue
        for index in range(len(RATIO_BINS) - 1):
            inside = [w for r, w in items if bucket(r, RATIO_BINS) == index]
            if not inside:
                continue
            wins = sum(inside)
            low, high = wilson(wins, len(inside))
            rows.append([source, bin_label(RATIO_BINS, index), len(inside), f"{wins / len(inside):.4f}", f"{low:.4f}", f"{high:.4f}"])
    return (["strength_source", "attacker_to_defender_ratio", "n", "attacker_win_rate", "ci95_low", "ci95_high"], rows)


def table_research(acc: Acc, seeds: int) -> Table:
    rows = []
    for (side, tree), levels in sorted(acc.research.items()):
        for level in sorted(levels):
            ticks = levels[level]
            rows.append([
                side, tree, level, f"{len(ticks)}/{seeds}", statistics.median(ticks),
                f"{statistics.fmean(ticks):.1f}", min(ticks), max(ticks),
            ])
    return (["side", "tree", "level", "seeds_reached", "median_tick", "mean_tick", "min_tick", "max_tick"], rows)


def table_orders(acc: Acc) -> Table:
    rows = []
    for side in sorted(acc.order_days):
        days = sorted(acc.order_days[side])
        rows.append([side, len(days), sum(days), f"{statistics.median(days):g}", days[-1]])
    return ["side", "side_days", "orders", "median", "max"], rows


def table_control(acc: Acc) -> Table:
    rows = []
    for tick in sorted(acc.snapshots):
        shots = acc.snapshots[tick]
        share = [statistics.fmean(s[i] / max(sum(s), 1) for s in shots) for i in range(3)]
        rows.append([tick, len(shots), *(f"{value:.4f}" for value in share)])
    return (["tick", "seeds", "alliance_share", "empire_share", "neutral_share"], rows)


def table_changes(acc: Acc) -> Table:
    buckets = sorted({b for b, _ in acc.control_changes})
    rows = [[b, acc.control_changes[(b, "alliance")], acc.control_changes[(b, "empire")]] for b in buckets]
    return (["tick_from", "to_alliance", "to_empire"], rows)


# ── reference ──────────────────────────────────────────────────────────────


def load_reference(path: Path) -> dict:
    if not path.exists():
        return {}
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def observed_scalars(acc: Acc) -> dict[str, float]:
    levels = [level for tree in acc.research.values() for level in tree]
    scalars: dict[str, float] = {}
    if levels:
        scalars["research.max_level_observed"] = max(levels)
    days = [n for values in acc.order_days.values() for n in values]
    if days:
        scalars["orders.max_per_side_day"] = max(days)
    return scalars


def odds_check(acc: Acc) -> list[str]:
    """Consistency check, not parity evidence: the Research member_roll
    trace logs chance and skill from one value, so they must agree."""
    pairs = [(odds, skill) for kind, odds, skill, _ in acc.mission_traces if "Research" in kind and skill is not None]
    if not pairs:
        return ["- Research chance vs skill/100 (consistency check): not checked (no traces)"]
    off = [p for p in pairs if abs(p[0] - p[1] / 100) > 0.005]
    verdict = "consistent" if not off else f"inconsistent in {len(off)} of {len(pairs)} traces"
    return [f"- Research chance vs skill/100 (consistency check, ghidra/notes/research.md:31): {verdict} ({len(pairs)} traces)"]


def reference_section(reference: dict, acc: Acc) -> list[str]:
    out = ["## Reference comparison", ""]
    if not reference:
        return out + ["No reference file loaded.", ""]
    scalars = observed_scalars(acc)
    out += ["| Reference | Value | Observed | Verdict | src |", "|---|---|---|---|---|"]
    for name, entry in reference.get("constants", {}).items():
        metric = entry.get("metric")
        value = json.dumps({k: entry[k] for k in ("value", "min", "max", "spread") if k in entry})
        observed, verdict = "-", "informational"
        if metric and metric in scalars:
            observed = f"{scalars[metric]:g}"
            low, high = entry.get("min"), entry.get("max")
            by_speed = entry.get("max_by_speed")
            ok = None
            if by_speed and acc.ai_days:
                # Each side-day against its own speed's bound.
                speeds = sorted({speed for speed, _ in acc.ai_days})
                value = ", ".join(f"max {by_speed.get(speed)} at {speed}" for speed in speeds)
                ok = all(by_speed.get(speed) is None or n <= by_speed[speed] for speed, n in acc.ai_days)
            elif by_speed:
                high = by_speed.get(acc.speed)
                value = f"max {high} at {acc.speed}"
            if ok is None:
                ok = (low is None or scalars[metric] >= low) and (high is None or scalars[metric] <= high)
            verdict = "within" if ok else "OUTSIDE"
        out.append(f"| {name} | {value} | {observed} | {verdict} | {entry.get('src', 'MISSING')} |")
    out += [""] + odds_check(acc)
    todo = [name for name, entry in reference.get("metrics", {}).items() if entry.get("value") is None]
    if todo:
        out += ["", "No cited reference yet (TODO):", ""] + [f"- {name}" for name in todo]
    return out + [""]


# ── report ─────────────────────────────────────────────────────────────────


def speed_line(acc: Acc) -> str:
    if acc.ai_days:
        speeds = ", ".join(sorted({speed for speed, _ in acc.ai_days}))
        return (f"Speed {speeds} (from {len(acc.ai_days)} ai_day records; each side-day is graded at its own"
                " speed in the reference comparison).")
    return f"Speed {acc.speed} (from --speed; the log has no ai_day records). Graded in the reference comparison."


def markdown(headers: list[str], rows: list[list]) -> list[str]:
    if not rows:
        return ["(no data)", ""]
    lines = ["| " + " | ".join(headers) + " |", "|" + "---|" * len(headers)]
    lines += ["| " + " | ".join(str(c) for c in row) + " |" for row in rows]
    return lines + [""]


def write_csv(path: Path, table: Table) -> None:
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.writer(handle)
        writer.writerow(table[0])
        writer.writerows(table[1])


def report(acc: Acc, args: argparse.Namespace, out: Path, reference: dict) -> Path:
    tables = {
        "missions": table_missions(acc),
        "mission-odds": table_odds(acc),
        "mission-skill": table_skill(acc),
        "table-rolls": table_tables(acc),
        "battles": table_battles(acc),
        "research": table_research(acc, acc.seeds),
        "orders": table_orders(acc),
        "control": table_control(acc),
        "control-changes": table_changes(acc),
    }
    for name, table in tables.items():
        write_csv(out / f"{name}.csv", table)
    ticks = acc.final_ticks
    lines = [
        "# Calibration report", "",
        f"- seeds analyzed: {acc.seeds} ({args.seeds if args.seeds else args.from_dir})",
        f"- ticks requested: {args.ticks}, final tick median {statistics.median(ticks) if ticks else 0}",
        f"- dual AI: {args.dual_ai}", f"- events: {acc.events}", "",
        "## Mission success by kind", "",
        *markdown(*tables["missions"]),
        "## Logged odds against realized success (trace records)", "",
        *markdown(*tables["mission-odds"]),
        "## Realized success by skill bucket (trace records)", "",
        *markdown(*tables["mission-skill"]),
        "## Table rolls (missions table_roll traces)", "",
        *markdown(*tables["table-rolls"]),
        "## Space battles", "",
        "Outcomes: " + (", ".join(f"{k} {v}" for k, v in sorted(acc.battle_outcomes.items())) or "none"), "",
        *markdown(*tables["battles"]),
        "## Orders per side per day", "",
        speed_line(acc), "",
        *markdown(*tables["orders"]),
        "## Days to research level", "",
        "Ticks are game days since the start. `seeds_reached` is how many runs got that far.", "",
        *markdown(*tables["research"]),
        "## Control share over time", "",
        *markdown(*tables["control"]),
        "Systems changing hands per 100 ticks:", "",
        *markdown(*tables["control-changes"]),
        *reference_section(reference, acc),
    ]
    path = out / "report.md"
    path.write_text("\n".join(lines), encoding="utf-8")
    return path


# ── main ───────────────────────────────────────────────────────────────────


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0], formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--seeds", help="A..B or a comma list")
    parser.add_argument("--ticks", type=int, default=400)
    parser.add_argument("--dual-ai", action="store_true")
    parser.add_argument("--jobs", type=int, default=4)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--dir", default=DEFAULT_DIR, help="the Mini's repository copy")
    parser.add_argument("--host", default=DEFAULT_HOST)
    parser.add_argument("--data", default="data/base", help="data dir on the Mini, relative to --dir")
    parser.add_argument("--trace", default="missions", help="playtest --trace subsystems; empty for none")
    parser.add_argument("--no-build", action="store_true")
    parser.add_argument("--reuse", action="store_true", help="keep seed files already in OUT/seeds")
    parser.add_argument("--from", dest="from_dir", type=Path, help="analyze seed-*.jsonl here; run nothing")
    parser.add_argument("--reference", type=Path, default=DEFAULT_REFERENCE)
    parser.add_argument("--speed", default="fast", choices=["fast", "medium", "slow", "very_slow"],
                        help="the run's game speed for the orders bound when the log has no ai_day records (default fast)")
    args = parser.parse_args()
    if not args.seeds and not args.from_dir:
        parser.error("give --seeds or --from")

    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    out = args.out or REPO / ".artifacts" / "calibration" / stamp
    out.mkdir(parents=True, exist_ok=True)
    files: list[Path] = []
    if args.from_dir:
        files = sorted(args.from_dir.glob("seed-*.jsonl"))
    else:
        seeds_dir = out / "seeds"
        seeds_dir.mkdir(exist_ok=True)
        seeds = parse_seeds(args.seeds)
        if not args.no_build:
            build(args.host, args.dir)
        todo = [s for s in seeds if not (args.reuse and (seeds_dir / f"seed-{s}.jsonl").exists())]
        with ThreadPoolExecutor(max_workers=max(1, args.jobs)) as pool:
            for seed, path in zip(todo, pool.map(lambda s: run_seed(s, args, stamp, seeds_dir), todo)):
                if path is None:
                    print(f"seed {seed} dropped", file=sys.stderr)
        files = [seeds_dir / f"seed-{s}.jsonl" for s in seeds if (seeds_dir / f"seed-{s}.jsonl").exists()]
    if not files:
        print("no seed files to analyze", file=sys.stderr)
        return 1

    acc = Acc()
    acc.speed = args.speed
    for path in files:
        accumulate(acc, telemetry.load(path))
    path = report(acc, args, out, load_reference(args.reference))
    print(path)
    return 0


if __name__ == "__main__":
    sys.exit(main())
