package main

import (
	"bytes"
	"encoding/binary"
	"errors"
	"math"
	"os"
	"path/filepath"
	"reflect"
	"testing"
)

type testEncyclopediaBMPOptions struct {
	width       int32
	height      int32
	bitCount    uint16
	compression uint32
	colorsUsed  uint32
	imageSize   *uint32
	trailing    int
}

func TestEncyclopediaBMPValidationHandlesPaletteStrideAndOrientation(t *testing.T) {
	tests := []struct {
		name            string
		bitCount        uint16
		height          int32
		wantStride      uint64
		wantPalette     uint32
		wantOrientation encyclopediaImageOrientation
	}{
		{name: "one bit bottom up", bitCount: 1, height: 2, wantStride: 4, wantPalette: 2, wantOrientation: encyclopediaOrientationBottomUp},
		{name: "four bit top down", bitCount: 4, height: -2, wantStride: 4, wantPalette: 16, wantOrientation: encyclopediaOrientationTopDown},
		{name: "eight bit bottom up", bitCount: 8, height: 2, wantStride: 4, wantPalette: 256, wantOrientation: encyclopediaOrientationBottomUp},
		{name: "sixteen bit top down", bitCount: 16, height: -2, wantStride: 8, wantOrientation: encyclopediaOrientationTopDown},
		{name: "twenty four bit bottom up", bitCount: 24, height: 2, wantStride: 12, wantOrientation: encyclopediaOrientationBottomUp},
		{name: "thirty two bit top down", bitCount: 32, height: -2, wantStride: 12, wantOrientation: encyclopediaOrientationTopDown},
	}
	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			bmp := buildTestEncyclopediaBMP(t, testEncyclopediaBMPOptions{
				width: 3, height: test.height, bitCount: test.bitCount, trailing: 2,
			})
			facts, err := inspectEncyclopediaBMP(bmp, defaultEncyclopediaImageLimits())
			if err != nil {
				t.Fatalf("inspectEncyclopediaBMP() error = %v", err)
			}
			if facts.Format != "bmp" || facts.Width != 3 || facts.Height != 2 || facts.BitCount != test.bitCount {
				t.Fatalf("image facts = %+v", facts)
			}
			if facts.Compression != encyclopediaCompressionRGB || facts.Orientation != test.wantOrientation {
				t.Fatalf("compression/orientation = %q/%q, want %q/%q", facts.Compression, facts.Orientation, encyclopediaCompressionRGB, test.wantOrientation)
			}
			if facts.RowStride != test.wantStride || facts.PixelBytes != test.wantStride*2 || facts.PaletteEntries != test.wantPalette {
				t.Fatalf("stride/pixels/palette = %d/%d/%d, want %d/%d/%d", facts.RowStride, facts.PixelBytes, facts.PaletteEntries, test.wantStride, test.wantStride*2, test.wantPalette)
			}
			if facts.TrailingBytes != 2 || facts.PixelCount != 6 || facts.DecodedRGBABytes != 24 {
				t.Fatalf("trailing/pixels/decoded = %d/%d/%d", facts.TrailingBytes, facts.PixelCount, facts.DecodedRGBABytes)
			}
		})
	}
}

