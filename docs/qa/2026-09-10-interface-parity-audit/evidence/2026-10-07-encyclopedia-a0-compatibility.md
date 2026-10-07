---
title: "Encyclopedia original-executable compatibility checkpoint"
description: "E35 production-browser comparison against lossless 640x480 original-executable captures under Wine"
category: "qa"
created: 2026-10-07
updated: 2026-10-07
tags: [encyclopedia, interface-parity, original-executable, wine, wasm, acceptance]
---

# Encyclopedia original-executable compatibility checkpoint

Date: 2026-10-07

Bead: `orlocal-818.35`

Candidate branch: `test/encyclopedia-conformance`

## Result

The adapted E35 production comparison passes for all twelve `OBJ-01` states
that have an honest original-executable baseline. The thirteenth state, the
approved plan's explicit source-unavailable presentation, has no corresponding
topic in the accepted original profile and remains functional A1 evidence
only. No original fixture was fabricated.

This is a compatibility checkpoint, not strict Windows parity. The original
executable was captured losslessly at 640 by 480 under the repository's Linux
Wine environment. The workspace policy states that Wine observations do not by
themselves establish Windows parity or finish project acceptance. Consequently
the canonical ledger cells remain pending even though the adapted comparison
tool reports twelve passes and one honest not-applicable state.

The normal-route production Encyclopedia is nevertheless working on the final
candidate: both factions traverse every category, open and navigate topics,
exercise the first and last endpoints, close to the cockpit, enter from a real
object popup, navigate contextually, and return to the exact retained caller.
The old-pack diagnostic still fails closed. Every fresh browser journey loads
exactly four resources and performs no navigation fetch.

## Defects found and corrected

The first comparison run failed and exposed production defects rather than
being waived:

- the surface used one approximate origin for both factions; production now
  uses original native origins `(62, 50)` for Alliance and `(125, 52)` for
  Empire, with the same scale transform on native and browser builds;
- topic titles began at the left edge and collided with the Back control; index
  and topic headings now use the recovered center line at native X `211`;
- the old WASM path could not satisfy runtime filesystem font reads and silently
  used egui's fallback; Liberation Sans regular and bold are now embedded so
  native and WASM use the same Arial-compatible metrics, with bold used for
  original heading/category roles;
- an unselected index silently borrowed the first topic as a current selection;
  an empty source state now remains empty until the user selects a row;
- retained caller windows could paint above the Encyclopedia; its modal layer
  now remains above the Fleet window and other callers;
- category hover now draws the original-style pale tooltip at the recovered
  cursor offset `(-2, +16)`.

Unit tests fix the origin, title span, font families, empty-selection behavior,
modal order, and tooltip offset. The production journey intentionally selects a
row with `Home` before `Enter`; it no longer depends on an invented initial
selection.

## Adapted A0 matrix

The ignored E51 crosswalk contains thirteen stable cell IDs. Its twelve
applicable cells point to SHA-256-verified PNGs captured from the actual
original executable. Index and all six filtered categories have both faction
baselines; topic, endpoints, and contextual entry use the retained bounded
captures available for those states.

| Cells | Requirement | Adapted result |
|---|---|---|
| `OBJ-01-C001` | index, both factions | pass |
| `OBJ-01-C002`–`C007` | systems, ships, facilities, missions, troops, personnel; both factions | pass |
| `OBJ-01-C008`–`C009` | topic text and exact 400-by-200 topic art; both factions | pass |
| `OBJ-01-C010` | disabled previous endpoint | pass |
| `OBJ-01-C011` | disabled next endpoint | pass within documented Wine repaint boundary |
| `OBJ-01-C012` | real contextual open | pass for caller-independent opaque rail |
| `OBJ-01-C013` | approved source-unavailable state | A0 not applicable; functional evidence only |

The tool crops each faction's exact 470-by-330 surface, validates the 640-by-480
source geometry and capture hashes, and writes original, production, and diff
PNGs to the ignored publication run. Region thresholds are explicit in source
and unit-tested. Representative results include:

- full-index difference: 4.38% Alliance and 4.58% Empire, maximum 12%;
- index-list difference: 9.85% for both faction index views, maximum 24%;
- topic-art difference: exactly 0 pixels for both factions;
- full-topic difference: 2.95% Alliance and 3.03% Empire, maximum 5%;
- topic-body difference: 12.01%, maximum 15%;
- all tested opaque rail regions remain below their 1% maximum.

The relatively broad text/list bounds admit the expected DirectWrite/Wine,
Chromium, and Liberation Sans rasterization differences; they do not replace
the zero-difference artwork or low-difference source-bitmap chrome checks.

## Production artifact and browser evidence

Final local package:

`dist/open-rebellion-web-e35-a0-final2.zip`

| Identity | SHA-256 or value |
|---|---|
| production WASM | `30c6c52fae9a43fc42dd5748a88ccbb7c45eb4d61c81991d3a09de56f34b82e2` |
| runtime ORPK | `e4c0157a8701aa1631f642db04c71b5e01d7280cf3e0cb691b428c502a1afa4e` |
| final local ZIP | `87bcabadf4a0035b844d5b1921c6f26c6b921894fc36363d5a930af5eb5c95ca` |
| canonical catalog | `354643f3a5cb58c7bfa92e094d687ba3188c037eac14a961ea843f6b50566994` |
| canonical manifest | `238b8879565ab2603594a8705f3538c652da3541b147457be09ec1f3ef25e8b1` |
| runtime ORPK size | 50,846,574 bytes |

Retained ignored production run:

`.artifacts/interface-parity/encyclopedia-publication-2026-10-07T15-04-53-054Z-3735917/summary.json`

