package main

import (
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"strconv"
)

const (
	encyclopediaCatalogSchemaVersion = 1
	encyclopediaMaxTopics            = 10_000
	encyclopediaMaxTitleBytes        = 64 << 10
	encyclopediaMaxBodyBytes         = 1 << 20
	encyclopediaMaxCatalogJSONBytes  = 64 << 20
	encyclopediaMaxCatalogJSONDepth  = 16
	encyclopediaMaxManifestJSONBytes = 32 << 20
	encyclopediaMaxManifestJSONDepth = 16
)

type encyclopediaCatalogProfileContract struct {
	TopicSort               encyclopediaTopicSort              `json:"topic_sort"`
	DefinitionCount         int                                `json:"definition_count"`
	DefinitionOrderSHA256   string                             `json:"definition_order_sha256"`
	SystemOrderSHA256       string                             `json:"system_order_sha256"`
	RegistryOrder           []uint32                           `json:"registry_order"`
	RegistryOrderSHA256     string                             `json:"registry_order_sha256"`
	SourceOrderOracleSHA256 string                             `json:"source_order_oracle_sha256"`
	CategoryLabels          []encyclopediaCatalogCategoryLabel `json:"category_labels"`
}

type encyclopediaCatalogCategoryLabel struct {
	Command    uint32 `json:"command"`
	LanguageID uint32 `json:"language_id"`
	SourceRole string `json:"source_role"`
	ResourceID uint32 `json:"resource_id"`
	RawLength  uint64 `json:"raw_length"`
	RawSHA256  string `json:"raw_sha256"`
	UTF8Length uint64 `json:"utf8_length"`
	UTF8SHA256 string `json:"utf8_sha256"`
	SourceRef  string `json:"source_ref"`
}

type encyclopediaTopicSort struct {
	Algorithm             string `json:"algorithm"`
	RepresentableEncoding string `json:"representable_encoding"`
	RepresentableFold     string `json:"representable_fold"`
	Unrepresentable       string `json:"unrepresentable"`
	TieBreak              string `json:"tie_break"`
}

type encyclopediaCatalog struct {
	SchemaVersion   int                           `json:"schema_version"`
	DefaultLanguage string                        `json:"default_language"`
	TopicSort       encyclopediaTopicSort         `json:"topic_sort"`
	Index           encyclopediaCatalogView       `json:"index"`
	Categories      []encyclopediaCatalogCategory `json:"categories"`
	Topics          map[string]encyclopediaTopic  `json:"topics"`
	Images          map[string]encyclopediaImage  `json:"images"`
	Bindings        []encyclopediaCatalogBinding  `json:"bindings"`
}

type encyclopediaCatalogView struct {
	Command   string            `json:"command"`
	Labels    map[string]string `json:"labels"`
	TopicIDs  []string          `json:"topic_ids"`
	SourceRef string            `json:"source_ref"`
}

type encyclopediaCatalogCategory struct {
	ID        string            `json:"id"`
	Command   string            `json:"command"`
	Labels    map[string]string `json:"labels"`
	TopicIDs  []string          `json:"topic_ids"`
	SourceRef string            `json:"source_ref"`
}

type encyclopediaTopic struct {
	Localized map[string]encyclopediaLocalizedContent `json:"localized"`
	SourceRef string                                  `json:"source_ref"`
}

type encyclopediaLocalizedContent struct {
	Title         string                     `json:"title"`
	Body          string                     `json:"body"`
	ImageID       *string                    `json:"image_id,omitempty"`
	ImageSelector *encyclopediaImageSelector `json:"image_selector,omitempty"`
}

type encyclopediaImageSelector struct {
	Kind            string                      `json:"kind"`
	AllianceImageID encyclopediaNullableImageID `json:"alliance_image_id"`
	EmpireImageID   encyclopediaNullableImageID `json:"empire_image_id"`
}

type encyclopediaNullableImageID struct {
	Present bool
	Value   *string
}

