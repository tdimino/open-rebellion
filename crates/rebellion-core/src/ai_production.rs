//! The AI's production modules 1, 2, 3, 5, 6, 7, 10 and 11
//! (`ghidra/notes/ai-order-emission.md`, "Production"): each spends a share
//! of the side's maintenance capacity on its units, and the holder of the
//! goal token builds the yards its units need.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::dat::Faction;
use crate::ids::SystemKey;
use crate::manufacturing::ManufacturingState;
use crate::troop_transport::TroopTransportState;
use crate::world::GameWorld;

/// A production module (`FUN_004bea20` creates them with their weights).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Module {
    /// Module 1: construction yards, mines and refineries.
    Yards,
    /// Module 2: mines and refineries while the load exceeds the capacity.
    DeficitRelief,
    /// Module 3: planetary defences, mines and refineries.
    Defences,
    /// Module 5: special forces.
    SpecialForces,
    /// Module 6: regiments.
    Regiments,
    /// Module 7: fighters for systems.
    SystemFighters,
    /// Module 10: fighters for fleets.
    FleetFighters,
    /// Module 11: capital ships.
    CapitalShips,
}

impl Module {
    /// Every module, in the scheduler's order: ascending kind. The list is a
    /// tree keyed by the kind each module stores at `+0x18`
    /// (`FUN_004ec940`, `FUN_005f4f10`, `FUN_005f5500`), whatever order
    /// `FUN_00485a50` adds them in; `FUN_004be980` walks its successor links
    /// from the cursor `+0x10`, wrapping to the leftmost (`FUN_005f5060`).
    pub const ALL: [Self; 8] = [
        Self::Yards,
        Self::DeficitRelief,
        Self::Defences,
        Self::SpecialForces,
        Self::Regiments,
        Self::SystemFighters,
        Self::FleetFighters,
        Self::CapitalShips,
    ];

    /// The module's share of the maintenance capacity, `m+0x24`
    /// (`FUN_004bea20`).
    #[must_use]
    pub const fn weight(self) -> i64 {
        match self {
            Self::CapitalShips => 30,
            Self::Regiments => 22,
            Self::SystemFighters => 17,
            Self::Defences => 9,
            Self::FleetFighters => 8,
            Self::Yards | Self::SpecialForces => 4,
            Self::DeficitRelief => 3,
        }
    }

    /// The module's goal bit (context `+8`), set by its facility phase
    /// `0x3ec` (`FUN_004cbe10`, `FUN_004ce810`, `FUN_004d6290`,
    /// `FUN_004d1970`, `FUN_004e15b0`, `FUN_004da0d0`, `FUN_004dcc40`,
    /// `FUN_004e4540`).
    #[must_use]
    pub const fn goal_bit(self) -> u8 {
        match self {
            Self::Defences => 0x1,
            Self::CapitalShips => 0x2,
            Self::SpecialForces => 0x4,
            Self::SystemFighters => 0x8,
            Self::FleetFighters => 0x10,
            Self::Regiments => 0x20,
            Self::DeficitRelief => 0x40,
            Self::Yards => 0x80,
        }
    }
}

/// A yard family: the producer of a module's units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum YardFamily {
    /// Family `0x28`: ships and fighters.
    Shipyard,
    /// Family `0x29`: regiments and special forces.
    TrainingFacility,
    /// Family `0x2a`: facilities.
    ConstructionYard,
}

impl YardFamily {
    /// The family of a class id's high byte (`FUN_00433e40`'s yard lists).
    const fn of(family: u8) -> Option<Self> {
        match family {
            0x28 => Some(Self::Shipyard),
            0x29 => Some(Self::TrainingFacility),
            0x2a => Some(Self::ConstructionYard),
            _ => None,
        }
    }

    /// The family's place in `sys+0x58`/`+0x5c`/`+0x60` (`FUN_00433e40`).
    const fn index(self) -> usize {
        match self {
            Self::Shipyard => 0,
            Self::TrainingFacility => 1,
            Self::ConstructionYard => 2,
        }
    }
}

/// The side context's counters the budgets read, as the census
/// `FUN_00418500` case 5 and `FUN_004182a0` case 1 fill them. Maintenance
/// is class `+0x4c` (`FUN_0053b870`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Census {
    /// `c+0x184`: the maintenance capacity, side `+0x58` (`FUN_00418970`).
    pub capacity: i64,
    /// `c+0x188`: the maintenance load, side `+0x70` (`FUN_00418990`).
    pub load: i64,
    /// `c+0x1d0`: the side's mines (family `0x2c`).
    pub mines: u32,
    /// `c+0x1d4`: the side's refineries (family `0x2d`).
    pub refineries: u32,
    /// `c+0x1d8`: regiments in systems.
    pub system_regiments: i64,
    /// `c+0x1dc`: defences (families `0x22`-`0x25`) in systems.
    pub system_defences: i64,
    /// `c+0x1e0`: fighters in systems.
    pub system_fighters: i64,
    /// `c+0x24c`: capital ships (fleet `+0x88`).
    pub capital_ships: i64,
    /// `c+0x250`: regiments aboard ships (fleet `+0x8c`).
    pub fleet_regiments: i64,
    /// `c+0x254`: fighters aboard ships (fleet `+0x90`).
    pub fleet_fighters: i64,
    /// `c+0x2c4`: special forces (character record `+0x64`).
    pub special_forces: i64,
    /// `sys+0xd0`, `+0xd4`, `+0xd8`: each system's shipyards', training
    /// facilities' and construction yards' maintenance (`FUN_00433e40`).
    pub yards: HashMap<SystemKey, [i64; 3]>,
}

impl Census {
    /// Count `side`'s objects. hyp: a sector sum over the side's systems
    /// (`FUN_0041b7e0`) covers the side's objects in every system; mines
    /// and refineries count wherever they stand, as side `+0x58` does.
    #[must_use]
    pub fn take(
        world: &GameWorld,
        manufacturing: &ManufacturingState,
        transport: &TroopTransportState,
        side: Faction,
    ) -> Self {
        let is_alliance = side == Faction::Alliance;
        let class = |id| {
            world
                .buildable_classes
                .get(id)
                .map_or(0, |class| i64::from(class.maintenance_cost))
        };
        let fighter = |key, count: u32| {
            world.fighter_classes.get(key).map_or(0, |class| {
                i64::from(class.maintenance_cost) * i64::from(count)
            })
        };
        let mut census = Self {
            capacity: crate::resources::maintenance_capacity(world, side),
            load: crate::resources::maintenance_used(world, manufacturing, side),
            ..Self::default()
        };
        for facility in world.production_facilities.values() {
            if facility.side == side {
                if facility.is_mine {
                    census.mines += 1;
                } else {
                    census.refineries += 1;
                }
            }
        }
        for (key, fleet) in world
            .fleets
            .iter()
            .filter(|(_, f)| f.is_alliance == is_alliance)
        {
            census.capital_ships += fleet
                .capital_ships
                .iter()
                .filter(|ship| ship.alive)
                .filter_map(|ship| world.capital_ship_classes.get(ship.class))
                .map(|class| i64::from(class.maintenance_cost))
                .sum::<i64>();
            census.fleet_fighters += fleet
                .fighters
                .iter()
                .map(|entry| fighter(entry.class, entry.count))
                .sum::<i64>();
            census.fleet_regiments += transport
                .cargo(key)
                .iter()
                .filter_map(|troop| world.troops.get(*troop))
                .map(|troop| class(&troop.class_dat_id))
                .sum::<i64>();
        }
        for (key, system) in &world.systems {
            census.system_regiments += system
                .ground_units
                .iter()
                .filter_map(|troop| world.troops.get(*troop))
                .filter(|troop| troop.is_alliance == is_alliance)
                .map(|troop| class(&troop.class_dat_id))
                .sum::<i64>();
            census.system_fighters += system
                .fighters
                .iter()
                .filter(|entry| entry.is_alliance == is_alliance)
                .map(|entry| fighter(entry.class, entry.count))
                .sum::<i64>();
            census.system_defences += system
                .defense_facilities
                .iter()
                .filter_map(|facility| world.defense_facilities.get(*facility))
                .filter(|facility| facility.side == side)
                .map(|facility| class(&facility.class_dat_id))
                .sum::<i64>();
            let mut yards = [0_i64; 3];
            for yard in system
                .manufacturing_facilities
                .iter()
                .filter_map(|facility| world.manufacturing_facilities.get(*facility))
                .filter(|yard| yard.side == side)
            {
                if let Some(family) = YardFamily::of(yard.class_dat_id.family()) {
                    yards[family.index()] += class(&yard.class_dat_id);
                }
            }
            if yards.iter().any(|&spent| spent != 0) {
                census.yards.insert(key, yards);
            }
        }
        census.special_forces = world
            .special_forces
            .values()
            .filter(|unit| unit.is_alliance == is_alliance)
            .map(|unit| class(&unit.class_dat_id))
            .sum();
        census
    }

    /// The maintenance of `family`'s yards over `systems`.
    #[must_use]
    pub fn yard_spend(&self, family: YardFamily, systems: &[SystemKey]) -> i64 {
        systems
            .iter()
            .filter_map(|system| self.yards.get(system))
            .map(|yards| yards[family.index()])
            .sum()
    }

    /// A module's budget for one purpose (`FUN_004cc220` and siblings):
    /// `pct`% of the module's share of the capacity, less what that purpose
    /// already spends, and nothing when it would exceed the unused capacity.
    #[must_use]
    pub fn budget(&self, module: Module, pct: i64, spent: i64) -> i64 {
        let share = module.weight() * self.capacity / 100;
        let budget = share * pct / 100 - spent;
        if budget > self.capacity - self.load {
            0
        } else {
            budget
        }
    }
}

/// The facility goal token (context `+8`, `+0xc`, `+0x10`;
/// `FUN_004178f0`): only the module whose bit the token holds builds
/// facilities, so one facility goal runs at a time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoalToken {
    /// `+8`: the modules waiting for a facility.
    pub pending: u8,
    /// `+0xc`: the module allowed to build one.
    pub token: u8,
    /// `+0x10`: the next module the scan offers the token to.
    pub seed: u8,
}

impl Default for GoalToken {
    /// `c+8 = 0`, `c+0xc = 0x80`, `c+0x10 = 0x40` (`FUN_004178f0`).
    fn default() -> Self {
        Self {
            pending: 0,
            token: 0x80,
            seed: 0x40,
        }
    }
}

impl GoalToken {
    /// Whether `module` holds the token.
    #[must_use]
    pub const fn held_by(self, module: Module) -> bool {
        self.token == module.goal_bit()
    }

    /// `FUN_004199b0`: with no goal pending, the token takes the seed and
    /// the seed moves one bit right (from `1` back to `0x80`); else the
    /// token moves right, wrapping at `0x80`, to the next pending bit, which
    /// it clears.
    pub fn pass(&mut self) {
        if self.pending == 0 {
            self.token = self.seed;
            self.seed = rotate_right(self.seed);
        } else {
            loop {
                self.token = rotate_right(self.token);
                if self.pending & self.token != 0 {
                    break;
                }
            }
            self.pending &= !self.token;
        }
    }
}

/// One bit right, `0` wrapping to `0x80` (`FUN_004199b0`).
const fn rotate_right(bit: u8) -> u8 {
    match bit >> 1 {
        0 => 0x80,
        next => next,
    }
}

// ---------------------------------------------------------------------------
// The system record (`FUN_00433e40`)
// ---------------------------------------------------------------------------

/// Coruscant's system index (`0x90000109`, `FUN_004d76f0`, `FUN_00433e40`).
const CORUSCANT: u32 = 0x109;

/// `sys+0x28` role bits, which survive the record's rebuild: the systems a
/// module claimed as its yard systems.
pub mod role {
    /// Modules 11, 10 and 7: a shipyard system (`FUN_004cafa0`).
    pub const SHIPYARD: u32 = 0x20_0000;
    /// Modules 5 and 6: a training system (`FUN_004d1970`, `FUN_004e15b0`).
    pub const TRAINING: u32 = 0x40_0000;
    /// Module 1: a construction-yard system (`FUN_004daf10`, `FUN_004db510`).
    pub const YARDS: u32 = 0x80_0000;
    /// Module 2 (`FUN_004e3810`).
    pub const RELIEF: u32 = 0x100_0000;
    /// Module 3 (`FUN_004d8ad0`, `FUN_004d8db0`).
    pub const DEFENCES: u32 = 0x200_0000;
    /// Every role: what the rebuild keeps (`FUN_00433e40`).
    pub const ANY: u32 = 0x3e0_0000;
    /// The three construction-yard roles (`FUN_004db510`, `D = 0x3800003`).
    pub const CONSTRUCTION: u32 = 0x380_0000;
}

/// One system as the side's AI sees it (`FUN_00433e40`; `ai-production.md`,
/// "F1"). Counts are the side's own objects.
#[derive(Debug, Clone, Default)]
pub struct SystemRecord {
    pub key: SystemKey,
    pub sector: crate::ids::SectorKey,
    /// `sys+0x28` (A).
    pub a: u32,
    /// `sys+0x30 & 1`, `& 4`: held by the side, by its enemy.
    pub own: bool,
    pub enemy: bool,
    pub populated: bool,
    pub coruscant: bool,
    /// `+0x48`: the side's popularity, 0..100.
    pub popularity: i64,
    /// `+0x4c`: the system's energy.
    pub energy: i64,
    /// `+0x50`, `+0x54`: mines and refineries.
    pub mines: i64,
    pub refineries: i64,
    /// `+0x58`, `+0x5c`, `+0x60`: shipyards, training facilities and
    /// construction yards.
    pub yards: [i64; 3],
    /// `+0x64`, `+0x6c`: shields (family `0x24`) and their target.
    pub shields: i64,
    pub shield_target: i64,
    /// `+0x70`: Death Star shields (family `0x25`).
    pub ds_shields: i64,
    /// `+0x74`, `+0x7c`: batteries (families `0x22`, `0x23`) and their
    /// target.
    pub batteries: i64,
    pub battery_target: i64,
    /// `+0x80`, `+0x84`: free raw-material nodes and energy slots.
    pub free_raw: i64,
    pub free_energy: i64,
    /// `+0x88`, `+0x90`: regiments present (any side), the side's
    /// regiments.
    pub regiments_present: i64,
    pub regiments: i64,
    /// `+0x98`, `+0x9c`, `+0xa0`: the regiment need, shortage and target.
    pub regiment_need: i64,
    pub regiment_shortage: i64,
    pub regiment_target: i64,
    /// `+0x94`: the regiment surplus.
    pub regiment_surplus: i64,
    /// `+0xa4`, `+0xac`: squadrons present, the side's.
    pub squadrons_present: i64,
    pub squadrons: i64,
    /// `+0xe0`, `+0x11c`: squadrons less the fighter target.
    pub fighter_balance: i64,
    pub fighter_surplus: i64,
    /// `+0xd0`, `+0xd4`, `+0xd8`: the yards' maintenance by family.
    pub yard_spend: [i64; 3],
    /// `+0xdc`: the defence balance; negative is a deficit.
    pub defence_balance: i64,
    /// `+0x114`: free facility slots.
    pub free_slots: i64,
    /// `+0x118`: the enemy threat at a neutral or enemy system.
    pub threat: i64,
    /// `sys+0x2c` (Q) and `sys+0x30` (S), the strategic words
    /// (`crate::ai_records::fill_system`).
    pub q: u32,
    pub s: u32,
    /// `+0x44`: the day the system was last seen blockaded.
    pub last_seen: i64,
    /// `+0x68`: the active shield generators' strength.
    pub shield_strength: i64,
    /// `+0x78`: the active batteries' attack.
    pub battery_attack: i64,
    /// `+0xac`, `+0xb0`, `+0xb4`: the active squadrons present (any side)
    /// and their attack and manoeuvre figures.
    pub squadrons_active: i64,
    pub squadron_attack: i64,
    pub squadron_manoeuvre: i64,
    /// `+0xe4`, `+0xe8`, `+0xec`, `+0xf4`: the foreign fleets' attack,
    /// manoeuvre and shield figures and squadrons.
    pub fleet_attack: i64,
    pub fleet_manoeuvre: i64,
    pub fleet_shield: i64,
    pub fleet_squadrons: i64,
    /// `+0x110`: an enemy headquarters flag and enemy leaders present.
    pub hq_count: i64,
}

/// `sys+0x28` bits the modules test (`ai-production.md`, "F1.2").
pub mod bit {
    /// `FUN_00433e40`.
    pub const UPRISING: u32 = 0x1;
    /// `FUN_00433e40`.
    pub const BLOCKADE: u32 = 0x2;
    /// `FUN_00433e40`.
    pub const BLOCKED: u32 = 0x3;
    /// `FUN_00433e40`.
    pub const FREE_ENERGY: u32 = 0x20;
    /// `FUN_00433e40`.
    pub const FREE_RAW: u32 = 0x40;
    /// `FUN_00433e40`.
    pub const MINE_SITE: u32 = 0x80;
    /// `FUN_00433e40`.
    pub const REFINERY_SITE: u32 = 0x100;
    /// `FUN_00433e40`.
    pub const SHIPYARD_READY: u32 = 0x200;
    /// `FUN_00433e40`.
    pub const TRAINING_READY: u32 = 0x400;
    /// `FUN_00433e40`.
    pub const YARD_READY: u32 = 0x800;
    /// `FUN_00433e40`.
    pub const ANY_YARD: u32 = 0x1000;
    /// `FUN_00433e40`.
    pub const REGIMENT_SHORT: u32 = 0x2000;
    /// `FUN_00433e40`.
    pub const DEFENCE_DEFICIT: u32 = 0x4000;
    /// `FUN_00433e40`.
    pub const SHIELD_SHORT: u32 = 0x8000;
    /// `FUN_00433e40`.
    pub const REGIMENT_SURPLUS: u32 = 0x1_0000;
    /// `FUN_00433e40`.
    pub const DEFENDER_SHORT: u32 = 0x2_0000;
    /// `FUN_00433e40`.
    pub const FIGHTER_SHORT: u32 = 0x4_0000;
    /// `FUN_00433e40`.
    pub const SQUADRONS: u32 = 0x8_0000;
    /// `FUN_00433e40`.
    pub const FIGHTER_SURPLUS: u32 = 0x10_0000;
    /// `FUN_00433e40`.
    pub const UNASSIGNED: u32 = 0x400_0000;
    /// `FUN_00433e40`.
    pub const DS_SHIELD_WANTED: u32 = 0x4000_0000;
    /// `FUN_00433e40`.
    pub const HQ: u32 = 0x8000_0000;
}

