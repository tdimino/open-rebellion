package main

import (
	"bytes"
	"encoding/binary"
	"os"
	"path/filepath"
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
	var stdout, stderr bytes.Buffer

	err := runCLI(
		[]string{"--source", sourceDir, "--output", outputDir},
		&stdout,
		&stderr,
		[]dllTarget{{Filename: "TEST.DLL", Directory: "test-dll", Expected: 1}},
	)
	if err != nil {
		t.Fatalf("runCLI() error = %v; stderr = %s", err, stderr.String())
	}
	if _, err := os.Stat(filepath.Join(outputDir, "test-dll", "BMP", "88.bmp")); err != nil {
		t.Fatalf("staged runtime asset: %v", err)
	}
	if !bytes.Contains(stdout.Bytes(), []byte("Verified 1 BMPs")) {
		t.Errorf("stdout = %q, want verification summary", stdout.String())
	}
}
