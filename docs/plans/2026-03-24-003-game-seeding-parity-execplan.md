# Fix Initial Game Seeding Parity With Star Wars Rebellion 1998

This ExecPlan is a living document. Sections Progress, Surprises &
Discoveries, Decision Log, and Outcomes & Retrospective must be
kept up to date as work proceeds.

## Purpose / Big Picture

Open Rebellion currently starts a new campaign from static DAT table placement. The original 1998 game starts from a procedural seeding pipeline that rolls character stats, assigns fixed named personnel to special systems, randomizes support and political control, generates a separate Rebel headquarters in the rim, computes planetary energy and raw materials, and spends maintenance budget to create most starting forces. This plan restores that startup behavior in dependency order so a new game looks and plays like the original instead of starting from placeholder `0.0` popularity and proximity-based fleet spread.

Someone following this plan should be able to start a new game and observe the major parity outcomes immediately. Luke and Leia should begin on Yavin, Vader and Palpatine should begin on Coruscant, the Rebel headquarters should be a separate random rim system instead of Yavin, controlled systems should start with non-zero loyalty values, and starting fleets from the fixed seed tables should exist only at Coruscant, Yavin, and the Rebel headquarters while the larger starting force pool comes from maintenance-budget unit seeding. Proof comes from targeted integration tests, save/load compatibility checks, and a green `PATH=/usr/bin:$PATH cargo check`.

## Progress

- [x] (2026-03-24 16:43Z) Read the required audit, roadmap, reverse-engineering notes, world model, seed loader, game setup flow, save format, and parameter dumps.
- [x] (2026-03-24 16:43Z) Verified the current baseline with `PATH=/usr/bin:$PATH cargo check`; the workspace builds successfully with pre-existing warnings only.
- [x] (2026-03-24 16:43Z) Confirmed that the seeding-critical parameters cited in the audit exist in local dumps: `data/base/json/GNPRTB.json` and `data/base/json/SDPRTB.json`.
- [ ] Milestone 1: Thread seeding inputs and side-parameter data into the loader before any seed logic runs.
- [ ] Milestone 2: Add the missing persistent system fields needed by seeding and decide the save-version strategy before changing serialized world layout.
- [ ] Milestone 3: Replace proximity-based special-system seeding with an explicit special-system pipeline for Coruscant, Yavin, and a randomized Rebel headquarters.
- [ ] Milestone 4: Roll character stats and place fixed named personnel after special systems exist.
- [ ] Milestone 5: Implement procedural control buckets, population state, and support initialization from GNPRTB and SDPRTB inputs.
- [ ] Milestone 6: Implement energy, raw materials, and procedural facility generation.
- [ ] Milestone 7: Implement maintenance-budget common-unit seeding and low-support garrison troops.
- [ ] Milestone 8: Finish difficulty and galaxy-size integration, add regression tests, and verify build/save behavior end to end.

## Surprises & Discoveries

- Observation: `crates/rebellion-data/src/lib.rs` currently calls `seeds::apply_seeds()` before loading `GNPRTB.DAT`, so the seed pipeline cannot legally use balance parameters yet.
  Evidence: `crates/rebellion-data/src/lib.rs` seeds in section 7, then loads GNPRTB in section 8.

- Observation: the parameters the audit treats as “GNPRTB seeding params” are split across two tables in this repo. General scalar seeding values such as `7721`, `7722`, `7730`, `7761`, and `7766` are present in `data/base/json/GNPRTB.json`, while side-sensitive and side-plus-difficulty values such as `5168`, `5169`, `5170`, `7680`, `7681`, `7682`, `7683`, `7684`, and `7685` are present in `data/base/json/SDPRTB.json`.
  Evidence: `tools/dat-dumper/src/types/side_params.rs` defines `SDPRTB.DAT`; the JSON dumps include those exact parameter ids.

- Observation: the current seed implementation hardcodes the Rebel headquarters to Yavin and spreads the fixed fleet tables across “nearest systems” instead of using the original three-system special-case model.
  Evidence: `crates/rebellion-data/src/seeds.rs` uses `select_starting_systems()` and routes `CMUNHQTB` and `FACLHQTB` to `YAVIN_SEQ_ID`.

- Observation: game setup choices already exist in the UI but never reach seeding.
  Evidence: `crates/rebellion-render/src/panels/game_setup.rs` exposes `GalaxySize` and `Difficulty`, but `crates/rebellion-app/src/main.rs` still calls `rebellion_data::load_game_data(&gdata_path)` with no seeding context.