/// What the records read besides the world.
pub struct RecordInputs<'a> {
    pub world: &'a GameWorld,
    pub manufacturing: &'a ManufacturingState,
    pub uprisings: &'a crate::uprising::UprisingState,
    pub research: &'a crate::research::ResearchState,
    pub side: Faction,
    /// `c+0x364..+0x390`.
    pub targets: &'a [i64; 12],
    /// The role bits the modules set.
    pub roles: &'a HashMap<SystemKey, u32>,
}

impl SystemRecord {
    /// Build the record of `key` (`FUN_00433e40`).
    ///
    /// hyp: every regiment and squadron of the side counts as a defender
    /// (`+0x8c`, `+0xa8`; the original keeps the ones whose class figures
    /// reach the side's best, `class-table+0x24`, `+0x28`). A yard's
    /// producer is idle when its area has no queue (`producer+0x58`).
    #[must_use]
    pub fn build(inputs: &RecordInputs<'_>, key: SystemKey) -> Option<Self> {
        let world = inputs.world;
        let side = inputs.side;
        let system = world.systems.get(key)?;
        let is_alliance = side == Faction::Alliance;
        let owner = system.control.faction();
        let mut r = Self {
            key,
            sector: system.sector,
            a: inputs.roles.get(&key).copied().unwrap_or(0) & role::ANY,
            own: owner == Some(side),
            enemy: owner.is_some_and(|o| o != side && o != Faction::Neutral),
            populated: system.is_populated,
            coruscant: system.dat_id.raw() & 0xff_ffff == CORUSCANT,
            energy: i64::from(system.total_energy),
            ..Self::default()
        };
        if inputs.uprisings.is_uprising(key) {
            r.a |= bit::UPRISING;
        }
        if crate::blockade::BlockadeSystem::system_is_blockaded(world, system) {
            r.a |= bit::BLOCKADE;
        }
        if r.coruscant || system.is_headquarters {
            r.a |= bit::HQ;
        }
        r.popularity = if r.own { 100 } else { 0 };
        if r.populated {
            let popularity = if is_alliance {
                system.popularity_alliance
            } else {
                system.popularity_empire
            };
            #[expect(clippy::cast_possible_truncation, reason = "a share in 0..=1")]
            {
                r.popularity = (f64::from(popularity) * 100.0).round() as i64;
            }
        }
        let (energy_used, raw_used) = crate::economy::calculate_resource_allocation(world, system);
        r.free_energy = r.energy - i64::from(energy_used);
        r.free_raw = i64::from(system.raw_materials) - i64::from(raw_used);
        let maintenance = |id: &crate::ids::DatId| {
            world
                .buildable_classes
                .get(id)
                .map_or(0, |class| i64::from(class.maintenance_cost))
        };
        for facility in system
            .production_facilities
            .iter()
            .filter_map(|k| world.production_facilities.get(*k))
            .filter(|f| f.side == side)
        {
            if facility.is_mine {
                r.mines += 1;
            } else {
                r.refineries += 1;
            }
        }
        for yard in system
            .manufacturing_facilities
            .iter()
            .filter_map(|k| world.manufacturing_facilities.get(*k))
            .filter(|y| y.side == side)
        {
            if let Some(family) = YardFamily::of(yard.class_dat_id.family()) {
                r.yards[family.index()] += 1;
                r.yard_spend[family.index()] += maintenance(&yard.class_dat_id);
                r.a |= bit::ANY_YARD;
            }
        }
        let blocked = r.a & bit::BLOCKED != 0;
        for (family, area, ready) in [
            (
                YardFamily::Shipyard,
                crate::manufacturing::ProductionArea::Shipyard,
                bit::SHIPYARD_READY,
            ),
            (
                YardFamily::TrainingFacility,
                crate::manufacturing::ProductionArea::TrainingFacility,
                bit::TRAINING_READY,
            ),
            (
                YardFamily::ConstructionYard,
                crate::manufacturing::ProductionArea::ConstructionYard,
                bit::YARD_READY,
            ),
        ] {
            let idle = inputs
                .manufacturing
                .queue(key, area)
                .is_none_or(crate::manufacturing::ProductionQueue::is_empty);
            if r.own && r.yards[family.index()] > 0 && !blocked && idle {
                r.a |= ready;
            }
        }
        for facility in system
            .defense_facilities
            .iter()
            .filter_map(|k| world.defense_facilities.get(*k))
            .filter(|d| d.side == side)
        {
            match facility.class_dat_id.family() {
                0x22 | 0x23 => r.batteries += 1,
                0x24 => r.shields += 1,
                0x25 => r.ds_shields += 1,
                _ => {}
            }
        }
        r.regiments_present = system.ground_units.len() as i64;
        r.regiments = system
            .ground_units
            .iter()
            .filter_map(|t| world.troops.get(*t))
            .filter(|t| t.is_alliance == is_alliance)
            .count() as i64;
        r.squadrons_present = system.fighters.iter().map(|e| i64::from(e.count)).sum();
        r.squadrons = system
            .fighters
            .iter()
            .filter(|e| e.is_alliance == is_alliance)
            .map(|e| i64::from(e.count))
            .sum();
        if r.own && r.squadrons > 0 {
            r.a |= bit::SQUADRONS;
        }
        let targets = inputs.targets;
        let special = r.a & (role::ANY | bit::HQ) != 0;
        if r.own && r.populated {
            // The regiment need: (0x4f - popularity) / 10, at most 6, while
            // popularity is below 0x46; doubled in an uprising.
            if r.popularity < 0x46 {
                r.regiment_need = ((0x4f - r.popularity) / 10).min(6);
                if r.a & bit::UPRISING != 0 {
                    r.regiment_need *= 2;
                }
            }
            let base = if r.a & bit::HQ != 0 {
                targets[8]
            } else if r.a & role::ANY != 0 {
                targets[7]
            } else {
                targets[6]
            };
            r.regiment_target = base.max(r.regiment_need);
            if r.regiments < r.regiment_target {
                r.regiment_shortage = r.regiment_target - r.regiments;
                r.a |= bit::REGIMENT_SHORT | bit::DEFENDER_SHORT;
            } else if r.regiments > r.regiment_target {
                r.regiment_surplus = r.regiments - r.regiment_target;
                r.a |= bit::REGIMENT_SURPLUS;
            }
            r.set_defence_targets(inputs, targets);
        }
        if r.own {
            let target = if r.a & bit::HQ != 0 {
                targets[11]
            } else if r.a & role::ANY != 0 {
                targets[10]
            } else {
                targets[9]
            };
            r.fighter_balance = if special || r.regiments >= 1 {
                r.squadrons - target
            } else {
                r.squadrons_present
            };
            r.fighter_surplus = r.squadrons - target;
            if r.fighter_balance < 0 {
                r.a |= bit::FIGHTER_SHORT;
            }
            if r.fighter_surplus < 0 {
                r.a |= bit::FIGHTER_SURPLUS;
            }
            if !blocked && r.a & role::ANY == 0 {
                r.a |= bit::UNASSIGNED;
            }
            if r.free_energy > 0 {
                r.a |= bit::FREE_ENERGY;
                if r.free_raw > 0 {
                    r.a |= bit::FREE_RAW;
                }
            }
            let used = r.batteries
                + r.refineries
                + r.mines
                + r.ds_shields
                + r.shields
                + i64::from(r.a & bit::HQ != 0);
            let site = (!r.populated && r.a & bit::FREE_ENERGY != 0)
                || (r.populated && r.a & role::ANY == 0 && !blocked && used < r.energy);
            if site {
                r.a |= bit::REFINERY_SITE;
                if r.free_raw > 0 {
                    r.a |= bit::MINE_SITE;
                }
            }
        }
        if !r.own {
            r.threat = if r.enemy {
                r.regiments_present + r.regiment_need + r.batteries
            } else {
                r.regiment_need + r.batteries
            };
        }
        let mut slots = r.energy;
        if r.a & role::SHIPYARD != 0 {
            slots -= r.yards[0];
        }
        if r.a & role::TRAINING != 0 {
            slots -= r.yards[1];
        }
        if r.a & role::CONSTRUCTION != 0 {
            slots -= r.yards[2];
        }
        slots -= r.shields.max(r.shield_target);
        slots -= r.batteries.max(r.battery_target);
        slots -= i64::from(r.a & bit::HQ != 0);
        r.free_slots = slots;
        Some(r)
    }

    /// The defence targets `+0x6c`, `+0x7c`, the balance `+0xdc` and the
    /// Death Star shield wish (`FUN_00433e40` tail; "F1.4").
    fn set_defence_targets(&mut self, inputs: &RecordInputs<'_>, targets: &[i64; 12]) {
        let energy = self.energy;
        if self.coruscant {
            let reserve = i64::from(inputs.side == Faction::Empire);
            let (shields, batteries) = (targets[1], targets[0]);
            if energy < reserve + 2 {
                self.shield_target = (energy - reserve).max(0);
                self.battery_target = 0;
            } else if energy < reserve + batteries + shields {
                self.shield_target = shields;
                self.battery_target = (energy - shields - reserve).max(0);
            } else {
                self.shield_target = shields;
                self.battery_target = batteries;
            }
            if inputs.side == Faction::Empire
                && self.ds_shields == 0
                && ds_shield_available(inputs.world, inputs.research)
                && own_death_star(inputs.world, inputs.side)
            {
                self.a |= bit::DS_SHIELD_WANTED;
            }
        } else if self.a & role::ANY != 0 {
            if targets[4] + targets[2] < energy {
                self.shield_target = targets[2];
                self.battery_target = targets[4];
            } else if targets[4] < energy {
                self.shield_target = targets[4];
            }
        } else if self.regiment_need > 0 && targets[5] + targets[4] < energy {
            self.shield_target = targets[4];
            self.battery_target = targets[5];
        }
        let a = self.shields - self.shield_target;
        let b = self.batteries - self.battery_target;
        self.defence_balance = match (a > 0, b > 0) {
            (true, false) => a,
            (false, true) => b,
            _ => a + b,
        };
        if a < 0 {
            self.a |= bit::SHIELD_SHORT;
        }
        if a <= 0 && b <= 0 && self.defence_balance < 0 {
            self.a |= bit::DEFENCE_DEFICIT;
        }
    }

    /// `(A & mask) == mask`.
    #[must_use]
    pub const fn has(&self, mask: u32) -> bool {
        self.a & mask == mask
    }

    /// The system key slot `k` (`sys + 0x48 + 4k`) the queries rank by
    /// (`FUN_0041a340`; slots filled by `FUN_00433e40`).
    #[must_use]
    pub fn slot(&self, k: u32) -> i64 {
        match k {
            0 => self.popularity,
            1 => self.energy,
            2 => self.mines,
            3 => self.refineries,
            4 => self.yards[0],
            5 => self.yards[1],
            6 => self.yards[2],
            7 => self.shields,
            8 => self.shield_strength,
            0xa => self.ds_shields,
            0xb => self.batteries,
            0xc => self.battery_attack,
            0xe => self.free_raw,
            0xf => self.free_energy,
            0x10 => self.regiments_present,
            // `+0x90`: the regiments present; the side's own in its
            // systems.
            0x12 => {
                if self.own {
                    self.regiments
                } else {
                    self.regiments_present
                }
            }
            0x13 => self.regiment_surplus,
            0x14 => self.regiment_need,
            0x15 => self.regiment_shortage,
            0x16 => self.regiment_target,
            0x17 => self.squadrons_present,
            0x19 => self.squadrons_active,
            0x1a => self.squadron_attack,
            0x1b => self.squadron_manoeuvre,
            0x21 => self.yard_spend.iter().sum(),
            0x25 => self.defence_balance,
            0x26 => self.fighter_balance,
            0x27 => self.fleet_attack,
            0x28 => self.fleet_manoeuvre,
            0x29 => self.fleet_shield,
            0x2b => self.fleet_squadrons,
            0x32 => self.hq_count,
            0x33 => self.free_slots,
            0x34 => self.threat,
            0x35 => self.fighter_surplus,
            _ => 0,
        }
    }
}

/// Class `0x25000004`, the Death Star shield, is producible for the
/// Empire (`c+4 & 0x1000000`, `FUN_004197b0`).
pub(crate) fn ds_shield_available(
    world: &GameWorld,
    research: &crate::research::ResearchState,
) -> bool {
    let id = crate::ids::DatId::new(0x2500_0004);
    world.buildable_classes.get(&id).is_some_and(|class| {
        class.serves(false)
            && research.is_unlocked(
                false,
                class.research_order,
                crate::research::TechType::Facility,
            )
    })
}

/// The side has a Death Star ship (`c+4 & 4`, `FUN_0041ba80`).
fn own_death_star(world: &GameWorld, side: Faction) -> bool {
    world
        .fleets
        .values()
        .any(|f| f.is_alliance == (side == Faction::Alliance) && f.has_death_star)
}

// ---------------------------------------------------------------------------
// Sector records (`FUN_00432350`) and the queries
// ---------------------------------------------------------------------------

/// One sector as the side's AI sees it (`FUN_004321e0`, `FUN_00432350`):
/// the words `rec+0x24`, `+0x28`, `+0x2c` and the sums at `rec+0x70 + 4k`.
#[derive(Debug, Clone, Default)]
pub struct SectorRecord {
    pub key: crate::ids::SectorKey,
    /// `rec+0x24`.
    pub flags: u32,
    /// `rec+0x28`.
    pub c28: u32,
    /// `rec+0x2c`.
    pub b: u32,
    pub systems: Vec<SystemKey>,
    /// Key slots by index (`rec + 0x70 + 4k`).
    pub slots: HashMap<u32, i64>,
    /// `rec+0x40`: the sector's map position.
    pub x: i64,
    pub y: i64,
    /// `rec+0x30`..`+0x3c`: the plan's kind, its parameter, the regional
    /// record and the plan entry.
    pub mark: crate::ai_records::SectorMark,
    /// `rec+0x18c`: -1 when a kind lacks its record, 1 when a record lacks
    /// its kind.
    pub sync: i64,
}

/// Sector slot indexes (`rec + 0x70 + 4k`).
pub mod slot {
    pub const VISIBLE: u32 = 0;
    pub const POPULATED: u32 = 1;
    /// `rec+0x78`: systems either side holds (`FUN_00432350`).
    pub const HELD: u32 = 2;
    /// `rec+0x7c`: the side's systems (`FUN_00432350`).
    pub const OWN: u32 = 3;
    /// `rec+0x80`: the other side's systems (`FUN_00432350`).
    pub const ENEMY: u32 = 4;
    /// `rec+0x84`: neutral systems (`FUN_00432350`).
    pub const NEUTRAL: u32 = 5;
    /// `rec+0xa0`: the enemy systems' threat (`FUN_00432350`).
    pub const ENEMY_THREAT: u32 = 0xc;
    /// `rec+0x108`: the own systems' yard maintenance (`FUN_00432350`).
    pub const OWN_YARDS: u32 = 0x26;
    /// `rec+0x130`: the enemy systems' yard maintenance (`FUN_00432350`).
    pub const ENEMY_YARDS: u32 = 0x30;
    /// `rec+0x144`: the weakest enemy shield strength (`FUN_00432350`).
    pub const WEAKEST_SHIELD: u32 = 0x35;
    /// `rec+0x190`, `+0x194`, `+0x198`: the foreign fleets' attack,
    /// manoeuvre and squadrons (`FUN_00432350`).
    pub const FLEET_ATTACK: u32 = 0x48;
    /// `rec+0x194`: manoeuvre (`FUN_00432350`).
    pub const FLEET_MANOEUVRE: u32 = 0x49;
    /// `rec+0x198`: squadrons (`FUN_00432350`).
    pub const FLEET_SQUADRONS: u32 = 0x4a;
    /// `rec+0x19c`: enemy headquarters flags and leaders (`FUN_00432350`).
    pub const HQ: u32 = 0x4b;
    /// `rec+0x1a0`: own unblockaded headquarters or Coruscant
    /// (`FUN_00432350`).
    pub const OWN_HQ: u32 = 0x4c;
}

impl SectorRecord {
    /// `FUN_004321e0`: fold every system record (`FUN_00432350`), then the
    /// derived bits and the record sync.
    fn build(
        world: &GameWorld,
        key: crate::ids::SectorKey,
        records: &HashMap<SystemKey, SystemRecord>,
        marks: &crate::ai_records::Marks,
    ) -> Self {
        let sector = &world.sectors[key];
        let mark = marks.sector(key);
        let mut rec = Self {
            key,
            systems: sector.systems.clone(),
            x: i64::from(sector.x),
            y: i64::from(sector.y),
            mark,
            ..Self::default()
        };
        rec.slots.insert(slot::WEAKEST_SHIELD, 100_000);
        for r in sector.systems.iter().filter_map(|s| records.get(s)) {
            rec.fold(r);
        }
        if rec.b & 0x100 == 0 {
            rec.slots.insert(slot::WEAKEST_SHIELD, 0);
        }
        let own = rec.slot(slot::OWN);
        if own > 0 {
            let total = rec.slot(7);
            rec.slots.insert(7, total / own);
        }
        if rec.slot(slot::VISIBLE) == 10 {
            rec.c28 |= 0x4000_0000;
        }
        if rec.slot(slot::POPULATED) == 10 {
            rec.b |= 0x1;
        }
        if rec.slot(slot::HELD) == 10 {
            rec.b |= 0x2;
        }
        if rec.slot(slot::HQ) > 0 {
            rec.b |= 0x80;
        }
        rec.set_mark(mark);
        rec
    }

