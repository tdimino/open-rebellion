---
title: Encyclopedia character bindings
description: Source-backed major/minor character title, body, artwork, and identity bindings for the first supported source profile
status: research-complete
last_updated: 2026-09-29
tags: [encyclopedia, characters, DAT, ENCYTEXT, ENCYBMAP, provenance]
---

# Encyclopedia character bindings

This note freezes the character-only research input for the first supported
encyclopedia profile. The canonical machine-readable result is
[`character-bindings.json`](../../../tools/stage-ui-assets/encyclopedia_profiles/fragments/character-bindings.json).
It is deliberately a fragment for E40, not an embedded decoder profile or a
runtime catalog. It contains identifiers, source hashes, selectors, and
citations only; it contains no original title/body prose or pixels.

The result accounts for every record in the two owned character tables:

| Source table | Rows | Header family interval | DAT ID intervals | Faction rows |
|---|---:|---|---|---|
| `MJCHARSD.DAT` | 6 | `[0x30, 0x38)` | `576–579`, `640–641` | 4 Alliance, 2 Empire |
| `MNCHARSD.DAT` | 54 | `[0x38, 0x3c)` | `832–857`, `896–923` | 26 Alliance, 28 Empire |

All 60 records are bound. There are zero unresolved source rows, zero repeated
title selectors, zero repeated body selectors, and zero repeated EData
filenames. The 60 selected body byte hashes and 60 selected art byte hashes are
also distinct. These are observations for the source hashes below, not
cross-edition invariants.

## Connected source path

The binding is not inferred from neighboring numeric ranges. The executable
connects the DAT record fields to the encyclopedia row and its consumers:

1. `FUN_00569300` registers `[0x30,0x38)` with factory `FUN_005915f0` and
   `[0x38,0x3c)` with `FUN_005912b0`
   ([lines 37–52](../../../ghidra/notes/FUN_00569300.c)). These intervals match
   the two owned DAT headers exactly.
2. Both factories install the same record-read path: vtable slot `+0x08`
   reaches `FUN_00591660`, then `FUN_00595780` → `FUN_00593cb0` →
   `FUN_0053b880` → `FUN_00584c10`. The shared reader consumes the 148-byte
   character shape. In particular, `FUN_00584c10` writes the fifth 32-bit
   common field to runtime `+0x2c`, then the two 16-bit fields to `+0x30` and
   `+0x32` ([lines 10–18](../../../ghidra/notes/FUN_00584c10.c)). Against the
   round-tripped DAT layout, file offset `0x14` is therefore the
   `TEXTSTRA.DLL` selector stored at runtime definition `+0x30`. File offset
   `0x16`, written to definition `+0x32`, is module selector `2` in all 60 rows.
   `FUN_00414830` registers `textstra.dll` as module `2` through
   `FUN_005fba30` → `FUN_005ff020`
   ([line 64](../../../ghidra/notes/FUN_00414830.c)), connecting the numeric
   DAT pair to `TEXTSTRA.DLL` rather than relying on a matching string ID.
3. `FUN_00567d90` loads each definition family and resolves definition `+0x30`
   through `FUN_005f2fc0`, storing the title string object at definition
   `+0x34` ([lines 29–48](../../../ghidra/notes/FUN_00567d90.c)). The fragment
   retains the numeric `TEXTSTRA.DLL` selector, never the proprietary string.
4. `FUN_00422620` resolves a definition with `FUN_0051cab0` and computes the
   topic key as `(definition[+0x30] & 0x0fff) + 0x1000`. For the row label it
   first subtracts `0x8000` from the low word of definition `+0x30` and sends
   that pair through `FUN_005f3010`; only an empty result falls back to the
   cached definition `+0x34` string before `FUN_00442130` constructs the row
   ([lines 178–208](../../../ghidra/notes/FUN_00422620.c)). `FUN_005f3010`
   reaches the resource-string callback through `FUN_005f37e0`; the callback
   at `0x0060aa30` resolves the pair's module and calls `LoadStringA`. The owned
   inventory proves all 60 adjusted candidate IDs absent across every
   `TEXTSTRA.DLL` LANGID and all 60 original LANGID-1033 selectors nonempty, so
   this exact profile takes the `+0x34` fallback. Its ordered cache path is the
   source of the class-backed character row; object-map iteration is not used
   as a display order.
5. `FUN_0045d400` uses the same low-12-bit-plus-`0x1000` key for class opens
   and for entity-to-definition opens, with `FUN_0045fd90` as the entity
   fallback ([lines 49–88](../../../ghidra/notes/FUN_0045d400_encyclopedia_loader.c)).
   An entity ID therefore does not directly become an encyclopedia topic ID.
6. For these definition rows, `FUN_0045fa60` falls through to the row key at
   `+0x0c`, passes it to `FUN_0045f970` for `ENCYBMAP.DLL`, and uses its low
   16 bits for the type-10 `ENCYTEXT.DLL` body resource
   ([lines 55–110](../../../ghidra/notes/FUN_0045fa60.c)). The decoded body is
   then passed through the connected `FUN_0041fc30` → `FUN_0041fd00`
   narrow-string layout/measurement path; this is source dataflow evidence,
   not a runtime or visual acceptance claim.

The source formula is consequently:

```text
title_resource_id           = character_record.text_stra_dll_id
row_title_candidate_id      = (title_resource_id - 0x8000) & 0xffff
row_title_fallback_if_empty = cached title_resource_id string at definition +0x34
class_key                   = title_resource_id & 0x0fff
body_resource_id            = class_key + 0x1000
art_lookup_id               = class_key + 0x1000
```