func TestEncyclopediaRejectsHeaderValidButCorruptBitmaps(t *testing.T) {
	tests := []struct {
		name   string
		mutate func([]byte) []byte
		code   encyclopediaImageDiagnosticCode
	}{
		{
			name: "truncated pixel plane",
			mutate: func(bmp []byte) []byte {
				bmp = bmp[:len(bmp)-1]
				binary.LittleEndian.PutUint32(bmp[2:6], uint32(len(bmp)))
				return bmp
			},
			code: encyclopediaImageInvalid,
		},
		{
			name: "palette overlaps pixels",
			mutate: func(bmp []byte) []byte {
				binary.LittleEndian.PutUint32(bmp[10:14], binary.LittleEndian.Uint32(bmp[10:14])-1)
				return bmp
			},
			code: encyclopediaImageInvalid,
		},
		{
			name: "palette index is out of range",
			mutate: func(bmp []byte) []byte {
				bmp[binary.LittleEndian.Uint32(bmp[10:14])] = 2
				return bmp
			},
			code: encyclopediaImageInvalid,
		},
		{
			name: "declared uncompressed image size is wrong",
			mutate: func(bmp []byte) []byte {
				binary.LittleEndian.PutUint32(bmp[34:38], 7)
				return bmp
			},
			code: encyclopediaImageInvalid,
		},
		{
			name: "zero height",
			mutate: func(bmp []byte) []byte {
				binary.LittleEndian.PutUint32(bmp[22:26], 0)
				return bmp
			},
			code: encyclopediaImageInvalid,
		},
	}
	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			colorsUsed := uint32(0)
			if test.name == "palette index is out of range" {
				colorsUsed = 2
			}
			bmp := buildTestEncyclopediaBMP(t, testEncyclopediaBMPOptions{
				width: 2, height: 2, bitCount: 8, colorsUsed: colorsUsed,
			})
			facts, err := inspectEncyclopediaBMP(test.mutate(bmp), defaultEncyclopediaImageLimits())
			if err == nil {
				t.Fatalf("inspectEncyclopediaBMP() facts = %+v, error = nil", facts)
			}
			requireEncyclopediaImageErrorCode(t, err, test.code)
			wantHeight := uint32(2)
			if test.name == "zero height" {
				wantHeight = 0
			}
			if facts.Format != "bmp" || facts.Width != 2 || facts.Height != wantHeight {
				t.Fatalf("rejected image lost parsed facts: %+v", facts)
			}
		})
	}
}

func TestEncyclopediaRejectsCompressionOutsideTheObservedSourceFormat(t *testing.T) {
	for _, compression := range []uint32{1, 2, 3, 4, 5} {
		t.Run(string(encyclopediaBMPCompressionName(compression)), func(t *testing.T) {
			bmp := buildTestEncyclopediaBMP(t, testEncyclopediaBMPOptions{
				width: 2, height: 2, bitCount: 8, compression: compression,
			})
			facts, err := inspectEncyclopediaBMP(bmp, defaultEncyclopediaImageLimits())
			if err == nil {
				t.Fatalf("compression %d accepted: %+v", compression, facts)
			}
			requireEncyclopediaImageErrorCode(t, err, encyclopediaImageUnsupported)
		})
	}
}

func TestEncyclopediaAllowsZeroUncompressedSizeAndTrailingBytes(t *testing.T) {
	zero := uint32(0)
	bmp := buildTestEncyclopediaBMP(t, testEncyclopediaBMPOptions{
		width: 3, height: 2, bitCount: 8, imageSize: &zero, trailing: 2,
	})
	facts, err := inspectEncyclopediaBMP(bmp, defaultEncyclopediaImageLimits())
	if err != nil {
		t.Fatalf("source-shaped BI_RGB image rejected: %v", err)
	}
	if facts.PixelBytes != 8 || facts.TrailingBytes != 2 {
		t.Fatalf("pixel plane/trailing bytes = %d/%d, want 8/2", facts.PixelBytes, facts.TrailingBytes)
	}
}

func TestEncyclopediaImageLimitsAcceptBoundaryAndRejectAbove(t *testing.T) {
	limits := defaultEncyclopediaImageLimits()
	base := buildTestEncyclopediaBMP(t, testEncyclopediaBMPOptions{width: 1, height: 1, bitCount: 24})
	exactBytes := append(base, make([]byte, int(limits.MaxImageBytes)-len(base))...)
	binary.LittleEndian.PutUint32(exactBytes[2:6], uint32(len(exactBytes)))
	if _, err := inspectEncyclopediaBMP(exactBytes, limits); err != nil {
		t.Fatalf("exact %d-byte limit rejected: %v", limits.MaxImageBytes, err)
	}
	aboveBytes := append(exactBytes, 0)
	binary.LittleEndian.PutUint32(aboveBytes[2:6], uint32(len(aboveBytes)))
	if _, err := inspectEncyclopediaBMP(aboveBytes, limits); err == nil {
		t.Fatal("image above byte limit accepted")
	} else {
		requireEncyclopediaImageErrorCode(t, err, encyclopediaImageResourceLimit)
	}

	exactPixels := buildTestEncyclopediaBMP(t, testEncyclopediaBMPOptions{width: 4000, height: 4000, bitCount: 8})
	if _, err := inspectEncyclopediaBMP(exactPixels, limits); err != nil {
		t.Fatalf("exact %d-pixel limit rejected: %v", limits.MaxPixels, err)
	}
	binary.LittleEndian.PutUint32(exactPixels[18:22], 4001)
	if _, err := inspectEncyclopediaBMP(exactPixels, limits); err == nil {
		t.Fatal("image above pixel limit accepted")
	} else {
		requireEncyclopediaImageErrorCode(t, err, encyclopediaImageResourceLimit)
	}
}

