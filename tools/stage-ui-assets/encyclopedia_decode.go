package main

import (
	"bytes"
	"embed"
	"encoding/json"
	"fmt"
	"io"
	"io/fs"
	"sort"
	"strconv"
	"strings"
)

const (
	encyclopediaSourceProfileKind          = "encyclopedia-source-profile"
	encyclopediaSourceProfileSchemaVersion = 1
)

//go:embed encyclopedia_profiles/*.json
var embeddedEncyclopediaProfiles embed.FS

type encyclopediaSourceProfile struct {
	Kind          string                             `json:"kind"`
	SchemaVersion int                                `json:"schema_version"`
	ProfileID     string                             `json:"profile_id"`
	Sources       []encyclopediaProfileSource        `json:"sources"`
	TextDecoder   encyclopediaTextDecoderProfile     `json:"text_decoder"`
	Evidence      encyclopediaDecoderProfileEvidence `json:"evidence"`
}

type encyclopediaProfileSource struct {
	Role                string                 `json:"role"`
	Basename            string                 `json:"basename"`
	Kind                encyclopediaSourceKind `json:"kind"`
	RawLength           uint64                 `json:"raw_length"`
	RawSHA256           string                 `json:"raw_sha256"`
	RequiredForDecoding bool                   `json:"required_for_decoding"`
}

type encyclopediaTextDecoderProfile struct {
	SourceRole      string                             `json:"source_role"`
	ResourceTypeID  uint32                             `json:"resource_type_id"`
	LanguageIDs     []uint32                           `json:"language_ids"`
	PECodePages     []uint32                           `json:"pe_code_pages"`
	Encoding        string                             `json:"encoding"`
	LineEndings     string                             `json:"line_endings"`
	AllowedControls []string                           `json:"allowed_controls"`
	TerminalNUL     encyclopediaTerminalNULProfile     `json:"terminal_nul"`
	NonASCII        encyclopediaNonASCIIProfileSummary `json:"non_ascii"`
}

type encyclopediaTerminalNULProfile struct {
	MinimumCount int `json:"minimum_count"`
	MaximumCount int `json:"maximum_count"`
}

type encyclopediaNonASCIIProfileSummary struct {
	RecordCount          int                               `json:"record_count"`
	OccurrenceCount      int                               `json:"occurrence_count"`
	RecordSetSHA256      string                            `json:"record_set_sha256"`
	ObservedByteMappings []encyclopediaObservedByteMapping `json:"observed_byte_mappings"`
}

type encyclopediaObservedByteMapping struct {
	Byte    uint8  `json:"byte"`
	Unicode string `json:"unicode"`
}

type encyclopediaDecoderProfileEvidence struct {
	ResearchExecutableRole string                                `json:"research_executable_role"`
	CallPath               []encyclopediaDecoderCallPathEvidence `json:"call_path"`
	Corpus                 encyclopediaDecoderCorpusEvidence     `json:"corpus"`
}

type encyclopediaDecoderCallPathEvidence struct {
	Address     string `json:"address"`
	Symbol      string `json:"symbol"`
	Observation string `json:"observation"`
}

type encyclopediaDecoderCorpusEvidence struct {
	RecordCount               int            `json:"record_count"`
	LanguageID                uint32         `json:"language_id"`
	PECodePage                uint32         `json:"pe_code_page"`
	TrailingNULHistogram      map[string]int `json:"trailing_nul_histogram"`
	LFCount                   int            `json:"lf_count"`
	TabCount                  int            `json:"tab_count"`
	OtherC0RecordCount        int            `json:"other_c0_record_count"`
	NonzeroAfterFirstNULCount int            `json:"nonzero_after_first_nul_count"`
}

type encyclopediaDecodedCorpus struct {
	ProfileID string
	Records   []encyclopediaDecodedRecord
}

type encyclopediaDecodedRecord struct {
	encyclopediaResearchRecord
	ProfileID string
	Encoding  string
	Text      string
}

