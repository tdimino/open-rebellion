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
	edata := flags.String("edata", "", "original EData directory (default: source/EData)")
	stringsOutput := flags.String("strings-output", "data/base/textstra.json", "runtime text string JSON file")
	encyclopediaOutput := flags.String("encyclopedia-output", "data/base/encyclopedia/source.json", "runtime Encyclopedia source JSON file")
	cutsceneOutput := flags.String("cutscene-output", "assets/references", "parent of ref-videos and cutscene-frames outputs")
	encyclopediaReportOnly := flags.Bool("encyclopedia-report-only", false, "stage or verify only the encyclopedia research report")
	encyclopediaReportOutput := flags.String("encyclopedia-report-output", "data/base/encyclopedia-research", "encyclopedia research report output directory")
	force := flags.Bool("force", false, "replace staged assets whose contents differ")
	verifyOnly := flags.Bool("verify", false, "verify staged assets without reading source files")
	tactical3D := flags.Bool("tactical-3d", false, "also stage and verify original type-301/type-303 tactical resources")
	tactical3DOnly := flags.Bool("tactical-3d-only", false, "stage or verify only original type-301/type-303 tactical resources")
	tactical3DConvert := flags.Bool("tactical-3d-convert", false, "convert or verify the staged tactical resources")
	tactical3DAssimpOracle := flags.String("tactical-3d-assimp-oracle", "", "verify staged tactical meshes against this Assimp executable")
	encyclopediaOnly := flags.Bool("encyclopedia-only", false, "stage or verify only ENCYTEXT and ENCYBMAP source data")
	if err := flags.Parse(args); err != nil {
		return err
	}
	if flags.NArg() != 0 {
		return fmt.Errorf("unexpected arguments: %v", flags.Args())
	}
	encyclopediaReportOutputSet := false
	edataSet := false
	flags.Visit(func(selected *flag.Flag) {
		if selected.Name == "encyclopedia-report-output" {
			encyclopediaReportOutputSet = true
		}
		if selected.Name == "edata" {
			edataSet = true
		}
	})
	if encyclopediaReportOutputSet && !*encyclopediaReportOnly {
		return fmt.Errorf("--encyclopedia-report-output requires --encyclopedia-report-only")
	}
	if edataSet && !*encyclopediaReportOnly {
		return fmt.Errorf("--edata requires --encyclopedia-report-only")
	}
	selectedExclusiveModes := 0
	for _, selected := range []bool{*tactical3D, *tactical3DOnly, *tactical3DConvert, *tactical3DAssimpOracle != "", *encyclopediaOnly, *encyclopediaReportOnly} {
		if selected {
			selectedExclusiveModes++
		}
	}
	if selectedExclusiveModes > 1 {
		return fmt.Errorf("--tactical-3d, --tactical-3d-only, --tactical-3d-convert, --tactical-3d-assimp-oracle, --encyclopedia-only, and --encyclopedia-report-only are mutually exclusive")
	}
	if *encyclopediaOnly {
		if !*verifyOnly {
			if err := stageEncyclopediaSource(*sourceDir, *encyclopediaOutput, *force, stdout); err != nil {
				return err
			}
		}
		return verifyEncyclopediaSource(*encyclopediaOutput, stdout)
	}
	if *encyclopediaReportOnly {
		if !*verifyOnly {
			if *edata == "" {
				*edata = filepath.Join(*sourceDir, "EData")
			}
			if err := stageEncyclopediaReportWithRequest(encyclopediaReportStageRequest{
				SourceDir:   *sourceDir,
				EDataDir:    *edata,
				OutputDir:   *encyclopediaReportOutput,
				ModRoots:    []string{"mods"},
				Force:       *force,
				ImageLimits: defaultEncyclopediaImageLimits(),
				Log:         stdout,
			}); err != nil {
				return err
			}
		}
		return verifyEncyclopediaReport(*encyclopediaReportOutput, stdout)
	}
	if *tactical3DAssimpOracle != "" {
		return verifyTactical3DWithAssimp(*outputDir, *tactical3DAssimpOracle, stdout)
	}
	if *tactical3DConvert {
		if !*verifyOnly {
			if _, err := stageTactical3DRuntime(*outputDir, *force, stdout); err != nil {
				return err
			}
		}
		return verifyTactical3DRuntime(*outputDir, stdout)
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
