package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestValidateEncyclopediaRuntimeConsumesApplicableE37BundleCases(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	for _, test := range []struct {
		name     string
		bundle   string
		wantCode string
	}{
		{name: "valid runtime-only bundle", bundle: "valid"},
		{name: "source-free validation does not claim DAT pairing", bundle: "invalid-binding-source-hash"},
		{name: "catalog digest contradiction", bundle: "invalid-catalog-hash", wantCode: "catalog_digest_mismatch"},
		{name: "files catalog digest contradiction", bundle: "invalid-files-catalog-hash", wantCode: "catalog_digest_mismatch"},
		{name: "missing image manifest entry", bundle: "invalid-missing-image-file-entry", wantCode: "manifest_file_set_mismatch"},
		{name: "extra manifest file", bundle: "invalid-extra-file-entry", wantCode: "manifest_file_set_mismatch"},
		{name: "changed image bytes", bundle: "invalid-image-file-hash", wantCode: "file_digest_mismatch"},
		{name: "descriptor digest contradiction", bundle: "invalid-image-descriptor-hash", wantCode: "image_digest_mismatch"},
		{name: "dangling source reference", bundle: "invalid-dangling-source-ref", wantCode: "dangling_source_ref"},
		{name: "base PNG is forbidden", bundle: "invalid-base-png", wantCode: "unsupported_base_image_format"},
		{name: "declared format differs from bytes", bundle: "invalid-base-format-mismatch", wantCode: "image_format_mismatch"},
	} {
		t.Run(test.name, func(t *testing.T) {
			catalog, manifest, files := loadE37RuntimeBundle(t, filepath.Join(root, "fixtures", "bundles", test.bundle))
			err := validateEncyclopediaRuntime(catalog, manifest, files)
			if test.wantCode == "" {
				if err != nil {
					t.Fatalf("validateEncyclopediaRuntime() error = %v", err)
				}
				return
			}
			if got := encyclopediaValidationCode(err); got != test.wantCode {
				t.Fatalf("validation code = %q (%v), want %q", got, err, test.wantCode)
			}
		})
	}
}

func TestE37ValidCatalogPreservesFamilyQualifiedDatIDsAndViewerFactionVariant(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	catalogBytes := mustReadTestFile(t, filepath.Join(root, "fixtures", "bundles", "valid", "catalog.json"))
	catalog, err := parseEncyclopediaCatalog(catalogBytes)
	if err != nil {
		t.Fatal(err)
	}

	familiesForDatID7 := make(map[string]struct{})
	foundFactionVariant := false
	for _, binding := range catalog.Bindings {
		if binding.DatID == 7 {
			familiesForDatID7[binding.Family] = struct{}{}
		}
		if binding.Variant != "viewer_faction" {
			continue
		}
		topic := catalog.Topics[binding.TopicID]
		for _, localized := range topic.Localized {
			if localized.ImageSelector == nil || localized.ImageSelector.Kind != "viewer_faction" {
				t.Fatalf("viewer-faction binding %s lacks its selector", binding.TopicID)
			}
		}
		foundFactionVariant = true
	}
	if len(familiesForDatID7) != 2 {
		t.Fatalf("family-qualified DAT ID 7 families = %#v, want two distinct families", familiesForDatID7)
	}
	if !foundFactionVariant {
		t.Fatal("valid E37 catalog lacks its viewer-faction variant")
	}
}

func TestValidateEncyclopediaRuntimeRejectsDuplicateKeysBeforeMapConstruction(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	validCatalog, manifest, files := loadE37RuntimeBundle(t, filepath.Join(root, "fixtures", "bundles", "valid"))
	_ = validCatalog
	for _, fixture := range []string{
		"catalog-duplicate-root-key.json",
		"catalog-escaped-equivalent-duplicate-key.json",
		"catalog-duplicate-topic-key.json",
		"catalog-duplicate-langid-key.json",
		"catalog-duplicate-localized-field.json",
	} {
		t.Run(fixture, func(t *testing.T) {
			catalog := mustReadTestFile(t, filepath.Join(root, "fixtures", "raw", fixture))
			files["catalog.json"] = encyclopediaAssetFactsFromBytes(catalog, "")
			if got := encyclopediaValidationCode(validateEncyclopediaRuntime(catalog, manifest, files)); got != "duplicate_key" {
				t.Fatalf("validation code = %q, want duplicate_key", got)
			}
		})
	}
}

