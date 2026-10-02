---
title: "Encyclopedia Data Extraction, Modding, and Display"
description: "Source-derived encyclopedia catalog, asset staging, mod overlays, and native/browser consumption"
type: design
status: wire-contract-approved
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
IndigoCompass coordinator-approved the E09 v1 wire contract on 2026-09-29;
that approval does not itself establish implementation or production
acceptance for the remaining work.

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

### Source evidence requirements used for v1 wire approval

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
4. Account for every extracted resource: bound, a historical documented alias,
   source-proven unused, or explicitly unresolved with a reason. The identified
   profile has 348 decoded ENCYTEXT resources: 347 bound, resource `7176`
   source-proven unused, zero aliases, and zero unresolved. An unresolved item
   may remain in research output but cannot silently become a runtime binding.
5. Recover availability/context rules separately from the static catalog. Being
   present in a DLL does not establish that a topic is visible in every campaign
   state, to both factions, or from every entry point.

For unsupported in-scope controls/encoding or uncertain joins, retain raw bytes
in the local extraction report and stop publication of the affected category.
Do not substitute externally written lore. Deferred alternate images remain
inventoried and unbound rather than stopping publication of an otherwise proven
first-profile category. A partial category implementation is labeled partial
and must not claim full P35 acceptance.

## 4. Coordinator-approved schema v1 wire contract

> **E09 v1 wire contract — coordinator-approved 2026-09-29.**
> Accepted E54 evidence proves one aggregate-only fleet topic, contradicting the
> earlier mandatory `topic.category_id` shape. The concrete response below and
> strict schemas form the approved wire contract. Coordinator-verified E09
> closure enables E37's synthetic fixtures. Runtime production remains gated by
> E31, E32, and E51; the embedded profile's
> `ready_for_schema_freeze: false` value is legacy embedded profile readiness
> pending E10 synchronization to the approved contract, not an E09-to-E37 gate.
> Subsequent review revisions also separate immutable base-wire provenance from
> Rust-only effective facts, make row sorting explicit, remove aliases from v1,
> preserve exact existing mod names, and define one retained-byte ledger.

Use one canonical `catalog.json` with a versioned envelope and keyed objects.
This is both readable by mod authors and simple to load into typed Rust maps.
A separate `manifest.json` records extraction provenance and file digests; it is
not a mod-editable catalog. Raw-resource reports are tooling artifacts only.

### Catalog fields

| Field | Proposed type and meaning |
|---|---|
| `schema_version` | Integer `1`; reject unsupported versions rather than guessing. |
| `default_language` | Decimal Windows LANGID string from the source profile, e.g. `"1033"`. |
| `topic_sort` | Exact constants naming the stable displayed-title sort, Windows-1252/Unicode key policy, and registry tie-break. |
| `index` | Aggregate command `0x6f` with 347-candidate potential membership in source-proven tie order. It is not a category; runtime world/viewer admission and title sorting produce display rows. |
| `categories` | Array in source-proven tab order. Each filtered view has a stable internal ID, command, localized labels and potential-membership/tie-order `topic_ids`. Tabs are not title-sorted. |
| `topics` | Map keyed by `original:<ENCYTEXT-resource-id>` for source-proven topics. Runtime aliases are not a v1 wire field. |
| `images` | Immutable base map keyed by canonical `edata:<number>` IDs for original artwork, containing relative `path`, `format`, byte length, dimensions, lowercase `sha256`, and manifest provenance. Generated mod descriptors are not serialized here. |
| `bindings` | Array joining `{family, dat_id, variant}` to `topic_id`; family names match world arenas where possible. Default variant is `"default"`. |

The index and category records contain `labels` (LANGID-to-string), unique
`topic_ids`, and a `source_ref`. `topic_ids` is potential membership in
source-proven order and is the stable comparator-equal tie-break, **not frozen
display order or a promise that every system candidate is live**. The first
profile uses a 147-definition recovered registry prefix followed by 200 systems
in ascending packed `(family << 24) | DatId` order. Category array position
alone is tab display order. A topic
contains `localized` plus `source_ref`; canonical identity, source binding, and
view membership remain separate. Unknown `aliases` fields reject in v1.
Any future runtime alias representation requires a new schema version;
historical research `documented_alias` evidence remains valid outside the wire.

The first-profile relationship rule is zero-or-one filtered memberships per
topic, with zero allowed only when the profile proves aggregate-only
membership. All 347 catalog candidates occur in `index.topic_ids`; 346 occur in exactly one
of commands `0x70..0x75`; source identity `0x08000004` occurs in none. This does
not invent a fleet category or promote command `0x6f` into membership.

