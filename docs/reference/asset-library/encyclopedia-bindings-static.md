# Encyclopedia system and facility binding evidence

Status: complete for the identified English source profile

Profile: `encytext-49aea545-lang-1033-v1`

Machine-readable record: `tools/stage-ui-assets/encyclopedia_profiles/fragments/static-bindings.json`

Merge owner: E40 (`orlocal-818.41`)

This note records source-backed title, body, artwork, identity, and category
bindings for world-location systems and the defense, manufacturing, and
production facility tables. It contains metadata only. It does not contain
original prose or pixels, does not define a runtime schema, and does not claim
runtime or visual acceptance.

The findings apply only when the source hashes in the fragment match. Counts
are observations of that profile, not universal validity rules.

## Result and accounting

| Source table | Source family interval | Rows | Result |
|---|---:|---:|---|
| `SYSTEMSD.DAT` | `[0x90, 0x98)` | 200 | 200 bound |
| `DEFFACSD.DAT` | `[0x22, 0x28)` | 6 | 6 bound |
| `MANFACSD.DAT` | `[0x28, 0x2c)` | 6 | 6 bound |
| `PROFACSD.DAT` | `[0x2c, 0x30)` | 2 | 2 bound |

All 214 scoped rows reconcile to a language-1033 title, ENCYTEXT body, and
ENCYBMAP/EData artwork reference. There are no unresolved scoped rows and no
proven title or body aliases: the profile has 214 distinct title selectors,
214 distinct body selectors, and 214 distinct body byte hashes. The 214 rows
use 40 distinct art lookup IDs/files/hashes. Seventeen system art resources
are deliberately shared by multiple system rows; those groups and every
member identity are explicit in the fragment.

## Stable source identity

The stable key is `(source_family, dat_id)`. The fragment's diagnostic packed
form is `(source_family << 24) | (dat_id & 0x00ffffff)`, rendered as eight hex
digits. A raw `dat_id` is not a key.

The four tables contain 214 family-qualified identities but only 206 distinct
raw IDs. Raw IDs 1 through 6 occur in more than one facility family. Both the
owned checker and its synthetic collision case reject a duplicate composite
identity and reject resolving a repeated raw ID by choosing the first match.
E40 must retain family qualification; it must not renumber records or infer a
join from a display name or vector position.

## Connected source paths

### Table registration and field writes

`FUN_00569300` registers the source-family intervals with their factories:

- `[0x90,0x98)` -> `FUN_00590180` for systems
- `[0x22,0x28)` -> `FUN_00591aa0` for defense facilities
- `[0x28,0x2c)` -> `FUN_005913f0` for manufacturing facilities
- `[0x2c,0x30)` -> `FUN_00590980` for production facilities

The corresponding reader paths are recorded in the fragment. Each reaches
`FUN_00584c10`, whose writes at
[`ghidra/notes/FUN_00584c10.c`](../../../ghidra/notes/FUN_00584c10.c) lines
14-16 place the common DAT fields at definition `+0x2c`, title selector
`+0x30`, and title-module selector `+0x32`. The system reader additionally
writes the DAT picture selector (file offset 28) to definition `+0x44`.
These are source fields and connected writes, not renderer-offset guesses.

In this profile every scoped row selects title module 2. `FUN_00414830`
registers `textstra.dll` as module 2 at
[`ghidra/notes/FUN_00414830.c`](../../../ghidra/notes/FUN_00414830.c) lines
61-65. `FUN_00567d90` resolves the selector stored at definition `+0x30` and
caches the result at `+0x34` (lines 36-43 in
[`ghidra/notes/FUN_00567d90.c`](../../../ghidra/notes/FUN_00567d90.c)).

### Title selection and fallback

During master-cache construction, `FUN_00422620` computes a row-preferred
title candidate as `(title_selector - 0x8000) & 0xffff` and attempts that
resource first. For definitions, an empty candidate falls back to the cached
title at definition `+0x34`; see lines 178-207 in
[`ghidra/notes/FUN_00422620.c`](../../../ghidra/notes/FUN_00422620.c).
Systems use the same preferred candidate and then `FUN_004f62d0`, which returns
the system-view title at `+0x34` when present, otherwise its definition's
cached title at `+0x34`; the connected system construction is at lines
219-235.

The preferred candidates are empty for all 214 rows in this inspected profile,
so the observed branch is preferred-empty then fallback. That observation is
retained per row; it is not generalized into a rule that the preferred branch
can never contain a title.

### Body selector

`FUN_00422620` constructs the canonical topic/body key as
`(title_selector & 0x0fff) + 0x1000` (definition lines 183-185; system lines
221-223). `FUN_0045fa60` takes the selected row's key at `+0x0c`, loads its
type-10 ENCYTEXT resource, and stores the resulting narrow string into the
body object. The connected load and body dataflow are at lines 101-131 of
[`ghidra/notes/FUN_0045fa60.c`](../../../ghidra/notes/FUN_0045fa60.c).
The accepted decoder profile owns byte decoding; this fragment retains the
raw body length and SHA-256 for every selected resource.

### Facility artwork

For these facility families, `FUN_0045fa60` uses the canonical topic key as
the ENCYBMAP lookup ID. The faction-specific artwork branches do not cover
families `[0x22,0x30)`. All 14 facility references reconcile exactly through
the language-1033 ENCYBMAP lookup and to an EData file. The mapping is not
assumed contiguous: one defense row uses lookup 4736 and resolves to
`EDATA.014`, while neighboring source rows resolve to the other proven
facility files. The fragment records every exact lookup ID, matched basename,
source length, and source hash.

