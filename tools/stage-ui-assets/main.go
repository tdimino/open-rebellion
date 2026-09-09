package main

import (
	"bytes"
	"debug/pe"
	"encoding/binary"
	"flag"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"unicode/utf16"
)

const bmpFileHeaderSize = 14

const (
	rtBitmap             = uint32(2)
	resourceSubdirectory = uint32(0x80000000)
)

var uiDLLTargets = []dllTarget{
	{Filename: "COMMON.DLL", Directory: "common-dll", Expected: 321},
	{Filename: "GOKRES.DLL", Directory: "gokres-dll", Expected: 580},
	{Filename: "STRATEGY.DLL", Directory: "strategy-dll", Expected: 1042},
	{Filename: "TACTICAL.DLL", Directory: "tactical-dll", Expected: 288},
}

var namedBitmapIDs = map[string]uint32{
	"COCKPIT_BUTTON_GAMESCALE_HUGE_UP":    15856,
	"COCKPIT_BUTTON_GAMESCALE_LARGE_UP":   15922,
	"COCKPIT_BUTTON_GAMESCALE_STD_UP":     15990,
	"DATA_BUTTON_UP_FIGHTERGROUP_RECOVER": 40720,
	"DATA_BUTTON_DN_FIGHTERGROUP_RECOVER": 40792,
	"DATA_BUTTON_UP_FIGHTERGROUP_TACTICS": 40864,
	"DATA_BUTTON_DN_FIGHTERGROUP_TACTICS": 40936,
}

type bitmapResource struct {
	ID       uint32
	Language uint32
	DIB      []byte
}

type stageResult struct {
	Written int
	Skipped int
}

type dllTarget struct {
	Filename  string
	Directory string
	Expected  int
}

type stageSummary struct {
	DLLs      int
	Resources int
	Written   int
	Skipped   int
}

type verifySummary struct {
	DLLs      int
	Resources int
}

type resourceDirectoryEntry struct {
	name   uint32
	target uint32
}

func dibToBMP(dib []byte) ([]byte, error) {
	if len(dib) < 40 {
		return nil, fmt.Errorf("DIB is too short: got %d bytes", len(dib))
	}

	headerSize := binary.LittleEndian.Uint32(dib[0:4])
	if headerSize < 40 || uint64(headerSize) > uint64(len(dib)) {
		return nil, fmt.Errorf("unsupported DIB header size %d", headerSize)
	}

	bitCount := binary.LittleEndian.Uint16(dib[14:16])
	colorsUsed := binary.LittleEndian.Uint32(dib[32:36])
	if colorsUsed == 0 && bitCount <= 8 {
		colorsUsed = uint32(1) << bitCount
	}

	pixelOffset := uint64(bmpFileHeaderSize) + uint64(headerSize) + uint64(colorsUsed)*4
	fileSize := uint64(bmpFileHeaderSize) + uint64(len(dib))
	if pixelOffset > fileSize || fileSize > uint64(^uint32(0)) {
		return nil, fmt.Errorf("invalid DIB dimensions: pixel offset %d, file size %d", pixelOffset, fileSize)
	}

	bmp := make([]byte, fileSize)
	copy(bmp[0:2], "BM")
	binary.LittleEndian.PutUint32(bmp[2:6], uint32(fileSize))
	binary.LittleEndian.PutUint32(bmp[10:14], uint32(pixelOffset))
	copy(bmp[bmpFileHeaderSize:], dib)
	return bmp, nil
}

