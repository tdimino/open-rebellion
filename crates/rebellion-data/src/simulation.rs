//! Shared simulation tick function for headless and interactive use.
//!
//! Extracts the per-tick system-advance logic from `rebellion-app/main.rs`
//! into a reusable function that returns structured `GameEventRecord`s
//! instead of pushing to a `MessageLog`.

use crate::integrator::{
    apply_ground_combat_result_inner, resolve_system_space_combat_for, PerceptionIntegrator,
};
use rebellion_core::ai::{AIAction, AIState, AISystem};
use rebellion_core::betrayal::{BetrayalState, BetrayalSystem};
use rebellion_core::blockade::{BlockadeState, BlockadeSystem};
use rebellion_core::bombardment::BombardmentSystem;
use rebellion_core::combat::{side_present, CombatSide, CombatSystem};
use rebellion_core::dat::Faction;
use rebellion_core::death_star::{DeathStarState, DeathStarSystem};
use rebellion_core::delivery::DeliveryState;
use rebellion_core::economy::{EconomyState, EconomySystem};
use rebellion_core::events::{EventState, EventSystem};
use rebellion_core::fog::{FogState, FogSystem};
use rebellion_core::game_events::GameEventRecord;
use rebellion_core::jedi::{JediState, JediSystem};
use rebellion_core::manufacturing::{ManufacturingState, ManufacturingSystem};
use rebellion_core::missions::{MissionState, MissionSystem};
use rebellion_core::movement::{reconcile_fleet_orbits, MovementState, MovementSystem};
use rebellion_core::repair::{RepairState, RepairSystem};
use rebellion_core::research::{ResearchState, ResearchSystem};
use rebellion_core::stockpiles::{StockpileEvent, StockpileState, StockpileSystem};
use rebellion_core::tick::{GameClock, TickEvent};
use rebellion_core::troop_transport::TroopTransportState;
use rebellion_core::uprising::{UprisingState, UprisingSystem};
use rebellion_core::victory::{VictoryState, VictorySystem};
use rebellion_core::world::{CampaignConfig, GameWorld, MstbTable};

// port: safety bound on repeated ground passes; the original assault (order 0x234) is untraced.
const MAX_SYSTEM_GROUND_COMBAT_ROUNDS: u32 = 256;

/// Bundles all mutable simulation state needed for a tick.
///
/// Mirrors the set of `*State` locals in `rebellion-app/src/main.rs`.
pub struct SimulationStates {
    pub clock: GameClock,
    pub manufacturing: ManufacturingState,
    pub missions: MissionState,
    pub events: EventState,
    pub ai: AIState,
    /// Optional second AI for dual-AI mode (controls the opposite faction).
    pub ai2: Option<AIState>,
    pub movement: MovementState,
    pub fog: FogState,
    pub blockade: BlockadeState,
    pub uprising: UprisingState,
    pub death_star: DeathStarState,
    pub research: ResearchState,
    pub jedi: JediState,
    pub victory: VictoryState,
    pub betrayal: BetrayalState,
    pub economy: EconomyState,
    pub repair: RepairState,
    pub troop_transport: TroopTransportState,
    /// Manufactured objects travelling to their destination (F-030).
    pub deliveries: DeliveryState,
    /// Each side's raw and refined materials and the facility cycles.
    pub stockpiles: StockpileState,
    /// Original new-game choices that continue to govern this campaign.
    pub campaign_config: CampaignConfig,
}

/// port: the `index`th roll of `tick`'s derived stream, one SplitMix64 step
/// over the tick, `salt` (the bits of the tick's first supplied roll, so each
/// seed has its own stream) and the index, in `[0, 1)`.
#[expect(
    clippy::cast_precision_loss,
    reason = "53 bits fill an f64 mantissa exactly"
)]
fn derived_roll(tick: u64, salt: u64, index: u64) -> f64 {
    let mut z = (tick ^ salt)
        .wrapping_mul(0x9e37_79b9_7f4a_7c15)
        .wrapping_add(index.wrapping_add(1).wrapping_mul(0xd1b5_4a32_d192_ed03));
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^= z >> 31;
    (z >> 11) as f64 / (1u64 << 53) as f64
}

/// The rounds a day runs less one (`FUN_0051dea0.c:11-30`, the per-day
/// count), from the `--config` AI tuning (`RoundsPerDay::at`).
#[must_use]
pub fn day_rate(clock: &GameClock, config: &rebellion_core::tuning::GameConfig) -> u16 {
    config.ai.rounds_per_day.at(clock.speed).max(1) - 1
}

/// The states one AI step reads and writes.
pub struct AiCtx<'a> {
    pub world: &'a mut GameWorld,
    pub missions: &'a mut MissionState,
    pub manufacturing: &'a mut ManufacturingState,
    pub movement: &'a mut MovementState,
    pub transport: &'a mut TroopTransportState,
    pub stockpiles: &'a mut StockpileState,
    pub victory: &'a mut VictoryState,
    pub death_star: &'a mut DeathStarState,
    pub blockade: &'a mut BlockadeState,
    pub research: &'a ResearchState,
    pub uprising: &'a UprisingState,
    pub config: &'a rebellion_core::tuning::GameConfig,
}

/// One AI order's fate, for the caller's records.
#[derive(Debug, Clone)]
pub struct AiOrderRecord {
    pub faction: Option<rebellion_core::ai::AiFaction>,
    /// The second AI of a dual-AI run.
    pub dual: bool,
    pub issued: rebellion_core::ai_scheduler::Issued,
    pub outcome: crate::integrator::AiOutcome,
    pub strike: crate::integrator::StrikeEffects,
}

impl AiOrderRecord {
    /// Who issued the order and when, as its `ai_action` record reads it.
    #[must_use]
    pub fn origin(&self) -> crate::telemetry::AiOrderOrigin<'static> {
        crate::telemetry::AiOrderOrigin {
            faction: self.faction.map_or("None", faction_name),
            module: self.issued.kind,
            round: self.issued.round,
            issued: self.issued.day,
            dual: self.dual,
        }
    }
}

const fn faction_name(faction: rebellion_core::ai::AiFaction) -> &'static str {
    match faction {
        rebellion_core::ai::AiFaction::Alliance => "Alliance",
        rebellion_core::ai::AiFaction::Empire => "Empire",
    }
}

/// The human player's advisor agent in its side's slot of a step
/// (`FUN_00486500` runs it in phase 8, `FUN_00439a10`).
pub struct AgentSlot<'a> {
    pub agent: &'a mut rebellion_core::agent_automation::PlayerAgent,
    /// The player's side; the Alliance holds the first slot.
    pub alliance: bool,
    pub economy: &'a rebellion_core::economy::EconomyState,
    /// The order the step applied, for the caller's log.
    pub applied: Option<rebellion_core::agent_automation::AgentOrder>,
}

