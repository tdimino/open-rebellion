package main

import (
	"bytes"
	"encoding/json"
	"encoding/xml"
	"fmt"
	"io"
	"math"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"strings"
)

const assimpComparisonTolerance = 0.00001

type assimpXMLDump struct {
	Scene assimpXMLScene `xml:"Scene"`
}

type assimpXMLScene struct {
	Materials []assimpXMLMaterial `xml:"MaterialList>Material"`
	Meshes    []assimpXMLMesh     `xml:"MeshList>Mesh"`
}

type assimpXMLMaterial struct {
	Properties []assimpXMLProperty `xml:"MatPropertyList>MatProperty"`
}

type assimpXMLProperty struct {
	Key   string `xml:"key,attr"`
	Type  string `xml:"type,attr"`
	Value string `xml:",chardata"`
}

type assimpXMLMesh struct {
	Types         string             `xml:"types,attr"`
	MaterialIndex int                `xml:"material_index,attr"`
	Faces         []assimpXMLFace    `xml:"FaceList>Face"`
	Positions     assimpXMLNumbers   `xml:"Positions"`
	Normals       assimpXMLNumbers   `xml:"Normals"`
	TextureCoords []assimpXMLNumbers `xml:"TextureCoords"`
}

type assimpXMLFace struct {
	Count int    `xml:"num,attr"`
	Value string `xml:",chardata"`
}

type assimpXMLNumbers struct {
	Count      int    `xml:"num,attr"`
	Components int    `xml:"num_components,attr"`
	Value      string `xml:",chardata"`
}

func verifyTactical3DWithAssimp(outputDir, assimpPath string, stdout io.Writer) error {
	if assimpPath == "" {
		return fmt.Errorf("Assimp executable path is empty")
	}
	info, err := os.Stat(assimpPath)
	if err != nil || !info.Mode().IsRegular() || info.Mode()&0o111 == 0 {
		return fmt.Errorf("Assimp path is not an executable regular file")
	}
	if err := verifyTactical3D(outputDir, tacticalMeshCount, tacticalTextureCount, io.Discard); err != nil {
		return err
	}
	if err := verifyTactical3DRuntime(outputDir, io.Discard); err != nil {
		return err
	}
	rawRoot := filepath.Join(outputDir, "tactical-dll", "TACTICAL3D")
	rawManifestBytes, err := readRegularFileBounded(filepath.Join(rawRoot, "manifest.json"), maxTacticalManifestBytes)
	if err != nil {
		return err
	}
	rawManifest, err := decodeTactical3DManifest(rawManifestBytes)
	if err != nil {
		return err
	}
	if rawManifest.SourceDLLSHA256 != originalTacticalDLLSHA256 {
		return fmt.Errorf("Assimp oracle requires the audited original TACTICAL.DLL")
	}
	runtimeManifest, err := readTactical3DRuntimeManifest(filepath.Join(rawRoot, "runtime", "manifest.json"))
	if err != nil {
		return err
	}
	if runtimeManifest.SourceDLLSHA256 != rawManifest.SourceDLLSHA256 || runtimeManifest.SourceManifestSHA256 != sha256Hex(rawManifestBytes) {
		return fmt.Errorf("tactical runtime manifest is not bound to the raw manifest")
	}
	runtimeMeshes := make(map[string]tactical3DRuntimeMeshRecord, len(runtimeManifest.Meshes))
	for _, mesh := range runtimeManifest.Meshes {
		runtimeMeshes[fmt.Sprintf("%d:%d", mesh.ID, mesh.Language)] = mesh
	}
	tempDir, err := os.MkdirTemp("", "open-rebellion-assimp-oracle-")
	if err != nil {
		return fmt.Errorf("create Assimp oracle directory: %w", err)
	}
	defer os.RemoveAll(tempDir)
	checked := 0
	for _, source := range rawManifest.Resources {
		if source.Type != tacticalMeshResourceType {
			continue
		}
		runtimeRecord, exists := runtimeMeshes[fmt.Sprintf("%d:%d", source.ID, source.Language)]
		if !exists || runtimeRecord.SourceSHA256 != source.SHA256 {
			return fmt.Errorf("type-301 mesh %d is not source-bound in the runtime manifest", source.ID)
		}
		data, err := readRegularFileBounded(filepath.Join(rawRoot, filepath.FromSlash(source.Object)), maxTacticalRawBytes)
		if err != nil {
			return fmt.Errorf("read type-301 mesh %d: %w", source.ID, err)
		}
		if sha256Hex(data) != source.SHA256 {
			return fmt.Errorf("type-301 mesh %d failed source verification", source.ID)
		}
		mesh, err := decodeTacticalBinaryX(data)
		if err != nil {
			return fmt.Errorf("decode type-301 mesh %d: %w", source.ID, err)
		}
		chunks, err := buildTacticalMeshChunks(mesh)
		if err != nil {
			return fmt.Errorf("build type-301 mesh %d: %w", source.ID, err)
		}
		if err := verifyRetainedTacticalMesh(filepath.Join(rawRoot, "runtime"), runtimeRecord, encodeTacticalMeshObject(mesh, chunks)); err != nil {
			return fmt.Errorf("bind type-301 mesh %d to retained runtime object: %w", source.ID, err)
		}
		dumpPath := filepath.Join(tempDir, fmt.Sprintf("%d-%d.assxml", source.ID, source.Language))
		command := exec.Command(assimpPath, "dump", filepath.Join(rawRoot, filepath.FromSlash(source.Object)), dumpPath, "-r")
		if output, err := command.CombinedOutput(); err != nil {
			return fmt.Errorf("Assimp raw dump for type-301 mesh %d failed: %w: %s", source.ID, err, strings.TrimSpace(string(output)))
		}
		dumpBytes, err := readRegularFileBounded(dumpPath, maxTacticalRawBytes)
		if err != nil {
			return fmt.Errorf("read Assimp dump for type-301 mesh %d: %w", source.ID, err)
		}
		if err := compareTacticalMeshToAssimp(mesh, chunks, dumpBytes); err != nil {
			return fmt.Errorf("type-301 mesh %d Assimp mismatch: %w", source.ID, err)
		}
		checked++
	}
	if checked != tacticalMeshCount {
		return fmt.Errorf("Assimp checked %d meshes, expected %d", checked, tacticalMeshCount)
	}
	fmt.Fprintf(stdout, "Verified %d tactical meshes against Assimp raw dumps\n", checked)
	return nil
}

