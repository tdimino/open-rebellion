//! The mission detection manager (`FUN_00547f60`): after each phase change a
//! mission's decoys draw off defenders, the defenders try to detect the team,
//! a disloyal member may betray it, and a detected team is exposed
//! (`ghidra/notes/decoy-roll.md`, "Re-read for the port").
//!
//! The run reads the world and returns its changes as [`MissionEffect`]s;
//! the mission's own end code and resign requests change in place.

use std::collections::HashSet;

use crate::dat::Faction;
use crate::ids::{FleetKey, SystemKey, TroopKey};
use crate::missions::{
    member_location_kind, member_skill, running_end_code, ActiveMission, MemberTransit,
    MissionEffect, MissionFaction, MissionMember,
};
use crate::uprising::{
    roll_injury, Draws, UprisingState, GNPRTB_INJURY_BASE, GNPRTB_INJURY_MIN_CHANCE,
    GNPRTB_INJURY_SPREAD,
};
use crate::world::{GameWorld, Skill};

/// GNPRTB 3584 (`DAT_006bb714`): the detection offset.
const GNPRTB_DETECTION_OFFSET: u16 = 3584;
/// GNPRTB 3584 as shipped.
const SHIPPED_DETECTION_OFFSET: i64 = -1;
/// GNPRTB 2565 as shipped: the injury roll's least chance.
const SHIPPED_INJURY_MIN_CHANCE: i64 = 1;
/// GNPRTB 2566 as shipped: the injury roll's base.
const SHIPPED_INJURY_BASE: i64 = 1;
/// GNPRTB 2567 as shipped: the injury roll's spread.
const SHIPPED_INJURY_SPREAD: i64 = 29;

/// Table ids 10..13 of `FUN_0058b420` (`uprising-incident.md`).
const TDECOYTB: &str = "TDECOYTB";
const FDECOYTB: &str = "FDECOYTB";
const FOILTB: &str = "FOILTB";
const RLEVADTB: &str = "RLEVADTB";

/// The phase the betrayal phase runs in: slot `+0x1c8` is `FUN_00592500`,
/// `+0x68 == 9`, for every agent class.
const PHASE_BETRAYAL: u8 = 9;

/// port: a mission's draws for one tick event, taken by the detection run
/// and the phase 10 roll. The original draws each roll inline from its
/// global generator; the port seeds a SplitMix64 per mission and tick from
/// one caller roll, so the caller's budget stays fixed.
pub(crate) struct MissionRng {
    state: u64,
}

impl MissionRng {
    /// port: the seed spreads the roll over 53 bits and mixes in the
    /// mission id with the SplitMix64 increment.
    #[must_use]
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "a roll in [0, 1) scaled to 53 bits fits u64"
    )]
    pub(crate) fn seeded(roll: f64, mission_id: u64) -> Self {
        let bits = (roll.clamp(0.0, 1.0) * f64::from(1u32 << 26) * f64::from(1u32 << 27)) as u64;
        Self {
            state: bits ^ mission_id.wrapping_mul(0x9e37_79b9_7f4a_7c15),
        }
    }

    /// port: one SplitMix64 step.
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
}

impl Draws for MissionRng {
    /// `FUN_0053e290(n)`: uniform over `0..=n`, or `-(0..=-n)` for negative
    /// `n`.
    fn draw(&mut self, n: i32) -> i32 {
        let span = u64::from(n.unsigned_abs());
        let value = i32::try_from(self.next_u64() % (span + 1)).unwrap_or(i32::MAX);
        if n < 0 {
            -value
        } else {
            value
        }
    }

    /// `FUN_0053e2f0(percent)`: a draw over `0..=99` below `percent`.
    fn chance(&mut self, percent: i32) -> bool {
        self.draw(99) < percent
    }
}

/// Which members and defenders a run counts (manager `+0x18..+0x30`).
///
/// port: the setup `FUN_00589a40` sets the ship and fighter flags
/// (`+0x28`, `+0x2c`) only with the fleet flag and the regiment flag
/// (`+0x30`) only with the system flag, so the port folds each into its walk
/// flag.
#[derive(Debug, Default, Clone, Copy)]
struct Flags {
    /// `+0x18`: members standing at a system.
    at_system: bool,
    /// `+0x1c`: members in a fleet.
    in_fleet: bool,
    /// `+0x20` with `+0x30`: walk the system's regiments.
    system: bool,
    /// `+0x24` with `+0x28` and `+0x2c`: walk the fleets' ships and fighters.
    fleets: bool,
}

/// The kinds one walk asks for (`FUN_00587640` arguments 2..4).
#[derive(Debug, Clone, Copy)]
struct Kinds {
    ships: bool,
    fighters: bool,
    regiments: bool,
}

const ALL: Kinds = Kinds {
    ships: true,
    fighters: true,
    regiments: true,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DefenderId {
    Regiment(TroopKey),
    Ship(FleetKey, usize),
    /// A fleet's fighter entry and the squadron within its count.
    Fighter(FleetKey, usize, u32),
}

/// A detector (`ghidra/notes/decoy-roll.md`, "Officer rank and detector
/// slots").
#[derive(Debug, Clone, Copy)]
struct Defender {
    id: DefenderId,
    in_fleet: bool,
    /// Slot `+0x1c4`: the class `detection`.
    detection: i32,
    side: Faction,
}

pub(crate) fn faction_of(is_alliance: bool) -> Faction {
    if is_alliance {
        Faction::Alliance
    } else {
        Faction::Empire
    }
}

/// `FUN_00587640`: the defenders at `location` in walk order.
///
/// port: the port keeps fighters in fleets only, so a system has no
/// fighters of its own, and a fleet's fighters follow all of its ships
/// where the original walks each ship's fighters and regiments after the
/// ship; fleets carry no regiments. A fleet in hyperspace reads its last
/// orbit.
fn walk(world: &GameWorld, location: SystemKey, flags: Flags, kinds: Kinds) -> Vec<Defender> {
    let mut defenders = Vec::new();
    let as_i32 = |value: u32| i32::try_from(value).unwrap_or(i32::MAX);
    if flags.system && kinds.regiments {
        if let Some(system) = world.systems.get(location) {
            for &key in &system.ground_units {
                let Some(troop) = world.troops.get(key) else {
                    continue;
                };
                let detection = world
                    .troop_classes
                    .get(&troop.class_dat_id)
                    .map_or(0, |class| class.detection);
                defenders.push(Defender {
                    id: DefenderId::Regiment(key),
                    in_fleet: false,
                    detection: as_i32(detection),
                    side: faction_of(troop.is_alliance),
                });
            }
        }
    }
    if flags.fleets {
        for (fleet_key, fleet) in &world.fleets {
            if fleet.location != location {
                continue;
            }
            let side = faction_of(fleet.is_alliance);
            if kinds.ships {
                for (index, ship) in fleet.capital_ships.iter().enumerate() {
                    if !ship.alive {
                        continue;
                    }
                    let detection = world
                        .capital_ship_classes
                        .get(ship.class)
                        .map_or(0, |class| class.detection);
                    defenders.push(Defender {
                        id: DefenderId::Ship(fleet_key, index),
                        in_fleet: true,
                        detection: as_i32(detection),
                        side,
                    });
                }
            }
            if kinds.fighters {
                for (index, entry) in fleet.fighters.iter().enumerate() {
                    let detection = world
                        .fighter_classes
                        .get(entry.class)
                        .map_or(0, |class| class.detection);
                    for squadron in 0..entry.count {
                        defenders.push(Defender {
                            id: DefenderId::Fighter(fleet_key, index, squadron),
                            in_fleet: true,
                            detection: as_i32(detection),
                            side,
                        });
                    }
                }
            }
        }
    }
    defenders
}

/// `FUN_0053e340` over a whole table: `Some(outcome)` when the table exists.
///
/// port: a present table always yields a row (`MstbTable::step_lookup`); an
/// absent table is data the port was not given, reported as `None`.
fn table_roll(world: &GameWorld, table: &str, x: i32, rng: &mut MissionRng) -> Option<bool> {
    let table = world.mission_tables.get(table)?;
    let value = i32::try_from(table.step_lookup(x)).unwrap_or(i32::MAX);
    Some(rng.chance(value))
}

fn param(world: &GameWorld, id: u16, shipped: i64) -> i32 {
    i32::try_from(crate::movement::gnprtb_or_shipped(world, id, shipped)).unwrap_or(0)
}

/// One run's state: the manager and its pool.
struct Run<'a> {
    world: &'a GameWorld,
    location: SystemKey,
    flags: Flags,
    can_resign: bool,
    decoyed: HashSet<DefenderId>,
    /// Members captured or destroyed during the run.
    gone: HashSet<MissionMember>,
    /// Pool counts `+0x34`, `+0x2c`, `+0x30`.
    defenders: i32,
    team: i32,
    decoys: i32,
    detected: bool,
}

