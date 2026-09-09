package main

import (
	"bytes"
	"encoding/binary"
	"io"
	"os"
	"path/filepath"
	"testing"
)

func TestStageBitmapResourcesWritesNumericBMPFilename(t *testing.T) {
	dib := make([]byte, 44)
	binary.LittleEndian.PutUint32(dib[0:4], 40)
	binary.LittleEndian.PutUint16(dib[12:14], 1)
	binary.LittleEndian.PutUint16(dib[14:16], 24)
	outputDir := filepath.Join(t.TempDir(), "BMP")

	result, err := stageBitmapResources([]bitmapResource{{ID: 4242, Language: 1033, DIB: dib}}, outputDir, false)
	if err != nil {
		t.Fatalf("stageBitmapResources() error = %v", err)
	}
	if result.Written != 1 || result.Skipped != 0 {
		t.Errorf("result = %+v, want 1 written and 0 skipped", result)
	}
	bmp, err := os.ReadFile(filepath.Join(outputDir, "4242.bmp"))
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(bmp[14:], dib) {
		t.Error("staged BMP does not contain the source DIB")
	}
}

func TestStageBitmapResourcesSkipsIdenticalExistingFile(t *testing.T) {
	dib := make([]byte, 44)
	binary.LittleEndian.PutUint32(dib[0:4], 40)
	binary.LittleEndian.PutUint16(dib[12:14], 1)
	binary.LittleEndian.PutUint16(dib[14:16], 24)
	resources := []bitmapResource{{ID: 7, Language: 1033, DIB: dib}}
	outputDir := filepath.Join(t.TempDir(), "BMP")

	if _, err := stageBitmapResources(resources, outputDir, false); err != nil {
		t.Fatal(err)
	}
	result, err := stageBitmapResources(resources, outputDir, false)
	if err != nil {
		t.Fatal(err)
	}
	if result.Written != 0 || result.Skipped != 1 {
		t.Errorf("second staging result = %+v, want 0 written and 1 skipped", result)
	}
}

func TestStageBitmapResourcesRejectsDuplicateResourceID(t *testing.T) {
	dib := make([]byte, 44)
	binary.LittleEndian.PutUint32(dib[0:4], 40)
	binary.LittleEndian.PutUint16(dib[12:14], 1)
	binary.LittleEndian.PutUint16(dib[14:16], 24)
	resources := []bitmapResource{
		{ID: 9, Language: 1033, DIB: dib},
		{ID: 9, Language: 1031, DIB: dib},
	}

	_, err := stageBitmapResources(resources, filepath.Join(t.TempDir(), "BMP"), false)
	if err == nil {
		t.Fatal("stageBitmapResources() error = nil, want duplicate ID error")
	}
}

func TestStageBitmapResourcesPreservesDifferentExistingFileWithoutForce(t *testing.T) {
	dib := make([]byte, 44)
	binary.LittleEndian.PutUint32(dib[0:4], 40)
	binary.LittleEndian.PutUint16(dib[12:14], 1)
	binary.LittleEndian.PutUint16(dib[14:16], 24)
	outputDir := filepath.Join(t.TempDir(), "BMP")
	if err := os.MkdirAll(outputDir, 0o755); err != nil {
		t.Fatal(err)
	}
	path := filepath.Join(outputDir, "12.bmp")
	original := []byte("existing")
	if err := os.WriteFile(path, original, 0o644); err != nil {
		t.Fatal(err)
	}

	_, err := stageBitmapResources([]bitmapResource{{ID: 12, Language: 1033, DIB: dib}}, outputDir, false)
	if err == nil {
		t.Fatal("stageBitmapResources() error = nil, want existing-file conflict")
	}
	got, readErr := os.ReadFile(path)
	if readErr != nil {
		t.Fatal(readErr)
	}
	if !bytes.Equal(got, original) {
		t.Fatalf("existing file was changed to %q", got)
	}
}

func TestStageBitmapResourcesReplacesDifferentExistingFileWithForce(t *testing.T) {
	dib := make([]byte, 44)
	binary.LittleEndian.PutUint32(dib[0:4], 40)
	binary.LittleEndian.PutUint16(dib[12:14], 1)
	binary.LittleEndian.PutUint16(dib[14:16], 24)
	outputDir := filepath.Join(t.TempDir(), "BMP")
	if err := os.MkdirAll(outputDir, 0o755); err != nil {
		t.Fatal(err)
	}
	path := filepath.Join(outputDir, "12.bmp")
	if err := os.WriteFile(path, []byte("existing"), 0o644); err != nil {
		t.Fatal(err)
	}

	result, err := stageBitmapResources([]bitmapResource{{ID: 12, Language: 1033, DIB: dib}}, outputDir, true)
	if err != nil {
		t.Fatalf("stageBitmapResources() error = %v", err)
	}
	if result.Written != 1 {
		t.Errorf("written = %d, want 1", result.Written)
	}
	got, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	if bytes.Equal(got, []byte("existing")) || !bytes.Equal(got[14:], dib) {
		t.Error("existing file was not replaced with converted BMP")
	}
}

func TestWriteFileAtomicallyReplacesDestinationWithoutTemporaryFiles(t *testing.T) {
	dir := t.TempDir()
	path := filepath.Join(dir, "42.bmp")
	if err := os.WriteFile(path, []byte("old"), 0o644); err != nil {
		t.Fatal(err)
	}

	if err := writeFileAtomically(path, []byte("new"), 0o644); err != nil {
		t.Fatalf("writeFileAtomically() error = %v", err)
	}
	got, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	if string(got) != "new" {
		t.Errorf("contents = %q, want new", got)
	}
	entries, err := os.ReadDir(dir)
	if err != nil {
		t.Fatal(err)
	}
	if len(entries) != 1 || entries[0].Name() != "42.bmp" {
		t.Fatalf("directory entries = %v, want only 42.bmp", entries)
	}
}

func TestStageTargetsExtractsDLLIntoRuntimeLayout(t *testing.T) {
	sourceDir := t.TempDir()
	outputDir := t.TempDir()
	dib := make([]byte, 44)
	binary.LittleEndian.PutUint32(dib[0:4], 40)
	binary.LittleEndian.PutUint16(dib[12:14], 1)
	binary.LittleEndian.PutUint16(dib[14:16], 24)
	if err := os.WriteFile(filepath.Join(sourceDir, "TEST.DLL"), buildTestPE32WithBitmap(t, 77, 1033, dib), 0o600); err != nil {
		t.Fatal(err)
	}

	summary, err := stageTargets(sourceDir, outputDir, []dllTarget{{Filename: "TEST.DLL", Directory: "test-dll", Expected: 1}}, nil, false, io.Discard)
	if err != nil {
		t.Fatalf("stageTargets() error = %v", err)
	}
	if summary.DLLs != 1 || summary.Resources != 1 || summary.Written != 1 {
		t.Errorf("summary = %+v, want one DLL/resource/write", summary)
	}
	if _, err := os.Stat(filepath.Join(outputDir, "test-dll", "BMP", "77.bmp")); err != nil {
		t.Fatalf("staged runtime asset: %v", err)
	}
}
