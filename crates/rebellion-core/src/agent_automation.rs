//! Player-side Agent automation: Manage Garrisons and Manage Production.
//!
//! The original game's Agent menu offers toggle-driven automation that
//! instructs the player's advisor to manage garrisons and production
//! autonomously. This module ports the sub-module dispatch system and the
//! two decision modules from `REBEXE.EXE`.
//!
//! # Architecture
//!
//! The computer opponent's AI (`crate::ai`) uses a **separate pipeline**
//! (`FUN_00519d00` chain) and shares no code with these player-side modules.
//! This module implements only the player-side Agent, as recovered from:
//!
//! - `FUN_00439320` (constructor)
//! - `FUN_00439950` (initialization — creates modules, sets state 2)
//! - `FUN_00439a10` (agent tick — state 4/5/6 cycle)
//! - `FUN_00439d60` (toggle mechanism)
//! - `FUN_00439e30` (is_running query)
//! - `FUN_004cc990` (VT\[4\] ready check)
//! - `FUN_004c6ce0` / `FUN_004c6f00` (Production decision logic)
//! - `FUN_004c7580` / `FUN_004c7650` / `FUN_004c7860` / `FUN_004c79e0`
//!   (Garrison decision logic)
//!
//! # State model
//!
//! Each sub-module has a `ModuleState`:
//! - `Suspended` (original state 2): the module is **off** and will not
//!   execute. `FUN_004cc990` (VT\[4\]) skips modules in state 2, so they
//!   never become ready. (`FUN_00439950` starts all modules in state 2.)
//! - `Disabled` (original state 0): transient; `VT[4]` promotes 0 → 1.
//! - `Ready` (original state 1): scheduled for execution this cycle.
//!
//! `FUN_00439d60` toggles between `Suspended` and `Disabled`:
//! - Suspended → Disabled = turn **ON** (counter++, bits 0x10/0x20).
//! - Not-Suspended → Suspended = turn **OFF** (counter--, bits 0x100/0x200).
//!
//! Evidence: `FUN_00439d60.c`, `FUN_00439e30.c`, `FUN_004cc990.c`,
//! `FUN_00439950.c`.

use serde::{Deserialize, Serialize};

use crate::economy::{EconomyState, SystemEconomy};
use crate::ids::{DatId, SystemKey};
use crate::manufacturing::{BuildableKind, FacilityBuild, ManufacturingState};
use crate::world::GameWorld;

// -------------------------------------------------------------------------
// Constants
// -------------------------------------------------------------------------

/// Maximum orders per agent cycle (`agent+0x18c`, `FUN_00439950`).
/// FUN_00439a10: `orders_issued < orders_limit` gates dispatch.
const ORDERS_LIMIT: u32 = 5; // FUN_00439950 line: param_1[99] = 5

// -------------------------------------------------------------------------
// Module identity
// -------------------------------------------------------------------------

/// The two automation modules exposed by the Agent menu.
/// Original module indices: 0x14 (Production), 0x15 (Garrisons).
/// (0x17 = Build Ships is out of scope.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AutomationModule {
    /// Module 0x14 — Manage Production (Alt+U, menu 0x116).
    /// Builds mines and refineries to balance raw/refined resources.
    Production,
    /// Module 0x15 — Manage Garrisons (Alt+G, menu 0x115).
    /// Trains troops to fill garrison deficits at under-garrisoned systems.
    Garrisons,
}

// -------------------------------------------------------------------------
// Module state
// -------------------------------------------------------------------------

/// Per-module execution state, matching the original's `+0x1c` field.
///
/// - `Suspended` = original state 2: module is OFF, `VT[4]` skips it.
/// - `Disabled`  = original state 0: transient idle; `VT[4]` promotes to `Ready`.
/// - `Ready`     = original state 1: scheduled for execution this cycle.
///
/// Evidence: `FUN_004cc990.c` (VT\[4\]), `FUN_00439d60.c` (toggle),
/// `FUN_00439950.c` (init sets all to state 2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModuleState {
    /// State 0: disabled/idle. `VT[4]` promotes this to `Ready`.
    Disabled,
    /// State 1: ready for execution this cycle.
    Ready,
    /// State 2: suspended (OFF). `VT[4]` never touches this.
    /// `FUN_00439950` initializes all modules to this state.
    Suspended,
}

impl Default for ModuleState {
    /// All modules start suspended (off). FUN_00439950.
    fn default() -> Self {
        ModuleState::Suspended
    }
}

// -------------------------------------------------------------------------
// Advisor flags
// -------------------------------------------------------------------------

/// Advisor message flag bits from `DAT_006b28b0`, scoped to the automation
/// toggle mechanism (`FUN_00439d60`).
///
/// The note's original table had ON/OFF bits swapped. Corrected mapping
/// (verified against `FUN_00439d60.c`):
/// - 0x10 = Garrisons turned ON (2→0 transition, `uVar4`)
/// - 0x20 = Production turned ON (2→0 transition, `uVar4`)
/// - 0x100 = Garrisons turned OFF (not-2→2 transition, `param_1`)
/// - 0x200 = Production turned OFF (not-2→2 transition, `param_1`)
/// - 0x1000000 = "too many toggles" (counter > 3)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvisorFlags(pub u32);

impl AdvisorFlags {
    pub const GARRISONS_ON: u32 = 0x10; // FUN_00439d60: uVar4 for 0x115
    pub const PRODUCTION_ON: u32 = 0x20; // FUN_00439d60: uVar4 for 0x116
    pub const GARRISONS_OFF: u32 = 0x100; // FUN_00439d60: param_1 for 0x115
    pub const PRODUCTION_OFF: u32 = 0x200; // FUN_00439d60: param_1 for 0x116
    pub const TOO_MANY_TOGGLES: u32 = 0x100_0000; // FUN_00439d60: counter > 3

    /// Set a flag bit.
    pub fn set(&mut self, bit: u32) {
        self.0 |= bit;
    }

    /// Test whether a flag bit is set.
    #[must_use]
    pub fn has(self, bit: u32) -> bool {
        self.0 & bit != 0
    }

    /// Drain all pending flags, returning the value and clearing the word.
    pub fn take(&mut self) -> u32 {
        let v = self.0;
        self.0 = 0;
        v
    }
}

// -------------------------------------------------------------------------
// AgentOrder
// -------------------------------------------------------------------------

/// An order produced by the agent automation for the app/integrator to apply.
///
/// Maps to the original's order objects (0x210 Start-Construction, 0x212
/// Train-Troops, 0x214 Build-Facility). The app enqueues these through the
/// existing `ManufacturingState::enqueue` path, constructing a `QueueItem`
/// from the fields here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentOrder {
    /// Build a production facility (mine or refinery) at a system.
    /// Original orders: 0x214 (set destination) then 0x210 (start construction,
    /// quantity 1). `FUN_004c70e0` + `FUN_004c7220`.
    BuildFacility {
        /// The system whose construction yard builds it.
        system: SystemKey,
        /// Where it goes once built: the yard's Destination (`0x214`) is set
        /// to this site before the build (`FUN_004c70e0`).
        destination: SystemKey,
        /// What to build.
        kind: BuildableKind,
        /// Construction time in game-days. port: derived from the facility
        /// class's `refined_material_cost` via the AI path; the original
        /// used the construction yard's rate.
        ticks: u32,
    },
    /// Train troops and deliver them to a target system.
    /// Original orders: 0x214 (set destination) then 0x212 (train-troops,
    /// quantity = deficit). `FUN_004c7a90` + `FUN_004c7bd0`.
    TrainTroops {
        /// System where training occurs (has a training facility).
        training_system: SystemKey,
        /// Destination system (the under-garrisoned target).
        target_system: SystemKey,
        /// What troop type to build.
        kind: BuildableKind,
        /// How many regiments to train.
        count: u32,
        /// Ticks per regiment. port: uses the AI's default troop build time
        /// since the original reads from the construction yard.
        ticks_per_unit: u32,
    },
}

