---
title: "UI Parity Completion ExecPlan"
type: feat
status: planning
date: 2026-03-23
project: open-rebellion
tags: [ui, parity, cockpit, tactical, audio, execplan]
---

# Ship The Remaining Original Rebellion UI Parity

This ExecPlan is a living document. Sections Progress, Surprises &
Discoveries, Decision Log, and Outcomes & Retrospective must be
kept up to date as work proceeds.

## Purpose / Big Picture

The goal is to replace the last obvious debug-era presentation seams so Open Rebellion feels like Star Wars Rebellion (1998) during normal play, not just in isolated screens. "Parity" in this document means matching what a player can see, click, hear, and understand: cockpit frame, cockpit buttons, galaxy-map facility and blockade overlays, tactical battles that pause the strategy layer and become playable, faction music and voice lines, and final art polish with original or HD-upscaled assets.

At the end of this plan, a player should be able to run the native app, start either faction, see the correct faction cockpit chrome, use cockpit buttons instead of relying on keyboard shortcuts, watch facility and blockade markers stay aligned on the map, enter a real tactical combat screen when the player is involved in a fleet battle, hear soundtrack and voice cues, and see portrait and ship art that prefers HD assets when present but still falls back to the original extracted BMP files.

## Progress

- [x] (2026-03-24 03:25Z) Read `docs/plans/2026-03-22-001-feat-complete-game-ui-rebuild-plan.md`, `agent_docs/roadmap.md`, `agent_docs/architecture.md`, `agent_docs/assets.md`, `agent_docs/dll-resource-catalog.md`, and `agent_docs/game-media.md`.
- [x] (2026-03-24 03:25Z) Inspected the current app loop, render modules, panel modules, asset folders, and build scripts to confirm what is already shipped versus still missing.
- [x] (2026-03-24 03:25Z) Verified the current baseline compiles with `PATH=/usr/bin:$PATH cargo check`.
- [ ] Build a staged runtime asset pipeline and shared BMP/PNG/WAV loader that both native and WASM can use.
- [ ] Ship Phase 2.1-2.2 cockpit frame and cockpit control buttons with full click routing.
- [ ] Ship Phase 3.1, 3.2, and 3.5 galaxy-map overlays: facility status icons, sector boundaries, and blockade markers.
- [ ] Add a tactical combat game mode and battle session state so player-involved combat no longer resolves only inside the galaxy loop.
- [ ] Finish the tactical combat view: placement, per-tick combat, HUD, orders, results, and ground-combat handoff.
- [ ] Integrate soundtrack, voice lines, and advisor-driven audio and animation.
- [ ] Apply final visual polish: HD fallback chain, portraits, ship sprites, and screenshot-driven layout cleanup.

## Surprises & Discoveries

Observation: the roadmap still lists Phase 4.4 through 4.6 as remaining work, but the tree already contains `crates/rebellion-render/src/panels/bombardment.rs`, `crates/rebellion-render/src/panels/death_star.rs`, and `crates/rebellion-render/src/panels/loyalty.rs`, and `crates/rebellion-app/src/main.rs` already wires them to the `B`, `D`, and `L` keys. Evidence: the modules exist, the imports are live, and the main egui pass calls `draw_bombardment`, `draw_death_star`, and `draw_loyalty`. The practical task here is a parity audit, not a greenfield feature build.

Observation: the tactical combat gap is larger than the earlier roadmap wording suggests. Evidence: `crates/rebellion-app/src/main.rs` only defines `GameMode::MainMenu`, `GameMode::GameSetup`, and `GameMode::Galaxy`, while `crates/rebellion-render/src/combat_view.rs` is only a battle report modal. Player-involved battles still resolve inside the galaxy tick loop by calling `CombatSystem::resolve_space` and `CombatSystem::resolve_ground` immediately.

Observation: a BMP decode and texture-cache pattern already exists and should be reused instead of reimplemented. Evidence: `crates/rebellion-render/src/encyclopedia.rs` already decodes BMP and PNG files with the `image` crate, registers egui textures, and falls back from HD PNG to original BMP.

Observation: the current WASM build stages DAT files and `textstra.json`, but no cockpit, icon, tactical, portrait, or audio assets. Evidence: `scripts/build-wasm.sh` copies `data/base/*.DAT`, `data/base/*.DLL`, and `textstra.json` into `web/data/base`, but never stages files from `assets/references/ref-ui-full/`, `assets/references/ref-ui/`, or any sound directory. Any new asset path that only works on native will immediately reopen the browser gap.

