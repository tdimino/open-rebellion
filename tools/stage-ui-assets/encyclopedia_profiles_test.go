package main

import (
	"bytes"
	"encoding/binary"
	"encoding/json"
	"os"
	"path/filepath"
	"reflect"
	"sort"
	"strconv"
	"strings"
	"testing"
	"time"
	"unicode/utf16"
)

func TestEncyclopediaInventoryDistinguishesNamesNumbersLanguagesAndDuplicates(t *testing.T) {
	root := t.TempDir()
	dllBytes := buildTestPE32WithEncyclopediaResources(t)
	writeTestFile(t, filepath.Join(root, "ENCYTEXT.DLL"), dllBytes)

	inventory, err := inventoryEncyclopediaSources(encyclopediaInventoryRequest{
		Roots: []encyclopediaSourceRoot{{Role: "install", Path: root}},
		Sources: []encyclopediaSourceSpec{{
			RootRole:       "install",
			Basename:       "ENCYTEXT.DLL",
			Kind:           encyclopediaSourceDLL,
			ResourceTypeID: rtEncyclopediaText,
		}},
		StartedAt: time.Date(2026, 9, 28, 18, 0, 0, 0, time.UTC),
	}, defaultEncyclopediaInventoryLimits())
	if err != nil {
		t.Fatalf("inventoryEncyclopediaSources() error = %v", err)
	}

	if got, want := len(inventory.Report.Records), 4; got != want {
		t.Fatalf("record count = %d, want %d", got, want)
	}

	type observedIdentity struct {
		kind       encyclopediaResourceIdentifierKind
		numericID  uint32
		name       string
		languageID uint32
		codePage   uint32
		duplicate  bool
		raw        string
	}
	observed := make([]observedIdentity, 0, len(inventory.Report.Records))
	for _, record := range inventory.Report.Records {
		var numericID uint32
		if record.ResourceID.NumericID != nil {
			numericID = *record.ResourceID.NumericID
		}
		observed = append(observed, observedIdentity{
			kind:       record.ResourceID.Kind,
			numericID:  numericID,
			name:       record.ResourceID.Name,
			languageID: record.LanguageID,
			codePage:   record.CodePage,
			duplicate:  record.DuplicateIdentity,
			raw:        string(record.RawBytes),
		})
		if record.Status != encyclopediaRecordInventoried {
			t.Errorf("record status = %q, want %q", record.Status, encyclopediaRecordInventoried)
		}
		if record.RawLength != uint64(len(record.RawBytes)) || record.RawSHA256 != byteSHA256(record.RawBytes) {
			t.Errorf("raw facts do not match preserved bytes: %+v", record)
		}
	}

	want := []observedIdentity{
		{kind: encyclopediaIdentifierNumeric, numericID: 7, languageID: 1031, codePage: 850, raw: "de"},
		{kind: encyclopediaIdentifierNumeric, numericID: 7, languageID: 1033, codePage: 0, duplicate: true, raw: "first"},
		{kind: encyclopediaIdentifierNumeric, numericID: 7, languageID: 1033, codePage: 0, duplicate: true, raw: "second"},
		{kind: encyclopediaIdentifierNamed, name: "7", languageID: 1033, codePage: 1252, raw: "named"},
	}
	// Duplicate occurrences are canonically ordered by their digest, not by PE
	// directory order, so compare those two raw payloads as a set.
	if len(observed) == len(want) && observed[1].raw > observed[2].raw {
		observed[1], observed[2] = observed[2], observed[1]
	}
	if want[1].raw > want[2].raw {
		want[1], want[2] = want[2], want[1]
	}
	if !reflect.DeepEqual(observed, want) {
		t.Fatalf("observed identities = %#v, want %#v", observed, want)
	}

	if got, want := len(inventory.Report.DuplicateIdentities), 1; got != want {
		t.Fatalf("duplicate identity count = %d, want %d", got, want)
	}
	duplicate := inventory.Report.DuplicateIdentities[0]
	if duplicate.ResourceID.Kind != encyclopediaIdentifierNumeric || duplicate.ResourceID.NumericID == nil || *duplicate.ResourceID.NumericID != 7 || duplicate.LanguageID != 1033 || duplicate.Occurrences != 2 {
		t.Fatalf("duplicate identity = %+v", duplicate)
	}
	if inventory.Report.Records[3].DuplicateIdentity {
		t.Fatal("named resource \"7\" collided with numeric resource 7")
	}
}

