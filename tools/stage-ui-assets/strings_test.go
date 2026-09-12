package main

import (
	"encoding/binary"
	"testing"
	"unicode/utf16"
)

func stringBundle(values map[int]string) []byte {
	var b []byte
	for i := 0; i < 16; i++ {
		u := utf16.Encode([]rune(values[i]))
		b = binary.LittleEndian.AppendUint16(b, uint16(len(u)))
		for _, c := range u {
			b = binary.LittleEndian.AppendUint16(b, c)
		}
	}
	return b
}
func TestExtractStringBundles(t *testing.T) {
	b := stringBundle(map[int]string{0: "Luke", 15: "星😀"})
	got, err := decodeStringResources([]rawResource{{ID: 2, Language: 1033, Data: b}})
	if err != nil {
		t.Fatal(err)
	}
	if len(got) != 2 || got[16] != "Luke" || got[31] != "星😀" {
		t.Fatal(got)
	}
	for _, r := range []rawResource{{ID: 0, Data: b}, {ID: 4097, Data: b}, {ID: 1, Data: b[:4]}, {ID: 1, Data: []byte{1, 0, 0, 0xd8}}} {
		if _, err := decodeStringResources([]rawResource{r}); err == nil {
			t.Fatal("accepted invalid string bundle")
		}
	}
	if _, err := decodeStringResources([]rawResource{{ID: 1, Data: b}, {ID: 1, Data: b}}); err == nil {
		t.Fatal("accepted duplicate language bundle")
	}
}
