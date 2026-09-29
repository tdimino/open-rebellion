package main

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"
)

const (
	encyclopediaResearchReportFilename = "source-report.json"
	encyclopediaReportSourceRole       = "encyclopedia_text"
	encyclopediaReportMaxBytes         = 32 << 20
	encyclopediaNamedPathChunkBytes    = 120
)

var errEncyclopediaUnknownDecodeProfile = errors.New("encyclopedia source has no approved lossless decoder profile")

type preparedEncyclopediaReport struct {
	report    encyclopediaResearchReport
	texts     []string
	edataRoot string
}

func stageEncyclopediaReport(sourceDir, edataDir, outputDir string, force bool, log io.Writer) error {
	if strings.TrimSpace(edataDir) == "" {
		edataDir = filepath.Join(sourceDir, "EData")
	}
	return stageEncyclopediaReportWithRequest(encyclopediaReportStageRequest{
		SourceDir:   sourceDir,
		EDataDir:    edataDir,
		OutputDir:   outputDir,
		Force:       force,
		ImageLimits: defaultEncyclopediaImageLimits(),
		Log:         log,
	})
}

type encyclopediaReportStageRequest struct {
	SourceDir   string
	EDataDir    string
	OutputDir   string
	ModRoots    []string
	Force       bool
	ImageLimits encyclopediaImageLimits
	Log         io.Writer
}

func stageEncyclopediaReportWithRequest(request encyclopediaReportStageRequest) error {
	if request.ImageLimits == (encyclopediaImageLimits{}) {
		request.ImageLimits = defaultEncyclopediaImageLimits()
	}
	if request.Log == nil {
		request.Log = io.Discard
	}
	if err := rejectEncyclopediaReportRuntimeRoot(request.OutputDir); err != nil {
		return err
	}
	resolvedSource, err := filepath.Abs(request.SourceDir)
	if err != nil {
		return fmt.Errorf("resolve encyclopedia source root: %w", err)
	}
	resolvedEData, err := filepath.Abs(request.EDataDir)
	if err != nil {
		return fmt.Errorf("resolve encyclopedia EData root: %w", err)
	}
	fmt.Fprintf(request.Log, "Encyclopedia source root: %s\nEncyclopedia EData root: %s\n", resolvedSource, resolvedEData)
	sourceRoots := []string{request.SourceDir}
	protectedRoots := append([]string(nil), request.ModRoots...)
	if strings.TrimSpace(request.EDataDir) != "" {
		sourceRoots = append(sourceRoots, request.EDataDir)
		protectedRoots = append(protectedRoots, request.EDataDir)
	}
	if entries, err := os.ReadDir(request.SourceDir); err == nil {
		for _, entry := range entries {
			// Keep a named GData symlink in the protected set. The shared
			// collision resolver follows it before comparing output roots.
			if strings.EqualFold(entry.Name(), "GData") {
				protectedRoots = append(protectedRoots, filepath.Join(request.SourceDir, entry.Name()))
			}
		}
	}
	_, err = stageEncyclopediaDirectory(encyclopediaDirectoryRequest{
		OutputDir:   request.OutputDir,
		SourceRoots: sourceRoots,
		ModRoots:    protectedRoots,
		Force:       request.Force,
		BuildCandidate: func(root string) error {
			inventory, err := inventoryEncyclopediaSources(encyclopediaInventoryRequest{
				Roots: []encyclopediaSourceRoot{{Role: encyclopediaReportSourceRole, Path: request.SourceDir}},
				Sources: []encyclopediaSourceSpec{{
					RootRole:       encyclopediaReportSourceRole,
					Basename:       "ENCYTEXT.DLL",
					Kind:           encyclopediaSourceDLL,
					ResourceTypeID: rtEncyclopediaText,
				}},
				StartedAt: time.Unix(0, 1).UTC(),
			}, defaultEncyclopediaInventoryLimits())
			if err != nil {
				return err
			}
			prepared, err := prepareEncyclopediaReport(inventory.Report)
			if err != nil {
				return err
			}
			if strings.TrimSpace(request.EDataDir) != "" {
				artwork, err := prepareEncyclopediaArtwork(request.SourceDir, request.EDataDir, request.ImageLimits)
				if err != nil {
					return err
				}
				prepared.report.Artwork = &artwork
				prepared.edataRoot = request.EDataDir
			}
			return writeEncyclopediaReportCandidate(root, prepared)
		},
		InspectOwned:    inspectEncyclopediaReportDirectory,
		RecoveryCommand: fmt.Sprintf("stage-ui-assets --encyclopedia-report-only --encyclopedia-report-output %q --force", request.OutputDir),
		Log:             request.Log,
	})
	return err
}

