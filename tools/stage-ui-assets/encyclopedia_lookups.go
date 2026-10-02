package main

import (
	"crypto/sha256"
	"encoding/binary"
	"encoding/hex"
	"fmt"
	"io"
	"math"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"unicode/utf16"
)

const (
	rtStringResource                 = uint32(6)
	identifiedEnglishEncybmapSHA256  = "fb545d19ae24b0277753494dbfaabf2dbdde660beab821287a32016c290e4560"
	maxEncyclopediaLookupSourceBytes = uint64(512 << 20)
)

type encyclopediaLookupResolution string

const (
	encyclopediaLookupExact      encyclopediaLookupResolution = "exact"
	encyclopediaLookupCaseFolded encyclopediaLookupResolution = "case_folded"
	encyclopediaLookupMissing    encyclopediaLookupResolution = "missing"
	encyclopediaLookupAmbiguous  encyclopediaLookupResolution = "case_ambiguous"
)

type encyclopediaLookupBlockRecord struct {
	BlockID            encyclopediaResourceIdentifier `json:"block_id"`
	LanguageID         uint16                         `json:"language_id"`
	CodePage           uint32                         `json:"code_page"`
	RawLength          uint64                         `json:"raw_length"`
	RawSHA256          string                         `json:"raw_sha256"`
	NonemptyEntryCount int                            `json:"nonempty_entry_count"`
	Status             encyclopediaRecordStatus       `json:"status"`
	Unresolved         *encyclopediaUnresolvedStatus  `json:"unresolved,omitempty"`
	RawBytes           []byte                         `json:"-"`
}

type encyclopediaLookupDecode struct {
	Lookups map[uint16]map[uint32]string    `json:"lookups"`
	Blocks  []encyclopediaLookupBlockRecord `json:"blocks"`
}

type encyclopediaLookupReference struct {
	LanguageID      uint16                       `json:"language_id"`
	LogicalID       uint32                       `json:"logical_id"`
	Filename        string                       `json:"filename"`
	FileNumber      *uint32                      `json:"file_number,omitempty"`
	Resolution      encyclopediaLookupResolution `json:"resolution"`
	MatchedBasename string                       `json:"matched_basename,omitempty"`
}

type encyclopediaLookupFileRecord struct {
	Basename   string `json:"basename"`
	FileNumber uint32 `json:"file_number"`
	RawLength  uint64 `json:"raw_length"`
	RawSHA256  string `json:"raw_sha256"`
}

type encyclopediaDuplicateLookupReference struct {
	Filename   string                        `json:"filename"`
	References []encyclopediaLookupReference `json:"references"`
}

type encyclopediaLookupCaseAmbiguity struct {
	FoldedFilename string   `json:"folded_filename"`
	Candidates     []string `json:"candidates"`
}

type encyclopediaLookupReconciliation struct {
	References          []encyclopediaLookupReference          `json:"references"`
	Files               []encyclopediaLookupFileRecord         `json:"files"`
	DuplicateReferences []encyclopediaDuplicateLookupReference `json:"duplicate_references"`
	CaseAmbiguities     []encyclopediaLookupCaseAmbiguity      `json:"case_ambiguities"`
	MissingFilenames    []encyclopediaLookupReference          `json:"missing_filenames"`
	UnreferencedFiles   []encyclopediaLookupFileRecord         `json:"unreferenced_files"`
}

type encyclopediaLookupInventory struct {
	SourceBasename  string                          `json:"source_basename"`
	SourceRawLength uint64                          `json:"source_raw_length"`
	SourceRawSHA256 string                          `json:"source_raw_sha256"`
	SourceRawBytes  []byte                          `json:"-"`
	Lookups         map[uint16]map[uint32]string    `json:"lookups"`
	Blocks          []encyclopediaLookupBlockRecord `json:"blocks"`
	encyclopediaLookupReconciliation
}

func decodeEncyclopediaLookups(resources []rawResource) (map[uint16]map[uint32]string, error) {
	decoded, err := decodeEncyclopediaLookupBlocks(resources)
	if err != nil {
		return nil, err
	}
	return decoded.Lookups, nil
}

