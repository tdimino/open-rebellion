# E36 Encyclopedia final reconciliation

Date: 2026-10-07

Bead: `orlocal-818.36`

Candidate branch: `test/encyclopedia-conformance`

Upstream baseline: `c460831f37cdaeb3acbae1d5bb19b61b23becb33`

Accepted audit checkpoint: `20f141847c13d4da23d2f0f1e8a2af2ecb2d6327`

## Result

The approved handoff adaptation is complete for its first production route.
W1 through W7, production publication and routing, technical isolation, and
the current visual gate pass. W8 automatic authoring reload remains the plan's
explicitly optional follow-up; native authors use **Reload Mods** after
Encyclopedia text or artwork changes.

The visual decision is deliberately bounded. Twelve applicable cells pass
lossless production/original-executable comparison under Wine. The thirteenth,
source-unavailable state passes functionally with original A0 not applicable.
The user accepted this as sufficient for the current delivery on 2026-10-07.
Native-Windows parity is not claimed, and a portable Windows capture kit is
separate tooling/repository scope rather than an Encyclopedia release blocker.

## Approved-plan traceability

| Plan phase | Bead result | Implementation commit(s) | Durable evidence |
|---|---|---|---|
| W0 provenance and preflight | `orlocal-818.57` closed | `7e280c3e` is already in upstream history | [Approved handoff plan](../../../plans/2026-10-02-feat-will-forster-encyclopedia-handoff-adaptation.md) |
| W1 current-schema corpus | `orlocal-818.58` closed | `0d4a5273` | [Canonical publication](2026-10-06-encyclopedia-canonical-publication.md), [technical acceptance](2026-10-07-encyclopedia-technical-acceptance.md) |
| W2 immutable atomic session | `orlocal-818.59` closed | `2e67c313` | [Content session](2026-10-06-encyclopedia-content-session.md) |
| W3 pure presenter and routes | `orlocal-818.60` closed | `72604ee1` | [Presenter](2026-10-06-encyclopedia-presenter.md) |
| W4 authentic topic surface | `orlocal-818.61` closed | `2934a736`, `01cd59d0`, `77acc48a` | [Topic surface](2026-10-06-encyclopedia-topic-surface.md), [canonical surface](2026-10-06-encyclopedia-canonical-surface.md), [A0 compatibility](2026-10-07-encyclopedia-a0-compatibility.md) |
| W5 publication and routing | `orlocal-818.62`, `orlocal-818.18`, `orlocal-818.30`, and `orlocal-818.32` closed | `ee7ff7fd`, `01cd59d0`, `6e09dbee`, `ff7ea6f0` | [Canonical publication](2026-10-06-encyclopedia-canonical-publication.md), [production activation](2026-10-07-encyclopedia-production-activation.md) |
| W6 original-first HD | `orlocal-818.63` closed | `b38c6d7e` | [HD selection](2026-10-06-encyclopedia-hd-selection.md) |
| W7 native overlays | `orlocal-818.64` closed | `b78a1980` | [Mod overlays](2026-10-07-encyclopedia-mod-overlays.md) |
| W8 watcher/authoring loop | optional first-route phase, explicitly deferred | no automatic Encyclopedia watcher commit | [README_MOD.md](../../../../README_MOD.md) documents the bounded manual reload workflow |
| Cross-cutting technical gate | `orlocal-818.34` closed | `2d2befff` | [Technical acceptance](2026-10-07-encyclopedia-technical-acceptance.md) |
| Current visual gate | `orlocal-818.35` closed | `77acc48a`, `20f14184` | [A0 compatibility](2026-10-07-encyclopedia-a0-compatibility.md) |

The earlier E01-E56 source-recovery, schema, validation, transport, mod-lifecycle,
and UI-control beads are closed in the local issue graph. Their accepted
results feed the P66A source authority and the W1-W7 rows above; this index does
not duplicate their lower-level reports.

## Specification coverage

| Contract area | Final authority and result |
|---|---|
| Owned extraction and source identity | P66A strictly stages 348 prose records and 191 logical mappings; [topic-source evidence](2026-10-01-encyclopedia-topic-source-bindings.md) records 346 complete and ten explicit source-empty topics per faction. |
| Validation and installation | W1 and W2 reject malformed or oversized candidates before one atomic session publication and retain the last-known-good session on failure. |
| Native, packed, and loose readers | W5 and E34 prove one logical fingerprint and exact referenced bytes; ORPK remains the release authority and loose content is development-only. |
| Presentation and navigation | W3, W4, E30, E32, and E35 cover both factions, all seven categories, bounded previous/next, cockpit and contextual origins, exact returns, and the unavailable-source state. |
| Production packaging | New builds require complete canonical content; old packs fail closed. `ff7ea6f0` restores clean Docker source staging, `EData` handoff, `FORCE_REBUILD`, and `PREPARE_MODDING=0` behavior. |
| HD and mods | W6 retains original-first selection. W7 applies validated native-only presentation overlays outside `GameWorld`; browser content remains base-only. |
| Persistence and determinism | E34 verifies unchanged save, replay, simulation, RNG, and world fingerprints. |
| Visual evidence | E35 supplies twelve applicable lossless Wine comparisons; source-unavailable is functional with original A0 not applicable. All thirteen `OBJ-01` cells pass the accepted current gate without a native-Windows claim. |

## Final reconciliation checks

The E36 changes did not rerun unchanged owned-data, browser, Wine, mutation, or
full Rust journeys already hash-identified by E34 and E35. E36 changed audit
records, documentation, and one shell integration path; it added a synthetic
regression for that path and reran the relevant gates:

```text
python3 -m unittest scripts.test_encyclopedia_staging.EncyclopediaProductionStagingTests.test_clean_docker_build_stages_required_encyclopedia_before_strict_wasm -v
  RED before repair: failed when the strict build found no canonical source/EData handoff
  GREEN after repair: 1 test passed

python3 -m unittest discover -s scripts -p 'test_encyclopedia_staging.py' -v
  3 tests passed

bash -n scripts/build-wasm.sh scripts/docker-build.sh scripts/package-web.sh
  passed

node scripts/validate-interface-parity-ledgers.mjs --check
  pass: 44 families, 43 required, 633 cells, 627 required, 27 recovery packages

git diff --check
  passed
```

An explicit `origin/main...20f14184` path review found 14 focused commits and
no tracked `.beads`, `data/base`, `web/data`, distribution, build-output, BMP,
PNG, ORPK, or ZIP path. Checked-in Encyclopedia JSON files are compact synthetic
fixtures. Generated source catalogs, original artwork, packages, browser/Wine
artifacts, credentials, and local installation paths remain ignored.

## Retained limitations and handoff

- Native-Windows parity is unclaimed; current original-executable evidence is
  the accepted Wine capture set.
- The source-unavailable state has no legitimate original topic and therefore
  no A0 image.
- Browser mods and filesystem discovery remain unsupported by design.
- Automatic Encyclopedia watcher reload is deferred; native manual reload is
  documented and validated.
- This branch is local and ahead of the fork. No push, PR creation, PR update,
  merge, or upstream write was performed during E36.
