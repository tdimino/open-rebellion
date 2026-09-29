---
title: "Encyclopedia Class Bindings: Mobile Units"
status: source-backed fragment pending E40 integration
date: 2026-09-29
---

# Encyclopedia class bindings: mobile units

This note recovers the first-profile encyclopedia bindings for capital-ship,
fighter, troop, and special-force class records. It does not cover characters,
systems, facilities, fleet instances, deployed unit instances, category-label
localization, or the global profile join. The canonical machine-readable result
is [`unit-bindings.json`](../../../tools/stage-ui-assets/encyclopedia_profiles/fragments/unit-bindings.json).
It is deliberately outside `encyclopedia_profiles/*.json`: E40 owns the reviewed
global merge, and the staging decoder must not treat this provisional fragment
as an independently complete source profile.

No original name, prose, or image bytes are stored here. The fragment contains
only numeric identities, filenames, hashes, citations, and status.

## Recovered dataflow

The join is connected through the original loader and encyclopedia caller; it
does not use names, renderer offsets, or positional assumptions:

1. `FUN_005674e0` registers the four files through `FUN_00567d90` using original
   resource selectors `0x6ac` (`CAPSHPSD.DAT`), `0x6ab` (`FIGHTSD.DAT`), `0x6aa`
   (`TROOPSD.DAT`), and `0x6af` (`SPECFCSD.DAT`). `FUN_005952a0` resolves the
   `0x642` directory resource and those file resources before opening them.
2. The shared record reader at `0x00584c10` calls `FUN_005f5610` to write the
   first DAT `u32` to definition `+0x18`, then writes the record family to
   `+0x2c`, the 16-bit TEXTSTRA selector to `+0x30`, and `field7` to `+0x32`.
   The existing Rust DAT dumper round-trips those same fields and all remaining
   record bytes.
3. `FUN_00402e40` constructs the source identity by retaining record `+0x18`
   as the low 24 bits and moving record `+0x2c` into the high byte. Conversely,
   `FUN_0051cab0` -> `FUN_00567790` uses the high byte to choose the family and
   the low 24 bits to find the class record. The stable identity is therefore
   `(record_family << 24) | record_id`, not the numeric record ID alone.
4. After each DAT load, `FUN_00567d90` resolves definition `+0x30` through
   `FUN_005f2fc0` and stores that fallback display title at `+0x34`. The scoped
   records' `field7` value is 2. The callback at `0x0060aa30` uses the selector
   pair's high word to resolve that module slot through `FUN_005fefd0`, then
   passes its low word to `LoadStringA`; original startup paths `FUN_00414830`
   and `FUN_00415440` register `TEXTSTRA.DLL` in slot 2. This connects the DAT
   selector to TEXTSTRA rather than inferring a title from a name.
5. `FUN_00422620` preserves the original `definition +0x30` selector in
   `local_84` and derives the body key from it as
   `(local_84 & 0x0fff) + 0x1000`. Title selection is a separate branch: it
   keeps the same module-slot high word, wraps the low word to
   `uint16(title_selector - 0x8000)`, and tries that preferred TEXTSTRA string.
   Only when `FUN_005f3070` reports the preferred result empty does it copy the
   already-resolved definition `+0x34` fallback. The owned profile has an empty
   preferred result for all 57 rows, so every selected title source is the
   original DAT selector; the body still always uses that original selector.
   This observed fallback outcome is recorded per row rather than assumed as a
   universal rule.
6. None of the scoped source-family bytes (`0x10`, `0x14`, `0x18`, `0x1c`,
   `0x3c`) enters either viewer-faction remap range in `FUN_0045fa60`
   (`[0x40,0x80)` or `[0x08,0x10)`). Its art key is therefore the canonical
   topic key unchanged. The accepted E05 decoder then resolves that LANGID 1033
   ENCYBMAP logical ID to the recorded EData filename.

### Class, not instance

`FUN_0045d400` handles class and entity contexts separately. A class context is
looked up directly through `FUN_0051cab0`. An entity context first resolves via
`FUN_004f2d10`, follows the returned object's definition pointer, and uses that
definition's `+0x30` to find the same canonical master topic. Consequently the
catalog binding belongs to the class definition. A fleet, capital-ship instance,
fighter group, deployed troop, or deployed special-force instance must resolve
its class at the application boundary; none of those instance identities occurs
in this fragment.

## Record accounting

Every scoped record is represented explicitly in the JSON fragment. The compact
rows use the declared `record_columns`; decimal `source_identity` values are
lossless JSON representations of the family-qualified identifiers.

| Family | DAT records | Record families | Definition/fallback title selectors | Preferred-title outcome | Body/art lookup IDs | EData files | Status |
|---|---:|---|---|---|---|---|---|
| Capital-ship classes | 30 | `0x14`, except record 136 is `0x18` | 10048-10062, 10112-10126 | all preferred selectors empty; fallback selected | 5952-5966, 6016-6030 | 042-071 | 30 bound |
| Fighter classes | 8 | `0x1c` | 9792-9795, 9856-9859 | all preferred selectors empty; fallback selected | 5696-5699, 5760-5763 | 034-041 | 8 bound |
| Troop classes | 10 | `0x10` | 9280-9284, 9344-9348 | all preferred selectors empty; fallback selected | 5184-5188, 5248-5252 | 015-024 | 10 bound |
| Special-force classes | 9 | `0x3c` | 9536-9539, 9600-9604 | all preferred selectors empty; fallback selected | 5440-5443, 5504-5508 | 025-033 | 9 bound |

