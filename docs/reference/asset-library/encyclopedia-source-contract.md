---
title: "Encyclopedia Source Inventory and Decoder Contract"
description: "Deterministic ENCYTEXT and ENCYBMAP observations, profile-bound decoding, and bounded source semantics"
category: "reference"
created: 2026-09-28
updated: 2026-09-29
tags: [encyclopedia, ENCYTEXT, ENCYBMAP, PE, provenance, research]
---

# Encyclopedia Source Inventory and Decoder Contract

This contract begins with the deliberately non-semantic source layer of the
encyclopedia pipeline, then records a bounded semantic research checkpoint. The
inventory covers source files and `ENCYTEXT.DLL` PE type-10 resources without
executing `REBEXE.EXE`, decoding prose, assigning topics, or publishing a runtime
catalog. A successful inventory proves what bytes were observed; it does not
prove what those bytes mean.

The inventory implementation is in
`tools/stage-ui-assets/encyclopedia_profiles.go`. It reuses the bounded PE
resource reader in `pe_resources.go` and preserves each resource's raw bytes.
`tools/stage-ui-assets/encyclopedia_decode.go` performs the separate,
profile-bound decode. Inventory remains useful when no decoder profile matches.

## Deterministic report

The report discriminator is `kind: "encyclopedia-research"` and its current
`schema_version` is `1`. Its ordered records contain only reproducible source
facts:

- explicit source, resource-record, and duplicate-identity counts derived during
  canonicalization;
- an explicit root role and source basename;
- source kind (`dll`, `exe`, or `dat`), byte length, and SHA-256;
- numeric versus named PE type and resource identities;
- Windows `LANGID` and the PE data-entry code-page field;
- raw resource length and SHA-256;
- whether the identity is duplicated; and
- a research status.

The duplicate-identity key is source root role, source basename, resource-type
identity, resource identity, and `LANGID`. Code page, length, and digest remain
facts about each occurrence rather than parts of its identity. Consequently a
named resource `"7"` cannot collide with numeric resource ID `7`, while two
numeric ID-7 records in the same language are reported as duplicate
occurrences.

Sources and resource records are canonically sorted before JSON serialization,
including status and unresolved `reason`/`next_proof` tie-breakers for otherwise
identical duplicate observations. Call order, PE directory order, file
modification time, and invocation time do not change the report. Raw resource
bytes are intentionally excluded from JSON; the inventory API retains exact
copies in each in-memory record so the later research stage can write `.bin`
evidence without rereading or guessing at text.

## Statuses and unresolved evidence

Every research record uses one of these version-1 statuses:

| Status | Meaning |
|---|---|
| `inventoried` | Identity and byte facts are known; no decoding or semantic claim is made. |
| `decoded` | A later evidence-backed decoder has losslessly interpreted the raw bytes. |
| `bound` | A later reviewed mapping binds the record to a canonical subject. |
| `documented_alias` | A later reviewed mapping proves that the record is an alias. |
| `source_proven_unused` | Connected source traversal proves the record has no admitted selector for this profile; it is accounted evidence, not an alias or unresolved guess. |
| `unresolved` | A specific attempted interpretation remains open. |

An `unresolved` record must contain both a nonempty `reason` and a nonempty
`next_proof`. Other statuses cannot carry unresolved details. Unknown meaning is
therefore retained as evidence rather than converted into replacement text or a
convenient runtime binding.

## Roots and installation separation

Callers declare every input root with a stable role such as `install` or
`gdata`, then associate each source basename with exactly one role. The
inventory code opens only that direct child; it does not search parent
directories, follow source-file symlinks, or fall back to another installation.
Separate DLL and DAT roots are supported, but their separation is explicit.

Deterministic output contains root roles and basenames, never host paths.
Resolved absolute roots and `started_at` belong only to the separate
`encyclopedia-inventory-run` version-1 run log. Generated reports should be
stored under an ignored research/evidence directory; the run log must remain a
separate ignored file so identical source bytes produce identical report bytes
on different hosts and at different times.

## Bounds and failure policy

The default inventory limits are 512 MiB per source file, 10,000 resource
records, 1 MiB per type-10 resource, and 128 MiB aggregate resource bytes. The
reader checks limits before copying resource payloads. Type-10 resource names
may be numeric or named, but language entries must carry numeric Windows
`LANGID` values; named language entries are unsupported and fatal rather than
silently skipped. Truncated resource directories, data entries outside the PE
section, oversized ranges, malformed resource names, and invalid source
specifications are also fatal. The caller receives no partial inventory.

Malformed PE is different from unresolved semantics: malformed bytes fail the
inventory, while valid bytes of unknown meaning remain preserved with an
explicit research status.

## Inspected English profile observation

Read-only inspection on 2026-09-27 observed the following source. These values
identify that inspected corpus; they are not universal validation rules for
other editions, languages, or installations.

| Source | SHA-256 | Resource observation |
|---|---|---|
| `ENCYTEXT.DLL` | `49aea545a5e09e5fe9115a22bc785690f103d2f931e08bd4a53a617a42636d8c` | 348 numeric type-10 records; `LANGID` 1033; PE code-page field 0 |

In particular, code page 0 does not establish UTF-8, Windows-1252, terminal
padding, control handling, or any other decoding policy. Those require the
executable and corpus evidence below. No original prose or raw resource bytes
are reproduced in this document.

## Canonical decoder profile

The single machine-readable source for the currently accepted decoder metadata
is
[`encytext-49aea545-lang-1033-v1.json`](../../../tools/stage-ui-assets/encyclopedia_profiles/encytext-49aea545-lang-1033-v1.json).
It is embedded with Go `embed`, strictly decoded, and validated when used, so a
built staging tool resolves it without the repository as its working directory.
This document explains the evidence and policy; it is not a second profile to
keep in sync.

Profile selection requires the `ENCYTEXT.DLL` basename (case-insensitive), DLL
kind, exact byte length, and exact SHA-256. `LANGID`, PE code page, resource
type, and every resource's preserved length/hash are then checked before text
decoding. A basename alone never selects a profile, and records from different
profiles cannot be silently combined.

