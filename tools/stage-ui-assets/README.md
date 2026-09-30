# stage-ui-assets

Extract original Star Wars Rebellion assets into the directory layout
Open Rebellion loads at runtime. One command stages 2,326 standard BMPs and
3,988 custom advisor frames from seven game DLLs, plus voices, menu effects,
and soundtrack WAVs, 15 cutscenes, and original text strings. It then verifies
all outputs. The Go code uses only its standard library; cutscene conversion
requires `ffmpeg` and `ffprobe`. No Python environment or Windows runtime is needed.

An opt-in tactical-only path also preserves all 87 type-301 DirectX meshes and
397 type-303 texture/palette resources from `TACTICAL.DLL` in a
content-addressed raw store. It does not require the media tools.

An opt-in Encyclopedia-only path extracts the owned English `ENCYTEXT.DLL`
prose and `ENCYBMAP.DLL` EDATA-name table into one ignored, checksummed JSON
source catalog. It uses strict Windows-1252 decoding and rejects unknown
languages, undefined bytes, duplicate IDs, malformed filenames, and traversal.

For standard bitmaps, the extractor preserves the original DIB bytes and adds a
BMP file header. For advisor animations, it preserves each custom PE type-302
resource byte for byte. It does not resize or re-encode the artwork.

## Requirements

- Go 1.22 or later to build or use `go run`.
- `ffmpeg` (with VP9/Opus encoding and Smacker decoding) and `ffprobe` on PATH
  for extraction. Verification needs neither tool.
- Your own copy of the seven UI DLLs listed below plus `VOICEFXA.DLL` and
  `VOICEFXE.DLL` and `TEXTSTRA.DLL`, together in one source directory, and the original
  `MDATA.300`–`MDATA.315` soundtrack files and the 15 movies listed below in `source/MDATA` or `--mdata`. Extraction reads these files without modifying them.
- `--encyclopedia-only` additionally requires `ENCYTEXT.DLL` and
  `ENCYBMAP.DLL` in `--source`; it does not require ffmpeg.

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

If the DLLs are in `data/base/` and the soundtrack and movies are in `data/base/MDATA/`,
no flags are needed:

```sh
go run ./tools/stage-ui-assets
```

All relative paths, including the defaults, resolve from the current working
directory—not from the executable's location. The output directory is created
as needed. A successful extraction reports:

```text
Staged 6291 UI resources from 6 DLLs (6291 written, 0 unchanged)
...
Verified 6291 UI resources across 6 DLLs
Staged 310 audio files (310 written, 0 unchanged)
Verified 310 audio files
Staged 1347 TEXTSTRA strings
...
Verified 1347 TEXTSTRA strings
Verified cutscene 000 (259 frames)
...
```

Write and unchanged counts depend on what is already staged.

## Encyclopedia research report

Generate only the encyclopedia source-research product, without checking
for or invoking `ffmpeg`/`ffprobe` and without staging the normal UI, audio,
strings, or cutscenes:

```sh
go run ./tools/stage-ui-assets --encyclopedia-report-only \
  --source "/path/to/Star Wars - Rebellion"
```

The default destination is `data/base/encyclopedia-research/`. Override it with
`--encyclopedia-report-output`. Artwork defaults to `<source>/EData`; use `--edata` to
select a different declared EData root. This focused mode reads `ENCYTEXT.DLL`
and `ENCYBMAP.DLL`, inventories the declared EData directory, and publishes a
validated, deterministic directory transaction containing:

```text
source-report.json
raw/encytext/<language>/<numeric-id>.bin
raw/encytext/<language>/<numeric-id>.txt
raw/encytext/<language>/named/<reversible-hex-name>.bin
assets/EDATA.NNN
```

The JSON discriminator is `kind: "encyclopedia-research"` with
`schema_version: 1`. Every supplied text resource is inventoried with its
numeric or named identity, LANGID, PE code page, byte length, SHA-256, and
interpretation status. Raw `.bin` bytes are preserved exactly. A `.txt` sibling
is emitted only when the complete source DLL identity matches an embedded,
reviewed lossless decoder profile; otherwise the record is explicitly
`unresolved` and no text is guessed. Named identifiers are UTF-8 hex encoded in
a separate namespace, with long encodings split across bounded path components,
so names never become unchecked paths or collide with numeric IDs.

