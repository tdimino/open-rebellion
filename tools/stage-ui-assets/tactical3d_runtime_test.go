package main

import (
	"bytes"
	"encoding/binary"
	"encoding/json"
	"io"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestStageTactical3DRuntimeIsDeterministicAndVerifiable(t *testing.T) {
	base := t.TempDir()
	rawRoot := writeSyntheticTacticalRawStore(t, base)

	first, err := stageTactical3DRuntime(base, false, io.Discard)
	if err != nil {
		t.Fatalf("stageTactical3DRuntime() error = %v", err)
	}
	if first.Meshes != tacticalMeshCount || first.Textures != tacticalTextureCount || first.Written == 0 || first.Written+first.Skipped != tacticalMeshCount+tacticalTextureCount {
		t.Fatalf("first summary = %+v", first)
	}
	manifestPath := filepath.Join(rawRoot, "runtime", "manifest.json")
	firstManifest, err := os.ReadFile(manifestPath)
	if err != nil {
		t.Fatal(err)
	}
	if err := verifyTactical3DRuntime(base, io.Discard); err != nil {
		t.Fatalf("verifyTactical3DRuntime() error = %v", err)
	}

	second, err := stageTactical3DRuntime(base, false, io.Discard)
	if err != nil {
		t.Fatal(err)
	}
	if second.Written != 0 || second.Skipped != tacticalMeshCount+tacticalTextureCount {
		t.Fatalf("second summary = %+v", second)
	}
	secondManifest, err := os.ReadFile(manifestPath)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(firstManifest, secondManifest) {
		t.Fatal("repeat conversion changed the runtime manifest")
	}
}

func TestVerifyTactical3DRuntimeRejectsTamperedObject(t *testing.T) {
	base := t.TempDir()
	rawRoot := writeSyntheticTacticalRawStore(t, base)
	if _, err := stageTactical3DRuntime(base, false, io.Discard); err != nil {
		t.Fatal(err)
	}
	manifestBytes, err := os.ReadFile(filepath.Join(rawRoot, "runtime", "manifest.json"))
	if err != nil {
		t.Fatal(err)
	}
	var manifest tactical3DRuntimeManifest
	if err := json.Unmarshal(manifestBytes, &manifest); err != nil {
		t.Fatal(err)
	}
	object := filepath.Join(rawRoot, "runtime", filepath.FromSlash(manifest.Meshes[0].Object))
	if err := os.WriteFile(object, []byte("tampered"), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := verifyTactical3DRuntime(base, io.Discard); err == nil {
		t.Fatal("tampered runtime object passed verification")
	}
}

func TestVerifyTactical3DRuntimeRejectsBrokenRelationshipGraph(t *testing.T) {
	mutations := map[string]func(*tactical3DRuntimeManifest){
		"negative_tail": func(manifest *tactical3DRuntimeManifest) {
			manifest.Textures[27].TrailingBytes = -1
			manifest.Textures[27].TrailingSHA256 = strings.Repeat("0", 64)
		},
		"oversized_tail": func(manifest *tactical3DRuntimeManifest) {
			manifest.Textures[27].TrailingBytes = 1 << 30
			manifest.Textures[27].TrailingSHA256 = strings.Repeat("0", 64)
		},
		"missing_mesh_binding": func(manifest *tactical3DRuntimeManifest) {
			manifest.Meshes[0].TextureBindings = nil
		},
		"missing_named_target": func(manifest *tactical3DRuntimeManifest) {
			manifest.Meshes[0].TextureBindings[0].ResourceName = "MISSING.BMP"
		},
		"missing_same_language_palette": func(manifest *tactical3DRuntimeManifest) {
			for index := range manifest.Textures {
				if manifest.Textures[index].Kind == "palette_rgb24" && manifest.Textures[index].ID == 5531 {
					manifest.Textures[index].Language = 1031
					return
				}
			}
		},
		"ambiguous_casefolded_name": func(manifest *tactical3DRuntimeManifest) {
			for index := range manifest.Textures {
				if manifest.Textures[index].IdentifierKind == "id" && manifest.Textures[index].Kind == "indexed_rle" {
					manifest.Textures[index].IdentifierKind = "name"
					manifest.Textures[index].ID = 0
					manifest.Textures[index].Name = "ship.bmp"
					return
				}
			}
		},
	}
	for name, mutate := range mutations {
		t.Run(name, func(t *testing.T) {
			base := t.TempDir()
			rawRoot := writeSyntheticTacticalRawStore(t, base)
			if _, err := stageTactical3DRuntime(base, false, io.Discard); err != nil {
				t.Fatal(err)
			}
			manifestPath := filepath.Join(rawRoot, "runtime", "manifest.json")
			manifestBytes, err := os.ReadFile(manifestPath)
			if err != nil {
				t.Fatal(err)
			}
			var manifest tactical3DRuntimeManifest
			if err := json.Unmarshal(manifestBytes, &manifest); err != nil {
				t.Fatal(err)
			}
			mutate(&manifest)
			writeJSONFile(t, manifestPath, manifest)
			if err := verifyTactical3DRuntime(base, io.Discard); err == nil {
				t.Fatal("broken runtime relationship graph passed verification")
			}
		})
	}
}

func TestVerifyTactical3DRuntimeRejectsPlanetPaletteBypass(t *testing.T) {
	base := t.TempDir()
	rawRoot := writeSyntheticTacticalRawStore(t, base)
	if _, err := stageTactical3DRuntime(base, false, io.Discard); err != nil {
		t.Fatal(err)
	}
	manifestPath := filepath.Join(rawRoot, "runtime", "manifest.json")
	manifestBytes, err := os.ReadFile(manifestPath)
	if err != nil {
		t.Fatal(err)
	}
	var manifest tactical3DRuntimeManifest
	if err := json.Unmarshal(manifestBytes, &manifest); err != nil {
		t.Fatal(err)
	}
	for index := range manifest.Textures {
		record := &manifest.Textures[index]
		if record.IdentifierKind != "id" || record.ID != 5501 || record.Kind != "indexed_rle" {
			continue
		}
		object, err := os.ReadFile(filepath.Join(rawRoot, "runtime", filepath.FromSlash(record.Object)))
		if err != nil {
			t.Fatal(err)
		}
		binary.LittleEndian.PutUint32(object[20:24], 0)
		binary.LittleEndian.PutUint32(object[24:28], tacticalPaletteRuleCode("battle_active"))
		hash := sha256Hex(object)
		record.PaletteID = 0
		record.PaletteRule = "battle_active"
		record.ObjectSHA256 = hash
		record.Object = "objects/" + hash + ".texture"
		if err := os.WriteFile(filepath.Join(rawRoot, "runtime", filepath.FromSlash(record.Object)), object, 0o644); err != nil {
			t.Fatal(err)
		}
		writeJSONFile(t, manifestPath, manifest)
		if err := verifyTactical3DRuntime(base, io.Discard); err == nil {
			t.Fatal("consistently rehashed planet palette bypass passed verification")
		}
		return
	}
	t.Fatal("synthetic planet texture was not found")
}

func TestStageTactical3DRuntimeRejectsDecodedOutputBudgetBeforeRLEExpansion(t *testing.T) {
	base := t.TempDir()
	rawRoot := writeSyntheticTacticalRawStore(t, base)
	manifestPath := filepath.Join(rawRoot, "manifest.json")
	manifestBytes, err := os.ReadFile(manifestPath)
	if err != nil {
		t.Fatal(err)
	}
	var manifest tactical3DManifest
	if err := json.Unmarshal(manifestBytes, &manifest); err != nil {
		t.Fatal(err)
	}
	largeDeclaration := make([]byte, 10)
	binary.LittleEndian.PutUint32(largeDeclaration[0:4], 4096)
	binary.LittleEndian.PutUint32(largeDeclaration[4:8], 4096)
	largeDeclaration[8] = 0
	largeDeclaration[9] = 1
	hash := sha256Hex(largeDeclaration)
	object := "objects/" + hash + ".bin"
	if err := os.WriteFile(filepath.Join(rawRoot, filepath.FromSlash(object)), largeDeclaration, 0o644); err != nil {
		t.Fatal(err)
	}
	for index := range manifest.Resources {
		if manifest.Resources[index].Type == tacticalTextureResourceType && !isTacticalPaletteResource(manifest.Resources[index]) {
			manifest.Resources[index].Size = len(largeDeclaration)
			manifest.Resources[index].SHA256 = hash
			manifest.Resources[index].Object = object
		}
	}
	writeJSONFile(t, manifestPath, manifest)
	if _, err := stageTactical3DRuntime(base, false, io.Discard); err == nil || !strings.Contains(err.Error(), "runtime objects exceed") {
		t.Fatalf("decoded output budget error = %v", err)
	}
}

func TestAllowedTacticalImageTailIsSourceBound(t *testing.T) {
	record := tactical3DManifestRecord{
		Type:           tacticalTextureResourceType,
		IdentifierKind: "id",
		ID:             4200,
		Language:       1033,
		SHA256:         "753d4144e6c0a3e15b8150fbdf9e264bcdf4e645ae3ee5ce2753f168420e1b7d",
	}
	if !allowedTacticalImageTail(originalTacticalDLLSHA256, record, []byte{2, 0xf9, 4, 0}) {
		t.Fatal("owned original 4200 tail was rejected")
	}
	record.SHA256 = sha256Hex([]byte("different"))
	if allowedTacticalImageTail(originalTacticalDLLSHA256, record, []byte{2, 0xf9, 4, 0}) {
		t.Fatal("different type-303 payload inherited the 4200 tail exception")
	}
}

func TestAllowedDerivedTacticalNormalsIsSourceBound(t *testing.T) {
	record := tactical3DRuntimeMeshRecord{
		ID:           2082,
		Language:     1033,
		SourceSHA256: originalMesh2082SHA256,
	}
	if !allowedDerivedTacticalNormals(originalTacticalDLLSHA256, record) {
		t.Fatal("owned original mesh 2082 was rejected")
	}
	record.ID = 2022
	if allowedDerivedTacticalNormals(originalTacticalDLLSHA256, record) {
		t.Fatal("different mesh inherited the derived-normal exception")
	}
}

func writeSyntheticTacticalRawStore(t *testing.T, base string) string {
	t.Helper()
	rawRoot := filepath.Join(base, "tactical-dll", "TACTICAL3D")
	objects := filepath.Join(rawRoot, "objects")
	if err := os.MkdirAll(objects, 0o755); err != nil {
		t.Fatal(err)
	}
	manifest := tactical3DManifest{
		SchemaVersion:   tactical3DSchemaVersion,
		Source:          "TACTICAL.DLL",
		SourceDLLSHA256: sha256Hex([]byte("synthetic tactical dll")),
	}
	add := func(record tactical3DManifestRecord, data []byte) {
		t.Helper()
		record.Size = len(data)
		record.SHA256 = sha256Hex(data)
		record.Object = "objects/" + record.SHA256 + ".bin"
		manifest.Resources = append(manifest.Resources, record)
		path := filepath.Join(rawRoot, filepath.FromSlash(record.Object))
		if _, err := os.Stat(path); os.IsNotExist(err) {
			if err := os.WriteFile(path, data, 0o644); err != nil {
				t.Fatal(err)
			}
		} else if err != nil {
			t.Fatal(err)
		}
	}
	mesh := buildSyntheticTacticalX(t, true)
	for index := 0; index < tacticalMeshCount; index++ {
		add(tactical3DManifestRecord{
			Type: tacticalMeshResourceType, IdentifierKind: "id", ID: uint32(2000 + index), Language: 1033,
		}, mesh)
	}
	for id := uint32(5531); id <= 5557; id++ {
		palette := make([]byte, 256*3)
		for index := range palette {
			palette[index] = byte(index + int(id))
		}
		add(tactical3DManifestRecord{
			Type: tacticalTextureResourceType, IdentifierKind: "id", ID: id, Language: 1033,
		}, palette)
	}
	image := make([]byte, 8)
	binary.LittleEndian.PutUint32(image[0:4], 1)
	binary.LittleEndian.PutUint32(image[4:8], 1)
	image = append(image, 1, 7)
	add(tactical3DManifestRecord{
		Type: tacticalTextureResourceType, IdentifierKind: "name", Name: "SHIP.BMP", Language: 1033,
	}, image)
	add(tactical3DManifestRecord{
		Type: tacticalTextureResourceType, IdentifierKind: "id", ID: 5501, Language: 1033,
	}, image)
	for index := 0; index < tacticalTextureCount-29; index++ {
		add(tactical3DManifestRecord{
			Type: tacticalTextureResourceType, IdentifierKind: "id", ID: uint32(6000 + index), Language: 1033,
		}, image)
	}
	manifestBytes, err := json.MarshalIndent(manifest, "", "  ")
	if err != nil {
		t.Fatal(err)
	}
	manifestBytes = append(manifestBytes, '\n')
	if err := os.WriteFile(filepath.Join(rawRoot, "manifest.json"), manifestBytes, 0o644); err != nil {
		t.Fatal(err)
	}
	return rawRoot
}

func writeJSONFile(t *testing.T, path string, value any) {
	t.Helper()
	data, err := json.MarshalIndent(value, "", "  ")
	if err != nil {
		t.Fatal(err)
	}
	data = append(data, '\n')
	if err := os.WriteFile(path, data, 0o644); err != nil {
		t.Fatal(err)
	}
}
