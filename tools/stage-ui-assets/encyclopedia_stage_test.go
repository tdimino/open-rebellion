package main

import (
	"bufio"
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"strings"
	"sync"
	"testing"
	"time"
)

const testEncyclopediaOwnershipFile = "ownership.json"

const (
	testEncyclopediaWriterHelperAction = "REBELLION_TEST_ENCYCLOPEDIA_WRITER_ACTION"
	testEncyclopediaWriterHelperMarker = "REBELLION_TEST_ENCYCLOPEDIA_WRITER_MARKER"
)

type testEncyclopediaOwnership struct {
	Files []string `json:"files"`
}

func buildTestEncyclopediaCandidate(files map[string]string) encyclopediaCandidateBuilder {
	return func(root string) error {
		paths := make([]string, 0, len(files)+1)
		for path, contents := range files {
			full := filepath.Join(root, filepath.FromSlash(path))
			if err := os.MkdirAll(filepath.Dir(full), 0o755); err != nil {
				return err
			}
			if err := os.WriteFile(full, []byte(contents), 0o644); err != nil {
				return err
			}
			paths = append(paths, path)
		}
		paths = append(paths, testEncyclopediaOwnershipFile)
		sort.Strings(paths)
		encoded, err := json.Marshal(testEncyclopediaOwnership{Files: paths})
		if err != nil {
			return err
		}
		return os.WriteFile(filepath.Join(root, testEncyclopediaOwnershipFile), append(encoded, '\n'), 0o644)
	}
}

func inspectTestEncyclopediaCandidate(root string) (encyclopediaOwnedInventory, error) {
	encoded, err := os.ReadFile(filepath.Join(root, testEncyclopediaOwnershipFile))
	if err != nil {
		return encyclopediaOwnedInventory{}, err
	}
	var ownership testEncyclopediaOwnership
	if err := json.Unmarshal(encoded, &ownership); err != nil {
		return encyclopediaOwnedInventory{}, err
	}
	return encyclopediaOwnedInventory{Files: ownership.Files}, nil
}

func testEncyclopediaRequest(output string, files map[string]string, force bool) encyclopediaDirectoryRequest {
	return encyclopediaDirectoryRequest{
		OutputDir:       output,
		Force:           force,
		BuildCandidate:  buildTestEncyclopediaCandidate(files),
		InspectOwned:    inspectTestEncyclopediaCandidate,
		RecoveryCommand: "stage-ui-assets --encyclopedia-report-only",
		Log:             io.Discard,
	}
}

func readTestFile(t *testing.T, path string) []byte {
	t.Helper()
	contents, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	return contents
}

func runTestEncyclopediaStage(request encyclopediaDirectoryRequest, result chan<- error, done *sync.WaitGroup) {
	defer done.Done()
	_, err := stageEncyclopediaDirectory(request)
	result <- err
}

func TestEncyclopediaWriterProcessHelper(t *testing.T) {
	action := os.Getenv(testEncyclopediaWriterHelperAction)
	if action == "" {
		return
	}
	marker := os.Getenv(testEncyclopediaWriterHelperMarker)
	if marker == "" {
		t.Fatal("writer helper marker is empty")
	}
	lease, err := acquireEncyclopediaWriter(marker)
	switch action {
	case "hold":
		if err != nil {
			t.Fatal(err)
		}
		fmt.Fprintln(os.Stdout, "acquired")
		_, _ = io.Copy(io.Discard, os.Stdin)
		releaseEncyclopediaWriter(lease)
	case "try":
		if err != nil {
			if !strings.Contains(err.Error(), "stage_busy") {
				t.Fatal(err)
			}
			fmt.Fprintln(os.Stdout, "busy")
			return
		}
		releaseEncyclopediaWriter(lease)
		fmt.Fprintln(os.Stdout, "acquired")
	case "exit":
		if err != nil {
			t.Fatal(err)
		}
		fmt.Fprintln(os.Stdout, "acquired")
		os.Exit(23)
	default:
		t.Fatalf("unknown writer helper action %q", action)
	}
}

