package main

import (
	"bytes"
	"crypto/rand"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"time"
)

const (
	encyclopediaTransactionVersion = 1
	encyclopediaCandidatePrefix    = "encyclopedia-stage-candidate-"
	encyclopediaBackupPrefix       = "encyclopedia-stage-backup-"
	encyclopediaStaleLockPrefix    = "encyclopedia-stage-stale-lock-"
	encyclopediaWriterGuardSuffix  = ".guard"
	encyclopediaArtworkKind        = "encyclopedia-artwork-research"
	encyclopediaArtworkVersion     = 1
)

type encyclopediaAssetUse string

const (
	encyclopediaAssetReferenced encyclopediaAssetUse = "referenced"
	encyclopediaAssetUnused     encyclopediaAssetUse = "unreferenced"
)

type encyclopediaStagedAsset struct {
	SourceBasename string               `json:"source_basename"`
	StagedPath     string               `json:"staged_path"`
	RawLength      uint64               `json:"raw_length"`
	RawSHA256      string               `json:"raw_sha256"`
	Use            encyclopediaAssetUse `json:"use"`
}

type encyclopediaArtworkResearch struct {
	Kind          string                      `json:"kind"`
	SchemaVersion int                         `json:"schema_version"`
	Lookup        encyclopediaLookupInventory `json:"lookup_inventory"`
	Images        encyclopediaImageInventory  `json:"image_inventory"`
	StagedAssets  []encyclopediaStagedAsset   `json:"staged_assets"`
}

func prepareEncyclopediaArtwork(sourceDir, edataDir string, limits encyclopediaImageLimits) (encyclopediaArtworkResearch, error) {
	images, err := inventoryEncyclopediaImages(edataDir, limits)
	if err != nil {
		return encyclopediaArtworkResearch{}, fmt.Errorf("inventory encyclopedia artwork: %w", err)
	}
	if images.Measurements.RejectedCount != 0 {
		rejected := make([]string, 0, images.Measurements.RejectedCount)
		for _, image := range images.Images {
			if image.Status == encyclopediaImageRejected {
				rejected = append(rejected, fmt.Sprintf("%s (%s: %s)", image.Filename, image.DiagnosticCode, image.Diagnostic))
			}
		}
		return encyclopediaArtworkResearch{}, fmt.Errorf("rejected EData inputs: %s", strings.Join(rejected, "; "))
	}
	lookupBytes, err := readBoundedEncyclopediaSource(filepath.Join(sourceDir, "ENCYBMAP.DLL"), maxEncyclopediaLookupSourceBytes)
	if err != nil {
		return encyclopediaArtworkResearch{}, fmt.Errorf("read ENCYBMAP.DLL: %w", err)
	}
	lookup, err := inventoryEncyclopediaLookups("ENCYBMAP.DLL", lookupBytes, edataDir)
	if err != nil {
		return encyclopediaArtworkResearch{}, fmt.Errorf("inventory encyclopedia artwork lookups: %w", err)
	}
	artwork := encyclopediaArtworkResearch{
		Kind:          encyclopediaArtworkKind,
		SchemaVersion: encyclopediaArtworkVersion,
		Lookup:        lookup,
		Images:        images,
	}
	artwork.StagedAssets, err = encyclopediaStagedAssets(lookup, images)
	if err != nil {
		return encyclopediaArtworkResearch{}, err
	}
	return canonicalEncyclopediaArtworkResearch(artwork)
}

func encyclopediaStagedAssets(lookup encyclopediaLookupInventory, images encyclopediaImageInventory) ([]encyclopediaStagedAsset, error) {
	referenced := make(map[string]struct{}, len(lookup.References))
	for _, reference := range lookup.References {
		switch reference.Resolution {
		case encyclopediaLookupExact, encyclopediaLookupCaseFolded:
			if reference.MatchedBasename == "" {
				return nil, fmt.Errorf("resolved artwork lookup %d/%d has no matched basename", reference.LanguageID, reference.LogicalID)
			}
			referenced[reference.MatchedBasename] = struct{}{}
		case encyclopediaLookupMissing:
		case encyclopediaLookupAmbiguous:
			return nil, fmt.Errorf("case-ambiguous artwork lookup %d/%d has no safe staged binding", reference.LanguageID, reference.LogicalID)
		default:
			return nil, fmt.Errorf("artwork lookup %d/%d has unsupported resolution %q", reference.LanguageID, reference.LogicalID, reference.Resolution)
		}
	}
	lookupFiles := make(map[string]encyclopediaLookupFileRecord, len(lookup.Files))
	for _, file := range lookup.Files {
		lookupFiles[file.Basename] = file
	}
	assets := make([]encyclopediaStagedAsset, 0, len(images.Images))
	for _, image := range images.Images {
		if image.Status != encyclopediaImageValid {
			return nil, fmt.Errorf("EData source %s is not valid for staging", image.Filename)
		}
		file, ok := lookupFiles[image.Filename]
		if !ok || file.RawLength != image.RawLength || file.RawSHA256 != image.RawSHA256 || file.FileNumber != image.Number {
			return nil, fmt.Errorf("EData source identity mismatch for %s", image.Filename)
		}
		use := encyclopediaAssetUnused
		if _, ok := referenced[image.Filename]; ok {
			use = encyclopediaAssetReferenced
		}
		assets = append(assets, encyclopediaStagedAsset{
			SourceBasename: image.Filename,
			StagedPath:     filepath.ToSlash(filepath.Join("assets", fmt.Sprintf("EDATA.%03d", image.Number))),
			RawLength:      image.RawLength,
			RawSHA256:      image.RawSHA256,
			Use:            use,
		})
	}
	if len(lookupFiles) != len(assets) {
		return nil, fmt.Errorf("lookup inventory has %d EData files but image inventory has %d", len(lookupFiles), len(assets))
	}
	sort.Slice(assets, func(i, j int) bool { return assets[i].StagedPath < assets[j].StagedPath })
	return assets, nil
}

