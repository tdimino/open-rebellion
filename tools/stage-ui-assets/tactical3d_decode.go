package main

import (
	"encoding/binary"
	"fmt"
	"math"
	"strings"
	"unicode"
)

const (
	xTokenName        = uint16(1)
	xTokenString      = uint16(2)
	xTokenInteger     = uint16(3)
	xTokenGUID        = uint16(5)
	xTokenIntegerList = uint16(6)
	xTokenFloatList   = uint16(7)
	xTokenOpenBrace   = uint16(10)
	xTokenCloseBrace  = uint16(11)
	xTokenComma       = uint16(19)
	xTokenSemicolon   = uint16(20)

	maxXTokenStringBytes = 4096
	maxXVertices         = 1 << 20
	maxXFaces            = 1 << 20
	maxXFaceCorners      = 1 << 24
	maxTacticalPixels    = 4096 * 4096
)

type tacticalVec3 struct {
	X float32
	Y float32
	Z float32
}

type tacticalVec2 struct {
	U float32
	V float32
}

type tacticalMaterial struct {
	Diffuse          [4]float32
	SpecularExponent float32
	Specular         [3]float32
	Emissive         [3]float32
	Texture          string
}

type decodedTacticalMesh struct {
	Version       string
	Positions     []tacticalVec3
	Faces         [][]uint32
	Normals       []tacticalVec3
	NormalFaces   [][]uint32
	UVs           []tacticalVec2
	Materials     []tacticalMaterial
	FaceMaterials []uint32
	BoundsMin     tacticalVec3
	BoundsMax     tacticalVec3
}

type tacticalMeshVertex struct {
	Position tacticalVec3
	Normal   tacticalVec3
	UV       tacticalVec2
}

type tacticalMeshChunk struct {
	Material uint32
	Vertices []tacticalMeshVertex
	Indices  []uint16
}

type tacticalIndexedImage struct {
	Width         uint32
	Height        uint32
	PaletteID     uint32
	PaletteRule   string
	Pixels        []byte
	TrailingBytes []byte
}

type xBinaryToken struct {
	kind   uint16
	text   string
	ints   []uint32
	floats []float32
}

type xBinaryReader struct {
	data   []byte
	cursor int
}