The profile also records the inspected `REBEXE.EXE` length and SHA-256 as
research provenance. That executable is explicitly marked
`required_for_decoding: false`: decoding a matched DLL does not require an EXE
in a flattened staging root, and neither inventory nor decoding executes it.

## Executable call-path evidence

The addresses below were recovered from the profile's recorded `REBEXE.EXE`
identity using the workflow in [`agent_docs/ghidra-re.md`](../../../agent_docs/ghidra-re.md).
The existing `ghidra/notes/FUN_*.c` exports corroborate the named functions;
address-level disassembly and corpus reports remain ignored research evidence.

| Address | Evidence relevant to text bytes |
|---|---|
| `0x00429f30` | Constructs the encyclopedia object through `FUN_0045d400`. |
| `0x0045d400` | Loads `encytext.dll` and `encybmap.dll`, then stores their module handles. This function is a loader, not by itself an encoding decoder. |
| `0x0045f480` | Dispatches the text-view mode to `FUN_0045fa60`. |
| `0x0045fa60` | Calls `FindResourceA` for resource type 10, then `LoadResource` and `LockResource`, and passes the resulting narrow byte pointer to `FUN_005f35b0`. It does not pass `SizeofResource` into that string path. After constructing the CString, it stores that string into the body object at `+0xa0` through `FUN_005f3090`, then calls `FUN_0041fc30`. |
| `0x005f35b0` → `0x005f3650` → `0x005f3630` | The copy path scans to the first NUL and copies the terminated narrow string. |
| `0x0041fc30` → `0x0041fd00` | The connected body text-layout/measurement path reads the same object `+0xa0` field through `FUN_00583c40` and passes that pointer to `DrawTextA` with length `-1`. The `0x2410` flags include `DT_CALCRECT` calculation behavior. |

This path contains no UTF-8 decoder and no call to the executable's unrelated
wide/narrow conversion helpers. Therefore neither the PE code-page value 0 nor
the mere presence of conversion imports is encoding evidence. The connected
`+0xa0` dataflow proves narrow, NUL-terminated body text reaches the original
layout/measurement code; it is not a claim of inspected runtime or visual
rendering acceptance.

## Encoding and non-ASCII corroboration

The accepted source identity has 348 records. Exactly 29 contain non-ASCII
bytes. Across those 29 records, the only non-ASCII byte is `0x92`, appearing 32
times. Read-only inspection of every occurrence corroborates punctuation use:
31 occurrences are between ASCII letters and one follows an ASCII letter before
whitespace; none occurs in a binary/control context.
Windows-1252 maps that byte to U+2019 RIGHT SINGLE QUOTATION MARK, while treating
it as ISO-8859-1 would introduce a C1 control. The narrow ANSI call path, source
language 1033, and the complete observed corpus jointly establish the profile's
Windows-1252 policy; PE code page 0 alone does not.

The canonical set digest over sorted lines
`resource-id|LANGID|raw-length|raw-SHA-256` for those 29 records is
`8f52e5969ecfc65484227ece50ec87ed02cdc39f0203f42c5dbf4b25ccd029d3`.
This permits all 29 identities to be rechecked without storing their prose.
The decoder maps valid Windows-1252 bytes directly to Unicode and rejects the
undefined bytes `0x81`, `0x8d`, `0x8f`, `0x90`, and `0x9d`; it never emits a
replacement character.

## Terminators, padding, newlines, and controls

`FUN_005f3630` proves that the first NUL terminates the original string. Corpus
inspection further found 346 resources with one terminal NUL and two resources
with two; no resource contains a nonzero byte after the first NUL. The profile
therefore permits exactly one terminator and at most one additional zero padding
byte. Missing NULs, three or more trailing NULs, embedded NULs followed by
content, and nonzero suffix bytes are malformed for this profile.

Across all 348 records the observed content contains 956 LF bytes, no CR or CRLF
sequences, 1,819 tab bytes, and no other C0 controls. Decoding preserves LF,
tabs, leading spaces, trailing spaces, blank lines, and other meaningful spacing
exactly. Bare CR, DEL, and unproved C0 controls are rejected instead of being
normalized or printed. Output is valid UTF-8 with LF line endings because the
profile's source already uses LF; no guessed newline rewrite is performed.

The decoded record keeps an exact copy of its original bytes, raw length, raw
SHA-256, resource identity, `LANGID`, and PE code page alongside the UTF-8 text,
encoding, and profile identifier. Successful decoding advances an `inventoried`
record to `decoded`; later `bound` or `documented_alias` states are not
downgraded. An explicitly `unresolved` record is not silently decoded.

## ENCYBMAP lookup inventory

`tools/stage-ui-assets/encyclopedia_lookups.go` provides the lookup-only API for
PE type 6 (`RT_STRING`) resources. It consumes the existing bounded PE snapshot
reader and returns `map[uint16]map[uint32]string`: `LANGID` → logical string ID
→ EData filename. Languages are grouped before decoding, so equal block IDs in
two languages remain separate identities. For a numeric block the logical ID
is exactly `(block_id - 1) * 16 + slot`; the calculation uses `uint32`, checks
its full range, and never wraps through the older TEXTSTRA `uint16` interface.
TEXTSTRA extraction and validation are unchanged.

Each block observation retains numeric versus named identity, `LANGID`, PE code
page, raw length, SHA-256, and an independent copy of the raw bytes. The block
decoder reads exactly 16 length-prefixed UTF-16LE entries, validates surrogate
pairs, and permits only up to three zero alignment bytes afterward. Truncated
lengths, unpaired surrogates, nonzero padding, out-of-range LANGIDs, numeric
block zero, logical-ID overflow, and duplicate `(block identity, LANGID)` rows
are fatal instead of being overwritten. A named block is decoded and retained
as an explicit `unresolved` observation, but contributes no logical IDs because
the numeric block formula cannot be applied without a recovered named selector.

