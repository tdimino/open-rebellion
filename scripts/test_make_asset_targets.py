#!/usr/bin/env python3

from __future__ import annotations

import subprocess
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
GAME_SOURCE = "/owned/Star Wars - Rebellion"


def dry_run(target: str) -> str:
    completed = subprocess.run(
        ["make", "--no-print-directory", "-n", target, f"GAME_SOURCE={GAME_SOURCE}"],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    if completed.returncode != 0:
        raise AssertionError(
            f"make -n {target} failed\nstdout:\n{completed.stdout}\nstderr:\n{completed.stderr}"
        )
    return completed.stdout


class MakeAssetTargetTests(unittest.TestCase):
    def test_stage_assets_derives_owned_inputs_and_skips_verification(self) -> None:
        output = dry_run("stage-assets")

        self.assertIn(f'--source "{GAME_SOURCE}"', output)
        self.assertIn(f'--mdata "{GAME_SOURCE}/MDATA"', output)
        self.assertIn("--encyclopedia-only", output)
        self.assertEqual(output.count("--no-verify"), 2)
        self.assertNotIn(" --verify", output)

    def test_verify_assets_checks_generic_and_encyclopedia_outputs(self) -> None:
        output = dry_run("verify-assets")

        self.assertEqual(output.count("--verify"), 2)
        self.assertIn("--encyclopedia-only", output)
        self.assertNotIn("--source", output)

    def test_run_derives_edata_but_keeps_staged_runtime_root(self) -> None:
        output = dry_run("run")

        self.assertIn(f'REBELLION_EDATA_DIR="{GAME_SOURCE}/EData"', output)
        self.assertIn("cargo run -p rebellion-app -- data/base", output)

    def test_extract_assets_remains_a_stage_assets_alias(self) -> None:
        self.assertEqual(dry_run("extract-assets"), dry_run("stage-assets"))


if __name__ == "__main__":
    unittest.main()
