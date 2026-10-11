"""Hold one VNC connection and wait for a region of the guest screen to change.

    vm-wait.py SERVER REGION COUNT TIMEOUT FILE

Run by `vm-game.sh wait`, which passes the password in VNC_PASSWORD so it
never appears in argv. REGION is a name below or `X,Y,W,H` in the 800x600
guest frame. The script takes a baseline crop, polls about twice a second,
and counts a change each time the crop differs from the previous one. After
COUNT changes it saves the full frame to FILE and the crop beside it
(`FILE` with `-region` before the extension), prints the elapsed time and
exits 0. On TIMEOUT seconds it saves the frame it has and exits 1.
REGION `alert` instead waits until a Battle Alert is open (COUNT is
ignored) and returns at once if one already is.

Polling one open connection replaces a fresh `vncdo capture` per frame, and
returning on a change replaces sleep-and-capture loops.
"""

import os
import sys
import time
from pathlib import Path

from PIL import ImageChops, ImageStat
from vncdotool import api

REGIONS = {
    # The galaxy view's day readout (two digits at x 662-668 on day 24-28,
    # 2026-10-11); wide enough for three digits.
    "day": (622, 22, 80, 18),
    # The middle of the screen, where the Battle Alert and message dialogs open.
    "center": (150, 120, 500, 340),
}
# Mean absolute difference (0-255) above which a crop counts as changed.
# Galaxy twinkle in the larger boxes stays below 2.0. Day digit flips score
# 1.86-5.78 and the day box is otherwise 0.00 exactly (2026-10-11), so it
# takes a lower bar.
THRESHOLD = 2.0
THRESHOLDS = {"day": 0.5}
POLL = 0.5

# `alert` waits for a state, not a change, so an alert already open when the
# wait starts still counts. The Battle Alert's middle (auto-resolve) button
# has 190 bright green pixels in this box; no other screen captured on
# 2026-10-11 had more than 62.
ALERT_BOX = (400, 438, 445, 462)
ALERT_MIN = 150


def alert_open(frame):
    data = frame.convert("RGB").crop(ALERT_BOX).tobytes()
    pixels = zip(data[0::3], data[1::3], data[2::3])
    return sum(1 for r, g, b in pixels if g > 180 and r < 120 and b < 120) >= ALERT_MIN


def grab(client, box):
    client.refreshScreen()
    frame = client.screen.copy()
    x, y, w, h = box
    return frame, frame.crop((x, y, x + w, y + h)).convert("L")


def main():
    if len(sys.argv) != 6:
        sys.exit(__doc__)
    server, region, count, timeout, out = sys.argv[1:]
    if region == "alert":
        return wait_alert(server, float(timeout), Path(out))
    box = REGIONS.get(region) or tuple(int(v) for v in region.split(","))
    threshold = THRESHOLDS.get(region, THRESHOLD)
    count, timeout, out = int(count), float(timeout), Path(out)
    client = api.connect(server, password=os.environ.get("VNC_PASSWORD") or None)
    try:
        start = time.monotonic()
        frame, last = grab(client, box)
        seen = 0
        while seen < count and time.monotonic() - start < timeout:
            time.sleep(POLL)
            frame, crop = grab(client, box)
            if ImageStat.Stat(ImageChops.difference(crop, last)).mean[0] > threshold:
                seen += 1
                last = crop
        frame.save(out)
        frame.crop((box[0], box[1], box[0] + box[2], box[1] + box[3])).save(
            out.with_name(f"{out.stem}-region{out.suffix}"))
        elapsed = time.monotonic() - start
        print(f"{region}: {seen}/{count} changes in {elapsed:.1f}s -> {out}")
        return 0 if seen >= count else 1
    finally:
        client.disconnect()
        api.shutdown()


def wait_alert(server, timeout, out):
    client = api.connect(server, password=os.environ.get("VNC_PASSWORD") or None)
    try:
        start = time.monotonic()
        while True:
            client.refreshScreen()
            frame = client.screen.copy()
            found = alert_open(frame)
            if found or time.monotonic() - start >= timeout:
                break
            time.sleep(POLL)
        frame.save(out)
        state = "open" if found else "not open"
        print(f"alert: {state} after {time.monotonic() - start:.1f}s -> {out}")
        return 0 if found else 1
    finally:
        client.disconnect()
        api.shutdown()


if __name__ == "__main__":
    sys.exit(main())