func (value *encyclopediaNullableImageID) UnmarshalJSON(data []byte) error {
	value.Present = true
	if bytes.Equal(data, []byte("null")) {
		value.Value = nil
		return nil
	}
	var decoded string
	if err := json.Unmarshal(data, &decoded); err != nil {
		return err
	}
	value.Value = &decoded
	return nil
}

func (value encyclopediaNullableImageID) MarshalJSON() ([]byte, error) {
	if value.Value == nil {
		return []byte("null"), nil
	}
	return json.Marshal(*value.Value)
}

type encyclopediaImage struct {
	Path       string `json:"path"`
	Format     string `json:"format"`
	ByteLength uint64 `json:"byte_length"`
	Width      uint32 `json:"width"`
	Height     uint32 `json:"height"`
	SHA256     string `json:"sha256"`
	SourceRef  string `json:"source_ref"`
}

type encyclopediaCatalogBinding struct {
	Family  string `json:"family"`
	DatID   uint32 `json:"dat_id"`
	Variant string `json:"variant"`
	TopicID string `json:"topic_id"`
}

type encyclopediaCatalogResourceKey struct {
	SourceRole string
	LanguageID uint32
	ResourceID uint32
}

type encyclopediaCatalogImageInput struct {
	Path       string
	Format     string
	ByteLength uint64
	Width      uint32
	Height     uint32
	SHA256     string
	SourceRef  string
}

type encyclopediaCatalogMappings struct {
	Titles         map[encyclopediaCatalogResourceKey]string
	CategoryLabels map[uint32]map[uint32]string
	Images         map[string]encyclopediaCatalogImageInput
	SourceHashes   map[string]string
}

// encyclopediaPotentialMembershipOracle is supplied by a validated source
// profile. It is deliberately not inferred from a captured live cache and is
// not a universal rule embedded in the JSON validator.
type encyclopediaPotentialMembershipOracle struct {
	Index      []string
	Categories map[string][]string
}

func validateEncyclopediaPotentialMembership(catalog encyclopediaCatalog, oracle encyclopediaPotentialMembershipOracle) error {
	if !equalStringSlices(catalog.Index.TopicIDs, oracle.Index) {
		return newEncyclopediaValidationError("potential_membership_mismatch", "index potential membership differs from its source profile")
	}
	if len(oracle.Categories) != len(catalog.Categories) {
		return newEncyclopediaValidationError("filtered_membership_mismatch", "source profile has %d filtered categories, catalog has %d", len(oracle.Categories), len(catalog.Categories))
	}
	for _, category := range catalog.Categories {
		want, ok := oracle.Categories[category.Command]
		if !ok || !equalStringSlices(category.TopicIDs, want) {
			return newEncyclopediaValidationError("filtered_membership_mismatch", "category %s differs from its source profile", category.Command)
		}
	}
	return nil
}

func approvedEncyclopediaTopicSort() encyclopediaTopicSort {
	return encyclopediaTopicSort{
		Algorithm:             "stable_display_title_v1",
		RepresentableEncoding: "windows-1252-strict",
		RepresentableFold:     "ascii-lowercase-only",
		Unrepresentable:       "unicode-15.1.0-scalar-lowercase-utf8-after-representable",
		TieBreak:              "registry-order",
	}
}

