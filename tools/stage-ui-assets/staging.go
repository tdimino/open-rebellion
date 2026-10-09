package main

import (
	"bytes"
	"fmt"
	"io"
	"os"
	"path/filepath"
)

type stageResult struct {
	Written int
	Skipped int
}

type stageSummary struct {
	DLLs      int
	Resources int
	Written   int
	Skipped   int
}

func stageBitmapResources(resources []bitmapResource, outputDir string, force bool) (stageResult, error) {
	seen := make(map[uint32]uint32, len(resources))
	for _, resource := range resources {
		if language, exists := seen[resource.ID]; exists {
			return stageResult{}, fmt.Errorf("duplicate bitmap resource ID %d for languages %d and %d", resource.ID, language, resource.Language)
		}
		seen[resource.ID] = resource.Language
	}

	if err := os.MkdirAll(outputDir, 0o755); err != nil {
		return stageResult{}, fmt.Errorf("create output directory: %w", err)
	}

	result := stageResult{}
	for _, resource := range resources {
		bmp, err := dibToBMP(resource.DIB)
		if err != nil {
			return result, fmt.Errorf("convert bitmap resource %d: %w", resource.ID, err)
		}
		path := filepath.Join(outputDir, fmt.Sprintf("%d.bmp", resource.ID))
		if existing, err := os.ReadFile(path); err == nil {
			if bytes.Equal(existing, bmp) {
				result.Skipped++
				continue
			}
			if !force {
				return result, fmt.Errorf("bitmap resource %d already exists with different contents (use --force to replace it)", resource.ID)
			}
		} else if !os.IsNotExist(err) {
			return result, fmt.Errorf("read existing bitmap resource %d: %w", resource.ID, err)
		}
		if err := writeFileAtomically(path, bmp, 0o644); err != nil {
			return result, fmt.Errorf("write bitmap resource %d: %w", resource.ID, err)
		}
		result.Written++
	}
	return result, nil
}

func stageRawResources(resources []rawResource, outputDir string, force bool) (stageResult, error) {
	seen := make(map[uint32]uint32, len(resources))
	for _, resource := range resources {
		if language, exists := seen[resource.ID]; exists {
			return stageResult{}, fmt.Errorf("duplicate raw resource ID %d for languages %d and %d", resource.ID, language, resource.Language)
		}
		seen[resource.ID] = resource.Language
	}

	if err := os.MkdirAll(outputDir, 0o755); err != nil {
		return stageResult{}, fmt.Errorf("create output directory: %w", err)
	}

	result := stageResult{}
	for _, resource := range resources {
		path := filepath.Join(outputDir, fmt.Sprintf("%d.bin", resource.ID))
		if existing, err := os.ReadFile(path); err == nil {
			if bytes.Equal(existing, resource.Data) {
				result.Skipped++
				continue
			}
			if !force {
				return result, fmt.Errorf("raw resource %d already exists with different contents (use --force to replace it)", resource.ID)
			}
		} else if !os.IsNotExist(err) {
			return result, fmt.Errorf("read existing raw resource %d: %w", resource.ID, err)
		}
		if err := writeFileAtomically(path, resource.Data, 0o644); err != nil {
			return result, fmt.Errorf("write raw resource %d: %w", resource.ID, err)
		}
		result.Written++
	}
	return result, nil
}

// stageFile writes one staged file, leaving an identical one in place.
func stageFile(data []byte, path string, force bool) (stageResult, error) {
	if existing, err := os.ReadFile(path); err == nil {
		if bytes.Equal(existing, data) {
			return stageResult{Skipped: 1}, nil
		}
		if !force {
			return stageResult{}, fmt.Errorf("%s already exists with different contents (use --force to replace it)", path)
		}
	} else if !os.IsNotExist(err) {
		return stageResult{}, err
	}
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return stageResult{}, err
	}
	if err := writeFileAtomically(path, data, 0o644); err != nil {
		return stageResult{}, err
	}
	return stageResult{Written: 1}, nil
}

func writeFileAtomically(path string, data []byte, mode os.FileMode) error {
	dir := filepath.Dir(path)
	temp, err := os.CreateTemp(dir, "."+filepath.Base(path)+".tmp-*")
	if err != nil {
		return err
	}
	tempPath := temp.Name()
	defer os.Remove(tempPath)

	if err := temp.Chmod(mode); err != nil {
		temp.Close()
		return err
	}
	if _, err := temp.Write(data); err != nil {
		temp.Close()
		return err
	}
	if err := temp.Sync(); err != nil {
		temp.Close()
		return err
	}
	if err := temp.Close(); err != nil {
		return err
	}
	return os.Rename(tempPath, path)
}

