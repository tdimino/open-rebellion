#!/usr/bin/env python3
"""Check rules over one telemetry JSONL file.

    scripts/invariants.py DIR|FILE [--cap N]

DIR is a native evidence folder or FILE any playtest JSONL (see
telemetry.py). Each rule prints one PASS, FAIL or SKIP line; the exit status
is 1 when any rule FAILs. Rules are functions in RULES, each taking the
records and the parsed arguments and returning (status, message).

orders-per-side-per-day  Applied AI orders per (day, faction) stay at or
                         under --cap (default 5). An order is an `ai_action`
                         record; when records carry an `outcome`, only
                         `applied` ones count. SKIPs when an ai_action lacks
                         `faction`, which older telemetry does not record.

Standard library only.
"""

from __future__ import annotations

import argparse
import statistics
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from telemetry import load, order_counts  # noqa: E402

PASS, FAIL, SKIP = "PASS", "FAIL", "SKIP"


def orders_per_side_per_day(records: list[dict], args: argparse.Namespace) -> tuple[str, str]:
    counts = order_counts(records)
    if not counts.actions:
        return SKIP, "no ai_action records"
    if counts.unknown:
        return SKIP, "telemetry predates per-order factions (an ai_action has no faction)"
    counts = counts.applied
    if not counts:
        return SKIP, "no applied orders"
    values = sorted(counts.values())
    at_cap = sum(1 for v in values if v >= args.cap)
    summary = (
        f"{len(values)} side-days, median {statistics.median(values):g}, max {values[-1]}, "
        f"{100 * at_cap / len(values):.1f}% at cap {args.cap}"
    )
    over = sorted(((tick, side), n) for (tick, side), n in counts.items() if n > args.cap)
    if over:
        worst = ", ".join(f"day {tick} {side}: {n}" for (tick, side), n in over[:5])
        return FAIL, f"{len(over)} side-days over cap ({worst}); {summary}"
    return PASS, summary


RULES = [
    ("orders-per-side-per-day", orders_per_side_per_day),
]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("path", type=Path, help="evidence folder or JSONL file")
    parser.add_argument("--cap", type=int, default=5, help="orders per side per day (default 5)")
    args = parser.parse_args()
    records = load(args.path)
    failed = False
    for name, rule in RULES:
        status, message = rule(records, args)
        print(f"{status} {name}: {message}")
        failed = failed or status == FAIL
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