At every load/reload the consumer applies overlay language maps in resolved mod
order, validates every retained localized record as complete, selects one whole
requested/default record, resolves that record's art, then stable-sorts the
current view by its displayed title. A newly added/deleted translation, title
rename or language change may reorder rows without changing membership; default
edits never leak into a surviving requested record. The approved `topic_sort` rule
strictly encodes Windows-1252
titles, folds ASCII `A..Z` only, leaves high bytes unchanged, and compares
unsigned bytes. Unrepresentable Unicode titles sort afterward by
the Unicode 15.1.0 full lowercase mapping applied independently per scalar,
encoded as UTF-8 without normalization. Consumers pin that table or prove Rust
`char::to_lowercase` matches it; whole-string context-sensitive lowercasing is
forbidden. Equal keys retain `topic_ids` registry order.
`FUN_00626ad0`/`FUN_005f59f0` prove the narrow-byte case-insensitive stable
behavior. The source registry tie order is also connected:
`FUN_00585b70` constructs the keyed container; `FUN_005f4f10` inserts through
registry vtable `0x0066a220+0x08` → `FUN_00585f50`; `FUN_005843d0` derives the
packed family-qualified DatId key; and insertion maintains the `+0x10`
successor followed by `FUN_00584570`. That packed system order differs from
SYSTEMSD `source_row` order at 180 positions.

E51 then observed two specific fresh runs, raw viewer selectors 1 and 2. Each
had equal complete snapshots at LCID `0x0409`/code page 1252 and 247 admitted
rows (147 definitions + 100 systems); their identity/key sequences were equal.
The independently selected-title oracle matches all 247 cache positions and all
six filtered projections with zero ASCII-fold ties. The two observed 100-system
iterator lists are exact projections of the source-proven 200-system packed-key
potential order. This is not evidence that every scenario or faction always
admits the same 100.

These results close the current-profile comparator choice. The deterministic
high-byte/Unicode behavior above remains an explicit port extension, not
original CRT/Unicode parity. A full 256-byte fold map, original Unicode parity,
and manufactured equal-title original-runtime fixtures are not prerequisites;
synthetic language, mod-rename, and tie cases must enforce the declared policy.
If another source profile exercises incompatible original bytes, revise its
sort contract rather than silently generalize this one.

Canonical content identity preserves the original unique ENCYTEXT body resource
key and family-qualified `{family, DatId}` binding. Original DLL/resource IDs
are never renumbered for convenience. Cache `+0x0c` joins all observed rows by
canonical resource key; cache `+0x68` is a runtime handle whose low bits differ
from the accepted definition DatId for 141 of 147 definition rows (while all 100
observed systems match). Runtime handles and pointers never replace stable
identity. The player sees localized names/text/art, not IDs.

The wire catalog therefore records 347 catalog candidates, not the 247 admitted
rows of either observation. Source proves the original selected-view and
type-`0xf2` ancestry predicate; it does not prove an existing Open Rebellion
mapping to exploration, faction, population or another world field. The two
specific fresh runs prove one shared 247-row outcome, not a universal
scenario-size or faction rule.

E31 (`orlocal-818.31`) owns the source-to-world/viewer admission mapping,
instantiated system membership, ancestry-predicate adaptation and world-epoch
re-evaluation. It emits one typed `AdmissionSnapshot` containing the epoch,
viewer and ordered admitted `BindingKey`/`AdmissionFact` pairs. `BindingKey`
preserves `{family, dat_id, variant}`. An absent instantiated system is not
admitted; with missing required source-equivalent selected-view/ancestry facts,
the original-parity surface remains unavailable with an actionable diagnostic. No
consumer may substitute every 200 systems or the captured 100.

E44 (`orlocal-818.46`) consumes those typed admitted IDs/facts as a pure
content/language/sort resolver with no world dependency. Admission therefore
precedes overlay merge, whole-record language selection, art resolution and
sorting without making E44 a second world adapter. E32 (`orlocal-818.32`)
requires E31 adapter evidence before production route enablement; that evidence
is not a wire-schema freeze prerequisite. Browser v1 remains base-only for mod
content. A separately labeled candidate inspector may remain a development aid,
but it is not original parity.

A base-wire localized content record contains required `title` and `body`, plus
either an optional base `image_id` (missing or null means no artwork) or the closed
typed `viewer_faction` image selector proven for 16 E54 mission/fleet topics.
The two shapes are mutually exclusive. Systems and all other standard art use
their resolved static canonical image ID. Arbitrary predicates and the
unproven `EDATA.192` alternate remain absent.