- Observation: adding serialized fields to `System` is not a free backward-compatible change in the current save system because saves are bincode snapshots of `GameWorld`.
  Evidence: `crates/rebellion-data/src/save.rs` documents versioned bincode saves and already rejects old layouts when `Character` changed in v3.

- Observation: there is no Firecrawl connector in this Codex session, and direct scraping of the GitHub wiki folder was not available from the provided tools.
  Evidence: MCP resource discovery returned no connectors; the plan therefore relies on the local audit that already cites the TheArchitect2018 seeding wiki plus repo-local parameter dumps and code.

## Decision Log

- Decision: treat `SDPRTB.DAT` as a first-class runtime input for seeding rather than trying to force all seeding values through `GnprtbParams`.
  Rationale: the strong and weak bucket percentages and maintenance-budget percentages are side-sensitive, and the repo already ships a parser for `SDPRTB.DAT`.
  Date/Author: 2026-03-24 / Codex

- Decision: keep the existing dual-popularity world model for now and seed it from the original game’s single 0-100 support value instead of replacing the entire popularity model in this task.
  Rationale: missions, uprising, UI, and manufacturing already read `popularity_alliance` and `popularity_empire`. A seeding helper can map original support into the current representation without widening this task into a full diplomacy-model rewrite.
  Date/Author: 2026-03-24 / Codex

- Decision: after rolling character stats, overwrite each `SkillPair` with the rolled concrete value in `base` and set `variance` to `0`.
  Rationale: this preserves existing gameplay call sites that already read `SkillPair.base`, avoids a large parallel “rolled stat” API, and still gives each new game a concrete start-state.
  Date/Author: 2026-03-24 / Codex

- Decision: introduce a dedicated seeding context object and deterministic RNG injection point even though the original game used non-deterministic random helpers.
  Rationale: parity code needs stable tests. The runtime path can still pick a fresh seed, while tests can pass a fixed seed and assert exact placements and bucket counts.
  Date/Author: 2026-03-24 / Codex

- Decision: plan for a save-version bump when `System` gains persisted seeding fields unless the implementer proves a safe migration path with fixture coverage.
  Rationale: the current save system serializes `GameWorld` with bincode, and previous struct-shape changes already broke compatibility.
  Date/Author: 2026-03-24 / Codex

## Outcomes & Retrospective

Pending implementation. The intended outcome is a dependency-ordered seeding rewrite that touches the smallest necessary surface area: `rebellion-core` for persistent world and parameter types, `rebellion-data` for data loading and seeding logic, and the game entry points that must pass galaxy size and difficulty. The main risk is not the formulas themselves but the interface seams: parameter ownership, save compatibility, and keeping old gameplay systems working while the seed model becomes more faithful.

## Context and Orientation

“Seeding” means the one-time creation of a brand new game state before the first turn. “Support” means the original game’s single loyalty value from `0` to `100` for a system. “Popularity” is Open Rebellion’s current dual-float representation, one float per faction. “Bucket” means one procedural control category used when assigning core systems: Alliance strong, Alliance weak, Empire strong, Empire weak, or neutral. “Special systems” means Coruscant, Yavin, and the randomized Rebel headquarters. “Common-unit seeding” means the budget-driven random force generation loop that spends starting maintenance allowance on unit bundles from the common seed tables.

The current implementation is split across two crates. `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-core/src/world/mod.rs` defines `System`, `Character`, `Fleet`, `GameWorld`, and `GnprtbParams`. `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-data/src/lib.rs` builds the world, loads DAT files, applies seeds, derives control from seeded assets, and back-fills character locations from fleets. `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-data/src/seeds.rs` currently contains a table dispatcher, not a procedural seed pipeline. `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-app/src/main.rs` and `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-playtest/src/main.rs` are the important callers because they create new worlds. `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-data/src/save.rs` owns save-version policy and must be considered before any serialized world layout change.

The reverse-engineering and gap-analysis sources are already local. `/Users/tomdimino/Desktop/Programming/open-rebellion/.subdaimon-output/seeding-parity-audit.md` is the main gap list and priority ranking. `/Users/tomdimino/Desktop/Programming/open-rebellion/agent_docs/roadmap.md` repeats the high-priority missing behaviors. `/Users/tomdimino/Desktop/Programming/open-rebellion/agent_docs/ghidra-re.md` explains the parameter-table model. `/Users/tomdimino/Desktop/Programming/open-rebellion/data/base/json/GNPRTB.json` and `/Users/tomdimino/Desktop/Programming/open-rebellion/data/base/json/SDPRTB.json` are convenient ground-truth dumps for parameter values during implementation and tests.

