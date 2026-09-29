package main

import (
	"bytes"
	"encoding/binary"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"testing"
	"time"
)

func TestEmbeddedEncyclopediaProfileHasCombinedBindingClosure(t *testing.T) {
	profiles, err := loadEmbeddedEncyclopediaProfiles()
	if err != nil {
		t.Fatalf("loadEmbeddedEncyclopediaProfiles() error = %v", err)
	}
	if got, want := len(profiles), 1; got != want {
		t.Fatalf("embedded profile count = %d, want %d", got, want)
	}

	bindings := profiles[0].Bindings
	if err := validateEncyclopediaProfileBindings(profiles[0], bindings); err != nil {
		t.Fatalf("validateEncyclopediaProfileBindings() error = %v", err)
	}
	if got, want := len(bindings.Records), 347; got != want {
		t.Fatalf("combined binding count = %d, want observed %d", got, want)
	}
	if got, want := len(bindings.ResourceAccounting.TextResources), 348; got != want {
		t.Fatalf("text accounting count = %d, want observed %d", got, want)
	}
	if got, want := len(bindings.ResourceAccounting.ArtLookups), 191; got != want {
		t.Fatalf("art lookup accounting count = %d, want observed %d", got, want)
	}
	if got, want := len(bindings.ResourceAccounting.ArtFiles), 187; got != want {
		t.Fatalf("art file accounting count = %d, want observed %d", got, want)
	}
	if !bindings.SchemaFit.ReadyForSchemaFreeze || len(bindings.SchemaFit.Blockers) != 0 {
		t.Fatalf("approved profile readiness = %v with blockers %#v", bindings.SchemaFit.ReadyForSchemaFreeze, bindings.SchemaFit.Blockers)
	}
	wantFamilies := map[string]int{
		"fleet_definitions":        1,
		"capital_ship_classes":     30,
		"fighter_classes":          8,
		"troop_classes":            10,
		"special_force_classes":    9,
		"major_characters":         6,
		"minor_characters":         54,
		"systems_world_locations":  200,
		"defense_facilities":       6,
		"manufacturing_facilities": 6,
		"production_facilities":    2,
		"mission_definitions":      15,
	}
	gotFamilies := make(map[string]int)
	for _, record := range bindings.Records {
		gotFamilies[record.Family]++
	}
	if got, want := len(gotFamilies), len(wantFamilies); got != want {
		t.Fatalf("combined family count = %d, want observed %d", got, want)
	}
	for family, want := range wantFamilies {
		if got := gotFamilies[family]; got != want {
			t.Fatalf("family %s count = %d, want observed %d", family, got, want)
		}
	}
	deferred192 := false
	for _, file := range bindings.ResourceAccounting.ArtFiles {
		if file.Basename != "EDATA.192" {
			continue
		}
		deferred192 = file.Status == encyclopediaBindingPublicationDeferred && file.BindingCount == 0 && file.DeferredTask == "orlocal-2kq"
	}
	for _, record := range bindings.Records {
		if record.Art.AssetBasename == "EDATA.192" {
			t.Fatal("deferred EDATA.192 was published as a binding")
		}
		for _, variant := range record.Art.ViewerFactionVariants {
			if variant.AssetBasename == "EDATA.192" {
				t.Fatal("deferred EDATA.192 was published as a faction binding")
			}
		}
	}
	for _, lookup := range bindings.ResourceAccounting.ArtLookups {
		if lookup.AssetBasename == "EDATA.192" {
			t.Fatal("deferred EDATA.192 was assigned an invented lookup")
		}
	}
	if !deferred192 {
		t.Fatal("EDATA.192 is not explicit publication-deferred inventory under orlocal-2kq")
	}
}

func TestCombinedEncyclopediaProfileClosesResidualSourceRowsAndFactionSelectors(t *testing.T) {
	profile := testCombinedEncyclopediaProfile(t)
	bindings := profile.Bindings
	residualText := make(map[uint32]struct{})
	residualLookups := make(map[uint32]struct{})
	residualFiles := make(map[string]struct{})

	if got, want := bindings.ResourceAccounting.Observations.SourceRowCount, 358; got != want {
		t.Fatalf("source rows = %d, want observed %d", got, want)
	}
	if got, want := bindings.ResourceAccounting.Observations.SourceProvenUnusedRowCount, 11; got != want {
		t.Fatalf("source-proven unused rows = %d, want observed %d", got, want)
	}
	if got, want := len(bindings.SourceCoverage), 2; got != want {
		t.Fatalf("residual source coverage tables = %d, want %d", got, want)
	}

	category := bindings.Categories[4]
	if category.Command != 0x73 || category.Status != "complete" || category.BindingCount != 15 || len(category.SourceFamilies) != 1 || category.SourceFamilies[0] != "mission_definitions" {
		t.Fatalf("command 0x73 category was not closed by the 15 admitted mission rows: %+v", category)
	}
	if bindings.Categories[0].Status != "complete" || bindings.Categories[0].BindingCount != 347 {
		t.Fatalf("aggregate category did not close over every bound topic: %+v", bindings.Categories[0])
	}

	var mission, fleet *encyclopediaBindingRecord
	for index := range bindings.Records {
		record := &bindings.Records[index]
		if record.SourceRole == "mission_definitions" || record.SourceRole == "fleet_definitions" {
			residualText[record.Body.ResourceID] = struct{}{}
			for _, variant := range record.Art.ViewerFactionVariants {
				residualLookups[variant.LookupID] = struct{}{}
				residualFiles[variant.AssetBasename] = struct{}{}
			}
		}
		switch record.Body.ResourceID {
		case 7184:
			mission = record
		case 7427:
			fleet = record
		}
	}
	for label, record := range map[string]*encyclopediaBindingRecord{"mission": mission, "fleet": fleet} {
		if record == nil {
			t.Fatalf("missing recovered %s residual binding", label)
		}
		if record.Art.SelectorKind != "viewer_faction_topic_key" || len(record.Art.ViewerFactionVariants) != 2 {
			t.Fatalf("%s art selector = %+v, want two source-proven faction variants", label, record.Art)
		}
		if got, want := record.Art.ViewerFactionVariants[0].LookupID, record.Body.ResourceID; got != want {
			t.Fatalf("%s Alliance lookup = %d, want body key %d", label, got, want)
		}
		if got, want := record.Art.ViewerFactionVariants[1].LookupID, record.Body.ResourceID+0x1000; got != want {
			t.Fatalf("%s Empire lookup = %d, want body key + 0x1000 (%d)", label, got, want)
		}
	}
	if fleet.CategoryCommand != 0x6f {
		t.Fatalf("fleet category command = %#x, want aggregate-only command 0x6f", fleet.CategoryCommand)
	}

	assertAccountingStatus := func(kind string, id uint32, got string) {
		t.Helper()
		if got != encyclopediaBindingSourceProvenUnused {
			t.Fatalf("%s %d status = %q, want %q", kind, id, got, encyclopediaBindingSourceProvenUnused)
		}
	}
	for _, resource := range bindings.ResourceAccounting.TextResources {
		if resource.ResourceID == 7176 {
			assertAccountingStatus("text", resource.ResourceID, resource.Status)
			residualText[resource.ResourceID] = struct{}{}
		}
	}
	for _, lookup := range bindings.ResourceAccounting.ArtLookups {
		if lookup.LogicalID == 7188 || lookup.LogicalID == 11284 {
			assertAccountingStatus("lookup", lookup.LogicalID, lookup.Status)
			residualLookups[lookup.LogicalID] = struct{}{}
		}
	}
	assertUint32Set := func(label string, got map[uint32]struct{}, want []uint32) {
		t.Helper()
		if len(got) != len(want) {
			t.Fatalf("%s identity count = %d, want %d", label, len(got), len(want))
		}
		for _, id := range want {
			if _, exists := got[id]; !exists {
				t.Fatalf("%s identity %d is not accounted", label, id)
			}
		}
	}
	assertUint32Set("residual text", residualText, []uint32{
		7176, 7184, 7185, 7186, 7187, 7189, 7190, 7191, 7200,
		7201, 7202, 7232, 7233, 7234, 7296, 7297, 7427,
	})
	assertUint32Set("residual lookup", residualLookups, []uint32{
		7184, 7185, 7186, 7187, 7188, 7189, 7190, 7191, 7200,
		7201, 7202, 7232, 7233, 7234, 7296, 7297, 7427, 11280,
		11281, 11282, 11283, 11284, 11285, 11286, 11287, 11296,
		11297, 11298, 11328, 11329, 11330, 11392, 11393, 11523,
	})
	wantFiles := []string{
		"EDATA.132", "EDATA.133", "EDATA.134", "EDATA.135", "EDATA.136",
		"EDATA.137", "EDATA.138", "EDATA.139", "EDATA.140", "EDATA.141",
		"EDATA.142", "EDATA.143", "EDATA.146", "EDATA.148", "EDATA.149",
		"EDATA.150", "EDATA.151", "EDATA.152", "EDATA.153", "EDATA.154",
		"EDATA.155", "EDATA.156", "EDATA.157", "EDATA.158", "EDATA.160",
		"EDATA.161", "EDATA.162", "EDATA.163", "EDATA.164",
	}
	if len(residualFiles) != len(wantFiles) {
		t.Fatalf("residual art-file count = %d, want %d", len(residualFiles), len(wantFiles))
	}
	for _, basename := range wantFiles {
		if _, exists := residualFiles[basename]; !exists {
			t.Fatalf("residual art file %s is not accounted by a bound selector", basename)
		}
	}
}

