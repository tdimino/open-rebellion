---
title: "Will Forster Encyclopedia Handoff Adaptation"
description: "Execution plan for adapting the strongest test, runtime, presentation, publication, HD, and modding ideas from PR #16 into the canonical P66 Encyclopedia pipeline"
type: feat
status: complete
created: 2026-10-02
updated: 2026-10-07
attribution: "Will Forster, PR #16"
source_pr: "https://github.com/tdimino/open-rebellion/pull/16"
source_head: "82ef2ccc7f5b256470adaa3797898b8d79f2d7fc"
tags: [encyclopedia, P66, handoff, conformance, modding, native, wasm]
---

# Will Forster Encyclopedia Handoff Adaptation

## Attribution and decision

This plan adapts the strongest ideas from Will Forster's
[Encyclopedia implementation handoff](https://github.com/tdimino/open-rebellion/pull/16),
head commit `82ef2ccc7f5b256470adaa3797898b8d79f2d7fc`. Will supplied the
architecture, implementation experiments, tests, and unusually thorough handoff
notes that informed the workstreams below. Thank you to Will for making this
next phase easier to reason about and verify.

PR #16 is a selective handoff, not a wholesale merge candidate. It contains 51
dependency-linked commits based on the earlier P65 boundary, overlaps later
upstream work in 13 files, and has four direct content conflicts. Open Rebellion
will therefore port the valuable patterns into the current P66 contracts in
small, independently verified commits. This preserves Will's design intent
without introducing a second Encyclopedia schema, runtime, or source authority.

When implementation is directly derived from Will's code, the commit body must
link PR #16 and credit Will. When only an idea is adapted, the commit and evidence
record should identify the relevant PR commit without claiming code authorship.

## Outcome

Deliver the production Encyclopedia on native and browser builds using the
canonical P66A source extraction and the pending P66B packed-transport
workstream. The result must add strict conformance testing, immutable validated
content sessions, source-style topic presentation and navigation, recoverable
publication, original-first HD selection, and bounded native mod overlays.

The production result must:

- retain one source-derived catalog and resource namespace on both platforms;
- resolve 346 of 356 topics per faction and preserve the ten source-empty
  mission records without invented fallback content;
- use original artwork by default and keep enhanced artwork opt-in;
- leave Encyclopedia content outside `GameWorld`, saves, command replay, and
  simulation fingerprints;
- preserve the browser startup request budget and fail closed on malformed or
  incomplete content;
- open through authentic cockpit and contextual routes only after the full
  route, navigation, close, and return journeys pass;
- keep all proprietary source data and generated packs ignored.

## Canonical authority

This plan does not replace the
[Encyclopedia Data Extraction, Modding, and Display design](2026-09-27-design-encyclopedia-data-pipeline.md).
That document owns the source format, entity binding, packaging, modding, and
display contract. This file owns the order in which selected PR #16 ideas are
adapted into that contract.

At plan creation, P66A is landed. P66B is an isolated transport workstream
pending final integration. Nothing in this plan treats P66B as landed evidence
or enables a production route before that checkpoint passes.

| Concern | Canonical authority | Adaptation rule |
|---|---|---|
| Source extraction | P66A `source.json`, manifest, hashes, and source-empty ledger | Extend tests around it. Do not replace it. |
| Entity binding | P66A family-qualified typed join | Present its results. Do not add a parallel ID system. |
| Browser transport | Planned P66B ORPK namespaces and validation | Install an immutable session after the canonical transport checkpoint lands. |
| Production routing | Current cockpit, object, and command contracts | Enable only after topic and return-path acceptance. |
| Visual acceptance | Interface parity audit `OBJ-01` cells | Browser evidence cannot substitute for missing original A0 baselines. |
| Mods | Existing `mod.toml` loader and dependency order | Add a presentation-content target outside `GameWorld`. |
| HD assets | Existing faithful-HD profile and digest policy | Original assets remain the default and fallback. |

## Strongest ideas to retain

| Idea from Will's handoff | Upstream adaptation | Delivery phase |
|---|---|---|
| Synthetic shared conformance corpus | Generate legal and adversarial fixtures against the P66A schema and P66B-defined pack keys. Run the same corpus through native and browser-facing readers. | W1 |
| Strict catalog and manifest validation | Reject duplicate identities, unsafe paths, count or digest mismatches, invalid UTF-8 boundaries, oversized payloads, and incomplete joins before publication. | W1 |
| Immutable validated content session | Parse, validate, bind, and install a complete session atomically. A failed replacement must leave the previous valid session usable. | W2 |
| Pure presenter and source-style navigation | Separate selected topic, faction, category, previous/next order, and empty-source state from drawing and platform I/O. | W3 |
| Explicit texture ownership | Cache decoded artwork by validated resource identity and release replaced sessions without per-frame decoding or stale handles. | W4 |
| Recoverable packed and loose publication | Keep ORPK as the release authority. Permit a development-only loose form through the same validator and publish either form atomically. | W5 |
| Original-first HD selection | Resolve original, enhanced, and fallback candidates through a deterministic selector governed by the existing HD profile. | W6 |
| Presence-aware native mod overlays | Apply overlays atomically and distinguish absent fields from explicit removal or replacement. | W7 |
| Coalesced watcher recovery | Treat native file watching as a later developer/mod-author convenience with debounce, validation, and last-known-good recovery. | W8 |
| Fixture-route consistency | Drive deterministic test routes through the same presenter and renderer used by production. Keep fixtures unavailable in release UI. | W3-W5 |

## Non-goals

- Do not merge or cherry-pick the 51-commit PR chain as a unit.
- Do not create an alternative catalog schema, runtime pack, or entity-binding
  authority beside P66A and the P66B workstream.
- Do not enable the production cockpit command merely because the catalog
  parses or a test-only fixture renders.
- Do not add browser-side mod installation, browser hot reload, or arbitrary
  filesystem discovery in this delivery.
- Do not make enhanced artwork the default or use it to pass original-interface
  visual acceptance.
- Do not add Encyclopedia presentation data to saves, multiplayer state,
  deterministic simulation, or replay commands.
- Do not commit original game data, generated proprietary packs, screenshots
  without a documented evidence purpose, or local installation paths.

## Execution sequence

### W0. Import preflight and provenance map

Before porting code, record each selected PR path and commit against its current
upstream destination. Confirm that current main already supplies the relevant
behavior before copying anything. Prefer a focused reimplementation where the
upstream type or namespace changed after P65.

Preflight requirements:

- repair the PR's isolated `rebellion-data` test compile defect before using its
  test module as a reference. The test needs `std::io::Read` for `.take(...)`;
- treat the PR's 45 passing `encyclopedia_contract` cases as useful design
  evidence, not as proof against current main;
- resolve the four known direct conflicts conceptually before extracting code:
  `interface_test_fixture.rs`, `rebellion-data/src/lib.rs`, the canonical
  Encyclopedia design plan, and `tools/stage-ui-assets/cli.go`;
- carry no generated or proprietary fixtures into the repository;
- keep the PR branch available as read-only provenance until the adaptation is
  complete.

Exit: every retained idea has one upstream owner, destination, test gate, and
source commit. No planned change requires a second runtime authority.

### W1. Current-schema conformance corpus

Build a compact synthetic corpus from the current P66A schema. Include one
valid minimal catalog, one complete factional join fixture, and targeted invalid
fixtures. Generate larger bounded cases during tests instead of checking in
large repetitive JSON.

Required rejection cases:

- duplicate topic, resource, family, or manifest identities;
- unknown required fields and invalid enum values where the schema is closed;
- missing required fields, null where a value is required, and integer overflow;
- traversal, absolute, mixed-separator, control-character, and case-collision
  paths;
- invalid encoded text boundaries and malformed JSON;
- declared count, byte length, digest, or resource-key mismatches;
- decompression or allocation requests beyond documented limits;
- bindings that reference absent source text or artwork except for the ten
  explicit source-empty mission records.

Run identical semantic cases through the native loader, runtime-pack reader,
and any development loose reader. Their errors may be platform-shaped, but
their accept/reject decisions and normalized catalog fingerprints must match.

Exit: the corpus passes on native and WASM-facing code paths, invalid content
fails before installation, and a deterministic fingerprint identifies the
installed logical catalog.

### W2. Immutable content session and atomic install

Introduce one read-only session that owns the validated catalog, typed bindings,
resource metadata, and texture-generation identity. Build the candidate fully
off to the side, then replace the active session in one operation.

Invariants:

- readers never observe a partially parsed catalog;
- a failed install leaves the previous valid session intact;
- replacing a session invalidates only Encyclopedia-owned caches;
- the session is platform-neutral after bytes have been loaded;
- installation does not mutate `GameWorld` or its serialized fingerprint;
- native and browser installations of the same pack produce the same logical
  fingerprint and topic counts.

Exit: valid replacement, invalid replacement, repeated install, teardown, and
last-known-good recovery tests pass without stale texture or state ownership.

### W3. Presenter, ordering, and deterministic routes

Add a pure presenter over the immutable session. It owns no textures and
performs no I/O. It derives the visible title, description, image resource,
category, faction, previous/next targets, source-empty state, and return route.

Required behavior:

- reproduce the source-derived category and index order exactly;
- preserve faction-specific topic binding;
- define boundary behavior for previous and next without wraparound guesses;
- show the ten source-empty mission records as explicit unavailable-source
  states, not fabricated descriptions or borrowed artwork;
- accept cockpit, index, object, and contextual entry intents through one API;
- close or return to the exact caller without leaking a fixture-only route;
- keep deterministic test selectors behind test compilation or fixture gates.

Exit: table-driven presenter tests cover all 356 logical objects per faction,
including 346 resolved topics and ten explicit source-empty records.

### W4. Authentic topic surface and texture lifecycle

Connect the presenter to the existing Encyclopedia window. Reuse the authentic
bitmap frame, tabs, controls, fonts, palette behavior, and hit geometry. The
topic surface must not introduce invented menus or generic web controls.

Required evidence:

- resource mapping from the selected logical topic to its exact staged asset;
- cache hit and decoded dimensions for the displayed bitmap;
- inspected screenshots for both factions, representative categories, long and
  short descriptions, source-empty topics, and previous/next boundaries;
- hover, pressed, selected, disabled, keyboard-focus, close, and return states;
- no per-frame decode, unbounded cache growth, stale session texture, console
  error, missing-asset error, or unexpected network request;
- correct behavior at the original 640x480 surface and supported browser
  viewports without stretching fixed bitmap controls.

Exit: the relevant `OBJ-01` implementation cells have deterministic A1 evidence.
Strict visual cells remain open until compared with lossless original A0
baselines.

### W5. Production routing and recoverable publication

Publish the validated catalog and resources through the existing pack builder.
ORPK remains the release format and startup request-budget authority. A loose
representation may exist for local development only if it passes the same
validator and cannot silently change production resolution order.

Publication must:

- write to a temporary target, validate the complete result, then replace the
  destination atomically;
- retain the previous valid artifact if staging, validation, or replacement
  fails;
- produce stable ordering, normalized metadata, and deterministic hashes;
- include a verification mode that reads the published artifact back;
- preserve the established browser startup request budget;
- leave command `0x131` fail-closed until the full topic, navigation, close, and
  return journeys pass on the packaged release artifact.

Exit: native and packaged browser builds consume the same logical catalog.
Cockpit and contextual journeys pass for both factions with clean console,
network, missing-asset, and return-state evidence.

### W6. Original-first HD selection

Adapt the PR's deterministic asset-choice model behind the existing faithful-HD
profile. The selector receives a validated topic resource and returns the
original asset unless an enabled, digest-approved enhanced asset is present.

Exit: original, enhanced, missing-enhanced, invalid-digest, size-mismatch, and
fallback cases pass. Original-mode screenshots remain the only inputs to strict
original-interface acceptance.

### W7. Native content overlays

Extend the current mod loader with an Encyclopedia presentation target outside
`GameWorld`. Apply enabled mods in existing dependency order to a candidate
session, validate the complete result, then install it atomically.

Overlay semantics must distinguish:

- field absent, which inherits the lower layer;
- field present with a replacement value;
- field explicitly removed where removal is legal;
- whole-topic replacement or addition where the extension contract permits it.

Exit: dependency order, partial overlay, explicit removal, missing artwork,
invalid overlay, rollback, disable, and base restoration tests pass. Save and
simulation fingerprints remain unchanged.

### W8. Native watcher and authoring loop

Add file watching only after W1 through W7 are stable. Coalesce related writes,
wait for files to settle, build a candidate session, and install only after full
validation. Surface actionable diagnostics while retaining the last-known-good
session.

Exit: burst writes, editor rename patterns, partial files, invalid updates,
recovery, repeated reload, and shutdown pass without event storms, partial
state, or cache leaks. This phase is optional for the first production route.

## Commit train

Each unit is reviewed, documented, committed, and pushed before the next unit
begins. Exact checkpoint numbers follow the active interface audit when work is
scheduled.

1. `test: add current-schema Encyclopedia conformance corpus`
2. `refactor: install immutable Encyclopedia content sessions`
3. `feat: add source-ordered Encyclopedia presenter`
4. `feat: render authentic Encyclopedia topic surfaces`
5. `feat: publish and route packaged Encyclopedia content`
6. `feat: select original-first Encyclopedia HD artwork`
7. `feat: apply native Encyclopedia content overlays`
8. `feat: recover native Encyclopedia authoring reloads`

Every commit updates the canonical plan, audit JSON, audit Markdown, recovery
record, and evidence index as applicable. A passing unit must remain revertible
without changing source extraction or save compatibility.

## Verification matrix

| Gate | Required proof |
|---|---|
| Source | Exact extraction counts, hashes, provenance, 346/356 factional resolution, and ten explicit source-empty records |
| Conformance | Shared valid and adversarial corpus with matching native and WASM-facing decisions |
| Installation | Atomic replacement, invalid-candidate rollback, deterministic catalog fingerprint, and clean teardown |
| Presentation | All topics enumerated, source order preserved, boundaries deterministic, and no invented fallback content |
| Artwork | Exact resource mapping, cache hit, native dimensions, and inspected displayed pixels |
| Interaction | Both factions; index, topic, previous, next, contextual entry, close, and exact return route |
| Packaging | Deterministic ORPK output, read-back verification, request-budget preservation, and no proprietary tracked output |
| Runtime | No browser console, network, missing-asset, panic, stale-cache, or per-frame-decode failures |
| Persistence | Save, replay, multiplayer, and simulation fingerprints unchanged |
| Mods | Deterministic dependency order, presence-aware overlay semantics, invalid-update rollback, and base restoration |
| Visual parity | A1 implementation evidence for every covered `OBJ-01` cell, then strict comparison to lossless A0 baselines |

The final production gate runs the packaged artifact, not a development server
or fixture-only route. Browser sessions start muted and close with their local
server after evidence capture.

## Rollback points

- W1 is test-only and can be reverted without runtime changes.
- W2 is inactive until a caller explicitly installs a validated session.
- W3 remains behind existing gated routes until W4 and W5 pass.
- W4 can fall back to the currently gated Encyclopedia command without changing
  source data or saves.
- W5 retains the previous valid pack when publication fails.
- W6 always falls back to original artwork.
- W7 retains the unmodified base session when an overlay fails.
- W8 retains the last-known-good session and can be disabled independently.

## Risks and controls

| Risk | Control |
|---|---|
| Competing schemas emerge from the handoff | Port behavior into P66 types. Do not retain PR-only public models. |
| The large PR obscures later upstream fixes | Compare each selected path against current main before adaptation. |
| Fixtures become a second production route | Compile or expose deterministic selectors only through the test harness. |
| Publication corrupts a working asset pack | Temporary output, full read-back validation, atomic replacement, previous-artifact retention. |
| HD or mods mask original parity gaps | Original mode is default and is the only strict original-interface acceptance input. |
| Hot reload expands first-release scope | Keep W8 optional and native-only after production routing passes. |
| Browser and native drift | One logical fingerprint, shared presenter tests, and packaged two-platform journeys. |
| Proprietary data enters Git | Synthetic fixtures only, ignored generated packs, and explicit staged-path review before every commit. |

## Provenance map

These PR #16 commits are design references. Current-main implementations may
use different files, types, and names.

| Adapted area | Will's reference commits |
|---|---|
| Conformance corpus and validation | `d146e705`, `9a76af3a`, `32c2166e` |
| Immutable sessions and bounded loading | `49732c2c`, `fe5d0eac`, `18f2dedf` |
| Presenter and navigation | `285e548e`, `40e9fb63`, `ad984a47`, `88da5090` |
| Recoverable publication and browser loading | `29573596`, `7e314614`, `26f55139`, `22b597bc` |
| HD selection and overlays | `4ded440c`, `01b461c0`, `13ee19dd`, `07c29e0c` |
| Native watcher recovery | `08bc84a3`, `1dbcf2a9`, `eb3a627f` |
| Fixture-route consistency | `335334d9` |
| P65 reconciliation and final handoff | `2df6c45f`, `82ef2ccc` |

## Definition of done

This plan is complete only when:

1. W1 through W7 pass, or a documented decision explicitly defers an optional
   workstream without weakening production parity.
2. Both factions traverse the packaged production Encyclopedia through index,
   topic, navigation, contextual entry, close, and exact return paths.
3. Native and browser builds install the same logical catalog and display the
   exact selected original assets with no runtime errors.
4. All covered `OBJ-01` cells have implementation evidence. No strict visual
   cell is marked accepted without a lossless original baseline and comparison.
5. Saves, replay, multiplayer, and deterministic simulation are unchanged.
6. The audit, roadmap, recovery record, plan indexes, and evidence indexes agree.
7. Generated source content, original game assets, secrets, and local paths
   remain outside version control.

## Decision log

- 2026-10-02: adapt the strongest PR #16 patterns into current P66 contracts.
  Do not merge the dependency-linked handoff wholesale.
- 2026-10-02: retain P66A source extraction and binding as the only catalog
  authority, and reserve the P66B ORPK workstream as the packaged delivery
  boundary once its checkpoint lands.
- 2026-10-02: schedule conformance, immutable sessions, presenter, authentic
  rendering, and production routing before optional authoring hot reload.
- 2026-10-06: implement W2 as a caller-owned, platform-neutral data session
  over P66A types. The inactive store performs full candidate preparation before
  one-pointer publication, admits only the source-backed 400-by-200 indexed BMP
  class within per-resource and aggregate byte limits, and exposes only an
  Encyclopedia texture generation. W5 remains responsible for native and
  packaged readers, cross-target reader parity, and production install.
- 2026-10-06: implement W3 as a pure projection over the installed W2 session.
  The presenter preserves the validated P65 sequence, keeps source-empty topics
  explicit, bounds native-style previous/next without wrap, and carries one
  immutable origin route through cockpit, index, object and typed contextual
  journeys. W4 still owns rendering/textures and W5 still owns production
  routing and reader parity.
- 2026-10-06: implement W4 through a render-owned borrowed DTO and a mechanical
  application adapter, preserving the one-way render dependency boundary. The
  authentic STRATEGY shell, index/topic controls, scrolling and keyboard paths
  now have a ten-case two-faction fixture-gated browser A1 matrix. One selected
  topic texture is keyed by session generation and exact resource metadata and
  released on every replacement, unavailable/index state, error, or teardown.
  Synthetic topic bytes prove the surface only; W5 still owns canonical-reader
  parity, packaged production journeys, and route activation, while original
  A0 comparison retains exact typography/title-placement acceptance.
- 2026-10-06: split W5 at the dependency boundary instead of creating a cycle.
  W5A publishes and installs the canonical bytes first so W1 and W2 can close
  their native/packed/loose parity gates; E30 then owns packaged visual
  acceptance, and E32 retains final command `0x131` activation. ORPK remains
  the production authority, while the development-only loose form uses an
  independently atomic immutable generation pointer and cannot override a
  present production pack.
- 2026-10-06: implement W6 as one native startup snapshot over the validated
  W2 session and the existing faithful-HD approval manifest. Original parity
  never reads the enhanced path; faithful-HD admits only source-matched,
  digest-approved, exact-4x PNG output and otherwise retains the complete
  original view. Browser builds and strict original-interface acceptance stay
  original-only, while E32 retains production route activation.
- 2026-10-07: implement W7 as a native presentation target reserved outside
  `GameWorld`. Resolve one dependency-first order with lexicographic ready
  tie-breaks and feed it to world and Encyclopedia consumers; parse
  presence-aware patch/replace/add/remove actions; confine bounded author BMPs;
  validate the immutable base and every complete layer; and publish only the
  final W2 candidate. Any failure retains the active snapshot, while disabling
  all layers rebuilds the exact base. Browser content and save/simulation
  serialization remain unchanged; W8 watcher recovery stays optional.
- 2026-10-07: activate E32 only as a paired route/package increment. New
  production browser builds require the complete canonical namespace before
  compilation, while old packs with the whole namespace absent still boot with
  the route unavailable. Cockpit and real object-popup callers now share the
  accepted presenter/renderer, preserve typed return origins, refresh across
  native session generations, and reconstruct raw live IDs only through exact
  source-family/name matches. Both-faction packaged journeys and a native real
  contextual journey pass; strict original A0 and `OBJ-01` acceptance remain
  E35 work.
- 2026-10-07: E34 accepts the current-main assembled technical candidate. The
  final native, packed, and development-loose readers match one logical
  fingerprint; two consecutive builds match at the WASM and ORPK boundaries;
  both production factions, old-pack compatibility, replay, save/load,
  workspace, scoped lint/format, mutation-identity, and proprietary-data/state
  isolation gates pass. The divergent PR #16 fixture/fetch harness names are
  superseded by the approved current-schema reader corpus and production
  publication harness rather than imported wholesale. Strict lossless A0 and
  `OBJ-01` acceptance now remain solely E35.
- 2026-10-07: E35's adapted original-executable-under-Wine comparison finds and
  corrects faction-specific origin, title placement, production-font,
  initial-selection, tooltip, and modal-order defects. The expanded
  final-package journey passes all twelve applicable `OBJ-01` comparisons; the
  plan-defined source-unavailable state is honestly A0-not-applicable and
  passes from functional evidence. The user explicitly accepts Wine evidence
  for the current delivery, so all thirteen `OBJ-01` cells and this plan's
  visual gate pass without claiming native-Windows parity. A portable Windows
  capture kit, if pursued later, belongs to separate tooling/repository scope.
- 2026-10-07: E36 reconciles the plan, bead graph, audit records, release
  documentation, and proposed branch diff. A test-first repair restores clean
  Docker source staging and owned `EData` handoff to the strict WASM build.
  W1-W7 and all required production gates are closed; optional W8 automatic
  reload remains deferred in favor of the documented native **Reload Mods**
  action. No push or pull-request action is part of this checkpoint.