func TestEncyclopediaImageArithmeticRejectsOverflow(t *testing.T) {
	if _, err := checkedEncyclopediaImageMultiply(math.MaxUint64, 2, "synthetic decoded bytes"); err == nil {
		t.Fatal("checkedEncyclopediaImageMultiply() accepted overflow")
	} else {
		requireEncyclopediaImageErrorCode(t, err, encyclopediaImageResourceLimit)
	}
	if _, err := addEncyclopediaAggregateBytes(math.MaxUint64, 1, math.MaxUint64); err == nil {
		t.Fatal("addEncyclopediaAggregateBytes() accepted overflow")
	} else {
		requireEncyclopediaImageErrorCode(t, err, encyclopediaImageResourceLimit)
	}
}

func TestEncyclopediaAggregateImageLimitAcceptsBoundaryAndRejectsAbove(t *testing.T) {
	limit := defaultEncyclopediaImageLimits().MaxAggregateImageBytes
	if got, err := addEncyclopediaAggregateBytes(96<<20, 32<<20, limit); err != nil || got != limit {
		t.Fatalf("exact aggregate limit = %d, %v; want %d", got, err, limit)
	}
	if _, err := addEncyclopediaAggregateBytes(limit, 1, limit); err == nil {
		t.Fatal("aggregate byte count above configured limit accepted")
	} else {
		requireEncyclopediaImageErrorCode(t, err, encyclopediaImageResourceLimit)
	}
}

func TestEncyclopediaImageInventoryRecordsSuppliedFilesWithoutFillingGaps(t *testing.T) {
	root := t.TempDir()
	first := buildTestEncyclopediaBMP(t, testEncyclopediaBMPOptions{width: 3, height: 2, bitCount: 24})
	third := buildTestEncyclopediaBMP(t, testEncyclopediaBMPOptions{width: 2, height: -3, bitCount: 4, trailing: 2})
	corrupt := buildTestEncyclopediaBMP(t, testEncyclopediaBMPOptions{width: 2, height: 2, bitCount: 8})
	corrupt = corrupt[:len(corrupt)-1]
	binary.LittleEndian.PutUint32(corrupt[2:6], uint32(len(corrupt)))
	writeTestFile(t, filepath.Join(root, "EDATA.001"), first)
	writeTestFile(t, filepath.Join(root, "EDATA.003"), third)
	writeTestFile(t, filepath.Join(root, "EDATA.005"), corrupt)
	writeTestFile(t, filepath.Join(root, "README.txt"), []byte("not an image"))

	inventory, err := inventoryEncyclopediaImages(root, defaultEncyclopediaImageLimits())
	if err != nil {
		t.Fatalf("inventoryEncyclopediaImages() error = %v", err)
	}
	if got, want := len(inventory.Images), 3; got != want {
		t.Fatalf("image record count = %d, want %d", got, want)
	}
	for index, wantNumber := range []uint32{1, 3, 5} {
		if inventory.Images[index].Number != wantNumber {
			t.Fatalf("record %d number = %d, want %d", index, inventory.Images[index].Number, wantNumber)
		}
		if inventory.Images[index].RawLength == 0 || len(inventory.Images[index].RawSHA256) != 64 {
			t.Fatalf("record %d lacks raw facts: %+v", index, inventory.Images[index])
		}
	}
	if inventory.Images[0].Status != encyclopediaImageValid || inventory.Images[1].Status != encyclopediaImageValid {
		t.Fatalf("valid records = %+v", inventory.Images[:2])
	}
	if inventory.Images[2].Status != encyclopediaImageRejected || inventory.Images[2].DiagnosticCode != encyclopediaImageInvalid {
		t.Fatalf("corrupt record = %+v", inventory.Images[2])
	}
	if got := inventory.Images[2].Facts; got.Format != "bmp" || got.Width != 2 || got.Height != 2 {
		t.Fatalf("corrupt record lost parsed dimensions: %+v", got)
	}
	wantAggregate := uint64(len(first) + len(third) + len(corrupt))
	measurements := inventory.Measurements
	if measurements.SuppliedCount != 3 || measurements.ValidCount != 2 || measurements.RejectedCount != 1 {
		t.Fatalf("inventory counts = %+v", measurements)
	}
	if measurements.AggregateBytes != wantAggregate || measurements.ValidAggregateBytes != uint64(len(first)+len(third)) {
		t.Fatalf("aggregate bytes = %d/%d, want %d/%d", measurements.AggregateBytes, measurements.ValidAggregateBytes, wantAggregate, len(first)+len(third))
	}
	if measurements.MaxPixels != 6 || measurements.MaxWidth != 3 || measurements.MaxHeight != 3 || measurements.MaxDecodedRGBABytes != 24 {
		t.Fatalf("aggregate maxima = %+v", measurements)
	}
	encoded, err := marshalEncyclopediaImageInventory(inventory)
	if err != nil {
		t.Fatal(err)
	}
	if bytes.Contains(encoded, []byte(root)) || bytes.Contains(encoded, []byte("EDATA.002")) || bytes.Contains(encoded, []byte("README.txt")) {
		t.Fatalf("deterministic inventory leaked a root or invented/non-EData file: %s", encoded)
	}
}

