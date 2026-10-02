package main

import (
	"bytes"
	"encoding/binary"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"sort"
	"strings"
	"testing"
)

func TestEncyclopediaLookupsPreserveLanguageAndLogicalStringIdentity(t *testing.T) {
	// The identified profile observation is ENCYBMAP block 297, slot 0 ->
	// logical ID 4736. The second language is synthetic and proves that a block
	// number is not a globally unique lookup identity.
	resources := []rawResource{
		{ID: 297, Language: 1033, CodePage: 0, Data: stringBundle(map[int]string{0: "EDATA.014", 7: "EDATA.077", 15: "星😀"})},
		{ID: 297, Language: 1031, CodePage: 1252, Data: stringBundle(map[int]string{0: "EDATA.115"})},
	}

	lookups, err := decodeEncyclopediaLookups(resources)
	if err != nil {
		t.Fatal(err)
	}
	if got := lookups[1033][4736]; got != "EDATA.014" {
		t.Fatalf("English lookup 4736 = %q, want EDATA.014", got)
	}
	if got := lookups[1031][4736]; got != "EDATA.115" {
		t.Fatalf("German lookup 4736 = %q, want EDATA.115", got)
	}
	if got := lookups[1033][4743]; got != "EDATA.077" {
		t.Fatalf("nonlinear English lookup 4743 = %q, want EDATA.077", got)
	}
	if got := lookups[1033][4751]; got != "星😀" {
		t.Fatalf("strict UTF-16 lookup 4751 = %q, want 星😀", got)
	}
}

func TestEncyclopediaLookupsRejectTruncatedLengthsAndUnpairedUTF16(t *testing.T) {
	tests := []struct {
		name string
		data []byte
	}{
		{name: "truncated length", data: []byte{1, 0}},
		{name: "unpaired high surrogate", data: invalidEncyclopediaStringBlock(0xd800)},
		{name: "unpaired low surrogate", data: invalidEncyclopediaStringBlock(0xdc00)},
	}
	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			if _, err := decodeEncyclopediaLookups([]rawResource{{ID: 1, Language: 1033, Data: test.data}}); err == nil {
				t.Fatal("malformed RT_STRING block was accepted")
			}
		})
	}
}

func TestEncyclopediaLookupsAcceptOnlyZeroAlignmentPaddingUpToThreeBytes(t *testing.T) {
	bundle := stringBundle(map[int]string{0: "EDATA.001"})
	for padding := 0; padding <= 3; padding++ {
		t.Run(fmt.Sprintf("%d zero bytes", padding), func(t *testing.T) {
			data := append([]byte(nil), bundle...)
			data = append(data, make([]byte, padding)...)
			if _, err := decodeEncyclopediaLookups([]rawResource{{ID: 1, Language: 1033, Data: data}}); err != nil {
				t.Fatalf("valid %d-byte alignment padding was rejected: %v", padding, err)
			}
		})
	}
	for _, test := range []struct {
		name   string
		suffix []byte
	}{
		{name: "four zero bytes", suffix: []byte{0, 0, 0, 0}},
		{name: "nonzero byte", suffix: []byte{1}},
	} {
		t.Run(test.name, func(t *testing.T) {
			data := append(append([]byte(nil), bundle...), test.suffix...)
			if _, err := decodeEncyclopediaLookups([]rawResource{{ID: 1, Language: 1033, Data: data}}); err == nil {
				t.Fatalf("invalid alignment padding %v was accepted", test.suffix)
			}
		})
	}
}

func TestEncyclopediaLookupsRejectInvalidLANGIDAndNumericBlockZero(t *testing.T) {
	bundle := stringBundle(map[int]string{0: "EDATA.001"})
	for _, test := range []struct {
		name     string
		resource rawResource
	}{
		{name: "LANGID above uint16", resource: rawResource{ID: 1, Language: 1 << 16, Data: bundle}},
		{name: "numeric block zero", resource: rawResource{ID: 0, Language: 1033, Data: bundle}},
	} {
		t.Run(test.name, func(t *testing.T) {
			if _, err := decodeEncyclopediaLookups([]rawResource{test.resource}); err == nil {
				t.Fatal("invalid RT_STRING identity was accepted")
			}
		})
	}
}

