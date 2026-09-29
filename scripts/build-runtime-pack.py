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
ENCYCLOPEDIA_PREFIX = "encyclopedia/assets/"

# Source-proven STRATEGY.DLL BMPs used by the original encyclopedia shell and
# every recovered control state. The two inner overlays are mode-specific and
# shared by both factions. Text resources 0x1842/0x1843 and font 0x299d are not
# bitmap identities and are intentionally absent.
REQUIRED_ENCYCLOPEDIA_CHROME = (
    0x285F, 0x2860, 0x2861, 0x2862, 0x2959, 0x295D,
    0x2882, 0x2883, 0x2888, 0x2889,
    0x288E, 0x288F, 0x2890, 0x2891, 0x2892, 0x2893,
    0x2886, 0x2887, 0x288C, 0x288D,
    0x2884, 0x2885, 0x288A, 0x288B,
    0x2864, 0x2863, 0x286E, 0x286D,
    0x286C, 0x286B, 0x2878, 0x2877,
    0x2868, 0x2867, 0x2874, 0x2873,
    0x2D60, 0x2D5F, 0x2D62, 0x2D61,
    0x2870, 0x286F, 0x287A, 0x2879,
    0x286A, 0x2869, 0x2876, 0x2875,
)


@dataclass(frozen=True)
class Entry:
    kind: int
    key: str
    path: Path
    expected_sha256: str | None = None