func TestParseEncyclopediaFilesConsumesE37StructuralCases(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	for _, test := range []struct {
		name     string
		kind     string
		fixture  string
		wantCode string
	}{
		{name: "catalog aliases unknown", kind: "catalog", fixture: "catalog-unknown-aliases.json", wantCode: "unknown_field"},
		{name: "catalog topic sort missing", kind: "catalog", fixture: "catalog-missing-topic-sort.json", wantCode: "missing_field"},
		{name: "catalog topic sort wrong", kind: "catalog", fixture: "catalog-wrong-topic-sort.json", wantCode: "invalid_topic_sort"},
		{name: "catalog static and selector", kind: "catalog", fixture: "catalog-mixed-static-selector.json", wantCode: "ambiguous_image_selector"},
		{name: "catalog faction side missing", kind: "catalog", fixture: "catalog-selector-missing-side.json", wantCode: "missing_field"},
		{name: "catalog selector kind", kind: "catalog", fixture: "catalog-selector-unknown-kind.json", wantCode: "unsupported_selector"},
		{name: "catalog localized unknown", kind: "catalog", fixture: "catalog-localized-unknown-field.json", wantCode: "unknown_field"},
		{name: "catalog bad language", kind: "catalog", fixture: "catalog-bad-langid.json", wantCode: "invalid_language_id"},
		{name: "catalog unknown version", kind: "catalog", fixture: "catalog-unknown-version.json", wantCode: "unsupported_version"},
		{name: "catalog image bytes", kind: "catalog", fixture: "catalog-image-byte-above.json", wantCode: "resource_limit:image_bytes"},
		{name: "catalog mod image ID", kind: "catalog", fixture: "catalog-bad-base-image-id.json", wantCode: "invalid_base_image_id"},
		{name: "catalog image format", kind: "catalog", fixture: "catalog-unsupported-image-format.json", wantCode: "unsupported_image_format"},
		{name: "catalog unsafe path", kind: "catalog", fixture: "catalog-unsafe-asset-path.json", wantCode: "unsafe_asset_path"},
		{name: "manifest schema missing", kind: "manifest", fixture: "manifest-missing-schema-version.json", wantCode: "missing_field"},
		{name: "manifest profile missing", kind: "manifest", fixture: "manifest-missing-source-profile.json", wantCode: "missing_field"},
		{name: "manifest extractor missing", kind: "manifest", fixture: "manifest-missing-extractor-version.json", wantCode: "missing_field"},
		{name: "manifest catalog digest missing", kind: "manifest", fixture: "manifest-missing-catalog-sha256.json", wantCode: "missing_field"},
		{name: "manifest files missing", kind: "manifest", fixture: "manifest-missing-files.json", wantCode: "missing_field"},
		{name: "manifest binding sources missing", kind: "manifest", fixture: "manifest-missing-binding-sources.json", wantCode: "missing_field"},
		{name: "manifest source records missing", kind: "manifest", fixture: "manifest-missing-source-records.json", wantCode: "missing_field"},
		{name: "manifest self hash", kind: "manifest", fixture: "manifest-self-hash.json", wantCode: "unexpected_runtime_file"},
		{name: "manifest unknown", kind: "manifest", fixture: "manifest-unknown-field.json", wantCode: "unknown_field"},
		{name: "manifest unsafe path", kind: "manifest", fixture: "manifest-unsafe-path.json", wantCode: "unsafe_asset_path"},
	} {
		t.Run(test.name, func(t *testing.T) {
			data := mustReadTestFile(t, filepath.Join(root, "fixtures", "schema", test.fixture))
			var err error
			if test.kind == "catalog" {
				_, err = parseEncyclopediaCatalog(data)
			} else {
				_, err = parseEncyclopediaManifest(data)
			}
			if got := encyclopediaValidationCode(err); got != test.wantCode {
				t.Fatalf("validation code = %q (%v), want %q", got, err, test.wantCode)
			}
		})
	}
}

func TestParseEncyclopediaFilesConsumesE37RawManifestAndUTF8Cases(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	for _, test := range []struct{ kind, fixture, code string }{
		{kind: "manifest", fixture: "manifest-duplicate-file-key.json", code: "duplicate_key"},
		{kind: "manifest", fixture: "manifest-duplicate-source-record-key.json", code: "duplicate_key"},
		{kind: "catalog", fixture: "catalog-invalid-utf8.json", code: "invalid_utf8"},
	} {
		t.Run(test.fixture, func(t *testing.T) {
			data := mustReadTestFile(t, filepath.Join(root, "fixtures", "raw", test.fixture))
			var err error
			if test.kind == "catalog" {
				_, err = parseEncyclopediaCatalog(data)
			} else {
				_, err = parseEncyclopediaManifest(data)
			}
			if got := encyclopediaValidationCode(err); got != test.code {
				t.Fatalf("validation code = %q (%v), want %q", got, err, test.code)
			}
		})
	}
}

func TestParseEncyclopediaCatalogRequiresEveryExactNestedWireField(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	original := mustReadTestFile(t, filepath.Join(root, "fixtures", "bundles", "valid", "catalog.json"))
	targets := []struct {
		name     string
		required []string
		object   func(map[string]any) map[string]any
	}{
		{name: "topic_sort", required: []string{"algorithm", "representable_encoding", "representable_fold", "unrepresentable", "tie_break"}, object: func(doc map[string]any) map[string]any { return doc["topic_sort"].(map[string]any) }},
		{name: "index", required: []string{"command", "labels", "topic_ids", "source_ref"}, object: func(doc map[string]any) map[string]any { return doc["index"].(map[string]any) }},
		{name: "category", required: []string{"id", "command", "labels", "topic_ids", "source_ref"}, object: func(doc map[string]any) map[string]any { return doc["categories"].([]any)[0].(map[string]any) }},
		{name: "topic", required: []string{"localized", "source_ref"}, object: func(doc map[string]any) map[string]any { return catalogFixtureTopic(doc, "original:60001") }},
		{name: "localized_content", required: []string{"title", "body"}, object: func(doc map[string]any) map[string]any { return catalogFixtureLocalized(doc, "original:60001", "1033") }},
		{name: "image_selector", required: []string{"kind", "alliance_image_id", "empire_image_id"}, object: func(doc map[string]any) map[string]any {
			return catalogFixtureLocalized(doc, "original:60004", "1033")["image_selector"].(map[string]any)
		}},
		{name: "image", required: []string{"path", "format", "byte_length", "width", "height", "sha256", "source_ref"}, object: func(doc map[string]any) map[string]any {
			return doc["images"].(map[string]any)["edata:1"].(map[string]any)
		}},
		{name: "binding", required: []string{"family", "dat_id", "variant", "topic_id"}, object: func(doc map[string]any) map[string]any { return doc["bindings"].([]any)[0].(map[string]any) }},
	}
	for _, target := range targets {
		for _, field := range target.required {
			t.Run(target.name+"_missing_"+field, func(t *testing.T) {
				changed := mutateJSONFixture(t, original, func(doc map[string]any) { delete(target.object(doc), field) })
				if got := encyclopediaValidationCode(catalogParseError(changed)); got != "missing_field" {
					t.Fatalf("validation code = %q, want missing_field", got)
				}
			})
		}
	}
}

