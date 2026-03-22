#!/usr/bin/env python3
"""
Evaluate a headless playtest JSONL log for game quality.

Reads a JSONL event log produced by `rebellion-playtest --output` and computes
a composite quality score in [0.0, 1.0]. Used by autoresearch_loop.py for the
Karpathy self-improvement loop: edit config → run headless → measure → keep/discard.

Usage:
    python3 scripts/eval_game_quality.py campaign.jsonl
    python3 scripts/eval_game_quality.py campaign.jsonl --json

Composite score formula (8 sub-metrics):
    score = 0.15 * event_diversity
          + 0.15 * combat_quality      (ratio + spread, NOT just count)
          + 0.15 * victory_timing
          + 0.10 * faction_balance
          + 0.15 * manufacturing_activity
          + 0.10 * control_dynamism    (how much the map changes)
          + 0.10 * mission_diversity
          + 0.10 * fleet_engagement    (attack ratio, not just movement)

Each sub-metric is normalized to [0.0, 1.0] before weighting.
A degenerate game is scored 0.0 regardless of sub-metrics.
"""

import json
import math
import sys
from collections import Counter, defaultdict
from pathlib import Path


def load_events(path: str) -> list[dict]:
    events = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if line:
                events.append(json.loads(line))
    return events


def bell_curve(value: float, center: float, sigma: float) -> float:
    """Gaussian bell curve normalized to peak at 1.0."""
    if sigma == 0:
        return 1.0 if value == center else 0.0
    return math.exp(-0.5 * ((value - center) / sigma) ** 2)