func TestCombinedEncyclopediaProfilePreservesAcceptedFamilyRecordsWhileAddingResiduals(t *testing.T) {
	profile := testCombinedEncyclopediaProfile(t)
	accepted := make([]encyclopediaBindingRecord, 0, 331)
	for _, record := range profile.Bindings.Records {
		if record.SourceRole == "mission_definitions" || record.SourceRole == "fleet_definitions" {
			continue
		}
		accepted = append(accepted, record)
	}
	if got, want := len(accepted), 331; got != want {
		t.Fatalf("preserved accepted records = %d, want %d", got, want)
	}
	encoded, err := json.Marshal(accepted)
	if err != nil {
		t.Fatal(err)
	}
	if got, want := byteSHA256(encoded), "e41c9795a8b0a55d5360cfcc3c729af3da897891bc5fbcb9e6f32e944049bff0"; got != want {
		t.Fatalf("preserved E40 record-set digest = %s, want %s", got, want)
	}
}

func TestCombinedEncyclopediaProfileRejectsResidualCoverageAndFactionContradictions(t *testing.T) {
	tests := []struct {
		name string
		do   func(*encyclopediaSourceProfile)
		want string
	}{
		{
			name: "missing excluded source row",
			do: func(profile *encyclopediaSourceProfile) {
				coverage := &profile.Bindings.SourceCoverage[0]
				coverage.ExcludedRows = coverage.ExcludedRows[1:]
				profile.Bindings.ResourceAccounting.Observations.SourceRowCount--
				profile.Bindings.ResourceAccounting.Observations.SourceProvenUnusedRowCount--
			},
			want: "source-row coverage",
		},
		{
			name: "excluded row duplicates a bound join",
			do: func(profile *encyclopediaSourceProfile) {
				coverage := &profile.Bindings.SourceCoverage[0]
				coverage.ExcludedRows[4].SourceRow = 4
			},
			want: "classifies row 4 more than once",
		},
		{
			name: "duplicate faction selector",
			do: func(profile *encyclopediaSourceProfile) {
				for index := range profile.Bindings.Records {
					record := &profile.Bindings.Records[index]
					if len(record.Art.ViewerFactionVariants) == 2 {
						record.Art.ViewerFactionVariants[1].ViewerFaction = record.Art.ViewerFactionVariants[0].ViewerFaction
						return
					}
				}
			},
			want: "faction variants",
		},
		{
			name: "wrong Empire selector",
			do: func(profile *encyclopediaSourceProfile) {
				for index := range profile.Bindings.Records {
					record := &profile.Bindings.Records[index]
					if len(record.Art.ViewerFactionVariants) == 2 {
						record.Art.ViewerFactionVariants[1].LookupID++
						return
					}
				}
			},
			want: "empire lookup",
		},
		{
			name: "source-proven unused resource gains a binding",
			do: func(profile *encyclopediaSourceProfile) {
				for index := range profile.Bindings.ResourceAccounting.ArtLookups {
					lookup := &profile.Bindings.ResourceAccounting.ArtLookups[index]
					if lookup.Status == encyclopediaBindingSourceProvenUnused {
						lookup.BindingCount = 1
						return
					}
				}
			},
			want: "source-proven-unused resource",
		},
	}
	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			profile := testCombinedEncyclopediaProfile(t)
			test.do(&profile)
			err := validateEncyclopediaSourceProfile(profile)
			if err == nil || !strings.Contains(err.Error(), test.want) {
				t.Fatalf("validation error = %v, want substring %q", err, test.want)
			}
		})
	}
}

