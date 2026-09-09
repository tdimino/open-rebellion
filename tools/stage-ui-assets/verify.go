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
	}
	return summary, nil
}