    /// `FUN_00432350`: one system's record into the sector's.
    #[expect(clippy::too_many_lines, reason = "one decompiled fold")]
    fn fold(&mut self, r: &SystemRecord) {
        use crate::ai_records::sys;
        let add = |rec: &mut Self, k: u32, v: i64| *rec.slots.entry(k).or_insert(0) += v;
        let max = |rec: &mut Self, k: u32, v: i64| {
            let e = rec.slots.entry(k).or_insert(0);
            *e = (*e).max(v);
        };
        let (a, s, q) = (r.a, r.s, r.q);
        let v = |k: u32| r.slot(k);
        if s & sys::OWN != 0 {
            self.b |= 0x4;
            add(self, 3, 1);
            for (k, vk) in [
                (7, 0),
                (8, 1),
                (0xe, 2),
                (0xf, 3),
                (0x16, 7),
                (0x17, 0xa),
                (0x18, 0xb),
                (0x19, 0x17),
                (0xa, 0xe),
                (9, 0xf),
                (0xb, 0x16),
                (0x20, 0x10),
                (0x23, 0x1e),
                (0x24, 0x1f),
                (0x25, 0x20),
                (0x26, 0x21),
                (0x1b, 0x1a),
                (0x48, 0x27),
                (0x49, 0x28),
                (0x4a, 0x2b),
            ] {
                add(self, k, v(vk));
            }
            if s & 0x4_0000 != 0 {
                self.b |= 0x80_0000;
                if s & 0x1_0000 != 0 {
                    self.b |= 0x100_0000;
                } else if s & 0x2_0000 != 0 {
                    self.b |= 0x200_0000;
                }
            }
            let open = a & bit::BLOCKED == 0;
            if a & bit::BLOCKADE == 0 {
                self.flags |= 0x8;
            }
            if open && s & sys::SPECIAL == 0 && v(0x33) > 0 {
                if a & role::SHIPYARD == 0 {
                    self.flags |= 0x20;
                }
                if a & role::TRAINING == 0 {
                    self.flags |= 0x40;
                }
                if a & role::CONSTRUCTION == 0 {
                    self.flags |= 0x80;
                }
            }
            if open {
                for (b, f) in [
                    (bit::MINE_SITE, 0x100),
                    (bit::REFINERY_SITE, 0x200),
                    (bit::FREE_ENERGY, 0x400),
                    (bit::FREE_RAW, 0x800),
                ] {
                    if a & b != 0 {
                        self.flags |= f;
                    }
                }
                if a & role::YARDS != 0 && a & bit::YARD_READY != 0 {
                    self.flags |= 0x1000;
                }
            }
            if a & bit::YARD_READY != 0 && a & role::CONSTRUCTION == 0 {
                self.flags |= 0x2000;
            }
            if a & role::ANY != 0 {
                self.flags |= 0x2000_0000;
                if a & role::SHIPYARD != 0 {
                    if a & bit::BLOCKADE == 0 {
                        self.b |= 0x2_0000;
                    }
                    add(self, 0x10, v(4));
                }
                if a & role::TRAINING != 0 {
                    add(self, 0x12, v(5));
                }
                if a & role::CONSTRUCTION != 0 {
                    add(self, 0x14, v(6));
                } else {
                    add(self, 0x15, v(6));
                }
            } else {
                add(self, 0x15, v(6));
            }
            if a & role::TRAINING == 0 {
                add(self, 0x13, v(5));
            }
            if a & role::SHIPYARD == 0 {
                add(self, 0x11, v(4));
            }
            if s & sys::TAGGED != 0 {
                add(self, 0x46, 1);
                if s & 0x2000_0000 == 0 {
                    self.b |= 0x1000_0000;
                }
            }
            let balance = v(0x25);
            if balance < 0 {
                self.flags |= 0x8000;
                add(self, 0x1c, 1);
                if a & role::ANY != 0 {
                    self.flags |= 0x4000;
                }
            } else if balance > 0 {
                self.flags |= 0x1_0000;
                add(self, 0x1d, 1);
            }
            if a & bit::HQ == 0 && v(0xb) > 0 && v(7) > 0 {
                self.flags |= 0x2_0000;
            }
            if a & bit::REGIMENT_SURPLUS != 0 {
                self.flags |= 0x8_0000;
                add(self, 0x1f, v(0x13));
                if a & 0x1000_0000 != 0 {
                    self.flags |= 0x10_0000;
                }
                if a & 0x2000_0000 != 0 {
                    self.flags |= 0x20_0000;
                }
                if s & 0x4000 != 0 {
                    self.b |= 0x800;
                }
                if s & 0x8000 != 0 {
                    self.b |= 0x1000;
                }
            }
            if a & bit::REGIMENT_SHORT != 0 {
                self.flags |= 0x4_0000;
                add(self, 0x1e, v(0x15));
                if a & role::ANY != 0 {
                    self.flags |= 0x80_0000;
                }
            }
            if a & bit::DEFENDER_SHORT != 0 {
                self.flags |= 0x40_0000;
            }
            if v(0x35) > 0 && s & 0x4_0000 == 0 && a & bit::BLOCKADE == 0 {
                self.flags |= 0x200_0000;
                add(self, 0x22, v(0x35));
            }
            if v(0x26) < 0 {
                self.flags |= 0x100_0000;
                add(self, 0x21, 1);
                if a & role::ANY != 0 {
                    self.flags |= 0x400_0000;
                }
                if a & bit::FIGHTER_SURPLUS != 0 {
                    self.flags |= 0x1000_0000;
                }
            }
            if a & bit::SQUADRONS != 0 {
                self.flags |= 0x800_0000;
            }
            if a & bit::UPRISING != 0 {
                self.flags |= 0x10;
            }
            if a & bit::HQ != 0 {
                self.flags |= 0x4000_0000;
                if s & sys::CORUSCANT != 0 {
                    self.b |= 0x200;
                }
                if a & bit::BLOCKADE == 0 {
                    add(self, slot::OWN_HQ, 1);
                }
            }
            if q & 0x1000_0000 != 0 {
                self.c28 |= 0x800_0000;
            }
        } else if s & sys::ENEMY != 0 {
            self.b |= 0x8;
            add(self, 4, 1);
            for (k, vk) in [
                (0x29, 2),
                (0x2a, 3),
                (0x2e, 7),
                (0x2f, 0xa),
                (0x48, 0x27),
                (0x49, 0x28),
                (0x4a, 0x2b),
                (0xc, 0x34),
                (0x30, 0x21),
                (0x2b, 4),
                (0x2c, 5),
                (0x2d, 6),
            ] {
                add(self, k, v(vk));
            }
            add(self, 0x28, v(1) - v(0xf));
            max(self, 0x31, v(0x34));
            max(self, 0x32, v(0x17));
            max(self, 0x33, v(0x1b));
            max(self, 0x34, v(0x1d));
            if a & bit::HQ != 0 {
                self.flags |= 0x8000_0000;
            }
            if s & sys::UNDEFENDED != 0 {
                self.b |= 0x100;
                let weakest = self.slot(slot::WEAKEST_SHIELD).min(v(8));
                self.slots.insert(slot::WEAKEST_SHIELD, weakest);
            }
            if s & sys::SHIELDS == 0 {
                self.b |= 0x400;
            }
            if s & 0x2000 == 0 {
                self.b |= 0x40_0000;
            }
            if q & 0x2f7_0000 != 0 {
                self.b |= 0x8_0000;
            }
        } else {
            self.b |= 0x10;
            add(self, 5, 1);
            for (k, vk) in [
                (0x48, 0x27),
                (0x49, 0x28),
                (0x4a, 0x2b),
                (0x2b, 4),
                (0x2c, 5),
                (0x2d, 6),
            ] {
                add(self, k, v(vk));
            }
            if a & bit::HQ != 0 {
                self.flags |= 0x4000_0000;
            }
            if s & sys::HELD != 0 {
                self.b |= 0x40;
            }
        }
        add(self, slot::HQ, v(0x32));
        if q & 0x800_0000 != 0 {
            add(self, slot::VISIBLE, 1);
        }
        if s & sys::POPULATED != 0 {
            add(self, slot::POPULATED, 1);
        }
        if s & sys::HELD != 0 {
            add(self, slot::HELD, 1);
        }
        if s & 0x10 != 0 {
            add(self, 6, 1);
        }
        if s & sys::RIM != 0 {
            self.b |= 0x20;
        }
        if s & sys::CORUSCANT != 0 {
            self.b |= 0x4000_0000;
        }
        if s & sys::HQ != 0 {
            self.b |= 0x8000_0000;
        }
        if s & sys::FOREIGN_FLEET != 0 {
            self.b |= 0x4_0000;
        }
    }

    #[must_use]
    pub fn slot(&self, k: u32) -> i64 {
        self.slots.get(&k).copied().unwrap_or(0)
    }

    /// A plan write-back (`FUN_00431790`, `FUN_004317b0`, `FUN_004317d0`):
    /// the new kind, record and entry, and the bits and sync they set.
    pub fn set_mark(&mut self, mark: crate::ai_records::SectorMark) {
        self.mark = mark;
        self.b &= !(0x800_0000 | 0x2000_0000 | 0x400_0000 | 0x1_0000);
        if mark.plan_id != 0 {
            self.b |= 0x800_0000;
        }
        if mark.exhausted {
            self.b |= 0x2000_0000;
        }
        self.sync = 0;
        if mark.kind != 0 {
            self.b |= 0x400_0000;
            if mark.record == 0 {
                self.sync = -1;
                self.b |= 0x1_0000;
            }
        } else if mark.record != 0 {
            self.sync = 1;
        }
    }
}

/// A ranked list's direction (`+0x1c` of the list, `FUN_0041c250`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    /// 1: the largest key ranks best.
    Largest,
    /// 2: the smallest key ranks best.
    Smallest,
}

/// Rank-sum selection (`FUN_00435790` + `FUN_004357b0`): each list is
/// sorted by its key with equal keys in random order; the pick is the id
/// present in every list with the largest sum of 1-based positions, the
/// first in the first list's order on a tie.
///
/// port: equal keys take their order from the AI's stream at once rather
/// than through the insertion sort's coin flips (`rand(10) < 5`).
#[must_use]
pub(crate) fn rank_pick<T: Copy + Eq>(
    lists: &[(Vec<(T, i64)>, Dir)],
    rng: &mut impl crate::uprising::Draws,
) -> Option<T> {
    let sorted: Vec<Vec<T>> = lists
        .iter()
        .map(|(items, dir)| {
            let mut keyed: Vec<(i64, i32, T)> = items
                .iter()
                .map(|&(id, key)| (key, rng.draw(9), id))
                .collect();
            keyed.sort_by(|a, b| match dir {
                Dir::Largest => a.0.cmp(&b.0).then(a.1.cmp(&b.1)),
                Dir::Smallest => b.0.cmp(&a.0).then(a.1.cmp(&b.1)),
            });
            keyed.into_iter().map(|(_, _, id)| id).collect()
        })
        .collect();
    let first = sorted.first()?;
    let mut best: Option<(T, usize)> = None;
    for &id in first {
        let mut score = 0;
        let mut everywhere = true;
        for list in &sorted {
            match list.iter().position(|&x| x == id) {
                Some(p) => score += p + 1,
                None => everywhere = false,
            }
        }
        if everywhere && best.is_none_or(|(_, s)| score > s) {
            best = Some((id, score));
        }
    }
    best.map(|(id, _)| id)
}

/// The filters of a system query (`FUN_0041a340`, `FUN_00431830`): A must
/// hold `require`, must hold none of `exclude`; `own` asks `sys+0x30 & 1`.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemFilter {
    pub require: u32,
    pub exclude: u32,
    pub own: bool,
}

impl SystemFilter {
    #[must_use]
    pub fn admits(self, r: &SystemRecord) -> bool {
        r.has(self.require) && r.a & self.exclude == 0 && (!self.own || r.own)
    }
}

// ---------------------------------------------------------------------------
// Orders, the module state and the round
// ---------------------------------------------------------------------------

/// A production order a module emits (`ai-production.md`, "Orders").
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Order {
    /// Build `0x210`/`0x211`/`0x212` at `producer`'s yards, after a
    /// Destination `0x214` to `destination` when set.
    Build {
        producer: SystemKey,
        kind: crate::manufacturing::BuildableKind,
        /// One unit's refined-material cost, the work its yards draw.
        work: u32,
        count: u32,
        destination: Option<crate::manufacturing::Destination>,
    },
    /// Scrap `0x200` of the listed objects.
    Scrap(Vec<crate::scrap::ScrapTarget>),
    /// Retire `0x242` of a special force.
    Retire(crate::ids::SpecialForceKey),
    /// Move `0x201` of one fleet's ships (by tag) to a system: module 11's
    /// repair move (`FUN_004c8e90`).
    MoveShips {
        fleet: crate::ids::FleetKey,
        ships: Vec<u32>,
        to: SystemKey,
    },
}

/// A module's persistent fields.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleState {
    /// `m+0x38`; 0 is the module's first phase.
    pub phase: u16,
    /// The role list (`m+0x78`, `m+0x74`, `m+0x6c`, `m+0x58`, `m+0x60`,
    /// `m+0x7c`): the systems the module claimed.
    pub list: Vec<SystemKey>,
    /// Module 11's request blockers (`m+0x28`), module 2's mine and
    /// refinery sites, module 5's class code (`m+0x40`), module 6's
    /// selector rotation (`m+0x78`).
    pub blockers: u32,
    pub sites: [Option<SystemKey>; 2],
    pub cursor: u32,
    /// The orders of the module's last phase still to issue, one a turn.
    /// port: the original steps a phase one call per order and re-reads
    /// the census at each; these were chosen on the census of the turn
    /// that ran the phase.
    #[serde(default)]
    pub queued: Vec<Order>,
    /// port: a bundled Destination and Build went out on the last turn; the
    /// next turn stands for the Build's own round (`ai-production.md`, F3).
    #[serde(default)]
    pub spent: bool,
    /// `m+0x3c`: the step a phase stopped at with an order, 0 when the
    /// phase ran to its end. The next turn resumes it.
    #[serde(default)]
    pub step: u8,
}

/// The side's production AI (`c+8..+0x10`, `c+0x14..+0x24`, `c+0x364..`,
/// the module list).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductionState {
    pub token: GoalToken,
    /// `c+0x364..+0x390` (`FUN_0041bd20`).
    pub targets: [i64; 12],
    /// `sys+0x28` role bits by system.
    #[serde(
        serialize_with = "crate::serde_ordered::serialize_hash_map",
        deserialize_with = "crate::serde_ordered::deserialize_hash_map"
    )]
    pub roles: HashMap<SystemKey, u32>,
    pub modules: [ModuleState; 8],
    /// The producer caches `c+0x14`, `+0x18`, `+0x1c`, `+0x20`, `+0x24`.
    pub caches: [Option<SystemKey>; 5],
    /// `c+4 & 0x40`: module 1 found no yard system.
    pub no_yard_site: bool,
}

impl Default for ProductionState {
    /// The targets `c+0x364`..`+0x390` start as `FUN_0041bd20` sets them;
    /// the token as `FUN_004178f0` does.
    fn default() -> Self {
        Self {
            token: GoalToken::default(),
            targets: [3, 3, 2, 1, 1, 0, 0, 3, 9, 0, 6, 12],
            roles: HashMap::new(),
            modules: Default::default(),
            caches: [None; 5],
            no_yard_site: false,
        }
    }
}

/// What one round reads.
pub struct RoundInputs<'a> {
    pub world: &'a GameWorld,
    pub manufacturing: &'a ManufacturingState,
    pub transport: &'a TroopTransportState,
    pub uprisings: &'a crate::uprising::UprisingState,
    pub research: &'a crate::research::ResearchState,
    pub movement: &'a crate::movement::MovementState,
    /// The strategic modules' marks (`crate::ai_records`).
    pub marks: &'a crate::ai_records::Marks,
    pub side: Faction,
    pub tick: u64,
    /// The in-day step whose round this is, 0 to the day's rate
    /// (`FUN_0051dea0` `+0x20`).
    pub round_in_day: u16,
}

/// One round's view: the census, the records and the stream.
pub struct Round<'a> {
    pub inputs: &'a RoundInputs<'a>,
    pub census: Census,
    pub records: HashMap<SystemKey, SystemRecord>,
    pub sectors: Vec<SectorRecord>,
    /// Every fleet's and tagged ship's record, both sides.
    pub fleets: HashMap<crate::ids::FleetKey, crate::ai_records::FleetRecord>,
    pub ships: HashMap<u32, crate::ai_records::ShipRecord>,
    pub(crate) rng: crate::mission_detection::MissionRng,
    /// `side+0x58 - +0x5c` as the round's orders spend it (`FUN_005385f0`).
    pub spare: i64,
    /// The context flags `c+4` module 13 leaves (`ai_strategy::ctx`).
    pub ctx: u32,
    /// The need bits `rec+0x64` of the side's regional records (context
    /// `+0xd8`) after module 8's pass, which module 11 reads.
    pub record_needs: Vec<u32>,
    /// Each held character's class G or I task id (character `+0x28`).
    pub task_of: HashMap<crate::ids::CharacterKey, u32>,
    /// The side's AI records changed since the census (`c+4 & 0x80000000`,
    /// `FUN_004184f0` callers); a turn that finishes dirty ends the round
    /// (`FUN_00485b40.c:59-67`).
    pub dirty: bool,
}

/// What one module turn did (`FUN_00485b40.c:31-63`).
pub(crate) enum Turn {
    /// The turn ended with this order (`:59`). `resume`: the order came in
    /// mid-pass, so `m+0x34` rises and the module goes first next round.
    Order {
        action: crate::ai::AIAction,
        resume: bool,
    },
    /// port: the Build half of a bundled Destination and Build. It ends the
    /// round like an order and counts toward the day's cap, but applies
    /// nothing.
    Spent,
    /// The turn ended without an order (`m+0x20` set, no `bVar1`).
    Finished,
}

