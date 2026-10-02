---
title: "Encyclopedia Schema Decisions — Coordinator-Approved V1 Wire Contract"
description: "Coordinator-approved v1 base files, runtime contracts, validation ownership, and budgets"
status: coordinator-approved-wire-v1
created: 2026-09-29
updated: 2026-09-29
tags: [encyclopedia, schema, provenance, modding, E09]
---

# Encyclopedia schema decisions — coordinator-approved v1 wire contract

> **Approval boundary:** IndigoCompass coordinator-approved the E09 v1 wire
> contract on 2026-09-29. Coordinator-verified E09 closure enables E37's
> synthetic file fixtures. The embedded profile deliberately remains
> `ready_for_schema_freeze: false`; that is legacy embedded profile readiness
> pending E10 synchronization, not a prerequisite for wire approval or E37.
> This approval does not authorize runtime production: production readiness
> remains gated by E31, E32, and E51.

## Approved wire decisions

The coordinator approved these wire decisions on 2026-09-29:

1. Canonical topics do not own a `category_id`. The aggregate index and six
   filtered categories own independent membership arrays. The one proven fleet
   topic in the aggregate and no filtered category remains valid.
2. `topic_ids` stores potential membership in source-proven insertion order. It
   is the stable tie-break, **not frozen display order or a promise that every
   system candidate is live**. At each load/reload, the
   consumer applies localized overlay maps in resolved mod order, validates all
   resulting records, selects one whole requested/default record, resolves its
   art, then stable-sorts rows by the displayed title under the rule below.
3. Category array order is the original tab order `0x70..0x75`; it is never
   title-sorted. Command `0x6f` remains the aggregate index, not a category.
4. The three JSON Schemas describe only serialized base catalog/manifest and
   author overlay files. Effective runtime records and mod image facts are Rust
   types documented here, not extra definitions smuggled into a file schema.
5. Runtime aliases are absent from v1. An unknown `aliases` field is rejected.
   Historical `documented_alias` inventory evidence is preserved, but this
   profile has zero topic/body aliases; shared art does not create one. Any
   future runtime alias representation requires a new schema version.
6. Base validation and effective validation remain separate. Permitted art
   overrides do not rewrite or weaken immutable source proof.
7. One global 512 MiB retained-byte cap governs accepted live buffers plus the
   one serialized candidate and its in-flight reads. It is not seven quotas and
   is not a process-memory promise.
8. Original family-qualified DAT identities and DLL/resource identities remain
   intact. Runtime pointers, registry nodes, and cache `+0x68` handles are
   evidence used by an adapter, never replacement content IDs or player-facing
   labels.
9. No source-to-port admission mapping is claimed here. E31 owns that adapter
   and its world-epoch re-evaluation; E44 receives typed admitted binding facts
   and remains independent of `GameWorld`. E32 requires adapter evidence before
   production route enablement, but that runtime proof is not a wire-freeze
   prerequisite.

This is the narrowest representation that retains all 347 bound topics, the
original aggregate/category behavior, the 16 proven faction selectors, and
existing mod-name compatibility without inventing prose or source semantics.

## Evidence and accounting baseline

All tracked material is metadata-only; no original localized prose, pixels, or
binary payloads are included.

