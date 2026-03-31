#!/usr/bin/env python3
"""
Evaluate a headless playtest JSONL log for original-game parity.

Unlike eval_game_quality.py (which measures gameplay balance), this script
measures how closely the simulation matches the original 1998 Star Wars
Rebellion's feature set. Used by autoresearch Track 1 for the parity loop.

Usage:
    python3 scripts/eval_parity.py campaign.jsonl
    python3 scripts/eval_parity.py campaign.jsonl --json

Composite score formula (6 sub-metrics):
    score = 0.25 * economy_activity
          + 0.20 * mission_completeness
          + 0.20 * combat_completeness
          + 0.15 * event_coverage
          + 0.15 * system_state_completeness
          + 0.05 * repair_activity

Each sub-metric is normalized to [0.0, 1.0] before weighting.
Degenerate: economy_activity < 0.3 → score = 0.0.
"""

import json
import sys
from collections import Counter
from pathlib import Path


def load_events(path: str) -> list[dict]:
    events = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if line:
                events.append(json.loads(line))
    return events


# ---------------------------------------------------------------------------
# Sub-metrics
# ---------------------------------------------------------------------------

# Economy event types that the original game produces every tick
ECONOMY_EVENT_TYPES = {
    "economy_tick",
    "support_drift",
    "collection_rate",
    "garrison_required",
    "control_changed",
}

# Mission kinds that should appear in a full-feature campaign
MISSION_KINDS = {
    "Diplomacy", "Recruitment", "Espionage", "Sabotage",
    "InciteUprising", "Assassination", "Abduction", "Rescue",
}

# Character-targeted mission kinds — parity requires target_character to be set
TARGETED_MISSION_KINDS = {"Assassination", "Abduction", "Recruitment"}

# System tags that should emit at least one event in a full campaign
EXPECTED_SYSTEMS = {
    "manufacturing", "movement", "combat", "fog", "missions",
    "events", "ai", "blockade", "uprising", "death_star",
    "research", "jedi", "victory", "betrayal", "story",
    "economy", "repair",
}


def economy_activity(events: list[dict]) -> float:
    """Fraction of expected economy event types that actually appear."""
    economy_events = [e for e in events if e.get("system") == "economy"]
    if not economy_events:
        return 0.0

    observed_types = set()
    for e in economy_events:
        observed_types.add(e.get("event_type", ""))

    return min(1.0, len(observed_types & ECONOMY_EVENT_TYPES) / max(1, len(ECONOMY_EVENT_TYPES)))


def mission_completeness(events: list[dict]) -> float:
    """Measures mission kind diversity from resolved missions.

    Score = fraction of expected mission kinds that appear in resolved missions.
    """
    resolved_kinds = set()

    for e in events:
        if e.get("event_type") == "mission_resolved":
            details = e.get("details", {})
            kind = details.get("kind", "")
            resolved_kinds.add(kind)

    return len(resolved_kinds & MISSION_KINDS) / max(1, len(MISSION_KINDS))


def combat_completeness(events: list[dict]) -> float:
    """Checks that combat events cover space, ground, and bombardment.

    3 combat types, each worth 1/3:
    - combat_space events
    - combat_ground events
    - bombardment events
    """
    has_space = False
    has_ground = False
    has_bombardment = False

    for e in events:
        et = e.get("event_type", "")
        if et == "combat_space":
            has_space = True
        elif et == "combat_ground":
            has_ground = True
        elif et == "bombardment":
            has_bombardment = True

    return (int(has_space) + int(has_ground) + int(has_bombardment)) / 3.0


def event_coverage(events: list[dict]) -> float:
    """Fraction of the 17 simulation system tags that emit at least one event."""
    observed_tags = set(e.get("system", "") for e in events)
    return len(observed_tags & EXPECTED_SYSTEMS) / len(EXPECTED_SYSTEMS)


def system_state_completeness(events: list[dict]) -> float:
    """Checks campaign snapshots for economy-enriched fields.

    5 fields expected in snapshot details: production_modifier, troop_surplus,
    has_shipyard, fleet_posture, collection_rate.
    """
    expected_fields = {"production_modifier", "troop_surplus", "has_shipyard",
                       "fleet_posture", "collection_rate"}
    observed = set()

    for e in events:
        if e.get("event_type") == "campaign_snapshot":
            details = e.get("details", {})
            for field in expected_fields:
                if field in details:
                    observed.add(field)
            # Also check nested system data
            for sys_data in details.get("systems", {}).values() if isinstance(details.get("systems"), dict) else []:
                for field in expected_fields:
                    if field in sys_data:
                        observed.add(field)

    return len(observed & expected_fields) / len(expected_fields)


def repair_activity(events: list[dict]) -> float:
    """1.0 if any ship_repaired events exist, 0.0 otherwise."""
    for e in events:
        if e.get("event_type") in ("ship_repaired", "ship_repair_started"):
            return 1.0
    return 0.0


# ---------------------------------------------------------------------------
# Composite
# ---------------------------------------------------------------------------

def evaluate(events: list[dict]) -> dict:
    """Compute parity metrics from a list of JSONL event dicts."""
    if not events:
        return {"score": 0.0, "degenerate": True, "reason": "no events"}

    metrics = {
        "economy_activity": economy_activity(events),
        "mission_completeness": mission_completeness(events),
        "combat_completeness": combat_completeness(events),
        "event_coverage": event_coverage(events),
        "system_state_completeness": system_state_completeness(events),
        "repair_activity": repair_activity(events),
    }

    # Degenerate detection
    if metrics["economy_activity"] < 0.3:
        return {
            "score": 0.0,
            "degenerate": True,
            "reason": f"economy_activity too low ({metrics['economy_activity']:.2f})",
            **metrics,
        }

    weights = {
        "economy_activity": 0.25,
        "mission_completeness": 0.20,
        "combat_completeness": 0.20,
        "event_coverage": 0.15,
        "system_state_completeness": 0.15,
        "repair_activity": 0.05,
    }

    score = sum(weights[k] * metrics[k] for k in weights)

    return {
        "score": round(score, 4),
        "degenerate": False,
        **{k: round(v, 4) for k, v in metrics.items()},
        "total_events": len(events),
        "final_tick": max(e.get("tick", 0) for e in events),
    }


def main():
    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <campaign.jsonl> [--json]", file=sys.stderr)
        sys.exit(1)

    path = sys.argv[1]
    use_json = "--json" in sys.argv

    events = load_events(path)
    result = evaluate(events)

    if use_json:
        print(json.dumps(result, indent=2))
    else:
        print(f"Parity Score: {result['score']:.4f}")
        if result.get("degenerate"):
            print(f"  DEGENERATE: {result['reason']}")
        for key in ("economy_activity", "mission_completeness", "combat_completeness",
                     "event_coverage", "system_state_completeness", "repair_activity"):
            if key in result:
                print(f"  {key}: {result[key]:.4f}")
        if "total_events" in result:
            print(f"  total_events: {result['total_events']}")
            print(f"  final_tick: {result['final_tick']}")


if __name__ == "__main__":
    main()
