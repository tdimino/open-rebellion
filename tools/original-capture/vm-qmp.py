"""Drive the capture VM's pointer over QEMU's QMP socket.

Runs on the Mac Mini, where QMP listens on loopback only
(`-qmp tcp:127.0.0.1:4446`); vm-game.sh pipes it over ssh:

    ssh mini python3 - point X Y          < vm-qmp.py
    ssh mini python3 - click X Y [right]
    ssh mini python3 - drag X1 Y1 X2 Y2
    ssh mini python3 - desk X Y            (Windows desktop: jump and click)
    ssh mini python3 - screendump FILE.ppm
    ssh mini python3 - hmp "COMMAND"      (any QEMU monitor command)

The game moves its own cursor by the tablet's relative motion at about
1.25x, clamped to the screen, so an absolute jump lands off target by an
amount that depends on where the cursor was. Every pointer command walks
the tablet to (0,0) in small steps, which pins the cursor to the corner,
then walks out to target/1.25; on a 5x5 grid this lands within 2 px
(2026-10-11). The tablet position is kept in STATE between calls.

There is no double-click here. Two clicks sent over QMP after a QMP walk
never registered as a double-click in the game (0 of 30, holds and gaps
from 30 to 200 ms), while two VNC clicks after the same walk did 20 of 20,
so vm-game.sh presses double-clicks over VNC.
"""
import json
import os
import socket
import sys
import time

PORT = 4446
WIDTH, HEIGHT = 800, 600
GAIN = float(os.environ.get("VM_GAIN", "1.25"))
STEP = 4
HOLD = 0.05
STATE = os.path.expanduser("~/VMs/rebellion/qmp-tablet.state")


class Qmp:
    def __init__(self):
        self.sock = socket.create_connection(("127.0.0.1", PORT), timeout=30)
        self.file = self.sock.makefile("rw")
        self.file.readline()
        self.call("qmp_capabilities")

    def call(self, command, **arguments):
        message = {"execute": command}
        if arguments:
            message["arguments"] = arguments
        self.file.write(json.dumps(message) + "\n")
        self.file.flush()
        while True:
            reply = json.loads(self.file.readline())
            if "event" in reply:
                continue
            if "error" in reply:
                sys.exit(f"{command}: {reply['error']['desc']}")
            return reply["return"]

    def move(self, x, y):
        # Absolute axes span 0..0x7fff across the frame.
        self.call("input-send-event", events=[
            {"type": "abs", "data": {"axis": "x", "value": round(x * 0x7FFF / (WIDTH - 1))}},
            {"type": "abs", "data": {"axis": "y", "value": round(y * 0x7FFF / (HEIGHT - 1))}},
        ])
        time.sleep(0.01)

    def button(self, name, down):
        self.call("input-send-event", events=[
            {"type": "btn", "data": {"down": down, "button": name}}])

    def walk(self, start, goal):
        n = max(1, int(max(abs(goal[0] - start[0]), abs(goal[1] - start[1])) / STEP))
        for i in range(1, n + 1):
            self.move(round(start[0] + (goal[0] - start[0]) * i / n),
                      round(start[1] + (goal[1] - start[1]) * i / n))


def tablet():
    try:
        with open(STATE) as f:
            return tuple(map(int, f.read().split()))
    except (OSError, ValueError):
        return (WIDTH - 1, HEIGHT - 1)


def remember(point):
    with open(STATE, "w") as f:
        f.write(f"{point[0]} {point[1]}\n")


def scaled(x, y):
    return (round(int(x) / GAIN), round(int(y) / GAIN))


def point(qmp, x, y):
    goal = scaled(x, y)
    qmp.walk(tablet(), (0, 0))
    qmp.walk((0, 0), goal)
    remember(goal)
    return goal


def click(qmp, name):
    time.sleep(0.3)
    qmp.button(name, True)
    time.sleep(HOLD)
    qmp.button(name, False)


def main(argv):
    command = argv[0]
    qmp = Qmp()
    if command == "point":
        point(qmp, argv[1], argv[2])
    elif command == "click":
        point(qmp, argv[1], argv[2])
        click(qmp, "right" if "right" in argv[3:] else "left")
    elif command == "desk":
        # Outside the game, Windows' own cursor follows the tablet exactly.
        qmp.move(int(argv[1]), int(argv[2]))
        click(qmp, "left")
        remember((int(argv[1]), int(argv[2])))
    elif command == "drag":
        start = point(qmp, argv[1], argv[2])
        time.sleep(0.3)
        qmp.button("left", True)
        time.sleep(0.3)
        end = scaled(argv[3], argv[4])
        qmp.walk(start, end)
        time.sleep(0.3)
        qmp.button("left", False)
        remember(end)
    elif command == "screendump":
        qmp.call("screendump", filename=argv[1], format="ppm")
    elif command == "hmp":
        # savevm/loadvm of a few GiB outlast the connect timeout; the call
        # returns only once the command has finished.
        qmp.sock.settimeout(300)
        print(qmp.call("human-monitor-command", **{"command-line": argv[1]}), end="")
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main(sys.argv[1:])