func decodeTacticalBinaryX(data []byte) (decodedTacticalMesh, error) {
	if len(data) < 16 {
		return decodedTacticalMesh{}, fmt.Errorf("binary X file is shorter than its header")
	}
	header := string(data[:16])
	if header != "xof 0302bin 0032" && header != "xof 0303bin 0032" {
		return decodedTacticalMesh{}, fmt.Errorf("unsupported binary X header %q", header)
	}
	reader := xBinaryReader{data: data, cursor: 16}
	if err := reader.expectObject("Header"); err != nil {
		return decodedTacticalMesh{}, err
	}
	headerValues, err := reader.expectIntegers()
	if err != nil {
		return decodedTacticalMesh{}, fmt.Errorf("Header: %w", err)
	}
	if len(headerValues) != 3 || headerValues[0] != 1 || headerValues[1] != 0 || headerValues[2] != 0 {
		return decodedTacticalMesh{}, fmt.Errorf("Header: unsupported values %v", headerValues)
	}
	if err := reader.expect(xTokenCloseBrace); err != nil {
		return decodedTacticalMesh{}, fmt.Errorf("Header: %w", err)
	}
	if err := reader.expectObject("Mesh"); err != nil {
		return decodedTacticalMesh{}, err
	}

	vertexCountValues, err := reader.expectIntegers()
	if err != nil {
		return decodedTacticalMesh{}, fmt.Errorf("Mesh vertices: %w", err)
	}
	if len(vertexCountValues) != 1 || vertexCountValues[0] == 0 || vertexCountValues[0] > maxXVertices {
		return decodedTacticalMesh{}, fmt.Errorf("Mesh: invalid vertex count %v", vertexCountValues)
	}
	vertexCount := int(vertexCountValues[0])
	positionValues, err := reader.expectFloats()
	if err != nil {
		return decodedTacticalMesh{}, fmt.Errorf("Mesh positions: %w", err)
	}
	if len(positionValues) != vertexCount*3 {
		return decodedTacticalMesh{}, fmt.Errorf("Mesh: got %d position values for %d vertices", len(positionValues), vertexCount)
	}
	mesh := decodedTacticalMesh{Version: header[4:8], Positions: make([]tacticalVec3, vertexCount)}
	for index := range mesh.Positions {
		mesh.Positions[index] = tacticalVec3{
			X: positionValues[index*3],
			Y: positionValues[index*3+1],
			Z: positionValues[index*3+2],
		}
		if !finiteVec3(mesh.Positions[index]) {
			return decodedTacticalMesh{}, fmt.Errorf("Mesh: position %d is not finite", index)
		}
	}
	mesh.BoundsMin, mesh.BoundsMax = tacticalBounds(mesh.Positions)

	faceValues, err := reader.expectIntegers()
	if err != nil {
		return decodedTacticalMesh{}, fmt.Errorf("Mesh faces: %w", err)
	}
	mesh.Faces, err = decodeTacticalFaces(faceValues, uint32(vertexCount), "Mesh")
	if err != nil {
		return decodedTacticalMesh{}, err
	}
	if err := decodeTacticalMaterialList(&reader, &mesh); err != nil {
		return decodedTacticalMesh{}, err
	}

	next, err := reader.next()
	if err != nil {
		return decodedTacticalMesh{}, err
	}
	if next.kind == xTokenName && next.text == "MeshNormals" {
		if err := reader.expect(xTokenOpenBrace); err != nil {
			return decodedTacticalMesh{}, fmt.Errorf("MeshNormals: %w", err)
		}
		normalCountValues, err := reader.expectIntegers()
		if err != nil {
			return decodedTacticalMesh{}, fmt.Errorf("MeshNormals: %w", err)
		}
		if len(normalCountValues) != 1 || normalCountValues[0] == 0 || normalCountValues[0] > maxXVertices {
			return decodedTacticalMesh{}, fmt.Errorf("MeshNormals: invalid count %v", normalCountValues)
		}
		normalValues, err := reader.expectFloats()
		if err != nil {
			return decodedTacticalMesh{}, fmt.Errorf("MeshNormals: %w", err)
		}
		if len(normalValues) != int(normalCountValues[0])*3 {
			return decodedTacticalMesh{}, fmt.Errorf("MeshNormals: got %d values for %d normals", len(normalValues), normalCountValues[0])
		}
		mesh.Normals = make([]tacticalVec3, normalCountValues[0])
		for index := range mesh.Normals {
			mesh.Normals[index] = tacticalVec3{X: normalValues[index*3], Y: normalValues[index*3+1], Z: normalValues[index*3+2]}
			if !finiteVec3(mesh.Normals[index]) {
				return decodedTacticalMesh{}, fmt.Errorf("MeshNormals: normal %d is not finite", index)
			}
		}
		normalFaceValues, err := reader.expectIntegers()
		if err != nil {
			return decodedTacticalMesh{}, fmt.Errorf("MeshNormals faces: %w", err)
		}
		mesh.NormalFaces, err = decodeTacticalFaces(normalFaceValues, uint32(len(mesh.Normals)), "MeshNormals")
		if err != nil {
			return decodedTacticalMesh{}, err
		}
		if len(mesh.NormalFaces) != len(mesh.Faces) {
			return decodedTacticalMesh{}, fmt.Errorf("MeshNormals: %d faces do not match %d mesh faces", len(mesh.NormalFaces), len(mesh.Faces))
		}
		for index := range mesh.Faces {
			if len(mesh.NormalFaces[index]) != len(mesh.Faces[index]) {
				return decodedTacticalMesh{}, fmt.Errorf("MeshNormals: face %d has %d corners, expected %d", index, len(mesh.NormalFaces[index]), len(mesh.Faces[index]))
			}
		}
		if err := reader.expect(xTokenCloseBrace); err != nil {
			return decodedTacticalMesh{}, fmt.Errorf("MeshNormals: %w", err)
		}
		next, err = reader.next()
		if err != nil {
			return decodedTacticalMesh{}, err
		}
	}
	if next.kind != xTokenName || next.text != "MeshTextureCoords" {
		return decodedTacticalMesh{}, fmt.Errorf("expected MeshTextureCoords, got %s", describeXToken(next))
	}
	if err := reader.expect(xTokenOpenBrace); err != nil {
		return decodedTacticalMesh{}, fmt.Errorf("MeshTextureCoords: %w", err)
	}
	uvCountValues, err := reader.expectIntegers()
	if err != nil {
		return decodedTacticalMesh{}, fmt.Errorf("MeshTextureCoords: %w", err)
	}
	if len(uvCountValues) != 1 || int(uvCountValues[0]) != vertexCount {
		return decodedTacticalMesh{}, fmt.Errorf("MeshTextureCoords: count %v does not match %d vertices", uvCountValues, vertexCount)
	}
	uvValues, err := reader.expectFloats()
	if err != nil {
		return decodedTacticalMesh{}, fmt.Errorf("MeshTextureCoords: %w", err)
	}
	if len(uvValues) != vertexCount*2 {
		return decodedTacticalMesh{}, fmt.Errorf("MeshTextureCoords: got %d values for %d coordinates", len(uvValues), vertexCount)
	}
	mesh.UVs = make([]tacticalVec2, vertexCount)
	for index := range mesh.UVs {
		mesh.UVs[index] = tacticalVec2{U: uvValues[index*2], V: uvValues[index*2+1]}
		if !finite32(mesh.UVs[index].U) || !finite32(mesh.UVs[index].V) {
			return decodedTacticalMesh{}, fmt.Errorf("MeshTextureCoords: coordinate %d is not finite", index)
		}
	}
	if err := reader.expect(xTokenCloseBrace); err != nil {
		return decodedTacticalMesh{}, fmt.Errorf("MeshTextureCoords: %w", err)
	}
	if err := reader.expect(xTokenCloseBrace); err != nil {
		return decodedTacticalMesh{}, fmt.Errorf("Mesh: %w", err)
	}
	if reader.cursor != len(reader.data) {
		return decodedTacticalMesh{}, fmt.Errorf("binary X file has %d trailing bytes", len(reader.data)-reader.cursor)
	}
	return mesh, nil
}

