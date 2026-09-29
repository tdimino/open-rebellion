package main

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"reflect"
	"strings"
	"testing"
)

func TestEncyclopediaChromeCatalogContainsOnlySourceProvenBitmapIDs(t *testing.T) {
	want := []uint32{
		0x285f, 0x2860, 0x2861, 0x2862, 0x2959, 0x295d,
		0x2882, 0x2883, 0x2888, 0x2889,
		0x288e, 0x288f, 0x2890, 0x2891, 0x2892, 0x2893,
		0x2886, 0x2887, 0x288c, 0x288d,
		0x2884, 0x2885, 0x288a, 0x288b,
		0x2864, 0x2863, 0x286e, 0x286d,
		0x286c, 0x286b, 0x2878, 0x2877,
		0x2868, 0x2867, 0x2874, 0x2873,
		0x2d60, 0x2d5f, 0x2d62, 0x2d61,
		0x2870, 0x286f, 0x287a, 0x2879,
		0x286a, 0x2869, 0x2876, 0x2875,
	}

	if !reflect.DeepEqual(requiredEncyclopediaChromeResourceIDs, want) {
		t.Fatalf("required encyclopedia chrome IDs = %#v, want %#v", requiredEncyclopediaChromeResourceIDs, want)
	}
	seen := make(map[uint32]bool, len(want))
	for _, resourceID := range requiredEncyclopediaChromeResourceIDs {
		if seen[resourceID] {
			t.Fatalf("duplicate required encyclopedia resource %#x", resourceID)
		}
		seen[resourceID] = true
	}
	for _, nonBitmapID := range []uint32{0x1842, 0x1843, 0x299d} {
		if seen[nonBitmapID] {
			t.Fatalf("text/font resource %#x must not be staged as encyclopedia chrome", nonBitmapID)
		}
	}

	var strategy *dllTarget
	for index := range uiDLLTargets {
		if uiDLLTargets[index].Filename == "STRATEGY.DLL" {
			strategy = &uiDLLTargets[index]
			break
		}
	}
	if strategy == nil {
		t.Fatal("STRATEGY.DLL target is missing")
	}
	if !reflect.DeepEqual(strategy.RequiredBMPs, want) {
		t.Fatalf("STRATEGY.DLL required BMPs = %#v, want %#v", strategy.RequiredBMPs, want)
	}
}

func TestVerifyTargetsRejectsMissingEncyclopediaChromeWithUnchangedCount(t *testing.T) {
	outputDir := t.TempDir()
	writeSyntheticChromeBMP(t, outputDir, "strategy-dll", 0x285f)
	writeSyntheticChromeBMP(t, outputDir, "strategy-dll", 999999)

	_, err := verifyTargets(outputDir, []dllTarget{{
		Filename:     "STRATEGY.DLL",
		Directory:    "strategy-dll",
		Expected:     2,
		RequiredBMPs: []uint32{0x285f, 0x2860},
	}}, io.Discard)
	if err == nil {
		t.Fatal("verifyTargets() accepted a count-preserving required-resource substitution")
	}
	for _, fragment := range []string{"STRATEGY.DLL", "strategy-dll/BMP/10336.bmp", "encyclopedia chrome"} {
		if !strings.Contains(err.Error(), fragment) {
			t.Fatalf("verifyTargets() error = %q, want source diagnostic containing %q", err, fragment)
		}
	}
}

func TestVerifyTargetsAcceptsRequiredChromeWithoutRejectingUnrelatedBitmaps(t *testing.T) {
	outputDir := t.TempDir()
	for _, resourceID := range []uint32{0x285f, 0x2860, 424242} {
		writeSyntheticChromeBMP(t, outputDir, "strategy-dll", resourceID)
	}

	summary, err := verifyTargets(outputDir, []dllTarget{{
		Filename:     "STRATEGY.DLL",
		Directory:    "strategy-dll",
		Expected:     3,
		RequiredBMPs: []uint32{0x285f, 0x2860},
	}}, io.Discard)
	if err != nil {
		t.Fatalf("verifyTargets() error = %v", err)
	}
	if summary != (verifySummary{DLLs: 1, Resources: 3}) {
		t.Fatalf("verifyTargets() summary = %+v", summary)
	}
}

// This opt-in test proves each source-proven STRATEGY.DLL resource survives
// the existing DIB-to-BMP staging path byte-for-byte. It writes only beneath a
// caller-provided ignored evidence directory, or a test temporary directory.
func TestOwnedEncyclopediaChromeMatchesStagedBytes(t *testing.T) {
	source := os.Getenv("REBELLION_ENCYCLOPEDIA_TEST_SOURCE")
	if source == "" {
		t.Skip("set REBELLION_ENCYCLOPEDIA_TEST_SOURCE to an owned installation root")
	}
	output := os.Getenv("REBELLION_ENCYCLOPEDIA_CHROME_TEST_OUTPUT")
	if output == "" {
		output = t.TempDir()
	} else if err := os.MkdirAll(output, 0o755); err != nil {
		t.Fatal(err)
	}

	var target dllTarget
	for _, candidate := range uiDLLTargets {
		if candidate.Filename == "STRATEGY.DLL" {
			target = candidate
			break
		}
	}
	if target.Filename == "" {
		t.Fatal("STRATEGY.DLL target is missing")
	}
	if _, err := stageTargets(source, output, []dllTarget{target}, namedBitmapIDs, true, io.Discard); err != nil {
		t.Fatal(err)
	}
	if _, err := verifyTargets(output, []dllTarget{target}, io.Discard); err != nil {
		t.Fatal(err)
	}

	resources, err := readPEBitmapResources(filepath.Join(source, target.Filename), namedBitmapIDs)
	if err != nil {
		t.Fatal(err)
	}
	byID := make(map[uint32][]byte, len(resources))
	for _, resource := range resources {
		bmp, err := dibToBMP(resource.DIB)
		if err != nil {
			t.Fatalf("convert source resource %d: %v", resource.ID, err)
		}
		byID[resource.ID] = bmp
	}
	for _, resourceID := range target.RequiredBMPs {
		staged, err := os.ReadFile(filepath.Join(output, target.Directory, "BMP", resourceFilename(resourceID)))
		if err != nil {
			t.Fatal(err)
		}
		if !bytes.Equal(staged, byID[resourceID]) {
			t.Fatalf("staged STRATEGY.DLL BMP resource %d differs from converted source bytes", resourceID)
		}
	}
	if len(target.RequiredBMPs) != 48 {
		t.Fatalf("verified %d required resources, want 48", len(target.RequiredBMPs))
	}
}

func writeSyntheticChromeBMP(t *testing.T, outputDir, dllDirectory string, resourceID uint32) {
	t.Helper()
	dib := make([]byte, 44)
	binary.LittleEndian.PutUint32(dib[0:4], 40)
	binary.LittleEndian.PutUint32(dib[4:8], 1)
	binary.LittleEndian.PutUint32(dib[8:12], 1)
	binary.LittleEndian.PutUint16(dib[12:14], 1)
	binary.LittleEndian.PutUint16(dib[14:16], 24)
	bmp, err := dibToBMP(dib)
	if err != nil {
		t.Fatal(err)
	}
	bmpDir := filepath.Join(outputDir, dllDirectory, "BMP")
	if err := os.MkdirAll(bmpDir, 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(bmpDir, resourceFilename(resourceID)), bmp, 0o644); err != nil {
		t.Fatal(err)
	}
}

func resourceFilename(resourceID uint32) string {
	return fmt.Sprintf("%d.bmp", resourceID)
}
