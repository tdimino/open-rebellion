#!/usr/bin/env bash
# Launch the native release build muted, for native GUI acceptance runs.
# A launch first stops this checkout's running game, so each checkout runs
# at most one.
#
#   scripts/launch-native.sh [--build] [--evidence DIR] [--commands FILE]
#                            [--seed N] [--live] [--load FILE] [--telemetry]
#                            [-- GAME_ARGS...]
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
# --live           open the live channel (needs --evidence): the game runs
#                  each line appended to DIR/commands.in and answers in
#                  DIR/commands.out; send with scripts/dev-send.sh. Captures
#                  go to DIR (OPEN_REBELLION_EVIDENCE). agent_docs/dev-commands.md
# --load FILE      start straight into a save (needs --live): FILE is copied to
#                  DIR/saves/<name>.reb and `Load: <name>` is the inbox's first
#                  line, so it answers as seq 1 and loads from the main menu.
#                  FILE must be on the host the game runs on. The save leaves
#                  out the message log, advisor and advice-topic state, open
#                  windows and any battle (agent_docs/dev-commands.md, "Save
#                  and Load")
# --telemetry      write DIR/telemetry.jsonl (needs --evidence): the game's
#                  events in the playtest's record schema, each with the
#                  command seq running at the time. OPEN_REBELLION_TRACE=
#                  missions,ai,research,blockade,combat,control, set in the
#                  caller's environment, adds decision traces.
#                  agent_docs/simulation.md, "Telemetry and Traces"
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
live=0
load=""
telemetry=0
game_args=()

while [ $# -gt 0 ]; do
  case "$1" in
    --build) build=1 ;;
    --evidence) evidence="$2"; shift ;;
    --seed) seed="$2"; shift ;;
    --live) live=1 ;;
    --load) load="$(cd "$(dirname "$2")" && pwd)/$(basename "$2")"; shift ;;
    --telemetry) telemetry=1 ;;
    --commands) commands="$(cd "$(dirname "$2")" && pwd)/$(basename "$2")"; shift ;;
    --) shift; game_args=("$@"); break ;;
    *) echo "unknown option: $1" >&2; exit 2 ;;
  esac
  shift
done
[ ${#game_args[@]} -eq 0 ] && game_args=(data/base)
if [ "$live" -eq 1 ] && [ -z "$evidence" ]; then
  echo "--live needs --evidence DIR" >&2; exit 2
fi
if [ -n "$load" ] && [ "$live" -ne 1 ]; then
  echo "--load needs --live and --evidence DIR" >&2; exit 2
fi
if [ -n "$load" ] && [ ! -f "$load" ]; then
  echo "no save $load" >&2; exit 2
fi
if [ "$telemetry" -eq 1 ] && [ -z "$evidence" ]; then
  echo "--telemetry needs --evidence DIR" >&2; exit 2
fi

if [ "$build" -eq 1 ]; then
  # ~/.local/bin/cc is not a C compiler; build with the sanitized PATH.
  (cd "$repo" && env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:"$HOME"/.cargo/bin \
    cargo build -q --release -p rebellion-app)
fi
[ -x "$binary" ] || { echo "no $binary; pass --build" >&2; exit 1; }

log=/dev/null
if [ -n "$evidence" ]; then
  mkdir -p "$evidence"
  evidence="$(cd "$evidence" && pwd)"
  log="$evidence/game.log"
fi

# One game per checkout: stop this checkout's earlier instance first. The
# match is the absolute binary path, so another checkout's game keeps running.
if pkill -f "^$binary( |$)"; then
  for _ in $(seq 1 40); do
    pgrep -f "^$binary( |$)" >/dev/null || break
    sleep 0.25
  done
fi

cd "$repo"
dev_env=()
[ -n "$commands" ] && dev_env=(OPEN_REBELLION_DEV=1 OPEN_REBELLION_COMMANDS="$commands")
[ -n "$seed" ] && dev_env+=(OPEN_REBELLION_DEV=1 OPEN_REBELLION_SEED="$seed")
[ -n "$evidence" ] && dev_env+=(OPEN_REBELLION_EVIDENCE="$evidence")
[ "$telemetry" -eq 1 ] && dev_env+=(OPEN_REBELLION_TELEMETRY=1)
if [ "$live" -eq 1 ]; then
  # A fresh inbox each launch: sequence numbers count its lines from 1.
  : >"$evidence/commands.in"
  if [ -n "$load" ]; then
    # The same file name `Load:` reads: <evidence>/saves/<name>.reb.
    name="$(basename "$load" .reb)"
    case "$name" in
      ""|.*|*[!A-Za-z0-9._-]*) echo "save name $name: use letters, digits, - _ ." >&2; exit 2 ;;
    esac
    mkdir -p "$evidence/saves"
    cp "$load" "$evidence/saves/$name.reb"
    echo "Load: $name" >>"$evidence/commands.in"
  fi
  dev_env+=(OPEN_REBELLION_DEV=1 OPEN_REBELLION_INBOX="$evidence/commands.in")
fi
env OPEN_REBELLION_MUTE=1 ${dev_env[@]+"${dev_env[@]}"} nohup "$binary" "${game_args[@]}" >"$log" 2>&1 &
pid=$!

window=""
if command -v cua-driver >/dev/null; then
  # A cold start loads the data before the window appears: wait up to 60 s.
  for _ in $(seq 1 240); do
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
if git -C "$repo" rev-parse --git-dir >/dev/null 2>&1; then
  commit="$(git -C "$repo" rev-parse --short HEAD)"
  dirty="$(git -C "$repo" status --porcelain --untracked-files=no | head -c1)"
else
  # The Mini's copy has no .git; scripts/mini-cargo.sh records its source.
  commit="$(sed -n 's/^source=\([^ ]*\).*/\1/p' "$repo/.build-source" 2>/dev/null)"
  dirty="$(sed -n 's/.*dirty_files=\([1-9][0-9]*\).*/\1/p' "$repo/.build-source" 2>/dev/null)"
  commit="${commit:-unknown}"
fi
summary="binary_sha256=$sha source=$commit${dirty:+ (dirty tree)} pid=$pid window_id=${window:-unknown} muted=1${commands:+ commands=$commands}${seed:+ seed=$seed}"
[ "$live" -eq 1 ] && summary="$summary inbox=$evidence/commands.in"
echo "$summary"
[ -n "$evidence" ] && echo "$summary" >"$evidence/binary.txt"
exit 0
