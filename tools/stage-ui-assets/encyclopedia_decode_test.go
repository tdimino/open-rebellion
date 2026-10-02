package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"testing"
	"time"
	"unicode/utf8"
)

const testEncyclopediaProfileSourceSHA256 = "49aea545a5e09e5fe9115a22bc785690f103d2f931e08bd4a53a617a42636d8c"

func TestEncyclopediaDecodesProvenNonASCIIWithoutLoss(t *testing.T) {
	raw := []byte{'P', 'i', 'l', 'o', 't', 0x92, 's', '\t', 'n', 'o', 't', 'e', '\n', 0}
	report := testEncyclopediaDecodeReport(raw)

	decoded, err := decodeEncyclopediaReport(report)
	if err != nil {
		t.Fatalf("decodeEncyclopediaReport() error = %v", err)
	}
	if got, want := len(decoded.Records), 1; got != want {
		t.Fatalf("decoded record count = %d, want %d", got, want)
	}
	record := decoded.Records[0]
	if got, want := record.Text, "Pilot’s\tnote\n"; got != want {
		t.Fatalf("decoded text = %q, want %q", got, want)
	}
	if got, want := record.Encoding, "windows-1252"; got != want {
		t.Errorf("encoding = %q, want %q", got, want)
	}
	if got, want := record.ProfileID, "encytext-49aea545-lang-1033-v1"; got != want {
		t.Errorf("profile ID = %q, want %q", got, want)
	}
	if !bytes.Equal(record.RawBytes, raw) || record.RawLength != uint64(len(raw)) || record.RawSHA256 != byteSHA256(raw) {
		t.Error("decoded record did not retain its exact raw bytes, length, and hash")
	}
	if record.Status != encyclopediaRecordDecoded {
		t.Errorf("decoded status = %q, want %q", record.Status, encyclopediaRecordDecoded)
	}
	if report.Records[0].Status != encyclopediaRecordInventoried {
		t.Error("decoding mutated the input research record")
	}
	if got := encodeSyntheticWindows1252(t, record.Text); !bytes.Equal(got, raw[:len(raw)-1]) {
		t.Fatalf("round-trip bytes = %x, want %x", got, raw[:len(raw)-1])
	}
}

func TestEncyclopediaDecoderPreservesMeaningfulWhitespace(t *testing.T) {
	raw := []byte("  heading  \n\n\tindented body \t \n  \x00")
	decoded, err := decodeEncyclopediaReport(testEncyclopediaDecodeReport(raw))
	if err != nil {
		t.Fatalf("decodeEncyclopediaReport() error = %v", err)
	}
	if got, want := decoded.Records[0].Text, "  heading  \n\n\tindented body \t \n  "; got != want {
		t.Fatalf("meaningful whitespace changed: got %q, want %q", got, want)
	}
}

func TestEncyclopediaDecoderRemovesOnlyProvenTerminalNULPadding(t *testing.T) {
	for _, test := range []struct {
		name string
		raw  []byte
		want string
	}{
		{name: "one terminal NUL", raw: []byte{'a', 0}, want: "a"},
		{name: "one zero padding byte after terminator", raw: []byte{'a', 0, 0}, want: "a"},
	} {
		t.Run(test.name, func(t *testing.T) {
			decoded, err := decodeEncyclopediaReport(testEncyclopediaDecodeReport(test.raw))
			if err != nil {
				t.Fatalf("decodeEncyclopediaReport() error = %v", err)
			}
			if got := decoded.Records[0].Text; got != test.want {
				t.Fatalf("text = %q, want %q", got, test.want)
			}
		})
	}

	for _, test := range []struct {
		name string
		raw  []byte
	}{
		{name: "missing terminator", raw: []byte("a")},
		{name: "more padding than the profile proves", raw: []byte{'a', 0, 0, 0}},
		{name: "nonzero bytes after terminator", raw: []byte{'a', 0, 'b', 0}},
	} {
		t.Run(test.name, func(t *testing.T) {
			if _, err := decodeEncyclopediaReport(testEncyclopediaDecodeReport(test.raw)); err == nil {
				t.Fatal("malformed terminal NUL/padding was accepted")
			}
		})
	}
}

