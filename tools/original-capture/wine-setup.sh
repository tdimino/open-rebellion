#!/usr/bin/env bash
# wine-setup.sh — Install and configure Wine on the Mac Mini for running the
# original REBEXE.EXE.  Everything lives under ~/or-wine/.
#
# The game is 32-bit.  Upstream macOS Wine sets up 32-bit code segments through
# the LDT, which Rosetta refuses ("rosetta error: LDT … not supported"), and
# GPTK 1.1 (Wine 7.7) aborts in alloc_pages_vprot.  The Sikarugir engine is a
# WoW64 build (i386-windows PE, x86_64-unix only) that runs 32-bit clients
# under Rosetta, as the Linux recipe's amd64-wow64 build does.
#
# Prerequisites:
#   - Rosetta 2 (softwareupdate --install-rosetta --agree-to-license)
#   - Game files at ~/or-native/star-wars-rebellion/
#
# Pinned versions and hashes:
#   Wine:      WS12WineSikarugir10.0_6, Sikarugir-App/Engines v1.0
#              (SHA-256 9da7ee0cbf386522f3a9906943726d9c3c125dbbd9ab120e3cde80e88d6091b2)
#   cnc-ddraw: 7.1.0.0    (SHA-256 0b13ab89a64c9918189b1dadd449ef6ed3cb3b7b19cabd96d8adbd95505bb908)
#   REBEXE:    b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab
#   d3drm.dll: e04e3781e7ff3466723565da8413b08b705533acda257773874f2f8166b3d4d6

set -euo pipefail

base="$HOME/or-wine"
game_source="$HOME/or-native/star-wars-rebellion"
engine="WS12WineSikarugir10.0_6"
engine_sha="9da7ee0cbf386522f3a9906943726d9c3c125dbbd9ab120e3cde80e88d6091b2"
engine_dir="$base/sikarugir-10.0_6"
# The engine links its dylibs (libinotify, freetype, gnutls …) from the
# wrapper template's Frameworks, found through DYLD_FALLBACK_LIBRARY_PATH.
# macOS strips DYLD_* from every SIP-protected program (sh, bash, nohup,
# env), so an exported value does not reach Wine through them: the wine
# and wineserver wrappers in bin/ set it themselves.
template="Template-1.0.11"
template_sha="9fa15479e7ff6abd99c1d07be285fb95f41fc6991586502427152b1f7d6ccb8a"
template_dir="$base/sikarugir-template-1.0.11"
frameworks="$template_dir/$template.app/Contents/Frameworks"
engine_bin="$engine_dir/wswine.bundle/bin"
wine_bin="$base/bin/wine"
wineserver="$base/bin/wineserver"
prefix="$base/prefix-sik"
game_dir="$prefix/drive_c/Rebellion"
cnc_version="7.1.0.0"

# ---- checks ----

if ! arch -x86_64 /usr/bin/true 2>/dev/null; then
    echo 'ERROR: Rosetta 2 is not installed.' >&2
    echo 'Run:  softwareupdate --install-rosetta --agree-to-license' >&2
    exit 1
fi

if [[ ! -f "$game_source/REBEXE.EXE" ]]; then
    echo "ERROR: Game files not found at $game_source" >&2
    exit 1
fi

# Verify exe hash.
exe_hash=$(shasum -a 256 "$game_source/REBEXE.EXE" | cut -d' ' -f1)
if [[ "$exe_hash" != "b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab" ]]; then
    echo "ERROR: REBEXE.EXE hash mismatch: $exe_hash" >&2
    exit 1
fi

mkdir -p "$base"/{downloads,logs,tmp,bin,run}

# ---- GUI-domain runner ----
#
# Wine needs the GUI (Aqua) session, which SSH does not have.  The launch
# agent com.openrebellion.wine runs wine-run.sh in that session whenever
# run/q changes; wine-run.sh runs each queued ID.cmd, oldest first, and
# leaves ID.out and ID.rc in run/done.  wine-queue.sh CMD queues one
# command, waits, prints its output and exits with its code.  Removing a
# queued ID.cmd before the runner takes it cancels the command.
# AbandonProcessGroup keeps a game started by a command alive after the
# command returns.

