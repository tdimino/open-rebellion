//! Shared simulation tick function for headless and interactive use.
//!
//! Extracts the per-tick system-advance logic from `rebellion-app/main.rs`
//! into a reusable function that returns structured `GameEventRecord`s
//! instead of pushing to a `MessageLog`.

use crate::integrator::{resolve_system_space_combat_for, PerceptionIntegrator};
use rebellion_core::ai::{AIAction, AIState, AISystem};
use rebellion_core::betrayal::{BetrayalState, BetrayalSystem};
use rebellion_core::blockade::{BlockadeState, BlockadeSystem};
use rebellion_core::combat::side_present;
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
    pub deliveries: &'a mut DeliveryState,
    pub stockpiles: &'a mut StockpileState,
    pub victory: &'a mut VictoryState,
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
    pub assault: crate::integrator::AssaultEffects,
    /// What phase 5 served after any other applied order: a moved
    /// regiment's holder rerun (`FUN_00508660`) and its shifts.
    pub fired: rebellion_core::support_shift::Fired,
    /// The trips of the characters its holder changes sent off
    /// (`integrator::sweep_requests`).
    pub swept: Vec<(
        rebellion_core::ids::SystemKey,
        Vec<crate::integrator::AdriftTrip>,
    )>,
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
    use crate::integrator::{
        apply_ai_order, resolve_ai_assault, resolve_ai_strike, AiOutcome, AssaultEffects,
        StrikeEffects,
    };
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
        let mut assault = AssaultEffects::default();
        let mut fired = rebellion_core::support_shift::Fired::default();
        let mut swept = Vec::new();
        let (outcome, strike) = if let Some(system) = held(ctx.world, ctx.transport, &issued.action)
        {
            (AiOutcome::Held { system }, StrikeEffects::default())
        } else if matches!(
            issued.action,
            AIAction::Bombard { .. } | AIAction::DestroySystem { .. } | AIAction::Assault { .. }
        ) {
            let mut states = crate::integrator::StrikeStates {
                movement: ctx.movement,
                manufacturing: ctx.manufacturing,
                transport: ctx.transport,
                deliveries: ctx.deliveries,
                missions: ctx.missions,
                uprisings: ctx.uprising,
                blockade: ctx.blockade,
                tick: day,
                step,
            };
            let mut roll = || rolls(1).first().copied().unwrap_or(0.0);
            let side = state.faction.map_or(rebellion_core::dat::Faction::Empire, |faction| {
                faction.as_mission_faction().into()
            });
            if matches!(issued.action, AIAction::Assault { .. }) {
                let (outcome, effects) =
                    resolve_ai_assault(&issued.action, side, ctx.world, &mut states, &mut roll);
                assault = effects;
                (outcome, StrikeEffects::default())
            } else {
                resolve_ai_strike(&issued.action, side, ctx.world, &mut states, &mut roll)
            }
        } else {
            let outcome = apply_ai_order(
                &issued.action,
                state,
                ctx.missions,
                ctx.manufacturing,
                ctx.deliveries,
                ctx.movement,
                ctx.transport,
                ctx.stockpiles,
                ctx.world,
                ctx.uprising,
                day,
            );
            // The order runs in this step's phase 5, which serves its events
            // (FUN_0051df30).
            fired = crate::integrator::fire_holder_changes(
                ctx.world,
                ctx.manufacturing,
                rebellion_core::support_shift::step(day, step),
            );
            swept = crate::integrator::sweep_requests(
                ctx.world,
                ctx.missions,
                ctx.transport,
                ctx.movement,
                ctx.blockade,
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
            assault,
            fired,
            swept,
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
        deliveries: &mut states.deliveries,
        stockpiles: &mut states.stockpiles,
        victory: &mut states.victory,
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
        world,
        manufacturing,
        transport,
        blockade,
        ..
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
            integrator.emit_assault(world, &record.assault);
            integrator.emit_fired(world, &record.fired.shifts, &record.fired.controls);
            integrator.emit_swept(world, &record.swept);
        }
        integrator.apply_blockade_events(world, manufacturing, transport, blockade, &runs);
    });
}

