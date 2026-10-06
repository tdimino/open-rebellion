---
title: "Encyclopedia canonical surface acceptance"
description: "E30 native-session and packaged-browser acceptance for the canonical base-profile Encyclopedia"
category: "qa"
created: 2026-10-06
updated: 2026-10-06
tags: [encyclopedia, interface-parity, wasm, native, acceptance]
---

# Encyclopedia canonical surface acceptance

## Result

E30 passes for the canonical base profile defined by the approved 2026-10-02
handoff-adaptation plan. The native reader and the packaged-browser reader
produce the same immutable logical catalog, and the packaged browser renders
and operates that catalog through the authentic W4 surface for both factions.

This gate does not activate command `0x131`; E32 retains production-route
activation. It also does not claim strict original Windows A0 comparison, W6
HD selection, or W7 native mod/null/language-overlay precedence.

## Acceptance matrix

The retained browser run contains 14 independent cases and 62 screenshots:

- both factions at 640 by 480 for canonical index, first topic, last topic,
  longest resolved topic, source-unavailable topic, and contextual topic;
- both factions at 800 by 600 for the longest resolved topic;
- all seven recovered category commands (`0x6f` through `0x75`) for each
  faction, for 14 category cells total;
- source-order Home/End selection, Enter topic opening, topic-to-index return,
  faction-specific close control, and return to the cockpit;
- strict non-wrapping first/last navigation and exact disabled-control chrome;
- a 1,015-byte longest body with a visible PageDown delta and deterministic
  scroll reset after adjacent-topic navigation;
- a source-unavailable object with no body or artwork, followed by a resolved
  adjacent topic and an exact pixel-identical return to the empty topic;
- contextual entry with the requested object and stable caller token retained
  through adjacent/back navigation and the authentic faction close control;
- native-size artwork comparison against owned EData bytes for 16 topic
  renders, with zero differing pixels in every comparison.

Every fresh page loaded exactly four successful resources: `/`, `/gl.js`,
`/open-rebellion-test.wasm`, and `/data/runtime.orpk`. Every in-view journey
retained a navigation request delta of zero. All page, request, and console
error lists were empty. The browser and local server were both closed by the
harness.

The canonical category counts were stable for both factions:

| Command | Label resource | Topics |
|---:|---:|---:|
| `0x6f` | `0x1850` | 356 |
| `0x70` | `0x1855` | 200 |
| `0x71` | `0x1854` | 38 |
| `0x72` | `0x1852` | 14 |
| `0x73` | `0x1851` | 25 |
| `0x74` | `0x1856` | 10 |
| `0x75` | `0x1853` | 69 |

The all-category view contains 346 resolved topics and the ten exact
source-empty topics. The browser observation channel records only stable IDs,
counts, byte lengths, resource filenames, dimensions, and digests; it does not
export source prose or topic names into tracked fixtures or evidence.

## Native and packaged identity

The ignored owned native test installed the canonical native bytes through the
W2 session boundary and exercised both audiences, every category, first/last
endpoints, the dynamically selected longest resolved topic, the first
source-unavailable topic, and a contextual topic with its exact return token.
Each resolved sample retained validated 400 by 200 artwork metadata and a
64-character SHA-256 identity.

| Identity | Value |
|---|---|
| P66A catalog SHA-256 | `354643f3a5cb58c7bfa92e094d687ba3188c037eac14a961ea843f6b50566994` |
| Logical catalog fingerprint | `5c4b64bfd739508e63a87118fd7cac8503ea2d34999144838074a52736b00fe3` |
| ORPK SHA-256 | `e4c0157a8701aa1631f642db04c71b5e01d7280cf3e0cb691b428c502a1afa4e` |
| Final fixture WASM SHA-256 | `d623d8813b28b88816e23319f17f56c39c6158af8a26183119662114433fbc2b` |

Canonical fixture starts are derived from the installed session, not hard-coded
owned identities. The fixture-only observation bridge emits a report only when
logical state changes, leaving the production WASM and HTML free of fixture
tokens.

## Retained local evidence