| Evidence | Accepted result used here |
|---|---|
| E40/E54 combined profile and residual traversal | 358 represented DAT rows: 347 bound topics and 11 source-proven exclusions. All 347 appear in the aggregate; 346 appear in one filtered command; source identity `0x08000004` is aggregate-only. |
| Text inventory | Exactly **348 decoded ENCYTEXT resources = 347 bound + 1 source-proven-unused resource `7176`**. Current profile: **zero runtime aliases, zero unresolved text records**. The combined traversal and [`encyclopedia-residual-bindings.md`](encyclopedia-residual-bindings.md) prove resource `7176` has no admitted selector. |
| Artwork inventory | E41 measured 187/187 valid inputs, 15,161,638 aggregate bytes, largest 81,080 bytes, maximum 400×200 / 80,000 pixels. `EDATA.192` is a separate `publication_deferred` art record under `orlocal-2kq`; it is not text accounting, an alias, or an unresolved selector. |
| E55 labels | Seven language-qualified `TEXTSTRA.DLL` type-6 selectors are proven. Display order is `0x6f..0x75`; construction order is `0x6f,0x73,0x72,0x75,0x71,0x70,0x74`. There is no recovered fallback selector. |
| Sort dataflow | `FUN_0060a790(...,2)` → `FUN_0060a890` → `FUN_00626ad0`, with insertion in `FUN_005f59f0`, sorts master and filtered rows by case-insensitive narrow-byte displayed title; comparator-equal rows retain insertion order. |
| Source registry order | `FUN_00585b70` builds registry containers, `FUN_005f4f10` inserts through vtable `0x0066a220+0x08` → `FUN_00585f50`, and `FUN_005843d0` supplies `(family << 24) | low24(DatId)`. The insertion path maintains `+0x10` as the in-order successor used by `FUN_00584570`. The 200-system packed-key order differs from SYSTEMSD `source_row` order at 180 positions. |
| Two live observations | Two specific fresh runs observed LCID `0x0409`, code page 1252, 247 admitted rows each (147 definitions + 100 systems), and equal identity/key sequences despite raw viewer selectors 1 and 2. Every 247-row cache sequence and six filtered projections exactly match the independently selected-title ASCII-fold sort. This is evidence for those runs, not universal 100-of-200 admission or scenario/faction causation. |
| E54 faction art | Sixteen mission/fleet topics have source-proven Alliance/Empire choices; three pairs intentionally share files. |

The identified profile JSON is 727,126 bytes at depth 7. The E55 label
fragment is 6,022 bytes at depth 4, E41 measurement artifact 114,116 bytes at
depth 4, and complete E06 research report 452,079 bytes at depth 7. These are
measured **inputs**, not invented catalog/manifest output sizes.

## Base wire contract

The strict v1 wire schemas are:

- [`encyclopedia-catalog.schema.json`](schemas/encyclopedia-catalog.schema.json)
- [`encyclopedia-manifest.schema.json`](schemas/encyclopedia-manifest.schema.json)
- [`encyclopedia-overlay.schema.json`](schemas/encyclopedia-overlay.schema.json)

Every object rejects unknown fields. The catalog root contains
`schema_version`, `default_language`, `topic_sort`, `index`, `categories`,
`topics`, `images`, and `bindings`. It does not contain `aliases`, effective
facts, generated mod IDs, retained bytes, or mod provenance.

### Identity, membership, and order

- `index.topic_ids` is the complete 347-candidate aggregate potential-membership
  registry. The two observed original runs admitted 247 rows after dynamic
  world/viewer filtering.
- Each `categories[i].topic_ids` is one source-backed filtered membership
  registry. Zero filtered memberships is allowed only with explicit
  aggregate-only source proof.
- Membership arrays retain source-proven potential order solely as the stable
  comparator-equal tie-break. The definition prefix has 147 rows in recovered
  registration/container order. The 200 system candidates follow ascending
  packed family-qualified DatId order, not SYSTEMSD row order. Object iteration
  is never order.
- `categories` array position is tab order. Title sorting never reorders tabs.
- Canonical topic IDs, family-qualified bindings, membership, and source
  provenance remain independent. A canonical topic key is the unique original
  ENCYTEXT body resource identity (`original:<resource-id>`); bindings preserve
  the original `{family, DatId}`. The original cache row `+0x0c` joins all 247
  observed rows to that key. Cache `+0x68` is a runtime packed handle: its full
  value matches source identity for all 100 observed systems but differs from
  the accepted definition identity for 141 of 147 definition rows. It must not
  renumber or replace the binding. UI surfaces render names, text, and art—not
  these IDs.

The explicit `topic_sort` wire constant is:

```json
{
  "algorithm": "stable_display_title_v1",
  "representable_encoding": "windows-1252-strict",
  "representable_fold": "ascii-lowercase-only",
  "unrepresentable": "unicode-15.1.0-scalar-lowercase-utf8-after-representable",
  "tie_break": "registry-order"
}
```

