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
RESOURCE_TYPES = ("BMP", "TYPE302")
# Advisor droid frames (type-302) are additive deltas chained from an anchor BMP; the
# family is the unit of work and of fallback (plan.json method_routing.indexed_animation).
ADVISOR_FAMILY_TRANSFORM = "deterministic-family-transform"
ADVISOR_CADENCE_SECONDS = 0.15
SPRITES_DIR = Path(__file__).resolve().parent / "sprites"


def _type302():
    """Lazy import of scripts/sprites/type302.py (kept out of the BMP/EDATA paths)."""
    import importlib.util

    cached = sys.modules.get("rebellion_sprites_type302")
    if cached is not None:
        return cached
    spec = importlib.util.spec_from_file_location(
        "rebellion_sprites_type302", SPRITES_DIR / "type302.py"
    )
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


@dataclass(frozen=True)
class AssetJob:
    key: str
    source: Path
    output: Path
    module: str
    resource_id: int
    module_source: Path | None = None
    resource_type: str = "BMP"
    family: str | None = None
    # TYPE302 only: anchor BMP followed by every authored frame up to and including this one.
    chain: tuple[Path, ...] = ()
    frame_index: int = 0
    family_size: int = 0


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


def decode_chain(chain: Iterable[Path]) -> Image.Image:
    """Logical mode-P frame for a TYPE302 chain: anchor BMP, then frames in authored order."""
    paths = list(chain)
    if len(paths) < 2:
        raise ValueError("a type-302 chain needs an anchor and at least one frame")
    t302 = _type302()
    anchor = t302.decode_anchor_bitmap(paths[0].read_bytes())
    frames = [(index, path.read_bytes()) for index, path in enumerate(paths[1:])]
    _, decoded = t302.decode_family(anchor, frames)[-1]
    return t302.to_image(decoded, anchor.palette)


def load_logical(job: AssetJob) -> Image.Image:
    """The exact indexed image the route scales: a BMP file, or a decoded type-302 chain."""
    if job.resource_type == "TYPE302":
        return decode_chain(job.chain)
    with Image.open(job.source) as opened:
        original = opened.copy()
        original.info = opened.info.copy()
    return original


def nearest_upscale_image(original: Image.Image, output: Path) -> Image.Image:
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
    return upscaled


def exact_nearest_match_image(original: Image.Image, output: Path) -> bool:
    try:
        with Image.open(output) as output_image:
            expected = original.resize(
                (original.width * SCALE, original.height * SCALE),
                resample=Image.Resampling.NEAREST,
            ).convert("RGBA")
            actual = output_image.convert("RGBA")
            return actual.size == expected.size and actual.tobytes() == expected.tobytes()
    except (OSError, ValueError):
        return False


