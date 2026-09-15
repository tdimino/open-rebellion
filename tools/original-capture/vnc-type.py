#!/usr/bin/env python3
"""Type a command line into the QEMU VNC guest with vncdotool, sending shifted
symbols as explicit shift-key chords.

QEMU's VNC server maps keysyms to scancodes without synthesising the shift
modifier, so `vncdo type 'D:\\setup.exe & x'` arrives as `D;\\setup.exe 7 x`.
This wrapper splits the text into unshifted runs (sent with `type`) and shifted
symbols (sent as `key shift-<base>`), then appends the caller's trailing vncdo
actions (for example `key enter pause 4 capture out.png`).

Usage:
  vnc-type.py <host::port> "<text>" [extra vncdo actions...]
"""
import os
import subprocess
import sys

SHIFTED = {
    "!": "1", "@": "2", "#": "3", "$": "4", "%": "5", "^": "6", "&": "7",
    "*": "8", "(": "9", ")": "0", "+": "=", "{": "[", "}": "]", "|": "\\",
    ":": ";", '"': "'", "<": ",", ">": ".", "?": "/", "~": "`",
}
# vncdotool resolves single characters through ord(), so the base key is the
# unshifted character itself. "_" cannot be spelled "shift--" (the dash is
# vncdo's modifier separator), so it is sent as an explicit keydown/keyup pair.


def actions_for(text: str) -> list[str]:
    out: list[str] = []
    run = ""
    for ch in text:
        if ch in SHIFTED or ch == "_":
            if run:
                out += ["type", run]
                run = ""
            if ch == "_":
                out += ["keydown", "shift", "key", "-", "keyup", "shift"]
            else:
                out += ["key", f"shift-{SHIFTED[ch]}"]
        else:
            run += ch
    if run:
        out += ["type", run]
    return out


def main() -> int:
    if len(sys.argv) < 3:
        print(__doc__, file=sys.stderr)
        return 2
    server, text, extra = sys.argv[1], sys.argv[2], sys.argv[3:]
    vncdo = os.path.expanduser("~/.local/bin/vncdo")
    cmd = [vncdo, "-s", server, "--timeout", "120", *actions_for(text), *extra]
    return subprocess.call(cmd, stdin=subprocess.DEVNULL)


if __name__ == "__main__":
    sys.exit(main())