func decodeEncyclopediaLookupBlocks(resources []rawResource) (encyclopediaLookupDecode, error) {
	if len(resources) == 0 {
		return encyclopediaLookupDecode{}, fmt.Errorf("encyclopedia lookup inventory requires at least one RT_STRING block")
	}
	ordered := append([]rawResource(nil), resources...)
	sort.SliceStable(ordered, func(i, j int) bool {
		if ordered[i].Language != ordered[j].Language {
			return ordered[i].Language < ordered[j].Language
		}
		left := encyclopediaLookupBlockIdentifier(ordered[i])
		right := encyclopediaLookupBlockIdentifier(ordered[j])
		if comparison := compareEncyclopediaResourceIdentifier(left, right); comparison != 0 {
			return comparison < 0
		}
		if ordered[i].CodePage != ordered[j].CodePage {
			return ordered[i].CodePage < ordered[j].CodePage
		}
		return byteSHA256(ordered[i].Data) < byteSHA256(ordered[j].Data)
	})

	result := encyclopediaLookupDecode{
		Lookups: make(map[uint16]map[uint32]string),
		Blocks:  make([]encyclopediaLookupBlockRecord, 0, len(ordered)),
	}
	seen := make(map[encyclopediaLookupBlockKey]struct{}, len(ordered))
	for _, resource := range ordered {
		if resource.Language > math.MaxUint16 {
			return encyclopediaLookupDecode{}, fmt.Errorf("RT_STRING block language %d is outside LANGID range", resource.Language)
		}
		languageID := uint16(resource.Language)
		identifier := encyclopediaLookupBlockIdentifier(resource)
		if err := validateEncyclopediaResourceIdentifier(identifier); err != nil {
			return encyclopediaLookupDecode{}, fmt.Errorf("invalid RT_STRING block identity: %w", err)
		}
		key := encyclopediaLookupBlockKey{
			LanguageID: languageID,
			Kind:       identifier.Kind,
		}
		if identifier.NumericID != nil {
			key.NumericID = *identifier.NumericID
		}
		key.Name = identifier.Name
		if _, exists := seen[key]; exists {
			return encyclopediaLookupDecode{}, fmt.Errorf("duplicate RT_STRING block %s for LANGID %d", formatEncyclopediaResourceIdentifier(identifier), languageID)
		}
		seen[key] = struct{}{}

		entries, err := decodeEncyclopediaStringBlock(resource.Data, identifier)
		if err != nil {
			return encyclopediaLookupDecode{}, err
		}
		nonempty := 0
		for _, value := range entries {
			if value != "" {
				nonempty++
			}
		}
		block := encyclopediaLookupBlockRecord{
			BlockID:            identifier,
			LanguageID:         languageID,
			CodePage:           resource.CodePage,
			RawLength:          uint64(len(resource.Data)),
			RawSHA256:          byteSHA256(resource.Data),
			NonemptyEntryCount: nonempty,
			Status:             encyclopediaRecordDecoded,
			RawBytes:           append([]byte(nil), resource.Data...),
		}
		if resource.Named {
			block.Status = encyclopediaRecordUnresolved
			block.Unresolved = &encyclopediaUnresolvedStatus{
				Reason:    "named RT_STRING blocks have no numeric block ID for the logical-ID formula",
				NextProof: "recover a source-backed named-block selector before assigning any logical lookup identity",
			}
			result.Blocks = append(result.Blocks, block)
			continue
		}
		if resource.ID == 0 {
			return encyclopediaLookupDecode{}, fmt.Errorf("numeric RT_STRING block ID 0 cannot produce logical string IDs")
		}
		if resource.ID-1 > (math.MaxUint32-15)/16 {
			return encyclopediaLookupDecode{}, fmt.Errorf("RT_STRING block ID %d overflows uint32 logical string IDs", resource.ID)
		}
		base := (resource.ID - 1) * 16
		languageLookups := result.Lookups[languageID]
		if languageLookups == nil {
			languageLookups = make(map[uint32]string)
			result.Lookups[languageID] = languageLookups
		}
		for slot, value := range entries {
			if value == "" {
				continue
			}
			logicalID := base + uint32(slot)
			if _, exists := languageLookups[logicalID]; exists {
				return encyclopediaLookupDecode{}, fmt.Errorf("duplicate RT_STRING logical ID %d for LANGID %d", logicalID, languageID)
			}
			languageLookups[logicalID] = value
		}
		result.Blocks = append(result.Blocks, block)
	}
	return result, nil
}

