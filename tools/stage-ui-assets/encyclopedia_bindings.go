package main

import (
	"fmt"
	"path/filepath"
	"strconv"
	"strings"
)

const (
	encyclopediaProfileBindingsKind          = "encyclopedia-profile-bindings"
	encyclopediaProfileBindingsSchemaVersion = 1
	encyclopediaBindingBound                 = "bound"
	encyclopediaBindingDocumentedAlias       = "documented_alias"
	encyclopediaBindingUnresolved            = "unresolved"
	encyclopediaBindingSourceProvenUnused    = "source_proven_unused"
	encyclopediaBindingPublicationDeferred   = "publication_deferred"
	encyclopediaBindingAggregateOnlyBlocker  = "aggregate-only-topic-membership"
)

type encyclopediaProfileBindings struct {
	Kind               string                                `json:"kind"`
	SchemaVersion      int                                   `json:"schema_version"`
	LanguageID         uint32                                `json:"language_id"`
	FragmentInputs     []encyclopediaBindingFragmentInput    `json:"fragment_inputs"`
	SourceCoverage     []encyclopediaBindingSourceCoverage   `json:"source_coverage"`
	Categories         []encyclopediaBindingCategory         `json:"categories"`
	Records            []encyclopediaBindingRecord           `json:"records"`
	ResourceAccounting encyclopediaBindingResourceAccounting `json:"resource_accounting"`
	SchemaFit          encyclopediaBindingSchemaFit          `json:"schema_fit"`
}

type encyclopediaBindingSourceCoverage struct {
	SourceRole   string                                 `json:"source_role"`
	HeaderBytes  uint32                                 `json:"header_bytes"`
	RecordBytes  uint32                                 `json:"record_bytes"`
	TotalRows    int                                    `json:"total_rows"`
	ExcludedRows []encyclopediaBindingExcludedSourceRow `json:"excluded_rows"`
}

type encyclopediaBindingExcludedSourceRow struct {
	SourceRow       uint32 `json:"source_row"`
	SourceFamily    uint32 `json:"source_family"`
	DatID           uint32 `json:"dat_id"`
	RawRecordSHA256 string `json:"raw_record_sha256"`
	Reason          string `json:"reason"`
}

type encyclopediaBindingFragmentInput struct {
	FragmentID string `json:"fragment_id"`
	Path       string `json:"path"`
	RawSHA256  string `json:"raw_sha256"`
	RowCount   int    `json:"row_count"`
}

type encyclopediaBindingCategory struct {
	Order                    int      `json:"order"`
	StableKey                string   `json:"stable_key"`
	Command                  uint32   `json:"command"`
	AggregateIndex           bool     `json:"aggregate_index"`
	FilterFamilyStart        *uint32  `json:"filter_family_start"`
	FilterFamilyEndExclusive *uint32  `json:"filter_family_end_exclusive"`
	SourceFamilies           []string `json:"source_families"`
	MembershipRule           string   `json:"membership_rule"`
	Status                   string   `json:"status"`
	BindingCount             int      `json:"binding_count"`
	Reason                   string   `json:"reason"`
	NextProof                string   `json:"next_proof"`
}

type encyclopediaBindingRecord struct {
	Family          string                   `json:"family"`
	SourceRole      string                   `json:"source_role"`
	SourceRow       uint32                   `json:"source_row"`
	SourceFamily    uint32                   `json:"source_family"`
	DatID           uint32                   `json:"dat_id"`
	SourceIdentity  uint32                   `json:"source_identity"`
	RawRecordSHA256 string                   `json:"raw_record_sha256"`
	Variant         string                   `json:"variant"`
	CategoryCommand uint32                   `json:"category_command"`
	Title           encyclopediaBindingTitle `json:"title"`
	Body            encyclopediaBindingBody  `json:"body"`
	Art             encyclopediaBindingArt   `json:"art"`
	Status          string                   `json:"status"`
	AliasOf         *uint32                  `json:"alias_of"`
}

type encyclopediaBindingTitle struct {
	SourceRole          string `json:"source_role"`
	OriginalResourceID  uint32 `json:"original_resource_id"`
	PreferredResourceID uint32 `json:"preferred_resource_id"`
	PreferredStatus     string `json:"preferred_status"`
	SelectedResourceID  uint32 `json:"selected_resource_id"`
	LanguageID          uint32 `json:"language_id"`
}

type encyclopediaBindingBody struct {
	SourceRole string `json:"source_role"`
	ResourceID uint32 `json:"resource_id"`
	LanguageID uint32 `json:"language_id"`
	RawLength  uint64 `json:"raw_length"`
	RawSHA256  string `json:"raw_sha256"`
}

type encyclopediaBindingArt struct {
	SourceRole            string                                 `json:"source_role"`
	SelectorKind          string                                 `json:"selector_kind"`
	PictureID             *uint32                                `json:"picture_id"`
	LookupID              uint32                                 `json:"lookup_id"`
	LanguageID            uint32                                 `json:"language_id"`
	AssetBasename         string                                 `json:"asset_basename"`
	RawLength             uint64                                 `json:"raw_length"`
	RawSHA256             string                                 `json:"raw_sha256"`
	ViewerFactionVariants []encyclopediaBindingArtFactionVariant `json:"viewer_faction_variants,omitempty"`
}

type encyclopediaBindingArtFactionVariant struct {
	ViewerFaction string `json:"viewer_faction"`
	LookupID      uint32 `json:"lookup_id"`
	AssetBasename string `json:"asset_basename"`
	RawLength     uint64 `json:"raw_length"`
	RawSHA256     string `json:"raw_sha256"`
}