func stageTargets(sourceDir, outputDir string, targets []dllTarget, namedIDs map[string]uint32, force bool, stdout io.Writer) (stageSummary, error) {
	summary := stageSummary{}
	for _, target := range targets {
		dllPath := filepath.Join(sourceDir, target.Filename)
		resources, err := readPEBitmapResources(dllPath, namedIDs)
		if err != nil {
			return summary, fmt.Errorf("%s: %w", target.Filename, err)
		}
		if len(resources) != target.Expected {
			return summary, fmt.Errorf("%s: found %d bitmap resources, expected %d", target.Filename, len(resources), target.Expected)
		}

		result, err := stageBitmapResources(resources, filepath.Join(outputDir, target.Directory, "BMP"), force)
		if err != nil {
			return summary, fmt.Errorf("%s: %w", target.Filename, err)
		}
		summary.DLLs++
		summary.Resources += len(resources)
		summary.Written += result.Written
		summary.Skipped += result.Skipped
		fmt.Fprintf(stdout, "%s: %d BMPs (%d written, %d unchanged)\n", target.Filename, len(resources), result.Written, result.Skipped)

		if target.ExpectedType302 > 0 {
			rawResources, err := readPERawResources(dllPath, rtAdvisorFrame)
			if err != nil {
				return summary, fmt.Errorf("%s: %w", target.Filename, err)
			}
			if len(rawResources) != target.ExpectedType302 {
				return summary, fmt.Errorf("%s: found %d type-302 resources, expected %d", target.Filename, len(rawResources), target.ExpectedType302)
			}
			rawResult, err := stageRawResources(rawResources, filepath.Join(outputDir, target.Directory, "TYPE302"), force)
			if err != nil {
				return summary, fmt.Errorf("%s: %w", target.Filename, err)
			}
			summary.Resources += len(rawResources)
			summary.Written += rawResult.Written
			summary.Skipped += rawResult.Skipped
			fmt.Fprintf(stdout, "%s: %d type-302 frames (%d written, %d unchanged)\n", target.Filename, len(rawResources), rawResult.Written, rawResult.Skipped)
		}
		if target.ExpectedRCData > 0 {
			scripts, err := readPERawResources(dllPath, rtRCData)
			if err != nil {
				return summary, fmt.Errorf("%s: %w", target.Filename, err)
			}
			if len(scripts) != target.ExpectedRCData {
				return summary, fmt.Errorf("%s: found %d RCDATA resources, expected %d", target.Filename, len(scripts), target.ExpectedRCData)
			}
			scriptResult, err := stageRawResources(scripts, filepath.Join(outputDir, target.Directory, "RCDATA"), force)
			if err != nil {
				return summary, fmt.Errorf("%s: %w", target.Filename, err)
			}
			summary.Resources += len(scripts)
			summary.Written += scriptResult.Written
			summary.Skipped += scriptResult.Skipped
			fmt.Fprintf(stdout, "%s: %d action scripts (%d written, %d unchanged)\n", target.Filename, len(scripts), scriptResult.Written, scriptResult.Skipped)
		}
		if target.ActionTable != "" {
			table, err := os.ReadFile(filepath.Join(sourceDir, "GData", target.ActionTable))
			if err != nil {
				return summary, fmt.Errorf("%s: %w", target.ActionTable, err)
			}
			tableResult, err := stageFile(table, filepath.Join(outputDir, target.Directory, "SPT", target.ActionTable), force)
			if err != nil {
				return summary, fmt.Errorf("%s: %w", target.ActionTable, err)
			}
			summary.Resources++
			summary.Written += tableResult.Written
			summary.Skipped += tableResult.Skipped
		}
		if target.ExpectedWaves > 0 {
			resources, err := readPEWaveResources(dllPath)
			if err != nil {
				return summary, fmt.Errorf("%s: %w", target.Filename, err)
			}
			waves, err := uniqueWaves(resources)
			if err != nil {
				return summary, fmt.Errorf("%s: %w", target.Filename, err)
			}
			if len(waves) != target.ExpectedWaves {
				return summary, fmt.Errorf("%s: found %d WAVE resources, expected %d", target.Filename, len(waves), target.ExpectedWaves)
			}
			var waveResult stageResult
			for id, data := range waves {
				result, err := stageFile(data, filepath.Join(outputDir, target.Directory, "WAVE", fmt.Sprintf("%d.wav", id)), force)
				if err != nil {
					return summary, fmt.Errorf("%s: %w", target.Filename, err)
				}
				waveResult.Written += result.Written
				waveResult.Skipped += result.Skipped
			}
			summary.Resources += len(waves)
			summary.Written += waveResult.Written
			summary.Skipped += waveResult.Skipped
			fmt.Fprintf(stdout, "%s: %d sounds (%d written, %d unchanged)\n", target.Filename, len(waves), waveResult.Written, waveResult.Skipped)
		}
	}
	return summary, nil
}