runner="$base/bin/wine-run.sh"
queue="$base/bin/wine-queue.sh"
plist="$base/com.openrebellion.wine.plist"
mkdir -p "$base/run/q" "$base/run/done"
# Write to a temporary file and rename it, so a runner executing the old
# copy (this script's caller, when queued) keeps reading an intact file.
cat > "$runner.tmp" <<'RUN'
#!/bin/sh
# wine-run.sh — Run each queued ~/or-wine/run/q/ID.cmd in the GUI session,
# oldest first, with its output in run/done/ID.out and its exit code in
# run/done/ID.rc.  Installed by wine-setup.sh.
base="$HOME/or-wine"
q="$base/run/q"
done_dir="$base/run/done"
# Results whose waiter gave up while the command ran are never read.
find "$done_dir" -type f -mmin +1440 -delete 2>/dev/null
while :; do
    next=$(ls "$q"/*.cmd 2>/dev/null | head -1)
    [ -n "$next" ] || exit 0
    id=$(basename "$next" .cmd)
    mv "$next" "$done_dir/$id.run" || continue
    rc=0
    # Setup's first run, queued from SSH, comes before env.sh exists.
    ( [ -f "$base/env.sh" ] && . "$base/env.sh"; eval "$(cat "$done_dir/$id.run")" ) \
        > "$done_dir/$id.out" 2>&1 || rc=$?
    # ID.rc lands before ID.run goes, so a waiter always sees one of
    # ID.cmd, ID.run or ID.rc until it has its result.
    echo "$rc" > "$done_dir/$id.rc.tmp"
    mv -f "$done_dir/$id.rc.tmp" "$done_dir/$id.rc"
    rm -f "$done_dir/$id.run"
done
RUN
cat > "$queue.tmp" <<'QUEUE'
#!/bin/sh
# wine-queue.sh CMD — Run the shell command line CMD in the GUI session
# through wine-run.sh; print its output and exit with its code (124 after
# WINE_QUEUE_LIMIT seconds, default 3600; 130 when the command was
# cancelled).  WINE_QUEUE_ID names the command, so a caller can cancel it
# by removing run/q/ID.cmd.  With WINE_QUEUE_EOF=cancel the caller holds
# stdin open while it waits (wine-game.sh over SSH): end of file means the
# caller is gone, a dropped connection included, and the waiter cancels the
# command and exits 130.  Installed by wine-setup.sh.
base="$HOME/or-wine"
q="$base/run/q"
done_dir="$base/run/done"
id="${WINE_QUEUE_ID:-$(date +%s).$$}"
case $id in
    ''|*[!0-9.]*) echo "wine-queue: WINE_QUEUE_ID must be digits and dots: $id" >&2; exit 2 ;;
esac
gone="$done_dir/$id.gone"
reader=
# A command given up on (a timeout, a signal, a caller gone) must never run
# later, and its result files go with the waiter.
trap '[ -z "$reader" ] || kill "$reader" 2>/dev/null; rm -f "$q/$id.tmp" "$q/$id.cmd" "$done_dir/$id.out" "$done_dir/$id.rc" "$gone"' EXIT
trap 'exit 129' HUP
trap 'exit 130' INT
trap 'exit 141' PIPE
trap 'exit 143' TERM
if [ "${WINE_QUEUE_EOF:-}" = cancel ]; then
    # An asynchronous list reads /dev/null as stdin, so the reader takes
    # the caller's stdin through fd 3.  It is a builtin loop, so killing
    # the subshell ends the read.
    exec 3<&0
    ( while IFS= read -r line; do :; done <&3; : > "$gone" ) > /dev/null 2>&1 &
    reader=$!
fi
printf '%s\n' "$*" > "$q/$id.tmp"
mv -f "$q/$id.tmp" "$q/$id.cmd"
start=$(date +%s)
while [ ! -f "$done_dir/$id.rc" ]; do
    sleep 0.5
    if [ $(( $(date +%s) - start )) -ge "${WINE_QUEUE_LIMIT:-3600}" ]; then
        echo "wine-queue: no result for $id after ${WINE_QUEUE_LIMIT:-3600} s" >&2
        exit 124
    fi
    if [ ! -f "$q/$id.cmd" ] && [ ! -f "$done_dir/$id.run" ] && [ ! -f "$done_dir/$id.rc" ]; then
        echo "wine-queue: $id was cancelled before it ran" >&2
        exit 130
    fi
    # The EXIT trap takes a still-queued command with it; one already
    # running finishes, and its unread result ages out of run/done.
    if [ -f "$gone" ]; then
        echo "wine-queue: the caller of $id is gone; cancelled" >&2
        exit 130
    fi
    # launchd can miss a command queued while the runner is exiting: a
    # change to run/q starts it again.
    if [ -f "$q/$id.cmd" ] && [ $(( ($(date +%s) - start) % 5 )) -eq 0 ]; then
        : > "$q/.nudge.$id"
        rm -f "$q/.nudge.$id"
    fi
done
cat "$done_dir/$id.out"
rc=$(cat "$done_dir/$id.rc")
rm -f "$done_dir/$id.out" "$done_dir/$id.rc"
exit "$rc"
QUEUE
chmod +x "$runner.tmp" "$queue.tmp"
mv -f "$runner.tmp" "$runner"
mv -f "$queue.tmp" "$queue"

cat > "$plist.tmp" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>Label</key>
	<string>com.openrebellion.wine</string>
	<key>ProgramArguments</key>
	<array>
		<string>$runner</string>
	</array>
	<key>WatchPaths</key>
	<array>
		<string>$base/run/q</string>
	</array>
	<key>ThrottleInterval</key>
	<integer>1</integer>
	<key>AbandonProcessGroup</key>
	<true/>
	<key>StandardErrorPath</key>
	<string>$base/logs/wine-agent.log</string>
	<key>StandardOutPath</key>
	<string>$base/logs/wine-agent.log</string>
</dict>
</plist>
PLIST
domain="gui/$(id -u)"
job="$domain/com.openrebellion.wine"
if ! cmp -s "$plist.tmp" "$plist" || ! launchctl print "$job" > /dev/null 2>&1; then
    # Never reload from inside the agent: bootout would end this script.
    [[ ${XPC_SERVICE_NAME:-} != com.openrebellion.wine ]] \
        || { echo 'ERROR: the launch agent changed; run setup over SSH or in Terminal.' >&2; exit 1; }
    mv -f "$plist.tmp" "$plist"
    launchctl bootout "$job" 2> /dev/null || true
    launchctl bootstrap "$domain" "$plist"
else
    rm -f "$plist.tmp"
fi

if [[ "$(launchctl managername)" != Aqua ]]; then
    # Over SSH: run the rest of setup in the GUI session and relay it.
    script="$(cd -- "$(dirname -- "$0")" && pwd)/$(basename -- "$0")"
    echo 'Running setup in the GUI session...'
    WINE_QUEUE_LIMIT=600 exec "$queue" "bash $(printf '%q' "$script")"
fi

# ---- Wine engine ----

engine_archive="$base/downloads/$engine.tar.xz"
if [[ ! -f "$engine_archive" ]]; then
    echo "Downloading $engine..."
    curl --proto '=https' --tlsv1.2 -fL \
        "https://github.com/Sikarugir-App/Engines/releases/download/v1.0/$engine.tar.xz" \
        -o "$engine_archive.part"
    mv "$engine_archive.part" "$engine_archive"
fi
actual=$(shasum -a 256 "$engine_archive" | cut -d' ' -f1)
if [[ "$actual" != "$engine_sha" ]]; then
    echo "ERROR: $engine hash mismatch: $actual" >&2
    exit 1
fi
if [[ ! -x "$engine_bin/wine" ]]; then
    mkdir -p "$engine_dir"
    tar -xJf "$engine_archive" -C "$engine_dir"
fi

template_archive="$base/downloads/$template.tar.xz"
if [[ ! -f "$template_archive" ]]; then
    echo "Downloading $template..."
    curl --proto '=https' --tlsv1.2 -fL \
        "https://github.com/Sikarugir-App/Template/releases/download/v1.0/$template.tar.xz" \
        -o "$template_archive.part"
    mv "$template_archive.part" "$template_archive"
fi
actual=$(shasum -a 256 "$template_archive" | cut -d' ' -f1)
if [[ "$actual" != "$template_sha" ]]; then
    echo "ERROR: $template hash mismatch: $actual" >&2
    exit 1
fi
if [[ ! -f "$frameworks/libinotify.0.dylib" ]]; then
    mkdir -p "$template_dir"
    tar -xJf "$template_archive" -C "$template_dir"
fi
for tool in wine wineserver; do
    cat > "$base/bin/$tool.tmp" <<WRAP
#!/bin/sh
# $tool — the engine's $tool with its Frameworks on the library path.
# Installed by wine-setup.sh.
export DYLD_FALLBACK_LIBRARY_PATH='$frameworks:/usr/lib'
exec '$engine_bin/$tool' "\$@"
WRAP
    chmod +x "$base/bin/$tool.tmp"
    mv -f "$base/bin/$tool.tmp" "$base/bin/$tool"
done

# ---- cnc-ddraw (verify, or download) ----

cnc_archive="$base/downloads/cnc-ddraw-v$cnc_version.zip"
cnc_dir="$base/cnc-ddraw-v$cnc_version"
if [[ ! -f "$cnc_archive" ]]; then
    echo "Downloading cnc-ddraw $cnc_version..."
    curl --proto '=https' --tlsv1.2 -fL \
        "https://github.com/FunkyFr3sh/cnc-ddraw/releases/download/v$cnc_version/cnc-ddraw.zip" \
        -o "$cnc_archive.part"
    mv "$cnc_archive.part" "$cnc_archive"
fi
expected="0b13ab89a64c9918189b1dadd449ef6ed3cb3b7b19cabd96d8adbd95505bb908"
actual=$(shasum -a 256 "$cnc_archive" | cut -d' ' -f1)
if [[ "$actual" != "$expected" ]]; then
    echo "ERROR: cnc-ddraw hash mismatch: $actual" >&2
    exit 1
fi
if [[ ! -f "$cnc_dir/ddraw.dll" ]]; then
    python3 -c "
import zipfile, pathlib, sys
with zipfile.ZipFile(sys.argv[1]) as z:
    for n in z.namelist():
        if n.startswith('/') or '..' in pathlib.PurePosixPath(n).parts:
            raise ValueError('Unsafe')
    z.extractall(sys.argv[2])
" "$cnc_archive" "$cnc_dir"
fi

# ---- Wine prefix ----

export WINEPREFIX="$prefix"
# A 64-bit prefix; the engine's WoW64 runs the 32-bit game inside it.
export WINEARCH=win64
# Suppress Wine debug noise and menu-builder popups.
export WINEDEBUG=-all
export WINEDLLOVERRIDES='ddraw=n;d3drm=n;winemenubuilder.exe=d;mscoree,mshtml='

if [[ ! -d "$prefix/drive_c/windows" ]]; then
    echo "Creating Wine prefix at $prefix ..."
    "$wine_bin" wineboot -u > "$base/logs/wineboot.log" 2>&1
    "$wineserver" -w
    echo "Prefix created."
fi

# shell32 links the profile folders (Desktop, Documents, Downloads …) to the
# home folders, so anything reading through them touches ~/Desktop and macOS
# asks whether wine may access it (shell32/shellpath.c).  It links only a
# folder that does not exist, so a real directory in each place stays, as
# winetricks isolate_home does.
for link in "$prefix/drive_c/users/$USER/"*; do
    [[ -L $link ]] || continue
    case $(readlink "$link") in
        "$HOME"/*) rm -f "$link"; mkdir "$link" ;;
    esac
done

# ---- game directory overlay ----

mkdir -p "$game_dir/SaveGame"

for source in "$game_source/"*; do
    name=${source##*/}
    case "$name" in
        app|tmp|SaveGame|ddraw.dll|d3drm.dll|__support|__redist|commonappdata) continue ;;
    esac
    if [[ -d "$source" ]]; then
        [[ -e "$game_dir/$name" ]] || ln -sfn "$source" "$game_dir/$name"
    else
        [[ -e "$game_dir/$name" && ! -L "$game_dir/$name" ]] || ln -sfn "$source" "$game_dir/$name"
    fi