`validate_bundle` derives immutable source-backed topic image capability from
the binding and verifies every base localized selector shape agrees. A
`viewer_faction` topic therefore authorizes a complete faction pair for a newly
added language even when that LANGID has no base record. Default/static topics
reject faction selectors. Effective overlays can replace art but cannot grant
or remove this topic capability.

Base image IDs are exactly `edata:<canonical-decimal-u32>` (7–16 ASCII bytes).
The catalog JSON Schema is base-wire-only and contains no effective definitions
or generic mod-ID union. Runtime types are a documented Rust boundary:

```text
BaseImageId(String)
ModImageId(String)
EffectiveImageId = Base(BaseImageId) | Mod(ModImageId)
EffectiveImageRef = None | Static(EffectiveImageId)
                  | ViewerFaction { alliance, empire }
EffectiveImageFacts = {
  image_id, format, byte_length, width, height, sha256,
  origin: BaseManifest { source_ref }
        | ModSnapshot { mod_name, relative_path },
  retained_bytes: Arc<[u8]>
}
```

The generated mod identity encodes the exact existing `ModManifest.name` UTF-8
bytes without normalization or case folding:

```text
mod:v1:<lowercase-hex-exact-UTF8-name>:<relative-path>
```

Thus `demo` is `mod:v1:64656d6f:encyclopedia/assets/test.png` and `MyMod` is
`mod:v1:4d794d6f64:encyclopedia/assets/test.png`. This avoids imposing the
documented-but-unenforced kebab-case convention. Empty, delimiter-bearing,
case-distinct and normalization-distinct names remain exact; exact duplicates
retain the existing resolver diagnostic. Facts retain the original name/path
and validators recompute the ID.

There is no mod-name length validity rule and no every-enabled-mod preflight.
Only an actual encyclopedia identity allocation computes checked
`7 + 2*UTF8_len(name) + 1 + path_len`, reserves the generated ID plus retained
name/path buffers under the one global candidate byte ledger, then allocates.
Overflow is `identity_length_overflow`; insufficient capacity is
`resource_limit:retained_bytes`. A name longer than 1,024 bytes succeeds with
adequate candidate budget and fails before allocation otherwise. Confined paths
remain at most 256 ASCII bytes. They resolve under the already discovered mod
root; encoded names never build filesystem roots. Traversal, symlink escape,
origin mismatch, ID/fact collision, and filesystem aliases reject instead of
overwriting a descriptor.

The wire contract's complete synthetic mod-image example is owned by `demo`, has ID
`mod:v1:64656d6f:encyclopedia/assets/test.png`, and uses a reproducibly
generated 70-byte, 1×1 RGBA PNG with SHA-256
`5b8ce344a9d7fe4bdf8780725fc2fc36dce3f297688e62f18c8848fce6fecb8b`.
Locked `pngjs` 7.0.0 generates it; Go's existing standard `image/png` decoder
independently performs a full decode and confirms the 1×1 pixel. A prior
25-byte plain-text placeholder had matching declared length/hash but is not PNG
and is now a retained negative check. Structural schema and digest agreement do
not replace bounded full image decoding.

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
v1 admits only `default` and the proven `viewer_faction` case. No expression
evaluator or mod-supplied executable predicate is introduced. Live statistics
remain read from the bound `GameWorld` record.

Localized overlay maps apply in resolved mod order before language selection.
After merge, validate every retained language record as complete; a null
language entry deletes that whole record. The resolver then selects one whole
record: requested LANGID, then catalog default, and resolves that record's art.
It does not combine a title in one language with a body or image selector in
another. Missing both records disables that topic with a diagnostic. E44 owns
added-translation reorder, deleted-translation fallback, default non-leakage and
incomplete-record rejection tests plus complete conformance against the pinned
Unicode 15.1.0 per-scalar lowercase table. Category labels independently use
requested LANGID then catalog
default. Missing both disables that category with a diagnostic and never
displays its internal key as prose. This is a deliberate robustness divergence
from the original absent-label empty display; all seven current profile labels
are present. A present empty label remains empty rather than triggering fallback
or invented text. Choosing another language is a catalog-loading parameter,
not a new settings UI here.

### Synthetic catalog example

All names, IDs and content below are fictional test data, not recovered joins.
The all-zero digest illustrates the field shape only; a real staged file must
carry and pass its computed digest.

