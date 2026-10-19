//! The side AI's ship and fleet records (`FUN_00484630`, `FUN_00433620`,
//! `FUN_00433780`), the marks the strategic modules keep on ships, fleets,
//! systems and sectors, and the context sums they read (`FUN_0041b7e0`,
//! `FUN_0041ba80`; `ghidra/notes/ai-order-emission.md`, "Records").
//!
//! The records are rebuilt every round from the world; only the marks
//! persist.

use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::ai_production::ClassNode;
use crate::dat::Faction;
use crate::ids::{FleetKey, SectorKey, SystemKey};
use crate::manufacturing::BuildableKind;
use crate::world::{GameWorld, GNPRTB_DAMAGE_COUNTER_MAX, SHIPPED_DAMAGE_COUNTER_MAX};

/// `ship+0x24` bits (`FUN_00484630`; the class bits come from the class
/// node, `FUN_004739b0`).
pub mod ship {
    /// Class node flag 1: a combat ship.
    pub const COMBAT: u32 = 0x1;
    /// Class node flag 2: a laser-armed ship without troop room (`FUN_004739b0`).
    pub const LASER: u32 = 0x2;
    /// Class node flag `0x80`: the Death Star (`FUN_004739b0`).
    pub const DEATH_STAR: u32 = 0x80;
    /// Class node flag `0x400`: a gravity well projector (`FUN_004739b0`).
    pub const GRAVITY_WELL: u32 = 0x400;
    /// Squadrons aboard (`FUN_00484630`).
    pub const CARRIES_FIGHTERS: u32 = 0x100;
    /// Regiments aboard (`FUN_00484630`).
    pub const CARRIES_REGIMENTS: u32 = 0x200;
    /// Damaged and still able to jump: a drive has its rating left
    /// (`FUN_00484630`, `FUN_00500820`; hyp).
    pub const DAMAGED: u32 = 0x1000;
    /// Damaged with both drives out (`FUN_00484630`, `FUN_00500820`; hyp).
    pub const DRIVES_OUT: u32 = 0x80_0000;
    /// Either damage flag (`FUN_00433780`, `FUN_004749b0`).
    pub const NOT_READY: u32 = DAMAGED | DRIVES_OUT;
    /// Object `+0x50` bit 4: in hyperspace (`FUN_00484630`).
    pub const EN_ROUTE: u32 = 0x800;
    /// Object `+0x50` bit 2 clear: not finished (`FUN_00484630`).
    pub const INCOMPLETE: u32 = 0x2000;
    /// The object belongs to the record's side (`FUN_00484630`).
    pub const OWN: u32 = 0x4000;
    /// Free: no regional record holds it (set at creation, `FUN_00484230`).
    pub const FREE: u32 = 0x2_0000;
    /// A character of the side aboard, not a prisoner (`FUN_00484630`).
    pub const OWN_CHARACTER: u32 = 0x40_0000;
    /// The record's flagship (`FUN_00479560`).
    pub const FLAGSHIP: u32 = 0x100_0000;
    /// A request to replace the flagship (`FUN_00478aa0`, `FUN_00478690`).
    pub const DEMOTE: u32 = 0x200_0000;
    /// Claimed by a fleet task (`FUN_00475e70`).
    pub const TASK: u32 = 0x1000_0000;
    /// The marks that outlive a refresh (`& 0x1b0204ff`, less the class
    /// bits the port rebuilds from the class).
    pub const MARKS: u32 = FREE | FLAGSHIP | DEMOTE | TASK;
}

/// `fleet+0x38` bits (`FUN_00433620`, `FUN_00433780`).
pub mod fleet {
    /// The side's fleet.
    pub const OWN: u32 = 0x1;
    /// Object `+0x50` bit 4: in hyperspace (`FUN_00433620`).
    pub const EN_ROUTE: u32 = 0x2;
    /// New, no record or task has adopted it (`FUN_00432e40`).
    pub const UNADOPTED: u32 = 0x4;
    /// A fleet task owns it (`FUN_00475fe0`).
    pub const TASK: u32 = 0x8;
    /// A Death Star aboard (`FUN_00433780`).
    pub const DEATH_STAR: u32 = 0x10;
    /// A gravity well projector aboard (`FUN_00433780`).
    pub const GRAVITY_WELL: u32 = 0x20;
    /// A damaged ship that can jump aboard (`FUN_00433780`).
    pub const DAMAGED: u32 = 0x1000;
    /// A damaged ship with both drives out aboard (`FUN_00433780`).
    pub const DRIVES_OUT: u32 = 0x2000;
    /// A ship in hyperspace (`FUN_00433780`).
    pub const SHIP_EN_ROUTE: u32 = 0x800;
    /// A ship neither in hyperspace nor unfinished (`FUN_00433780`).
    pub const ACTIVE: u32 = 0x4000;
    /// A ship free of `0x801000` (`FUN_00433780`).
    pub const READY: u32 = 0x8000;
    /// A character of the side aboard (ship `0x400000`) (`FUN_00433780`).
    pub const OWN_CHARACTER: u32 = 0x400_0000;
    /// The posture needs (`FUN_004b1070`): engage, assault, bombard, drop.
    pub const ENGAGE: u32 = 0x1000_0000;
    /// Assault (`FUN_004b1070`).
    pub const ASSAULT: u32 = 0x2000_0000;
    /// Bombard or Destroy System (`FUN_004b1070`).
    pub const BOMBARD: u32 = 0x4000_0000;
    /// Drop regiments (`FUN_004b1070`).
    pub const DROP: u32 = 0x8000_0000;
    /// The four posture needs (`FUN_004b1070`).
    pub const NEEDS: u32 = 0xf000_0000;
    /// The marks that outlive a refresh (`& 0xf000000c`, `FUN_00433620`).
    pub const MARKS: u32 = 0xf000_000c;
}

