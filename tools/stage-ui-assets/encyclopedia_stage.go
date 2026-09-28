package main

import (
	"crypto/rand"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
	"time"
)

const (
	encyclopediaTransactionVersion = 1
	encyclopediaCandidatePrefix    = "encyclopedia-stage-candidate-"
	encyclopediaBackupPrefix       = "encyclopedia-stage-backup-"
	encyclopediaStaleLockPrefix    = "encyclopedia-stage-stale-lock-"
	encyclopediaWriterGuardSuffix  = ".guard"
)

var errEncyclopediaWriterGuardBusy = errors.New("encyclopedia writer guard is locked")

type encyclopediaCandidateBuilder func(root string) error

type encyclopediaOwnedInventory struct {
	Files []string
}

type encyclopediaOwnedInspector func(root string) (encyclopediaOwnedInventory, error)

type encyclopediaDirectoryRequest struct {
	OutputDir       string
	SourceRoots     []string
	ModRoots        []string
	Force           bool
	BuildCandidate  encyclopediaCandidateBuilder
	InspectOwned    encyclopediaOwnedInspector
	RecoveryCommand string
	Log             io.Writer
}

type encyclopediaPublicationResult struct {
	Changed   bool
	Recovered bool
}

type encyclopediaTransactionPhase string

const (
	encyclopediaPhaseCandidateValidated  encyclopediaTransactionPhase = "candidate_validated"
	encyclopediaPhaseDestinationBackedUp encyclopediaTransactionPhase = "destination_backed_up"
	encyclopediaPhaseCandidatePublished  encyclopediaTransactionPhase = "candidate_published"
	encyclopediaPhaseCleanupPending      encyclopediaTransactionPhase = "cleanup_pending"
)

type encyclopediaTransaction struct {
	Version        int                          `json:"version"`
	OutputBase     string                       `json:"output_base"`
	CandidateBase  string                       `json:"candidate_base"`
	BackupBase     string                       `json:"backup_base"`
	HadDestination bool                         `json:"had_destination"`
	Phase          encyclopediaTransactionPhase `json:"phase"`
}

type encyclopediaWriterMarker struct {
	Version   int    `json:"version"`
	PID       int    `json:"pid"`
	Token     string `json:"token"`
	CreatedAt string `json:"created_at"`
}

type encyclopediaPublicationOps struct {
	rename       func(string, string) error
	remove       func(string) error
	removeAll    func(string) error
	writeJournal func(string, encyclopediaTransaction) error
}

func defaultEncyclopediaPublicationOps() encyclopediaPublicationOps {
	return encyclopediaPublicationOps{
		rename:    os.Rename,
		remove:    os.Remove,
		removeAll: os.RemoveAll,
		writeJournal: func(path string, transaction encyclopediaTransaction) error {
			return writeEncyclopediaTransaction(path, transaction)
		},
	}
}

type encyclopediaPublicationPaths struct {
	output  string
	parent  string
	base    string
	marker  string
	journal string
}

type encyclopediaWriterLease struct {
	path  string
	token string
	guard *os.File
}

func stageEncyclopediaDirectory(request encyclopediaDirectoryRequest) (encyclopediaPublicationResult, error) {
	return stageEncyclopediaDirectoryWithOps(request, defaultEncyclopediaPublicationOps())
}

