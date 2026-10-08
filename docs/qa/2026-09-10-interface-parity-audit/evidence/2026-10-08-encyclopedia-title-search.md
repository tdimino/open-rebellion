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

The first production implementation also allowed title-field keystrokes to
reach Galaxy shortcuts beneath the modal. Queries containing `S` or `R` could
therefore open Save/Load or reset the map, and Escape could close the
Encyclopedia before the same frame evaluated the cockpit shortcuts.

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

Keyboard ownership is now sampled once at the beginning of each frame. An
Encyclopedia that was open at that point retains the complete frame, including
the frame in which Escape closes it. The small owner enum is reusable by later
modal surfaces without changing the behavior of intentionally non-modal
cockpit panels.

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
- The full workspace, Go staging, Python packaging, Node acceptance, ledger,
  formatting, and diff gates passed on the final revision.
- The production browser journey passed both factions, the old-pack
  unavailable case, exact four-request startup, contextual return, and zero
  page errors. A separate fresh muted Chromium pass typed `star` with real key
  events in both factions, selected `Star Galleon`, and returned to the command
  center with one Escape without exposing Save/Load or another panel.

The browser journey clicks the real canvas field, types `tallon`, observes
object `0x3800034d` (`Talon Karrde`), clears the field and observes a null
selection, retypes the query, and presses Enter. It also switches from All
Databases to Personnel and back while retaining the query and match. The opened `EDATA.091` artwork
matched the owned bitmap with zero differing pixels in both faction journeys.
All existing category, first/last, long-body, contextual-return, close, and
scaled-render journeys remained green.

Latest ignored local evidence:

- canonical run: `.artifacts/interface-parity/encyclopedia-canonical-surface-2026-10-08T18-47-57-104Z-89270/`
- canonical summary SHA-256: `2f55dc0adba859cc1f48294f83d74a2bb9008625f3d983faee285cacba8b1307`
- production run: `.artifacts/interface-parity/encyclopedia-publication-2026-10-08T18-49-34-613Z-92002/`
- production summary SHA-256: `2c92f3900cf7cc8f9d52cd50bf43ea022031a92e5848274f5111faae6b53d1fd`
- fixture WASM SHA-256: `7dfbfcbbd98f13178f25458a3d0f05fdaf3a348df65ee85fd1842300baffd4c3`
- final production WASM SHA-256: `c307f0136c5b765d7d7b46abe0f8f01e63f7658a6b022aa99691d099ceb7be0b`
- packaged ZIP SHA-256: `54bc2f05baad2a3348178558a8d97f93992b61dcbeb9061f2d616324b5dad3b1`
- runtime pack SHA-256: `5c441dbf6d78a5a4afbc827b753b00090bdb8e0fda5cbf40daa5cbf41555e613`
- live-browser evidence: `/tmp/pr18-browser-evidence/`
- live-browser hash-manifest SHA-256: `80093b19b3a6da749f58e7a27ca617efac45a28d20589e0bb5c71c0175746b9e`
- visible catalog fingerprint: `20c342868cee50e80ef3b94b9f81a898c48f4b593ddd0d83ab69b67d755ae9ea`

Owned EData, generated runtime packs, screenshots, and browser profiles remain
ignored and uncommitted.
