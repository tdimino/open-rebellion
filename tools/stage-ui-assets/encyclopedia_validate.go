package main

import (
	"bytes"
	"encoding/binary"
	"encoding/json"
	"fmt"
	"io"
	"path"
	"regexp"
	"strconv"
	"strings"
	"unicode/utf8"
)

type encyclopediaValidationError struct {
	Code   string
	Detail string
}

func (err *encyclopediaValidationError) Error() string {
	if err.Detail == "" {
		return err.Code
	}
	return err.Code + ": " + err.Detail
}

func newEncyclopediaValidationError(code, format string, args ...any) error {
	return &encyclopediaValidationError{Code: code, Detail: fmt.Sprintf(format, args...)}
}

func encyclopediaValidationCode(err error) string {
	if err == nil {
		return "ok"
	}
	if validationErr, ok := err.(*encyclopediaValidationError); ok {
		return validationErr.Code
	}
	return "internal_error"
}

type encyclopediaManifest struct {
	SchemaVersion    int                                         `json:"schema_version"`
	SourceProfile    string                                      `json:"source_profile"`
	ExtractorVersion string                                      `json:"extractor_version"`
	CatalogSHA256    string                                      `json:"catalog_sha256"`
	Files            map[string]string                           `json:"files"`
	BindingSources   []encyclopediaManifestBindingSource         `json:"binding_sources"`
	SourceRecords    map[string]encyclopediaManifestSourceRecord `json:"source_records"`
}

type encyclopediaManifestBindingSource struct {
	Basename string `json:"basename"`
	SHA256   string `json:"sha256"`
}

type encyclopediaManifestSourceRecord struct {
	SourceBasename   string                                 `json:"source_basename"`
	SourceSHA256     string                                 `json:"source_sha256"`
	ResourceType     encyclopediaManifestResourceIdentifier `json:"resource_type"`
	ResourceID       encyclopediaManifestResourceIdentifier `json:"resource_id"`
	LanguageID       uint32                                 `json:"language_id"`
	RawLength        uint64                                 `json:"raw_length"`
	RawSHA256        string                                 `json:"raw_sha256"`
	Decoder          string                                 `json:"decoder"`
	Encoding         string                                 `json:"encoding"`
	MappingCitations []string                               `json:"mapping_citations"`
}

type encyclopediaManifestResourceIdentifier struct {
	Kind  string          `json:"kind"`
	Value json.RawMessage `json:"value"`
}

type encyclopediaAssetFacts struct {
	Bytes      []byte
	SHA256     string
	ByteLength uint64
	Format     string
	Width      uint32
	Height     uint32
}

var (
	encyclopediaLangIDPattern    = regexp.MustCompile(`^(0|[1-9][0-9]{0,4})$`)
	encyclopediaStableIDPattern  = regexp.MustCompile(`^[a-z][a-z0-9._-]{0,31}:[A-Za-z0-9][A-Za-z0-9._:-]{0,223}$`)
	encyclopediaBaseImagePattern = regexp.MustCompile(`^edata:(0|[1-9][0-9]{0,9})$`)
	encyclopediaFamilyPattern    = regexp.MustCompile(`^[a-z][a-z0-9_]*$`)
	encyclopediaTokenPattern     = regexp.MustCompile(`^[A-Za-z0-9][A-Za-z0-9._-]{0,127}$`)
	encyclopediaSourceRefPattern = regexp.MustCompile(`^[A-Za-z0-9][A-Za-z0-9._:/-]{0,255}$`)
)

func encyclopediaAssetFactsFromBytes(data []byte, _ string) encyclopediaAssetFacts {
	facts := encyclopediaAssetFacts{
		Bytes: append([]byte(nil), data...), SHA256: byteSHA256(data), ByteLength: uint64(len(data)),
	}
	if len(data) >= 2 && data[0] == 'B' && data[1] == 'M' {
		facts.Format = "bmp"
		if inspected, err := inspectEncyclopediaBMP(data, defaultEncyclopediaImageLimits()); err == nil {
			facts.Width, facts.Height = inspected.Width, inspected.Height
		}
	} else if len(data) >= 24 && bytes.Equal(data[:8], []byte{137, 80, 78, 71, 13, 10, 26, 10}) && string(data[12:16]) == "IHDR" {
		facts.Format = "png"
		facts.Width = binary.BigEndian.Uint32(data[16:20])
		facts.Height = binary.BigEndian.Uint32(data[20:24])
	}
	return facts
}

func parseEncyclopediaCatalog(data []byte) (encyclopediaCatalog, error) {
	if err := validateRawEncyclopediaJSON(data, encyclopediaMaxCatalogJSONBytes, encyclopediaMaxCatalogJSONDepth); err != nil {
		return encyclopediaCatalog{}, err
	}
	if err := validateEncyclopediaCatalogWireShape(data); err != nil {
		return encyclopediaCatalog{}, err
	}
	var catalog encyclopediaCatalog
	if err := decodeStrictEncyclopediaJSON(data, &catalog); err != nil {
		return encyclopediaCatalog{}, classifyEncyclopediaJSONError(err)
	}
	if err := validateEncyclopediaCatalog(catalog); err != nil {
		return encyclopediaCatalog{}, err
	}
	return catalog, nil
}

func parseEncyclopediaManifest(data []byte) (encyclopediaManifest, error) {
	if err := validateRawEncyclopediaJSON(data, encyclopediaMaxManifestJSONBytes, encyclopediaMaxManifestJSONDepth); err != nil {
		return encyclopediaManifest{}, err
	}
	if err := validateEncyclopediaManifestWireShape(data); err != nil {
		return encyclopediaManifest{}, err
	}
	var manifest encyclopediaManifest
	if err := decodeStrictEncyclopediaJSON(data, &manifest); err != nil {
		return encyclopediaManifest{}, classifyEncyclopediaJSONError(err)
	}
	if err := validateEncyclopediaManifest(manifest); err != nil {
		return encyclopediaManifest{}, err
	}
	return manifest, nil
}

