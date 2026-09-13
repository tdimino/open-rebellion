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

func TestReadPERawResourcesPreservesType302Bytes(t *testing.T) {
	frame := validType302Fixture()
	dll := buildTestPE32WithResource(t, rtAdvisorFrame, 2002, 1033, frame)
	path := filepath.Join(t.TempDir(), "TEST.DLL")
	if err := os.WriteFile(path, dll, 0o600); err != nil {
		t.Fatal(err)
	}

	resources, err := readPERawResources(path, rtAdvisorFrame)
	if err != nil {
		t.Fatalf("readPERawResources() error = %v", err)
	}
	if len(resources) != 1 {
		t.Fatalf("resource count = %d, want 1", len(resources))
	}
	if resources[0].ID != 2002 || resources[0].Language != 1033 {
		t.Errorf("resource identity = (%d, %d), want (2002, 1033)", resources[0].ID, resources[0].Language)
	}
	if !bytes.Equal(resources[0].Data, frame) {
		t.Error("type-302 resource does not match source bytes")
	}
}

func TestReadPEMixedRawResourcesPreservesNamedIdentifier(t *testing.T) {
	data := []byte("named tactical texture")
	dll := buildTestPE32WithNamedResource(t, tacticalTextureResourceType, "EMPIRE_TEST_CLOSE.BMP", 1033, data)
	binary.LittleEndian.PutUint32(dll[0x268:0x26c], 1252)
	binary.LittleEndian.PutUint32(dll[0x26c:0x270], 7)
	path := filepath.Join(t.TempDir(), "TEST.DLL")
	if err := os.WriteFile(path, dll, 0o600); err != nil {
		t.Fatal(err)
	}

	source, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	resources, err := readPEMixedRawResourcesFromBytes(source, tacticalTextureResourceType, rawResourceLimits{
		MaxCount:          1,
		MaxResourceBytes:  1024,
		MaxAggregateBytes: 1024,
	})
	if err != nil {
		t.Fatalf("readPEMixedRawResources() error = %v", err)
	}
	if len(resources) != 1 {
		t.Fatalf("resource count = %d, want 1", len(resources))
	}
	resource := resources[0]
	if !resource.Named || resource.Name != "EMPIRE_TEST_CLOSE.BMP" || resource.Language != 1033 || resource.CodePage != 1252 || resource.Reserved != 7 {
		t.Errorf("resource identity = %+v", resource)
	}
	if !bytes.Equal(resource.Data, data) {
		t.Error("named resource does not match source bytes")
	}
	if _, err := readPERawResources(path, tacticalTextureResourceType); err == nil {
		t.Fatal("strict numeric raw-resource reader accepted a named entry")
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

func TestResourceNameRejectsMalformedUTF16(t *testing.T) {
	for _, codeUnit := range []uint16{0xd800, 0xdc00} {
		data := make([]byte, 4)
		binary.LittleEndian.PutUint16(data[0:2], 1)
		binary.LittleEndian.PutUint16(data[2:4], codeUnit)
		if _, err := resourceName(data, resourceSubdirectory); err == nil {
			t.Fatalf("malformed UTF-16 code unit %#x was accepted", codeUnit)
		}
	}
}

func TestParseMixedRawResourcesEnforcesLimitsBeforeCopying(t *testing.T) {
	resourceData := buildRawResourceLimitFixture([]uint32{3, 3})
	resolveCalls := 0
	resolve := func(_ uint32, size uint32) ([]byte, error) {
		resolveCalls++
		return make([]byte, size), nil
	}
	_, err := parseMixedRawResources(resourceData, resolve, tacticalTextureResourceType, rawResourceLimits{
		MaxCount:          2,
		MaxResourceBytes:  4,
		MaxAggregateBytes: 4,
	})
	if err == nil {
		t.Fatal("aggregate resource limit was not enforced")
	}
	if resolveCalls != 1 {
		t.Fatalf("resolver calls = %d, want 1 before aggregate rejection", resolveCalls)
	}

	resourceData = buildRawResourceLimitFixture([]uint32{5})
	resolveCalls = 0
	_, err = parseMixedRawResources(resourceData, resolve, tacticalTextureResourceType, rawResourceLimits{
		MaxCount:          1,
		MaxResourceBytes:  4,
		MaxAggregateBytes: 4,
	})
	if err == nil {
		t.Fatal("per-resource limit was not enforced")
	}
	if resolveCalls != 0 {
		t.Fatalf("oversized resource reached resolver %d times", resolveCalls)
	}

	resourceData = buildRawResourceLimitFixture([]uint32{1, 1})
	resolveCalls = 0
	_, err = parseMixedRawResources(resourceData, resolve, tacticalTextureResourceType, rawResourceLimits{
		MaxCount:          1,
		MaxResourceBytes:  4,
		MaxAggregateBytes: 4,
	})
	if err == nil {
		t.Fatal("resource count limit was not enforced")
	}
	if resolveCalls != 1 {
		t.Fatalf("resolver calls = %d, want 1 before count rejection", resolveCalls)
	}
}

func buildRawResourceLimitFixture(sizes []uint32) []byte {
	resourceData := make([]byte, 0xb0)
	putResourceDirectory(resourceData, 0x00, 0, 1)
	putResourceEntry(resourceData, 0x10, tacticalTextureResourceType, resourceSubdirectory|0x20)
	putResourceDirectory(resourceData, 0x20, 0, uint16(len(sizes)))
	for index, size := range sizes {
		languageDirectory := 0x50 + index*0x20
		dataEntry := 0x90 + index*0x10
		putResourceEntry(resourceData, 0x30+index*8, uint32(4000+index), resourceSubdirectory|uint32(languageDirectory))
		putResourceDirectory(resourceData, languageDirectory, 0, 1)
		putResourceEntry(resourceData, languageDirectory+0x10, 1033, uint32(dataEntry))
		binary.LittleEndian.PutUint32(resourceData[dataEntry:dataEntry+4], uint32(0x2000+index*0x10))
		binary.LittleEndian.PutUint32(resourceData[dataEntry+4:dataEntry+8], size)
	}
	return resourceData
}
