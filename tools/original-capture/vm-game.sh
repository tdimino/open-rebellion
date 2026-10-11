#!/usr/bin/env bash
# Drive the original game on the Mac Mini capture VM from this Mac.
#
#   vm-game.sh status            QEMU process, VNC reachability, monitor mice
#   vm-game.sh up                start the VM if needed, wait for the desktop,
#                                launch REBEXE, accept UAC, skip the intros and
#                                switch to the absolute tablet
#   vm-game.sh recover           kill a wedged QEMU, restart UTM, then `up`
#   vm-game.sh cap FILE          capture the screen to FILE
#   vm-game.sh click X Y [FILE]  left-click (tablet), optionally capture after
#   vm-game.sh rclick X Y [FILE] right-click, optionally capture after
#   vm-game.sh drag X1 Y1 X2 Y2 [FILE]  left-drag between two points
#   vm-game.sh key KEY [FILE]    send a vncdo key (esc, f1, alt-y, ...)
#   vm-game.sh vnc ACTIONS...    raw vncdo actions
#   vm-game.sh wait REGION [N] [TIMEOUT] FILE
#                                hold one VNC connection until REGION (day,
#                                center or X,Y,W,H) changes N times (default
#                                1, timeout 120 s), then capture to FILE
#   vm-game.sh mon "COMMAND"     run a QEMU monitor command
#
# The VNC password comes from the keychain and is never printed. Coordinates
# are the 800x600 guest frame a capture shows. See README.md, "Saves" for the
# pointer and focus caveats.
set -euo pipefail

MINI=${VM_MINI:-tamarru@mac-mini-ts}
VNC_HOST=100.125.235.4
VNC=${VNC_HOST}::5901
MONITOR_PORT=4445
VMDIR='~/VMs/rebellion'

vnc() {
  local pass
  pass=$(security find-internet-password -s "$VNC_HOST" -w 2>/dev/null || true)
  timeout "${VNC_TIMEOUT:-60}" uvx --quiet --from vncdotool vncdo -s "$VNC" ${pass:+-p "$pass"} "$@"
}

mon() {
  ssh "$MINI" "(echo '$1'; sleep 1) | nc 127.0.0.1 $MONITOR_PORT" \
    | tr -d '\033' | grep -v -e '^QEMU' -e '\[K' || true
}

qemu_running() {
  ssh "$MINI" 'pgrep -f QEMUHelper.xpc/Contents/MacOS/QEM >/dev/null'
}

vm_ctl() {
  ssh "$MINI" "echo $1 > $VMDIR/vm-ctl.cmd; launchctl kickstart gui/501/com.openrebellion.vmctl; sleep 3; tail -1 $VMDIR/vm-ctl.log"
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

SHOT=$(mktemp -t vm-game).png
trap 'rm -f "$SHOT"' EXIT
vnc_answers() { VNC_TIMEOUT=20 vnc capture "$SHOT" >/dev/null 2>&1; }
desktop_ready() { vnc_answers && taskbar_up "$SHOT"; }

up() {
  if ! qemu_running; then
    echo "starting the VM"
    vm_ctl start
  fi
  wait_for "VNC" 36 vnc_answers
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
  ssh "$MINI" 'pkill -9 -f QEMUHelper.xpc/Contents/MacOS/QEM; pkill -x UTM; sleep 3; open -a UTM; sleep 8'
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

cap_after() { [[ -n ${1:-} ]] && vnc pause 1 capture "$1" || true; }

# The game's cursor follows the tablet's relative motion at about 1.28x
# (1024/800), clamped at the screen edge, so an absolute `move` jump lands
# off target by an amount that depends on where the cursor was. Walking the
# tablet to (0,0) in small steps pins the game cursor to the corner; walking
# out to target/1.28 then lands on target (2026-10-10, Load a Saved Game
# disc at 535,315 hit from the corner).
STATE=${TMPDIR:-/tmp}/vm-game-tablet
GAIN=1.28
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
  if [[ ${4:-} == double ]]; then
    # shellcheck disable=SC2086
    vnc $actions pause 0.3 mousedown "$3" mouseup "$3" pause 0.1 mousedown "$3" mouseup "$3"
  else
    # shellcheck disable=SC2086
    vnc $actions pause 0.3 mousedown "$3" mouseup "$3"
  fi
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
  click) press "$1" "$2" 1; cap_after "${3:-}" ;;
  rclick) press "$1" "$2" 3; cap_after "${3:-}" ;;
  dclick) press "$1" "$2" 1 double; cap_after "${3:-}" ;;
  drag) drag "$1" "$2" "$3" "$4"; cap_after "${5:-}" ;;
  point)
    goal=$(walk_to "$1" "$2" 2>&1 >/dev/null)
    # shellcheck disable=SC2046
    vnc $(cat "$STATE.actions")
    echo "$goal" > "$STATE"
    cap_after "${3:-}"
    ;;
  key) vnc key "$1"; cap_after "${2:-}" ;;
  vnc) vnc "$@" ;;
  wait) wait_change "$@" ;;
  mon) mon "$1" ;;
  *) sed -n '2,21p' "$0"; exit 2 ;;
esac
