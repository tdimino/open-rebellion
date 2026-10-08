#!/usr/bin/env bash
# Report the disk the build cache and run evidence hold, and warn when
# target/ passes its limit. Cargo never prunes target/: every change to a
# crate's code, flags or features writes a new hashed copy of its outputs
# and test binaries, and the old copies stay.
#
#   scripts/check-build-cache.sh            report; warn over the limit
#   scripts/check-build-cache.sh --prune    also cargo clean when over it,
#                                           and remove stale cargo-mutants
#                                           scratch copies
#
# OPEN_REBELLION_TARGET_LIMIT_GB sets the limit (default 20). Run --prune
# only when no build, test or mutants run is in flight.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
limit_gb="${OPEN_REBELLION_TARGET_LIMIT_GB:-20}"
prune=false
[[ "${1:-}" == "--prune" ]] && prune=true

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

echo "build cache: target/ ${target_gb} GB (limit ${limit_gb} GB), .artifacts/ ${artifacts_gb} GB, cargo-mutants scratch copies ${#stale_mutants[@]}"

if (( target_gb >= limit_gb )); then
  if $prune; then
    env PATH="/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:$HOME/.cargo/bin" \
      cargo clean --manifest-path "$root/Cargo.toml"
  else
    echo "WARNING: target/ is over ${limit_gb} GB; run scripts/check-build-cache.sh --prune when no build is running"
  fi
fi

if (( ${#stale_mutants[@]} > 0 )); then
  if $prune; then
    rm -rf "${stale_mutants[@]}"
  else
    echo "WARNING: ${#stale_mutants[@]} cargo-mutants scratch copies remain in $tmp; --prune removes them once no mutants run is in flight"
  fi
fi
