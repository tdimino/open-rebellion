#!/usr/bin/env python3
"""
Karpathy-style autoresearch loop for Open Rebellion simulation parity.

Unlike autoresearch_loop.py (which mutates config parameters), this loop uses
`claude -p` to generate code mutations that implement missing game mechanics.

Five invariants:
    1. Single mutable surface (one code change per iteration)
    2. Fixed eval budget (3 seeds × 5000 ticks)
    3. One scalar metric (parity score from eval_parity.py)
    4. Binary keep/discard
    5. Git-as-memory (each keep is a commit)

Usage:
    # Build release binary first
    PATH="/usr/bin:$PATH" cargo build -p rebellion-playtest --release

    # Run 10 iterations
    python3 scripts/autoresearch_parity_loop.py --iterations 10

    # Dry run (show what would happen without running claude -p)
    python3 scripts/autoresearch_parity_loop.py --iterations 5 --dry-run
"""

import argparse
import json
import os
import statistics
import subprocess
import sys
import tempfile
import time
from pathlib import Path

SCRIPT_DIR = Path(__file__).parent
PROJECT_DIR = SCRIPT_DIR.parent
EVAL_SCRIPT = SCRIPT_DIR / "eval_parity.py"
PROGRAM_MD = PROJECT_DIR / "autoresearch" / "parity" / "program.md"
DEFAULT_BINARY = PROJECT_DIR / "target" / "release" / "rebellion-playtest"
DATA_DIR = PROJECT_DIR / "data" / "base"
RESULTS_TSV = PROJECT_DIR / "autoresearch" / "parity" / "results.tsv"

IMPROVEMENT_THRESHOLD = 0.005
DEFAULT_SEEDS = [42, 123, 999]
DEFAULT_TICKS = 5000


def run_campaign(binary: Path, seed: int, ticks: int) -> Path:
    """Run a headless campaign and return the JSONL output path."""
    output = Path(tempfile.gettempdir()) / f"parity-seed{seed}-{os.getpid()}.jsonl"
    # Remove stale file to prevent evaluating old data on binary failure
    if output.exists():
        output.unlink()
    cmd = [
        str(binary), str(DATA_DIR),
        "--seed", str(seed),
        "--ticks", str(ticks),
        "--dual-ai",
        "--output", str(output),
    ]
    env = dict(os.environ)
    env["PATH"] = f"/usr/bin:{env.get('PATH', '')}"
    result = subprocess.run(cmd, capture_output=True, text=True, env=env, timeout=120)
    if result.returncode != 0:
        print(f"  WARN: seed {seed} binary failed (exit {result.returncode})", file=sys.stderr)
    return output


def evaluate(jsonl_path: Path) -> dict:
    """Run eval_parity.py and return the result dict."""
    if not jsonl_path.exists():
        return {"score": 0.0, "degenerate": True, "reason": "jsonl file missing (binary failed?)"}
    result = subprocess.run(
        [sys.executable, str(EVAL_SCRIPT), str(jsonl_path), "--json"],
        capture_output=True, text=True, timeout=30,
    )
    if result.returncode != 0:
        return {"score": 0.0, "degenerate": True, "reason": f"eval failed: {result.stderr[:200]}"}
    try:
        return json.loads(result.stdout)
    except json.JSONDecodeError:
        return {"score": 0.0, "degenerate": True, "reason": f"bad output: {result.stdout[:200]}"}


def aggregate(results: list[dict]) -> float:
    """Aggregate per-seed scores: median - 0.5 * stddev."""
    scores = [r["score"] for r in results]
    if not scores:
        return 0.0
    degenerate_count = sum(1 for r in results if r.get("degenerate"))
    if degenerate_count > len(scores) * 0.25:
        return 0.0
    median = statistics.median(scores)
    stddev = statistics.stdev(scores) if len(scores) > 1 else 0.0
    return round(median - 0.5 * stddev, 4)


def eval_current(binary: Path, seeds: list[int], ticks: int) -> tuple[float, list[dict]]:
    """Evaluate the current codebase across all seeds."""
    results = []
    for seed in seeds:
        jsonl = run_campaign(binary, seed, ticks)
        result = evaluate(jsonl)
        result["seed"] = seed
        results.append(result)
    score = aggregate(results)
    return score, results


