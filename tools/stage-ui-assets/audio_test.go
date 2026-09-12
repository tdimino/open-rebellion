package main

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"strings"

	"os"
	"path/filepath"
	"testing"
)

func testWAV() []byte {
	b := make([]byte, 46)
	copy(b, "RIFF")
	binary.LittleEndian.PutUint32(b[4:], 38)
	copy(b[8:], "WAVEfmt ")
	binary.LittleEndian.PutUint32(b[16:], 16)
	binary.LittleEndian.PutUint16(b[20:], 1)
	binary.LittleEndian.PutUint16(b[22:], 1)
	binary.LittleEndian.PutUint32(b[24:], 11025)
	binary.LittleEndian.PutUint32(b[28:], 22050)
	binary.LittleEndian.PutUint16(b[32:], 2)
	binary.LittleEndian.PutUint16(b[34:], 16)
	copy(b[36:], "data")
	binary.LittleEndian.PutUint32(b[40:], 2)
	return b
}

func TestValidateWAV(t *testing.T) {
	valid := testWAV()
	if err := validateWAV(valid); err != nil {
		t.Fatal(err)
	}
	for _, data := range [][]byte{nil, []byte("SMK2"), valid[:40], append(append([]byte{}, valid...), 0)} {
		if validateWAV(data) == nil {
			t.Fatal("accepted invalid WAV")
		}
	}
	bad := append([]byte{}, valid...)
	binary.LittleEndian.PutUint32(bad[40:], 1000)
	if validateWAV(bad) == nil {
		t.Fatal("accepted truncated chunk")
	}
}

func TestAudioWritesPreserveBytesAndRequireForce(t *testing.T) {
	path := filepath.Join(t.TempDir(), "music", "test.wav")
	data := testWAV()
	written, err := writeAudio(path, data, false)
	if err != nil || !written {
		t.Fatalf("write: %v %v", written, err)
	}
	written, err = writeAudio(path, data, false)
	if err != nil || written {
		t.Fatalf("repeat: %v %v", written, err)
	}
	changed := append([]byte{}, data...)
	changed[44] = 1
	if _, err = writeAudio(path, changed, false); err == nil {
		t.Fatal("overwrote without force")
	}
	if _, err = writeAudio(path, changed, true); err != nil {
		t.Fatal(err)
	}
	got, _ := os.ReadFile(path)
	if !bytes.Equal(got, changed) {
		t.Fatal("bytes changed")
	}
}

func TestReadNamedWaveResource(t *testing.T) {
	wav := testWAV()
	pe := buildTestPE32WithResource(t, 10, 14001, 1033, wav)
	// Replace the resource type with the UTF-16 name WAVE in unused section space.
	binary.LittleEndian.PutUint16(pe[0x20c:], 1)
	binary.LittleEndian.PutUint16(pe[0x20e:], 0)
	binary.LittleEndian.PutUint32(pe[0x210:], 0x80000180)
	binary.LittleEndian.PutUint16(pe[0x380:], 4)
	for i, c := range "WAVE" {
		binary.LittleEndian.PutUint16(pe[0x382+i*2:], uint16(c))
	}
	path := filepath.Join(t.TempDir(), "VOICE.DLL")
	if err := os.WriteFile(path, pe, 0600); err != nil {
		t.Fatal(err)
	}
	resources, err := readPEWaveResources(path)
	if err != nil {
		t.Fatal(err)
	}
	if len(resources) != 1 || resources[0].ID != 14001 || !bytes.Equal(resources[0].Data, wav) {
		t.Fatal("wrong WAVE resource")
	}
}

func writeWaveDLL(t *testing.T, dir, name string, ids ...uint32) {
	t.Helper()
	const rva = 0x1000
	const sectionSize = 4096
	pe := append(buildTestPE32WithResource(t, 10, 0, 1033, nil)[:0x200], make([]byte, sectionSize)...)
	// PE32 section descriptor and resource directory sizes.
	binary.LittleEndian.PutUint32(pe[0x178+8:], sectionSize)
	binary.LittleEndian.PutUint32(pe[0x178+16:], sectionSize)
	binary.LittleEndian.PutUint32(pe[0x108+4:], sectionSize)
	r := pe[0x200:]
	putResourceDirectory(r, 0, 1, 0)
	putResourceEntry(r, 0x10, 0x80000040, 0x80000060)
	binary.LittleEndian.PutUint16(r[0x40:], 4)
	for i, c := range "WAVE" {
		binary.LittleEndian.PutUint16(r[0x42+i*2:], uint16(c))
	}
	putResourceDirectory(r, 0x60, 0, uint16(len(ids)))
	for i, id := range ids {
		lang := 0x100 + i*0x40
		dataOffset := 0x400 + i*0x100
		putResourceEntry(r, 0x70+i*8, id, uint32(lang)|0x80000000)
		putResourceDirectory(r, lang, 0, 1)
		putResourceEntry(r, lang+16, 1033, uint32(lang+24))
		wav := testWAV()
		binary.LittleEndian.PutUint32(r[lang+24:], uint32(rva+dataOffset))
		binary.LittleEndian.PutUint32(r[lang+28:], uint32(len(wav)))
		copy(r[dataOffset:], wav)
	}
	if err := os.WriteFile(filepath.Join(dir, name), pe, 0600); err != nil {
		t.Fatal(err)
	}
}

