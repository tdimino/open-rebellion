//! Mission system: all 11 mission types with MSTB probability lookup.
//!
//! Missions are assigned to characters and execute over multiple game-days.
//! When a mission completes, success is determined either by a quadratic
//! probability formula (fallback) or by a piecewise-linear MSTB table lookup
//! when `world.mission_tables` is populated by rebellion-data.
//!
//! # Design
//!
//! - `MissionState` holds every active mission and the members travelling
//!   to a mission target.
//! - `MissionSystem::advance(state, world, uprisings, tick_events, rolls)`
//!   returns a `MissionAdvance`: member moves and releases, phase 10
//!   results, and the missions that ended.
//! - The caller provides pre-generated random rolls so rebellion-core stays
//!   dependency-free and fully deterministic in tests.
//!
//! # Probability
//!
//! Two paths, in priority order:
//! 1. **MSTB lookup**: if `world.mission_tables` contains an entry for this mission kind,
//!    use `MstbTable::lookup(skill_score)` (piecewise-linear over `IntTableEntry` thresholds).
//! 2. **Quadratic fallback**: `clamp(a·score² + b·score + c, min%, max%)` — same as
//!    rebellion2's Mission.cs coefficients. Used when MSTB tables are not yet loaded.
//!
//! Combined: `total_prob = agent_prob · (1 − foil_prob)`.
//!
//! # Mission Types
//!
//! Nine types ported from REBEXE.EXE's 13-case dispatch (`FUN_0050d5a0`):
//! Diplomacy, Recruitment, Sabotage, Assassination, Espionage, Rescue, Abduction,
//! `InciteUprising`, Autoscrap.
//!
//! # Lifecycle
//!
//! A mission steps through phases `0..=0xb` (`FUN_005227d0`,
//! `ghidra/notes/mission-lifecycle.md`). It waits in phase 4 for its
//! members' transit and in phase 8 for its MISSNSD timer, rolls in phase 10,
//! repeats 10 -> 8 when its record says so, and ends in phase `0xb` once the
//! validator (`FUN_00522480`) or the last step sets an end code.
//!
//! After each phase change the detection run (`mission_detection`,
//! `FUN_00547f60`) lets the decoys draw off defenders, the defenders try to
//! detect the team, and phase 9 looks for a traitor; a detected team is
//! exposed, and its members evade and resign or are captured.
//!
//! The phase 10 roll and the probability paths above are interim until
//! F-019 phase 4 ports the per-member roll.

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::ids::{CharacterKey, SpecialForceKey, SystemKey};
use crate::tick::TickEvent;
use crate::world::{Character, GameWorld, Skill};

// ---------------------------------------------------------------------------
// MissionKind
// ---------------------------------------------------------------------------

/// All nine mission types recognised by the REBEXE.EXE dispatch (`FUN_0050d5a0`).
///
/// Type codes match the original binary's 13-case switch where applicable.
/// Coefficients are ported from rebellion2's Mission.cs; they serve as
/// fallback when `GameWorld::mission_tables` is not yet populated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissionKind {
    // ── Living Galaxy (implemented) ─────────────────────────────────────────
    /// Shift a system's popularity toward the sending faction.
    /// DIPLMSTB.DAT. Skill: diplomacy.
    Diplomacy,

    /// Recruit an available character for the sending faction.
    /// RCRTMSTB.DAT. Skill: leadership.
    Recruitment,

    // ── War Machine phase ────────────────────────────────────────────────────
    /// Destroy an enemy facility (type code 6 in REBEXE.EXE).
    /// SBTGMSTB.DAT. Skill: espionage.
    Sabotage,

    /// Kill an enemy character (type code 7 in REBEXE.EXE).
    /// ASSNMSTB.DAT. Skill: combat.
    Assassination,

    /// Gather intelligence on an enemy system.
    /// ESPIMSTB.DAT. Skill: espionage.
    Espionage,

    /// Free a captured character.
    /// RESCMSTB.DAT. Skill: combat.
    Rescue,

    /// Capture an enemy character.
    /// ABDCMSTB.DAT. Skill: espionage.
    Abduction,

    /// Trigger a planetary uprising.
    /// INCTMSTB.DAT. Skill: diplomacy.
    InciteUprising,

    /// Automatic scrapping of obsolete units (type code 21 / 0x15).
    /// No character required; always succeeds.
    Autoscrap,

    /// Suppress an active uprising on a system.
    /// SUBDMSTB.DAT. Skill: diplomacy.
    SubdueUprising,

    /// Sabotage the Death Star's construction (delay or destroy).
    /// DSSBMSTB.DAT. Skill: espionage.
    DeathStarSabotage,
}

impl MissionKind {
    /// DAT file stem for this kind's MSTB table, ids `0x14..0x1d` of
    /// `FUN_0058b420` (`ghidra/notes/uprising-incident.md`). `None` for
    /// `Autoscrap`, which has none.
    #[must_use]
    pub fn mstb_key(self) -> Option<&'static str> {
        match self {
            MissionKind::Diplomacy => Some("DIPLMSTB"),
            MissionKind::Recruitment => Some("RCRTMSTB"),
            MissionKind::Sabotage => Some("SBTGMSTB"),
            MissionKind::Assassination => Some("ASSNMSTB"),
            MissionKind::Espionage => Some("ESPIMSTB"),
            MissionKind::Rescue => Some("RESCMSTB"),
            MissionKind::Abduction => Some("ABDCMSTB"),
            MissionKind::InciteUprising => Some("INCTMSTB"),
            MissionKind::SubdueUprising => Some("SUBDMSTB"),
            MissionKind::DeathStarSabotage => Some("DSSBMSTB"),
            MissionKind::Autoscrap => None,
        }
    }

    /// Mission duration range in game-days: (`min_ticks`, `max_ticks`).
    ///
    /// These ranges are not the original's: a mission's timer is its MISSNSD
    /// record's minimum plus `rand(0..=spread)` (`FUN_005236e0`; Diplomacy is
    /// 5 and 10). port: they time only a kind with no loaded record.
    #[must_use]
    pub fn tick_range(self) -> (u32, u32) {
        match self {
            MissionKind::Diplomacy | MissionKind::Recruitment => (15, 20),
            MissionKind::Sabotage
            | MissionKind::Rescue
            | MissionKind::InciteUprising
            | MissionKind::SubdueUprising => (20, 30),
            MissionKind::Assassination | MissionKind::Abduction => (25, 35),
            MissionKind::Espionage => (15, 25),
            MissionKind::DeathStarSabotage => (30, 40),
            MissionKind::Autoscrap => (1, 1),
        }
    }
}

// ---------------------------------------------------------------------------
// MissionFaction
// ---------------------------------------------------------------------------

/// Which faction is sending the mission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissionFaction {
    Alliance,
    Empire,
}

impl From<MissionFaction> for crate::dat::Faction {
    fn from(faction: MissionFaction) -> Self {
        match faction {
            MissionFaction::Alliance => Self::Alliance,
            MissionFaction::Empire => Self::Empire,
        }
    }
}

// ---------------------------------------------------------------------------
// ActiveMission
// ---------------------------------------------------------------------------

/// One mission member: a character or a special-forces unit. The original
/// accepts DatId families `0x30..0x3f` (`FUN_00522b30`,
/// `ghidra/notes/decoy-roll.md`, "Adding members").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MissionMember {
    Character(CharacterKey),
    SpecialForce(SpecialForceKey),
}

impl MissionMember {
    /// The member's character key, when it is a character.
    #[must_use]
    pub fn character(self) -> Option<CharacterKey> {
        match self {
            Self::Character(key) => Some(key),
            Self::SpecialForce(_) => None,
        }
    }
}

/// A mission currently in progress.
///
/// Members sit in three lists, as in the original mission object: the team
/// (`+0x84`), the decoys (`+0x8c`), and the captured (`+0x94`)
/// (`ghidra/notes/decoy-roll.md`, "Mission members").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveMission {
    /// Unique identifier within `MissionState` (sequential, never reused).
    pub id: u64,
    /// What kind of mission this is.
    pub kind: MissionKind,
    /// Which faction dispatched this mission.
    pub faction: MissionFaction,
    /// Members who act on the mission and roll for success.
    pub team: Vec<MissionMember>,
    /// Members who draw defenders away and never roll for success.
    pub decoys: Vec<MissionMember>,
    /// Prisoners among the requested members (`FUN_0054bb90`).
    pub captured: Vec<MissionMember>,
    /// The target system.
    pub target_system: SystemKey,
    /// The target character (for assassination, abduction, rescue). None for area missions.
    pub target_character: Option<CharacterKey>,
    /// The object a Sabotage or DS Sabotage mission names (order `+0x4c`,
    /// read back by `FUN_00521030`).
    pub target_object: Option<MissionTarget>,
    /// Phase `+0x68`, `0..=0xb`, advanced by the stepper `FUN_005227d0`
    /// (`ghidra/notes/mission-lifecycle.md`, "Phase stepping").
    pub phase: u8,
    /// Ready bit `+0xa4` bit 0: the next step may run.
    pub ready: bool,
    /// End code `+0x64`; 0 while the mission runs. Any other value sends the
    /// next step to phase `0xb`.
    pub end_code: u8,
    /// The day the order was given; the creation steps run from it.
    pub ordered_tick: u64,
    /// Where the lead member stood when phase 4 started (`+0x6c`); phases 5
    /// and 6 act only when it differs from the target (`FUN_00520bb0`).
    pub origin: Option<SystemKey>,
    /// Travel speed phase 2 gave the character members (`+0x9a`,
    /// `FUN_00548370`).
    pub speed: i64,
    /// The day the current wait (transit or timer) began.
    pub wait_start: u64,
    /// The day timer `0x38b` fires while the mission waits in phase 8.
    pub timer_due: Option<u64>,
    /// Whether the destruction check (`FUN_00545240`) has run for the
    /// destroyed container. The original runs it once, on the destruction.
    pub container_loss_seen: bool,
    /// Members with a resign request (role `+0x78` bit 3, `FUN_00534640`),
    /// set when an exposed member may resign (record column 9).
    pub resigning: Vec<MissionMember>,
}

/// The phase that starts every member's transit (`FUN_00520b20`).
pub const PHASE_TRANSIT: u8 = 4;
/// The phase that waits for timer `0x38b` (`FUN_00520b30`).
pub const PHASE_TIMER: u8 = 8;
/// The phase whose entry rolls for success (`FUN_00592f50`).
pub const PHASE_OUTCOME: u8 = 10;
/// The last phase (`FUN_005227d0`).
pub const PHASE_END: u8 = 0xb;

impl ActiveMission {
    /// A new mission in phase 0 with its ready bit set, as creation leaves it
    /// (`FUN_0054bac0` -> `FUN_00522130(this, 1)`).
    #[must_use]
    pub fn new(
        id: u64,
        kind: MissionKind,
        faction: MissionFaction,
        team: Vec<MissionMember>,
        target_system: SystemKey,
        ordered_tick: u64,
    ) -> Self {
        ActiveMission {
            id,
            kind,
            faction,
            team,
            decoys: Vec::new(),
            captured: Vec::new(),
            target_system,
            target_character: None,
            target_object: None,
            phase: 0,
            ready: true,
            end_code: 0,
            ordered_tick,
            origin: None,
            speed: 100,
            wait_start: ordered_tick,
            timer_due: None,
            container_loss_seen: false,
            resigning: Vec::new(),
        }
    }

    /// A mission whose members have arrived and whose timer fires on
    /// `due_tick` (phase 8, waiting).
    #[must_use]
    pub fn timed(
        id: u64,
        kind: MissionKind,
        faction: MissionFaction,
        team: Vec<MissionMember>,
        target_system: SystemKey,
        due_tick: u64,
    ) -> Self {
        let mut mission = Self::new(id, kind, faction, team, target_system, 0);
        mission.phase = PHASE_TIMER;
        mission.ready = false;
        mission.origin = Some(target_system);
        mission.timer_due = Some(due_tick);
        mission
    }

