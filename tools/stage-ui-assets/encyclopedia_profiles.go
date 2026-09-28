package main

import (
	"encoding/json"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"time"
)

const (
	rtEncyclopediaText = uint32(10)

	encyclopediaResearchKind          = "encyclopedia-research"
	encyclopediaResearchSchemaVersion = 1
	encyclopediaRunLogKind            = "encyclopedia-inventory-run"
	encyclopediaRunLogSchemaVersion   = 1
)

type encyclopediaSourceKind string

const (
	encyclopediaSourceDLL encyclopediaSourceKind = "dll"
	encyclopediaSourceEXE encyclopediaSourceKind = "exe"
	encyclopediaSourceDAT encyclopediaSourceKind = "dat"
)

type encyclopediaResourceIdentifierKind string

const (
	encyclopediaIdentifierNumeric encyclopediaResourceIdentifierKind = "numeric"
	encyclopediaIdentifierNamed   encyclopediaResourceIdentifierKind = "named"
)

type encyclopediaRecordStatus string

const (
	encyclopediaRecordInventoried     encyclopediaRecordStatus = "inventoried"
	encyclopediaRecordDecoded         encyclopediaRecordStatus = "decoded"
	encyclopediaRecordBound           encyclopediaRecordStatus = "bound"
	encyclopediaRecordDocumentedAlias encyclopediaRecordStatus = "documented_alias"
	encyclopediaRecordUnresolved      encyclopediaRecordStatus = "unresolved"
)

type encyclopediaSourceRoot struct {
	Role string
	Path string
}

type encyclopediaSourceSpec struct {
	RootRole       string
	Basename       string
	Kind           encyclopediaSourceKind
	ResourceTypeID uint32
}

type encyclopediaInventoryRequest struct {
	Roots     []encyclopediaSourceRoot
	Sources   []encyclopediaSourceSpec
	StartedAt time.Time
}

type encyclopediaInventoryLimits struct {
	MaxSourceBytes            uint64
	MaxResourceCount          int
	MaxResourceBytes          uint64
	MaxAggregateResourceBytes uint64
}

type encyclopediaInventory struct {
	Report encyclopediaResearchReport
	RunLog encyclopediaInventoryRunLog
}

type encyclopediaResearchReport struct {
	Kind                   string                                  `json:"kind"`
	SchemaVersion          int                                     `json:"schema_version"`
	SourceCount            int                                     `json:"source_count"`
	RecordCount            int                                     `json:"record_count"`
	DuplicateIdentityCount int                                     `json:"duplicate_identity_count"`
	Sources                []encyclopediaSourceRecord              `json:"sources"`
	Records                []encyclopediaResearchRecord            `json:"records"`
	DuplicateIdentities    []encyclopediaDuplicateResourceIdentity `json:"duplicate_identities"`
}

type encyclopediaSourceRecord struct {
	RootRole  string                 `json:"root_role"`
	Basename  string                 `json:"basename"`
	Kind      encyclopediaSourceKind `json:"kind"`
	RawLength uint64                 `json:"raw_length"`
	RawSHA256 string                 `json:"raw_sha256"`
}

type encyclopediaResourceIdentifier struct {
	Kind      encyclopediaResourceIdentifierKind `json:"kind"`
	NumericID *uint32                            `json:"numeric_id,omitempty"`
	Name      string                             `json:"name,omitempty"`
}

type encyclopediaUnresolvedStatus struct {
	Reason    string `json:"reason"`
	NextProof string `json:"next_proof"`
}

type encyclopediaResearchRecord struct {
	SourceRootRole    string                         `json:"source_root_role"`
	SourceBasename    string                         `json:"source_basename"`
	ResourceType      encyclopediaResourceIdentifier `json:"resource_type"`
	ResourceID        encyclopediaResourceIdentifier `json:"resource_id"`
	LanguageID        uint32                         `json:"language_id"`
	CodePage          uint32                         `json:"code_page"`
	RawLength         uint64                         `json:"raw_length"`
	RawSHA256         string                         `json:"raw_sha256"`
	DuplicateIdentity bool                           `json:"duplicate_identity"`
	Status            encyclopediaRecordStatus       `json:"status"`
	Unresolved        *encyclopediaUnresolvedStatus  `json:"unresolved,omitempty"`
	RawBytes          []byte                         `json:"-"`
}