func rejectEncyclopediaReportRuntimeRoot(outputDir string) error {
	info, err := os.Lstat(outputDir)
	if os.IsNotExist(err) {
		return nil
	}
	if err != nil || info.Mode()&os.ModeSymlink != 0 || !info.IsDir() {
		return nil
	}
	if encyclopediaRuntimeCatalogPresent(outputDir) {
		return fmt.Errorf("output_mode_mismatch: runtime encyclopedia catalog cannot be replaced by a research report")
	}
	return nil
}

func verifyEncyclopediaReport(outputDir string, log io.Writer) error {
	return verifyEncyclopediaDirectory(
		outputDir,
		inspectEncyclopediaReportDirectory,
		fmt.Sprintf("stage-ui-assets --encyclopedia-report-only --encyclopedia-report-output %q --force", outputDir),
		log,
	)
}

func prepareEncyclopediaReport(report encyclopediaResearchReport) (preparedEncyclopediaReport, error) {
	canonical, err := canonicalEncyclopediaResearchReport(report)
	if err != nil {
		return preparedEncyclopediaReport{}, err
	}
	if len(canonical.Records) == 0 {
		return preparedEncyclopediaReport{report: canonical}, nil
	}

	profiles, err := loadEmbeddedEncyclopediaProfiles()
	if err != nil {
		return preparedEncyclopediaReport{}, err
	}
	for _, record := range canonical.Records {
		source, err := sourceForEncyclopediaRecord(canonical.Sources, record)
		if err != nil {
			return preparedEncyclopediaReport{}, err
		}
		if _, _, err := matchEncyclopediaDecodeProfile(profiles, source); err != nil {
			if !strings.Contains(err.Error(), "unsupported source profile") {
				return preparedEncyclopediaReport{}, err
			}
			for index := range canonical.Records {
				canonical.Records[index].Status = encyclopediaRecordUnresolved
				canonical.Records[index].Unresolved = &encyclopediaUnresolvedStatus{
					Reason:    errEncyclopediaUnknownDecodeProfile.Error(),
					NextProof: "add a reviewed source profile matching the source length and SHA-256 before decoding",
				}
			}
			canonical, err = canonicalEncyclopediaResearchReport(canonical)
			if err != nil {
				return preparedEncyclopediaReport{}, err
			}
			return preparedEncyclopediaReport{report: canonical, texts: make([]string, len(canonical.Records))}, nil
		}
	}

	decoded, err := decodeEncyclopediaReport(canonical)
	if err != nil {
		return preparedEncyclopediaReport{}, err
	}
	if len(decoded.Records) != len(canonical.Records) {
		return preparedEncyclopediaReport{}, fmt.Errorf("decoded encyclopedia record count %d does not match inventory count %d", len(decoded.Records), len(canonical.Records))
	}
	texts := make([]string, len(decoded.Records))
	for index := range decoded.Records {
		canonical.Records[index] = decoded.Records[index].encyclopediaResearchRecord
		texts[index] = decoded.Records[index].Text
	}
	canonical, err = canonicalEncyclopediaResearchReport(canonical)
	if err != nil {
		return preparedEncyclopediaReport{}, err
	}
	return preparedEncyclopediaReport{report: canonical, texts: texts}, nil
}

