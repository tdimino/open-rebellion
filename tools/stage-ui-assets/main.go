package main

import (
	"flag"
	"fmt"
	"os"
)

func main() {
	if err := runCLI(os.Args[1:], os.Stdout, os.Stderr, uiDLLTargets); err != nil {
		if err == flag.ErrHelp {
			return
		}
		fmt.Fprintln(os.Stderr, "ERROR:", err)
		os.Exit(1)
	}
}