func TestParseEncyclopediaCatalogRejectsForbiddenNullsAndCaseAliases(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	original := mustReadTestFile(t, filepath.Join(root, "fixtures", "bundles", "valid", "catalog.json"))
	for _, test := range []struct {
		name string
		do   func(map[string]any)
	}{
		{name: "null root array", do: func(doc map[string]any) { doc["categories"] = nil }},
		{name: "null index labels map", do: func(doc map[string]any) { doc["index"].(map[string]any)["labels"] = nil }},
		{name: "null label value", do: func(doc map[string]any) { doc["index"].(map[string]any)["labels"].(map[string]any)["1033"] = nil }},
		{name: "null topic map value", do: func(doc map[string]any) { doc["topics"].(map[string]any)["original:60001"] = nil }},
		{name: "null localized map", do: func(doc map[string]any) { catalogFixtureTopic(doc, "original:60001")["localized"] = nil }},
		{name: "null localized value", do: func(doc map[string]any) {
			catalogFixtureTopic(doc, "original:60001")["localized"].(map[string]any)["1033"] = nil
		}},
		{name: "null title", do: func(doc map[string]any) { catalogFixtureLocalized(doc, "original:60001", "1033")["title"] = nil }},
		{name: "null body", do: func(doc map[string]any) { catalogFixtureLocalized(doc, "original:60001", "1033")["body"] = nil }},
		{name: "wrong case body", do: func(doc map[string]any) {
			localized := catalogFixtureLocalized(doc, "original:60001", "1033")
			localized["Body"] = localized["body"]
			delete(localized, "body")
		}},
		{name: "null image selector", do: func(doc map[string]any) {
			catalogFixtureLocalized(doc, "original:60004", "1033")["image_selector"] = nil
		}},
		{name: "null image map value", do: func(doc map[string]any) { doc["images"].(map[string]any)["edata:1"] = nil }},
		{name: "null binding dat id", do: func(doc map[string]any) { doc["bindings"].([]any)[0].(map[string]any)["dat_id"] = nil }},
		{name: "null category item", do: func(doc map[string]any) { doc["categories"].([]any)[0] = nil }},
		{name: "null binding item", do: func(doc map[string]any) { doc["bindings"].([]any)[0] = nil }},
		{name: "null topic id item", do: func(doc map[string]any) { doc["index"].(map[string]any)["topic_ids"].([]any)[0] = nil }},
		{name: "null image id plus selector", do: func(doc map[string]any) { catalogFixtureLocalized(doc, "original:60004", "1033")["image_id"] = nil }},
	} {
		t.Run(test.name, func(t *testing.T) {
			changed := mutateJSONFixture(t, original, test.do)
			if err := catalogParseError(changed); err == nil {
				t.Fatal("schema-invalid catalog was accepted")
			}
		})
	}
}

func TestParseEncyclopediaCatalogRejectsCaseAliasesAtEveryFixedObjectLevel(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	original := mustReadTestFile(t, filepath.Join(root, "fixtures", "bundles", "valid", "catalog.json"))
	for _, test := range []struct {
		name string
		do   func(map[string]any)
	}{
		{name: "root", do: func(doc map[string]any) { doc["Schema_Version"] = doc["schema_version"] }},
		{name: "topic sort", do: func(doc map[string]any) { doc["topic_sort"].(map[string]any)["Algorithm"] = "stable_display_title_v1" }},
		{name: "index", do: func(doc map[string]any) { doc["index"].(map[string]any)["Command"] = "0x6f" }},
		{name: "category", do: func(doc map[string]any) { doc["categories"].([]any)[0].(map[string]any)["ID"] = "command:0x70" }},
		{name: "topic", do: func(doc map[string]any) { catalogFixtureTopic(doc, "original:60001")["Localized"] = map[string]any{} }},
		{name: "localized", do: func(doc map[string]any) { catalogFixtureLocalized(doc, "original:60001", "1033")["BODY"] = "alias" }},
		{name: "selector", do: func(doc map[string]any) {
			catalogFixtureLocalized(doc, "original:60004", "1033")["image_selector"].(map[string]any)["Kind"] = "viewer_faction"
		}},
		{name: "image", do: func(doc map[string]any) {
			doc["images"].(map[string]any)["edata:1"].(map[string]any)["Path"] = "assets/EDATA.001"
		}},
		{name: "binding", do: func(doc map[string]any) { doc["bindings"].([]any)[0].(map[string]any)["Dat_ID"] = float64(7) }},
	} {
		t.Run(test.name, func(t *testing.T) {
			changed := mutateJSONFixture(t, original, test.do)
			if got := encyclopediaValidationCode(catalogParseError(changed)); got != "unknown_field" {
				t.Fatalf("validation code = %q, want unknown_field", got)
			}
		})
	}
}

