#!/usr/bin/env python3
"""Evaluate a playtest JSONL log against the golden-value oracle.

This script does not assign a balance score. It compares observable JSONL
evidence against the known-original constants captured in
`scripts/golden_values.json` and reports:

- `pass`: the log matches the oracle for that check
- `fail`: the log disagrees with the oracle
- `skip`: the log does not contain enough evidence to evaluate the check

Usage:
    python3 scripts/eval_parity.py campaign.jsonl
    python3 scripts/eval_parity.py campaign.jsonl --json
    python3 scripts/eval_parity.py campaign.jsonl --golden scripts/golden_values.json
"""

from __future__ import annotations

import argparse
import json
import sys
from collections import Counter
from pathlib import Path


DEFAULT_GOLDEN_PATH = Path(__file__).with_name("golden_values.json")


def load_events(path: Path) -> list[dict]:
    """Parse a playtest JSONL log. Skips non-JSON lines (e.g. summary footer)."""
    events: list[dict] = []
    with path.open() as handle:
        for line in handle:
            line = line.strip()
            # Skip blank lines and any trailing human-readable summary.
            if not line or not line.startswith("{"):
                continue
            try:
                events.append(json.loads(line))
            except json.JSONDecodeError:
                # Non-JSON line in the stream — skip silently.
                continue
    return events


def load_golden(path: Path) -> dict:
    with path.open() as handle:
        return json.load(handle)


def make_check(
    name: str,
    status: str,
    expected=None,
    observed=None,
    details: str = "",
) -> dict:
    return {
        "name": name,
        "status": status,
        "expected": expected,
        "observed": observed,
        "details": details,
    }


def unique_event_ticks(events: list[dict], event_type: str) -> list[int]:
    return sorted({int(event.get("tick", 0)) for event in events if event.get("event_type") == event_type})


def dominant_interval(ticks: list[int]) -> int | None:
    deltas = [b - a for a, b in zip(ticks, ticks[1:]) if b > a]
    if not deltas:
        return None
    return Counter(deltas).most_common(1)[0][0]


def event_types_present(events: list[dict]) -> set[str]:
    return {event.get("event_type", "") for event in events}


def event_id_set(events: list[dict]) -> set[int]:
    ids: set[int] = set()
    for event in events:
        if event.get("event_type") != "event_fired":
            continue
        details = event.get("details", {})
        if isinstance(details, dict) and isinstance(details.get("event_id"), int):
            ids.add(details["event_id"])
    return ids


def build_presence_checks(
    events: list[dict],
    prefix: str,
    required_types: list[str],
) -> list[dict]:
    observed = event_types_present(events)
    return [
        make_check(
            name=f"{prefix}.{event_type}",
            status="pass" if event_type in observed else "fail",
            expected=True,
            observed=event_type in observed,
            details=f"event_type={event_type}",
        )
        for event_type in required_types
    ]


