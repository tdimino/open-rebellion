package main

import (
	"encoding/binary"
	"testing"
)

func TestDIBToBMPAddsFileHeaderAndPaletteOffset(t *testing.T) {
	dib := make([]byte, 40+8+4)
	binary.LittleEndian.PutUint32(dib[0:4], 40) // BITMAPINFOHEADER
	binary.LittleEndian.PutUint32(dib[4:8], 1)  // width
	binary.LittleEndian.PutUint32(dib[8:12], 1) // height
	binary.LittleEndian.PutUint16(dib[12:14], 1)
	binary.LittleEndian.PutUint16(dib[14:16], 8) // bits per pixel
	binary.LittleEndian.PutUint32(dib[32:36], 2) // palette entries

	bmp, err := dibToBMP(dib)
	if err != nil {
		t.Fatalf("dibToBMP() error = %v", err)
	}
	if got := string(bmp[:2]); got != "BM" {
		t.Fatalf("signature = %q, want BM", got)
	}
	if got, want := binary.LittleEndian.Uint32(bmp[2:6]), uint32(len(dib)+14); got != want {
		t.Errorf("file size = %d, want %d", got, want)
	}
	if got, want := binary.LittleEndian.Uint32(bmp[10:14]), uint32(14+40+8); got != want {
		t.Errorf("pixel offset = %d, want %d", got, want)
	}
}

func TestValidateBMPRejectsIncorrectDeclaredFileSize(t *testing.T) {
	dib := make([]byte, 44)
	binary.LittleEndian.PutUint32(dib[0:4], 40)
	binary.LittleEndian.PutUint16(dib[12:14], 1)
	binary.LittleEndian.PutUint16(dib[14:16], 24)
	bmp, err := dibToBMP(dib)
	if err != nil {
		t.Fatal(err)
	}
	binary.LittleEndian.PutUint32(bmp[2:6], uint32(len(bmp)+1))

	if err := validateBMP(bmp); err == nil {
		t.Fatal("validateBMP() error = nil, want file-size error")
	}
}

func TestValidateBMPRejectsIncorrectSignature(t *testing.T) {
	bmp := make([]byte, 54)
	copy(bmp[:2], "ZZ")
	binary.LittleEndian.PutUint32(bmp[2:6], uint32(len(bmp)))
	binary.LittleEndian.PutUint32(bmp[10:14], 54)
	binary.LittleEndian.PutUint32(bmp[14:18], 40)

	if err := validateBMP(bmp); err == nil {
		t.Fatal("validateBMP() error = nil, want signature error")
	}
}

func TestValidateBMPRejectsPixelOffsetBeforeDIBHeaderEnd(t *testing.T) {
	bmp := make([]byte, 54)
	copy(bmp[:2], "BM")
	binary.LittleEndian.PutUint32(bmp[2:6], uint32(len(bmp)))
	binary.LittleEndian.PutUint32(bmp[10:14], 20)
	binary.LittleEndian.PutUint32(bmp[14:18], 40)

	if err := validateBMP(bmp); err == nil {
		t.Fatal("validateBMP() error = nil, want pixel-offset error")
	}
}