At load/reload, apply overlay language maps to the base in resolved mod order,
validate every retained localized record as complete, select the requested
LANGID when present or the complete default record otherwise, resolve that
record's art, then stable-sort enabled rows. A newly added translation can be
selected and reorder a row; deleting its whole language record restores whole-
record fallback; default-language edits never leak into a surviving requested
record. Renaming a topic can move it without changing membership.

The recovered executable and two observations now close the current-profile
sort decision without overclaiming general locale parity. Both fresh processes
sampled LCID `0x0409` and code page 1252 with a constructed cache. All selected
titles are ASCII, have zero folded ties, and the independently joined 247-row
order matches the original cache at every position in both runs. Source also
proves stable equal-key insertion in `FUN_005f59f0` and the potential registry
order described above. The portable policy remains explicit for future modded
or translated titles:

1. A title strictly representable in Windows-1252 gets class 0. Encode it and
   fold ASCII bytes `A..Z` to `a..z`; leave high bytes unchanged.
2. An unrepresentable title gets class 1. Apply the Unicode **15.1.0** full
   lowercase mapping independently to each scalar (including unconditional
   expansions), encode UTF-8, and do not normalize. The consumer must use a
   pinned table or prove its Rust `char::to_lowercase` mapping matches Unicode
   15.1.0; it must not inherit a toolchain upgrade silently. Whole-string
   context rules are forbidden: uppercase `ΟΣ` maps scalar-by-scalar to `οσ`,
   not the context-sensitive final-sigma `ος` produced by JavaScript
   `"ΟΣ".toLowerCase()`.
3. Sort class 0 before class 1, compare keys as unsigned byte sequences, and
   retain registry order for equal keys.

This is exact current-profile evidence plus a declared deterministic extension,
not a claim that the original CRT performs Unicode sorting or that every CP1252
high byte has been observed. A full 256-byte fold map, original Unicode parity,
and manufactured equal-title original-runtime fixtures are not prerequisites
for v1: high bytes and unrepresentable Unicode follow the declared port policy,
and synthetic mod rename/tie cases must test it. A future source profile whose
original titles exercise different byte behavior must declare a new reviewed
sort contract rather than silently reuse this one. A0 visual/runtime matrix work
remains separate.

### Candidate admission and platform policy

The base wire carries 347 catalog candidates: 147 source-proven definitions and
200 potential systems. It never freezes the observed 247 admitted rows. Source
proves that the original selects a requested side view and excludes a system
when the selected view's ancestry contains type `0xf2`; it does **not** prove an
existing Open Rebellion mapping to exploration, faction, population, or another
world field. The 100-system set shared by two specific fresh runs is a regression
oracle for those observations only; neither source nor capture proves a universal
scenario size, faction cause, or permanent 100-row rule.

E31 (`orlocal-818.31`) owns the source-to-world/viewer admission mapping,
instantiated system membership, source-ancestry predicate adaptation, and
re-evaluation on every world epoch. Its typed handoff is an `AdmissionSnapshot`
containing the world epoch/viewer identity and ordered admitted
`BindingKey`/`AdmissionFact` pairs. `BindingKey` is the preserved
`{family, dat_id, variant}` identity; each fact records whether the candidate is
a source-proven definition or an instantiated system with the required
source-equivalent view/ancestry evidence. An absent instantiated system is not
admitted. With missing required source-equivalent facts, the original-parity
surface remains unavailable with an actionable diagnostic; it never means `all 200` or
the captured `100`.

E44 (`orlocal-818.46`) consumes only those typed admitted `BindingKey`s and
admission facts plus catalog/overlay inputs. It has no world dependency: it
merges localized overlays, validates complete records, performs whole-record
fallback and art resolution, then stable-sorts the admitted rows. E32
(`orlocal-818.32`) requires E31 adapter evidence before production route
enablement. That runtime adapter proof is not a wire-schema freeze prerequisite.
Browser v1 remains base-content-only; a separately labeled candidate inspector
may show candidates, but it is not original UI acceptance.

### Localization, labels, and base images