/// `sys+0x30` bits (S, `FUN_00433e40`).
pub mod sys {
    pub const OWN: u32 = 0x1;
    /// Held by neither side (`FUN_00433e40`).
    pub const NEUTRAL: u32 = 0x2;
    /// Held by the other side (`FUN_00433e40`).
    pub const ENEMY: u32 = 0x4;
    /// Set by `FUN_00433e40` (`FUN_00433e40`).
    pub const POPULATED: u32 = 0x8;
    /// Held by a side, or populated (`FUN_00433e40`).
    pub const HELD: u32 = 0x20;
    /// System family `0x92`-`0x93` (`FUN_00433e40`).
    pub const RIM: u32 = 0x40;
    /// An active Death Star shield (`FUN_00433e40`).
    pub const DS_SHIELD: u32 = 0x80;
    /// More than one shield generator (`FUN_00433e40`).
    pub const SHIELDS: u32 = 0x100;
    /// A foreign fleet with an active ship (`FUN_00433e40`).
    pub const FOREIGN_FLEET: u32 = 0x200;
    /// Such a fleet carries the Death Star (`FUN_00433e40`).
    pub const FOREIGN_DEATH_STAR: u32 = 0x400;
    /// An enemy system with installations and no battery (`FUN_00433e40`).
    pub const UNDEFENDED: u32 = 0x800;
    /// Loading-system claims by posture (`FUN_004af8a0` and siblings).
    pub const LOADING: u32 = 0x7_0000;
    /// A fleet task's target (`FUN_004744b0`).
    pub const TASK_CLAIM: u32 = 0x8_0000;
    /// Posture fleet claims (`FUN_004af6d0`).
    pub const ENGAGE_CLAIM: u32 = 0x10_0000;
    /// Assault (`FUN_004af6d0`).
    pub const ASSAULT_CLAIM: u32 = 0x20_0000;
    /// Bombard (`FUN_004af6d0`).
    pub const BOMBARD_CLAIM: u32 = 0x40_0000;
    /// Drop (`FUN_004af6d0`).
    pub const DROP_CLAIM: u32 = 0x100_0000;
    /// The four posture fleet claims (`FUN_004af6d0`).
    pub const FLEET_CLAIMS: u32 = 0x170_0000;
    /// System index `0x121` (`FUN_00433e40`).
    pub const SPECIAL: u32 = 0x800_0000;
    /// Tagged by the controller (`FUN_004e5870`).
    pub const TAGGED: u32 = 0x1000_0000;
    /// Coruscant, system index `0x109` (`FUN_00433e40`).
    pub const CORUSCANT: u32 = 0x4000_0000;
    /// The owner's headquarters stands here (`FUN_00526a60`).
    pub const HQ: u32 = 0x8000_0000;
    /// The marks that outlive a refresh (`& 0x35ff0000`, `FUN_00433e40`).
    pub const MARKS: u32 = 0x35ff_0000;
}

/// One ship as the side's AI sees it (`ship+0x24..+0x94`).
#[derive(Debug, Clone, Default)]
pub struct ShipRecord {
    /// The world tag (the original's object id).
    pub tag: u32,
    pub fleet: FleetKey,
    pub flags: u32,
    /// `+0x30`, `+0x34`: the regional record holding it and that record's
    /// plan entry.
    pub record: u32,
    pub plan: u32,
    /// `+0x48`: bombardment.
    pub bombard: i64,
    /// `+0x50`: the shield figure, the flagship key.
    pub shield: i64,
    /// `+0x58`, `+0x5c`: squadrons aboard, active ones.
    pub squadrons: i64,
    pub squadrons_active: i64,
    /// `+0x60`, `+0x6c`: fighter and troop capacity.
    pub hangar: i64,
    pub troop_room: i64,
    /// `+0x64`, `+0x68`: regiments aboard, active ones.
    pub regiments: i64,
    pub regiments_active: i64,
    /// `+0x7c`, `+0x80`: attack and manoeuvre figures with active
    /// squadrons.
    pub attack: i64,
    pub manoeuvre: i64,
    /// `+0x90`, `+0x94`: the same figures with every squadron.
    pub attack_all: i64,
    pub manoeuvre_all: i64,
}