Observation: the working tree is already dirty and contains unrelated modified and untracked files. Evidence: `git status --short` reported existing changes in `Cargo.toml`, `web/open-rebellion.wasm`, and several untracked asset folders. Recovery instructions in this plan must avoid destructive cleanup.

## Decision Log

Decision: start with a staged runtime asset layer before touching cockpit, galaxy overlay, tactical, or audio features. Rationale: every requested remaining phase depends on loading extracted BMP or WAV files, and the project already supports both native and WASM. A one-off native loader would create fast local progress but slow the full parity finish. Date/Author: 2026-03-24 / Codex.

Decision: treat `assets/references/ref-ui-full/` and `assets/references/ref-ui/` as immutable source material and copy only the needed subset into runtime-visible folders with stable logical names. "Staging" here means copying derived runtime files into a directory the app and the browser build can both read, while preserving the original extraction folders as source-of-truth references. Rationale: the extracted folders are excellent references but poor runtime contracts because names vary by DLL and are not staged for WASM. Date/Author: 2026-03-24 / Codex.

Decision: split tactical combat into a foundation milestone and a completion milestone. Rationale: adding a battle mode, pause boundary, and persistent battle session is a different risk from adding per-tick combat, orders, and HUD art. If these are merged into one large step, there is no safe checkpoint where player combat stops auto-resolving but the strategy game still works. Date/Author: 2026-03-24 / Codex.

Decision: defer HD art replacement until the original-art path is functionally correct. Rationale: logic bugs are much easier to spot while every sprite and button still matches the original resource IDs. Once mappings and hit boxes are stable, HD PNG overrides can be added without changing behavior. Date/Author: 2026-03-24 / Codex.

Decision: keep existing keyboard shortcuts as a fallback until cockpit buttons fully cover the same actions. Rationale: this preserves developer velocity during the transition and makes it easier to verify that a broken button is a UI bug rather than a missing game action. Date/Author: 2026-03-24 / Codex.

## Outcomes & Retrospective

Planning is complete. The repository now has a dependency-ordered execution plan that starts with the real blocker, which is asset runtime plumbing, then moves through cockpit parity, galaxy overlays, tactical combat, audio and advisors, and only then visual polish. The main remaining implementation risk is tactical combat scope, because it crosses `rebellion-app`, `rebellion-render`, and `rebellion-core` at once.

## Context and Orientation

Open Rebellion is a Rust workspace with separate crates for game state, rendering, and application orchestration. `crates/rebellion-core` owns simulation rules and must stay deterministic. `crates/rebellion-render` owns macroquad and egui drawing. `crates/rebellion-app` owns the top-level loop, mode switching, asset path setup, and audio engine.

The current strategy UI is partly rebuilt. `crates/rebellion-app/src/main.rs` already has a main menu, game setup, theme application, message log, galaxy map draw order, panel visibility flags, keyboard shortcuts, and audio startup. `crates/rebellion-render/src/lib.rs` already renders the galaxy map, system detail panel, system and fleet context menus, and the bottom status bar. `crates/rebellion-render/src/encyclopedia.rs` already proves that BMP and PNG decoding works in the render crate. `crates/rebellion-app/src/audio.rs` already proves that `quad-snd` is the chosen audio backend and can play looped music plus one-shot sound effects.

The asset source-of-truth already exists. `agent_docs/dll-resource-catalog.md` documents the DLL ranges, counts, and meanings. The extracted files live under `assets/references/ref-ui-full/` and curated subsets live under `assets/references/ref-ui/`. The key folders for this plan are `assets/references/ref-ui-full/common-dll/`, `assets/references/ref-ui-full/strategy-dll/`, `assets/references/ref-ui-full/tactical-dll/`, `assets/references/ref-ui-full/voice-alliance/`, and `assets/references/ref-ui-full/voice-empire/`. Decoded videos already live under `assets/references/ref-videos/`.

The browser path matters. `scripts/build-wasm.sh` currently stages only `data/base` content into `web/data/base`. The native app can read local files directly, but the browser build can only fetch files that have been copied into `web/`. This is why the first milestone explicitly includes asset staging and not just runtime decoding.

Key files to read before implementation are:

