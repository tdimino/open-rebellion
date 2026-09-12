package main

import (
	"bytes"
	"crypto/sha256"
	"encoding/binary"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"unicode/utf16"
)

func decodeStringResources(resources []rawResource) (map[uint16]string, error) {
	out := make(map[uint16]string)
	seen := make(map[uint32]bool)
	for _, r := range resources {
		if r.ID == 0 || r.ID > 4096 {
			return nil, fmt.Errorf("string bundle ID %d outside u16 string range", r.ID)
		}
		if seen[r.ID] {
			return nil, fmt.Errorf("duplicate string bundle %d across languages", r.ID)
		}
		seen[r.ID] = true
		pos := 0
		for slot := uint32(0); slot < 16; slot++ {
			if pos+2 > len(r.Data) {
				return nil, fmt.Errorf("truncated string bundle %d", r.ID)
			}
			n := int(binary.LittleEndian.Uint16(r.Data[pos:]))
			pos += 2
			if pos+n*2 > len(r.Data) {
				return nil, fmt.Errorf("truncated string in bundle %d", r.ID)
			}
			units := make([]uint16, n)
			for i := range units {
				units[i] = binary.LittleEndian.Uint16(r.Data[pos+i*2:])
			}
			pos += n * 2
			for i := 0; i < n; i++ {
				c := units[i]
				if c >= 0xd800 && c <= 0xdbff {
					if i+1 >= n || units[i+1] < 0xdc00 || units[i+1] > 0xdfff {
						return nil, fmt.Errorf("invalid UTF-16 in bundle %d", r.ID)
					}
					i++
				} else if c >= 0xdc00 && c <= 0xdfff {
					return nil, fmt.Errorf("invalid UTF-16 in bundle %d", r.ID)
				}
			}
			if n > 0 {
				out[uint16((r.ID-1)*16+slot)] = string(utf16.Decode(units))
			}
		}
		// PE string resources may have alignment padding, but no additional text.
		if len(r.Data)-pos > 3 {
			return nil, fmt.Errorf("extra data in string bundle %d", r.ID)
		}
		for _, b := range r.Data[pos:] {
			if b != 0 {
				return nil, fmt.Errorf("nonzero string bundle padding")
			}
		}
	}
	if len(out) == 0 {
		return nil, fmt.Errorf("TEXTSTRA contains no strings")
	}
	return out, nil
}

func fileSHA256(path string) (string, error) {
	f, err := os.Open(path)
	if err != nil {
		return "", err
	}
	defer f.Close()
	h := sha256.New()
	if _, err := io.Copy(h, f); err != nil {
		return "", err
	}
	return hex.EncodeToString(h.Sum(nil)), nil
}
func byteSHA256(b []byte) string { h := sha256.Sum256(b); return hex.EncodeToString(h[:]) }
func writeAsset(path string, b []byte, force bool) error {
	if old, err := os.ReadFile(path); err == nil {
		if bytes.Equal(old, b) {
			return nil
		}
		if !force {
			return fmt.Errorf("%s differs (use --force to replace it)", path)
		}
	} else if !os.IsNotExist(err) {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(path), 0755); err != nil {
		return err
	}
	return writeFileAtomically(path, b, 0644)
}

type stringManifest struct {
	Count  int    `json:"count"`
	SHA256 string `json:"sha256"`
}

func stageStrings(source, output string, force bool, log io.Writer) error {
	rs, err := readPERawResources(filepath.Join(source, "TEXTSTRA.DLL"), 6)
	if err != nil {
		return err
	}
	strings, err := decodeStringResources(rs)
	if err != nil {
		return err
	}
	b, err := json.MarshalIndent(strings, "", "  ")
	if err != nil {
		return err
	}
	b = append(b, '\n')
	manifest, err := json.MarshalIndent(stringManifest{len(strings), byteSHA256(b)}, "", "  ")
	if err != nil {
		return err
	}
	if err := writeAsset(output, b, force); err != nil {
		return err
	}
	if err := writeAsset(output+".manifest.json", append(manifest, '\n'), force); err != nil {
		return err
	}
	fmt.Fprintf(log, "Staged %d TEXTSTRA strings\n", len(strings))
	return nil
}
func verifyStrings(output string, log io.Writer) error {
	b, err := os.ReadFile(output)
	if err != nil {
		return err
	}
	var strings map[uint16]string
	if err := json.Unmarshal(b, &strings); err != nil {
		return err
	}
	mb, err := os.ReadFile(output + ".manifest.json")
	if err != nil {
		return err
	}
	var m stringManifest
	if err := json.Unmarshal(mb, &m); err != nil {
		return err
	}
	if len(strings) == 0 || m.Count != len(strings) || m.SHA256 != byteSHA256(b) {
		return fmt.Errorf("TEXTSTRA inventory or checksum mismatch")
	}
	for _, value := range strings {
		if value == "" {
			return fmt.Errorf("empty TEXTSTRA entry")
		}
	}
	fmt.Fprintf(log, "Verified %d TEXTSTRA strings\n", len(strings))
	return nil
}
