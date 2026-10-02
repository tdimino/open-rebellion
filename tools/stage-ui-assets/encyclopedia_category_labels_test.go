package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"reflect"
	"sort"
	"strings"
	"testing"
)

const encyclopediaCategoryLabelFragment = "encyclopedia_profiles/fragments/encyclopedia-category-labels.json"

type encyclopediaCategoryLabelEvidence struct {
	Kind                    string                             `json:"kind"`
	SchemaVersion           int                                `json:"schema_version"`
	ProfileID               string                             `json:"profile_id"`
	Status                  string                             `json:"status"`
	Sources                 []encyclopediaCategoryLabelSource  `json:"sources"`
	Language                encyclopediaCategoryLabelLanguage  `json:"language"`
	Resource                encyclopediaCategoryLabelResource  `json:"resource"`
	Dataflow                encyclopediaCategoryLabelDataflow  `json:"dataflow"`
	SourceConstructionOrder []int                              `json:"source_construction_order"`
	Commands                []encyclopediaCategoryLabelCommand `json:"commands"`
}

type encyclopediaCategoryLabelSource struct {
	Role      string `json:"role"`
	Basename  string `json:"basename"`
	RawLength uint64 `json:"raw_length"`
	RawSHA256 string `json:"raw_sha256"`
}

type encyclopediaCategoryLabelLanguage struct {
	SelectedLanguageID       uint32   `json:"selected_language_id"`
	ObservedLanguageIDs      []uint32 `json:"observed_language_ids"`
	MultipleLanguagePolicy   string   `json:"multiple_language_policy"`
	RuntimeSelectionEvidence string   `json:"runtime_selection_evidence"`
}

type encyclopediaCategoryLabelResource struct {
	ModuleSelector                uint32 `json:"module_selector"`
	ModuleBasename                string `json:"module_basename"`
	ResourceType                  uint32 `json:"resource_type"`
	ResourceTypeName              string `json:"resource_type_name"`
	BlockID                       uint32 `json:"block_id"`
	LanguageID                    uint32 `json:"language_id"`
	CodePage                      uint32 `json:"code_page"`
	RawLength                     uint64 `json:"raw_length"`
	RawSHA256                     string `json:"raw_sha256"`
	SelectorFallback              string `json:"selector_fallback"`
	ZeroSelectorBehavior          string `json:"zero_selector_behavior"`
	PresentEmptyBehavior          string `json:"present_empty_behavior"`
	AbsentResourceBehavior        string `json:"absent_resource_behavior"`
	RuntimeEmptyAbsentDistinction string `json:"runtime_empty_absent_distinction"`
	ProfileInventoryDistinction   string `json:"profile_inventory_distinction"`
}

type encyclopediaCategoryLabelDataflow struct {
	ConstructorFunction            string `json:"constructor_function"`
	ResourcePairTemplateAddress    string `json:"resource_pair_template_address"`
	ModuleRegistrationFunction     string `json:"module_registration_function"`
	ModuleRegistrationPushAddress  string `json:"module_registration_push_address"`
	ModuleRegistrationCallAddress  string `json:"module_registration_call_address"`
	ResourceLookupCallbackFunction string `json:"resource_lookup_callback_function"`
	LoadStringCallAddress          string `json:"load_string_call_address"`
	LabelAttachFunction            string `json:"label_attach_function"`
	LabelCopyFunction              string `json:"label_copy_function"`
	SelectedCategoryFunction       string `json:"selected_category_function"`
	SelectedLabelReadAddress       string `json:"selected_label_read_address"`
	SelectedLabelWriteCallAddress  string `json:"selected_label_write_call_address"`
}

type encyclopediaCategoryLabelFilter struct {
	Start uint32 `json:"start"`
	End   uint32 `json:"end"`
}

type encyclopediaCategoryLabelCommand struct {
	DisplayOrdinal             int                              `json:"display_ordinal"`
	Command                    int                              `json:"command"`
	Role                       string                           `json:"role"`
	Filter                     *encyclopediaCategoryLabelFilter `json:"filter,omitempty"`
	X                          int                              `json:"x"`
	ConstructionOrdinal        int                              `json:"construction_ordinal"`
	Selector                   uint32                           `json:"selector"`
	ResourceBlockID            uint32                           `json:"resource_block_id"`
	BlockSlot                  int                              `json:"block_slot"`
	ResourcePresence           string                           `json:"resource_presence"`
	ContentState               string                           `json:"content_state"`
	UTF8Length                 int                              `json:"utf8_length"`
	UTF8SHA256                 string                           `json:"utf8_sha256"`
	CommandPushAddress         string                           `json:"command_push_address"`
	LabelAttachmentCallAddress string                           `json:"label_attachment_call_address"`
}

