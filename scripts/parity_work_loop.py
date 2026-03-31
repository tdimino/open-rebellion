#!/usr/bin/env python3
"""
Sequential parity work loop — implements non-autoresearch tasks via claude -p.

Reads task definitions from a JSON manifest, picks the next unblocked task by
priority, dispatches claude -p to implement it, runs tests, and commits on
success. Tracks progress in a TSV log.

Usage:
    # Dry run — show task order without executing
    python3 scripts/parity_work_loop.py --dry-run

    # Execute all unblocked tasks
    python3 scripts/parity_work_loop.py

    # Execute only the next N tasks
    python3 scripts/parity_work_loop.py --max-tasks 3

    # Skip to a specific task ID
    python3 scripts/parity_work_loop.py --start-at F3

Architecture:
    1. Load task manifest (scripts/parity_tasks.json)
    2. Build dependency graph, find unblocked tasks
    3. Sort by priority (F > A > U > C > WASM > Eval)
    4. For each task:
       a. Print task brief
       b. Run claude -p with task-specific prompt
       c. cargo test
       d. If tests pass: git commit + mark complete
       e. If tests fail: git discard + mark failed
       f. Re-evaluate unblocked set
    5. Write progress log to autoresearch/parity-work/results.tsv
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
MANIFEST = SCRIPT_DIR / "parity_tasks.json"
RESULTS_TSV = PROJECT_DIR / "autoresearch" / "parity-work" / "results.tsv"
PROGRESS_JSON = PROJECT_DIR / "autoresearch" / "parity-work" / "progress.json"

# Priority order for task categories
PRIORITY = {"F": 0, "A": 1, "U": 2, "C": 3, "WASM": 4, "Eval": 5}


def load_manifest() -> list[dict]:
    with open(MANIFEST) as f:
        return json.load(f)


def save_progress(completed: list[str], failed: list[str]):
    PROGRESS_JSON.parent.mkdir(parents=True, exist_ok=True)
    with open(PROGRESS_JSON, "w") as f:
        json.dump({"completed": completed, "failed": failed}, f, indent=2)


def load_progress() -> tuple[list[str], list[str]]:
    if PROGRESS_JSON.exists():
        with open(PROGRESS_JSON) as f:
            data = json.load(f)
            return data.get("completed", []), data.get("failed", [])
    return [], []


def get_unblocked(tasks: list[dict], completed: set[str]) -> list[dict]:
    """Return tasks whose dependencies are all completed."""
    result = []
    for task in tasks:
        if task["id"] in completed:
            continue
        blocked_by = task.get("blocked_by", [])
        if all(dep in completed for dep in blocked_by):
            result.append(task)
    return result


def sort_by_priority(tasks: list[dict]) -> list[dict]:
    """Sort tasks by category priority, then by ID."""
    def key(t):
        cat = t.get("category", "WASM")
        pri = PRIORITY.get(cat, 99)
        return (pri, t["id"])
    return sorted(tasks, key=key)


def build_and_test() -> tuple[bool, str]:
    """Run cargo test. Returns (success, output)."""
    env = dict(os.environ)
    env["PATH"] = f"/usr/bin:{env.get('PATH', '')}"

    result = subprocess.run(
        ["cargo", "test", "-p", "rebellion-core", "-p", "rebellion-data"],
        capture_output=True, text=True, env=env, timeout=180,
    )
    if result.returncode != 0:
        return False, result.stderr[-500:]
    return True, "all tests pass"


def run_claude(task: dict) -> tuple[bool, float]:
    """Run claude -p to implement a task. Returns (success, wall_time)."""
    prompt = f"""You are implementing a specific parity task for Open Rebellion at /Users/tomdimino/Desktop/Programming/open-rebellion.

## Task: {task['id']} — {task['title']}

{task['prompt']}

## Rules
1. Implement this ONE task completely.
2. All tests must pass: PATH="/usr/bin:$PATH" cargo test -p rebellion-core -p rebellion-data
3. Follow existing code patterns (read files before modifying).
4. Do NOT modify unrelated files.
5. Do NOT touch tuning.rs config values, eval scripts, or autoresearch infrastructure.
6. Add tests for new functionality.

