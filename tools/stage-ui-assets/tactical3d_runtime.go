package main

import (
	"bytes"
	"encoding/binary"
	"encoding/json"
	"fmt"
	"io"
	"math"
	"os"
	"path/filepath"
)

const (
	tactical3DRuntimeSchemaVersion = 1
	maxTacticalRuntimeBytes        = 256 << 20
	tacticalMeshObjectMagic        = "ORTMESH\x00"
	tacticalIndexObjectMagic       = "ORTINDEX"
	tacticalPaletteObjectMagic     = "ORTPAL00"
	originalTacticalDLLSHA256      = "db12cdcfb6c85cec572a425307d2ad617bbc850027657424c3cc1f61c6ec275a"
	originalMesh2082SHA256         = "7f723dc76b14352e4db4426e9a1985757f777380caaf485386a2c320187c7459"
)

type tactical3DRuntimeManifest struct {
	SchemaVersion        int                              `json:"schema_version"`
	SourceManifestSHA256 string                           `json:"source_manifest_sha256"`
	SourceDLLSHA256      string                           `json:"source_dll_sha256"`
	Meshes               []tactical3DRuntimeMeshRecord    `json:"meshes"`
	Textures             []tactical3DRuntimeTextureRecord `json:"textures"`
}

type tactical3DRuntimeMeshRecord struct {
	ID              uint32                     `json:"id"`
	Language        uint32                     `json:"language"`
	SourceSHA256    string                     `json:"source_sha256"`
	ObjectSHA256    string                     `json:"object_sha256"`
	Object          string                     `json:"object"`
	Vertices        int                        `json:"vertices"`
	Faces           int                        `json:"faces"`
	Triangles       int                        `json:"triangles"`
	Chunks          int                        `json:"chunks"`
	Materials       int                        `json:"materials"`
	Normals         string                     `json:"normals"`
	BoundsMin       [3]float32                 `json:"bounds_min"`
	BoundsMax       [3]float32                 `json:"bounds_max"`
	TextureBindings []tactical3DTextureBinding `json:"texture_bindings"`
}

type tactical3DTextureBinding struct {
	Material             int    `json:"material"`
	XFilename            string `json:"x_filename"`
	Resolution           string `json:"resolution"`
	ResourceName         string `json:"resource_name"`
	ResourceLanguage     uint32 `json:"resource_language"`
	ResourceSourceSHA256 string `json:"resource_source_sha256"`
}

type tactical3DRuntimeTextureRecord struct {
	IdentifierKind string `json:"identifier_kind"`
	ID             uint32 `json:"id,omitempty"`
	Name           string `json:"name,omitempty"`
	Language       uint32 `json:"language"`
	Kind           string `json:"kind"`
	Width          uint32 `json:"width,omitempty"`
	Height         uint32 `json:"height,omitempty"`
	PaletteID      uint32 `json:"palette_id,omitempty"`
	PaletteRule    string `json:"palette_rule,omitempty"`
	TrailingBytes  int    `json:"trailing_bytes,omitempty"`
	TrailingSHA256 string `json:"trailing_sha256,omitempty"`
	SourceSHA256   string `json:"source_sha256"`
	ObjectSHA256   string `json:"object_sha256"`
	Object         string `json:"object"`
}

type tactical3DRuntimeSummary struct {
	Meshes   int
	Textures int
	Written  int
	Skipped  int
}

type preparedTacticalRuntimeObject struct {
	key    string
	data   []byte
	object string
}

func stageTactical3DRuntime(outputDir string, force bool, stdout io.Writer) (tactical3DRuntimeSummary, error) {
	rawRoot := filepath.Join(outputDir, "tactical-dll", "TACTICAL3D")
	rawManifestBytes, err := readRegularFileBounded(filepath.Join(rawRoot, "manifest.json"), maxTacticalManifestBytes)
	if err != nil {
		return tactical3DRuntimeSummary{}, fmt.Errorf("read tactical 3D manifest: %w", err)
	}
	rawManifest, err := decodeTactical3DManifest(rawManifestBytes)
	if err != nil {
		return tactical3DRuntimeSummary{}, err
	}
	return stageTactical3DRuntimeResources(rawRoot, rawManifestBytes, rawManifest, force, stdout)
}

