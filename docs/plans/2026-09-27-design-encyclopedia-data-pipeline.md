---
title: "Encyclopedia Data Extraction, Modding, and Display"
description: "Proposed source-derived encyclopedia catalog, asset staging, mod overlays, and native/browser consumption"
type: design
status: draft
created: 2026-09-27
updated: 2026-10-02
tags: [encyclopedia, assets, modding, native, wasm, P35, RE-ENC-01]
---

# Encyclopedia Data Extraction, Modding, and Display

The
[Will Forster Encyclopedia Handoff Adaptation](2026-10-02-feat-will-forster-encyclopedia-handoff-adaptation.md)
is the execution companion for selectively bringing PR #16's strongest ideas
into this canonical design without creating a second schema or runtime.

## Implementation checkpoint

P62 implements the first bounded transport slice: strict `EDATA.NNN`
validation, a namespaced ORPK v3 payload, dedicated browser artwork cache,
lazy nearest-neighbor decoding, native original-install path resolution, and
exact missing-asset diagnostics. See the
[P62 evidence record](../qa/2026-09-10-interface-parity-audit/evidence/2026-09-28-encyclopedia-artwork-transport.md).
The dedicated muted browser gate also proves `EDATA.042` at native 400x200
size with all 80,000 source pixels matching. Its test-only renderer is absent
from production and does not stand in for an encyclopedia window.

P66A implements the local English source-data gate. The dependency-free Go
stager now extracts all 348 `ENCYTEXT` type-10 records and all 191 logical
`ENCYBMAP` strings into an ignored, checksummed catalog with strict
Windows-1252 decoding. The typed Rust join reproduces the ordinary low-12-bit
plus `0x1000` lookup, the 26-value system-picture switch, and the factional
mission-art offset. It resolves 346 of the 356 index objects for each faction
and reports the ten source-empty mission records without fallback content. All
186 mapped EDATA filenames exist in the 187-file owned set; unreferenced
`EDATA.192` remains outside the proven lookup table. See the
[P66A evidence record](../qa/2026-09-10-interface-parity-audit/evidence/2026-10-01-encyclopedia-topic-source-bindings.md).

Browser packaging and installation, topic composition, contextual and
production routing, exact previous/next behavior, mod overlays, and visual A0
acceptance remain planned. Command `0x131` therefore remains fail-closed, and
no `OBJ-01` acceptance cell passes from P62 through P66A alone.

## 1. Purpose and scope

Extract the original encyclopedia into readable, structured UTF-8 data, stage
its artwork predictably, and have the encyclopedia display that data on native
and browser builds. Mod authors should change text and replace artwork without
editing DLLs or Rust. Re-extraction must never overwrite a mod author's work.

This began as a proposed design at repository baseline
`e101e6c7bc74ec75487e16d81b1c2bb55025562c`. P62, P64, P65, and P66A now
implement the transport, index shell, index catalog, and local source-binding
checkpoints described above. Sections that still describe browser topic
transport, composition, navigation, contextual entry, mod overlays, or
production routing remain future work and confer no acceptance by themselves.

The first delivery includes original-data extraction, a validated catalog,
native and browser asset loading, native mod overrides, and an original-style
encyclopedia index/topic view. Browser mod discovery/upload/hot reload remains
outside this delivery, consistent with the current mod runtime. Both platforms
must display the same unmodified base catalog. Further language packs, custom
topics and browser mod installation are extension points, not implicit features.

The data pipeline can land in independently reviewable slices before the full
original UI is ready. Do not enable the currently gated cockpit/F7 route merely
because a JSON file parses: the selected UI slice must have source mappings,
working navigation, and inspected native/browser evidence.

## 2. Existing contracts to follow

