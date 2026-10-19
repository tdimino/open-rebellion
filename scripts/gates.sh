#!/usr/bin/env bash
# Run the pre-commit gates in order, one PASS/FAIL line each.
#
#   scripts/gates.sh [--keep-going] [--only NAME[,NAME]]
#
# Gates: test, replay, coverage, provenance, wasm, leak-worker, leak-private.
# Cargo runs on the Mini through scripts/mini-cargo.sh; every call but the
# last one run passes --keep, so the build cache survives between gates and
# the final call prunes it to that call's units. The golden is only run
# here, never regenerated.
#
# leak-private greps .beads/issues.jsonl for the name in $GATES_PRIVATE_NAME
# (kept out of the tracked script) and is skipped when the variable is unset.
#
# Stops at the first failure unless --keep-going. Exits 1 if any gate failed.
set -uo pipefail

repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo"
export CARGO_TERM_COLOR=never

all=(test replay coverage provenance wasm leak-worker leak-private)
keep_going=0
only=""
while [ $# -gt 0 ]; do
  case "$1" in
    --keep-going) keep_going=1 ;;
    --only) only="$2"; shift ;;
    *) echo "usage: $0 [--keep-going] [--only NAME[,NAME]]" >&2; exit 2 ;;
  esac
  shift
done

selected=()
for name in "${all[@]}"; do
  if [ -z "$only" ] || [[ ",$only," == *",$name,"* ]]; then selected+=("$name"); fi
done
[ ${#selected[@]} -gt 0 ] || { echo "no gate matches --only $only (gates: ${all[*]})" >&2; exit 2; }

mini=scripts/mini-cargo.sh

gate_test()       { "$mini" --keep -- test --workspace --color never; }
gate_replay()     { "$mini" --keep -- test -p rebellion-data --test replay_manifest --color never -- --ignored; }
gate_coverage()   { "$mini" --keep -- test -p rebellion-data --test telemetry_coverage --color never -- --ignored; }
gate_provenance() { "$mini" --keep -- run -q -p provenance-scan --color never -- check; }
gate_wasm()       { "$mini" --keep -- check --target wasm32-unknown-unknown -p rebellion-app --color never; }
gate_leak-worker() { [ "$(grep -c worker_ .beads/issues.jsonl)" = 0 ]; }
gate_leak-private() {
  [ -n "${GATES_PRIVATE_NAME:-}" ] || return 77
  [ "$(grep -ci -- "$GATES_PRIVATE_NAME" .beads/issues.jsonl)" = 0 ]
}

failed=0
for index in "${!selected[@]}"; do
  name="${selected[$index]}"
  # The last selected cargo gate is the one that prunes the cache.
  last=1
  for later in "${selected[@]:$((index + 1))}"; do
    case "$later" in test|replay|coverage|provenance|wasm) last=0 ;; esac
  done
  fn="gate_${name}"
  if [ "$last" = 1 ]; then
    case "$name" in
      test|replay|coverage|provenance|wasm)
        body="$(declare -f "$fn" | sed 's/ --keep//')"
        eval "$body" ;;
    esac
  fi
  "$fn"
  status=$?
  if [ "$status" = 77 ]; then
    echo "SKIP $name (GATES_PRIVATE_NAME unset)"
  elif [ "$status" = 0 ]; then
    echo "PASS $name"
  else
    echo "FAIL $name (exit $status)"
    failed=1
    [ "$keep_going" = 1 ] || break
  fi
done
exit "$failed"