func TestEncyclopediaDecoderRejectsUnknownProfileOrControlBytes(t *testing.T) {
	t.Run("unknown source hash", func(t *testing.T) {
		report := testEncyclopediaDecodeReport([]byte{'a', 0})
		report.Sources[0].RawSHA256 = strings.Repeat("0", 64)
		if _, err := decodeEncyclopediaReport(report); err == nil {
			t.Fatal("unknown source profile was decoded")
		}
	})

	for _, test := range []struct {
		name string
		raw  []byte
	}{
		{name: "unproved C0 control", raw: []byte{'a', 0x01, 0}},
		{name: "bare carriage return", raw: []byte{'a', '\r', 0}},
		{name: "unproved CRLF newline", raw: []byte{'a', '\r', '\n', 0}},
		{name: "DEL control", raw: []byte{'a', 0x7f, 0}},
		{name: "undefined Windows-1252 byte", raw: []byte{'a', 0x81, 0}},
	} {
		t.Run(test.name, func(t *testing.T) {
			if _, err := decodeEncyclopediaReport(testEncyclopediaDecodeReport(test.raw)); err == nil {
				t.Fatal("unknown control or encoding byte was accepted")
			}
		})
	}
}

func TestEncyclopediaDecoderRejectsMalformedRecordFacts(t *testing.T) {
	for _, mutate := range []struct {
		name string
		do   func(*encyclopediaResearchReport)
	}{
		{name: "raw length mismatch", do: func(report *encyclopediaResearchReport) { report.Records[0].RawLength++ }},
		{name: "raw hash mismatch", do: func(report *encyclopediaResearchReport) { report.Records[0].RawSHA256 = strings.Repeat("f", 64) }},
		{name: "raw bytes unavailable", do: func(report *encyclopediaResearchReport) { report.Records[0].RawBytes = nil }},
		{name: "unsupported language", do: func(report *encyclopediaResearchReport) { report.Records[0].LanguageID = 1031 }},
		{name: "unproved PE code page", do: func(report *encyclopediaResearchReport) { report.Records[0].CodePage = 1252 }},
		{name: "explicitly unresolved record", do: func(report *encyclopediaResearchReport) {
			report.Records[0].Status = encyclopediaRecordUnresolved
			report.Records[0].Unresolved = &encyclopediaUnresolvedStatus{
				Reason:    "encoding evidence is incomplete",
				NextProof: "inspect the original caller",
			}
		}},
		{name: "wrong resource type", do: func(report *encyclopediaResearchReport) {
			report.Records[0].ResourceType = numericEncyclopediaResourceIdentifier(6)
		}},
	} {
		t.Run(mutate.name, func(t *testing.T) {
			report := testEncyclopediaDecodeReport([]byte{'a', 0})
			mutate.do(&report)
			if _, err := decodeEncyclopediaReport(report); err == nil {
				t.Fatal("malformed or unsupported record facts were accepted")
			}
		})
	}
}

func TestEmbeddedEncyclopediaProfileResolvesOutsideRepositoryWorkingDirectory(t *testing.T) {
	original, err := os.Getwd()
	if err != nil {
		t.Fatal(err)
	}
	if err := os.Chdir(t.TempDir()); err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() {
		if err := os.Chdir(original); err != nil {
			t.Errorf("restore working directory: %v", err)
		}
	})

	decoded, err := decodeEncyclopediaReport(testEncyclopediaDecodeReport([]byte{'a', 0}))
	if err != nil {
		t.Fatalf("embedded profile failed outside repository working directory: %v", err)
	}
	if got, want := decoded.ProfileID, "encytext-49aea545-lang-1033-v1"; got != want {
		t.Fatalf("profile ID = %q, want %q", got, want)
	}
}