The same report carries language-qualified RT_STRING lookup evidence, missing
references, duplicate references, case ambiguity, supplied/unreferenced image
identities, and measured BMP facts. Every valid supplied `EDATA.NNN` is copied
byte-for-byte, including gaps and unreferenced files. A supplied filename never
creates a topic, selector, or runtime allowlist entry; for example, an
unreferenced alternate remains inventory-only. Case-ambiguous identities are
rejected rather than selecting a spelling. Images are limited to 32 MiB each
and 128 MiB in aggregate, with the aggregate checked before image reads or
copies. Unsupported, corrupt, colliding, or over-budget images fail the
transaction and leave the previously published owned set intact.

The report is research evidence, not a runtime catalog. Report mode refuses a
destination containing runtime `catalog.json` or `manifest.json`, even with
`--force`. It also refuses source/EData and declared mod-root collisions. This
checkpoint does not bind strings or images to topics and does not produce a
runtime-loadable catalog.

Verify an existing report without reading an original installation or writing
anything:

```sh
go run ./tools/stage-ui-assets --encyclopedia-report-only --verify \
  --encyclopedia-report-output ./data/base/encyclopedia-research
```

Verification rechecks the report schema, exact generated-file ownership, raw
lengths and hashes, every proven text decode, and all staged image bytes and
measurements. It does not need or read `--source` or `--edata`. Active or
interrupted publication returns a recovery command but verification never
acquires a writer marker or repairs, renames, or deletes transaction files. A
staging rerun may perform validated recovery. Byte-identical reruns are no-ops,
including with `--force`; changed owned output requires `--force`; unknown user
files always block replacement and are never deleted.

The default research destination is covered by `data/base/*` in `.gitignore`.
Keep custom research destinations outside tracked paths: original prose, raw
resources, and generated reports must not be committed or distributed.

## Canonical encyclopedia catalog

Generate the runtime catalog and its complete local research evidence without
checking or invoking `ffmpeg`/`ffprobe`:

```sh
go run ./tools/stage-ui-assets --encyclopedia-canonical-only \
  --source "/path/to/Star Wars - Rebellion" \
  --encyclopedia-canonical-output ./data/base/encyclopedia
```

The default canonical destination is `data/base/encyclopedia/`. Artwork
defaults to `<source>/EData`; pass `--edata` when EData is elsewhere. The
selected profile's DAT files may be flattened beside the DLLs or kept in
`<source>/GData`. `REBEXE.EXE` is research evidence but is not required for
catalog extraction. The built tool embeds its reviewed profile and therefore
works from an unrelated current working directory.

The canonical directory contains two products with separate ownership:

```text
catalog.json                         # strict runtime wire catalog
manifest.json                        # catalog + referenced-runtime-file hashes
assets/EDATA.NNN                     # every valid supplied original, byte exact
source-report.json                   # local extraction/ownership evidence
raw/encytext/<language>/<id>.bin
raw/encytext/<language>/<id>.txt
```

`manifest.json` lists `catalog.json` and only the artwork referenced by the
catalog; it never hashes itself. The source report owns raw text and supplied
but unreferenced artwork. Those local evidence files are not runtime inputs and
are not copied into a browser runtime mirror. In particular, the deferred
unproven alternate `EDATA.192` remains local inventory only.

Canonical extraction requires an exact supported DLL/DAT profile. It verifies
the actual selected DAT hashes before applying reviewed bindings, fully
validates the candidate catalog and BMP bytes, and then uses the same directory
transaction as report mode. An identical rerun is a no-op, including with
`--force`; changed generated bytes require `--force`; unknown files always
block replacement. A failure restores the prior validated directory.

Verify a runtime-only directory without the original DLLs, DATs, raw records,
or report:

```sh
go run ./tools/stage-ui-assets --encyclopedia-canonical-only --verify \
  --encyclopedia-canonical-output ./data/base/encyclopedia
```

This checks the catalog, manifest, exact runtime allowlist, referenced BMP
bytes, digests, formats, dimensions, and resource budgets. It does not claim
that a current installation's DAT files match the manifest; extraction and the
later package adapter perform that separate pairing check. If a source report
is present, verification additionally checks its complete generated ownership
and raw/decoded evidence. Report-only and canonical roots are never silently
reinterpreted as one another.