func decodeEncyclopediaStringBlock(data []byte, identifier encyclopediaResourceIdentifier) ([16]string, error) {
	var entries [16]string
	position := 0
	for slot := range entries {
		if position+2 > len(data) {
			return entries, fmt.Errorf("truncated RT_STRING block %s before slot %d length", formatEncyclopediaResourceIdentifier(identifier), slot)
		}
		length := int(binary.LittleEndian.Uint16(data[position : position+2]))
		position += 2
		if length > (len(data)-position)/2 {
			return entries, fmt.Errorf("truncated RT_STRING block %s in slot %d", formatEncyclopediaResourceIdentifier(identifier), slot)
		}
		units := make([]uint16, length)
		for index := range units {
			units[index] = binary.LittleEndian.Uint16(data[position+index*2 : position+index*2+2])
		}
		position += length * 2
		if err := validateEncyclopediaUTF16(units); err != nil {
			return entries, fmt.Errorf("RT_STRING block %s slot %d: %w", formatEncyclopediaResourceIdentifier(identifier), slot, err)
		}
		entries[slot] = string(utf16.Decode(units))
	}
	if padding := len(data) - position; padding > 3 {
		return entries, fmt.Errorf("RT_STRING block %s has %d bytes after its 16 entries", formatEncyclopediaResourceIdentifier(identifier), padding)
	}
	for _, value := range data[position:] {
		if value != 0 {
			return entries, fmt.Errorf("RT_STRING block %s has nonzero alignment padding", formatEncyclopediaResourceIdentifier(identifier))
		}
	}
	return entries, nil
}

func validateEncyclopediaUTF16(units []uint16) error {
	for index := 0; index < len(units); index++ {
		switch value := units[index]; {
		case value >= 0xd800 && value <= 0xdbff:
			if index+1 >= len(units) || units[index+1] < 0xdc00 || units[index+1] > 0xdfff {
				return fmt.Errorf("unpaired high UTF-16 surrogate")
			}
			index++
		case value >= 0xdc00 && value <= 0xdfff:
			return fmt.Errorf("unpaired low UTF-16 surrogate")
		}
	}
	return nil
}

