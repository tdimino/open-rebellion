#!/usr/bin/env python3

from __future__ import annotations

import importlib.util
import hashlib
import json
import struct
import sys
import tempfile
import unittest
from pathlib import Path


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

            entries = PACKER.collect_entries(base, ui, edata_dir=edata)
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
                PACKER.collect_entries(base, ui, edata_dir=edata)

            (edata / "EDATA.042").write_bytes(self._indexed_bmp())
            (edata / "EDATA.bad").write_bytes(self._indexed_bmp())
            with self.assertRaisesRegex(ValueError, "invalid EData filename"):
                PACKER.collect_entries(base, ui, edata_dir=edata)

            (edata / "EDATA.bad").unlink()
            missing_palette = bytearray(self._indexed_bmp())
            struct.pack_into("<I", missing_palette, 10, 54)
            (edata / "EDATA.042").write_bytes(missing_palette)
            with self.assertRaisesRegex(ValueError, "EDATA.042"):
                PACKER.collect_entries(base, ui, edata_dir=edata)

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