type encyclopediaBindingResourceAccounting struct {
	Observations  encyclopediaBindingObservations   `json:"observations"`
	TextResources []encyclopediaBindingTextResource `json:"text_resources"`
	ArtLookups    []encyclopediaBindingArtLookup    `json:"art_lookups"`
	ArtFiles      []encyclopediaBindingArtFile      `json:"art_files"`
}

type encyclopediaBindingObservations struct {
	SourceRowCount              int `json:"source_row_count"`
	BoundRowCount               int `json:"bound_row_count"`
	DocumentedAliasRowCount     int `json:"documented_alias_row_count"`
	SourceProvenUnusedRowCount  int `json:"source_proven_unused_row_count"`
	UnresolvedSourceRowCount    int `json:"unresolved_source_row_count"`
	TextRecordCount             int `json:"text_record_count"`
	NonemptyLookupStringCount   int `json:"nonempty_lookup_string_count"`
	DistinctLookupFilenameCount int `json:"distinct_lookup_filename_count"`
	ImageFileCount              int `json:"image_file_count"`
}

type encyclopediaBindingTextResource struct {
	SourceRole   string `json:"source_role"`
	ResourceID   uint32 `json:"resource_id"`
	LanguageID   uint32 `json:"language_id"`
	RawLength    uint64 `json:"raw_length"`
	RawSHA256    string `json:"raw_sha256"`
	Status       string `json:"status"`
	BindingCount int    `json:"binding_count"`
	Reason       string `json:"reason"`
	NextProof    string `json:"next_proof"`
}

type encyclopediaBindingArtLookup struct {
	SourceRole    string `json:"source_role"`
	LogicalID     uint32 `json:"logical_id"`
	LanguageID    uint32 `json:"language_id"`
	AssetBasename string `json:"asset_basename"`
	Status        string `json:"status"`
	BindingCount  int    `json:"binding_count"`
	Reason        string `json:"reason"`
	NextProof     string `json:"next_proof"`
}

type encyclopediaBindingArtFile struct {
	Basename     string `json:"basename"`
	RawLength    uint64 `json:"raw_length"`
	RawSHA256    string `json:"raw_sha256"`
	Status       string `json:"status"`
	BindingCount int    `json:"binding_count"`
	Reason       string `json:"reason"`
	NextProof    string `json:"next_proof"`
	DeferredTask string `json:"deferred_task"`
}

type encyclopediaBindingSchemaFit struct {
	OneCategoryPerBoundTopic string                     `json:"one_category_per_bound_topic"`
	FullIndexSelector        string                     `json:"full_index_selector"`
	AliasIdentity            string                     `json:"alias_identity"`
	CharacterDiscriminator   string                     `json:"character_discriminator"`
	ClassInstanceBoundary    string                     `json:"class_instance_boundary"`
	ReadyForSchemaFreeze     bool                       `json:"ready_for_schema_freeze"`
	Blockers                 []encyclopediaBindingIssue `json:"blockers"`
}

type encyclopediaBindingIssue struct {
	ID        string `json:"id"`
	Reason    string `json:"reason"`
	NextProof string `json:"next_proof"`
}

func validateEncyclopediaProfileBindings(profile encyclopediaSourceProfile, bindings encyclopediaProfileBindings) error {
	if bindings.Kind != encyclopediaProfileBindingsKind || bindings.SchemaVersion != encyclopediaProfileBindingsSchemaVersion {
		return fmt.Errorf("unsupported kind/schema %q/%d", bindings.Kind, bindings.SchemaVersion)
	}
	if !containsUint32(profile.TextDecoder.LanguageIDs, bindings.LanguageID) {
		return fmt.Errorf("binding language %d is not supported by the decoder", bindings.LanguageID)
	}
	if err := validateEncyclopediaBindingFragmentInputs(bindings); err != nil {
		return err
	}
	sources := make(map[string]encyclopediaProfileSource, len(profile.Sources))
	for _, source := range profile.Sources {
		sources[source.Role] = source
	}
	if err := validateEncyclopediaBindingCategories(bindings); err != nil {
		return err
	}
	if err := validateEncyclopediaBindingRecords(bindings, sources); err != nil {
		return err
	}
	if err := validateEncyclopediaBindingSourceCoverage(bindings, sources); err != nil {
		return err
	}
	if err := validateEncyclopediaBindingResourceAccounting(bindings, sources, profile.Evidence.Corpus); err != nil {
		return err
	}
	return validateEncyclopediaBindingSchemaFit(bindings)
}

func validateEncyclopediaBindingFragmentInputs(bindings encyclopediaProfileBindings) error {
	if len(bindings.FragmentInputs) == 0 {
		return fmt.Errorf("no binding fragment provenance")
	}
	seenIDs := make(map[string]struct{}, len(bindings.FragmentInputs))
	seenPaths := make(map[string]struct{}, len(bindings.FragmentInputs))
	rows := 0
	for _, input := range bindings.FragmentInputs {
		if !validStableBindingName(input.FragmentID) {
			return fmt.Errorf("invalid fragment ID %q", input.FragmentID)
		}
		if _, exists := seenIDs[input.FragmentID]; exists {
			return fmt.Errorf("duplicate fragment ID %q", input.FragmentID)
		}
		seenIDs[input.FragmentID] = struct{}{}
		clean := filepath.ToSlash(filepath.Clean(input.Path))
		if clean != input.Path || strings.HasPrefix(clean, "../") || strings.HasPrefix(clean, "/") || !strings.HasPrefix(clean, "encyclopedia_profiles/fragments/") || filepath.Ext(clean) != ".json" {
			return fmt.Errorf("invalid fragment path %q", input.Path)
		}
		if _, exists := seenPaths[clean]; exists {
			return fmt.Errorf("duplicate fragment path %q", clean)
		}
		seenPaths[clean] = struct{}{}
		if !validSHA256(input.RawSHA256) || input.RowCount < 1 {
			return fmt.Errorf("fragment %q has invalid hash/count", input.FragmentID)
		}
		rows += input.RowCount
	}
	if rows != len(bindings.Records) {
		return fmt.Errorf("fragment row count %d does not close over %d combined records", rows, len(bindings.Records))
	}
	return nil
}

