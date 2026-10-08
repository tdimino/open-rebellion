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

func TestRunCLIStageOnlySkipsTheVerificationPass(t *testing.T) {
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
	extra := filepath.Join(outputDir, "test-dll", "BMP", "999.bmp")
	if err := os.MkdirAll(filepath.Dir(extra), 0o700); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(extra, []byte("deliberate extra file"), 0o600); err != nil {
		t.Fatal(err)
	}
	var stdout, stderr bytes.Buffer

	err := runTestCLI(
		[]string{
			"--no-verify",
			"--source", sourceDir,
			"--output", outputDir,
			"--audio-output", filepath.Join(outputDir, "sounds"),
		},
		&stdout,
		&stderr,
		[]dllTarget{{Filename: "TEST.DLL", Directory: "test-dll", Expected: 1}},
	)
	if err != nil {
		t.Fatalf("stage-only CLI error = %v; stderr = %s", err, stderr.String())
	}
	if strings.Contains(stdout.String(), "Verified") {
		t.Fatalf("stage-only output unexpectedly ran verification: %q", stdout.String())
	}
	if _, err := os.Stat(filepath.Join(outputDir, "test-dll", "BMP", "88.bmp")); err != nil {
		t.Fatalf("staged runtime asset: %v", err)
	}

	stdout.Reset()
	err = runTestCLI(
		[]string{
			"--verify",
			"--source", filepath.Join(sourceDir, "missing"),
			"--output", outputDir,
			"--audio-output", filepath.Join(outputDir, "sounds"),
		},
		&stdout,
		&stderr,
		[]dllTarget{{Filename: "TEST.DLL", Directory: "test-dll", Expected: 1}},
	)
	if err == nil {
		t.Fatal("verification unexpectedly accepted the deliberate extra file")
	}
}

func TestRunCLIRejectsStageOnlyVerification(t *testing.T) {
	err := runCLIWithMedia(
		[]string{"--no-verify", "--verify"},
		io.Discard,
		io.Discard,
		nil,
		nil,
		nil,
	)
	if err == nil {
		t.Fatal("--no-verify and --verify were accepted together")
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
