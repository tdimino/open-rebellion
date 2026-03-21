#!/usr/bin/env python3
"""
Evaluate a headless playtest JSONL log for game quality.

Reads a JSONL event log produced by `rebellion-playtest --output` and computes
a composite quality score in [0.0, 1.0]. Used by autoresearch_ai.py for the
Karpathy self-improvement loop: edit config → run headless → measure → keep/discard.

Usage:
    python3 scripts/eval_game_quality.py campaign.jsonl
    python3 scripts/eval_game_quality.py campaign.jsonl --json

Composite score formula:
    score = 0.25 * event_diversity
          + 0.25 * combat_ratio
          + 0.20 * victory_timing
          + 0.15 * faction_balance
          + 0.15 * manufacturing_activity

Each sub-metric is normalized to [0.0, 1.0] before weighting.
A degenerate game (0 combat, blowout, etc.) is scored 0.0 regardless.
"""

import json
import math
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


def bell_curve(value: float, center: float, sigma: float) -> float:
    """Gaussian bell curve normalized to peak at 1.0."""
    return math.exp(-0.5 * ((value - center) / sigma) ** 2)


def evaluate(events: list[dict]) -> dict:
    """Compute quality metrics from a list of JSONL event dicts."""
    if not events:
        return {"score": 0.0, "degenerate": True, "reason": "no events"}

    # Count by event type
    type_counts = Counter(e["event_type"] for e in events)
    total = len(events)
    final_tick = max(e["tick"] for e in events)

    # --- Sub-metrics ---

    # 1. Event diversity: distinct_types / 24 possible types
    distinct_types = len(type_counts)
    event_diversity = min(distinct_types / 12.0, 1.0)  # 12+ types → 1.0

    # 2. Combat ratio: combat events / total events, target 5-15%
    combat_events = (
        type_counts.get("combat_space", 0)
        + type_counts.get("combat_ground", 0)
        + type_counts.get("bombardment", 0)
    )
    combat_pct = combat_events / total if total > 0 else 0.0
    # Bell curve centered at 10% with sigma 5%
    combat_ratio = bell_curve(combat_pct, 0.10, 0.05)

    # 3. Victory timing: bell curve centered at tick 3000, sigma 1000
    # No victory → use final_tick (penalizes games that stagnate)
    victory_tick = final_tick
    for e in events:
        if e["event_type"] == "victory_check":
            victory_tick = e["tick"]
            break
    victory_timing = bell_curve(victory_tick, 3000, 1000)

    # 4. Faction balance: ratio of AI actions by faction
    alliance_actions = 0
    empire_actions = 0
    for e in events:
        if e["event_type"] == "ai_action":
            details = e.get("details", {})
            if details.get("dual_ai"):
                alliance_actions += 1
            else:
                empire_actions += 1
    total_ai = alliance_actions + empire_actions
    if total_ai > 0:
        balance_ratio = min(alliance_actions, empire_actions) / max(alliance_actions, empire_actions)
    else:
        balance_ratio = 0.0
    faction_balance = balance_ratio  # 1.0 = perfectly balanced

    # 5. Manufacturing activity: target 20-80 completions in 5000 ticks
    mfg_count = type_counts.get("manufacturing_complete", 0)
    # Scale to tick count
    mfg_per_5k = mfg_count * (5000.0 / max(final_tick, 1))
    manufacturing_activity = bell_curve(mfg_per_5k, 50, 30)

    # 6. Fleet movement diversity
    fleet_arrived = type_counts.get("fleet_arrived", 0)
    fleet_ratio = min(fleet_arrived / max(total * 0.05, 1), 1.0)

    # --- Degenerate game detection ---
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
    elif victory_tick < 800:
        degenerate = True
        reason = f"victory at tick {victory_tick} (too fast)"

    # Attack vs reinforce balance
    attack_orders = 0
    reinforce_orders = 0
    for e in events:
        if e["event_type"] == "ai_action":
            d = e.get("details", {})
            if d.get("type") == "MoveFleet":
                if d.get("reason") == "Attack":
                    attack_orders += 1
                elif d.get("reason") == "Reinforce":
                    reinforce_orders += 1
    total_moves = attack_orders + reinforce_orders
    if total_moves > 0 and attack_orders == 0 and combat_events == 0:
        degenerate = True
        reason = "no attack orders and no combat"

    # --- Composite score ---
    if degenerate:
        score = 0.0
    else:
        score = (
            0.25 * event_diversity
            + 0.25 * combat_ratio
            + 0.20 * victory_timing
            + 0.15 * faction_balance
            + 0.15 * manufacturing_activity
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
        "fleet_arrived": fleet_arrived,
        "manufacturing_complete": mfg_count,
        "attack_orders": attack_orders,
        "reinforce_orders": reinforce_orders,
        "victory_tick": victory_tick if victory_tick != final_tick else None,
        "sub_scores": {
            "event_diversity": round(event_diversity, 4),
            "combat_ratio": round(combat_ratio, 4),
            "victory_timing": round(victory_timing, 4),
            "faction_balance": round(faction_balance, 4),
            "manufacturing_activity": round(manufacturing_activity, 4),
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
        print(f"Combat: {result['combat_events']} ({result['combat_pct']}%)")
        print(f"Fleet arrivals: {result['fleet_arrived']}")
        print(f"Manufacturing: {result['manufacturing_complete']}")
        print(f"Attack/Reinforce: {result['attack_orders']}/{result['reinforce_orders']}")
        print(f"Sub-scores: {json.dumps(result['sub_scores'])}")


if __name__ == "__main__":
    main()
