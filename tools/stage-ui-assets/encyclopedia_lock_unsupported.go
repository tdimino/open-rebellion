//go:build aix || js || plan9 || solaris || wasip1

package main

import (
	"fmt"
	"os"
)

func lockEncyclopediaWriterGuard(_ *os.File) error {
	return fmt.Errorf("encyclopedia publication requires a supported process-locking platform")
}

func unlockEncyclopediaWriterGuard(_ *os.File) error {
	return nil
}

func encyclopediaProcessAlive(pid int) (bool, error) {
	return false, fmt.Errorf("cannot inspect writer pid %d on this platform", pid)
}