### System artwork

The connected system path is
`FUN_0045fa60 -> FUN_004f3220 -> FUN_00509610 -> FUN_0045f660 ->
FUN_0045f970`. `FUN_00509610` reads the definition pointer at view `+0x2c`
and then the picture selector at definition `+0x44`; see
[`ghidra/notes/FUN_00509610.c`](../../../ghidra/notes/FUN_00509610.c) lines
5-6. `FUN_0045f660` maps picture IDs 1 through 23 to
`0x2b5c + picture_id - 1`, then maps 24 to `0x2b75`, 25 to `0x2b73`, and 26
to `0x2b74`; the nonlinear tail is explicit at lines 51-60 of
[`ghidra/notes/FUN_0045f660.c`](../../../ghidra/notes/FUN_0045f660.c).

The 200 system rows use picture IDs 1 through 26. All 26 resulting lookup IDs
reconcile through language-1033 ENCYBMAP and to EData metadata. Shared use is
preserved as explicit shared-resource groups, not collapsed into aliases.

## Category membership and availability

The recovered source exposes two selectors for this scope:

| Stable category decision | Command | Source filter | Membership source |
|---|---:|---:|---|
| systems/world locations | `0x70` | `[0x90,0x98)` | viewer-side system iterator, excluding an ancestor of type `0xf2` |
| facilities | `0x72` | `[0x20,0x30)` | master definition cache admits scoped families `[0x22,0x30)` |

The command filters are connected at lines 57-76 of
[`ghidra/notes/FUN_0045f100.c`](../../../ghidra/notes/FUN_0045f100.c).
`FUN_00422620` builds the underlying master collection and admits the scoped
facility intervals at lines 108-153. Families `[0x20,0x22)` are not admitted
by that cache, so numbering gaps must not create topics.

Defense, manufacturing, and production are three source tables within one
recovered facility selector, not evidence for three separately invented UI
categories. No additional original category is required to account for this
scope. E40 must merge these two decisions with the other family fragments
without silently splitting facilities or materializing gap IDs.

The facility DAT alliance/empire build flags are preserved as source metadata.
The connected master-cache path does not consult them when admitting facility
encyclopedia definitions, so they do not hide topics in this source path. This
is not a broader claim about buildability or gameplay availability. System
membership is context-derived through the viewer-side iterator rather than a
claim that every static system definition is always listed.

## Aliases, absence, and deferred art

- No scoped title/body alias is proven in this profile; all body hashes are
  distinct.
- Seventeen system artwork identities are proven shared. The fragment lists
  their exact member keys.
- No source row is dropped, and no topic is created from a numbering gap.
- `EDATA.192` is inventory-only and unused. Its original display behavior and
  any alternate predicate remain unproven and deferred to `orlocal-2kq`.
  File presence is not a binding.

## Source identities

| Basename | Bytes | SHA-256 |
|---|---:|---|
| `SYSTEMSD.DAT` | 8,816 | `6eb60fd2f5ce9ce8f72cf96033e31b84dc38093b77dad13036adef2f4e20fdd2` |
| `DEFFACSD.DAT` | 376 | `0800ada559f09acf201fd8faa41e5b8a8475a69812ce6cf6a96dbc006603ed3d` |
| `MANFACSD.DAT` | 352 | `be2955e57d862eb00fd39ed7cc8a5f3f4a4f575565e16f27e560970c264e0ed7` |
| `PROFACSD.DAT` | 128 | `dc93413d5217ab9fc1a0c35cf0cdf84fc590698337a31aed4e9fdc360680d631` |
| `TEXTSTRA.DLL` | 150,528 | `61a10bf3797f49b1121e2fba2cee7d3949a5bc7215e1501e2df71ecbedb53d4c` |
| `ENCYTEXT.DLL` | 145,920 | `49aea545a5e09e5fe9115a22bc785690f103d2f931e08bd4a53a617a42636d8c` |
| `ENCYBMAP.DLL` | 34,816 | `fb545d19ae24b0277753494dbfaabf2dbdde660beab821287a32016c290e4560` |
| `REBEXE.EXE` | 2,822,656 | `b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab` |

`REBEXE.EXE` is research provenance only and is not a required flattened
staging input.

## Reproduction

With an owned installation, round-trip each DAT through the existing codec:

```sh
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/tomdimino/.cargo/bin \
  cargo run -q -p dat-dumper -- \
  --input "$REBELLION_ENCYCLOPEDIA_TEST_SOURCE/GData/SYSTEMSD.DAT" \
  --output /tmp/SYSTEMSD.json --roundtrip
```

Repeat with `DEFFACSD.DAT`, `MANFACSD.DAT`, and `PROFACSD.DAT`. The ignored
E39 evidence directory retains the exact outputs and commands used for this
profile.

The ignored owned-source checker parses all four DATs, validates the source
hashes, re-inventories TEXTSTRA/ENCYTEXT/ENCYBMAP/EData through the accepted
Go readers, and compares those independent observations with every fragment
row:

```sh
REBELLION_ENCYCLOPEDIA_TEST_SOURCE=/path/to/owned/install \
  go test -vet=off \
  -overlay=.artifacts/encyclopedia/E39/overlay.json \
  ./tools/stage-ui-assets \
  -run TestOwnedE39StaticBindingsReconcileEverySourceRowAndRejectAmbiguity \
  -count=1 -v
```

The overlay is ignored because it opens owned source inputs. Normal package
tests and vet run without an overlay. The checker proves metadata
reconciliation, not runtime rendering, UI geometry, or visual acceptance.
