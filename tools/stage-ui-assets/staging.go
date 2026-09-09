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
	}
	return summary, nil
}