/// One in-day step (`FUN_0051dea0` `+0x20`): each side's pending order is
/// applied, the Alliance's first (`FUN_00436250`), then each side runs its
/// round on the one world (`FUN_004361a0`), neither seeing the other's new
/// order. A step that applied anything rolls for the regiments running a
/// blockade, as the original does at departure in that step's phase 5.
/// `sides` holds each AI and whether it is the second of a dual-AI run;
/// `agent` the human player's advisor agent, which takes the player's slot.
#[expect(
    clippy::too_many_arguments,
    reason = "A step reads the AI sides, the player's agent, the held predicate and the rolls."
)]
pub fn ai_step(
    ctx: &mut AiCtx<'_>,
    sides: &mut [(&mut AIState, bool)],
    mut agent: Option<&mut AgentSlot<'_>>,
    step: u16,
    day: u64,
    held: &dyn Fn(
        &GameWorld,
        &TroopTransportState,
        &AIAction,
    ) -> Option<rebellion_core::ids::SystemKey>,
    rolls: &mut dyn FnMut(usize) -> Vec<f64>,
) -> (
    Vec<AiOrderRecord>,
    Vec<rebellion_core::blockade::BlockadeEvent>,
) {
    use crate::integrator::{apply_ai_order, resolve_ai_strike, AiOutcome, StrikeEffects};
    // FUN_004361a0 steps the Alliance (session +0x10) before the Empire.
    sides.sort_by_key(|(state, _)| {
        !matches!(state.faction, Some(rebellion_core::ai::AiFaction::Alliance))
    });
    // The agent's slot is the player's side: before the AI sides when the
    // player is the Alliance, after them when the Empire.
    let agent_first = agent.as_ref().is_some_and(|slot| slot.alliance);
    let apply_agent = |slot: &mut Option<&mut AgentSlot<'_>>, ctx: &mut AiCtx<'_>| {
        if let Some(slot) = slot {
            slot.applied = slot.agent.take_pending();
            if let Some(order) = &slot.applied {
                rebellion_core::agent_automation::apply_orders(
                    std::slice::from_ref(order),
                    ctx.manufacturing,
                );
            }
        }
    };
    if agent_first {
        apply_agent(&mut agent, ctx);
    }
    let mut records = Vec::new();
    for (state, dual) in sides.iter_mut() {
        let Some(issued) = state.scheduler.pending.take() else {
            continue;
        };
        let (outcome, strike) = if let Some(system) = held(ctx.world, ctx.transport, &issued.action)
        {
            (AiOutcome::Held { system }, StrikeEffects::default())
        } else if matches!(
            issued.action,
            AIAction::Bombard { .. } | AIAction::DestroySystem { .. }
        ) {
            resolve_ai_strike(
                &issued.action,
                ctx.world,
                ctx.victory,
                ctx.death_star,
                ctx.movement,
                ctx.manufacturing,
                ctx.blockade,
                day,
            )
        } else {
            let outcome = apply_ai_order(
                &issued.action,
                state,
                ctx.missions,
                ctx.manufacturing,
                ctx.movement,
                ctx.transport,
                ctx.stockpiles,
                ctx.world,
                ctx.uprising,
                day,
            );
            (outcome, StrikeEffects::default())
        };
        // A refused or held order restarts its module (FUN_004dd810).
        AISystem::settle(state, issued.kind, outcome.is_applied());
        records.push(AiOrderRecord {
            faction: state.faction,
            dual: *dual,
            issued,
            outcome,
            strike,
        });
    }
    if !agent_first {
        apply_agent(&mut agent, ctx);
    }
    let mut runs = Vec::new();
    if records.iter().any(|record| record.outcome.is_applied()) {
        rebellion_core::carriage::tag_all(ctx.world);
        let running =
            BlockadeSystem::running_regiments(ctx.blockade, ctx.world, ctx.movement, ctx.transport);
        let running_rolls = rolls(running.len());
        runs = BlockadeSystem::resolve_running(&running, &running_rolls, day);
    }
    let step_agent = |slot: &mut Option<&mut AgentSlot<'_>>, ctx: &AiCtx<'_>| {
        if let Some(slot) = slot {
            slot.agent
                .step(ctx.world, ctx.manufacturing, slot.economy, slot.alliance);
        }
    };
    if agent_first {
        step_agent(&mut agent, ctx);
    }
    for (state, _) in sides.iter_mut() {
        let inputs = rebellion_core::ai_scheduler::AiInputs {
            world: ctx.world,
            manufacturing: ctx.manufacturing,
            movement: ctx.movement,
            config: ctx.config,
            research: ctx.research,
            uprisings: ctx.uprising,
            transport: ctx.transport,
            missions: ctx.missions,
            tick: day,
            round_in_day: step,
        };
        state.scheduler.pending = AISystem::round(state, &inputs);
    }
    if !agent_first {
        step_agent(&mut agent, ctx);
    }
    (records, runs)
}

/// [`ai_step`] over the simulation's states, with its records emitted and
/// its blockade runs applied. Headless holds no order.
fn ai_step_emitting(
    world: &mut GameWorld,
    states: &mut SimulationStates,
    config: &rebellion_core::tuning::GameConfig,
    integrator: &mut PerceptionIntegrator,
    step: u16,
    day: u64,
    rolls: &mut dyn FnMut(usize) -> Vec<f64>,
) {
    let mut ctx = AiCtx {
        world,
        missions: &mut states.missions,
        manufacturing: &mut states.manufacturing,
        movement: &mut states.movement,
        transport: &mut states.troop_transport,
        stockpiles: &mut states.stockpiles,
        victory: &mut states.victory,
        death_star: &mut states.death_star,
        blockade: &mut states.blockade,
        research: &states.research,
        uprising: &states.uprising,
        config,
    };
    let mut sides: Vec<(&mut AIState, bool)> = vec![(&mut states.ai, false)];
    if let Some(ai2) = states.ai2.as_mut() {
        sides.push((ai2, true));
    }
    let (records, runs) = ai_step(
        &mut ctx,
        &mut sides,
        None,
        step,
        day,
        &|_, _, _| None,
        rolls,
    );
    let AiCtx {
        world, transport, ..
    } = ctx;
    integrator.at_tick(day, |integrator| {
        for record in &records {
            integrator.emit_ai_order(
                world,
                &record.issued.action,
                &record.origin(),
                &record.outcome,
            );
            integrator.emit_strike(world, &record.strike);
        }
        integrator.apply_blockade_events(world, transport, &runs);
    });
}

/// The day's end for every side (`FUN_00485de0.c:7-11`): each side's
/// day as its `ai_day` record reads it, then its order count clears.
pub fn close_ai_days(
    sides: &mut [(&mut AIState, bool)],
    clock: &GameClock,
    config: &rebellion_core::tuning::GameConfig,
    day: u64,
) -> Vec<crate::telemetry::AiDay<'static>> {
    use rebellion_core::tick::GameSpeed;
    let speed = clock.speed;
    let rate = day_rate(clock, config);
    let speed_name = match speed {
        GameSpeed::VerySlow => "very_slow",
        GameSpeed::Slow => "slow",
        GameSpeed::Medium => "medium",
        GameSpeed::Fast | GameSpeed::Paused => "fast",
    };
    let mut days = Vec::new();
    for (state, dual) in sides.iter_mut() {
        if let Some(faction) = state.faction {
            let scheduler = &state.scheduler;
            days.push(crate::telemetry::AiDay {
                faction: faction_name(faction),
                day,
                speed: speed_name,
                paused: speed == GameSpeed::Paused,
                rate,
                rounds: if scheduler.carried {
                    0
                } else {
                    scheduler.round_in_day + 1
                },
                orders: scheduler.orders_today,
                dual: *dual,
            });
        }
        AISystem::end_day(state);
    }
    days
}

/// [`close_ai_days`] over the simulation's sides, with its records emitted.
fn close_ai_days_emitting(
    states: &mut SimulationStates,
    integrator: &mut PerceptionIntegrator,
    config: &rebellion_core::tuning::GameConfig,
    day: u64,
) {
    let mut sides: Vec<(&mut AIState, bool)> = vec![(&mut states.ai, false)];
    if let Some(ai2) = states.ai2.as_mut() {
        sides.push((ai2, true));
    }
    for closed in close_ai_days(&mut sides, &states.clock, config, day) {
        integrator.emit(
            rebellion_core::game_events::SYS_AI,
            rebellion_core::game_events::EVT_AI_DAY,
            crate::telemetry::ai_day_payload(&closed),
        );
    }
}

