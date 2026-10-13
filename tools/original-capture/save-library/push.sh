#!/usr/bin/env bash
# Copy the manifest's saves into C:\Rebellion\SaveLibrary on the guest via
# spare slot 10, then write INDEX.txt there. Slot 10 is cleared afterwards.
set -euo pipefail
V=/Users/tomdimino/Desktop/Programming/open-rebellion/tools/original-capture/vm-game.sh
ROOT=/Users/tomdimino/Desktop/Programming/open-rebellion
D=$(cd "$(dirname "$0")" && pwd)
LIB='C:\Rebellion\SaveLibrary'
$V gexec "if not exist $LIB mkdir $LIB" >/dev/null
{
  printf 'Original-game save library (Star Wars Rebellion, GOG build, this VM)\r\n'
  printf 'To use one: copy it over a slot, e.g.\r\n'
  printf '  copy /y C:\\Rebellion\\SaveLibrary\\NAME.sav C:\\Rebellion\\SaveGame\\SAVEGAME.007\r\n'
  printf 'then F1, Saved Games. Saved Games lists filled slots plus one empty row,\r\n'
  printf 'so fill slots without gaps. Slot 5 holds empire-day488-late: leave it.\r\n'
  printf 'Mac side: tools/original-capture/vm-game.sh save-in FILE SLOT does the same.\r\n\r\n'
} > "$D/INDEX.txt"
while IFS=$'\t' read -r -u 3 src name desc; do
  path=$ROOT/$src
  sum=$(shasum -a 256 "$path" | cut -c1-64)
  $V save-in "$path" 10 >/dev/null
  $V gexec "copy /y C:\\Rebellion\\SaveGame\\SAVEGAME.010 $LIB\\$name" >/dev/null
  guest=$($V gexec "certutil -hashfile $LIB\\$name SHA256" | sed -n 3p | tr -d ' \r')
  [[ $guest == "$sum" ]] || { echo "hash mismatch: $name" >&2; exit 1; }
  printf '%s\r\n  SHA-256 %s\r\n  %s\r\n\r\n' "$name" "$sum" "$desc" >> "$D/INDEX.txt"
  echo "ok $name"
done 3< "$D/manifest.tsv"
$V save-in "$D/INDEX.txt" 10 >/dev/null
$V gexec "copy /y C:\\Rebellion\\SaveGame\\SAVEGAME.010 $LIB\\INDEX.txt" >/dev/null
$V gexec "del /q C:\\Rebellion\\SaveGame\\SAVEGAME.010" >/dev/null
$V gexec "dir /b $LIB"