| Existing system | Reuse or extend |
|---|---|
| [Go staging tool](../../tools/stage-ui-assets/README.md) | Reuse bounded PE-resource parsing, string decoding, hashes, identical-output no-op behavior, `--force` and `--verify`. Add a focused encyclopedia mode. |
| [TEXTSTRA staging](../../tools/stage-ui-assets/strings.go) | Follow resource-to-JSON conversion; preserve language rather than merging different languages into one map. |
| [DAT dumping](../../README_MOD.md) | Keep reference dumps in ignored `data/base/json/`. They identify entities but are not the runtime encyclopedia catalog. |
| [Browser pack builder](../../scripts/build-runtime-pack.py) and [Rust reader](../../crates/rebellion-app/src/runtime_pack.rs) | Extend the existing ORPK v3 transport and validation, retaining one packed startup fetch. |
| [Mod loader/runtime](../../crates/rebellion-data/src/mods.rs) | Reuse `mod.toml`, enabled mods, dependency resolution, diagnostics and RFC 7396 merge semantics. Add an explicit content target outside `GameWorld`. |
| [Encyclopedia renderer](../../crates/rebellion-render/src/encyclopedia.rs) | Replace approximate image selection and the WASM texture stub with catalog lookups and shared image bytes. |
| [Faithful-HD policy](../../crates/rebellion-render/src/bmp_cache.rs) | Original images remain the default; HD substitutions still require the existing profile, manifest and digest checks. |
| [Interface RE ledger](../qa/2026-09-10-interface-parity-audit/reverse-engineering-ledger.json) | RE-ENC-01 / OBJ-01 supplies the source and capture gate; P35 remains unaccepted until that gate passes. |