type expectedCategoryLabel struct {
	command             int
	role                string
	filterStart         uint32
	filterEnd           uint32
	x                   int
	constructionOrdinal int
	selector            uint32
	utf8Length          int
	utf8SHA256          string
	commandPushAddress  string
	attachmentAddress   string
}

var expectedCategoryLabels = []expectedCategoryLabel{
	{0x6f, "aggregate_index", 0, 0, 0, 0, 0x1850, 13, "6eb4b5f6d5723be1f6c1c215af21ca9149462b40087cafb6637777f97d57eeae", "0x0045e822", "0x0045e866"},
	{0x70, "filtered_category", 0x90, 0x98, 52, 5, 0x1855, 15, "d7b3e461cbdbaa15b2b96188ee474e1de4515ba105232325f43e45c6a5d31c60", "0x0045eba3", "0x0045ebe3"},
	{0x71, "filtered_category", 0x14, 0x20, 104, 4, 0x1854, 13, "ec24d33c0dbd2405fcf24d105925e25124ae419f49b7c5e04d764e71ced8a1a2", "0x0045eb0f", "0x0045eb52"},
	{0x72, "filtered_category", 0x20, 0x30, 156, 2, 0x1852, 19, "155e2c2bbc3c90dd466fb31a72f53d2c8194d4fb5cc94465530c247e3df52c70", "0x0045e997", "0x0045e9de"},
	{0x73, "filtered_category", 0x40, 0x80, 208, 1, 0x1851, 17, "df53b255f1559fe2354e3e9ce6eff356609b663384d0eb26c033c91840329163", "0x0045e8d7", "0x0045e922"},
	{0x74, "filtered_category", 0x10, 0x14, 260, 6, 0x1856, 14, "e9f59e5a11146ecf63026e64239d27623e5b3554ac517ab94a9bebbf0c76c8c4", "0x0045ec56", "0x0045ec99"},
	{0x75, "filtered_category", 0x30, 0x40, 312, 3, 0x1853, 18, "ca001895d7cc850d888a3009136ba631f615bb39c46578a42aeb026e111064be", "0x0045ea4f", "0x0045ea9a"},
}

func TestEncyclopediaCategoryLabelEvidenceIsCompleteAndSourceBacked(t *testing.T) {
	evidence := readEncyclopediaCategoryLabelEvidence(t)
	if err := validateEncyclopediaCategoryLabelEvidence(evidence); err != nil {
		t.Fatal(err)
	}
}

func TestEncyclopediaCategoryLabelEvidenceRejectsIncompleteOrAmbiguousMetadata(t *testing.T) {
	canonical := readEncyclopediaCategoryLabelEvidence(t)
	tests := []struct {
		name   string
		mutate func(*encyclopediaCategoryLabelEvidence)
		want   string
	}{
		{name: "duplicate command", mutate: func(e *encyclopediaCategoryLabelEvidence) { e.Commands[1].Command = e.Commands[0].Command }, want: "duplicate command"},
		{name: "missing command", mutate: func(e *encyclopediaCategoryLabelEvidence) { e.Commands = e.Commands[:6] }, want: "seven commands"},
		{name: "wrong executable source", mutate: func(e *encyclopediaCategoryLabelEvidence) { e.Sources[0].RawSHA256 = strings.Repeat("0", 64) }, want: "source identity"},
		{name: "wrong module source", mutate: func(e *encyclopediaCategoryLabelEvidence) { e.Resource.ModuleBasename = "STRATEGY.DLL" }, want: "resource identity"},
		{name: "wrong display order", mutate: func(e *encyclopediaCategoryLabelEvidence) {
			e.Commands[0], e.Commands[1] = e.Commands[1], e.Commands[0]
		}, want: "command order"},
		{name: "wrong construction order", mutate: func(e *encyclopediaCategoryLabelEvidence) {
			e.SourceConstructionOrder[0], e.SourceConstructionOrder[1] = e.SourceConstructionOrder[1], e.SourceConstructionOrder[0]
		}, want: "construction order"},
		{name: "ambiguous language", mutate: func(e *encyclopediaCategoryLabelEvidence) {
			e.Language.ObservedLanguageIDs = append(e.Language.ObservedLanguageIDs, 1031)
		}, want: "language"},
		{name: "missing provenance", mutate: func(e *encyclopediaCategoryLabelEvidence) { e.Dataflow.LoadStringCallAddress = "" }, want: "dataflow"},
	}
	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			mutant := cloneEncyclopediaCategoryLabelEvidence(t, canonical)
			test.mutate(&mutant)
			err := validateEncyclopediaCategoryLabelEvidence(mutant)
			if err == nil || !strings.Contains(err.Error(), test.want) {
				t.Fatalf("validation error = %v, want error containing %q", err, test.want)
			}
		})
	}
}

