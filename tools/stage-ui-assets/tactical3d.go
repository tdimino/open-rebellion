package main

import (
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"unicode"
)

const (
	tacticalMeshResourceType    = uint32(301)
	tacticalTextureResourceType = uint32(303)
	tacticalMeshCount           = 87
	tacticalTextureCount        = 397
	tactical3DSchemaVersion     = 1
	maxTacticalSourceBytes      = 256 << 20
	maxTacticalManifestBytes    = 4 << 20
	maxTacticalRawBytes         = 64 << 20
	maxTacticalAggregateBytes   = 128 << 20
	maxTacticalDecodedMeshBytes = 128 << 20
)

type tactical3DManifest struct {
	SchemaVersion   int                        `json:"schema_version"`
	Source          string                     `json:"source"`
	SourceDLLSHA256 string                     `json:"source_dll_sha256"`
	Resources       []tactical3DManifestRecord `json:"resources"`
}

type tactical3DManifestRecord struct {
	Type           uint32 `json:"type"`
	IdentifierKind string `json:"identifier_kind"`
	ID             uint32 `json:"id,omitempty"`
	Name           string `json:"name,omitempty"`
	Language       uint32 `json:"language"`
	CodePage       uint32 `json:"code_page"`
	Reserved       uint32 `json:"reserved"`
	Size           int    `json:"size"`
	SHA256         string `json:"sha256"`
	Object         string `json:"object"`
}

type tactical3DStageSummary struct {
	Resources int
	Written   int
	Skipped   int
}

type preparedTacticalResource struct {
	key    string
	data   []byte
	object string
}

func stageTactical3D(sourceDir, outputDir string, force bool, stdout io.Writer) (tactical3DStageSummary, error) {
	dllPath := filepath.Join(sourceDir, "TACTICAL.DLL")
	sourceBytes, err := readRegularFileBounded(dllPath, maxTacticalSourceBytes)
	if err != nil {
		return tactical3DStageSummary{}, fmt.Errorf("TACTICAL.DLL: read source snapshot: %w", err)
	}
	meshes, err := readPEMixedRawResourcesFromBytes(sourceBytes, tacticalMeshResourceType, rawResourceLimits{
		MaxCount:          tacticalMeshCount,
		MaxResourceBytes:  maxTacticalRawBytes,
		MaxAggregateBytes: maxTacticalAggregateBytes,
	})
	if err != nil {
		return tactical3DStageSummary{}, fmt.Errorf("TACTICAL.DLL type 301: %w", err)
	}
	remainingAggregateBytes, err := remainingResourceBudget(maxTacticalAggregateBytes, meshes)
	if err != nil {
		return tactical3DStageSummary{}, fmt.Errorf("TACTICAL.DLL type 301: %w", err)
	}
	if remainingAggregateBytes == 0 {
		return tactical3DStageSummary{}, fmt.Errorf("TACTICAL.DLL type 301 exhausts the %d-byte aggregate limit", maxTacticalAggregateBytes)
	}
	textures, err := readPEMixedRawResourcesFromBytes(sourceBytes, tacticalTextureResourceType, rawResourceLimits{
		MaxCount:          tacticalTextureCount,
		MaxResourceBytes:  maxTacticalRawBytes,
		MaxAggregateBytes: remainingAggregateBytes,
	})
	if err != nil {
		return tactical3DStageSummary{}, fmt.Errorf("TACTICAL.DLL type 303: %w", err)
	}
	return stageTactical3DResources(
		filepath.Join(outputDir, "tactical-dll", "TACTICAL3D"),
		sourceBytes,
		meshes,
		textures,
		tacticalMeshCount,
		tacticalTextureCount,
		force,
		stdout,
	)
}

