#!/usr/bin/env bash
# Drive the original game on the Mac Mini capture VM from this Mac.
#
#   vm-game.sh status            QEMU process, VNC reachability, monitor mice
#   vm-game.sh up                start the VM if needed, wait for the desktop,
#                                launch REBEXE, accept UAC, skip the intros and
#                                switch to the absolute tablet
#   vm-game.sh recover           kill a wedged QEMU, restart UTM, then `up`
#   vm-game.sh cap FILE          capture the screen to FILE
#   vm-game.sh point X Y [FILE]  move the game cursor to X,Y
#   vm-game.sh click X Y [FILE]  left-click, optionally capture after
#   vm-game.sh rclick X Y [FILE] right-click, optionally capture after
#   vm-game.sh dclick X Y [FILE] double-click
#   vm-game.sh drag X1 Y1 X2 Y2 [FILE]  left-drag between two points
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
vmctl() { ssh "$MINI" "$VMDIR/vmctl-run.sh $(printf '%q' "$1") ${2:-60}"; }

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
  # Win+R C:\rebellion\rebexe.exe; the VNC link drops Shift, so ':' is a chord.
  vnc key super-r pause 2 type c key 'shift-;' type '\rebellion\rebexe.exe' key enter pause 5 key alt-y pause 15 \
    key esc pause 3 key esc pause 3 key esc pause 5
  mon 'mouse_set 1' >/dev/null
  echo "game launched; tablet absolute"
}

recover() {
  echo "killing QEMU and restarting UTM"
  ssh "$MINI" 'pkill -9 -f qemu-aarch64-softmmu; pkill -x UTM; sleep 3; open -a UTM; sleep 8'
  up
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
  recover) recover ;;
  cap) vnc capture "$1" ;;
  point|click|rclick|dclick) pointer "$cmd" "$1" "$2"; cap_after "${3:-}" ;;
  drag)
    if [[ $INPUT == vnc ]]; then drag "$1" "$2" "$3" "$4"; else qmp drag "$1" "$2" "$3" "$4"; fi
    cap_after "${5:-}"
    ;;
  desk) qmp desk "$1" "$2"; cap_after "${3:-}" ;;
  key) vnc key "$1"; cap_after "${2:-}" ;;
  vnc) vnc "$@" ;;
  wait) wait_change "$@" ;;
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
  *) sed -n '2,57p' "$0"; exit 2 ;;
esac
