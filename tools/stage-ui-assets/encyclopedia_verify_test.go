package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func createTestEncyclopediaBundle(t *testing.T, root, contents string) {
	t.Helper()
	if err := os.MkdirAll(root, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := buildTestEncyclopediaCandidate(map[string]string{"source-report.json": contents})(root); err != nil {
		t.Fatal(err)
	}
}

func writeTestEncyclopediaTransaction(t *testing.T, paths encyclopediaPublicationPaths, transaction encyclopediaTransaction) {
	t.Helper()
	guard, err := os.OpenFile(paths.marker+encyclopediaWriterGuardSuffix, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0o600)
	if err == nil {
		if err := guard.Close(); err != nil {
			t.Fatal(err)
		}
	} else if !os.IsExist(err) {
		t.Fatal(err)
	}
	if err := writeEncyclopediaTransaction(paths.journal, transaction); err != nil {
		t.Fatal(err)
	}
}

func testEncyclopediaMarkerToken(t *testing.T) string {
	t.Helper()
	token, err := randomEncyclopediaToken()
	if err != nil {
		t.Fatal(err)
	}
	return token
}

func testEncyclopediaTransaction(paths encyclopediaPublicationPaths, phase encyclopediaTransactionPhase, hadDestination bool) encyclopediaTransaction {
	return encyclopediaTransaction{
		Version:        encyclopediaTransactionVersion,
		OutputBase:     paths.base,
		CandidateBase:  "." + paths.base + "-" + encyclopediaCandidatePrefix + "fixture",
		BackupBase:     "." + paths.base + "-" + encyclopediaBackupPrefix + "fixture",
		HadDestination: hadDestination,
		Phase:          phase,
	}
}

func TestEncyclopediaVerifyNeedsNoSourceAndCreatesNoWriterMarker(t *testing.T) {
	parent := t.TempDir()
	output := filepath.Join(parent, "encyclopedia-research")
	createTestEncyclopediaBundle(t, output, "synthetic")
	before := snapshotTestTree(t, parent)

	var log bytes.Buffer
	if err := verifyEncyclopediaDirectory(output, inspectTestEncyclopediaCandidate, "recover encyclopedia", &log); err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(log.String(), "Verified encyclopedia directory") {
		t.Fatalf("verification log = %q", log.String())
	}
	if diff := diffTestTree(before, snapshotTestTree(t, parent)); diff != "" {
		t.Fatalf("verification wrote to filesystem:\n%s", diff)
	}
	paths, err := prepareEncyclopediaPublicationPaths(output)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := os.Lstat(paths.marker); !os.IsNotExist(err) {
		t.Fatalf("verification created writer marker: %v", err)
	}
}

func TestEncyclopediaVerifyReportsEveryTransactionPhaseWithoutRepair(t *testing.T) {
	phases := []encyclopediaTransactionPhase{
		encyclopediaPhaseCandidateValidated,
		encyclopediaPhaseDestinationBackedUp,
		encyclopediaPhaseCandidatePublished,
		encyclopediaPhaseCleanupPending,
	}
	for _, phase := range phases {
		t.Run(string(phase), func(t *testing.T) {
			parent := t.TempDir()
			output := filepath.Join(parent, "encyclopedia")
			paths, err := prepareEncyclopediaPublicationPaths(output)
			if err != nil {
				t.Fatal(err)
			}
			transaction := testEncyclopediaTransaction(paths, phase, true)
			candidate := filepath.Join(parent, transaction.CandidateBase)
			backup := filepath.Join(parent, transaction.BackupBase)
			switch phase {
			case encyclopediaPhaseCandidateValidated:
				createTestEncyclopediaBundle(t, output, "old")
				createTestEncyclopediaBundle(t, candidate, "new")
			case encyclopediaPhaseDestinationBackedUp:
				createTestEncyclopediaBundle(t, backup, "old")
				createTestEncyclopediaBundle(t, candidate, "new")
			case encyclopediaPhaseCandidatePublished, encyclopediaPhaseCleanupPending:
				createTestEncyclopediaBundle(t, output, "new")
				createTestEncyclopediaBundle(t, backup, "old")
			}
			writeTestEncyclopediaTransaction(t, paths, transaction)
			before := snapshotTestTree(t, parent)

			err = verifyEncyclopediaDirectory(output, inspectTestEncyclopediaCandidate, "recover encyclopedia", io.Discard)
			if err == nil || !strings.Contains(err.Error(), string(phase)) || !strings.Contains(err.Error(), "recover encyclopedia") {
				t.Fatalf("phase diagnostic = %v", err)
			}
			if diff := diffTestTree(before, snapshotTestTree(t, parent)); diff != "" {
				t.Fatalf("verification repaired phase %s:\n%s", phase, diff)
			}
		})
	}
}

func TestEncyclopediaVerifyReportsActiveWriterWhileDestinationIsAbsent(t *testing.T) {
	parent := t.TempDir()
	output := filepath.Join(parent, "encyclopedia")
	paths, err := prepareEncyclopediaPublicationPaths(output)
	if err != nil {
		t.Fatal(err)
	}
	marker := encyclopediaWriterMarker{
		Version:   encyclopediaTransactionVersion,
		PID:       os.Getpid(),
		Token:     testEncyclopediaMarkerToken(t),
		CreatedAt: "2026-09-28T00:00:00Z",
	}
	encoded, err := json.Marshal(marker)
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(paths.marker, encoded, 0o600); err != nil {
		t.Fatal(err)
	}
	before := snapshotTestTree(t, parent)

	err = verifyEncyclopediaDirectory(output, inspectTestEncyclopediaCandidate, "recover encyclopedia", io.Discard)
	if err == nil || !strings.Contains(err.Error(), "active") || !strings.Contains(err.Error(), "recover encyclopedia") {
		t.Fatalf("active-writer diagnostic = %v", err)
	}
	if diff := diffTestTree(before, snapshotTestTree(t, parent)); diff != "" {
		t.Fatalf("verification changed active transaction:\n%s", diff)
	}
}

func TestEncyclopediaStageRecoversInterruptedBackupBeforeContinuing(t *testing.T) {
	parent := t.TempDir()
	output := filepath.Join(parent, "encyclopedia")
	paths, err := prepareEncyclopediaPublicationPaths(output)
	if err != nil {
		t.Fatal(err)
	}
	transaction := testEncyclopediaTransaction(paths, encyclopediaPhaseDestinationBackedUp, true)
	candidate := filepath.Join(parent, transaction.CandidateBase)
	backup := filepath.Join(parent, transaction.BackupBase)
	createTestEncyclopediaBundle(t, candidate, "abandoned-new")
	createTestEncyclopediaBundle(t, backup, "old")
	writeTestEncyclopediaTransaction(t, paths, transaction)

	request := testEncyclopediaRequest(output, map[string]string{"source-report.json": "old"}, true)
	result, err := stageEncyclopediaDirectory(request)
	if err != nil {
		t.Fatal(err)
	}
	if !result.Recovered || result.Changed {
		t.Fatalf("recovery result = %+v", result)
	}
	if got := string(readTestFile(t, filepath.Join(output, "source-report.json"))); got != "old" {
		t.Fatalf("recovered output = %q", got)
	}
	for _, path := range []string{candidate, backup, paths.journal, paths.marker} {
		if _, err := os.Lstat(path); !os.IsNotExist(err) {
			t.Fatalf("recovery left %s: %v", path, err)
		}
	}
}

func TestEncyclopediaStageCompletesValidatedCleanupRecovery(t *testing.T) {
	parent := t.TempDir()
	output := filepath.Join(parent, "encyclopedia")
	paths, err := prepareEncyclopediaPublicationPaths(output)
	if err != nil {
		t.Fatal(err)
	}
	transaction := testEncyclopediaTransaction(paths, encyclopediaPhaseCleanupPending, true)
	backup := filepath.Join(parent, transaction.BackupBase)
	createTestEncyclopediaBundle(t, output, "new")
	createTestEncyclopediaBundle(t, backup, "old")
	writeTestEncyclopediaTransaction(t, paths, transaction)

	request := testEncyclopediaRequest(output, map[string]string{"source-report.json": "new"}, true)
	result, err := stageEncyclopediaDirectory(request)
	if err != nil {
		t.Fatal(err)
	}
	if !result.Recovered || result.Changed {
		t.Fatalf("cleanup recovery result = %+v", result)
	}
	if _, err := os.Lstat(backup); !os.IsNotExist(err) {
		t.Fatalf("validated backup not cleaned: %v", err)
	}
}

func TestEncyclopediaStagePublishesSoleValidatedCandidateAfterInterruptedInitialPublication(t *testing.T) {
	parent := t.TempDir()
	output := filepath.Join(parent, "encyclopedia")
	paths, err := prepareEncyclopediaPublicationPaths(output)
	if err != nil {
		t.Fatal(err)
	}
	transaction := testEncyclopediaTransaction(paths, encyclopediaPhaseDestinationBackedUp, false)
	candidate := filepath.Join(parent, transaction.CandidateBase)
	createTestEncyclopediaBundle(t, candidate, "first")
	writeTestEncyclopediaTransaction(t, paths, transaction)

	request := testEncyclopediaRequest(output, map[string]string{"source-report.json": "first"}, true)
	result, err := stageEncyclopediaDirectory(request)
	if err != nil {
		t.Fatal(err)
	}
	if !result.Recovered || result.Changed {
		t.Fatalf("initial recovery result = %+v", result)
	}
	if got := string(readTestFile(t, filepath.Join(output, "source-report.json"))); got != "first" {
		t.Fatalf("recovered initial output = %q", got)
	}
}

func TestEncyclopediaStageAcceptsPublishedCandidateFromInitialCrashWindow(t *testing.T) {
	parent := t.TempDir()
	output := filepath.Join(parent, "encyclopedia")
	paths, err := prepareEncyclopediaPublicationPaths(output)
	if err != nil {
		t.Fatal(err)
	}
	transaction := testEncyclopediaTransaction(paths, encyclopediaPhaseDestinationBackedUp, false)
	createTestEncyclopediaBundle(t, output, "first")
	writeTestEncyclopediaTransaction(t, paths, transaction)

	request := testEncyclopediaRequest(output, map[string]string{"source-report.json": "first"}, true)
	result, err := stageEncyclopediaDirectory(request)
	if err != nil {
		t.Fatal(err)
	}
	if !result.Recovered || result.Changed {
		t.Fatalf("initial published recovery result = %+v", result)
	}
	if got := string(readTestFile(t, filepath.Join(output, "source-report.json"))); got != "first" {
		t.Fatalf("recovered initial output = %q", got)
	}
}

func TestEncyclopediaStageContinuesAfterInitialRollbackJournalCleanupFailure(t *testing.T) {
	parent := t.TempDir()
	output := filepath.Join(parent, "encyclopedia")
	paths, err := prepareEncyclopediaPublicationPaths(output)
	if err != nil {
		t.Fatal(err)
	}
	transaction := testEncyclopediaTransaction(paths, encyclopediaPhaseDestinationBackedUp, false)
	writeTestEncyclopediaTransaction(t, paths, transaction)

	request := testEncyclopediaRequest(output, map[string]string{"source-report.json": "retry"}, true)
	result, err := stageEncyclopediaDirectory(request)
	if err != nil {
		t.Fatal(err)
	}
	if !result.Recovered || !result.Changed {
		t.Fatalf("initial rollback cleanup result = %+v", result)
	}
	if got := string(readTestFile(t, filepath.Join(output, "source-report.json"))); got != "retry" {
		t.Fatalf("retried initial output = %q", got)
	}
}

func TestEncyclopediaRecoveryRefusesUnexpectedBackupForInitialPublication(t *testing.T) {
	parent := t.TempDir()
	output := filepath.Join(parent, "encyclopedia")
	paths, err := prepareEncyclopediaPublicationPaths(output)
	if err != nil {
		t.Fatal(err)
	}
	transaction := testEncyclopediaTransaction(paths, encyclopediaPhaseCleanupPending, false)
	createTestEncyclopediaBundle(t, output, "new")
	createTestEncyclopediaBundle(t, filepath.Join(parent, transaction.BackupBase), "unexpected")
	writeTestEncyclopediaTransaction(t, paths, transaction)
	before := snapshotTestTree(t, parent)

	request := testEncyclopediaRequest(output, map[string]string{"source-report.json": "new"}, true)
	if _, err := stageEncyclopediaDirectory(request); err == nil || !strings.Contains(err.Error(), "ambiguous") {
		t.Fatalf("unexpected initial backup recovery error = %v", err)
	}
	if diff := diffTestTree(before, snapshotTestTree(t, parent)); diff != "" {
		t.Fatalf("ambiguous initial recovery changed filesystem:\n%s", diff)
	}
}

func TestEncyclopediaRecoveryRefusesInvalidOwnedSetsAndPreservesUnknownBytes(t *testing.T) {
	parent := t.TempDir()
	output := filepath.Join(parent, "encyclopedia")
	paths, err := prepareEncyclopediaPublicationPaths(output)
	if err != nil {
		t.Fatal(err)
	}
	transaction := testEncyclopediaTransaction(paths, encyclopediaPhaseDestinationBackedUp, true)
	candidate := filepath.Join(parent, transaction.CandidateBase)
	backup := filepath.Join(parent, transaction.BackupBase)
	createTestEncyclopediaBundle(t, candidate, "new")
	createTestEncyclopediaBundle(t, backup, "old")
	unknown := []byte{0xde, 0xad, 0xbe, 0xef, 0, '\n'}
	unknownPath := filepath.Join(backup, "user-notes.bin")
	if err := os.WriteFile(unknownPath, unknown, 0o600); err != nil {
		t.Fatal(err)
	}
	writeTestEncyclopediaTransaction(t, paths, transaction)
	before := snapshotTestTree(t, parent)

	request := testEncyclopediaRequest(output, map[string]string{"source-report.json": "new"}, true)
	if _, err := stageEncyclopediaDirectory(request); err == nil || !strings.Contains(err.Error(), "user-notes.bin") {
		t.Fatalf("invalid recovery error = %v", err)
	}
	if diff := diffTestTree(before, snapshotTestTree(t, parent)); diff != "" {
		t.Fatalf("invalid recovery changed filesystem:\n%s", diff)
	}
	if got := readTestFile(t, unknownPath); !bytes.Equal(got, unknown) {
		t.Fatalf("unknown recovery file changed: %x", got)
	}
}

func TestEncyclopediaRecoveryFailuresRetainValidatedBundlesAtEveryPhase(t *testing.T) {
	tests := []struct {
		phase encyclopediaTransactionPhase
		setup func(t *testing.T, paths encyclopediaPublicationPaths, transaction encyclopediaTransaction)
		fail  func(*encyclopediaPublicationOps)
	}{
		{
			phase: encyclopediaPhaseCandidateValidated,
			setup: func(t *testing.T, paths encyclopediaPublicationPaths, transaction encyclopediaTransaction) {
				createTestEncyclopediaBundle(t, paths.output, "old")
				createTestEncyclopediaBundle(t, filepath.Join(paths.parent, transaction.CandidateBase), "new")
			},
			fail: func(ops *encyclopediaPublicationOps) {
				ops.removeAll = func(string) error { return fmt.Errorf("injected candidate cleanup failure") }
			},
		},
		{
			phase: encyclopediaPhaseDestinationBackedUp,
			setup: func(t *testing.T, paths encyclopediaPublicationPaths, transaction encyclopediaTransaction) {
				createTestEncyclopediaBundle(t, filepath.Join(paths.parent, transaction.BackupBase), "old")
				createTestEncyclopediaBundle(t, filepath.Join(paths.parent, transaction.CandidateBase), "new")
			},
			fail: func(ops *encyclopediaPublicationOps) {
				rename := ops.rename
				ops.rename = func(oldPath, newPath string) error {
					if filepath.Base(oldPath) == ".encyclopedia-"+encyclopediaBackupPrefix+"fixture" {
						return fmt.Errorf("injected recovery rename failure")
					}
					return rename(oldPath, newPath)
				}
			},
		},
		{
			phase: encyclopediaPhaseCandidatePublished,
			setup: func(t *testing.T, paths encyclopediaPublicationPaths, transaction encyclopediaTransaction) {
				createTestEncyclopediaBundle(t, paths.output, "new")
				createTestEncyclopediaBundle(t, filepath.Join(paths.parent, transaction.BackupBase), "old")
			},
			fail: func(ops *encyclopediaPublicationOps) {
				ops.removeAll = func(string) error { return fmt.Errorf("injected backup cleanup failure") }
			},
		},
		{
			phase: encyclopediaPhaseCleanupPending,
			setup: func(t *testing.T, paths encyclopediaPublicationPaths, transaction encyclopediaTransaction) {
				createTestEncyclopediaBundle(t, paths.output, "new")
				createTestEncyclopediaBundle(t, filepath.Join(paths.parent, transaction.BackupBase), "old")
			},
			fail: func(ops *encyclopediaPublicationOps) {
				ops.removeAll = func(string) error { return fmt.Errorf("injected cleanup recovery failure") }
			},
		},
	}

	for _, test := range tests {
		t.Run(string(test.phase), func(t *testing.T) {
			parent := t.TempDir()
			output := filepath.Join(parent, "encyclopedia")
			paths, err := prepareEncyclopediaPublicationPaths(output)
			if err != nil {
				t.Fatal(err)
			}
			transaction := testEncyclopediaTransaction(paths, test.phase, true)
			test.setup(t, paths, transaction)
			writeTestEncyclopediaTransaction(t, paths, transaction)
			before := snapshotTestTree(t, parent)
			ops := defaultEncyclopediaPublicationOps()
			test.fail(&ops)
			request := testEncyclopediaRequest(output, map[string]string{"source-report.json": "irrelevant"}, true)
			request.BuildCandidate = func(string) error { return fmt.Errorf("builder must not run during failed recovery") }
			if _, err := stageEncyclopediaDirectoryWithOps(request, ops); err == nil {
				t.Fatal("injected recovery failure returned nil")
			}
			if diff := diffTestTree(before, snapshotTestTree(t, parent)); diff != "" {
				t.Fatalf("failed recovery changed valid bundles:\n%s", diff)
			}
		})
	}
}

func TestEncyclopediaRecoveryRetryFinishesCandidateCleanupAfterPriorBundleWasRestored(t *testing.T) {
	parent := t.TempDir()
	output := filepath.Join(parent, "encyclopedia")
	paths, err := prepareEncyclopediaPublicationPaths(output)
	if err != nil {
		t.Fatal(err)
	}
	transaction := testEncyclopediaTransaction(paths, encyclopediaPhaseDestinationBackedUp, true)
	candidate := filepath.Join(parent, transaction.CandidateBase)
	backup := filepath.Join(parent, transaction.BackupBase)
	createTestEncyclopediaBundle(t, candidate, "abandoned-new")
	createTestEncyclopediaBundle(t, backup, "old")
	writeTestEncyclopediaTransaction(t, paths, transaction)

	ops := defaultEncyclopediaPublicationOps()
	removeAll := ops.removeAll
	ops.removeAll = func(path string) error {
		if path == candidate {
			return fmt.Errorf("injected candidate cleanup failure")
		}
		return removeAll(path)
	}
	request := testEncyclopediaRequest(output, map[string]string{"source-report.json": "old"}, true)
	if _, err := stageEncyclopediaDirectoryWithOps(request, ops); err == nil {
		t.Fatal("candidate cleanup failure returned nil")
	}
	if got := string(readTestFile(t, filepath.Join(output, "source-report.json"))); got != "old" {
		t.Fatalf("prior bundle was not restored before cleanup failure: %q", got)
	}
	if _, err := os.Stat(candidate); err != nil {
		t.Fatalf("validated candidate was not retained: %v", err)
	}

	result, err := stageEncyclopediaDirectory(request)
	if err != nil {
		t.Fatal(err)
	}
	if !result.Recovered || result.Changed {
		t.Fatalf("cleanup retry result = %+v", result)
	}
	if _, err := os.Lstat(candidate); !os.IsNotExist(err) {
		t.Fatalf("cleanup retry left candidate: %v", err)
	}
}

func TestEncyclopediaRecoveryFinishesCandidateCleanupAfterPublishedRollback(t *testing.T) {
	parent := t.TempDir()
	output := filepath.Join(parent, "encyclopedia")
	paths, err := prepareEncyclopediaPublicationPaths(output)
	if err != nil {
		t.Fatal(err)
	}
	transaction := testEncyclopediaTransaction(paths, encyclopediaPhaseCandidatePublished, true)
	candidate := filepath.Join(parent, transaction.CandidateBase)
	createTestEncyclopediaBundle(t, output, "old")
	createTestEncyclopediaBundle(t, candidate, "abandoned-new")
	writeTestEncyclopediaTransaction(t, paths, transaction)

	request := testEncyclopediaRequest(output, map[string]string{"source-report.json": "old"}, true)
	result, err := stageEncyclopediaDirectory(request)
	if err != nil {
		t.Fatal(err)
	}
	if !result.Recovered || result.Changed {
		t.Fatalf("published rollback cleanup result = %+v", result)
	}
	if _, err := os.Lstat(candidate); !os.IsNotExist(err) {
		t.Fatalf("published rollback cleanup left candidate: %v", err)
	}
}

func TestEncyclopediaRecoveryRetryFinishesJournalCleanupAfterRollback(t *testing.T) {
	tests := []struct {
		name      string
		configure func(*encyclopediaPublicationOps)
	}{
		{
			name: "candidate validated",
			configure: func(ops *encyclopediaPublicationOps) {
				rename := ops.rename
				ops.rename = func(oldPath, newPath string) error {
					if strings.Contains(filepath.Base(newPath), encyclopediaBackupPrefix) {
						return fmt.Errorf("injected backup rename failure")
					}
					return rename(oldPath, newPath)
				}
			},
		},
		{
			name: "destination backed up",
			configure: func(ops *encyclopediaPublicationOps) {
				rename := ops.rename
				ops.rename = func(oldPath, newPath string) error {
					if filepath.Base(newPath) == "encyclopedia" && strings.Contains(filepath.Base(oldPath), encyclopediaCandidatePrefix) {
						return fmt.Errorf("injected candidate rename failure")
					}
					return rename(oldPath, newPath)
				}
			},
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			parent := t.TempDir()
			output := filepath.Join(parent, "encyclopedia")
			paths, err := prepareEncyclopediaPublicationPaths(output)
			if err != nil {
				t.Fatal(err)
			}
			if _, err := stageEncyclopediaDirectory(testEncyclopediaRequest(output, map[string]string{"source-report.json": "old"}, false)); err != nil {
				t.Fatal(err)
			}

			ops := defaultEncyclopediaPublicationOps()
			test.configure(&ops)
			remove := ops.remove
			ops.remove = func(path string) error {
				if path == paths.journal {
					return fmt.Errorf("injected journal cleanup failure")
				}
				return remove(path)
			}
			request := testEncyclopediaRequest(output, map[string]string{"source-report.json": "new"}, true)
			if _, err := stageEncyclopediaDirectoryWithOps(request, ops); err == nil {
				t.Fatal("rollback journal cleanup failure returned nil")
			}
			if got := string(readTestFile(t, filepath.Join(output, "source-report.json"))); got != "old" {
				t.Fatalf("prior bundle was not restored: %q", got)
			}
			if _, err := os.Stat(paths.journal); err != nil {
				t.Fatalf("failed journal cleanup did not retain journal: %v", err)
			}

			retry := testEncyclopediaRequest(output, map[string]string{"source-report.json": "old"}, true)
			result, err := stageEncyclopediaDirectory(retry)
			if err != nil {
				t.Fatal(err)
			}
			if !result.Recovered || result.Changed {
				t.Fatalf("journal cleanup retry result = %+v", result)
			}
			if _, err := os.Lstat(paths.journal); !os.IsNotExist(err) {
				t.Fatalf("journal cleanup retry left journal: %v", err)
			}
		})
	}
}

