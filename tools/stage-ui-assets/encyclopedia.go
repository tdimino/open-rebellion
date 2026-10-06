package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
)

const (
	encyclopediaSchemaVersion = 1
	encyclopediaLanguageID    = 1033
	encyclopediaEncoding      = "windows-1252"
	encyclopediaExpectedTexts = 348
	encyclopediaExpectedMaps  = 191
	encyclopediaJSONByteLimit = 64 * 1024 * 1024
	encyclopediaManifestLimit = 32 * 1024 * 1024
	encyclopediaResourceLimit = 10_000
	encyclopediaBodyByteLimit = 1_048_576
)

var encyclopediaArtworkName = regexp.MustCompile(`^EDATA\.([0-9]{3})$`)

type encyclopediaText struct {
	Body       string `json:"body"`
	BodySHA256 string `json:"body_sha256"`
}

type encyclopediaSource struct {
	SchemaVersion  int                         `json:"schema_version"`
	LanguageID     uint32                      `json:"language_id"`
	Encoding       string                      `json:"encoding"`
	SourceCodePage uint32                      `json:"source_code_page"`
	Texts          map[uint16]encyclopediaText `json:"texts"`
	Artwork        map[uint16]string           `json:"artwork"`
}

type encyclopediaSourceFiles struct {
	TextSHA256    string `json:"encytext_sha256"`
	ArtworkSHA256 string `json:"encybmap_sha256"`
}

type encyclopediaSourceCounts struct {
	Texts   int `json:"texts"`
	Artwork int `json:"artwork_mappings"`
}

type encyclopediaSourceManifest struct {
	SchemaVersion int                      `json:"schema_version"`
	CatalogSHA256 string                   `json:"catalog_sha256"`
	SourceFiles   encyclopediaSourceFiles  `json:"source_files"`
	Counts        encyclopediaSourceCounts `json:"counts"`
}

func decodeEncyclopediaSource(textResources, bitmapResources []rawResource) (encyclopediaSource, error) {
	texts, err := decodeEncyclopediaTexts(textResources)
	if err != nil {
		return encyclopediaSource{}, err
	}
	for _, resource := range bitmapResources {
		if resource.Named || resource.ID == 0 {
			return encyclopediaSource{}, fmt.Errorf("ENCYBMAP has invalid numeric string-bundle ID %d", resource.ID)
		}
		if resource.Language != encyclopediaLanguageID {
			return encyclopediaSource{}, fmt.Errorf("ENCYBMAP string bundle %d has unsupported language %d", resource.ID, resource.Language)
		}
		if resource.CodePage != 0 || resource.Reserved != 0 {
			return encyclopediaSource{}, fmt.Errorf("ENCYBMAP string bundle %d has unsupported PE metadata (code page %d, reserved %d)", resource.ID, resource.CodePage, resource.Reserved)
		}
	}
	bitmapStrings, err := decodeStringResources(bitmapResources)
	if err != nil {
		return encyclopediaSource{}, fmt.Errorf("decode ENCYBMAP: %w", err)
	}
	artwork := make(map[uint16]string, len(bitmapStrings))
	for resourceID, filename := range bitmapStrings {
		if resourceID == 0 || !encyclopediaArtworkName.MatchString(filename) {
			return encyclopediaSource{}, fmt.Errorf("ENCYBMAP resource %d has invalid artwork name %q", resourceID, filename)
		}
		artwork[resourceID] = filename
	}
	return encyclopediaSource{
		SchemaVersion:  encyclopediaSchemaVersion,
		LanguageID:     encyclopediaLanguageID,
		Encoding:       encyclopediaEncoding,
		SourceCodePage: 0,
		Texts:          texts,
		Artwork:        artwork,
	}, nil
}

