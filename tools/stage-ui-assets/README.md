# stage-ui-assets

Extract the original Star Wars Rebellion UI bitmaps into the directory layout
Open Rebellion loads at runtime. One command stages 2,231 BMPs from four game
DLLs and checks the resulting files. It uses only the Go standard library: no
Python environment, third-party packages, or Windows runtime is needed.

The extractor preserves each resource's original DIB bytes, including its
palette and pixel data, and adds the BMP file header. It does not resize or
re-encode the artwork.

## Requirements

- Go 1.22 or later to build or use `go run`.
- Your own copy of the four original game DLLs listed below, together in one
  source directory. Extraction reads these files without modifying them.

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

If the DLLs are already in `data/base/`, no flags are needed:

```sh
go run ./tools/stage-ui-assets
```

All relative paths, including the defaults, resolve from the current working
directory—not from the executable's location. The output directory is created
as needed. A successful extraction ends with:

```text
Staged 2231 BMPs from 4 DLLs (2231 written, 0 unchanged)
...
Verified 2231 BMPs across 4 DLLs
```

Write and unchanged counts depend on what is already staged.

## Output layout

Each DLL has its own directory, so equal resource IDs in different DLLs do not
collide. Numeric resource IDs become filenames, for example:
`data/base/ui/strategy-dll/BMP/900.bmp`.

| Required source file | Expected BMPs | Directory under `--output` |
| --- | ---: | --- |
| `COMMON.DLL` | 321 | `common-dll/BMP/` |
| `GOKRES.DLL` | 580 | `gokres-dll/BMP/` |
| `STRATEGY.DLL` | 1,042 | `strategy-dll/BMP/` |
| `TACTICAL.DLL` | 288 | `tactical-dll/BMP/` |

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

Unknown named resources and duplicate IDs within a DLL—including IDs shared by
multiple languages—cause an error rather than selecting one silently. All four
DLLs and their expected counts are fixed; there is no single-DLL selection flag.

## Verify or refresh existing assets

Check an existing output directory without reading the source DLLs or writing
files:

```sh
./stage-ui-assets --verify --output ./data/base/ui
```

Verification checks each DLL directory's expected `.bmp` count, canonical numeric
filenames, BMP signature, declared file size, DIB header size, and pixel-offset
bounds. It ignores subdirectories and files without the lowercase `.bmp`
extension. `--source` and `--force` have no effect with `--verify`.

Verification does **not** compare files against the DLLs, check an exact inventory
of resource IDs, decode every pixel, or prove that the game displays the assets.
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
| `--source` | `data/base` | Directory containing the four DLLs |
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

This tool stages bitmap resources only. It does not extract sound, animation,
DAT tables, or EData images, and it does not generate the browser manifest or
runtime pack. The repository's [WASM build script](../../scripts/build-wasm.sh)
consumes `data/base/ui/` for browser packaging. See the
[asset guide](../../agent_docs/assets.md) for the broader pipeline.

## Development checks

From the repository root:

```sh
go test -count=1 ./tools/stage-ui-assets
go vet ./tools/stage-ui-assets
```

Tests construct synthetic PE files and DIB data, so they run without proprietary
game files. They cover resource traversal, named-ID mappings, BMP headers,
runtime output paths, duplicate-ID rejection, preserving or replacing existing
files, temporary-file cleanup, and CLI staging with verification.
