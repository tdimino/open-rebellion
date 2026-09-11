package main

import (
	"encoding/binary"
	"fmt"
)

const (
	type302HeaderSize = 17
	maxType302Width   = 640
	maxType302Height  = 480
	maxType302Pixels  = maxType302Width * maxType302Height
)

// validateType302 checks the custom sparse-frame layout used by ALSPRITE.DLL
// and EMSPRITE.DLL. The executable loader at FUN_0041c6c0 reads the 17-byte
// header, one little-endian scanline offset per row, then the encoded payload.
// Each row alternates an unchanged-pixel skip count and an additive byte run.
func validateType302(data []byte) error {
	if len(data) < type302HeaderSize {
		return fmt.Errorf("type-302 frame is too short: got %d bytes", len(data))
	}
	width := int(binary.LittleEndian.Uint16(data[0:2]))
	height := int(binary.LittleEndian.Uint16(data[2:4]))
	payloadSize := int(binary.LittleEndian.Uint32(data[4:8]))
	if width <= 0 || height <= 0 || width > maxType302Width || height > maxType302Height || width*height > maxType302Pixels {
		return fmt.Errorf("type-302 frame has invalid dimensions %dx%d", width, height)
	}
	payloadStart := type302HeaderSize + height*4
	if payloadStart > len(data) || payloadSize != len(data)-payloadStart {
		return fmt.Errorf("type-302 payload size %d does not match actual size %d", payloadSize, len(data)-payloadStart)
	}
	payload := data[payloadStart:]
	for row := 0; row < height; row++ {
		offsetStart := type302HeaderSize + row*4
		offset := int(binary.LittleEndian.Uint32(data[offsetStart : offsetStart+4]))
		if offset < 0 || offset >= len(payload) {
			return fmt.Errorf("type-302 row %d offset %d is outside %d-byte payload", row, offset, len(payload))
		}

		x := 0
		cursor := offset
		literal := false
		for x < width {
			if cursor >= len(payload) {
				return fmt.Errorf("type-302 row %d ends before pixel %d", row, x)
			}
			count := int(payload[cursor])
			cursor++
			if x+count > width {
				return fmt.Errorf("type-302 row %d run crosses width at pixel %d", row, x)
			}
			if literal {
				if cursor+count > len(payload) {
					return fmt.Errorf("type-302 row %d literal run is truncated", row)
				}
				cursor += count
			}
			x += count
			literal = !literal
		}
	}
	return nil
}