The old [assets sketch](../../agent_docs/assets.md#pipeline-3-encyclopedia-content)
proposed `dat_id -> { name, description, faction, category }` in
`data/encyclopedia.json`, including newly written descriptions. This design
supersedes that sketch with extraction of original descriptions, explicit
resource provenance, and family-qualified entity bindings. It does not replace
existing DAT or save schemas.

### Alternatives considered

1. **Read DLLs during gameplay.** Minimal staging, but introduces native/browser
   divergence and leaves mod authors dependent on binary editing. Reject.
2. **Put descriptions into `GameWorld` and its existing JSON arena patches.**
   Superficially convenient, but presentation assets would enter saves and
   simulation fingerprints. Reject.
3. **Stage a separate catalog and consume it through existing asset/mod paths.**
   Recommended: one base representation for both platforms, traceable extraction,
   editable overrides, and no change to simulation ownership.

## 3. Source data and evidence boundaries

The owned installation root is the directory containing the DLLs, `EData/` and
usually `GData/`. The current local installation is `data/base/`; staging must
also accept an external installation without copying the entire installation.
Do not assume the DLL root, DAT directory and staging destination are identical.

| Source | Actual role | Extraction rule |
|---|---|---|
| `ENCYTEXT.DLL` | Encyclopedia prose in PE type 10 (`RT_RCDATA`) resources | Preserve numeric resource ID, Windows language ID, original bytes/hash and decoded text. |
| `ENCYBMAP.DLL` | Image filename lookup strings in PE type 6 (`RT_STRING`) blocks | Decode length-prefixed UTF-16LE; logical string ID is `(block_id - 1) * 16 + slot`. Block ID is not topic ID. |
| `EData/EDATA.NNN` | Original topic artwork, BMP bytes despite the extension | Copy original bytes, validate header/dimensions, keep the file number as asset identity. |
| `TEXTSTRA.DLL` | Entity display names | Reuse extraction; name selection must follow verified entity/topic bindings. |
| `GData/*.DAT` | Entity/class records and stable numeric IDs | Join using entity family and `DatId`, never a slotmap key, name or vector index. |
| `REBEXE.EXE` | Topic/category construction, navigation, lookup selection and state-dependent variants | Recover metadata contracts; do not execute the original binary in the staging tool. |
| Existing UI DLLs such as `COMMON`, `STRATEGY`, `GOKRES` | Index/topic chrome and control images where proven | Reuse `data/base/ui/<dll>/BMP/<id>.bmp` and `BmpCache`; recover exact IDs before drawing. |

### Inspection performed for this design

Read-only PE inspection of the owned English installation on 2026-09-27 found:

- `ENCYTEXT.DLL`: 348 type-10 records, language 1033, resource code-page field 0.
  All records end in NUL; 29 contain non-ASCII bytes.
  SHA-256: `49aea545a5e09e5fe9115a22bc785690f103d2f931e08bd4a53a617a42636d8c`.
- `ENCYBMAP.DLL`: 31 type-6 blocks, language 1033, code-page field 0;
  191 nonempty logical strings reference 186 distinct filenames. Lookup key
  4736 resolves to `EDATA.014`.
  SHA-256: `fb545d19ae24b0277753494dbfaabf2dbdde660beab821287a32016c290e4560`.
- The committed [EData inventory](../reference/asset-library/edata-inventory.json)
  records 187 images of 400×200 pixels, with gaps in the 1–192 number range.
  Image count, mapping count and text count therefore are not interchangeable.

These are installation-specific observations, not universal count constraints.
No original descriptions or pixel data are reproduced in this document.

### Approved first-profile alternate-image scope

The user explicitly deferred unproven alternate-image support on 2026-09-29.
The first supported profile publishes only standard topic images and the
source-proven faction and system selectors. `EDATA.192` and any other unproven
alternate artwork remain inventoried with unresolved provenance but unused: no
catalog binding, campaign predicate, runtime switch, or alternate-image
acceptance claim is permitted. Asset existence does not establish original
encyclopedia display.

This deferral is tracked by `orlocal-2kq`. It is not proof that an alternate was
implemented, recovered, absent from original behavior, or accepted visually,
and it is not a blocker for E08, schema freeze, first-profile publication, or
first-profile UI acceptance. Retain compatible `variant` and image-identity
fields so later source-proven work can extend the catalog without fabricating a
current binding. All standard/faction/system selector, identity, availability,
context, navigation, and capture requirements remain unchanged.

### Required source work before freezing schema v1

1. Trace `FUN_0045d400` and its callers, using
   [entity graphics evidence](../reference/asset-library/entity-graphics.md) and
   the project [Ghidra workflow](../../agent_docs/ghidra-re.md). The documented
   class-key arithmetic is a useful lead, not proof of every DAT-to-topic join.
2. Establish the byte encoding and control-character rules of ENCYTEXT.
   Code page 0 does **not** prove UTF-8 or Windows-1252. Corroborate the decoder
   against the original executable and the 29 non-ASCII records. Reject unknown
   source profiles rather than using lossy replacement characters.
3. Recover category IDs, labels, ordering, topic titles, previous/next behavior,
   all entity families, system mappings, and every image selector supported by
   the first profile. Do not extrapolate the renderer's current family offsets.
   Unproven alternate art, including the proposed alternate Luke case, follows
   the approved deferral above rather than blocking schema v1.
4. Account for every extracted resource: bound to a topic, a documented alias,
   or explicitly unresolved with a reason. An unresolved item may remain in the
   extraction report but cannot silently become a fabricated runtime binding.
5. Recover availability/context rules separately from the static catalog. Being
   present in a DLL does not establish that a topic is visible in every campaign
   state, to both factions, or from every entry point.

For unsupported in-scope controls/encoding or uncertain joins, retain raw bytes
in the local extraction report and stop publication of the affected category.
Do not substitute externally written lore. Deferred alternate images remain
inventoried and unbound rather than stopping publication of an otherwise proven
first-profile category. A partial category implementation is labeled partial
and must not claim full P35 acceptance.

## 4. Proposed schema v1

Use one canonical `catalog.json` with a versioned envelope and keyed objects.
This is both readable by mod authors and simple to load into typed Rust maps.
A separate `manifest.json` records extraction provenance and file digests; it is
not a mod-editable catalog. Raw-resource reports are tooling artifacts only.

### Catalog fields

| Field | Proposed type and meaning |
|---|---|
| `schema_version` | Integer `1`; reject unsupported versions rather than guessing. |
| `default_language` | Decimal Windows LANGID string from the source profile, e.g. `"1033"`. |
| `categories` | Map of stable source-derived category keys to localized labels and ordered `topic_ids`. The explicit arrays determine display order. |
| `topics` | Map keyed by `original:<ENCYTEXT-resource-id>` for original topics. A source-profile mapping must prove that a record is a topic; explicit aliases handle shared records. |
| `images` | Map keyed by `edata:<number>` for original artwork, containing relative `path`, `format`, `width`, `height`, and lowercase `sha256`. |
| `bindings` | Array joining `{family, dat_id, variant}` to `topic_id`; family names match world arenas where possible. Default variant is `"default"`. |

A category record contains `labels` (LANGID-to-string), `topic_ids` (ordered,
unique within that category) and a `source_ref` into the extraction manifest.
A topic record contains `category_id`, `localized` (LANGID-to-content map), and
`source_ref`. A localized content record contains `title`, `body`, and optional `image_id`
(string; missing or null both mean no artwork). This allows localized artwork without duplicating topic IDs.
Each topic belongs to one category in v1; any original cross-category aliases
must be represented and source-proven before schema freeze, not silently lost.

`body` is plain Unicode text, not HTML or Markdown. Normalize line endings to
LF, remove only the proven terminal NUL/padding, and preserve meaningful spacing
and paragraph breaks. If the original format contains control sequences, the
source gate must specify their representation before v1 is finalized; arbitrary
control bytes must not reach the renderer as printable prose.

Bindings use family-qualified IDs because DAT IDs are not globally unique.
Class-backed entries bind to class records, not individual fleet instances.
Merged character arenas also require verification of their DAT ID namespace;
if major/minor IDs overlap, add an explicit source-table discriminator before
schema freeze. Unknown or ambiguous bindings are errors, not first-match wins.
A variant key is a named, source-proven case selected by typed application code;
no expression evaluator or mod-supplied executable predicates are introduced.
Live statistics remain read from the bound `GameWorld` record.

The language resolver selects a whole localized record: requested LANGID, then
catalog default. It does not combine a title in one language with a body in
another. Missing both records disables that topic with a diagnostic. Choosing
another language is a catalog-loading parameter, not a new settings UI here.

### Synthetic catalog example

All names, IDs and content below are fictional test data, not recovered joins.
The all-zero digest illustrates the field shape only; a real staged file must
carry and pass its computed digest.

```json
{
  "schema_version": 1,
  "default_language": "1033",
  "categories": {
    "ships": {
      "labels": {"1033": "Test ships"},
      "topic_ids": ["original:60001"],
      "source_ref": "fixture/category/ships"
    }
  },
  "topics": {
    "original:60001": {
      "category_id": "ships",
      "localized": {
        "1033": {
          "title": "Example cruiser",
          "body": "Synthetic description.\n\nA second paragraph.",
          "image_id": "edata:42"
        }
      },
      "source_ref": "fixture/topic/60001"
    }
  },
  "images": {
    "edata:42": {
      "path": "assets/EDATA.042",
      "format": "bmp",
      "width": 400,
      "height": 200,
      "sha256": "0000000000000000000000000000000000000000000000000000000000000000"
    }
  },
  "bindings": [
    {"family": "capital_ship_classes", "dat_id": 7, "variant": "default", "topic_id": "original:60001"}
  ]
}
```

### Provenance manifest and validation

`manifest.json` has `schema_version: 1`, `source_profile`, `extractor_version`,
`catalog_sha256`, a runtime relative-file-to-digest map (catalog and referenced images only),
`binding_sources` (DAT basenames and SHA-256 digests used to derive bindings),
and `source_records` keyed by
`source_ref`. Source records identify DLL basename/hash, resource type,
resource ID (or original name), LANGID, raw resource hash, decoder/encoding, and
mapping evidence (native function/table or reviewed mapping record). Keep
absolute installation paths and extraction timestamps in a separate local log,
so identical inputs produce identical staged bytes.

The proposed checked-in schemas live at
`docs/reference/asset-library/schemas/encyclopedia-{catalog,manifest,overlay}.schema.json`.
They validate structure; Rust/Go validators also enforce relationships and
hashes. Neither Go nor Rust should require a new production JSON-Schema engine:
use existing JSON facilities plus typed validation, and test shared synthetic
fixtures against both implementations. Reject duplicate JSON keys before map
insertion, unknown fields, invalid UTF-8, duplicate binding tuples, dangling
references, missing source records, unsupported formats and invalid hashes.

Paths are slash-separated and relative to their declared asset root. Reject
absolute paths, drive prefixes, `..`, backslashes and symlinks escaping that root.
All image files must decode, match their declared dimensions and pass a pixel
count/byte-size bound before texture allocation. V1 original images are BMP;
mod replacements may be BMP or PNG, using already-supported decoders.
Proposed initial safety limits: 10,000 topics, 1 MiB UTF-8 per localized body,
32 MiB per image, 16 million pixels per image, and 128 MiB aggregate staged image
bytes. Verify these generous limits against real extraction and record any
change; do not rely solely on trusting a manifest's dimensions.

## 5. Extraction and staging layout

Extend `tools/stage-ui-assets`; do not introduce a second standalone extractor.
Its generic resource reader already retains ID/name/language/code-page metadata.
Its string decoder already understands Windows string blocks but currently
rejects duplicate bundle IDs across languages: group resources by LANGID first,
then reuse the decoder per group without changing TEXTSTRA's existing output.

P66A implements the focused owned-English source extractor and offline verifier:

```bash
go run ./tools/stage-ui-assets \
  --source /path/to/owned-install \
  --encyclopedia-only \
  --encyclopedia-output data/base/encyclopedia/source.json

go run ./tools/stage-ui-assets \
  --encyclopedia-only --verify \
  --encyclopedia-output data/base/encyclopedia/source.json
```

This bounded source catalog is the input to the later runtime/browser schema;
it does not by itself implement the versioned overlay model proposed below.

The focused mode must run before the existing ffmpeg/cutscene prerequisites; a
text/image extraction must not require unrelated media conversion. A later
runtime-transport slice must integrate this stage into the regular full staging
command. `--force` follows the existing explicit-overwrite rule for generated
output; it never reaches `mods/`.

```text
<owned-install>/
  ENCYTEXT.DLL, ENCYBMAP.DLL, TEXTSTRA.DLL, REBEXE.EXE
  EData/EDATA.NNN
  GData/*.DAT

<repo>/
  data/base/encyclopedia/             # ignored native runtime base
    catalog.json
    manifest.json
    assets/EDATA.042                 # original bytes, copied from EData
    source-report.json              # resource inventory, unresolved items
    raw/encytext/<lang>/<id>.bin     # local reversible extraction evidence
  data/base/ui/<dll>/BMP/<id>.bmp    # existing staged window chrome
  data/base/json/                    # existing optional DAT reference dumps
  mods/my-mod/
    mod.toml
    encyclopedia.json               # editable overlay, never generated here
    encyclopedia/assets/cruiser.png
  web/data/encyclopedia/             # ignored loose browser staging mirror
    catalog.json
    manifest.json
    assets/EDATA.042
  web/data/runtime.orpk              # packaged browser transport
```

Resolve DLL/EData inputs from `--source`; resolve DAT inputs from its `GData/`
child or the explicitly documented flattened-install profile. Add an explicit
`--edata` directory override for container builds that stage DLLs separately,
following the existing `--mdata` input pattern. Do not silently
switch between different installations. Validate the declared source profile
and report both resolved input roots before generating bindings.

Extract into a sibling temporary directory, validate all generated files and
bindings, then publish the complete set with rollback on failure. A failed run
must leave the previous catalog usable. Sort object keys and source inventories;
category/topic display arrays retain recovered order. Repeated extraction is a
byte-identical no-op. Source outputs are owned by this stage only; clean up its
stale generated files through the manifest, never an unrestricted directory wipe.
`--verify` is read-only and validates the staged output without needing the DLLs.

Stage every valid supplied EData image (including currently unreferenced ones)
and record unused/missing cases in the report. The source report inventories
raw bytes and unreferenced images separately; those files are not requirements
of the runtime manifest and are not shipped in the browser pack. Package only the catalog's
referenced images. Never construct nonexistent filenames to fill numeric gaps.
The runtime image descriptor points to the staged copy; the original `EData/`
installation remains unchanged. No lossy conversion, upscaling, or recoloring
occurs in the original-parity path.

Commit the extractor, validators, schemas, mapping metadata/source citations,
and synthetic fixtures/examples. Original prose, raw resource bytes, artwork,
installation paths and runtime packs stay ignored. Existing `.gitignore` rules
cover `data/base/*` and `web/data/`; tests must verify that new fixture/example
paths contain only synthetic or contributor-authored material.

## 6. Browser packaging and transport

Extend `collect_entries` in `scripts/build-runtime-pack.py` with an explicit
encyclopedia manifest allowlist, not a recursive "include every file" rule.
Add these ORPK entries using existing kind 0 (`game_files`):

```text
encyclopedia/catalog.json
encyclopedia/manifest.json
encyclopedia/assets/EDATA.042
```

Kind 0 already transports opaque bytes; adding namespaced keys requires no new
kind or binary-format version. The catalog has its own schema version. Do not
pretend EData is a DLL bitmap by inventing a `DllSource` or numeric BMP ID.
Update the installer to remove the `encyclopedia/` namespace into a dedicated
catalog/asset byte store **before** passing remaining entries to `set_file_cache`.
The current DAT reader uses basename keys; putting encyclopedia assets through
that reader would lose their namespace. Its existing behavior stays unchanged.

`build-wasm.sh` stages the validated catalog/manifest and referenced image bytes
under `web/data/encyclopedia/`, and the pack builder repeats the validation.
`package-web.sh` continues to ship `runtime.orpk` and its existing artifact hashes.
The container entry point, [`scripts/docker-build.sh`](../../scripts/docker-build.sh),
already invokes `stage-ui-assets` before `build-wasm.sh`. Extend that invocation
to locate/pass the owned installation's EData directory as well as the staged
DLLs; its current DAT/DLL copy does not stage EData. Use the same extractor and
validation as native staging. This must not depend on `PREPARE_MODDING=1`, which
is optional reference dumping.

Production packaging requires a valid catalog for the advertised encyclopedia
feature. A pack with a corrupt catalog or missing declared image fails package
validation. An older pack lacking the whole namespace may still start the game
with Encyclopedia unavailable and a clear diagnostic. Do not silently show the
old approximate catalog. A partially present namespace is an integrity error.
For development's existing loose-file fallback, fetch the manifest and catalog,
validate them, then fetch only declared images with bounded concurrency. A bad
present pack must not silently fall back to loose assets and conceal corruption.

Base JSON and image bytes are prefetched; GPU textures are decoded lazily for
visible topics and cached by `(asset identity, content digest, render profile)`.
Keep the existing four-request packed startup model; topic navigation must not
fetch another copy of an already installed asset.

## 7. Runtime ownership and display

The native app accepts a selected GData path; `data/base/encyclopedia/` is only
the default staging destination, not an unconditional runtime lookup. Resolve
the catalog beside the selected installation's GData directory (or inside the
documented flattened layout), with an explicit catalog-path override for a
separate staging root. Before binding topics, compare manifest `binding_sources`
against the selected base DAT bytes, before world mods are applied. A missing or
mismatched source set disables encyclopedia bindings with a diagnostic naming
the selected roots; never silently use another installation's catalog. For
browser packs, validate the same pairing against the DAT entries in the pack.
Include alternate-GData and wrong-catalog cases in loader tests.

