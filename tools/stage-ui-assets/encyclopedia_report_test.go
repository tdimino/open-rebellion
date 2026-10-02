package main

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"io"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func writeSyntheticEncyclopediaTextDLL(t *testing.T, sourceDir string, resourceID encyclopediaResourceIdentifier, raw []byte) {
	t.Helper()
	var image []byte
	switch resourceID.Kind {
	case encyclopediaIdentifierNumeric:
		image = buildTestPE32WithResource(t, rtEncyclopediaText, *resourceID.NumericID, 1033, raw)
	case encyclopediaIdentifierNamed:
		image = buildTestPE32WithNamedResource(t, rtEncyclopediaText, resourceID.Name, 1033, raw)
	default:
		t.Fatalf("unsupported synthetic resource identifier: %+v", resourceID)
	}
	if err := os.WriteFile(filepath.Join(sourceDir, "ENCYTEXT.DLL"), image, 0o600); err != nil {
		t.Fatal(err)
	}
	edataRoot := filepath.Join(sourceDir, "EData")
	if err := os.MkdirAll(edataRoot, 0o755); err != nil {
		t.Fatal(err)
	}
	bmp := buildTestEncyclopediaBMP(t, testEncyclopediaBMPOptions{width: 1, height: 1, bitCount: 24})
	writeTestFile(t, filepath.Join(edataRoot, "EDATA.001"), bmp)
	lookupDLL := buildTestPE32WithResource(t, rtStringResource, 1, 1033, stringBundle(map[int]string{0: "EDATA.001"}))
	if err := os.WriteFile(filepath.Join(sourceDir, "ENCYBMAP.DLL"), lookupDLL, 0o600); err != nil {
		t.Fatal(err)
	}
}

func readTestEncyclopediaResearchReport(t *testing.T, output string) encyclopediaResearchReport {
	t.Helper()
	encoded, err := os.ReadFile(filepath.Join(output, encyclopediaResearchReportFilename))
	if err != nil {
		t.Fatal(err)
	}
	var report encyclopediaResearchReport
	if err := json.Unmarshal(encoded, &report); err != nil {
		t.Fatal(err)
	}
	return report
}

func testPublishedEncyclopediaDecodeReport(raw []byte) encyclopediaResearchReport {
	report := testEncyclopediaDecodeReport(raw)
	report.Sources[0].RootRole = encyclopediaReportSourceRole
	report.Records[0].SourceRootRole = encyclopediaReportSourceRole
	return report
}

func TestEncyclopediaUnknownProfileInventoriesRawWithoutGuessedText(t *testing.T) {
	source := t.TempDir()
	output := filepath.Join(t.TempDir(), "encyclopedia-research")
	writeSyntheticEncyclopediaTextDLL(t, source, numericEncyclopediaResourceIdentifier(42), []byte("synthetic unknown profile\x00"))

	if err := stageEncyclopediaReport(source, "", output, false, io.Discard); err != nil {
		t.Fatal(err)
	}
	report := readTestEncyclopediaResearchReport(t, output)
	if report.Kind != encyclopediaResearchKind || report.SchemaVersion != encyclopediaResearchSchemaVersion {
		t.Fatalf("report discriminator = %q/%d", report.Kind, report.SchemaVersion)
	}
	if len(report.Records) != 1 || report.Records[0].Status != encyclopediaRecordUnresolved || report.Records[0].Unresolved == nil {
		t.Fatalf("unknown-profile record = %+v", report.Records)
	}
	rawPath, err := encyclopediaReportRecordPath(report.Records[0], ".bin")
	if err != nil {
		t.Fatal(err)
	}
	if got := readTestFile(t, filepath.Join(output, filepath.FromSlash(rawPath))); !bytes.Equal(got, []byte("synthetic unknown profile\x00")) {
		t.Fatalf("preserved raw bytes = %q", got)
	}
	textPath, err := encyclopediaReportRecordPath(report.Records[0], ".txt")
	if err != nil {
		t.Fatal(err)
	}
	if _, err := os.Lstat(filepath.Join(output, filepath.FromSlash(textPath))); !os.IsNotExist(err) {
		t.Fatalf("unknown profile produced guessed text: %v", err)
	}
	if err := verifyEncyclopediaReport(output, io.Discard); err != nil {
		t.Fatal(err)
	}
}