func TestEncyclopediaInventoryRecordsSourceIdentityAndPreservesOpaqueBytes(t *testing.T) {
	installRoot := t.TempDir()
	datRoot := t.TempDir()
	invalidUTF8 := []byte{0xff, 0x00, 0x81, '\n'}
	writeTestFile(t, filepath.Join(installRoot, "ENCYTEXT.DLL"), buildTestPE32WithResource(t, rtEncyclopediaText, 42, 1033, invalidUTF8))
	writeTestFile(t, filepath.Join(installRoot, "REBEXE.EXE"), []byte("synthetic executable identity"))
	writeTestFile(t, filepath.Join(datRoot, "CAPSHPSD.DAT"), []byte("synthetic dat identity"))

	inventory, err := inventoryEncyclopediaSources(encyclopediaInventoryRequest{
		Roots: []encyclopediaSourceRoot{
			{Role: "gdata", Path: datRoot},
			{Role: "install", Path: installRoot},
		},
		Sources: []encyclopediaSourceSpec{
			{RootRole: "gdata", Basename: "CAPSHPSD.DAT", Kind: encyclopediaSourceDAT},
			{RootRole: "install", Basename: "REBEXE.EXE", Kind: encyclopediaSourceEXE},
			{RootRole: "install", Basename: "ENCYTEXT.DLL", Kind: encyclopediaSourceDLL, ResourceTypeID: rtEncyclopediaText},
		},
		StartedAt: time.Date(2026, 9, 28, 18, 1, 2, 0, time.FixedZone("test", 2*60*60)),
	}, defaultEncyclopediaInventoryLimits())
	if err != nil {
		t.Fatalf("inventoryEncyclopediaSources() error = %v", err)
	}

	if got, want := len(inventory.Report.Sources), 3; got != want {
		t.Fatalf("source count = %d, want %d", got, want)
	}
	if inventory.Report.SourceCount != 3 || inventory.Report.RecordCount != 1 || inventory.Report.DuplicateIdentityCount != 0 {
		t.Fatalf("explicit report counts = sources %d, records %d, duplicates %d", inventory.Report.SourceCount, inventory.Report.RecordCount, inventory.Report.DuplicateIdentityCount)
	}
	for _, source := range inventory.Report.Sources {
		if source.Basename != filepath.Base(source.Basename) || source.RawLength == 0 || len(source.RawSHA256) != 64 {
			t.Errorf("incomplete source identity: %+v", source)
		}
	}
	if got := inventory.Report.Records[0].RawBytes; !bytes.Equal(got, invalidUTF8) {
		t.Fatalf("raw resource bytes = %v, want %v", got, invalidUTF8)
	}

	reportBytes, err := marshalEncyclopediaResearchReport(inventory.Report)
	if err != nil {
		t.Fatalf("marshalEncyclopediaResearchReport() error = %v", err)
	}
	if bytes.Contains(reportBytes, invalidUTF8) {
		t.Fatal("opaque source bytes leaked into the deterministic JSON report")
	}
	for _, forbidden := range []string{installRoot, datRoot, "2026-09-28T"} {
		if strings.Contains(string(reportBytes), forbidden) {
			t.Fatalf("deterministic report contains run context %q", forbidden)
		}
	}
	if got, want := inventory.RunLog.StartedAt, "2026-09-28T16:01:02Z"; got != want {
		t.Fatalf("run-log started_at = %q, want %q", got, want)
	}
	if len(inventory.RunLog.Roots) != 2 || !filepath.IsAbs(inventory.RunLog.Roots[0].Path) || !filepath.IsAbs(inventory.RunLog.Roots[1].Path) {
		t.Fatalf("run log did not retain explicit absolute roots: %+v", inventory.RunLog.Roots)
	}
}