func TestCombinedEncyclopediaProfileValidatesEverySharedFactionArtSelection(t *testing.T) {
	base := testCombinedEncyclopediaProfile(t)
	type sharedFixture struct {
		SourceIdentity uint32
		Basename       string
	}
	shared := make([]sharedFixture, 0, 3)
	for _, record := range base.Bindings.Records {
		variants := record.Art.ViewerFactionVariants
		if len(variants) != 2 || variants[0].AssetBasename != variants[1].AssetBasename {
			continue
		}
		shared = append(shared, sharedFixture{
			SourceIdentity: record.SourceIdentity,
			Basename:       variants[0].AssetBasename,
		})
		file := findBindingArtFile(t, base.Bindings.ResourceAccounting.ArtFiles, variants[0].AssetBasename)
		if file.BindingCount != 2 {
			t.Fatalf("valid shared file %s binding count = %d, want 2 selections", file.Basename, file.BindingCount)
		}
	}
	if got, want := len(shared), 3; got != want {
		t.Fatalf("shared-faction fixtures = %d, want observed %d", got, want)
	}

	mutations := []struct {
		name string
		do   func(*encyclopediaBindingArtFactionVariant)
	}{
		{
			name: "hash",
			do: func(variant *encyclopediaBindingArtFactionVariant) {
				variant.RawSHA256 = strings.Repeat("0", 64)
			},
		},
		{
			name: "byte length",
			do: func(variant *encyclopediaBindingArtFactionVariant) {
				variant.RawLength++
			},
		},
	}
	for _, fixture := range shared {
		for side := 0; side < 2; side++ {
			for _, mutation := range mutations {
				name := fmt.Sprintf("%s/%s/%s", fixture.Basename, []string{"alliance", "empire"}[side], mutation.name)
				t.Run(name, func(t *testing.T) {
					profile := testCombinedEncyclopediaProfile(t)
					for index := range profile.Bindings.Records {
						record := &profile.Bindings.Records[index]
						if record.SourceIdentity != fixture.SourceIdentity {
							continue
						}
						mutation.do(&record.Art.ViewerFactionVariants[side])
						err := validateEncyclopediaSourceProfile(profile)
						if err == nil || !strings.Contains(err.Error(), "contradicts accounted file evidence") {
							t.Fatalf("validation error = %v, want per-selection file evidence contradiction", err)
						}
						return
					}
					t.Fatalf("missing shared-faction record 0x%08x", fixture.SourceIdentity)
				})
			}
		}
	}
}

func findBindingArtFile(t *testing.T, files []encyclopediaBindingArtFile, basename string) encyclopediaBindingArtFile {
	t.Helper()
	for _, file := range files {
		if file.Basename == basename {
			return file
		}
	}
	t.Fatalf("missing art-file accounting for %s", basename)
	return encyclopediaBindingArtFile{}
}

func TestCombinedEncyclopediaProfileRequiresNamedBlockerOnlyWhileAggregateShapeIsUnresolved(t *testing.T) {
	profile := testCombinedEncyclopediaProfile(t)
	foundAggregateOnly := false
	for _, record := range profile.Bindings.Records {
		if record.CategoryCommand == 0x6f {
			foundAggregateOnly = true
			break
		}
	}
	if !foundAggregateOnly {
		t.Fatal("profile lacks the source-proven aggregate-only topic fixture")
	}
	profile.Bindings.SchemaFit.ReadyForSchemaFreeze = false
	profile.Bindings.SchemaFit.Blockers = []encyclopediaBindingIssue{{
		ID: "synthetic-other-gate", Reason: "a synthetic unrelated gate remains", NextProof: "close the synthetic unrelated gate",
	}}

	err := validateEncyclopediaSourceProfile(profile)
	if err == nil || !strings.Contains(err.Error(), "aggregate-only-topic-membership") {
		t.Fatalf("validation error = %v, want aggregate-only blocker while the wire shape is unresolved", err)
	}
}

func TestCombinedEncyclopediaProfileAcceptsApprovedAggregateOnlyMembershipShape(t *testing.T) {
	profile := testCombinedEncyclopediaProfile(t)
	if !profile.Bindings.SchemaFit.ReadyForSchemaFreeze || len(profile.Bindings.SchemaFit.Blockers) != 0 {
		t.Fatalf("profile readiness = %v, blockers = %#v", profile.Bindings.SchemaFit.ReadyForSchemaFreeze, profile.Bindings.SchemaFit.Blockers)
	}
	if err := validateEncyclopediaSourceProfile(profile); err != nil {
		t.Fatalf("approved aggregate-only membership shape was rejected: %v", err)
	}
}

func TestCombinedEncyclopediaProfileRejectsReadinessWithUnresolvedEvidence(t *testing.T) {
	profile := testCombinedEncyclopediaProfile(t)
	profile.Bindings.Categories[4].Status = encyclopediaBindingUnresolved
	profile.Bindings.Categories[4].Reason = "synthetic unresolved category evidence"
	profile.Bindings.Categories[4].NextProof = "supply connected source evidence"
	profile.Bindings.SchemaFit.ReadyForSchemaFreeze = true
	profile.Bindings.SchemaFit.Blockers = nil

	err := validateEncyclopediaSourceProfile(profile)
	if err == nil || !strings.Contains(err.Error(), "schema-freeze readiness contradicts unresolved evidence") {
		t.Fatalf("validation error = %v, want unresolved-evidence readiness contradiction", err)
	}
}

func TestCombinedEncyclopediaProfileAcceptsReadinessWithOnlyDeferredPublicationEvidence(t *testing.T) {
	bindings := encyclopediaProfileBindings{
		ResourceAccounting: encyclopediaBindingResourceAccounting{
			ArtFiles: []encyclopediaBindingArtFile{{
				Basename:     "EDATA.192",
				Status:       encyclopediaBindingPublicationDeferred,
				DeferredTask: "orlocal-2kq",
			}},
		},
		SchemaFit: encyclopediaBindingSchemaFit{
			OneCategoryPerBoundTopic: "every synthetic topic has one category",
			FullIndexSelector:        "the full index is not topic membership",
			AliasIdentity:            "aliases are explicit",
			CharacterDiscriminator:   "character identities are unambiguous",
			ClassInstanceBoundary:    "instances resolve source classes",
			ReadyForSchemaFreeze:     true,
		},
	}
	if err := validateEncyclopediaBindingSchemaFit(bindings); err != nil {
		t.Fatalf("fully reconciled synthetic profile with explicit deferred publication evidence was rejected: %v", err)
	}
}

func TestCombinedEncyclopediaProfileRejectsOmittedSourceProvenUnusedDecoderCorpusResource(t *testing.T) {
	profile := testCombinedEncyclopediaProfile(t)
	for index, resource := range profile.Bindings.ResourceAccounting.TextResources {
		if resource.Status != encyclopediaBindingSourceProvenUnused {
			continue
		}
		profile.Bindings.ResourceAccounting.TextResources = append(
			profile.Bindings.ResourceAccounting.TextResources[:index],
			profile.Bindings.ResourceAccounting.TextResources[index+1:]...,
		)
		profile.Bindings.ResourceAccounting.Observations.TextRecordCount--

		err := validateEncyclopediaSourceProfile(profile)
		if err == nil || !strings.Contains(err.Error(), "decoder corpus record count") {
			t.Fatalf("validation error = %v, want decoder-corpus accounting mismatch", err)
		}
		return
	}
	t.Fatal("embedded profile has no source-proven-unused text resource fixture")
}

