//! The four icons a sector window draws around each planet (`FUN_00459e30`),
//! which rule shows each one, and its art. Recovery notes:
//! `ghidra/notes/sector-quadrants.md`.
//!
//! Each rule ends in `FUN_0045d140(item, state, enabled)`. The state is a
//! side, which picks the art (`FUN_0045ca80`), and a zero `enabled` removes
//! the item, so an icon with nothing to show is not drawn.

use std::collections::{HashMap, HashSet};

use rebellion_core::dat::{ExplorationStatus, Faction};
use rebellion_core::fog::FogState;
use rebellion_core::ids::SystemKey;
use rebellion_core::missions::{MissionMember, MissionState};
use rebellion_core::movement::MovementState;
use rebellion_core::world::GameWorld;

use crate::fleet_window::{control_side, faction_side, fleet_side, icon_side};
use crate::system_window::opposing_contents_visible;

/// One of the four overlay items, by its kind (`flag >> 16`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Quadrant {
    /// Kind 4, top-left: the System window (type 9).
    System,
    /// Kind 8, bottom-left: the System Defenses window (type 10).
    Defenses,
    /// Kind `0x10`, top-right: the Fleet window (type 4).
    Fleets,
    /// Kind `0x40`, bottom-right: the Missions window (type 11).
    Missions,
}

impl Quadrant {
    /// The order `FUN_00459e30` creates the items in.
    pub const ALL: [Self; 4] = [Self::System, Self::Defenses, Self::Fleets, Self::Missions];
}

/// `FUN_0045ca80(kind, state, 0/1)`: the item's two bitmaps for a side.
/// Kinds 4 and 8 have art for sides 0 and 3 as well; the fleet and mission
/// kinds have none.
#[must_use]
pub const fn quadrant_art(quadrant: Quadrant, side: u8) -> Option<(u32, u32)> {
    match (quadrant, side) {
        (Quadrant::System, 1) => Some((10771, 10772)),
        (Quadrant::System, 2) => Some((10779, 10780)),
        (Quadrant::System, 0 | 3) => Some((10787, 10788)),
        (Quadrant::Defenses, 1) => Some((10773, 10774)),
        (Quadrant::Defenses, 2) => Some((10781, 10782)),
        (Quadrant::Defenses, 0 | 3) => Some((10789, 10790)),
        (Quadrant::Fleets, 1) => Some((10775, 10776)),
        (Quadrant::Fleets, 2) => Some((10783, 10784)),
        (Quadrant::Missions, 1) => Some((10777, 10778)),
        (Quadrant::Missions, 2) => Some((10785, 10786)),
        _ => None,
    }
}

/// The side a quadrant item shows, or `None` when it is hidden.
#[must_use]
pub fn quadrant_side(
    world: &GameWorld,
    fog: &FogState,
    missions: &MissionState,
    movement: &MovementState,
    player: Faction,
    system: SystemKey,
    quadrant: Quadrant,
) -> Option<u8> {
    let contents = SystemContents::new(world, fog, player, system)?;
    match quadrant {
        // FUN_0045cdc0: the system's side bits while it has manufacturing or
        // production facilities (families 0x28..0x30, FUN_0053b6e0).
        Quadrant::System => (contents.manufacturing_and_production() > 0).then(|| contents.side()),
        // FUN_0045ce80: the system's side bits while it has defense
        // facilities, regiments, squadrons, or personnel off a visible
        // mission.
        Quadrant::Defenses => (contents.defenses(missions) > 0).then(|| contents.side()),
        // FUN_0045ccc0, ported in `fleet_window`.
        Quadrant::Fleets => match icon_side(world, movement, fog, player, system) {
            (_, 0) => None,
            (side, _) => Some(side),
        },
        // FUN_0045d090: FUN_004a1f60's side; disabled for 0 and 3.
        Quadrant::Missions => match mission_side(&contents, missions) {
            0 | 3 => None,
            side => Some(side),
        },
    }
}

