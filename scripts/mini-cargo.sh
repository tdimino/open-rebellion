#!/usr/bin/env bash
# Run Cargo on the Mac Mini instead of this Mac, so no build cache grows here.
#
#   scripts/mini-cargo.sh [--pull PATH]... [--keep] [--] CARGO_ARGS...
#   scripts/mini-cargo.sh --run 'COMMAND'     e.g. --run ./scripts/build-wasm.sh
#
# 1. Syncs git's file list (tracked, plus untracked files git does not
#    ignore) to the Mini's copy, and deletes there the files an earlier sync
#    sent that are gone here. Ignored files on the Mini (target/, .artifacts/,
#    data/base/) are never touched.
# 2. Writes .build-source there (commit, dirty file count) for evidence.
# 3. Runs `cargo CARGO_ARGS` (or COMMAND) with the sanitized PATH and streams
#    its output. Cargo also writes its JSON unit messages
#    (--message-format=json-render-diagnostics, added after the subcommand),
#    which go to a file instead of the terminal.
# 4. Unless --keep: scripts/prune-target.py removes every unit this build
#    did not use, stale codegen objects and old incremental directories, so
#    the Mini keeps what one build needs. Each build appends a line to
#    ~/or-native/build-cache.log. A failed build or a --run COMMAND gives
#    no complete unit list; its prune skips step 1 of prune-target.py.
# 5. --pull PATH copies PATH back from the Mini after a successful build
#    (e.g. web/open-rebellion.wasm).
#
# Exits with Cargo's status.
#
# OPEN_REBELLION_BUILD_HOST  ssh host (default mac-mini-ts)
# OPEN_REBELLION_BUILD_DIR   repository copy on it (default or-native/open-rebellion)
set -euo pipefail

repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
host="${OPEN_REBELLION_BUILD_HOST:-mac-mini-ts}"
dir="${OPEN_REBELLION_BUILD_DIR:-or-native/open-rebellion}"
pulls=()
keep=0
run=""
while [ $# -gt 0 ]; do
  case "$1" in
    --pull) pulls+=("$2"); shift ;;
    --keep) keep=1 ;;
    --run) run="$2"; shift ;;
    --) shift; break ;;
    *) break ;;
  esac
  shift
done
[ -n "$run" ] || [ $# -gt 0 ] || { echo "usage: $0 [--pull PATH]... [--keep] [--run CMD | -- CARGO_ARGS...]" >&2; exit 2; }

cd "$repo"
manifest="$(mktemp)"
# Two runs syncing at once would delete each other's files: one run's
# deletions read the manifest while the other is still writing it. The sync,
# up to the deletions, therefore holds a lock on the Mini (mkdir is atomic);
# a lock older than ten minutes belongs to a dead run and is broken. The
# owner token lets the exit trap release only its own lock.
token="$$-$RANDOM-$(date +%s)"
lock=".mini-cargo-sync.lock"
release="[ \"\$(cat $lock/owner 2>/dev/null)\" = $token ] && rm -rf $lock; true"
trap 'rm -f "$manifest"; ssh "$host" "cd $(printf %q "$dir") && $release" 2>/dev/null || true' EXIT
ssh "$host" "cd $(printf '%q' "$dir") && \
  until mkdir $lock 2>/dev/null; do \
    [ -n \"\$(find $lock -maxdepth 0 -mmin +10 2>/dev/null)\" ] && rm -rf $lock; sleep 1; \
  done && echo $token > $lock/owner"
git ls-files -z --cached --others --exclude-standard \
  | tr '\0' '\n' | grep -v -E '^(mutants\.out|mutants\.out\.old)/' | while IFS= read -r f; do
      if [ -e "$f" ]; then printf '%s\n' "$f"; fi
    done > "$manifest"
rsync -a --files-from="$manifest" ./ "$host:$dir/"
rsync -q "$manifest" "$host:$dir/.mini-cargo-manifest.new"

commit="$(git rev-parse --short HEAD)"
dirty="$(git status --porcelain | wc -l | tr -d ' ')"

quote() { printf '%q ' "$@"; }
units=""
if [ -n "$run" ]; then
  command="$run"
else
  # The message format goes right after the subcommand, before any `--`.
  sub="$1"; shift
  case "$sub" in
    build|b|check|c|test|t|run|r|bench|clippy|doc|rustc)
      if ! printf '%s\n' "$@" | grep -q -- '--message-format'; then
        units=".mini-cargo-units.json"
        set -- --message-format=json-render-diagnostics "$@"
      fi ;;
  esac
  command="cargo $(quote "$sub" "$@")"
  # Unit messages to the file, everything else (tests, the program) through.
  [ -n "$units" ] && command="set -o pipefail; $command | python3 scripts/prune-target.py --tee-units $units"
fi

status=0
# shellcheck disable=SC2029  # the command is built and quoted here on purpose
ssh "$host" "cd $(printf '%q' "$dir") && \
  { [ -f .mini-cargo-manifest ] && comm -23 <(sort .mini-cargo-manifest) <(sort .mini-cargo-manifest.new) | while IFS= read -r f; do rm -f -- \"\$f\"; done; true; } && \
  mv .mini-cargo-manifest.new .mini-cargo-manifest && \
  { $release; } && \
  printf 'source=%s dirty_files=%s synced=%s\n' $commit $dirty \"\$(date -u +%Y-%m-%dT%H:%M:%SZ)\" > .build-source && \
  export PATH=\"\$HOME/.cargo/bin:/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin\" CARGO_TERM_COLOR=always && \
  rm -f .mini-cargo-units.json && \
  { $command; rc=\$?; \
    if [ $keep -eq 0 ]; then u=; [ \$rc -eq 0 ] && u=$units; \
      python3 scripts/prune-target.py --force \${u:+--units=\$u} \
      --log \"\$HOME/or-native/build-cache.log\"; fi; \
    exit \$rc; }" || status=$?

if [ "$status" -eq 0 ]; then
  for path in "${pulls[@]+"${pulls[@]}"}"; do
    rsync -a "$host:$dir/$path" "$repo/$path"
  done
fi
exit "$status"