- `.artifacts/interface-parity/encyclopedia-canonical-surface-2026-10-06T22-57-32-777Z-2663369/summary.json`
- summary SHA-256
  `58fc285de2481de36b0f16c0980e5624e6ddb20f12fca35be15314caa4111d42`
- Chrome `153.0.8010.12`
- 62 ignored PNG captures beneath the same run directory
- W4 regression run:
  `.artifacts/interface-parity/encyclopedia-surface-2026-10-06T23-00-37-820Z-2664730/summary.json`
  with SHA-256
  `bb2a08e9b6d6486b6be687335501ad23ee07ede44c315d6e8f981e9322ae58cf`

Owned source files, original artwork, generated ORPK/WASM artifacts, browser
screenshots, and complete runtime logs remain ignored and uncommitted.

## Verification

```text
TDD red:
cargo test -p rebellion-app \
  canonical_fixture_starts_are_derived_from_the_installed_session \
  --bin open-rebellion
FAIL: canonical from_session/start API absent

cargo test -p rebellion-app \
  canonical_observation_reports_identity_without_exporting_owned_prose \
  --bin open-rebellion
FAIL: canonical observation API absent

cargo test -p rebellion-app \
  canonical_fixture_starts_are_derived_from_the_installed_session \
  --bin open-rebellion
FAIL: contextual canonical fixture start absent

LIBRARY_PATH=<local ALSA linker shim> \
  cargo test -p rebellion-app --bin open-rebellion
76 passed; 0 failed; 2 owned-data tests ignored

LIBRARY_PATH=<local ALSA linker shim> \
REBELLION_ENCYCLOPEDIA_TEST_SOURCE=<ignored P66A source> \
REBELLION_EDATA_DIR=<owned EData> \
  cargo test -p rebellion-app \
  owned_native_session_covers_the_canonical_e30_state_matrix \
  --bin open-rebellion -- --ignored --nocapture
1 passed

cargo mutants -p rebellion-app \
  --file crates/rebellion-app/src/encyclopedia_surface.rs \
  --re 'EncyclopediaSurfaceFixture::(from_session|observation)|longest_resolved_object_id|retain_longest' \
  -- --bin open-rebellion encyclopedia_surface
16 total: 14 caught, 2 unviable, 0 missed

cargo clippy -p rebellion-app --bin open-rebellion --no-deps \
  --features interface-test-fixtures --target wasm32-unknown-unknown -- \
  -D warnings -A dead-code -A unused-imports -A unused-variables \
  -A clippy::too-many-arguments -A clippy::type-complexity \
  -A clippy::needless-borrow
PASS; allowances name pre-existing target-specific findings

node --test tools/interface-parity/*.test.mjs
26 passed

LIBRARY_PATH=<local ALSA linker shim> cargo test --workspace --quiet
1,431 passed; 0 failed; 41 intentionally ignored

REBELLION_EDATA_DIR=<owned EData> bash scripts/build-interface-test-wasm.sh
PASS; canonical ORPK reproduced, CMD-02 validator passed, production fixture
exclusion passed, isolated final fixture site prepared

REBELLION_EDATA_DIR=<owned EData> \
  node tools/interface-parity/encyclopedia-canonical-surface.mjs
PASS; 14 cases, 14 category cells, 62 screenshots, 16 exact artwork checks,
zero navigation requests, zero runtime errors, contextual return retained,
browser/server closed

node tools/interface-parity/encyclopedia-surface.mjs
PASS; 10 unchanged W4 synthetic regression cases

node --check tools/interface-parity/encyclopedia-canonical-surface.mjs
rustfmt --edition 2021 --check crates/rebellion-app/src/encyclopedia_surface.rs
git diff --check
PASS
```

The focused UBS Rust scan completed without a reported finding. Its JavaScript
phase stopped producing output and was terminated after a bounded wait, so no
completed UBS result is claimed for the harness; Node syntax/unit checks,
manual review, and both live browser runs are the retained JavaScript evidence.

## Remaining gates

Strict original-game A0 comparison remains open and is not reclassified as
functional acceptance. W6 owns approved HD/base selection, W7 owns native
overlay and fallback precedence, and E32 owns production route activation.
