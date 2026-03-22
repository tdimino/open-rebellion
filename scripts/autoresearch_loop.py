#!/usr/bin/env python3
"""
Karpathy-style autoresearch loop for Open Rebellion game balance.

Iteratively mutates AI/game parameters, runs headless campaigns across
multiple seeds, evaluates quality, and keeps improvements.

Usage:
    # Build release binary first
    PATH="/usr/bin:$PATH" cargo build -p rebellion-playtest --release

    # Run 10 iterations with 3 seeds each
    python3 scripts/autoresearch_loop.py --iterations 10 --seeds 42 123 999

    # Resume from a previous best config
    python3 scripts/autoresearch_loop.py --iterations 20 --config configs/autoresearch/best.json

Architecture:
    1. Load or create base config (JSON with tunable parameters)
    2. For each iteration:
       a. Mutate one parameter randomly within bounds
       b. Run rebellion-playtest for each seed
       c. Evaluate each run with eval_game_quality.py
       d. Aggregate scores: median - 0.5 * stddev
       e. If candidate > incumbent + threshold: keep (promote to best)
       f. Otherwise: discard
    3. Write best config and iteration log
"""

import argparse
import json
import math
import os
import random
import shutil
import statistics
import subprocess
import sys
import tempfile
import time
from pathlib import Path

# Resolve paths relative to this script
SCRIPT_DIR = Path(__file__).parent
PROJECT_DIR = SCRIPT_DIR.parent
EVAL_SCRIPT = SCRIPT_DIR / "eval_game_quality.py"
DEFAULT_BINARY = PROJECT_DIR / "target" / "release" / "rebellion-playtest"
DATA_DIR = PROJECT_DIR / "data" / "base"
CONFIG_DIR = PROJECT_DIR / "configs" / "autoresearch"

# Minimum improvement threshold to accept a candidate
IMPROVEMENT_THRESHOLD = 0.01

# Default tunable parameters with (min, max, step) bounds
DEFAULT_CONFIG = {
    "ai": {
        "tick_interval": {"value": 7, "min": 3, "max": 21, "step": 2},
        "diplomacy_skill_threshold": {"value": 60, "min": 30, "max": 90, "step": 10},
        "espionage_skill_threshold": {"value": 50, "min": 30, "max": 80, "step": 10},
        "max_construction_yards": {"value": 5, "min": 2, "max": 10, "step": 1},
        "max_covert_ops_per_eval": {"value": 3, "min": 1, "max": 8, "step": 1},
    },
    "movement": {
        "distance_scale": {"value": 2, "min": 1, "max": 5, "step": 1},
        "min_transit_ticks": {"value": 10, "min": 5, "max": 25, "step": 5},
    },
    "production": {
        "capship_threshold": {"value": 5, "min": 2, "max": 10, "step": 1},
        "fighter_ratio": {"value": 3, "min": 1, "max": 6, "step": 1},
    },
    "scoring": {
        "victory_center": {"value": 3000, "min": 1500, "max": 5000, "step": 250},
        "combat_pct_center": {"value": 10, "min": 3, "max": 20, "step": 1},
        "mfg_target": {"value": 50, "min": 15, "max": 100, "step": 5},
    },
}


def flatten_config(config: dict) -> dict:
    """Extract just the values from the config (strip min/max/step)."""
    flat = {}
    for section, params in config.items():
        flat[section] = {}
        for key, spec in params.items():
            if isinstance(spec, dict) and "value" in spec:
                flat[section][key] = spec["value"]
            else:
                flat[section][key] = spec
    return flat


def mutate_config(config: dict) -> tuple[dict, str]:
    """Randomly mutate one parameter within bounds. Returns (new_config, mutation_description)."""
    new_config = json.loads(json.dumps(config))  # deep copy

    # Pick a random section and parameter
    sections = list(new_config.keys())
    section = random.choice(sections)
    params = list(new_config[section].keys())
    param = random.choice(params)
    spec = new_config[section][param]

    if not isinstance(spec, dict) or "value" not in spec:
        return new_config, "no mutation (non-tunable param)"

    old_value = spec["value"]
    step = spec.get("step", 1)
    lo = spec.get("min", old_value - step * 5)
    hi = spec.get("max", old_value + step * 5)

    # Random direction: up or down by 1 step
    direction = random.choice([-1, 1])
    new_value = old_value + direction * step

    # Clamp within bounds
    new_value = max(lo, min(hi, new_value))
    if new_value == old_value:
        new_value = old_value - direction * step
        new_value = max(lo, min(hi, new_value))

    spec["value"] = new_value
    desc = f"{section}.{param}: {old_value} → {new_value}"
    return new_config, desc


