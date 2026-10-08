package main

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

type fullStageFixture struct {
	source  string
	output  string
	args    []string
	targets []dllTarget
}

func newFullStageFixture(t *testing.T) fullStageFixture {
	t.Helper()
	source := t.TempDir()
	output := t.TempDir()
	dib := make([]byte, 44)
	binary.LittleEndian.PutUint32(dib[0:4], 40)
	binary.LittleEndian.PutUint16(dib[12:14], 1)
	binary.LittleEndian.PutUint16(dib[14:16], 24)
	if err := os.WriteFile(filepath.Join(source, "TEST.DLL"), buildTestPE32WithBitmap(t, 88, 1033, dib), 0o600); err != nil {
		t.Fatal(err)
	}
	writeAudioFixture(t, source)
	return fullStageFixture{
		source: source,
		output: output,
		args: []string{
			"--no-verify",
			"--source", source,
			"--output", output,
			"--audio-output", filepath.Join(output, "sounds"),
		},
		targets: []dllTarget{{Filename: "TEST.DLL", Directory: "test-dll", Expected: 1}},
	}
}

func (fixture fullStageFixture) stage(t *testing.T, run mediaRunner) (string, error) {
	t.Helper()
	var stdout, stderr bytes.Buffer
	err := runTestCLIWithRunner(t, fixture.args, &stdout, &stderr, fixture.targets, run)
	if err != nil {
		return stdout.String(), fmt.Errorf("stage CLI: %w; stderr = %s", err, stderr.String())
	}
	return stdout.String(), nil
}

func TestFullCLIStageCacheSkipsUnchangedExtraction(t *testing.T) {
	fixture := newFullStageFixture(t)
	if _, err := fixture.stage(t, fakeMedia); err != nil {
		t.Fatal(err)
	}
	cache := filepath.Join(fixture.output, ".stage-ui-assets.json")
	if _, err := os.Stat(cache); err != nil {
		t.Fatalf("staging cache: %v", err)
	}

	rejectMedia := func(name string, args ...string) ([]byte, error) {
		return nil, fmt.Errorf("unexpected media invocation: %s %v", name, args)
	}
	stdout, err := fixture.stage(t, rejectMedia)
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(stdout, "Staging cache hit") {
		t.Fatalf("stdout = %q, want cache-hit summary", stdout)
	}
}

func TestFullCLIStageCacheInvalidatesChangedInputsAndMissingOutputs(t *testing.T) {
	tests := []struct {
		name   string
		mutate func(t *testing.T, fixture fullStageFixture) []string
	}{
		{
			name: "changed source",
			mutate: func(t *testing.T, fixture fullStageFixture) []string {
				t.Helper()
				path := filepath.Join(fixture.source, "ENCYTEXT.DLL")
				file, err := os.OpenFile(path, os.O_APPEND|os.O_WRONLY, 0)
				if err != nil {
					t.Fatal(err)
				}
				if _, err := file.Write([]byte("changed")); err != nil {
					file.Close()
					t.Fatal(err)
				}
				if err := file.Close(); err != nil {
					t.Fatal(err)
				}
				return fixture.args
			},
		},
		{
			name: "missing output",
			mutate: func(t *testing.T, fixture fullStageFixture) []string {
				t.Helper()
				if err := os.Remove(filepath.Join(fixture.output, "test-dll", "BMP", "88.bmp")); err != nil {
					t.Fatal(err)
				}
				return fixture.args
			},
		},
		{
			name: "force requested",
			mutate: func(t *testing.T, fixture fullStageFixture) []string {
				t.Helper()
				return append(append([]string{}, fixture.args...), "--force")
			},
		},
	}
	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			fixture := newFullStageFixture(t)
			if _, err := fixture.stage(t, fakeMedia); err != nil {
				t.Fatal(err)
			}
			cache := filepath.Join(fixture.output, ".stage-ui-assets.json")
			if _, err := os.Stat(cache); err != nil {
				t.Fatalf("staging cache: %v", err)
			}
			fixture.args = test.mutate(t, fixture)

			mediaCalls := 0
			rejectMedia := func(name string, args ...string) ([]byte, error) {
				mediaCalls++
				return nil, fmt.Errorf("expected cache invalidation: %s %v", name, args)
			}
			if _, err := fixture.stage(t, rejectMedia); err == nil || mediaCalls == 0 {
				t.Fatalf("unsafe cache hit: err = %v, media calls = %d", err, mediaCalls)
			}
		})
	}
}