func stageEncyclopediaDirectoryWithOps(request encyclopediaDirectoryRequest, ops encyclopediaPublicationOps) (encyclopediaPublicationResult, error) {
	paths, err := prepareEncyclopediaPublicationPaths(request.OutputDir)
	if err != nil {
		return encyclopediaPublicationResult{}, err
	}
	if err := validateEncyclopediaRootCollisions(paths.output, request.SourceRoots, request.ModRoots); err != nil {
		return encyclopediaPublicationResult{}, err
	}
	if request.BuildCandidate == nil {
		return encyclopediaPublicationResult{}, fmt.Errorf("encyclopedia publisher requires a candidate builder")
	}
	if request.InspectOwned == nil {
		return encyclopediaPublicationResult{}, fmt.Errorf("encyclopedia publisher requires an ownership inspector")
	}
	if request.Log == nil {
		request.Log = io.Discard
	}
	if err := os.MkdirAll(paths.parent, 0o755); err != nil {
		return encyclopediaPublicationResult{}, fmt.Errorf("create encyclopedia output parent: %w", err)
	}
	lease, err := acquireEncyclopediaWriter(paths.marker)
	if err != nil {
		return encyclopediaPublicationResult{}, err
	}
	defer releaseEncyclopediaWriter(lease)

	recovered, err := recoverEncyclopediaPublication(paths, request.InspectOwned, request.RecoveryCommand, ops)
	if err != nil {
		return encyclopediaPublicationResult{}, err
	}

	candidate, err := os.MkdirTemp(paths.parent, "."+paths.base+"-"+encyclopediaCandidatePrefix)
	if err != nil {
		return encyclopediaPublicationResult{Recovered: recovered}, fmt.Errorf("create sibling encyclopedia candidate: %w", err)
	}
	candidateOwnedByRun := true
	defer func() {
		if candidateOwnedByRun {
			_ = os.RemoveAll(candidate)
		}
	}()
	if err := request.BuildCandidate(candidate); err != nil {
		return encyclopediaPublicationResult{Recovered: recovered}, fmt.Errorf("build encyclopedia candidate: %w", err)
	}
	candidateInventory, err := inspectEncyclopediaOwnedDirectory(candidate, request.InspectOwned)
	if err != nil {
		return encyclopediaPublicationResult{Recovered: recovered}, fmt.Errorf("validate encyclopedia candidate: %w", err)
	}

	destinationExists, err := encyclopediaDirectoryExists(paths.output)
	if err != nil {
		return encyclopediaPublicationResult{Recovered: recovered}, err
	}
	var destinationInventory encyclopediaOwnedInventory
	if destinationExists {
		destinationInventory, err = inspectEncyclopediaOwnedDirectory(paths.output, request.InspectOwned)
		if err != nil {
			return encyclopediaPublicationResult{Recovered: recovered}, fmt.Errorf("output_not_owned: %w", err)
		}
		same, err := encyclopediaDirectoriesEqual(paths.output, destinationInventory, candidate, candidateInventory)
		if err != nil {
			return encyclopediaPublicationResult{Recovered: recovered}, fmt.Errorf("compare encyclopedia output: %w", err)
		}
		if same {
			return encyclopediaPublicationResult{Changed: false, Recovered: recovered}, nil
		}
		if !request.Force {
			return encyclopediaPublicationResult{Recovered: recovered}, fmt.Errorf("output_changed: generated encyclopedia output differs (use --force to replace it)")
		}
	}

	backupToken, err := randomEncyclopediaToken()
	if err != nil {
		return encyclopediaPublicationResult{Recovered: recovered}, fmt.Errorf("create encyclopedia backup identity: %w", err)
	}
	transaction := encyclopediaTransaction{
		Version:        encyclopediaTransactionVersion,
		OutputBase:     paths.base,
		CandidateBase:  filepath.Base(candidate),
		BackupBase:     "." + paths.base + "-" + encyclopediaBackupPrefix + backupToken,
		HadDestination: destinationExists,
		Phase:          encyclopediaPhaseCandidateValidated,
	}
	backup := filepath.Join(paths.parent, transaction.BackupBase)
	if err := ops.writeJournal(paths.journal, transaction); err != nil {
		return encyclopediaPublicationResult{Recovered: recovered}, fmt.Errorf("record candidate transaction: %w", err)
	}

	oldMoved := false
	newInstalled := false
	rollback := func(cause error) (encyclopediaPublicationResult, error) {
		recoveryErr := rollbackEncyclopediaPublication(paths, candidate, backup, oldMoved, newInstalled, ops)
		if recoveryErr != nil {
			candidateOwnedByRun = false
			return encyclopediaPublicationResult{Recovered: recovered}, fmt.Errorf("publication failed: %v; automatic recovery failed: %w; recover with: %s", cause, recoveryErr, encyclopediaRecoveryCommand(request.RecoveryCommand, paths.output))
		}
		return encyclopediaPublicationResult{Recovered: recovered}, cause
	}

	if destinationExists {
		if err := ops.rename(paths.output, backup); err != nil {
			return rollback(fmt.Errorf("move prior encyclopedia output to backup: %w", err))
		}
		oldMoved = true
	}
	transaction.Phase = encyclopediaPhaseDestinationBackedUp
	if err := ops.writeJournal(paths.journal, transaction); err != nil {
		return rollback(fmt.Errorf("record backed-up transaction: %w", err))
	}
	if err := ops.rename(candidate, paths.output); err != nil {
		return rollback(fmt.Errorf("publish encyclopedia candidate: %w", err))
	}
	candidateOwnedByRun = false
	newInstalled = true
	transaction.Phase = encyclopediaPhaseCandidatePublished
	if err := ops.writeJournal(paths.journal, transaction); err != nil {
		return rollback(fmt.Errorf("record published transaction: %w", err))
	}
	if _, err := inspectEncyclopediaOwnedDirectory(paths.output, request.InspectOwned); err != nil {
		return rollback(fmt.Errorf("validate published encyclopedia output: %w", err))
	}
	if oldMoved {
		if _, err := inspectEncyclopediaOwnedDirectory(backup, request.InspectOwned); err != nil {
			return rollback(fmt.Errorf("validate prior encyclopedia backup before cleanup: %w", err))
		}
	}
	transaction.Phase = encyclopediaPhaseCleanupPending
	if err := ops.writeJournal(paths.journal, transaction); err != nil {
		return rollback(fmt.Errorf("record cleanup transaction: %w", err))
	}
	if oldMoved {
		if err := ops.removeAll(backup); err != nil {
			return encyclopediaPublicationResult{Changed: true, Recovered: recovered}, fmt.Errorf("published encyclopedia output but backup cleanup failed: %w; recover with: %s", err, encyclopediaRecoveryCommand(request.RecoveryCommand, paths.output))
		}
	}
	if err := ops.remove(paths.journal); err != nil && !os.IsNotExist(err) {
		return encyclopediaPublicationResult{Changed: true, Recovered: recovered}, fmt.Errorf("published encyclopedia output but transaction cleanup failed: %w; recover with: %s", err, encyclopediaRecoveryCommand(request.RecoveryCommand, paths.output))
	}
	if err := syncEncyclopediaDirectory(paths.parent); err != nil {
		return encyclopediaPublicationResult{Changed: true, Recovered: recovered}, fmt.Errorf("sync encyclopedia output parent: %w", err)
	}
	fmt.Fprintf(request.Log, "Published encyclopedia directory %s\n", paths.output)
	return encyclopediaPublicationResult{Changed: true, Recovered: recovered}, nil
}