func validateEncyclopediaCatalogWireShape(data []byte) error {
	root, err := exactEncyclopediaJSONObject(data, "catalog", []string{"schema_version", "default_language", "topic_sort", "index", "categories", "topics", "images", "bindings"}, nil)
	if err != nil {
		return err
	}
	if _, err := encyclopediaJSONUint(root["schema_version"], "catalog.schema_version"); err != nil {
		return err
	}
	if _, err := encyclopediaJSONString(root["default_language"], "catalog.default_language"); err != nil {
		return err
	}
	if err := validateEncyclopediaTopicSortWire(root["topic_sort"]); err != nil {
		return err
	}
	if err := validateEncyclopediaViewWire(root["index"], "catalog.index", false); err != nil {
		return err
	}
	categories, err := encyclopediaJSONArray(root["categories"], "catalog.categories")
	if err != nil {
		return err
	}
	for index, raw := range categories {
		if err := validateEncyclopediaViewWire(raw, fmt.Sprintf("catalog.categories[%d]", index), true); err != nil {
			return err
		}
	}
	topics, err := encyclopediaJSONObject(root["topics"], "catalog.topics")
	if err != nil {
		return err
	}
	for topicID, raw := range topics {
		if err := validateEncyclopediaTopicWire(raw, "catalog.topics."+topicID); err != nil {
			return err
		}
	}
	images, err := encyclopediaJSONObject(root["images"], "catalog.images")
	if err != nil {
		return err
	}
	for imageID, raw := range images {
		if err := validateEncyclopediaImageWire(raw, "catalog.images."+imageID); err != nil {
			return err
		}
	}
	bindings, err := encyclopediaJSONArray(root["bindings"], "catalog.bindings")
	if err != nil {
		return err
	}
	for index, raw := range bindings {
		if err := validateEncyclopediaBindingWire(raw, fmt.Sprintf("catalog.bindings[%d]", index)); err != nil {
			return err
		}
	}
	return nil
}

func validateEncyclopediaTopicSortWire(raw json.RawMessage) error {
	object, err := exactEncyclopediaJSONObject(raw, "catalog.topic_sort", []string{"algorithm", "representable_encoding", "representable_fold", "unrepresentable", "tie_break"}, nil)
	if err != nil {
		return err
	}
	for _, field := range []string{"algorithm", "representable_encoding", "representable_fold", "unrepresentable", "tie_break"} {
		if _, err := encyclopediaJSONString(object[field], "catalog.topic_sort."+field); err != nil {
			return err
		}
	}
	return nil
}

func validateEncyclopediaViewWire(raw json.RawMessage, scope string, category bool) error {
	required := []string{"command", "labels", "topic_ids", "source_ref"}
	if category {
		required = append([]string{"id"}, required...)
	}
	object, err := exactEncyclopediaJSONObject(raw, scope, required, nil)
	if err != nil {
		return err
	}
	if category {
		if _, err := encyclopediaJSONString(object["id"], scope+".id"); err != nil {
			return err
		}
	}
	if _, err := encyclopediaJSONString(object["command"], scope+".command"); err != nil {
		return err
	}
	if err := validateEncyclopediaLabelsWire(object["labels"], scope+".labels"); err != nil {
		return err
	}
	if err := validateEncyclopediaStringArrayWire(object["topic_ids"], scope+".topic_ids"); err != nil {
		return err
	}
	_, err = encyclopediaJSONString(object["source_ref"], scope+".source_ref")
	return err
}

func validateEncyclopediaLabelsWire(raw json.RawMessage, scope string) error {
	labels, err := encyclopediaJSONObject(raw, scope)
	if err != nil {
		return err
	}
	for languageID, rawLabel := range labels {
		if _, err := encyclopediaJSONString(rawLabel, scope+"."+languageID); err != nil {
			return err
		}
	}
	return nil
}

func validateEncyclopediaStringArrayWire(raw json.RawMessage, scope string) error {
	values, err := encyclopediaJSONArray(raw, scope)
	if err != nil {
		return err
	}
	for index, value := range values {
		if _, err := encyclopediaJSONString(value, fmt.Sprintf("%s[%d]", scope, index)); err != nil {
			return err
		}
	}
	return nil
}

func validateEncyclopediaTopicWire(raw json.RawMessage, scope string) error {
	object, err := exactEncyclopediaJSONObject(raw, scope, []string{"localized", "source_ref"}, nil)
	if err != nil {
		return err
	}
	localized, err := encyclopediaJSONObject(object["localized"], scope+".localized")
	if err != nil {
		return err
	}
	for languageID, rawContent := range localized {
		if err := validateEncyclopediaLocalizedContentWire(rawContent, scope+".localized."+languageID); err != nil {
			return err
		}
	}
	_, err = encyclopediaJSONString(object["source_ref"], scope+".source_ref")
	return err
}

func validateEncyclopediaLocalizedContentWire(raw json.RawMessage, scope string) error {
	object, err := exactEncyclopediaJSONObject(raw, scope, []string{"title", "body"}, []string{"image_id", "image_selector"})
	if err != nil {
		return err
	}
	if _, err := encyclopediaJSONString(object["title"], scope+".title"); err != nil {
		return err
	}
	if _, err := encyclopediaJSONString(object["body"], scope+".body"); err != nil {
		return err
	}
	imageID, hasImageID := object["image_id"]
	selector, hasSelector := object["image_selector"]
	if hasImageID && hasSelector {
		return newEncyclopediaValidationError("ambiguous_image_selector", "%s has both image_id and image_selector", scope)
	}
	if hasImageID && !encyclopediaJSONNull(imageID) {
		if _, err := encyclopediaJSONString(imageID, scope+".image_id"); err != nil {
			return err
		}
	}
	if hasSelector {
		return validateEncyclopediaImageSelectorWire(selector, scope+".image_selector")
	}
	return nil
}