### Normal browser builds

`scripts/build-wasm.sh` can run the focused canonical stage before compiling
when an owned source root is supplied explicitly:

```sh
REBELLION_ENCYCLOPEDIA_SOURCE="/path/to/Star Wars - Rebellion" \
REBELLION_EDATA_DIR="/path/to/Star Wars - Rebellion/EData" \
FORCE_REBUILD=1 \
./scripts/build-wasm.sh
```

`REBELLION_ENCYCLOPEDIA_SOURCE` identifies the reviewed DLL/DAT profile;
`REBELLION_EDATA_DIR` is only the Go stage's explicit artwork input. The packer
never recursively injects that EData directory. `REBEXE.EXE` is not required.
The canonical stage defaults to `data/base/encyclopedia/` and may be overridden
with `REBELLION_ENCYCLOPEDIA_STAGE`.

The build publishes `web/data/runtime.orpk` and
`web/data/encyclopedia/` from the same immutable, verified byte generation.
The loose mirror contains only `catalog.json`, `manifest.json`, and the
manifest-referenced art. It excludes the source report, raw evidence, and
unreferenced inventory such as deferred EDATA.192. `package-web.sh` continues
to ship the ORPK and its artifact hashes; the mirror is the ignored development
fallback.

Before the later production-route gate, a completely absent canonical stage
warns and publishes a pack without the namespace. A report-only, partial, or
corrupt stage always fails. Tests and downstream release tooling can select the
future strict policy with `REBELLION_REQUIRE_ENCYCLOPEDIA=1`; it is deliberately
not the default here.

The pack and mirror use one sibling transaction record and an exclusive writer
lock. Changed publications back up both prior artifacts, publish both
candidates, then retire the backups only after the completed inventory is
durable. The lock is opened without following links and its stable regular-file
identity is checked before use. The journal binds candidates and backups to
their exact device/inode, byte digest, and complete file/directory-tree
inventory. Recovery validates every surviving artifact before mutation,
reconciles operations that completed before an interruption, and can be
restarted before or after every restore, cleanup, or journal update. Unknown
mirror, backup, or candidate files, directories, and symlinks are never
removed, including with `FORCE_REBUILD=1`. An interrupted writer is recovered
by rerunning the same build command; a live writer reports a busy diagnostic.
If recovery reports a missing or unrecognized artifact, preserve the sibling
publication files and inspect them rather than deleting either output manually.
Pack, mirror, source-input, lock, journal, backup, and candidate paths must be
canonical and non-colliding; aliases and unsafe path types fail before the
writer lock or output candidates are created.

The container path performs the same stage before `build-wasm.sh`, even with
`PREPARE_MODDING=0`:

```sh
ORIGINAL_GAME_DIR="/original-game" \
REBELLION_EDATA_DIR="/original-game/EData" \
PREPARE_MODDING=0 FORCE_REBUILD=1 \
./scripts/docker-build.sh
```

It copies the existing flattened DLL/DAT inputs, passes the owned EData root
through Go's `--edata`, and propagates the force and rollout policy flags. It
does not depend on optional DAT JSON dumps.

