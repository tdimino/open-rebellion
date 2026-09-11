# Close The Final 3 Percent To Original Rebellion Parity

> **Status update (2026-09-08):** This plan is retained as implementation
> history, but its narrow “final 3 percent” conclusion is superseded for release
> acceptance by the [full-functionality audit](../qa/2026-09-08-full-functionality-audit/).
> Independent Astra and verified Fable 5.1 reviews found additional persistence,
> simulation, deterministic replay, browser packaging/performance, visual proof,
> and multiplayer-foundation work.

This ExecPlan is a living document. Sections Progress, Surprises &
Discoveries, Decision Log, and Outcomes & Retrospective must be
kept up to date as work proceeds.

## Purpose / Big Picture

Open Rebellion already has the strategic simulation loop, the economy rewrite, the
Open Souls refactor, the repair system, the Death Star cleanup path, and 403 tests.
The remaining work is no longer "finish the game." It is "close a small number of
specific parity holes so the project can honestly claim full feature parity with the
1998 original Star Wars Rebellion on native desktop builds."

Someone following this plan should be able to see the difference in three ways.
First, the last partially wired mission types become fully usable because the player
and AI can target characters, and covert missions can actually be foiled. Second,
the remaining AI fidelity gaps close, especially dispatch validation and ground-force
movement. Third, the last presentation gaps disappear: the encyclopedia shows real
system art, cutscenes play, and the advisor follows authored animation sequences
instead of a frame-cycling fallback.

## Progress