## Verification
After implementing, run: PATH="/usr/bin:$PATH" cargo test
Confirm zero failures before finishing."""

    env = dict(os.environ)
    env.pop("ANTHROPIC_API_KEY", None)
    env["PATH"] = f"/usr/bin:{env.get('PATH', '')}"

    t0 = time.time()
    result = subprocess.run(
        ["claude", "-p", prompt, "--dangerously-skip-permissions", "--max-turns", "30"],
        capture_output=True, text=True, env=env, timeout=900,
        cwd=str(PROJECT_DIR),
    )
    elapsed = time.time() - t0
    return result.returncode == 0, elapsed


def git_commit(task_id: str, title: str):
    subprocess.run(["git", "add", "-A"], cwd=str(PROJECT_DIR), capture_output=True)
    subprocess.run(
        ["git", "commit", "-m",
         f"feat: {task_id} — {title}\n\n"
         f"Autoresearch parity work loop.\n\n"
         f"Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"],
        cwd=str(PROJECT_DIR), capture_output=True,
    )


def git_discard():
    subprocess.run(["git", "checkout", "--", "."], cwd=str(PROJECT_DIR), capture_output=True)
    subprocess.run(["git", "clean", "-fd"], cwd=str(PROJECT_DIR), capture_output=True)


def append_tsv(row: dict):
    RESULTS_TSV.parent.mkdir(parents=True, exist_ok=True)
    header = "task_id\ttitle\tstatus\tclaude_time\ttest_result\n"
    if not RESULTS_TSV.exists():
        RESULTS_TSV.write_text(header)
    with open(RESULTS_TSV, "a") as f:
        f.write(f"{row['id']}\t{row['title']}\t{row['status']}\t"
                f"{row['claude_time']:.1f}s\t{row['test_result']}\n")


def main():
    parser = argparse.ArgumentParser(description="Sequential parity work loop")
    parser.add_argument("--dry-run", action="store_true", help="Show task order without executing")
    parser.add_argument("--max-tasks", type=int, default=999, help="Max tasks to execute")
    parser.add_argument("--start-at", type=str, help="Skip to this task ID")
    args = parser.parse_args()

    tasks = load_manifest()
    completed, failed = load_progress()
    completed_set = set(completed)

    print(f"{'='*60}")
    print(f"PARITY WORK LOOP")
    print(f"{'='*60}")
    print(f"Total tasks: {len(tasks)}")
    print(f"Completed: {len(completed)}")
    print(f"Failed: {len(failed)}")
    print()

    tasks_done = 0
    skip_until = args.start_at

    while tasks_done < args.max_tasks:
        unblocked = get_unblocked(tasks, completed_set)
        unblocked = [t for t in unblocked if t["id"] not in failed]
        unblocked = sort_by_priority(unblocked)

        if not unblocked:
            print("No more unblocked tasks.")
            break

        # Skip to start-at if specified
        if skip_until:
            found = False
            for i, t in enumerate(unblocked):
                if t["id"] == skip_until:
                    unblocked = unblocked[i:]
                    found = True
                    break
            if not found:
                print(f"Task {skip_until} not found in unblocked set. Available: {[t['id'] for t in unblocked]}")
                break
            skip_until = None  # only skip once

        task = unblocked[0]

        print(f"{'─'*60}")
        print(f"[{tasks_done + 1}] {task['id']}: {task['title']}")
        print(f"    Category: {task['category']} | Est: {task.get('est_loc', '?')} LOC")
        print(f"    Files: {', '.join(task.get('files', []))}")

        if args.dry_run:
            print(f"    (dry run — would execute claude -p)")
            completed_set.add(task["id"])  # pretend complete for dep resolution
            tasks_done += 1
            continue

        # Execute
        claude_ok, claude_time = run_claude(task)

        if not claude_ok:
            print(f"    claude -p FAILED ({claude_time:.1f}s)")
            git_discard()
            failed.append(task["id"])
            save_progress(completed, failed)
            append_tsv({"id": task["id"], "title": task["title"],
                        "status": "CLAUDE_FAIL", "claude_time": claude_time,
                        "test_result": "n/a"})
            tasks_done += 1
            continue

        # Test
        test_ok, test_msg = build_and_test()

        if test_ok:
            git_commit(task["id"], task["title"])
            completed.append(task["id"])
            completed_set.add(task["id"])
            save_progress(completed, failed)
            print(f"    DONE ({claude_time:.1f}s)")
            append_tsv({"id": task["id"], "title": task["title"],
                        "status": "DONE", "claude_time": claude_time,
                        "test_result": "pass"})
        else:
            git_discard()
            failed.append(task["id"])
            save_progress(completed, failed)
            print(f"    TESTS FAILED ({claude_time:.1f}s)")
            append_tsv({"id": task["id"], "title": task["title"],
                        "status": "TEST_FAIL", "claude_time": claude_time,
                        "test_result": test_msg[:100]})

        tasks_done += 1

    print(f"\n{'='*60}")
    print(f"COMPLETE: {len(completed)} done, {len(failed)} failed, "
          f"{len(tasks) - len(completed) - len(failed)} remaining")


if __name__ == "__main__":
    main()
