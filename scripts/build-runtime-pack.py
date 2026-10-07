#!/usr/bin/env python3
"""Build the deterministic Open Rebellion browser runtime asset pack."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import struct
import tempfile
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
ENCYCLOPEDIA_NAMESPACE = "encyclopedia/"
ENCYCLOPEDIA_PREFIX = f"{ENCYCLOPEDIA_NAMESPACE}assets/"
ENCYCLOPEDIA_CATALOG_KEY = f"{ENCYCLOPEDIA_NAMESPACE}catalog.json"
ENCYCLOPEDIA_MANIFEST_KEY = f"{ENCYCLOPEDIA_NAMESPACE}manifest.json"
ENCYCLOPEDIA_SOURCE_BYTES_LIMIT = 64 * 1024 * 1024
ENCYCLOPEDIA_MANIFEST_BYTES_LIMIT = 32 * 1024 * 1024
ENCYCLOPEDIA_BODY_BYTES_LIMIT = 1024 * 1024
ENCYCLOPEDIA_RESOURCE_LIMIT = 10_000
ENCYCLOPEDIA_EXPECTED_TEXTS = 348
ENCYCLOPEDIA_EXPECTED_ARTWORK_MAPPINGS = 191
ENCYCLOPEDIA_EXPECTED_ENCYTEXT_SHA256 = (
    "49aea545a5e09e5fe9115a22bc785690f103d2f931e08bd4a53a617a42636d8c"
)
ENCYCLOPEDIA_EXPECTED_ENCYBMAP_SHA256 = (
    "fb545d19ae24b0277753494dbfaabf2dbdde660beab821287a32016c290e4560"
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


def collect_entries(
    base_dir: Path,
    ui_dir: Path,
    audio_dir: Path | None = None,
    tactical_runtime_dir: Path | None = None,
    edata_dir: Path | None = None,
    encyclopedia_source: Path | None = None,
    require_encyclopedia: bool = False,
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

    if (edata_dir is None) != (encyclopedia_source is None):
        raise ValueError(
            "Encyclopedia publication requires both source catalog and EData directory"
        )
    if require_encyclopedia and encyclopedia_source is None:
        raise ValueError(
            "required Encyclopedia publication is absent; provide source catalog and EData directory"
        )
    if encyclopedia_source is not None and edata_dir is not None:
        entries.extend(
            collect_encyclopedia_entries(
                encyclopedia_source,
                edata_dir,
                require_owned_profile=require_encyclopedia,
            )
        )

    entries.sort(key=lambda entry: (entry.kind, entry.key))
    keys = [(entry.kind, entry.key) for entry in entries]
    if len(keys) != len(set(keys)):
        raise ValueError("runtime pack contains duplicate keys")
    return entries


def _read_regular_bounded(path: Path, limit: int, description: str) -> bytes:
    if path.is_symlink() or not path.is_file():
        raise ValueError(f"{description} is missing or unsafe: {path}")
    size = path.stat().st_size
    if size > limit:
        raise ValueError(f"{description} exceeds the byte limit: {path}")
    return path.read_bytes()


def _strict_json(data: bytes, description: str) -> dict:
    def unique_object(pairs: list[tuple[str, object]]) -> dict:
        result = {}
        for key, value in pairs:
            if key in result:
                raise ValueError(f"{description} contains duplicate key {key!r}")
            result[key] = value
        return result

    try:
        value = json.loads(data, object_pairs_hook=unique_object)
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise ValueError(f"invalid {description}: {error}") from error
    if not isinstance(value, dict):
        raise ValueError(f"invalid {description}: expected an object")
    return value


def _valid_sha256(value: object) -> bool:
    return (
        isinstance(value, str)
        and len(value) == 64
        and all(character in "0123456789abcdef" for character in value)
    )


def _is_exact_int(value: object, expected: int) -> bool:
    return type(value) is int and value == expected


def _canonical_resource_map(value: object, description: str) -> dict[str, object]:
    if not isinstance(value, dict) or not value or len(value) > ENCYCLOPEDIA_RESOURCE_LIMIT:
        raise ValueError(f"invalid Encyclopedia {description} resource map")
    identities: set[int] = set()
    for key in value:
        if (
            not isinstance(key, str)
            or not key.isascii()
            or not key.isdigit()
            or int(key) == 0
            or int(key) > 0xFFFF
            or str(int(key)) != key
            or int(key) in identities
        ):
            raise ValueError(f"invalid Encyclopedia {description} resource identity {key!r}")
        identities.add(int(key))
    return value


def _valid_edata_filename(value: object) -> bool:
    return (
        isinstance(value, str)
        and len(value) == len("EDATA.000")
        and value.startswith("EDATA.")
        and value[6:].isascii()
        and value[6:].isdigit()
    )


def collect_encyclopedia_entries(
    source: Path,
    edata_dir: Path,
    *,
    require_owned_profile: bool = False,
) -> list[Entry]:
    """Collect one complete P66A catalog, sidecar, and referenced artwork set."""
    catalog_bytes = _read_regular_bounded(
        source, ENCYCLOPEDIA_SOURCE_BYTES_LIMIT, "Encyclopedia source catalog"
    )
    manifest_path = Path(f"{source}.manifest.json")
    manifest_bytes = _read_regular_bounded(
        manifest_path,
        ENCYCLOPEDIA_MANIFEST_BYTES_LIMIT,
        "Encyclopedia source manifest",
    )
    catalog = _strict_json(catalog_bytes, "Encyclopedia source catalog")
    manifest = _strict_json(manifest_bytes, "Encyclopedia source manifest")
    if set(catalog) != {
        "schema_version",
        "language_id",
        "encoding",
        "source_code_page",
        "texts",
        "artwork",
    }:
        raise ValueError("Encyclopedia source catalog has unknown or missing fields")
    if (
        not _is_exact_int(catalog["schema_version"], 1)
        or not _is_exact_int(catalog["language_id"], 1033)
        or catalog["encoding"] != "windows-1252"
        or not _is_exact_int(catalog["source_code_page"], 0)
    ):
        raise ValueError("unsupported Encyclopedia source profile")
    texts = _canonical_resource_map(catalog["texts"], "text")
    artwork = _canonical_resource_map(catalog["artwork"], "artwork")
    for resource, record in texts.items():
        if not isinstance(record, dict) or set(record) != {"body", "body_sha256"}:
            raise ValueError(f"invalid Encyclopedia text resource {resource}")
        body = record["body"]
        if (
            not isinstance(body, str)
            or not body
            or len(body.encode("utf-8")) > ENCYCLOPEDIA_BODY_BYTES_LIMIT
            or not _valid_sha256(record["body_sha256"])
            or hashlib.sha256(body.encode("utf-8")).hexdigest()
            != record["body_sha256"]
        ):
            raise ValueError(f"invalid Encyclopedia text resource {resource}")
    for resource, filename in artwork.items():
        if not _valid_edata_filename(filename):
            raise ValueError(
                f"invalid Encyclopedia artwork resource {resource}: {filename!r}"
            )

    expected_catalog_digest = hashlib.sha256(catalog_bytes).hexdigest()
    if (
        set(manifest) != {
            "schema_version",
            "catalog_sha256",
            "source_files",
            "counts",
        }
        or not _is_exact_int(manifest["schema_version"], 1)
        or manifest["catalog_sha256"] != expected_catalog_digest
        or not isinstance(manifest["source_files"], dict)
        or set(manifest["source_files"]) != {
            "encytext_sha256",
            "encybmap_sha256",
        }
        or not all(_valid_sha256(value) for value in manifest["source_files"].values())
        or not isinstance(manifest["counts"], dict)
        or set(manifest["counts"]) != {"texts", "artwork_mappings"}
        or not _is_exact_int(manifest["counts"]["texts"], len(texts))
        or not _is_exact_int(
            manifest["counts"]["artwork_mappings"], len(artwork)
        )
    ):
        raise ValueError("Encyclopedia source manifest count or integrity mismatch")
    if require_owned_profile and (
        len(texts) != ENCYCLOPEDIA_EXPECTED_TEXTS
        or len(artwork) != ENCYCLOPEDIA_EXPECTED_ARTWORK_MAPPINGS
        or manifest["source_files"]["encytext_sha256"]
        != ENCYCLOPEDIA_EXPECTED_ENCYTEXT_SHA256
        or manifest["source_files"]["encybmap_sha256"]
        != ENCYCLOPEDIA_EXPECTED_ENCYBMAP_SHA256
    ):
        raise ValueError(
            "required Encyclopedia publication must contain the verified owned "
            "English profile: 348 texts and 191 artwork mappings"
        )
    if edata_dir.is_symlink() or not edata_dir.is_dir():
        raise ValueError(f"EData directory does not exist or is unsafe: {edata_dir}")

    entries = [
        Entry(
            KIND_GAME_DATA,
            ENCYCLOPEDIA_CATALOG_KEY,
            source,
            expected_catalog_digest,
        ),
        Entry(
            KIND_GAME_DATA,
            ENCYCLOPEDIA_MANIFEST_KEY,
            manifest_path,
            hashlib.sha256(manifest_bytes).hexdigest(),
        ),
    ]
    for filename in sorted(set(artwork.values())):
        path = edata_dir / filename
        validate_edata_bitmap(path)
        entries.append(
            Entry(
                KIND_GAME_DATA,
                f"{ENCYCLOPEDIA_PREFIX}{filename}",
                path,
                hashlib.sha256(path.read_bytes()).hexdigest(),
            )
        )
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
        if path.is_symlink() or not path.is_file():
            raise ValueError("missing or unsafe file")
        data = path.read_bytes()
        if len(data) < 54 or data[:2] != b"BM":
            raise ValueError("invalid BMP header")
        declared_len = struct.unpack_from("<I", data, 2)[0]
        offset = struct.unpack_from("<I", data, 10)[0]
        dib_size, width, height, planes, bits, compression = struct.unpack_from(
            "<IiiHHI", data, 14
        )
        colors_used = struct.unpack_from("<I", data, 46)[0]
        stride = ((400 * bits + 31) // 32) * 4
        palette_end = 14 + dib_size + 256 * 4
        if (
            dib_size < 40
            or declared_len != len(data)
            or width != 400
            or abs(height) != 200
            or planes != 1
            or bits != 8
            or compression != 0
            or colors_used not in {0, 256}
            or offset < palette_end
            or len(data) < offset + stride * 200
        ):
            raise ValueError(
                "expected an uncompressed 400x200x8 bitmap with exact declared byte length"
            )
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
        handle.flush()
        os.fsync(handle.fileno())
    return written


def _fsync_directory(path: Path) -> None:
    descriptor = os.open(path, os.O_RDONLY)
    try:
        os.fsync(descriptor)
    finally:
        os.close(descriptor)


def publish_pack(entries: list[Entry], output: Path) -> int:
    """Serialize, verify, and atomically replace one ORPK artifact."""
    output.parent.mkdir(parents=True, exist_ok=True)
    descriptor, candidate_name = tempfile.mkstemp(
        prefix=f".{output.name}.", suffix=".tmp", dir=output.parent
    )
    os.close(descriptor)
    candidate = Path(candidate_name)
    try:
        written = write_pack(entries, candidate)
        verify_pack(candidate, entries)
        os.replace(candidate, output)
        _fsync_directory(output.parent)
        return written
    except BaseException:
        candidate.unlink(missing_ok=True)
        raise


def _encyclopedia_files(entries: list[Entry]) -> dict[str, bytes]:
    files = {}
    for entry in entries:
        if entry.kind != KIND_GAME_DATA or not entry.key.startswith(
            ENCYCLOPEDIA_NAMESPACE
        ):
            continue
        relative = entry.key.removeprefix(ENCYCLOPEDIA_NAMESPACE)
        parts = Path(relative).parts
        if (
            not relative
            or relative.startswith("/")
            or "\\" in relative
            or any(part in {"", ".", ".."} for part in parts)
            or relative in files
        ):
            raise ValueError(f"invalid Encyclopedia loose path: {relative!r}")
        files[relative] = entry_bytes(entry)
    if files and not {"catalog.json", "manifest.json"}.issubset(files):
        raise ValueError("partial Encyclopedia namespace")
    return files


def _encyclopedia_generation(files: dict[str, bytes]) -> str:
    digest = hashlib.sha256(b"open-rebellion:encyclopedia-loose:v1\0")
    for relative, data in sorted(files.items()):
        encoded = relative.encode("utf-8")
        digest.update(len(encoded).to_bytes(8, "little"))
        digest.update(encoded)
        digest.update(len(data).to_bytes(8, "little"))
        digest.update(data)
    return digest.hexdigest()


def _atomic_write(path: Path, data: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    descriptor, candidate_name = tempfile.mkstemp(
        prefix=f".{path.name}.", suffix=".tmp", dir=path.parent
    )
    candidate = Path(candidate_name)
    try:
        with os.fdopen(descriptor, "wb") as handle:
            handle.write(data)
            handle.flush()
            os.fsync(handle.fileno())
        os.replace(candidate, path)
        _fsync_directory(path.parent)
    except BaseException:
        candidate.unlink(missing_ok=True)
        raise


def publish_loose_encyclopedia(entries: list[Entry], mirror: Path) -> str | None:
    """Publish immutable loose bytes, then atomically activate their generation."""
    if mirror.is_symlink():
        raise ValueError(f"unsafe Encyclopedia loose mirror: {mirror}")
    mirror.mkdir(parents=True, exist_ok=True)
    generations = mirror / "generations"
    if generations.is_symlink():
        raise ValueError(f"unsafe Encyclopedia generations directory: {generations}")
    generations.mkdir(exist_ok=True)
    if not generations.is_dir():
        raise ValueError(f"invalid Encyclopedia generations directory: {generations}")
    files = _encyclopedia_files(entries)
    if not files:
        pointer = {"schema_version": 1, "available": False, "generation": None}
        _atomic_write(
            mirror / "current.json",
            (json.dumps(pointer, sort_keys=True, separators=(",", ":")) + "\n").encode(),
        )
        return None

    generation = _encyclopedia_generation(files)
    destination = generations / generation
    if destination.exists():
        _verify_loose_generation(destination, files)
    else:
        candidate = Path(tempfile.mkdtemp(prefix=".candidate-", dir=generations))
        try:
            for relative, data in sorted(files.items()):
                target = candidate / relative
                target.parent.mkdir(parents=True, exist_ok=True)
                with target.open("xb") as handle:
                    handle.write(data)
                    handle.flush()
                    os.fsync(handle.fileno())
            for directory in sorted(
                [candidate, *(path for path in candidate.rglob("*") if path.is_dir())],
                key=lambda value: len(value.parts),
                reverse=True,
            ):
                _fsync_directory(directory)
            _verify_loose_generation(candidate, files)
            os.replace(candidate, destination)
            _fsync_directory(generations)
        except BaseException:
            if candidate.exists():
                shutil.rmtree(candidate)
            raise
    pointer = {"schema_version": 1, "available": True, "generation": generation}
    _atomic_write(
        mirror / "current.json",
        (json.dumps(pointer, sort_keys=True, separators=(",", ":")) + "\n").encode(),
    )
    return generation


def _verify_loose_generation(root: Path, expected: dict[str, bytes]) -> None:
    if root.is_symlink() or not root.is_dir():
        raise ValueError(f"unsafe Encyclopedia loose generation: {root}")
    observed = {
        path.relative_to(root).as_posix(): path.read_bytes()
        for path in root.rglob("*")
        if path.is_file() and not path.is_symlink()
    }
    if observed != expected:
        raise ValueError("Encyclopedia loose generation verification failed")
    if any(path.is_symlink() for path in root.rglob("*")):
        raise ValueError("Encyclopedia loose generation contains a symlink")


def verify_loose_encyclopedia(entries: list[Entry], mirror: Path) -> None:
    files = _encyclopedia_files(entries)
    pointer = _strict_json(
        _read_regular_bounded(mirror / "current.json", 4096, "Encyclopedia loose pointer"),
        "Encyclopedia loose pointer",
    )
    expected_generation = _encyclopedia_generation(files) if files else None
    if pointer != {
        "schema_version": 1,
        "available": bool(files),
        "generation": expected_generation,
    }:
        raise ValueError("Encyclopedia loose pointer verification failed")
    if files:
        _verify_loose_generation(mirror / "generations" / expected_generation, files)


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
    parser.add_argument("--encyclopedia-source", type=Path)
    parser.add_argument("--encyclopedia-mirror", type=Path)
    parser.add_argument("--require-encyclopedia", action="store_true")
    parser.add_argument("--output", type=Path)
    parser.add_argument("--validate-ui-only", action="store_true")
    parser.add_argument("--verify-only", action="store_true")
    args = parser.parse_args()
    try:
        validate_options_resources(args.ui)
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
        args.base,
        args.ui,
        args.audio,
        args.tactical_runtime,
        args.edata,
        args.encyclopedia_source,
        args.require_encyclopedia,
    )
    if not entries:
        parser.error("refusing to create an empty runtime pack")
    if args.verify_only:
        if not args.output.is_file():
            parser.error(f"runtime pack does not exist: {args.output}")
        verify_pack(args.output, entries)
        if args.encyclopedia_mirror is not None:
            verify_loose_encyclopedia(entries, args.encyclopedia_mirror)
        written = args.output.stat().st_size
    else:
        written = publish_pack(entries, args.output)
        if args.encyclopedia_mirror is not None:
            publish_loose_encyclopedia(entries, args.encyclopedia_mirror)
            verify_loose_encyclopedia(entries, args.encyclopedia_mirror)

    encyclopedia_assets = sum(
        entry.kind == KIND_GAME_DATA and entry.key.startswith(ENCYCLOPEDIA_PREFIX)
        for entry in entries
    )
    encyclopedia_metadata = sum(
        entry.kind == KIND_GAME_DATA
        and entry.key in {ENCYCLOPEDIA_CATALOG_KEY, ENCYCLOPEDIA_MANIFEST_KEY}
        for entry in entries
    )
    game_files = (
        sum(entry.kind == KIND_GAME_DATA for entry in entries)
        - encyclopedia_assets
        - encyclopedia_metadata
    )
    bitmaps = sum(entry.kind == KIND_BITMAP for entry in entries)
    audio_files = sum(entry.kind == KIND_AUDIO for entry in entries)
    advisor_frames = sum(entry.kind == KIND_ADVISOR_FRAME for entry in entries)
    tactical_meshes = sum(entry.kind == KIND_TACTICAL_MESH for entry in entries)
    tactical_textures = sum(entry.kind == KIND_TACTICAL_TEXTURE for entry in entries)
    print(
        f"Runtime pack: {game_files} game files + "
        f"{encyclopedia_metadata} encyclopedia metadata + "
        f"{encyclopedia_assets} encyclopedia assets + {bitmaps} bitmaps + "
        f"{advisor_frames} advisor frames + {audio_files} audio files + "
        f"{tactical_meshes} tactical meshes + {tactical_textures} tactical textures, "
        f"{written} bytes ({args.output})"
    )


if __name__ == "__main__":
    main()