- [x] (2026-03-31 06:52Z) Read `CLAUDE.md` and the required architecture, roadmap, game-domain, simulation, AI, Ghidra, asset, DLL catalog, report, and completed plan documents.
- [x] (2026-03-31 06:52Z) Grepped the repository for `TODO`, `FIXME`, and `HACK` comments and separated live source comments from stale documentation and vendored code comments.
- [x] (2026-03-31 06:52Z) Reconciled stale gaps from the 2026-03-26 cross-reference report against later completed work in Knesset Ereshkigal, Knesset Ptah, and Knesset Hephaestus.
- [x] (2026-09-08) Replaced the approximate fleet-list icon offset with exact mappings for all 38 fighter and capital-ship classes; Astra-medium R9 passed the browser bitmap and interaction gate. Evidence: `../qa/2026-09-08-full-functionality-audit/evidence/2026-09-08-fleet-miniatures.md`.
- [x] (2026-09-08) Shipped deterministic `runtime.orpk` with 52 game-data entries and 2,231 BMPs. Astra-medium R6 passed both factions with four cold-load requests, zero loose asset requests, correct transparent fleet art, working interactions, and no errors across twenty zoom steps. Evidence: `../qa/2026-09-08-full-functionality-audit/evidence/2026-09-08-runtime-pack.md`.
- [x] (2026-09-08) Added F-011A canonical save-state fingerprints and v9 integrity metadata. The seeded two-run probe passes; Astra-medium R3 passed 34 browser save/reload/load, lossless metadata, bitmap, network, and error assertions. Evidence: `../qa/2026-09-08-full-functionality-audit/evidence/2026-09-08-state-fingerprints.md`.
- [x] (2026-09-09) Added the F-011B1 save v10 continuation envelope, deterministic populated typed-map fingerprints, and real-v9-artifact migration. All 483 workspace tests passed; Astra-medium R2 passed 40/40 browser storage, reload, continuation, bitmap, network, and error assertions. Evidence: `../qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-state-continuation.md`.
- [x] (2026-09-09) Closed F-001 browser Save/Load/Delete wiring and fixed empty-slot Load activation found by Astra R1. All 486 workspace tests passed; Astra-medium R2 passed 33/33 deletion, persistence, bitmap, network, and error assertions. Evidence: `../qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-save-delete.md`.
- [x] (2026-09-09) Replaced the browser custom text menu and visible setup page with the binary-mapped shuttle cockpit, direct faction start, and single-loop original menu music. Astra-medium R2 exercised the full browser matrix and R3 passed the corrected 11/11 commit gate. Evidence: `../qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-cockpit.md`.
- [x] (2026-09-09) Added F-016B campaign configuration persistence in save v11 and enforced Standard principal-leader capture versus Headquarters Only victory rules. All 495 workspace tests passed; Astra-medium R2 passed 18/18 browser setup, reload, bitmap, storage, network, and error assertions. Evidence: `../qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-game-setup-propagation.md`.
- [x] (2026-09-09) Completed F-016C: Credits, explicit M4 multiplayer status, gain/mute, return-to-menu, clean second-campaign reset, exact `MDATA.300` *Return of the Jedi* cue, and original `COMMON.DLL` menu effects. All 500 tests and native build/startup pass; Astra-medium completion evidence is in `../qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-completion.md`. Native interactive visual acceptance and semantic canvas accessibility remain P03 release-hardening work; P04 is complete.
- [x] (2026-09-09) Completed F-016D browser semantic accessibility without altering the bitmap-only cockpit. All 502 tests pass; Astra-medium passed nine of nine browser gates, 84/84 focus transitions, all 14 controls, every destination, `MDATA.300`, and all four original effects with four requests and zero runtime errors. Evidence: `../qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-semantics.md`. F-016E then closed the native visual gate.
- [x] (2026-09-09) Completed F-016E and P03. The binary reference proves the original had no standalone mute control; Open Rebellion now adds one documented 30×22 music-only extension styled from the Jiff Gorda/SWG Project Thorn reference. Native presentation was accepted, all 504 tests pass, and Astra-medium passed 10/10 browser gates across four viewports with exact hit/paint bounds, music muted after its initial-state assertion, SFX preserved, and zero errors. Evidence: `../qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-music-toggle.md`.
- [x] (2026-09-09) Added F-011B2 replay format v1 with strict typed command order, checkpoint positions, configuration identity, and canonical per-file/aggregate fingerprints for all 51 simulation DAT inputs. All 511 workspace tests, the native fixture, and the WASM compile gate pass; execution and cross-runtime equivalence remain open. Evidence: `../qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-replay-contract.md`.
- [x] (2026-09-09) Added the F-011B3 recorder/executor, fail-fast state checkpoints, save-v11 continuation proof, stable manufacturing/movement/blockade/AI ordering, and a five-process 200-system native golden. All 518 workspace tests and the WASM compile gate pass; native/WASM execution equivalence remains open. Evidence: `../qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-replay-execution.md`.
- [x] (2026-09-09) Added F-011B4 exact-artifact native/WASM replay equivalence. The 13,482-byte seed-42 artifact produces the same initial fingerprint, nine checkpoints, tick-25 final fingerprint, and artifact text in both runtimes; Astra medium passed success, invalid-query, missing-pack, and normal four-request browser gates with zero errors. F-007B intentionally updated tick-10 onward state, and the re-reviewed artifact passes all 530 workspace unit tests plus native/WASM equivalence. Evidence: `../qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-replay-wasm-equivalence.md`.
- [x] (2026-09-09) Completed F-007A: active movement orders cannot be replaced, AI fleet selection excludes transit and same-pass reservations, and rejected AI moves produce no false telemetry, movement message, or departure audio. The seed-42 5,000-tick run cut attack orders from 306,012 to 78,946 and reached 1.006 accepted moves per arrival. Astra medium passed both faction bitmap paths with four requests and zero errors. Fleet creation, arrival merging, combat backlog, and victory remain open under F-007. Evidence: `../qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-fleet-redispatch.md`.
- [x] (2026-09-09) Completed F-007B: accepted departures leave their origin index, stale indexes reconcile before production, and compatible anonymous arrivals consolidate deterministically while character and Death Star task forces remain distinct. Five 5,000-tick seeds now keep fleet arenas at or below 1.4 times their initial size and moves at or below 1.002 times arrivals. Native/WASM replay, 530 workspace tests, and Astra medium bitmap acceptance pass. Ordinary player dispatch, transit balance, combat distribution, and victory remain open. Evidence: `../qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-fleet-position-consolidation.md`.
- [x] (2026-09-09) Completed F-007C: the bitmap galaxy and fleet panel now issue validated player movement through the authoritative departure path, remove the fleet from its origin, show destination and countdown without overlap, and prevent repeat dispatch. The final Astra-medium pass found no P0-P3 issue across Alliance and Empire, with four requests per faction and zero runtime or asset errors. All 533 workspace tests and exact native/WASM replay pass. Evidence: `../qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-player-fleet-dispatch.md`.
- [x] (2026-09-10) Completed F-007D: every hostile task force at a system now enters one bounded engagement; fighter losses persist; local wings launch; shield handling and small-wing attrition make real progress; and unchanged stalemates do not reopen every five ticks. Seed 42 falls from 603 space events, including 602 recurring Xyquine draws, to two decisive engagements. All 541 workspace tests, exact native/WASM replay, five-seed diagnostics, and Astra-medium bitmap startup pass. F-007E retains the newly exposed AI balance, transit fan-in, repair telemetry, and victory failures. Evidence: `../qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-system-combat-resolution.md`.
- [x] (2026-09-10) Completed the F-007E logistics checkpoint: AI production requires system control; hostile neutral-orbit fleets remain targetable; friendly cycling stops; HQ defense is retained; surface troops survive blockade; compound `TROOPSD.DAT` IDs resolve; and persisted save-v12 repair episodes replace per-tick repair-start spam. Five runs finish with no transit, one move per arrival, at most 1.2× initial fleets, and 0–5 repair starts. Battle spread, Alliance/Empire balance, ground transport/occupation, leader capture, and victory remain open. Evidence: `../qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-ai-campaign-logistics.md`.
- [x] (2026-09-10) Completed the verified transport and occupation portion of F-007E: save-v13 fleet cargo enforces living transport capacity, survives transit and consolidation, lands after orbital control, continues unresolved surface battles, persists tactical casualties, and changes political control. Five 5,000-tick runs exercise that invasion chain. All 567 workspace tests, exact native/WASM replay, and Astra-medium two-faction bitmap/browser gates pass for the scoped path. A source review then reopened deterministic character capture and occupation-based Imperial HQ victory as parity defects. Evidence: `../qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-troop-transport-occupation.md`.
- [x] (2026-09-10) Completed F-007E player troop dispatch: the bitmap destination-first fleet chooser lists same-faction surface regiments, reports living transport capacity, blocks excess selection, embarks atomically, preserves cargo through authoritative transit, and lands it automatically. All 568 workspace tests and the packaged WASM pass. Astra medium demonstrated both factions, the 0/0, 2/2, and 3/3 capacity gates, Alliance arrival and landing, 12/12 HTTP responses, intact bitmaps, and zero browser or asset errors. Evidence: `../qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-player-troop-dispatch.md`.
- [x] (2026-09-10) Established the sister [original-interface audit](../qa/2026-09-10-interface-parity-audit/) with 43 stable surface families, 370 retained reference images, exact visual and hotspot thresholds, and a six-tranche reconstruction plan. The shuttle menu passes its scoped implementation gate but still needs the complete original interaction-state capture matrix. The in-campaign command center, galaxy, system navigation, advisor behavior beyond idle motion, object windows, tactical shell, and multiplayer surfaces are not yet original-parity.
- [x] (2026-09-10) Completed RE-ADV-01: the runtime pack now carries all 3,988 original type-302 advisor frames, and both factions render their original two-droid idle runs in native and WASM builds. All 582 workspace tests pass; Astra-medium found no P0-P3 issue in the scoped browser gate. Authored actions, voice sequencing, unobscured apertures, and full command-center parity remain open. Evidence: `../qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-authentic-droid-advisors.md`.
- [ ] Implement Milestone 1: targeted mission parity and covert foil parity.
- [ ] Implement Milestone 2: remaining AI decision parity.
- [ ] Implement Milestone 3: remaining UI and media parity.
- [ ] Implement Milestone 4: residual combat and Death Star fidelity cleanup.
- [ ] Decide whether to stop at "original feature parity" or add an optional strict legacy-behavior mode for existing augmentations.