func buildEncyclopediaCatalog(profile encyclopediaSourceProfile, texts encyclopediaDecodedCorpus, mappings encyclopediaCatalogMappings) (encyclopediaCatalog, error) {
	if err := validateEncyclopediaSourceProfile(profile); err != nil {
		return encyclopediaCatalog{}, fmt.Errorf("validate source profile: %w", err)
	}
	if !profile.Bindings.SchemaFit.ReadyForSchemaFreeze || len(profile.Bindings.SchemaFit.Blockers) != 0 {
		return encyclopediaCatalog{}, fmt.Errorf("source profile %q is not synchronized with the approved catalog contract", profile.ProfileID)
	}
	if texts.ProfileID != profile.ProfileID {
		return encyclopediaCatalog{}, fmt.Errorf("decoded text profile %q does not match %q", texts.ProfileID, profile.ProfileID)
	}
	for _, source := range profile.Sources {
		if source.Role != "display_strings" {
			continue
		}
		if mappings.SourceHashes[source.Role] != source.RawSHA256 {
			return encyclopediaCatalog{}, fmt.Errorf("mapping source %q does not match profile hash", source.Role)
		}
	}

	expectedTexts := make(map[encyclopediaCatalogResourceKey]encyclopediaBindingTextResource, len(profile.Bindings.ResourceAccounting.TextResources))
	for _, resource := range profile.Bindings.ResourceAccounting.TextResources {
		expectedTexts[encyclopediaCatalogResourceKey{SourceRole: resource.SourceRole, LanguageID: resource.LanguageID, ResourceID: resource.ResourceID}] = resource
	}
	if len(texts.Records) != len(expectedTexts) {
		return encyclopediaCatalog{}, fmt.Errorf("decoded corpus has %d records, profile accounts for %d", len(texts.Records), len(expectedTexts))
	}
	bodies := make(map[encyclopediaCatalogResourceKey]string, len(texts.Records))
	for _, record := range texts.Records {
		if record.ProfileID != profile.ProfileID || record.Encoding != profile.TextDecoder.Encoding || (record.Status != encyclopediaRecordDecoded && record.Status != encyclopediaRecordBound) {
			return encyclopediaCatalog{}, fmt.Errorf("decoded resource does not carry the selected profile/encoding/status")
		}
		if record.ResourceID.Kind != encyclopediaIdentifierNumeric || record.ResourceID.NumericID == nil {
			return encyclopediaCatalog{}, fmt.Errorf("decoded text contains a named resource")
		}
		roleKey := encyclopediaCatalogResourceKey{SourceRole: profile.TextDecoder.SourceRole, LanguageID: record.LanguageID, ResourceID: *record.ResourceID.NumericID}
		expected, ok := expectedTexts[roleKey]
		if !ok || record.RawLength != expected.RawLength || record.RawSHA256 != expected.RawSHA256 {
			return encyclopediaCatalog{}, fmt.Errorf("decoded resource %d/%d does not match profile accounting", record.LanguageID, *record.ResourceID.NumericID)
		}
		if _, duplicate := bodies[roleKey]; duplicate {
			return encyclopediaCatalog{}, fmt.Errorf("decoded corpus repeats resource %d/%d", record.LanguageID, *record.ResourceID.NumericID)
		}
		key := encyclopediaCatalogResourceKey{SourceRole: record.SourceBasename, LanguageID: record.LanguageID, ResourceID: *record.ResourceID.NumericID}
		// Decoded corpora normally carry the source basename while bindings carry
		// the profile role. Index both exact identities without changing bytes.
		bodies[key] = record.Text
		bodies[roleKey] = record.Text
	}
	for _, image := range profile.Bindings.ResourceAccounting.ArtFiles {
		if image.Status != encyclopediaBindingBound {
			continue
		}
		provided, ok := mappings.Images[image.Basename]
		if !ok || provided.ByteLength != image.RawLength || provided.SHA256 != image.RawSHA256 {
			return encyclopediaCatalog{}, fmt.Errorf("image facts for %s do not match profile accounting", image.Basename)
		}
	}

	recordByIdentity := make(map[uint32]encyclopediaBindingRecord, len(profile.Bindings.Records))
	for _, record := range profile.Bindings.Records {
		recordByIdentity[record.SourceIdentity] = record
	}
	catalog := encyclopediaCatalog{
		SchemaVersion:   encyclopediaCatalogSchemaVersion,
		DefaultLanguage: strconv.FormatUint(uint64(profile.Bindings.LanguageID), 10),
		TopicSort:       profile.Catalog.TopicSort,
		Topics:          make(map[string]encyclopediaTopic, len(profile.Bindings.Records)),
		Images:          make(map[string]encyclopediaImage),
		Bindings:        make([]encyclopediaCatalogBinding, 0, len(profile.Bindings.Records)),
	}

	for _, identity := range profile.Catalog.RegistryOrder {
		record, ok := recordByIdentity[identity]
		if !ok {
			return encyclopediaCatalog{}, fmt.Errorf("registry order refers to unknown source identity 0x%08x", identity)
		}
		bodyKey := encyclopediaCatalogResourceKey{SourceRole: record.Body.SourceRole, LanguageID: record.Body.LanguageID, ResourceID: record.Body.ResourceID}
		body, ok := bodies[bodyKey]
		if !ok {
			return encyclopediaCatalog{}, fmt.Errorf("missing decoded body %s/%d/%d", bodyKey.SourceRole, bodyKey.LanguageID, bodyKey.ResourceID)
		}
		titleKey := encyclopediaCatalogResourceKey{SourceRole: record.Title.SourceRole, LanguageID: record.Title.LanguageID, ResourceID: record.Title.SelectedResourceID}
		title, ok := mappings.Titles[titleKey]
		if !ok {
			return encyclopediaCatalog{}, fmt.Errorf("missing selected title %s/%d/%d", titleKey.SourceRole, titleKey.LanguageID, titleKey.ResourceID)
		}
		localized := encyclopediaLocalizedContent{Title: title, Body: body}
		switch record.Art.SelectorKind {
		case "canonical_topic_key", "system_picture":
			imageID, err := addCatalogImage(record.Art.AssetBasename, mappings.Images, catalog.Images)
			if err != nil {
				return encyclopediaCatalog{}, fmt.Errorf("topic resource %d: %w", record.Body.ResourceID, err)
			}
			localized.ImageID = &imageID
		case "viewer_faction_topic_key":
			if len(record.Art.ViewerFactionVariants) != 2 {
				return encyclopediaCatalog{}, fmt.Errorf("topic resource %d has incomplete faction art", record.Body.ResourceID)
			}
			selector := encyclopediaImageSelector{Kind: "viewer_faction"}
			for _, variant := range record.Art.ViewerFactionVariants {
				imageID, err := addCatalogImage(variant.AssetBasename, mappings.Images, catalog.Images)
				if err != nil {
					return encyclopediaCatalog{}, fmt.Errorf("topic resource %d faction %s: %w", record.Body.ResourceID, variant.ViewerFaction, err)
				}
				switch variant.ViewerFaction {
				case "alliance":
					selector.AllianceImageID = encyclopediaNullableImageID{Present: true, Value: &imageID}
				case "empire":
					selector.EmpireImageID = encyclopediaNullableImageID{Present: true, Value: &imageID}
				default:
					return encyclopediaCatalog{}, fmt.Errorf("topic resource %d has unknown viewer faction %q", record.Body.ResourceID, variant.ViewerFaction)
				}
			}
			localized.ImageSelector = &selector
		default:
			return encyclopediaCatalog{}, fmt.Errorf("topic resource %d has unsupported art selector %q", record.Body.ResourceID, record.Art.SelectorKind)
		}
		topicID := topicIDForResource(record.Body.ResourceID)
		if _, exists := catalog.Topics[topicID]; exists {
			return encyclopediaCatalog{}, fmt.Errorf("duplicate topic ID %q", topicID)
		}
		catalog.Topics[topicID] = encyclopediaTopic{
			Localized: map[string]encyclopediaLocalizedContent{strconv.FormatUint(uint64(record.Body.LanguageID), 10): localized},
			SourceRef: sourceRefForText(record.Body.LanguageID, record.Body.ResourceID),
		}
		catalog.Bindings = append(catalog.Bindings, encyclopediaCatalogBinding{
			Family: record.Family, DatID: record.DatID, Variant: record.Variant, TopicID: topicID,
		})
	}

	labelsByCommand := make(map[uint32]encyclopediaCatalogCategoryLabel, len(profile.Catalog.CategoryLabels))
	for _, label := range profile.Catalog.CategoryLabels {
		labelsByCommand[label.Command] = label
	}
	for index, category := range profile.Bindings.Categories {
		labelEvidence, ok := labelsByCommand[category.Command]
		if !ok {
			return encyclopediaCatalog{}, fmt.Errorf("category command 0x%x lacks label evidence", category.Command)
		}
		provided := mappings.CategoryLabels[category.Command]
		label, ok := provided[labelEvidence.LanguageID]
		if !ok {
			return encyclopediaCatalog{}, fmt.Errorf("category command 0x%x lacks label language %d", category.Command, labelEvidence.LanguageID)
		}
		if uint64(len([]byte(label))) != labelEvidence.UTF8Length || byteSHA256([]byte(label)) != labelEvidence.UTF8SHA256 {
			return encyclopediaCatalog{}, fmt.Errorf("category command 0x%x label does not match source evidence", category.Command)
		}
		view := encyclopediaCatalogView{
			Command:   commandString(category.Command),
			Labels:    map[string]string{strconv.FormatUint(uint64(labelEvidence.LanguageID), 10): label},
			TopicIDs:  topicIDsForCategory(profile.Catalog.RegistryOrder, recordByIdentity, category.Command, category.AggregateIndex),
			SourceRef: labelEvidence.SourceRef,
		}
		if index == 0 {
			catalog.Index = view
			continue
		}
		catalog.Categories = append(catalog.Categories, encyclopediaCatalogCategory{
			ID: "command:" + commandString(category.Command), Command: view.Command, Labels: view.Labels, TopicIDs: view.TopicIDs, SourceRef: view.SourceRef,
		})
	}

	if err := validateEncyclopediaCatalog(catalog); err != nil {
		return encyclopediaCatalog{}, fmt.Errorf("validate built catalog: %w", err)
	}
	return catalog, nil
}