func decodeTacticalMaterialList(reader *xBinaryReader, mesh *decodedTacticalMesh) error {
	if err := reader.expectObject("MeshMaterialList"); err != nil {
		return err
	}
	values, err := reader.expectIntegers()
	if err != nil {
		return fmt.Errorf("MeshMaterialList: %w", err)
	}
	if len(values) < 3 || values[0] == 0 || values[0] > 256 || values[1] == 0 {
		return fmt.Errorf("MeshMaterialList: invalid header %v", values)
	}
	materialCount := int(values[0])
	faceMaterialCount := int(values[1])
	if len(values) != 2+faceMaterialCount {
		return fmt.Errorf("MeshMaterialList: got %d values for %d face-material indices", len(values), faceMaterialCount)
	}
	if faceMaterialCount != 1 && faceMaterialCount != len(mesh.Faces) {
		return fmt.Errorf("MeshMaterialList: %d face-material indices do not match %d faces", faceMaterialCount, len(mesh.Faces))
	}
	mesh.FaceMaterials = append([]uint32(nil), values[2:]...)
	if faceMaterialCount == 1 {
		for len(mesh.FaceMaterials) < len(mesh.Faces) {
			mesh.FaceMaterials = append(mesh.FaceMaterials, mesh.FaceMaterials[0])
		}
	}
	for face, material := range mesh.FaceMaterials {
		if material >= uint32(materialCount) {
			return fmt.Errorf("MeshMaterialList: face %d references material %d of %d", face, material, materialCount)
		}
	}
	mesh.Materials = make([]tacticalMaterial, 0, materialCount)
	for index := 0; index < materialCount; index++ {
		if err := reader.expectObject("Material"); err != nil {
			return fmt.Errorf("MeshMaterialList material %d: %w", index, err)
		}
		floats, err := reader.expectFloats()
		if err != nil {
			return fmt.Errorf("Material %d: %w", index, err)
		}
		if len(floats) != 11 {
			return fmt.Errorf("Material %d: got %d values, expected 11", index, len(floats))
		}
		for valueIndex, value := range floats {
			if !finite32(value) {
				return fmt.Errorf("Material %d: value %d is not finite", index, valueIndex)
			}
		}
		material := tacticalMaterial{SpecularExponent: floats[4]}
		copy(material.Diffuse[:], floats[:4])
		copy(material.Specular[:], floats[5:8])
		copy(material.Emissive[:], floats[8:11])
		next, err := reader.next()
		if err != nil {
			return fmt.Errorf("Material %d: %w", index, err)
		}
		if next.kind == xTokenName && (next.text == "TextureFilename" || next.text == "TextureFileName") {
			if err := reader.expect(xTokenOpenBrace); err != nil {
				return fmt.Errorf("TextureFilename: %w", err)
			}
			texture, err := reader.next()
			if err != nil {
				return fmt.Errorf("TextureFilename: %w", err)
			}
			if texture.kind != xTokenString {
				return fmt.Errorf("TextureFilename: expected string, got %s", describeXToken(texture))
			}
			if err := validateTacticalResourceName(texture.text); err != nil {
				return fmt.Errorf("TextureFilename: %w", err)
			}
			material.Texture = texture.text
			if err := reader.expect(xTokenCloseBrace); err != nil {
				return fmt.Errorf("TextureFilename: %w", err)
			}
			next, err = reader.next()
			if err != nil {
				return fmt.Errorf("Material %d: %w", index, err)
			}
		}
		if next.kind != xTokenCloseBrace {
			return fmt.Errorf("Material %d: expected closing brace, got %s", index, describeXToken(next))
		}
		mesh.Materials = append(mesh.Materials, material)
	}
	if err := reader.expect(xTokenCloseBrace); err != nil {
		return fmt.Errorf("MeshMaterialList: %w", err)
	}
	return nil
}