The owned major rows produce keys `6208–6211` and `6272–6273`; the owned minor
rows produce `6720–6745` and `6784–6811`. Strict LANGID-1033 lookup recovery
maps those 60 keys to `EDATA.072` through `EDATA.131` without gaps or reused
filenames. Each body selector exists as a numeric type-10 resource at LANGID
1033. The fragment records every raw record, body, and art length/hash so E40
can detect a mismatched source instead of borrowing another character image.

## Character namespace decision

The original source identity remains
`(source_family << 24) | (dat_id & 0x00ffffff)`. The first profile's 6 major
and 54 minor raw `dat_id` values are disjoint, both within each table and across
the merged character arena. A merged typed character `dat_id` is therefore
unambiguous for this exact profile; no source-table discriminator is required
for its 60 rows.

That conclusion is profile-bound, not a license to select the first match. The
fragment's validation policy is `reject_duplicate_dat_id_across_character_tables`.
If another legitimate profile overlaps, schema review must introduce a
source-table discriminator. It must not renumber the original IDs or resolve a
collision by table order.

Synthetic failure shape:

```text
MJCHARSD.DAT row: dat_id=7
MNCHARSD.DAT row: dat_id=7
result: reject duplicate character dat_id; no first-match resolution
```

## Aliases, shared resources, and absence

No exact source alias or shared title/body/art resource occurs among these 60
rows: all 60 title IDs, all 60 body identities, and all 60 EData basenames are
distinct. This does not assert that equal-looking prose or pixels would be
different semantically; byte equality is not used to invent an alias. A later
profile may add explicit aliases only with its own source evidence.

The fragment has an empty `unresolved_rows` list because all rows in these two
tables reconcile. Missing selectors, missing files, ambiguous case-folded
filenames, duplicate resource identities, or a major/minor ID collision are
hard failures of the retained checker rather than absent rows silently dropped
from the fragment.

## Deferred alternate artwork

The approved scope revision applies unchanged. `EDATA.192` is present in the
owned image inventory but has no recovered `ENCYBMAP.DLL` reference or
connected selector/predicate for this profile. It remains
`inventory_only_unused` under deferred task `orlocal-2kq`. It is not attached
to any character row, and this note makes no claim that the original program
could never display it. File presence does not establish a topic, selector, or
campaign predicate.

The standard character bindings above are complete without that alternate.
No alternate-art UI behavior or visual acceptance is claimed here.

## Source identities

| Basename | Bytes | SHA-256 | Role |
|---|---:|---|---|
| `MJCHARSD.DAT` | 904 | `be8bcba5d55dad6ca6909c459d65071f17ceeaf229d1d8c89013c97d5ba09410` | Major table |
| `MNCHARSD.DAT` | 8,008 | `5eea04fa7a11306b7870f584b07d09d2070e206062e0898ebb67c514acfc44eb` | Minor table |
| `TEXTSTRA.DLL` | 150,528 | `61a10bf3797f49b1121e2fba2cee7d3949a5bc7215e1501e2df71ecbedb53d4c` | Title selectors |
| `ENCYTEXT.DLL` | 145,920 | `49aea545a5e09e5fe9115a22bc785690f103d2f931e08bd4a53a617a42636d8c` | Body identities |
| `ENCYBMAP.DLL` | 34,816 | `fb545d19ae24b0277753494dbfaabf2dbdde660beab821287a32016c290e4560` | Art lookups |
| `REBEXE.EXE` | 2,822,656 | `b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab` | Static provenance |

These hashes identify the inspected profile without naming a distributor or
edition. `REBEXE.EXE` is research provenance and is not a required flattened
staging input.

## Reproduction and gates

The repository's `dat-dumper` parses and byte-round-trips both tables:

```sh
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/home/will/.cargo/bin \
  cargo run -p dat-dumper -- \
  --gdata "$REBELLION_ENCYCLOPEDIA_TEST_SOURCE/GData" \
  --file MJCHARSD.DAT --output .artifacts/encyclopedia/E38/dat-dumper

env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/home/will/.cargo/bin \
  cargo run -p dat-dumper -- \
  --gdata "$REBELLION_ENCYCLOPEDIA_TEST_SOURCE/GData" \
  --file MNCHARSD.DAT --output .artifacts/encyclopedia/E38/dat-dumper
```

The ignored, exact owned-source check reparses both tables, hashes every
148-byte row, inventories language-qualified TEXTSTRA, ENCYTEXT, and ENCYBMAP
resources through the accepted Go APIs, proves the row-title candidate/fallback
outcome, verifies every EData length/hash, checks all 60 fragment rows, and
exercises the synthetic namespace collision:

```sh
REBELLION_ENCYCLOPEDIA_TEST_SOURCE=/path/to/owned/install \
  go test -vet=off \
  -overlay=.artifacts/encyclopedia/E38/overlay.json \
  ./tools/stage-ui-assets \
  -run TestOwnedE38CharacterBindingsReconcileBothTablesAndRejectNamespaceCollisions \
  -count=1 -v
```

`-vet=off` is limited to this external virtual test file because `go vet`
cannot open the overlay-only package path. Normal package verification keeps
vet enabled. The checker and generator are retained under ignored
`.artifacts/encyclopedia/E38/`; generated DAT JSON, original bytes, source
paths, and invocation details remain outside Git.

This evidence closes only character source bindings. E40 owns combined
metadata and schema-freeze integration. Runtime catalog loading, browser
behavior, and visual acceptance remain outside E38.