func validateEncyclopediaImageSelectorWire(raw json.RawMessage, scope string) error {
	object, err := exactEncyclopediaJSONObject(raw, scope, []string{"kind", "alliance_image_id", "empire_image_id"}, nil)
	if err != nil {
		return err
	}
	if _, err := encyclopediaJSONString(object["kind"], scope+".kind"); err != nil {
		return err
	}
	for _, field := range []string{"alliance_image_id", "empire_image_id"} {
		if !encyclopediaJSONNull(object[field]) {
			if _, err := encyclopediaJSONString(object[field], scope+"."+field); err != nil {
				return err
			}
		}
	}
	return nil
}

func validateEncyclopediaImageWire(raw json.RawMessage, scope string) error {
	object, err := exactEncyclopediaJSONObject(raw, scope, []string{"path", "format", "byte_length", "width", "height", "sha256", "source_ref"}, nil)
	if err != nil {
		return err
	}
	for _, field := range []string{"path", "format", "sha256", "source_ref"} {
		if _, err := encyclopediaJSONString(object[field], scope+"."+field); err != nil {
			return err
		}
	}
	for _, field := range []string{"byte_length", "width", "height"} {
		if _, err := encyclopediaJSONUint(object[field], scope+"."+field); err != nil {
			return err
		}
	}
	return nil
}

func validateEncyclopediaBindingWire(raw json.RawMessage, scope string) error {
	object, err := exactEncyclopediaJSONObject(raw, scope, []string{"family", "dat_id", "variant", "topic_id"}, nil)
	if err != nil {
		return err
	}
	for _, field := range []string{"family", "variant", "topic_id"} {
		if _, err := encyclopediaJSONString(object[field], scope+"."+field); err != nil {
			return err
		}
	}
	_, err = encyclopediaJSONUint(object["dat_id"], scope+".dat_id")
	return err
}

func validateEncyclopediaManifestWireShape(data []byte) error {
	root, err := exactEncyclopediaJSONObject(data, "manifest", []string{"schema_version", "source_profile", "extractor_version", "catalog_sha256", "files", "binding_sources", "source_records"}, nil)
	if err != nil {
		return err
	}
	if _, err := encyclopediaJSONUint(root["schema_version"], "manifest.schema_version"); err != nil {
		return err
	}
	for _, field := range []string{"source_profile", "extractor_version", "catalog_sha256"} {
		if _, err := encyclopediaJSONString(root[field], "manifest."+field); err != nil {
			return err
		}
	}
	files, err := encyclopediaJSONObject(root["files"], "manifest.files")
	if err != nil {
		return err
	}
	for filePath, digest := range files {
		if _, err := encyclopediaJSONString(digest, "manifest.files."+filePath); err != nil {
			return err
		}
	}
	bindingSources, err := encyclopediaJSONArray(root["binding_sources"], "manifest.binding_sources")
	if err != nil {
		return err
	}
	for index, raw := range bindingSources {
		if err := validateEncyclopediaBindingSourceWire(raw, fmt.Sprintf("manifest.binding_sources[%d]", index)); err != nil {
			return err
		}
	}
	sourceRecords, err := encyclopediaJSONObject(root["source_records"], "manifest.source_records")
	if err != nil {
		return err
	}
	for sourceRef, raw := range sourceRecords {
		if err := validateEncyclopediaSourceRecordWire(raw, "manifest.source_records."+sourceRef); err != nil {
			return err
		}
	}
	return nil
}

func validateEncyclopediaBindingSourceWire(raw json.RawMessage, scope string) error {
	object, err := exactEncyclopediaJSONObject(raw, scope, []string{"basename", "sha256"}, nil)
	if err != nil {
		return err
	}
	for _, field := range []string{"basename", "sha256"} {
		if _, err := encyclopediaJSONString(object[field], scope+"."+field); err != nil {
			return err
		}
	}
	return nil
}

func validateEncyclopediaSourceRecordWire(raw json.RawMessage, scope string) error {
	required := []string{"source_basename", "source_sha256", "resource_type", "resource_id", "language_id", "raw_length", "raw_sha256", "decoder", "encoding", "mapping_citations"}
	object, err := exactEncyclopediaJSONObject(raw, scope, required, nil)
	if err != nil {
		return err
	}
	for _, field := range []string{"source_basename", "source_sha256", "raw_sha256", "decoder", "encoding"} {
		if _, err := encyclopediaJSONString(object[field], scope+"."+field); err != nil {
			return err
		}
	}
	for _, field := range []string{"language_id", "raw_length"} {
		if _, err := encyclopediaJSONUint(object[field], scope+"."+field); err != nil {
			return err
		}
	}
	for _, field := range []string{"resource_type", "resource_id"} {
		if err := validateEncyclopediaResourceIdentifierWire(object[field], scope+"."+field); err != nil {
			return err
		}
	}
	return validateEncyclopediaStringArrayWire(object["mapping_citations"], scope+".mapping_citations")
}

func validateEncyclopediaResourceIdentifierWire(raw json.RawMessage, scope string) error {
	object, err := exactEncyclopediaJSONObject(raw, scope, []string{"kind", "value"}, nil)
	if err != nil {
		return err
	}
	kind, err := encyclopediaJSONString(object["kind"], scope+".kind")
	if err != nil {
		return err
	}
	switch kind {
	case "numeric":
		_, err = encyclopediaJSONUint(object["value"], scope+".value")
	case "named":
		_, err = encyclopediaJSONString(object["value"], scope+".value")
	default:
		return newEncyclopediaValidationError("invalid_source_record", "%s has unknown identifier kind %q", scope, kind)
	}
	return err
}

