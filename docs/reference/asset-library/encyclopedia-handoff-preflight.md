---
title: "Encyclopedia PR #16 handoff preflight"
description: "Current-main ownership and provenance map for selectively adapting PR #16"
type: reference
status: complete
created: 2026-10-06
updated: 2026-10-06
source_pr: "https://github.com/tdimino/open-rebellion/pull/16"
source_head: "82ef2ccc7f5b256470adaa3797898b8d79f2d7fc"
baseline: "3393d5c8"
tags: [encyclopedia, P66, provenance, handoff]
---

# Encyclopedia PR #16 handoff preflight

## Result

PR #16 remains useful implementation evidence, but it cannot be replayed onto
current main as an independent Encyclopedia stack. Upstream P65 and P66A now
own the immutable 356-object index, source extraction, source manifest, and
family-qualified topic binding. Every retained PR idea below has one
current-main owner, one adaptation destination, and one verification gate.

The implementation order remains W1 through W8 from the
[handoff adaptation plan](../../plans/2026-10-02-feat-will-forster-encyclopedia-handoff-adaptation.md).
P66A is the only catalog and binding authority. P66B is still a future
transport boundary; this preflight does not claim that its catalog namespace
or installer has landed.

This map was made against `origin/main` commit `3393d5c8`. The PR provenance
head is retained read-only at `82ef2ccc`. No PR production module, fixture
bundle, original-game asset, or generated pack was copied during W0.

## Current-main ownership

| Concern | Current owner and behavior | Adaptation boundary |
|---|---|---|
| Object index | `rebellion-data/src/encyclopedia_catalog.rs` constructs the immutable localized index from the canonical DAT tables and `TEXTSTRA`, preserving compound object identity, seven source categories, and global display order. | Extend this public model and its tests; do not restore the PR-only `rebellion_data::encyclopedia` schema. |
| Source extraction | `tools/stage-ui-assets/encyclopedia.go` extracts the owned English `ENCYTEXT` and `ENCYBMAP` resources into ignored `source.json` plus its checksum/count manifest. | Tighten this schema and publication path in place. `tools/stage-ui-assets/cli.go` remains the sole CLI owner. |
| Topic binding | `rebellion-data/src/encyclopedia_topics.rs` parses the P66A source profile, validates the manifest, and binds every object for one audience while preserving missing text, artwork mapping, or system-picture reasons. | W1 conformance cases target these public functions. Later sessions wrap their output instead of recomputing IDs. |
| Existing native load | `rebellion-app/src/main.rs` rebuilds the source index outside `GameWorld` and adapts it to `OriginalEncyclopediaCatalog`. | W2 replaces ad-hoc composition with one validated session after P66B supplies the packaged catalog bytes. |
| Existing browser artwork transport | `scripts/build-runtime-pack.py` packages validated 400x200 EDATA bitmaps under `encyclopedia/assets/`; `runtime_pack.rs::take_namespace` removes that namespace before `set_encyclopedia_asset_cache`. | P66B must add the catalog/manifest namespace. W1 may test current ORPK parsing, but must not invent the pending key contract. |
| Index rendering | `rebellion-render/src/encyclopedia.rs` owns the source-style 470x330 index shell, categories, selection, scrolling, and deterministic index fixtures. The older generic `draw_encyclopedia` panel still owns the currently reachable topic panel. | W3/W4 extend the source-style renderer and retire the generic topic path only after presenter and route gates pass. |
| Texture and HD policy | `rebellion-render/src/encyclopedia.rs` owns EDATA texture caching; `bmp_cache.rs` owns the original-first `AssetRenderProfile` and digest-approved faithful-HD fallback policy. | W4 session identity invalidates Encyclopedia-owned textures. W6 composes with the existing HD policy rather than adding a second selector. |
| Mods | `rebellion-data/src/mods.rs` owns `mod.toml`, dependency ordering, and world JSON patches. It has no separate Encyclopedia content target on current main. | W7 adds a presentation target outside `GameWorld`, reusing dependency order while keeping installation in the Encyclopedia session layer. |
| Deterministic routes | `rebellion-app/src/interface_test_fixture.rs` has stable scenario codes 38, 40, and 41 for artwork, index shell, and index catalog. | W3-W5 add scenarios to this registry and use production presenter/render paths; they do not restore the PR's parallel fixture harness. |

## Retained-idea provenance

Paths in the second column describe the PR implementation being used as a
reference. They are not import destinations.

