---
title: "Encyclopedia Source Inventory Contract"
description: "Deterministic ENCYTEXT and source-file observations for encyclopedia research"
category: "reference"
created: 2026-09-28
updated: 2026-09-28
tags: [encyclopedia, ENCYTEXT, PE, provenance, research]
---

# Encyclopedia Source Inventory Contract

This contract defines the first, deliberately non-semantic layer of the
encyclopedia pipeline. It inventories source files and `ENCYTEXT.DLL` PE type-10
resources without executing `REBEXE.EXE`, decoding prose, assigning topics, or
publishing a runtime catalog. A successful inventory proves what bytes were
observed; it does not prove what those bytes mean.

The implementation is in
`tools/stage-ui-assets/encyclopedia_profiles.go`. It reuses the bounded PE
resource reader in `pe_resources.go` and preserves each resource's raw bytes for
later, profile-bound decoding.

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
padding, control handling, or any other decoding policy. Those are separate
source-recovery gates. No original prose or raw resource bytes are reproduced
in this document.

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

## Non-claims

This inventory does not establish text encoding, category or topic identity,
title selection, DAT binding, EData lookup, visibility, navigation, or original
UI behavior. It is not a runtime catalog and advances neither P35 nor strict
RE-ENC-01 acceptance on its own.