/// Run one simulation tick across all 15 systems.
///
/// Advances each system in the canonical order (economy → manufacturing → movement →
/// combat → blockade → fog → missions → events → AI (with its Bombard orders) →
/// running regiments → uprising → betrayal → `death_star` (with the AI's Destroy
/// System shots) → research → repair → jedi → victory), applies effects to
/// `world`, and returns a `Vec<GameEventRecord>` describing everything that
/// happened.
///
/// `tick_events` comes from `GameClock::advance()`. `rolls` is a pre-generated
/// slice of uniform `[0,1)` f64 values consumed sequentially by systems that
/// need randomness. Pass at least 1024 rolls to cover a typical tick.
///
/// `wall_ms` is the wall-clock milliseconds since session start, used for
/// the `wall_ms` field on each event record.
#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
#[expect(
    clippy::cast_possible_truncation,
    reason = "Retain the existing simulation rounding, saturation and fixed-width arithmetic semantics."
)]
pub fn run_simulation_tick(
    world: &mut GameWorld,
    states: &mut SimulationStates,
    tick_events: &[TickEvent],
    rolls: &[f64],
    wall_ms: u64,
    config: &rebellion_core::tuning::GameConfig,
) -> Vec<GameEventRecord> {
    let Some(last_tick_event) = tick_events.last() else {
        return Vec::new();
    };

    let mut integrator = PerceptionIntegrator::new(last_tick_event.tick, wall_ms);
    let roll_cursor = std::cell::Cell::new(0usize);
    let derived_count = std::cell::Cell::new(0u64);
    let current_tick = last_tick_event.tick;
    let roll_salt = rolls.first().map_or(0, |roll| roll.to_bits());

    // `MovementState` is authoritative while a fleet is in hyperspace. Repair
    // stale save/index state before economy and manufacturing inspect orbits.
    reconcile_fleet_orbits(&states.movement, world);
    // System bit 11 before anything reads it (FUN_005178b0).
    rebellion_core::death_star::refresh_sectors(world, &states.movement);

    // Helper: consume N rolls from the slice. A supplied buffer that runs
    // out is continued from a SplitMix64 stream keyed on the tick and the
    // seed (port: the original draws every roll from its generator and never
    // runs short).
    let take_rolls = |n: usize| -> Vec<f64> {
        let start = roll_cursor.get();
        let end = (start + n).min(rolls.len());
        roll_cursor.set(end);
        let mut v: Vec<f64> = rolls[start..end].to_vec();
        while v.len() < n {
            let index = derived_count.get();
            derived_count.set(index + 1);
            v.push(derived_roll(current_tick, roll_salt, index));
        }
        v
    };

    // The day before closes: an AI day a native save left open finishes its
    // rounds (FUN_0051dea0), then every side's order count clears
    // (FUN_00485de0 via phase 13). Its last round's order stays pending.
    if let Some(first) = tick_events.first() {
        if states.ai.scheduler.day_open() {
            let day = first.tick.saturating_sub(1);
            let rate = day_rate(&states.clock, config);
            for step in states.ai.scheduler.next_step()..=rate {
                ai_step_emitting(
                    world,
                    states,
                    config,
                    &mut integrator,
                    step,
                    day,
                    &mut |n| take_rolls(n),
                );
            }
            close_ai_days_emitting(states, &mut integrator, config, day);
        }
    }

    // ── 0. Economy (runs BEFORE manufacturing — affects production) ──────
    let economy_events = EconomySystem::advance_with_uprisings(
        &mut states.economy,
        world,
        tick_events,
        world.difficulty_index,
        &states.uprising,
    );
    integrator.apply_economy_events(world, &economy_events);

    // ── 0b. Stockpiles: facility cycles, yard work, overdraft scraps ─────
    let stockpile_events = StockpileSystem::advance(
        &mut states.stockpiles,
        world,
        &mut states.manufacturing,
        &states.economy,
        tick_events,
        states.blockade.blockaded_systems(),
        &take_rolls(tick_events.len()),
    );
    integrator.emit_maintenance_shortfall(world, &states.troop_transport, &stockpile_events);
    rebellion_core::scrap::scrap_all(
        world,
        &mut states.stockpiles,
        &mut states.troop_transport,
        stockpile_events.iter().filter_map(|event| match *event {
            StockpileEvent::OverdraftScrap { target, .. } => Some(target),
            _ => None,
        }),
    );

    // ── 1. Manufacturing ─────────────────────────────────────────────────
    // Use advance_tracked so the K6 EVT_MANUFACTURING_IDLE telemetry is
    // emitted for any system whose queue transitioned empty this tick.
    let mfg_advance = ManufacturingSystem::advance_tracked(
        &mut states.manufacturing,
        tick_events,
        states.blockade.blockaded_systems(),
    );
    integrator.apply_build_completions(world, &mfg_advance.completions);
    integrator.apply_manufacturing_idle(world, &mfg_advance.newly_idle);
    // Remote products travel, then complete at their destination (F-030).
    states.deliveries.depart(world, &mfg_advance.departures);
    let delivered = states.deliveries.advance(world, tick_events);
    integrator.apply_build_completions(world, &delivered.arrivals);
    integrator.apply_boarded(world, &delivered.boarded);
    integrator.apply_deliveries_lost(world, &delivered.lost);

    // ── 2. Movement ──────────────────────────────────────────────────────
    let arrivals = MovementSystem::advance(&mut states.movement, tick_events);
    integrator.apply_arrivals(
        world,
        &states.movement,
        &mut states.troop_transport,
        &arrivals,
    );
    // Regiments travelling on their own (FUN_00556430, event 0x387).
    let regiments = states.troop_transport.advance_transit(world, current_tick);
    integrator.apply_regiment_arrivals(world, &regiments);
    // An arriving Death Star sets bit 11 in its new sector (event 0x301).
    rebellion_core::death_star::refresh_sectors(world, &states.movement);

    // ── 3. Combat ────────────────────────────────────────────────────────
    // FUN_0051ddf0 -> FUN_00517270: a system fights on every
    // pass that finds both sides present; when both remain, no result is
    // shown and the battle fires again next pass. There is no cooldown.
    let combat_triggers: Vec<_> = world
        .systems
        .keys()
        .filter(|&sys_key| {
            // FUN_00509710 / FUN_0050b8e0: both sides present, each with a
            // fleet or squadrons the system holds for it.
            side_present(world, sys_key, true) && side_present(world, sys_key, false)
        })
        .collect();

    for sys_key in combat_triggers {
        // One roll seeds the battle's own generator (`OriginalTacticalRng`).
        let seed = (take_rolls(1)[0] * f64::from(u32::MAX)) as u32;
        // port: with one AI the side it does not play is the person's
        // (FUN_004865b0 gives it the withdrawal threshold 0.0); with two,
        // or none, both are computer players.
        let human_is_alliance = match (&states.ai2, states.ai.faction) {
            (None, Some(rebellion_core::ai::AiFaction::Empire)) => Some(true),
            (None, Some(rebellion_core::ai::AiFaction::Alliance)) => Some(false),
            _ => None,
        };
        let space_result = resolve_system_space_combat_for(
            world,
            &mut states.troop_transport,
            &mut states.movement,
            sys_key,
            seed,
            current_tick,
            human_is_alliance,
        );
        rebellion_core::tactical::apply::reconcile_death_star_result(
            world,
            &mut states.death_star,
            &mut states.victory,
        );

        integrator.emit_system_space_combat(world, &space_result);

        // Orbital bombardment follows a decisive space victory. Ground
        // assault is resolved below after surviving transports have landed.
        // port: this follow-up has no source of its own; the original's
        // bombardment is the Bombard order (or-port-bombardment-8usw).
        // Alliance is always coded as attacker, Empire as defender in the trigger above.
        // The space combat winner gets to follow up with ground assault + orbital bombardment.
        let winner_info = match (space_result.winner, space_result.winner_fleet) {
            (CombatSide::Attacker, Some(fleet)) => Some((fleet, true)), // Alliance won
            (CombatSide::Defender, Some(fleet)) => Some((fleet, false)), // Empire won
            _ => None,
        };
        if let Some((winner_fleet, winner_is_alliance)) = winner_info {
            let brd_result = BombardmentSystem::resolve_bombardment(
                world,
                winner_fleet,
                sys_key,
                world.difficulty_index,
                current_tick,
            );
            let attacker = if winner_is_alliance {
                Faction::Alliance
            } else {
                Faction::Empire
            };
            let headquarters_destroyed = VictorySystem::apply_headquarters_bombardment(
                &states.victory,
                world,
                &brd_result,
                attacker,
            );
            // FUN_0058e320 ends a bombardment run, as for the AI's Bombard.
            // port: the run rolls no hits, so +0x28 stays clear.
            rebellion_core::bombardment::finalize(
                world,
                sys_key,
                attacker,
                rebellion_core::bombardment::BombardmentRun::default(),
            );
            integrator.emit_bombardment(world, sys_key, brd_result.damage, headquarters_destroyed);
        }
    }

    let destroyed_cargo = states.troop_transport.destroy_untransportable_cargo(world);
    integrator.emit_destroyed_transport_cargo(&destroyed_cargo);

    // ── 3b. Troop landing, ground combat, and occupation ────────────────
    // A fleet may land cargo only after it is the sole orbital faction. This
    // also covers unopposed invasions, which do not create a space-combat
    // trigger. Surface combat continues on later ticks until it is decisive.
    let ground_systems: Vec<_> = world.systems.keys().collect();
    for sys_key in ground_systems {
        let orbiting: Vec<_> = world
            .systems
            .get(sys_key)
            .map(|system| system.fleets.clone())
            .unwrap_or_default();
        let has_alliance_fleet = orbiting.iter().any(|fleet| {
            world
                .fleets
                .get(*fleet)
                .is_some_and(|value| value.is_alliance)
        });
        let has_empire_fleet = orbiting.iter().any(|fleet| {
            world
                .fleets
                .get(*fleet)
                .is_some_and(|value| !value.is_alliance)
        });
        let orbital_winner = match (has_alliance_fleet, has_empire_fleet) {
            (true, false) => Some(Faction::Alliance),
            (false, true) => Some(Faction::Empire),
            _ => None,
        };

        if let Some(faction) = orbital_winner {
            let landing_fleets: Vec<_> =
                orbiting
                    .iter()
                    .copied()
                    .filter(|fleet| {
                        world.fleets.get(*fleet).is_some_and(|value| {
                            value.is_alliance == (faction == Faction::Alliance)
                        }) && !states.troop_transport.is_held(*fleet)
                    })
                    .collect();

            // An unopposed Imperial invasion of the mobile Alliance HQ has no
            // space battle to trigger the normal bombardment follow-up. Strike
            // the HQ before landing, but only when a transport is actually
            // carrying an invasion force. Occupation alone is not sufficient.
            // port: no source of its own (or-port-bombardment-8usw).
            if faction == Faction::Empire
                && sys_key == states.victory.alliance_hq
                && world
                    .systems
                    .get(sys_key)
                    .is_some_and(|system| system.is_headquarters && !system.is_destroyed)
            {
                if let Some(bombardment_fleet) = landing_fleets
                    .iter()
                    .copied()
                    .find(|fleet| states.troop_transport.carried_count(*fleet) > 0)
                {
                    let result = BombardmentSystem::resolve_bombardment(
                        world,
                        bombardment_fleet,
                        sys_key,
                        world.difficulty_index,
                        current_tick,
                    );
                    let headquarters_destroyed = VictorySystem::apply_headquarters_bombardment(
                        &states.victory,
                        world,
                        &result,
                        Faction::Empire,
                    );
                    // FUN_0058e320, as above.
                    rebellion_core::bombardment::finalize(
                        world,
                        sys_key,
                        Faction::Empire,
                        rebellion_core::bombardment::BombardmentRun::default(),
                    );
                    integrator.emit_bombardment(
                        world,
                        sys_key,
                        result.damage,
                        headquarters_destroyed,
                    );
                }
            }

            for fleet in landing_fleets {
                integrator.apply_troop_landing(world, &mut states.troop_transport, fleet, sys_key);
            }
        }

        let (alliance_troops, empire_troops) = world
            .systems
            .get(sys_key)
            .map(|system| {
                system
                    .ground_units
                    .iter()
                    .fold((0_usize, 0_usize), |counts, troop| {
                        match world.troops.get(*troop) {
                            Some(value) if value.regiment_strength > 0 && value.is_alliance => {
                                (counts.0 + 1, counts.1)
                            }
                            Some(value) if value.regiment_strength > 0 => (counts.0, counts.1 + 1),
                            _ => counts,
                        }
                    })
            })
            .unwrap_or_default();

        let occupying_faction = match (alliance_troops > 0, empire_troops > 0) {
            (true, true) => {
                let attacker_is_alliance =
                    orbital_winner.is_none_or(|faction| faction == Faction::Alliance);
                let ground_rolls = take_rolls(256);
                let mut final_winner = CombatSide::Draw;
                let mut total_engagements = 0;
                let mut rounds = 0;
                let mut stalemate = false;
                while rounds < MAX_SYSTEM_GROUND_COMBAT_ROUNDS {
                    let result = CombatSystem::resolve_ground(
                        world,
                        sys_key,
                        attacker_is_alliance,
                        world.difficulty_index,
                        &ground_rolls,
                        current_tick,
                    );
                    let made_progress = result
                        .troop_damage
                        .iter()
                        .any(|event| event.strength_after < event.strength_before);
                    final_winner = result.winner;
                    total_engagements += result.troop_damage.len();
                    rounds += 1;
                    apply_ground_combat_result_inner(&result, world);
                    if final_winner != CombatSide::Draw || !made_progress {
                        stalemate = final_winner == CombatSide::Draw;
                        break;
                    }
                }
                if rounds == MAX_SYSTEM_GROUND_COMBAT_ROUNDS && final_winner == CombatSide::Draw {
                    stalemate = true;
                }
                integrator.emit_system_ground_combat(
                    world,
                    sys_key,
                    final_winner,
                    total_engagements,
                    rounds,
                    stalemate,
                );
                match final_winner {
                    CombatSide::Attacker => Some(if attacker_is_alliance {
                        Faction::Alliance
                    } else {
                        Faction::Empire
                    }),
                    CombatSide::Defender => Some(if attacker_is_alliance {
                        Faction::Empire
                    } else {
                        Faction::Alliance
                    }),
                    CombatSide::Draw => None,
                }
            }
            (true, false) => Some(Faction::Alliance),
            (false, true) => Some(Faction::Empire),
            (false, false) => None,
        };

        if let Some(winner) = occupying_faction {
            let control = world.systems.get(sys_key).map(|system| system.control);
            if control != Some(rebellion_core::world::ControlKind::Controlled(winner)) {
                integrator.apply_ground_occupation(world, sys_key, winner, current_tick);
            }
        }
    }

    // Observe embarked regiments after arrivals and landings, so a fleet that
    // arrives and is ordered away again this tick still copies the arrival
    // system's withdraw percent. Their rolls happen at the blockade step.
    let mut running_regiments = if tick_events.is_empty() {
        Vec::new()
    } else {
        BlockadeSystem::running_regiments(
            &mut states.blockade,
            world,
            &states.movement,
            &states.troop_transport,
        )
    };

    // A Death Star lost in battle clears bit 11 before the blockade and
    // uprising steps read it (event 0x301).
    rebellion_core::death_star::refresh_sectors(world, &states.movement);

    // The blockade rule reruns as an arrival or a battle settles, before the
    // AI's orders take effect (`FUN_0050b8e0` through event 0x311,
    // `ghidra/notes/blockade-bit.md`, "When it runs"): a fleet the AI sends on
    // the day it arrives still blockades for that day.
    let mut blockade_events = BlockadeSystem::advance(&mut states.blockade, world, tick_events);

    // ── 4. Fog of war ────────────────────────────────────────────────────
    let reveals = FogSystem::advance(&mut states.fog, world, &states.movement);
    integrator.emit_fog_reveals(&reveals, world);

    // ── 5. Missions ──────────────────────────────────────────────────────
    let mission_rolls = take_rolls(
        states.missions.len() * tick_events.len() * rebellion_core::missions::ROLLS_PER_MISSION,
    );
    let mission_advance = MissionSystem::advance(
        &mut states.missions,
        world,
        &states.uprising,
        tick_events,
        &mission_rolls,
    );
    rebellion_core::missions::apply_advance_effects(world, &mission_advance.effects);
    for ended in &mission_advance.ended {
        integrator.emit(
            rebellion_core::game_events::SYS_MISSIONS,
            rebellion_core::game_events::EVT_MISSION_ENDED,
            crate::telemetry::mission_ended_payload(ended),
        );
    }
    states.ai.free_mission_members(&mission_advance.effects);
    if let Some(ref mut ai2) = states.ai2 {
        ai2.free_mission_members(&mission_advance.effects);
    }
    for result in &mission_advance.results {
        integrator.apply_mission_result(world, result, &mut states.uprising, &mut states.research);
        // Knesset Shamash-Bet #R11: emit `EVT_CHARACTER_KILLED` telemetry for
        // mission-side assassinations. The integrator's `MissionEffect::CharacterKilled`
        // arm marks `is_killed = true` via `mark_killed()` instead of deleting
        // from the arena, so the lookup below ALWAYS succeeds under correct
        // invariants — the `<unknown>` fallback is a real invariant break and
        // panics in debug builds.
        for effect in &result.effects {
            if let rebellion_core::missions::MissionEffect::CharacterKilled { character, .. } =
                effect
            {
                debug_assert!(
                    world.characters.contains_key(*character),
                    "EVT_CHARACTER_KILLED target missing from arena — R11 invariant break"
                );
                let (name, dat_id) = world.characters.get(*character).map_or_else(
                    || (String::from("<unknown>"), 0),
                    |c| (c.name.clone(), c.dat_id.raw()),
                );
                integrator.emit(
                    rebellion_core::game_events::SYS_MISSIONS,
                    rebellion_core::game_events::EVT_CHARACTER_KILLED,
                    serde_json::json!({
                        "name": name,
                        "dat_id": dat_id,
                        "cause": "assassination",
                    }),
                );
            }
        }

        // Knesset Shamash-Bet #R6/#R7/#R8: mission-side notification events.
        //
        // These are state-transition telemetry markers on the outcome of the
        // mission, NOT `Random`-gated EventSystem entries. The CI guard
        // `notification_events_never_use_random` protects the corresponding
        // story-event IDs from being misused.
        match result.outcome {
            rebellion_core::missions::MissionOutcome::Success => {
                // #R6: Successful espionage yields informant intel.
                if matches!(
                    result.kind,
                    rebellion_core::missions::MissionKind::Espionage
                ) {
                    let char_name = result
                        .character
                        .and_then(|key| world.characters.get(key))
                        .map_or_else(|| String::from("<unknown>"), |c| c.name.clone());
                    let sys_name = world
                        .systems
                        .get(result.target_system)
                        .map_or_else(|| String::from("<unknown>"), |s| s.name.clone());
                    integrator.emit(
                        rebellion_core::game_events::SYS_MISSIONS,
                        rebellion_core::game_events::EVT_INFORMANT_INTEL,
                        serde_json::json!({
                            "character": char_name,
                            "system": sys_name,
                        }),
                    );
                }
            }
            rebellion_core::missions::MissionOutcome::Failure => {
                // #R8: Mission failure on combat/assassination → character health hit.
                if matches!(
                    result.kind,
                    rebellion_core::missions::MissionKind::Assassination
                        | rebellion_core::missions::MissionKind::Abduction
                        | rebellion_core::missions::MissionKind::Rescue
                ) {
                    let char_name = result
                        .character
                        .and_then(|key| world.characters.get(key))
                        .map_or_else(|| String::from("<unknown>"), |c| c.name.clone());
                    integrator.emit(
                        rebellion_core::game_events::SYS_MISSIONS,
                        rebellion_core::game_events::EVT_CHARACTER_HEALTH,
                        serde_json::json!({
                            "character": char_name,
                            "mission_kind": format!("{:?}", result.kind),
                        }),
                    );
                }
            }
        }
    }

    // ── 5b. Character escapes ────────────────────────────────────────────
    let escape_rolls = take_rolls(world.characters.len());
    let escape_effects = MissionSystem::check_escapes(world, &escape_rolls);
    integrator.apply_escape_effects(world, &escape_effects);

    // ── 6. Events ────────────────────────────────────────────────────────
    let event_rolls: Vec<f32> = take_rolls(16).iter().map(|&r| r as f32).collect();
    let fired_events = EventSystem::advance(&mut states.events, world, tick_events, &event_rolls);
    integrator.apply_fired_events(
        world,
        &fired_events,
        &mut states.jedi,
        current_tick,
        &states.movement,
    );

    // ── 8. Blockade ──────────────────────────────────────────────────────
    if let Some(last) = tick_events.last() {
        running_regiments.extend(BlockadeSystem::running_regiments(
            &mut states.blockade,
            world,
            &states.movement,
            &states.troop_transport,
        ));
        let running_rolls = take_rolls(running_regiments.len());
        blockade_events.extend(BlockadeSystem::resolve_running(
            &running_regiments,
            &running_rolls,
            last.tick,
        ));
    }
    integrator.apply_blockade_events(world, &mut states.troop_transport, &blockade_events);

    // ── 9. Uprising ──────────────────────────────────────────────────────
    let uprising_rolls = take_rolls(UprisingSystem::roll_budget(
        &states.uprising,
        world,
        tick_events.last().map_or(0, |event| event.tick),
    ));
    let empty_table = MstbTable::new(vec![]);
    let uprising_events = UprisingSystem::advance(
        &mut states.uprising,
        world,
        &states.economy,
        &states.missions,
        tick_events,
        &uprising_rolls,
    );
    integrator.apply_uprising_events(world, &uprising_events);

    // ── 10. Betrayal ─────────────────────────────────────────────────────
    let betrayal_rolls = take_rolls(world.characters.len());
    let loyalty_tb = world.mission_tables.get("UPRIS1TB").unwrap_or(&empty_table);
    let betrayal_events = BetrayalSystem::advance(
        &mut states.betrayal,
        world,
        tick_events,
        &betrayal_rolls,
        loyalty_tb,
    );
    integrator.apply_betrayal_events(world, &betrayal_events);

    // ── 11. Death Star ───────────────────────────────────────────────────
    let ds_events = DeathStarSystem::advance(&mut states.death_star, world, tick_events);
    integrator.apply_death_star_events(world, &ds_events);
    // Update victory state and clean up destroyed systems; each character
    // killed gets an `EVT_CHARACTER_KILLED` record. Character-death story
    // events are strictly next-tick reactive.
    for evt in &ds_events {
        if let rebellion_core::death_star::DeathStarEvent::PlanetDestroyed { system, .. } = evt {
            let killed = crate::integrator::destroy_system(
                world,
                *system,
                &mut states.victory,
                &mut states.death_star,
                &mut states.movement,
                &mut states.manufacturing,
                &mut states.blockade,
            );
            integrator.emit_killed(world, *system, &killed);
        }
    }

    // ── 12. Research ─────────────────────────────────────────────────────
    let research_results = ResearchSystem::advance(
        &mut states.research,
        world,
        &states.uprising,
        tick_events,
        &take_rolls(tick_events.len()),
    );
    integrator.apply_research_results(&research_results);

    // ── 12b. Ship repair ────────────────────────────────────────────────
    let repair_events = RepairSystem::advance(
        &mut states.repair,
        world,
        &states.uprising,
        tick_events,
        &take_rolls(RepairSystem::roll_budget(world, tick_events.len())),
    );
    integrator.apply_repair_events(world, &repair_events);

    // ── 13. Jedi training ────────────────────────────────────────────────
    let jedi_rolls = take_rolls(states.jedi.training.len().max(1));
    let jedi_events = JediSystem::advance(&mut states.jedi, world, tick_events, &jedi_rolls);
    integrator.apply_jedi_events(world, &jedi_events, &mut states.jedi);

    // ── 13b. AI ──────────────────────────────────────────────────────────
    // Phase 8 (FUN_004361a0) follows every due timer of phase 5, so the
    // rounds run after the daily stages. port: the decompiles do not place
    // the flagship naming, the rank lapses or the victory check relative to
    // phase 8; they follow it here. port: a call carrying several days runs
    // one day's rounds.
    let headquarters_only = states.campaign_config.victory_conditions
        == rebellion_core::world::VictoryConditions::HeadquartersOnly;
    states.ai.set_headquarters_only(headquarters_only);
    states.ai.scheduler.day_started = true;
    if let Some(ai2) = states.ai2.as_mut() {
        ai2.set_headquarters_only(headquarters_only);
        ai2.scheduler.day_started = true;
    }
    rebellion_core::carriage::tag_all(world);
    let rate = day_rate(&states.clock, config);
    for step in 0..=rate {
        ai_step_emitting(
            world,
            states,
            config,
            &mut integrator,
            step,
            current_tick,
            &mut |n| take_rolls(n),
        );
    }

    // port: a fleet that gained its side's flagship class this tick, by a
    // build or a join on arrival, takes its signature name (fleet_name_bank).
    world.name_flagship_fleets();

    // Officers who were captured, sent on a mission or moved off their post
    // this tick lose their rank (FUN_004eeed0, FUN_00560d50).
    rebellion_core::officer_rank::clear_lapsed_ranks(world);

    // ── 14. Victory check ────────────────────────────────────────────────
    integrator.emit_victory_check(&states.victory);
    if let Some(outcome) = VictorySystem::check(
        &states.victory,
        world,
        tick_events,
        states.campaign_config.victory_conditions,
    ) {
        integrator.apply_victory(&outcome, &mut states.victory, world);
    }

    // ── 15. Campaign snapshot (every 250 ticks) ────────────────────────
    if current_tick.is_multiple_of(250) && current_tick > 0 {
        integrator.emit_campaign_snapshot(world, states.movement.len(), &states.economy);
    }

    // The decision traces the steps above recorded (`rebellion_core::trace`).
    for record in crate::telemetry::drain_trace_records(integrator.tick(), integrator.wall_ms()) {
        integrator.push(record);
    }

    integrator.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::ai::AiFaction;
    use rebellion_core::dat::Faction;
    use rebellion_core::fog::FogState;
    use rebellion_core::game_events::{
        EVT_BOMBARDMENT, EVT_CAPTURE, EVT_COMBAT_GROUND, EVT_CONTROL_CHANGED, EVT_FLEET_ARRIVED,
        EVT_TROOP_MOVED, EVT_VICTORY,
    };
    use rebellion_core::ids::SectorKey;
    use rebellion_core::ids::{DatId, FleetKey, TroopKey};
    use rebellion_core::movement::begin_fleet_transit;
    use rebellion_core::world::{
        CapitalShipClass, Character, ControlKind, Fleet, ShipInstance, System, TroopClassDef,
        TroopUnit, VictoryConditions,
    };

    /// GNPRTB 7760 (60 at every difficulty), the support a side needs to
    /// hold a system without regiments (FUN_0055a080).
    fn support_control_threshold() -> rebellion_core::world::GnprtbParams {
        let entry = rebellion_core::world::GnprtbEntry {
            parameter_id: 7760,
            development: 60,
            alliance_sp_easy: 60,
            alliance_sp_medium: 60,
            alliance_sp_hard: 60,
            empire_sp_easy: 60,
            empire_sp_medium: 60,
            empire_sp_hard: 60,
            multiplayer: 60,
        };
        rebellion_core::world::GnprtbParams::new(vec![entry])
    }

    fn make_test_states() -> (GameWorld, SimulationStates) {
        let mut world = GameWorld::default();
        // Need at least 2 systems for VictoryState
        let s1 = world.systems.insert(rebellion_core::world::System {
            dat_id: rebellion_core::ids::DatId::new(1),
            name: "System A".into(),
            sector: SectorKey::default(),
            x: 0,
            y: 0,
            exploration_status: rebellion_core::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Uncontrolled,
            espionage_rating: 0.0,
        });
        let s2 = world.systems.insert(rebellion_core::world::System {
            dat_id: rebellion_core::ids::DatId::new(2),
            name: "System B".into(),
            sector: SectorKey::default(),
            x: 100,
            y: 100,
            exploration_status: rebellion_core::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Uncontrolled,
            espionage_rating: 0.0,
        });

        let states = SimulationStates {
            clock: GameClock::new(),
            manufacturing: ManufacturingState::new(),
            missions: MissionState::new(),
            events: EventState::new(),
            ai: AIState::new(AiFaction::Empire),
            ai2: None,
            movement: MovementState::new(),
            fog: FogState::new(Faction::Alliance),
            blockade: BlockadeState::new(),
            uprising: UprisingState::new(),
            death_star: DeathStarState::default(),
            research: ResearchState::new(),
            jedi: JediState::new(),
            victory: VictoryState::new(s1, s2),
            betrayal: BetrayalState::new(),
            economy: EconomyState::default(),
            repair: RepairState::default(),
            troop_transport: TroopTransportState::default(),
            deliveries: DeliveryState::default(),
            stockpiles: StockpileState::default(),
            campaign_config: CampaignConfig::default(),
        };
        (world, states)
    }

    /// An Alliance fleet with room for one regiment and one Alliance
    /// regiment on the surface, both at the first system, which the Alliance
    /// holds.
    fn world_with_loading_fleet() -> (GameWorld, SimulationStates, FleetKey, TroopKey) {
        let (mut world, states) = make_test_states();
        let system = world.systems.keys().next().unwrap();
        world.systems[system].control = ControlKind::Controlled(Faction::Alliance);
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            name: "Troop Transport".into(),
            is_alliance: true,
            hull: 100,
            troop_capacity: 1,
            ..CapitalShipClass::default()
        });
        let fleet = world.fleets.insert(Fleet {
            location: system,
            capital_ships: vec![ShipInstance::new(class, 100, true)],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        });
        world.systems[system].fleets.push(fleet);
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(20),
            is_alliance: true,
            regiment_strength: 100,
        });
        world.systems[system].ground_units.push(troop);
        (world, states, fleet, troop)
    }

    fn run_one_tick(world: &mut GameWorld, states: &mut SimulationStates) {
        run_simulation_tick(
            world,
            states,
            &[TickEvent { tick: 1 }],
            &[0.99; 2048],
            10,
            &rebellion_core::tuning::GameConfig::default(),
        );
    }

    #[test]
    fn a_regiment_loaded_by_order_stays_aboard_at_the_system_it_loaded_at() {
        // port: the hold (ghidra/notes/fleet-window.md, "Port notes"); the
        // original keeps a loaded regiment in its ship until a landing order.
        let (mut world, mut states, fleet, troop) = world_with_loading_fleet();
        let system = world.fleets[fleet].location;
        states
            .troop_transport
            .load(&mut world, fleet, &[troop])
            .unwrap();

        run_one_tick(&mut world, &mut states);

        assert_eq!(states.troop_transport.cargo(fleet), [troop]);
        assert!(states.troop_transport.is_held(fleet));
        assert!(!world.systems[system].ground_units.contains(&troop));

        // The control: cargo embarked for a departure lands here.
        let (mut world, mut states, fleet, troop) = world_with_loading_fleet();
        states
            .troop_transport
            .embark(&mut world, fleet, &[troop])
            .unwrap();

        run_one_tick(&mut world, &mut states);

        assert!(states.troop_transport.cargo(fleet).is_empty());
        assert!(world.systems[system].ground_units.contains(&troop));
    }

    #[test]
    fn a_day_names_a_fleet_that_gained_its_sides_flagship_class() {
        // port: fleet_name_bank's signature names, under Canonical naming.
        let (mut world, mut states, fleet, _) = world_with_loading_fleet();
        world.start_fleet_naming(rebellion_core::world::FleetNaming::Canonical);
        let class = world.fleets[fleet].capital_ships[0].class;
        world.capital_ship_classes[class].dat_id =
            rebellion_core::fleet_name_bank::MON_CALAMARI_CRUISER;

        run_one_tick(&mut world, &mut states);

        assert_eq!(world.fleet_name(fleet), Some("Rebel Command Fleet"));
    }

    #[test]
    fn empty_tick_events_returns_empty() {
        let (mut world, mut states) = make_test_states();

        let result = run_simulation_tick(
            &mut world,
            &mut states,
            &[],
            &[],
            0,
            &rebellion_core::tuning::GameConfig::default(),
        );
        assert!(result.is_empty());
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
    )]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Retain the existing simulation rounding, saturation and fixed-width arithmetic semantics."
    )]
    fn surface_battle_continues_on_later_tick_after_transport_empties() {
        let mut world = GameWorld::default();
        let make_system = |dat_id, name: &str, control| System {
            dat_id: DatId::new(dat_id),
            name: name.into(),
            sector: SectorKey::default(),
            x: dat_id as u16 * 100,
            y: 0,
            exploration_status: rebellion_core::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control,
            espionage_rating: 0.0,
        };
        let target = world.systems.insert(make_system(
            1,
            "Contested Surface",
            ControlKind::Controlled(Faction::Empire),
        ));
        let other = world.systems.insert(make_system(
            2,
            "Other System",
            ControlKind::Controlled(Faction::Alliance),
        ));
        let fleet = world.fleets.insert(Fleet {
            location: target,
            capital_ships: vec![],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        });
        world.systems[target].fleets.push(fleet);
        let troop_class = DatId::new(20);
        world.troop_classes.insert(
            troop_class,
            TroopClassDef {
                attack_strength: 0,
                defense_strength: 10,
                detection: 0,
            },
        );
        let alliance_troop = world.troops.insert(TroopUnit {
            class_dat_id: troop_class,
            is_alliance: true,
            regiment_strength: 1_000,
        });
        let empire_troop = world.troops.insert(TroopUnit {
            class_dat_id: troop_class,
            is_alliance: false,
            regiment_strength: 1_000,
        });
        world.systems[target]
            .ground_units
            .extend([alliance_troop, empire_troop]);

        let mut states = SimulationStates {
            clock: GameClock::new(),
            manufacturing: ManufacturingState::new(),
            missions: MissionState::new(),
            events: EventState::new(),
            ai: AIState::new(AiFaction::Empire),
            ai2: None,
            movement: MovementState::new(),
            fog: FogState::new(Faction::Alliance),
            blockade: BlockadeState::new(),
            uprising: UprisingState::new(),
            death_star: DeathStarState::default(),
            research: ResearchState::new(),
            jedi: JediState::new(),
            victory: VictoryState::new(other, target),
            betrayal: BetrayalState::new(),
            economy: EconomyState::default(),
            repair: RepairState::default(),
            troop_transport: TroopTransportState::default(),
            deliveries: DeliveryState::default(),
            stockpiles: StockpileState::default(),
            campaign_config: CampaignConfig::default(),
        };
        let config = rebellion_core::tuning::GameConfig::default();

        let first_events = run_simulation_tick(
            &mut world,
            &mut states,
            &[TickEvent { tick: 1 }],
            &[0.99; 2_048],
            10,
            &config,
        );
        let first_strength = world.troops[alliance_troop].regiment_strength;
        assert_eq!(first_strength, 744);
        assert!(states.troop_transport.is_empty());
        assert!(first_events
            .iter()
            .any(|event| event.event_type == EVT_COMBAT_GROUND));

        let second_events = run_simulation_tick(
            &mut world,
            &mut states,
            &[TickEvent { tick: 2 }],
            &[0.99; 2_048],
            20,
            &config,
        );

        assert!(world.troops[alliance_troop].regiment_strength < first_strength);
        assert!(world.troops.contains_key(empire_troop));
        assert!(states.troop_transport.is_empty());
        assert!(second_events
            .iter()
            .any(|event| event.event_type == EVT_COMBAT_GROUND));
        assert_ne!(
            world.systems[target].control,
            ControlKind::Controlled(Faction::Alliance)
        );
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
    )]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Retain the existing simulation rounding, saturation and fixed-width arithmetic semantics."
    )]
    fn decisive_imperial_bombardment_destroys_the_alliance_hq_facility() {
        let mut world = GameWorld::default();
        world.gnprtb = support_control_threshold();
        // Each HQ's people back its holder, so it stays held without troops.
        let make_system = |dat_id, name: &str, control, is_headquarters| System {
            dat_id: DatId::new(dat_id),
            name: name.into(),
            sector: SectorKey::default(),
            x: dat_id as u16 * 100,
            y: 0,
            exploration_status: rebellion_core::dat::ExplorationStatus::Explored,
            popularity_alliance: if control == ControlKind::Controlled(Faction::Alliance) {
                0.9
            } else {
                0.1
            },
            popularity_empire: if control == ControlKind::Controlled(Faction::Empire) {
                0.9
            } else {
                0.1
            },
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters,
            is_destroyed: false,
            control,
            espionage_rating: 0.0,
        };
        let alliance_hq = world.systems.insert(make_system(
            1,
            "Alliance HQ",
            ControlKind::Controlled(Faction::Alliance),
            true,
        ));
        let coruscant = world.systems.insert(make_system(
            2,
            "Coruscant",
            ControlKind::Controlled(Faction::Empire),
            true,
        ));
        let alliance_class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(61),
            name: "Alliance Scout".into(),
            is_alliance: true,
            is_empire: false,
            hull: 1,
            ..CapitalShipClass::default()
        });
        let empire_class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(62),
            name: "Imperial Bombardment Ship".into(),
            is_alliance: false,
            is_empire: true,
            hull: 1_000,
            turbolaser_fore: 1_000,
            // The tactical engine fires only with a range and a recharge, as
            // every armed shipped class has (CAPSHPSD: recharge 6-18, range 50-60).
            turbolaser_range: 60,
            weapon_recharge_rate: 8,
            bombardment_modifier: 100,
            ..CapitalShipClass::default()
        });
        let alliance_fleet = world.fleets.insert(Fleet {
            location: alliance_hq,
            capital_ships: vec![ShipInstance::new(alliance_class, 1, true)],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        });
        let empire_fleet = world.fleets.insert(Fleet {
            location: alliance_hq,
            capital_ships: vec![ShipInstance::new(empire_class, 1_000, false)],
            fighters: vec![],
            characters: vec![],
            is_alliance: false,
            has_death_star: false,
        });
        world.systems[alliance_hq]
            .fleets
            .extend([alliance_fleet, empire_fleet]);

        let mut states = SimulationStates {
            clock: GameClock::new(),
            manufacturing: ManufacturingState::new(),
            missions: MissionState::new(),
            events: EventState::new(),
            ai: AIState::new(AiFaction::Empire),
            ai2: None,
            movement: MovementState::new(),
            fog: FogState::new(Faction::Alliance),
            blockade: BlockadeState::new(),
            uprising: UprisingState::new(),
            death_star: DeathStarState::default(),
            research: ResearchState::new(),
            jedi: JediState::new(),
            victory: VictoryState::new(alliance_hq, coruscant),
            betrayal: BetrayalState::new(),
            economy: EconomyState::default(),
            repair: RepairState::default(),
            troop_transport: TroopTransportState::default(),
            deliveries: DeliveryState::default(),
            stockpiles: StockpileState::default(),
            campaign_config: CampaignConfig {
                victory_conditions: VictoryConditions::HeadquartersOnly,
                ..CampaignConfig::default()
            },
        };

        let events = run_simulation_tick(
            &mut world,
            &mut states,
            &[TickEvent { tick: 1 }],
            &[0.5; 2_048],
            10,
            &rebellion_core::tuning::GameConfig::default(),
        );

        assert!(!world.systems[alliance_hq].is_headquarters);
        assert!(
            !states.victory.resolved,
            "bombardment still requires occupation"
        );
        let bombardment = events
            .iter()
            .find(|event| event.event_type == EVT_BOMBARDMENT)
            .expect("bombardment telemetry");
        assert_eq!(bombardment.details["headquarters_destroyed"], true);
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
    )]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Retain the existing simulation rounding, saturation and fixed-width arithmetic semantics."
    )]
    fn unopposed_imperial_invasion_bombards_before_occupying_alliance_hq() {
        let mut world = GameWorld::default();
        let add_system = |world: &mut GameWorld, dat_id, name: &str, control, is_headquarters| {
            world.systems.insert(System {
                dat_id: DatId::new(dat_id),
                name: name.into(),
                sector: SectorKey::default(),
                x: dat_id as u16 * 100,
                y: 0,
                exploration_status: rebellion_core::dat::ExplorationStatus::Explored,
                popularity_alliance: 0.5,
                popularity_empire: 0.5,
                is_populated: true,
                total_energy: 0,
                raw_materials: 0,
                fleets: vec![],
                ground_units: vec![],
                special_forces: vec![],
                fighters: Vec::new(),
                defense_facilities: vec![],
                manufacturing_facilities: vec![],
                production_facilities: vec![],
                is_headquarters,
                is_destroyed: false,
                control,
                espionage_rating: 0.0,
            })
        };
        let alliance_hq = add_system(
            &mut world,
            1,
            "Alliance HQ",
            ControlKind::Controlled(Faction::Alliance),
            true,
        );
        let coruscant = add_system(
            &mut world,
            2,
            "Coruscant",
            ControlKind::Controlled(Faction::Empire),
            true,
        );
        let origin = add_system(
            &mut world,
            3,
            "Imperial Staging",
            ControlKind::Controlled(Faction::Empire),
            false,
        );
        let transport_class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(63),
            name: "Imperial Assault Transport".into(),
            is_alliance: false,
            is_empire: true,
            hull: 100,
            troop_capacity: 1,
            bombardment_modifier: 100,
            ..CapitalShipClass::default()
        });
        let fleet = world.fleets.insert(Fleet {
            location: origin,
            capital_ships: vec![ShipInstance::new(transport_class, 100, false)],
            fighters: vec![],
            characters: vec![],
            is_alliance: false,
            has_death_star: false,
        });
        world.systems[origin].fleets.push(fleet);
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(20),
            is_alliance: false,
            regiment_strength: 100,
        });
        world.systems[origin].ground_units.push(troop);

        let mut states = SimulationStates {
            clock: GameClock::new(),
            manufacturing: ManufacturingState::new(),
            missions: MissionState::new(),
            events: EventState::new(),
            ai: AIState::new(AiFaction::Empire),
            ai2: None,
            movement: MovementState::new(),
            fog: FogState::new(Faction::Alliance),
            blockade: BlockadeState::new(),
            uprising: UprisingState::new(),
            death_star: DeathStarState::default(),
            research: ResearchState::new(),
            jedi: JediState::new(),
            victory: VictoryState::new(alliance_hq, coruscant),
            betrayal: BetrayalState::new(),
            economy: EconomyState::default(),
            repair: RepairState::default(),
            troop_transport: TroopTransportState::default(),
            deliveries: DeliveryState::default(),
            stockpiles: StockpileState::default(),
            campaign_config: CampaignConfig {
                victory_conditions: VictoryConditions::HeadquartersOnly,
                ..CampaignConfig::default()
            },
        };
        states
            .troop_transport
            .embark(&mut world, fleet, &[troop])
            .unwrap();
        assert!(begin_fleet_transit(
            &mut states.movement,
            &mut world,
            fleet,
            alliance_hq,
            1,
        ));

        let events = run_simulation_tick(
            &mut world,
            &mut states,
            &[TickEvent { tick: 1 }],
            &[0.99; 2_048],
            10,
            &rebellion_core::tuning::GameConfig::default(),
        );

        assert!(!world.systems[alliance_hq].is_headquarters);
        assert_eq!(
            world.systems[alliance_hq].control,
            ControlKind::Controlled(Faction::Empire)
        );
        assert!(states.victory.resolved);
        assert!(events.iter().any(|event| {
            event.event_type == EVT_BOMBARDMENT && event.details["headquarters_destroyed"] == true
        }));
        assert!(events.iter().any(|event| event.event_type == EVT_VICTORY));
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
    )]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Retain the existing simulation rounding, saturation and fixed-width arithmetic semantics."
    )]
    fn troop_transport_arrives_lands_occupies_and_captures() {
        let mut world = GameWorld::default();
        world.gnprtb = support_control_threshold();
        let add_system = |world: &mut GameWorld, dat_id, name: &str, control, is_headquarters| {
            world.systems.insert(System {
                dat_id: DatId::new(dat_id),
                name: name.into(),
                sector: SectorKey::default(),
                x: dat_id as u16 * 100,
                y: 0,
                exploration_status: rebellion_core::dat::ExplorationStatus::Explored,
                popularity_alliance: if control == ControlKind::Controlled(Faction::Alliance) {
                    0.9
                } else {
                    0.1
                },
                popularity_empire: if control == ControlKind::Controlled(Faction::Empire) {
                    0.9
                } else {
                    0.1
                },
                is_populated: true,
                total_energy: 0,
                raw_materials: 0,
                fleets: vec![],
                ground_units: vec![],
                special_forces: vec![],
                fighters: Vec::new(),
                defense_facilities: vec![],
                manufacturing_facilities: vec![],
                production_facilities: vec![],
                is_headquarters,
                is_destroyed: false,
                control,
                espionage_rating: 0.0,
            })
        };
        let origin = add_system(
            &mut world,
            1,
            "Alliance Base",
            ControlKind::Controlled(Faction::Alliance),
            true,
        );
        let target = add_system(
            &mut world,
            2,
            "Imperial HQ",
            ControlKind::Controlled(Faction::Empire),
            true,
        );
        let transport_class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(64),
            name: "Troop Transport".into(),
            is_alliance: true,
            is_empire: false,
            hull: 100,
            troop_capacity: 1,
            ..CapitalShipClass::default()
        });
        let fleet = world.fleets.insert(Fleet {
            location: origin,
            capital_ships: vec![ShipInstance::new(transport_class, 100, true)],
            fighters: vec![],
            characters: vec![],
            is_alliance: true,
            has_death_star: false,
        });
        world.systems[origin].fleets.push(fleet);
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(20),
            is_alliance: true,
            regiment_strength: 100,
        });
        world.systems[origin].ground_units.push(troop);
        let prisoner = world.characters.insert(Character {
            dat_id: DatId::new(4),
            name: "Imperial Leader".into(),
            is_empire: true,
            current_system: Some(target),
            ..Character::default()
        });

        let mut states = SimulationStates {
            clock: GameClock::new(),
            manufacturing: ManufacturingState::new(),
            missions: MissionState::new(),
            events: EventState::new(),
            ai: AIState::new(AiFaction::Empire),
            ai2: None,
            movement: MovementState::new(),
            fog: FogState::new(Faction::Alliance),
            blockade: BlockadeState::new(),
            uprising: UprisingState::new(),
            death_star: DeathStarState::default(),
            research: ResearchState::new(),
            jedi: JediState::new(),
            victory: VictoryState::new(origin, target),
            betrayal: BetrayalState::new(),
            economy: EconomyState::default(),
            repair: RepairState::default(),
            troop_transport: TroopTransportState::default(),
            deliveries: DeliveryState::default(),
            stockpiles: StockpileState::default(),
            campaign_config: CampaignConfig::default(),
        };
        states
            .troop_transport
            .embark(&mut world, fleet, &[troop])
            .unwrap();
        states.campaign_config.victory_conditions = VictoryConditions::HeadquartersOnly;
        assert!(begin_fleet_transit(
            &mut states.movement,
            &mut world,
            fleet,
            target,
            1,
        ));

        let events = run_simulation_tick(
            &mut world,
            &mut states,
            &[TickEvent { tick: 1 }],
            &[0.99; 2048],
            10,
            &rebellion_core::tuning::GameConfig::default(),
        );

        assert!(states.troop_transport.is_empty());
        assert!(world.systems[target].ground_units.contains(&troop));
        assert_eq!(
            world.systems[target].control,
            ControlKind::Controlled(Faction::Alliance)
        );
        assert!(world.characters[prisoner].is_captive);
        assert_eq!(
            world.characters[prisoner].captured_by,
            Some(Faction::Alliance)
        );
        assert!(states.victory.resolved);
        for event_type in [
            EVT_FLEET_ARRIVED,
            EVT_TROOP_MOVED,
            EVT_CONTROL_CHANGED,
            EVT_CAPTURE,
            EVT_VICTORY,
        ] {
            assert!(
                events.iter().any(|event| event.event_type == event_type),
                "missing {event_type} telemetry"
            );
        }
    }
}
