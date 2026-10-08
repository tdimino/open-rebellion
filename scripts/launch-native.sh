#!/usr/bin/env bash
# Launch the native release build muted, for native GUI acceptance runs.
#
#   scripts/launch-native.sh [--build] [--evidence DIR] [--commands FILE]
#                            [--seed N] [-- GAME_ARGS...]
#
# --build          rebuild target/release/open-rebellion first
# --evidence DIR   write binary.txt (sha256, source commit, pid, window id)
#                  and game.log into DIR
# --commands FILE  run FILE's developer commands, one per line, and enable
#                  the command palette (OPEN_REBELLION_COMMANDS,
#                  OPEN_REBELLION_DEV=1); each logs [dev-command] to game.log
# --seed N         seed every new campaign with N (OPEN_REBELLION_SEED), so
#                  a script meets the same galaxy each run; the seed, like a
#                  script, needs OPEN_REBELLION_DEV=1, which this sets
#
# OPEN_REBELLION_MUTE=1 silences music, effects and cutscenes from launch,
# so the options-screen sliders need no clicks. Prints the pid and, when
# cua-driver is installed, the window id to pass to its calls.
set -euo pipefail

repo="$(cd "$(dirname "$0")/.." && pwd)"
binary="$repo/target/release/open-rebellion"
build=0
evidence=""
commands=""
seed=""
game_args=()

while [ $# -gt 0 ]; do
  case "$1" in
    --build) build=1 ;;
    --evidence) evidence="$2"; shift ;;
    --seed) seed="$2"; shift ;;
    --commands) commands="$(cd "$(dirname "$2")" && pwd)/$(basename "$2")"; shift ;;
    --) shift; game_args=("$@"); break ;;
    *) echo "unknown option: $1" >&2; exit 2 ;;
  esac
  shift
done
[ ${#game_args[@]} -eq 0 ] && game_args=(data/base)

if [ "$build" -eq 1 ]; then
  # ~/.local/bin/cc is not a C compiler; build with the sanitized PATH.
  (cd "$repo" && env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:"$HOME"/.cargo/bin \
    cargo build -q --release -p rebellion-app)
fi
[ -x "$binary" ] || { echo "no $binary; pass --build" >&2; exit 1; }

log=/dev/null
if [ -n "$evidence" ]; then
  mkdir -p "$evidence"
  log="$evidence/game.log"
fi

cd "$repo"
dev_env=()
[ -n "$commands" ] && dev_env=(OPEN_REBELLION_DEV=1 OPEN_REBELLION_COMMANDS="$commands")
[ -n "$seed" ] && dev_env+=(OPEN_REBELLION_DEV=1 OPEN_REBELLION_SEED="$seed")
env OPEN_REBELLION_MUTE=1 ${dev_env[@]+"${dev_env[@]}"} nohup "$binary" "${game_args[@]}" >"$log" 2>&1 &
pid=$!

window=""
if command -v cua-driver >/dev/null; then
  # A cold start loads the data before the window appears: wait up to 30 s.
  for _ in $(seq 1 120); do
    window="$(cua-driver call list_windows '{}' 2>/dev/null | python3 -c '
import json, sys
pid = int(sys.argv[1])
# The pid also owns a menu-bar strip; the game window is the largest.
own = [w for w in json.load(sys.stdin).get("windows", []) if w.get("pid") == pid]
area = lambda w: w["bounds"]["width"] * w["bounds"]["height"]
if own and area(max(own, key=area)) > 100 * 100:
    print(max(own, key=area)["window_id"])
' "$pid" || true)"
    [ -n "$window" ] && break
    sleep 0.25
  done
fi

sha="$(shasum -a 256 "$binary" | cut -d' ' -f1)"
commit="$(git -C "$repo" rev-parse --short HEAD)"
dirty="$(git -C "$repo" status --porcelain --untracked-files=no | head -c1)"
summary="binary_sha256=$sha source=$commit${dirty:+ (dirty tree)} pid=$pid window_id=${window:-unknown} muted=1${commands:+ commands=$commands}${seed:+ seed=$seed}"
echo "$summary"
[ -n "$evidence" ] && echo "$summary" >"$evidence/binary.txt"
exit 0