func stageTactical3DRuntimeResources(rawRoot string, rawManifestBytes []byte, rawManifest tactical3DManifest, force bool, stdout io.Writer) (tactical3DRuntimeSummary, error) {
	if rawManifest.SchemaVersion != tactical3DSchemaVersion || rawManifest.Source != "TACTICAL.DLL" || !validSHA256(rawManifest.SourceDLLSHA256) {
		return tactical3DRuntimeSummary{}, fmt.Errorf("invalid tactical 3D source manifest header")
	}
	if len(rawManifest.Resources) != tacticalMeshCount+tacticalTextureCount {
		return tactical3DRuntimeSummary{}, fmt.Errorf("tactical 3D source manifest has %d resources; expected %d", len(rawManifest.Resources), tacticalMeshCount+tacticalTextureCount)
	}
	runtimeRoot := filepath.Join(rawRoot, "runtime")
	manifest := tactical3DRuntimeManifest{
		SchemaVersion:        tactical3DRuntimeSchemaVersion,
		SourceManifestSHA256: sha256Hex(rawManifestBytes),
		SourceDLLSHA256:      rawManifest.SourceDLLSHA256,
	}
	namedTextures := make(map[string][]tactical3DManifestRecord)
	palettes := make(map[uint32][]tactical3DManifestRecord)
	seenSource := make(map[string]struct{}, len(rawManifest.Resources))
	meshSources := 0
	textureSources := 0
	for _, record := range rawManifest.Resources {
		resource, err := rawResourceFromManifest(record)
		if err != nil {
			return tactical3DRuntimeSummary{}, err
		}
		key, err := tacticalResourceKey(record.Type, resource)
		if err != nil {
			return tactical3DRuntimeSummary{}, err
		}
		if _, exists := seenSource[key]; exists {
			return tactical3DRuntimeSummary{}, fmt.Errorf("duplicate tactical 3D source resource %s", key)
		}
		seenSource[key] = struct{}{}
		if record.Size <= 0 || record.Size > maxTacticalRawBytes || !validSHA256(record.SHA256) || record.Object != "objects/"+record.SHA256+".bin" {
			return tactical3DRuntimeSummary{}, fmt.Errorf("tactical 3D source resource %s has invalid size, hash, or object identity", key)
		}
		switch record.Type {
		case tacticalMeshResourceType:
			meshSources++
		case tacticalTextureResourceType:
			textureSources++
			if record.IdentifierKind == "name" {
				folded := normalizeTacticalTextureName(record.Name)
				for _, prior := range namedTextures[folded] {
					if prior.Language == record.Language {
						return tactical3DRuntimeSummary{}, fmt.Errorf("duplicate case-insensitive type-303 name %q for language %d", record.Name, record.Language)
					}
				}
				namedTextures[folded] = append(namedTextures[folded], record)
			}
			if isTacticalPaletteResource(record) {
				for _, prior := range palettes[record.ID] {
					if prior.Language == record.Language {
						return tactical3DRuntimeSummary{}, fmt.Errorf("duplicate tactical palette %d for language %d", record.ID, record.Language)
					}
				}
				palettes[record.ID] = append(palettes[record.ID], record)
			}
		default:
			return tactical3DRuntimeSummary{}, fmt.Errorf("unsupported tactical source resource type %d", record.Type)
		}
	}
	if meshSources != tacticalMeshCount || textureSources != tacticalTextureCount {
		return tactical3DRuntimeSummary{}, fmt.Errorf("tactical 3D source manifest has %d meshes and %d textures; expected %d and %d", meshSources, textureSources, tacticalMeshCount, tacticalTextureCount)
	}
	for paletteID := uint32(5531); paletteID <= 5557; paletteID++ {
		if len(palettes[paletteID]) == 0 {
			return tactical3DRuntimeSummary{}, fmt.Errorf("missing tactical palette %d", paletteID)
		}
	}

	sourcePayloads := make([][]byte, len(rawManifest.Resources))
	decodedMeshes := make(map[int]decodedTacticalMesh, tacticalMeshCount)
	var sourceAggregate uint64
	var minimumRuntimeAggregate uint64
	for index, source := range rawManifest.Resources {
		data, err := readRegularFileBounded(filepath.Join(rawRoot, filepath.FromSlash(source.Object)), maxTacticalRawBytes)
		if err != nil {
			return tactical3DRuntimeSummary{}, fmt.Errorf("read source tactical object %s: %w", source.Object, err)
		}
		if len(data) != source.Size || sha256Hex(data) != source.SHA256 {
			return tactical3DRuntimeSummary{}, fmt.Errorf("source tactical object %s failed size or SHA-256 verification", source.Object)
		}
		if sourceAggregate > maxTacticalAggregateBytes || uint64(len(data)) > maxTacticalAggregateBytes-sourceAggregate {
			return tactical3DRuntimeSummary{}, fmt.Errorf("source tactical objects exceed %d bytes", maxTacticalAggregateBytes)
		}
		sourceAggregate += uint64(len(data))
		validatedResource, err := rawResourceFromManifest(source)
		if err != nil {
			return tactical3DRuntimeSummary{}, err
		}
		validatedResource.Data = data
		if err := validateTacticalRawResource(source.Type, validatedResource); err != nil {
			return tactical3DRuntimeSummary{}, err
		}
		sourcePayloads[index] = data
		var minimumObjectBytes uint64
		switch source.Type {
		case tacticalMeshResourceType:
			mesh, err := decodeTacticalBinaryX(data)
			if err != nil {
				return tactical3DRuntimeSummary{}, fmt.Errorf("type-301 mesh %d: %w", source.ID, err)
			}
			if len(mesh.Normals) == 0 && !(rawManifest.SourceDLLSHA256 == originalTacticalDLLSHA256 && source.ID == 2082 && source.Language == 1033 && source.SHA256 == originalMesh2082SHA256) {
				return tactical3DRuntimeSummary{}, fmt.Errorf("type-301 mesh %d has unsupported missing normals", source.ID)
			}
			minimumObjectBytes, err = minimumTacticalMeshObjectBytes(mesh)
			if err != nil {
				return tactical3DRuntimeSummary{}, fmt.Errorf("type-301 mesh %d: %w", source.ID, err)
			}
			decodedMeshes[index] = mesh
		case tacticalTextureResourceType:
			if isTacticalPaletteResource(source) {
				if len(data) != 256*3 {
					return tactical3DRuntimeSummary{}, fmt.Errorf("type-303 palette %d has %d bytes, expected 768", source.ID, len(data))
				}
				minimumObjectBytes = uint64(len(tacticalPaletteObjectMagic) + 2*4 + len(data))
			} else {
				minimumObjectBytes, err = minimumTacticalIndexObjectBytes(data)
				if err != nil {
					return tactical3DRuntimeSummary{}, fmt.Errorf("type-303 resource %s: %w", tacticalManifestIdentifier(source), err)
				}
			}
		}
		if err := reserveTacticalRuntimeBytes(&minimumRuntimeAggregate, minimumObjectBytes); err != nil {
			return tactical3DRuntimeSummary{}, fmt.Errorf("source tactical resource %s: %w", tacticalManifestIdentifier(source), err)
		}
	}

	prepared := make([]preparedTacticalRuntimeObject, 0, len(rawManifest.Resources))
	summary := tactical3DRuntimeSummary{}
	var runtimeAggregate uint64
	for sourceIndex, source := range rawManifest.Resources {
		data := sourcePayloads[sourceIndex]
		switch source.Type {
		case tacticalMeshResourceType:
			mesh := decodedMeshes[sourceIndex]
			chunks, err := buildTacticalMeshChunks(mesh)
			if err != nil {
				return summary, fmt.Errorf("type-301 mesh %d: %w", source.ID, err)
			}
			objectBytes := encodeTacticalMeshObject(mesh, chunks)
			if err := reserveTacticalRuntimeBytes(&runtimeAggregate, uint64(len(objectBytes))); err != nil {
				return summary, fmt.Errorf("type-301 mesh %d: %w", source.ID, err)
			}
			objectHash := sha256Hex(objectBytes)
			record := tactical3DRuntimeMeshRecord{
				ID:           source.ID,
				Language:     source.Language,
				SourceSHA256: source.SHA256,
				ObjectSHA256: objectHash,
				Object:       "objects/" + objectHash + ".mesh",
				Vertices:     len(mesh.Positions),
				Faces:        len(mesh.Faces),
				Chunks:       len(chunks),
				Materials:    len(mesh.Materials),
				Normals:      "source",
				BoundsMin:    [3]float32{mesh.BoundsMin.X, mesh.BoundsMin.Y, mesh.BoundsMin.Z},
				BoundsMax:    [3]float32{mesh.BoundsMax.X, mesh.BoundsMax.Y, mesh.BoundsMax.Z},
			}
			if len(mesh.Normals) == 0 {
				record.Normals = "derived_face"
			}
			for _, chunk := range chunks {
				record.Triangles += len(chunk.Indices) / 3
			}
			for materialIndex, material := range mesh.Materials {
				if material.Texture == "" {
					continue
				}
				resolved, err := resolveTacticalNamedTexture(namedTextures, material.Texture, source.Language)
				if err != nil {
					return summary, fmt.Errorf("type-301 mesh %d texture %q has no named type-303 resource", source.ID, material.Texture)
				}
				resolution := "exact"
				if material.Texture != resolved.Name {
					resolution = "windows_case_insensitive"
				}
				record.TextureBindings = append(record.TextureBindings, tactical3DTextureBinding{
					Material:             materialIndex,
					XFilename:            material.Texture,
					Resolution:           resolution,
					ResourceName:         resolved.Name,
					ResourceLanguage:     resolved.Language,
					ResourceSourceSHA256: resolved.SHA256,
				})
			}
			manifest.Meshes = append(manifest.Meshes, record)
			prepared = append(prepared, preparedTacticalRuntimeObject{
				key:    fmt.Sprintf("mesh %d", source.ID),
				data:   objectBytes,
				object: filepath.Join(runtimeRoot, filepath.FromSlash(record.Object)),
			})
			summary.Meshes++
		case tacticalTextureResourceType:
			record := tactical3DRuntimeTextureRecord{
				IdentifierKind: source.IdentifierKind,
				ID:             source.ID,
				Name:           source.Name,
				Language:       source.Language,
				SourceSHA256:   source.SHA256,
			}
			var objectBytes []byte
			if isTacticalPaletteResource(source) {
				record.Kind = "palette_rgb24"
				objectBytes = encodeTacticalPaletteObject(source.ID, data)
			} else {
				paletteID, paletteRule, err := tacticalPaletteBinding(source)
				if err != nil {
					return summary, err
				}
				if paletteID != 0 {
					if _, err := resolveTacticalPalette(palettes, paletteID, source.Language); err != nil {
						return summary, fmt.Errorf("type-303 resource %s requires missing palette %d", tacticalManifestIdentifier(source), paletteID)
					}
				}
				image, err := decodeTacticalIndexedImage(data, paletteID, paletteRule)
				if err != nil {
					return summary, fmt.Errorf("type-303 resource %s: %w", tacticalManifestIdentifier(source), err)
				}
				if len(image.TrailingBytes) != 0 && !allowedTacticalImageTail(rawManifest.SourceDLLSHA256, source, image.TrailingBytes) {
					return summary, fmt.Errorf("type-303 resource %s has %d unsupported trailing bytes", tacticalManifestIdentifier(source), len(image.TrailingBytes))
				}
				record.Kind = "indexed_rle"
				record.Width = image.Width
				record.Height = image.Height
				record.PaletteID = image.PaletteID
				record.PaletteRule = image.PaletteRule
				record.TrailingBytes = len(image.TrailingBytes)
				if len(image.TrailingBytes) != 0 {
					record.TrailingSHA256 = sha256Hex(image.TrailingBytes)
				}
				objectBytes = encodeTacticalIndexObject(image)
			}
			if err := reserveTacticalRuntimeBytes(&runtimeAggregate, uint64(len(objectBytes))); err != nil {
				return summary, fmt.Errorf("type-303 resource %s: %w", tacticalManifestIdentifier(source), err)
			}
			objectHash := sha256Hex(objectBytes)
			record.ObjectSHA256 = objectHash
			record.Object = "objects/" + objectHash + ".texture"
			manifest.Textures = append(manifest.Textures, record)
			prepared = append(prepared, preparedTacticalRuntimeObject{
				key:    "texture " + tacticalManifestIdentifier(source),
				data:   objectBytes,
				object: filepath.Join(runtimeRoot, filepath.FromSlash(record.Object)),
			})
			summary.Textures++
		default:
			return summary, fmt.Errorf("unsupported tactical resource type %d", source.Type)
		}
	}
	if summary.Meshes != tacticalMeshCount || summary.Textures != tacticalTextureCount {
		return summary, fmt.Errorf("converted %d meshes and %d textures, expected %d and %d", summary.Meshes, summary.Textures, tacticalMeshCount, tacticalTextureCount)
	}
	manifestBytes, err := json.MarshalIndent(manifest, "", "  ")
	if err != nil {
		return summary, fmt.Errorf("encode tactical runtime manifest: %w", err)
	}
	manifestBytes = append(manifestBytes, '\n')
	if len(manifestBytes) > maxTacticalManifestBytes {
		return summary, fmt.Errorf("tactical runtime manifest exceeds %d bytes", maxTacticalManifestBytes)
	}
	manifestPath := filepath.Join(runtimeRoot, "manifest.json")
	manifestMatches := false
	if existing, err := readRegularFileBounded(manifestPath, maxTacticalManifestBytes); err == nil {
		if bytes.Equal(existing, manifestBytes) {
			manifestMatches = true
		} else if !force {
			return summary, fmt.Errorf("tactical runtime manifest already exists with different contents (use --force to replace it)")
		}
	} else if !os.IsNotExist(err) {
		return summary, fmt.Errorf("read tactical runtime manifest: %w", err)
	}
	if err := os.MkdirAll(filepath.Join(runtimeRoot, "objects"), 0o755); err != nil {
		return tactical3DRuntimeSummary{}, fmt.Errorf("create tactical runtime object directory: %w", err)
	}
	for _, object := range prepared {
		written, err := stageContentAddressedObject(object.object, object.data)
		if err != nil {
			return summary, fmt.Errorf("stage tactical runtime %s: %w", object.key, err)
		}
		if written {
			summary.Written++
		} else {
			summary.Skipped++
		}
	}
	if !manifestMatches {
		if err := writeFileAtomically(manifestPath, manifestBytes, 0o644); err != nil {
			return summary, fmt.Errorf("write tactical runtime manifest: %w", err)
		}
	}
	fmt.Fprintf(stdout, "Converted %d tactical meshes and %d type-303 resources (%d written, %d unchanged)\n", summary.Meshes, summary.Textures, summary.Written, summary.Skipped)
	return summary, nil
}