### Current continuation milestones

- [ ] M0: complete native GUI restart acceptance and browser persistence hardening. Browser Save/Load/Delete, HD/troop-data paths, the self-contained browser package, F-011A fingerprints, the F-011B1 v10 continuation envelope, the F-011B2/B3 replay pipeline, and F-011B4 native/WASM fixture equivalence are verified.
- [ ] M1: stop fleet redispatch/spawn/backlog feedback loops and prove deterministic native/WASM replay across five 5,000-tick seeds. F-007A prevents redispatch and travel resets. F-007B closes stale position, production attachment, and compatible arrival accumulation. F-007C closes ordinary player dispatch. F-007D closes the system combat backlog. F-007E now closes ownership, friendly-cycle, transit, troop-class, repair-state, troop transport, player dispatch, political occupation, and the source-backed asymmetric victory contract. Capture/evasion, faction liveness, target acquisition, the wider campaign loop, and five-seed cross-runtime replay remain.
- [ ] M2: converge app, playtest, automatic combat, and tactical combat on one authoritative simulation/event path.
- [ ] M3: complete interface tranches UIP-T01 through UIP-T05, extend the runtime pack with every required original UI family plus compression and bounded caches, then finish authored advisor actions and voice, high-DPI rendering, browser audio, and Chrome/Firefox/Safari acceptance. RE-ADV-01 establishes the original two-faction advisor idle-frame foundation.
- [ ] M4: build validated commands, an authoritative fog-safe server, prediction/reconciliation, reconnect, and multiplayer security after determinism passes.
- [ ] M5: continuously gate format, clippy, release builds, app/browser integration, campaign bounds, all interface surface-state cells, bitmap screenshots, provenance traces, and performance budgets.
- [ ] v1.0: deploy the complete browser build to Cloudflare Pages behind secret-backed password middleware and signed sessions, then pass Astra-medium authentication, single-player, multiplayer, performance, and bitmap acceptance against the deployed URL.

