package main

import (
	"bytes"
	"encoding/binary"
	"encoding/json"
	"io"
	"os"
	"path/filepath"
	"testing"
)

func testUncompressedX() []byte {
	return append([]byte("xof 0303bin 0032"), 1, 2, 3, 4)
}

func testCompressedX(declared uint32) []byte {
	data := append([]byte("xof 0303bzip0032"), make([]byte, 4)...)
	binary.LittleEndian.PutUint32(data[16:20], declared)
	return append(data, 1, 0, 3, 0, 'C', 'K', 0)
}

func TestStageTactical3DResourcesCreatesContentAddressedManifest(t *testing.T) {
	base := t.TempDir()
	root := filepath.Join(base, "tactical-dll", "TACTICAL3D")
	meshes := []rawResource{{ID: 2560, Language: 1033, Data: testUncompressedX()}}
	textures := []rawResource{
		{ID: 4000, Language: 1033, Data: []byte("fighter")},
		{Name: "EMPIRE_IMPERIAL_1_STAR_DESTROYER_CLOSE.BMP", Named: true, Language: 1033, Data: []byte("capital")},
	}

	summary, err := stageTactical3DResources(root, []byte("test tactical dll"), meshes, textures, 1, 2, false, io.Discard)
	if err != nil {
		t.Fatalf("stageTactical3DResources() error = %v", err)
	}
	if summary.Resources != 3 || summary.Written != 3 || summary.Skipped != 0 {
		t.Fatalf("first summary = %+v", summary)
	}
	manifestBytes, err := os.ReadFile(filepath.Join(root, "manifest.json"))
	if err != nil {
		t.Fatal(err)
	}
	var manifest tactical3DManifest
	if err := json.Unmarshal(manifestBytes, &manifest); err != nil {
		t.Fatal(err)
	}
	if manifest.SourceDLLSHA256 != sha256Hex([]byte("test tactical dll")) || len(manifest.Resources) != 3 {
		t.Fatalf("manifest = %+v", manifest)
	}
	for _, record := range manifest.Resources {
		if record.Object != "objects/"+record.SHA256+".bin" {
			t.Errorf("object path = %q", record.Object)
		}
	}
	if err := verifyTactical3D(base, 1, 2, io.Discard); err != nil {
		t.Fatalf("verifyTactical3D() error = %v", err)
	}

	second, err := stageTactical3DResources(root, []byte("test tactical dll"), meshes, textures, 1, 2, false, io.Discard)
	if err != nil {
		t.Fatal(err)
	}
	if second.Written != 0 || second.Skipped != 3 {
		t.Fatalf("second summary = %+v", second)
	}
	secondManifest, err := os.ReadFile(filepath.Join(root, "manifest.json"))
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(manifestBytes, secondManifest) {
		t.Fatal("identical staging did not reproduce the manifest")
	}

	missingObject := filepath.Join(root, filepath.FromSlash(manifest.Resources[0].Object))
	if err := os.Remove(missingObject); err != nil {
		t.Fatal(err)
	}
	repaired, err := stageTactical3DResources(root, []byte("test tactical dll"), meshes, textures, 1, 2, false, io.Discard)
	if err != nil {
		t.Fatal(err)
	}
	if repaired.Written != 1 || repaired.Skipped != 2 {
		t.Fatalf("repair summary = %+v", repaired)
	}
	if err := verifyTactical3D(base, 1, 2, io.Discard); err != nil {
		t.Fatalf("repaired tactical resources failed verification: %v", err)
	}
}

func TestStageTactical3DResourcesRejectsUnsafeAndAmbiguousNames(t *testing.T) {
	mesh := []rawResource{{ID: 2022, Language: 1033, Data: testUncompressedX()}}
	for _, name := range []string{"../escape.BMP", "bad/name.BMP", "bad\nname.BMP"} {
		textures := []rawResource{{Name: name, Named: true, Language: 1033, Data: []byte("texture")}}
		if _, err := stageTactical3DResources(t.TempDir(), []byte("dll"), mesh, textures, 1, 1, false, io.Discard); err == nil {
			t.Errorf("unsafe name %q was accepted", name)
		}
	}
	textures := []rawResource{
		{Name: "SHIP_CLOSE.BMP", Named: true, Language: 1033, Data: []byte("a")},
		{Name: "ship_close.bmp", Named: true, Language: 1033, Data: []byte("b")},
	}
	if _, err := stageTactical3DResources(t.TempDir(), []byte("dll"), mesh, textures, 1, 2, false, io.Discard); err == nil {
		t.Fatal("ambiguous case-folded names were accepted")
	}
}

