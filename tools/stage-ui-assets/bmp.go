package main

import (
	"encoding/binary"
	"fmt"
)

const bmpFileHeaderSize = 14

func dibToBMP(dib []byte) ([]byte, error) {
	if len(dib) < 40 {
		return nil, fmt.Errorf("DIB is too short: got %d bytes", len(dib))
	}

	headerSize := binary.LittleEndian.Uint32(dib[0:4])
	if headerSize < 40 || uint64(headerSize) > uint64(len(dib)) {
		return nil, fmt.Errorf("unsupported DIB header size %d", headerSize)
	}

	bitCount := binary.LittleEndian.Uint16(dib[14:16])
	colorsUsed := binary.LittleEndian.Uint32(dib[32:36])
	if colorsUsed == 0 && bitCount <= 8 {
		colorsUsed = uint32(1) << bitCount
	}

	pixelOffset := uint64(bmpFileHeaderSize) + uint64(headerSize) + uint64(colorsUsed)*4
	fileSize := uint64(bmpFileHeaderSize) + uint64(len(dib))
	if pixelOffset > fileSize || fileSize > uint64(^uint32(0)) {
		return nil, fmt.Errorf("invalid DIB dimensions: pixel offset %d, file size %d", pixelOffset, fileSize)
	}

	bmp := make([]byte, fileSize)
	copy(bmp[0:2], "BM")
	binary.LittleEndian.PutUint32(bmp[2:6], uint32(fileSize))
	binary.LittleEndian.PutUint32(bmp[10:14], uint32(pixelOffset))
	copy(bmp[bmpFileHeaderSize:], dib)
	return bmp, nil
}

func validateBMP(bmp []byte) error {
	if len(bmp) < bmpFileHeaderSize+4 {
		return fmt.Errorf("BMP is too short: got %d bytes", len(bmp))
	}
	if string(bmp[:2]) != "BM" {
		return fmt.Errorf("invalid BMP signature %q", bmp[:2])
	}
	if declared := binary.LittleEndian.Uint32(bmp[2:6]); uint64(declared) != uint64(len(bmp)) {
		return fmt.Errorf("declared file size %d does not match actual size %d", declared, len(bmp))
	}
	dibHeaderSize := binary.LittleEndian.Uint32(bmp[14:18])
	if dibHeaderSize < 40 || uint64(bmpFileHeaderSize)+uint64(dibHeaderSize) > uint64(len(bmp)) {
		return fmt.Errorf("unsupported DIB header size %d", dibHeaderSize)
	}
	pixelOffset := binary.LittleEndian.Uint32(bmp[10:14])
	minimumOffset := uint64(bmpFileHeaderSize) + uint64(dibHeaderSize)
	if uint64(pixelOffset) < minimumOffset || uint64(pixelOffset) > uint64(len(bmp)) {
		return fmt.Errorf("invalid pixel offset %d for DIB header ending at %d", pixelOffset, minimumOffset)
	}
	return nil
}