/// One fleet as the side's AI sees it (`fleet+0x28..+0xbc`).
#[derive(Debug, Clone, Default)]
pub struct FleetRecord {
    pub key: FleetKey,
    /// `+0x28`.
    pub location: SystemKey,
    /// `+0x2c`: the posture's target system.
    pub target: Option<SystemKey>,
    /// `+0x30`, `+0x34`: the regional record holding it, its plan entry.
    pub record: u32,
    pub plan: u32,
    pub flags: u32,
    /// `+0x48`: its ships' tags, in list order.
    pub ships: Vec<u32>,
    /// `+0x54` .. `+0xb8` (`FUN_00433780`).
    pub attack: i64,
    pub manoeuvre: i64,
    pub shield: i64,
    pub bombard: i64,
    pub squadrons: i64,
    pub squadrons_active: i64,
    pub hangar: i64,
    pub regiments: i64,
    pub regiments_active: i64,
    pub troop_room: i64,
    /// `+0x9c`: ships with class flag 2.
    pub laser_ships: i64,
    /// `+0xa8`, `+0xac`: free troop and hangar room.
    pub free_troop_room: i64,
    pub free_hangar: i64,
    /// `+0xb4`, `+0xb8`.
    pub attack_all: i64,
    pub manoeuvre_all: i64,
}

impl FleetRecord {
    #[must_use]
    pub const fn has(&self, mask: u32) -> bool {
        self.flags & mask == mask
    }
}

/// A fleet's persistent marks (`fleet+0x2c`, `+0x30`, `+0x34`, the kept
/// `+0x38` bits).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FleetMark {
    pub flags: u32,
    pub record: u32,
    pub plan: u32,
    pub target: Option<SystemKey>,
}

impl Default for FleetMark {
    /// A new fleet is unadopted (`FUN_00432e40`).
    fn default() -> Self {
        Self {
            flags: fleet::UNADOPTED,
            record: 0,
            plan: 0,
            target: None,
        }
    }
}

/// A ship's persistent marks (`ship+0x30`, `+0x34`, the kept `+0x24` bits).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShipMark {
    pub flags: u32,
    pub record: u32,
    pub plan: u32,
}

impl Default for ShipMark {
    /// A new ship is free (`FUN_00484230`).
    fn default() -> Self {
        Self {
            flags: ship::FREE,
            record: 0,
            plan: 0,
        }
    }
}

/// A sector's persistent fields (`rec+0x30`, `+0x34`, `+0x38`, `+0x3c`, the
/// kept `+0x2c` bits).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SectorMark {
    /// The posture kind the plan wrote (1-6).
    pub kind: u8,
    /// Kind 4's facility category.
    pub param: u8,
    /// The regional record linked to it.
    pub record: u32,
    /// The plan entry id (sticky key).
    pub plan_id: u32,
    /// `rec+0x2c` bit `0x20000000`: kind 2 visited every candidate.
    pub exhausted: bool,
}

/// The side's persistent AI marks.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Marks {
    pub ships: BTreeMap<u32, ShipMark>,
    #[serde(
        serialize_with = "crate::serde_ordered::serialize_hash_map",
        deserialize_with = "crate::serde_ordered::deserialize_hash_map"
    )]
    pub fleets: HashMap<FleetKey, FleetMark>,
    /// `sys+0x30 & 0x35ff0000`.
    #[serde(
        serialize_with = "crate::serde_ordered::serialize_hash_map",
        deserialize_with = "crate::serde_ordered::deserialize_hash_map"
    )]
    pub systems: HashMap<SystemKey, u32>,
    #[serde(
        serialize_with = "crate::serde_ordered::serialize_hash_map",
        deserialize_with = "crate::serde_ordered::deserialize_hash_map"
    )]
    pub sectors: HashMap<SectorKey, SectorMark>,
    /// `sys+0x44`, which the rebuild keeps: the day each system was last
    /// seen blockaded.
    #[serde(
        serialize_with = "crate::serde_ordered::serialize_hash_map",
        deserialize_with = "crate::serde_ordered::deserialize_hash_map"
    )]
    pub last_seen: HashMap<SystemKey, i64>,
}