Filename reconciliation is bounded to one declared EData root and its direct
regular-file children. Only the source-shaped, case-insensitive
`EDATA.` + three decimal digit basename form receives a file number; separators,
unnumbered suffixes, and paths are rejected rather than assigned invented IDs.
The deterministic reconciliation records:

- every language-qualified logical ID and its exact lookup string;
- exact, unique case-folded, missing, or case-ambiguous resolution;
- duplicate filename references together with every `(LANGID, logical ID)`;
- each supplied EData basename, derived decimal number, byte length, and
  SHA-256; and
- missing lookup filenames and supplied-but-unreferenced files.

Case-fold collisions in the declared root are reported as ambiguous and are
not resolved even if one candidate has the lookup's exact spelling. Every file
in that ambiguous candidate set is nevertheless marked referenced evidence, so
none is mislabeled as genuinely unreferenced; unrelated files remain in the
unreferenced inventory. EData metadata is hashed as a bounded-memory stream
after regular-file and path/open-file identity checks, without imposing a
runtime pixel-size limit on research evidence. The lookup inventory does not
parse BMP pixels, copy files, publish runtime assets, or infer a topic binding.
BMP validation belongs to E41 and physical staging belongs to E06.

### Inspected English ENCYBMAP observation

The identified `ENCYBMAP.DLL` has SHA-256
`fb545d19ae24b0277753494dbfaabf2dbdde660beab821287a32016c290e4560`.
Read-only reconciliation against the explicitly declared EData root observed:

| Fact | Observation |
|---|---:|
| Numeric RT_STRING blocks | 31 |
| Block `LANGID` | 1033 |
| Block PE code-page field | 0 |
| Nonempty logical strings | 191 |
| Distinct case-folded filenames | 186 |
| Duplicate-reference groups / excess references | 5 / 5 |
| Missing filenames / case ambiguities | 0 / 0 |
| Logical ID 4736 | `EDATA.014` |

These counts and the mapping are observations for this exact DLL and EData
inventory, not universal validation rules. The separate owned EData observation
contains 187 files; the one filename not referenced by this lookup table is
`EDATA.192`. Under the approved first-profile scope it remains inventory-only
and unused. That absence of a lookup is not proof that no original behavior can
ever select the file, and it creates no alternate-art predicate or runtime
binding.

The opt-in owned check is:

```bash
REBELLION_ENCYCLOPEDIA_TEST_SOURCE=/path/to/owned-install \
  go test ./tools/stage-ui-assets \
  -run TestOwnedEncyclopediaLookupInventoryPreservesInputsAndRetainsIgnoredEvidence \
  -count=1 -v
```

It compares the ENCYBMAP snapshot and all reconciled EData length/hash facts
after inventory, then writes metadata only to ignored
`.artifacts/encyclopedia/E05-owned-lookups.json`. It never writes original
lookup tables, pixels, or generated packs into Git.

## Combined family bindings and resource closure

The canonical embedded profile now also contains the reviewed E07, E38, E39,
and E54 binding merge. The four files under
`tools/stage-ui-assets/encyclopedia_profiles/fragments/` remain source-research
inputs; their paths, SHA-256 digests, and observed row counts are recorded in
the profile. A built staging binary consumes the combined representation in the
root profile and does not need those fragments or the repository working
directory.

The merge preserves the family-qualified identity
`(source_family << 24) | dat_id`, the original/preferred/selected title
selectors, language-qualified body and art selectors, original resource byte
hashes, and exact EData basenames. It rejects duplicate binding tuples,
ambiguous source identities, conflicting source-file identities, missing
source references, implicit duplicate body topics, inconsistent accounting,
and silently omitted category commands. Shared art is allowed because it is
counted independently from topic identity; a shared body requires one canonical
record plus an explicit `documented_alias` / `alias_of` relationship.

For this exact profile the accepted family fragments contain:

| Source family | Rows | Original category command |
|---|---:|---:|
| Capital-ship classes | 30 | `0x71` |
| Fighter classes | 8 | `0x71` |
| Troop classes | 10 | `0x74` |
| Special-force classes | 9 | `0x75` |
| Major characters | 6 | `0x75` |
| Minor characters | 54 | `0x75` |
| Systems/world locations | 200 | `0x70` |
| Defense facilities | 6 | `0x72` |
| Manufacturing facilities | 6 | `0x72` |
| Production facilities | 2 | `0x72` |
| Mission definitions | 15 | `0x73` |
| Aggregate-only fleet definition | 1 | `0x6f` index only |
| **Observed bound rows** | **347** | — |

All 347 rows are bound, select distinct ENCYTEXT body identities, and retain
the profile-observed empty preferred-title branch with the original selector's
fallback selected. The 214 static rows use 40 distinct art identities because
systems share source-proven picture selectors; this sharing is not a topic
alias. The 60 major/minor character `DatId` values do not overlap in this
profile, but the source family remains part of canonical identity and a future
profile with overlap must be rejected or add a reviewed table discriminator.
Unit bindings remain class bindings: fleet and entity instances must resolve
their source class at the application boundary rather than use a slotmap or raw
instance ID as a topic identity. The additional 27 owned residual-table rows
are completely represented as 16 bound topics and 11 source-proven exclusions;
the latter are neither topics nor unresolved omissions.

The seven original commands retain this order and role:

| Ordinal | Command | Combined-profile result |
|---:|---:|---|
| 0 | `0x6f` | Complete aggregate potential membership over 347 catalog candidates; live world/viewer admission may remove systems. Includes one source-proven family-`0x08` fleet topic with no filtered category. |
| 1 | `0x70` | 200 potential system/world-location rows in source-proven packed-key order; live context filtering selects a subset. |
| 2 | `0x71` | 38 capital/fighter class rows. |
| 3 | `0x72` | 14 facility rows; the three facility tables remain one original selector. |
| 4 | `0x73` | 15 admitted mission-definition rows from the `[0x40,0x80)` source table. |
| 5 | `0x74` | 10 troop class rows. |
| 6 | `0x75` | 60 character and 9 special-force rows. |

