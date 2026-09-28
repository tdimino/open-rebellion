package main

import (
	"bytes"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

func verifyEncyclopediaDirectory(output string, inspect encyclopediaOwnedInspector, recoveryCommand string, log io.Writer) error {
	paths, err := prepareEncyclopediaPublicationPaths(output)
	if err != nil {
		return err
	}
	if inspect == nil {
		return fmt.Errorf("encyclopedia verifier requires an ownership inspector")
	}
	if log == nil {
		log = io.Discard
	}
	if info, err := os.Lstat(paths.marker); err == nil {
		if info.Mode()&os.ModeSymlink != 0 || !info.Mode().IsRegular() {
			return fmt.Errorf("active encyclopedia publication has unsafe writer marker %s; recover with: %s", paths.marker, encyclopediaRecoveryCommand(recoveryCommand, paths.output))
		}
		marker, markerErr := readEncyclopediaWriterMarker(paths.marker)
		if markerErr != nil {
			return fmt.Errorf("active encyclopedia publication has unreadable writer marker: %v; recover with: %s", markerErr, encyclopediaRecoveryCommand(recoveryCommand, paths.output))
		}
		return fmt.Errorf("active encyclopedia publication by pid %d; recover with: %s", marker.PID, encyclopediaRecoveryCommand(recoveryCommand, paths.output))
	} else if !os.IsNotExist(err) {
		return fmt.Errorf("inspect encyclopedia writer marker: %w", err)
	}
	transaction, exists, err := readEncyclopediaTransaction(paths)
	if err != nil {
		return fmt.Errorf("interrupted encyclopedia publication has invalid journal: %v; recover with: %s", err, encyclopediaRecoveryCommand(recoveryCommand, paths.output))
	}
	if exists {
		return fmt.Errorf("interrupted encyclopedia publication at phase %s; recover with: %s", transaction.Phase, encyclopediaRecoveryCommand(recoveryCommand, paths.output))
	}
	present, err := encyclopediaDirectoryExists(paths.output)
	if err != nil {
		return err
	}
	if !present {
		return fmt.Errorf("encyclopedia output is absent: %s", paths.output)
	}
	inventory, err := inspectEncyclopediaOwnedDirectory(paths.output, inspect)
	if err != nil {
		return fmt.Errorf("verify encyclopedia output: %w", err)
	}
	fmt.Fprintf(log, "Verified encyclopedia directory %s (%d owned files)\n", paths.output, len(inventory.Files))
	return nil
}

func inspectEncyclopediaOwnedDirectory(root string, inspect encyclopediaOwnedInspector) (encyclopediaOwnedInventory, error) {
	actual, err := scanEncyclopediaDirectory(root)
	if err != nil {
		return encyclopediaOwnedInventory{}, err
	}
	inventory, err := inspect(root)
	if err != nil {
		return encyclopediaOwnedInventory{}, err
	}
	normalized, err := normalizeEncyclopediaOwnedFiles(inventory.Files)
	if err != nil {
		return encyclopediaOwnedInventory{}, err
	}
	if len(actual.files) != len(normalized) {
		return encyclopediaOwnedInventory{}, describeEncyclopediaOwnershipDifference(actual.files, normalized)
	}
	for index := range actual.files {
		if actual.files[index] != normalized[index] {
			return encyclopediaOwnedInventory{}, describeEncyclopediaOwnershipDifference(actual.files, normalized)
		}
	}
	expectedDirectories := encyclopediaOwnedDirectories(normalized)
	if len(actual.directories) != len(expectedDirectories) {
		return encyclopediaOwnedInventory{}, describeEncyclopediaDirectoryDifference(actual.directories, expectedDirectories)
	}
	for index := range actual.directories {
		if actual.directories[index] != expectedDirectories[index] {
			return encyclopediaOwnedInventory{}, describeEncyclopediaDirectoryDifference(actual.directories, expectedDirectories)
		}
	}
	return encyclopediaOwnedInventory{Files: normalized}, nil
}

type encyclopediaDirectoryTree struct {
	files       []string
	directories []string
}

func scanEncyclopediaDirectory(root string) (encyclopediaDirectoryTree, error) {
	info, err := os.Lstat(root)
	if err != nil {
		return encyclopediaDirectoryTree{}, err
	}
	if info.Mode()&os.ModeSymlink != 0 || !info.IsDir() {
		return encyclopediaDirectoryTree{}, fmt.Errorf("unsafe symlink or non-directory root %s", root)
	}
	tree := encyclopediaDirectoryTree{}
	err = filepath.WalkDir(root, func(path string, entry os.DirEntry, walkErr error) error {
		if walkErr != nil {
			return walkErr
		}
		if path == root {
			return nil
		}
		info, err := entry.Info()
		if err != nil {
			return err
		}
		if info.Mode()&os.ModeSymlink != 0 {
			return fmt.Errorf("unsafe symlink in generated directory: %s", path)
		}
		relative, err := filepath.Rel(root, path)
		if err != nil {
			return err
		}
		relative = filepath.ToSlash(relative)
		if info.IsDir() {
			tree.directories = append(tree.directories, relative)
			return nil
		}
		if !info.Mode().IsRegular() {
			return fmt.Errorf("unsupported generated file type: %s", path)
		}
		tree.files = append(tree.files, relative)
		return nil
	})
	if err != nil {
		return encyclopediaDirectoryTree{}, err
	}
	sort.Strings(tree.files)
	sort.Strings(tree.directories)
	return tree, nil
}

func normalizeEncyclopediaOwnedFiles(files []string) ([]string, error) {
	normalized := make([]string, 0, len(files))
	seen := make(map[string]string, len(files))
	for _, name := range files {
		if name == "" || strings.ContainsRune(name, '\x00') || strings.Contains(name, "\\") || filepath.IsAbs(name) {
			return nil, fmt.Errorf("unsafe owned path %q", name)
		}
		cleaned := filepath.ToSlash(filepath.Clean(filepath.FromSlash(name)))
		if cleaned == "." || cleaned == ".." || strings.HasPrefix(cleaned, "../") || cleaned != name {
			return nil, fmt.Errorf("unsafe owned path %q", name)
		}
		folded := strings.ToLower(cleaned)
		if prior, exists := seen[folded]; exists {
			return nil, fmt.Errorf("path_collision: owned paths %q and %q collide", prior, cleaned)
		}
		seen[folded] = cleaned
		normalized = append(normalized, cleaned)
	}
	sort.Strings(normalized)
	return normalized, nil
}

func describeEncyclopediaOwnershipDifference(actual, owned []string) error {
	ownedSet := make(map[string]struct{}, len(owned))
	for _, name := range owned {
		ownedSet[name] = struct{}{}
	}
	for _, name := range actual {
		if _, ok := ownedSet[name]; !ok {
			return fmt.Errorf("unknown user file %q is not in generated ownership inventory", name)
		}
	}
	actualSet := make(map[string]struct{}, len(actual))
	for _, name := range actual {
		actualSet[name] = struct{}{}
	}
	for _, name := range owned {
		if _, ok := actualSet[name]; !ok {
			return fmt.Errorf("owned generated file %q is missing", name)
		}
	}
	return fmt.Errorf("generated ownership inventory does not match directory")
}

func encyclopediaOwnedDirectories(files []string) []string {
	directories := make(map[string]struct{})
	for _, name := range files {
		for parent := filepath.ToSlash(filepath.Dir(filepath.FromSlash(name))); parent != "."; parent = filepath.ToSlash(filepath.Dir(filepath.FromSlash(parent))) {
			directories[parent] = struct{}{}
		}
	}
	result := make([]string, 0, len(directories))
	for directory := range directories {
		result = append(result, directory)
	}
	sort.Strings(result)
	return result
}

func describeEncyclopediaDirectoryDifference(actual, owned []string) error {
	ownedSet := make(map[string]struct{}, len(owned))
	for _, name := range owned {
		ownedSet[name] = struct{}{}
	}
	for _, name := range actual {
		if _, ok := ownedSet[name]; !ok {
			return fmt.Errorf("unknown user directory %q is not implied by generated ownership inventory", name)
		}
	}
	actualSet := make(map[string]struct{}, len(actual))
	for _, name := range actual {
		actualSet[name] = struct{}{}
	}
	for _, name := range owned {
		if _, ok := actualSet[name]; !ok {
			return fmt.Errorf("owned generated directory %q is missing", name)
		}
	}
	return fmt.Errorf("generated ownership directory set does not match inventory")
}

func encyclopediaDirectoriesEqual(leftRoot string, left encyclopediaOwnedInventory, rightRoot string, right encyclopediaOwnedInventory) (bool, error) {
	if len(left.Files) != len(right.Files) {
		return false, nil
	}
	for index, name := range left.Files {
		if right.Files[index] != name {
			return false, nil
		}
		equal, err := encyclopediaRegularFilesEqual(filepath.Join(leftRoot, filepath.FromSlash(name)), filepath.Join(rightRoot, filepath.FromSlash(name)))
		if err != nil {
			return false, err
		}
		if !equal {
			return false, nil
		}
	}
	return true, nil
}

func encyclopediaRegularFilesEqual(leftPath, rightPath string) (bool, error) {
	left, leftInfo, err := openEncyclopediaRegularFile(leftPath)
	if err != nil {
		return false, err
	}
	defer left.Close()
	right, rightInfo, err := openEncyclopediaRegularFile(rightPath)
	if err != nil {
		return false, err
	}
	defer right.Close()
	if leftInfo.Size() != rightInfo.Size() {
		return false, nil
	}
	leftBuffer := make([]byte, 64<<10)
	rightBuffer := make([]byte, 64<<10)
	for {
		leftCount, leftErr := left.Read(leftBuffer)
		rightCount, rightErr := right.Read(rightBuffer)
		if leftCount != rightCount || !bytes.Equal(leftBuffer[:leftCount], rightBuffer[:rightCount]) {
			return false, nil
		}
		if leftErr == io.EOF && rightErr == io.EOF {
			return true, nil
		}
		if leftErr != nil && leftErr != io.EOF {
			return false, leftErr
		}
		if rightErr != nil && rightErr != io.EOF {
			return false, rightErr
		}
	}
}

func openEncyclopediaRegularFile(path string) (*os.File, os.FileInfo, error) {
	before, err := os.Lstat(path)
	if err != nil {
		return nil, nil, err
	}
	if before.Mode()&os.ModeSymlink != 0 || !before.Mode().IsRegular() {
		return nil, nil, fmt.Errorf("unsafe non-regular generated file %s", path)
	}
	file, err := os.Open(path)
	if err != nil {
		return nil, nil, err
	}
	after, err := file.Stat()
	if err != nil {
		file.Close()
		return nil, nil, err
	}
	if !os.SameFile(before, after) || !after.Mode().IsRegular() {
		file.Close()
		return nil, nil, fmt.Errorf("generated file changed while opening: %s", path)
	}
	return file, after, nil
}

func readEncyclopediaRegularFile(path string, maxBytes int64) ([]byte, error) {
	file, info, err := openEncyclopediaRegularFile(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	if info.Size() > maxBytes {
		return nil, fmt.Errorf("%s exceeds %d-byte limit", path, maxBytes)
	}
	contents, err := io.ReadAll(io.LimitReader(file, maxBytes+1))
	if err != nil {
		return nil, err
	}
	if int64(len(contents)) > maxBytes {
		return nil, fmt.Errorf("%s exceeds %d-byte limit", path, maxBytes)
	}
	return contents, nil
}

func resolveEncyclopediaOutputPath(path string) (string, error) {
	if strings.TrimSpace(path) == "" {
		return "", fmt.Errorf("path_collision: encyclopedia output path is empty")
	}
	absolute, err := filepath.Abs(filepath.Clean(path))
	if err != nil {
		return "", err
	}
	current := filepath.VolumeName(absolute) + string(filepath.Separator)
	relative := strings.TrimPrefix(absolute, current)
	for _, component := range strings.Split(relative, string(filepath.Separator)) {
		if component == "" {
			continue
		}
		current = filepath.Join(current, component)
		info, err := os.Lstat(current)
		if os.IsNotExist(err) {
			break
		}
		if err != nil {
			return "", err
		}
		if info.Mode()&os.ModeSymlink != 0 {
			return "", fmt.Errorf("unsafe symlink in encyclopedia output path: %s", current)
		}
	}
	return absolute, nil
}

func validateEncyclopediaRootCollisions(output string, sourceRoots, modRoots []string) error {
	for _, source := range sourceRoots {
		resolved, err := resolveEncyclopediaComparisonPath(source)
		if err != nil {
			return fmt.Errorf("resolve source root: %w", err)
		}
		if output == resolved || encyclopediaPathWithin(resolved, output) {
			return fmt.Errorf("path_collision: encyclopedia output %s would replace source root %s", output, resolved)
		}
	}
	for _, modRoot := range modRoots {
		resolved, err := resolveEncyclopediaComparisonPath(modRoot)
		if err != nil {
			return fmt.Errorf("resolve mod root: %w", err)
		}
		if encyclopediaPathWithin(output, resolved) || encyclopediaPathWithin(resolved, output) {
			return fmt.Errorf("path_collision: encyclopedia output %s overlaps mod root %s", output, resolved)
		}
	}
	return nil
}

func resolveEncyclopediaComparisonPath(path string) (string, error) {
	absolute, err := filepath.Abs(filepath.Clean(path))
	if err != nil {
		return "", err
	}
	resolved, err := filepath.EvalSymlinks(absolute)
	if err == nil {
		return resolved, nil
	}
	if !os.IsNotExist(err) {
		return "", err
	}
	return absolute, nil
}

func encyclopediaPathWithin(path, root string) bool {
	relative, err := filepath.Rel(root, path)
	if err != nil {
		return false
	}
	return relative == "." || (relative != ".." && !strings.HasPrefix(relative, ".."+string(filepath.Separator)))
}