def run_campaign(binary: Path, data_dir: Path, seed: int, ticks: int, output_path: Path, config_path: Path | None = None) -> float:
    """Run a single headless campaign and return wall time in seconds."""
    cmd = [
        str(binary),
        str(data_dir),
        "--seed", str(seed),
        "--ticks", str(ticks),
        "--dual-ai",
        "--output", str(output_path),
    ]
    if config_path:
        cmd.extend(["--config", str(config_path)])
    env = dict(os.environ)
    env["PATH"] = f"/usr/bin:{env.get('PATH', '')}"

    start = time.time()
    result = subprocess.run(cmd, capture_output=True, text=True, env=env, timeout=120)
    elapsed = time.time() - start

    if result.returncode != 0:
        print(f"  WARN: seed {seed} failed: {result.stderr[:200]}", file=sys.stderr)
        return elapsed

    return elapsed


def evaluate_campaign(jsonl_path: Path) -> dict:
    """Run eval_game_quality.py on a JSONL file and return the result dict."""
    result = subprocess.run(
        [sys.executable, str(EVAL_SCRIPT), str(jsonl_path), "--json"],
        capture_output=True, text=True, timeout=30,
    )
    if result.returncode != 0:
        return {"score": 0.0, "degenerate": True, "reason": f"eval failed: {result.stderr[:200]}"}
    return json.loads(result.stdout)


def aggregate_scores(results: list[dict]) -> dict:
    """Aggregate per-seed scores into a candidate score."""
    scores = [r["score"] for r in results]
    degenerate_count = sum(1 for r in results if r.get("degenerate"))

    if not scores:
        return {"aggregate": 0.0, "mean": 0.0, "median": 0.0, "stddev": 0.0,
                "degenerate_count": 0, "rejected": True, "reject_reason": "no results"}

    mean = statistics.mean(scores)
    median = statistics.median(scores)
    stddev = statistics.stdev(scores) if len(scores) > 1 else 0.0

    # Reject if >25% of seeds are degenerate
    if degenerate_count > len(scores) * 0.25:
        return {"aggregate": 0.0, "mean": mean, "median": median, "stddev": stddev,
                "degenerate_count": degenerate_count, "rejected": True,
                "reject_reason": f"{degenerate_count}/{len(scores)} seeds degenerate"}

    # Score = median - 0.5 * stddev (penalizes variance)
    aggregate = median - 0.5 * stddev

    return {"aggregate": round(aggregate, 4), "mean": round(mean, 4),
            "median": round(median, 4), "stddev": round(stddev, 4),
            "degenerate_count": degenerate_count, "rejected": False}