The residual closure is source-backed rather than filled from numeric gaps.
`FUN_005674e0` maps executable selectors `0x6b0` and `0x6b2` to `MISSNSD.DAT`
and `FLEETSD.DAT`; `FUN_00569300` registers their family intervals;
`0x00590bb0` and `0x0058fe20` read their records through `FUN_00584c10`; and
`FUN_00422620` applies the master-cache admission predicates. Command `0x73`
then filters the retained `[0x40,0x80)` rows. The connected
`FUN_0045fa60` path selects the Alliance body-key lookup or the Empire
body-key-plus-`0x1000` lookup without changing the body identity.

The complete owned traversal accounts for 25 mission rows as 15 bound and 10
source-proven unused, and for two fleet rows as one aggregate-only binding and
one source-proven-unused placeholder. The text accounting is exact:
**348 decoded ENCYTEXT resources = 347 bound + 1 source-proven-unused resource
`7176`; zero aliases and zero unresolved text records**. The traversal closes
16 of E40's 17 residual text resources and 32 of its 34 residual lookup
strings. Text resource `7176` and lookups `7188`/`11284` have no admitted source
selector and are explicitly `source_proven_unused`; they carry zero references
and no open next-proof gate.
All 29 formerly residual EData files are selected by other bound lookup
identities. Shared files remain art resources rather than topic aliases.

[`encyclopedia-residual-bindings.md`](encyclopedia-residual-bindings.md)
records the complete source path, exact table accounting, selectors, source
hashes, and reproduction commands. `EDATA.192` remains the 187th image and is
separately marked `publication_deferred`: it is inventory-only under
`orlocal-2kq`, with no topic, lookup, predicate, or original-display claim.

The representation closes the source identities but exposes a schema
contradiction: 346 bound rows have one filtered category, while source identity
`0x08000004` is aggregate-only and `0x6f` is an index rather than topic
membership. The previously approved design requires `category_id` and one
category per topic. The approved E09 wire contract therefore specifies separate
aggregate and filtered membership arrays, with no membership field on the
canonical topic. Their
`topic_ids` retain source-proven potential membership and stable tie order and
are not frozen display order or a claim that every system exists in a live
world. The 147 definition candidates retain recovered registration/container
order; the 200-system tail uses ascending packed family-qualified DatId order.
That shape preserves the fleet topic without
inventing a category and keeps source
bindings, membership, and ordering independent. Runtime aliases are absent from
the approved v1 wire contract: an `aliases` field is rejected. Historical
research status `documented_alias` remains available, but this profile has zero
topic/body aliases and shared artwork is not one. Future runtime alias support
requires a new schema version. The full comparison and
validation inventory is in
[`encyclopedia-schema-decisions.md`](encyclopedia-schema-decisions.md).

The same approved E09 wire contract keeps the serialized catalog/manifest an
immutable base bundle. Base image IDs are canonical `edata:<number>` values whose descriptors
and `source_ref` entries close through that manifest. Effective mod art is a
Rust-only DTO, not a catalog-schema definition. Its identity reversibly encodes
the exact original mod-name UTF-8 bytes without normalization or case folding:
`mod:v1:<lowercase-hex-UTF8-name>:encyclopedia/assets/<path>`. Thus `demo`
becomes `mod:v1:64656d6f:encyclopedia/assets/test.png`, while `MyMod` becomes
`mod:v1:4d794d6f64:encyclopedia/assets/test.png`. Empty, delimiter-bearing,
case-distinct and normalization-distinct names remain reversible. A
`mod_snapshot` fact retains
the original name, path and inspected bytes; it never
fabricates a source record or changes the base manifest/file digests. The
contract's `validate_bundle(base, manifest, files)` performs source-profile and
base selector/binding checks. Its separate
`validate_effective_catalog(catalog, image_facts)` checks effective reference
closure, retained bytes/facts and budgets without reapplying base selector-shape
proof to permitted content replacements. Protected binding tuples remain
immutable. `validate_bundle` derives immutable source-backed topic capability
from the binding and requires its base localized selector shapes to agree. A
binding variant `viewer_faction` yields that exact topic capability. An overlay
faction pair is atomic and permitted by that topic capability, even for
a newly added language with no base localized record. It can restore a pair
after an earlier static/null override, but no overlay can grant or remove the
capability and a static base topic can never gain it. Rebuilding without a mod
restores the base selector. Exact
identity grammars, collision rules and cases are in the decision note, not the
base catalog schema.

There is no mod-name length validity rule or every-enabled-mod preflight. Only
an actual encyclopedia identity allocation performs checked
`7 + 2*UTF8_len(name) + 1 + path_len` arithmetic, reserves its generated ID and
retained original name/path buffers under the one global candidate byte budget,
then allocates. Arithmetic overflow is `identity_length_overflow`; insufficient
budget is `resource_limit:retained_bytes`. The confined path remains at most 256
ASCII bytes and is resolved under the already discovered `ModManifest.path`;
the encoded name never supplies a filesystem root.

IndigoCompass coordinator-approved the E09 v1 wire contract on 2026-09-29.
Coordinator-verified E09 closure enables E37's synthetic file fixtures. The
embedded profile deliberately remains `ready_for_schema_freeze: false`; that is
legacy embedded profile readiness pending E10 synchronization to the approved
contract, not a circular prerequisite for wire approval or E37. Production
readiness remains gated by E31, E32, and E51. Accepted E55 commit
`9ae03be852212fa2ff031f8d2d065a20c2767575` is present in this integration and
closes the source provenance for all seven localized labels, but its fragment
is not yet merged into the embedded root profile. That existing
`category-label-selectors` blocker also remains until the same reviewed
synchronization. E09 now closes the current-profile comparator decision with
source-proven stable insertion, source-proven potential registry order, and two
specific fresh 247-row observations; the stale embedded
`topic-title-comparator-evidence` blocker remains only until E10 synchronizes the
reviewed metadata. The declared high-byte/Unicode extension is not original CRT
parity.
Command `0x73` and residual resource accounting are closed.
The raw-ID versus runtime-class adapter remains a downstream application-boundary
gate, not a source/schema blocker.

