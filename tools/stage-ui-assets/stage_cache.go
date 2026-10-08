package main

import (
	"encoding/json"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"reflect"
	"sort"
)

const (
	stageCacheVersion   = 1
	stageCacheByteLimit = 32 * 1024 * 1024
)

// Bump stageCacheVersion whenever full staging changes the expected bytes or
// inventory without changing its inputs or command-line configuration.
type stageCacheConfig struct {
	SourceDir          string `json:"source_dir"`
	OutputDir          string `json:"output_dir"`
	AudioOutput        string `json:"audio_output"`
	MData              string `json:"mdata"`
	StringsOutput      string `json:"strings_output"`
	EncyclopediaOutput string `json:"encyclopedia_output"`
	CutsceneOutput     string `json:"cutscene_output"`
	TargetProfile      string `json:"target_profile"`
	MovieProfile       string `json:"movie_profile"`
	Tactical3D         bool   `json:"tactical_3d"`
}

type stageCacheFile struct {
	Path   string `json:"path"`
	Size   int64  `json:"size"`
	SHA256 string `json:"sha256,omitempty"`
}

type stageCacheManifest struct {
	Version int              `json:"version"`
	Config  stageCacheConfig `json:"config"`
	Sources []stageCacheFile `json:"sources"`
	Outputs []stageCacheFile `json:"outputs"`
}

func newStageCacheConfig(
	sourceDir, outputDir, audioOutput, mdata, stringsOutput, encyclopediaOutput, cutsceneOutput string,
	targets []dllTarget,
	movieIDs []string,
	tactical3D bool,
) (stageCacheConfig, error) {
	targetBytes, err := json.Marshal(struct {
		Targets  []dllTarget       `json:"targets"`
		NamedIDs map[string]uint32 `json:"named_ids"`
	}{targets, namedBitmapIDs})
	if err != nil {
		return stageCacheConfig{}, err
	}
	movieBytes, err := json.Marshal(movieIDs)
	if err != nil {
		return stageCacheConfig{}, err
	}
	return stageCacheConfig{
		SourceDir:          filepath.Clean(sourceDir),
		OutputDir:          filepath.Clean(outputDir),
		AudioOutput:        filepath.Clean(audioOutput),
		MData:              filepath.Clean(mdata),
		StringsOutput:      filepath.Clean(stringsOutput),
		EncyclopediaOutput: filepath.Clean(encyclopediaOutput),
		CutsceneOutput:     filepath.Clean(cutsceneOutput),
		TargetProfile:      byteSHA256(targetBytes),
		MovieProfile:       byteSHA256(movieBytes),
		Tactical3D:         tactical3D,
	}, nil
}

func stageCachePath(config stageCacheConfig) string {
	return filepath.Join(filepath.Dir(config.StringsOutput), ".stage-ui-assets.json")
}

func stageSourcePaths(config stageCacheConfig, targets []dllTarget, movieIDs []string) []string {
	paths := make(map[string]struct{})
	add := func(path string) { paths[filepath.Clean(path)] = struct{}{} }
	for _, target := range targets {
		add(filepath.Join(config.SourceDir, target.Filename))
	}
	for _, voice := range voiceAudio {
		add(filepath.Join(config.SourceDir, voice.DLL))
	}
	for _, name := range []string{"COMMON.DLL", "TEXTSTRA.DLL", "ENCYTEXT.DLL", "ENCYBMAP.DLL"} {
		add(filepath.Join(config.SourceDir, name))
	}
	for id := 300; id <= 315; id++ {
		add(filepath.Join(config.MData, fmt.Sprintf("MDATA.%d", id)))
	}
	for _, id := range movieIDs {
		add(filepath.Join(config.MData, "MDATA."+id))
	}
	result := make([]string, 0, len(paths))
	for path := range paths {
		result = append(result, path)
	}
	sort.Strings(result)
	return result
}

func fingerprintStageSources(paths []string) ([]stageCacheFile, error) {
	files := make([]stageCacheFile, 0, len(paths))
	for _, path := range paths {
		info, err := os.Stat(path)
		if err != nil {
			return nil, err
		}
		if !info.Mode().IsRegular() {
			return nil, fmt.Errorf("staging source is not a regular file: %s", path)
		}
		hash, err := fileSHA256(path)
		if err != nil {
			return nil, err
		}
		files = append(files, stageCacheFile{Path: path, Size: info.Size(), SHA256: hash})
	}
	return files, nil
}

