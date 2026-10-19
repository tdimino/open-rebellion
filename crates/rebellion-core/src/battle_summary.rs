//! The Battle Summary's contents: the end-of-battle results window (window
//! kind 8, `FUN_0046f140`) over the battle result object (`FUN_0049ac00`;
//! `ghidra/notes/tactical-results.md`).
//!
//! Each side's forces fill three lists that the window's four filter tabs
//! cut by family, as the Operation Report's do
//! ([`crate::operation_report`]): Operational (Survivors on Personnel),
//! Captured and Destroyed (Killed). The destroyed lists come from the
//! Battle Alert's record of every object at the battle's start, filed by
//! its destroyed bit (`FUN_0049b720`); the others are what still stands
//! (`FUN_00472010`). There is no Damaged list: damage shows only as the
//! picture's burn marks (manual p. 153, Fig. 4.18).

use crate::ids::{CharacterKey, FleetKey};
use crate::operation_report::{character, regiment, squadron, with_family, ReportUnit};
use crate::tactical::{BattleSession, OriginalBattleRng, TacticalShip};
use crate::troop_transport::TroopTransportState;
use crate::uprising::Draws;
use crate::world::GameWorld;

/// One entry of a list: the object, and whether its picture carries the
/// burn marks of a damaged object (GOKRES `+0x2000`, `FUN_0042c3b0.c:350`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SummaryEntry {
    pub unit: ReportUnit,
    pub burned: bool,
}

/// One side's lists.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BattleSummarySide {
    /// Operational, on Personnel the Survivors.
    pub operational: Vec<SummaryEntry>,
    /// Personnel whose captive bit (`+0xac` bit 0) is set.
    pub captured: Vec<SummaryEntry>,
    /// Destroyed, on Personnel the Killed.
    pub destroyed: Vec<SummaryEntry>,
}

/// A regiment recorded aboard a fleet at the battle's start.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Rider {
    unit: ReportUnit,
    fleet: FleetKey,
    is_alliance: bool,
    /// The carrying ship by its tag; 0 rides on the fleet.
    carrier: u32,
}

/// A character aboard one of the battle's fleets when it began.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrewMember {
    pub key: CharacterKey,
    pub unit: ReportUnit,
    pub fleet: FleetKey,
    /// The fleet's side, the side whose list files it (`FUN_00472010`).
    pub is_alliance: bool,
    /// The roster index of the ship it rides
    /// ([`crate::carriage::character_ship`]); `None` on a fleet with no
    /// live ship.
    pub ship: Option<usize>,
    /// Effective combat, slot `+0x1f0` (`FUN_004edc40`, short `+0x86`).
    pub combat: i32,
    pub major: bool,
    /// The captive bit (`+0xac` bit 0).
    pub captive: bool,
}

/// What the import does to a character (`FUN_00502840`, the capital ship's
/// hook `+0x11c`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrewFate {
    /// Its ship lives on.
    Aboard,
    /// Its ship was lost: `FUN_004fb520` calls its slot `+0xb4`
    /// (`FUN_004ef2e0` -> `FUN_00534790`, the Adrift role), and it lived
    /// through the injury roll.
    Adrift,
    /// Its ship was lost and the injury roll destroyed it.
    Killed,
}

/// GNPRTB 2568 (`0xa08`, `DAT_006b90a8`, shipped 1): the least chance of
/// the ship-loss injury (`FUN_0053ea00`).
const GNPRTB_SHIP_LOSS_INJURY_MIN_CHANCE: u16 = 2568;
/// GNPRTB 2569 (`0xa09`, `DAT_006b90a4`, shipped 1): its base.
const GNPRTB_SHIP_LOSS_INJURY_BASE: u16 = 2569;
/// GNPRTB 2570 (`0xa0a`, `DAT_006b9098`, shipped 29): its spread.
const GNPRTB_SHIP_LOSS_INJURY_SPREAD: u16 = 2570;
/// GNPRTB 2577 (`0xa11`, `DAT_006b90c8`, shipped 25): the percent chance
/// that an injured minor dies (`FUN_0056f160` -> `FUN_0053eb30`).
const GNPRTB_MINOR_DEATH_CHANCE: u16 = 2577;

/// The strategic draws (`FUN_0053e290`, `FUN_0053e2f0`) over the battle's
/// stream. port: the write-back holds no strategic stream, so the crew
/// rolls draw from the battle's, after the battle.
struct BattleDraws<'a>(&'a mut OriginalBattleRng);

impl Draws for BattleDraws<'_> {
    fn draw(&mut self, n: i32) -> i32 {
        if n < 0 {
            -self.0.between(0, 1 - n)
        } else {
            self.0.between(0, n + 1)
        }
    }

    /// `FUN_0053e2f0(percent)`: a draw over `0..=99` below `percent`.
    fn chance(&mut self, percent: i32) -> bool {
        self.draw(99) < percent
    }
}

/// The fleets of the battle, attacker's first, in the order the session
/// meets them.
fn battle_fleets(session: &BattleSession) -> Vec<FleetKey> {
    let mut fleets: Vec<FleetKey> = Vec::new();
    for fleet in [session.attacker_fleet, session.defender_fleet]
        .into_iter()
        .chain(session.ships.iter().map(|ship| ship.fleet))
        .chain(
            session
                .death_star
                .and_then(|station| station.ship)
                .map(|(fleet, _)| Some(fleet)),
        )
        .flatten()
    {
        if !fleets.contains(&fleet) {
            fleets.push(fleet);
        }
    }
    fleets
}

/// Every character aboard the battle's fleets, as the world holds them
/// before the write-back. port: the effective combat (`FUN_004edc40`,
/// short `+0x86`) is the base plus half the variance, as the uprising's
/// injury reads it (`uprising.rs`).
#[must_use]
pub fn crew(session: &BattleSession, world: &GameWorld) -> Vec<CrewMember> {
    let mut out = Vec::new();
    for fleet in battle_fleets(session) {
        let Some(value) = world.fleets.get(fleet) else {
            continue;
        };
        let ship = crate::carriage::character_ship(value);
        for &key in &value.characters {
            let Some(found) = world.characters.get(key).filter(|found| !found.is_killed) else {
                continue;
            };
            out.extend(character(world, key).map(|unit| CrewMember {
                key,
                unit,
                fleet,
                is_alliance: value.is_alliance,
                ship,
                combat: (found.combat.base + found.combat.variance / 2).cast_signed(),
                major: found.is_major,
                captive: found.is_captive,
            }));
        }
    }
    out
}