func verifyTactical3DRuntime(outputDir string, stdout io.Writer) error {
	root := filepath.Join(outputDir, "tactical-dll", "TACTICAL3D", "runtime")
	manifestBytes, err := readRegularFileBounded(filepath.Join(root, "manifest.json"), maxTacticalManifestBytes)
	if err != nil {
		return fmt.Errorf("read tactical runtime manifest: %w", err)
	}
	var manifest tactical3DRuntimeManifest
	decoder := json.NewDecoder(bytes.NewReader(manifestBytes))
	decoder.DisallowUnknownFields()
	if err := decoder.Decode(&manifest); err != nil {
		return fmt.Errorf("decode tactical runtime manifest: %w", err)
	}
	if err := decoder.Decode(&struct{}{}); err != io.EOF {
		return fmt.Errorf("tactical runtime manifest has trailing JSON data")
	}
	if manifest.SchemaVersion != tactical3DRuntimeSchemaVersion || !validSHA256(manifest.SourceManifestSHA256) || !validSHA256(manifest.SourceDLLSHA256) {
		return fmt.Errorf("invalid tactical runtime manifest header")
	}
	if len(manifest.Meshes) != tacticalMeshCount || len(manifest.Textures) != tacticalTextureCount {
		return fmt.Errorf("tactical runtime manifest has %d meshes and %d textures, expected %d and %d", len(manifest.Meshes), len(manifest.Textures), tacticalMeshCount, tacticalTextureCount)
	}
	namedTextures, palettes, err := indexTacticalRuntimeTextures(manifest.Textures)
	if err != nil {
		return err
	}
	seen := make(map[string]struct{}, len(manifest.Meshes)+len(manifest.Textures))
	var aggregate uint64
	for _, record := range manifest.Meshes {
		key := fmt.Sprintf("mesh:%d:%d", record.ID, record.Language)
		if _, exists := seen[key]; exists {
			return fmt.Errorf("duplicate tactical runtime %s", key)
		}
		seen[key] = struct{}{}
		if !validSHA256(record.SourceSHA256) || record.Object != "objects/"+record.ObjectSHA256+".mesh" || record.Vertices <= 0 || record.Faces <= 0 || record.Triangles <= 0 || record.Chunks <= 0 || record.Materials <= 0 || record.Materials > 256 || (record.Normals != "source" && record.Normals != "derived_face") {
			return fmt.Errorf("%s has invalid manifest metadata", key)
		}
		if record.Normals == "derived_face" && !allowedDerivedTacticalNormals(manifest.SourceDLLSHA256, record) {
			return fmt.Errorf("%s has unsupported derived normals", key)
		}
		seenBindings := make(map[int]struct{}, len(record.TextureBindings))
		for _, binding := range record.TextureBindings {
			if binding.Material < 0 || binding.Material >= record.Materials || binding.ResourceLanguage != record.Language || validateTacticalResourceName(binding.XFilename) != nil || validateTacticalResourceName(binding.ResourceName) != nil || !validSHA256(binding.ResourceSourceSHA256) || (binding.Resolution != "exact" && binding.Resolution != "windows_case_insensitive") {
				return fmt.Errorf("%s has invalid texture binding", key)
			}
			if _, exists := seenBindings[binding.Material]; exists {
				return fmt.Errorf("%s has duplicate material texture binding", key)
			}
			seenBindings[binding.Material] = struct{}{}
			if normalizeTacticalTextureName(binding.XFilename) != normalizeTacticalTextureName(binding.ResourceName) || (binding.Resolution == "exact") != (binding.XFilename == binding.ResourceName) {
				return fmt.Errorf("%s has inconsistent texture binding", key)
			}
			target, exists := namedTextures[tacticalRuntimeNamedTextureKey(binding.ResourceName, binding.ResourceLanguage)]
			if !exists || target.Name != binding.ResourceName || target.SourceSHA256 != binding.ResourceSourceSHA256 {
				return fmt.Errorf("%s references a missing or mismatched named texture", key)
			}
		}
		data, err := verifyTacticalRuntimeObject(root, record.Object, record.ObjectSHA256, &aggregate)
		if err != nil {
			return fmt.Errorf("%s: %w", key, err)
		}
		if err := validateTacticalMeshObject(data, record); err != nil {
			return fmt.Errorf("%s: %w", key, err)
		}
	}
	for _, record := range manifest.Textures {
		key := fmt.Sprintf("texture:%s:%d", runtimeTextureIdentifier(record), record.Language)
		if _, exists := seen[key]; exists {
			return fmt.Errorf("duplicate tactical runtime %s", key)
		}
		seen[key] = struct{}{}
		if !validSHA256(record.SourceSHA256) || record.Object != "objects/"+record.ObjectSHA256+".texture" {
			return fmt.Errorf("%s has invalid manifest metadata", key)
		}
		if record.IdentifierKind == "name" {
			if record.ID != 0 || validateTacticalResourceName(record.Name) != nil {
				return fmt.Errorf("%s has invalid named identity", key)
			}
		} else if record.IdentifierKind != "id" || record.Name != "" {
			return fmt.Errorf("%s has invalid numeric identity", key)
		}
		switch record.Kind {
		case "palette_rgb24":
			if record.IdentifierKind != "id" || record.ID < 5531 || record.ID > 5557 || record.Width != 0 || record.Height != 0 || record.PaletteID != 0 || record.PaletteRule != "" || record.TrailingBytes != 0 || record.TrailingSHA256 != "" {
				return fmt.Errorf("%s has invalid palette metadata", key)
			}
		case "indexed_rle":
			if record.Width == 0 || record.Height == 0 || uint64(record.Width)*uint64(record.Height) > maxTacticalPixels {
				return fmt.Errorf("%s has invalid image dimensions", key)
			}
			planet := record.IdentifierKind == "id" && record.ID >= 5501 && record.ID <= 5527
			if planet {
				if record.PaletteRule != "planet_pair" || record.PaletteID != record.ID+30 {
					return fmt.Errorf("%s has invalid planet palette binding", key)
				}
				if _, exists := palettes[tacticalRuntimePaletteKey(record.PaletteID, record.Language)]; !exists {
					return fmt.Errorf("%s references a missing same-language palette", key)
				}
			} else if record.PaletteRule != "battle_active" || record.PaletteID != 0 {
				return fmt.Errorf("%s has invalid active palette binding", key)
			}
			if (record.TrailingBytes == 0) != (record.TrailingSHA256 == "") || (record.TrailingBytes != 0 && (record.TrailingBytes != 4 || !validSHA256(record.TrailingSHA256))) {
				return fmt.Errorf("%s has invalid trailing-data metadata", key)
			}
			if record.TrailingBytes != 0 {
				source := tactical3DManifestRecord{
					Type:           tacticalTextureResourceType,
					IdentifierKind: record.IdentifierKind,
					ID:             record.ID,
					Name:           record.Name,
					Language:       record.Language,
					SHA256:         record.SourceSHA256,
				}
				if !allowedTacticalImageTail(manifest.SourceDLLSHA256, source, []byte{0, 0, 0, 0}) {
					return fmt.Errorf("%s has unsupported trailing data", key)
				}
			}
		default:
			return fmt.Errorf("%s has unknown texture kind %q", key, record.Kind)
		}
		data, err := verifyTacticalRuntimeObject(root, record.Object, record.ObjectSHA256, &aggregate)
		if err != nil {
			return fmt.Errorf("%s: %w", key, err)
		}
		if err := validateTacticalTextureObject(data, record); err != nil {
			return fmt.Errorf("%s: %w", key, err)
		}
	}
	fmt.Fprintf(stdout, "Verified tactical runtime pack with %d meshes and %d type-303 resources\n", len(manifest.Meshes), len(manifest.Textures))
	return nil
}