impl Marks {
    #[must_use]
    pub fn ship(&self, tag: u32) -> ShipMark {
        self.ships.get(&tag).copied().unwrap_or_default()
    }

    #[must_use]
    pub fn fleet(&self, key: FleetKey) -> FleetMark {
        self.fleets.get(&key).copied().unwrap_or_default()
    }

    #[must_use]
    pub fn system(&self, key: SystemKey) -> u32 {
        self.systems.get(&key).copied().unwrap_or(0)
    }

    #[must_use]
    pub fn sector(&self, key: SectorKey) -> SectorMark {
        self.sectors.get(&key).copied().unwrap_or_default()
    }

    pub fn ship_mut(&mut self, tag: u32) -> &mut ShipMark {
        self.ships.entry(tag).or_default()
    }

    pub fn fleet_mut(&mut self, key: FleetKey) -> &mut FleetMark {
        self.fleets.entry(key).or_default()
    }

    pub fn system_mut(&mut self, key: SystemKey) -> &mut u32 {
        self.systems.entry(key).or_insert(0)
    }

    pub fn sector_mut(&mut self, key: SectorKey) -> &mut SectorMark {
        self.sectors.entry(key).or_default()
    }

    /// Drop the marks of ships and fleets that no longer exist.
    pub fn prune(&mut self, world: &GameWorld) {
        let tags: std::collections::HashSet<u32> = world
            .fleets
            .values()
            .flat_map(|f| f.capital_ships.iter().map(|s| s.tag))
            .collect();
        self.ships.retain(|tag, _| tags.contains(tag));
        self.fleets.retain(|key, _| world.fleets.contains_key(*key));
    }
}

/// The side's best producible class figures (`c+0x164`..`+0x174`,
/// `FUN_0042e210`, `FUN_0042e960`).
#[derive(Debug, Clone, Copy, Default)]
pub struct BestClasses {
    /// `c+0x164`: the largest attack score (key `0x200`) of a combat class.
    pub attack: i64,
    /// `c+0x168`: the largest laser-ship manoeuvre score (key `0x400`).
    pub manoeuvre: i64,
    /// `c+0x16c`: the largest fighter capacity, the Death Star aside.
    pub hangar: i64,
    /// `c+0x170`: the largest troop capacity.
    pub troop_room: i64,
    /// `c+0x174`: 1 when a bombardment class exists.
    pub bombard: i64,
}

impl BestClasses {
    /// `FUN_0042e210`: each figure is `FUN_0042e960` over the producible
    /// classes (attack: mask 1, key `0x200`; manoeuvre: mask 2, key `0x400`).
    #[must_use]
    pub fn of(world: &GameWorld, research: &crate::research::ResearchState, side: Faction) -> Self {
        let mut best = Self::default();
        for kind in crate::build_selection::listed_classes(
            world,
            research,
            crate::manufacturing::ProductionArea::Shipyard,
            side == Faction::Alliance,
        ) {
            if !matches!(kind, BuildableKind::CapitalShip(_)) {
                continue;
            }
            let Some(node) = ClassNode::of(world, kind) else {
                continue;
            };
            if node.flags & ship::COMBAT != 0 {
                best.attack = best.attack.max(node.key(0x200));
            }
            if node.flags & ship::LASER != 0 {
                best.manoeuvre = best.manoeuvre.max(node.key(0x400));
            }
            if node.flags & ship::DEATH_STAR == 0 {
                best.hangar = best.hangar.max(node.n44);
            }
            best.troop_room = best.troop_room.max(node.n48);
            if node.n38 > 0 {
                best.bombard = 1;
            }
        }
        best
    }
}

/// What the records read.
pub struct RecordWorld<'a> {
    pub world: &'a GameWorld,
    pub movement: &'a crate::movement::MovementState,
    pub transport: &'a crate::troop_transport::TroopTransportState,
    pub marks: &'a Marks,
    pub side: Faction,
}