The current code has three especially important constraints. First, `load_game_data()` seeds before parameter loading, so ordering must change. Second, `System` has no persistent `total_energy` or `raw_materials` fields yet. Third, the current UI and simulation expect dual popularity floats, so seeding should adapt the original support rules into that representation rather than trying to replace it during this task.

## Plan of Work

Milestone 1 adds a proper seeding context and loads the right parameter tables before any seed logic runs. Update `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-core/src/world/mod.rs` with a side-parameter runtime type, likely parallel to `GnprtbParams`, and give it an accessor that resolves one parameter id by difficulty and faction side. Update `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-data/src/lib.rs` so `GNPRTB.DAT` and `SDPRTB.DAT` are loaded before seeding, and so seeding receives a `SeedOptions` or `NewGameSeedConfig` value containing `GalaxySize`, `Difficulty`, player faction, and an RNG seed. Keep a wrapper that preserves existing call sites during the migration, but make the new path the one that the app and playtest binaries use. This milestone is the prerequisite for every other milestone.

Milestone 2 extends the persistent world model. Update `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-core/src/world/mod.rs` so `System` gains the missing seeding fields needed later in the pipeline, at minimum `is_populated`, `total_energy`, and `raw_materials`. If the implementation needs a compact helper field for original support, keep it internal or derive it, because the existing systems should continue to read dual popularity floats. At the same time, update `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-data/src/save.rs` and its fixtures so the save format has an explicit policy before the first serialized layout change lands. The clean path is a `SAVE_VERSION` bump plus either a v4 migration shim or an intentional clear rejection message for old saves.

Milestone 3 replaces the current special-system shortcuts with an explicit special-system phase inside `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-data/src/seeds.rs`. Remove the proximity model from `select_starting_systems()` for the fixed tables. Instead, shuffle the eligible systems, choose a Rebel headquarters from the rim that is not Yavin, mark Coruscant, Yavin, and HQ as populated, charted, controlled, and `100` support, and route fixed seed tables accordingly: `CMUNEFTB` to Coruscant, `CMUNYVTB` to Yavin, `CMUNHQTB` and `FACLHQTB` to the randomized HQ, and `FACLCRTB` to Coruscant. This milestone also fixes priority item eight because fixed fleets should now live only at the three special systems.

Milestone 4 makes characters usable at game start. Still in `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-data/src/seeds.rs`, add a character setup pass that runs after special systems exist and before gameplay systems look at the characters. Implement the original roll rules against the loaded major and minor character tables, including Jedi chance, force-level roll, and Empire leadership bonus where applicable. After the roll, normalize each `SkillPair` to a concrete `base` with `variance = 0`. Then place fixed named personnel by setting `current_system` and `current_fleet` directly: Luke, Leia, Han, Wedge, Chewbacca, and Jan Dodonna at Yavin; Mon Mothma at the randomized HQ; Palpatine and, per the requested priority, Vader at Coruscant unless later wiki evidence proves Vader belongs in the random Empire main-list phase instead. If the wiki confirms Vader belongs in the random personnel step, keep that as a documented exception and leave the fixed-placement test for Palpatine only.

Milestone 5 implements procedural political control, population, and support. Use sector group, not exploration status, to classify systems as core or rim. Count core systems, compute strong and weak bucket percentages from `SDPRTB` parameters `7680` and `7681`, subtract Coruscant from the Empire strong bucket, shuffle the system order, and drain the buckets into actual systems. Once ownership exists, initialize population status from `GNPRTB` `7730` for core and `7731` for rim. Then initialize support by mapping original formulas into existing popularity floats: side-controlled core systems use `SDPRTB` `7682` through `7685`, neutral core systems use `GNPRTB` `7764`, rim populated systems use `GNPRTB` `7765`, and unpopulated systems land at neutral `50`. The helper that writes support should also set `control` consistently so later systems do not depend on the current post-hoc “asset ownership implies control” approximation.

Milestone 6 adds economy-aware planetary seeding. With `total_energy` and `raw_materials` now available on `System`, compute core and rim resource values from `GNPRTB` `7721` through `7727`, clamp to the original limits, and cap raw materials by energy where required. Then add procedural facility generation using `SYFCCRTB.DAT` and `SYFCRMTB.DAT`, which are already documented in `/Users/tomdimino/Desktop/Programming/open-rebellion/agent_docs/dat-formats.md` but not loaded by the current seed code. The facility loop should iterate once per energy slot, decide mine versus non-mine via `GNPRTB` `7766` and `7767`, and place weighted facilities until the slot budget is exhausted. Post-adjust energy and raw values so the generated facilities are legal according to the original rules.