func decodeEncyclopediaTexts(resources []rawResource) (map[uint16]encyclopediaText, error) {
	if len(resources) == 0 {
		return nil, fmt.Errorf("ENCYTEXT contains no text resources")
	}
	texts := make(map[uint16]encyclopediaText, len(resources))
	for _, resource := range resources {
		if resource.Named || resource.ID == 0 || resource.ID > 0xffff {
			return nil, fmt.Errorf("ENCYTEXT has invalid numeric resource ID %d", resource.ID)
		}
		if resource.Language != encyclopediaLanguageID {
			return nil, fmt.Errorf("ENCYTEXT resource %d has unsupported language %d", resource.ID, resource.Language)
		}
		if resource.CodePage != 0 || resource.Reserved != 0 {
			return nil, fmt.Errorf("ENCYTEXT resource %d has unsupported PE metadata (code page %d, reserved %d)", resource.ID, resource.CodePage, resource.Reserved)
		}
		id := uint16(resource.ID)
		if _, duplicate := texts[id]; duplicate {
			return nil, fmt.Errorf("duplicate ENCYTEXT resource %d", id)
		}
		body, err := decodeWindows1252Text(resource.Data)
		if err != nil {
			return nil, fmt.Errorf("decode ENCYTEXT resource %d: %w", id, err)
		}
		if body == "" {
			return nil, fmt.Errorf("ENCYTEXT resource %d is empty", id)
		}
		texts[id] = encyclopediaText{Body: body, BodySHA256: byteSHA256([]byte(body))}
	}
	return texts, nil
}

func decodeWindows1252Text(raw []byte) (string, error) {
	if len(raw) == 0 || raw[len(raw)-1] != 0 {
		return "", fmt.Errorf("missing terminal NUL")
	}
	end := len(raw)
	for end > 0 && raw[end-1] == 0 {
		end--
	}
	if bytes.IndexByte(raw[:end], 0) >= 0 {
		return "", fmt.Errorf("interior NUL")
	}
	var builder strings.Builder
	for _, value := range raw[:end] {
		if value < 0x80 || value >= 0xa0 {
			builder.WriteRune(rune(value))
			continue
		}
		mapped, ok := windows1252Controls[value]
		if !ok {
			return "", fmt.Errorf("undefined Windows-1252 byte 0x%02x", value)
		}
		builder.WriteRune(mapped)
	}
	return strings.ReplaceAll(strings.ReplaceAll(builder.String(), "\r\n", "\n"), "\r", "\n"), nil
}

var windows1252Controls = map[byte]rune{
	0x80: '\u20ac', 0x82: '\u201a', 0x83: '\u0192', 0x84: '\u201e',
	0x85: '\u2026', 0x86: '\u2020', 0x87: '\u2021', 0x88: '\u02c6',
	0x89: '\u2030', 0x8a: '\u0160', 0x8b: '\u2039', 0x8c: '\u0152',
	0x8e: '\u017d', 0x91: '\u2018', 0x92: '\u2019', 0x93: '\u201c',
	0x94: '\u201d', 0x95: '\u2022', 0x96: '\u2013', 0x97: '\u2014',
	0x98: '\u02dc', 0x99: '\u2122', 0x9a: '\u0161', 0x9b: '\u203a',
	0x9c: '\u0153', 0x9e: '\u017e', 0x9f: '\u0178',
}