- `docs/plans/2026-03-22-001-feat-complete-game-ui-rebuild-plan.md`
- `agent_docs/roadmap.md`
- `agent_docs/architecture.md`
- `agent_docs/dll-resource-catalog.md`
- `agent_docs/assets.md`
- `agent_docs/game-media.md`
- `crates/rebellion-app/src/main.rs`
- `crates/rebellion-app/src/audio.rs`
- `crates/rebellion-render/src/lib.rs`
- `crates/rebellion-render/src/encyclopedia.rs`
- `crates/rebellion-render/src/combat_view.rs`
- `crates/rebellion-render/src/panels/bombardment.rs`
- `crates/rebellion-render/src/panels/death_star.rs`
- `crates/rebellion-render/src/panels/loyalty.rs`
- `scripts/build-wasm.sh`

## Plan of Work

Milestone 1 is the asset runtime foundation. Add one shared module in `crates/rebellion-render/src/` that can decode original BMP files and optional HD PNG overrides, cache textures, and expose simple lookup calls by logical asset name. Add one staging script in `scripts/` that copies the exact cockpit, cockpit-button, facility-icon, tactical-HUD, portrait, ship-sprite, soundtrack, and voice-line files needed for runtime into stable directories such as `data/base/ui/`, `data/sounds/`, and `web/data/ui/`. Extend `scripts/build-wasm.sh` so the same staged asset set reaches `web/`. Do not let each new feature open files from `assets/references/` on its own.

Milestone 2 is cockpit parity. Create `crates/rebellion-render/src/cockpit.rs` and move the cockpit frame, button art, button hit boxes, and button state drawing there. Update `crates/rebellion-render/src/lib.rs` so the galaxy map renders inside a clipped viewport defined by the cockpit frame rather than filling the entire screen behind a bottom egui status bar. Update `crates/rebellion-app/src/main.rs` so cockpit buttons drive the same state changes that the current keyboard shortcuts drive: officers, fleets, manufacturing, missions, research, encyclopedia, save or load, and game speed. During this milestone, audit the already-present bombardment, Death Star, and loyalty panels and close any parity holes that become obvious once the cockpit buttons are the main entry point.

Milestone 3 is galaxy overlay parity. Add a dedicated overlay module, either by extracting a new `crates/rebellion-render/src/galaxy_overlays.rs` or by clearly partitioning new functions inside `crates/rebellion-render/src/lib.rs`. Render facility status icons from the staged STRATEGY.DLL assets, translucent sector boundary polygons derived from each sector's system positions, and blockade indicators tied to `BlockadeState`. If the render layer cannot distinguish mine versus refinery or shipyard versus advanced shipyard from current runtime data, promote the missing facility-class metadata into `GameWorld` through `crates/rebellion-core/src/world/mod.rs` and `crates/rebellion-data/src/lib.rs` instead of relying on brittle filename or string matching.

Milestone 4 is tactical mode foundation. Extend `GameMode` in `crates/rebellion-app/src/main.rs` to include at least `TacticalCombat`, and add a battle session type that holds the fleets involved, the system, placement state, selected units, timing, and a snapshot of combat data that survives across frames. Create `crates/rebellion-render/src/tactical_view.rs` for battlefield drawing and input. Change combat routing in `crates/rebellion-app/src/main.rs` so AI-versus-AI battles can still auto-resolve in the galaxy loop, but any battle involving the player enters the tactical mode first instead of being immediately resolved.

Milestone 5 is tactical combat completion. Extend `crates/rebellion-core/src/combat.rs` with a step-wise combat API that can advance one tactical slice at a time while keeping deterministic rules. Use TACTICAL.DLL HUD and ship assets in `crates/rebellion-render/src/tactical_view.rs` to implement ship placement, focus-fire targeting, retreat, fighter launch and recall, weapon and shield feedback, and combat results. Reuse `crates/rebellion-render/src/combat_view.rs` as the post-battle report surface instead of discarding it. Finish this milestone only when battle results are applied back into `GameWorld` and the player can return to `GameMode::Galaxy` with the correct fleet and troop losses.

Milestone 6 is audio and advisor integration. Expand `crates/rebellion-app/src/audio.rs` so it can load the staged 16 soundtrack WAV files, 285 voice lines, and any retained UI sound effects from the staged sound directories instead of the current placeholder naming scheme. Add `crates/rebellion-render/src/advisor.rs` for animated advisor frames and BIN-driven sequence playback, then wire advisor and voice triggers from `crates/rebellion-app/src/main.rs` based on major game events, mission results, combat results, and cockpit context. This is where Phase 2.3 belongs, because the advisor is both a visual system and an audio system.

