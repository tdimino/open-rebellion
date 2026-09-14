#!/usr/bin/env python3
"""Python port of the advisor droid type-302 sparse-frame decoder.

Mirrors ``crates/rebellion-render/src/advisor.rs``:

* ``decode_anchor_bitmap``  — 8-bit uncompressed BMP anchor -> indices (top-down) + RGB palette
* ``decode_type302_frame``  — 17-byte header (u16 w, u16 h, u32 payload_size, 5 unused),
  ``height`` little-endian u32 row offsets, then per row alternating skip / literal runs whose
  literal bytes are ``wrapping_add`` deltas onto the base indices. Index 0 renders alpha 0.
* ``decode_family``         — the runtime chains frames: after each decode the base indices
  become that frame's indices (``load_authored_faction_frames``), so frame N is a delta on
  frame N-1, not on the anchor. Only the authored ranges below are chained by the runtime.

Authored families (``AuthoredFrameSpec::for_faction``):
  alsprite-dll  2001 -> 2002..2024   3331 -> 3332..3346
  emsprite-dll  2001 -> 2002..2016   3001 -> 3002..3016

The staged corpora hold 1,640 / 2,348 frames; frames outside the authored ranges have a
well-defined *pure* decode (delta on the dimension-matched anchor) but no runtime chain order.

CLI:
  python3 scripts/sprites/type302.py digests --out digests.json      # pure + chain FNV-1a64 digests
  python3 scripts/sprites/type302.py export --dll alsprite-dll --family 2001 --out out/
"""

from __future__ import annotations

import argparse
import json
import struct
import sys
from dataclasses import dataclass
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parents[2]
UI_ROOT = PROJECT_ROOT / "data" / "base" / "ui"
HEADER_LEN = 17
MAX_WIDTH, MAX_HEIGHT = 640, 480
FRAME_INTERVAL_SECONDS = 0.15  # DEFAULT_FRAME_INTERVAL in advisor.rs

AUTHORED_FAMILIES: dict[str, list[tuple[int, int, int]]] = {
    # dll dir -> [(anchor, first, last), ...]
    "alsprite-dll": [(2001, 2002, 2024), (3331, 3332, 3346)],
    "emsprite-dll": [(2001, 2002, 2016), (3001, 3002, 3016)],
}


class Type302Error(ValueError):
    pass


@dataclass
class FrameBase:
    width: int
    height: int
    indices: bytearray          # row-major, top-down
    palette: list[tuple[int, int, int]]


@dataclass
class DecodedFrame:
    width: int
    height: int
    indices: bytes
    rgba: bytes


# --------------------------------------------------------------- anchor BMP

def decode_anchor_bitmap(data: bytes) -> FrameBase:
    if len(data) < 54 or data[:2] != b"BM":
        raise Type302Error("not a BMP")
    pixel_offset = struct.unpack_from("<I", data, 10)[0]
    dib = 14
    header_size = struct.unpack_from("<I", data, dib)[0]
    if header_size < 40 or dib + header_size > len(data):
        raise Type302Error("invalid DIB header")
    width_s, height_s = struct.unpack_from("<ii", data, 18)
    planes = struct.unpack_from("<H", data, 26)[0]
    bit_count = struct.unpack_from("<H", data, dib + 14)[0]
    compression = struct.unpack_from("<I", data, 30)[0]
    if width_s <= 0 or height_s == 0 or planes != 1 or bit_count != 8 or compression != 0:
        raise Type302Error("anchor must be an uncompressed 8-bit BMP")
    width, height = width_s, abs(height_s)
    if width > MAX_WIDTH or height > MAX_HEIGHT:
        raise Type302Error("anchor too large")
    colors_used = struct.unpack_from("<I", data, dib + 32)[0]
    color_count = 256 if colors_used == 0 else colors_used
    if not 1 <= color_count <= 256:
        raise Type302Error("invalid colour count")
    palette_start = dib + header_size
    palette_end = palette_start + color_count * 4
    if pixel_offset < palette_end or palette_end > len(data):
        raise Type302Error("palette overlaps pixels")
    entries = data[palette_start:palette_end]
    palette = [(entries[i + 2], entries[i + 1], entries[i]) for i in range(0, len(entries), 4)]
    row_stride = (width + 3) & ~3
    source_end = pixel_offset + row_stride * height
    if source_end > len(data):
        raise Type302Error("truncated pixel data")
    indices = bytearray(width * height)
    for out_row in range(height):
        src_row = (height - 1 - out_row) if height_s > 0 else out_row
        start = pixel_offset + src_row * row_stride
        indices[out_row * width:(out_row + 1) * width] = data[start:start + width]
    if max(indices) >= len(palette):
        raise Type302Error("anchor index exceeds palette")
    return FrameBase(width, height, indices, palette)