func canonicalEncyclopediaArtworkResearch(artwork encyclopediaArtworkResearch) (encyclopediaArtworkResearch, error) {
	if artwork.Kind != encyclopediaArtworkKind || artwork.SchemaVersion != encyclopediaArtworkVersion {
		return encyclopediaArtworkResearch{}, fmt.Errorf("unsupported encyclopedia artwork research contract %q/%d", artwork.Kind, artwork.SchemaVersion)
	}
	canonical := artwork
	canonical.Lookup.SourceRawBytes = nil
	canonical.Lookup.Blocks = append([]encyclopediaLookupBlockRecord(nil), artwork.Lookup.Blocks...)
	for index := range canonical.Lookup.Blocks {
		canonical.Lookup.Blocks[index].RawBytes = nil
	}
	canonical.Lookup.Lookups = cloneEncyclopediaLookups(artwork.Lookup.Lookups)
	canonical.Lookup.References = append([]encyclopediaLookupReference(nil), artwork.Lookup.References...)
	canonical.Lookup.Files = append([]encyclopediaLookupFileRecord(nil), artwork.Lookup.Files...)
	canonical.Lookup.DuplicateReferences = append([]encyclopediaDuplicateLookupReference(nil), artwork.Lookup.DuplicateReferences...)
	for index := range canonical.Lookup.DuplicateReferences {
		canonical.Lookup.DuplicateReferences[index].References = append([]encyclopediaLookupReference(nil), artwork.Lookup.DuplicateReferences[index].References...)
	}
	canonical.Lookup.CaseAmbiguities = append([]encyclopediaLookupCaseAmbiguity(nil), artwork.Lookup.CaseAmbiguities...)
	for index := range canonical.Lookup.CaseAmbiguities {
		canonical.Lookup.CaseAmbiguities[index].Candidates = append([]string(nil), artwork.Lookup.CaseAmbiguities[index].Candidates...)
	}
	canonical.Lookup.MissingFilenames = append([]encyclopediaLookupReference(nil), artwork.Lookup.MissingFilenames...)
	canonical.Lookup.UnreferencedFiles = append([]encyclopediaLookupFileRecord(nil), artwork.Lookup.UnreferencedFiles...)
	canonical.Images.Images = append([]encyclopediaImageRecord(nil), artwork.Images.Images...)
	canonical.StagedAssets = append([]encyclopediaStagedAsset(nil), artwork.StagedAssets...)

	if canonical.Lookup.SourceBasename != "ENCYBMAP.DLL" || !validSHA256(canonical.Lookup.SourceRawSHA256) {
		return encyclopediaArtworkResearch{}, fmt.Errorf("artwork lookup source identity is invalid")
	}
	if canonical.Lookup.SourceRawLength == 0 {
		return encyclopediaArtworkResearch{}, fmt.Errorf("artwork lookup source length is zero")
	}
	if _, err := marshalEncyclopediaImageInventory(canonical.Images); err != nil {
		return encyclopediaArtworkResearch{}, err
	}
	sort.Slice(canonical.Lookup.Blocks, func(i, j int) bool {
		left, right := canonical.Lookup.Blocks[i], canonical.Lookup.Blocks[j]
		if left.LanguageID != right.LanguageID {
			return left.LanguageID < right.LanguageID
		}
		return compareEncyclopediaResourceIdentifier(left.BlockID, right.BlockID) < 0
	})
	sort.Slice(canonical.Lookup.References, func(i, j int) bool {
		left, right := canonical.Lookup.References[i], canonical.Lookup.References[j]
		if left.LanguageID != right.LanguageID {
			return left.LanguageID < right.LanguageID
		}
		return left.LogicalID < right.LogicalID
	})
	sort.Slice(canonical.Lookup.Files, func(i, j int) bool { return canonical.Lookup.Files[i].Basename < canonical.Lookup.Files[j].Basename })
	sort.Slice(canonical.Lookup.MissingFilenames, func(i, j int) bool {
		left, right := canonical.Lookup.MissingFilenames[i], canonical.Lookup.MissingFilenames[j]
		if left.LanguageID != right.LanguageID {
			return left.LanguageID < right.LanguageID
		}
		return left.LogicalID < right.LogicalID
	})
	sort.Slice(canonical.Lookup.UnreferencedFiles, func(i, j int) bool {
		return canonical.Lookup.UnreferencedFiles[i].Basename < canonical.Lookup.UnreferencedFiles[j].Basename
	})
	for index := range canonical.Lookup.DuplicateReferences {
		sort.Slice(canonical.Lookup.DuplicateReferences[index].References, func(i, j int) bool {
			left := canonical.Lookup.DuplicateReferences[index].References[i]
			right := canonical.Lookup.DuplicateReferences[index].References[j]
			if left.LanguageID != right.LanguageID {
				return left.LanguageID < right.LanguageID
			}
			return left.LogicalID < right.LogicalID
		})
	}
	sort.Slice(canonical.Lookup.DuplicateReferences, func(i, j int) bool {
		left := foldEncyclopediaFilename(canonical.Lookup.DuplicateReferences[i].Filename)
		right := foldEncyclopediaFilename(canonical.Lookup.DuplicateReferences[j].Filename)
		if left != right {
			return left < right
		}
		return canonical.Lookup.DuplicateReferences[i].Filename < canonical.Lookup.DuplicateReferences[j].Filename
	})
	for index := range canonical.Lookup.CaseAmbiguities {
		sort.Strings(canonical.Lookup.CaseAmbiguities[index].Candidates)
	}
	sort.Slice(canonical.Lookup.CaseAmbiguities, func(i, j int) bool {
		return canonical.Lookup.CaseAmbiguities[i].FoldedFilename < canonical.Lookup.CaseAmbiguities[j].FoldedFilename
	})
	sort.Slice(canonical.Images.Images, func(i, j int) bool {
		if canonical.Images.Images[i].Number != canonical.Images.Images[j].Number {
			return canonical.Images.Images[i].Number < canonical.Images.Images[j].Number
		}
		return canonical.Images.Images[i].Filename < canonical.Images.Images[j].Filename
	})
	sort.Slice(canonical.StagedAssets, func(i, j int) bool {
		return canonical.StagedAssets[i].StagedPath < canonical.StagedAssets[j].StagedPath
	})

	if err := validateEncyclopediaArtworkLookupEvidence(canonical.Lookup); err != nil {
		return encyclopediaArtworkResearch{}, err
	}
	wantAssets, err := encyclopediaStagedAssets(canonical.Lookup, canonical.Images)
	if err != nil {
		return encyclopediaArtworkResearch{}, err
	}
	if len(wantAssets) != len(canonical.StagedAssets) {
		return encyclopediaArtworkResearch{}, fmt.Errorf("artwork staged ownership count is %d, want %d", len(canonical.StagedAssets), len(wantAssets))
	}
	for index := range wantAssets {
		if canonical.StagedAssets[index] != wantAssets[index] {
			return encyclopediaArtworkResearch{}, fmt.Errorf("artwork staged ownership differs for %s", wantAssets[index].SourceBasename)
		}
	}
	return canonical, nil
}