type testEncyclopediaWriterProcess struct {
	command *exec.Cmd
	stdin   io.WriteCloser
	stdout  *bufio.Reader
	stderr  bytes.Buffer
}

type testEncyclopediaWriterOutcome struct {
	lease encyclopediaWriterLease
	err   error
}

func runTestEncyclopediaWriterAcquire(marker string, start <-chan struct{}, done chan<- testEncyclopediaWriterOutcome) {
	<-start
	lease, err := acquireEncyclopediaWriter(marker)
	done <- testEncyclopediaWriterOutcome{lease: lease, err: err}
}

func startTestEncyclopediaWriterProcess(t *testing.T, action, marker string) *testEncyclopediaWriterProcess {
	t.Helper()
	command := exec.Command(os.Args[0], "-test.run=^TestEncyclopediaWriterProcessHelper$")
	command.Env = append(os.Environ(),
		testEncyclopediaWriterHelperAction+"="+action,
		testEncyclopediaWriterHelperMarker+"="+marker,
	)
	stdin, err := command.StdinPipe()
	if err != nil {
		t.Fatal(err)
	}
	stdout, err := command.StdoutPipe()
	if err != nil {
		t.Fatal(err)
	}
	process := &testEncyclopediaWriterProcess{
		command: command,
		stdin:   stdin,
		stdout:  bufio.NewReader(stdout),
	}
	command.Stderr = &process.stderr
	if err := command.Start(); err != nil {
		t.Fatal(err)
	}
	line, err := process.stdout.ReadString('\n')
	if err != nil {
		_ = command.Wait()
		t.Fatalf("read writer helper readiness: %v (stderr %q)", err, process.stderr.String())
	}
	if line != "acquired\n" {
		_ = command.Process.Kill()
		_ = command.Wait()
		t.Fatalf("writer helper readiness = %q (stderr %q)", line, process.stderr.String())
	}
	return process
}

func runTestEncyclopediaWriterAttempt(t *testing.T, marker string) string {
	t.Helper()
	command := exec.Command(os.Args[0], "-test.run=^TestEncyclopediaWriterProcessHelper$")
	command.Env = append(os.Environ(),
		testEncyclopediaWriterHelperAction+"=try",
		testEncyclopediaWriterHelperMarker+"="+marker,
	)
	output, err := command.CombinedOutput()
	if err != nil {
		t.Fatalf("writer contender failed: %v (%s)", err, output)
	}
	return string(output)
}

func TestEncyclopediaByteIdenticalForceRerunLeavesPublishedDirectoryUntouched(t *testing.T) {
	parent := t.TempDir()
	output := filepath.Join(parent, "encyclopedia-research")
	request := testEncyclopediaRequest(output, map[string]string{
		"raw/encytext/1033/1.bin": "synthetic raw bytes",
		"source-report.json":      "synthetic report",
	}, false)

	first, err := stageEncyclopediaDirectory(request)
	if err != nil {
		t.Fatal(err)
	}
	if !first.Changed {
		t.Fatal("first publication reported no change")
	}
	info, err := os.Stat(output)
	if err != nil {
		t.Fatal(err)
	}
	beforeModTime := info.ModTime()
	beforeFiles := snapshotTestTree(t, parent)

	time.Sleep(10 * time.Millisecond)
	request.Force = true
	second, err := stageEncyclopediaDirectory(request)
	if err != nil {
		t.Fatal(err)
	}
	if second.Changed {
		t.Fatal("byte-identical forced rerun replaced published output")
	}
	afterInfo, err := os.Stat(output)
	if err != nil {
		t.Fatal(err)
	}
	if !afterInfo.ModTime().Equal(beforeModTime) {
		t.Fatalf("output directory modtime changed: before %s after %s", beforeModTime, afterInfo.ModTime())
	}
	if diff := diffTestTree(beforeFiles, snapshotTestTree(t, parent)); diff != "" {
		t.Fatalf("byte-identical rerun changed filesystem:\n%s", diff)
	}
}