type encyclopediaDuplicateResourceIdentity struct {
	SourceRootRole string                         `json:"source_root_role"`
	SourceBasename string                         `json:"source_basename"`
	ResourceType   encyclopediaResourceIdentifier `json:"resource_type"`
	ResourceID     encyclopediaResourceIdentifier `json:"resource_id"`
	LanguageID     uint32                         `json:"language_id"`
	Occurrences    int                            `json:"occurrences"`
}

type encyclopediaInventoryRunLog struct {
	Kind          string                         `json:"kind"`
	SchemaVersion int                            `json:"schema_version"`
	StartedAt     string                         `json:"started_at"`
	Roots         []encyclopediaInventoryRunRoot `json:"roots"`
}

type encyclopediaInventoryRunRoot struct {
	Role string `json:"role"`
	Path string `json:"path"`
}

func defaultEncyclopediaInventoryLimits() encyclopediaInventoryLimits {
	return encyclopediaInventoryLimits{
		MaxSourceBytes:            512 << 20,
		MaxResourceCount:          10_000,
		MaxResourceBytes:          1 << 20,
		MaxAggregateResourceBytes: 128 << 20,
	}
}

func inventoryEncyclopediaSources(request encyclopediaInventoryRequest, limits encyclopediaInventoryLimits) (encyclopediaInventory, error) {
	if request.StartedAt.IsZero() {
		return encyclopediaInventory{}, fmt.Errorf("encyclopedia inventory started_at is required for the separate run log")
	}
	if err := validateEncyclopediaInventoryLimits(limits); err != nil {
		return encyclopediaInventory{}, err
	}

	rootPaths := make(map[string]string, len(request.Roots))
	rootKeys := make(map[string]struct{}, len(request.Roots))
	runRoots := make([]encyclopediaInventoryRunRoot, 0, len(request.Roots))
	for _, root := range request.Roots {
		if err := validateEncyclopediaRootRole(root.Role); err != nil {
			return encyclopediaInventory{}, err
		}
		roleKey := strings.ToLower(root.Role)
		if _, exists := rootKeys[roleKey]; exists {
			return encyclopediaInventory{}, fmt.Errorf("duplicate encyclopedia source root role %q", root.Role)
		}
		absolutePath, err := filepath.Abs(root.Path)
		if err != nil {
			return encyclopediaInventory{}, fmt.Errorf("resolve encyclopedia source root %q: %w", root.Role, err)
		}
		info, err := os.Stat(absolutePath)
		if err != nil {
			return encyclopediaInventory{}, fmt.Errorf("inspect encyclopedia source root %q: %w", root.Role, err)
		}
		if !info.IsDir() {
			return encyclopediaInventory{}, fmt.Errorf("encyclopedia source root %q is not a directory", root.Role)
		}
		rootKeys[roleKey] = struct{}{}
		rootPaths[root.Role] = filepath.Clean(absolutePath)
		runRoots = append(runRoots, encyclopediaInventoryRunRoot{Role: root.Role, Path: filepath.Clean(absolutePath)})
	}
	if len(rootPaths) == 0 {
		return encyclopediaInventory{}, fmt.Errorf("encyclopedia inventory requires at least one explicit source root")
	}
	sort.Slice(runRoots, func(i, j int) bool {
		if runRoots[i].Role != runRoots[j].Role {
			return runRoots[i].Role < runRoots[j].Role
		}
		return runRoots[i].Path < runRoots[j].Path
	})

	sources := append([]encyclopediaSourceSpec(nil), request.Sources...)
	sort.Slice(sources, func(i, j int) bool { return lessEncyclopediaSourceSpec(sources[i], sources[j]) })
	seenSources := make(map[string]struct{}, len(sources))
	report := encyclopediaResearchReport{
		Kind:          encyclopediaResearchKind,
		SchemaVersion: encyclopediaResearchSchemaVersion,
		Sources:       make([]encyclopediaSourceRecord, 0, len(sources)),
	}
	for _, source := range sources {
		rootPath, ok := rootPaths[source.RootRole]
		if !ok {
			return encyclopediaInventory{}, fmt.Errorf("encyclopedia source %q refers to unknown root role %q", source.Basename, source.RootRole)
		}
		if err := validateEncyclopediaSourceSpec(source); err != nil {
			return encyclopediaInventory{}, err
		}
		sourceKey := strings.ToLower(source.RootRole) + "\x00" + strings.ToLower(source.Basename)
		if _, exists := seenSources[sourceKey]; exists {
			return encyclopediaInventory{}, fmt.Errorf("duplicate encyclopedia source %q under root role %q", source.Basename, source.RootRole)
		}
		seenSources[sourceKey] = struct{}{}

		path := filepath.Join(rootPath, source.Basename)
		data, err := readBoundedEncyclopediaSource(path, limits.MaxSourceBytes)
		if err != nil {
			return encyclopediaInventory{}, fmt.Errorf("read encyclopedia source %s/%s: %w", source.RootRole, source.Basename, err)
		}
		report.Sources = append(report.Sources, encyclopediaSourceRecord{
			RootRole:  source.RootRole,
			Basename:  source.Basename,
			Kind:      source.Kind,
			RawLength: uint64(len(data)),
			RawSHA256: byteSHA256(data),
		})

		if source.ResourceTypeID == 0 {
			continue
		}
		rawResources, err := readPEStrictMixedRawResourcesFromBytes(data, source.ResourceTypeID, rawResourceLimits{
			MaxCount:          limits.MaxResourceCount,
			MaxResourceBytes:  limits.MaxResourceBytes,
			MaxAggregateBytes: limits.MaxAggregateResourceBytes,
		})
		if err != nil {
			return encyclopediaInventory{}, fmt.Errorf("inventory resource type %d from %s/%s: %w", source.ResourceTypeID, source.RootRole, source.Basename, err)
		}
		resourceType := numericEncyclopediaResourceIdentifier(source.ResourceTypeID)
		for _, raw := range rawResources {
			resourceID := numericEncyclopediaResourceIdentifier(raw.ID)
			if raw.Named {
				resourceID = namedEncyclopediaResourceIdentifier(raw.Name)
			}
			rawBytes := append([]byte(nil), raw.Data...)
			report.Records = append(report.Records, encyclopediaResearchRecord{
				SourceRootRole: source.RootRole,
				SourceBasename: source.Basename,
				ResourceType:   resourceType,
				ResourceID:     resourceID,
				LanguageID:     raw.Language,
				CodePage:       raw.CodePage,
				RawLength:      uint64(len(rawBytes)),
				RawSHA256:      byteSHA256(rawBytes),
				Status:         encyclopediaRecordInventoried,
				RawBytes:       rawBytes,
			})
		}
	}

	canonical, err := canonicalEncyclopediaResearchReport(report)
	if err != nil {
		return encyclopediaInventory{}, err
	}
	return encyclopediaInventory{
		Report: canonical,
		RunLog: encyclopediaInventoryRunLog{
			Kind:          encyclopediaRunLogKind,
			SchemaVersion: encyclopediaRunLogSchemaVersion,
			StartedAt:     request.StartedAt.UTC().Format(time.RFC3339Nano),
			Roots:         runRoots,
		},
	}, nil
}

