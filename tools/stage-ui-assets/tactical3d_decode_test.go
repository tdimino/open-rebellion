package main

import (
	"encoding/binary"
	"math"
	"testing"
)

func TestDecodeTacticalBinaryXAndRuntimeMeshObject(t *testing.T) {
	data := buildSyntheticTacticalX(t, true)
	mesh, err := decodeTacticalBinaryX(data)
	if err != nil {
		t.Fatalf("decodeTacticalBinaryX() error = %v", err)
	}
	if mesh.Version != "0303" || len(mesh.Positions) != 3 || len(mesh.Faces) != 1 || len(mesh.Normals) != 1 || len(mesh.UVs) != 3 || len(mesh.Materials) != 1 {
		t.Fatalf("decoded mesh = %+v", mesh)
	}
	if mesh.Materials[0].Texture != "ship.bmp" || len(mesh.FaceMaterials) != 1 || mesh.FaceMaterials[0] != 0 {
		t.Fatalf("decoded material = %+v; face materials = %v", mesh.Materials[0], mesh.FaceMaterials)
	}
	chunks, err := buildTacticalMeshChunks(mesh)
	if err != nil {
		t.Fatalf("buildTacticalMeshChunks() error = %v", err)
	}
	if len(chunks) != 1 || len(chunks[0].Vertices) != 3 || len(chunks[0].Indices) != 3 {
		t.Fatalf("chunks = %+v", chunks)
	}
	object := encodeTacticalMeshObject(mesh, chunks)
	record := tactical3DRuntimeMeshRecord{
		Vertices: 3, Faces: 1, Triangles: 1, Chunks: 1, Materials: 1,
		Normals:   "source",
		BoundsMin: [3]float32{0, 0, 0},
		BoundsMax: [3]float32{1, 1, 0},
		TextureBindings: []tactical3DTextureBinding{{
			Material: 0, XFilename: "ship.bmp",
		}},
	}
	if err := validateTacticalMeshObject(object, record); err != nil {
		t.Fatalf("validateTacticalMeshObject() error = %v", err)
	}
}

func TestDecodeTacticalBinaryXDerivesMissingNormals(t *testing.T) {
	mesh, err := decodeTacticalBinaryX(buildSyntheticTacticalX(t, false))
	if err != nil {
		t.Fatal(err)
	}
	if len(mesh.Normals) != 0 {
		t.Fatalf("source normals = %v, want none", mesh.Normals)
	}
	chunks, err := buildTacticalMeshChunks(mesh)
	if err != nil {
		t.Fatal(err)
	}
	normal := chunks[0].Vertices[0].Normal
	if math.Abs(float64(normal.Z-1)) > 1e-6 || normal.X != 0 || normal.Y != 0 {
		t.Fatalf("derived normal = %+v, want +Z", normal)
	}
}

func TestDecodeTacticalBinaryXRejectsMalformedData(t *testing.T) {
	valid := buildSyntheticTacticalX(t, true)
	for name, mutate := range map[string]func([]byte) []byte{
		"header": func(data []byte) []byte {
			data[8] = 't'
			return data
		},
		"truncated": func(data []byte) []byte { return data[:len(data)-1] },
		"non_finite": func(data []byte) []byte {
			needle := math.Float32bits(1)
			for offset := 16; offset+4 <= len(data); offset++ {
				if binary.LittleEndian.Uint32(data[offset:offset+4]) == needle {
					binary.LittleEndian.PutUint32(data[offset:offset+4], math.Float32bits(float32(math.Inf(1))))
					break
				}
			}
			return data
		},
	} {
		t.Run(name, func(t *testing.T) {
			data := mutate(append([]byte(nil), valid...))
			if _, err := decodeTacticalBinaryX(data); err == nil {
				t.Fatal("malformed binary X data was accepted")
			}
		})
	}
}

func TestDecodeTacticalIndexedImageAndPaletteRules(t *testing.T) {
	data := make([]byte, 8)
	binary.LittleEndian.PutUint32(data[0:4], 256)
	binary.LittleEndian.PutUint32(data[4:8], 1)
	data = append(data, 0, 7, 2, 9)
	image, err := decodeTacticalIndexedImage(data, 0, "battle_active")
	if err != nil {
		t.Fatal(err)
	}
	if len(image.Pixels) != 256 || image.Pixels[0] != 7 || image.Pixels[255] != 7 || len(image.TrailingBytes) != 2 {
		t.Fatalf("decoded image = %+v", image)
	}
	object := encodeTacticalIndexObject(image)
	record := tactical3DRuntimeTextureRecord{
		Kind: "indexed_rle", Width: 256, Height: 1, PaletteRule: "battle_active", TrailingBytes: 2, TrailingSHA256: sha256Hex(image.TrailingBytes),
	}
	if err := validateTacticalTextureObject(object, record); err != nil {
		t.Fatalf("validateTacticalTextureObject() error = %v", err)
	}

	planet := tactical3DManifestRecord{Type: tacticalTextureResourceType, IdentifierKind: "id", ID: 5504}
	paletteID, rule, err := tacticalPaletteBinding(planet)
	if err != nil || paletteID != 5534 || rule != "planet_pair" {
		t.Fatalf("planet palette = %d %q %v", paletteID, rule, err)
	}
	ship := tactical3DManifestRecord{Type: tacticalTextureResourceType, IdentifierKind: "name", Name: "SHIP.BMP"}
	paletteID, rule, err = tacticalPaletteBinding(ship)
	if err != nil || paletteID != 0 || rule != "battle_active" {
		t.Fatalf("ship palette = %d %q %v", paletteID, rule, err)
	}
}