func TestEncyclopediaStageReplacesDeadWriterMarkerBeforeRecovery(t *testing.T) {
	parent := t.TempDir()
	output := filepath.Join(parent, "encyclopedia")
	paths, err := prepareEncyclopediaPublicationPaths(output)
	if err != nil {
		t.Fatal(err)
	}
	transaction := testEncyclopediaTransaction(paths, encyclopediaPhaseDestinationBackedUp, true)
	createTestEncyclopediaBundle(t, filepath.Join(parent, transaction.CandidateBase), "abandoned-new")
	createTestEncyclopediaBundle(t, filepath.Join(parent, transaction.BackupBase), "old")
	writeTestEncyclopediaTransaction(t, paths, transaction)
	marker := encyclopediaWriterMarker{
		Version:   encyclopediaTransactionVersion,
		PID:       1 << 30,
		Token:     testEncyclopediaMarkerToken(t),
		CreatedAt: "2026-09-28T00:00:00Z",
	}
	encoded, err := json.Marshal(marker)
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(paths.marker, encoded, 0o600); err != nil {
		t.Fatal(err)
	}

	result, err := stageEncyclopediaDirectory(testEncyclopediaRequest(output, map[string]string{"source-report.json": "old"}, true))
	if err != nil {
		t.Fatal(err)
	}
	if !result.Recovered || result.Changed {
		t.Fatalf("dead-writer recovery result = %+v", result)
	}
	if _, err := os.Lstat(paths.marker); !os.IsNotExist(err) {
		t.Fatalf("writer marker remained after recovery: %v", err)
	}
}