func marshalEncyclopediaResearchReport(report encyclopediaResearchReport) ([]byte, error) {
	canonical, err := canonicalEncyclopediaResearchReport(report)
	if err != nil {
		return nil, err
	}
	data, err := json.MarshalIndent(canonical, "", "  ")
	if err != nil {
		return nil, fmt.Errorf("marshal encyclopedia research report: %w", err)
	}
	return append(data, '\n'), nil
}

func canonicalEncyclopediaResearchReport(report encyclopediaResearchReport) (encyclopediaResearchReport, error) {
	canonical := report
	canonical.Sources = make([]encyclopediaSourceRecord, len(report.Sources))
	copy(canonical.Sources, report.Sources)
	canonical.Records = make([]encyclopediaResearchRecord, len(report.Records))
	copy(canonical.Records, report.Records)

	if canonical.Kind != encyclopediaResearchKind {
		return encyclopediaResearchReport{}, fmt.Errorf("encyclopedia research report kind %q is unsupported", canonical.Kind)
	}
	if canonical.SchemaVersion != encyclopediaResearchSchemaVersion {
		return encyclopediaResearchReport{}, fmt.Errorf("encyclopedia research report schema version %d is unsupported", canonical.SchemaVersion)
	}

	sort.Slice(canonical.Sources, func(i, j int) bool { return lessEncyclopediaSourceRecord(canonical.Sources[i], canonical.Sources[j]) })
	knownSources := make(map[string]struct{}, len(canonical.Sources))
	for _, source := range canonical.Sources {
		if err := validateEncyclopediaSourceRecord(source); err != nil {
			return encyclopediaResearchReport{}, err
		}
		key := encyclopediaSourceRecordKey(source.RootRole, source.Basename)
		if _, exists := knownSources[key]; exists {
			return encyclopediaResearchReport{}, fmt.Errorf("duplicate encyclopedia report source %s/%s", source.RootRole, source.Basename)
		}
		knownSources[key] = struct{}{}
	}

	sort.Slice(canonical.Records, func(i, j int) bool { return lessEncyclopediaResearchRecord(canonical.Records[i], canonical.Records[j]) })
	identityCounts := make(map[encyclopediaResearchIdentityKey]int, len(canonical.Records))
	for _, record := range canonical.Records {
		if err := validateEncyclopediaResearchRecord(record); err != nil {
			return encyclopediaResearchReport{}, err
		}
		if _, exists := knownSources[encyclopediaSourceRecordKey(record.SourceRootRole, record.SourceBasename)]; !exists {
			return encyclopediaResearchReport{}, fmt.Errorf("encyclopedia record refers to unknown source %s/%s", record.SourceRootRole, record.SourceBasename)
		}
		identityCounts[researchIdentityKey(record)]++
	}

	canonical.DuplicateIdentities = make([]encyclopediaDuplicateResourceIdentity, 0)
	seenDuplicates := make(map[encyclopediaResearchIdentityKey]struct{})
	for index := range canonical.Records {
		record := &canonical.Records[index]
		key := researchIdentityKey(*record)
		count := identityCounts[key]
		record.DuplicateIdentity = count > 1
		if count < 2 {
			continue
		}
		if _, seen := seenDuplicates[key]; seen {
			continue
		}
		seenDuplicates[key] = struct{}{}
		canonical.DuplicateIdentities = append(canonical.DuplicateIdentities, encyclopediaDuplicateResourceIdentity{
			SourceRootRole: record.SourceRootRole,
			SourceBasename: record.SourceBasename,
			ResourceType:   record.ResourceType,
			ResourceID:     record.ResourceID,
			LanguageID:     record.LanguageID,
			Occurrences:    count,
		})
	}
	canonical.SourceCount = len(canonical.Sources)
	canonical.RecordCount = len(canonical.Records)
	canonical.DuplicateIdentityCount = len(canonical.DuplicateIdentities)
	return canonical, nil
}

