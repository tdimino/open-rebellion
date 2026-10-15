#!/usr/bin/env python3
"""Shrink a Cargo target directory to what the last build used.

    scripts/prune-target.py [--units FILE] [--target DIR] [--keep-incremental N]
                            [--max-gb N] [--log FILE] [--force]
    cargo … --message-format=json-render-diagnostics | scripts/prune-target.py --tee-units FILE

scripts/mini-cargo.sh runs this on the Mac Mini after every build, and a
daily launchd agent runs it there as a backstop
(scripts/install-mini-prune-agent.sh). Cargo deletes nothing on its own, and
three things grow:

1. Units no build needs any more. Every change to features, `-p` selection
   or flags hashes a unit anew and leaves the old outputs. With --units
   (the build's `--message-format=json` messages), every entry of deps/,
   .fingerprint/, build/ and examples/ in a profile the build used is
   removed unless its 16-hex unit hash is among the build's units, fresh or
   rebuilt. Cargo reports fresh units, so a repeat build stays fresh. Access
   times are not used: APFS does not update them reliably, and
   `cargo sweep --file`, which relies on them, deleted fresh units here.
2. Stale codegen objects. With split-debuginfo "unpacked", Cargo's macOS dev
   default, rustc keeps each unit's *.rcgu.o for the debug map, and since
   rustc 1.88 writes them under a new name on every build and never deletes
   the old set (rust-lang/rust#161824). An object is kept only when an
   executable names it in its debug map (N_OSO).
3. Incremental directories, one per crate and build variant. rustc collects
   old sessions inside one, never a whole directory. Each crate keeps its
   N most recently used (default 2: a library and its tests). The deps
   objects are hard links into them, so this frees the most space.

Past --max-gb (default 12) after all that, `cargo clean` runs. Each run
appends one JSON line to --log. Without --force it does nothing while
another cargo or rustc runs.
"""

import argparse
import concurrent.futures
import filecmp
import json
import os
import re
import shutil
import socket
import subprocess
import sys
import time
from pathlib import Path

UNIT_HASH = re.compile(r"-([0-9a-f]{16})(?:\.|$)")
REPO = Path(__file__).resolve().parent.parent
os.environ["PATH"] = os.pathsep.join(
    [str(Path.home() / ".cargo/bin"), "/usr/bin", "/bin", "/usr/sbin", "/sbin", "/opt/homebrew/bin"]
)


def size_mb(path):
    out = subprocess.run(["du", "-sm", str(path)], capture_output=True, text=True).stdout
    return int(out.split()[0]) if out else 0


def remove(path):
    if path.is_dir() and not path.is_symlink():
        shutil.rmtree(path, ignore_errors=True)
    else:
        path.unlink(missing_ok=True)


def build_units(units_file, target):
    """The unit hashes a build used and the profile directories it built in."""
    hashes, profiles, uplifted = set(), set(), []
    for line in Path(units_file).read_text(errors="replace").splitlines():
        if not line.startswith('{"reason"'):
            continue
        message = json.loads(line)
        paths = list(message.get("filenames") or [])
        if message.get("executable"):
            paths.append(message["executable"])
        if message.get("out_dir"):
            paths.append(message["out_dir"])
        for name in paths:
            path = Path(name)
            unit = [m.group(1) for m in map(UNIT_HASH.search, path.parts) if m]
            hashes.update(unit)
            parents = [p for p in path.parents if p.name in ("deps", "build", "examples")]
            profile = parents[0].parent if parents else path.parent
            if target in profile.parents:
                profiles.add(profile)
                if not unit:
                    uplifted.append(path)
    # A binary is reported only by its uplifted copy (`debug/replay-gate`)
    # of `deps/replay_gate-<hash>`, a hard link or a byte copy: the deps file
    # with the same contents names the unit.
    for path in uplifted:
        if not path.is_file():
            continue
        stem = path.name.split(".")[0].replace("-", "_")
        for candidate in (path.parent / "deps").glob(stem + "-*"):
            found = UNIT_HASH.search(candidate.name)
            if (
                found
                and candidate.suffix == path.suffix
                and candidate.stat().st_size == path.stat().st_size
                and filecmp.cmp(candidate, path, shallow=False)
            ):
                hashes.add(found.group(1))
    return hashes, profiles


