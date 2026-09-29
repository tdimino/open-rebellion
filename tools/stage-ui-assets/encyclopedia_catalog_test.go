package main

import (
	"bytes"
	"fmt"
	"testing"
)

func TestBuildEncyclopediaCatalogEmitsApprovedPotentialMembershipDeterministically(t *testing.T) {
	profile := testEmbeddedCatalogProfile(t)
	texts := encyclopediaDecodedCorpus{ProfileID: profile.ProfileID}
	mappings := encyclopediaCatalogMappings{
		Titles:         make(map[encyclopediaCatalogResourceKey]string),
		CategoryLabels: make(map[uint32]map[uint32]string),
		Images:         make(map[string]encyclopediaCatalogImageInput),
		SourceHashes:   make(map[string]string),
	}
	for _, source := range profile.Sources {
		if source.Role == "display_strings" {
			mappings.SourceHashes[source.Role] = source.RawSHA256
		}
	}
	for _, resource := range profile.Bindings.ResourceAccounting.TextResources {
		resourceID := resource.ResourceID
		texts.Records = append(texts.Records, encyclopediaDecodedRecord{
			encyclopediaResearchRecord: encyclopediaResearchRecord{
				SourceBasename: "ENCYTEXT.DLL",
				ResourceID:     numericEncyclopediaResourceIdentifier(resourceID),
				LanguageID:     resource.LanguageID,
				RawLength:      resource.RawLength,
				RawSHA256:      resource.RawSHA256,
				Status:         encyclopediaRecordDecoded,
			},
			ProfileID: profile.ProfileID,
			Encoding:  "windows-1252",
			Text:      fmt.Sprintf("Synthetic body %d", resourceID),
		})
	}
	for _, record := range profile.Bindings.Records {
		mappings.Titles[encyclopediaCatalogResourceKey{
			SourceRole: record.Title.SourceRole,
			LanguageID: record.Title.LanguageID,
			ResourceID: record.Title.SelectedResourceID,
		}] = fmt.Sprintf("Synthetic title %08x", record.SourceIdentity)
	}
	for index := range profile.Catalog.CategoryLabels {
		label := &profile.Catalog.CategoryLabels[index]
		value := fmt.Sprintf("Synthetic label 0x%x", label.Command)
		label.UTF8Length = uint64(len([]byte(value)))
		label.UTF8SHA256 = byteSHA256([]byte(value))
		mappings.CategoryLabels[label.Command] = map[uint32]string{
			label.LanguageID: value,
		}
	}
	for _, image := range profile.Bindings.ResourceAccounting.ArtFiles {
		if image.Status != encyclopediaBindingBound {
			continue
		}
		mappings.Images[image.Basename] = encyclopediaCatalogImageInput{
			Path:       "assets/" + image.Basename,
			Format:     "bmp",
			ByteLength: image.RawLength,
			Width:      400,
			Height:     200,
			SHA256:     image.RawSHA256,
			SourceRef:  "edata/" + image.Basename,
		}
	}

	first, err := buildEncyclopediaCatalog(profile, texts, mappings)
	if err != nil {
		t.Fatalf("buildEncyclopediaCatalog() error = %v", err)
	}
	second, err := buildEncyclopediaCatalog(profile, texts, mappings)
	if err != nil {
		t.Fatalf("second buildEncyclopediaCatalog() error = %v", err)
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
		t.Fatal("identical catalog inputs did not produce byte-identical output")
	}
	if got, want := len(first.Index.TopicIDs), 347; got != want {
		t.Fatalf("index candidate count = %d, want %d", got, want)
	}
	if got, want := len(first.Categories), 6; got != want {
		t.Fatalf("filtered category count = %d, want %d", got, want)
	}
	if got, want := first.Index.TopicIDs[0], topicIDForResource(profile.Bindings.RecordsBySourceIdentityForTest(t, profile.Catalog.RegistryOrder[0]).Body.ResourceID); got != want {
		t.Fatalf("first index topic = %q, want registry first %q", got, want)
	}
	aggregateOnly := ""
	for _, record := range profile.Bindings.Records {
		if record.CategoryCommand == 0x6f {
			aggregateOnly = topicIDForResource(record.Body.ResourceID)
			break
		}
	}
	if aggregateOnly == "" {
		t.Fatal("profile lacks the source-proven aggregate-only binding")
	}
	if !containsCatalogTopicID(first.Index.TopicIDs, aggregateOnly) {
		t.Fatalf("aggregate index omits source-proven fleet topic %q", aggregateOnly)
	}
	for _, category := range first.Categories {
		if containsCatalogTopicID(category.TopicIDs, aggregateOnly) {
			t.Fatalf("aggregate-only fleet topic appears in filtered category %s", category.Command)
		}
	}
	if got, want := first.TopicSort, approvedEncyclopediaTopicSort(); got != want {
		t.Fatalf("topic sort = %#v, want %#v", got, want)
	}
	missingEvidence := texts
	missingEvidence.Records = missingEvidence.Records[:len(missingEvidence.Records)-1]
	if _, err := buildEncyclopediaCatalog(profile, missingEvidence, mappings); err == nil {
		t.Fatal("catalog build accepted an omitted source-accounting record")
	}
	for command, labels := range mappings.CategoryLabels {
		labels[profile.Bindings.LanguageID] += " changed"
		mappings.CategoryLabels[command] = labels
		break
	}
	if _, err := buildEncyclopediaCatalog(profile, texts, mappings); err == nil {
		t.Fatal("catalog build accepted a category label that contradicted source evidence")
	}
}