func TestEncyclopediaResearchReportCanonicalizesInputOrder(t *testing.T) {
	root := t.TempDir()
	writeTestFile(t, filepath.Join(root, "ENCYTEXT.DLL"), buildTestPE32WithEncyclopediaResources(t))
	writeTestFile(t, filepath.Join(root, "REBEXE.EXE"), []byte("synthetic executable identity"))

	request := encyclopediaInventoryRequest{
		Roots: []encyclopediaSourceRoot{{Role: "install", Path: root}},
		Sources: []encyclopediaSourceSpec{
			{RootRole: "install", Basename: "REBEXE.EXE", Kind: encyclopediaSourceEXE},
			{RootRole: "install", Basename: "ENCYTEXT.DLL", Kind: encyclopediaSourceDLL, ResourceTypeID: rtEncyclopediaText},
		},
		StartedAt: time.Date(2026, 9, 28, 18, 0, 0, 0, time.UTC),
	}
	first, err := inventoryEncyclopediaSources(request, defaultEncyclopediaInventoryLimits())
	if err != nil {
		t.Fatal(err)
	}
	request.Sources[0], request.Sources[1] = request.Sources[1], request.Sources[0]
	request.StartedAt = request.StartedAt.Add(10 * time.Minute)
	second, err := inventoryEncyclopediaSources(request, defaultEncyclopediaInventoryLimits())
	if err != nil {
		t.Fatal(err)
	}

	firstBytes, err := marshalEncyclopediaResearchReport(first.Report)
	if err != nil {
		t.Fatal(err)
	}
	secondBytes, err := marshalEncyclopediaResearchReport(second.Report)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(firstBytes, secondBytes) {
		t.Fatalf("reports differ by request order/time:\nfirst:\n%s\nsecond:\n%s", firstBytes, secondBytes)
	}

	shuffled := first.Report
	shuffled.Sources[0], shuffled.Sources[1] = shuffled.Sources[1], shuffled.Sources[0]
	for left, right := 0, len(shuffled.Records)-1; left < right; left, right = left+1, right-1 {
		shuffled.Records[left], shuffled.Records[right] = shuffled.Records[right], shuffled.Records[left]
	}
	shuffledBytes, err := marshalEncyclopediaResearchReport(shuffled)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(firstBytes, shuffledBytes) {
		t.Fatalf("canonical marshaling retained caller order:\nfirst:\n%s\nshuffled:\n%s", firstBytes, shuffledBytes)
	}
}

func TestEncyclopediaResearchReportOrdersDuplicateStatusesCanonically(t *testing.T) {
	base := encyclopediaResearchRecord{
		SourceRootRole: "install",
		SourceBasename: "ENCYTEXT.DLL",
		ResourceType:   numericEncyclopediaResourceIdentifier(rtEncyclopediaText),
		ResourceID:     numericEncyclopediaResourceIdentifier(7),
		LanguageID:     1033,
		RawSHA256:      byteSHA256(nil),
		Status:         encyclopediaRecordInventoried,
	}
	unresolved := base
	unresolved.Status = encyclopediaRecordUnresolved
	unresolved.Unresolved = &encyclopediaUnresolvedStatus{
		Reason:    "semantic meaning is not established",
		NextProof: "trace the original caller",
	}
	report := encyclopediaResearchReport{
		Kind:          encyclopediaResearchKind,
		SchemaVersion: encyclopediaResearchSchemaVersion,
		Sources: []encyclopediaSourceRecord{{
			RootRole:  "install",
			Basename:  "ENCYTEXT.DLL",
			Kind:      encyclopediaSourceDLL,
			RawSHA256: byteSHA256(nil),
		}},
		Records: []encyclopediaResearchRecord{base, unresolved},
	}

	first, err := marshalEncyclopediaResearchReport(report)
	if err != nil {
		t.Fatal(err)
	}
	report.Records[0], report.Records[1] = report.Records[1], report.Records[0]
	second, err := marshalEncyclopediaResearchReport(report)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(first, second) {
		t.Fatalf("canonical report depends on duplicate status input order:\nfirst:\n%s\nsecond:\n%s", first, second)
	}
	if bytes.Count(first, []byte(`"status"`)) != 2 || !bytes.Contains(first, []byte(`"status": "inventoried"`)) || !bytes.Contains(first, []byte(`"status": "unresolved"`)) {
		t.Fatalf("canonical report did not retain both duplicate status records:\n%s", first)
	}
}