func TestEncyclopediaCleanupFailureLeavesRecoverablePublishedBundle(t *testing.T) {
	parent := t.TempDir()
	output := filepath.Join(parent, "encyclopedia")
	if _, err := stageEncyclopediaDirectory(testEncyclopediaRequest(output, map[string]string{"source-report.json": "old"}, false)); err != nil {
		t.Fatal(err)
	}
	ops := defaultEncyclopediaPublicationOps()
	removeAll := ops.removeAll
	ops.removeAll = func(path string) error {
		if strings.Contains(filepath.Base(path), encyclopediaBackupPrefix) {
			return fmt.Errorf("injected post-publication cleanup failure")
		}
		return removeAll(path)
	}
	request := testEncyclopediaRequest(output, map[string]string{"source-report.json": "new"}, true)
	if _, err := stageEncyclopediaDirectoryWithOps(request, ops); err == nil || !strings.Contains(err.Error(), "recover with") {
		t.Fatalf("cleanup failure error = %v", err)
	}
	if got := string(readTestFile(t, filepath.Join(output, "source-report.json"))); got != "new" {
		t.Fatalf("validated published bundle was not retained: %q", got)
	}
	beforeVerify := snapshotTestTree(t, parent)
	if err := verifyEncyclopediaDirectory(output, inspectTestEncyclopediaCandidate, "recover encyclopedia", io.Discard); err == nil {
		t.Fatal("verify accepted cleanup-pending transaction")
	}
	if diff := diffTestTree(beforeVerify, snapshotTestTree(t, parent)); diff != "" {
		t.Fatalf("verify repaired cleanup failure:\n%s", diff)
	}
	result, err := stageEncyclopediaDirectory(request)
	if err != nil {
		t.Fatal(err)
	}
	if !result.Recovered || result.Changed {
		t.Fatalf("cleanup retry result = %+v", result)
	}
}