func TestOwnedEncyclopediaDecoderCorroboratesProfileWithoutChangingInputs(t *testing.T) {
	sourceRoot := os.Getenv("REBELLION_ENCYCLOPEDIA_TEST_SOURCE")
	if sourceRoot == "" {
		t.Skip("set REBELLION_ENCYCLOPEDIA_TEST_SOURCE to an owned installation root")
	}
	absoluteRoot, err := filepath.Abs(sourceRoot)
	if err != nil {
		t.Fatal(err)
	}
	basename, ok := findCaseInsensitiveFile(t, absoluteRoot, "ENCYTEXT.DLL")
	if !ok {
		t.Fatalf("%s does not contain ENCYTEXT.DLL", absoluteRoot)
	}
	path := filepath.Join(absoluteRoot, basename)
	before, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}

	inventory, err := inventoryEncyclopediaSources(encyclopediaInventoryRequest{
		Roots: []encyclopediaSourceRoot{{Role: "install", Path: absoluteRoot}},
		Sources: []encyclopediaSourceSpec{{
			RootRole:       "install",
			Basename:       basename,
			Kind:           encyclopediaSourceDLL,
			ResourceTypeID: rtEncyclopediaText,
		}},
		StartedAt: time.Unix(1, 0),
	}, defaultEncyclopediaInventoryLimits())
	if err != nil {
		t.Fatal(err)
	}
	decoded, err := decodeEncyclopediaReport(inventory.Report)
	if err != nil {
		t.Fatal(err)
	}
	after, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(before, after) {
		t.Fatal("owned ENCYTEXT input changed during inventory or decoding")
	}

	if got, want := len(decoded.Records), 348; got != want {
		t.Fatalf("identified profile record count = %d, want observed %d", got, want)
	}
	nonASCIIRecords := 0
	nonASCIIOccurrences := 0
	decodedNonASCIIOccurrences := 0
	terminalNULHistogram := map[int]int{}
	lineFeeds := 0
	tabs := 0
	recordSetLines := make([]string, 0)
	for _, record := range decoded.Records {
		if !utf8.ValidString(record.Text) || strings.ContainsRune(record.Text, utf8.RuneError) {
			t.Fatalf("resource %s did not decode to lossless UTF-8", formatEncyclopediaResourceIdentifier(record.ResourceID))
		}
		occurrences := bytes.Count(record.RawBytes, []byte{0x92})
		rawNonASCII := 0
		for _, value := range record.RawBytes {
			if value >= 0x80 {
				rawNonASCII++
			}
		}
		if rawNonASCII != occurrences {
			t.Fatalf("resource %s contains a non-ASCII byte outside the recovered profile mapping", formatEncyclopediaResourceIdentifier(record.ResourceID))
		}
		decodedNonASCIIOccurrences += strings.Count(record.Text, "’")
		lineFeeds += bytes.Count(record.RawBytes, []byte{'\n'})
		tabs += bytes.Count(record.RawBytes, []byte{'\t'})
		terminalNULs := 0
		for index := len(record.RawBytes) - 1; index >= 0 && record.RawBytes[index] == 0; index-- {
			terminalNULs++
		}
		terminalNULHistogram[terminalNULs]++
		if occurrences > 0 {
			nonASCIIRecords++
			nonASCIIOccurrences += occurrences
			recordSetLines = append(recordSetLines, fmt.Sprintf("%d|%d|%d|%s", *record.ResourceID.NumericID, record.LanguageID, record.RawLength, record.RawSHA256))
		}
	}
	sort.Strings(recordSetLines)
	nonASCIISetSHA256 := byteSHA256([]byte(strings.Join(recordSetLines, "\n") + "\n"))
	if nonASCIIRecords != 29 || nonASCIIOccurrences != 32 || decodedNonASCIIOccurrences != 32 || nonASCIISetSHA256 != "8f52e5969ecfc65484227ece50ec87ed02cdc39f0203f42c5dbf4b25ccd029d3" {
		t.Fatalf("non-ASCII corroboration = records %d, raw occurrences %d, decoded occurrences %d, digest %s", nonASCIIRecords, nonASCIIOccurrences, decodedNonASCIIOccurrences, nonASCIISetSHA256)
	}
	if terminalNULHistogram[1] != 346 || terminalNULHistogram[2] != 2 || len(terminalNULHistogram) != 2 || lineFeeds != 956 || tabs != 1819 {
		t.Fatalf("control corroboration = terminal NULs %v, LF %d, tabs %d", terminalNULHistogram, lineFeeds, tabs)
	}

	evidence := struct {
		SchemaVersion           int    `json:"schema_version"`
		ProfileID               string `json:"profile_id"`
		SourceBasename          string `json:"source_basename"`
		SourceSHA256            string `json:"source_sha256"`
		RecordCount             int    `json:"record_count"`
		NonASCIIRecordCount     int    `json:"non_ascii_record_count"`
		NonASCIIOccurrenceCount int    `json:"non_ascii_occurrence_count"`
		NonASCIIRecordSetSHA256 string `json:"non_ascii_record_set_sha256"`
		LFCount                 int    `json:"lf_count"`
		TabCount                int    `json:"tab_count"`
		OneTerminalNULRecords   int    `json:"one_terminal_nul_records"`
		TwoTerminalNULRecords   int    `json:"two_terminal_nul_records"`
		UnresolvedRecordCount   int    `json:"unresolved_record_count"`
	}{
		SchemaVersion:           1,
		ProfileID:               decoded.ProfileID,
		SourceBasename:          basename,
		SourceSHA256:            byteSHA256(before),
		RecordCount:             len(decoded.Records),
		NonASCIIRecordCount:     nonASCIIRecords,
		NonASCIIOccurrenceCount: nonASCIIOccurrences,
		NonASCIIRecordSetSHA256: nonASCIISetSHA256,
		LFCount:                 lineFeeds,
		TabCount:                tabs,
		OneTerminalNULRecords:   terminalNULHistogram[1],
		TwoTerminalNULRecords:   terminalNULHistogram[2],
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
	writeTestFile(t, filepath.Join(evidenceDir, "E02-owned-decoder.json"), append(evidenceBytes, '\n'))
	t.Logf("retained metadata-only decoder evidence under %s", evidenceDir)
}