func decodeTactical3DManifest(data []byte) (tactical3DManifest, error) {
	var manifest tactical3DManifest
	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	if err := decoder.Decode(&manifest); err != nil {
		return tactical3DManifest{}, fmt.Errorf("decode tactical 3D manifest: %w", err)
	}
	if err := decoder.Decode(&struct{}{}); err != io.EOF {
		return tactical3DManifest{}, fmt.Errorf("tactical 3D manifest has trailing JSON data")
	}
	return manifest, nil
}

func encodeTacticalMeshObject(mesh decodedTacticalMesh, chunks []tacticalMeshChunk) []byte {
	data := make([]byte, 0, 128)
	data = append(data, tacticalMeshObjectMagic...)
	data = appendUint32(data, 1)
	data = appendUint32(data, uint32(len(mesh.Materials)))
	data = appendUint32(data, uint32(len(chunks)))
	data = appendUint32(data, uint32(len(mesh.Positions)))
	data = appendUint32(data, uint32(len(mesh.Faces)))
	normalMode := uint32(0)
	if len(mesh.Normals) == 0 {
		normalMode = 1
	}
	data = appendUint32(data, normalMode)
	for _, value := range []float32{mesh.BoundsMin.X, mesh.BoundsMin.Y, mesh.BoundsMin.Z, mesh.BoundsMax.X, mesh.BoundsMax.Y, mesh.BoundsMax.Z} {
		data = appendFloat32(data, value)
	}
	for _, material := range mesh.Materials {
		for _, value := range material.Diffuse {
			data = appendFloat32(data, value)
		}
		data = appendFloat32(data, material.SpecularExponent)
		for _, value := range material.Specular {
			data = appendFloat32(data, value)
		}
		for _, value := range material.Emissive {
			data = appendFloat32(data, value)
		}
		data = appendUint32(data, uint32(len(material.Texture)))
		data = append(data, material.Texture...)
	}
	for _, chunk := range chunks {
		data = appendUint32(data, chunk.Material)
		data = appendUint32(data, uint32(len(chunk.Vertices)))
		data = appendUint32(data, uint32(len(chunk.Indices)))
		for _, vertex := range chunk.Vertices {
			for _, value := range []float32{vertex.Position.X, vertex.Position.Y, vertex.Position.Z, vertex.Normal.X, vertex.Normal.Y, vertex.Normal.Z, vertex.UV.U, vertex.UV.V} {
				data = appendFloat32(data, value)
			}
		}
		for _, index := range chunk.Indices {
			data = binary.LittleEndian.AppendUint16(data, index)
		}
	}
	return data
}