func verifyRetainedTacticalMesh(root string, record tactical3DRuntimeMeshRecord, expected []byte) error {
	expectedHash := sha256Hex(expected)
	expectedObject := "objects/" + expectedHash + ".mesh"
	if record.ObjectSHA256 != expectedHash || record.Object != expectedObject {
		return fmt.Errorf("runtime manifest does not identify the source-derived mesh object")
	}
	actual, err := readRegularFileBounded(filepath.Join(root, filepath.FromSlash(record.Object)), maxTacticalRawBytes)
	if err != nil {
		return err
	}
	if !bytes.Equal(actual, expected) {
		return fmt.Errorf("retained runtime mesh differs from the source-derived object")
	}
	return nil
}

func readTactical3DRuntimeManifest(path string) (tactical3DRuntimeManifest, error) {
	data, err := readRegularFileBounded(path, maxTacticalManifestBytes)
	if err != nil {
		return tactical3DRuntimeManifest{}, err
	}
	var manifest tactical3DRuntimeManifest
	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	if err := decoder.Decode(&manifest); err != nil {
		return tactical3DRuntimeManifest{}, fmt.Errorf("decode tactical runtime manifest: %w", err)
	}
	if err := decoder.Decode(&struct{}{}); err != io.EOF {
		return tactical3DRuntimeManifest{}, fmt.Errorf("tactical runtime manifest has trailing JSON data")
	}
	return manifest, nil
}

