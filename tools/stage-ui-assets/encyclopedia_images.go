package main

import (
	"crypto/sha256"
	"encoding/binary"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"math"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

const (
	encyclopediaMaxImageBytes          = uint64(32 << 20)
	encyclopediaMaxImagePixels         = uint64(16_000_000)
	encyclopediaMaxAggregateImageBytes = uint64(128 << 20)

	dibWidthOffset       = 4
	dibHeightOffset      = 8
	dibPlanesOffset      = 12
	dibCompressionOffset = 16
	dibImageSizeOffset   = 20
)

type encyclopediaImageDiagnosticCode string

const (
	encyclopediaImageInvalid           encyclopediaImageDiagnosticCode = "invalid_image"
	encyclopediaImageUnsupported       encyclopediaImageDiagnosticCode = "unsupported_image"
	encyclopediaImageResourceLimit     encyclopediaImageDiagnosticCode = "resource_limit"
	encyclopediaImageUnsafePath        encyclopediaImageDiagnosticCode = "unsafe_asset_path"
	encyclopediaImageIdentityCollision encyclopediaImageDiagnosticCode = "asset_identity_collision"
)

type encyclopediaImageError struct {
	Code   encyclopediaImageDiagnosticCode
	Detail string
}

func (err *encyclopediaImageError) Error() string {
	return string(err.Code) + ": " + err.Detail
}

type encyclopediaImageOrientation string

const (
	encyclopediaOrientationBottomUp encyclopediaImageOrientation = "bottom_up"
	encyclopediaOrientationTopDown  encyclopediaImageOrientation = "top_down"
)

type encyclopediaBMPCompression string

const (
	encyclopediaCompressionRGB       encyclopediaBMPCompression = "BI_RGB"
	encyclopediaCompressionRLE8      encyclopediaBMPCompression = "BI_RLE8"
	encyclopediaCompressionRLE4      encyclopediaBMPCompression = "BI_RLE4"
	encyclopediaCompressionBitfields encyclopediaBMPCompression = "BI_BITFIELDS"
	encyclopediaCompressionJPEG      encyclopediaBMPCompression = "BI_JPEG"
	encyclopediaCompressionPNG       encyclopediaBMPCompression = "BI_PNG"
)

type encyclopediaImageLimits struct {
	MaxImageBytes          uint64
	MaxPixels              uint64
	MaxAggregateImageBytes uint64
}

type encyclopediaImageStatus string

const (
	encyclopediaImageValid    encyclopediaImageStatus = "valid"
	encyclopediaImageRejected encyclopediaImageStatus = "rejected"
)

type encyclopediaImageFacts struct {
	Format           string                       `json:"format"`
	Width            uint32                       `json:"width"`
	Height           uint32                       `json:"height"`
	BitCount         uint16                       `json:"bit_count"`
	Compression      encyclopediaBMPCompression   `json:"compression"`
	Orientation      encyclopediaImageOrientation `json:"orientation"`
	PixelOffset      uint32                       `json:"pixel_offset"`
	RowStride        uint64                       `json:"row_stride"`
	PixelBytes       uint64                       `json:"pixel_bytes"`
	PixelCount       uint64                       `json:"pixel_count"`
	DecodedRGBABytes uint64                       `json:"decoded_rgba_bytes"`
	PaletteEntries   uint32                       `json:"palette_entries"`
	TrailingBytes    uint64                       `json:"trailing_bytes"`
}

const (
	encyclopediaImageInventoryKind          = "encyclopedia-image-inventory"
	encyclopediaImageInventorySchemaVersion = 1
)

type encyclopediaImageRecord struct {
	Number         uint32                          `json:"number"`
	Filename       string                          `json:"filename"`
	Status         encyclopediaImageStatus         `json:"status"`
	RawLength      uint64                          `json:"raw_length"`
	RawSHA256      string                          `json:"raw_sha256,omitempty"`
	Facts          encyclopediaImageFacts          `json:"image"`
	DiagnosticCode encyclopediaImageDiagnosticCode `json:"diagnostic_code,omitempty"`
	Diagnostic     string                          `json:"diagnostic,omitempty"`
}

type encyclopediaImageMeasurements struct {
	SuppliedCount           int    `json:"supplied_count"`
	ValidCount              int    `json:"valid_count"`
	RejectedCount           int    `json:"rejected_count"`
	AggregateBytes          uint64 `json:"aggregate_bytes"`
	ValidAggregateBytes     uint64 `json:"valid_aggregate_bytes"`
	MaxImageBytes           uint64 `json:"max_image_bytes"`
	MaxImageBytesFilename   string `json:"max_image_bytes_filename,omitempty"`
	MaxPixels               uint64 `json:"max_pixels"`
	MaxPixelsFilename       string `json:"max_pixels_filename,omitempty"`
	MaxDecodedRGBABytes     uint64 `json:"max_decoded_rgba_bytes"`
	MaxDecodedBytesFilename string `json:"max_decoded_bytes_filename,omitempty"`
	MaxWidth                uint32 `json:"max_width"`
	MaxHeight               uint32 `json:"max_height"`
}

type encyclopediaImageInventory struct {
	Kind          string                        `json:"kind"`
	SchemaVersion int                           `json:"schema_version"`
	Measurements  encyclopediaImageMeasurements `json:"measurements"`
	Images        []encyclopediaImageRecord     `json:"images"`
}

type encyclopediaImageCandidate struct {
	Number   uint32
	Filename string
	Path     string
	Info     os.FileInfo
}

func defaultEncyclopediaImageLimits() encyclopediaImageLimits {
	return encyclopediaImageLimits{
		MaxImageBytes:          encyclopediaMaxImageBytes,
		MaxPixels:              encyclopediaMaxImagePixels,
		MaxAggregateImageBytes: encyclopediaMaxAggregateImageBytes,
	}
}

func inventoryEncyclopediaImages(root string, limits encyclopediaImageLimits) (encyclopediaImageInventory, error) {
	if err := validateEncyclopediaImageLimits(limits); err != nil {
		return encyclopediaImageInventory{}, err
	}
	absoluteRoot, err := filepath.Abs(root)
	if err != nil {
		return encyclopediaImageInventory{}, fmt.Errorf("resolve EData root: %w", err)
	}
	rootInfo, err := os.Lstat(absoluteRoot)
	if err != nil {
		return encyclopediaImageInventory{}, fmt.Errorf("inspect EData root: %w", err)
	}
	if rootInfo.Mode()&os.ModeSymlink != 0 || !rootInfo.IsDir() {
		return encyclopediaImageInventory{}, newEncyclopediaImageError(encyclopediaImageUnsafePath, "EData root must be a non-symlink directory")
	}
	entries, err := os.ReadDir(absoluteRoot)
	if err != nil {
		return encyclopediaImageInventory{}, fmt.Errorf("read EData root: %w", err)
	}

	candidates := make([]encyclopediaImageCandidate, 0, len(entries))
	for _, entry := range entries {
		number, ok := parseEncyclopediaImageFilename(entry.Name())
		if !ok {
			continue
		}
		path := filepath.Join(absoluteRoot, entry.Name())
		info, err := os.Lstat(path)
		if err != nil {
			return encyclopediaImageInventory{}, fmt.Errorf("inspect EData image %s: %w", entry.Name(), err)
		}
		candidates = append(candidates, encyclopediaImageCandidate{Number: number, Filename: entry.Name(), Path: path, Info: info})
	}
	sort.Slice(candidates, func(i, j int) bool {
		if candidates[i].Number != candidates[j].Number {
			return candidates[i].Number < candidates[j].Number
		}
		left, right := strings.ToLower(candidates[i].Filename), strings.ToLower(candidates[j].Filename)
		if left != right {
			return left < right
		}
		return candidates[i].Filename < candidates[j].Filename
	})

	inventory := encyclopediaImageInventory{
		Kind:          encyclopediaImageInventoryKind,
		SchemaVersion: encyclopediaImageInventorySchemaVersion,
		Images:        make([]encyclopediaImageRecord, 0, len(candidates)),
	}
	inventory.Measurements.SuppliedCount = len(candidates)
	identityCounts := make(map[uint32]int, len(candidates))
	for _, candidate := range candidates {
		identityCounts[candidate.Number]++
		if candidate.Info.Mode()&os.ModeSymlink != 0 || !candidate.Info.Mode().IsRegular() {
			continue
		}
		if candidate.Info.Size() < 0 {
			return encyclopediaImageInventory{}, newEncyclopediaImageError(encyclopediaImageInvalid, "%s has a negative file length", candidate.Filename)
		}
		inventory.Measurements.AggregateBytes, err = addEncyclopediaAggregateBytes(
			inventory.Measurements.AggregateBytes,
			uint64(candidate.Info.Size()),
			limits.MaxAggregateImageBytes,
		)
		if err != nil {
			return encyclopediaImageInventory{}, fmt.Errorf("measure %s: %w", candidate.Filename, err)
		}
	}

	for _, candidate := range candidates {
		record := encyclopediaImageRecord{
			Number:   candidate.Number,
			Filename: candidate.Filename,
			Status:   encyclopediaImageRejected,
			Facts:    encyclopediaImageFacts{Format: "bmp"},
		}
		if candidate.Info.Mode()&os.ModeSymlink != 0 || !candidate.Info.Mode().IsRegular() {
			record.DiagnosticCode = encyclopediaImageUnsafePath
			record.Diagnostic = "EData image is not a direct regular file"
			inventory.Images = append(inventory.Images, record)
			inventory.Measurements.RejectedCount++
			continue
		}
		record.RawLength = uint64(candidate.Info.Size())
		updateEncyclopediaRawMaxima(&inventory.Measurements, record)
		if record.RawLength > limits.MaxImageBytes {
			record.RawSHA256, err = hashEncyclopediaImageFile(candidate.Path, candidate.Info)
			if err != nil {
				return encyclopediaImageInventory{}, fmt.Errorf("hash EData image %s: %w", candidate.Filename, err)
			}
			record.DiagnosticCode = encyclopediaImageResourceLimit
			record.Diagnostic = fmt.Sprintf("image length %d exceeds the %d-byte limit", record.RawLength, limits.MaxImageBytes)
		} else {
			data, readErr := readEncyclopediaImageFile(candidate.Path, candidate.Info, limits.MaxImageBytes)
			if readErr != nil {
				return encyclopediaImageInventory{}, fmt.Errorf("read EData image %s: %w", candidate.Filename, readErr)
			}
			record.RawSHA256 = byteSHA256(data)
			record.Facts, err = inspectEncyclopediaBMP(data, limits)
			if err == nil {
				record.Status = encyclopediaImageValid
			} else {
				var imageErr *encyclopediaImageError
				if !errors.As(err, &imageErr) {
					return encyclopediaImageInventory{}, fmt.Errorf("inspect EData image %s: %w", candidate.Filename, err)
				}
				record.DiagnosticCode = imageErr.Code
				record.Diagnostic = imageErr.Detail
			}
		}
		if identityCounts[candidate.Number] > 1 {
			record.Status = encyclopediaImageRejected
			record.DiagnosticCode = encyclopediaImageIdentityCollision
			record.Diagnostic = fmt.Sprintf("multiple supplied filenames resolve to EData number %03d", candidate.Number)
		}
		if record.Status == encyclopediaImageValid {
			inventory.Measurements.ValidCount++
			inventory.Measurements.ValidAggregateBytes, err = addEncyclopediaAggregateBytes(
				inventory.Measurements.ValidAggregateBytes,
				record.RawLength,
				limits.MaxAggregateImageBytes,
			)
			if err != nil {
				return encyclopediaImageInventory{}, err
			}
			updateEncyclopediaDecodedMaxima(&inventory.Measurements, record)
		} else {
			inventory.Measurements.RejectedCount++
		}
		inventory.Images = append(inventory.Images, record)
	}
	return inventory, nil
}

func marshalEncyclopediaImageInventory(inventory encyclopediaImageInventory) ([]byte, error) {
	if inventory.Kind != encyclopediaImageInventoryKind {
		return nil, fmt.Errorf("unsupported encyclopedia image inventory kind %q", inventory.Kind)
	}
	if inventory.SchemaVersion != encyclopediaImageInventorySchemaVersion {
		return nil, fmt.Errorf("unsupported encyclopedia image inventory schema version %d", inventory.SchemaVersion)
	}
	if inventory.Measurements.SuppliedCount != len(inventory.Images) || inventory.Measurements.ValidCount+inventory.Measurements.RejectedCount != len(inventory.Images) {
		return nil, fmt.Errorf("encyclopedia image inventory counts do not match records")
	}
	data, err := json.MarshalIndent(inventory, "", "  ")
	if err != nil {
		return nil, fmt.Errorf("marshal encyclopedia image inventory: %w", err)
	}
	return append(data, '\n'), nil
}

func validateEncyclopediaImageLimits(limits encyclopediaImageLimits) error {
	if limits.MaxImageBytes == 0 || limits.MaxImageBytes > math.MaxInt64-1 {
		return newEncyclopediaImageError(encyclopediaImageResourceLimit, "image-byte limit must be between 1 and %d", uint64(math.MaxInt64-1))
	}
	if limits.MaxPixels == 0 || limits.MaxAggregateImageBytes == 0 {
		return newEncyclopediaImageError(encyclopediaImageResourceLimit, "pixel and aggregate image-byte limits must be positive")
	}
	return nil
}

func parseEncyclopediaImageFilename(name string) (uint32, bool) {
	if len(name) != len("EDATA.000") || !strings.EqualFold(name[:6], "EDATA.") {
		return 0, false
	}
	number := uint32(0)
	for _, value := range []byte(name[6:]) {
		if value < '0' || value > '9' {
			return 0, false
		}
		number = number*10 + uint32(value-'0')
	}
	return number, true
}

func readEncyclopediaImageFile(path string, expected os.FileInfo, maxBytes uint64) ([]byte, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	opened, err := file.Stat()
	if err != nil {
		return nil, err
	}
	if !opened.Mode().IsRegular() || !os.SameFile(expected, opened) {
		return nil, newEncyclopediaImageError(encyclopediaImageUnsafePath, "EData image changed identity before reading")
	}
	data, err := io.ReadAll(io.LimitReader(file, int64(maxBytes)+1))
	if err != nil {
		return nil, err
	}
	if uint64(len(data)) > maxBytes {
		return nil, newEncyclopediaImageError(encyclopediaImageResourceLimit, "image grew beyond the %d-byte limit while reading", maxBytes)
	}
	if int64(len(data)) != expected.Size() {
		return nil, fmt.Errorf("image length changed from %d to %d while reading", expected.Size(), len(data))
	}
	return data, nil
}

func hashEncyclopediaImageFile(path string, expected os.FileInfo) (string, error) {
	file, err := os.Open(path)
	if err != nil {
		return "", err
	}
	defer file.Close()
	opened, err := file.Stat()
	if err != nil {
		return "", err
	}
	if !opened.Mode().IsRegular() || !os.SameFile(expected, opened) {
		return "", newEncyclopediaImageError(encyclopediaImageUnsafePath, "EData image changed identity before hashing")
	}
	hash := sha256.New()
	written, err := io.Copy(hash, io.LimitReader(file, expected.Size()))
	if err != nil {
		return "", err
	}
	if written != expected.Size() {
		return "", fmt.Errorf("image length changed from %d to %d while hashing", expected.Size(), written)
	}
	var extra [1]byte
	extraBytes, readErr := file.Read(extra[:])
	if extraBytes != 0 {
		return "", fmt.Errorf("image grew beyond its measured %d-byte length while hashing", expected.Size())
	}
	if readErr != nil && !errors.Is(readErr, io.EOF) {
		return "", readErr
	}
	return hex.EncodeToString(hash.Sum(nil)), nil
}

func updateEncyclopediaRawMaxima(measurements *encyclopediaImageMeasurements, record encyclopediaImageRecord) {
	if record.RawLength > measurements.MaxImageBytes || (record.RawLength == measurements.MaxImageBytes && (measurements.MaxImageBytesFilename == "" || record.Filename < measurements.MaxImageBytesFilename)) {
		measurements.MaxImageBytes = record.RawLength
		measurements.MaxImageBytesFilename = record.Filename
	}
}

func updateEncyclopediaDecodedMaxima(measurements *encyclopediaImageMeasurements, record encyclopediaImageRecord) {
	if record.Facts.PixelCount > measurements.MaxPixels || (record.Facts.PixelCount == measurements.MaxPixels && (measurements.MaxPixelsFilename == "" || record.Filename < measurements.MaxPixelsFilename)) {
		measurements.MaxPixels = record.Facts.PixelCount
		measurements.MaxPixelsFilename = record.Filename
	}
	if record.Facts.DecodedRGBABytes > measurements.MaxDecodedRGBABytes || (record.Facts.DecodedRGBABytes == measurements.MaxDecodedRGBABytes && (measurements.MaxDecodedBytesFilename == "" || record.Filename < measurements.MaxDecodedBytesFilename)) {
		measurements.MaxDecodedRGBABytes = record.Facts.DecodedRGBABytes
		measurements.MaxDecodedBytesFilename = record.Filename
	}
	if record.Facts.Width > measurements.MaxWidth {
		measurements.MaxWidth = record.Facts.Width
	}
	if record.Facts.Height > measurements.MaxHeight {
		measurements.MaxHeight = record.Facts.Height
	}
}

func inspectEncyclopediaBMP(data []byte, limits encyclopediaImageLimits) (encyclopediaImageFacts, error) {
	facts := encyclopediaImageFacts{Format: "bmp"}
	if limits.MaxImageBytes == 0 || limits.MaxPixels == 0 || limits.MaxAggregateImageBytes == 0 {
		return facts, newEncyclopediaImageError(encyclopediaImageResourceLimit, "image limits must be positive")
	}
	if uint64(len(data)) > limits.MaxImageBytes {
		return facts, newEncyclopediaImageError(encyclopediaImageResourceLimit, "image length %d exceeds the %d-byte limit", len(data), limits.MaxImageBytes)
	}
	if len(data) < bmpFileHeaderSize+dibInfoHeaderSize {
		return facts, newEncyclopediaImageError(encyclopediaImageInvalid, "BMP is too short: got %d bytes", len(data))
	}
	if data[bmpSignatureOffset] != 'B' || data[bmpSignatureOffset+1] != 'M' {
		return facts, newEncyclopediaImageError(encyclopediaImageInvalid, "invalid BMP signature")
	}
	if declared := binary.LittleEndian.Uint32(data[bmpFileSizeOffset : bmpFileSizeOffset+bmpDwordSize]); uint64(declared) != uint64(len(data)) {
		return facts, newEncyclopediaImageError(encyclopediaImageInvalid, "declared file size %d does not match actual size %d", declared, len(data))
	}

	dib := data[bmpFileHeaderSize:]
	dibSize := binary.LittleEndian.Uint32(dib[dibHeaderSizeOffset : dibHeaderSizeOffset+bmpDwordSize])
	if dibSize != dibInfoHeaderSize {
		return facts, newEncyclopediaImageError(encyclopediaImageUnsupported, "DIB header size %d is outside the observed BITMAPINFOHEADER format", dibSize)
	}
	width := int32(binary.LittleEndian.Uint32(dib[dibWidthOffset : dibWidthOffset+bmpDwordSize]))
	signedHeight := int32(binary.LittleEndian.Uint32(dib[dibHeightOffset : dibHeightOffset+bmpDwordSize]))
	if width > 0 {
		facts.Width = uint32(width)
	}
	if signedHeight != 0 && signedHeight != math.MinInt32 {
		if signedHeight < 0 {
			facts.Height = uint32(-signedHeight)
			facts.Orientation = encyclopediaOrientationTopDown
		} else {
			facts.Height = uint32(signedHeight)
			facts.Orientation = encyclopediaOrientationBottomUp
		}
	}
	if width <= 0 || signedHeight == 0 || signedHeight == math.MinInt32 {
		return facts, newEncyclopediaImageError(encyclopediaImageInvalid, "invalid BMP dimensions %dx%d", width, signedHeight)
	}
	if planes := binary.LittleEndian.Uint16(dib[dibPlanesOffset : dibPlanesOffset+bmpWordSize]); planes != 1 {
		return facts, newEncyclopediaImageError(encyclopediaImageInvalid, "invalid BMP plane count %d", planes)
	}

	facts.BitCount = binary.LittleEndian.Uint16(dib[dibBitCountOffset : dibBitCountOffset+bmpWordSize])
	switch facts.BitCount {
	case 1, 4, 8, 16, 24, 32:
	default:
		return facts, newEncyclopediaImageError(encyclopediaImageUnsupported, "unsupported BMP bit depth %d", facts.BitCount)
	}
	compressionValue := binary.LittleEndian.Uint32(dib[dibCompressionOffset : dibCompressionOffset+bmpDwordSize])
	facts.Compression = encyclopediaBMPCompressionName(compressionValue)
	if compressionValue != 0 {
		return facts, newEncyclopediaImageError(encyclopediaImageUnsupported, "compression %s is outside the source-observed BI_RGB format", facts.Compression)
	}

	var err error
	facts.PixelCount, err = checkedEncyclopediaImageMultiply(uint64(facts.Width), uint64(facts.Height), "pixel count")
	if err != nil {
		return facts, err
	}
	if facts.PixelCount > limits.MaxPixels {
		return facts, newEncyclopediaImageError(encyclopediaImageResourceLimit, "pixel count %d exceeds the %d-pixel limit", facts.PixelCount, limits.MaxPixels)
	}
	facts.DecodedRGBABytes, err = checkedEncyclopediaImageMultiply(facts.PixelCount, 4, "decoded RGBA bytes")
	if err != nil {
		return facts, err
	}
	bitsPerRow, err := checkedEncyclopediaImageMultiply(uint64(facts.Width), uint64(facts.BitCount), "row bit count")
	if err != nil {
		return facts, err
	}
	alignedBits, err := checkedEncyclopediaImageAdd(bitsPerRow, 31, "aligned row bit count")
	if err != nil {
		return facts, err
	}
	facts.RowStride, err = checkedEncyclopediaImageMultiply(alignedBits/32, 4, "row stride")
	if err != nil {
		return facts, err
	}
	facts.PixelBytes, err = checkedEncyclopediaImageMultiply(facts.RowStride, uint64(facts.Height), "pixel-plane length")
	if err != nil {
		return facts, err
	}

	colorsUsed := binary.LittleEndian.Uint32(dib[dibColorsUsedOffset : dibColorsUsedOffset+bmpDwordSize])
	if facts.BitCount <= 8 {
		maximumEntries := uint32(1) << facts.BitCount
		if colorsUsed == 0 {
			facts.PaletteEntries = maximumEntries
		} else if colorsUsed > maximumEntries {
			return facts, newEncyclopediaImageError(encyclopediaImageInvalid, "palette declares %d entries for %d-bit pixels", colorsUsed, facts.BitCount)
		} else {
			facts.PaletteEntries = colorsUsed
		}
	} else {
		facts.PaletteEntries = colorsUsed
	}
	paletteBytes, err := checkedEncyclopediaImageMultiply(uint64(facts.PaletteEntries), bmpPaletteEntrySize, "palette length")
	if err != nil {
		return facts, err
	}
	minimumPixelOffset, err := checkedEncyclopediaImageAdd(uint64(bmpFileHeaderSize)+uint64(dibSize), paletteBytes, "minimum pixel offset")
	if err != nil {
		return facts, err
	}
	facts.PixelOffset = binary.LittleEndian.Uint32(data[bmpOffBitsOffset : bmpOffBitsOffset+bmpDwordSize])
	if uint64(facts.PixelOffset) < minimumPixelOffset || uint64(facts.PixelOffset) > uint64(len(data)) {
		return facts, newEncyclopediaImageError(encyclopediaImageInvalid, "pixel offset %d does not contain the declared header and palette ending at %d", facts.PixelOffset, minimumPixelOffset)
	}
	imageSize := binary.LittleEndian.Uint32(dib[dibImageSizeOffset : dibImageSizeOffset+bmpDwordSize])
	if imageSize != 0 && uint64(imageSize) != facts.PixelBytes {
		return facts, newEncyclopediaImageError(encyclopediaImageInvalid, "declared pixel-plane length %d does not match required length %d", imageSize, facts.PixelBytes)
	}
	pixelEnd, err := checkedEncyclopediaImageAdd(uint64(facts.PixelOffset), facts.PixelBytes, "pixel-plane end")
	if err != nil {
		return facts, err
	}
	if pixelEnd > uint64(len(data)) {
		return facts, newEncyclopediaImageError(encyclopediaImageInvalid, "pixel plane ending at %d exceeds file length %d", pixelEnd, len(data))
	}
	facts.TrailingBytes = uint64(len(data)) - pixelEnd
	if err := validateEncyclopediaPaletteIndices(data, facts); err != nil {
		return facts, err
	}
	return facts, nil
}

func validateEncyclopediaPaletteIndices(data []byte, facts encyclopediaImageFacts) error {
	if facts.BitCount > 8 {
		return nil
	}
	for row := uint64(0); row < uint64(facts.Height); row++ {
		rowStart := uint64(facts.PixelOffset) + row*facts.RowStride
		for column := uint64(0); column < uint64(facts.Width); column++ {
			var index uint8
			switch facts.BitCount {
			case 8:
				index = data[rowStart+column]
			case 4:
				value := data[rowStart+column/2]
				if column%2 == 0 {
					index = value >> 4
				} else {
					index = value & 0x0f
				}
			case 1:
				value := data[rowStart+column/8]
				index = (value >> (7 - column%8)) & 1
			}
			if uint32(index) >= facts.PaletteEntries {
				return newEncyclopediaImageError(encyclopediaImageInvalid, "palette index %d at row %d column %d exceeds %d entries", index, row, column, facts.PaletteEntries)
			}
		}
	}
	return nil
}

func encyclopediaBMPCompressionName(value uint32) encyclopediaBMPCompression {
	switch value {
	case 0:
		return encyclopediaCompressionRGB
	case 1:
		return encyclopediaCompressionRLE8
	case 2:
		return encyclopediaCompressionRLE4
	case 3:
		return encyclopediaCompressionBitfields
	case 4:
		return encyclopediaCompressionJPEG
	case 5:
		return encyclopediaCompressionPNG
	default:
		return encyclopediaBMPCompression(fmt.Sprintf("UNKNOWN_%d", value))
	}
}

func checkedEncyclopediaImageMultiply(left, right uint64, description string) (uint64, error) {
	if left != 0 && right > math.MaxUint64/left {
		return 0, newEncyclopediaImageError(encyclopediaImageResourceLimit, "%s overflows uint64", description)
	}
	return left * right, nil
}

func checkedEncyclopediaImageAdd(left, right uint64, description string) (uint64, error) {
	if right > math.MaxUint64-left {
		return 0, newEncyclopediaImageError(encyclopediaImageResourceLimit, "%s overflows uint64", description)
	}
	return left + right, nil
}

func addEncyclopediaAggregateBytes(current, next, limit uint64) (uint64, error) {
	total, err := checkedEncyclopediaImageAdd(current, next, "aggregate image bytes")
	if err != nil {
		return 0, err
	}
	if total > limit {
		return 0, newEncyclopediaImageError(encyclopediaImageResourceLimit, "aggregate image bytes %d exceed the %d-byte limit", total, limit)
	}
	return total, nil
}

func newEncyclopediaImageError(code encyclopediaImageDiagnosticCode, format string, arguments ...any) error {
	return &encyclopediaImageError{Code: code, Detail: fmt.Sprintf(format, arguments...)}
}