Proposed ownership follows the existing data/render/app split:

- `rebellion-data::encyclopedia` defines typed catalog/manifest parsing, validation,
  topic/entity resolution and pure overlay application. It accepts bytes and
  has no graphics dependency. Native filesystem adapters and browser preloaded
  bytes feed the same parser.
- `rebellion-app` loads the base content after data/assets are available, applies
  enabled native content overlays, and owns the resulting `EncyclopediaCatalog`
  and asset provider for the application session.
- `rebellion-render::encyclopedia` owns selection, scrolling/navigation state and
  texture handles. It consumes the catalog and an asset-byte provider rather
  than reading DLLs or deriving image IDs from list positions.
- `GameWorld`, save bodies, simulation RNG, replay commands and state fingerprints
  contain no encyclopedia prose, GPU handles, or asset bytes.

At entry, resolve the caller's family-qualified entity if present; otherwise
open the recovered index. Build categories and lists from explicit catalog
ordering, display the selected localized title/body/image, and obtain live
stats from the bound world record. Fleet instances first resolve their class.
Apply original context/availability rules at the app boundary, with a pure,
testable resolver; do not conflate mod-visible text with gameplay knowledge.

Reproduce original category navigation, topic selection, scrolling, previous/
next and Return/close behavior using recovered controls and staged chrome.
Original command `0x131`/F7 and contextual opens should reach the same selection
model once the source/visual gate permits enabling the route. Do not re-enable
the four-tab egui approximation as the final original-parity surface.