func TestOwnedEncyclopediaCatalogBuildsDeterministicallyFromPairedSources(t *testing.T) {
	sourceRoot := os.Getenv("REBELLION_ENCYCLOPEDIA_TEST_SOURCE")
	if sourceRoot == "" {
		t.Skip("set REBELLION_ENCYCLOPEDIA_TEST_SOURCE to an owned installation root")
	}
	absoluteRoot, err := filepath.Abs(sourceRoot)
	if err != nil {
		t.Fatal(err)
	}
	textName, ok := findCaseInsensitiveFile(t, absoluteRoot, "ENCYTEXT.DLL")
	if !ok {
		t.Fatal("owned source missing ENCYTEXT.DLL")
	}
	titleName, ok := findCaseInsensitiveFile(t, absoluteRoot, "TEXTSTRA.DLL")
	if !ok {
		t.Fatal("owned source missing TEXTSTRA.DLL")
	}
	edataName, ok := findCaseInsensitiveDirectory(t, absoluteRoot, "EData")
	if !ok {
		t.Fatal("owned source missing EData")
	}

	inventory, err := inventoryEncyclopediaSources(encyclopediaInventoryRequest{
		Roots: []encyclopediaSourceRoot{{Role: "install", Path: absoluteRoot}},
		Sources: []encyclopediaSourceSpec{{
			RootRole: "install", Basename: textName, Kind: encyclopediaSourceDLL, ResourceTypeID: rtEncyclopediaText,
		}},
		StartedAt: time.Unix(1, 0),
	}, defaultEncyclopediaInventoryLimits())
	if err != nil {
		t.Fatal(err)
	}
	decoded, err := decodeEncyclopediaReport(inventory.Report)
	if err != nil {
		t.Fatal(err)
	}
	profiles, err := loadEmbeddedEncyclopediaProfiles()
	if err != nil {
		t.Fatal(err)
	}
	profile := profiles[0]
	dataRoot := absoluteRoot
	if gdataName, found := findCaseInsensitiveDirectory(t, absoluteRoot, "GData"); found {
		dataRoot = filepath.Join(absoluteRoot, gdataName)
	}
	pairingManifest := encyclopediaManifest{}
	observedDATHashes := make(map[string]string)
	for _, source := range profile.Sources {
		if source.Kind != encyclopediaSourceDAT {
			continue
		}
		basename, found := findCaseInsensitiveFile(t, dataRoot, source.Basename)
		if !found {
			basename, found = findCaseInsensitiveFile(t, absoluteRoot, source.Basename)
		}
		if !found {
			t.Fatalf("owned source missing %s", source.Basename)
		}
		filePath := filepath.Join(dataRoot, basename)
		if _, err := os.Stat(filePath); err != nil {
			filePath = filepath.Join(absoluteRoot, basename)
		}
		digest, err := fileSHA256(filePath)
		if err != nil {
			t.Fatal(err)
		}
		pairingManifest.BindingSources = append(pairingManifest.BindingSources, encyclopediaManifestBindingSource{Basename: source.Basename, SHA256: source.RawSHA256})
		observedDATHashes[basename] = digest
	}
	if err := verifyBindingSources(pairingManifest, observedDATHashes); err != nil {
		t.Fatalf("owned DAT source pairing failed: %v", err)
	}

	titleResources, err := readPERawResources(filepath.Join(absoluteRoot, titleName), 6)
	if err != nil {
		t.Fatal(err)
	}
	titles, err := decodeStringResources(titleResources)
	if err != nil {
		t.Fatal(err)
	}
	imageInventory, err := inventoryEncyclopediaImages(filepath.Join(absoluteRoot, edataName), defaultEncyclopediaImageLimits())
	if err != nil {
		t.Fatal(err)
	}
	mappings := encyclopediaCatalogMappings{
		Titles:         make(map[encyclopediaCatalogResourceKey]string),
		CategoryLabels: make(map[uint32]map[uint32]string),
		Images:         make(map[string]encyclopediaCatalogImageInput),
		SourceHashes:   make(map[string]string),
	}
	titleDigest, err := fileSHA256(filepath.Join(absoluteRoot, titleName))
	if err != nil {
		t.Fatal(err)
	}
	mappings.SourceHashes["display_strings"] = titleDigest
	for _, record := range profile.Bindings.Records {
		value, exists := titles[uint16(record.Title.SelectedResourceID)]
		if !exists {
			t.Fatalf("selected title resource %d is absent", record.Title.SelectedResourceID)
		}
		mappings.Titles[encyclopediaCatalogResourceKey{SourceRole: record.Title.SourceRole, LanguageID: record.Title.LanguageID, ResourceID: record.Title.SelectedResourceID}] = value
	}
	for _, label := range profile.Catalog.CategoryLabels {
		value, exists := titles[uint16(label.ResourceID)]
		if !exists {
			t.Fatalf("category label resource %d is absent", label.ResourceID)
		}
		mappings.CategoryLabels[label.Command] = map[uint32]string{label.LanguageID: value}
	}
	for _, image := range imageInventory.Images {
		if image.Status != encyclopediaImageValid {
			t.Fatalf("owned image %s is not valid: %s", image.Filename, image.Diagnostic)
		}
		mappings.Images[image.Filename] = encyclopediaCatalogImageInput{
			Path: "assets/" + image.Filename, Format: "bmp", ByteLength: image.RawLength,
			Width: image.Facts.Width, Height: image.Facts.Height, SHA256: image.RawSHA256,
			SourceRef: "edata/" + image.Filename,
		}
	}

	first, err := buildEncyclopediaCatalog(profile, decoded, mappings)
	if err != nil {
		t.Fatal(err)
	}
	second, err := buildEncyclopediaCatalog(profile, decoded, mappings)
	if err != nil {
		t.Fatal(err)
	}
	firstBytes, err := marshalEncyclopediaCatalog(first)
	if err != nil {
		t.Fatal(err)
	}
	secondBytes, err := marshalEncyclopediaCatalog(second)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(firstBytes, secondBytes) {
		t.Fatal("owned catalog construction was not byte deterministic")
	}
	if got, want := len(first.Topics), 347; got != want {
		t.Fatalf("catalog topic count = %d, want %d", got, want)
	}
	if _, published := first.Topics[topicIDForResource(7176)]; published {
		t.Fatal("source-proven-unused ENCYTEXT resource 7176 was published as a topic")
	}
}