func TestCombinedEncyclopediaProfileRejectsAmbiguousOrIncompleteEvidence(t *testing.T) {
	tests := []struct {
		name string
		do   func(*encyclopediaSourceProfile)
		want string
	}{
		{
			name: "duplicate binding tuple",
			do: func(profile *encyclopediaSourceProfile) {
				profile.Bindings.Records[1].SourceFamily = profile.Bindings.Records[0].SourceFamily
				profile.Bindings.Records[1].DatID = profile.Bindings.Records[0].DatID
				profile.Bindings.Records[1].SourceIdentity = profile.Bindings.Records[0].SourceIdentity
				profile.Bindings.Records[1].Variant = profile.Bindings.Records[0].Variant
			},
			want: "duplicate binding tuple",
		},
		{
			name: "source identity collision across variants",
			do: func(profile *encyclopediaSourceProfile) {
				profile.Bindings.Records[1].SourceFamily = profile.Bindings.Records[0].SourceFamily
				profile.Bindings.Records[1].DatID = profile.Bindings.Records[0].DatID
				profile.Bindings.Records[1].SourceIdentity = profile.Bindings.Records[0].SourceIdentity
				profile.Bindings.Records[1].Variant = "synthetic-other-variant"
			},
			want: "ambiguous source identity",
		},
		{
			name: "missing source reference",
			do: func(profile *encyclopediaSourceProfile) {
				profile.Bindings.Records[0].SourceRole = "missing_table"
			},
			want: "missing/non-DAT source role",
		},
		{
			name: "missing original category",
			do: func(profile *encyclopediaSourceProfile) {
				profile.Bindings.Categories = append(profile.Bindings.Categories[:4], profile.Bindings.Categories[5:]...)
			},
			want: "category count",
		},
		{
			name: "unaccounted body resource",
			do: func(profile *encyclopediaSourceProfile) {
				profile.Bindings.ResourceAccounting.TextResources = profile.Bindings.ResourceAccounting.TextResources[1:]
				profile.Bindings.ResourceAccounting.Observations.TextRecordCount--
			},
			want: "unaccounted",
		},
		{
			name: "dangling art lookup file",
			do: func(profile *encyclopediaSourceProfile) {
				for index := range profile.Bindings.ResourceAccounting.ArtLookups {
					lookup := &profile.Bindings.ResourceAccounting.ArtLookups[index]
					if lookup.LogicalID == 7184 {
						lookup.AssetBasename = "EDATA.999"
						return
					}
				}
			},
			want: "contradicts bound record evidence",
		},
		{
			name: "deferred image silently made unresolved",
			do: func(profile *encyclopediaSourceProfile) {
				for index := range profile.Bindings.ResourceAccounting.ArtFiles {
					file := &profile.Bindings.ResourceAccounting.ArtFiles[index]
					if file.Basename == "EDATA.192" {
						file.Status = encyclopediaBindingUnresolved
						return
					}
				}
			},
			want: "EDATA.192",
		},
		{
			name: "profile DLL source hash conflict",
			do: func(profile *encyclopediaSourceProfile) {
				conflict := profile.Sources[0]
				conflict.Role = "conflicting_copy"
				conflict.RawSHA256 = strings.Repeat("0", 64)
				profile.Sources = append(profile.Sources, conflict)
			},
			want: "conflicting identities",
		},
		{
			name: "profile DAT source hash conflict",
			do: func(profile *encyclopediaSourceProfile) {
				for _, source := range profile.Sources {
					if source.Kind != encyclopediaSourceDAT {
						continue
					}
					conflict := source
					conflict.Role = "conflicting_dat_copy"
					conflict.RawSHA256 = strings.Repeat("f", 64)
					profile.Sources = append(profile.Sources, conflict)
					return
				}
			},
			want: "conflicting identities",
		},
		{
			name: "category omits a bound family",
			do: func(profile *encyclopediaSourceProfile) {
				profile.Bindings.Categories[2].SourceFamilies = profile.Bindings.Categories[2].SourceFamilies[:1]
			},
			want: "source-family coverage",
		},
		{
			name: "binding DAT becomes decoding prerequisite",
			do: func(profile *encyclopediaSourceProfile) {
				for index := range profile.Sources {
					if profile.Sources[index].Kind == encyclopediaSourceDAT {
						profile.Sources[index].RequiredForDecoding = true
						return
					}
				}
			},
			want: "must not be required for flattened text decoding",
		},
		{
			name: "noncanonical record order",
			do: func(profile *encyclopediaSourceProfile) {
				profile.Bindings.Records[0], profile.Bindings.Records[1] = profile.Bindings.Records[1], profile.Bindings.Records[0]
			},
			want: "canonically ordered",
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			profile := testCombinedEncyclopediaProfile(t)
			test.do(&profile)
			err := validateEncyclopediaSourceProfile(profile)
			if err == nil || !strings.Contains(err.Error(), test.want) {
				t.Fatalf("validation error = %v, want substring %q", err, test.want)
			}
		})
	}
}

func TestCombinedEncyclopediaProfileRepresentsExplicitAliasesAndSharedArt(t *testing.T) {
	profile := testCombinedEncyclopediaProfile(t)
	canonicalIndex, aliasIndex := -1, -1
	for index := range profile.Bindings.Records {
		if profile.Bindings.Records[index].Art.SelectorKind != "canonical_topic_key" {
			continue
		}
		if canonicalIndex == -1 {
			canonicalIndex = index
		} else {
			aliasIndex = index
			break
		}
	}
	if canonicalIndex == -1 || aliasIndex == -1 {
		t.Fatal("profile lacks two canonical-topic records for alias fixture")
	}
	canonical := profile.Bindings.Records[canonicalIndex]
	alias := &profile.Bindings.Records[aliasIndex]
	oldBodyID := alias.Body.ResourceID
	oldLookupID := alias.Art.LookupID
	oldArtBasename := alias.Art.AssetBasename
	alias.Title = canonical.Title
	alias.Body = canonical.Body
	alias.Art = canonical.Art
	alias.Status = encyclopediaBindingDocumentedAlias
	alias.AliasOf = uint32Pointer(canonical.SourceIdentity)
	profile.Bindings.ResourceAccounting.Observations.BoundRowCount--
	profile.Bindings.ResourceAccounting.Observations.DocumentedAliasRowCount++
	for index := range profile.Bindings.ResourceAccounting.TextResources {
		resource := &profile.Bindings.ResourceAccounting.TextResources[index]
		switch resource.ResourceID {
		case canonical.Body.ResourceID:
			resource.BindingCount++
		case oldBodyID:
			resource.Status = encyclopediaBindingUnresolved
			resource.BindingCount = 0
			resource.Reason = "synthetic alias leaves this inventoried resource unresolved"
			resource.NextProof = "supply a connected source selector"
		}
	}
	for index := range profile.Bindings.ResourceAccounting.ArtLookups {
		lookup := &profile.Bindings.ResourceAccounting.ArtLookups[index]
		switch lookup.LogicalID {
		case canonical.Art.LookupID:
			lookup.BindingCount++
		case oldLookupID:
			lookup.Status = encyclopediaBindingUnresolved
			lookup.BindingCount = 0
			lookup.Reason = "synthetic alias leaves this lookup unresolved"
			lookup.NextProof = "supply a connected source selector"
		}
	}
	for index := range profile.Bindings.ResourceAccounting.ArtFiles {
		file := &profile.Bindings.ResourceAccounting.ArtFiles[index]
		switch file.Basename {
		case canonical.Art.AssetBasename:
			file.BindingCount++
		case oldArtBasename:
			file.Status = encyclopediaBindingUnresolved
			file.BindingCount = 0
			file.Reason = "synthetic alias leaves this art file unresolved"
			file.NextProof = "supply a connected source selector"
		}
	}
	// This synthetic historical-inventory shape deliberately leaves evidence
	// unresolved, so it must not retain the approved profile's ready status.
	profile.Bindings.SchemaFit.ReadyForSchemaFreeze = false
	profile.Bindings.SchemaFit.Blockers = []encyclopediaBindingIssue{
		{
			ID:        encyclopediaBindingAggregateOnlyBlocker,
			Reason:    "the synthetic research shape has not adopted the approved aggregate-only wire contract",
			NextProof: "adopt the approved aggregate-only membership representation",
		},
		{
			ID:        "synthetic-alias-evidence",
			Reason:    "the synthetic alias leaves text, lookup, and art evidence unresolved",
			NextProof: "supply connected source selectors for the displaced evidence",
		},
	}
	if err := validateEncyclopediaSourceProfile(profile); err != nil {
		t.Fatalf("explicit alias shape was rejected: %v", err)
	}

	shared := false
	for _, lookup := range profile.Bindings.ResourceAccounting.ArtLookups {
		if lookup.BindingCount > 1 {
			shared = true
			break
		}
	}
	if !shared {
		t.Fatal("identified profile did not preserve source-proven shared system art")
	}
}

