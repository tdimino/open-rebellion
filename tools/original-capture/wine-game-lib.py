#!/usr/bin/env python3
"""BMP-to-PNG converter and screenshot utilities for wine-game.sh.

Usage:
    python3 wine-game-lib.py bmp2png INPUT.bmp OUTPUT.png
    python3 wine-game-lib.py crop INPUT.png X Y W H OUTPUT.png

crop and diff take X Y W H in the VM's 800x600 reference frame and scale
them to each image, as drive.exe scales input.  diff prints the mean
absolute difference of the two boxes over all channels, 0-255.  changed
prints the fraction of pixels whose channels differ by more than 60 in
sum: two Battle Alert titles differ in 8% of their box but in only a few
units of mean, since the box is mostly black.
    python3 wine-game-lib.py diff A.png B.png X Y W H
    python3 wine-game-lib.py changed A.png B.png X Y W H
"""

import struct
import sys
import zlib
from pathlib import Path


def png_chunk(kind: bytes, data: bytes) -> bytes:
    return (struct.pack(">I", len(data)) + kind + data +
            struct.pack(">I", zlib.crc32(kind + data) & 0xFFFF_FFFF))


def write_png(path: str, width: int, height: int, rgb: bytes) -> None:
    stride = width * 3
    rows = bytearray()
    for y in range(height):
        rows.append(0)  # filter byte
        rows.extend(rgb[y * stride:(y + 1) * stride])
    payload = (b"\x89PNG\r\n\x1a\n" +
               png_chunk(b"IHDR", struct.pack(">2I5B", width, height,
                                              8, 2, 0, 0, 0)) +
               png_chunk(b"IDAT", zlib.compress(bytes(rows))) +
               png_chunk(b"IEND", b""))
    Path(path).write_bytes(payload)


def bmp_to_png(bmp_path: str, png_path: str) -> None:
    data = Path(bmp_path).read_bytes()
    if data[:2] != b"BM":
        raise ValueError("Not a BMP file")
    off = struct.unpack_from("<I", data, 10)[0]
    hdr_size = struct.unpack_from("<I", data, 14)[0]
    w = struct.unpack_from("<i", data, 18)[0]
    h = struct.unpack_from("<i", data, 22)[0]
    bpp = struct.unpack_from("<H", data, 28)[0]
    if bpp != 24:
        raise ValueError(f"Only 24-bit BMP supported, got {bpp}")
    top_down = h < 0
    h = abs(h)
    stride = ((w * 3 + 3) & ~3)
    rgb = bytearray(w * h * 3)
    for y in range(h):
        src_y = y if top_down else (h - 1 - y)
        src_off = off + src_y * stride
        for x in range(w):
            b, g, r = data[src_off + x * 3:src_off + x * 3 + 3]
            dst = (y * w + x) * 3
            rgb[dst] = r
            rgb[dst + 1] = g
            rgb[dst + 2] = b
    write_png(png_path, w, h, bytes(rgb))
    print(f"{w}x{h} -> {png_path}")


REF_W, REF_H = 800, 600


def read_png(png_path: str) -> tuple[int, int, bytes]:
    """Decode a PNG that write_png produced (8-bit RGB, filter 0)."""
    data = Path(png_path).read_bytes()
    if data[:8] != b"\x89PNG\r\n\x1a\n":
        raise ValueError("Not a PNG")
    pos = 8
    width = height = 0
    idat = b""
    while pos < len(data):
        length = struct.unpack(">I", data[pos:pos + 4])[0]
        chunk_type = data[pos + 4:pos + 8]
        chunk_data = data[pos + 8:pos + 8 + length]
        if chunk_type == b"IHDR":
            width, height = struct.unpack(">II", chunk_data[:8])
        elif chunk_type == b"IDAT":
            idat += chunk_data
        pos += 12 + length
    raw = zlib.decompress(idat)
    stride = width * 3
    pixels = bytearray(width * height * 3)
    for row in range(height):
        if raw[row * (stride + 1)] != 0:
            raise ValueError("Only filter-0 PNGs from write_png are supported")
        src = row * (stride + 1) + 1
        pixels[row * stride:(row + 1) * stride] = raw[src:src + stride]
    return width, height, bytes(pixels)


def ref_box(width: int, height: int, x: int, y: int, cw: int, ch: int):
    """Scale a reference-frame rectangle to the image, rounding outward."""
    x0, y0 = x * width // REF_W, y * height // REF_H
    x1 = min(-(-(x + cw) * width // REF_W), width)
    y1 = min(-(-(y + ch) * height // REF_H), height)
    return x0, y0, x1 - x0, y1 - y0


def box_pixels(png_path: str, x: int, y: int, cw: int, ch: int):
    width, height, pixels = read_png(png_path)
    x, y, cw, ch = ref_box(width, height, x, y, cw, ch)
    rows = [pixels[((y + r) * width + x) * 3:((y + r) * width + x + cw) * 3]
            for r in range(ch)]
    return cw, ch, b"".join(rows)


def crop_png(png_path: str, x: int, y: int, cw: int, ch: int, out: str) -> None:
    """Crop a reference-frame region from a PNG that write_png produced."""
    cw, ch, pixels = box_pixels(png_path, x, y, cw, ch)
    write_png(out, cw, ch, pixels)


def diff_png(a_path: str, b_path: str, x: int, y: int, cw: int, ch: int) -> float:
    a = box_pixels(a_path, x, y, cw, ch)
    b = box_pixels(b_path, x, y, cw, ch)
    if a[:2] != b[:2]:
        raise ValueError(f"Boxes differ in size: {a[:2]} and {b[:2]}")
    return sum(abs(p - q) for p, q in zip(a[2], b[2])) / len(a[2])


def changed_png(a_path: str, b_path: str, x: int, y: int, cw: int, ch: int) -> float:
    a = box_pixels(a_path, x, y, cw, ch)
    b = box_pixels(b_path, x, y, cw, ch)
    if a[:2] != b[:2]:
        raise ValueError(f"Boxes differ in size: {a[:2]} and {b[:2]}")
    pa, pb = a[2], b[2]
    count = sum(1 for i in range(0, len(pa), 3)
                if abs(pa[i] - pb[i]) + abs(pa[i + 1] - pb[i + 1]) + abs(pa[i + 2] - pb[i + 2]) > 60)
    return count / (len(pa) // 3)


def main():
    if len(sys.argv) < 2:
        print(__doc__, file=sys.stderr)
        return 2
    cmd = sys.argv[1]
    if cmd == "bmp2png" and len(sys.argv) == 4:
        bmp_to_png(sys.argv[2], sys.argv[3])
    elif cmd == "crop" and len(sys.argv) == 8:
        crop_png(sys.argv[2], int(sys.argv[3]), int(sys.argv[4]),
                 int(sys.argv[5]), int(sys.argv[6]), sys.argv[7])
    elif cmd == "diff" and len(sys.argv) == 8:
        print(f"{diff_png(sys.argv[2], sys.argv[3], *map(int, sys.argv[4:8])):.2f}")
    elif cmd == "changed" and len(sys.argv) == 8:
        print(f"{changed_png(sys.argv[2], sys.argv[3], *map(int, sys.argv[4:8])):.4f}")
    else:
        print(__doc__, file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())