func exactEncyclopediaJSONObject(raw []byte, scope string, required, optional []string) (map[string]json.RawMessage, error) {
	object, err := encyclopediaJSONObject(raw, scope)
	if err != nil {
		return nil, err
	}
	allowed := make(map[string]struct{}, len(required)+len(optional))
	for _, field := range required {
		allowed[field] = struct{}{}
	}
	for _, field := range optional {
		allowed[field] = struct{}{}
	}
	for field := range object {
		if _, ok := allowed[field]; !ok {
			return nil, newEncyclopediaValidationError("unknown_field", "%s contains %q", scope, field)
		}
	}
	for _, field := range required {
		if _, ok := object[field]; !ok {
			return nil, newEncyclopediaValidationError("missing_field", "%s omits %q", scope, field)
		}
	}
	return object, nil
}

func encyclopediaJSONObject(raw []byte, scope string) (map[string]json.RawMessage, error) {
	if encyclopediaJSONNull(raw) {
		return nil, newEncyclopediaValidationError("invalid_type", "%s must be an object, not null", scope)
	}
	var object map[string]json.RawMessage
	if err := json.Unmarshal(raw, &object); err != nil || object == nil {
		return nil, newEncyclopediaValidationError("invalid_type", "%s must be an object", scope)
	}
	return object, nil
}

func encyclopediaJSONArray(raw []byte, scope string) ([]json.RawMessage, error) {
	if encyclopediaJSONNull(raw) {
		return nil, newEncyclopediaValidationError("invalid_type", "%s must be an array, not null", scope)
	}
	var values []json.RawMessage
	if err := json.Unmarshal(raw, &values); err != nil || values == nil {
		return nil, newEncyclopediaValidationError("invalid_type", "%s must be an array", scope)
	}
	return values, nil
}

func encyclopediaJSONString(raw []byte, scope string) (string, error) {
	if encyclopediaJSONNull(raw) {
		return "", newEncyclopediaValidationError("invalid_type", "%s must be a string, not null", scope)
	}
	var value string
	if err := json.Unmarshal(raw, &value); err != nil {
		return "", newEncyclopediaValidationError("invalid_type", "%s must be a string", scope)
	}
	return value, nil
}

func encyclopediaJSONUint(raw []byte, scope string) (uint64, error) {
	if encyclopediaJSONNull(raw) {
		return 0, newEncyclopediaValidationError("invalid_type", "%s must be a nonnegative integer, not null", scope)
	}
	var value uint64
	if err := json.Unmarshal(raw, &value); err != nil {
		return 0, newEncyclopediaValidationError("invalid_type", "%s must be a nonnegative integer", scope)
	}
	return value, nil
}

func encyclopediaJSONNull(raw []byte) bool {
	return bytes.Equal(bytes.TrimSpace(raw), []byte("null"))
}

func marshalEncyclopediaManifest(manifest encyclopediaManifest) ([]byte, error) {
	if err := validateEncyclopediaManifest(manifest); err != nil {
		return nil, err
	}
	data, err := json.MarshalIndent(manifest, "", "  ")
	if err != nil {
		return nil, fmt.Errorf("marshal encyclopedia manifest: %w", err)
	}
	return append(data, '\n'), nil
}

func validateEncyclopediaRuntime(catalogBytes, manifestBytes []byte, files map[string]encyclopediaAssetFacts) error {
	catalog, err := parseEncyclopediaCatalog(catalogBytes)
	if err != nil {
		return err
	}
	manifest, err := parseEncyclopediaManifest(manifestBytes)
	if err != nil {
		return err
	}
	if err := validateEncyclopediaSourceRefClosure(catalog, manifest); err != nil {
		return err
	}

	wantPaths := map[string]struct{}{"catalog.json": {}}
	for _, image := range catalog.Images {
		wantPaths[image.Path] = struct{}{}
	}
	if len(manifest.Files) != len(wantPaths) {
		return newEncyclopediaValidationError("manifest_file_set_mismatch", "manifest has %d files, catalog requires %d", len(manifest.Files), len(wantPaths))
	}
	for filePath := range wantPaths {
		if _, ok := manifest.Files[filePath]; !ok {
			return newEncyclopediaValidationError("manifest_file_set_mismatch", "manifest omits %q", filePath)
		}
	}
	if len(files) != len(wantPaths) {
		return newEncyclopediaValidationError("manifest_file_set_mismatch", "observed file set has %d files, want %d", len(files), len(wantPaths))
	}
	for filePath := range files {
		if _, ok := wantPaths[filePath]; !ok {
			return newEncyclopediaValidationError("manifest_file_set_mismatch", "observed unexpected file %q", filePath)
		}
	}

	catalogDigest := byteSHA256(catalogBytes)
	if manifest.CatalogSHA256 != catalogDigest || manifest.Files["catalog.json"] != catalogDigest {
		return newEncyclopediaValidationError("catalog_digest_mismatch", "catalog bytes do not match both manifest catalog digests")
	}
	if facts, ok := files["catalog.json"]; !ok || facts.ByteLength != uint64(len(catalogBytes)) || facts.SHA256 != catalogDigest || !bytes.Equal(facts.Bytes, catalogBytes) {
		return newEncyclopediaValidationError("file_digest_mismatch", "catalog facts do not match catalog bytes")
	}

	var aggregateImageBytes uint64
	for imageID, descriptor := range catalog.Images {
		if descriptor.Format != "bmp" {
			return newEncyclopediaValidationError("unsupported_base_image_format", "%s declares %q; immutable v1 base art must be bmp", imageID, descriptor.Format)
		}
		facts, ok := files[descriptor.Path]
		if !ok {
			return newEncyclopediaValidationError("manifest_file_set_mismatch", "missing observed bytes for %q", descriptor.Path)
		}
		if facts.Format != descriptor.Format {
			return newEncyclopediaValidationError("image_format_mismatch", "%s declares %q but bytes are %q", imageID, descriptor.Format, facts.Format)
		}
		if facts.ByteLength > encyclopediaMaxImageBytes {
			return newEncyclopediaValidationError("resource_limit:image_bytes", "%s has %d bytes", imageID, facts.ByteLength)
		}
		inspected, inspectErr := inspectEncyclopediaBMP(facts.Bytes, defaultEncyclopediaImageLimits())
		if inspectErr != nil {
			return newEncyclopediaValidationError("invalid_image", "%s: %v", imageID, inspectErr)
		}
		actualDigest := byteSHA256(facts.Bytes)
		if facts.ByteLength != uint64(len(facts.Bytes)) || facts.SHA256 != actualDigest || manifest.Files[descriptor.Path] != actualDigest {
			return newEncyclopediaValidationError("file_digest_mismatch", "%s file facts or manifest digest differ", imageID)
		}
		if descriptor.SHA256 != actualDigest {
			return newEncyclopediaValidationError("image_digest_mismatch", "%s descriptor digest differs", imageID)
		}
		if facts.Width != inspected.Width || facts.Height != inspected.Height || descriptor.ByteLength != facts.ByteLength || descriptor.Width != inspected.Width || descriptor.Height != inspected.Height {
			return newEncyclopediaValidationError("image_facts_mismatch", "%s descriptor facts differ from decoded bytes", imageID)
		}
		if aggregateImageBytes > encyclopediaMaxAggregateImageBytes-facts.ByteLength {
			return newEncyclopediaValidationError("resource_limit:effective_image_bytes", "base image set exceeds %d bytes", encyclopediaMaxAggregateImageBytes)
		}
		aggregateImageBytes += facts.ByteLength
	}
	return nil
}

