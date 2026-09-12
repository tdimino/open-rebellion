package main

import (
	"bytes"
	"encoding/binary"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"sort"
)

// These aliases follow the native app's music_file mapping. MDATA.201/202
// are Smacker movies and remain the cutscene decoder's responsibility.
var musicAliases = map[int][]string{
	300: {"main_theme.wav", "endor.wav"},
	306: {"imperial.wav"}, 307: {"battle.wav"}, 312: {"hoth.wav"},
}
var menuAudio = map[uint32]string{
	8000: "menu_galaxy_size.wav", 8001: "menu_load_options.wav",
	8002: "menu_quit.wav", 8004: "menu_select.wav",
}
var voiceAudio = []struct{ DLL, Faction, Suffix string }{
	{"VOICEFXA.DLL", "alliance", "voicefxa"}, {"VOICEFXE.DLL", "empire", "voicefxe"},
}

// Validate the RIFF envelope and chunk boundaries without re-encoding samples.
// Unknown chunks and non-PCM formats are retained, including decoder metadata.
func validateWAV(data []byte) error {
	if len(data) < 12 || string(data[:4]) != "RIFF" || string(data[8:12]) != "WAVE" {
		return fmt.Errorf("not a RIFF WAVE file")
	}
	riffEnd := uint64(binary.LittleEndian.Uint32(data[4:8])) + 8
	// Original VOICEFXE WAVE 15053 stores its final zero pad outside the
	// declared odd RIFF length. Retain that byte and reject other trailing data.
	paddedEnd := riffEnd + (riffEnd & 1)
	if riffEnd != uint64(len(data)) && !(paddedEnd == uint64(len(data)) && data[len(data)-1] == 0) {
		return fmt.Errorf("RIFF size does not match file length")
	}
	haveFmt, haveData := false, false
	for offset := uint64(12); offset < uint64(len(data)); {
		if offset+8 > uint64(len(data)) {
			return fmt.Errorf("truncated WAV chunk header")
		}
		size := uint64(binary.LittleEndian.Uint32(data[offset+4 : offset+8]))
		end := offset + 8 + size
		if end > riffEnd {
			return fmt.Errorf("truncated WAV chunk payload")
		}
		switch string(data[offset : offset+4]) {
		case "fmt ":
			if haveFmt || size < 16 {
				return fmt.Errorf("invalid or duplicate WAV format chunk")
			}
			if binary.LittleEndian.Uint16(data[offset+10:]) == 0 || binary.LittleEndian.Uint32(data[offset+12:]) == 0 {
				return fmt.Errorf("invalid WAV channels or sample rate")
			}
			haveFmt = true
		case "data":
			if haveData || size == 0 {
				return fmt.Errorf("empty or duplicate WAV data chunk")
			}
			haveData = true
		}
		offset = end + (size & 1)
		if offset > uint64(len(data)) {
			return fmt.Errorf("missing WAV chunk padding")
		}
	}
	if !haveFmt || !haveData {
		return fmt.Errorf("WAV requires format and data chunks")
	}
	return nil
}

func writeAudio(path string, data []byte, force bool) (bool, error) {
	if err := validateWAV(data); err != nil {
		return false, err
	}
	if existing, err := os.ReadFile(path); err == nil {
		if bytes.Equal(existing, data) {
			return false, nil
		}
		if !force {
			return false, fmt.Errorf("%s exists with different contents (use --force to replace it)", path)
		}
	} else if !os.IsNotExist(err) {
		return false, err
	}
	if err := os.MkdirAll(filepath.Dir(path), 0755); err != nil {
		return false, err
	}
	if err := writeFileAtomically(path, data, 0644); err != nil {
		return false, err
	}
	return true, nil
}

func uniqueWaves(resources []rawResource) (map[uint32][]byte, error) {
	result := make(map[uint32][]byte)
	for _, r := range resources {
		if _, ok := result[r.ID]; ok {
			return nil, fmt.Errorf("duplicate WAVE ID %d across languages", r.ID)
		}
		if err := validateWAV(r.Data); err != nil {
			return nil, fmt.Errorf("WAVE %d: %w", r.ID, err)
		}
		result[r.ID] = r.Data
	}
	return result, nil
}