func validateEncyclopediaInventoryLimits(limits encyclopediaInventoryLimits) error {
	if limits.MaxSourceBytes == 0 || limits.MaxSourceBytes > uint64(^uint64(0)>>1)-1 {
		return fmt.Errorf("encyclopedia source-byte limit must be between 1 and the supported reader maximum")
	}
	if limits.MaxResourceCount <= 0 || limits.MaxResourceBytes == 0 || limits.MaxAggregateResourceBytes == 0 {
		return fmt.Errorf("encyclopedia resource count and byte limits must be positive")
	}
	return nil
}

func validateEncyclopediaRootRole(role string) error {
	if role == "" || role != strings.TrimSpace(role) || strings.ContainsAny(role, "/\\\x00\r\n") {
		return fmt.Errorf("invalid encyclopedia source root role %q", role)
	}
	return nil
}

func validateEncyclopediaSourceSpec(source encyclopediaSourceSpec) error {
	if err := validateEncyclopediaRootRole(source.RootRole); err != nil {
		return err
	}
	if source.Basename == "" || source.Basename == "." || source.Basename == ".." || filepath.Base(source.Basename) != source.Basename || strings.ContainsAny(source.Basename, "/\\\x00\r\n") {
		return fmt.Errorf("invalid encyclopedia source basename %q", source.Basename)
	}
	wantExtension := map[encyclopediaSourceKind]string{
		encyclopediaSourceDLL: ".dll",
		encyclopediaSourceEXE: ".exe",
		encyclopediaSourceDAT: ".dat",
	}[source.Kind]
	if wantExtension == "" {
		return fmt.Errorf("unsupported encyclopedia source kind %q", source.Kind)
	}
	if !strings.EqualFold(filepath.Ext(source.Basename), wantExtension) {
		return fmt.Errorf("encyclopedia source %q does not match kind %q", source.Basename, source.Kind)
	}
	if source.ResourceTypeID != 0 {
		if source.Kind != encyclopediaSourceDLL || source.ResourceTypeID != rtEncyclopediaText {
			return fmt.Errorf("only DLL resource type %d is supported by the encyclopedia text inventory", rtEncyclopediaText)
		}
	}
	return nil
}

