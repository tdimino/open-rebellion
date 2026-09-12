package main

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"image/png"
	"io"
	"math"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"strings"
)

var cutsceneIDs = []string{"000", "001", "003", "004", "005", "101", "102", "103", "104", "105", "106", "107", "108", "201", "202"}

type mediaRunner func(string, ...string) ([]byte, error)

func runMedia(name string, args ...string) ([]byte, error) {
	b, e := exec.Command(name, args...).CombinedOutput()
	if e != nil {
		return nil, fmt.Errorf("%s: %w: %s", name, e, b)
	}
	return b, nil
}

type movieMetadata struct {
	FPS        float64 `json:"fps"`
	Width      int     `json:"width"`
	Height     int     `json:"height"`
	FrameCount int     `json:"frame_count"`
}
type movieManifest struct {
	Version      int               `json:"version"`
	SourceSHA256 string            `json:"source_sha256"`
	Metadata     movieMetadata     `json:"metadata"`
	Files        map[string]string `json:"files"`
}

func moviePaths(id string, count int) []string {
	paths := []string{filepath.Join("ref-videos", id+".webm"), filepath.Join("cutscene-frames", id+".wav"), filepath.Join("cutscene-frames", id, "metadata.json")}
	for i := 1; i <= count; i++ {
		paths = append(paths, filepath.Join("cutscene-frames", id, fmt.Sprintf("frame-%05d.png", i)))
	}
	return paths
}
func validMovieMetadata(m movieMetadata) bool {
	return m.Width > 0 && m.Width <= 65535 && m.Height > 0 && m.Height <= 65535 && m.FrameCount > 0 && m.FrameCount <= 100000 && m.FPS > 0 && !math.IsInf(m.FPS, 0) && !math.IsNaN(m.FPS)
}
func verifyCutscene(root, id string) (movieManifest, error) {
	var m movieManifest
	b, e := os.ReadFile(filepath.Join(root, "cutscene-frames", id, "extraction.json"))
	if e != nil {
		return m, e
	}
	if e = json.Unmarshal(b, &m); e != nil {
		return m, e
	}
	if m.Version != 1 || len(m.SourceSHA256) != 64 || !validMovieMetadata(m.Metadata) {
		return m, fmt.Errorf("invalid cutscene %s manifest", id)
	}
	paths := moviePaths(id, m.Metadata.FrameCount)
	if len(paths) != len(m.Files) {
		return m, fmt.Errorf("cutscene %s inventory mismatch", id)
	}
	for _, p := range paths {
		expected, ok := m.Files[filepath.ToSlash(p)]
		if !ok {
			return m, fmt.Errorf("missing inventory entry %s", p)
		}
		actual, e := fileSHA256(filepath.Join(root, p))
		if e != nil {
			return m, e
		}
		if actual != expected {
			return m, fmt.Errorf("cutscene checksum mismatch: %s", p)
		}
	}
	mb, e := os.ReadFile(filepath.Join(root, "cutscene-frames", id, "metadata.json"))
	if e != nil {
		return m, e
	}
	var meta movieMetadata
	if e = json.Unmarshal(mb, &meta); e != nil {
		return m, e
	}
	if meta != m.Metadata {
		return m, fmt.Errorf("cutscene metadata mismatch")
	}
	return m, nil
}
func probeMovie(path string, run mediaRunner) (movieMetadata, error) {
	b, e := run("ffprobe", "-v", "error", "-select_streams", "v:0", "-show_entries", "stream=width,height,avg_frame_rate", "-of", "json", path)
	if e != nil {
		return movieMetadata{}, e
	}
	var p struct {
		Streams []struct {
			Width  int    `json:"width"`
			Height int    `json:"height"`
			Rate   string `json:"avg_frame_rate"`
		} `json:"streams"`
	}
	if e = json.Unmarshal(b, &p); e != nil {
		return movieMetadata{}, e
	}
	if len(p.Streams) != 1 {
		return movieMetadata{}, fmt.Errorf("expected one video stream")
	}
	parts := strings.Split(p.Streams[0].Rate, "/")
	if len(parts) != 2 {
		return movieMetadata{}, fmt.Errorf("invalid video frame rate")
	}
	n, e := strconv.ParseFloat(parts[0], 64)
	if e != nil {
		return movieMetadata{}, e
	}
	d, e := strconv.ParseFloat(parts[1], 64)
	if e != nil || d == 0 {
		return movieMetadata{}, fmt.Errorf("invalid video frame rate denominator")
	}
	m := movieMetadata{FPS: n / d, Width: p.Streams[0].Width, Height: p.Streams[0].Height, FrameCount: 1}
	if !validMovieMetadata(m) {
		return m, fmt.Errorf("invalid video dimensions or rate")
	}
	return m, nil
}
func stageCutscene(source, output, id string, force bool, run mediaRunner, log io.Writer) error {
	src := filepath.Join(source, "MDATA."+id)
	hash, e := fileSHA256(src)
	if e != nil {
		return e
	}
	if m, e := verifyCutscene(output, id); e == nil && m.SourceSHA256 == hash {
		fmt.Fprintf(log, "Cutscene %s unchanged (%d frames)\n", id, m.Metadata.FrameCount)
		return nil
	}
	for _, p := range []string{filepath.Join(output, "ref-videos", id+".webm"), filepath.Join(output, "cutscene-frames", id+".wav"), filepath.Join(output, "cutscene-frames", id)} {
		if _, e := os.Lstat(p); e == nil && !force {
			return fmt.Errorf("cutscene %s exists without matching verified extraction (use --force to replace it)", id)
		} else if e != nil && !os.IsNotExist(e) {
			return e
		}
	}
	if e := os.MkdirAll(output, 0755); e != nil {
		return e
	}
	temp, e := os.MkdirTemp(output, ".cutscene-"+id+"-")
	if e != nil {
		return e
	}
	retainTemp := false
	defer func() {
		if !retainTemp {
			os.RemoveAll(temp)
		}
	}()
	videos := filepath.Join(temp, "ref-videos")
	frames := filepath.Join(temp, "cutscene-frames", id)
	if e := os.MkdirAll(videos, 0755); e != nil {
		return e
	}
	if e := os.MkdirAll(frames, 0755); e != nil {
		return e
	}
	webm := filepath.Join(videos, id+".webm")
	audio := filepath.Join(temp, "cutscene-frames", id+".wav")
	fmt.Fprintf(log, "Converting cutscene %s\n", id)
	if _, e := run("ffmpeg", "-y", "-nostdin", "-v", "error", "-i", src, "-map", "0:v:0", "-map", "0:a:0", "-c:v", "libvpx-vp9", "-threads", "2", "-crf", "30", "-b:v", "0", "-c:a", "libopus", webm); e != nil {
		return e
	}
	meta, e := probeMovie(webm, run)
	if e != nil {
		return e
	}
	if _, e := run("ffmpeg", "-y", "-nostdin", "-v", "error", "-i", webm, "-map", "0:v:0", "-fps_mode", "passthrough", filepath.Join(frames, "frame-%05d.png")); e != nil {
		return e
	}
	if _, e := run("ffmpeg", "-y", "-nostdin", "-v", "error", "-i", webm, "-map", "0:a:0", "-vn", "-c:a", "pcm_s16le", "-f", "wav", audio); e != nil {
		return e
	}
	entries, e := os.ReadDir(frames)
	if e != nil {
		return e
	}
	meta.FrameCount = len(entries)
	if !validMovieMetadata(meta) {
		return fmt.Errorf("invalid decoded frame count")
	}
	for i := 1; i <= meta.FrameCount; i++ {
		f, e := os.Open(filepath.Join(frames, fmt.Sprintf("frame-%05d.png", i)))
		if e != nil {
			return e
		}
		cfg, e := png.DecodeConfig(f)
		f.Close()
		if e != nil {
			return e
		}
		if cfg.Width != meta.Width || cfg.Height != meta.Height {
			return fmt.Errorf("cutscene %s frame dimensions mismatch", id)
		}
	}
	wav, e := os.ReadFile(audio)
	if e != nil {
		return e
	}
	if e := validateWAV(wav); e != nil {
		return e
	}
	header := make([]byte, 4)
	f, e := os.Open(webm)
	if e != nil {
		return e
	}
	_, e = io.ReadFull(f, header)
	f.Close()
	if e != nil || !bytes.Equal(header, []byte{0x1a, 0x45, 0xdf, 0xa3}) {
		return fmt.Errorf("invalid WebM output")
	}
	mb, e := json.MarshalIndent(meta, "", "  ")
	if e != nil {
		return e
	}
	if e := os.WriteFile(filepath.Join(frames, "metadata.json"), append(mb, '\n'), 0644); e != nil {
		return e
	}
	manifest := movieManifest{Version: 1, SourceSHA256: hash, Metadata: meta, Files: make(map[string]string)}
	for _, p := range moviePaths(id, meta.FrameCount) {
		h, e := fileSHA256(filepath.Join(temp, p))
		if e != nil {
			return e
		}
		manifest.Files[filepath.ToSlash(p)] = h
	}
	b, e := json.MarshalIndent(manifest, "", "  ")
	if e != nil {
		return e
	}
	if e := os.WriteFile(filepath.Join(frames, "extraction.json"), append(b, '\n'), 0644); e != nil {
		return e
	}
	if _, e := verifyCutscene(temp, id); e != nil {
		return e
	}
	// Finish conversion and validation before replacing any old output.
	retainTemp, e = publishCutscene(temp, output, id, os.Rename)
	if e != nil {
		return e
	}
	fmt.Fprintf(log, "Staged cutscene %s (%d frames)\n", id, meta.FrameCount)
	return nil
}
func stageCutscenes(source, output string, force bool, ids []string, run mediaRunner, log io.Writer) error {
	for _, id := range ids {
		if e := stageCutscene(source, output, id, force, run, log); e != nil {
			return fmt.Errorf("cutscene %s: %w", id, e)
		}
	}
	return nil
}
func verifyCutscenes(output string, ids []string, log io.Writer) error {
	for _, id := range ids {
		m, e := verifyCutscene(output, id)
		if e != nil {
			return e
		}
		fmt.Fprintf(log, "Verified cutscene %s (%d frames)\n", id, m.Metadata.FrameCount)
	}
	return nil
}