func validateEncyclopediaBindingCategories(bindings encyclopediaProfileBindings) error {
	wantCommands := []uint32{0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75}
	if len(bindings.Categories) != len(wantCommands) {
		return fmt.Errorf("category count = %d, want %d source commands", len(bindings.Categories), len(wantCommands))
	}
	counts := make(map[uint32]int)
	families := make(map[uint32]map[string]struct{})
	for _, record := range bindings.Records {
		counts[record.CategoryCommand]++
		if families[record.CategoryCommand] == nil {
			families[record.CategoryCommand] = make(map[string]struct{})
		}
		families[record.CategoryCommand][record.Family] = struct{}{}
	}
	seenKeys := make(map[string]struct{}, len(bindings.Categories))
	for index, category := range bindings.Categories {
		if category.Order != index || category.Command != wantCommands[index] {
			return fmt.Errorf("category order %d has command 0x%x, want 0x%x", index, category.Command, wantCommands[index])
		}
		if !validStableBindingName(category.StableKey) || strings.TrimSpace(category.MembershipRule) == "" {
			return fmt.Errorf("category 0x%x has incomplete stable identity/membership", category.Command)
		}
		if _, exists := seenKeys[category.StableKey]; exists {
			return fmt.Errorf("duplicate category stable key %q", category.StableKey)
		}
		seenKeys[category.StableKey] = struct{}{}
		if category.Command == 0x6f {
			if !category.AggregateIndex || category.FilterFamilyStart != nil || category.FilterFamilyEndExclusive != nil || category.BindingCount != len(bindings.Records) {
				return fmt.Errorf("command 0x6f must be the complete aggregate index")
			}
		} else {
			if category.AggregateIndex || category.FilterFamilyStart == nil || category.FilterFamilyEndExclusive == nil || *category.FilterFamilyStart >= *category.FilterFamilyEndExclusive {
				return fmt.Errorf("category 0x%x has invalid family filter", category.Command)
			}
			if category.BindingCount != counts[category.Command] {
				return fmt.Errorf("category 0x%x binding count = %d, want %d", category.Command, category.BindingCount, counts[category.Command])
			}
			declaredFamilies := make(map[string]struct{}, len(category.SourceFamilies))
			for _, family := range category.SourceFamilies {
				if !validStableBindingName(family) {
					return fmt.Errorf("category 0x%x has invalid source family %q", category.Command, family)
				}
				if _, exists := declaredFamilies[family]; exists {
					return fmt.Errorf("category 0x%x repeats source family %q", category.Command, family)
				}
				declaredFamilies[family] = struct{}{}
			}
			if len(declaredFamilies) != len(families[category.Command]) {
				return fmt.Errorf("category 0x%x source-family coverage is incomplete", category.Command)
			}
			for family := range families[category.Command] {
				if _, exists := declaredFamilies[family]; !exists {
					return fmt.Errorf("category 0x%x omits bound source family %q", category.Command, family)
				}
			}
		}
		switch category.Status {
		case "complete":
			if category.Reason != "" || category.NextProof != "" {
				return fmt.Errorf("complete category 0x%x carries unresolved evidence", category.Command)
			}
		case encyclopediaBindingUnresolved:
			if strings.TrimSpace(category.Reason) == "" || strings.TrimSpace(category.NextProof) == "" {
				return fmt.Errorf("unresolved category 0x%x lacks reason/next proof", category.Command)
			}
		default:
			return fmt.Errorf("category 0x%x has invalid status %q", category.Command, category.Status)
		}
	}
	return nil
}

