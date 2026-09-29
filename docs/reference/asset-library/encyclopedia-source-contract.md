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

The canonical embedded profile now also contains the reviewed E07, E38, and
E39 binding merge. The three files under
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
| **Observed accepted rows** | **331** | — |

All 331 rows are bound, select distinct ENCYTEXT body identities, and retain
the profile-observed empty preferred-title branch with the original selector's
fallback selected. The 214 static rows use 40 distinct art identities because
systems share source-proven picture selectors; this sharing is not a topic
alias. The 60 major/minor character `DatId` values do not overlap in this
profile, but the source family remains part of canonical identity and a future
profile with overlap must be rejected or add a reviewed table discriminator.
Unit bindings remain class bindings: fleet and entity instances must resolve
their source class at the application boundary rather than use a slotmap or raw
instance ID as a topic identity.

The seven original commands retain this order and role:

| Ordinal | Command | Combined-profile result |
|---:|---:|---|
| 0 | `0x6f` | Aggregate index over admitted rows; not a second topic category. Incomplete while `0x73` is unresolved. |
| 1 | `0x70` | 200 context-filtered system/world-location rows. |
| 2 | `0x71` | 38 capital/fighter class rows. |
| 3 | `0x72` | 14 facility rows; the three facility tables remain one original selector. |
| 4 | `0x73` | **Unresolved:** no accepted family fragment supplies its `[0x40,0x80)` rows. |
| 5 | `0x74` | 10 troop class rows. |
| 6 | `0x75` | 60 character and 9 special-force rows. |

The owned resource inventory makes the incomplete category concrete rather
than filling it from numeric gaps. The 331 accepted rows account for 331 of 348
ENCYTEXT records and 157 of 191 ENCYBMAP logical IDs. These exact residual
identities remain `unresolved` with a next-proof requirement:

```text
ENCYTEXT: 7176, 7184, 7185, 7186, 7187, 7189, 7190, 7191, 7200,
          7201, 7202, 7232, 7233, 7234, 7296, 7297, 7427
ENCYBMAP: 7184, 7185, 7186, 7187, 7188, 7189, 7190, 7191, 7200,
          7201, 7202, 7232, 7233, 7234, 7296, 7297, 7427, 11280,
          11281, 11282, 11283, 11284, 11285, 11286, 11287, 11296,
          11297, 11298, 11328, 11329, 11330, 11392, 11393, 11523
EData:    132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
          143, 146, 148, 149, 150, 151, 152, 153, 154, 155, 156,
          157, 158, 160, 161, 162, 163, 164
```

Those 34 lookup IDs reference 29 distinct supplied files because five
filenames have duplicate references. This numerical relationship is an
inventory observation, not proof that the residual records form 17 topics or
use a particular faction rule. Closure requires the connected DAT
registration, record field writes, and title/body/art selector paths for
command `0x73`; names, numbering gaps, and file presence are insufficient.
`EDATA.192` is the 187th image and is separately marked
`publication_deferred`: it remains inventory-only under `orlocal-2kq`, with no
topic, lookup, predicate, or original-display claim.

The representation settles the schema shape for the accepted rows: one
filtered category per bound topic, `0x6f` as an aggregate selector, separate
source and canonical-topic identities, explicit aliases, and a retained
character source-family discriminator. It deliberately records
`ready_for_schema_freeze: false`. E09 remains blocked on command `0x73` source
joins and the still-unconnected localized category-label selectors. The raw-ID
versus runtime-class adapter remains a downstream application-boundary gate.

The opt-in reconciliation gate reads the owned sources without modifying them
and compares all source, text, lookup, and image identities to the profile:

```bash
REBELLION_ENCYCLOPEDIA_TEST_SOURCE=/path/to/owned-install \
  go test ./tools/stage-ui-assets \
  -run TestOwnedEncyclopediaCombinedBindingProfileReconcilesInventories \
  -count=1 -v
```

The 331, 348, 191, 186, and 187 values above are observations for the exact
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

The localized selector labels remain unjoined, so stable keys are commands and
ranges rather than guessed names. Master definition enumeration admits
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
then system-iterator order.

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

The application boundary must use closed types and pure rules. It must not
accept expressions, scripts, or mod-supplied predicates:

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

## Non-claims

The profile establishes lossless byte decoding and 331 reviewed family bindings
only for its exact source identities. The semantic checkpoint establishes only
the bounded static rules named above; it does not establish localized category
labels, command `0x73` family joins, a complete topic inventory, the deferred
alternate Luke predicate, or original runtime/visual acceptance. Deferred
alternate inventory is not a runtime binding. Inventory and research records
are not a runtime catalog and do not by themselves advance P35 or strict
RE-ENC-01 acceptance.