func stageTactical3DResources(outputDir string, sourceBytes []byte, meshes, textures []rawResource, expectedMeshes, expectedTextures int, force bool, stdout io.Writer) (tactical3DStageSummary, error) {
	if len(meshes) != expectedMeshes {
		return tactical3DStageSummary{}, fmt.Errorf("found %d type-301 resources, expected %d", len(meshes), expectedMeshes)
	}
	if len(textures) != expectedTextures {
		return tactical3DStageSummary{}, fmt.Errorf("found %d type-303 resources, expected %d", len(textures), expectedTextures)
	}
	if len(sourceBytes) == 0 {
		return tactical3DStageSummary{}, fmt.Errorf("TACTICAL.DLL source is empty")
	}

	manifest := tactical3DManifest{
		SchemaVersion:   tactical3DSchemaVersion,
		Source:          "TACTICAL.DLL",
		SourceDLLSHA256: sha256Hex(sourceBytes),
	}
	resources := make([]struct {
		typeID   uint32
		resource rawResource
	}, 0, len(meshes)+len(textures))
	for _, resource := range meshes {
		resources = append(resources, struct {
			typeID   uint32
			resource rawResource
		}{tacticalMeshResourceType, resource})
	}
	for _, resource := range textures {
		resources = append(resources, struct {
			typeID   uint32
			resource rawResource
		}{tacticalTextureResourceType, resource})
	}

	seen := make(map[string]struct{}, len(resources))
	caseFoldedNames := make(map[string]string)
	summary := tactical3DStageSummary{Resources: len(resources)}
	prepared := make([]preparedTacticalResource, 0, len(resources))
	for _, typed := range resources {
		if err := validateTacticalRawResource(typed.typeID, typed.resource); err != nil {
			return summary, err
		}
		key, err := tacticalResourceKey(typed.typeID, typed.resource)
		if err != nil {
			return summary, err
		}
		if _, exists := seen[key]; exists {
			return summary, fmt.Errorf("duplicate tactical resource %s", key)
		}
		seen[key] = struct{}{}
		if typed.resource.Named {
			folded := fmt.Sprintf("%d:%d:%s", typed.typeID, typed.resource.Language, strings.ToUpper(typed.resource.Name))
			if prior, exists := caseFoldedNames[folded]; exists && prior != typed.resource.Name {
				return summary, fmt.Errorf("ambiguous case-folded tactical resource names %q and %q", prior, typed.resource.Name)
			}
			caseFoldedNames[folded] = typed.resource.Name
		}

		hash := sha256Hex(typed.resource.Data)
		relativeObject := filepath.ToSlash(filepath.Join("objects", hash+".bin"))
		record := tactical3DManifestRecord{
			Type:     typed.typeID,
			Language: typed.resource.Language,
			CodePage: typed.resource.CodePage,
			Reserved: typed.resource.Reserved,
			Size:     len(typed.resource.Data),
			SHA256:   hash,
			Object:   relativeObject,
		}
		if typed.resource.Named {
			record.IdentifierKind = "name"
			record.Name = typed.resource.Name
		} else {
			record.IdentifierKind = "id"
			record.ID = typed.resource.ID
		}
		manifest.Resources = append(manifest.Resources, record)
		prepared = append(prepared, preparedTacticalResource{
			key:    key,
			data:   typed.resource.Data,
			object: filepath.Join(outputDir, filepath.FromSlash(relativeObject)),
		})
	}
	sortTacticalManifest(manifest.Resources)
	manifestBytes, err := json.MarshalIndent(manifest, "", "  ")
	if err != nil {
		return summary, fmt.Errorf("encode tactical 3D manifest: %w", err)
	}
	manifestBytes = append(manifestBytes, '\n')
	manifestPath := filepath.Join(outputDir, "manifest.json")
	manifestMatches := false
	if existing, err := readRegularFileBounded(manifestPath, maxTacticalManifestBytes); err == nil {
		if bytes.Equal(existing, manifestBytes) {
			manifestMatches = true
		} else if !force {
			return summary, fmt.Errorf("tactical 3D manifest already exists with different contents (use --force to replace it)")
		}
	} else if !os.IsNotExist(err) {
		return summary, fmt.Errorf("read tactical 3D manifest: %w", err)
	}

	if err := os.MkdirAll(filepath.Join(outputDir, "objects"), 0o755); err != nil {
		return tactical3DStageSummary{}, fmt.Errorf("create tactical 3D object directory: %w", err)
	}
	for _, resource := range prepared {
		written, err := stageContentAddressedObject(resource.object, resource.data)
		if err != nil {
			return summary, fmt.Errorf("stage tactical resource %s: %w", resource.key, err)
		}
		if written {
			summary.Written++
		} else {
			summary.Skipped++
		}
	}
	if !manifestMatches {
		if err := writeFileAtomically(manifestPath, manifestBytes, 0o644); err != nil {
			return summary, fmt.Errorf("write tactical 3D manifest: %w", err)
		}
	}
	fmt.Fprintf(stdout, "TACTICAL.DLL: staged %d tactical 3D resources (%d written, %d unchanged)\n", summary.Resources, summary.Written, summary.Skipped)
	return summary, nil
}