func rollbackEncyclopediaPublication(paths encyclopediaPublicationPaths, candidate, backup string, oldMoved, newInstalled bool, ops encyclopediaPublicationOps) error {
	var recovery []error
	if newInstalled {
		if err := ops.rename(paths.output, candidate); err != nil {
			recovery = append(recovery, fmt.Errorf("retain failed candidate: %w", err))
			return errors.Join(recovery...)
		}
	}
	if oldMoved {
		if err := ops.rename(backup, paths.output); err != nil {
			recovery = append(recovery, fmt.Errorf("restore prior output: %w", err))
		}
	}
	if len(recovery) != 0 {
		return errors.Join(recovery...)
	}
	if err := ops.removeAll(candidate); err != nil && !os.IsNotExist(err) {
		recovery = append(recovery, fmt.Errorf("remove failed candidate: %w", err))
	}
	if err := ops.remove(paths.journal); err != nil && !os.IsNotExist(err) {
		recovery = append(recovery, fmt.Errorf("remove transaction journal: %w", err))
	}
	if len(recovery) != 0 {
		return errors.Join(recovery...)
	}
	return syncEncyclopediaDirectory(paths.parent)
}

func prepareEncyclopediaPublicationPaths(output string) (encyclopediaPublicationPaths, error) {
	resolved, err := resolveEncyclopediaOutputPath(output)
	if err != nil {
		return encyclopediaPublicationPaths{}, err
	}
	base := filepath.Base(resolved)
	if base == "." || base == string(filepath.Separator) || base == "" {
		return encyclopediaPublicationPaths{}, fmt.Errorf("path_collision: encyclopedia output must name a directory")
	}
	parent := filepath.Dir(resolved)
	return encyclopediaPublicationPaths{
		output:  resolved,
		parent:  parent,
		base:    base,
		marker:  filepath.Join(parent, "."+base+".encyclopedia-stage.lock"),
		journal: filepath.Join(parent, "."+base+".encyclopedia-stage.transaction.json"),
	}, nil
}

