package main

import (
	"bytes"
	"debug/pe"
	"encoding/binary"
	"fmt"
	"unicode/utf16"
)

const (
	rtBitmap             = uint32(2)
	rtAdvisorFrame       = uint32(302)
	resourceSubdirectory = uint32(0x80000000)
)

type bitmapResource struct {
	ID       uint32
	Language uint32
	DIB      []byte
}

type rawResource struct {
	ID       uint32
	Name     string
	Named    bool
	Language uint32
	CodePage uint32
	Reserved uint32
	Data     []byte
}

type rawResourceLimits struct {
	MaxCount          int
	MaxResourceBytes  uint64
	MaxAggregateBytes uint64
}

type resourceDirectoryEntry struct {
	name   uint32
	target uint32
}

func parseBitmapResources(resourceData []byte, resolveRVA func(uint32, uint32) ([]byte, error), namedIDs map[string]uint32) ([]bitmapResource, error) {
	types, err := readResourceDirectory(resourceData, 0)
	if err != nil {
		return nil, fmt.Errorf("read resource types: %w", err)
	}

	var resources []bitmapResource
	for _, resourceType := range types {
		if resourceType.name&resourceSubdirectory != 0 || resourceType.name != rtBitmap {
			continue
		}
		if resourceType.target&resourceSubdirectory == 0 {
			return nil, fmt.Errorf("RT_BITMAP entry does not point to a directory")
		}

		ids, err := readResourceDirectory(resourceData, resourceType.target&^resourceSubdirectory)
		if err != nil {
			return nil, fmt.Errorf("read bitmap IDs: %w", err)
		}
		for _, idEntry := range ids {
			resourceID, err := bitmapResourceID(resourceData, idEntry.name, namedIDs)
			if err != nil {
				return nil, err
			}
			if idEntry.target&resourceSubdirectory == 0 {
				return nil, fmt.Errorf("bitmap resource %d does not point to a language directory", resourceID)
			}

			languages, err := readResourceDirectory(resourceData, idEntry.target&^resourceSubdirectory)
			if err != nil {
				return nil, fmt.Errorf("read languages for bitmap %d: %w", resourceID, err)
			}
			for _, languageEntry := range languages {
				if languageEntry.name&resourceSubdirectory != 0 {
					continue
				}
				if languageEntry.target&resourceSubdirectory != 0 {
					return nil, fmt.Errorf("bitmap resource %d language %d points to a directory", resourceID, languageEntry.name)
				}

				dataEntryOffset := languageEntry.target
				if uint64(dataEntryOffset)+16 > uint64(len(resourceData)) {
					return nil, fmt.Errorf("bitmap resource %d language %d has an out-of-bounds data entry", resourceID, languageEntry.name)
				}
				dataRVA := binary.LittleEndian.Uint32(resourceData[dataEntryOffset : dataEntryOffset+4])
				dataSize := binary.LittleEndian.Uint32(resourceData[dataEntryOffset+4 : dataEntryOffset+8])
				dib, err := resolveRVA(dataRVA, dataSize)
				if err != nil {
					return nil, fmt.Errorf("read bitmap resource %d language %d: %w", resourceID, languageEntry.name, err)
				}
				resources = append(resources, bitmapResource{
					ID:       resourceID,
					Language: languageEntry.name,
					DIB:      append([]byte(nil), dib...),
				})
			}
		}
	}

	return resources, nil
}

func parseRawResources(resourceData []byte, resolveRVA func(uint32, uint32) ([]byte, error), resourceTypeID uint32) ([]rawResource, error) {
	return parseRawResourcesMode(resourceData, resolveRVA, resourceTypeID, "", false, rawResourceLimits{})
}

func parseTypedRawResources(resourceData []byte, resolveRVA func(uint32, uint32) ([]byte, error), resourceTypeID uint32, typeName string) ([]rawResource, error) {
	return parseRawResourcesMode(resourceData, resolveRVA, resourceTypeID, typeName, false, rawResourceLimits{})
}

func parseMixedRawResources(resourceData []byte, resolveRVA func(uint32, uint32) ([]byte, error), resourceTypeID uint32, limits rawResourceLimits) ([]rawResource, error) {
	return parseRawResourcesMode(resourceData, resolveRVA, resourceTypeID, "", true, limits)
}