func writeEncyclopediaReportCandidate(root string, prepared preparedEncyclopediaReport) error {
	paths, err := encyclopediaReportRecordPaths(prepared.report.Records)
	if err != nil {
		return err
	}
	for index, record := range prepared.report.Records {
		if uint64(len(record.RawBytes)) != record.RawLength || byteSHA256(record.RawBytes) != record.RawSHA256 {
			return fmt.Errorf("record %s has no validated preserved bytes", formatEncyclopediaResourceIdentifier(record.ResourceID))
		}
		if err := writeEncyclopediaReportFile(root, paths[index].raw, record.RawBytes); err != nil {
			return err
		}
		if record.Status == encyclopediaRecordDecoded {
			if index >= len(prepared.texts) {
				return fmt.Errorf("decoded record %s has no proven text", formatEncyclopediaResourceIdentifier(record.ResourceID))
			}
			if err := writeEncyclopediaReportFile(root, paths[index].text, []byte(prepared.texts[index])); err != nil {
				return err
			}
		}
	}
	if prepared.report.Artwork != nil {
		if strings.TrimSpace(prepared.edataRoot) == "" {
			return fmt.Errorf("artwork report has no declared EData source root")
		}
		for _, asset := range prepared.report.Artwork.StagedAssets {
			if err := copyEncyclopediaStagedAsset(prepared.edataRoot, root, asset); err != nil {
				return fmt.Errorf("stage %s: %w", asset.SourceBasename, err)
			}
		}
	}
	encoded, err := marshalEncyclopediaResearchReport(prepared.report)
	if err != nil {
		return err
	}
	return writeEncyclopediaReportFile(root, encyclopediaResearchReportFilename, encoded)
}

func writeEncyclopediaReportFile(root, relative string, contents []byte) error {
	path := filepath.Join(root, filepath.FromSlash(relative))
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return err
	}
	if err := os.WriteFile(path, contents, 0o644); err != nil {
		return fmt.Errorf("write encyclopedia report file %s: %w", relative, err)
	}
	return nil
}

type encyclopediaReportPaths struct {
	raw  string
	text string
}

func encyclopediaReportRecordPath(record encyclopediaResearchRecord, extension string) (string, error) {
	return encyclopediaReportRecordPathAtOccurrence(record, extension, 1)
}

func encyclopediaReportRecordPathAtOccurrence(record encyclopediaResearchRecord, extension string, occurrence int) (string, error) {
	if extension != ".bin" && extension != ".txt" {
		return "", fmt.Errorf("unsupported encyclopedia report extension %q", extension)
	}
	if occurrence < 1 {
		return "", fmt.Errorf("invalid encyclopedia report occurrence %d", occurrence)
	}
	language := strconv.FormatUint(uint64(record.LanguageID), 10)
	var components []string
	switch record.ResourceID.Kind {
	case encyclopediaIdentifierNumeric:
		if record.ResourceID.NumericID == nil {
			return "", fmt.Errorf("numeric encyclopedia resource has no ID")
		}
		components = []string{strconv.FormatUint(uint64(*record.ResourceID.NumericID), 10)}
	case encyclopediaIdentifierNamed:
		if record.ResourceID.Name == "" {
			return "", fmt.Errorf("named encyclopedia resource has an empty name")
		}
		encoded := hex.EncodeToString([]byte(record.ResourceID.Name))
		components = []string{"named"}
		for len(encoded) > encyclopediaNamedPathChunkBytes {
			components = append(components, encoded[:encyclopediaNamedPathChunkBytes])
			encoded = encoded[encyclopediaNamedPathChunkBytes:]
		}
		components = append(components, encoded)
	default:
		return "", fmt.Errorf("unsupported encyclopedia resource identifier kind %q", record.ResourceID.Kind)
	}
	last := len(components) - 1
	if occurrence > 1 {
		components[last] += "~" + strconv.Itoa(occurrence)
	}
	components[last] += extension
	return filepath.ToSlash(filepath.Join(append([]string{"raw", "encytext", language}, components...)...)), nil
}

func encyclopediaReportRecordPaths(records []encyclopediaResearchRecord) ([]encyclopediaReportPaths, error) {
	occurrences := make(map[encyclopediaResearchIdentityKey]int, len(records))
	result := make([]encyclopediaReportPaths, len(records))
	for index, record := range records {
		key := researchIdentityKey(record)
		occurrences[key]++
		raw, err := encyclopediaReportRecordPathAtOccurrence(record, ".bin", occurrences[key])
		if err != nil {
			return nil, err
		}
		text, err := encyclopediaReportRecordPathAtOccurrence(record, ".txt", occurrences[key])
		if err != nil {
			return nil, err
		}
		result[index] = encyclopediaReportPaths{raw: raw, text: text}
	}
	return result, nil
}