The source image ID must be logged with cache-hit status for browser evidence.
Missing optional art uses the source-proven empty-image behavior; never borrow
another entity's picture. A malformed body or missing required title cannot
crash or mutate a campaign. Keep an actionable diagnostic at the loading/UI
boundary. Original images use nearest sampling; existing approved HD behavior
remains opt-in and is not broadened by this change.

On new campaign or save load, retain immutable catalog bytes and re-resolve
world bindings. Do not hold stale entity references across world replacement.
Closing the encyclopedia preserves the caller and does not advance or reset
simulation state as a side effect.

## 8. Mod contract

Keep the existing `mods/<name>/mod.toml`. Reserve the root filename
`encyclopedia.json` as a content overlay target. Its outer structure is an array
of patch objects, matching existing overlay files. Topic selectors are strings
because these are content IDs, not world-arena DatIds; this is an explicit new
target, not an alias that the existing world patcher already understands.

Synthetic example:

```json
[
  {
    "id": "original:60001",
    "localized": {
      "1033": {
        "body": "My replacement description.",
        "image": {"path": "encyclopedia/assets/cruiser.png"}
      }
    }
  }
]
```

`id` selects an existing topic and is removed before applying the patch. V1
allows only `localized.<LANGID>.title`, `body`, and the author-facing `image`
field. Omitted values inherit the base. `image` is either `{ "path": "..." }`
relative to that mod's root, or null to explicitly remove optional art. The
adapter validates the file, computes its digest/dimensions, creates a
`mod:<mod-name>:<relative-path>` image descriptor, and translates `image` into
canonical `image_id` before RFC 7396 merge. Authors do not maintain image hashes.
Null title/body/language deletions follow merge-patch semantics but are rejected
if they leave a required localized record invalid. In particular, removing a
body is not the same as deliberately setting it to the empty string.