// -------------------------------------------------------------------------
// PlayerAgent
// -------------------------------------------------------------------------

/// The player-side agent automation state.
///
/// Serializable for save/load. Default = both modules suspended (OFF),
/// matching `FUN_00439950`.
///
/// The app calls [`PlayerAgent::advance`] each game-day tick to get orders,
/// then applies them through the existing `ManufacturingState::enqueue` path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAgent {
    /// Per-module state. FUN_00439950: all start `Suspended`.
    production_state: ModuleState,
    garrison_state: ModuleState,

    /// Toggle counter (`agent+0x160`). Incremented when a module is turned
    /// ON (Suspended→Disabled); decremented when turned OFF. If it exceeds
    /// 3 during an ON toggle, `TOO_MANY_TOGGLES` fires instead of the
    /// per-module ON bit.
    /// FUN_00439d60.
    toggle_counter: i32,

    /// Advisor message flags (`DAT_006b28b0` bits relevant to automation).
    /// FUN_00439d60 sets these; the app reads and clears them.
    pub advisor_flags: AdvisorFlags,

    /// Orders issued this cycle (`agent+0x188`). Reset each cycle.
    /// FUN_00439a10 gates dispatch on `orders_issued < ORDERS_LIMIT`.
    orders_issued: u32,

    /// Agent Advice (menu item 0x11e): `DAT_006b28b0` bit `0x8000` clear.
    /// `FUN_00439320` sets the bit, turning advice off, unless the game's
    /// mode (`FUN_00401b00`, `DAT_006be3b8 + 0x104`) is 1 or 4; the manual
    /// (p. 22) has advice on at Easy only. hyp: mode 1 is Easy. port: the
    /// proactive advice it gates (`FUN_0043a0b0`, `FUN_00439bc0`, the
    /// agent's `+0x154` notifications) is not ported, so it gates nothing yet.
    advice: bool,
}

impl Default for PlayerAgent {
    /// FUN_00439950: all modules start suspended (OFF).
    fn default() -> Self {
        Self {
            production_state: ModuleState::Suspended,
            garrison_state: ModuleState::Suspended,
            toggle_counter: 0,
            advisor_flags: AdvisorFlags::default(),
            orders_issued: 0,
            advice: false,
        }
    }
}

impl PlayerAgent {
    /// Create a new agent with both automations OFF (suspended).
    /// FUN_00439950.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// A new game's agent (`FUN_00439320`): both automations off, and
    /// Agent Advice on only at Easy.
    #[must_use]
    pub fn for_new_game(difficulty: crate::world::SeedDifficulty) -> Self {
        Self {
            advice: difficulty == crate::world::SeedDifficulty::Easy,
            ..Self::default()
        }
    }

    /// Whether Agent Advice is on (menu item 0x11e checked, `FUN_00487900`).
    #[must_use]
    pub const fn advice(&self) -> bool {
        self.advice
    }

    /// Agent Advice (0x11e): `FUN_00439e80` flips bit `0x8000`.
    pub fn toggle_advice(&mut self) -> bool {
        self.advice = !self.advice;
        self.advice
    }

    // -----------------------------------------------------------------
    // Toggle — FUN_00439d60
    // -----------------------------------------------------------------

    /// Toggle a module on or off. Returns `true` if the module is now
    /// running (state != Suspended), `false` if suspended.
    ///
    /// Faithfully ports `FUN_00439d60`: state 2 (Suspended) → 0 (Disabled)
    /// turns ON; any other state → 2 (Suspended) turns OFF.
    ///
    /// The toggle counter increments on each ON toggle. If it exceeds 3,
    /// the `TOO_MANY_TOGGLES` advisor flag fires instead of the per-module
    /// ON flag.
    ///
    /// Evidence: `FUN_00439d60.c`.
    pub fn toggle(&mut self, module: AutomationModule) -> bool {
        let state = self.module_state_mut(module);
        let (on_bit, off_bit) = match module {
            // FUN_00439d60: 0x115 → uVar4=0x10, param_1=0x100
            AutomationModule::Garrisons => {
                (AdvisorFlags::GARRISONS_ON, AdvisorFlags::GARRISONS_OFF)
            }
            // FUN_00439d60: 0x116 → uVar4=0x20, param_1=0x200
            AutomationModule::Production => {
                (AdvisorFlags::PRODUCTION_ON, AdvisorFlags::PRODUCTION_OFF)
            }
        };

        if *state == ModuleState::Suspended {
            // Suspended → Disabled = turn ON.
            // FUN_00439d60: state 2 → 0, counter++.
            *state = ModuleState::Disabled;
            self.toggle_counter += 1;
            if self.toggle_counter > 3 {
                // FUN_00439d60: counter > 3 → TOO_MANY_TOGGLES
                self.advisor_flags.set(AdvisorFlags::TOO_MANY_TOGGLES);
            } else {
                self.advisor_flags.set(on_bit);
            }
            true
        } else {
            // Not-Suspended → Suspended = turn OFF.
            // FUN_00439d60: state → 2, counter--.
            *state = ModuleState::Suspended;
            self.toggle_counter -= 1;
            self.advisor_flags.set(off_bit);
            false
        }
    }

    // -----------------------------------------------------------------
    // Query — FUN_00439e30
    // -----------------------------------------------------------------

    /// Whether a module is currently running (not suspended).
    ///
    /// `FUN_00439e30` returns 1 when `state != 2`. In the original this
    /// checks the menu item (`FUN_00487900`): checked while it runs.
    ///
    /// Evidence: `FUN_00439e30.c`.
    #[must_use]
    pub fn is_running(&self, module: AutomationModule) -> bool {
        self.module_state(module) != ModuleState::Suspended
    }

    // -----------------------------------------------------------------
    // Agent tick — FUN_00439a10
    // -----------------------------------------------------------------

    /// Run one agent cycle. Called each game-day tick.
    ///
    /// Ports the state-4/5/6 dispatch loop from `FUN_00439a10`:
    /// 1. Ready all eligible modules (`VT[4]`, `FUN_004cc990`).
    /// 2. Execute each ready module's decision logic.
    /// 3. Collect orders, capped at [`ORDERS_LIMIT`] per cycle.
    ///
    /// Returns a list of [`AgentOrder`] values for the app to apply through
    /// the manufacturing enqueue path.
    ///
    /// port: one whole cycle per call, made once a day; the original spends
    /// a tick on each of states 4, 5 and 6 and a call on each module phase.
    ///
    /// # Arguments
    ///
    /// - `world` — read-only game world for system/facility queries.
    /// - `mfg` — current manufacturing state (to check queue lengths).
    /// - `economy` — current economy state (for garrison requirements).
    /// - `player_is_alliance` — which side the player controls.
    ///
    /// Evidence: `FUN_00439a10.c`, `FUN_004cc990.c`.
    pub fn advance(
        &mut self,
        world: &GameWorld,
        mfg: &ManufacturingState,
        economy: &EconomyState,
        player_is_alliance: bool,
    ) -> Vec<AgentOrder> {
        // FUN_00439a10 state 6: ready all modules via VT[4] (FUN_004cc990).
        // Promotes Disabled(0) → Ready(1); skips Ready(1) and Suspended(2).
        self.ready_modules();

        // FUN_00439a10 state 4: execute each ready module.
        self.orders_issued = 0;
        let mut orders = Vec::new();

        // Production first (module 0x14), then Garrisons (0x15).
        // port: original uses a BST iterator with round-robin; we use a
        // fixed order since there are only two modules.
        if self.production_state == ModuleState::Ready {
            if let Some(order) = self.run_production(world, mfg, economy, player_is_alliance) {
                if self.orders_issued < ORDERS_LIMIT {
                    orders.push(order);
                    self.orders_issued += 1;
                }
            }
            // FUN_00439a10: after execution, if completed and state != Suspended,
            // set state = Disabled. (Suspended modules stay Suspended.)
            if self.production_state != ModuleState::Suspended {
                self.production_state = ModuleState::Disabled;
            }
        }

        if self.garrison_state == ModuleState::Ready {
            if let Some(order) = self.run_garrisons(world, mfg, economy, player_is_alliance) {
                if self.orders_issued < ORDERS_LIMIT {
                    orders.push(order);
                    self.orders_issued += 1;
                }
            }
            if self.garrison_state != ModuleState::Suspended {
                self.garrison_state = ModuleState::Disabled;
            }
        }

        orders
    }