func TestOwnedEncyclopediaCategoryLabelEvidenceMatchesOriginalResourcesWithoutPrintingProse(t *testing.T) {
	sourceRoot := os.Getenv("REBELLION_ENCYCLOPEDIA_TEST_SOURCE")
	if sourceRoot == "" {
		t.Skip("set REBELLION_ENCYCLOPEDIA_TEST_SOURCE to an owned installation root")
	}
	evidence := readEncyclopediaCategoryLabelEvidence(t)
	if err := validateEncyclopediaCategoryLabelEvidence(evidence); err != nil {
		t.Fatal(err)
	}
	absoluteRoot, err := filepath.Abs(sourceRoot)
	if err != nil {
		t.Fatal(err)
	}

	for _, source := range evidence.Sources {
		basename, ok := findCaseInsensitiveFile(t, absoluteRoot, source.Basename)
		if !ok {
			t.Fatalf("owned source lacks required %s", source.Basename)
		}
		length, digest, err := hashEncyclopediaLookupFile(filepath.Join(absoluteRoot, basename))
		if err != nil {
			t.Fatal(err)
		}
		if length != source.RawLength || digest != source.RawSHA256 {
			t.Fatalf("owned %s identity = (%d,%s), want (%d,%s)", source.Basename, length, digest, source.RawLength, source.RawSHA256)
		}
	}

	dllName, ok := findCaseInsensitiveFile(t, absoluteRoot, evidence.Resource.ModuleBasename)
	if !ok {
		t.Fatalf("owned source lacks %s", evidence.Resource.ModuleBasename)
	}
	resources, err := readPERawResources(filepath.Join(absoluteRoot, dllName), evidence.Resource.ResourceType)
	if err != nil {
		t.Fatal(err)
	}
	var block *rawResource
	for index := range resources {
		resource := &resources[index]
		if resource.ID != evidence.Resource.BlockID {
			continue
		}
		if block != nil {
			t.Fatalf("resource block %d has ambiguous language/code-page entries", evidence.Resource.BlockID)
		}
		block = resource
	}
	if block == nil {
		t.Fatalf("resource block %d is absent", evidence.Resource.BlockID)
	}
	if block.Language != evidence.Resource.LanguageID || block.CodePage != evidence.Resource.CodePage {
		t.Fatalf("resource block language/code page = %d/%d, want %d/%d", block.Language, block.CodePage, evidence.Resource.LanguageID, evidence.Resource.CodePage)
	}
	if uint64(len(block.Data)) != evidence.Resource.RawLength || byteSHA256(block.Data) != evidence.Resource.RawSHA256 {
		t.Fatalf("resource block identity differs from reviewed metadata")
	}
	decoded, err := decodeEncyclopediaStringBlock(block.Data, encyclopediaResourceIdentifier{Kind: encyclopediaIdentifierNumeric, NumericID: uint32Pointer(block.ID)})
	if err != nil {
		t.Fatal(err)
	}
	for _, command := range evidence.Commands {
		value := decoded[command.BlockSlot]
		if value == "" {
			t.Fatalf("command %#x selector %#x is not present and nonempty", command.Command, command.Selector)
		}
		if len([]byte(value)) != command.UTF8Length || byteSHA256([]byte(value)) != command.UTF8SHA256 {
			t.Fatalf("command %#x label metadata differs from owned resource", command.Command)
		}
	}

	retained := struct {
		SchemaVersion     int    `json:"schema_version"`
		ProfileID         string `json:"profile_id"`
		ExecutableSHA256  string `json:"executable_sha256"`
		StringDLLSHA256   string `json:"string_dll_sha256"`
		ResourceBlockID   uint32 `json:"resource_block_id"`
		LanguageID        uint32 `json:"language_id"`
		CodePage          uint32 `json:"code_page"`
		CommandCount      int    `json:"command_count"`
		PresentNonempty   int    `json:"present_nonempty_count"`
		SelectorSetSHA256 string `json:"selector_set_sha256"`
	}{
		SchemaVersion:     1,
		ProfileID:         evidence.ProfileID,
		ExecutableSHA256:  evidence.Sources[0].RawSHA256,
		StringDLLSHA256:   evidence.Sources[1].RawSHA256,
		ResourceBlockID:   evidence.Resource.BlockID,
		LanguageID:        evidence.Resource.LanguageID,
		CodePage:          evidence.Resource.CodePage,
		CommandCount:      len(evidence.Commands),
		PresentNonempty:   len(evidence.Commands),
		SelectorSetSHA256: categoryLabelSelectorSetSHA256(evidence.Commands),
	}
	retainedBytes, err := json.MarshalIndent(retained, "", "  ")
	if err != nil {
		t.Fatal(err)
	}
	repoRoot, err := filepath.Abs(filepath.Join("..", ".."))
	if err != nil {
		t.Fatal(err)
	}
	evidencePath := filepath.Join(repoRoot, ".artifacts", "encyclopedia", "E55-owned-category-labels.json")
	if err := os.MkdirAll(filepath.Dir(evidencePath), 0o700); err != nil {
		t.Fatal(err)
	}
	writeTestFile(t, evidencePath, append(retainedBytes, '\n'))
	t.Logf("retained metadata-only category-label evidence at %s", evidencePath)
}

