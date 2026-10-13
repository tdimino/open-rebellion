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

    vm-wait.py SERVER alert-title TITLE.png TIMEOUT FILE

waits until a Battle Alert is open, recognised by its button bar matching
`chrome.png` beside TITLE.png (ALERT_CHROME overrides), then compares the
alert's title band with TITLE.png: exit 0 when it is that alert, 3 when it
is another one (FILE holds the frame, so the caller can resolve it and wait
again), 1 on TIMEOUT.

Polling one open connection replaces a fresh `vncdo capture` per frame, and
returning on a change replaces sleep-and-capture loops.
"""

import os
import sys
import time
from pathlib import Path

from PIL import Image, ImageChops, ImageStat
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
# The Alliance's alert draws the same button red and further left: 167
# bright red pixels in this box, no other screen more than 65 (2026-10-12).
ALLIANCE_ALERT_BOX = (320, 436, 380, 462)

# vncdotool waits up to an hour for each reply. A refresh once stalled for
# 43 minutes past a 45 s timeout (2026-10-12), so bound every call.
CALL_TIMEOUT = 20
# A bounded call that times out drops the connection; reconnect and carry on
# rather than lose a long wait to one stalled refresh.
RECONNECTS = 5

# The Battle Alert's three-button bar is the same on every alert (Carida and
# Coruscant differ by 1.3 mean) and far from the results and galaxy screens
# (66-86). Its title band matches the same battle within 0.5 and differs
# from other battles by 14.9-19.1.
CHROME_BOX = (172, 432, 672, 468)
TITLE_BOX = (180, 86, 660, 116)
CHROME_MAX = 8.0
TITLE_MAX = 5.0


def connect(server):
    return api.connect(server, password=os.environ.get("VNC_PASSWORD") or None,
                       timeout=CALL_TIMEOUT)


class Screen:
    """One VNC connection that reconnects when a bounded call times out."""

    def __init__(self, server):
        self.server = server
        self.client = connect(server)

    def frame(self):
        for attempt in range(RECONNECTS + 1):
            try:
                self.client.refreshScreen()
                return self.client.screen.copy()
            except TimeoutError:
                if attempt == RECONNECTS:
                    raise
                print("vnc: refresh timed out, reconnecting", file=sys.stderr)
                try:
                    self.client.disconnect()
                except Exception:
                    pass
                time.sleep(2)
                self.client = connect(self.server)

    def close(self):
        try:
            self.client.disconnect()
        finally:
            api.shutdown()


def mean_diff(a, b, box):
    return ImageStat.Stat(ImageChops.difference(
        a.convert("RGB").crop(box), b.convert("RGB").crop(box)).convert("L")).mean[0]


def alert_open(frame):
    rgb = frame.convert("RGB")

    def count(box, hit):
        data = rgb.crop(box).tobytes()
        return sum(1 for p in zip(data[0::3], data[1::3], data[2::3]) if hit(*p))

    empire = count(ALERT_BOX, lambda r, g, b: g > 180 and r < 120 and b < 120)
    alliance = count(ALLIANCE_ALERT_BOX, lambda r, g, b: r > 180 and g < 120 and b < 120)
    return max(empire, alliance) >= ALERT_MIN


def grab(screen, box):
    frame = screen.frame()
    x, y, w, h = box
    return frame, frame.crop((x, y, x + w, y + h)).convert("L")


def main():
    if len(sys.argv) != 6:
        sys.exit(__doc__)
    server, region, count, timeout, out = sys.argv[1:]
    if region == "alert":
        return wait_alert(server, float(timeout), Path(out))
    if region == "alert-title":
        return wait_alert_title(server, Path(count), float(timeout), Path(out))
    box = REGIONS.get(region) or tuple(int(v) for v in region.split(","))
    threshold = THRESHOLDS.get(region, THRESHOLD)
    count, timeout, out = int(count), float(timeout), Path(out)
    client = Screen(server)
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
        client.close()


def wait_alert(server, timeout, out):
    client = Screen(server)
    try:
        start = time.monotonic()
        while True:
            frame = client.frame()
            found = alert_open(frame)
            if found or time.monotonic() - start >= timeout:
                break
            time.sleep(POLL)
        frame.save(out)
        state = "open" if found else "not open"
        print(f"alert: {state} after {time.monotonic() - start:.1f}s -> {out}")
        return 0 if found else 1
    finally:
        client.close()


def wait_alert_title(server, title_png, timeout, out):
    title = Image.open(title_png)
    chrome = Image.open(os.environ.get("ALERT_CHROME") or title_png.with_name("chrome.png"))
    client = Screen(server)
    try:
        start = time.monotonic()
        while True:
            frame = client.frame()
            if mean_diff(frame, chrome, CHROME_BOX) <= CHROME_MAX:
                # Let the alert finish drawing its title before reading it.
                time.sleep(1)
                frame = client.frame()
                score = mean_diff(frame, title, TITLE_BOX)
                frame.save(out)
                found = score <= TITLE_MAX
                which = "target" if found else "other"
                print(f"alert: {which} (title {score:.1f}) after {time.monotonic() - start:.1f}s -> {out}")
                return 0 if found else 3
            if time.monotonic() - start >= timeout:
                frame.save(out)
                print(f"alert: none after {time.monotonic() - start:.1f}s -> {out}")
                return 1
            time.sleep(POLL)
    finally:
        client.close()


if __name__ == "__main__":
    sys.exit(main())