The stable finding IDs, exact exit metrics, P00–P40 feature ledger, and browser
budgets live in the audit JSON. Parameter autoresearch remains paused until M1
closes so it does not tune around known simulation defects.

## Surprises & Discoveries

- Observation: the biggest gap named in the 2026-03-26 community cross-reference report, the economy tick loop, is no longer a live gap.
  Evidence: `CLAUDE.md` says Knesset Ereshkigal completed the economy and integrator work, `crates/rebellion-core/src/economy.rs` exists, and `crates/rebellion-data/src/simulation.rs` routes economy through the integrator.

- Observation: the 2026-03-28 TODO-resolution plan is also stale as a parity source because its last deferred TODO was already closed.
  Evidence: `CLAUDE.md` records Knesset Hephaestus as completed, and `crates/rebellion-core/src/repair.rs` now uses per-hull `ShipInstance` state.

- Observation: there are effectively no remaining `TODO` or `FIXME` markers in the Rust source tree.
  Evidence: `rg -n --hidden --glob '!target/**' --glob '!.git/**' --glob '!docs/**' --glob '!agent_docs/**' --glob '!.subdaimon-output/**' --glob '!web/gl.js' '(TODO|FIXME|HACK|todo!|unimplemented!)' crates tools scripts web` returned no live source hits. The only grep hits were documentation notes and three unrelated vendored comments in `web/gl.js`.

- Observation: the most important live gameplay gap is not economy or save/load. It is targeted-mission wiring.
  Evidence: `crates/rebellion-render/src/panels/mod.rs` defines `PanelAction::DispatchMission` without a target character, `crates/rebellion-app/src/main.rs` calls `mission_state.dispatch(kind, faction, character, target, duration_roll)`, and `crates/rebellion-core/src/missions.rs` still documents that target-character formulas are correct but inert without dispatch plumbing.

- Observation: the remaining UI parity work is a full in-campaign surface reconstruction, not merely resource lookup or media playback.
  Evidence: the [original-interface audit](../qa/2026-09-10-interface-parity-audit/) proves that current strategy controls, galaxy rendering, system sidebar, most object windows, tactical composition, and live ground-combat presentation are replacements or absent. RE-ADV-01 now packs and renders all original advisor idle frames, while authored actions, voice, briefing, dialog, and encyclopedia families remain open.

- Observation: the official manual plus period and official captures provide broad visual coverage, but rare transient states still need controlled captures from the original executable.
  Evidence: the interface audit retains 370 classified images and lists explicit gaps for button-state permutations, Death Star actions, tactical damage states, endings, rare events, and two-peer multiplayer.

- Observation: F-007E now completes transport, landing, surface combat, and political occupation, but campaign noncompletion remains multi-causal.
  Evidence: five current runs carry and land 8–15 regiments and resolve 6–9 ground engagements. Current occupation deterministically captures 5–6 characters, which remains a placeholder. The victory checker also equates Imperial occupation with destruction of the mobile Alliance HQ, lets Death Star outcomes bypass the Standard contract, and suppresses victory before tick 200. Campaign activity remains only 10–19 combined space/ground engagements, Alliance liveness collapses, and no run acquires every randomized Standard victory target.

## Decision Log

- Decision: use current source plus `CLAUDE.md` plus the completed 2026-03-26 and 2026-03-28 plans as the source of truth, not the older roadmap or the first-pass cross-reference report.
  Rationale: the older documents still report gaps that later sprints already closed, especially economy, repair, telemetry coverage, and Death Star cleanup.
  Date/Author: 2026-03-31 / Codex

- Decision: define "100% parity" as "the native game reproduces the original 1998 game's mechanics, story/media surfaces, and shipped player-facing features with original assets available."
  Rationale: web-only save support, HD art generation, and mod UX are not original Rebellion features and should not block a 100% parity claim.
  Date/Author: 2026-03-31 / Codex

- Decision: count interface parity by required surface-state cells, with no visible invented or unknown elements in parity mode.
  Rationale: resource decode success and a functioning replacement panel do not prove original bitmap identity, composition, geometry, hotspot behavior, navigation, animation, or audio. Any failed, partial, blocked, or unknown required cell prevents a 100% interface claim.
  Date/Author: 2026-09-10 / Codex