/// The characters and special forces at `system` the player sees, each with
/// its side and whether it is on a hidden mission; none at an unexplored
/// system.
pub(crate) fn visible_members(
    world: &GameWorld,
    fog: &FogState,
    player: Faction,
    system: SystemKey,
) -> Vec<(MissionMember, u8, bool)> {
    SystemContents::new(world, fog, player, system)
        .map(|contents| contents.members().collect())
        .unwrap_or_default()
}

/// `FUN_004a1f60(system, player side)` over what the player sees: the other
/// side's missions win, then the player's, else 3.
pub(crate) fn missions_side(
    world: &GameWorld,
    fog: &FogState,
    missions: &MissionState,
    player: Faction,
    system: SystemKey,
) -> u8 {
    SystemContents::new(world, fog, player, system)
        .map_or(3, |contents| mission_side(&contents, missions))
}

/// The quadrant item's two bitmaps, or `None` when it is not drawn.
#[must_use]
pub fn quadrant_icon(
    world: &GameWorld,
    fog: &FogState,
    missions: &MissionState,
    movement: &MovementState,
    player: Faction,
    system: SystemKey,
    quadrant: Quadrant,
) -> Option<(u32, u32)> {
    quadrant_side(world, fog, missions, movement, player, system, quadrant)
        .and_then(|side| quadrant_art(quadrant, side))
}

/// What the player sees at a system: every side's objects where the System
/// window shows opposing contents, otherwise only the player's own.
///
/// port: the original reads the galaxy view's system (hyp: the player
/// side's view, `FUN_00539fd0`); the port uses the System window's
/// visibility so an icon never reveals what that window hides.
struct SystemContents<'a> {
    world: &'a GameWorld,
    system: SystemKey,
    player_side: u8,
    opposing_visible: bool,
}

impl<'a> SystemContents<'a> {
    fn new(
        world: &'a GameWorld,
        fog: &FogState,
        player: Faction,
        system: SystemKey,
    ) -> Option<Self> {
        let value = world.systems.get(system)?;
        if value.exploration_status == ExplorationStatus::Unexplored {
            return None;
        }
        Some(Self {
            world,
            system,
            player_side: faction_side(player),
            opposing_visible: opposing_contents_visible(world, fog, player, system),
        })
    }

    /// The system's side bits `(+0x24 >> 6) & 3`.
    fn side(&self) -> u8 {
        self.world
            .systems
            .get(self.system)
            .map_or(0, |value| control_side(value.control))
    }

    fn seen(&self, side: u8) -> bool {
        self.opposing_visible || side == self.player_side
    }

    fn manufacturing_and_production(&self) -> usize {
        let Some(value) = self.world.systems.get(self.system) else {
            return 0;
        };
        let manufacturing = value.manufacturing_facilities.iter().filter(|key| {
            self.world
                .manufacturing_facilities
                .get(**key)
                .is_some_and(|facility| self.seen(crate::fleet_window::faction_side(facility.side)))
        });
        let production = value.production_facilities.iter().filter(|key| {
            self.world
                .production_facilities
                .get(**key)
                .is_some_and(|facility| self.seen(crate::fleet_window::faction_side(facility.side)))
        });
        manufacturing.count() + production.count()
    }

    /// `FUN_0045ce80`'s sum: defense facilities (`FUN_00526fd0`), regiments
    /// (`FUN_00504c40`), squadrons (`FUN_005039d0`), and each member of
    /// families `0x30..0x40` with no mission key or on a hidden mission.
    ///
    /// port: squadrons live only aboard fleets in the port, so none count.
    fn defenses(&self, missions: &MissionState) -> usize {
        let Some(value) = self.world.systems.get(self.system) else {
            return 0;
        };
        let defense = value.defense_facilities.iter().filter(|key| {
            self.world
                .defense_facilities
                .get(**key)
                .is_some_and(|facility| self.seen(crate::fleet_window::faction_side(facility.side)))
        });
        let regiments = value.ground_units.iter().filter(|key| {
            self.world
                .troops
                .get(**key)
                .is_some_and(|troop| self.seen(fleet_side(troop.is_alliance)))
        });
        let keys = mission_keys(missions);
        let idle = self
            .members()
            .filter(|(member, _, hidden)| *hidden || !keys.contains_key(member));
        defense.count() + regiments.count() + idle.count()
    }

