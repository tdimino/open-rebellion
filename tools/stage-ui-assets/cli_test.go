package main

import (
	"bytes"
	"encoding/binary"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

func TestRunCLIStagesAndVerifiesConfiguredTargets(t *testing.T) {
	sourceDir := t.TempDir()
	outputDir := t.TempDir()
	dib := make([]byte, 44)
	binary.LittleEndian.PutUint32(dib[0:4], 40)
	binary.LittleEndian.PutUint16(dib[12:14], 1)
	binary.LittleEndian.PutUint16(dib[14:16], 24)
	if err := os.WriteFile(filepath.Join(sourceDir, "TEST.DLL"), buildTestPE32WithBitmap(t, 88, 1033, dib), 0o600); err != nil {
		t.Fatal(err)
	}
	writeAudioFixture(t, sourceDir)
	var stdout, stderr bytes.Buffer

	err := runTestCLI(
		[]string{"--source", sourceDir, "--output", outputDir, "--audio-output", filepath.Join(outputDir, "sounds")},
		&stdout,
		&stderr,
		[]dllTarget{{Filename: "TEST.DLL", Directory: "test-dll", Expected: 1}},
	)
	if err != nil {
		t.Fatalf("runTestCLI() error = %v; stderr = %s", err, stderr.String())
	}
	if _, err := os.Stat(filepath.Join(outputDir, "test-dll", "BMP", "88.bmp")); err != nil {
		t.Fatalf("staged runtime asset: %v", err)
	}
	if !bytes.Contains(stdout.Bytes(), []byte("Verified 1 UI resources")) {
		t.Errorf("stdout = %q, want verification summary", stdout.String())
	}
}

func TestRunCLITactical3DConvertMode(t *testing.T) {
	outputDir := t.TempDir()
	writeSyntheticTacticalRawStore(t, outputDir)
	var stdout, stderr bytes.Buffer
	if err := runCLIWithMedia(
		[]string{"--tactical-3d-convert", "--output", outputDir},
		&stdout,
		&stderr,
		nil,
		nil,
		nil,
	); err != nil {
		t.Fatalf("convert CLI error = %v; stderr = %s", err, stderr.String())
	}
	if !bytes.Contains(stdout.Bytes(), []byte("Verified tactical runtime pack")) {
		t.Fatalf("stdout = %q", stdout.String())
	}
	stdout.Reset()
	if err := runCLIWithMedia(
		[]string{"--tactical-3d-convert", "--verify", "--output", outputDir},
		&stdout,
		&stderr,
		nil,
		nil,
		nil,
	); err != nil {
		t.Fatalf("verify CLI error = %v; stderr = %s", err, stderr.String())
	}
}

func TestRunCLIRejectsMultipleTacticalModes(t *testing.T) {
	err := runCLIWithMedia(
		[]string{"--tactical-3d-only", "--tactical-3d-convert"},
		io.Discard,
		io.Discard,
		nil,
		nil,
		nil,
	)
	if err == nil {
		t.Fatal("multiple tactical modes were accepted")
	}
}

func TestEncyclopediaFocusedModeNeverInvokesMediaTools(t *testing.T) {
	source := t.TempDir()
	output := filepath.Join(t.TempDir(), "research")
	writeSyntheticEncyclopediaTextDLL(t, source, numericEncyclopediaResourceIdentifier(23), []byte("focused report\x00"))
	bmp := buildTestEncyclopediaBMP(t, testEncyclopediaBMPOptions{width: 1, height: 1, bitCount: 24})
	writeSyntheticEncyclopediaArtInputs(t, source, filepath.Join(source, "EData"), map[int]string{0: "EDATA.001"}, map[string][]byte{"EDATA.001": bmp})
	mediaCalls := 0
	failingMedia := func(string, ...string) ([]byte, error) {
		mediaCalls++
		return nil, io.ErrUnexpectedEOF
	}
	var stdout, stderr bytes.Buffer
	if err := runCLIWithMedia([]string{
		"--encyclopedia-report-only",
		"--source", source,
		"--encyclopedia-report-output", output,
	}, &stdout, &stderr, nil, nil, failingMedia); err != nil {
		t.Fatalf("focused report CLI error = %v; stderr = %s", err, stderr.String())
	}
	if mediaCalls != 0 {
		t.Fatalf("focused report invoked media tools %d times", mediaCalls)
	}
	if got := readTestFile(t, filepath.Join(output, "assets", "EDATA.001")); !bytes.Equal(got, bmp) {
		t.Fatal("focused report did not use the default source/EData root")
	}
	if err := runCLIWithMedia([]string{
		"--encyclopedia-report-only",
		"--verify",
		"--source", filepath.Join(t.TempDir(), "missing-install"),
		"--edata", filepath.Join(t.TempDir(), "missing-edata"),
		"--encyclopedia-report-output", output,
	}, &stdout, &stderr, nil, nil, failingMedia); err != nil {
		t.Fatalf("source-free report verify CLI error = %v; stderr = %s", err, stderr.String())
	}
	if mediaCalls != 0 {
		t.Fatalf("report verification invoked media tools %d times", mediaCalls)
	}
}

func TestCanonicalEncyclopediaFocusedVerifyNeverInvokesMediaTools(t *testing.T) {
	output := filepath.Join(t.TempDir(), "encyclopedia")
	copyTestEncyclopediaRuntimeBundle(t, output)
	mediaCalls := 0
	failingMedia := func(string, ...string) ([]byte, error) {
		mediaCalls++
		return nil, io.ErrUnexpectedEOF
	}
	var stdout, stderr bytes.Buffer
	if err := runCLIWithMedia([]string{
		"--encyclopedia-canonical-only",
		"--verify",
		"--source", filepath.Join(t.TempDir(), "missing-install"),
		"--edata", filepath.Join(t.TempDir(), "missing-edata"),
		"--encyclopedia-canonical-output", output,
	}, &stdout, &stderr, nil, nil, failingMedia); err != nil {
		t.Fatalf("canonical verify CLI error = %v; stderr = %s", err, stderr.String())
	}
	if mediaCalls != 0 {
		t.Fatalf("canonical verification invoked media tools %d times", mediaCalls)
	}
}

func TestEncyclopediaFocusedModesAreMutuallyExclusive(t *testing.T) {
	err := runCLIWithMedia(
		[]string{"--encyclopedia-report-only", "--encyclopedia-canonical-only"},
		io.Discard,
		io.Discard,
		nil,
		nil,
		func(string, ...string) ([]byte, error) { return nil, io.ErrUnexpectedEOF },
	)
	if err == nil || !strings.Contains(err.Error(), "mutually exclusive") {
		t.Fatalf("focused mode conflict error = %v", err)
	}
}

func TestFullStageRejectsPartialEncyclopediaBeforeMediaPrerequisites(t *testing.T) {
	source := t.TempDir()
	if err := os.WriteFile(filepath.Join(source, "ENCYTEXT.DLL"), []byte("partial"), 0o600); err != nil {
		t.Fatal(err)
	}
	mediaCalls := 0
	var stdout bytes.Buffer
	err := runCLIWithMedia(
		[]string{
			"--source", source,
			"--edata", filepath.Join(t.TempDir(), "missing-edata"),
			"--encyclopedia-canonical-output", filepath.Join(t.TempDir(), "encyclopedia"),
		},
		&stdout,
		io.Discard,
		nil,
		nil,
		func(string, ...string) ([]byte, error) {
			mediaCalls++
			return nil, io.ErrUnexpectedEOF
		},
	)
	if err == nil {
		t.Fatal("partial encyclopedia input was accepted")
	}
	if mediaCalls != 0 {
		t.Fatalf("partial encyclopedia input invoked media tools %d times", mediaCalls)
	}
}

func TestOwnedBuiltCanonicalToolWorksFromExternalDirectoryWithoutExecutable(t *testing.T) {
	ownedRoot := os.Getenv("REBELLION_ENCYCLOPEDIA_TEST_SOURCE")
	if ownedRoot == "" {
		t.Skip("set REBELLION_ENCYCLOPEDIA_TEST_SOURCE to an owned installation root")
	}
	ownedRoot, err := filepath.Abs(ownedRoot)
	if err != nil {
		t.Fatal(err)
	}
	profile := testEmbeddedCatalogProfile(t)
	flatSource := filepath.Join(t.TempDir(), "flat-source")
	if err := os.MkdirAll(flatSource, 0o700); err != nil {
		t.Fatal(err)
	}
	for _, source := range profile.Sources {
		if source.Kind == encyclopediaSourceEXE {
			continue
		}
		searchRoot := ownedRoot
		if source.Kind == encyclopediaSourceDAT {
			if gdata, found := findCaseInsensitiveDirectory(t, ownedRoot, "GData"); found {
				searchRoot = filepath.Join(ownedRoot, gdata)
			}
		}
		name, found := findCaseInsensitiveFile(t, searchRoot, source.Basename)
		if !found {
			t.Fatalf("owned source missing %s", source.Basename)
		}
		contents, err := os.ReadFile(filepath.Join(searchRoot, name))
		if err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(filepath.Join(flatSource, source.Basename), contents, 0o600); err != nil {
			t.Fatal(err)
		}
	}
	if _, err := os.Stat(filepath.Join(flatSource, "REBEXE.EXE")); !os.IsNotExist(err) {
		t.Fatalf("flattened source unexpectedly contains REBEXE.EXE: %v", err)
	}
	edataName, found := findCaseInsensitiveDirectory(t, ownedRoot, "EData")
	if !found {
		t.Fatal("owned source missing EData")
	}
	edataRoot := filepath.Join(ownedRoot, edataName)
	beforeFlatSource := snapshotTestTree(t, flatSource)
	beforeEData := snapshotTestTree(t, edataRoot)

	repoRoot, err := filepath.Abs(filepath.Join("..", ".."))
	if err != nil {
		t.Fatal(err)
	}
	binaryPath := filepath.Join(t.TempDir(), "stage-ui-assets")
	build := exec.Command("go", "build", "-o", binaryPath, "./tools/stage-ui-assets")
	build.Dir = repoRoot
	if output, err := build.CombinedOutput(); err != nil {
		t.Fatalf("build stage-ui-assets: %v\n%s", err, output)
	}
	externalWorkingDirectory := t.TempDir()
	outputRoot := filepath.Join(t.TempDir(), "canonical")
	run := func(extra ...string) []byte {
		t.Helper()
		args := []string{
			"--encyclopedia-canonical-only",
			"--source", flatSource,
			"--edata", edataRoot,
			"--encyclopedia-canonical-output", outputRoot,
		}
		args = append(args, extra...)
		command := exec.Command(binaryPath, args...)
		command.Dir = externalWorkingDirectory
		combined, err := command.CombinedOutput()
		if err != nil {
			t.Fatalf("built canonical tool: %v\n%s", err, combined)
		}
		return combined
	}
	run()
	first := snapshotTestTree(t, outputRoot)
	manifestBytes := readTestFile(t, filepath.Join(outputRoot, encyclopediaManifestFilename))
	manifest, err := parseEncyclopediaManifest(manifestBytes)
	if err != nil {
		t.Fatal(err)
	}
	report := readTestEncyclopediaResearchReport(t, outputRoot)
	if report.Artwork == nil {
		t.Fatal("canonical output omitted local artwork evidence")
	}
	if len(report.Artwork.StagedAssets) != 187 {
		t.Fatalf("staged artwork count = %d, want 187", len(report.Artwork.StagedAssets))
	}
	if _, runtimeFile := manifest.Files["assets/EDATA.192"]; runtimeFile {
		t.Fatal("deferred unreferenced EDATA.192 entered the runtime manifest")
	}
	if _, err := os.Stat(filepath.Join(outputRoot, "assets", "EDATA.192")); err != nil {
		t.Fatalf("unreferenced EDATA.192 was not retained as local evidence: %v", err)
	}
	run()
	if diff := diffTestTree(first, snapshotTestTree(t, outputRoot)); diff != "" {
		t.Fatalf("identical built-tool rerun changed canonical output:\n%s", diff)
	}
	run("--force")
	if diff := diffTestTree(first, snapshotTestTree(t, outputRoot)); diff != "" {
		t.Fatalf("forced identical built-tool rerun changed canonical output:\n%s", diff)
	}
	verify := exec.Command(binaryPath,
		"--encyclopedia-canonical-only", "--verify",
		"--source", filepath.Join(t.TempDir(), "missing-source"),
		"--edata", filepath.Join(t.TempDir(), "missing-edata"),
		"--encyclopedia-canonical-output", outputRoot,
	)
	verify.Dir = externalWorkingDirectory
	if combined, err := verify.CombinedOutput(); err != nil {
		t.Fatalf("source-free built-tool verify: %v\n%s", err, combined)
	}
	if diff := diffTestTree(beforeFlatSource, snapshotTestTree(t, flatSource)); diff != "" {
		t.Fatalf("built tool changed flattened source bytes:\n%s", diff)
	}
	if diff := diffTestTree(beforeEData, snapshotTestTree(t, edataRoot)); diff != "" {
		t.Fatalf("built tool changed owned EData bytes:\n%s", diff)
	}
}

func TestEncyclopediaReportOutputFlagRequiresFocusedMode(t *testing.T) {
	mediaCalls := 0
	err := runCLIWithMedia(
		[]string{"--encyclopedia-report-output", filepath.Join(t.TempDir(), "research")},
		io.Discard,
		io.Discard,
		nil,
		nil,
		func(string, ...string) ([]byte, error) {
			mediaCalls++
			return nil, io.ErrUnexpectedEOF
		},
	)
	if err == nil || !strings.Contains(err.Error(), "--encyclopedia-report-only") {
		t.Fatalf("cross-mode encyclopedia report output error = %v", err)
	}
	if mediaCalls != 0 {
		t.Fatalf("cross-mode rejection invoked media tools %d times", mediaCalls)
	}
}

func TestCanonicalEncyclopediaOutputFlagIsAvailableToFullStage(t *testing.T) {
	mediaCalls := 0
	err := runCLIWithMedia(
		[]string{"--encyclopedia-canonical-output", filepath.Join(t.TempDir(), "canonical")},
		io.Discard,
		io.Discard,
		nil,
		nil,
		func(string, ...string) ([]byte, error) {
			mediaCalls++
			return nil, io.ErrUnexpectedEOF
		},
	)
	if err == nil || !strings.Contains(err.Error(), "cutscene extraction requires ffmpeg") {
		t.Fatalf("full-stage encyclopedia output error = %v", err)
	}
	if mediaCalls != 1 {
		t.Fatalf("full stage media calls = %d, want 1 after absent encyclopedia warning", mediaCalls)
	}
}

func TestEncyclopediaEDataFlagIsAvailableToFullStage(t *testing.T) {
	mediaCalls := 0
	err := runCLIWithMedia(
		[]string{"--edata", t.TempDir()},
		io.Discard,
		io.Discard,
		nil,
		nil,
		func(string, ...string) ([]byte, error) {
			mediaCalls++
			return nil, io.ErrUnexpectedEOF
		},
	)
	if err == nil || !strings.Contains(err.Error(), "cutscene extraction requires ffmpeg") {
		t.Fatalf("full-stage --edata error = %v", err)
	}
	if mediaCalls != 1 {
		t.Fatalf("full stage media calls = %d, want 1 after absent encyclopedia warning", mediaCalls)
	}
}

func TestEncyclopediaFocusedModeUsesExplicitEDataOverride(t *testing.T) {
	source := t.TempDir()
	edata := t.TempDir()
	output := filepath.Join(t.TempDir(), "research")
	writeSyntheticEncyclopediaTextDLL(t, source, numericEncyclopediaResourceIdentifier(29), []byte("explicit EData\x00"))
	bmp := buildTestEncyclopediaBMP(t, testEncyclopediaBMPOptions{width: 2, height: 1, bitCount: 24})
	writeSyntheticEncyclopediaArtInputs(t, source, edata, map[int]string{0: "EDATA.009"}, map[string][]byte{"EDATA.009": bmp})
	if err := runCLIWithMedia(
		[]string{"--encyclopedia-report-only", "--source", source, "--edata", edata, "--encyclopedia-report-output", output},
		io.Discard,
		io.Discard,
		nil,
		nil,
		func(string, ...string) ([]byte, error) { return nil, io.ErrUnexpectedEOF },
	); err != nil {
		t.Fatal(err)
	}
	if got := readTestFile(t, filepath.Join(output, "assets", "EDATA.009")); !bytes.Equal(got, bmp) {
		t.Fatal("explicit --edata bytes were not staged")
	}
}

func TestEncyclopediaFocusedModeUsesDefaultResearchOutput(t *testing.T) {
	originalWorkingDirectory, err := os.Getwd()
	if err != nil {
		t.Fatal(err)
	}
	workingDirectory := t.TempDir()
	if err := os.Chdir(workingDirectory); err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() {
		if err := os.Chdir(originalWorkingDirectory); err != nil {
			t.Errorf("restore working directory: %v", err)
		}
	})
	source := t.TempDir()
	writeSyntheticEncyclopediaTextDLL(t, source, numericEncyclopediaResourceIdentifier(41), []byte("default output\x00"))
	bmp := buildTestEncyclopediaBMP(t, testEncyclopediaBMPOptions{width: 1, height: 1, bitCount: 24})
	writeSyntheticEncyclopediaArtInputs(t, source, filepath.Join(source, "EData"), map[int]string{0: "EDATA.001"}, map[string][]byte{"EDATA.001": bmp})
	if err := runCLIWithMedia(
		[]string{"--encyclopedia-report-only", "--source", source},
		io.Discard,
		io.Discard,
		nil,
		nil,
		func(string, ...string) ([]byte, error) { return nil, io.ErrUnexpectedEOF },
	); err != nil {
		t.Fatal(err)
	}
	want := filepath.Join(workingDirectory, "data", "base", "encyclopedia-research", encyclopediaResearchReportFilename)
	if _, err := os.Stat(want); err != nil {
		t.Fatalf("default encyclopedia report output: %v", err)
	}
}