impl ProductionState {
    /// A production module's turn: one phase (`step`), issued an order a
    /// turn. An order before a phase's last step sets done and raises
    /// `m+0x34`, and the last step sets done alone (`FUN_004c8ba0.c:12-14,
    /// 23-27, 31-33`). So an order resumes when the phase holds more orders
    /// or stopped before its end (`ModuleState::step`), and so does a
    /// bundled Destination and Build, whose Build the next turn spends
    /// (`Turn::Spent`).
    pub(crate) fn turn(&mut self, module: Module, round: &mut Round<'_>) -> Turn {
        let phase = self.module(module).phase;
        let state = self.module(module);
        let turn = if std::mem::take(&mut state.spent) {
            Turn::Spent
        } else if !state.queued.is_empty() {
            let order = state.queued.remove(0);
            self.issue(module, order)
        } else {
            let mut emitted = self.step(module, round);
            if emitted.is_empty() {
                Turn::Finished
            } else {
                let order = emitted.remove(0);
                self.module(module).queued = emitted;
                self.issue(module, order)
            }
        };
        let queued = self.module(module).queued.len();
        crate::trace::record(crate::trace::TraceMask::AI, "production_turn", || {
            let (order, resume) = match &turn {
                Turn::Order { action, resume } => (Some(format!("{action:?}")), *resume),
                Turn::Spent | Turn::Finished => (None, false),
            };
            serde_json::json!({
                "module": format!("{module:?}"),
                "phase": phase,
                "spent": matches!(turn, Turn::Spent),
                "order": order,
                "resume": resume,
                "queued": queued,
            })
        });
        turn
    }

    /// The turn that issues `order`: it resumes while the phase holds more
    /// orders or stopped before its end, or when it is a bundled Destination
    /// and Build.
    fn issue(&mut self, module: Module, order: Order) -> Turn {
        let state = self.module(module);
        let bundled = matches!(
            order,
            Order::Build {
                destination: Some(_),
                ..
            }
        );
        state.spent = bundled;
        Turn::Order {
            resume: bundled || !state.queued.is_empty() || state.step != 0,
            action: crate::ai::AIAction::from(order),
        }
    }

    /// Slot 6 (`FUN_004dd810.c:5-8`): a refused order restarts the module,
    /// its phase at the start and its queued orders dropped.
    pub(crate) fn reject(&mut self, module: Module) {
        let state = self.module(module);
        state.phase = 0;
        state.step = 0;
        state.queued.clear();
        state.spent = false;
    }

    fn step(&mut self, module: Module, round: &mut Round<'_>) -> Vec<Order> {
        match module {
            Module::Defences => self.defences(round),
            Module::Yards => self.yards(round),
            Module::DeficitRelief => self.relief(round),
            Module::CapitalShips => self.capital_ships(round),
            Module::FleetFighters => self.fleet_fighters(round),
            Module::SystemFighters => self.system_fighters(round),
            Module::SpecialForces => self.special_forces(round),
            Module::Regiments => self.regiments(round),
        }
    }

    fn module(&mut self, module: Module) -> &mut ModuleState {
        let index = Module::ALL.iter().position(|&m| m == module).unwrap_or(0);
        &mut self.modules[index]
    }

    /// Give `system` the role `bit` and add it to `module`'s list. The role
    /// write marks the system record (`FUN_00433e30` -> `FUN_004321d0` ->
    /// `FUN_004184f0`, e.g. `FUN_004cacc0.c:113-114`), so the round is dirty.
    fn claim(&mut self, module: Module, system: SystemKey, bit: u32, round: &mut Round<'_>) {
        round.dirty = true;
        *self.roles.entry(system).or_insert(0) |= bit;
        if let Some(record) = round.records.get_mut(&system) {
            record.a |= bit;
        }
        let list = &mut self.module(module).list;
        if !list.contains(&system) {
            list.push(system);
        }
    }

    /// Clear `system`'s role `bit` and drop it from `module`'s list; dirty,
    /// as `claim` (e.g. `FUN_004ca300.c:113-116`).
    fn release(&mut self, module: Module, system: SystemKey, bit: u32, round: &mut Round<'_>) {
        round.dirty = true;
        if let Some(roles) = self.roles.get_mut(&system) {
            *roles &= !bit;
        }
        if let Some(record) = round.records.get_mut(&system) {
            record.a &= !bit;
        }
        self.module(module).list.retain(|&s| s != system);
    }
}

impl<'a> Round<'a> {
    /// The round's census and records (`FUN_00418500`, `FUN_00433e40`,
    /// `FUN_00432350`) and its stream.
    pub fn new(state: &ProductionState, inputs: &'a RoundInputs<'a>) -> Self {
        let census = Census::take(
            inputs.world,
            inputs.manufacturing,
            inputs.transport,
            inputs.side,
        );
        let record_inputs = RecordInputs {
            world: inputs.world,
            manufacturing: inputs.manufacturing,
            uprisings: inputs.uprisings,
            research: inputs.research,
            side: inputs.side,
            targets: &state.targets,
            roles: &state.roles,
        };
        let fleet_inputs = crate::ai_records::RecordWorld {
            world: inputs.world,
            movement: inputs.movement,
            transport: inputs.transport,
            marks: inputs.marks,
            side: inputs.side,
        };
        let (fleets, ships) = crate::ai_records::fleet_records(&fleet_inputs);
        let day = i64::try_from(inputs.tick).unwrap_or(i64::MAX);
        let records: HashMap<SystemKey, SystemRecord> = inputs
            .world
            .systems
            .keys()
            .filter_map(|key| {
                let mut r = SystemRecord::build(&record_inputs, key)?;
                crate::ai_records::fill_system(&mut r, &fleet_inputs, &fleets, day);
                Some((key, r))
            })
            .collect();
        let mut keys: Vec<_> = inputs.world.sectors.keys().collect();
        keys.sort_by_key(|k| inputs.world.sectors[*k].dat_id.raw());
        let sectors = keys
            .into_iter()
            .map(|k| SectorRecord::build(inputs.world, k, &records, inputs.marks))
            .collect();
        let spare = census.capacity - census.load;
        Self {
            inputs,
            census,
            records,
            sectors,
            fleets,
            ships,
            // port: the original draws from the session generator
            // (`DAT_006b14d8`); each round seeds its own stream, the day's
            // first as before.
            rng: crate::mission_detection::MissionRng::seeded(
                0.0,
                inputs.tick.wrapping_mul(4)
                    + u64::from(inputs.side == Faction::Alliance)
                    + 0x5052_4f44
                    + (u64::from(inputs.round_in_day) << 32),
            ),
            spare,
            ctx: 0,
            record_needs: Vec::new(),
            task_of: HashMap::new(),
            dirty: false,
        }
    }

    fn record(&self, key: SystemKey) -> Option<&SystemRecord> {
        self.records.get(&key)
    }

    /// `FUN_00419a00`: the best sector holding `flags`, ranked by the key
    /// slots (a rank sum when several).
    fn sector(&mut self, flags: u32, keys: &[(u32, Dir)]) -> Option<crate::ids::SectorKey> {
        let candidates: Vec<&SectorRecord> = self
            .sectors
            .iter()
            .filter(|s| s.flags & flags == flags)
            .collect();
        let lists: Vec<(Vec<(crate::ids::SectorKey, i64)>, Dir)> = keys
            .iter()
            .map(|&(slot, dir)| {
                (
                    candidates.iter().map(|s| (s.key, s.slot(slot))).collect(),
                    dir,
                )
            })
            .collect();
        rank_pick(&lists, &mut self.rng)
    }

    /// `FUN_00419b80`: the sector holding `flags` nearest `from`'s.
    /// hyp: distance between sector positions (`FUN_0041b7b0`).
    fn nearest_sector(&mut self, from: SystemKey, flags: u32) -> Option<crate::ids::SectorKey> {
        let origin = self.inputs.world.systems.get(from)?.sector;
        let o = self.sectors.iter().find(|s| s.key == origin)?;
        let (ox, oy) = (o.x, o.y);
        let items: Vec<(crate::ids::SectorKey, i64)> = self
            .sectors
            .iter()
            .filter(|s| s.flags & flags == flags)
            .map(|s| (s.key, (s.x - ox).pow(2) + (s.y - oy).pow(2)))
            .collect();
        rank_pick(&[(items, Dir::Smallest)], &mut self.rng)
    }

    /// `FUN_00419b80` with a `rec+0x2c` mask: the sector holding `flags` in
    /// `rec+0x24` and `b` in `rec+0x2c` nearest `from`'s.
    fn nearest_sector_with(
        &mut self,
        from: SystemKey,
        flags: u32,
        b: u32,
    ) -> Option<crate::ids::SectorKey> {
        let origin = self.inputs.world.systems.get(from)?.sector;
        let o = self.sectors.iter().find(|s| s.key == origin)?;
        let (ox, oy) = (o.x, o.y);
        let items: Vec<(crate::ids::SectorKey, i64)> = self
            .sectors
            .iter()
            .filter(|s| s.flags & flags == flags && s.b & b == b)
            .map(|s| (s.key, (s.x - ox).pow(2) + (s.y - oy).pow(2)))
            .collect();
        rank_pick(&[(items, Dir::Smallest)], &mut self.rng)
    }

    /// `FUN_0041a340`: the best system of `sector` the filter admits.
    fn system_in(
        &mut self,
        sector: crate::ids::SectorKey,
        filter: SystemFilter,
        keys: &[(u32, Dir)],
    ) -> Option<SystemKey> {
        let systems = self
            .sectors
            .iter()
            .find(|s| s.key == sector)?
            .systems
            .clone();
        let admitted: Vec<&SystemRecord> = systems
            .iter()
            .filter_map(|s| self.records.get(s))
            .filter(|r| filter.admits(r))
            .collect();
        let lists: Vec<(Vec<(SystemKey, i64)>, Dir)> = keys
            .iter()
            .map(|&(slot, dir)| {
                (
                    admitted.iter().map(|r| (r.key, r.slot(slot))).collect(),
                    dir,
                )
            })
            .collect();
        rank_pick(&lists, &mut self.rng)
    }

    /// `FUN_0041a400` (no decompile): the system of `sector` the filter
    /// admits nearest `near`. hyp: by system position.
    fn nearest_system_in(
        &mut self,
        sector: crate::ids::SectorKey,
        near: SystemKey,
        filter: SystemFilter,
    ) -> Option<SystemKey> {
        let world = self.inputs.world;
        let at = world
            .systems
            .get(near)
            .map(|s| (i64::from(s.x), i64::from(s.y)))?;
        let systems = self
            .sectors
            .iter()
            .find(|s| s.key == sector)?
            .systems
            .clone();
        let items: Vec<(SystemKey, i64)> = systems
            .iter()
            .filter_map(|s| self.records.get(s))
            .filter(|r| filter.admits(r))
            .filter_map(|r| {
                let s = world.systems.get(r.key)?;
                Some((
                    r.key,
                    (i64::from(s.x) - at.0).pow(2) + (i64::from(s.y) - at.1).pow(2),
                ))
            })
            .collect();
        rank_pick(&[(items, Dir::Smallest)], &mut self.rng)
    }

    /// `FUN_0041b230`: the side's producible class of `area` whose node
    /// flags hold `mask`, best by `key` (0: at random); with its
    /// maintenance.
    fn class(
        &mut self,
        area: crate::manufacturing::ProductionArea,
        mask: u32,
        key: u32,
        dir: Dir,
    ) -> Option<(crate::manufacturing::BuildableKind, i64)> {
        let ranked = self.classes(area, mask, key, dir);
        ranked.into_iter().next()
    }

    /// `FUN_0041b270`: the whole list, best first.
    fn classes(
        &mut self,
        area: crate::manufacturing::ProductionArea,
        mask: u32,
        key: u32,
        dir: Dir,
    ) -> Vec<(crate::manufacturing::BuildableKind, i64)> {
        let world = self.inputs.world;
        let is_alliance = self.inputs.side == Faction::Alliance;
        let mut nodes: Vec<(crate::manufacturing::BuildableKind, ClassNode)> =
            crate::build_selection::listed_classes(world, self.inputs.research, area, is_alliance)
                .into_iter()
                .filter_map(|kind| Some((kind, ClassNode::of(world, kind)?)))
                .filter(|(_, node)| node.flags & mask == mask)
                .collect();
        if key == 0 {
            if nodes.is_empty() {
                return Vec::new();
            }
            let n = i32::try_from(nodes.len() - 1).unwrap_or(0);
            let pick = usize::try_from(crate::uprising::Draws::draw(&mut self.rng, n)).unwrap_or(0);
            let (kind, node) = nodes.swap_remove(pick.min(nodes.len() - 1));
            return vec![(kind, node.maintenance)];
        }
        let mut out = Vec::new();
        while !nodes.is_empty() {
            let items: Vec<(usize, i64)> = nodes
                .iter()
                .enumerate()
                .map(|(i, (_, n))| (i, n.key(key)))
                .collect();
            let Some(best) = rank_pick(&[(items, dir)], &mut self.rng) else {
                break;
            };
            let (kind, node) = nodes.remove(best);
            out.push((kind, node.maintenance));
        }
        out
    }

    /// The executor's maintenance test (`FUN_005385f0` → `FUN_0052e970`,
    /// `0xf3`): an order whose units carry maintenance beyond the spare
    /// capacity is refused; units without maintenance always pass.
    fn afford(&mut self, kind: crate::manufacturing::BuildableKind, count: u32) -> bool {
        let maintenance = crate::build_selection::unit_costs(self.inputs.world, kind)
            .map_or(0, |(_, m)| i64::from(m));
        let total = maintenance * i64::from(count);
        if total != 0 && total > self.spare {
            return false;
        }
        self.spare -= total;
        true
    }

    /// A Build order, refused when the side cannot carry its maintenance.
    fn build(
        &mut self,
        producer: SystemKey,
        kind: crate::manufacturing::BuildableKind,
        count: u32,
        destination: Option<SystemKey>,
    ) -> Option<Order> {
        self.build_to(
            producer,
            kind,
            count,
            destination.map(crate::manufacturing::Destination::System),
        )
    }

    /// A Build order after a Destination to a system or a fleet. The
    /// Destination goes out even to the producer's own system, which sets
    /// the area back home (modules 10, 7, 6 and the facility modules; 11
    /// and 5 issue none, so their products take the area's standing one).
    fn build_to(
        &mut self,
        producer: SystemKey,
        kind: crate::manufacturing::BuildableKind,
        count: u32,
        destination: Option<crate::manufacturing::Destination>,
    ) -> Option<Order> {
        let work =
            crate::build_selection::unit_costs(self.inputs.world, kind).map_or(0, |(w, _)| w);
        (count > 0 && self.afford(kind, count)).then_some(Order::Build {
            producer,
            kind,
            work,
            count,
            destination,
        })
    }
}

/// A class's AI node (`FUN_004739b0`, `FUN_004736c0`, `FUN_004734d0`,
/// `FUN_00472cc0`, `FUN_00473090`, `FUN_00473290`; `ai-production.md`,
/// "U1"): its flags and the figures its keys read.
#[derive(Debug, Clone, Copy, Default)]
pub struct ClassNode {
    pub flags: u32,
    pub maintenance: i64,
    pub cost: i64,
    /// `+0x30`, `+0x34`, `+0x38`, `+0x3c`, `+0x44`, `+0x48`, `+0x4c`,
    /// `+0x58`.
    pub n30: i64,
    pub n34: i64,
    pub n38: i64,
    pub n3c: i64,
    pub n44: i64,
    pub n48: i64,
    pub n4c: i64,
    pub n58: i64,
    /// The node's kind, which picks its key function.
    pub node: NodeKind,
}

/// Which key function a node uses.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NodeKind {
    #[default]
    Facility,
    Defence,
    CapitalShip,
    Fighter,
    Regiment,
    SpecialForce,
}

impl ClassNode {
    /// The node of `kind`'s class.
    #[must_use]
    pub fn of(world: &GameWorld, kind: crate::manufacturing::BuildableKind) -> Option<Self> {
        use crate::manufacturing::BuildableKind as K;
        let (cost, maintenance) = crate::build_selection::unit_costs(world, kind)?;
        let mut node = Self {
            maintenance: i64::from(maintenance),
            cost: i64::from(cost),
            ..Self::default()
        };
        match kind {
            K::CapitalShip(key) => node.capital_ship(world.capital_ship_classes.get(key)?),
            K::Fighter(key) => node.fighter(world.fighter_classes.get(key)?),
            K::Troop(id) => {
                let class = world.troop_classes.get(&id)?;
                node.node = NodeKind::Regiment;
                node.n30 = i64::from(class.attack_strength);
                node.n34 = i64::from(class.defense_strength);
                if node.n34 <= node.n30 {
                    node.flags |= 0x1000;
                }
                if node.n30 <= node.n34 {
                    node.flags |= 0x2000;
                }
            }
            K::SpecialForce(id) => {
                node.node = NodeKind::SpecialForce;
                // FUN_00402e40: the class bit by id.
                node.flags = match id.raw() & 0xff_ffff {
                    1 | 5 => 0x100_0000,
                    2 | 6 => 0x200_0000,
                    3 | 7 => 0x400_0000,
                    4 | 8 => 0x800_0000,
                    _ => 0,
                };
            }
            K::DefenseFacility(build) => {
                let class = world.buildable_classes.get(&build.class)?;
                node.node = NodeKind::Defence;
                node.n30 = i64::from(class.stats.attack_strength);
                node.n34 = i64::from(class.stats.shield_strength);
                node.flags = match build.class.family() {
                    0x22 | 0x23 => 0x8_0000,
                    0x24 => 0x2_0000,
                    0x25 => 0x4_0000,
                    _ => 0,
                };
            }
            K::ManufacturingFacility(build) | K::ProductionFacility(build) => {
                let class = world.buildable_classes.get(&build.class)?;
                node.node = NodeKind::Facility;
                node.n30 = i64::from(class.processing_rate);
                node.flags = match build.class.family() {
                    0x28 => 0x4000,
                    0x29 => 0x8000,
                    0x2a => 0x1_0000,
                    _ => 0,
                };
            }
        }
        Some(node)
    }

