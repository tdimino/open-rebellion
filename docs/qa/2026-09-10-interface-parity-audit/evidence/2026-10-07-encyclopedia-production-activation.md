---
title: "Encyclopedia production activation"
description: "E32 strict package policy, production cockpit/contextual routes, exact returns, and old-pack compatibility"
category: "qa"
created: 2026-10-07
updated: 2026-10-07
tags: [encyclopedia, production, routing, packaging, native, wasm]
---

# Encyclopedia production activation

## Result

E32 passes its bounded production-activation gate. New browser builds now
require the complete canonical Encyclopedia source, manifest, and referenced
EDATA set before compilation and pass `--require-encyclopedia` to both pack
construction and read-back verification. A corrupt, partial, or absent
namespace therefore cannot produce a new route-enabled package. An older
user-supplied ORPK with the whole namespace absent still boots the unrelated
campaign and reports command `0x131` unavailable.

Native and browser production paths now use the same W2 session, W3 presenter,
and W4 renderer as the accepted fixture route. F7 and cockpit command `0x131`
open the faction-matched index. Real object popup command `0x100` resolves a
stable source compound identity and opens the matching topic. Internal
category/topic transitions retain one immutable return origin; Escape returns
to the command center or the exact retained contextual window.

The surface is a foreground modal over retained command-center windows. A
native acceptance run caught both the earlier middle-layer stacking and the
live world's raw capital-class ID (`0x45`) before source-family reconstruction.
The final route resolves the displayed Corellian Corvette to source identity
`0x14000045`, paints its topic over the Fleet window, navigates to its adjacent
topic, and returns to that Fleet window.

No Encyclopedia bytes enter `GameWorld`, save bodies, replay, multiplayer, or
simulation fingerprints. Fixture selectors remain absent from production.
Generated packages, owned source data, and screenshots remain ignored.

## Route and lifecycle contract

- Cockpit entry uses the current player faction and preserves a typed
  `Cockpit` return route.
- Object-popup entry maps character, troop, special-force, capital-class, and
  system selections to canonical compound catalog IDs. A raw live ID is
  rejoined only within its source family and, where available, exact source
  name; ambiguous or unsupported fleet/quadrant selections fail to the
  presenter's explicit unresolved index instead of guessing.
- Mission-dialog command `0x67` preserves its recovered caller identity but
  remains an explicit unresolved-context index entry because the selected
  mission-kind-to-object join is not source-proven.
- New campaign and save load close any open surface. While open, the surface
  blocks underlying strategic/cockpit input.
- A native overlay generation change refreshes HD selection and rebinds the
  controller to the new immutable session before presentation. Invalid overlay
  candidates retain the last-known-good session under W7.

## Production package and browser evidence

The final `package-web.sh dev` output was exercised directly from
`dist/open-rebellion-web-dev`, without a fixture query, bridge, or inspector
request. Both factions completed:

1. normal new-campaign start;
2. F7 index, Enter topic, adjacent-topic navigation, and exact cockpit return;
3. re-entry after return;
4. F3 Fleet Finder, real Fleet window, real ship popup, matching contextual
   topic, adjacent-topic navigation, and exact contextual return.

Each fresh run made exactly four successful requests: `/`, `/gl.js`,
`/open-rebellion.wasm`, and `/data/runtime.orpk`. The separate old-pack run
removed the whole Encyclopedia namespace in transit, retained the same
four-request startup, booted a campaign, and left F7 unavailable without a
runtime error.

Final artifact identity:

- production WASM SHA-256:
  `6213bcf860e658893f1adb2f85b1e65b2c52ae32c4d04e2eea1e7280d6f93ff8`;
- runtime ORPK SHA-256:
  `e4c0157a8701aa1631f642db04c71b5e01d7280cf3e0cb691b428c502a1afa4e`;
- runtime ORPK size: 50,846,574 bytes;
- source catalog SHA-256:
  `354643f3a5cb58c7bfa92e094d687ba3188c037eac14a961ea843f6b50566994`;
- source manifest SHA-256:
  `238b8879565ab2603594a8705f3538c652da3541b147457be09ec1f3ef25e8b1`;
- namespace: two metadata entries and 186 exact referenced artwork entries;
- Chrome: `153.0.8010.12`.

Retained local ignored browser summary:

`.artifacts/interface-parity/encyclopedia-publication-2026-10-07T02-04-31-264Z-3013352/summary.json`

## Native and save/load evidence

Normal native launches without fixture or bridge input completed the cockpit
index/topic/next/return journey for Alliance and Empire. A further Alliance
journey used the real Fleet Finder and object popup and visibly proved the
foreground contextual topic and exact Fleet-window return. Its four screenshot
hashes are:

- object menu: `cc42e65a41ddbb51b28b64bef820c46ce14f4647f1326a266c7587a1c4d8193e`;
- Corellian Corvette topic:
  `bd441212b4530099fae020d503b3c07f525fef0cf3615781c3443923cfdc6684`;
- adjacent topic:
  `1e6354b620d727b664bc2fb3221dbe6d17b16c7f509d82b5c8c5a34b8aecb156`;
- contextual return:
  `927ef59abf5152e81ac77e0550477ae5ff65c40cf94daf35db8c5834b8a88b96`.

Retained local ignored native evidence:

- `.artifacts/interface-parity/e32-native-alliance/`;
- `.artifacts/interface-parity/e32-native-empire/`;
- `.artifacts/interface-parity/e32-native-contextual-alliance-pass/`.

The VPS has no usable ALSA device. Quad-snd worker threads report that host
limitation, while the application UI remains live and completes each route.
An isolated temporary `libasound.so` linker shim was required for native Rust
test/build linking; no repository or system library was changed.

The normal packaged Game Options smoke passes current-version save, overwrite
cancel, matching load and restored Galaxy fingerprints, slot-ten load/delete,
delete cancellation and acceptance, corrupt-slot handling, browser restart,
and zero page errors for both factions. Retained ignored final-package results
are under `.artifacts/interface-parity/e32-game-options-final/`.

## Verification

```text
python3 -m unittest discover -s scripts -p test_build_runtime_pack.py
10 passed

python3 -m unittest discover -s scripts -p test_encyclopedia_staging.py
2 passed

LIBRARY_PATH=<isolated ALSA linker shim> \
  cargo test -p rebellion-app encyclopedia -- --nocapture
29 passed; 0 failed; 3 owned-source tests ignored

cargo test -p rebellion-data encyclopedia -- --nocapture
44 passed; 0 failed

LIBRARY_PATH=<isolated ALSA linker shim> cargo test --workspace --quiet
1,457 passed; 0 failed; 39 intentionally ignored

node --test tools/interface-parity/*.test.mjs
26 passed

node tools/interface-parity/verify-production-exclusion.mjs
production_fixture_tokens=0; test_fixture_bridge=true

cargo check -p rebellion-app --bin open-rebellion \
  --target wasm32-unknown-unknown
PASS

cargo clippy -p rebellion-data --lib --no-deps -- -D warnings
PASS

LIBRARY_PATH=<isolated ALSA linker shim> \
  cargo clippy -p rebellion-app --bin open-rebellion --no-deps -- \
  -D warnings <named pre-existing allowances>
PASS

LIBRARY_PATH=<isolated ALSA linker shim> cargo mutants -p rebellion-app \
  --file crates/rebellion-app/src/encyclopedia_surface.rs \
  --re 'EncyclopediaSurfaceController::(is_open|open|close|presentation|apply_action)' \
  -- --bin open-rebellion encyclopedia_surface::tests
8 total: 6 caught, 2 unviable, 0 missed

LIBRARY_PATH=<isolated ALSA linker shim> cargo mutants -p rebellion-app \
  --file crates/rebellion-app/src/main.rs --re 'catalog_object_id' -- \
  --bin open-rebellion encyclopedia_route_tests
12/12 catalog-object resolver mutants caught. Cargo-mutants 27.1 also included
5 unrelated struct-field mutants despite the inclusion regex; those existing
window/draw fields were outside this scoped assertion and survived.

REBELLION_EDATA_DIR=<owned EData> bash scripts/build-wasm.sh
PASS; strict canonical namespace and read-back verification

REBELLION_EDATA_DIR=<owned EData> bash scripts/package-web.sh dev
PASS; final package created

REBELLION_EDATA_DIR=<owned EData> \
OPEN_REBELLION_ENCYCLOPEDIA_SITE=dist/open-rebellion-web-dev \
  node tools/interface-parity/encyclopedia-publication.mjs
PASS; both factions, cockpit and contextual journeys, exact returns,
four requests, old-pack unavailable compatibility, zero runtime errors
```

The scoped mutation run initially found the session-generation comparison was
not asserted. A replacement-session regression now proves the controller
observes the new generation before presentation; the fresh run has zero missed
mutants. Final route-helper mutation totals are recorded after their focused
run above; every resolver mutant was caught.

Workspace warnings are the existing missing lint-inheritance,
macro-semicolon, target-specific unused-code, absent optional audio-resource,
and missing `wasm-opt` warnings.

## Deliberate boundary

This is implementation and A1 route evidence, not strict original-interface
acceptance. Lossless original Windows A0 comparison and the complete `OBJ-01`
faction/viewport matrix remain E35 work. W8 coalesced filesystem watching
also remains optional; explicit Mod Manager reload and atomic last-known-good
recovery are the supported native path in this increment.