func validateEncyclopediaArtworkLookupEvidence(lookup encyclopediaLookupInventory) error {
	if lookup.SourceRawLength > maxEncyclopediaLookupSourceBytes {
		return fmt.Errorf("artwork lookup source length %d exceeds %d-byte limit", lookup.SourceRawLength, maxEncyclopediaLookupSourceBytes)
	}
	if len(lookup.Blocks) == 0 {
		return fmt.Errorf("artwork lookup evidence has no RT_STRING blocks")
	}
	type lookupKey struct {
		language uint16
		logical  uint32
	}
	values := make(map[lookupKey]string)
	for language, languageLookups := range lookup.Lookups {
		for logicalID, filename := range languageLookups {
			if _, ok := parseEncyclopediaEDataFilename(filename); !ok {
				return fmt.Errorf("artwork lookup %d/%d has invalid filename %q", language, logicalID, filename)
			}
			values[lookupKey{language: language, logical: logicalID}] = filename
		}
	}
	blockKeys := make(map[encyclopediaLookupBlockKey]struct{}, len(lookup.Blocks))
	nonempty := 0
	for _, block := range lookup.Blocks {
		if err := validateEncyclopediaResourceIdentifier(block.BlockID); err != nil {
			return fmt.Errorf("invalid artwork RT_STRING block identity: %w", err)
		}
		if !validSHA256(block.RawSHA256) || block.RawLength == 0 || block.NonemptyEntryCount < 0 {
			return fmt.Errorf("artwork RT_STRING block %s has invalid raw facts", formatEncyclopediaResourceIdentifier(block.BlockID))
		}
		key := encyclopediaLookupBlockKey{LanguageID: block.LanguageID, Kind: block.BlockID.Kind, Name: block.BlockID.Name}
		if block.BlockID.NumericID != nil {
			key.NumericID = *block.BlockID.NumericID
		}
		if _, exists := blockKeys[key]; exists {
			return fmt.Errorf("duplicate artwork RT_STRING block %s for LANGID %d", formatEncyclopediaResourceIdentifier(block.BlockID), block.LanguageID)
		}
		blockKeys[key] = struct{}{}
		switch block.BlockID.Kind {
		case encyclopediaIdentifierNamed:
			if block.Status != encyclopediaRecordUnresolved || block.Unresolved == nil || strings.TrimSpace(block.Unresolved.Reason) == "" || strings.TrimSpace(block.Unresolved.NextProof) == "" {
				return fmt.Errorf("named artwork RT_STRING block must retain unresolved evidence")
			}
		case encyclopediaIdentifierNumeric:
			if block.Status != encyclopediaRecordDecoded || block.Unresolved != nil || block.BlockID.NumericID == nil || *block.BlockID.NumericID == 0 {
				return fmt.Errorf("numeric artwork RT_STRING block has invalid decode status")
			}
			if *block.BlockID.NumericID-1 > (^uint32(0)-15)/16 {
				return fmt.Errorf("artwork RT_STRING block ID %d overflows logical IDs", *block.BlockID.NumericID)
			}
			base := (*block.BlockID.NumericID - 1) * 16
			count := 0
			for slot := uint32(0); slot < 16; slot++ {
				if _, ok := values[lookupKey{language: block.LanguageID, logical: base + slot}]; ok {
					count++
				}
			}
			if count != block.NonemptyEntryCount {
				return fmt.Errorf("artwork RT_STRING block %s reports %d nonempty entries, found %d", formatEncyclopediaResourceIdentifier(block.BlockID), block.NonemptyEntryCount, count)
			}
			nonempty += count
		}
	}
	if nonempty != len(values) || len(lookup.References) != len(values) {
		return fmt.Errorf("artwork lookup/reference counts do not match decoded block entries")
	}

	files := make(map[string]encyclopediaLookupFileRecord, len(lookup.Files))
	filesByFold := make(map[string][]encyclopediaLookupFileRecord, len(lookup.Files))
	for _, file := range lookup.Files {
		number, ok := parseEncyclopediaEDataFilename(file.Basename)
		if !ok || number != file.FileNumber || !validSHA256(file.RawSHA256) {
			return fmt.Errorf("artwork file identity is invalid for %q", file.Basename)
		}
		if _, exists := files[file.Basename]; exists {
			return fmt.Errorf("duplicate artwork file identity %q", file.Basename)
		}
		files[file.Basename] = file
		folded := foldEncyclopediaFilename(file.Basename)
		filesByFold[folded] = append(filesByFold[folded], file)
	}
	if len(lookup.CaseAmbiguities) != 0 {
		return fmt.Errorf("case-ambiguous EData identities cannot be published as staged artwork evidence")
	}
	seenReferences := make(map[lookupKey]struct{}, len(lookup.References))
	var missing []encyclopediaLookupReference
	referencedFiles := make(map[string]struct{})
	referencesByFold := make(map[string][]encyclopediaLookupReference)
	for _, reference := range lookup.References {
		key := lookupKey{language: reference.LanguageID, logical: reference.LogicalID}
		filename, ok := values[key]
		if !ok || filename != reference.Filename {
			return fmt.Errorf("artwork reference %d/%d does not match decoded lookup", reference.LanguageID, reference.LogicalID)
		}
		if _, exists := seenReferences[key]; exists {
			return fmt.Errorf("duplicate artwork reference %d/%d", reference.LanguageID, reference.LogicalID)
		}
		seenReferences[key] = struct{}{}
		fileNumber, validFilename := parseEncyclopediaEDataFilename(reference.Filename)
		if !validFilename || reference.FileNumber == nil || *reference.FileNumber != fileNumber {
			return fmt.Errorf("artwork reference %d/%d has invalid file identity", reference.LanguageID, reference.LogicalID)
		}
		referencesByFold[foldEncyclopediaFilename(reference.Filename)] = append(referencesByFold[foldEncyclopediaFilename(reference.Filename)], reference)
		candidates := filesByFold[foldEncyclopediaFilename(reference.Filename)]
		switch len(candidates) {
		case 0:
			if reference.Resolution != encyclopediaLookupMissing {
				return fmt.Errorf("artwork reference %d/%d resolves to %q but the file is missing", reference.LanguageID, reference.LogicalID, reference.Resolution)
			}
			if reference.MatchedBasename != "" {
				return fmt.Errorf("missing artwork reference %d/%d has a matched basename", reference.LanguageID, reference.LogicalID)
			}
			missing = append(missing, reference)
		case 1:
			file := candidates[0]
			wantResolution := encyclopediaLookupCaseFolded
			if file.Basename == reference.Filename {
				wantResolution = encyclopediaLookupExact
			}
			if reference.Resolution == encyclopediaLookupMissing {
				return fmt.Errorf("artwork reference %d/%d is marked missing but %s exists", reference.LanguageID, reference.LogicalID, file.Basename)
			}
			if reference.Resolution != wantResolution || reference.MatchedBasename != file.Basename {
				return fmt.Errorf("artwork reference %d/%d resolution %q/%q does not match file %s", reference.LanguageID, reference.LogicalID, reference.Resolution, reference.MatchedBasename, file.Basename)
			}
			referencedFiles[file.Basename] = struct{}{}
		default:
			return fmt.Errorf("artwork reference %d/%d has %d case-ambiguous file candidates", reference.LanguageID, reference.LogicalID, len(candidates))
		}
	}
	if len(missing) != len(lookup.MissingFilenames) {
		return fmt.Errorf("artwork missing-reference count is %d, want %d", len(lookup.MissingFilenames), len(missing))
	}
	wantMissingJSON, err := json.Marshal(missing)
	if err != nil {
		return err
	}
	gotMissingJSON, err := json.Marshal(lookup.MissingFilenames)
	if err != nil {
		return err
	}
	if !bytes.Equal(wantMissingJSON, gotMissingJSON) {
		return fmt.Errorf("artwork missing-reference evidence differs from decoded lookups")
	}
	var wantUnreferenced []encyclopediaLookupFileRecord
	for _, file := range lookup.Files {
		if _, ok := referencedFiles[file.Basename]; !ok {
			wantUnreferenced = append(wantUnreferenced, file)
		}
	}
	if len(wantUnreferenced) != len(lookup.UnreferencedFiles) {
		return fmt.Errorf("artwork unreferenced-file count is %d, want %d", len(lookup.UnreferencedFiles), len(wantUnreferenced))
	}
	for index := range wantUnreferenced {
		if wantUnreferenced[index] != lookup.UnreferencedFiles[index] {
			return fmt.Errorf("artwork unreferenced-file evidence differs for %s", wantUnreferenced[index].Basename)
		}
	}
	var wantDuplicates []encyclopediaDuplicateLookupReference
	for _, references := range referencesByFold {
		if len(references) > 1 {
			wantDuplicates = append(wantDuplicates, encyclopediaDuplicateLookupReference{
				Filename: canonicalLookupFilename(references), References: references,
			})
		}
	}
	sort.Slice(wantDuplicates, func(i, j int) bool {
		return foldEncyclopediaFilename(wantDuplicates[i].Filename) < foldEncyclopediaFilename(wantDuplicates[j].Filename)
	})
	wantDuplicateJSON, err := json.Marshal(wantDuplicates)
	if err != nil {
		return err
	}
	gotDuplicateJSON, err := json.Marshal(lookup.DuplicateReferences)
	if err != nil {
		return err
	}
	if !bytes.Equal(wantDuplicateJSON, gotDuplicateJSON) {
		return fmt.Errorf("artwork duplicate-reference evidence does not match decoded lookups")
	}
	return nil
}