func verifyBindingSources(manifest encyclopediaManifest, observedDATHashes map[string]string) error {
	observed := make(map[string]string, len(observedDATHashes))
	for basename, digest := range observedDATHashes {
		folded := strings.ToLower(basename)
		if _, duplicate := observed[folded]; duplicate {
			return newEncyclopediaValidationError("binding_source_mismatch", "ambiguous observed DAT basename %q", basename)
		}
		observed[folded] = digest
	}
	for _, source := range manifest.BindingSources {
		digest, ok := observed[strings.ToLower(source.Basename)]
		if !ok || digest != source.SHA256 {
			return newEncyclopediaValidationError("binding_source_mismatch", "%s does not match manifest source pairing", source.Basename)
		}
	}
	return nil
}

func validateEncyclopediaCatalog(catalog encyclopediaCatalog) error {
	if catalog.SchemaVersion != encyclopediaCatalogSchemaVersion {
		return newEncyclopediaValidationError("unsupported_version", "catalog schema_version = %d", catalog.SchemaVersion)
	}
	if !validEncyclopediaLangID(catalog.DefaultLanguage) {
		return newEncyclopediaValidationError("invalid_language_id", "default language %q", catalog.DefaultLanguage)
	}
	if catalog.TopicSort != approvedEncyclopediaTopicSort() {
		return newEncyclopediaValidationError("invalid_topic_sort", "catalog topic_sort is not the approved v1 rule")
	}
	if len(catalog.Topics) == 0 {
		return newEncyclopediaValidationError("missing_field", "topics must be nonempty")
	}
	if len(catalog.Topics) > encyclopediaMaxTopics {
		return newEncyclopediaValidationError("resource_limit:topics", "catalog has %d topics", len(catalog.Topics))
	}
	if len(catalog.Images) > 20_000 || len(catalog.Bindings) > 40_000 {
		return newEncyclopediaValidationError("resource_limit:catalog_entries", "catalog has %d images and %d bindings", len(catalog.Images), len(catalog.Bindings))
	}
	if catalog.Index.Command != "0x6f" {
		return newEncyclopediaValidationError("category_order", "index command is %q", catalog.Index.Command)
	}
	if err := validateEncyclopediaLabels(catalog.Index.Labels); err != nil {
		return err
	}
	if !validEncyclopediaSourceRef(catalog.Index.SourceRef) {
		return newEncyclopediaValidationError("invalid_source_ref", "invalid index source_ref")
	}
	if len(catalog.Categories) != 6 {
		return newEncyclopediaValidationError("category_order", "filtered category count = %d", len(catalog.Categories))
	}
	seenCommands := map[string]struct{}{"0x6f": {}}
	for index, category := range catalog.Categories {
		wantCommand := fmt.Sprintf("0x%x", 0x70+index)
		if _, duplicate := seenCommands[category.Command]; duplicate {
			return newEncyclopediaValidationError("duplicate_category_command", "command %s repeats", category.Command)
		}
		seenCommands[category.Command] = struct{}{}
		if category.Command != wantCommand || category.ID != "command:"+wantCommand {
			return newEncyclopediaValidationError("category_order", "category %d is %q/%q, want %q", index, category.ID, category.Command, wantCommand)
		}
		if err := validateEncyclopediaLabels(category.Labels); err != nil {
			return err
		}
		if !validEncyclopediaSourceRef(category.SourceRef) {
			return newEncyclopediaValidationError("invalid_source_ref", "category %s source_ref", category.Command)
		}
	}

	for imageID := range catalog.Images {
		if !encyclopediaBaseImagePattern.MatchString(imageID) {
			return newEncyclopediaValidationError("invalid_base_image_id", "%q", imageID)
		}
	}

	for topicID, topic := range catalog.Topics {
		if !validEncyclopediaStableID(topicID) {
			return newEncyclopediaValidationError("invalid_topic_id", "%q", topicID)
		}
		if !validEncyclopediaSourceRef(topic.SourceRef) {
			return newEncyclopediaValidationError("invalid_source_ref", "topic %s", topicID)
		}
		if len(topic.Localized) == 0 || len(topic.Localized) > 256 {
			return newEncyclopediaValidationError("invalid_localized_record", "topic %s has %d languages", topicID, len(topic.Localized))
		}
		for language, localized := range topic.Localized {
			if !validEncyclopediaLangID(language) {
				return newEncyclopediaValidationError("invalid_language_id", "topic %s language %q", topicID, language)
			}
			if len([]byte(localized.Title)) > encyclopediaMaxTitleBytes {
				return newEncyclopediaValidationError("resource_limit:title_bytes", "topic %s language %s", topicID, language)
			}
			if len([]byte(localized.Body)) > encyclopediaMaxBodyBytes {
				return newEncyclopediaValidationError("resource_limit:body_bytes", "topic %s language %s", topicID, language)
			}
			if localized.ImageID != nil && localized.ImageSelector != nil {
				return newEncyclopediaValidationError("ambiguous_image_selector", "topic %s language %s", topicID, language)
			}
			if localized.ImageID != nil {
				if !encyclopediaBaseImagePattern.MatchString(*localized.ImageID) {
					return newEncyclopediaValidationError("invalid_base_image_id", "%q", *localized.ImageID)
				}
				if _, ok := catalog.Images[*localized.ImageID]; !ok {
					return newEncyclopediaValidationError("dangling_image_reference", "topic %s refers to %s", topicID, *localized.ImageID)
				}
			}
			if localized.ImageSelector != nil {
				selector := localized.ImageSelector
				if selector.Kind != "viewer_faction" {
					return newEncyclopediaValidationError("unsupported_selector", "topic %s selector %q", topicID, selector.Kind)
				}
				if !selector.AllianceImageID.Present || !selector.EmpireImageID.Present {
					return newEncyclopediaValidationError("missing_field", "topic %s faction selector is incomplete", topicID)
				}
				for _, imageID := range []*string{selector.AllianceImageID.Value, selector.EmpireImageID.Value} {
					if imageID == nil {
						continue
					}
					if _, ok := catalog.Images[*imageID]; !ok {
						return newEncyclopediaValidationError("dangling_image_reference", "topic %s refers to %s", topicID, *imageID)
					}
				}
			}
		}
	}

	referencedImages := make(map[string]struct{})
	for _, topic := range catalog.Topics {
		for _, localized := range topic.Localized {
			if localized.ImageID != nil {
				referencedImages[*localized.ImageID] = struct{}{}
			}
			if localized.ImageSelector != nil {
				if localized.ImageSelector.AllianceImageID.Value != nil {
					referencedImages[*localized.ImageSelector.AllianceImageID.Value] = struct{}{}
				}
				if localized.ImageSelector.EmpireImageID.Value != nil {
					referencedImages[*localized.ImageSelector.EmpireImageID.Value] = struct{}{}
				}
			}
		}
	}
	var describedImageBytes uint64
	seenPaths := make(map[string]string, len(catalog.Images))
	for imageID, image := range catalog.Images {
		if !encyclopediaBaseImagePattern.MatchString(imageID) {
			return newEncyclopediaValidationError("invalid_base_image_id", "%q", imageID)
		}
		if _, used := referencedImages[imageID]; !used {
			return newEncyclopediaValidationError("unexpected_runtime_file", "unreferenced base image %s", imageID)
		}
		if !validEncyclopediaAssetPath(image.Path) {
			return newEncyclopediaValidationError("unsafe_asset_path", "%q", image.Path)
		}
		if previous, duplicate := seenPaths[image.Path]; duplicate && previous != imageID {
			return newEncyclopediaValidationError("asset_identity_collision", "%s and %s share %s", previous, imageID, image.Path)
		}
		seenPaths[image.Path] = imageID
		if image.Format != "bmp" && image.Format != "png" {
			return newEncyclopediaValidationError("unsupported_image_format", "%q", image.Format)
		}
		if image.ByteLength == 0 || image.ByteLength > encyclopediaMaxImageBytes {
			return newEncyclopediaValidationError("resource_limit:image_bytes", "%s has %d bytes", imageID, image.ByteLength)
		}
		pixels := uint64(image.Width) * uint64(image.Height)
		if image.Width == 0 || image.Height == 0 || pixels > encyclopediaMaxImagePixels {
			return newEncyclopediaValidationError("resource_limit:image_pixels", "%s has %d pixels", imageID, pixels)
		}
		if !validSHA256(image.SHA256) || !validEncyclopediaSourceRef(image.SourceRef) {
			return newEncyclopediaValidationError("invalid_image", "%s has invalid digest/provenance", imageID)
		}
		if describedImageBytes > encyclopediaMaxAggregateImageBytes-image.ByteLength {
			return newEncyclopediaValidationError("resource_limit:effective_image_bytes", "base descriptors exceed aggregate image limit")
		}
		describedImageBytes += image.ByteLength
	}

	indexTopics := make(map[string]struct{}, len(catalog.Index.TopicIDs))
	if err := validateCatalogTopicIDList(catalog.Index.TopicIDs, catalog.Topics, indexTopics); err != nil {
		return err
	}
	if len(indexTopics) != len(catalog.Topics) {
		return newEncyclopediaValidationError("potential_membership_mismatch", "index has %d of %d catalog candidates", len(indexTopics), len(catalog.Topics))
	}
	filteredTopics := make(map[string]string)
	for _, category := range catalog.Categories {
		seen := make(map[string]struct{}, len(category.TopicIDs))
		if err := validateCatalogTopicIDList(category.TopicIDs, catalog.Topics, seen); err != nil {
			return err
		}
		for topicID := range seen {
			if _, indexed := indexTopics[topicID]; !indexed {
				return newEncyclopediaValidationError("potential_membership_mismatch", "%s is filtered but absent from index", topicID)
			}
			if previous, duplicate := filteredTopics[topicID]; duplicate {
				return newEncyclopediaValidationError("filtered_membership_mismatch", "%s appears in %s and %s", topicID, previous, category.Command)
			}
			filteredTopics[topicID] = category.Command
		}
	}

	seenBindings := make(map[string]struct{}, len(catalog.Bindings))
	boundTopics := make(map[string]encyclopediaCatalogBinding, len(catalog.Bindings))
	for _, binding := range catalog.Bindings {
		if !encyclopediaFamilyPattern.MatchString(binding.Family) || (binding.Variant != "default" && binding.Variant != "viewer_faction") {
			return newEncyclopediaValidationError("invalid_binding", "%s/%d/%s", binding.Family, binding.DatID, binding.Variant)
		}
		if _, ok := catalog.Topics[binding.TopicID]; !ok {
			return newEncyclopediaValidationError("dangling_topic_reference", "binding refers to %s", binding.TopicID)
		}
		tuple := fmt.Sprintf("%s\x00%d\x00%s", binding.Family, binding.DatID, binding.Variant)
		if _, duplicate := seenBindings[tuple]; duplicate {
			return newEncyclopediaValidationError("ambiguous_binding", "%s", tuple)
		}
		seenBindings[tuple] = struct{}{}
		if previous, duplicate := boundTopics[binding.TopicID]; duplicate && previous != binding {
			return newEncyclopediaValidationError("ambiguous_binding", "topic %s has multiple binding tuples", binding.TopicID)
		}
		boundTopics[binding.TopicID] = binding
	}
	for topicID, topic := range catalog.Topics {
		binding, ok := boundTopics[topicID]
		if !ok {
			return newEncyclopediaValidationError("dangling_topic_reference", "topic %s has no binding", topicID)
		}
		for _, localized := range topic.Localized {
			isFaction := localized.ImageSelector != nil
			if (binding.Variant == "viewer_faction") != isFaction {
				return newEncyclopediaValidationError("binding_selector_mismatch", "topic %s binding %s and selector disagree", topicID, binding.Variant)
			}
		}
	}
	return nil
}

