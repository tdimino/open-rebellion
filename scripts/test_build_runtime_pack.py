#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import importlib.util
import json
import struct
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock


SCRIPT = Path(__file__).with_name("build-runtime-pack.py")
SPEC = importlib.util.spec_from_file_location("build_runtime_pack", SCRIPT)
assert SPEC is not None and SPEC.loader is not None
PACKER = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = PACKER
SPEC.loader.exec_module(PACKER)


class RuntimePackBuilderTests(unittest.TestCase):
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

    def _encyclopedia_source(self, root: Path) -> tuple[Path, Path]:
        source = root / "source.json"
        body = "Synthetic publication body."
        catalog = {
            "schema_version": 1,
            "language_id": 1033,
            "encoding": "windows-1252",
            "source_code_page": 0,
            "texts": {
                "5952": {
                    "body": body,
                    "body_sha256": hashlib.sha256(body.encode()).hexdigest(),
                }
            },
            "artwork": {"5952": "EDATA.014"},
        }
        catalog_bytes = json.dumps(
            catalog, sort_keys=True, separators=(",", ":")
        ).encode()
        source.write_bytes(catalog_bytes)
        manifest = {
            "schema_version": 1,
            "catalog_sha256": hashlib.sha256(catalog_bytes).hexdigest(),
            "source_files": {
                "encytext_sha256": "c" * 64,
                "encybmap_sha256": "d" * 64,
            },
            "counts": {"texts": 1, "artwork_mappings": 1},
        }
        manifest_path = root / "source.json.manifest.json"
        manifest_path.write_text(
            json.dumps(manifest, sort_keys=True, separators=(",", ":")),
            encoding="utf-8",
        )
        edata = root / "EData"
        edata.mkdir()
        (edata / "EDATA.014").write_bytes(self._indexed_bmp())
        return source, edata

    def _canonical_count_source(self, root: Path) -> tuple[Path, Path]:
        source, edata = self._encyclopedia_source(root)
        catalog = json.loads(source.read_text(encoding="utf-8"))
        catalog["texts"] = {
            str(resource_id): {
                "body": f"Synthetic publication body {resource_id}.",
                "body_sha256": hashlib.sha256(
                    f"Synthetic publication body {resource_id}.".encode()
                ).hexdigest(),
            }
            for resource_id in range(1, 349)
        }
        catalog["artwork"] = {
            str(resource_id): "EDATA.014" for resource_id in range(1, 192)
        }
        catalog_bytes = json.dumps(
            catalog, sort_keys=True, separators=(",", ":")
        ).encode()
        source.write_bytes(catalog_bytes)
        manifest = json.loads(
            Path(f"{source}.manifest.json").read_text(encoding="utf-8")
        )
        manifest["catalog_sha256"] = hashlib.sha256(catalog_bytes).hexdigest()
        manifest["source_files"] = {
            "encytext_sha256": PACKER.ENCYCLOPEDIA_EXPECTED_ENCYTEXT_SHA256,
            "encybmap_sha256": PACKER.ENCYCLOPEDIA_EXPECTED_ENCYBMAP_SHA256,
        }
        manifest["counts"] = {"texts": 348, "artwork_mappings": 191}
        Path(f"{source}.manifest.json").write_text(
            json.dumps(manifest, sort_keys=True, separators=(",", ":")),
            encoding="utf-8",
        )
        return source, edata

    def test_canonical_encyclopedia_namespace_is_complete_and_exact(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            base = root / "base"
            ui = root / "ui"
            base.mkdir()
            ui.mkdir()
            (base / "SYSTEMSD.DAT").write_bytes(b"systems")
            trailing_root = root / "trailing"
            trailing_root.mkdir()
            source, edata = self._encyclopedia_source(trailing_root)

            entries = PACKER.collect_entries(
                base, ui, edata_dir=edata, encyclopedia_source=source
            )
            self.assertEqual(
                [
                    (entry.kind, entry.key)
                    for entry in entries
                    if entry.key.startswith(PACKER.ENCYCLOPEDIA_NAMESPACE)
                ],
                [
                    (PACKER.KIND_GAME_DATA, "encyclopedia/assets/EDATA.014"),
                    (PACKER.KIND_GAME_DATA, "encyclopedia/catalog.json"),
                    (PACKER.KIND_GAME_DATA, "encyclopedia/manifest.json"),
                ],
            )

            (edata / "EDATA.014").unlink()
            with self.assertRaisesRegex(ValueError, "EDATA.014"):
                PACKER.collect_entries(
                    base, ui, edata_dir=edata, encyclopedia_source=source
                )

            (edata / "EDATA.014").write_bytes(self._indexed_bmp())
            manifest = Path(f"{source}.manifest.json")
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            payload["counts"]["artwork_mappings"] = 2
            manifest.write_text(json.dumps(payload), encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "manifest count"):
                PACKER.collect_entries(
                    base, ui, edata_dir=edata, encyclopedia_source=source
                )

            source, edata = self._encyclopedia_source(root)
            (edata / "EDATA.014").write_bytes(self._indexed_bmp() + b"trailing")
            with self.assertRaisesRegex(ValueError, "declared byte length"):
                PACKER.collect_entries(
                    base, ui, edata_dir=edata, encyclopedia_source=source
                )

    def test_required_encyclopedia_rejects_absent_partial_and_noncanonical_sources(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            base = root / "base"
            ui = root / "ui"
            base.mkdir()
            ui.mkdir()
            (base / "SYSTEMSD.DAT").write_bytes(b"systems")

            with self.assertRaisesRegex(ValueError, "required Encyclopedia"):
                PACKER.collect_entries(base, ui, require_encyclopedia=True)

            source, edata = self._encyclopedia_source(root)
            with self.assertRaisesRegex(ValueError, "348 texts and 191 artwork mappings"):
                PACKER.collect_entries(
                    base,
                    ui,
                    edata_dir=edata,
                    encyclopedia_source=source,
                    require_encyclopedia=True,
                )

            canonical_root = root / "canonical"
            canonical_root.mkdir()
            source, edata = self._canonical_count_source(canonical_root)
            with self.assertRaisesRegex(ValueError, "canonical catalog identity"):
                PACKER.collect_entries(
                    base,
                    ui,
                    edata_dir=edata,
                    encyclopedia_source=source,
                    require_encyclopedia=True,
                )

    def test_encyclopedia_artwork_limits_match_the_rust_session(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source, edata = self._encyclopedia_source(root)
            artwork = self._indexed_bmp()

            with mock.patch.object(
                PACKER, "ENCYCLOPEDIA_IMAGE_BYTES_LIMIT", len(artwork) - 1
            ):
                with self.assertRaisesRegex(ValueError, "byte limit"):
                    PACKER.collect_encyclopedia_entries(source, edata)

            catalog = json.loads(source.read_text(encoding="utf-8"))
            catalog["artwork"] = {
                str(resource_id): f"EDATA.{resource_id:03}"
                for resource_id in range(1, 5)
            }
            catalog_bytes = json.dumps(
                catalog, sort_keys=True, separators=(",", ":")
            ).encode()
            source.write_bytes(catalog_bytes)
            manifest_path = Path(f"{source}.manifest.json")
            manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
            manifest["catalog_sha256"] = hashlib.sha256(catalog_bytes).hexdigest()
            manifest["counts"]["artwork_mappings"] = 4
            manifest_path.write_text(
                json.dumps(manifest, sort_keys=True, separators=(",", ":")),
                encoding="utf-8",
            )
            for resource_id in range(1, 5):
                (edata / f"EDATA.{resource_id:03}").write_bytes(artwork)

            with mock.patch.object(
                PACKER, "ENCYCLOPEDIA_ARTWORK_BYTES_LIMIT", len(artwork) * 3
            ):
                with self.assertRaisesRegex(ValueError, "aggregate byte limit"):
                    PACKER.collect_encyclopedia_entries(source, edata)

    def test_validated_artwork_digest_detects_replacement_before_packaging(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source, edata = self._encyclopedia_source(root)
            artwork_path = edata / "EDATA.014"
            original_validator = PACKER.validate_edata_bitmap

            def validate_then_replace(path: Path) -> bytes:
                validated = original_validator(path)
                replacement = bytearray(validated)
                replacement[-1] ^= 0xFF
                path.write_bytes(replacement)
                return validated

            with mock.patch.object(
                PACKER,
                "validate_edata_bitmap",
                side_effect=validate_then_replace,
            ):
                entries = PACKER.collect_encyclopedia_entries(source, edata)

            artwork_entry = next(
                entry
                for entry in entries
                if entry.key == "encyclopedia/assets/EDATA.014"
            )
            with self.assertRaisesRegex(ValueError, "changed after validation"):
                PACKER.entry_bytes(artwork_entry)

    def test_atomic_pack_publication_retains_last_known_good_on_failure(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            payload = root / "payload"
            payload.write_bytes(b"first")
            output = root / "runtime.orpk"
            entries = [PACKER.Entry(PACKER.KIND_GAME_DATA, "one", payload)]
            PACKER.publish_pack(entries, output)
            previous = output.read_bytes()

            payload.write_bytes(b"second")
            guarded = [
                PACKER.Entry(
                    PACKER.KIND_GAME_DATA,
                    "one",
                    payload,
                    expected_sha256=hashlib.sha256(b"different").hexdigest(),
                )
            ]
            with self.assertRaisesRegex(ValueError, "changed after validation"):
                PACKER.publish_pack(guarded, output)

            self.assertEqual(output.read_bytes(), previous)
            self.assertEqual(list(root.glob(".runtime.orpk.*.tmp")), [])

    def test_loose_mirror_activates_one_validated_generation_atomically(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            base = root / "base"
            ui = root / "ui"
            base.mkdir()
            ui.mkdir()
            (base / "SYSTEMSD.DAT").write_bytes(b"systems")
            source, edata = self._encyclopedia_source(root)
            entries = PACKER.collect_entries(
                base, ui, edata_dir=edata, encyclopedia_source=source
            )
            mirror = root / "loose" / "encyclopedia"

            generation = PACKER.publish_loose_encyclopedia(entries, mirror)
            pointer = json.loads((mirror / "current.json").read_text(encoding="utf-8"))
            self.assertEqual(pointer["generation"], generation)
            self.assertTrue(pointer["available"])
            published = mirror / "generations" / generation
            self.assertEqual((published / "catalog.json").read_bytes(), source.read_bytes())
            self.assertTrue((published / "assets" / "EDATA.014").is_file())
            PACKER.verify_loose_encyclopedia(entries, mirror)

            self.assertEqual(
                PACKER.publish_loose_encyclopedia(entries, mirror), generation
            )
            self.assertEqual(len(list((mirror / "generations").iterdir())), 1)

            active_pointer = (mirror / "current.json").read_bytes()
            (edata / "EDATA.014").write_bytes(b"changed after validation")
            with self.assertRaisesRegex(ValueError, "changed after validation"):
                PACKER.publish_loose_encyclopedia(entries, mirror)
            self.assertEqual((mirror / "current.json").read_bytes(), active_pointer)

            self.assertIsNone(PACKER.publish_loose_encyclopedia([], mirror))
            PACKER.verify_loose_encyclopedia([], mirror)
            self.assertEqual(
                json.loads((mirror / "current.json").read_text(encoding="utf-8")),
                {"schema_version": 1, "available": False, "generation": None},
            )

            unsafe = root / "unsafe"
            unsafe.mkdir()
            outside = root / "outside"
            outside.mkdir()
            (unsafe / "generations").symlink_to(outside, target_is_directory=True)
            with self.assertRaisesRegex(ValueError, "unsafe.*generations"):
                PACKER.publish_loose_encyclopedia(entries, unsafe)

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

    def test_edata_artwork_is_validated_and_namespaced(self) -> None:
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

            entries = PACKER.collect_edata_entries(edata)
            edata_entries = [
                (entry.kind, entry.key) for entry in entries
                if entry.key.startswith(PACKER.ENCYCLOPEDIA_PREFIX)
            ]
            self.assertEqual(
                edata_entries,
                [
                    (PACKER.KIND_GAME_DATA, "encyclopedia/assets/EDATA.001"),
                    (PACKER.KIND_GAME_DATA, "encyclopedia/assets/EDATA.042"),
                ],
            )

            (edata / "EDATA.042").write_bytes(self._indexed_bmp(width=399))
            with self.assertRaisesRegex(ValueError, "EDATA.042"):
                PACKER.collect_edata_entries(edata)

            (edata / "EDATA.042").write_bytes(self._indexed_bmp())
            (edata / "EDATA.bad").write_bytes(self._indexed_bmp())
            with self.assertRaisesRegex(ValueError, "invalid EData filename"):
                PACKER.collect_edata_entries(edata)

            (edata / "EDATA.bad").unlink()
            missing_palette = bytearray(self._indexed_bmp())
            struct.pack_into("<I", missing_palette, 10, 54)
            (edata / "EDATA.042").write_bytes(missing_palette)
            with self.assertRaisesRegex(ValueError, "EDATA.042"):
                PACKER.collect_edata_entries(edata)

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


if __name__ == "__main__":
    unittest.main()