func parseRawResourcesMode(resourceData []byte, resolveRVA func(uint32, uint32) ([]byte, error), resourceTypeID uint32, typeName string, allowNamed bool, limits rawResourceLimits) ([]rawResource, error) {
	types, err := readResourceDirectory(resourceData, 0)
	if err != nil {
		return nil, fmt.Errorf("read resource types: %w", err)
	}

	var resources []rawResource
	var aggregateBytes uint64
	for _, resourceType := range types {
		if typeName != "" {
			if resourceType.name&resourceSubdirectory == 0 {
				continue
			}
			name, err := resourceName(resourceData, resourceType.name)
			if err != nil {
				return nil, err
			}
			if name != typeName {
				continue
			}
		} else if resourceType.name&resourceSubdirectory != 0 || resourceType.name != resourceTypeID {
			continue
		}
		if resourceType.target&resourceSubdirectory == 0 {
			return nil, fmt.Errorf("resource type %d does not point to a directory", resourceTypeID)
		}

		ids, err := readResourceDirectory(resourceData, resourceType.target&^resourceSubdirectory)
		if err != nil {
			return nil, fmt.Errorf("read resource type %d IDs: %w", resourceTypeID, err)
		}
		for _, idEntry := range ids {
			resourceID := idEntry.name
			resourceNameValue := ""
			resourceNamed := idEntry.name&resourceSubdirectory != 0
			if resourceNamed && !allowNamed {
				return nil, fmt.Errorf("resource type %d has unsupported named entry", resourceTypeID)
			}
			if resourceNamed {
				resourceNameValue, err = resourceName(resourceData, idEntry.name)
				if err != nil {
					return nil, fmt.Errorf("read resource type %d name: %w", resourceTypeID, err)
				}
				resourceID = 0
			}
			resourceLabel := fmt.Sprintf("ID %d", resourceID)
			if resourceNamed {
				resourceLabel = fmt.Sprintf("name %q", resourceNameValue)
			}
			if idEntry.target&resourceSubdirectory == 0 {
				return nil, fmt.Errorf("resource type %d %s does not point to a language directory", resourceTypeID, resourceLabel)
			}

			languages, err := readResourceDirectory(resourceData, idEntry.target&^resourceSubdirectory)
			if err != nil {
				return nil, fmt.Errorf("read languages for resource type %d %s: %w", resourceTypeID, resourceLabel, err)
			}
			for _, languageEntry := range languages {
				if languageEntry.name&resourceSubdirectory != 0 {
					continue
				}
				if languageEntry.target&resourceSubdirectory != 0 {
					return nil, fmt.Errorf("resource type %d %s language %d points to a directory", resourceTypeID, resourceLabel, languageEntry.name)
				}

				dataEntryOffset := languageEntry.target
				if uint64(dataEntryOffset)+16 > uint64(len(resourceData)) {
					return nil, fmt.Errorf("resource type %d %s language %d has an out-of-bounds data entry", resourceTypeID, resourceLabel, languageEntry.name)
				}
				dataRVA := binary.LittleEndian.Uint32(resourceData[dataEntryOffset : dataEntryOffset+4])
				dataSize := binary.LittleEndian.Uint32(resourceData[dataEntryOffset+4 : dataEntryOffset+8])
				codePage := binary.LittleEndian.Uint32(resourceData[dataEntryOffset+8 : dataEntryOffset+12])
				reserved := binary.LittleEndian.Uint32(resourceData[dataEntryOffset+12 : dataEntryOffset+16])
				if limits.MaxCount > 0 && len(resources) >= limits.MaxCount {
					return nil, fmt.Errorf("resource type %d exceeds the %d-resource limit", resourceTypeID, limits.MaxCount)
				}
				if limits.MaxResourceBytes > 0 && uint64(dataSize) > limits.MaxResourceBytes {
					return nil, fmt.Errorf("resource type %d %s language %d size %d exceeds the %d-byte limit", resourceTypeID, resourceLabel, languageEntry.name, dataSize, limits.MaxResourceBytes)
				}
				if uint64(dataSize) > ^uint64(0)-aggregateBytes {
					return nil, fmt.Errorf("resource type %d aggregate size overflows", resourceTypeID)
				}
				aggregateBytes += uint64(dataSize)
				if limits.MaxAggregateBytes > 0 && aggregateBytes > limits.MaxAggregateBytes {
					return nil, fmt.Errorf("resource type %d aggregate size %d exceeds the %d-byte limit", resourceTypeID, aggregateBytes, limits.MaxAggregateBytes)
				}
				data, err := resolveRVA(dataRVA, dataSize)
				if err != nil {
					return nil, fmt.Errorf("read resource type %d %s language %d: %w", resourceTypeID, resourceLabel, languageEntry.name, err)
				}
				resources = append(resources, rawResource{
					ID:       resourceID,
					Name:     resourceNameValue,
					Named:    resourceNamed,
					Language: languageEntry.name,
					CodePage: codePage,
					Reserved: reserved,
					Data:     append([]byte(nil), data...),
				})
			}
		}
	}

	return resources, nil
}