func validateEncyclopediaManifest(manifest encyclopediaManifest) error {
	if manifest.SchemaVersion != 1 {
		return newEncyclopediaValidationError("unsupported_version", "manifest schema_version = %d", manifest.SchemaVersion)
	}
	if !encyclopediaTokenPattern.MatchString(manifest.SourceProfile) || !encyclopediaTokenPattern.MatchString(manifest.ExtractorVersion) {
		return newEncyclopediaValidationError("missing_field", "manifest source_profile/extractor_version is absent or invalid")
	}
	if !validSHA256(manifest.CatalogSHA256) || len(manifest.Files) == 0 || len(manifest.BindingSources) == 0 || len(manifest.SourceRecords) == 0 {
		return newEncyclopediaValidationError("missing_field", "manifest provenance fields are incomplete")
	}
	if len(manifest.Files) > 20_001 || len(manifest.BindingSources) > 256 || len(manifest.SourceRecords) > 100_000 {
		return newEncyclopediaValidationError("resource_limit:manifest_entries", "manifest collections exceed v1 bounds")
	}
	if _, selfHash := manifest.Files["manifest.json"]; selfHash {
		return newEncyclopediaValidationError("unexpected_runtime_file", "manifest must not hash itself")
	}
	if digest, ok := manifest.Files["catalog.json"]; !ok || !validSHA256(digest) {
		return newEncyclopediaValidationError("missing_field", "manifest files omits catalog.json")
	}
	for filePath, digest := range manifest.Files {
		if filePath != "catalog.json" && !validEncyclopediaAssetPath(filePath) {
			return newEncyclopediaValidationError("unsafe_asset_path", "%q", filePath)
		}
		if !validSHA256(digest) {
			return newEncyclopediaValidationError("invalid_digest", "%q", filePath)
		}
	}
	seenSources := make(map[string]struct{}, len(manifest.BindingSources))
	for _, source := range manifest.BindingSources {
		folded := strings.ToLower(source.Basename)
		if !encyclopediaTokenPattern.MatchString(source.Basename) || !validSHA256(source.SHA256) {
			return newEncyclopediaValidationError("invalid_binding_source", "%q", source.Basename)
		}
		if _, duplicate := seenSources[folded]; duplicate {
			return newEncyclopediaValidationError("binding_source_mismatch", "duplicate binding source %q", source.Basename)
		}
		seenSources[folded] = struct{}{}
	}
	for sourceRef, record := range manifest.SourceRecords {
		if !validEncyclopediaSourceRef(sourceRef) || !encyclopediaTokenPattern.MatchString(record.SourceBasename) || !validSHA256(record.SourceSHA256) || !validSHA256(record.RawSHA256) || record.LanguageID > 65535 || record.RawLength > encyclopediaMaxImageBytes || record.Decoder == "" || len(record.Decoder) > 128 || record.Encoding == "" || len(record.Encoding) > 128 || len(record.MappingCitations) == 0 || len(record.MappingCitations) > 64 {
			return newEncyclopediaValidationError("invalid_source_record", "%q", sourceRef)
		}
		if err := validateManifestResourceIdentifier(record.ResourceType); err != nil {
			return err
		}
		if err := validateManifestResourceIdentifier(record.ResourceID); err != nil {
			return err
		}
		seenCitations := make(map[string]struct{}, len(record.MappingCitations))
		for _, citation := range record.MappingCitations {
			if citation == "" || len(citation) > 512 {
				return newEncyclopediaValidationError("invalid_source_record", "%q citation", sourceRef)
			}
			if _, duplicate := seenCitations[citation]; duplicate {
				return newEncyclopediaValidationError("invalid_source_record", "%q repeats citation", sourceRef)
			}
			seenCitations[citation] = struct{}{}
		}
	}
	return nil
}

