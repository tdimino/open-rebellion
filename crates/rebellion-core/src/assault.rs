//! Planetary Assault: order `0x234` (`ghidra/notes/ground-assault.md`).
//!
//! The order flags each team fleet (`FUN_004feb60`, fleet `+0x58` bit 7)
//! and runs one system command per system of the team (`FUN_00563390` ->
//! `FUN_0058c1e0` -> `FUN_0058c120`) in the step that issued it: the count
//! (`FUN_0058c2d0`), the defence facilities' fire (`FUN_0058c3c0`), one
//! engagement per attacking regiment (`FUN_0058c500`), the collateral hits
//! (`FUN_0058c580`), the landing (`FUN_0058c810`) and the cleanup
//! (`FUN_0058c8b0`). There are no rounds: a pass that leaves both sides
//! standing changes nothing more, and a second order runs it again.
//!
//! The holder the run leaves (`FUN_0050b610`, run by event `0x301` for each
//! killed regiment and `0x320` for each landed one) and the blockade loss
//! counters live with the integrator, which holds the blockade state.
//!
//! port: the fleet bit and the system's `+0x88` bit `0x80` are set by the
//! order and cleared by the run's own cleanup (`FUN_0058c8b0`), so they
//! live only in the run's fleet list, not on the world.

use crate::blockade::{BlockadeState, UnitLoss, REASON_GROUND_COMBAT};
use crate::dat::Faction;
use crate::ids::{DefenseFacilityKey, FleetKey, SystemKey, TroopKey};
use crate::mission_detection::MissionRng;
use crate::missions::MissionTarget;
use crate::movement::MovementState;
use crate::officer_rank::CommandKind;
use crate::troop_transport::TroopTransportState;
use crate::uprising::Draws;
use crate::world::{FacilityRef, GameWorld};

/// GNPRTB `0x200` (5, `DAT_006bb788`, `FUN_0055ea00.c:9`): a defence
/// facility's attack over it is its percent chance to fire
/// (`FUN_0055ead0.c:9`).
const GNPRTB_FIRE_DIVISOR: u16 = 0x200;
/// GNPRTB `0x201` (30, `DAT_006bb780`, `FUN_0055ea00.c:10`): each side's
/// General's leadership over it joins the roll (`FUN_0055eb00.c:12-13`).
const GNPRTB_LEADERSHIP_DIVISOR: u16 = 0x201;
/// GNPRTB `0x202` (10, `DAT_006bb78c`, `FUN_0055ea00.c:17`): the
/// engagement's draw `rand[0, n]` (`FUN_0055eb00.c:14`).
const GNPRTB_ENGAGEMENT_DRAW: u16 = 0x202;
/// GNPRTB `0x203` (4, `DAT_006bb778`, `FUN_0055ea00.c:24`): at or below it
/// the attacking regiment dies (`FUN_0055eb00.c:16`).
const GNPRTB_ATTACKER_DIES: u16 = 0x203;
/// GNPRTB `0x204` (6, `DAT_006bb784`, `FUN_0055ea00.c:31`): at or above it
/// the defending regiment dies (`FUN_0055eb00.c:17`).
const GNPRTB_DEFENDER_DIES: u16 = 0x204;
/// GNPRTB `0x205` (8, `DAT_006bb77c`, `FUN_0055ea00.c:38`): each
/// engagement's percent chance of a collateral hit (`FUN_0055eb80.c:11`).
const GNPRTB_COLLATERAL_CHANCE: u16 = 0x205;
/// GNPRTB `0x1e03` (2, `DAT_006bb474`, `FUN_00558bb0.c:24`, read by
/// `FUN_0055a0b0`): the shield generators at which the system refuses the
/// order (`FUN_00507d00.c:42-53`).
const GNPRTB_SHIELD_LIMIT: u16 = 0x1e03;
/// The shield generators' family (`[0x24,0x25)`, `FUN_005278c0`).
const SHIELD_FAMILY: u8 = 0x24;

/// Why the order cannot be issued (the enable slot `FUN_005814d0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssaultRefusal {
    /// `1`/`0x16`: the order has no fleet to command (`FUN_0051fe20`).
    NoTeam,
    /// `1`/`1`: a fleet command's `+0x18` refuses another side's fleet or
    /// one en route (`FUN_004ffa70` runs `FUN_004f9860` first).
    FleetRefused,
    /// `8`/`1`: a fleet command's `+0x18` finds no ship in its fleet
    /// (`FUN_004ffa70.c:15-60`, `crate::bombardment::ships_refusal`).
    NoShip,
    /// `1`/`0x10`: no ship of the fleet is completed and not en route, and
    /// the last one walked is not completed (`FUN_004ffa70.c:15-60`).
    LastShipUnfinished,
    /// `1`/`0x11`: no ship of the fleet is completed and not en route, and
    /// the last one walked is en route (`FUN_004ffa70.c:15-60`).
    LastShipEnRoute,
    /// `1`/`0x14`: the team's systems have different holders
    /// (`FUN_00553770`).
    MixedHolders,
    /// `8`/`7`: a fleet's blockade bit (`+0x58` bit `0x20`) is clear
    /// (`FUN_004fddd0`), or no team fleet has it (`FUN_00581ae0`).
    NotBlockaded,
    /// `8`/`5`: no blockading fleet holds a regiment (`FUN_00581ae0`).
    NoRegiments,
    /// `1`/`0x22`: the system is destroyed (`FUN_00507d00`).
    Destroyed,
    /// `0x90`/`5`: GNPRTB `0x1e03` or more shield generators stand at the
    /// system (`FUN_00507d00`).
    Shields,
}

impl AssaultRefusal {
    /// The two-word status the order carries: `FUN_0051fe20` (`1`/`0x16`),
    /// `FUN_004f9860` through `FUN_004ffa70` (`1`/`1`),
    /// `FUN_004ffa70.c:15-60` (`8`/`1`, `1`/`0x10`, `1`/`0x11`),
    /// `FUN_00553770` (`1`/`0x14`), `FUN_004fddd0` and `FUN_00581ae0`
    /// (`8`/`7`, `8`/`5`), `FUN_00507d00` (`1`/`0x22`, `0x90`/`5`).
    #[must_use]
    pub const fn status(self) -> (u32, u32) {
        match self {
            Self::NoTeam => (1, 0x16),
            Self::FleetRefused => (1, 1),
            Self::NoShip => (8, 1),
            Self::LastShipUnfinished => (1, 0x10),
            Self::LastShipEnRoute => (1, 0x11),
            Self::MixedHolders => (1, 0x14),
            Self::NotBlockaded => (8, 7),
            Self::NoRegiments => (8, 5),
            Self::Destroyed => (1, 0x22),
            Self::Shields => (0x90, 5),
        }
    }
}

impl From<crate::bombardment::ShipsRefusal> for AssaultRefusal {
    fn from(refusal: crate::bombardment::ShipsRefusal) -> Self {
        match refusal {
            crate::bombardment::ShipsRefusal::NoShip => Self::NoShip,
            crate::bombardment::ShipsRefusal::Unfinished => Self::LastShipUnfinished,
            crate::bombardment::ShipsRefusal::EnRoute => Self::LastShipEnRoute,
        }
    }
}

const fn side_of(is_alliance: bool) -> Faction {
    if is_alliance {
        Faction::Alliance
    } else {
        Faction::Empire
    }
}