/// Whether the strategic ship at `index` of `fleet` lives on after the
/// battle; one the battle did not hold lives on.
fn roster_ship_survives(session: &BattleSession, fleet: FleetKey, index: usize) -> bool {
    if let Some(ship) = session
        .ships
        .iter()
        .find(|ship| ship.fleet == Some(fleet) && ship.fleet_ship_index == index)
    {
        return ship_survives(ship);
    }
    match session.death_star {
        Some(station) if station.ship == Some((fleet, index)) => death_star_survives(station.hull),
        _ => true,
    }
}

/// The import's crew pass, in `crew`'s order. A ship destroyed in battle
/// (reason `0xc`, `FUN_0040cfc0`, `FUN_0040d170`) runs its hook `+0x11c`
/// (`FUN_00502840`, every capital-ship vtable): `FUN_004fb520` sets each
/// child adrift, then, its reason being 1 to `0x10`, each character aboard
/// takes slot `+0x2e8` (`FUN_004ef640`): with chance
/// `max(G2568, 100 - combat)` an injury `rand(chance) + rand(G2570) + G2569`
/// (`FUN_0053ea00`), then slot `+0x2f0` (`FUN_004ef6d0`), which destroys it
/// when slot `+0x2e0` holds: never for a major (`FUN_006158b0`), a GNPRTB
/// 2577 roll for a minor (`FUN_0056f160`). port: the injury itself is not
/// stored (F-026).
pub fn crew_fates(
    session: &BattleSession,
    crew: &[CrewMember],
    world: &GameWorld,
    rng: &mut OriginalBattleRng,
) -> Vec<CrewFate> {
    let param = |id, shipped| {
        i32::try_from(crate::movement::gnprtb_or_shipped(world, id, shipped)).unwrap_or(0)
    };
    let mut draws = BattleDraws(rng);
    crew.iter()
        .map(|member| {
            let lost = member
                .ship
                .is_some_and(|index| !roster_ship_survives(session, member.fleet, index));
            if !lost {
                return CrewFate::Aboard;
            }
            let injured = crate::uprising::roll_injury(
                member.combat,
                param(GNPRTB_SHIP_LOSS_INJURY_MIN_CHANCE, 1),
                param(GNPRTB_SHIP_LOSS_INJURY_SPREAD, 29),
                param(GNPRTB_SHIP_LOSS_INJURY_BASE, 1),
                &mut draws,
            )
            .is_some_and(|injury| injury > 0);
            if injured && !member.major && draws.chance(param(GNPRTB_MINOR_DEATH_CHANCE, 25)) {
                CrewFate::Killed
            } else {
                CrewFate::Adrift
            }
        })
        .collect()
}

/// The import's crew pass for one battle: each character aboard the
/// battle's fleets as the battle ended, with its fate, and the battle's
/// generator after the draws.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettledCrew {
    pub members: Vec<(CrewMember, CrewFate)>,
    pub rng: OriginalBattleRng,
}

/// Draw `session`'s crew pass ([`crew`], [`crew_fates`]) from its
/// generator as it stands.
#[must_use]
pub fn settle_crew(session: &BattleSession, world: &GameWorld) -> SettledCrew {
    let members = crew(session, world);
    let mut rng = session.battle_rng;
    let fates = crew_fates(session, &members, world, &mut rng);
    SettledCrew {
        members: members.into_iter().zip(fates).collect(),
        rng,
    }
}

/// The Battle Alert's record (`FUN_0049c0d0.c:70-155`): what each side's
/// fleets carried when the battle began. The ships and squadrons stay in the
/// session; the regiments and characters are taken from the strategic world,
/// which a simulated battle changes before its results show.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BattleRecord {
    /// The tag of each session ship's strategic ship, by session index.
    ship_tags: Vec<u32>,
    /// The Death Star's entry and its ship's tag.
    death_star: Option<(ReportUnit, u32)>,
    regiments: Vec<Rider>,
}

/// A ship lives on after the import unless it was lost in the battle: one
/// that withdrew leaves with its hull (`FUN_0040cfc0` destroys only state
/// `0xb`, a live hull reading at least 1.0). The test
/// `tactical::apply::apply_results` writes back.
fn ship_survives(ship: &TacticalShip) -> bool {
    (ship.alive || ship.retreated) && ship.hull_current > 0
}

impl BattleRecord {
    /// Record `session`'s fleets as the world holds them before the battle.
    #[must_use]
    pub fn record(
        session: &BattleSession,
        world: &GameWorld,
        transport: &TroopTransportState,
    ) -> Self {
        let ship_tags = session
            .ships
            .iter()
            .map(|ship| {
                ship.fleet
                    .and_then(|fleet| world.fleets.get(fleet))
                    .and_then(|fleet| fleet.capital_ships.get(ship.fleet_ship_index))
                    .map_or(0, |found| found.tag)
            })
            .collect();
        let death_star = session
            .death_star
            .and_then(|station| station.ship)
            .and_then(|(fleet, index)| world.fleets.get(fleet)?.capital_ships.get(index))
            .and_then(|ship| {
                Some((
                    crate::operation_report::capital_ship(world, ship)?,
                    ship.tag,
                ))
            });
        let mut regiments = Vec::new();
        for fleet in battle_fleets(session) {
            let Some(value) = world.fleets.get(fleet) else {
                continue;
            };
            // FUN_0049c0d0.c:79,90-95: the regiments of each usable ship
            // (mode 1), so none aboard a ship unfinished or on its own leg.
            // port: a regiment with no carrier rides the fleet itself.
            let aboard_usable = |carrier: u32| {
                carrier == 0
                    || value
                        .capital_ships
                        .iter()
                        .any(|ship| ship.usable() && ship.tag == carrier)
            };
            for &troop in transport
                .cargo(fleet)
                .iter()
                .filter(|&&troop| aboard_usable(transport.carrier(troop)))
            {
                regiments.extend(regiment(world, troop).map(|unit| Rider {
                    unit,
                    fleet,
                    is_alliance: value.is_alliance,
                    carrier: transport.carrier(troop),
                }));
            }
        }
        Self {
            ship_tags,
            death_star,
            regiments,
        }
    }

