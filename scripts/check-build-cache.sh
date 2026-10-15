#!/usr/bin/env bash
# Report the disk the build cache and run evidence hold, and keep both from
# piling up. Builds run on the Mac Mini (scripts/mini-cargo.sh), which keeps
# only what its latest build used; this Mac keeps no target/. Browser and
# parity runs write a new timestamped folder under .artifacts/interface-parity/
# each time.
#
#   scripts/check-build-cache.sh            report; warn over the limits
#   scripts/check-build-cache.sh --prune    prune now (run only when no
#                                           build, test or mutants run is
#                                           in flight)
#   scripts/check-build-cache.sh --auto     prune unless a cargo, rustc or
#                                           cargo-mutants process is running;
#                                           the SessionStart hook uses this
#
# Pruning:
#   - runs `cargo clean` when this Mac has a target/ (a local build ran);
#   - removes stale cargo-mutants scratch copies, and scratch target
#     directories (any target/ entry cargo does not make) older than 2 days;
#   - deletes stale run folders (see stale_runs below);
#   - moves evidence older than the age limit to the Mac Mini archive under
#     the same relative path (see archive_old below).
#
# OPEN_REBELLION_TARGET_LIMIT_GB   local target/ allowed before cleaning
#                                  (default 0: none)
# OPEN_REBELLION_RUN_KEEP_DAYS     age limit for runs and evidence (default 14)
# OPEN_REBELLION_ARCHIVE           rsync destination (default
#                                  mac-mini-ts:open-rebellion-archive)
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
limit_gb="${OPEN_REBELLION_TARGET_LIMIT_GB:-0}"
keep_days="${OPEN_REBELLION_RUN_KEEP_DAYS:-14}"
archive="${OPEN_REBELLION_ARCHIVE:-mac-mini-ts:open-rebellion-archive}"
mode=report
case "${1:-}" in
  --prune) mode=prune ;;
  --auto) mode=auto ;;
  "") ;;
  *) echo "usage: $0 [--prune|--auto]" >&2; exit 2 ;;
esac

if [[ $mode == auto ]]; then
  if pgrep -x cargo >/dev/null || pgrep -x rustc >/dev/null || pgrep -f cargo-mutants >/dev/null; then
    echo "build cache: a cargo, rustc or cargo-mutants process is running; reporting only"
    mode=report
  else
    mode=prune
  fi
fi

size_gb() {
  [[ -e "$1" ]] || { echo 0; return; }
  du -sk "$1" 2>/dev/null | awk '{ printf "%d", $1 / 1048576 }'
}

target_gb="$(size_gb "$root/target")"
artifacts_gb="$(size_gb "$root/.artifacts")"
tmp="${TMPDIR:-/tmp}"
stale_mutants=()
for dir in "$tmp"/cargo-mutants-*; do
  [[ -d "$dir" ]] && stale_mutants+=("$dir")
done

# Entries cargo makes in target/; anything else is a scratch target dir
# (a worktree's CARGO_TARGET_DIR, a one-off check).
scratch_targets=()
if [[ -d "$root/target" ]]; then
  while IFS= read -r dir; do
    case "$(basename "$dir")" in
      debug|release|doc|tmp|package|wasm32-*|*-apple-darwin|*-linux-gnu|*-windows-*) ;;
      *) scratch_targets+=("$dir") ;;
    esac
  done < <(find "$root/target" -mindepth 1 -maxdepth 1 -type d -mtime +2)
fi

echo "build cache: local target/ ${target_gb} GB (limit ${limit_gb} GB), .artifacts/ ${artifacts_gb} GB, cargo-mutants scratch copies ${#stale_mutants[@]}, scratch target dirs ${#scratch_targets[@]}"
# The Mini's last pruned build (scripts/prune-target.py --log).
mini="$(ssh -o ConnectTimeout=3 -o BatchMode=yes "${OPEN_REBELLION_BUILD_HOST:-mac-mini-ts}" 'tail -1 or-native/build-cache.log' 2>/dev/null || true)"
[[ -n $mini ]] && echo "build cache (Mini, last build): $mini"

if [[ -d "$root/target" ]] && (( target_gb >= limit_gb )); then
  if [[ $mode == prune ]]; then
    env PATH="/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:$HOME/.cargo/bin" \
      cargo clean --manifest-path "$root/Cargo.toml"
  else
    echo "WARNING: this Mac has a target/ (${target_gb} GB); build with scripts/mini-cargo.sh, and run scripts/check-build-cache.sh --prune when no build is running"
  fi
