package main

import (
	"bytes"
	"encoding/binary"
	"io"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestRunCLIStagesAndVerifiesConfiguredTargets(t *testing.T) {
	sourceDir := t.TempDir()
	outputDir := t.TempDir()
	dib := make([]byte, 44)
	binary.LittleEndian.PutUint32(dib[0:4], 40)
	binary.LittleEndian.PutUint16(dib[12:14], 1)
	binary.LittleEndian.PutUint16(dib[14:16], 24)
	if err := os.WriteFile(filepath.Join(sourceDir, "TEST.DLL"), buildTestPE32WithBitmap(t, 88, 1033, dib), 0o600); err != nil {
		t.Fatal(err)
	}
	writeAudioFixture(t, sourceDir)
	var stdout, stderr bytes.Buffer

	err := runTestCLI(
		[]string{"--source", sourceDir, "--output", outputDir, "--audio-output", filepath.Join(outputDir, "sounds")},
		&stdout,
		&stderr,
		[]dllTarget{{Filename: "TEST.DLL", Directory: "test-dll", Expected: 1}},
	)
	if err != nil {
		t.Fatalf("runTestCLI() error = %v; stderr = %s", err, stderr.String())
	}
	if _, err := os.Stat(filepath.Join(outputDir, "test-dll", "BMP", "88.bmp")); err != nil {
		t.Fatalf("staged runtime asset: %v", err)
	}
	if !bytes.Contains(stdout.Bytes(), []byte("Verified 1 UI resources")) {
		t.Errorf("stdout = %q, want verification summary", stdout.String())
	}
}

func TestRunCLITactical3DConvertMode(t *testing.T) {
	outputDir := t.TempDir()
	writeSyntheticTacticalRawStore(t, outputDir)
	var stdout, stderr bytes.Buffer
	if err := runCLIWithMedia(
		[]string{"--tactical-3d-convert", "--output", outputDir},
		&stdout,
		&stderr,
		nil,
		nil,
		nil,
	); err != nil {
		t.Fatalf("convert CLI error = %v; stderr = %s", err, stderr.String())
	}
	if !bytes.Contains(stdout.Bytes(), []byte("Verified tactical runtime pack")) {
		t.Fatalf("stdout = %q", stdout.String())
	}
	stdout.Reset()
	if err := runCLIWithMedia(
		[]string{"--tactical-3d-convert", "--verify", "--output", outputDir},
		&stdout,
		&stderr,
		nil,
		nil,
		nil,
	); err != nil {
		t.Fatalf("verify CLI error = %v; stderr = %s", err, stderr.String())
	}
}

func TestRunCLIRejectsMultipleTacticalModes(t *testing.T) {
	err := runCLIWithMedia(
		[]string{"--tactical-3d-only", "--tactical-3d-convert"},
		io.Discard,
		io.Discard,
		nil,
		nil,
		nil,
	)
	if err == nil {
		t.Fatal("multiple tactical modes were accepted")
	}
}

func TestEncyclopediaFocusedModeNeverInvokesMediaTools(t *testing.T) {
	source := t.TempDir()
	output := filepath.Join(t.TempDir(), "research")
	writeSyntheticEncyclopediaTextDLL(t, source, numericEncyclopediaResourceIdentifier(23), []byte("focused report\x00"))
	mediaCalls := 0
	failingMedia := func(string, ...string) ([]byte, error) {
		mediaCalls++
		return nil, io.ErrUnexpectedEOF
	}
	var stdout, stderr bytes.Buffer
	if err := runCLIWithMedia([]string{
		"--encyclopedia-report-only",
		"--source", source,
		"--encyclopedia-report-output", output,
	}, &stdout, &stderr, nil, nil, failingMedia); err != nil {
		t.Fatalf("focused report CLI error = %v; stderr = %s", err, stderr.String())
	}
	if mediaCalls != 0 {
		t.Fatalf("focused report invoked media tools %d times", mediaCalls)
	}
	if err := runCLIWithMedia([]string{
		"--encyclopedia-report-only",
		"--verify",
		"--source", filepath.Join(t.TempDir(), "missing-install"),
		"--encyclopedia-report-output", output,
	}, &stdout, &stderr, nil, nil, failingMedia); err != nil {
		t.Fatalf("source-free report verify CLI error = %v; stderr = %s", err, stderr.String())
	}
	if mediaCalls != 0 {
		t.Fatalf("report verification invoked media tools %d times", mediaCalls)
	}
}

func TestEncyclopediaReportOutputFlagRequiresFocusedMode(t *testing.T) {
	mediaCalls := 0
	err := runCLIWithMedia(
		[]string{"--encyclopedia-report-output", filepath.Join(t.TempDir(), "research")},
		io.Discard,
		io.Discard,
		nil,
		nil,
		func(string, ...string) ([]byte, error) {
			mediaCalls++
			return nil, io.ErrUnexpectedEOF
		},
	)
	if err == nil || !strings.Contains(err.Error(), "--encyclopedia-report-only") {
		t.Fatalf("cross-mode encyclopedia output error = %v", err)
	}
	if mediaCalls != 0 {
		t.Fatalf("cross-mode rejection invoked media tools %d times", mediaCalls)
	}
}

func TestEncyclopediaFocusedModeUsesDefaultResearchOutput(t *testing.T) {
	originalWorkingDirectory, err := os.Getwd()
	if err != nil {
		t.Fatal(err)
	}
	workingDirectory := t.TempDir()
	if err := os.Chdir(workingDirectory); err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() {
		if err := os.Chdir(originalWorkingDirectory); err != nil {
			t.Errorf("restore working directory: %v", err)
		}
	})
	source := t.TempDir()
	writeSyntheticEncyclopediaTextDLL(t, source, numericEncyclopediaResourceIdentifier(41), []byte("default output\x00"))
	if err := runCLIWithMedia(
		[]string{"--encyclopedia-report-only", "--source", source},
		io.Discard,
		io.Discard,
		nil,
		nil,
		func(string, ...string) ([]byte, error) { return nil, io.ErrUnexpectedEOF },
	); err != nil {
		t.Fatal(err)
	}
	want := filepath.Join(workingDirectory, "data", "base", "encyclopedia-research", encyclopediaResearchReportFilename)
	if _, err := os.Stat(want); err != nil {
		t.Fatalf("default encyclopedia report output: %v", err)
	}
}