func verifyTactical3D(outputDir string, expectedMeshes, expectedTextures int, stdout io.Writer) error {
	root := filepath.Join(outputDir, "tactical-dll", "TACTICAL3D")
	manifestBytes, err := readRegularFileBounded(filepath.Join(root, "manifest.json"), maxTacticalManifestBytes)
	if err != nil {
		return fmt.Errorf("read tactical 3D manifest: %w", err)
	}
	var manifest tactical3DManifest
	decoder := json.NewDecoder(bytes.NewReader(manifestBytes))
	decoder.DisallowUnknownFields()
	if err := decoder.Decode(&manifest); err != nil {
		return fmt.Errorf("decode tactical 3D manifest: %w", err)
	}
	if err := decoder.Decode(&struct{}{}); err != io.EOF {
		return fmt.Errorf("tactical 3D manifest has trailing JSON data")
	}
	if manifest.SchemaVersion != tactical3DSchemaVersion || manifest.Source != "TACTICAL.DLL" || !validSHA256(manifest.SourceDLLSHA256) {
		return fmt.Errorf("invalid tactical 3D manifest header")
	}
	if len(manifest.Resources) != expectedMeshes+expectedTextures {
		return fmt.Errorf("tactical manifest has %d resources; expected %d", len(manifest.Resources), expectedMeshes+expectedTextures)
	}
	meshCount := 0
	textureCount := 0
	var aggregateBytes uint64
	seen := make(map[string]struct{}, len(manifest.Resources))
	caseFoldedNames := make(map[string]string)
	for _, record := range manifest.Resources {
		resource, err := rawResourceFromManifest(record)
		if err != nil {
			return err
		}
		key, err := tacticalResourceKey(record.Type, resource)
		if err != nil {
			return err
		}
		if _, exists := seen[key]; exists {
			return fmt.Errorf("duplicate tactical manifest resource %s", key)
		}
		seen[key] = struct{}{}
		if resource.Named {
			folded := fmt.Sprintf("%d:%d:%s", record.Type, resource.Language, strings.ToUpper(resource.Name))
			if prior, exists := caseFoldedNames[folded]; exists && prior != resource.Name {
				return fmt.Errorf("ambiguous case-folded tactical manifest names %q and %q", prior, resource.Name)
			}
			caseFoldedNames[folded] = resource.Name
		}
		if record.Type == tacticalMeshResourceType {
			meshCount++
		} else if record.Type == tacticalTextureResourceType {
			textureCount++
		} else {
			return fmt.Errorf("unsupported tactical manifest resource type %d", record.Type)
		}
		if !validSHA256(record.SHA256) || record.Object != "objects/"+record.SHA256+".bin" {
			return fmt.Errorf("tactical manifest resource %s has invalid object identity", key)
		}
		if record.Size <= 0 || record.Size > maxTacticalRawBytes {
			return fmt.Errorf("tactical manifest resource %s has invalid %d-byte size", key, record.Size)
		}
		if uint64(record.Size) > ^uint64(0)-aggregateBytes {
			return fmt.Errorf("tactical manifest aggregate size overflows")
		}
		aggregateBytes += uint64(record.Size)
		if aggregateBytes > maxTacticalAggregateBytes {
			return fmt.Errorf("tactical manifest aggregate size %d exceeds the %d-byte limit", aggregateBytes, maxTacticalAggregateBytes)
		}
		data, err := readRegularFileBounded(filepath.Join(root, filepath.FromSlash(record.Object)), maxTacticalRawBytes)
		if err != nil {
			return fmt.Errorf("read tactical resource %s: %w", key, err)
		}
		if len(data) != record.Size || sha256Hex(data) != record.SHA256 {
			return fmt.Errorf("tactical resource %s failed size or SHA-256 verification", key)
		}
		resource.Data = data
		if err := validateTacticalRawResource(record.Type, resource); err != nil {
			return err
		}
	}
	if meshCount != expectedMeshes || textureCount != expectedTextures {
		return fmt.Errorf("tactical manifest has %d type-301 and %d type-303 resources; expected %d and %d", meshCount, textureCount, expectedMeshes, expectedTextures)
	}
	fmt.Fprintf(stdout, "Verified %d tactical 3D resources (%d type-301, %d type-303)\n", len(manifest.Resources), meshCount, textureCount)
	return nil
}

func rawResourceFromManifest(record tactical3DManifestRecord) (rawResource, error) {
	resource := rawResource{
		ID:       record.ID,
		Name:     record.Name,
		Language: record.Language,
		CodePage: record.CodePage,
		Reserved: record.Reserved,
	}
	switch record.IdentifierKind {
	case "id":
		if record.Name != "" {
			return rawResource{}, fmt.Errorf("numeric tactical resource includes a name")
		}
	case "name":
		if record.Name == "" || record.ID != 0 {
			return rawResource{}, fmt.Errorf("named tactical resource has invalid identifier fields")
		}
		resource.Named = true
	default:
		return rawResource{}, fmt.Errorf("unknown tactical identifier kind %q", record.IdentifierKind)
	}
	return resource, nil
}