func cloneEncyclopediaLookups(source map[uint16]map[uint32]string) map[uint16]map[uint32]string {
	result := make(map[uint16]map[uint32]string, len(source))
	for language, values := range source {
		copyValues := make(map[uint32]string, len(values))
		for id, value := range values {
			copyValues[id] = value
		}
		result[language] = copyValues
	}
	return result
}

func stagedEncyclopediaImageInventory(artwork encyclopediaArtworkResearch) (encyclopediaImageInventory, error) {
	result := artwork.Images
	result.Images = append([]encyclopediaImageRecord(nil), artwork.Images.Images...)
	stagedBasenames := make(map[string]string, len(artwork.StagedAssets))
	for _, asset := range artwork.StagedAssets {
		stagedBasenames[asset.SourceBasename] = filepath.Base(filepath.FromSlash(asset.StagedPath))
	}
	for index := range result.Images {
		staged, ok := stagedBasenames[result.Images[index].Filename]
		if !ok {
			return encyclopediaImageInventory{}, fmt.Errorf("image inventory source %s has no staged ownership record", result.Images[index].Filename)
		}
		result.Images[index].Filename = staged
	}
	for source, staged := range stagedBasenames {
		if result.Measurements.MaxImageBytesFilename == source {
			result.Measurements.MaxImageBytesFilename = staged
		}
		if result.Measurements.MaxPixelsFilename == source {
			result.Measurements.MaxPixelsFilename = staged
		}
		if result.Measurements.MaxDecodedBytesFilename == source {
			result.Measurements.MaxDecodedBytesFilename = staged
		}
	}
	sort.Slice(result.Images, func(i, j int) bool {
		if result.Images[i].Number != result.Images[j].Number {
			return result.Images[i].Number < result.Images[j].Number
		}
		return result.Images[i].Filename < result.Images[j].Filename
	})
	return result, nil
}

func copyEncyclopediaStagedAsset(sourceRoot, candidateRoot string, asset encyclopediaStagedAsset) error {
	sourcePath := filepath.Join(sourceRoot, asset.SourceBasename)
	pathInfo, err := os.Lstat(sourcePath)
	if err != nil {
		return err
	}
	if pathInfo.Mode()&os.ModeSymlink != 0 || !pathInfo.Mode().IsRegular() || pathInfo.Size() < 0 || uint64(pathInfo.Size()) != asset.RawLength {
		return fmt.Errorf("EData source %s changed identity or length before copy", asset.SourceBasename)
	}
	source, err := os.Open(sourcePath)
	if err != nil {
		return err
	}
	defer source.Close()
	opened, err := source.Stat()
	if err != nil {
		return err
	}
	if !opened.Mode().IsRegular() || !os.SameFile(pathInfo, opened) {
		return fmt.Errorf("EData source %s changed identity before copy", asset.SourceBasename)
	}

	destinationPath := filepath.Join(candidateRoot, filepath.FromSlash(asset.StagedPath))
	if err := os.MkdirAll(filepath.Dir(destinationPath), 0o755); err != nil {
		return err
	}
	destination, err := os.OpenFile(destinationPath, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0o644)
	if err != nil {
		return err
	}
	hash := sha256.New()
	written, copyErr := io.Copy(io.MultiWriter(destination, hash), io.LimitReader(source, int64(asset.RawLength)+1))
	closeErr := destination.Close()
	if copyErr != nil {
		return copyErr
	}
	if closeErr != nil {
		return closeErr
	}
	if written < 0 || uint64(written) != asset.RawLength || hex.EncodeToString(hash.Sum(nil)) != asset.RawSHA256 {
		return fmt.Errorf("EData source %s changed while copying", asset.SourceBasename)
	}
	var extra [1]byte
	if count, readErr := source.Read(extra[:]); count != 0 || (readErr != nil && !errors.Is(readErr, io.EOF)) {
		return fmt.Errorf("EData source %s grew while copying", asset.SourceBasename)
	}
	pathAfter, err := os.Lstat(sourcePath)
	if err != nil || pathAfter.Mode()&os.ModeSymlink != 0 || !pathAfter.Mode().IsRegular() || !os.SameFile(opened, pathAfter) || pathAfter.Size() != opened.Size() {
		return fmt.Errorf("EData source %s changed identity after copy", asset.SourceBasename)
	}
	return nil
}

var errEncyclopediaWriterGuardBusy = errors.New("encyclopedia writer guard is locked")

type encyclopediaCandidateBuilder func(root string) error

type encyclopediaOwnedInventory struct {
	Files []string
}

type encyclopediaOwnedInspector func(root string) (encyclopediaOwnedInventory, error)

