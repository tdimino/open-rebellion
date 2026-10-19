//! The computer player's round scheduler (`FUN_00485b40`, `FUN_004be980`,
//! `FUN_004bede0`; `ghidra/notes/ai-order-emission.md`, "Scheduler and
//! executor").
//!
//! A round takes the census (state 4), re-arms every module (state 5) and
//! steps the armed modules (state 3): the priority pick first, else the
//! round-robin cursor, each module at most once, until a module finishes
//! with an order, finishes after a record change, or none is left armed.
//! Every module sets done with its order, so a round issues at most one,
//! and a side at most `DAY_ORDERS` a day.

use serde::{Deserialize, Serialize};

use crate::ai::{AIAction, AIState, AISystem, AiFaction};
use crate::ai_production::{Module, Round, RoundInputs, Turn};
use crate::ai_research::Pseudo;
use crate::manufacturing::ManufacturingState;
use crate::mission_planning::Planner;
use crate::research::{ResearchState, ResearchSystem, TechType};
use crate::tuning::GameConfig;
use crate::world::GameWorld;

/// The orders a side issues a day (`+0x428`, `FUN_00485a50.c:35`): the
/// default of `AiConfig::day_orders`.
pub const DAY_ORDERS: u8 = 10;

/// The modules of the list `+0x64` (`FUN_00485a50.c:21-35`), by kind.
const KINDS: u8 = 14;

/// One side's scheduler state (`FUN_00485b40` and the list's cursors).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Scheduler {
    /// `+0x10`: the kind the round-robin last took (`FUN_004be980`); 0 none.
    pub cursor: u8,
    /// `+0x14`: where the priority scan starts (`FUN_004bede0`); 0 none.
    pub scan: u8,
    /// `m+0x34` of kinds 1 to 14, by kind less one.
    pub priority: [u32; 14],
    /// `+0x420`: the orders issued today, counted at issue
    /// (`FUN_00485b40.c:45-48`).
    pub orders_today: u16,
    /// The in-day step of the day's last round so far (`FUN_0051dea0`
    /// `+0x20`); `end_day` resets it to 0.
    pub round_in_day: u16,
    /// port: the day ended since the last round, so a refusal reported now
    /// belongs to a counter `end_day` already cleared, and the next round
    /// is the day's first.
    pub carried: bool,
    /// The order of this side's last round, applied at the next in-day step
    /// (phase 3 hands it to list `+0xa4`; phase 5 executes it,
    /// `FUN_00436250`, `FUN_0051df30`).
    pub pending: Option<Issued>,
    /// port: the day's tick ran (`FUN_0051dea0` counts its steps from then),
    /// so the next tick closes it even when its first round never ran, as
    /// when a battle held that round past the day's end.
    pub day_started: bool,
}

impl Default for Scheduler {
    /// A new campaign has no open day: its first round is the day's first.
    /// `priority` holds `m+0x34` of the 14 modules (`FUN_00485a50.c:21-35`).
    fn default() -> Self {
        Self {
            cursor: 0,
            scan: 0,
            priority: [0; 14],
            orders_today: 0,
            round_in_day: 0,
            carried: true,
            pending: None,
            day_started: false,
        }
    }
}

impl Scheduler {
    /// Whether a day is open: its tick ran or a round ran since the last
    /// `end_day`, so the next tick's prologue has rounds to finish.
    #[must_use]
    pub const fn day_open(&self) -> bool {
        self.day_started || !self.carried
    }

    /// The in-day step whose round runs next (`FUN_0051dea0` `+0x20`).
    #[must_use]
    pub const fn next_step(&self) -> u16 {
        if self.carried {
            0
        } else {
            self.round_in_day + 1
        }
    }
}

/// What one round reads.
pub struct AiInputs<'a> {
    pub world: &'a GameWorld,
    pub manufacturing: &'a ManufacturingState,
    pub movement: &'a crate::movement::MovementState,
    pub config: &'a GameConfig,
    pub research: &'a ResearchState,
    pub uprisings: &'a crate::uprising::UprisingState,
    pub transport: &'a crate::troop_transport::TroopTransportState,
    /// The side's running missions, for the Diplomacy planner's claim.
    pub missions: &'a crate::missions::MissionState,
    pub tick: u64,
    /// The in-day step, 0 to the day's rate (`FUN_0051dea0` `+0x20`).
    pub round_in_day: u16,
}