/// Which list a walk visits.
#[derive(Debug, Clone, Copy)]
struct Lists {
    chars: bool,
    sforces: bool,
    team: bool,
    decoys: bool,
}

impl Run<'_> {
    /// `FUN_005883b0`: a member counts when it has no request, is not
    /// decoying when asked, and stands where the flags look.
    ///
    /// port: only a decoy decoys, and the lists that skip decoying members
    /// (the team's, and betrayal's before any decoy acts) never meet one, so
    /// the port keeps no decoying mark.
    fn counts(&self, mission: &ActiveMission, member: MissionMember) -> bool {
        if mission.resigning.contains(&member) || self.gone.contains(&member) {
            return false;
        }
        match member_location_kind(self.world, member) {
            Some(true) => self.flags.in_fleet,
            Some(false) => self.flags.at_system,
            None => false,
        }
    }

    /// `FUN_00587bb0`: team special forces, team characters, decoy special
    /// forces, decoy characters.
    fn members(&self, mission: &ActiveMission, lists: Lists) -> Vec<MissionMember> {
        let mut out = Vec::new();
        let mut visit = |list: &[MissionMember], special: bool| {
            for &member in list {
                let is_special = matches!(member, MissionMember::SpecialForce(_));
                if is_special == special && self.counts(mission, member) {
                    out.push(member);
                }
            }
        };
        if lists.team {
            if lists.sforces {
                visit(&mission.team, true);
            }
            if lists.chars {
                visit(&mission.team, false);
            }
        }
        if lists.decoys {
            if lists.sforces {
                visit(&mission.decoys, true);
            }
            if lists.chars {
                visit(&mission.decoys, false);
            }
        }
        out
    }

    fn skill(&self, member: MissionMember, skill: Skill) -> i32 {
        member_skill(self.world, member, skill)
            .map_or(0, |value| i32::try_from(value).unwrap_or(i32::MAX))
    }

    /// `FUN_005888f0` / `FUN_005349e0`: the member rolls RLEVADTB against the
    /// defender's officer, evading or being captured. port: the port assigns
    /// no officer ranks, so the captor is the defender and its combat is 0.
    fn expose(
        &mut self,
        mission: &mut ActiveMission,
        defender: Defender,
        member: MissionMember,
        rng: &mut MissionRng,
        out: &mut Vec<MissionEffect>,
    ) {
        let combat = self.skill(member, Skill::Combat);
        match (table_roll(self.world, RLEVADTB, combat, rng), member) {
            // Slot +0x208: evade. A character also rolls the injury.
            (Some(true), MissionMember::Character(character)) => {
                self.request_resign(mission, member);
                self.injure(character, combat, rng, out);
            }
            (Some(true), MissionMember::SpecialForce(_)) => {
                self.request_resign(mission, member);
            }
            // Slot +0x20c: captured by the defender's side.
            (Some(false), MissionMember::Character(character)) => {
                self.injure(character, combat, rng, out);
                out.push(MissionEffect::CharacterCaptured {
                    character,
                    captured_by: match defender.side {
                        Faction::Alliance => MissionFaction::Alliance,
                        _ => MissionFaction::Empire,
                    },
                    at_system: self.location,
                });
                // port: the capture applies after the run, so the run marks
                // the prisoner gone from its lists now.
                self.gone.insert(member);
            }
            // FUN_00503eb0: a captured special force is destroyed.
            (Some(false), MissionMember::SpecialForce(unit)) => {
                out.push(MissionEffect::SpecialForceDestroyed { unit });
                self.gone.insert(member);
            }
            (None, _) => {}
        }
        self.request_resign(mission, member);
        // port: a resigning team member also leaves the team count, which
        // nothing reads once the team is exposed; the port drops it.
        if mission.resigning.contains(&member) && mission.decoys.contains(&member) {
            self.decoys -= 1;
        }
    }

    /// `FUN_00534640` when the member may resign (bit 4, record column 9).
    /// port: the port has no remove requests (bit 2).
    fn request_resign(&self, mission: &mut ActiveMission, member: MissionMember) {
        if self.can_resign && !mission.resigning.contains(&member) {
            mission.resigning.push(member);
        }
    }

    /// Slot `+0x2e4` `FUN_004ef5f0`: the injury roll `FUN_0053e990`.
    fn injure(
        &self,
        character: crate::ids::CharacterKey,
        combat: i32,
        rng: &mut MissionRng,
        out: &mut Vec<MissionEffect>,
    ) {
        if let Some(injury) = roll_injury(
            combat,
            param(
                self.world,
                GNPRTB_INJURY_MIN_CHANCE,
                SHIPPED_INJURY_MIN_CHANCE,
            ),
            param(self.world, GNPRTB_INJURY_SPREAD, SHIPPED_INJURY_SPREAD),
            param(self.world, GNPRTB_INJURY_BASE, SHIPPED_INJURY_BASE),
            rng,
        ) {
            out.push(MissionEffect::CharacterInjured { character, injury });
        }
    }

    /// `FUN_0058a020`, one walk: each defender not decoyed meets a random
    /// decoy (`FUN_00589620`), which draws it off on a TDECOYTB/FDECOYTB roll
    /// or is exposed.
    fn decoy_walk(
        &mut self,
        mission: &mut ActiveMission,
        kinds: Kinds,
        rng: &mut MissionRng,
        out: &mut Vec<MissionEffect>,
    ) {
        let decoy_lists = Lists {
            chars: true,
            sforces: true,
            team: false,
            decoys: true,
        };
        for defender in walk(self.world, self.location, self.flags, kinds) {
            if self.decoyed.contains(&defender.id) {
                continue;
            }
            if self.decoys <= 0 {
                return;
            }
            let pick = rng.draw(self.decoys - 1);
            let Some(decoy) = usize::try_from(pick)
                .ok()
                .and_then(|index| self.members(mission, decoy_lists).get(index).copied())
            else {
                return;
            };
            // port: the port assigns no officer ranks, so the officer term
            // `espionage * G3588 / 100` (DAT_006bb710, shipped 35) is 0.
            let x = self.skill(decoy, Skill::Espionage) - defender.detection;
            let table = if defender.in_fleet {
                FDECOYTB
            } else {
                TDECOYTB
            };
            // port: an absent table leaves the defender in place.
            match table_roll(self.world, table, x, rng) {
                Some(true) => {
                    self.decoyed.insert(defender.id);
                    self.defenders -= 1;
                }
                Some(false) => self.expose(mission, defender, decoy, rng, out),
                None => {}
            }
        }
    }

    /// `FUN_0058a130` with functor `FUN_005896e0`: each defender not decoyed
    /// rolls FOILTB until one detects the team.
    fn detect(&mut self, mission: &ActiveMission, rng: &mut MissionRng) {
        if self.team == 0 {
            return;
        }
        let team_lists = Lists {
            chars: true,
            sforces: true,
            team: true,
            decoys: false,
        };
        let team = self.members(mission, team_lists);
        let espionage: i32 = team
            .iter()
            .map(|&member| self.skill(member, Skill::Espionage))
            .sum::<i32>()
            / self.team;
        let special_forces = i32::try_from(
            team.iter()
                .filter(|member| matches!(member, MissionMember::SpecialForce(_)))
                .count(),
        )
        .unwrap_or(i32::MAX);
        let offset = param(
            self.world,
            GNPRTB_DETECTION_OFFSET,
            SHIPPED_DETECTION_OFFSET,
        );
        for defender in walk(self.world, self.location, self.flags, ALL) {
            if self.decoyed.contains(&defender.id) {
                continue;
            }
            // port: the port assigns no officer ranks, so the officer term
            // `espionage * G3589 / 100` (DAT_006bb70c, shipped 35) is 0.
            let x = espionage - defender.detection - special_forces - offset;
            // port: an absent FOILTB detects nothing.
            if table_roll(self.world, FOILTB, x, rng) == Some(true) {
                self.detected = true;
                return;
            }
        }
    }

    /// `FUN_00589f10`: the first team or decoy member not decoying that
    /// fails `draw(0..99) < 100 - loyalty` betrays the mission
    /// (`FUN_00588da0`, `FUN_0055e520`). port: the other members' learning
    /// the traitor (`+0xa0`, `FUN_00588e80`) is not ported.
    fn betray(&mut self, mission: &ActiveMission, rng: &mut MissionRng) {
        let lists = Lists {
            chars: true,
            sforces: true,
            team: true,
            decoys: true,
        };
        for member in self.members(mission, lists) {
            if rng.chance(100 - self.skill(member, Skill::Loyalty)) {
                self.detected = true;
                return;
            }
        }
    }

    /// `FUN_0058a1c0`'s walks: every team character (families `0x30..0x37`,
    /// then `0x38..0x3b`), then every team special force, not decoying, faces
    /// a random defender (`FUN_00588650`) until none is left.
    ///
    /// port: a member holding a key at `+0x9c` takes `FUN_00588d30`, which is
    /// not read; the port holds no such key.
    fn expose_team(
        &mut self,
        mission: &mut ActiveMission,
        rng: &mut MissionRng,
        out: &mut Vec<MissionEffect>,
    ) {
        let lists = Lists {
            chars: true,
            sforces: false,
            team: true,
            decoys: false,
        };
        let family = |member: &MissionMember| {
            member
                .character()
                .and_then(|key| self.world.characters.get(key))
                .map_or(0, |c| c.dat_id.raw() >> 24)
        };
        let characters = self.members(mission, lists);
        let (early, late): (Vec<_>, Vec<_>) = characters
            .into_iter()
            .partition(|member| family(member) < 0x38);
        let special_forces = self.members(
            mission,
            Lists {
                chars: false,
                sforces: true,
                ..lists
            },
        );
        for member in early.into_iter().chain(late).chain(special_forces) {
            if self.defenders <= 0 {
                return;
            }
            let pick = rng.draw(self.defenders - 1);
            let Some(defender) = usize::try_from(pick).ok().and_then(|index| {
                walk(self.world, self.location, self.flags, ALL)
                    .into_iter()
                    .filter(|defender| !self.decoyed.contains(&defender.id))
                    .nth(index)
            }) else {
                return;
            };
            self.expose(mission, defender, member, rng, out);
        }
    }
}