def validate_options_resources(ui_dir: Path) -> None:
    """Refuse stale UI staging that would omit the original confirmation controls."""
    for resource, width, height in [(10623, 412, 176), (10624, 57, 28),
                                    (10625, 57, 28), (10626, 57, 28), (10627, 57, 28)]:
        path = ui_dir / "rebdlog-dll" / "BMP" / f"{resource}.bmp"
        try:
            data = path.read_bytes()
            if len(data) < 54 or data[:2] != b"BM":
                raise ValueError("invalid BMP header")
            offset = struct.unpack_from("<I", data, 10)[0]
            dib_size, actual_width, actual_height, planes, bits, compression = struct.unpack_from("<IiiHHI", data, 14)
            stride = ((width * bits + 31) // 32) * 4
            if (dib_size < 40 or actual_width != width or abs(actual_height) != height
                    or planes != 1 or bits != 8 or compression != 0
                    or offset < 54 or len(data) < offset + stride * height):
                raise ValueError("invalid dimensions or truncated bitmap")
        except (OSError, ValueError, struct.error) as error:
            raise ValueError(f"required options resource REBDLOG {resource}: {error}; restage UI assets") from error


def validate_encyclopedia_chrome_resources(ui_dir: Path) -> None:
    """Require each source-proven encyclopedia bitmap by exact runtime key."""
    for resource in REQUIRED_ENCYCLOPEDIA_CHROME:
        relative = Path("strategy-dll") / "BMP" / f"{resource}.bmp"
        path = ui_dir / relative
        try:
            data = path.read_bytes()
            if len(data) < 54 or data[:2] != b"BM":
                raise ValueError("invalid BMP header")
        except (OSError, ValueError) as error:
            raise ValueError(
                f"required encyclopedia chrome STRATEGY.DLL BMP resource {resource} "
                f"at {relative.as_posix()}: {error}; restage UI assets"
            ) from error


def collect_entries(
    base_dir: Path,
    ui_dir: Path,
    audio_dir: Path | None = None,
    tactical_runtime_dir: Path | None = None,
    edata_dir: Path | None = None,
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
        entries.extend(collect_tactical_runtime_entries(runtime_dir))

    if edata_dir is not None:
        entries.extend(collect_edata_entries(edata_dir))

    entries.sort(key=lambda entry: (entry.kind, entry.key))
    keys = [(entry.kind, entry.key) for entry in entries]
    if len(keys) != len(set(keys)):
        raise ValueError("runtime pack contains duplicate keys")
    return entries


def collect_edata_entries(edata_dir: Path) -> list[Entry]:
    """Collect validated original encyclopedia bitmaps under a namespaced key."""
    if not edata_dir.is_dir():
        raise ValueError(f"EData directory does not exist: {edata_dir}")

    numbered: list[tuple[int, Path]] = []
    for path in edata_dir.iterdir():
        if not path.is_file() or not path.name.startswith("EDATA."):
            continue
        suffix = path.name.removeprefix("EDATA.")
        if len(suffix) != 3 or not suffix.isascii() or not suffix.isdigit():
            raise ValueError(f"invalid EData filename: {path.name}")
        numbered.append((int(suffix), path))

    if not numbered:
        raise ValueError(f"EData directory contains no EDATA.NNN artwork: {edata_dir}")

    entries: list[Entry] = []
    seen: set[int] = set()
    for number, path in sorted(numbered):
        if number in seen:
            raise ValueError(f"duplicate EData identity: {number:03}")
        seen.add(number)
        validate_edata_bitmap(path)
        entries.append(Entry(KIND_GAME_DATA, f"{ENCYCLOPEDIA_PREFIX}{path.name}", path))
    return entries


def validate_edata_bitmap(path: Path) -> None:
    """Validate the fixed 400x200 indexed BMP contract before packaging."""
    try:
        data = path.read_bytes()
        if len(data) < 54 or data[:2] != b"BM":
            raise ValueError("invalid BMP header")
        offset = struct.unpack_from("<I", data, 10)[0]
        dib_size, width, height, planes, bits, compression = struct.unpack_from(
            "<IiiHHI", data, 14
        )
        stride = ((400 * bits + 31) // 32) * 4
        palette_end = 14 + dib_size + 256 * 4
        if (
            dib_size < 40
            or width != 400
            or abs(height) != 200
            or planes != 1
            or bits != 8
            or compression != 0
            or offset < palette_end
            or len(data) < offset + stride * 200
        ):
            raise ValueError("expected an uncompressed 400x200x8 bitmap")
    except (OSError, ValueError, struct.error) as error:
        raise ValueError(f"invalid encyclopedia artwork {path.name}: {error}") from error


def collect_tactical_runtime_entries(runtime_dir: Path) -> list[Entry]:
    manifest_path = runtime_dir / "manifest.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    if manifest.get("schema_version") != 1:
        raise ValueError("unsupported tactical runtime manifest version")

    meshes = manifest.get("meshes")
    textures = manifest.get("textures")
    if not isinstance(meshes, list) or not meshes:
        raise ValueError("tactical runtime manifest contains no meshes")
    if not isinstance(textures, list) or not textures:
        raise ValueError("tactical runtime manifest contains no textures")

    named_textures: dict[tuple[str, int], dict] = {}
    for texture in textures:
        if texture.get("identifier_kind") != "name":
            continue
        name = texture.get("name")
        language = texture.get("language")
        if not isinstance(name, str) or not name or not isinstance(language, int):
            raise ValueError("tactical runtime has an invalid named texture identity")
        key = (name.casefold(), language)
        if key in named_textures:
            raise ValueError(f"duplicate named tactical texture {name}/{language}")
        named_textures[key] = texture

    entries: list[Entry] = []
    for mesh in meshes:
        mesh_id = mesh.get("id")
        language = mesh.get("language")
        if not isinstance(mesh_id, int) or mesh_id <= 0 or not isinstance(language, int):
            raise ValueError("tactical runtime has an invalid mesh identity")
        for binding in mesh.get("texture_bindings") or []:
            name = binding.get("resource_name")
            binding_language = binding.get("resource_language")
            if (
                not isinstance(name, str)
                or (name.casefold(), binding_language) not in named_textures
            ):
                raise ValueError(
                    f"mesh {mesh_id}/{language} references a missing named tactical texture"
                )
        mesh_path, mesh_digest = checked_runtime_object(runtime_dir, mesh, ".mesh")
        entries.append(
            Entry(
                KIND_TACTICAL_MESH,
                f"{mesh_id}/{language}",
                mesh_path,
                mesh_digest,
            )
        )

    for texture in textures:
        identifier_kind = texture.get("identifier_kind")
        language = texture.get("language")
        if not isinstance(language, int):
            raise ValueError("tactical runtime has an invalid texture language")
        if identifier_kind == "name":
            identifier = texture.get("name")
            if not isinstance(identifier, str) or not identifier:
                raise ValueError("tactical runtime has an invalid named texture identity")
        elif identifier_kind == "id":
            identifier = texture.get("id")
            if not isinstance(identifier, int) or identifier <= 0:
                raise ValueError("tactical runtime has an invalid numeric texture identity")
        else:
            raise ValueError("tactical runtime has an unknown texture identity kind")
        texture_path, texture_digest = checked_runtime_object(
            runtime_dir, texture, ".texture"
        )
        entries.append(
            Entry(
                KIND_TACTICAL_TEXTURE,
                f"{identifier}/{language}",
                texture_path,
                texture_digest,
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
    parser.add_argument("--base", type=Path)
    parser.add_argument("--ui", type=Path, required=True)
    parser.add_argument("--audio", type=Path)
    parser.add_argument("--tactical-runtime", type=Path)
    parser.add_argument("--edata", type=Path)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--validate-ui-only", action="store_true")
    args = parser.parse_args()
    try:
        validate_options_resources(args.ui)
        validate_encyclopedia_chrome_resources(args.ui)
    except ValueError as error:
        parser.error(str(error))
    if args.validate_ui_only:
        return
    if args.base is None or args.output is None:
        parser.error("--base and --output are required when building a runtime pack")

    if not args.base.is_dir():
        parser.error(f"game-data directory does not exist: {args.base}")
    if not args.ui.is_dir():
        parser.error(f"UI directory does not exist: {args.ui}")

    entries = collect_entries(
        args.base, args.ui, args.audio, args.tactical_runtime, args.edata
    )
    if not entries:
        parser.error("refusing to create an empty runtime pack")
    written = write_pack(entries, args.output)
    verify_pack(args.output, entries)

    encyclopedia_assets = sum(
        entry.kind == KIND_GAME_DATA and entry.key.startswith(ENCYCLOPEDIA_PREFIX)
        for entry in entries
    )
    game_files = sum(entry.kind == KIND_GAME_DATA for entry in entries) - encyclopedia_assets
    bitmaps = sum(entry.kind == KIND_BITMAP for entry in entries)
    audio_files = sum(entry.kind == KIND_AUDIO for entry in entries)
    advisor_frames = sum(entry.kind == KIND_ADVISOR_FRAME for entry in entries)
    tactical_meshes = sum(entry.kind == KIND_TACTICAL_MESH for entry in entries)
    tactical_textures = sum(entry.kind == KIND_TACTICAL_TEXTURE for entry in entries)
    print(
        f"Runtime pack: {game_files} game files + "
        f"{encyclopedia_assets} encyclopedia assets + {bitmaps} bitmaps + "
        f"{advisor_frames} advisor frames + {audio_files} audio files + "
        f"{tactical_meshes} tactical meshes + {tactical_textures} tactical textures, "
        f"{written} bytes ({args.output})"
    )


if __name__ == "__main__":
    main()