func validateEncyclopediaBindingRecords(bindings encyclopediaProfileBindings, sources map[string]encyclopediaProfileSource) error {
	if len(bindings.Records) == 0 {
		return fmt.Errorf("no combined binding records")
	}
	categories := make(map[uint32]encyclopediaBindingCategory, len(bindings.Categories))
	for _, category := range bindings.Categories {
		categories[category.Command] = category
	}
	tupleSeen := make(map[string]struct{}, len(bindings.Records))
	identitySeen := make(map[uint32]struct{}, len(bindings.Records))
	recordByIdentity := make(map[uint32]encyclopediaBindingRecord, len(bindings.Records))
	topicOwners := make(map[string][]encyclopediaBindingRecord)
	sourceRows := make(map[string]map[uint32]struct{})
	for index, record := range bindings.Records {
		if !validStableBindingName(record.Family) || record.Variant == "" {
			return fmt.Errorf("record %d has invalid family/variant", index)
		}
		source, ok := sources[record.SourceRole]
		if !ok || source.Kind != encyclopediaSourceDAT {
			return fmt.Errorf("record %d references missing/non-DAT source role %q", index, record.SourceRole)
		}
		if record.SourceFamily > 0xff || record.DatID > 0x00ffffff || record.SourceIdentity != record.SourceFamily<<24|record.DatID {
			return fmt.Errorf("record %d has inconsistent family-qualified source identity", index)
		}
		tuple := fmt.Sprintf("%d/%d/%s", record.SourceFamily, record.DatID, record.Variant)
		if _, exists := tupleSeen[tuple]; exists {
			return fmt.Errorf("duplicate binding tuple %s", tuple)
		}
		tupleSeen[tuple] = struct{}{}
		if _, exists := identitySeen[record.SourceIdentity]; exists {
			return fmt.Errorf("ambiguous source identity 0x%08x", record.SourceIdentity)
		}
		if index > 0 && record.SourceIdentity < bindings.Records[index-1].SourceIdentity {
			return fmt.Errorf("combined bindings are not canonically ordered by source identity")
		}
		identitySeen[record.SourceIdentity] = struct{}{}
		recordByIdentity[record.SourceIdentity] = record
		if sourceRows[record.SourceRole] == nil {
			sourceRows[record.SourceRole] = make(map[uint32]struct{})
		}
		if _, exists := sourceRows[record.SourceRole][record.SourceRow]; exists {
			return fmt.Errorf("source %q repeats row %d", record.SourceRole, record.SourceRow)
		}
		sourceRows[record.SourceRole][record.SourceRow] = struct{}{}
		if record.RawRecordSHA256 != "" && !validSHA256(record.RawRecordSHA256) {
			return fmt.Errorf("record 0x%08x has invalid raw-record hash", record.SourceIdentity)
		}
		category, ok := categories[record.CategoryCommand]
		if !ok {
			return fmt.Errorf("record 0x%08x has invalid category command 0x%x", record.SourceIdentity, record.CategoryCommand)
		}
		if category.AggregateIndex {
			for _, filtered := range bindings.Categories {
				if filtered.AggregateIndex || filtered.FilterFamilyStart == nil || filtered.FilterFamilyEndExclusive == nil {
					continue
				}
				if record.SourceFamily >= *filtered.FilterFamilyStart && record.SourceFamily < *filtered.FilterFamilyEndExclusive {
					return fmt.Errorf("record 0x%08x uses aggregate-only membership despite matching category 0x%x", record.SourceIdentity, filtered.Command)
				}
			}
		} else {
			if category.FilterFamilyStart == nil || category.FilterFamilyEndExclusive == nil || record.SourceFamily < *category.FilterFamilyStart || record.SourceFamily >= *category.FilterFamilyEndExclusive {
				return fmt.Errorf("record 0x%08x family %d is outside category 0x%x", record.SourceIdentity, record.SourceFamily, category.Command)
			}
		}
		if err := validateEncyclopediaBindingRecordSelectors(bindings, record, sources); err != nil {
			return fmt.Errorf("record 0x%08x: %w", record.SourceIdentity, err)
		}
		switch record.Status {
		case encyclopediaBindingBound:
			if record.AliasOf != nil {
				return fmt.Errorf("bound record unexpectedly declares alias_of")
			}
		case encyclopediaBindingDocumentedAlias:
			if record.AliasOf == nil || *record.AliasOf == record.SourceIdentity {
				return fmt.Errorf("documented alias lacks a distinct canonical identity")
			}
		default:
			return fmt.Errorf("unsupported combined record status %q", record.Status)
		}
		topicKey := resourceLanguageKey(record.Body.LanguageID, record.Body.ResourceID)
		topicOwners[topicKey] = append(topicOwners[topicKey], record)
	}
	coveredRoles := make(map[string]struct{}, len(bindings.SourceCoverage))
	for _, coverage := range bindings.SourceCoverage {
		coveredRoles[coverage.SourceRole] = struct{}{}
	}
	for sourceRole, rows := range sourceRows {
		if _, covered := coveredRoles[sourceRole]; covered {
			continue
		}
		for row := 0; row < len(rows); row++ {
			if _, exists := rows[uint32(row)]; !exists {
				return fmt.Errorf("source %q omits row %d", sourceRole, row)
			}
		}
	}

	for key, owners := range topicOwners {
		if len(owners) == 1 {
			if owners[0].Status == encyclopediaBindingDocumentedAlias {
				return fmt.Errorf("topic %s has an alias without its canonical record", key)
			}
			continue
		}
		canonicalCount := 0
		var canonical encyclopediaBindingRecord
		for _, owner := range owners {
			if owner.Status == encyclopediaBindingBound {
				canonicalCount++
				canonical = owner
			}
		}
		if canonicalCount != 1 {
			return fmt.Errorf("topic %s has %d canonical records; duplicate resources require one explicit alias owner", key, canonicalCount)
		}
		for _, owner := range owners {
			if owner.Status != encyclopediaBindingDocumentedAlias {
				continue
			}
			if owner.AliasOf == nil || *owner.AliasOf != canonical.SourceIdentity || owner.Body.RawLength != canonical.Body.RawLength || owner.Body.RawSHA256 != canonical.Body.RawSHA256 {
				return fmt.Errorf("topic %s has an inconsistent documented alias", key)
			}
			if _, exists := recordByIdentity[*owner.AliasOf]; !exists {
				return fmt.Errorf("topic %s aliases missing source identity", key)
			}
		}
	}
	return nil
}