func TestEncyclopediaEmptyUnknownResourceIsPreservedAndSourceFreeVerifiable(t *testing.T) {
	source := t.TempDir()
	output := filepath.Join(t.TempDir(), "encyclopedia-research")
	writeSyntheticEncyclopediaTextDLL(t, source, numericEncyclopediaResourceIdentifier(43), []byte{})

	if err := stageEncyclopediaReport(source, "", output, false, io.Discard); err != nil {
		t.Fatalf("stage valid empty resource: %v", err)
	}
	report := readTestEncyclopediaResearchReport(t, output)
	if len(report.Records) != 1 || report.Records[0].RawLength != 0 || report.Records[0].RawSHA256 != byteSHA256(nil) {
		t.Fatalf("empty resource facts = %+v", report.Records)
	}
	if report.Records[0].Status != encyclopediaRecordUnresolved {
		t.Fatalf("empty unknown-profile status = %q", report.Records[0].Status)
	}
	rawPath, err := encyclopediaReportRecordPath(report.Records[0], ".bin")
	if err != nil {
		t.Fatal(err)
	}
	info, err := os.Stat(filepath.Join(output, filepath.FromSlash(rawPath)))
	if err != nil {
		t.Fatal(err)
	}
	if info.Size() != 0 {
		t.Fatalf("preserved empty resource size = %d", info.Size())
	}
	if err := verifyEncyclopediaReport(output, io.Discard); err != nil {
		t.Fatalf("source-free verify empty resource: %v", err)
	}
}

func TestEncyclopediaCandidateRejectsNilBytesForNonemptyRecord(t *testing.T) {
	report := testPublishedEncyclopediaDecodeReport([]byte("nonempty\x00"))
	report.Records[0].RawBytes = nil
	if err := writeEncyclopediaReportCandidate(t.TempDir(), preparedEncyclopediaReport{report: report}); err == nil {
		t.Fatal("candidate accepted nil bytes for a nonempty record")
	}
}

func TestEncyclopediaEmptyKnownProfileStillRequiresDecoderEvidence(t *testing.T) {
	if _, err := prepareEncyclopediaReport(testPublishedEncyclopediaDecodeReport(nil)); err == nil {
		t.Fatal("known profile decoded an empty record without its proven terminal NUL")
	}
}

func TestEncyclopediaProvenProfileWritesExactDecodedText(t *testing.T) {
	raw := []byte{'P', 'i', 'l', 'o', 't', 0x92, 's', '\t', 'n', 'o', 't', 'e', '\n', 0}
	prepared, err := prepareEncyclopediaReport(testPublishedEncyclopediaDecodeReport(raw))
	if err != nil {
		t.Fatal(err)
	}
	root := t.TempDir()
	if err := writeEncyclopediaReportCandidate(root, prepared); err != nil {
		t.Fatal(err)
	}
	if _, err := inspectEncyclopediaReportDirectory(root); err != nil {
		t.Fatal(err)
	}
	path, err := encyclopediaReportRecordPath(prepared.report.Records[0], ".txt")
	if err != nil {
		t.Fatal(err)
	}
	if got, want := string(readTestFile(t, filepath.Join(root, filepath.FromSlash(path)))), "Pilot’s\tnote\n"; got != want {
		t.Fatalf("decoded text = %q, want %q", got, want)
	}
}

func TestEncyclopediaNamedResourcePathsAreReversibleAndSeparate(t *testing.T) {
	name := "../42\\topic"
	named := encyclopediaResearchRecord{ResourceID: namedEncyclopediaResourceIdentifier(name), LanguageID: 1033}
	numeric := encyclopediaResearchRecord{ResourceID: numericEncyclopediaResourceIdentifier(42), LanguageID: 1033}
	namedPath, err := encyclopediaReportRecordPath(named, ".bin")
	if err != nil {
		t.Fatal(err)
	}
	numericPath, err := encyclopediaReportRecordPath(numeric, ".bin")
	if err != nil {
		t.Fatal(err)
	}
	if namedPath == numericPath || !strings.HasPrefix(namedPath, "raw/encytext/1033/named/") || strings.Contains(namedPath, name) {
		t.Fatalf("named path = %q, numeric path = %q", namedPath, numericPath)
	}
	encodedName := strings.TrimSuffix(filepath.Base(namedPath), ".bin")
	decoded, err := hex.DecodeString(encodedName)
	if err != nil || string(decoded) != name {
		t.Fatalf("encoded name %q decoded to %q, %v", encodedName, decoded, err)
	}
}