func stageAudio(source, mdata, output string, force bool, log io.Writer) error {
	// Collect and validate all inputs before writing any audio files.
	type asset struct {
		name string
		data []byte
	}
	var assets []asset
	inventory := make(map[string][]uint32)
	for _, v := range voiceAudio {
		resources, err := readPEWaveResources(filepath.Join(source, v.DLL))
		if err != nil {
			return fmt.Errorf("%s: %w", v.DLL, err)
		}
		waves, err := uniqueWaves(resources)
		if err != nil {
			return fmt.Errorf("%s: %w", v.DLL, err)
		}
		if len(waves) == 0 {
			return fmt.Errorf("%s contains no WAVE resources", v.DLL)
		}
		for _, r := range resources {
			inventory[v.Faction] = append(inventory[v.Faction], r.ID)
			assets = append(assets, asset{filepath.Join("voice", v.Faction, fmt.Sprintf("%d-%s.wav", r.ID, v.Suffix)), r.Data})
		}
	}
	resources, err := readPEWaveResources(filepath.Join(source, "COMMON.DLL"))
	if err != nil {
		return err
	}
	waves, err := uniqueWaves(resources)
	if err != nil {
		return err
	}
	for _, id := range []uint32{8000, 8001, 8002, 8004} {
		data, ok := waves[id]
		if !ok {
			return fmt.Errorf("COMMON.DLL is missing WAVE %d", id)
		}
		assets = append(assets, asset{filepath.Join("sfx", menuAudio[id]), data})
	}
	for id := 300; id <= 315; id++ {
		path := filepath.Join(mdata, fmt.Sprintf("MDATA.%d", id))
		data, err := os.ReadFile(path)
		if err != nil {
			return err
		}
		if err := validateWAV(data); err != nil {
			return fmt.Errorf("%s: %w", path, err)
		}
		names := append([]string{fmt.Sprintf("%d.wav", id)}, musicAliases[id]...)
		for _, name := range names {
			assets = append(assets, asset{filepath.Join("music", name), data})
		}
	}
	for _, ids := range inventory {
		sort.Slice(ids, func(i, j int) bool { return ids[i] < ids[j] })
	}
	manifest, err := json.MarshalIndent(inventory, "", "  ")
	if err != nil {
		return err
	}
	manifest = append(manifest, '\n')
	manifestPath := filepath.Join(output, "voice-inventory.json")
	if existing, err := os.ReadFile(manifestPath); err == nil {
		if !bytes.Equal(existing, manifest) && !force {
			return fmt.Errorf("voice inventory differs (use --force to replace it)")
		}
	} else if !os.IsNotExist(err) {
		return err
	}
	written := 0
	for _, a := range assets {
		changed, err := writeAudio(filepath.Join(output, a.name), a.data, force)
		if err != nil {
			return err
		}
		if changed {
			written++
		}
	}
	if existing, err := os.ReadFile(manifestPath); err != nil || !bytes.Equal(existing, manifest) {
		if err := writeFileAtomically(manifestPath, manifest, 0644); err != nil {
			return err
		}
	}
	fmt.Fprintf(log, "Staged %d audio files (%d written, %d unchanged)\n", len(assets), written, len(assets)-written)
	return nil
}

func verifyAudio(output string, log io.Writer) error {
	count := 0
	check := func(path string) error {
		data, err := os.ReadFile(path)
		if err != nil {
			return err
		}
		if err := validateWAV(data); err != nil {
			return fmt.Errorf("%s: %w", path, err)
		}
		count++
		return nil
	}
	for id := 300; id <= 315; id++ {
		for _, name := range append([]string{fmt.Sprintf("%d.wav", id)}, musicAliases[id]...) {
			if err := check(filepath.Join(output, "music", name)); err != nil {
				return err
			}
		}
	}
	for _, id := range []uint32{8000, 8001, 8002, 8004} {
		if err := check(filepath.Join(output, "sfx", menuAudio[id])); err != nil {
			return err
		}
	}
	manifest, err := os.ReadFile(filepath.Join(output, "voice-inventory.json"))
	if err != nil {
		return err
	}
	var inventory map[string][]uint32
	if err := json.Unmarshal(manifest, &inventory); err != nil {
		return fmt.Errorf("read voice inventory: %w", err)
	}
	if len(inventory) != len(voiceAudio) {
		return fmt.Errorf("voice inventory must contain both factions")
	}
	for _, v := range voiceAudio {
		ids := inventory[v.Faction]
		if len(ids) == 0 {
			return fmt.Errorf("empty %s voice inventory", v.Faction)
		}
		seen := make(map[uint32]bool)
		for _, id := range ids {
			if seen[id] {
				return fmt.Errorf("duplicate %s voice inventory ID %d", v.Faction, id)
			}
			seen[id] = true
			if err := check(filepath.Join(output, "voice", v.Faction, fmt.Sprintf("%d-%s.wav", id, v.Suffix))); err != nil {
				return err
			}
		}
	}
	fmt.Fprintf(log, "Verified %d audio files\n", count)
	return nil
}
