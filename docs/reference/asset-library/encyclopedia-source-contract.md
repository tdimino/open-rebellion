---
title: "Encyclopedia Source Inventory and Decoder Contract"
description: "Deterministic ENCYTEXT source observations and profile-bound lossless decoding"
category: "reference"
created: 2026-09-28
updated: 2026-09-28
tags: [encyclopedia, ENCYTEXT, PE, provenance, research]
---

# Encyclopedia Source Inventory and Decoder Contract

This contract defines the first, deliberately non-semantic layer of the
encyclopedia pipeline. It inventories source files and `ENCYTEXT.DLL` PE type-10
resources without executing `REBEXE.EXE`, decoding prose, assigning topics, or
publishing a runtime catalog. A successful inventory proves what bytes were
observed; it does not prove what those bytes mean.

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

The profile establishes lossless byte decoding only for its exact source
identity. It does not establish category or topic identity, title selection,
DAT binding, EData lookup, visibility, navigation, or original UI behavior. The
inventory and decoded research records are not a runtime catalog and do not by
themselves advance P35 or strict RE-ENC-01 acceptance.
