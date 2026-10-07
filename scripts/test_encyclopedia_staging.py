#!/usr/bin/env python3

from __future__ import annotations

import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent


class EncyclopediaProductionStagingTests(unittest.TestCase):
    def test_wasm_build_requires_encyclopedia_before_compilation(self) -> None:
        script = (ROOT / "scripts" / "build-wasm.sh").read_text(encoding="utf-8")

        source_gate = script.index("production browser build requires canonical Encyclopedia")
        compile_step = script.index("cargo build --manifest-path")
        self.assertLess(source_gate, compile_step)
        self.assertIn("--require-encyclopedia", script)
        self.assertIn('--encyclopedia-source "$ENCYCLOPEDIA_SOURCE"', script)
        self.assertIn('--edata "$EDATA_DIR"', script)
        self.assertNotIn("Encyclopedia will remain unavailable", script)

    def test_every_production_package_path_delegates_to_the_strict_wasm_build(self) -> None:
        package = (ROOT / "scripts" / "package-web.sh").read_text(encoding="utf-8")
        docker = (ROOT / "scripts" / "docker-build.sh").read_text(encoding="utf-8")

        self.assertIn('bash "$ROOT/scripts/build-wasm.sh"', package)
        self.assertIn("./scripts/build-wasm.sh", docker)


if __name__ == "__main__":
    unittest.main()