func TestEncyclopediaNamedResourceCannotEscapeReportRoot(t *testing.T) {
	source := t.TempDir()
	parent := t.TempDir()
	output := filepath.Join(parent, "research")
	name := "../../outside\\42"
	writeSyntheticEncyclopediaTextDLL(t, source, namedEncyclopediaResourceIdentifier(name), []byte("named raw\x00"))
	if err := stageEncyclopediaReport(source, "", output, false, io.Discard); err != nil {
		t.Fatal(err)
	}
	report := readTestEncyclopediaResearchReport(t, output)
	path, err := encyclopediaReportRecordPath(report.Records[0], ".bin")
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(readTestFile(t, filepath.Join(output, filepath.FromSlash(path))), []byte("named raw\x00")) {
		t.Fatal("named resource bytes were not preserved")
	}
	if _, err := os.Lstat(filepath.Join(parent, "outside")); !os.IsNotExist(err) {
		t.Fatalf("named resource escaped report root: %v", err)
	}
}

func TestEncyclopediaByteIdenticalForcedReportRerunIsNoOp(t *testing.T) {
	source := t.TempDir()
	output := filepath.Join(t.TempDir(), "research")
	writeSyntheticEncyclopediaTextDLL(t, source, numericEncyclopediaResourceIdentifier(19), []byte("same bytes\x00"))
	if err := stageEncyclopediaReport(source, "", output, false, io.Discard); err != nil {
		t.Fatal(err)
	}
	before := snapshotTestTree(t, output)
	if err := stageEncyclopediaReport(source, "", output, true, io.Discard); err != nil {
		t.Fatal(err)
	}
	if diff := diffTestTree(before, snapshotTestTree(t, output)); diff != "" {
		t.Fatalf("byte-identical forced rerun changed report:\n%s", diff)
	}
}

func TestEncyclopediaVerifyIsReadOnlyAndNeedsNoInstall(t *testing.T) {
	parent := t.TempDir()
	source := filepath.Join(parent, "install")
	output := filepath.Join(parent, "encyclopedia-research")
	if err := os.Mkdir(source, 0o755); err != nil {
		t.Fatal(err)
	}
	writeSyntheticEncyclopediaTextDLL(t, source, numericEncyclopediaResourceIdentifier(7), []byte("source-free verify\x00"))
	if err := stageEncyclopediaReport(source, "", output, false, io.Discard); err != nil {
		t.Fatal(err)
	}
	if err := os.RemoveAll(source); err != nil {
		t.Fatal(err)
	}
	before := snapshotTestTree(t, parent)
	if err := verifyEncyclopediaReport(output, io.Discard); err != nil {
		t.Fatal(err)
	}
	if diff := diffTestTree(before, snapshotTestTree(t, parent)); diff != "" {
		t.Fatalf("source-free verification wrote to disk:\n%s", diff)
	}
}

func TestEncyclopediaVerifyRejectsTamperingWithoutRepair(t *testing.T) {
	source := t.TempDir()
	parent := t.TempDir()
	output := filepath.Join(parent, "research")
	writeSyntheticEncyclopediaTextDLL(t, source, numericEncyclopediaResourceIdentifier(31), []byte("untampered\x00"))
	if err := stageEncyclopediaReport(source, "", output, false, io.Discard); err != nil {
		t.Fatal(err)
	}
	report := readTestEncyclopediaResearchReport(t, output)
	rawPath, err := encyclopediaReportRecordPath(report.Records[0], ".bin")
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(output, filepath.FromSlash(rawPath)), []byte("tampered"), 0o644); err != nil {
		t.Fatal(err)
	}
	before := snapshotTestTree(t, parent)
	if err := verifyEncyclopediaReport(output, io.Discard); err == nil {
		t.Fatal("verification accepted tampered raw bytes")
	}
	if diff := diffTestTree(before, snapshotTestTree(t, parent)); diff != "" {
		t.Fatalf("failed verification repaired output:\n%s", diff)
	}
}

