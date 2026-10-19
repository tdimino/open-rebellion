//! Victory condition detection.
//!
//! Checks the asymmetric original-game objectives each tick:
//! - **Alliance**: hold or destroy Coruscant (CA or CD) and, in Standard
//!   mode, hold Emperor Palpatine and Darth Vader captive.
//! - **Empire**: remove the Alliance headquarters (AD: the HQ object missing
//!   or destroyed, no occupation needed) and, in Standard mode, hold Mon
//!   Mothma and Luke Skywalker captive.
//! - **Death Star route**: destroying the Alliance-HQ system destroys the HQ
//!   with it (AD); destroying Coruscant sets CD, which counts for the
//!   Alliance.
//! - **Headquarters Only**: omits only the principal-leader requirements.
//!
//! The flags are `FUN_005180e0`'s (`ghidra/notes/victory-conditions.md`).
//!
//! # Architecture
//!
//! Follows the stateless advance pattern:
//! ```text
//! VictorySystem::check(&VictoryState, &world, &[TickEvent], VictoryConditions)
//!     -> Option<VictoryOutcome>
//! ```
//! Returns `None` every tick until a terminal condition is met, then returns
//! `Some(VictoryOutcome)`. The caller sets `VictoryState::resolved = true` to
//! suppress repeated checks after the first outcome.
//!
//! # Source
//!
//! Ghidra RE: `ghidra/notes/victory-conditions.md` — the win word
//! `FUN_005180e0` (AD, CA, CD and the leaders), notification `0x128`
//! (`SideVictoryConditionsNotif`).
//! `IsHeadquarters` flag → `System::is_headquarters`.

use serde::{Deserialize, Serialize};

use crate::dat::Faction;
use crate::ids::SystemKey;
use crate::tick::TickEvent;
use crate::world::{GameWorld, VictoryConditions};

// ---------------------------------------------------------------------------
// The objects the win word reads (`FUN_005180e0`)
// ---------------------------------------------------------------------------

/// Luke Skywalker (LC, `FUN_005180e0`).
pub const LUKE: u32 = 0x3200_0242;
/// Mon Mothma (MC, `FUN_005180e0`).
pub const MON_MOTHMA: u32 = 0x3000_0240;
/// Darth Vader (DC, `FUN_005180e0`).
pub const VADER: u32 = 0x3500_0281;
/// Emperor Palpatine (EC, `FUN_005180e0`).
pub const PALPATINE: u32 = 0x3400_0280;
/// Coruscant (CA, CD, `FUN_005180e0`).
pub const CORUSCANT: u32 = 0x9000_0109;

/// The win word's bits (`FUN_005180e0`; `ghidra/notes/victory-flag-word.md`):
/// the Alliance HQ gone.
pub const AD: u32 = 0x1;
/// Luke a prisoner (`FUN_005180e0`).
pub const LC: u32 = 0x10;
/// Mon Mothma a prisoner (`FUN_005180e0`).
pub const MC: u32 = 0x20;
/// Coruscant held by the Alliance (`FUN_005180e0`).
pub const CA: u32 = 0x100;
/// Coruscant destroyed (`FUN_005180e0`).
pub const CD: u32 = 0x200;
/// Vader a prisoner (`FUN_005180e0`).
pub const DC: u32 = 0x1000;
/// Palpatine a prisoner (`FUN_005180e0`).
pub const EC: u32 = 0x2000;
/// The Empire wins (`FUN_005180e0`).
pub const EW: u32 = 0x10_0000;
/// The Alliance wins (`FUN_005180e0`).
pub const AW: u32 = 0x100_0000;
/// The game is over (`FUN_005180e0`).
pub const GO: u32 = 0x1000_0000;

// ---------------------------------------------------------------------------
// VictoryOutcome
// ---------------------------------------------------------------------------

