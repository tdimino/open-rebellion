#!/usr/bin/env bash
# Drive the original game on the Mac Mini capture VM from this Mac.
#
#   vm-game.sh status            QEMU process, VNC reachability, monitor mice
#   vm-game.sh up                start the VM if needed, wait for the desktop,
#                                launch REBEXE, accept UAC, skip the intros and
#                                switch to the absolute tablet
#   vm-game.sh recover [a0]      kill a wedged QEMU, restart UTM, then `up`
#                                (`up-a0` with a0)
#   vm-game.sh up-a0             the A0 route: start the capture watcher and
#                                the game as scheduled tasks on the desktop
#   vm-game.sh a0-push           copy vm/capture.ps1 and a0-tasks.ps1 to C:\a0
#   vm-game.sh a0-setup          a0-push, then register the A0 tasks and set
#                                DisplayFilter=point (vm/a0-tasks.ps1)
#   vm-game.sh a0-watch          restart the watcher task
#   vm-game.sh a0-game           end the game task and start it again, to the
#                                main menu, relaunching until the scaling
#                                phase is the calibrated one (A0_PHASE_TRIES)
#   vm-game.sh a0-run FILE       run a PowerShell FILE on the guest desktop
#                                (A0Run task) and print its output
#   vm-game.sh a0 NAME [OUTDIR] [Key=Value ...]
#                                the game's exact 640x480 frame, rebuilt in the
#                                guest from the full screen; Keys are capture.ps1
#                                parameters
#                                (CellId, Requirement, StateSetupKind, ...;
#                                InputTrace='a|b|c'; FullClient=1 probes;
#                                A0_ON_START="click X Y" sends that command
#                                once a burst has taken its first frame, over
#                                10 s late; ClickOnStart=1 with A0_PARK="X Y"
#                                clicks in the guest right after frame 0)
#   vm-game.sh a0-cell CELL KIND SETUP 'a|b|c' [NOTES]
#                                capture a ledger cell (requirement from the
#                                ledger) and register it with the ingester;
#                                A0_BEFORE=NAME adds an earlier `a0` frame
#   vm-game.sh a0-register CELL [FRAME]
#                                register a pulled capture; FRAME names the
#                                burst frame that stands for the cell
#   vm-game.sh a0-audio start|stop CELL
#                                record the guest's sound around a cell's
#                                frame; a0-register then ingests it too
#   vm-game.sh cap FILE          capture the screen to FILE
#   vm-game.sh point X Y [FILE]  move the game cursor to X,Y
#   vm-game.sh click X Y [FILE]  left-click, optionally capture after
#   vm-game.sh rclick X Y [FILE] right-click, optionally capture after
#   vm-game.sh dclick X Y [FILE] double-click
#   vm-game.sh drag X1 Y1 X2 Y2 [FILE]  left-drag between two points
#   vm-game.sh press X Y [right] [FILE]  hold a button down on X,Y (a
#                                pressed state; capture with A0_PARK=none)
#   vm-game.sh release [right]   let it go
#   vm-game.sh desk X Y [FILE]   click on the Windows desktop (taskbar, a
#                                console), where the cursor is not the game's
#   vm-game.sh key KEY [FILE]    send a vncdo key (esc, f1, alt-y, ...)
#   vm-game.sh vnc ACTIONS...    raw vncdo actions
#   vm-game.sh wait REGION [N] [TIMEOUT] FILE
#                                hold one VNC connection until REGION (day,
#                                center or X,Y,W,H) changes N times (default
#                                1, timeout 120 s), then capture to FILE;
#                                REGION alert waits until a Battle Alert is
#                                open, including one already open
#   vm-game.sh run-to-alert TITLE.png TIMEOUT FILE
#                                run until the alert whose title matches
#                                TITLE.png (an 800x600 frame of it) opens,
#                                auto-resolving every other alert
#   vm-game.sh save-out SLOT NAME  copy save slot SLOT (1-10) to NAME.sav
#                                here; prints both SHA-256 values
#   vm-game.sh save-in FILE SLOT copy FILE into save slot SLOT
#   vm-game.sh gexec "CMDLINE"   run cmd /c CMDLINE in the guest (as SYSTEM,
#                                no desktop): exit code, stdout, stderr
#   vm-game.sh mon "COMMAND"     run a QEMU monitor command
#
# In-game verbs (the galaxy view, no dialog open):
#   vm-game.sh load SLOT [FILE]  F1, load slot SLOT (1-6, the rows Saved Games
#                                shows), confirm, wait out the load
#   vm-game.sh save SLOT NAME    F1, save over filled slot SLOT as NAME
#                                (lowercase letters, digits, space, hyphen)
#   vm-game.sh resolve [FILE]    at a Battle Alert: auto-resolve, close the result
#   vm-game.sh take-command [FILE]  at a Battle Alert: open the battlefield
#                                (paused); launch fresh first (see below)
#   vm-game.sh withdraw [FILE]   on the battlefield: Battle Options, Withdraw,
#                                confirm, unpause
#
#   vm-game.sh run FILE [OUTDIR] run a command file: one vm-game.sh command
#                                per line, `#` comments; $OUT is OUTDIR
#                                (default: FILE's folder). Stops at a failure.
#
# Every call appends a JSON line to $VM_LOG (default .artifacts/vm/log.jsonl):
# time, command, arguments, exit code, seconds, and the SHA-256 of any PNG
# it wrote.
#
# Pointer commands walk the tablet over the loopback QMP socket on the Mini
# (vm-qmp.py); VM_INPUT=vnc walks it over VNC instead. Double-clicks always
# press over VNC: two QMP-sent clicks after a QMP walk never registered as a
# double-click (0 of 30), while the same walk followed by VNC presses did 20/20
# (2026-10-11). The VNC password comes from the keychain and is never
# printed. Coordinates are the 800x600 guest frame a capture shows. See
# README.md, "Saves" for the pointer and focus caveats.
set -euo pipefail

