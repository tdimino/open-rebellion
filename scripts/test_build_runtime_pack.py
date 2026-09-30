#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import importlib.util
import json
import os
import shutil
import struct
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock


SCRIPT = Path(__file__).with_name("build-runtime-pack.py")
REPO_ROOT = SCRIPT.parent.parent
VALID_BUNDLE = (
    REPO_ROOT
    / "tests"
    / "fixtures"
    / "encyclopedia"
    / "fixtures"
    / "bundles"
    / "valid"
)
SPEC = importlib.util.spec_from_file_location("build_runtime_pack", SCRIPT)
assert SPEC is not None and SPEC.loader is not None
PACKER = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = PACKER
SPEC.loader.exec_module(PACKER)


class RuntimePackBuilderTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls._verifier_directory = tempfile.TemporaryDirectory()
        cls.verifier = Path(cls._verifier_directory.name) / "stage-ui-assets"
        subprocess.run(
            ["go", "build", "-o", str(cls.verifier), "./tools/stage-ui-assets"],
            cwd=REPO_ROOT,
            check=True,
            timeout=PACKER.ENCYCLOPEDIA_VERIFIER_TIMEOUT_SECONDS,
        )
        cls._previous_verifier = os.environ.get("REBELLION_STAGE_UI_ASSETS")
        os.environ["REBELLION_STAGE_UI_ASSETS"] = str(cls.verifier)

    @classmethod
    def tearDownClass(cls) -> None:
        if cls._previous_verifier is None:
            os.environ.pop("REBELLION_STAGE_UI_ASSETS", None)
        else:
            os.environ["REBELLION_STAGE_UI_ASSETS"] = cls._previous_verifier
        cls._verifier_directory.cleanup()

    @staticmethod
    def _indexed_bmp(width: int = 400, height: int = 200) -> bytes:
        stride = (width + 3) & ~3
        data = bytearray(1078 + stride * height)
        data[:2] = b"BM"
        struct.pack_into("<I", data, 2, len(data))
        struct.pack_into("<I", data, 10, 1078)
        struct.pack_into("<IiiHHI", data, 14, 40, width, height, 1, 8, 0)
        struct.pack_into("<I", data, 34, stride * height)
        return bytes(data)

    def test_encyclopedia_chrome_requires_exact_source_ids_not_just_count(self) -> None:
        expected = (
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
        self.assertEqual(PACKER.REQUIRED_ENCYCLOPEDIA_CHROME, expected)
        self.assertEqual(len(set(expected)), 48)
        self.assertTrue({0x1842, 0x1843, 0x299D}.isdisjoint(expected))

        with tempfile.TemporaryDirectory() as directory:
            ui = Path(directory)
            bmp_dir = ui / "strategy-dll" / "BMP"
            bmp_dir.mkdir(parents=True)
            payload = self._indexed_bmp(width=1, height=1)
            for resource_id in expected:
                (bmp_dir / f"{resource_id}.bmp").write_bytes(payload)
            PACKER.validate_encyclopedia_chrome_resources(ui)

            missing = expected[-1]
            (bmp_dir / f"{missing}.bmp").unlink()
            (bmp_dir / "999999.bmp").write_bytes(payload)
            with self.assertRaisesRegex(
                ValueError,
                rf"STRATEGY\.DLL BMP resource {missing}.*strategy-dll/BMP/{missing}\.bmp",
            ):
                PACKER.validate_encyclopedia_chrome_resources(ui)

    def test_encyclopedia_chrome_and_unrelated_bitmap_reach_browser_pack_unchanged(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            base = root / "base"
            ui = root / "ui"
            bmp_dir = ui / "strategy-dll" / "BMP"
            base.mkdir()
            bmp_dir.mkdir(parents=True)
            payloads = {}
            for index, resource_id in enumerate(PACKER.REQUIRED_ENCYCLOPEDIA_CHROME):
                payload = self._indexed_bmp(width=1 + index % 3, height=1)
                payloads[f"strategy-dll/{resource_id}"] = payload
                (bmp_dir / f"{resource_id}.bmp").write_bytes(payload)
            payloads["strategy-dll/424242"] = b"unrelated bitmap bytes"
            (bmp_dir / "424242.bmp").write_bytes(payloads["strategy-dll/424242"])

            PACKER.validate_encyclopedia_chrome_resources(ui)
            entries = PACKER.collect_entries(base, ui)
            packed = root / "runtime.orpk"
            PACKER.write_pack(entries, packed)
            PACKER.verify_pack(packed, entries)

            bitmap_entries = {
                entry.key: PACKER.entry_bytes(entry)
                for entry in entries
                if entry.kind == PACKER.KIND_BITMAP
            }
            self.assertEqual(bitmap_entries, payloads)

    @staticmethod
    def _copy_valid_encyclopedia(root: Path) -> tuple[Path, Path]:
        encyclopedia = root / "encyclopedia"
        base = root / "base"
        encyclopedia.mkdir()
        base.mkdir()
        shutil.copy2(VALID_BUNDLE / "catalog.json", encyclopedia / "catalog.json")
        shutil.copy2(VALID_BUNDLE / "manifest.json", encyclopedia / "manifest.json")
        shutil.copytree(VALID_BUNDLE / "assets", encyclopedia / "assets")
        shutil.copy2(VALID_BUNDLE / "sources" / "SYNTHETIC.DAT", base / "SYNTHETIC.DAT")
        return encyclopedia, base

    @staticmethod
    def _pack_entries(path: Path) -> list[tuple[int, str, bytes]]:
        contents = path.read_bytes()
        magic, version, flags, count = PACKER.HEADER.unpack_from(contents)
        if (magic, version, flags) != (PACKER.MAGIC, PACKER.VERSION, 0):
            raise AssertionError("test helper observed an invalid pack header")
        cursor = PACKER.HEADER.size
        entries = []
        for _ in range(count):
            kind, key_len, data_len = PACKER.ENTRY_HEADER.unpack_from(contents, cursor)
            cursor += PACKER.ENTRY_HEADER.size
            key = contents[cursor : cursor + key_len].decode("utf-8")
            cursor += key_len
            data = contents[cursor : cursor + data_len]
            cursor += data_len
            entries.append((kind, key, data))
        if cursor != len(contents):
            raise AssertionError("test helper observed trailing pack bytes")
        return entries

    def test_options_packaging_rejects_each_missing_confirmation_bitmap(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            bmp_dir = root / "rebdlog-dll" / "BMP"
            bmp_dir.mkdir(parents=True)
            for resource, width, height in [(10623,412,176), (10624,57,28), (10625,57,28), (10626,57,28), (10627,57,28)]:
                stride = (width + 3) & ~3
                data = bytearray(1078 + stride * height)
                data[:2] = b"BM"
                struct.pack_into("<I", data, 10, 1078)
                struct.pack_into("<IiiHH", data, 14, 40, width, height, 1, 8)
                (bmp_dir / f"{resource}.bmp").write_bytes(data)
            PACKER.validate_options_resources(root)
            for bitmap in bmp_dir.glob("*.bmp"):
                data = bitmap.read_bytes()
                bitmap.unlink()
                with self.assertRaisesRegex(ValueError, bitmap.stem):
                    PACKER.validate_options_resources(root)
                bitmap.write_bytes(data)
            bitmap = bmp_dir / "10624.bmp"
            indexed = bitmap.read_bytes()
            converted = bytearray(54 + 172 * 28)
            converted[:54] = indexed[:54]
            struct.pack_into("<I", converted, 10, 54)
            struct.pack_into("<H", converted, 28, 24)
            bitmap.write_bytes(converted)
            with self.assertRaisesRegex(ValueError, "10624"):
                PACKER.validate_options_resources(root)
            bitmap.write_bytes(indexed)
            (bmp_dir / "10623.bmp").write_bytes(b"BMtruncated")
            with self.assertRaisesRegex(ValueError, "10623"):
                PACKER.validate_options_resources(root)

    def test_pack_is_independent_of_textstra_key_order(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            base = root / "base"
            bmp = root / "ui" / "strategy-dll" / "BMP"
            base.mkdir()
            bmp.mkdir(parents=True)
            (base / "SYSTEMSD.DAT").write_bytes(b"systems")
            (bmp / "900.bmp").write_bytes(b"bitmap")
            textstra = base / "textstra.json"

            textstra.write_text('{"2":"second","1":"first"}', encoding="utf-8")
            entries = PACKER.collect_entries(base, root / "ui")
            first = root / "first.orpk"
            PACKER.write_pack(entries, first)

            textstra.write_text('{"1":"first","2":"second"}', encoding="utf-8")
            entries = PACKER.collect_entries(base, root / "ui")
            second = root / "second.orpk"
            PACKER.write_pack(entries, second)

            self.assertEqual(first.read_bytes(), second.read_bytes())
            PACKER.verify_pack(second, entries)

    def test_optional_audio_uses_relative_runtime_keys(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            base = root / "base"
            bmp = root / "ui" / "common-dll" / "BMP"
            audio = root / "audio" / "music"
            base.mkdir()
            bmp.mkdir(parents=True)
            audio.mkdir(parents=True)
            (base / "SYSTEMSD.DAT").write_bytes(b"systems")
            (bmp / "20001.bmp").write_bytes(b"bitmap")
            (audio / "main_theme.wav").write_bytes(b"wave")
            (audio / "battle.wav").write_bytes(b"battle")
            sfx = root / "audio" / "sfx"
            sfx.mkdir()
            tactical_names = [
                *(
                    f"tactical_event_{event:02x}_{variant}.wav"
                    for event in range(0x0D, 0x14)
                    for variant in range(3)
                ),
                "tactical_event_14_0.wav",
            ]
            for name in tactical_names:
                (sfx / name).write_bytes(b"cue")

            entries = PACKER.collect_entries(base, root / "ui", root / "audio")
            self.assertIn(
                (PACKER.KIND_AUDIO, "music/main_theme.wav"),
                [(entry.kind, entry.key) for entry in entries],
            )
            self.assertIn(
                (PACKER.KIND_AUDIO, "music/battle.wav"),
                [(entry.kind, entry.key) for entry in entries],
            )
            self.assertIn(
                (PACKER.KIND_AUDIO, "sfx/tactical_event_0d_0.wav"),
                [(entry.kind, entry.key) for entry in entries],
            )
            self.assertIn(
                (PACKER.KIND_AUDIO, "sfx/tactical_event_14_0.wav"),
                [(entry.kind, entry.key) for entry in entries],
            )

    def test_type302_advisor_frames_use_typed_runtime_keys(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            base = root / "base"
            bmp = root / "ui" / "alsprite-dll" / "BMP"
            frames = root / "ui" / "alsprite-dll" / "TYPE302"
            base.mkdir()
            bmp.mkdir(parents=True)
            frames.mkdir(parents=True)
            (base / "SYSTEMSD.DAT").write_bytes(b"systems")
            (bmp / "2001.bmp").write_bytes(b"palette anchor")
            (frames / "2002.bin").write_bytes(b"sparse frame")

            entries = PACKER.collect_entries(base, root / "ui")
            self.assertIn(
                (PACKER.KIND_ADVISOR_FRAME, "alsprite-dll/2002"),
                [(entry.kind, entry.key) for entry in entries],
            )

    def test_raw_edata_without_a_canonical_manifest_is_not_packaged(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            base = root / "base"
            ui = root / "ui"
            edata = root / "EData"
            base.mkdir()
            ui.mkdir()
            edata.mkdir()
            (base / "SYSTEMSD.DAT").write_bytes(b"systems")
            (edata / "EDATA.042").write_bytes(self._indexed_bmp())
            (edata / "EDATA.001").write_bytes(self._indexed_bmp())

            entries = PACKER.collect_entries(
                base,
                ui,
                encyclopedia_dir=root / "missing-canonical-stage",
            )
            self.assertFalse(
                any(entry.key.startswith("encyclopedia/") for entry in entries)
            )

    def test_canonical_encyclopedia_uses_real_verifier_and_manifest_allowlist(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            encyclopedia, base = self._copy_valid_encyclopedia(root)

            entries = PACKER.collect_encyclopedia_entries(encyclopedia, base)

            self.assertEqual(
                [(entry.kind, entry.key) for entry in entries],
                [
                    (PACKER.KIND_GAME_DATA, "encyclopedia/assets/EDATA.001"),
                    (PACKER.KIND_GAME_DATA, "encyclopedia/assets/EDATA.002"),
                    (PACKER.KIND_GAME_DATA, "encyclopedia/assets/EDATA.003"),
                    (PACKER.KIND_GAME_DATA, "encyclopedia/catalog.json"),
                    (PACKER.KIND_GAME_DATA, "encyclopedia/manifest.json"),
                ],
            )
            self.assertTrue(all(entry.expected_sha256 is not None for entry in entries))

    def test_encyclopedia_collection_excludes_local_report_raw_and_unreferenced_art(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            encyclopedia, base = self._copy_valid_encyclopedia(root)
            (encyclopedia / "source-report.json").write_text("{}", encoding="utf-8")
            raw = encyclopedia / "raw" / "encytext" / "1033"
            raw.mkdir(parents=True)
            (raw / "60001.bin").write_bytes(b"local evidence")
            (encyclopedia / "assets" / "EDATA.192").write_bytes(self._indexed_bmp())

            with mock.patch.object(PACKER, "verify_encyclopedia_stage"):
                entries = PACKER.collect_encyclopedia_entries(encyclopedia, base)

            keys = {entry.key for entry in entries}
            self.assertNotIn("encyclopedia/source-report.json", keys)
            self.assertNotIn("encyclopedia/raw/encytext/1033/60001.bin", keys)
            self.assertNotIn("encyclopedia/assets/EDATA.192", keys)

    def test_encyclopedia_collection_rejects_dat_mismatch_and_missing_image(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            encyclopedia, base = self._copy_valid_encyclopedia(root)
            (base / "SYNTHETIC.DAT").write_bytes(b"different selected package DAT")
            with self.assertRaisesRegex(ValueError, "binding source.*SYNTHETIC.DAT"):
                PACKER.collect_encyclopedia_entries(encyclopedia, base)

            shutil.copy2(
                VALID_BUNDLE / "sources" / "SYNTHETIC.DAT",
                base / "SYNTHETIC.DAT",
            )
            (encyclopedia / "assets" / "EDATA.002").unlink()
            with self.assertRaisesRegex(ValueError, "verification failed"):
                PACKER.collect_encyclopedia_entries(encyclopedia, base)

    def test_encyclopedia_collection_rejects_unsafe_and_symlink_paths_it_consumes(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            encyclopedia, base = self._copy_valid_encyclopedia(root)
            manifest_path = encyclopedia / "manifest.json"
            manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
            manifest["files"]["assets/../escape.bmp"] = "0" * 64
            manifest_path.write_text(json.dumps(manifest), encoding="utf-8")
            with mock.patch.object(PACKER, "verify_encyclopedia_stage"):
                with self.assertRaisesRegex(ValueError, "unsafe encyclopedia path"):
                    PACKER.collect_encyclopedia_entries(encyclopedia, base)

            shutil.copy2(VALID_BUNDLE / "manifest.json", manifest_path)
            image = encyclopedia / "assets" / "EDATA.001"
            external = root / "external.bmp"
            external.write_bytes(image.read_bytes())
            image.unlink()
            image.symlink_to(external)
            with mock.patch.object(PACKER, "verify_encyclopedia_stage"):
                with self.assertRaisesRegex(ValueError, "symlink|escape"):
                    PACKER.collect_encyclopedia_entries(encyclopedia, base)

    def test_encyclopedia_manifest_limit_rejects_before_unbounded_json_read(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            encyclopedia, base = self._copy_valid_encyclopedia(root)
            with (encyclopedia / "manifest.json").open("wb") as manifest:
                manifest.truncate(32 * 1024 * 1024 + 1)

            with mock.patch.object(PACKER, "verify_encyclopedia_stage") as verifier:
                with self.assertRaisesRegex(ValueError, "resource limit.*manifest.json"):
                    PACKER.collect_encyclopedia_entries(encyclopedia, base)
            verifier.assert_not_called()

    def test_encyclopedia_verifier_is_time_bounded_and_reports_timeout(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            completed = subprocess.CompletedProcess([], 0, "", "")
            with mock.patch.object(
                PACKER.subprocess, "run", return_value=completed
            ) as run:
                PACKER.verify_encyclopedia_stage(root)
            self.assertEqual(
                run.call_args.kwargs["timeout"],
                PACKER.ENCYCLOPEDIA_VERIFIER_TIMEOUT_SECONDS,
            )

            timeout = subprocess.TimeoutExpired(["stage-ui-assets"], 1)
            with mock.patch.object(PACKER.subprocess, "run", side_effect=timeout):
                with self.assertRaisesRegex(ValueError, "timed out"):
                    PACKER.verify_encyclopedia_stage(root)

    def test_optional_absence_is_empty_but_required_absence_rejects(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            base = root / "base"
            ui = root / "ui"
            base.mkdir()
            ui.mkdir()
            (base / "SYSTEMSD.DAT").write_bytes(b"systems")
            missing = root / "encyclopedia"

            entries = PACKER.collect_entries(
                base, ui, encyclopedia_dir=missing, require_encyclopedia=False
            )
            self.assertFalse(any(entry.key.startswith("encyclopedia/") for entry in entries))
            with self.assertRaisesRegex(ValueError, "required encyclopedia"):
                PACKER.collect_entries(
                    base, ui, encyclopedia_dir=missing, require_encyclopedia=True
                )

    def test_present_broken_namespace_symlink_is_corrupt_not_optional_absence(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            base = root / "base"
            ui = root / "ui"
            base.mkdir()
            ui.mkdir()
            (base / "SYSTEMSD.DAT").write_bytes(b"systems")
            encyclopedia = root / "encyclopedia"
            encyclopedia.symlink_to(root / "missing-target", target_is_directory=True)

            with self.assertRaisesRegex(ValueError, "unsafe encyclopedia root"):
                PACKER.collect_entries(
                    base,
                    ui,
                    encyclopedia_dir=encyclopedia,
                    require_encyclopedia=False,
                )

    def test_encyclopedia_json_bytes_are_exact_and_pack_is_stable(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            encyclopedia, base = self._copy_valid_encyclopedia(root)
            ui = root / "ui"
            ui.mkdir()
            entries = PACKER.collect_entries(base, ui, encyclopedia_dir=encyclopedia)
            first = root / "first.orpk"
            second = root / "second.orpk"

            PACKER.write_pack(entries, first)
            PACKER.write_pack(entries, second)

            self.assertEqual(first.read_bytes(), second.read_bytes())
            packed = {key: data for _, key, data in self._pack_entries(first)}
            self.assertEqual(
                packed["encyclopedia/catalog.json"],
                (encyclopedia / "catalog.json").read_bytes(),
            )
            self.assertEqual(
                packed["encyclopedia/manifest.json"],
                (encyclopedia / "manifest.json").read_bytes(),
            )
            self.assertEqual(PACKER.VERSION, 3)
            self.assertEqual(
                (
                    PACKER.KIND_GAME_DATA,
                    PACKER.KIND_BITMAP,
                    PACKER.KIND_AUDIO,
                    PACKER.KIND_ADVISOR_FRAME,
                    PACKER.KIND_TACTICAL_MESH,
                    PACKER.KIND_TACTICAL_TEXTURE,
                ),
                (0, 1, 2, 3, 4, 5),
            )
            self.assertEqual(
                {entry.kind for entry in entries if entry.key.startswith("encyclopedia/")},
                {PACKER.KIND_GAME_DATA},
            )

    def test_source_mutation_uses_snapshot_and_write_failure_preserves_pack(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            encyclopedia, base = self._copy_valid_encyclopedia(root)
            ui = root / "ui"
            ui.mkdir()
            entries = PACKER.collect_entries(base, ui, encyclopedia_dir=encyclopedia)
            output = root / "runtime.orpk"
            PACKER.write_pack(entries, output)
            previous = output.read_bytes()

            original_entry_bytes = PACKER.entry_bytes
            calls = 0

            def mutate_during_write(entry: object) -> bytes:
                nonlocal calls
                calls += 1
                if calls == 2:
                    (encyclopedia / "catalog.json").write_bytes(
                        b"changed while the candidate was being written"
                    )
                return original_entry_bytes(entry)

            with mock.patch.object(
                PACKER, "entry_bytes", side_effect=mutate_during_write
            ):
                PACKER.write_pack(entries, output)
            self.assertEqual(output.read_bytes(), previous)
            packed = {key: data for _, key, data in self._pack_entries(output)}
            self.assertEqual(
                packed["encyclopedia/catalog.json"],
                (VALID_BUNDLE / "catalog.json").read_bytes(),
            )
            self.assertEqual(list(root.glob(".runtime.orpk.*.tmp")), [])

            shutil.copy2(VALID_BUNDLE / "catalog.json", encyclopedia / "catalog.json")
            entries = PACKER.collect_entries(base, ui, encyclopedia_dir=encyclopedia)
            calls = 0

            def fail_after_one_entry(entry: object) -> bytes:
                nonlocal calls
                calls += 1
                if calls == 2:
                    raise OSError("injected pack write failure")
                return original_entry_bytes(entry)

            with mock.patch.object(PACKER, "entry_bytes", side_effect=fail_after_one_entry):
                with self.assertRaisesRegex(OSError, "injected pack write failure"):
                    PACKER.write_pack(entries, output)
            self.assertEqual(output.read_bytes(), previous)
            self.assertEqual(list(root.glob(".runtime.orpk.*.tmp")), [])

    def test_entry_byte_limit_rejects_growth_before_pack_replacement(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "catalog.json"
            source.write_bytes(b"1234")
            entry = PACKER.Entry(
                PACKER.KIND_GAME_DATA,
                "encyclopedia/catalog.json",
                source,
                hashlib.sha256(b"1234").hexdigest(),
                root,
                max_bytes=4,
            )
            output = root / "runtime.orpk"
            PACKER.write_pack([entry], output)
            previous = output.read_bytes()
            with source.open("wb") as handle:
                handle.truncate(5)

            with self.assertRaisesRegex(ValueError, "resource limit.*catalog.json"):
                PACKER.write_pack([entry], output)
            self.assertEqual(output.read_bytes(), previous)
            self.assertEqual(list(root.glob(".runtime.orpk.*.tmp")), [])

    def test_selected_dat_mutation_publishes_retained_paired_snapshot(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            encyclopedia, base = self._copy_valid_encyclopedia(root)
            ui = root / "ui"
            ui.mkdir()
            entries = PACKER.collect_entries(base, ui, encyclopedia_dir=encyclopedia)
            output = root / "runtime.orpk"
            PACKER.write_pack(entries, output)
            previous = output.read_bytes()

            (base / "SYNTHETIC.DAT").write_bytes(b"mutated after source pairing")
            PACKER.write_pack(entries, output)
            self.assertEqual(output.read_bytes(), previous)
            packed = {key: data for _, key, data in self._pack_entries(output)}
            self.assertEqual(
                packed["SYNTHETIC.DAT"],
                (VALID_BUNDLE / "sources" / "SYNTHETIC.DAT").read_bytes(),
            )

    def test_semantic_verification_and_writer_use_the_same_catalog_snapshot(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            encyclopedia, base = self._copy_valid_encyclopedia(root)
            ui = root / "ui"
            ui.mkdir()
            output = root / "runtime.orpk"
            sentinel = root / "sentinel.dat"
            sentinel.write_bytes(b"previous complete pack")
            PACKER.write_pack(
                [PACKER.Entry(PACKER.KIND_GAME_DATA, "sentinel.dat", sentinel)],
                output,
            )
            previous = output.read_bytes()

            catalog_path = encyclopedia / "catalog.json"
            manifest_path = encyclopedia / "manifest.json"
            valid_catalog = catalog_path.read_bytes()
            valid_manifest = manifest_path.read_bytes()
            invalid_catalog = b"{}"
            manifest = json.loads(valid_manifest)
            invalid_digest = hashlib.sha256(invalid_catalog).hexdigest()
            manifest["catalog_sha256"] = invalid_digest
            manifest["files"]["catalog.json"] = invalid_digest
            invalid_manifest = json.dumps(
                manifest, sort_keys=True, separators=(",", ":")
            ).encode("utf-8")
            catalog_path.write_bytes(invalid_catalog)
            manifest_path.write_bytes(invalid_manifest)

            real_verify = PACKER.verify_encyclopedia_stage

            def verify_during_valid_source_window(verified_root: Path) -> None:
                verified_catalog = verified_root / "catalog.json"
                verified_manifest = verified_root / "manifest.json"
                invalid_verified_catalog = verified_catalog.read_bytes()
                invalid_verified_manifest = verified_manifest.read_bytes()
                verified_catalog.write_bytes(valid_catalog)
                verified_manifest.write_bytes(valid_manifest)
                try:
                    real_verify(verified_root)
                finally:
                    verified_catalog.write_bytes(invalid_verified_catalog)
                    verified_manifest.write_bytes(invalid_verified_manifest)

            with mock.patch.object(
                PACKER,
                "verify_encyclopedia_stage",
                side_effect=verify_during_valid_source_window,
            ):
                with self.assertRaisesRegex(
                    ValueError, "encyclopedia verification failed"
                ):
                    PACKER.collect_entries(
                        base, ui, encyclopedia_dir=encyclopedia
                    )

            self.assertEqual(output.read_bytes(), previous)
            self.assertEqual(list(root.glob(".runtime.orpk.*.tmp")), [])

    def test_dat_pairing_and_writer_use_the_same_selected_dat_snapshot(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            encyclopedia, base = self._copy_valid_encyclopedia(root)
            ui = root / "ui"
            ui.mkdir()
            output = root / "runtime.orpk"
            sentinel = root / "sentinel.dat"
            sentinel.write_bytes(b"previous complete pack")
            PACKER.write_pack(
                [PACKER.Entry(PACKER.KIND_GAME_DATA, "sentinel.dat", sentinel)],
                output,
            )
            previous = output.read_bytes()

            dat_path = base / "SYNTHETIC.DAT"
            valid_dat = dat_path.read_bytes()
            wrong_dat = b"different selected package DAT with stable bytes"
            dat_path.write_bytes(wrong_dat)
            real_verify_binding_sources = PACKER.verify_binding_sources

            def verify_during_valid_dat_window(
                manifest: dict, selected_base: Path, *args: object, **kwargs: object
            ) -> object:
                dat_path.write_bytes(valid_dat)
                try:
                    return real_verify_binding_sources(
                        manifest, selected_base, *args, **kwargs
                    )
                finally:
                    dat_path.write_bytes(wrong_dat)

            with mock.patch.object(
                PACKER,
                "verify_binding_sources",
                side_effect=verify_during_valid_dat_window,
            ):
                with self.assertRaisesRegex(
                    ValueError, "binding source mismatch.*SYNTHETIC.DAT"
                ):
                    PACKER.collect_entries(
                        base, ui, encyclopedia_dir=encyclopedia
                    )

            self.assertEqual(output.read_bytes(), previous)
            self.assertEqual(list(root.glob(".runtime.orpk.*.tmp")), [])

    def test_complete_tactical_runtime_is_packed_and_hash_verified(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            base = root / "base"
            ui = root / "ui"
            runtime = ui / "tactical-dll" / "TACTICAL3D" / "runtime"
            objects = runtime / "objects"
            base.mkdir()
            ui.mkdir(exist_ok=True)
            objects.mkdir(parents=True)
            (base / "SYSTEMSD.DAT").write_bytes(b"systems")

            mesh_payloads = {
                2560: b"ORTMESH close",
                2561: b"ORTMESH medium",
                2562: b"ORTMESH far",
            }
            texture_payloads = {
                "SDESTI52.BMP": b"ORTINDEX close",
                "SDESTI_M.BMP": b"ORTINDEX medium",
            }
            palette_payloads = {
                palette_id: b"ORTPAL00" + palette_id.to_bytes(4, "little")
                for palette_id in range(5531, 5558)
            }
            mesh_hashes = {
                mesh_id: hashlib.sha256(payload).hexdigest()
                for mesh_id, payload in mesh_payloads.items()
            }
            texture_hashes = {
                name: hashlib.sha256(payload).hexdigest()
                for name, payload in texture_payloads.items()
            }
            palette_hashes = {
                palette_id: hashlib.sha256(payload).hexdigest()
                for palette_id, payload in palette_payloads.items()
            }
            for mesh_id, payload in mesh_payloads.items():
                (objects / f"{mesh_hashes[mesh_id]}.mesh").write_bytes(payload)
            for name, payload in texture_payloads.items():
                (objects / f"{texture_hashes[name]}.texture").write_bytes(payload)
            for palette_id, payload in palette_payloads.items():
                (objects / f"{palette_hashes[palette_id]}.texture").write_bytes(payload)
            (runtime / "manifest.json").write_text(
                json.dumps(
                    {
                        "schema_version": 1,
                        "meshes": [
                            {
                                "id": mesh_id,
                                "language": 1033,
                                "object_sha256": mesh_hashes[mesh_id],
                                "object": f"objects/{mesh_hashes[mesh_id]}.mesh",
                                "texture_bindings": (
                                    []
                                    if mesh_id == 2562
                                    else [
                                        {
                                            "resource_name": (
                                                "sdesti52.bmp"
                                                if mesh_id == 2560
                                                else "sdesti_m.bmp"
                                            ),
                                            "resource_language": 1033,
                                        }
                                    ]
                                ),
                            }
                            for mesh_id in mesh_payloads
                        ],
                        "textures": [
                            {
                                "identifier_kind": "name",
                                "name": name,
                                "language": 1033,
                                "kind": "indexed_rle",
                                "palette_rule": "battle_active",
                                "object_sha256": texture_hashes[name],
                                "object": f"objects/{texture_hashes[name]}.texture",
                            }
                            for name in texture_payloads
                        ]
                        + [
                            {
                                "identifier_kind": "id",
                                "id": palette_id,
                                "language": 1033,
                                "kind": "palette_rgb24",
                                "object_sha256": palette_hashes[palette_id],
                                "object": f"objects/{palette_hashes[palette_id]}.texture",
                            }
                            for palette_id in palette_payloads
                        ],
                    }
                ),
                encoding="utf-8",
            )

            entries = PACKER.collect_entries(base, ui)
            keys = [(entry.kind, entry.key) for entry in entries]
            self.assertEqual(
                [key for kind, key in keys if kind == PACKER.KIND_TACTICAL_MESH],
                ["2560/1033", "2561/1033", "2562/1033"],
            )
            self.assertEqual(
                [key for kind, key in keys if kind == PACKER.KIND_TACTICAL_TEXTURE],
                [
                    *[f"{palette_id}/1033" for palette_id in range(5531, 5558)],
                    "SDESTI52.BMP/1033",
                    "SDESTI_M.BMP/1033",
                ],
            )

            manifest_path = runtime / "manifest.json"
            manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
            manifest["meshes"][0]["texture_bindings"][0][
                "resource_language"
            ] = 9999
            manifest_path.write_text(json.dumps(manifest), encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "missing named tactical texture"):
                PACKER.collect_entries(base, ui)

            manifest["meshes"][0]["texture_bindings"][0][
                "resource_language"
            ] = 1033
            manifest_path.write_text(json.dumps(manifest), encoding="utf-8")
            entries = PACKER.collect_entries(base, ui)
            (objects / f"{mesh_hashes[2560]}.mesh").write_bytes(
                b"changed after collection"
            )
            with self.assertRaisesRegex(ValueError, "changed after validation"):
                PACKER.write_pack(entries, root / "changed.orpk")

            (objects / f"{mesh_hashes[2560]}.mesh").write_bytes(b"tampered")
            with self.assertRaisesRegex(ValueError, "SHA-256"):
                PACKER.collect_entries(base, ui)


class EncyclopediaOwnedParityHarnessTests(unittest.TestCase):
    @staticmethod
    def _sha256(payload: bytes) -> str:
        return hashlib.sha256(payload).hexdigest()

    @staticmethod
    def _runtime_pack(entries: list[tuple[int, str, bytes]]) -> bytes:
        payload = bytearray(PACKER.HEADER.pack(PACKER.MAGIC, PACKER.VERSION, 0, len(entries)))
        for kind, key, contents in entries:
            encoded_key = key.encode("utf-8")
            payload.extend(PACKER.ENTRY_HEADER.pack(kind, len(encoded_key), len(contents)))
            payload.extend(encoded_key)
            payload.extend(contents)
        return bytes(payload)

    def test_encyclopedia_smoke_verifier_matches_pack_loose_and_dat_provenance(self) -> None:
        verifier = REPO_ROOT / "tools" / "interface-parity" / "encyclopedia-smoke.mjs"
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "source"
            site = root / "site"
            encyclopedia = source / "encyclopedia"
            assets = encyclopedia / "assets"
            base = source
            assets.mkdir(parents=True)
            (site / "data").mkdir(parents=True)

            dat_bytes = b"synthetic systems provenance"
            image_bytes = b"synthetic image bytes"
            title = "Synthetic title"
            body = "ab\u2019cd " + ("synthetic viewport line " * 80)
            catalog = {
                "schema_version": 1,
                "default_language": "1033",
                "topic_sort": {
                    "algorithm": "synthetic",
                    "representable_encoding": "synthetic",
                    "representable_fold": "synthetic",
                    "unrepresentable": "synthetic",
                    "tie_break": "registry_order",
                },
                "index": {
                    "command": "aggregate",
                    "labels": {"1033": "Synthetic index"},
                    "topic_ids": ["original:1"],
                    "source_ref": "synthetic",
                },
                "categories": [],
                "topics": {
                    "original:1": {
                        "localized": {
                            "1033": {
                                "title": title,
                                "body": body,
                                "image_id": "edata:1",
                            }
                        },
                        "source_ref": "synthetic",
                    }
                },
                "images": {
                    "edata:1": {
                        "path": "assets/EDATA.001",
                        "format": "bmp",
                        "byte_length": len(image_bytes),
                        "width": 1,
                        "height": 1,
                        "sha256": self._sha256(image_bytes),
                        "source_ref": "synthetic",
                    }
                },
                "bindings": [
                    {
                        "family": "system_locations",
                        "dat_id": 7,
                        "variant": "default",
                        "topic_id": "original:1",
                    }
                ],
            }
            catalog_bytes = json.dumps(catalog, separators=(",", ":")).encode()
            manifest = {
                "schema_version": 1,
                "source_profile": "synthetic-owned-parity-v1",
                "catalog_sha256": self._sha256(catalog_bytes),
                "files": {
                    "catalog.json": self._sha256(catalog_bytes),
                    "assets/EDATA.001": self._sha256(image_bytes),
                },
                "binding_sources": [
                    {"basename": "SYSTEMSD.DAT", "sha256": self._sha256(dat_bytes)}
                ],
                "source_records": {},
            }
            manifest_bytes = json.dumps(manifest, separators=(",", ":")).encode()
            pack_bytes = self._runtime_pack(
                [
                    (PACKER.KIND_GAME_DATA, "encyclopedia/catalog.json", catalog_bytes),
                    (PACKER.KIND_GAME_DATA, "encyclopedia/manifest.json", manifest_bytes),
                    (PACKER.KIND_GAME_DATA, "encyclopedia/assets/EDATA.001", image_bytes),
                ]
            )
            (encyclopedia / "catalog.json").write_bytes(catalog_bytes)
            (encyclopedia / "manifest.json").write_bytes(manifest_bytes)
            (assets / "EDATA.001").write_bytes(image_bytes)
            (base / "SYSTEMSD.DAT").write_bytes(dat_bytes)
            (site / "data" / "runtime.orpk").write_bytes(pack_bytes)

            scenario = {
                "schema_version": 1,
                "family": "encyclopedia-base-parity",
                "source_profile": "synthetic-owned-parity-v1",
                "artifacts": {
                    "pack_sha256": self._sha256(pack_bytes),
                    "pack_byte_length": len(pack_bytes),
                    "catalog_sha256": self._sha256(catalog_bytes),
                    "manifest_sha256": self._sha256(manifest_bytes),
                    "encyclopedia_namespace_entries": 3,
                },
                "fixture_codes": {"alliance": 298, "empire": 554},
                "owned_no_art_topic_count": 0,
                "body_viewport": {"x": 102, "y": 286, "width": 395, "height": 80},
                "visible_text_checks": [
                    {
                        "topic_id": "original:1",
                        "purpose": "long_body_scroll",
                        "input_sequence": ["PageDown", "PageDown", "PageDown"],
                        "expected_distinct_viewports": 4,
                    },
                    {
                        "topic_id": "original:1",
                        "purpose": "owned_non_ascii",
                        "input_sequence": [],
                        "codepoint": {
                            "scalar": "U+2019",
                            "char_index": 2,
                            "utf8_byte_offset": 2,
                            "utf8_hex": "e28099",
                            "context_char_start": 0,
                            "context_char_end_exclusive": 5,
                            "context_sha256": self._sha256("ab\u2019cd".encode()),
                        },
                        "expected_capture": "initial_body_viewport",
                    },
                ],
                "probes": [
                    {
                        "topic_id": "original:1",
                        "binding": {
                            "family": "system_locations",
                            "dat_id": 7,
                            "variant": "default",
                        },
                        "title_sha256": self._sha256(title.encode()),
                        "body_sha256": self._sha256(body.encode()),
                        "sort_index": 0,
                        "images": {
                            "alliance": {
                                "asset_id": "edata:1",
                                "sha256": self._sha256(image_bytes),
                            },
                            "empire": {
                                "asset_id": "edata:1",
                                "sha256": self._sha256(image_bytes),
                            },
                        },
                    }
                ],
            }
            scenario_path = root / "scenario.json"
            scenario_path.write_text(json.dumps(scenario), encoding="utf-8")
            result_path = root / "verification.json"

            completed = subprocess.run(
                [
                    "node",
                    str(verifier),
                    "--verify-artifacts-only",
                    "--scenario",
                    str(scenario_path),
                    "--source-root",
                    str(source),
                    "--site",
                    str(site),
                    "--output",
                    str(result_path),
                ],
                cwd=REPO_ROOT,
                capture_output=True,
                text=True,
                timeout=30,
            )
            self.assertEqual(completed.returncode, 0, completed.stderr)
            result = json.loads(result_path.read_text(encoding="utf-8"))
            self.assertEqual(result["status"], "pass")
            self.assertEqual(result["encyclopedia_namespace_entries"], 3)
            self.assertEqual(result["binding_sources_verified"], 1)
            self.assertEqual(result["probes_verified"], 1)
            self.assertEqual(result["visible_text_checks_verified"], 2)

    @unittest.skipUnless(
        os.environ.get("REBELLION_ENCYCLOPEDIA_TEST_SOURCE"),
        "owned encyclopedia parity requires REBELLION_ENCYCLOPEDIA_TEST_SOURCE",
    )
    def test_owned_encyclopedia_publication_matches_the_packed_browser_artifact(self) -> None:
        source = Path(os.environ["REBELLION_ENCYCLOPEDIA_TEST_SOURCE"])
        site_value = os.environ.get("REBELLION_ENCYCLOPEDIA_TEST_SITE")
        if site_value is None:
            self.fail("owned encyclopedia parity requires REBELLION_ENCYCLOPEDIA_TEST_SITE")
        site = Path(site_value)
        scenario = (
            REPO_ROOT
            / "tools"
            / "interface-parity"
            / "scenarios"
            / "encyclopedia-base.json"
        )
        verifier = REPO_ROOT / "tools" / "interface-parity" / "encyclopedia-smoke.mjs"
        with tempfile.TemporaryDirectory() as directory:
            result_path = Path(directory) / "verification.json"
            completed = subprocess.run(
                [
                    "node",
                    str(verifier),
                    "--verify-artifacts-only",
                    "--scenario",
                    str(scenario),
                    "--source-root",
                    str(source),
                    "--site",
                    str(site),
                    "--output",
                    str(result_path),
                ],
                cwd=REPO_ROOT,
                capture_output=True,
                text=True,
                timeout=30,
            )
            self.assertEqual(completed.returncode, 0, completed.stderr)
            result = json.loads(result_path.read_text(encoding="utf-8"))
            self.assertEqual(
                result["runtime_pack_sha256"],
                "e9ae76b50a43d811c01d8634f9099809ec6c144f9fbcd93f364b6879001fb57b",
            )
            self.assertEqual(result["encyclopedia_namespace_entries"], 188)
            self.assertEqual(result["binding_sources_verified"], 12)
            self.assertEqual(result["probes_verified"], 4)
            self.assertEqual(result["owned_no_art_topic_count"], 0)


if __name__ == "__main__":
    unittest.main()