    /// The node's key (`FUN_00473930`, `FUN_00473640`, `FUN_00473490`,
    /// `FUN_00473250`, `FUN_00473060`); 1 maintenance and 4 refined cost
    /// for every node.
    #[must_use]
    pub fn key(&self, key: u32) -> i64 {
        match (self.node, key) {
            (_, 1) => self.maintenance,
            (_, 4) => self.cost,
            (NodeKind::CapitalShip, 0x200) => self.n30,
            (NodeKind::CapitalShip, 0x8_0000) => self.n58,
            (NodeKind::CapitalShip | NodeKind::Fighter, 0x400) => self.n34,
            (NodeKind::CapitalShip, 0x800) => self.n38,
            (NodeKind::CapitalShip, 0x80) => self.n44,
            (NodeKind::CapitalShip, 0x100) => self.n48,
            (NodeKind::CapitalShip, 2) | (NodeKind::Regiment, 2) => self.n4c,
            (NodeKind::CapitalShip, 0x1_0000) => self.n3c,
            (NodeKind::Fighter, 0x8000) => self.n48,
            (NodeKind::Regiment, 0x1000) => self.n30,
            (NodeKind::Regiment, 0x2000) => self.n34,
            (NodeKind::Defence, 0x1_0000) => self.n34,
            (NodeKind::Defence, 0x4_0000) => self.n30,
            (NodeKind::Facility, 0x4000) => self.n30,
            _ => 0,
        }
    }

    /// `FUN_004739b0`.
    fn capital_ship(&mut self, class: &crate::world::CapitalShipClass) {
        self.node = NodeKind::CapitalShip;
        let arcs = [
            (
                class.turbolaser_fore,
                class.ion_cannon_fore,
                class.laser_cannon_fore,
            ),
            (
                class.turbolaser_aft,
                class.ion_cannon_aft,
                class.laser_cannon_aft,
            ),
            (
                class.turbolaser_port,
                class.ion_cannon_port,
                class.laser_cannon_port,
            ),
            (
                class.turbolaser_starboard,
                class.ion_cannon_starboard,
                class.laser_cannon_starboard,
            ),
        ];
        let recharge = i64::from(class.weapon_recharge_rate);
        self.n38 = i64::from(class.bombardment_modifier != 0);
        self.n3c = i64::from(class.shield_recharge_rate) * 100;
        self.n44 = i64::from(class.fighter_capacity);
        self.n48 = i64::from(class.troop_capacity);
        self.n4c = i64::from(class.detection);
        // Class `+0x2c` in 0x18..=0x1b: in CAPSHPSD only the Death Star's
        // record carries family 0x18.
        if class.is_death_star() {
            self.flags |= 0x80;
        } else {
            self.flags |= 0x10_0000;
            // The best arc by 100·(T+I) + 100·L/6, the first on a tie.
            let mut best: Option<(i64, i64)> = None;
            let mut best_b: Option<(i64, i64)> = None;
            for &(t, i, l) in &arcs {
                let (t, i, l) = (i64::from(t), i64::from(i), i64::from(l));
                let a = 100 * (t + i) + 100 * l / 6;
                if a > 0 && best.is_none_or(|(score, _)| a > score) {
                    best = Some((a, t + i + l));
                }
                let b = 10 * l + 10 * t / 2;
                if b > 0 && best_b.is_none_or(|(score, _)| b > score) {
                    best_b = Some((b, l + t));
                }
            }
            if let Some((a, n)) = best.filter(|&(_, n)| n > 0) {
                self.n30 = recharge * a / n;
            }
            if let Some((b, n)) = best_b.filter(|&(_, n)| n > 0) {
                self.n34 = i64::from(class.maneuverability) * recharge * b / n;
            }
            let gravity = class.gravity_well_projector > 0;
            if gravity {
                self.flags |= 0x400;
            } else {
                if arcs.iter().any(|&(t, i, _)| t + i > 0) {
                    self.flags |= 0x1;
                }
                if class.troop_capacity == 0 && arcs.iter().any(|&(_, _, l)| l > 0) {
                    self.flags |= 0x2;
                }
            }
        }
        if class.troop_capacity > 0 {
            let before = self.flags;
            self.flags |= 0x200;
            if before & 0x403 == 0 && before & 0x10_0000 != 0 {
                self.flags |= 0x4;
            }
            if before & 0x1 != 0 {
                self.flags |= 0x8;
            }
        }
        if class.fighter_capacity > 0 {
            self.flags |= 0x100;
        }
        if class.bombardment_modifier > 0 {
            self.flags |= 0x800;
        }
        self.flags |= match class.hull {
            h if h < 1100 => 0x10,
            h if h < 2000 => 0x20,
            _ => 0x40,
        };
        if self.maintenance > 0 {
            self.n58 = self.n30 * 10 / self.maintenance;
        }
    }

    /// `FUN_004736c0`.
    fn fighter(&mut self, class: &crate::world::FighterClass) {
        self.node = NodeKind::Fighter;
        self.flags = 0x20_0000;
        let squadron = i64::from(class.squadron_size);
        let (ion, laser) = (
            i64::from(class.ion_cannon_fore),
            i64::from(class.laser_cannon_fore),
        );
        let numerator = 100 * ion + 100 * laser / 6;
        if numerator > 0 && ion + laser > 0 {
            self.n30 = numerator * squadron / (ion + laser);
        }
        if laser > 0 {
            let x = i64::from(class.maneuverability) * squadron * 10;
            self.n34 = x + x * i64::from(class.shield_strength) / 8;
        }
        self.n48 = i64::from(class.torpedoes) * squadron;
        self.n4c = squadron;
    }
}

// ---------------------------------------------------------------------------
// Shared facility steps
// ---------------------------------------------------------------------------

/// Mine and Refinery, the classes modules 3, 1 and 2 name
/// (`FUN_004d6e70`, `FUN_004db2f0`, `FUN_004e25c0`).
const MINE: u32 = 0x2c00_0001;
/// The Refinery class (`FUN_004db2f0` step 6, `0x2d000002`).
const REFINERY: u32 = 0x2d00_0002;

impl Round<'_> {
    /// Every candidate of a system query, best first (`FUN_004357b0`
    /// popped until empty).
    fn systems_ranked(
        &mut self,
        sector: crate::ids::SectorKey,
        filter: SystemFilter,
        keys: &[(u32, Dir)],
    ) -> Vec<SystemKey> {
        let Some(systems) = self
            .sectors
            .iter()
            .find(|s| s.key == sector)
            .map(|s| s.systems.clone())
        else {
            return Vec::new();
        };
        let mut left: Vec<SystemKey> = systems
            .into_iter()
            .filter(|s| self.records.get(s).is_some_and(|r| filter.admits(r)))
            .collect();
        let mut out = Vec::new();
        while !left.is_empty() {
            let lists: Vec<(Vec<(SystemKey, i64)>, Dir)> = keys
                .iter()
                .map(|&(slot, dir)| {
                    (
                        left.iter()
                            .map(|&s| (s, self.records[&s].slot(slot)))
                            .collect(),
                        dir,
                    )
                })
                .collect();
            let Some(best) = rank_pick(&lists, &mut self.rng) else {
                break;
            };
            left.retain(|&s| s != best);
            out.push(best);
        }
        out
    }

    /// The side's objects of `families` at `system`, in list order (the
    /// system's child lists the Scrap steps walk, `FUN_004db820`).
    fn objects(&self, system: SystemKey, families: &[u8]) -> Vec<crate::scrap::ScrapTarget> {
        use crate::scrap::ScrapTarget;
        let world = self.inputs.world;
        let side = self.inputs.side;
        let Some(sys) = world.systems.get(system) else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for &family in families {
            match family {
                0x22..=0x25 => out.extend(
                    sys.defense_facilities
                        .iter()
                        .filter(|k| {
                            world.defense_facilities.get(**k).is_some_and(|d| {
                                d.side == side && d.class_dat_id.family() == family
                            })
                        })
                        .map(|k| ScrapTarget::Defense(*k)),
                ),
                0x28..=0x2a => out.extend(
                    sys.manufacturing_facilities
                        .iter()
                        .filter(|k| {
                            world.manufacturing_facilities.get(**k).is_some_and(|y| {
                                y.side == side && y.class_dat_id.family() == family
                            })
                        })
                        .map(|k| ScrapTarget::Manufacturing(*k)),
                ),
                0x2c | 0x2d => out.extend(
                    sys.production_facilities
                        .iter()
                        .filter(|k| {
                            world
                                .production_facilities
                                .get(**k)
                                .is_some_and(|p| p.side == side && p.is_mine == (family == 0x2c))
                        })
                        .map(|k| ScrapTarget::Production(*k)),
                ),
                _ => {}
            }
        }
        out
    }

    /// The make-room Scrap at a new site (`FUN_004d8fe0`, `FUN_004d7cf0`,
    /// `FUN_004db820`, `FUN_004dc650`, `FUN_004e31b0`, `FUN_004e3a40`;
    /// "F3.6"): its shipyards and training facilities without their role,
    /// then `third` (Death Star shields, or construction yards without a
    /// construction role); with nothing to scrap and no free energy, a
    /// refinery or a mine.
    fn make_room(&self, site: SystemKey, third: MakeRoom) -> Option<Order> {
        let r = self.record(site)?;
        let mut targets = Vec::new();
        if r.a & role::SHIPYARD == 0 {
            targets.extend(self.objects(site, &[0x28]));
        }
        if r.a & role::TRAINING == 0 {
            targets.extend(self.objects(site, &[0x29]));
        }
        match third {
            MakeRoom::DeathStarShields => targets.extend(self.objects(site, &[0x25])),
            MakeRoom::ConstructionYards if r.a & role::CONSTRUCTION == 0 => {
                targets.extend(self.objects(site, &[0x2a]));
            }
            MakeRoom::ConstructionYards | MakeRoom::None => {}
        }
        if targets.is_empty() && r.free_energy == 0 {
            let (mines, refineries) = (
                i64::from(self.census.mines),
                i64::from(self.census.refineries),
            );
            if mines < refineries || r.mines == 0 {
                targets.extend(self.objects(site, &[0x2d]).into_iter().take(1));
            }
            if targets.is_empty() && (refineries <= mines || r.refineries == 0) {
                targets.extend(self.objects(site, &[0x2c]).into_iter().take(1));
            }
        }
        (!targets.is_empty()).then_some(Order::Scrap(targets))
    }

    /// A mine or refinery site (`FUN_004dc150`, `FUN_004dc3f0`,
    /// `FUN_004e2c70`, `FUN_004e2f10`). hyp: module 3's `FUN_004d98a0` and
    /// `FUN_004d99d0` choose as module 1's do.
    fn mine_site(&mut self, mine: bool, producer: SystemKey) -> Option<SystemKey> {
        let open = SystemFilter {
            require: if mine {
                bit::MINE_SITE
            } else {
                bit::REFINERY_SITE
            },
            exclude: bit::BLOCKED,
            own: true,
        };
        let found = if mine {
            self.sector(
                0x100,
                &[(7, Dir::Largest), (10, Dir::Largest), (9, Dir::Largest)],
            )
            .and_then(|s| {
                self.system_in(
                    s,
                    open,
                    &[(0xe, Dir::Largest), (0, Dir::Largest), (0xf, Dir::Largest)],
                )
            })
        } else {
            self.sector(0x200, &[(7, Dir::Largest), (9, Dir::Largest)])
                .and_then(|s| {
                    let filter = SystemFilter {
                        exclude: role::ANY | bit::BLOCKED,
                        ..open
                    };
                    self.system_in(s, filter, &[(0xf, Dir::Largest), (0, Dir::Largest)])
                })
        };
        found.or_else(|| {
            let (flags, require) = if mine {
                (0x800, bit::FREE_RAW)
            } else {
                (0x400, bit::FREE_ENERGY)
            };
            let sector = self.nearest_sector(producer, flags)?;
            self.nearest_system_in(
                sector,
                producer,
                SystemFilter {
                    require,
                    exclude: bit::BLOCKED,
                    own: true,
                },
            )
        })
    }

    /// The mine or refinery the census asks for: a refinery while the side
    /// has fewer refineries than mines (modules 3 and 1), or no more
    /// (module 2, `refinery_on_tie`).
    fn mine_or_refinery(
        &self,
        refinery_on_tie: bool,
    ) -> (bool, crate::manufacturing::BuildableKind) {
        let (mines, refineries) = (self.census.mines, self.census.refineries);
        let refinery = if refinery_on_tie {
            refineries <= mines
        } else {
            refineries < mines
        };
        let mine = !refinery;
        let class = if mine { MINE } else { REFINERY };
        (
            mine,
            crate::manufacturing::BuildableKind::ProductionFacility(
                crate::manufacturing::FacilityBuild {
                    class: crate::ids::DatId::new(class),
                    is_alliance: self.inputs.side == Faction::Alliance,
                },
            ),
        )
    }

    /// The family's yard class with the smallest processing rate
    /// (`FUN_0041b230(c, out, 0x2a, mask, 0x4000, 2)`).
    fn yard_class(&mut self, mask: u32) -> Option<(crate::manufacturing::BuildableKind, i64)> {
        self.class(
            crate::manufacturing::ProductionArea::ConstructionYard,
            mask,
            0x4000,
            Dir::Smallest,
        )
    }

    /// The construction-yard producer nearest a site (`FUN_004cafa0`,
    /// `FUN_004d8db0`, `FUN_004e3810`): a ready module-1 yard system in the
    /// nearest sector holding one, and its construction yards less one when
    /// it has more than one (module 1 keeps them all).
    fn producer_base(&mut self, site: SystemKey) -> Option<(SystemKey, i64)> {
        let sector = self.nearest_sector(site, 0x1000)?;
        let base = self.nearest_system_in(
            sector,
            site,
            SystemFilter {
                require: role::YARDS | bit::YARD_READY,
                exclude: bit::BLOCKED,
                own: true,
            },
        )?;
        let yards = self.record(base)?.yards[2];
        Some((base, if yards > 1 { yards - 1 } else { yards }))
    }
}

/// What a module builds its producers from: the yard family, the role it
/// claims, and the sector flag and key slots its site queries read
/// (`FUN_004cacc0`, `FUN_004d8ad0`, and their goal setters).
#[derive(Debug, Clone, Copy)]
struct YardKind {
    family: YardFamily,
    /// The role the site takes.
    role: u32,
    /// The roles that keep a system out of the site query.
    taken: u32,
    /// `rec+0x24`: an unclaimed system with free slots.
    sector_flag: u32,
    /// `rec+0x70 + 4k`: yards of the family at unclaimed systems.
    sector_slot: u32,
    /// `sys+0x48 + 4k`: yards of the family.
    system_slot: u32,
    /// The facility node flag of the family's classes.
    class_mask: u32,
}

impl YardKind {
    /// The construction-yard site and class (`FUN_004d8ad0`, `FUN_004d9520`).
    const fn construction(role: u32) -> Self {
        Self {
            family: YardFamily::ConstructionYard,
            role,
            taken: role::CONSTRUCTION,
            sector_flag: 0x80,
            sector_slot: 0x15,
            system_slot: 6,
            class_mask: 0x1_0000,
        }
    }

    /// The shipyard site and class (`FUN_004ce810`).
    const SHIPYARD: Self = Self {
        family: YardFamily::Shipyard,
        role: role::SHIPYARD,
        taken: role::SHIPYARD,
        sector_flag: 0x20,
        sector_slot: 0x11,
        system_slot: 4,
        class_mask: 0x4000,
    };

    /// The training site and class (`FUN_004d1970`, `FUN_004e15b0`).
    const TRAINING: Self = Self {
        family: YardFamily::TrainingFacility,
        role: role::TRAINING,
        taken: role::TRAINING,
        sector_flag: 0x40,
        sector_slot: 0x13,
        system_slot: 5,
        class_mask: 0x8000,
    };
}

/// The third family a make-room Scrap clears.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MakeRoom {
    DeathStarShields,
    ConstructionYards,
    None,
}

impl ProductionState {
    /// A construction-yard module's new site (`FUN_004d8ad0`,
    /// `FUN_004db510`, `FUN_004e3530`): the listed system with free slots
    /// and the fewest construction yards, else the best unclaimed one,
    /// which joins the list with the module's role. With its free slots.
    fn yard_site(
        &mut self,
        module: Module,
        kind: YardKind,
        round: &mut Round<'_>,
    ) -> Option<(SystemKey, i64)> {
        let listed = self.module(module).list.clone();
        let family = kind.family.index();
        let mut best: Option<(SystemKey, i64, i64)> = None;
        for &s in &listed {
            if let Some(r) = round.record(s) {
                if r.own
                    && r.a & bit::BLOCKED == 0
                    && r.free_slots > 0
                    && r.yards[family] < best.map_or(1000, |b| b.2)
                {
                    best = Some((s, r.free_slots, r.yards[family]));
                }
            }
        }
        if let Some((s, slots, _)) = best {
            return Some((s, slots));
        }
        let mut sector_keys = vec![(kind.sector_slot, Dir::Largest)];
        let mut system_keys = vec![(kind.system_slot, Dir::Largest)];
        if !listed.is_empty() {
            sector_keys.push((4, Dir::Smallest));
            system_keys.push((0x33, Dir::Largest));
        }
        let sector = round.sector(kind.sector_flag, &sector_keys)?;
        let site = round.system_in(
            sector,
            SystemFilter {
                require: 0,
                exclude: kind.taken | bit::BLOCKED,
                own: true,
            },
            &system_keys,
        )?;
        self.claim(module, site, kind.role, round);
        let slots = round.record(site).map_or(0, |r| r.free_slots);
        Some((site, slots))
    }