func decodeEncyclopediaReport(report encyclopediaResearchReport) (encyclopediaDecodedCorpus, error) {
	canonical, err := canonicalEncyclopediaResearchReport(report)
	if err != nil {
		return encyclopediaDecodedCorpus{}, fmt.Errorf("validate encyclopedia report for decoding: %w", err)
	}
	profiles, err := loadEmbeddedEncyclopediaProfiles()
	if err != nil {
		return encyclopediaDecodedCorpus{}, err
	}
	if len(canonical.Records) == 0 {
		return encyclopediaDecodedCorpus{}, fmt.Errorf("encyclopedia decoder requires at least one source record")
	}

	result := encyclopediaDecodedCorpus{Records: make([]encyclopediaDecodedRecord, 0, len(canonical.Records))}
	for _, record := range canonical.Records {
		if record.ResourceType.Kind != encyclopediaIdentifierNumeric || record.ResourceType.NumericID == nil || *record.ResourceType.NumericID != rtEncyclopediaText {
			return encyclopediaDecodedCorpus{}, fmt.Errorf("resource %s from %s is not ENCYTEXT type %d", formatEncyclopediaResourceIdentifier(record.ResourceID), record.SourceBasename, rtEncyclopediaText)
		}
		source, err := sourceForEncyclopediaRecord(canonical.Sources, record)
		if err != nil {
			return encyclopediaDecodedCorpus{}, err
		}
		profile, decoderSource, err := matchEncyclopediaDecodeProfile(profiles, source)
		if err != nil {
			return encyclopediaDecodedCorpus{}, fmt.Errorf("resource %s from %s: %w", formatEncyclopediaResourceIdentifier(record.ResourceID), record.SourceBasename, err)
		}
		if result.ProfileID == "" {
			result.ProfileID = profile.ProfileID
		} else if result.ProfileID != profile.ProfileID {
			return encyclopediaDecodedCorpus{}, fmt.Errorf("one encyclopedia decode cannot combine profiles %q and %q", result.ProfileID, profile.ProfileID)
		}
		decoded, err := decodeEncyclopediaRecord(*profile, *decoderSource, record)
		if err != nil {
			return encyclopediaDecodedCorpus{}, fmt.Errorf("decode resource %s from %s: %w", formatEncyclopediaResourceIdentifier(record.ResourceID), record.SourceBasename, err)
		}
		result.Records = append(result.Records, decoded)
	}
	return result, nil
}

func loadEmbeddedEncyclopediaProfiles() ([]encyclopediaSourceProfile, error) {
	names, err := fs.Glob(embeddedEncyclopediaProfiles, "encyclopedia_profiles/*.json")
	if err != nil {
		return nil, fmt.Errorf("list embedded encyclopedia profiles: %w", err)
	}
	if len(names) == 0 {
		return nil, fmt.Errorf("no embedded encyclopedia source profiles")
	}
	sort.Strings(names)
	profiles := make([]encyclopediaSourceProfile, 0, len(names))
	seenIDs := make(map[string]struct{}, len(names))
	for _, name := range names {
		data, err := embeddedEncyclopediaProfiles.ReadFile(name)
		if err != nil {
			return nil, fmt.Errorf("read embedded encyclopedia profile %s: %w", name, err)
		}
		decoder := json.NewDecoder(bytes.NewReader(data))
		decoder.DisallowUnknownFields()
		var profile encyclopediaSourceProfile
		if err := decoder.Decode(&profile); err != nil {
			return nil, fmt.Errorf("decode embedded encyclopedia profile %s: %w", name, err)
		}
		if err := requireJSONEOF(decoder); err != nil {
			return nil, fmt.Errorf("decode embedded encyclopedia profile %s: %w", name, err)
		}
		if err := validateEncyclopediaSourceProfile(profile); err != nil {
			return nil, fmt.Errorf("validate embedded encyclopedia profile %s: %w", name, err)
		}
		if _, exists := seenIDs[profile.ProfileID]; exists {
			return nil, fmt.Errorf("duplicate embedded encyclopedia profile ID %q", profile.ProfileID)
		}
		seenIDs[profile.ProfileID] = struct{}{}
		profiles = append(profiles, profile)
	}
	sort.Slice(profiles, func(i, j int) bool { return profiles[i].ProfileID < profiles[j].ProfileID })
	return profiles, nil
}

func requireJSONEOF(decoder *json.Decoder) error {
	var extra any
	if err := decoder.Decode(&extra); err != io.EOF {
		if err == nil {
			return fmt.Errorf("multiple JSON values are not allowed")
		}
		return err
	}
	return nil
}