func TestEncyclopediaResearchReportOrdersDuplicateUnresolvedDetailsCanonically(t *testing.T) {
	base := encyclopediaResearchRecord{
		SourceRootRole: "install",
		SourceBasename: "ENCYTEXT.DLL",
		ResourceType:   numericEncyclopediaResourceIdentifier(rtEncyclopediaText),
		ResourceID:     numericEncyclopediaResourceIdentifier(7),
		LanguageID:     1033,
		RawSHA256:      byteSHA256(nil),
		Status:         encyclopediaRecordUnresolved,
		Unresolved: &encyclopediaUnresolvedStatus{
			Reason:    "alias relationship is unknown",
			NextProof: "trace the alias consumer",
		},
	}
	other := base
	other.Unresolved = &encyclopediaUnresolvedStatus{
		Reason:    "decoder semantics are unknown",
		NextProof: "compare independent decoder traces",
	}
	report := encyclopediaResearchReport{
		Kind:          encyclopediaResearchKind,
		SchemaVersion: encyclopediaResearchSchemaVersion,
		Sources: []encyclopediaSourceRecord{{
			RootRole:  "install",
			Basename:  "ENCYTEXT.DLL",
			Kind:      encyclopediaSourceDLL,
			RawSHA256: byteSHA256(nil),
		}},
		Records: []encyclopediaResearchRecord{base, other},
	}

	first, err := marshalEncyclopediaResearchReport(report)
	if err != nil {
		t.Fatal(err)
	}
	report.Records[0], report.Records[1] = report.Records[1], report.Records[0]
	second, err := marshalEncyclopediaResearchReport(report)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(first, second) {
		t.Fatalf("canonical report depends on unresolved evidence input order:\nfirst:\n%s\nsecond:\n%s", first, second)
	}
	for _, evidence := range []string{
		"alias relationship is unknown",
		"trace the alias consumer",
		"decoder semantics are unknown",
		"compare independent decoder traces",
	} {
		if !bytes.Contains(first, []byte(evidence)) {
			t.Fatalf("canonical report dropped unresolved evidence %q:\n%s", evidence, first)
		}
	}
}

func TestEncyclopediaInventoryRejectsTruncatedAndOversizedPERanges(t *testing.T) {
	base := buildTestPE32WithResource(t, rtEncyclopediaText, 42, 1033, []byte("payload"))

	tests := []struct {
		name   string
		mutate func([]byte) []byte
		limits encyclopediaInventoryLimits
	}{
		{
			name: "truncated resource section",
			mutate: func(source []byte) []byte {
				return source[:0x270]
			},
			limits: defaultEncyclopediaInventoryLimits(),
		},
		{
			name: "range beyond PE section",
			mutate: func(source []byte) []byte {
				copy := append([]byte(nil), source...)
				binary.LittleEndian.PutUint32(copy[0x260:0x264], 0x1000+0x1f0)
				binary.LittleEndian.PutUint32(copy[0x264:0x268], 0x40)
				return copy
			},
			limits: defaultEncyclopediaInventoryLimits(),
		},
		{
			name: "resource above configured bound",
			mutate: func(source []byte) []byte {
				copy := append([]byte(nil), source...)
				binary.LittleEndian.PutUint32(copy[0x264:0x268], 8)
				return copy
			},
			limits: encyclopediaInventoryLimits{
				MaxSourceBytes:            1 << 20,
				MaxResourceCount:          4,
				MaxResourceBytes:          7,
				MaxAggregateResourceBytes: 64,
			},
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			root := t.TempDir()
			writeTestFile(t, filepath.Join(root, "ENCYTEXT.DLL"), test.mutate(append([]byte(nil), base...)))
			_, err := inventoryEncyclopediaSources(encyclopediaInventoryRequest{
				Roots:     []encyclopediaSourceRoot{{Role: "install", Path: root}},
				Sources:   []encyclopediaSourceSpec{{RootRole: "install", Basename: "ENCYTEXT.DLL", Kind: encyclopediaSourceDLL, ResourceTypeID: rtEncyclopediaText}},
				StartedAt: time.Now(),
			}, test.limits)
			if err == nil {
				t.Fatal("malformed or oversized PE input was accepted")
			}
		})
	}
}