```json
{
  "schema_version": 1,
  "default_language": "1033",
  "topic_sort": {
    "algorithm": "stable_display_title_v1",
    "representable_encoding": "windows-1252-strict",
    "representable_fold": "ascii-lowercase-only",
    "unrepresentable": "unicode-15.1.0-scalar-lowercase-utf8-after-representable",
    "tie_break": "registry-order"
  },
  "index": {
    "command": "0x6f",
    "labels": {"1033": "Synthetic aggregate"},
    "topic_ids": ["original:60001", "original:60002"],
    "source_ref": "fixture/label/index"
  },
  "categories": [
    {
      "id": "command:0x70",
      "command": "0x70",
      "labels": {"1033": "Test ships"},
      "topic_ids": ["original:60001"],
      "source_ref": "fixture/category/ships"
    }
  ],
  "topics": {
    "original:60001": {
      "localized": {
        "1033": {
          "title": "Example cruiser",
          "body": "Synthetic description.\n\nA second paragraph.",
          "image_id": "edata:42"
        }
      },
      "source_ref": "fixture/topic/60001"
    },
    "original:60002": {
      "localized": {
        "1033": {
          "title": "Aggregate-only example",
          "body": "Synthetic aggregate-only description.",
          "image_selector": {
            "kind": "viewer_faction",
            "alliance_image_id": "edata:43",
            "empire_image_id": "edata:44"
          }
        }
      },
      "source_ref": "fixture/topic/60002"
    }
  },
  "images": {
    "edata:42": {
      "path": "assets/EDATA.042",
      "format": "bmp",
      "byte_length": 81080,
      "width": 400,
      "height": 200,
      "sha256": "0000000000000000000000000000000000000000000000000000000000000000",
      "source_ref": "fixture/image/42"
    },
    "edata:43": {
      "path": "assets/EDATA.043",
      "format": "bmp",
      "byte_length": 81080,
      "width": 400,
      "height": 200,
      "sha256": "0000000000000000000000000000000000000000000000000000000000000000",
      "source_ref": "fixture/image/43"
    },
    "edata:44": {
      "path": "assets/EDATA.044",
      "format": "bmp",
      "byte_length": 81080,
      "width": 400,
      "height": 200,
      "sha256": "0000000000000000000000000000000000000000000000000000000000000000",
      "source_ref": "fixture/image/44"
    }
  },
  "bindings": [
    {"family": "capital_ship_classes", "dat_id": 7, "variant": "default", "topic_id": "original:60001"},
    {"family": "fixture_fleet", "dat_id": 2, "variant": "viewer_faction", "topic_id": "original:60002"}
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

Validation has two non-interchangeable entry points:

```text
validate_bundle(base, manifest, files) -> ValidatedBaseBundle
validate_effective_catalog(catalog, image_facts) -> ValidatedEffectiveCatalog
```

`validate_bundle` proves exact immutable catalog/manifest hashes, the base file
set and bytes, every base `source_ref`, binding-source identity, and base
binding/selector agreement. `validate_effective_catalog` runs on a candidate
seeded from that validated bundle. It proves every effective image reference
has one matching retained buffer/fact, recomputes length/digest/decode facts,
checks origin/ID equality and limits, and rejects unused mod facts. It does not
compare the effective catalog to the base catalog hash, mutate the base
manifest/file set, fabricate source records, or require mod facts to close
through base provenance.
The approved E09 wire contract retains 10,000 topics, 1 MiB UTF-8 per localized body,
32 MiB per image, 16 million pixels per image, and 128 MiB aggregate
staged/effective image bytes. It adds 64 MiB/depth 16 for catalog JSON,
32 MiB/depth 16 for manifest JSON, 16 MiB/depth 8 for one overlay, and a
single 512 MiB retained-byte ledger across live base/mod buffers, the serialized
candidate, in-flight reads, and actual generated identity/retained name/path
UTF-8 buffers. Reserve a checked exact size before reading each chunk or
allocating an identity; count one shared allocation once and an actual copy
again. Only actual encyclopedia candidates allocate or can fail admission;
unrelated enabled mods are not preflighted. Candidate failure releases its
reservations while the old live state remains; successful publication precedes
old-state release. The 128 MiB logical asset-set and per-file/parser/decode
limits are orthogonal, not quotas composing 512 MiB. Decoded/GPU/container
memory is outside this ledger, so this is not a total-process-memory promise.
These are implementation budgets, not original-game limits. The
[E09 decision note](../reference/asset-library/encyclopedia-schema-decisions.md)
separates measured inputs from conservative output limits. It assigns shared
file conformance to E37, presentation/fallback/sorting to Rust consumer tests,
and effective images/mod identity/transactions to adapter/session tests.

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
open the recovered index. Build category tabs in explicit catalog order. Apply
overlay language maps, validate complete records, select requested/default whole
records, resolve art, then stable-sort topic lists under `topic_sort`, using
membership arrays as equal-key tie order. Display the
selected localized title/body/image and obtain live stats from the bound world
record. Fleet instances first resolve their class.
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
relative to that mod's root, null to explicitly remove optional art, or the
atomic pair `{ "alliance": {"path":"..."}|null,
"empire": {"path":"..."}|null }`. Both pair keys are required. The pair is
allowed only when the validated immutable source-backed topic capability is
`viewer_faction`; this includes a new language with no base record. No effective
override can grant or remove that capability, while a later layer may use it to
restore a pair after a static/null override. A static base topic rejects the
pair. The adapter validates each non-null file, computes
digest/dimensions, retains exact bytes in the candidate snapshot, creates the
`mod:v1:<hex-exact-UTF8-name>:<relative-path>` Rust identity, and translates the whole
image value atomically. Authors do not maintain IDs, hashes, or facts.
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
Effective mod image facts are verified against retained owning-mod snapshot
bytes, not inserted into or checked against the original base manifest. On a
base `viewer_faction` record, static replacement removes the selector, null
removes art, and a complete faction pair replaces both sides atomically. These
leave the binding untouched. Source-profile selector agreement is a base-bundle
check and is not reapplied to reject permitted effective content. Disabling the
mod rebuilds from base and restores its selector. A forbidden binding edit,
partial/failed pair, or missing retained bytes rejects the candidate.

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

E37's shared corpus owns only serialized file shape, raw parsing, immutable
relationships, digests, and limits. Whole-record language resolution, missing
versus empty category labels, internal-key non-display, title sorting (base
reproduction, language changes, mod renames, ties and non-ASCII), and navigation
are E44's world-independent Rust consumer tests. E31 tests instantiated
membership, source-equivalent ancestry adaptation, missing-fact diagnostics,
typed admitted `BindingKey`s/admission facts, and world-epoch re-evaluation.
Effective image capability, exact mod-name encoding, collisions, retained
bytes, and rollback are Rust adapter/session tests.

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

## 10. Approved wire decisions and implementation limits

The approved wire contract records the planned Go extractor, ignored
`data/base/encyclopedia/` staging, separate versioned catalog, ORPK v3 kind-0
namespaced transport, existing native mod ordering/merge semantics, and
original-style UI consumption. No production dependency, save format, or DAT
format change is requested.

Wire approval used the recovered encoding, category/membership structure, proven
first-profile image selectors, visibility rules, source-proven stable potential
order, two complete 247-row observed-order reproductions, and bounded
deterministic port extension. Runtime aliases are removed from v1; historical
`documented_alias` research status remains, while the current profile has zero
topic/body aliases and shared art does not imply one.
Unproven alternate-image predicates are explicitly deferred to `orlocal-2kq`;
their assets remain inventoried but unused and do not block the current schema.
E54 proves that one topic is aggregate-only, and E55 proves all seven label
selectors. The approved E09 wire contract separates aggregate/category potential
view arrays from canonical topics, retains typed viewer-faction art, and pins
strict catalog/manifest/overlay schemas and budgets. Coordinator-verified E09
closure enables E37 (`orlocal-818.43`) executable file conformance. E10
(`orlocal-818.10`) subsequently synchronizes the legacy embedded profile/Go
validator and catalog producer to the approved contract; its
`ready_for_schema_freeze: false` value is legacy embedded profile readiness
pending E10 and cannot circularly prevent E09-to-E37. E11
(`orlocal-818.11`) owns strict Rust wire parsing. E31 (`orlocal-818.31`) owns
source-to-world/viewer admission mapping and hands typed admitted `BindingKey`s
and admission facts to E44 (`orlocal-818.46`), which remains a pure
content/language/sort resolver with no world dependency. E32
(`orlocal-818.32`) requires adapter evidence for production enablement, but that
is not a wire-schema freeze prerequisite. Production readiness remains gated by
E31, E32, and E51. E42 (`orlocal-818.44`) publishes only the validated result.
Synthetic tie/mod-rename tests enforce the port extension. A0 full-matrix
visual/runtime acceptance and `orlocal-2kq` remain separate.
These handoffs are evidence-backed contracts, not claims that downstream
implementation or tests exist. Wire approval does not claim that the planned
runtime system exists or is production-ready.
