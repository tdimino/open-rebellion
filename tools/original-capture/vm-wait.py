"""Hold one VNC connection and wait for a region of the guest screen to change.

    vm-wait.py SERVER REGION COUNT TIMEOUT FILE

Run by `vm-game.sh wait`, which passes the password in VNC_PASSWORD so it
never appears in argv. REGION is a name below or `X,Y,W,H` in the 800x600
guest frame. The script takes a baseline crop, polls about twice a second,
and counts a change each time the crop differs from the previous one. After
COUNT changes it saves the full frame to FILE and the crop beside it
(`FILE` with `-region` before the extension), prints the elapsed time and
exits 0. On TIMEOUT seconds it saves the frame it has and exits 1.

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
# Mean absolute difference (0-255) above which a crop counts as changed. A
# digit flip in the day box scores well above it; galaxy twinkle stays below.
THRESHOLD = 2.0
POLL = 0.5


def grab(client, box):
    client.refreshScreen()
    frame = client.screen.copy()
    x, y, w, h = box
    return frame, frame.crop((x, y, x + w, y + h)).convert("L")


def main():
    if len(sys.argv) != 6:
        sys.exit(__doc__)
    server, region, count, timeout, out = sys.argv[1:]
    box = REGIONS.get(region) or tuple(int(v) for v in region.split(","))
    count, timeout, out = int(count), float(timeout), Path(out)
    client = api.connect(server, password=os.environ.get("VNC_PASSWORD") or None)
    try:
        start = time.monotonic()
        frame, last = grab(client, box)
        seen = 0
        while seen < count and time.monotonic() - start < timeout:
            time.sleep(POLL)
            frame, crop = grab(client, box)
            if ImageStat.Stat(ImageChops.difference(crop, last)).mean[0] > THRESHOLD:
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


if __name__ == "__main__":
    sys.exit(main())