Overlay language maps are merged in resolved mod order before language choice.
After the whole overlay set, every retained localized record must contain its
complete required fields; an entire language may be deleted with a null language
entry. Only then does whole-record fallback select requested then default and
resolve that record's art. It never combines title/body/art from different
languages. Missing both requested and default topic records disables the topic
with a diagnostic. Category labels resolve requested then default independently.
Missing both disables the category with `missing_localized_label`; this is a
**deliberate robustness divergence** from the original absent-label empty
display. A present empty label remains present and empty. Internal keys never
become prose. All seven current profile labels are present.

A base localized record has required `title` and `body`, plus either optional
static `image_id` or a closed `viewer_faction` selector. The two are mutually
exclusive. Base image IDs are `edata:<canonical-u32-decimal>`. Systems and
standard art are static. No expression predicate or deferred alternate
(`EDATA.192`) enters v1.

`validate_bundle` derives one immutable source-backed topic image capability
from the family-qualified binding. A `viewer_faction` binding requires every
base localized record's source-proven image shape to be the complete faction
selector; a default/static binding rejects any faction selector. That validated
topic capability—not the existence or current shape of a localized record—
authorizes a faction pair. Consequently a mod may add a new requested LANGID
with a pair to one of the 16 proven topics, while no overlay can grant or remove
that permission.

The manifest authenticates exact immutable base bytes. The required invariant
is:

```text
manifest.catalog_sha256
  == manifest.files["catalog.json"]
  == SHA256(exact catalog.json bytes)
```

Every base `source_ref` closes through `source_records`; exact base image paths,
digests, decoded facts, and file-set equality are verified. `manifest.json`
does not self-hash. Runtime mod facts never enter this file.

## Runtime Rust boundary (not a file schema)

The later Rust consumer owns types equivalent to:

```rust
struct BaseImageId(String); // edata:<canonical decimal u32>
struct ModImageId(String);  // mod:v1:<lowercase hex exact UTF-8 name>:<safe path>
enum EffectiveImageId { Base(BaseImageId), Mod(ModImageId) }

enum EffectiveImageRef {
    None,
    Static(EffectiveImageId),
    ViewerFaction {
        alliance: Option<EffectiveImageId>,
        empire: Option<EffectiveImageId>,
    },
}

struct EffectiveLocalizedRecord {
    title: String,
    body: String,
    image: EffectiveImageRef,
}

enum ImageOrigin {
    BaseManifest { source_ref: String },
    ModSnapshot { mod_name: String, relative_path: String },
}

struct EffectiveImageFacts {
    image_id: EffectiveImageId,
    format: ImageFormat,
    byte_length: u64,
    width: u32,
    height: u32,
    sha256: Sha256,
    origin: ImageOrigin,
    retained_bytes: Arc<[u8]>,
}
```

These are in-memory contracts, not a fourth production schema and not valid
catalog root definitions.

### Exact mod-name encoding and compatibility

`mods.rs` describes kebab-case but does not enforce it. The encyclopedia
adapter must not retroactively rename or case-fold existing mods. It encodes
the exact `ModManifest.name` UTF-8 bytes as lowercase hexadecimal:

```text
ModImageId = "mod:v1:" + hex_lower(UTF8(mod_name)) + ":" + ModImagePath
```

Examples:

- `demo` → `mod:v1:64656d6f:encyclopedia/assets/test.png`
- `MyMod` → `mod:v1:4d794d6f64:encyclopedia/assets/test.png`

No Unicode normalization or case folding occurs, so empty names, names with
delimiters such as `:`, `MyMod`/`mymod`, and canonically equivalent but byte-
distinct Unicode names all remain reversible and distinct. For example an
empty name yields `mod:v1::...` and `:` yields the hex component `3a`. The
adapter records the original `mod_name` and path in facts and recomputes the ID
when validating origin equality.

