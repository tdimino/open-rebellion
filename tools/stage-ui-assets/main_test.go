package main

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"testing"
	"unicode/utf16"
)

func TestDIBToBMPAddsFileHeaderAndPaletteOffset(t *testing.T) {
	dib := make([]byte, 40+8+4)
	binary.LittleEndian.PutUint32(dib[0:4], 40) // BITMAPINFOHEADER
	binary.LittleEndian.PutUint32(dib[4:8], 1)  // width
	binary.LittleEndian.PutUint32(dib[8:12], 1) // height
	binary.LittleEndian.PutUint16(dib[12:14], 1)
	binary.LittleEndian.PutUint16(dib[14:16], 8) // bits per pixel
	binary.LittleEndian.PutUint32(dib[32:36], 2) // palette entries

	bmp, err := dibToBMP(dib)
	if err != nil {
		t.Fatalf("dibToBMP() error = %v", err)
	}
	if got := string(bmp[:2]); got != "BM" {
		t.Fatalf("signature = %q, want BM", got)
	}
	if got, want := binary.LittleEndian.Uint32(bmp[2:6]), uint32(len(dib)+14); got != want {
		t.Errorf("file size = %d, want %d", got, want)
	}
	if got, want := binary.LittleEndian.Uint32(bmp[10:14]), uint32(14+40+8); got != want {
		t.Errorf("pixel offset = %d, want %d", got, want)
	}
}

func TestParseBitmapResourcesWalksTypeIDAndLanguageDirectories(t *testing.T) {
	const resourceRVA = uint32(0x2000)
	dib := make([]byte, 44)
	binary.LittleEndian.PutUint32(dib[0:4], 40)
	binary.LittleEndian.PutUint16(dib[12:14], 1)
	binary.LittleEndian.PutUint16(dib[14:16], 24)

	section := make([]byte, 0x80+len(dib))
	putResourceDirectory(section, 0x00, 0, 1)
	putResourceEntry(section, 0x10, 2, 0x80000020) // RT_BITMAP
	putResourceDirectory(section, 0x20, 0, 1)
	putResourceEntry(section, 0x30, 100, 0x80000040)
	putResourceDirectory(section, 0x40, 0, 1)
	putResourceEntry(section, 0x50, 1033, 0x60)
	binary.LittleEndian.PutUint32(section[0x60:0x64], resourceRVA+0x80)
	binary.LittleEndian.PutUint32(section[0x64:0x68], uint32(len(dib)))
	copy(section[0x80:], dib)

	resolve := func(rva, size uint32) ([]byte, error) {
		if rva < resourceRVA || uint64(rva-resourceRVA)+uint64(size) > uint64(len(section)) {
			return nil, fmt.Errorf("RVA outside test section")
		}
		start := rva - resourceRVA
		return section[start : start+size], nil
	}

	resources, err := parseBitmapResources(section, resolve)
	if err != nil {
		t.Fatalf("parseBitmapResources() error = %v", err)
	}
	if len(resources) != 1 {
		t.Fatalf("resource count = %d, want 1", len(resources))
	}
	if resources[0].ID != 100 || resources[0].Language != 1033 {
		t.Errorf("resource identity = (%d, %d), want (100, 1033)", resources[0].ID, resources[0].Language)
	}
	if !bytes.Equal(resources[0].DIB, dib) {
		t.Error("resource DIB does not match source bytes")
	}
}

func TestParseBitmapResourcesMapsKnownNamedBitmapToCatalogID(t *testing.T) {
	const resourceRVA = uint32(0x2000)
	const nameOffset = uint32(0x70)
	const dibOffset = uint32(0xc0)
	name := "COCKPIT_BUTTON_GAMESCALE_HUGE_UP"
	dib := make([]byte, 44)
	binary.LittleEndian.PutUint32(dib[0:4], 40)
	binary.LittleEndian.PutUint16(dib[12:14], 1)
	binary.LittleEndian.PutUint16(dib[14:16], 24)

	section := make([]byte, int(dibOffset)+len(dib))
	putResourceDirectory(section, 0x00, 0, 1)
	putResourceEntry(section, 0x10, 2, 0x80000020)
	putResourceDirectory(section, 0x20, 1, 0)
	putResourceEntry(section, 0x30, resourceSubdirectory|nameOffset, 0x80000040)
	putResourceDirectory(section, 0x40, 0, 1)
	putResourceEntry(section, 0x50, 1033, 0x60)
	binary.LittleEndian.PutUint32(section[0x60:0x64], resourceRVA+dibOffset)
	binary.LittleEndian.PutUint32(section[0x64:0x68], uint32(len(dib)))
	encodedName := utf16.Encode([]rune(name))
	binary.LittleEndian.PutUint16(section[nameOffset:nameOffset+2], uint16(len(encodedName)))
	for i, codeUnit := range encodedName {
		start := int(nameOffset) + 2 + i*2
		binary.LittleEndian.PutUint16(section[start:start+2], codeUnit)
	}
	copy(section[dibOffset:], dib)

	resolve := func(rva, size uint32) ([]byte, error) {
		start := rva - resourceRVA
		return section[start : start+size], nil
	}
	resources, err := parseBitmapResources(section, resolve)
	if err != nil {
		t.Fatalf("parseBitmapResources() error = %v", err)
	}
	if len(resources) != 1 || resources[0].ID != 15856 {
		t.Fatalf("resources = %+v, want named bitmap mapped to ID 15856", resources)
	}
}

