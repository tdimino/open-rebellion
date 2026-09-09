package main

import (
	"encoding/binary"
	"io"
	"os"
	"path/filepath"
	"testing"
)

func TestVerifyTargetsAcceptsCompleteRuntimeLayout(t *testing.T) {
	outputDir := t.TempDir()
	dib := make([]byte, 44)
	binary.LittleEndian.PutUint32(dib[0:4], 40)
	binary.LittleEndian.PutUint16(dib[12:14], 1)
	binary.LittleEndian.PutUint16(dib[14:16], 24)
	bmp, err := dibToBMP(dib)
	if err != nil {
		t.Fatal(err)
	}
	bmpDir := filepath.Join(outputDir, "test-dll", "BMP")
	if err := os.MkdirAll(bmpDir, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(bmpDir, "77.bmp"), bmp, 0o644); err != nil {
		t.Fatal(err)
	}

	summary, err := verifyTargets(outputDir, []dllTarget{{Directory: "test-dll", Expected: 1}}, io.Discard)
	if err != nil {
		t.Fatalf("verifyTargets() error = %v", err)
	}
	if summary.DLLs != 1 || summary.Resources != 1 {
		t.Errorf("summary = %+v, want one DLL/resource", summary)
	}
}