The total is 57 bound records, 57 distinct body resources, and 57 distinct art
files. The 57 preferred title selectors were also checked individually: all
decode as empty, and all 57 rows therefore record the definition selector as
the selected fallback. There are no scoped aliases, missing mappings,
source-proven absences, or unresolved records. This is a scoped result, not a
claim about other source families. Shared numeric record IDs demonstrate why
family qualification is mandatory: IDs 1-8 occur in the fighter, troop, and
special-force files but resolve to different source identities, text resources,
and art.

The art sequence happens to be contiguous inside these observed groups, but the
join is not `record_id + offset`: for example, numeric record 1 maps to three
different topic/art IDs in the three families, while capital record 64 maps to
lookup 5952 / `EDATA.042`. Publication must retain the explicit lookup results
and must never fill filename gaps or extrapolate from those examples.

## Reproduction

All commands were run from the E07 worktree with a sanitized PATH. Original
inputs remained read-only; generated dumps and detailed observations were kept
under ignored `.artifacts/encyclopedia/`.

```text
env PATH=/home/will/.cargo/bin:/usr/bin:/bin:/home/will/.local/bin \
  RCH_VISIBILITY=summary rch exec -- cargo run -p dat-dumper -- \
  --gdata '/home/will/Star Wars - Rebellion/GData' \
  --file CAPSHPSD.DAT --output .artifacts/encyclopedia/E07-dat-dumps
```

The same command was repeated for `FIGHTSD.DAT`, `TROOPSD.DAT`, and
`SPECFCSD.DAT`. The round-tripped dump counts and raw source identities were:

| Source | Bytes | SHA-256 | Records | Retained dump SHA-256 |
|---|---:|---|---:|---|
| CAPSHPSD.DAT | 6016 | `36fdaaeef4057d70e10a1593950cee5b197ce38d7d1b6a06201a74c93bf44515` | 30 | `e92b4e56df238f75a9ddaa21a4edd2afa14910c94910d0585bca34f568388adf` |
| FIGHTSD.DAT | 1360 | `b62598d9a35fa3fe9fb65622f2720d10e6fabba055ccba41b87bdf07895268a5` | 8 | `883eb670a8e06c9410b8f018e62be997e9063e3c7c489ee30094fc90dce9632c` |
| TROOPSD.DAT | 696 | `b70a0ca33f0956d8528af9f784c4094c2a1609ad7be437c21ae2b90f6dd4eace` | 10 | `c32e4c96ddc63e7b4ad22d82564524c02c15947e1be4e844bcc649b41d781662` |
| SPECFCSD.DAT | 1060 | `d61fc07ae332e5cfb15f644b1198e0ec1905903c2a3b6bc4886cae10c4361e46` | 9 | `c170b523dfc37e143d68c3f8fbf688e008f1e9304ce9f61f7af7309ec7d10c47` |

The owned-resource probe uses the fragment rows as its inputs rather than
reconstructing hard-coded selector ranges. For every row it applies the exact
preferred/fallback title branch, checks the recorded preferred status and
selected selector, checks the ENCYTEXT body resource, compares the exact
language-qualified ENCYBMAP filename, and confirms the referenced EData file.
Its retained evidence contains only selectors, statuses, byte lengths, and
hashes—never decoded title/body prose or pixels. Synthetic cases separately
prove that a nonempty preferred title wins and an empty preferred title falls
back:

```text
env REBELLION_ENCYCLOPEDIA_TEST_SOURCE='/home/will/Star Wars - Rebellion' \
  PATH=/home/will/.cargo/bin:/usr/bin:/bin:/home/will/.local/bin \
  go test -vet=off \
  -overlay=.artifacts/encyclopedia/E07-unit-lookups-overlay.json \
  ./tools/stage-ui-assets -run TestE07OwnedUnitLookupSelectors -count=1 -v
```

The resulting `.artifacts/encyclopedia/E07-unit-lookups.json` has SHA-256
`a4a1bc51b9cd03ce62c81cf37c0f609dbb3ac0029c1ecf068e8a39d0d6c8aabc`.
It includes raw body and image lengths and hashes without retaining prose or
pixels. The original executable's base-DAT reader disassembly excerpt is kept
ignored as `E07-base-dat-reader-disassembly.txt` with SHA-256
`ea509a779f946b0ef21f6cf9638734c6707e568443a470d09a0fa21f310c67c2`.

## Integration limits

- E40 must merge this fragment with the other family fragments and the reviewed
  category/global metadata; this file alone is not a complete staging profile.
- The current port's capital-ship and fighter class arenas still use unqualified
  record IDs in their constructors, whereas troop and special-force class maps
  already restore their family byte. E40/runtime integration must reconcile the
  app boundary to the source identities above rather than weakening this join or
  rebinding to instances.
- The capital file header family is `0x14`, but record 136 has source family
  `0x18`. `FUN_00567790` extracts that high byte and calls `FUN_0058ec90`, which
  walks registered DAT containers and selects the one whose header range is
  `family_id <= family < field4`. The capital container range is `[20,28)`, so
  family 24 correctly resolves inside it. The per-record family remains the
  source identity consumed by `FUN_00402e40`; replacing it with the header's
  lower bound would silently change that identity.
- Alternate artwork remains deferred under `orlocal-2kq`. No alternate binding,
  guessed campaign predicate, or unused `EDATA.192` mapping appears here.
- This is source/static evidence. It does not claim browser rendering, original
  A0 capture acceptance, global schema freeze, or complete encyclopedia scope.
