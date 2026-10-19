#!/usr/bin/env bash
# Run a developer-command script against the game live on the Mac Mini, from
# this Mac, and pull the evidence back.
#
#   scripts/dev-run.sh EVIDENCE_DIR SCRIPT [--host H] [--timeout S] [--keep-going]
#
# SCRIPT is a local file with one developer command per line (blank lines and
# `#` lines are skipped; `Include: PATH` lines are expanded here first by
# scripts/dev-expand.sh, so the game never sees one). The script goes through ONE ssh into
# `scripts/dev-send.sh EVIDENCE_DIR --file -` in the Mini's repository copy,
# so each command waits for its result before the next, and the result lines
# print here. Afterwards the evidence folder (captures, logs, telemetry) is
# rsynced from the Mini to the same path here. rsync never deletes, and the
# pull runs even when the batch stopped early. Exits with dev-send's status
# (0 ran, 1 refused, 2 timeout or usage), or rsync's when the batch passed.
#
# The game must already be running with --live --evidence EVIDENCE_DIR
# (agent_docs/dev-commands.md). EVIDENCE_DIR is relative to the repository
# copy, like the launcher's.
#
# OPEN_REBELLION_BUILD_HOST  ssh host (default mac-mini-ts)
# OPEN_REBELLION_BUILD_DIR   repository copy on it (default or-native/open-rebellion)
set -euo pipefail

usage() {
  echo "usage: $0 EVIDENCE_DIR SCRIPT [--host H] [--timeout S] [--keep-going]" >&2
  exit 2
}

[ $# -ge 2 ] || usage
evidence="$1" script="$2"
shift 2
host="${OPEN_REBELLION_BUILD_HOST:-mac-mini-ts}"
dir="${OPEN_REBELLION_BUILD_DIR:-or-native/open-rebellion}"
send_args=()
while [ $# -gt 0 ]; do
  case "$1" in
    --host) [ $# -ge 2 ] || usage; host="$2"; shift ;;
    --timeout) [ $# -ge 2 ] || usage; send_args+=(--timeout "$2"); shift ;;
    --keep-going) send_args+=(--keep-going) ;;
    *) echo "unknown option: $1" >&2; exit 2 ;;
  esac
  shift
done
[ -f "$script" ] || { echo "no script $script" >&2; exit 2; }
case "$evidence" in
  /*|*..*) echo "EVIDENCE_DIR must be relative to the repository, without '..'" >&2; exit 2 ;;
esac

repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
q_evidence="$(printf '%q' "$evidence")"
q_dir="$(printf '%q' "$dir")"
q_args=""
[ ${#send_args[@]} -eq 0 ] || q_args="$(printf '%q ' "${send_args[@]}")"

expanded="$("$repo/scripts/dev-expand.sh" "$script")" || exit 2

status=0
ssh "$host" "cd $q_dir && scripts/dev-send.sh $q_evidence --file - $q_args" <<<"$expanded" || status=$?

mkdir -p "$repo/$evidence"
pull=0
rsync -a "$host:$q_dir/$q_evidence/" "$repo/$evidence/" || pull=$?
[ "$status" -ne 0 ] || status="$pull"
exit "$status"