/// Every tagged ship's and every fleet's record, both sides
/// (`FUN_00484630`, `FUN_00433620`, `FUN_00433780`). A ship without a tag
/// has no record yet (port: `carriage::tag_all` tags every living ship
/// before each AI pass).
#[must_use]
pub fn fleet_records(
    inputs: &RecordWorld<'_>,
) -> (HashMap<FleetKey, FleetRecord>, HashMap<u32, ShipRecord>) {
    let world = inputs.world;
    let mut fleets = HashMap::new();
    let mut ships = HashMap::new();
    for (key, value) in &world.fleets {
        let side = if value.is_alliance {
            Faction::Alliance
        } else {
            Faction::Empire
        };
        let own = side == inputs.side;
        let mark = if own {
            inputs.marks.fleet(key)
        } else {
            FleetMark {
                flags: 0,
                ..FleetMark::default()
            }
        };
        let mut record = FleetRecord {
            key,
            location: value.location,
            target: mark.target,
            record: mark.record,
            plan: mark.plan,
            flags: mark.flags & fleet::MARKS,
            ..FleetRecord::default()
        };
        if own {
            record.flags |= fleet::OWN;
        }
        let en_route = inputs.movement.is_in_transit(key);
        if en_route {
            record.flags |= fleet::EN_ROUTE;
        }
        for (index, instance) in value.capital_ships.iter().enumerate() {
            if !instance.alive || instance.tag == 0 {
                continue;
            }
            let Some(s) = ship_record(inputs, key, index, own, en_route) else {
                continue;
            };
            fold(&mut record, &s);
            record.ships.push(s.tag);
            ships.insert(s.tag, s);
        }
        fleets.insert(key, record);
    }
    (fleets, ships)
}

/// `FUN_00484630`: one ship's flags and figures. The original scales the
/// attack, manoeuvre and shield figures by the weapons and shields left:
/// `W` and `S` are the class's weapon and shield figures (`cls+0xdc`,
/// `cls+0xe0`, node `+0x50`, `+0x54`), a counter removes `W` (or `S`) times
/// the counter over GNPRTB `0x800` (`FUN_00500f60`, `FUN_00500fa0`,
/// `FUN_0053e170`), and `FUN_004738d0`, `FUN_004738f0`, `FUN_00473910`
/// scale the node figure by what is left. A damaged ship carries `0x1000`,
/// or `0x800000` when `FUN_00500820` (the drive speed) is 0, and keeps no
/// bombard figure under `0x1000`. Squadrons and regiments that ride the
/// fleet rather than a ship count on its first ship.
fn ship_record(
    inputs: &RecordWorld<'_>,
    fleet: FleetKey,
    index: usize,
    own: bool,
    en_route: bool,
) -> Option<ShipRecord> {
    let world = inputs.world;
    let value = world.fleets.get(fleet)?;
    let instance = value.capital_ships.get(index)?;
    let node = ClassNode::of(world, BuildableKind::CapitalShip(instance.class))?;
    let class = world.capital_ship_classes.get(instance.class)?;
    let mark = if own {
        inputs.marks.ship(instance.tag)
    } else {
        ShipMark::default()
    };
    let mut r = ShipRecord {
        tag: instance.tag,
        fleet,
        flags: (node.flags & 0x4ff) | (mark.flags & ship::MARKS),
        record: mark.record,
        plan: mark.plan,
        bombard: node.n38,
        shield: node.key(0x1_0000),
        hangar: node.n44,
        troop_room: node.n48,
        ..ShipRecord::default()
    };
    // The class figures the node's divisors hold (`FUN_004739b0` `+0x50`,
    // `+0x54`) and the damage the counters take off them.
    let weapon = i64::from(class.weapon_recharge_rate);
    let shield = i64::from(class.shield_recharge_rate);
    let counter_max = crate::movement::gnprtb_or_shipped(
        world,
        GNPRTB_DAMAGE_COUNTER_MAX,
        SHIPPED_DAMAGE_COUNTER_MAX,
    );
    let weapon_left = weapon - weapon * i64::from(instance.damage_counter(1)) / counter_max;
    let shield_left = shield - shield * i64::from(instance.damage_counter(0)) / counter_max;
    let scaled = |figure: i64, left: i64, whole: i64| {
        if whole > 0 {
            figure * left / whole
        } else {
            0
        }
    };
    if instance.is_damaged(class.hull.cast_signed()) {
        r.flags |= if crate::movement::capital_ship_instance_speed(world, instance, class) == 0 {
            ship::DRIVES_OUT
        } else {
            ship::DAMAGED
        };
    }
    if r.flags & ship::DAMAGED != 0 {
        r.bombard = 0;
    }
    r.shield = scaled(node.key(0x1_0000), shield_left, shield);
    r.attack = scaled(node.n30, weapon_left, weapon);
    r.attack_all = r.attack;
    if r.flags & ship::LASER != 0 {
        r.manoeuvre = scaled(node.n34, weapon_left, weapon);
        r.manoeuvre_all = r.manoeuvre;
    }
    if own {
        r.flags |= ship::OWN;
    }
    if en_route {
        r.flags |= ship::EN_ROUTE;
    }
    let carried = |carrier: u32| carrier == instance.tag || (carrier == 0 && index == 0);
    for entry in value.fighters.iter().filter(|e| carried(e.carrier)) {
        let Some(fnode) = ClassNode::of(world, BuildableKind::Fighter(entry.class)) else {
            continue;
        };
        let n = i64::from(entry.count);
        r.squadrons += n;
        r.flags |= ship::CARRIES_FIGHTERS;
        r.attack_all += fnode.n30 * n;
        r.manoeuvre_all += fnode.n34 * n;
        r.attack += fnode.n30 * n;
        r.manoeuvre += fnode.n34 * n;
        r.squadrons_active += n;
    }
    for _ in inputs
        .transport
        .cargo(fleet)
        .iter()
        .filter(|t| carried(inputs.transport.carrier(**t)))
        .filter_map(|t| world.troops.get(*t))
    {
        r.flags |= ship::CARRIES_REGIMENTS;
        r.regiments += 1;
        r.regiments_active += 1;
    }
    if index == 0 {
        let side_is_alliance = inputs.side == Faction::Alliance;
        for character in value
            .characters
            .iter()
            .filter_map(|c| world.characters.get(*c))
        {
            let theirs = character.is_alliance == side_is_alliance;
            if theirs && character.captured_by.is_none() {
                r.flags |= ship::OWN_CHARACTER;
            }
        }
    }
    Some(r)
}