func decodeTacticalFaces(values []uint32, vertexCount uint32, label string) ([][]uint32, error) {
	if len(values) == 0 || values[0] == 0 || values[0] > maxXFaces {
		return nil, fmt.Errorf("%s: invalid face count", label)
	}
	faceCount := int(values[0])
	faces := make([][]uint32, 0, faceCount)
	cursor := 1
	totalCorners := uint64(0)
	for faceIndex := 0; faceIndex < faceCount; faceIndex++ {
		if cursor >= len(values) {
			return nil, fmt.Errorf("%s: face %d is missing its corner count", label, faceIndex)
		}
		cornerCount := int(values[cursor])
		cursor++
		if cornerCount < 3 || cornerCount > len(values)-cursor {
			return nil, fmt.Errorf("%s: face %d has invalid corner count %d", label, faceIndex, cornerCount)
		}
		totalCorners += uint64(cornerCount)
		if totalCorners > maxXFaceCorners {
			return nil, fmt.Errorf("%s: face corners exceed %d", label, maxXFaceCorners)
		}
		face := append([]uint32(nil), values[cursor:cursor+cornerCount]...)
		for corner, index := range face {
			if index >= vertexCount {
				return nil, fmt.Errorf("%s: face %d corner %d index %d exceeds %d vertices", label, faceIndex, corner, index, vertexCount)
			}
		}
		faces = append(faces, face)
		cursor += cornerCount
	}
	if cursor != len(values) {
		return nil, fmt.Errorf("%s: %d unused face-list values", label, len(values)-cursor)
	}
	return faces, nil
}