// Roll back every replacement on a publication error. A failed rollback keeps
// the temporary directory (including its backups) for recovery.
func publishCutscene(temp, output, id string, rename func(string, string) error) (bool, error) {
	type replacement struct {
		src, dst, backup  string
		hadOld, installed bool
	}
	paths := []string{filepath.Join("ref-videos", id+".webm"), filepath.Join("cutscene-frames", id+".wav"), filepath.Join("cutscene-frames", id)}
	items := make([]replacement, len(paths))
	for i, p := range paths {
		items[i] = replacement{src: filepath.Join(temp, p), dst: filepath.Join(output, p), backup: filepath.Join(temp, fmt.Sprintf("backup-%d", i))}
		info, e := os.Lstat(items[i].dst)
		if e == nil {
			if (i == 2 && !info.IsDir()) || (i < 2 && !info.Mode().IsRegular()) {
				return false, fmt.Errorf("unexpected destination type: %s", items[i].dst)
			}
		} else if !os.IsNotExist(e) {
			return false, e
		}
		if e := os.MkdirAll(filepath.Dir(items[i].dst), 0755); e != nil {
			return false, e
		}
	}
	rollback := func(cause error) (bool, error) {
		var recovery []error
		for i := len(items) - 1; i >= 0; i-- {
			item := &items[i]
			if item.installed {
				if e := rename(item.dst, item.src); e != nil {
					recovery = append(recovery, e)
					continue
				}
			}
			if item.hadOld {
				if e := rename(item.backup, item.dst); e != nil {
					recovery = append(recovery, e)
				}
			}
		}
		if len(recovery) > 0 {
			return true, fmt.Errorf("publish failed: %v; backups retained at %s; rollback: %w", cause, temp, errors.Join(recovery...))
		}
		return false, cause
	}
	for i := range items {
		item := &items[i]
		if _, e := os.Lstat(item.dst); e == nil {
			if e := rename(item.dst, item.backup); e != nil {
				return rollback(e)
			}
			item.hadOld = true
		} else if !os.IsNotExist(e) {
			return rollback(e)
		}
		if e := rename(item.src, item.dst); e != nil {
			return rollback(e)
		}
		item.installed = true
	}
	return false, nil
}
