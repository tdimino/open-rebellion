---
title: "Original Encyclopedia Category-Label Evidence"
status: source-backed fragment pending E09 integration
date: 2026-09-29
---

# Original encyclopedia category-label evidence

This note recovers the localized label source for the seven original
encyclopedia commands `0x6f..0x75`. The canonical metadata-only result is
[`encyclopedia-category-labels.json`](../../../tools/stage-ui-assets/encyclopedia_profiles/fragments/encyclopedia-category-labels.json).
It is deliberately outside `encyclopedia_profiles/*.json`: E09 owns the later
schema decision and E54 owns the shared profile/source-contract paths. No
localized label text, original pixels, or binary bytes are stored here.

The result applies to the reviewed English source profile only:

| Source | Bytes | SHA-256 | Role |
|---|---:|---|---|
| `REBEXE.EXE` | 2,822,656 | `b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab` | control construction and string-resource dataflow |
| `TEXTSTRA.DLL` | 150,528 | `61a10bf3797f49b1121e2fba2cee7d3949a5bc7215e1501e2df71ecbedb53d4c` | localized category-label resources |

## Connected source path

The labels are string resources, not baked bitmap prose and not names inferred
from category filters or artwork:

1. [`FUN_00414830`](../../../ghidra/notes/FUN_00414830.c) pushes module slot
   `2` at `0x0041495c`, resolves the literal `textstra.dll`, and registers it
   through `FUN_005fba30` at call address `0x00414966`. This proves that the
   high word `2` later copied from `DAT_0065d424` means `TEXTSTRA.DLL`; it must
   not be guessed to mean `STRATEGY.DLL`.
2. [`FUN_0045ddc0`](../../../ghidra/notes/FUN_0045ddc0.c) creates the seven
   category controls. For each control it copies that module-slot template,
   writes one low-word selector `0x1850..0x1856`, then calls
   [`FUN_00600a40`](../../../ghidra/notes/FUN_00600a40.c) to attach the resolved
   string to the control. The selectors are not in display order.
3. The string callback `FUN_0060aa30` resolves the high word through
   `FUN_005fefd0` and calls `LoadStringA` with the low word at `0x0060aa62`.
   `LoadStringA` has no explicit LANGID argument; the owned DLL contains one
   observed entry for this block, LANGID 1033, so this profile is unambiguous.
   `FUN_00600a40` obtains the resulting buffer through `FUN_005f2fc0` and
   copies it to control field `+0x84` through `FUN_00600970`.
4. When a category becomes current,
   [`FUN_0045f100`](../../../ghidra/notes/FUN_0045f100.c) finds the child by its
   command at `0x0045f410`, reads that child's attached text at `0x0045f415`,
   and calls `FUN_00601aa0` at `0x0045f422` to update the selected-category
   static. This connects construction to the displayed label.