Milestone 7 implements the force mass that the current game is missing. Start with maintenance-budget common-unit seeding using `SDPRTB` `5168`, `5169`, and `5170` for standard, large, and huge galaxy sizes. Compute existing maintenance cost from the fixed fleets, garrisons, and facilities already placed, derive available budget per side, choose random eligible owned systems, and repeatedly roll bundle groups from `CMUNALTB` and `CMUNEMTB` until the budget is exhausted or no legal placement remains. Ship bundles should create or join fleets; ground bundles should go to system inventory. After that, add the low-support garrison pass using `GNPRTB` `7761` and `7762`, so controlled systems below the threshold receive the correct number of uprising-prevention troops.

Milestone 8 finishes integration and regression coverage. Update `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-app/src/main.rs` and `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-playtest/src/main.rs` so new-game setup values reach the loader, and so tests can construct deterministic seed contexts without UI. Replace or narrow the existing post-hoc control-derivation block in `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-data/src/lib.rs`, because control should now come from the procedural seed pipeline rather than being rediscovered from placed assets. Then add unit and integration tests in `rebellion-core`, `rebellion-data`, and save-load fixtures to lock down each parity behavior listed in the priority order.

## Concrete Steps

Work from `/Users/tomdimino/Desktop/Programming/open-rebellion`.

First, confirm the parameter ownership and current code paths before editing:

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
rg -n "load_game_data|apply_seeds|select_starting_systems|current_system|current_fleet" crates/rebellion-data crates/rebellion-app crates/rebellion-playtest
rg -n '"parameter_id": (5129|5168|5169|5170|7680|7681|7682|7683|7684|7685|7721|7722|7723|7724|7725|7726|7727|7730|7731|7761|7762|7764|7765|7766|7767)' data/base/json/GNPRTB.json data/base/json/SDPRTB.json
```

Expected output is a short list of hits in `crates/rebellion-data/src/lib.rs`, `crates/rebellion-data/src/seeds.rs`, `crates/rebellion-app/src/main.rs`, and the JSON dumps for every seeding parameter listed above.

Next, implement Milestone 1 and immediately verify the new loader path still compiles:

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
PATH=/usr/bin:$PATH cargo check
```

Expected output is `Finished 'dev' profile` with zero new errors. Existing unrelated warnings may remain until the branch is cleaned separately.

After Milestone 2, prove the save policy explicitly. If the branch chooses a version bump, the command below should show updated fixture coverage and either a migration path or an intentional rejection message:

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
PATH=/usr/bin:$PATH cargo test -p rebellion-data save::tests::round_trip_save_load -- --exact
PATH=/usr/bin:$PATH cargo test -p rebellion-data save::tests::v4_or_v5_compatibility_path -- --exact
```

Expected output is two passing tests. The second test name can differ, but it must prove the chosen compatibility policy, not merely compile.

After Milestones 3 through 5, add and run deterministic seeding tests:

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
PATH=/usr/bin:$PATH cargo test -p rebellion-data seeds::tests::special_systems_match_original_roles -- --exact --nocapture
PATH=/usr/bin:$PATH cargo test -p rebellion-data seeds::tests::fixed_named_characters_spawn_at_expected_systems -- --exact --nocapture
PATH=/usr/bin:$PATH cargo test -p rebellion-data seeds::tests::support_ranges_match_original_rules -- --exact --nocapture
PATH=/usr/bin:$PATH cargo test -p rebellion-data seeds::tests::core_bucket_counts_follow_sdprtb_percentages -- --exact --nocapture
```

Expected output is all tests passing under a fixed RNG seed. The test bodies should print enough context on failure to show which system or bucket violated parity.