/// `FUN_00433780`: fold one ship into its fleet.
fn fold(f: &mut FleetRecord, s: &ShipRecord) {
    let flags = s.flags;
    if flags & ship::CARRIES_FIGHTERS != 0 {
        f.flags |= 0x2_0000;
    }
    if flags & ship::CARRIES_REGIMENTS != 0 {
        f.flags |= 0x4_0000;
    }
    if flags & ship::OWN_CHARACTER != 0 {
        f.flags |= fleet::OWN_CHARACTER;
    }
    if flags & ship::EN_ROUTE != 0 {
        f.flags |= fleet::SHIP_EN_ROUTE;
    } else if flags & ship::INCOMPLETE == 0 {
        f.flags |= fleet::ACTIVE;
    }
    if flags & ship::INCOMPLETE != 0 {
        f.flags |= 0x400;
    }
    if flags & ship::DEATH_STAR != 0 {
        f.flags |= fleet::DEATH_STAR;
    }
    if flags & ship::GRAVITY_WELL != 0 {
        f.flags |= fleet::GRAVITY_WELL;
    }
    // A damaged ship leaves the class counts and the ready bit, and gives
    // the fleet `0x1000` or `0x2000` (`FUN_00433780`).
    if flags & ship::NOT_READY == 0 {
        f.flags |= fleet::READY;
    } else {
        if flags & ship::DRIVES_OUT != 0 {
            f.flags |= fleet::DRIVES_OUT;
        }
        if flags & ship::DAMAGED != 0 {
            f.flags |= fleet::DAMAGED;
        }
    }
    if flags & ship::LASER != 0 && flags & ship::NOT_READY == 0 {
        f.laser_ships += 1;
    }
    if flags & ship::INCOMPLETE == 0 {
        f.attack += s.attack;
        f.manoeuvre += s.manoeuvre;
        f.shield += s.shield;
        f.bombard += s.bombard;
        f.squadrons += s.squadrons;
        f.hangar += s.hangar;
        f.squadrons_active += s.squadrons_active;
        f.troop_room += s.troop_room;
        f.regiments += s.regiments;
        f.regiments_active += s.regiments_active;
        f.attack_all += s.attack_all;
        f.manoeuvre_all += s.manoeuvre_all;
    }
    if flags & (ship::EN_ROUTE | ship::INCOMPLETE) == 0 {
        f.free_troop_room += s.troop_room - s.regiments;
        f.free_hangar += s.hangar - s.squadrons;
    }
}