/// The order a round issued and the kind of the module that issued it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Issued {
    pub kind: u8,
    pub action: AIAction,
    /// The day and the in-day step of the round that issued it.
    pub day: u64,
    pub round: u16,
}

/// How a round ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum End {
    /// `+0x420` reached `+0x428` (`FUN_00485b40.c:21`).
    Capped,
    /// A module finished with an order (`:59`).
    Order,
    /// port: a bundled Destination and Build's second round.
    Spent,
    /// A module finished after a record change (`c+4 & 0x80000000`, `:59`).
    Dirty,
    /// No armed module was left (`:24-29`, `:65-67`).
    Lap,
}

/// The kind after `kind` in the list, the head after the last or none
/// (`FUN_005f5060` leftmost, `+0x10` successor).
const fn succ(kind: u8) -> u8 {
    if kind == 0 || kind >= KINDS {
        1
    } else {
        kind + 1
    }
}

/// The production module of a kind (`FUN_00485a50.c:21-35`).
const fn production_module(kind: u8) -> Option<Module> {
    match kind {
        1 => Some(Module::Yards),
        2 => Some(Module::DeficitRelief),
        3 => Some(Module::Defences),
        5 => Some(Module::SpecialForces),
        6 => Some(Module::Regiments),
        7 => Some(Module::SystemFighters),
        10 => Some(Module::FleetFighters),
        11 => Some(Module::CapitalShips),
        _ => None,
    }
}

impl Scheduler {
    /// The next module of the round and whether the priority scan chose it
    /// (`FUN_004be980`).
    ///
    /// `FUN_004bede0.c:12-17` moves the scan start one module on at every
    /// call; from there one lap takes the armed module with the strictly
    /// largest `m+0x34` above 0, the first on a tie (`:18-32`), and leaves
    /// the cursor. Otherwise the cursor moves on until an armed module, and
    /// an empty lap leaves it one past where it was (`FUN_004be980.c:12-37`).
    fn pick(&mut self, armed: &[bool; 14]) -> Option<(u8, bool)> {
        self.scan = succ(self.scan);
        let mut best = None;
        let mut most = 0;
        let mut kind = self.scan;
        for _ in 0..KINDS {
            let at = usize::from(kind - 1);
            if armed[at] && self.priority[at] > most {
                most = self.priority[at];
                best = Some(kind);
            }
            kind = succ(kind);
        }
        if let Some(kind) = best {
            return Some((kind, true));
        }
        self.cursor = succ(self.cursor);
        let start = self.cursor;
        loop {
            if armed[usize::from(self.cursor - 1)] {
                return Some((self.cursor, false));
            }
            self.cursor = succ(self.cursor);
            if self.cursor == start {
                return None;
            }
        }
    }
}