/// The order's enable test for a team of fleets (`FUN_005814d0`), in the
/// original's order: the order base (`FUN_0051fe20`: a team, each fleet
/// command's `+0x18` through `FUN_004ffa70`, one holder by
/// `FUN_00553770`); each fleet's blockade bit (fleet slot `+0x1bc`,
/// `FUN_004fddd0.c:21-30`); the team gate (`FUN_00581ae0.c:43-104`): some
/// fleet with the bit holds a regiment aboard a usable ship (`.c:63-80`,
/// both walks mode 1); then each system command's
/// `+0x1bc` (`FUN_00507d00.c:23-56`): not destroyed, and fewer than GNPRTB
/// `0x1e03` active shield generators of either side.
///
/// Fleet `+0x58` bit `0x20` is the copy of its system's blockade bit
/// (`FUN_0050c0b0` -> `FUN_004fe3f0`), so an unpopulated neutral system,
/// never blockaded, is never assaulted; the system command's own test of
/// the bit (`0x90`/6) reads the bit every fleet copied, and passes with
/// them.
///
/// # Errors
/// The first refusal, in the original's order.
pub fn order_enabled(
    world: &GameWorld,
    movement: &MovementState,
    transport: &TroopTransportState,
    blockade: &BlockadeState,
    fleets: &[FleetKey],
    side: Faction,
) -> Result<(), AssaultRefusal> {
    if fleets.is_empty() {
        return Err(AssaultRefusal::NoTeam);
    }
    // Each fleet command's +0x18 (FUN_004fdf70 -> FUN_004ffa70).
    for &fleet in fleets {
        let Some(value) = world
            .fleets
            .get(fleet)
            .filter(|value| side_of(value.is_alliance) == side && !movement.is_in_transit(fleet))
        else {
            return Err(AssaultRefusal::FleetRefused);
        };
        if let Some(refusal) = crate::bombardment::ships_refusal(value) {
            return Err(refusal.into());
        }
    }
    let systems: Vec<SystemKey> = fleets
        .iter()
        .map(|&fleet| world.fleets[fleet].location)
        .collect();
    let holder = crate::bombardment::holder(world, systems[0]);
    if systems
        .iter()
        .any(|&system| crate::bombardment::holder(world, system) != holder)
    {
        return Err(AssaultRefusal::MixedHolders);
    }
    if !systems.iter().all(|&system| blockade.is_blockaded(system)) {
        return Err(AssaultRefusal::NotBlockaded);
    }
    // FUN_00581ae0.c:63-80: the fleet's usable ships (FUN_00502db0, mode
    // 1) and the regiments aboard each (FUN_00504c40, mode 1), the walk
    // the run's attackers take.
    if attackers(world, transport, fleets).is_empty() {
        return Err(AssaultRefusal::NoRegiments);
    }
    let limit = world
        .gnprtb
        .value(GNPRTB_SHIELD_LIMIT, world.difficulty_index);
    for &system in &systems {
        let Some(sys) = world.systems.get(system) else {
            continue;
        };
        if sys.is_destroyed {
            return Err(AssaultRefusal::Destroyed);
        }
        let shields = sys
            .defense_facilities
            .iter()
            .filter_map(|&key| world.defense_facilities.get(key))
            .filter(|facility| facility.class_dat_id.family() == SHIELD_FAMILY)
            .count();
        if i32::try_from(shields).unwrap_or(i32::MAX) >= limit {
            return Err(AssaultRefusal::Shields);
        }
    }
    Ok(())
}

/// What killed an attacking regiment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cause {
    /// A defence facility's shot (`FUN_0058c3c0`).
    DefenceFire,
    /// Its engagement (`FUN_0058bec0`).
    Engagement,
}

/// One defence facility's turn in the fire (`FUN_0058c3c0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefenceShot {
    pub facility: DefenseFacilityKey,
    /// Its `attack / 5` percent roll passed (`FUN_0055ead0`).
    pub fired: bool,
    /// The attacking regiment the shot killed; `None` on a miss.
    pub killed: Option<TroopKey>,
}

/// How an engagement ended (`FUN_0055eb00`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    AttackerDied,
    DefenderDied,
    Stood,
}

/// One attacking regiment's engagement (`FUN_0058bec0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Engagement {
    pub attacker: TroopKey,
    pub defender: TroopKey,
    /// The roll with both sides' terms.
    pub roll: i32,
    pub outcome: Outcome,
}

/// What a run did, for the control pass, the messages and telemetry.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AssaultReport {
    pub system: Option<SystemKey>,
    /// The assaulting side: the first flagged fleet's (`FUN_0058c1e0`).
    pub assaulter: Option<Faction>,
    /// The holder when the run began (manager `+0x18`).
    pub holder_before: Option<Faction>,
    /// The holder after the run's control pass; the integrator fills it.
    pub holder_after: Option<Faction>,
    /// The attacking and defending regiments counted at the start
    /// (`+0x1c`, `+0x20`).
    pub attackers: u32,
    pub defenders: u32,
    pub shots: Vec<DefenceShot>,
    pub engagements: Vec<Engagement>,
    pub attackers_lost: Vec<(TroopKey, Cause)>,
    pub defenders_lost: Vec<TroopKey>,
    /// Facilities the collateral hits destroyed, either side's.
    pub facilities_lost: Vec<FacilityRef>,
    /// Energy and raw material the collateral hits took.
    pub energy: u32,
    pub raw_material: u32,
    pub landed: Vec<TroopKey>,
    /// Each dead regiment with its owner, value and reason `0xe`, for the
    /// blockade loss counters (`FUN_00508c60`).
    pub losses: Vec<UnitLoss>,
}

/// `FUN_0058ba10`: the attacking fleets at the system, each one's usable
/// capital ships (`FUN_00502e30`, mode 1: alive, completed and not on a
/// leg of its own, `ShipInstance::usable`), and the regiments aboard each
/// ship (`FUN_00504cc0`, mode 1), in that order, with the regiment's
/// fleet.
fn attackers(
    world: &GameWorld,
    transport: &TroopTransportState,
    fleets: &[FleetKey],
) -> Vec<(TroopKey, FleetKey)> {
    let mut list = Vec::new();
    for &fleet in fleets {
        let Some(value) = world.fleets.get(fleet) else {
            continue;
        };
        let cargo = transport.cargo(fleet);
        for ship in value
            .capital_ships
            .iter()
            .filter(|ship| ship.usable() && ship.tag != 0)
        {
            for &troop in cargo
                .iter()
                .filter(|&&troop| transport.carrier(troop) == ship.tag)
            {
                list.push((troop, fleet));
            }
        }
    }
    list
}

/// `FUN_0058bbd0` -> `FUN_00504cc0(system, 1, holder)`: the holder's
/// regiments standing at the system. Special forces, characters and
/// facilities are no defenders.
fn defenders(world: &GameWorld, system: SystemKey, holder: Faction) -> Vec<TroopKey> {
    world.systems.get(system).map_or_else(Vec::new, |sys| {
        sys.ground_units
            .iter()
            .copied()
            .filter(|&key| {
                world
                    .troops
                    .get(key)
                    .is_some_and(|unit| side_of(unit.is_alliance) == holder)
            })
            .collect()
    })
}

/// `FUN_00539d70(system, 1)` less the HQ (`FUN_00526cf0`,
/// `FUN_0058c580.c:36-51`): the system's facilities of both sides. port:
/// the HQ is the system's flag, not one of its facilities, and the port
/// walks its defence, manufacturing and production lists in turn.
fn collateral_facilities(world: &GameWorld, system: SystemKey) -> Vec<FacilityRef> {
    world.systems.get(system).map_or_else(Vec::new, |sys| {
        sys.defense_facilities
            .iter()
            .map(|&key| FacilityRef::Defense(key))
            .chain(
                sys.manufacturing_facilities
                    .iter()
                    .map(|&key| FacilityRef::Manufacturing(key)),
            )
            .chain(
                sys.production_facilities
                    .iter()
                    .map(|&key| FacilityRef::Production(key)),
            )
            .collect()
    })
}

/// A regiment class's TROOPSD attack (class `+0x64`, `FUN_00520b90`) and
/// defence (`+0x68`, `FUN_00520ba0`).
fn troop_strengths(world: &GameWorld, troop: TroopKey) -> (i32, i32) {
    world
        .troops
        .get(troop)
        .and_then(|unit| world.troop_classes.get(&unit.class_dat_id))
        .map_or((0, 0), |class| {
            (
                class.attack_strength.cast_signed(),
                class.defense_strength.cast_signed(),
            )
        })
}

fn to_i32(count: usize) -> i32 {
    i32::try_from(count).unwrap_or(i32::MAX)
}