Forbid changing source IDs, bindings, categories, provenance, schema version or
base image descriptors through this overlay. Reject unknown selectors/fields
and duplicate topic selectors within one file. New topics/categories are a
future version requiring explicit ordering and binding semantics; do not turn
a typo into an implicit topic creation.

`ModContent::from_dir` currently scans every root JSON file and assumes a patch
array, while `ModLoader::apply` targets `GameWorld` arenas. Explicitly split out
the encyclopedia target before world application so it cannot be skipped as an
unknown arena or serialized into saves. Resolve the enabled dependency order
once and use that same order for world and encyclopedia content. Later overlays
win per field; authors declare a dependency for intentional overriding. Require
a deterministic name tie-break for unrelated mods in the shared resolver, with
regression tests for existing world overlays rather than a second sorting rule.

Validate base content against its immutable manifest before any overlays.
Effective mod image descriptors are verified against their owning mod files,
not inserted into or checked against the original base manifest.

Apply one mod's encyclopedia changes to a copy, validate every patch and image,
then publish the batch atomically. On failure keep the previous valid content
and report the mod/topic/path through existing mod diagnostics. Recompute from
the immutable base when toggling or reloading, so disabling a mod restores the
original text. Extend the native watcher to include declared image assets;
invalid edits retain the last good snapshot, while removed/disabled mods trigger
a rebuild of the enabled set. Invalidate only changed texture digests and retain
selection by topic ID when still present.