func TestEncyclopediaChangedOwnedOutputRequiresForce(t *testing.T) {
	output := filepath.Join(t.TempDir(), "encyclopedia")
	original := testEncyclopediaRequest(output, map[string]string{"source-report.json": "original"}, false)
	if _, err := stageEncyclopediaDirectory(original); err != nil {
		t.Fatal(err)
	}

	changed := testEncyclopediaRequest(output, map[string]string{"source-report.json": "replacement"}, false)
	if _, err := stageEncyclopediaDirectory(changed); err == nil || !strings.Contains(err.Error(), "--force") {
		t.Fatalf("changed publication without --force error = %v", err)
	}
	if got := string(readTestFile(t, filepath.Join(output, "source-report.json"))); got != "original" {
		t.Fatalf("rejected publication changed output to %q", got)
	}

	changed.Force = true
	result, err := stageEncyclopediaDirectory(changed)
	if err != nil {
		t.Fatal(err)
	}
	if !result.Changed {
		t.Fatal("forced changed publication reported no change")
	}
	if got := string(readTestFile(t, filepath.Join(output, "source-report.json"))); got != "replacement" {
		t.Fatalf("forced publication left %q", got)
	}
}

func TestEncyclopediaUnknownUserFileSurvivesForceByteForByte(t *testing.T) {
	output := filepath.Join(t.TempDir(), "encyclopedia")
	request := testEncyclopediaRequest(output, map[string]string{"source-report.json": "original"}, false)
	if _, err := stageEncyclopediaDirectory(request); err != nil {
		t.Fatal(err)
	}
	unknownPath := filepath.Join(output, "authors-notes.txt")
	unknown := []byte{0, 1, 2, 3, 0xff, '\n'}
	if err := os.WriteFile(unknownPath, unknown, 0o600); err != nil {
		t.Fatal(err)
	}

	request = testEncyclopediaRequest(output, map[string]string{"source-report.json": "replacement"}, true)
	if _, err := stageEncyclopediaDirectory(request); err == nil || !strings.Contains(err.Error(), "authors-notes.txt") {
		t.Fatalf("forced publication with unknown file error = %v", err)
	}
	if got := readTestFile(t, unknownPath); !bytes.Equal(got, unknown) {
		t.Fatalf("unknown file changed: %x", got)
	}
	if got := string(readTestFile(t, filepath.Join(output, "source-report.json"))); got != "original" {
		t.Fatalf("owned file changed despite refusal: %q", got)
	}
}

func TestEncyclopediaUnknownEmptyDirectorySurvivesForce(t *testing.T) {
	output := filepath.Join(t.TempDir(), "encyclopedia")
	request := testEncyclopediaRequest(output, map[string]string{"source-report.json": "original"}, false)
	if _, err := stageEncyclopediaDirectory(request); err != nil {
		t.Fatal(err)
	}
	unknownDir := filepath.Join(output, "author-work")
	if err := os.Mkdir(unknownDir, 0o700); err != nil {
		t.Fatal(err)
	}

	request = testEncyclopediaRequest(output, map[string]string{"source-report.json": "replacement"}, true)
	if _, err := stageEncyclopediaDirectory(request); err == nil || !strings.Contains(err.Error(), "author-work") {
		t.Fatalf("forced publication with unknown directory error = %v", err)
	}
	info, err := os.Stat(unknownDir)
	if err != nil || !info.IsDir() {
		t.Fatalf("unknown directory did not survive: %v", err)
	}
	if got := string(readTestFile(t, filepath.Join(output, "source-report.json"))); got != "original" {
		t.Fatalf("owned file changed despite refusal: %q", got)
	}
}

