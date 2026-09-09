package main

import (
	"encoding/binary"
	"testing"
	"unicode/utf16"
)

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
	resources, err := parseBitmapResources(section, resolve, namedBitmapIDs)
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
			id, err := bitmapResourceID(resourceData, resourceSubdirectory, namedBitmapIDs)
			if err != nil {
				t.Fatalf("bitmapResourceID() error = %v", err)
			}
			if id != test.id {
				t.Errorf("ID = %d, want %d", id, test.id)
			}
		})
	}
}