func TestEncyclopediaLookupsRejectDuplicateLanguageQualifiedIDs(t *testing.T) {
	bundle := stringBundle(map[int]string{0: "EDATA.001"})
	_, err := decodeEncyclopediaLookups([]rawResource{
		{ID: 1, Language: 1033, Data: bundle},
		{ID: 1, Language: 1033, Data: append([]byte(nil), bundle...)},
	})
	if err == nil || !strings.Contains(err.Error(), "duplicate") {
		t.Fatalf("duplicate language-qualified block error = %v", err)
	}
}

func TestEncyclopediaLookupsRejectAnEmptyResourceSet(t *testing.T) {
	if _, err := decodeEncyclopediaLookups(nil); err == nil {
		t.Fatal("empty RT_STRING resource set was accepted")
	}
}

func TestEncyclopediaLookupsKeepLogicalIDsBeyondUint16WithoutWrapping(t *testing.T) {
	lookups, err := decodeEncyclopediaLookups([]rawResource{{
		ID:       4097,
		Language: 1033,
		Data:     stringBundle(map[int]string{0: "EDATA.101"}),
	}})
	if err != nil {
		t.Fatal(err)
	}
	if got := lookups[1033][65536]; got != "EDATA.101" {
		t.Fatalf("lookup 65536 = %q, want EDATA.101", got)
	}
	if _, wrapped := lookups[1033][0]; wrapped {
		t.Fatal("logical ID wrapped to uint16")
	}
	upper, err := decodeEncyclopediaLookups([]rawResource{{
		ID:       268435456,
		Language: 1033,
		Data:     stringBundle(map[int]string{15: "EDATA.999"}),
	}})
	if err != nil {
		t.Fatal(err)
	}
	if got := upper[1033][uint32(^uint32(0))]; got != "EDATA.999" {
		t.Fatalf("maximum uint32 logical ID = %q, want EDATA.999", got)
	}

	if _, err := decodeEncyclopediaLookups([]rawResource{{
		ID:       268435457,
		Language: 1033,
		Data:     stringBundle(map[int]string{15: "EDATA.101"}),
	}}); err == nil {
		t.Fatal("overflowing logical ID range was accepted")
	}
}

func TestEncyclopediaLookupBlocksRecordNamedAndNumericIdentityWithoutAliasing(t *testing.T) {
	numericData := stringBundle(map[int]string{0: "EDATA.001"})
	namedData := stringBundle(map[int]string{0: "EDATA.002"})
	resources := []rawResource{
		{ID: 1, Language: 1033, CodePage: 0, Data: numericData},
		{Name: "1", Named: true, Language: 1033, CodePage: 1200, Data: namedData},
	}

	decoded, err := decodeEncyclopediaLookupBlocks(resources)
	if err != nil {
		t.Fatal(err)
	}
	if got := decoded.Lookups[1033][0]; got != "EDATA.001" {
		t.Fatalf("numeric block lookup = %q, want EDATA.001", got)
	}
	if got, want := len(decoded.Blocks), 2; got != want {
		t.Fatalf("block observation count = %d, want %d", got, want)
	}
	if decoded.Blocks[0].BlockID.Kind != encyclopediaIdentifierNumeric || decoded.Blocks[0].Status != encyclopediaRecordDecoded {
		t.Fatalf("numeric block observation = %#v", decoded.Blocks[0])
	}
	if decoded.Blocks[1].BlockID.Kind != encyclopediaIdentifierNamed || decoded.Blocks[1].BlockID.Name != "1" {
		t.Fatalf("named block identity = %#v", decoded.Blocks[1].BlockID)
	}
	if decoded.Blocks[1].Status != encyclopediaRecordUnresolved || decoded.Blocks[1].Unresolved == nil {
		t.Fatalf("named block status = %#v", decoded.Blocks[1])
	}
	if !bytes.Equal(decoded.Blocks[0].RawBytes, numericData) || !bytes.Equal(decoded.Blocks[1].RawBytes, namedData) {
		t.Fatal("raw RT_STRING bytes were not retained")
	}
	resources[0].Data[0] ^= 0xff
	if bytes.Equal(decoded.Blocks[0].RawBytes, resources[0].Data) {
		t.Fatal("retained RT_STRING bytes alias caller storage")
	}
}