func TestEncyclopediaPublicationFailuresRestorePreviousValidBundle(t *testing.T) {
	tests := []struct {
		name      string
		configure func(*encyclopediaPublicationOps)
	}{
		{
			name: "candidate journal",
			configure: func(ops *encyclopediaPublicationOps) {
				write := ops.writeJournal
				ops.writeJournal = func(path string, transaction encyclopediaTransaction) error {
					if transaction.Phase == encyclopediaPhaseCandidateValidated {
						return fmt.Errorf("injected candidate journal failure")
					}
					return write(path, transaction)
				}
			},
		},
		{
			name: "backup rename",
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
			name: "backup journal",
			configure: func(ops *encyclopediaPublicationOps) {
				write := ops.writeJournal
				ops.writeJournal = func(path string, transaction encyclopediaTransaction) error {
					if transaction.Phase == encyclopediaPhaseDestinationBackedUp {
						return fmt.Errorf("injected backup journal failure")
					}
					return write(path, transaction)
				}
			},
		},
		{
			name: "candidate rename",
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
		{
			name: "published journal",
			configure: func(ops *encyclopediaPublicationOps) {
				write := ops.writeJournal
				ops.writeJournal = func(path string, transaction encyclopediaTransaction) error {
					if transaction.Phase == encyclopediaPhaseCandidatePublished {
						return fmt.Errorf("injected published journal failure")
					}
					return write(path, transaction)
				}
			},
		},
		{
			name: "cleanup journal",
			configure: func(ops *encyclopediaPublicationOps) {
				write := ops.writeJournal
				ops.writeJournal = func(path string, transaction encyclopediaTransaction) error {
					if transaction.Phase == encyclopediaPhaseCleanupPending {
						return fmt.Errorf("injected cleanup journal failure")
					}
					return write(path, transaction)
				}
			},
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			parent := t.TempDir()
			output := filepath.Join(parent, "encyclopedia")
			if _, err := stageEncyclopediaDirectory(testEncyclopediaRequest(output, map[string]string{"source-report.json": "old"}, false)); err != nil {
				t.Fatal(err)
			}
			before := snapshotTestTree(t, parent)
			ops := defaultEncyclopediaPublicationOps()
			test.configure(&ops)
			request := testEncyclopediaRequest(output, map[string]string{"source-report.json": "new"}, true)
			if _, err := stageEncyclopediaDirectoryWithOps(request, ops); err == nil {
				t.Fatal("injected publication failure returned nil")
			}
			if got := string(readTestFile(t, filepath.Join(output, "source-report.json"))); got != "old" {
				t.Fatalf("previous bundle was not restored: %q", got)
			}
			if diff := diffTestTree(before, snapshotTestTree(t, parent)); diff != "" {
				t.Fatalf("recovered failure changed parent tree:\n%s", diff)
			}
		})
	}
}

func TestEncyclopediaConcurrentWritersNeverOverlap(t *testing.T) {
	output := filepath.Join(t.TempDir(), "encyclopedia")
	started := make(chan struct{})
	release := make(chan struct{})
	request := testEncyclopediaRequest(output, map[string]string{"source-report.json": "first"}, false)
	baseBuild := request.BuildCandidate
	request.BuildCandidate = func(root string) error {
		close(started)
		<-release
		return baseBuild(root)
	}

	var wg sync.WaitGroup
	wg.Add(1)
	firstResult := make(chan error, 1)
	go runTestEncyclopediaStage(request, firstResult, &wg)
	<-started

	second := testEncyclopediaRequest(output, map[string]string{"source-report.json": "second"}, true)
	if _, err := stageEncyclopediaDirectory(second); err == nil || !strings.Contains(err.Error(), "stage_busy") {
		t.Fatalf("concurrent writer error = %v", err)
	}
	close(release)
	wg.Wait()
	if err := <-firstResult; err != nil {
		t.Fatal(err)
	}
	if got := string(readTestFile(t, filepath.Join(output, "source-report.json"))); got != "first" {
		t.Fatalf("winner output = %q", got)
	}
}