/// The `0x234` system run at `system` for the team's `fleets` there
/// (`FUN_00563390` -> `FUN_0058c1e0` -> `FUN_0058c120`), seeded from one
/// caller roll. It moves regiments, facilities, energy and raw material; it
/// writes no holder. port: the original draws from its one generator; a
/// seeded stream keeps the caller's budget at one roll.
pub fn resolve(
    world: &mut GameWorld,
    transport: &mut TroopTransportState,
    system: SystemKey,
    fleets: &[FleetKey],
    roll: f64,
) -> AssaultReport {
    let mut draws = MissionRng::seeded(roll, crate::bombardment::system_seed(world, system));
    resolve_with(world, transport, system, fleets, &mut draws)
}

#[expect(
    clippy::too_many_lines,
    reason = "The run's six steps share its counts and read in the original's order."
)]
fn resolve_with(
    world: &mut GameWorld,
    transport: &mut TroopTransportState,
    system: SystemKey,
    fleets: &[FleetKey],
    draws: &mut impl Draws,
) -> AssaultReport {
    // FUN_0058c1e0.c:20-37: the first flagged fleet's side attacks.
    let Some(sys) = world.systems.get(system) else {
        return AssaultReport::default();
    };
    let flagged: Vec<FleetKey> = sys
        .fleets
        .iter()
        .copied()
        .filter(|fleet| fleets.contains(fleet))
        .collect();
    let Some(assaulter) = flagged
        .first()
        .map(|&fleet| side_of(world.fleets[fleet].is_alliance))
    else {
        return AssaultReport::default();
    };
    let team: Vec<FleetKey> = flagged
        .into_iter()
        .filter(|&fleet| side_of(world.fleets[fleet].is_alliance) == assaulter)
        .collect();
    let holder = crate::bombardment::holder(world, system);
    let value = |id| world.gnprtb.value(id, world.difficulty_index);
    // port: `.max(1)` only guards a divisor absent from the data.
    let fire_divisor = value(GNPRTB_FIRE_DIVISOR).max(1);
    let leadership_divisor = value(GNPRTB_LEADERSHIP_DIVISOR).max(1);
    let (engagement_draw, attacker_dies, defender_dies, collateral_chance) = (
        value(GNPRTB_ENGAGEMENT_DRAW),
        value(GNPRTB_ATTACKER_DIES),
        value(GNPRTB_DEFENDER_DIES),
        value(GNPRTB_COLLATERAL_CHANCE),
    );

    // Step 1 (FUN_0058c2d0): count both sides.
    let attackers_at_start = to_i32(attackers(world, transport, &team).len());
    let defenders_at_start = to_i32(defenders(world, system, holder).len());
    let (mut left, mut standing) = (attackers_at_start, defenders_at_start);
    let mut report = AssaultReport {
        system: Some(system),
        assaulter: Some(assaulter),
        holder_before: Some(holder),
        attackers: attackers_at_start.cast_unsigned(),
        defenders: defenders_at_start.cast_unsigned(),
        ..AssaultReport::default()
    };

    // Step 2 (FUN_0058c3c0): every active defence facility of either side,
    // in container order, fires with chance attack / GNPRTB 0x200 at the
    // index-th current attacker; an index past the survivors misses. The
    // loop stops once a shot kills the last attacker.
    let batteries: Vec<(DefenseFacilityKey, i32)> = world.systems[system]
        .defense_facilities
        .iter()
        .filter_map(|&key| {
            let facility = world.defense_facilities.get(key)?;
            let attack = world
                .buildable_classes
                .get(&facility.class_dat_id)
                .map_or(0, |class| class.stats.attack_strength.cast_signed());
            Some((key, attack))
        })
        .collect();
    let mut stop = false;
    for (facility, attack) in batteries {
        if stop {
            break;
        }
        let fired = draws.chance(attack / fire_divisor);
        let mut killed = None;
        if fired {
            let index = draws.draw(attackers_at_start - 1);
            let current = attackers(world, transport, &team);
            if let Some(&(troop, _)) = usize::try_from(index).ok().and_then(|at| current.get(at)) {
                kill_attacker(world, transport, troop, Cause::DefenceFire, &mut report);
                left -= 1;
                stop = left == 0;
                killed = Some(troop);
            }
        }
        report.shots.push(DefenceShot {
            facility,
            fired,
            killed,
        });
    }

    // Step 3 (FUN_0058c500 -> FUN_0058bec0): each attacker in walk order
    // picks the index-th current defender over the starting count; a hit
    // counts an engagement and rolls rand[0, G 0x202] plus its fleet
    // General's leadership / G 0x201 and its class attack, less the
    // system General's leadership / G 0x201 and the defender's class
    // defence. The walk stops once either side has none left.
    let mut engagements = 0;
    if left != 0 && standing != 0 {
        let lead_defence = crate::bombardment::defending_general(world, system, holder)
            .map_or(0, |key| crate::bombardment::leadership(world, key));
        for (attacker, fleet) in attackers(world, transport, &team) {
            if left == 0 || standing == 0 {
                break;
            }
            let index = draws.draw(defenders_at_start - 1);
            let current = defenders(world, system, holder);
            let Some(&defender) = usize::try_from(index).ok().and_then(|at| current.get(at)) else {
                continue;
            };
            engagements += 1;
            let lead_attack = crate::bombardment::fleet_officer(world, fleet, CommandKind::General)
                .map_or(0, |key| crate::bombardment::leadership(world, key));
            let (attack, _) = troop_strengths(world, attacker);
            let (_, defence) = troop_strengths(world, defender);
            let roll = draws.draw(engagement_draw) + (lead_attack / leadership_divisor + attack)
                - (lead_defence / leadership_divisor + defence);
            // FUN_0058bec0.c:93-106: the defender's death is read first.
            let outcome = if roll >= defender_dies {
                kill_defender(world, defender, &mut report);
                standing -= 1;
                Outcome::DefenderDied
            } else if roll <= attacker_dies {
                kill_attacker(world, transport, attacker, Cause::Engagement, &mut report);
                left -= 1;
                Outcome::AttackerDied
            } else {
                Outcome::Stood
            };
            report.engagements.push(Engagement {
                attacker,
                defender,
                roll,
                outcome,
            });
        }
    }

    // Step 4 (FUN_0058c580): one GNPRTB 0x205 percent roll per engagement,
    // then per hit one draw over the non-HQ facilities plus a slot for
    // nonzero energy and one for nonzero raw material, recounted per hit.
    let hits = (0..engagements)
        .filter(|_| draws.chance(collateral_chance))
        .count();
    for _ in 0..hits {
        let facilities = collateral_facilities(world, system);
        let sys = &world.systems[system];
        let (energy, raw) = (sys.total_energy != 0, sys.raw_materials != 0);
        let count = to_i32(facilities.len());
        let total = count + i32::from(energy) + i32::from(raw);
        if total == 0 {
            break;
        }
        // FUN_0055ebc0.c:17-35.
        let pick = draws.draw(total - 1);
        if let Some(&facility) = usize::try_from(pick).ok().and_then(|at| facilities.get(at)) {
            report.facilities_lost.push(facility);
            crate::missions::destroy_target(
                world,
                match facility {
                    FacilityRef::Defense(key) => MissionTarget::DefenseFacility(key),
                    FacilityRef::Manufacturing(key) => MissionTarget::ManufacturingFacility(key),
                    FacilityRef::Production(key) => MissionTarget::ProductionFacility(key),
                },
            );
        } else if pick == count && energy {
            let sys = &mut world.systems[system];
            sys.total_energy -= 1;
            report.energy += 1;
        } else if raw {
            let sys = &mut world.systems[system];
            sys.raw_materials -= 1;
            report.raw_material += 1;
        }
    }

    // Step 5 (FUN_0058c810): with attackers left and no defender, the
    // first FUN_00559fe0(side, 0) regiments in walk order land.
    if left != 0 && standing == 0 {
        let limit = crate::economy::garrison_requirement(
            assaulter,
            0,
            false,
            false,
            &world.gnprtb,
            world.difficulty_index,
        );
        let landing: Vec<(TroopKey, FleetKey)> = attackers(world, transport, &team)
            .into_iter()
            .take(usize::try_from(limit).unwrap_or(0))
            .collect();
        let mut down = Vec::new();
        for &fleet in &team {
            let troops: Vec<TroopKey> = landing
                .iter()
                .filter(|(_, each)| *each == fleet)
                .map(|&(troop, _)| troop)
                .collect();
            if !troops.is_empty() {
                // The team's fleets orbit the system, so each lands; the
                // report takes only the regiments that did.
                down.extend(
                    transport
                        .disembark_selected(world, fleet, system, &troops)
                        .unwrap_or_default(),
                );
            }
        }
        report.landed = landing
            .into_iter()
            .map(|(troop, _)| troop)
            .filter(|troop| down.contains(troop))
            .collect();
    }
    // Step 6 (FUN_0058c8b0) clears the flags the run held.
    report
}