func TestEncyclopediaVerifyRejectsNoncanonicalReportMetadataWithoutRepair(t *testing.T) {
	source := t.TempDir()
	parent := t.TempDir()
	output := filepath.Join(parent, "research")
	writeSyntheticEncyclopediaTextDLL(t, source, numericEncyclopediaResourceIdentifier(33), []byte("canonical report\x00"))
	if err := stageEncyclopediaReport(source, "", output, false, io.Discard); err != nil {
		t.Fatal(err)
	}
	reportPath := filepath.Join(output, encyclopediaResearchReportFilename)
	encoded := readTestFile(t, reportPath)
	tampered := bytes.Replace(encoded, []byte(`"source_count": 1`), []byte(`"source_count": 99`), 1)
	if bytes.Equal(tampered, encoded) {
		t.Fatal("test did not locate source_count")
	}
	if err := os.WriteFile(reportPath, tampered, 0o644); err != nil {
		t.Fatal(err)
	}
	before := snapshotTestTree(t, parent)
	if err := verifyEncyclopediaReport(output, io.Discard); err == nil {
		t.Fatal("verification accepted noncanonical report metadata")
	}
	if diff := diffTestTree(before, snapshotTestTree(t, parent)); diff != "" {
		t.Fatalf("verification rewrote noncanonical report:\n%s", diff)
	}
}

func TestEncyclopediaVerifyRejectsDowngradedKnownProfile(t *testing.T) {
	prepared, err := prepareEncyclopediaReport(testPublishedEncyclopediaDecodeReport([]byte("proven text\x00")))
	if err != nil {
		t.Fatal(err)
	}
	root := t.TempDir()
	if err := writeEncyclopediaReportCandidate(root, prepared); err != nil {
		t.Fatal(err)
	}
	report := prepared.report
	report.Records[0].Status = encyclopediaRecordUnresolved
	report.Records[0].Unresolved = &encyclopediaUnresolvedStatus{
		Reason:    "pretend this profile is unknown",
		NextProof: "ignore the embedded profile",
	}
	encoded, err := marshalEncyclopediaResearchReport(report)
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(root, encyclopediaResearchReportFilename), encoded, 0o644); err != nil {
		t.Fatal(err)
	}
	textPath, err := encyclopediaReportRecordPath(report.Records[0], ".txt")
	if err != nil {
		t.Fatal(err)
	}
	if err := os.Remove(filepath.Join(root, filepath.FromSlash(textPath))); err != nil {
		t.Fatal(err)
	}
	if err := verifyEncyclopediaReport(root, io.Discard); err == nil {
		t.Fatal("verification accepted a known profile downgraded to unresolved")
	}
}

func TestEncyclopediaVerifyRejectsRecordsOutsideTextReportContract(t *testing.T) {
	for _, test := range []struct {
		name   string
		mutate func(*encyclopediaResearchReport)
	}{
		{
			name: "wrong resource type",
			mutate: func(report *encyclopediaResearchReport) {
				report.Records[0].ResourceType = numericEncyclopediaResourceIdentifier(6)
			},
		},
		{
			name: "wrong source contract",
			mutate: func(report *encyclopediaResearchReport) {
				report.Sources[0].RootRole = "other"
				report.Records[0].SourceRootRole = "other"
			},
		},
	} {
		t.Run(test.name, func(t *testing.T) {
			source := t.TempDir()
			output := filepath.Join(t.TempDir(), "research")
			writeSyntheticEncyclopediaTextDLL(t, source, numericEncyclopediaResourceIdentifier(35), []byte("contract bytes\x00"))
			if err := stageEncyclopediaReport(source, "", output, false, io.Discard); err != nil {
				t.Fatal(err)
			}
			report := readTestEncyclopediaResearchReport(t, output)
			test.mutate(&report)
			encoded, err := marshalEncyclopediaResearchReport(report)
			if err != nil {
				t.Fatal(err)
			}
			if err := os.WriteFile(filepath.Join(output, encyclopediaResearchReportFilename), encoded, 0o644); err != nil {
				t.Fatal(err)
			}
			if err := verifyEncyclopediaReport(output, io.Discard); err == nil {
				t.Fatal("verification accepted a report outside the text-report contract")
			}
		})
	}
}