func TestParseEncyclopediaCatalogRejectsWrongNestedScalarTypes(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	original := mustReadTestFile(t, filepath.Join(root, "fixtures", "bundles", "valid", "catalog.json"))
	for _, test := range []struct {
		name string
		do   func(map[string]any)
	}{
		{name: "label number", do: func(doc map[string]any) {
			doc["index"].(map[string]any)["labels"].(map[string]any)["1033"] = float64(1)
		}},
		{name: "topic id boolean", do: func(doc map[string]any) { doc["index"].(map[string]any)["topic_ids"].([]any)[0] = true }},
		{name: "title number", do: func(doc map[string]any) { catalogFixtureLocalized(doc, "original:60001", "1033")["title"] = float64(1) }},
		{name: "image byte length string", do: func(doc map[string]any) {
			doc["images"].(map[string]any)["edata:1"].(map[string]any)["byte_length"] = "1"
		}},
		{name: "binding dat id string", do: func(doc map[string]any) { doc["bindings"].([]any)[0].(map[string]any)["dat_id"] = "7" }},
	} {
		t.Run(test.name, func(t *testing.T) {
			changed := mutateJSONFixture(t, original, test.do)
			if got := encyclopediaValidationCode(catalogParseError(changed)); got != "invalid_type" {
				t.Fatalf("validation code = %q, want invalid_type", got)
			}
		})
	}
}

func TestParseEncyclopediaCatalogPreservesSchemaPermittedEmptyZeroAndNull(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	original := mustReadTestFile(t, filepath.Join(root, "fixtures", "bundles", "valid", "catalog.json"))
	changed := mutateJSONFixture(t, original, func(doc map[string]any) {
		localized := catalogFixtureLocalized(doc, "original:60001", "1033")
		localized["body"] = ""
		localized["image_id"] = nil
		catalogFixtureLocalized(doc, "original:60004", "1033")["image_selector"].(map[string]any)["alliance_image_id"] = nil
		doc["bindings"].([]any)[0].(map[string]any)["dat_id"] = float64(0)
	})
	if _, err := parseEncyclopediaCatalog(changed); err != nil {
		t.Fatalf("schema-permitted empty body, null image, and zero dat_id rejected: %v", err)
	}
}

func TestParseEncyclopediaManifestRequiresEveryExactNestedWireField(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	original := mustReadTestFile(t, filepath.Join(root, "fixtures", "bundles", "valid", "manifest.json"))
	targets := []struct {
		name     string
		required []string
		object   func(map[string]any) map[string]any
	}{
		{name: "binding_source", required: []string{"basename", "sha256"}, object: func(doc map[string]any) map[string]any { return doc["binding_sources"].([]any)[0].(map[string]any) }},
		{name: "source_record", required: []string{"source_basename", "source_sha256", "resource_type", "resource_id", "language_id", "raw_length", "raw_sha256", "decoder", "encoding", "mapping_citations"}, object: firstManifestSourceRecord},
		{name: "resource_type", required: []string{"kind", "value"}, object: func(doc map[string]any) map[string]any {
			return firstManifestSourceRecord(doc)["resource_type"].(map[string]any)
		}},
		{name: "resource_id", required: []string{"kind", "value"}, object: func(doc map[string]any) map[string]any {
			return firstManifestSourceRecord(doc)["resource_id"].(map[string]any)
		}},
	}
	for _, target := range targets {
		for _, field := range target.required {
			t.Run(target.name+"_missing_"+field, func(t *testing.T) {
				changed := mutateJSONFixture(t, original, func(doc map[string]any) { delete(target.object(doc), field) })
				if got := encyclopediaValidationCode(manifestParseError(changed)); got != "missing_field" {
					t.Fatalf("validation code = %q, want missing_field", got)
				}
			})
		}
	}
}

func TestParseEncyclopediaManifestRejectsForbiddenNullsAndCaseAliases(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	original := mustReadTestFile(t, filepath.Join(root, "fixtures", "bundles", "valid", "manifest.json"))
	for _, test := range []struct {
		name string
		do   func(map[string]any)
	}{
		{name: "null files map", do: func(doc map[string]any) { doc["files"] = nil }},
		{name: "null file digest", do: func(doc map[string]any) { doc["files"].(map[string]any)["catalog.json"] = nil }},
		{name: "null binding sources array", do: func(doc map[string]any) { doc["binding_sources"] = nil }},
		{name: "null binding source item", do: func(doc map[string]any) { doc["binding_sources"].([]any)[0] = nil }},
		{name: "null source records map", do: func(doc map[string]any) { doc["source_records"] = nil }},
		{name: "null source record value", do: func(doc map[string]any) {
			for key := range doc["source_records"].(map[string]any) {
				doc["source_records"].(map[string]any)[key] = nil
				break
			}
		}},
		{name: "null language id", do: func(doc map[string]any) { firstManifestSourceRecord(doc)["language_id"] = nil }},
		{name: "null raw length", do: func(doc map[string]any) { firstManifestSourceRecord(doc)["raw_length"] = nil }},
		{name: "wrong case raw length", do: func(doc map[string]any) {
			record := firstManifestSourceRecord(doc)
			record["Raw_Length"] = record["raw_length"]
			delete(record, "raw_length")
		}},
		{name: "null resource identifier", do: func(doc map[string]any) { firstManifestSourceRecord(doc)["resource_id"] = nil }},
		{name: "null resource value", do: func(doc map[string]any) {
			firstManifestSourceRecord(doc)["resource_id"].(map[string]any)["value"] = nil
		}},
		{name: "null citations array", do: func(doc map[string]any) { firstManifestSourceRecord(doc)["mapping_citations"] = nil }},
		{name: "null citation item", do: func(doc map[string]any) { firstManifestSourceRecord(doc)["mapping_citations"].([]any)[0] = nil }},
	} {
		t.Run(test.name, func(t *testing.T) {
			changed := mutateJSONFixture(t, original, test.do)
			if err := manifestParseError(changed); err == nil {
				t.Fatal("schema-invalid manifest was accepted")
			}
		})
	}
}