    /// Every member in list order: team, decoys, captured (`FUN_00525bb0`).
    pub fn members(&self) -> impl Iterator<Item = MissionMember> + '_ {
        self.team
            .iter()
            .chain(&self.decoys)
            .chain(&self.captured)
            .copied()
    }

    /// The first team character. The resolver rolls for this one member
    /// until the per-member roll lands (port: interim, F-019 phase 4).
    #[must_use]
    pub fn lead_character(&self) -> Option<CharacterKey> {
        self.team.iter().find_map(|member| member.character())
    }

    /// Whether the mission waits on its members' transit (phase 4).
    #[must_use]
    pub fn in_transit(&self) -> bool {
        self.phase == PHASE_TRANSIT
    }

    /// Days until the current wait ends: the timer in phase 8, the last
    /// arrival in phase 4 (from `arrivals`), else none.
    #[must_use]
    pub fn days_left(&self, now: u64, last_arrival: Option<u64>) -> Option<u64> {
        let due = match self.phase {
            PHASE_TIMER => self.timer_due,
            PHASE_TRANSIT => last_arrival,
            _ => None,
        }?;
        Some(due.saturating_sub(now))
    }

    /// Progress through the current wait, in [0.0, 1.0].
    #[must_use]
    #[expect(
        clippy::cast_precision_loss,
        reason = "a display fraction over day counts far below 2^52"
    )]
    pub fn wait_fraction(&self, now: u64, last_arrival: Option<u64>) -> f32 {
        let due = match self.phase {
            PHASE_TIMER => self.timer_due,
            PHASE_TRANSIT => last_arrival,
            _ => None,
        };
        match due {
            Some(due) if due > self.wait_start => {
                let done = now
                    .saturating_sub(self.wait_start)
                    .min(due - self.wait_start);
                done as f32 / (due - self.wait_start) as f32
            }
            _ => 1.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Target objects
// ---------------------------------------------------------------------------

/// The object a Sabotage or DS Sabotage mission names. Sabotage refuses the
/// Death Star (families `0x18..0x1c`) and characters (`0x30..0x3c`,
/// `FUN_0056a110`); DS Sabotage needs the Death Star (`FUN_005744c0`).
///
/// port: capital ships other than the Death Star and fighter squadrons have
/// no identity in the port, so they cannot be named yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissionTarget {
    DefenseFacility(crate::ids::DefenseFacilityKey),
    ManufacturingFacility(crate::ids::ManufacturingFacilityKey),
    ProductionFacility(crate::ids::ProductionFacilityKey),
    Troop(crate::ids::TroopKey),
    SpecialForce(crate::ids::SpecialForceKey),
    /// The Death Star hull (class `0x88`, family `0x18`) in this fleet.
    DeathStar(crate::ids::FleetKey),
}

/// What the validator reads about a target object.
struct ObjectState {
    side: crate::dat::Faction,
    location: Option<SystemKey>,
}

impl MissionTarget {
    /// The side holding the object and where it stands, or `None` once it
    /// is gone.
    fn state(self, world: &GameWorld) -> Option<ObjectState> {
        let faction_of = crate::mission_detection::faction_of;
        let at = |listed: &dyn Fn(&crate::world::System) -> bool| {
            world
                .systems
                .iter()
                .find(|(_, sys)| listed(sys))
                .map(|(key, _)| key)
        };
        let (side, location) = match self {
            MissionTarget::DefenseFacility(key) => (
                world.defense_facilities.get(key)?.side,
                at(&|sys| sys.defense_facilities.contains(&key)),
            ),
            MissionTarget::ManufacturingFacility(key) => (
                world.manufacturing_facilities.get(key)?.side,
                at(&|sys| sys.manufacturing_facilities.contains(&key)),
            ),
            MissionTarget::ProductionFacility(key) => (
                world.production_facilities.get(key)?.side,
                at(&|sys| sys.production_facilities.contains(&key)),
            ),
            MissionTarget::Troop(key) => (
                faction_of(world.troops.get(key)?.is_alliance),
                at(&|sys| sys.ground_units.contains(&key)),
            ),
            MissionTarget::SpecialForce(key) => (
                faction_of(world.special_forces.get(key)?.is_alliance),
                at(&|sys| sys.special_forces.contains(&key)),
            ),
            MissionTarget::DeathStar(key) => {
                let fleet = world.fleets.get(key)?;
                fleet.capital_ships.iter().find(|ship| {
                    ship.alive
                        && world
                            .capital_ship_classes
                            .get(ship.class)
                            .is_some_and(crate::world::CapitalShipClass::is_death_star)
                })?;
                (faction_of(fleet.is_alliance), Some(fleet.location))
            }
        };
        Some(ObjectState {
            side,
            location,
        })
    }
}

/// Destroy a sabotaged target (slot `+0xac(6)`, `FUN_005746e0`).
///
/// port: an emptied fleet keeps its slot until the fleet cleanup removes
/// it; a regiment aboard a fleet is not at its container, so the validator
/// ends the mission before it can be named here.
pub fn destroy_target(world: &mut GameWorld, target: MissionTarget) {
    match target {
        MissionTarget::DefenseFacility(key) => {
            for (_, sys) in &mut world.systems {
                sys.defense_facilities.retain(|&k| k != key);
            }
            world.defense_facilities.remove(key);
        }
        MissionTarget::ManufacturingFacility(key) => {
            for (_, sys) in &mut world.systems {
                sys.manufacturing_facilities.retain(|&k| k != key);
            }
            world.manufacturing_facilities.remove(key);
        }
        MissionTarget::ProductionFacility(key) => {
            for (_, sys) in &mut world.systems {
                sys.production_facilities.retain(|&k| k != key);
            }
            world.production_facilities.remove(key);
        }
        MissionTarget::Troop(key) => {
            for (_, sys) in &mut world.systems {
                sys.ground_units.retain(|&k| k != key);
            }
            world.troops.remove(key);
        }
        MissionTarget::SpecialForce(key) => destroy_special_force(world, key),
        MissionTarget::DeathStar(key) => {
            let classes = &world.capital_ship_classes;
            if let Some(fleet) = world.fleets.get_mut(key) {
                if let Some(hull) = fleet.capital_ships.iter_mut().find(|ship| {
                    ship.alive
                        && classes
                            .get(ship.class)
                            .is_some_and(crate::world::CapitalShipClass::is_death_star)
                }) {
                    hull.alive = false;
                }
                fleet.capital_ships.retain(|ship| ship.alive);
                fleet.has_death_star = false;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// MissionState
// ---------------------------------------------------------------------------

/// A request for a new mission, the port's mission order (`0x240`..`0x242`,
/// `ghidra/notes/ai-mission-planning.md`): the chosen team and decoys, the
/// mission kind, and its target.
#[derive(Debug, Clone, PartialEq)]
pub struct MissionRequest {
    pub kind: MissionKind,
    pub faction: MissionFaction,
    pub team: Vec<MissionMember>,
    pub decoys: Vec<MissionMember>,
    pub target_system: SystemKey,
    pub target_character: Option<CharacterKey>,
    /// The object a Sabotage or DS Sabotage order names (`+0x4c`).
    pub target_object: Option<MissionTarget>,
    /// The day the order is given.
    pub tick: u64,
}

impl MissionRequest {
    /// A request with one character on the team and no decoys.
    #[must_use]
    pub fn single(
        kind: MissionKind,
        faction: MissionFaction,
        character: CharacterKey,
        target_system: SystemKey,
        target_character: Option<CharacterKey>,
        tick: u64,
    ) -> Self {
        Self {
            kind,
            faction,
            team: vec![MissionMember::Character(character)],
            decoys: Vec::new(),
            target_system,
            target_character,
            target_object: None,
            tick,
        }
    }
}

/// Why [`MissionState::dispatch_guarded`] refused a request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissionRefusal {
    /// No free member remained on the team (`FUN_0054bb90`, `0x40`/`0x91`).
    EmptyTeam,
    /// A member is missing, on the other side, already on a mission, or
    /// elsewhere than the other members (`FUN_00522b30`).
    MemberUnavailable(MissionMember),
    /// Sabotage names no object or the Death Star (`FUN_0056a110`, `0x40`/
    /// `0x28`), or DS Sabotage names no Death Star (`FUN_005744c0`,
    /// `0x40`/`0x29`).
    TargetUnavailable,
    /// The record does not admit these members: a special force or
    /// character the record does not take, or a side it does not run for
    /// (`FUN_00583320`, `0x40`/`0x01`).
    MembersNotAllowed,
}

/// What the member checks read about one member.
struct MemberState {
    is_alliance: bool,
    /// A character the side has recruited; special forces always are
    /// (`+0x50 & 4`, `FUN_004f9860`).
    recruited: bool,
    busy: bool,
    prisoner: bool,
    location: MemberLocation,
}

/// Where a member is: a fleet, else a system (the original compares each
/// member's location key, slot `+0xc`, in `FUN_00522b30`).
///
/// port: `FUN_00522b30` also compares the en route bit (`+0x50` bit 5) and
/// the arrival tick (`+0x44`); the port has no en route members yet, so
/// members with no location match each other.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MemberLocation {
    Fleet(crate::ids::FleetKey),
    System(SystemKey),
    Nowhere,
}

fn member_state(world: &GameWorld, member: MissionMember) -> Option<MemberState> {
    match member {
        MissionMember::Character(key) => {
            // port: the original deletes a killed character; the port keeps
            // it in the arena (`Character::mark_killed`), so it counts as gone.
            let c = world.characters.get(key).filter(|c| !c.is_killed)?;
            let location = match (c.current_fleet, c.current_system) {
                (Some(fleet), _) => MemberLocation::Fleet(fleet),
                (None, Some(system)) => MemberLocation::System(system),
                (None, None) => MemberLocation::Nowhere,
            };
            Some(MemberState {
                is_alliance: c.is_alliance,
                recruited: c.recruited,
                busy: c.on_mission || c.on_mandatory_mission,
                prisoner: c.is_captive,
                location,
            })
        }
        MissionMember::SpecialForce(key) => {
            let unit = world.special_forces.get(key)?;
            let location = world
                .systems
                .iter()
                .find(|(_, system)| system.special_forces.contains(&key))
                .map_or(MemberLocation::Nowhere, |(system, _)| {
                    MemberLocation::System(system)
                });
            Some(MemberState {
                is_alliance: unit.is_alliance,
                recruited: true,
                busy: unit.on_mission,
                prisoner: false,
                location,
            })
        }
    }
}

/// The mission bits a character contributes to the member mask
/// (`FUN_004ed260`).
const CHARACTER_MISSION_BIT: u32 = 0x1_0000;

/// Whether `members` may undertake a mission whose record admits `rules`
/// (`FUN_005830a0` builds the mask, `FUN_00583320` checks it). Every bit of
/// the special forces' SPECFCSD masks and of the characters' `0x10000` must
/// be allowed, and the members must all be of one side that the record
/// admits; no side at all fails too (status `0x16`). Skills and rank play no
/// part.
#[must_use]
pub fn members_allowed(
    world: &GameWorld,
    rules: crate::world::MissionMemberRules,
    members: impl IntoIterator<Item = MissionMember>,
) -> bool {
    let (mut special_forces, mut characters) = (0_u32, 0_u32);
    let (mut alliance, mut empire) = (false, false);
    for member in members {
        let is_alliance = match member {
            MissionMember::Character(key) => {
                let Some(c) = world.characters.get(key) else {
                    continue;
                };
                characters |= CHARACTER_MISSION_BIT;
                c.is_alliance
            }
            MissionMember::SpecialForce(key) => {
                let Some(unit) = world.special_forces.get(key) else {
                    continue;
                };
                // port: a unit whose class is missing contributes no bits.
                special_forces |= world
                    .special_force_classes
                    .get(&unit.class_dat_id)
                    .map_or(0, |class| class.mission_mask);
                unit.is_alliance
            }
        };
        if is_alliance {
            alliance = true;
        } else {
            empire = true;
        }
    }
    special_forces & rules.special_force_mask == special_forces
        && characters & rules.character_mask == characters
        && alliance != empire
        && (!alliance || rules.alliance)
        && (!empire || rules.empire)
}

/// The missions the player's dialog lists for a team, decoys, and target
/// system, in MISSNSD order (`FUN_004f5380` -> `FUN_005422f0`): every record
/// that is not hidden, whose member rules admit the members, and whose class
/// validator passes on a mission built from them (`FUN_0054c590` runs it on
/// a temporary mission, `FUN_00582b90`).
///
/// port: records outside the port's kinds are skipped (Recon, Research, and
/// Jedi Training are a separate finding), and so are kinds that need a
/// character or object target, since this target is a system; the dialog's
/// drop onto a character or object is F-019 phase 7.
#[must_use]
pub fn available_kinds(
    world: &GameWorld,
    uprisings: &crate::uprising::UprisingState,
    faction: MissionFaction,
    team: &[MissionMember],
    decoys: &[MissionMember],
    target_system: SystemKey,
) -> Vec<MissionKind> {
    world
        .mission_records
        .iter()
        .filter(|record| !record.hidden)
        .filter_map(|record| {
            crate::mission_planning::KINDS
                .into_iter()
                .find(|kind| kind.record_id() == Some(record.dat_id))
                .map(|kind| (kind, record))
        })
        .filter(|(kind, _)| kind.target_rules() == TargetRules::System)
        .filter(|(_, record)| {
            members_allowed(world, record.members, team.iter().chain(decoys).copied())
        })
        .filter(|(kind, _)| {
            let mut mission =
                ActiveMission::new(0, *kind, faction, team.to_vec(), target_system, 0);
            mission.decoys = decoys.to_vec();
            running_end_code(&mission, world, uprisings, &[]) == 0
        })
        .map(|(kind, _)| kind)
        .collect()
}

/// Whether two present members stand at the same location, as
/// `FUN_0054c200` requires of every member after the first free one.
pub(crate) fn members_together(world: &GameWorld, a: MissionMember, b: MissionMember) -> bool {
    match (member_state(world, a), member_state(world, b)) {
        (Some(a), Some(b)) => a.location == b.location,
        _ => false,
    }
}

/// Whether a present member stands in a fleet (`Some(true)`) or at a system
/// (`Some(false)`); `None` without a location.
pub(crate) fn member_location_kind(world: &GameWorld, member: MissionMember) -> Option<bool> {
    match member_state(world, member)?.location {
        MemberLocation::Fleet(_) => Some(true),
        MemberLocation::System(_) => Some(false),
        MemberLocation::Nowhere => None,
    }
}

/// A member's skill: a character's rolled template (base plus half of any
/// unrolled variance, as `MissionKind::skill_score` reads it), or a special
/// force's rolled skill. `None` when the member no longer exists.
#[must_use]
pub fn member_skill(world: &GameWorld, member: MissionMember, skill: Skill) -> Option<u32> {
    match member {
        MissionMember::Character(key) => world.characters.get(key).map(|c| {
            let pair = c.skill(skill);
            // port: seeding rolls every character's variance into its base
            // (FUN_00535e40); half of an unrolled variance matches skill_score.
            pair.base + pair.variance / 2
        }),
        MissionMember::SpecialForce(key) => world
            .special_forces
            .get(key)
            .map(|unit| unit.skills[skill as usize]),
    }
}

/// Slot `+0x274`: `member`'s success chance in percent, the MSTB row
/// (`FUN_0053e240` -> `FUN_0055bed0`, a step lookup `FUN_00595090`) for the
/// class's input (`ghidra/notes/decoy-roll.md`, "The success roll"):
///
/// - Diplomacy (`FUN_00573ff0`, `FUN_0055c680`): `(stormtroopers - opposing
///   support) + diplomacy`.
/// - Espionage (`FUN_00573090`): espionage. Rescue (`FUN_0056ae30`): combat.
/// - Sabotage and DS Sabotage (`FUN_0056a2d0`, `FUN_00574600`):
///   `(espionage + combat) / 2`.
/// - Recruitment (`FUN_0056b7b0`): `leadership - opposing support`.
/// - Incite (`FUN_005719d0`): `(leadership - opposing support) -
///   stormtroopers`; Subdue (`FUN_00569b90`): `(stormtroopers - opposing
///   support) + leadership`.
/// - Abduction and Assassination (`FUN_00576e50`, `FUN_005765c0`): `combat -
///   target combat`.
///
/// The opposing support is `FUN_00507270` for side `2 - (side != 1)`, and
/// the stormtroopers are `FUN_005091f0`. A missing table, row, target system
/// (`FUN_00586720`), or target character (`FUN_00586c80`) gives 0.
#[must_use]
pub fn member_chance(
    world: &GameWorld,
    kind: MissionKind,
    faction: MissionFaction,
    target_system: SystemKey,
    target_character: Option<CharacterKey>,
    member: MissionMember,
) -> i32 {
    let skill = |skill| {
        member_skill(world, member, skill).map_or(0, |value| i32::try_from(value).unwrap_or(0))
    };
    let side = crate::dat::Faction::from(faction);
    let opponent = if side == crate::dat::Faction::Alliance {
        crate::dat::Faction::Empire
    } else {
        crate::dat::Faction::Alliance
    };
    let system = world.systems.get(target_system);
    let support = |sys| crate::uprising::support_points(sys, opponent);
    let stormtroopers = |sys| crate::uprising::stormtroopers(world, sys);
    let target_combat = || {
        target_character
            .filter(|key| world.characters.contains_key(*key))
            .map(|key| skill_of(world, key, Skill::Combat))
    };
    let input = match kind {
        MissionKind::Diplomacy => {
            system.map(|sys| stormtroopers(sys) - support(sys) + skill(Skill::Diplomacy))
        }
        MissionKind::Espionage => Some(skill(Skill::Espionage)),
        MissionKind::Rescue => Some(skill(Skill::Combat)),
        MissionKind::Sabotage | MissionKind::DeathStarSabotage => {
            Some((skill(Skill::Espionage) + skill(Skill::Combat)) / 2)
        }
        MissionKind::Recruitment => system.map(|sys| skill(Skill::Leadership) - support(sys)),
        MissionKind::InciteUprising => {
            system.map(|sys| skill(Skill::Leadership) - support(sys) - stormtroopers(sys))
        }
        MissionKind::SubdueUprising => {
            system.map(|sys| stormtroopers(sys) - support(sys) + skill(Skill::Leadership))
        }
        MissionKind::Abduction | MissionKind::Assassination => {
            target_combat().map(|target| skill(Skill::Combat) - target)
        }
        // Autoscrap is no MSTB mission; it always succeeds.
        MissionKind::Autoscrap => return 100,
    };
    let (Some(input), Some(table)) = (
        input,
        kind.mstb_key()
            .and_then(|key| world.mission_tables.get(key)),
    ) else {
        return 0;
    };
    i32::try_from(table.step_lookup(input)).unwrap_or(i32::MAX)
}

/// A character's skill as [`member_skill`] reads it.
fn skill_of(world: &GameWorld, character: CharacterKey, skill: Skill) -> i32 {
    member_skill(world, MissionMember::Character(character), skill)
        .map_or(0, |value| i32::try_from(value).unwrap_or(0))
}

/// Set or clear a member's on-mission role flag. Clearing it also clears a
/// character's hidden-mission flag.
///
/// The original's leave path `FUN_00536220` recomputes OnMission (bit 7,
/// `FUN_00534950` → `FUN_00534800`) and clears bits 0 and 2..5; where it
/// clears OnHiddenMission (bit 8) is untraced. port: bit 8 clears with bit 7.
pub fn set_on_mission(world: &mut GameWorld, member: MissionMember, on_mission: bool) {
    match member {
        MissionMember::Character(key) => {
            if let Some(c) = world.characters.get_mut(key) {
                c.on_mission = on_mission;
                if !on_mission {
                    c.on_hidden_mission = false;
                }
            }
        }
        MissionMember::SpecialForce(key) => {
            if let Some(unit) = world.special_forces.get_mut(key) {
                unit.on_mission = on_mission;
            }
        }
    }
}

/// Apply a [`MissionEffect::MemberMoved`]: take the member out of its fleet
/// or system, then put it at `to`, or leave it off the map while it travels.
/// A killed character stays where `mark_killed` left it.
pub fn move_member(world: &mut GameWorld, member: MissionMember, to: Option<SystemKey>) {
    match member {
        MissionMember::Character(key) => {
            if world.characters.get(key).is_none_or(|c| c.is_killed) {
                return;
            }
            for (_, fleet) in &mut world.fleets {
                fleet.characters.retain(|&c| c != key);
            }
            let character = &mut world.characters[key];
            character.current_fleet = None;
            character.current_system = to;
        }
        MissionMember::SpecialForce(key) => {
            if !world.special_forces.contains_key(key) {
                return;
            }
            for (_, system) in &mut world.systems {
                system.special_forces.retain(|&unit| unit != key);
            }
            if let Some(system) = to.and_then(|to| world.systems.get_mut(to)) {
                system.special_forces.push(key);
            }
        }
    }
}

/// Apply [`MissionAdvance::effects`] in order: member moves and releases,
/// and the detection's captures, lost special forces, and injuries. The
/// native and browser app shares this with the headless integrator.
pub fn apply_advance_effects(world: &mut GameWorld, effects: &[MissionEffect]) {
    for effect in effects {
        match effect {
            MissionEffect::MemberMoved { member, to } => move_member(world, *member, *to),
            MissionEffect::MemberAvailable { member } => set_on_mission(world, *member, false),
            MissionEffect::CharacterCaptured {
                character,
                captured_by,
                at_system,
            } => capture_character(world, *character, *captured_by, *at_system),
            MissionEffect::SpecialForceDestroyed { unit } => destroy_special_force(world, *unit),
            _ => {}
        }
    }
}

/// Raise `character`'s base `skill` by `amount` ([`MissionEffect::SkillRaised`]).
/// The setters (`FUN_00533e20`..`FUN_005341a0`) take the new short as given.
pub fn raise_skill(world: &mut GameWorld, character: CharacterKey, skill: Skill, amount: i32) {
    if let Some(c) = world.characters.get_mut(character) {
        let pair = c.skill_mut(skill);
        pair.base = pair.base.saturating_add_signed(amount);
    }
}

/// `FUN_0055ef30`'s pool: `side`'s living minor characters (families
/// `0x38..0x3c`, MNCHARSD, `FUN_0056f450`) it has not recruited (`+0x50` bit
/// 1 clear). port: the original walks its character container
/// (`FUN_00506e20`); the port walks them in `DatId` order.
pub(crate) fn recruit_pool(world: &GameWorld, side: crate::dat::Faction) -> Vec<CharacterKey> {
    let mut pool: Vec<(crate::ids::DatId, CharacterKey)> = world
        .characters
        .iter()
        .filter(|(_, c)| {
            !c.is_major && !c.recruited && !c.is_killed && character_side(c) == Some(side)
        })
        .map(|(key, c)| (c.dat_id, key))
        .collect();
    pool.sort_by_key(|&(dat_id, _)| dat_id.raw());
    pool.into_iter().map(|(_, key)| key).collect()
}

/// Apply [`MissionEffect::CharacterRecruited`]: `FUN_0055fe70` places the
/// recruit at `system` (slot `+0xa8`) and sets `+0x50` bits 1 and 2
/// (`FUN_004f7480`, `FUN_004f74f0`); taking the last one sets the side's
/// `+0xb8` (`FUN_0052f590`). port: bit 2 is not modelled.
pub fn recruit_character(
    world: &mut GameWorld,
    character: CharacterKey,
    system: SystemKey,
    faction: MissionFaction,
    pool_emptied: bool,
) {
    if let Some(c) = world.characters.get_mut(character) {
        c.recruited = true;
        c.current_system = Some(system);
        c.current_fleet = None;
    }
    if pool_emptied {
        world.set_recruit_pool_empty(faction.into());
    }
}

/// Hold `character` at `at_system` as `captured_by`'s prisoner.
pub fn capture_character(
    world: &mut GameWorld,
    character: CharacterKey,
    captured_by: MissionFaction,
    at_system: SystemKey,
) {
    if let Some(c) = world.characters.get_mut(character) {
        c.is_captive = true;
        c.captured_by = Some(captured_by.into());
        c.current_system = Some(at_system);
        c.current_fleet = None;
    }
    for (_, fleet) in &mut world.fleets {
        fleet.characters.retain(|&k| k != character);
    }
}

/// Remove a special force from its system and the world.
pub fn destroy_special_force(world: &mut GameWorld, unit: crate::ids::SpecialForceKey) {
    for (_, system) in &mut world.systems {
        system.special_forces.retain(|&key| key != unit);
    }
    world.special_forces.remove(unit);
}

/// One mission member travelling to its mission's target, as the original's
/// en route member (`+0x50` bit 5, arrival tick `+0x44`; `FUN_00556430`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemberTransit {
    pub member: MissionMember,
    /// The mission that sent it.
    pub mission_id: u64,
    /// Its destination, the mission's target container `+0x74`.
    pub to: SystemKey,
    /// The day it arrives.
    pub arrival: u64,
}

/// All active missions across the galaxy.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MissionState {
    missions: VecDeque<ActiveMission>,
    next_id: u64,
    /// Members on their way to a target. A member keeps travelling when its
    /// mission ends, as its own transit timer does in the original.
    en_route: Vec<MemberTransit>,
}

impl MissionState {
    #[must_use]
    pub fn new() -> Self {
        MissionState {
            missions: VecDeque::new(),
            next_id: 0,
            en_route: Vec::new(),
        }
    }

    /// Members travelling to a mission target.
    #[must_use]
    pub fn en_route(&self) -> &[MemberTransit] {
        &self.en_route
    }

    /// The last arrival day among a mission's travelling members.
    #[must_use]
    pub fn last_arrival(&self, mission_id: u64) -> Option<u64> {
        self.en_route
            .iter()
            .filter(|transit| transit.mission_id == mission_id)
            .map(|transit| transit.arrival)
            .max()
    }

    /// Whether `member` is travelling.
    #[must_use]
    pub fn is_en_route(&self, member: MissionMember) -> bool {
        self.en_route.iter().any(|transit| transit.member == member)
    }

    /// Put `transit` on the road, for other modules' tests.
    #[cfg(test)]
    pub(crate) fn push_transit(&mut self, transit: MemberTransit) {
        self.en_route.push(transit);
    }

    /// Dispatch a new mission and return its assigned id. Members are taken
    /// as given; [`MissionState::dispatch_guarded`] applies the original's
    /// member rules. The mission steps from phase 0 on the next advance.
    pub fn dispatch(&mut self, request: MissionRequest) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let mut mission = ActiveMission::new(
            id,
            request.kind,
            request.faction,
            request.team,
            request.target_system,
            request.tick,
        );
        mission.decoys = request.decoys;
        mission.target_character = request.target_character;
        mission.target_object = request.target_object;
        self.missions.push_back(mission);
        id
    }

    /// Dispatch a mission after the original's member checks, and mark every
    /// member on a mission.
    ///
    /// `FUN_0054bb90` moves prisoners from either list to the captured list
    /// and refuses an empty team (message `0x40`/`0x91`). `FUN_00522b30` then
    /// accepts each member only if it is on the mission's side, is not
    /// already a member, has no mission yet (`RoleOnMissionNotif`
    /// `FUN_00536b00`; a mandatory mission counts, `FUN_00536b80`), and
    /// shares the location of the members already accepted
    /// (`ghidra/notes/decoy-roll.md`, "Adding members"). Completion clears
    /// the flags through `MemberAvailable`.
    pub fn dispatch_guarded(
        &mut self,
        request: MissionRequest,
        world: &mut GameWorld,
    ) -> Result<u64, MissionRefusal> {
        let side_is_alliance = request.faction == MissionFaction::Alliance;
        // The creation checks of FUN_0056a110 and FUN_005744c0 read the
        // target object's family (slot +4). port: a missing object is
        // refused, because every original order names one.
        let death_star = request
            .target_object
            .map(|target| matches!(target, MissionTarget::DeathStar(_)));
        let target_ok = match request.kind {
            MissionKind::Sabotage => death_star == Some(false),
            MissionKind::DeathStarSabotage => death_star == Some(true),
            _ => true,
        };
        if !target_ok {
            return Err(MissionRefusal::TargetUnavailable);
        }
        // FUN_0054bb90 moves every prisoner out of the team and decoy lists
        // into the captured list first. FUN_0054c200 then adds the team, the
        // decoys, and the captured, in that order, so the first free team
        // member sets the location the others must share.
        let mut requested: [Vec<(MissionMember, MemberState)>; 3] = Default::default();
        for (member, list) in request
            .team
            .iter()
            .map(|m| (*m, 0))
            .chain(request.decoys.iter().map(|m| (*m, 1)))
        {
            let state =
                member_state(world, member).ok_or(MissionRefusal::MemberUnavailable(member))?;
            requested[if state.prisoner { 2 } else { list }].push((member, state));
        }
        // port: one refused member refuses the whole request. FUN_0054c200
        // keeps adding after a refusal and returns false; what its caller
        // does with that is untraced.
        let mut lists: [Vec<MissionMember>; 3] = Default::default();
        let mut location = None;
        for (index, members) in requested.iter().enumerate() {
            for &(member, ref state) in members {
                if lists.iter().any(|list| list.contains(&member)) {
                    continue;
                }
                // Saves written before dispatch set the flag can hold an
                // active mission for a member whose flag is still false.
                // port: a member still travelling to an ended mission's
                // target is refused; the original compares its en route bit
                // and arrival tick instead (FUN_00522b30).
                if state.is_alliance != side_is_alliance || state.busy || self.engaged(member) {
                    return Err(MissionRefusal::MemberUnavailable(member));
                }
                match location {
                    None => location = Some(state.location),
                    Some(here) if here != state.location => {
                        return Err(MissionRefusal::MemberUnavailable(member));
                    }
                    Some(_) => {}
                }
                lists[index].push(member);
            }
        }
        let [team, decoys, captured] = lists;
        if team.is_empty() {
            return Err(MissionRefusal::EmptyTeam);
        }
        // FUN_005420d0 checks the mask of all three lists (FUN_00583320).
        // port: a kind with no MISSNSD record admits any members.
        if let Some(record) = request
            .kind
            .record_id()
            .and_then(|id| world.mission_record(id))
        {
            let members = team.iter().chain(&decoys).chain(&captured).copied();
            if !members_allowed(world, record.members, members) {
                return Err(MissionRefusal::MembersNotAllowed);
            }
        }
        for member in team.iter().chain(&decoys).chain(&captured) {
            set_on_mission(world, *member, true);
        }
        let id = self.dispatch(MissionRequest {
            team,
            decoys,
            ..request
        });
        if let Some(mission) = self.missions.back_mut() {
            mission.captured = captured;
        }
        Ok(id)
    }

    /// Whether `member` belongs to a queued mission or is still travelling
    /// to one.
    fn engaged(&self, member: MissionMember) -> bool {
        self.missions
            .iter()
            .any(|mission| mission.members().any(|x| x == member))
            || self.is_en_route(member)
    }

    /// Whether the object pop-up menu enables Mission (order `0x240`) for
    /// `team`. No target or kind is checked yet. `FUN_0051fe20` builds a
    /// mission-create command from the team (`FUN_004f4a00`), and
    /// `FUN_005429e0` checks it:
    ///
    /// - every member is the player's (`FUN_004f9860`, status 1), recruited
    ///   (status 2), not on a mission (`FUN_00533ce0`), not travelling
    ///   (status 4), and not a prisoner (`FUN_004ed560`);
    /// - all located members share one place (`FUN_0054bf00`, `0x40`/3);
    /// - at least one member has a location (`0x16`). An empty team fails
    ///   the same way (`FUN_0054bb90`).
    ///
    /// port: `FUN_004ed560` also refuses an injured character (`+0x94`), and
    /// `FUN_004f9860` an object with `+0x50 & 8`; the port tracks neither.
    #[must_use]
    pub fn mission_order_enabled(
        &self,
        world: &GameWorld,
        faction: MissionFaction,
        team: &[MissionMember],
    ) -> bool {
        let side_is_alliance = faction == MissionFaction::Alliance;
        let mut place = None;
        for &member in team {
            let Some(state) = member_state(world, member) else {
                return false;
            };
            if state.is_alliance != side_is_alliance
                || !state.recruited
                || state.busy
                || state.prisoner
                || self.engaged(member)
            {
                return false;
            }
            if state.location == MemberLocation::Nowhere {
                continue;
            }
            match place {
                None => place = Some(state.location),
                Some(here) if here != state.location => return false,
                Some(_) => {}
            }
        }
        place.is_some()
    }

    /// Queue a mission built in a test, past the dispatch rules.
    #[cfg(test)]
    pub(crate) fn push_timed(&mut self, mission: ActiveMission) {
        self.next_id = self.next_id.max(mission.id + 1);
        self.missions.push_back(mission);
    }

    /// Cancel a mission by id and free its members for another assignment.
    pub fn release(&mut self, id: u64, world: &mut GameWorld) -> Option<ActiveMission> {
        let mission = self.cancel(id)?;
        for member in mission.members() {
            set_on_mission(world, member, false);
        }
        Some(mission)
    }

    /// Cancel a mission by id. Returns the mission if found, None otherwise.
    pub fn cancel(&mut self, id: u64) -> Option<ActiveMission> {
        if let Some(pos) = self.missions.iter().position(|m| m.id == id) {
            self.missions.remove(pos)
        } else {
            None
        }
    }

    /// All active missions (read-only).
    #[must_use]
    pub fn missions(&self) -> &VecDeque<ActiveMission> {
        &self.missions
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.missions.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.missions.is_empty()
    }
}

// ---------------------------------------------------------------------------
// MissionEffect
// ---------------------------------------------------------------------------

/// What actually changed in the game world as a result of a mission.
///
/// The caller in `main.rs` is responsible for applying these effects to
/// `GameWorld`. `MissionSystem` is stateless — it only produces the events.
#[derive(Debug, Clone, PartialEq)]
pub enum MissionEffect {
    // ── Living Galaxy ────────────────────────────────────────────────────────
    /// A Diplomacy success (`FUN_00574120`) changes `side`'s support at
    /// `system` by `points` through `FUN_0050c9f0`
    /// ([`crate::uprising::apply_support_change`]).
    SupportGained {
        system: SystemKey,
        side: crate::dat::Faction,
        points: i32,
    },
    /// A successful member's base skill rises by `amount` (GNPRTB
    /// 6156..6168, the skill setters `FUN_00533e20`..`FUN_005341a0`).
    SkillRaised {
        character: CharacterKey,
        skill: Skill,
        amount: i32,
    },
    /// A pool character joins `faction` at `system` ([`recruit_character`]).
    CharacterRecruited {
        character: CharacterKey,
        system: SystemKey,
        faction: MissionFaction,
        /// The pick took the side's last pool character (`FUN_0055fc80`).
        pool_emptied: bool,
    },

    // ── War Machine ──────────────────────────────────────────────────────────
    /// A Sabotage or DS Sabotage target at `system` is destroyed
    /// ([`destroy_target`]).
    TargetSabotaged {
        target: MissionTarget,
        system: SystemKey,
    },

    /// A character was killed — remove from the game.
    CharacterKilled {
        character: CharacterKey,
        faction: MissionFaction,
    },

    /// A character was captured by the opposing faction.
    ///
    /// The captured character is held at `at_system` until rescued or the game ends.
    CharacterCaptured {
        character: CharacterKey,
        /// Which faction now holds the character.
        captured_by: MissionFaction,
        at_system: SystemKey,
    },

    /// A captured character was successfully rescued.
    CharacterRescued {
        character: CharacterKey,
        /// Faction the character was returned to.
        returned_to: MissionFaction,
        at_system: SystemKey,
    },

    /// Intelligence gathered — reveal fog of war over the target system.
    SystemIntelligenceGathered {
        system: SystemKey,
        /// Faction that gained the intelligence.
        faction: MissionFaction,
    },

    /// An Incite Uprising member's success ran the uprising incident
    /// `FUN_0050d030` at the target (`FUN_00571a60`); apply it with
    /// [`crate::uprising::apply_uprising_event`].
    UprisingIncident(crate::uprising::UprisingEvent),

    // ── Member availability tracking ────────────────────────────────────────
    /// A member has left its mission and may take another
    /// (`RoleOnMissionNotif` `FUN_00536b00` cleared).
    MemberAvailable { member: MissionMember },
    /// A member moved for its mission: off the map when it starts a transit
    /// (`to: None`), into the target system when it lands (`FUN_00556390`).
    MemberMoved {
        member: MissionMember,
        to: Option<SystemKey>,
    },

    // ── Escape ──────────────────────────────────────────────────────────────
    /// A captured character escaped on their own.
    CharacterEscaped {
        character: CharacterKey,
        escaped_to_alliance: bool,
    },

    /// A Subdue Uprising succeeded (`FUN_00569c20`): `side` wins
    /// `support_gain` points (`FUN_0055cb10`), then the revolt ends if the
    /// system is garrisoned (`FUN_0050c910`).
    UprisingSubdued {
        system: SystemKey,
        side: crate::dat::Faction,
        support_gain: i32,
    },

    /// A special force captured on a mission is destroyed (slot `+0x20c`,
    /// `FUN_00503eb0` -> slot `+0xac(9)`).
    SpecialForceDestroyed { unit: crate::ids::SpecialForceKey },

    /// A character took the injury roll `FUN_004ef5f0` (health `+0x94`).
    /// port: the port stores no injury (F-026), so applying it changes
    /// nothing.
    CharacterInjured {
        character: CharacterKey,
        injury: i32,
    },
}

// ---------------------------------------------------------------------------
// MissionOutcome / MissionResult
// ---------------------------------------------------------------------------

/// Whether a mission's phase 10 succeeded: its result `+0x60` is 3
/// (`FUN_00521880`), else 2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissionOutcome {
    Success,
    Failure,
}

/// The result emitted when a mission resolves.
#[derive(Debug, Clone)]
pub struct MissionResult {
    /// The id of the mission that resolved.
    pub mission_id: u64,
    /// The game-day on which the mission resolved.
    pub tick: u64,
    pub kind: MissionKind,
    pub faction: MissionFaction,
    /// The first team character (`ActiveMission::lead_character`), named in
    /// the mission's messages.
    pub character: Option<CharacterKey>,
    pub target_system: SystemKey,
    pub outcome: MissionOutcome,
    /// World-state changes to apply, in the order the members acted.
    pub effects: Vec<MissionEffect>,
}

// ---------------------------------------------------------------------------
// The running validator
// ---------------------------------------------------------------------------

/// Han Solo's DatId (`FUN_00506f50`); his presence speeds a mission's
/// characters (`FUN_00548370`).
const HAN_SOLO: u32 = 0x3300_0243;
/// GNPRTB parameter 3083, the Han Solo mission speed (`FUN_005725a0`).
const GNPRTB_HAN_SOLO_SPEED: u16 = 3083;
/// Shipped GNPRTB.DAT value of parameter 3083, used only when no GNPRTB
/// table is loaded.
const SHIPPED_HAN_SOLO_SPEED: i64 = 50;
/// GNPRTB 6156..6168 (`FUN_0055bfd0`), each shipped 1: the base skill a
/// successful member gains.
const GNPRTB_DIPLOMACY_RAISE: u16 = 6156;
/// GNPRTB 6157 (`DAT_006bb534`): Espionage's espionage raise.
const GNPRTB_ESPIONAGE_RAISE: u16 = 6157;
/// GNPRTB 6159 (`DAT_006bb5b0`): Recruitment's leadership raise.
const GNPRTB_RECRUITMENT_RAISE: u16 = 6159;
/// GNPRTB 6160 (`DAT_006bb5a0`): Incite's leadership raise.
const GNPRTB_INCITE_RAISE: u16 = 6160;
/// GNPRTB 6161 (`DAT_006bb538`): Subdue's leadership raise.
const GNPRTB_SUBDUE_RAISE: u16 = 6161;
/// GNPRTB 6162 (`DAT_006bb55c`): Rescue's combat raise.
const GNPRTB_RESCUE_RAISE: u16 = 6162;
/// GNPRTB 6163 (`DAT_006bb574`): Abduction's combat raise.
const GNPRTB_ABDUCTION_RAISE: u16 = 6163;
/// GNPRTB 6164 (`DAT_006bb584`): Assassination's combat raise.
const GNPRTB_ASSASSINATION_RAISE: u16 = 6164;
/// GNPRTB 6165 (`DAT_006bb58c`): Sabotage's espionage raise.
const GNPRTB_SABOTAGE_ESPIONAGE_RAISE: u16 = 6165;
/// GNPRTB 6166 (`DAT_006bb518`): Sabotage's combat raise.
const GNPRTB_SABOTAGE_COMBAT_RAISE: u16 = 6166;
/// GNPRTB 6167 (`DAT_006bb5c0`): DS Sabotage's espionage raise.
const GNPRTB_DS_SABOTAGE_ESPIONAGE_RAISE: u16 = 6167;
/// GNPRTB 6168 (`DAT_006bb57c`): DS Sabotage's combat raise.
const GNPRTB_DS_SABOTAGE_COMBAT_RAISE: u16 = 6168;
/// The shipped value of every skill raise, GNPRTB 6156..6168.
const SHIPPED_SKILL_RAISE: i64 = 1;