MINI=${VM_MINI:-tamarru@mac-mini-ts}
VNC_HOST=100.125.235.4
VNC=${VNC_HOST}::5901
MONITOR_PORT=4445
VMDIR='~/VMs/rebellion'
EXCHANGE=/Users/tamarru/VMs/rebellion/exchange
HERE=$(cd "$(dirname "$0")" && pwd)
INPUT=${VM_INPUT:-qmp}
A0_DIR='C:\a0'
# Where `a0` parks the game's cursor (it is drawn into the frame).
A0_PARK=${A0_PARK:-799 599}
# The game's taskbar button on the 800x600 desktop (2026-10-11).
A0_TASKBAR=${A0_TASKBAR:-510 576}

# A Tailscale drop left an ssh to the Mini waiting forever mid-transfer
# (2026-10-11); fail within ~40 s instead, so the caller can retry.
SSH_OPTS=(-o ConnectTimeout=10 -o ServerAliveInterval=10 -o ServerAliveCountMax=3)
# A Tailscale connection that stops answering held calls for 15 and 33
# minutes (2026-10-11) without ServerAlive ending it, so every call is also
# bounded (124 on expiry); SSH_TIMEOUT raises it for a longer call.
ssh() { timeout "${SSH_TIMEOUT:-180}" ssh "${SSH_OPTS[@]}" "$@"; }
scp() { timeout "${SSH_TIMEOUT:-180}" scp "${SSH_OPTS[@]}" "$@"; }

# vm-qmp.py runs on the Mini, where QMP listens on loopback only.
qmp() {
  local args
  args=$(printf '%q ' "$@")
  ssh "$MINI" "python3 - $args" < "$HERE/vm-qmp.py"
}

# vncdo runs in-process with the password from the environment, so it never
# reaches a process's argv.
vnc() {
  VNC_PASSWORD=$(security find-internet-password -s "$VNC_HOST" -w 2>/dev/null || true) \
    timeout "${VNC_TIMEOUT:-60}" uvx --quiet --with vncdotool python -c '
import os, sys
from vncdotool.command import vncdo
password = os.environ.pop("VNC_PASSWORD", "")
vncdo(["-s", sys.argv[1]] + (["-p", password] if password else []) + sys.argv[2:])
' "$VNC" "$@"
}

# A failed ssh or monitor connection fails the call; only the filtering of
# the monitor's banner may come up empty.
mon() {
  local out
  out=$(ssh "$MINI" "(echo '$1'; sleep 1) | nc 127.0.0.1 $MONITOR_PORT") || return
  printf '%s\n' "$out" | tr -d '\033' | grep -v -e '^QEMU' -e '\[K' || true
}

# Run a vm-ctl.applescript command through the GUI-domain launch agent.
# The Mini's wrapper gives up after TIMEOUT; this side allows a minute more.
# A dropped or expired connection is retried once; every vm-ctl command we
# send is safe to repeat.
vmctl() {
  local rc=0 try
  for try in 1 2; do
    rc=0
    SSH_TIMEOUT=$(( ${2:-60} + 60 )) ssh "$MINI" "$VMDIR/vmctl-run.sh $(printf '%q' "$1") ${2:-60}" || rc=$?
    [[ $rc -eq 255 || $rc -eq 124 ]] || return "$rc"
    echo "vmctl: connection lost (rc $rc), retrying" >&2
  done
  return "$rc"
}

qemu_running() {
  ssh "$MINI" 'pgrep -f qemu-aarch64-softmmu >/dev/null'
}


# Mean brightness of the bottom 40 rows; the Windows 11 taskbar is light,
# the boot and Welcome screens are dark.
taskbar_up() {
  local shot=$1
  uvx --quiet --with pillow python -c "
import sys
from PIL import Image, ImageStat
im = Image.open(sys.argv[1]).convert('L')
w, h = im.size
sys.exit(0 if ImageStat.Stat(im.crop((0, h - 40, w, h))).mean[0] > 180 else 1)
" "$shot"
}

wait_for() {
  local what=$1 tries=$2 check=$3 i
  for ((i = 1; i <= tries; i++)); do
    if $check; then echo "$what after $i tries"; return 0; fi
    sleep 5
  done
  echo "timed out waiting for $what" >&2
  return 1
}

LOG=${VM_LOG:-$HERE/../../.artifacts/vm/log.jsonl}
START=$(python3 -c 'import time; print(time.time())')
ARGV=("$@")

# One JSON line per call, written on exit so failures are logged too.
log_call() {
  local rc=$1
  mkdir -p "$(dirname "$LOG")"
  python3 - "$rc" "$START" "${ARGV[@]}" >> "$LOG" <<'EOF' || true
import datetime, hashlib, json, os, sys, time
rc, start, argv = int(sys.argv[1]), float(sys.argv[2]), sys.argv[3:]
shots = {}
for a in argv:
    if a.endswith(".png") and os.path.isfile(a) and os.path.getmtime(a) >= start - 1:
        shots[a] = hashlib.sha256(open(a, "rb").read()).hexdigest()
print(json.dumps({
    "t": datetime.datetime.fromtimestamp(start).astimezone().isoformat(timespec="seconds"),
    "cmd": argv[0] if argv else "status", "args": argv[1:], "rc": rc,
    "s": round(time.time() - start, 1), "png": shots,
}))
EOF
}

# mktemp makes the file without the suffix; remove both.
SHOT_BASE=$(mktemp -t vm-game)
SHOT=$SHOT_BASE.png
trap 'rc=$?; rm -f "$SHOT_BASE" "$SHOT"; log_call "$rc"' EXIT
vnc_answers() { VNC_TIMEOUT=20 vnc capture "$SHOT" >/dev/null 2>&1; }
desktop_ready() { vnc_answers && taskbar_up "$SHOT"; }

up() {
  desktop_up
  # Win+R C:\rebellion\rebexe.exe; the VNC link drops Shift, so ':' is a chord.
  vnc key super-r pause 2 type c key 'shift-;' type '\rebellion\rebexe.exe' key enter pause 5 key alt-y pause 15 \
    key esc pause 3 key esc pause 3 key esc pause 5
  mon 'mouse_set 1' >/dev/null
  echo "game launched; tablet absolute"
}