func TestBitmapResourceIDMapsKnownTacticalNamesToCatalogIDs(t *testing.T) {
	tests := []struct {
		name string
		id   uint32
	}{
		{name: "DATA_BUTTON_UP_FIGHTERGROUP_RECOVER", id: 40720},
		{name: "DATA_BUTTON_DN_FIGHTERGROUP_RECOVER", id: 40792},
		{name: "DATA_BUTTON_UP_FIGHTERGROUP_TACTICS", id: 40864},
		{name: "DATA_BUTTON_DN_FIGHTERGROUP_TACTICS", id: 40936},
	}
	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			encoded := utf16.Encode([]rune(test.name))
			resourceData := make([]byte, 2+len(encoded)*2)
			binary.LittleEndian.PutUint16(resourceData[0:2], uint16(len(encoded)))
			for i, codeUnit := range encoded {
				binary.LittleEndian.PutUint16(resourceData[2+i*2:4+i*2], codeUnit)
			}
			id, err := bitmapResourceID(resourceData, resourceSubdirectory)
			if err != nil {
				t.Fatalf("bitmapResourceID() error = %v", err)
			}
			if id != test.id {
				t.Errorf("ID = %d, want %d", id, test.id)
			}
		})
	}
}

func putResourceDirectory(dst []byte, offset int, named, ids uint16) {
	binary.LittleEndian.PutUint16(dst[offset+12:offset+14], named)
	binary.LittleEndian.PutUint16(dst[offset+14:offset+16], ids)
}

func putResourceEntry(dst []byte, offset int, name, target uint32) {
	binary.LittleEndian.PutUint32(dst[offset:offset+4], name)
	binary.LittleEndian.PutUint32(dst[offset+4:offset+8], target)
}

func TestReadPEBitmapResourcesReadsResourceDataDirectory(t *testing.T) {
	dib := make([]byte, 44)
	binary.LittleEndian.PutUint32(dib[0:4], 40)
	binary.LittleEndian.PutUint16(dib[12:14], 1)
	binary.LittleEndian.PutUint16(dib[14:16], 24)

	dll := buildTestPE32WithBitmap(t, 4242, 1033, dib)
	path := filepath.Join(t.TempDir(), "TEST.DLL")
	if err := os.WriteFile(path, dll, 0o600); err != nil {
		t.Fatal(err)
	}

	resources, err := readPEBitmapResources(path)
	if err != nil {
		t.Fatalf("readPEBitmapResources() error = %v", err)
	}
	if len(resources) != 1 {
		t.Fatalf("resource count = %d, want 1", len(resources))
	}
	if resources[0].ID != 4242 || resources[0].Language != 1033 {
		t.Errorf("resource identity = (%d, %d), want (4242, 1033)", resources[0].ID, resources[0].Language)
	}
	if !bytes.Equal(resources[0].DIB, dib) {
		t.Error("resource DIB does not match source bytes")
	}
}

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

	summary, err := stageTargets(sourceDir, outputDir, []dllTarget{{Filename: "TEST.DLL", Directory: "test-dll", Expected: 1}}, false, io.Discard)
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