/// A skill raise's GNPRTB value, or the shipped one without a table.
fn skill_raise(world: &GameWorld, id: u16) -> i32 {
    i32::try_from(crate::movement::gnprtb_or_shipped(
        world,
        id,
        SHIPPED_SKILL_RAISE,
    ))
    .unwrap_or(0)
}

/// Full support, the Diplomacy end threshold (`DAT_00661a88`, `FUN_00573ee0`).
const FULL_SUPPORT: i32 = 100;

/// Which rules the class validator adds to the base rules
/// (`ghidra/notes/mission-lifecycle.md`, "Validators per class").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TargetRules {
    /// `FUN_005868c0`: the uprising columns.
    System,
    /// `FUN_00586e20`: the prisoner columns.
    Character,
    /// `FUN_00593500`: the base rules only, while running.
    Object,
}

impl MissionKind {
    /// The kind's MISSNSD.DAT record id (`family << 24 | index`), `None`
    /// for the port-only `Autoscrap`. The families are the class slot `+4`
    /// codes (`ghidra/notes/mission-lifecycle.md`, "Slots per class").
    #[must_use]
    pub fn record_id(self) -> Option<crate::ids::DatId> {
        let raw = match self {
            MissionKind::Diplomacy => 0x5100_0010,
            MissionKind::Rescue => 0x6100_0011,
            MissionKind::Sabotage => 0x6900_0012,
            MissionKind::Espionage => 0x5200_0013,
            MissionKind::Recruitment => 0x5500_0016,
            MissionKind::Abduction => 0x6200_0017,
            MissionKind::InciteUprising => 0x5600_0040,
            MissionKind::DeathStarSabotage => 0x6a00_0041,
            MissionKind::SubdueUprising => 0x5700_0080,
            MissionKind::Assassination => 0x6300_0081,
            MissionKind::Autoscrap => return None,
        };
        Some(crate::ids::DatId::new(raw))
    }

    pub(crate) fn target_rules(self) -> TargetRules {
        match self {
            MissionKind::Rescue | MissionKind::Abduction | MissionKind::Assassination => {
                TargetRules::Character
            }
            MissionKind::Sabotage | MissionKind::DeathStarSabotage | MissionKind::Autoscrap => {
                TargetRules::Object
            }
            MissionKind::Diplomacy
            | MissionKind::Espionage
            | MissionKind::Recruitment
            | MissionKind::InciteUprising
            | MissionKind::SubdueUprising => TargetRules::System,
        }
    }
}

/// What the base rules read about a mission's target `T`.
struct TargetView {
    side: Option<crate::dat::Faction>,
    destroyed: bool,
    en_route: bool,
    location: Option<SystemKey>,
    prisoner: bool,
}

/// A character's location key: its fleet's system, else its own.
fn character_location(world: &GameWorld, character: &Character) -> Option<SystemKey> {
    match character.current_fleet {
        Some(fleet) => world.fleets.get(fleet).map(|f| f.location),
        None => character.current_system,
    }
}

/// A member's location key, as [`character_location`] reads a character.
fn member_location(world: &GameWorld, member: MissionMember) -> Option<SystemKey> {
    match member_state(world, member)?.location {
        MemberLocation::Fleet(fleet) => world.fleets.get(fleet).map(|f| f.location),
        MemberLocation::System(system) => Some(system),
        MemberLocation::Nowhere => None,
    }
}

/// The side a character reads as. Inference: a prisoner reads as its
/// captor's side (`ghidra/notes/mission-lifecycle.md`, "The running checks").
fn character_side(character: &Character) -> Option<crate::dat::Faction> {
    use crate::dat::Faction;
    if character.is_captive {
        character.captured_by
    } else if character.is_alliance {
        Some(Faction::Alliance)
    } else if character.is_empire {
        Some(Faction::Empire)
    } else {
        None
    }
}

/// The target the validator reads, `None` when it is missing (rule 1).
///
/// port: the target of a system kind is its target system, which is also
/// its container; an object kind reads its named object (a mission without
/// one reads as missing). A system reads as located in itself (`hyp:` the system
/// slot `+0xc` is untraced) and as its holder's side (inference).
fn target_view(
    mission: &ActiveMission,
    world: &GameWorld,
    en_route: &[MemberTransit],
) -> Option<TargetView> {
    match mission.kind.target_rules() {
        // FUN_00521030 reads the order's object. port: the original keeps a
        // destroyed object (+0x50 bit 3) for its rule 4; the port removes
        // it, so a removed object reads as the opponent's and destroyed.
        TargetRules::Object => {
            let object = mission.target_object?.state(world);
            let opponent = match crate::dat::Faction::from(mission.faction) {
                crate::dat::Faction::Alliance => crate::dat::Faction::Empire,
                _ => crate::dat::Faction::Alliance,
            };
            Some(TargetView {
                side: Some(object.as_ref().map_or(opponent, |object| object.side)),
                destroyed: object.is_none(),
                en_route: false,
                location: object.and_then(|object| object.location),
                prisoner: false,
            })
        }
        TargetRules::System => {
            let sys = world.systems.get(mission.target_system)?;
            Some(TargetView {
                side: crate::uprising::holder(sys),
                destroyed: sys.is_destroyed,
                en_route: false,
                location: Some(mission.target_system),
                prisoner: false,
            })
        }
        TargetRules::Character => {
            let key = mission.target_character?;
            let character = world.characters.get(key)?;
            Some(TargetView {
                side: character_side(character),
                destroyed: character.is_killed,
                // port: only a mission member's transit is visible here; a
                // target aboard a moving fleet reads its fleet's location.
                en_route: en_route
                    .iter()
                    .any(|transit| transit.member == MissionMember::Character(key)),
                location: character_location(world, character),
                prisoner: character.is_captive,
            })
        }
    }
}

/// The validator `FUN_00522480` over the running checks: the end code the
/// first failing rule sets, or 0 (`ghidra/notes/mission-lifecycle.md`, "The
/// validator and the end codes").
///
/// port: the validator reads the real container and target. Before phase 5
/// with a remote target (`FUN_00520af0` false), the original reads the
/// side's own copy of them instead (`FUN_00521160`, `FUN_005211c0` through
/// `FUN_004f2d10`); the port keeps no per-side copies.
pub(crate) fn running_end_code(
    mission: &ActiveMission,
    world: &GameWorld,
    uprisings: &crate::uprising::UprisingState,
    en_route: &[MemberTransit],
) -> u8 {
    use crate::dat::Faction;
    // Rule 1 (FUN_00522480.c:42-52): end 5 unless a team member stands
    // without a remove or resign request; an empty team ends too, and a
    // killed member is deleted. port: the port has no remove requests, and
    // +0xa4 bit 1, which lifts the rule, is untraced and taken as clear.
    let standing = mission.team.iter().any(|&member| {
        member_state(world, member).is_some() && !mission.resigning.contains(&member)
    });
    if !standing {
        return 5;
    }
    // port: a kind with no MISSNSD record runs no other checks.
    let Some(record) = mission
        .kind
        .record_id()
        .and_then(|id| world.mission_record(id))
    else {
        return 0;
    };
    // FUN_005830a0: members of both sides (status 0x14) or of none (0x16)
    // skip every later check without an end code. Rule 1 has already ended
    // a mission with no standing team member, so some member has a side.
    let sides: Vec<bool> = mission
        .members()
        .filter_map(|member| member_state(world, member).map(|state| state.is_alliance))
        .collect();
    if sides.contains(&true) && sides.contains(&false) {
        return 0;
    }
    let rules = record.rules;
    // FUN_00523450 rule 1: a missing target or container skips the rest.
    let Some(container) = world.systems.get(mission.target_system) else {
        return 0;
    };
    let Some(target) = target_view(mission, world, en_route) else {
        return 0;
    };
    // Rule 2 (col 11): the container is destroyed.
    if rules.container_loss_ends && container.is_destroyed {
        return 7;
    }
    // Rule 3 (cols 15..17): the target's side against the mission's.
    let side = Faction::from(mission.faction);
    let opponent = match side {
        Faction::Alliance => Faction::Empire,
        Faction::Empire | Faction::Neutral => Faction::Alliance,
    };
    let side_allowed = match target.side {
        Some(s) if s == side => rules.own_side_target,
        Some(s) if s == opponent => rules.opponent_target,
        _ => rules.other_side_target,
    };
    if !side_allowed {
        return 8;
    }
    // Rule 4 (col 14): the target is destroyed.
    if rules.target_loss_ends && target.destroyed {
        return 6;
    }
    // Rules 5 and 6: the target is travelling or has left the container.
    if target.en_route || target.location != Some(mission.target_system) {
        return 6;
    }
    // Rule 7 (col 13): the container must be a populated system.
    if rules.needs_populated_container && !container.is_populated {
        return 0xd;
    }
    match mission.kind.target_rules() {
        // FUN_005868c0 (cols 18, 19).
        TargetRules::System => {
            let revolting = uprisings
                .active_uprisings
                .contains_key(&mission.target_system);
            let allowed = if revolting {
                rules.revolting_target
            } else {
                rules.calm_target
            };
            if !allowed {
                return 6;
            }
        }
        // FUN_00586e20 (cols 20, 21).
        TargetRules::Character => {
            let allowed = if target.prisoner {
                rules.prisoner_target
            } else {
                rules.free_target
            };
            if !allowed {
                return 6;
            }
        }
        TargetRules::Object => {}
    }
    // FUN_00573ee0: Diplomacy stops at full support.
    if mission.kind == MissionKind::Diplomacy
        && crate::uprising::support_points(container, side) == FULL_SUPPORT
    {
        return 0xf;
    }
    // FUN_0056b370: Recruitment stops once its side recruited the last pool
    // character (side +0xb8). port: its +0xc test and the FUN_0056b680 and
    // FUN_0056b550 rules are not traced.
    if mission.kind == MissionKind::Recruitment && world.recruit_pool_empty(side) {
        return 0x10;
    }
    0
}

// ---------------------------------------------------------------------------
// MissionSystem
// ---------------------------------------------------------------------------

/// Stateless system that steps missions through their phases.
///
/// # RNG contract
///
/// The caller provides `rolls`: [`ROLLS_PER_MISSION`] per active mission per
/// tick event. Mission `i` (its position when the event starts) on event `e`
/// draws from the window starting at `(e * n + i) * ROLLS_PER_MISSION`, `n`
/// being the number of missions when the advance starts, so one mission's
/// draws never shift another's. A draw beyond the window, or beyond the
/// slice, reads `0.5`. Each roll is uniform [0, 1). port: the original
/// draws inline; the windows keep the port's draws deterministic per
/// mission.
pub struct MissionSystem;

/// Draws one mission may take on one tick event: the creation chain's timer
/// and the repeat's timer, then the seed of the mission's stream, which the
/// detection run and the phase 10 roll draw from. port: the size of the
/// port's per-mission window.
pub const ROLLS_PER_MISSION: usize = 3;

/// port: the window slot that seeds the mission's stream; the draws before
/// it are taken in order.
const STREAM_SEED_ROLL: usize = 2;

/// One mission's window of rolls for one tick event.
struct MissionRolls<'a> {
    rolls: &'a [f64],
    next: usize,
    stream: crate::mission_detection::MissionRng,
}

impl MissionRolls<'_> {
    /// port: a draw past the window reads 0.5, the port's under-supply
    /// fallback.
    fn next(&mut self) -> f64 {
        let roll = self
            .rolls
            .get(self.next)
            .filter(|_| self.next < STREAM_SEED_ROLL)
            .copied()
            .unwrap_or(0.5);
        self.next += 1;
        roll
    }
}

/// A mission that reached phase `0xb` and was removed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissionEnded {
    pub mission_id: u64,
    pub kind: MissionKind,
    pub faction: MissionFaction,
    /// The end code (`+0x64`): 1 for a mission that ran its course, else
    /// the validator's reason.
    pub end_code: u8,
    pub tick: u64,
}

/// What one [`MissionSystem::advance`] produced.
#[derive(Debug, Clone, Default)]
pub struct MissionAdvance {
    /// Phase 10 results, one per pass (a repeating mission may resolve
    /// again), in the order they resolved.
    pub results: Vec<MissionResult>,
    /// Member moves and releases in the order they happened. The caller
    /// applies them before `results`.
    pub effects: Vec<MissionEffect>,
    /// Missions that ended, in the order they ended.
    pub ended: Vec<MissionEnded>,
}

/// What a phase step reads besides the mission.
struct StepContext<'a> {
    world: &'a GameWorld,
    uprisings: &'a crate::uprising::UprisingState,
    /// Every mission as the tick event began, for the uprising incident's
    /// mission terms.
    missions: &'a VecDeque<ActiveMission>,
}

impl MissionSystem {
    /// Step every mission through the ticks in `tick_events`
    /// (`ghidra/notes/mission-lifecycle.md`, "Phase stepping").
    ///
    /// For each tick, members whose transit is due arrive, a mission whose
    /// members have all arrived (phase 4) or whose timer is due (phase 8)
    /// becomes ready, a lost container or target runs the validator
    /// (`FUN_00545240`), and each ready mission steps through phases until
    /// it waits again or ends. A new mission first runs its creation steps
    /// on the day it was ordered.
    ///
    /// The caller applies [`MissionAdvance::effects`], then each result's
    /// effects, to `GameWorld`.
    pub fn advance(
        state: &mut MissionState,
        world: &GameWorld,
        uprisings: &crate::uprising::UprisingState,
        tick_events: &[TickEvent],
        rolls: &[f64],
    ) -> MissionAdvance {
        let mut out = MissionAdvance::default();
        let count = state.missions.len();
        for (event_index, event) in tick_events.iter().enumerate() {
            let now = event.tick;
            // FUN_00545820: members whose transit is due arrive.
            let (arrived, travelling): (Vec<_>, Vec<_>) = std::mem::take(&mut state.en_route)
                .into_iter()
                .partition(|transit| transit.arrival <= now);
            state.en_route = travelling;
            out.effects.extend(
                arrived
                    .into_iter()
                    .map(|transit| MissionEffect::MemberMoved {
                        member: transit.member,
                        to: Some(transit.to),
                    }),
            );

            let missions = std::mem::take(&mut state.missions);
            let ctx = StepContext {
                world,
                uprisings,
                missions: &missions,
            };
            for (index, mut mission) in missions.clone().into_iter().enumerate() {
                // port: the mission's own window (see the RNG contract).
                let start = (event_index * count + index) * ROLLS_PER_MISSION;
                let window = rolls
                    .get(start..(start + ROLLS_PER_MISSION).min(rolls.len()))
                    .unwrap_or(&[]);
                let mut draws = MissionRolls {
                    rolls: window,
                    next: 0,
                    stream: crate::mission_detection::MissionRng::seeded(
                        window.get(STREAM_SEED_ROLL).copied().unwrap_or(0.5),
                        mission.id,
                    ),
                };

                // port: dispatch only queues the mission; its creation steps
                // run here, dated the day it was ordered.
                if mission.phase == 0 {
                    let ordered = mission.ordered_tick.min(now);
                    Self::run(&mut mission, &ctx, state, &mut out, ordered, &mut draws);
                }
                if mission.phase < PHASE_END {
                    Self::wake(&mut mission, &ctx, state, now);
                    Self::run(&mut mission, &ctx, state, &mut out, now, &mut draws);
                }
                if mission.phase == PHASE_END {
                    out.effects.extend(
                        mission
                            .members()
                            .map(|member| MissionEffect::MemberAvailable { member }),
                    );
                    out.ended.push(MissionEnded {
                        mission_id: mission.id,
                        kind: mission.kind,
                        faction: mission.faction,
                        end_code: mission.end_code,
                        tick: now,
                    });
                } else {
                    state.missions.push_back(mission);
                }
            }
        }
        out
    }

    /// Set a waiting mission's ready bit or end code for day `now`.
    fn wake(mission: &mut ActiveMission, ctx: &StepContext<'_>, state: &MissionState, now: u64) {
        // FUN_00545240, once per destruction: a destroyed container or
        // target runs the validator; a destroyed container then ends the
        // mission with code 7 when its phase (+0x54 -> +0x1c, FUN_00520ac0)
        // is above 6 (FUN_00520ae0), past the members' landing. A killed
        // target's check may repeat: the validator either ends the mission
        // (a dead target has no location, rule 6) or skips every rule.
        let container_lost = !mission.container_loss_seen
            && ctx
                .world
                .systems
                .get(mission.target_system)
                .is_some_and(|sys| sys.is_destroyed);
        let target_lost = mission.kind.target_rules() == TargetRules::Character
            && mission
                .target_character
                .and_then(|key| ctx.world.characters.get(key))
                .is_some_and(|c| c.is_killed);
        mission.container_loss_seen |= container_lost;
        if mission.end_code == 0 && (container_lost || target_lost) {
            mission.end_code = running_end_code(mission, ctx.world, ctx.uprisings, &state.en_route);
            if mission.end_code == 0 && container_lost && mission.phase > 6 {
                mission.end_code = 7;
            }
        }
        match mission.phase {
            // FUN_00545820 -> FUN_00522280.
            PHASE_TRANSIT => {
                if mission.end_code == 0 && Self::transit_over(mission, ctx.world, state) {
                    mission.ready = true;
                }
            }
            // FUN_00522a10: timer 0x38b fired.
            PHASE_TIMER => {
                if mission.timer_due.is_some_and(|due| due <= now) {
                    mission.ready = true;
                }
            }
            _ => {}
        }
    }

    /// Whether phase 4's wait is over (`FUN_00522280.c:66-100`): it holds
    /// only while every member still travels, so a mission with no member,
    /// or with any member off the road, is ready. The original deletes a
    /// killed member, so a dead one is skipped: a team killed on the way
    /// counts as arrived.
    fn transit_over(mission: &ActiveMission, world: &GameWorld, state: &MissionState) -> bool {
        let (mut travelling, mut stopped) = (false, false);
        for member in mission.members() {
            if member_state(world, member).is_none() {
                continue;
            }
            if state.is_en_route(member) {
                travelling = true;
            } else {
                stopped = true;
            }
        }
        stopped || !travelling
    }

    /// The stepper `FUN_005227d0`, run until the mission waits or ends.
    fn run(
        mission: &mut ActiveMission,
        ctx: &StepContext<'_>,
        state: &mut MissionState,
        out: &mut MissionAdvance,
        now: u64,
        draws: &mut MissionRolls<'_>,
    ) {
        loop {
            if mission.phase == PHASE_END {
                return;
            }
            let next = if mission.end_code != 0 {
                PHASE_END
            } else if mission.ready {
                mission.ready = false;
                let repeats = mission
                    .kind
                    .record_id()
                    .and_then(|id| ctx.world.mission_record(id))
                    .is_some_and(|record| record.repeats);
                if mission.phase == PHASE_OUTCOME && repeats {
                    PHASE_TIMER
                } else {
                    mission.phase + 1
                }
            } else {
                return;
            };
            let previous = mission.phase;
            mission.phase = next;
            Self::enter(mission, previous, ctx, state, out, now, draws);
            // FUN_00546ea0 runs the detection manager after the change.
            crate::mission_detection::run(
                mission,
                ctx.world,
                ctx.uprisings,
                &state.en_route,
                &mut draws.stream,
                &mut out.effects,
            );
            // FUN_00522980: every phase but 4 and 8 steps on at once.
            if !matches!(mission.phase, PHASE_TRANSIT | PHASE_TIMER) {
                mission.ready = true;
            }
        }
    }

    /// The phase handler `FUN_00524b70`: leave `previous`, enter the
    /// mission's new phase.
    fn enter(
        mission: &mut ActiveMission,
        previous: u8,
        ctx: &StepContext<'_>,
        state: &mut MissionState,
        out: &mut MissionAdvance,
        now: u64,
        draws: &mut MissionRolls<'_>,
    ) {
        let world = ctx.world;
        let validate = |mission: &mut ActiveMission, state: &MissionState| {
            if mission.end_code == 0 {
                mission.end_code = running_end_code(mission, world, ctx.uprisings, &state.en_route);
            }
        };
        // Leaving phase 8 runs the validator, then disarms timer 0x38b.
        if previous == PHASE_TIMER {
            validate(mission, state);
            mission.timer_due = None;
        }
        match mission.phase {
            2 => {
                // FUN_00522870 settles the team's location (+0x6c).
                let origin = mission
                    .members()
                    .find_map(|member| member_location(world, member));
                mission.origin = origin;
                mission.speed = Self::character_speed(mission, world);
            }
            PHASE_TRANSIT => {
                validate(mission, state);
                if mission.end_code == 0 {
                    Self::depart(mission, world, state, out, now);
                }
                mission.wait_start = now;
                // FUN_00522280.
                if mission.end_code == 0 && Self::transit_over(mission, ctx.world, state) {
                    mission.ready = true;
                }
            }
            // Phases 5 and 6 act only away from the origin (FUN_00520bb0);
            // phase 6 lands the members, which the arrivals already did.
            5 | 6 => {
                if mission.origin != Some(mission.target_system) {
                    validate(mission, state);
                }
            }
            PHASE_TIMER => {
                let delay = match mission
                    .kind
                    .record_id()
                    .and_then(|id| world.mission_record(id))
                {
                    Some(record) => crate::uprising::one_roll_timer_delay(
                        i32::try_from(record.timer_min_days).unwrap_or(i32::MAX),
                        i32::try_from(record.timer_spread_days).unwrap_or(i32::MAX),
                        draws.next(),
                    ),
                    // port: a kind with no record keeps the port's range.
                    None => {
                        let (min, max) = mission.kind.tick_range();
                        crate::uprising::one_roll_timer_delay(
                            i32::try_from(min).unwrap_or(i32::MAX),
                            i32::try_from(max - min).unwrap_or(i32::MAX),
                            draws.next(),
                        )
                    }
                };
                mission.wait_start = now;
                mission.timer_due = Some(now + delay);
            }
            PHASE_OUTCOME => {
                let result = Self::roll_members(mission, ctx, now, &mut draws.stream, &out.results);
                out.results.push(result);
            }
            PHASE_END => {
                validate(mission, state);
                if mission.end_code == 0 {
                    mission.end_code = 1;
                }
                // Slot +0x284 (FUN_00592c80) raises observation level 6 at
                // the target for the mission's side. port: left to the
                // detection work of F-019 phase 3.
            }
            _ => {}
        }
    }

    /// The speed `FUN_00548370` gives a mission's characters: GNPRTB 3083
    /// when Han Solo is a free member and no special force is, else the
    /// GNPRTB 1 default.
    fn character_speed(mission: &ActiveMission, world: &GameWorld) -> i64 {
        let han_free = mission.members().any(|member| {
            member
                .character()
                .and_then(|key| world.characters.get(key))
                .is_some_and(|c| c.dat_id.raw() == HAN_SOLO && !c.is_captive)
        });
        let has_special_force = mission
            .members()
            .any(|member| matches!(member, MissionMember::SpecialForce(_)));
        if han_free && !has_special_force {
            crate::movement::gnprtb_or_shipped(world, GNPRTB_HAN_SOLO_SPEED, SHIPPED_HAN_SOLO_SPEED)
        } else {
            crate::movement::default_speed(world)
        }
    }

    /// Phase 4's departures (`FUN_00556430`, `FUN_00556390`): every member
    /// travels from the origin to the target system and stays there. A
    /// member already there lands at once. With no origin, `FUN_00556430`
    /// (`:31-33`) starts no transit and no one moves.
    ///
    /// Every member travels at the speed phase 2 set: a special force
    /// travels at the default (`build-delivery.md`), and any special force
    /// member already holds the characters to it (`FUN_00548370`).
    fn depart(
        mission: &ActiveMission,
        world: &GameWorld,
        state: &mut MissionState,
        out: &mut MissionAdvance,
        now: u64,
    ) {
        let Some(from) = mission.origin else {
            return;
        };
        let position = |key: SystemKey| world.systems.get(key).map_or((0, 0), |s| (s.x, s.y));
        let to = mission.target_system;
        for member in mission.members() {
            if member_state(world, member).is_none() {
                continue;
            }
            let days = crate::movement::transit_ticks_between(
                world,
                position(from),
                position(to),
                mission.speed,
            );
            if days == 0 {
                out.effects.push(MissionEffect::MemberMoved {
                    member,
                    to: Some(to),
                });
            } else {
                // port: a travelling member is off the map, as a delivery is.
                out.effects
                    .push(MissionEffect::MemberMoved { member, to: None });
                state.en_route.push(MemberTransit {
                    member,
                    mission_id: mission.id,
                    to,
                    arrival: now + u64::from(days),
                });
            }
        }
    }