def prune_units(hashes, profiles):
    removed = 0
    for profile in profiles:
        for sub in ("deps", ".fingerprint", "build", "examples"):
            folder = profile / sub
            if not folder.is_dir():
                continue
            for entry in folder.iterdir():
                found = UNIT_HASH.search(entry.name)
                if found and found.group(1) not in hashes:
                    remove(entry)
                    removed += 1
    return removed


def debug_map_objects(binary):
    out = subprocess.run(["nm", "-ap", str(binary)], capture_output=True, text=True).stdout
    return {
        line.rsplit("/", 1)[-1]
        for line in out.splitlines()
        if " OSO " in line and line.endswith(".rcgu.o") and "(" not in line
    }


def prune_objects(target):
    removed = 0
    for deps in list(target.glob("*/deps")) + list(target.glob("*/*/deps")):
        objects = [p for p in deps.iterdir() if p.name.endswith(".rcgu.o")]
        if not objects:
            continue
        profile = deps.parent
        binaries = [
            p
            for p in list(deps.iterdir()) + list(profile.glob("build/*/build-script-*")) + list(profile.iterdir())
            if p.is_file() and not p.name.endswith((".d", ".rlib", ".rmeta", ".o")) and os.access(p, os.X_OK)
        ] + list(deps.glob("*.dylib"))
        live = set()
        with concurrent.futures.ThreadPoolExecutor(max_workers=os.cpu_count() or 4) as pool:
            for names in pool.map(debug_map_objects, set(binaries)):
                live |= names
        for path in objects:
            if path.name not in live:
                path.unlink(missing_ok=True)
                removed += 1
    return removed


def prune_incremental(target, keep):
    removed = 0
    for folder in list(target.glob("*/incremental")) + list(target.glob("*/*/incremental")):
        crates = {}
        for session in folder.iterdir():
            if session.is_dir():
                crates.setdefault(session.name.rsplit("-", 1)[0], []).append(session)
        for sessions in crates.values():
            sessions.sort(key=lambda p: p.stat().st_mtime, reverse=True)
            for session in sessions[keep:]:
                remove(session)
                removed += 1
    return removed


def busy():
    for name in ("cargo", "rustc"):
        if subprocess.run(["pgrep", "-x", name], capture_output=True).returncode == 0:
            return True
    return False


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--units")
    parser.add_argument("--target", default=str(REPO / "target"))
    parser.add_argument("--keep-incremental", type=int, default=2)
    parser.add_argument("--max-gb", type=int, default=int(os.environ.get("OPEN_REBELLION_TARGET_MAX_GB", "12")))
    parser.add_argument("--log")
    parser.add_argument("--force", action="store_true")
    parser.add_argument("--tee-units", help="copy stdin to stdout, Cargo's unit messages to this file")
    args = parser.parse_args()

    if args.tee_units:
        with open(args.tee_units, "w", buffering=1) as units:
            for line in sys.stdin:
                (units if line.startswith('{"reason"') else sys.stdout).write(line)
                sys.stdout.flush()
        return 0

    target = Path(args.target).resolve()
    if not target.is_dir():
        return 0
    record = {"time": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()), "host": socket.gethostname()}
    record["before_mb"] = size_mb(target)
    record.update(units_removed=0, objects_removed=0, incremental_removed=0, cleaned=False, skipped="")
    if not args.force and busy():
        record["skipped"] = "a cargo or rustc process is running"
    else:
        if args.units and Path(args.units).is_file():
            hashes, profiles = build_units(args.units, target)
            if hashes:
                record["units_removed"] = prune_units(hashes, profiles)
        record["objects_removed"] = prune_objects(target)
        record["incremental_removed"] = prune_incremental(target, args.keep_incremental)
        if size_mb(target) > args.max_gb * 1024:
            subprocess.run(["cargo", "clean", "-q", "--target-dir", str(target)], cwd=REPO)
            record["cleaned"] = True
    record["after_mb"] = size_mb(target)
    if args.log:
        with open(args.log, "a") as log:
            log.write(json.dumps(record) + "\n")
    print(
        "prune-target: {before_mb} MB -> {after_mb} MB; {units_removed} stale units, "
        "{objects_removed} stale objects, {incremental_removed} incremental dirs".format(**record)
        + ("; skipped: " + record["skipped"] if record["skipped"] else "")
        + ("; cleaned" if record["cleaned"] else ""),
        file=sys.stderr,
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