func validateEncyclopediaBindingSourceCoverage(bindings encyclopediaProfileBindings, sources map[string]encyclopediaProfileSource) error {
	recordRows := make(map[string]map[uint32]encyclopediaBindingRecord)
	for _, record := range bindings.Records {
		if recordRows[record.SourceRole] == nil {
			recordRows[record.SourceRole] = make(map[uint32]encyclopediaBindingRecord)
		}
		recordRows[record.SourceRole][record.SourceRow] = record
	}
	seenRoles := make(map[string]struct{}, len(bindings.SourceCoverage))
	for _, coverage := range bindings.SourceCoverage {
		if _, exists := seenRoles[coverage.SourceRole]; exists {
			return fmt.Errorf("duplicate source-row coverage for %q", coverage.SourceRole)
		}
		seenRoles[coverage.SourceRole] = struct{}{}
		source, exists := sources[coverage.SourceRole]
		if !exists || source.Kind != encyclopediaSourceDAT {
			return fmt.Errorf("source-row coverage references missing/non-DAT source role %q", coverage.SourceRole)
		}
		if coverage.HeaderBytes == 0 || coverage.RecordBytes == 0 || coverage.TotalRows < 1 || source.RawLength < uint64(coverage.HeaderBytes) {
			return fmt.Errorf("source-row coverage for %q contradicts source length", coverage.SourceRole)
		}
		bodyBytes := source.RawLength - uint64(coverage.HeaderBytes)
		if bodyBytes%uint64(coverage.RecordBytes) != 0 || bodyBytes/uint64(coverage.RecordBytes) != uint64(coverage.TotalRows) {
			return fmt.Errorf("source-row coverage for %q contradicts source length", coverage.SourceRole)
		}
		coveredRows := make(map[uint32]struct{}, coverage.TotalRows)
		for sourceRow, record := range recordRows[coverage.SourceRole] {
			if sourceRow >= uint32(coverage.TotalRows) || !validSHA256(record.RawRecordSHA256) {
				return fmt.Errorf("source-row coverage for %q has invalid bound row %d", coverage.SourceRole, sourceRow)
			}
			coveredRows[sourceRow] = struct{}{}
		}
		previous := uint32(0)
		for index, excluded := range coverage.ExcludedRows {
			if excluded.SourceRow >= uint32(coverage.TotalRows) || excluded.SourceFamily > 0xff || excluded.DatID > 0x00ffffff || !validSHA256(excluded.RawRecordSHA256) || strings.TrimSpace(excluded.Reason) == "" {
				return fmt.Errorf("source-row coverage for %q has invalid excluded row %d", coverage.SourceRole, excluded.SourceRow)
			}
			if index > 0 && excluded.SourceRow <= previous {
				return fmt.Errorf("source-row coverage for %q has noncanonical/duplicate excluded rows", coverage.SourceRole)
			}
			previous = excluded.SourceRow
			if _, exists := coveredRows[excluded.SourceRow]; exists {
				return fmt.Errorf("source-row coverage for %q classifies row %d more than once", coverage.SourceRole, excluded.SourceRow)
			}
			coveredRows[excluded.SourceRow] = struct{}{}
		}
		if len(coveredRows) != coverage.TotalRows {
			return fmt.Errorf("source-row coverage for %q accounts for %d of %d rows", coverage.SourceRole, len(coveredRows), coverage.TotalRows)
		}
		for row := 0; row < coverage.TotalRows; row++ {
			if _, exists := coveredRows[uint32(row)]; !exists {
				return fmt.Errorf("source-row coverage for %q omits row %d", coverage.SourceRole, row)
			}
		}
	}
	return nil
}

func validateEncyclopediaBindingRecordSelectors(bindings encyclopediaProfileBindings, record encyclopediaBindingRecord, sources map[string]encyclopediaProfileSource) error {
	titleSource, ok := sources[record.Title.SourceRole]
	if !ok || titleSource.Kind != encyclopediaSourceDLL || record.Title.LanguageID != bindings.LanguageID {
		return fmt.Errorf("invalid title source/language evidence for role %q", record.Title.SourceRole)
	}
	bodySource, ok := sources[record.Body.SourceRole]
	if !ok || bodySource.Kind != encyclopediaSourceDLL || record.Body.LanguageID != bindings.LanguageID || record.Body.RawLength == 0 || !validSHA256(record.Body.RawSHA256) {
		return fmt.Errorf("invalid body source/resource evidence")
	}
	artSource, ok := sources[record.Art.SourceRole]
	if !ok || artSource.Kind != encyclopediaSourceDLL || record.Art.LanguageID != bindings.LanguageID {
		return fmt.Errorf("invalid art source/language evidence")
	}
	wantBody := (record.Title.OriginalResourceID & 0x0fff) + 0x1000
	if record.Body.ResourceID != wantBody {
		return fmt.Errorf("body resource %d does not derive from original title selector %d", record.Body.ResourceID, record.Title.OriginalResourceID)
	}
	wantPreferred := (record.Title.OriginalResourceID - 0x8000) & 0xffff
	if record.Title.PreferredResourceID != wantPreferred {
		return fmt.Errorf("preferred title resource %d does not derive from original selector %d", record.Title.PreferredResourceID, record.Title.OriginalResourceID)
	}
	switch record.Title.PreferredStatus {
	case "empty":
		if record.Title.SelectedResourceID != record.Title.OriginalResourceID {
			return fmt.Errorf("empty preferred title did not select original fallback")
		}
	case "nonempty":
		if record.Title.SelectedResourceID != record.Title.PreferredResourceID {
			return fmt.Errorf("nonempty preferred title was not selected")
		}
	default:
		return fmt.Errorf("unsupported preferred-title status %q", record.Title.PreferredStatus)
	}
	switch record.Art.SelectorKind {
	case "canonical_topic_key":
		if record.Art.PictureID != nil || len(record.Art.ViewerFactionVariants) != 0 || record.Art.LookupID != record.Body.ResourceID || !validEncyclopediaBindingArtPayload(record.Art.AssetBasename, record.Art.RawLength, record.Art.RawSHA256) {
			return fmt.Errorf("canonical-topic art selector does not match body key")
		}
	case "system_picture":
		if record.Art.PictureID == nil || *record.Art.PictureID == 0 || len(record.Art.ViewerFactionVariants) != 0 || !validEncyclopediaBindingArtPayload(record.Art.AssetBasename, record.Art.RawLength, record.Art.RawSHA256) {
			return fmt.Errorf("system-picture art selector lacks picture ID")
		}
	case "viewer_faction_topic_key":
		family := record.SourceFamily
		if !((family >= 0x40 && family < 0x80) || (family >= 0x08 && family < 0x10)) {
			return fmt.Errorf("viewer-faction art selector is invalid for source family 0x%x", family)
		}
		if record.Art.PictureID != nil || record.Art.LookupID != 0 || record.Art.AssetBasename != "" || record.Art.RawLength != 0 || record.Art.RawSHA256 != "" || len(record.Art.ViewerFactionVariants) != 2 {
			return fmt.Errorf("viewer-faction art selector must carry exactly two faction variants")
		}
		wantFactions := []string{"alliance", "empire"}
		for index, variant := range record.Art.ViewerFactionVariants {
			if variant.ViewerFaction != wantFactions[index] || !validEncyclopediaBindingArtPayload(variant.AssetBasename, variant.RawLength, variant.RawSHA256) {
				return fmt.Errorf("viewer-faction art selector has invalid faction variants")
			}
			wantLookup := record.Body.ResourceID + uint32(index)*0x1000
			if variant.LookupID != wantLookup {
				return fmt.Errorf("%s lookup %d does not match source selector %d", variant.ViewerFaction, variant.LookupID, wantLookup)
			}
		}
	default:
		return fmt.Errorf("unsupported art selector kind %q", record.Art.SelectorKind)
	}
	return nil
}