func TestEncyclopediaRecoveryRejectsJournalPathsOutsideSiblingNamespace(t *testing.T) {
	parent := t.TempDir()
	output := filepath.Join(parent, "encyclopedia")
	paths, err := prepareEncyclopediaPublicationPaths(output)
	if err != nil {
		t.Fatal(err)
	}
	createTestEncyclopediaBundle(t, output, "old")
	outside := filepath.Join(t.TempDir(), "do-not-touch")
	if err := os.WriteFile(outside, []byte("user data"), 0o600); err != nil {
		t.Fatal(err)
	}
	transaction := testEncyclopediaTransaction(paths, encyclopediaPhaseCandidateValidated, true)
	transaction.CandidateBase = filepath.Join("..", "..", filepath.Base(outside))
	encoded, err := json.Marshal(transaction)
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(paths.journal, encoded, 0o600); err != nil {
		t.Fatal(err)
	}
	before := readTestFile(t, outside)

	if _, err := stageEncyclopediaDirectory(testEncyclopediaRequest(output, map[string]string{"source-report.json": "new"}, true)); err == nil || !strings.Contains(err.Error(), "escape") {
		t.Fatalf("escaping journal error = %v", err)
	}
	if got := readTestFile(t, outside); !bytes.Equal(got, before) {
		t.Fatalf("outside file changed: %x", got)
	}
}
