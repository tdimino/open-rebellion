package main

import (
	"bytes"
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
	if got, want := len(bindings.Records), 331; got != want {
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
	if bindings.SchemaFit.ReadyForSchemaFreeze {
		t.Fatal("profile incorrectly claims schema-freeze readiness while source gates remain")
	}
	wantFamilies := map[string]int{
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

func TestCombinedEncyclopediaProfileRejectsReadinessWithUnresolvedEvidence(t *testing.T) {
	profile := testCombinedEncyclopediaProfile(t)
	profile.Bindings.SchemaFit.ReadyForSchemaFreeze = true
	profile.Bindings.SchemaFit.Blockers = nil

	err := validateEncyclopediaSourceProfile(profile)
	if err == nil || !strings.Contains(err.Error(), "schema-freeze readiness contradicts unresolved evidence") {
		t.Fatalf("validation error = %v, want unresolved-evidence readiness contradiction", err)
	}
}

func TestCombinedEncyclopediaProfileAcceptsReadinessWithOnlyDeferredPublicationEvidence(t *testing.T) {
	profile := testCombinedEncyclopediaProfile(t)
	for index := range profile.Bindings.Categories {
		category := &profile.Bindings.Categories[index]
		if category.Status == encyclopediaBindingUnresolved {
			category.Status = "complete"
			category.Reason = ""
			category.NextProof = ""
		}
	}

	textResources := profile.Bindings.ResourceAccounting.TextResources[:0]
	for _, resource := range profile.Bindings.ResourceAccounting.TextResources {
		if resource.Status != encyclopediaBindingUnresolved {
			textResources = append(textResources, resource)
		}
	}
	profile.Bindings.ResourceAccounting.TextResources = textResources
	profile.Bindings.ResourceAccounting.Observations.TextRecordCount = len(textResources)
	profile.Evidence.Corpus.RecordCount = len(textResources)
	profile.Evidence.Corpus.TrailingNULHistogram = map[string]int{"1": len(textResources)}
	profile.Evidence.Corpus.LFCount = 0
	profile.Evidence.Corpus.TabCount = 0

	artLookups := profile.Bindings.ResourceAccounting.ArtLookups[:0]
	distinctFilenames := make(map[string]struct{})
	for _, lookup := range profile.Bindings.ResourceAccounting.ArtLookups {
		if lookup.Status == encyclopediaBindingUnresolved {
			continue
		}
		artLookups = append(artLookups, lookup)
		distinctFilenames[strings.ToLower(lookup.AssetBasename)] = struct{}{}
	}
	profile.Bindings.ResourceAccounting.ArtLookups = artLookups
	profile.Bindings.ResourceAccounting.Observations.NonemptyLookupStringCount = len(artLookups)
	profile.Bindings.ResourceAccounting.Observations.DistinctLookupFilenameCount = len(distinctFilenames)

	artFiles := profile.Bindings.ResourceAccounting.ArtFiles[:0]
	deferredFiles := 0
	for _, file := range profile.Bindings.ResourceAccounting.ArtFiles {
		if file.Status == encyclopediaBindingUnresolved {
			continue
		}
		artFiles = append(artFiles, file)
		if file.Status == encyclopediaBindingPublicationDeferred {
			deferredFiles++
		}
	}
	profile.Bindings.ResourceAccounting.ArtFiles = artFiles
	profile.Bindings.ResourceAccounting.Observations.ImageFileCount = len(artFiles)
	profile.Bindings.SchemaFit.ReadyForSchemaFreeze = true
	profile.Bindings.SchemaFit.Blockers = nil

	if deferredFiles != 1 {
		t.Fatalf("synthetic profile has %d deferred files, want one explicit deferred publication item", deferredFiles)
	}
	if err := validateEncyclopediaSourceProfile(profile); err != nil {
		t.Fatalf("fully reconciled synthetic profile with explicit deferred publication evidence was rejected: %v", err)
	}
}

func TestCombinedEncyclopediaProfileRejectsOmittedUnresolvedDecoderCorpusResource(t *testing.T) {
	profile := testCombinedEncyclopediaProfile(t)
	for index, resource := range profile.Bindings.ResourceAccounting.TextResources {
		if resource.Status != encyclopediaBindingUnresolved {
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
	t.Fatal("embedded profile has no unresolved text resource fixture")
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
			want: "references missing accounted file",
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
	canonical := profile.Bindings.Records[0]
	alias := &profile.Bindings.Records[1]
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