type encyclopediaDirectoryRequest struct {
	OutputDir       string
	SourceRoots     []string
	ModRoots        []string
	Force           bool
	BuildCandidate  encyclopediaCandidateBuilder
	InspectOwned    encyclopediaOwnedInspector
	RecoveryCommand string
	Log             io.Writer
}

type encyclopediaPublicationResult struct {
	Changed   bool
	Recovered bool
}

type encyclopediaTransactionPhase string

const (
	encyclopediaPhaseCandidateValidated  encyclopediaTransactionPhase = "candidate_validated"
	encyclopediaPhaseDestinationBackedUp encyclopediaTransactionPhase = "destination_backed_up"
	encyclopediaPhaseCandidatePublished  encyclopediaTransactionPhase = "candidate_published"
	encyclopediaPhaseCleanupPending      encyclopediaTransactionPhase = "cleanup_pending"
)

type encyclopediaTransaction struct {
	Version        int                          `json:"version"`
	OutputBase     string                       `json:"output_base"`
	CandidateBase  string                       `json:"candidate_base"`
	BackupBase     string                       `json:"backup_base"`
	HadDestination bool                         `json:"had_destination"`
	Phase          encyclopediaTransactionPhase `json:"phase"`
}

type encyclopediaWriterMarker struct {
	Version   int    `json:"version"`
	PID       int    `json:"pid"`
	Token     string `json:"token"`
	CreatedAt string `json:"created_at"`
}

type encyclopediaPublicationOps struct {
	rename       func(string, string) error
	remove       func(string) error
	removeAll    func(string) error
	writeJournal func(string, encyclopediaTransaction) error
}

func defaultEncyclopediaPublicationOps() encyclopediaPublicationOps {
	return encyclopediaPublicationOps{
		rename:    os.Rename,
		remove:    os.Remove,
		removeAll: os.RemoveAll,
		writeJournal: func(path string, transaction encyclopediaTransaction) error {
			return writeEncyclopediaTransaction(path, transaction)
		},
	}
}

type encyclopediaPublicationPaths struct {
	output  string
	parent  string
	base    string
	marker  string
	journal string
}

type encyclopediaWriterLease struct {
	path  string
	token string
	guard *os.File
}

func stageEncyclopediaDirectory(request encyclopediaDirectoryRequest) (encyclopediaPublicationResult, error) {
	return stageEncyclopediaDirectoryWithOps(request, defaultEncyclopediaPublicationOps())
}

func stageEncyclopediaDirectoryWithOps(request encyclopediaDirectoryRequest, ops encyclopediaPublicationOps) (encyclopediaPublicationResult, error) {
	paths, err := prepareEncyclopediaPublicationPaths(request.OutputDir)
	if err != nil {
		return encyclopediaPublicationResult{}, err
	}
	if err := validateEncyclopediaRootCollisions(paths.output, request.SourceRoots, request.ModRoots); err != nil {
		return encyclopediaPublicationResult{}, err
	}
	if request.BuildCandidate == nil {
		return encyclopediaPublicationResult{}, fmt.Errorf("encyclopedia publisher requires a candidate builder")
	}
	if request.InspectOwned == nil {
		return encyclopediaPublicationResult{}, fmt.Errorf("encyclopedia publisher requires an ownership inspector")
	}
	if request.Log == nil {
		request.Log = io.Discard
	}
	if err := os.MkdirAll(paths.parent, 0o755); err != nil {
		return encyclopediaPublicationResult{}, fmt.Errorf("create encyclopedia output parent: %w", err)
	}
	lease, err := acquireEncyclopediaWriter(paths.marker)
	if err != nil {
		return encyclopediaPublicationResult{}, err
	}
	defer releaseEncyclopediaWriter(lease)

	recovered, err := recoverEncyclopediaPublication(paths, request.InspectOwned, request.RecoveryCommand, ops)
	if err != nil {
		return encyclopediaPublicationResult{}, err
	}

	candidate, err := os.MkdirTemp(paths.parent, "."+paths.base+"-"+encyclopediaCandidatePrefix)
	if err != nil {
		return encyclopediaPublicationResult{Recovered: recovered}, fmt.Errorf("create sibling encyclopedia candidate: %w", err)
	}
	candidateOwnedByRun := true
	defer func() {
		if candidateOwnedByRun {
			_ = os.RemoveAll(candidate)
		}
	}()
	if err := request.BuildCandidate(candidate); err != nil {
		return encyclopediaPublicationResult{Recovered: recovered}, fmt.Errorf("build encyclopedia candidate: %w", err)
	}
	candidateInventory, err := inspectEncyclopediaOwnedDirectory(candidate, request.InspectOwned)
	if err != nil {
		return encyclopediaPublicationResult{Recovered: recovered}, fmt.Errorf("validate encyclopedia candidate: %w", err)
	}

	destinationExists, err := encyclopediaDirectoryExists(paths.output)
	if err != nil {
		return encyclopediaPublicationResult{Recovered: recovered}, err
	}
	var destinationInventory encyclopediaOwnedInventory
	if destinationExists {
		destinationInventory, err = inspectEncyclopediaOwnedDirectory(paths.output, request.InspectOwned)
		if err != nil {
			return encyclopediaPublicationResult{Recovered: recovered}, fmt.Errorf("output_not_owned: %w", err)
		}
		same, err := encyclopediaDirectoriesEqual(paths.output, destinationInventory, candidate, candidateInventory)
		if err != nil {
			return encyclopediaPublicationResult{Recovered: recovered}, fmt.Errorf("compare encyclopedia output: %w", err)
		}
		if same {
			return encyclopediaPublicationResult{Changed: false, Recovered: recovered}, nil
		}
		if !request.Force {
			return encyclopediaPublicationResult{Recovered: recovered}, fmt.Errorf("output_changed: generated encyclopedia output differs (use --force to replace it)")
		}
	}

	backupToken, err := randomEncyclopediaToken()
	if err != nil {
		return encyclopediaPublicationResult{Recovered: recovered}, fmt.Errorf("create encyclopedia backup identity: %w", err)
	}
	transaction := encyclopediaTransaction{
		Version:        encyclopediaTransactionVersion,
		OutputBase:     paths.base,
		CandidateBase:  filepath.Base(candidate),
		BackupBase:     "." + paths.base + "-" + encyclopediaBackupPrefix + backupToken,
		HadDestination: destinationExists,
		Phase:          encyclopediaPhaseCandidateValidated,
	}
	backup := filepath.Join(paths.parent, transaction.BackupBase)
	if err := ops.writeJournal(paths.journal, transaction); err != nil {
		return encyclopediaPublicationResult{Recovered: recovered}, fmt.Errorf("record candidate transaction: %w", err)
	}

	oldMoved := false
	newInstalled := false
	rollback := func(cause error) (encyclopediaPublicationResult, error) {
		recoveryErr := rollbackEncyclopediaPublication(paths, candidate, backup, oldMoved, newInstalled, ops)
		if recoveryErr != nil {
			candidateOwnedByRun = false
			return encyclopediaPublicationResult{Recovered: recovered}, fmt.Errorf("publication failed: %v; automatic recovery failed: %w; recover with: %s", cause, recoveryErr, encyclopediaRecoveryCommand(request.RecoveryCommand, paths.output))
		}
		return encyclopediaPublicationResult{Recovered: recovered}, cause
	}

	if destinationExists {
		if err := ops.rename(paths.output, backup); err != nil {
			return rollback(fmt.Errorf("move prior encyclopedia output to backup: %w", err))
		}
		oldMoved = true
	}
	transaction.Phase = encyclopediaPhaseDestinationBackedUp
	if err := ops.writeJournal(paths.journal, transaction); err != nil {
		return rollback(fmt.Errorf("record backed-up transaction: %w", err))
	}
	if err := ops.rename(candidate, paths.output); err != nil {
		return rollback(fmt.Errorf("publish encyclopedia candidate: %w", err))
	}
	candidateOwnedByRun = false
	newInstalled = true
	transaction.Phase = encyclopediaPhaseCandidatePublished
	if err := ops.writeJournal(paths.journal, transaction); err != nil {
		return rollback(fmt.Errorf("record published transaction: %w", err))
	}
	if _, err := inspectEncyclopediaOwnedDirectory(paths.output, request.InspectOwned); err != nil {
		return rollback(fmt.Errorf("validate published encyclopedia output: %w", err))
	}
	if oldMoved {
		if _, err := inspectEncyclopediaOwnedDirectory(backup, request.InspectOwned); err != nil {
			return rollback(fmt.Errorf("validate prior encyclopedia backup before cleanup: %w", err))
		}
	}
	transaction.Phase = encyclopediaPhaseCleanupPending
	if err := ops.writeJournal(paths.journal, transaction); err != nil {
		return rollback(fmt.Errorf("record cleanup transaction: %w", err))
	}
	if oldMoved {
		if err := ops.removeAll(backup); err != nil {
			return encyclopediaPublicationResult{Changed: true, Recovered: recovered}, fmt.Errorf("published encyclopedia output but backup cleanup failed: %w; recover with: %s", err, encyclopediaRecoveryCommand(request.RecoveryCommand, paths.output))
		}
	}
	if err := ops.remove(paths.journal); err != nil && !os.IsNotExist(err) {
		return encyclopediaPublicationResult{Changed: true, Recovered: recovered}, fmt.Errorf("published encyclopedia output but transaction cleanup failed: %w; recover with: %s", err, encyclopediaRecoveryCommand(request.RecoveryCommand, paths.output))
	}
	if err := syncEncyclopediaDirectory(paths.parent); err != nil {
		return encyclopediaPublicationResult{Changed: true, Recovered: recovered}, fmt.Errorf("sync encyclopedia output parent: %w", err)
	}
	fmt.Fprintf(request.Log, "Published encyclopedia directory %s\n", paths.output)
	return encyclopediaPublicationResult{Changed: true, Recovered: recovered}, nil
}