func TestCombinedEncyclopediaProfilePreservesPreferredTitleBranches(t *testing.T) {
	profile := testCombinedEncyclopediaProfile(t)
	record := &profile.Bindings.Records[0]
	record.Title.PreferredStatus = "nonempty"
	record.Title.SelectedResourceID = record.Title.PreferredResourceID
	if err := validateEncyclopediaSourceProfile(profile); err != nil {
		t.Fatalf("valid nonempty preferred-title branch was rejected: %v", err)
	}
	record.Title.SelectedResourceID = record.Title.OriginalResourceID
	if err := validateEncyclopediaSourceProfile(profile); err == nil || !strings.Contains(err.Error(), "nonempty preferred title") {
		t.Fatalf("incorrect preferred-title selection error = %v", err)
	}
}

func TestCombinedEncyclopediaProfileFragmentHashesMatchReviewedInputs(t *testing.T) {
	profile := testCombinedEncyclopediaProfile(t)
	for _, input := range profile.Bindings.FragmentInputs {
		data, err := os.ReadFile(input.Path)
		if err != nil {
			t.Fatalf("read fragment %s: %v", input.Path, err)
		}
		if got := byteSHA256(data); got != input.RawSHA256 {
			t.Fatalf("fragment %s SHA-256 = %s, want %s", input.Path, got, input.RawSHA256)
		}
	}
}

func TestCombinedEncyclopediaProfileSourceHashesMatchReviewedFragments(t *testing.T) {
	profile := testCombinedEncyclopediaProfile(t)
	type sourceIdentity struct {
		Basename  string `json:"basename"`
		RawLength uint64 `json:"raw_length"`
		RawSHA256 string `json:"raw_sha256"`
	}
	type fragmentSources struct {
		Sources     []sourceIdentity `json:"sources"`
		SourceFiles []sourceIdentity `json:"source_files"`
	}
	want := make(map[string]sourceIdentity)
	for _, path := range []string{
		"encyclopedia_profiles/fragments/unit-bindings.json",
		"encyclopedia_profiles/fragments/character-bindings.json",
		"encyclopedia_profiles/fragments/static-bindings.json",
		"encyclopedia_profiles/fragments/encyclopedia-residual-bindings.json",
	} {
		var fragment fragmentSources
		readBindingFragment(t, path, &fragment)
		for _, source := range append(fragment.Sources, fragment.SourceFiles...) {
			key := strings.ToLower(source.Basename)
			if previous, exists := want[key]; exists && (previous.RawLength != source.RawLength || previous.RawSHA256 != source.RawSHA256) {
				t.Fatalf("reviewed fragments conflict for source %s", source.Basename)
			}
			want[key] = source
		}
	}
	if got, expected := len(profile.Sources), len(want); got != expected {
		t.Fatalf("profile source count = %d, reviewed fragment identities = %d", got, expected)
	}
	for _, source := range profile.Sources {
		expected, exists := want[strings.ToLower(source.Basename)]
		if !exists || source.RawLength != expected.RawLength || source.RawSHA256 != expected.RawSHA256 {
			t.Fatalf("profile source %s is missing from or contradicts reviewed fragments", source.Basename)
		}
	}
}

func TestCombinedEncyclopediaProfileClosesEveryFragmentIdentity(t *testing.T) {
	profile := testCombinedEncyclopediaProfile(t)
	want := make(map[uint32]string)

	var units struct {
		Families []struct {
			Family  string              `json:"family"`
			Records [][]json.RawMessage `json:"records"`
		} `json:"families"`
	}
	readBindingFragment(t, "encyclopedia_profiles/fragments/unit-bindings.json", &units)
	for _, family := range units.Families {
		for _, row := range family.Records {
			if len(row) < 3 {
				t.Fatalf("unit fragment family %s has a truncated compact row", family.Family)
			}
			var identity uint32
			if err := json.Unmarshal(row[2], &identity); err != nil {
				t.Fatal(err)
			}
			addExpectedFragmentIdentity(t, want, identity, family.Family)
		}
	}

	var characters struct {
		Bindings []struct {
			SourceTable    string `json:"source_table"`
			SourceIdentity string `json:"source_identity"`
		} `json:"bindings"`
	}
	readBindingFragment(t, "encyclopedia_profiles/fragments/character-bindings.json", &characters)
	for _, record := range characters.Bindings {
		family := "minor_characters"
		if record.SourceTable == "MJCHARSD.DAT" {
			family = "major_characters"
		}
		addExpectedFragmentIdentity(t, want, parseHexSourceIdentity(t, record.SourceIdentity), family)
	}

	var statics struct {
		Bindings []struct {
			SourceKind     string `json:"source_kind"`
			SourceIdentity string `json:"source_identity"`
		} `json:"bindings"`
	}
	readBindingFragment(t, "encyclopedia_profiles/fragments/static-bindings.json", &statics)
	staticFamilies := map[string]string{
		"system":                 "systems_world_locations",
		"defense_facility":       "defense_facilities",
		"manufacturing_facility": "manufacturing_facilities",
		"production_facility":    "production_facilities",
	}
	for _, record := range statics.Bindings {
		family, exists := staticFamilies[record.SourceKind]
		if !exists {
			t.Fatalf("static fragment has unknown source kind %q", record.SourceKind)
		}
		addExpectedFragmentIdentity(t, want, parseHexSourceIdentity(t, record.SourceIdentity), family)
	}

	var residual struct {
		Bindings []struct {
			Family         string `json:"family"`
			SourceIdentity uint32 `json:"source_identity"`
		} `json:"bindings"`
	}
	readBindingFragment(t, "encyclopedia_profiles/fragments/encyclopedia-residual-bindings.json", &residual)
	for _, record := range residual.Bindings {
		addExpectedFragmentIdentity(t, want, record.SourceIdentity, record.Family)
	}

	if got, expected := len(profile.Bindings.Records), len(want); got != expected {
		t.Fatalf("combined records = %d, fragment identities = %d", got, expected)
	}
	for _, record := range profile.Bindings.Records {
		family, exists := want[record.SourceIdentity]
		if !exists {
			t.Fatalf("combined source identity 0x%08x is absent from accepted fragments", record.SourceIdentity)
		}
		if family != record.Family {
			t.Fatalf("combined source identity 0x%08x family = %q, want fragment family %q", record.SourceIdentity, record.Family, family)
		}
		delete(want, record.SourceIdentity)
	}
	if len(want) != 0 {
		t.Fatalf("combined profile omitted %d accepted fragment identities", len(want))
	}
}