func TestEncyclopediaConcurrentDeadMarkerReclamationNeverGrantsTwoLeases(t *testing.T) {
	marker := filepath.Join(t.TempDir(), "writer.lock")
	dead := encyclopediaWriterMarker{
		Version: encyclopediaTransactionVersion,
		PID:     1 << 30,
		Token:   testEncyclopediaMarkerToken(t),
	}
	encoded, err := json.Marshal(dead)
	if err != nil {
		t.Fatal(err)
	}
	for attempt := 0; attempt < 1000; attempt++ {
		if err := os.WriteFile(marker, encoded, 0o600); err != nil {
			t.Fatal(err)
		}
		start := make(chan struct{})
		done := make(chan testEncyclopediaWriterOutcome, 2)
		for worker := 0; worker < 2; worker++ {
			go runTestEncyclopediaWriterAcquire(marker, start, done)
		}
		close(start)
		first, second := <-done, <-done
		if first.err == nil && second.err == nil {
			releaseEncyclopediaWriter(first.lease)
			releaseEncyclopediaWriter(second.lease)
			t.Fatalf("attempt %d granted simultaneous leases %q and %q", attempt, first.lease.token, second.lease.token)
		}
		if first.err == nil {
			releaseEncyclopediaWriter(first.lease)
		}
		if second.err == nil {
			releaseEncyclopediaWriter(second.lease)
		}
		if err := os.Remove(marker); err != nil && !os.IsNotExist(err) {
			t.Fatal(err)
		}
	}
}

func TestEncyclopediaSeparateProcessWriterRemainsExclusiveWithoutDiagnosticMarker(t *testing.T) {
	marker := filepath.Join(t.TempDir(), "writer.lock")
	holder := startTestEncyclopediaWriterProcess(t, "hold", marker)
	stopped := false
	defer func() {
		if stopped {
			return
		}
		_ = holder.stdin.Close()
		_ = holder.command.Process.Kill()
		_ = holder.command.Wait()
	}()
	if err := os.Remove(marker); err != nil {
		t.Fatal(err)
	}

	if output := runTestEncyclopediaWriterAttempt(t, marker); !strings.Contains(output, "busy") || strings.Contains(output, "acquired") {
		t.Fatalf("separate-process contender output = %q", output)
	}
	if err := holder.stdin.Close(); err != nil {
		t.Fatal(err)
	}
	if err := holder.command.Wait(); err != nil {
		t.Fatalf("writer holder exit: %v (stderr %q)", err, holder.stderr.String())
	}
	stopped = true
}

func TestEncyclopediaAbruptWriterExitReleasesProcessLockForRecovery(t *testing.T) {
	marker := filepath.Join(t.TempDir(), "writer.lock")
	holder := startTestEncyclopediaWriterProcess(t, "exit", marker)
	_ = holder.stdin.Close()
	err := holder.command.Wait()
	exitError, ok := err.(*exec.ExitError)
	if !ok || exitError.ExitCode() != 23 {
		t.Fatalf("abrupt writer helper exit = %v (stderr %q)", err, holder.stderr.String())
	}

	lease, err := acquireEncyclopediaWriter(marker)
	if err != nil {
		t.Fatalf("recover after abrupt writer exit: %v (stderr %q)", err, holder.stderr.String())
	}
	releaseEncyclopediaWriter(lease)
}