func buildTacticalMeshChunks(mesh decodedTacticalMesh) ([]tacticalMeshChunk, error) {
	if len(mesh.Faces) == 0 || len(mesh.FaceMaterials) != len(mesh.Faces) || len(mesh.UVs) != len(mesh.Positions) {
		return nil, fmt.Errorf("mesh is incomplete")
	}
	chunks := make([]tacticalMeshChunk, 0, len(mesh.Materials))
	for faceIndex, face := range mesh.Faces {
		material := mesh.FaceMaterials[faceIndex]
		faceNormal, err := tacticalFaceNormal(mesh.Positions, face)
		if err != nil && len(mesh.Normals) == 0 {
			return nil, fmt.Errorf("face %d: %w", faceIndex, err)
		}
		for corner := 1; corner+1 < len(face); corner++ {
			positionCorners := [3]int{0, corner, corner + 1}
			if len(chunks) == 0 || chunks[len(chunks)-1].Material != material || len(chunks[len(chunks)-1].Vertices)+3 > math.MaxUint16+1 {
				chunks = append(chunks, tacticalMeshChunk{Material: material})
			}
			chunk := &chunks[len(chunks)-1]
			for _, faceCorner := range positionCorners {
				positionIndex := face[faceCorner]
				normal := faceNormal
				if len(mesh.Normals) != 0 {
					normalIndex := mesh.NormalFaces[faceIndex][faceCorner]
					normal = mesh.Normals[normalIndex]
				}
				chunk.Vertices = append(chunk.Vertices, tacticalMeshVertex{
					Position: mesh.Positions[positionIndex],
					Normal:   normal,
					UV:       mesh.UVs[positionIndex],
				})
				chunk.Indices = append(chunk.Indices, uint16(len(chunk.Vertices)-1))
			}
		}
	}
	return chunks, nil
}

func decodeTacticalIndexedImage(data []byte, paletteID uint32, paletteRule string) (tacticalIndexedImage, error) {
	if len(data) < 10 {
		return tacticalIndexedImage{}, fmt.Errorf("type-303 image is shorter than its header and first run")
	}
	width := binary.LittleEndian.Uint32(data[0:4])
	height := binary.LittleEndian.Uint32(data[4:8])
	if width == 0 || height == 0 || uint64(width)*uint64(height) > maxTacticalPixels {
		return tacticalIndexedImage{}, fmt.Errorf("type-303 image has invalid dimensions %dx%d", width, height)
	}
	pixelCount := int(width * height)
	pixels := make([]byte, 0, pixelCount)
	cursor := 8
	for len(pixels) < pixelCount {
		if cursor+2 > len(data) {
			return tacticalIndexedImage{}, fmt.Errorf("type-303 image ended after %d of %d pixels", len(pixels), pixelCount)
		}
		run := int(data[cursor])
		if run == 0 {
			run = 256
		}
		color := data[cursor+1]
		cursor += 2
		if run > pixelCount-len(pixels) {
			return tacticalIndexedImage{}, fmt.Errorf("type-303 run exceeds %d-pixel image", pixelCount)
		}
		for count := 0; count < run; count++ {
			pixels = append(pixels, color)
		}
	}
	return tacticalIndexedImage{
		Width:         width,
		Height:        height,
		PaletteID:     paletteID,
		PaletteRule:   paletteRule,
		Pixels:        pixels,
		TrailingBytes: append([]byte(nil), data[cursor:]...),
	}, nil
}