func reconcileEncyclopediaLookupFiles(lookups map[uint16]map[uint32]string, edataRoot string) (encyclopediaLookupReconciliation, error) {
	rootInfo, err := os.Stat(edataRoot)
	if err != nil {
		return encyclopediaLookupReconciliation{}, fmt.Errorf("inspect declared EData root: %w", err)
	}
	if !rootInfo.IsDir() {
		return encyclopediaLookupReconciliation{}, fmt.Errorf("declared EData root is not a directory")
	}

	entries, err := os.ReadDir(edataRoot)
	if err != nil {
		return encyclopediaLookupReconciliation{}, fmt.Errorf("read declared EData root: %w", err)
	}
	files := make([]encyclopediaLookupFileRecord, 0, len(entries))
	filesByFold := make(map[string][]encyclopediaLookupFileRecord)
	for _, entry := range entries {
		fileNumber, ok := parseEncyclopediaEDataFilename(entry.Name())
		if !ok {
			continue
		}
		rawLength, rawSHA256, err := hashEncyclopediaLookupFile(filepath.Join(edataRoot, entry.Name()))
		if err != nil {
			return encyclopediaLookupReconciliation{}, fmt.Errorf("hash EData file %q: %w", entry.Name(), err)
		}
		file := encyclopediaLookupFileRecord{
			Basename:   entry.Name(),
			FileNumber: fileNumber,
			RawLength:  rawLength,
			RawSHA256:  rawSHA256,
		}
		files = append(files, file)
		folded := foldEncyclopediaFilename(entry.Name())
		filesByFold[folded] = append(filesByFold[folded], file)
	}
	sort.Slice(files, func(i, j int) bool { return files[i].Basename < files[j].Basename })
	for folded := range filesByFold {
		sort.Slice(filesByFold[folded], func(i, j int) bool {
			return filesByFold[folded][i].Basename < filesByFold[folded][j].Basename
		})
	}

	result := encyclopediaLookupReconciliation{Files: files}
	for folded, candidates := range filesByFold {
		if len(candidates) < 2 {
			continue
		}
		names := make([]string, len(candidates))
		for index, candidate := range candidates {
			names[index] = candidate.Basename
		}
		result.CaseAmbiguities = append(result.CaseAmbiguities, encyclopediaLookupCaseAmbiguity{
			FoldedFilename: folded,
			Candidates:     names,
		})
	}
	sort.Slice(result.CaseAmbiguities, func(i, j int) bool {
		return result.CaseAmbiguities[i].FoldedFilename < result.CaseAmbiguities[j].FoldedFilename
	})

	languages := make([]int, 0, len(lookups))
	for languageID := range lookups {
		languages = append(languages, int(languageID))
	}
	sort.Ints(languages)
	referencesByFold := make(map[string][]encyclopediaLookupReference)
	referencedFiles := make(map[string]struct{})
	for _, languageValue := range languages {
		languageID := uint16(languageValue)
		logicalIDs := make([]uint32, 0, len(lookups[languageID]))
		for logicalID := range lookups[languageID] {
			logicalIDs = append(logicalIDs, logicalID)
		}
		sort.Slice(logicalIDs, func(i, j int) bool { return logicalIDs[i] < logicalIDs[j] })
		for _, logicalID := range logicalIDs {
			filename := lookups[languageID][logicalID]
			fileNumber, ok := parseEncyclopediaEDataFilename(filename)
			if !ok || filename != filepath.Base(filename) || strings.ContainsAny(filename, `/\\`) {
				return encyclopediaLookupReconciliation{}, fmt.Errorf("lookup LANGID %d logical ID %d has unsupported EData filename %q", languageID, logicalID, filename)
			}
			fileNumberCopy := fileNumber
			reference := encyclopediaLookupReference{
				LanguageID: languageID,
				LogicalID:  logicalID,
				Filename:   filename,
				FileNumber: &fileNumberCopy,
				Resolution: encyclopediaLookupMissing,
			}
			folded := foldEncyclopediaFilename(filename)
			candidates := filesByFold[folded]
			switch len(candidates) {
			case 0:
				result.MissingFilenames = append(result.MissingFilenames, reference)
			case 1:
				reference.MatchedBasename = candidates[0].Basename
				if filename == candidates[0].Basename {
					reference.Resolution = encyclopediaLookupExact
				} else {
					reference.Resolution = encyclopediaLookupCaseFolded
				}
				referencedFiles[candidates[0].Basename] = struct{}{}
			default:
				reference.Resolution = encyclopediaLookupAmbiguous
				for _, candidate := range candidates {
					referencedFiles[candidate.Basename] = struct{}{}
				}
			}
			result.References = append(result.References, reference)
			referencesByFold[folded] = append(referencesByFold[folded], reference)
		}
	}
	for _, references := range referencesByFold {
		if len(references) < 2 {
			continue
		}
		result.DuplicateReferences = append(result.DuplicateReferences, encyclopediaDuplicateLookupReference{
			Filename:   canonicalLookupFilename(references),
			References: append([]encyclopediaLookupReference(nil), references...),
		})
	}
	sort.Slice(result.DuplicateReferences, func(i, j int) bool {
		return foldEncyclopediaFilename(result.DuplicateReferences[i].Filename) < foldEncyclopediaFilename(result.DuplicateReferences[j].Filename)
	})
	for _, file := range files {
		if _, referenced := referencedFiles[file.Basename]; !referenced {
			result.UnreferencedFiles = append(result.UnreferencedFiles, file)
		}
	}
	return result, nil
}