The separate resource `0x1843` is rendered into the index background, as
documented by the [UI contract](encyclopedia-ui-contract.md#client-composition).
Its role does not replace or define the seven category-control labels.

## Exact command and selector mapping

Display order follows increasing control x-coordinate. Source construction
order is `0x6f, 0x73, 0x72, 0x75, 0x71, 0x70, 0x74`; keeping both orders is
essential because construction order is visibly non-linear. The filter ranges
below are the already accepted semantic identities from E40. They corroborate
which command is aggregate or filtered, but they were not used to invent label
text.

All selectors resolve to `TEXTSTRA.DLL`, `RT_STRING` type 6, block 390,
LANGID 1033, code page 0. The block is 308 bytes with SHA-256
`e428a70d970e78e7979356426fb8da15b49c06cc18cac8af8cf8d0f9d75b6d3d`.
The content columns contain byte length and digest only.

| Display | Command | Semantic role | Filter `[start,end)` | x | Construction | Selector / slot | UTF-8 bytes | UTF-8 SHA-256 | command push / label attach |
|---:|---:|---|---|---:|---:|---|---:|---|---|
| 0 | `0x6f` | aggregate index | none | 0 | 0 | `0x1850` / 0 | 13 | `6eb4b5f6d5723be1f6c1c215af21ca9149462b40087cafb6637777f97d57eeae` | `0x0045e822` / `0x0045e866` |
| 1 | `0x70` | filtered category | `[0x90,0x98)` | 52 | 5 | `0x1855` / 5 | 15 | `d7b3e461cbdbaa15b2b96188ee474e1de4515ba105232325f43e45c6a5d31c60` | `0x0045eba3` / `0x0045ebe3` |
| 2 | `0x71` | filtered category | `[0x14,0x20)` | 104 | 4 | `0x1854` / 4 | 13 | `ec24d33c0dbd2405fcf24d105925e25124ae419f49b7c5e04d764e71ced8a1a2` | `0x0045eb0f` / `0x0045eb52` |
| 3 | `0x72` | filtered category | `[0x20,0x30)` | 156 | 2 | `0x1852` / 2 | 19 | `155e2c2bbc3c90dd466fb31a72f53d2c8194d4fb5cc94465530c247e3df52c70` | `0x0045e997` / `0x0045e9de` |
| 4 | `0x73` | filtered category | `[0x40,0x80)` | 208 | 1 | `0x1851` / 1 | 17 | `df53b255f1559fe2354e3e9ce6eff356609b663384d0eb26c033c91840329163` | `0x0045e8d7` / `0x0045e922` |
| 5 | `0x74` | filtered category | `[0x10,0x14)` | 260 | 6 | `0x1856` / 6 | 14 | `e9f59e5a11146ecf63026e64239d27623e5b3554ac517ab94a9bebbf0c76c8c4` | `0x0045ec56` / `0x0045ec99` |
| 6 | `0x75` | filtered category | `[0x30,0x40)` | 312 | 3 | `0x1853` / 3 | 18 | `ca001895d7cc850d888a3009136ba631f615bb39c46578a42aeb026e111064be` | `0x0045ea4f` / `0x0045ea9a` |

Command `0x6f` is the aggregate index command. It intentionally has no family
filter. Commands `0x70..0x75` are the six filtered categories. A downstream
consumer must retain that distinction rather than assigning an invented family
name to `0x6f`.

## Empty, absent, and fallback behavior

There is no alternate category-label selector and no fallback selector in the
recovered call path. `FUN_005f37e0` initializes its result buffer empty before
calling the resource callback. `LoadStringA` returns zero for both an absent
selector and a present zero-length string; the runtime therefore presents both
cases as an empty label. `FUN_00600a40` still attaches that empty buffer unless
allocation itself failed. A zero module or resource selector is not used by
any of these seven fixed entries.

Static PE inventory supplies the distinction the runtime loses: in this
profile, block 390 is present for the single observed language/code-page pair,
and all seven selector slots are present and nonempty. The fragment records
both the general runtime behavior and this source-profile observation. A later
profile with multiple matching languages must be rejected as ambiguous until
its selection rule is proved; it must not silently choose one.

## Reproduction and retained evidence

The permanent synthetic tests parse the fragment strictly and reject missing
or duplicate commands, changed display/construction order, wrong executable or
DLL identities, wrong dataflow addresses, ambiguous language evidence, and
missing provenance. The opt-in owned test uses the fragment rows as inputs,
reads `RT_STRING` block 390 through the existing bounded PE reader, and compares
presence, language, code page, byte length, and digest without logging or
retaining decoded prose:

```text
env REBELLION_ENCYCLOPEDIA_TEST_SOURCE='/home/will/Star Wars - Rebellion' \
  PATH=/home/will/.cargo/bin:/usr/bin:/bin:/home/will/.local/bin \
  go test ./tools/stage-ui-assets \
  -run TestOwnedEncyclopediaCategoryLabelEvidenceMatchesOriginalResourcesWithoutPrintingProse \
  -count=1 -v
```

Focused disassembly excerpts and probe output are retained under ignored
`.artifacts/encyclopedia/E55/`; they contain addresses and metadata only. The
owned permanent gate writes the same bounded result to ignored
`.artifacts/encyclopedia/E55-owned-category-labels.json`.

## Downstream decision and limits

E09 can use this fragment to define localized category labels without adding
English internal-key prose: the catalog must bind each stable command identity
to its reviewed language-qualified source selector (or a separately reviewed
localized derivative). Display order and source construction order are
separate facts. A missing/empty selector is not permission to synthesize a
category name.

This result closes label provenance only. It does not freeze the public schema,
publish a catalog, implement runtime UI, claim an A0 capture, or change the
approved alternate-art deferral. Reverting this evidence requires removing
only this note, its fragment, and its focused tests; existing profiles and
original inputs remain untouched.