### Coordinator-approved v1 wire resource budgets

Measured inputs for the identified profile are 347 topics; a 1,015-byte
maximum decoded UTF-8 record and 99,113 decoded bytes across all 348 records;
187 images totaling 15,161,638 bytes with an 81,080-byte, 400×200 / 80,000-pixel
maximum; and metadata inputs no deeper than seven JSON containers. No complete
catalog or manifest exists, so no output size is represented as measured.

The approved E09 wire contract retains 10,000 topics, 1 MiB of UTF-8 per
localized body, 32 MiB and 16,000,000 pixels per image, and 128 MiB aggregate staged/effective
image bytes. It adds pre-parse limits of 64 MiB/depth 16 for a catalog,
32 MiB/depth 16 for a manifest, and 16 MiB/depth 8 for one overlay, plus one
512 MiB global retained-byte cap across live base/mod buffers, the single
serialized candidate, its in-flight reads, and actual generated identity/name/
path UTF-8 buffers. Reserve before each read/chunk/allocation using checked
exact sizes; count an actual shared allocation once and a copy again; release failed
candidate reservations while retaining the old live state, or publish the new
state before releasing the old. The 128 MiB staged/effective asset-set limit
and per-file/parser/decode limits are orthogonal rather than partitions of this
cap. Decoded/GPU/object allocations are excluded; this is not a
total-process-memory promise. Exact
measurement provenance, headroom, and boundary ownership are in the E09
decision note; these values are part of the approved v1 wire contract.

The opt-in reconciliation gate reads the owned sources without modifying them
and compares all source, text, lookup, and image identities to the profile:

```bash
REBELLION_ENCYCLOPEDIA_TEST_SOURCE=/path/to/owned-install \
  go test ./tools/stage-ui-assets \
  -run TestOwnedEncyclopediaCombinedBindingProfileReconcilesInventories \
  -count=1 -v
```

The 347, 358, 348, 191, 186, and 187 values above are observations for the exact
source hashes in the profile, never universal validity rules.

## Semantic research checkpoint

This checkpoint is bounded to the same inspected `REBEXE.EXE` identity recorded
by the decoder profile. It records source semantics needed before a catalog
schema can freeze; it does not add runtime catalog data or claim visual
acceptance. The strict embedded source profile now carries the combined binding
and accounting representation above while preserving the accepted decoder
fields. Dynamic selection, availability, navigation, and capture decisions
remain in this contract and `RE-ENC-01`.

The user-approved 2026-09-29 scope publishes only source-proven standard,
faction, and system image selectors for the first profile. Unproven alternate
art remains inventoried and unused under deferred task `orlocal-2kq`: no runtime
binding, predicate, UI switch, or original-display claim is admitted. This
deferral is not proof of recovery or absence and does not weaken the remaining
identity, availability, context, navigation, or capture gates.

### Master cache, selectors, and ordering

The following source paths are connected dataflow, not nearby calls:

| Source path | Recovered rule |
|---|---|
| `FUN_00421c70` → shell `+0x474`; `FUN_00422620` lines 78–272 | The shell starts with a null master topic collection. `FUN_00422620` constructs and populates it only while null; later calls do not refresh it. |
| `FUN_0051caf0` → `FUN_00567800` → `FUN_0051cab0`; `FUN_004f31b0` / `FUN_004f6010` | Master rows come from registry definitions plus a viewer-side system iterator, not from a category-time walk of live campaign objects. |
| `FUN_00585b70` → `FUN_005f5440` / `FUN_005f4f10`; registry vtable `0x0066a220 + 0x08` → `FUN_00585f50` → `FUN_005843d0`; `FUN_00584570` | A system container stores low-24-bit DatId at `+0x18` and family at `+0x20`. The registry comparator orders `(family << 24) | DatId`; insertion maintains `+0x10` as its in-order successor and the live iterator follows that link. Thus packed-key order, not SYSTEMSD row order, is the source-proven potential order. |
| `FUN_0045ddc0` → `FUN_0045f100` | Builds the seven selectors at fixed x positions. Changed, nonforced family selection in index mode filters the retained master cache by source-identity high byte. |
| `FUN_0045f100` → `FUN_00608280` | Command `0x6f` or an eligible forced change binds shell `+0x474` into the list control and refreshes it. It does not clear or manufacture an empty collection. |
| `FUN_0060a790(..., 2)` → vtable `0x0066e148 + 4` → `FUN_0060a890` → `FUN_00626ad0`; insertion in `FUN_005f59f0` | Both master and filtered collections are ordered doubly linked lists sorted by case-insensitive narrow-byte display text. Comparator-equal rows retain source insertion order. |
| `FUN_00429f30` → `FUN_0045d400` → `FUN_0045fd90` | Resolves empty, class, and entity contexts to a canonical definition-derived topic key, with a connected entity fallback; raw entity identity is not automatically topic identity. |
| `FUN_00442130` → vtable `0x00659ba0` → `FUN_004ad730` / `FUN_004ad750` | Initializes rows enabled and implements next/previous traversal that recursively skips disabled rows. |
| `FUN_0045da70` → current-topic vtable `+0x0c` / `+0x10` → `FUN_0045fa60` | Commands `0x84` and `0x83` use the skip-disabled neighbors. A null result retains the current topic; there is no endpoint wrap. |
| `FUN_0045fa60` → `FUN_0045f970` | Selects the EData lookup key from cached source identity, viewer side, or the system picture selector before loading topic text. |
| `FUN_004f31b0` → `FUN_0053ef50` / `FUN_0053f090` → type-`0x90` vtable `0x00663698 + 0x10` → `FUN_004f6330` | Selects the requested side view for each system and excludes it when its `+0x1c` container-ancestry chain contains an object whose virtual type is `0xf2`. |