func TestEmbeddedEncyclopediaProfileCarriesApprovedCatalogContract(t *testing.T) {
	profile := testEmbeddedCatalogProfile(t)
	if !profile.Bindings.SchemaFit.ReadyForSchemaFreeze {
		t.Fatal("embedded profile remains blocked after approved E09-to-E10 synchronization")
	}
	if len(profile.Bindings.SchemaFit.Blockers) != 0 {
		t.Fatalf("embedded profile has stale schema blockers: %#v", profile.Bindings.SchemaFit.Blockers)
	}
	if got, want := len(profile.Catalog.RegistryOrder), 347; got != want {
		t.Fatalf("registry order count = %d, want %d", got, want)
	}
	if got, want := profile.Catalog.DefinitionCount, 147; got != want {
		t.Fatalf("definition registry prefix = %d, want %d", got, want)
	}
	if got, want := profile.Catalog.DefinitionOrderSHA256, "34374e74355f237621ce045e181768d5324754f17ea8e53eb8a0c816aeefd8a2"; got != want {
		t.Fatalf("definition order hash = %q, want reviewed %q", got, want)
	}
	if got, want := profile.Catalog.SystemOrderSHA256, "51fff06a0298a91b6b4036894b9c65cf3c91d5c5f6bb7d126f7b27421bb16f0f"; got != want {
		t.Fatalf("system order hash = %q, want reviewed %q", got, want)
	}
	if got, want := profile.Catalog.SourceOrderOracleSHA256, "a9da9373d2ca0a4e2dfc1053f44dae0e3e0193584f090029e35b7714c434175a"; got != want {
		t.Fatalf("source oracle hash = %q, want reviewed %q", got, want)
	}
	if got, want := len(profile.Catalog.CategoryLabels), 7; got != want {
		t.Fatalf("category label evidence count = %d, want %d", got, want)
	}
	if got, want := profile.Bindings.ResourceAccounting.Observations.TextRecordCount, 348; got != want {
		t.Fatalf("text accounting = %d, want %d", got, want)
	}
	unused := 0
	for _, record := range profile.Bindings.ResourceAccounting.TextResources {
		if record.Status == encyclopediaBindingSourceProvenUnused {
			unused++
			if record.ResourceID != 7176 || record.BindingCount != 0 {
				t.Fatalf("unexpected unused text accounting: %#v", record)
			}
		}
	}
	if unused != 1 {
		t.Fatalf("source-proven-unused text count = %d, want 1", unused)
	}
}

func TestEncyclopediaCatalogProfileRejectsOrderAndLabelEvidenceDrift(t *testing.T) {
	for _, test := range []struct {
		name string
		do   func(*encyclopediaSourceProfile)
	}{
		{name: "wrong sort constant", do: func(profile *encyclopediaSourceProfile) { profile.Catalog.TopicSort.TieBreak = "title-id" }},
		{name: "duplicate registry identity", do: func(profile *encyclopediaSourceProfile) {
			profile.Catalog.RegistryOrder[1] = profile.Catalog.RegistryOrder[0]
		}},
		{name: "system tail out of packed order", do: func(profile *encyclopediaSourceProfile) {
			index := profile.Catalog.DefinitionCount
			profile.Catalog.RegistryOrder[index], profile.Catalog.RegistryOrder[index+1] = profile.Catalog.RegistryOrder[index+1], profile.Catalog.RegistryOrder[index]
		}},
		{name: "missing category label", do: func(profile *encyclopediaSourceProfile) {
			profile.Catalog.CategoryLabels = profile.Catalog.CategoryLabels[:6]
		}},
	} {
		t.Run(test.name, func(t *testing.T) {
			profile := testEmbeddedCatalogProfile(t)
			test.do(&profile)
			if err := validateEncyclopediaCatalogProfile(profile); err == nil {
				t.Fatal("drifted catalog profile evidence was accepted")
			}
		})
	}
}

func testEmbeddedCatalogProfile(t *testing.T) encyclopediaSourceProfile {
	t.Helper()
	profiles, err := loadEmbeddedEncyclopediaProfiles()
	if err != nil {
		t.Fatal(err)
	}
	if len(profiles) != 1 {
		t.Fatalf("embedded profiles = %d, want 1", len(profiles))
	}
	return profiles[0]
}

func (bindings encyclopediaProfileBindings) RecordsBySourceIdentityForTest(t *testing.T, identity uint32) encyclopediaBindingRecord {
	t.Helper()
	for _, record := range bindings.Records {
		if record.SourceIdentity == identity {
			return record
		}
	}
	t.Fatalf("missing binding source identity 0x%08x", identity)
	return encyclopediaBindingRecord{}
}

func containsCatalogTopicID(values []string, wanted string) bool {
	for _, value := range values {
		if value == wanted {
			return true
		}
	}
	return false
}