func TestEncyclopediaDanglingWriterGuardSymlinkNeverCreatesTarget(t *testing.T) {
	parent := t.TempDir()
	target := filepath.Join(t.TempDir(), "must-not-be-created")
	output := filepath.Join(parent, "encyclopedia")
	paths, err := prepareEncyclopediaPublicationPaths(output)
	if err != nil {
		t.Fatal(err)
	}
	guard := paths.marker + encyclopediaWriterGuardSuffix
	if err := os.Symlink(target, guard); err != nil {
		t.Fatal(err)
	}

	if _, err := stageEncyclopediaDirectory(testEncyclopediaRequest(output, map[string]string{"source-report.json": "synthetic"}, false)); err == nil {
		t.Fatal("dangling writer guard symlink was accepted")
	}
	if _, err := os.Lstat(target); !os.IsNotExist(err) {
		t.Fatalf("refused writer guard created target: %v", err)
	}
	if got, err := os.Readlink(guard); err != nil || got != target {
		t.Fatalf("writer guard symlink = %q, %v; want %q", got, err, target)
	}
}

func TestEncyclopediaExistingWriterGuardSymlinkLeavesTargetUntouched(t *testing.T) {
	parent := t.TempDir()
	target := filepath.Join(t.TempDir(), "outside-file")
	want := []byte("user-owned outside bytes\x00\xff")
	if err := os.WriteFile(target, want, 0o640); err != nil {
		t.Fatal(err)
	}
	output := filepath.Join(parent, "encyclopedia")
	paths, err := prepareEncyclopediaPublicationPaths(output)
	if err != nil {
		t.Fatal(err)
	}
	guard := paths.marker + encyclopediaWriterGuardSuffix
	if err := os.Symlink(target, guard); err != nil {
		t.Fatal(err)
	}

	if _, err := stageEncyclopediaDirectory(testEncyclopediaRequest(output, map[string]string{"source-report.json": "synthetic"}, false)); err == nil {
		t.Fatal("existing writer guard symlink was accepted")
	}
	if got := readTestFile(t, target); !bytes.Equal(got, want) {
		t.Fatalf("outside target changed: got %x, want %x", got, want)
	}
	if got, err := os.Readlink(guard); err != nil || got != target {
		t.Fatalf("writer guard symlink = %q, %v; want %q", got, err, target)
	}
}

func TestEncyclopediaAllowsNestedSourceOutputButRejectsSourceAndModCollisions(t *testing.T) {
	root := t.TempDir()
	source := filepath.Join(root, "owned-install")
	mods := filepath.Join(root, "mods")
	if err := os.MkdirAll(source, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.MkdirAll(mods, 0o755); err != nil {
		t.Fatal(err)
	}

	nested := testEncyclopediaRequest(filepath.Join(source, "encyclopedia"), map[string]string{"source-report.json": "nested"}, false)
	nested.SourceRoots = []string{source}
	nested.ModRoots = []string{mods}
	if _, err := stageEncyclopediaDirectory(nested); err != nil {
		t.Fatalf("intended nested output was rejected: %v", err)
	}

	tests := []struct {
		name     string
		output   string
		sources  []string
		modRoots []string
	}{
		{name: "equals source", output: source, sources: []string{source}},
		{name: "contains source", output: root, sources: []string{source}},
		{name: "equals mods", output: mods, modRoots: []string{mods}},
		{name: "inside mods", output: filepath.Join(mods, "encyclopedia"), modRoots: []string{mods}},
		{name: "contains mods", output: root, modRoots: []string{mods}},
	}
	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			request := testEncyclopediaRequest(test.output, map[string]string{"source-report.json": "collision"}, true)
			request.SourceRoots = test.sources
			request.ModRoots = test.modRoots
			if _, err := stageEncyclopediaDirectory(request); err == nil || !strings.Contains(err.Error(), "path_collision") {
				t.Fatalf("collision error = %v", err)
			}
		})
	}
}