    /// Whether the strategic ship tagged `tag` in `fleet` lives on.
    /// port: a tag no battle ship carries names a ship the battle did not
    /// touch, which lives on.
    fn carrier_survives(&self, session: &BattleSession, fleet: FleetKey, tag: u32) -> bool {
        if let Some((ship, _)) = session
            .ships
            .iter()
            .zip(&self.ship_tags)
            .find(|(ship, &ship_tag)| ship.fleet == Some(fleet) && ship_tag == tag)
        {
            return ship_survives(ship);
        }
        match (session.death_star, &self.death_star) {
            (Some(station), Some((_, station_tag)))
                if *station_tag == tag && station.ship.map(|(key, _)| key) == Some(fleet) =>
            {
                death_star_survives(station.hull)
            }
            _ => true,
        }
    }

    /// The lists of the side `alliance` (rail button `0x70` for side 1,
    /// `0x71` for side 2, `FUN_004714e0.c:80-90`), in the battle's order:
    /// the ships, the Death Star, the squadrons, the regiments, the
    /// characters.
    #[must_use]
    pub fn side(
        &self,
        session: &BattleSession,
        world: &GameWorld,
        alliance: bool,
    ) -> BattleSummarySide {
        let mut out = BattleSummarySide::default();
        let file = |out: &mut BattleSummarySide, unit: ReportUnit, lost: bool, burned: bool| {
            let entry = SummaryEntry {
                unit,
                burned: lost || burned,
            };
            if lost {
                out.destroyed.push(entry);
            } else {
                out.operational.push(entry);
            }
        };
        for ship in session
            .ships
            .iter()
            .filter(|ship| ship.identity.is_alliance == alliance)
        {
            if let Some(class) = world.capital_ship_classes.get(ship.class_key) {
                let unit = ReportUnit {
                    dat_id: with_family(class.dat_id, class.family),
                    name: ship.name.clone(),
                    major: false,
                    object: None,
                };
                // The damaged bit (`+0x50` bit 0x200, FUN_00501c00): hull
                // damage or a damage counter, as the import writes them.
                file(
                    &mut out,
                    unit,
                    !ship_survives(ship),
                    ship.hull_current < ship.hull_max || ship.subsystem_damage.total_hits() > 0,
                );
            }
        }
        // The Death Star is a ship of family 0x18, filed on Capital Ships
        // (FUN_004714e0.c:92-101): destroyed when its station has no hull
        // left (FUN_0040d170, the hull truncated as `__ftol` does), damaged
        // (FUN_00501c00) when the station leaves its ship less than the
        // class hull.
        if let (Some(station), Some((unit, _))) = (session.death_star, &self.death_star) {
            if station.is_alliance == alliance {
                file(
                    &mut out,
                    unit.clone(),
                    !death_star_survives(station.hull),
                    station.hull < station.hull_max,
                );
            }
        }
        // FUN_0040d260: a squadron is destroyed when its craft count is 0,
        // which FUN_005b8e00 gives only in state 0xb; one docked or jumped
        // keeps a count of 1 or more. Its damaged bit is its squad-size
        // damage, the craft lost (FUN_00503340); the port seats every
        // squadron at its class's size.
        for fighter in session
            .fighters
            .iter()
            .filter(|fighter| fighter.identity.is_alliance == alliance)
        {
            let Some(class) = world.fighter_classes.get(fighter.class_key) else {
                continue;
            };
            if let Some(unit) = squadron(world, fighter.class_key) {
                file(
                    &mut out,
                    unit,
                    fighter.squad_count == 0,
                    fighter.squad_count < class.squadron_size,
                );
            }
        }
        // A regiment has no import; it dies with its carrying ship
        // (FUN_004f8710, the destroyed container's children).
        for rider in self
            .regiments
            .iter()
            .filter(|rider| rider.is_alliance == alliance)
        {
            let lost = !self.carrier_survives(session, rider.fleet, rider.carrier);
            file(&mut out, rider.unit.clone(), lost, false);
        }
        // The import's crew pass, the one draw `apply_results` applies
        // (`BattleSession::settle_crew`). A destroyed character is filed
        // Killed (`FUN_0049b720`, bit 3). FUN_00472010 (tab 0x68) walks the
        // side's existing ships (`FUN_00502db0` mode 3) and files a captive
        // (`+0xac` bit 0) Captured, any other character Survivors; one adrift
        // from a lost ship sits in no existing ship and in no in-transit list
        // (`+0x70` takes bit 0x20), so no column holds it.
        let settled = session
            .settled_crew
            .clone()
            .unwrap_or_else(|| settle_crew(session, world));
        for (member, fate) in settled.members {
            if member.is_alliance != alliance {
                continue;
            }
            let entry = SummaryEntry {
                unit: member.unit.clone(),
                burned: false,
            };
            match fate {
                CrewFate::Killed => out.destroyed.push(entry),
                CrewFate::Adrift => {}
                CrewFate::Aboard if member.captive => out.captured.push(entry),
                CrewFate::Aboard => out.operational.push(entry),
            }
        }
        out
    }
}