func stageEncyclopediaSource(sourceDir, output string, force bool, log io.Writer) error {
	textPath := filepath.Join(sourceDir, "ENCYTEXT.DLL")
	bitmapPath := filepath.Join(sourceDir, "ENCYBMAP.DLL")
	textSnapshot, err := os.ReadFile(textPath)
	if err != nil {
		return err
	}
	bitmapSnapshot, err := os.ReadFile(bitmapPath)
	if err != nil {
		return err
	}
	textResources, err := readPERawResourcesFromBytes(textSnapshot, 10)
	if err != nil {
		return fmt.Errorf("read %s snapshot: %w", filepath.Base(textPath), err)
	}
	bitmapResources, err := readPERawResourcesFromBytes(bitmapSnapshot, 6)
	if err != nil {
		return fmt.Errorf("read %s snapshot: %w", filepath.Base(bitmapPath), err)
	}
	catalog, err := decodeEncyclopediaSource(textResources, bitmapResources)
	if err != nil {
		return err
	}
	if err := validateOwnedEnglishEncyclopediaInventory(catalog); err != nil {
		return err
	}
	catalogBytes, err := json.MarshalIndent(catalog, "", "  ")
	if err != nil {
		return err
	}
	catalogBytes = append(catalogBytes, '\n')
	manifest := encyclopediaSourceManifest{
		SchemaVersion: encyclopediaSchemaVersion,
		CatalogSHA256: byteSHA256(catalogBytes),
		SourceFiles: encyclopediaSourceFiles{
			TextSHA256:    byteSHA256(textSnapshot),
			ArtworkSHA256: byteSHA256(bitmapSnapshot),
		},
		Counts: encyclopediaSourceCounts{Texts: len(catalog.Texts), Artwork: len(catalog.Artwork)},
	}
	manifestBytes, err := json.MarshalIndent(manifest, "", "  ")
	if err != nil {
		return err
	}
	manifestBytes = append(manifestBytes, '\n')
	if err := writeAsset(output, catalogBytes, force); err != nil {
		return err
	}
	if err := writeAsset(output+".manifest.json", manifestBytes, force); err != nil {
		return err
	}
	fmt.Fprintf(log, "Staged %d ENCYTEXT topics and %d ENCYBMAP mappings\n", len(catalog.Texts), len(catalog.Artwork))
	return nil
}

func verifyEncyclopediaSource(output string, log io.Writer) error {
	catalogBytes, err := os.ReadFile(output)
	if err != nil {
		return err
	}
	var catalog encyclopediaSource
	if err := decodeStrictEncyclopediaJSON(catalogBytes, &catalog); err != nil {
		return err
	}
	if err := validateEncyclopediaSource(catalog); err != nil {
		return err
	}
	if err := validateOwnedEnglishEncyclopediaInventory(catalog); err != nil {
		return err
	}
	manifestBytes, err := os.ReadFile(output + ".manifest.json")
	if err != nil {
		return err
	}
	if len(manifestBytes) > encyclopediaManifestLimit {
		return fmt.Errorf("encyclopedia source manifest JSON byte limit exceeded")
	}
	var manifest encyclopediaSourceManifest
	if err := decodeStrictEncyclopediaJSON(manifestBytes, &manifest); err != nil {
		return err
	}
	if manifest.SchemaVersion != encyclopediaSchemaVersion || manifest.CatalogSHA256 != byteSHA256(catalogBytes) {
		return fmt.Errorf("encyclopedia source manifest version or catalog checksum mismatch")
	}
	if manifest.Counts.Texts != len(catalog.Texts) || manifest.Counts.Artwork != len(catalog.Artwork) {
		return fmt.Errorf("encyclopedia source manifest count mismatch")
	}
	if !validSHA256(manifest.SourceFiles.TextSHA256) || !validSHA256(manifest.SourceFiles.ArtworkSHA256) {
		return fmt.Errorf("encyclopedia source manifest has invalid source checksum")
	}
	fmt.Fprintf(log, "Verified %d ENCYTEXT topics and %d ENCYBMAP mappings\n", len(catalog.Texts), len(catalog.Artwork))
	return nil
}

func decodeStrictEncyclopediaJSON(data []byte, destination any) error {
	if len(data) > encyclopediaJSONByteLimit {
		return fmt.Errorf("encyclopedia JSON byte limit exceeded")
	}
	if err := rejectDuplicateJSONKeys(data); err != nil {
		return err
	}
	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	if err := decoder.Decode(destination); err != nil {
		return err
	}
	if err := decoder.Decode(&struct{}{}); err != io.EOF {
		if err == nil {
			return fmt.Errorf("encyclopedia JSON has a trailing value")
		}
		return err
	}
	return nil
}