func testEncyclopediaDecodeReport(raw []byte) encyclopediaResearchReport {
	return encyclopediaResearchReport{
		Kind:          encyclopediaResearchKind,
		SchemaVersion: encyclopediaResearchSchemaVersion,
		Sources: []encyclopediaSourceRecord{{
			RootRole:  "install",
			Basename:  "ENCYTEXT.DLL",
			Kind:      encyclopediaSourceDLL,
			RawLength: 145920,
			RawSHA256: testEncyclopediaProfileSourceSHA256,
		}},
		Records: []encyclopediaResearchRecord{{
			SourceRootRole: "install",
			SourceBasename: "ENCYTEXT.DLL",
			ResourceType:   numericEncyclopediaResourceIdentifier(rtEncyclopediaText),
			ResourceID:     numericEncyclopediaResourceIdentifier(100),
			LanguageID:     1033,
			CodePage:       0,
			RawLength:      uint64(len(raw)),
			RawSHA256:      byteSHA256(raw),
			Status:         encyclopediaRecordInventoried,
			RawBytes:       append([]byte(nil), raw...),
		}},
	}
}

func encodeSyntheticWindows1252(t *testing.T, value string) []byte {
	t.Helper()
	encoded := make([]byte, 0, len(value))
	for _, runeValue := range value {
		switch {
		case runeValue <= 0x7f:
			encoded = append(encoded, byte(runeValue))
		case runeValue == '\u2019':
			encoded = append(encoded, 0x92)
		default:
			t.Fatalf("test encoder has no mapping for U+%04X", runeValue)
		}
	}
	return encoded
}