func inventoryEncyclopediaLookups(sourceBasename string, sourceBytes []byte, edataRoot string) (encyclopediaLookupInventory, error) {
	if sourceBasename == "" || sourceBasename != filepath.Base(sourceBasename) || strings.ContainsAny(sourceBasename, `/\\`) {
		return encyclopediaLookupInventory{}, fmt.Errorf("invalid ENCYBMAP source basename %q", sourceBasename)
	}
	if !strings.EqualFold(sourceBasename, "ENCYBMAP.DLL") {
		return encyclopediaLookupInventory{}, fmt.Errorf("encyclopedia lookup source %q is not ENCYBMAP.DLL", sourceBasename)
	}
	if uint64(len(sourceBytes)) > maxEncyclopediaLookupSourceBytes {
		return encyclopediaLookupInventory{}, fmt.Errorf("ENCYBMAP source size %d exceeds the %d-byte limit", len(sourceBytes), maxEncyclopediaLookupSourceBytes)
	}
	resources, err := readPEStrictMixedRawResourcesFromBytes(sourceBytes, rtStringResource, rawResourceLimits{
		MaxCount:          4096,
		MaxResourceBytes:  1 << 20,
		MaxAggregateBytes: 32 << 20,
	})
	if err != nil {
		return encyclopediaLookupInventory{}, fmt.Errorf("inventory ENCYBMAP RT_STRING resources: %w", err)
	}
	decoded, err := decodeEncyclopediaLookupBlocks(resources)
	if err != nil {
		return encyclopediaLookupInventory{}, err
	}
	reconciliation, err := reconcileEncyclopediaLookupFiles(decoded.Lookups, edataRoot)
	if err != nil {
		return encyclopediaLookupInventory{}, err
	}
	return encyclopediaLookupInventory{
		SourceBasename:                   sourceBasename,
		SourceRawLength:                  uint64(len(sourceBytes)),
		SourceRawSHA256:                  byteSHA256(sourceBytes),
		SourceRawBytes:                   append([]byte(nil), sourceBytes...),
		Lookups:                          decoded.Lookups,
		Blocks:                           decoded.Blocks,
		encyclopediaLookupReconciliation: reconciliation,
	}, nil
}

type encyclopediaLookupBlockKey struct {
	LanguageID uint16
	Kind       encyclopediaResourceIdentifierKind
	NumericID  uint32
	Name       string
}

func encyclopediaLookupBlockIdentifier(resource rawResource) encyclopediaResourceIdentifier {
	if resource.Named {
		return namedEncyclopediaResourceIdentifier(resource.Name)
	}
	return numericEncyclopediaResourceIdentifier(resource.ID)
}

func parseEncyclopediaEDataFilename(filename string) (uint32, bool) {
	if len(filename) != len("EDATA.000") || !strings.EqualFold(filename[:6], "EDATA.") {
		return 0, false
	}
	number := uint32(0)
	for _, value := range []byte(filename[6:]) {
		if value < '0' || value > '9' {
			return 0, false
		}
		number = number*10 + uint32(value-'0')
	}
	return number, true
}

func foldEncyclopediaFilename(filename string) string {
	return strings.ToUpper(filename)
}

func canonicalLookupFilename(references []encyclopediaLookupReference) string {
	filenames := make([]string, len(references))
	for index, reference := range references {
		filenames[index] = reference.Filename
	}
	sort.Strings(filenames)
	return filenames[0]
}

func hashEncyclopediaLookupFile(path string) (uint64, string, error) {
	pathInfo, err := os.Lstat(path)
	if err != nil {
		return 0, "", err
	}
	if pathInfo.Mode()&os.ModeSymlink != 0 {
		return 0, "", fmt.Errorf("source is a symlink")
	}
	if !pathInfo.Mode().IsRegular() {
		return 0, "", fmt.Errorf("source is not a regular file")
	}

	file, err := os.Open(path)
	if err != nil {
		return 0, "", err
	}
	defer file.Close()
	openedInfo, err := file.Stat()
	if err != nil {
		return 0, "", err
	}
	if !openedInfo.Mode().IsRegular() || !os.SameFile(pathInfo, openedInfo) {
		return 0, "", fmt.Errorf("source identity changed while opening")
	}

	hash := sha256.New()
	count, err := io.Copy(hash, file)
	if err != nil {
		return 0, "", err
	}
	openedAfter, err := file.Stat()
	if err != nil {
		return 0, "", err
	}
	pathAfter, err := os.Lstat(path)
	if err != nil {
		return 0, "", err
	}
	if pathAfter.Mode()&os.ModeSymlink != 0 || !pathAfter.Mode().IsRegular() ||
		!os.SameFile(openedInfo, openedAfter) || !os.SameFile(openedInfo, pathAfter) {
		return 0, "", fmt.Errorf("source identity changed while hashing")
	}
	if count < 0 || openedInfo.Size() != count || openedAfter.Size() != count || pathAfter.Size() != count {
		return 0, "", fmt.Errorf("source size changed while hashing")
	}
	return uint64(count), hex.EncodeToString(hash.Sum(nil)), nil
}
