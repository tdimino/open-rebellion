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

            entries = PACKER.collect_entries(base, root / "ui", root / "audio")
            self.assertIn(
                (PACKER.KIND_AUDIO, "music/main_theme.wav"),
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

    def test_tactical_proof_entries_are_selected_and_hash_verified(self) -> None:
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

            mesh_bytes = b"ORTMESH proof"
            texture_bytes = b"ORTINDEX proof"
            mesh_hash = hashlib.sha256(mesh_bytes).hexdigest()
            texture_hash = hashlib.sha256(texture_bytes).hexdigest()
            (objects / f"{mesh_hash}.mesh").write_bytes(mesh_bytes)
            (objects / f"{texture_hash}.texture").write_bytes(texture_bytes)
            (runtime / "manifest.json").write_text(
                json.dumps(
                    {
                        "schema_version": 1,
                        "meshes": [
                            {
                                "id": 2560,
                                "language": 1033,
                                "object_sha256": mesh_hash,
                                "object": f"objects/{mesh_hash}.mesh",
                                "texture_bindings": [
                                    {
                                        "resource_name": "sdesti52.bmp",
                                        "resource_language": 1033,
                                    }
                                ],
                            }
                        ],
                        "textures": [
                            {
                                "identifier_kind": "name",
                                "name": "SDESTI52.BMP",
                                "language": 1033,
                                "kind": "indexed_rle",
                                "palette_rule": "battle_active",
                                "object_sha256": texture_hash,
                                "object": f"objects/{texture_hash}.texture",
                            }
                        ],
                    }
                ),
                encoding="utf-8",
            )

            entries = PACKER.collect_entries(base, ui)
            keys = [(entry.kind, entry.key) for entry in entries]
            self.assertIn((PACKER.KIND_TACTICAL_MESH, "2560/1033"), keys)
            self.assertIn(
                (PACKER.KIND_TACTICAL_TEXTURE, "SDESTI52.BMP/1033"), keys
            )

            manifest_path = runtime / "manifest.json"
            manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
            manifest["meshes"][0]["texture_bindings"][0][
                "resource_language"
            ] = 9999
            manifest_path.write_text(json.dumps(manifest), encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "does not bind"):
                PACKER.collect_entries(base, ui)

            manifest["meshes"][0]["texture_bindings"][0][
                "resource_language"
            ] = 1033
            manifest_path.write_text(json.dumps(manifest), encoding="utf-8")
            entries = PACKER.collect_entries(base, ui)
            (objects / f"{mesh_hash}.mesh").write_bytes(b"changed after collection")
            with self.assertRaisesRegex(ValueError, "changed after validation"):
                PACKER.write_pack(entries, root / "changed.orpk")

            (objects / f"{mesh_hash}.mesh").write_bytes(b"tampered")
            with self.assertRaisesRegex(ValueError, "SHA-256"):
                PACKER.collect_entries(base, ui)


if __name__ == "__main__":
    unittest.main()