desktop_up() {
  local started=
  if ! qemu_running; then
    echo "starting the VM"
    vmctl start
    started=1
  fi
  wait_for "VNC" 36 vnc_answers
  # UTM keeps the shared folder in its registry but stops serving it after
  # the VM starts; setting it again brings Z: back. Only after a start: set
  # on a VM already running, it left UTM's guest-agent RPC timing out until
  # the VM was powered off (2026-10-11).
  [[ -z $started ]] || vmctl "share $EXCHANGE" >/dev/null
  if desktop_ready; then
    echo "desktop already up"
  else
    wait_for "desktop" 48 desktop_ready
    sleep 10  # let auto-login settle before Win+R
  fi
}

# vmexec "CMDLINE" [TIMEOUT]: run in the guest and return the guest
# command's own exit code (vmctl itself succeeds whenever the agent answers),
# printing the output, or sending it to stderr when the command failed.
vmexec() {
  local out rc
  out=$(vmctl "exec $1" "${2:-60}") || true
  rc=$(sed -n '1s/^exit \([0-9][0-9]*\).*/\1/p' <<<"$out")
  if [[ ${rc:-1} != 0 ]]; then
    printf '%s\n' "$out" >&2
    return "${rc:-1}"
  fi
  printf '%s\n' "$out"
}

# The A0 route. The guest agent runs in session 0, which has no desktop, so
# the watcher and the full-screen game start as scheduled tasks on the
# logged-on user's desktop (vm/a0-tasks.ps1, registered by `a0-setup`, which
# also sets DDrawCompat's DisplayFilter=point); nothing is typed.
up_a0() {
  desktop_up
  a0_setup
  vmexec "schtasks /Run /TN A0Watcher" >/dev/null || { echo "A0Watcher did not start" >&2; exit 1; }
  a0_game
}

# Start the game task, or end the running game and start it again
# (take-command needs a fresh launch). The game runs one copy at a time, and
# one started some other way is not the task's, so it is ended by pid.
#
# Each launch picks DDrawCompat's scaling phase, and some come up with the
# repeated columns at 2 and 7 instead of 3 and 8, which A0 refuses (README,
# "A0 captures"). So after each launch a full-screen probe of the main menu
# is checked, and the game relaunched until the phase is the calibrated one,
# up to A0_PHASE_TRIES launches (default 5); A0_PHASE_TRIES=0 skips the check.
a0_game() {
  local try tries=${A0_PHASE_TRIES:-5} dir cols
  if (( tries == 0 )); then a0_launch; return; fi
  dir=$(mktemp -d)
  for ((try = 1; try <= tries; try++)); do
    a0_launch
    "$0" a0 phase-probe "$dir" FullClient=1 >/dev/null 2>&1 || { echo "phase probe failed" >&2; continue; }
    cols=$(python3 - "$dir/phase-probe.png" <<'EOF'
import sys
from PIL import Image
im = Image.open(sys.argv[1]).convert("RGB"); w, h = im.size; px = im.load()
print(*[x for x in range(1, 40) if all(px[x, y] == px[x - 1, y] for y in range(h))][:3])
EOF
)
    if [[ $cols == "3 8 13" ]]; then
      echo "scaling phase calibrated (repeated columns $cols) on launch $try"
      rm -rf "$dir"
      return 0
    fi
    echo "launch $try: repeated columns $cols, not 3 8 13; relaunching" >&2
  done
  rm -rf "$dir"
  echo "no calibrated launch in $tries tries" >&2
  return 1
}

a0_launch() {
  local pid
  for pid in $(vmctl "exec cmd /c tasklist /fo csv /nh | findstr /i REBEXE.EXE" 2>/dev/null | awk -F'","' '/REBEXE/ {print $2}'); do
    vmctl "exec taskkill /pid $pid /f" >/dev/null
  done
  vmexec "schtasks /Run /TN A0Game" >/dev/null || { echo "A0Game did not start" >&2; exit 1; }
  # A task's window does not get the focus: click the cockpit's ceiling,
  # which on the main menu is no control.
  vnc pause 10
  qmp desk 400 20 >/dev/null
  vnc pause 2 key esc pause 3 key esc pause 3 key esc pause 5
  mon 'mouse_set 1' >/dev/null
  echo "game launched full screen; watcher on $A0_DIR"
}

recover() {
  echo "killing QEMU and restarting UTM"
  ssh "$MINI" 'pkill -9 -f qemu-aarch64-softmmu; pkill -x UTM; sleep 3; open -a UTM; sleep 8'
  if [[ ${1:-} == a0 ]]; then up_a0; else up; fi
}