func TestEncyclopediaMalformedInputPreservesPreviousReport(t *testing.T) {
	source := t.TempDir()
	parent := t.TempDir()
	output := filepath.Join(parent, "encyclopedia-research")
	writeSyntheticEncyclopediaTextDLL(t, source, numericEncyclopediaResourceIdentifier(9), []byte("valid first report\x00"))
	if err := stageEncyclopediaReport(source, "", output, false, io.Discard); err != nil {
		t.Fatal(err)
	}
	before := snapshotTestTree(t, parent)
	if err := os.WriteFile(filepath.Join(source, "ENCYTEXT.DLL"), []byte("MZ truncated"), 0o600); err != nil {
		t.Fatal(err)
	}
	if err := stageEncyclopediaReport(source, "", output, true, io.Discard); err == nil {
		t.Fatal("malformed source unexpectedly replaced the report")
	}
	if diff := diffTestTree(before, snapshotTestTree(t, parent)); diff != "" {
		t.Fatalf("failed extraction changed prior output:\n%s", diff)
	}
}

func TestEncyclopediaReportCannotReplaceRuntimeCatalog(t *testing.T) {
	source := t.TempDir()
	parent := t.TempDir()
	output := filepath.Join(parent, "encyclopedia")
	writeSyntheticEncyclopediaTextDLL(t, source, numericEncyclopediaResourceIdentifier(11), []byte("report bytes\x00"))
	if err := os.Mkdir(output, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(output, "catalog.json"), []byte("canonical catalog bytes"), 0o600); err != nil {
		t.Fatal(err)
	}
	before := snapshotTestTree(t, parent)
	if err := stageEncyclopediaReport(source, "", output, true, io.Discard); err == nil {
		t.Fatal("report mode replaced a runtime catalog")
	}
	if diff := diffTestTree(before, snapshotTestTree(t, parent)); diff != "" {
		t.Fatalf("rejected cross-mode output changed:\n%s", diff)
	}
}

func TestEncyclopediaReportRejectsSourceOutputCollisionBeforeWriting(t *testing.T) {
	source := t.TempDir()
	writeSyntheticEncyclopediaTextDLL(t, source, numericEncyclopediaResourceIdentifier(37), []byte("source remains\x00"))
	before := snapshotTestTree(t, source)
	if err := stageEncyclopediaReport(source, "", source, true, io.Discard); err == nil || !strings.Contains(err.Error(), "path_collision") {
		t.Fatalf("source/output collision error = %v", err)
	}
	if diff := diffTestTree(before, snapshotTestTree(t, source)); diff != "" {
		t.Fatalf("collision refusal changed source:\n%s", diff)
	}
}

func TestOwnedEncyclopediaReportIsDeterministicAndSourceFreeVerifiable(t *testing.T) {
	source := os.Getenv("REBELLION_ENCYCLOPEDIA_TEST_SOURCE")
	if source == "" {
		t.Skip("set REBELLION_ENCYCLOPEDIA_TEST_SOURCE to an owned installation root")
	}
	first := filepath.Join(t.TempDir(), "first")
	second := filepath.Join(t.TempDir(), "second")
	if err := stageEncyclopediaReport(source, "", first, false, io.Discard); err != nil {
		t.Fatal(err)
	}
	if err := stageEncyclopediaReport(source, "", second, false, io.Discard); err != nil {
		t.Fatal(err)
	}
	if diff := diffTestTree(snapshotTestTree(t, first), snapshotTestTree(t, second)); diff != "" {
		t.Fatalf("owned report extractions differ:\n%s", diff)
	}
	if err := verifyEncyclopediaReport(first, io.Discard); err != nil {
		t.Fatal(err)
	}
	report := readTestEncyclopediaResearchReport(t, first)
	if len(report.Records) != 348 {
		t.Fatalf("owned report record count = %d, want 348", len(report.Records))
	}
	for _, record := range report.Records {
		if record.Status != encyclopediaRecordDecoded {
			t.Fatalf("owned record %s status = %q", formatEncyclopediaResourceIdentifier(record.ResourceID), record.Status)
		}
	}
}