func TestEncyclopediaLookupObservationsAreDeterministicAcrossResourceOrder(t *testing.T) {
	resources := []rawResource{
		{ID: 2, Language: 1033, CodePage: 0, Data: stringBundle(map[int]string{0: "EDATA.014"})},
		{ID: 1, Language: 1031, CodePage: 1252, Data: stringBundle(map[int]string{3: "EDATA.042"})},
		{Name: "A", Named: true, Language: 1033, CodePage: 0, Data: stringBundle(map[int]string{0: "EDATA.077"})},
	}
	forward, err := decodeEncyclopediaLookupBlocks(resources)
	if err != nil {
		t.Fatal(err)
	}
	reversedResources := append([]rawResource(nil), resources...)
	for left, right := 0, len(reversedResources)-1; left < right; left, right = left+1, right-1 {
		reversedResources[left], reversedResources[right] = reversedResources[right], reversedResources[left]
	}
	reversed, err := decodeEncyclopediaLookupBlocks(reversedResources)
	if err != nil {
		t.Fatal(err)
	}
	forwardJSON, err := json.Marshal(forward)
	if err != nil {
		t.Fatal(err)
	}
	reversedJSON, err := json.Marshal(reversed)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(forwardJSON, reversedJSON) {
		t.Fatalf("canonical lookup observations depend on PE resource order:\nforward: %s\nreverse: %s", forwardJSON, reversedJSON)
	}
}

func TestEncyclopediaLookupReconciliationReportsNonlinearMissingAndUnreferencedFiles(t *testing.T) {
	root := t.TempDir()
	writeTestFile(t, filepath.Join(root, "EDATA.014"), []byte("fourteen"))
	writeTestFile(t, filepath.Join(root, "EDATA.042"), []byte("forty-two"))
	writeTestFile(t, filepath.Join(root, "EDATA.077"), []byte("seventy-seven"))
	lookups := map[uint16]map[uint32]string{
		1033: {
			4736: "EDATA.014",
			21:   "EDATA.042",
			90:   "EDATA.999",
		},
	}

	reconciliation, err := reconcileEncyclopediaLookupFiles(lookups, root)
	if err != nil {
		t.Fatal(err)
	}
	if got, want := len(reconciliation.MissingFilenames), 1; got != want || reconciliation.MissingFilenames[0].Filename != "EDATA.999" {
		t.Fatalf("missing filenames = %#v, want EDATA.999", reconciliation.MissingFilenames)
	}
	if got, want := len(reconciliation.UnreferencedFiles), 1; got != want || reconciliation.UnreferencedFiles[0].Basename != "EDATA.077" {
		t.Fatalf("unreferenced files = %#v, want EDATA.077", reconciliation.UnreferencedFiles)
	}
	resolved := map[uint32]encyclopediaLookupReference{}
	for _, reference := range reconciliation.References {
		resolved[reference.LogicalID] = reference
	}
	if resolved[4736].FileNumber == nil || *resolved[4736].FileNumber != 14 || resolved[4736].Resolution != encyclopediaLookupExact {
		t.Fatalf("lookup 4736 reconciliation = %#v", resolved[4736])
	}
	if resolved[21].FileNumber == nil || *resolved[21].FileNumber != 42 {
		t.Fatalf("nonlinear lookup 21 reconciliation = %#v", resolved[21])
	}
	if resolved[90].FileNumber == nil || *resolved[90].FileNumber != 999 || resolved[90].Resolution != encyclopediaLookupMissing {
		t.Fatalf("absent lookup 90 reconciliation = %#v", resolved[90])
	}
	var file14 encyclopediaLookupFileRecord
	for _, file := range reconciliation.Files {
		if file.Basename == "EDATA.014" {
			file14 = file
			break
		}
	}
	if file14.RawLength != 8 || file14.RawSHA256 != "fef8f41cdcd2038663b027a0cfbe252d39510bc162b599d1d4aa683873cc21d7" {
		t.Fatalf("EDATA.014 metadata = %#v, want length 8 and exact SHA-256", file14)
	}
}