# One connection polls a region until it changes (vm-wait.py); the password
# goes through the environment, never argv.
wait_change() {
  local region=$1 n=1 limit=120 file
  shift
  [[ $# -ge 3 ]] && { n=$1; limit=$2; shift 2; }
  [[ $# -ge 2 ]] && { n=$1; shift; }
  file=$1
  VNC_PASSWORD=$(security find-internet-password -s "$VNC_HOST" -w 2>/dev/null || true) \
    uvx --quiet --with vncdotool --with pillow python "$(dirname "$0")/vm-wait.py" \
    "$VNC" "$region" "$n" "$limit" "$file"
}

cap_after() { [[ -z ${1:-} ]] || vnc pause 1 capture "$1"; }

# run-to-alert TITLE.png TIMEOUT FILE: let the game run until the Battle
# Alert whose title band matches TITLE.png opens, auto-resolving every other
# alert on the way (vm-wait.py alert-title). TITLE.png is any 800x600 frame
# of that alert; chrome.png beside it is any alert frame of the same side.
run_to_alert() {
  local title=$1 limit=$2 file=$3 start rc left
  start=$(date +%s)
  while :; do
    left=$(( limit - ($(date +%s) - start) ))
    (( left > 0 )) || { echo "run-to-alert: no matching alert in $limit s" >&2; return 1; }
    rc=0
    wait_change alert-title "$title" "$left" "$file" || rc=$?
    case $rc in
      0) return 0 ;;
      3) resolve ;;
      *) return "$rc" ;;
    esac
  done
}

# The game's cursor follows the tablet's relative motion at about 1.25x,
# clamped at the screen edge, so an absolute `move` jump lands off target by
# an amount that depends on where the cursor was. Walking the tablet to
# (0,0) in small steps pins the game cursor to the corner; walking out to
# target/1.25 then lands within 2 px across a 5x5 grid (2026-10-11). The
# earlier 1.28 fell 15-20 px short near the right and bottom edges.
STATE=${TMPDIR:-/tmp}/vm-game-tablet
GAIN=${VM_GAIN:-1.25}
walk_to() {
  local from
  from=$(cat "$STATE" 2>/dev/null || echo "625 470")
  # shellcheck disable=SC2046
  python3 - "$from" "$1" "$2" "$GAIN" > "$STATE.actions" <<'EOF'
import sys
x0, y0 = map(int, sys.argv[1].split())
tx, ty, gain = int(sys.argv[2]), int(sys.argv[3]), float(sys.argv[4])
goal = (round(tx / gain), round(ty / gain))
def path(a, b, step=4):
    n = max(1, int(max(abs(b[0] - a[0]), abs(b[1] - a[1])) / step))
    for i in range(1, n + 1):
        print("move", round(a[0] + (b[0] - a[0]) * i / n), round(a[1] + (b[1] - a[1]) * i / n), "pause 0.01", end=" ")
path((x0, y0), (0, 0))
path((0, 0), goal)
print()
print(*goal, file=sys.stderr)
EOF
}

# Walk to a guest-frame point, then press BUTTON (1 left, 3 right).
press() {
  local actions goal
  goal=$(walk_to "$1" "$2" 2>&1 >/dev/null)
  actions=$(cat "$STATE.actions")
  # shellcheck disable=SC2086
  vnc $actions pause 0.3 mousedown "$3" mouseup "$3"
  echo "$goal" > "$STATE"
}

# Press at the first point, walk the held button to the second, release.
# The game reads the drag from the tablet's motion, so the second leg is
# walked in the same small steps at the same gain.
drag() {
  local goal end actions
  goal=$(walk_to "$1" "$2" 2>&1 >/dev/null)
  actions=$(cat "$STATE.actions")
  end=$(python3 -c "import sys; g=float(sys.argv[3]); print(round(int(sys.argv[1])/g), round(int(sys.argv[2])/g))" "$3" "$4" "$GAIN")
  # shellcheck disable=SC2046,SC2086
  vnc $actions pause 0.3 mousedown 1 pause 0.3 $(python3 -c "
import sys
x0, y0, x1, y1 = map(int, sys.argv[1:5])
n = max(1, int(max(abs(x1 - x0), abs(y1 - y0)) / 3))
print(' '.join(f'move {round(x0 + (x1 - x0) * i / n)} {round(y0 + (y1 - y0) * i / n)} pause 0.02' for i in range(1, n + 1)))
" $goal $end) pause 0.3 mouseup 1
  echo "$end" > "$STATE"
}

# Move the pointer, then press: QMP by default, VNC with VM_INPUT=vnc.
pointer() {
  local what=$1 x=$2 y=$3
  if [[ $INPUT == vnc ]]; then
    case $what in
      point)
        goal=$(walk_to "$x" "$y" 2>&1 >/dev/null)
        # shellcheck disable=SC2046
        vnc $(cat "$STATE.actions")
        echo "$goal" > "$STATE"
        ;;
      click) press "$x" "$y" 1 ;;
      rclick) press "$x" "$y" 3 ;;
      dclick) pointer point "$x" "$y"; vnc click 1 pause 0.05 click 1 ;;
    esac
  else
    case $what in
      point) qmp point "$x" "$y" ;;
      click) qmp click "$x" "$y" ;;
      rclick) qmp click "$x" "$y" right ;;
      dclick) qmp point "$x" "$y"; vnc click 1 pause 0.05 click 1 ;;
    esac
  fi
}

# A guest command typed into Win+R. The VNC link drops Shift, so
# vnc-type.py's actions_for spells shifted symbols as chords. The shared
# folder is the guest's Z: drive (SPICE WebDAV, mapped by spice-webdavd).
run_guest() {
  local -a actions
  eval "actions=($(python3 - "$1" "$HERE/vnc-type.py" <<'EOF'
import importlib.util, shlex, sys
spec = importlib.util.spec_from_file_location("vnc_type", sys.argv[2])
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
print(shlex.join(module.actions_for(sys.argv[1])))
EOF
))"
  vnc key super-r pause 2 "${actions[@]}" key enter
}

# Saves move through the QEMU guest agent (an x64 qemu-ga under Windows'
# emulation; UTM's execute/pull/push), which works in the background and
# leaves the game focused. VM_XFER=webdav types a copy into Win+R instead,
# through the shared folder, and takes focus from the game.
slot_path() { printf 'C:\\Rebellion\\SaveGame\\SAVEGAME.%03d' "$1"; }

save_out() {
  local slot=$1 name=$2 n
  ssh "$MINI" "rm -f $EXCHANGE/saves-out/$name.sav"
  if [[ ${VM_XFER:-qga} == qga ]]; then
    ssh "$MINI" ": > $EXCHANGE/saves-out/$name.sav"
    vmctl "pull $(slot_path "$slot") $EXCHANGE/saves-out/$name.sav"
  else
    run_guest "cmd /c copy /y $(slot_path "$slot") z:\\saves-out\\$name.sav"
    for ((n = 0; n < 30; n++)); do
      ssh "$MINI" "test -s $EXCHANGE/saves-out/$name.sav" && break
      sleep 1
    done
  fi
  scp -q "$MINI:$EXCHANGE/saves-out/$name.sav" "$name.sav"
  shasum -a 256 "$name.sav"
  vmctl "exec certutil -hashfile $(slot_path "$slot") SHA256" | sed -n 3p
}