/// Terminal game states returned by `VictorySystem::check`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VictoryOutcome {
    /// The Alliance holds Coruscant (CA) or Coruscant is destroyed (CD).
    HqCaptured {
        winner: Faction,
        loser: Faction,
        hq_system: SystemKey,
    },

    /// The Alliance headquarters is gone (AD) and its system stands.
    HqDestroyed {
        winner: Faction,
        loser: Faction,
        hq_system: SystemKey,
    },

    /// The Death Star destroyed the planet holding the Alliance HQ, and the
    /// HQ with it (AD).
    DeathStarVictory { target_system: SystemKey },
}

// ---------------------------------------------------------------------------
// VictoryState
// ---------------------------------------------------------------------------

/// Configuration for victory detection — set at game start.
///
/// Stores the HQ system for each faction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VictoryState {
    /// The Alliance headquarters system key; the null key when the data
    /// marks none, which the word reads as AD (`FUN_00506110` null).
    pub alliance_hq: SystemKey,
    /// The Empire headquarters system key; the null key when the data marks
    /// none. The word reads Coruscant by its id instead.
    pub empire_hq: SystemKey,
    /// The win word last delivered (notification `0x128`); its change files
    /// the word's messages and its win bits decide a tie.
    #[serde(default)]
    pub word: u32,
    /// Set `true` once a `VictoryOutcome` has been returned to suppress
    /// re-firing on subsequent ticks.
    pub resolved: bool,
}

impl VictoryState {
    #[must_use]
    pub fn new(alliance_hq: SystemKey, empire_hq: SystemKey) -> Self {
        VictoryState {
            alliance_hq,
            empire_hq,
            word: 0,
            resolved: false,
        }
    }
}

// ---------------------------------------------------------------------------
// VictorySystem
// ---------------------------------------------------------------------------

/// Stateless victory-condition evaluator.
pub struct VictorySystem;

impl VictorySystem {
    /// Evaluate all victory conditions against the current world state.
    ///
    /// Returns `Some(VictoryOutcome)` the first tick the word sets a win bit;
    /// `None` otherwise. Skips frames without a simulation tick and
    /// already-resolved games. The original rules do not impose a minimum day.
    /// The caller must set `state.resolved = true` after acting on a result.
    #[must_use]
    pub fn check(
        state: &VictoryState,
        world: &GameWorld,
        tick_events: &[TickEvent],
        victory_conditions: VictoryConditions,
    ) -> Option<VictoryOutcome> {
        if tick_events.is_empty() || state.resolved {
            return None;
        }
        let word = Self::word(state, world, victory_conditions)?;
        let coruscant = Self::coruscant(world)?;
        if word & AW != 0 {
            Some(VictoryOutcome::HqCaptured {
                winner: Faction::Alliance,
                loser: Faction::Empire,
                hq_system: coruscant,
            })
        } else if word & EW != 0 {
            let destroyed = world
                .systems
                .get(state.alliance_hq)
                .is_some_and(|system| system.is_destroyed);
            Some(if destroyed {
                // A destroyed planet takes the HQ with it (FUN_004f8710).
                VictoryOutcome::DeathStarVictory {
                    target_system: state.alliance_hq,
                }
            } else {
                VictoryOutcome::HqDestroyed {
                    winner: Faction::Empire,
                    loser: Faction::Alliance,
                    hq_system: state.alliance_hq,
                }
            })
        } else {
            None
        }
    }