func validateEncyclopediaSourceRefClosure(catalog encyclopediaCatalog, manifest encyclopediaManifest) error {
	references := []string{catalog.Index.SourceRef}
	for _, category := range catalog.Categories {
		references = append(references, category.SourceRef)
	}
	for _, topic := range catalog.Topics {
		references = append(references, topic.SourceRef)
	}
	for _, image := range catalog.Images {
		references = append(references, image.SourceRef)
	}
	for _, sourceRef := range references {
		if _, ok := manifest.SourceRecords[sourceRef]; !ok {
			return newEncyclopediaValidationError("dangling_source_ref", "%q", sourceRef)
		}
	}
	return nil
}

func validateCatalogTopicIDList(values []string, topics map[string]encyclopediaTopic, seen map[string]struct{}) error {
	for _, topicID := range values {
		if _, duplicate := seen[topicID]; duplicate {
			return newEncyclopediaValidationError("duplicate_topic_id", "%q", topicID)
		}
		seen[topicID] = struct{}{}
		if _, ok := topics[topicID]; !ok {
			return newEncyclopediaValidationError("dangling_topic_reference", "%q", topicID)
		}
	}
	return nil
}

func validateEncyclopediaLabels(labels map[string]string) error {
	if len(labels) == 0 || len(labels) > 256 {
		return newEncyclopediaValidationError("missing_field", "labels must be nonempty")
	}
	for language, label := range labels {
		if !validEncyclopediaLangID(language) {
			return newEncyclopediaValidationError("invalid_language_id", "%q", language)
		}
		if len([]byte(label)) > encyclopediaMaxTitleBytes {
			return newEncyclopediaValidationError("resource_limit:label_bytes", "language %s", language)
		}
	}
	return nil
}