func TestDecodeTacticalIndexedImageRejectsRunBounds(t *testing.T) {
	data := make([]byte, 8)
	binary.LittleEndian.PutUint32(data[0:4], 2)
	binary.LittleEndian.PutUint32(data[4:8], 2)
	if _, err := decodeTacticalIndexedImage(append(append([]byte(nil), data...), 5, 1), 0, "battle_active"); err == nil {
		t.Fatal("overshooting RLE run was accepted")
	}
	if _, err := decodeTacticalIndexedImage(append(append([]byte(nil), data...), 2, 1), 0, "battle_active"); err == nil {
		t.Fatal("undershooting RLE stream was accepted")
	}
}

func buildSyntheticTacticalX(t *testing.T, normals bool) []byte {
	t.Helper()
	data := []byte("xof 0303bin 0032")
	data = appendXName(data, "Header")
	data = appendXToken(data, xTokenOpenBrace)
	data = appendXIntegers(data, 1, 0, 0)
	data = appendXToken(data, xTokenCloseBrace)
	data = appendXName(data, "Mesh")
	data = appendXToken(data, xTokenOpenBrace)
	data = appendXIntegers(data, 3)
	data = appendXFloats(data, 0, 0, 0, 1, 0, 0, 0, 1, 0)
	data = appendXIntegers(data, 1, 3, 0, 1, 2)
	data = appendXName(data, "MeshMaterialList")
	data = appendXToken(data, xTokenOpenBrace)
	data = appendXIntegers(data, 1, 1, 0)
	data = appendXName(data, "Material")
	data = appendXToken(data, xTokenOpenBrace)
	data = appendXFloats(data, 1, 1, 1, 1, 15, 1, 1, 1, 0, 0, 0)
	data = appendXName(data, "TextureFilename")
	data = appendXToken(data, xTokenOpenBrace)
	data = appendXString(data, "ship.bmp")
	data = appendXToken(data, xTokenCloseBrace)
	data = appendXToken(data, xTokenCloseBrace)
	data = appendXToken(data, xTokenCloseBrace)
	if normals {
		data = appendXName(data, "MeshNormals")
		data = appendXToken(data, xTokenOpenBrace)
		data = appendXIntegers(data, 1)
		data = appendXFloats(data, 0, 0, 1)
		data = appendXIntegers(data, 1, 3, 0, 0, 0)
		data = appendXToken(data, xTokenCloseBrace)
	}
	data = appendXName(data, "MeshTextureCoords")
	data = appendXToken(data, xTokenOpenBrace)
	data = appendXIntegers(data, 3)
	data = appendXFloats(data, 0, 0, 1, 0, 0, 1)
	data = appendXToken(data, xTokenCloseBrace)
	data = appendXToken(data, xTokenCloseBrace)
	return data
}

func appendXToken(data []byte, token uint16) []byte {
	return binary.LittleEndian.AppendUint16(data, token)
}

func appendXName(data []byte, value string) []byte {
	data = appendXToken(data, xTokenName)
	data = binary.LittleEndian.AppendUint32(data, uint32(len(value)))
	return append(data, value...)
}

func appendXString(data []byte, value string) []byte {
	data = appendXToken(data, xTokenString)
	data = binary.LittleEndian.AppendUint32(data, uint32(len(value)))
	data = append(data, value...)
	return appendXToken(data, xTokenSemicolon)
}

func appendXIntegers(data []byte, values ...uint32) []byte {
	data = appendXToken(data, xTokenIntegerList)
	data = binary.LittleEndian.AppendUint32(data, uint32(len(values)))
	for _, value := range values {
		data = binary.LittleEndian.AppendUint32(data, value)
	}
	return data
}

func appendXFloats(data []byte, values ...float32) []byte {
	data = appendXToken(data, xTokenFloatList)
	data = binary.LittleEndian.AppendUint32(data, uint32(len(values)))
	for _, value := range values {
		data = binary.LittleEndian.AppendUint32(data, math.Float32bits(value))
	}
	return data
}