impl AISystem {
    /// One round for the side (`FUN_00485b40` states 4, 5 and 3): the
    /// order it issued, if any.
    pub fn round(state: &mut AIState, inputs: &AiInputs<'_>) -> Option<Issued> {
        let faction = state.faction?;
        state.scheduler.carried = false;
        state.scheduler.round_in_day = inputs.round_in_day;
        // The cap (+0x428) is the --config AI tuning's, the original's 10
        // by default.
        if state.scheduler.orders_today >= inputs.config.ai.day_orders {
            // port: the census is skipped too; nothing the AI reads changes
            // within the day's capped rounds.
            trace_round(state, faction, inputs, &[], None, End::Capped, None);
            return None;
        }
        let world = inputs.world;
        let side: crate::dat::Faction = faction.as_mission_faction().into();
        let alliance = matches!(faction, AiFaction::Alliance);

        // State 4: the census (FUN_00418500).
        let marks = state.marks.clone();
        let round_inputs = RoundInputs {
            world,
            manufacturing: inputs.manufacturing,
            transport: inputs.transport,
            uprisings: inputs.uprisings,
            research: inputs.research,
            movement: inputs.movement,
            marks: &marks,
            side,
            tick: inputs.tick,
            round_in_day: inputs.round_in_day,
        };
        let mut round = Round::new(&state.production, &round_inputs);
        round.ctx = state.strategy.flags;
        round.task_of = state
            .character_tasks
            .tasks
            .iter()
            .filter_map(|t| Some((t.character?, t.id)))
            .collect();
        // Module 11 reads the regional records' need bits (rec+0x64), which
        // module 8's turns refresh below.
        round.record_needs = state.regions.records.iter().map(|r| r.need).collect();
        round.sabotage = state.regions.sabotage_requests(world, inputs.missions, side);
        let exhausted = TechType::ALL
            .map(|tech| ResearchSystem::tree_exhausted(world, inputs.research, alliance, tech));
        // port: one planner a round; its stream is the day's for round 0.
        let mut planner = Planner::new(
            world,
            &state.busy_characters,
            alliance,
            inputs.tick + (u64::from(inputs.round_in_day) << 32),
            exhausted,
        );
        planner.claim(state.character_tasks.held());
        planner.claim(state.research_planners.held());

        // State 5: every module re-armed (FUN_004cc990).
        let mut armed = [true; 14];
        // port: module 14, the renames, is not ported.
        armed[13] = false;

        // State 3: the steps.
        let mut visited = [0u8; 14];
        let mut steps = 0;
        let mut ended = (End::Lap, None);
        let mut issued = None;
        while let Some((kind, priority_pick)) = state.scheduler.pick(&armed) {
            let at = usize::from(kind - 1);
            state.scheduler.priority[at] = 0;
            visited[steps] = kind;
            steps += 1;
            let turn = match kind {
                4 => state.research_planners.turn(
                    &mut round,
                    &mut planner,
                    faction,
                    |pseudo, planner, round| {
                        // The pseudo-planner's pass: one dispatch at most.
                        let mut actions = Vec::new();
                        let missions = inputs.missions;
                        match pseudo {
                            Pseudo::Officers => AISystem::evaluate_officers(
                                planner,
                                round,
                                missions,
                                faction,
                                1,
                                &mut actions,
                            ),
                            Pseudo::Espionage => AISystem::evaluate_espionage(
                                planner,
                                round,
                                faction,
                                1,
                                &mut actions,
                            ),
                            Pseudo::Rescue => {
                                AISystem::evaluate_rescue(planner, round, faction, 1, &mut actions);
                            }
                            Pseudo::Reconnaissance => AISystem::evaluate_reconnaissance(
                                planner,
                                round,
                                faction,
                                1,
                                &mut actions,
                            ),
                            Pseudo::UprisingPrevention => AISystem::evaluate_uprising_prevention(
                                planner,
                                round,
                                missions,
                                faction,
                                1,
                                &mut actions,
                            ),
                        }
                        actions.into_iter().next()
                    },
                ),
                8 => {
                    let turn = state.regions.turn(&mut round, &mut state.marks);
                    // Module 11 reads the live records (rec+0x64,
                    // ai_production.rs `capital_ships`), as module 8 leaves them.
                    round.record_needs = state.regions.records.iter().map(|r| r.need).collect();
                    round.sabotage = state.regions.sabotage_requests(world, inputs.missions, side);
                    turn
                }
                9 => state.fleet_tasks.turn(&mut round, &mut state.marks),
                12 => {
                    state
                        .character_tasks
                        .turn(&mut round, &mut state.regions, &mut planner, side)
                }
                13 => {
                    // Module 13 (FUN_004bf320): the mode pick and the
                    // controller, then slot 8 (FUN_004e69d0) sizes the
                    // fleet tasks to c+0x224.
                    state.strategy.cycle(
                        &mut round,
                        &mut state.marks,
                        &mut state.regions,
                        state.production.token.pending,
                    );
                    round.ctx = state.strategy.flags;
                    let headquarters = crate::ai_strategy::Sums::of(&round).headquarters;
                    state
                        .fleet_tasks
                        .size(&mut round, &mut state.marks, headquarters);
                    // The controller's planner request (FUN_004e5710),
                    // which module 4's next turn consumes.
                    state.research_planners.request(&planner, exhausted);
                    Turn::Finished
                }
                _ => match production_module(kind) {
                    Some(module) => state.production.turn(module, &mut round),
                    None => Turn::Finished,
                },
            };
            // A finished module is disarmed (`:57`).
            armed[at] = false;
            match turn {
                Turn::Order { action, resume } => {
                    if resume {
                        state.scheduler.priority[at] += 1;
                    }
                    state.scheduler.orders_today = state.scheduler.orders_today.saturating_add(1);
                    ended = (End::Order, Some((kind, priority_pick, resume)));
                    issued = Some(Issued {
                        kind,
                        action,
                        day: inputs.tick,
                        round: inputs.round_in_day,
                    });
                    break;
                }
                Turn::Spent => {
                    state.scheduler.orders_today = state.scheduler.orders_today.saturating_add(1);
                    ended = (End::Spent, Some((kind, priority_pick, false)));
                    break;
                }
                Turn::Finished if round.dirty => {
                    ended = (End::Dirty, Some((kind, priority_pick, false)));
                    break;
                }
                Turn::Finished => {}
            }
        }

        // The census's own bookkeeping.
        for record in round.records.values() {
            if record.last_seen != 0 {
                state.marks.last_seen.insert(record.key, record.last_seen);
            }
        }
        state.marks.prune(world);
        trace_round(
            state,
            faction,
            inputs,
            &visited[..steps],
            ended.1,
            ended.0,
            issued.as_ref(),
        );
        issued
    }