# ------------------------------------------------------------ type-302 frame

def frame_header(data: bytes) -> tuple[int, int, int]:
    if len(data) < HEADER_LEN:
        raise Type302Error("truncated header")
    width, height, declared = struct.unpack_from("<HHI", data, 0)
    return width, height, declared


def indexed_rgba(indices: bytes, palette: list[tuple[int, int, int]]) -> bytes:
    out = bytearray(len(indices) * 4)
    lut = [bytes((r, g, b, 0 if i == 0 else 255)) for i, (r, g, b) in enumerate(palette)]
    if len(lut) < 256:
        lut += [None] * (256 - len(lut))
    for k, idx in enumerate(indices):
        px = lut[idx]
        if px is None:
            raise Type302Error(f"palette too small for index {idx}")
        out[k * 4:k * 4 + 4] = px
    return bytes(out)


def decode_type302_frame(data: bytes, base: FrameBase) -> DecodedFrame:
    """Pure decode of one frame as a delta on ``base`` (does not mutate ``base``)."""
    width, height, declared = frame_header(data)
    if width == 0 or height == 0 or width > MAX_WIDTH or height > MAX_HEIGHT:
        raise Type302Error(f"invalid dimensions {width}x{height}")
    if (width, height) != (base.width, base.height):
        raise Type302Error(f"frame {width}x{height} does not match base {base.width}x{base.height}")
    payload_start = HEADER_LEN + height * 4
    payload = data[payload_start:]
    if len(payload) != declared:
        raise Type302Error(f"payload size {len(payload)} != declared {declared}")
    if len(base.indices) != width * height:
        raise Type302Error("invalid anchor bitmap")
    indices = bytearray(base.indices)
    for row in range(height):
        offset = struct.unpack_from("<I", data, HEADER_LEN + row * 4)[0]
        if offset >= len(payload):
            raise Type302Error(f"row {row} offset {offset} out of bounds")
        cursor, x, literal = offset, 0, False
        while x < width:
            if cursor >= len(payload):
                raise Type302Error(f"truncated run row {row} x {x}")
            count = payload[cursor]
            cursor += 1
            if x + count > width:
                raise Type302Error(f"row {row} overflow at x {x} count {count}")
            if literal:
                deltas = payload[cursor:cursor + count]
                if len(deltas) != count:
                    raise Type302Error(f"truncated literal row {row} x {x}")
                dest = row * width + x
                for i, delta in enumerate(deltas):
                    indices[dest + i] = (indices[dest + i] + delta) & 0xFF
                cursor += count
            x += count
            literal = not literal
    return DecodedFrame(width, height, bytes(indices), indexed_rgba(bytes(indices), base.palette))


def decode_family(anchor: FrameBase, frames: list[tuple[int, bytes]]) -> list[tuple[int, DecodedFrame]]:
    """Chain decode in the given order, exactly as the runtime does."""
    base = FrameBase(anchor.width, anchor.height, bytearray(anchor.indices), anchor.palette)
    out = []
    for resource_id, data in frames:
        decoded = decode_type302_frame(data, base)
        base.indices = bytearray(decoded.indices)
        out.append((resource_id, decoded))
    return out


# ------------------------------------------------------------------ helpers

