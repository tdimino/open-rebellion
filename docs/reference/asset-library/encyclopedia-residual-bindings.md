---
title: Encyclopedia residual bindings
description: Source-backed command 0x73, aggregate-only fleet, and residual resource accounting for the first supported profile
status: research-complete
last_updated: 2026-09-29
tags: [encyclopedia, DAT, ENCYTEXT, ENCYBMAP, provenance]
---

# Encyclopedia residual bindings

This note closes the residual source identities left by the E40 family merge for
the exact inspected English profile. The canonical machine-readable research
input is
[`encyclopedia-residual-bindings.json`](../../../tools/stage-ui-assets/encyclopedia_profiles/fragments/encyclopedia-residual-bindings.json).
The strict embedded profile contains the integrated result. Neither file is a
runtime catalog, and neither contains original prose or pixels.

The result is bounded to the source hashes below. Its counts are observations,
not validity rules for another installation or language.

## Result and accounting

| Source table | Header family interval | Source rows | Bound topics | Source-proven unused rows |
|---|---:|---:|---:|---:|
| `MISSNSD.DAT` | `[0x40,0x80)` | 25 | 15 under command `0x73` | 10 |
| `FLEETSD.DAT` | `[0x08,0x10)` | 2 | 1 aggregate-only | 1 |
| **Total** | — | **27** | **16** | **11** |

The 16 bound rows close 16 of E40's 17 residual ENCYTEXT resources and 32 of
its 34 residual language-1033 ENCYBMAP lookups. The exact closed exceptions are:

- ENCYTEXT resource `7176` has no admitted source definition whose canonical
  body key is `7176`;
- ENCYBMAP lookups `7188` and `11284` are the two side-qualified forms of that
  absent source selector; and
- all three are `source_proven_unused`, with zero binding references, rather
  than aliases or silently dropped unknowns.

The 29 EData files in E40's residual list are all selected by at least one of
the 32 bound lookup identities. In particular, the filenames referenced by the
two unused lookup strings are independently selected by other bound lookup
identities. A shared file is therefore retained as shared art, not converted
into a topic alias. `EDATA.192` remains the separate inventory-only deferred
file under `orlocal-2kq`; it has no binding or inferred predicate.

After integration the exact profile has 358 represented DAT source rows: 347
bound topics and 11 source-proven exclusions. Its complete source inventory has
348 text resources, 191 nonempty lookup strings, 186 distinct referenced
filenames, and 187 supplied images. The profile accounts for 347 bound text
resources plus one source-proven-unused text resource, 189 bound lookup strings
plus two source-proven-unused lookup strings, 186 bound images, and the one
publication-deferred image.

## Connected DAT registration and field dataflow

The bindings are not inferred from filenames, display text, or numeric gaps:

1. `FUN_005674e0` sends executable resource selectors `0x6b0` and `0x6b2`
   through `FUN_00567d90`. Strict type-10 inventory of the identified
   `REBEXE.EXE` maps those selectors to `MISSNSD.DAT` and `FLEETSD.DAT`.
2. `FUN_00569300` registers `[0x40,0x80)` with `FUN_00590b40` and
   `[0x08,0x10)` with `FUN_0058fdb0`. Those intervals exactly match the owned
   DAT headers.
3. `FUN_00590b40` constructs the mission definition with `FUN_00590a90`, which
   installs vtable `0x0066aac0`. Its read slot at `+0x08` is `0x00590bb0`.
   That reader calls `FUN_00584c10`, then reads the mission-specific fields into
   definition `+0x40` through `+0x94`; file offset `0x34` reaches definition
   `+0x5c`.
4. `FUN_0058fdb0` constructs the generic fleet definition. Its read slot at
   `0x0058fe20` calls `FUN_00584c10` for the complete 24-byte record.
5. `FUN_00584c10` writes the common record ID, family, title selector, and
   module selector to the definition fields used by the encyclopedia path. The
   owned rows all resolve their nonzero titles through module 2,
   `TEXTSTRA.DLL`.

The mission-reader slot and generic-reader slot were verified against the
identified executable's vtables and address-level disassembly. The tracked
Ghidra exports establish the constructors, registration, and common reader;
the exact disassembly commands and hashes are retained in ignored E54 evidence.

## Membership, availability, and navigation

`FUN_00422620` builds the master collection before any category projection. For
this scope it applies these source predicates:

- definition families `[0x50,0x80)` are admitted only when definition `+0x5c`
  is zero;
- definition families `[0x40,0x50)` are not admitted by the master-cache
  intervals, even though command `0x73` has the wider filter `[0x40,0x80)`;
- family `[0x08,0x10)` is admitted to the master cache; and
- canonical-key deduplication occurs before the ordered insertion described in
  the source contract.

The owned `MISSNSD.DAT` has four `[0x40,0x50)` rows, six `[0x50,0x80)` rows
whose definition `+0x5c` value is nonzero, and 15 admitted rows. Every row and
raw-record SHA-256 is explicit in the fragment. The table's other faction-like
fields are preserved as metadata, but this connected master-cache path does not
consult them; that is not a claim about mission eligibility elsewhere in the
game.