func TestStageTactical3DResourcesPreflightsBeforeWriting(t *testing.T) {
	root := filepath.Join(t.TempDir(), "staged")
	meshes := []rawResource{{ID: 2022, Language: 1033, Data: testUncompressedX()}}
	textures := []rawResource{
		{Name: "SHIP_CLOSE.BMP", Named: true, Language: 1033, Data: []byte("a")},
		{Name: "ship_close.bmp", Named: true, Language: 1033, Data: []byte("b")},
	}
	if _, err := stageTactical3DResources(root, []byte("dll"), meshes, textures, 1, 2, false, io.Discard); err == nil {
		t.Fatal("ambiguous case-folded names were accepted")
	}
	if _, err := os.Stat(root); !os.IsNotExist(err) {
		t.Fatalf("preflight failure mutated output directory: %v", err)
	}
}

func TestStageTactical3DResourcesRequiresForceForManifestReplacement(t *testing.T) {
	base := t.TempDir()
	root := filepath.Join(base, "tactical-dll", "TACTICAL3D")
	mesh := []rawResource{{ID: 2022, Language: 1033, Data: testUncompressedX()}}
	originalTexture := []rawResource{{ID: 4000, Language: 1033, Data: []byte("original")}}
	if _, err := stageTactical3DResources(root, []byte("dll-a"), mesh, originalTexture, 1, 1, false, io.Discard); err != nil {
		t.Fatal(err)
	}

	replacementTexture := []rawResource{{ID: 4000, Language: 1033, Data: []byte("replacement")}}
	if _, err := stageTactical3DResources(root, []byte("dll-b"), mesh, replacementTexture, 1, 1, false, io.Discard); err == nil {
		t.Fatal("different tactical manifest was replaced without --force")
	}
	objects, err := os.ReadDir(filepath.Join(root, "objects"))
	if err != nil {
		t.Fatal(err)
	}
	if len(objects) != 2 {
		t.Fatalf("rejected replacement wrote objects: got %d, want 2", len(objects))
	}

	summary, err := stageTactical3DResources(root, []byte("dll-b"), mesh, replacementTexture, 1, 1, true, io.Discard)
	if err != nil {
		t.Fatal(err)
	}
	if summary.Written != 1 || summary.Skipped != 1 {
		t.Fatalf("forced replacement summary = %+v", summary)
	}
	if err := verifyTactical3D(base, 1, 1, io.Discard); err != nil {
		t.Fatalf("forced tactical replacement failed verification: %v", err)
	}
}