func bitmapResourceID(resourceData []byte, rawName uint32, namedIDs map[string]uint32) (uint32, error) {
	if rawName&resourceSubdirectory == 0 {
		return rawName, nil
	}
	name, err := resourceName(resourceData, rawName)
	if err != nil {
		return 0, err
	}
	id, ok := namedIDs[name]
	if !ok {
		return 0, fmt.Errorf("unsupported named bitmap resource %q", name)
	}
	return id, nil
}

func readResourceDirectory(resourceData []byte, offset uint32) ([]resourceDirectoryEntry, error) {
	if uint64(offset)+16 > uint64(len(resourceData)) {
		return nil, fmt.Errorf("directory at offset %#x is outside resource data", offset)
	}
	named := binary.LittleEndian.Uint16(resourceData[offset+12 : offset+14])
	ids := binary.LittleEndian.Uint16(resourceData[offset+14 : offset+16])
	count := uint64(named) + uint64(ids)
	entriesOffset := uint64(offset) + 16
	if entriesOffset+count*8 > uint64(len(resourceData)) {
		return nil, fmt.Errorf("directory at offset %#x has %d out-of-bounds entries", offset, count)
	}

	entries := make([]resourceDirectoryEntry, 0, count)
	for i := uint64(0); i < count; i++ {
		entryOffset := entriesOffset + i*8
		entries = append(entries, resourceDirectoryEntry{
			name:   binary.LittleEndian.Uint32(resourceData[entryOffset : entryOffset+4]),
			target: binary.LittleEndian.Uint32(resourceData[entryOffset+4 : entryOffset+8]),
		})
	}
	return entries, nil
}

func readPEBitmapResources(path string, namedIDs map[string]uint32) ([]bitmapResource, error) {
	file, err := pe.Open(path)
	if err != nil {
		return nil, fmt.Errorf("open PE file: %w", err)
	}
	defer file.Close()

	resourceDirectory, err := peDataDirectory(file, 2)
	if err != nil {
		return nil, err
	}
	if resourceDirectory.VirtualAddress == 0 || resourceDirectory.Size == 0 {
		return nil, fmt.Errorf("PE file has no resource directory")
	}

	resourceData, err := readPERange(file, resourceDirectory.VirtualAddress, resourceDirectory.Size)
	if err != nil {
		return nil, fmt.Errorf("read resource directory: %w", err)
	}
	return parseBitmapResources(resourceData, func(rva, size uint32) ([]byte, error) {
		return readPERange(file, rva, size)
	}, namedIDs)
}

func readPERawResources(path string, resourceTypeID uint32) ([]rawResource, error) {
	return readPETypedRawResources(path, resourceTypeID, "")
}

func readPEMixedRawResourcesFromBytes(source []byte, resourceTypeID uint32, limits rawResourceLimits) ([]rawResource, error) {
	file, err := pe.NewFile(bytes.NewReader(source))
	if err != nil {
		return nil, fmt.Errorf("open PE snapshot: %w", err)
	}
	defer file.Close()

	resourceDirectory, err := peDataDirectory(file, 2)
	if err != nil {
		return nil, err
	}
	if resourceDirectory.VirtualAddress == 0 || resourceDirectory.Size == 0 {
		return nil, fmt.Errorf("PE file has no resource directory")
	}

	resourceData, err := readPERange(file, resourceDirectory.VirtualAddress, resourceDirectory.Size)
	if err != nil {
		return nil, fmt.Errorf("read resource directory: %w", err)
	}
	return parseMixedRawResources(resourceData, func(rva, size uint32) ([]byte, error) {
		return readPERange(file, rva, size)
	}, resourceTypeID, limits)
}

func readPEWaveResources(path string) ([]rawResource, error) {
	return readPETypedRawResources(path, 0, "WAVE")
}