func TestEncyclopediaImageInventoryRejectsCaseCollisionsAndSymlinks(t *testing.T) {
	root := t.TempDir()
	bmp := buildTestEncyclopediaBMP(t, testEncyclopediaBMPOptions{width: 1, height: 1, bitCount: 8})
	writeTestFile(t, filepath.Join(root, "EDATA.001"), bmp)
	writeTestFile(t, filepath.Join(root, "edata.001"), bmp)
	outside := filepath.Join(t.TempDir(), "outside.bmp")
	writeTestFile(t, outside, bmp)
	if err := os.Symlink(outside, filepath.Join(root, "EDATA.002")); err != nil {
		t.Fatal(err)
	}

	inventory, err := inventoryEncyclopediaImages(root, defaultEncyclopediaImageLimits())
	if err != nil {
		t.Fatal(err)
	}
	if len(inventory.Images) != 3 || inventory.Measurements.RejectedCount != 3 {
		t.Fatalf("collision/symlink inventory = %+v", inventory)
	}
	for _, record := range inventory.Images {
		if record.Status != encyclopediaImageRejected {
			t.Fatalf("unsafe record accepted: %+v", record)
		}
		switch record.Number {
		case 1:
			if record.DiagnosticCode != encyclopediaImageIdentityCollision {
				t.Fatalf("case collision diagnostic = %+v", record)
			}
		case 2:
			if record.DiagnosticCode != encyclopediaImageUnsafePath || record.RawSHA256 != "" {
				t.Fatalf("symlink diagnostic = %+v", record)
			}
		default:
			t.Fatalf("unexpected record = %+v", record)
		}
	}
	got, err := os.ReadFile(outside)
	if err != nil {
		t.Fatal(err)
	}
	if !bytes.Equal(got, bmp) {
		t.Fatal("inventory changed the symlink target")
	}
}

