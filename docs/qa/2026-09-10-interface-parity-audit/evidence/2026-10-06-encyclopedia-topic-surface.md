# Encyclopedia authentic topic-surface checkpoint

Date: 2026-10-06
Plan: `docs/plans/2026-10-02-feat-will-forster-encyclopedia-handoff-adaptation.md`, W4
Bead: `orlocal-818.61`
Pre-W4 branch checkpoint: `72604ee14280df81d1be2b61582d10c026330032`

## Scope

W4 connects the W3 presenter to a renderer-owned, graphics-neutral surface DTO
and the existing source-recovered Encyclopedia window. The application adapter
is mechanical: it borrows the immutable W2 session's prose and exact validated
art bytes without copying them, maps audience/mode/category/navigation fields,
and preserves W3's immutable return route through each follow-up transition.
The renderer still has no dependency on `rebellion-data`, and neither layer
performs catalog or filesystem I/O while drawing.

The surface reuses STRATEGY 10335/10336 faction shells, 10337 topic overlay,
10338 index panel, faction rails 10585/10589, all seven recovered category
controls, topic/index rail controls, and previous/next normal, pressed, and
disabled resources. The original 400-by-200 topic art is drawn at `(12,31)`;
the bounded body aperture is `(17,231,395,80)`. Index selection, nine-row page
movement, category traversal, bounded previous/next, body scrolling, mode
switching, close, transparent hit rejection, and
captured press/release behavior all emit typed actions back to the application.

This is still a production-dormant rollback point. The new adapter and fixture
controller compile only for tests or `interface-test-fixtures`; command `0x131`
remains fail-closed. W5 owns canonical native/ORPK readers, identical logical
fingerprints across them, packaged journeys, and production route activation.

## Texture ownership

`EncyclopediaTopicTextureCache` retains at most one selected-topic handle. Its
key contains the W2 texture generation, resource identity, filename, and digest.
An unchanged key is a cache hit; a topic/resource/generation change, index or
non-topic frame, decode error, dimension mismatch, explicit clear, or
cache drop releases the old handle. Decoding occurs only on a key change, the
decoded dimensions must equal the W2 metadata, and upload uses nearest sampling.
The session gate now prevents an incomplete visible endpoint from reaching the
surface at all.

## Synthetic evidence boundary

The checked-in W4 fixture is redistributable synthetic P66A-shaped data. It
covers short prose, long wrapped prose, an explicit newline, a long token, four
ordered entries, both factions, and both navigation endpoints. Its historical
source SHA-256 is
`3962987aff1ea257ae1b3915dd80773355faa9a888eff4ed9f15ba2bfba6715a`.
It is not original prose, an A0 capture, or proof of the W5 canonical packaged
reader. The displayed synthetic bitmaps are generated only inside the
fixture-gated application module; authentic STRATEGY chrome comes from the
ignored owned runtime staging area and is not committed.

## Browser A1 gate

`tools/interface-parity/encyclopedia-surface.mjs` ran ten fresh muted Chromium
cases: four starts for each faction at 640 by 480, plus the middle topic for
each faction at 800 by 600. Each case requested exactly `/`, `gl.js`,
`open-rebellion-test.wasm`, and `data/runtime.orpk`, all HTTP 200, with no failed
request, page error, or console error. The run exercised stable screenshots,
long-body scrolling, hover invariance, captured pressed art, release-to-next,
disabled endpoints, index mode, category/list keyboard movement, Enter,
Escape/close, and viewport scaling.

Final run:

```text
.artifacts/interface-parity/encyclopedia-surface-2026-10-06T21-14-47-568Z-2506001
status: pass; cases: 10; browser: Chrome 153.0.8010.12
fixture WASM SHA-256: c56ae2089c9bbc7d1da50338a99536b7f02bd2259d9bc113717c4e63a0b0b1ec
runtime pack SHA-256: a0e6a877ad63c47a9d6977795220ec4189349c26a81ed3a2698f230d6e7641d3
summary SHA-256: 34dfae85c93cadcc7b34870ad8c3eb94c17fb4710dfb025282abf2af6ebbeb1b
```

At native size, every resolved frame contains all 80,000 expected solid
synthetic art pixels. Both disabled-control comparisons differ from their exact
STRATEGY resource by zero pixels, and both unavailable-topic cases contain zero
pixels of the prior middle-topic color. The scaled cases retain 124,251 of
125,000 expected solid pixels; the sub-one-percent edge difference is the
browser's scaled raster boundary, not stretched fixed-control geometry.