func encodeTacticalIndexObject(image tacticalIndexedImage) []byte {
	data := append([]byte(nil), tacticalIndexObjectMagic...)
	data = appendUint32(data, 1)
	data = appendUint32(data, image.Width)
	data = appendUint32(data, image.Height)
	data = appendUint32(data, image.PaletteID)
	data = appendUint32(data, tacticalPaletteRuleCode(image.PaletteRule))
	data = appendUint32(data, uint32(len(image.Pixels)))
	data = appendUint32(data, uint32(len(image.TrailingBytes)))
	data = append(data, image.Pixels...)
	data = append(data, image.TrailingBytes...)
	return data
}

func encodeTacticalPaletteObject(id uint32, rgb []byte) []byte {
	data := append([]byte(nil), tacticalPaletteObjectMagic...)
	data = appendUint32(data, 1)
	data = appendUint32(data, id)
	data = append(data, rgb...)
	return data
}

func validateTacticalMeshObject(data []byte, record tactical3DRuntimeMeshRecord) error {
	reader := tacticalRuntimeReader{data: data}
	if err := reader.expectMagic(tacticalMeshObjectMagic); err != nil {
		return err
	}
	version, err := reader.uint32()
	if err != nil || version != 1 {
		return fmt.Errorf("invalid mesh object version")
	}
	materialCount, err := reader.uint32()
	if err != nil {
		return err
	}
	chunkCount, err := reader.uint32()
	if err != nil || int(chunkCount) != record.Chunks {
		return fmt.Errorf("mesh chunk count does not match manifest")
	}
	if materialCount == 0 || materialCount > 256 || int(materialCount) != record.Materials || chunkCount == 0 || chunkCount > maxXFaces {
		return fmt.Errorf("invalid mesh material or chunk count")
	}
	vertices, err := reader.uint32()
	if err != nil || int(vertices) != record.Vertices {
		return fmt.Errorf("mesh source vertex count does not match manifest")
	}
	faces, err := reader.uint32()
	if err != nil || int(faces) != record.Faces {
		return fmt.Errorf("mesh source face count does not match manifest")
	}
	normalMode, err := reader.uint32()
	if err != nil || (normalMode == 0) != (record.Normals == "source") || normalMode > 1 {
		return fmt.Errorf("mesh normal mode does not match manifest")
	}
	expectedBounds := [6]float32{record.BoundsMin[0], record.BoundsMin[1], record.BoundsMin[2], record.BoundsMax[0], record.BoundsMax[1], record.BoundsMax[2]}
	for index := 0; index < 6; index++ {
		value, err := reader.float32()
		if err != nil || !finite32(value) || math.Float32bits(value) != math.Float32bits(expectedBounds[index]) {
			return fmt.Errorf("invalid mesh bound")
		}
	}
	bindings := make(map[uint32]tactical3DTextureBinding, len(record.TextureBindings))
	for _, binding := range record.TextureBindings {
		material := uint32(binding.Material)
		if _, exists := bindings[material]; exists {
			return fmt.Errorf("duplicate mesh material texture binding")
		}
		bindings[material] = binding
	}
	for material := uint32(0); material < materialCount; material++ {
		for index := 0; index < 11; index++ {
			value, err := reader.float32()
			if err != nil || !finite32(value) {
				return fmt.Errorf("invalid mesh material")
			}
		}
		length, err := reader.uint32()
		if err != nil || length > maxXTokenStringBytes || uint64(length) > uint64(len(data)-reader.cursor) {
			return fmt.Errorf("invalid mesh material texture name")
		}
		texture := string(data[reader.cursor : reader.cursor+int(length)])
		reader.cursor += int(length)
		binding, bound := bindings[material]
		if texture == "" {
			if bound {
				return fmt.Errorf("untextured mesh material has a texture binding")
			}
		} else if !bound || binding.XFilename != texture || validateTacticalResourceName(texture) != nil {
			return fmt.Errorf("mesh material texture does not match its binding")
		}
	}
	triangles := 0
	for chunk := uint32(0); chunk < chunkCount; chunk++ {
		material, err := reader.uint32()
		if err != nil || material >= materialCount {
			return fmt.Errorf("invalid mesh chunk material")
		}
		vertices, err := reader.uint32()
		if err != nil || vertices == 0 || vertices > math.MaxUint16+1 {
			return fmt.Errorf("invalid mesh chunk vertex count")
		}
		indices, err := reader.uint32()
		if err != nil || indices == 0 || indices%3 != 0 {
			return fmt.Errorf("invalid mesh chunk index count")
		}
		for index := uint64(0); index < uint64(vertices)*8; index++ {
			value, err := reader.float32()
			if err != nil || !finite32(value) {
				return fmt.Errorf("invalid mesh vertex value")
			}
		}
		for index := uint32(0); index < indices; index++ {
			value, err := reader.uint16()
			if err != nil || uint32(value) >= vertices {
				return fmt.Errorf("invalid mesh index")
			}
		}
		triangles += int(indices / 3)
	}
	if reader.cursor != len(data) || triangles != record.Triangles {
		return fmt.Errorf("mesh object size or triangle count does not match manifest")
	}
	return nil
}