func writeEncyclopediaTransaction(path string, transaction encyclopediaTransaction) error {
	encoded, err := json.MarshalIndent(transaction, "", "  ")
	if err != nil {
		return err
	}
	if err := writeFileAtomically(path, append(encoded, '\n'), 0o600); err != nil {
		return err
	}
	return syncEncyclopediaDirectory(filepath.Dir(path))
}

func acquireEncyclopediaWriter(path string) (encyclopediaWriterLease, error) {
	guard, err := acquireEncyclopediaWriterGuard(path + encyclopediaWriterGuardSuffix)
	if err != nil {
		if errors.Is(err, errEncyclopediaWriterGuardBusy) {
			return encyclopediaWriterLease{}, fmt.Errorf("stage_busy: encyclopedia writer owns %s", path)
		}
		return encyclopediaWriterLease{}, fmt.Errorf("acquire encyclopedia writer guard: %w", err)
	}
	keepGuard := false
	defer func() {
		if !keepGuard {
			releaseEncyclopediaWriterGuard(guard)
		}
	}()

	for attempts := 0; attempts < 4; attempts++ {
		token, err := randomEncyclopediaToken()
		if err != nil {
			return encyclopediaWriterLease{}, fmt.Errorf("create encyclopedia writer identity: %w", err)
		}
		marker := encyclopediaWriterMarker{
			Version:   encyclopediaTransactionVersion,
			PID:       os.Getpid(),
			Token:     token,
			CreatedAt: time.Now().UTC().Format(time.RFC3339Nano),
		}
		encoded, err := json.Marshal(marker)
		if err != nil {
			return encyclopediaWriterLease{}, err
		}
		file, err := os.OpenFile(path, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0o600)
		if err == nil {
			if _, err = file.Write(append(encoded, '\n')); err == nil {
				err = file.Sync()
			}
			closeErr := file.Close()
			if err == nil {
				err = closeErr
			}
			if err != nil {
				_ = os.Remove(path)
				return encyclopediaWriterLease{}, fmt.Errorf("write encyclopedia writer marker: %w", err)
			}
			keepGuard = true
			return encyclopediaWriterLease{path: path, token: token, guard: guard}, nil
		}
		if !os.IsExist(err) {
			return encyclopediaWriterLease{}, fmt.Errorf("create encyclopedia writer marker: %w", err)
		}
		existing, err := readEncyclopediaWriterMarker(path)
		if err != nil {
			return encyclopediaWriterLease{}, fmt.Errorf("stage_busy: writer marker cannot be safely inspected: %w", err)
		}
		alive, err := encyclopediaProcessAlive(existing.PID)
		if err != nil {
			return encyclopediaWriterLease{}, fmt.Errorf("stage_busy: cannot rule out writer pid %d: %w", existing.PID, err)
		}
		if alive {
			return encyclopediaWriterLease{}, fmt.Errorf("stage_busy: encyclopedia writer pid %d owns %s", existing.PID, path)
		}
		staleToken, err := randomEncyclopediaToken()
		if err != nil {
			return encyclopediaWriterLease{}, fmt.Errorf("create stale writer identity: %w", err)
		}
		stale := filepath.Join(filepath.Dir(path), "."+filepath.Base(path)+"-"+encyclopediaStaleLockPrefix+staleToken)
		if err := os.Rename(path, stale); err != nil {
			if os.IsNotExist(err) {
				continue
			}
			return encyclopediaWriterLease{}, fmt.Errorf("stage_busy: quarantine stale writer marker: %w", err)
		}
		_ = os.Remove(stale)
	}
	return encyclopediaWriterLease{}, fmt.Errorf("stage_busy: writer marker changed repeatedly")
}