There is **no mod-name validity or 1,024-byte ceiling** and no every-enabled-mod
preflight. Only an actual encyclopedia candidate that needs the identity may
fail admission. Before allocating, checked arithmetic computes
`7 + 2*UTF8_len(name) + 1 + path_len`; overflow is
`identity_length_overflow`. Reserve the exact generated ID allocation and any
retained original-name/path UTF-8 buffers under the one global candidate byte
ledger before constructing them; insufficient capacity is
`resource_limit:retained_bytes`. Thus a name above 1,024 bytes succeeds when
the candidate has budget and fails before allocation otherwise. The confined
art path remains at most 256 ASCII bytes. Exact duplicate names retain the
existing resolver diagnostic. Case-/normalization-distinct names do not
collide. Any ID/origin mismatch, different tuple reusing an ID, or two lexical
paths resolving to one file is rejected, never overwritten.

The path grammar is intentionally narrower than the mod name: slash-separated
ASCII under `encyclopedia/assets/`, no absolute/drive prefix, backslash,
colon-bearing component, empty/`.`/`..` segment, unsupported extension, or
symlink escape. File access always starts from the already discovered
`ModManifest.path` plus this confined relative path; the encoded name is never
used to construct a filesystem root.

### Effective image overrides

Overlay `image` meanings are:

- omitted: inherit the preceding effective value;
- `null`: clear all art;
- `{ "path": "..." }`: install one static mod image;
- `{ "alliance": {"path":"..."}|null,
     "empire": {"path":"..."}|null }`: atomically install a faction pair.

Both faction keys are required. A pair is allowed only when the validated
immutable source-backed **topic capability** is `viewer_faction`. It is valid
for a newly added language on one of those 16 topics even though no base record
exists for that LANGID. It is rejected on a static/no-art base topic even if an
earlier overlay installed a selector. No overlay can grant or remove the topic
capability; it allows a later layer to restore a pair after an earlier static
or null replacement. Each non-null side is independently
confined, bounded-read, fully decoded, and retained. Any side failure rejects
the whole mod batch; no half-pair publishes. Static and null replacement remain
valid on viewer-faction topics. Protected bindings never change. Disabling a
mod rebuilds from the validated base and restores its original selector.

Authors never provide generated IDs, facts, digests, `source_ref`, or manifest
entries. The adapter owns those translations.

### Base versus effective validation

```text
validate_bundle(base, manifest, files) -> ValidatedBaseBundle
validate_effective_catalog(catalog, image_facts) -> ValidatedEffectiveCatalog
```

`validate_bundle` checks immutable JSON/file bytes, exact file sets and hashes,
base reference closure, source profile membership/order, binding uniqueness,
and agreement between each binding-derived topic capability and every base
localized selector shape. `validate_effective_catalog` operates only on a
candidate seeded from that validated bundle. It checks every selected effective
ID against exactly one retained buffer/fact, recomputes length/digest/full
decode facts, enforces origin/ID equality and budgets, and rejects unused mod
facts. It does not mutate the base manifest, fabricate source records, compare
an effective catalog to the base catalog hash, or reapply base selector shape
to reject a permitted content override.

A complete valid synthetic mod fact uses the ID
`mod:v1:64656d6f:encyclopedia/assets/test.png`, original owner `demo`, a
reproducibly generated 70-byte 1×1 RGBA PNG (pixel `#123456ff`), and SHA-256
`5b8ce344a9d7fe4bdf8780725fc2fc36dce3f297688e62f18c8848fce6fecb8b`.
Go's standard `image/png` independently decodes it and confirms dimensions and
pixel data. The earlier 25 plain-text bytes named as PNG are an explicit
negative: matching a declared length/hash is not a decode.

The corresponding Rust DTO evidence projection (not a JSON file accepted by
any of the three schemas) is:

```json
{
  "image_id": "mod:v1:64656d6f:encyclopedia/assets/test.png",
  "format": "png",
  "byte_length": 70,
  "width": 1,
  "height": 1,
  "sha256": "5b8ce344a9d7fe4bdf8780725fc2fc36dce3f297688e62f18c8848fce6fecb8b",
  "origin": {
    "kind": "mod_snapshot",
    "mod_name": "demo",
    "relative_path": "encyclopedia/assets/test.png"
  }
}
```

The associated `Arc<[u8]>` supplies the 70 decoded-and-hashed bytes; it is not
serialized into that projection.

## Resource budgets and retained-byte transaction

These are implementation safety limits, not original-game limits.