func readPETypedRawResources(path string, resourceTypeID uint32, typeName string) ([]rawResource, error) {
	file, err := pe.Open(path)
	if err != nil {
		return nil, fmt.Errorf("open PE file: %w", err)
	}
	defer file.Close()

	resourceDirectory, err := peDataDirectory(file, 2)
	if err != nil {
		return nil, err
	}
	if resourceDirectory.VirtualAddress == 0 || resourceDirectory.Size == 0 {
		return nil, fmt.Errorf("PE file has no resource directory")
	}

	resourceData, err := readPERange(file, resourceDirectory.VirtualAddress, resourceDirectory.Size)
	if err != nil {
		return nil, fmt.Errorf("read resource directory: %w", err)
	}
	return parseTypedRawResources(resourceData, func(rva, size uint32) ([]byte, error) {
		return readPERange(file, rva, size)
	}, resourceTypeID, typeName)
}

func peDataDirectory(file *pe.File, index int) (pe.DataDirectory, error) {
	switch header := file.OptionalHeader.(type) {
	case *pe.OptionalHeader32:
		if index < 0 || index >= len(header.DataDirectory) || uint32(index) >= header.NumberOfRvaAndSizes {
			return pe.DataDirectory{}, fmt.Errorf("PE32 data directory %d is unavailable", index)
		}
		return header.DataDirectory[index], nil
	case *pe.OptionalHeader64:
		if index < 0 || index >= len(header.DataDirectory) || uint32(index) >= header.NumberOfRvaAndSizes {
			return pe.DataDirectory{}, fmt.Errorf("PE32+ data directory %d is unavailable", index)
		}
		return header.DataDirectory[index], nil
	default:
		return pe.DataDirectory{}, fmt.Errorf("PE file has no supported optional header")
	}
}

func readPERange(file *pe.File, rva, size uint32) ([]byte, error) {
	requestedStart := uint64(rva)
	requestedEnd := requestedStart + uint64(size)
	for _, section := range file.Sections {
		sectionStart := uint64(section.VirtualAddress)
		sectionSpan := uint64(section.VirtualSize)
		if rawSize := uint64(section.Size); rawSize > sectionSpan {
			sectionSpan = rawSize
		}
		sectionEnd := sectionStart + sectionSpan
		if requestedStart < sectionStart || requestedEnd > sectionEnd {
			continue
		}

		data, err := section.Data()
		if err != nil {
			return nil, fmt.Errorf("read section %q: %w", section.Name, err)
		}
		offset := requestedStart - sectionStart
		if offset+uint64(size) > uint64(len(data)) {
			return nil, fmt.Errorf("RVA %#x size %d exceeds raw data for section %q", rva, size, section.Name)
		}
		return data[offset : offset+uint64(size)], nil
	}
	return nil, fmt.Errorf("RVA %#x size %d is not contained in a PE section", rva, size)
}

func resourceName(resourceData []byte, rawName uint32) (string, error) {
	offset := rawName &^ resourceSubdirectory
	if uint64(offset)+2 > uint64(len(resourceData)) {
		return "", fmt.Errorf("PE resource name at offset %#x is outside resource data", offset)
	}
	length := binary.LittleEndian.Uint16(resourceData[offset : offset+2])
	end := uint64(offset) + 2 + uint64(length)*2
	if end > uint64(len(resourceData)) {
		return "", fmt.Errorf("PE resource name at offset %#x is truncated", offset)
	}
	codeUnits := make([]uint16, length)
	for i := range codeUnits {
		start := uint64(offset) + 2 + uint64(i)*2
		codeUnits[i] = binary.LittleEndian.Uint16(resourceData[start : start+2])
	}
	for i := 0; i < len(codeUnits); i++ {
		switch {
		case codeUnits[i] >= 0xd800 && codeUnits[i] <= 0xdbff:
			if i+1 >= len(codeUnits) || codeUnits[i+1] < 0xdc00 || codeUnits[i+1] > 0xdfff {
				return "", fmt.Errorf("PE resource name at offset %#x contains malformed UTF-16", offset)
			}
			i++
		case codeUnits[i] >= 0xdc00 && codeUnits[i] <= 0xdfff:
			return "", fmt.Errorf("PE resource name at offset %#x contains malformed UTF-16", offset)
		}
	}
	return string(utf16.Decode(codeUnits)), nil
}
