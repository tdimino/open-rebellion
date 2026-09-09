package main

import (
	"flag"
	"fmt"
	"io"
)

func runCLI(args []string, stdout, stderr io.Writer, targets []dllTarget) error {
	flags := flag.NewFlagSet("stage-ui-assets", flag.ContinueOnError)
	flags.SetOutput(stderr)
	sourceDir := flags.String("source", "data/base", "directory containing the original game DLLs")
	outputDir := flags.String("output", "data/base/ui", "runtime UI asset directory")
	force := flags.Bool("force", false, "replace staged BMPs whose contents differ")
	verifyOnly := flags.Bool("verify", false, "verify staged BMPs without extracting DLLs")
	if err := flags.Parse(args); err != nil {
		return err
	}
	if flags.NArg() != 0 {
		return fmt.Errorf("unexpected arguments: %v", flags.Args())
	}

	if !*verifyOnly {
		summary, err := stageTargets(*sourceDir, *outputDir, targets, namedBitmapIDs, *force, stdout)
		if err != nil {
			return err
		}
		fmt.Fprintf(stdout, "Staged %d BMPs from %d DLLs (%d written, %d unchanged)\n", summary.Resources, summary.DLLs, summary.Written, summary.Skipped)
	}

	verified, err := verifyTargets(*outputDir, targets, stdout)
	if err != nil {
		return err
	}
	fmt.Fprintf(stdout, "Verified %d BMPs across %d DLLs\n", verified.Resources, verified.DLLs)
	return nil
}