func validEncyclopediaBindingArtPayload(basename string, rawLength uint64, rawSHA256 string) bool {
	return validEDataBasename(basename) && rawLength > 0 && validSHA256(rawSHA256)
}

type encyclopediaBindingArtSelection struct {
	LookupID      uint32
	AssetBasename string
	RawLength     uint64
	RawSHA256     string
}

func encyclopediaBindingArtSelections(art encyclopediaBindingArt) []encyclopediaBindingArtSelection {
	if art.SelectorKind != "viewer_faction_topic_key" {
		return []encyclopediaBindingArtSelection{{
			LookupID:      art.LookupID,
			AssetBasename: art.AssetBasename,
			RawLength:     art.RawLength,
			RawSHA256:     art.RawSHA256,
		}}
	}
	selections := make([]encyclopediaBindingArtSelection, 0, len(art.ViewerFactionVariants))
	for _, variant := range art.ViewerFactionVariants {
		selections = append(selections, encyclopediaBindingArtSelection{
			LookupID:      variant.LookupID,
			AssetBasename: variant.AssetBasename,
			RawLength:     variant.RawLength,
			RawSHA256:     variant.RawSHA256,
		})
	}
	return selections
}

func validateEncyclopediaBindingResourceAccounting(bindings encyclopediaProfileBindings, sources map[string]encyclopediaProfileSource, corpus encyclopediaDecoderCorpusEvidence) error {
	accounting := bindings.ResourceAccounting
	observations := accounting.Observations
	sourceProvenUnusedRows := 0
	for _, coverage := range bindings.SourceCoverage {
		sourceProvenUnusedRows += len(coverage.ExcludedRows)
	}
	if observations.SourceRowCount != len(bindings.Records)+sourceProvenUnusedRows || observations.SourceProvenUnusedRowCount != sourceProvenUnusedRows || observations.TextRecordCount != len(accounting.TextResources) || observations.NonemptyLookupStringCount != len(accounting.ArtLookups) || observations.ImageFileCount != len(accounting.ArtFiles) {
		return fmt.Errorf("resource observation counts do not match the represented inventory")
	}
	boundRows, aliasRows := 0, 0
	for _, record := range bindings.Records {
		if record.Status == encyclopediaBindingBound {
			boundRows++
		} else if record.Status == encyclopediaBindingDocumentedAlias {
			aliasRows++
		}
	}
	if observations.BoundRowCount != boundRows || observations.DocumentedAliasRowCount != aliasRows || observations.UnresolvedSourceRowCount != 0 {
		return fmt.Errorf("source-row status counts do not match combined bindings")
	}

	bodyRefs := make(map[string][]encyclopediaBindingRecord)
	lookupRefs := make(map[string][]encyclopediaBindingRecord)
	fileRefs := make(map[string][]encyclopediaBindingArtSelection)
	for _, record := range bindings.Records {
		bodyKey := resourceLanguageKey(record.Body.LanguageID, record.Body.ResourceID)
		bodyRefs[bodyKey] = append(bodyRefs[bodyKey], record)
		for _, selection := range encyclopediaBindingArtSelections(record.Art) {
			lookupKey := resourceLanguageKey(record.Art.LanguageID, selection.LookupID)
			lookupRefs[lookupKey] = append(lookupRefs[lookupKey], record)
			fileKey := strings.ToLower(selection.AssetBasename)
			fileRefs[fileKey] = append(fileRefs[fileKey], selection)
		}
	}

	seenText := make(map[string]struct{}, len(accounting.TextResources))
	for index, resource := range accounting.TextResources {
		if index > 0 {
			previous := accounting.TextResources[index-1]
			if resource.LanguageID < previous.LanguageID || resource.LanguageID == previous.LanguageID && resource.ResourceID <= previous.ResourceID {
				return fmt.Errorf("text resource accounting is not canonically ordered")
			}
		}
		key := resourceLanguageKey(resource.LanguageID, resource.ResourceID)
		if _, exists := seenText[key]; exists {
			return fmt.Errorf("duplicate text accounting identity %s", key)
		}
		seenText[key] = struct{}{}
		source, ok := sources[resource.SourceRole]
		if !ok || source.Kind != encyclopediaSourceDLL || resource.RawLength == 0 || !validSHA256(resource.RawSHA256) {
			return fmt.Errorf("text accounting %s has invalid source evidence", key)
		}
		refs := bodyRefs[key]
		if err := validateBindingAccountingStatus(resource.Status, resource.BindingCount, len(refs), resource.Reason, resource.NextProof, false); err != nil {
			return fmt.Errorf("text accounting %s: %w", key, err)
		}
		for _, ref := range refs {
			if ref.Body.SourceRole != resource.SourceRole || ref.Body.RawLength != resource.RawLength || ref.Body.RawSHA256 != resource.RawSHA256 {
				return fmt.Errorf("text accounting %s contradicts bound record evidence", key)
			}
		}
	}
	for key := range bodyRefs {
		if _, exists := seenText[key]; !exists {
			return fmt.Errorf("bound text resource %s is unaccounted", key)
		}
	}
	if observations.TextRecordCount != corpus.RecordCount {
		return fmt.Errorf("text resource accounting count %d does not match decoder corpus record count %d", observations.TextRecordCount, corpus.RecordCount)
	}
	if bindings.LanguageID != corpus.LanguageID {
		return fmt.Errorf("binding language %d does not match decoder corpus language %d", bindings.LanguageID, corpus.LanguageID)
	}

	seenLookups := make(map[string]struct{}, len(accounting.ArtLookups))
	filenameSet := make(map[string]struct{})
	for index, lookup := range accounting.ArtLookups {
		if index > 0 {
			previous := accounting.ArtLookups[index-1]
			if lookup.LanguageID < previous.LanguageID || lookup.LanguageID == previous.LanguageID && lookup.LogicalID <= previous.LogicalID {
				return fmt.Errorf("art lookup accounting is not canonically ordered")
			}
		}
		key := resourceLanguageKey(lookup.LanguageID, lookup.LogicalID)
		if _, exists := seenLookups[key]; exists {
			return fmt.Errorf("duplicate art lookup identity %s", key)
		}
		seenLookups[key] = struct{}{}
		source, ok := sources[lookup.SourceRole]
		if !ok || source.Kind != encyclopediaSourceDLL || !validEDataBasename(lookup.AssetBasename) {
			return fmt.Errorf("art lookup %s has invalid source evidence", key)
		}
		filenameSet[strings.ToLower(lookup.AssetBasename)] = struct{}{}
		refs := lookupRefs[key]
		if err := validateBindingAccountingStatus(lookup.Status, lookup.BindingCount, len(refs), lookup.Reason, lookup.NextProof, false); err != nil {
			return fmt.Errorf("art lookup %s: %w", key, err)
		}
		for _, ref := range refs {
			matched := false
			for _, selection := range encyclopediaBindingArtSelections(ref.Art) {
				if selection.LookupID == lookup.LogicalID && strings.EqualFold(selection.AssetBasename, lookup.AssetBasename) {
					matched = true
					break
				}
			}
			if ref.Art.SourceRole != lookup.SourceRole || !matched {
				return fmt.Errorf("art lookup %s contradicts bound record evidence", key)
			}
		}
	}
	for key := range lookupRefs {
		if _, exists := seenLookups[key]; !exists {
			return fmt.Errorf("bound art lookup %s is unaccounted", key)
		}
	}
	if observations.DistinctLookupFilenameCount != len(filenameSet) {
		return fmt.Errorf("distinct lookup filename count = %d, want %d represented names", observations.DistinctLookupFilenameCount, len(filenameSet))
	}

	seenFiles := make(map[string]struct{}, len(accounting.ArtFiles))
	for index, file := range accounting.ArtFiles {
		if index > 0 && file.Basename <= accounting.ArtFiles[index-1].Basename {
			return fmt.Errorf("art file accounting is not canonically ordered")
		}
		key := strings.ToLower(file.Basename)
		if !validEDataBasename(file.Basename) || file.RawLength == 0 || !validSHA256(file.RawSHA256) {
			return fmt.Errorf("art file %q has invalid identity", file.Basename)
		}
		if _, exists := seenFiles[key]; exists {
			return fmt.Errorf("duplicate/case-ambiguous art file %q", file.Basename)
		}
		seenFiles[key] = struct{}{}
		refs := fileRefs[key]
		allowDeferred := file.Status == encyclopediaBindingPublicationDeferred
		if err := validateBindingAccountingStatus(file.Status, file.BindingCount, len(refs), file.Reason, file.NextProof, allowDeferred); err != nil {
			return fmt.Errorf("art file %q: %w", file.Basename, err)
		}
		for _, selection := range refs {
			if !strings.EqualFold(selection.AssetBasename, file.Basename) || selection.RawLength != file.RawLength || selection.RawSHA256 != file.RawSHA256 {
				return fmt.Errorf("art selection %d/%q contradicts accounted file evidence", selection.LookupID, selection.AssetBasename)
			}
		}
		if file.Status == encyclopediaBindingPublicationDeferred {
			if !validStableBindingName(file.DeferredTask) {
				return fmt.Errorf("publication-deferred art file %q lacks a stable follow-up task", file.Basename)
			}
		} else if file.DeferredTask != "" {
			return fmt.Errorf("non-deferred art file %q carries a deferred task", file.Basename)
		}
	}
	for key := range fileRefs {
		if _, exists := seenFiles[key]; !exists {
			return fmt.Errorf("bound art file %q is unaccounted", key)
		}
	}
	for key := range filenameSet {
		if _, exists := seenFiles[key]; !exists {
			return fmt.Errorf("art lookup references missing accounted file %q", key)
		}
	}
	return nil
}