func releaseEncyclopediaWriter(lease encyclopediaWriterLease) {
	marker, err := readEncyclopediaWriterMarker(lease.path)
	if err == nil && marker.Token == lease.token {
		_ = os.Remove(lease.path)
	}
	releaseEncyclopediaWriterGuard(lease.guard)
}

func acquireEncyclopediaWriterGuard(path string) (*os.File, error) {
	// The guard inode is intentionally persistent. Removing a lock file after
	// unlocking lets a contender lock the old inode while another process
	// creates and locks a replacement at the same path.
	guard, err := os.OpenFile(path, os.O_RDWR|os.O_CREATE|os.O_EXCL, 0o600)
	if err != nil {
		if !os.IsExist(err) {
			return nil, err
		}
		guard, err = os.OpenFile(path, os.O_RDWR, 0)
		if err != nil {
			return nil, err
		}
	}
	closeWithError := func(cause error) (*os.File, error) {
		_ = guard.Close()
		return nil, cause
	}
	pathInfo, err := os.Lstat(path)
	if err != nil {
		return closeWithError(err)
	}
	guardInfo, err := guard.Stat()
	if err != nil {
		return closeWithError(err)
	}
	if pathInfo.Mode()&os.ModeSymlink != 0 || !pathInfo.Mode().IsRegular() || !os.SameFile(pathInfo, guardInfo) {
		return closeWithError(fmt.Errorf("unsafe encyclopedia writer guard %s", path))
	}
	if err := lockEncyclopediaWriterGuard(guard); err != nil {
		return closeWithError(err)
	}
	return guard, nil
}

func releaseEncyclopediaWriterGuard(guard *os.File) {
	if guard == nil {
		return
	}
	_ = unlockEncyclopediaWriterGuard(guard)
	_ = guard.Close()
}

func readEncyclopediaWriterMarker(path string) (encyclopediaWriterMarker, error) {
	data, err := readEncyclopediaRegularFile(path, 64<<10)
	if err != nil {
		return encyclopediaWriterMarker{}, err
	}
	var marker encyclopediaWriterMarker
	if err := json.Unmarshal(data, &marker); err != nil {
		return encyclopediaWriterMarker{}, err
	}
	if marker.Version != encyclopediaTransactionVersion || marker.PID <= 0 || marker.Token == "" {
		return encyclopediaWriterMarker{}, fmt.Errorf("invalid writer marker")
	}
	return marker, nil
}

func randomEncyclopediaToken() (string, error) {
	var token [8]byte
	if _, err := rand.Read(token[:]); err != nil {
		return "", err
	}
	return hex.EncodeToString(token[:]), nil
}

func encyclopediaRecoveryCommand(command, output string) string {
	if strings.TrimSpace(command) != "" {
		return command
	}
	return fmt.Sprintf("rerun staging with --encyclopedia-output %q", output)
}

func syncEncyclopediaDirectory(path string) error {
	directory, err := os.Open(path)
	if err != nil {
		return err
	}
	defer directory.Close()
	return directory.Sync()
}

func encyclopediaDirectoryExists(path string) (bool, error) {
	info, err := os.Lstat(path)
	if os.IsNotExist(err) {
		return false, nil
	}
	if err != nil {
		return false, fmt.Errorf("inspect encyclopedia output: %w", err)
	}
	if info.Mode()&os.ModeSymlink != 0 {
		return false, fmt.Errorf("unsafe symlink at encyclopedia output %s", path)
	}
	if !info.IsDir() {
		return false, fmt.Errorf("encyclopedia output is not a directory: %s", path)
	}
	return true, nil
}

