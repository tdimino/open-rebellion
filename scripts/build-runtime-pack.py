#!/usr/bin/env python3
"""Build the deterministic Open Rebellion browser runtime asset pack."""

from __future__ import annotations

import argparse
import hashlib
import json
import struct
from dataclasses import dataclass
from pathlib import Path


MAGIC = b"ORPK"
VERSION = 3
HEADER = struct.Struct("<4sHHI")
ENTRY_HEADER = struct.Struct("<BHI")
KIND_GAME_DATA = 0
KIND_BITMAP = 1
KIND_AUDIO = 2
KIND_ADVISOR_FRAME = 3
KIND_TACTICAL_MESH = 4
KIND_TACTICAL_TEXTURE = 5

TACTICAL_PROOF_LANGUAGE = 1033
TACTICAL_LOD_FAMILY = {
    2560: "SDESTI52.BMP",
    2561: "SDESTI_M.BMP",
    2562: None,
}
TACTICAL_PALETTE_IDS = range(5531, 5558)


@dataclass(frozen=True)
class Entry:
    kind: int
    key: str
    path: Path
    expected_sha256: str | None = None


def collect_entries(
    base_dir: Path,
    ui_dir: Path,
    audio_dir: Path | None = None,
    tactical_runtime_dir: Path | None = None,
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

    runtime_dir = tactical_runtime_dir or (
        ui_dir / "tactical-dll" / "TACTICAL3D" / "runtime"
    )
    if runtime_dir.is_dir():
        entries.extend(collect_tactical_proof_entries(runtime_dir))

    entries.sort(key=lambda entry: (entry.kind, entry.key))
    keys = [(entry.kind, entry.key) for entry in entries]
    if len(keys) != len(set(keys)):
        raise ValueError("runtime pack contains duplicate keys")
    return entries


def collect_tactical_proof_entries(runtime_dir: Path) -> list[Entry]:
    manifest_path = runtime_dir / "manifest.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    if manifest.get("schema_version") != 1:
        raise ValueError("unsupported tactical runtime manifest version")

    entries: list[Entry] = []
    selected_textures: dict[str, dict] = {}
    for mesh_id, expected_texture in TACTICAL_LOD_FAMILY.items():
        matches = [
            record
            for record in manifest.get("meshes", [])
            if record.get("id") == mesh_id
            and record.get("language") == TACTICAL_PROOF_LANGUAGE
        ]
        if len(matches) != 1:
            raise ValueError(f"tactical runtime lacks unique mesh {mesh_id}/1033")
        mesh = matches[0]
        bindings = mesh.get("texture_bindings") or []
        if expected_texture is None:
            if bindings:
                raise ValueError(f"tactical mesh {mesh_id} unexpectedly binds a texture")
        else:
            binding = bindings[0] if len(bindings) == 1 else {}
            if (
                str(binding.get("resource_name", "")).casefold()
                != expected_texture.casefold()
                or binding.get("resource_language") != TACTICAL_PROOF_LANGUAGE
            ):
                raise ValueError(
                    f"tactical mesh {mesh_id} does not bind {expected_texture}/1033"
                )
            textures = [
                record
                for record in manifest.get("textures", [])
                if record.get("identifier_kind") == "name"
                and record.get("name", "").casefold() == expected_texture.casefold()
                and record.get("language") == TACTICAL_PROOF_LANGUAGE
            ]
            if len(textures) != 1:
                raise ValueError(
                    f"tactical runtime lacks unique texture {expected_texture}/1033"
                )
            texture = textures[0]
            if (
                texture.get("kind") != "indexed_rle"
                or texture.get("palette_rule") != "battle_active"
            ):
                raise ValueError(
                    f"{expected_texture} does not retain the active battle-palette rule"
                )
            selected_textures[expected_texture] = texture

        mesh_path, mesh_digest = checked_runtime_object(runtime_dir, mesh, ".mesh")
        entries.append(
            Entry(
                KIND_TACTICAL_MESH,
                f"{mesh_id}/{TACTICAL_PROOF_LANGUAGE}",
                mesh_path,
                mesh_digest,
            )
        )

    for texture_name, texture in selected_textures.items():
        texture_path, texture_digest = checked_runtime_object(
            runtime_dir, texture, ".texture"
        )
        entries.append(
            Entry(
                KIND_TACTICAL_TEXTURE,
                f"{texture_name}/{TACTICAL_PROOF_LANGUAGE}",
                texture_path,
                texture_digest,
            )
        )
    for palette_id in TACTICAL_PALETTE_IDS:
        matches = [
            record
            for record in manifest.get("textures", [])
            if record.get("identifier_kind") == "id"
            and record.get("id") == palette_id
            and record.get("language") == TACTICAL_PROOF_LANGUAGE
            and record.get("kind") == "palette_rgb24"
        ]
        if len(matches) != 1:
            raise ValueError(
                f"tactical runtime lacks unique palette {palette_id}/1033"
            )
        palette = matches[0]
        palette_path, palette_digest = checked_runtime_object(
            runtime_dir, palette, ".texture"
        )
        entries.append(
            Entry(
                KIND_TACTICAL_TEXTURE,
                f"{palette_id}/{TACTICAL_PROOF_LANGUAGE}",
                palette_path,
                palette_digest,
            )
        )
    return entries


def checked_runtime_object(
    runtime_dir: Path, record: dict, suffix: str
) -> tuple[Path, str]:
    digest = record.get("object_sha256", "")
    relative = record.get("object", "")
    expected = f"objects/{digest}{suffix}"
    if len(digest) != 64 or any(char not in "0123456789abcdef" for char in digest):
        raise ValueError("tactical runtime object has an invalid SHA-256")
    if relative != expected:
        raise ValueError("tactical runtime object path is not content-addressed")
    path = runtime_dir / relative
    data = path.read_bytes()
    if hashlib.sha256(data).hexdigest() != digest:
        raise ValueError("tactical runtime object failed SHA-256 verification")
    return path, digest


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
    if (
        entry.expected_sha256 is not None
        and hashlib.sha256(data).hexdigest() != entry.expected_sha256
    ):
        raise ValueError(
            f"runtime pack source changed after validation: {entry.key}"
        )
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
    # Length equality is checked above. Avoid ``zip(strict=True)`` so the
    # verifier also runs under macOS's system Python 3.9.
    for actual, entry in zip(observed, expected):
        if actual != (entry.kind, entry.key, entry_bytes(entry)):
            raise ValueError(f"runtime pack verification failed for {entry.key}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base", type=Path, required=True)
    parser.add_argument("--ui", type=Path, required=True)
    parser.add_argument("--audio", type=Path)
    parser.add_argument("--tactical-runtime", type=Path)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    if not args.base.is_dir():
        parser.error(f"game-data directory does not exist: {args.base}")
    if not args.ui.is_dir():
        parser.error(f"UI directory does not exist: {args.ui}")

    entries = collect_entries(args.base, args.ui, args.audio, args.tactical_runtime)
    if not entries:
        parser.error("refusing to create an empty runtime pack")
    written = write_pack(entries, args.output)
    verify_pack(args.output, entries)

    game_files = sum(entry.kind == KIND_GAME_DATA for entry in entries)
    bitmaps = sum(entry.kind == KIND_BITMAP for entry in entries)
    audio_files = sum(entry.kind == KIND_AUDIO for entry in entries)
    advisor_frames = sum(entry.kind == KIND_ADVISOR_FRAME for entry in entries)
    tactical_meshes = sum(entry.kind == KIND_TACTICAL_MESH for entry in entries)
    tactical_textures = sum(entry.kind == KIND_TACTICAL_TEXTURE for entry in entries)
    print(
        f"Runtime pack: {game_files} game files + {bitmaps} bitmaps + "
        f"{advisor_frames} advisor frames + {audio_files} audio files + "
        f"{tactical_meshes} tactical meshes + {tactical_textures} tactical textures, "
        f"{written} bytes ({args.output})"
    )


if __name__ == "__main__":
    main()