fi

if (( ${#stale_mutants[@]} > 0 )); then
  if [[ $mode == prune ]]; then
    rm -rf "${stale_mutants[@]}"
  else
    echo "WARNING: ${#stale_mutants[@]} cargo-mutants scratch copies remain in $tmp; --prune removes them once no mutants run is in flight"
  fi
fi

if (( ${#scratch_targets[@]} > 0 )); then
  if [[ $mode == prune ]]; then
    rm -rf "${scratch_targets[@]}"
  else
    echo "WARNING: scratch target dirs older than two days: ${scratch_targets[*]}"
  fi
fi

run_pattern='[A-Za-z0-9._-]*20[0-9]{2}-[0-9]{2}-[0-9]{2}T[0-9-]+Z-[0-9]+'
runs="$root/.artifacts/interface-parity"

# Print the stale run folders of .artifacts/interface-parity/, one name per
# line. A timestamped run (NAME-<ISO>Z-<pid> or <ISO>Z-<pid>) is stale when
# nothing names it (no tracked file, progress.json or a0/ record), it is
# older than keep_days, and it is not one of the two newest runs of its
# NAME: a superseded attempt that no evidence cites.
stale_runs() {
  [[ -d $runs ]] || return 0
  {
    git -C "$root" grep -hoE "$run_pattern" || true
    cat "$root/progress.json" 2>/dev/null || true
    grep -rhoE "$run_pattern" "$runs/a0" 2>/dev/null || true
  } > "$tmp/or-run-refs.$$"
  python3 - "$runs" "$keep_days" "$tmp/or-run-refs.$$" <<'EOF'
import re, sys, time
from pathlib import Path

runs, keep_days = Path(sys.argv[1]), int(sys.argv[2])
refs = Path(sys.argv[3]).read_text()
pattern = re.compile(r"^(.*?)-?(20\d\d-\d\d-\d\dT[\d-]+Z-\d+)$")
groups = {}
for d in runs.iterdir():
    m = pattern.match(d.name)
    if d.is_dir() and m:
        groups.setdefault(m.group(1), []).append(d)
cutoff = time.time() - keep_days * 86400
for dirs in groups.values():
    dirs.sort(key=lambda d: d.name)
    for d in dirs[:-2]:
        if d.name not in refs and d.stat().st_mtime < cutoff:
            print(d.name)
EOF
  rm -f "$tmp/or-run-refs.$$"
}

stale=()
while IFS= read -r name; do stale+=("$name"); done < <(stale_runs)
if (( ${#stale[@]} > 0 )); then
  if [[ $mode == prune ]]; then
    for name in "${stale[@]}"; do rm -rf "${runs:?}/$name"; done
    echo "run folders: deleted ${#stale[@]} stale"
  else
    echo "WARNING: ${#stale[@]} stale run folders in .artifacts/interface-parity; --prune deletes them"
  fi
fi

# Evidence folders (each entry one level inside an .artifacts/ category, such
# as native-checks/2026-10-08-stockpiles) that nothing has written for
# keep_days move to $archive/.artifacts/<same path>. The local copy is
# deleted only after rsync succeeds. The A0 captures (interface-parity/a0)
# and the staged web build (interface-parity/site) stay local.
old=()
if [[ -d $root/.artifacts ]]; then
  while IFS= read -r dir; do
    rel="${dir#"$root"/}"
    case "$rel" in
      .artifacts/interface-parity/a0|.artifacts/interface-parity/site) continue ;;
    esac
    [[ -n $(find "$dir" -mtime "-${keep_days}" -print -quit) ]] && continue
    old+=("$rel")
  done < <(find "$root/.artifacts" -mindepth 2 -maxdepth 2)
fi
if (( ${#old[@]} > 0 )); then
  if [[ $mode == prune ]]; then
    host="${archive%%:*}" dest="${archive#*:}"
    if ssh -o ConnectTimeout=10 -o BatchMode=yes "$host" "mkdir -p '$dest'" &&
       printf '%s\n' "${old[@]}" | rsync -a -r --files-from=- "$root/" "$archive/"; then
      for rel in "${old[@]}"; do rm -rf "${root:?}/$rel"; done
      echo "evidence: moved ${#old[@]} folders older than ${keep_days} days to $archive"
    else
      echo "WARNING: could not reach $archive; ${#old[@]} old evidence folders kept"
    fi
  else
    echo "WARNING: ${#old[@]} evidence folders older than ${keep_days} days; --prune moves them to $archive"
  fi
fi