func recoverEncyclopediaPublication(paths encyclopediaPublicationPaths, inspect encyclopediaOwnedInspector, recoveryCommand string, ops encyclopediaPublicationOps) (bool, error) {
	transaction, exists, err := readEncyclopediaTransaction(paths)
	if err != nil {
		return false, fmt.Errorf("interrupted encyclopedia transaction cannot be recovered: %w; recover with: %s", err, encyclopediaRecoveryCommand(recoveryCommand, paths.output))
	}
	if !exists {
		return false, nil
	}
	candidate := filepath.Join(paths.parent, transaction.CandidateBase)
	backup := filepath.Join(paths.parent, transaction.BackupBase)
	outputState := inspectEncyclopediaDirectoryState(paths.output, inspect)
	candidateState := inspectEncyclopediaDirectoryState(candidate, inspect)
	backupState := inspectEncyclopediaDirectoryState(backup, inspect)
	for name, state := range map[string]encyclopediaDirectoryState{"output": outputState, "candidate": candidateState, "backup": backupState} {
		if state.err != nil {
			return false, fmt.Errorf("interrupted encyclopedia transaction has invalid %s: %w; recover with: %s", name, state.err, encyclopediaRecoveryCommand(recoveryCommand, paths.output))
		}
	}

	removeValidated := func(path string, state encyclopediaDirectoryState) error {
		if !state.exists || state.err != nil {
			return fmt.Errorf("refuse cleanup of unvalidated directory %s", path)
		}
		return ops.removeAll(path)
	}
	finish := func() (bool, error) {
		if err := ops.remove(paths.journal); err != nil && !os.IsNotExist(err) {
			return false, err
		}
		return true, syncEncyclopediaDirectory(paths.parent)
	}
	restoreBackup := func() (bool, error) {
		if outputState.exists {
			return false, fmt.Errorf("ambiguous recovery: output and backup both exist")
		}
		if !backupState.exists {
			return false, fmt.Errorf("ambiguous recovery: no validated prior backup")
		}
		if err := ops.rename(backup, paths.output); err != nil {
			return false, err
		}
		if candidateState.exists {
			if err := removeValidated(candidate, candidateState); err != nil {
				return false, err
			}
		}
		return finish()
	}
	rollbackPublished := func() (bool, error) {
		if !outputState.exists || !backupState.exists || candidateState.exists {
			return false, fmt.Errorf("ambiguous recovery: cannot identify published candidate and prior backup")
		}
		if err := ops.rename(paths.output, candidate); err != nil {
			return false, err
		}
		if err := ops.rename(backup, paths.output); err != nil {
			_ = ops.rename(candidate, paths.output)
			return false, err
		}
		candidateState.exists = true
		if err := removeValidated(candidate, candidateState); err != nil {
			return false, err
		}
		return finish()
	}

	switch transaction.Phase {
	case encyclopediaPhaseCandidateValidated:
		if transaction.HadDestination {
			switch {
			case outputState.exists && !candidateState.exists && !backupState.exists:
				return finish()
			case outputState.exists && candidateState.exists && !backupState.exists:
				if err := removeValidated(candidate, candidateState); err != nil {
					return false, err
				}
				return finish()
			case !outputState.exists && backupState.exists:
				return restoreBackup()
			case outputState.exists && backupState.exists && !candidateState.exists:
				return rollbackPublished()
			default:
				return false, fmt.Errorf("ambiguous recovery at %s; recover with: %s", transaction.Phase, encyclopediaRecoveryCommand(recoveryCommand, paths.output))
			}
		}
		if !outputState.exists && candidateState.exists && !backupState.exists {
			if err := ops.rename(candidate, paths.output); err != nil {
				return false, err
			}
			return finish()
		}
		if outputState.exists && !candidateState.exists && !backupState.exists {
			return finish()
		}
	case encyclopediaPhaseDestinationBackedUp:
		if transaction.HadDestination && backupState.exists {
			if !outputState.exists {
				return restoreBackup()
			}
			if outputState.exists && !candidateState.exists {
				return rollbackPublished()
			}
		}
		if transaction.HadDestination && outputState.exists && !candidateState.exists && !backupState.exists {
			return finish()
		}
		if transaction.HadDestination && outputState.exists && candidateState.exists && !backupState.exists {
			if err := removeValidated(candidate, candidateState); err != nil {
				return false, err
			}
			return finish()
		}
		if !transaction.HadDestination && !backupState.exists {
			switch {
			case outputState.exists && !candidateState.exists:
				return finish()
			case !outputState.exists && candidateState.exists:
				if err := ops.rename(candidate, paths.output); err != nil {
					return false, err
				}
				return finish()
			case !outputState.exists && !candidateState.exists:
				return finish()
			}
		}
	case encyclopediaPhaseCandidatePublished, encyclopediaPhaseCleanupPending:
		if !transaction.HadDestination && backupState.exists {
			return false, fmt.Errorf("ambiguous recovery: initial publication unexpectedly has a prior backup")
		}
		if transaction.HadDestination && outputState.exists && candidateState.exists && !backupState.exists {
			if err := removeValidated(candidate, candidateState); err != nil {
				return false, err
			}
			return finish()
		}
		if outputState.exists && !candidateState.exists {
			if backupState.exists {
				if err := removeValidated(backup, backupState); err != nil {
					return false, err
				}
			}
			return finish()
		}
		if !outputState.exists && transaction.HadDestination && backupState.exists {
			return restoreBackup()
		}
		if !outputState.exists && !transaction.HadDestination && candidateState.exists && !backupState.exists {
			if err := ops.rename(candidate, paths.output); err != nil {
				return false, err
			}
			return finish()
		}
	default:
		return false, fmt.Errorf("unknown encyclopedia transaction phase %q", transaction.Phase)
	}
	return false, fmt.Errorf("ambiguous recovery at %s; recover with: %s", transaction.Phase, encyclopediaRecoveryCommand(recoveryCommand, paths.output))
}