func TestEncyclopediaReportRejectsIncompleteUnresolvedStatus(t *testing.T) {
	numericID := uint32(1)
	report := encyclopediaResearchReport{
		Kind:          encyclopediaResearchKind,
		SchemaVersion: encyclopediaResearchSchemaVersion,
		Sources: []encyclopediaSourceRecord{{
			RootRole:  "install",
			Basename:  "ENCYTEXT.DLL",
			Kind:      encyclopediaSourceDLL,
			RawSHA256: byteSHA256(nil),
		}},
		Records: []encyclopediaResearchRecord{{
			SourceRootRole: "install",
			SourceBasename: "ENCYTEXT.DLL",
			ResourceType:   encyclopediaResourceIdentifier{Kind: encyclopediaIdentifierNumeric, NumericID: uint32Pointer(rtEncyclopediaText)},
			ResourceID:     encyclopediaResourceIdentifier{Kind: encyclopediaIdentifierNumeric, NumericID: &numericID},
			LanguageID:     1033,
			RawSHA256:      byteSHA256(nil),
			Status:         encyclopediaRecordUnresolved,
		}},
	}
	if _, err := marshalEncyclopediaResearchReport(report); err == nil {
		t.Fatal("unresolved record without a reason and next proof was accepted")
	}

	report.Records[0].Unresolved = &encyclopediaUnresolvedStatus{
		Reason:    "semantic meaning is not established",
		NextProof: "trace the original caller",
	}
	if _, err := marshalEncyclopediaResearchReport(report); err != nil {
		t.Fatalf("complete unresolved record rejected: %v", err)
	}
}