save_in() {
  local file=$1 slot=$2 base
  base=$(basename "$file")
  scp -q "$file" "$MINI:$EXCHANGE/saves-in/$base"
  shasum -a 256 "$file"
  if [[ ${VM_XFER:-qga} == qga ]]; then
    vmctl "push $EXCHANGE/saves-in/$base $(slot_path "$slot")"
  else
    run_guest "cmd /c copy /y z:\\saves-in\\$base $(slot_path "$slot")"
    sleep 3
  fi
  vmctl "exec certutil -hashfile $(slot_path "$slot") SHA256" | sed -n 3p
}

# Copy the scripts in and register the tasks (safe to repeat).
a0_setup() {
  a0_push
  vmexec "powershell -noprofile -executionpolicy bypass -file $A0_DIR\\a0-tasks.ps1" ||
    { echo "a0-tasks.ps1 failed" >&2; exit 1; }
}

# Run a PowerShell FILE on the guest's desktop (the A0Run task) and print its
# output: the guest agent's own session 0 has no display.
a0_run() {
  local n
  scp -q "$1" "$MINI:$EXCHANGE/a0/run.ps1"
  vmctl "exec cmd /c del /q $A0_DIR\\run.out $A0_DIR\\run.done" >/dev/null || true
  vmctl "push $EXCHANGE/a0/run.ps1 $A0_DIR\\run.ps1" >/dev/null
  vmexec "schtasks /Run /TN A0Run" >/dev/null || { echo "A0Run did not start" >&2; exit 1; }
  # run.done appears once run.out is complete; the listing names it.
  for ((n = 0; n < 12; n++)); do
    [[ $(vmctl "exec cmd /c dir /b $A0_DIR\\run.done" || true) == *run.done* ]] && break
    sleep 3
  done
  ((n < 12)) || { echo "A0Run did not finish in about 60 s" >&2; exit 1; }
  vmexec "cmd /c type $A0_DIR\\run.out"
}

# Restart the watcher loop (each capture already runs the current script).
a0_watch() {
  a0_push
  vmctl "exec powershell -noprofile -executionpolicy bypass -file $A0_DIR\\capture.ps1 -StopWatch" || true
  vmexec "schtasks /Run /TN A0Watcher" || { echo "A0Watcher did not start" >&2; exit 1; }
}


# A0 captures go through capture.ps1's watch mode (`up-a0`): a request JSON
# pushed into C:\a0\req is captured in the logged-on session while the game
# keeps the focus, and answered with C:\a0\out\NAME.done or NAME.err. The
# request is pushed beside the folder and moved in, so the watcher never
# reads half a file.
a0_push() {
  ssh "$MINI" "mkdir -p $EXCHANGE/a0"
  scp -q "$HERE/vm/capture.ps1" "$MINI:$EXCHANGE/a0/capture.ps1"
  vmctl "exec cmd /c mkdir $A0_DIR\\req $A0_DIR\\out" >/dev/null || true
  vmctl "push $EXCHANGE/a0/capture.ps1 $A0_DIR\\capture.ps1"
  scp -q "$HERE/vm/a0-tasks.ps1" "$MINI:$EXCHANGE/a0/a0-tasks.ps1"
  vmctl "push $EXCHANGE/a0/a0-tasks.ps1 $A0_DIR\\a0-tasks.ps1"
}

# pull_one FILE OUT [TIMEOUT]: copy C:\a0\out\FILE into OUT through the
# Mini; an empty copy means the agent's pull failed.
pull_one() {
  ssh "$MINI" ": > $EXCHANGE/a0/$1"
  vmctl "pull $A0_DIR\\out\\$1 $EXCHANGE/a0/$1" "${3:-60}" >/dev/null
  scp -q "$MINI:$EXCHANGE/a0/$1" "$2/$1"
  [[ -s $2/$1 ]] || { echo "pull of $1 came back empty" >&2; exit 1; }
}

# Fetch C:\a0\out\NAME.json, .png and what the JSON names into OUT.
a0_pull() {
  local name=$1 out=$2 ext extra
  rm -rf "$out/$name.png" "$out/$name.json" "$out/$name.screen.png" "$out/$name.burst.zip" "$out/$name.burst"
  pull_one "$name.json" "$out"
  # PowerShell 5 writes UTF-8 with a BOM, which JSON.parse (the ingester) rejects.
  python3 -c "import sys,pathlib; p=pathlib.Path(sys.argv[1]); p.write_text(p.read_text(encoding='utf-8-sig'), encoding='utf-8')" "$out/$name.json"
  # The metadata names what else to pull: a cell frame's 800x600 source, so
  # the rebuild can be rechecked, and a burst's frames in one zip. A probe's
  # frame is the screen itself (same hash), so it writes no separate source.
  extra=$(python3 -c "import json,sys; m=json.load(open(sys.argv[1])); print(*(['screen.png'] if m.get('screen_sha256') not in (None, m['sha256']) else []) + (['burst.zip'] if m.get('burst') else []))" "$out/$name.json")
  for ext in png $extra; do pull_one "$name.$ext" "$out" 180; done
  shasum -a 256 "$out/$name.png"
  # Every still the guest hashed is held to that hash, not only cell frames.
  python3 -c "import hashlib,json,sys; m=json.load(open(sys.argv[1])); h=hashlib.sha256(open(sys.argv[2],'rb').read()).hexdigest(); sys.exit(0 if h==m['sha256'] else 'frame '+h+' is not the one the metadata names')" "$out/$name.json" "$out/$name.png"
  [[ -f $out/$name.burst.zip ]] || return 0
  # Unpack to NAME.burst/ and hold every frame to the hashes the guest wrote.
  python3 - "$out/$name.json" "$out/$name.burst.zip" "$out/$name.burst" <<'EOF'
import hashlib, json, sys, zipfile
meta, archive, folder = json.load(open(sys.argv[1])), sys.argv[2], sys.argv[3]
frames = meta["burst"]["frames"]
zipfile.ZipFile(archive).extractall(folder)
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
for f in frames:
    stem = f"{folder}/f{f['index']:02d}"
    if sha(f"{stem}.png") != f["sha256"] or sha(f"{stem}.screen.png") != f["screen_sha256"]:
        sys.exit(f"burst frame {f['index']} does not match its metadata")
if frames[-1]["sha256"] != meta["sha256"]:
    sys.exit("the burst's last frame is not the capture's frame")
print(f"burst: {len(frames)} frames over {frames[-1]['t_ms']} ms")
EOF
}