func validateEncyclopediaSourceProfile(profile encyclopediaSourceProfile) error {
	if profile.Kind != encyclopediaSourceProfileKind || profile.SchemaVersion != encyclopediaSourceProfileSchemaVersion {
		return fmt.Errorf("unsupported kind/schema %q/%d", profile.Kind, profile.SchemaVersion)
	}
	if strings.TrimSpace(profile.ProfileID) == "" || strings.ContainsAny(profile.ProfileID, "/\\\x00\r\n") {
		return fmt.Errorf("invalid profile ID %q", profile.ProfileID)
	}
	if len(profile.Sources) == 0 {
		return fmt.Errorf("profile %q has no source identities", profile.ProfileID)
	}
	sourcesByRole := make(map[string]encyclopediaProfileSource, len(profile.Sources))
	for _, source := range profile.Sources {
		if strings.TrimSpace(source.Role) == "" || strings.ContainsAny(source.Role, "/\\\x00\r\n") {
			return fmt.Errorf("profile %q has invalid source role %q", profile.ProfileID, source.Role)
		}
		if _, exists := sourcesByRole[source.Role]; exists {
			return fmt.Errorf("profile %q repeats source role %q", profile.ProfileID, source.Role)
		}
		if source.Basename == "" || strings.ContainsAny(source.Basename, "/\\\x00\r\n") {
			return fmt.Errorf("profile %q has invalid source basename %q", profile.ProfileID, source.Basename)
		}
		switch source.Kind {
		case encyclopediaSourceDLL, encyclopediaSourceEXE, encyclopediaSourceDAT:
		default:
			return fmt.Errorf("profile %q source %q has invalid kind %q", profile.ProfileID, source.Role, source.Kind)
		}
		if source.RawLength == 0 || !validSHA256(source.RawSHA256) {
			return fmt.Errorf("profile %q source %q has invalid length/hash", profile.ProfileID, source.Role)
		}
		sourcesByRole[source.Role] = source
	}

	decoder := profile.TextDecoder
	decoderSource, ok := sourcesByRole[decoder.SourceRole]
	if !ok || !decoderSource.RequiredForDecoding || decoderSource.Kind != encyclopediaSourceDLL {
		return fmt.Errorf("profile %q decoder source role %q is not a required DLL", profile.ProfileID, decoder.SourceRole)
	}
	if decoder.ResourceTypeID != rtEncyclopediaText {
		return fmt.Errorf("profile %q decoder resource type = %d, want %d", profile.ProfileID, decoder.ResourceTypeID, rtEncyclopediaText)
	}
	if !strictlyIncreasingUint32(decoder.LanguageIDs) || !strictlyIncreasingUint32AllowZero(decoder.PECodePages) {
		return fmt.Errorf("profile %q language IDs and PE code pages must be nonempty, unique, and sorted", profile.ProfileID)
	}
	if decoder.Encoding != "windows-1252" || decoder.LineEndings != "lf" {
		return fmt.Errorf("profile %q has unsupported encoding/line endings %q/%q", profile.ProfileID, decoder.Encoding, decoder.LineEndings)
	}
	if !equalStrings(decoder.AllowedControls, []string{"lf", "tab"}) {
		return fmt.Errorf("profile %q has unsupported control policy", profile.ProfileID)
	}
	if decoder.TerminalNUL.MinimumCount < 1 || decoder.TerminalNUL.MaximumCount < decoder.TerminalNUL.MinimumCount {
		return fmt.Errorf("profile %q has invalid terminal NUL bounds", profile.ProfileID)
	}
	if decoder.NonASCII.RecordCount < 1 || decoder.NonASCII.OccurrenceCount < decoder.NonASCII.RecordCount || !validSHA256(decoder.NonASCII.RecordSetSHA256) || len(decoder.NonASCII.ObservedByteMappings) == 0 {
		return fmt.Errorf("profile %q has incomplete non-ASCII evidence", profile.ProfileID)
	}
	seenMappings := make(map[uint8]struct{}, len(decoder.NonASCII.ObservedByteMappings))
	for _, mapping := range decoder.NonASCII.ObservedByteMappings {
		if mapping.Byte < 0x80 || mapping.Unicode == "" {
			return fmt.Errorf("profile %q has invalid observed byte mapping", profile.ProfileID)
		}
		if _, exists := seenMappings[mapping.Byte]; exists {
			return fmt.Errorf("profile %q repeats observed byte 0x%02x", profile.ProfileID, mapping.Byte)
		}
		mappedRune, ok := decodeWindows1252Byte(mapping.Byte)
		if !ok || mapping.Unicode != fmt.Sprintf("U+%04X", mappedRune) {
			return fmt.Errorf("profile %q mapping for byte 0x%02x contradicts Windows-1252", profile.ProfileID, mapping.Byte)
		}
		seenMappings[mapping.Byte] = struct{}{}
	}

	researchExecutable, ok := sourcesByRole[profile.Evidence.ResearchExecutableRole]
	if !ok || researchExecutable.Kind != encyclopediaSourceEXE || researchExecutable.RequiredForDecoding {
		return fmt.Errorf("profile %q evidence executable role must identify an optional EXE", profile.ProfileID)
	}
	if len(profile.Evidence.CallPath) == 0 {
		return fmt.Errorf("profile %q has no executable call-path evidence", profile.ProfileID)
	}
	for _, citation := range profile.Evidence.CallPath {
		if !strings.HasPrefix(citation.Address, "0x") || citation.Symbol == "" || strings.TrimSpace(citation.Observation) == "" {
			return fmt.Errorf("profile %q has incomplete call-path evidence", profile.ProfileID)
		}
	}
	corpus := profile.Evidence.Corpus
	if corpus.RecordCount < decoder.NonASCII.RecordCount || !containsUint32(decoder.LanguageIDs, corpus.LanguageID) || !containsUint32(decoder.PECodePages, corpus.PECodePage) || len(corpus.TrailingNULHistogram) == 0 || corpus.LFCount < 0 || corpus.TabCount < 0 || corpus.OtherC0RecordCount != 0 || corpus.NonzeroAfterFirstNULCount != 0 {
		return fmt.Errorf("profile %q has inconsistent corpus evidence", profile.ProfileID)
	}
	histogramRecords := 0
	for terminalNULText, count := range corpus.TrailingNULHistogram {
		terminalNULs, err := strconv.Atoi(terminalNULText)
		if err != nil || terminalNULs < decoder.TerminalNUL.MinimumCount || terminalNULs > decoder.TerminalNUL.MaximumCount || count < 1 {
			return fmt.Errorf("profile %q has invalid terminal NUL corpus evidence", profile.ProfileID)
		}
		histogramRecords += count
	}
	if histogramRecords != corpus.RecordCount {
		return fmt.Errorf("profile %q terminal NUL histogram accounts for %d records, want %d", profile.ProfileID, histogramRecords, corpus.RecordCount)
	}
	return nil
}