    /// The win word `FUN_005180e0` builds from zero: `None` while a principal
    /// leader or Coruscant is missing, the word left as it was. The component
    /// bits are written for both game types; EW and AW by the game type,
    /// and when both qualify the side whose bit [`VictoryState::word`]
    /// already held keeps it, else AW.
    #[must_use]
    pub fn word(
        state: &VictoryState,
        world: &GameWorld,
        victory_conditions: VictoryConditions,
    ) -> Option<u32> {
        let captive = |id: u32| {
            world
                .characters
                .values()
                .find(|character| character.dat_id.is_record(id))
                .map(|character| character.is_captive)
        };
        let leaders = [
            (captive(LUKE)?, LC),
            (captive(MON_MOTHMA)?, MC),
            (captive(VADER)?, DC),
            (captive(PALPATINE)?, EC),
        ];
        let coruscant_key = Self::coruscant(world)?;
        let coruscant = &world.systems[coruscant_key];

        // A prisoner counts whoever holds it, a jailed one too (+0xac bit 0,
        // FUN_00549580).
        let mut word = leaders
            .into_iter()
            .filter(|&(prisoner, _)| prisoner)
            .fold(0, |word, (_, bit)| word | bit);
        // AD: the HQ object is gone, whoever holds its system.
        if world
            .systems
            .get(state.alliance_hq)
            .is_none_or(|system| system.is_destroyed || !system.is_headquarters)
        {
            word |= AD;
        }
        // CA: Coruscant's holder bits (+0x24 & 0xc0 == 0x40), which keep the
        // earlier holder through a stalemate (FUN_0050b610).
        if crate::bombardment::holder(world, coruscant_key) == Faction::Alliance {
            word |= CA;
        }
        if coruscant.is_destroyed {
            word |= CD;
        }

        let all = |bits: u32| word & bits == bits;
        let (empire, alliance) = match victory_conditions {
            VictoryConditions::Standard => (
                all(AD | LC | MC),
                all(DC | EC) && word & (CA | CD) != 0,
            ),
            VictoryConditions::HeadquartersOnly => (all(AD), word & (CA | CD) != 0),
        };
        word |= match (empire, alliance) {
            (true, true) if state.word & AW == 0 && state.word & EW != 0 => EW | GO,
            (_, true) => AW | GO,
            (true, false) => EW | GO,
            (false, false) => 0,
        };
        Some(word)
    }

