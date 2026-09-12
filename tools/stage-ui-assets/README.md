# stage-ui-assets

Extract original Star Wars Rebellion UI resources into the directory layout
Open Rebellion loads at runtime. One command stages 2,303 standard BMPs and
3,988 custom advisor frames from six game DLLs, plus voices, menu effects,
and soundtrack WAVs. It then verifies both UI and audio outputs. It
uses only the Go standard library. No Python environment, third-party package,
or Windows runtime is needed.

For standard bitmaps, the extractor preserves the original DIB bytes and adds a
BMP file header. For advisor animations, it preserves each custom PE type-302
resource byte for byte. It does not resize or re-encode the artwork.

## Requirements

- Go 1.22 or later to build or use `go run`.
- Your own copy of the six UI DLLs listed below plus `VOICEFXA.DLL` and
  `VOICEFXE.DLL`, together in one source directory, and the original
  `MDATA.300`–`MDATA.315` soundtrack files in `source/MDATA` or `--mdata`. Extraction reads these files without modifying them.

The compiled executable does not require Go to run. Game files are not included
in this repository.

## Build and run

Run these commands from the **repository root**:

```sh
go build -o ./stage-ui-assets ./tools/stage-ui-assets
./stage-ui-assets --source "/path/to/Star Wars - Rebellion" --output ./data/base/ui
```

Point `--source` at the directory containing the DLLs themselves. The tool does
not search subdirectories. On a case-sensitive filesystem, their filenames must
match the uppercase names in the table below.

On Windows, build with `-o stage-ui-assets.exe` and run
`.\stage-ui-assets.exe` with the same flags.

Alternatively, build and execute in one step:

```sh
go run ./tools/stage-ui-assets --source "/path/to/Star Wars - Rebellion"
```

If the DLLs are in `data/base/` and the soundtrack is in `data/base/MDATA/`,
no flags are needed:

```sh
go run ./tools/stage-ui-assets
```

All relative paths, including the defaults, resolve from the current working
directory—not from the executable's location. The output directory is created
as needed. A successful extraction ends with:

```text
Staged 6291 UI resources from 6 DLLs (6291 written, 0 unchanged)
...
Verified 6291 UI resources across 6 DLLs
Staged 310 audio files (310 written, 0 unchanged)
Verified 310 audio files
```

Write and unchanged counts depend on what is already staged.

## Output layout

Each DLL has its own directory, so equal resource IDs in different DLLs do not
collide. Numeric resource IDs become filenames, for example:
`data/base/ui/strategy-dll/BMP/900.bmp`.

| Required source file | Standard BMPs | Type-302 frames | Directory under `--output` |
| --- | ---: | ---: | --- |
| `COMMON.DLL` | 321 | 0 | `common-dll/` |
| `GOKRES.DLL` | 580 | 0 | `gokres-dll/` |
| `STRATEGY.DLL` | 1,042 | 0 | `strategy-dll/` |
| `TACTICAL.DLL` | 288 | 0 | `tactical-dll/` |
| `ALSPRITE.DLL` | 38 | 1,640 | `alsprite-dll/` |
| `EMSPRITE.DLL` | 34 | 2,348 | `emsprite-dll/` |

Standard resources are written to `BMP/{id}.bmp`; custom advisor frames are
written unchanged to `TYPE302/{id}.bin`.

The tool also maps seven known string-named resources to the numeric filenames
used by the runtime catalog:

| Resource name | Output ID |
| --- | ---: |
| `COCKPIT_BUTTON_GAMESCALE_HUGE_UP` | 15856 |
| `COCKPIT_BUTTON_GAMESCALE_LARGE_UP` | 15922 |
| `COCKPIT_BUTTON_GAMESCALE_STD_UP` | 15990 |
| `DATA_BUTTON_UP_FIGHTERGROUP_RECOVER` | 40720 |
| `DATA_BUTTON_DN_FIGHTERGROUP_RECOVER` | 40792 |
| `DATA_BUTTON_UP_FIGHTERGROUP_TACTICS` | 40864 |
| `DATA_BUTTON_DN_FIGHTERGROUP_TACTICS` | 40936 |

Unknown named resources and duplicate IDs within a DLL, including IDs shared by
multiple languages—cause an error rather than selecting one silently. All six
DLLs and their expected counts are fixed; there is no single-DLL selection flag.

## Verify or refresh existing assets

Check an existing output directory without reading the source DLLs or writing
files:

```sh
./stage-ui-assets --verify --output ./data/base/ui
```

Verification checks each DLL directory's expected resource counts and canonical
numeric filenames. It validates BMP signatures, declared sizes, DIB headers,
and pixel offsets. It also validates every type-302 header, scanline table,
payload size, unchanged skip, additive run, and row boundary.
`--source` and `--force` have no effect with `--verify`.

Verification does **not** compare files against the DLLs, check an exact inventory
of resource IDs, apply the advisor palette, or prove that the game displays the assets.
For a byte-for-byte comparison with the source-derived BMPs, rerun extraction:

```sh
./stage-ui-assets --source "/path/to/Star Wars - Rebellion" --output ./data/base/ui
```

Identical files are left untouched and counted as unchanged. A differing file
stops extraction unless you explicitly request replacement:

```sh
./stage-ui-assets --source "/path/to/Star Wars - Rebellion" --output ./data/base/ui --force
```

