#!/usr/bin/env python3
"""
Karpathy-style autoresearch loop for Open Rebellion browser (WASM) functionality.

Uses `claude -p` to generate code mutations that fix WASM gaps, then evaluates
with dev-browser via eval_browser.py.

Five invariants:
    1. Single mutable surface (one WASM fix per iteration)
    2. Fixed eval budget (build + 5 browser checks)
    3. One scalar metric (browser score from eval_browser.py)
    4. Binary keep/discard
    5. Git-as-memory (each keep is a commit)

Usage:
    python3 scripts/autoresearch_browser_loop.py --iterations 10
    python3 scripts/autoresearch_browser_loop.py --iterations 5 --dry-run
"""

import argparse
import json
import os
import subprocess
import sys
import time
from pathlib import Path

SCRIPT_DIR = Path(__file__).parent
PROJECT_DIR = SCRIPT_DIR.parent
EVAL_SCRIPT = SCRIPT_DIR / "eval_browser.py"
PROGRAM_MD = PROJECT_DIR / "autoresearch" / "browser" / "program.md"
RESULTS_TSV = PROJECT_DIR / "autoresearch" / "browser" / "results.tsv"

IMPROVEMENT_THRESHOLD = 0.05  # browser checks are coarser (pass/fail)


def evaluate_browser(skip_build: bool = False) -> dict:
    """Run eval_browser.py and return the result dict."""
    cmd = [sys.executable, str(EVAL_SCRIPT), "--json"]
    if skip_build:
        cmd.append("--skip-build")
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=600)
    if result.returncode != 0:
        return {"score": 0.0, "degenerate": True, "reason": f"eval failed: {result.stderr[-200:]}"}
    try:
        return json.loads(result.stdout)
    except json.JSONDecodeError:
        return {"score": 0.0, "degenerate": True, "reason": f"bad output: {result.stdout[-200:]}"}


def build_and_test() -> bool:
    """Run cargo test + WASM build. Returns True if both pass."""
    env = dict(os.environ)
    env["PATH"] = f"/usr/bin:{env.get('PATH', '')}"

    # Native tests
    test_result = subprocess.run(
        ["cargo", "test", "-p", "rebellion-core", "-p", "rebellion-data"],
        capture_output=True, text=True, env=env, timeout=120,
    )
    if test_result.returncode != 0:
        print(f"  TESTS FAILED")
        return False

    # WASM build
    build_result = subprocess.run(
        ["bash", str(PROJECT_DIR / "scripts" / "build-wasm.sh")],
        capture_output=True, text=True, env=env, timeout=300,
        cwd=str(PROJECT_DIR),
    )
    if build_result.returncode != 0:
        print(f"  WASM BUILD FAILED")
        return False

    return True


def run_claude_mutation(program_md: Path, score: float, iteration: int) -> bool:
    """Use claude -p to generate a code mutation."""
    prompt = f"""Read {program_md} for context.

Current browser score: {score:.4f} (iteration {iteration}).

Implement exactly ONE fix from the priority list in program.md.
Pick the highest-priority item that hasn't been done yet.
Make the smallest change that moves the score upward.

After making the change, verify with:
  PATH="/usr/bin:$PATH" cargo test -p rebellion-core -p rebellion-data
  bash scripts/build-wasm.sh

Changes must work on BOTH native and WASM. Use #[cfg(target_arch = "wasm32")] for browser-only paths.
Do NOT make more than one logical change."""

    env = dict(os.environ)
    env.pop("ANTHROPIC_API_KEY", None)
    env["PATH"] = f"/usr/bin:{env.get('PATH', '')}"

    result = subprocess.run(
        ["claude", "-p", prompt, "--dangerously-skip-permissions", "--max-turns", "20"],
        capture_output=True, text=True, env=env, timeout=600,
        cwd=str(PROJECT_DIR),
    )
    return result.returncode == 0


def git_commit(message: str):
    subprocess.run(["git", "add", "-u"], cwd=str(PROJECT_DIR), capture_output=True)
    subprocess.run(["git", "commit", "-m", message], cwd=str(PROJECT_DIR), capture_output=True)


def git_discard():
    """Discard uncommitted changes to tracked files. Only cleans new files in crates/tools/."""
    subprocess.run(["git", "checkout", "--", "."], cwd=str(PROJECT_DIR), capture_output=True)
    subprocess.run(["git", "clean", "-fd", "--", "crates/", "tools/"], cwd=str(PROJECT_DIR), capture_output=True)