- Decision: treat existing augmentations as non-blocking unless they prevent access to the original behavior.
  Rationale: the project already intentionally improves some systems, especially AI target scoring and movement tuning. Those do not need to be removed to finish original feature parity, but they should be documented separately from parity blockers.
  Date/Author: 2026-03-31 / Codex

- Decision: estimate LOC as implementation plus tests, excluding generated assets and one-off batch outputs.
  Rationale: that estimate is what matters for staffing and parallelization.
  Date/Author: 2026-03-31 / Codex

- Decision: preserve character-bearing and Death Star task forces as distinct fleets while consolidating compatible anonymous arrivals.
  Rationale: this bounds fleet-record growth without erasing command assignments or strategically significant ship identity.
  Date/Author: 2026-09-09 / Codex

## Outcomes & Retrospective

This audit reduces the live parity backlog to a small, concrete set of gameplay,
AI, and presentation items. The roadmap from 97% to 100% is not another economy
rewrite or another sweeping refactor. It is a sequence of targeted closures.

The highest-value conclusion is that only one item looks like a real P0 release
blocker for a 100% parity claim: the missing target-character plumbing that leaves
Recruitment, Assassination, and Abduction structurally correct but not fully usable.
Everything else falls into significant AI fidelity, media fidelity, or polish.

The second important conclusion is that several frequently mentioned backlog items
are augmentation work, not parity work. If the team wants the shortest path to
100%, those items should be explicitly de-scoped from the parity burndown.

## Context and Orientation

Open Rebellion is a multi-crate Rust workspace rooted at
`/Users/tomdimino/Desktop/Programming/open-rebellion`. The main gameplay logic
lives in `crates/rebellion-core/src/`. The simulation orchestrator and world
mutation live in `crates/rebellion-data/src/simulation.rs` and
`crates/rebellion-data/src/integrator.rs`. The player-facing UI lives in
`crates/rebellion-render/src/`. The desktop entry point lives in
`crates/rebellion-app/src/main.rs`.

For this plan, "parity" means behavior or presentation that the original 1998 game
had and that Open Rebellion still lacks or approximates. "Augmentation" means a
feature that improves or extends the remake beyond the original, such as browser
save support or HD asset generation.

The documents reviewed for this audit were:

`CLAUDE.md`
`agent_docs/architecture.md`
`agent_docs/roadmap.md`
`agent_docs/game-domain.md`
`agent_docs/simulation.md`
`agent_docs/systems/ai-parity-tracker.md`
`agent_docs/ghidra-re.md`
`agent_docs/assets.md`
`agent_docs/dll-resource-catalog.md`
`docs/reports/2026-03-26-community-disassembly-cross-reference.md`
`docs/plans/2026-03-26-001-feat-eval-driven-parity-open-souls-refactor-plan.md`
`docs/plans/2026-03-28-001-feat-knesset-ptah-todo-resolution-plan.md`

The current repo state that matters most is:

`crates/rebellion-core/src/missions.rs` already has target-character-aware formulas,
but dispatch paths do not provide target characters or real foil defense values.

`crates/rebellion-core/src/ai.rs` still uses a simplified `can_dispatch()`.
Transport-backed AI fleet actions now carry troops, while player troop commands,
validator parity, and faction-specific budgets remain incomplete.

`crates/rebellion-render/src/encyclopedia.rs` does not resolve star-system art
through `ENCYBMAP.DLL`.

`crates/rebellion-render/src/advisor.rs` now decodes the original type-302 cumulative
advisor frames and renders the verified Alliance and Imperial idle runs. The BIN
control data for authored actions, timing, and voice sequencing is not fully mapped.

`crates/rebellion-core/src/combat.rs` still calls its weapon-fire path an
approximation of the original per-arc resolver.

`crates/rebellion-core/src/death_star.rs` still documents exact construction and
nearby-warning values as approximate.

## Plan of Work

The work should start with mission targeting because that closes the single clearest
parity blocker and unlocks the rest of the covert-operations cleanup. The underlying
math is already present in `crates/rebellion-core/src/missions.rs`. The missing work
is plumbing: the UI, `PanelAction`, the app entry point, the AI action type, and the
mission-state dispatcher all need to carry `target_character`. Once that exists,
Recruitment, Assassination, and Abduction stop being half-wired systems.

The next step should finish covert mission defense and AI dispatch fidelity. Add a
real defense-score path for foil checks in `crates/rebellion-core/src/missions.rs`,
then expand `crates/rebellion-core/src/ai.rs` so `can_dispatch()` reflects the 18
decoded validators closely enough to match original gating. Once the richer dispatch
context exists, add troop deployment, faction-specific budgets, and the remaining
small AI behaviors that still diverge from the original.