# a0 NAME [OUTDIR] [Key=Value ...]: Keys are capture.ps1 parameters;
# InputTrace takes '|'-separated steps, FullClient=1 the uncropped probe.
a0_capture() {
  local name=$1 out=.artifacts/original-checks/a0 n listing
  shift
  if [[ $# -gt 0 && $1 != *=* ]]; then out=$1; shift; fi
  [[ $name =~ ^[A-Za-z0-9._-]+$ ]] || { echo "name: A-Za-z0-9._-" >&2; exit 2; }
  mkdir -p "$out"
  local rid
  rid=$(uuidgen)
  python3 - "$name" "$rid" "$@" > "$out/$name.request.json" <<'EOF'
import json, sys
name, rid, pairs = sys.argv[1], sys.argv[2], sys.argv[3:]
req = {"Name": name, "RequestId": rid}
for pair in pairs:
    key, _, value = pair.partition("=")
    req[key] = value.split("|") if key == "InputTrace" else value
if req.pop("FullClient", "") in ("1", "true"):
    req["FullClient"] = True
print(json.dumps(req))
EOF
  # The game draws its own cursor into the frame: park it in the corner, or
  # on a control for a hover (A0_PARK="X Y"). A0_PARK=none leaves it where
  # it is, as a held button (`press`) needs: a park walk would drag.
  [[ $A0_PARK == none ]] || qmp point "${A0_PARK% *}" "${A0_PARK#* }" >/dev/null
  scp -q "$out/$name.request.json" "$MINI:$EXCHANGE/a0/$name.request.json"
  vmctl "push $EXCHANGE/a0/$name.request.json $A0_DIR\\$name.tmp" >/dev/null
  # A stale answer from an earlier request of the same name would end the wait.
  vmctl "exec cmd /c del /q $A0_DIR\\out\\$name.*" >/dev/null || true
  vmexec "cmd /c move /y $A0_DIR\\$name.tmp $A0_DIR\\req\\$name.json" >/dev/null ||
    { echo "request $name did not reach the watcher" >&2; exit 1; }
  # A0_ON_START="click 717 397": once a burst has taken its first frame
  # (capture.ps1 writes NAME.started), send that vm-game.sh command, so an
  # input such as an unpause falls inside the burst instead of before it.
  if [[ -n ${A0_ON_START:-} ]]; then
    local started=
    for ((n = 0; n < 6; n++)); do
      vmctl "exec powershell -noprofile -command for(\$i=0;\$i -lt 40 -and -not (Test-Path $A0_DIR\\out\\$name.started) -and -not (Test-Path $A0_DIR\\out\\$name.err);\$i++){Start-Sleep -Milliseconds 100}" 20 >/dev/null || true
      listing=$(vmctl "exec cmd /c dir /b $A0_DIR\\out\\$name.*" || true)
      [[ $listing == *"$name.started"* ]] && { started=1; break; }
      [[ $listing == *"$name.err"* ]] && break
    done
    if [[ -n $started ]]; then
      # shellcheck disable=SC2086
      "$0" $A0_ON_START
      echo "A0_ON_START sent after the burst's first frame: $A0_ON_START"
    else
      echo "A0_ON_START not sent: the burst did not start" >&2
    fi
  fi
  # Wait in the guest, in short calls: one agent call held for 30 s made UTM
  # report "Timed out waiting for RPC" (2026-10-11).
  # Ten rounds (about 80 s), plus a burst's own length and its encoding
  # (two PNGs a frame and the zip) at about a second a frame.
  local rounds
  rounds=$(python3 -c "import json,sys; r=json.load(open(sys.argv[1])); b=int(r.get('Burst') or 0); print(10 + (b * int(r.get('IntervalMs') or 250) + b * 1000) // 8000 + 1 if b > 0 else 10)" "$out/$name.request.json")
  for ((n = 0; n < rounds; n++)); do
    vmctl "exec powershell -noprofile -command for(\$i=0;\$i -lt 40 -and -not (Test-Path $A0_DIR\\out\\$name.done) -and -not (Test-Path $A0_DIR\\out\\$name.err);\$i++){Start-Sleep -Milliseconds 200}" 20 >/dev/null || true
    listing=$(vmctl "exec cmd /c dir /b $A0_DIR\\out\\$name.*" || true)
    [[ $listing == *"$name.done"* || $listing == *"$name.err"* ]] && break
  done
  if [[ $listing == *"$name.err"* ]]; then
    vmctl "exec cmd /c type $A0_DIR\\out\\$name.err" >&2
    exit 1
  fi
  [[ $listing == *"$name.done"* ]] || { echo "no answer from the watcher in $((rounds * 8)) s" >&2; exit 1; }
  a0_pull "$name" "$out"
  # A late answer to an earlier request of the same name carries another id.
  python3 -c "import json,sys; m=json.load(open(sys.argv[1])); sys.exit(0 if m.get('request_id')==sys.argv[2] else 'answer is for request '+str(m.get('request_id'))+', not '+sys.argv[2])" "$out/$name.json" "$rid"
  python3 -c "import json,sys; m=json.load(open(sys.argv[1], encoding='utf-8-sig')); print(m['width'], m['height'], 'client', m.get('client_size'), 'guest sha', m['sha256'])" "$out/$name.json"
  if [[ -f $out/$name.screen.png ]]; then
    python3 -c "import hashlib,json,sys; m=json.load(open(sys.argv[1])); h=hashlib.sha256(open(sys.argv[2],'rb').read()).hexdigest(); sys.exit(0 if h==m.get('screen_sha256') else 'screen frame '+h+' is not the one the metadata names')" "$out/$name.json" "$out/$name.screen.png"
  fi
}

# a0-cell CELL KIND SETUP 'step|step|...' [NOTES]: capture a ledger cell and
# register it. The requirement comes from the surface ledger, so it always
# matches; the ingester then checks the PNG, hashes and provenance and writes
# only to .artifacts/interface-parity/a0/. A0_BEFORE=NAME pairs the cell with
# an earlier capture (`a0 NAME`) of the same setup, for a cell about a
# transition.
a0_cell() {
  local cell=$1 kind=$2 setup=$3 trace=$4 notes=${5:-} req root out
  root=$(cd "$HERE/../.." && pwd)
  out=$root/.artifacts/original-checks/a0
  req=$(python3 - "$root" "$cell" <<'EOF'
import json, sys
ledger = json.load(open(f"{sys.argv[1]}/docs/qa/2026-09-10-interface-parity-audit/2026-10-06-surface-ledger.json"))
found = {c["id"]: c["requirement"] for s in ledger["surfaces"] for c in s.get("acceptance_cells", [])}
if sys.argv[2] not in found:
    sys.exit(f"{sys.argv[2]}: not in the surface ledger")
try:
    manifest = json.load(open(f"{sys.argv[1]}/.artifacts/interface-parity/a0/manifest.json"))
except FileNotFoundError:
    manifest = {"captures": []}
# Capturing first would overwrite the registered cell's working files.
if any(c.get("cell_id") == sys.argv[2] for c in manifest.get("captures", [])):
    sys.exit(f"{sys.argv[2]}: already registered in the A0 manifest")
print(found[sys.argv[2]])
EOF
)
  # A0_BURST="N MS": N frames MS apart instead of one stable frame, for a
  # cell about motion.
  local -a burst=()
  [[ -n ${A0_BURST:-} ]] && burst=(Burst="${A0_BURST% *}" IntervalMs="${A0_BURST#* }")
  a0_capture "$cell" "$out" CellId="$cell" Requirement="$req" StateSetupKind="$kind" \
    StateSetupIdentifier="$setup" StateSetupNotes="$notes" InputTrace="$trace" \
    ${burst[@]+"${burst[@]}"}
  a0_register "$cell"
}

# a0-register CELL [FRAME]: register a pulled capture
# (.artifacts/original-checks/a0/CELL.*) with the ingester, as a0-cell does.
# FRAME names the burst frame that stands for the cell, when the event is
# not the burst's last frame (a kill the original holds for 3 s before the
# results screen). A0_BEFORE=NAME pairs it with an earlier `a0` frame.
a0_register() {
  local cell=$1 frame=${2:-} root a0=.artifacts/original-checks/a0
  local -a extra=()
  root=$(cd "$HERE/../.." && pwd)
  [[ -n ${A0_BEFORE:-} ]] && extra=(--before="$a0/$A0_BEFORE.png" --before-metadata="$a0/$A0_BEFORE.json")
  [[ -d $root/$a0/$cell.burst ]] && extra+=(--burst="$a0/$cell.burst")
  [[ -n $frame ]] && extra+=(--cell-frame="$frame")
  [[ -f $root/$a0/$cell.wav ]] && extra+=(--audio="$a0/$cell.wav" --audio-metadata="$a0/$cell.wav.json")
  (cd "$root" && node tools/interface-parity/ingest-tactical-a0.mjs \
    --manifest=.artifacts/interface-parity/a0/manifest.json \
    --png="$a0/$cell.png" --metadata="$a0/$cell.json" \
    ${extra[@]+"${extra[@]}"}) |
    python3 -c "import json,sys; r=json.load(sys.stdin); print(r['status'], r['cell_id'], r['sha256'][:12], 'a0', r.get('a0_coverage'))"
}

# a0-audio start|stop CELL: record the guest's sound for a cell. QEMU's
# wavcapture taps UTM's audiodev (spice, id audio0) on the Mini; `stop`
# ends it, pulls CELL.wav beside the cell's capture, checks both hashes and
# writes CELL.wav.json (hash, start and stop times, source), which a0-register then
# passes to the ingester. Start before the cell's frame, stop after it.
a0_audio() {
  local action=$1 cell=$2 a0 remote index started stopped local_sha remote_sha
  a0=$(cd "$HERE/../.." && pwd)/.artifacts/original-checks/a0
  remote=$EXCHANGE/a0/$cell.wav
  [[ $cell =~ ^TAC-0[1-8]-C[0-9]{3}$ ]] || { echo "a0-audio: $cell is not a cell id" >&2; exit 2; }
  case $action in
    start)
      started=$(date -u +%Y-%m-%dT%H:%M:%SZ)
      mon "wavcapture $remote audio0 44100 16 2" >/dev/null
      mon "info capture" | grep -q -- "$remote" || { echo "a0-audio: capture did not start" >&2; exit 1; }
      printf '%s\n' "$started" >"$a0/$cell.wav.started"
      echo "recording $cell from $started"
      ;;
    stop)
      index=$(mon "info capture" | grep -- "$remote" | sed -n 's/^\[\([0-9]*\)\].*/\1/p' | head -1)
      [[ -n $index ]] || { echo "a0-audio: no capture of $cell is running" >&2; exit 1; }
      [[ -f $a0/$cell.wav.started ]] || { echo "a0-audio: no start time for $cell" >&2; exit 1; }
      stopped=$(date -u +%Y-%m-%dT%H:%M:%SZ)
      mon "stopcapture $index" >/dev/null
      scp -q "$MINI:$remote" "$a0/$cell.wav"
      local_sha=$(shasum -a 256 "$a0/$cell.wav" | cut -d' ' -f1)
      remote_sha=$(ssh "$MINI" "shasum -a 256 $remote" | cut -d' ' -f1)
      [[ $local_sha == "$remote_sha" ]] || { echo "a0-audio: $cell.wav changed in transfer" >&2; exit 1; }
      python3 -c 'import json,sys; json.dump({"sha256": sys.argv[1], "started_at_utc": sys.argv[2], "stopped_at_utc": sys.argv[3], "source": "QEMU wavcapture of audiodev audio0 (spice), 44100 Hz 16-bit stereo, on the capture VM host"}, open(sys.argv[4], "w"), indent=2)' \
        "$local_sha" "$(cat "$a0/$cell.wav.started")" "$stopped" "$a0/$cell.wav.json"
      rm "$a0/$cell.wav.started"
      echo "$local_sha  $a0/$cell.wav"
      ;;
    *) echo "a0-audio start|stop CELL" >&2; exit 2 ;;
  esac
}

