---
title: "Encyclopedia original-first HD selection"
description: "W6 deterministic native faithful-HD selection and original-parity fallback evidence"
category: "qa"
created: 2026-10-06
updated: 2026-10-06
tags: [encyclopedia, faithful-hd, native, interface-parity, acceptance]
---

# Encyclopedia original-first HD selection

## Result

W6 passes its current-schema functional gate. Native startup prepares one
immutable selection snapshot from the installed W2 session and the existing
faithful-HD manifest authority. The original validated artwork remains the
default and every rejected enhanced candidate falls back to the exact original
bytes. Browser builds remain original-only.

The selector performs no frame-time file access, hashing, or image decoding.
It admits an enhanced candidate only when all of the following agree:

- the explicit `faithful-hd` profile is active;
- the manifest entry is approved and has completed review metadata;
- the original filename, SHA-256, and 400 by 200 dimensions match the installed
  immutable session;
- the output SHA-256 matches the retained PNG bytes;
- manifest and decoded dimensions are exactly 1,600 by 800, the existing
  faithful-HD pipeline's required 4x scale.

The W4 adapter carries the selected digest, dimensions, bytes, and linear
sampling into the renderer without changing the W3 presentation. The selected
texture key includes session generation, resource identity, digest, filename,
and sampling policy. A profile-only nearest-to-linear change therefore releases
the prior handle instead of reporting a cache hit.

## Tracked acceptance matrix

Synthetic tests cover:

- original-parity with an unreadable sentinel path, proving the path is not
  consulted and the exact original view is retained;
- one reviewed, digest-valid, exact-4x PNG selected with linear sampling;
- approved but missing output;
- output digest mismatch;
- manifest/decoded size mismatch;
- exact `EDATA.NNN` identity boundaries;
- unchanged immutable session bytes after selection;
- W4 adapter propagation of selected identity and sampling;
- texture-cache invalidation when only the sampling profile changes.

All missing or invalid cases retain the complete original
`EncyclopediaArtworkView`; no partial enhanced state is published.

## Original-parity browser regression

The isolated post-W6 fixture WASM retained original-parity behavior across the
complete E30 canonical matrix:

- 14 cases and 62 screenshots across both factions;
- 14 category cells and 16 exact owned-art comparisons;
- zero navigation requests and zero runtime errors;
- all logical observations, journey results, and screenshot hashes identical
  to the accepted E30 baseline;
- CMD-02 catalog validation passed 152 executions;
- production fixture exclusion retained zero fixture tokens.

Retained local evidence:

- `.artifacts/interface-parity/encyclopedia-canonical-surface-2026-10-06T23-28-35-747Z-2690790/summary.json`
- summary SHA-256
  `32cb07bf32dc2bf6379d91c14b3b07e7fb82cefa195a140129194db09fbff5a4`
- fixture WASM SHA-256
  `ea1fed37f380ca44ca1e0ebd5ae70598c275c607001d26f842a57fc57d735fad`
- unchanged ORPK SHA-256
  `e4c0157a8701aa1631f642db04c71b5e01d7280cf3e0cb691b428c502a1afa4e`
- Chrome `153.0.8010.12`

Generated WASM, screenshots, runtime logs, owned original bytes, and synthetic
generated PNGs remain ignored or test-temporary and are not committed.

## Verification

```text
TDD red:
LIBRARY_PATH=<local ALSA linker shim> \
  cargo test -p rebellion-app \
  approved_exact_four_x_png_replaces_only_the_rendered_view \
  --bin open-rebellion
FAIL: prepared selector, HD adapter, and linear sampling did not exist

LIBRARY_PATH=<local ALSA linker shim> \
  cargo test -p rebellion-app encyclopedia_hd::tests --bin open-rebellion
PASS

LIBRARY_PATH=<local ALSA linker shim> \
  cargo test -p rebellion-render --lib
550 passed; 0 failed; 3 ignored

LIBRARY_PATH=<local ALSA linker shim> \
  cargo test -p rebellion-app --bin open-rebellion
80 passed; 0 failed; 3 ignored

LIBRARY_PATH=<local ALSA linker shim> cargo test --workspace --quiet
1,436 passed; 0 failed; 42 intentionally ignored

node --test tools/interface-parity/*.test.mjs
26 passed

cargo check -p rebellion-app --bin open-rebellion \
  --features interface-test-fixtures --target wasm32-unknown-unknown
PASS; browser target remains original-only

cargo clippy -p rebellion-app --bin open-rebellion --no-deps -- \
  -D warnings <named pre-existing allowances>
PASS

cargo clippy -p rebellion-render --lib --no-deps -- \
  -D warnings <named pre-existing allowances>
PASS

cargo mutants -p rebellion-app \
  --file crates/rebellion-app/src/encyclopedia_hd.rs \
  --re 'prepare_native_encyclopedia_hd|PreparedEncyclopediaHd::select|edata_resource_id' \
  -- --bin open-rebellion encyclopedia_hd
11 total: 10 caught, 1 unviable, 0 missed

REBELLION_EDATA_DIR=<owned EData> \
  node tools/interface-parity/encyclopedia-canonical-surface.mjs
PASS; all accepted E30 states and screenshot hashes identical
```

## Deliberate boundary

No separately owned, manifest-approved EDATA HD pack is installed in the
workspace, and W6 does not generate or commit one. The ignored
`owned_approved_hd_pack_selects_only_validated_linear_artwork` test requires
`REBELLION_HD_ROOT` and will exercise a real approved pack when one exists.
This absence does not weaken original fallback or selection-policy tests, but
it means this checkpoint does not claim a live enhanced-art screenshot.

W7 retains explicit native overlay provenance and precedence. E32 retains
production command routing. Strict original Windows A0 comparison remains a
separate acceptance gate and continues to use original-parity captures only.