After simulation parity is closed, finish presentation parity. Parse the remaining
resource lookup tables needed for real star-system encyclopedia art and accurate UI
sprite selection. Then add native video playback for the decoded WebM cutscenes that
stand in for Smacker videos. Finish by extending the verified original advisor idle
frames with BIN-driven action, timing, and voice sequencing.

The final step is small but important. Resolve the last known combat and Death Star
approximations so the codebase no longer documents them as guesses. If the team wants
to market "strict behavioral parity" rather than "feature parity plus augmentations,"
add a legacy-mode pass after the core parity items are done. That work should not
delay the main 100% feature-parity claim.

## Gap Inventory

| ID | Category | Priority | Gap | Evidence | Estimated LOC | Dependencies | Counts Toward 100% Parity |
|---|---|---:|---|---|---:|---|---|
| F1 | Formula/Logic | P0 | Wire `target_character` end to end for Recruitment, Assassination, and Abduction. Add UI selection, AI target selection, dispatcher plumbing, and save-safe mission creation using the existing `ActiveMission.target_character` field. | `docs/plans/2026-03-26-001-feat-eval-driven-parity-open-souls-refactor-plan.md` says formulas are "structurally correct but inert at runtime." `crates/rebellion-render/src/panels/mod.rs` and `crates/rebellion-app/src/main.rs` do not carry a target character. | 180-280 | None | Yes |
| F2 | Formula/Logic | P1 | Replace the mission foil stub with a real counter-intelligence and defense-score path so covert missions can be foiled by defenders in original-style play. | `crates/rebellion-core/src/missions.rs` sets `let foil = foil_prob(0.0, true);` and comments that the defense score system is still a stub. | 140-220 | F1 recommended, not strictly required | Yes |
| A1 | AI | P1 | Port the remaining AI dispatch validators so `can_dispatch()` reflects the decoded 18-check original gate instead of the current basic ownership and busy-state checks. | `agent_docs/systems/ai-parity-tracker.md` still marks validator parity as partial. `crates/rebellion-core/src/ai.rs` `can_dispatch()` only checks faction, commander flag, busy, captive, and mission flags. | 300-450 | None | Yes |
| A2 | AI | P1 | Finish troop deployment parity by refining transport-backed AI target selection until it reaches all Standard victory objectives. | F-007E now carries troops through AI and player fleet dispatch, transit, landing, occupation, and capture. Five-seed runs still miss complete randomized victory targets. | 120-220 | A1 helpful | Yes |
| A3 | AI | P2 | Add faction-specific deployment budgets to `AiConfig` and fleet deployment logic so Alliance and Empire stop sharing one budget model. | `agent_docs/systems/ai-parity-tracker.md` and `agent_docs/roadmap.md` both still call this out. `crates/rebellion-core/src/tuning.rs` has no faction budget fields. | 40-80 | None | Yes |
| A4 | AI | P2 | Close the remaining strategic AI deltas: proactive uprising prevention, resource-balancing decisions, Death Star escort or retreat behavior, and production-order fidelity where the original differed from current heuristics. | These are the remaining live AI behavior gaps called out in `agent_docs/roadmap.md` and the 2026-03-26 cross-reference report. | 180-320 | A1 and A2 | Yes |
| U1 | UI | P2 | Parse the remaining DLL lookup tables needed for faithful art selection: `ENCYBMAP.DLL` for star-system encyclopedia art and real lookup-based resource mapping for tactical sprites. Fleet-list mini-icons are complete. | `crates/rebellion-render/src/encyclopedia.rs` uses a placeholder for systems and `crates/rebellion-render/src/tactical_view.rs` documents approximate sprite mappings. Commit `4589d2e` closed the fleet-list portion with exact GOKRES mappings and Astra-medium proof. | 180-300 | None | Yes |
| U2 | UI | P2 | Add native playback for the already-decoded cutscene videos used for intro, campaign setup, story events, and victory or defeat sequences. | `agent_docs/game-domain.md` and `agent_docs/roadmap.md` still list video playback as missing. There is no `crates/rebellion-render/src/video_player.rs` and no video code in `crates/`. | 280-450 | None | Yes |
| F3 | Formula/Logic | P2 | Replace the approximate aggregate weapon-fire resolver with a closer port of the original per-arc weapon-fire behavior. | `crates/rebellion-core/src/combat.rs` calls `phase_weapon_fire()` a known approximation of the original per-arc resolver. | 250-400 | None | Yes |
| C1 | Cosmetic | P3 | Extend the verified original advisor idle runs with BIN-driven authored actions, timing, and voice sequencing so the droids behave like the original briefings and cockpit advisors. | RE-ADV-01 renders the complete type-302 idle-frame families. The interface audit keeps RE-ADV-02 open for control-sequence mapping and unobscured visual acceptance. | 120-240 | RE-ADV-01 complete | Yes |
| F4 | Formula/Logic | P3 | Resolve the last documented Death Star and combat approximations: exact construction duration, exact nearby-warning radius, and any remaining residual combat-edge checks that are still only described as approximations. | `crates/rebellion-core/src/death_star.rs` marks the construction duration and warning radius as approximate. The remaining combat approximations are documented inline in `crates/rebellion-core/src/combat.rs`. | 40-80 | F3 recommended | Yes |