func validateTacticalTextureObject(data []byte, record tactical3DRuntimeTextureRecord) error {
	reader := tacticalRuntimeReader{data: data}
	switch record.Kind {
	case "palette_rgb24":
		if err := reader.expectMagic(tacticalPaletteObjectMagic); err != nil {
			return err
		}
		version, err := reader.uint32()
		if err != nil || version != 1 {
			return fmt.Errorf("invalid palette object version")
		}
		id, err := reader.uint32()
		if err != nil || id != record.ID || !reader.skip(256*3) || reader.cursor != len(data) {
			return fmt.Errorf("invalid palette object")
		}
	case "indexed_rle":
		if err := reader.expectMagic(tacticalIndexObjectMagic); err != nil {
			return err
		}
		values := make([]uint32, 7)
		for index := range values {
			value, err := reader.uint32()
			if err != nil {
				return err
			}
			values[index] = value
		}
		version, width, height, paletteID, paletteRule, pixels, trailing := values[0], values[1], values[2], values[3], values[4], values[5], values[6]
		if version != 1 || width != record.Width || height != record.Height || paletteID != record.PaletteID || paletteRule != tacticalPaletteRuleCode(record.PaletteRule) || uint64(width)*uint64(height) != uint64(pixels) || int(trailing) != record.TrailingBytes {
			return fmt.Errorf("indexed texture header does not match manifest")
		}
		if !reader.skip(uint64(pixels)) || uint64(trailing) > uint64(len(data)-reader.cursor) {
			return fmt.Errorf("invalid indexed texture object size")
		}
		tail := data[reader.cursor : reader.cursor+int(trailing)]
		reader.cursor += int(trailing)
		if reader.cursor != len(data) || (len(tail) != 0 && sha256Hex(tail) != record.TrailingSHA256) {
			return fmt.Errorf("indexed texture tail or object size does not match manifest")
		}
	default:
		return fmt.Errorf("unknown tactical texture kind %q", record.Kind)
	}
	return nil
}