func compareTacticalMeshToAssimp(mesh decodedTacticalMesh, chunks []tacticalMeshChunk, data []byte) error {
	var dump assimpXMLDump
	if err := xml.Unmarshal(data, &dump); err != nil {
		return fmt.Errorf("decode ASSXML: %w", err)
	}
	if len(dump.Scene.Meshes) != 1 || len(chunks) != 1 {
		return fmt.Errorf("got %d Assimp meshes and %d runtime chunks; expected one each", len(dump.Scene.Meshes), len(chunks))
	}
	assimpMesh := dump.Scene.Meshes[0]
	chunk := chunks[0]
	if strings.TrimSpace(assimpMesh.Types) != "triangles" || assimpMesh.MaterialIndex != int(chunk.Material) {
		return fmt.Errorf("primitive or material identity differs")
	}
	if len(assimpMesh.Faces)*3 != len(chunk.Indices) {
		return fmt.Errorf("face count differs")
	}
	for faceIndex, face := range assimpMesh.Faces {
		indices, err := parseAssimpIntegers(face.Value, face.Count)
		if err != nil || face.Count != 3 {
			return fmt.Errorf("face %d is invalid", faceIndex)
		}
		base := faceIndex * 3
		if indices[0] != int(chunk.Indices[base+2]) || indices[1] != int(chunk.Indices[base+1]) || indices[2] != int(chunk.Indices[base]) {
			return fmt.Errorf("face %d winding or connectivity differs", faceIndex)
		}
	}
	positions, err := parseAssimpNumbers(assimpMesh.Positions, 3)
	if err != nil {
		return fmt.Errorf("positions: %w", err)
	}
	if len(positions) != len(chunk.Vertices)*3 {
		return fmt.Errorf("position count differs")
	}
	for index, vertex := range chunk.Vertices {
		expected := [3]float64{float64(vertex.Position.X), float64(vertex.Position.Y), -float64(vertex.Position.Z)}
		for component := 0; component < 3; component++ {
			if !oracleClose(positions[index*3+component], expected[component]) {
				return fmt.Errorf("position %d component %d differs", index, component)
			}
		}
	}
	if len(mesh.Normals) == 0 {
		if assimpMesh.Normals.Count != 0 || strings.TrimSpace(assimpMesh.Normals.Value) != "" {
			return fmt.Errorf("Assimp supplied normals absent from the source")
		}
	} else {
		normals, err := parseAssimpNumbers(assimpMesh.Normals, 3)
		if err != nil {
			return fmt.Errorf("normals: %w", err)
		}
		if len(normals) != len(chunk.Vertices)*3 {
			return fmt.Errorf("normal count differs")
		}
		for index, vertex := range chunk.Vertices {
			expected := [3]float64{float64(vertex.Normal.X), float64(vertex.Normal.Y), -float64(vertex.Normal.Z)}
			for component := 0; component < 3; component++ {
				if !oracleClose(normals[index*3+component], expected[component]) {
					return fmt.Errorf("normal %d component %d differs", index, component)
				}
			}
		}
	}
	if len(assimpMesh.TextureCoords) != 1 {
		return fmt.Errorf("got %d texture-coordinate sets", len(assimpMesh.TextureCoords))
	}
	uvs, err := parseAssimpNumbers(assimpMesh.TextureCoords[0], 2)
	if err != nil {
		return fmt.Errorf("texture coordinates: %w", err)
	}
	if len(uvs) != len(chunk.Vertices)*2 {
		return fmt.Errorf("texture-coordinate count differs")
	}
	for index, vertex := range chunk.Vertices {
		if !oracleClose(uvs[index*2], float64(vertex.UV.U)) || !oracleClose(uvs[index*2+1], 1-float64(vertex.UV.V)) {
			return fmt.Errorf("texture coordinate %d differs", index)
		}
	}
	if len(dump.Scene.Materials) != len(mesh.Materials) {
		return fmt.Errorf("material count differs")
	}
	for index := range mesh.Materials {
		if err := compareTacticalMaterialToAssimp(mesh.Materials[index], dump.Scene.Materials[index]); err != nil {
			return fmt.Errorf("material %d: %w", index, err)
		}
	}
	return nil
}

func compareTacticalMaterialToAssimp(material tacticalMaterial, assimp assimpXMLMaterial) error {
	properties := make(map[string]assimpXMLProperty, len(assimp.Properties))
	for _, property := range assimp.Properties {
		properties[property.Key] = property
	}
	checks := []struct {
		key      string
		expected []float32
	}{
		{"$clr.diffuse", material.Diffuse[:]},
		{"$clr.specular", material.Specular[:]},
		{"$clr.emissive", material.Emissive[:]},
		{"$mat.shininess", []float32{material.SpecularExponent}},
	}
	for _, check := range checks {
		property, exists := properties[check.key]
		if !exists {
			return fmt.Errorf("missing %s", check.key)
		}
		values, err := parseAssimpFloatText(property.Value, len(check.expected))
		if err != nil {
			return fmt.Errorf("%s: %w", check.key, err)
		}
		for index, expected := range check.expected {
			if !oracleClose(values[index], float64(expected)) {
				return fmt.Errorf("%s component %d differs", check.key, index)
			}
		}
	}
	texture := ""
	if property, exists := properties["$tex.file"]; exists {
		texture = strings.Trim(strings.TrimSpace(property.Value), "\"")
	}
	if texture != material.Texture {
		return fmt.Errorf("texture filename differs")
	}
	return nil
}

func parseAssimpNumbers(block assimpXMLNumbers, components int) ([]float64, error) {
	if block.Components != components || block.Count < 0 {
		return nil, fmt.Errorf("invalid numeric block metadata")
	}
	return parseAssimpFloatText(block.Value, block.Count*components)
}

func parseAssimpFloatText(value string, expected int) ([]float64, error) {
	fields := strings.Fields(value)
	if len(fields) != expected {
		return nil, fmt.Errorf("got %d values, expected %d", len(fields), expected)
	}
	values := make([]float64, len(fields))
	for index, field := range fields {
		parsed, err := strconv.ParseFloat(field, 64)
		if err != nil || math.IsNaN(parsed) || math.IsInf(parsed, 0) {
			return nil, fmt.Errorf("invalid numeric value %q", field)
		}
		values[index] = parsed
	}
	return values, nil
}

func parseAssimpIntegers(value string, expected int) ([]int, error) {
	fields := strings.Fields(value)
	if len(fields) != expected || expected < 0 {
		return nil, fmt.Errorf("got %d indices, expected %d", len(fields), expected)
	}
	values := make([]int, len(fields))
	for index, field := range fields {
		parsed, err := strconv.Atoi(field)
		if err != nil || parsed < 0 {
			return nil, fmt.Errorf("invalid index %q", field)
		}
		values[index] = parsed
	}
	return values, nil
}

func oracleClose(actual, expected float64) bool {
	return math.Abs(actual-expected) <= assimpComparisonTolerance
}