func TestParseEncyclopediaManifestRejectsCaseAliasesAtEveryFixedObjectLevel(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	original := mustReadTestFile(t, filepath.Join(root, "fixtures", "bundles", "valid", "manifest.json"))
	for _, test := range []struct {
		name string
		do   func(map[string]any)
	}{
		{name: "root", do: func(doc map[string]any) { doc["Source_Profile"] = doc["source_profile"] }},
		{name: "binding source", do: func(doc map[string]any) {
			doc["binding_sources"].([]any)[0].(map[string]any)["Basename"] = "SYNTHETIC.DAT"
		}},
		{name: "source record", do: func(doc map[string]any) { firstManifestSourceRecord(doc)["Raw_Length"] = float64(29) }},
		{name: "resource identifier", do: func(doc map[string]any) {
			firstManifestSourceRecord(doc)["resource_id"].(map[string]any)["Value"] = float64(60001)
		}},
	} {
		t.Run(test.name, func(t *testing.T) {
			changed := mutateJSONFixture(t, original, test.do)
			if got := encyclopediaValidationCode(manifestParseError(changed)); got != "unknown_field" {
				t.Fatalf("validation code = %q, want unknown_field", got)
			}
		})
	}
}

func TestParseEncyclopediaManifestRejectsWrongNestedScalarTypes(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	original := mustReadTestFile(t, filepath.Join(root, "fixtures", "bundles", "valid", "manifest.json"))
	for _, test := range []struct {
		name string
		do   func(map[string]any)
	}{
		{name: "file digest number", do: func(doc map[string]any) { doc["files"].(map[string]any)["catalog.json"] = float64(1) }},
		{name: "binding basename number", do: func(doc map[string]any) { doc["binding_sources"].([]any)[0].(map[string]any)["basename"] = float64(1) }},
		{name: "language id string", do: func(doc map[string]any) { firstManifestSourceRecord(doc)["language_id"] = "1033" }},
		{name: "resource value string", do: func(doc map[string]any) {
			firstManifestSourceRecord(doc)["resource_id"].(map[string]any)["value"] = "60001"
		}},
		{name: "citation number", do: func(doc map[string]any) { firstManifestSourceRecord(doc)["mapping_citations"].([]any)[0] = float64(1) }},
	} {
		t.Run(test.name, func(t *testing.T) {
			changed := mutateJSONFixture(t, original, test.do)
			if got := encyclopediaValidationCode(manifestParseError(changed)); got != "invalid_type" {
				t.Fatalf("validation code = %q, want invalid_type", got)
			}
		})
	}
}

func TestParseEncyclopediaManifestPreservesSchemaPermittedNumericZero(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	original := mustReadTestFile(t, filepath.Join(root, "fixtures", "bundles", "valid", "manifest.json"))
	changed := mutateJSONFixture(t, original, func(doc map[string]any) {
		record := firstManifestSourceRecord(doc)
		record["language_id"] = float64(0)
		record["raw_length"] = float64(0)
		record["resource_id"].(map[string]any)["value"] = float64(0)
	})
	if _, err := parseEncyclopediaManifest(changed); err != nil {
		t.Fatalf("schema-permitted numeric zero rejected: %v", err)
	}
}

func TestEncyclopediaManifestMarshallingIsDeterministic(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	manifest, err := parseEncyclopediaManifest(mustReadTestFile(t, filepath.Join(root, "fixtures", "bundles", "valid", "manifest.json")))
	if err != nil {
		t.Fatal(err)
	}
	first, err := marshalEncyclopediaManifest(manifest)
	if err != nil {
		t.Fatal(err)
	}
	second, err := marshalEncyclopediaManifest(manifest)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(first, second) {
		t.Fatal("identical manifest models did not produce byte-identical JSON")
	}
}

func TestValidateEncyclopediaRuntimeRejectsRelationshipFailures(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	_, manifest, files := loadE37RuntimeBundle(t, filepath.Join(root, "fixtures", "bundles", "valid"))
	for _, test := range []struct{ fixture, code string }{
		{fixture: "catalog-frozen-admitted-subset.json", code: "potential_membership_mismatch"},
		{fixture: "catalog-dangling-index-topic.json", code: "dangling_topic_reference"},
		{fixture: "catalog-duplicate-index-topic.json", code: "duplicate_topic_id"},
		{fixture: "catalog-duplicate-category-topic.json", code: "duplicate_topic_id"},
		{fixture: "catalog-duplicate-category-command.json", code: "duplicate_category_command"},
		{fixture: "catalog-wrong-category-order.json", code: "category_order"},
		{fixture: "catalog-duplicate-binding-tuple.json", code: "ambiguous_binding"},
		{fixture: "catalog-dangling-binding-topic.json", code: "dangling_topic_reference"},
		{fixture: "catalog-dangling-image-reference.json", code: "dangling_image_reference"},
		{fixture: "catalog-default-binding-faction-selector.json", code: "binding_selector_mismatch"},
		{fixture: "catalog-faction-binding-static-selector.json", code: "binding_selector_mismatch"},
	} {
		t.Run(test.fixture, func(t *testing.T) {
			catalog := mustReadTestFile(t, filepath.Join(root, "fixtures", "relationships", test.fixture))
			files["catalog.json"] = encyclopediaAssetFactsFromBytes(catalog, "")
			// Relationship diagnostics must win before immutable digest checks.
			if got := encyclopediaValidationCode(validateEncyclopediaRuntime(catalog, manifest, files)); got != test.code {
				t.Fatalf("validation code = %q, want %q", got, test.code)
			}
		})
	}
}

