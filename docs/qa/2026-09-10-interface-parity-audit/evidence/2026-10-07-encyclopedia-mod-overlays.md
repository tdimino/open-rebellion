---
title: "Native Encyclopedia content overlays"
description: "W7 deterministic presence-aware native overlays, atomic publication, rollback, and base-restoration evidence"
category: "qa"
created: 2026-10-07
updated: 2026-10-07
tags: [encyclopedia, modding, native, atomic, interface-parity]
---

# Native Encyclopedia content overlays

## Result

W7 passes its native functional gate. Root `encyclopedia.json` is now a
reserved presentation-content target outside the `GameWorld` patch map. Native
startup and explicit reload resolve enabled mods once in dependency-first
order, using a lexicographic ready tie-break, and pass that exact order to both
world and Encyclopedia consumers.

The W2 store validates the immutable base first, applies each ordered layer to
an off-side candidate, validates the complete effective session after every
layer, and publishes only the final snapshot. An acquisition, parse, artwork,
dependency, or effective-session error leaves the active `Arc` and
Encyclopedia texture generation unchanged. Every toggle/reload candidate is
rebuilt from the immutable base, so disabling all layers restores the exact
base fingerprint and resource set.

Browser builds remain base-only. No Encyclopedia overlay bytes, prose, image
bytes, or GPU state enter `GameWorld`, saves, replay, multiplayer, or simulation
fingerprints. No proprietary content or generated asset pack is tracked.

## Overlay contract

Each overlay is a bounded JSON array keyed by numeric P65/P66 object ID. The
parser distinguishes absent fields, explicit `null`, and replacement values and
rejects unknown fields, duplicate selectors, unsafe paths, malformed actions,
and oversized input.

The supported operations are:

- `patch` (default), where absent fields inherit and present fields replace;
- `replace`, requiring title, body, and image replacements;
- `add`, requiring a new supported non-system object ID, unique nonzero text
  resource ID, title, body, and image;
- `remove`, deleting exactly one whole topic and accepting no content fields.

Existing text-resource identities and category definitions are immutable. A
title cannot be removed. Explicit body/image removal is legal only when the
result satisfies the source-approved empty-topic contract. Later valid layers
win per field.

Author images must use a relative `encyclopedia/assets/*.bmp` identity. Native
acquisition rejects symlinked mod roots and every symlinked path component,
requires a regular file, and bounds the overlay to 16 MiB, patches to 10,000,
each image to 32 MiB, and retained artwork per mod to 128 MiB. The W2 validator
then requires the original 400-by-200 uncompressed 8-bit indexed BMP class.
Already-loaded author bytes receive a stable
`mod:v1:<mod-name>:<relative-path>` identity; they are not admitted to the
original EDATA or faithful-HD approval namespace.

## Test coverage

Synthetic tests cover:

- deterministic lexicographic ordering for unrelated mods in either discovery
  order and dependency-first precedence for related mods;
- exclusion of the reserved target from world patches and exact retention of
  missing/bytes/read-error states;
- partial inheritance, later-layer field precedence, whole replacement,
  permitted addition/removal, and exact base restoration;
- removal cleanup for unreferenced source/art records while retaining records
  still shared by another topic;
- legal explicit null restoration of a source-empty topic;
- immutable existing text-resource IDs and complete validation after each
  layer;
- missing artwork, invalid overlay candidates, exact active-`Arc` and texture
  generation rollback, and unchanged serialized `GameWorld` bytes;
- missing prefixes, empty/dot/oversized/invalid path segments, exact lowercase
  BMP extension, and 256/257-byte path boundaries;
- symlinked artwork and a symlinked text-only mod root.

The targeted mutation run initially exposed eleven unasserted path-validator
boundaries. After adding the explicit boundary matrix, the same 25-mutant run
finished with 24 caught, one compile-time-unviable mutant, zero missed, and zero
timeouts. A separate 18-mutant application-helper run then exposed source
cleanup and immutable text-identity gaps. After strengthening those regressions,
the fresh run finished with 17 caught, one compile-time-unviable mutant, zero
missed, and zero timeouts.

## Base-only browser regression

The current W7 fixture build retained the complete accepted E30/W6 canonical
surface matrix:

- 14 cases and 62 screenshots across both factions;
- 14 category cells and 16 exact owned-art comparisons;
- zero navigation requests and zero runtime errors;
- all 62 screenshot hashes identical to the accepted W6 baseline;
- CMD-02 catalog validation passed 152 executions;
- production fixture exclusion retained zero fixture tokens;
- browser and local server both closed.