func sourceForEncyclopediaRecord(sources []encyclopediaSourceRecord, record encyclopediaResearchRecord) (encyclopediaSourceRecord, error) {
	for _, source := range sources {
		if source.RootRole == record.SourceRootRole && source.Basename == record.SourceBasename {
			return source, nil
		}
	}
	return encyclopediaSourceRecord{}, fmt.Errorf("resource %s refers to missing source %s/%s", formatEncyclopediaResourceIdentifier(record.ResourceID), record.SourceRootRole, record.SourceBasename)
}

func matchEncyclopediaDecodeProfile(profiles []encyclopediaSourceProfile, source encyclopediaSourceRecord) (*encyclopediaSourceProfile, *encyclopediaProfileSource, error) {
	var matchedProfile *encyclopediaSourceProfile
	var matchedSource *encyclopediaProfileSource
	for profileIndex := range profiles {
		profile := &profiles[profileIndex]
		for sourceIndex := range profile.Sources {
			candidate := &profile.Sources[sourceIndex]
			if candidate.Role != profile.TextDecoder.SourceRole || !strings.EqualFold(candidate.Basename, source.Basename) || candidate.Kind != source.Kind || candidate.RawLength != source.RawLength || candidate.RawSHA256 != source.RawSHA256 {
				continue
			}
			if matchedProfile != nil {
				return nil, nil, fmt.Errorf("source identity ambiguously matches profiles %q and %q", matchedProfile.ProfileID, profile.ProfileID)
			}
			matchedProfile = profile
			matchedSource = candidate
		}
	}
	if matchedProfile == nil {
		return nil, nil, fmt.Errorf("unsupported source profile for %s length %d SHA-256 %s", source.Basename, source.RawLength, source.RawSHA256)
	}
	return matchedProfile, matchedSource, nil
}