/// The strategic words and figures of a system record (`FUN_00433e40`):
/// S and Q, the foreign fleets present, the squadron and defence figures,
/// and at a non-own system the owner's installations, its regiment need
/// and threat.
///
/// hyp: `obj+0x88` bit 1, without which the record stops at `Q | 0x80`, is
/// read as the system being explored. port: counts at a non-own system are
/// its owner's objects, which `FUN_00433e40` counts as everything there.
pub fn fill_system(
    r: &mut crate::ai_production::SystemRecord,
    inputs: &RecordWorld<'_>,
    fleets: &HashMap<FleetKey, FleetRecord>,
    day: i64,
) {
    use crate::ai_production::bit;
    let world = inputs.world;
    let Some(system) = world.systems.get(r.key) else {
        return;
    };
    let mut s = inputs.marks.system(r.key) & sys::MARKS;
    let family = system.dat_id.family();
    if (0x92..=0x93).contains(&family) {
        s |= sys::RIM;
    }
    s |= if r.own {
        sys::OWN | sys::HELD
    } else if r.enemy {
        sys::ENEMY | sys::HELD
    } else {
        sys::NEUTRAL
    };
    r.last_seen = inputs.marks.last_seen.get(&r.key).copied().unwrap_or(0);
    if system.exploration_status != crate::dat::ExplorationStatus::Explored {
        r.q |= 0x80;
        r.s = s;
        return;
    }
    r.q |= 0x800_0000;
    if r.a & bit::BLOCKADE != 0 && s & (sys::NEUTRAL | sys::ENEMY) != 0 {
        r.last_seen = day;
    }
    let index = system.dat_id.raw() & 0xff_ffff;
    if index == 0x109 {
        s |= sys::CORUSCANT;
    }
    if index == 0x121 {
        s |= sys::SPECIAL;
    }
    if system.is_headquarters {
        s |= sys::HQ;
        r.q |= 0x200_0000;
    }
    for record in system.fleets.iter().filter_map(|f| fleets.get(f)) {
        if record.flags & fleet::OWN == 0 && record.flags & fleet::ACTIVE != 0 {
            r.fleet_attack += record.attack_all;
            r.fleet_manoeuvre += record.manoeuvre_all;
            r.fleet_shield += record.shield;
            r.fleet_squadrons += record.squadrons;
            s |= sys::FOREIGN_FLEET;
            r.q |= 0x80_0000;
            if record.flags & fleet::DEATH_STAR != 0 {
                s |= sys::FOREIGN_DEATH_STAR;
                if inputs.side == Faction::Alliance {
                    r.q |= 0x100_0000;
                }
            }
        }
    }
    let side_is_alliance = inputs.side == Faction::Alliance;
    for c in world
        .characters
        .values()
        .filter(|c| c.current_system == Some(r.key) && c.current_fleet.is_none() && !c.is_killed)
    {
        let enemy = c.is_alliance != side_is_alliance;
        if enemy && !c.is_captive && crate::mission_planning::is_leader(c) {
            r.hq_count += 1;
        }
    }
    // The owner's installations at a non-own system.
    if !r.own {
        let owner = system.control.faction();
        let of_owner = |side: Faction| owner.is_none_or(|o| o == side);
        r.mines = 0;
        r.refineries = 0;
        for f in system
            .production_facilities
            .iter()
            .filter_map(|k| world.production_facilities.get(*k))
            .filter(|f| of_owner(f.side))
        {
            if f.is_mine {
                r.mines += 1;
            } else {
                r.refineries += 1;
            }
        }
        r.yards = [0; 3];
        r.yard_spend = [0; 3];
        for y in system
            .manufacturing_facilities
            .iter()
            .filter_map(|k| world.manufacturing_facilities.get(*k))
            .filter(|y| of_owner(y.side))
        {
            if let Some(i) = yard_index(y.class_dat_id.family()) {
                r.yards[i] += 1;
                r.yard_spend[i] += world
                    .buildable_classes
                    .get(&y.class_dat_id)
                    .map_or(0, |c| i64::from(c.maintenance_cost));
            }
        }
        r.batteries = 0;
        r.shields = 0;
        r.ds_shields = 0;
    }
    for d in system
        .defense_facilities
        .iter()
        .filter_map(|k| world.defense_facilities.get(*k))
        .filter(|d| {
            let owner = system.control.faction();
            if r.own {
                d.side == inputs.side
            } else {
                owner.is_none_or(|o| o == d.side)
            }
        })
    {
        let class = world.buildable_classes.get(&d.class_dat_id);
        match d.class_dat_id.family() {
            0x22 | 0x23 => {
                if !r.own {
                    r.batteries += 1;
                }
                r.battery_attack += class.map_or(0, |c| i64::from(c.stats.attack_strength));
            }
            0x24 => {
                if !r.own {
                    r.shields += 1;
                }
                r.shield_strength += class.map_or(0, |c| i64::from(c.stats.shield_strength));
            }
            0x25 => {
                if !r.own {
                    r.ds_shields += 1;
                }
                // FUN_00433e40 marks an operational shield (object +0x50
                // bit 1). port: a facility enters the world finished and
                // has no operational state, so every one is.
                s |= sys::DS_SHIELD;
            }
            _ => {}
        }
    }
    if r.shields > 1 {
        s |= sys::SHIELDS;
    }
    if system.is_populated {
        s |= sys::POPULATED | sys::HELD;
        if r.popularity < 0x46 {
            r.q |= 0x1000_0000;
        }
        for entry in &system.fighters {
            let Some(node) = ClassNode::of(world, BuildableKind::Fighter(entry.class)) else {
                continue;
            };
            let n = i64::from(entry.count);
            r.squadrons_active += n;
            r.squadron_attack += node.n30 * n;
            r.squadron_manoeuvre += node.n34 * n;
        }
        if !r.own {
            let regiments = r.regiments_present;
            if r.enemy {
                if 0x50 - r.popularity > 0 {
                    r.regiment_need = (r.regiment_need + (0x59 - r.popularity) / 10).min(6);
                }
                r.threat = regiments + r.regiment_need + r.batteries;
                if r.popularity > 0x32 {
                    r.q |= 0x100;
                }
                if r.a & bit::HQ != 0 {
                    r.hq_count += 1;
                }
                if r.shield_strength > 0 {
                    r.q |= 0x4_0000;
                }
                if s & sys::DS_SHIELD != 0 {
                    r.q |= 0x8_0000;
                }
                if r.battery_attack > 0 {
                    r.q |= 0x10_0000;
                }
                if regiments > 0 {
                    r.q |= 0x20_0000;
                }
                if r.squadrons_active > 0 {
                    r.q |= 0x40_0000;
                }
                let installed =
                    r.mines + r.refineries + r.yards.iter().sum::<i64>() + r.ds_shields + r.shields;
                if installed > 0 && r.batteries == 0 {
                    s |= sys::UNDEFENDED;
                }
                if r.yards.iter().any(|&n| n > 0) {
                    r.q |= 0x2_0000;
                }
                if r.mines > 0 || r.refineries > 0 {
                    r.q |= 0x1_0000;
                }
            } else {
                if 0x5a - r.popularity > 0 {
                    r.regiment_need = (r.regiment_need + (99 - r.popularity) / 10).min(6);
                }
                r.threat = r.batteries + r.regiment_need;
                if s & sys::FOREIGN_FLEET == 0 {
                    r.q |= 0x40;
                }
                if r.a & bit::HQ != 0 {
                    r.hq_count += 1;
                }
            }
        } else {
            if r.a & bit::UPRISING != 0 && s & sys::FOREIGN_FLEET == 0 {
                r.q |= 0x20;
            }
            if r.q & 0x1000_0000 != 0 && s & sys::FOREIGN_FLEET == 0 && r.a & bit::UPRISING == 0 {
                r.q |= 0x40;
            }
        }
    } else if !r.own {
        r.threat = 0;
    }
    if r.own {
        r.q |= 0x201;
    } else if r.enemy {
        r.q |= 0x280;
    } else {
        r.q |= 0x80;
    }
    r.s = s;
}