func validateBindingAccountingStatus(status string, declaredCount, actualCount int, reason, nextProof string, allowDeferred bool) error {
	if status == encyclopediaBindingSourceProvenUnused && (declaredCount != 0 || actualCount != 0 || strings.TrimSpace(reason) == "" || nextProof != "") {
		return fmt.Errorf("source-proven-unused resource has a binding or incomplete closed evidence")
	}
	if declaredCount != actualCount {
		return fmt.Errorf("binding count = %d, want %d", declaredCount, actualCount)
	}
	switch status {
	case encyclopediaBindingBound:
		if actualCount < 1 || reason != "" || nextProof != "" {
			return fmt.Errorf("bound resource has invalid count or unresolved evidence")
		}
	case encyclopediaBindingUnresolved:
		if actualCount != 0 || strings.TrimSpace(reason) == "" || strings.TrimSpace(nextProof) == "" {
			return fmt.Errorf("unresolved resource lacks zero-count reason/next proof")
		}
	case encyclopediaBindingSourceProvenUnused:
		// The complete zero-reference evidence was checked before the generic
		// declared/actual count comparison so contradictions get a useful error.
	case encyclopediaBindingPublicationDeferred:
		if !allowDeferred || actualCount != 0 || strings.TrimSpace(reason) == "" || strings.TrimSpace(nextProof) == "" {
			return fmt.Errorf("publication-deferred resource lacks explicit policy/proof gate")
		}
	default:
		return fmt.Errorf("unsupported status %q", status)
	}
	return nil
}