func TestValidateBMPRejectsIncorrectDeclaredFileSize(t *testing.T) {
	dib := make([]byte, 44)
	binary.LittleEndian.PutUint32(dib[0:4], 40)
	binary.LittleEndian.PutUint16(dib[12:14], 1)
	binary.LittleEndian.PutUint16(dib[14:16], 24)
	bmp, err := dibToBMP(dib)
	if err != nil {
		t.Fatal(err)
	}
	binary.LittleEndian.PutUint32(bmp[2:6], uint32(len(bmp)+1))

	if err := validateBMP(bmp); err == nil {
		t.Fatal("validateBMP() error = nil, want file-size error")
	}
}

func TestValidateBMPRejectsIncorrectSignature(t *testing.T) {
	bmp := make([]byte, 54)
	copy(bmp[:2], "ZZ")
	binary.LittleEndian.PutUint32(bmp[2:6], uint32(len(bmp)))
	binary.LittleEndian.PutUint32(bmp[10:14], 54)
	binary.LittleEndian.PutUint32(bmp[14:18], 40)

	if err := validateBMP(bmp); err == nil {
		t.Fatal("validateBMP() error = nil, want signature error")
	}
}

func TestValidateBMPRejectsPixelOffsetBeforeDIBHeaderEnd(t *testing.T) {
	bmp := make([]byte, 54)
	copy(bmp[:2], "BM")
	binary.LittleEndian.PutUint32(bmp[2:6], uint32(len(bmp)))
	binary.LittleEndian.PutUint32(bmp[10:14], 20)
	binary.LittleEndian.PutUint32(bmp[14:18], 40)

	if err := validateBMP(bmp); err == nil {
		t.Fatal("validateBMP() error = nil, want pixel-offset error")
	}
}

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

func buildTestPE32WithBitmap(t *testing.T, id, language uint32, dib []byte) []byte {
	t.Helper()
	const (
		peOffset      = 0x80
		optionalSize  = 224
		sectionRVA    = 0x1000
		sectionOffset = 0x200
		sectionSize   = 0x200
	)

	resource := make([]byte, sectionSize)
	putResourceDirectory(resource, 0x00, 0, 1)
	putResourceEntry(resource, 0x10, 2, 0x80000020)
	putResourceDirectory(resource, 0x20, 0, 1)
	putResourceEntry(resource, 0x30, id, 0x80000040)
	putResourceDirectory(resource, 0x40, 0, 1)
	putResourceEntry(resource, 0x50, language, 0x60)
	binary.LittleEndian.PutUint32(resource[0x60:0x64], sectionRVA+0x80)
	binary.LittleEndian.PutUint32(resource[0x64:0x68], uint32(len(dib)))
	copy(resource[0x80:], dib)

	file := make([]byte, sectionOffset+sectionSize)
	copy(file[0:2], "MZ")
	binary.LittleEndian.PutUint32(file[0x3c:0x40], peOffset)
	copy(file[peOffset:peOffset+4], "PE\x00\x00")
	coff := peOffset + 4
	binary.LittleEndian.PutUint16(file[coff:coff+2], 0x14c)
	binary.LittleEndian.PutUint16(file[coff+2:coff+4], 1)
	binary.LittleEndian.PutUint16(file[coff+16:coff+18], optionalSize)
	binary.LittleEndian.PutUint16(file[coff+18:coff+20], 0x2102)

	optional := coff + 20
	binary.LittleEndian.PutUint16(file[optional:optional+2], 0x10b)
	binary.LittleEndian.PutUint32(file[optional+32:optional+36], 0x1000)
	binary.LittleEndian.PutUint32(file[optional+36:optional+40], 0x200)
	binary.LittleEndian.PutUint32(file[optional+56:optional+60], 0x2000)
	binary.LittleEndian.PutUint32(file[optional+60:optional+64], 0x200)
	binary.LittleEndian.PutUint32(file[optional+92:optional+96], 16)
	resourceDirectory := optional + 96 + 2*8
	binary.LittleEndian.PutUint32(file[resourceDirectory:resourceDirectory+4], sectionRVA)
	binary.LittleEndian.PutUint32(file[resourceDirectory+4:resourceDirectory+8], sectionSize)

	section := optional + optionalSize
	copy(file[section:section+8], ".rsrc")
	binary.LittleEndian.PutUint32(file[section+8:section+12], sectionSize)
	binary.LittleEndian.PutUint32(file[section+12:section+16], sectionRVA)
	binary.LittleEndian.PutUint32(file[section+16:section+20], sectionSize)
	binary.LittleEndian.PutUint32(file[section+20:section+24], sectionOffset)
	binary.LittleEndian.PutUint32(file[section+36:section+40], 0x40000040)
	copy(file[sectionOffset:], resource)
	return file
}