    /// The goal setter (`FUN_004da0d0`, `FUN_004dcc40`, `FUN_004e4540`;
    /// "F6"): claim an unclaimed yard system the facility budget covers,
    /// else ask for the token when the module's list has no yard.
    fn goal(&mut self, module: Module, kind: YardKind, budget: i64, round: &mut Round<'_>) {
        let family = kind.family.index();
        if budget >= 1 {
            if let Some(sector) =
                round.sector(kind.sector_flag, &[(kind.sector_slot, Dir::Largest)])
            {
                let unclaimed = round
                    .sectors
                    .iter()
                    .find(|s| s.key == sector)
                    .map_or(0, |s| s.slot(kind.sector_slot));
                if unclaimed > 0 {
                    let ranked = round.systems_ranked(
                        sector,
                        SystemFilter {
                            require: 0,
                            exclude: kind.taken | bit::BLOCKED,
                            own: true,
                        },
                        &[(kind.system_slot, Dir::Largest)],
                    );
                    let found = ranked.into_iter().find(|s| {
                        round
                            .record(*s)
                            .is_some_and(|r| r.yards[family] > 0 && r.yard_spend[family] <= budget)
                    });
                    if let Some(site) = found {
                        self.claim(module, site, kind.role, round);
                        return;
                    }
                }
            }
        }
        let list = self.module(module).list.clone();
        let head_has_yards = list
            .first()
            .and_then(|s| round.record(*s))
            .is_some_and(|r| r.yards[family] > 0);
        if list.len() < 2 && !head_has_yards {
            let cost = round.yard_class(kind.class_mask).map_or(0, |(_, m)| m);
            if cost <= budget {
                self.token.pending |= module.goal_bit();
            }
        }
    }

    /// Retire a construction yard (`FUN_004d95c0`, `FUN_004e4020`): from a listed system with two or more, or with one
    /// when the list holds others.
    fn retire_yard(
        &mut self,
        module: Module,
        bit_role: u32,
        round: &mut Round<'_>,
    ) -> Option<Order> {
        for s in self.module(module).list.clone() {
            let Some(r) = round.record(s).cloned() else {
                continue;
            };
            if r.yards[2] < 1 {
                self.release(module, s, bit_role, round);
                continue;
            }
            if r.a & bit::ANY_YARD == 0 {
                continue;
            }
            if r.yards[2] == 1 {
                if self.module(module).list.len() <= 1 {
                    continue;
                }
                self.release(module, s, bit_role, round);
            }
            let yard = round.objects(s, &[0x2a]).into_iter().next()?;
            return Some(Order::Scrap(vec![yard]));
        }
        None
    }

    /// A construction-yard module's facility pass after its site and
    /// producer: make room, then Build as many yards as the budget, the
    /// producer and the site allow (`FUN_004d9520`, `FUN_004dbd60`,
    /// `FUN_004e3f80`).
    fn build_yards(
        &mut self,
        module: Module,
        kind: YardKind,
        site: (SystemKey, i64),
        producer: (SystemKey, i64),
        round: &mut Round<'_>,
    ) -> Vec<Order> {
        let mut orders: Vec<Order> = round
            .make_room(site.0, MakeRoom::DeathStarShields)
            .into_iter()
            .collect();
        let budget = self.facility_budget(module, round);
        if budget < 0 {
            // Module 1 never retires: `FUN_004dbd60` zeroes `m+0x4c` (line
            // 14), so its negative-budget result 5 (lines 26-29) is
            // overwritten with 6 (lines 56-60) and `FUN_004dbe70` is not
            // reached.
            if kind.family == YardFamily::ConstructionYard && module != Module::Yards {
                orders.extend(self.retire_yard(module, kind.role, round));
            }
            return orders;
        }
        if budget == 0 {
            return orders;
        }
        let Some((class, maintenance)) = round.yard_class(kind.class_mask) else {
            return orders;
        };
        let mut count = if maintenance > 0 {
            budget / maintenance
        } else {
            0
        };
        count = count.min(producer.1);
        let free = round.record(site.0).map_or(0, |r| r.free_energy);
        count = count.min(free.min(site.1));
        if count > 0 {
            let count = u32::try_from(count).unwrap_or(0);
            orders.extend(round.build(producer.0, class, count, Some(site.0)));
        }
        orders
    }

    /// A module's facility budget (`FUN_004cc280`, `FUN_004cec80`,
    /// `FUN_004d6700`, `FUN_004d1df0`, `FUN_004e1a20`, `FUN_004da530`,
    /// `FUN_004dd050`, `FUN_004e4950`): its share less its listed systems'
    /// yard maintenance. Module 11 subtracts the construction yards'
    /// (`sys+0xd8`), as read.
    fn facility_budget(&mut self, module: Module, round: &Round<'_>) -> i64 {
        let (pct, family) = match module {
            Module::CapitalShips => (10, YardFamily::ConstructionYard),
            Module::FleetFighters | Module::SystemFighters => (20, YardFamily::Shipyard),
            Module::SpecialForces => (30, YardFamily::TrainingFacility),
            Module::Regiments => (20, YardFamily::TrainingFacility),
            Module::Defences => (10, YardFamily::ConstructionYard),
            Module::Yards | Module::DeficitRelief => (100, YardFamily::ConstructionYard),
        };
        let list = self.module(module).list.clone();
        let spent = round.census.yard_spend(family, &list);
        round.census.budget(module, pct, spent)
    }

    /// A module's facility phase `0x3ec`: its site, then, holding the token
    /// and with a construction-yard producer near the site, Build its
    /// yards; otherwise the goal setter.
    fn facility_pass(
        &mut self,
        module: Module,
        kind: YardKind,
        round: &mut Round<'_>,
    ) -> Vec<Order> {
        let Some(site) = self.yard_site(module, kind, round) else {
            return Vec::new();
        };
        self.token.pending &= !module.goal_bit();
        if self.token.held_by(module) {
            if let Some(producer) = round.producer_base(site.0) {
                self.claim(module, site.0, kind.role, round);
                self.token.pass();
                return self.build_yards(module, kind, site, producer, round);
            }
        }
        let budget = self.facility_budget(module, round);
        self.goal(module, kind, budget, round);
        Vec::new()
    }

    /// The first listed system that holds `bit_role` with a ready
    /// construction yard (`FUN_004d7ac0`, `FUN_004daca0`, `FUN_004e21d0`);
    /// listed systems lost or without the role leave the list.
    fn yard_producer(
        &mut self,
        module: Module,
        bit_role: u32,
        round: &mut Round<'_>,
    ) -> Option<(SystemKey, i64)> {
        for s in self.module(module).list.clone() {
            let Some(r) = round.record(s).cloned() else {
                self.module(module).list.retain(|&x| x != s);
                continue;
            };
            if r.own && r.a & bit_role == 0 {
                self.release(module, s, bit_role, round);
                continue;
            }
            if r.own && r.yards[2] > 0 && r.has(bit::YARD_READY) && r.a & bit::BLOCKED == 0 {
                let cap = if module == Module::Yards || r.yards[2] <= 1 {
                    r.yards[2]
                } else {
                    r.yards[2] - 1
                };
                return Some((s, cap));
            }
        }
        None
    }

    // -----------------------------------------------------------------------
    // Module 3: defences (`FUN_004d6d30`)
    // -----------------------------------------------------------------------

    /// Phases `0x3ed` (surplus), `0x3ee` (a defence, else a mine or
    /// refinery) and `0x3ec` (construction yards), until one ends the pass
    /// (`ai-production.md`, "F3").
    fn defences(&mut self, round: &mut Round<'_>) -> Vec<Order> {
        const M: Module = Module::Defences;
        loop {
            match self.module(M).phase {
                0x3ee => {
                    self.module(M).phase = 0x3ec;
                    if let Some(orders) = self.defence_build(round) {
                        return orders;
                    }
                }
                0x3ec => {
                    self.module(M).phase = 0x3ed;
                    return self.defence_yards(round);
                }
                _ => {
                    self.module(M).phase = 0x3ee;
                    // FUN_004d71e0: the own system with the largest surplus.
                    let target = round
                        .sector(0x1_0000, &[(0x1d, Dir::Largest)])
                        .and_then(|s| {
                            round.system_in(
                                s,
                                SystemFilter {
                                    own: true,
                                    ..SystemFilter::default()
                                },
                                &[(0x25, Dir::Largest)],
                            )
                        });
                    if let Some(r) = target.and_then(|t| round.record(t).cloned()) {
                        if r.defence_balance > 0 {
                            // FUN_004d7330: the surplus shields, then
                            // batteries.
                            let shields = usize::try_from(r.shields - r.shield_target).unwrap_or(0);
                            let batteries =
                                usize::try_from(r.batteries - r.battery_target).unwrap_or(0);
                            let mut targets: Vec<_> = round
                                .objects(r.key, &[0x24])
                                .into_iter()
                                .take(shields)
                                .collect();
                            targets.extend(
                                round
                                    .objects(r.key, &[0x22, 0x23])
                                    .into_iter()
                                    .take(batteries),
                            );
                            if !targets.is_empty() {
                                return vec![Order::Scrap(targets)];
                            }
                            return Vec::new();
                        }
                    }
                }
            }
        }
    }

    /// Phase `0x3ee` (`FUN_004d6e70`); `None` when it found no producer
    /// and the module goes on to `0x3ec`.
    fn defence_build(&mut self, round: &mut Round<'_>) -> Option<Vec<Order>> {
        const M: Module = Module::Defences;
        let producer = self.yard_producer(M, role::DEFENCES, round)?;
        let (target, need) = self.defence_target(producer.0, round);
        let mut orders = Vec::new();
        if let (Some(target), true) = (target, need) {
            orders.extend(round.make_room(target, MakeRoom::ConstructionYards));
            let budget = round.census.budget(M, 90, round.census.system_defences);
            if budget < 0 {
                orders.extend(self.defence_overdraft(round));
                return Some(orders);
            }
            if budget > 0 {
                if let Some(r) = round.record(target).cloned() {
                    if r.free_energy > 0 {
                        let (mask, key) = if r.a & bit::DS_SHIELD_WANTED != 0 {
                            (0x4_0000, 0x1_0000)
                        } else if r.a & bit::SHIELD_SHORT == 0 {
                            (0x8_0000, 0x4_0000)
                        } else {
                            (0x2_0000, 0x1_0000)
                        };
                        let class = round.class(
                            crate::manufacturing::ProductionArea::ConstructionYard,
                            mask,
                            key,
                            Dir::Largest,
                        );
                        if let Some((kind, maintenance)) =
                            class.filter(|&(_, m)| m != 0 && m <= budget)
                        {
                            let _ = maintenance;
                            orders.extend(round.build(producer.0, kind, 1, Some(target)));
                            return Some(orders);
                        }
                    }
                }
            }
        }
        // Step 9: a refinery while refineries trail mines, else a mine.
        // hyp: the site Scrap FUN_004d9b00 (every yard at the site) is left
        // out; its reach is unsettled ("F8").
        let (mine, kind) = round.mine_or_refinery(false);
        let Some(site) = round.mine_site(mine, producer.0) else {
            return Some(orders);
        };
        orders.extend(round.build(producer.0, kind, 1, Some(site)));
        Some(orders)
    }

    /// `FUN_004d76f0` ("F3.4"): Coruscant first, then the role system with
    /// the largest deficit, then any deficit system near the producer.
    fn defence_target(
        &mut self,
        producer: SystemKey,
        round: &mut Round<'_>,
    ) -> (Option<SystemKey>, bool) {
        let coruscant = round.records.values().find(|r| r.coruscant).cloned();
        let shields_wanted = self.targets[2];
        if let Some(c) = coruscant
            .as_ref()
            .filter(|c| c.own && c.a & bit::BLOCKADE == 0)
        {
            let need = if c.a & bit::DS_SHIELD_WANTED != 0 {
                true
            } else {
                c.shields < shields_wanted
            };
            if need {
                return (Some(c.key), true);
            }
        }
        let filter = SystemFilter {
            require: bit::DEFENCE_DEFICIT,
            exclude: bit::UNASSIGNED | bit::BLOCKED,
            own: true,
        };
        let mut target = None;
        if let Some(sector) = round.sector(0x4000, &[(0x1c, Dir::Largest)]) {
            target = round.system_in(sector, filter, &[(0x25, Dir::Smallest)]);
        }
        if target.is_none() {
            if let Some(c) = coruscant.as_ref() {
                if c.own && c.defence_balance < 0 && c.a & bit::BLOCKADE == 0 {
                    return (Some(c.key), true);
                }
            }
            if let Some(sector) = round.nearest_sector(producer, 0x8000) {
                target = round.system_in(
                    sector,
                    SystemFilter {
                        require: bit::DEFENCE_DEFICIT,
                        exclude: bit::BLOCKED,
                        own: true,
                    },
                    &[(0x25, Dir::Smallest)],
                );
            }
        }
        match target.and_then(|t| round.record(t)) {
            Some(r) => (Some(r.key), r.defence_balance < 0),
            None => {
                if round
                    .census
                    .budget(Module::Defences, 90, round.census.system_defences)
                    > 0
                    && self.targets[4] < 2
                {
                    self.targets[4] += 1;
                    // FUN_00418240 -> FUN_004311d0 marks every system record
                    // (FUN_004d4370.c:121-123).
                    round.dirty = true;
                }
                (None, false)
            }
        }
    }

    /// `FUN_004d8520` ("F3.5"): scrap one defence where the side has a
    /// surplus, else where it has shields and batteries both.
    fn defence_overdraft(&mut self, round: &mut Round<'_>) -> Option<Order> {
        let own = SystemFilter {
            own: true,
            ..SystemFilter::default()
        };
        let system = round
            .sector(0x1_0000, &[(0x1d, Dir::Largest)])
            .and_then(|s| round.system_in(s, own, &[(0x25, Dir::Largest)]))
            .or_else(|| {
                round.sector(0x2_0000, &[(3, Dir::Largest)]).and_then(|s| {
                    round.system_in(s, own, &[(7, Dir::Largest), (0xb, Dir::Largest)])
                })
            })?;
        for family in [0x22, 0x23, 0x24] {
            if let Some(target) = round.objects(system, &[family]).into_iter().next() {
                return Some(Order::Scrap(vec![target]));
            }
        }
        None
    }

    /// Phase `0x3ec` of module 3 (`FUN_004d7060`, "F3.8").
    fn defence_yards(&mut self, round: &mut Round<'_>) -> Vec<Order> {
        self.facility_pass(
            Module::Defences,
            YardKind::construction(role::DEFENCES),
            round,
        )
    }

    // -----------------------------------------------------------------------
    // Module 1: construction yards (`FUN_004dabe0`)
    // -----------------------------------------------------------------------

    /// Phase 0 finds the module's yard system; `0x3ec` builds yards there
    /// or a mine or refinery ("F4").
    fn yards(&mut self, round: &mut Round<'_>) -> Vec<Order> {
        const M: Module = Module::Yards;
        let producer = self.yard_producer(M, role::YARDS, round).or_else(|| {
            if !self.module(M).list.is_empty() {
                return None;
            }
            // FUN_004daf10: an unclaimed system with a ready yard.
            let sector = round.sector(
                0x2000,
                &[(4, Dir::Smallest), (0x15, Dir::Largest), (8, Dir::Largest)],
            );
            let site = sector.and_then(|s| {
                round.system_in(
                    s,
                    SystemFilter {
                        require: bit::YARD_READY,
                        exclude: role::CONSTRUCTION,
                        own: true,
                    },
                    &[(6, Dir::Largest), (0, Dir::Largest), (0x33, Dir::Largest)],
                )
            });
            let Some(site) = site else {
                self.no_yard_site = true;
                return None;
            };
            if let Some(roles) = self.roles.get_mut(&site) {
                *roles &= !role::CONSTRUCTION;
            }
            self.claim(M, site, role::YARDS, round);
            round
                .record(site)
                .filter(|r| r.has(bit::YARD_READY))
                .map(|r| (site, r.yards[2]))
        });
        let kind = YardKind::construction(role::YARDS);
        let Some(producer) = producer else {
            if !self.token.held_by(M) || self.no_yard_site {
                let budget = self.facility_budget(M, round);
                self.goal(M, kind, budget, round);
            }
            return Vec::new();
        };
        // Phase 0x3ec (FUN_004db2f0).
        self.token.pending &= !M.goal_bit();
        if !self.token.held_by(M) {
            let budget = self.facility_budget(M, round);
            self.goal(M, kind, budget, round);
            return Vec::new();
        }
        self.token.pass();
        let Some(site) = self.yard_site(M, kind, round) else {
            return Vec::new();
        };
        let orders = self.build_yards(M, kind, site, producer, round);
        if orders.iter().any(|o| matches!(o, Order::Build { .. })) {
            return orders;
        }
        // Step 6: no yard to build; a refinery or a mine.
        let (mine, kind) = round.mine_or_refinery(false);
        let Some(mine_site) = round.mine_site(mine, producer.0) else {
            return orders;
        };
        let mut orders = orders;
        if round
            .record(mine_site)
            .is_some_and(|r| r.a & role::ANY == 0)
        {
            orders.extend(round.make_room(mine_site, MakeRoom::None));
        }
        orders.extend(round.build(producer.0, kind, 1, Some(mine_site)));
        orders
    }

    // -----------------------------------------------------------------------
    // Module 2: deficit relief (`FUN_004e2120`)
    // -----------------------------------------------------------------------