Summary SHA-256:
`de5bc8fcdc1e980ab2e5da23f6a286655a0be98369a0c76b3414f3d9d011a3da`.

Retained ignored comparison:

`.artifacts/interface-parity/encyclopedia-publication-2026-10-07T15-04-53-054Z-3735917/a0-comparison/summary.json`

Comparison summary SHA-256:
`cc099c024bc97f8f0487a0062e9c86d44077a9f9f6eac07e90c0ab8abe54485c`.

Chrome `153.0.8010.12` ran muted. Both faction journeys, all seven category
states, topic/next/last-endpoint navigation, cockpit return, contextual
open/next/exact return, and old-pack unavailable behavior passed. Each launch
requested only `/`, `/gl.js`, `/open-rebellion.wasm`, and
`/data/runtime.orpk`; every request returned 200 and no page, request, or
console error was recorded. The harness closed its browser and server.

Native normal-route journeys and native contextual foreground behavior were
already retained by E32, while E30 records exact selected EDATA identities,
dimensions, digests, and selected-only texture ownership. W6 and W7 separately
record native-only HD and mod behavior. E35 changes only shared renderer/app
geometry, fonts, selection, and ordering and passes the native Rust targets;
it does not reinterpret browser base-only evidence as native HD/mod evidence.

## Verification

```text
TDD red:
production_surface_uses_the_original_native_window_origin
FAIL: Alliance and Empire both used the approximate (85, 55) origin

topic_header_uses_the_original_centered_title_span
FAIL: topic title used left-aligned X 36

production_font_definitions_embed_metric_compatible_faces
FAIL: WASM could not load runtime filesystem font paths

production_surface_is_modal_above_retained_callers
FAIL: retained Fleet window could cover the Encyclopedia

category_tooltip_uses_the_original_cursor_offset
FAIL: tooltip behavior was absent

LIBRARY_PATH=<local ALSA linker shim> \
  cargo test -p rebellion-render encyclopedia --lib
31 passed

LIBRARY_PATH=<local ALSA linker shim> \
  cargo test -p rebellion-render theme::tests --lib
3 passed

LIBRARY_PATH=<local ALSA linker shim> \
  cargo test -p rebellion-app encyclopedia --bin open-rebellion
30 passed; 3 owned-source tests intentionally ignored

node --test tools/interface-parity/*.test.mjs
29 passed

cargo check --workspace --all-targets
PASS; established lint-inheritance, target dead-code, and macro warnings only

cargo clippy -p rebellion-render --lib --no-deps -- \
  -D warnings <named established render allowances>
cargo clippy -p rebellion-app --bin open-rebellion --no-deps -- \
  -D warnings <named established app allowances>
PASS

rustfmt --edition 2021 --check \
  crates/rebellion-app/src/encyclopedia_surface.rs \
  crates/rebellion-render/src/encyclopedia.rs \
  crates/rebellion-render/src/theme.rs
node --check tools/interface-parity/encyclopedia-publication.mjs
node --check tools/interface-parity/encyclopedia-a0-acceptance.mjs
git diff --check
PASS

cargo mutants \
  --file crates/rebellion-app/src/encyclopedia_surface.rs \
  --re encyclopedia_surface_origin \
  -- --bin open-rebellion encyclopedia_surface::tests
21 caught; 0 missed

cargo mutants \
  --file crates/rebellion-render/src/encyclopedia.rs \
  --re 'encyclopedia_surface_order|encyclopedia_category_tooltip_origin|encyclopedia_topic_title_layout' \
  -- --lib encyclopedia
7 caught; 3 unviable; 0 missed

cargo mutants \
  --file crates/rebellion-render/src/theme.rs \
  --re 'production_font_definitions|original_bold_font|load_fonts' \
  -- --lib theme::tests
3 caught; 0 missed

REBELLION_EDATA_DIR=<owned ignored EData> bash scripts/build-wasm.sh
REBELLION_EDATA_DIR=<owned ignored EData> \
  bash scripts/package-web.sh e35-a0-final2
PASS; final package identities above

REBELLION_EDATA_DIR=<owned ignored EData> \
OPEN_REBELLION_ENCYCLOPEDIA_SITE=dist/open-rebellion-web-e35-a0-final2 \
  node tools/interface-parity/encyclopedia-publication.mjs
PASS; both factions, every category, endpoints, exact returns, old pack,
four-request startup, zero runtime errors

OPEN_REBELLION_ENCYCLOPEDIA_A0_CROSSWALK=<ignored E51 crosswalk> \
OPEN_REBELLION_ENCYCLOPEDIA_A0_ROOT=<ignored original capture root> \
OPEN_REBELLION_ENCYCLOPEDIA_PUBLICATION_RUN=<final production run> \
  node tools/interface-parity/encyclopedia-a0-acceptance.mjs
PASS; 12 applicable cells pass, source-unavailable cell is honest N/A
```

The first origin mutation run left four centering operators alive because its
viewports had zero letterbox offsets; wide and tall cases were added and the
repeat caught all 21 mutants. The first tooltip run similarly exposed two
scale operators at scale 1; the scaled case then caught both. These are the
final no-missed results above.

## Remaining strict gate

The functional Encyclopedia and the adapted original-executable compatibility
matrix pass. Strict `OBJ-01` ledger acceptance remains open until the same
required visual states are captured on an original Windows runtime (or the
workspace policy is explicitly revised). The source-unavailable state remains
functional-only because no legitimate original topic supplies its A0.

No original PNG, owned EDATA, generated package, full runtime log, local path,
or proprietary source content is committed by this checkpoint.