func TestE37SyntheticMembershipOracleRejectsInventedFilteredMembership(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	validBytes := mustReadTestFile(t, filepath.Join(root, "fixtures", "bundles", "valid", "catalog.json"))
	valid, err := parseEncyclopediaCatalog(validBytes)
	if err != nil {
		t.Fatal(err)
	}
	oracle := encyclopediaPotentialMembershipOracle{
		Index:      append([]string(nil), valid.Index.TopicIDs...),
		Categories: make(map[string][]string, len(valid.Categories)),
	}
	for _, category := range valid.Categories {
		oracle.Categories[category.Command] = append([]string(nil), category.TopicIDs...)
	}
	if err := validateEncyclopediaPotentialMembership(valid, oracle); err != nil {
		t.Fatalf("valid synthetic source profile rejected: %v", err)
	}
	inventedBytes := mustReadTestFile(t, filepath.Join(root, "fixtures", "relationships", "catalog-invented-filtered-membership.json"))
	invented, err := parseEncyclopediaCatalog(inventedBytes)
	if err != nil {
		t.Fatalf("fixture is structurally valid before source-profile validation: %v", err)
	}
	if got := encyclopediaValidationCode(validateEncyclopediaPotentialMembership(invented, oracle)); got != "filtered_membership_mismatch" {
		t.Fatalf("validation code = %q, want filtered_membership_mismatch", got)
	}
}

func TestVerifyBindingSourcesIsSeparateFromSourceFreeRuntimeIntegrity(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	catalogBytes, manifestBytes, files := loadE37RuntimeBundle(t, filepath.Join(root, "fixtures", "bundles", "valid"))
	if err := validateEncyclopediaRuntime(catalogBytes, manifestBytes, files); err != nil {
		t.Fatalf("source-free runtime validation failed: %v", err)
	}
	manifest, err := parseEncyclopediaManifest(manifestBytes)
	if err != nil {
		t.Fatal(err)
	}
	if err := verifyBindingSources(manifest, map[string]string{
		"synthetic.dat": "39fb2c329d34fbdd94bb2a2a596b694597eb00b8b402905ec4920f560210d322",
	}); err != nil {
		t.Fatalf("verifyBindingSources() error = %v", err)
	}
	if got := encyclopediaValidationCode(verifyBindingSources(manifest, map[string]string{"SYNTHETIC.DAT": strings.Repeat("0", 64)})); got != "binding_source_mismatch" {
		t.Fatalf("wrong DAT validation code = %q, want binding_source_mismatch", got)
	}
}

func TestVerifyBindingSourcesConsumesE37PairingFailure(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	bundle := filepath.Join(root, "fixtures", "bundles", "invalid-binding-source-hash")
	manifest, err := parseEncyclopediaManifest(mustReadTestFile(t, filepath.Join(bundle, "manifest.json")))
	if err != nil {
		t.Fatal(err)
	}
	actualDAT := mustReadTestFile(t, filepath.Join(bundle, "sources", "SYNTHETIC.DAT"))
	if got := encyclopediaValidationCode(verifyBindingSources(manifest, map[string]string{"SYNTHETIC.DAT": byteSHA256(actualDAT)})); got != "binding_source_mismatch" {
		t.Fatalf("validation code = %q, want binding_source_mismatch", got)
	}
}

func TestRawJSONScannerAcceptsNestedValuesAndRepeatedKeysInSeparateScopes(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	for _, fixture := range []string{"valid-nested-values.json", "valid-repeated-keys-distinct-scopes.json"} {
		data := mustReadTestFile(t, filepath.Join(root, "fixtures", "raw", fixture))
		if err := validateRawEncyclopediaJSON(data, encyclopediaMaxCatalogJSONBytes, encyclopediaMaxCatalogJSONDepth); err != nil {
			t.Fatalf("%s: validateRawEncyclopediaJSON() error = %v", fixture, err)
		}
	}
}

func TestEncyclopediaCatalogResourceBoundariesUseUTF8BytesAndCheckedPixels(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	baseBytes := mustReadTestFile(t, filepath.Join(root, "fixtures", "bundles", "valid", "catalog.json"))
	base, err := parseEncyclopediaCatalog(baseBytes)
	if err != nil {
		t.Fatal(err)
	}

	for _, test := range []struct {
		name string
		do   func(*encyclopediaCatalog)
		code string
	}{
		{name: "body exact bytes", do: func(c *encyclopediaCatalog) { setFirstCatalogBody(c, strings.Repeat("b", encyclopediaMaxBodyBytes)) }},
		{name: "body one above bytes", do: func(c *encyclopediaCatalog) { setFirstCatalogBody(c, strings.Repeat("b", encyclopediaMaxBodyBytes+1)) }, code: "resource_limit:body_bytes"},
		{name: "multibyte body one above bytes", do: func(c *encyclopediaCatalog) {
			setFirstCatalogBody(c, strings.Repeat("é", encyclopediaMaxBodyBytes/2+1))
		}, code: "resource_limit:body_bytes"},
		{name: "title exact bytes", do: func(c *encyclopediaCatalog) { setFirstCatalogTitle(c, strings.Repeat("t", encyclopediaMaxTitleBytes)) }},
		{name: "title one above bytes", do: func(c *encyclopediaCatalog) {
			setFirstCatalogTitle(c, strings.Repeat("t", encyclopediaMaxTitleBytes+1))
		}, code: "resource_limit:title_bytes"},
		{name: "label exact bytes", do: func(c *encyclopediaCatalog) {
			c.Index.Labels["1033"] = strings.Repeat("l", encyclopediaMaxTitleBytes)
		}},
		{name: "label one above bytes", do: func(c *encyclopediaCatalog) {
			c.Index.Labels["1033"] = strings.Repeat("l", encyclopediaMaxTitleBytes+1)
		}, code: "resource_limit:label_bytes"},
		{name: "image bytes exact", do: func(c *encyclopediaCatalog) {
			image := c.Images["edata:1"]
			image.ByteLength = encyclopediaMaxImageBytes
			c.Images["edata:1"] = image
		}},
		{name: "image bytes above", do: func(c *encyclopediaCatalog) {
			image := c.Images["edata:1"]
			image.ByteLength = encyclopediaMaxImageBytes + 1
			c.Images["edata:1"] = image
		}, code: "resource_limit:image_bytes"},
		{name: "image pixels exact", do: func(c *encyclopediaCatalog) {
			image := c.Images["edata:1"]
			image.Width, image.Height = 4000, 4000
			c.Images["edata:1"] = image
		}},
		{name: "image pixels above", do: func(c *encyclopediaCatalog) {
			image := c.Images["edata:1"]
			image.Width, image.Height = 4001, 4000
			c.Images["edata:1"] = image
		}, code: "resource_limit:image_pixels"},
	} {
		t.Run(test.name, func(t *testing.T) {
			catalog := cloneCatalogForTest(t, base)
			test.do(&catalog)
			err := validateEncyclopediaCatalog(catalog)
			if got := encyclopediaValidationCode(err); got != map[bool]string{true: test.code, false: "ok"}[test.code != ""] {
				t.Fatalf("validation code = %q (%v), want %q", got, err, test.code)
			}
		})
	}
}