Displayed selector order is fixed by control position, not map iteration:

| Order | Command | x | Bound/filter behavior |
|---:|---:|---:|---|
| 0 | `0x6f` | 0 | bind the full shell `+0x474` master cache |
| 1 | `0x70` | 52 | filter cached source high byte `[0x90, 0x98)` |
| 2 | `0x71` | 104 | filter `[0x14, 0x20)` |
| 3 | `0x72` | 156 | filter `[0x20, 0x30)` |
| 4 | `0x73` | 208 | filter `[0x40, 0x80)` |
| 5 | `0x74` | 260 | filter `[0x10, 0x14)` |
| 6 | `0x75` | 312 | filter `[0x30, 0x40)` |

Stable keys remain commands and ranges rather than guessed names. Accepted E55
commit `9ae03be852212fa2ff031f8d2d065a20c2767575` proves that all seven displayed
labels come from language-qualified `TEXTSTRA.DLL` selectors; it also proves
that display order differs from source construction order and that the
recovered source has no alternate label selector. E09 consumes that evidence
in its approved wire contract. A present empty label remains empty. If both
requested and default labels are absent, the proposed consumer disables that category with a
diagnostic instead of showing the original empty label; this is a deliberate
robustness divergence, and all seven current labels are present. The root
profile remains intentionally unsynchronized until review. Master definition
enumeration admits
`[0x08,0x20)`, `[0x22,0x40)`, and `[0x50,0x80)`; the last range additionally
requires resolved definition `+0x5c == 0`. Thus the broader outer selectors do
not make `[0x20,0x22)` or `[0x40,0x50)` rows appear in this master cache. System
rows `[0x90,0x98)` come from the viewer-side iterator with its exclusion flag
enabled. Every admitted row derives the canonical key
`(definition +0x30 & 0x0fff) + 0x1000`, and `FUN_0060a860` deduplicates on that
key before insertion.

`FUN_0060a890` mode 2 compares row text at `+0x14`. `FUN_00626ad0` folds ASCII
case when no locale is active and otherwise maps bytes through the current CRT
case map before byte comparison; this is not locale collation. `FUN_005f59f0`
walks past comparator-equal rows, so ties preserve definition-registry order and
then the packed-key system iterator order. The latter is independently connected:
`FUN_00585b70` constructs keyed containers, the registry comparator
`FUN_00585f50` gets both keys from `FUN_005843d0`, generic insertion
`FUN_005f4f10` threads `+0x10` in order, and `FUN_00584570` advances through that
thread. The 200-candidate packed order differs from SYSTEMSD `source_row` order
at 180 positions. Each of the two live 100-system iterator sequences is exactly
the packed order projected to that run's admitted membership.

The approved E09 wire contract stores an explicit `topic_sort` rule and treats
each view's `topic_ids` as membership plus registry tie-break, not display order. Runtime
processing applies localized overlay maps in resolved mod order, validates all
retained records as complete, selects the whole requested/default record,
resolves that record's art, and only then sorts. Thus added/deleted translations
and mod title changes may reorder rows while default edits never leak into a
surviving requested record and category tab order remains fixed. E44 owns these
consumer pipeline tests and complete conformance against the pinned lowercase
table. The proposed deterministic port strictly
encodes Windows-1252 titles, ASCII-lowercases `A..Z` while preserving high
bytes, and places unrepresentable Unicode titles afterward using the pinned
Unicode 15.1.0 full lowercase mapping independently per scalar, encoded as
UTF-8 without normalization. A consumer must pin the mapping or prove Rust
`char::to_lowercase` matches it; whole-string contextual lowercasing is not the
contract. Unsigned byte comparison and stable registry tie order complete the
rule.

E51 supplied two specific fresh runs, one with raw viewer selector 1 and one
with selector 2. Each process recorded LCID `0x0409` and code page 1252 in two
equal complete snapshots; each cache contained 247 admitted rows—147 definition
candidates and 100 systems—with the same canonical-key/identity sequences. The
private selected-title join maps all 247 rows by unique canonical resource key,
then reproduces all 247 cache positions and all six filtered projections under
ASCII folding with zero folded ties. The runtime packed `+0x68` identity agrees
with accepted source identity for all 100 systems, but its low bits are not a
definition DatId: 141 of 147 definition rows differ. Preserve the original
family-qualified `{family, DatId}` and DLL/resource identities; never renumber
from a cache handle. Player UI continues to expose names, text, and art, not
these evidence keys.

This closes the current-profile comparator choice while retaining exact limits.
The two runs do not prove a universal 100-of-200 admission, scenario-size cause,
faction cause, general CP1252 folding, or original Unicode behavior. A full
256-byte fold map, original Unicode parity, and manufactured equal-title runtime
fixtures are not prerequisites: v1 deliberately preserves non-ASCII CP1252
bytes, pins the documented Unicode extension, and tests renamed/equal mod titles
synthetically. A future source profile with different original-title bytes must
carry a separately reviewed sort rule.

The immutable catalog therefore carries 347 catalog candidates and potential
membership. The two observed runs admitted 247 rows; they are regression
observations, not a wire invariant. Source proves the original selected-view and
type-`0xf2` ancestry predicate, but no existing Open Rebellion mapping to world,
viewer, exploration, population or faction fields is proven.

E31 (`orlocal-818.31`) owns the source-to-world/viewer admission mapping,
instantiated system membership, source-ancestry predicate adaptation and
world-epoch re-evaluation. It hands E44 a typed admission snapshot containing
the epoch/viewer plus ordered admitted `BindingKey`s and `AdmissionFact`s. An
absent instantiated system is not admitted. With missing required source-equivalent
facts, the original-parity surface remains unavailable with an actionable
diagnostic; it must not show all 200 candidates or hard-code the observed 100.
E44 (`orlocal-818.46`) consumes those admitted IDs/facts and remains a pure
content/language/sort resolver with no world dependency. E32
(`orlocal-818.32`) requires adapter evidence before production enablement; that
runtime evidence is not a wire-schema freeze prerequisite. A diagnostic
candidate inspector may be labeled separately. A0 full-matrix acceptance
remains outside E09.