Normal full staging runs the same canonical stage before media prerequisites.
For the current rollout, entirely absent encyclopedia-specific inputs emit a
warning and let the remaining assets stage. Any partial, corrupt, mixed, or
unsupported encyclopedia input fails instead of being skipped.

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
| `REBDLOG.DLL` | 23 | 0 | `rebdlog-dll/` |

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
multiple languages—cause an error rather than selecting one silently. The one
exception is `REBDLOG.DLL`'s `DLG_CORNER_GRAB_FRAME`, which `REBEXE.EXE` never
loads by name; it is skipped rather than given an invented ID. All seven
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
| `--source` | `data/base` | Directory containing UI, voice, and TEXTSTRA DLLs |
| `--output` | `data/base/ui` | Root of the staged UI directories |
| `--audio-output` | `data/sounds` | Audio output directory |
| `--mdata` | `source/MDATA` | Original soundtrack and cutscene directory |
| `--edata` | `source/EData` | Original encyclopedia artwork directory (report, canonical, or full mode) |
| `--strings-output` | `data/base/textstra.json` | Original string JSON output |
| `--encyclopedia-output` | `data/base/encyclopedia/source.json` | Ignored ENCYTEXT/ENCYBMAP source catalog |
| `--cutscene-output` | `assets/references` | Parent for `ref-videos` and `cutscene-frames` |
| `--encyclopedia-report-only` | `false` | Stage or verify only the encyclopedia source research report |
| `--encyclopedia-report-output` | `data/base/encyclopedia-research` | Research report destination (requires report-only mode) |
| `--encyclopedia-canonical-only` | `false` | Stage or verify only the canonical encyclopedia bundle |
| `--encyclopedia-canonical-output` | `data/base/encyclopedia` | Canonical bundle destination for canonical-only and full modes |
| `--verify` | `false` | Check existing output without extraction |
| `--encyclopedia-only` | `false` | Stage or verify only the ENCYTEXT/ENCYBMAP source catalog |
| `--force` | `false` | Replace files whose contents differ |
| `--tactical-3d` | `false` | Add tactical type-301/type-303 staging to the full extraction |
| `--tactical-3d-only` | `false` | Stage or verify only tactical type-301/type-303 resources |
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

This tool stages UI resources, audio, cutscenes, original text strings, and
opt-in raw tactical meshes and textures. Its separate tactical converter
decodes staged binary-X geometry and type-303 indexed images and palettes into
a deterministic content-addressed runtime store. It does not extract SPT/BIN/FDT control data,
briefing animation, DAT tables, or EData images. It does not generate the browser
manifest or runtime pack. The repository's
[WASM build script](../../scripts/build-wasm.sh) consumes `data/base/ui/` for
browser packaging. See the
[asset guide](../../agent_docs/assets.md) for the broader pipeline.

## Tactical 3D raw staging

Stage only the original tactical 3D resources without requiring other DLLs,
soundtrack files, cutscenes, `ffmpeg`, or `ffprobe`:

```sh
go run ./tools/stage-ui-assets --tactical-3d-only \
  --source "/path/to/Star Wars - Rebellion" \
  --output ./data/base/ui
```

Verify an existing tactical raw store without reading `TACTICAL.DLL`:

```sh
go run ./tools/stage-ui-assets --tactical-3d-only --verify \
  --output ./data/base/ui
```

The output is `tactical-dll/TACTICAL3D/manifest.json` plus
`TACTICAL3D/objects/{sha256}.bin`. The manifest records resource type, numeric
or exact named identifier, language, code page, reserved value, size, object
hash, and source DLL hash. Resource names never become filesystem paths.
Extraction rejects path separators, control characters, case-folding
ambiguities, duplicate identifiers, unsupported X headers, unexpected counts,
and bounded-size violations. Verification rehashes every object and checks the
expected 87 type-301 and 397 type-303 records.

The source DLL is read once into a bounded snapshot, and that same snapshot is
hashed and parsed. Tactical traversal applies count, per-resource, and aggregate
limits before payload copies. Verification requires regular object files and
uses bounded reads before hashing them.

Convert a verified raw store without rereading `TACTICAL.DLL`:

```sh
go run ./tools/stage-ui-assets --tactical-3d-convert \
  --output ./data/base/ui
```

Verify an existing converted store without reading its raw source objects:

```sh
go run ./tools/stage-ui-assets --tactical-3d-convert --verify \
  --output ./data/base/ui
```

Independently compare every owned original mesh with pinned Assimp 6.0.5 raw
output, including connectivity, positions, normals, UVs, materials, texture
names, and the explicit handedness and UV-origin transforms:

```sh
go run ./tools/stage-ui-assets \
  --tactical-3d-assimp-oracle /path/to/assimp-6.0.5/bin/assimp \
  --output ./data/base/ui
```

The oracle mode is a development gate, requires the audited original
`TACTICAL.DLL`, and never becomes a runtime dependency.