func TestCombinedEncyclopediaProfilePreservesReviewedSelectors(t *testing.T) {
	profile := testCombinedEncyclopediaProfile(t)
	combined := make(map[uint32]encyclopediaBindingRecord, len(profile.Bindings.Records))
	for _, record := range profile.Bindings.Records {
		combined[record.SourceIdentity] = record
	}

	var units struct {
		LanguageID uint32 `json:"language_id"`
		Families   []struct {
			Family  string              `json:"family"`
			Records [][]json.RawMessage `json:"records"`
		} `json:"families"`
	}
	readBindingFragment(t, "encyclopedia_profiles/fragments/unit-bindings.json", &units)
	for _, family := range units.Families {
		for _, row := range family.Records {
			identity := rawJSONUint32(t, row[2])
			record := combined[identity]
			got := fmt.Sprintf("%s/%d/%d/%s/%d/%d/%d/%s/%d", record.Family, record.Title.OriginalResourceID, record.Title.PreferredResourceID, record.Title.PreferredStatus, record.Title.SelectedResourceID, record.Body.ResourceID, record.Art.LookupID, record.Art.AssetBasename, record.Title.LanguageID)
			want := fmt.Sprintf("%s/%d/%d/%s/%d/%d/%d/%s/%d", family.Family, rawJSONUint32(t, row[3]), rawJSONUint32(t, row[9]), rawJSONString(t, row[10]), rawJSONUint32(t, row[11]), rawJSONUint32(t, row[4]), rawJSONUint32(t, row[6]), rawJSONString(t, row[7]), units.LanguageID)
			if got != want {
				t.Fatalf("unit identity 0x%08x selector projection = %q, want fragment %q", identity, got, want)
			}
		}
	}

	type fragmentTitle struct {
		ResourceID             uint32 `json:"resource_id"`
		LanguageID             uint32 `json:"language_id"`
		RowPreferredResourceID uint32 `json:"row_preferred_resource_id"`
		ObservedProfileBranch  string `json:"observed_profile_branch"`
	}
	type fragmentBody struct {
		ResourceID uint32 `json:"resource_id"`
		LanguageID uint32 `json:"language_id"`
		RawLength  uint64 `json:"raw_length"`
		RawSHA256  string `json:"raw_sha256"`
	}
	type fragmentArt struct {
		SelectorKind  string  `json:"selector_kind"`
		PictureID     *uint32 `json:"picture_id"`
		LookupID      uint32  `json:"lookup_id"`
		LanguageID    uint32  `json:"language_id"`
		AssetBasename string  `json:"asset_basename"`
		RawLength     uint64  `json:"raw_length"`
		RawSHA256     string  `json:"raw_sha256"`
	}
	type objectBinding struct {
		SourceIdentity  string        `json:"source_identity"`
		RawRecordSHA256 string        `json:"raw_record_sha256"`
		Title           fragmentTitle `json:"title"`
		Body            fragmentBody  `json:"body"`
		Art             fragmentArt   `json:"art"`
	}
	for _, fixture := range []struct {
		path       string
		characters bool
	}{
		{path: "encyclopedia_profiles/fragments/character-bindings.json", characters: true},
		{path: "encyclopedia_profiles/fragments/static-bindings.json"},
	} {
		var fragment struct {
			Bindings []objectBinding `json:"bindings"`
		}
		readBindingFragment(t, fixture.path, &fragment)
		for _, expected := range fragment.Bindings {
			identity := parseHexSourceIdentity(t, expected.SourceIdentity)
			record := combined[identity]
			preferred := expected.Title.RowPreferredResourceID
			preferredStatus := "empty"
			if fixture.characters {
				preferred = (expected.Title.ResourceID - 0x8000) & 0xffff
			} else if expected.Title.ObservedProfileBranch != "preferred_empty_then_fallback" {
				preferredStatus = "nonempty"
			}
			if record.RawRecordSHA256 != expected.RawRecordSHA256 ||
				record.Title.OriginalResourceID != expected.Title.ResourceID ||
				record.Title.PreferredResourceID != preferred ||
				record.Title.PreferredStatus != preferredStatus ||
				record.Title.SelectedResourceID != expected.Title.ResourceID ||
				record.Title.LanguageID != expected.Title.LanguageID ||
				record.Body.ResourceID != expected.Body.ResourceID || record.Body.LanguageID != expected.Body.LanguageID || record.Body.RawLength != expected.Body.RawLength || record.Body.RawSHA256 != expected.Body.RawSHA256 ||
				record.Art.LookupID != expected.Art.LookupID || record.Art.LanguageID != expected.Art.LanguageID || record.Art.AssetBasename != expected.Art.AssetBasename || record.Art.RawLength != expected.Art.RawLength || record.Art.RawSHA256 != expected.Art.RawSHA256 {
				t.Fatalf("binding identity 0x%08x does not preserve reviewed title/body/art evidence from %s", identity, fixture.path)
			}
			if !fixture.characters && (record.Art.SelectorKind != expected.Art.SelectorKind || !equalOptionalUint32(record.Art.PictureID, expected.Art.PictureID)) {
				t.Fatalf("binding identity 0x%08x does not preserve reviewed static art selector", identity)
			}
		}
	}
}

