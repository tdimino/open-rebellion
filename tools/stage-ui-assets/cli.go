package main

import (
	"flag"
	"fmt"
	"io"
	"path/filepath"
)

func runCLI(args []string, stdout, stderr io.Writer, targets []dllTarget) error {
	return runCLIWithMedia(args, stdout, stderr, targets, cutsceneIDs, runMedia)
}

func runCLIWithMedia(args []string, stdout, stderr io.Writer, targets []dllTarget, movieIDs []string, run mediaRunner) error {
	flags := flag.NewFlagSet("stage-ui-assets", flag.ContinueOnError)
	flags.SetOutput(stderr)
	sourceDir := flags.String("source", "data/base", "directory containing the original game DLLs")
	outputDir := flags.String("output", "data/base/ui", "runtime UI asset directory")
	audioOutput := flags.String("audio-output", "data/sounds", "runtime audio directory")
	mdata := flags.String("mdata", "", "original MDATA directory (default: source/MDATA)")
	stringsOutput := flags.String("strings-output", "data/base/textstra.json", "runtime text string JSON file")
	cutsceneOutput := flags.String("cutscene-output", "assets/references", "parent of ref-videos and cutscene-frames outputs")
	force := flags.Bool("force", false, "replace staged assets whose contents differ")
	verifyOnly := flags.Bool("verify", false, "verify staged assets without reading source files")
	tactical3D := flags.Bool("tactical-3d", false, "also stage and verify original type-301/type-303 tactical resources")
	tactical3DOnly := flags.Bool("tactical-3d-only", false, "stage or verify only original type-301/type-303 tactical resources")
	if err := flags.Parse(args); err != nil {
		return err
	}
	if flags.NArg() != 0 {
		return fmt.Errorf("unexpected arguments: %v", flags.Args())
	}
	if *tactical3D && *tactical3DOnly {
		return fmt.Errorf("--tactical-3d and --tactical-3d-only are mutually exclusive")
	}
	if *tactical3DOnly {
		if !*verifyOnly {
			if _, err := stageTactical3D(*sourceDir, *outputDir, *force, stdout); err != nil {
				return err
			}
		}
		return verifyTactical3D(*outputDir, tacticalMeshCount, tacticalTextureCount, stdout)
	}

	if !*verifyOnly {
		for _, tool := range []string{"ffmpeg", "ffprobe"} {
			if _, err := run(tool, "-version"); err != nil {
				return fmt.Errorf("cutscene extraction requires %s: %w", tool, err)
			}
		}
		summary, err := stageTargets(*sourceDir, *outputDir, targets, namedBitmapIDs, *force, stdout)
		if err != nil {
			return err
		}
		fmt.Fprintf(stdout, "Staged %d UI resources from %d DLLs (%d written, %d unchanged)\n", summary.Resources, summary.DLLs, summary.Written, summary.Skipped)
		if *tactical3D {
			if _, err := stageTactical3D(*sourceDir, *outputDir, *force, stdout); err != nil {
				return err
			}
		}
	}

	verified, err := verifyTargets(*outputDir, targets, stdout)
	if err != nil {
		return err
	}
	fmt.Fprintf(stdout, "Verified %d UI resources across %d DLLs\n", verified.Resources, verified.DLLs)
	if *tactical3D {
		if err := verifyTactical3D(*outputDir, tacticalMeshCount, tacticalTextureCount, stdout); err != nil {
			return err
		}
	}
	if !*verifyOnly {
		if *mdata == "" {
			*mdata = filepath.Join(*sourceDir, "MDATA")
		}
		if err := stageAudio(*sourceDir, *mdata, *audioOutput, *force, stdout); err != nil {
			return err
		}
	}
	if err := verifyAudio(*audioOutput, stdout); err != nil {
		return err
	}
	if !*verifyOnly {
		if err := stageStrings(*sourceDir, *stringsOutput, *force, stdout); err != nil {
			return err
		}
		if err := stageCutscenes(*mdata, *cutsceneOutput, *force, movieIDs, run, stdout); err != nil {
			return err
		}
	}
	if err := verifyStrings(*stringsOutput, stdout); err != nil {
		return err
	}
	if err := verifyCutscenes(*cutsceneOutput, movieIDs, stdout); err != nil {
		return err
	}
	return nil
}