done

# Native d3drm.dll from GOG support files.
cp -f "$game_source/__support/add/d3drm.dll" "$game_dir/d3drm.dll"

# cnc-ddraw: GDI renderer, windowed, 30 fps cap, same as the Linux recipe.
cp -f "$cnc_dir/ddraw.dll" "$game_dir/ddraw.dll"
python3 -c "
import sys
ini = open(sys.argv[1]).read()
ini = ini.replace('renderer=auto', 'renderer=gdi')
ini = ini.replace('windowed=false', 'windowed=true')
ini = ini.replace('maxfps=-1', 'maxfps=30')
# devmode: without it cnc-ddraw drops mouse input until it locks the
# cursor, which injected input never does (wndproc.c, v7.1.0.0).
if '[REBEXE]\n' not in ini:
    sys.exit('ddraw.ini has no [REBEXE] section')
ini = ini.replace('[REBEXE]\n', '[REBEXE]\ndevmode=true\n', 1)
with open(sys.argv[2], 'w') as f:
    f.write(ini)
" "$cnc_dir/ddraw.ini" "$game_dir/ddraw.ini"

# ---- drive.exe helper ----

# Built from wine-drive.c beside this script, with the first 16 hex digits
# of the source's SHA-256 as drive-cap.dll's build ID (wine-drive.c,
# same_module).  drive.exe loads
# drive-cap.dll from its own folder into the game.  Copy and rename, so a
# copy the running game has mapped is never rewritten.  drive64.exe, the
# 64-bit build of the same source, moves the mouse without warping the Mac
# pointer (wine-drive.c, "input without warping").
# launchd and SSH sessions leave Homebrew off PATH.
mingw=$(PATH="$PATH:/opt/homebrew/bin:/usr/local/bin" command -v i686-w64-mingw32-gcc || true)
mingw64=$(PATH="$PATH:/opt/homebrew/bin:/usr/local/bin" command -v x86_64-w64-mingw32-gcc || true)
[[ -n $mingw && -n $mingw64 ]] \
    || { echo 'ERROR: i686- or x86_64-w64-mingw32-gcc not found (brew install mingw-w64).' >&2; exit 1; }