func TestOwnedEncyclopediaCombinedBindingProfileReconcilesInventories(t *testing.T) {
	sourceRoot := os.Getenv("REBELLION_ENCYCLOPEDIA_TEST_SOURCE")
	if sourceRoot == "" {
		t.Skip("set REBELLION_ENCYCLOPEDIA_TEST_SOURCE to an owned installation root")
	}
	absoluteRoot, err := filepath.Abs(sourceRoot)
	if err != nil {
		t.Fatal(err)
	}
	profile := testCombinedEncyclopediaProfile(t)
	gdataName, ok := findCaseInsensitiveDirectory(t, absoluteRoot, "GData")
	if !ok {
		t.Fatalf("%s does not contain GData", absoluteRoot)
	}
	edataName, ok := findCaseInsensitiveDirectory(t, absoluteRoot, "EData")
	if !ok {
		t.Fatalf("%s does not contain EData", absoluteRoot)
	}
	gdataRoot := filepath.Join(absoluteRoot, gdataName)
	edataRoot := filepath.Join(absoluteRoot, edataName)

	ownedSourcePaths := make(map[string]string, len(profile.Sources))
	for _, source := range profile.Sources {
		root := absoluteRoot
		if source.Kind == encyclopediaSourceDAT {
			root = gdataRoot
		}
		basename, found := findCaseInsensitiveFile(t, root, source.Basename)
		if !found {
			t.Fatalf("owned source missing %s", source.Basename)
		}
		path := filepath.Join(root, basename)
		ownedSourcePaths[source.Role] = path
		length, digest, err := hashEncyclopediaLookupFile(path)
		if err != nil {
			t.Fatalf("hash owned source %s: %v", source.Basename, err)
		}
		if length != source.RawLength || digest != source.RawSHA256 {
			t.Fatalf("owned source %s identity = %d/%s, want %d/%s", source.Basename, length, digest, source.RawLength, source.RawSHA256)
		}
	}

	textBasename, _ := findCaseInsensitiveFile(t, absoluteRoot, "ENCYTEXT.DLL")
	textInventory, err := inventoryEncyclopediaSources(encyclopediaInventoryRequest{
		Roots:     []encyclopediaSourceRoot{{Role: "install", Path: absoluteRoot}},
		Sources:   []encyclopediaSourceSpec{{RootRole: "install", Basename: textBasename, Kind: encyclopediaSourceDLL, ResourceTypeID: rtEncyclopediaText}},
		StartedAt: time.Unix(1, 0),
	}, defaultEncyclopediaInventoryLimits())
	if err != nil {
		t.Fatal(err)
	}
	if got, want := len(textInventory.Report.Records), len(profile.Bindings.ResourceAccounting.TextResources); got != want {
		t.Fatalf("owned text records = %d, want profile accounting %d", got, want)
	}
	textAccounting := make(map[string]encyclopediaBindingTextResource)
	for _, resource := range profile.Bindings.ResourceAccounting.TextResources {
		textAccounting[resourceLanguageKey(resource.LanguageID, resource.ResourceID)] = resource
	}
	for _, record := range textInventory.Report.Records {
		resource, exists := textAccounting[resourceLanguageKey(record.LanguageID, *record.ResourceID.NumericID)]
		if !exists || resource.RawLength != record.RawLength || resource.RawSHA256 != record.RawSHA256 {
			t.Fatalf("owned text resource %d/%d is absent or contradicts profile accounting", record.LanguageID, *record.ResourceID.NumericID)
		}
	}

	lookupBasename, _ := findCaseInsensitiveFile(t, absoluteRoot, "ENCYBMAP.DLL")
	lookupBefore, err := os.ReadFile(filepath.Join(absoluteRoot, lookupBasename))
	if err != nil {
		t.Fatal(err)
	}
	lookupInventory, err := inventoryEncyclopediaLookups(lookupBasename, lookupBefore, edataRoot)
	if err != nil {
		t.Fatal(err)
	}
	if got, want := len(lookupInventory.References), len(profile.Bindings.ResourceAccounting.ArtLookups); got != want {
		t.Fatalf("owned lookup count = %d, want profile accounting %d", got, want)
	}
	if got, want := len(lookupInventory.Files), len(profile.Bindings.ResourceAccounting.ArtFiles); got != want {
		t.Fatalf("owned art file count = %d, want profile accounting %d", got, want)
	}
	lookupAccounting := make(map[string]encyclopediaBindingArtLookup)
	for _, lookup := range profile.Bindings.ResourceAccounting.ArtLookups {
		lookupAccounting[resourceLanguageKey(lookup.LanguageID, lookup.LogicalID)] = lookup
	}
	for _, reference := range lookupInventory.References {
		lookup, exists := lookupAccounting[resourceLanguageKey(uint32(reference.LanguageID), reference.LogicalID)]
		if !exists || lookup.AssetBasename != reference.MatchedBasename {
			t.Fatalf("owned lookup %d/%d is absent or contradicts profile accounting", reference.LanguageID, reference.LogicalID)
		}
	}
	fileAccounting := make(map[string]encyclopediaBindingArtFile)
	for _, file := range profile.Bindings.ResourceAccounting.ArtFiles {
		fileAccounting[strings.ToLower(file.Basename)] = file
	}
	for _, file := range lookupInventory.Files {
		accounted, exists := fileAccounting[strings.ToLower(file.Basename)]
		if !exists || accounted.RawLength != file.RawLength || accounted.RawSHA256 != file.RawSHA256 {
			t.Fatalf("owned art file %s is absent or contradicts profile accounting", file.Basename)
		}
	}
	lookupAfter, err := os.ReadFile(filepath.Join(absoluteRoot, lookupBasename))
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(lookupBefore, lookupAfter) {
		t.Fatal("owned ENCYBMAP input changed during reconciliation")
	}
	for _, source := range profile.Sources {
		length, digest, err := hashEncyclopediaLookupFile(ownedSourcePaths[source.Role])
		if err != nil {
			t.Fatalf("rehash owned source %s: %v", source.Basename, err)
		}
		if length != source.RawLength || digest != source.RawSHA256 {
			t.Fatalf("owned source %s changed during reconciliation", source.Basename)
		}
	}
}

