#!/usr/bin/env python3
"""Build the deterministic Open Rebellion browser runtime asset pack."""

from __future__ import annotations

import argparse
import json
import struct
from dataclasses import dataclass
from pathlib import Path


MAGIC = b"ORPK"
VERSION = 2
HEADER = struct.Struct("<4sHHI")
ENTRY_HEADER = struct.Struct("<BHI")
KIND_GAME_DATA = 0
KIND_BITMAP = 1
KIND_AUDIO = 2
KIND_ADVISOR_FRAME = 3


@dataclass(frozen=True)
class Entry:
    kind: int
    key: str
    path: Path


def collect_entries(
    base_dir: Path, ui_dir: Path, audio_dir: Path | None = None
) -> list[Entry]:
    entries = [
        Entry(KIND_GAME_DATA, path.name, path)
        for path in sorted(base_dir.glob("*.DAT"), key=lambda item: item.name)
    ]

    textstra = base_dir / "textstra.json"
    if textstra.is_file():
        entries.append(Entry(KIND_GAME_DATA, textstra.name, textstra))

    for dll_dir in sorted(ui_dir.iterdir(), key=lambda item: item.name):
        bmp_dir = dll_dir / "BMP"
        if bmp_dir.is_dir():
            for path in sorted(bmp_dir.glob("*.bmp"), key=lambda item: int(item.stem)):
                int(path.stem)  # Reject non-numeric resource names before writing.
                entries.append(
                    Entry(KIND_BITMAP, f"{dll_dir.name}/{path.stem}", path)
                )

        frame_dir = dll_dir / "TYPE302"
        if frame_dir.is_dir():
            for path in sorted(frame_dir.glob("*.bin"), key=lambda item: int(item.stem)):
                int(path.stem)
                entries.append(
                    Entry(KIND_ADVISOR_FRAME, f"{dll_dir.name}/{path.stem}", path)
                )

    if audio_dir is not None and audio_dir.is_dir():
        for path in sorted(audio_dir.rglob("*.wav")):
            entries.append(Entry(KIND_AUDIO, path.relative_to(audio_dir).as_posix(), path))

    entries.sort(key=lambda entry: (entry.kind, entry.key))
    keys = [(entry.kind, entry.key) for entry in entries]
    if len(keys) != len(set(keys)):
        raise ValueError("runtime pack contains duplicate keys")
    return entries


def write_pack(entries: list[Entry], output: Path) -> int:
    output.parent.mkdir(parents=True, exist_ok=True)
    written = HEADER.size
    with output.open("wb") as handle:
        handle.write(HEADER.pack(MAGIC, VERSION, 0, len(entries)))
        for entry in entries:
            key = entry.key.encode("utf-8")
            data = entry_bytes(entry)
            if not key or len(key) > 0xFFFF:
                raise ValueError(f"invalid runtime-pack key length: {entry.key!r}")
            if len(data) > 0xFFFFFFFF:
                raise ValueError(f"runtime-pack entry is too large: {entry.path}")
            handle.write(ENTRY_HEADER.pack(entry.kind, len(key), len(data)))
            handle.write(key)
            handle.write(data)
            written += ENTRY_HEADER.size + len(key) + len(data)
    return written


def entry_bytes(entry: Entry) -> bytes:
    data = entry.path.read_bytes()
    if entry.kind == KIND_GAME_DATA and entry.key == "textstra.json":
        parsed = json.loads(data)
        return json.dumps(
            parsed, ensure_ascii=False, sort_keys=True, separators=(",", ":")
        ).encode("utf-8")
    return data


def verify_pack(path: Path, expected: list[Entry]) -> None:
    contents = path.read_bytes()
    if len(contents) < HEADER.size:
        raise ValueError("runtime pack is shorter than its header")
    magic, version, flags, count = HEADER.unpack_from(contents)
    if (magic, version, flags, count) != (MAGIC, VERSION, 0, len(expected)):
        raise ValueError("runtime pack header verification failed")

    cursor = HEADER.size
    observed: list[tuple[int, str, bytes]] = []
    for _ in range(count):
        if cursor + ENTRY_HEADER.size > len(contents):
            raise ValueError("runtime pack entry header is truncated")
        kind, key_len, data_len = ENTRY_HEADER.unpack_from(contents, cursor)
        cursor += ENTRY_HEADER.size
        end = cursor + key_len + data_len
        if end > len(contents):
            raise ValueError("runtime pack entry payload is truncated")
        key = contents[cursor : cursor + key_len].decode("utf-8")
        cursor += key_len
        data = contents[cursor : cursor + data_len]
        cursor += data_len
        observed.append((kind, key, data))

    if cursor != len(contents):
        raise ValueError("runtime pack has trailing bytes")
    if len(observed) != len(expected):
        raise ValueError("runtime pack entry count changed during verification")
    for actual, entry in zip(observed, expected, strict=True):
        if actual != (entry.kind, entry.key, entry_bytes(entry)):
            raise ValueError(f"runtime pack verification failed for {entry.key}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base", type=Path, required=True)
    parser.add_argument("--ui", type=Path, required=True)
    parser.add_argument("--audio", type=Path)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    if not args.base.is_dir():
        parser.error(f"game-data directory does not exist: {args.base}")
    if not args.ui.is_dir():
        parser.error(f"UI directory does not exist: {args.ui}")

    entries = collect_entries(args.base, args.ui, args.audio)
    if not entries:
        parser.error("refusing to create an empty runtime pack")
    written = write_pack(entries, args.output)
    verify_pack(args.output, entries)

    game_files = sum(entry.kind == KIND_GAME_DATA for entry in entries)
    bitmaps = sum(entry.kind == KIND_BITMAP for entry in entries)
    audio_files = sum(entry.kind == KIND_AUDIO for entry in entries)
    advisor_frames = sum(entry.kind == KIND_ADVISOR_FRAME for entry in entries)
    print(
        f"Runtime pack: {game_files} game files + {bitmaps} bitmaps + "
        f"{advisor_frames} advisor frames + {audio_files} audio files, "
        f"{written} bytes ({args.output})"
    )


if __name__ == "__main__":
    main()