/// An attacking regiment's slot `+0xac(0xe)`: it leaves its ship and the
/// arena, and its value joins its side's loss counter A
/// (`FUN_00508c60`).
fn kill_attacker(
    world: &mut GameWorld,
    transport: &mut TroopTransportState,
    troop: TroopKey,
    cause: Cause,
    report: &mut AssaultReport,
) {
    if let Some(loss) = loss(world, troop) {
        report.losses.push(loss);
    }
    report.attackers_lost.push((troop, cause));
    transport.destroy_embarked(world, troop);
}

/// A defending regiment's slot `+0xac(0xe)`: it leaves the system and the
/// arena, and its value joins its side's loss counter A.
fn kill_defender(world: &mut GameWorld, troop: TroopKey, report: &mut AssaultReport) {
    if let Some(loss) = loss(world, troop) {
        report.losses.push(loss);
    }
    report.defenders_lost.push(troop);
    crate::missions::destroy_target(world, MissionTarget::Troop(troop));
}

/// A regiment's loss as event `0x300` carries it (`FUN_00508c60`): its
/// side, its slot `+0x1c0` value and reason `0xe`.
fn loss(world: &GameWorld, troop: TroopKey) -> Option<UnitLoss> {
    let unit = world.troops.get(troop)?;
    Some(UnitLoss {
        side: side_of(unit.is_alliance),
        value: crate::support_shift::regiment_value(world, unit.class_dat_id),
        reason: REASON_GROUND_COMBAT,
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::carriage::ProductRoom;
    use super::*;
    use crate::ids::{CapitalShipKey, DatId};
    use crate::tick::TickEvent;
    use crate::world::{
        BuildableClass, CapitalShipClass, Character, ClassStats, ControlKind,
        DefenseFacilityInstance, Fleet, GnprtbEntry, GnprtbParams, ManufacturingFacilityInstance,
        Sector, ShipInstance, System, TroopClassDef, TroopUnit,
    };
    use std::collections::VecDeque;

    const BOTH: [(Faction, Faction); 2] = [
        (Faction::Empire, Faction::Alliance),
        (Faction::Alliance, Faction::Empire),
    ];

    /// Draws taken in order, each request's span recorded; an exhausted
    /// script yields 0.
    #[derive(Default)]
    struct Script {
        values: VecDeque<i32>,
        asked: Vec<i32>,
    }

    impl Draws for Script {
        fn draw(&mut self, n: i32) -> i32 {
            self.asked.push(n);
            self.values.pop_front().unwrap_or(0)
        }

        fn chance(&mut self, percent: i32) -> bool {
            self.draw(99) < percent
        }
    }

    fn script(values: &[i32]) -> Script {
        Script {
            values: values.iter().copied().collect(),
            asked: Vec::new(),
        }
    }

    fn entry(id: u16, value: i32) -> GnprtbEntry {
        GnprtbEntry {
            parameter_id: u32::from(id),
            development: value,
            alliance_sp_easy: value,
            alliance_sp_medium: value,
            alliance_sp_hard: value,
            empire_sp_easy: value,
            empire_sp_medium: value,
            empire_sp_hard: value,
            multiplayer: value,
        }
    }

    /// A system held by `holder` with the shipped GNPRTB `0x200`-`0x205`
    /// (5, 30, 10, 4, 6, 8), `0x1e03` (2), 7761 (60) and 7762 (-10).
    fn world(holder: Option<Faction>) -> (GameWorld, SystemKey) {
        let mut world = GameWorld {
            gnprtb: GnprtbParams::new(vec![
                entry(0x200, 5),
                entry(0x201, 30),
                entry(0x202, 10),
                entry(0x203, 4),
                entry(0x204, 6),
                entry(0x205, 8),
                entry(0x1e03, 2),
                entry(7761, 60),
                entry(7762, -10),
            ]),
            ..GameWorld::default()
        };
        let sector = world.sectors.insert(Sector::default());
        let system = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0091),
            sector,
            is_populated: true,
            control: holder.map_or(ControlKind::Uncontrolled, ControlKind::Controlled),
            ..System::default()
        });
        world.sectors[sector].systems.push(system);
        (world, system)
    }

    /// port: test fixture values, not rules.
    fn transport_class(world: &mut GameWorld) -> CapitalShipKey {
        world.capital_ship_classes.insert(CapitalShipClass {
            family: 0x14,
            hull: 100,
            troop_capacity: 20,
            ..CapitalShipClass::default()
        })
    }

    fn fleet(world: &mut GameWorld, system: SystemKey, side: Faction, ships: usize) -> FleetKey {
        let class = transport_class(world);
        let is_alliance = side == Faction::Alliance;
        let key = world.fleets.insert(Fleet {
            location: system,
            capital_ships: (0..ships)
                .map(|_| ShipInstance::new(class, 100, is_alliance))
                .collect(),
            is_alliance,
            ..Fleet::default()
        });
        world.systems[system].fleets.push(key);
        key
    }

    /// A regiment class (TROOPSD attack and defence) valued `value`.
    /// port: test fixture values, not rules.
    fn troop_class(world: &mut GameWorld, attack: u32, defence: u32, value: u32) -> DatId {
        let id = DatId::new(0x1000_0000 | (attack << 16) | (defence << 8) | value);
        world.troop_classes.insert(
            id,
            TroopClassDef {
                attack_strength: attack,
                defense_strength: defence,
                detection: 0,
            },
        );
        world.buildable_classes.insert(
            id,
            BuildableClass {
                stats: ClassStats {
                    uprising_defense: value,
                    ..ClassStats::default()
                },
                ..BuildableClass::default()
            },
        );
        id
    }

    fn regiment(world: &mut GameWorld, system: SystemKey, side: Faction, class: DatId) -> TroopKey {
        let key = world.troops.insert(TroopUnit {
            class_dat_id: class,
            is_alliance: side == Faction::Alliance,
        });
        world.systems[system].ground_units.push(key);
        key
    }

    /// `count` regiments of `class` loaded aboard `fleet`.
    fn load(
        world: &mut GameWorld,
        transport: &mut TroopTransportState,
        fleet: FleetKey,
        class: DatId,
        count: usize,
    ) -> Vec<TroopKey> {
        let system = world.fleets[fleet].location;
        let side = side_of(world.fleets[fleet].is_alliance);
        let troops: Vec<TroopKey> = (0..count)
            .map(|_| regiment(world, system, side, class))
            .collect();
        if !troops.is_empty() {
            transport
                .embark(world, &ProductRoom::default(), fleet, &troops)
                .unwrap();
        }
        troops
    }

    /// A defence facility of `family` with `attack` (DEFFACSD).
    fn defence(
        world: &mut GameWorld,
        system: SystemKey,
        side: Faction,
        family: u8,
        attack: u32,
    ) -> DefenseFacilityKey {
        let class_dat_id = DatId::new((u32::from(family) << 24) | attack);
        world.buildable_classes.insert(
            class_dat_id,
            BuildableClass {
                stats: ClassStats {
                    attack_strength: attack,
                    ..ClassStats::default()
                },
                ..BuildableClass::default()
            },
        );
        let key = world
            .defense_facilities
            .insert(DefenseFacilityInstance { class_dat_id, side });
        world.systems[system].defense_facilities.push(key);
        key
    }

    fn yard(world: &mut GameWorld, system: SystemKey, side: Faction) -> FacilityRef {
        let key = world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                class_dat_id: DatId::new(0x2800_0001),
                side,
                ..ManufacturingFacilityInstance::default()
            });
        world.systems[system].manufacturing_facilities.push(key);
        FacilityRef::Manufacturing(key)
    }

    fn general(
        world: &mut GameWorld,
        system: SystemKey,
        side: Faction,
        leadership: u32,
        fleet: Option<FleetKey>,
    ) {
        let key = world.characters.insert(Character {
            current_system: Some(system),
            current_fleet: fleet,
            is_alliance: side == Faction::Alliance,
            is_empire: side == Faction::Empire,
            command_kind: CommandKind::General,
            ..Character::default()
        });
        world.characters[key].leadership.base = leadership;
        if let Some(fleet) = fleet {
            world.fleets[fleet].characters.push(key);
        }
    }

    fn blockade(world: &GameWorld) -> BlockadeState {
        let mut state = BlockadeState::new();
        crate::blockade::BlockadeSystem::advance(&mut state, world, &[TickEvent { tick: 1 }]);
        state
    }

    /// An assault scene: `side`'s fleet of one ship carrying `attackers`
    /// regiments over `holder`'s system with `defenders` regiments.
    struct Scene {
        world: GameWorld,
        system: SystemKey,
        fleet: FleetKey,
        transport: TroopTransportState,
        attackers: Vec<TroopKey>,
        defenders: Vec<TroopKey>,
    }

    /// port: test fixture values, not rules.
    fn staged(side: Faction, holder: Faction, attackers: usize, defenders: usize) -> Scene {
        let (mut world, system) = world(Some(holder));
        let class = troop_class(&mut world, 0, 0, 7);
        let fleet = fleet(&mut world, system, side, 1);
        let mut transport = TroopTransportState::new();
        let attackers = load(&mut world, &mut transport, fleet, class, attackers);
        let defenders = (0..defenders)
            .map(|_| regiment(&mut world, system, holder, class))
            .collect();
        Scene {
            world,
            system,
            fleet,
            transport,
            attackers,
            defenders,
        }
    }

    fn run(scene: &mut Scene, draws: &mut Script) -> AssaultReport {
        let fleets = [scene.fleet];
        resolve_with(
            &mut scene.world,
            &mut scene.transport,
            scene.system,
            &fleets,
            draws,
        )
    }

    fn enabled(scene: &Scene) -> Result<(), AssaultRefusal> {
        let side = side_of(scene.world.fleets[scene.fleet].is_alliance);
        order_enabled(
            &scene.world,
            &MovementState::new(),
            &scene.transport,
            &blockade(&scene.world),
            &[scene.fleet],
            side,
        )
    }

    #[test]
    fn a_blockading_fleet_with_a_regiment_aboard_may_assault() {
        // FUN_005814d0: every refusal passes.
        for (side, holder) in BOTH {
            let scene = staged(side, holder, 1, 1);
            assert_eq!(enabled(&scene), Ok(()), "{side:?}");
        }
    }

    #[test]
    fn assault_refuses_a_fleet_whose_last_ship_is_unfinished_with_1_0x10_and_an_en_route_one_with_1_0x11_for_either_side(
    ) {
        // FUN_004fdf70 -> FUN_004ffa70.c:15-60 (ground-assault.md section 1
        // item 1): with no completed ship that is not en route, the last
        // ship's status stands, uncompleted 1/0x10 or en route 1/0x11. An
        // ordered ship is uncompleted (FUN_004f7d50(.., 1, 0); the route-1
        // day-115 save: +0x50 0x42).
        for (side, holder) in BOTH {
            let mut scene = staged(side, holder, 1, 1);
            scene.world.fleets[scene.fleet].capital_ships[0].completed = false;
            assert_eq!(
                enabled(&scene),
                Err(AssaultRefusal::LastShipUnfinished),
                "{side:?}"
            );
            let ship = &mut scene.world.fleets[scene.fleet].capital_ships[0];
            ship.completed = true;
            ship.transit = Some(crate::world::ShipTransit {
                from: scene.system,
                to: scene.system,
                arrival_tick: 9,
            });
            assert_eq!(
                enabled(&scene),
                Err(AssaultRefusal::LastShipEnRoute),
                "{side:?}"
            );
        }
    }

    #[test]
    fn each_assault_refusal_carries_the_status_its_function_writes() {
        // The pairs the advisor reacts to, either side alike: FUN_0051fe20
        // (1/0x16), FUN_004f9860 through FUN_004ffa70 (1/1),
        // FUN_004ffa70.c:15-60 (8/1, 1/0x10, 1/0x11), FUN_00553770 (1/0x14),
        // FUN_004fddd0 and FUN_00581ae0 (8/7, 8/5), FUN_00507d00 (1/0x22,
        // 0x90/5).
        for (refusal, status) in [
            (AssaultRefusal::NoTeam, (1, 0x16)),
            (AssaultRefusal::FleetRefused, (1, 1)),
            (AssaultRefusal::NoShip, (8, 1)),
            (AssaultRefusal::LastShipUnfinished, (1, 0x10)),
            (AssaultRefusal::LastShipEnRoute, (1, 0x11)),
            (AssaultRefusal::MixedHolders, (1, 0x14)),
            (AssaultRefusal::NotBlockaded, (8, 7)),
            (AssaultRefusal::NoRegiments, (8, 5)),
            (AssaultRefusal::Destroyed, (1, 0x22)),
            (AssaultRefusal::Shields, (0x90, 5)),
        ] {
            assert_eq!(refusal.status(), status, "{refusal:?}");
        }
    }

    #[test]
    fn a_fleet_whose_system_is_not_blockaded_is_refused() {
        // FUN_004fddd0.c:21-30 (8/7) reads fleet +0x58 bit 0x20, the copy
        // of its system's blockade bit (FUN_0050c0b0 -> FUN_004fe3f0): the
        // holder's own fleet in orbit lifts the bit (FUN_0050b8e0), and an
        // unpopulated neutral system (side 3) is never blockaded
        // ("populated or side not 3").
        for (side, holder) in BOTH {
            let mut scene = staged(side, holder, 1, 1);
            fleet(&mut scene.world, scene.system, holder, 1);
            assert_eq!(
                enabled(&scene),
                Err(AssaultRefusal::NotBlockaded),
                "{side:?}"
            );
            let mut neutral = staged(side, holder, 1, 0);
            neutral.world.systems[neutral.system].control = ControlKind::Uncontrolled;
            neutral.world.systems[neutral.system].is_populated = false;
            assert_eq!(
                enabled(&neutral),
                Err(AssaultRefusal::NotBlockaded),
                "{side:?}"
            );
        }
    }

    #[test]
    fn a_fleet_alone_at_a_populated_neutral_system_may_assault() {
        // A neutral system is side 3 (FUN_0055a080, FUN_0050b610), which
        // FUN_0050b8e0 lets any fleet blockade while populated, so
        // FUN_004fddd0's bit check passes; TEXTSTRA 0x7093 and 0x7094 report
        // an assault on a neutral system (ground-assault.md).
        for (side, holder) in BOTH {
            let mut neutral = staged(side, holder, 1, 0);
            neutral.world.systems[neutral.system].control = ControlKind::Uncontrolled;
            assert!(neutral.world.systems[neutral.system].is_populated);
            assert_eq!(enabled(&neutral), Ok(()), "{side:?}");
        }
    }

    #[test]
    fn a_blockading_fleet_without_a_regiment_is_refused() {
        // FUN_00581ae0.c:96-104 (8/5).
        for (side, holder) in BOTH {
            let scene = staged(side, holder, 0, 1);
            assert_eq!(
                enabled(&scene),
                Err(AssaultRefusal::NoRegiments),
                "{side:?}"
            );
        }
    }

    #[test]
    fn a_destroyed_system_is_refused() {
        // FUN_00507d00.c:24-31 (1/0x22).
        for (side, holder) in BOTH {
            let mut scene = staged(side, holder, 1, 1);
            scene.world.systems[scene.system].is_destroyed = true;
            assert_eq!(enabled(&scene), Err(AssaultRefusal::Destroyed), "{side:?}");
        }
    }

    #[test]
    fn another_sides_or_an_empty_team_is_refused() {
        // FUN_0051fe20 (1/0x16) and FUN_004ffa70 (1/1): the order's side
        // must own every fleet.
        for (side, holder) in BOTH {
            let scene = staged(side, holder, 1, 1);
            let blockade = blockade(&scene.world);
            let movement = MovementState::new();
            let refused = |fleets: &[FleetKey], as_side| {
                order_enabled(
                    &scene.world,
                    &movement,
                    &scene.transport,
                    &blockade,
                    fleets,
                    as_side,
                )
            };
            assert_eq!(
                refused(&[scene.fleet], holder),
                Err(AssaultRefusal::FleetRefused)
            );
            assert_eq!(refused(&[], side), Err(AssaultRefusal::NoTeam));
        }
    }

    #[test]
    fn two_shield_generators_of_either_side_refuse_and_one_does_not() {
        // FUN_00507d00.c:42-53: FUN_005278c0 counts [0x24,0x25) with no
        // side, against GNPRTB 0x1e03 (2; FUN_00558bb0.c:24, read by
        // FUN_0055a0b0).
        for (side, holder) in BOTH {
            let mut scene = staged(side, holder, 1, 1);
            defence(&mut scene.world, scene.system, holder, SHIELD_FAMILY, 0);
            assert_eq!(enabled(&scene), Ok(()), "{side:?}");
            defence(&mut scene.world, scene.system, side, SHIELD_FAMILY, 0);
            assert_eq!(enabled(&scene), Err(AssaultRefusal::Shields), "{side:?}");
        }
    }

    #[test]
    fn a_weapon_always_fires_and_a_zero_attack_facility_draws_but_never_does() {
        // FUN_0055ead0.c:9: rand[0,99] < attack / GNPRTB 0x200. A KDY
        // (attack 2000) fires on any draw; a shield generator (0) still
        // draws its roll (FUN_0058c3c0.c:32-34).
        for (side, holder) in BOTH {
            let mut scene = staged(side, holder, 2, 0);
            let shield = defence(&mut scene.world, scene.system, holder, SHIELD_FAMILY, 0);
            let kdy = defence(&mut scene.world, scene.system, holder, 0x22, 2000);
            // Shield: 0 (no fire); KDY: 99 (fires), index 1.
            let mut draws = script(&[0, 99, 1]);
            let report = run(&mut scene, &mut draws);
            assert_eq!(draws.asked, [99, 99, 1], "{side:?}");
            assert_eq!(
                report.shots,
                [
                    DefenceShot {
                        facility: shield,
                        fired: false,
                        killed: None
                    },
                    DefenceShot {
                        facility: kdy,
                        fired: true,
                        killed: Some(scene.attackers[1])
                    },
                ]
            );
            assert_eq!(
                report.attackers_lost,
                [(scene.attackers[1], Cause::DefenceFire)]
            );
            assert!(!scene.world.troops.contains_key(scene.attackers[1]));
            assert_eq!(report.landed, [scene.attackers[0]]);
        }
    }

    #[test]
    fn a_battery_fires_on_a_draw_below_its_attack_over_gnprtb_0x200() {
        // FUN_0055ead0.c:9: rand[0,99] < attack / GNPRTB 0x200 (5,
        // FUN_0055ea00.c:9). An attack of 400 fires on 79 and below, not on
        // 80 (FUN_0058c3c0.c:32-45).
        for (side, holder) in BOTH {
            for (draw, fired) in [(80, false), (79, true)] {
                let mut scene = staged(side, holder, 1, 0);
                defence(&mut scene.world, scene.system, holder, 0x22, 400);
                let report = run(&mut scene, &mut script(&[draw, 0]));
                assert_eq!(report.shots[0].fired, fired, "{side:?} {draw}");
                let killed = fired.then_some(scene.attackers[0]);
                assert_eq!(report.shots[0].killed, killed, "{side:?} {draw}");
            }
        }
    }

    #[test]
    fn a_shot_past_the_surviving_attackers_misses() {
        // FUN_0058c3c0.c:38-45: the index is over the starting count, the
        // pick over the current list (FUN_0058bd00).
        for (side, holder) in BOTH {
            let mut scene = staged(side, holder, 2, 0);
            defence(&mut scene.world, scene.system, holder, 0x22, 2000);
            defence(&mut scene.world, scene.system, holder, 0x23, 800);
            // First shot kills index 0; the second draws index 1 of one left.
            let mut draws = script(&[0, 0, 0, 1]);
            let report = run(&mut scene, &mut draws);
            assert_eq!(report.shots[1].killed, None, "{side:?}");
            assert_eq!(report.attackers_lost.len(), 1);
            assert_eq!(report.landed, [scene.attackers[1]]);
        }
    }

    #[test]
    fn the_fire_stops_with_no_draw_once_the_last_attacker_dies() {
        // FUN_0058c3c0.c:29, 41-43: the stop flag is read before the next
        // facility, which draws nothing.
        for (side, holder) in BOTH {
            let mut scene = staged(side, holder, 1, 1);
            defence(&mut scene.world, scene.system, holder, 0x22, 2000);
            defence(&mut scene.world, scene.system, holder, 0x22, 2000);
            let mut draws = script(&[0, 0]);
            let report = run(&mut scene, &mut draws);
            assert_eq!(draws.asked, [99, 0], "{side:?}");
            assert_eq!(report.shots.len(), 1);
            assert!(report.engagements.is_empty());
            assert!(report.landed.is_empty());
        }
    }

    /// One attacker against one defender, the classes and Generals set,
    /// the engagement drawing `roll`.
    fn engage(
        side: Faction,
        holder: Faction,
        roll: i32,
        attack: u32,
        defence: u32,
        leads: (u32, u32),
    ) -> (Scene, AssaultReport) {
        let (mut world, system) = world(Some(holder));
        let attack_class = troop_class(&mut world, attack, 0, 3);
        let defence_class = troop_class(&mut world, 0, defence, 5);
        let fleet = fleet(&mut world, system, side, 1);
        let mut transport = TroopTransportState::new();
        let attackers = load(&mut world, &mut transport, fleet, attack_class, 1);
        let defenders = vec![regiment(&mut world, system, holder, defence_class)];
        if leads.0 > 0 {
            general(&mut world, system, side, leads.0, Some(fleet));
        }
        if leads.1 > 0 {
            general(&mut world, system, holder, leads.1, None);
        }
        let mut scene = Scene {
            world,
            system,
            fleet,
            transport,
            attackers,
            defenders,
        };
        let mut draws = script(&[0, roll]);
        let report = run(&mut scene, &mut draws);
        (scene, report)
    }

    #[test]
    fn four_or_less_kills_the_attacker_six_or_more_the_defender_five_stands() {
        // FUN_0055eb00.c:14-17: r = rand[0,10] + attack - defence against
        // GNPRTB 0x203 (4) and 0x204 (6) (FUN_0055ea00.c:24, 31); attack is
        // the attacker's TROOPSD class +0x64 (FUN_00520b90), defence the
        // defender's +0x68 (FUN_00520ba0).
        for (side, holder) in BOTH {
            for (roll, outcome) in [
                (4, Outcome::AttackerDied),
                (5, Outcome::Stood),
                (6, Outcome::DefenderDied),
            ] {
                let (scene, report) = engage(side, holder, roll + 3, 2, 5, (0, 0));
                assert_eq!(report.engagements[0].roll, roll, "{side:?}");
                assert_eq!(report.engagements[0].outcome, outcome, "{side:?}");
                let attacker_alive = scene.world.troops.contains_key(scene.attackers[0]);
                let defender_alive = scene.world.troops.contains_key(scene.defenders[0]);
                assert_eq!(attacker_alive, outcome != Outcome::AttackerDied);
                assert_eq!(defender_alive, outcome != Outcome::DefenderDied);
            }
        }
    }

    #[test]
    fn each_generals_leadership_counts_in_whole_thirties() {
        // FUN_0055eb00.c:12-13: leadership / GNPRTB 0x201 (30), truncated;
        // the fleet's General (FUN_004fd790(fleet, 3)) adds, the system's
        // (FUN_00509330(system, 3)) subtracts.
        for (side, holder) in BOTH {
            for (leads, term) in [
                ((29, 0), 0),
                ((30, 0), 1),
                ((60, 0), 2),
                ((0, 59), -1),
                ((0, 60), -2),
                ((60, 30), 1),
            ] {
                let (_, report) = engage(side, holder, 5, 0, 0, leads);
                assert_eq!(report.engagements[0].roll, 5 + term, "{side:?} {leads:?}");
            }
        }
    }

    #[test]
    fn a_pick_past_the_standing_defenders_draws_no_roll_and_counts_no_engagement() {
        // FUN_0058bec0.c:39-60: rand[0, D0 - 1]; an index at or past the
        // defenders left picks none, and rand[0,10] is not drawn.
        for (side, holder) in BOTH {
            let mut scene = staged(side, holder, 3, 2);
            // First attacker kills defender 0 (roll 10 >= 6); the second
            // picks index 1 of one left; the third picks index 0 and stands.
            let mut draws = script(&[0, 10, 1, 0, 5]);
            let report = run(&mut scene, &mut draws);
            assert_eq!(draws.asked, [1, 10, 1, 1, 10, 99, 99], "{side:?}");
            assert_eq!(report.engagements.len(), 2);
            assert_eq!(report.defenders_lost, [scene.defenders[0]]);
        }
    }

    #[test]
    fn the_walk_ends_when_the_last_defender_dies() {
        // FUN_0058c500 walks the attackers through FUN_0058bec0, whose
        // stop flag (FUN_0058bec0.c:108-115) ends the walk once a side has
        // none left.
        for (side, holder) in BOTH {
            let mut scene = staged(side, holder, 3, 1);
            let mut draws = script(&[0, 10]);
            let report = run(&mut scene, &mut draws);
            assert_eq!(draws.asked, [0, 10, 99], "{side:?}");
            assert_eq!(report.engagements.len(), 1);
        }
    }

    #[test]
    fn regiments_aboard_a_lost_ship_do_not_land() {
        // FUN_0058ba10 walks the fleet's live ships (FUN_00502e30, mode 1)
        // and the regiments aboard each (FUN_00504cc0).
        for (side, holder) in BOTH {
            let (mut world, system) = world(Some(holder));
            let class = troop_class(&mut world, 0, 0, 7);
            let fleet = fleet(&mut world, system, side, 2);
            let mut transport = TroopTransportState::new();
            let attackers = load(&mut world, &mut transport, fleet, class, 2);
            let lost = transport.carrier(attackers[0]);
            for ship in &mut world.fleets[fleet].capital_ships {
                ship.alive = ship.tag != lost;
            }
            let kept: Vec<TroopKey> = attackers
                .iter()
                .copied()
                .filter(|&troop| transport.carrier(troop) != lost)
                .collect();
            let mut scene = Scene {
                world,
                system,
                fleet,
                transport,
                attackers,
                defenders: Vec::new(),
            };

            let report = run(&mut scene, &mut script(&[]));

            assert_eq!(report.landed, kept, "{side:?}");
        }
    }

    #[test]
    fn up_to_six_survivors_land_in_walk_order_and_none_while_a_defender_stands() {
        // FUN_0058c810.c:16-22: FUN_00559fe0(side, 0) = ceil(60 / 10) = 6
        // (GNPRTB 7761, 7762); FUN_0058be10 lands them in order.
        for (side, holder) in BOTH {
            let mut scene = staged(side, holder, 8, 0);
            let report = run(&mut scene, &mut script(&[]));
            assert_eq!(report.landed, &scene.attackers[..6], "{side:?}");
            assert_eq!(scene.transport.cargo(scene.fleet), &scene.attackers[6..]);
            let ground = &scene.world.systems[scene.system].ground_units;
            assert!(scene.attackers[..6].iter().all(|t| ground.contains(t)));

            let mut few = staged(side, holder, 4, 0);
            let report = run(&mut few, &mut script(&[]));
            assert_eq!(report.landed, few.attackers, "{side:?}");

            let mut held = staged(side, holder, 4, 1);
            // Every engagement picks the defender and stands.
            let mut draws = script(&[0, 5, 0, 5, 0, 5, 0, 5]);
            let report = run(&mut held, &mut draws);
            assert!(report.landed.is_empty(), "{side:?}");
            assert_eq!(held.transport.cargo(held.fleet), held.attackers);
        }
    }

    #[test]
    fn collateral_rolls_each_engagement_then_one_draw_per_hit_sparing_the_hq() {
        // FUN_0058c580.c:34-103, FUN_0055eb80, FUN_0055ebc0: E percent rolls
        // against GNPRTB 0x205 (8), then per hit rand[0, T-1] with
        // T = facilities + (energy != 0) + (raw != 0); the facilities are
        // both sides' (FUN_00539d70(system, 1)) and the HQ is no slot
        // (FUN_00526cf0).
        for (side, holder) in BOTH {
            let mut scene = staged(side, holder, 2, 2);
            scene.world.systems[scene.system].is_headquarters = holder == Faction::Alliance;
            let own = yard(&mut scene.world, scene.system, holder);
            let theirs = yard(&mut scene.world, scene.system, side);
            scene.world.systems[scene.system].total_energy = 1;
            scene.world.systems[scene.system].raw_materials = 1;
            // Two engagements that stand; both collateral rolls hit (7 < 8);
            // the hits take the yard at 1, then energy at index 1 of 3.
            let mut draws = script(&[0, 5, 1, 5, 7, 7, 1, 1]);
            let report = run(&mut scene, &mut draws);
            assert_eq!(draws.asked, [1, 10, 1, 10, 99, 99, 3, 2], "{side:?}");
            assert_eq!(report.facilities_lost, [theirs]);
            assert_eq!(report.energy, 1);
            assert_eq!(report.raw_material, 0);
            let sys = &scene.world.systems[scene.system];
            assert_eq!(sys.total_energy, 0, "{side:?}");
            assert_eq!(sys.manufacturing_facilities.len(), 1);
            assert_eq!(
                FacilityRef::Manufacturing(sys.manufacturing_facilities[0]),
                own
            );
            assert_eq!(sys.is_headquarters, holder == Faction::Alliance);
        }
    }

    #[test]
    fn a_hit_on_the_energy_slot_takes_raw_material_when_energy_is_spent() {
        // FUN_0055ebc0.c:23-34: r == N takes energy while it is nonzero,
        // else raw material; r > N raw material.
        for (side, holder) in BOTH {
            let mut scene = staged(side, holder, 1, 1);
            scene.world.systems[scene.system].raw_materials = 2;
            let mut draws = script(&[0, 5, 0, 0]);
            let report = run(&mut scene, &mut draws);
            assert_eq!(draws.asked, [0, 10, 99, 0], "{side:?}");
            assert_eq!(report.raw_material, 1);
            assert_eq!(scene.world.systems[scene.system].raw_materials, 1);
        }
    }

    #[test]
    fn a_hit_with_nothing_left_to_take_ends_the_collateral() {
        // FUN_0058c580.c:61-64: T = 0 stops the loop with no draw.
        for (side, holder) in BOTH {
            let mut scene = staged(side, holder, 1, 1);
            let mut draws = script(&[0, 5, 0]);
            let report = run(&mut scene, &mut draws);
            assert_eq!(draws.asked, [0, 10, 99], "{side:?}");
            assert!(report.facilities_lost.is_empty());
        }
    }

    #[test]
    fn one_battery_and_two_against_two_draw_in_the_ledgers_order() {
        // ground-assault.md, "Draw ledger": fire, engagements, collateral.
        for (side, holder) in BOTH {
            let mut scene = staged(side, holder, 2, 2);
            defence(&mut scene.world, scene.system, holder, 0x22, 2000);
            scene.world.systems[scene.system].total_energy = 3;
            // Fire: the KDY's roll, then index 1 over A0 - 1 kills the second
            // attacker. The survivor picks defender 0 and kills it (10).
            // Collateral: one roll (1 < 8), then index 1 of the KDY and
            // energy takes energy.
            let mut draws = script(&[0, 1, 0, 10, 1, 1]);
            let report = run(&mut scene, &mut draws);
            assert_eq!(draws.asked, [99, 1, 1, 10, 99, 1], "{side:?}");
            assert_eq!(
                report.attackers_lost,
                [(scene.attackers[1], Cause::DefenceFire)]
            );
            assert_eq!(report.defenders_lost, [scene.defenders[0]]);
            assert_eq!(report.energy, 1);
            assert!(report.landed.is_empty());
        }
    }

    #[test]
    fn every_dead_regiment_carries_its_value_and_reason_0xe() {
        // FUN_0058c3c0.c:44, FUN_0058bec0.c:106: slot +0xac(0xe); event
        // 0x300 credits the owner's counter A with slot +0x1c0
        // (FUN_00508c60); facilities count nothing.
        for (side, holder) in BOTH {
            let (_, report) = engage(side, holder, 0, 0, 0, (0, 0));
            assert_eq!(
                report.losses,
                [UnitLoss {
                    side,
                    value: 3,
                    reason: REASON_GROUND_COMBAT
                }]
            );
            let (_, report) = engage(side, holder, 10, 0, 0, (0, 0));
            assert_eq!(
                report.losses,
                [UnitLoss {
                    side: holder,
                    value: 5,
                    reason: REASON_GROUND_COMBAT
                }]
            );
        }
    }

    #[test]
    fn the_first_flagged_fleets_side_attacks_and_names_the_report() {
        // FUN_0058c1e0.c:20-43: the attacker is the first flagged fleet's
        // side, the holder read from the system's side bits.
        for (side, holder) in BOTH {
            let mut scene = staged(side, holder, 1, 1);
            let report = run(&mut scene, &mut script(&[0, 5]));
            assert_eq!(report.system, Some(scene.system));
            assert_eq!(report.assaulter, Some(side));
            assert_eq!(report.holder_before, Some(holder));
            assert_eq!((report.attackers, report.defenders), (1, 1));
            assert_eq!(report.holder_after, None);
            let none = resolve_with(
                &mut scene.world,
                &mut scene.transport,
                scene.system,
                &[],
                &mut script(&[]),
            );
            assert_eq!(none, AssaultReport::default());
        }
    }

    #[test]
    fn a_fleet_the_order_did_not_flag_takes_no_part() {
        // FUN_004feb60 flags each team fleet (+0x58 bit 7), and FUN_0058ba10
        // walks only the attacker side's flagged fleets (ground-assault.md,
        // section 3, "Attackers"): a fleet of that side the order did not
        // name neither counts nor lands.
        for (side, holder) in BOTH {
            let mut scene = staged(side, holder, 1, 0);
            let class = troop_class(&mut scene.world, 0, 0, 7);
            let idle = fleet(&mut scene.world, scene.system, side, 1);
            let aboard = load(&mut scene.world, &mut scene.transport, idle, class, 2);
            let report = run(&mut scene, &mut script(&[]));
            assert_eq!(report.attackers, 1, "{side:?}");
            assert_eq!(report.landed, scene.attackers, "{side:?}");
            assert_eq!(scene.transport.cargo(idle), aboard, "{side:?}");
        }
    }

    /// `side`'s fleet of two ships over `holder`'s system with one regiment
    /// aboard the first ship it was given and, when `second`, another
    /// aboard the other ship once the first is made unusable by `unusable`.
    /// Returns the scene and the regiment aboard the unusable ship.
    fn split_cargo(
        side: Faction,
        holder: Faction,
        unusable: fn(&mut ShipInstance, SystemKey),
        second: bool,
    ) -> (Scene, TroopKey) {
        let (mut world, system) = world(Some(holder));
        let class = troop_class(&mut world, 0, 0, 7);
        let fleet = fleet(&mut world, system, side, 2);
        let mut transport = TroopTransportState::new();
        let stranded = load(&mut world, &mut transport, fleet, class, 1)[0];
        let tag = transport.carrier(stranded);
        let ship = world.fleets[fleet]
            .capital_ships
            .iter_mut()
            .find(|ship| ship.tag == tag)
            .unwrap();
        unusable(ship, system);
        let mut attackers = vec![stranded];
        if second {
            attackers.extend(load(&mut world, &mut transport, fleet, class, 1));
            assert_ne!(transport.carrier(attackers[1]), tag);
        }
        let scene = Scene {
            world,
            system,
            fleet,
            transport,
            attackers,
            defenders: Vec::new(),
        };
        (scene, stranded)
    }

    /// An ordered ship (`completed` clear) and one on a leg of its own
    /// (`transit`): neither is usable (`+0x50` bit 0, `0x004f7b80`).
    const UNUSABLE: [fn(&mut ShipInstance, SystemKey); 2] = [
        |ship, _| ship.completed = false,
        |ship, system| {
            ship.transit = Some(crate::world::ShipTransit {
                from: system,
                to: system,
                arrival_tick: 9,
            });
        },
    ];

    #[test]
    fn a_regiment_aboard_only_an_unfinished_or_en_route_ship_is_refused_for_either_side() {
        // FUN_00581ae0.c:63-80 (8/5): the gate walks the fleet's usable
        // ships (FUN_00502db0, mode 1) and the regiments aboard each
        // (FUN_00504c40, mode 1); the fleet's other ship keeps it usable.
        for (side, holder) in BOTH {
            for unusable in UNUSABLE {
                let (scene, _) = split_cargo(side, holder, unusable, false);
                assert!(scene.world.fleets[scene.fleet].usable(), "{side:?}");
                assert_eq!(
                    enabled(&scene),
                    Err(AssaultRefusal::NoRegiments),
                    "{side:?}"
                );
            }
        }
    }

    #[test]
    fn regiments_aboard_an_unfinished_or_en_route_ship_neither_count_nor_land_for_either_side() {
        // FUN_0058ba10.c:37 walks the attacker side's fleets' ships in mode
        // 1 (FUN_00502e30): usable ships only, and the regiments aboard
        // each (FUN_00504cc0).
        for (side, holder) in BOTH {
            for unusable in UNUSABLE {
                let (mut scene, stranded) = split_cargo(side, holder, unusable, true);
                let report = run(&mut scene, &mut script(&[]));
                assert_eq!(report.attackers, 1, "{side:?}");
                assert_eq!(report.landed, [scene.attackers[1]], "{side:?}");
                assert_eq!(scene.transport.cargo(scene.fleet), [stranded], "{side:?}");
            }
        }
    }

    #[test]
    fn only_the_holders_regiments_standing_at_the_system_defend() {
        // FUN_0058bbd0 -> FUN_00504cc0(system, 1, holder side): a regiment
        // of the attacker's side already on the ground is no defender, and
        // no attacker either (ground-assault.md, section 3, "Defenders").
        for (side, holder) in BOTH {
            let mut scene = staged(side, holder, 1, 1);
            let class = troop_class(&mut scene.world, 0, 0, 7);
            let grounded = regiment(&mut scene.world, scene.system, side, class);
            // The engagement picks index 0 of one defender and stands.
            let mut draws = script(&[0, 5]);
            let report = run(&mut scene, &mut draws);
            assert_eq!((report.attackers, report.defenders), (1, 1), "{side:?}");
            assert_eq!(draws.asked[0], 0, "{side:?}");
            assert_eq!(report.engagements[0].defender, scene.defenders[0]);
            assert!(scene.world.troops.contains_key(grounded), "{side:?}");
        }
    }
}