    /// Coruscant, by its id (`FUN_005180e0`).
    fn coruscant(world: &GameWorld) -> Option<SystemKey> {
        world
            .systems
            .iter()
            .find(|(_, system)| system.dat_id.is_record(CORUSCANT))
            .map(|(key, _)| key)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dat::ExplorationStatus;
    use crate::dat::SectorGroup;
    use crate::ids::DatId;
    use crate::tick::TickEvent;
    use crate::world::ControlKind;
    use crate::world::{Character, GameWorld, Sector, System};

    fn tick(n: u64) -> TickEvent {
        TickEvent { tick: n }
    }

    /// Build a minimal world with one sector and two systems.
    fn make_world() -> (GameWorld, SystemKey, SystemKey) {
        let mut world = GameWorld::default();

        let sector_key = world.sectors.insert(Sector {
            dat_id: DatId::new(0x9200_0000),
            name: "Test Sector".into(),
            group: SectorGroup::RimOuter,
            x: 0,
            y: 0,
            systems: vec![],
        });

        let a_hq = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0000),
            name: "Yavin IV".into(),
            sector: sector_key,
            x: 100,
            y: 200,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.8,
            popularity_empire: 0.2,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: true,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Alliance),
        });

        let e_hq = world.systems.insert(System {
            dat_id: DatId::new(CORUSCANT),
            name: "Coruscant".into(),
            sector: sector_key,
            x: 500,
            y: 500,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.1,
            popularity_empire: 0.9,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: true,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Empire),
        });

        for id in [LUKE, MON_MOTHMA, VADER, PALPATINE] {
            world.characters.insert(Character {
                dat_id: DatId::new(id),
                ..Character::default()
            });
        }

        (world, a_hq, e_hq)
    }

    fn leader(world: &mut GameWorld, id: u32) -> &mut Character {
        world
            .characters
            .values_mut()
            .find(|character| character.dat_id.raw() == id)
            .expect("make_world seeds the four leaders")
    }

    fn capture_leader(world: &mut GameWorld, id: u32, captured_by: Faction) {
        let leader = leader(world, id);
        leader.is_captive = true;
        leader.captured_by = Some(captured_by);
    }

    /// The ids as `rebellion-data` loads them: a SYSTEMSD or MJCHARSD
    /// record's id is its index alone, the family kept apart
    /// (`load_game_data`: `DatId::new(dat.id)`, `system_families`).
    fn as_loaded(world: &mut GameWorld) {
        for (key, system) in &mut world.systems {
            world.system_families.insert(key, system.dat_id.family());
            system.dat_id = DatId::new(system.dat_id.index());
        }
        for character in world.characters.values_mut() {
            character.dat_id = DatId::new(character.dat_id.index());
        }
    }

    #[test]
    fn the_word_finds_the_loaded_leaders_and_coruscant_by_their_record_index() {
        // FUN_005180e0 looks up Coruscant (0x109) and the four leaders
        // (0x240, 0x242, 0x280, 0x281) by id; a loaded world holds them
        // without their family byte, and either side still wins.
        for conditions in [
            VictoryConditions::HeadquartersOnly,
            VictoryConditions::Standard,
        ] {
            let (mut world, a, e) = make_world();
            world.systems[a].is_headquarters = false;
            capture_leader(&mut world, LUKE, Faction::Empire);
            capture_leader(&mut world, MON_MOTHMA, Faction::Empire);
            as_loaded(&mut world);
            let state = VictoryState::new(a, e);
            assert_eq!(
                VictorySystem::word(&state, &world, conditions),
                Some(AD | LC | MC | EW | GO),
                "Empire {conditions:?}"
            );

            let (mut world, a, e) = make_world();
            world.systems[e].control = ControlKind::Controlled(Faction::Alliance);
            capture_leader(&mut world, VADER, Faction::Alliance);
            capture_leader(&mut world, PALPATINE, Faction::Alliance);
            as_loaded(&mut world);
            let state = VictoryState::new(a, e);
            assert_eq!(
                VictorySystem::word(&state, &world, conditions),
                Some(CA | DC | EC | AW | GO),
                "Alliance {conditions:?}"
            );
        }
    }

    #[test]
    fn when_both_sides_win_at_once_the_alliance_wins() {
        // FUN_005180e0: with neither win bit set before, a word where both
        // sides qualify takes AW (victory-conditions.md, The rules).
        for conditions in [
            VictoryConditions::HeadquartersOnly,
            VictoryConditions::Standard,
        ] {
            let (mut world, a, e) = make_world();
            world.systems[a].is_headquarters = false;
            world.systems[e].control = ControlKind::Controlled(Faction::Alliance);
            for (id, captor) in [
                (PALPATINE, Faction::Alliance),
                (VADER, Faction::Alliance),
                (LUKE, Faction::Empire),
                (MON_MOTHMA, Faction::Empire),
            ] {
                capture_leader(&mut world, id, captor);
            }
            let state = VictoryState::new(a, e);
            let out = VictorySystem::check(&state, &world, &[tick(1)], conditions);
            assert!(
                matches!(
                    out,
                    Some(VictoryOutcome::HqCaptured {
                        winner: Faction::Alliance,
                        ..
                    })
                ),
                "{conditions:?}: {out:?}"
            );
        }
    }

    #[test]
    fn the_word_carries_each_component_bit_for_either_game_type() {
        // FUN_005180e0 writes the component bits whatever the game type.
        for conditions in [
            VictoryConditions::HeadquartersOnly,
            VictoryConditions::Standard,
        ] {
            let (mut world, a, e) = make_world();
            let state = VictoryState::new(a, e);
            assert_eq!(VictorySystem::word(&state, &world, conditions), Some(0));
            capture_leader(&mut world, LUKE, Faction::Empire);
            capture_leader(&mut world, VADER, Faction::Alliance);
            world.systems[e].is_destroyed = true;
            assert_eq!(
                VictorySystem::word(&state, &world, conditions),
                Some(LC | DC | CD | if conditions == VictoryConditions::Standard {
                    0
                } else {
                    AW | GO
                })
            );
        }
    }

    #[test]
    fn standard_prisoners_alone_win_neither_side() {
        // FUN_005180e0, Standard: EW needs AD, LC and MC; AW needs DC, EC and
        // CA or CD. The prisoners without the objective give no win bit.
        let (mut world, a, e) = make_world();
        let state = VictoryState::new(a, e);
        capture_leader(&mut world, MON_MOTHMA, Faction::Empire);
        assert_eq!(
            VictorySystem::word(&state, &world, VictoryConditions::Standard),
            Some(MC)
        );
        capture_leader(&mut world, VADER, Faction::Alliance);
        capture_leader(&mut world, PALPATINE, Faction::Alliance);
        assert_eq!(
            VictorySystem::word(&state, &world, VictoryConditions::Standard),
            Some(MC | DC | EC)
        );
    }

    #[test]
    fn a_stalemated_coruscant_counts_for_the_side_that_held_it_before() {
        // CA tests Coruscant's +0x24 & 0xc0 == 0x40 (FUN_005180e0), and both
        // sides' regiments keep the earlier troop holder in those bits
        // (FUN_0050b610.c:45-70).
        for (earlier, later, word) in [
            (Faction::Alliance, Faction::Empire, Some(CA | AW | GO)),
            (Faction::Empire, Faction::Alliance, Some(0)),
        ] {
            let (mut world, a, e) = make_world();
            let state = VictoryState::new(a, e);
            world.systems[e].control = ControlKind::Controlled(earlier);
            for (index, side) in [earlier, later].into_iter().enumerate() {
                let unit = world.troops.insert(crate::world::TroopUnit {
                    class_dat_id: DatId::new(0x1000_0001),
                    is_alliance: side == Faction::Alliance,
                });
                world.systems[e].ground_units.push(unit);
                if index == 0 {
                    crate::support_shift::refresh_troop_bits(&mut world, e);
                }
            }
            crate::support_shift::rebuild_control(&mut world, e);
            assert_eq!(
                VictorySystem::word(&state, &world, VictoryConditions::HeadquartersOnly),
                word,
                "{earlier:?}"
            );
        }
    }

    #[test]
    fn a_win_bit_already_held_keeps_its_side_in_a_tie() {
        // FUN_005180e0: both qualifying, an earlier EW keeps the Empire's
        // win unless AW was set too.
        let (mut world, a, e) = make_world();
        world.systems[a].is_headquarters = false;
        world.systems[e].control = ControlKind::Controlled(Faction::Alliance);
        let mut state = VictoryState::new(a, e);
        let conditions = VictoryConditions::HeadquartersOnly;
        state.word = EW;
        assert_eq!(
            VictorySystem::word(&state, &world, conditions),
            Some(AD | CA | EW | GO)
        );
        state.word = EW | AW;
        assert_eq!(
            VictorySystem::word(&state, &world, conditions),
            Some(AD | CA | AW | GO)
        );
    }

    #[test]
    fn the_leaders_are_known_by_their_ids_not_their_names() {
        // FUN_005180e0 looks the four leaders up by id.
        let (mut world, a, e) = make_world();
        world.systems[e].is_destroyed = true;
        for id in [PALPATINE, VADER] {
            capture_leader(&mut world, id, Faction::Alliance);
            leader(&mut world, id).name = "Renamed".into();
        }
        world.characters.insert(Character {
            name: "Darth Vader".into(),
            ..Character::default()
        });
        let state = VictoryState::new(a, e);
        assert!(matches!(
            VictorySystem::check(&state, &world, &[tick(1)], VictoryConditions::Standard),
            Some(VictoryOutcome::HqCaptured {
                winner: Faction::Alliance,
                ..
            })
        ));
    }

    #[test]
    fn a_missing_leader_or_coruscant_leaves_no_side_a_win() {
        // FUN_005180e0 returns 0, the word unchanged, when a leader or
        // Coruscant is missing, whatever the game type.
        for missing in [LUKE, MON_MOTHMA, VADER, PALPATINE, CORUSCANT] {
            for conditions in [
                VictoryConditions::HeadquartersOnly,
                VictoryConditions::Standard,
            ] {
                let (mut world, a, e) = make_world();
                world.systems[a].is_headquarters = false;
                for (id, captor) in [
                    (LUKE, Faction::Empire),
                    (MON_MOTHMA, Faction::Empire),
                ] {
                    capture_leader(&mut world, id, captor);
                }
                if missing == CORUSCANT {
                    world.systems[e].dat_id = DatId::new(0x9000_0001);
                } else {
                    let key = world
                        .characters
                        .iter()
                        .find(|(_, character)| character.dat_id.raw() == missing)
                        .map(|(key, _)| key)
                        .expect("seeded");
                    world.characters.remove(key);
                }
                let state = VictoryState::new(a, e);
                assert_eq!(
                    VictorySystem::check(&state, &world, &[tick(1)], conditions),
                    None,
                    "{missing:#x} {conditions:?}"
                );
            }
        }
    }

    #[test]
    fn no_outcome_without_ticks() {
        let (world, a, e) = make_world();
        let state = VictoryState::new(a, e);
        assert!(VictorySystem::check(&state, &world, &[], VictoryConditions::Standard).is_none());
    }

    #[test]
    fn no_outcome_when_resolved() {
        let (world, a, e) = make_world();
        let mut state = VictoryState::new(a, e);
        state.resolved = true;
        assert!(
            VictorySystem::check(&state, &world, &[tick(1)], VictoryConditions::Standard).is_none()
        );
    }

    #[test]
    fn occupation_without_destruction_does_not_defeat_alliance() {
        let (mut world, a, e) = make_world();
        world.systems.get_mut(a).unwrap().control = ControlKind::Controlled(Faction::Empire);

        let state = VictoryState::new(a, e);
        assert!(VictorySystem::check(
            &state,
            &world,
            &[tick(1)],
            VictoryConditions::HeadquartersOnly,
        )
        .is_none());
    }

    #[test]
    fn headquarters_only_alliance_captures_coruscant_on_first_tick() {
        let (mut world, a, e) = make_world();
        world.systems.get_mut(e).unwrap().control = ControlKind::Controlled(Faction::Alliance);

        let state = VictoryState::new(a, e);
        let out = VictorySystem::check(
            &state,
            &world,
            &[tick(1)],
            VictoryConditions::HeadquartersOnly,
        );
        assert!(matches!(
            out,
            Some(VictoryOutcome::HqCaptured {
                winner: Faction::Alliance,
                loser: Faction::Empire,
                ..
            })
        ));
    }

    /// `FUN_005180e0` sets AD when the HQ object is missing or destroyed
    /// and reads no control of its system (`victory-conditions.md`, "No
    /// occupation is required").
    #[test]
    fn a_removed_alliance_hq_wins_without_imperial_control() {
        let (mut world, alliance_hq, empire_hq) = make_world();
        world.systems[alliance_hq].is_headquarters = false;

        let state = VictoryState::new(alliance_hq, empire_hq);
        assert!(matches!(
            VictorySystem::check(
                &state,
                &world,
                &[tick(1)],
                VictoryConditions::HeadquartersOnly
            ),
            Some(VictoryOutcome::HqDestroyed {
                winner: Faction::Empire,
                loser: Faction::Alliance,
                ..
            })
        ));

        world.systems[alliance_hq].control = ControlKind::Controlled(Faction::Empire);
        assert!(matches!(
            VictorySystem::check(
                &state,
                &world,
                &[tick(1)],
                VictoryConditions::HeadquartersOnly
            ),
            Some(VictoryOutcome::HqDestroyed {
                winner: Faction::Empire,
                loser: Faction::Alliance,
                ..
            })
        ));
    }

    #[test]
    fn death_star_hq_destruction_satisfies_headquarters_only() {
        let (mut world, a, e) = make_world();
        world.systems.get_mut(a).unwrap().is_destroyed = true;
        let state = VictoryState::new(a, e);

        let out = VictorySystem::check(
            &state,
            &world,
            &[tick(1)],
            VictoryConditions::HeadquartersOnly,
        );
        assert!(matches!(out, Some(VictoryOutcome::DeathStarVictory { .. })));
    }

    /// CD (Coruscant's `+0x50` bit 3) counts for the Alliance: Headquarters
    /// Only's AW is CA or CD (`FUN_005180e0`, `game-end-path.md`).
    #[test]
    fn a_destroyed_coruscant_wins_headquarters_only_for_the_alliance() {
        let (mut world, alliance_hq, empire_hq) = make_world();
        world.systems[empire_hq].is_destroyed = true;
        let state = VictoryState::new(alliance_hq, empire_hq);

        assert!(matches!(
            VictorySystem::check(
                &state,
                &world,
                &[tick(1)],
                VictoryConditions::HeadquartersOnly,
            ),
            Some(VictoryOutcome::HqCaptured {
                winner: Faction::Alliance,
                loser: Faction::Empire,
                ..
            })
        ));
    }

    /// Standard's AW is DC, EC and (CA or CD): a destroyed Coruscant waits
    /// for both Imperial leaders (`FUN_005180e0`).
    #[test]
    fn standard_destroyed_coruscant_waits_for_both_imperial_leaders() {
        let (mut world, alliance_hq, empire_hq) = make_world();
        world.systems[empire_hq].is_destroyed = true;
        let state = VictoryState::new(alliance_hq, empire_hq);
        assert!(
            VictorySystem::check(&state, &world, &[tick(1)], VictoryConditions::Standard).is_none()
        );

        capture_leader(&mut world, PALPATINE, Faction::Alliance);
        capture_leader(&mut world, VADER, Faction::Alliance);
        assert!(matches!(
            VictorySystem::check(&state, &world, &[tick(1)], VictoryConditions::Standard),
            Some(VictoryOutcome::HqCaptured {
                winner: Faction::Alliance,
                ..
            })
        ));
    }

    #[test]
    fn standard_destroyed_hq_waits_for_both_alliance_leaders() {
        let (mut world, alliance_hq, empire_hq) = make_world();
        world.systems[alliance_hq].is_headquarters = false;
        world.systems[alliance_hq].control = ControlKind::Controlled(Faction::Empire);
        capture_leader(&mut world, LUKE, Faction::Empire);

        let state = VictoryState::new(alliance_hq, empire_hq);
        let tick = tick(1);
        assert!(
            VictorySystem::check(&state, &world, &[tick], VictoryConditions::Standard,).is_none()
        );

        capture_leader(&mut world, MON_MOTHMA, Faction::Empire);
        assert!(matches!(
            VictorySystem::check(&state, &world, &[tick], VictoryConditions::Standard,),
            Some(VictoryOutcome::HqDestroyed {
                winner: Faction::Empire,
                ..
            })
        ));
    }

    /// `FUN_005180e0` reads each leader's prisoner bit (`+0xac` bit 0) and
    /// nothing else: Vader free holds the win back, Vader a prisoner gives it
    /// (`victory-conditions.md`, "A prisoner counts whoever holds it").
    #[test]
    fn standard_hq_capture_waits_for_both_empire_leaders() {
        let (mut world, alliance_hq, empire_hq) = make_world();
        world.systems.get_mut(empire_hq).unwrap().control =
            ControlKind::Controlled(Faction::Alliance);
        capture_leader(&mut world, PALPATINE, Faction::Alliance);
        capture_leader(&mut world, VADER, Faction::Alliance);
        let vader = world
            .characters
            .iter()
            .find(|(_, character)| character.dat_id.raw() == VADER)
            .map(|(key, _)| key)
            .unwrap();
        world.characters[vader].is_captive = false;
        world.characters[vader].captured_by = None;

        let state = VictoryState::new(alliance_hq, empire_hq);
        let tick = tick(1);
        assert!(
            VictorySystem::check(&state, &world, &[tick], VictoryConditions::Standard,).is_none()
        );

        world.characters[vader].is_captive = true;
        world.characters[vader].captured_by = Some(Faction::Alliance);
        assert!(matches!(
            VictorySystem::check(&state, &world, &[tick], VictoryConditions::Standard,),
            Some(VictoryOutcome::HqCaptured {
                winner: Faction::Alliance,
                ..
            })
        ));
    }

    /// A prisoner counts whoever holds it (`FUN_005180e0` tests only `+0xac`
    /// bit 0), so leaders the adrift sweep jailed with no captor
    /// (`FUN_00549580`, status 0) count for either side's Standard win.
    #[test]
    fn leaders_jailed_with_no_captor_count_for_either_sides_standard_win() {
        for winner in [Faction::Alliance, Faction::Empire] {
            let (mut world, alliance_hq, empire_hq) = make_world();
            let leaders = if winner == Faction::Alliance {
                world.systems[empire_hq].is_destroyed = true;
                [PALPATINE, VADER]
            } else {
                world.systems[alliance_hq].is_destroyed = true;
                [LUKE, MON_MOTHMA]
            };
            for id in leaders {
                let leader = leader(&mut world, id);
                leader.is_captive = true;
                leader.abode = Some(crate::world::Abode::jail_of(winner != Faction::Alliance));
            }

            let state = VictoryState::new(alliance_hq, empire_hq);
            let outcome =
                VictorySystem::check(&state, &world, &[tick(1)], VictoryConditions::Standard);

            let won = match outcome {
                Some(
                    VictoryOutcome::HqCaptured { winner: side, .. }
                    | VictoryOutcome::HqDestroyed { winner: side, .. },
                ) => Some(side),
                Some(VictoryOutcome::DeathStarVictory { .. }) => Some(Faction::Empire),
                None => None,
            };
            assert_eq!(won, Some(winner));
        }
    }

    #[test]
    fn incomplete_imperial_objective_does_not_mask_complete_alliance_victory() {
        let (mut world, alliance_hq, empire_hq) = make_world();
        world.systems[alliance_hq].is_headquarters = false;
        world.systems[alliance_hq].control = ControlKind::Controlled(Faction::Empire);
        world.systems[empire_hq].control = ControlKind::Controlled(Faction::Alliance);
        capture_leader(&mut world, PALPATINE, Faction::Alliance);
        capture_leader(&mut world, VADER, Faction::Alliance);

        let state = VictoryState::new(alliance_hq, empire_hq);
        assert!(matches!(
            VictorySystem::check(&state, &world, &[tick(1)], VictoryConditions::Standard),
            Some(VictoryOutcome::HqCaptured {
                winner: Faction::Alliance,
                ..
            })
        ));
    }

    #[test]
    fn standard_death_star_route_waits_for_both_alliance_leaders() {
        let (mut world, alliance_hq, empire_hq) = make_world();
        world.systems.get_mut(alliance_hq).unwrap().is_destroyed = true;
        let state = VictoryState::new(alliance_hq, empire_hq);

        assert!(
            VictorySystem::check(&state, &world, &[tick(1)], VictoryConditions::Standard,)
                .is_none()
        );

        capture_leader(&mut world, LUKE, Faction::Empire);
        capture_leader(&mut world, MON_MOTHMA, Faction::Empire);
        assert!(matches!(
            VictorySystem::check(&state, &world, &[tick(1)], VictoryConditions::Standard),
            Some(VictoryOutcome::DeathStarVictory { .. })
        ));
    }
}