/// The characters the operation's holder changes flagged leave (event
/// `0x307`, `integrator::sweep_requests`), with their trips' records.
fn sweep_requests_emitting(
    world: &mut GameWorld,
    states: &mut SimulationStates,
    integrator: &mut PerceptionIntegrator,
    tick: u64,
) {
    let swept = crate::integrator::sweep_requests(
        world,
        &mut states.missions,
        &states.troop_transport,
        &states.movement,
        &states.blockade,
        tick,
    );
    integrator.emit_swept(world, &swept);
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
    let economy_events = EconomySystem::advance(&mut states.economy, world, tick_events);
    integrator.apply_economy_events(world, &economy_events);

    // ── 0b. Stockpiles: facility cycles, yard work, overdraft scraps ─────
    let stockpile_events = StockpileSystem::advance(
        &mut states.stockpiles,
        world,
        &mut states.manufacturing,
        tick_events,
        states.blockade.blockaded_systems(),
        &take_rolls(tick_events.len()),
    );
    integrator.emit_maintenance_shortfall(world, &states.troop_transport, &stockpile_events);
    let overdraft = rebellion_core::scrap::scrap_all_reporting(
        world,
        &mut states.stockpiles,
        &mut states.troop_transport,
        stockpile_events.iter().filter_map(|event| match *event {
            StockpileEvent::OverdraftScrap { target, .. } => Some(target),
            _ => None,
        }),
    );
    integrator.emit_scrapped_aboard(world, &overdraft.losses);
    // A scrapped regiment's holder rerun (FUN_00508660).
    integrator.fire_support_shifts(world, &mut states.manufacturing);

    // ── 1. Manufacturing ─────────────────────────────────────────────────
    // Use advance_tracked so the K6 EVT_MANUFACTURING_IDLE telemetry is
    // emitted for any system whose queue transitioned empty this tick.
    let mfg_advance = ManufacturingSystem::advance_tracked(
        &mut states.manufacturing,
        tick_events,
        states.blockade.blockaded_systems(),
    );
    // An ordered capital ship completes in place in its fleet (FUN_0052bee0).
    rebellion_core::manufacturing::finish_ships(
        world,
        &states.movement,
        &mfg_advance.finished_ships,
    );
    integrator.apply_build_completions(
        world,
        &mut states.manufacturing,
        &mfg_advance.completions,
    );
    integrator.apply_manufacturing_idle(world, &mfg_advance.newly_idle);
    // Remote products travel, then complete at their destination (F-030);
    // a capital ship travels on its own leg inside its destination fleet.
    let travelling = crate::integrator::travelling_products(&mfg_advance.departures);
    states.deliveries.depart(world, &travelling);
    let delivered = states.deliveries.advance(world, tick_events);
    integrator.apply_build_completions(
        world,
        &mut states.manufacturing,
        &delivered.arrivals,
    );
    integrator.apply_boarded(
        world,
        &mut states.manufacturing,
        &mut states.troop_transport,
        &delivered.boarded,
    );
    integrator.apply_deliveries_lost(world, &delivered.lost);
    let ships = rebellion_core::delivery::advance_ship_legs(world, &states.movement, tick_events);
    integrator.apply_ship_arrivals(world, &ships);
    for arrival in &delivered.arrivals {
        if matches!(
            arrival.kind,
            rebellion_core::manufacturing::BuildableKind::SpecialForce(_)
        ) {
            rebellion_core::death_star::deploy_special_forces(
                world,
                &states.troop_transport,
                &states.movement,
                arrival.system,
            );
        }
    }
    // Ordered ships whose queue entries are gone.
    rebellion_core::manufacturing::seat_ordered_ships(world, &mut states.manufacturing);

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
    integrator.fire_support_shifts(world, &mut states.manufacturing);
    sweep_requests_emitting(world, states, &mut integrator, current_tick);

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
            Some(&states.manufacturing),
            &rebellion_core::carriage::ProductRoom::new(&states.manufacturing, &states.deliveries),
        );

        integrator.emit_system_space_combat(world, &space_result);
    }

    let destroyed_cargo = states.troop_transport.destroy_untransportable_cargo(world);
    integrator.emit_destroyed_transport_cargo(&destroyed_cargo);

    // No regiment lands without an order: the assault (0x234) or a Move
    // (0x201) puts one down (ghidra/notes/ground-assault.md, "Landing without
    // the order").

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

    // The blockade rule reruns as an arrival or a battle settles, before the
    // AI's orders take effect (`FUN_0050b8e0` through event 0x311,
    // `ghidra/notes/blockade-bit.md`, "When it runs"): a fleet the AI sends on
    // the day it arrives still blockades for that day.
    let mut blockade_events = BlockadeSystem::advance(&mut states.blockade, world, tick_events);
    // The same rule writes the battle bit; each change sweeps the system's
    // adrift characters (event 0x304, FUN_00548e90).
    let adrift_trips = tick_events.last().map_or_else(Vec::new, |last| {
        rebellion_core::death_star::sweep_battle_changes(
            world,
            &mut states.missions,
            &mut states.blockade,
            &states.troop_transport,
            &states.movement,
            last.tick,
        )
    });
    for (system, started) in &adrift_trips {
        integrator.emit_adrift_trips(world, &states.missions, *system, started);
    }
    sweep_requests_emitting(world, states, &mut integrator, current_tick);

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
        integrator.apply_mission_result(
            world,
            &mut states.manufacturing,
            result,
            &mut states.uprising,
            &mut states.research,
        );
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

    sweep_requests_emitting(world, states, &mut integrator, current_tick);

    // ── 5a. Members going home from ended missions (FUN_00548120) ────────
    for ended in &mission_advance.ended {
        let started = rebellion_core::death_star::return_members(
            world,
            &mut states.missions,
            &states.troop_transport,
            &states.movement,
            &states.blockade,
            ended,
            current_tick,
        );
        if let Some(system) = ended.origin {
            integrator.emit_adrift_trips(world, &states.missions, system, &started);
        }
    }

    // ── 5b. Character escapes ────────────────────────────────────────────
    let escape_roll = take_rolls(1)[0];
    let escape_effects = MissionSystem::escape_step(world, current_tick, escape_roll);
    integrator.apply_escape_effects(world, &escape_effects);
    // Each release's autoscrap request sweeps the escapee's location at once
    // (event 0x307, FUN_00548da0).
    for effect in &escape_effects {
        if let rebellion_core::missions::MissionEffect::CharacterEscaped { character, .. } = effect
        {
            let started = rebellion_core::death_star::free_escapee(
                world,
                &mut states.missions,
                &states.troop_transport,
                &states.movement,
                &states.blockade,
                *character,
                current_tick,
            );
            if let Some(system) = world.characters.get(*character).and_then(|c| c.current_system) {
                integrator.emit_adrift_trips(world, &states.missions, system, &started);
            }
        }
    }

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
    integrator.apply_blockade_events(
        world,
        &mut states.manufacturing,
        &mut states.troop_transport,
        &states.blockade,
        &blockade_events,
    );
    sweep_requests_emitting(world, states, &mut integrator, current_tick);

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
    integrator.apply_uprising_events(world, &mut states.manufacturing, &uprising_events);
    sweep_requests_emitting(world, states, &mut integrator, current_tick);

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
    let word = (!tick_events.is_empty() && !states.victory.resolved)
        .then(|| {
            VictorySystem::word(&states.victory, world, states.campaign_config.victory_conditions)
        })
        .flatten();
    if let Some(outcome) = VictorySystem::check(
        &states.victory,
        world,
        tick_events,
        states.campaign_config.victory_conditions,
    ) {
        integrator.apply_victory(&outcome, &mut states.victory, world);
    }
    // The word the next evaluation's tie reads (FUN_005180e0).
    if let Some(word) = word {
        states.victory.word = word;
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
    use rebellion_core::carriage::ProductRoom;
    use rebellion_core::dat::Faction;
    use rebellion_core::fog::FogState;
    use rebellion_core::game_events::{
        EVT_ASSAULT, EVT_BOMBARDMENT, EVT_CONTROL_CHANGED, EVT_FLEET_ARRIVED, EVT_VICTORY,
    };
    use rebellion_core::ids::SectorKey;
    use rebellion_core::ids::{DatId, FleetKey, TroopKey};
    use rebellion_core::movement::begin_fleet_transit;
    use rebellion_core::world::{
        CapitalShipClass, Character, ControlKind, Fleet, ShipInstance, System, TroopUnit,
        VictoryConditions,
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
            bombarding: false,
        });
        world.systems[system].fleets.push(fleet);
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(20),
            is_alliance: true,
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
    fn a_regiment_aboard_stays_aboard_until_an_order_puts_it_down() {
        // No landing without the order: the landing callback FUN_0058be10 is
        // reached only from the assault (ghidra/notes/ground-assault.md,
        // "Landing without the order").
        let (mut world, mut states, fleet, troop) = world_with_loading_fleet();
        let system = world.fleets[fleet].location;
        states
            .troop_transport
            .load(&mut world, &ProductRoom::default(), fleet, &[troop])
            .unwrap();

        run_one_tick(&mut world, &mut states);

        assert_eq!(states.troop_transport.cargo(fleet), [troop]);
        assert!(!world.systems[system].ground_units.contains(&troop));
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

    /// A special-force product deployed into a destroyed system has no leg
    /// (`FUN_005150c0.c:76` -> `FUN_00545180`); the presence check
    /// `FUN_00556710` gives it the autoscrap request (`FUN_00503e20(0)`),
    /// whose event `0x307` runs the adrift sweep: a system of its side takes
    /// it, else `FUN_00549580.c:56-60` destroys it with reason 9
    /// (`death-star-strategic.md`, "Arrival at a destroyed system").
    #[test]
    fn a_special_force_deployed_into_a_destroyed_system_leaves_it_the_same_day() {
        let class = DatId::new(0x3c00_0001);
        for is_alliance in [true, false] {
            for refuge_is_own in [true, false] {
                let (mut world, mut states) = make_test_states();
                let systems: Vec<_> = world.systems.keys().collect();
                let (refuge, target) = (systems[0], systems[1]);
                let refuge_side = Faction::of_alliance(is_alliance == refuge_is_own);
                world.systems[refuge].control = ControlKind::Controlled(refuge_side);
                world.systems[target].is_destroyed = true;
                world.buildable_classes.insert(
                    class,
                    rebellion_core::world::BuildableClass {
                        is_alliance,
                        ..Default::default()
                    },
                );
                states.deliveries.depart(
                    &world,
                    &[rebellion_core::manufacturing::Departure {
                        origin: refuge,
                        destination: rebellion_core::manufacturing::Destination::System(target),
                        tick: 1,
                        kind: rebellion_core::manufacturing::BuildableKind::SpecialForce(class),
                        carrier: 0,
                    }],
                );

                run_one_tick(&mut world, &mut states);

                let case = format!("{is_alliance} {refuge_is_own}");
                assert!(world.systems[target].special_forces.is_empty(), "{case}");
                assert_eq!(
                    world.systems[refuge].special_forces.len(),
                    usize::from(refuge_is_own),
                    "{case}"
                );
                assert_eq!(world.special_forces.len(), usize::from(refuge_is_own), "{case}");
            }
        }
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

    /// The assault's landing limit, GNPRTB 7761 (60) and 7762 (-10):
    /// `FUN_00559fe0(side, 0)` = 6, and its shield limit GNPRTB 0x1e03 (2,
    /// `FUN_00507d00`); with `threshold`, also GNPRTB 7760 (60),
    /// the support that holds a system without regiments (FUN_0055a080).
    fn assault_gnprtb(threshold: bool) -> rebellion_core::world::GnprtbParams {
        let entry = |id: u32, value: i32| rebellion_core::world::GnprtbEntry {
            parameter_id: id,
            development: value,
            alliance_sp_easy: value,
            alliance_sp_medium: value,
            alliance_sp_hard: value,
            empire_sp_easy: value,
            empire_sp_medium: value,
            empire_sp_hard: value,
            multiplayer: value,
        };
        let mut entries = vec![entry(7761, 60), entry(7762, -10), entry(0x1e03, 2)];
        if threshold {
            entries.push(entry(7760, 60));
        }
        rebellion_core::world::GnprtbParams::new(entries)
    }

    /// The player's Planetary Assault by `fleet` on the simulation's states
    /// (`integrator::order_assault`).
    fn assault(
        world: &mut GameWorld,
        states: &mut SimulationStates,
        fleet: FleetKey,
        side: Faction,
    ) -> Vec<crate::integrator::AssaultEffects> {
        crate::integrator::order_assault(
            world,
            &[fleet],
            side,
            &mut crate::integrator::StrikeStates {
                movement: &mut states.movement,
                manufacturing: &mut states.manufacturing,
                transport: &mut states.troop_transport,
                deliveries: &mut states.deliveries,
                missions: &mut states.missions,
                uprisings: &states.uprising,
                blockade: &mut states.blockade,
                tick: 1,
                step: 0,
            },
            &mut || 0.5,
        )
        .expect("the blockading fleet may assault")
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
    fn a_decisive_imperial_space_win_bombards_nothing_without_the_order() {
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
            bombarding: false,
        });
        let empire_fleet = world.fleets.insert(Fleet {
            location: alliance_hq,
            capital_ships: vec![ShipInstance::new(empire_class, 1_000, false)],
            fighters: vec![],
            characters: vec![],
            is_alliance: false,
            bombarding: false,
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

        // FUN_0058d800's one caller is message 0x224 (FUN_005637c0): the
        // original bombards only on the order, never after a won battle.
        assert!(world.systems[alliance_hq].is_headquarters);
        assert!(!states.victory.resolved);
        assert!(events
            .iter()
            .all(|event| event.event_type != EVT_BOMBARDMENT));
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
    fn an_imperial_fleet_takes_the_alliance_hq_only_by_the_assault_order() {
        let mut world = GameWorld {
            gnprtb: assault_gnprtb(false),
            ..GameWorld::default()
        };
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
            bombarding: false,
        });
        world.systems[origin].fleets.push(fleet);
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(20),
            is_alliance: false,
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
            .embark(&mut world, &ProductRoom::default(), fleet, &[troop])
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

        // The arrival lands nothing and bombards nothing: the landing and
        // FUN_0058d800 run only on their orders.
        assert_eq!(states.troop_transport.cargo(fleet), [troop]);
        assert_eq!(
            world.systems[alliance_hq].control,
            ControlKind::Controlled(Faction::Alliance)
        );
        assert!(events
            .iter()
            .all(|event| event.event_type != EVT_BOMBARDMENT));

        // The fleet blockades the HQ (FUN_0050b8e0), so the order passes and
        // the landing takes the undefended system (FUN_0058c810,
        // FUN_0050b610).
        let runs = assault(&mut world, &mut states, fleet, Faction::Empire);
        assert_eq!(runs[0].report.as_ref().unwrap().landed, [troop]);
        assert_eq!(
            world.systems[alliance_hq].control,
            ControlKind::Controlled(Faction::Empire)
        );
        // The capture removes the HQ: the control write hands each object
        // to the new holder or destroys it with reason 3 (FUN_00510d70 ->
        // FUN_004f6f40 -> FUN_004f8680), and the HQ's class serves only the
        // Alliance (game-end-path.md, "Control change"; FUN_00517cd0 then
        // shifts its sector). Coruscant keeps its flag.
        assert!(!world.systems[alliance_hq].is_headquarters);
        assert!(world.systems[coruscant].is_headquarters);
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
    fn an_alliance_fleet_takes_coruscant_by_the_assault_and_captures_nobody() {
        let mut world = GameWorld {
            gnprtb: assault_gnprtb(true),
            ..GameWorld::default()
        };
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
        let refuge = add_system(
            &mut world,
            3,
            "Imperial Refuge",
            ControlKind::Controlled(Faction::Empire),
            false,
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
            bombarding: false,
        });
        world.systems[origin].fleets.push(fleet);
        let troop = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(20),
            is_alliance: true,
        });
        world.systems[origin].ground_units.push(troop);
        let officer = world.characters.insert(Character {
            dat_id: DatId::new(4),
            name: "Imperial Leader".into(),
            is_empire: true,
            current_system: Some(target),
            ..Character::default()
        });
        // FUN_005180e0 reads Coruscant and the four principal leaders by id.
        world.systems[target].dat_id = DatId::new(rebellion_core::victory::CORUSCANT);
        for id in [
            rebellion_core::victory::LUKE,
            rebellion_core::victory::MON_MOTHMA,
            rebellion_core::victory::VADER,
            rebellion_core::victory::PALPATINE,
        ] {
            world.characters.insert(Character {
                dat_id: DatId::new(id),
                current_system: Some(origin),
                ..Character::default()
            });
        }

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
            .embark(&mut world, &ProductRoom::default(), fleet, &[troop])
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

        // The arrival lands nothing (ground-assault.md, "Landing without
        // the order").
        assert_eq!(states.troop_transport.cargo(fleet), [troop]);
        assert_eq!(
            world.systems[target].control,
            ControlKind::Controlled(Faction::Empire)
        );
        assert!(events
            .iter()
            .any(|event| event.event_type == EVT_FLEET_ARRIVED));

        // The order lands the regiment and the holder turns; the owner
        // change sends the officer at Coruscant, free, to the nearest
        // Imperial system (FUN_004ef260, event 0x307; ground-assault.md,
        // section 4).
        let runs = assault(&mut world, &mut states, fleet, Faction::Alliance);
        assert!(states.troop_transport.is_empty());
        assert!(world.systems[target].ground_units.contains(&troop));
        assert_eq!(
            world.systems[target].control,
            ControlKind::Controlled(Faction::Alliance)
        );
        assert!(!world.characters[officer].is_captive);
        assert!(world.characters[officer].on_hidden_mission);
        assert!(states.missions.missions().iter().any(|mission| {
            mission.target_system == refuge
                && mission.team == vec![rebellion_core::missions::MissionMember::Character(officer)]
        }));
        let mut integrator = PerceptionIntegrator::new(1, 0);
        integrator.emit_assault(&world, &runs[0]);
        let records = integrator.finish();
        for event_type in [EVT_ASSAULT, EVT_CONTROL_CHANGED] {
            assert!(
                records.iter().any(|event| event.event_type == event_type),
                "missing {event_type} telemetry"
            );
        }
        // One record of the capture, and the officer's trip (event 0x307).
        let count = |event_type| {
            records
                .iter()
                .filter(|event| event.event_type == event_type)
                .count()
        };
        assert_eq!(count(EVT_CONTROL_CHANGED), 1);
        assert!(records.iter().any(|event| {
            event.event_type == rebellion_core::game_events::EVT_MISSION_DISPATCHED
                && event.details["kind"] == "Adrift"
                && event.details["target_system"] == "Imperial Refuge"
        }));

        // Coruscant held by the Alliance wins a Headquarters game (CA,
        // FUN_005180e0) at the next check.
        let events = run_simulation_tick(
            &mut world,
            &mut states,
            &[TickEvent { tick: 2 }],
            &[0.99; 2048],
            10,
            &rebellion_core::tuning::GameConfig::default(),
        );
        assert!(states.victory.resolved);
        assert!(events.iter().any(|event| event.event_type == EVT_VICTORY));
    }

    #[test]
    fn a_flagged_character_leaves_for_its_side_the_day_it_is_flagged() {
        // Event 0x307 sweeps the flagged character's system at once
        // (FUN_00515480 -> FUN_00545df0 -> FUN_00548da0): one at an enemy
        // system leaves for the nearest system of its side (FUN_00543040).
        for is_alliance in [true, false] {
            let own = if is_alliance {
                ControlKind::Controlled(Faction::Alliance)
            } else {
                ControlKind::Controlled(Faction::Empire)
            };
            let enemy = if is_alliance {
                ControlKind::Controlled(Faction::Empire)
            } else {
                ControlKind::Controlled(Faction::Alliance)
            };
            let mut world = GameWorld {
                gnprtb: support_control_threshold(),
                ..GameWorld::default()
            };
            let system = |control, x, support: f32| System {
                name: "System".into(),
                x,
                is_populated: true,
                popularity_alliance: support,
                popularity_empire: 1.0 - support,
                control,
                ..System::default()
            };
            let held = if is_alliance { 0.1 } else { 0.9 };
            let at = world.systems.insert(system(enemy, 100, held));
            let refuge = world.systems.insert(system(own, 300, 1.0 - held));
            let character = world.characters.insert(Character {
                is_alliance,
                is_empire: !is_alliance,
                current_system: Some(at),
                adrift: true,
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
                research: ResearchState::new(),
                jedi: JediState::new(),
                victory: VictoryState::new(refuge, at),
                betrayal: BetrayalState::new(),
                economy: EconomyState::default(),
                repair: RepairState::default(),
                troop_transport: TroopTransportState::default(),
                deliveries: DeliveryState::default(),
                stockpiles: StockpileState::default(),
                campaign_config: CampaignConfig::default(),
            };
            let events = run_simulation_tick(
                &mut world,
                &mut states,
                &[TickEvent { tick: 1 }],
                &[0.99; 2_048],
                10,
                &rebellion_core::tuning::GameConfig::default(),
            );
            assert!(
                world.characters[character].on_hidden_mission,
                "{is_alliance}"
            );
            assert!(states
                .missions
                .missions()
                .iter()
                .any(|mission| mission.target_system == refuge));
            assert!(events.iter().any(|event| {
                event.event_type == rebellion_core::game_events::EVT_MISSION_DISPATCHED
                    && event.details["kind"] == "Adrift"
            }));
        }
    }
}