## Work Packages

The fastest safe execution plan is to run four work packages in this order.

Work Package A is the parity blocker package. It owns `crates/rebellion-core/src/missions.rs`,
`crates/rebellion-render/src/panels/mod.rs`, `crates/rebellion-render/src/panels/missions.rs`,
and the mission-dispatch call sites in `crates/rebellion-app/src/main.rs` and
`crates/rebellion-core/src/ai.rs`. This package closes F1 and most of F2.

Work Package B is the AI package. It owns `crates/rebellion-core/src/ai.rs` and
`crates/rebellion-core/src/tuning.rs`. This package closes A1 through A4. It can
start in parallel with Work Package A if the AI team agrees on the new
`target_character` field shape early, but it integrates more cleanly after A lands.

Work Package C is the resource-lookup and UI-media package. It owns
`crates/rebellion-render/src/encyclopedia.rs`, `crates/rebellion-render/src/tactical_view.rs`,
`crates/rebellion-render/src/panels/fleets.rs`, and any new loader/parser code needed for
`ENCYBMAP.DLL` or other DLL lookup tables. It also owns a new
`crates/rebellion-render/src/video_player.rs` plus the native app hooks in
`crates/rebellion-app/src/main.rs`. This package closes U1 and U2 and can run in
parallel with Work Package B.

Work Package D is the polish package. It owns `crates/rebellion-render/src/advisor.rs`,
`crates/rebellion-core/src/combat.rs`, and `crates/rebellion-core/src/death_star.rs`.
This package closes C1 and F4 and should land last because it depends on no
upstream interfaces and is mostly cleanup.

## Concrete Steps

The commands below assume the working directory is:

`cd /Users/tomdimino/Desktop/Programming/open-rebellion`

| Milestone | Command | Expected Output |
|---|---|---|
| Reconfirm no live source TODOs | `rg -n --hidden --glob '!target/**' --glob '!.git/**' --glob '!docs/**' --glob '!agent_docs/**' --glob '!.subdaimon-output/**' --glob '!web/gl.js' '(TODO|FIXME|HACK|todo!|unimplemented!)' crates tools scripts web` | No matches in Rust or Python source. |
| Mission targeting work | `PATH="/usr/bin:$PATH" cargo test target_character -- --nocapture` | New mission-targeting tests pass. |
| Covert foil work | `PATH="/usr/bin:$PATH" cargo test foil -- --nocapture` | New foil and counter-intelligence tests pass. |
| AI validator work | `PATH="/usr/bin:$PATH" cargo test can_dispatch -- --nocapture` | Validator tests pass, including rejected invalid dispatches. |
| AI troop movement work | `PATH="/usr/bin:$PATH" cargo test troop -- --nocapture` | AI troop-deployment tests pass. |
| Resource lookup work | `PATH="/usr/bin:$PATH" cargo test encyclopedia tactical_view fleets -- --nocapture` | Lookup-table and mapping tests pass. |
| Full core test sweep | `PATH="/usr/bin:$PATH" cargo test` | All tests pass and the count stays at or above the current baseline. |
| Desktop parity smoke test | `PATH="/usr/bin:$PATH" cargo run -p rebellion-app -- data/base` | Native app launches to the main menu with no panic. |
| Headless AI smoke test | `PATH="/usr/bin:$PATH" cargo run -p rebellion-playtest -- data/base --seed 42 --ticks 5000 --dual-ai --summary` | Campaign runs to completion and still produces non-degenerate output. |

## Validation and Acceptance

Milestone 1 is accepted when a human player can open the Missions panel in the
desktop app, choose Recruitment, Assassination, or Abduction, choose a target
character, dispatch the mission, and later see the effect land on that exact
target. This same milestone is also accepted when the AI can generate at least
one mission action with a real target character instead of dispatching only
system-targeted covert missions.

Milestone 2 is accepted when covert missions can fail because of defender
counter-intelligence, when AI dispatches are rejected for the same kinds of
capacity and composition reasons as the original, and when a long dual-AI
campaign shows ground-force movement and not just ship movement. The simplest
observable proof is a headless playtest log that contains troop relocation and
at least one foiled mission.