def evaluate(events: list[dict], golden: dict) -> dict:
    values = golden["values"]
    checks: list[dict] = []

    if not events:
        checks.append(make_check("log.non_empty", "fail", expected=True, observed=False, details="No JSONL events found."))
        return finalize(events, checks, golden)

    # AI cadence: compare the observed evaluation spacing to both the original
    # and the current throttled implementation.
    ai_ticks = unique_event_ticks(events, "ai_action")
    if len(ai_ticks) < 2:
        checks.append(make_check("ai.interval_vs_original", "skip", expected=values["ai"]["original_tick_interval_days"], observed=None, details="Need at least two unique ai_action ticks."))
        checks.append(make_check("ai.interval_vs_current", "skip", expected=values["ai"]["current_tick_interval_days"], observed=None, details="Need at least two unique ai_action ticks."))
    else:
        observed_ai_interval = dominant_interval(ai_ticks)
        tick_preview = ai_ticks[:10]
        preview_suffix = "..." if len(ai_ticks) > 10 else ""
        details = f"unique ai_action ticks={tick_preview}{preview_suffix}"
        checks.append(
            make_check(
                "ai.interval_vs_original",
                "pass" if observed_ai_interval == values["ai"]["original_tick_interval_days"] else "fail",
                expected=values["ai"]["original_tick_interval_days"],
                observed=observed_ai_interval,
                details=details,
            )
        )
        checks.append(
            make_check(
                "ai.interval_vs_current",
                "pass" if observed_ai_interval == values["ai"]["current_tick_interval_days"] else "fail",
                expected=values["ai"]["current_tick_interval_days"],
                observed=observed_ai_interval,
                details=details,
            )
        )

    # Economy and combat parity-critical event families.
    checks.extend(
        build_presence_checks(
            events,
            "economy",
            ["economy_tick", "support_drift", "collection_rate", "garrison_required", "control_changed"],
        )
    )
    checks.extend(
        build_presence_checks(
            events,
            "combat",
            ["combat_space", "combat_ground", "bombardment"],
        )
    )
    checks.extend(
        build_presence_checks(
            events,
            "repair",
            ["ship_repair_started", "ship_repaired"],
        )
    )

    # Movement: the current implementation clamps arrivals to at least 10 ticks.
    arrival_ticks = unique_event_ticks(events, "fleet_arrived")
    if not arrival_ticks:
        checks.append(make_check("movement.earliest_arrival_tick", "skip", expected=f">={values['movement']['current_min_transit_ticks']}", observed=None, details="No fleet_arrived events in log."))
    else:
        earliest_arrival = min(arrival_ticks)
        checks.append(
            make_check(
                "movement.earliest_arrival_tick",
                "pass" if earliest_arrival >= values["movement"]["current_min_transit_ticks"] else "fail",
                expected=f">={values['movement']['current_min_transit_ticks']}",
                observed=earliest_arrival,
                details=f"arrival_ticks={arrival_ticks[:10]}{'...' if len(arrival_ticks) > 10 else ''}",
            )
        )

    # Death Star events are only meaningful once the log runs long enough for
    # construction to be theoretically possible.
    final_tick = max(int(event.get("tick", 0)) for event in events)
    ds_required = ["death_star_construction", "death_star_fired", "death_star_shield_status"]
    if final_tick < values["death_star"]["construction_ticks"]:
        for event_type in ds_required:
            checks.append(
                make_check(
                    f"death_star.{event_type}",
                    "skip",
                    expected=True,
                    observed=False,
                    details=f"log ended at tick {final_tick}, below construction threshold {values['death_star']['construction_ticks']}",
                )
            )
    else:
        checks.extend(build_presence_checks(events, "death_star", ds_required))

    # Victory timing: if a victory event exists, it must not fire before the
    # current minimum victory tick.
    victory_ticks = unique_event_ticks(events, "victory_check")
    if not victory_ticks:
        checks.append(make_check("victory.first_tick", "skip", expected=f">={values['victory']['current_min_victory_tick']}", observed=None, details="No victory_check events in log."))
    else:
        first_victory_tick = min(victory_ticks)
        checks.append(
            make_check(
                "victory.first_tick",
                "pass" if first_victory_tick >= values["victory"]["current_min_victory_tick"] else "fail",
                expected=f">={values['victory']['current_min_victory_tick']}",
                observed=first_victory_tick,
                details=f"victory_ticks={victory_ticks}",
            )
        )

    # Research: the mechanics docs map the three original completion event IDs.
    fired_ids = event_id_set(events)
    research_ids = {
        values["research"]["research_completion_event_ship"],
        values["research"]["research_completion_event_troop"],
        values["research"]["research_completion_event_facility"],
    }
    if not fired_ids:
        checks.append(make_check("research.completion_event_ids", "skip", expected=sorted(research_ids), observed=[], details="No event_fired records in log."))
    else:
        observed_research_ids = sorted(fired_ids & research_ids)
        checks.append(
            make_check(
                "research.completion_event_ids",
                "pass" if observed_research_ids else "fail",
                expected=sorted(research_ids),
                observed=observed_research_ids,
                details=f"all event_fired ids={sorted(fired_ids)}",
            )
        )

    return finalize(events, checks, golden)


def finalize(events: list[dict], checks: list[dict], golden: dict) -> dict:
    counts = Counter(check["status"] for check in checks)
    total_events = len(events)
    final_tick = max((int(event.get("tick", 0)) for event in events), default=0)
    distinct_event_types = Counter(event.get("event_type", "") for event in events)

    if counts["fail"] > 0:
        overall = "fail"
    elif counts["pass"] > 0:
        overall = "pass"
    else:
        overall = "skip"

    return {
        "overall": overall,
        "summary": {
            "pass": counts["pass"],
            "fail": counts["fail"],
            "skip": counts["skip"],
        },
        "golden_version": golden.get("version"),
        "golden_source": golden.get("source"),
        "observed": {
            "total_events": total_events,
            "final_tick": final_tick,
            "distinct_event_types": len(distinct_event_types),
            "event_type_counts": dict(distinct_event_types),
        },
        "checks": checks,
    }


def print_text_report(result: dict) -> None:
    print(f"Parity Oracle: {result['overall'].upper()}")
    print(
        "Summary: "
        f"{result['summary']['pass']} pass, "
        f"{result['summary']['fail']} fail, "
        f"{result['summary']['skip']} skip"
    )
    print(
        "Observed: "
        f"{result['observed']['total_events']} events, "
        f"final_tick={result['observed']['final_tick']}, "
        f"{result['observed']['distinct_event_types']} event types"
    )
    print(f"Golden: v{result['golden_version']} from {result['golden_source']}")
    print()

    for check in result["checks"]:
        label = check["status"].upper().ljust(4)
        print(f"[{label}] {check['name']}")
        if check["expected"] is not None or check["observed"] is not None:
            print(f"  expected={check['expected']} observed={check['observed']}")
        if check["details"]:
            print(f"  {check['details']}")


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Compare a playtest JSONL log against the golden-value oracle.")
    parser.add_argument("campaign", type=Path, help="Path to rebellion-playtest --jsonl output")
    parser.add_argument("--golden", type=Path, default=DEFAULT_GOLDEN_PATH, help="Path to golden_values.json")
    parser.add_argument("--json", action="store_true", help="Emit machine-readable JSON instead of text")
    return parser.parse_args(argv)


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    events = load_events(args.campaign)
    golden = load_golden(args.golden)
    result = evaluate(events, golden)

    if args.json:
        print(json.dumps(result, indent=2))
    else:
        print_text_report(result)

    return 0 if result["overall"] != "fail" else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