    /// The system's characters and special forces the player sees, each with
    /// its side and whether it is on a hidden mission.
    fn members(&self) -> impl Iterator<Item = (MissionMember, u8, bool)> + '_ {
        system_members(self.world, self.system).filter(|(_, side, _)| self.seen(*side))
    }
}

/// The characters and special forces at `system`, each with its side and
/// whether it is on a hidden mission (role `+0x78` bit 8).
///
/// port: special forces carry no hidden-mission flag.
pub(crate) fn system_members(
    world: &GameWorld,
    system: SystemKey,
) -> impl Iterator<Item = (MissionMember, u8, bool)> + '_ {
    let characters = world
        .characters
        .iter()
        .filter(move |(_, character)| {
            !character.is_killed && character.current_system == Some(system)
        })
        .map(|(key, character)| {
            let side = if character.is_alliance {
                1
            } else if character.is_empire {
                2
            } else {
                0
            };
            (
                MissionMember::Character(key),
                side,
                character.on_hidden_mission,
            )
        });
    let forces = world
        .systems
        .get(system)
        .into_iter()
        .flat_map(|value| value.special_forces.iter())
        .filter_map(|key| {
            let force = world.special_forces.get(*key)?;
            Some((
                MissionMember::SpecialForce(*key),
                fleet_side(force.is_alliance),
                false,
            ))
        });
    characters.chain(forces)
}

/// Each mission member's mission key (`+0x68`): the mission whose team,
/// decoys, or captured list holds it.
pub(crate) fn mission_keys(missions: &MissionState) -> HashMap<MissionMember, u64> {
    let mut keys = HashMap::new();
    for mission in missions.missions() {
        for member in mission
            .team
            .iter()
            .chain(&mission.decoys)
            .chain(&mission.captured)
        {
            keys.insert(*member, mission.id);
        }
    }
    keys
}