# Game Options (F1), Saved Games: the six rows' centre y, each row's name
# field and load button x, and the two dialogs' buttons (2026-10-11).
ROWS=(113 166 218 270 323 376)
NAME_X=245 LOAD_X=384
YES=(350 377)       # the check mark of "Load without saving?" / "Save anyway?"
SAVED_OK=(398 375)  # "Selected Game Saved Successfully"; closes Game Options
row_y() {
  [[ $1 =~ ^[1-6]$ ]] || { echo "slot $1: Saved Games shows rows 1-6" >&2; exit 2; }
  echo "${ROWS[$1 - 1]}"
}

load_slot() {
  local y
  y=$(row_y "$1")
  vnc key f1 pause 1
  pointer click "$LOAD_X" "$y"
  pointer click "${YES[@]}"
  # A day-30 save shows the galaxy view 8-11 s after the check mark.
  vnc pause 12
  cap_after "${2:-}"
}

save_slot() {
  local y bsp=()
  y=$(row_y "$1")
  [[ $2 =~ ^[a-z0-9\ -]{1,31}$ ]] || { echo "name: lowercase letters, digits, space, hyphen (VNC drops Shift)" >&2; exit 2; }
  for ((i = 0; i < 40; i++)); do bsp+=(key bsp); done
  vnc key f1 pause 1
  pointer click "$NAME_X" "$y"
  vnc key end "${bsp[@]}" type "$2" key enter pause 1
  pointer click "${YES[@]}"
  vnc pause 2
  pointer click "${SAVED_OK[@]}"
}

