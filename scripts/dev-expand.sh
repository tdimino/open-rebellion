#!/usr/bin/env bash
# Expand `Include: PATH` lines in a developer-command script and print the
# result, so the game never sees an `Include:` line.
#
#   scripts/dev-expand.sh FILE|-
#
# PATH is relative to the including file's directory, else to the repository
# root. Includes nest up to 8 deep. A missing file or a cycle is an error that
# names the file and line (`FILE:LINE: ...`) and exits 2. Every other line,
# `#` comments included, is printed unchanged. FILE `-` reads standard input,
# whose includes resolve against the repository root.
# scripts/dev-send.sh --file and scripts/dev-run.sh both call this script.
set -euo pipefail

[ $# -eq 1 ] || { echo "usage: $0 FILE|-" >&2; exit 2; }
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
max_depth=8

# expand FILE LABEL DEPTH STACK: print FILE with its includes spliced in.
# STACK holds the absolute paths being expanded, one per line, for cycles.
expand() {
  local file="$1" label="$2" depth="$3" stack="$4" n=0 line path target base src="$1"
  base="$repo"; [ "$file" != "-" ] || src=/dev/stdin
  [ "$file" = "-" ] || base="$(cd "$(dirname "$file")" && pwd)"
  while IFS= read -r line || [ -n "$line" ]; do
    n=$((n + 1))
    line="${line%$'\r'}"
    case "$line" in
      Include:*)
        path="${line#Include:}"
        path="${path#"${path%%[![:space:]]*}"}"
        path="${path%"${path##*[![:space:]]}"}"
        [ -n "$path" ] || { echo "$label:$n: Include: needs a path" >&2; return 2; }
        if [ "$depth" -ge "$max_depth" ]; then
          echo "$label:$n: includes nest deeper than $max_depth ($path)" >&2
          return 2
        fi
        case "$path" in
          /*) target="$path" ;;
          *) target="$base/$path"; [ -f "$target" ] || target="$repo/$path" ;;
        esac
        [ -f "$target" ] || { echo "$label:$n: no include file $path" >&2; return 2; }
        target="$(cd "$(dirname "$target")" && pwd)/$(basename "$target")"
        case "$stack" in
          *"$target"$'\n'*) echo "$label:$n: include cycle through $path" >&2; return 2 ;;
        esac
        expand "$target" "$target" $((depth + 1)) "$stack$target"$'\n' || return $?
        ;;
      *) printf '%s\n' "$line" ;;
    esac
  done <"$src"
}

if [ "$1" = "-" ]; then
  expand - "<stdin>" 0 ""
else
  [ -f "$1" ] || { echo "no file $1" >&2; exit 2; }
  abs="$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"
  expand "$abs" "$1" 0 "$abs"$'\n'
fi
