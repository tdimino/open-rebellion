#!/usr/bin/env bash
# Send developer commands to a game launched with --live, and print each
# result.
#
#   scripts/dev-send.sh DIR "<command>" [--timeout SECONDS]
#   scripts/dev-send.sh DIR --file FILE|- [--timeout SECONDS] [--keep-going]
#
# DIR is the launcher's --evidence folder. The command is appended to
# DIR/commands.in. The script waits for the line's result in
# DIR/commands.out and prints it as one JSON line. It exits 0 when the
# command ran, 1 when it was refused or skipped, and 2 on a timeout or bad
# usage. agent_docs/dev-commands.md lists the commands.
#
# --file first expands `Include: PATH` lines (scripts/dev-expand.sh), then sends
# each non-empty line that does not start with `#` in turn, each
# through the same lock and wait loop, and prints each result line. FILE `-`
# reads standard input. --timeout applies to each line. The batch stops at the
# first refused line and exits 1; with --keep-going it sends the rest and
# exits 1 if any line was refused. A timeout always stops the batch (exit 2):
# the game is not answering, so later lines would pile up unread.
set -euo pipefail

usage() {
  echo "usage: $0 DIR \"<command>\" [--timeout S]" >&2
  echo "       $0 DIR --file FILE|- [--timeout S] [--keep-going]" >&2
  exit 2
}

[ $# -ge 2 ] || usage
dir="$1" timeout=30 file="" keep_going=0 command=""
shift
if [ "$1" = "--file" ]; then
  [ $# -ge 2 ] || usage
  file="$2"
  shift 2
else
  command="$1"
  shift
fi
while [ $# -gt 0 ]; do
  case "$1" in
    --timeout) [ $# -ge 2 ] || usage; timeout="$2"; shift ;;
    --keep-going) keep_going=1 ;;
    *) echo "unknown option: $1" >&2; exit 2 ;;
  esac
  shift
done
[ -z "$file" ] || [ -f "$file" ] || [ "$file" = "-" ] || { echo "no file $file" >&2; exit 2; }
[ "$keep_going" = 0 ] || [ -n "$file" ] || { echo "--keep-going needs --file" >&2; exit 2; }
inbox="$dir/commands.in" out="$dir/commands.out"
[ -f "$inbox" ] || { echo "no $inbox; launch with --live --evidence $dir" >&2; exit 2; }

# Send one command and print its result. Returns 0 when it ran, 1 when it was
# refused or skipped, 2 on a timeout or when the lock is stuck.
send_one() {
  local line="$1" n lock="$dir/.commands.lock" deadline result
  # mkdir is atomic: the lock keeps two senders' lines and numbers apart.
  for _ in $(seq 1 100); do mkdir "$lock" 2>/dev/null && break; sleep 0.05; done
  [ -d "$lock" ] || { echo "cannot take $lock" >&2; return 2; }
  trap 'rmdir "$lock" 2>/dev/null || true' EXIT
  printf '%s\n' "$line" >>"$inbox"
  n="$(wc -l <"$inbox" | tr -d ' ')"
  rmdir "$lock"
  trap - EXIT

  deadline=$(( $(date +%s) + timeout ))
  while :; do
    result="$(grep -F "{\"seq\":$n," "$out" 2>/dev/null | head -1 || true)"
    if [ -n "$result" ]; then
      echo "$result"
      case "$result" in
        *'"status":"done"'*) return 0 ;;
        *) return 1 ;;
      esac
    fi
    [ "$(date +%s)" -lt "$deadline" ] || { echo "{\"seq\":$n,\"status\":\"timeout\"}"; return 2; }
    sleep 0.1
  done
}

if [ -z "$file" ]; then
  case "$command" in
    *$'\n'*) echo "one command per call" >&2; exit 2 ;;
  esac
  send_one "$command"
  exit $?
fi

# The commands come in on fd 3, so nothing a command runs can read them.
# An include error stops the batch before anything is sent.
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
expanded="$("$here/dev-expand.sh" "$file")" || exit 2
exec 3<<<"$expanded"
refused=0
while IFS= read -r line <&3 || [ -n "$line" ]; do
  line="${line%$'\r'}"
  case "$line" in
    ''|'#'*) continue ;;
  esac
  # A line of only blanks is as empty as an empty one.
  [ -n "${line//[[:space:]]/}" ] || continue
  status=0
  send_one "$line" || status=$?
  case "$status" in
    0) ;;
    1) refused=1; [ "$keep_going" = 1 ] || exit 1 ;;
    *) exit "$status" ;;
  esac
done
exit "$refused"