Milestone 7 is final visual polish. Add HD override lookup for cockpit art, buttons, portraits, tactical sprites, and ship panels using the same fallback model already proven by `crates/rebellion-render/src/encyclopedia.rs`: prefer HD PNG, then original BMP. Keep the original resource-ID mapping stable while upscaled art is produced in `data/hd/`. Use the curated asset counts in `agent_docs/assets.md` to batch work in a controlled order: cockpit and buttons first, portraits next, then tactical ship sprites. Finish with a screenshot sweep on Alliance and Empire campaigns plus one tactical battle.

## Concrete Steps

1. Confirm the baseline before touching code.

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
PATH=/usr/bin:$PATH cargo check
```

Expected output: the workspace finishes `cargo check` without errors. Warnings are acceptable at the start of the plan, but new milestones should not introduce errors.

2. Confirm the source asset inventory that the staging script will consume.

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
find assets/references/ref-ui-full/common-dll -maxdepth 1 -type f | wc -l
find assets/references/ref-ui-full/strategy-dll -maxdepth 1 -type f | wc -l
find assets/references/ref-ui-full/tactical-dll -maxdepth 1 -type f | wc -l
find assets/references/ref-ui-full/voice-alliance -maxdepth 1 -type f | wc -l
find assets/references/ref-ui-full/voice-empire -maxdepth 1 -type f | wc -l
```

Expected output: counts close to the documented inventory, especially 321 COMMON BMPs, 1042 STRATEGY BMPs, 288 TACTICAL BMPs, 153 Alliance voice WAV files, and 132 Empire voice WAV files.

3. Add and run the asset staging command after Milestone 1 lands.

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
python3 scripts/stage-ui-assets.py --target native --target web
```

Expected output: a staged runtime asset tree appears under `data/base/ui/`, `data/sounds/`, and `web/data/ui/` without modifying the source extraction folders under `assets/references/`.

4. Verify cockpit parity in the native app after Milestone 2 lands.

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
PATH=/usr/bin:$PATH cargo run -p rebellion-app --bin open-rebellion -- data/base
```

Expected output: after starting a new game, the galaxy map is framed by the correct faction cockpit and the player can open the major panels and change speed with clickable cockpit buttons.

5. Verify the browser staging path after Milestone 1 and Milestone 2 land.

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
bash scripts/build-wasm.sh
python3 -m http.server 8080 -d web
```

Expected output: `web/open-rebellion.wasm` rebuilds, `web/data/` contains the staged UI assets, and a browser session can load the same cockpit art and buttons.

6. Use the existing debug command palette to force tactical and overlay scenarios during development.

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
PATH=/usr/bin:$PATH cargo run -p rebellion-app --bin open-rebellion -- data/base
```

Expected output: in a debug build, pressing the backtick key opens the command palette from `crates/rebellion-render/src/panels/command_palette.rs`, which can advance ticks, reveal all systems, and toggle dual AI to force map events and combat situations faster than waiting through a full campaign.

7. Keep the combat rules green while tactical mode is added.

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
PATH=/usr/bin:$PATH cargo test -p rebellion-core combat
```

Expected output: combat unit tests pass before and after adding any step-wise tactical combat API.

8. Re-run the full compile gate at the end of every milestone.

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
PATH=/usr/bin:$PATH cargo check
PATH=/usr/bin:$PATH cargo build --target wasm32-unknown-unknown -p rebellion-app --release
```

Expected output: both native and WASM builds succeed, proving the new asset and mode work did not silently become native-only.

## Validation and Acceptance

Acceptance for Milestone 1 is that one cockpit BMP, one cockpit button state, one facility icon, one tactical HUD panel, and one soundtrack file can be loaded through the new staged runtime path on native, and the same staged files are present under `web/data/` after running `scripts/build-wasm.sh`.

Acceptance for Milestone 2 is that both factions render distinct cockpit chrome, the bottom debug-like status bar is no longer the primary control surface, and the player can click cockpit buttons to open officers, fleets, manufacturing, missions, research, encyclopedia, save or load, and speed controls without using the keyboard.