def evaluate(events: list[dict]) -> dict:
    """Compute quality metrics from a list of JSONL event dicts."""
    if not events:
        return {"score": 0.0, "degenerate": True, "reason": "no events"}

    # Count by event type
    type_counts = Counter(e["event_type"] for e in events)
    total = len(events)
    final_tick = max(e["tick"] for e in events)

    # ── Extract structured data from events ──────────────────────────

    # Combat locations (for concentration analysis)
    combat_locations = Counter()
    combat_events = 0
    for e in events:
        if e["event_type"] in ("combat_space", "combat_ground", "bombardment"):
            combat_events += 1
            system = e.get("details", {}).get("system", "unknown")
            combat_locations[system] += 1

    # Faction activity (use explicit faction field, not dual_ai flag)
    faction_actions = Counter()
    for e in events:
        if e["event_type"] == "ai_action":
            d = e.get("details", {})
            faction = d.get("faction", "Empire" if not d.get("dual_ai") else "Alliance")
            faction_actions[faction] += 1

    # Fleet movement patterns
    attack_orders = 0
    reinforce_orders = 0
    attack_targets = Counter()
    for e in events:
        if e["event_type"] == "ai_action":
            d = e.get("details", {})
            if d.get("type") == "MoveFleet":
                if d.get("reason") == "Attack":
                    attack_orders += 1
                    attack_targets[d.get("to", "?")] += 1
                elif d.get("reason") == "Reinforce":
                    reinforce_orders += 1

    # Mission kinds (for diversity)
    mission_kinds = Counter()
    for e in events:
        if e["event_type"] == "mission_resolved":
            kind = e.get("details", {}).get("kind", "unknown")
            mission_kinds[kind] += 1

    # Manufacturing
    mfg_count = type_counts.get("manufacturing_complete", 0)
    mfg_kinds = Counter()
    for e in events:
        if e["event_type"] == "manufacturing_complete":
            kind = e.get("details", {}).get("kind", "unknown")
            mfg_kinds[kind] += 1

    # Control changes (dedicated control_changed event + uprising_began fallback)
    control_changes = type_counts.get("control_changed", 0) + type_counts.get("uprising_began", 0)

    # Fleet arrivals
    fleet_arrived = type_counts.get("fleet_arrived", 0)

    # Victory
    victory_tick = None
    for e in events:
        if e["event_type"] == "victory_check":
            victory_tick = e["tick"]
            break

    # ── Sub-metrics ──────────────────────────────────────────────────

    # 1. Event diversity: distinct_types / 12 target
    distinct_types = len(type_counts)
    event_diversity = min(distinct_types / 12.0, 1.0)

    # 2. Combat quality: ratio × spread
    # Ratio: bell curve centered at 10% with sigma 5%
    combat_pct = combat_events / total if total > 0 else 0.0
    combat_ratio_score = bell_curve(combat_pct, 0.10, 0.05)

    # Spread: penalize concentration. If all battles at 1 system, spread=0.
    # If evenly distributed across N systems, spread approaches 1.
    if combat_events > 0 and len(combat_locations) > 0:
        top_location_pct = combat_locations.most_common(1)[0][1] / combat_events
        # Ideal: no single system has > 40% of battles
        combat_spread = max(0.0, 1.0 - (top_location_pct - 0.4) / 0.6) if top_location_pct > 0.4 else 1.0
        combat_quality = combat_ratio_score * 0.6 + combat_spread * 0.4
    else:
        combat_spread = 0.0
        combat_quality = 0.0

    # 3. Victory timing: bell curve centered at 3000, sigma 1000
    effective_victory = victory_tick if victory_tick else final_tick
    victory_timing = bell_curve(effective_victory, 3000, 1000)

    # 4. Faction balance: ratio of actions (using explicit faction field)
    alliance_acts = faction_actions.get("Alliance", 0)
    empire_acts = faction_actions.get("Empire", 0)
    total_faction = alliance_acts + empire_acts
    if total_faction > 0:
        faction_balance = min(alliance_acts, empire_acts) / max(alliance_acts, empire_acts)
    else:
        faction_balance = 0.0

    # 5. Manufacturing activity: bell curve centered at 50, sigma 30
    mfg_per_5k = mfg_count * (5000.0 / max(final_tick, 1))
    manufacturing_activity = bell_curve(mfg_per_5k, 50, 30)

    # 6. Control dynamism: how much the map changes
    # Target: 5-25 control changes per 5000 ticks
    changes_per_5k = control_changes * (5000.0 / max(final_tick, 1))
    control_dynamism = bell_curve(changes_per_5k, 15, 10)

    # 7. Mission diversity: how many distinct mission types are used
    distinct_missions = len(mission_kinds)
    mission_diversity = min(distinct_missions / 6.0, 1.0)  # 6+ types → 1.0

    # 8. Fleet engagement: attack orders as fraction of total moves
    total_moves = attack_orders + reinforce_orders
    if total_moves > 0:
        attack_ratio = attack_orders / total_moves
        # Target: 25-50% attacks (rest reinforce/garrison)
        fleet_engagement = bell_curve(attack_ratio, 0.35, 0.15)
    else:
        fleet_engagement = 0.0

    # ── Degenerate game detection ────────────────────────────────────
    degenerate = False
    reason = None

    if combat_events == 0:
        degenerate = True
        reason = "no combat events"
    elif distinct_types < 6:
        degenerate = True
        reason = f"only {distinct_types} event types"
    elif mfg_count < 2:
        degenerate = True
        reason = f"only {mfg_count} manufacturing completions"
    elif victory_tick and victory_tick < 800:
        degenerate = True
        reason = f"victory at tick {victory_tick} (too fast)"
    elif combat_events > 0 and len(combat_locations) == 1:
        loc = list(combat_locations.keys())[0]
        degenerate = True
        reason = f"all {combat_events} battles at single system ({loc})"
    elif total_moves > 10 and attack_orders == 0:
        degenerate = True
        reason = "no attack orders despite fleet activity"

    # ── Composite score ──────────────────────────────────────────────
    if degenerate:
        score = 0.0
    else:
        score = (
            0.15 * event_diversity
            + 0.15 * combat_quality
            + 0.15 * victory_timing
            + 0.10 * faction_balance
            + 0.15 * manufacturing_activity
            + 0.10 * control_dynamism
            + 0.10 * mission_diversity
            + 0.10 * fleet_engagement
        )

    return {
        "score": round(score, 4),
        "degenerate": degenerate,
        "reason": reason,
        "final_tick": final_tick,
        "total_events": total,
        "distinct_event_types": distinct_types,
        "combat_events": combat_events,
        "combat_pct": round(combat_pct * 100, 2),
        "combat_locations": len(combat_locations),
        "combat_top_location": combat_locations.most_common(1)[0] if combat_locations else None,
        "combat_spread": round(combat_spread, 4),
        "fleet_arrived": fleet_arrived,
        "manufacturing_complete": mfg_count,
        "manufacturing_kinds": dict(mfg_kinds),
        "attack_orders": attack_orders,
        "reinforce_orders": reinforce_orders,
        "attack_targets": dict(attack_targets.most_common(5)),
        "control_changes": control_changes,
        "mission_kinds": dict(mission_kinds),
        "distinct_mission_kinds": distinct_missions,
        "faction_actions": dict(faction_actions),
        "victory_tick": victory_tick,
        "sub_scores": {
            "event_diversity": round(event_diversity, 4),
            "combat_quality": round(combat_quality, 4),
            "combat_ratio": round(combat_ratio_score, 4),
            "combat_spread": round(combat_spread, 4),
            "victory_timing": round(victory_timing, 4),
            "faction_balance": round(faction_balance, 4),
            "manufacturing_activity": round(manufacturing_activity, 4),
            "control_dynamism": round(control_dynamism, 4),
            "mission_diversity": round(mission_diversity, 4),
            "fleet_engagement": round(fleet_engagement, 4),
        },
    }


def main():
    if len(sys.argv) < 2:
        print("Usage: eval_game_quality.py <jsonl_file> [--json]", file=sys.stderr)
        sys.exit(1)

    path = sys.argv[1]
    json_output = "--json" in sys.argv

    events = load_events(path)
    result = evaluate(events)

    if json_output:
        print(json.dumps(result, indent=2))
    else:
        print(f"Score: {result['score']:.4f}")
        if result["degenerate"]:
            print(f"DEGENERATE: {result['reason']}")
        print(f"Events: {result['total_events']} ({result['distinct_event_types']} types)")
        print(f"Combat: {result['combat_events']} at {result['combat_locations']} systems ({result['combat_pct']}%)")
        if result["combat_top_location"]:
            print(f"  Top combat location: {result['combat_top_location'][0]} ({result['combat_top_location'][1]} battles)")
        print(f"Manufacturing: {result['manufacturing_complete']} ({result['manufacturing_kinds']})")
        print(f"Fleet orders: {result['attack_orders']} attack / {result['reinforce_orders']} reinforce")
        print(f"Control changes: {result['control_changes']}")
        print(f"Mission kinds: {result['distinct_mission_kinds']} ({result['mission_kinds']})")
        print(f"Faction activity: {result['faction_actions']}")
        print(f"Sub-scores:")
        for k, v in result["sub_scores"].items():
            bar = "█" * int(v * 20) + "░" * (20 - int(v * 20))
            print(f"  {k:24s} {bar} {v:.4f}")


if __name__ == "__main__":
    main()