    /// The applier's verdict on the order `kind` issued. A refused order
    /// runs the module's slot 6 (`FUN_004dd810.c:5-8`: done, phase, step
    /// and `m+0x34` cleared) and was never counted (`FUN_00485b40.c:40-41`).
    pub fn settle(state: &mut AIState, kind: u8, accepted: bool) {
        if accepted || !(1..=KINDS).contains(&kind) {
            return;
        }
        match kind {
            8 => state.regions.reject(),
            9 => state.fleet_tasks.reject(),
            // Modules 4 and 12 run a planner's or a task's pass whole, so
            // each turn starts at state 1 and slot 6 has no step to clear.
            _ => {
                if let Some(module) = production_module(kind) {
                    state.production.reject(module);
                }
            }
        }
        state.scheduler.priority[usize::from(kind - 1)] = 0;
        if !state.scheduler.carried {
            state.scheduler.orders_today = state.scheduler.orders_today.saturating_sub(1);
        }
    }

    /// The day's end (`FUN_00485de0.c:7-11`, from phase 13 through
    /// `FUN_0041d050` -> `FUN_00436530`): the order count clears.
    pub fn end_day(state: &mut AIState) {
        state.scheduler.orders_today = 0;
        state.scheduler.round_in_day = 0;
        state.scheduler.carried = true;
        state.scheduler.day_started = false;
    }
}

/// port: tooling. One `ai/round` record per round.
fn trace_round(
    state: &AIState,
    faction: AiFaction,
    inputs: &AiInputs<'_>,
    visited: &[u8],
    ender: Option<(u8, bool, bool)>,
    end: End,
    issued: Option<&Issued>,
) {
    crate::trace::record(crate::trace::TraceMask::AI, "round", || {
        let s = &state.scheduler;
        serde_json::json!({
            "side": format!("{faction:?}"),
            "tick": inputs.tick,
            "round_in_day": inputs.round_in_day,
            "visited": visited,
            "kind": ender.map(|(kind, _, _)| kind),
            "priority_pick": ender.map(|(_, pick, _)| pick),
            "result": match end {
                End::Capped => "capped",
                End::Order => "order",
                End::Spent => "spent",
                End::Dirty => "dirty",
                End::Lap => "lap",
            },
            "resume": ender.filter(|_| end == End::Order).map(|(_, _, resume)| resume),
            "order": issued.map(|i| format!("{:?}", i.action)),
            "orders_today": s.orders_today,
            "cursor": s.cursor,
            "scan": s.scan,
        })
    });
}