Acceptance for Milestone 3 is that systems with facilities show the correct icon family, blockaded systems display a clear blockade marker, and sector boundaries remain visually locked to the correct region while the camera pans and zooms. A novice tester must be able to tell, by sight alone, which systems are industrial and which are blockaded.

Acceptance for Milestone 4 is that a player-involved fleet battle no longer resolves only in the galaxy tick loop. Instead, the app enters `TacticalCombat`, pauses the strategy view, and shows a battle screen with deployment space and visible ships.

Acceptance for Milestone 5 is that the player can place ships, issue at least focus-fire and retreat orders, watch combat advance in visible steps, receive a readable battle report, and return to the galaxy with fleet losses correctly applied. `PATH=/usr/bin:$PATH cargo test -p rebellion-core combat` must still pass.

Acceptance for Milestone 6 is that a fresh campaign plays background music, cockpit interactions produce audible feedback where appropriate, and at least one mission or combat event triggers a faction voice line and advisor animation without crashing or desynchronizing the main loop.

Acceptance for Milestone 7 is that the app prefers HD PNG overrides when present, still works when `data/hd/` is missing, and produces a final screenshot set that shows Alliance cockpit, Empire cockpit, galaxy overlays, one tactical battle, and one portrait-heavy panel all using the intended art path.

The final release acceptance gate is one full manual run with `PATH=/usr/bin:$PATH cargo run -p rebellion-app --bin open-rebellion -- data/base`, one WASM rebuild with `bash scripts/build-wasm.sh`, and one tactical-battle smoke test in each faction. No milestone is complete until all three pass.

## Idempotence and Recovery

The code-editing steps in this plan are safe to repeat. `cargo check`, `cargo test -p rebellion-core combat`, and `bash scripts/build-wasm.sh` are already idempotent. The new staging script must also be idempotent: rerunning it should overwrite generated runtime assets in place without mutating the original extracted files under `assets/references/`.

Recovery for generated assets should avoid destructive Git commands. If staged UI or sound files are wrong, delete only the generated output folders such as `data/base/ui/`, `data/sounds/`, `web/data/ui/`, or a specific `data/hd/` subfolder, then rerun the staging command. If a code milestone breaks the build, use `git diff` and `git restore --source=HEAD -- <specific file>` only on files changed by that milestone. Do not use `git reset --hard`, because the worktree already contains unrelated user changes.

If the tactical combat work becomes unstable, recover by keeping Milestone 4's battle-session entry and exit code behind a feature flag or temporary branch-local toggle while `CombatSystem::step()` is still incomplete. That gives a safe checkpoint where strategy mode is intact and the session shell exists without forcing a broken battle loop on every player combat.

## Interfaces and Dependencies

The implementation must continue using `macroquad 0.4`, `egui-macroquad 0.17`, `image 0.25`, and `quad-snd 0.2`. No new rendering or audio framework is needed for this plan.

The render crate needs one shared asset-cache interface, whether it is named `UiAssetCache`, `DllAssetCache`, or something similar. It must support original BMP decode, optional HD PNG override, texture caching, and a browser-friendly staged asset path. The interface should be used by cockpit art, facility icons, tactical HUD art, portraits, and ship sprites so that all art follows the same rules.

`crates/rebellion-app/src/main.rs` must grow a richer `GameMode` and a persistent tactical battle state. A minimal end-state interface is a `GameMode` enum that includes `TacticalCombat`, plus a battle-session struct that records system, fleets, selection state, placement state, and battle tick timing.

`crates/rebellion-core/src/combat.rs` needs a step-wise combat API in addition to the current one-shot resolution path. The exact name may vary, but the end-state contract must allow the app to ask combat to advance one visible slice at a time and then apply the returned events back into `GameWorld` only when the battle resolves.

The facility overlay work may require new runtime metadata tables in `crates/rebellion-core/src/world/mod.rs` and `crates/rebellion-data/src/lib.rs`, especially for manufacturing and production facility classes. If the renderer cannot know which icon to draw from current instance data alone, promote that data into `GameWorld` rather than teaching the renderer to guess.

`crates/rebellion-app/src/audio.rs` must keep `quad-snd` as the playback layer, but it will need broader track and clip indexing than the current placeholder `MusicTrack` and `SfxKind` mapping. The end state must support soundtrack selection by context, one-shot UI and event sounds, and faction voice-line lookup.
