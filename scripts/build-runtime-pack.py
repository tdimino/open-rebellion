#!/usr/bin/env python3
"""Build the deterministic Open Rebellion browser runtime asset pack."""

from __future__ import annotations

import argparse
import contextlib
import errno
import fcntl
import hashlib
import json
import os
import stat
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
PUBLICATION_SCHEMA_VERSION = 1
PUBLICATION_PHASES = {
    "prepared",
    "old_pack_backed_up",
    "old_mirror_backed_up",
    "mirror_published",
    "pack_published",
    "committed",
    "complete",
}
# Pending previous/desired inventories and compact exact artifact identities fit
# at the worst-case 20,001-file bound while remaining bounded before parsing.
PUBLICATION_RECORD_MAX_BYTES = 16 * 1024 * 1024
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


def _write_pack_candidate(entries: list[Entry], candidate: Path) -> int:
    """Serialize and verify one ORPK candidate without publishing it."""
    if len(entries) > 0xFFFFFFFF:
        raise ValueError("runtime pack contains too many entries")
    written = HEADER.size
    with candidate.open("wb") as handle:
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
    return written


def write_pack(entries: list[Entry], output: Path) -> int:
    """Publish one pack atomically; paired web publication uses the same serializer."""
    output.parent.mkdir(parents=True, exist_ok=True)
    descriptor, candidate_name = tempfile.mkstemp(
        prefix=f".{output.name}.", suffix=".tmp", dir=output.parent
    )
    os.close(descriptor)
    candidate = Path(candidate_name)
    try:
        written = _write_pack_candidate(entries, candidate)
        os.replace(candidate, output)
        return written
    except BaseException:
        candidate.unlink(missing_ok=True)
        raise


def _publication_paths(output: Path, mirror: Path) -> dict[str, Path]:
    try:
        output_parent = output.parent.resolve(strict=True)
        mirror_parent = mirror.parent.resolve(strict=True)
    except OSError as error:
        raise ValueError(
            "runtime publication parent must already exist and be accessible"
        ) from error
    if output_parent != mirror_parent:
        raise ValueError(
            "runtime pack and encyclopedia mirror must be sibling build artifacts"
        )
    if output.is_symlink() or mirror.is_symlink():
        raise ValueError("runtime publication targets must not be symlinks")
    canonical_output = output_parent / output.name
    canonical_mirror = mirror_parent / mirror.name
    if (
        output.resolve(strict=False) != canonical_output
        or mirror.resolve(strict=False) != canonical_mirror
    ):
        raise ValueError("runtime publication targets must use canonical sibling paths")
    paths = {
        "lock": output_parent / f".{output.name}.encyclopedia.lock",
        "record": output_parent / f".{output.name}.encyclopedia-publication.json",
        "pack_backup": output_parent / f".{output.name}.encyclopedia.backup",
        "mirror_backup": output_parent / f".{mirror.name}.encyclopedia.backup",
    }
    controlled = [canonical_output, canonical_mirror, *paths.values()]
    if len(set(controlled)) != len(controlled):
        raise ValueError(
            "runtime publication output, mirror, and control paths must be distinct; "
            "a path collision was detected"
        )
    observed_identities = {}
    for controlled_path in controlled:
        observed = _lstat(controlled_path)
        if observed is None:
            continue
        identity = (observed.st_dev, observed.st_ino)
        if identity in observed_identities:
            raise ValueError(
                "runtime publication output, mirror, and control paths alias the "
                f"same object: {observed_identities[identity]} and {controlled_path}"
            )
        observed_identities[identity] = controlled_path
    if output.exists() and (not output.is_file() or output.is_symlink()):
        raise ValueError("runtime pack target exists with an unsafe type")
    if mirror.exists() and (not mirror.is_dir() or mirror.is_symlink()):
        raise ValueError("encyclopedia mirror target exists with an unsafe type")
    return paths


def _validate_publication_input_paths(
    entries: list[Entry], output: Path, mirror: Path, paths: dict[str, Path]
) -> None:
    controlled = {
        output.resolve(strict=False),
        mirror.resolve(strict=False),
        *(path.resolve(strict=False) for path in paths.values()),
    }
    controlled_identities = set()
    for controlled_path in controlled:
        observed = _lstat(controlled_path)
        if observed is not None:
            controlled_identities.add((observed.st_dev, observed.st_ino))
    mirror_root = mirror.resolve(strict=False)
    for entry in entries:
        if entry.retained_bytes is not None:
            continue
        try:
            source = entry.path.resolve(strict=True)
        except OSError as error:
            raise ValueError(f"runtime pack input is unavailable: {entry.path}") from error
        observed = source.stat()
        if (
            source in controlled
            or mirror_root in source.parents
            or (observed.st_dev, observed.st_ino) in controlled_identities
        ):
            raise ValueError(
                f"runtime pack input collides with a publication path: {entry.path}"
            )