Milestone 3 is accepted when the encyclopedia no longer shows a gray placeholder
for star systems, tactical battles use stable lookup-based sprites instead of
heuristic mappings, and the native build plays the intro and victory or defeat
videos from the decoded WebM assets.

Milestone 4 is accepted when the advisor animation visibly follows authored
timing rather than a generic loop and when combat and Death Star code no longer
document their remaining behavior as approximations.

The project reaches "100% parity" for this plan when all eleven gap rows in the
Gap Inventory table are closed on native desktop builds with original assets
present. It does not need browser save/load, browser mod discovery, HD upscales,
or newly generated tactical 3D assets to make that claim.

## Idempotence and Recovery

Each work package should land as a separate branch or commit series. None of the
live parity items require a save-format bump if implemented carefully because the
main missing mission field, `target_character`, already exists on `ActiveMission`.
That means Milestone 1 can be retried safely without invalidating old saves.

The safest recovery path for each milestone is to revert only the branch or commit
series for that milestone and rerun the focused tests from the Concrete Steps
table. Avoid batching UI/media changes with simulation changes because the bugs
look similar from the outside but have different rollback costs.

For media work, keep graceful fallbacks until acceptance is complete. The
encyclopedia can keep its placeholder path until the real lookup is validated.
Advisor resource corruption should continue to fail closed instead of silently
substituting invented motion. Video playback should fail closed by skipping the
cutscene, not by blocking gameplay startup.

## Interfaces and Dependencies

The most important interface changes expected by this plan are:

| File | Required Interface at Completion |
|---|---|
| `crates/rebellion-render/src/panels/mod.rs` | `PanelAction::DispatchMission` carries `target_character: Option<CharacterKey>`. |
| `crates/rebellion-render/src/panels/missions.rs` | `MissionsPanelState` tracks a selected target character and renders the selector only for the mission kinds that need it. |
| `crates/rebellion-core/src/missions.rs` | `MissionState::dispatch()` and `dispatch_guarded()` accept `target_character: Option<CharacterKey>`. `ActiveMission::new()` receives that value instead of always defaulting to `None`. |
| `crates/rebellion-core/src/ai.rs` | `AIAction::DispatchMission` carries `target_character: Option<CharacterKey>`, and either a new troop-movement action or an equivalent faithful ground-force command path exists. |
| `crates/rebellion-core/src/tuning.rs` | `AiConfig` adds explicit faction-budget fields if A3 is implemented cleanly through configuration. |
| `crates/rebellion-render/src/encyclopedia.rs` | System art lookup uses parsed data from `ENCYBMAP.DLL` instead of a placeholder path. |
| `crates/rebellion-render/src/tactical_view.rs` | Tactical sprite selection uses a lookup table rather than the current linear approximation. |
| `crates/rebellion-render/src/panels/fleets.rs` | Complete in `4589d2e`: GOKRES mini-icon selection uses exact faction/record-order mappings and unknown IDs fail closed. |
| `crates/rebellion-render/src/video_player.rs` | New module for native video playback, integrated from `crates/rebellion-app/src/main.rs`. |
| `crates/rebellion-render/src/advisor.rs` | Advisor state consumes BIN-driven sequence data or a parsed equivalent table for frame ordering and timing. |

## What Counts As Parity Versus Augmentation

The following items are required for 100% original parity because the 1998 game
had them and the current codebase still lacks or approximates them:

The eleven gaps in the Gap Inventory table.

The following items are augmentation work and should not delay the parity claim:

| Area | Why It Is Augmentation, Not Original-Parity Work | Estimated LOC |
|---|---|---:|
| Browser save/load in `crates/rebellion-data/src/save.rs` | The original game was a native desktop title. Browser persistence is a remake convenience feature. | 250-400 |
| Browser mod discovery and hot reload in `crates/rebellion-data/src/mods.rs` | Original Rebellion did not have browser mods. This is remake-specific infrastructure. | 200-350 |
| HD upscaled DLL and EDATA asset rollout from `agent_docs/assets.md` | The original shipped only the low-resolution art. HD assets are a value-add pack, not a parity requirement. | 50-150 code plus asset ops |
| Generated 3D model pipeline and new sprite sheets | The original tactical art already exists in `TACTICAL.DLL`. Generating new 3D assets is modernization, not parity. | 200-400 scripts plus asset ops |
| Strict legacy-behavior mode for existing augmentations | Optional. Useful if the team wants a "purist mode," but not required for feature parity. | 120-240 |