func validateManifestResourceIdentifier(identifier encyclopediaManifestResourceIdentifier) error {
	switch identifier.Kind {
	case "numeric":
		var value uint32
		if len(identifier.Value) == 0 || json.Unmarshal(identifier.Value, &value) != nil {
			return newEncyclopediaValidationError("invalid_source_record", "invalid numeric resource identifier")
		}
	case "named":
		var value string
		if len(identifier.Value) == 0 || json.Unmarshal(identifier.Value, &value) != nil || value == "" || len(value) > 256 {
			return newEncyclopediaValidationError("invalid_source_record", "invalid named resource identifier")
		}
	default:
		return newEncyclopediaValidationError("invalid_source_record", "unknown resource identifier kind %q", identifier.Kind)
	}
	return nil
}

func validateRawEncyclopediaJSON(data []byte, maxBytes, maxDepth int) error {
	if len(data) > maxBytes {
		return newEncyclopediaValidationError("resource_limit:json_bytes", "%d > %d", len(data), maxBytes)
	}
	if !utf8.Valid(data) {
		return newEncyclopediaValidationError("invalid_utf8", "JSON input is not valid UTF-8")
	}
	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.UseNumber()
	if err := scanEncyclopediaJSONValue(decoder, 1, maxDepth); err != nil {
		return err
	}
	if token, err := decoder.Token(); err != io.EOF {
		if err == nil {
			return newEncyclopediaValidationError("invalid_json", "extra token %v", token)
		}
		return newEncyclopediaValidationError("invalid_json", "%v", err)
	}
	return nil
}

func scanEncyclopediaJSONValue(decoder *json.Decoder, depth, maxDepth int) error {
	token, err := decoder.Token()
	if err != nil {
		return newEncyclopediaValidationError("invalid_json", "%v", err)
	}
	delim, compound := token.(json.Delim)
	if !compound {
		return nil
	}
	if depth > maxDepth {
		return newEncyclopediaValidationError("resource_limit:json_depth", "depth %d exceeds %d", depth, maxDepth)
	}
	switch delim {
	case '{':
		seen := make(map[string]struct{})
		for decoder.More() {
			keyToken, err := decoder.Token()
			if err != nil {
				return newEncyclopediaValidationError("invalid_json", "%v", err)
			}
			key, ok := keyToken.(string)
			if !ok {
				return newEncyclopediaValidationError("invalid_json", "object key is not a string")
			}
			if _, duplicate := seen[key]; duplicate {
				return newEncyclopediaValidationError("duplicate_key", "%q", key)
			}
			seen[key] = struct{}{}
			if err := scanEncyclopediaJSONValue(decoder, depth+1, maxDepth); err != nil {
				return err
			}
		}
		closing, err := decoder.Token()
		if err != nil || closing != json.Delim('}') {
			return newEncyclopediaValidationError("invalid_json", "unterminated object")
		}
	case '[':
		for decoder.More() {
			if err := scanEncyclopediaJSONValue(decoder, depth+1, maxDepth); err != nil {
				return err
			}
		}
		closing, err := decoder.Token()
		if err != nil || closing != json.Delim(']') {
			return newEncyclopediaValidationError("invalid_json", "unterminated array")
		}
	default:
		return newEncyclopediaValidationError("invalid_json", "unexpected delimiter %q", delim)
	}
	return nil
}

func decodeStrictEncyclopediaJSON(data []byte, target any) error {
	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	decoder.UseNumber()
	if err := decoder.Decode(target); err != nil {
		return err
	}
	return requireJSONEOF(decoder)
}

func classifyEncyclopediaJSONError(err error) error {
	if strings.Contains(err.Error(), "unknown field") {
		return newEncyclopediaValidationError("unknown_field", "%v", err)
	}
	return newEncyclopediaValidationError("invalid_json", "%v", err)
}

func validEncyclopediaLangID(value string) bool {
	if !encyclopediaLangIDPattern.MatchString(value) {
		return false
	}
	number, err := strconv.ParseUint(value, 10, 16)
	return err == nil && number <= 65535
}

func validEncyclopediaStableID(value string) bool {
	return len(value) >= 3 && len(value) <= 256 && encyclopediaStableIDPattern.MatchString(value)
}

func validEncyclopediaSourceRef(value string) bool {
	return len(value) >= 1 && len(value) <= 256 && encyclopediaSourceRefPattern.MatchString(value)
}

func validEncyclopediaAssetPath(value string) bool {
	if len(value) < 8 || len(value) > 256 || !strings.HasPrefix(value, "assets/") || strings.ContainsAny(value, "\\\x00\r\n") || path.Clean(value) != value {
		return false
	}
	for _, part := range strings.Split(value, "/") {
		if part == "" || part == "." || part == ".." || len(part) > 128 {
			return false
		}
		for index, r := range part {
			if (r >= 'A' && r <= 'Z') || (r >= 'a' && r <= 'z') || (r >= '0' && r <= '9') || (index > 0 && (r == '.' || r == '_' || r == '-')) {
				continue
			}
			return false
		}
	}
	return true
}
