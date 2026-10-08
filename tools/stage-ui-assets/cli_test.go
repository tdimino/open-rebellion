package main

import (
	"bytes"
	"encoding/binary"
	"fmt"
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
		t,
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
	if _, err := os.Stat(filepath.Join(outputDir, "encyclopedia", "source.json")); err != nil {
		t.Fatalf("staged Encyclopedia source: %v", err)
	}
	if !bytes.Contains(stdout.Bytes(), []byte("Verified 1 UI resources")) {
		t.Errorf("stdout = %q, want verification summary", stdout.String())
	}
	if !bytes.Contains(stdout.Bytes(), []byte("Verified 348 ENCYTEXT topics and 191 ENCYBMAP mappings")) {
		t.Errorf("stdout = %q, want Encyclopedia verification summary", stdout.String())
	}
}

func writeCompleteEncyclopediaFixture(t *testing.T, source string) {
	t.Helper()
	texts := make([]rawResource, encyclopediaExpectedTexts)
	for index := range texts {
		id := uint32(index + 1)
		texts[index] = rawResource{
			ID:       id,
			Language: encyclopediaLanguageID,
			Data:     []byte(fmt.Sprintf("Synthetic topic %d\x00", id)),
		}
	}
	if err := os.WriteFile(
		filepath.Join(source, "ENCYTEXT.DLL"),
		buildTestPE32WithResources(t, 10, texts),
		0o600,
	); err != nil {
		t.Fatal(err)
	}

	bundles := make(map[uint32]map[int]string)
	for id := 1; id <= encyclopediaExpectedMaps; id++ {
		bundleID := uint32(id/16 + 1)
		slot := id % 16
		if bundles[bundleID] == nil {
			bundles[bundleID] = make(map[int]string)
		}
		bundles[bundleID][slot] = fmt.Sprintf("EDATA.%03d", id)
	}
	artwork := make([]rawResource, 0, len(bundles))
	for bundleID := uint32(1); bundleID <= uint32(len(bundles)); bundleID++ {
		artwork = append(artwork, rawResource{
			ID:       bundleID,
			Language: encyclopediaLanguageID,
			Data:     stringBundle(bundles[bundleID]),
		})
	}
	if err := os.WriteFile(
		filepath.Join(source, "ENCYBMAP.DLL"),
		buildTestPE32WithResources(t, 6, artwork),
		0o600,
	); err != nil {
		t.Fatal(err)
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
		t,
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
		t,
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