    // -----------------------------------------------------------------
    // VT[4] — FUN_004cc990
    // -----------------------------------------------------------------

    /// Ready all eligible modules. Ports `FUN_004cc990`:
    /// if state != 1 (Ready) && state != 2 (Suspended): set state = 1.
    fn ready_modules(&mut self) {
        for state in [&mut self.production_state, &mut self.garrison_state] {
            // FUN_004cc990: skip if already Ready(1) or Suspended(2).
            if *state != ModuleState::Ready && *state != ModuleState::Suspended {
                *state = ModuleState::Ready;
            }
        }
    }

    // -----------------------------------------------------------------
    // Production module — FUN_004c6ce0 / FUN_004c6f00
    // -----------------------------------------------------------------

    /// Run the Manage Production decision logic.
    ///
    /// Phase 1 (`FUN_004c6d90`): find an owned system with idle construction
    /// capacity (has a production facility slot, queue not full).
    ///
    /// Phase 2 (`FUN_004c6f00`): compare total raw output vs total refined
    /// output across all player-owned systems. If raw < refined, build a
    /// mine; else build a refinery. Quantity is always 1.
    ///
    /// Evidence: `FUN_004c6ce0.c`, `FUN_004c6f00.c`.
    fn run_production(
        &self,
        world: &GameWorld,
        mfg: &ManufacturingState,
        economy: &EconomyState,
        player_is_alliance: bool,
    ) -> Option<AgentOrder> {
        let our_faction = if player_is_alliance {
            crate::dat::Faction::Alliance
        } else {
            crate::dat::Faction::Empire
        };

        // Phase 2 (FUN_004c6f00): decide mine vs refinery.
        // agent_context+0xa8 vs +0xac — hyp: total raw material production
        // rate vs total refined resource production rate across all owned
        // systems.
        let (total_raw, total_refined) = count_player_resources(world, economy, player_is_alliance);

        // FUN_004c6f00: if raw < refined → mine (0x2c), else refinery (0x2d).
        let build_mine = total_raw < total_refined;

        // Find a production facility key to use as the class template.
        // port: pick the first owned production facility of the desired type.
        let facility = find_production_facility_class(world, player_is_alliance, build_mine)?;

        let kind = BuildableKind::ProductionFacility(facility);

        // The site (FUN_004c6f00's sorts 5..7, untraced): port: the first
        // owned system with room, under the economy's caps (energy slots for
        // every facility, raw deposits for mines; FUN_0050ad60,
        // FUN_0050b0b0), counting those already on their way there.
        let site = world
            .systems
            .iter()
            .find(|(key, sys)| {
                sys.control.is_controlled_by(our_faction)
                    && sys.is_populated
                    && !sys.is_destroyed
                    && has_room(world, mfg, *key, build_mine)
            })
            .map(|(key, _)| key)?;

        // Phase 1 (FUN_004c6d90): an idle construction yard. port: an owned
        // system with a construction-capable (non-shipyard) facility and
        // room in its queue (FUN_00509670's slot is not modelled).
        let target_system = world
            .systems
            .iter()
            .find(|(key, sys)| {
                sys.control.is_controlled_by(our_faction)
                    && !sys.is_destroyed
                    && sys.manufacturing_facilities.iter().any(|facility| {
                        world
                            .manufacturing_facilities
                            .get(*facility)
                            .is_some_and(|value| {
                                value.is_alliance == player_is_alliance && !value.is_shipyard
                            })
                    })
                    && mfg.queued_at(*key) < 3
            })
            .map(|(key, _)| key)?;

        // port: ticks derived from the AI's default facility build time.
        // The original reads from the construction yard's processing rate.
        let ticks = 30_u32;

        Some(AgentOrder::BuildFacility {
            system: target_system,
            destination: site,
            kind,
            ticks,
        })
    }

    // -----------------------------------------------------------------
    // Garrison module — FUN_004c7580 / FUN_004c7650 / FUN_004c7860 / FUN_004c79e0
    // -----------------------------------------------------------------

    /// Run the Manage Garrisons decision logic.
    ///
    /// Phase 2 (`FUN_004c7650`): find the most under-garrisoned owned system.
    ///
    /// Phase 1 (`FUN_004c7860`): locate a system with training capacity near
    /// the target.
    ///
    /// Phase 3 (`FUN_004c79e0`): calculate troop deficit =
    /// `garrison_need(+0x80) - current_troops(+0x84)`, clamped by available
    /// training capacity.
    ///
    /// Evidence: `FUN_004c7580.c`, `FUN_004c7650.c`, `FUN_004c7860.c`,
    /// `FUN_004c79e0.c`.
    fn run_garrisons(
        &self,
        world: &GameWorld,
        mfg: &ManufacturingState,
        economy: &EconomyState,
        player_is_alliance: bool,
    ) -> Option<AgentOrder> {
        let our_faction = if player_is_alliance {
            crate::dat::Faction::Alliance
        } else {
            crate::dat::Faction::Empire
        };

        // Phase 2 (FUN_004c7650): find the most under-garrisoned owned system.
        // Deficit = garrison_requirement - friendly_troops.
        // Original sorts by deficit (sort priority 0x19).
        let (target_system, deficit) =
            find_most_undergarrisoned(world, economy, our_faction, player_is_alliance)?;

        // Phase 1 (FUN_004c7860): find a system with training capacity.
        // Original: queries for entities with flag 0x22000 referencing the
        // target, sorted by distance (sort 2). Then refines with flag 0x104,
        // mask 0x800 for available training slots.
        // port: find any owned system with a manufacturing facility that is
        // NOT a shipyard (training facilities are non-shipyard manufacturing
        // facilities) and has room in its queue.
        let training_system = find_training_system(world, our_faction, player_is_alliance)?;

        // Phase 3 (FUN_004c79e0): calculate build count.
        // deficit = agent_context->garrison_need(+0x80) - current_troops(+0x84)
        // The deficit is already calculated; clamp by available training
        // capacity.
        // FUN_004c79e0: `build_count = min(deficit / unit_capacity, available)`
        // port: unit_capacity is 1 regiment per build, available = deficit
        // (the original clamps by construction yard capacity which we
        // approximate as the deficit itself).
        // port: regiments already in training for the target count against
        // its deficit, so the module does not order them again each day.
        let deficit = deficit.saturating_sub(queued_for(mfg, target_system, |kind| {
            matches!(kind, BuildableKind::Troop(_))
        }));
        let count = deficit.min(ORDERS_LIMIT); // port: cap at orders limit

        if count == 0 {
            return None;
        }

        // Find a troop class to build.
        let troop_class = find_troop_class(world, player_is_alliance)?;
        let kind = BuildableKind::Troop(troop_class);

        // port: ticks per regiment from the AI's default.
        let ticks_per_unit = 15_u32;

        Some(AgentOrder::TrainTroops {
            training_system,
            target_system,
            kind,
            count,
            ticks_per_unit,
        })
    }

    // -----------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------

    fn module_state(&self, module: AutomationModule) -> ModuleState {
        match module {
            AutomationModule::Production => self.production_state,
            AutomationModule::Garrisons => self.garrison_state,
        }
    }