func inspectEncyclopediaReportDirectory(root string) (encyclopediaOwnedInventory, error) {
	reportPath := filepath.Join(root, encyclopediaResearchReportFilename)
	encoded, err := readEncyclopediaRegularFile(reportPath, encyclopediaReportMaxBytes)
	if err != nil {
		if os.IsNotExist(err) && encyclopediaRuntimeCatalogPresent(root) {
			return encyclopediaOwnedInventory{}, fmt.Errorf("output_mode_mismatch: runtime encyclopedia catalog cannot be used as a research report")
		}
		return encyclopediaOwnedInventory{}, err
	}
	decoder := json.NewDecoder(bytes.NewReader(encoded))
	decoder.DisallowUnknownFields()
	var report encyclopediaResearchReport
	if err := decoder.Decode(&report); err != nil {
		return encyclopediaOwnedInventory{}, fmt.Errorf("decode encyclopedia source report: %w", err)
	}
	if err := requireJSONEOF(decoder); err != nil {
		return encyclopediaOwnedInventory{}, fmt.Errorf("decode encyclopedia source report: %w", err)
	}
	report, err = canonicalEncyclopediaResearchReport(report)
	if err != nil {
		return encyclopediaOwnedInventory{}, err
	}
	if err := validatePublishedEncyclopediaReportShape(report); err != nil {
		return encyclopediaOwnedInventory{}, err
	}
	canonicalBytes, err := marshalEncyclopediaResearchReport(report)
	if err != nil {
		return encyclopediaOwnedInventory{}, err
	}
	if !bytes.Equal(encoded, canonicalBytes) {
		return encyclopediaOwnedInventory{}, fmt.Errorf("encyclopedia source report is not in canonical generated form")
	}
	paths, err := encyclopediaReportRecordPaths(report.Records)
	if err != nil {
		return encyclopediaOwnedInventory{}, err
	}
	owned := []string{encyclopediaResearchReportFilename}
	for index := range report.Records {
		record := &report.Records[index]
		raw, err := readEncyclopediaRegularFile(filepath.Join(root, filepath.FromSlash(paths[index].raw)), int64(defaultEncyclopediaInventoryLimits().MaxResourceBytes))
		if err != nil {
			return encyclopediaOwnedInventory{}, fmt.Errorf("read preserved resource %s: %w", paths[index].raw, err)
		}
		if uint64(len(raw)) != record.RawLength || byteSHA256(raw) != record.RawSHA256 {
			return encyclopediaOwnedInventory{}, fmt.Errorf("preserved resource %s does not match reported length/hash", paths[index].raw)
		}
		record.RawBytes = raw
		owned = append(owned, paths[index].raw)
	}
	prepared, err := prepareEncyclopediaReport(report)
	if err != nil {
		return encyclopediaOwnedInventory{}, fmt.Errorf("verify encyclopedia interpretation status: %w", err)
	}
	preparedBytes, err := marshalEncyclopediaResearchReport(prepared.report)
	if err != nil {
		return encyclopediaOwnedInventory{}, err
	}
	if !bytes.Equal(encoded, preparedBytes) {
		return encyclopediaOwnedInventory{}, fmt.Errorf("encyclopedia source report does not use the strongest approved interpretation status")
	}
	for index, record := range prepared.report.Records {
		switch record.Status {
		case encyclopediaRecordDecoded:
			owned = append(owned, paths[index].text)
			text, err := readEncyclopediaRegularFile(filepath.Join(root, filepath.FromSlash(paths[index].text)), int64(defaultEncyclopediaInventoryLimits().MaxResourceBytes)*4)
			if err != nil {
				return encyclopediaOwnedInventory{}, fmt.Errorf("read proven text %s: %w", paths[index].text, err)
			}
			if !bytes.Equal(text, []byte(prepared.texts[index])) {
				return encyclopediaOwnedInventory{}, fmt.Errorf("proven text %s does not match lossless decoding", paths[index].text)
			}
		case encyclopediaRecordUnresolved:
		default:
			return encyclopediaOwnedInventory{}, fmt.Errorf("research report record %s has invalid publication status %q", formatEncyclopediaResourceIdentifier(record.ResourceID), record.Status)
		}
	}
	if report.Artwork != nil {
		if len(report.Artwork.StagedAssets) != 0 {
			assetRoot := filepath.Join(root, "assets")
			images, err := inventoryEncyclopediaImages(assetRoot, defaultEncyclopediaImageLimits())
			if err != nil {
				return encyclopediaOwnedInventory{}, fmt.Errorf("verify staged encyclopedia artwork: %w", err)
			}
			if images.Measurements.RejectedCount != 0 {
				return encyclopediaOwnedInventory{}, fmt.Errorf("verify staged encyclopedia artwork: %d rejected images", images.Measurements.RejectedCount)
			}
			actualImages, err := marshalEncyclopediaImageInventory(images)
			if err != nil {
				return encyclopediaOwnedInventory{}, err
			}
			reportedStagedImages, err := stagedEncyclopediaImageInventory(*report.Artwork)
			if err != nil {
				return encyclopediaOwnedInventory{}, err
			}
			reportedImages, err := marshalEncyclopediaImageInventory(reportedStagedImages)
			if err != nil {
				return encyclopediaOwnedInventory{}, err
			}
			if !bytes.Equal(actualImages, reportedImages) {
				return encyclopediaOwnedInventory{}, fmt.Errorf("staged encyclopedia artwork differs from reported image inventory")
			}
		}
		for _, asset := range report.Artwork.StagedAssets {
			owned = append(owned, asset.StagedPath)
		}
	}
	return encyclopediaOwnedInventory{Files: owned}, nil
}

