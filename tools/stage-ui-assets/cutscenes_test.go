package main

import (
	"bytes"
	"fmt"
	"image"
	"image/png"
	"io"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func fakeMedia(name string, args ...string) ([]byte, error) {
	if len(args) > 0 && args[0] == "-version" {
		return nil, nil
	}
	if name == "ffprobe" {
		return []byte(`{"streams":[{"width":2,"height":2,"avg_frame_rate":"15/1"}]}`), nil
	}
	output := args[len(args)-1]
	if strings.Contains(output, "%05d") {
		for i := 1; i <= 2; i++ {
			p := fmt.Sprintf(output, i)
			f, e := os.Create(p)
			if e != nil {
				return nil, e
			}
			e = png.Encode(f, image.NewRGBA(image.Rect(0, 0, 2, 2)))
			f.Close()
			if e != nil {
				return nil, e
			}
		}
		return nil, nil
	}
	data := []byte{0x1a, 0x45, 0xdf, 0xa3, 0, 0, 0, 0}
	if strings.HasSuffix(output, ".wav") {
		data = testWAV()
	}
	return nil, os.WriteFile(output, data, 0600)
}
func TestCutsceneExtractionAndVerification(t *testing.T) {
	source, output := t.TempDir(), t.TempDir()
	if e := os.WriteFile(filepath.Join(source, "MDATA.000"), []byte("SMK2fixture"), 0600); e != nil {
		t.Fatal(e)
	}
	var log bytes.Buffer
	if e := stageCutscene(source, output, "000", false, fakeMedia, &log); e != nil {
		t.Fatal(e)
	}
	if _, e := verifyCutscene(output, "000"); e != nil {
		t.Fatal(e)
	}
	fail := func(string, ...string) ([]byte, error) { return nil, fmt.Errorf("unexpected decoder invocation") }
	if e := stageCutscene(source, output, "000", false, fail, &log); e != nil {
		t.Fatal("unchanged extraction reran decoder", e)
	}
	frame := filepath.Join(output, "cutscene-frames", "000", "frame-00002.png")
	if e := os.Remove(frame); e != nil {
		t.Fatal(e)
	}
	if _, e := verifyCutscene(output, "000"); e == nil {
		t.Fatal("missed deleted frame")
	}
	if e := stageCutscene(source, output, "000", false, fakeMedia, &log); e == nil {
		t.Fatal("replaced without force")
	}
	if e := stageCutscene(source, output, "000", true, fakeMedia, &log); e != nil {
		t.Fatal(e)
	}
	before, _ := os.ReadFile(frame)
	if e := stageCutscene(source, output, "000", true, fail, &log); e != nil {
		t.Fatal("unchanged source should skip even with force", e)
	}
	if e := os.WriteFile(filepath.Join(source, "MDATA.000"), []byte("SMK2changed"), 0600); e != nil {
		t.Fatal(e)
	}
	if e := stageCutscene(source, output, "000", true, fail, &log); e == nil {
		t.Fatal("expected decoder failure")
	}
	after, _ := os.ReadFile(frame)
	if !bytes.Equal(before, after) {
		t.Fatal("decoder failure damaged existing frames")
	}
}

func runTestCLI(t *testing.T, args []string, stdout, stderr io.Writer, targets []dllTarget) error {
	return runTestCLIWithRunner(t, args, stdout, stderr, targets, fakeMedia)
}

func runTestCLIWithRunner(t *testing.T, args []string, stdout, stderr io.Writer, targets []dllTarget, run mediaRunner) error {
	t.Helper()
	var output string
	var source string
	verifyOnly := false
	for i, arg := range args {
		if arg == "--audio-output" {
			output = filepath.Dir(args[i+1])
		}
		if arg == "--source" {
			source = args[i+1]
		}
		if arg == "--verify" {
			verifyOnly = true
		}
	}
	if output == "" {
		return fmt.Errorf("test must isolate audio output")
	}
	if source == "" {
		return fmt.Errorf("test must provide a source directory")
	}
	if _, err := os.Stat(filepath.Join(source, "ENCYTEXT.DLL")); !verifyOnly && os.IsNotExist(err) {
		writeCompleteEncyclopediaFixture(t, source)
	}
	args = append(
		append([]string{}, args...),
		"--strings-output", filepath.Join(output, "textstra.json"),
		"--encyclopedia-output", filepath.Join(output, "encyclopedia", "source.json"),
		"--cutscene-output", filepath.Join(output, "media"),
	)
	return runCLIWithMedia(args, stdout, stderr, targets, []string{"000"}, run)
}

func TestCutscenePublishRollsBackEveryOutput(t *testing.T) {
	root, temp := t.TempDir(), t.TempDir()
	paths := []string{"ref-videos/000.webm", "cutscene-frames/000.wav", "cutscene-frames/000/frame.png"}
	for _, base := range []string{root, temp} {
		for _, p := range paths {
			full := filepath.Join(base, p)
			if e := os.MkdirAll(filepath.Dir(full), 0700); e != nil {
				t.Fatal(e)
			}
			if e := os.WriteFile(full, []byte(base), 0600); e != nil {
				t.Fatal(e)
			}
		}
	}
	calls := 0
	rename := func(a, b string) error {
		calls++
		if calls == 6 {
			return fmt.Errorf("injected frame publication failure")
		}
		return os.Rename(a, b)
	}
	retain, err := publishCutscene(temp, root, "000", rename)
	if err == nil || retain {
		t.Fatalf("expected recovered publication failure: %v %v", retain, err)
	}
	for _, p := range paths {
		b, e := os.ReadFile(filepath.Join(root, p))
		if e != nil || string(b) != root {
			t.Fatalf("old output not restored: %s %v", p, e)
		}
	}
}

func TestCutscenePublishKeepsBackupsIfRollbackFails(t *testing.T) {
	root, temp := t.TempDir(), t.TempDir()
	for _, base := range []string{root, temp} {
		for _, p := range []string{"ref-videos/000.webm", "cutscene-frames/000.wav", "cutscene-frames/000/frame.png"} {
			full := filepath.Join(base, p)
			if e := os.MkdirAll(filepath.Dir(full), 0700); e != nil {
				t.Fatal(e)
			}
			if e := os.WriteFile(full, []byte(base), 0600); e != nil {
				t.Fatal(e)
			}
		}
	}
	calls := 0
	retain, err := publishCutscene(temp, root, "000", func(a, b string) error {
		calls++
		if calls == 6 || calls == 8 {
			return fmt.Errorf("injected failure")
		}
		return os.Rename(a, b)
	})
	if err == nil || !retain {
		t.Fatalf("rollback failure must retain backups: %v %v", retain, err)
	}
	old, e := os.ReadFile(filepath.Join(temp, "backup-1"))
	if e != nil || string(old) != root {
		t.Fatal("missing retained original WAV", e)
	}
}

func TestCutscenePublishPreflightsDestinationTypes(t *testing.T) {
	root, temp := t.TempDir(), t.TempDir()
	if e := os.MkdirAll(filepath.Join(root, "cutscene-frames", "000.wav"), 0700); e != nil {
		t.Fatal(e)
	}
	calls := 0
	_, err := publishCutscene(temp, root, "000", func(a, b string) error { calls++; return os.Rename(a, b) })
	if err == nil || calls != 0 {
		t.Fatalf("unexpected output type must fail before renames: %d %v", calls, err)
	}
}