Preserve the existing active-mod name/version metadata and hash mechanism.
Encyclopedia bytes themselves do not enter saves or simulation fingerprints.
A combined simulation/content mod may still affect gameplay through its normal
world overlay; this design does not change that mod's existing compatibility
semantics. Content validation/diagnostics must run on startup, new campaign and
mod reload without double-applying world patches.

Browser v1 consumes the unmodified staged base catalog. Existing browser mod
filesystem APIs are stubs; parity of a *modded* native session is not promised.
Future browser installation should supply the same manifest/overlay/image byte
provider and pure patcher, not a separate schema or DLL decoder. Shipping a
pre-modded pack needs a separate package/mod identity contract and is deferred.

## 9. Delivery slices and acceptance

| Slice | Depends on | Deliverable and proof |
|---|---|---|
| A: source contract and schema | Design review | Establish encoding, all identity joins, category/proven-selector/availability rules for the first profile; retain unproven alternates as inventoried, unused deferred evidence; add schemas and synthetic valid/invalid examples. No invented mappings. |
| B: extraction/staging | A | Go extraction, manifest verification, repeatability, safe replacement and complete resource accounting. Original files remain unmodified. |
| C: runtime loading and native overlays | A, B | Shared parser and asset provider, separate content ownership, overlay diagnostics and atomic reload; no save-schema change. |
| D: browser transport | B, C | Pack collector/installer and loose fallback consume identical base bytes; staging included in normal container/web builds. |
| E: original index/topic UI | C, D plus recovered controls | All scoped categories, text, images, navigation and caller return work in both factions and both platforms. Enable routes only for accepted implementation scope. |
| F: evidence and mod guide | B–E | Update README_MOD, asset docs and P35/RE-ENC-01 evidence with exact package hashes and stated remaining parity gaps. |