func verifyTacticalRuntimeObject(root, object, expectedHash string, aggregate *uint64) ([]byte, error) {
	if !validSHA256(expectedHash) || object == "" || filepath.ToSlash(object) != object || filepath.Clean(object) != filepath.FromSlash(object) || filepath.Dir(object) != "objects" {
		return nil, fmt.Errorf("invalid runtime object identity")
	}
	data, err := readRegularFileBounded(filepath.Join(root, filepath.FromSlash(object)), maxTacticalRawBytes)
	if err != nil {
		return nil, err
	}
	if sha256Hex(data) != expectedHash {
		return nil, fmt.Errorf("runtime object failed SHA-256 verification")
	}
	if *aggregate > maxTacticalRuntimeBytes || uint64(len(data)) > maxTacticalRuntimeBytes-*aggregate {
		return nil, fmt.Errorf("tactical runtime objects exceed %d bytes", maxTacticalRuntimeBytes)
	}
	*aggregate += uint64(len(data))
	return data, nil
}

type tacticalRuntimeReader struct {
	data   []byte
	cursor int
}

func (reader *tacticalRuntimeReader) expectMagic(value string) error {
	if len(reader.data)-reader.cursor < len(value) || string(reader.data[reader.cursor:reader.cursor+len(value)]) != value {
		return fmt.Errorf("invalid tactical runtime object magic")
	}
	reader.cursor += len(value)
	return nil
}

func (reader *tacticalRuntimeReader) uint16() (uint16, error) {
	if reader.cursor+2 > len(reader.data) {
		return 0, io.ErrUnexpectedEOF
	}
	value := binary.LittleEndian.Uint16(reader.data[reader.cursor : reader.cursor+2])
	reader.cursor += 2
	return value, nil
}

func (reader *tacticalRuntimeReader) uint32() (uint32, error) {
	if reader.cursor+4 > len(reader.data) {
		return 0, io.ErrUnexpectedEOF
	}
	value := binary.LittleEndian.Uint32(reader.data[reader.cursor : reader.cursor+4])
	reader.cursor += 4
	return value, nil
}

func (reader *tacticalRuntimeReader) float32() (float32, error) {
	value, err := reader.uint32()
	return math.Float32frombits(value), err
}

func (reader *tacticalRuntimeReader) skip(count uint64) bool {
	if count > uint64(len(reader.data)-reader.cursor) {
		return false
	}
	reader.cursor += int(count)
	return true
}

func appendUint32(data []byte, value uint32) []byte {
	return binary.LittleEndian.AppendUint32(data, value)
}

func appendFloat32(data []byte, value float32) []byte {
	return appendUint32(data, math.Float32bits(value))
}

func tacticalManifestIdentifier(record tactical3DManifestRecord) string {
	if record.IdentifierKind == "name" {
		return record.Name
	}
	return fmt.Sprintf("%d", record.ID)
}

func runtimeTextureIdentifier(record tactical3DRuntimeTextureRecord) string {
	if record.IdentifierKind == "name" {
		return record.Name
	}
	return fmt.Sprintf("%d", record.ID)
}

func indexTacticalRuntimeTextures(resources []tactical3DRuntimeTextureRecord) (map[string]tactical3DRuntimeTextureRecord, map[string]tactical3DRuntimeTextureRecord, error) {
	named := make(map[string]tactical3DRuntimeTextureRecord)
	palettes := make(map[string]tactical3DRuntimeTextureRecord)
	paletteIDs := make(map[uint32]struct{})
	for _, resource := range resources {
		if resource.IdentifierKind == "name" {
			if validateTacticalResourceName(resource.Name) != nil {
				return nil, nil, fmt.Errorf("invalid named tactical runtime texture")
			}
			key := tacticalRuntimeNamedTextureKey(resource.Name, resource.Language)
			if _, exists := named[key]; exists {
				return nil, nil, fmt.Errorf("duplicate case-insensitive tactical runtime texture %q for language %d", resource.Name, resource.Language)
			}
			named[key] = resource
		}
		if resource.Kind == "palette_rgb24" {
			key := tacticalRuntimePaletteKey(resource.ID, resource.Language)
			if _, exists := palettes[key]; exists {
				return nil, nil, fmt.Errorf("duplicate tactical runtime palette %d for language %d", resource.ID, resource.Language)
			}
			palettes[key] = resource
			paletteIDs[resource.ID] = struct{}{}
		}
	}
	for id := uint32(5531); id <= 5557; id++ {
		if _, exists := paletteIDs[id]; !exists {
			return nil, nil, fmt.Errorf("missing tactical runtime palette %d", id)
		}
	}
	if len(palettes) != 27 {
		return nil, nil, fmt.Errorf("tactical runtime has %d palettes; expected 27", len(palettes))
	}
	return named, palettes, nil
}