func rollbackEncyclopediaPublication(paths encyclopediaPublicationPaths, candidate, backup string, oldMoved, newInstalled bool, ops encyclopediaPublicationOps) error {
	var recovery []error
	if newInstalled {
		if err := ops.rename(paths.output, candidate); err != nil {
			recovery = append(recovery, fmt.Errorf("retain failed candidate: %w", err))
			return errors.Join(recovery...)
		}
	}
	if oldMoved {
		if err := ops.rename(backup, paths.output); err != nil {
			recovery = append(recovery, fmt.Errorf("restore prior output: %w", err))
		}
	}
	if len(recovery) != 0 {
		return errors.Join(recovery...)
	}
	if err := ops.removeAll(candidate); err != nil && !os.IsNotExist(err) {
		recovery = append(recovery, fmt.Errorf("remove failed candidate: %w", err))
	}
	if err := ops.remove(paths.journal); err != nil && !os.IsNotExist(err) {
		recovery = append(recovery, fmt.Errorf("remove transaction journal: %w", err))
	}
	if len(recovery) != 0 {
		return errors.Join(recovery...)
	}
	return syncEncyclopediaDirectory(paths.parent)
}

func prepareEncyclopediaPublicationPaths(output string) (encyclopediaPublicationPaths, error) {
	resolved, err := resolveEncyclopediaOutputPath(output)
	if err != nil {
		return encyclopediaPublicationPaths{}, err
	}
	base := filepath.Base(resolved)
	if base == "." || base == string(filepath.Separator) || base == "" {
		return encyclopediaPublicationPaths{}, fmt.Errorf("path_collision: encyclopedia output must name a directory")
	}
	parent := filepath.Dir(resolved)
	return encyclopediaPublicationPaths{
		output:  resolved,
		parent:  parent,
		base:    base,
		marker:  filepath.Join(parent, "."+base+".encyclopedia-stage.lock"),
		journal: filepath.Join(parent, "."+base+".encyclopedia-stage.transaction.json"),
	}, nil
}

func writeEncyclopediaTransaction(path string, transaction encyclopediaTransaction) error {
	encoded, err := json.MarshalIndent(transaction, "", "  ")
	if err != nil {
		return err
	}
	if err := writeFileAtomically(path, append(encoded, '\n'), 0o600); err != nil {
		return err
	}
	return syncEncyclopediaDirectory(filepath.Dir(path))
}