func TestEncyclopediaCatalogTopicCountExactAndAboveLimit(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	base, err := parseEncyclopediaCatalog(mustReadTestFile(t, filepath.Join(root, "fixtures", "bundles", "valid", "catalog.json")))
	if err != nil {
		t.Fatal(err)
	}
	for len(base.Topics) < encyclopediaMaxTopics {
		number := uint32(70_000 + len(base.Topics))
		topicID := topicIDForResource(number)
		base.Topics[topicID] = encyclopediaTopic{
			Localized: map[string]encyclopediaLocalizedContent{"1033": {Title: "Synthetic", Body: "Synthetic boundary topic."}},
			SourceRef: "fixture/topic/60007",
		}
		base.Index.TopicIDs = append(base.Index.TopicIDs, topicID)
		base.Bindings = append(base.Bindings, encyclopediaCatalogBinding{Family: "fixture_boundary", DatID: number, Variant: "default", TopicID: topicID})
	}
	if err := validateEncyclopediaCatalog(base); err != nil {
		t.Fatalf("exact topic limit rejected: %v", err)
	}
	above := cloneCatalogForTest(t, base)
	aboveID := topicIDForResource(99_999)
	above.Topics[aboveID] = encyclopediaTopic{Localized: map[string]encyclopediaLocalizedContent{"1033": {Title: "Above", Body: "Above."}}, SourceRef: "fixture/topic/60007"}
	if got := encyclopediaValidationCode(validateEncyclopediaCatalog(above)); got != "resource_limit:topics" {
		t.Fatalf("above-topic validation code = %q", got)
	}
}

func TestEncyclopediaCatalogAggregateImageBytesExactAndAboveLimit(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	base, err := parseEncyclopediaCatalog(mustReadTestFile(t, filepath.Join(root, "fixtures", "bundles", "valid", "catalog.json")))
	if err != nil {
		t.Fatal(err)
	}
	for imageID, image := range base.Images {
		image.ByteLength = encyclopediaMaxImageBytes
		base.Images[imageID] = image
	}
	addSyntheticCatalogImageTopic(&base, 4, encyclopediaMaxImageBytes)
	if err := validateEncyclopediaCatalog(base); err != nil {
		t.Fatalf("exact aggregate image limit rejected: %v", err)
	}
	above := cloneCatalogForTest(t, base)
	addSyntheticCatalogImageTopic(&above, 5, 1)
	if got := encyclopediaValidationCode(validateEncyclopediaCatalog(above)); got != "resource_limit:effective_image_bytes" {
		t.Fatalf("above-aggregate validation code = %q", got)
	}
}

func addSyntheticCatalogImageTopic(catalog *encyclopediaCatalog, number uint32, byteLength uint64) {
	imageID := fmt.Sprintf("edata:%d", number)
	topicID := fmt.Sprintf("original:%d", 80_000+number)
	catalog.Images[imageID] = encyclopediaImage{
		Path: fmt.Sprintf("assets/EDATA.%03d", number), Format: "bmp", ByteLength: byteLength,
		Width: 1, Height: 1, SHA256: strings.Repeat("a", 64), SourceRef: "fixture/image/1",
	}
	catalog.Topics[topicID] = encyclopediaTopic{
		Localized: map[string]encyclopediaLocalizedContent{"1033": {Title: "Synthetic", Body: "Synthetic.", ImageID: &imageID}},
		SourceRef: "fixture/topic/60007",
	}
	catalog.Index.TopicIDs = append(catalog.Index.TopicIDs, topicID)
	catalog.Bindings = append(catalog.Bindings, encyclopediaCatalogBinding{Family: "fixture_boundary", DatID: number, Variant: "default", TopicID: topicID})
}