type encyclopediaDirectoryState struct {
	exists    bool
	inventory encyclopediaOwnedInventory
	err       error
}

func inspectEncyclopediaDirectoryState(path string, inspect encyclopediaOwnedInspector) encyclopediaDirectoryState {
	exists, err := encyclopediaDirectoryExists(path)
	if err != nil {
		return encyclopediaDirectoryState{exists: true, err: err}
	}
	if !exists {
		return encyclopediaDirectoryState{}
	}
	inventory, err := inspectEncyclopediaOwnedDirectory(path, inspect)
	return encyclopediaDirectoryState{exists: true, inventory: inventory, err: err}
}

func readEncyclopediaTransaction(paths encyclopediaPublicationPaths) (encyclopediaTransaction, bool, error) {
	data, err := readEncyclopediaRegularFile(paths.journal, 64<<10)
	if os.IsNotExist(err) {
		return encyclopediaTransaction{}, false, nil
	}
	if err != nil {
		return encyclopediaTransaction{}, false, err
	}
	var transaction encyclopediaTransaction
	if err := json.Unmarshal(data, &transaction); err != nil {
		return encyclopediaTransaction{}, false, err
	}
	if transaction.Version != encyclopediaTransactionVersion || transaction.OutputBase != paths.base {
		return encyclopediaTransaction{}, false, fmt.Errorf("transaction does not match output")
	}
	if !validEncyclopediaSidecarBase(transaction.CandidateBase, paths.base, encyclopediaCandidatePrefix) ||
		!validEncyclopediaSidecarBase(transaction.BackupBase, paths.base, encyclopediaBackupPrefix) {
		return encyclopediaTransaction{}, false, fmt.Errorf("transaction paths escape the output parent")
	}
	return transaction, true, nil
}

func validEncyclopediaSidecarBase(name, outputBase, prefix string) bool {
	if filepath.Base(name) != name || name == "." || strings.ContainsAny(name, `/\\`) {
		return false
	}
	return strings.HasPrefix(name, "."+outputBase+"-"+prefix) && len(name) > len("."+outputBase+"-"+prefix)
}
