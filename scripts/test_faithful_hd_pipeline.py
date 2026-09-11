#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = ["pillow"]
# ///

import contextlib
import importlib.util
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path

from PIL import Image


SCRIPT = Path(__file__).with_name("faithful_hd_pipeline.py")
SPEC = importlib.util.spec_from_file_location("faithful_hd_pipeline", SCRIPT)
PIPELINE = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
sys.modules[SPEC.name] = PIPELINE
SPEC.loader.exec_module(PIPELINE)


def indexed_bmp(path: Path) -> None:
    image = Image.new("P", (2, 2))
    palette = [0, 0, 0, 255, 0, 0, 0, 0, 255] + [0] * (768 - 9)
    image.putpalette(palette)
    image.putdata([0, 1, 2, 1])
    image.save(path, format="BMP")


class FaithfulHdPipelineTests(unittest.TestCase):
    def test_verify_rejects_missing_manifest(self):
        with tempfile.TemporaryDirectory() as directory:
            with contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(
                io.StringIO()
            ):
                result = PIPELINE.verify(Path(directory) / "missing.json")
            self.assertEqual(result, 1)

    def test_nearest_upscale_preserves_exact_logical_pixels(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "source.bmp"
            output = root / "output.png"
            indexed_bmp(source)

            original, upscaled = PIPELINE.nearest_upscale(source, output)

            self.assertEqual(original.mode, "P")
            self.assertEqual(upscaled.mode, "P")
            self.assertEqual(upscaled.size, (8, 8))
            self.assertTrue(PIPELINE.exact_nearest_match(source, output))

    def test_generated_record_contains_identity_and_starts_unapproved(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "10001.bmp"
            output = root / "10001.png"
            indexed_bmp(source)
            original, upscaled = PIPELINE.nearest_upscale(source, output)
            job = PIPELINE.AssetJob(
                key="common-dll/10001",
                source=source,
                output=output,
                module="COMMON.DLL",
                resource_id=10001,
            )

            record = PIPELINE.build_record(job, original, upscaled)

            self.assertFalse(record["approved"])
            self.assertEqual(record["route"]["method"], "nearest-4x")
            self.assertIsNotNone(record["source"]["palette_sha256"])
            self.assertTrue(record["gates"]["exact_nearest_reconstruction"])
            self.assertEqual(record["output"]["sha256"], PIPELINE.sha256_file(output))

    def test_verify_rejects_a_modified_output(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "10001.bmp"
            output = root / "10001.png"
            manifest_path = root / "manifest.json"
            indexed_bmp(source)
            original, upscaled = PIPELINE.nearest_upscale(source, output)
            job = PIPELINE.AssetJob(
                key="common-dll/10001",
                source=source,
                output=output,
                module="COMMON.DLL",
                resource_id=10001,
            )
            manifest = PIPELINE.empty_manifest()
            manifest["assets"][job.key] = PIPELINE.build_record(job, original, upscaled)
            manifest_path.write_text(json.dumps(manifest), encoding="utf-8")
            output.write_bytes(b"changed")

            with contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(
                io.StringIO()
            ):
                result = PIPELINE.verify(manifest_path)
            self.assertEqual(result, 1)

    def test_verify_rejects_tampered_route_provenance(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "10001.bmp"
            output = root / "10001.png"
            manifest_path = root / "manifest.json"
            indexed_bmp(source)
            original, upscaled = PIPELINE.nearest_upscale(source, output)
            job = PIPELINE.AssetJob(
                key="common-dll/10001",
                source=source,
                output=output,
                module="COMMON.DLL",
                resource_id=10001,
            )
            manifest = PIPELINE.empty_manifest()
            record = PIPELINE.build_record(job, original, upscaled)
            record["route"]["scale"] = 8
            manifest["assets"][job.key] = record
            manifest_path.write_text(json.dumps(manifest), encoding="utf-8")

            with contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(
                io.StringIO()
            ):
                result = PIPELINE.verify(manifest_path)
            self.assertEqual(result, 1)

    def test_verify_rejects_asset_key_identity_mismatch(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "10001.bmp"
            output = root / "10001.png"
            manifest_path = root / "manifest.json"
            indexed_bmp(source)
            original, upscaled = PIPELINE.nearest_upscale(source, output)
            job = PIPELINE.AssetJob(
                key="common-dll/10001",
                source=source,
                output=output,
                module="COMMON.DLL",
                resource_id=10001,
            )
            manifest = PIPELINE.empty_manifest()
            record = PIPELINE.build_record(job, original, upscaled)
            record["asset_id"]["resource_id"] = 999
            manifest["assets"][job.key] = record
            manifest_path.write_text(json.dumps(manifest), encoding="utf-8")

            with contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(
                io.StringIO()
            ):
                result = PIPELINE.verify(manifest_path)
            self.assertEqual(result, 1)

    def test_generate_verify_and_review_approval_are_explicit(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source_root = root / "ui"
            source = source_root / "common-dll" / "BMP" / "10001.bmp"
            source.parent.mkdir(parents=True)
            indexed_bmp(source)
            output_root = root / "hd"
            manifest_path = output_root / "manifest.json"
            jobs = PIPELINE.ui_jobs(source_root, output_root, "common-dll")

            with contextlib.redirect_stdout(io.StringIO()):
                generated = PIPELINE.generate(jobs, manifest_path, False, None)
                verified = PIPELINE.verify(manifest_path)

            manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
            self.assertEqual(generated, 0)
            self.assertEqual(verified, 0)
            self.assertFalse(manifest["assets"]["common-dll/10001"]["approved"])

            with contextlib.redirect_stdout(io.StringIO()):
                approved = PIPELINE.approve(
                    manifest_path,
                    "common-dll/10001",
                    "test-reviewer",
                    "test-evidence",
                )
            manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
            self.assertEqual(approved, 0)
            self.assertTrue(manifest["assets"]["common-dll/10001"]["approved"])
            self.assertEqual(
                manifest["assets"]["common-dll/10001"]["review"]["evidence"],
                "test-evidence",
            )

    def test_approval_rejects_blank_review_metadata(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "10001.bmp"
            output = root / "10001.png"
            manifest_path = root / "manifest.json"
            indexed_bmp(source)
            original, upscaled = PIPELINE.nearest_upscale(source, output)
            job = PIPELINE.AssetJob(
                key="common-dll/10001",
                source=source,
                output=output,
                module="COMMON.DLL",
                resource_id=10001,
            )
            manifest = PIPELINE.empty_manifest()
            manifest["assets"][job.key] = PIPELINE.build_record(job, original, upscaled)
            manifest_path.write_text(json.dumps(manifest), encoding="utf-8")

            with contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(
                io.StringIO()
            ):
                result = PIPELINE.approve(manifest_path, job.key, " ", "test-evidence")
            self.assertEqual(result, 1)

    def test_verify_rejects_invalid_review_and_image_metadata(self):
        mutations = (
            ("review", "reviewer", None),
            ("review", "evidence", 123),
            ("source", "transparency", {"kind": "palette-index", "index": 1}),
            ("output", "format", "jpeg"),
            ("output", "color_space", "linear"),
            ("output", "alpha_mode", "premultiplied"),
        )
        for section, field, value in mutations:
            with (
                self.subTest(section=section, field=field),
                tempfile.TemporaryDirectory() as directory,
            ):
                root = Path(directory)
                source = root / "10001.bmp"
                output = root / "10001.png"
                manifest_path = root / "manifest.json"
                indexed_bmp(source)
                original, upscaled = PIPELINE.nearest_upscale(source, output)
                job = PIPELINE.AssetJob(
                    key="common-dll/10001",
                    source=source,
                    output=output,
                    module="COMMON.DLL",
                    resource_id=10001,
                )
                record = PIPELINE.build_record(job, original, upscaled)
                record["approved"] = True
                record["review"] = {
                    "reviewer": "test-reviewer",
                    "evidence": "test-evidence",
                }
                record["gates"]["human_review"] = "pass"
                record[section][field] = value
                manifest = PIPELINE.empty_manifest()
                manifest["assets"][job.key] = record
                manifest_path.write_text(json.dumps(manifest), encoding="utf-8")

                with (
                    contextlib.redirect_stdout(io.StringIO()),
                    contextlib.redirect_stderr(io.StringIO()),
                ):
                    result = PIPELINE.verify(manifest_path)
                self.assertEqual(result, 1)

    def test_generate_does_not_reuse_a_different_source_for_the_same_key(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            first_source = root / "first" / "common-dll" / "BMP" / "10001.bmp"
            second_source = root / "second" / "common-dll" / "BMP" / "10001.bmp"
            first_source.parent.mkdir(parents=True)
            second_source.parent.mkdir(parents=True)
            indexed_bmp(first_source)
            second_image = Image.new("RGB", (2, 2), "green")
            second_image.save(second_source, format="BMP")
            output_root = root / "hd"
            manifest_path = output_root / "manifest.json"

            with contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(
                io.StringIO()
            ):
                first_result = PIPELINE.generate(
                    PIPELINE.ui_jobs(root / "first", output_root, "common-dll"),
                    manifest_path,
                    False,
                    None,
                )
                second_result = PIPELINE.generate(
                    PIPELINE.ui_jobs(root / "second", output_root, "common-dll"),
                    manifest_path,
                    False,
                    None,
                )
                forced_result = PIPELINE.generate(
                    PIPELINE.ui_jobs(root / "second", output_root, "common-dll"),
                    manifest_path,
                    True,
                    None,
                )

            manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
            record = manifest["assets"]["common-dll/10001"]
            self.assertEqual(first_result, 0)
            self.assertEqual(second_result, 1)
            self.assertEqual(forced_result, 0)
            self.assertEqual(
                record["source"]["path"], PIPELINE.relative_path(second_source)
            )
            self.assertEqual(
                record["source"]["sha256"], PIPELINE.sha256_file(second_source)
            )


if __name__ == "__main__":
    unittest.main()