def fnv1a64(data: bytes) -> str:
    h = 0xCBF29CE484222325
    for b in data:
        h ^= b
        h = (h * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
    return f"{h:016x}"


def load_anchor(dll: str, resource_id: int) -> FrameBase:
    return decode_anchor_bitmap((UI_ROOT / dll / "BMP" / f"{resource_id}.bmp").read_bytes())


def frame_path(dll: str, resource_id: int) -> Path:
    return UI_ROOT / dll / "TYPE302" / f"{resource_id}.bin"


def staged_frames(dll: str) -> list[int]:
    return sorted(int(p.stem) for p in (UI_ROOT / dll / "TYPE302").glob("*.bin") if p.stem.isdigit())


def to_image(frame: DecodedFrame, palette: list[tuple[int, int, int]]):
    """Mode-P Pillow image with transparency index 0 and the anchor palette block."""
    from PIL import Image
    img = Image.frombytes("P", (frame.width, frame.height), frame.indices)
    flat = [c for rgb in palette for c in rgb] + [0] * (768 - 3 * len(palette))
    img.putpalette(flat)
    img.info["transparency"] = 0
    return img


# ---------------------------------------------------------------------- CLI

def cmd_digests(a):
    """Pure digest for every staged frame + chain digests for the authored families."""
    result: dict[str, dict] = {}
    for dll in AUTHORED_FAMILIES:
        anchors = {aid: load_anchor(dll, aid) for aid, _, _ in AUTHORED_FAMILIES[dll]}
        by_size = {(b.width, b.height): aid for aid, b in anchors.items()}
        for rid in staged_frames(dll):
            data = frame_path(dll, rid).read_bytes()
            w, h, _ = frame_header(data)
            aid = by_size.get((w, h))
            rec: dict = {"anchor": aid}
            if aid is None:
                rec["error"] = f"no anchor for {w}x{h}"
            else:
                try:
                    rec["pure"] = fnv1a64(decode_type302_frame(data, anchors[aid]).rgba)
                except Type302Error as exc:
                    rec["error"] = str(exc)
            result[f"{dll}/{rid}"] = rec
        for aid, first, last in AUTHORED_FAMILIES[dll]:
            frames = authored_chain(dll, first, last)
            for rid, decoded in decode_family(anchors[aid], frames):
                result[f"{dll}/{rid}"]["chain"] = fnv1a64(decoded.rgba)
    Path(a.out).write_text(json.dumps(result, indent=1, sort_keys=True))
    errors = sum(1 for r in result.values() if "error" in r)
    chained = sum(1 for r in result.values() if "chain" in r)
    print(f"frames={len(result)} pure_ok={len(result) - errors} errors={errors} chained={chained} -> {a.out}")
    return 1 if errors else 0


def authored_chain(dll: str, first: int, last: int) -> list[tuple[int, bytes]]:
    """Frames of an authored family in id order, stopping at the first missing file.

    Mirrors load_authored_faction_frames (advisor.rs): the runtime breaks the chain at the first
    gap, so nothing past a gap is ever rendered and must not be digested or exported either.
    """
    frames = []
    for rid in range(first, last + 1):
        path = frame_path(dll, rid)
        if not path.exists():
            break
        frames.append((rid, path.read_bytes()))
    return frames


def cmd_export(a):
    from PIL import Image  # noqa: F401
    fam = next((f for f in AUTHORED_FAMILIES[a.dll] if f[0] == a.family), None)
    if fam is None:
        sys.exit(f"unknown family {a.dll}/{a.family}; known: {AUTHORED_FAMILIES[a.dll]}")
    aid, first, last = fam
    anchor = load_anchor(a.dll, aid)
    frames = authored_chain(a.dll, first, last)
    if len(frames) != last - first + 1:
        print(f"WARNING: chain {a.dll}/{aid} stops at {first + len(frames) - 1}; runtime would stop there too", file=sys.stderr)
    out = Path(a.out)
    out.mkdir(parents=True, exist_ok=True)
    to_image(DecodedFrame(anchor.width, anchor.height, bytes(anchor.indices),
                          indexed_rgba(bytes(anchor.indices), anchor.palette)), anchor.palette
             ).save(out / f"{aid}.png", transparency=0)
    for rid, decoded in decode_family(anchor, frames):
        to_image(decoded, anchor.palette).save(out / f"{rid}.png", transparency=0)
    print(f"exported anchor {aid} + {len(frames)} frames ({anchor.width}x{anchor.height}) -> {out}")
    return 0


def main() -> int:
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = p.add_subparsers(dest="cmd", required=True)
    d = sub.add_parser("digests")
    d.add_argument("--out", required=True)
    d.set_defaults(fn=cmd_digests)
    e = sub.add_parser("export")
    e.add_argument("--dll", choices=sorted(AUTHORED_FAMILIES), required=True)
    e.add_argument("--family", type=int, required=True, help="anchor resource id")
    e.add_argument("--out", required=True)
    e.set_defaults(fn=cmd_export)
    a = p.parse_args()
    return a.fn(a)


if __name__ == "__main__":
    raise SystemExit(main())