def build_and_test() -> bool:
    """Build release binary and run tests. Returns True if both pass."""
    env = dict(os.environ)
    env["PATH"] = f"/usr/bin:{env.get('PATH', '')}"

    try:
        # Tests first (faster feedback)
        test_result = subprocess.run(
            ["cargo", "test", "-p", "rebellion-core", "-p", "rebellion-data"],
            capture_output=True, text=True, env=env, timeout=180,
        )
        if test_result.returncode != 0:
            print(f"  TESTS FAILED: {test_result.stderr[-200:]}")
            return False

        # Build release (LTO can take a while)
        build_result = subprocess.run(
            ["cargo", "build", "-p", "rebellion-playtest", "--release"],
            capture_output=True, text=True, env=env, timeout=300,
        )
        if build_result.returncode != 0:
            print(f"  BUILD FAILED: {build_result.stderr[-200:]}")
            return False
    except subprocess.TimeoutExpired:
        print("  BUILD/TEST TIMEOUT")
        return False

    return True


def get_live_sub_metrics(binary: Path, seed: int, ticks: int) -> str:
    """Run one eval and format sub-metrics for prompt injection."""
    jsonl = run_campaign(binary, seed, ticks)
    result = evaluate(jsonl)
    lines = []
    for key in ("economy_activity", "mission_completeness", "combat_completeness",
                "event_coverage", "system_state_completeness", "repair_activity"):
        val = result.get(key, 0.0)
        status = "DONE" if val >= 0.99 else f"{val:.3f}"
        lines.append(f"  {key}: {status}")
    return "\n".join(lines)


def run_claude_mutation(program_md: Path, score: float, iteration: int, sub_metrics: str) -> bool:
    """Use claude -p to generate a code mutation. Returns True if claude succeeds."""
    program_content = program_md.read_text()
    prompt = f"""You are an autoresearch agent improving simulation parity for Open Rebellion.

<program>
{program_content}
</program>

Current parity score: {score:.4f} (iteration {iteration}).
Live sub-metric breakdown:
{sub_metrics}

Implement exactly ONE improvement from the priority list above.
Pick the highest-priority item whose sub-metric is still below 1.0.
Make the smallest change that moves the eval score upward.

After making the change:
1. Run: PATH="/usr/bin:$PATH" cargo test -p rebellion-core -p rebellion-data
2. Build: PATH="/usr/bin:$PATH" cargo build -p rebellion-playtest --release
3. Eval: ./target/release/rebellion-playtest data/base --seed 42 --ticks 5000 --dual-ai --output /tmp/parity-check.jsonl && python3 scripts/eval_parity.py /tmp/parity-check.jsonl --json
4. Confirm the targeted sub-metric improved. If not, debug before finishing.

FORBIDDEN — do NOT modify: scripts/eval_parity.py, scripts/autoresearch_parity_loop.py, crates/rebellion-core/src/tuning.rs, configs/.
Do NOT create new files or modules. Do NOT make more than one logical change."""

    env = dict(os.environ)
    # Use subscription, not API key
    env.pop("ANTHROPIC_API_KEY", None)
    env["PATH"] = f"/usr/bin:{env.get('PATH', '')}"

    result = subprocess.run(
        ["claude", "-p", prompt, "--dangerously-skip-permissions", "--max-turns", "20"],
        capture_output=True, text=True, env=env, timeout=600,
        cwd=str(PROJECT_DIR),
    )
    return result.returncode == 0


def git_commit(message: str):
    """Create a git commit with all changes."""
    subprocess.run(["git", "add", "-u"], cwd=str(PROJECT_DIR), capture_output=True)
    subprocess.run(["git", "add", "crates/", "tools/"], cwd=str(PROJECT_DIR), capture_output=True)
    subprocess.run(
        ["git", "commit", "-m", message],
        cwd=str(PROJECT_DIR), capture_output=True, text=True,
    )


def git_discard():
    """Discard uncommitted changes to tracked files. Only cleans new files in crates/tools/."""
    subprocess.run(["git", "checkout", "--", "."], cwd=str(PROJECT_DIR), capture_output=True)
    subprocess.run(["git", "clean", "-fd", "--", "crates/", "tools/"], cwd=str(PROJECT_DIR), capture_output=True)


def append_tsv(path: Path, row: dict):
    """Append a row to the results TSV file."""
    path.parent.mkdir(parents=True, exist_ok=True)
    header = "iteration\tscore\tdelta\taccepted\tmutation_time\teval_time\n"
    if not path.exists():
        path.write_text(header)
    with open(path, "a") as f:
        f.write(f"{row['iteration']}\t{row['score']:.4f}\t{row['delta']:+.4f}\t"
                f"{'KEEP' if row['accepted'] else 'DISCARD'}\t"
                f"{row['mutation_time']:.1f}s\t{row['eval_time']:.1f}s\n")