def main():
    parser = argparse.ArgumentParser(description="Autoresearch loop for Open Rebellion")
    parser.add_argument("--iterations", type=int, default=10, help="Number of iterations")
    parser.add_argument("--seeds", type=int, nargs="+", default=[42, 123, 999], help="RNG seeds")
    parser.add_argument("--ticks", type=int, default=5000, help="Ticks per campaign")
    parser.add_argument("--config", type=str, help="Starting config JSON")
    parser.add_argument("--binary", type=str, default=str(DEFAULT_BINARY), help="Path to playtest binary")
    parser.add_argument("--output-dir", type=str, help="Output directory for iteration logs")
    args = parser.parse_args()

    binary = Path(args.binary)
    if not binary.exists():
        print(f"Binary not found: {binary}", file=sys.stderr)
        print("Build with: PATH=\"/usr/bin:$PATH\" cargo build -p rebellion-playtest --release", file=sys.stderr)
        sys.exit(1)

    # Load or create config
    if args.config and Path(args.config).exists():
        with open(args.config) as f:
            config = json.load(f)
        print(f"Loaded config from {args.config}")
    else:
        config = DEFAULT_CONFIG
        print("Using default config")

    # Set up output directory
    if args.output_dir:
        output_dir = Path(args.output_dir)
    else:
        output_dir = Path(tempfile.mkdtemp(prefix="autoresearch-"))
    output_dir.mkdir(parents=True, exist_ok=True)
    print(f"Output: {output_dir}")

    # ── Baseline evaluation ──────────────────────────────────────────
    print(f"\n{'='*60}")
    print(f"BASELINE: {len(args.seeds)} seeds × {args.ticks} ticks")
    print(f"{'='*60}")

    # Write the flattened config for the binary to consume
    def write_config(cfg: dict, path: Path):
        """Write flattened config (values only) to JSON for rebellion-playtest --config."""
        with open(path, "w") as f:
            json.dump(flatten_config(cfg), f, indent=2)

    baseline_config_path = output_dir / "baseline_config.json"
    write_config(config, baseline_config_path)

    baseline_results = []
    for seed in args.seeds:
        jsonl_path = output_dir / f"baseline-seed{seed}.jsonl"
        elapsed = run_campaign(binary, DATA_DIR, seed, args.ticks, jsonl_path, baseline_config_path)
        result = evaluate_campaign(jsonl_path)
        result["seed"] = seed
        result["elapsed"] = round(elapsed, 2)
        baseline_results.append(result)
        status = "DEGEN" if result.get("degenerate") else f"{result['score']:.4f}"
        print(f"  Seed {seed}: {status} ({elapsed:.1f}s)")

    baseline = aggregate_scores(baseline_results)
    incumbent_score = baseline["aggregate"]
    incumbent_config = json.loads(json.dumps(config))
    print(f"\nBaseline aggregate: {incumbent_score:.4f} (mean={baseline['mean']:.4f}, std={baseline['stddev']:.4f})")

    if baseline["rejected"]:
        print(f"WARNING: Baseline rejected — {baseline['reject_reason']}")

    # ── Iteration loop ───────────────────────────────────────────────
    improvements = 0
    history = []

    for iteration in range(1, args.iterations + 1):
        print(f"\n{'─'*60}")
        print(f"Iteration {iteration}/{args.iterations}")

        # Mutate
        candidate_config, mutation = mutate_config(incumbent_config)
        print(f"  Mutation: {mutation}")

        # Run seeds
        iter_dir = output_dir / f"iter-{iteration:03d}"
        iter_dir.mkdir(exist_ok=True)

        # Write candidate config for the binary
        candidate_config_path = iter_dir / "config.json"
        write_config(candidate_config, candidate_config_path)

        candidate_results = []
        total_elapsed = 0.0
        for seed in args.seeds:
            jsonl_path = iter_dir / f"seed{seed}.jsonl"
            elapsed = run_campaign(binary, DATA_DIR, seed, args.ticks, jsonl_path, candidate_config_path)
            total_elapsed += elapsed
            result = evaluate_campaign(jsonl_path)
            result["seed"] = seed
            candidate_results.append(result)

        candidate = aggregate_scores(candidate_results)
        candidate_score = candidate["aggregate"]

        # Decision
        improvement = candidate_score - incumbent_score
        accepted = not candidate["rejected"] and improvement >= IMPROVEMENT_THRESHOLD

        status_icon = "✓" if accepted else "✗"
        print(f"  {status_icon} Score: {candidate_score:.4f} (Δ={improvement:+.4f}) [{total_elapsed:.1f}s]")

        if candidate["rejected"]:
            print(f"    Rejected: {candidate['reject_reason']}")

        if accepted:
            incumbent_score = candidate_score
            incumbent_config = candidate_config
            improvements += 1
            print(f"    ACCEPTED — new incumbent: {incumbent_score:.4f}")

            # Save best config
            best_path = output_dir / "best.json"
            with open(best_path, "w") as f:
                json.dump(incumbent_config, f, indent=2)

        history.append({
            "iteration": iteration,
            "mutation": mutation,
            "candidate_score": candidate_score,
            "incumbent_score": incumbent_score,
            "improvement": round(improvement, 4),
            "accepted": accepted,
            "rejected": candidate.get("rejected", False),
            "wall_time": round(total_elapsed, 2),
            "per_seed": [{
                "seed": r["seed"],
                "score": r["score"],
                "degenerate": r.get("degenerate", False),
                "combat": r.get("combat_events", 0),
                "mfg": r.get("manufacturing_complete", 0),
            } for r in candidate_results],
        })

    # ── Final report ─────────────────────────────────────────────────
    print(f"\n{'='*60}")
    print(f"AUTORESEARCH COMPLETE")
    print(f"{'='*60}")
    print(f"Iterations:   {args.iterations}")
    print(f"Improvements: {improvements}")
    print(f"Final score:  {incumbent_score:.4f} (baseline: {baseline['aggregate']:.4f})")
    print(f"Output:       {output_dir}")

    # Save history
    history_path = output_dir / "history.json"
    with open(history_path, "w") as f:
        json.dump(history, f, indent=2)
    print(f"History:      {history_path}")

    # Save final config
    final_path = output_dir / "final_config.json"
    with open(final_path, "w") as f:
        json.dump(incumbent_config, f, indent=2)
    print(f"Best config:  {final_path}")

    # Save flattened config (values only) for rebellion-playtest --config
    flat_path = output_dir / "final_game_config.json"
    write_config(incumbent_config, flat_path)
    print(f"Game config:  {flat_path}")

    # Also copy to project configs dir
    CONFIG_DIR.mkdir(parents=True, exist_ok=True)
    shutil.copy(final_path, CONFIG_DIR / "best.json")
    shutil.copy(flat_path, CONFIG_DIR / "best_game_config.json")
    print(f"Copied to:    {CONFIG_DIR / 'best.json'}")
    print(f"              {CONFIG_DIR / 'best_game_config.json'}")


if __name__ == "__main__":
    main()