func rejectDuplicateJSONKeys(data []byte) error {
	decoder := json.NewDecoder(bytes.NewReader(data))
	if err := scanUniqueJSONValue(decoder); err != nil {
		return err
	}
	if _, err := decoder.Token(); err != io.EOF {
		if err == nil {
			return fmt.Errorf("encyclopedia JSON has a trailing value")
		}
		return err
	}
	return nil
}

func scanUniqueJSONValue(decoder *json.Decoder) error {
	token, err := decoder.Token()
	if err != nil {
		return err
	}
	delimiter, ok := token.(json.Delim)
	if !ok {
		return nil
	}
	switch delimiter {
	case '{':
		keys := make(map[string]struct{})
		numericKeys := make(map[uint16]string)
		for decoder.More() {
			keyToken, err := decoder.Token()
			if err != nil {
				return err
			}
			key, ok := keyToken.(string)
			if !ok {
				return fmt.Errorf("encyclopedia JSON object key is not a string")
			}
			if _, duplicate := keys[key]; duplicate {
				return fmt.Errorf("duplicate encyclopedia JSON key %q", key)
			}
			keys[key] = struct{}{}
			if numericKey, err := strconv.ParseUint(key, 10, 16); err == nil {
				resourceID := uint16(numericKey)
				if prior, duplicate := numericKeys[resourceID]; duplicate {
					return fmt.Errorf("duplicate encyclopedia resource identity %d from keys %q and %q", resourceID, prior, key)
				}
				numericKeys[resourceID] = key
			}
			if err := scanUniqueJSONValue(decoder); err != nil {
				return err
			}
		}
	case '[':
		for decoder.More() {
			if err := scanUniqueJSONValue(decoder); err != nil {
				return err
			}
		}
	default:
		return fmt.Errorf("unexpected encyclopedia JSON delimiter %q", delimiter)
	}
	_, err = decoder.Token()
	return err
}

func validateEncyclopediaSource(catalog encyclopediaSource) error {
	if catalog.SchemaVersion != encyclopediaSchemaVersion || catalog.LanguageID != encyclopediaLanguageID || catalog.Encoding != encyclopediaEncoding || catalog.SourceCodePage != 0 {
		return fmt.Errorf("unsupported encyclopedia source profile")
	}
	if len(catalog.Texts) == 0 || len(catalog.Artwork) == 0 {
		return fmt.Errorf("encyclopedia source catalog is empty")
	}
	if len(catalog.Texts) > encyclopediaResourceLimit || len(catalog.Artwork) > encyclopediaResourceLimit {
		return fmt.Errorf("encyclopedia source catalog exceeds the resource limit")
	}
	for id, text := range catalog.Texts {
		if id == 0 || text.Body == "" || len(text.Body) > encyclopediaBodyByteLimit || text.BodySHA256 != byteSHA256([]byte(text.Body)) {
			return fmt.Errorf("invalid encyclopedia text %d", id)
		}
	}
	for id, filename := range catalog.Artwork {
		if id == 0 || !encyclopediaArtworkName.MatchString(filename) {
			return fmt.Errorf("invalid encyclopedia artwork mapping %d", id)
		}
	}
	return nil
}

func validateOwnedEnglishEncyclopediaInventory(catalog encyclopediaSource) error {
	if len(catalog.Texts) != encyclopediaExpectedTexts || len(catalog.Artwork) != encyclopediaExpectedMaps {
		return fmt.Errorf(
			"owned English Encyclopedia inventory mismatch: got %d texts and %d artwork mappings; want %d and %d",
			len(catalog.Texts),
			len(catalog.Artwork),
			encyclopediaExpectedTexts,
			encyclopediaExpectedMaps,
		)
	}
	return nil
}