    /// Phase 10 (`FUN_00592f50`): each team character, then each team
    /// special force (`FUN_00526090`, `FUN_00525e70`), draws `0..99` below
    /// its chance (slot `+0x274`, [`member_chance`]) and acts on a success
    /// (slot `+0x27c`); the class's slot `+0x280` then applies the result
    /// `+0x60` once (`ghidra/notes/mission-lifecycle.md`, "Outcomes").
    /// Decoys and captives never roll. port: the draws come from the
    /// mission's seeded stream, and a recruit in `earlier`, this step's
    /// results so far, has left the pool although the world applies it later.
    fn roll_members(
        mission: &ActiveMission,
        ctx: &StepContext<'_>,
        now: u64,
        stream: &mut crate::mission_detection::MissionRng,
        earlier: &[MissionResult],
    ) -> MissionResult {
        use crate::uprising::Draws;
        let world = ctx.world;
        let side = crate::dat::Faction::from(mission.faction);
        let target = mission.target_system;
        let characters = mission
            .team
            .iter()
            .filter(|member| member.character().is_some());
        let special_forces = mission
            .team
            .iter()
            .filter(|member| member.character().is_none());
        let members: Vec<MissionMember> = characters
            .chain(special_forces)
            .copied()
            .filter(|&member| member_state(world, member).is_some())
            .collect();
        let mut effects = Vec::new();
        // +0x60: 0 unset, 2 failed, 3 succeeded (FUN_00521880).
        let mut result = 0_u8;
        // The target of an in-roll action, once it has been acted on.
        let mut target_taken = false;
        // Subdue's view of the target as the successes change it.
        let mut system = world.systems.get(target).cloned();
        let mut revolting = ctx.uprisings.is_uprising(target);
        // FUN_0055ef30: the side's characters Recruitment may still pick.
        let mut pool = if mission.kind == MissionKind::Recruitment {
            let taken: Vec<CharacterKey> = earlier
                .iter()
                .flat_map(|result| &result.effects)
                .filter_map(|effect| match effect {
                    MissionEffect::CharacterRecruited { character, .. } => Some(*character),
                    _ => None,
                })
                .collect();
            recruit_pool(world, side)
                .into_iter()
                .filter(|character| !taken.contains(character))
                .collect()
        } else {
            Vec::new()
        };
        for member in members {
            if mission.kind == MissionKind::SubdueUprising && !(system.is_some() && revolting) {
                // FUN_00569c20 needs the target in an uprising (+0x88 bit 2).
                continue;
            }
            if mission.kind == MissionKind::InciteUprising && system.is_none() {
                continue;
            }
            let chance = member_chance(
                world,
                mission.kind,
                mission.faction,
                target,
                mission.target_character,
                member,
            );
            if !stream.chance(chance) {
                continue;
            }
            // Slot +0x1d8 holds for a character (FUN_0040f340) and not for a
            // special force (FUN_006158b0).
            let raise = |skill: Skill, id: u16| {
                member
                    .character()
                    .map(|character| MissionEffect::SkillRaised {
                        character,
                        skill,
                        amount: skill_raise(world, id),
                    })
            };
            match mission.kind {
                // FUN_005740a0.
                MissionKind::Diplomacy => {
                    result = 3;
                    effects.extend(raise(Skill::Diplomacy, GNPRTB_DIPLOMACY_RAISE));
                }
                // FUN_00573540: the raise needs a target (FUN_00521070) of
                // another side.
                MissionKind::Espionage => {
                    result = 3;
                    let foreign = world
                        .systems
                        .get(target)
                        .is_some_and(|sys| crate::uprising::holder(sys) != Some(side));
                    if foreign {
                        effects.extend(raise(Skill::Espionage, GNPRTB_ESPIONAGE_RAISE));
                    }
                }
                // FUN_0056b9a0: at the target system (FUN_00586720), a
                // random pool character (FUN_0055fc80, draw FUN_0053e290)
                // joins there (FUN_0055fe70); then 3 and the leadership
                // raise. An empty pool leaves the result alone.
                MissionKind::Recruitment => {
                    let pick = if world.systems.contains_key(target) {
                        stream.pick(pool.len())
                    } else {
                        None
                    };
                    if let Some(index) = pick {
                        let character = pool.remove(index);
                        effects.push(MissionEffect::CharacterRecruited {
                            character,
                            system: target,
                            faction: mission.faction,
                            pool_emptied: pool.is_empty(),
                        });
                        result = 3;
                        effects.extend(raise(Skill::Leadership, GNPRTB_RECRUITMENT_RAISE));
                    }
                }
                // FUN_0056ae50: the target's slot +0x210 frees it.
                MissionKind::Rescue => {
                    result = 3;
                    effects.extend(raise(Skill::Combat, GNPRTB_RESCUE_RAISE));
                    if let Some(character) = mission.target_character.filter(|_| !target_taken) {
                        target_taken = true;
                        effects.push(MissionEffect::CharacterRescued {
                            character,
                            returned_to: mission.faction,
                            at_system: target,
                        });
                    }
                }
                // FUN_00576eb0: the target's slot +0x20c takes it prisoner.
                MissionKind::Abduction => {
                    result = 3;
                    effects.extend(raise(Skill::Combat, GNPRTB_ABDUCTION_RAISE));
                    if let Some(character) = mission.target_character.filter(|_| !target_taken) {
                        target_taken = true;
                        effects.push(MissionEffect::CharacterCaptured {
                            character,
                            captured_by: mission.faction,
                            at_system: target,
                        });
                    }
                }
                // FUN_00576620: slot +0x2ec kills the target; the result and
                // the raise need it destroyed (+0x50 bit 3). port: the kill
                // always destroys.
                MissionKind::Assassination => {
                    if let Some(character) = mission.target_character {
                        if !target_taken {
                            target_taken = true;
                            effects.push(MissionEffect::CharacterKilled {
                                character,
                                faction: mission.faction,
                            });
                        }
                        result = 3;
                        effects.extend(raise(Skill::Combat, GNPRTB_ASSASSINATION_RAISE));
                    }
                }
                // FUN_00571a60: the uprising incident, then 2 while the
                // holder keeps a regiment there (FUN_00509020), else 3.
                // port: its other test, the system's +0x84 bits 2..3 against
                // the opponent, reads a field no recovered function writes.
                MissionKind::InciteUprising => {
                    let event =
                        crate::uprising::incite_incident(world, ctx.missions, target, now, stream);
                    let lost: Vec<crate::ids::TroopKey> = event
                        .iter()
                        .flat_map(|event| match event {
                            crate::uprising::UprisingEvent::UprisingIncident { losses, .. } => {
                                losses.clone()
                            }
                            _ => Vec::new(),
                        })
                        .filter_map(|loss| match loss {
                            crate::uprising::IncidentLoss::Regiment(troop) => Some(troop),
                            _ => None,
                        })
                        .collect();
                    let garrisoned = world.systems.get(target).is_some_and(|sys| {
                        crate::uprising::holder(sys).is_some_and(|holder| {
                            sys.ground_units.iter().any(|key| {
                                !lost.contains(key)
                                    && world.troops.get(*key).is_some_and(|troop| {
                                        crate::mission_detection::faction_of(troop.is_alliance)
                                            == holder
                                    })
                            })
                        })
                    });
                    effects.extend(event.map(MissionEffect::UprisingIncident));
                    result = if garrisoned { 2 } else { 3 };
                    if result == 3 {
                        effects.extend(raise(Skill::Leadership, GNPRTB_INCITE_RAISE));
                    }
                }
                // FUN_00569c20: support (FUN_0055cb10, FUN_0050c9f0), then the
                // end check (FUN_0050c910); 3 once the uprising ended, else 2.
                MissionKind::SubdueUprising => {
                    if let Some(sys) = system.as_mut() {
                        let gain = crate::uprising::mission_support_gain(
                            world,
                            sys,
                            side,
                            crate::uprising::SupportGain::Subdue,
                            stream,
                        );
                        *sys = crate::uprising::with_support_change(world, sys, side, gain);
                        effects.push(MissionEffect::UprisingSubdued {
                            system: target,
                            side,
                            support_gain: gain,
                        });
                        if crate::uprising::garrison_covers(world, sys) {
                            revolting = false;
                        }
                    }
                    result = if revolting { 2 } else { 3 };
                    if result == 3 {
                        effects.extend(raise(Skill::Leadership, GNPRTB_SUBDUE_RAISE));
                    }
                }
                // FUN_0056a300 and FUN_00574630.
                MissionKind::Sabotage => {
                    result = 3;
                    effects.extend(raise(Skill::Espionage, GNPRTB_SABOTAGE_ESPIONAGE_RAISE));
                    effects.extend(raise(Skill::Combat, GNPRTB_SABOTAGE_COMBAT_RAISE));
                }
                MissionKind::DeathStarSabotage => {
                    result = 3;
                    effects.extend(raise(Skill::Espionage, GNPRTB_DS_SABOTAGE_ESPIONAGE_RAISE));
                    effects.extend(raise(Skill::Combat, GNPRTB_DS_SABOTAGE_COMBAT_RAISE));
                }
                MissionKind::Autoscrap => result = 3,
            }
        }
        // Slot +0x280: an unset result fails (FUN_00576700).
        if result == 0 {
            result = 2;
        }
        if result == 3 {
            match mission.kind {
                // FUN_00574120: FUN_0055cac0 support through FUN_0050c9f0.
                MissionKind::Diplomacy => {
                    if let Some(sys) = world.systems.get(target) {
                        let points = crate::uprising::mission_support_gain(
                            world,
                            sys,
                            side,
                            crate::uprising::SupportGain::Diplomacy,
                            stream,
                        );
                        if points != 0 {
                            effects.push(MissionEffect::SupportGained {
                                system: target,
                                side,
                                points,
                            });
                        }
                    }
                }
                // FUN_00573610: observation level 10 at a system target
                // (FUN_0050d5a0). port: the port reveals the system; the
                // counts of FUN_0055c940 are not ported.
                MissionKind::Espionage => {
                    if world.systems.contains_key(target) {
                        effects.push(MissionEffect::SystemIntelligenceGathered {
                            system: target,
                            faction: mission.faction,
                        });
                    }
                }
                // FUN_005746e0: slot +0xac(6) destroys the order's object
                // (FUN_00521030).
                MissionKind::Sabotage | MissionKind::DeathStarSabotage => {
                    effects.extend(
                        mission
                            .target_object
                            .filter(|object| object.state(world).is_some())
                            .map(|object| MissionEffect::TargetSabotaged {
                                target: object,
                                system: target,
                            }),
                    );
                }
                _ => {}
            }
        }
        MissionResult {
            mission_id: mission.id,
            tick: now,
            kind: mission.kind,
            faction: mission.faction,
            character: mission.lead_character(),
            target_system: target,
            outcome: if result == 3 {
                MissionOutcome::Success
            } else {
                MissionOutcome::Failure
            },
            effects,
        }
    }

