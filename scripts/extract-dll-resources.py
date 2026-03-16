#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = ["pefile", "pillow"]
# ///
"""
Extract BMP resources from Win32 PE DLLs (Star Wars Rebellion asset extraction).

Extracts RT_BITMAP (type 2) and RT_RCDATA (type 10) resources from DLLs like
ALSPRITE.DLL, EMSPRITE.DLL, ALBRIEF.DLL, EMBRIEF.DLL, REBDLOG.DLL.

Usage:
    uv run scripts/extract-dll-resources.py ALSPRITE.DLL --output assets/references/ref-ui/07-droid-advisors/alliance/
    uv run scripts/extract-dll-resources.py EMBRIEF.DLL --output assets/references/ref-ui/12-empire-briefing/
    uv run scripts/extract-dll-resources.py --all          # Extract all unextracted DLLs
    uv run scripts/extract-dll-resources.py --list ALSPRITE.DLL  # List resources without extracting
"""

import argparse
import struct
import sys
from pathlib import Path

import pefile


GAME_DIR = Path.home() / "Desktop" / "Programming" / "star-wars-rebellion"
PROJECT_DIR = Path(__file__).parent.parent
REF_UI = PROJECT_DIR / "assets" / "references" / "ref-ui"

# RT_BITMAP = 2, RT_RCDATA = 10, RT_GROUP_ICON = 14
RT_BITMAP = 2
RT_RCDATA = 10

# BMP file header (14 bytes) — needed because PE RT_BITMAP resources
# store DIB data WITHOUT the BMP file header
BMP_HEADER_SIZE = 14


def make_bmp_header(dib_size: int) -> bytes:
    """Create a BMP file header for a DIB (device-independent bitmap)."""
    file_size = BMP_HEADER_SIZE + dib_size
    # Read DIB header to get offset to pixel data
    # BITMAPINFOHEADER is 40 bytes, then palette follows
    return struct.pack('<2sIHHI', b'BM', file_size, 0, 0, 0)


def extract_bitmaps(dll_path: Path, output_dir: Path, list_only: bool = False) -> int:
    """Extract all bitmap resources from a PE DLL. Returns count."""
    pe = pefile.PE(str(dll_path))
    output_dir.mkdir(parents=True, exist_ok=True)

    count = 0
    if not hasattr(pe, 'DIRECTORY_ENTRY_RESOURCE'):
        print(f"  No resources found in {dll_path.name}")
        return 0

    for entry in pe.DIRECTORY_ENTRY_RESOURCE.entries:
        resource_type = entry.struct.Id

        if resource_type not in (RT_BITMAP, RT_RCDATA):
            continue

        type_name = "bitmap" if resource_type == RT_BITMAP else "rcdata"

        if not hasattr(entry, 'directory'):
            continue

        for res_entry in entry.directory.entries:
            res_id = res_entry.struct.Id
            if res_id is None:
                continue

            if not hasattr(res_entry, 'directory'):
                continue

            for lang_entry in res_entry.directory.entries:
                data_rva = lang_entry.data.struct.OffsetToData
                size = lang_entry.data.struct.Size
                data = pe.get_memory_mapped_image()[data_rva:data_rva + size]

                if list_only:
                    print(f"  {res_id:6d}  {type_name:8s}  {size:>8d} bytes")
                    count += 1
                    continue

                # For RT_BITMAP, data is a DIB — prepend BMP file header
                if resource_type == RT_BITMAP:
                    # Read BITMAPINFOHEADER to calculate proper offset
                    if len(data) >= 4:
                        header_size = struct.unpack_from('<I', data, 0)[0]
                        # Calculate palette size
                        if header_size >= 40 and len(data) >= 32:
                            bit_count = struct.unpack_from('<H', data, 14)[0]
                            colors_used = struct.unpack_from('<I', data, 32)[0]
                            if colors_used == 0 and bit_count <= 8:
                                colors_used = 1 << bit_count
                            palette_size = colors_used * 4
                            pixel_offset = BMP_HEADER_SIZE + header_size + palette_size
                        else:
                            pixel_offset = BMP_HEADER_SIZE + header_size

                        file_size = BMP_HEADER_SIZE + len(data)
                        bmp_header = struct.pack('<2sIHHI',
                            b'BM', file_size, 0, 0, pixel_offset)
                        file_data = bmp_header + data
                        ext = ".bmp"
                    else:
                        file_data = data
                        ext = ".bin"
                else:
                    # RT_RCDATA — check magic bytes
                    if data[:2] == b'BM':
                        ext = ".bmp"
                    elif data[:4] == b'\x89PNG':
                        ext = ".png"
                    elif data[:3] == b'GIF':
                        ext = ".gif"
                    else:
                        ext = ".bin"
                    file_data = data

                fname = f"{res_id:05d}-{dll_path.stem.lower()}{ext}"
                fpath = output_dir / fname
                fpath.write_bytes(file_data)
                count += 1

    if not list_only:
        print(f"  Extracted {count} resources from {dll_path.name} → {output_dir}")
    else:
        print(f"  Listed {count} resources in {dll_path.name}")

    return count


# Default extraction targets
TARGETS = {
    'ALBRIEF.DLL': '11-alliance-briefing',
    'EMBRIEF.DLL': '12-empire-briefing',
    'ALSPRITE.DLL': '07-droid-advisors/alliance',
    'EMSPRITE.DLL': '07-droid-advisors/empire',
    'REBDLOG.DLL': '13-dialogs',
    'CDDERR.DLL': '14-error-dialogs',
}


def main():
    parser = argparse.ArgumentParser(description="Extract BMP resources from Win32 PE DLLs")
    parser.add_argument("dll", nargs="?", help="DLL filename (e.g., ALSPRITE.DLL)")
    parser.add_argument("--output", "-o", type=Path, help="Output directory")
    parser.add_argument("--game-dir", type=Path, default=GAME_DIR, help="Game installation directory")
    parser.add_argument("--all", action="store_true", help="Extract all unextracted DLLs")
    parser.add_argument("--list", action="store_true", help="List resources without extracting")
    args = parser.parse_args()

    if args.all:
        total = 0
        for dll_name, subdir in TARGETS.items():
            dll_path = args.game_dir / dll_name
            if not dll_path.exists():
                print(f"  SKIP: {dll_name} not found in {args.game_dir}")
                continue
            out = REF_UI / subdir
            total += extract_bitmaps(dll_path, out, args.list)
        print(f"\nTotal: {total} resources")
        return

    if not args.dll:
        parser.print_help()
        sys.exit(1)

    dll_path = args.game_dir / args.dll
    if not dll_path.exists():
        # Try as absolute path
        dll_path = Path(args.dll)
    if not dll_path.exists():
        print(f"ERROR: {args.dll} not found", file=sys.stderr)
        sys.exit(1)

    if args.output:
        out = args.output
    elif dll_path.stem.upper() in TARGETS:
        out = REF_UI / TARGETS[dll_path.stem.upper() + '.DLL']
    else:
        out = Path(f"extracted-{dll_path.stem.lower()}")

    extract_bitmaps(dll_path, out, args.list)


if __name__ == "__main__":
    main()