func TestEncyclopediaLookupReconciliationStreamsLargeFileMetadataWithBoundedAllocation(t *testing.T) {
	root := t.TempDir()
	path := filepath.Join(root, "EDATA.001")
	file, err := os.Create(path)
	if err != nil {
		t.Fatal(err)
	}
	if err := file.Truncate(64 << 20); err != nil {
		file.Close()
		t.Fatal(err)
	}
	if err := file.Close(); err != nil {
		t.Fatal(err)
	}

	runtime.GC()
	var before, after runtime.MemStats
	runtime.ReadMemStats(&before)
	reconciliation, err := reconcileEncyclopediaLookupFiles(nil, root)
	runtime.ReadMemStats(&after)
	if err != nil {
		t.Fatal(err)
	}
	if got, want := len(reconciliation.Files), 1; got != want || reconciliation.Files[0].RawLength != 64<<20 {
		t.Fatalf("large-file metadata = %#v, want one 64 MiB record", reconciliation.Files)
	}
	if allocated := after.TotalAlloc - before.TotalAlloc; allocated > 16<<20 {
		t.Fatalf("metadata hashing allocated %d bytes for one 64 MiB file", allocated)
	}
}

func TestEncyclopediaLookupReconciliationRejectsSymlinkAndNonregularEDataEntries(t *testing.T) {
	t.Run("symlink", func(t *testing.T) {
		root := t.TempDir()
		target := filepath.Join(root, "target")
		writeTestFile(t, target, []byte("target"))
		if err := os.Symlink(target, filepath.Join(root, "EDATA.001")); err != nil {
			t.Skipf("cannot create symlink on this platform: %v", err)
		}
		if _, err := reconcileEncyclopediaLookupFiles(nil, root); err == nil || !strings.Contains(err.Error(), "symlink") {
			t.Fatalf("symlink error = %v, want explicit rejection", err)
		}
	})

	t.Run("directory", func(t *testing.T) {
		root := t.TempDir()
		if err := os.Mkdir(filepath.Join(root, "EDATA.002"), 0o700); err != nil {
			t.Fatal(err)
		}
		if _, err := reconcileEncyclopediaLookupFiles(nil, root); err == nil || !strings.Contains(err.Error(), "regular file") {
			t.Fatalf("directory error = %v, want nonregular rejection", err)
		}
	})
}

func TestEncyclopediaLookupReconciliationRecordsDuplicateReferencesAndCaseAmbiguity(t *testing.T) {
	root := t.TempDir()
	writeTestFile(t, filepath.Join(root, "EDATA.014"), []byte("upper"))
	writeTestFile(t, filepath.Join(root, "edata.014"), []byte("lower"))
	writeTestFile(t, filepath.Join(root, "EDATA.077"), []byte("unrelated"))
	lookups := map[uint16]map[uint32]string{
		1033: {1: "EDATA.014", 2: "EDATA.014"},
		1031: {1: "edata.014"},
	}

	reconciliation, err := reconcileEncyclopediaLookupFiles(lookups, root)
	if err != nil {
		t.Fatal(err)
	}
	if got, want := len(reconciliation.DuplicateReferences), 1; got != want {
		t.Fatalf("duplicate reference groups = %d, want %d: %#v", got, want, reconciliation.DuplicateReferences)
	}
	if got, want := len(reconciliation.DuplicateReferences[0].References), 3; got != want {
		t.Fatalf("duplicate reference count = %d, want %d", got, want)
	}
	if got, want := len(reconciliation.CaseAmbiguities), 1; got != want {
		t.Fatalf("case ambiguities = %d, want %d: %#v", got, want, reconciliation.CaseAmbiguities)
	}
	if got, want := reconciliation.CaseAmbiguities[0].Candidates, []string{"EDATA.014", "edata.014"}; fmt.Sprint(got) != fmt.Sprint(want) {
		t.Fatalf("case ambiguity candidates = %#v, want %#v", got, want)
	}
	for _, reference := range reconciliation.References {
		if reference.Resolution != encyclopediaLookupAmbiguous {
			t.Fatalf("ambiguous reference was reported as %q", reference.Resolution)
		}
	}
	if got, want := len(reconciliation.UnreferencedFiles), 1; got != want || reconciliation.UnreferencedFiles[0].Basename != "EDATA.077" {
		t.Fatalf("unreferenced files = %#v, want only unrelated EDATA.077", reconciliation.UnreferencedFiles)
	}
}