Retained local ignored evidence:

- `.artifacts/interface-parity/encyclopedia-canonical-surface-2026-10-07T00-30-41-495Z-2814862/summary.json`
- summary SHA-256
  `b0e32c6d22904b7ac9a7e842f9a26de21a0b78188e358085ab2feb1da3dba07b`
- fixture WASM SHA-256
  `90d4821012705a68c74b1df2144281dc989d96df540b9c1d9f768166ffa1a1b2`
- unchanged ORPK SHA-256
  `e4c0157a8701aa1631f642db04c71b5e01d7280cf3e0cb691b428c502a1afa4e`
- Chrome `151.0.7922.34`

The first current-source browser attempt correctly caught a dropped
`disposition=Installed` startup-log field introduced while moving mod discovery
ahead of session installation. The observable logging contract was restored
from the actual base-install disposition; the rebuilt full matrix above then
passed. The production `web/open-rebellion.wasm` was not rebuilt or modified.

## Verification

```text
TDD red:
cargo test -p rebellion-data --lib mods::tests::reserved_encyclopedia_target_is_retained_outside_world_patches
FAIL: ModContent had no separately retained Encyclopedia target

cargo test -p rebellion-data --lib encyclopedia_overlay::tests
FAIL: presence-aware overlay parser and types did not exist

cargo test -p rebellion-data --lib encyclopedia_session::tests::overlays_inherit_absent_fields_and_later_layers_win_in_supplied_order
FAIL: ordered candidate-session installation did not exist

LIBRARY_PATH=<local ALSA linker shim> cargo test -p rebellion-app --bin open-rebellion encyclopedia_mods::tests
PASS; 4 tests

cargo test -p rebellion-data --lib
146 passed; 0 failed; 13 ignored

LIBRARY_PATH=<local ALSA linker shim> \
  cargo test -p rebellion-app --bin open-rebellion
84 passed; 0 failed; 3 ignored

LIBRARY_PATH=<local ALSA linker shim> cargo test --workspace --quiet
1,451 passed; 0 failed; 42 intentionally ignored

node --test tools/interface-parity/*.test.mjs
26 passed

cargo check -p rebellion-app --bin open-rebellion \
  --features interface-test-fixtures --target wasm32-unknown-unknown
PASS; browser remains base-only

cargo clippy -p rebellion-data --lib --no-deps -- -D warnings
PASS

LIBRARY_PATH=<local ALSA linker shim> \
  cargo clippy -p rebellion-app --bin open-rebellion --no-deps -- \
  -D warnings <named pre-existing allowances>
PASS

cargo mutants -p rebellion-data \
  --file crates/rebellion-data/src/encyclopedia_overlay.rs \
  --re 'parse_encyclopedia_overlay|apply_encyclopedia_overlay_layer|valid_mod_image_path' \
  -- --lib encyclopedia
25 total: 24 caught, 1 unviable, 0 missed, 0 timed out

cargo mutants -p rebellion-data \
  --file crates/rebellion-data/src/encyclopedia_overlay.rs \
  --re 'apply_topic_patch|require_value|remove_unreferenced_source_for_entry|artwork_keys|non_system_artwork_keys' \
  -- --lib encyclopedia
18 total: 17 caught, 1 unviable, 0 missed, 0 timed out

REBELLION_EDATA_DIR=<owned EData> \
  node tools/interface-parity/encyclopedia-canonical-surface.mjs
PASS; 14 cases, 62 screenshots, 16 exact artwork checks, zero navigation
requests, zero runtime errors, all screenshot hashes identical to W6
```

Workspace warnings are the existing missing lint-inheritance, tactical
macro-semicolon, and target-specific WASM unused-code warnings. No new scoped
Clippy finding remains.

## Deliberate boundary

W7 does not implement the optional W8 settled-write filesystem watcher. The
existing explicit Mod Manager reload action acquires and validates a complete
candidate; live burst-write, editor-rename, recovery-loop, and shutdown
behavior remain W8 work.

Native overlay semantics are an approved extension and do not contribute to
strict original-interface parity. Browser and strict evidence stay on the
unmodified canonical base profile. E32 retains command `0x131` production-route
activation, and lossless original Windows A0 comparison remains open.