func TestEncyclopediaJSONBoundsRejectBeforeTypedAllocation(t *testing.T) {
	exactDepth := []byte(strings.Repeat("[", encyclopediaMaxCatalogJSONDepth) + "0" + strings.Repeat("]", encyclopediaMaxCatalogJSONDepth))
	if err := validateRawEncyclopediaJSON(exactDepth, encyclopediaMaxCatalogJSONBytes, encyclopediaMaxCatalogJSONDepth); err != nil {
		t.Fatalf("exact JSON depth rejected: %v", err)
	}
	aboveDepth := []byte(strings.Repeat("[", encyclopediaMaxCatalogJSONDepth+1) + "0" + strings.Repeat("]", encyclopediaMaxCatalogJSONDepth+1))
	if got := encyclopediaValidationCode(validateRawEncyclopediaJSON(aboveDepth, encyclopediaMaxCatalogJSONBytes, encyclopediaMaxCatalogJSONDepth)); got != "resource_limit:json_depth" {
		t.Fatalf("above-depth validation code = %q", got)
	}
	exactBytes := bytes.Repeat([]byte{' '}, encyclopediaMaxCatalogJSONBytes)
	exactBytes[0] = '0'
	if err := validateRawEncyclopediaJSON(exactBytes, encyclopediaMaxCatalogJSONBytes, encyclopediaMaxCatalogJSONDepth); err != nil {
		t.Fatalf("exact JSON byte limit rejected: %v", err)
	}
	aboveBytes := make([]byte, encyclopediaMaxCatalogJSONBytes+1)
	if got := encyclopediaValidationCode(validateRawEncyclopediaJSON(aboveBytes, encyclopediaMaxCatalogJSONBytes, encyclopediaMaxCatalogJSONDepth)); got != "resource_limit:json_bytes" {
		t.Fatalf("above-byte validation code = %q", got)
	}
}

func TestValidateEncyclopediaRuntimeFullyDecodesBMPBeforeAcceptingFacts(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	bundle := filepath.Join(root, "fixtures", "bundles", "valid")
	catalog, manifest, files := loadE37RuntimeBundle(t, bundle)
	corrupt := mustReadTestFile(t, filepath.Join(root, "fixtures", "images", "corrupt-bmp.bmp"))
	files["assets/EDATA.001"] = encyclopediaAssetFactsFromBytes(corrupt, "bmp")
	if got := encyclopediaValidationCode(validateEncyclopediaRuntime(catalog, manifest, files)); got != "invalid_image" {
		t.Fatalf("validation code = %q, want invalid_image", got)
	}
}

func TestValidateEncyclopediaRuntimeRejectsObservedImageFactContradiction(t *testing.T) {
	root := testEncyclopediaFixtureRoot(t)
	bundle := filepath.Join(root, "fixtures", "bundles", "valid")
	catalog, manifest, files := loadE37RuntimeBundle(t, bundle)
	facts := files["assets/EDATA.001"]
	facts.Width++
	files["assets/EDATA.001"] = facts
	if got := encyclopediaValidationCode(validateEncyclopediaRuntime(catalog, manifest, files)); got != "image_facts_mismatch" {
		t.Fatalf("validation code = %q, want image_facts_mismatch", got)
	}
}

func cloneCatalogForTest(t *testing.T, catalog encyclopediaCatalog) encyclopediaCatalog {
	t.Helper()
	data, err := json.Marshal(catalog)
	if err != nil {
		t.Fatal(err)
	}
	var clone encyclopediaCatalog
	if err := json.Unmarshal(data, &clone); err != nil {
		t.Fatal(err)
	}
	return clone
}

func setFirstCatalogBody(catalog *encyclopediaCatalog, value string) {
	for topicID, topic := range catalog.Topics {
		for language, localized := range topic.Localized {
			localized.Body = value
			topic.Localized[language] = localized
			catalog.Topics[topicID] = topic
			return
		}
	}
}

func setFirstCatalogTitle(catalog *encyclopediaCatalog, value string) {
	for topicID, topic := range catalog.Topics {
		for language, localized := range topic.Localized {
			localized.Title = value
			topic.Localized[language] = localized
			catalog.Topics[topicID] = topic
			return
		}
	}
}

func mutateJSONFixture(t *testing.T, original []byte, mutate func(map[string]any)) []byte {
	t.Helper()
	var document map[string]any
	if err := json.Unmarshal(original, &document); err != nil {
		t.Fatal(err)
	}
	mutate(document)
	changed, err := json.Marshal(document)
	if err != nil {
		t.Fatal(err)
	}
	return changed
}

func catalogFixtureTopic(document map[string]any, topicID string) map[string]any {
	return document["topics"].(map[string]any)[topicID].(map[string]any)
}

func catalogFixtureLocalized(document map[string]any, topicID, languageID string) map[string]any {
	return catalogFixtureTopic(document, topicID)["localized"].(map[string]any)[languageID].(map[string]any)
}

func firstManifestSourceRecord(document map[string]any) map[string]any {
	return document["source_records"].(map[string]any)["fixture/category/0x6f"].(map[string]any)
}

func catalogParseError(data []byte) error {
	_, err := parseEncyclopediaCatalog(data)
	return err
}

func manifestParseError(data []byte) error {
	_, err := parseEncyclopediaManifest(data)
	return err
}

func testEncyclopediaFixtureRoot(t *testing.T) string {
	t.Helper()
	root := filepath.Join("..", "..", "tests", "fixtures", "encyclopedia")
	if _, err := os.Stat(filepath.Join(root, "cases.json")); err != nil {
		t.Fatalf("locate E37 fixtures: %v", err)
	}
	return root
}

func loadE37RuntimeBundle(t *testing.T, root string) ([]byte, []byte, map[string]encyclopediaAssetFacts) {
	t.Helper()
	catalog := mustReadTestFile(t, filepath.Join(root, "catalog.json"))
	manifest := mustReadTestFile(t, filepath.Join(root, "manifest.json"))
	files := map[string]encyclopediaAssetFacts{"catalog.json": encyclopediaAssetFactsFromBytes(catalog, "")}
	entries, err := os.ReadDir(filepath.Join(root, "assets"))
	if err != nil {
		t.Fatal(err)
	}
	for _, entry := range entries {
		data := mustReadTestFile(t, filepath.Join(root, "assets", entry.Name()))
		files["assets/"+entry.Name()] = encyclopediaAssetFactsFromBytes(data, "bmp")
	}
	return catalog, manifest, files
}

func mustReadTestFile(t *testing.T, path string) []byte {
	t.Helper()
	data, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	return data
}