func validateEncyclopediaBindingSchemaFit(bindings encyclopediaProfileBindings) error {
	fit := bindings.SchemaFit
	if strings.TrimSpace(fit.OneCategoryPerBoundTopic) == "" || strings.TrimSpace(fit.FullIndexSelector) == "" || strings.TrimSpace(fit.AliasIdentity) == "" || strings.TrimSpace(fit.CharacterDiscriminator) == "" || strings.TrimSpace(fit.ClassInstanceBoundary) == "" {
		return fmt.Errorf("schema-fit decisions are incomplete")
	}
	seen := make(map[string]struct{}, len(fit.Blockers))
	for _, blocker := range fit.Blockers {
		if !validStableBindingName(blocker.ID) || strings.TrimSpace(blocker.Reason) == "" || strings.TrimSpace(blocker.NextProof) == "" {
			return fmt.Errorf("invalid schema-fit blocker %q", blocker.ID)
		}
		if _, exists := seen[blocker.ID]; exists {
			return fmt.Errorf("duplicate schema-fit blocker %q", blocker.ID)
		}
		seen[blocker.ID] = struct{}{}
	}
	if fit.ReadyForSchemaFreeze && len(fit.Blockers) != 0 {
		return fmt.Errorf("schema-freeze readiness contradicts %d open blockers", len(fit.Blockers))
	}
	unresolvedEvidence := countUnresolvedEncyclopediaBindingEvidence(bindings)
	if fit.ReadyForSchemaFreeze && unresolvedEvidence != 0 {
		return fmt.Errorf("schema-freeze readiness contradicts unresolved evidence in %d category/resource records", unresolvedEvidence)
	}
	aggregateCommands := make(map[uint32]struct{})
	for _, category := range bindings.Categories {
		if category.AggregateIndex {
			aggregateCommands[category.Command] = struct{}{}
		}
	}
	hasAggregateOnlyTopic := false
	for _, record := range bindings.Records {
		if _, aggregateOnly := aggregateCommands[record.CategoryCommand]; aggregateOnly {
			hasAggregateOnlyTopic = true
			break
		}
	}
	_, hasAggregateOnlyBlocker := seen[encyclopediaBindingAggregateOnlyBlocker]
	if hasAggregateOnlyTopic && !hasAggregateOnlyBlocker {
		return fmt.Errorf("schema-fit requires %q blocker for aggregate-only topic membership", encyclopediaBindingAggregateOnlyBlocker)
	}
	if !fit.ReadyForSchemaFreeze && len(fit.Blockers) == 0 {
		return fmt.Errorf("schema-freeze not ready without named blockers")
	}
	return nil
}

func countUnresolvedEncyclopediaBindingEvidence(bindings encyclopediaProfileBindings) int {
	count := 0
	for _, category := range bindings.Categories {
		if category.Status == encyclopediaBindingUnresolved {
			count++
		}
	}
	for _, resource := range bindings.ResourceAccounting.TextResources {
		if resource.Status == encyclopediaBindingUnresolved {
			count++
		}
	}
	for _, lookup := range bindings.ResourceAccounting.ArtLookups {
		if lookup.Status == encyclopediaBindingUnresolved {
			count++
		}
	}
	for _, file := range bindings.ResourceAccounting.ArtFiles {
		if file.Status == encyclopediaBindingUnresolved {
			count++
		}
	}
	return count
}

func validStableBindingName(value string) bool {
	if value == "" {
		return false
	}
	for _, r := range value {
		if (r >= 'a' && r <= 'z') || (r >= '0' && r <= '9') || r == '_' || r == '-' {
			continue
		}
		return false
	}
	return true
}

func validEDataBasename(value string) bool {
	if len(value) != len("EDATA.000") || !strings.HasPrefix(value, "EDATA.") {
		return false
	}
	number, err := strconv.Atoi(value[len("EDATA."):])
	return err == nil && number >= 0 && number <= 999
}

func resourceLanguageKey(languageID, resourceID uint32) string {
	return fmt.Sprintf("%d/%d", languageID, resourceID)
}