The converted output is `TACTICAL3D/runtime/manifest.json` plus hashed `.mesh`
and `.texture` objects. It retains source and object hashes, material and named
texture links, original indexed pixels, the dynamic battle-palette rule, all
27 planet palettes, and bounded source-specific exceptions. Both raw and
converted stores remain ignored and must originate from an owned installation.
Neither is included in the browser runtime pack yet. Browser transport,
camera rules, and semantic entity identities remain in the subsequent passes
of the [tactical 3D asset plan](../../docs/plans/2026-09-12-feat-tactical-3d-asset-pipeline.md).

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
temporary-file cleanup, tactical snapshot and size bounds, raw-manifest
reproducibility and repair, and CLI staging with verification.

## Audio extraction

Every extraction stages the original voice lines, four menu effects, and all
16 soundtrack files alongside the UI assets, cutscenes, and strings. Verification
checks all of these by default. No audio opt-in flag is needed. Audio extraction also uses
only the Go standard library.

From the repository root:

```sh
make extract-assets GAME_SOURCE="/path/to/Star Wars - Rebellion"
```

`GAME_SOURCE` must contain the seven UI DLLs plus `VOICEFXA.DLL`,
`VOICEFXE.DLL`, and `TEXTSTRA.DLL`. `MDATA_DIR` defaults to `GAME_SOURCE/MDATA`.
If the DLLs have already been copied into `data/base`, point at the original media directory:

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

Audio from Smacker movies (`MDATA.201`/`.202` included) is extracted with the
cutscenes into their WAV sidecars. Generic gameplay SFX without established source mappings are
not fabricated. Extraction does not change which voice lines or music cues the
app currently plays. Original audio files must never be committed or distributed
with the source code.

Validate changes to this tool with:

```sh
make fmt-go test-go vet-go
```


## Cutscenes and original strings

Normal extraction includes these assets, and `make verify-assets` checks them
along with UI and audio. There are no per-family enable flags.

`TEXTSTRA.DLL` RT_STRING bundles become `data/base/textstra.json`, using the same
numeric string IDs and JSON object shape as the browser's existing string loader.
UTF-16 and bundle bounds are checked; duplicate language bundles are rejected.
A `.manifest.json` sidecar records the output count and SHA-256 so verification
can detect missing or changed text without reopening the DLL. Browser packaging
can use `--strings-output web/data/base/textstra.json` for its staging directory.

For the original English Encyclopedia source profile:

```sh
go run ./tools/stage-ui-assets \
  --encyclopedia-only \
  --source "/path/to/Star Wars - Rebellion"
```

This writes ignored `data/base/encyclopedia/source.json` plus a manifest with
the catalog and source-DLL checksums. `--verify --encyclopedia-only` validates
those files without reopening the DLLs. The source catalog does not enable the
production window by itself; runtime topic binding and UI acceptance remain
separate gates.

The supported movie IDs are `000`, `001`, `003`, `004`, `005`, `101`, `102`, `103`,
`104`, `105`, `106`, `107`, `108`, `201`, and `202`. For each original `MDATA.ID`,
the tool uses ffmpeg/ffprobe to produce:

- `assets/references/ref-videos/ID.webm`: VP9 video with Opus audio for browser playback.
- `assets/references/cutscene-frames/ID/frame-00001.png` onward: native playback frames.
- `assets/references/cutscene-frames/ID/metadata.json`: dimensions, FPS, and frame count.
- `assets/references/cutscene-frames/ID.wav`: decoded PCM audio for native playback.
- `assets/references/cutscene-frames/ID/extraction.json`: source SHA-256, extraction
  version, metadata, and output file hashes for verification and reuse.

PNG frames and WAV sidecars are decoded from the generated WebM, matching the
existing native pipeline. Decoding and validation finish in a temporary output
before publication. A decoder failure leaves prior assets in place. Individual
outputs are published by rename and the frame directory/completion record last.
A reported publication error rolls back all three outputs; if restoration itself
fails, backups are retained and their location is reported. A process crash
during publication may still leave an incomplete set, which verification rejects.

Verified cutscenes with an unchanged source are reused, including when `--force`
is supplied. Changed, damaged, or legacy outputs without an extraction record
require `--force` before replacement. The first complete run can take tens of
minutes and uses additional disk space for decoded PNG frames.

Verification checks expected movie IDs, metadata, every recorded frame, WebM,
and WAV checksum without accessing the original media or invoking ffmpeg.
This is file integrity validation, not proof of visible or audible playback.
Generic gameplay SFX without known mappings and other unlisted original resources
remain outside the supported extraction inventory.
