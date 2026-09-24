#!/usr/bin/env python3

from __future__ import annotations

import importlib.util
import hashlib
import json
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
            (sfx / "tactical_ship_destroyed.wav").write_bytes(b"cue")

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
                (PACKER.KIND_AUDIO, "sfx/tactical_ship_destroyed.wav"),
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