func readEncyclopediaCategoryLabelEvidence(t *testing.T) encyclopediaCategoryLabelEvidence {
	t.Helper()
	data, err := os.ReadFile(encyclopediaCategoryLabelFragment)
	if err != nil {
		t.Fatal(err)
	}
	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	var evidence encyclopediaCategoryLabelEvidence
	if err := decoder.Decode(&evidence); err != nil {
		t.Fatal(err)
	}
	if err := decoder.Decode(&struct{}{}); err != io.EOF {
		t.Fatalf("category-label fragment has trailing content: %v", err)
	}
	return evidence
}

func cloneEncyclopediaCategoryLabelEvidence(t *testing.T, evidence encyclopediaCategoryLabelEvidence) encyclopediaCategoryLabelEvidence {
	t.Helper()
	data, err := json.Marshal(evidence)
	if err != nil {
		t.Fatal(err)
	}
	var clone encyclopediaCategoryLabelEvidence
	if err := json.Unmarshal(data, &clone); err != nil {
		t.Fatal(err)
	}
	return clone
}

func validateEncyclopediaCategoryLabelEvidence(evidence encyclopediaCategoryLabelEvidence) error {
	if evidence.Kind != "encyclopedia-category-label-evidence" || evidence.SchemaVersion != 1 || evidence.ProfileID != "encytext-49aea545-lang-1033-v1" || evidence.Status != "complete" {
		return fmt.Errorf("invalid fragment identity/status")
	}
	wantSources := []encyclopediaCategoryLabelSource{
		{Role: "research_executable", Basename: "REBEXE.EXE", RawLength: 2822656, RawSHA256: "b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab"},
		{Role: "localized_category_labels", Basename: "TEXTSTRA.DLL", RawLength: 150528, RawSHA256: "61a10bf3797f49b1121e2fba2cee7d3949a5bc7215e1501e2df71ecbedb53d4c"},
	}
	if !reflect.DeepEqual(evidence.Sources, wantSources) {
		return fmt.Errorf("source identity differs from reviewed executable/DLL")
	}
	if evidence.Language.SelectedLanguageID != 1033 || !reflect.DeepEqual(evidence.Language.ObservedLanguageIDs, []uint32{1033}) || evidence.Language.MultipleLanguagePolicy != "reject_ambiguous_language" || evidence.Language.RuntimeSelectionEvidence != "LoadStringA_has_no_explicit_LANGID;profile_has_single_observed_LANGID_1033" {
		return fmt.Errorf("language identity or selection evidence is ambiguous")
	}
	wantResource := encyclopediaCategoryLabelResource{
		ModuleSelector:                2,
		ModuleBasename:                "TEXTSTRA.DLL",
		ResourceType:                  6,
		ResourceTypeName:              "RT_STRING",
		BlockID:                       390,
		LanguageID:                    1033,
		CodePage:                      0,
		RawLength:                     308,
		RawSHA256:                     "e428a70d970e78e7979356426fb8da15b49c06cc18cac8af8cf8d0f9d75b6d3d",
		SelectorFallback:              "none",
		ZeroSelectorBehavior:          "not_used_by_fixed_entries",
		PresentEmptyBehavior:          "empty_label",
		AbsentResourceBehavior:        "empty_label",
		RuntimeEmptyAbsentDistinction: "conflated_by_LoadStringA_zero_result",
		ProfileInventoryDistinction:   "all_seven_selectors_present_nonempty",
	}
	if !reflect.DeepEqual(evidence.Resource, wantResource) {
		return fmt.Errorf("resource identity or empty/absent semantics differ from reviewed evidence")
	}
	wantDataflow := encyclopediaCategoryLabelDataflow{
		ConstructorFunction:            "FUN_0045ddc0",
		ResourcePairTemplateAddress:    "0x0065d424",
		ModuleRegistrationFunction:     "FUN_00414830",
		ModuleRegistrationPushAddress:  "0x0041495c",
		ModuleRegistrationCallAddress:  "0x00414966",
		ResourceLookupCallbackFunction: "FUN_0060aa30",
		LoadStringCallAddress:          "0x0060aa62",
		LabelAttachFunction:            "FUN_00600a40",
		LabelCopyFunction:              "FUN_00600970",
		SelectedCategoryFunction:       "FUN_0045f100",
		SelectedLabelReadAddress:       "0x0045f415",
		SelectedLabelWriteCallAddress:  "0x0045f422",
	}
	if !reflect.DeepEqual(evidence.Dataflow, wantDataflow) {
		return fmt.Errorf("dataflow provenance differs from reviewed source addresses")
	}
	wantConstructionOrder := []int{0x6f, 0x73, 0x72, 0x75, 0x71, 0x70, 0x74}
	if !reflect.DeepEqual(evidence.SourceConstructionOrder, wantConstructionOrder) {
		return fmt.Errorf("construction order differs from FUN_0045ddc0")
	}
	if len(evidence.Commands) != len(expectedCategoryLabels) {
		return fmt.Errorf("expected seven commands, got %d", len(evidence.Commands))
	}
	seen := make(map[int]struct{}, len(evidence.Commands))
	for index, expected := range expectedCategoryLabels {
		command := evidence.Commands[index]
		if _, exists := seen[command.Command]; exists {
			return fmt.Errorf("duplicate command %#x", command.Command)
		}
		seen[command.Command] = struct{}{}
		if command.DisplayOrdinal != index || command.Command != expected.command || command.X != expected.x || command.ConstructionOrdinal != expected.constructionOrdinal || command.Selector != expected.selector || command.ResourceBlockID != 390 || command.BlockSlot != int(expected.selector&0xf) {
			return fmt.Errorf("command order/selector provenance at display ordinal %d differs from source", index)
		}
		if command.Role != expected.role {
			return fmt.Errorf("command %#x role = %q, want %q", command.Command, command.Role, expected.role)
		}
		if expected.role == "aggregate_index" {
			if command.Filter != nil {
				return fmt.Errorf("aggregate command %#x unexpectedly has a category filter", command.Command)
			}
		} else if command.Filter == nil || command.Filter.Start != expected.filterStart || command.Filter.End != expected.filterEnd {
			return fmt.Errorf("command %#x filter differs from canonical semantics", command.Command)
		}
		if command.ResourcePresence != "present" || command.ContentState != "nonempty" || command.UTF8Length != expected.utf8Length || command.UTF8SHA256 != expected.utf8SHA256 {
			return fmt.Errorf("command %#x resource presence/content metadata differs from owned profile", command.Command)
		}
		if command.CommandPushAddress != expected.commandPushAddress || command.LabelAttachmentCallAddress != expected.attachmentAddress {
			return fmt.Errorf("command %#x source reference differs from FUN_0045ddc0", command.Command)
		}
	}
	byConstruction := append([]encyclopediaCategoryLabelCommand(nil), evidence.Commands...)
	sort.Slice(byConstruction, func(i, j int) bool {
		return byConstruction[i].ConstructionOrdinal < byConstruction[j].ConstructionOrdinal
	})
	for index, command := range byConstruction {
		if command.Command != evidence.SourceConstructionOrder[index] {
			return fmt.Errorf("command %#x construction order is inconsistent", command.Command)
		}
	}
	return nil
}

func categoryLabelSelectorSetSHA256(commands []encyclopediaCategoryLabelCommand) string {
	lines := make([]string, 0, len(commands))
	for _, command := range commands {
		lines = append(lines, fmt.Sprintf("%#x|%#x|%d|%s", command.Command, command.Selector, command.UTF8Length, command.UTF8SHA256))
	}
	return byteSHA256([]byte(strings.Join(lines, "\n") + "\n"))
}