### Selection, routing, and typed application boundary

`FUN_0045f100` applies its transition checks in this exact order:

1. If the requested command already equals `this + 0x118`, return immediately,
   even if the call is forced.
2. If mode is topic (`this + 0x114 == 2`) and force is zero, a changed command
   also returns immediately.
3. Otherwise command `0x6f` or a forced change binds the full master cache.
   Current topic clears only when force is nonzero and mode is not topic.
4. A changed, nonforced family command in index mode rebuilds and binds the
   sorted filtered projection.

The following closed types and pure rules describe recovered **source
semantics**, not an already implemented port mapping. They must not accept
expressions, scripts, or mod-supplied predicates:

```text
ViewerFaction = Alliance | Empire
OpenContext = Index | Class(SourceIdentity) | Entity(SourceIdentity)
Direction = Backward | Forward
SelectionForce = Normal | Forced
SystemSourceAncestry = ContainsTypeF2 | NoTypeF2

build_master_topic_cache(definitions, viewer_side_systems) -> MasterTopicCache
select_collection(selected, requested, force, mode, master) -> SelectionResult
canonical_topic_key(definition_field) -> TopicKey
resolve_open(context, master, fallback_association) -> Index | Topic(TopicKey)
next_enabled_topic(direction, current) -> Stay | Topic(TopicKey)
art_lookup_key(cached_identity, viewer, system_picture) -> Result<EncybmapKey, UnresolvedEvidence>
include_system(ancestry) -> ancestry == NoTypeF2
```

E31 adapts real port state to those source-equivalent facts and publishes:

```text
BindingKey = { family, dat_id, variant }
AdmissionFact = DefinitionPresent | InstantiatedSystem { selected_view, ancestry }
AdmissionSnapshot = { world_epoch, viewer, admitted: [(BindingKey, AdmissionFact)] }
```

Only a complete single-epoch snapshot is valid. Absence from instantiated
system membership means not admitted; absence of a required selected-view or
ancestry fact is an unavailable diagnostic, not a guessed boolean. E44 accepts
the snapshot as data and never reads `GameWorld`.

Required inputs are viewer side, retained shell cache, selected/requested
command, force, encyclopedia mode, typed class/entity identity, resolved
definition `+0x30`, entity fallback association, current topic and enabled
links, system picture selector, and whether the selected system side-view's
container ancestry contains source type `0xf2`. Static asset presence is never
an availability input. `SystemSourceAncestry` is deliberately structural: the
inspected source proves the test but does not justify renaming type `0xf2` as a
knowledge, destruction, or visibility state.

`FUN_0045d400` resolves class context through `FUN_0051cab0` and entity context
through `FUN_004f2d10`; both use `(definition +0x30 & 0x0fff) + 0x1000` to look
up the deduplicated master row. On an entity-key miss, `FUN_0045fd90` retries the
same original identity outside `[0xa0,0xb0)`, so that branch cannot introduce a
different canonical key. Inside that range it resolves through `FUN_004f2f60`,
then uses either `FUN_0040d760` or the associated object at `+0x1c` before
retrying the definition-derived key. Failure leaves current topic null and
selects index mode. Exhaustive family/DAT joins remain E07 work.

For source families with identity high byte in `[0x40,0x80)` or `[0x08,0x10)`,
`FUN_0045fa60` adds `0x1000` for source side 1 or `0x2000` for side 2 after
masking to the low 12 bits. System families `[0x90,0x98)` follow
`FUN_004f3220` → `FUN_00509610` → `FUN_0045f660`; selectors 1–26 map to
ENCYBMAP keys `0x2b5c`–`0x2b75`.

### Decision scenarios, remaining gates, and deferred work

