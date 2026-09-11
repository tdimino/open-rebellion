#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = ["pillow"]
# ///
"""Generate and verify manifest-governed faithful-HD assets.

This is the deterministic Tier 1 route. It preserves indexed palettes and uses
nearest-neighbor 4x scaling. Learned-model candidates belong in a later,
separately measured Tier 2 adapter and must produce the same manifest shape.

Generated assets are never runtime-eligible until an explicit review records
``approved: true`` with a reviewer and evidence path.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import platform
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterable

import PIL
from PIL import Image


PROJECT_ROOT = Path(__file__).resolve().parent.parent
DEFAULT_UI_SOURCE = PROJECT_ROOT / "data" / "base" / "ui"
DEFAULT_EDATA_SOURCE = PROJECT_ROOT / "data" / "base" / "EData"
DEFAULT_OUTPUT = PROJECT_ROOT / "data" / "hd"
DEFAULT_MANIFEST = DEFAULT_OUTPUT / "manifest.json"
DEFAULT_MODULE_ROOT = Path(
    os.environ.get("REBELLION_GAME_DIR", PROJECT_ROOT.parent / "star-wars-rebellion")
)
SCHEMA_VERSION = 1
PROFILE = "faithful-hd"
METHOD = "nearest-4x"
SCALE = 4


@dataclass(frozen=True)
class AssetJob:
    key: str
    source: Path
    output: Path
    module: str
    resource_id: int
    module_source: Path | None = None


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def canonical_digest(value: Any) -> str:
    encoded = json.dumps(
        value, sort_keys=True, separators=(",", ":"), ensure_ascii=True
    ).encode("utf-8")
    return sha256_bytes(encoded)


def relative_path(path: Path) -> str:
    try:
        return path.resolve().relative_to(PROJECT_ROOT.resolve()).as_posix()
    except ValueError:
        return path.resolve().as_posix()


def empty_manifest() -> dict[str, Any]:
    return {
        "schema_version": SCHEMA_VERSION,
        "profile": PROFILE,
        "generator": {
            "path": relative_path(Path(__file__)),
            "sha256": sha256_file(Path(__file__)),
            "python": platform.python_version(),
            "pillow": PIL.__version__,
        },
        "assets": {},
    }


def load_manifest(path: Path) -> dict[str, Any]:
    if not path.exists():
        return empty_manifest()
    manifest = json.loads(path.read_text(encoding="utf-8"))
    if manifest.get("schema_version") != SCHEMA_VERSION:
        raise ValueError(
            f"unsupported manifest schema {manifest.get('schema_version')!r}"
        )
    if manifest.get("profile") != PROFILE:
        raise ValueError(f"manifest profile must be {PROFILE!r}")
    if not isinstance(manifest.get("assets"), dict):
        raise ValueError("manifest assets must be an object")
    if not isinstance(manifest.get("generator"), dict):
        raise ValueError("manifest generator must be an object")
    return manifest


def write_manifest(path: Path, manifest: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = json.dumps(manifest, indent=2, sort_keys=True) + "\n"
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(payload, encoding="utf-8")
    os.replace(temporary, path)


def palette_sha256(image: Image.Image) -> str | None:
    palette = image.getpalette()
    if palette is None:
        return None
    return sha256_bytes(bytes(palette))


def transparency_record(image: Image.Image) -> dict[str, Any]:
    transparency = image.info.get("transparency")
    if isinstance(transparency, int):
        return {"kind": "palette-index", "index": transparency}
    if isinstance(transparency, bytes):
        return {"kind": "palette-alpha-table", "sha256": sha256_bytes(transparency)}
    return {"kind": "none"}


def nearest_upscale(source: Path, output: Path) -> tuple[Image.Image, Image.Image]:
    with Image.open(source) as opened:
        original = opened.copy()
        original.info = opened.info.copy()

    upscaled = original.resize(
        (original.width * SCALE, original.height * SCALE),
        resample=Image.Resampling.NEAREST,
    )
    output.parent.mkdir(parents=True, exist_ok=True)
    temporary = output.with_suffix(output.suffix + ".tmp")
    save_args: dict[str, Any] = {"format": "PNG", "optimize": False, "compress_level": 9}
    transparency = original.info.get("transparency")
    if transparency is not None:
        save_args["transparency"] = transparency
    upscaled.save(temporary, **save_args)
    os.replace(temporary, output)
    return original, upscaled


def exact_nearest_match(source: Path, output: Path) -> bool:
    try:
        with Image.open(source) as source_image, Image.open(output) as output_image:
            expected = source_image.resize(
                (source_image.width * SCALE, source_image.height * SCALE),
                resample=Image.Resampling.NEAREST,
            ).convert("RGBA")
            actual = output_image.convert("RGBA")
            return actual.size == expected.size and actual.tobytes() == expected.tobytes()
    except (OSError, ValueError):
        return False


def build_record(job: AssetJob, original: Image.Image, upscaled: Image.Image) -> dict[str, Any]:
    module_path = job.module_source if job.module_source and job.module_source.is_file() else None
    route = {
        "profile": PROFILE,
        "tier": 1,
        "method": METHOD,
        "scale": SCALE,
        "deterministic": True,
        "model_sha256": None,
        "device": "cpu",
        "implementation_sha256": sha256_file(Path(__file__)),
    }
    return {
        "approved": False,
        "review": None,
        "asset_id": {
            "module": job.module,
            "resource_type": "BMP",
            "resource_id": job.resource_id,
            "locale": "en",
        },
        "source": {
            "path": relative_path(job.source),
            "sha256": sha256_file(job.source),
            "module_path": relative_path(module_path) if module_path else None,
            "module_sha256": sha256_file(module_path) if module_path else None,
            "palette_sha256": palette_sha256(original),
            "width": original.width,
            "height": original.height,
            "mode": original.mode,
            "transparency": transparency_record(original),
        },
        "classification": {
            "family": "unclassified",
            "protected_masks": ["palette-indices"] if original.mode == "P" else [],
        },
        "route": {**route, "config_sha256": canonical_digest(route)},
        "output": {
            "path": relative_path(job.output),
            "sha256": sha256_file(job.output),
            "width": upscaled.width,
            "height": upscaled.height,
            "mode": upscaled.mode,
            "format": "png",
            "color_space": "srgb",
            "alpha_mode": "straight",
        },
        "gates": {
            "exact_nearest_reconstruction": exact_nearest_match(job.source, job.output),
            "human_review": "pending",
        },
    }


def ui_jobs(
    source_root: Path,
    output_root: Path,
    dll: str | None,
    module_root: Path | None = None,
) -> Iterable[AssetJob]:
    directories = [source_root / dll] if dll else sorted(source_root.glob("*-dll"))
    for directory in directories:
        bmp_dir = directory / "BMP"
        if not bmp_dir.is_dir():
            continue
        module = f"{directory.name.removesuffix('-dll').upper()}.DLL"
        for source in sorted(bmp_dir.glob("*.bmp")):
            try:
                resource_id = int(source.stem)
            except ValueError:
                continue
            key = f"{directory.name}/{resource_id}"
            yield AssetJob(
                key=key,
                source=source,
                output=output_root / directory.name / f"{resource_id}.png",
                module=module,
                resource_id=resource_id,
                module_source=(module_root / module) if module_root else None,
            )


def edata_jobs(source_root: Path, output_root: Path) -> Iterable[AssetJob]:
    for source in sorted(source_root.glob("EDATA.*")):
        try:
            resource_id = int(source.suffix[1:])
        except ValueError:
            continue
        name = f"EDATA_{resource_id:03}"
        yield AssetJob(
            key=f"edata/{name}",
            source=source,
            output=output_root / "EData" / f"{name}.png",
            module="EDATA",
            resource_id=resource_id,
        )


def matching_record(job: AssetJob, record: dict[str, Any] | None) -> bool:
    if not isinstance(record, dict) or not job.output.exists():
        return False
    source_record = record.get("source")
    output_record = record.get("output")
    asset_id = record.get("asset_id")
    if not all(isinstance(value, dict) for value in (source_record, output_record, asset_id)):
        return False
    if (
        source_record.get("path") != relative_path(job.source)
        or output_record.get("path") != relative_path(job.output)
        or asset_id.get("module") != job.module
        or asset_id.get("resource_id") != job.resource_id
    ):
        return False
    if verify_record(record, job.key) is not None:
        return False
    module_source = job.module_source
    recorded_module_hash = record.get("source", {}).get("module_sha256")
    if module_source is not None and module_source.is_file():
        return (
            source_record.get("module_path") == relative_path(module_source)
            and recorded_module_hash == sha256_file(module_source)
        )
    return source_record.get("module_path") is None and recorded_module_hash is None


def generate(
    jobs: Iterable[AssetJob], manifest_path: Path, force: bool, limit: int | None
) -> int:
    manifest = load_manifest(manifest_path)
    manifest["generator"] = empty_manifest()["generator"]
    selected = list(jobs)
    if limit is not None:
        selected = selected[:limit]
    if not selected:
        print("No source assets found", file=sys.stderr)
        return 1

    generated = 0
    skipped = 0
    blocked = 0
    for job in selected:
        existing = manifest["assets"].get(job.key)
        if matching_record(job, existing) and not force:
            skipped += 1
            continue
        if job.output.exists() and not force:
            print(
                f"UNAPPROVED {job.key}: output exists without matching provenance; use --force",
                file=sys.stderr,
            )
            blocked += 1
            continue

        original, upscaled = nearest_upscale(job.source, job.output)
        record = build_record(job, original, upscaled)
        if not record["gates"]["exact_nearest_reconstruction"]:
            raise RuntimeError(f"exact reconstruction failed for {job.key}")
        manifest["assets"][job.key] = record
        generated += 1

    write_manifest(manifest_path, manifest)
    print(f"generated={generated} skipped={skipped} blocked={blocked}")
    return 1 if blocked else 0


def verify(manifest_path: Path) -> int:
    if not manifest_path.is_file():
        print(f"Manifest does not exist: {manifest_path}", file=sys.stderr)
        return 1
    manifest = load_manifest(manifest_path)
    failures: list[str] = []
    failed_keys: set[str] = set()
    generator = manifest["generator"]
    if not valid_sha256(generator.get("sha256")):
        failures.append("manifest generator SHA-256 is invalid")
    for key, record in sorted(manifest["assets"].items()):
        record_failures: list[str] = []
        verification_error = verify_record(record, key)
        if verification_error is not None:
            record_failures.append(f"{key}: {verification_error}")
        if record.get("approved"):
            review = record.get("review")
            if (
                record.get("gates", {}).get("human_review") != "pass"
                or not isinstance(review, dict)
                or not isinstance(review.get("reviewer"), str)
                or not review["reviewer"].strip()
                or not isinstance(review.get("evidence"), str)
                or not review["evidence"].strip()
            ):
                record_failures.append(f"{key}: approved without complete human review")
        if record_failures:
            failures.extend(record_failures)
            failed_keys.add(key)

    for failure in failures:
        print(f"FAIL {failure}", file=sys.stderr)
    print(
        f"verified={len(manifest['assets']) - len(failed_keys)} "
        f"failed_assets={len(failed_keys)} failures={len(failures)}"
    )
    return 1 if failures else 0


def approve(manifest_path: Path, key: str, reviewer: str, evidence: str) -> int:
    if not manifest_path.is_file():
        print(f"Manifest does not exist: {manifest_path}", file=sys.stderr)
        return 1
    manifest = load_manifest(manifest_path)
    reviewer = reviewer.strip()
    evidence = evidence.strip()
    if not reviewer or not evidence:
        print("Reviewer and evidence must be non-empty", file=sys.stderr)
        return 1
    record = manifest["assets"].get(key)
    if record is None:
        print(f"Unknown asset key: {key}", file=sys.stderr)
        return 1
    verification_error = verify_record(record, key)
    if verification_error is not None:
        print(f"Asset cannot be approved: {verification_error}", file=sys.stderr)
        return 1
    record["approved"] = True
    record["review"] = {"reviewer": reviewer, "evidence": evidence}
    record["gates"]["human_review"] = "pass"
    manifest["generator"] = empty_manifest()["generator"]
    write_manifest(manifest_path, manifest)
    print(f"approved={key}")
    return 0


def verify_record(record: dict[str, Any], key: str | None = None) -> str | None:
    asset_id = record.get("asset_id", {})
    source_record = record.get("source", {})
    output_record = record.get("output", {})
    route = record.get("route", {})
    gates = record.get("gates", {})
    if not all(
        isinstance(value, dict)
        for value in (asset_id, source_record, output_record, route, gates)
    ):
        return "asset identity, source, output, route, and gates must be objects"
    resource_id = asset_id.get("resource_id")
    module = asset_id.get("module")
    if (
        not isinstance(resource_id, int)
        or not isinstance(module, str)
        or asset_id.get("resource_type") != "BMP"
        or asset_id.get("locale") != "en"
    ):
        return "asset identity is invalid"
    if key is not None:
        expected_key = (
            f"edata/EDATA_{resource_id:03}"
            if module == "EDATA"
            else f"{module.removesuffix('.DLL').lower()}-dll/{resource_id}"
        )
        if key != expected_key:
            return "asset key does not match its identity"
    source = PROJECT_ROOT / source_record.get("path", "")
    output = PROJECT_ROOT / output_record.get("path", "")
    if not source.is_file() or not output.is_file():
        return "source or output missing"
    if sha256_file(source) != source_record.get("sha256"):
        return "source hash mismatch"
    if sha256_file(output) != output_record.get("sha256"):
        return "output hash mismatch"
    module_path = source_record.get("module_path")
    module_hash = source_record.get("module_sha256")
    if module_path is not None:
        module_source = PROJECT_ROOT / module_path
        if not module_source.is_file() or sha256_file(module_source) != module_hash:
            return "source module hash mismatch"
    elif module_hash is not None:
        return "source module path is missing"
    if route.get("profile") != PROFILE or route.get("method") != METHOD:
        return "unsupported profile or route"
    if route.get("tier") != 1 or route.get("scale") != SCALE:
        return "unsupported tier or scale"
    if route.get("deterministic") is not True or route.get("device") != "cpu":
        return "deterministic route metadata mismatch"
    if route.get("model_sha256") is not None:
        return "deterministic nearest route must not name a model"
    if not valid_sha256(route.get("implementation_sha256")):
        return "implementation SHA-256 is invalid"
    recorded_config = route.get("config_sha256")
    config = {key: value for key, value in route.items() if key != "config_sha256"}
    if recorded_config != canonical_digest(config):
        return "route configuration hash mismatch"
    try:
        with Image.open(source) as source_image, Image.open(output) as output_image:
            if source_image.size != (
                source_record.get("width"),
                source_record.get("height"),
            ) or source_image.mode != source_record.get("mode"):
                return "source image metadata mismatch"
            if palette_sha256(source_image) != source_record.get("palette_sha256"):
                return "source palette hash mismatch"
            if transparency_record(source_image) != source_record.get("transparency"):
                return "source transparency metadata mismatch"
            if output_image.size != (
                output_record.get("width"),
                output_record.get("height"),
            ) or output_image.mode != output_record.get("mode"):
                return "output image metadata mismatch"
            if output_image.format != "PNG":
                return "output format mismatch"
    except (OSError, ValueError):
        return "source or output image cannot be decoded"
    if output_record.get("format") != "png":
        return "output format metadata mismatch"
    if output_record.get("color_space") != "srgb":
        return "output color-space metadata mismatch"
    if output_record.get("alpha_mode") != "straight":
        return "output alpha-mode metadata mismatch"
    if not exact_nearest_match(source, output):
        return "nearest reconstruction mismatch"
    if gates.get("exact_nearest_reconstruction") is not True:
        return "exact reconstruction gate is not recorded"
    return None


def valid_sha256(value: Any) -> bool:
    return (
        isinstance(value, str)
        and len(value) == 64
        and all(character in "0123456789abcdefABCDEF" for character in value)
    )


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    subparsers = parser.add_subparsers(dest="command", required=True)

    generate_parser = subparsers.add_parser("generate")
    generate_parser.add_argument("--kind", choices=["ui", "edata"], default="ui")
    generate_parser.add_argument("--source", type=Path)
    generate_parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    generate_parser.add_argument("--dll")
    generate_parser.add_argument("--module-root", type=Path, default=DEFAULT_MODULE_ROOT)
    generate_parser.add_argument("--limit", type=int)
    generate_parser.add_argument("--force", action="store_true")

    subparsers.add_parser("verify")

    approve_parser = subparsers.add_parser("approve")
    approve_parser.add_argument("key")
    approve_parser.add_argument("--reviewer", required=True)
    approve_parser.add_argument("--evidence", required=True)
    return parser


def main() -> int:
    args = build_parser().parse_args()
    manifest_path = args.manifest.resolve()
    if args.command == "verify":
        return verify(manifest_path)
    if args.command == "approve":
        return approve(manifest_path, args.key, args.reviewer, args.evidence)

    output_root = args.output.resolve()
    if args.kind == "ui":
        source_root = (args.source or DEFAULT_UI_SOURCE).resolve()
        jobs = ui_jobs(source_root, output_root, args.dll, args.module_root.resolve())
    else:
        if args.dll:
            raise ValueError("--dll applies only to --kind ui")
        source_root = (args.source or DEFAULT_EDATA_SOURCE).resolve()
        jobs = edata_jobs(source_root, output_root)
    return generate(jobs, manifest_path, args.force, args.limit)


if __name__ == "__main__":
    raise SystemExit(main())