func TestAudioCLIStagesAndVerifiesWithoutSources(t *testing.T) {
	source, output := t.TempDir(), t.TempDir()
	writeAudioFixture(t, source)
	var out, errs bytes.Buffer
	args := []string{"--source", source, "--output", filepath.Join(output, "ui"), "--audio-output", filepath.Join(output, "sounds")}
	if err := runCLI(args, &out, &errs, nil); err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(out.String(), "Staged 28 audio files (28 written, 0 unchanged)") {
		t.Fatal(out.String())
	}
	out.Reset()
	if err := runCLI(args, &out, &errs, nil); err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(out.String(), "0 written, 28 unchanged") {
		t.Fatal(out.String())
	}
	// Verify must operate on staged files without consulting source DLLs/MDATA.
	verify := []string{"--verify", "--source", filepath.Join(source, "missing"), "--output", filepath.Join(output, "ui"), "--audio-output", filepath.Join(output, "sounds")}
	if err := runCLI(verify, &out, &errs, nil); err != nil {
		t.Fatal(err)
	}
	voice := filepath.Join(output, "sounds", "voice", "alliance", "14002-voicefxa.wav")
	if err := os.Remove(voice); err != nil {
		t.Fatal(err)
	}
	if err := runCLI(verify, &out, &errs, nil); err == nil {
		t.Fatal("verification missed deleted voice")
	}
	if err := runCLI(args, &out, &errs, nil); err != nil {
		t.Fatal(err)
	}
	track := filepath.Join(output, "sounds", "music", "main_theme.wav")
	if err := os.WriteFile(track, []byte("broken"), 0600); err != nil {
		t.Fatal(err)
	}
	if err := runCLI(verify, &out, &errs, nil); err == nil {
		t.Fatal("accepted corrupt audio")
	}
	if err := runCLI(args, &out, &errs, nil); err == nil {
		t.Fatal("replaced corrupt audio without force")
	}
	if err := runCLI(append(args, "--force"), &out, &errs, nil); err != nil {
		t.Fatal(err)
	}
}

func TestDuplicateVoiceLanguagesAreRejected(t *testing.T) {
	_, err := uniqueWaves([]rawResource{{ID: 1, Language: 1033, Data: testWAV()}, {ID: 1, Language: 1036, Data: testWAV()}})
	if err == nil {
		t.Fatal("accepted ambiguous voice languages")
	}
}

func TestOriginalOddRIFFPadding(t *testing.T) {
	wav := testWAV()
	binary.LittleEndian.PutUint32(wav[4:], 37)
	binary.LittleEndian.PutUint32(wav[40:], 1)
	if err := validateWAV(wav); err != nil {
		t.Fatal(err)
	}
	wav[45] = 1
	if validateWAV(wav) == nil {
		t.Fatal("accepted nonzero RIFF padding")
	}
}

func TestRIFFPaddingCannotContainSampleData(t *testing.T) {
	wav := testWAV()
	binary.LittleEndian.PutUint32(wav[4:], 37)
	if validateWAV(wav) == nil {
		t.Fatal("accepted sample data beyond RIFF envelope")
	}
}

func writeAudioFixture(t *testing.T, source string) {
	t.Helper()
	mdata := filepath.Join(source, "MDATA")
	if err := os.Mkdir(mdata, 0700); err != nil {
		t.Fatal(err)
	}
	writeWaveDLL(t, source, "VOICEFXA.DLL", 14001, 14002)
	writeWaveDLL(t, source, "VOICEFXE.DLL", 15001)
	writeWaveDLL(t, source, "COMMON.DLL", 8000, 8001, 8002, 8004)
	for id := 300; id <= 315; id++ {
		if err := os.WriteFile(filepath.Join(mdata, fmt.Sprintf("MDATA.%d", id)), testWAV(), 0600); err != nil {
			t.Fatal(err)
		}
	}
}