func tacticalRuntimeNamedTextureKey(name string, language uint32) string {
	return fmt.Sprintf("%d:%s", language, normalizeTacticalTextureName(name))
}

func tacticalRuntimePaletteKey(id, language uint32) string {
	return fmt.Sprintf("%d:%d", language, id)
}

func resolveTacticalNamedTexture(resources map[string][]tactical3DManifestRecord, name string, language uint32) (tactical3DManifestRecord, error) {
	candidates := resources[normalizeTacticalTextureName(name)]
	var match *tactical3DManifestRecord
	for index := range candidates {
		if candidates[index].Language != language {
			continue
		}
		if match != nil {
			return tactical3DManifestRecord{}, fmt.Errorf("ambiguous named texture %q for language %d", name, language)
		}
		match = &candidates[index]
	}
	if match == nil {
		return tactical3DManifestRecord{}, fmt.Errorf("missing named texture %q for language %d", name, language)
	}
	return *match, nil
}

func resolveTacticalPalette(resources map[uint32][]tactical3DManifestRecord, id, language uint32) (tactical3DManifestRecord, error) {
	var match *tactical3DManifestRecord
	for index := range resources[id] {
		if resources[id][index].Language != language {
			continue
		}
		if match != nil {
			return tactical3DManifestRecord{}, fmt.Errorf("ambiguous palette %d for language %d", id, language)
		}
		match = &resources[id][index]
	}
	if match == nil {
		return tactical3DManifestRecord{}, fmt.Errorf("missing palette %d for language %d", id, language)
	}
	return *match, nil
}

func tacticalPaletteRuleCode(rule string) uint32 {
	switch rule {
	case "battle_active":
		return 1
	case "planet_pair":
		return 2
	default:
		return 0
	}
}

func allowedTacticalImageTail(sourceDLLSHA256 string, source tactical3DManifestRecord, tail []byte) bool {
	if sourceDLLSHA256 != originalTacticalDLLSHA256 || source.IdentifierKind != "id" || source.Language != 1033 || len(tail) != 4 {
		return false
	}
	expected := map[uint32]string{
		4200: "753d4144e6c0a3e15b8150fbdf9e264bcdf4e645ae3ee5ce2753f168420e1b7d",
		4201: "e0b34d757c14b2cde24244999e4f84a31b678c2d4f44671397f23332fde66f62",
		4202: "6c47a26f0c0c71ef948a8e68378e73dcbc91079b05fa1fdc76667cf416e208ad",
		4203: "ab34fb0c3de5bd6adb94af46ac45b7740aca0ec45f14178612c2c85f908449df",
		4204: "9ad87c599c39b4de50cb6f0e59808275007dba2049e374988218547d77ca179e",
	}
	return source.SHA256 == expected[source.ID]
}

func allowedDerivedTacticalNormals(sourceDLLSHA256 string, record tactical3DRuntimeMeshRecord) bool {
	return sourceDLLSHA256 == originalTacticalDLLSHA256 &&
		record.ID == 2082 &&
		record.Language == 1033 &&
		record.SourceSHA256 == originalMesh2082SHA256
}

func minimumTacticalMeshObjectBytes(mesh decodedTacticalMesh) (uint64, error) {
	bytes := uint64(len(tacticalMeshObjectMagic) + 6*4 + 6*4)
	for _, material := range mesh.Materials {
		bytes += 11*4 + 4 + uint64(len(material.Texture))
	}
	var triangles uint64
	for _, face := range mesh.Faces {
		if len(face) < 3 {
			return 0, fmt.Errorf("mesh contains a face with fewer than three corners")
		}
		triangles += uint64(len(face) - 2)
	}
	if triangles > ^uint64(0)/(3*(8*4+2)) {
		return 0, fmt.Errorf("decoded mesh size overflows")
	}
	bytes += triangles * 3 * (8*4 + 2)
	return bytes, nil
}

func minimumTacticalIndexObjectBytes(data []byte) (uint64, error) {
	if len(data) < 10 {
		return 0, fmt.Errorf("type-303 image is shorter than its header and first run")
	}
	width := binary.LittleEndian.Uint32(data[0:4])
	height := binary.LittleEndian.Uint32(data[4:8])
	pixels := uint64(width) * uint64(height)
	if width == 0 || height == 0 || pixels > maxTacticalPixels {
		return 0, fmt.Errorf("type-303 image has invalid dimensions %dx%d", width, height)
	}
	return uint64(len(tacticalIndexObjectMagic)) + 7*4 + pixels, nil
}

func checkTacticalRuntimeBudget(current, additional uint64) error {
	if additional > maxTacticalRawBytes {
		return fmt.Errorf("runtime object exceeds %d bytes", maxTacticalRawBytes)
	}
	if current > maxTacticalRuntimeBytes || additional > maxTacticalRuntimeBytes-current {
		return fmt.Errorf("tactical runtime objects exceed %d bytes", maxTacticalRuntimeBytes)
	}
	return nil
}

func reserveTacticalRuntimeBytes(current *uint64, additional uint64) error {
	if err := checkTacticalRuntimeBudget(*current, additional); err != nil {
		return err
	}
	*current += additional
	return nil
}