func marshalEncyclopediaCatalog(catalog encyclopediaCatalog) ([]byte, error) {
	if err := validateEncyclopediaCatalog(catalog); err != nil {
		return nil, err
	}
	data, err := json.MarshalIndent(catalog, "", "  ")
	if err != nil {
		return nil, fmt.Errorf("marshal encyclopedia catalog: %w", err)
	}
	return append(data, '\n'), nil
}

func addCatalogImage(basename string, supplied map[string]encyclopediaCatalogImageInput, target map[string]encyclopediaImage) (string, error) {
	number, ok := parseEncyclopediaImageFilename(basename)
	if !ok {
		return "", fmt.Errorf("invalid EData basename %q", basename)
	}
	imageID := "edata:" + strconv.FormatUint(uint64(number), 10)
	input, ok := supplied[basename]
	if !ok {
		return "", fmt.Errorf("missing image facts for %s", basename)
	}
	wantPath := "assets/" + basename
	if input.Path != wantPath || input.Format != "bmp" || input.ByteLength == 0 || input.Width == 0 || input.Height == 0 || !validSHA256(input.SHA256) || input.SourceRef == "" {
		return "", fmt.Errorf("invalid image facts for %s", basename)
	}
	image := encyclopediaImage{Path: input.Path, Format: input.Format, ByteLength: input.ByteLength, Width: input.Width, Height: input.Height, SHA256: input.SHA256, SourceRef: input.SourceRef}
	if previous, exists := target[imageID]; exists && previous != image {
		return "", fmt.Errorf("image identity collision for %q", imageID)
	}
	target[imageID] = image
	return imageID, nil
}

