#!/usr/bin/env bash
# wine-game.sh — Drive the original REBEXE.EXE under Wine (Sikarugir WoW64) on macOS.
#
# Mirrors vm-game.sh verbs where they make sense:
#   wine-game.sh up              Start Wine + the game, past the intros
#   wine-game.sh down            Stop Wine and the game
#   wine-game.sh status          Print pid/session info (exit 1 if stopped)
#   wine-game.sh click X Y       Left-click at 800x600 coordinates
#   wine-game.sh rclick X Y      Right-click
#   wine-game.sh dclick X Y      Double-click
#   wine-game.sh drag X1 Y1 X2 Y2  Drag from (X1,Y1) to (X2,Y2)
#   wine-game.sh key NAME [N]    Send a keystroke N times (esc, enter, f1, alt+f4, …)
#   wine-game.sh type TEXT       Type text into the focused field
#   wine-game.sh save SLOT NAME  Save in game to slot 1-6 under NAME
#   wine-game.sh cap FILE        Screenshot to FILE.png (the 640x480 game client)
#   wine-game.sh load SLOT [menu]  Load a save in game (F1), or from the main menu
#   wine-game.sh save-in FILE SLOT  Copy a save file into a slot (1-10)
#   wine-game.sh save-out SLOT NAME  Copy a save slot to NAME.sav (a bare NAME
#                                goes to ~/or-wine/out on the Mini)
#   wine-game.sh day [SIDE] [FILE]  Capture the day readout (alliance or empire)
#   wine-game.sh clock           Print day=N steps=M sched=ADDR from game memory
#   wine-game.sh peek PTR OFF [N]  Print the dword at *PTR + OFF in game memory, or
#                                a histogram of N reads 1 ms apart
#   wine-game.sh timescale K     Run the game's clock K times faster (1-64; 1 = real)
#   wine-game.sh stop-at DAY     Hold the game at the first step of DAY, which must
#                                be after the current day (0 releases the hold).
#                                While held the frame is frozen and the game acts
#                                on no input until the release; messages and clock
#                                work
#   wine-game.sh run-to-day DAY [LIMIT]
#                                Play on (at the current speed and scale) and hold
#                                at the step that starts DAY; print the clock.  On
#                                timeout the stop is disarmed, so nothing holds later
#   wine-game.sh messages [N]    Print the newest N (default 20) Message Index
#                                entries: day, step, kind, category, read, title,
#                                body (from game memory; days from the load on)
#   wine-game.sh speed LEVEL [SIDE]  Set game speed: pause, vslow, medium, fast
#                                (Alliance); pause, fast (Empire).  Pause opens
#                                "Resume Game Play?"; speed resume closes it
#   wine-game.sh alert-ref NAME  Save the open Battle Alert as reference NAME
#   wine-game.sh alert [FILE]    Print alert or none (exit 1) for the current screen
#   wine-game.sh resolve [FILE]  Auto-resolve the open Battle Alert (exit 1 when none
#                                is open; without a reference it clicks unchecked)
#   wine-game.sh run-to-alert TITLE.png LIMIT FILE
#                                Run until the alert whose title matches TITLE.png
#                                (a Wine frame of it) opens, resolving any other
#   wine-game.sh sync            (development Mac) copy these scripts to the
#                                Mini and run wine-setup.sh there
#
# All coordinates are in the VM's 800x600 reference frame.  The helper
# (drive.exe) translates to the actual client rectangle.
#
# drive.exe makes the game Wine's foreground window and injects hardware
# input with SendInput, so the Wine window does NOT need to be the macOS
# foreground app and no Accessibility grants are required.  SendInput's
# mouse moves warp the Mini's real pointer, so once up's selftest passes,
# mouse verbs go to drive64.exe, which sends them as Wine's macOS driver
# does and leaves the pointer alone.  Screenshots run inside the game
# process (drive-cap.dll).
#
# On the Mini, run verbs in the GUI session (README, Wine).  On a machine
# without ~/or-wine/env.sh, such as the development Mac, wine-game.sh runs
# each verb on WINE_HOST (default mac-mini-ts) through that queue: input
# files go there first and output files come back, with their SHA-256.

set -euo pipefail
script_dir="$(cd -- "$(dirname -- "$0")" && pwd)"
base="$HOME/or-wine"
env_file="$base/env.sh"
run_dir="$base/run"
log_dir="$base/logs"
lib="$script_dir/wine-game-lib.py"