func TestEncyclopediaRejectsSymlinkEscapeAndCaseFoldedPathCollision(t *testing.T) {
	parent := t.TempDir()
	outside := t.TempDir()
	outputLink := filepath.Join(parent, "encyclopedia")
	if err := os.Symlink(outside, outputLink); err != nil {
		t.Fatal(err)
	}
	request := testEncyclopediaRequest(outputLink, map[string]string{"source-report.json": "escape"}, true)
	if _, err := stageEncyclopediaDirectory(request); err == nil || !strings.Contains(err.Error(), "symlink") {
		t.Fatalf("symlink output error = %v", err)
	}

	output := filepath.Join(parent, "safe-encyclopedia")
	request = testEncyclopediaRequest(output, map[string]string{}, false)
	request.BuildCandidate = func(root string) error {
		if err := os.WriteFile(filepath.Join(root, "FILE.txt"), []byte("one"), 0o644); err != nil {
			return err
		}
		if err := os.WriteFile(filepath.Join(root, "file.TXT"), []byte("two"), 0o644); err != nil {
			return err
		}
		ownership, _ := json.Marshal(testEncyclopediaOwnership{Files: []string{"FILE.txt", "file.TXT"}})
		return os.WriteFile(filepath.Join(root, testEncyclopediaOwnershipFile), ownership, 0o644)
	}
	request.InspectOwned = func(root string) (encyclopediaOwnedInventory, error) {
		return encyclopediaOwnedInventory{Files: []string{"FILE.txt", "file.TXT", testEncyclopediaOwnershipFile}}, nil
	}
	if _, err := stageEncyclopediaDirectory(request); err == nil || !strings.Contains(err.Error(), "path_collision") {
		t.Fatalf("case-folded collision error = %v", err)
	}

	request = testEncyclopediaRequest(filepath.Join(parent, "symlink-candidate"), map[string]string{}, false)
	request.BuildCandidate = func(root string) error {
		if err := os.Symlink(filepath.Join(outside, "secret"), filepath.Join(root, "escape.bin")); err != nil {
			return err
		}
		ownership, _ := json.Marshal(testEncyclopediaOwnership{Files: []string{"escape.bin", testEncyclopediaOwnershipFile}})
		return os.WriteFile(filepath.Join(root, testEncyclopediaOwnershipFile), ownership, 0o644)
	}
	if _, err := stageEncyclopediaDirectory(request); err == nil || !strings.Contains(err.Error(), "symlink") {
		t.Fatalf("candidate symlink error = %v", err)
	}
}

func snapshotTestTree(t *testing.T, root string) map[string]string {
	t.Helper()
	result := make(map[string]string)
	err := filepath.WalkDir(root, func(path string, entry os.DirEntry, err error) error {
		if err != nil {
			return err
		}
		rel, err := filepath.Rel(root, path)
		if err != nil {
			return err
		}
		if rel == "." {
			return nil
		}
		info, err := entry.Info()
		if err != nil {
			return err
		}
		switch {
		case info.Mode()&os.ModeSymlink != 0:
			target, err := os.Readlink(path)
			if err != nil {
				return err
			}
			result[filepath.ToSlash(rel)] = "symlink:" + target
		case info.IsDir():
			result[filepath.ToSlash(rel)] = "dir"
		case info.Mode().IsRegular():
			contents, err := os.ReadFile(path)
			if err != nil {
				return err
			}
			result[filepath.ToSlash(rel)] = fmt.Sprintf("file:%o:%x", info.Mode().Perm(), contents)
		default:
			result[filepath.ToSlash(rel)] = "other:" + info.Mode().String()
		}
		return nil
	})
	if err != nil {
		t.Fatal(err)
	}
	return result
}

func diffTestTree(before, after map[string]string) string {
	keys := make(map[string]struct{}, len(before)+len(after))
	for key := range before {
		keys[key] = struct{}{}
	}
	for key := range after {
		keys[key] = struct{}{}
	}
	ordered := make([]string, 0, len(keys))
	for key := range keys {
		ordered = append(ordered, key)
	}
	sort.Strings(ordered)
	var diff strings.Builder
	for _, key := range ordered {
		if before[key] != after[key] {
			fmt.Fprintf(&diff, "%s: %q -> %q\n", key, before[key], after[key])
		}
	}
	return diff.String()
}
