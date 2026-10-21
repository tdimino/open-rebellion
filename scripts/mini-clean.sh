#!/usr/bin/env bash
# Remove what Open Rebellion work leaves behind on the Mac Mini. It reports by
# default and deletes only with --apply. scripts/disk-report.sh shows the
# space before and after.
#
#   scripts/mini-clean.sh                    report what would go
#   scripts/mini-clean.sh --apply            delete it
#   scripts/mini-clean.sh --apply --caches   also trim Homebrew's and uv's caches
#
# What goes, under ~/or-native:
#   - a private clone (a folder with Cargo.toml other than open-rebellion)
#     that nothing has written for OPEN_REBELLION_CLONE_KEEP_DAYS (default 3)
#     and that no process runs in or names. A younger clone keeps its tree,
#     and its target/ is pruned like the shared copy's (prune-target.py);
#   - cargo-mutants output folders (mutants*, mut-*) and loose diffs, patches,
#     replay dumps and *.err logs (but prune-target.err) older than
#     OPEN_REBELLION_RUN_KEEP_DAYS (default 2), while no cargo-mutants runs;
#   - cargo-mutants scratch copies in $TMPDIR older than a day.
# Never touched: the shared copy's tree, ~/open-rebellion-data, the VMs, the
# UTM container, the launchd agent's logs, or anything outside the project
# but those $TMPDIR copies and, with --caches, Homebrew's and uv's caches.
#
# The daily launchd agent (scripts/install-mini-prune-agent.sh) runs
# `--local --apply --caches` after the target prune.
#
# OPEN_REBELLION_BUILD_HOST   the Mini (default mac-mini-ts)
set -euo pipefail

host="${OPEN_REBELLION_BUILD_HOST:-mac-mini-ts}"
script="${BASH_SOURCE[0]:-}"
local=0 apply=0 caches=0
for arg in "$@"; do
  case "$arg" in
    --local) local=1 ;;
    --apply) apply=1 ;;
    --caches) caches=1 ;;
    *) echo "usage: $0 [--local] [--apply] [--caches]" >&2; exit 2 ;;
  esac
done
if (( ! local )); then
  # The script goes as an argument, not on stdin, which its commands would read.
  exec ssh "$host" "OPEN_REBELLION_CLONE_KEEP_DAYS=${OPEN_REBELLION_CLONE_KEEP_DAYS:-3} \
OPEN_REBELLION_RUN_KEEP_DAYS=${OPEN_REBELLION_RUN_KEEP_DAYS:-2} \
bash -c $(printf %q "$(cat "$script")") mini-clean --local $*"
fi

base="$HOME/or-native"
shared="$base/open-rebellion"
clone_days="${OPEN_REBELLION_CLONE_KEEP_DAYS:-3}"
run_days="${OPEN_REBELLION_RUN_KEEP_DAYS:-2}"
export PATH="$HOME/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin"
freed_before=$(df -k / | awk 'NR == 2 { print $4 }')

gb() { du -sk "$1" 2>/dev/null | awk '{ printf "%.1f GB", $1 / 1048576 }'; }

# True when a process runs inside $1 or names it on its command line.
in_use() {
  ps -axo command= | grep -F -- "$1" | grep -qv grep && return 0
  lsof -d cwd -Fn 2>/dev/null | grep -q "^n$1\(/\|$\)"
}

remove() {
  if (( apply )); then
    rm -rf -- "$1" && echo "deleted   $(basename "$1")  $2"
  else
    echo "would delete $(basename "$1")  $2"
  fi
}

mutants_running=0
pgrep -f cargo-mutants >/dev/null && mutants_running=1

for dir in "$base"/*/; do
  dir=${dir%/}
  [[ -f $dir/Cargo.toml && $dir != "$shared" ]] || continue
  if in_use "$dir"; then
    echo "in use    $(basename "$dir")"
  elif [[ -z $(find "$dir" -mtime "-$clone_days" -print -quit 2>/dev/null) ]]; then
    remove "$dir" "(clone, idle over $clone_days days, target $(gb "$dir/target"))"
    continue
  else
    echo "keep      $(basename "$dir")  (written within $clone_days days, target $(gb "$dir/target"))"
  fi
  if [[ -d $dir/target ]] && (( apply )); then
    python3 "$shared/scripts/prune-target.py" --target "$dir/target" --log "$base/build-cache.log" ||
      echo "prune skipped for $(basename "$dir") (a build holds its lock)"
  fi
done

if (( mutants_running )); then
  echo "cargo-mutants is running; its output folders stay"
else
  while IFS= read -r path; do
    remove "$path" "(run output older than $run_days days)"
  done < <(find "$base" -mindepth 1 -maxdepth 1 -mtime "+$run_days" \
    \( -type d \( -name 'mutants*' -o -name 'mut-*' \) -o \
       -type f \( -name '*.diff' -o -name '*.patch' -o -name 'replay_*.json' -o -name '*.err' \) \
       ! -name 'prune-target.err' \) | sort)
  while IFS= read -r path; do
    remove "$path" "(cargo-mutants scratch copy)"
  done < <(find "${TMPDIR:-/tmp}" -mindepth 1 -maxdepth 1 -type d -name 'cargo-mutants-*' -mtime +1 2>/dev/null)
fi

if (( caches )); then
  if (( apply )); then
    brew cleanup --prune=7 -q 2>/dev/null || true
    command -v uv >/dev/null && uv cache prune -q 2>/dev/null || true
    echo "caches: Homebrew and uv trimmed"
  else
    echo "would trim Homebrew ($(gb "$HOME/Library/Caches/Homebrew")) and uv ($(gb "$HOME/.cache/uv")) caches"
  fi
fi

freed_after=$(df -k / | awk 'NR == 2 { print $4 }')
(( apply )) && awk -v a="$freed_before" -v b="$freed_after" 'BEGIN { printf "freed %.1f GB; %.1f GB free\n", (b - a) / 1048576, b / 1048576 }'
true
