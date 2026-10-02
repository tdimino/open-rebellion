//go:build windows

package main

import (
	"errors"
	"fmt"
	"os"
	"syscall"
	"unsafe"
)

const (
	windowsLockfileFailImmediately = 0x00000001
	windowsLockfileExclusiveLock   = 0x00000002
	windowsStillActive             = 259
)

var (
	windowsKernel32         = syscall.NewLazyDLL("kernel32.dll")
	windowsLockFileEx       = windowsKernel32.NewProc("LockFileEx")
	windowsUnlockFileEx     = windowsKernel32.NewProc("UnlockFileEx")
	windowsLockViolation    = syscall.Errno(33)
	windowsSharingViolation = syscall.Errno(32)
	windowsInvalidParameter = syscall.Errno(87)
)

func lockEncyclopediaWriterGuard(guard *os.File) error {
	overlapped := syscall.Overlapped{}
	result, _, callErr := windowsLockFileEx.Call(
		guard.Fd(),
		windowsLockfileFailImmediately|windowsLockfileExclusiveLock,
		0,
		1,
		0,
		uintptr(unsafe.Pointer(&overlapped)),
	)
	if result != 0 {
		return nil
	}
	if errors.Is(callErr, windowsLockViolation) || errors.Is(callErr, windowsSharingViolation) {
		return errEncyclopediaWriterGuardBusy
	}
	if callErr == syscall.Errno(0) {
		return fmt.Errorf("LockFileEx failed")
	}
	return callErr
}

func unlockEncyclopediaWriterGuard(guard *os.File) error {
	overlapped := syscall.Overlapped{}
	result, _, callErr := windowsUnlockFileEx.Call(
		guard.Fd(),
		0,
		1,
		0,
		uintptr(unsafe.Pointer(&overlapped)),
	)
	if result != 0 {
		return nil
	}
	if callErr == syscall.Errno(0) {
		return fmt.Errorf("UnlockFileEx failed")
	}
	return callErr
}

func encyclopediaProcessAlive(pid int) (bool, error) {
	if uint64(pid) > uint64(^uint32(0)) {
		return false, fmt.Errorf("pid %d exceeds Windows process identifier range", pid)
	}
	handle, err := syscall.OpenProcess(syscall.PROCESS_QUERY_INFORMATION, false, uint32(pid))
	if err != nil {
		switch {
		case errors.Is(err, windowsInvalidParameter):
			return false, nil
		case errors.Is(err, syscall.ERROR_ACCESS_DENIED):
			return true, nil
		default:
			return false, err
		}
	}
	defer syscall.CloseHandle(handle)
	var exitCode uint32
	if err := syscall.GetExitCodeProcess(handle, &exitCode); err != nil {
		return false, err
	}
	return exitCode == windowsStillActive, nil
}