Writes use a temporary file in the destination directory, sync and close it,
then rename it into place. This is per-file handling, not a transaction for the
whole run: files written before an error remain available for the next attempt.
The tool never removes extra files; unexpected extra `.bmp` files can therefore
make the final count check fail even with `--force`.

## Flags and failures

| Flag | Default | Purpose |
| --- | --- | --- |
| `--source` | `data/base` | Directory containing the six DLLs |
| `--output` | `data/base/ui` | Root of the staged asset directories |
| `--verify` | `false` | Check existing output without extraction |
| `--force` | `false` | Replace files whose contents differ |
| `--help` | | Print usage |

Successful runs and help exit with status 0. Errors exit with status 1 and an
`ERROR:` message on stderr. Positional arguments are not accepted.

- **Missing DLL:** check `--source`, filenames, and case.
- **Unexpected resource count or unsupported named resource:** the input does
  not match this tool's supported inventory. `--force` does not bypass these
  checks.
- **Existing file differs:** retain it by choosing another output directory,
  or use `--force` to replace it from the source DLL.
- **Verification fails:** inspect the reported file or directory. Rerun
  extraction to restore missing files; use `--force` for differing files.

This tool stages standard bitmaps and advisor type-302 animation frames. It does
not extract sound, SPT/BIN/FDT control data, briefing animation, tactical meshes
or textures, DAT tables, or EData images. It does not generate the browser
manifest or runtime pack. The repository's
[WASM build script](../../scripts/build-wasm.sh) consumes `data/base/ui/` for
browser packaging. See the
[asset guide](../../agent_docs/assets.md) for the broader pipeline.

## Development checks

From the repository root:

```sh
go test -count=1 ./tools/stage-ui-assets
go vet ./tools/stage-ui-assets
```

Tests construct synthetic PE files, DIB data, and type-302 frames, so they run
without proprietary game files. They cover resource traversal, named-ID
mappings, BMP headers, sparse-frame validation, dimension limits, runtime output
paths, duplicate-ID rejection, preserving or replacing existing files,
temporary-file cleanup, and CLI staging with verification.

## Audio extraction

Every extraction stages the original voice lines, four menu effects, and all
16 soundtrack files alongside the UI assets. Verification checks both UI and
audio by default. No audio opt-in flag is needed. Audio extraction also uses
only the Go standard library.

From the repository root:

```sh
make extract-assets GAME_SOURCE="/path/to/Star Wars - Rebellion"
```

`GAME_SOURCE` must contain the six UI DLLs plus `VOICEFXA.DLL` and
`VOICEFXE.DLL`. `MDATA_DIR` defaults to `GAME_SOURCE/MDATA`. If the DLLs have
already been copied into `data/base`, point at the original soundtrack directory:

```sh
make extract-assets MDATA_DIR="/path/to/Star Wars - Rebellion/MDATA"
```

For custom outputs or overwrite/verification options, use the extractor flags:

```sh
go run ./tools/stage-ui-assets --source "/path/to/game" \
  --mdata "/path/to/game/MDATA" --audio-output data/sounds

make verify-assets
```

`--audio-output` defaults to `data/sounds`, which is already ignored by Git.
`--output` continues to control UI output only. With `--verify`, neither
source DLLs nor MDATA files are read, and no files are written.

| Source | Audio output under `--audio-output` |
| --- | --- |
| `VOICEFXA.DLL` named `WAVE` resources | `voice/alliance/{id}-voicefxa.wav` |
| `VOICEFXE.DLL` named `WAVE` resources | `voice/empire/{id}-voicefxe.wav` |
| `COMMON.DLL` WAVE 8000, 8001, 8002, 8004 | `sfx/menu_galaxy_size.wav`, `menu_load_options.wav`, `menu_quit.wav`, `menu_select.wav` |
| `MDATA.300` through `MDATA.315` | `music/300.wav` through `music/315.wav` |
| `MDATA.300` | Also `music/main_theme.wav` and `music/endor.wav` |
| `MDATA.306`, `.307`, `.312` | Also `music/imperial.wav`, `music/battle.wav`, `music/hoth.wav` |

The soundtrack source files are already WAVs despite their numeric extensions.
All audio is copied byte for byte, without resampling or transcoding. The tool
checks RIFF/WAVE signatures, declared file size, chunk boundaries/padding, and
nonempty format/data chunks. The original Empire voice 15053 has one zero
padding byte outside its odd RIFF length; that padding is accepted and retained. It rejects duplicate DLL resource IDs across
languages. Identical output files remain untouched; differing files require
`--force`, and replacements use atomic file writes. A failed extraction can
leave earlier completed files; rerunning safely resumes them.

Extraction records both factions’ voice IDs in `voice-inventory.json`.
Verification requires every recorded voice clip and the fixed soundtrack/menu
paths, so missing clips fail even without the original DLLs. It validates
structure, not playback, byte identity against the originals, or a canonical
voice ID inventory independent of the extraction.

Victory/defeat music from Smacker movies (`MDATA.201`/`.202`) remains part of the
cutscene decoder. Generic gameplay SFX without established source mappings are
not fabricated. Extraction does not change which voice lines or music cues the
app currently plays. Original audio files must never be committed or distributed
with the source code.

Validate changes to this tool with:

```sh
make fmt-go test-go vet-go
```