func validateEncyclopediaSourceRecord(source encyclopediaSourceRecord) error {
	if err := validateEncyclopediaRootRole(source.RootRole); err != nil {
		return err
	}
	if err := validateEncyclopediaSourceSpec(encyclopediaSourceSpec{RootRole: source.RootRole, Basename: source.Basename, Kind: source.Kind}); err != nil {
		return err
	}
	if !validSHA256(source.RawSHA256) {
		return fmt.Errorf("encyclopedia source %s/%s has invalid raw SHA-256", source.RootRole, source.Basename)
	}
	return nil
}

func validateEncyclopediaResearchRecord(record encyclopediaResearchRecord) error {
	if err := validateEncyclopediaRootRole(record.SourceRootRole); err != nil {
		return err
	}
	if record.SourceBasename == "" || filepath.Base(record.SourceBasename) != record.SourceBasename || strings.ContainsAny(record.SourceBasename, "/\\\x00\r\n") {
		return fmt.Errorf("invalid encyclopedia record source basename %q", record.SourceBasename)
	}
	if err := validateEncyclopediaResourceIdentifier(record.ResourceType); err != nil {
		return fmt.Errorf("invalid encyclopedia resource type: %w", err)
	}
	if err := validateEncyclopediaResourceIdentifier(record.ResourceID); err != nil {
		return fmt.Errorf("invalid encyclopedia resource identity: %w", err)
	}
	if !validSHA256(record.RawSHA256) {
		return fmt.Errorf("encyclopedia resource from %s has invalid raw SHA-256", record.SourceBasename)
	}
	if record.RawBytes != nil && (record.RawLength != uint64(len(record.RawBytes)) || record.RawSHA256 != byteSHA256(record.RawBytes)) {
		return fmt.Errorf("encyclopedia resource from %s has raw facts that do not match its preserved bytes", record.SourceBasename)
	}
	switch record.Status {
	case encyclopediaRecordInventoried, encyclopediaRecordDecoded, encyclopediaRecordBound, encyclopediaRecordDocumentedAlias:
		if record.Unresolved != nil {
			return fmt.Errorf("encyclopedia record status %q cannot carry unresolved details", record.Status)
		}
	case encyclopediaRecordUnresolved:
		if record.Unresolved == nil || strings.TrimSpace(record.Unresolved.Reason) == "" || strings.TrimSpace(record.Unresolved.NextProof) == "" {
			return fmt.Errorf("unresolved encyclopedia record requires reason and next_proof")
		}
	default:
		return fmt.Errorf("unsupported encyclopedia record status %q", record.Status)
	}
	return nil
}

func validateEncyclopediaResourceIdentifier(identifier encyclopediaResourceIdentifier) error {
	switch identifier.Kind {
	case encyclopediaIdentifierNumeric:
		if identifier.NumericID == nil || identifier.Name != "" {
			return fmt.Errorf("numeric identifier must contain only numeric_id")
		}
	case encyclopediaIdentifierNamed:
		if identifier.NumericID != nil || identifier.Name == "" {
			return fmt.Errorf("named identifier must contain only name")
		}
	default:
		return fmt.Errorf("unsupported identifier kind %q", identifier.Kind)
	}
	return nil
}

func readBoundedEncyclopediaSource(path string, maxBytes uint64) ([]byte, error) {
	info, err := os.Lstat(path)
	if err != nil {
		return nil, err
	}
	if info.Mode()&os.ModeSymlink != 0 {
		return nil, fmt.Errorf("source is a symlink")
	}
	if !info.Mode().IsRegular() {
		return nil, fmt.Errorf("source is not a regular file")
	}
	if uint64(info.Size()) > maxBytes {
		return nil, fmt.Errorf("source length %d exceeds the %d-byte limit", info.Size(), maxBytes)
	}
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	data, err := io.ReadAll(io.LimitReader(file, int64(maxBytes)+1))
	if err != nil {
		return nil, err
	}
	if uint64(len(data)) > maxBytes {
		return nil, fmt.Errorf("source grew beyond the %d-byte limit while reading", maxBytes)
	}
	return data, nil
}

func numericEncyclopediaResourceIdentifier(id uint32) encyclopediaResourceIdentifier {
	copy := id
	return encyclopediaResourceIdentifier{Kind: encyclopediaIdentifierNumeric, NumericID: &copy}
}

func namedEncyclopediaResourceIdentifier(name string) encyclopediaResourceIdentifier {
	return encyclopediaResourceIdentifier{Kind: encyclopediaIdentifierNamed, Name: name}
}