func TestEncyclopediaLookupReconciliationRejectsUnsafeOrUnnumberedNamesWithoutInventingIDs(t *testing.T) {
	root := t.TempDir()
	writeTestFile(t, filepath.Join(root, "EDATA.001"), []byte("one"))
	for _, filename := range []string{"../EDATA.001", "subdir/EDATA.001", "EDATA.ONE"} {
		t.Run(filename, func(t *testing.T) {
			_, err := reconcileEncyclopediaLookupFiles(map[uint16]map[uint32]string{1033: {1: filename}}, root)
			if err == nil {
				t.Fatalf("lookup filename %q was accepted", filename)
			}
		})
	}
}

func TestOwnedEncyclopediaLookupInventoryPreservesInputsAndRetainsIgnoredEvidence(t *testing.T) {
	sourceRoot := os.Getenv("REBELLION_ENCYCLOPEDIA_TEST_SOURCE")
	if sourceRoot == "" {
		t.Skip("set REBELLION_ENCYCLOPEDIA_TEST_SOURCE to an owned installation root")
	}
	absoluteRoot, err := filepath.Abs(sourceRoot)
	if err != nil {
		t.Fatal(err)
	}
	dllBasename, ok := findCaseInsensitiveFile(t, absoluteRoot, "ENCYBMAP.DLL")
	if !ok {
		t.Fatalf("%s does not contain ENCYBMAP.DLL", absoluteRoot)
	}
	edataBasename, ok := findCaseInsensitiveDirectory(t, absoluteRoot, "EData")
	if !ok {
		t.Fatalf("%s does not contain EData", absoluteRoot)
	}
	dllPath := filepath.Join(absoluteRoot, dllBasename)
	edataRoot := filepath.Join(absoluteRoot, edataBasename)
	dllBefore, err := os.ReadFile(dllPath)
	if err != nil {
		t.Fatal(err)
	}
	if got, want := byteSHA256(dllBefore), identifiedEnglishEncybmapSHA256; got != want {
		t.Fatalf("owned ENCYBMAP identity = %s, want identified profile %s", got, want)
	}

	inventory, err := inventoryEncyclopediaLookups(dllBasename, dllBefore, edataRoot)
	if err != nil {
		t.Fatal(err)
	}
	dllAfter, err := os.ReadFile(dllPath)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(dllBefore, dllAfter) {
		t.Fatal("owned ENCYBMAP input changed during lookup inventory")
	}
	for _, asset := range inventory.Files {
		path := filepath.Join(edataRoot, asset.Basename)
		data, err := os.ReadFile(path)
		if err != nil {
			t.Fatal(err)
		}
		if uint64(len(data)) != asset.RawLength || byteSHA256(data) != asset.RawSHA256 {
			t.Fatalf("owned EData input %s changed during lookup inventory", asset.Basename)
		}
	}

	english := inventory.Lookups[1033]
	if got, want := english[4736], "EDATA.014"; got != want {
		t.Fatalf("identified profile lookup 4736 = %q, want %q", got, want)
	}
	if got, want := len(inventory.Blocks), 31; got != want {
		t.Fatalf("identified profile block count = %d, want observed %d", got, want)
	}
	for _, block := range inventory.Blocks {
		if block.BlockID.Kind != encyclopediaIdentifierNumeric || block.LanguageID != 1033 || block.CodePage != 0 || block.Status != encyclopediaRecordDecoded {
			t.Fatalf("identified profile block metadata is outside observed numeric/LANGID/code-page facts: %#v", block)
		}
	}
	if got, want := len(english), 191; got != want {
		t.Fatalf("identified profile nonempty string count = %d, want observed %d", got, want)
	}
	distinctFilenames := make(map[string]struct{})
	for _, filename := range english {
		distinctFilenames[strings.ToLower(filename)] = struct{}{}
	}
	if got, want := len(distinctFilenames), 186; got != want {
		t.Fatalf("identified profile distinct filename count = %d, want observed %d", got, want)
	}
	if len(inventory.MissingFilenames) != 0 || len(inventory.CaseAmbiguities) != 0 {
		t.Fatalf("identified profile reconciliation has missing=%#v ambiguous=%#v", inventory.MissingFilenames, inventory.CaseAmbiguities)
	}
	if got, want := len(inventory.Files), 187; got != want {
		t.Fatalf("identified profile EData count = %d, want observed %d", got, want)
	}
	if got, want := len(inventory.UnreferencedFiles), 1; got != want || !strings.EqualFold(inventory.UnreferencedFiles[0].Basename, "EDATA.192") {
		t.Fatalf("identified profile unreferenced files = %#v, want inventory-only EDATA.192", inventory.UnreferencedFiles)
	}
	duplicateReferenceExcess := 0
	for _, duplicate := range inventory.DuplicateReferences {
		duplicateReferenceExcess += len(duplicate.References) - 1
	}
	if got, want := duplicateReferenceExcess, 5; got != want {
		t.Fatalf("identified profile duplicate reference excess = %d, want observed %d", got, want)
	}

	assetLines := make([]string, 0, len(inventory.Files))
	for _, asset := range inventory.Files {
		assetLines = append(assetLines, fmt.Sprintf("%s|%d|%s", asset.Basename, asset.RawLength, asset.RawSHA256))
	}
	sort.Strings(assetLines)
	evidence := struct {
		SchemaVersion            int    `json:"schema_version"`
		SourceBasename           string `json:"source_basename"`
		SourceSHA256             string `json:"source_sha256"`
		BlockCount               int    `json:"block_count"`
		LanguageID               uint16 `json:"language_id"`
		CodePage                 uint32 `json:"code_page"`
		NonemptyStringCount      int    `json:"nonempty_string_count"`
		DistinctFilenameCount    int    `json:"distinct_filename_count"`
		Lookup4736               string `json:"lookup_4736"`
		EDataFileCount           int    `json:"edata_file_count"`
		EDataIdentitySetSHA256   string `json:"edata_identity_set_sha256"`
		MissingFilenameCount     int    `json:"missing_filename_count"`
		UnreferencedFileCount    int    `json:"unreferenced_file_count"`
		DuplicateGroupCount      int    `json:"duplicate_reference_group_count"`
		DuplicateReferenceExcess int    `json:"duplicate_reference_excess_count"`
		CaseAmbiguityCount       int    `json:"case_ambiguity_count"`
	}{
		SchemaVersion:            1,
		SourceBasename:           dllBasename,
		SourceSHA256:             byteSHA256(dllBefore),
		BlockCount:               len(inventory.Blocks),
		LanguageID:               1033,
		CodePage:                 0,
		NonemptyStringCount:      len(english),
		DistinctFilenameCount:    len(distinctFilenames),
		Lookup4736:               english[4736],
		EDataFileCount:           len(inventory.Files),
		EDataIdentitySetSHA256:   byteSHA256([]byte(strings.Join(assetLines, "\n") + "\n")),
		MissingFilenameCount:     len(inventory.MissingFilenames),
		UnreferencedFileCount:    len(inventory.UnreferencedFiles),
		DuplicateGroupCount:      len(inventory.DuplicateReferences),
		DuplicateReferenceExcess: duplicateReferenceExcess,
		CaseAmbiguityCount:       len(inventory.CaseAmbiguities),
	}
	evidenceBytes, err := json.MarshalIndent(evidence, "", "  ")
	if err != nil {
		t.Fatal(err)
	}
	repoRoot, err := filepath.Abs(filepath.Join("..", ".."))
	if err != nil {
		t.Fatal(err)
	}
	evidenceDir := filepath.Join(repoRoot, ".artifacts", "encyclopedia")
	if err := os.MkdirAll(evidenceDir, 0o700); err != nil {
		t.Fatal(err)
	}
	evidencePath := filepath.Join(evidenceDir, "E05-owned-lookups.json")
	writeTestFile(t, evidencePath, append(evidenceBytes, '\n'))
	t.Logf("retained metadata-only lookup evidence at %s", evidencePath)
}

func invalidEncyclopediaStringBlock(codeUnit uint16) []byte {
	data := binary.LittleEndian.AppendUint16(nil, 1)
	data = binary.LittleEndian.AppendUint16(data, codeUnit)
	for slot := 1; slot < 16; slot++ {
		data = binary.LittleEndian.AppendUint16(data, 0)
	}
	return data
}

func findCaseInsensitiveDirectory(t *testing.T, root, wanted string) (string, bool) {
	t.Helper()
	entries, err := os.ReadDir(root)
	if err != nil {
		t.Fatal(err)
	}
	match := ""
	for _, entry := range entries {
		if !entry.IsDir() || !strings.EqualFold(entry.Name(), wanted) {
			continue
		}
		if match != "" {
			t.Fatalf("ambiguous case-insensitive directory %q under %s", wanted, root)
		}
		match = entry.Name()
	}
	return match, match != ""
}