| Retained idea | PR commits and reference paths | Current-main destination and owner | Test gate | Decision |
|---|---|---|---|---|
| Compact shared conformance corpus and strict validation | `d146e705`: `tests/fixtures/encyclopedia/**`; `9a76af3a`: `crates/rebellion-data/src/encyclopedia/{mod.rs,model.rs}`, `tests/encyclopedia_contract.rs`; `32c2166e`: `encyclopedia/validate.rs` and contract tests | W1: `rebellion-data` tests around `encyclopedia_catalog.rs` and `encyclopedia_topics.rs`; Go tests around `tools/stage-ui-assets/encyclopedia.go`; ORPK cases in `rebellion-app/src/runtime_pack.rs` only for landed keys | One compact valid P66A fixture plus generated adversarial cases produce matching accept/reject outcomes; extend to P66B catalog keys when they land | Port cases and invariants, not PR wire structs or its large checked-in generated bundle tree. |
| Immutable validated byte session and bounded native load | `49732c2c`: `rebellion-render/src/encyclopedia_assets.rs`; `fe5d0eac`: `rebellion-app/src/encyclopedia_session.rs`; `18f2dedf`: `encyclopedia_runtime.rs` | W2: one app-owned session assembled from `EncyclopediaCatalog`, `EncyclopediaTopicCatalog`, and the P66B transport result; bounded byte inspection may live beside the relevant parser | Valid install, invalid replacement rollback, repeated install, teardown, identical native/browser logical fingerprint, and unchanged `GameWorld` serialization | Adapt lifecycle and ownership rules. Do not copy PR catalog/model types or establish a second loader. |
| Pure presenter and stable view | `285e548e`: `rebellion-app/src/encyclopedia_presenter.rs`, `rebellion-render/src/encyclopedia_view.rs` | W3: app-owned pure presenter over the W2 session; renderer receives an immutable view | Table-driven coverage of all 356 objects for both audiences, exact order, 346 complete and ten explicit source-empty records, deterministic boundaries and return intents | Reimplement against P66A identities and the existing original index state. |
| Navigation and bounded texture ownership | `40e9fb63`: `encyclopedia_textures.rs`; `ad984a47`: `encyclopedia_navigation.rs`; `88da5090`: source-backed surface changes | W3/W4: navigation joins the presenter; texture/session generation integrates with `rebellion-render/src/encyclopedia.rs` and its EDATA cache | Previous/next boundaries, exact return route, cache hit, decode dimensions, generation invalidation, no per-frame decode, and no stale handle after replacement | Preserve the useful separation of presentation, navigation, and texture ownership without adding parallel renderer entry points. |
| Recoverable publication | `29573596`: `tools/stage-ui-assets/encyclopedia_{stage,verify,lock}*`; `7e314614`: runtime-pack and web packaging scripts | W5: extend `stage-ui-assets` source publication and the canonical `scripts/build-runtime-pack.py` ORPK builder | Temporary publish, read-back validation, atomic replacement, deterministic bytes/hashes, failure retains prior artifact, packaged native/browser fingerprint equality | Adapt atomic publication around the current P66A outputs and future P66B namespace. ORPK remains release authority. |
| Bounded browser loading and development loose form | `26f55139`: `rebellion-app/src/encyclopedia_fetch.rs`; `22b597bc`: `encyclopedia_loose.rs` | W5: current `runtime_pack.rs` is the packaged reader. A loose reader, if retained, is development-only and must feed the same validator/session | Status, byte, allocation, path, duplicate, truncation, and request-budget cases; packaged journey is the production acceptance gate | Do not land a loose form before P66B fixes the canonical keys and validation boundary. |
| Original-first HD selection | `4ded440c`: `rebellion-app/src/encyclopedia_hd.rs` plus presenter/cache changes | W6: compose Encyclopedia resource selection with `bmp_cache.rs::AssetRenderProfile` and its approval-manifest digest checks | Original, approved enhanced, missing enhanced, bad source/output digest, wrong dimensions, and original fallback | Retain deterministic selection, but the current shared faithful-HD policy owns approval and fallback. |
| Presence-aware native overlays | `01b461c0`: reserved Encyclopedia target in `mods.rs`; `13ee19dd`: `encyclopedia/overlay.rs`; `07c29e0c`: `encyclopedia_mods.rs` and tests | W7: `rebellion-data/src/mods.rs` separates presentation bytes from world patches; the W2 app session validates and atomically installs the overlay result | Dependency order, absent/replace/remove semantics, invalid overlay rollback, disable/base restoration, unchanged save and simulation fingerprints | Adapt explicit presence semantics and confinement. Never merge Encyclopedia content into world JSON. |
| Coalesced native watcher recovery | `08bc84a3`: `encyclopedia_lifecycle.rs`; `1dbcf2a9`: `encyclopedia_watcher.rs`; `eb3a627f`: lifecycle/fixture acceptance | W8: optional native app authoring layer over the W2 installer and W7 resolved mod roots | Burst writes, rename patterns, partial/invalid files, recovery, repeated reload, shutdown, no event storm or cache leak | Defer until W1-W7 are stable; native-only and last-known-good. |
| Fixture-route consistency | `335334d9`: PR fixture router and browser surface scenarios | W3-W5: extend `interface_test_fixture.rs` and interface-parity scenarios using the production presenter and renderer | Stable scenario registration, release build exclusion, packaged route coverage, clean console/network/missing-asset evidence | Port assertions and route discipline, not the PR's parallel `encyclopedia_test_fixture.rs`. |
| P65/P66 reconciliation lessons | `2df6c45f`: PR-side catalog provenance; `82ef2ccc`: optional-content reconciliation and `lib.rs` changes | P66A `encyclopedia_catalog.rs` and `encyclopedia_topics.rs` remain authoritative | Current-main data tests plus the W1 356-object/factional corpus | No code transplant. These commits explain intent and edge cases only. |