func topicIDsForCategory(order []uint32, records map[uint32]encyclopediaBindingRecord, command uint32, aggregate bool) []string {
	result := make([]string, 0, len(order))
	for _, identity := range order {
		record := records[identity]
		if aggregate || record.CategoryCommand == command {
			result = append(result, topicIDForResource(record.Body.ResourceID))
		}
	}
	return result
}

func topicIDForResource(resourceID uint32) string {
	return "original:" + strconv.FormatUint(uint64(resourceID), 10)
}

func commandString(command uint32) string {
	return fmt.Sprintf("0x%x", command)
}

func sourceRefForText(languageID, resourceID uint32) string {
	return fmt.Sprintf("encytext/%d/%d", languageID, resourceID)
}

func validateEncyclopediaCatalogProfile(profile encyclopediaSourceProfile) error {
	contract := profile.Catalog
	if contract.TopicSort != approvedEncyclopediaTopicSort() {
		return fmt.Errorf("catalog topic-sort contract is not the approved v1 rule")
	}
	if contract.DefinitionCount < 0 || contract.DefinitionCount > len(contract.RegistryOrder) || len(contract.RegistryOrder) != len(profile.Bindings.Records) {
		return fmt.Errorf("catalog registry order has %d definitions / %d rows", contract.DefinitionCount, len(contract.RegistryOrder))
	}
	byIdentity := make(map[uint32]encyclopediaBindingRecord, len(profile.Bindings.Records))
	for _, record := range profile.Bindings.Records {
		byIdentity[record.SourceIdentity] = record
	}
	seen := make(map[uint32]struct{}, len(contract.RegistryOrder))
	for index, identity := range contract.RegistryOrder {
		record, ok := byIdentity[identity]
		if !ok {
			return fmt.Errorf("catalog registry order contains unknown identity 0x%08x", identity)
		}
		if _, duplicate := seen[identity]; duplicate {
			return fmt.Errorf("catalog registry order repeats identity 0x%08x", identity)
		}
		seen[identity] = struct{}{}
		isSystem := record.Family == "systems_world_locations"
		if (index < contract.DefinitionCount) == isSystem {
			return fmt.Errorf("catalog registry order definition/system partition is invalid at %d", index)
		}
		if index > contract.DefinitionCount && identity <= contract.RegistryOrder[index-1] {
			return fmt.Errorf("catalog system registry tail is not in ascending packed-key order at %d", index)
		}
	}
	if sha256Uint32Order(contract.RegistryOrder) != contract.RegistryOrderSHA256 {
		return fmt.Errorf("catalog registry order digest mismatch")
	}
	if sha256Uint32Order(contract.RegistryOrder[:contract.DefinitionCount]) != contract.DefinitionOrderSHA256 || sha256Uint32Order(contract.RegistryOrder[contract.DefinitionCount:]) != contract.SystemOrderSHA256 || !validSHA256(contract.SourceOrderOracleSHA256) {
		return fmt.Errorf("catalog definition/system/oracle evidence digest mismatch")
	}
	if len(contract.CategoryLabels) != len(profile.Bindings.Categories) {
		return fmt.Errorf("catalog category label evidence count = %d, want %d", len(contract.CategoryLabels), len(profile.Bindings.Categories))
	}
	for index, label := range contract.CategoryLabels {
		if label.Command != profile.Bindings.Categories[index].Command || label.LanguageID != profile.Bindings.LanguageID || label.SourceRole != "display_strings" || label.ResourceID == 0 || label.RawLength == 0 || !validSHA256(label.RawSHA256) || label.UTF8Length == 0 || !validSHA256(label.UTF8SHA256) || label.SourceRef == "" {
			return fmt.Errorf("catalog category label evidence %d is incomplete", index)
		}
	}
	return nil
}

func sha256Uint32Order(values []uint32) string {
	encoded, err := json.Marshal(values)
	if err != nil {
		panic(err)
	}
	sum := sha256.Sum256(append(encoded, '\n'))
	return hex.EncodeToString(sum[:])
}

func equalStringSlices(left, right []string) bool {
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