def main():
    parser = argparse.ArgumentParser(description="Autoresearch parity loop (code mutations via claude -p)")
    parser.add_argument("--iterations", type=int, default=10)
    parser.add_argument("--seeds", type=int, nargs="+", default=DEFAULT_SEEDS)
    parser.add_argument("--ticks", type=int, default=DEFAULT_TICKS)
    parser.add_argument("--dry-run", action="store_true", help="Show plan without running")
    args = parser.parse_args()

    binary = DEFAULT_BINARY
    if not binary.exists():
        print(f"Binary not found: {binary}", file=sys.stderr)
        print("Build with: PATH=\"/usr/bin:$PATH\" cargo build -p rebellion-playtest --release")
        sys.exit(1)

    if not PROGRAM_MD.exists():
        print(f"Program not found: {PROGRAM_MD}", file=sys.stderr)
        sys.exit(1)

    # Safety: refuse to start with dirty working tree
    dirty = subprocess.run(
        ["git", "status", "--porcelain"], cwd=str(PROJECT_DIR),
        capture_output=True, text=True,
    ).stdout.strip()
    if dirty and not args.dry_run:
        print("ERROR: Working tree is dirty. Commit or stash changes first.", file=sys.stderr)
        print(dirty[:500], file=sys.stderr)
        sys.exit(1)

    # Baseline
    print(f"{'='*60}")
    print(f"AUTORESEARCH PARITY LOOP")
    print(f"{'='*60}")
    print(f"Seeds: {args.seeds}")
    print(f"Ticks: {args.ticks}")
    print(f"Iterations: {args.iterations}")
    print(f"Threshold: {IMPROVEMENT_THRESHOLD}")
    print()

    print("Evaluating baseline...")
    baseline_score, baseline_results = eval_current(binary, args.seeds, args.ticks)
    print(f"Baseline: {baseline_score:.4f}")
    for r in baseline_results:
        status = "DEGEN" if r.get("degenerate") else f"{r['score']:.4f}"
        print(f"  Seed {r['seed']}: {status}")
    print()

    if args.dry_run:
        print("DRY RUN — would run:")
        for i in range(1, args.iterations + 1):
            print(f"  Iteration {i}: claude -p mutation → build → test → eval → keep/discard")
        return

    incumbent_score = baseline_score
    improvements = 0

    for iteration in range(1, args.iterations + 1):
        print(f"{'─'*60}")
        print(f"Iteration {iteration}/{args.iterations} (incumbent: {incumbent_score:.4f})")

        # Get live sub-metrics for prompt context
        sub_metrics = get_live_sub_metrics(binary, args.seeds[0], args.ticks)
        print(f"  Sub-metrics:\n{sub_metrics}")

        # Mutate via claude -p
        t0 = time.time()
        success = run_claude_mutation(PROGRAM_MD, incumbent_score, iteration, sub_metrics)
        mutation_time = time.time() - t0

        if not success:
            print(f"  claude -p failed ({mutation_time:.1f}s) — DISCARD")
            git_discard()
            append_tsv(RESULTS_TSV, {
                "iteration": iteration, "score": incumbent_score,
                "delta": 0.0, "accepted": False,
                "mutation_time": mutation_time, "eval_time": 0.0,
            })
            continue

        # Build and test
        if not build_and_test():
            print(f"  Build/test failed — DISCARD")
            git_discard()
            append_tsv(RESULTS_TSV, {
                "iteration": iteration, "score": incumbent_score,
                "delta": 0.0, "accepted": False,
                "mutation_time": mutation_time, "eval_time": 0.0,
            })
            continue

        # Evaluate
        t1 = time.time()
        candidate_score, candidate_results = eval_current(binary, args.seeds, args.ticks)
        eval_time = time.time() - t1

        delta = candidate_score - incumbent_score
        accepted = delta >= IMPROVEMENT_THRESHOLD

        icon = "✓" if accepted else "✗"
        print(f"  {icon} Score: {candidate_score:.4f} (Δ={delta:+.4f}) "
              f"[mutation: {mutation_time:.1f}s, eval: {eval_time:.1f}s]")

        if accepted:
            incumbent_score = candidate_score
            improvements += 1
            git_commit(f"autoresearch(parity): iteration {iteration} — score {candidate_score:.4f} (Δ={delta:+.4f})")
            print(f"    KEEP — new incumbent: {incumbent_score:.4f}")
        else:
            git_discard()
            print(f"    DISCARD")

        append_tsv(RESULTS_TSV, {
            "iteration": iteration, "score": candidate_score,
            "delta": delta, "accepted": accepted,
            "mutation_time": mutation_time, "eval_time": eval_time,
        })

    # Final report
    print(f"\n{'='*60}")
    print(f"COMPLETE: {improvements}/{args.iterations} improvements")
    print(f"Score: {baseline_score:.4f} → {incumbent_score:.4f} ({incumbent_score - baseline_score:+.4f})")
    print(f"Results: {RESULTS_TSV}")


if __name__ == "__main__":
    main()
