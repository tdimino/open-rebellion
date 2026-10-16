#!/usr/bin/env python3
"""Query game telemetry: native `telemetry.jsonl` or a headless playtest log.

    scripts/telemetry.py DIR|FILE [--sys S] [--type T] [--tick A..B]
                         [--seq N] [--trace S] [--json]
    scripts/telemetry.py DIR|FILE --orders [--cap N]
    scripts/telemetry.py DIR|FILE --strip-wall [-o OUT]

DIR is a native evidence folder (its `telemetry.jsonl` is read); FILE is any
JSONL of GameEventRecord lines, such as `rebellion-playtest --output`. Each
line carries `tick`, `wall_ms`, `system`, `event_type` and `details`. Native
runs add `source` ("native"; headless is "headless", and a line without it is
taken as headless), an optional `seq` (the developer command in flight) and
`trace` records (event_type `trace`; `system` is the subsystem: missions,
ai, research, blockade, combat, control; `details.kind` names the decision). Missing fields
are tolerated everywhere.

Filters combine with AND. --sys and --type match the system and event type
exactly. --tick takes `A..B` (inclusive; either end may be omitted, `N` alone
means one tick). --seq keeps the records written under one developer command.
--trace S keeps only trace records of subsystem S. Without --json each match
prints as one line: tick, seq, system/type, details. With --json the matching
lines print unchanged, so the output pipes into jq or another run. A summary
of counts by system/type goes to stderr when the output is a terminal.

--orders prints the orders-per-side-per-day readout from `ai_action` events
(see `order_counts`): per faction the median and max orders in a side-day, the
share of side-days at or above --cap (default 5) and the total, with refused
and held counts. A day is the order's `issued` day, else the record's `tick`. An event with an `outcome`
counts only when it is `applied`; one without counts always. Events with no
faction count under "unknown". --strip-wall writes the JSONL to stdout (or -o
OUT) with every `wall_ms` key removed at any depth and keys sorted, so two runs
compare byte for byte.

Also importable: `load(path)`, `matches(record, ...)` and `order_counts(records)`
serve calibrate.py, invariants.py and dev-report.py. Standard library only.
"""

from __future__ import annotations

import argparse
import json
import statistics
import sys
from collections import Counter
from pathlib import Path


def resolve(path: Path) -> Path:
    """The JSONL a DIR or FILE argument names."""
    return path / "telemetry.jsonl" if path.is_dir() else path


def load(path: Path) -> list[dict]:
    """Every JSON object line of a telemetry file, in file order.

    Blank lines, non-JSON lines (a trailing human summary) and non-object
    JSON are skipped. Each record gains `_line`, its 1-based line number.
    """
    records: list[dict] = []
    with resolve(path).open(encoding="utf-8", errors="replace") as handle:
        for number, raw in enumerate(handle, 1):
            raw = raw.strip()
            if not raw.startswith("{"):
                continue
            try:
                record = json.loads(raw)
            except json.JSONDecodeError:
                continue
            if isinstance(record, dict):
                record["_line"] = number
                records.append(record)
    return records


def details(record: dict) -> dict:
    value = record.get("details")
    return value if isinstance(value, dict) else {}


def field(record: dict, *names: str):
    """The first of `names` found at the record's top level, else in details."""
    body = details(record)
    for name in names:
        if name in record:
            return record[name]
        if name in body:
            return body[name]
    return None


def is_trace(record: dict) -> bool:
    return record.get("event_type") == "trace"


def trace_subsystem(record: dict) -> str | None:
    """The subsystem a trace record names (its `system`), or None for any
    other record."""
    if not is_trace(record):
        return None
    value = record.get("system")
    return None if value is None else str(value)


def parse_ticks(text: str) -> tuple[int | None, int | None]:
    if ".." not in text:
        return int(text), int(text)
    low, high = text.split("..", 1)
    return (int(low) if low else None, int(high) if high else None)


def matches(
    record: dict,
    system: str | None = None,
    event_type: str | None = None,
    ticks: tuple[int | None, int | None] = (None, None),
    seq: int | None = None,
    trace: str | None = None,
) -> bool:
    if system is not None and record.get("system") != system:
        return False
    if event_type is not None and record.get("event_type") != event_type:
        return False
    tick = record.get("tick")
    low, high = ticks
    if low is not None and (not isinstance(tick, int) or tick < low):
        return False
    if high is not None and (not isinstance(tick, int) or tick > high):
        return False
    if seq is not None:
        try:
            if int(record.get("seq")) != seq:
                return False
        except (TypeError, ValueError):
            return False
    if trace is not None and trace_subsystem(record) != trace:
        return False
    return True


UNKNOWN_SIDE = "unknown"


class OrderCounts:
    """AI orders per (day, faction), from `ai_action` records."""

    def __init__(self) -> None:
        self.applied: Counter[tuple[object, str]] = Counter()
        self.refused: Counter[str] = Counter()
        self.held: Counter[str] = Counter()
        self.actions = 0
        self.unknown = 0  # ai_action records with no faction

    def sides(self) -> list[str]:
        names = {side for _, side in self.applied} | set(self.refused) | set(self.held)
        return sorted(names)

    def per_day(self, side: str) -> list[int]:
        """Sorted order counts of each side-day on which `side` issued any."""
        return sorted(n for (_, who), n in self.applied.items() if who == side)