After Milestones 6 and 7, verify economy and force generation:

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
PATH=/usr/bin:$PATH cargo test -p rebellion-data seeds::tests::energy_and_raw_materials_respect_param_ranges -- --exact --nocapture
PATH=/usr/bin:$PATH cargo test -p rebellion-data seeds::tests::maintenance_budget_seeding_spends_without_overshoot -- --exact --nocapture
PATH=/usr/bin:$PATH cargo test -p rebellion-data seeds::tests::fixed_fleet_tables_only_target_special_systems -- --exact --nocapture
```

Expected output is passing tests that show the new starting forces come from the budget loop rather than the old proximity spread.

Finish with a full workspace verification pass:

```bash
cd /Users/tomdimino/Desktop/Programming/open-rebellion
PATH=/usr/bin:$PATH cargo test
PATH=/usr/bin:$PATH cargo check
```

Expected output is a green test suite and a green final `cargo check`.

## Validation and Acceptance

Acceptance for character seeding is that a deterministic new-game seed places Luke and Leia on Yavin, Palpatine on Coruscant, Mon Mothma on the randomized Rebel headquarters, and resolves the Vader rule in whichever way the final wiki-backed implementation chooses. The important thing is that the rule is explicit, covered by a regression test, and no named starting character is left with `current_system = None` unless they are intentionally fleet-only.

Acceptance for support and procedural control is that no system starts at `(0.0, 0.0)` popularity unless it is intentionally unpopulated and neutral under the new mapping. Controlled core systems must land in the expected ranges derived from `SDPRTB` `7682` through `7685`, neutral core systems must land in the `41` to `59` window driven by `GNPRTB` `7764`, and rim systems must match the original populated versus unpopulated rules. Coruscant, Yavin, and the Rebel headquarters must all start charted, populated, and fully loyal to their owning side.

Acceptance for HQ and fleet behavior is that Yavin is no longer the Rebel headquarters, the Rebel headquarters is always a rim system, and the fixed fleet tables stop appearing in arbitrary nearby systems. `CMUNEFTB` must seed only at Coruscant. `CMUNYVTB` must seed only at Yavin. `CMUNHQTB` and `FACLHQTB` must seed only at the chosen headquarters. The Alliance fleet’s 50/50 Yavin-versus-HQ behavior from parameter `5129` must be represented in a deterministic test by forcing both branches with fixed RNG.

Acceptance for economy and force mass is that every populated system ends seeding with legal `total_energy` and `raw_materials`, procedural facilities do not exceed those capacities, and the maintenance-budget loop adds the majority of starting forces without spending beyond the calculated allowance. The total seeded maintenance for each faction must be less than or equal to its budget ceiling for the chosen galaxy size and difficulty, and the budget should be measurably different between at least one easy-versus-hard or standard-versus-huge comparison.

Acceptance for integration is that the setup screen’s galaxy size and difficulty now influence seeding, save behavior is deliberate and tested, and the full workspace still builds with `PATH=/usr/bin:$PATH cargo check`.

## Idempotence and Recovery

Every implementation step in this plan should be safe to repeat in a fresh branch checkout. The parameter-loading changes are additive. The seeding pipeline should be pure with respect to a supplied RNG, so rerunning the same test with the same seed should produce identical results. New tests should build fresh worlds instead of mutating shared fixtures.

Recovery needs to be planned before the first serialized layout change lands. If `System` gains new persisted fields, either keep the branch isolated until the save migration or version bump is complete, or land the save policy in the same changeset so no intermediate commit creates unreadable saves without a clear error. If the seeding rewrite destabilizes gameplay, temporarily keep the old `load_game_data()` wrapper and hide the new pipeline behind the new seeded loader until tests are green.

The clean rollback path is file-level and local: revert the seeding-pipeline edits in `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-data/src/seeds.rs`, restore the old loader ordering in `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-data/src/lib.rs`, and revert any save-version bump in `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-data/src/save.rs` together so the workspace returns to one consistent state.

## Interfaces and Dependencies

At completion, `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-core/src/world/mod.rs` should expose a runtime side-parameter type analogous to `GnprtbParams`, with an accessor along the lines of `value(param_id: u16, difficulty: u8, faction: rebellion_core::dat::Faction) -> i32`. `GameWorld` should carry this table so `rebellion-data` can seed from it without reparsing files, and `System` should expose persisted seeding fields for population and economy.

At completion, `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-data/src/lib.rs` should have a seeded new-game entry point that accepts setup inputs rather than silently using defaults. A good target is a new signature such as `load_game_data_with_options(gdata_path: &Path, options: &SeedOptions) -> anyhow::Result<GameWorld>` while preserving `load_game_data(gdata_path: &Path)` as a convenience wrapper during migration. `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-data/src/seeds.rs` should accept `SeedOptions`, access both GNPRTB and SDPRTB runtime tables, and take an injected RNG for deterministic tests.

The likely new dependency is `rand` in `/Users/tomdimino/Desktop/Programming/open-rebellion/crates/rebellion-data/Cargo.toml`, plus `rand_xoshiro` if the branch wants to use the same generator family already present in the app for deterministic tests. No new binary parsing code is required for `SDPRTB.DAT`; the parser already exists at `/Users/tomdimino/Desktop/Programming/open-rebellion/tools/dat-dumper/src/types/side_params.rs`. The implementation should reuse it instead of inventing a second side-parameter format.