`FUN_0045f100` projects command `0x73` over `[0x40,0x80)`, so it returns the 15
admitted mission rows. They inherit the master collection's case-folded title
ordering and the existing skip-disabled previous/next behavior. The recovered
source does not add a second ordering rule for this command.

The one nonplaceholder `FLEETSD.DAT` row is admitted to the master cache but no
filtered command covers family `[0x08,0x10)`. It is therefore an aggregate-only
topic under command `0x6f`, not an invented filtered category. The second fleet
row has family zero and title selector zero and is source-proven unused. Context
opens may resolve the aggregate-only topic through the existing class/entity
definition route; raw entity identity still does not directly become topic
identity.

This source result contradicts the approved design's mandatory `category_id`
and one-category-per-topic shape. Aggregate membership is not silently made
optional, and command `0x6f` is not promoted into an invented topic category.
Before E09 freeze, the approved design and fixtures must be revised to represent
or explicitly scope source identity `0x08000004`, with the decision reviewed
against this source evidence. The embedded profile names this
`aggregate-only-topic-membership` blocker.

## Title, body, and faction-art selectors

For all 16 bound rows, the connected selectors are:

```text
title_resource_id           = definition +0x30
row_title_candidate_id      = uint16(title_resource_id - 0x8000)
row_title_fallback_if_empty = cached title at definition +0x34
body_resource_id            = (title_resource_id & 0x0fff) + 0x1000
Alliance art lookup         = body_resource_id
Empire art lookup           = body_resource_id + 0x1000
```

The preferred title candidate is empty and the original selector fallback is
nonempty for every bound row in this exact profile. Every selected body exists
as a numeric type-10 resource at LANGID 1033. `FUN_0045fa60` keeps the body key
fixed while choosing the two art lookup keys for source side 1 and side 2. The
fragment records both variants with exact lookup identity, EData basename,
length, and SHA-256; the strict validator rejects missing, duplicated, swapped,
or noncanonical faction variants.

These two source-proven viewer-faction selectors are not speculative alternate
art. They do not authorize an `EDATA.192` binding, campaign predicate, or UI
switch.

## Source identities

| Basename | Bytes | SHA-256 | Role |
|---|---:|---|---|
| `MISSNSD.DAT` | 2,816 | `114d1cfc5abb41e57f3bd76d8993503efefc5daa5cdeb0abdfbc5608fefd25a5` | Mission definitions |
| `FLEETSD.DAT` | 64 | `ee5c3c6a46b8f86a518ca88a95035063edb2920d26d8ff69775299f43ac0c25e` | Aggregate-only fleet definition |
| `TEXTSTRA.DLL` | 150,528 | `61a10bf3797f49b1121e2fba2cee7d3949a5bc7215e1501e2df71ecbedb53d4c` | Title selectors |
| `ENCYTEXT.DLL` | 145,920 | `49aea545a5e09e5fe9115a22bc785690f103d2f931e08bd4a53a617a42636d8c` | Body identities |
| `ENCYBMAP.DLL` | 34,816 | `fb545d19ae24b0277753494dbfaabf2dbdde660beab821287a32016c290e4560` | Art lookups |
| `REBEXE.EXE` | 2,822,656 | `b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab` | Static provenance |

`REBEXE.EXE` is research provenance only; it is not required in flattened
staging input and is never executed by these checks.

## Reproduction and remaining gate

The existing DAT dumper byte-round-trips both source tables:

```sh
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/home/will/.cargo/bin \
  cargo run -q -p dat-dumper -- \
  --gdata "$REBELLION_ENCYCLOPEDIA_TEST_SOURCE/GData" \
  --file MISSNSD.DAT --output .artifacts/encyclopedia/E54/dat-dumper

env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/home/will/.cargo/bin \
  cargo run -q -p dat-dumper -- \
  --gdata "$REBELLION_ENCYCLOPEDIA_TEST_SOURCE/GData" \
  --file FLEETSD.DAT --output .artifacts/encyclopedia/E54/dat-dumper
```

The permanent owned check reparses both DATs, hashes every row, re-inventories
the title and art DLLs with the accepted readers, and verifies each bound or
excluded row against the strict embedded profile:

```sh
REBELLION_ENCYCLOPEDIA_TEST_SOURCE=/path/to/owned-install \
  go test ./tools/stage-ui-assets \
  -run 'TestOwnedEncyclopedia(CombinedBindingProfileReconcilesInventories|ResidualBindingsReconcileSourceRowsAndSelectors)$' \
  -count=1 -v
```

This closes command `0x73` and E40's residual source identities for the
inspected profile. It does not freeze the runtime schema. E09 must resolve the
aggregate-only membership design contradiction above and integrate accepted E55
localized-label evidence from commit
`9ae03be852212fa2ff031f8d2d065a20c2767575`; that commit is not part of this
E54 worktree. This note also does not claim runtime, browser,
navigation-capture, or visual acceptance.