/// `FUN_004a1f60(system, player side)`: the distinct visible missions of
/// the system's members, split by side. The other side's missions win, then
/// the player's; with neither it returns 3.
fn mission_side(contents: &SystemContents<'_>, missions: &MissionState) -> u8 {
    let own = contents.player_side;
    let other = if own == 1 { 2 } else { 1 };
    let keys = mission_keys(missions);
    let mut own_missions = HashSet::new();
    let mut other_missions = HashSet::new();
    for (member, side, hidden) in contents.members() {
        let Some(&key) = keys.get(&member) else {
            continue;
        };
        if hidden {
            continue;
        }
        if side == own {
            own_missions.insert(key);
        } else if side == other {
            other_missions.insert(key);
        }
    }
    if !other_missions.is_empty() {
        other
    } else if !own_missions.is_empty() {
        own
    } else {
        3
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::dat::SectorGroup;
    use rebellion_core::ids::{CharacterKey, DatId};
    use rebellion_core::missions::{MissionFaction, MissionKind, MissionRequest};
    use rebellion_core::world::{
        Character, ControlKind, DefenseFacilityInstance, ManufacturingFacilityInstance,
        ProductionFacilityInstance, Sector, SpecialForceUnit, System, TroopUnit,
    };

    fn world(control: ControlKind) -> (GameWorld, SystemKey) {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(36),
            name: "Sesswenna".into(),
            group: SectorGroup::Core,
            x: 317,
            y: 248,
            systems: Vec::new(),
        });
        let system = world.systems.insert(System {
            dat_id: DatId::new(100),
            name: "Sluis Van".into(),
            sector,
            x: 320,
            y: 250,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: Vec::new(),
            ground_units: Vec::new(),
            special_forces: Vec::new(),
            defense_facilities: Vec::new(),
            manufacturing_facilities: Vec::new(),
            production_facilities: Vec::new(),
            is_headquarters: false,
            is_destroyed: false,
            control,
        });
        (world, system)
    }

    fn alliance_world() -> (GameWorld, SystemKey) {
        world(ControlKind::Controlled(Faction::Alliance))
    }

    fn side(
        world: &GameWorld,
        missions: &MissionState,
        player: Faction,
        system: SystemKey,
        quadrant: Quadrant,
    ) -> Option<u8> {
        // The player sees the system, so every side's objects count.
        let mut fog = FogState::new(player);
        fog.reveal(system);
        quadrant_side(
            world,
            &fog,
            missions,
            &rebellion_core::movement::MovementState::default(),
            player,
            system,
            quadrant,
        )
    }

    fn alliance_side(world: &GameWorld, system: SystemKey, quadrant: Quadrant) -> Option<u8> {
        side(
            world,
            &MissionState::new(),
            Faction::Alliance,
            system,
            quadrant,
        )
    }

    fn add_mine(world: &mut GameWorld, system: SystemKey, is_alliance: bool) {
        let mine = world
            .production_facilities
            .insert(ProductionFacilityInstance {
                class_dat_id: DatId::new(0x2c00_0001),
                side: rebellion_core::dat::Faction::of_alliance(is_alliance),
                is_mine: true,
            });
        world.systems[system].production_facilities.push(mine);
    }

    fn add_character(world: &mut GameWorld, system: SystemKey, is_alliance: bool) -> CharacterKey {
        world.characters.insert(Character {
            dat_id: DatId::new(832),
            name: "Agent".into(),
            is_alliance,
            is_empire: !is_alliance,
            current_system: Some(system),
            recruited: true,
            ..Default::default()
        })
    }

    fn send_on_mission(missions: &mut MissionState, member: CharacterKey, system: SystemKey) {
        missions.dispatch(MissionRequest::single(
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            member,
            system,
            None,
            0,
        ));
    }

    #[test]
    fn each_kind_has_its_own_art_for_each_side() {
        // FUN_0045ca80: kinds 4 and 8 also have art for sides 0 and 3; the
        // fleet and mission kinds have none.
        let table = [
            (
                Quadrant::System,
                [
                    Some((10787, 10788)),
                    Some((10771, 10772)),
                    Some((10779, 10780)),
                    Some((10787, 10788)),
                ],
            ),
            (
                Quadrant::Defenses,
                [
                    Some((10789, 10790)),
                    Some((10773, 10774)),
                    Some((10781, 10782)),
                    Some((10789, 10790)),
                ],
            ),
            (
                Quadrant::Fleets,
                [None, Some((10775, 10776)), Some((10783, 10784)), None],
            ),
            (
                Quadrant::Missions,
                [None, Some((10777, 10778)), Some((10785, 10786)), None],
            ),
        ];
        for (quadrant, art) in table {
            for (side, expected) in (0u8..).zip(art) {
                assert_eq!(
                    quadrant_art(quadrant, side),
                    expected,
                    "{quadrant:?} side {side}"
                );
            }
            assert_eq!(quadrant_art(quadrant, 4), None);
        }
    }

    #[test]
    fn an_empty_system_shows_no_icon() {
        // FUN_0045d140: a zero count removes the item.
        let (world, system) = alliance_world();
        for quadrant in Quadrant::ALL {
            assert_eq!(
                alliance_side(&world, system, quadrant),
                None,
                "{quadrant:?}"
            );
        }
    }

    #[test]
    fn the_system_icon_shows_the_systems_side_while_it_has_manufacturing_or_production() {
        // FUN_0045cdc0: FUN_0053b6e0 counts families 0x28..0x30; the state is
        // the system's side bits.
        let (mut world, system) = world(ControlKind::Controlled(Faction::Empire));
        add_mine(&mut world, system, false);
        assert_eq!(
            side(
                &world,
                &MissionState::new(),
                Faction::Empire,
                system,
                Quadrant::System
            ),
            Some(2)
        );

        let (mut world, system) = alliance_world();
        let yard = world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                class_dat_id: DatId::new(0x2800_0001),
                side: rebellion_core::dat::Faction::Alliance,
                is_shipyard: true,
            });
        world.systems[system].manufacturing_facilities.push(yard);
        assert_eq!(alliance_side(&world, system, Quadrant::System), Some(1));
    }

    #[test]
    fn the_system_icon_takes_a_neutral_or_contested_systems_side() {
        // FUN_0045cdc0 passes the side bits 0 and 3 through; FUN_0045ca80
        // gives both the neutral art.
        for (control, expected) in [(ControlKind::Uncontrolled, 0), (ControlKind::Contested, 3)] {
            let (mut world, system) = world(control);
            add_mine(&mut world, system, true);
            assert_eq!(
                alliance_side(&world, system, Quadrant::System),
                Some(expected)
            );
        }
    }

    #[test]
    fn defenses_do_not_light_the_system_icon() {
        // FUN_0053b6e0's range 0x28..0x30 excludes the defense families.
        let (mut world, system) = alliance_world();
        let shield = world.defense_facilities.insert(DefenseFacilityInstance {
            class_dat_id: DatId::new(0x2400_0003),
            side: rebellion_core::dat::Faction::Alliance,
        });
        world.systems[system].defense_facilities.push(shield);
        assert_eq!(alliance_side(&world, system, Quadrant::System), None);
        assert_eq!(alliance_side(&world, system, Quadrant::Defenses), Some(1));
    }

    #[test]
    fn a_regiment_or_a_special_force_lights_the_defenses_icon() {
        // FUN_0045ce80: FUN_00504c40 counts regiments, FUN_00536da0 the
        // families 0x30..0x40 that hold special forces.
        let (mut world, system) = alliance_world();
        let regiment = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0001),
            is_alliance: true,
            regiment_strength: 100,
        });
        world.systems[system].ground_units.push(regiment);
        assert_eq!(alliance_side(&world, system, Quadrant::Defenses), Some(1));

        let (mut world, system) = alliance_world();
        let force = world.special_forces.insert(SpecialForceUnit {
            class_dat_id: DatId::new(0x3c00_0001),
            is_alliance: true,
            skills: [0; 8],
            on_mission: false,
        });
        world.systems[system].special_forces.push(force);
        assert_eq!(alliance_side(&world, system, Quadrant::Defenses), Some(1));
        assert_eq!(alliance_side(&world, system, Quadrant::System), None);
    }

    #[test]
    fn personnel_light_the_defenses_icon_only_off_a_visible_mission() {
        // FUN_0045ce80 counts a member when FUN_004ece60 finds no mission key
        // (+0x68) or role +0x78 bit 8 (OnHiddenMission) is set.
        let (mut world, system) = alliance_world();
        let agent = add_character(&mut world, system, true);
        let player = Faction::Alliance;
        assert_eq!(
            side(
                &world,
                &MissionState::new(),
                player,
                system,
                Quadrant::Defenses
            ),
            Some(1)
        );

        let mut missions = MissionState::new();
        send_on_mission(&mut missions, agent, system);
        assert_eq!(
            side(&world, &missions, player, system, Quadrant::Defenses),
            None
        );

        world.characters[agent].on_hidden_mission = true;
        assert_eq!(
            side(&world, &missions, player, system, Quadrant::Defenses),
            Some(1)
        );
    }

    #[test]
    fn the_missions_icon_shows_the_players_side_for_its_own_visible_mission() {
        // FUN_0045d090 / FUN_004a1f60: the player's set only, so its side.
        let (mut world, system) = alliance_world();
        let agent = add_character(&mut world, system, true);
        let mut missions = MissionState::new();
        let player = Faction::Alliance;
        assert_eq!(
            side(&world, &missions, player, system, Quadrant::Missions),
            None
        );

        send_on_mission(&mut missions, agent, system);
        assert_eq!(
            side(&world, &missions, player, system, Quadrant::Missions),
            Some(1)
        );
        // An Empire player sees the same mission as the other side's.
        assert_eq!(
            side(
                &world,
                &missions,
                Faction::Empire,
                system,
                Quadrant::Missions
            ),
            Some(1)
        );
    }

    #[test]
    fn the_other_sides_missions_take_the_missions_icon() {
        // FUN_004a1f60 returns the other side whenever its set is non-empty.
        let (mut world, system) = alliance_world();
        let ours = add_character(&mut world, system, true);
        let theirs = add_character(&mut world, system, false);
        let mut missions = MissionState::new();
        send_on_mission(&mut missions, ours, system);
        send_on_mission(&mut missions, theirs, system);
        assert_eq!(
            side(
                &world,
                &missions,
                Faction::Alliance,
                system,
                Quadrant::Missions
            ),
            Some(2)
        );
        assert_eq!(
            side(
                &world,
                &missions,
                Faction::Empire,
                system,
                Quadrant::Missions
            ),
            Some(1)
        );
    }

    #[test]
    fn a_hidden_mission_does_not_light_the_missions_icon() {
        // FUN_004a1f60 skips members with role +0x78 bit 8.
        let (mut world, system) = alliance_world();
        let agent = add_character(&mut world, system, true);
        world.characters[agent].on_hidden_mission = true;
        let mut missions = MissionState::new();
        send_on_mission(&mut missions, agent, system);
        assert_eq!(
            side(
                &world,
                &missions,
                Faction::Alliance,
                system,
                Quadrant::Missions
            ),
            None
        );
    }

    #[test]
    fn members_elsewhere_or_killed_do_not_count() {
        // FUN_00536da0 walks only the system's own members.
        let (mut world, system) = alliance_world();
        let away = add_character(&mut world, system, true);
        world.characters[away].current_system = None;
        let dead = add_character(&mut world, system, true);
        world.characters[dead].is_killed = true;
        assert_eq!(alliance_side(&world, system, Quadrant::Defenses), None);
    }

    #[test]
    fn hidden_enemy_contents_light_no_icon_until_the_player_sees_the_system() {
        // port: the System window's visibility (opposing_contents_visible).
        let (mut world, system) = world(ControlKind::Controlled(Faction::Empire));
        add_mine(&mut world, system, false);
        let missions = MissionState::new();
        let mut fog = FogState::new(Faction::Alliance);
        assert_eq!(
            quadrant_side(
                &world,
                &fog,
                &missions,
                &rebellion_core::movement::MovementState::default(),
                Faction::Alliance,
                system,
                Quadrant::System
            ),
            None
        );
        fog.reveal(system);
        assert_eq!(
            quadrant_side(
                &world,
                &fog,
                &missions,
                &rebellion_core::movement::MovementState::default(),
                Faction::Alliance,
                system,
                Quadrant::System
            ),
            Some(2)
        );
        // The player's own objects show at an enemy system it cannot see.
        add_mine(&mut world, system, true);
        let blind = FogState::new(Faction::Alliance);
        assert_eq!(
            quadrant_side(
                &world,
                &blind,
                &missions,
                &rebellion_core::movement::MovementState::default(),
                Faction::Alliance,
                system,
                Quadrant::System
            ),
            Some(2)
        );
    }

    #[test]
    fn an_unexplored_system_shows_no_icon() {
        // port: as the System window, an unexplored system lists nothing.
        let (mut world, system) = alliance_world();
        add_mine(&mut world, system, true);
        world.systems[system].exploration_status = ExplorationStatus::Unexplored;
        assert_eq!(alliance_side(&world, system, Quadrant::System), None);
    }

    #[test]
    fn quadrant_icon_pairs_the_side_with_its_art() {
        let (mut world, system) = alliance_world();
        add_mine(&mut world, system, true);
        let fog = FogState::new(Faction::Alliance);
        let missions = MissionState::new();
        assert_eq!(
            quadrant_icon(
                &world,
                &fog,
                &missions,
                &rebellion_core::movement::MovementState::default(),
                Faction::Alliance,
                system,
                Quadrant::System
            ),
            Some((10771, 10772))
        );
        assert_eq!(
            quadrant_icon(
                &world,
                &fog,
                &missions,
                &rebellion_core::movement::MovementState::default(),
                Faction::Alliance,
                system,
                Quadrant::Defenses
            ),
            None
        );
    }
}