func tacticalPaletteBinding(resource tactical3DManifestRecord) (uint32, string, error) {
	if resource.Type != tacticalTextureResourceType {
		return 0, "", fmt.Errorf("resource is not type 303")
	}
	if resource.IdentifierKind == "id" && resource.ID >= 5501 && resource.ID <= 5527 {
		return resource.ID + 30, "planet_pair", nil
	}
	return 0, "battle_active", nil
}

func isTacticalPaletteResource(resource tactical3DManifestRecord) bool {
	return resource.Type == tacticalTextureResourceType && resource.IdentifierKind == "id" && resource.ID >= 5531 && resource.ID <= 5557
}

func (reader *xBinaryReader) next() (xBinaryToken, error) {
	if reader.cursor+2 > len(reader.data) {
		return xBinaryToken{}, fmt.Errorf("unexpected end of binary X data at byte %d", reader.cursor)
	}
	kind := binary.LittleEndian.Uint16(reader.data[reader.cursor : reader.cursor+2])
	reader.cursor += 2
	token := xBinaryToken{kind: kind}
	switch kind {
	case xTokenName, xTokenString:
		length, err := reader.readUint32()
		if err != nil {
			return xBinaryToken{}, err
		}
		if length == 0 || length > maxXTokenStringBytes || uint64(length) > uint64(len(reader.data)-reader.cursor) {
			return xBinaryToken{}, fmt.Errorf("invalid binary X string length %d at byte %d", length, reader.cursor-4)
		}
		textBytes := reader.data[reader.cursor : reader.cursor+int(length)]
		reader.cursor += int(length)
		for _, value := range textBytes {
			if value > 0x7f || unicode.IsControl(rune(value)) {
				return xBinaryToken{}, fmt.Errorf("binary X string contains non-ASCII or control data")
			}
		}
		token.text = string(textBytes)
		if kind == xTokenString {
			if reader.cursor+2 > len(reader.data) {
				return xBinaryToken{}, fmt.Errorf("binary X string is missing its terminator")
			}
			terminator := binary.LittleEndian.Uint16(reader.data[reader.cursor : reader.cursor+2])
			reader.cursor += 2
			if terminator != xTokenSemicolon && terminator != xTokenComma {
				return xBinaryToken{}, fmt.Errorf("binary X string has invalid terminator %d", terminator)
			}
		}
	case xTokenInteger:
		value, err := reader.readUint32()
		if err != nil {
			return xBinaryToken{}, err
		}
		token.ints = []uint32{value}
	case xTokenGUID:
		if reader.cursor+16 > len(reader.data) {
			return xBinaryToken{}, fmt.Errorf("truncated binary X GUID")
		}
		reader.cursor += 16
	case xTokenIntegerList, xTokenFloatList:
		count, err := reader.readUint32()
		if err != nil {
			return xBinaryToken{}, err
		}
		if count > maxXFaceCorners || uint64(count)*4 > uint64(len(reader.data)-reader.cursor) {
			return xBinaryToken{}, fmt.Errorf("invalid binary X list count %d", count)
		}
		if kind == xTokenIntegerList {
			token.ints = make([]uint32, count)
			for index := range token.ints {
				token.ints[index] = binary.LittleEndian.Uint32(reader.data[reader.cursor : reader.cursor+4])
				reader.cursor += 4
			}
		} else {
			token.floats = make([]float32, count)
			for index := range token.floats {
				token.floats[index] = math.Float32frombits(binary.LittleEndian.Uint32(reader.data[reader.cursor : reader.cursor+4]))
				reader.cursor += 4
			}
		}
	case xTokenOpenBrace, xTokenCloseBrace:
	default:
		return xBinaryToken{}, fmt.Errorf("unsupported binary X token %d at byte %d", kind, reader.cursor-2)
	}
	return token, nil
}

func (reader *xBinaryReader) readUint32() (uint32, error) {
	if reader.cursor+4 > len(reader.data) {
		return 0, fmt.Errorf("unexpected end of binary X data at byte %d", reader.cursor)
	}
	value := binary.LittleEndian.Uint32(reader.data[reader.cursor : reader.cursor+4])
	reader.cursor += 4
	return value, nil
}