| Resource | Measured identified profile | Proposed v1 limit |
|---|---:|---:|
| Topics | 347 bound | 10,000 |
| UTF-8 body | max 1,015 bytes; 99,113 across 348 decoded records | 1 MiB per localized body |
| Title/category label | E55 labels 13–19 bytes; complete serialized title catalog not built | 64 KiB UTF-8 per value |
| Image file | max 81,080 bytes | 32 MiB |
| Image pixels | max 80,000 | 16,000,000 |
| Staged/effective image bytes | 15,161,638 across 187 files | 128 MiB logical asset set |
| Catalog JSON | not yet measured; profile input 727,126 bytes/depth 7 | 64 MiB / depth 16 |
| Manifest JSON | not yet measured; report input 452,079 bytes/depth 7 | 32 MiB / depth 16 |
| One overlay JSON | no measured corpus | 16 MiB / depth 8 / 10,000 patches |
| All retained byte buffers | 15,161,638 measured base-art input; no mod corpus | one global 512 MiB cap, including generated identity and retained name/path UTF-8 buffers |

The 32 MiB file, 16-million-pixel decode, and 128 MiB staged/effective logical
asset-set limits are orthogonal to the retained-buffer cap. They are not
partitions of 512 MiB.

Reload/candidate admission is serialized. Before every read, streamed chunk,
or generated identity/name/path buffer allocation, reserve its checked exact
size under the one global ledger; unread capacity is not an allocation license.
Generated identity length uses checked multiply/add before allocation. The
ledger includes current live base/mod bytes, retained identity UTF-8 buffers,
the one candidate, and in-flight read buffers. Only actual encyclopedia
candidates allocate or can fail this admission; unrelated enabled mods are not
preflighted. One allocation shared by `Arc` references counts once; an actual
copy counts again. A failed or superseded candidate
releases its reservations and buffers while the prior valid live state remains
retained. A successful candidate becomes live before the old state's
reservations are released. Decoded scratch pixels, Rust container overhead,
GPU textures, allocator fragmentation, and unrelated process memory are outside
this byte ledger and require their own controls; 512 MiB is not a total-process
memory promise.

## Validation ownership and case inventory

E37 owns shared **file conformance** only. E31 owns world/viewer admission and
its typed facts; E44 owns world-independent content/language/sort resolution.
Effective overlays, mod identity, and candidate transactions belong to
adapter/session tests. Ajv shape checks do not claim runtime behavior, admission,
or image validity.

### E37 shared file corpus

| Case | Expected result / owner |
|---|---|
| minimal base catalog/manifest/overlay | Accept strict shapes; exact catalog/file digests close. |
| aggregate-only topic | Accept in index and no category only with source proof; invented category or missing index member rejects. |
| category order | `0x70..0x75` accepts; wrong/duplicate command rejects. This tests tabs, not title rows. |
| topic registry arrays | Unknown or duplicate ID rejects; array is membership/tie-break order. |
| `topic_sort` exact constants | Accept only the five declared constants; omission/unknown rule rejects. |
| `aliases` field anywhere | Reject as unknown in v1. |
| base image/selector | Static, no-art, and complete faction selector accept; mixed static+selector, missing side, unsupported kind, or dangling base image rejects. |
| immutable binding/source proof | Family-qualified duplicate or base selector mismatch rejects in `validate_bundle`. |
| manifest closure | Self-hash, extra/missing file, digest contradiction, dangling source ref, wrong binding-source hash, or unsafe path rejects. |
| raw JSON | Duplicate key, invalid UTF-8, byte/depth above limit, or unknown field rejects before ordinary typed use. |
| overlay image shapes | Omitted, null, static path, and complete `{alliance,empire}` pair are structurally valid; missing faction side, direct generated ID, unsafe path, membership/binding/provenance edit rejects. Capability is a semantic adapter check. |
| body/image/pixel boundaries | Exact limit accepts; one above rejects before unbounded allocation/decode. |

### Rust consumer unit tests