func validateTacticalRawResource(typeID uint32, resource rawResource) error {
	if len(resource.Data) == 0 || len(resource.Data) > maxTacticalRawBytes {
		return fmt.Errorf("tactical type-%d resource has invalid %d-byte size", typeID, len(resource.Data))
	}
	if resource.Named {
		if err := validateTacticalResourceName(resource.Name); err != nil {
			return err
		}
	} else if resource.Name != "" {
		return fmt.Errorf("numeric tactical resource includes a name")
	}
	if typeID == tacticalMeshResourceType {
		if resource.Named {
			return fmt.Errorf("type-301 mesh %q must use a numeric resource ID", resource.Name)
		}
		if len(resource.Data) < 16 {
			return fmt.Errorf("type-301 mesh %d is shorter than its X header", resource.ID)
		}
		header := string(resource.Data[:16])
		compressed := header == "xof 0302bzip0032" || header == "xof 0303bzip0032"
		uncompressed := header == "xof 0302bin 0032" || header == "xof 0303bin 0032"
		if !compressed && !uncompressed {
			return fmt.Errorf("type-301 mesh %d has unsupported X header %q", resource.ID, header)
		}
		if compressed {
			if len(resource.Data) < 20 {
				return fmt.Errorf("compressed type-301 mesh %d lacks its declared size", resource.ID)
			}
			declared := uint64(resource.Data[16]) | uint64(resource.Data[17])<<8 | uint64(resource.Data[18])<<16 | uint64(resource.Data[19])<<24
			if declared < 16 || declared > maxTacticalDecodedMeshBytes {
				return fmt.Errorf("compressed type-301 mesh %d has invalid declared size %d", resource.ID, declared)
			}
		}
	} else if typeID != tacticalTextureResourceType {
		return fmt.Errorf("unsupported tactical raw resource type %d", typeID)
	}
	return nil
}

func validateTacticalResourceName(name string) error {
	if name == "" || name == "." || name == ".." || strings.ContainsAny(name, `/\\`) {
		return fmt.Errorf("unsafe tactical resource name %q", name)
	}
	for _, char := range name {
		if unicode.IsControl(char) {
			return fmt.Errorf("unsafe tactical resource name %q", name)
		}
	}
	return nil
}

func tacticalResourceKey(typeID uint32, resource rawResource) (string, error) {
	if resource.Named {
		if err := validateTacticalResourceName(resource.Name); err != nil {
			return "", err
		}
		return fmt.Sprintf("type=%d:name=%s:language=%d", typeID, resource.Name, resource.Language), nil
	}
	return fmt.Sprintf("type=%d:id=%d:language=%d", typeID, resource.ID, resource.Language), nil
}

func stageContentAddressedObject(path string, data []byte) (bool, error) {
	if existing, err := readRegularFileBounded(path, int64(len(data))); err == nil {
		if bytes.Equal(existing, data) {
			return false, nil
		}
		return false, fmt.Errorf("content-addressed object differs from its SHA-256 path")
	} else if !os.IsNotExist(err) {
		return false, err
	}
	if err := writeFileAtomically(path, data, 0o644); err != nil {
		return false, err
	}
	return true, nil
}

func remainingResourceBudget(limit uint64, resources []rawResource) (uint64, error) {
	used := uint64(0)
	for _, resource := range resources {
		if uint64(len(resource.Data)) > limit-used {
			return 0, fmt.Errorf("aggregate resource size exceeds the %d-byte limit", limit)
		}
		used += uint64(len(resource.Data))
	}
	return limit - used, nil
}

func sortTacticalManifest(records []tactical3DManifestRecord) {
	sort.Slice(records, func(i, j int) bool {
		left, right := records[i], records[j]
		if left.Type != right.Type {
			return left.Type < right.Type
		}
		if left.IdentifierKind != right.IdentifierKind {
			return left.IdentifierKind < right.IdentifierKind
		}
		if left.ID != right.ID {
			return left.ID < right.ID
		}
		if left.Name != right.Name {
			return left.Name < right.Name
		}
		return left.Language < right.Language
	})
}

func sha256Hex(data []byte) string {
	hash := sha256.Sum256(data)
	return hex.EncodeToString(hash[:])
}

func validSHA256(value string) bool {
	if len(value) != sha256.Size*2 {
		return false
	}
	_, err := hex.DecodeString(value)
	return err == nil && value == strings.ToLower(value)
}

func readRegularFileBounded(path string, maxBytes int64) ([]byte, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	info, err := file.Stat()
	if err != nil {
		return nil, err
	}
	if !info.Mode().IsRegular() {
		return nil, fmt.Errorf("not a regular file")
	}
	if info.Size() < 0 || info.Size() > maxBytes {
		return nil, fmt.Errorf("file size %d exceeds the %d-byte limit", info.Size(), maxBytes)
	}
	data, err := io.ReadAll(io.LimitReader(file, maxBytes+1))
	if err != nil {
		return nil, err
	}
	if int64(len(data)) != info.Size() {
		return nil, fmt.Errorf("file changed size while being read")
	}
	return data, nil
}