drive_src="$(cd -- "$(dirname -- "$0")" && pwd)/wine-drive.c"
build_id=$(shasum -a 256 "$drive_src" | cut -c1-16)
"$mingw" -O2 -Wall -o "$base/bin/drive.exe" "$drive_src" -lgdi32 -luser32
"$mingw" -O2 -Wall -shared -DWINE_CAP_DLL -DDRIVE_BUILD="\"$build_id\"" -Wl,--kill-at \
    -o "$base/bin/drive-cap.dll" "$drive_src" -lgdi32 -luser32
"$mingw64" -O2 -Wall -o "$base/bin/drive64.exe" "$drive_src" -lgdi32 -luser32
for helper in drive.exe drive-cap.dll drive64.exe; do
    cp -f "$base/bin/$helper" "$game_dir/$helper.tmp"
    mv -f "$game_dir/$helper.tmp" "$game_dir/$helper"
done

# ---- registry ----

reg() { "$wine_bin" reg "$@" >> "$base/logs/registry.log" 2>&1; }
key='HKLM\Software\LucasArts Entertainment Company LLC\Star Wars Rebellion'

for name in InstalledLocation DataLocation DllLocation TooBigLocation; do
    reg add "$key" /v "$name" /t REG_SZ /d 'C:\Rebellion' /f /reg:32