    fn module_state_mut(&mut self, module: AutomationModule) -> &mut ModuleState {
        match module {
            AutomationModule::Production => &mut self.production_state,
            AutomationModule::Garrisons => &mut self.garrison_state,
        }
    }
}

/// Issue the agent's orders as the original does: the Destination order
/// (`0x214`) on the building area's manager, then the build (`0x210`) or
/// training (`0x212`) order (`FUN_004c70e0`/`FUN_004c7220`,
/// `FUN_004c7a90`/`FUN_004c7bd0`). The destination stays the area's.
pub fn apply_orders(orders: &[AgentOrder], mfg: &mut ManufacturingState) {
    use crate::manufacturing::{ProductionArea, QueueItem};
    for order in orders {
        match *order {
            AgentOrder::BuildFacility {
                system,
                destination,
                kind,
                ticks,
            } => {
                mfg.set_destination(system, ProductionArea::of(kind), destination);
                mfg.enqueue(system, QueueItem::new(kind, ticks, ticks));
            }
            AgentOrder::TrainTroops {
                training_system,
                target_system,
                kind,
                count,
                ticks_per_unit,
            } => {
                mfg.set_destination(training_system, ProductionArea::of(kind), target_system);
                for _ in 0..count {
                    mfg.enqueue(
                        training_system,
                        QueueItem::new(kind, ticks_per_unit, ticks_per_unit),
                    );
                }
            }
        }
    }
}

// =========================================================================
// Free-standing helpers
// =========================================================================

/// Count total raw material production (mines) and refined production
/// (non-mine production facilities, i.e. refineries) across all systems
/// controlled by the player.
///
/// hyp: maps to `agent_context+0xa8` (raw) and `+0xac` (refined) in
/// `FUN_004c6f00`. The original's agent context likely aggregates these
/// per-faction totals from the economy pipeline.
fn count_player_resources(
    world: &GameWorld,
    _economy: &EconomyState,
    player_is_alliance: bool,
) -> (u32, u32) {
    let our_faction = if player_is_alliance {
        crate::dat::Faction::Alliance
    } else {
        crate::dat::Faction::Empire
    };

    let mut total_raw: u32 = 0;
    let mut total_refined: u32 = 0;

    for (_sys_key, sys) in &world.systems {
        if !sys.control.is_controlled_by(our_faction) {
            continue;
        }
        // Count mines and refineries owned by the player at this system.
        for pfk in &sys.production_facilities {
            if let Some(pf) = world.production_facilities.get(*pfk) {
                if pf.is_alliance == player_is_alliance {
                    if pf.is_mine {
                        total_raw += 1;
                    } else {
                        total_refined += 1;
                    }
                }
            }
        }
    }

    (total_raw, total_refined)
}

/// Products in any queue bound for `site` whose kind passes `wanted`: those
/// queued at `site` itself with no other destination, and those delivered
/// there from elsewhere.
fn queued_for(
    mfg: &ManufacturingState,
    site: SystemKey,
    wanted: impl Fn(BuildableKind) -> bool,
) -> u32 {
    let count = mfg
        .queues()
        .iter()
        .flat_map(|((system, _), queue)| queue.items().iter().map(move |item| (*system, item)))
        .filter(|(system, item)| item.destination.unwrap_or(*system) == site && wanted(item.kind))
        .count();
    u32::try_from(count).unwrap_or(u32::MAX)
}

/// Whether `site` has room for one more production facility, a mine when
/// `mine`: its facilities, built and on their way, under its energy slots
/// (`FUN_0050ad60`), and its mines under its raw deposits (`FUN_0050b0b0`).
fn has_room(world: &GameWorld, mfg: &ManufacturingState, site: SystemKey, mine: bool) -> bool {
    let Some(system) = world.systems.get(site) else {
        return false;
    };
    let is_mine = |kind: BuildableKind| match kind {
        BuildableKind::ProductionFacility(build) => build.class.family() == 0x2c,
        _ => false,
    };
    let facilities = system.production_facilities.len() as u32
        + queued_for(mfg, site, |kind| {
            matches!(kind, BuildableKind::ProductionFacility(_))
        });
    let mines = system
        .production_facilities
        .iter()
        .filter(|key| {
            world
                .production_facilities
                .get(**key)
                .is_some_and(|value| value.is_mine)
        })
        .count() as u32
        + queued_for(mfg, site, is_mine);
    facilities < u32::from(system.total_energy)
        && (!mine || mines < u32::from(system.raw_materials))
}

/// Find the most under-garrisoned system controlled by `our_faction`.
///
/// Returns `(system_key, deficit)` where `deficit > 0`, or `None` if no
/// system has a garrison deficit.
///
/// FUN_004c7650: queries for owned systems needing garrisons, sorted by
/// deficit (sort priority 0x19).
///
/// Evidence: `FUN_004c7650.c`, `FUN_004c79e0.c`.
fn find_most_undergarrisoned(
    world: &GameWorld,
    economy: &EconomyState,
    our_faction: crate::dat::Faction,
    player_is_alliance: bool,
) -> Option<(SystemKey, u32)> {
    let mut best: Option<(SystemKey, u32)> = None;

    for (sys_key, sys) in &world.systems {
        if !sys.control.is_controlled_by(our_faction) || !sys.is_populated || sys.is_destroyed {
            continue;
        }

        // Get garrison requirement from economy state.
        let requirement = economy
            .per_system
            .get(&sys_key)
            .map_or(0, |eco: &SystemEconomy| eco.garrison_requirement);

        if requirement == 0 {
            continue;
        }

        // Count friendly troops at this system.
        // FUN_004c79e0: agent_context->current_troops (+0x84).
        let friendly_troops: u32 = sys
            .ground_units
            .iter()
            .filter(|tk| {
                world
                    .troops
                    .get(**tk)
                    .is_some_and(|t| t.is_alliance == player_is_alliance)
            })
            .count() as u32;

        if friendly_troops >= requirement {
            continue;
        }

        let deficit = requirement - friendly_troops;
        if best.is_none_or(|(_, best_deficit)| deficit > best_deficit) {
            best = Some((sys_key, deficit));
        }
    }

    best
}

/// Find a system with a training facility (non-shipyard manufacturing
/// facility) controlled by the player.
///
/// FUN_004c7860: queries for entities with flag 0x22000, sorted by distance
/// to the target. port: we simply find any owned system with a training
/// facility.
///
/// Evidence: `FUN_004c7860.c`.
fn find_training_system(
    world: &GameWorld,
    our_faction: crate::dat::Faction,
    player_is_alliance: bool,
) -> Option<SystemKey> {
    for (sys_key, sys) in &world.systems {
        if !sys.control.is_controlled_by(our_faction) || !sys.is_populated || sys.is_destroyed {
            continue;
        }

        // A training facility is a non-shipyard manufacturing facility.
        let has_training = sys.manufacturing_facilities.iter().any(|mfk| {
            world
                .manufacturing_facilities
                .get(*mfk)
                .is_some_and(|f| f.is_alliance == player_is_alliance && !f.is_shipyard)
        });

        if has_training {
            return Some(sys_key);
        }
    }
    None
}

/// Find a troop class key the player can build.
///
/// port: picks the first troop instance owned by the player and returns
/// its class as the template. The original uses entity type 0x29 queries.
///
/// Evidence: `FUN_004c79e0.c` (entity type 0x29).
fn find_troop_class(world: &GameWorld, player_is_alliance: bool) -> Option<DatId> {
    world
        .troops
        .iter()
        .find(|(_, t)| t.is_alliance == player_is_alliance)
        .map(|(_, troop)| troop.class_dat_id)
}