func acquireEncyclopediaWriter(path string) (encyclopediaWriterLease, error) {
	guard, err := acquireEncyclopediaWriterGuard(path + encyclopediaWriterGuardSuffix)
	if err != nil {
		if errors.Is(err, errEncyclopediaWriterGuardBusy) {
			return encyclopediaWriterLease{}, fmt.Errorf("stage_busy: encyclopedia writer owns %s", path)
		}
		return encyclopediaWriterLease{}, fmt.Errorf("acquire encyclopedia writer guard: %w", err)
	}
	keepGuard := false
	defer func() {
		if !keepGuard {
			releaseEncyclopediaWriterGuard(guard)
		}
	}()

	for attempts := 0; attempts < 4; attempts++ {
		token, err := randomEncyclopediaToken()
		if err != nil {
			return encyclopediaWriterLease{}, fmt.Errorf("create encyclopedia writer identity: %w", err)
		}
		marker := encyclopediaWriterMarker{
			Version:   encyclopediaTransactionVersion,
			PID:       os.Getpid(),
			Token:     token,
			CreatedAt: time.Now().UTC().Format(time.RFC3339Nano),
		}
		encoded, err := json.Marshal(marker)
		if err != nil {
			return encyclopediaWriterLease{}, err
		}
		file, err := os.OpenFile(path, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0o600)
		if err == nil {
			if _, err = file.Write(append(encoded, '\n')); err == nil {
				err = file.Sync()
			}
			closeErr := file.Close()
			if err == nil {
				err = closeErr
			}
			if err != nil {
				_ = os.Remove(path)
				return encyclopediaWriterLease{}, fmt.Errorf("write encyclopedia writer marker: %w", err)
			}
			keepGuard = true
			return encyclopediaWriterLease{path: path, token: token, guard: guard}, nil
		}
		if !os.IsExist(err) {
			return encyclopediaWriterLease{}, fmt.Errorf("create encyclopedia writer marker: %w", err)
		}
		existing, err := readEncyclopediaWriterMarker(path)
		if err != nil {
			return encyclopediaWriterLease{}, fmt.Errorf("stage_busy: writer marker cannot be safely inspected: %w", err)
		}
		alive, err := encyclopediaProcessAlive(existing.PID)
		if err != nil {
			return encyclopediaWriterLease{}, fmt.Errorf("stage_busy: cannot rule out writer pid %d: %w", existing.PID, err)
		}
		if alive {
			return encyclopediaWriterLease{}, fmt.Errorf("stage_busy: encyclopedia writer pid %d owns %s", existing.PID, path)
		}
		staleToken, err := randomEncyclopediaToken()
		if err != nil {
			return encyclopediaWriterLease{}, fmt.Errorf("create stale writer identity: %w", err)
		}
		stale := filepath.Join(filepath.Dir(path), "."+filepath.Base(path)+"-"+encyclopediaStaleLockPrefix+staleToken)
		if err := os.Rename(path, stale); err != nil {
			if os.IsNotExist(err) {
				continue
			}
			return encyclopediaWriterLease{}, fmt.Errorf("stage_busy: quarantine stale writer marker: %w", err)
		}
		_ = os.Remove(stale)
	}
	return encyclopediaWriterLease{}, fmt.Errorf("stage_busy: writer marker changed repeatedly")
}

func releaseEncyclopediaWriter(lease encyclopediaWriterLease) {
	marker, err := readEncyclopediaWriterMarker(lease.path)
	if err == nil && marker.Token == lease.token {
		_ = os.Remove(lease.path)
	}
	releaseEncyclopediaWriterGuard(lease.guard)
}

func acquireEncyclopediaWriterGuard(path string) (*os.File, error) {
	// The guard inode is intentionally persistent. Removing a lock file after
	// unlocking lets a contender lock the old inode while another process
	// creates and locks a replacement at the same path.
	guard, err := os.OpenFile(path, os.O_RDWR|os.O_CREATE|os.O_EXCL, 0o600)
	if err != nil {
		if !os.IsExist(err) {
			return nil, err
		}
		guard, err = os.OpenFile(path, os.O_RDWR, 0)
		if err != nil {
			return nil, err
		}
	}
	closeWithError := func(cause error) (*os.File, error) {
		_ = guard.Close()
		return nil, cause
	}
	pathInfo, err := os.Lstat(path)
	if err != nil {
		return closeWithError(err)
	}
	guardInfo, err := guard.Stat()
	if err != nil {
		return closeWithError(err)
	}
	if pathInfo.Mode()&os.ModeSymlink != 0 || !pathInfo.Mode().IsRegular() || !os.SameFile(pathInfo, guardInfo) {
		return closeWithError(fmt.Errorf("unsafe encyclopedia writer guard %s", path))
	}
	if err := lockEncyclopediaWriterGuard(guard); err != nil {
		return closeWithError(err)
	}
	return guard, nil
}

func releaseEncyclopediaWriterGuard(guard *os.File) {
	if guard == nil {
		return
	}
	_ = unlockEncyclopediaWriterGuard(guard)
	_ = guard.Close()
}

func readEncyclopediaWriterMarker(path string) (encyclopediaWriterMarker, error) {
	data, err := readEncyclopediaRegularFile(path, 64<<10)
	if err != nil {
		return encyclopediaWriterMarker{}, err
	}
	var marker encyclopediaWriterMarker
	if err := json.Unmarshal(data, &marker); err != nil {
		return encyclopediaWriterMarker{}, err
	}
	if marker.Version != encyclopediaTransactionVersion || marker.PID <= 0 || marker.Token == "" {
		return encyclopediaWriterMarker{}, fmt.Errorf("invalid writer marker")
	}
	return marker, nil
}

func randomEncyclopediaToken() (string, error) {
	var token [8]byte
	if _, err := rand.Read(token[:]); err != nil {
		return "", err
	}
	return hex.EncodeToString(token[:]), nil
}

func encyclopediaRecoveryCommand(command, output string) string {
	if strings.TrimSpace(command) != "" {
		return command
	}
	return fmt.Sprintf("rerun staging with --encyclopedia-output %q", output)
}

func syncEncyclopediaDirectory(path string) error {
	directory, err := os.Open(path)
	if err != nil {
		return err
	}
	defer directory.Close()
	return directory.Sync()
}

func encyclopediaDirectoryExists(path string) (bool, error) {
	info, err := os.Lstat(path)
	if os.IsNotExist(err) {
		return false, nil
	}
	if err != nil {
		return false, fmt.Errorf("inspect encyclopedia output: %w", err)
	}
	if info.Mode()&os.ModeSymlink != 0 {
		return false, fmt.Errorf("unsafe symlink at encyclopedia output %s", path)
	}
	if !info.IsDir() {
		return false, fmt.Errorf("encyclopedia output is not a directory: %s", path)
	}
	return true, nil
}

