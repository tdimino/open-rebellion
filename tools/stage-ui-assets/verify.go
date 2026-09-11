package main

import (
	"fmt"
	"io"
	"os"
	"path/filepath"
)

type verifySummary struct {
	DLLs      int
	Resources int
}

func verifyTargets(outputDir string, targets []dllTarget, stdout io.Writer) (verifySummary, error) {
	summary := verifySummary{}
	for _, target := range targets {
		bmpDir := filepath.Join(outputDir, target.Directory, "BMP")
		entries, err := os.ReadDir(bmpDir)
		if err != nil {
			return summary, fmt.Errorf("%s: read staged directory: %w", target.Directory, err)
		}

		count := 0
		for _, entry := range entries {
			if entry.IsDir() || filepath.Ext(entry.Name()) != ".bmp" {
				continue
			}
			var resourceID uint32
			if _, err := fmt.Sscanf(entry.Name(), "%d.bmp", &resourceID); err != nil || entry.Name() != fmt.Sprintf("%d.bmp", resourceID) {
				return summary, fmt.Errorf("%s: non-numeric bitmap filename %q", target.Directory, entry.Name())
			}
			bmp, err := os.ReadFile(filepath.Join(bmpDir, entry.Name()))
			if err != nil {
				return summary, fmt.Errorf("%s/%s: %w", target.Directory, entry.Name(), err)
			}
			if err := validateBMP(bmp); err != nil {
				return summary, fmt.Errorf("%s/%s: %w", target.Directory, entry.Name(), err)
			}
			count++
		}
		if count != target.Expected {
			return summary, fmt.Errorf("%s: found %d staged BMPs, expected %d", target.Directory, count, target.Expected)
		}
		summary.DLLs++
		summary.Resources += count
		fmt.Fprintf(stdout, "%s: verified %d BMPs\n", target.Directory, count)

		if target.ExpectedType302 > 0 {
			frameDir := filepath.Join(outputDir, target.Directory, "TYPE302")
			entries, err := os.ReadDir(frameDir)
			if err != nil {
				return summary, fmt.Errorf("%s: read staged type-302 directory: %w", target.Directory, err)
			}
			frameCount := 0
			for _, entry := range entries {
				if entry.IsDir() || filepath.Ext(entry.Name()) != ".bin" {
					continue
				}
				var resourceID uint32
				if _, err := fmt.Sscanf(entry.Name(), "%d.bin", &resourceID); err != nil || entry.Name() != fmt.Sprintf("%d.bin", resourceID) {
					return summary, fmt.Errorf("%s: non-numeric type-302 filename %q", target.Directory, entry.Name())
				}
				data, err := os.ReadFile(filepath.Join(frameDir, entry.Name()))
				if err != nil {
					return summary, fmt.Errorf("%s/%s: %w", target.Directory, entry.Name(), err)
				}
				if err := validateType302(data); err != nil {
					return summary, fmt.Errorf("%s/%s: %w", target.Directory, entry.Name(), err)
				}
				frameCount++
			}
			if frameCount != target.ExpectedType302 {
				return summary, fmt.Errorf("%s: found %d staged type-302 frames, expected %d", target.Directory, frameCount, target.ExpectedType302)
			}
			summary.Resources += frameCount
			fmt.Fprintf(stdout, "%s: verified %d type-302 frames\n", target.Directory, frameCount)
		}
	}
	return summary, nil
}