| Scenario | Trigger and expected source outcome | Capture need |
|---|---|---|
| Alliance / Empire | For applicable families, side 1 uses low12 + `0x1000`; side 2 uses low12 + `0x2000`; text identity stays fixed. | Capture the same topic for both sides. |
| Same-command repeat | Requested command equals selected command: immediate no-op before force or mode checks. | Repeat index and family commands, including forced calls. |
| Changed family in index mode | Changed `0x70`–`0x75`, nonforced: rebuild the sorted projection over the retained master. | Change between families and record order/membership. |
| Changed `0x6f` | After early-return eligibility, bind the full master cache, not an empty list. | Record the full-cache view. |
| Forced, non-topic / topic | A changed forced command binds the master; non-topic clears current, topic retains current. | Capture both modes. |
| Nonforced topic change | A changed command returns without changing command, collection, or current topic. | Attempt category change in topic mode. |
| Campaign change after construction | The if-null master cache does not refresh; same-shell category changes only filter it. A new shell may build from new inputs. | Compare one shell before/after a controlled change, then reconstruct it. |
| Context-free / class / entity open | Empty context opens index. Class and entity contexts resolve through definition-derived canonical keys. A non-`[0xa0,0xb0)` miss only retries the same identity; the special range may remap through its connected association. | Capture all three, a non-special miss, and both special-range association branches. |
| Unavailable context | Failed direct and fallback resolution leaves no current topic and enters index mode. | Capture a stale/unavailable identity. |
| Faction / system art | Faction ranges use side keys; system topics use their 1–26 picture selector. | Capture both sides and representative system selectors. |
| System side-view admitted | `FUN_0053f090` selects the requested side view; `FUN_004f6330` reaches the end of its `+0x1c` container ancestry without encountering virtual type `0xf2`. | Capture the same system from both sides while retaining the selected view and ancestry identities. |
| System side-view excluded | The same ancestry walk encounters virtual type `0xf2`, returns nonzero, and the iterator continues without publishing that system row. | Capture a source-backed `0xf2` ancestry case; do not infer a friendlier gameplay label from the type code. |
| Alternate Jedi Luke art | Owned profile maps `0x1842` to `EDATA.074`, leaves `0x2842` empty, and has zero `EDATA.192` string mappings. Recovered static evidence includes the `EDATA\` directory-literal reference at `0x0045f8d2` inside `FUN_0045f7b0` and the identified `FUN_0045f970` callers (`0x0045fbef`, `0x0046a2ff`), which consume table selectors; it does not establish a connected `EDATA.192` predicate. `EDATA.192` remains inventoried but unused for this profile. | **Deferred — `orlocal-2kq`:** publish no binding, predicate, UI switch, or original-display claim. Preserve the evidence and future paired-capture need; the deferral does not block the first profile. |
| First / middle / last topic | Navigation follows sorted enabled rows; endpoints return null, disable their direction, retain current, and do not wrap. | Capture both endpoints and both directions from a middle row. |
| Disabled linked row | `FUN_004ad730` / `FUN_004ad750` recursively skip `+0x6c == 0`; encyclopedia-created rows begin enabled. | Capture only after identifying a connected writer; do not name the flag as knowledge/visibility from shape alone. |
| Unavailable entry | A row absent from master construction or removed by canonical-key deduplication cannot appear in projections or context opens. | Capture an excluded gap and a deduplicated class/entity pair. |

The side-specific system filter is structurally recovered. The type-`0x90`
factory `FUN_00566c70` calls `FUN_00566b90`, which installs vtable
`0x00663698`; its `+0x10` slot is `FUN_004f6330`. That function follows the
selected view's container pointer at `+0x1c`, tests each ancestor's virtual type
at vtable `+4`, and returns nonzero on type `0xf2`. `FUN_0053f090` rejects that
view and continues iteration when the result is nonzero. Type `0xf2` is
constructed by `FUN_005696b0`, whose vtable `0x006639b8 + 4` resolves to
`FUN_00569880` and returns `0xf2`. This proves the inclusion predicate without
proving a higher-level gameplay name for that source container.

Current evidence for the inspected profile does not establish a connected
`EDATA.192` selector or predicate, so no alternate-Luke rule, runtime binding,
or UI switch is admitted. Under the approved scope this unresolved alternate is
inventoried and unused, deferred to `orlocal-2kq`, and does not block E08 or the
first-profile schema/publication. That decision is not proof that original
behavior is impossible or that the alternate was implemented. A connected
original-runtime/load trace of this build, additional connected code/data
evidence, or another legitimately owned profile may support future work.
Runtime captures for both the proven selectors and any future alternate remain
explicit corroboration gates and are not claimed as visual acceptance here.

## Owned-source check

Ordinary tests use synthetic PE fixtures and require no game installation. An
opt-in check inventories a contributor-owned source root:

```bash
REBELLION_ENCYCLOPEDIA_TEST_SOURCE=/path/to/owned-install \
  go test ./tools/stage-ui-assets \
  -run TestOwnedEncyclopediaInventoryPreservesInputsAndRetainsIgnoredEvidence \
  -count=1 -v
```

The check uses the declared installation root and its `GData` child when
present, hashes the selected DLL/EXE/DAT inputs before and after inventory, and
fails if any input changes. It never launches the executable. It retains:

- `.artifacts/encyclopedia/E01-owned-inventory.json` — deterministic basenames,
  hashes, counts, languages, code pages, duplicate identities, and statuses;
- `.artifacts/encyclopedia/E01-owned-inventory.run.json` — invocation root roles,
  absolute paths, and start time.

Both files are ignored. They may contain installation metadata and must not be
committed. For the identified English `ENCYTEXT.DLL` hash only, the opt-in test
also checks the observed count, language, and code-page values above. An unknown
hash is still inventoried; it is not silently labeled as that profile.

The decoder has a separate opt-in gate:

```bash
REBELLION_ENCYCLOPEDIA_TEST_SOURCE=/path/to/owned-install \
  go test ./tools/stage-ui-assets \
  -run TestOwnedEncyclopediaDecoderCorroboratesProfileWithoutChangingInputs \
  -count=1 -v
```

It requires the exact embedded profile identity, decodes all 348 records,
rechecks the 29-record set digest and all 32 observed `0x92` occurrences, rejects
replacement output, compares the original DLL bytes before and after, and writes
only metadata to ignored `.artifacts/encyclopedia/E02-owned-decoder.json`.
Original prose and decoded text are never committed.

The coordinator approved the E09 v1 wire contract on 2026-09-29.
Coordinator-verified E09 closure enables E37 (`orlocal-818.43`) to turn the file
cases into the shared synthetic conformance corpus. E10 (`orlocal-818.10`)
subsequently synchronizes the embedded profile, Go validator and 347-candidate
producer to that approved contract; the legacy
`ready_for_schema_freeze: false` profile flag does not prevent E09-to-E37.
E11 (`orlocal-818.11`) parses the strict wire types. E31
(`orlocal-818.31`) owns source-to-world/viewer admission mapping and emits typed
admitted `BindingKey`s/admission facts; E44 (`orlocal-818.46`) consumes them as a
world-independent content/language/sort resolver with no world dependency. E32
(`orlocal-818.32`) requires adapter evidence for production enablement, not for
wire-contract approval. E09 does not implement or claim those gates. Production
readiness remains gated by E31, E32, and E51. E42 publication and later
native/browser consumers remain downstream of their verified results.

## Non-claims

The profile establishes lossless byte decoding, 347 reviewed bindings, and
complete source/resource accounting only for its exact source identities. E55
establishes the seven label selectors, and E09 records a concrete response to
the aggregate-only topic contradiction, but neither is merged into profile
readiness by this wire approval. This document freezes only the v1 wire
contract; it does not mark the embedded profile ready, recover the deferred
alternate predicate, authorize production consumers, or provide original
runtime/visual acceptance. Deferred alternate inventory is not a
runtime binding. Inventory and research records are not a runtime catalog and
do not by themselves advance P35 or strict RE-ENC-01 acceptance.
