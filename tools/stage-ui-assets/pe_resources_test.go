package main

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"os"
	"path/filepath"
	"testing"
)

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

	resources, err := parseBitmapResources(section, resolve, nil)
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

	resources, err := readPEBitmapResources(path, nil)
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

func TestBitmapResourceIDUsesSuppliedMapping(t *testing.T) {
	// A one-character UTF-16 resource name, independent of the game catalog.
	data := []byte{1, 0, 'X', 0}
	id, err := bitmapResourceID(data, resourceSubdirectory, map[string]uint32{"X": 123})
	if err != nil || id != 123 {
		t.Fatalf("bitmapResourceID() = (%d, %v), want (123, nil)", id, err)
	}
	if _, err := bitmapResourceID(data, resourceSubdirectory, nil); err == nil {
		t.Fatal("unmapped name accepted")
	}
}