func validatePublishedEncyclopediaReportShape(report encyclopediaResearchReport) error {
	limits := defaultEncyclopediaInventoryLimits()
	if len(report.Sources) != 1 {
		return fmt.Errorf("encyclopedia text report has %d sources, want exactly ENCYTEXT.DLL", len(report.Sources))
	}
	source := report.Sources[0]
	if source.RootRole != encyclopediaReportSourceRole || source.Basename != "ENCYTEXT.DLL" || source.Kind != encyclopediaSourceDLL {
		return fmt.Errorf("encyclopedia text report has unsupported source contract %s/%s (%s)", source.RootRole, source.Basename, source.Kind)
	}
	if source.RawLength > limits.MaxSourceBytes {
		return fmt.Errorf("encyclopedia text source length %d exceeds %d-byte limit", source.RawLength, limits.MaxSourceBytes)
	}
	if len(report.Records) > limits.MaxResourceCount {
		return fmt.Errorf("encyclopedia text report has %d records, exceeds %d-record limit", len(report.Records), limits.MaxResourceCount)
	}
	aggregate := uint64(0)
	for _, record := range report.Records {
		if record.SourceRootRole != source.RootRole || record.SourceBasename != source.Basename {
			return fmt.Errorf("encyclopedia text record uses unsupported source %s/%s", record.SourceRootRole, record.SourceBasename)
		}
		if record.ResourceType.Kind != encyclopediaIdentifierNumeric || record.ResourceType.NumericID == nil || *record.ResourceType.NumericID != rtEncyclopediaText {
			return fmt.Errorf("encyclopedia text record has unsupported resource type %s", formatEncyclopediaResourceIdentifier(record.ResourceType))
		}
		if record.RawLength > limits.MaxResourceBytes {
			return fmt.Errorf("encyclopedia text record %s length %d exceeds %d-byte limit", formatEncyclopediaResourceIdentifier(record.ResourceID), record.RawLength, limits.MaxResourceBytes)
		}
		if aggregate > limits.MaxAggregateResourceBytes-record.RawLength {
			return fmt.Errorf("encyclopedia text aggregate exceeds %d-byte limit", limits.MaxAggregateResourceBytes)
		}
		aggregate += record.RawLength
	}
	return nil
}

func encyclopediaRuntimeCatalogPresent(root string) bool {
	for _, name := range []string{"catalog.json", "manifest.json"} {
		if _, err := os.Lstat(filepath.Join(root, name)); err == nil {
			return true
		}
	}
	return false
}
