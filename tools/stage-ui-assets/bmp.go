package main

import (
	"encoding/binary"
	"fmt"
)

// Windows WORD and DWORD fields occupy 2 and 4 bytes in the serialized headers.
const (
	bmpWordSize  = 2
	bmpDwordSize = 4
)

// BITMAPFILEHEADER offsets are relative to the start of the BMP file.
// https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapfileheader
const (
	bmpFileHeaderSize  = 14
	bmpSignatureOffset = 0  // bfType (WORD)
	bmpFileSizeOffset  = 2  // bfSize (DWORD)
	bmpOffBitsOffset   = 10 // bfOffBits (DWORD)
)

// BITMAPINFOHEADER offsets are relative to the start of the DIB, which follows
// BITMAPFILEHEADER in a BMP file. Supported larger DIB headers share these fields.
// https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfoheader
const (
	dibInfoHeaderSize   = 40
	dibHeaderSizeOffset = 0  // biSize (DWORD)
	dibBitCountOffset   = 14 // biBitCount (WORD)
	dibColorsUsedOffset = 32 // biClrUsed (DWORD)
)

// Each palette entry is an RGBQUAD: blue, green, red, and a reserved byte.
// https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-rgbquad
const bmpPaletteEntrySize = 4

func dibToBMP(dib []byte) ([]byte, error) {
	if len(dib) < dibInfoHeaderSize {
		return nil, fmt.Errorf("DIB is too short: got %d bytes", len(dib))
	}

	headerSize := binary.LittleEndian.Uint32(dib[dibHeaderSizeOffset : dibHeaderSizeOffset+bmpDwordSize])
	if headerSize < dibInfoHeaderSize || uint64(headerSize) > uint64(len(dib)) {
		return nil, fmt.Errorf("unsupported DIB header size %d", headerSize)
	}

	bitCount := binary.LittleEndian.Uint16(dib[dibBitCountOffset : dibBitCountOffset+bmpWordSize])
	colorsUsed := binary.LittleEndian.Uint32(dib[dibColorsUsedOffset : dibColorsUsedOffset+bmpDwordSize])
	if colorsUsed == 0 && bitCount <= 8 {
		colorsUsed = uint32(1) << bitCount
	}

	pixelOffset := uint64(bmpFileHeaderSize) + uint64(headerSize) + uint64(colorsUsed)*bmpPaletteEntrySize
	fileSize := uint64(bmpFileHeaderSize) + uint64(len(dib))
	if pixelOffset > fileSize || fileSize > uint64(^uint32(0)) {
		return nil, fmt.Errorf("invalid DIB dimensions: pixel offset %d, file size %d", pixelOffset, fileSize)
	}

	bmp := make([]byte, fileSize)
	copy(bmp[bmpSignatureOffset:bmpSignatureOffset+bmpWordSize], "BM")
	binary.LittleEndian.PutUint32(bmp[bmpFileSizeOffset:bmpFileSizeOffset+bmpDwordSize], uint32(fileSize))
	binary.LittleEndian.PutUint32(bmp[bmpOffBitsOffset:bmpOffBitsOffset+bmpDwordSize], uint32(pixelOffset))
	copy(bmp[bmpFileHeaderSize:], dib)
	return bmp, nil
}

func validateBMP(bmp []byte) error {
	if len(bmp) < bmpFileHeaderSize+dibHeaderSizeOffset+bmpDwordSize {
		return fmt.Errorf("BMP is too short: got %d bytes", len(bmp))
	}
	signature := bmp[bmpSignatureOffset : bmpSignatureOffset+bmpWordSize]
	if string(signature) != "BM" {
		return fmt.Errorf("invalid BMP signature %q", signature)
	}
	if declared := binary.LittleEndian.Uint32(bmp[bmpFileSizeOffset : bmpFileSizeOffset+bmpDwordSize]); uint64(declared) != uint64(len(bmp)) {
		return fmt.Errorf("declared file size %d does not match actual size %d", declared, len(bmp))
	}
	dib := bmp[bmpFileHeaderSize:]
	dibHeaderSize := binary.LittleEndian.Uint32(dib[dibHeaderSizeOffset : dibHeaderSizeOffset+bmpDwordSize])
	if dibHeaderSize < dibInfoHeaderSize || uint64(bmpFileHeaderSize)+uint64(dibHeaderSize) > uint64(len(bmp)) {
		return fmt.Errorf("unsupported DIB header size %d", dibHeaderSize)
	}
	pixelOffset := binary.LittleEndian.Uint32(bmp[bmpOffBitsOffset : bmpOffBitsOffset+bmpDwordSize])
	minimumOffset := uint64(bmpFileHeaderSize) + uint64(dibHeaderSize)
	if uint64(pixelOffset) < minimumOffset || uint64(pixelOffset) > uint64(len(bmp)) {
		return fmt.Errorf("invalid pixel offset %d for DIB header ending at %d", pixelOffset, minimumOffset)
	}
	return nil
}