func TestOwnedEncyclopediaResidualBindingsReconcileSourceRowsAndSelectors(t *testing.T) {
	sourceRoot := os.Getenv("REBELLION_ENCYCLOPEDIA_TEST_SOURCE")
	if sourceRoot == "" {
		t.Skip("set REBELLION_ENCYCLOPEDIA_TEST_SOURCE to an owned installation root")
	}
	absoluteRoot, err := filepath.Abs(sourceRoot)
	if err != nil {
		t.Fatal(err)
	}
	gdataName, ok := findCaseInsensitiveDirectory(t, absoluteRoot, "GData")
	if !ok {
		t.Fatalf("%s does not contain GData", absoluteRoot)
	}
	edataName, ok := findCaseInsensitiveDirectory(t, absoluteRoot, "EData")
	if !ok {
		t.Fatalf("%s does not contain EData", absoluteRoot)
	}
	profile := testCombinedEncyclopediaProfile(t)
	bindingsByRoleAndRow := make(map[string]map[uint32]encyclopediaBindingRecord)
	for _, record := range profile.Bindings.Records {
		if bindingsByRoleAndRow[record.SourceRole] == nil {
			bindingsByRoleAndRow[record.SourceRole] = make(map[uint32]encyclopediaBindingRecord)
		}
		bindingsByRoleAndRow[record.SourceRole][record.SourceRow] = record
	}

	textstraName, ok := findCaseInsensitiveFile(t, absoluteRoot, "TEXTSTRA.DLL")
	if !ok {
		t.Fatal("owned source missing TEXTSTRA.DLL")
	}
	titleResources, err := readPERawResources(filepath.Join(absoluteRoot, textstraName), rtStringResource)
	if err != nil {
		t.Fatal(err)
	}
	titles, err := decodeStringResources(titleResources)
	if err != nil {
		t.Fatal(err)
	}
	encybmapName, ok := findCaseInsensitiveFile(t, absoluteRoot, "ENCYBMAP.DLL")
	if !ok {
		t.Fatal("owned source missing ENCYBMAP.DLL")
	}
	encybmapBytes, err := os.ReadFile(filepath.Join(absoluteRoot, encybmapName))
	if err != nil {
		t.Fatal(err)
	}
	lookupInventory, err := inventoryEncyclopediaLookups(encybmapName, encybmapBytes, filepath.Join(absoluteRoot, edataName))
	if err != nil {
		t.Fatal(err)
	}
	lookups := lookupInventory.Lookups[uint16(profile.Bindings.LanguageID)]

	expectedHeaders := map[string][2]uint32{
		"mission_definitions": {0x40, 0x80},
		"fleet_definitions":   {0x08, 0x10},
	}
	for _, coverage := range profile.Bindings.SourceCoverage {
		source := sourceByRole(t, profile.Sources, coverage.SourceRole)
		basename, found := findCaseInsensitiveFile(t, filepath.Join(absoluteRoot, gdataName), source.Basename)
		if !found {
			t.Fatalf("owned source missing %s", source.Basename)
		}
		data, err := os.ReadFile(filepath.Join(absoluteRoot, gdataName, basename))
		if err != nil {
			t.Fatal(err)
		}
		if got, want := int(binary.LittleEndian.Uint32(data[4:8])), coverage.TotalRows; got != want {
			t.Fatalf("%s row count = %d, want %d", source.Basename, got, want)
		}
		header := expectedHeaders[coverage.SourceRole]
		if gotStart, gotEnd := binary.LittleEndian.Uint32(data[8:12]), binary.LittleEndian.Uint32(data[12:16]); gotStart != header[0] || gotEnd != header[1] {
			t.Fatalf("%s family interval = [%#x,%#x), want [%#x,%#x)", source.Basename, gotStart, gotEnd, header[0], header[1])
		}
		excluded := make(map[uint32]encyclopediaBindingExcludedSourceRow)
		for _, row := range coverage.ExcludedRows {
			excluded[row.SourceRow] = row
		}
		for rowIndex := 0; rowIndex < coverage.TotalRows; rowIndex++ {
			start := int(coverage.HeaderBytes) + rowIndex*int(coverage.RecordBytes)
			raw := data[start : start+int(coverage.RecordBytes)]
			datID := binary.LittleEndian.Uint32(raw[0:4])
			family := binary.LittleEndian.Uint32(raw[16:20])
			rawHash := byteSHA256(raw)
			if record, bound := bindingsByRoleAndRow[coverage.SourceRole][uint32(rowIndex)]; bound {
				if record.DatID != datID || record.SourceFamily != family || record.RawRecordSHA256 != rawHash {
					t.Fatalf("%s row %d identity/hash contradicts embedded binding", source.Basename, rowIndex)
				}
				titleID := uint32(binary.LittleEndian.Uint16(raw[20:22]))
				if record.Title.OriginalResourceID != titleID || binary.LittleEndian.Uint16(raw[22:24]) != 2 || titles[uint16(titleID)] == "" {
					t.Fatalf("%s row %d title selector/module does not resolve through TEXTSTRA", source.Basename, rowIndex)
				}
				if titles[uint16(record.Title.PreferredResourceID)] != "" || record.Title.PreferredStatus != "empty" || record.Title.SelectedResourceID != titleID {
					t.Fatalf("%s row %d does not preserve the observed preferred-empty fallback", source.Basename, rowIndex)
				}
				for _, variant := range record.Art.ViewerFactionVariants {
					if got := lookups[variant.LookupID]; got != variant.AssetBasename {
						t.Fatalf("%s row %d %s lookup %d = %q, want %q", source.Basename, rowIndex, variant.ViewerFaction, variant.LookupID, got, variant.AssetBasename)
					}
				}
				continue
			}
			row, omitted := excluded[uint32(rowIndex)]
			if !omitted || row.DatID != datID || row.SourceFamily != family || row.RawRecordSHA256 != rawHash {
				t.Fatalf("%s row %d is neither a matching bound row nor a matching source-proven exclusion", source.Basename, rowIndex)
			}
			if coverage.SourceRole == "mission_definitions" {
				hidden := binary.LittleEndian.Uint32(raw[52:56])
				if family >= 0x50 && family < 0x80 && hidden == 0 {
					t.Fatalf("%s row %d was excluded despite satisfying the recovered cache predicate", source.Basename, rowIndex)
				}
			} else if family >= 0x08 && family < 0x10 && binary.LittleEndian.Uint16(raw[20:22]) != 0 {
				t.Fatalf("%s row %d was excluded despite satisfying the recovered cache predicate", source.Basename, rowIndex)
			}
		}
	}
}

func sourceByRole(t *testing.T, sources []encyclopediaProfileSource, role string) encyclopediaProfileSource {
	t.Helper()
	for _, source := range sources {
		if source.Role == role {
			return source
		}
	}
	t.Fatalf("profile lacks source role %q", role)
	return encyclopediaProfileSource{}
}

func testCombinedEncyclopediaProfile(t *testing.T) encyclopediaSourceProfile {
	t.Helper()
	profiles, err := loadEmbeddedEncyclopediaProfiles()
	if err != nil {
		t.Fatal(err)
	}
	data, err := json.Marshal(profiles[0])
	if err != nil {
		t.Fatal(err)
	}
	var clone encyclopediaSourceProfile
	if err := json.Unmarshal(data, &clone); err != nil {
		t.Fatal(err)
	}
	return clone
}

func readBindingFragment(t *testing.T, path string, target any) {
	t.Helper()
	data, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	if err := json.Unmarshal(data, target); err != nil {
		t.Fatal(err)
	}
}

func parseHexSourceIdentity(t *testing.T, value string) uint32 {
	t.Helper()
	parsed, err := strconv.ParseUint(strings.TrimPrefix(value, "0x"), 16, 32)
	if err != nil {
		t.Fatalf("parse source identity %q: %v", value, err)
	}
	return uint32(parsed)
}

func addExpectedFragmentIdentity(t *testing.T, identities map[uint32]string, identity uint32, family string) {
	t.Helper()
	if previous, exists := identities[identity]; exists {
		t.Fatalf("accepted fragments ambiguously repeat source identity 0x%08x in %s and %s", identity, previous, family)
	}
	identities[identity] = family
}

func rawJSONUint32(t *testing.T, raw json.RawMessage) uint32 {
	t.Helper()
	var value uint32
	if err := json.Unmarshal(raw, &value); err != nil {
		t.Fatal(err)
	}
	return value
}

func rawJSONString(t *testing.T, raw json.RawMessage) string {
	t.Helper()
	var value string
	if err := json.Unmarshal(raw, &value); err != nil {
		t.Fatal(err)
	}
	return value
}

func equalOptionalUint32(left, right *uint32) bool {
	if left == nil || right == nil {
		return left == nil && right == nil
	}
	return *left == *right
}