func parseBitmapResources(resourceData []byte, resolveRVA func(uint32, uint32) ([]byte, error)) ([]bitmapResource, error) {
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
			resourceID, err := bitmapResourceID(resourceData, idEntry.name)
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

func bitmapResourceID(resourceData []byte, rawName uint32) (uint32, error) {
	if rawName&resourceSubdirectory == 0 {
		return rawName, nil
	}
	offset := rawName &^ resourceSubdirectory
	if uint64(offset)+2 > uint64(len(resourceData)) {
		return 0, fmt.Errorf("bitmap resource name at offset %#x is outside resource data", offset)
	}
	length := binary.LittleEndian.Uint16(resourceData[offset : offset+2])
	end := uint64(offset) + 2 + uint64(length)*2
	if end > uint64(len(resourceData)) {
		return 0, fmt.Errorf("bitmap resource name at offset %#x is truncated", offset)
	}
	codeUnits := make([]uint16, length)
	for i := range codeUnits {
		start := uint64(offset) + 2 + uint64(i)*2
		codeUnits[i] = binary.LittleEndian.Uint16(resourceData[start : start+2])
	}
	name := string(utf16.Decode(codeUnits))
	id, ok := namedBitmapIDs[name]
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

func readPEBitmapResources(path string) ([]bitmapResource, error) {
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
	})
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

func stageBitmapResources(resources []bitmapResource, outputDir string, force bool) (stageResult, error) {
	seen := make(map[uint32]uint32, len(resources))
	for _, resource := range resources {
		if language, exists := seen[resource.ID]; exists {
			return stageResult{}, fmt.Errorf("duplicate bitmap resource ID %d for languages %d and %d", resource.ID, language, resource.Language)
		}
		seen[resource.ID] = resource.Language
	}

	if err := os.MkdirAll(outputDir, 0o755); err != nil {
		return stageResult{}, fmt.Errorf("create output directory: %w", err)
	}

	result := stageResult{}
	for _, resource := range resources {
		bmp, err := dibToBMP(resource.DIB)
		if err != nil {
			return result, fmt.Errorf("convert bitmap resource %d: %w", resource.ID, err)
		}
		path := filepath.Join(outputDir, fmt.Sprintf("%d.bmp", resource.ID))
		if existing, err := os.ReadFile(path); err == nil {
			if bytes.Equal(existing, bmp) {
				result.Skipped++
				continue
			}
			if !force {
				return result, fmt.Errorf("bitmap resource %d already exists with different contents (use --force to replace it)", resource.ID)
			}
		} else if !os.IsNotExist(err) {
			return result, fmt.Errorf("read existing bitmap resource %d: %w", resource.ID, err)
		}
		if err := writeFileAtomically(path, bmp, 0o644); err != nil {
			return result, fmt.Errorf("write bitmap resource %d: %w", resource.ID, err)
		}
		result.Written++
	}
	return result, nil
}

func writeFileAtomically(path string, data []byte, mode os.FileMode) error {
	dir := filepath.Dir(path)
	temp, err := os.CreateTemp(dir, "."+filepath.Base(path)+".tmp-*")
	if err != nil {
		return err
	}
	tempPath := temp.Name()
	defer os.Remove(tempPath)

	if err := temp.Chmod(mode); err != nil {
		temp.Close()
		return err
	}
	if _, err := temp.Write(data); err != nil {
		temp.Close()
		return err
	}
	if err := temp.Sync(); err != nil {
		temp.Close()
		return err
	}
	if err := temp.Close(); err != nil {
		return err
	}
	return os.Rename(tempPath, path)
}

func stageTargets(sourceDir, outputDir string, targets []dllTarget, force bool, stdout io.Writer) (stageSummary, error) {
	summary := stageSummary{}
	for _, target := range targets {
		dllPath := filepath.Join(sourceDir, target.Filename)
		resources, err := readPEBitmapResources(dllPath)
		if err != nil {
			return summary, fmt.Errorf("%s: %w", target.Filename, err)
		}
		if len(resources) != target.Expected {
			return summary, fmt.Errorf("%s: found %d bitmap resources, expected %d", target.Filename, len(resources), target.Expected)
		}

		result, err := stageBitmapResources(resources, filepath.Join(outputDir, target.Directory, "BMP"), force)
		if err != nil {
			return summary, fmt.Errorf("%s: %w", target.Filename, err)
		}
		summary.DLLs++
		summary.Resources += len(resources)
		summary.Written += result.Written
		summary.Skipped += result.Skipped
		fmt.Fprintf(stdout, "%s: %d BMPs (%d written, %d unchanged)\n", target.Filename, len(resources), result.Written, result.Skipped)
	}
	return summary, nil
}

func validateBMP(bmp []byte) error {
	if len(bmp) < bmpFileHeaderSize+4 {
		return fmt.Errorf("BMP is too short: got %d bytes", len(bmp))
	}
	if string(bmp[:2]) != "BM" {
		return fmt.Errorf("invalid BMP signature %q", bmp[:2])
	}
	if declared := binary.LittleEndian.Uint32(bmp[2:6]); uint64(declared) != uint64(len(bmp)) {
		return fmt.Errorf("declared file size %d does not match actual size %d", declared, len(bmp))
	}
	dibHeaderSize := binary.LittleEndian.Uint32(bmp[14:18])
	if dibHeaderSize < 40 || uint64(bmpFileHeaderSize)+uint64(dibHeaderSize) > uint64(len(bmp)) {
		return fmt.Errorf("unsupported DIB header size %d", dibHeaderSize)
	}
	pixelOffset := binary.LittleEndian.Uint32(bmp[10:14])
	minimumOffset := uint64(bmpFileHeaderSize) + uint64(dibHeaderSize)
	if uint64(pixelOffset) < minimumOffset || uint64(pixelOffset) > uint64(len(bmp)) {
		return fmt.Errorf("invalid pixel offset %d for DIB header ending at %d", pixelOffset, minimumOffset)
	}
	return nil
}

func verifyTargets(outputDir string, targets []dllTarget, stdout io.Writer) (verifySummary, error) {
	summary := verifySummary{}
	for _, target := range targets {
		bmpDir := filepath.Join(outputDir, target.Directory, "BMP")
		entries, err := os.ReadDir(bmpDir)
		if err != nil {
			return summary, fmt.Errorf("%s: read staged directory: %w", target.Directory, err)
		}

		count := 0
		for _, entry := range entries {
			if entry.IsDir() || filepath.Ext(entry.Name()) != ".bmp" {
				continue
			}
			var resourceID uint32
			if _, err := fmt.Sscanf(entry.Name(), "%d.bmp", &resourceID); err != nil || entry.Name() != fmt.Sprintf("%d.bmp", resourceID) {
				return summary, fmt.Errorf("%s: non-numeric bitmap filename %q", target.Directory, entry.Name())
			}
			bmp, err := os.ReadFile(filepath.Join(bmpDir, entry.Name()))
			if err != nil {
				return summary, fmt.Errorf("%s/%s: %w", target.Directory, entry.Name(), err)
			}
			if err := validateBMP(bmp); err != nil {
				return summary, fmt.Errorf("%s/%s: %w", target.Directory, entry.Name(), err)
			}
			count++
		}
		if count != target.Expected {
			return summary, fmt.Errorf("%s: found %d staged BMPs, expected %d", target.Directory, count, target.Expected)
		}
		summary.DLLs++
		summary.Resources += count
		fmt.Fprintf(stdout, "%s: verified %d BMPs\n", target.Directory, count)
	}
	return summary, nil
}

func runCLI(args []string, stdout, stderr io.Writer, targets []dllTarget) error {
	flags := flag.NewFlagSet("stage-ui-assets", flag.ContinueOnError)
	flags.SetOutput(stderr)
	sourceDir := flags.String("source", "data/base", "directory containing the original game DLLs")
	outputDir := flags.String("output", "data/base/ui", "runtime UI asset directory")
	force := flags.Bool("force", false, "replace staged BMPs whose contents differ")
	verifyOnly := flags.Bool("verify", false, "verify staged BMPs without extracting DLLs")
	if err := flags.Parse(args); err != nil {
		return err
	}
	if flags.NArg() != 0 {
		return fmt.Errorf("unexpected arguments: %v", flags.Args())
	}

	if !*verifyOnly {
		summary, err := stageTargets(*sourceDir, *outputDir, targets, *force, stdout)
		if err != nil {
			return err
		}
		fmt.Fprintf(stdout, "Staged %d BMPs from %d DLLs (%d written, %d unchanged)\n", summary.Resources, summary.DLLs, summary.Written, summary.Skipped)
	}

	verified, err := verifyTargets(*outputDir, targets, stdout)
	if err != nil {
		return err
	}
	fmt.Fprintf(stdout, "Verified %d BMPs across %d DLLs\n", verified.Resources, verified.DLLs)
	return nil
}

func main() {
	if err := runCLI(os.Args[1:], os.Stdout, os.Stderr, uiDLLTargets); err != nil {
		if err == flag.ErrHelp {
			return
		}
		fmt.Fprintln(os.Stderr, "ERROR:", err)
		os.Exit(1)
	}
}