func TestEncyclopediaImageInventoryEnforcesByteBudgets(t *testing.T) {
	root := t.TempDir()
	first := buildTestEncyclopediaBMP(t, testEncyclopediaBMPOptions{width: 1, height: 1, bitCount: 8})
	second := buildTestEncyclopediaBMP(t, testEncyclopediaBMPOptions{width: 1, height: 1, bitCount: 24})
	writeTestFile(t, filepath.Join(root, "EDATA.001"), first)
	writeTestFile(t, filepath.Join(root, "EDATA.002"), second)

	limits := defaultEncyclopediaImageLimits()
	limits.MaxAggregateImageBytes = uint64(len(first) + len(second))
	if _, err := inventoryEncyclopediaImages(root, limits); err != nil {
		t.Fatalf("exact aggregate limit rejected: %v", err)
	}
	limits.MaxAggregateImageBytes--
	if _, err := inventoryEncyclopediaImages(root, limits); err == nil {
		t.Fatal("aggregate above limit accepted")
	} else {
		requireEncyclopediaImageErrorCode(t, err, encyclopediaImageResourceLimit)
	}

	limits = defaultEncyclopediaImageLimits()
	limits.MaxImageBytes = uint64(len(first) - 1)
	inventory, err := inventoryEncyclopediaImages(root, limits)
	if err != nil {
		t.Fatal(err)
	}
	record := inventory.Images[0]
	if record.Status != encyclopediaImageRejected || record.DiagnosticCode != encyclopediaImageResourceLimit || record.RawLength != uint64(len(first)) || record.RawSHA256 != byteSHA256(first) {
		t.Fatalf("over-limit image facts = %+v", record)
	}
}

func TestOwnedEncyclopediaImageInventoryMeasuresSuppliedCorpus(t *testing.T) {
	sourceRoot := os.Getenv("REBELLION_ENCYCLOPEDIA_TEST_SOURCE")
	if sourceRoot == "" {
		t.Skip("set REBELLION_ENCYCLOPEDIA_TEST_SOURCE to an owned installation root")
	}
	edataRoot := filepath.Join(sourceRoot, "EData")
	first, err := inventoryEncyclopediaImages(edataRoot, defaultEncyclopediaImageLimits())
	if err != nil {
		t.Fatal(err)
	}
	second, err := inventoryEncyclopediaImages(edataRoot, defaultEncyclopediaImageLimits())
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(first, second) {
		t.Fatal("owned EData inventory changed across two read-only measurements")
	}
	if first.Measurements.SuppliedCount == 0 || first.Measurements.ValidCount != first.Measurements.SuppliedCount || first.Measurements.RejectedCount != 0 {
		t.Fatalf("owned EData contains rejected or missing images: %+v", first.Measurements)
	}
	for _, record := range first.Images {
		if record.Status != encyclopediaImageValid || record.RawSHA256 == "" || record.RawLength == 0 {
			t.Fatalf("incomplete owned image record: %+v", record)
		}
	}
	if encytextName, ok := findCaseInsensitiveFile(t, sourceRoot, "ENCYTEXT.DLL"); ok {
		encytext, err := readBoundedEncyclopediaSource(filepath.Join(sourceRoot, encytextName), defaultEncyclopediaInventoryLimits().MaxSourceBytes)
		if err != nil {
			t.Fatal(err)
		}
		const observedEnglishENCYTEXTHash = "49aea545a5e09e5fe9115a22bc785690f103d2f931e08bd4a53a617a42636d8c"
		if byteSHA256(encytext) == observedEnglishENCYTEXTHash {
			wantMeasurements := encyclopediaImageMeasurements{
				SuppliedCount: 187, ValidCount: 187,
				AggregateBytes: 15_161_638, ValidAggregateBytes: 15_161_638,
				MaxImageBytes: 81_080, MaxImageBytesFilename: "EDATA.001",
				MaxPixels: 80_000, MaxPixelsFilename: "EDATA.001",
				MaxDecodedRGBABytes: 320_000, MaxDecodedBytesFilename: "EDATA.001",
				MaxWidth: 400, MaxHeight: 200,
			}
			if !reflect.DeepEqual(first.Measurements, wantMeasurements) {
				t.Fatalf("identified English image measurements = %+v, want %+v", first.Measurements, wantMeasurements)
			}
			trailingTwo := 0
			numbers := make(map[uint32]struct{}, len(first.Images))
			for _, record := range first.Images {
				numbers[record.Number] = struct{}{}
				if record.Facts.Width != 400 || record.Facts.Height != 200 || record.Facts.BitCount != 8 || record.Facts.Compression != encyclopediaCompressionRGB || record.Facts.Orientation != encyclopediaOrientationBottomUp {
					t.Fatalf("identified English image has unexpected facts: %+v", record)
				}
				if record.Facts.TrailingBytes == 2 {
					trailingTwo++
				} else if record.Facts.TrailingBytes != 0 {
					t.Fatalf("identified English image has unexpected trailing bytes: %+v", record)
				}
			}
			if trailingTwo != 26 {
				t.Fatalf("identified English images with two trailing bytes = %d, want 26", trailingTwo)
			}
			missing := make([]uint32, 0)
			for number := uint32(1); number <= 192; number++ {
				if _, exists := numbers[number]; !exists {
					missing = append(missing, number)
				}
			}
			if want := []uint32{144, 145, 147, 159, 165}; !reflect.DeepEqual(missing, want) {
				t.Fatalf("identified English EData gaps = %v, want %v", missing, want)
			}
		}
	}

	encoded, err := marshalEncyclopediaImageInventory(first)
	if err != nil {
		t.Fatal(err)
	}
	if bytes.Contains(encoded, []byte(sourceRoot)) {
		t.Fatal("owned image inventory contains the installation path")
	}
	repoRoot, err := filepath.Abs(filepath.Join("..", ".."))
	if err != nil {
		t.Fatal(err)
	}
	evidenceDir := filepath.Join(repoRoot, ".artifacts", "encyclopedia")
	if err := os.MkdirAll(evidenceDir, 0o700); err != nil {
		t.Fatal(err)
	}
	evidencePath := filepath.Join(evidenceDir, "E41-owned-images.json")
	writeTestFile(t, evidencePath, encoded)
	t.Logf("retained ignored image measurements at %s: %+v", evidencePath, first.Measurements)
}