    /// Phase 0 picks a mine (`0x3ea`) or refinery (`0x3eb`) pass when the
    /// module has a yard system, else the yard pass `0x3ec` (`FUN_004e2120`;
    /// "F5").
    fn relief(&mut self, round: &mut Round<'_>) -> Vec<Order> {
        const M: Module = Module::DeficitRelief;
        let mut orders = Vec::new();
        // Step 1-2: while the load exceeds the capacity, scrap a yard at
        // an unclaimed system.
        if round.census.capacity < round.census.load {
            let site = round
                .sector(
                    0xe0,
                    &[
                        (0x15, Dir::Largest),
                        (0x11, Dir::Largest),
                        (0x13, Dir::Largest),
                    ],
                )
                .and_then(|s| {
                    round.system_in(
                        s,
                        SystemFilter {
                            require: 0,
                            exclude: role::ANY,
                            own: true,
                        },
                        &[(6, Dir::Largest), (4, Dir::Largest), (5, Dir::Largest)],
                    )
                });
            if let Some(site) = site {
                if let Some(t) = round.objects(site, &[0x28, 0x29, 0x2a]).into_iter().next() {
                    orders.push(Order::Scrap(vec![t]));
                }
            }
        }
        if let Some(producer) = self.yard_producer(M, role::RELIEF, round) {
            let (mine, kind) = round.mine_or_refinery(true);
            let slot = usize::from(!mine);
            let kept = self.module(M).sites[slot].filter(|s| {
                round.record(*s).is_some_and(|r| {
                    r.a & bit::BLOCKED == 0
                        && r.has(if mine {
                            bit::MINE_SITE
                        } else {
                            bit::REFINERY_SITE
                        })
                })
            });
            let site = kept.or_else(|| round.mine_site(mine, producer.0));
            self.module(M).sites[slot] = site;
            let Some(site) = site else {
                return orders;
            };
            orders.extend(round.make_room(site, MakeRoom::ConstructionYards));
            orders.extend(round.build(producer.0, kind, 1, Some(site)));
            return orders;
        }
        orders.extend(self.facility_pass(M, YardKind::construction(role::RELIEF), round));
        orders
    }
}

// ---------------------------------------------------------------------------
// Unit modules 11, 10, 7, 5 and 6
// ---------------------------------------------------------------------------

/// The producer caches (`c+0x14`..`+0x24`).
#[derive(Debug, Clone, Copy)]
enum Cache {
    Regiments = 0,
    SpecialForces = 1,
    SystemFighters = 2,
    FleetFighters = 3,
    CapitalShips = 4,
}

/// Module 6's budget (`FUN_004e19c0`).
fn regiment_budget(round: &Round<'_>) -> i64 {
    round.census.budget(
        Module::Regiments,
        80,
        round.census.fleet_regiments + round.census.system_regiments,
    )
}

/// The count cut every unit module shares (`FUN_004cd230`, `FUN_004d46e0`,
/// `FUN_004d0590`, `FUN_004df9a0`): a count whose cost is not exactly the
/// budget and reaches it becomes budget / cost.
fn cut_to_budget(count: i64, maintenance: i64, budget: i64) -> i64 {
    let total = maintenance * count;
    if total != budget && budget <= total && maintenance > 0 {
        budget / maintenance
    } else {
        count
    }
}

impl ProductionState {
    /// A unit module's producer (`FUN_004ca300`, `FUN_004cccc0`,
    /// `FUN_004d3ff0`, `FUN_004d01a0`, `FUN_004df160`): the first listed
    /// system the side holds, with the module's role and a ready yard of the
    /// family, else the first cache in `fallbacks` that passes the same
    /// test. With its yards of the family.
    fn unit_producer(
        &mut self,
        module: Module,
        kind: YardKind,
        own_cache: Cache,
        fallbacks: &[Cache],
        round: &mut Round<'_>,
    ) -> Option<(SystemKey, i64)> {
        let family = kind.family.index();
        let ready = match kind.family {
            YardFamily::Shipyard => bit::SHIPYARD_READY,
            YardFamily::TrainingFacility => bit::TRAINING_READY,
            YardFamily::ConstructionYard => bit::YARD_READY,
        };
        let usable = |r: &SystemRecord| {
            r.own
                && r.a & kind.role != 0
                && r.yards[family] > 0
                && r.a & ready != 0
                && r.a & bit::BLOCKED == 0
        };
        self.caches[own_cache as usize] = None;
        let mut found = None;
        for s in self.module(module).list.clone() {
            let Some(r) = round.record(s).cloned() else {
                self.module(module).list.retain(|&x| x != s);
                continue;
            };
            if !r.own || r.a & kind.role == 0 {
                self.release(module, s, kind.role, round);
                continue;
            }
            if usable(&r) {
                found = Some(s);
                break;
            }
        }
        let found = found.or_else(|| {
            fallbacks
                .iter()
                .filter_map(|c| self.caches[*c as usize])
                .find(|s| round.record(*s).is_some_and(usable))
        })?;
        self.caches[own_cache as usize] = Some(found);
        Some((found, round.record(found)?.yards[family]))
    }

    // -----------------------------------------------------------------------
    // Module 11: capital ships (`FUN_004c8b30`)
    // -----------------------------------------------------------------------

    /// Phases `0x3f9` (one ship), `0x3ec` (shipyards) and `0x3f8` (the
    /// repair move) ("U5").
    ///
    /// hyp: phase `0x3f8`'s steps 3 and 4 (`FUN_004c94c0`, `FUN_004c9cb0`)
    /// are not read and emit nothing; after step 2's order (`FUN_004c8e90`)
    /// the next turn runs them.
    fn capital_ships(&mut self, round: &mut Round<'_>) -> Vec<Order> {
        const M: Module = Module::CapitalShips;
        if self.module(M).phase == 0x3ec {
            self.module(M).phase = 0x3f8;
            return self.facility_pass(M, YardKind::SHIPYARD, round);
        }
        if self.module(M).phase == 0x3f8 {
            // FUN_004c8ba0: step 2's order sets done and raises m+0x34
            // (:10-15), so the phase resumes at step 3 next turn.
            if self.module(M).step == 0 {
                if let Some(order) = self.repair_move(round) {
                    self.module(M).step = 3;
                    return vec![order];
                }
            }
            // Steps 3 and 4 (:29-34, :23-27), then phase 0x3f9.
            self.module(M).step = 0;
            self.module(M).phase = 0x3f9;
            return Vec::new();
        }
        self.module(M).phase = 0x3ec;
        let budget = round.census.budget(M, 90, round.census.capital_ships);
        let Some(producer) = self.unit_producer(
            M,
            YardKind::SHIPYARD,
            Cache::CapitalShips,
            &[Cache::SystemFighters],
            round,
        ) else {
            if budget < 0 {
                return self.scrap_ship(round).into_iter().collect();
            }
            return self.capital_ships(round);
        };
        let (mask, key, blocker) = self.ship_request(round);
        self.module(M).blockers &= 0x80ff_ffff;
        if budget < 0 {
            return self.scrap_ship(round).into_iter().collect();
        }
        if budget > 0 {
            let ranked = round.classes(
                crate::manufacturing::ProductionArea::Shipyard,
                mask,
                key,
                Dir::Largest,
            );
            if let Some((kind, _)) = ranked.into_iter().find(|&(_, m)| m <= budget) {
                return round.build(producer.0, kind, 1, None).into_iter().collect();
            }
            self.module(M).blockers |= blocker;
        }
        Vec::new()
    }