func lessEncyclopediaSourceSpec(left, right encyclopediaSourceSpec) bool {
	if left.RootRole != right.RootRole {
		return left.RootRole < right.RootRole
	}
	leftName, rightName := strings.ToLower(left.Basename), strings.ToLower(right.Basename)
	if leftName != rightName {
		return leftName < rightName
	}
	if left.Basename != right.Basename {
		return left.Basename < right.Basename
	}
	if left.Kind != right.Kind {
		return left.Kind < right.Kind
	}
	return left.ResourceTypeID < right.ResourceTypeID
}

func lessEncyclopediaSourceRecord(left, right encyclopediaSourceRecord) bool {
	return lessEncyclopediaSourceSpec(
		encyclopediaSourceSpec{RootRole: left.RootRole, Basename: left.Basename, Kind: left.Kind},
		encyclopediaSourceSpec{RootRole: right.RootRole, Basename: right.Basename, Kind: right.Kind},
	)
}

func lessEncyclopediaResearchRecord(left, right encyclopediaResearchRecord) bool {
	if left.SourceRootRole != right.SourceRootRole {
		return left.SourceRootRole < right.SourceRootRole
	}
	leftSource, rightSource := strings.ToLower(left.SourceBasename), strings.ToLower(right.SourceBasename)
	if leftSource != rightSource {
		return leftSource < rightSource
	}
	if left.SourceBasename != right.SourceBasename {
		return left.SourceBasename < right.SourceBasename
	}
	if comparison := compareEncyclopediaResourceIdentifier(left.ResourceType, right.ResourceType); comparison != 0 {
		return comparison < 0
	}
	if comparison := compareEncyclopediaResourceIdentifier(left.ResourceID, right.ResourceID); comparison != 0 {
		return comparison < 0
	}
	if left.LanguageID != right.LanguageID {
		return left.LanguageID < right.LanguageID
	}
	if left.CodePage != right.CodePage {
		return left.CodePage < right.CodePage
	}
	if left.RawSHA256 != right.RawSHA256 {
		return left.RawSHA256 < right.RawSHA256
	}
	if left.RawLength != right.RawLength {
		return left.RawLength < right.RawLength
	}
	if left.Status != right.Status {
		return left.Status < right.Status
	}
	if left.Unresolved == nil || right.Unresolved == nil {
		return left.Unresolved == nil && right.Unresolved != nil
	}
	if left.Unresolved.Reason != right.Unresolved.Reason {
		return left.Unresolved.Reason < right.Unresolved.Reason
	}
	return left.Unresolved.NextProof < right.Unresolved.NextProof
}

func compareEncyclopediaResourceIdentifier(left, right encyclopediaResourceIdentifier) int {
	if left.Kind != right.Kind {
		if left.Kind == encyclopediaIdentifierNumeric {
			return -1
		}
		return 1
	}
	if left.Kind == encyclopediaIdentifierNumeric {
		leftID, rightID := uint32(0), uint32(0)
		if left.NumericID != nil {
			leftID = *left.NumericID
		}
		if right.NumericID != nil {
			rightID = *right.NumericID
		}
		switch {
		case leftID < rightID:
			return -1
		case leftID > rightID:
			return 1
		default:
			return 0
		}
	}
	return strings.Compare(left.Name, right.Name)
}

type encyclopediaResearchIdentityKey struct {
	rootRole      string
	basename      string
	typeKind      encyclopediaResourceIdentifierKind
	typeNumericID uint32
	typeName      string
	resourceKind  encyclopediaResourceIdentifierKind
	resourceID    uint32
	resourceName  string
	languageID    uint32
}

func researchIdentityKey(record encyclopediaResearchRecord) encyclopediaResearchIdentityKey {
	key := encyclopediaResearchIdentityKey{
		rootRole:     strings.ToLower(record.SourceRootRole),
		basename:     strings.ToLower(record.SourceBasename),
		typeKind:     record.ResourceType.Kind,
		typeName:     record.ResourceType.Name,
		resourceKind: record.ResourceID.Kind,
		resourceName: record.ResourceID.Name,
		languageID:   record.LanguageID,
	}
	if record.ResourceType.NumericID != nil {
		key.typeNumericID = *record.ResourceType.NumericID
	}
	if record.ResourceID.NumericID != nil {
		key.resourceID = *record.ResourceID.NumericID
	}
	return key
}

func encyclopediaSourceRecordKey(rootRole, basename string) string {
	return strings.ToLower(rootRole) + "\x00" + strings.ToLower(basename)
}