@contextlib.contextmanager
def _publication_lock(path: Path):
    flags = os.O_RDWR | os.O_CREAT
    flags |= getattr(os, "O_CLOEXEC", 0) | getattr(os, "O_NOFOLLOW", 0)
    try:
        descriptor = os.open(path, flags, 0o600)
    except OSError as error:
        if error.errno in {
            errno.ELOOP,
            errno.EISDIR,
            errno.ENOTDIR,
            errno.EPERM,
        }:
            raise ValueError(f"unsafe runtime publication lock object: {path}") from error
        raise
    try:
        opened = os.fstat(descriptor)
        try:
            named = os.lstat(path)
        except OSError as error:
            raise ValueError(f"unsafe runtime publication lock identity: {path}") from error
        if (
            not stat.S_ISREG(opened.st_mode)
            or opened.st_nlink != 1
            or (opened.st_dev, opened.st_ino) != (named.st_dev, named.st_ino)
        ):
            raise ValueError(f"unsafe runtime publication lock object: {path}")
        try:
            fcntl.flock(descriptor, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError as error:
            raise ValueError(
                f"runtime publication is busy; another writer holds {path}"
            ) from error
        named_after_lock = os.lstat(path)
        if (opened.st_dev, opened.st_ino) != (
            named_after_lock.st_dev,
            named_after_lock.st_ino,
        ):
            raise ValueError(f"unsafe runtime publication lock identity: {path}")
        try:
            yield
        finally:
            fcntl.flock(descriptor, fcntl.LOCK_UN)
    finally:
        os.close(descriptor)


def _read_publication_record(path: Path) -> dict | None:
    observed = _lstat(path)
    if observed is None:
        return None
    if not stat.S_ISREG(observed.st_mode):
        raise ValueError(f"unsafe runtime publication record: {path}")
    data = read_bounded_file(path, PUBLICATION_RECORD_MAX_BYTES, path.name)
    try:
        record = json.loads(data)
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise ValueError(f"invalid runtime publication record: {error}") from error
    if (
        not isinstance(record, dict)
        or record.get("schema_version") != PUBLICATION_SCHEMA_VERSION
        or record.get("phase") not in PUBLICATION_PHASES
    ):
        raise ValueError("invalid runtime publication record schema or phase")
    return record


def _fsync_directory(path: Path) -> None:
    directory_descriptor = os.open(path, os.O_RDONLY)
    try:
        os.fsync(directory_descriptor)
    finally:
        os.close(directory_descriptor)


def _write_publication_record(path: Path, record: dict) -> None:
    data = json.dumps(record, sort_keys=True, separators=(",", ":")).encode("utf-8")
    if len(data) > PUBLICATION_RECORD_MAX_BYTES:
        raise ValueError(
            "runtime publication record exceeds its bounded resource budget"
        )
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


def _encyclopedia_mirror_files(entries: list[Entry]) -> dict[str, bytes]:
    files: dict[str, bytes] = {}
    for entry in entries:
        if entry.kind != KIND_GAME_DATA or not entry.key.startswith(
            ENCYCLOPEDIA_NAMESPACE
        ):
            continue
        relative = entry.key[len(ENCYCLOPEDIA_NAMESPACE) :]
        if relative == "manifest.json":
            checked = PurePosixPath(relative)
        else:
            checked = checked_runtime_asset_path(relative)
        if checked.as_posix() != relative or relative in files:
            raise ValueError(f"invalid duplicate encyclopedia mirror path: {relative}")
        if entry.retained_bytes is None:
            raise ValueError(
                f"encyclopedia mirror requires retained verified bytes: {relative}"
            )
        files[relative] = entry_bytes(entry)
    required = {"catalog.json", "manifest.json"}
    if files and not required.issubset(files):
        missing = ", ".join(sorted(required - files.keys()))
        raise ValueError(f"partial encyclopedia namespace is missing {missing}")
    return files


def _write_mirror_candidate(candidate: Path, files: dict[str, bytes]) -> None:
    directories = {candidate}
    for relative, data in sorted(files.items()):
        destination = candidate / PurePosixPath(relative)
        destination.parent.mkdir(parents=True, exist_ok=True)
        directories.add(destination.parent)
        with destination.open("xb") as handle:
            handle.write(data)
            handle.flush()
            os.fsync(handle.fileno())
    for directory in sorted(directories, key=lambda path: len(path.parts), reverse=True):
        _fsync_directory(directory)


def _directory_tree_inventory(root: Path) -> tuple[dict[str, str], list[str]]:
    if root.is_symlink() or not root.is_dir():
        raise ValueError(f"unsafe encyclopedia mirror: {root}")
    inventory: dict[str, str] = {}
    directories = []
    for path in sorted(root.rglob("*")):
        relative = path.relative_to(root).as_posix()
        if path.is_symlink():
            raise ValueError(f"encyclopedia mirror contains a symlink: {relative}")
        if path.is_dir():
            directories.append(relative)
            continue
        if not path.is_file():
            raise ValueError(f"encyclopedia mirror contains a non-file: {relative}")
        inventory[relative] = sha256_file(path)
    return inventory, directories


def _mirror_inventory(root: Path) -> dict[str, str]:
    return _directory_tree_inventory(root)[0]


def _owned_mirror_directories(owned_files: dict[str, str]) -> list[str]:
    directories = set()
    for relative in owned_files:
        parent = PurePosixPath(relative).parent
        while parent != PurePosixPath("."):
            directories.add(parent.as_posix())
            parent = parent.parent
    return sorted(directories)


def _lstat(path: Path):
    try:
        return path.lstat()
    except FileNotFoundError:
        return None


def _file_artifact_identity(path: Path) -> dict:
    observed = _lstat(path)
    if observed is None or not stat.S_ISREG(observed.st_mode):
        raise ValueError(f"owned publication file is missing or unsafe: {path}")
    return {
        "kind": "file",
        "device": observed.st_dev,
        "inode": observed.st_ino,
        "byte_len": observed.st_size,
        "sha256": sha256_file(path),
    }


def _inventory_sha256(value: object) -> str:
    encoded = json.dumps(value, sort_keys=True, separators=(",", ":")).encode(
        "utf-8"
    )
    return hashlib.sha256(encoded).hexdigest()


def _directory_artifact_identity(path: Path) -> dict:
    observed = _lstat(path)
    if observed is None or not stat.S_ISDIR(observed.st_mode):
        raise ValueError(f"owned publication directory is missing or unsafe: {path}")
    files, directories = _directory_tree_inventory(path)
    return {
        "kind": "directory",
        "device": observed.st_dev,
        "inode": observed.st_ino,
        "file_count": len(files),
        "directory_count": len(directories),
        "files_sha256": _inventory_sha256(files),
        "directories_sha256": _inventory_sha256(directories),
    }


def _valid_artifact_identity(value: object, kind: str) -> bool:
    if not isinstance(value, dict) or value.get("kind") != kind:
        return False
    for key in ("device", "inode"):
        if not isinstance(value.get(key), int) or isinstance(value.get(key), bool):
            return False
        if value[key] < 0:
            return False
    if kind == "file":
        return (
            isinstance(value.get("byte_len"), int)
            and not isinstance(value.get("byte_len"), bool)
            and value["byte_len"] >= 0
            and valid_sha256(value.get("sha256"))
            and set(value) == {
                "kind",
                "device",
                "inode",
                "byte_len",
                "sha256",
            }
        )
    return (
        isinstance(value.get("file_count"), int)
        and not isinstance(value.get("file_count"), bool)
        and value["file_count"] >= 0
        and isinstance(value.get("directory_count"), int)
        and not isinstance(value.get("directory_count"), bool)
        and value["directory_count"] >= 0
        and valid_sha256(value.get("files_sha256"))
        and valid_sha256(value.get("directories_sha256"))
        and set(value)
        == {
            "kind",
            "device",
            "inode",
            "file_count",
            "directory_count",
            "files_sha256",
            "directories_sha256",
        }
    )


def _artifact_identity(path: Path, kind: str) -> dict | None:
    if _lstat(path) is None:
        return None
    return (
        _file_artifact_identity(path)
        if kind == "file"
        else _directory_artifact_identity(path)
    )


def _classify_artifact(
    path: Path, kind: str, expected: dict[str, dict | None]
) -> str | None:
    actual = _artifact_identity(path, kind)
    if actual is None:
        return None
    matches = [
        label
        for label, identity in expected.items()
        if identity is not None and actual == identity
    ]
    if len(matches) != 1:
        raise ValueError(
            f"unrecognized or ambiguous owned publication artifact: {path}"
        )
    return matches[0]


def _directory_is_owned_subset(
    path: Path, identity: dict, owned_files: dict[str, str]
) -> bool:
    observed = _lstat(path)
    if observed is None or not stat.S_ISDIR(observed.st_mode):
        return False
    if (observed.st_dev, observed.st_ino) != (
        identity["device"],
        identity["inode"],
    ):
        return False
    files, directories = _directory_tree_inventory(path)
    expected_directories = set(_owned_mirror_directories(owned_files))
    if not set(files).issubset(owned_files) or not set(directories).issubset(
        expected_directories
    ):
        raise ValueError(
            f"owned publication directory gained an unrecognized path: {path}"
        )
    if any(owned_files[relative] != digest for relative, digest in files.items()):
        raise ValueError(
            f"owned publication directory contains changed bytes: {path}"
        )
    return True


def _classify_directory_artifact(
    path: Path,
    expected: dict[str, tuple[dict | None, dict[str, str]]],
    partial_labels: set[str] | None = None,
) -> str | None:
    if _lstat(path) is None:
        return None
    partial_labels = partial_labels or set()
    actual = _directory_artifact_identity(path)
    matches = []
    for label, (identity, owned_files) in expected.items():
        if identity is None:
            continue
        if actual == identity or (
            label in partial_labels
            and _directory_is_owned_subset(path, identity, owned_files)
        ):
            matches.append(label)
    if len(matches) != 1:
        raise ValueError(
            f"unrecognized or ambiguous owned publication artifact: {path}"
        )
    return matches[0]


def _remove_owned_file(path: Path, identity: dict) -> None:
    if _file_artifact_identity(path) != identity:
        raise ValueError(f"refusing to delete an unowned publication file: {path}")
    path.unlink()
    _fsync_directory(path.parent)


def _remove_owned_directory(
    path: Path, identity: dict, owned_files: dict[str, str]
) -> None:
    if _lstat(path) is None:
        return
    if not _directory_is_owned_subset(path, identity, owned_files):
        raise ValueError(f"refusing to delete an unowned publication directory: {path}")
    for relative, digest in sorted(owned_files.items()):
        target = path / PurePosixPath(relative)
        observed = _lstat(target)
        if observed is None:
            continue
        if (
            not stat.S_ISREG(observed.st_mode)
            or sha256_file(target) != digest
        ):
            raise ValueError(
                f"refusing to delete a changed publication file: {target}"
            )
        target.unlink()
        _fsync_directory(target.parent)
    for relative in sorted(
        _owned_mirror_directories(owned_files),
        key=lambda value: len(PurePosixPath(value).parts),
        reverse=True,
    ):
        target = path / PurePosixPath(relative)
        if _lstat(target) is None:
            continue
        target.rmdir()
        _fsync_directory(target.parent)
    if _lstat(path) is not None:
        observed = _lstat(path)
        if (
            observed is None
            or not stat.S_ISDIR(observed.st_mode)
            or (observed.st_dev, observed.st_ino)
            != (identity["device"], identity["inode"])
        ):
            raise ValueError(
                f"refusing to delete a changed publication directory: {path}"
            )
        path.rmdir()
    _fsync_directory(path.parent)


def _complete_publication_record(
    output: Path, mirror: Path, pack_digest: str, files: dict[str, bytes]
) -> dict:
    return {
        "schema_version": PUBLICATION_SCHEMA_VERSION,
        "phase": "complete",
        "output": output.name,
        "mirror": mirror.name,
        "pack_sha256": pack_digest,
        "mirror_present": bool(files),
        "owned_files": {
            relative: hashlib.sha256(data).hexdigest()
            for relative, data in sorted(files.items())
        },
    }


def _validate_complete_record_shape(
    record: object, output: Path, mirror: Path, description: str
) -> dict:
    if not isinstance(record, dict) or record.get("phase") != "complete":
        raise ValueError(f"invalid {description} runtime publication inventory")
    owned = record.get("owned_files")
    if (
        record.get("schema_version") != PUBLICATION_SCHEMA_VERSION
        or record.get("output") != output.name
        or record.get("mirror") != mirror.name
        or not valid_sha256(record.get("pack_sha256"))
        or not isinstance(record.get("mirror_present"), bool)
        or not isinstance(owned, dict)
        or any(
            not isinstance(key, str) or not valid_sha256(value)
            for key, value in owned.items()
        )
        or set(record)
        != {
            "schema_version",
            "phase",
            "output",
            "mirror",
            "pack_sha256",
            "mirror_present",
            "owned_files",
        }
    ):
        raise ValueError(f"invalid {description} runtime publication inventory")
    if record["mirror_present"] != bool(owned):
        raise ValueError(f"invalid {description} runtime publication mirror inventory")
    try:
        for relative in owned:
            checked = (
                PurePosixPath(relative)
                if relative == "manifest.json"
                else checked_runtime_asset_path(relative)
            )
            if checked.as_posix() != relative:
                raise ValueError
    except ValueError as error:
        raise ValueError(
            f"invalid {description} runtime publication owned path"
        ) from error
    if owned and not {"catalog.json", "manifest.json"}.issubset(owned):
        raise ValueError(
            f"invalid {description} runtime publication required inventory"
        )
    return record


def _validate_complete_publication(
    record: dict | None, output: Path, mirror: Path
) -> None:
    if record is None:
        if mirror.exists():
            raise ValueError(
                f"encyclopedia mirror is not owned by a publication inventory: {mirror}"
            )
        return
    if record.get("phase") != "complete":
        raise ValueError("runtime publication requires recovery before validation")
    record = _validate_complete_record_shape(record, output, mirror, "completed")
    owned = record["owned_files"]
    if not output.is_file() or output.is_symlink():
        raise ValueError("owned runtime pack is missing or unsafe")
    if sha256_file(output) != record["pack_sha256"]:
        raise ValueError("owned runtime pack changed outside publication")
    if record["mirror_present"]:
        observed, observed_directories = _directory_tree_inventory(mirror)
        unknown = sorted(set(observed) - set(owned))
        if unknown:
            raise ValueError(
                "encyclopedia mirror contains files not owned by its inventory: "
                + ", ".join(unknown)
            )
        if observed != owned:
            raise ValueError("owned encyclopedia mirror changed outside publication")
        if observed_directories != _owned_mirror_directories(owned):
            raise ValueError(
                "owned encyclopedia mirror directory inventory changed outside publication"
            )
    elif mirror.exists():
        raise ValueError("unowned encyclopedia mirror exists beside an absent publication")


def _safe_record_candidate(parent: Path, name: object, prefix: str) -> Path | None:
    if name is None:
        return None
    if (
        not isinstance(name, str)
        or Path(name).name != name
        or not name.startswith(prefix)
        or not name.endswith(".tmp")
    ):
        raise ValueError("unsafe candidate path in runtime publication record")
    return parent / name


RECOVERY_PROGRESS = {
    None,
    "validated",
    "pack_restored",
    "mirror_restored",
    "pack_candidate_removed",
    "mirror_candidate_removed",
    "pack_backup_removed",
    "mirror_backup_removed",
}
ROLLBACK_RECOVERY_PROGRESS = (
    "validated",
    "pack_restored",
    "mirror_restored",
    "pack_candidate_removed",
    "mirror_candidate_removed",
)
COMMITTED_RECOVERY_PROGRESS = (
    "pack_backup_removed",
    "mirror_backup_removed",
    "pack_candidate_removed",
    "mirror_candidate_removed",
)


def _validate_pending_publication_record(
    record: dict, output: Path, mirror: Path
) -> dict:
    if set(record) != {
        "schema_version",
        "phase",
        "output",
        "mirror",
        "pack_candidate",
        "mirror_candidate",
        "previous_pack_exists",
        "previous_mirror_exists",
        "previous_pack_identity",
        "previous_mirror_identity",
        "pack_candidate_identity",
        "mirror_candidate_identity",
        "previous_record",
        "desired_record",
        "recovery_progress",
    }:
        raise ValueError("runtime publication recovery record fields are invalid")
    if (
        record.get("schema_version") != PUBLICATION_SCHEMA_VERSION
        or record.get("phase") not in PUBLICATION_PHASES - {"complete"}
    ):
        raise ValueError("runtime publication recovery record schema or phase is invalid")
    if record.get("output") != output.name or record.get("mirror") != mirror.name:
        raise ValueError("runtime publication record targets do not match this build")
    previous_pack = record.get("previous_pack_exists")
    previous_mirror = record.get("previous_mirror_exists")
    previous_record = record.get("previous_record")
    desired_record = _validate_complete_record_shape(
        record.get("desired_record"), output, mirror, "desired"
    )
    previous_pack_identity = record.get("previous_pack_identity")
    previous_mirror_identity = record.get("previous_mirror_identity")
    pack_candidate_identity = record.get("pack_candidate_identity")
    mirror_candidate_identity = record.get("mirror_candidate_identity")
    if not isinstance(previous_pack, bool) or not isinstance(previous_mirror, bool):
        raise ValueError("runtime publication recovery record is incomplete")
    if previous_record is not None:
        previous_record = _validate_complete_record_shape(
            previous_record, output, mirror, "previous"
        )
        if not previous_pack:
            raise ValueError("runtime publication previous pack presence is contradictory")
    if previous_pack != (previous_pack_identity is not None) or (
        previous_pack_identity is not None
        and not _valid_artifact_identity(previous_pack_identity, "file")
    ):
        raise ValueError("runtime publication previous pack identity is invalid")
    if previous_mirror != (previous_mirror_identity is not None) or (
        previous_mirror_identity is not None
        and not _valid_artifact_identity(previous_mirror_identity, "directory")
    ):
        raise ValueError("runtime publication previous mirror identity is invalid")
    if not _valid_artifact_identity(pack_candidate_identity, "file") or not (
        _valid_artifact_identity(mirror_candidate_identity, "directory")
    ):
        raise ValueError("runtime publication candidate identity is invalid")
    if pack_candidate_identity["sha256"] != desired_record["pack_sha256"]:
        raise ValueError("runtime publication pack candidate identity is contradictory")
    if (
        mirror_candidate_identity["file_count"]
        != len(desired_record["owned_files"])
        or mirror_candidate_identity["files_sha256"]
        != _inventory_sha256(desired_record["owned_files"])
        or mirror_candidate_identity["directory_count"]
        != len(_owned_mirror_directories(desired_record["owned_files"]))
        or mirror_candidate_identity["directories_sha256"]
        != _inventory_sha256(
            _owned_mirror_directories(desired_record["owned_files"])
        )
    ):
        raise ValueError("runtime publication mirror candidate identity is contradictory")
    if previous_record is not None:
        if previous_pack_identity["sha256"] != previous_record["pack_sha256"]:
            raise ValueError("runtime publication previous pack identity is contradictory")
        if previous_record["mirror_present"] != previous_mirror:
            raise ValueError("runtime publication previous mirror presence is contradictory")
        if previous_mirror and (
            previous_mirror_identity["file_count"]
            != len(previous_record["owned_files"])
            or previous_mirror_identity["files_sha256"]
            != _inventory_sha256(previous_record["owned_files"])
            or previous_mirror_identity["directory_count"]
            != len(_owned_mirror_directories(previous_record["owned_files"]))
            or previous_mirror_identity["directories_sha256"]
            != _inventory_sha256(
                _owned_mirror_directories(previous_record["owned_files"])
            )
        ):
            raise ValueError("runtime publication previous mirror identity is contradictory")
    elif previous_mirror:
        raise ValueError("runtime publication has an unowned previous mirror")
    progress = record.get("recovery_progress")
    allowed_progress = (
        COMMITTED_RECOVERY_PROGRESS
        if record["phase"] == "committed"
        else ROLLBACK_RECOVERY_PROGRESS
    )
    if progress not in RECOVERY_PROGRESS or (
        progress is not None and progress not in allowed_progress
    ):
        raise ValueError("runtime publication recovery progress is invalid")
    return {
        "previous_pack": previous_pack_identity,
        "previous_mirror": previous_mirror_identity,
        "pack_candidate": pack_candidate_identity,
        "mirror_candidate": mirror_candidate_identity,
        "previous_record": previous_record,
        "desired_record": desired_record,
    }


def _validate_recovery_artifacts(
    output: Path,
    mirror: Path,
    paths: dict[str, Path],
    pack_candidate: Path,
    mirror_candidate: Path,
    identities: dict,
    record: dict,
) -> dict[str, str | None]:
    previous_files = (
        identities["previous_record"]["owned_files"]
        if identities["previous_record"] is not None
        else {}
    )
    desired_files = identities["desired_record"]["owned_files"]
    progress = record.get("recovery_progress")
    states = {
        "output": _classify_artifact(
            output,
            "file",
            {
                "previous": identities["previous_pack"],
                "desired": identities["pack_candidate"],
            },
        ),
        "pack_backup": _classify_artifact(
            paths["pack_backup"],
            "file",
            {"previous": identities["previous_pack"]},
        ),
        "pack_candidate": _classify_artifact(
            pack_candidate,
            "file",
            {"desired": identities["pack_candidate"]},
        ),
        "mirror": _classify_directory_artifact(
            mirror,
            {
                "previous": (identities["previous_mirror"], previous_files),
                "desired": (identities["mirror_candidate"], desired_files),
            },
            {"desired"} if progress == "pack_restored" else set(),
        ),
        "mirror_backup": _classify_directory_artifact(
            paths["mirror_backup"],
            {"previous": (identities["previous_mirror"], previous_files)},
            {"previous"}
            if record["phase"] == "committed"
            and progress == "pack_backup_removed"
            else set(),
        ),
        "mirror_candidate": _classify_directory_artifact(
            mirror_candidate,
            {"desired": (identities["mirror_candidate"], desired_files)},
            {"desired"} if progress == "pack_candidate_removed" else set(),
        ),
    }
    for label, members in (
        ("previous pack", ("output", "pack_backup")),
        ("desired pack", ("output", "pack_candidate")),
        ("previous mirror", ("mirror", "mirror_backup")),
        ("desired mirror", ("mirror", "mirror_candidate")),
    ):
        state_name = "previous" if label.startswith("previous") else "desired"
        if sum(states[member] == state_name for member in members) > 1:
            raise ValueError(f"ambiguous duplicate {label} recovery artifacts")
    return states


def _write_recovery_progress(
    record_path: Path, record: dict, progress: str
) -> dict:
    order = (
        COMMITTED_RECOVERY_PROGRESS
        if record["phase"] == "committed"
        else ROLLBACK_RECOVERY_PROGRESS
    )
    current = record.get("recovery_progress")
    if current is not None and order.index(current) >= order.index(progress):
        return record
    updated = dict(record)
    updated["recovery_progress"] = progress
    _write_publication_record(record_path, updated)
    return updated


def _restore_previous_pack(
    output: Path,
    backup: Path,
    identities: dict,
    states: dict[str, str | None],
) -> None:
    previous = identities["previous_pack"]
    desired = identities["pack_candidate"]
    if previous is None:
        if states["pack_backup"] is not None:
            raise ValueError("unexpected runtime pack backup without a previous pack")
        if states["output"] == "desired":
            _remove_owned_file(output, desired)
        elif states["output"] is not None:
            raise ValueError("cannot recover absent previous runtime pack")
        return
    if states["output"] == "previous":
        if states["pack_backup"] is not None:
            raise ValueError("duplicate previous runtime pack during recovery")
        return
    if states["pack_backup"] != "previous":
        raise ValueError("cannot recover previous runtime pack: owned backup is missing")
    if states["output"] == "desired":
        _remove_owned_file(output, desired)
    elif states["output"] is not None:
        raise ValueError("cannot recover previous runtime pack from current output")
    os.replace(backup, output)
    _fsync_directory(output.parent)
    if _file_artifact_identity(output) != previous:
        raise ValueError("restored runtime pack does not match its prior identity")


def _restore_previous_mirror(
    mirror: Path,
    backup: Path,
    identities: dict,
    states: dict[str, str | None],
) -> None:
    previous = identities["previous_mirror"]
    desired = identities["mirror_candidate"]
    desired_files = identities["desired_record"]["owned_files"]
    if previous is None:
        if states["mirror_backup"] is not None:
            raise ValueError("unexpected mirror backup without a previous mirror")
        if states["mirror"] == "desired":
            _remove_owned_directory(mirror, desired, desired_files)
        elif states["mirror"] is not None:
            raise ValueError("cannot recover absent previous encyclopedia mirror")
        return
    if states["mirror"] == "previous":
        if states["mirror_backup"] is not None:
            raise ValueError("duplicate previous encyclopedia mirror during recovery")
        return
    if states["mirror_backup"] != "previous":
        raise ValueError(
            "cannot recover previous encyclopedia mirror: owned backup is missing"
        )
    if states["mirror"] == "desired":
        _remove_owned_directory(mirror, desired, desired_files)
    elif states["mirror"] is not None:
        raise ValueError("cannot recover previous encyclopedia mirror from current output")
    os.replace(backup, mirror)
    _fsync_directory(mirror.parent)
    if _directory_artifact_identity(mirror) != previous:
        raise ValueError("restored encyclopedia mirror does not match its prior identity")


def _remove_owned_if_present(
    path: Path,
    kind: str,
    identity: dict,
    owned_files: dict[str, str] | None = None,
) -> None:
    if _lstat(path) is None:
        return
    if kind == "file":
        _remove_owned_file(path, identity)
    else:
        _remove_owned_directory(path, identity, owned_files or {})


def _recover_runtime_publication(
    record: dict,
    record_path: Path,
    output: Path,
    mirror: Path,
    paths: dict[str, Path],
) -> None:
    if record.get("phase") == "complete":
        return
    parent = output.parent.resolve(strict=True)
    pack_candidate = _safe_record_candidate(
        parent, record.get("pack_candidate"), f".{output.name}."
    )
    mirror_candidate = _safe_record_candidate(
        parent, record.get("mirror_candidate"), f".{mirror.name}."
    )
    if pack_candidate is None or mirror_candidate is None:
        raise ValueError("runtime publication recovery candidates are incomplete")
    identities = _validate_pending_publication_record(record, output, mirror)
    states = _validate_recovery_artifacts(
        output,
        mirror,
        paths,
        pack_candidate,
        mirror_candidate,
        identities,
        record,
    )
    desired_record = identities["desired_record"]
    previous_record = identities["previous_record"]
    previous_files = (
        previous_record["owned_files"] if previous_record is not None else {}
    )
    desired_files = desired_record["owned_files"]

    if record["phase"] == "committed":
        _validate_complete_publication(desired_record, output, mirror)
        _remove_owned_if_present(
            paths["pack_backup"], "file", identities["previous_pack"]
        )
        record = _write_recovery_progress(
            record_path, record, "pack_backup_removed"
        )
        _remove_owned_if_present(
            paths["mirror_backup"],
            "directory",
            identities["previous_mirror"],
            previous_files,
        )
        record = _write_recovery_progress(
            record_path, record, "mirror_backup_removed"
        )
        _remove_owned_if_present(
            pack_candidate, "file", identities["pack_candidate"]
        )
        record = _write_recovery_progress(
            record_path, record, "pack_candidate_removed"
        )
        _remove_owned_if_present(
            mirror_candidate,
            "directory",
            identities["mirror_candidate"],
            desired_files,
        )
        _write_recovery_progress(
            record_path, record, "mirror_candidate_removed"
        )
        _write_publication_record(record_path, desired_record)
        return

    record = _write_recovery_progress(record_path, record, "validated")
    _restore_previous_pack(
        output, paths["pack_backup"], identities, states
    )
    record = _write_recovery_progress(record_path, record, "pack_restored")
    current_states = _validate_recovery_artifacts(
        output,
        mirror,
        paths,
        pack_candidate,
        mirror_candidate,
        identities,
        record,
    )
    _restore_previous_mirror(
        mirror, paths["mirror_backup"], identities, current_states
    )
    record = _write_recovery_progress(record_path, record, "mirror_restored")
    _remove_owned_if_present(pack_candidate, "file", identities["pack_candidate"])
    record = _write_recovery_progress(
        record_path, record, "pack_candidate_removed"
    )
    _remove_owned_if_present(
        mirror_candidate,
        "directory",
        identities["mirror_candidate"],
        desired_files,
    )
    record = _write_recovery_progress(
        record_path, record, "mirror_candidate_removed"
    )
    if previous_record is None:
        record_path.unlink()
        _fsync_directory(record_path.parent)
    else:
        _write_publication_record(record_path, previous_record)


def publish_runtime_artifacts(
    entries: list[Entry], output: Path, mirror: Path
) -> int:
    """Publish an ORPK and loose encyclopedia mirror as one recoverable generation."""
    paths = _publication_paths(output, mirror)
    _validate_publication_input_paths(entries, output, mirror, paths)
    with _publication_lock(paths["lock"]):
        current = _read_publication_record(paths["record"])
        if current is not None and current.get("phase") != "complete":
            _recover_runtime_publication(current, paths["record"], output, mirror, paths)
            current = _read_publication_record(paths["record"])
        _validate_complete_publication(current, output, mirror)
        unexpected_backups = [
            str(path)
            for path in (paths["pack_backup"], paths["mirror_backup"])
            if _lstat(path) is not None
        ]
        if unexpected_backups:
            raise ValueError(
                "runtime publication backup exists without a pending owned "
                "transaction: " + ", ".join(unexpected_backups)
            )

        files = _encyclopedia_mirror_files(entries)
        pack_descriptor, pack_name = tempfile.mkstemp(
            prefix=f".{output.name}.", suffix=".tmp", dir=output.parent
        )
        os.close(pack_descriptor)
        pack_candidate = Path(pack_name)
        mirror_candidate = Path(
            tempfile.mkdtemp(
                prefix=f".{mirror.name}.", suffix=".tmp", dir=mirror.parent
            )
        )
        pack_candidate_identity = None
        mirror_candidate_identity = None
        try:
            written = _write_pack_candidate(entries, pack_candidate)
            _write_mirror_candidate(mirror_candidate, files)
            pack_candidate_identity = _file_artifact_identity(pack_candidate)
            mirror_candidate_identity = _directory_artifact_identity(
                mirror_candidate
            )
            desired = _complete_publication_record(
                output, mirror, sha256_file(pack_candidate), files
            )
            if current == desired:
                _remove_owned_file(pack_candidate, pack_candidate_identity)
                _remove_owned_directory(
                    mirror_candidate,
                    mirror_candidate_identity,
                    desired["owned_files"],
                )
                return written

            for backup in (paths["pack_backup"], paths["mirror_backup"]):
                if _lstat(backup) is not None:
                    raise ValueError(
                        f"runtime publication backup exists without recovery record: {backup}"
                    )
            previous_pack_identity = (
                _file_artifact_identity(output)
                if _lstat(output) is not None
                else None
            )
            previous_mirror_identity = (
                _directory_artifact_identity(mirror)
                if _lstat(mirror) is not None
                else None
            )
            pending = {
                "schema_version": PUBLICATION_SCHEMA_VERSION,
                "phase": "prepared",
                "output": output.name,
                "mirror": mirror.name,
                "pack_candidate": pack_candidate.name,
                "mirror_candidate": mirror_candidate.name,
                "previous_pack_exists": previous_pack_identity is not None,
                "previous_mirror_exists": previous_mirror_identity is not None,
                "previous_pack_identity": previous_pack_identity,
                "previous_mirror_identity": previous_mirror_identity,
                "pack_candidate_identity": pack_candidate_identity,
                "mirror_candidate_identity": mirror_candidate_identity,
                "previous_record": current,
                "desired_record": desired,
                "recovery_progress": None,
            }
            _write_publication_record(paths["record"], pending)
            try:
                if _lstat(output) is not None:
                    os.replace(output, paths["pack_backup"])
                pending["phase"] = "old_pack_backed_up"
                _write_publication_record(paths["record"], pending)

                if _lstat(mirror) is not None:
                    os.replace(mirror, paths["mirror_backup"])
                pending["phase"] = "old_mirror_backed_up"
                _write_publication_record(paths["record"], pending)

                if files:
                    os.replace(mirror_candidate, mirror)
                else:
                    _remove_owned_directory(
                        mirror_candidate,
                        mirror_candidate_identity,
                        desired["owned_files"],
                    )
                pending["phase"] = "mirror_published"
                _write_publication_record(paths["record"], pending)

                os.replace(pack_candidate, output)
                pending["phase"] = "pack_published"
                _write_publication_record(paths["record"], pending)
                pending["phase"] = "committed"
                _write_publication_record(paths["record"], pending)
                _recover_runtime_publication(
                    pending, paths["record"], output, mirror, paths
                )
            except BaseException:
                recovery = _read_publication_record(paths["record"])
                if recovery is not None:
                    _recover_runtime_publication(
                        recovery, paths["record"], output, mirror, paths
                    )
                raise

            return written
        except BaseException:
            recovery = _read_publication_record(paths["record"])
            if recovery is None or recovery.get("phase") == "complete":
                if (
                    pack_candidate_identity is not None
                    and _lstat(pack_candidate) is not None
                ):
                    _remove_owned_file(pack_candidate, pack_candidate_identity)
                if (
                    mirror_candidate_identity is not None
                    and _lstat(mirror_candidate) is not None
                ):
                    _remove_owned_directory(
                        mirror_candidate,
                        mirror_candidate_identity,
                        desired["owned_files"],
                    )
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
    parser.add_argument(
        "--encyclopedia-mirror",
        type=Path,
        help=(
            "publish the retained canonical encyclopedia generation as a loose "
            "sibling of the runtime pack"
        ),
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
    written = (
        publish_runtime_artifacts(entries, args.output, args.encyclopedia_mirror)
        if args.encyclopedia_mirror is not None
        else write_pack(entries, args.output)
    )

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