# ---- client mode (development Mac) ----

client_sums() {
    (cd "$script_dir" && shasum -a 256 wine-game.sh wine-game-lib.py wine-drive.c wine-setup.sh)
}

client_sync() {
    local host=$1
    scp -q "$script_dir"/{wine-game.sh,wine-game-lib.py,wine-drive.c,wine-setup.sh} "$host:or-wine/bin/"
    ssh "$host" 'bash ~/or-wine/bin/wine-setup.sh'
}

# The verb in flight, for client_abort.
client_host="" client_id="" client_x="" client_ssh=""

# On a signal, take the verb's queued command and transfer folder off the
# Mini, so a verb given up on never reaches the game later.  A command
# already running finishes.
client_abort() {
    trap - HUP INT QUIT ALRM TERM
    if [[ -n $client_ssh ]]; then kill "$client_ssh" 2> /dev/null || true; fi
    ssh "$client_host" "rm -f or-wine/run/q/$client_id.cmd; rm -rf $client_x" || true
    exit "$1"
}

# Run one verb on the Mini.  File arguments are rewritten to a transfer
# folder there: inputs are copied up first, outputs copied back after.
client() {
    local host=${WINE_HOST:-mac-mini-ts} verb=${1:-}
    shift || true
    if [[ $verb == sync ]]; then client_sync "$host"; return; fi

    # Paths on the Mini are relative to its home folder, where the verb runs.
    local id x
    id="$(date +%s).$$"
    x="or-wine/tmp/xfer/$id"
    local -a args=("$@") ins=() outs=()
    # Indices of input and output file arguments, per verb.
    case "$verb" in
        cap)          outs=(0) ;;
        alert|resolve) if [[ $# -ge 1 ]]; then outs=(0); fi ;;
        day)          if [[ $# -ge 2 ]]; then outs=(1); fi ;;
        save-in)      ins=(0) ;;
        save-out)     outs=(1) ;;
        run-to-alert) ins=(0); outs=(2) ;;
    esac
    # bash 3.2 (macOS /bin/bash) treats an empty "${a[@]}" as unset.
    local -a pulls=()
    local i name local_path files=$(( ${#ins[@]} + ${#outs[@]} ))
    # A missing file argument goes through unchanged, for the verb's usage line.
    for i in ${ins[@]+"${ins[@]}"}; do
        [[ -z ${args[i]+set} || -f ${args[i]} ]] || { echo "No such file: ${args[i]}" >&2; return 2; }
    done
    client_host=$host client_id=$id client_x=$x
    trap 'client_abort 129' HUP
    trap 'client_abort 130' INT
    trap 'client_abort 131' QUIT
    trap 'client_abort 142' ALRM
    trap 'client_abort 143' TERM
    (( files == 0 )) || ssh "$host" "mkdir -p $x"
    for i in ${ins[@]+"${ins[@]}"}; do
        [[ -n ${args[i]+set} ]] || continue
        name=$(basename -- "${args[i]}")
        scp -q "${args[i]}" "$host:$x/$name" || { ssh "$host" "rm -rf $x"; trap - HUP INT QUIT ALRM TERM; return 1; }
        args[i]="$x/$name"
    done
    for i in ${outs[@]+"${outs[@]}"}; do
        [[ -n ${args[i]+set} ]] || continue
        local_path=${args[i]}
        case "$verb" in
            cap|alert|resolve|day|run-to-alert) local_path="${local_path%.png}.png" ;;
            save-out) local_path="${local_path%.sav}.sav" ;;
        esac
        name=$(basename -- "$local_path")
        args[i]="$x/$name"
        pulls+=("$name" "$local_path")
    done

    local cmdline hold rc=0
    cmdline="cd ~ && bash or-wine/bin/wine-game.sh $(printf '%q ' "$verb" ${args[@]+"${args[@]}"})"
    # The Mini's waiter cancels the verb when its stdin ends
    # (WINE_QUEUE_EOF=cancel).  That stdin is a pipe whose only writer is
    # this shell, so it ends when the connection drops or this shell dies,
    # however it dies.  ssh runs in the background, so a trapped signal
    # ends the wait at once rather than after the verb.
    hold=$(mktemp -d)
    mkfifo "$hold/stdin"
    exec 3<> "$hold/stdin" 4< "$hold/stdin"
    rm -rf "$hold"
    # The Mini's copies must match this checkout, or verbs run old code.
    ssh "$host" "cd ~/or-wine/bin && printf %s $(printf '%q' "$(client_sums)") | shasum -a 256 -c --status \
        || { echo 'The Mini runs other copies of the Wine scripts: run wine-game.sh sync.' >&2; exit 125; }
        WINE_QUEUE_ID=$id WINE_QUEUE_EOF=cancel ~/or-wine/bin/wine-queue.sh $(printf '%q' "$cmdline")" \
        <&4 3>&- 4<&- &
    client_ssh=$!
    wait "$client_ssh" || rc=$?
    client_ssh=""
    exec 3>&- 4<&-
    for ((i = 0; i < ${#pulls[@]}; i += 2)); do
        if scp -q "$host:$x/${pulls[i]}" "${pulls[i + 1]}" 2> /dev/null; then
            echo "${pulls[i + 1]} sha256=$(shasum -a 256 "${pulls[i + 1]}" | cut -d' ' -f1)"
        fi
    done
    (( files == 0 )) || ssh "$host" "rm -rf $x"
    trap - HUP INT QUIT ALRM TERM
    return "$rc"
}

if [[ ! -f "$env_file" ]]; then
    client "$@"
    exit
fi
# shellcheck disable=SC1090
. "$env_file"

mkdir -p "$run_dir" "$log_dir" "$base/tmp" "$base/out"

# ---- internal helpers ----

drive() {
    # Run the Windows-side helper under Wine.  WINEDEBUG=-all silences
    # Wine's fixme lines; the helper's own errors still reach stderr.  Its
    # console output ends lines with CR LF.  Mouse verbs go to drive64.exe,
    # which leaves the Mac pointer alone, once up's selftest has passed.
    local helper=drive.exe
    case ${1:-} in
        click|rclick|dclick|drag) [[ -f $run_dir/mouse ]] && helper=drive64.exe ;;
    esac
    WINEDEBUG=-all "$WINE_BIN" "$WINE_GAME_DIR/$helper" "$@" | tr -d '\r'
}

# Check that drive64.exe moves Wine's cursor without moving the Mac pointer
# and, if it does, point the game's GetCursorPos at drive64.exe's moves
# (wine-drive.c, "input without warping").  Otherwise mouse verbs stay on
# drive.exe, whose moves warp the pointer.
mouse_check() {
    rm -f "$run_dir/mouse"
    local out
    if out=$(WINEDEBUG=-all "$WINE_BIN" "$WINE_GAME_DIR/drive64.exe" selftest 2>&1 | tr -d '\r') \
        && drive cursor on > /dev/null; then
        echo drive64.exe > "$run_dir/mouse"
        echo 'Mouse input leaves the Mac pointer alone (drive64.exe).'
    else
        printf '%s\n' "$out" >&2
        echo 'WARNING: drive64.exe selftest failed; mouse moves will warp the Mac pointer.' >&2
    fi
}

game_pid() {
    # The game's Wine process, by its exact process name.
    pgrep -x REBEXE.EXE 2>/dev/null | head -1 || true
}

is_running() {
    [[ -n "$(game_pid)" ]]
}

wait_for_window() {
    local limit=${1:-60} start=$SECONDS
    while (( SECONDS - start < limit )); do
        if drive find 2> /dev/null | grep -qx found; then
            return 0
        fi
        sleep 0.2
    done
    return 1
}

bmp_to_png() {
    python3 "$lib" bmp2png "$1" "$2"
}

file_hash() {
    shasum -a 256 "$1" | cut -d' ' -f1
}

# Exit 0 when VALUE < LIMIT, 1 when not, 2 when VALUE is not a number.
below() {
    [[ $1 =~ ^[0-9]+(\.[0-9]+)?$ ]] || { echo "Not a measurement: '$1'" >&2; return 2; }
    awk -v d="$1" -v m="$2" 'BEGIN { exit !(d + 0 < m + 0) }'
}

# ---- verbs ----

do_up() {
    if is_running; then
        echo "Game is already running (pid $(game_pid))."
        return 0
    fi

    echo "Starting REBEXE.EXE under Wine..."

    # Launch the game.
    (
        cd "$WINE_GAME_DIR"
        WINEDEBUG=-all nohup "$WINE_BIN" REBEXE.EXE \
            > "$log_dir/game.log" 2>&1 &
    )

    # Wine's services take 80 s or more to start while the Mini builds.
    echo "Waiting for game window (up to 180s)..."
    if ! wait_for_window 180; then
        echo "ERROR: Game window did not appear.  Check $log_dir/game.log" >&2
        grep -v -E '^[[:space:]]|mvk-info' "$log_dir/game.log" | tail -20 >&2 || true
        return 1
    fi

    local pid
    pid=$(game_pid)
    echo "DISPLAY=macOS-cocoa"
    echo "pid=$pid"
    echo "session=$base"

    # Save session state.
    echo "$pid" > "$run_dir/game.pid"

    # Skip the intro cinematics.
    sleep 2
    drive key esc
    sleep 0.5
    drive key esc
    sleep 0.5
    drive key esc
    sleep 2

    is_running || { echo 'ERROR: the game exited while skipping the intros.' >&2; return 1; }
    echo "Game is past the intros (main menu not checked by image)."
    mouse_check
}

do_down() {
    if ! is_running; then
        echo "stopped"
        return 0
    fi
    # Send Alt+F4 to quit gracefully.
    drive key alt+f4 2>/dev/null || true
    sleep 2
    # Force kill if still running.
    if is_running; then
        "$WINE_SERVER" -k 2>/dev/null || true
    fi
    sleep 1
    rm -f "$run_dir/game.pid" "$run_dir/mouse"
    echo "stopped"
}

do_status() {
    if is_running; then
        echo "pid=$(game_pid)"
        echo "session=$base"
        drive size 2>/dev/null || true
        if [[ -f $run_dir/mouse ]]; then echo 'mouse=drive64.exe'; else echo 'mouse=drive.exe (warps)'; fi
    else
        echo "stopped"
        return 1
    fi
}

# Check reference points given as X Y [X Y]: X 0-799, Y 0-599, so a typo
# never becomes a click at (0,0) or outside the game.
check_points() {
    local v limit i=0
    for v in "$@"; do
        if (( i % 2 == 0 )); then limit=800; else limit=600; fi
        [[ $v =~ ^[0-9]+$ ]] && (( 10#$v < limit )) \
            || { echo "Not a point in the 800x600 frame: '$v' (X 0-799, Y 0-599)" >&2; return 2; }
        i=$((i + 1))
    done
}

do_click() {
    [[ $# -ge 2 ]] || { echo 'click X Y' >&2; return 2; }
    check_points "$1" "$2" || return 2
    drive click "$1" "$2"
}

do_rclick() {
    [[ $# -ge 2 ]] || { echo 'rclick X Y' >&2; return 2; }
    check_points "$1" "$2" || return 2
    drive rclick "$1" "$2"
}

do_dclick() {
    [[ $# -ge 2 ]] || { echo 'dclick X Y' >&2; return 2; }
    check_points "$1" "$2" || return 2
    drive dclick "$1" "$2"
}

do_drag() {
    [[ $# -ge 4 ]] || { echo 'drag X1 Y1 X2 Y2' >&2; return 2; }
    check_points "$1" "$2" "$3" "$4" || return 2
    drive drag "$1" "$2" "$3" "$4"
}

do_key() {
    [[ $# -ge 1 ]] || { echo 'key NAME [COUNT]' >&2; return 2; }
    drive key "$1" "${2:-1}"
}

do_type() {
    [[ $# -ge 1 ]] || { echo 'type TEXT' >&2; return 2; }
    drive type "$1"
}

do_cap() {
    [[ $# -ge 1 ]] || { echo 'cap FILE' >&2; return 2; }
    local out="$1"
    # Ensure .png extension for the final output.
    local png_out="${out%.bmp}"
    png_out="${png_out%.png}.png"

    local tmp_bmp="$base/tmp/cap_$$.bmp"
    local wine_bmp="$WINE_GAME_DIR/capture.bmp"
    # A capture that timed out earlier may still land: never take it for this one.
    rm -f "$wine_bmp"
    drive cap 'C:\Rebellion\capture.bmp'
    # Copy from Wine prefix to host.
    if [[ ! -f "$wine_bmp" ]]; then
        echo "ERROR: Capture failed — no BMP written" >&2
        return 1
    fi
    mv "$wine_bmp" "$tmp_bmp"
    bmp_to_png "$tmp_bmp" "$png_out"
    rm -f "$tmp_bmp"
    local hash
    hash=$(file_hash "$png_out")
    echo "sha256=$hash"
    echo "$png_out"
}

do_clock() {
    drive clock
}

do_peek() {
    [[ $# -ge 2 && $1 =~ ^(0x[0-9a-fA-F]+|[0-9]+)$ && $2 =~ ^(0x[0-9a-fA-F]+|[0-9]+)$ \
        && ${3:-0} =~ ^[0-9]+$ ]] \
        || { echo 'peek PTR OFF [N]  (PTR and OFF decimal or 0x hex; N samples 0-100000)' >&2; return 2; }
    drive peek "${@:1:3}"
}

# The scheduler's address from a clock line (empty when there is no game).
sched_of() {
    sed -n 's/.* sched=\(0x[0-9a-f]*\)$/\1/p' <<<"$1"
}

# The day from a clock line (empty when there is no game).
day_of() {
    sed -n 's/^day=\([0-9]*\) .*/\1/p' <<<"$1"
}

# Refuse a hold at DAY unless DAY is still ahead: the first step of a day
# that has begun is past, and the hold would land at once on a later step.
day_ahead() {
    local line now
    line=$(drive clock) || return 1
    now=$(day_of "$line")
    (( $1 > now )) \
        || { echo "Day $1 has already begun (the game is on day $now): pick a later day." >&2; return 2; }
}

do_load() {
    [[ $# -ge 1 && $1 =~ ^[0-9]+$ && ${2:-} =~ ^(menu)?$ ]] \
        || { echo 'load SLOT (1-6) [menu]' >&2; return 2; }
    local slot=$1 from=${2:-game}
    (( slot >= 1 && slot <= 6 )) || { echo 'Slot must be 1-6 (the six rows on screen).' >&2; return 2; }

    # In game, F1 opens Game Options, which shows the Saved Games list, and
    # a load asks "Load without saving?".  From the main menu, Saved Games
    # (535,315) opens the same list and loads without asking.  Row y values
    # and button x from vm-game.sh (800x600 coordinates).
    local -a ROWS=(113 166 218 270 323 376)
    local LOAD_X=384
    local YES_X=350 YES_Y=377
    local y=${ROWS[$((slot - 1))]}
    local before="" after=""

    if [[ $from == menu ]]; then
        drive click 535 315
    else
        before=$(drive clock) || { echo 'No game in progress: use load SLOT menu.' >&2; return 1; }
        drive key f1
    fi
    sleep 1
    drive click "$LOAD_X" "$y"
    if [[ $from == game ]]; then
        sleep 0.5
        drive click "$YES_X" "$YES_Y"
    fi
    # A save with a day-30 alert shows the galaxy 8-11 s after confirmation.
    sleep 12
    # A load builds a new game state, scheduler included, but the new
    # scheduler can take the old one's freed memory: only a clock that is
    # the same in address, day and steps proves that nothing loaded.
    after=$(drive clock 2> /dev/null) || { echo "ERROR: no game after loading slot $slot." >&2; return 1; }
    if [[ -n $before && $after == "$before" ]]; then
        echo "ERROR: slot $slot did not load (the game clock is unchanged: $after)." >&2
        return 1
    fi
    if [[ -n $before && $(sched_of "$after") == "$(sched_of "$before")" ]]; then
        echo "WARNING: the day scheduler kept its address ($before -> $after);" \
            "check the day against the save before trusting this load." >&2
    fi
    # The step hook logs each day's first step, which turns a message's
    # filing step into a day.
    drive hook
    echo "Loaded slot $slot: $after"
}

do_save() {
    [[ $# -ge 2 && $1 =~ ^[0-9]+$ ]] || { echo 'save SLOT (1-6) NAME' >&2; return 2; }
    local slot=$1 name=$2
    (( slot >= 1 && slot <= 6 )) || { echo 'Slot must be 1-6 (the six rows on screen).' >&2; return 2; }
    [[ $name =~ ^[A-Za-z0-9\ -]{1,31}$ ]] \
        || { echo 'NAME: 1-31 letters, digits, spaces or hyphens' >&2; return 2; }

    # In game, as vm-game.sh save: F1, the slot's name field, replace the
    # name, Enter, "Save anyway?", "Selected Game Saved Successfully", then
    # Escape and Game Options' galaxy icon back to the running galaxy.
    local -a ROWS=(113 166 218 270 323 376)
    local NAME_X=245 YES_X=350 YES_Y=377 OK_X=398 OK_Y=375 GALAXY_X=228 GALAXY_Y=490
    local y=${ROWS[$((slot - 1))]}
    local file before after
    printf -v file '%s/SaveGame/SAVEGAME.%03d' "$WINE_GAME_DIR" "$slot"
    before=$( [[ -f $file ]] && file_hash "$file" || echo none )

    drive key f1
    sleep 1
    drive click "$NAME_X" "$y"
    drive key end
    drive key backspace 40
    drive type "$name"
    drive key enter
    sleep 1
    drive click "$YES_X" "$YES_Y"
    sleep 2
    drive click "$OK_X" "$OK_Y"
    sleep 1
    drive key esc
    sleep 1
    drive click "$GALAXY_X" "$GALAXY_Y"
    sleep 1
    after=$( [[ -f $file ]] && file_hash "$file" || echo none )
    if [[ $after == none || $after == "$before" ]]; then
        echo "ERROR: slot $slot's file did not change; the save did not happen." >&2
        return 1
    fi
    echo "Saved slot $slot as \"$name\" sha256=$after"
}

do_save_in() {
    [[ $# -ge 2 && $2 =~ ^[0-9]+$ ]] || { echo 'save-in FILE SLOT (1-10)' >&2; return 2; }
    local file="$1"
    local slot=$2
    (( slot >= 1 && slot <= 10 )) || { echo 'Slot must be 1-10.' >&2; return 2; }

    local padded
    printf -v padded '%03d' "$slot"
    local dest="$WINE_GAME_DIR/SaveGame/SAVEGAME.$padded"
    cp "$file" "$dest"
    echo "Copied $file -> slot $slot"
    echo "sha256=$(file_hash "$dest")"
}

do_save_out() {
    [[ $# -ge 2 && $1 =~ ^[0-9]+$ ]] || { echo 'save-out SLOT NAME' >&2; return 2; }
    local slot=$1
    local dest="${2%.sav}.sav"
    # The runner's working folder is /, so a bare name goes to ~/or-wine/out.
    [[ $dest == */* ]] || dest="$base/out/$dest"
    (( slot >= 1 && slot <= 10 )) || { echo 'Slot must be 1-10.' >&2; return 2; }

    local padded
    printf -v padded '%03d' "$slot"
    local src="$WINE_GAME_DIR/SaveGame/SAVEGAME.$padded"
    [[ -f "$src" ]] || { echo "Slot $slot is empty." >&2; return 1; }
    cp "$src" "$dest"
    echo "Copied slot $slot -> $dest"
    echo "sha256=$(file_hash "$dest")"
}

do_day() {
    # Capture the day readout, in 800x600 reference coordinates (crop scales
    # them to the capture).  Alliance (125,20,63,19) was measured under Wine
    # on day 61-110; Empire (622,22,80,18) is vm-wait.py's day region.
    local side="${1:-alliance}" box
    case "$side" in
        alliance) box=(125 20 63 19) ;;
        empire)   box=(622 22 80 18) ;;
        *) echo 'day [alliance|empire] [FILE]' >&2; return 2 ;;
    esac
    local frame="$base/tmp/day_$$.png"
    local crop_png="${2:-$base/tmp/day_crop_$$.png}"
    crop_png="${crop_png%.png}.png"

    capture_to "$frame"
    python3 "$lib" crop "$frame" "${box[@]}" "$crop_png"
    rm -f "$frame"
    echo "Day readout saved to: $crop_png"
    echo "sha256=$(file_hash "$crop_png")"
}

# ---- Battle Alert ----
#
# An alert is open when its button bar matches one of the reference frames
# that alert-ref saved (mean difference under ALERT_DIFF; galaxy frames
# score about 49).  Its title matches TITLE.png when under TITLE_CHANGED of
# the title box's pixels changed (two different titles change about 8%).
# References are game output, so they stay on the Mini; keep one per side.

ALERT_REFS="$base/ref"
ALERT_BAR=(172 435 500 32)
ALERT_TITLE=(175 78 500 35)
ALERT_DIFF=10
TITLE_CHANGED=0.01

capture_to() {
    do_cap "$1" > /dev/null
}

# Exit 0 for an alert, 1 for none, 2 when no reference or no measurement.
is_alert() {
    local ref d rc seen=0
    for ref in "$ALERT_REFS"/alert-*.png; do
        [[ -f $ref ]] || continue
        seen=1
        d=$(python3 "$lib" diff "$1" "$ref" "${ALERT_BAR[@]}") || return 2
        if below "$d" "$ALERT_DIFF"; then return 0; else rc=$?; (( rc == 1 )) || return 2; fi
    done
    (( seen )) || { echo 'No alert reference: run alert-ref NAME with an alert open.' >&2; return 2; }
    return 1
}

do_alert_ref() {
    [[ $# -ge 1 && $1 =~ ^[a-z0-9-]+$ ]] || { echo 'alert-ref NAME (such as empire or alliance)' >&2; return 2; }
    mkdir -p "$ALERT_REFS"
    capture_to "$ALERT_REFS/alert-$1.png"
    echo "Saved the alert reference to $ALERT_REFS/alert-$1.png"
}

do_alert() {
    local file="${1:-$base/tmp/alert_$$.png}" rc=0
    file="${file%.png}.png"
    capture_to "$file" || return 2
    is_alert "$file" || rc=$?
    [[ -n ${1:-} ]] || rm -f "$file"
    case $rc in
        0) echo alert ;;
        1) echo none ;;
    esac
    return "$rc"
}

have_alert_refs() {
    local ref
    for ref in "$ALERT_REFS"/alert-*.png; do
        [[ -f $ref ]] && return 0
    done
    return 1
}

# The Battle Alert's auto-resolve button, then Escape past the result.
press_resolve() {
    drive click 420 450
    sleep 3
    drive key esc
    sleep 1
}

# Resolve only an alert that is open: on the galaxy the button's point is
# an ordinary click.  Without a reference there is no check to make.
do_resolve() {
    local frame="$base/tmp/resolve_$$.png" rc=0
    if have_alert_refs; then
        capture_to "$frame" || return 2
        is_alert "$frame" || rc=$?
        rm -f "$frame"
        case $rc in
            0) ;;
            1) echo 'No Battle Alert is open; nothing resolved.' >&2; return 1 ;;
            *) return 2 ;;
        esac
    else
        echo 'WARNING: no alert reference (alert-ref NAME), so resolve clicks without checking for an alert.' >&2
    fi
    press_resolve
    [[ -z ${1:-} ]] || capture_to "$1"
    echo resolved
}

do_run_to_alert() {
    [[ $# -ge 3 && $2 =~ ^[0-9]+$ ]] || { echo 'run-to-alert TITLE.png LIMIT FILE' >&2; return 2; }
    local title=$1 limit=$2 file="${3%.png}.png" start=$SECONDS frame="$base/tmp/run_$$.png" d rc
    [[ -f $title ]] || { echo "No title frame: $title" >&2; return 2; }
    while (( SECONDS - start < limit )); do
        capture_to "$frame" || return 2
        rc=0
        is_alert "$frame" || rc=$?
        (( rc != 2 )) || { rm -f "$frame"; return 2; }
        if (( rc == 0 )); then
            d=$(python3 "$lib" changed "$frame" "$title" "${ALERT_TITLE[@]}") || { rm -f "$frame"; return 2; }
            rc=0
            below "$d" "$TITLE_CHANGED" || rc=$?
            (( rc != 2 )) || { rm -f "$frame"; return 2; }
            if (( rc == 0 )); then
                mv -f "$frame" "$file"
                echo "Alert matched after $((SECONDS - start)) s: $file"
                return 0
            fi
            press_resolve
            echo "Resolved a non-matching alert (title changed $d)"
        fi
        sleep 0.5
    done
    rm -f "$frame"
    echo "run-to-alert: no matching alert in $limit s" >&2
    return 1
}

do_timescale() {
    [[ $# -ge 1 && $1 =~ ^[0-9]+$ ]] || { echo 'timescale K (1-64)' >&2; return 2; }
    drive timescale "$1"
}

do_messages() {
    [[ ${1:-20} =~ ^[0-9]+$ ]] || { echo 'messages [N]' >&2; return 2; }
    drive messages "${1:-20}" 'C:\Rebellion\messages.txt'
    rm -f "$WINE_GAME_DIR/messages.txt"
}

do_stop_at() {
    [[ $# -ge 1 && $1 =~ ^[0-9]+$ ]] || { echo 'stop-at DAY (0 releases)' >&2; return 2; }
    (( 10#$1 == 0 )) || day_ahead "$((10#$1))" || return
    drive stopat "$((10#$1))"
}

# The hold stops the strategic step, not the game speed: set a speed first.
do_run_to_day() {
    [[ $# -ge 1 && $1 =~ ^[1-9][0-9]*$ && ${2:-600} =~ ^[0-9]+$ ]] \
        || { echo 'run-to-day DAY [LIMIT seconds, default 600]' >&2; return 2; }
    local day=$1 limit=${2:-600} start=$SECONDS line="" last="" now
    day_ahead "$day" || return
    drive stopat "$day" > /dev/null
    while (( SECONDS - start < limit )); do
        line=$(drive clock)
        now=$(day_of "$line")
        # Held once the day is reached and two reads a second apart agree.
        if (( now >= day )) && [[ $line == "$last" ]]; then
            echo "Held: $line"
            return 0
        fi
        last=$line
        sleep 1
    done
    # Disarm the stop, or the game holds at DAY later, unasked.  Only the
    # hold landing clears the stop by itself, so a clock at or past DAY
    # after the disarm means the hold landed at the deadline.
    drive stopat disarm > /dev/null
    line=$(drive clock)
    now=$(day_of "$line")
    if (( now >= day )); then
        echo "Held: $line"
        return 0
    fi
    echo "run-to-day: day $day not reached in $limit s ($line); the stop is disarmed" >&2
    return 1
}

do_speed() {
    [[ $# -ge 1 ]] || { echo 'speed pause|vslow|medium|fast|resume [alliance|empire]' >&2; return 2; }
    # Right-click the day readout to open the speed menu.
    # Coordinates from vm-game.sh README (800x600, calibrated 2026-10-16).
    # Alliance: readout 165,30; menu Pause 208,45 / VSlow 230,70 / Medium 220,120 / Fast 208,145.
    # Empire:   readout 645,30; menu Pause 668,45 / Fast 695,137.
    local level="$1"
    local side="${2:-alliance}"

    # Pause opens the modal "Resume Game Play?" on either side; its check
    # mark (398,373) resumes at the earlier speed.  Other verbs miss while
    # it is open.
    if [[ $level == resume ]]; then
        drive click 398 373
        sleep 0.3
        echo "Resumed"
        return 0
    fi

    if [[ "$side" == "empire" ]]; then
        case "$level" in
            pause|fast) ;;
            *) echo "Empire speed menu: only pause and fast have known coordinates." >&2; return 2 ;;
        esac
        drive rclick 645 30
        sleep 0.3
        case "$level" in
            pause)  drive click 668 45 ;;
            fast)   drive click 695 137 ;;
        esac
    else
        case "$level" in
            pause|vslow|medium|fast) ;;
            *) echo "Unknown speed: $level (use pause, vslow, medium, fast)" >&2; return 2 ;;
        esac
        drive rclick 165 30
        sleep 0.3
        case "$level" in
            pause)  drive click 208 45 ;;
            vslow)  drive click 230 70 ;;
            medium) drive click 220 120 ;;
            fast)   drive click 208 145 ;;
        esac
    fi
    sleep 0.3
    echo "Speed set to $level ($side)"
}

# ---- dispatch ----

cmd="${1:-}"
shift || true

case "$cmd" in
    up)       do_up ;;
    down)     do_down ;;
    status)   do_status ;;
    click)    do_click "$@" ;;
    rclick)   do_rclick "$@" ;;
    dclick)   do_dclick "$@" ;;
    drag)     do_drag "$@" ;;
    key)      do_key "$@" ;;
    type)     do_type "$@" ;;
    save)     do_save "$@" ;;
    cap)      do_cap "$@" ;;
    load)     do_load "$@" ;;
    save-in)  do_save_in "$@" ;;
    save-out) do_save_out "$@" ;;
    day)      do_day "$@" ;;
    speed)    do_speed "$@" ;;
    clock)    do_clock ;;
    peek)     do_peek "$@" ;;
    alert-ref) do_alert_ref "$@" ;;
    alert)    do_alert "$@" ;;
    resolve)  do_resolve "$@" ;;
    run-to-alert) do_run_to_alert "$@" ;;
    timescale) do_timescale "$@" ;;
    stop-at)  do_stop_at "$@" ;;
    messages) do_messages "$@" ;;
    run-to-day) do_run_to_day "$@" ;;
    *)
        echo "Usage: wine-game.sh up|down|status|click|rclick|dclick|drag|key|type|cap|load|save|save-in|save-out|day|clock|peek|messages|timescale|stop-at|run-to-day|speed|alert-ref|alert|resolve|run-to-alert|sync" >&2
        exit 2 ;;
esac