def append_tsv(path: Path, row: dict):
    header = "iteration\tscore\tdelta\taccepted\tmutation_time\teval_time\n"
    if not path.exists():
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(header)
    with open(path, "a") as f:
        f.write(f"{row['iteration']}\t{row['score']:.4f}\t{row['delta']:+.4f}\t"
                f"{'KEEP' if row['accepted'] else 'DISCARD'}\t"
                f"{row['mutation_time']:.1f}s\t{row['eval_time']:.1f}s\n")


def main():
    parser = argparse.ArgumentParser(description="Autoresearch browser loop (WASM fixes via claude -p)")
    parser.add_argument("--iterations", type=int, default=10)
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()

    # Safety: refuse to start with dirty working tree
    dirty = subprocess.run(
        ["git", "status", "--porcelain"], cwd=str(PROJECT_DIR),
        capture_output=True, text=True,
    ).stdout.strip()
    if dirty and not args.dry_run:
        print("ERROR: Working tree is dirty. Commit or stash changes first.", file=sys.stderr)
        print(dirty[:500], file=sys.stderr)
        sys.exit(1)

    print(f"{'='*60}")
    print(f"AUTORESEARCH BROWSER LOOP")
    print(f"{'='*60}")
    print(f"Iterations: {args.iterations}")
    print(f"Threshold: {IMPROVEMENT_THRESHOLD}")
    print()

    # Baseline
    print("Evaluating baseline (includes WASM build)...")
    baseline = evaluate_browser(skip_build=False)
    baseline_score = baseline["score"]
    print(f"Baseline: {baseline_score:.4f}")
    for key in ("wasm_build", "wasm_size", "dat_loading", "ui_rendering", "save_load"):
        val = baseline.get(key, 0.0)
        icon = "pass" if val == 1.0 else "FAIL"
        detail = baseline.get("details", {}).get(key, "")
        print(f"  {key}: {icon} — {detail}")
    print()

    if args.dry_run:
        print("DRY RUN — would run:")
        for i in range(1, args.iterations + 1):
            print(f"  Iteration {i}: claude -p mutation → build → wasm build → eval → keep/discard")
        return

    incumbent_score = baseline_score
    improvements = 0

    for iteration in range(1, args.iterations + 1):
        print(f"{'─'*60}")
        print(f"Iteration {iteration}/{args.iterations} (incumbent: {incumbent_score:.4f})")

        # Mutate
        t0 = time.time()
        success = run_claude_mutation(PROGRAM_MD, incumbent_score, iteration)
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

        # Build + test (native + WASM)
        if not build_and_test():
            print(f"  Build/test failed — DISCARD")
            git_discard()
            append_tsv(RESULTS_TSV, {
                "iteration": iteration, "score": incumbent_score,
                "delta": 0.0, "accepted": False,
                "mutation_time": mutation_time, "eval_time": 0.0,
            })
            continue

        # Evaluate in browser
        t1 = time.time()
        candidate = evaluate_browser(skip_build=True)  # already built
        eval_time = time.time() - t1
        candidate_score = candidate["score"]

        delta = candidate_score - incumbent_score
        accepted = delta >= IMPROVEMENT_THRESHOLD

        icon = "✓" if accepted else "✗"
        print(f"  {icon} Score: {candidate_score:.4f} (Δ={delta:+.4f}) "
              f"[mutation: {mutation_time:.1f}s, eval: {eval_time:.1f}s]")

        if accepted:
            incumbent_score = candidate_score
            improvements += 1
            git_commit(f"autoresearch(browser): iteration {iteration} — score {candidate_score:.4f} (Δ={delta:+.4f})")
            print(f"    KEEP — new incumbent: {incumbent_score:.4f}")
        else:
            git_discard()
            print(f"    DISCARD")

        append_tsv(RESULTS_TSV, {
            "iteration": iteration, "score": candidate_score,
            "delta": delta, "accepted": accepted,
            "mutation_time": mutation_time, "eval_time": eval_time,
        })

    print(f"\n{'='*60}")
    print(f"COMPLETE: {improvements}/{args.iterations} improvements")
    print(f"Score: {baseline_score:.4f} → {incumbent_score:.4f} ({incumbent_score - baseline_score:+.4f})")
    print(f"Results: {RESULTS_TSV}")


if __name__ == "__main__":
    main()
