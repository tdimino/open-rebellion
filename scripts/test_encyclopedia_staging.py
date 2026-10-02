#!/usr/bin/env python3
"""Synthetic E18 canonical-stage and coordinated-publication tests."""

from __future__ import annotations

import contextlib
import fcntl
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


REPO_ROOT = Path(__file__).resolve().parent.parent
PACK_SCRIPT = REPO_ROOT / "scripts" / "build-runtime-pack.py"
VALID_BUNDLE = (
    REPO_ROOT
    / "tests"
    / "fixtures"
    / "encyclopedia"
    / "fixtures"
    / "bundles"
    / "valid"
)


def load_pack_module():
    spec = importlib.util.spec_from_file_location("e18_runtime_pack", PACK_SCRIPT)
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    assert spec.loader is not None
    spec.loader.exec_module(module)
    return module


pack = load_pack_module()


def retained_entry(key: str, data: bytes, root: Path):
    diagnostic = root / key.replace("/", "-")
    return pack.Entry(
        pack.KIND_GAME_DATA,
        key,
        diagnostic,
        hashlib.sha256(data).hexdigest(),
        max_bytes=len(data),
        retained_bytes=data,
    )


def encyclopedia_entries(root: Path, catalog: bytes = b'{"generation":1}'):
    manifest = b'{"schema_version":1}'
    image = b"BMsynthetic-image"
    return [
        retained_entry("encyclopedia/catalog.json", catalog, root),
        retained_entry("encyclopedia/manifest.json", manifest, root),
        retained_entry("encyclopedia/assets/EDATA.001", image, root),
        retained_entry("FLEET.DAT", b"synthetic-dat", root),
    ]


def read_pack_entries(path: Path):
    contents = path.read_bytes()
    magic, version, flags, count = pack.HEADER.unpack_from(contents)
    assert (magic, version, flags) == (pack.MAGIC, pack.VERSION, 0)
    cursor = pack.HEADER.size
    observed = {}
    for _ in range(count):
        kind, key_len, data_len = pack.ENTRY_HEADER.unpack_from(contents, cursor)
        cursor += pack.ENTRY_HEADER.size
        key = contents[cursor : cursor + key_len].decode("utf-8")
        cursor += key_len
        data = contents[cursor : cursor + data_len]
        cursor += data_len
        observed[(kind, key)] = data
    assert cursor == len(contents)
    return observed


def mirror_bytes(root: Path):
    if not root.exists():
        return None
    return {
        path.relative_to(root).as_posix(): path.read_bytes()
        for path in sorted(root.rglob("*"))
        if path.is_file()
    }


class SimulatedTermination(BaseException):
    pass


def filesystem_inventory(root: Path):
    inventory = {}
    for path in sorted(root.rglob("*")):
        relative = path.relative_to(root).as_posix()
        if path.is_symlink():
            inventory[relative] = ("symlink", os.readlink(path))
        elif path.is_dir():
            inventory[relative] = ("directory",)
        elif path.is_file():
            inventory[relative] = (
                "file",
                hashlib.sha256(path.read_bytes()).hexdigest(),
            )
        else:
            inventory[relative] = ("other", path.lstat().st_mode)
    return inventory


def interrupted_publication(root: Path, phase: str, seed_previous: bool = True):
    output = root / "runtime.orpk"
    mirror = root / "encyclopedia"
    old_entries = encyclopedia_entries(root, b'{"generation":1}')
    new_entries = encyclopedia_entries(root, b'{"generation":2}')
    if seed_previous:
        pack.publish_runtime_artifacts(old_entries, output, mirror)
    real_write_record = pack._write_publication_record
    interrupted = False

    def terminate_after_phase(path, record):
        nonlocal interrupted
        real_write_record(path, record)
        if record.get("phase") == phase and not interrupted:
            interrupted = True
            raise SimulatedTermination(phase)

    with mock.patch.object(
        pack, "_write_publication_record", side_effect=terminate_after_phase
    ):
        with mock.patch.object(
            pack,
            "_recover_runtime_publication",
            side_effect=SimulatedTermination(phase),
        ):
            with unittest.TestCase().assertRaises(SimulatedTermination):
                pack.publish_runtime_artifacts(new_entries, output, mirror)
    assert interrupted
    record_path = root / ".runtime.orpk.encyclopedia-publication.json"
    return output, mirror, old_entries, new_entries, record_path