func TestVerifyTactical3DDetectsObjectTampering(t *testing.T) {
	base := t.TempDir()
	root := filepath.Join(base, "tactical-dll", "TACTICAL3D")
	mesh := []rawResource{{ID: 2022, Language: 1033, Data: testUncompressedX()}}
	texture := []rawResource{{ID: 4000, Language: 1033, Data: []byte("texture")}}
	if _, err := stageTactical3DResources(root, []byte("dll"), mesh, texture, 1, 1, false, io.Discard); err != nil {
		t.Fatal(err)
	}
	manifestBytes, err := os.ReadFile(filepath.Join(root, "manifest.json"))
	if err != nil {
		t.Fatal(err)
	}
	var manifest tactical3DManifest
	if err := json.Unmarshal(manifestBytes, &manifest); err != nil {
		t.Fatal(err)
	}
	object := filepath.Join(root, filepath.FromSlash(manifest.Resources[0].Object))
	if err := os.WriteFile(object, []byte("tampered"), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := verifyTactical3D(base, 1, 1, io.Discard); err == nil {
		t.Fatal("tampered content-addressed object passed verification")
	}
}

func TestVerifyTactical3DRejectsOversizedObjectBeforeReading(t *testing.T) {
	base := t.TempDir()
	root := filepath.Join(base, "tactical-dll", "TACTICAL3D")
	mesh := []rawResource{{ID: 2022, Language: 1033, Data: testUncompressedX()}}
	texture := []rawResource{{ID: 4000, Language: 1033, Data: []byte("texture")}}
	if _, err := stageTactical3DResources(root, []byte("dll"), mesh, texture, 1, 1, false, io.Discard); err != nil {
		t.Fatal(err)
	}
	manifestBytes, err := os.ReadFile(filepath.Join(root, "manifest.json"))
	if err != nil {
		t.Fatal(err)
	}
	var manifest tactical3DManifest
	if err := json.Unmarshal(manifestBytes, &manifest); err != nil {
		t.Fatal(err)
	}
	object := filepath.Join(root, filepath.FromSlash(manifest.Resources[0].Object))
	if err := os.Truncate(object, maxTacticalRawBytes+1); err != nil {
		t.Fatal(err)
	}
	if _, err := stageTactical3DResources(root, []byte("dll"), mesh, texture, 1, 1, false, io.Discard); err == nil {
		t.Fatal("repeat staging read an oversized content-addressed object")
	}
	if err := verifyTactical3D(base, 1, 1, io.Discard); err == nil {
		t.Fatal("oversized content-addressed object passed verification")
	}
}

func TestTactical3DRejectsOversizedManifestBeforeReading(t *testing.T) {
	base := t.TempDir()
	root := filepath.Join(base, "tactical-dll", "TACTICAL3D")
	if err := os.MkdirAll(root, 0o755); err != nil {
		t.Fatal(err)
	}
	manifestPath := filepath.Join(root, "manifest.json")
	if err := os.WriteFile(manifestPath, []byte("oversized"), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := os.Truncate(manifestPath, maxTacticalManifestBytes+1); err != nil {
		t.Fatal(err)
	}

	mesh := []rawResource{{ID: 2022, Language: 1033, Data: testUncompressedX()}}
	texture := []rawResource{{ID: 4000, Language: 1033, Data: []byte("texture")}}
	if _, err := stageTactical3DResources(root, []byte("dll"), mesh, texture, 1, 1, false, io.Discard); err == nil {
		t.Fatal("repeat staging read an oversized manifest")
	}
	if err := verifyTactical3D(base, 1, 1, io.Discard); err == nil {
		t.Fatal("oversized tactical manifest passed verification")
	}
}

func TestCombinedTacticalResourceBudgetRejectsBeforeTextureResolution(t *testing.T) {
	remaining, err := remainingResourceBudget(5, []rawResource{{Data: []byte("abc")}})
	if err != nil {
		t.Fatal(err)
	}
	if remaining != 2 {
		t.Fatalf("remaining resource budget = %d, want 2", remaining)
	}
	resolveCalls := 0
	_, err = parseMixedRawResources(
		buildRawResourceLimitFixture([]uint32{3}),
		func(_ uint32, size uint32) ([]byte, error) {
			resolveCalls++
			return make([]byte, size), nil
		},
		tacticalTextureResourceType,
		rawResourceLimits{MaxCount: 1, MaxResourceBytes: 4, MaxAggregateBytes: remaining},
	)
	if err == nil {
		t.Fatal("combined mesh and texture budget overflow was accepted")
	}
	if resolveCalls != 0 {
		t.Fatalf("over-budget texture reached resolver %d times", resolveCalls)
	}
}

func TestReadRegularFileBoundedRejectsOversizedInput(t *testing.T) {
	path := filepath.Join(t.TempDir(), "oversized.bin")
	if err := os.WriteFile(path, []byte("12345"), 0o600); err != nil {
		t.Fatal(err)
	}
	if _, err := readRegularFileBounded(path, 4); err == nil {
		t.Fatal("oversized source file was read")
	}
}

func TestValidateTacticalRawResourceBoundsCompressedMesh(t *testing.T) {
	valid := rawResource{ID: 2560, Language: 1033, Data: testCompressedX(4096)}
	if err := validateTacticalRawResource(tacticalMeshResourceType, valid); err != nil {
		t.Fatalf("valid compressed header rejected: %v", err)
	}
	invalid := valid
	invalid.Data = testCompressedX(maxTacticalDecodedMeshBytes + 1)
	if err := validateTacticalRawResource(tacticalMeshResourceType, invalid); err == nil {
		t.Fatal("oversized declared mesh was accepted")
	}
}