/// `FUN_0040d170`: the station's hull truncated to the ship's.
fn death_star_survives(hull: f32) -> bool {
    hull.max(0.0).trunc() > 0.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::carriage::ProductRoom;
    use crate::ids::DatId;
    use crate::tactical::test_support::{test_death_star, test_fighter, test_session, test_ship};
    use crate::tactical::{CombatWinner, TacticalDock};
    use crate::world::{
        BuildableClass, CapitalShipClass, Character, FighterClass, Fleet, ShipInstance, System,
        TroopUnit,
    };

    /// port: test fixture values, not rules.
    struct Battle {
        world: GameWorld,
        transport: TroopTransportState,
        fleet: FleetKey,
        cruiser: crate::ids::CapitalShipKey,
        wing: crate::ids::FighterKey,
    }

    fn battle(alliance: bool, tags: &[u32]) -> Battle {
        let mut world = GameWorld::default();
        let system = world.systems.insert(System::default());
        let cruiser = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(0x1400_0040),
            name: "Cruiser".into(),
            troop_capacity: 4,
            ..CapitalShipClass::default()
        });
        let wing = world.fighter_classes.insert(FighterClass {
            dat_id: DatId::new(0x1c00_0001),
            name: "Wing".into(),
            squadron_size: 12,
            ..FighterClass::default()
        });
        let ships = tags
            .iter()
            .map(|&tag| {
                let mut ship = ShipInstance::new(cruiser, 10, alliance);
                ship.tag = tag;
                ship
            })
            .collect();
        let fleet = world.fleets.insert(Fleet {
            location: system,
            capital_ships: ships,
            is_alliance: alliance,
            ..Fleet::default()
        });
        world.systems[system].fleets.push(fleet);
        Battle {
            world,
            transport: TroopTransportState::default(),
            fleet,
            cruiser,
            wing,
        }
    }

    fn ship(battle: &Battle, index: usize, alliance: bool, hull: i32) -> TacticalShip {
        let mut ship = test_ship(0x40, index, alliance, hull > 0);
        ship.class_key = battle.cruiser;
        ship.fleet = Some(battle.fleet);
        ship.fleet_ship_index = index;
        ship.hull_max = 10;
        ship.hull_current = hull;
        ship.name = format!("Cruiser {}", index + 1);
        ship
    }

    fn names(entries: &[SummaryEntry]) -> Vec<&str> {
        entries
            .iter()
            .map(|entry| entry.unit.name.as_str())
            .collect()
    }

    #[test]
    fn a_ship_is_destroyed_only_with_no_hull_and_otherwise_operational_however_damaged() {
        // FUN_0040cfc0 -> FUN_00501ae0: only state 0xb destroys a ship; a
        // live hull reads at least 1.0. There is no Damaged column
        // (FUN_0046f360.c:314-322); damage is the picture's burn marks.
        for alliance in [true, false] {
            let battle = battle(alliance, &[0, 0, 0, 0]);
            let mut withdrew = ship(&battle, 3, alliance, 4);
            withdrew.alive = false;
            withdrew.retreated = true;
            let mut session = test_session(
                vec![
                    ship(&battle, 0, alliance, 10),
                    ship(&battle, 1, alliance, 1),
                    ship(&battle, 2, alliance, 0),
                    withdrew,
                    ship(&battle, 0, !alliance, 10),
                ],
                Vec::new(),
                true,
            );
            session.attacker_is_alliance = alliance;
            let record = BattleRecord::record(&session, &battle.world, &battle.transport);
            let side = record.side(&session, &battle.world, alliance);

            assert_eq!(
                names(&side.operational),
                ["Cruiser 1", "Cruiser 2", "Cruiser 4"]
            );
            assert_eq!(names(&side.destroyed), ["Cruiser 3"]);
            assert!(side.captured.is_empty());
            assert_eq!(
                side.operational
                    .iter()
                    .map(|entry| entry.burned)
                    .collect::<Vec<_>>(),
                [false, true, true],
                "alliance {alliance}"
            );
            assert!(side.destroyed[0].burned);
            assert_eq!(side.operational[0].unit.family(), 0x14);
        }
    }

    #[test]
    fn a_squadron_is_destroyed_only_with_no_craft_and_one_docked_or_jumped_stays_operational() {
        // FUN_0040d260 -> FUN_00503370: destroyed at craft count 0, which
        // FUN_005b8e00 gives only in state 0xb; docked (states 1, 5) or
        // jumped (state 9) keeps a count of 1 or more.
        for alliance in [true, false] {
            let battle = battle(alliance, &[]);
            let squadron = |edit: &dyn Fn(&mut crate::tactical::TacticalFighter)| {
                let mut fighter = test_fighter(1, alliance);
                fighter.class_key = battle.wing;
                edit(&mut fighter);
                fighter
            };
            let session = test_session(
                Vec::new(),
                vec![
                    squadron(&|fighter| {
                        fighter.docked = Some(TacticalDock::Ready);
                        fighter.alive = false;
                    }),
                    squadron(&|fighter| {
                        fighter.docked = Some(TacticalDock::Recovered);
                        fighter.hull_current = 40.0;
                        fighter.squad_count = 5;
                    }),
                    squadron(&|fighter| fighter.alive = false),
                    squadron(&|fighter| {
                        fighter.alive = false;
                        fighter.hull_current = 0.0;
                        fighter.squad_count = 0;
                    }),
                    {
                        let mut other = test_fighter(5, !alliance);
                        other.class_key = battle.wing;
                        other
                    },
                ],
                true,
            );
            let record = BattleRecord::record(&session, &battle.world, &battle.transport);
            let side = record.side(&session, &battle.world, alliance);

            assert_eq!(side.operational.len(), 3, "alliance {alliance}");
            assert_eq!(side.destroyed.len(), 1, "alliance {alliance}");
            assert_eq!(side.destroyed[0].unit.family(), 0x1c);
            assert_eq!(
                side.operational
                    .iter()
                    .map(|entry| entry.burned)
                    .collect::<Vec<_>>(),
                [false, true, false]
            );
        }
    }

    #[test]
    fn the_death_star_is_a_capital_ship_destroyed_only_when_its_hull_truncates_to_none() {
        // FUN_004714e0.c:92-101: tab 0x65 holds families [0x14,0x1c), the
        // Death Star's 0x18 among them; FUN_0040d170 destroys it when the
        // station's hull, truncated, is 0.
        let mut battle = battle(false, &[0]);
        let station_class = battle.world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(0x1800_0001),
            family: 0x18,
            name: "Death Star".into(),
            ..CapitalShipClass::default()
        });
        battle.world.fleets[battle.fleet].capital_ships[0] =
            ShipInstance::new(station_class, 10, false);
        let side = |hull: f32| {
            let mut session = test_session(Vec::new(), Vec::new(), true);
            let mut station = test_death_star(false);
            station.ship = Some((battle.fleet, 0));
            station.hull = hull;
            session.death_star = Some(station);
            let record = BattleRecord::record(&session, &battle.world, &battle.transport);
            assert_eq!(
                record.side(&session, &battle.world, true),
                BattleSummarySide::default()
            );
            record.side(&session, &battle.world, false)
        };

        let whole = side(100.0);
        assert_eq!(names(&whole.operational), ["Death Star"]);
        assert_eq!(whole.operational[0].unit.family(), 0x18);
        assert!(!whole.operational[0].burned);
        assert!(side(40.0).operational[0].burned);
        assert_eq!(names(&side(0.5).destroyed), ["Death Star"]);
        assert_eq!(names(&side(0.0).destroyed), ["Death Star"]);
    }

    #[test]
    fn a_regiment_dies_with_the_ship_that_carries_it_not_by_the_fleet_s_room() {
        // FUN_00472010 tab 0x67 lists each ship's own regiments
        // (FUN_00504c40(ship)); a destroyed ship's children die with it
        // (FUN_004f8710). The surviving ship's room is not what decides.
        for alliance in [true, false] {
            let mut battle = battle(alliance, &[1, 2]);
            for (name, raw) in [("Army", 0x1000_0001), ("Fleet Troops", 0x1000_0002)] {
                battle.world.buildable_classes.insert(
                    DatId::new(raw),
                    BuildableClass {
                        name: name.into(),
                        ..BuildableClass::default()
                    },
                );
            }
            let system = battle.world.fleets[battle.fleet].location;
            let troops: Vec<_> = [0x1000_0001, 0x1000_0001, 0x1000_0002]
                .into_iter()
                .map(|raw| {
                    battle.world.troops.insert(TroopUnit {
                        class_dat_id: DatId::new(raw),
                        is_alliance: alliance,
                    })
                })
                .collect();
            battle.world.systems[system].ground_units = troops.clone();
            battle
                .transport
                .embark(
                    &mut battle.world,
                    &ProductRoom::default(),
                    battle.fleet,
                    &troops,
                )
                .expect("room aboard");
            let carried_by_first = troops
                .iter()
                .filter(|&&troop| battle.transport.carrier(troop) == 1)
                .count();
            let session = test_session(
                vec![
                    ship(&battle, 0, alliance, 10),
                    ship(&battle, 1, alliance, 0),
                ],
                Vec::new(),
                true,
            );
            let record = BattleRecord::record(&session, &battle.world, &battle.transport);
            let side = record.side(&session, &battle.world, alliance);

            let regiments = |entries: &[SummaryEntry]| {
                entries
                    .iter()
                    .filter(|entry| entry.unit.family() == 0x10)
                    .count()
            };
            assert_eq!(
                regiments(&side.operational),
                carried_by_first,
                "alliance {alliance}"
            );
            assert_eq!(
                regiments(&side.destroyed),
                3 - carried_by_first,
                "alliance {alliance}"
            );
            assert!(carried_by_first < 3, "the second ship carries some");
        }
    }

    #[test]
    fn the_record_lists_no_regiment_aboard_a_ship_on_its_own_leg_for_either_side() {
        // FUN_0049c0d0.c:68,79,90-95 walk the fleets in mode 1, each one's
        // ships in mode 1 and each ship's children in mode 1: a ship on its
        // own leg from its yard (bit 5, FUN_00556430) is not usable
        // (0x004f7b80), so its regiments are not in the record.
        for alliance in [true, false] {
            let mut battle = battle(alliance, &[1, 2]);
            battle.world.buildable_classes.insert(
                DatId::new(0x1000_0001),
                BuildableClass {
                    name: "Army".into(),
                    ..BuildableClass::default()
                },
            );
            let system = battle.world.fleets[battle.fleet].location;
            let troops: Vec<_> = (0..2)
                .map(|_| {
                    battle.world.troops.insert(TroopUnit {
                        class_dat_id: DatId::new(0x1000_0001),
                        is_alliance: alliance,
                    })
                })
                .collect();
            battle.world.systems[system].ground_units = troops.clone();
            battle
                .transport
                .embark(
                    &mut battle.world,
                    &ProductRoom::default(),
                    battle.fleet,
                    &troops,
                )
                .expect("room aboard");
            let carriers: Vec<u32> = troops
                .iter()
                .map(|&troop| battle.transport.carrier(troop))
                .collect();
            assert!(carriers.contains(&1) && carriers.contains(&2));
            battle.world.fleets[battle.fleet].capital_ships[1].transit =
                Some(crate::world::ShipTransit {
                    from: system,
                    to: system,
                    arrival_tick: 9,
                });
            let session = test_session(vec![ship(&battle, 0, alliance, 10)], Vec::new(), true);

            let record = BattleRecord::record(&session, &battle.world, &battle.transport);
            let recorded: Vec<u32> = record.regiments.iter().map(|rider| rider.carrier).collect();
            assert_eq!(recorded, [1], "alliance {alliance}");
        }
    }

    #[test]
    fn burn_marks_follow_the_damaged_bit_hull_or_counters_for_a_ship_and_lost_craft_for_a_squadron()
    {
        // +0x50 bit 0x200 draws the burn marks (FUN_0042c3b0.c:350). A ship
        // sets it with hull damage or any damage counter (FUN_00501c00), which
        // the import writes from the battle (FUN_0040cfc0); a squadron with
        // its squad-size damage, the craft lost (FUN_00503340, slot +0x254,
        // written by FUN_0040d260 from the craft count).
        for alliance in [true, false] {
            let battle = battle(alliance, &[0, 0]);
            let mut hit = ship(&battle, 1, alliance, 10);
            hit.subsystem_damage.engines = 1;
            let squadron = |count: u32, hull: f32| {
                let mut fighter = test_fighter(1, alliance);
                fighter.class_key = battle.wing;
                fighter.squad_count = count;
                fighter.hull_current = hull;
                fighter
            };
            let session = test_session(
                vec![ship(&battle, 0, alliance, 10), hit],
                vec![squadron(12, 96.0), squadron(12, 95.5), squadron(5, 96.0)],
                true,
            );
            let record = BattleRecord::record(&session, &battle.world, &battle.transport);
            let side = record.side(&session, &battle.world, alliance);

            assert_eq!(
                side.operational
                    .iter()
                    .map(|entry| entry.burned)
                    .collect::<Vec<_>>(),
                [false, true, false, false, true],
                "alliance {alliance}"
            );
        }
    }

    #[test]
    fn a_ship_out_of_the_battle_or_left_with_no_hull_is_destroyed() {
        // The write-back's test (apply_results): a ship lives on only alive
        // or withdrawn, and with hull left (FUN_0040cfc0, state 0xb).
        for alliance in [true, false] {
            let battle = battle(alliance, &[0, 0]);
            let mut struck = ship(&battle, 0, alliance, 5);
            struck.alive = false;
            let mut emptied = ship(&battle, 1, alliance, 0);
            emptied.alive = true;
            let session = test_session(vec![struck, emptied], Vec::new(), true);
            let record = BattleRecord::record(&session, &battle.world, &battle.transport);
            let side = record.side(&session, &battle.world, alliance);

            assert!(side.operational.is_empty(), "alliance {alliance}");
            assert_eq!(names(&side.destroyed), ["Cruiser 1", "Cruiser 2"]);
        }
    }

    /// Two regiments of `battle.fleet` as the record holds them: one aboard
    /// the ship tagged 3 and one aboard a ship tagged 9.
    fn regiments_aboard(battle: &Battle, alliance: bool) -> Vec<Rider> {
        [3, 9]
            .into_iter()
            .map(|carrier| Rider {
                unit: ReportUnit {
                    dat_id: DatId::new(0x1000_0001),
                    name: format!("Army {carrier}"),
                    major: false,
                    object: None,
                },
                fleet: battle.fleet,
                is_alliance: alliance,
                carrier,
            })
            .collect()
    }

    #[test]
    fn a_regiment_dies_with_the_death_star_that_carries_it() {
        // FUN_004f8710: a destroyed container's children die with it.
        // port: a tag no battle ship carries lives on.
        let mut battle = battle(false, &[1, 3]);
        let station_class = battle.world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(0x1800_0001),
            family: 0x18,
            name: "Death Star".into(),
            troop_capacity: 4,
            ..CapitalShipClass::default()
        });
        let mut station_ship = ShipInstance::new(station_class, 10, false);
        station_ship.tag = 3;
        battle.world.fleets[battle.fleet].capital_ships[1] = station_ship;
        let riders = regiments_aboard(&battle, false);
        let side = |cruiser_hull: i32, station_hull: f32| {
            let mut session = test_session(
                vec![ship(&battle, 0, false, cruiser_hull)],
                Vec::new(),
                true,
            );
            let mut station = test_death_star(false);
            station.ship = Some((battle.fleet, 1));
            station.hull = station_hull;
            session.death_star = Some(station);
            let mut record = BattleRecord::record(&session, &battle.world, &battle.transport);
            record.regiments.clone_from(&riders);
            let side = record.side(&session, &battle.world, false);
            side.destroyed
                .iter()
                .filter(|entry| entry.unit.family() == 0x10)
                .map(|entry| entry.unit.name.clone())
                .collect::<Vec<_>>()
        };

        assert!(side(10, 100.0).is_empty());
        // The station alone keeps the fleet, and its own regiment.
        assert!(side(0, 100.0).is_empty());
        // The station's regiment goes with it; the cruiser keeps the fleet.
        assert_eq!(side(10, 0.0), ["Army 3"]);
        // Tag 9 names no ship of the battle.
        assert_eq!(side(0, 0.0), ["Army 3"]);
    }

    /// A major officer, a minor pilot and a prisoner of the other side
    /// aboard `battle.fleet`, every combat 0, so the ship-loss injury roll
    /// always hits (`max(G2568, 100 - 0)`), and GNPRTB 2577 at 100, so an
    /// injured minor always dies.
    fn crew_aboard(battle: &mut Battle, alliance: bool) -> [CharacterKey; 3] {
        battle.world.gnprtb = crate::world::GnprtbParams::uniform(&[(2577, 100)]);
        let mut add = |raw: u32, name: &str, major: bool, captive: bool, side: bool| {
            battle.world.characters.insert(Character {
                dat_id: DatId::new(raw),
                name: name.into(),
                is_major: major,
                is_captive: captive,
                is_alliance: side,
                ..Character::default()
            })
        };
        let keys = [
            add(0x3000_0240, "Officer", true, false, alliance),
            add(0x3800_0340, "Pilot", false, false, alliance),
            add(0x3800_0341, "Prisoner", false, true, !alliance),
        ];
        battle.world.fleets[battle.fleet].characters = keys.to_vec();
        keys
    }

    #[test]
    fn a_lost_ship_s_minor_dies_on_the_gnprtb_2577_roll_and_a_major_is_only_set_adrift() {
        // FUN_00502840 (capital ship hook +0x11c, reason 0xc in 1..0x10):
        // each character aboard takes FUN_004ef640's injury
        // (FUN_0053ea00, chance max(G2568, 100 - combat)), then
        // FUN_004ef6d0 destroys it when slot +0x2e0 holds: FUN_006158b0 (0)
        // for a major, the G2577 roll for a minor (FUN_0056f160). Every
        // child is set adrift first (FUN_004fb520 -> FUN_004ef2e0).
        for alliance in [true, false] {
            let mut battle = battle(alliance, &[0]);
            crew_aboard(&mut battle, alliance);
            let fates = |hull: i32| {
                let session =
                    test_session(vec![ship(&battle, 0, alliance, hull)], Vec::new(), true);
                let members = crew(&session, &battle.world);
                crew_fates(
                    &session,
                    &members,
                    &battle.world,
                    &mut OriginalBattleRng::default(),
                )
            };

            assert_eq!(fates(10), [CrewFate::Aboard; 3], "alliance {alliance}");
            assert_eq!(
                fates(0),
                [CrewFate::Adrift, CrewFate::Killed, CrewFate::Killed],
                "alliance {alliance}"
            );
        }
    }

    #[test]
    fn personnel_reads_the_killed_as_killed_a_captive_aboard_as_captured_and_lists_no_one_adrift() {
        // FUN_004714e0: Killed from the record's destroyed list
        // (FUN_0049b720, bit 3); Survivors and Captured (+0xac bit 0) from
        // the side's existing ships (FUN_00472010, FUN_00502db0 mode 3),
        // where a character adrift from a lost ship no longer sits.
        for alliance in [true, false] {
            let mut battle = battle(alliance, &[0]);
            crew_aboard(&mut battle, alliance);
            let side = |hull: i32| {
                let session =
                    test_session(vec![ship(&battle, 0, alliance, hull)], Vec::new(), true);
                let record = BattleRecord::record(&session, &battle.world, &battle.transport);
                assert_eq!(
                    record.side(&session, &battle.world, !alliance),
                    BattleSummarySide::default()
                );
                record.side(&session, &battle.world, alliance)
            };

            let kept = side(10);
            assert_eq!(names(&kept.operational), ["Cruiser 1", "Officer", "Pilot"]);
            assert_eq!(names(&kept.captured), ["Prisoner"]);
            assert_eq!(names(&kept.destroyed), Vec::<&str>::new());
            let lost = side(0);
            assert_eq!(names(&lost.operational), Vec::<&str>::new());
            assert!(lost.captured.is_empty(), "alliance {alliance}");
            assert_eq!(names(&lost.destroyed), ["Cruiser 1", "Pilot", "Prisoner"]);
        }
    }

    #[test]
    fn the_strategic_draws_span_zero_to_n_and_a_chance_draws_below_its_percent() {
        // FUN_0053e290(n): 0..=n, or -(0..=-n) for a negative n;
        // FUN_0053e2f0(p): a draw of 0..=99 below p. port: over the
        // battle's stream (FUN_005af420's between).
        for (n, span) in [(29, 30), (0, 1), (-5, 6)] {
            let mut ours = OriginalBattleRng::default();
            let mut theirs = OriginalBattleRng::default();
            for _ in 0..8 {
                let expected = theirs.between(0, span);
                let expected = if n < 0 { -expected } else { expected };
                assert_eq!(BattleDraws(&mut ours).draw(n), expected, "n {n}");
            }
        }
        let mut ours = OriginalBattleRng::default();
        let mut theirs = OriginalBattleRng::default();
        for percent in [0, 1, 37, 50, 99, 100] {
            for _ in 0..8 {
                let draw = theirs.between(0, 100);
                assert_eq!(BattleDraws(&mut ours).chance(percent), draw < percent);
            }
        }
    }

    #[test]
    fn a_chance_misses_on_a_draw_equal_to_its_percent_and_hits_one_below() {
        // FUN_0053e2f0: `FUN_0053e2e0() < percent`, strict, where
        // FUN_0053e2e0 draws FUN_0053e290(99).
        for offset in [0, 1] {
            let mut ours = OriginalBattleRng::default();
            let mut theirs = OriginalBattleRng::default();
            for _ in 0..16 {
                let draw = theirs.between(0, 100);
                assert_eq!(
                    BattleDraws(&mut ours).chance(draw + offset),
                    offset == 1,
                    "draw {draw}"
                );
            }
        }
    }

    #[test]
    fn a_ship_the_battle_did_not_hold_keeps_its_crew_aboard_when_the_death_star_falls() {
        // FUN_00502840 runs only on a ship the import destroys (reason 0xc,
        // FUN_0040cfc0 for a battle ship, FUN_0040d170 for the station); a
        // ship of the fleet the battle never held is not written back. Only
        // the Empire fields a Death Star.
        let mut battle = battle(false, &[0, 0]);
        let station_class = battle.world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(0x1800_0001),
            family: 0x18,
            name: "Death Star".into(),
            ..CapitalShipClass::default()
        });
        battle.world.fleets[battle.fleet].capital_ships[1] =
            ShipInstance::new(station_class, 10, false);
        crew_aboard(&mut battle, false);
        let mut session = test_session(Vec::new(), Vec::new(), true);
        let mut station = test_death_star(false);
        station.ship = Some((battle.fleet, 1));
        station.hull = 0.0;
        session.death_star = Some(station);
        let members = crew(&session, &battle.world);
        assert!(members.iter().all(|member| member.ship == Some(0)));

        assert_eq!(
            crew_fates(
                &session,
                &members,
                &battle.world,
                &mut OriginalBattleRng::default()
            ),
            [CrewFate::Aboard; 3]
        );
    }

    #[test]
    fn a_lost_ship_s_minor_with_a_wound_of_exactly_nothing_lives_adrift() {
        // FUN_004ef6d0 acts only on a wound above 0 (`-1 < w && w != 0`).
        // Combat 0 makes the injury chance 100 (FUN_0053ea00), so the
        // wound is rand(100) + rand(G2570) + G2569; G2569 is set to cancel
        // the two draws, or to leave 1. GNPRTB 2577 at 100 kills any
        // injured minor.
        let mut probe = OriginalBattleRng::default();
        let mut draws = BattleDraws(&mut probe);
        assert!(draws.chance(100));
        let rolled = draws.draw(100) + draws.draw(29);
        assert!(rolled > 1, "a base of 0 would read as the shipped 1");
        for alliance in [true, false] {
            let mut battle = battle(alliance, &[0]);
            let pilot = battle.world.characters.insert(Character {
                dat_id: DatId::new(0x3800_0340),
                name: "Pilot".into(),
                is_alliance: alliance,
                ..Character::default()
            });
            battle.world.fleets[battle.fleet].characters = vec![pilot];
            let fates = |base: i32| {
                let mut world = battle.world.clone();
                world.gnprtb = crate::world::GnprtbParams::uniform(&[(2577, 100), (2569, base)]);
                let session = test_session(vec![ship(&battle, 0, alliance, 0)], Vec::new(), true);
                let members = crew(&session, &world);
                crew_fates(
                    &session,
                    &members,
                    &world,
                    &mut OriginalBattleRng::default(),
                )
            };

            assert_eq!(fates(-rolled), [CrewFate::Adrift], "alliance {alliance}");
            assert_eq!(fates(1 - rolled), [CrewFate::Killed], "alliance {alliance}");
        }
    }

    #[test]
    fn a_crew_member_carries_its_effective_combat_and_the_ship_it_rides() {
        // FUN_004edc40 (+0x1f0, short +0x86). port: base plus half the
        // variance, the uprising's reading; it rides the fleet's first live
        // ship (carriage::character_ship).
        for alliance in [true, false] {
            let mut battle = battle(alliance, &[0, 0]);
            battle.world.fleets[battle.fleet].capital_ships[0].alive = false;
            let officer = battle.world.characters.insert(Character {
                dat_id: DatId::new(0x3000_0240),
                name: "Officer".into(),
                combat: crate::world::SkillPair {
                    base: 40,
                    variance: 30,
                },
                ..Character::default()
            });
            battle.world.fleets[battle.fleet].characters = vec![officer];
            let session = test_session(vec![ship(&battle, 1, alliance, 10)], Vec::new(), true);
            let members = crew(&session, &battle.world);
            assert_eq!(members.len(), 1);
            assert_eq!(members[0].combat, 55, "alliance {alliance}");
            assert_eq!(members[0].ship, Some(1));
            assert_eq!(members[0].is_alliance, alliance);
        }
    }

    #[test]
    fn the_death_star_s_crew_shares_its_fate_and_a_wound_of_nothing_kills_no_one() {
        // FUN_00502840 is the hook of both capital-ship classes (0x0065d650,
        // the Death Star's 0x0065d930), so a lost station's characters take
        // the roll; FUN_004ef6d0 acts only on a wound above 0.
        let mut battle = battle(false, &[0, 0]);
        let station_class = battle.world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(0x1800_0001),
            family: 0x18,
            name: "Death Star".into(),
            ..CapitalShipClass::default()
        });
        battle.world.fleets[battle.fleet].capital_ships[0] =
            ShipInstance::new(station_class, 10, false);
        crew_aboard(&mut battle, false);
        let fates = |hull: f32, world: &GameWorld| {
            let mut session = test_session(vec![ship(&battle, 1, false, 10)], Vec::new(), true);
            let mut station = test_death_star(false);
            station.ship = Some((battle.fleet, 0));
            station.hull = hull;
            session.death_star = Some(station);
            let members = crew(&session, world);
            crew_fates(&session, &members, world, &mut OriginalBattleRng::default())
        };

        assert_eq!(fates(100.0, &battle.world), [CrewFate::Aboard; 3]);
        assert_eq!(
            fates(0.0, &battle.world),
            [CrewFate::Adrift, CrewFate::Killed, CrewFate::Killed]
        );
        let mut harmless = battle.world.clone();
        harmless.gnprtb = crate::world::GnprtbParams::uniform(&[(2577, 100), (2569, -200)]);
        assert_eq!(fates(0.0, &harmless), [CrewFate::Adrift; 3]);
    }

    // FUN_0040a050 imports the battle (FUN_0040c780, state 0x16/0x17)
    // before the result event of state 9 builds the summary, so the
    // summary files the fates the import applied. The crew pass is drawn
    // once as the battle ends; a character who left the fleet after the
    // Battle Alert's record is neither drawn for nor filed.
    #[test]
    fn the_summary_files_the_crew_fates_the_write_back_applies_for_either_side() {
        for alliance in [true, false] {
            let mut battle = battle(alliance, &[0]);
            let [officer, pilot, prisoner] = crew_aboard(&mut battle, alliance);
            let system = battle.world.fleets[battle.fleet].location;
            let enemy_class = battle.world.capital_ship_classes.insert(CapitalShipClass {
                dat_id: DatId::new(0x1400_0041),
                hull: 10,
                ..CapitalShipClass::default()
            });
            let enemy = battle.world.fleets.insert(Fleet {
                location: system,
                capital_ships: vec![ShipInstance::new(enemy_class, 10, !alliance)],
                is_alliance: !alliance,
                ..Fleet::default()
            });
            battle.world.systems[system].fleets.push(enemy);
            for key in [officer, pilot, prisoner] {
                battle.world.characters[key].current_system = Some(system);
                battle.world.characters[key].current_fleet = Some(battle.fleet);
            }
            let mut session =
                BattleSession::new(&battle.world, system, enemy, battle.fleet, true, 1, 0x5eed);
            let record = BattleRecord::record(&session, &battle.world, &battle.transport);
            // The pilot leaves the fleet before the battle ends.
            battle.world.fleets[battle.fleet]
                .characters
                .retain(|&key| key != pilot);
            battle.world.characters[pilot].current_fleet = None;
            for ship in session
                .ships
                .iter_mut()
                .filter(|ship| ship.fleet == Some(battle.fleet))
            {
                ship.alive = false;
                ship.hull_current = 0;
            }
            session.winner = Some(CombatWinner::Attacker);

            session.settle_crew(&battle.world);
            let summary = record.side(&session, &battle.world, alliance);
            crate::tactical::apply::apply_results(
                &session,
                &mut battle.world,
                &mut battle.transport,
                &mut crate::movement::MovementState::new(),
                &ProductRoom::default(),
                &mut Vec::new(),
            );

            let killed: Vec<&str> = [officer, pilot, prisoner]
                .into_iter()
                .filter(|&key| battle.world.characters[key].is_killed)
                .map(|key| battle.world.characters[key].name.as_str())
                .collect();
            assert_eq!(killed, ["Prisoner"], "alliance {alliance}");
            assert_eq!(
                names(&summary.destroyed),
                ["Cruiser", "Prisoner"],
                "alliance {alliance}"
            );
            // Simulate shows the results after the write-back: the same draw.
            assert_eq!(
                record.side(&session, &battle.world, alliance),
                summary,
                "alliance {alliance}"
            );
        }
    }

    #[test]
    fn the_write_back_kills_the_minors_and_holds_the_rest_adrift_at_the_system_uncaptured() {
        // The import's crew pass (FUN_00502840) and, with the fleet gone, its
        // characters held adrift by the system for the adrift sweep
        // (FUN_004ef2e0 -> FUN_00534790); no path takes them captive.
        for alliance in [true, false] {
            let mut battle = battle(alliance, &[0]);
            let [officer, pilot, prisoner] = crew_aboard(&mut battle, alliance);
            let system = battle.world.fleets[battle.fleet].location;
            let enemy_class = battle.world.capital_ship_classes.insert(CapitalShipClass {
                dat_id: DatId::new(0x1400_0041),
                hull: 10,
                ..CapitalShipClass::default()
            });
            let enemy = battle.world.fleets.insert(Fleet {
                location: system,
                capital_ships: vec![ShipInstance::new(enemy_class, 10, !alliance)],
                is_alliance: !alliance,
                ..Fleet::default()
            });
            battle.world.systems[system].fleets.push(enemy);
            for key in [officer, pilot, prisoner] {
                battle.world.characters[key].current_system = Some(system);
                battle.world.characters[key].current_fleet = Some(battle.fleet);
            }
            let mut session =
                BattleSession::new(&battle.world, system, enemy, battle.fleet, true, 1, 0x5eed);
            for ship in session
                .ships
                .iter_mut()
                .filter(|ship| ship.fleet == Some(battle.fleet))
            {
                ship.alive = false;
                ship.hull_current = 0;
            }
            session.winner = Some(CombatWinner::Attacker);

            crate::tactical::apply::apply_results(
                &session,
                &mut battle.world,
                &mut battle.transport,
                &mut crate::movement::MovementState::new(),
                &ProductRoom::default(),
                &mut Vec::new(),
            );

            assert!(!battle.world.fleets.contains_key(battle.fleet));
            let character = |key: CharacterKey| &battle.world.characters[key];
            assert!(character(pilot).is_killed, "alliance {alliance}");
            assert!(character(prisoner).is_killed);
            assert!(!character(officer).is_killed);
            assert!(character(officer).adrift);
            assert!(!character(officer).is_captive);
            assert_eq!(character(officer).captured_by, None);
            assert_eq!(character(officer).current_system, Some(system));
            assert_eq!(character(officer).current_fleet, None);
        }
    }
}
