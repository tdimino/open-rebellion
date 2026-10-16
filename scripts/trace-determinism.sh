#!/usr/bin/env bash
# Does recording traces change the simulation?
#
#   scripts/trace-determinism.sh [--seed N] [--ticks N]
#
# Runs the release playtest on the Mini twice with --dual-ai, once with
# --trace all and once without, each writing a save with --save-out, and
# compares the save fingerprints the playtest prints. Prints MATCH or
# DIFFER; exits 1 on DIFFER or a failed run. The release binary must be
# built already (scripts/mini-cargo.sh --keep -- build --release -p
# rebellion-playtest). Outputs go under /tmp on the Mini and are deleted.
#
# OPEN_REBELLION_BUILD_HOST  ssh host (default mac-mini-ts)
# OPEN_REBELLION_BUILD_DIR   repository copy on it (default or-native/open-rebellion)
set -euo pipefail

host="${OPEN_REBELLION_BUILD_HOST:-mac-mini-ts}"
dir="${OPEN_REBELLION_BUILD_DIR:-or-native/open-rebellion}"
seed=42
ticks=1000
while [ $# -gt 0 ]; do
  case "$1" in
    --seed) seed="$2"; shift ;;
    --ticks) ticks="$2"; shift ;;
    *) echo "usage: $0 [--seed N] [--ticks N]" >&2; exit 2 ;;
  esac
  shift
done

q_dir="$(printf %q "$dir")"
work="/tmp/trace-determinism-$$"
# Each run's stderr carries "Saved day N to PATH (fingerprint F)".
remote="cd $q_dir && mkdir -p $work && \
  target/release/rebellion-playtest data/base --seed $seed --ticks $ticks --dual-ai --trace all \
    --save-out $work/traced.reb >/dev/null 2>$work/traced.log; traced=\$?; \
  target/release/rebellion-playtest data/base --seed $seed --ticks $ticks --dual-ai \
    --save-out $work/plain.reb >/dev/null 2>$work/plain.log; plain=\$?; \
  echo traced \$traced; echo plain \$plain; \
  echo traced-fp \$(sed -n 's/.*fingerprint \\([^)]*\\)).*/\\1/p' $work/traced.log); \
  echo plain-fp \$(sed -n 's/.*fingerprint \\([^)]*\\)).*/\\1/p' $work/plain.log); \
  rm -rf $work"
out="$(ssh "$host" "$remote")"
echo "$out"

get() { printf '%s\n' "$out" | awk -v k="$1" '$1 == k { print $2 }'; }
traced_fp="$(get traced-fp)"
plain_fp="$(get plain-fp)"
if [ "$(get traced)" != 0 ] || [ "$(get plain)" != 0 ] || [ -z "$traced_fp" ] || [ -z "$plain_fp" ]; then
  echo "FAILED: a run exited non-zero or printed no fingerprint" >&2
  exit 1
fi
if [ "$traced_fp" = "$plain_fp" ]; then
  echo "MATCH seed=$seed ticks=$ticks fingerprint=$traced_fp"
else
  echo "DIFFER seed=$seed ticks=$ticks traced=$traced_fp plain=$plain_fp"
  exit 1
fi