| Case | Expected result |
|---|---|
| complete base recovered-order reproduction | Reproduce the 347-candidate potential tie order, then apply explicit test membership. For each of the two observed 247-row memberships, the declared ASCII key must match all 247 original cache positions and all filtered projections. Never freeze SYSTEMSD `source_row` order. |
| language-dependent titles | Post-overlay requested/default whole-record selection occurs before art resolution and stable row sort; changing language may reorder rows. |
| renamed mod title | Later effective title changes position after overlays without changing membership. |
| equal comparator keys | Synthetic renamed/modded equal titles retain the view's potential `topic_ids` order after dynamic admission. Source `FUN_005f59f0` proves stable insertion; an original equal-title capture is not required for this deterministic extension. |
| Windows-1252 representable non-ASCII | High bytes remain unchanged under proposed port rule; compare unsigned bytes. |
| Unicode unrepresentable title | Goes to class 1, pinned Unicode 15.1.0 per-scalar lowercase UTF-8 key, after representable titles; `οσ` and `ΟΣ` tie in registry order while whole-string JS final-sigma mapping is a rejected counterexample. |
| whole-record fallback | Select requested complete record or complete default; `catalog-mixed-language` is a failing consumer assertion, not a shared file fixture. |
| missing category label | Disable with diagnostic (deliberate divergence from original absent-label empty display). |
| present empty category label | Display empty; do not fallback or invent an internal key. `catalog-internal-key-as-label` is a failing consumer assertion, not a file fixture. |
| viewer-faction resolution | Select the proper effective side and preserve shared descriptors; navigation uses stable sorted enabled rows and does not wrap. |

### E31 admission-adapter test inventory

| Case | Expected result |
|---|---|
| instantiated system with complete source-equivalent facts | Emit its original `BindingKey` plus typed admission fact for the current viewer/world epoch. |
| absent instantiated system | Do not admit it; never fill from the 200-candidate list. |
| ancestry-equivalent exclusion | Preserve the source type-`0xf2` exclusion through a proven port mapping; do not rename it to an unproved gameplay field. |
| required fact unavailable | Mark the original-parity surface unavailable with an actionable diagnostic; never guess admission. |
| world epoch changes | Recompute one complete admission snapshot; do not mix facts from epochs or retain stale admitted IDs. |
| native/browser parity | Equivalent typed source facts produce identical ordered admitted `BindingKey`s. |

### E44 world-independent localized overlay/resolver test inventory

E44 receives an immutable ordered admission snapshot from E31 and has no
`GameWorld` dependency.

| Case | Expected result |
|---|---|
| overlay adds requested-language record | Merge in resolved mod order, validate it complete, select it, resolve its art, and allow its title to change stable display order. |
| overlay deletes requested-language record | Delete the whole language entry, validate remaining records, then select the complete default record. |
| overlay updates default while requested survives | Requested title/body/art remain one unchanged whole record; no default field leaks into it. |
| overlay leaves an incomplete localized record | Reject the candidate before language selection, art resolution, or sorting; keep the prior live state. |
| requested and default both absent after overlays | Disable with the named missing-record diagnostic; never synthesize or mix languages. |
| pinned Unicode lowercase conformance | Exercise the complete generated Unicode 15.1.0 per-scalar mapping table, including expansions and the `ΟΣ` final-sigma counterexample; reject toolchain-version drift. |

### Rust overlay adapter and session tests