func stageCacheHit(config stageCacheConfig, targets []dllTarget, movieIDs []string) (bool, int, int) {
	data, err := os.ReadFile(stageCachePath(config))
	if err != nil || len(data) > stageCacheByteLimit {
		return false, 0, 0
	}
	var manifest stageCacheManifest
	if err := json.Unmarshal(data, &manifest); err != nil || manifest.Version != stageCacheVersion || manifest.Config != config || len(manifest.Sources) == 0 || len(manifest.Outputs) == 0 {
		return false, 0, 0
	}
	sources, err := fingerprintStageSources(stageSourcePaths(config, targets, movieIDs))
	if err != nil || !reflect.DeepEqual(sources, manifest.Sources) {
		return false, 0, 0
	}
	for _, output := range manifest.Outputs {
		info, err := os.Lstat(output.Path)
		if err != nil || !info.Mode().IsRegular() || info.Size() != output.Size {
			return false, 0, 0
		}
	}
	return true, len(manifest.Sources), len(manifest.Outputs)
}

func addStageOutput(outputs map[string]stageCacheFile, path string) error {
	path = filepath.Clean(path)
	info, err := os.Lstat(path)
	if err != nil {
		return err
	}
	if !info.Mode().IsRegular() {
		return fmt.Errorf("staged output is not a regular file: %s", path)
	}
	outputs[path] = stageCacheFile{Path: path, Size: info.Size()}
	return nil
}

func addStageOutputTree(outputs map[string]stageCacheFile, root string) error {
	return filepath.WalkDir(root, func(path string, entry fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if entry.IsDir() {
			return nil
		}
		return addStageOutput(outputs, path)
	})
}

func collectStageOutputs(config stageCacheConfig, targets []dllTarget, movieIDs []string) ([]stageCacheFile, error) {
	outputs := make(map[string]stageCacheFile)
	for _, target := range targets {
		if err := addStageOutputTree(outputs, filepath.Join(config.OutputDir, target.Directory, "BMP")); err != nil {
			return nil, err
		}
		if target.ExpectedType302 > 0 {
			if err := addStageOutputTree(outputs, filepath.Join(config.OutputDir, target.Directory, "TYPE302")); err != nil {
				return nil, err
			}
		}
	}
	if config.Tactical3D {
		if err := addStageOutputTree(outputs, filepath.Join(config.OutputDir, "tactical-dll", "TACTICAL3D")); err != nil {
			return nil, err
		}
	}
	if err := addStageOutputTree(outputs, config.AudioOutput); err != nil {
		return nil, err
	}
	for _, path := range []string{
		config.StringsOutput,
		config.StringsOutput + ".manifest.json",
		config.EncyclopediaOutput,
		config.EncyclopediaOutput + ".manifest.json",
	} {
		if err := addStageOutput(outputs, path); err != nil {
			return nil, err
		}
	}
	for _, id := range movieIDs {
		extraction := filepath.Join(config.CutsceneOutput, "cutscene-frames", id, "extraction.json")
		data, err := os.ReadFile(extraction)
		if err != nil {
			return nil, err
		}
		var manifest movieManifest
		if err := json.Unmarshal(data, &manifest); err != nil || manifest.Version != 1 || !validMovieMetadata(manifest.Metadata) {
			return nil, fmt.Errorf("invalid cutscene %s manifest", id)
		}
		if err := addStageOutput(outputs, extraction); err != nil {
			return nil, err
		}
		for _, path := range moviePaths(id, manifest.Metadata.FrameCount) {
			if err := addStageOutput(outputs, filepath.Join(config.CutsceneOutput, path)); err != nil {
				return nil, err
			}
		}
	}
	paths := make([]string, 0, len(outputs))
	for path := range outputs {
		paths = append(paths, path)
	}
	sort.Strings(paths)
	files := make([]stageCacheFile, 0, len(paths))
	for _, path := range paths {
		files = append(files, outputs[path])
	}
	return files, nil
}

func publishStageCache(config stageCacheConfig, targets []dllTarget, movieIDs []string) error {
	sources, err := fingerprintStageSources(stageSourcePaths(config, targets, movieIDs))
	if err != nil {
		return err
	}
	outputs, err := collectStageOutputs(config, targets, movieIDs)
	if err != nil {
		return err
	}
	manifest := stageCacheManifest{Version: stageCacheVersion, Config: config, Sources: sources, Outputs: outputs}
	data, err := json.MarshalIndent(manifest, "", "  ")
	if err != nil {
		return err
	}
	data = append(data, '\n')
	if len(data) > stageCacheByteLimit {
		return fmt.Errorf("staging cache exceeds %d-byte limit", stageCacheByteLimit)
	}
	path := stageCachePath(config)
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return err
	}
	return writeFileAtomically(path, data, 0o644)
}