func buildTestEncyclopediaBMP(t *testing.T, options testEncyclopediaBMPOptions) []byte {
	t.Helper()
	if options.width <= 0 || options.height == 0 || options.height == math.MinInt32 {
		t.Fatalf("invalid test BMP dimensions %dx%d", options.width, options.height)
	}
	height := uint64(options.height)
	if options.height < 0 {
		height = uint64(-options.height)
	}
	bitsPerRow := uint64(options.width) * uint64(options.bitCount)
	rowStride := ((bitsPerRow + 31) / 32) * 4
	pixelBytes := rowStride * height
	paletteEntries := options.colorsUsed
	if paletteEntries == 0 && options.bitCount <= 8 {
		paletteEntries = uint32(1) << options.bitCount
	}
	pixelOffset := uint64(14 + 40 + paletteEntries*4)
	fileSize := pixelOffset + pixelBytes + uint64(options.trailing)
	if fileSize > math.MaxUint32 || fileSize > uint64(math.MaxInt) {
		t.Fatalf("test BMP is too large: %d", fileSize)
	}
	bmp := make([]byte, int(fileSize))
	copy(bmp[0:2], "BM")
	binary.LittleEndian.PutUint32(bmp[2:6], uint32(fileSize))
	binary.LittleEndian.PutUint32(bmp[10:14], uint32(pixelOffset))
	binary.LittleEndian.PutUint32(bmp[14:18], 40)
	binary.LittleEndian.PutUint32(bmp[18:22], uint32(options.width))
	binary.LittleEndian.PutUint32(bmp[22:26], uint32(options.height))
	binary.LittleEndian.PutUint16(bmp[26:28], 1)
	binary.LittleEndian.PutUint16(bmp[28:30], options.bitCount)
	binary.LittleEndian.PutUint32(bmp[30:34], options.compression)
	imageSize := uint32(pixelBytes)
	if options.imageSize != nil {
		imageSize = *options.imageSize
	}
	binary.LittleEndian.PutUint32(bmp[34:38], imageSize)
	binary.LittleEndian.PutUint32(bmp[46:50], options.colorsUsed)
	return bmp
}

func requireEncyclopediaImageErrorCode(t *testing.T, err error, want encyclopediaImageDiagnosticCode) {
	t.Helper()
	var imageErr *encyclopediaImageError
	if !errors.As(err, &imageErr) {
		t.Fatalf("error %T %v is not an encyclopediaImageError", err, err)
	}
	if imageErr.Code != want {
		t.Fatalf("error code = %q, want %q (%v)", imageErr.Code, want, err)
	}
}