/// A yard family's place in `sys+0x58`/`+0x5c`/`+0x60` (`FUN_00433e40`):
/// shipyards `0x28` (`FUN_0052cab0`), training `0x29` (`FUN_0052c740`),
/// construction yards `0x2a` (`FUN_0052c420`).
const fn yard_index(family: u8) -> Option<usize> {
    match family {
        0x28 => Some(0),
        0x29 => Some(1),
        0x2a => Some(2),
        _ => None,
    }
}

/// The computer player's battle withdrawal threshold (`FUN_004e6110`, the
/// answer of component `0xd` that `FUN_00485d50` stores in the side's
/// `+0x3c`; `ghidra/notes/morale-retreat.md`): the share of the enemy's
/// strength below which its side leaves a battle. It is 0.9, except at a
/// system the side holds with squadrons of its own: 0.0 at its headquarters
/// (`sys+0x28` bit `0x80000000`), elsewhere its fleets' attack (record
/// `+0x58`) over the others' there, or the own attack alone when no other
/// fleet is, at most 0.9.
/// port: the headquarters bit is Coruscant or the headquarters flag, as
/// `SystemRecord::build` sets it.
#[must_use]
pub fn battle_withdraw_threshold(
    world: &GameWorld,
    movement: &crate::movement::MovementState,
    transport: &crate::troop_transport::TroopTransportState,
    system: SystemKey,
    side: Faction,
) -> f32 {
    const DEFAULT: f32 = 0.9;
    let Some(value) = world.systems.get(system) else {
        return DEFAULT;
    };
    // `+0xac` counts the system's active squadrons, any side
    // (`FUN_00433e40.c:564-583`; `SystemRecord::squadrons_active`).
    // port: every squadron in the world is active, as `fill_system` counts.
    if value.control.faction() != Some(side) || value.fighters.iter().all(|entry| entry.count == 0)
    {
        return DEFAULT;
    }
    if value.is_headquarters || value.dat_id.raw() & 0xff_ffff == 0x109 {
        return 0.0;
    }
    let marks = Marks::default();
    let (fleets, _) = fleet_records(&RecordWorld {
        world,
        movement,
        transport,
        marks: &marks,
        side,
    });
    let (mut own, mut other) = (0_i64, 0_i64);
    for record in value.fleets.iter().filter_map(|key| fleets.get(key)) {
        if record.flags & fleet::OWN == 0 {
            other += record.attack;
        } else {
            own += record.attack;
        }
    }
    let mut ratio = own as f32;
    if other > 0 {
        ratio /= other as f32;
    }
    ratio.min(DEFAULT)
}