func (reader *xBinaryReader) expect(kind uint16) error {
	token, err := reader.next()
	if err != nil {
		return err
	}
	if token.kind != kind {
		return fmt.Errorf("expected token %d, got %s", kind, describeXToken(token))
	}
	return nil
}

func (reader *xBinaryReader) expectObject(name string) error {
	token, err := reader.next()
	if err != nil {
		return err
	}
	if token.kind != xTokenName || token.text != name {
		return fmt.Errorf("expected %s object, got %s", name, describeXToken(token))
	}
	if err := reader.expect(xTokenOpenBrace); err != nil {
		return fmt.Errorf("%s: %w", name, err)
	}
	return nil
}

func (reader *xBinaryReader) expectIntegers() ([]uint32, error) {
	token, err := reader.next()
	if err != nil {
		return nil, err
	}
	if token.kind != xTokenIntegerList && token.kind != xTokenInteger {
		return nil, fmt.Errorf("expected integer data, got %s", describeXToken(token))
	}
	return token.ints, nil
}

func (reader *xBinaryReader) expectFloats() ([]float32, error) {
	token, err := reader.next()
	if err != nil {
		return nil, err
	}
	if token.kind != xTokenFloatList {
		return nil, fmt.Errorf("expected float list, got %s", describeXToken(token))
	}
	return token.floats, nil
}

func describeXToken(token xBinaryToken) string {
	if token.text != "" {
		return fmt.Sprintf("token %d %q", token.kind, token.text)
	}
	return fmt.Sprintf("token %d", token.kind)
}

func tacticalBounds(positions []tacticalVec3) (tacticalVec3, tacticalVec3) {
	minimum := positions[0]
	maximum := positions[0]
	for _, position := range positions[1:] {
		minimum.X = min(minimum.X, position.X)
		minimum.Y = min(minimum.Y, position.Y)
		minimum.Z = min(minimum.Z, position.Z)
		maximum.X = max(maximum.X, position.X)
		maximum.Y = max(maximum.Y, position.Y)
		maximum.Z = max(maximum.Z, position.Z)
	}
	return minimum, maximum
}

func tacticalFaceNormal(positions []tacticalVec3, face []uint32) (tacticalVec3, error) {
	origin := positions[face[0]]
	for corner := 1; corner+1 < len(face); corner++ {
		left := subtractVec3(positions[face[corner]], origin)
		right := subtractVec3(positions[face[corner+1]], origin)
		normal := crossVec3(left, right)
		length := math.Sqrt(float64(normal.X*normal.X + normal.Y*normal.Y + normal.Z*normal.Z))
		if length > 1e-12 && !math.IsInf(length, 0) && !math.IsNaN(length) {
			scale := float32(1 / length)
			return tacticalVec3{X: normal.X * scale, Y: normal.Y * scale, Z: normal.Z * scale}, nil
		}
	}
	return tacticalVec3{}, fmt.Errorf("degenerate face has no usable normal")
}

func subtractVec3(left, right tacticalVec3) tacticalVec3 {
	return tacticalVec3{X: left.X - right.X, Y: left.Y - right.Y, Z: left.Z - right.Z}
}

func crossVec3(left, right tacticalVec3) tacticalVec3 {
	return tacticalVec3{
		X: left.Y*right.Z - left.Z*right.Y,
		Y: left.Z*right.X - left.X*right.Z,
		Z: left.X*right.Y - left.Y*right.X,
	}
}

func finiteVec3(value tacticalVec3) bool {
	return finite32(value.X) && finite32(value.Y) && finite32(value.Z)
}

func finite32(value float32) bool {
	return !math.IsNaN(float64(value)) && !math.IsInf(float64(value), 0)
}

func normalizeTacticalTextureName(value string) string {
	return strings.ToUpper(value)
}