Visual inspection covered Alliance index and topic transitions and the
Imperial 800-by-600 scaled topic. The known
`topic_title_arrow_overlap` remains visible. Its exact title/font placement is
not changed speculatively and remains part of the strict original A0 gate.

## Defects found during verification

The first browser interaction used egui's short `clicked()` state. The harness
holds the button while capturing pressed art, so releasing later failed to open
the next topic. The fixed renderer owns one captured command ID: an opaque
source-alpha press captures it, the pressed bitmap remains while held, release
over the same control emits the action, and any primary release clears capture.
The final browser matrix passes this regression.

Final review also found that body scroll could carry into another long topic
and that the surface read arrow keys even when an unrelated widget owned egui
focus. View state now resets scroll on generation/topic identity changes,
requests the index-list or topic-body focus only on mode transition or a press
inside the surface, consumes source keys only while that child owns focus, and
leaves unrelated focused widgets' input untouched.

Mutation testing also found two real missing boundaries. An intermediate
category Left-key equality mutation survived until a direct `0x73 -> 0x72`
assertion was added. A decoded-height-only metadata mismatch survived until a
pre-upload `400x200` versus validated `400x199` rejection test was added.

## Verification

```text
cargo test -p rebellion-render encyclopedia --lib
26 passed; 0 failed

cargo test -p rebellion-app encyclopedia --bin open-rebellion
4 passed; 0 failed

LIBRARY_PATH="$PWD/.artifacts/native-link" cargo test --workspace
1,420 passed; 0 failed; 39 ignored (owned-asset tests)

cargo clippy -p rebellion-render --lib --no-deps -- -D warnings \
  -A clippy::unusual-byte-groupings -A clippy::chunks-exact-to-as-chunks \
  -A clippy::too-many-arguments -A clippy::map-or-identity
PASS; allowances name pre-existing unrelated render findings

cargo clippy -p rebellion-app --bin open-rebellion --no-deps \
  --features interface-test-fixtures --target wasm32-unknown-unknown -- \
  -D warnings -A dead-code -A unused-imports -A unused-variables \
  -A clippy::too-many-arguments -A clippy::type-complexity \
  -A clippy::needless-borrow
PASS; allowances name pre-existing target-specific findings

cargo build --target wasm32-unknown-unknown -p rebellion-app --release \
  --features interface-test-fixtures
node tools/interface-parity/prepare-site.mjs
node tools/interface-parity/validate-catalog.mjs
node tools/interface-parity/verify-production-exclusion.mjs
PASS; CMD-02 38 scenarios / 152 executions; production fixture tokens 0;
fixture bridge present. The complete build script also passed before the final
focus-only correction; the listed commands rebuilt and validated the exact
final fixture artifact without regenerating the production WASM.

node tools/interface-parity/encyclopedia-surface.mjs
PASS; 10 cases; 40/40 expected requests HTTP 200; zero runtime errors

cargo mutants --file crates/rebellion-render/src/encyclopedia_surface.rs \
  --output /data/tmp/or-w4-surface-mutants-final -- --lib encyclopedia
15 mutants: 13 caught, 2 unviable, 0 missed

cargo mutants --file crates/rebellion-render/src/encyclopedia_textures.rs \
  --output /data/tmp/or-w4-texture-mutants-final -- --lib encyclopedia
10 mutants: 6 caught, 4 unviable, 0 missed

rustfmt --edition 2021 --check \
  crates/rebellion-app/src/encyclopedia_surface.rs \
  crates/rebellion-render/src/encyclopedia_surface.rs \
  crates/rebellion-render/src/encyclopedia_textures.rs \
  crates/rebellion-data/src/encyclopedia_catalog.rs
PASS; repository-wide formatting retains unrelated baseline deltas

git diff --check
PASS
```

Native tests and mutation runs used the ignored local ALSA linker-name shim.
The production build deliberately staged zero Encyclopedia assets because no
production EData root was supplied; that preserves the fail-closed W4 boundary.
The test-feature artifact alone contains the synthetic fixture controller.

## Remaining work

W5 must read the same canonical bytes through native and packaged browser
readers, prove matching logical fingerprints, rerun these journeys on the
release artifact, connect cockpit/contextual routes, and verify exact return
states before enabling command `0x131`. Lossless original Windows captures,
font/title placement (including the arrow overlap), and strict `OBJ-01` visual
acceptance remain open.