func decodeEncyclopediaRecord(profile encyclopediaSourceProfile, source encyclopediaProfileSource, record encyclopediaResearchRecord) (encyclopediaDecodedRecord, error) {
	decoder := profile.TextDecoder
	if !strings.EqualFold(source.Basename, record.SourceBasename) || !containsUint32(decoder.LanguageIDs, record.LanguageID) || !containsUint32(decoder.PECodePages, record.CodePage) {
		return encyclopediaDecodedRecord{}, fmt.Errorf("record identity, LANGID %d, or PE code page %d is outside profile %q", record.LanguageID, record.CodePage, profile.ProfileID)
	}
	if record.Status == encyclopediaRecordUnresolved {
		return encyclopediaDecodedRecord{}, fmt.Errorf("record is explicitly unresolved: %s", record.Unresolved.Reason)
	}
	if record.RawBytes == nil || record.RawLength != uint64(len(record.RawBytes)) || record.RawSHA256 != byteSHA256(record.RawBytes) {
		return encyclopediaDecodedRecord{}, fmt.Errorf("raw bytes do not match the inventoried length/hash")
	}

	terminalNULCount := 0
	for index := len(record.RawBytes) - 1; index >= 0 && record.RawBytes[index] == 0; index-- {
		terminalNULCount++
	}
	if terminalNULCount < decoder.TerminalNUL.MinimumCount || terminalNULCount > decoder.TerminalNUL.MaximumCount {
		return encyclopediaDecodedRecord{}, fmt.Errorf("terminal NUL count %d is outside proven profile bounds %d..%d", terminalNULCount, decoder.TerminalNUL.MinimumCount, decoder.TerminalNUL.MaximumCount)
	}
	content := record.RawBytes[:len(record.RawBytes)-terminalNULCount]
	if bytes.IndexByte(content, 0) >= 0 {
		return encyclopediaDecodedRecord{}, fmt.Errorf("non-padding bytes follow the first NUL terminator")
	}
	text, err := decodeWindows1252EncyclopediaText(content, decoder.AllowedControls)
	if err != nil {
		return encyclopediaDecodedRecord{}, err
	}

	decodedRecord := record
	decodedRecord.RawBytes = append([]byte(nil), record.RawBytes...)
	if decodedRecord.Status == encyclopediaRecordInventoried {
		decodedRecord.Status = encyclopediaRecordDecoded
	}
	return encyclopediaDecodedRecord{
		encyclopediaResearchRecord: decodedRecord,
		ProfileID:                  profile.ProfileID,
		Encoding:                   decoder.Encoding,
		Text:                       text,
	}, nil
}

func decodeWindows1252EncyclopediaText(content []byte, allowedControls []string) (string, error) {
	allowLF := containsString(allowedControls, "lf")
	allowTab := containsString(allowedControls, "tab")
	var decoded strings.Builder
	decoded.Grow(len(content))
	for offset, value := range content {
		switch {
		case value == '\n' && allowLF:
			decoded.WriteByte(value)
		case value == '\t' && allowTab:
			decoded.WriteByte(value)
		case value < 0x20 || value == 0x7f:
			return "", fmt.Errorf("unsupported control byte 0x%02x at offset %d", value, offset)
		case value < 0x80:
			decoded.WriteByte(value)
		case value >= 0xa0:
			decoded.WriteRune(rune(value))
		default:
			mapped, ok := decodeWindows1252Byte(value)
			if !ok {
				return "", fmt.Errorf("undefined Windows-1252 byte 0x%02x at offset %d", value, offset)
			}
			decoded.WriteRune(mapped)
		}
	}
	return decoded.String(), nil
}

func decodeWindows1252Byte(value byte) (rune, bool) {
	switch {
	case value < 0x80:
		return rune(value), true
	case value >= 0xa0:
		return rune(value), true
	default:
		mapped := windows1252C1Runes[value-0x80]
		return mapped, mapped != 0
	}
}

var windows1252C1Runes = [32]rune{
	'€', 0, '‚', 'ƒ', '„', '…', '†', '‡', 'ˆ', '‰', 'Š', '‹', 'Œ', 0, 'Ž', 0,
	0, '‘', '’', '“', '”', '•', '–', '—', '˜', '™', 'š', '›', 'œ', 0, 'ž', 'Ÿ',
}

func formatEncyclopediaResourceIdentifier(identifier encyclopediaResourceIdentifier) string {
	if identifier.Kind == encyclopediaIdentifierNumeric && identifier.NumericID != nil {
		return fmt.Sprintf("numeric:%d", *identifier.NumericID)
	}
	if identifier.Kind == encyclopediaIdentifierNamed {
		return fmt.Sprintf("named:%q", identifier.Name)
	}
	return "invalid"
}

func strictlyIncreasingUint32(values []uint32) bool {
	if len(values) == 0 || values[0] == 0 {
		return false
	}
	for index := 1; index < len(values); index++ {
		if values[index] <= values[index-1] {
			return false
		}
	}
	return true
}

func strictlyIncreasingUint32AllowZero(values []uint32) bool {
	if len(values) == 0 {
		return false
	}
	for index := 1; index < len(values); index++ {
		if values[index] <= values[index-1] {
			return false
		}
	}
	return true
}

func containsUint32(values []uint32, wanted uint32) bool {
	for _, value := range values {
		if value == wanted {
			return true
		}
	}
	return false
}

func containsString(values []string, wanted string) bool {
	for _, value := range values {
		if value == wanted {
			return true
		}
	}
	return false
}

func equalStrings(left, right []string) bool {
	if len(left) != len(right) {
		return false
	}
	for index := range left {
		if left[index] != right[index] {
			return false
		}
	}
	return true
}
