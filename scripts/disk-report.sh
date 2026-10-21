#!/usr/bin/env bash
# Show where the disk goes on this Mac and on the Mac Mini: free space, the
# project's own folders, then the largest folders in the home directory.
#
#   scripts/disk-report.sh            both machines
#   scripts/disk-report.sh --mac      this Mac only
#   scripts/disk-report.sh --mini     the Mini only
#   scripts/disk-report.sh --local    the Mini's report, run on the Mini itself
#
# On the Mini a private clone (or-native/wt-*, pw-*, ...) is an APFS clone of
# the shared copy, so du counts the blocks it shares; the column that counts
# is its target/, which each clone builds on its own. scripts/mini-clean.sh
# removes idle clones and old run output.
#
# OPEN_REBELLION_BUILD_HOST   the Mini (default mac-mini-ts)
set -euo pipefail

host="${OPEN_REBELLION_BUILD_HOST:-mac-mini-ts}"
script="${BASH_SOURCE[0]:-}"
where=both
case "${1:-}" in
  --mac) where=mac ;;
  --mini) where=mini ;;
  --local) where=local ;;
  "") ;;
  *) echo "usage: $0 [--mac|--mini|--local]" >&2; exit 2 ;;
esac

# The N largest of the given paths, in GB with one decimal. awk applies the
# limit and reads everything, so no early pipe close trips pipefail.
sizes() {
  local limit=$1 p
  shift
  for p in "$@"; do
    if [[ -e $p ]]; then du -sk "$p" 2>/dev/null || true; fi
  done | sort -rn | awk -v n="$limit" 'NR <= n { kb = $1; $1 = ""; sub(/^ /, ""); printf "  %7.1f GB  %s\n", kb / 1048576, $0 }'
}

age_days() {
  local newest
  newest=$(find "$1" -type f -mtime -30 -print0 2>/dev/null | xargs -0 stat -f %m 2>/dev/null | sort -rn | head -1)
  [[ -n $newest ]] && echo $(( ($(date +%s) - newest) / 86400 )) || echo ">30"
}

report() {
  local machine=$1 repo=$2
  echo "== $machine ($(hostname -s)) =="
  df -h / | awk 'NR == 2 { printf "free %s of %s\n", $4, $2 }'
  if [[ $machine == Mini ]]; then
    local base="$HOME/or-native" shared="$HOME/or-native/open-rebellion" d
    echo "-- project: the shared copy"
    sizes 99 "$shared/target" "$shared/assets/references" "$shared/.artifacts" "$shared/web/data" "$shared/data"
    echo "-- project: private clones (target/ is each clone's own; idle = days since a write)"
    for d in "$base"/*/; do
      d=${d%/}
      [[ -f $d/Cargo.toml && $d != "$shared" ]] || continue
      printf "  %7.1f GB  %-14s idle %s d\n" \
        "$(du -sk "$d/target" 2>/dev/null | awk '{ print $1 / 1048576 }' || echo 0)" "$(basename "$d")" "$(age_days "$d")"
    done
    echo "-- project: run output and other folders"
    sizes 12 "$base"/mutants* "$base"/mut-* "$base"/evidence "$HOME/open-rebellion-archive" \
      "$HOME/open-rebellion-data" "$HOME/or-wine" "$HOME/VMs" \
      "$HOME/Library/Containers/com.utmapp.UTM" "$HOME/Library/Caches/ms-playwright"
    echo "-- caches"
    sizes 8 "$HOME/Library/Caches"/* "$HOME/.cache"/* "$HOME/.cargo/registry" "$HOME/.rustup" "$HOME/.npm"
  else
    echo "-- project: the repository"
    sizes 99 "$repo/target" "$repo/.artifacts" "$repo/assets/references" "$repo/data" "$repo/ghidra" "$repo/web/data" "$repo/dist"
    echo "-- project: .artifacts by folder"
    sizes 10 "$repo/.artifacts"/*
    echo "-- caches"
    sizes 8 "$HOME/Library/Caches"/* "$HOME/.cache"/* "$HOME/.cargo/registry" "$HOME/.rustup" "$HOME/.npm"
  fi
  echo "-- home, largest folders"
  sizes 12 "$HOME"/* "$HOME"/.[!.]* "$HOME/Library"/*
  echo
}

case $where in
  local) report Mini "" ;;
  mac) report Mac "$(cd "$(dirname "$script")/.." && pwd)" ;;
  mini) ssh "$host" "bash -c $(printf %q "$(cat "$script")") disk-report --local" ;;
  both) report Mac "$(cd "$(dirname "$script")/.." && pwd)"; ssh "$host" "bash -c $(printf %q "$(cat "$script")") disk-report --local" ;;
esac
