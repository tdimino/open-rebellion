#!/usr/bin/env bash
# Send one developer command to a game launched with --live, and print its
# result.
#
#   scripts/dev-send.sh DIR "<command>" [--timeout SECONDS]
#
# DIR is the launcher's --evidence folder. The command is appended to
# DIR/commands.in. The script waits for the line's result in
# DIR/commands.out and prints it as one JSON line. It exits 0 when the
# command ran, 1 when it was refused or skipped, and 2 on a timeout or bad
# usage. agent_docs/dev-commands.md lists the commands.
set -euo pipefail

[ $# -ge 2 ] || { echo "usage: $0 DIR \"<command>\" [--timeout S]" >&2; exit 2; }
dir="$1" command="$2" timeout=30
shift 2
while [ $# -gt 0 ]; do
  case "$1" in
    --timeout) timeout="$2"; shift ;;
    *) echo "unknown option: $1" >&2; exit 2 ;;
  esac
  shift
done
inbox="$dir/commands.in" out="$dir/commands.out"
[ -f "$inbox" ] || { echo "no $inbox; launch with --live --evidence $dir" >&2; exit 2; }
case "$command" in
  *$'\n'*) echo "one command per call" >&2; exit 2 ;;
esac

# mkdir is atomic: the lock keeps two senders' lines and numbers apart.
lock="$dir/.commands.lock"
for _ in $(seq 1 100); do mkdir "$lock" 2>/dev/null && break; sleep 0.05; done
[ -d "$lock" ] || { echo "cannot take $lock" >&2; exit 2; }
trap 'rmdir "$lock" 2>/dev/null || true' EXIT
printf '%s\n' "$command" >>"$inbox"
seq="$(wc -l <"$inbox" | tr -d ' ')"
rmdir "$lock"
trap - EXIT

deadline=$(( $(date +%s) + timeout ))
while :; do
  result="$(grep -F "{\"seq\":$seq," "$out" 2>/dev/null | head -1 || true)"
  if [ -n "$result" ]; then
    echo "$result"
    case "$result" in
      *'"status":"done"'*) exit 0 ;;
      *) exit 1 ;;
    esac
  fi
  [ "$(date +%s)" -lt "$deadline" ] || { echo "{\"seq\":$seq,\"status\":\"timeout\"}"; exit 2; }
  sleep 0.1
done