/// The run `FUN_00547f60` after a phase change: one pass of
/// `FUN_00589970`, then the second pass's validator.
///
/// Modes come from the constant `0x4112` (`FUN_005236e0`): phase 2 mode 2,
/// phases 3 and 7 mode 1, phase 9 mode 4, else 0. The second pass
/// (`+0x44` clear) sets skip for modes 1, 2, and 4, so only its validator
/// acts.
pub(crate) fn run(
    mission: &mut ActiveMission,
    world: &GameWorld,
    uprisings: &UprisingState,
    en_route: &[MemberTransit],
    rng: &mut MissionRng,
    out: &mut Vec<MissionEffect>,
) {
    if mission.end_code != 0 {
        return;
    }
    // FUN_00520ad0 picks the target location +0x78 past phase 4, else the
    // members' location (FUN_00520f40).
    let location = if mission.phase > 4 {
        Some(mission.target_system)
    } else {
        mission.origin
    };
    let Some(location) = location.filter(|&system| world.systems.contains_key(system)) else {
        return;
    };
    let validate = |mission: &mut ActiveMission| {
        if mission.end_code == 0 {
            mission.end_code = running_end_code(mission, world, uprisings, en_route);
        }
    };
    first_pass(mission, world, location, rng, out, &validate);
    validate(mission);
    // FUN_004f9510 tells the opponent of a mission that ended here; not read.
}

