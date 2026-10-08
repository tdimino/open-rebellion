---
title: "Encyclopedia native title-search correction"
date: 2026-10-08
status: pass
bead: orlocal-818.69
---

# Encyclopedia native title-search correction

## Defect

The source-backed Encyclopedia painted the current topic name over the original
blue selector field, but the field was not focusable or editable. Pointer focus,
incremental title lookup, clearing, and Enter-to-open were therefore absent.

## Recovered contract

The original field is control `0x64` at `(143,45,245,18)`. Its edit procedure
posts notification `0x408` after text changes. `FUN_0045d8f0` responds by
calling `FUN_00609650` against the current index list and storing the selected
record. Notification `0x407` opens that record in topic mode on Enter.

`FUN_00609650` is deliberately naive: it selects the first ordered title with
the longest case-insensitive common prefix. It does not require the full query
to match, so `tallon` selects `Talon Karrde`. Empty input clears selection.
Pointer or list selection copies the selected title into the field, while a
category rebuild reapplies the current query.

The implementation keeps that behavior in renderer-local state and emits only
the existing presenter actions plus an explicit clear-selection action. It
does not add substring, token, or fuzzy matching and performs no I/O.

## Verification

- `cargo check -p rebellion-app --tests` passed. The only app diagnostics were
  the two existing macro trailing-semicolon warnings.
- The focused matcher tests cover typo retention, ASCII case folding, ordered
  tie/no-prefix behavior, and empty input: three tests passed (a temporary
  linker-only `libasound.so` alias targeted the installed `libasound.so.2`;
  no repository or system library was changed).
- `cargo test -p rebellion-render encyclopedia --lib` passed 42 tests; the
  matching `rebellion-app` filter passed nine tests with one owned-data test
  intentionally ignored.
- `REBELLION_EDATA_DIR=/data/projects/open-rebellion/open-rebellion/data/base/EData bash scripts/build-interface-test-wasm.sh`
  passed its 38-scenario / 152-execution validator and fixture exclusion gate.
- `REBELLION_EDATA_DIR=/data/projects/open-rebellion/open-rebellion/data/base/EData node tools/interface-parity/encyclopedia-canonical-surface.mjs`
  passed all 12 Alliance/Empire canonical cases with zero browser errors and
  zero navigation requests.

The browser journey clicks the real canvas field, types `tallon`, observes
object `0x3800034d` (`Talon Karrde`), clears the field and observes a null
selection, retypes the query, and presses Enter. It also switches from All
Databases to Personnel and back while retaining the query and match. The opened `EDATA.091` artwork
matched the owned bitmap with zero differing pixels in both faction journeys.
All existing category, first/last, long-body, contextual-return, close, and
scaled-render journeys remained green.

Ignored local evidence:

- run directory: `.artifacts/interface-parity/encyclopedia-canonical-surface-2026-10-08T14-52-24-481Z-936906/`
- summary SHA-256: `6db3859094f598ed6f6f5ca21c9d511f11d97d47d5c5cbc4eac94fbc2af57854`
- fixture WASM SHA-256: `11ce614a8108f1a4279d7e4a19eaa5066f5b2cb404332ec169b6662b43cb0519`
- final production WASM SHA-256: `228de3dad2f1935394fcb6bb42ad6340d144ecaf586a0d874237dbab0ea60d11`
- runtime pack SHA-256: `e4c0157a8701aa1631f642db04c71b5e01d7280cf3e0cb691b428c502a1afa4e`
- visible catalog fingerprint: `20c342868cee50e80ef3b94b9f81a898c48f4b593ddd0d83ab69b67d755ae9ea`

Owned EData, generated runtime packs, screenshots, and browser profiles remain
ignored and uncommitted.