class CoordinatedPublicationTests(unittest.TestCase):
    def test_every_recorded_phase_recovers_to_one_complete_generation(self):
        class SimulatedTermination(BaseException):
            pass

        for phase in sorted(pack.PUBLICATION_PHASES):
            with self.subTest(phase=phase), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                output = root / "runtime.orpk"
                mirror = root / "encyclopedia"
                old_entries = encyclopedia_entries(root, b'{"generation":1}')
                new_entries = encyclopedia_entries(root, b'{"generation":2}')
                pack.publish_runtime_artifacts(old_entries, output, mirror)
                real_write_record = pack._write_publication_record
                interrupted = False

                def terminate_after_phase(path, record):
                    nonlocal interrupted
                    real_write_record(path, record)
                    if record.get("phase") == phase and not interrupted:
                        interrupted = True
                        raise SimulatedTermination(phase)

                with mock.patch.object(
                    pack,
                    "_write_publication_record",
                    side_effect=terminate_after_phase,
                ):
                    recovery_interruption = (
                        mock.patch.object(
                            pack,
                            "_recover_runtime_publication",
                            side_effect=SimulatedTermination(phase),
                        )
                        if phase != "complete"
                        else contextlib.nullcontext()
                    )
                    with recovery_interruption:
                        with self.assertRaises(SimulatedTermination):
                            pack.publish_runtime_artifacts(
                                new_entries, output, mirror
                            )
                self.assertTrue(interrupted)

                expected = (
                    new_entries
                    if phase in {"committed", "complete"}
                    else old_entries
                )
                pack.publish_runtime_artifacts(expected, output, mirror)

                self.assertEqual(
                    pack._read_publication_record(
                        root / ".runtime.orpk.encyclopedia-publication.json"
                    )["phase"],
                    "complete",
                )
                packed = read_pack_entries(output)
                expected_catalog = next(
                    entry.retained_bytes
                    for entry in expected
                    if entry.key == "encyclopedia/catalog.json"
                )
                self.assertEqual(
                    packed[(pack.KIND_GAME_DATA, "encyclopedia/catalog.json")],
                    expected_catalog,
                )
                self.assertEqual(
                    mirror_bytes(mirror)["catalog.json"], expected_catalog
                )
                self.assertEqual(list(root.glob("*.tmp")), [])
                self.assertEqual(list(root.glob("*.backup")), [])

    def test_real_verifier_allowlist_is_identical_in_pack_and_mirror(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            encyclopedia = root / "canonical"
            base = root / "base"
            ui = root / "ui"
            shutil.copytree(VALID_BUNDLE, encyclopedia)
            base.mkdir()
            ui.mkdir()
            shutil.copy2(
                VALID_BUNDLE / "sources" / "SYNTHETIC.DAT",
                base / "SYNTHETIC.DAT",
            )
            entries = pack.collect_entries(
                base, ui, encyclopedia_dir=encyclopedia
            )
            output = root / "runtime.orpk"
            mirror = root / "encyclopedia"

            pack.publish_runtime_artifacts(entries, output, mirror)

            loose = mirror_bytes(mirror)
            manifest = json.loads((VALID_BUNDLE / "manifest.json").read_bytes())
            self.assertEqual(set(loose), {"manifest.json", *manifest["files"]})
            packed = read_pack_entries(output)
            for relative, data in loose.items():
                self.assertEqual(
                    packed[(pack.KIND_GAME_DATA, f"encyclopedia/{relative}")],
                    data,
                )

    def test_pack_and_mirror_publish_the_same_retained_generation(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            output = root / "runtime.orpk"
            mirror = root / "encyclopedia"
            entries = encyclopedia_entries(root)

            pack.publish_runtime_artifacts(entries, output, mirror)

            packed = read_pack_entries(output)
            loose = mirror_bytes(mirror)
            self.assertEqual(
                loose,
                {
                    "assets/EDATA.001": b"BMsynthetic-image",
                    "catalog.json": b'{"generation":1}',
                    "manifest.json": b'{"schema_version":1}',
                },
            )
            for relative, data in loose.items():
                self.assertEqual(
                    packed[(pack.KIND_GAME_DATA, f"encyclopedia/{relative}")],
                    data,
                )
            record = pack._read_publication_record(
                root / ".runtime.orpk.encyclopedia-publication.json"
            )
            self.assertEqual(record["phase"], "complete")
            self.assertEqual(
                record["owned_files"],
                {
                    relative: hashlib.sha256(data).hexdigest()
                    for relative, data in loose.items()
                },
            )

    def test_identical_pair_is_a_true_no_op(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            output = root / "runtime.orpk"
            mirror = root / "encyclopedia"
            entries = encyclopedia_entries(root)
            pack.publish_runtime_artifacts(entries, output, mirror)
            record_path = root / ".runtime.orpk.encyclopedia-publication.json"
            before = {
                "pack": output.stat().st_mtime_ns,
                "mirror": mirror.stat().st_mtime_ns,
                "record": record_path.stat().st_mtime_ns,
            }

            pack.publish_runtime_artifacts(entries, output, mirror)

            self.assertEqual(
                before,
                {
                    "pack": output.stat().st_mtime_ns,
                    "mirror": mirror.stat().st_mtime_ns,
                    "record": record_path.stat().st_mtime_ns,
                },
            )

    def test_failed_second_artifact_publish_restores_previous_complete_pair(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            output = root / "runtime.orpk"
            mirror = root / "encyclopedia"
            pack.publish_runtime_artifacts(
                encyclopedia_entries(root, b'{"generation":1}'), output, mirror
            )
            previous_pack = output.read_bytes()
            previous_mirror = mirror_bytes(mirror)
            real_replace = pack.os.replace
            injected = False

            def fail_pack_publish(source, destination):
                nonlocal injected
                source_path = Path(source)
                destination_path = Path(destination)
                if (
                    destination_path == output
                    and source_path != output
                    and not injected
                ):
                    injected = True
                    raise OSError("injected pack publication failure")
                return real_replace(source, destination)

            with mock.patch.object(pack.os, "replace", side_effect=fail_pack_publish):
                with self.assertRaisesRegex(OSError, "injected pack"):
                    pack.publish_runtime_artifacts(
                        encyclopedia_entries(root, b'{"generation":2}'),
                        output,
                        mirror,
                    )

            self.assertEqual(output.read_bytes(), previous_pack)
            self.assertEqual(mirror_bytes(mirror), previous_mirror)
            self.assertEqual(list(root.glob("*.tmp")), [])
            self.assertEqual(list(root.glob("*.backup")), [])

    def test_next_writer_recovers_a_recorded_interrupted_publication(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            output = root / "runtime.orpk"
            mirror = root / "encyclopedia"
            original_entries = encyclopedia_entries(root, b'{"generation":1}')
            pack.publish_runtime_artifacts(original_entries, output, mirror)
            previous_pack = output.read_bytes()
            previous_mirror = mirror_bytes(mirror)
            real_replace = pack.os.replace
            injected = False

            def fail_once(source, destination):
                nonlocal injected
                if Path(destination) == output and not injected:
                    injected = True
                    raise OSError("injected interruption")
                return real_replace(source, destination)

            with mock.patch.object(pack.os, "replace", side_effect=fail_once):
                with mock.patch.object(
                    pack,
                    "_recover_runtime_publication",
                    side_effect=RuntimeError("simulated process termination"),
                ):
                    with self.assertRaisesRegex(RuntimeError, "termination"):
                        pack.publish_runtime_artifacts(
                            encyclopedia_entries(root, b'{"generation":2}'),
                            output,
                            mirror,
                        )

            interrupted = pack._read_publication_record(
                root / ".runtime.orpk.encyclopedia-publication.json"
            )
            self.assertNotEqual(interrupted["phase"], "complete")

            pack.publish_runtime_artifacts(original_entries, output, mirror)

            self.assertEqual(output.read_bytes(), previous_pack)
            self.assertEqual(mirror_bytes(mirror), previous_mirror)
            self.assertEqual(
                pack._read_publication_record(
                    root / ".runtime.orpk.encyclopedia-publication.json"
                )["phase"],
                "complete",
            )
            self.assertEqual(list(root.glob("*.tmp")), [])
            self.assertEqual(list(root.glob("*.backup")), [])

    def test_every_recovery_mutation_is_restartable_before_and_after_operation(self):
        observed_recovery_operations = set()

        def run_recovery_with_trace(
            root, phase, fail_index=None, fail_after=False
        ):
            output, mirror, old_entries, new_entries, record_path = interrupted_publication(
                root, phase
            )
            paths = pack._publication_paths(output, mirror)
            record = pack._read_publication_record(record_path)
            events = []
            in_record_write = False
            real_replace = pack.os.replace
            real_unlink = Path.unlink
            real_rmdir = Path.rmdir
            real_write_record = pack._write_publication_record

            def operation(label, action, *args, **kwargs):
                index = len(events)
                events.append(label)
                if fail_index == index and not fail_after:
                    raise SimulatedTermination(f"before {label}")
                result = action(*args, **kwargs)
                if fail_index == index and fail_after:
                    raise SimulatedTermination(f"after {label}")
                return result

            def traced_replace(source, destination):
                if in_record_write:
                    return real_replace(source, destination)
                return operation(
                    f"replace:{Path(source).name}->{Path(destination).name}",
                    real_replace,
                    source,
                    destination,
                )

            def traced_unlink(path, *args, **kwargs):
                if in_record_write:
                    return real_unlink(path, *args, **kwargs)
                return operation(
                    f"unlink:{Path(path).name}",
                    real_unlink,
                    path,
                    *args,
                    **kwargs,
                )

            def traced_rmdir(path, *args, **kwargs):
                return operation(
                    f"rmdir:{Path(path).name}",
                    real_rmdir,
                    path,
                    *args,
                    **kwargs,
                )

            def traced_write_record(path, value):
                nonlocal in_record_write
                def write():
                    nonlocal in_record_write
                    in_record_write = True
                    try:
                        return real_write_record(path, value)
                    finally:
                        in_record_write = False

                return operation(
                    f"journal:{value.get('phase')}:{value.get('recovery_progress')}",
                    write,
                )

            with mock.patch.object(pack.os, "replace", side_effect=traced_replace):
                with mock.patch.object(Path, "unlink", new=traced_unlink):
                    with mock.patch.object(Path, "rmdir", new=traced_rmdir):
                        with mock.patch.object(
                            pack,
                            "_write_publication_record",
                            side_effect=traced_write_record,
                        ):
                            pack._recover_runtime_publication(
                                record, record_path, output, mirror, paths
                            )
            expected_entries = new_entries if phase == "committed" else old_entries
            return events, output, mirror, expected_entries

        for phase, expected_catalog in (
            ("prepared", b'{"generation":1}'),
            ("mirror_published", b'{"generation":1}'),
            ("pack_published", b'{"generation":1}'),
            ("committed", b'{"generation":2}'),
        ):
            with tempfile.TemporaryDirectory() as baseline_directory:
                baseline_events, _, _, _ = run_recovery_with_trace(
                    Path(baseline_directory), phase
                )
            self.assertGreaterEqual(len(baseline_events), 4)
            observed_recovery_operations.update(
                event.split(":", 1)[0] for event in baseline_events
            )

            for event_index, event in enumerate(baseline_events):
                for fail_after in (False, True):
                    with self.subTest(
                        phase=phase, event=event, fail_after=fail_after
                    ):
                        with tempfile.TemporaryDirectory() as directory:
                            root = Path(directory)
                            with self.assertRaises(SimulatedTermination):
                                run_recovery_with_trace(
                                    root,
                                    phase,
                                    fail_index=event_index,
                                    fail_after=fail_after,
                                )
                            output = root / "runtime.orpk"
                            mirror = root / "encyclopedia"
                            expected_entries = encyclopedia_entries(
                                root, expected_catalog
                            )
                            for _ in range(3):
                                pack.publish_runtime_artifacts(
                                    expected_entries, output, mirror
                                )
                            self.assertEqual(
                                read_pack_entries(output)[
                                    (
                                        pack.KIND_GAME_DATA,
                                        "encyclopedia/catalog.json",
                                    )
                                ],
                                expected_catalog,
                            )
                            self.assertEqual(
                                mirror_bytes(mirror)["catalog.json"],
                                expected_catalog,
                            )
                            self.assertEqual(
                                pack._read_publication_record(
                                    root
                                    / ".runtime.orpk.encyclopedia-publication.json"
                                )["phase"],
                                "complete",
                            )
        self.assertEqual(
            observed_recovery_operations,
            {"replace", "unlink", "rmdir", "journal"},
        )

    def test_first_publication_record_removal_is_restartable(self):
        for fail_after in (False, True):
            with self.subTest(fail_after=fail_after):
                with tempfile.TemporaryDirectory() as directory:
                    root = Path(directory)
                    output, mirror, _, _, record_path = interrupted_publication(
                        root, "mirror_published", seed_previous=False
                    )
                    paths = pack._publication_paths(output, mirror)
                    record = pack._read_publication_record(record_path)
                    real_unlink = Path.unlink
                    interrupted = False

                    def interrupt_record_removal(path, *args, **kwargs):
                        nonlocal interrupted
                        if Path(path) != record_path or interrupted:
                            return real_unlink(path, *args, **kwargs)
                        interrupted = True
                        if not fail_after:
                            raise SimulatedTermination("before record removal")
                        result = real_unlink(path, *args, **kwargs)
                        raise SimulatedTermination("after record removal")

                    with mock.patch.object(
                        Path, "unlink", new=interrupt_record_removal
                    ):
                        with self.assertRaises(SimulatedTermination):
                            pack._recover_runtime_publication(
                                record, record_path, output, mirror, paths
                            )
                    self.assertTrue(interrupted)
                    for _ in range(3):
                        remaining = pack._read_publication_record(record_path)
                        if remaining is None:
                            break
                        pack._recover_runtime_publication(
                            remaining, record_path, output, mirror, paths
                        )

                    self.assertFalse(output.exists())
                    self.assertFalse(mirror.exists())
                    self.assertFalse(record_path.exists())
                    self.assertEqual(list(root.glob("*.backup")), [])
                    self.assertEqual(list(root.glob("*.tmp")), [])

    def test_recovery_validates_every_artifact_before_first_mutation(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            output, mirror, old_entries, _, record_path = interrupted_publication(
                root, "mirror_published"
            )
            mirror_backup = root / ".encyclopedia.encyclopedia.backup"
            user_file = mirror_backup / "user-notes.txt"
            user_file.write_text("mine", encoding="utf-8")
            before = filesystem_inventory(root)

            with self.assertRaisesRegex(ValueError, "owned|inventory|unrecognized"):
                pack.publish_runtime_artifacts(old_entries, output, mirror)

            self.assertEqual(filesystem_inventory(root), before)
            self.assertNotEqual(
                pack._read_publication_record(record_path)["phase"], "complete"
            )

        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            output, mirror, old_entries, _, record_path = interrupted_publication(
                root, "mirror_published"
            )
            record = pack._read_publication_record(record_path)
            record["pack_candidate_identity"]["sha256"] = "malformed"
            pack._write_publication_record(record_path, record)
            before = filesystem_inventory(root)

            with self.assertRaisesRegex(ValueError, "identity|record"):
                pack.publish_runtime_artifacts(old_entries, output, mirror)

            self.assertEqual(filesystem_inventory(root), before)

    def test_committed_recovery_refuses_unowned_cleanup_artifacts(self):
        for injected_kind in (
            "backup-file",
            "backup-directory",
            "forged-candidate",
        ):
            with self.subTest(injected_kind=injected_kind):
                with tempfile.TemporaryDirectory() as directory:
                    root = Path(directory)
                    output, mirror, _, new_entries, record_path = interrupted_publication(
                        root, "committed"
                    )
                    if injected_kind in {"backup-file", "backup-directory"}:
                        injected = (
                            root
                            / ".encyclopedia.encyclopedia.backup"
                            / (
                                "user-notes.txt"
                                if injected_kind == "backup-file"
                                else "user-directory"
                            )
                        )
                        if injected_kind == "backup-file":
                            injected.write_text("mine", encoding="utf-8")
                        else:
                            injected.mkdir()
                    else:
                        injected = root / ".encyclopedia.forged.tmp"
                        injected.mkdir()
                        (injected / "user-notes.txt").write_text(
                            "mine", encoding="utf-8"
                        )
                        record = pack._read_publication_record(record_path)
                        record["mirror_candidate"] = injected.name
                        pack._write_publication_record(record_path, record)
                    before = filesystem_inventory(root)

                    with self.assertRaisesRegex(
                        ValueError, "owned|identity|inventory|unrecognized"
                    ):
                        pack.publish_runtime_artifacts(new_entries, output, mirror)

                    self.assertEqual(filesystem_inventory(root), before)
                    self.assertTrue(injected.exists())

    def test_lock_rejects_symlink_and_nonregular_objects_without_writes(self):
        for lock_kind in ("symlink", "directory"):
            with self.subTest(lock_kind=lock_kind):
                with tempfile.TemporaryDirectory() as directory:
                    root = Path(directory)
                    output = root / "runtime.orpk"
                    mirror = root / "encyclopedia"
                    lock_path = root / ".runtime.orpk.encyclopedia.lock"
                    if lock_kind == "symlink":
                        external = root / "external.lock"
                        external.write_text("user lock", encoding="utf-8")
                        lock_path.symlink_to(external)
                    else:
                        lock_path.mkdir()
                    before = filesystem_inventory(root)

                    with self.assertRaisesRegex(ValueError, "unsafe.*lock"):
                        pack.publish_runtime_artifacts(
                            encyclopedia_entries(root), output, mirror
                        )

                    self.assertEqual(filesystem_inventory(root), before)

    def test_publication_rejects_target_control_and_input_collisions_before_writes(self):
        collision_cases = (
            ("same-target", "runtime.orpk", "runtime.orpk"),
            (
                "mirror-is-pack-backup",
                "runtime.orpk",
                ".runtime.orpk.encyclopedia.backup",
            ),
            (
                "mirror-is-record",
                "runtime.orpk",
                ".runtime.orpk.encyclopedia-publication.json",
            ),
            (
                "mirror-is-lock",
                "runtime.orpk",
                ".runtime.orpk.encyclopedia.lock",
            ),
        )
        for label, output_name, mirror_name in collision_cases:
            with self.subTest(label=label):
                with tempfile.TemporaryDirectory() as directory:
                    root = Path(directory)
                    before = filesystem_inventory(root)
                    with self.assertRaisesRegex(ValueError, "collid|distinct"):
                        pack.publish_runtime_artifacts(
                            encyclopedia_entries(root),
                            root / output_name,
                            root / mirror_name,
                        )
                    self.assertEqual(filesystem_inventory(root), before)

        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            lock_path = root / ".runtime.orpk.encyclopedia.lock"
            record_path = root / ".runtime.orpk.encyclopedia-publication.json"
            lock_path.write_bytes(b"")
            os.link(lock_path, record_path)
            before = filesystem_inventory(root)

            with self.assertRaisesRegex(ValueError, "collid|distinct|alias"):
                pack.publish_runtime_artifacts(
                    encyclopedia_entries(root),
                    root / "runtime.orpk",
                    root / "encyclopedia",
                )

            self.assertEqual(filesystem_inventory(root), before)

        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            output = root / "runtime.orpk"
            output.write_bytes(b"selected package input")
            source_entry = pack.Entry(
                pack.KIND_GAME_DATA,
                "FLEET.DAT",
                output,
                hashlib.sha256(output.read_bytes()).hexdigest(),
                max_bytes=len(output.read_bytes()),
            )
            entries = encyclopedia_entries(root) + [source_entry]
            before = filesystem_inventory(root)

            with self.assertRaisesRegex(ValueError, "input.*collid"):
                pack.publish_runtime_artifacts(
                    entries, output, root / "encyclopedia"
                )

            self.assertEqual(filesystem_inventory(root), before)

        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            output = root / "runtime.orpk"
            output.write_bytes(b"selected package input")
            source_alias = root / "selected-package-alias.dat"
            os.link(output, source_alias)
            source_entry = pack.Entry(
                pack.KIND_GAME_DATA,
                "FLEET.DAT",
                source_alias,
                hashlib.sha256(source_alias.read_bytes()).hexdigest(),
                max_bytes=len(source_alias.read_bytes()),
            )
            entries = encyclopedia_entries(root) + [source_entry]
            before = filesystem_inventory(root)

            with self.assertRaisesRegex(ValueError, "input.*collid"):
                pack.publish_runtime_artifacts(
                    entries, output, root / "encyclopedia"
                )

            self.assertEqual(filesystem_inventory(root), before)

    def test_unknown_mirror_files_and_directories_are_never_deleted(self):
        for unknown_kind in ("file", "directory"):
            with self.subTest(unknown_kind=unknown_kind):
                with tempfile.TemporaryDirectory() as directory:
                    root = Path(directory)
                    output = root / "runtime.orpk"
                    mirror = root / "encyclopedia"
                    pack.publish_runtime_artifacts(
                        encyclopedia_entries(root), output, mirror
                    )
                    previous_pack = output.read_bytes()
                    unknown = mirror / (
                        "notes.txt" if unknown_kind == "file" else "user-directory"
                    )
                    if unknown_kind == "file":
                        unknown.write_text("mine", encoding="utf-8")
                    else:
                        unknown.mkdir()

                    with self.assertRaisesRegex(
                        ValueError, "not owned|changed outside|directory"
                    ):
                        pack.publish_runtime_artifacts(
                            encyclopedia_entries(root, b'{"generation":2}'),
                            output,
                            mirror,
                        )

                    self.assertEqual(output.read_bytes(), previous_pack)
                    self.assertTrue(unknown.exists())
                    if unknown_kind == "file":
                        self.assertEqual(
                            unknown.read_text(encoding="utf-8"), "mine"
                        )

    def test_complete_absence_removes_only_the_owned_loose_mirror(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            output = root / "runtime.orpk"
            mirror = root / "encyclopedia"
            pack.publish_runtime_artifacts(encyclopedia_entries(root), output, mirror)
            base_only = [retained_entry("FLEET.DAT", b"new-dat", root)]

            pack.publish_runtime_artifacts(base_only, output, mirror)

            self.assertFalse(mirror.exists())
            self.assertNotIn(
                (pack.KIND_GAME_DATA, "encyclopedia/catalog.json"),
                read_pack_entries(output),
            )

    def test_live_writer_lock_rejects_a_second_publication(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            output = root / "runtime.orpk"
            mirror = root / "encyclopedia"
            lock_path = root / ".runtime.orpk.encyclopedia.lock"
            lock_path.touch()
            with lock_path.open("r+") as handle:
                fcntl.flock(handle.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
                with self.assertRaisesRegex(ValueError, "publication.*busy"):
                    pack.publish_runtime_artifacts(
                        encyclopedia_entries(root), output, mirror
                    )

    def test_publication_record_budget_accepts_exact_and_rejects_one_above(self):
        with tempfile.TemporaryDirectory() as directory:
            record_path = Path(directory) / "publication.json"
            record = {
                "schema_version": pack.PUBLICATION_SCHEMA_VERSION,
                "phase": "complete",
                "padding": "synthetic",
            }
            encoded = json.dumps(
                record, sort_keys=True, separators=(",", ":")
            ).encode("utf-8")
            with mock.patch.object(
                pack, "PUBLICATION_RECORD_MAX_BYTES", len(encoded)
            ):
                pack._write_publication_record(record_path, record)
            previous = record_path.read_bytes()

            with mock.patch.object(
                pack, "PUBLICATION_RECORD_MAX_BYTES", len(encoded) - 1
            ):
                with self.assertRaisesRegex(ValueError, "resource budget"):
                    pack._write_publication_record(record_path, record)
            self.assertEqual(record_path.read_bytes(), previous)

    def test_symlink_publication_targets_cannot_escape_the_build_root(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            external_pack = root / "external.orpk"
            external_pack.write_bytes(b"user pack")
            output = root / "runtime.orpk"
            output.symlink_to(external_pack)
            with self.assertRaisesRegex(ValueError, "must not be symlinks"):
                pack.publish_runtime_artifacts(
                    encyclopedia_entries(root), output, root / "encyclopedia"
                )
            self.assertEqual(external_pack.read_bytes(), b"user pack")

            output.unlink()
            external_mirror = root / "external-mirror"
            external_mirror.mkdir()
            user_file = external_mirror / "notes.txt"
            user_file.write_text("mine", encoding="utf-8")
            mirror = root / "encyclopedia"
            mirror.symlink_to(external_mirror, target_is_directory=True)
            with self.assertRaisesRegex(ValueError, "must not be symlinks"):
                pack.publish_runtime_artifacts(
                    encyclopedia_entries(root), output, mirror
                )
            self.assertEqual(user_file.read_text(encoding="utf-8"), "mine")

    def test_optional_required_and_partial_namespace_policy(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            base = root / "base"
            ui = root / "ui"
            base.mkdir()
            ui.mkdir()
            (base / "FLEET.DAT").write_bytes(b"synthetic")
            absent = root / "absent"

            optional = pack.collect_entries(
                base, ui, encyclopedia_dir=absent, require_encyclopedia=False
            )
            self.assertFalse(
                any(entry.key.startswith("encyclopedia/") for entry in optional)
            )
            with self.assertRaisesRegex(ValueError, "required.*absent"):
                pack.collect_entries(
                    base, ui, encyclopedia_dir=absent, require_encyclopedia=True
                )

            partial = root / "partial"
            partial.mkdir()
            (partial / "source-report.json").write_text("{}", encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "manifest.json"):
                pack.collect_entries(
                    base, ui, encyclopedia_dir=partial, require_encyclopedia=False
                )


class BuildScriptIntegrationTests(unittest.TestCase):
    def make_executable(self, path: Path, contents: str):
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(contents, encoding="utf-8")
        path.chmod(0o755)

    def test_build_wasm_stages_explicit_source_and_publishes_pack_with_mirror(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory) / "repo"
            scripts = root / "scripts"
            scripts.mkdir(parents=True)
            shutil.copy2(REPO_ROOT / "scripts" / "build-wasm.sh", scripts)
            (scripts / "build-runtime-pack.py").write_text("# intercepted\n")
            (root / "Cargo.toml").write_text("[workspace]\n", encoding="utf-8")
            (root / "web").mkdir()
            (root / "web" / "gl.js").write_text("// fixture\n", encoding="utf-8")
            (root / "data" / "base" / "ui" / "fixture-dll" / "BMP").mkdir(
                parents=True
            )
            (root / "data" / "base" / "ui" / "fixture-dll" / "BMP" / "1.bmp").write_bytes(
                b"BMfixture"
            )
            (root / "data" / "base" / "FLEET.DAT").write_bytes(b"dat")
            (root / "custom-target" / "wasm32-unknown-unknown" / "release").mkdir(
                parents=True
            )
            wasm = (
                root
                / "custom-target"
                / "wasm32-unknown-unknown"
                / "release"
                / "open-rebellion.wasm"
            )
            wasm.write_bytes(b"wasm")
            self.make_executable(
                root / "custom-target" / "release" / "dat-dumper",
                "#!/bin/sh\nexit 0\n",
            )
            source = root / "owned-source"
            edata = source / "EData"
            edata.mkdir(parents=True)
            log = root / "commands.log"
            fake_bin = root / "fake-bin"
            self.make_executable(fake_bin / "cargo", "#!/bin/sh\nexit 0\n")
            self.make_executable(
                fake_bin / "go",
                f"#!/bin/sh\nprintf 'go %s\\n' \"$*\" >> '{log}'\nexit 0\n",
            )
            self.make_executable(
                fake_bin / "python3",
                "#!/bin/sh\n"
                f"case \"$1\" in *build-runtime-pack.py) printf 'pack %s\\n' \"$*\" >> '{log}'; "
                "while [ $# -gt 0 ]; do "
                "if [ \"$1\" = --output ]; then shift; "
                "mkdir -p \"$(dirname \"$1\")\"; printf pack > \"$1\"; fi; "
                "shift; done; exit 0;; esac\n"
                "exec /usr/bin/python3 \"$@\"\n",
            )
            env = os.environ.copy()
            env.update(
                {
                    "PATH": f"{fake_bin}:/usr/bin:/bin",
                    "REBELLION_ENCYCLOPEDIA_SOURCE": str(source),
                    "REBELLION_EDATA_DIR": str(edata),
                    "REBELLION_REQUIRE_ENCYCLOPEDIA": "1",
                    "FORCE_REBUILD": "1",
                    "CARGO_TARGET_DIR": "custom-target",
                }
            )

            result = subprocess.run(
                ["bash", str(scripts / "build-wasm.sh")],
                cwd=root,
                env=env,
                capture_output=True,
                text=True,
                check=False,
            )

            self.assertEqual(result.returncode, 0, result.stderr + result.stdout)
            commands = log.read_text(encoding="utf-8")
            self.assertIn(f"--encyclopedia-canonical-only --source {source}", commands)
            self.assertIn(f"--edata {edata}", commands)
            self.assertIn("--force", commands)
            pack_line = next(line for line in commands.splitlines() if "--output" in line)
            self.assertIn("--encyclopedia-mirror", pack_line)
            self.assertIn("--require-encyclopedia", pack_line)
            self.assertNotIn("--edata", pack_line)

    def test_docker_stage_uses_external_edata_without_exe_or_prepare_modding(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory) / "repo"
            scripts = root / "scripts"
            scripts.mkdir(parents=True)
            shutil.copy2(REPO_ROOT / "scripts" / "docker-build.sh", scripts)
            original = root / "owned-source"
            (original / "GData").mkdir(parents=True)
            (original / "GData" / "FLEET.DAT").write_bytes(b"dat")
            (original / "ENCYTEXT.DLL").write_bytes(b"dll")
            (original / "EData").mkdir()
            (original / "EData" / "EDATA.001").write_bytes(b"BMfixture")
            log = root / "commands.log"
            fake_bin = root / "fake-bin"
            self.make_executable(
                fake_bin / "go",
                f"#!/bin/sh\nprintf 'go %s\\n' \"$*\" >> '{log}'\nexit 0\n",
            )
            self.make_executable(
                scripts / "build-wasm.sh",
                "#!/bin/sh\n"
                f"printf 'force=%s edata=%s\\n' \"${{FORCE_REBUILD:-}}\" "
                f"\"${{REBELLION_EDATA_DIR:-}}\" >> '{log}'\n",
            )
            env = os.environ.copy()
            env.update(
                {
                    "PATH": f"{fake_bin}:/usr/bin:/bin",
                    "ORIGINAL_GAME_DIR": str(original),
                    "FORCE_REBUILD": "1",
                    "PREPARE_MODDING": "0",
                }
            )

            result = subprocess.run(
                ["bash", str(scripts / "docker-build.sh")],
                cwd=root,
                env=env,
                capture_output=True,
                text=True,
                check=False,
            )

            self.assertEqual(result.returncode, 0, result.stderr + result.stdout)
            commands = log.read_text(encoding="utf-8")
            self.assertIn(f"--edata {original / 'EData'}", commands)
            self.assertIn(
                "--encyclopedia-canonical-output data/base/encyclopedia", commands
            )
            self.assertIn("--force", commands)
            self.assertIn(f"force=1 edata={original / 'EData'}", commands)
            self.assertFalse((original / "REBEXE.EXE").exists())

    def test_package_web_forwards_explicit_requirement_and_ships_the_pack(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory) / "repo"
            scripts = root / "scripts"
            scripts.mkdir(parents=True)
            shutil.copy2(REPO_ROOT / "scripts" / "package-web.sh", scripts)
            log = root / "build.log"
            self.make_executable(
                scripts / "build-wasm.sh",
                "#!/bin/sh\n"
                f"printf 'require=%s\\n' \"${{REBELLION_REQUIRE_ENCYCLOPEDIA:-}}\" > '{log}'\n"
                "mkdir -p web/data\n"
                "printf wasm > web/open-rebellion.wasm\n"
                "printf pack > web/data/runtime.orpk\n",
            )
            (root / "web").mkdir()
            (root / "web" / "index.html").write_text("fixture", encoding="utf-8")
            (root / "web" / "gl.js").write_text("fixture", encoding="utf-8")
            env = os.environ.copy()
            env["REBELLION_REQUIRE_ENCYCLOPEDIA"] = "1"

            result = subprocess.run(
                ["bash", str(scripts / "package-web.sh"), "fixture"],
                cwd=root,
                env=env,
                capture_output=True,
                text=True,
                check=False,
            )

            self.assertEqual(result.returncode, 0, result.stderr + result.stdout)
            self.assertEqual(log.read_text(encoding="utf-8"), "require=1\n")
            distribution = root / "dist" / "open-rebellion-web-fixture"
            self.assertEqual((distribution / "data" / "runtime.orpk").read_bytes(), b"pack")
            self.assertFalse((distribution / "data" / "encyclopedia").exists())
            sums = (distribution / "SHA256SUMS").read_text(encoding="utf-8")
            self.assertIn("./data/runtime.orpk", sums)
            self.assertGreater(
                (root / "dist" / "open-rebellion-web-fixture.zip").stat().st_size,
                0,
            )


if __name__ == "__main__":
    unittest.main()