fn first_pass(
    mission: &mut ActiveMission,
    world: &GameWorld,
    location: SystemKey,
    rng: &mut MissionRng,
    out: &mut Vec<MissionEffect>,
    validate: &dyn Fn(&mut ActiveMission),
) {
    // FUN_00589a40: the validator, then the skips.
    validate(mission);
    if mission.end_code != 0 {
        return;
    }
    let Some(record) = mission
        .kind
        .record_id()
        .and_then(|id| world.mission_record(id))
    else {
        return;
    };
    if !record.detection_phases {
        return;
    }
    let mode = match mission.phase {
        2 => 2,
        3 | 7 => 1,
        PHASE_BETRAYAL => 4,
        _ => return,
    };
    let side = Faction::from(mission.faction);
    let opponent = if side == Faction::Alliance {
        Faction::Empire
    } else {
        Faction::Alliance
    };
    let fleet_here = |faction: Faction| {
        world
            .fleets
            .values()
            .any(|fleet| fleet.location == location && faction_of(fleet.is_alliance) == faction)
    };
    let opponent_fleet = fleet_here(opponent);
    let own_fleet = fleet_here(side);
    let holder_is_opponent = world
        .systems
        .get(location)
        .and_then(crate::uprising::holder)
        == Some(opponent);
    let mut flags = Flags::default();
    match mode {
        1 => {
            flags.in_fleet = true;
            flags.at_system = mission.origin != Some(mission.target_system);
            if opponent_fleet && !own_fleet {
                flags.fleets = true;
            }
        }
        2 => {
            flags.at_system = true;
            if holder_is_opponent {
                flags.system = true;
            }
        }
        _ => {
            (flags.at_system, flags.in_fleet) = (true, true);
            // port: the target of a system or object kind is its system
            // (target_view), read as a system; a character target is not.
            let target_is_system =
                mission.kind.target_rules() != crate::missions::TargetRules::Character;
            if target_is_system {
                if holder_is_opponent {
                    flags.system = true;
                }
            } else if opponent_fleet {
                flags.fleets = true;
            }
        }
    }
    let mut run = Run {
        world,
        location,
        flags,
        can_resign: record.can_resign,
        decoyed: HashSet::new(),
        gone: HashSet::new(),
        defenders: 0,
        team: 0,
        decoys: 0,
        detected: false,
    };
    // FUN_005885f0.
    run.defenders = i32::try_from(walk(world, location, flags, ALL).len()).unwrap_or(i32::MAX);
    let count =
        |run: &Run<'_>, lists| i32::try_from(run.members(mission, lists).len()).unwrap_or(0);
    run.team = count(
        &run,
        Lists {
            chars: true,
            sforces: true,
            team: true,
            decoys: false,
        },
    );
    run.decoys = count(
        &run,
        Lists {
            chars: true,
            sforces: true,
            team: false,
            decoys: true,
        },
    );
    // FUN_00589e40, the phase for two fixed characters: not read.
    if mission.phase == PHASE_BETRAYAL {
        run.betray(mission, rng);
    }
    if !run.detected {
        if flags.system {
            run.decoy_walk(mission, ALL, rng, out);
        }
        if flags.fleets {
            run.decoy_walk(
                mission,
                Kinds {
                    ships: true,
                    fighters: false,
                    regiments: false,
                },
                rng,
                out,
            );
            run.decoy_walk(
                mission,
                Kinds {
                    ships: false,
                    fighters: true,
                    regiments: false,
                },
                rng,
                out,
            );
        }
    }
    if !run.detected {
        run.detect(mission, rng);
    }
    if run.detected {
        // Slot +0x1dc: 3, or 4 past phase 4, when members may resign.
        if record.can_resign {
            mission.end_code = if mission.phase > 4 { 4 } else { 3 };
        }
        run.expose_team(mission, rng, out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{CharacterKey, DatId, SpecialForceKey};
    use crate::missions::MissionKind;
    use crate::world::{
        Character, ControlKind, Fleet, MissionRecord, MissionTargetRules, MstbEntry, MstbTable,
        ShipInstance, SkillPair, SpecialForceUnit, System, TroopClassDef, TroopUnit,
    };

    /// A populated system held by `control`.
    fn system(world: &mut GameWorld, control: ControlKind) -> SystemKey {
        world.systems.insert(System {
            dat_id: DatId::new(0x9000_0001),
            name: "System".into(),
            sector: crate::ids::SectorKey::default(),
            x: 0,
            y: 0,
            exploration_status: crate::dat::ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control,
        })
    }

    fn imperial() -> ControlKind {
        ControlKind::Controlled(Faction::Empire)
    }

    /// The shipped MISSNSD Sabotage record `0x69000012` with detection
    /// phases and resign on; the target rules admit any calm system, so the
    /// validator leaves the run alone.
    fn sabotage_record(can_resign: bool) -> MissionRecord {
        MissionRecord {
            dat_id: DatId::new(0x6900_0012),
            timer_min_days: 1,
            timer_spread_days: 0,
            repeats: false,
            hidden: false,
            detection_phases: true,
            can_resign,
            rules: MissionTargetRules {
                own_side_target: true,
                other_side_target: true,
                opponent_target: true,
                calm_target: true,
                ..MissionTargetRules::default()
            },
            members: crate::world::MissionMemberRules {
                alliance: true,
                empire: true,
                special_force_mask: 0x402,
                character_mask: 0x1_0000,
            },
        }
    }

    /// A table whose every row is `percent`.
    fn table(world: &mut GameWorld, name: &str, percent: u32) {
        world.mission_tables.insert(
            name.to_string(),
            MstbTable::new(vec![MstbEntry {
                threshold: 0,
                value: percent,
            }]),
        );
    }

    fn world_with(control: ControlKind) -> (GameWorld, SystemKey) {
        let mut world = GameWorld::default();
        world.mission_records.push(sabotage_record(true));
        let here = system(&mut world, control);
        (world, here)
    }

    fn agent(world: &mut GameWorld, at: SystemKey, loyalty: u32) -> MissionMember {
        MissionMember::Character(world.characters.insert(Character {
            name: "Agent".into(),
            is_alliance: true,
            loyalty: SkillPair {
                base: loyalty,
                variance: 0,
            },
            current_system: Some(at),
            ..Default::default()
        }))
    }

    fn imperial_regiment(world: &mut GameWorld, at: SystemKey) -> TroopKey {
        regiment(world, at, false, 0)
    }

    /// A regiment whose class detection (slot `+0x1c4`) is `detection`.
    fn regiment(
        world: &mut GameWorld,
        at: SystemKey,
        is_alliance: bool,
        detection: u32,
    ) -> TroopKey {
        let class = DatId::new(0x1000_0000 | detection);
        world.troop_classes.insert(
            class,
            TroopClassDef {
                attack_strength: 0,
                defense_strength: 0,
                detection,
            },
        );
        let key = world.troops.insert(TroopUnit {
            class_dat_id: class,
            is_alliance,
            regiment_strength: 1,
        });
        world.systems[at].ground_units.push(key);
        key
    }

    /// A loyal Alliance agent of family `family` with the given espionage
    /// and combat.
    fn skilled(
        world: &mut GameWorld,
        at: SystemKey,
        family: u32,
        espionage: u32,
        combat: u32,
    ) -> MissionMember {
        let pair = |base| SkillPair { base, variance: 0 };
        MissionMember::Character(world.characters.insert(Character {
            dat_id: DatId::new(family << 24 | 1),
            name: "Agent".into(),
            is_alliance: true,
            loyalty: pair(100),
            espionage: pair(espionage),
            combat: pair(combat),
            current_system: Some(at),
            ..Default::default()
        }))
    }

    /// A table that yields 100 only for `x` in `low..=high`, else 0.
    fn band(world: &mut GameWorld, name: &str, low: i32, high: i32) {
        world.mission_tables.insert(
            name.to_string(),
            MstbTable::new(vec![
                MstbEntry {
                    threshold: i32::MIN,
                    value: 0,
                },
                MstbEntry {
                    threshold: low,
                    value: 100,
                },
                MstbEntry {
                    threshold: high + 1,
                    value: 0,
                },
            ]),
        );
    }

    fn special_force(world: &mut GameWorld, at: SystemKey, espionage: u32) -> MissionMember {
        let mut skills = [0; 8];
        skills[Skill::Espionage as usize] = espionage;
        let unit = world.special_forces.insert(SpecialForceUnit {
            class_dat_id: DatId::new(0x1400_0001),
            is_alliance: true,
            skills,
            on_mission: true,
        });
        world.systems[at].special_forces.push(unit);
        MissionMember::SpecialForce(unit)
    }

    fn imperial_fleet(world: &mut GameWorld, at: SystemKey, is_alliance: bool) -> FleetKey {
        world.fleets.insert(Fleet {
            location: at,
            capital_ships: vec![ShipInstance {
                class: crate::ids::CapitalShipKey::default(),
                hull_current: 1,
                shield_weapon_packed: 0,
                alive: true,
                name: None,
            }],
            fighters: vec![],
            characters: vec![],
            is_alliance,
            has_death_star: false,
        })
    }

    fn run_seeded(mission: &mut ActiveMission, world: &GameWorld, roll: f64) -> Vec<MissionEffect> {
        let mut out = Vec::new();
        let mut rng = MissionRng::seeded(roll, mission.id);
        run(
            mission,
            world,
            &UprisingState::default(),
            &[],
            &mut rng,
            &mut out,
        );
        out
    }

    fn sabotage(team: Vec<MissionMember>, at: SystemKey, phase: u8) -> ActiveMission {
        let mut mission = ActiveMission::new(
            0,
            MissionKind::Sabotage,
            MissionFaction::Alliance,
            team,
            at,
            0,
        );
        mission.phase = phase;
        mission.origin = Some(at);
        mission
    }

    fn run_once(mission: &mut ActiveMission, world: &GameWorld) -> Vec<MissionEffect> {
        run_seeded(mission, world, 0.25)
    }

    fn character(member: MissionMember) -> CharacterKey {
        member.character().expect("a character member")
    }

    #[test]
    fn an_opposing_regiment_detects_a_team_in_phase_two_and_the_evading_team_resigns() {
        // FUN_005896e0 detects on a FOILTB success; slot +0x1dc ends 3 at or
        // before phase 4 when members may resign; an evading member
        // (RLEVADTB success, slot +0x208) takes a resign request and the
        // injury roll, whose chance max(G2565, 100 - combat) is 100 here.
        let (mut world, here) = world_with(imperial());
        imperial_regiment(&mut world, here);
        table(&mut world, FOILTB, 100);
        table(&mut world, RLEVADTB, 100);
        let lead = agent(&mut world, here, 100);
        let mut mission = sabotage(vec![lead], here, 2);

        let out = run_once(&mut mission, &world);

        assert_eq!(mission.end_code, 3);
        assert_eq!(mission.resigning, vec![lead]);
        assert!(matches!(
            out.as_slice(),
            [MissionEffect::CharacterInjured { character: c, .. }] if *c == character(lead)
        ));
    }

    #[test]
    fn a_member_who_fails_to_evade_is_injured_and_captured_by_the_defender_s_side() {
        // Slot +0x20c: the captured character takes the injury roll, then
        // becomes the defender side's prisoner at the location.
        let (mut world, here) = world_with(imperial());
        imperial_regiment(&mut world, here);
        table(&mut world, FOILTB, 100);
        table(&mut world, RLEVADTB, 0);
        let lead = agent(&mut world, here, 100);
        let mut mission = sabotage(vec![lead], here, 2);

        let out = run_once(&mut mission, &world);

        assert_eq!(mission.end_code, 3);
        assert!(matches!(
            out.as_slice(),
            [
                MissionEffect::CharacterInjured { .. },
                MissionEffect::CharacterCaptured {
                    captured_by: MissionFaction::Empire,
                    at_system,
                    ..
                },
            ] if *at_system == here
        ));
    }

    #[test]
    fn a_captured_special_force_is_destroyed() {
        // FUN_00503eb0: the special-force capture slot destroys the unit.
        let (mut world, here) = world_with(imperial());
        imperial_regiment(&mut world, here);
        table(&mut world, FOILTB, 100);
        table(&mut world, RLEVADTB, 0);
        let unit: SpecialForceKey = world.special_forces.insert(SpecialForceUnit {
            class_dat_id: DatId::new(0x1400_0001),
            is_alliance: true,
            skills: [0; 8],
            on_mission: true,
        });
        world.systems[here].special_forces.push(unit);
        let mut mission = sabotage(vec![MissionMember::SpecialForce(unit)], here, 2);

        let out = run_once(&mut mission, &world);

        assert!(matches!(
            out.as_slice(),
            [MissionEffect::SpecialForceDestroyed { unit: u }] if *u == unit
        ));
    }

    #[test]
    fn a_system_the_mission_s_own_side_holds_has_no_defenders_in_phase_two() {
        // FUN_00589a40 mode 2 walks the system only when the opponent holds it.
        let (mut world, here) = world_with(ControlKind::Controlled(Faction::Alliance));
        imperial_regiment(&mut world, here);
        table(&mut world, FOILTB, 100);
        let lead = agent(&mut world, here, 100);
        let mut mission = sabotage(vec![lead], here, 2);

        let out = run_once(&mut mission, &world);

        assert_eq!(mission.end_code, 0);
        assert!(out.is_empty());
    }

    #[test]
    fn a_zero_or_absent_foiltb_detects_nothing() {
        // FUN_005896e0 detects only on a FOILTB success. port: an absent
        // table detects nothing.
        for foil in [Some(0), None] {
            let (mut world, here) = world_with(imperial());
            imperial_regiment(&mut world, here);
            if let Some(percent) = foil {
                table(&mut world, FOILTB, percent);
            }
            let lead = agent(&mut world, here, 100);
            let mut mission = sabotage(vec![lead], here, 2);

            assert!(run_once(&mut mission, &world).is_empty());
            assert_eq!(mission.end_code, 0, "FOILTB {foil:?}");
        }
    }

    #[test]
    fn a_decoy_that_draws_off_the_only_regiment_leaves_the_team_undetected() {
        // FUN_0058a020: a TDECOYTB success decoys the defender, which the
        // detection walk then skips.
        let (mut world, here) = world_with(imperial());
        imperial_regiment(&mut world, here);
        table(&mut world, FOILTB, 100);
        table(&mut world, TDECOYTB, 100);
        let lead = agent(&mut world, here, 100);
        let decoy = agent(&mut world, here, 100);
        let mut mission = sabotage(vec![lead], here, 2);
        mission.decoys = vec![decoy];

        let out = run_once(&mut mission, &world);

        assert_eq!(mission.end_code, 0);
        assert!(out.is_empty());
        assert!(mission.resigning.is_empty());
    }

    #[test]
    fn a_decoy_that_fails_is_exposed_and_resigns_while_the_team_runs_on() {
        // FUN_00589620: a failed decoy roll exposes the decoy (FUN_005888f0).
        let (mut world, here) = world_with(imperial());
        imperial_regiment(&mut world, here);
        table(&mut world, FOILTB, 0);
        table(&mut world, TDECOYTB, 0);
        table(&mut world, RLEVADTB, 100);
        let lead = agent(&mut world, here, 100);
        let decoy = agent(&mut world, here, 100);
        let mut mission = sabotage(vec![lead], here, 2);
        mission.decoys = vec![decoy];

        run_once(&mut mission, &world);

        assert_eq!(mission.end_code, 0);
        assert_eq!(mission.resigning, vec![decoy]);
    }

    #[test]
    fn a_disloyal_member_betrays_the_mission_in_phase_nine() {
        // FUN_00589f10 / FUN_00588da0: draw(0..99) < 100 - loyalty betrays;
        // past phase 4 a detected mission ends 4. No table is present, so
        // only the betrayal can detect.
        for (loyalty, end_code) in [(0, 4), (100, 0)] {
            let (mut world, here) = world_with(imperial());
            let lead = agent(&mut world, here, loyalty);
            let mut mission = sabotage(vec![lead], here, PHASE_BETRAYAL);

            run_once(&mut mission, &world);

            assert_eq!(mission.end_code, end_code, "loyalty {loyalty}");
        }
    }

    #[test]
    fn an_opposing_fleet_at_the_origin_detects_a_team_bound_elsewhere_in_phase_three() {
        // Mode 1 counts members at a system only when the origin is not the
        // target, and walks the fleets when only the opponent has one there.
        let (mut world, here) = world_with(imperial());
        let there = system(&mut world, ControlKind::Uncontrolled);
        table(&mut world, FOILTB, 100);
        imperial_fleet(&mut world, here, false);
        for (target, end_code) in [(there, 3), (here, 0)] {
            let lead = agent(&mut world, here, 100);
            let mut mission = sabotage(vec![lead], target, 3);
            mission.origin = Some(here);

            run_once(&mut mission, &world);

            assert_eq!(mission.end_code, end_code, "target {target:?}");
        }
        // An own fleet there as well keeps mode 1 off the fleets.
        imperial_fleet(&mut world, here, true);
        let lead = agent(&mut world, here, 100);
        let mut mission = sabotage(vec![lead], there, 3);
        mission.origin = Some(here);
        run_once(&mut mission, &world);
        assert_eq!(mission.end_code, 0);
    }

    #[test]
    fn a_detected_team_that_may_not_resign_runs_on() {
        // Slot +0x1dc sets an end code only when the record lets members
        // resign (column 9), and FUN_00534640 requests nothing without it.
        let (mut world, here) = world_with(imperial());
        world.mission_records = vec![sabotage_record(false)];
        imperial_regiment(&mut world, here);
        table(&mut world, FOILTB, 100);
        table(&mut world, RLEVADTB, 100);
        let lead = agent(&mut world, here, 100);
        let mut mission = sabotage(vec![lead], here, 2);

        run_once(&mut mission, &world);

        assert_eq!(mission.end_code, 0);
        assert!(mission.resigning.is_empty());
    }

    #[test]
    fn a_team_whose_every_member_resigns_ends_with_code_five() {
        // FUN_00522480.c:42-52: end 5 unless a team member stands without a
        // resign request.
        let (mut world, here) = world_with(imperial());
        let lead = agent(&mut world, here, 100);
        let mut mission = sabotage(vec![lead], here, 8);
        assert_eq!(
            running_end_code(&mission, &world, &UprisingState::default(), &[]),
            0
        );
        mission.resigning = vec![lead];
        assert_eq!(
            running_end_code(&mission, &world, &UprisingState::default(), &[]),
            5
        );
    }

    #[test]
    fn detection_draws_stay_in_range_and_repeat_for_the_same_seed() {
        // Our helper: draw(n) covers 0..=n (FUN_0053e290), 0 for n <= 0.
        let mut rng = MissionRng::seeded(0.5, 7);
        let draws: Vec<i32> = (0..200).map(|_| rng.draw(3)).collect();
        assert!(draws.iter().all(|d| (0..=3).contains(d)));
        assert!((0..=3).all(|n| draws.contains(&n)));
        let mut again = MissionRng::seeded(0.5, 7);
        assert_eq!(draws, (0..200).map(|_| again.draw(3)).collect::<Vec<_>>());
        let negative: Vec<i32> = (0..200).map(|_| rng.draw(-3)).collect();
        assert!((-3..=0).all(|n| negative.contains(&n)));
        assert!(!rng.chance(0));
        assert!(rng.chance(100));
    }

    #[test]
    fn detection_draws_are_splitmix64_seeded_by_the_roll_and_the_mission() {
        // Our helper: the published SplitMix64 first output from state 0,
        // and a mission id that changes the stream.
        assert_eq!(MissionRng::seeded(0.0, 0).next_u64(), 0xe220_a839_7b1d_cdaf);
        assert_eq!(MissionRng::seeded(0.5, 0).state, 1 << 52);
        let first = |roll, id| MissionRng::seeded(roll, id).next_u64();
        assert_ne!(first(0.0, 1), first(0.0, 0));
        assert_ne!(first(0.5, 1), first(0.0, 1));
    }

    #[test]
    fn the_defender_walk_takes_regiments_then_each_fleet_s_live_ships_then_fighters() {
        // FUN_00587640: the system's regiments, then per fleet at the
        // location its ships and fighters (port: fighters after all ships).
        let (mut world, here) = world_with(imperial());
        let there = system(&mut world, imperial());
        let troop = imperial_regiment(&mut world, here);
        let fleet = imperial_fleet(&mut world, here, false);
        world.fleets[fleet].capital_ships.push(ShipInstance {
            class: crate::ids::CapitalShipKey::default(),
            hull_current: 0,
            shield_weapon_packed: 0,
            alive: false,
            name: None,
        });
        world.fleets[fleet]
            .fighters
            .push(crate::world::FighterEntry {
                class: crate::ids::FighterKey::default(),
                count: 2,
            });
        imperial_fleet(&mut world, there, false);
        let ids = |flags, kinds| -> Vec<DefenderId> {
            walk(&world, here, flags, kinds)
                .iter()
                .map(|d| d.id)
                .collect()
        };
        let both = Flags {
            system: true,
            fleets: true,
            ..Flags::default()
        };
        let only = |ships, fighters, regiments| Kinds {
            ships,
            fighters,
            regiments,
        };

        assert_eq!(
            ids(both, ALL),
            vec![
                DefenderId::Regiment(troop),
                DefenderId::Ship(fleet, 0),
                DefenderId::Fighter(fleet, 0, 0),
                DefenderId::Fighter(fleet, 0, 1),
            ]
        );
        assert_eq!(
            ids(both, only(true, false, false)),
            vec![DefenderId::Ship(fleet, 0)]
        );
        assert_eq!(
            ids(both, only(false, false, true)),
            vec![DefenderId::Regiment(troop)]
        );
        let fleets = Flags {
            fleets: true,
            ..Flags::default()
        };
        assert_eq!(ids(fleets, only(false, false, true)), vec![]);
        let system_only = Flags {
            system: true,
            ..Flags::default()
        };
        assert_eq!(ids(system_only, only(true, true, false)), vec![]);
    }

    #[test]
    fn a_detection_roll_reads_the_team_s_average_espionage_less_detection_forces_and_offset() {
        // FUN_005896e0: x = avg team espionage - detection - special forces
        // - G3584 (shipped -1). Espionage 30, 10 and 20 average 20; a
        // resigning member's 90 is not counted; detection 5 and one special
        // force give x = 20 - 5 - 1 + 1 = 15.
        let (mut world, here) = world_with(imperial());
        regiment(&mut world, here, false, 5);
        band(&mut world, FOILTB, 15, 15);
        let team = vec![
            skilled(&mut world, here, 0x30, 30, 0),
            skilled(&mut world, here, 0x30, 10, 0),
            special_force(&mut world, here, 20),
            skilled(&mut world, here, 0x30, 90, 0),
        ];
        let mut mission = sabotage(team.clone(), here, 2);
        mission.resigning = vec![team[3]];

        run_once(&mut mission, &world);

        assert_eq!(mission.end_code, 3);
    }

    #[test]
    fn a_decoy_roll_reads_the_decoy_s_espionage_less_the_defender_s_detection() {
        // FUN_00589620: x = decoy espionage - defender detection (port: no
        // officer term). Espionage 10 against detection 4 gives x = 6.
        let (mut world, here) = world_with(imperial());
        regiment(&mut world, here, false, 4);
        table(&mut world, FOILTB, 100);
        band(&mut world, TDECOYTB, 6, 6);
        let lead = agent(&mut world, here, 100);
        let decoy = skilled(&mut world, here, 0x30, 10, 0);
        let mut mission = sabotage(vec![lead], here, 2);
        mission.decoys = vec![decoy];

        run_once(&mut mission, &world);

        assert_eq!(mission.end_code, 0);
    }

    #[test]
    fn an_evading_member_s_injury_draws_after_the_detection_and_evasion_rolls() {
        // FUN_0053e990 with combat 30: chance max(G2565, 70) = 70, injury
        // rand(70) + rand(G2567 = 29) + G2566 = 1. Before it the run draws
        // FOILTB's chance, the defender pick draw(0), and RLEVADTB's chance.
        let (mut world, here) = world_with(imperial());
        imperial_regiment(&mut world, here);
        table(&mut world, FOILTB, 100);
        table(&mut world, RLEVADTB, 100);
        let lead = skilled(&mut world, here, 0x30, 0, 30);
        let mut mission = sabotage(vec![lead], here, 2);

        let out = run_seeded(&mut mission, &world, 0.0);

        let mut expected = MissionRng::seeded(0.0, 0);
        expected.draw(99);
        expected.draw(0);
        expected.draw(99);
        assert!(expected.chance(70), "the seed should reach the injury");
        let injury = expected.draw(70) + expected.draw(29) + 1;
        assert_eq!(
            out,
            vec![MissionEffect::CharacterInjured {
                character: character(lead),
                injury
            }]
        );
    }

    #[test]
    fn an_exposed_team_faces_defenders_in_character_family_order_then_special_forces() {
        // FUN_0058a1c0: team characters of families 0x30..0x37, then
        // 0x38..0x3b, then team special forces, whatever the list order.
        let (mut world, here) = world_with(imperial());
        imperial_regiment(&mut world, here);
        table(&mut world, FOILTB, 100);
        table(&mut world, RLEVADTB, 0);
        let unit = special_force(&mut world, here, 0);
        let late = skilled(&mut world, here, 0x38, 0, 0);
        let early = skilled(&mut world, here, 0x37, 0, 0);
        let mut mission = sabotage(vec![unit, late, early], here, 2);

        let out = run_once(&mut mission, &world);

        let order: Vec<MissionMember> = out
            .iter()
            .filter_map(|effect| match effect {
                MissionEffect::CharacterCaptured { character, .. } => {
                    Some(MissionMember::Character(*character))
                }
                MissionEffect::SpecialForceDestroyed { unit } => {
                    Some(MissionMember::SpecialForce(*unit))
                }
                _ => None,
            })
            .collect();
        assert_eq!(order, vec![early, late, unit]);
    }

    #[test]
    fn an_alliance_defender_captures_for_the_alliance() {
        // Slot +0x20c: the captor is the defender's side.
        let (mut world, here) = world_with(ControlKind::Controlled(Faction::Alliance));
        regiment(&mut world, here, true, 0);
        table(&mut world, FOILTB, 100);
        table(&mut world, RLEVADTB, 0);
        let imperial_agent = world.characters.insert(Character {
            is_empire: true,
            loyalty: SkillPair {
                base: 100,
                variance: 0,
            },
            current_system: Some(here),
            ..Default::default()
        });
        let mut mission = sabotage(vec![MissionMember::Character(imperial_agent)], here, 2);
        mission.faction = MissionFaction::Empire;

        let out = run_once(&mut mission, &world);

        assert!(out.iter().any(|effect| matches!(
            effect,
            MissionEffect::CharacterCaptured {
                captured_by: MissionFaction::Alliance,
                ..
            }
        )));
    }

    #[test]
    fn a_destroyed_decoy_is_not_drawn_again() {
        // FUN_00503eb0 destroys a captured special force; without a resign
        // request (column 9 off) it stays in the decoy count, and the next
        // defender's pick finds no decoy left.
        let (mut world, here) = world_with(imperial());
        world.mission_records = vec![sabotage_record(false)];
        imperial_regiment(&mut world, here);
        imperial_regiment(&mut world, here);
        table(&mut world, FOILTB, 0);
        table(&mut world, TDECOYTB, 0);
        table(&mut world, RLEVADTB, 0);
        let lead = agent(&mut world, here, 100);
        let mut mission = sabotage(vec![lead], here, 2);
        mission.decoys = vec![special_force(&mut world, here, 0)];

        let out = run_once(&mut mission, &world);

        let destroyed = out
            .iter()
            .filter(|effect| matches!(effect, MissionEffect::SpecialForceDestroyed { .. }))
            .count();
        assert_eq!(destroyed, 1);
    }

    #[test]
    fn a_decoy_that_evades_without_resigning_meets_the_next_defender_too() {
        // FUN_005888f0.c:61-84: without a resign request (column 9 off) an
        // evading decoy stays in the decoy count, so the second regiment's
        // pick lands on it again; each evasion rolls the injury (combat 0,
        // chance 100).
        let (mut world, here) = world_with(imperial());
        world.mission_records = vec![sabotage_record(false)];
        imperial_regiment(&mut world, here);
        imperial_regiment(&mut world, here);
        table(&mut world, FOILTB, 0);
        table(&mut world, TDECOYTB, 0);
        table(&mut world, RLEVADTB, 100);
        let lead = agent(&mut world, here, 100);
        let mut mission = sabotage(vec![lead], here, 2);
        mission.decoys = vec![agent(&mut world, here, 100)];

        let out = run_once(&mut mission, &world);

        assert_eq!(out.len(), 2);
        assert!(out
            .iter()
            .all(|effect| matches!(effect, MissionEffect::CharacterInjured { .. })));
    }

    #[test]
    fn a_captured_decoy_is_not_drawn_again() {
        // Slot +0x20c takes the decoy prisoner; port: the run marks it gone,
        // so without a resign request the next defender finds no decoy.
        let (mut world, here) = world_with(imperial());
        world.mission_records = vec![sabotage_record(false)];
        imperial_regiment(&mut world, here);
        imperial_regiment(&mut world, here);
        table(&mut world, FOILTB, 0);
        table(&mut world, TDECOYTB, 0);
        table(&mut world, RLEVADTB, 0);
        let lead = agent(&mut world, here, 100);
        let mut mission = sabotage(vec![lead], here, 2);
        mission.decoys = vec![agent(&mut world, here, 100)];

        let out = run_once(&mut mission, &world);

        let captured = out
            .iter()
            .filter(|effect| matches!(effect, MissionEffect::CharacterCaptured { .. }))
            .count();
        assert_eq!(captured, 1);
    }

    #[test]
    fn each_defender_meets_a_decoy_that_has_not_resigned() {
        // FUN_005888f0.c:61-84: a resigning decoy leaves the decoy count, so
        // the next defender's pick draw(0..decoys-1) lands on the other one.
        for id in 0..16 {
            let (mut world, here) = world_with(imperial());
            imperial_regiment(&mut world, here);
            imperial_regiment(&mut world, here);
            table(&mut world, FOILTB, 0);
            table(&mut world, TDECOYTB, 0);
            table(&mut world, RLEVADTB, 100);
            let lead = agent(&mut world, here, 100);
            let decoys = vec![agent(&mut world, here, 100), agent(&mut world, here, 100)];
            let mut mission = sabotage(vec![lead], here, 2);
            mission.id = id;
            mission.decoys = decoys.clone();

            run_once(&mut mission, &world);

            assert_eq!(mission.resigning.len(), 2, "mission {id}");
            assert!(decoys.iter().all(|d| mission.resigning.contains(d)));
        }
    }

    #[test]
    fn a_decoyed_defender_leaves_the_count_the_exposed_team_draws_from() {
        // FUN_0058a020: a decoyed defender leaves +0x34, so each exposed team
        // member's pick draw(0..defenders-1) lands on the one left. The decoy
        // draws off detection 0 (x = 10) and fails against detection 50.
        for id in 0..16 {
            let (mut world, here) = world_with(imperial());
            regiment(&mut world, here, false, 0);
            regiment(&mut world, here, false, 50);
            table(&mut world, FOILTB, 100);
            band(&mut world, TDECOYTB, 0, 100);
            table(&mut world, RLEVADTB, 100);
            let lead = agent(&mut world, here, 100);
            let mut mission = sabotage(vec![lead], here, 2);
            mission.id = id;
            mission.decoys = vec![skilled(&mut world, here, 0x30, 10, 0)];

            run_once(&mut mission, &world);

            assert!(mission.resigning.contains(&lead), "mission {id}");
        }
    }

    #[test]
    fn past_phase_four_the_run_looks_at_the_target_not_the_origin() {
        // FUN_00520ad0: the target location +0x78 past phase 4. Phase 7 is
        // mode 1; the opponent's fleet waits at the target only.
        let (mut world, here) = world_with(imperial());
        let there = system(&mut world, ControlKind::Uncontrolled);
        table(&mut world, FOILTB, 100);
        imperial_fleet(&mut world, here, false);
        let lead = agent(&mut world, here, 100);
        let mut mission = sabotage(vec![lead], here, 7);
        mission.origin = Some(there);

        run_once(&mut mission, &world);

        assert_eq!(mission.end_code, 4);
    }

    #[test]
    fn phase_two_counts_no_member_aboard_a_fleet() {
        // FUN_00589a40 mode 2 sets only the at-system member filter (+0x18).
        let (mut world, here) = world_with(imperial());
        imperial_regiment(&mut world, here);
        table(&mut world, FOILTB, 100);
        let fleet = imperial_fleet(&mut world, here, true);
        let lead = agent(&mut world, here, 100);
        world.characters[character(lead)].current_fleet = Some(fleet);
        let mut mission = sabotage(vec![lead], here, 2);

        run_once(&mut mission, &world);

        assert_eq!(mission.end_code, 0);
    }

    #[test]
    fn a_phase_nine_system_mission_is_detected_by_the_holder_s_regiments() {
        // FUN_00589a40 mode 4: a system or object target walks the system
        // when the opponent holds it.
        let (mut world, here) = world_with(imperial());
        imperial_regiment(&mut world, here);
        table(&mut world, FOILTB, 100);
        let lead = agent(&mut world, here, 100);
        let mut mission = sabotage(vec![lead], here, PHASE_BETRAYAL);

        run_once(&mut mission, &world);

        assert_eq!(mission.end_code, 4);
    }
}