| Case | Expected result |
|---|---|
| empty / `:` / `demo` / `MyMod` / normalization- and case-distinct names | Generate reversible `mod:v1:<exact-UTF8-hex>:...`; every byte-distinct name remains distinct, including empty `mod:v1::...`. |
| duplicate exact mod name | Preserve existing resolver rejection; never overwrite facts. |
| name above 1,024 bytes with adequate candidate budget | Checked length and reservation succeed; identity is generated only for the actual encyclopedia allocation. |
| same long name with insufficient candidate budget | Reject `resource_limit:retained_bytes` before identity allocation; no generic mod-name error or unrelated-mod preflight. |
| identity arithmetic overflow | Reject `identity_length_overflow` before reserve/allocation. |
| discovered root independence | Resolve the confined art path under `ModManifest.path`; never derive a filesystem root from encoded name bytes. |
| path/ID/origin collision | Traversal, escape, tuple mismatch, aliasing, or conflicting facts rejects before publish. |
| valid decoded mod image | Keep base manifest/catalog bytes unchanged; retained fact closes by ID/origin and full decode. |
| plain text declared PNG | Reject decode even if declared length/hash agree. |
| viewer topic static/null replacement | Accept, remove prior selector as appropriate, preserve binding. |
| viewer topic new-language faction pair | Accept atomically from immutable source-backed topic capability even without a base record for that LANGID. |
| viewer topic complete faction pair | Accept atomically; pair can restore after an earlier static/null override because overlays cannot remove immutable topic capability. |
| static-base faction pair | Reject `image_override_capability`; prior effective changes cannot grant permission. |
| missing pair side or missing retained side bytes | Reject whole batch and keep prior live state. |
| forbidden binding edit | Reject before effective validation. |
| mod disable | Rebuild from base and restore original viewer selector. |
| retained ledger exact/above cap | Reserve-before-read accepts exact, rejects one above, releases failed candidate, and retains prior live state. Shared buffer counts once; copy counts again. |

## Remaining gates and execution-ready handoff

The coordinator approved this six-file E09 v1 wire contract on 2026-09-29.
Coordinator-verified E09 closure enables E37 to build its synthetic file corpus;
E10 producer/profile synchronization follows the approved contract rather than
gating it. The current
profile comparator decision is evidence-complete for E09: source proves stable
tie order, two specific fresh 247-row observations reproduce the complete
current ASCII-title order, and v1 explicitly defines behavior outside that
observed corpus. The embedded profile has intentionally not been edited here.
Its `ready_for_schema_freeze: false` value is legacy embedded profile readiness
pending E10 synchronization; it does not prevent E09 wire approval or E37
fixtures:

- E10 (`orlocal-818.10`) owns the embedded profile/Go validator and catalog
  builder synchronization: integrate the E55 label fragment; replace the
  aggregate-only and stale comparator blockers with the reviewed candidate,
  packed-system-order, identity-adapter, and sort metadata; emit all 347
  candidates; validate the 147-definition prefix, 200-system packed-key tail,
  family-qualified bindings, resource-key joins, and 348 = 347 + 1 accounting.
- E37 (`orlocal-818.43`) owns executable shared file conformance: turn the file
  cases above into strict JSON/Ajv plus cross-validator fixtures, including
  aggregate-only membership and rejection of frozen 247/ad-hoc system lists.
- E31 (`orlocal-818.31`) owns source-to-world/viewer admission mapping,
  instantiated membership, ancestry adaptation and world-epoch re-evaluation.
  It hands typed admitted `BindingKey`s/admission facts to E44. E32
  (`orlocal-818.32`) requires that adapter evidence before production enablement;
  neither item is a wire-schema freeze prerequisite.
- E44 (`orlocal-818.46`) owns whole-record language resolution and stable runtime
  row ordering from E31's admitted IDs/facts, with no world dependency; it must
  cover added/deleted translation, default non-leakage, mod rename, tie, and
  pinned Unicode extension cases.
- E11 (`orlocal-818.11`) and the later Rust validation/session owners must
  preserve `{family, DatId}` and canonical resource keys, never cache `+0x68`
  definition low bits. E31 must prove equivalent native/browser admission facts;
  absence of required source-equivalent facts keeps the original-parity surface
  unavailable rather than displaying all 200 candidates.
- E42 (`orlocal-818.44`) may publish only after E10/E37 validated outputs agree;
  E50 (`orlocal-818.49`) applies the already shared mod order to content once per
  update. None of these handoffs is implemented or tested by E09.
- E51 and all A0/runtime/UI acceptance gates remain unchanged. Production
  readiness remains gated by E31, E32, and E51. Full A0
  category/faction/context capture is not a wire-contract substitute or an E09
  deliverable; `orlocal-2kq` remains the deferred alternate-art task.

Rollback removes the three wire schemas and this decision note and reverts the
marked E09 wire-contract sections in source/design documentation. Accepted evidence,
embedded profiles, ignored measurements, and original inputs remain untouched.