def static_region_shimmer(
    previous: tuple[Image.Image, Image.Image] | None,
    current: tuple[Image.Image, Image.Image],
) -> int:
    """Count output pixels that changed where the source pixel did not (must be 0).

    ``previous``/``current`` are (logical, upscaled) pairs of consecutive family frames.
    """
    if previous is None:
        return 0
    prev_src, prev_out = previous
    cur_src, cur_out = current
    if prev_src.size != cur_src.size or prev_out.size != cur_out.size:
        return -1
    src_same = [a == b for a, b in zip(prev_src.tobytes(), cur_src.tobytes())]
    prev_bytes = prev_out.convert("RGBA").tobytes()
    cur_bytes = cur_out.convert("RGBA").tobytes()
    width = cur_src.width
    out_width = cur_out.width
    shimmer = 0
    for y in range(cur_out.height):
        row_same = src_same[(y // SCALE) * width:(y // SCALE + 1) * width]
        row_start = y * out_width * 4
        for x in range(out_width):
            if not row_same[x // SCALE]:
                continue
            offset = row_start + x * 4
            if prev_bytes[offset:offset + 4] != cur_bytes[offset:offset + 4]:
                shimmer += 1
    return shimmer


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


def build_record(
    job: AssetJob,
    original: Image.Image,
    upscaled: Image.Image,
    shimmer: int | None = None,
) -> dict[str, Any]:
    module_path = job.module_source if job.module_source and job.module_source.is_file() else None
    advisor = job.resource_type == "TYPE302"
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
    if advisor:
        route["family_transform"] = ADVISOR_FAMILY_TRANSFORM
    source: dict[str, Any] = {
        "path": relative_path(job.source),
        "sha256": sha256_file(job.source),
        "module_path": relative_path(module_path) if module_path else None,
        "module_sha256": sha256_file(module_path) if module_path else None,
        "palette_sha256": palette_sha256(original),
        "width": original.width,
        "height": original.height,
        "mode": original.mode,
        "transparency": transparency_record(original),
    }
    gates: dict[str, Any] = {
        "exact_nearest_reconstruction": (
            exact_nearest_match_image(original, job.output)
            if advisor
            else exact_nearest_match(job.source, job.output)
        ),
        "human_review": "pending",
    }
    if advisor:
        source["anchor_path"] = relative_path(job.chain[0])
        source["anchor_sha256"] = sha256_file(job.chain[0])
        source["chain"] = [relative_path(path) for path in job.chain]
        source["chain_sha256"] = [sha256_file(path) for path in job.chain]
        gates.update(
            {
                "frame_index": job.frame_index,
                "frame_count": job.family_size,
                "frame_order": "authored-ascending",
                "cadence_seconds": ADVISOR_CADENCE_SECONDS,
                "static_region_shimmer": 0 if shimmer is None else shimmer,
                "family_atomic": True,
            }
        )
    return {
        "approved": False,
        "review": None,
        "asset_id": {
            "module": job.module,
            "resource_type": job.resource_type,
            "resource_id": job.resource_id,
            "locale": "en",
        },
        "source": source,
        "classification": {
            "family": job.family or "unclassified",
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
        "gates": gates,
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


def advisor_jobs(
    source_root: Path,
    output_root: Path,
    dll: str | None,
    module_root: Path | None = None,
) -> Iterable[AssetJob]:
    """One job per authored advisor frame, families kept whole (atomic).

    A family whose anchor or any authored frame is missing yields nothing and is reported,
    so partial families can never enter the manifest.
    """
    families = _type302().AUTHORED_FAMILIES
    for directory_name in sorted(families):
        if dll and directory_name != dll:
            continue
        directory = source_root / directory_name
        module = f"{directory_name.removesuffix('-dll').upper()}.DLL"
        for anchor_id, first, last in families[directory_name]:
            anchor = directory / "BMP" / f"{anchor_id}.bmp"
            frames = [directory / "TYPE302" / f"{rid}.bin" for rid in range(first, last + 1)]
            missing = [p for p in [anchor, *frames] if not p.is_file()]
            if missing:
                print(
                    f"FAMILY-INCOMPLETE {directory_name}/{anchor_id}: missing {len(missing)} file(s)",
                    file=sys.stderr,
                )
                continue
            family = f"{directory_name}/{anchor_id}"
            for index, (rid, frame) in enumerate(zip(range(first, last + 1), frames)):
                yield AssetJob(
                    key=f"{directory_name}/{rid}",
                    source=frame,
                    output=output_root / directory_name / f"{rid}.png",
                    module=module,
                    resource_id=rid,
                    module_source=(module_root / module) if module_root else None,
                    resource_type="TYPE302",
                    family=family,
                    chain=(anchor, *frames[: index + 1]),
                    frame_index=index,
                    family_size=len(frames),
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
    previous_frame: dict[str, tuple[Image.Image, Image.Image] | None] = {}
    for job in selected:
        existing = manifest["assets"].get(job.key)
        if matching_record(job, existing) and not force:
            skipped += 1
            if job.family:
                with Image.open(job.output) as existing_output:
                    existing_output.load()
                    previous_frame[job.family] = (load_logical(job), existing_output.copy())
            continue
        if job.output.exists() and not force:
            print(
                f"UNAPPROVED {job.key}: output exists without matching provenance; use --force",
                file=sys.stderr,
            )
            blocked += 1
            continue

        if job.resource_type == "TYPE302":
            original = load_logical(job)
            upscaled = nearest_upscale_image(original, job.output)
            shimmer = static_region_shimmer(previous_frame.get(job.family), (original, upscaled))
            previous_frame[job.family] = (original, upscaled)
            record = build_record(job, original, upscaled, shimmer)
            if shimmer != 0:
                raise RuntimeError(f"static-region shimmer {shimmer} for {job.key}")
        else:
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

    # Atomic family fallback: one failing advisor frame invalidates its whole family.
    families: dict[str, list[str]] = {}
    for key, record in manifest["assets"].items():
        if record.get("asset_id", {}).get("resource_type") == "TYPE302":
            families.setdefault(record.get("classification", {}).get("family", ""), []).append(key)
    for family, keys in sorted(families.items()):
        broken = sorted(key for key in keys if key in failed_keys)
        if not broken:
            continue
        for key in keys:
            if key not in failed_keys:
                failures.append(f"{key}: family {family} fails atomically ({broken[0]} failed)")
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
    resource_type = asset_id.get("resource_type")
    if (
        not isinstance(resource_id, int)
        or not isinstance(module, str)
        or resource_type not in RESOURCE_TYPES
        or asset_id.get("locale") != "en"
    ):
        return "asset identity is invalid"
    advisor = resource_type == "TYPE302"
    if advisor and module == "EDATA":
        return "advisor frames cannot come from EDATA"
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
    if advisor and route.get("family_transform") != ADVISOR_FAMILY_TRANSFORM:
        return "advisor route must record the deterministic family transform"
    recorded_config = route.get("config_sha256")
    config = {key: value for key, value in route.items() if key != "config_sha256"}
    if recorded_config != canonical_digest(config):
        return "route configuration hash mismatch"
    if advisor:
        chain_error = verify_advisor_chain(source_record, source)
        if chain_error is not None:
            return chain_error
    try:
        if advisor:
            source_image = decode_chain(PROJECT_ROOT / path for path in source_record["chain"])
        else:
            source_image = Image.open(source)
        with source_image, Image.open(output) as output_image:
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
            reconstructed = exact_nearest_match_image(source_image, output)
    except (OSError, ValueError):
        return "source or output image cannot be decoded"
    if output_record.get("format") != "png":
        return "output format metadata mismatch"
    if output_record.get("color_space") != "srgb":
        return "output color-space metadata mismatch"
    if output_record.get("alpha_mode") != "straight":
        return "output alpha-mode metadata mismatch"
    if not reconstructed:
        return "nearest reconstruction mismatch"
    if gates.get("exact_nearest_reconstruction") is not True:
        return "exact reconstruction gate is not recorded"
    if advisor:
        gate_error = verify_advisor_gates(gates, source_record)
        if gate_error is not None:
            return gate_error
    return None


def verify_advisor_chain(source_record: dict[str, Any], source: Path) -> str | None:
    chain = source_record.get("chain")
    digests = source_record.get("chain_sha256")
    if (
        not isinstance(chain, list)
        or not isinstance(digests, list)
        or len(chain) != len(digests)
        or len(chain) < 2
        or not all(isinstance(value, str) for value in chain + digests)
    ):
        return "advisor chain metadata is invalid"
    if PROJECT_ROOT / chain[0] != PROJECT_ROOT / source_record.get("anchor_path", ""):
        return "advisor chain does not start at its anchor"
    if (PROJECT_ROOT / chain[-1]).resolve() != source.resolve():
        return "advisor chain does not end at the source frame"
    if source_record.get("anchor_sha256") != digests[0]:
        return "advisor anchor hash mismatch"
    for path, digest in zip(chain, digests):
        file = PROJECT_ROOT / path
        if not file.is_file() or sha256_file(file) != digest:
            return f"advisor chain hash mismatch at {path}"
    return None


def verify_advisor_gates(gates: dict[str, Any], source_record: dict[str, Any]) -> str | None:
    frame_count = gates.get("frame_count")
    frame_index = gates.get("frame_index")
    chain_frames = len(source_record.get("chain", [])) - 1
    if not isinstance(frame_count, int) or not isinstance(frame_index, int):
        return "advisor frame gates are not recorded"
    if frame_index != chain_frames - 1 or frame_index < 0 or frame_index >= frame_count:
        return "advisor frame index does not match its chain"
    if gates.get("frame_order") != "authored-ascending":
        return "advisor frame order gate mismatch"
    if gates.get("cadence_seconds") != ADVISOR_CADENCE_SECONDS:
        return "advisor cadence gate mismatch"
    if gates.get("static_region_shimmer") != 0:
        return "advisor static-region shimmer gate is not zero"
    if gates.get("family_atomic") is not True:
        return "advisor family is not atomic"
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
    generate_parser.add_argument("--kind", choices=["ui", "edata", "advisor"], default="ui")
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
    elif args.kind == "advisor":
        source_root = (args.source or DEFAULT_UI_SOURCE).resolve()
        jobs = advisor_jobs(source_root, output_root, args.dll, args.module_root.resolve())
    else:
        if args.dll:
            raise ValueError("--dll applies only to --kind ui")
        source_root = (args.source or DEFAULT_EDATA_SOURCE).resolve()
        jobs = edata_jobs(source_root, output_root)
    return generate(jobs, manifest_path, args.force, args.limit)


if __name__ == "__main__":
    raise SystemExit(main())
