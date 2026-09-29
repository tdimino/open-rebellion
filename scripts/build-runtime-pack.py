#!/usr/bin/env python3
"""Build the deterministic Open Rebellion browser runtime asset pack."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import string
import struct
import subprocess
import tempfile
from dataclasses import dataclass, replace
from pathlib import Path, PurePosixPath


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
ENCYCLOPEDIA_VERIFIER_ENV = "REBELLION_STAGE_UI_ASSETS"
ENCYCLOPEDIA_VERIFIER_TIMEOUT_SECONDS = 120
REPO_ROOT = Path(__file__).resolve().parent.parent
DEFAULT_ENCYCLOPEDIA_DIR = REPO_ROOT / "data" / "base" / "encyclopedia"
MAX_CATALOG_BYTES = 64 * 1024 * 1024
MAX_MANIFEST_BYTES = 32 * 1024 * 1024
MAX_IMAGE_BYTES = 32 * 1024 * 1024
MAX_AGGREGATE_IMAGE_BYTES = 128 * 1024 * 1024
MAX_PACK_ENTRY_BYTES = 0xFFFFFFFF
SHA256_ALPHABET = frozenset(string.hexdigits.lower())
PATH_SEGMENT_ALPHABET = frozenset(string.ascii_letters + string.digits + "._-")

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
    confined_root: Path | None = None
    max_bytes: int | None = None
    # When present, these immutable bytes are authoritative; path is diagnostic only.
    retained_bytes: bytes | None = None


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
    encyclopedia_dir: Path | None = None,
    require_encyclopedia: bool = False,
) -> list[Entry]:
    del edata_dir  # Raw EData is local evidence; only a canonical manifest may ship it.
    entries = collect_package_dat_entries(base_dir)

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

    if encyclopedia_dir is not None:
        encyclopedia_entries = collect_encyclopedia_entries(
            encyclopedia_dir, base_dir, entries
        )
        if require_encyclopedia and not encyclopedia_entries:
            raise ValueError(
                f"required encyclopedia namespace is absent: {encyclopedia_dir}"
            )
        entries.extend(encyclopedia_entries)
    elif require_encyclopedia:
        raise ValueError("required encyclopedia namespace has no configured root")

    entries.sort(key=lambda entry: (entry.kind, entry.key))
    keys = [(entry.kind, entry.key) for entry in entries]
    if len(keys) != len(set(keys)):
        raise ValueError("runtime pack contains duplicate keys")
    return entries


def collect_package_dat_entries(base_dir: Path) -> list[Entry]:
    entries = []
    for path in sorted(
        (
            item
            for item in base_dir.iterdir()
            if item.name.casefold().endswith(".dat") and item.is_file()
        ),
        key=lambda item: item.name,
    ):
        checked = checked_confined_file(base_dir, path.name)
        entries.append(
            Entry(
                KIND_GAME_DATA,
                path.name,
                checked,
                sha256_file(checked),
                base_dir,
            )
        )
    return entries


def collect_encyclopedia_entries(
    root: Path,
    base_dir: Path,
    package_entries: list[Entry] | None = None,
) -> list[Entry]:
    """Collect the exact E42-verified runtime allowlist and selected DAT pairing."""
    if root.is_symlink():
        raise ValueError(f"unsafe encyclopedia root: {root}")
    if not root.exists():
        return []
    if not root.is_dir():
        raise ValueError(f"unsafe encyclopedia root: {root}")

    manifest_path = checked_confined_file(root, "manifest.json")
    manifest_bytes = read_bounded_file(
        manifest_path, MAX_MANIFEST_BYTES, "manifest.json"
    )
    try:
        manifest = json.loads(manifest_bytes)
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise ValueError(
            f"encyclopedia verification failed: invalid manifest: {error}"
        ) from error
    if not isinstance(manifest, dict):
        raise ValueError(
            "encyclopedia verification failed: manifest must be an object"
        )

    files = manifest.get("files")
    if not isinstance(files, dict) or "catalog.json" not in files:
        raise ValueError(
            "encyclopedia verification failed: manifest files must include catalog.json"
        )

    retained_files = {"manifest.json": manifest_bytes}
    entries = [
        Entry(
            KIND_GAME_DATA,
            f"{ENCYCLOPEDIA_NAMESPACE}manifest.json",
            manifest_path,
            hashlib.sha256(manifest_bytes).hexdigest(),
            root,
            MAX_MANIFEST_BYTES,
            manifest_bytes,
        )
    ]
    aggregate_image_bytes = 0
    for relative, expected_digest in files.items():
        relative_path = checked_runtime_asset_path(relative)
        if not valid_sha256(expected_digest):
            raise ValueError(
                f"encyclopedia verification failed: invalid digest for {relative!r}"
            )
        path = checked_confined_file(root, relative_path.as_posix())
        max_bytes = (
            MAX_CATALOG_BYTES
            if relative_path.as_posix() == "catalog.json"
            else MAX_IMAGE_BYTES
        )
        if relative_path.as_posix() != "catalog.json":
            remaining = MAX_AGGREGATE_IMAGE_BYTES - aggregate_image_bytes
            max_bytes = min(max_bytes, remaining)
        data = read_bounded_file(path, max_bytes, relative_path.as_posix())
        observed_digest = hashlib.sha256(data).hexdigest()
        if observed_digest != expected_digest:
            raise ValueError(
                f"encyclopedia verification failed: digest mismatch for {relative}"
            )
        entries.append(
            Entry(
                KIND_GAME_DATA,
                f"{ENCYCLOPEDIA_NAMESPACE}{relative_path.as_posix()}",
                path,
                expected_digest,
                root,
                max_bytes,
                data,
            )
        )
        retained_files[relative_path.as_posix()] = data
        if relative_path.as_posix() != "catalog.json":
            aggregate_image_bytes += len(data)

    selected_entries = package_entries
    if selected_entries is None:
        selected_entries = collect_package_dat_entries(base_dir)
    replacements = verify_binding_sources(manifest, base_dir, selected_entries)
    verify_encyclopedia_snapshot(retained_files)
    if package_entries is not None:
        for index, replacement in replacements.items():
            package_entries[index] = replacement
    entries.sort(key=lambda entry: (entry.kind, entry.key))
    return entries


def checked_runtime_asset_path(value: object) -> PurePosixPath:
    if not isinstance(value, str) or not 8 <= len(value) <= 256:
        raise ValueError(f"unsafe encyclopedia path: {value!r}")
    if "\\" in value or value.startswith("/") or "//" in value:
        raise ValueError(f"unsafe encyclopedia path: {value!r}")
    parts = value.split("/")
    if any(
        not part
        or part in {".", ".."}
        or len(part) > 128
        or part[0] not in string.ascii_letters + string.digits
        or any(char not in PATH_SEGMENT_ALPHABET for char in part)
        for part in parts
    ):
        raise ValueError(f"unsafe encyclopedia path: {value!r}")
    path = PurePosixPath(value)
    if value != "catalog.json" and (not parts or parts[0] != "assets"):
        raise ValueError(f"unsafe encyclopedia path: {value!r}")
    return path


def checked_confined_file(root: Path, relative: str) -> Path:
    if root.is_symlink() or not root.is_dir():
        raise ValueError(f"unsafe or missing runtime-pack root: {root}")
    candidate = root
    for part in PurePosixPath(relative).parts:
        if part in {"", ".", ".."}:
            raise ValueError(f"unsafe encyclopedia path: {relative!r}")
        candidate = candidate / part
        if candidate.is_symlink():
            raise ValueError(f"encyclopedia path uses a symlink: {relative}")
    try:
        root_resolved = root.resolve(strict=True)
        candidate_resolved = candidate.resolve(strict=True)
        candidate_resolved.relative_to(root_resolved)
    except (FileNotFoundError, RuntimeError, ValueError) as error:
        raise ValueError(
            f"encyclopedia verification failed: missing or escaping file {relative}"
        ) from error
    if not candidate_resolved.is_file():
        raise ValueError(
            f"encyclopedia verification failed: non-regular file {relative}"
        )
    return candidate


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def read_bounded_file(path: Path, max_bytes: int, label: str) -> bytes:
    ensure_file_size_limit(path, max_bytes, label)
    with path.open("rb") as handle:
        data = handle.read(max_bytes + 1)
    if len(data) > max_bytes:
        raise ValueError(
            f"resource limit for {label}: file grew beyond {max_bytes} bytes"
        )
    return data


def ensure_file_size_limit(path: Path, max_bytes: int, label: str) -> None:
    size = path.stat().st_size
    if size < 0 or size > max_bytes:
        raise ValueError(
            f"resource limit for {label}: {size} bytes exceeds {max_bytes}"
        )


def valid_sha256(value: object) -> bool:
    return (
        isinstance(value, str)
        and len(value) == 64
        and value == value.lower()
        and all(char in SHA256_ALPHABET for char in value)
    )


def verify_binding_sources(
    manifest: dict,
    base_dir: Path,
    package_entries: list[Entry] | None = None,
) -> dict[int, Entry]:
    sources = manifest.get("binding_sources")
    if not isinstance(sources, list) or not sources:
        raise ValueError(
            "encyclopedia verification failed: manifest binding_sources is empty"
        )
    if package_entries is None:
        package_entries = collect_package_dat_entries(base_dir)
    candidates: dict[str, list[tuple[int, Entry]]] = {}
    for index, entry in enumerate(package_entries):
        if entry.kind == KIND_GAME_DATA and entry.key.casefold().endswith(".dat"):
            candidates.setdefault(entry.key.casefold(), []).append((index, entry))
    replacements: dict[int, Entry] = {}
    for source in sources:
        if not isinstance(source, dict):
            raise ValueError("encyclopedia verification failed: invalid binding source")
        basename = source.get("basename")
        expected = source.get("sha256")
        if not isinstance(basename, str) or not valid_sha256(expected):
            raise ValueError("encyclopedia verification failed: invalid binding source")
        matches = candidates.get(basename.casefold(), [])
        if len(matches) != 1:
            raise ValueError(
                f"binding source {basename} must resolve to exactly one selected package DAT"
            )
        index, entry = matches[0]
        if entry.expected_sha256 != expected:
            raise ValueError(
                f"binding source mismatch for selected package DAT {basename}"
            )
        selected_size = entry.path.stat().st_size
        if selected_size > MAX_PACK_ENTRY_BYTES:
            raise ValueError(f"runtime-pack entry is too large: {entry.path}")
        data = entry_bytes(replace(entry, max_bytes=selected_size))
        if hashlib.sha256(data).hexdigest() != expected:
            raise ValueError(
                f"binding source mismatch for selected package DAT {basename}"
            )
        replacements[index] = replace(
            entry,
            max_bytes=len(data),
            retained_bytes=data,
        )
    return replacements


def verify_encyclopedia_snapshot(retained_files: dict[str, bytes]) -> None:
    """Run E42 against an owned snapshot of the exact bytes retained for writing."""
    with tempfile.TemporaryDirectory(
        prefix="open-rebellion-encyclopedia-pack-"
    ) as directory:
        snapshot_root = Path(directory)
        destinations = []
        for relative, data in retained_files.items():
            destination = snapshot_root / PurePosixPath(relative)
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(data)
            destinations.append(destination)
        for destination in destinations:
            destination.chmod(0o400)
        for child in sorted(
            (path for path in snapshot_root.rglob("*") if path.is_dir()),
            key=lambda path: len(path.parts),
            reverse=True,
        ):
            child.chmod(0o500)
        snapshot_root.chmod(0o500)
        try:
            verify_encyclopedia_stage(snapshot_root)
        except OSError as error:
            raise ValueError(
                "encyclopedia verification failed: sealed snapshot mutation "
                f"was refused: {error}"
            ) from error
        for relative, expected in retained_files.items():
            observed = read_bounded_file(
                snapshot_root / PurePosixPath(relative),
                len(expected),
                f"verified snapshot {relative}",
            )
            if observed != expected:
                raise ValueError(
                    f"encyclopedia verifier changed retained snapshot {relative}"
                )


def verify_encyclopedia_stage(root: Path) -> None:
    executable = os.environ.get(ENCYCLOPEDIA_VERIFIER_ENV)
    if executable:
        command = [executable]
    else:
        command = ["go", "run", "./tools/stage-ui-assets"]
    command.extend(
        [
            "--encyclopedia-only",
            "--verify",
            "--encyclopedia-output",
            str(root.resolve(strict=True)),
        ]
    )
    try:
        result = subprocess.run(
            command,
            cwd=REPO_ROOT,
            check=False,
            capture_output=True,
            text=True,
            timeout=ENCYCLOPEDIA_VERIFIER_TIMEOUT_SECONDS,
        )
    except subprocess.TimeoutExpired as error:
        raise ValueError(
            "encyclopedia verification timed out after "
            f"{ENCYCLOPEDIA_VERIFIER_TIMEOUT_SECONDS} seconds"
        ) from error
    except OSError as error:
        raise ValueError(
            f"encyclopedia verification failed to start: {error}"
        ) from error
    if result.returncode != 0:
        detail = (result.stderr or result.stdout).strip()
        raise ValueError(
            f"encyclopedia verification failed: {detail or f'exit {result.returncode}'}"
        )

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
    if len(entries) > 0xFFFFFFFF:
        raise ValueError("runtime pack contains too many entries")
    descriptor, candidate_name = tempfile.mkstemp(
        prefix=f".{output.name}.", suffix=".tmp", dir=output.parent
    )
    candidate = Path(candidate_name)
    try:
        written = HEADER.size
        with os.fdopen(descriptor, "wb") as handle:
            handle.write(HEADER.pack(MAGIC, VERSION, 0, len(entries)))
            for entry in entries:
                key = entry.key.encode("utf-8")
                data = entry_bytes(entry)
                if not key or len(key) > 0xFFFF:
                    raise ValueError(f"invalid runtime-pack key length: {entry.key!r}")
                if len(data) > MAX_PACK_ENTRY_BYTES:
                    raise ValueError(f"runtime-pack entry is too large: {entry.path}")
                handle.write(ENTRY_HEADER.pack(entry.kind, len(key), len(data)))
                handle.write(key)
                handle.write(data)
                written += ENTRY_HEADER.size + len(key) + len(data)
            handle.flush()
            os.fsync(handle.fileno())
        verify_pack(candidate, entries)
        candidate.chmod(0o644)
        os.replace(candidate, output)
        return written
    except BaseException:
        candidate.unlink(missing_ok=True)
        raise


def entry_bytes(entry: Entry) -> bytes:
    if entry.retained_bytes is not None:
        data = entry.retained_bytes
    else:
        path = entry.path
        if entry.confined_root is not None:
            try:
                relative = path.relative_to(entry.confined_root).as_posix()
            except ValueError as error:
                raise ValueError(
                    f"runtime pack source escapes its declared root: {entry.key}"
                ) from error
            path = checked_confined_file(entry.confined_root, relative)
        data = (
            read_bounded_file(path, entry.max_bytes, entry.key)
            if entry.max_bytes is not None
            else path.read_bytes()
        )
    if entry.max_bytes is not None and len(data) > entry.max_bytes:
        raise ValueError(
            f"resource limit for {entry.key}: {len(data)} bytes exceeds "
            f"{entry.max_bytes}"
        )
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
    parser.add_argument(
        "--edata",
        type=Path,
        help="deprecated raw artwork input; canonical manifest staging is required",
    )
    parser.add_argument(
        "--encyclopedia",
        type=Path,
        default=DEFAULT_ENCYCLOPEDIA_DIR,
        help="canonical E42 encyclopedia stage",
    )
    parser.add_argument("--require-encyclopedia", action="store_true")
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

    if args.edata is not None:
        print(
            "WARNING: --edata no longer packages raw artwork; use a canonical "
            "--encyclopedia stage instead"
        )
    try:
        entries = collect_entries(
            args.base,
            args.ui,
            args.audio,
            args.tactical_runtime,
            args.edata,
            args.encyclopedia,
            args.require_encyclopedia,
        )
    except ValueError as error:
        parser.error(str(error))
    if not entries:
        parser.error("refusing to create an empty runtime pack")
    written = write_pack(entries, args.output)

    encyclopedia_files = sum(
        entry.kind == KIND_GAME_DATA and entry.key.startswith(ENCYCLOPEDIA_NAMESPACE)
        for entry in entries
    )
    game_files = (
        sum(entry.kind == KIND_GAME_DATA for entry in entries) - encyclopedia_files
    )
    bitmaps = sum(entry.kind == KIND_BITMAP for entry in entries)
    audio_files = sum(entry.kind == KIND_AUDIO for entry in entries)
    advisor_frames = sum(entry.kind == KIND_ADVISOR_FRAME for entry in entries)
    tactical_meshes = sum(entry.kind == KIND_TACTICAL_MESH for entry in entries)
    tactical_textures = sum(entry.kind == KIND_TACTICAL_TEXTURE for entry in entries)
    print(
        f"Runtime pack: {game_files} game files + "
        f"{encyclopedia_files} encyclopedia files + {bitmaps} bitmaps + "
        f"{advisor_frames} advisor frames + {audio_files} audio files + "
        f"{tactical_meshes} tactical meshes + {tactical_textures} tactical textures, "
        f"{written} bytes ({args.output})"
    )


if __name__ == "__main__":
    main()