func recoverEncyclopediaPublication(paths encyclopediaPublicationPaths, inspect encyclopediaOwnedInspector, recoveryCommand string, ops encyclopediaPublicationOps) (bool, error) {
	transaction, exists, err := readEncyclopediaTransaction(paths)
	if err != nil {
		return false, fmt.Errorf("interrupted encyclopedia transaction cannot be recovered: %w; recover with: %s", err, encyclopediaRecoveryCommand(recoveryCommand, paths.output))
	}
	if !exists {
		return false, nil
	}
	candidate := filepath.Join(paths.parent, transaction.CandidateBase)
	backup := filepath.Join(paths.parent, transaction.BackupBase)
	outputState := inspectEncyclopediaDirectoryState(paths.output, inspect)
	candidateState := inspectEncyclopediaDirectoryState(candidate, inspect)
	backupState := inspectEncyclopediaDirectoryState(backup, inspect)
	for name, state := range map[string]encyclopediaDirectoryState{"output": outputState, "candidate": candidateState, "backup": backupState} {
		if state.err != nil {
			return false, fmt.Errorf("interrupted encyclopedia transaction has invalid %s: %w; recover with: %s", name, state.err, encyclopediaRecoveryCommand(recoveryCommand, paths.output))
		}
	}

	removeValidated := func(path string, state encyclopediaDirectoryState) error {
		if !state.exists || state.err != nil {
			return fmt.Errorf("refuse cleanup of unvalidated directory %s", path)
		}
		return ops.removeAll(path)
	}
	finish := func() (bool, error) {
		if err := ops.remove(paths.journal); err != nil && !os.IsNotExist(err) {
			return false, err
		}
		return true, syncEncyclopediaDirectory(paths.parent)
	}
	restoreBackup := func() (bool, error) {
		if outputState.exists {
			return false, fmt.Errorf("ambiguous recovery: output and backup both exist")
		}
		if !backupState.exists {
			return false, fmt.Errorf("ambiguous recovery: no validated prior backup")
		}
		if err := ops.rename(backup, paths.output); err != nil {
			return false, err
		}
		if candidateState.exists {
			if err := removeValidated(candidate, candidateState); err != nil {
				return false, err
			}
		}
		return finish()
	}
	rollbackPublished := func() (bool, error) {
		if !outputState.exists || !backupState.exists || candidateState.exists {
			return false, fmt.Errorf("ambiguous recovery: cannot identify published candidate and prior backup")
		}
		if err := ops.rename(paths.output, candidate); err != nil {
			return false, err
		}
		if err := ops.rename(backup, paths.output); err != nil {
			_ = ops.rename(candidate, paths.output)
			return false, err
		}
		candidateState.exists = true
		if err := removeValidated(candidate, candidateState); err != nil {
			return false, err
		}
		return finish()
	}

	switch transaction.Phase {
	case encyclopediaPhaseCandidateValidated:
		if transaction.HadDestination {
			switch {
			case outputState.exists && !candidateState.exists && !backupState.exists:
				return finish()
			case outputState.exists && candidateState.exists && !backupState.exists:
				if err := removeValidated(candidate, candidateState); err != nil {
					return false, err
				}
				return finish()
			case !outputState.exists && backupState.exists:
				return restoreBackup()
			case outputState.exists && backupState.exists && !candidateState.exists:
				return rollbackPublished()
			default:
				return false, fmt.Errorf("ambiguous recovery at %s; recover with: %s", transaction.Phase, encyclopediaRecoveryCommand(recoveryCommand, paths.output))
			}
		}
		if !outputState.exists && candidateState.exists && !backupState.exists {
			if err := ops.rename(candidate, paths.output); err != nil {
				return false, err
			}
			return finish()
		}
		if outputState.exists && !candidateState.exists && !backupState.exists {
			return finish()
		}
	case encyclopediaPhaseDestinationBackedUp:
		if transaction.HadDestination && backupState.exists {
			if !outputState.exists {
				return restoreBackup()
			}
			if outputState.exists && !candidateState.exists {
				return rollbackPublished()
			}
		}
		if transaction.HadDestination && outputState.exists && !candidateState.exists && !backupState.exists {
			return finish()
		}
		if transaction.HadDestination && outputState.exists && candidateState.exists && !backupState.exists {
			if err := removeValidated(candidate, candidateState); err != nil {
				return false, err
			}
			return finish()
		}
		if !transaction.HadDestination && !backupState.exists {
			switch {
			case outputState.exists && !candidateState.exists:
				return finish()
			case !outputState.exists && candidateState.exists:
				if err := ops.rename(candidate, paths.output); err != nil {
					return false, err
				}
				return finish()
			case !outputState.exists && !candidateState.exists:
				return finish()
			}
		}
	case encyclopediaPhaseCandidatePublished, encyclopediaPhaseCleanupPending:
		if !transaction.HadDestination && backupState.exists {
			return false, fmt.Errorf("ambiguous recovery: initial publication unexpectedly has a prior backup")
		}
		if transaction.HadDestination && outputState.exists && candidateState.exists && !backupState.exists {
			if err := removeValidated(candidate, candidateState); err != nil {
				return false, err
			}
			return finish()
		}
		if outputState.exists && !candidateState.exists {
			if backupState.exists {
				if err := removeValidated(backup, backupState); err != nil {
					return false, err
				}
			}
			return finish()
		}
		if !outputState.exists && transaction.HadDestination && backupState.exists {
			return restoreBackup()
		}
		if !outputState.exists && !transaction.HadDestination && candidateState.exists && !backupState.exists {
			if err := ops.rename(candidate, paths.output); err != nil {
				return false, err
			}
			return finish()
		}
	default:
		return false, fmt.Errorf("unknown encyclopedia transaction phase %q", transaction.Phase)
	}
	return false, fmt.Errorf("ambiguous recovery at %s; recover with: %s", transaction.Phase, encyclopediaRecoveryCommand(recoveryCommand, paths.output))
}

type encyclopediaDirectoryState struct {
	exists    bool
	inventory encyclopediaOwnedInventory
	err       error
}

func inspectEncyclopediaDirectoryState(path string, inspect encyclopediaOwnedInspector) encyclopediaDirectoryState {
	exists, err := encyclopediaDirectoryExists(path)
	if err != nil {
		return encyclopediaDirectoryState{exists: true, err: err}
	}
	if !exists {
		return encyclopediaDirectoryState{}
	}
	inventory, err := inspectEncyclopediaOwnedDirectory(path, inspect)
	return encyclopediaDirectoryState{exists: true, inventory: inventory, err: err}
}

func readEncyclopediaTransaction(paths encyclopediaPublicationPaths) (encyclopediaTransaction, bool, error) {
	data, err := readEncyclopediaRegularFile(paths.journal, 64<<10)
	if os.IsNotExist(err) {
		return encyclopediaTransaction{}, false, nil
	}
	if err != nil {
		return encyclopediaTransaction{}, false, err
	}
	var transaction encyclopediaTransaction
	if err := json.Unmarshal(data, &transaction); err != nil {
		return encyclopediaTransaction{}, false, err
	}
	if transaction.Version != encyclopediaTransactionVersion || transaction.OutputBase != paths.base {
		return encyclopediaTransaction{}, false, fmt.Errorf("transaction does not match output")
	}
	if !validEncyclopediaSidecarBase(transaction.CandidateBase, paths.base, encyclopediaCandidatePrefix) ||
		!validEncyclopediaSidecarBase(transaction.BackupBase, paths.base, encyclopediaBackupPrefix) {
		return encyclopediaTransaction{}, false, fmt.Errorf("transaction paths escape the output parent")
	}
	return transaction, true, nil
}

func validEncyclopediaSidecarBase(name, outputBase, prefix string) bool {
	if filepath.Base(name) != name || name == "." || strings.ContainsAny(name, `/\\`) {
		return false
	}
	return strings.HasPrefix(name, "."+outputBase+"-"+prefix) && len(name) > len("."+outputBase+"-"+prefix)
}