/// Find a production facility key to use as a class template for building
/// a mine or refinery.
///
/// port: picks the first owned production facility of the requested type
/// (mine or refinery). The original uses DatId 0x2c (mine) or 0x2d
/// (refinery) entity type lookups.
///
/// Evidence: `FUN_004c6f00.c` (entity types 0x2c, 0x2d).
fn find_production_facility_class(
    world: &GameWorld,
    player_is_alliance: bool,
    want_mine: bool,
) -> Option<FacilityBuild> {
    world
        .production_facilities
        .iter()
        .find(|(_, f)| f.is_alliance == player_is_alliance && f.is_mine == want_mine)
        .map(|(_, facility)| FacilityBuild {
            class: facility.class_dat_id,
            is_alliance: player_is_alliance,
        })
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::economy::{EconomyState, SystemEconomy};
    use crate::ids::{ManufacturingFacilityKey, ProductionFacilityKey, SystemKey, TroopKey};
    use crate::manufacturing::ManufacturingState;
    use crate::world::{
        ControlKind, GameWorld, ManufacturingFacilityInstance, ProductionFacilityInstance, System,
        TroopUnit,
    };

    // Helper: create a minimal GameWorld with one system.
    fn minimal_world(
        is_alliance: bool,
    ) -> (GameWorld, SystemKey, EconomyState, ManufacturingState) {
        let mut world = GameWorld::default();
        let sys = System {
            dat_id: crate::ids::DatId(0x9000_0001),
            name: "Coruscant".into(),
            sector: world.sectors.insert(crate::world::Sector {
                dat_id: crate::ids::DatId(0x8000_0001),
                name: "Core".into(),
                group: crate::dat::SectorGroup::Core,
                x: 0,
                y: 0,
                systems: vec![],
            }),
            x: 100,
            y: 100,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 10,
            raw_materials: 10,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: if is_alliance {
                ControlKind::Controlled(crate::dat::Faction::Alliance)
            } else {
                ControlKind::Controlled(crate::dat::Faction::Empire)
            },
        };
        let sys_key = world.systems.insert(sys);
        let eco = EconomyState::default();
        let mfg = ManufacturingState::new();
        (world, sys_key, eco, mfg)
    }

    /// Add a production facility (mine or refinery) to a system.
    fn add_production_facility(
        world: &mut GameWorld,
        sys_key: SystemKey,
        is_alliance: bool,
        is_mine: bool,
    ) -> ProductionFacilityKey {
        let pfk = world
            .production_facilities
            .insert(ProductionFacilityInstance {
                class_dat_id: crate::ids::DatId(if is_mine { 0x2c00_0001 } else { 0x2d00_0001 }),
                is_alliance,
                is_mine,
            });
        world
            .systems
            .get_mut(sys_key)
            .unwrap()
            .production_facilities
            .push(pfk);
        pfk
    }

    /// Add a manufacturing facility (training center) to a system.
    fn add_training_facility(
        world: &mut GameWorld,
        sys_key: SystemKey,
        is_alliance: bool,
    ) -> ManufacturingFacilityKey {
        let mfk = world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                class_dat_id: crate::ids::DatId(0xa400_0001),
                is_alliance,
                is_shipyard: false,
            });
        world
            .systems
            .get_mut(sys_key)
            .unwrap()
            .manufacturing_facilities
            .push(mfk);
        mfk
    }

    /// Add a troop unit to a system.
    fn add_troop(world: &mut GameWorld, sys_key: SystemKey, is_alliance: bool) -> TroopKey {
        let tk = world.troops.insert(TroopUnit {
            class_dat_id: crate::ids::DatId(0x2900_0001),
            is_alliance,
            regiment_strength: 100,
        });
        world
            .systems
            .get_mut(sys_key)
            .unwrap()
            .ground_units
            .push(tk);
        tk
    }

    /// Add another system, a copy of `like` with nothing on it.
    fn add_system(world: &mut GameWorld, like: SystemKey) -> SystemKey {
        let mut system = world.systems[like].clone();
        system.ground_units.clear();
        system.manufacturing_facilities.clear();
        system.production_facilities.clear();
        world.systems.insert(system)
    }

    /// Add a shipyard to a system.
    fn add_shipyard(world: &mut GameWorld, sys_key: SystemKey, is_alliance: bool) {
        let mfk = world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                class_dat_id: crate::ids::DatId(0xa000_0001),
                is_alliance,
                is_shipyard: true,
            });
        world.systems[sys_key].manufacturing_facilities.push(mfk);
    }

    /// The facility a BuildFacility order builds, and where.
    fn built(orders: &[AgentOrder]) -> Option<(SystemKey, SystemKey, bool)> {
        orders.iter().find_map(|order| match *order {
            AgentOrder::BuildFacility {
                system,
                destination,
                kind: BuildableKind::ProductionFacility(build),
                ..
            } => Some((system, destination, build.class.family() == 0x2c)),
            _ => None,
        })
    }

    fn producing() -> PlayerAgent {
        let mut agent = PlayerAgent::new();
        agent.toggle(AutomationModule::Production);
        agent
    }

    fn garrisoning() -> PlayerAgent {
        let mut agent = PlayerAgent::new();
        agent.toggle(AutomationModule::Garrisons);
        agent
    }

    fn requires(eco: &mut EconomyState, system: SystemKey, garrison_requirement: u32) {
        eco.per_system.insert(
            system,
            SystemEconomy {
                garrison_requirement,
                ..SystemEconomy::default()
            },
        );
    }

    // -----------------------------------------------------------------
    // Toggle semantics — FUN_00439d60
    // -----------------------------------------------------------------

    /// Both automations start suspended (OFF). FUN_00439950 sets all module
    /// states to 2 (Suspended).
    #[test]
    fn both_automations_start_off() {
        // FUN_00439950: all modules initialized to state 2 (Suspended).
        let agent = PlayerAgent::new();
        assert!(!agent.is_running(AutomationModule::Production));
        assert!(!agent.is_running(AutomationModule::Garrisons));
    }

    /// Toggling a suspended module turns it on (Suspended → Disabled).
    /// FUN_00439d60: state 2 → state 0.
    #[test]
    fn agent_advice_starts_on_at_easy_only_and_its_menu_item_flips_it() {
        // FUN_00439320: bit 0x8000 (advice off) unless the mode is 1;
        // manual p. 22: on at Easy, off at Medium and Hard. FUN_00439e80.
        use crate::world::SeedDifficulty;
        assert!(PlayerAgent::for_new_game(SeedDifficulty::Easy).advice());
        for difficulty in [SeedDifficulty::Medium, SeedDifficulty::Hard] {
            let mut agent = PlayerAgent::for_new_game(difficulty);
            assert!(!agent.advice());
            assert!(!agent.is_running(AutomationModule::Garrisons));
            assert!(agent.toggle_advice());
            assert!(agent.advice());
            assert!(!agent.toggle_advice());
        }
    }

    #[test]
    fn toggle_suspended_module_turns_it_on() {
        // FUN_00439d60: when state == 2, set state = 0 (turn ON).
        let mut agent = PlayerAgent::new();
        let running = agent.toggle(AutomationModule::Production);
        assert!(running);
        assert!(agent.is_running(AutomationModule::Production));
    }

    /// Toggling a running module suspends it (not-Suspended → Suspended).
    /// FUN_00439d60: state != 2 → state 2.
    #[test]
    fn toggle_running_module_turns_it_off() {
        // FUN_00439d60: when state != 2, set state = 2 (turn OFF).
        let mut agent = PlayerAgent::new();
        agent.toggle(AutomationModule::Garrisons); // ON
        let running = agent.toggle(AutomationModule::Garrisons); // OFF
        assert!(!running);
        assert!(!agent.is_running(AutomationModule::Garrisons));
    }

    /// The toggle counter tracks ON toggles. FUN_00439d60:
    /// counter++ on ON, counter-- on OFF.
    #[test]
    fn toggle_counter_tracks_on_off_toggles() {
        // FUN_00439d60: counter incremented on each ON toggle, decremented
        // on each OFF toggle.
        let mut agent = PlayerAgent::new();
        assert_eq!(agent.toggle_counter, 0);

        agent.toggle(AutomationModule::Production); // ON → counter = 1
        assert_eq!(agent.toggle_counter, 1);

        agent.toggle(AutomationModule::Garrisons); // ON → counter = 2
        assert_eq!(agent.toggle_counter, 2);

        agent.toggle(AutomationModule::Production); // OFF → counter = 1
        assert_eq!(agent.toggle_counter, 1);
    }

    /// When the toggle counter exceeds 3, the TOO_MANY_TOGGLES flag fires
    /// instead of the per-module ON flag.
    /// FUN_00439d60: `if (3 < uVar3) DAT_006b28b0 |= 0x1000000`.
    #[test]
    fn too_many_toggles_fires_after_counter_exceeds_three() {
        // FUN_00439d60: after 4th ON toggle, counter > 3 → TOO_MANY_TOGGLES.
        // Two modules cannot raise the counter past 2 by themselves; the
        // counter is set to reach the boundary.
        let mut agent = PlayerAgent::new();
        agent.toggle_counter = 2;
        agent.toggle(AutomationModule::Production); // ON → counter = 3
        assert!(agent.advisor_flags.has(AdvisorFlags::PRODUCTION_ON));
        assert!(!agent.advisor_flags.has(AdvisorFlags::TOO_MANY_TOGGLES));

        let mut agent = PlayerAgent::new();
        agent.toggle_counter = 3;
        agent.toggle(AutomationModule::Production); // ON → counter = 4 > 3
        assert!(agent.advisor_flags.has(AdvisorFlags::TOO_MANY_TOGGLES));
        assert!(!agent.advisor_flags.has(AdvisorFlags::PRODUCTION_ON));
    }

    /// Toggle ON sets the correct per-module advisor flag.
    /// FUN_00439d60: ON sets uVar4 (0x10 for Garrisons, 0x20 for Production).
    #[test]
    fn toggle_on_sets_correct_advisor_flag() {
        // FUN_00439d60: toggle ON → set on_bit (uVar4).
        let mut agent = PlayerAgent::new();
        agent.toggle(AutomationModule::Garrisons);
        assert!(agent.advisor_flags.has(AdvisorFlags::GARRISONS_ON));
        assert!(!agent.advisor_flags.has(AdvisorFlags::GARRISONS_OFF));

        let mut agent2 = PlayerAgent::new();
        agent2.toggle(AutomationModule::Production);
        assert!(agent2.advisor_flags.has(AdvisorFlags::PRODUCTION_ON));
        assert!(!agent2.advisor_flags.has(AdvisorFlags::PRODUCTION_OFF));
    }

    /// Toggle OFF sets the correct per-module advisor flag.
    /// FUN_00439d60: OFF sets param_1 (0x100 for Garrisons, 0x200 for Production).
    #[test]
    fn toggle_off_sets_correct_advisor_flag() {
        // FUN_00439d60: toggle OFF → set off_bit (param_1).
        let mut agent = PlayerAgent::new();
        agent.toggle(AutomationModule::Garrisons); // ON
        agent.advisor_flags.take(); // clear
        agent.toggle(AutomationModule::Garrisons); // OFF
        assert!(agent.advisor_flags.has(AdvisorFlags::GARRISONS_OFF));
        assert!(!agent.advisor_flags.has(AdvisorFlags::GARRISONS_ON));
    }

    // -----------------------------------------------------------------
    // Suspended module issues nothing — FUN_004cc990 + FUN_00439a10
    // -----------------------------------------------------------------

    /// A suspended module never becomes ready and produces no orders.
    /// FUN_004cc990: skips state 2; FUN_00439a10: only executes ready modules.
    #[test]
    fn suspended_module_issues_no_orders() {
        // FUN_004cc990: VT[4] skips Suspended(2), so it never becomes Ready(1).
        // FUN_00439a10: only dispatches Ready modules.
        let mut agent = PlayerAgent::new(); // both Suspended
        let (world, _sys_key, eco, mfg) = minimal_world(true);
        let orders = agent.advance(&world, &mfg, &eco, true);
        assert!(orders.is_empty());
    }

    // -----------------------------------------------------------------
    // Production: mine vs refinery — FUN_004c6f00
    // -----------------------------------------------------------------

    /// Production picks a mine when raw output < refined output.
    /// FUN_004c6f00: `if (agent_context+0xa8 < agent_context+0xac)` → mine.
    #[test]
    fn production_picks_mine_when_raw_less_than_refined() {
        // FUN_004c6f00: raw < refined → build mine (entity type 0x2c).
        let (mut world, sys_key, eco, mfg) = minimal_world(true);

        // FUN_004c6d90 needs an idle construction yard.
        add_training_facility(&mut world, sys_key, true);
        // Add 1 mine and 3 refineries → raw(1) < refined(3) → pick mine.
        add_production_facility(&mut world, sys_key, true, true); // mine
        add_production_facility(&mut world, sys_key, true, false); // refinery
        add_production_facility(&mut world, sys_key, true, false); // refinery
        add_production_facility(&mut world, sys_key, true, false); // refinery

        let mut agent = PlayerAgent::new();
        agent.toggle(AutomationModule::Production); // turn ON
        agent.advisor_flags.take();

        let orders = agent.advance(&world, &mfg, &eco, true);
        assert_eq!(orders.len(), 1);
        match &orders[0] {
            AgentOrder::BuildFacility { kind, .. } => {
                if let BuildableKind::ProductionFacility(build) = kind {
                    assert_eq!(
                        build.class.family(),
                        0x2c,
                        "should build a mine when raw < refined"
                    );
                } else {
                    panic!("expected ProductionFacility");
                }
            }
            _ => panic!("expected BuildFacility order"),
        }
    }

    /// Production picks a refinery when raw output >= refined output.
    /// FUN_004c6f00: `if (agent_context+0xa8 >= agent_context+0xac)` → refinery.
    #[test]
    fn production_picks_refinery_when_raw_gte_refined() {
        // FUN_004c6f00: raw >= refined → build refinery (entity type 0x2d).
        let (mut world, sys_key, eco, mfg) = minimal_world(true);

        // FUN_004c6d90 needs an idle construction yard.
        add_training_facility(&mut world, sys_key, true);
        // Add 3 mines and 1 refinery → raw(3) >= refined(1) → pick refinery.
        add_production_facility(&mut world, sys_key, true, true); // mine
        add_production_facility(&mut world, sys_key, true, true); // mine
        add_production_facility(&mut world, sys_key, true, true); // mine
        add_production_facility(&mut world, sys_key, true, false); // refinery

        let mut agent = PlayerAgent::new();
        agent.toggle(AutomationModule::Production);
        agent.advisor_flags.take();

        let orders = agent.advance(&world, &mfg, &eco, true);
        assert_eq!(orders.len(), 1);
        match &orders[0] {
            AgentOrder::BuildFacility { kind, .. } => {
                if let BuildableKind::ProductionFacility(build) = kind {
                    assert_eq!(
                        build.class.family(),
                        0x2d,
                        "should build a refinery when raw >= refined"
                    );
                } else {
                    panic!("expected ProductionFacility");
                }
            }
            _ => panic!("expected BuildFacility order"),
        }
    }

    /// port: production stops where a site has no room under the economy's
    /// caps (FUN_0050ad60 energy, FUN_0050b0b0 raw deposits), counting the
    /// facilities already queued for it, and the yard delivers to the site.
    #[test]
    fn production_builds_only_where_there_is_room_and_delivers_there() {
        let (mut world, sys_key, eco, mut mfg) = minimal_world(true);
        add_training_facility(&mut world, sys_key, true);
        add_production_facility(&mut world, sys_key, true, true);
        add_production_facility(&mut world, sys_key, true, false);
        world.systems[sys_key].total_energy = 3;
        let mut agent = PlayerAgent::new();
        agent.toggle(AutomationModule::Production);

        let orders = agent.advance(&world, &mfg, &eco, true);
        let AgentOrder::BuildFacility {
            system,
            destination,
            kind,
            ticks,
        } = orders[0].clone()
        else {
            panic!("expected BuildFacility order");
        };
        assert_eq!((system, destination), (sys_key, sys_key));
        mfg.enqueue(
            system,
            crate::manufacturing::QueueItem::new(kind, ticks, ticks),
        );

        // Three facilities, built or queued, fill the three energy slots.
        assert!(agent.advance(&world, &mfg, &eco, true).is_empty());
    }

    #[test]
    fn an_issued_order_sets_its_areas_destination_then_queues_the_build() {
        // FUN_004c7a90 (0x214) then FUN_004c7bd0 (0x212, quantity = count).
        let (mut world, yard, _, mut mfg) = minimal_world(true);
        let target = world.systems.insert(world.systems[yard].clone());
        let kind = BuildableKind::Troop(DatId::new(0x1000_0001));
        apply_orders(
            &[AgentOrder::TrainTroops {
                training_system: yard,
                target_system: target,
                kind,
                count: 2,
                ticks_per_unit: 15,
            }],
            &mut mfg,
        );
        let queued: Vec<_> = mfg
            .queue(yard, crate::manufacturing::ProductionArea::TrainingFacility)
            .unwrap()
            .items()
            .iter()
            .map(|item| item.destination)
            .collect();
        assert_eq!(queued, [Some(target), Some(target)]);
        assert_eq!(
            mfg.destination(yard, crate::manufacturing::ProductionArea::TrainingFacility),
            Some(target)
        );
    }

    /// port: regiments already training for a system count against its
    /// deficit (FUN_004c79e0's +0x84), so none are ordered twice.
    #[test]
    fn garrisons_do_not_order_regiments_already_in_training() {
        let (mut world, sys_key, mut eco, mut mfg) = minimal_world(true);
        add_training_facility(&mut world, sys_key, true);
        let troop = add_troop(&mut world, sys_key, true);
        eco.per_system.insert(
            sys_key,
            SystemEconomy {
                garrison_requirement: 3,
                ..SystemEconomy::default()
            },
        );
        let mut agent = PlayerAgent::new();
        agent.toggle(AutomationModule::Garrisons);
        let orders = agent.advance(&world, &mfg, &eco, true);
        let AgentOrder::TrainTroops {
            count,
            training_system,
            target_system,
            ..
        } = orders[0].clone()
        else {
            panic!("expected TrainTroops order");
        };
        assert_eq!(count, 2);
        for _ in 0..count {
            mfg.enqueue(
                training_system,
                crate::manufacturing::QueueItem::new(
                    BuildableKind::Troop(world.troops[troop].class_dat_id),
                    15,
                    15,
                )
                .delivered_to(target_system),
            );
        }
        assert!(agent.advance(&world, &mfg, &eco, true).is_empty());
    }

    // -----------------------------------------------------------------
    // Garrisons: deficit targeting — FUN_004c7650 + FUN_004c79e0
    // -----------------------------------------------------------------

    /// Garrisons targets the system with the largest troop deficit and caps
    /// the training count.
    /// FUN_004c7650: sort by deficit (0x19).
    /// FUN_004c79e0: deficit = garrison_need - current_troops.
    #[test]
    fn garrisons_targets_largest_deficit_and_caps_count() {
        // FUN_004c7650: finds most under-garrisoned system.
        // FUN_004c79e0: deficit = requirement - friendly_troops.
        let (mut world, sys1, mut eco, mfg) = minimal_world(true);

        // sys1: requirement=4, troops=1 → deficit=3
        eco.per_system.insert(
            sys1,
            SystemEconomy {
                garrison_requirement: 4,
                ..SystemEconomy::default()
            },
        );
        add_troop(&mut world, sys1, true);

        // sys2: requirement=2, troops=0 → deficit=2
        let sys2 = world.systems.insert(System {
            dat_id: crate::ids::DatId(0x9000_0002),
            name: "Hoth".into(),
            sector: world.sectors.keys().next().unwrap(),
            x: 50,
            y: 50,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.3,
            popularity_empire: 0.7,
            is_populated: true,
            total_energy: 5,
            raw_materials: 5,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(crate::dat::Faction::Alliance),
        });
        eco.per_system.insert(
            sys2,
            SystemEconomy {
                garrison_requirement: 2,
                ..SystemEconomy::default()
            },
        );

        // Add a training facility to sys1.
        add_training_facility(&mut world, sys1, true);

        // Add a troop so find_troop_class finds something.
        // (The troop at sys1 already serves this purpose.)

        let mut agent = PlayerAgent::new();
        agent.toggle(AutomationModule::Garrisons);
        agent.advisor_flags.take();

        let orders = agent.advance(&world, &mfg, &eco, true);
        assert_eq!(orders.len(), 1);
        match &orders[0] {
            AgentOrder::TrainTroops {
                target_system,
                count,
                ..
            } => {
                // Should target sys1 (deficit=3 > sys2's deficit=2).
                assert_eq!(*target_system, sys1, "should target the largest deficit");
                // Count should be 3 (the deficit), capped at ORDERS_LIMIT.
                assert_eq!(*count, 3);
            }
            _ => panic!("expected TrainTroops order"),
        }
    }

    /// FUN_004c6f00: a mine only while raw < refined, so a tie builds a
    /// refinery.
    #[test]
    fn production_picks_a_refinery_when_raw_equals_refined() {
        let (mut world, sys_key, eco, mfg) = minimal_world(true);
        add_training_facility(&mut world, sys_key, true);
        for mine in [true, true, false, false] {
            add_production_facility(&mut world, sys_key, true, mine);
        }
        let orders = producing().advance(&world, &mfg, &eco, true);
        assert_eq!(built(&orders), Some((sys_key, sys_key, false)));
    }

    /// port: the yard is the first owned system with a non-shipyard
    /// manufacturing facility of the player's and fewer than three queued.
    #[test]
    fn production_takes_the_first_owned_yard_with_a_short_queue() {
        let (mut world, site, eco, mut mfg) = minimal_world(true);
        add_production_facility(&mut world, site, true, true);
        add_production_facility(&mut world, site, true, false);
        let shipyard_only = add_system(&mut world, site);
        add_shipyard(&mut world, shipyard_only, true);
        let full = add_system(&mut world, site);
        add_training_facility(&mut world, full, true);
        let open = add_system(&mut world, site);
        add_training_facility(&mut world, open, true);
        let troop = add_troop(&mut world, site, true);
        let class = world.troops[troop].class_dat_id;
        let item = || crate::manufacturing::QueueItem::new(BuildableKind::Troop(class), 15, 15);
        for _ in 0..3 {
            mfg.enqueue(full, item());
        }
        for _ in 0..2 {
            mfg.enqueue(open, item());
        }
        let orders = producing().advance(&world, &mfg, &eco, true);
        assert_eq!(built(&orders), Some((open, site, false)));
    }

    /// port: mines built and queued for a site fill its raw deposits
    /// (FUN_0050b0b0).
    #[test]
    fn queued_mines_fill_a_sites_deposits() {
        let (mut world, sys_key, eco, mut mfg) = minimal_world(true);
        add_training_facility(&mut world, sys_key, true);
        let mine = add_production_facility(&mut world, sys_key, true, true);
        for _ in 0..3 {
            add_production_facility(&mut world, sys_key, true, false);
        }
        world.systems[sys_key].raw_materials = 2;
        mfg.enqueue(
            sys_key,
            crate::manufacturing::QueueItem::new(
                BuildableKind::ProductionFacility(FacilityBuild {
                    class: world.production_facilities[mine].class_dat_id,
                    is_alliance: true,
                }),
                30,
                30,
            ),
        );
        assert!(producing().advance(&world, &mfg, &eco, true).is_empty());
        world.systems[sys_key].raw_materials = 3;
        assert_eq!(
            built(&producing().advance(&world, &mfg, &eco, true)),
            Some((sys_key, sys_key, true))
        );
    }

    /// port: a refinery needs an energy slot only, not a deposit.
    #[test]
    fn a_refinery_needs_no_deposit() {
        let (mut world, sys_key, eco, mfg) = minimal_world(true);
        add_training_facility(&mut world, sys_key, true);
        for mine in [true, true, false] {
            add_production_facility(&mut world, sys_key, true, mine);
        }
        world.systems[sys_key].raw_materials = 2;
        assert_eq!(
            built(&producing().advance(&world, &mfg, &eco, true)),
            Some((sys_key, sys_key, false))
        );
    }

    /// port: only facilities on their way take a site's energy slots.
    #[test]
    fn a_queued_regiment_takes_no_energy_slot() {
        let (mut world, sys_key, eco, mut mfg) = minimal_world(true);
        add_training_facility(&mut world, sys_key, true);
        add_production_facility(&mut world, sys_key, true, true);
        add_production_facility(&mut world, sys_key, true, false);
        world.systems[sys_key].total_energy = 3;
        let troop = add_troop(&mut world, sys_key, true);
        mfg.enqueue(
            sys_key,
            crate::manufacturing::QueueItem::new(
                BuildableKind::Troop(world.troops[troop].class_dat_id),
                15,
                15,
            ),
        );
        assert!(built(&producing().advance(&world, &mfg, &eco, true)).is_some());
    }

    /// FUN_004c7650 sorts by deficit: the larger wins wherever it is
    /// listed, and a tie keeps the first.
    #[test]
    fn garrisons_take_the_larger_deficit_and_the_first_of_a_tie() {
        let (mut world, first, mut eco, mfg) = minimal_world(true);
        add_training_facility(&mut world, first, true);
        add_troop(&mut world, first, true);
        let second = add_system(&mut world, first);
        requires(&mut eco, first, 3);
        requires(&mut eco, second, 3);
        let target = |eco: &EconomyState| match garrisoning().advance(&world, &mfg, eco, true)[..] {
            [AgentOrder::TrainTroops { target_system, .. }] => target_system,
            ref other => panic!("expected one TrainTroops order: {other:?}"),
        };
        assert_eq!(
            target(&eco),
            second,
            "deficit 3 at the second beats 2 at the first"
        );
        requires(&mut eco, first, 4);
        assert_eq!(target(&eco), first, "a tie keeps the first");
    }

    /// port: an unpopulated or destroyed system is neither garrisoned nor
    /// trains; regiments train at the first populated, standing system
    /// with a non-shipyard facility, as the player's own regiment class.
    #[test]
    fn regiments_train_at_a_standing_populated_yard_for_a_standing_populated_system() {
        let (mut world, unpopulated, mut eco, mfg) = minimal_world(true);
        world.systems[unpopulated].is_populated = false;
        add_training_facility(&mut world, unpopulated, true);
        let destroyed = add_system(&mut world, unpopulated);
        world.systems[destroyed].is_populated = true;
        world.systems[destroyed].is_destroyed = true;
        add_training_facility(&mut world, destroyed, true);
        let shipyard_only = add_system(&mut world, destroyed);
        world.systems[shipyard_only].is_destroyed = false;
        add_shipyard(&mut world, shipyard_only, true);
        let yard = add_system(&mut world, shipyard_only);
        add_training_facility(&mut world, yard, true);
        add_troop(&mut world, yard, false);
        let own = add_troop(&mut world, yard, true);
        requires(&mut eco, unpopulated, 5);
        requires(&mut eco, destroyed, 5);
        requires(&mut eco, yard, 3);

        let orders = garrisoning().advance(&world, &mfg, &eco, true);
        assert_eq!(
            orders,
            [AgentOrder::TrainTroops {
                training_system: yard,
                target_system: yard,
                kind: BuildableKind::Troop(world.troops[own].class_dat_id),
                count: 2,
                ticks_per_unit: 15,
            }]
        );
    }

    // -----------------------------------------------------------------
    // One cycle — FUN_00439a10, FUN_004cc990
    // -----------------------------------------------------------------

    /// Each running module issues its order in a cycle, and a module that
    /// ran returns to state 0 (FUN_00439a10). port: two modules of one order
    /// each never reach the cycle's cap of 5, so the cap does not bind.
    #[test]
    fn each_running_module_issues_one_order_and_returns_to_state_zero() {
        let (mut world, sys_key, mut eco, mfg) = minimal_world(true);
        add_production_facility(&mut world, sys_key, true, true);
        add_production_facility(&mut world, sys_key, true, false);
        add_training_facility(&mut world, sys_key, true);
        add_troop(&mut world, sys_key, true);
        eco.per_system.insert(
            sys_key,
            SystemEconomy {
                garrison_requirement: 10,
                ..SystemEconomy::default()
            },
        );

        let mut agent = PlayerAgent::new();
        agent.toggle(AutomationModule::Production);
        agent.toggle(AutomationModule::Garrisons);
        let orders = agent.advance(&world, &mfg, &eco, true);
        assert!(matches!(
            orders.as_slice(),
            [
                AgentOrder::BuildFacility { .. },
                AgentOrder::TrainTroops { .. }
            ]
        ));
        assert_eq!(agent.production_state, ModuleState::Disabled);
        assert_eq!(agent.garrison_state, ModuleState::Disabled);
    }

    /// FUN_004cc990 readies only state 0: a suspended module stays
    /// suspended while the other runs.
    #[test]
    fn a_suspended_module_stays_out_while_the_other_runs() {
        let (mut world, sys_key, mut eco, mfg) = minimal_world(true);
        add_production_facility(&mut world, sys_key, true, true);
        add_production_facility(&mut world, sys_key, true, false);
        add_training_facility(&mut world, sys_key, true);
        add_troop(&mut world, sys_key, true);
        eco.per_system.insert(
            sys_key,
            SystemEconomy {
                garrison_requirement: 10,
                ..SystemEconomy::default()
            },
        );

        let mut agent = PlayerAgent::new();
        agent.toggle(AutomationModule::Garrisons);
        let orders = agent.advance(&world, &mfg, &eco, true);
        assert!(matches!(
            orders.as_slice(),
            [AgentOrder::TrainTroops { .. }]
        ));
        assert_eq!(agent.production_state, ModuleState::Suspended);
    }

    // -----------------------------------------------------------------
    // Determinism
    // -----------------------------------------------------------------

    /// The agent produces identical output given identical input.
    /// No RNG is involved in the decision logic.
    #[test]
    fn advance_is_deterministic() {
        let (mut world, sys_key, mut eco, mfg) = minimal_world(true);
        add_production_facility(&mut world, sys_key, true, true);
        add_production_facility(&mut world, sys_key, true, false);
        add_training_facility(&mut world, sys_key, true);
        add_troop(&mut world, sys_key, true);
        eco.per_system.insert(
            sys_key,
            SystemEconomy {
                garrison_requirement: 3,
                ..SystemEconomy::default()
            },
        );

        let run = |agent: &mut PlayerAgent| -> Vec<AgentOrder> {
            // Reset to a known state.
            agent.toggle(AutomationModule::Production);
            agent.toggle(AutomationModule::Garrisons);
            agent.advisor_flags.take();
            agent.advance(&world, &mfg, &eco, true)
        };

        let mut a = PlayerAgent::new();
        let mut b = PlayerAgent::new();
        let orders_a = run(&mut a);
        let orders_b = run(&mut b);
        assert_eq!(orders_a, orders_b, "same input must produce same output");
    }
}
