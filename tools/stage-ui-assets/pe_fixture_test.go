package main

import (
	"encoding/binary"
	"testing"
)

func putResourceDirectory(dst []byte, offset int, named, ids uint16) {
	binary.LittleEndian.PutUint16(dst[offset+12:offset+14], named)
	binary.LittleEndian.PutUint16(dst[offset+14:offset+16], ids)
}

func putResourceEntry(dst []byte, offset int, name, target uint32) {
	binary.LittleEndian.PutUint32(dst[offset:offset+4], name)
	binary.LittleEndian.PutUint32(dst[offset+4:offset+8], target)
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
