//! The opening briefing tour's cockpit steps (`FUN_004c30c0` for the
//! Alliance, `FUN_004c0fc0` for the Empire): what each step shows on the
//! galaxy view while the agent droid narrates it
//! (`ghidra/notes/droid-advisor-triggers.md`).

use rebellion_core::ids::SystemKey;
use rebellion_core::world::GameWorld;

use crate::cockpit::{CockpitFaction, GidHighlight, GidMode};

/// What a highlight display marks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TourTarget {
    /// A system by the low 24 bits of its id (`FUN_005f55d0(list, id)`).
    System(u32),
    /// The system of the side's headquarters (object `0x20000005`).
    Headquarters,
    /// The system a character is at, by the low 24 bits of its id: the
    /// first family `0x90..0x97` object on its chain.
    Character(u32),
}

/// What a briefing step does on the cockpit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TourStep {
    /// Show a display (`FUN_0041d8c0` → `FUN_00425d00`).
    Display(GidMode),
    /// Show a highlight display (`FUN_0041d890`) with its caption.
    Highlight {
        mode: GidMode,
        target: TourTarget,
        caption: &'static str,
    },
    /// Step 11 (`FUN_0041dbe0` → `FUN_0041e320`): Popular Support again,
    /// and the game clock runs.
    ReleaseClock,
    /// Step 12 (`FUN_0041d9d0`): the tour holds the cockpit's input.
    LockInput,
    /// Step 13 (`FUN_0041da80`, `FUN_0041d770(1, 0x82)`): input returns and
    /// the Message Index opens on Advice.
    Finish,
}

const fn highlight(mode: GidMode, target: TourTarget, caption: &'static str) -> TourStep {
    TourStep::Highlight {
        mode,
        target,
        caption,
    }
}

const CORUSCANT: u32 = 0x109;
const YAVIN: u32 = 0x121;

/// The step `step` of `faction`'s tour, or `None` for a step that does
/// nothing (step 10).
#[must_use]
pub const fn tour_step(faction: CockpitFaction, step: u32) -> Option<TourStep> {
    use GidMode::{
        AllDefenses, DisplayOff, EnemyMilitaryControl, HighlightAlliance, HighlightEmpire,
        IdleFleets, LoyalToEnemy, LoyalToPlayer, PlayerMilitaryControl, PopularSupport,
        UnexploredSystems,
    };
    use TourStep::Display;
    Some(match (faction, step) {
        (_, 1) => Display(PopularSupport),
        // Mode 0x20 shares 0x21's markers and title (FUN_0042b330).
        (_, 5) => Display(IdleFleets),
        (_, 6) => Display(AllDefenses),
        (_, 9) => Display(DisplayOff),
        (_, 11) => TourStep::ReleaseClock,
        (_, 12) => TourStep::LockInput,
        (_, 13) => TourStep::Finish,
        (_, 0xe) => Display(LoyalToPlayer),
        (_, 0xf) => Display(LoyalToEnemy),
        (CockpitFaction::Alliance, 2) => Display(LoyalToPlayer),
        (CockpitFaction::Alliance, 3) => highlight(
            HighlightAlliance,
            TourTarget::Headquarters,
            "Alliance Headquarters",
        ),
        (CockpitFaction::Alliance, 4 | 8 | 0x14) => {
            highlight(HighlightEmpire, TourTarget::System(CORUSCANT), "Coruscant")
        }
        (CockpitFaction::Alliance, 7) => highlight(
            HighlightAlliance,
            TourTarget::Character(0x240),
            "Mon Mothma",
        ),
        (CockpitFaction::Alliance, 0x10) => Display(EnemyMilitaryControl),
        (CockpitFaction::Alliance, 0x11) => Display(UnexploredSystems),
        (CockpitFaction::Alliance, 0x12) => {
            highlight(HighlightAlliance, TourTarget::System(YAVIN), "Yavin")
        }
        (CockpitFaction::Alliance, 0x13) => highlight(
            HighlightAlliance,
            TourTarget::Character(0x242),
            "Luke Skywalker",
        ),
        (CockpitFaction::Empire, 2 | 0x12) => {
            highlight(HighlightEmpire, TourTarget::System(CORUSCANT), "Coruscant")
        }
        (CockpitFaction::Empire, 3 | 8 | 0x14) => {
            highlight(HighlightAlliance, TourTarget::System(YAVIN), "Yavin")
        }
        (CockpitFaction::Empire, 4 | 0x11) => Display(UnexploredSystems),
        (CockpitFaction::Empire, 7) => highlight(
            HighlightEmpire,
            TourTarget::Character(0x280),
            "Emperor Palpatine",
        ),
        (CockpitFaction::Empire, 0x10) => Display(PlayerMilitaryControl),
        (CockpitFaction::Empire, 0x13) => {
            highlight(HighlightEmpire, TourTarget::Character(0x281), "Darth Vader")
        }
        _ => return None,
    })
}

/// The systems `target` marks for `faction`'s player.
#[must_use]
pub fn resolve_target(
    world: &GameWorld,
    faction: CockpitFaction,
    target: TourTarget,
) -> Vec<SystemKey> {
    let low = |raw: u32| raw & 0x00ff_ffff;
    match target {
        TourTarget::System(id) => world
            .systems
            .iter()
            .filter(|(_, system)| low(system.dat_id.raw()) == id)
            .map(|(key, _)| key)
            .collect(),
        TourTarget::Headquarters => {
            let side = match faction {
                CockpitFaction::Alliance => rebellion_core::dat::Faction::Alliance,
                CockpitFaction::Empire => rebellion_core::dat::Faction::Empire,
            };
            world
                .systems
                .iter()
                .filter(|(_, system)| {
                    system.manufacturing_facilities.iter().any(|key| {
                        world
                            .manufacturing_facilities
                            .get(*key)
                            .is_some_and(|facility| {
                                facility.side == side
                                    && (0x20..=0x22).contains(&(facility.class_dat_id.raw() >> 24))
                            })
                    })
                })
                .map(|(key, _)| key)
                .collect()
        }
        TourTarget::Character(id) => world
            .characters
            .values()
            .filter(|character| low(character.dat_id.raw()) == id)
            .filter_map(|character| {
                character.current_system.or_else(|| {
                    character
                        .current_fleet
                        .and_then(|fleet| world.fleets.get(fleet))
                        .map(|fleet| fleet.location)
                })
            })
            .collect(),
    }
}

/// The highlight a step shows, resolved against the world.
#[must_use]
pub fn resolve_highlight(
    world: &GameWorld,
    faction: CockpitFaction,
    target: TourTarget,
    caption: &'static str,
) -> GidHighlight {
    GidHighlight {
        systems: resolve_target(world, faction, target),
        caption,
    }
}