func TestOwnedEncyclopediaInventoryPreservesInputsAndRetainsIgnoredEvidence(t *testing.T) {
	sourceRoot := os.Getenv("REBELLION_ENCYCLOPEDIA_TEST_SOURCE")
	if sourceRoot == "" {
		t.Skip("set REBELLION_ENCYCLOPEDIA_TEST_SOURCE to an owned installation root")
	}
	absoluteRoot, err := filepath.Abs(sourceRoot)
	if err != nil {
		t.Fatal(err)
	}

	installNames := make([]string, 0, 4)
	for _, wanted := range []string{"ENCYTEXT.DLL", "ENCYBMAP.DLL", "TEXTSTRA.DLL", "REBEXE.EXE"} {
		if actual, ok := findCaseInsensitiveFile(t, absoluteRoot, wanted); ok {
			installNames = append(installNames, actual)
		}
	}
	if !containsFold(installNames, "ENCYTEXT.DLL") {
		t.Fatalf("%s does not contain ENCYTEXT.DLL", absoluteRoot)
	}

	datRoot := filepath.Join(absoluteRoot, "GData")
	if info, statErr := os.Stat(datRoot); statErr != nil || !info.IsDir() {
		datRoot = absoluteRoot
	}
	datEntries, err := os.ReadDir(datRoot)
	if err != nil {
		t.Fatal(err)
	}
	datNames := make([]string, 0)
	for _, entry := range datEntries {
		if !entry.IsDir() && strings.EqualFold(filepath.Ext(entry.Name()), ".DAT") {
			datNames = append(datNames, entry.Name())
		}
	}
	sort.Slice(datNames, func(i, j int) bool { return strings.ToLower(datNames[i]) < strings.ToLower(datNames[j]) })

	request := encyclopediaInventoryRequest{
		Roots: []encyclopediaSourceRoot{
			{Role: "install", Path: absoluteRoot},
			{Role: "gdata", Path: datRoot},
		},
		StartedAt: time.Now(),
	}
	for _, basename := range installNames {
		kind := encyclopediaSourceDLL
		resourceTypeID := uint32(0)
		if strings.EqualFold(filepath.Ext(basename), ".EXE") {
			kind = encyclopediaSourceEXE
		}
		if strings.EqualFold(basename, "ENCYTEXT.DLL") {
			resourceTypeID = rtEncyclopediaText
		}
		request.Sources = append(request.Sources, encyclopediaSourceSpec{
			RootRole: "install", Basename: basename, Kind: kind, ResourceTypeID: resourceTypeID,
		})
	}
	for _, basename := range datNames {
		request.Sources = append(request.Sources, encyclopediaSourceSpec{
			RootRole: "gdata", Basename: basename, Kind: encyclopediaSourceDAT,
		})
	}

	before := snapshotOwnedInputs(t, request)
	inventory, err := inventoryEncyclopediaSources(request, defaultEncyclopediaInventoryLimits())
	if err != nil {
		t.Fatal(err)
	}
	after := snapshotOwnedInputs(t, request)
	if !reflect.DeepEqual(before, after) {
		t.Fatal("owned input length/hash snapshot changed during inventory")
	}

	encytextHash := ""
	for _, source := range inventory.Report.Sources {
		if strings.EqualFold(source.Basename, "ENCYTEXT.DLL") {
			encytextHash = source.RawSHA256
		}
	}
	const observedEnglishENCYTEXTHash = "49aea545a5e09e5fe9115a22bc785690f103d2f931e08bd4a53a617a42636d8c"
	if encytextHash == observedEnglishENCYTEXTHash {
		if got, want := len(inventory.Report.Records), 348; got != want {
			t.Fatalf("identified English profile ENCYTEXT record count = %d, want observed %d", got, want)
		}
		for _, record := range inventory.Report.Records {
			if record.LanguageID != 1033 || record.CodePage != 0 {
				t.Fatalf("identified English profile record has LANGID/code page %d/%d", record.LanguageID, record.CodePage)
			}
		}
	}

	reportBytes, err := marshalEncyclopediaResearchReport(inventory.Report)
	if err != nil {
		t.Fatal(err)
	}
	runLogBytes, err := json.MarshalIndent(inventory.RunLog, "", "  ")
	if err != nil {
		t.Fatal(err)
	}
	runLogBytes = append(runLogBytes, '\n')
	repoRoot, err := filepath.Abs(filepath.Join("..", ".."))
	if err != nil {
		t.Fatal(err)
	}
	evidenceDir := filepath.Join(repoRoot, ".artifacts", "encyclopedia")
	if err := os.MkdirAll(evidenceDir, 0o700); err != nil {
		t.Fatal(err)
	}
	writeTestFile(t, filepath.Join(evidenceDir, "E01-owned-inventory.json"), reportBytes)
	writeTestFile(t, filepath.Join(evidenceDir, "E01-owned-inventory.run.json"), runLogBytes)
	t.Logf("retained ignored owned-source evidence under %s", evidenceDir)
}