done
reg add "$key" /v EXEPath /t REG_SZ /d 'C:\Rebellion\REBEXE.EXE' /f /reg:32
reg add "$key" /v EncyLocation /t REG_SZ /d 'C:\Rebellion\EDATA' /f /reg:32

# Windows 98 version override for the exe (matches Linux recipe).
reg add 'HKCU\Software\Wine\AppDefaults\REBEXE.EXE' /v Version /t REG_SZ /d win98 /f

# Tell Wine to use native ddraw and d3drm (loaded from the game directory).
reg add 'HKCU\Software\Wine\DllOverrides' /v ddraw /t REG_SZ /d native /f
reg add 'HKCU\Software\Wine\DllOverrides' /v d3drm /t REG_SZ /d native /f

# Flush the registry to the prefix, unless a game is running: the game
# would go with the server.  It reads the new values when next started.
if pgrep -x REBEXE.EXE > /dev/null; then
    echo 'The game is running; restart it to load the new helpers and settings.'
else
    "$wineserver" -k 2>/dev/null || true
    "$wineserver" -w 2>/dev/null || true
fi

# ---- env file ----

cat > "$base/env.sh" <<ENV
export WINEPREFIX='$prefix'
export WINEARCH=win64
export WINEDEBUG=-all
export WINEDLLOVERRIDES='ddraw=n;d3drm=n;winemenubuilder.exe=d;mscoree,mshtml='
export WINE_GAME_DIR='$game_dir'
export WINE_BIN='$wine_bin'
export WINE_SERVER='$wineserver'
export WINE_BASE='$base'
ENV

echo "Ready.  Source $base/env.sh; launch with wine-game.sh up"
echo ""
echo "Versions:"
echo "  Wine:        $("$wine_bin" --version)"
echo "  cnc-ddraw:   $cnc_version"
echo "  REBEXE.EXE:  $exe_hash"
