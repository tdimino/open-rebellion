package main

import (
	"bytes"
	"os"
	"path/filepath"
	"testing"
)

func TestCompareTacticalMeshToAssimpChecksDecodedValues(t *testing.T) {
	mesh, err := decodeTacticalBinaryX(buildSyntheticTacticalX(t, true))
	if err != nil {
		t.Fatal(err)
	}
	chunks, err := buildTacticalMeshChunks(mesh)
	if err != nil {
		t.Fatal(err)
	}
	dump := []byte(`<?xml version="1.0" encoding="utf-8"?>
<ASSIMP format_id="1"><Scene>
<MaterialList num="1"><Material><MatPropertyList num="5">
<MatProperty key="$clr.diffuse" type="float">1 1 1 1</MatProperty>
<MatProperty key="$clr.specular" type="float">1 1 1</MatProperty>
<MatProperty key="$clr.emissive" type="float">0 0 0</MatProperty>
<MatProperty key="$mat.shininess" type="float">15</MatProperty>
<MatProperty key="$tex.file" type="string">"ship.bmp"</MatProperty>
</MatPropertyList></Material></MaterialList>
<MeshList num="1"><Mesh types=" triangles " material_index="0">
<FaceList num="1"><Face num="3">2 1 0</Face></FaceList>
<Positions num="3" num_components="3">0 0 0 1 0 0 0 1 0</Positions>
<Normals num="3" num_components="3">0 0 -1 0 0 -1 0 0 -1</Normals>
<TextureCoords num="3" num_components="2">0 1 1 1 0 0</TextureCoords>
</Mesh></MeshList></Scene></ASSIMP>`)
	if err := compareTacticalMeshToAssimp(mesh, chunks, dump); err != nil {
		t.Fatalf("matching Assimp dump was rejected: %v", err)
	}
	mutated := bytes.Replace(dump, []byte("1 0 0 0 1 0"), []byte("2 0 0 0 1 0"), 1)
	if err := compareTacticalMeshToAssimp(mesh, chunks, mutated); err == nil {
		t.Fatal("mismatched Assimp positions were accepted")
	}
	badConnectivity := append([]tacticalMeshChunk(nil), chunks...)
	badConnectivity[0].Indices = append([]uint16(nil), chunks[0].Indices...)
	badConnectivity[0].Indices[2] = 1
	if err := compareTacticalMeshToAssimp(mesh, badConnectivity, dump); err == nil {
		t.Fatal("mismatched runtime connectivity was accepted")
	}
}

func TestVerifyRetainedTacticalMeshBindsSourceDerivedBytes(t *testing.T) {
	root := t.TempDir()
	if err := os.Mkdir(filepath.Join(root, "objects"), 0o755); err != nil {
		t.Fatal(err)
	}
	expected := []byte("source-derived-runtime-mesh")
	hash := sha256Hex(expected)
	record := tactical3DRuntimeMeshRecord{
		ObjectSHA256: hash,
		Object:       "objects/" + hash + ".mesh",
	}
	if err := os.WriteFile(filepath.Join(root, filepath.FromSlash(record.Object)), expected, 0o644); err != nil {
		t.Fatal(err)
	}
	if err := verifyRetainedTacticalMesh(root, record, expected); err != nil {
		t.Fatalf("source-derived runtime mesh was rejected: %v", err)
	}

	mutated := []byte("consistently-rehashed-mutated-mesh")
	mutatedHash := sha256Hex(mutated)
	record.ObjectSHA256 = mutatedHash
	record.Object = "objects/" + mutatedHash + ".mesh"
	if err := os.WriteFile(filepath.Join(root, filepath.FromSlash(record.Object)), mutated, 0o644); err != nil {
		t.Fatal(err)
	}
	if err := verifyRetainedTacticalMesh(root, record, expected); err == nil {
		t.Fatal("consistently rehashed runtime mesh escaped the source-derived binding")
	}
}