# Auto-resolve is the alert's middle button. The result box ignores an
# Escape sent right after the click, so wait for it to draw.
resolve() {
  pointer click 420 450
  vnc pause 3 key esc pause 1
  cap_after "${1:-}"
}

# The Battle Alert's buttons are Retreat (255,450), auto-resolve (420,450)
# and Take Command (590,450), by their tooltips (2026-10-11). The battlefield
# opens paused. Take Command crashed a game that had loaded about ten saves
# in one run (rebexe.exe access violation, 2026-10-11) and worked after a
# fresh `up`, so start a tactical run from a fresh launch.
take_command() {
  pointer click 590 450
  vnc pause 12
  cap_after "${1:-}"
}

# Battle Options (770,398), Withdraw From Battle (656,78), its confirmation's
# check mark (702,301), then the pause button (717,397) to let the battle
# play out. The Results box follows; `wait center` catches it.
withdraw() {
  pointer click 770 398
  vnc pause 1
  pointer click 656 78
  vnc pause 2
  pointer click 702 301
  vnc pause 1
  pointer click 717 397
  cap_after "${1:-}"
}

# Each line is one vm-game.sh command, word-split by the shell so quotes and
# $OUT work. The file is ours; never run one from elsewhere.
run_file() {
  local file=$1 line n=0
  local -a args
  OUT=${2:-$(cd "$(dirname "$file")" && pwd)}
  export OUT
  mkdir -p "$OUT"
  while IFS= read -r line || [[ -n $line ]]; do
    n=$((n + 1))
    [[ $line =~ ^[[:space:]]*(#|$) ]] && continue
    eval "args=($line)"
    echo "[$n] $line"
    # ssh would otherwise read the rest of the file as its stdin.
    "$0" "${args[@]}" < /dev/null || { echo "stopped at line $n" >&2; return 1; }
  done < "$file"
}

cmd=${1:-status}
shift || true
case $cmd in
  status)
    if qemu_running; then echo "QEMU: running"; else echo "QEMU: not running"; fi
    if vnc_answers; then echo "VNC: answering"; else echo "VNC: no answer"; fi
    mon 'info mice' | grep Mouse || true
    ;;
  up) up ;;
  up-a0) up_a0 ;;
  a0-game) a0_game ;;
  a0-push) a0_push ;;
  a0-setup) a0_setup ;;
  a0-watch) a0_watch ;;
  a0-run) a0_run "$1" ;;
  a0) a0_capture "$@" ;;
  a0-cell) a0_cell "$@" ;;
  a0-register) a0_register "$@" ;;
  a0-audio) a0_audio "$@" ;;
  recover) recover "${1:-}" ;;
  cap) vnc capture "$1" ;;
  point|click|rclick|dclick) pointer "$cmd" "$1" "$2"; cap_after "${3:-}" ;;
  drag)
    if [[ $INPUT == vnc ]]; then drag "$1" "$2" "$3" "$4"; else qmp drag "$1" "$2" "$3" "$4"; fi
    cap_after "${5:-}"
    ;;
  desk) qmp desk "$1" "$2"; cap_after "${3:-}" ;;
  press)
    if [[ ${3:-} == right ]]; then qmp press "$1" "$2" right; cap_after "${4:-}"
    else qmp press "$1" "$2"; cap_after "${3:-}"; fi
    ;;
  release) qmp release ${1:+"$1"} ;;
  key) vnc key "$1"; cap_after "${2:-}" ;;
  vnc) vnc "$@" ;;
  wait) wait_change "$@" ;;
  run-to-alert) run_to_alert "$1" "$2" "$3" ;;
  save-out) save_out "$1" "$2" ;;
  save-in) save_in "$1" "$2" ;;
  gexec)
    # Exit with the guest command's code, or 1 when no "exit N" line came back.
    out=$(vmctl "exec $1" "${2:-60}") || true
    printf '%s\n' "$out"
    rc=$(sed -n '1s/^exit \([0-9][0-9]*\).*/\1/p' <<<"$out")
    exit "${rc:-1}"
    ;;
  mon) mon "$1" ;;
  load) load_slot "$1" "${2:-}" ;;
  save) save_slot "$1" "$2" ;;
  resolve) resolve "${1:-}" ;;
  take-command) take_command "${1:-}" ;;
  withdraw) withdraw "${1:-}" ;;
  run) run_file "$1" "${2:-}" ;;
  *) sed -n '2,/^set -euo/p' "$0" | sed '$d'; exit 2 ;;
esac