func buildTestPE32WithEncyclopediaResources(t *testing.T) []byte {
	t.Helper()
	file := buildTestPE32WithResource(t, rtEncyclopediaText, 1, 1033, nil)
	const (
		sectionOffset = 0x200
		sectionRVA    = 0x1000
	)
	resource := file[sectionOffset : sectionOffset+0x200]
	clear(resource)
	putResourceDirectory(resource, 0x00, 0, 1)
	putResourceEntry(resource, 0x10, rtEncyclopediaText, resourceSubdirectory|0x20)
	putResourceDirectory(resource, 0x20, 1, 3)

	encodedName := utf16.Encode([]rune("7"))
	binary.LittleEndian.PutUint16(resource[0x120:0x122], uint16(len(encodedName)))
	for index, value := range encodedName {
		binary.LittleEndian.PutUint16(resource[0x122+index*2:0x124+index*2], value)
	}

	type fixtureRecord struct {
		name              uint32
		languageDirectory uint32
		language          uint32
		dataEntry         uint32
		payloadOffset     uint32
		codePage          uint32
		payload           []byte
	}
	records := []fixtureRecord{
		{name: resourceSubdirectory | 0x120, languageDirectory: 0x60, language: 1033, dataEntry: 0xe0, payloadOffset: 0x140, codePage: 1252, payload: []byte("named")},
		{name: 7, languageDirectory: 0x78, language: 1033, dataEntry: 0xf0, payloadOffset: 0x150, payload: []byte("first")},
		{name: 7, languageDirectory: 0x90, language: 1033, dataEntry: 0x100, payloadOffset: 0x160, payload: []byte("second")},
		{name: 7, languageDirectory: 0xa8, language: 1031, dataEntry: 0x110, payloadOffset: 0x170, codePage: 850, payload: []byte("de")},
	}
	for index, record := range records {
		putResourceEntry(resource, 0x30+index*8, record.name, resourceSubdirectory|record.languageDirectory)
		putResourceDirectory(resource, int(record.languageDirectory), 0, 1)
		putResourceEntry(resource, int(record.languageDirectory)+0x10, record.language, record.dataEntry)
		binary.LittleEndian.PutUint32(resource[record.dataEntry:record.dataEntry+4], sectionRVA+record.payloadOffset)
		binary.LittleEndian.PutUint32(resource[record.dataEntry+4:record.dataEntry+8], uint32(len(record.payload)))
		binary.LittleEndian.PutUint32(resource[record.dataEntry+8:record.dataEntry+12], record.codePage)
		copy(resource[record.payloadOffset:], record.payload)
	}
	return file
}

func writeTestFile(t *testing.T, path string, data []byte) {
	t.Helper()
	if err := os.WriteFile(path, data, 0o600); err != nil {
		t.Fatal(err)
	}
}

func uint32Pointer(value uint32) *uint32 {
	return &value
}

func findCaseInsensitiveFile(t *testing.T, root, wanted string) (string, bool) {
	t.Helper()
	entries, err := os.ReadDir(root)
	if err != nil {
		t.Fatal(err)
	}
	match := ""
	for _, entry := range entries {
		if entry.IsDir() || !strings.EqualFold(entry.Name(), wanted) {
			continue
		}
		if match != "" {
			t.Fatalf("ambiguous case-insensitive source %q under %s", wanted, root)
		}
		match = entry.Name()
	}
	return match, match != ""
}

func containsFold(values []string, wanted string) bool {
	for _, value := range values {
		if strings.EqualFold(value, wanted) {
			return true
		}
	}
	return false
}

func snapshotOwnedInputs(t *testing.T, request encyclopediaInventoryRequest) map[string]string {
	t.Helper()
	roots := make(map[string]string, len(request.Roots))
	for _, root := range request.Roots {
		roots[root.Role] = root.Path
	}
	snapshot := make(map[string]string, len(request.Sources))
	for _, source := range request.Sources {
		data, err := os.ReadFile(filepath.Join(roots[source.RootRole], source.Basename))
		if err != nil {
			t.Fatal(err)
		}
		snapshot[source.RootRole+"/"+source.Basename] = byteSHA256(data) + ":" + strconv.Itoa(len(data))
	}
	return snapshot
}