def order_counts(records: list[dict]) -> OrderCounts:
    """Count AI orders by (day, faction). An `ai_action` with an `outcome`
    counts only when `applied` (others tally as refused or held per side);
    one without counts always. Day is the day the order was issued
    (`issued`; a day's last round applies the next day), else the record's
    `tick`; the faction is read from the top level or `details`, else
    counted as "unknown"."""
    counts = OrderCounts()
    for record in records:
        if record.get("event_type") != "ai_action":
            continue
        counts.actions += 1
        faction = field(record, "faction")
        if faction is None:
            counts.unknown += 1
        side = UNKNOWN_SIDE if faction is None else str(faction)
        outcome = field(record, "outcome")
        if outcome is None or outcome == "applied":
            day = field(record, "issued")
            counts.applied[(record.get("tick") if day is None else day, side)] += 1
        elif outcome == "refused":
            counts.refused[side] += 1
        elif outcome == "held":
            counts.held[side] += 1
    return counts


def orders_readout(records: list[dict], cap: int) -> list[str]:
    counts = order_counts(records)
    if not counts.actions:
        return ["no ai_action records"]
    lines = []
    for side in counts.sides():
        days = counts.per_day(side)
        row = f"{side:10} orders {sum(days):>6}"
        if days:
            at_cap = sum(1 for n in days if n >= cap)
            row += (f"  side-days {len(days):>5}  median {statistics.median(days):g}  max {days[-1]}"
                    f"  at/over cap {cap}: {100 * at_cap / len(days):.1f}%")
        row += f"  refused {counts.refused[side]}  held {counts.held[side]}"
        lines.append(row)
    lines.append(f"total applied orders {sum(counts.applied.values())}")
    if counts.unknown:
        lines.append(f"{counts.unknown} ai_action records carry no faction (counted under \"{UNKNOWN_SIDE}\")")
    return lines


def strip_key(value, key: str):
    """`value` with `key` removed from every dict at any depth."""
    if isinstance(value, dict):
        return {k: strip_key(v, key) for k, v in value.items() if k != key}
    if isinstance(value, list):
        return [strip_key(v, key) for v in value]
    return value


def render(record: dict) -> str:
    """One display line for a record."""
    seq = record.get("seq")
    where = f"#{seq}" if seq is not None else "-"
    source = record.get("source")
    label = f"{record.get('system', '?')}/{record.get('event_type', '?')}"
    sub = trace_subsystem(record)
    if sub:
        label += f"[{sub}]"
    body = {k: v for k, v in details(record).items()}
    text = json.dumps(body, separators=(",", ":"), ensure_ascii=False) if body else ""
    tag = f" ({source})" if source else ""
    return f"t{record.get('tick', '?'):>6} {where:>5} {label}{tag} {text}".rstrip()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("path", type=Path, help="evidence DIR or telemetry JSONL FILE")
    parser.add_argument("--sys", dest="system", help="system name, e.g. combat")
    parser.add_argument("--type", dest="event_type", help="event type, e.g. combat_space")
    parser.add_argument("--tick", help="A..B inclusive; A.., ..B or N")
    parser.add_argument("--seq", type=int, help="developer command number")
    parser.add_argument("--trace", help="trace subsystem")
    parser.add_argument("--json", action="store_true", help="print matching lines as JSON")
    parser.add_argument("--orders", action="store_true", help="orders per side per day from ai_action events")
    parser.add_argument("--cap", type=int, default=5, help="--orders: cap per side-day (default 5)")
    parser.add_argument("--strip-wall", action="store_true", help="write the JSONL without wall_ms, keys sorted")
    parser.add_argument("-o", "--out", type=Path, help="--strip-wall: write here instead of stdout")
    args = parser.parse_args()

    target = resolve(args.path)
    if not target.is_file():
        print(f"no telemetry at {target}", file=sys.stderr)
        return 2
    try:
        ticks = parse_ticks(args.tick) if args.tick else (None, None)
    except ValueError:
        print(f"bad --tick {args.tick!r}: use A..B", file=sys.stderr)
        return 2

    if args.strip_wall:
        lines = [
            json.dumps(strip_key({k: v for k, v in r.items() if k != "_line"}, "wall_ms"),
                       sort_keys=True, separators=(",", ":"), ensure_ascii=False)
            for r in load(target)
        ]
        text = "".join(line + "\n" for line in lines)
        if args.out:
            args.out.write_text(text, encoding="utf-8")
        else:
            sys.stdout.write(text)
        return 0
    if args.orders:
        print("\n".join(orders_readout(load(target), args.cap)))
        return 0

    counts: Counter[str] = Counter()
    for record in load(target):
        if not matches(record, args.system, args.event_type, ticks, args.seq, args.trace):
            continue
        counts[f"{record.get('system', '?')}/{record.get('event_type', '?')}"] += 1
        shown = {k: v for k, v in record.items() if k != "_line"} if args.json else None
        print(json.dumps(shown, separators=(",", ":"), ensure_ascii=False) if args.json else render(record))
    if sys.stdout.isatty() and counts:
        print(f"\n{sum(counts.values())} records", file=sys.stderr)
        for name, count in counts.most_common():
            print(f"  {name:40} {count:>7}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