## Direct-conflict rulings

| Conflict path | Ruling |
|---|---|
| `crates/rebellion-app/src/interface_test_fixture.rs` | Preserve current scenario IDs and registry. Add W3-W5 scenarios incrementally and delegate to production presenter/render functions. Do not replace it with PR `encyclopedia_test_fixture.rs` or renumber existing routes. |
| `crates/rebellion-data/src/lib.rs` | Preserve the public P66A `encyclopedia_catalog` and `encyclopedia_topics` modules. W1 imports those APIs directly; it does not restore `pub mod encyclopedia` or PR-only model exports. |
| `docs/plans/2026-09-27-design-encyclopedia-data-pipeline.md` | The upstream design remains canonical. This preflight and the handoff adaptation plan record provenance and sequencing only; PR design text cannot become a second contract. |
| `tools/stage-ui-assets/cli.go` | Preserve the current `--encyclopedia-only`, `--encyclopedia-output`, and `--verify` surface. Publication hardening extends that path; PR-specific catalog/report modes are not imported. |

## PR compile and test evidence

The adaptation plan recorded an isolated test compile defect involving
`Read::take`. At the retained PR head `82ef2ccc`, that defect is already
repaired: `crates/rebellion-data/src/mods.rs` imports `std::io::Read`, and
`crates/rebellion-app/tests/encyclopedia_lifecycle.rs` imports both `Read` and
`Write`. A detached build of the retained head completed successfully:

```text
cargo test -p rebellion-data --no-run
Finished test profile; all rebellion-data test executables built.

cargo test -p rebellion-data --test encyclopedia_contract
45 passed; 0 failed
```

Those 45 cases validate the PR schema, not P66A, so they remain design evidence
only. W1 must express selected invariants against current-main types and wire
data. Old-PR and current-main builds also need distinct `CARGO_TARGET_DIR`
values; their packages share names and versions, and a shared target cache can
reuse the wrong branch artifact.

## W1 handoff

W1 starts at the current public parsing boundary, before any session or UI
work:

1. Add a compact synthetic P66A source/manifest fixture and test helpers under
   `crates/rebellion-data/tests/`; generate large or repetitive cases in memory.
2. Exercise `parse_encyclopedia_source_with_manifest` and
   `bind_encyclopedia_topics` without introducing a new public schema.
3. Add matching strictness cases to `tools/stage-ui-assets` where extraction or
   source publication owns the rejection.
4. Exercise current ORPK structural rejection in `runtime_pack.rs`, but defer
   catalog-key parity cases until the P66B namespace is present on main.
5. Keep all fixture bytes synthetic and small. Do not copy the PR's generated
   fixture tree, original assets, local installation paths, or pack outputs.

W1's first commit gate is the current P66A parser/binder corpus. Its final
native/browser parity gate remains dependent on the canonical P66B transport
checkpoint; that dependency does not authorize an interim transport schema.

## W0 verification

The following checks were run from clean current main unless noted:

```text
CARGO_TARGET_DIR=/data/tmp/cargo-target-encyclopedia-w0 \
  cargo test -p rebellion-data encyclopedia --lib
8 passed; 0 failed

go test ./tools/stage-ui-assets -run Encyclopedia -count=1
ok

CARGO_TARGET_DIR=/data/tmp/cargo-target-encyclopedia-w0 \
  cargo check -p rebellion-app --tests
passed with two pre-existing future-incompatible macro semicolon warnings
```

`cargo test -p rebellion-app runtime_pack` compiled but could not link on this
host because `libasound` is absent (`rust-lld: unable to find library
-lasound`). `cargo check -p rebellion-app --tests` provides the compile gate;
the runtime-pack unit tests remain a W1 CI/host gate rather than a claimed pass.

W0 exit is satisfied: each retained idea has one owner, destination, test gate,
and source commit; the four direct conflicts have explicit rulings; and no
planned change requires a second runtime authority.
