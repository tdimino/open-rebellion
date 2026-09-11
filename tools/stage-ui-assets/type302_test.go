package main

import (
	"encoding/binary"
	"testing"
)

func validType302Fixture() []byte {
	payload := []byte{
		1, 2, 4, 5, 1, // row 0: skip 1, draw 2, skip 1
		0, 4, 1, 2, 3, 4, // row 1: draw all 4 pixels
	}
	data := make([]byte, type302HeaderSize+2*4+len(payload))
	binary.LittleEndian.PutUint16(data[0:2], 4)
	binary.LittleEndian.PutUint16(data[2:4], 2)
	binary.LittleEndian.PutUint32(data[4:8], uint32(len(payload)))
	binary.LittleEndian.PutUint32(data[type302HeaderSize:type302HeaderSize+4], 0)
	binary.LittleEndian.PutUint32(data[type302HeaderSize+4:type302HeaderSize+8], 5)
	copy(data[type302HeaderSize+8:], payload)
	return data
}

func TestValidateType302AcceptsSparseScanlines(t *testing.T) {
	if err := validateType302(validType302Fixture()); err != nil {
		t.Fatalf("validateType302() error = %v", err)
	}
}

func TestValidateType302RejectsTruncatedLiteralRun(t *testing.T) {
	data := validType302Fixture()
	data = data[:len(data)-1]
	binary.LittleEndian.PutUint32(data[4:8], uint32(len(data)-(type302HeaderSize+2*4)))
	if err := validateType302(data); err == nil {
		t.Fatal("validateType302() error = nil, want truncated-run error")
	}
}

func TestValidateType302RejectsOutOfBoundsRowOffset(t *testing.T) {
	data := validType302Fixture()
	binary.LittleEndian.PutUint32(data[type302HeaderSize:type302HeaderSize+4], 999)
	if err := validateType302(data); err == nil {
		t.Fatal("validateType302() error = nil, want row-offset error")
	}
}

func TestValidateType302RejectsPathologicalDimensions(t *testing.T) {
	height := int(^uint16(0))
	data := make([]byte, type302HeaderSize+height*4+1)
	binary.LittleEndian.PutUint16(data[0:2], ^uint16(0))
	binary.LittleEndian.PutUint16(data[2:4], ^uint16(0))
	binary.LittleEndian.PutUint32(data[4:8], 1)
	if err := validateType302(data); err == nil {
		t.Fatal("validateType302() error = nil, want dimension-limit error")
	}
}
