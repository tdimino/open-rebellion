#!/usr/bin/env python3

from __future__ import annotations

import os
import shutil
import stat
import subprocess
import tempfile
import textwrap
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent


class EncyclopediaProductionStagingTests(unittest.TestCase):
    def test_clean_docker_build_stages_required_encyclopedia_before_strict_wasm(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "scripts").mkdir()
            (root / "tools" / "stage-ui-assets").mkdir(parents=True)
            shutil.copy2(
                ROOT / "scripts" / "docker-build.sh",
                root / "scripts" / "docker-build.sh",
            )

            original = root / "owned"
            (original / "EData").mkdir(parents=True)
            (original / "GData").mkdir()
            (original / "ENCYTEXT.DLL").write_bytes(b"text")
            (original / "ENCYBMAP.DLL").write_bytes(b"map")
            (original / "GData" / "TEST.DAT").write_bytes(b"dat")

            fake_bin = root / "bin"
            fake_bin.mkdir()
            fake_go = fake_bin / "go"
            fake_go.write_text(
                textwrap.dedent(
                    """\
                    #!/usr/bin/env bash
                    set -euo pipefail
                    printf '%s\\n' "$*" >> data/go-args.log
                    output=""
                    encyclopedia=0
                    while [ "$#" -gt 0 ]; do
                        case "$1" in
                            --encyclopedia-only) encyclopedia=1 ;;
                            --encyclopedia-output) shift; output="$1" ;;
                        esac
                        shift
                    done
                    if [ "$encyclopedia" = 1 ]; then
                        mkdir -p "$(dirname "$output")"
                        printf '{}\\n' > "$output"
                        printf '{}\\n' > "$output.manifest.json"
                    fi
                    """
                ),
                encoding="utf-8",
            )
            fake_go.chmod(fake_go.stat().st_mode | stat.S_IXUSR)

            fake_wasm = root / "scripts" / "build-wasm.sh"
            fake_wasm.write_text(
                textwrap.dedent(
                    """\
                    #!/usr/bin/env bash
                    set -euo pipefail
                    test -f data/base/encyclopedia/source.json
                    test -f data/base/encyclopedia/source.json.manifest.json
                    test "$REBELLION_EDATA_DIR" = "$ORIGINAL_GAME_DIR/EData"
                    printf '%s\\n' "$REBELLION_EDATA_DIR" > data/strict-build-edata.log
                    """
                ),
                encoding="utf-8",
            )
            fake_wasm.chmod(fake_wasm.stat().st_mode | stat.S_IXUSR)

            environment = os.environ.copy()
            environment.update(
                {
                    "PATH": f"{fake_bin}:{environment['PATH']}",
                    "ORIGINAL_GAME_DIR": str(original),
                    "FORCE_REBUILD": "1",
                    "PREPARE_MODDING": "0",
                }
            )
            completed = subprocess.run(
                ["bash", "scripts/docker-build.sh"],
                cwd=root,
                env=environment,
                capture_output=True,
                text=True,
                check=False,
            )

            self.assertEqual(
                completed.returncode,
                0,
                msg=f"stdout:\n{completed.stdout}\nstderr:\n{completed.stderr}",
            )
            self.assertEqual(
                (root / "data" / "strict-build-edata.log").read_text(
                    encoding="utf-8"
                ).strip(),
                str(original / "EData"),
            )
            invocations = (root / "data" / "go-args.log").read_text(
                encoding="utf-8"
            )
            self.assertIn("--encyclopedia-only", invocations)
            self.assertIn(
                "--encyclopedia-output data/base/encyclopedia/source.json",
                invocations,
            )
            self.assertEqual(invocations.count("--force"), 2)

    def test_wasm_build_requires_encyclopedia_before_compilation(self) -> None:
        script = (ROOT / "scripts" / "build-wasm.sh").read_text(encoding="utf-8")

        source_gate = script.index("production browser build requires canonical Encyclopedia")
        audit_step = script.index("encyclopedia-source-audit")
        compile_step = script.index("cargo build --manifest-path")
        self.assertLess(source_gate, compile_step)
        self.assertLess(audit_step, compile_step)
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