    /// Check for captured characters escaping on their own.
    ///
    /// For each character held by the opposing faction, look up ESCAPETB
    /// and roll against the escape probability. Returns one `CharacterEscaped`
    /// effect per successful escape.
    #[must_use]
    pub fn check_escapes(world: &GameWorld, rolls: &[f64]) -> Vec<MissionEffect> {
        let Some(table) = world.mission_tables.get("ESCAPETB") else {
            return Vec::new();
        };

        let mut effects = Vec::new();
        let mut roll_iter = rolls.iter().copied();

        for (char_key, character) in &world.characters {
            if !character.is_captive {
                continue;
            }
            let roll = roll_iter.next().unwrap_or(1.0); // 1.0 = no escape (safe default)
                                                        // Use loyalty as the skill score for escape probability
            let skill_score = character.loyalty.base.cast_signed();
            let escape_prob = f64::from(table.lookup(skill_score)) / 100.0;
            if roll < escape_prob {
                // Character escapes back to their home faction.
                // is_alliance reflects original allegiance — capture tracks captor,
                // not the character's identity.
                let escaped_to_alliance = character.is_alliance;
                effects.push(MissionEffect::CharacterEscaped {
                    character: char_key,
                    escaped_to_alliance,
                });
            }
        }

        effects
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{CharacterKey, SystemKey};
    use crate::uprising::UprisingState;
    use crate::world::{Character, GameWorld, SkillPair};

    #[test]
    fn a_kind_with_no_record_times_its_mission_from_the_port_range() {
        // port: without a MISSNSD record the timer draws from tick_range.
        for (roll, due) in [(0.0, 15), (1.0, 20)] {
            let (system, _) = mock_keys();
            let mut world = minimal_world();
            let character = agent_at(&mut world, system, true);
            let mut state = MissionState::new();
            state.dispatch(MissionRequest::single(
                MissionKind::Diplomacy,
                MissionFaction::Alliance,
                character,
                system,
                None,
                0,
            ));
            step(&mut state, &world, 1, &[roll]);
            assert_eq!(state.missions()[0].timer_due, Some(due), "roll {roll}");
        }
    }

    // --- MissionState tests ---

    fn mock_keys() -> (SystemKey, CharacterKey) {
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let mut chr_sm: slotmap::SlotMap<CharacterKey, ()> = slotmap::SlotMap::with_key();
        (sys_sm.insert(()), chr_sm.insert(()))
    }

    #[test]
    fn a_dispatched_mission_starts_in_phase_zero_with_its_ready_bit_set() {
        // FUN_0054bac0 -> FUN_00522130(this, 1).
        let (system, character) = mock_keys();
        let mut state = MissionState::new();
        let id = state.dispatch(MissionRequest::single(
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            character,
            system,
            None,
            7,
        ));
        assert_eq!(state.len(), 1);
        let m = &state.missions()[0];
        assert_eq!(m.id, id);
        assert_eq!((m.phase, m.ready, m.ordered_tick), (0, true, 7));
        assert_eq!(m.timer_due, None);
    }

    #[test]
    fn cancel_removes_mission() {
        let (system, character) = mock_keys();
        let mut state = MissionState::new();
        let id = state.dispatch(MissionRequest::single(
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            character,
            system,
            None,
            0,
        ));
        let removed = state.cancel(id);
        assert!(removed.is_some());
        assert!(state.is_empty());
    }

    // --- MissionSystem::advance tests ---

    fn minimal_world() -> GameWorld {
        GameWorld::default()
    }

    fn skill_pair(base: u32) -> SkillPair {
        SkillPair { base, variance: 0 }
    }

    fn character_with_diplomacy(world: &mut GameWorld, diplomacy: u32) -> CharacterKey {
        world.characters.insert(Character {
            name: "Test".into(),
            is_alliance: true,
            diplomacy: skill_pair(diplomacy),
            can_be_commander: true,
            ..Default::default()
        })
    }

    #[test]
    fn no_tick_events_no_results() {
        let (system, character) = mock_keys();
        let mut state = MissionState::new();
        state.dispatch(MissionRequest::single(
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            character,
            system,
            None,
            0,
        ));
        let world = minimal_world();
        let results =
            MissionSystem::advance(&mut state, &world, &UprisingState::default(), &[], &[]).results;
        assert!(results.is_empty());
        assert_eq!(state.len(), 1);
    }

    #[test]
    fn a_mission_waits_in_phase_eight_until_its_timer_is_due() {
        // FUN_00522a10 sets the ready bit only when timer 0x38b fires.
        let (system, character) = mock_keys();
        let mut state = MissionState::new();
        state.missions.push_back(ActiveMission::timed(
            0,
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            vec![MissionMember::Character(character)],
            system,
            3,
        ));
        let world = minimal_world();
        let advance = MissionSystem::advance(
            &mut state,
            &world,
            &UprisingState::default(),
            &[TickEvent { tick: 1 }],
            &[0.0],
        );
        assert!(advance.results.is_empty());
        let mission = &state.missions()[0];
        assert_eq!((mission.phase, mission.ready), (PHASE_TIMER, false));
        assert_eq!(mission.days_left(1, None), Some(2));
    }

    #[test]
    fn mission_resolves_at_zero_ticks() {
        let mut world = minimal_world();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let system = sys_sm.insert(());
        let character = character_with_diplomacy(&mut world, 80);

        let mut state = MissionState::new();
        // Manually insert a mission with 1 tick remaining
        state.missions.push_back(ActiveMission::timed(
            0,
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            vec![MissionMember::Character(character)],
            system,
            1, // will resolve on next tick
        ));
        state.next_id = 1;

        // Roll = 0.01 → 1% of 100 → very likely to succeed with high-skill character
        let results = MissionSystem::advance(
            &mut state,
            &world,
            &UprisingState::default(),
            &[TickEvent { tick: 1 }],
            &[0.01],
        )
        .results;
        assert_eq!(results.len(), 1);
        assert!(state.is_empty());
        assert_eq!(results[0].kind, MissionKind::Diplomacy);
    }

    #[test]
    fn guaranteed_failure_with_roll_one() {
        let mut world = minimal_world();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let system = sys_sm.insert(());
        let character = character_with_diplomacy(&mut world, 50);

        let mut state = MissionState::new();
        state.missions.push_back(ActiveMission::timed(
            0,
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            vec![MissionMember::Character(character)],
            system,
            1,
        ));
        state.next_id = 1;

        // roll = 1.0 → 100% of 100 → fails since success_prob <= 100
        let advance = MissionSystem::advance(
            &mut state,
            &world,
            &UprisingState::default(),
            &[TickEvent { tick: 1 }],
            &[1.0],
        );
        assert_eq!(advance.results[0].outcome, MissionOutcome::Failure);
        // A failure has no world effect; the release comes with the end.
        assert!(advance.results[0].effects.is_empty());
        assert_eq!(
            advance.effects,
            vec![MissionEffect::MemberAvailable {
                member: MissionMember::Character(character)
            }]
        );
    }

    #[test]
    fn multiple_missions_resolve_independently() {
        let mut world = minimal_world();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let sys_a = sys_sm.insert(());
        let sys_b = sys_sm.insert(());
        let char_a = character_with_diplomacy(&mut world, 70);
        let char_b = character_with_diplomacy(&mut world, 70);

        let mut state = MissionState::new();
        state.missions.push_back(ActiveMission::timed(
            0,
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            vec![MissionMember::Character(char_a)],
            sys_a,
            1,
        ));
        state.missions.push_back(ActiveMission::timed(
            1,
            MissionKind::Diplomacy,
            MissionFaction::Empire,
            vec![MissionMember::Character(char_b)],
            sys_b,
            3,
        ));
        state.next_id = 2;

        let results = MissionSystem::advance(
            &mut state,
            &world,
            &UprisingState::default(),
            &[TickEvent { tick: 1 }],
            &[0.0], // first mission succeeds
        )
        .results;
        // Only mission 0 completes (1 tick); mission 1 has 2 ticks left
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].mission_id, 0);
        assert_eq!(state.len(), 1);
        assert_eq!(state.missions()[0].timer_due, Some(3));
    }

    // --- New mission kind tests ---

    #[test]
    fn autoscrap_always_succeeds() {
        let (system, _) = mock_keys();
        let mut world = minimal_world();
        let character = agent_at(&mut world, system, false);
        let mut state = MissionState::new();
        state.missions.push_back(ActiveMission::timed(
            0,
            MissionKind::Autoscrap,
            MissionFaction::Empire,
            vec![MissionMember::Character(character)],
            system,
            1,
        ));
        state.next_id = 1;

        // roll = 1.0 → Autoscrap ignores the roll and always succeeds.
        let results = MissionSystem::advance(
            &mut state,
            &world,
            &UprisingState::default(),
            &[TickEvent { tick: 1 }],
            &[1.0],
        )
        .results;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].outcome, MissionOutcome::Success);
    }

    #[test]
    fn mstb_key_matches_expected_dat_stems() {
        assert_eq!(MissionKind::Diplomacy.mstb_key(), Some("DIPLMSTB"));
        assert_eq!(MissionKind::Recruitment.mstb_key(), Some("RCRTMSTB"));
        assert_eq!(MissionKind::Sabotage.mstb_key(), Some("SBTGMSTB"));
        assert_eq!(MissionKind::Assassination.mstb_key(), Some("ASSNMSTB"));
        assert_eq!(MissionKind::Espionage.mstb_key(), Some("ESPIMSTB"));
        assert_eq!(MissionKind::Rescue.mstb_key(), Some("RESCMSTB"));
        assert_eq!(MissionKind::Abduction.mstb_key(), Some("ABDCMSTB"));
        assert_eq!(MissionKind::InciteUprising.mstb_key(), Some("INCTMSTB"));
        assert_eq!(MissionKind::Autoscrap.mstb_key(), None);
    }

    // --- Character availability tracking tests ---

    #[test]
    fn every_team_decoy_and_captured_member_is_freed_when_its_mission_completes() {
        // FUN_00525bb0 walks the team, decoy, and captured lists; completion
        // clears each member's RoleOnMissionNotif flag (FUN_00536b00).
        let mut world = minimal_world();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let system = sys_sm.insert(());
        let lead = character_with_diplomacy(&mut world, 80);
        let prisoner = character_with_diplomacy(&mut world, 10);
        let decoy = MissionMember::SpecialForce(world.special_forces.insert(
            crate::world::SpecialForceUnit {
                class_dat_id: crate::ids::DatId::new(0x3c00_0001),
                is_alliance: true,
                skills: [0; 8],
                on_mission: true,
            },
        ));

        let mut state = MissionState::new();
        let mut mission = ActiveMission::timed(
            0,
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            vec![MissionMember::Character(lead)],
            system,
            1,
        );
        mission.decoys = vec![decoy];
        mission.captured = vec![MissionMember::Character(prisoner)];
        state.missions.push_back(mission);
        state.next_id = 1;

        let advance = MissionSystem::advance(
            &mut state,
            &world,
            &UprisingState::default(),
            &[TickEvent { tick: 1 }],
            &[0.0],
        );
        assert_eq!(advance.results.len(), 1);
        let freed: Vec<MissionMember> = advance
            .effects
            .iter()
            .filter_map(|e| match e {
                MissionEffect::MemberAvailable { member } => Some(*member),
                _ => None,
            })
            .collect();
        assert_eq!(
            freed,
            vec![
                MissionMember::Character(lead),
                decoy,
                MissionMember::Character(prisoner)
            ]
        );
    }

    // --- Escape tests ---

    #[test]
    fn check_escapes_returns_escaped_when_roll_below_prob() {
        use crate::dat::Faction;
        use crate::world::{MstbEntry, MstbTable};

        let mut world = minimal_world();
        let _char_key = world.characters.insert(Character {
            name: "Captive".into(),
            is_alliance: true,
            loyalty: skill_pair(50), // loyalty base = 50
            captured_by: Some(Faction::Empire),
            capture_tick: Some(10),
            is_captive: true,
            ..Default::default()
        });

        // ESCAPETB: loyalty 50 → 80% escape probability
        let table = MstbTable::new(vec![
            MstbEntry {
                threshold: 0,
                value: 50,
            },
            MstbEntry {
                threshold: 50,
                value: 80,
            },
            MstbEntry {
                threshold: 100,
                value: 95,
            },
        ]);
        world.mission_tables.insert("ESCAPETB".to_string(), table);

        // Roll 0.5 < 0.80 → escapes
        let effects = MissionSystem::check_escapes(&world, &[0.5]);
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            MissionEffect::CharacterEscaped {
                escaped_to_alliance,
                ..
            } => {
                // Captured by Empire → escapes TO Alliance
                assert!(*escaped_to_alliance);
            }
            other => panic!("expected CharacterEscaped, got {other:?}"),
        }
    }

    #[test]
    fn check_escapes_skips_non_captive() {
        use crate::world::{MstbEntry, MstbTable};

        let mut world = minimal_world();
        world.characters.insert(Character {
            name: "Free".into(),
            is_alliance: true,
            loyalty: skill_pair(90),
            // is_captive defaults to false
            ..Default::default()
        });

        let table = MstbTable::new(vec![
            MstbEntry {
                threshold: 0,
                value: 100,
            }, // would always escape if checked
        ]);
        world.mission_tables.insert("ESCAPETB".to_string(), table);

        // Roll that would succeed — but character is not captive
        let effects = MissionSystem::check_escapes(&world, &[0.0]);
        assert!(effects.is_empty());
    }

    #[test]
    fn check_escapes_returns_empty_without_escapetb() {
        let world = minimal_world();
        // No ESCAPETB in mission_tables
        let effects = MissionSystem::check_escapes(&world, &[0.0, 0.0, 0.0]);
        assert!(effects.is_empty());
    }

    #[test]
    fn dispatch_guarded_blocks_mandatory_mission() {
        let mut world = minimal_world();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let system = sys_sm.insert(());
        let character = world.characters.insert(Character {
            name: "Mandatory".into(),
            is_alliance: true,
            is_major: true,
            diplomacy: skill_pair(80),
            can_be_commander: true,
            on_mandatory_mission: true, // <-- mandatory
            ..Default::default()
        });

        let mut state = MissionState::new();
        let result = state.dispatch_guarded(
            MissionRequest::single(
                MissionKind::Diplomacy,
                MissionFaction::Alliance,
                character,
                system,
                None,
                0,
            ),
            &mut world,
        );
        assert_eq!(
            result,
            Err(MissionRefusal::MemberUnavailable(MissionMember::Character(
                character
            ))),
            "mandatory mission character should be blocked"
        );
        assert!(state.is_empty());
    }

    #[test]
    fn a_character_with_an_active_mission_is_refused_even_without_the_flag() {
        let mut world = minimal_world();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let system = sys_sm.insert(());
        let character = world.characters.insert(Character {
            name: "Agent".into(),
            is_alliance: true,
            ..Default::default()
        });
        // An older save: the mission is in flight but on_mission was never set.
        let mut state = MissionState::new();
        state.dispatch(MissionRequest::single(
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            character,
            system,
            None,
            0,
        ));

        let second = state.dispatch_guarded(
            MissionRequest::single(
                MissionKind::Espionage,
                MissionFaction::Alliance,
                character,
                system,
                None,
                0,
            ),
            &mut world,
        );

        assert_eq!(
            second,
            Err(MissionRefusal::MemberUnavailable(MissionMember::Character(
                character
            )))
        );
        assert_eq!(state.len(), 1);
    }

    #[test]
    fn a_dispatched_character_cannot_take_a_second_mission_until_released() {
        let mut world = minimal_world();
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        let system = sys_sm.insert(());
        let character = world.characters.insert(Character {
            name: "Agent".into(),
            is_alliance: true,
            diplomacy: skill_pair(80),
            ..Default::default()
        });
        let mut state = MissionState::new();
        let send = |state: &mut MissionState, world: &mut GameWorld| {
            state.dispatch_guarded(
                MissionRequest::single(
                    MissionKind::Diplomacy,
                    MissionFaction::Alliance,
                    character,
                    system,
                    None,
                    0,
                ),
                world,
            )
        };

        let first = send(&mut state, &mut world).expect("first dispatch");
        assert!(world.characters[character].on_mission);
        assert!(send(&mut state, &mut world).is_err());
        assert_eq!(state.len(), 1);

        state.release(first, &mut world);
        assert!(!world.characters[character].on_mission);
        assert!(send(&mut state, &mut world).is_ok());
    }

    // --- Member rules (FUN_0054bb90, FUN_00522b30) ---

    /// A loyal agent (loyalty 100, so phase 9 never finds a traitor,
    /// `FUN_00588da0`).
    fn agent_at(world: &mut GameWorld, system: SystemKey, is_alliance: bool) -> CharacterKey {
        world.characters.insert(Character {
            name: "Agent".into(),
            is_alliance,
            is_empire: !is_alliance,
            loyalty: crate::world::SkillPair {
                base: 100,
                variance: 0,
            },
            current_system: Some(system),
            recruited: true,
            ..Default::default()
        })
    }

    fn two_systems() -> (SystemKey, SystemKey) {
        let mut sys_sm: slotmap::SlotMap<SystemKey, ()> = slotmap::SlotMap::with_key();
        (sys_sm.insert(()), sys_sm.insert(()))
    }

    fn request(
        team: Vec<MissionMember>,
        decoys: Vec<MissionMember>,
        at: SystemKey,
    ) -> MissionRequest {
        MissionRequest {
            kind: MissionKind::Sabotage,
            faction: MissionFaction::Alliance,
            team,
            decoys,
            target_system: at,
            target_character: None,
            // FUN_0056a110 reads only the object's family at creation.
            target_object: Some(MissionTarget::ManufacturingFacility(
                crate::ids::ManufacturingFacilityKey::default(),
            )),
            tick: 0,
        }
    }

    // --- The pop-up menu's Mission item (FUN_0051fe20, FUN_005429e0) ---

    #[test]
    fn mission_is_enabled_for_a_free_recruited_member_of_the_players_side() {
        let mut world = minimal_world();
        let (here, _) = two_systems();
        let agent = MissionMember::Character(agent_at(&mut world, here, true));
        let state = MissionState::new();
        assert!(state.mission_order_enabled(&world, MissionFaction::Alliance, &[agent]));
        // FUN_004f9860: the other side's player may not order it (status 1).
        assert!(!state.mission_order_enabled(&world, MissionFaction::Empire, &[agent]));
    }

    #[test]
    fn mission_is_disabled_for_a_busy_captive_or_unrecruited_member() {
        // FUN_004f9860 (unrecruited, status 2), FUN_00533ce0 (on a mission),
        // FUN_004ed560 (a prisoner, for order 0x240).
        type Spoil = fn(&mut Character);
        let spoils: [(&str, Spoil); 4] = [
            ("on a mission", |c| c.on_mission = true),
            ("on a mandatory mission", |c| c.on_mandatory_mission = true),
            ("a prisoner", |c| c.is_captive = true),
            ("unrecruited", |c| c.recruited = false),
        ];
        for (why, spoil) in spoils {
            let mut world = minimal_world();
            let (here, _) = two_systems();
            let key = agent_at(&mut world, here, true);
            spoil(&mut world.characters[key]);
            let state = MissionState::new();
            assert!(
                !state.mission_order_enabled(
                    &world,
                    MissionFaction::Alliance,
                    &[MissionMember::Character(key)]
                ),
                "{why}"
            );
        }
    }

    #[test]
    fn mission_is_disabled_while_a_member_travels_or_serves_a_queued_mission() {
        // FUN_004f9860 refuses an object en route (status 4); FUN_00533ce0
        // one already on a mission.
        let mut world = minimal_world();
        let (here, there) = two_systems();
        let agent = MissionMember::Character(agent_at(&mut world, here, true));
        let mut travelling = MissionState::new();
        travelling.push_transit(MemberTransit {
            member: agent,
            mission_id: 7,
            to: there,
            arrival: 10,
        });
        assert!(!travelling.mission_order_enabled(&world, MissionFaction::Alliance, &[agent]));

        let mut queued = MissionState::new();
        queued.dispatch(request(vec![agent], vec![], here));
        assert!(!queued.mission_order_enabled(&world, MissionFaction::Alliance, &[agent]));
    }

    #[test]
    fn mission_needs_every_located_member_at_one_place_and_one_located_member() {
        // FUN_0054bf00: members apart fail with 0x40/3, a team without a
        // located member with 0x16; FUN_0054bb90 fails an empty team.
        let mut world = minimal_world();
        let (here, there) = two_systems();
        let a = MissionMember::Character(agent_at(&mut world, here, true));
        let b = MissionMember::Character(agent_at(&mut world, here, true));
        let far = MissionMember::Character(agent_at(&mut world, there, true));
        let nowhere_key = agent_at(&mut world, here, true);
        world.characters[nowhere_key].current_system = None;
        let nowhere = MissionMember::Character(nowhere_key);
        let state = MissionState::new();
        let enabled = |team: &[MissionMember]| {
            state.mission_order_enabled(&world, MissionFaction::Alliance, team)
        };

        assert!(enabled(&[a, b]));
        assert!(!enabled(&[a, far]));
        assert!(enabled(&[a, nowhere]));
        assert!(!enabled(&[nowhere]));
        assert!(!enabled(&[]));
    }

    #[test]
    fn mission_is_disabled_for_a_member_that_no_longer_exists() {
        // port: a killed character stays in the arena but counts as gone.
        let mut world = minimal_world();
        let (here, _) = two_systems();
        let key = agent_at(&mut world, here, true);
        world.characters[key].is_killed = true;
        let state = MissionState::new();
        assert!(!state.mission_order_enabled(
            &world,
            MissionFaction::Alliance,
            &[MissionMember::Character(key)]
        ));
    }

    #[test]
    fn a_prisoner_goes_to_the_captured_list_and_a_team_of_prisoners_is_refused() {
        // FUN_0054bb90 moves a prisoner (slot +0x1d4) from either list to the
        // captured list, then refuses an empty team with 0x40/0x91.
        let mut world = minimal_world();
        let (here, _) = two_systems();
        let lead = agent_at(&mut world, here, true);
        let prisoner = agent_at(&mut world, here, true);
        world.characters[prisoner].is_captive = true;
        let mut state = MissionState::new();

        let alone = request(vec![MissionMember::Character(prisoner)], vec![], here);
        assert_eq!(
            state.dispatch_guarded(alone, &mut world),
            Err(MissionRefusal::EmptyTeam)
        );
        assert!(!world.characters[prisoner].on_mission);

        let team = vec![
            MissionMember::Character(lead),
            MissionMember::Character(prisoner),
        ];
        state
            .dispatch_guarded(request(team, vec![], here), &mut world)
            .expect("the free lead keeps the team non-empty");
        let mission = &state.missions()[0];
        assert_eq!(mission.team, vec![MissionMember::Character(lead)]);
        assert_eq!(mission.captured, vec![MissionMember::Character(prisoner)]);
    }

    #[test]
    fn a_member_of_the_other_side_is_refused() {
        // FUN_00522b30: the member's side (+0x24 & 0xc0) must match the mission's.
        let mut world = minimal_world();
        let (here, _) = two_systems();
        let lead = agent_at(&mut world, here, true);
        let imperial = agent_at(&mut world, here, false);
        let mut state = MissionState::new();
        let team = vec![
            MissionMember::Character(lead),
            MissionMember::Character(imperial),
        ];

        assert_eq!(
            state.dispatch_guarded(request(team, vec![], here), &mut world),
            Err(MissionRefusal::MemberUnavailable(MissionMember::Character(
                imperial
            )))
        );
        assert!(state.is_empty());
        assert!(!world.characters[lead].on_mission);
    }

    #[test]
    fn members_at_different_locations_are_refused() {
        // FUN_00522b30: a member must share the location key (slot +0xc) of
        // the members already on the mission.
        let mut world = minimal_world();
        let (here, there) = two_systems();
        let lead = agent_at(&mut world, here, true);
        let far = agent_at(&mut world, there, true);
        let mut state = MissionState::new();
        let team = vec![
            MissionMember::Character(lead),
            MissionMember::Character(far),
        ];

        assert_eq!(
            state.dispatch_guarded(request(team, vec![], here), &mut world),
            Err(MissionRefusal::MemberUnavailable(MissionMember::Character(
                far
            )))
        );
    }

    #[test]
    fn a_member_named_twice_joins_once() {
        // FUN_00522b30 skips a member already in the team, decoy, or captured
        // list (FUN_00520c30).
        let mut world = minimal_world();
        let (here, _) = two_systems();
        let lead = agent_at(&mut world, here, true);
        let mut state = MissionState::new();
        let me = MissionMember::Character(lead);

        let decoy = MissionMember::Character(agent_at(&mut world, here, true));
        let prisoner = agent_at(&mut world, here, true);
        world.characters[prisoner].is_captive = true;
        let prisoner = MissionMember::Character(prisoner);

        state
            .dispatch_guarded(
                request(
                    vec![me, me, prisoner, prisoner],
                    vec![me, decoy, decoy],
                    here,
                ),
                &mut world,
            )
            .expect("one lead");
        let mission = &state.missions()[0];
        assert_eq!(mission.team, vec![me]);
        assert_eq!(mission.decoys, vec![decoy]);
        assert_eq!(mission.captured, vec![prisoner]);
    }

    #[test]
    fn the_lead_is_the_first_team_character_and_a_special_force_has_no_character_key() {
        // port: the interim resolver rolls one character until F-019 phase 4
        // ports the per-member roll; a special force is never that character.
        let mut sf: slotmap::SlotMap<SpecialForceKey, ()> = slotmap::SlotMap::with_key();
        let unit = MissionMember::SpecialForce(sf.insert(()));
        let mut chr: slotmap::SlotMap<CharacterKey, ()> = slotmap::SlotMap::with_key();
        let (character, second) = (chr.insert(()), chr.insert(()));
        let (system, _) = mock_keys();
        let mission = ActiveMission::timed(
            0,
            MissionKind::Sabotage,
            MissionFaction::Alliance,
            vec![
                unit,
                MissionMember::Character(character),
                MissionMember::Character(second),
            ],
            system,
            1,
        );

        assert_eq!(unit.character(), None);
        assert_eq!(mission.lead_character(), Some(character));
        let empty = ActiveMission::timed(
            1,
            MissionKind::Sabotage,
            MissionFaction::Alliance,
            vec![unit],
            system,
            1,
        );
        assert_eq!(empty.lead_character(), None);
    }

    #[test]
    fn leaving_a_mission_clears_the_hidden_mission_flag_and_joining_keeps_it() {
        // port: leaving clears RoleOnMission (bit 7) and RoleOnHiddenMission
        // (bit 8) together. FUN_00536220 recomputes bit 7 on leave, and where
        // bit 8 clears is untraced. Joining sets only bit 7 here (FUN_00522b30
        // sets bit 8 from the record, F-019 phase 2).
        let mut world = minimal_world();
        let (here, _) = two_systems();
        let lead = agent_at(&mut world, here, true);
        world.characters[lead].on_hidden_mission = true;

        set_on_mission(&mut world, MissionMember::Character(lead), true);
        assert!(world.characters[lead].on_hidden_mission);
        set_on_mission(&mut world, MissionMember::Character(lead), false);
        assert!(!world.characters[lead].on_mission);
        assert!(!world.characters[lead].on_hidden_mission);
    }

    /// A special force on the surface of a new system (special forces live
    /// in a system's list, not in a fleet).
    fn unit_on_surface(world: &mut GameWorld, is_alliance: bool) -> (SpecialForceKey, SystemKey) {
        let unit = world.special_forces.insert(crate::world::SpecialForceUnit {
            class_dat_id: crate::ids::DatId::new(0x3c00_0003),
            is_alliance,
            skills: [0; 8],
            on_mission: false,
        });
        let at = world.systems.insert(crate::world::System {
            dat_id: crate::ids::DatId::new(0x9000_0001),
            name: "Base".into(),
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
            special_forces: vec![unit],
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: crate::world::ControlKind::Uncontrolled,
        });
        (unit, at)
    }

    #[test]
    fn a_killed_character_is_refused() {
        // FUN_00522b30 accepts only a member that exists. port: a killed
        // character stays in the arena (Character::mark_killed), so it counts
        // as gone.
        let mut world = minimal_world();
        let (here, _) = two_systems();
        let dead = agent_at(&mut world, here, true);
        world.characters[dead].mark_killed();
        let mut state = MissionState::new();
        let member = MissionMember::Character(dead);

        assert_eq!(
            state.dispatch_guarded(request(vec![member], vec![], here), &mut world),
            Err(MissionRefusal::MemberUnavailable(member))
        );
        assert!(state.is_empty());
        assert!(!world.characters[dead].on_mission);
    }

    #[test]
    fn a_member_that_no_longer_exists_is_refused_and_no_flag_is_set() {
        // FUN_00522b30 accepts only a member that exists; flags are set only
        // after every member passes.
        let mut world = minimal_world();
        let (here, _) = two_systems();
        let lead = agent_at(&mut world, here, true);
        let gone = agent_at(&mut world, here, true);
        world.characters.remove(gone);
        let mut state = MissionState::new();
        let team = vec![
            MissionMember::Character(lead),
            MissionMember::Character(gone),
        ];

        assert_eq!(
            state.dispatch_guarded(request(team, vec![], here), &mut world),
            Err(MissionRefusal::MemberUnavailable(MissionMember::Character(
                gone
            )))
        );
        assert!(!world.characters[lead].on_mission);
    }

    #[test]
    fn a_special_force_already_on_a_mission_is_refused_and_no_flag_is_set() {
        // FUN_00522b30: a member must have no mission yet (+0x68 key empty).
        let mut world = minimal_world();
        let (unit, at) = unit_on_surface(&mut world, true);
        world.special_forces[unit].on_mission = true;
        let lead = agent_at(&mut world, at, true);
        let mut state = MissionState::new();
        let decoy = MissionMember::SpecialForce(unit);

        assert_eq!(
            state.dispatch_guarded(
                request(vec![MissionMember::Character(lead)], vec![decoy], at),
                &mut world
            ),
            Err(MissionRefusal::MemberUnavailable(decoy))
        );
        assert!(!world.characters[lead].on_mission);
    }

    #[test]
    fn a_member_held_on_another_missions_captured_list_is_busy() {
        // FUN_00522b30: a member must have no mission yet; a captured member
        // has one (FUN_0054c200 adds the captured list too).
        let mut world = minimal_world();
        let (here, _) = two_systems();
        let prisoner = MissionMember::Character(agent_at(&mut world, here, true));
        let mut state = MissionState::new();
        let mut held = ActiveMission::timed(
            0,
            MissionKind::Rescue,
            MissionFaction::Alliance,
            vec![MissionMember::Character(agent_at(&mut world, here, true))],
            here,
            5,
        );
        held.captured = vec![prisoner];
        state.missions.push_back(held);
        state.next_id = 1;

        assert_eq!(
            state.dispatch_guarded(request(vec![prisoner], vec![], here), &mut world),
            Err(MissionRefusal::MemberUnavailable(prisoner))
        );
    }

    #[test]
    fn a_prisoner_named_as_a_decoy_goes_to_the_captured_list_not_the_decoys() {
        // FUN_0054bb90 moves a prisoner out of the decoy list before any add,
        // so FUN_00522b30 never sees a prisoner as a decoy; it records the
        // mission on every member it accepts (FUN_00534230).
        let mut world = minimal_world();
        let (here, _) = two_systems();
        let lead = agent_at(&mut world, here, true);
        let prisoner = agent_at(&mut world, here, true);
        world.characters[prisoner].is_captive = true;
        let mut state = MissionState::new();

        state
            .dispatch_guarded(
                request(
                    vec![MissionMember::Character(lead)],
                    vec![MissionMember::Character(prisoner)],
                    here,
                ),
                &mut world,
            )
            .expect("dispatch");
        let mission = &state.missions()[0];
        assert!(mission.decoys.is_empty());
        assert_eq!(mission.captured, vec![MissionMember::Character(prisoner)]);
        assert!(world.characters[prisoner].on_mission);
    }

    #[test]
    fn the_first_free_team_member_sets_the_location_even_after_a_prisoner() {
        // FUN_0054bb90 moves prisoners out first; FUN_0054c200 then adds the
        // team before the captured list, so a prisoner held elsewhere is the
        // member refused, not the free lead named after it.
        let mut world = minimal_world();
        let (here, there) = two_systems();
        let prisoner = agent_at(&mut world, there, true);
        world.characters[prisoner].is_captive = true;
        let lead = agent_at(&mut world, here, true);
        let mut state = MissionState::new();
        let team = vec![
            MissionMember::Character(prisoner),
            MissionMember::Character(lead),
        ];

        assert_eq!(
            state.dispatch_guarded(request(team, vec![], here), &mut world),
            Err(MissionRefusal::MemberUnavailable(MissionMember::Character(
                prisoner
            )))
        );
    }

    #[test]
    fn a_fleet_character_and_a_surface_character_at_one_system_are_refused() {
        // FUN_00522b30 compares the location key (slot +0xc): a fleet in orbit
        // and the system below it are different locations.
        let mut world = minimal_world();
        let (here, _) = two_systems();
        let mut fleets: slotmap::SlotMap<crate::ids::FleetKey, ()> = slotmap::SlotMap::with_key();
        let aboard = agent_at(&mut world, here, true);
        world.characters[aboard].current_fleet = Some(fleets.insert(()));
        let below = agent_at(&mut world, here, true);
        let mut state = MissionState::new();
        let team = vec![
            MissionMember::Character(aboard),
            MissionMember::Character(below),
        ];

        assert_eq!(
            state.dispatch_guarded(request(team, vec![], here), &mut world),
            Err(MissionRefusal::MemberUnavailable(MissionMember::Character(
                below
            )))
        );
    }

    #[test]
    fn a_special_force_decoy_joins_the_decoys_and_is_freed_on_release() {
        // FUN_0054c200 adds the decoy list with as_decoy = 1 (FUN_00522b30
        // accepts families 0x30..0x3f); releasing clears each member's flag.
        let mut world = minimal_world();
        let (here, _) = two_systems();
        let lead = agent_at(&mut world, here, true);
        let (unit, at) = unit_on_surface(&mut world, true);
        world.characters[lead].current_system = Some(at);
        let mut state = MissionState::new();
        let decoy = MissionMember::SpecialForce(unit);

        let id = state
            .dispatch_guarded(
                request(vec![MissionMember::Character(lead)], vec![decoy], at),
                &mut world,
            )
            .expect("dispatch");
        assert_eq!(state.missions()[0].decoys, vec![decoy]);
        assert!(world.special_forces[unit].on_mission);

        state.release(id, &mut world);
        assert!(!world.special_forces[unit].on_mission);
        assert!(!world.characters[lead].on_mission);
    }

    // --- Phase 10: the per-member roll (FUN_00592f50) ---

    /// A table whose every row is `percent`.
    fn always(world: &mut GameWorld, kind: MissionKind, percent: u32) {
        world.mission_tables.insert(
            kind.mstb_key().unwrap().to_string(),
            crate::world::MstbTable::new(vec![crate::world::MstbEntry {
                threshold: 0,
                value: percent,
            }]),
        );
    }

    /// Run phase 10 of a `kind` mission by `team` at `target` on day 1.
    fn phase_ten(
        world: &GameWorld,
        kind: MissionKind,
        team: Vec<MissionMember>,
        target: SystemKey,
        target_character: Option<CharacterKey>,
    ) -> MissionResult {
        let mut mission = ActiveMission::timed(0, kind, MissionFaction::Alliance, team, target, 1);
        mission.target_character = target_character;
        let mut state = MissionState::new();
        state.push_timed(mission);
        step(&mut state, world, 1, &[]).results.remove(0)
    }

    fn raised(result: &MissionResult) -> Vec<(CharacterKey, Skill, i32)> {
        result
            .effects
            .iter()
            .filter_map(|effect| match effect {
                MissionEffect::SkillRaised {
                    character,
                    skill,
                    amount,
                } => Some((*character, *skill, *amount)),
                _ => None,
            })
            .collect()
    }

    fn held_by(world: &mut GameWorld, system: SystemKey, side: crate::dat::Faction) {
        world.systems[system].control = crate::world::ControlKind::Controlled(side);
    }

    #[test]
    fn every_team_character_rolls_and_a_success_raises_its_skill_by_gnprtb() {
        // FUN_00592f50 rolls each team character, then each team special
        // force; decoys never roll. FUN_005740a0 raises diplomacy by GNPRTB
        // 6156 (shipped 1) through slot +0x1d8, which a special force lacks
        // (FUN_006158b0).
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        always(&mut world, MissionKind::Diplomacy, 100);
        let first = agent_at(&mut world, here, true);
        let second = agent_at(&mut world, here, true);
        let unit = world.special_forces.insert(crate::world::SpecialForceUnit {
            class_dat_id: crate::ids::DatId::new(0x1400_0001),
            is_alliance: true,
            skills: [0; 8],
            on_mission: true,
        });
        let team = vec![
            MissionMember::SpecialForce(unit),
            MissionMember::Character(first),
            MissionMember::Character(second),
        ];

        let result = phase_ten(&world, MissionKind::Diplomacy, team, here, None);

        assert_eq!(result.outcome, MissionOutcome::Success);
        assert_eq!(
            raised(&result),
            vec![(first, Skill::Diplomacy, 1), (second, Skill::Diplomacy, 1)]
        );
    }

    #[test]
    fn a_failed_roll_leaves_the_result_failed_and_changes_nothing() {
        // Slot +0x280 (FUN_00576700) turns an unset result into 2.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        always(&mut world, MissionKind::Diplomacy, 0);
        let envoy = MissionMember::Character(agent_at(&mut world, here, true));

        let result = phase_ten(&world, MissionKind::Diplomacy, vec![envoy], here, None);

        assert_eq!(result.outcome, MissionOutcome::Failure);
        assert!(result.effects.is_empty());
    }

    #[test]
    fn diplomacy_wins_support_at_its_side_s_or_a_neutral_system_but_not_the_opponent_s() {
        // FUN_00574120 -> FUN_0055cac0: G6183 + rand(0..=G6184) at home,
        // G6185 + rand(0..=G6186) at a neutral system (side 3,
        // FUN_004f8c60), 0 at a system the opponent holds.
        let mut world = minimal_world();
        world.gnprtb =
            crate::world::GnprtbParams::uniform(&[(6183, 4), (6184, 0), (6185, 2), (6186, 0)]);
        let here = system_at(&mut world, 0);
        always(&mut world, MissionKind::Diplomacy, 100);
        let envoy = MissionMember::Character(agent_at(&mut world, here, true));
        let support = |result: &MissionResult| {
            result
                .effects
                .iter()
                .filter(|effect| matches!(effect, MissionEffect::SupportGained { .. }))
                .cloned()
                .collect::<Vec<_>>()
        };

        held_by(&mut world, here, crate::dat::Faction::Alliance);
        let home = phase_ten(&world, MissionKind::Diplomacy, vec![envoy], here, None);
        held_by(&mut world, here, crate::dat::Faction::Empire);
        let away = phase_ten(&world, MissionKind::Diplomacy, vec![envoy], here, None);
        world.systems[here].control = crate::world::ControlKind::Uncontrolled;
        let neutral = phase_ten(&world, MissionKind::Diplomacy, vec![envoy], here, None);

        assert_eq!(
            support(&home),
            vec![MissionEffect::SupportGained {
                system: here,
                side: crate::dat::Faction::Alliance,
                points: 4,
            }]
        );
        assert!(support(&away).is_empty());
        assert_eq!(
            support(&neutral),
            vec![MissionEffect::SupportGained {
                system: here,
                side: crate::dat::Faction::Alliance,
                points: 2,
            }]
        );
        assert_eq!(away.outcome, MissionOutcome::Success);
    }

    #[test]
    fn espionage_raises_espionage_only_at_a_system_its_side_does_not_hold() {
        // FUN_00573540: the raise needs a target of another side; FUN_00573610
        // raises observation at a system target either way.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        always(&mut world, MissionKind::Espionage, 100);
        let spy = agent_at(&mut world, here, true);

        held_by(&mut world, here, crate::dat::Faction::Alliance);
        let home = phase_ten(
            &world,
            MissionKind::Espionage,
            vec![MissionMember::Character(spy)],
            here,
            None,
        );
        world.systems[here].control = crate::world::ControlKind::Uncontrolled;
        let away = phase_ten(
            &world,
            MissionKind::Espionage,
            vec![MissionMember::Character(spy)],
            here,
            None,
        );

        assert!(raised(&home).is_empty());
        assert_eq!(raised(&away), vec![(spy, Skill::Espionage, 1)]);
        for result in [&home, &away] {
            assert!(result
                .effects
                .contains(&MissionEffect::SystemIntelligenceGathered {
                    system: here,
                    faction: MissionFaction::Alliance,
                }));
        }
    }

    #[test]
    fn a_rescue_frees_its_target_once_and_raises_each_successful_member() {
        // FUN_0056ae50: each success raises combat (G6162) and calls the
        // target's slot +0x210; the port frees it once.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        always(&mut world, MissionKind::Rescue, 100);
        let (a, b) = (
            agent_at(&mut world, here, true),
            agent_at(&mut world, here, true),
        );
        let prisoner = agent_at(&mut world, here, true);
        world.characters[prisoner].is_captive = true;
        let team = vec![MissionMember::Character(a), MissionMember::Character(b)];

        let result = phase_ten(&world, MissionKind::Rescue, team, here, Some(prisoner));

        let freed = result
            .effects
            .iter()
            .filter(|effect| matches!(effect, MissionEffect::CharacterRescued { .. }))
            .count();
        assert_eq!(freed, 1);
        assert_eq!(
            raised(&result),
            vec![(a, Skill::Combat, 1), (b, Skill::Combat, 1)]
        );
    }

    #[test]
    fn an_abduction_takes_its_target_prisoner_for_the_mission_s_side() {
        // FUN_00576eb0: the target's slot +0x20c with the member as captor.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        always(&mut world, MissionKind::Abduction, 100);
        let agent = agent_at(&mut world, here, true);
        let target = agent_at(&mut world, here, false);

        let result = phase_ten(
            &world,
            MissionKind::Abduction,
            vec![MissionMember::Character(agent)],
            here,
            Some(target),
        );

        assert_eq!(
            result.effects,
            vec![
                MissionEffect::SkillRaised {
                    character: agent,
                    skill: Skill::Combat,
                    amount: 1
                },
                MissionEffect::CharacterCaptured {
                    character: target,
                    captured_by: MissionFaction::Alliance,
                    at_system: here,
                },
            ]
        );
    }

    #[test]
    fn an_assassination_kills_its_target_once_and_raises_every_successful_member() {
        // FUN_00576620: slot +0x2ec kills; with the target destroyed each
        // success sets 3 and raises combat (G6164).
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        always(&mut world, MissionKind::Assassination, 100);
        let (a, b) = (
            agent_at(&mut world, here, true),
            agent_at(&mut world, here, true),
        );
        let target = agent_at(&mut world, here, false);
        let team = vec![MissionMember::Character(a), MissionMember::Character(b)];

        let result = phase_ten(&world, MissionKind::Assassination, team, here, Some(target));

        let kills = result
            .effects
            .iter()
            .filter(|effect| matches!(effect, MissionEffect::CharacterKilled { .. }))
            .count();
        assert_eq!(kills, 1);
        assert_eq!(
            raised(&result),
            vec![(a, Skill::Combat, 1), (b, Skill::Combat, 1)]
        );
        assert_eq!(result.outcome, MissionOutcome::Success);
    }

    #[test]
    fn an_incite_success_runs_the_incident_and_fails_while_the_holder_keeps_a_regiment() {
        // FUN_00571a60: FUN_0050d030 runs, then the result is 2 while the
        // holder has a regiment there (FUN_00509020), else 3 with a
        // leadership raise (G6160).
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        held_by(&mut world, here, crate::dat::Faction::Empire);
        always(&mut world, MissionKind::InciteUprising, 100);
        let agitator = agent_at(&mut world, here, true);
        let team = || vec![MissionMember::Character(agitator)];
        let incidents = |result: &MissionResult| {
            result
                .effects
                .iter()
                .filter(|effect| matches!(effect, MissionEffect::UprisingIncident(_)))
                .count()
        };

        let open = phase_ten(&world, MissionKind::InciteUprising, team(), here, None);
        let troop = world.troops.insert(crate::world::TroopUnit {
            class_dat_id: crate::ids::DatId::new(0x1000_0007),
            is_alliance: false,
            regiment_strength: 1,
        });
        world.systems[here].ground_units.push(troop);
        let garrisoned = phase_ten(&world, MissionKind::InciteUprising, team(), here, None);

        assert_eq!(open.outcome, MissionOutcome::Success);
        assert_eq!(raised(&open), vec![(agitator, Skill::Leadership, 1)]);
        assert_eq!(garrisoned.outcome, MissionOutcome::Failure);
        assert!(raised(&garrisoned).is_empty());
        assert_eq!((incidents(&open), incidents(&garrisoned)), (1, 1));
    }

    #[test]
    fn a_subdue_member_rolls_only_during_an_uprising_and_succeeds_once_it_ends() {
        // FUN_00569c20: nothing without the uprising bit (+0x88 bit 2); a
        // success adds FUN_0055cb10 support, runs FUN_0050c910, and sets 3
        // only once the uprising ended.
        let mut world = minimal_world();
        world.gnprtb =
            crate::world::GnprtbParams::uniform(&[(6187, 1), (6188, 0), (7761, 60), (7762, -10)]);
        let here = system_at(&mut world, 0);
        held_by(&mut world, here, crate::dat::Faction::Alliance);
        world.systems[here].popularity_alliance = 0.3;
        world.systems[here].popularity_empire = 0.7;
        always(&mut world, MissionKind::SubdueUprising, 100);
        let governor = agent_at(&mut world, here, true);
        let run = |world: &GameWorld, uprisings: &UprisingState| {
            let mut state = MissionState::new();
            state.push_timed(ActiveMission::timed(
                0,
                MissionKind::SubdueUprising,
                MissionFaction::Alliance,
                vec![MissionMember::Character(governor)],
                here,
                1,
            ));
            MissionSystem::advance(&mut state, world, uprisings, &[TickEvent { tick: 1 }], &[])
                .results
                .remove(0)
        };
        let calm = UprisingState::default();
        let mut revolt = UprisingState::default();
        revolt.active_uprisings.insert(
            here,
            crate::uprising::ActiveUprising {
                started_tick: 0,
                next_incident_tick: None,
            },
        );

        let quiet = run(&world, &calm);
        let unended = run(&world, &revolt);
        for _ in 0..3 {
            let troop = world.troops.insert(crate::world::TroopUnit {
                class_dat_id: crate::ids::DatId::new(0x1000_0001),
                is_alliance: true,
                regiment_strength: 1,
            });
            world.systems[here].ground_units.push(troop);
        }
        let ended = run(&world, &revolt);

        assert!(quiet.effects.is_empty());
        assert_eq!(quiet.outcome, MissionOutcome::Failure);
        assert_eq!(unended.outcome, MissionOutcome::Failure);
        assert_eq!(
            unended.effects,
            vec![MissionEffect::UprisingSubdued {
                system: here,
                side: crate::dat::Faction::Alliance,
                support_gain: 1,
            }]
        );
        assert_eq!(ended.outcome, MissionOutcome::Success);
        assert_eq!(raised(&ended), vec![(governor, Skill::Leadership, 1)]);
    }

    #[test]
    fn an_incite_whose_incident_destroys_the_holder_s_last_regiment_succeeds() {
        // FUN_00571a60 tests the holder's regiments after FUN_0050d030: here
        // UPRIS1TB gives code 2 (FUN_0050d150 destroys a regiment), so none
        // is left and the result is 3.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        held_by(&mut world, here, crate::dat::Faction::Empire);
        always(&mut world, MissionKind::InciteUprising, 100);
        world.mission_tables.insert(
            "UPRIS1TB".into(),
            crate::world::MstbTable::new(vec![crate::world::MstbEntry {
                threshold: i32::MIN,
                value: 2,
            }]),
        );
        let troop = world.troops.insert(crate::world::TroopUnit {
            class_dat_id: crate::ids::DatId::new(0x1000_0007),
            is_alliance: false,
            regiment_strength: 1,
        });
        world.systems[here].ground_units.push(troop);
        let agitator = agent_at(&mut world, here, true);

        let result = phase_ten(
            &world,
            MissionKind::InciteUprising,
            vec![MissionMember::Character(agitator)],
            here,
            None,
        );

        assert_eq!(result.outcome, MissionOutcome::Success);
        assert!(result.effects.iter().any(|effect| matches!(
            effect,
            MissionEffect::UprisingIncident(crate::uprising::UprisingEvent::UprisingIncident {
                losses,
                ..
            }) if losses == &[crate::uprising::IncidentLoss::Regiment(troop)]
        )));
    }

    #[test]
    fn both_sabotages_raise_espionage_and_combat_by_their_own_gnprtb() {
        // FUN_0056a300: G6165 and G6166; FUN_00574630: G6167 and G6168.
        let mut world = minimal_world();
        world.gnprtb =
            crate::world::GnprtbParams::uniform(&[(6165, 2), (6166, 3), (6167, 4), (6168, 5)]);
        let here = system_at(&mut world, 0);
        let saboteur = agent_at(&mut world, here, true);
        let mut raises = Vec::new();
        for kind in [MissionKind::Sabotage, MissionKind::DeathStarSabotage] {
            always(&mut world, kind, 100);
            let team = vec![MissionMember::Character(saboteur)];
            raises.push(raised(&phase_ten(&world, kind, team, here, None)));
        }

        assert_eq!(
            raises,
            vec![
                vec![
                    (saboteur, Skill::Espionage, 2),
                    (saboteur, Skill::Combat, 3)
                ],
                vec![
                    (saboteur, Skill::Espionage, 4),
                    (saboteur, Skill::Combat, 5)
                ],
            ]
        );
    }

    #[test]
    fn a_raise_reads_its_gnprtb_value_and_adds_to_the_base_skill() {
        // FUN_005740a0 adds GNPRTB 6156 to the base diplomacy (+0x58) through
        // FUN_00533e20.
        let mut world = minimal_world();
        world.gnprtb = crate::world::GnprtbParams::uniform(&[(6156, 3)]);
        let here = system_at(&mut world, 0);
        always(&mut world, MissionKind::Diplomacy, 100);
        let envoy = agent_at(&mut world, here, true);
        world.characters[envoy].diplomacy = skill_pair(40);

        let result = phase_ten(
            &world,
            MissionKind::Diplomacy,
            vec![MissionMember::Character(envoy)],
            here,
            None,
        );
        assert_eq!(raised(&result), vec![(envoy, Skill::Diplomacy, 3)]);
        raise_skill(&mut world, envoy, Skill::Diplomacy, 3);

        assert_eq!(world.characters[envoy].diplomacy.base, 43);
    }

    // --- Slot +0x274, the phase 10 chance ---

    /// A table whose row `t` holds `t + 200`, so the chance reads back its
    /// input.
    fn echo_table(world: &mut GameWorld, name: &str) {
        let rows = (-200..=200)
            .map(|threshold| crate::world::MstbEntry {
                threshold,
                value: u32::try_from(threshold + 200).unwrap(),
            })
            .collect();
        world
            .mission_tables
            .insert(name.to_string(), crate::world::MstbTable::new(rows));
    }

    fn character_with_skills(
        world: &mut GameWorld,
        espionage: u32,
        combat: u32,
        diplomacy: u32,
        leadership: u32,
    ) -> MissionMember {
        MissionMember::Character(world.characters.insert(Character {
            name: "Agent".into(),
            is_alliance: true,
            diplomacy: skill_pair(diplomacy),
            espionage: skill_pair(espionage),
            combat: skill_pair(combat),
            leadership: skill_pair(leadership),
            can_be_commander: true,
            ..Default::default()
        }))
    }

    #[test]
    fn each_class_reads_its_recovered_table_input() {
        // Slot +0x274 per class (decoy-roll.md, "The success roll"): an
        // Empire-held system at 60 Empire support with two Stormtrooper
        // regiments (FUN_005091f0) against an Alliance member.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        world.systems[here].popularity_empire = 0.6;
        world.systems[here].popularity_alliance = 0.4;
        world.systems[here].control =
            crate::world::ControlKind::Controlled(crate::dat::Faction::Empire);
        for _ in 0..2 {
            let troop = world.troops.insert(crate::world::TroopUnit {
                class_dat_id: crate::ids::DatId::new(0x1000_0006),
                is_alliance: false,
                regiment_strength: 1,
            });
            world.systems[here].ground_units.push(troop);
        }
        for kind in [
            MissionKind::Diplomacy,
            MissionKind::Espionage,
            MissionKind::Rescue,
            MissionKind::Sabotage,
            MissionKind::DeathStarSabotage,
            MissionKind::Recruitment,
            MissionKind::InciteUprising,
            MissionKind::SubdueUprising,
            MissionKind::Abduction,
            MissionKind::Assassination,
        ] {
            echo_table(&mut world, kind.mstb_key().unwrap());
        }
        let agent = character_with_skills(&mut world, 60, 40, 80, 70);
        let target = character_with_skills(&mut world, 0, 25, 0, 0).character();
        let chance =
            |kind| member_chance(&world, kind, MissionFaction::Alliance, here, target, agent) - 200;

        // FUN_0055c680: (stormtroopers - opposing support) + diplomacy.
        assert_eq!(chance(MissionKind::Diplomacy), 2 - 60 + 80);
        assert_eq!(chance(MissionKind::Espionage), 60);
        assert_eq!(chance(MissionKind::Rescue), 40);
        // FUN_0055c890 / FUN_0055c8d0: (espionage + combat) / 2.
        assert_eq!(chance(MissionKind::Sabotage), 50);
        assert_eq!(chance(MissionKind::DeathStarSabotage), 50);
        // FUN_0055c700: leadership - opposing support.
        assert_eq!(chance(MissionKind::Recruitment), 70 - 60);
        // FUN_0055c740: (leadership - opposing support) - stormtroopers.
        assert_eq!(chance(MissionKind::InciteUprising), 70 - 60 - 2);
        // FUN_0055c780: (stormtroopers - opposing support) + leadership.
        assert_eq!(chance(MissionKind::SubdueUprising), 2 - 60 + 70);
        // FUN_0055c810 / FUN_0055c850: combat - target combat.
        assert_eq!(chance(MissionKind::Abduction), 40 - 25);
        assert_eq!(chance(MissionKind::Assassination), 40 - 25);
    }

    #[test]
    fn a_chance_without_its_table_system_or_target_is_zero() {
        // FUN_0053e240 leaves 0 without a row; FUN_00586720 and FUN_00586c80
        // leave the chance unset without a target.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        let elsewhere = system_at(&mut world, 1);
        world.systems.remove(elsewhere);
        let agent = character_with_skills(&mut world, 60, 40, 80, 70);
        let chance = |world: &GameWorld, kind, system| {
            member_chance(world, kind, MissionFaction::Alliance, system, None, agent)
        };
        assert_eq!(chance(&world, MissionKind::Espionage, here), 0);
        echo_table(&mut world, "DIPLMSTB");
        echo_table(&mut world, "ASSNMSTB");
        assert_eq!(chance(&world, MissionKind::Diplomacy, elsewhere), 0);
        assert_eq!(chance(&world, MissionKind::Assassination, here), 0);
        assert_eq!(chance(&world, MissionKind::Diplomacy, here), 200 - 50 + 80);
    }

    #[test]
    fn a_special_force_rolls_on_its_own_skills() {
        // Slots +0x1e0 and +0x1f0 of vtable 0x0065e160 read the unit's base
        // espionage and combat (FUN_00503c40, FUN_00503c80).
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        echo_table(&mut world, "SBTGMSTB");
        let mut skills = [0; 8];
        skills[Skill::Espionage as usize] = 30;
        skills[Skill::Combat as usize] = 50;
        let unit = world.special_forces.insert(crate::world::SpecialForceUnit {
            class_dat_id: crate::ids::DatId::new(0x1400_0001),
            is_alliance: true,
            skills,
            on_mission: false,
        });
        let chance = member_chance(
            &world,
            MissionKind::Sabotage,
            MissionFaction::Alliance,
            here,
            None,
            MissionMember::SpecialForce(unit),
        );
        assert_eq!(chance, 200 + 40);
    }
    // --- Lifecycle (FUN_005227d0, FUN_00524b70, FUN_00522480) ---

    /// A system at `(x, 0)`, populated, held by no one.
    fn system_at(world: &mut GameWorld, x: u16) -> SystemKey {
        world.systems.insert(crate::world::System {
            dat_id: crate::ids::DatId::new(0x9000_0001),
            name: "System".into(),
            sector: crate::ids::SectorKey::default(),
            x,
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
            control: crate::world::ControlKind::Uncontrolled,
        })
    }

    /// The shipped MISSNSD Diplomacy record `0x51000010`: timer (5, 10),
    /// repeating, detection phases and resign on, columns 11..21 =
    /// 1 1 1 1 1 0 0 1 0 0.
    fn diplomacy_record() -> crate::world::MissionRecord {
        crate::world::MissionRecord {
            dat_id: crate::ids::DatId::new(0x5100_0010),
            timer_min_days: 5,
            timer_spread_days: 10,
            repeats: true,
            hidden: false,
            detection_phases: true,
            can_resign: true,
            rules: crate::world::MissionTargetRules {
                container_loss_ends: true,
                needs_populated_container: true,
                target_loss_ends: true,
                own_side_target: true,
                other_side_target: true,
                opponent_target: false,
                revolting_target: false,
                calm_target: true,
                prisoner_target: false,
                free_target: false,
            },
            members: crate::world::MissionMemberRules {
                alliance: true,
                empire: true,
                special_force_mask: 0,
                character_mask: 0x1_0000,
            },
        }
    }

    /// The shipped MISSNSD Rescue record `0x61000011`: timer (1, 6), not
    /// repeating, detection phases and resign on, columns 11..21 =
    /// 0 0 1 0 0 1 0 0 1 0.
    fn rescue_record() -> crate::world::MissionRecord {
        crate::world::MissionRecord {
            dat_id: crate::ids::DatId::new(0x6100_0011),
            timer_min_days: 1,
            timer_spread_days: 6,
            repeats: false,
            hidden: false,
            detection_phases: true,
            can_resign: true,
            rules: crate::world::MissionTargetRules {
                target_loss_ends: true,
                opponent_target: true,
                prisoner_target: true,
                ..Default::default()
            },
            members: crate::world::MissionMemberRules {
                alliance: true,
                empire: true,
                special_force_mask: 0x802,
                character_mask: 0x1_0000,
            },
        }
    }

    fn step(
        state: &mut MissionState,
        world: &GameWorld,
        tick: u64,
        rolls: &[f64],
    ) -> MissionAdvance {
        MissionSystem::advance(
            state,
            world,
            &UprisingState::default(),
            &[TickEvent { tick }],
            rolls,
        )
    }

    fn diplomacy_from(
        world: &mut GameWorld,
        from: SystemKey,
        to: SystemKey,
    ) -> (MissionState, CharacterKey) {
        world.mission_records = vec![diplomacy_record()];
        let envoy = agent_at(world, from, true);
        let mut state = MissionState::new();
        state
            .dispatch_guarded(
                MissionRequest::single(
                    MissionKind::Diplomacy,
                    MissionFaction::Alliance,
                    envoy,
                    to,
                    None,
                    0,
                ),
                world,
            )
            .expect("a free envoy");
        (state, envoy)
    }

    #[test]
    fn a_new_mission_steps_to_the_transit_phase_and_sends_its_members_off_the_map() {
        // FUN_00524b70 phase 4: FUN_00556430 starts each member's transit
        // from the origin +0x6c; the phase waits for the arrivals.
        let mut world = minimal_world();
        let from = system_at(&mut world, 0);
        let to = system_at(&mut world, 100);
        let (mut state, envoy) = diplomacy_from(&mut world, from, to);

        let advance = step(&mut state, &world, 1, &[]);

        let mission = &state.missions()[0];
        assert_eq!((mission.phase, mission.ready), (PHASE_TRANSIT, false));
        assert_eq!(mission.origin, Some(from));
        let member = MissionMember::Character(envoy);
        assert_eq!(
            advance.effects,
            vec![MissionEffect::MemberMoved { member, to: None }]
        );
        // isqrt(100^2) / GNPRTB 5120 (5) * 100 / 100 = 20 days from day 0.
        assert_eq!(
            state.en_route(),
            &[MemberTransit {
                member,
                mission_id: mission.id,
                to,
                arrival: 20
            }]
        );
        assert_eq!(
            mission.days_left(1, state.last_arrival(mission.id)),
            Some(19)
        );
    }

    #[test]
    fn the_mission_waits_for_its_last_arrival_then_arms_the_record_timer() {
        // FUN_00545820 -> FUN_00522280 sets the ready bit once no member is
        // en route; phase 8 arms timer 0x38b for min + rand(0..=spread)
        // (FUN_005236e0, FUN_00586130): 5 + floor(0.3 * 11) = 8 days.
        let mut world = minimal_world();
        let from = system_at(&mut world, 0);
        let to = system_at(&mut world, 100);
        let (mut state, envoy) = diplomacy_from(&mut world, from, to);
        step(&mut state, &world, 1, &[]);

        assert!(step(&mut state, &world, 19, &[]).effects.is_empty());
        assert_eq!(state.missions()[0].phase, PHASE_TRANSIT);

        let advance = step(&mut state, &world, 20, &[0.3]);
        assert_eq!(
            advance.effects,
            vec![MissionEffect::MemberMoved {
                member: MissionMember::Character(envoy),
                to: Some(to)
            }]
        );
        assert!(state.en_route().is_empty());
        let mission = &state.missions()[0];
        assert_eq!(mission.phase, PHASE_TIMER);
        assert_eq!(mission.timer_due, Some(28));
    }

    #[test]
    fn members_at_the_target_land_at_once_and_the_timer_starts_on_the_order_day() {
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        let (mut state, envoy) = diplomacy_from(&mut world, here, here);

        let advance = step(&mut state, &world, 1, &[0.0]);

        assert_eq!(
            advance.effects,
            vec![MissionEffect::MemberMoved {
                member: MissionMember::Character(envoy),
                to: Some(here)
            }]
        );
        assert!(state.en_route().is_empty());
        assert_eq!(state.missions()[0].timer_due, Some(5));
    }

    #[test]
    fn a_repeating_mission_returns_to_the_timer_after_each_outcome() {
        // FUN_005227d0: phase 10 steps to 8 when record +0x58 is set.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        let (mut state, _) = diplomacy_from(&mut world, here, here);
        step(&mut state, &world, 1, &[0.0]);

        // Outcome 0.99, then the new timer draw 1.0: 5 + 10 days.
        let advance = step(&mut state, &world, 5, &[0.99, 1.0]);

        assert_eq!(advance.results.len(), 1);
        assert!(advance.ended.is_empty());
        let mission = &state.missions()[0];
        assert_eq!(mission.phase, PHASE_TIMER);
        assert_eq!(mission.timer_due, Some(20));
    }

    #[test]
    fn a_mission_that_runs_its_course_ends_with_code_one_and_frees_its_members() {
        // FUN_00522480 in phase 0xb with no code sets end code 1; the
        // Rescue record does not repeat.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        world.mission_records = vec![rescue_record()];
        let hero = agent_at(&mut world, here, true);
        let prisoner = agent_at(&mut world, here, true);
        world.characters[prisoner].is_captive = true;
        world.characters[prisoner].captured_by = Some(crate::dat::Faction::Empire);
        let mut state = MissionState::new();
        state
            .dispatch_guarded(
                MissionRequest::single(
                    MissionKind::Rescue,
                    MissionFaction::Alliance,
                    hero,
                    here,
                    Some(prisoner),
                    0,
                ),
                &mut world,
            )
            .expect("a free hero");
        // 1 + floor(1.0 * 7).min(6) = 7 days.
        step(&mut state, &world, 1, &[1.0]);
        assert_eq!(state.missions()[0].timer_due, Some(7));

        let advance = step(&mut state, &world, 7, &[0.99]);

        assert_eq!(advance.results.len(), 1);
        assert_eq!(advance.ended.len(), 1);
        assert_eq!(advance.ended[0].end_code, 1);
        assert!(advance.effects.contains(&MissionEffect::MemberAvailable {
            member: MissionMember::Character(hero)
        }));
        assert!(state.is_empty());
    }

    /// A Rescue of an Alliance prisoner held by the Empire, past its
    /// creation steps and waiting for its timer on day 7.
    fn waiting_rescue(world: &mut GameWorld) -> (MissionState, CharacterKey) {
        let here = system_at(world, 0);
        world.mission_records = vec![rescue_record()];
        let hero = agent_at(world, here, true);
        let prisoner = agent_at(world, here, true);
        world.characters[prisoner].is_captive = true;
        world.characters[prisoner].captured_by = Some(crate::dat::Faction::Empire);
        let mut state = MissionState::new();
        state
            .dispatch_guarded(
                MissionRequest::single(
                    MissionKind::Rescue,
                    MissionFaction::Alliance,
                    hero,
                    here,
                    Some(prisoner),
                    0,
                ),
                world,
            )
            .expect("a free hero");
        step(&mut state, world, 1, &[1.0]);
        (state, prisoner)
    }

    #[test]
    fn a_rescue_ends_with_code_eight_once_its_target_is_back_on_its_side() {
        // FUN_00592600 rule 3: a freed prisoner reads as the mission's own
        // side (inference), and Rescue's column 15 is 0.
        let mut world = minimal_world();
        let (mut state, prisoner) = waiting_rescue(&mut world);
        world.characters[prisoner].is_captive = false;
        world.characters[prisoner].captured_by = None;

        // A live target changing sides is seen only on a phase change.
        assert!(step(&mut state, &world, 2, &[]).ended.is_empty());
        let advance = step(&mut state, &world, 7, &[0.0]);

        assert!(advance.results.is_empty());
        assert_eq!(advance.ended[0].end_code, 8);
    }

    #[test]
    fn a_rescue_ends_with_code_six_when_its_opposing_target_is_not_a_prisoner() {
        // FUN_00586e20: a free target needs column 21, which Rescue lacks.
        let mut world = minimal_world();
        let (mut state, target) = waiting_rescue(&mut world);
        let officer = &mut world.characters[target];
        officer.is_captive = false;
        officer.captured_by = None;
        officer.is_alliance = false;
        officer.is_empire = true;

        let advance = step(&mut state, &world, 7, &[0.0]);

        assert!(advance.results.is_empty());
        assert_eq!(advance.ended[0].end_code, 6);
    }

    #[test]
    fn diplomacy_ends_with_code_0xf_once_its_side_holds_full_support() {
        // FUN_00573ee0: FUN_00507270 == 100 ends the mission.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        let (mut state, _) = diplomacy_from(&mut world, here, here);
        step(&mut state, &world, 1, &[0.0]);
        world.systems[here].popularity_alliance = 1.0;

        let advance = step(&mut state, &world, 5, &[0.0]);

        assert!(advance.results.is_empty());
        assert_eq!(advance.ended[0].end_code, 0xf);
    }

    /// MISSNSD Recruitment `0x55000016`, columns 11..21 = 1 1 1 1 0 0 1 1
    /// 0 0 (`ghidra/notes/mission-lifecycle.md`). port: the timer is
    /// Diplomacy's, which these tests do not read.
    fn recruitment_record() -> crate::world::MissionRecord {
        crate::world::MissionRecord {
            dat_id: crate::ids::DatId::new(0x5500_0016),
            rules: crate::world::MissionTargetRules {
                other_side_target: false,
                revolting_target: true,
                ..diplomacy_record().rules
            },
            ..diplomacy_record()
        }
    }

    /// An Alliance character outside the side's roster (`+0x50` bit 1 clear).
    fn pool_character(world: &mut GameWorld, dat_id: u32) -> CharacterKey {
        world.characters.insert(Character {
            dat_id: crate::ids::DatId::new(dat_id),
            is_alliance: true,
            ..Default::default()
        })
    }

    fn recruits(result: &MissionResult) -> Vec<(CharacterKey, bool)> {
        result
            .effects
            .iter()
            .filter_map(|effect| match effect {
                MissionEffect::CharacterRecruited {
                    character,
                    pool_emptied,
                    ..
                } => Some((*character, *pool_emptied)),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn a_recruiter_signs_one_of_its_side_s_unrecruited_characters_and_gains_leadership() {
        // FUN_0056b9a0 -> FUN_0055fc80: a random minor character (families
        // 0x38..0x3c) of the side with +0x50 bit 1 clear (FUN_0055ef30); a
        // killed, major, or enemy one is not in the pool. The raise is
        // GNPRTB 6159.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        always(&mut world, MissionKind::Recruitment, 100);
        let recruiter = agent_at(&mut world, here, true);
        let first = pool_character(&mut world, 2);
        let second = pool_character(&mut world, 3);
        let dead = pool_character(&mut world, 4);
        world.characters[dead].is_killed = true;
        let enemy = pool_character(&mut world, 5);
        world.characters[enemy].is_alliance = false;
        world.characters[enemy].is_empire = true;
        let major = pool_character(&mut world, 6);
        world.characters[major].is_major = true;

        let result = phase_ten(
            &world,
            MissionKind::Recruitment,
            vec![MissionMember::Character(recruiter)],
            here,
            None,
        );

        let signed = recruits(&result);
        assert_eq!(signed.len(), 1);
        assert!([first, second].contains(&signed[0].0));
        assert!(!signed[0].1, "one pool character is left");
        assert_eq!(result.outcome, MissionOutcome::Success);
        assert_eq!(raised(&result), vec![(recruiter, Skill::Leadership, 1)]);
    }

    #[test]
    fn each_successful_recruiter_signs_a_different_character_and_the_last_empties_the_pool() {
        // FUN_0055fc80 sets side +0xb8 (FUN_0052f590) when it takes the
        // last one; a recruiter that finds the pool empty signs no one.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        always(&mut world, MissionKind::Recruitment, 100);
        let team = (0..3)
            .map(|_| MissionMember::Character(agent_at(&mut world, here, true)))
            .collect();
        let first = pool_character(&mut world, 2);
        let second = pool_character(&mut world, 3);

        let result = phase_ten(&world, MissionKind::Recruitment, team, here, None);

        let signed = recruits(&result);
        assert_eq!(signed.len(), 2);
        assert!(signed.iter().any(|&(c, _)| c == first));
        assert!(signed.iter().any(|&(c, _)| c == second));
        assert_eq!(signed.iter().filter(|&&(_, emptied)| emptied).count(), 1);
        assert!(signed[1].1, "the second pick takes the last one");
        assert_eq!(raised(&result).len(), 2);
    }

    #[test]
    fn a_killed_major_or_enemy_character_is_never_recruited() {
        // FUN_0055ef30 counts only the side's minor characters (families
        // 0x38..0x3c through FUN_0056f450) with +0x50 bit 1 clear.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        always(&mut world, MissionKind::Recruitment, 100);
        let recruiter = agent_at(&mut world, here, true);
        let dead = pool_character(&mut world, 2);
        world.characters[dead].is_killed = true;
        let major = pool_character(&mut world, 3);
        world.characters[major].is_major = true;
        let enemy = pool_character(&mut world, 4);
        world.characters[enemy].is_alliance = false;
        world.characters[enemy].is_empire = true;
        let signed = pool_character(&mut world, 5);
        world.characters[signed].recruited = true;

        let result = phase_ten(
            &world,
            MissionKind::Recruitment,
            vec![MissionMember::Character(recruiter)],
            here,
            None,
        );

        assert!(recruits(&result).is_empty());
    }

    #[test]
    fn a_recruiter_with_an_empty_pool_fails_without_a_raise() {
        // FUN_0056b9a0 sets 3 only after a recruit; slot +0x280 turns the
        // unset result into 2.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        always(&mut world, MissionKind::Recruitment, 100);
        let recruiter = agent_at(&mut world, here, true);

        let result = phase_ten(
            &world,
            MissionKind::Recruitment,
            vec![MissionMember::Character(recruiter)],
            here,
            None,
        );

        assert!(recruits(&result).is_empty());
        assert!(raised(&result).is_empty());
        assert_eq!(result.outcome, MissionOutcome::Failure);
    }

    #[test]
    fn two_recruitments_in_one_step_cannot_sign_the_same_character() {
        // port: the world applies recruits after the step, so a later
        // mission skips the pick of an earlier one, as FUN_0055fc80 would
        // skip its +0x50 bit 1.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        always(&mut world, MissionKind::Recruitment, 100);
        let only = pool_character(&mut world, 2);
        let mut state = MissionState::new();
        for id in 0..2 {
            let recruiter = agent_at(&mut world, here, true);
            state.push_timed(ActiveMission::timed(
                id,
                MissionKind::Recruitment,
                MissionFaction::Alliance,
                vec![MissionMember::Character(recruiter)],
                here,
                1,
            ));
        }

        let advance = step(&mut state, &world, 1, &[]);

        let signed: Vec<_> = advance.results.iter().flat_map(recruits).collect();
        assert_eq!(signed, vec![(only, true)]);
    }

    #[test]
    fn a_recruit_joins_at_the_target_and_an_emptied_pool_marks_its_side() {
        // FUN_0055fe70: slot +0xa8 places it, FUN_004f7480 sets +0x50 bit
        // 1; FUN_0052f590 sets the side's +0xb8.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        let recruit = pool_character(&mut world, 2);
        let other = pool_character(&mut world, 3);

        recruit_character(&mut world, recruit, here, MissionFaction::Alliance, false);
        assert!(!world.recruit_pool_empty(crate::dat::Faction::Alliance));
        recruit_character(&mut world, other, here, MissionFaction::Alliance, true);

        let c = &world.characters[recruit];
        assert!(c.recruited);
        assert_eq!(c.current_system, Some(here));
        assert!(world.recruit_pool_empty(crate::dat::Faction::Alliance));
        assert!(!world.recruit_pool_empty(crate::dat::Faction::Empire));
    }

    #[test]
    fn recruitment_ends_with_code_0x10_once_its_side_recruited_the_last_pool_character() {
        // FUN_0056b370: after the system rules, side +0xb8 ends it with 0x10.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        held_by(&mut world, here, crate::dat::Faction::Alliance);
        world.mission_records = vec![recruitment_record()];
        let recruiter = agent_at(&mut world, here, true);
        let mut state = MissionState::new();
        state
            .dispatch_guarded(
                MissionRequest::single(
                    MissionKind::Recruitment,
                    MissionFaction::Alliance,
                    recruiter,
                    here,
                    None,
                    0,
                ),
                &mut world,
            )
            .expect("a free recruiter");
        step(&mut state, &world, 1, &[0.0]);
        assert!(step(&mut state, &world, 2, &[0.0]).ended.is_empty());
        world.set_recruit_pool_empty(crate::dat::Faction::Alliance);

        let advance = step(&mut state, &world, 5, &[0.0]);

        assert!(advance.results.is_empty());
        assert_eq!(advance.ended[0].end_code, 0x10);
    }

    /// MISSNSD Sabotage `0x69000012`, columns 11..21 = 0 0 1 0 1 1 0 0 0 0
    /// (`ghidra/notes/mission-lifecycle.md`). port: the timer is
    /// Diplomacy's, which these tests do not read.
    fn sabotage_record() -> crate::world::MissionRecord {
        crate::world::MissionRecord {
            dat_id: crate::ids::DatId::new(0x6900_0012),
            rules: crate::world::MissionTargetRules {
                container_loss_ends: false,
                needs_populated_container: false,
                target_loss_ends: true,
                own_side_target: false,
                other_side_target: true,
                opponent_target: true,
                revolting_target: false,
                calm_target: false,
                prisoner_target: false,
                free_target: false,
            },
            members: crate::world::MissionMemberRules {
                alliance: true,
                empire: true,
                special_force_mask: 0x402,
                character_mask: 0x1_0000,
            },
            ..diplomacy_record()
        }
    }

    /// An Empire shipyard at `system`.
    fn enemy_yard(
        world: &mut GameWorld,
        system: SystemKey,
    ) -> crate::ids::ManufacturingFacilityKey {
        let key =
            world
                .manufacturing_facilities
                .insert(crate::world::ManufacturingFacilityInstance {
                    class_dat_id: crate::ids::DatId::new(0x2800_0001),
                    side: crate::dat::Faction::Empire,
                    is_shipyard: true,
                });
        world.systems[system].manufacturing_facilities.push(key);
        key
    }

    /// An Empire fleet at `system` with a Star Destroyer and the Death Star.
    fn death_star_fleet(world: &mut GameWorld, system: SystemKey) -> crate::ids::FleetKey {
        let destroyer = world
            .capital_ship_classes
            .insert(crate::world::CapitalShipClass::default());
        let death_star = world
            .capital_ship_classes
            .insert(crate::world::CapitalShipClass {
                dat_id: crate::ids::DatId::new(crate::world::DEATH_STAR_CLASS_ID),
                ..Default::default()
            });
        let fleet = world.fleets.insert(crate::world::Fleet {
            location: system,
            capital_ships: vec![
                crate::world::ShipInstance::new(destroyer, 100, false),
                crate::world::ShipInstance::new(death_star, 100, false),
            ],
            fighters: vec![],
            characters: vec![],
            is_alliance: false,
            has_death_star: true,
        });
        world.systems[system].fleets.push(fleet);
        fleet
    }

    fn sabotage_request(
        world: &mut GameWorld,
        kind: MissionKind,
        at: SystemKey,
        target: Option<MissionTarget>,
    ) -> MissionRequest {
        let agent = agent_at(world, at, true);
        MissionRequest {
            target_object: target,
            ..MissionRequest::single(kind, MissionFaction::Alliance, agent, at, None, 0)
        }
    }

    #[test]
    fn sabotage_needs_an_object_other_than_the_death_star_and_ds_sabotage_needs_the_death_star() {
        // FUN_0056a110 refuses a family 0x18..0x1c target (0x40/0x28);
        // FUN_005744c0 refuses any other (0x40/0x29). port: every order
        // names an object, so none is refused too.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        let yard = MissionTarget::ManufacturingFacility(enemy_yard(&mut world, here));
        let death_star = MissionTarget::DeathStar(death_star_fleet(&mut world, here));
        let mut state = MissionState::new();
        for (kind, target, allowed) in [
            (MissionKind::Sabotage, None, false),
            (MissionKind::Sabotage, Some(death_star), false),
            (MissionKind::Sabotage, Some(yard), true),
            (MissionKind::DeathStarSabotage, None, false),
            (MissionKind::DeathStarSabotage, Some(yard), false),
            (MissionKind::DeathStarSabotage, Some(death_star), true),
        ] {
            let request = sabotage_request(&mut world, kind, here, target);
            let outcome = state.dispatch_guarded(request, &mut world);
            assert_eq!(
                outcome.is_ok(),
                allowed,
                "{kind:?} naming {target:?}: {outcome:?}"
            );
            if !allowed {
                assert_eq!(outcome, Err(MissionRefusal::TargetUnavailable));
            }
        }
    }

    #[test]
    fn a_successful_sabotage_destroys_its_target_object_once() {
        // FUN_005746e0 runs once after every member rolled: on 3, slot
        // +0xac(6) destroys the order's object (FUN_00521030).
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        always(&mut world, MissionKind::Sabotage, 100);
        let yard = enemy_yard(&mut world, here);
        let team = (0..2)
            .map(|_| MissionMember::Character(agent_at(&mut world, here, true)))
            .collect();
        let mut mission = ActiveMission::timed(
            0,
            MissionKind::Sabotage,
            MissionFaction::Alliance,
            team,
            here,
            1,
        );
        mission.target_object = Some(MissionTarget::ManufacturingFacility(yard));
        let mut state = MissionState::new();
        state.push_timed(mission);

        let result = step(&mut state, &world, 1, &[]).results.remove(0);

        let destroyed: Vec<_> = result
            .effects
            .iter()
            .filter_map(|effect| match effect {
                MissionEffect::TargetSabotaged { target, system } => Some((*target, *system)),
                _ => None,
            })
            .collect();
        assert_eq!(
            destroyed,
            vec![(MissionTarget::ManufacturingFacility(yard), here)]
        );
        assert_eq!(result.outcome, MissionOutcome::Success);
    }

    #[test]
    fn a_failed_sabotage_destroys_nothing() {
        // FUN_005746e0 turns the unset result into 2 and destroys only on 3.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        always(&mut world, MissionKind::Sabotage, 0);
        let yard = enemy_yard(&mut world, here);
        let agent = agent_at(&mut world, here, true);
        let mut mission = ActiveMission::timed(
            0,
            MissionKind::Sabotage,
            MissionFaction::Alliance,
            vec![MissionMember::Character(agent)],
            here,
            1,
        );
        mission.target_object = Some(MissionTarget::ManufacturingFacility(yard));
        let mut state = MissionState::new();
        state.push_timed(mission);

        let result = step(&mut state, &world, 1, &[]).results.remove(0);

        assert!(!result
            .effects
            .iter()
            .any(|effect| matches!(effect, MissionEffect::TargetSabotaged { .. })));
        assert_eq!(result.outcome, MissionOutcome::Failure);
    }

    #[test]
    fn a_destroyed_object_leaves_its_system_and_the_world() {
        // Slot +0xac(6) destroys the object; each kind leaves its system's
        // list and its arena.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        let yard = enemy_yard(&mut world, here);
        let turret = world
            .defense_facilities
            .insert(crate::world::DefenseFacilityInstance {
                class_dat_id: crate::ids::DatId::new(0x2200_0001),
                side: crate::dat::Faction::Empire,
            });
        world.systems[here].defense_facilities.push(turret);
        let mine = world
            .production_facilities
            .insert(crate::world::ProductionFacilityInstance {
                class_dat_id: crate::ids::DatId::new(0x2c00_0001),
                side: crate::dat::Faction::Empire,
                is_mine: true,
            });
        world.systems[here].production_facilities.push(mine);
        let regiment = world.troops.insert(crate::world::TroopUnit {
            class_dat_id: crate::ids::DatId::new(0x1000_0001),
            is_alliance: false,
            regiment_strength: 10,
        });
        world.systems[here].ground_units.push(regiment);
        let unit = world.special_forces.insert(crate::world::SpecialForceUnit {
            class_dat_id: crate::ids::DatId::new(0x3c00_0001),
            is_alliance: false,
            skills: [0; 8],
            on_mission: false,
        });
        world.systems[here].special_forces.push(unit);

        for target in [
            MissionTarget::ManufacturingFacility(yard),
            MissionTarget::DefenseFacility(turret),
            MissionTarget::ProductionFacility(mine),
            MissionTarget::Troop(regiment),
            MissionTarget::SpecialForce(unit),
        ] {
            assert!(target.state(&world).is_some(), "{target:?} stands");
            destroy_target(&mut world, target);
            assert!(target.state(&world).is_none(), "{target:?} is gone");
        }
        let sys = &world.systems[here];
        assert!(sys.manufacturing_facilities.is_empty());
        assert!(sys.defense_facilities.is_empty());
        assert!(sys.production_facilities.is_empty());
        assert!(sys.ground_units.is_empty());
        assert!(sys.special_forces.is_empty());
        assert!(world.manufacturing_facilities.is_empty());
        assert!(world.troops.is_empty());
    }

    #[test]
    fn ds_sabotage_destroys_only_the_fleet_s_death_star_hull() {
        // FUN_005746e0 destroys the order's object, the Death Star (class
        // 0x88, family 0x18); the fleet's other hulls stay.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        let fleet = death_star_fleet(&mut world, here);
        let target = MissionTarget::DeathStar(fleet);

        destroy_target(&mut world, target);

        let fleet = &world.fleets[fleet];
        assert!(!fleet.has_death_star);
        assert_eq!(fleet.capital_ships.len(), 1);
        assert!(!world.capital_ship_classes[fleet.capital_ships[0].class].is_death_star());
        assert!(target.state(&world).is_none());
    }

    /// A Sabotage of `target` at `here`, waiting in its timer phase.
    fn waiting_sabotage(
        world: &mut GameWorld,
        here: SystemKey,
        target: MissionTarget,
    ) -> MissionState {
        world.mission_records = vec![sabotage_record()];
        let request = sabotage_request(world, MissionKind::Sabotage, here, Some(target));
        let mut state = MissionState::new();
        state
            .dispatch_guarded(request, world)
            .expect("a free saboteur");
        step(&mut state, world, 1, &[0.0]);
        assert!(step(&mut state, world, 2, &[0.0]).ended.is_empty());
        state
    }

    #[test]
    fn a_sabotage_ends_with_code_six_once_its_object_is_destroyed() {
        // FUN_00593500 base rule 4 (col 14): a destroyed target ends 6.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        let yard = enemy_yard(&mut world, here);
        let mut state =
            waiting_sabotage(&mut world, here, MissionTarget::ManufacturingFacility(yard));
        destroy_target(&mut world, MissionTarget::ManufacturingFacility(yard));

        let advance = step(&mut state, &world, 5, &[0.0]);

        assert!(advance.results.is_empty());
        assert_eq!(advance.ended[0].end_code, 6);
    }

    #[test]
    fn a_sabotage_ends_with_code_six_once_its_object_leaves_the_target_system() {
        // Base rules 5 and 6: the target has left its container.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        let there = system_at(&mut world, 50);
        let regiment = world.troops.insert(crate::world::TroopUnit {
            class_dat_id: crate::ids::DatId::new(0x1000_0001),
            is_alliance: false,
            regiment_strength: 10,
        });
        world.systems[here].ground_units.push(regiment);
        let mut state = waiting_sabotage(&mut world, here, MissionTarget::Troop(regiment));
        world.systems[here].ground_units.clear();
        world.systems[there].ground_units.push(regiment);

        let advance = step(&mut state, &world, 5, &[0.0]);

        assert_eq!(advance.ended[0].end_code, 6);
    }

    #[test]
    fn a_sabotage_ends_with_code_eight_once_its_object_belongs_to_its_own_side() {
        // Base rule 3 (col 15): Sabotage may not target its own side.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        let yard = enemy_yard(&mut world, here);
        let mut state =
            waiting_sabotage(&mut world, here, MissionTarget::ManufacturingFacility(yard));
        world.manufacturing_facilities[yard].side = crate::dat::Faction::Alliance;

        let advance = step(&mut state, &world, 5, &[0.0]);

        assert_eq!(advance.ended[0].end_code, 8);
    }

    #[test]
    fn diplomacy_ends_with_code_eight_at_a_system_the_opponent_holds() {
        // FUN_00592600 rule 3: an opponent target reads column 17, 0 for
        // Diplomacy.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        let (mut state, _) = diplomacy_from(&mut world, here, here);
        step(&mut state, &world, 1, &[0.0]);
        world.systems[here].control =
            crate::world::ControlKind::Controlled(crate::dat::Faction::Empire);

        // The validator runs only on a phase change: the mission waits on.
        assert!(step(&mut state, &world, 2, &[]).ended.is_empty());
        let advance = step(&mut state, &world, 5, &[0.0]);

        assert!(advance.results.is_empty());
        assert_eq!(advance.ended[0].end_code, 8);
    }

    #[test]
    fn a_destroyed_target_system_ends_a_waiting_mission_at_once() {
        // FUN_00545240 runs the validator when the container is destroyed;
        // Diplomacy's column 11 ends it with code 7 before the timer fires.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        let (mut state, _) = diplomacy_from(&mut world, here, here);
        step(&mut state, &world, 1, &[0.0]);
        world.systems[here].is_destroyed = true;

        let advance = step(&mut state, &world, 2, &[0.0]);

        assert!(advance.results.is_empty());
        assert_eq!(advance.ended[0].end_code, 7);
    }

    #[test]
    fn a_member_still_travelling_when_its_mission_ends_keeps_travelling() {
        // FUN_00592c80 never moves the members; each transit runs on.
        let mut world = minimal_world();
        let from = system_at(&mut world, 0);
        let to = system_at(&mut world, 100);
        let (mut state, envoy) = diplomacy_from(&mut world, from, to);
        step(&mut state, &world, 1, &[]);
        world.systems[to].is_destroyed = true;

        let advance = step(&mut state, &world, 2, &[]);

        assert_eq!(advance.ended[0].end_code, 7);
        assert!(state.is_en_route(MissionMember::Character(envoy)));
        let arrival = step(&mut state, &world, 20, &[]);
        assert_eq!(
            arrival.effects,
            vec![MissionEffect::MemberMoved {
                member: MissionMember::Character(envoy),
                to: Some(to)
            }]
        );
    }

    #[test]
    fn han_solo_on_the_team_halves_his_mission_s_travel_time() {
        // FUN_00548370: GNPRTB 3083 (50) when Han Solo is a free member
        // and no special force is; 20 days become 10.
        let mut world = minimal_world();
        let from = system_at(&mut world, 0);
        let to = system_at(&mut world, 100);
        let (mut state, envoy) = diplomacy_from(&mut world, from, to);
        world.characters[envoy].dat_id = crate::ids::DatId::new(0x3300_0243);

        step(&mut state, &world, 1, &[]);

        assert_eq!(state.missions()[0].speed, 50);
        assert_eq!(state.en_route()[0].arrival, 10);
    }

    #[test]
    fn a_special_force_on_han_solo_s_mission_keeps_the_default_speed() {
        // FUN_00525dc0 / FUN_00525a00: any special force member cancels
        // the Han Solo speed.
        let mut world = minimal_world();
        world.mission_records = vec![diplomacy_record()];
        let from = system_at(&mut world, 0);
        let to = system_at(&mut world, 100);
        let han = agent_at(&mut world, from, true);
        world.characters[han].dat_id = crate::ids::DatId::new(0x3300_0243);
        let unit = world.special_forces.insert(crate::world::SpecialForceUnit {
            class_dat_id: crate::ids::DatId::new(0x3c00_0003),
            is_alliance: true,
            skills: [0; 8],
            on_mission: false,
        });
        world.systems[from].special_forces.push(unit);
        let mut state = MissionState::new();
        state
            .dispatch_guarded(
                MissionRequest {
                    kind: MissionKind::Diplomacy,
                    faction: MissionFaction::Alliance,
                    team: vec![MissionMember::Character(han)],
                    decoys: vec![MissionMember::SpecialForce(unit)],
                    target_system: to,
                    target_character: None,
                    target_object: None,
                    tick: 0,
                },
                &mut world,
            )
            .expect("both members share a system");

        step(&mut state, &world, 1, &[]);

        assert_eq!(state.missions()[0].speed, 100);
        assert!(state.en_route().iter().all(|transit| transit.arrival == 20));
    }

    #[test]
    fn each_mission_draws_from_its_own_window_of_rolls() {
        // port: ROLLS_PER_MISSION per mission per tick. Each repeating
        // Diplomacy re-arms timer 0x38b (MISSNSD 5 + rand(0..=10)) from the
        // first draw of its own window.
        let (system, _) = mock_keys();
        let mut world = minimal_world();
        world.mission_records = vec![diplomacy_record()];
        let character = agent_at(&mut world, system, true);
        let mut state = MissionState::new();
        for id in 0..3 {
            state.push_timed(ActiveMission::timed(
                id,
                MissionKind::Diplomacy,
                MissionFaction::Alliance,
                vec![MissionMember::Character(character)],
                system,
                1,
            ));
        }
        let mut rolls = vec![0.99; 3 * ROLLS_PER_MISSION];
        rolls[2 * ROLLS_PER_MISSION] = 0.0;

        step(&mut state, &world, 1, &rolls);

        let due: Vec<_> = state.missions().iter().map(|m| m.timer_due).collect();
        assert_eq!(due, vec![Some(16), Some(16), Some(6)]);
    }

    #[test]
    fn a_moved_member_leaves_its_fleet_or_system_and_lands_at_the_target() {
        // FUN_00556390: slot +0xa8 moves the member into the container.
        let mut world = minimal_world();
        let from = system_at(&mut world, 0);
        let to = system_at(&mut world, 100);
        let pilot = agent_at(&mut world, from, true);
        let fleet = world.fleets.insert(crate::world::Fleet {
            location: from,
            capital_ships: vec![],
            fighters: vec![],
            characters: vec![pilot],
            is_alliance: true,
            has_death_star: false,
        });
        world.characters[pilot].current_fleet = Some(fleet);
        let unit = world.special_forces.insert(crate::world::SpecialForceUnit {
            class_dat_id: crate::ids::DatId::new(0x3c00_0003),
            is_alliance: true,
            skills: [0; 8],
            on_mission: true,
        });
        world.systems[from].special_forces.push(unit);

        move_member(&mut world, MissionMember::Character(pilot), None);
        move_member(&mut world, MissionMember::SpecialForce(unit), None);
        assert!(world.fleets[fleet].characters.is_empty());
        assert_eq!(world.characters[pilot].current_fleet, None);
        assert_eq!(world.characters[pilot].current_system, None);
        assert!(world.systems[from].special_forces.is_empty());

        move_member(&mut world, MissionMember::Character(pilot), Some(to));
        move_member(&mut world, MissionMember::SpecialForce(unit), Some(to));
        assert_eq!(world.characters[pilot].current_system, Some(to));
        assert_eq!(world.systems[to].special_forces, vec![unit]);
    }

    /// An Assassination of an Empire officer, with the shipped Assassination
    /// columns but a 10-day timer, waiting from day 1.
    fn waiting_assassination(
        world: &mut GameWorld,
        with_record: bool,
    ) -> (MissionState, CharacterKey) {
        let here = system_at(world, 0);
        if with_record {
            world.mission_records = vec![crate::world::MissionRecord {
                dat_id: crate::ids::DatId::new(0x6300_0081),
                timer_min_days: 10,
                timer_spread_days: 0,
                repeats: false,
                hidden: false,
                detection_phases: true,
                can_resign: true,
                rules: crate::world::MissionTargetRules {
                    target_loss_ends: true,
                    opponent_target: true,
                    free_target: true,
                    ..Default::default()
                },
                // port: the shipped record admits only Empire members; this
                // fixture's assassin is Alliance, so both sides are allowed.
                members: crate::world::MissionMemberRules {
                    alliance: true,
                    empire: true,
                    special_force_mask: 0x800,
                    character_mask: 0x1_0000,
                },
            }];
        }
        let assassin = agent_at(world, here, true);
        let officer = agent_at(world, here, false);
        let mut state = MissionState::new();
        state
            .dispatch_guarded(
                MissionRequest::single(
                    MissionKind::Assassination,
                    MissionFaction::Alliance,
                    assassin,
                    here,
                    Some(officer),
                    0,
                ),
                world,
            )
            .expect("a free assassin");
        step(&mut state, world, 1, &[0.0]);
        (state, officer)
    }

    #[test]
    fn a_killed_target_ends_a_waiting_mission_on_the_day_it_dies() {
        // FUN_00545240 runs the validator when the target (+0x70,
        // FUN_00520cb0) is destroyed; column 14 ends it with code 6.
        let mut world = minimal_world();
        let (mut state, officer) = waiting_assassination(&mut world, true);
        world.characters[officer].mark_killed();

        let advance = step(&mut state, &world, 3, &[]);

        assert!(advance.results.is_empty());
        assert_eq!(
            advance
                .ended
                .iter()
                .map(|e| (e.end_code, e.tick))
                .collect::<Vec<_>>(),
            vec![(6, 3)]
        );
    }

    #[test]
    fn only_a_destroyed_container_ends_a_mission_with_code_seven() {
        // FUN_00545240.c:85-111 sets code 7 for a destroyed container; a
        // killed target that the validator lets pass leaves no code.
        // port: without a record the validator checks nothing.
        let mut world = minimal_world();
        let (mut state, officer) = waiting_assassination(&mut world, false);
        world.characters[officer].mark_killed();

        let advance = step(&mut state, &world, 3, &[]);

        assert!(advance.ended.is_empty());
        assert_eq!(state.missions()[0].end_code, 0);
    }

    #[test]
    fn a_mission_that_lands_at_a_changed_target_ends_before_arming_its_timer() {
        // FUN_00524b70 phases 5 and 6 run the validator when the target
        // differs from the origin (FUN_00520bb0): an envoy landing at a
        // system the opponent took on the way ends with code 8.
        let mut world = minimal_world();
        let from = system_at(&mut world, 0);
        let to = system_at(&mut world, 100);
        let (mut state, _) = diplomacy_from(&mut world, from, to);
        step(&mut state, &world, 1, &[]);
        world.systems[to].control =
            crate::world::ControlKind::Controlled(crate::dat::Faction::Empire);

        let advance = step(&mut state, &world, 20, &[0.0]);

        assert_eq!(
            advance
                .ended
                .iter()
                .map(|e| (e.end_code, e.tick))
                .collect::<Vec<_>>(),
            vec![(8, 20)]
        );
    }

    #[test]
    fn the_wait_fraction_follows_the_timer_or_the_last_arrival() {
        let (system, character) = mock_keys();
        let team = vec![MissionMember::Character(character)];
        let mut timed = ActiveMission::timed(
            0,
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            team,
            system,
            10,
        );
        timed.wait_start = 2;
        assert!((timed.wait_fraction(6, None) - 0.5).abs() < 1e-6);
        assert!((timed.wait_fraction(40, None) - 1.0).abs() < 1e-6);
        assert!(!timed.in_transit());

        let mut travelling = timed.clone();
        travelling.phase = PHASE_TRANSIT;
        travelling.timer_due = None;
        assert!(travelling.in_transit());
        assert!((travelling.wait_fraction(4, Some(6)) - 0.5).abs() < 1e-6);
        assert!((travelling.wait_fraction(4, None) - 1.0).abs() < 1e-6);

        let mut due_now = timed;
        due_now.timer_due = Some(2);
        assert!((due_now.wait_fraction(2, None) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn a_recruitment_at_a_friendly_system_runs_its_course() {
        // FUN_00592600 rule 3: an own-side target reads column 15 (1 for
        // Recruitment), not column 16 (0).
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        world.systems[here].control =
            crate::world::ControlKind::Controlled(crate::dat::Faction::Alliance);
        world.mission_records = vec![crate::world::MissionRecord {
            dat_id: crate::ids::DatId::new(0x5500_0016),
            timer_min_days: 5,
            timer_spread_days: 0,
            repeats: false,
            hidden: false,
            detection_phases: true,
            can_resign: true,
            rules: crate::world::MissionTargetRules {
                container_loss_ends: true,
                needs_populated_container: true,
                target_loss_ends: true,
                own_side_target: true,
                revolting_target: true,
                calm_target: true,
                ..Default::default()
            },
            members: crate::world::MissionMemberRules {
                alliance: true,
                empire: true,
                special_force_mask: 0,
                character_mask: 0x1_0000,
            },
        }];
        let recruiter = agent_at(&mut world, here, true);
        let mut state = MissionState::new();
        state
            .dispatch_guarded(
                MissionRequest::single(
                    MissionKind::Recruitment,
                    MissionFaction::Alliance,
                    recruiter,
                    here,
                    None,
                    0,
                ),
                &mut world,
            )
            .expect("a free recruiter");
        step(&mut state, &world, 1, &[0.0]);

        let advance = step(&mut state, &world, 5, &[0.99]);

        assert_eq!(advance.results.len(), 1);
        assert_eq!(advance.ended[0].end_code, 1);
    }

    #[test]
    fn a_target_that_leaves_the_system_ends_the_mission_with_code_six() {
        // FUN_00592600 rule 6: the target's location differs from C.
        let mut world = minimal_world();
        let elsewhere = system_at(&mut world, 50);
        let (mut state, officer) = waiting_assassination(&mut world, true);
        world.characters[officer].current_system = Some(elsewhere);

        let advance = step(&mut state, &world, 11, &[0.0]);

        assert!(advance.results.is_empty());
        assert_eq!(advance.ended[0].end_code, 6);
    }

    #[test]
    fn another_member_s_transit_does_not_count_as_the_target_travelling() {
        // FUN_00592600 rule 5 reads the target's own en route bit.
        let mut world = minimal_world();
        let (mut state, _) = waiting_assassination(&mut world, true);
        let bystander = agent_at(&mut world, state.missions()[0].target_system, true);
        state.en_route.push(MemberTransit {
            member: MissionMember::Character(bystander),
            mission_id: 99,
            to: state.missions()[0].target_system,
            arrival: 100,
        });

        let advance = step(&mut state, &world, 11, &[0.99]);

        assert_eq!(advance.results.len(), 1);
        assert_eq!(advance.ended[0].end_code, 1);
    }

    #[test]
    fn a_mission_whose_team_is_gone_ends_with_code_five() {
        // FUN_00522480.c:42-52: no team member stands without a request once
        // the killed envoy is deleted, so the validator gives 5 before the
        // FUN_005830a0 summary.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        let (mut state, envoy) = diplomacy_from(&mut world, here, here);
        step(&mut state, &world, 1, &[0.0]);
        world.characters[envoy].mark_killed();

        let advance = step(&mut state, &world, 5, &[0.99, 0.0]);

        assert!(advance.results.is_empty());
        assert_eq!(advance.ended[0].end_code, 5);
    }

    #[test]
    fn each_tick_of_an_advance_gives_every_mission_its_own_window() {
        // port: mission i on event e draws from (e * n + i) * ROLLS_PER_MISSION.
        let (system, _) = mock_keys();
        let mut world = minimal_world();
        let character = agent_at(&mut world, system, true);
        let mut state = MissionState::new();
        world.mission_records = vec![diplomacy_record()];
        for (id, due) in [(0, 2), (1, 50)] {
            state.push_timed(ActiveMission::timed(
                id,
                MissionKind::Diplomacy,
                MissionFaction::Alliance,
                vec![MissionMember::Character(character)],
                system,
                due,
            ));
        }
        let mut rolls = vec![0.99; 2 * 2 * ROLLS_PER_MISSION];
        rolls[2 * ROLLS_PER_MISSION] = 0.0;

        MissionSystem::advance(
            &mut state,
            &world,
            &UprisingState::default(),
            &[TickEvent { tick: 1 }, TickEvent { tick: 2 }],
            &rolls,
        );

        // Mission 0 repeats on event 1 (tick 2) and re-arms from roll 0.0.
        assert_eq!(state.missions()[0].timer_due, Some(2 + 5));
    }

    #[test]
    fn a_validator_code_stands_when_the_container_is_also_destroyed() {
        // FUN_00545240.c:85-111 sets code 7 only when the validator set
        // none: a freed prisoner still ends the Rescue with code 8.
        let mut world = minimal_world();
        let (mut state, prisoner) = waiting_rescue(&mut world);
        world.characters[prisoner].is_captive = false;
        world.characters[prisoner].captured_by = None;
        let here = state.missions()[0].target_system;
        world.systems[here].is_destroyed = true;

        let advance = step(&mut state, &world, 2, &[]);

        assert_eq!(advance.ended[0].end_code, 8);
    }

    /// A Rescue sent from `(0, 0)` to a prisoner at `(100, 0)`, its hero
    /// travelling after day 1 and landing on day 20.
    fn travelling_rescue(world: &mut GameWorld) -> (MissionState, SystemKey) {
        let from = system_at(world, 0);
        let to = system_at(world, 100);
        world.mission_records = vec![rescue_record()];
        let hero = agent_at(world, from, true);
        let prisoner = agent_at(world, to, true);
        world.characters[prisoner].is_captive = true;
        world.characters[prisoner].captured_by = Some(crate::dat::Faction::Empire);
        let mut state = MissionState::new();
        state
            .dispatch_guarded(
                MissionRequest::single(
                    MissionKind::Rescue,
                    MissionFaction::Alliance,
                    hero,
                    to,
                    Some(prisoner),
                    0,
                ),
                world,
            )
            .expect("a free hero");
        step(&mut state, world, 1, &[]);
        (state, to)
    }

    #[test]
    fn a_container_destroyed_while_the_members_travel_never_gives_code_seven() {
        // FUN_00545240.c:100-104 gives code 7 only when FUN_00520ae0 holds:
        // the phase at +0x54 -> +0x1c (written by FUN_00524b70) is above 6.
        // The check runs once, on the destruction; Rescue's column 11 is 0.
        let mut world = minimal_world();
        let (mut state, to) = travelling_rescue(&mut world);
        world.systems[to].is_destroyed = true;

        assert!(step(&mut state, &world, 2, &[]).ended.is_empty());
        step(&mut state, &world, 20, &[1.0]);
        let advance = step(&mut state, &world, 21, &[]);

        assert!(advance.ended.is_empty());
        let mission = &state.missions()[0];
        assert_eq!((mission.phase, mission.end_code), (PHASE_TIMER, 0));
    }

    #[test]
    fn a_container_destroyed_after_the_members_land_ends_the_mission_with_code_seven() {
        // FUN_00545240.c:100-104: phase 8 is above 6, so code 7 even though
        // Rescue's column 11 does not end it through the validator.
        let mut world = minimal_world();
        let (mut state, _) = waiting_rescue(&mut world);
        let here = state.missions()[0].target_system;
        world.systems[here].is_destroyed = true;

        let advance = step(&mut state, &world, 2, &[]);

        assert_eq!(advance.ended[0].end_code, 7);
    }

    #[test]
    fn a_live_target_s_change_waits_for_the_next_phase_change() {
        // FUN_00545240 runs the validator only for a destroyed container or
        // target; a freed prisoner is seen when the timer fires.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        world.mission_records = vec![rescue_record()];
        let hero = agent_at(&mut world, here, true);
        let prisoner = agent_at(&mut world, here, true);
        let mut rescue = ActiveMission::timed(
            0,
            MissionKind::Rescue,
            MissionFaction::Alliance,
            vec![MissionMember::Character(hero)],
            here,
            7,
        );
        rescue.target_character = Some(prisoner);
        let mut state = MissionState::new();
        state.push_timed(rescue);

        assert!(step(&mut state, &world, 2, &[]).ended.is_empty());
        assert_eq!(step(&mut state, &world, 7, &[]).ended[0].end_code, 8);
    }

    #[test]
    fn a_transit_ends_once_any_member_is_no_longer_travelling() {
        // FUN_00522280.c:66-100: the mission waits only while every member
        // still travels; one member off the road sets the ready bit.
        let mut world = minimal_world();
        let from = system_at(&mut world, 0);
        let to = system_at(&mut world, 100);
        world.mission_records = vec![diplomacy_record()];
        let first = agent_at(&mut world, from, true);
        let second = agent_at(&mut world, from, true);
        let mut state = MissionState::new();
        state
            .dispatch_guarded(
                MissionRequest {
                    kind: MissionKind::Diplomacy,
                    faction: MissionFaction::Alliance,
                    team: vec![
                        MissionMember::Character(first),
                        MissionMember::Character(second),
                    ],
                    decoys: vec![],
                    target_system: to,
                    target_character: None,
                    target_object: None,
                    tick: 0,
                },
                &mut world,
            )
            .expect("two free envoys");
        step(&mut state, &world, 1, &[]);
        state
            .en_route
            .retain(|transit| transit.member != MissionMember::Character(first));

        step(&mut state, &world, 2, &[0.0]);

        assert_eq!(state.missions()[0].phase, PHASE_TIMER);
    }

    #[test]
    fn a_team_killed_on_the_way_counts_as_arrived() {
        // FUN_00522280: the original deletes a killed member, so no member
        // is left travelling and the wait ends at once; phase 5's validator
        // then ends the teamless mission with code 5 (FUN_00522480.c:42-52).
        let mut world = minimal_world();
        let from = system_at(&mut world, 0);
        let to = system_at(&mut world, 100);
        let (mut state, envoy) = diplomacy_from(&mut world, from, to);
        step(&mut state, &world, 1, &[]);
        world.characters[envoy].mark_killed();

        let advance = step(&mut state, &world, 2, &[0.0]);

        assert_eq!(
            advance
                .ended
                .iter()
                .map(|e| (e.end_code, e.tick))
                .collect::<Vec<_>>(),
            vec![(5, 2)]
        );
    }

    #[test]
    fn members_with_no_origin_stay_where_they_are() {
        // FUN_00556430.c:31-33: a null origin (+0x6c) starts no transit and
        // moves no one.
        let mut world = minimal_world();
        let from = system_at(&mut world, 0);
        let to = system_at(&mut world, 100);
        let (mut state, envoy) = diplomacy_from(&mut world, from, to);
        world.characters[envoy].current_system = None;

        let advance = step(&mut state, &world, 1, &[0.0]);

        assert!(advance.effects.is_empty());
        assert!(state.en_route().is_empty());
        let mission = &state.missions()[0];
        assert_eq!((mission.origin, mission.phase), (None, PHASE_TIMER));
    }

    #[test]
    fn a_cancelled_mission_s_members_travel_on_and_land_at_its_target() {
        // port: release frees the flags, and each member's own transit runs
        // on as it does for a mission that ends (MemberTransit); the
        // original's cancel path is untraced.
        let mut world = minimal_world();
        let from = system_at(&mut world, 0);
        let to = system_at(&mut world, 100);
        let (mut state, envoy) = diplomacy_from(&mut world, from, to);
        step(&mut state, &world, 1, &[]);
        let id = state.missions()[0].id;

        state.release(id, &mut world).expect("the mission runs");

        let member = MissionMember::Character(envoy);
        assert!(!world.characters[envoy].on_mission);
        assert!(state.is_en_route(member));
        assert_eq!(
            state.dispatch_guarded(
                MissionRequest::single(
                    MissionKind::Diplomacy,
                    MissionFaction::Alliance,
                    envoy,
                    from,
                    None,
                    2,
                ),
                &mut world,
            ),
            Err(MissionRefusal::MemberUnavailable(member))
        );
        let arrival = step(&mut state, &world, 20, &[]);
        assert_eq!(
            arrival.effects,
            vec![MissionEffect::MemberMoved {
                member,
                to: Some(to)
            }]
        );
    }

    #[test]
    fn a_killed_character_stays_where_it_died() {
        // Character::mark_killed takes the dead off the map; a late
        // arrival leaves them there.
        let mut world = minimal_world();
        let from = system_at(&mut world, 0);
        let to = system_at(&mut world, 100);
        let pilot = agent_at(&mut world, from, true);
        world.characters[pilot].mark_killed();
        let resting = world.characters[pilot].current_system;

        move_member(&mut world, MissionMember::Character(pilot), Some(to));

        assert_eq!(world.characters[pilot].current_system, resting);
        assert_ne!(resting, Some(to));
    }

    #[test]
    fn member_effects_move_then_free_the_member() {
        let mut world = minimal_world();
        let from = system_at(&mut world, 0);
        let to = system_at(&mut world, 100);
        let envoy = agent_at(&mut world, from, true);
        world.characters[envoy].on_mission = true;
        let member = MissionMember::Character(envoy);

        apply_advance_effects(
            &mut world,
            &[
                MissionEffect::MemberMoved {
                    member,
                    to: Some(to),
                },
                MissionEffect::MemberAvailable { member },
                MissionEffect::CharacterKilled {
                    character: envoy,
                    faction: MissionFaction::Empire,
                },
            ],
        );

        let character = &world.characters[envoy];
        assert_eq!(character.current_system, Some(to));
        assert!(!character.on_mission);
        assert!(!character.is_killed);
    }

    #[test]
    fn detection_effects_take_the_prisoner_and_destroy_the_special_force() {
        // Slot +0x20c holds the captured character at the location for the
        // captor; FUN_00503eb0 destroys a captured special force.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        let elsewhere = system_at(&mut world, 100);
        let spy = agent_at(&mut world, elsewhere, true);
        let fleet = world.fleets.insert(crate::world::Fleet {
            location: elsewhere,
            capital_ships: vec![],
            fighters: vec![],
            characters: vec![spy],
            is_alliance: true,
            has_death_star: false,
        });
        world.characters[spy].current_fleet = Some(fleet);
        let unit = |world: &mut GameWorld| {
            let key = world.special_forces.insert(crate::world::SpecialForceUnit {
                class_dat_id: crate::ids::DatId::new(0x1400_0001),
                is_alliance: true,
                skills: [0; 8],
                on_mission: true,
            });
            world.systems[here].special_forces.push(key);
            key
        };
        let (lost, kept) = (unit(&mut world), unit(&mut world));

        apply_advance_effects(
            &mut world,
            &[
                MissionEffect::CharacterCaptured {
                    character: spy,
                    captured_by: MissionFaction::Empire,
                    at_system: here,
                },
                MissionEffect::SpecialForceDestroyed { unit: lost },
            ],
        );

        let prisoner = &world.characters[spy];
        assert!(prisoner.is_captive);
        assert_eq!(prisoner.captured_by, Some(crate::dat::Faction::Empire));
        assert_eq!(
            (prisoner.current_system, prisoner.current_fleet),
            (Some(here), None)
        );
        assert!(world.fleets[fleet].characters.is_empty());
        assert!(!world.special_forces.contains_key(lost));
        assert_eq!(world.systems[here].special_forces, vec![kept]);
        assert!(world.special_forces.contains_key(kept));
    }

    #[test]
    fn a_mission_with_members_of_both_sides_skips_the_running_checks() {
        // FUN_005830a0 status 0x14: no end code, though Diplomacy at a
        // system the opponent holds would end 8 (column 17).
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        world.systems[here].control =
            crate::world::ControlKind::Controlled(crate::dat::Faction::Empire);
        world.mission_records = vec![diplomacy_record()];
        let envoy = MissionMember::Character(agent_at(&mut world, here, true));
        let mut mission = ActiveMission::timed(
            0,
            MissionKind::Diplomacy,
            MissionFaction::Alliance,
            vec![envoy],
            here,
            5,
        );
        let uprisings = UprisingState::default();
        assert_eq!(running_end_code(&mission, &world, &uprisings, &[]), 8);

        mission.decoys = vec![MissionMember::Character(agent_at(&mut world, here, false))];

        assert_eq!(running_end_code(&mission, &world, &uprisings, &[]), 0);
    }

    #[test]
    fn a_draw_past_the_timers_reads_the_fallback_not_the_stream_seed() {
        // port: slot STREAM_SEED_ROLL only seeds the mission's stream.
        let window = [0.1, 0.2, 0.9];
        let mut draws = MissionRolls {
            rolls: &window,
            next: 0,
            stream: crate::mission_detection::MissionRng::seeded(0.9, 0),
        };
        let taken: Vec<f64> = (0..3).map(|_| draws.next()).collect();
        assert_eq!(taken, vec![0.1, 0.2, 0.5]);
    }

    // --- Member legality (FUN_005830a0, FUN_00583320) ---

    /// A special force of a class whose SPECFCSD mission mask is `mask`.
    fn unit_of_mask(world: &mut GameWorld, mask: u32, is_alliance: bool) -> MissionMember {
        let class_dat_id = crate::ids::DatId::new(0x3c00_0000 | mask);
        world.special_force_classes.insert(
            class_dat_id,
            crate::world::SpecialForceClassDef {
                skills: [skill_pair(0); 8],
                mission_mask: mask,
            },
        );
        MissionMember::SpecialForce(world.special_forces.insert(crate::world::SpecialForceUnit {
            class_dat_id,
            is_alliance,
            skills: [0; 8],
            on_mission: false,
        }))
    }

    #[test]
    fn a_special_force_is_allowed_only_when_the_record_takes_every_bit_of_its_class() {
        // FUN_00583320: the members' SPECFCSD masks must be a subset of the
        // record's +0x48. The shipped Rescue record takes 0x802.
        let mut world = minimal_world();
        let rescue = rescue_record().members;
        let commandos = unit_of_mask(&mut world, 0x2, true);
        let spies = unit_of_mask(&mut world, 0x8, true);

        assert!(members_allowed(&world, rescue, [commandos]));
        assert!(!members_allowed(&world, rescue, [spies]));
        assert!(!members_allowed(&world, rescue, [commandos, spies]));
    }

    #[test]
    fn a_record_with_no_special_force_bits_takes_only_characters() {
        // The shipped Diplomacy record has +0x48 = 0 and +0x4c = 0x10000.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        let diplomacy = diplomacy_record().members;
        let envoy = MissionMember::Character(agent_at(&mut world, here, true));
        let unit = unit_of_mask(&mut world, 0x2, true);

        assert!(members_allowed(&world, diplomacy, [envoy]));
        assert!(!members_allowed(&world, diplomacy, [envoy, unit]));
        let no_characters = crate::world::MissionMemberRules {
            character_mask: 0,
            ..diplomacy
        };
        assert!(!members_allowed(&world, no_characters, [envoy]));
    }

    #[test]
    fn members_must_share_one_side_that_the_record_runs_for() {
        // FUN_00583320: both sides fail (0x14), no side fails (0x16), and a
        // side needs the record's +0x40 (Alliance) or +0x44 (Empire). The
        // shipped Assassination record is Empire-only.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        let rules = crate::world::MissionMemberRules {
            alliance: false,
            empire: true,
            special_force_mask: 0x800,
            character_mask: 0x1_0000,
        };
        let rebel = MissionMember::Character(agent_at(&mut world, here, true));
        let imperial = MissionMember::Character(agent_at(&mut world, here, false));

        assert!(members_allowed(&world, rules, [imperial]));
        assert!(!members_allowed(&world, rules, [rebel]));
        assert!(!members_allowed(&world, rules, [imperial, rebel]));
        assert!(!members_allowed(&world, rules, []));
        // The shipped DS Sabotage record is Alliance-only.
        let alliance_only = crate::world::MissionMemberRules {
            alliance: true,
            empire: false,
            ..rules
        };
        assert!(members_allowed(&world, alliance_only, [rebel]));
        assert!(!members_allowed(&world, alliance_only, [imperial]));
    }

    #[test]
    fn a_request_whose_members_the_record_does_not_admit_is_refused() {
        // FUN_005420d0 runs the member check on the team, decoys, and
        // captured before anything is created.
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        world.mission_records = vec![diplomacy_record()];
        let envoy = MissionMember::Character(agent_at(&mut world, here, true));
        let unit = unit_of_mask(&mut world, 0x2, true);
        world.systems[here].special_forces.push(match unit {
            MissionMember::SpecialForce(key) => key,
            MissionMember::Character(_) => unreachable!(),
        });
        let mut state = MissionState::new();

        let refused = state.dispatch_guarded(
            MissionRequest {
                decoys: vec![unit],
                ..MissionRequest::single(
                    MissionKind::Diplomacy,
                    MissionFaction::Alliance,
                    envoy.character().unwrap(),
                    here,
                    None,
                    0,
                )
            },
            &mut world,
        );

        assert_eq!(refused, Err(MissionRefusal::MembersNotAllowed));
        assert!(state.is_empty());
        assert!(!world.characters[envoy.character().unwrap()].on_mission);
    }

    // --- The dialog's mission list (FUN_004f5380, FUN_005422f0) ---

    /// An Alliance envoy at an Alliance system, with records in `records`.
    fn dialog_world(
        records: Vec<crate::world::MissionRecord>,
    ) -> (GameWorld, SystemKey, MissionMember) {
        let mut world = minimal_world();
        let here = system_at(&mut world, 0);
        world.systems[here].control =
            crate::world::ControlKind::Controlled(crate::dat::Faction::Alliance);
        world.mission_records = records;
        let envoy = MissionMember::Character(agent_at(&mut world, here, true));
        (world, here, envoy)
    }

    #[test]
    fn the_dialog_lists_the_records_the_team_may_undertake_in_missnsd_order() {
        // FUN_005422f0 walks the records in table order. Rescue names a
        // character, so a system target leaves it out (port:).
        let (world, here, envoy) = dialog_world(vec![
            recruitment_record(),
            rescue_record(),
            diplomacy_record(),
        ]);

        let kinds = available_kinds(
            &world,
            &UprisingState::default(),
            MissionFaction::Alliance,
            &[envoy],
            &[],
            here,
        );

        assert_eq!(
            kinds,
            vec![MissionKind::Recruitment, MissionKind::Diplomacy]
        );
    }

    #[test]
    fn the_dialog_leaves_out_a_record_whose_validator_refuses_the_target() {
        // FUN_0054c590 runs the class validator: Recruitment takes no target
        // of a third side (column 16), Diplomacy does.
        let (mut world, here, envoy) = dialog_world(vec![recruitment_record(), diplomacy_record()]);
        world.systems[here].control = crate::world::ControlKind::Uncontrolled;

        let kinds = available_kinds(
            &world,
            &UprisingState::default(),
            MissionFaction::Alliance,
            &[envoy],
            &[],
            here,
        );

        assert_eq!(kinds, vec![MissionKind::Diplomacy]);
    }

    #[test]
    fn the_dialog_leaves_out_a_record_whose_member_rules_refuse_a_decoy() {
        // FUN_005830a0 masks the decoys too; Diplomacy takes no special force.
        let (mut world, here, envoy) = dialog_world(vec![diplomacy_record()]);
        let unit = unit_of_mask(&mut world, 0x2, true);

        let with_decoy = available_kinds(
            &world,
            &UprisingState::default(),
            MissionFaction::Alliance,
            &[envoy],
            &[unit],
            here,
        );
        let alone = available_kinds(
            &world,
            &UprisingState::default(),
            MissionFaction::Alliance,
            &[envoy],
            &[],
            here,
        );

        assert!(with_decoy.is_empty());
        assert_eq!(alone, vec![MissionKind::Diplomacy]);
    }

    #[test]
    fn the_dialog_never_lists_a_hidden_record() {
        // FUN_005422f0 skips records with +0x5c set.
        let hidden = crate::world::MissionRecord {
            hidden: true,
            ..diplomacy_record()
        };
        let (world, here, envoy) = dialog_world(vec![hidden]);

        let kinds = available_kinds(
            &world,
            &UprisingState::default(),
            MissionFaction::Alliance,
            &[envoy],
            &[],
            here,
        );

        assert!(kinds.is_empty());
    }
}