    /// `FUN_004c8e90` (phase `0x3f8`, step 2): the largest free own fleet
    /// holding a damaged ship that is not en route, from a system the enemy
    /// holds, else from one without the shipyard role, sends its mobile
    /// damaged ships (`0x1000` set, `0x800000` and `0x800` clear) to the
    /// nearest own unblockaded shipyard-role system, else to the nearest
    /// own unblockaded system. hyp: a Move to the fleet's own system is
    /// not issued.
    fn repair_move(&mut self, round: &mut Round<'_>) -> Option<Order> {
        use crate::ai_records::{fleet, ship};
        // FUN_0041a4c0(c, 0x1005, 0x802, 0, largest): own, unadopted,
        // holding a damaged ship, neither en route nor holding a ship en route;
        // ranked by ship count (`fleet+0x50`), largest first.
        let mut pool: Vec<(crate::ids::FleetKey, i64)> = round
            .inputs
            .world
            .fleets
            .keys()
            .filter_map(|key| round.fleets.get(&key))
            .filter(|f| {
                f.flags & (fleet::OWN | fleet::UNADOPTED | fleet::DAMAGED)
                    == (fleet::OWN | fleet::UNADOPTED | fleet::DAMAGED)
                    && f.flags & (fleet::EN_ROUTE | fleet::SHIP_EN_ROUTE) == 0
            })
            .map(|f| (f.key, i64::try_from(f.ships.len()).unwrap_or(i64::MAX)))
            .collect();
        let mut ranked = Vec::new();
        while !pool.is_empty() {
            let Some(best) = rank_pick(&[(pool.clone(), Dir::Largest)], &mut round.rng) else {
                break;
            };
            pool.retain(|&(key, _)| key != best);
            ranked.push(best);
        }
        let at = |round: &Round<'_>, key: crate::ids::FleetKey| {
            round
                .fleets
                .get(&key)
                .and_then(|f| round.record(f.location))
                .cloned()
        };
        let chosen = ranked
            .iter()
            .copied()
            .find(|&k| at(round, k).is_some_and(|r| r.enemy))
            .or_else(|| {
                ranked
                    .iter()
                    .copied()
                    .find(|&k| at(round, k).is_some_and(|r| r.a & role::SHIPYARD == 0))
            })?;
        let (location, tags) = {
            let f = &round.fleets[&chosen];
            (f.location, f.ships.clone())
        };
        let team: Vec<u32> = tags
            .into_iter()
            .filter(|t| {
                round.ships.get(t).is_some_and(|s| {
                    s.flags & ship::DAMAGED != 0
                        && s.flags & (ship::DRIVES_OUT | ship::EN_ROUTE) == 0
                })
            })
            .collect();
        if team.is_empty() {
            return None;
        }
        let target = round
            .nearest_sector_with(location, 0x8, 0x2_0000)
            .and_then(|sector| {
                round.nearest_system_in(
                    sector,
                    location,
                    SystemFilter {
                        require: role::SHIPYARD,
                        exclude: bit::BLOCKADE,
                        own: false,
                    },
                )
            })
            .or_else(|| {
                let sector = round.nearest_sector_with(location, 0x8, 0x4)?;
                round.nearest_system_in(
                    sector,
                    location,
                    SystemFilter {
                        require: 0,
                        exclude: bit::BLOCKADE,
                        own: true,
                    },
                )
            })?;
        (target != location).then_some(Order::MoveShips {
            fleet: chosen,
            ships: team,
            to: target,
        })
    }

    /// `FUN_004ca5e0` ("U5"): the class mask, key and blocker of the ship
    /// module 11 asks for: the regional records' shortfalls first, then
    /// the fallback rows.
    fn ship_request(&mut self, round: &Round<'_>) -> (u32, u32, u32) {
        let blockers = self.module(Module::CapitalShips).blockers;
        let clear = |b: u32| blockers & b == 0;
        let world = round.inputs.world;
        let is_alliance = round.inputs.side == Faction::Alliance;
        // The records whose need `rec+0x64` holds each shortfall bit, while
        // its blocker is clear; bit 0x800 only while c+4 & 0x1000000 (the
        // Death Star shield class is producible).
        let shield = !is_alliance && ds_shield_available(world, round.inputs.research);
        let count = |bit: u32, blocker: u32| {
            if !clear(blocker) || (bit == 0x800 && !shield) {
                return 0;
            }
            round.record_needs.iter().filter(|&&n| n & bit != 0).count()
        };
        let n1 = count(0x1, 0x100_0000);
        let n80 = count(0x80, 0x200_0000);
        let n4 = count(0x4, 0x400_0000);
        let n8 = count(0x8, 0x800_0000);
        let n10 = count(0x10, 0x1000_0000);
        let n800 = count(0x800, 0x4000_0000);
        if n4 >= 1 {
            return (0x10_0004, 0x100, 0x400_0000);
        }
        if n1 >= 1 && [n80, n4, n10, n8, n800].iter().all(|&n| n1 >= n) {
            return (0x10_0001, 0x8_0000, 0x100_0000);
        }
        if n80 >= 1 && [n4, n10, n8, n800].iter().all(|&n| n80 >= n) {
            return (0x10_0002, 0x400, 0x200_0000);
        }
        if n10 >= 1 && n10 >= n8 && n10 >= n800 {
            return (0x10_0800, 0x800, 0x1000_0000);
        }
        if n8 >= 1 && n8 >= n800 {
            return (0x10_0102, 0x80, 0x800_0000);
        }
        if n800 >= 1 {
            return (0x10_0400, 0x1_0000, 0x4000_0000);
        }
        let own_fleets: Vec<&crate::world::Fleet> = world
            .fleets
            .values()
            .filter(|f| f.is_alliance == is_alliance)
            .collect();
        let node = |class| {
            ClassNode::of(
                world,
                crate::manufacturing::BuildableKind::CapitalShip(class),
            )
        };
        let ships = || {
            own_fleets
                .iter()
                .flat_map(|f| f.capital_ships.iter().filter(|s| s.alive))
                .filter_map(|s| node(s.class))
        };
        // c+0x260: troop capacity of transports (node flag 4).
        let transport: i64 = ships().filter(|n| n.flags & 0x4 != 0).map(|n| n.n48).sum();
        // c+0x20c: the largest threat at a system the side does not hold.
        let threat = round
            .records
            .values()
            .filter(|r| !r.own)
            .map(|r| r.threat)
            .max()
            .unwrap_or(0);
        // c+0x268, c+0x26c, c+0x27c: hyp: over every own fleet (fleet+0x38
        // bit 4 untraced).
        let flag1 = ships().filter(|n| n.flags & 0x1 != 0).count();
        let flag2 = ships().filter(|n| n.flags & 0x2 != 0).count();
        let capacity: i64 = ships().map(|n| n.n44).sum();
        // c+0x16c: the best fighter capacity of a producible non-Death Star
        // capital ship class.
        let best_carrier = crate::build_selection::listed_classes(
            world,
            round.inputs.research,
            crate::manufacturing::ProductionArea::Shipyard,
            is_alliance,
        )
        .into_iter()
        .filter_map(|k| ClassNode::of(world, k))
        .filter(|n| n.node == NodeKind::CapitalShip && n.flags & 0x10_0000 != 0)
        .map(|n| n.n44)
        .max()
        .unwrap_or(0);
        let own_death_star = own_death_star(world, round.inputs.side);
        let listed = self.module(Module::CapitalShips).list.len();
        if clear(0x400_0000) && transport < threat {
            (0x10_0004, 0x100, 0x400_0000)
        } else if !is_alliance && listed > 1 && !own_death_star && clear(0x2000_0000) {
            (0x80, 0x200, 0x2000_0000)
        } else if clear(0x100_0000) && flag1 < 1 {
            (0x10_0001, 0x8_0000, 0x100_0000)
        } else if clear(0x200_0000) && flag2 < 1 {
            (0x10_0002, 0x400, 0x200_0000)
        } else if clear(0x800_0000) && capacity <= best_carrier {
            (0x10_0102, 0x80, 0x800_0000)
        } else {
            (0, 0, 0)
        }
    }

    /// `FUN_004cab20`: scrap the cheapest ship of the own fleet with the
    /// least ship maintenance. hyp: the fleet and ship flag filters
    /// (`0x5`/`0x402`, `0x24000`/`0x2800`) admit every fleet and ship.
    fn scrap_ship(&mut self, round: &mut Round<'_>) -> Option<Order> {
        let world = round.inputs.world;
        let is_alliance = round.inputs.side == Faction::Alliance;
        let maintenance = |class| {
            world
                .capital_ship_classes
                .get(class)
                .map_or(0, |c: &crate::world::CapitalShipClass| {
                    i64::from(c.maintenance_cost)
                })
        };
        let fleets: Vec<(crate::ids::FleetKey, i64)> = world
            .fleets
            .iter()
            .filter(|(_, f)| f.is_alliance == is_alliance)
            .map(|(k, f)| {
                (
                    k,
                    f.capital_ships
                        .iter()
                        .filter(|s| s.alive)
                        .map(|s| maintenance(s.class))
                        .sum(),
                )
            })
            .filter(|&(_, m)| m > 0)
            .collect();
        let fleet = rank_pick(&[(fleets, Dir::Smallest)], &mut round.rng)?;
        let ships: Vec<(usize, i64)> = world.fleets[fleet]
            .capital_ships
            .iter()
            .enumerate()
            .filter(|(_, s)| s.alive)
            .map(|(i, s)| (i, maintenance(s.class)))
            .collect();
        let index = rank_pick(&[(ships, Dir::Smallest)], &mut round.rng)?;
        Some(Order::Scrap(vec![crate::scrap::ScrapTarget::CapitalShip {
            fleet,
            index,
        }]))
    }

    // -----------------------------------------------------------------------
    // Module 10: fighters for fleets (`FUN_004cc9c0`)
    // -----------------------------------------------------------------------

    /// Phases `0x3f2` and `0x3ec` (`FUN_004cca20`; "U6").
    fn fleet_fighters(&mut self, round: &mut Round<'_>) -> Vec<Order> {
        const M: Module = Module::FleetFighters;
        if self.module(M).phase == 0x3ec {
            self.module(M).phase = 0x3f2;
            return self.facility_pass(M, YardKind::SHIPYARD, round);
        }
        self.module(M).phase = 0x3ec;
        let budget = round.census.budget(M, 80, round.census.fleet_fighters);
        let producer = self.unit_producer(
            M,
            YardKind::SHIPYARD,
            Cache::FleetFighters,
            &[Cache::SystemFighters, Cache::CapitalShips],
            round,
        );
        let target = producer.and_then(|_| Self::hangar_target(round));
        let (Some(producer), Some(target)) = (producer, target) else {
            if budget < 0 {
                return self.scrap_fighter_aboard(round).into_iter().collect();
            }
            return self.fleet_fighters(round);
        };
        if budget < 0 {
            return self.scrap_fighter_aboard(round).into_iter().collect();
        }
        if budget == 0 {
            return Vec::new();
        }
        let key = if crate::uprising::Draws::draw(&mut round.rng, 9) > 5 {
            0x8000
        } else {
            0x400
        };
        let Some((kind, maintenance)) = round.class(
            crate::manufacturing::ProductionArea::Shipyard,
            0x20_0000,
            key,
            Dir::Largest,
        ) else {
            return Vec::new();
        };
        let count = cut_to_budget(1, maintenance, budget).min(producer.1);
        let count = u32::try_from(count).unwrap_or(0);
        round
            .build_to(
                producer.0,
                kind,
                count,
                Some(crate::manufacturing::Destination::Fleet(target)),
            )
            .into_iter()
            .collect()
    }

    /// `FUN_004cd040` with no regional record: the own fleet with free
    /// hangar slots that has the fewest (`FUN_0041a4c0(c, 0x100005, 0xc02,
    /// 0x17, 2)`); the Destination names the fleet.
    fn hangar_target(round: &mut Round<'_>) -> Option<crate::ids::FleetKey> {
        let world = round.inputs.world;
        let is_alliance = round.inputs.side == Faction::Alliance;
        let fleets: Vec<(crate::ids::FleetKey, i64)> = world
            .fleets
            .iter()
            .filter(|(_, f)| f.is_alliance == is_alliance)
            .filter_map(|(k, f)| {
                let capacity: i64 = f
                    .capital_ships
                    .iter()
                    .filter(|s| s.alive)
                    .filter_map(|s| world.capital_ship_classes.get(s.class))
                    .map(|c| i64::from(c.fighter_capacity))
                    .sum();
                let aboard: i64 = f.fighters.iter().map(|e| i64::from(e.count)).sum();
                (capacity > aboard).then_some((k, capacity - aboard))
            })
            .collect();
        rank_pick(&[(fleets, Dir::Smallest)], &mut round.rng)
    }

    /// `FUN_004cd310`: scrap a fighter squadron aboard the own fleet with
    /// the most fighter maintenance. hyp: the squadron with the least
    /// maintenance stands for the smallest vtable `+500` value.
    fn scrap_fighter_aboard(&mut self, round: &mut Round<'_>) -> Option<Order> {
        let world = round.inputs.world;
        let is_alliance = round.inputs.side == Faction::Alliance;
        let cost = |class| {
            world
                .fighter_classes
                .get(class)
                .map_or(0, |c: &crate::world::FighterClass| {
                    i64::from(c.maintenance_cost)
                })
        };
        let fleets: Vec<(crate::ids::FleetKey, i64)> = world
            .fleets
            .iter()
            .filter(|(_, f)| f.is_alliance == is_alliance && f.fighters.iter().any(|e| e.count > 0))
            .map(|(k, f)| {
                (
                    k,
                    f.fighters
                        .iter()
                        .map(|e| cost(e.class) * i64::from(e.count))
                        .sum(),
                )
            })
            .collect();
        let fleet = rank_pick(&[(fleets, Dir::Largest)], &mut round.rng)?;
        let entries: Vec<(usize, i64)> = world.fleets[fleet]
            .fighters
            .iter()
            .enumerate()
            .filter(|(_, e)| e.count > 0)
            .map(|(i, e)| (i, cost(e.class)))
            .collect();
        let entry = rank_pick(&[(entries, Dir::Smallest)], &mut round.rng)?;
        Some(Order::Scrap(vec![crate::scrap::ScrapTarget::Fighter {
            fleet,
            entry,
        }]))
    }

    // -----------------------------------------------------------------------
    // Module 7: fighters for systems (`FUN_004d3100`)
    // -----------------------------------------------------------------------

    /// Phases `0x3f2` and `0x3ec` ("U7"). hyp: phase `0x3f1` (surplus
    /// moves, `FUN_004d3170`) is not read and emits nothing.
    fn system_fighters(&mut self, round: &mut Round<'_>) -> Vec<Order> {
        const M: Module = Module::SystemFighters;
        if self.module(M).phase == 0x3ec {
            self.module(M).phase = 0x3f2;
            return self.facility_pass(M, YardKind::SHIPYARD, round);
        }
        self.module(M).phase = 0x3ec;
        let producer = self.unit_producer(
            M,
            YardKind::SHIPYARD,
            Cache::SystemFighters,
            &[Cache::FleetFighters, Cache::CapitalShips],
            round,
        );
        let target = match producer {
            Some(p) => self.fighter_target(p.0, round),
            None => None,
        };
        let (Some(producer), Some(target)) = (producer, target) else {
            if round.census.budget(M, 90, round.census.system_fighters) < 0 {
                return self.scrap_system_fighter(round).into_iter().collect();
            }
            return self.system_fighters(round);
        };
        let budget = round.census.budget(M, 80, round.census.system_fighters);
        if budget < 0 {
            return self.scrap_system_fighter(round).into_iter().collect();
        }
        if budget == 0 {
            return Vec::new();
        }
        let Some(r) = round.record(target).cloned() else {
            return Vec::new();
        };
        let (key, dir) = if r.a & (role::ANY | bit::HQ) == 0 {
            (1, Dir::Smallest)
        } else {
            (0x400, Dir::Largest)
        };
        let Some((kind, maintenance)) = round.class(
            crate::manufacturing::ProductionArea::Shipyard,
            0x20_0000,
            key,
            dir,
        ) else {
            return Vec::new();
        };
        let count = cut_to_budget(-r.fighter_balance, maintenance, budget).min(producer.1);
        let count = u32::try_from(count).unwrap_or(0);
        round
            .build(producer.0, kind, count, Some(target))
            .into_iter()
            .collect()
    }

    /// `FUN_004d4370`: an own system short of fighters, a role system
    /// first, near the producer; else the fighter targets rise.
    fn fighter_target(&mut self, producer: SystemKey, round: &mut Round<'_>) -> Option<SystemKey> {
        for (flags, exclude) in [
            (0x400_0000, bit::UNASSIGNED | bit::BLOCKED),
            (0x4000_0000, bit::BLOCKED),
            (0x100_0000, bit::BLOCKED),
        ] {
            if let Some(sector) = round.nearest_sector(producer, flags) {
                let filter = SystemFilter {
                    require: bit::FIGHTER_SHORT,
                    exclude,
                    own: true,
                };
                if let Some(s) = round.system_in(sector, filter, &[(0x26, Dir::Smallest)]) {
                    if round.record(s).is_some_and(|r| r.fighter_balance < 0) {
                        return Some(s);
                    }
                }
            }
        }
        if round
            .census
            .budget(Module::SystemFighters, 80, round.census.system_fighters)
            > 0
        {
            for index in 9..=11 {
                self.targets[index] += 1;
            }
            // FUN_00418240 -> FUN_004311d0 (FUN_004d76f0.c:138).
            round.dirty = true;
        }
        None
    }

    /// `FUN_004d4800`: over budget, a squadron leaves a system with a
    /// surplus. hyp: `FUN_0041a600` and the Move it may issue are not
    /// read; the squadron is scrapped where the surplus is largest.
    fn scrap_system_fighter(&mut self, round: &mut Round<'_>) -> Option<Order> {
        let is_alliance = round.inputs.side == Faction::Alliance;
        // The records are walked in the world's system order: rank_pick
        // draws per entry, so a hash map's order would reach the stream.
        let systems: Vec<(SystemKey, i64)> = round
            .inputs
            .world
            .systems
            .keys()
            .filter_map(|key| round.records.get(&key))
            .filter(|r| r.own && r.fighter_surplus > 0)
            .map(|r| (r.key, r.fighter_surplus))
            .collect();
        let system = rank_pick(&[(systems, Dir::Largest)], &mut round.rng)?;
        let world = round.inputs.world;
        let entry = world.systems[system]
            .fighters
            .iter()
            .find(|e| e.is_alliance == is_alliance && e.count > 0)?;
        Some(Order::Scrap(vec![
            crate::scrap::ScrapTarget::SystemFighter {
                system,
                class: entry.class,
                is_alliance,
            },
        ]))
    }

    // -----------------------------------------------------------------------
    // Module 5: special forces (`FUN_004cfed0`)
    // -----------------------------------------------------------------------

    /// Phases `0x3f4` and `0x3ec` (`FUN_004cfed0`; "U8").
    fn special_forces(&mut self, round: &mut Round<'_>) -> Vec<Order> {
        const M: Module = Module::SpecialForces;
        if self.module(M).phase == 0x3ec {
            self.module(M).phase = 0x3f4;
            return self.facility_pass(M, YardKind::TRAINING, round);
        }
        self.module(M).phase = 0x3ec;
        let budget = round.census.budget(M, 70, round.census.special_forces);
        let producer = self.unit_producer(
            M,
            YardKind::TRAINING,
            Cache::SpecialForces,
            &[Cache::Regiments],
            round,
        );
        let mask = producer.and_then(|_| self.special_force_need(round));
        let (Some(producer), Some(mask)) = (producer, mask) else {
            if budget < 0 {
                return self.retire_special_force(round).into_iter().collect();
            }
            return self.special_forces(round);
        };
        if budget < 0 {
            return self.retire_special_force(round).into_iter().collect();
        }
        if budget == 0 {
            return Vec::new();
        }
        let Some((kind, maintenance)) = round.class(
            crate::manufacturing::ProductionArea::TrainingFacility,
            mask,
            1,
            Dir::Largest,
        ) else {
            return Vec::new();
        };
        let count = u32::try_from(cut_to_budget(1, maintenance, budget)).unwrap_or(0);
        round
            .build(producer.0, kind, count, None)
            .into_iter()
            .collect()
    }

    /// `FUN_004d0480`: the next class bit, from where the cycle stopped,
    /// whose special forces number fewer than max(`c+0x2fc`, 8). hyp:
    /// `c+0x2fc` reads 0 (the character census zeroes it).
    fn special_force_need(&mut self, round: &Round<'_>) -> Option<u32> {
        let world = round.inputs.world;
        let is_alliance = round.inputs.side == Faction::Alliance;
        let mut counts = [0_i64; 4];
        for unit in world
            .special_forces
            .values()
            .filter(|u| u.is_alliance == is_alliance)
        {
            let bit = match unit.class_dat_id.raw() & 0xff_ffff {
                1 | 5 => Some(0),
                2 | 6 => Some(1),
                3 | 7 => Some(2),
                4 | 8 => Some(3),
                _ => None,
            };
            if let Some(b) = bit {
                counts[b] += 1;
            }
        }
        let state = self.module(Module::SpecialForces);
        for _ in 0..4 {
            state.cursor = (state.cursor + 1) % 4;
            let k = state.cursor as usize;
            if counts[k] < 8 {
                return Some(0x100_0000 << k);
            }
        }
        None
    }

    /// `FUN_004d0610`: retire the special force with the most maintenance.
    /// hyp: a Retire removes the unit as a Scrap does.
    fn retire_special_force(&mut self, round: &mut Round<'_>) -> Option<Order> {
        let world = round.inputs.world;
        let is_alliance = round.inputs.side == Faction::Alliance;
        let units: Vec<(crate::ids::SpecialForceKey, i64)> = world
            .special_forces
            .iter()
            // Retire is disabled for a unit en route (FUN_004f9860).
            .filter(|(_, u)| u.is_alliance == is_alliance && !u.on_mission && !u.en_route)
            .map(|(k, u)| {
                (
                    k,
                    world
                        .buildable_classes
                        .get(&u.class_dat_id)
                        .map_or(0, |c| i64::from(c.maintenance_cost)),
                )
            })
            .collect();
        rank_pick(&[(units, Dir::Largest)], &mut round.rng).map(Order::Retire)
    }

    // -----------------------------------------------------------------------
    // Module 6: regiments (`FUN_004dd7a0`)
    // -----------------------------------------------------------------------

    /// Phases `0x3f0` and `0x3ec` ("U9"). hyp: phase `0x3ef` (surplus
    /// moves) is not read and emits nothing.
    fn regiments(&mut self, round: &mut Round<'_>) -> Vec<Order> {
        const M: Module = Module::Regiments;
        if self.module(M).phase == 0x3ec {
            self.module(M).phase = 0x3f0;
            return self.facility_pass(M, YardKind::TRAINING, round);
        }
        self.module(M).phase = 0x3ec;
        let producer = self.unit_producer(
            M,
            YardKind::TRAINING,
            Cache::Regiments,
            &[Cache::SpecialForces],
            round,
        );
        let need = producer.and_then(|p| self.regiment_need(p.0, round));
        let (Some(producer), Some((target, count, mask, key, dir))) = (producer, need) else {
            return self.regiments(round);
        };
        let budget = regiment_budget(round);
        // FUN_004df9a0: a strong side with no budget lowers the regiment
        // targets.
        if round.ctx & crate::ai_strategy::ctx::STRONG != 0 && budget < 1 {
            for (index, floor) in [(8, 9), (7, 3), (6, 0)] {
                if self.targets[index] > floor {
                    self.targets[index] -= 1;
                    // FUN_00418270 -> FUN_004311d0 (FUN_004df9a0.c:37-43).
                    round.dirty = true;
                }
            }
        }
        if budget < 0 {
            return self.scrap_regiment(round).into_iter().collect();
        }
        if budget == 0 {
            return Vec::new();
        }
        let Some((kind, maintenance)) = round.class(
            crate::manufacturing::ProductionArea::TrainingFacility,
            mask,
            key,
            dir,
        ) else {
            return Vec::new();
        };
        let count = cut_to_budget(count, maintenance, budget).min(producer.1);
        let count = u32::try_from(count).unwrap_or(0);
        round
            .build(producer.0, kind, count, Some(target))
            .into_iter()
            .collect()
    }

    /// `FUN_004df440`: the selectors in the rotation's order until one
    /// names a system: (target, count, mask, key, direction).
    fn regiment_need(
        &mut self,
        producer: SystemKey,
        round: &mut Round<'_>,
    ) -> Option<(SystemKey, i64, u32, u32, Dir)> {
        let state = self.module(Module::Regiments);
        let rotation = state.cursor;
        state.cursor = if rotation >= 4 { 1 } else { rotation + 1 };
        if rotation == 0 {
            return None;
        }
        const A: u8 = 0;
        const B: u8 = 1;
        const C: u8 = 2;
        const D: u8 = 3;
        const E: u8 = 4;
        let order: &[u8] = match rotation {
            1 => &[A, B, D, E, C],
            2 => &[B, D, D, E, C, A],
            3 => &[E, C, A, B, D],
            _ => &[C, A, B, D, E],
        };
        let short = |exclude| SystemFilter {
            require: bit::REGIMENT_SHORT,
            exclude,
            own: true,
        };
        let mut failed = 0u32;
        let mut result = None;
        for &selector in order {
            let found = match selector {
                A => round.nearest_sector(producer, 0x80_0000).and_then(|s| {
                    round
                        .system_in(
                            s,
                            short(bit::UNASSIGNED | bit::BLOCKED),
                            &[(0x15, Dir::Smallest)],
                        )
                        .map(|t| (t, 0x2000, 0x2000, Dir::Largest))
                }),
                B => round.nearest_sector(producer, 0x40_0000).and_then(|s| {
                    let filter = SystemFilter {
                        require: bit::DEFENDER_SHORT,
                        exclude: bit::BLOCKED,
                        own: true,
                    };
                    round
                        .system_in(s, filter, &[(0x15, Dir::Smallest)])
                        .map(|t| {
                            if round
                                .record(t)
                                .is_some_and(|r| r.a & (role::ANY | bit::HQ) != 0)
                            {
                                (t, 0x2000, 0x2000, Dir::Largest)
                            } else {
                                (t, 0x2000, 1, Dir::Smallest)
                            }
                        })
                }),
                C => round
                    .sector(0x4000_0000, &[(0x1e, Dir::Smallest)])
                    .and_then(|s| {
                        round
                            .system_in(s, short(bit::BLOCKED), &[(0x15, Dir::Smallest)])
                            .map(|t| (t, 0x2000, 0x2000, Dir::Largest))
                    }),
                D => round.nearest_sector(producer, 0x4_0000).and_then(|s| {
                    round
                        .system_in(s, short(bit::BLOCKED), &[(0x15, Dir::Smallest)])
                        .map(|t| (t, 0x2000, 1, Dir::Smallest))
                }),
                _ => {
                    // FUN_004e12e0: the producer's own system, with as many
                    // attackers as it has training facilities.
                    let cap = round.record(producer).map_or(0, |r| r.yards[1]);
                    result = Some((producer, cap, 0x1000, 0x1000, Dir::Largest));
                    break;
                }
            };
            if let Some((target, mask, key, dir)) = found {
                let count = round.record(target).map_or(0, |r| r.regiment_shortage);
                result = Some((target, count, mask, key, dir));
                break;
            }
            failed |= [1, 8, 2, 4][usize::from(selector)];
        }
        // m+0x28: with A to D all failed, a weak side with budget raises
        // the regiment targets.
        if failed & 0xf == 0xf
            && round.ctx & crate::ai_strategy::ctx::WEAK != 0
            && regiment_budget(round) > 0
        {
            for (index, ceiling) in [(8, 12), (7, 6), (6, 4)] {
                if self.targets[index] < ceiling {
                    self.targets[index] += 1;
                    // FUN_00418240 -> FUN_004311d0 (FUN_004df440.c:244-250).
                    round.dirty = true;
                }
            }
        }
        result
    }

    /// `FUN_004dfa80`: scrap a regiment where the side has a surplus.
    fn scrap_regiment(&mut self, round: &mut Round<'_>) -> Option<Order> {
        let world = round.inputs.world;
        let is_alliance = round.inputs.side == Faction::Alliance;
        let sector = round.sector(0x8_0000, &[(0x1f, Dir::Smallest)])?;
        let filter = SystemFilter {
            require: bit::REGIMENT_SURPLUS,
            exclude: 0,
            own: true,
        };
        let system = round.system_in(sector, filter, &[(0x13, Dir::Smallest)])?;
        let troop = world.systems[system].ground_units.iter().find(|t| {
            world
                .troops
                .get(**t)
                .is_some_and(|t| t.is_alliance == is_alliance)
        })?;
        Some(Order::Scrap(vec![crate::scrap::ScrapTarget::Troop(*troop)]))
    }
}