Required tests cover synthetic PE blocks (including malformed lengths, UTF-16,
languages and non-ASCII text), decoding without data loss, namespace collisions,
unresolved joins, missing EData numbers, path escapes, corrupt images, duplicate
keys, version rejection, deterministic extraction and Go/Rust validator agreement.
Owned-data tests stay opt-in/ignored and assert the *identified source profile*,
not universal English counts. No proprietary text is added to golden fixtures.

Overlay tests prove dependency precedence, validation rollback, null/omission
semantics, image replacement, disable/reload restoration, no stale texture reuse,
and unchanged unrelated world state. Native/browser tests resolve the same
base topics and image hashes; save/load rebinds successfully without copying
presentation content into the save. Prove current pack compatibility explicitly.

Browser journeys run muted and inspect both factions, index/topic navigation,
long text, image correctness, return/context behavior, and missing assets.
Retain network/console logs, resource IDs, cache-hit evidence and artifact hashes.
Use independent browser acceptance and the original-executable capture gate for
strict parity; successful decoding or a unit test is not visual acceptance.
Run scoped mutation tests for the eventual parser/resolver/overlay implementation,
workspace checks, staging tests, package build, and interface-ledger validation.

## 10. Decisions for review and implementation limits

This proposal recommends the Go extractor, ignored `data/base/encyclopedia/`
staging, a separate versioned catalog, ORPK v3 kind-0 namespaced transport,
existing native mod ordering/merge semantics, and original-style UI consumption.
No production dependency, save format, or DAT format change is requested.

Source recovery must settle the encoding, category/alias structure, proven
first-profile image selectors and visibility rules before freezing schema v1.
Unproven alternate-image predicates are explicitly deferred to `orlocal-2kq`;
their assets remain inventoried but unused and do not block the current schema.
If in-scope findings contradict the proposed model, revise this design and its
fixtures before coding the runtime. These are evidence questions with named
sources and failure gates, not permission to guess. Implementation scheduling
and any schema expansion follow review of this document; this documentation
contribution does not claim that the proposed system already exists.
