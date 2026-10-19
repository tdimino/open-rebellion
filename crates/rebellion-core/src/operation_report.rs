//! The Operation Report's contents: message kind `0xf` (ctor `FUN_00496460`,
//! vtable `0x0065b870`), which serves the bombardment and the assault
//! reports (`ghidra/notes/operation-report-window.md`).
//!
//! The message holds six object lists that the strike's events fill
//! (`FUN_00496770`, `FUN_00496f50`, `FUN_00497380`): the fleet side's
//! operational (`+0x78`), destroyed (`+0x88`) and captured (`+0x90`)
//! objects, and the system's (`+0x70`, `+0x80`, `+0x98`). The report
//! window (`FUN_0044c410`) lists them by family under six tabs.

use crate::dat::Faction;
use crate::ids::{CharacterKey, DatId, FleetKey, SpecialForceKey, SystemKey};
use crate::troop_transport::TroopTransportState;
use crate::world::{FacilityRef, GameWorld};

/// One object a list holds: its record, whose high byte is the family the
/// window's tabs filter on (`FUN_0044e500`), and its name (`FUN_004f62d0`),
/// kept because a destroyed object leaves the world.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportUnit {
    /// The class record, or a character's own record (families
    /// `0x30`-`0x3b`).
    pub dat_id: DatId,
    pub name: String,
    /// A major character, whose mini lies in another range
    /// (`tactical::character_mini_resource_id`).
    pub major: bool,
    /// The live object the window's status marks read as it draws
    /// ([`ReportObject`]); `None` reads no state.
    pub object: Option<ReportObject>,
}

/// The object an entry stands for. The list holds the object itself, and
/// each refill draws it through `FUN_0042c3b0`, whose status marks read its
/// state then (`FUN_0044edb0`). port: squadrons and regiments have no
/// handle of their own, so one aboard a fleet reads the fleet it was listed
/// in (`FUN_004f8240`: en route while its container is), and one at the
/// system or destroyed reads nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportObject {
    /// A squadron or regiment aboard the fleet.
    Aboard(FleetKey),
    /// A capital ship by its world-unique tag (`ShipInstance::tag`), and
    /// the fleet it was listed in for a ship not yet tagged.
    Ship { fleet: FleetKey, tag: u32 },
    Character(CharacterKey),
    SpecialForce(SpecialForceKey),
    /// An object destroyed and gone from the world, with the damaged bit
    /// (`+0x50` bit 9) its view kept: the destroy slot `+0xac`
    /// (`FUN_004f71d0`) writes only the cause `+0x40`, so a ship the hull
    /// loss killed keeps the bit `FUN_00501ae0` set first.
    Destroyed { damaged: bool },
}

impl ReportUnit {
    /// The family byte the tabs read (`id >> 24`, `FUN_0044e500.c:178`).
    #[must_use]
    pub fn family(&self) -> u8 {
        self.dat_id.family()
    }
}

/// STRATEGY 11162, the intact landscape with a shield generator: the
/// picture (`+0x2e`) of a bombardment that destroyed nothing
/// (`FUN_00496770` code 2).
pub const PICTURE_BOMBARD_INTACT: u16 = 0x2b9a;
/// STRATEGY 11163, the ruined landscape: something at the system died
/// (`+0x80`, `FUN_00496770` code 2).
pub const PICTURE_BOMBARD_RUINED: u16 = 0x2b9b;
/// STRATEGY 11164, a ship exploding: only the fleet side lost something
/// (`+0x88`, `FUN_00496770` code 2).
pub const PICTURE_BOMBARD_SHIP_LOST: u16 = 0x2b9c;
/// STRATEGY 11160, an assault by side 1 (`FUN_00496770` code 2).
pub const PICTURE_ASSAULT_ALLIANCE: u16 = 0x2b98;
/// STRATEGY 11161, an assault by any other side (`FUN_00496770` code 2).
pub const PICTURE_ASSAULT_EMPIRE: u16 = 0x2b99;

/// The report message's fields (`FUN_00496460`, `FUN_00496380`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationReport {
    /// `+0x5c`.
    pub system: SystemKey,
    /// `+0x64`: an assault (event `0x150`) rather than a bombardment.
    pub assault: bool,
    /// `+0x68`: the system's holder at the event, 0 for a neutral system.
    pub holder: Faction,
    /// `+0x6c`: the bombarding or assaulting fleet's owner.
    pub fleet_owner: Faction,
    /// The fleets whose event `0x182`/`0x183` filled the fleet side, for
    /// the Display layer's fleet button.
    pub fleets: Vec<FleetKey>,
    /// `+0x78`, `+0x88`, `+0x90`.
    pub fleet_operational: Vec<ReportUnit>,
    pub fleet_destroyed: Vec<ReportUnit>,
    pub fleet_captured: Vec<ReportUnit>,
    /// `+0x70`, `+0x80`, `+0x98`.
    pub system_operational: Vec<ReportUnit>,
    pub system_destroyed: Vec<ReportUnit>,
    pub system_captured: Vec<ReportUnit>,
}

impl OperationReport {
    /// An empty report of a run at `system`.
    #[must_use]
    pub fn new(system: SystemKey, assault: bool, holder: Faction, fleet_owner: Faction) -> Self {
        Self {
            system,
            assault,
            holder,
            fleet_owner,
            fleets: Vec::new(),
            fleet_operational: Vec::new(),
            fleet_destroyed: Vec::new(),
            fleet_captured: Vec::new(),
            system_operational: Vec::new(),
            system_destroyed: Vec::new(),
            system_captured: Vec::new(),
        }
    }

    /// The picture (`+0x2e`, `FUN_00496770` code 2): a bombardment shows
    /// the ruined landscape when anything at the system was destroyed
    /// (`+0x80`), else the exploding ship when the fleet side lost
    /// something (`+0x88`), else the intact landscape; an assault shows the
    /// owner's art, `0x2b98` for side 1 and `0x2b99` otherwise.
    #[must_use]
    pub fn picture(&self) -> u16 {
        if self.assault {
            if self.fleet_owner == Faction::Alliance {
                PICTURE_ASSAULT_ALLIANCE
            } else {
                PICTURE_ASSAULT_EMPIRE
            }
        } else if !self.system_destroyed.is_empty() {
            PICTURE_BOMBARD_RUINED
        } else if !self.fleet_destroyed.is_empty() {
            PICTURE_BOMBARD_SHIP_LOST
        } else {
            PICTURE_BOMBARD_INTACT
        }
    }

    /// Event `0x182` with value 0 for each fleet (`FUN_00496770`), sent as
    /// the run's finalize clears the fleets' bombard bit: each ship of the
    /// fleet, then each child of that ship, goes to `+0x78`, a captive
    /// Personnel child (`+0xac` bit 0) to `+0x90` instead.
    pub fn fill_fleets(
        &mut self,
        world: &GameWorld,
        transport: &TroopTransportState,
        fleets: &[FleetKey],
    ) {
        for &fleet in fleets {
            if world.fleets.contains_key(fleet) && !self.fleets.contains(&fleet) {
                self.fleets.push(fleet);
            }
            for (unit, captive) in fleet_units(world, transport, fleet) {
                if captive {
                    self.fleet_captured.push(unit);
                } else {
                    self.fleet_operational.push(unit);
                }
            }
        }
    }

    /// Event `0x14f` with value 0 (`FUN_00496f50`): each child of the
    /// system goes to `+0x70`, a captive Personnel child to `+0x98`.
    pub fn fill_system(&mut self, world: &GameWorld) {
        for (unit, child) in system_units(world, self.system) {
            if matches!(child, Child::Character { captive: true, .. }) {
                self.system_captured.push(unit);
            } else {
                self.system_operational.push(unit);
            }
        }
    }

    /// Event `0x150` with value 0 (`FUN_00496f50`), the assault's: each
    /// child of the system splits against the holder (`+0x68`): its
    /// regiments (families `0x10`-`0x13`) go to `+0x70` and any other
    /// regiment to `+0x78`; a free character to `+0x70`, a captive one to
    /// `+0x90` when its own side (`FUN_004edc80`) is the holder, else
    /// `+0x98`; everything else to `+0x70`.
    pub fn fill_assaulted_system(&mut self, world: &GameWorld) {
        for (unit, child) in system_units(world, self.system) {
            match child {
                Child::Regiment(owner) if owner != self.holder => {
                    self.fleet_operational.push(unit);
                }
                Child::Character {
                    captive: true,
                    side,
                } => {
                    if side == self.holder {
                        self.fleet_captured.push(unit);
                    } else {
                        self.system_captured.push(unit);
                    }
                }
                _ => self.system_operational.push(unit),
            }
        }
    }
}

/// What a child of a system is, as the fills sort it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Child {
    /// A regiment and its owner.
    Regiment(Faction),
    /// A character (Personnel), its captive bit (`+0xac` bit 0) and its
    /// own side.
    Character { captive: bool, side: Faction },
    Other,
}

/// FIGHTSD.DAT's family (its header's `family_id`, `0x1c`), which a
/// loaded squadron class's id lacks.
const FIGHTSD_FAMILY: u8 = 0x1c;

/// A class id with its family byte. The tabs sort by family (`id >> 24`,
/// `FUN_0044e500.c:178`); a loaded CAPSHPSD or FIGHTSD id is the bare
/// record id, so it takes its file's family.
pub(crate) fn with_family(dat_id: DatId, family: u8) -> DatId {
    if dat_id.family() == 0 {
        DatId::new(u32::from(family) << 24 | dat_id.index())
    } else {
        dat_id
    }
}

/// A capital ship's entry: its own name, else its class's (`FUN_004f6270`),
/// under its record's family (`0x14`, the Death Star `0x18`).
#[must_use]
pub fn capital_ship(world: &GameWorld, ship: &crate::world::ShipInstance) -> Option<ReportUnit> {
    let class = world.capital_ship_classes.get(ship.class)?;
    Some(ReportUnit {
        dat_id: with_family(class.dat_id, class.family),
        name: ship.name.clone().unwrap_or_else(|| class.name.clone()),
        major: false,
        object: None,
    })
}

/// The entry of the ship at `index` in `fleet`, just destroyed: its
/// [`capital_ship`] entry with the damaged bit it died with
/// ([`ReportObject::Destroyed`]; `FUN_00501c00`: hull damage or a damage
/// counter).
#[must_use]
pub fn destroyed_ship(world: &GameWorld, fleet: FleetKey, index: usize) -> Option<ReportUnit> {
    let ship = world.fleets.get(fleet)?.capital_ships.get(index)?;
    let class = world.capital_ship_classes.get(ship.class)?;
    Some(ReportUnit {
        object: Some(ReportObject::Destroyed {
            damaged: ship.is_damaged(class.hull.cast_signed()),
        }),
        ..capital_ship(world, ship)?
    })
}

/// One squadron of `class`.
#[must_use]
pub fn squadron(world: &GameWorld, class: crate::ids::FighterKey) -> Option<ReportUnit> {
    let class = world.fighter_classes.get(class)?;
    Some(ReportUnit {
        dat_id: with_family(class.dat_id, FIGHTSD_FAMILY),
        name: class.name.clone(),
        major: false,
        object: None,
    })
}

/// A record's entry under its class's TEXTSTRA name.
fn class_unit(world: &GameWorld, dat_id: DatId) -> ReportUnit {
    ReportUnit {
        dat_id,
        name: world
            .buildable_classes
            .get(&dat_id)
            .map(|class| class.name.clone())
            .unwrap_or_default(),
        major: false,
        object: None,
    }
}

/// A regiment's entry.
#[must_use]
pub fn regiment(world: &GameWorld, troop: crate::ids::TroopKey) -> Option<ReportUnit> {
    let unit = world.troops.get(troop)?;
    Some(class_unit(world, unit.class_dat_id))
}

/// A facility's entry.
#[must_use]
pub fn facility(world: &GameWorld, facility: FacilityRef) -> Option<ReportUnit> {
    let class = match facility {
        FacilityRef::Defense(key) => world.defense_facilities.get(key)?.class_dat_id,
        FacilityRef::Manufacturing(key) => world.manufacturing_facilities.get(key)?.class_dat_id,
        FacilityRef::Production(key) => world.production_facilities.get(key)?.class_dat_id,
    };
    Some(class_unit(world, class))
}

/// The Alliance HQ's entry, its ALLFACSD record (family `0x20`, record 1),
/// which its event `0x302` lists like any object (`FUN_00497380`).
#[must_use]
pub fn headquarters(world: &GameWorld) -> ReportUnit {
    class_unit(world, DatId::new(crate::bombardment::HEADQUARTERS_CLASS))
}

/// A character's entry: its own record and name. The tabs sort by family
/// (`id >> 24`, `FUN_0044e500.c:178`), which a loaded character's id lacks,
/// so the entry takes its file's: MJCHARSD `0x30`, MNCHARSD `0x38`.
/// port: a major's own family (`0x30`-`0x37`, Luke `0x32`) is not in the
/// file; every major reads `0x30`, which sorts the same.
#[must_use]
pub fn character(world: &GameWorld, key: CharacterKey) -> Option<ReportUnit> {
    let value = world.characters.get(key)?;
    let family: u32 = if value.is_major { 0x30 } else { 0x38 };
    Some(ReportUnit {
        dat_id: DatId::new(family << 24 | value.dat_id.index()),
        name: value.display_name().to_owned(),
        major: value.is_major,
        object: Some(ReportObject::Character(key)),
    })
}

/// A special force's entry.
#[must_use]
pub fn special_force(world: &GameWorld, key: SpecialForceKey) -> Option<ReportUnit> {
    let unit = world.special_forces.get(key)?;
    Some(ReportUnit {
        object: Some(ReportObject::SpecialForce(key)),
        ..class_unit(world, unit.class_dat_id)
    })
}

/// A fleet's objects in the original's walk (`FUN_00502db0` over the ships,
/// `FUN_004fcd00` over each ship's children), each with its captive bit.
/// The squadrons and regiments a ship carries follow it, and the fleet's
/// characters follow the ship they ride (`carriage::character_ship`).
fn fleet_units(
    world: &GameWorld,
    transport: &TroopTransportState,
    fleet: FleetKey,
) -> Vec<(ReportUnit, bool)> {
    let Some(value) = world.fleets.get(fleet) else {
        return Vec::new();
    };
    let aboard = |unit: ReportUnit| {
        (
            ReportUnit {
                object: Some(ReportObject::Aboard(fleet)),
                ..unit
            },
            false,
        )
    };
    let carried = |tag: u32, out: &mut Vec<(ReportUnit, bool)>| {
        for entry in value.fighters.iter().filter(|entry| entry.carrier == tag) {
            for _ in 0..entry.count {
                out.extend(squadron(world, entry.class).map(aboard));
            }
        }
        for &troop in transport.cargo(fleet) {
            if transport.carrier(troop) == tag {
                out.extend(regiment(world, troop).map(aboard));
            }
        }
    };
    let characters = |out: &mut Vec<(ReportUnit, bool)>| {
        for &key in &value.characters {
            if let Some(found) = world.characters.get(key).filter(|found| !found.is_killed) {
                out.extend(character(world, key).map(|unit| (unit, found.is_captive)));
            }
        }
    };
    let rider = crate::carriage::character_ship(value);
    let mut out = Vec::new();
    let mut riders_listed = false;
    for (index, ship) in value.capital_ships.iter().enumerate() {
        // FUN_00496770.c:56,157: the ships in mode 1 (FUN_00502db0(fleet,
        // 1)), so none unfinished or on its own leg (0x004f7b80).
        if !ship.usable() {
            continue;
        }
        out.extend(capital_ship(world, ship).map(|unit| {
            (
                ReportUnit {
                    object: Some(ReportObject::Ship {
                        fleet,
                        tag: ship.tag,
                    }),
                    ..unit
                },
                false,
            )
        }));
        carried(ship.tag, &mut out);
        if rider == Some(index) {
            characters(&mut out);
            riders_listed = true;
        }
    }
    // port: the port keeps no character's ship, so characters whose ship
    // the walk left out still list, after the ships.
    if !riders_listed {
        characters(&mut out);
    }
    out
}

/// A system's children (`FUN_004fcd00`), each with its captive bit:
/// regiments on the surface, its own squadrons, the facilities, then the
/// characters and special forces there. Fleets are children too but no tab
/// lists their families (`0x08`-`0x0f`). hyp: the system's view holds both
/// sides' children; port: characters on a mission are a mission's children,
/// not the system's, and the port's lists come in that kind order.
fn system_units(world: &GameWorld, system: SystemKey) -> Vec<(ReportUnit, Child)> {
    let Some(sys) = world.systems.get(system) else {
        return Vec::new();
    };
    let mut out: Vec<(ReportUnit, Child)> = Vec::new();
    for &troop in &sys.ground_units {
        let owner = world
            .troops
            .get(troop)
            .map(|unit| Faction::of_alliance(unit.is_alliance));
        out.extend(
            regiment(world, troop)
                .zip(owner)
                .map(|(unit, owner)| (unit, Child::Regiment(owner))),
        );
    }
    for entry in &sys.fighters {
        for _ in 0..entry.count {
            out.extend(squadron(world, entry.class).map(|unit| (unit, Child::Other)));
        }
    }
    let facilities = sys
        .defense_facilities
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
        );
    for key in facilities {
        out.extend(facility(world, key).map(|unit| (unit, Child::Other)));
    }
    for (key, value) in &world.characters {
        if value.current_system == Some(system)
            && value.current_fleet.is_none()
            && value.abode.is_none()
            && !value.is_killed
            && !value.on_mission
        {
            let child = Child::Character {
                captive: value.is_captive,
                side: Faction::of_alliance(value.is_alliance),
            };
            out.extend(character(world, key).map(|unit| (unit, child)));
        }
    }
    for &key in &sys.special_forces {
        if world
            .special_forces
            .get(key)
            .is_some_and(|unit| !unit.on_mission)
        {
            out.extend(special_force(world, key).map(|unit| (unit, Child::Other)));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use crate::carriage::ProductRoom;
    use super::*;
    use crate::world::{
        BuildableClass, CapitalShipClass, Character, FighterClass, FighterEntry, Fleet,
        ShipInstance, SpecialForceUnit, System, TroopUnit,
    };

    /// port: test fixture values, not rules.
    fn named(world: &mut GameWorld, raw: u32, name: &str) -> DatId {
        let id = DatId::new(raw);
        world.buildable_classes.insert(
            id,
            BuildableClass {
                name: name.into(),
                ..BuildableClass::default()
            },
        );
        id
    }

    fn unit(raw: u32, name: &str) -> ReportUnit {
        ReportUnit {
            dat_id: DatId::new(raw),
            name: name.into(),
            major: false,
            object: None,
        }
    }

    #[test]
    fn the_picture_follows_the_destroyed_lists_and_the_assaults_owner() {
        // FUN_00496770 code 2: 0x2b9b with +0x80, else 0x2b9c with +0x88,
        // else 0x2b9a (the shield-blocked Bespin, guide-063); an assault
        // takes 0x2b98 for side 1, else 0x2b99.
        for owner in [Faction::Alliance, Faction::Empire] {
            let mut report =
                OperationReport::new(SystemKey::default(), false, Faction::Neutral, owner);
            assert_eq!(report.picture(), 0x2b9a);
            report.fleet_destroyed.push(unit(0x1400_0001, "ship"));
            assert_eq!(report.picture(), 0x2b9c);
            report.system_destroyed.push(unit(0x1000_0001, "regiment"));
            assert_eq!(report.picture(), 0x2b9b);
            report.assault = true;
            let assault = if owner == Faction::Alliance {
                0x2b98
            } else {
                0x2b99
            };
            assert_eq!(report.picture(), assault, "{owner:?}");
        }
    }

    #[test]
    fn a_fleet_lists_each_ship_then_what_it_carries_and_its_captives_apart() {
        // FUN_00496770 on event 0x182 value 0: ships (FUN_00502db0), each
        // ship's children (FUN_004fcd00) after it; Personnel with +0xac
        // bit 0 to +0x90.
        for alliance in [true, false] {
            let mut world = GameWorld::default();
            let system = world.systems.insert(System::default());
            let cruiser = world.capital_ship_classes.insert(CapitalShipClass {
                dat_id: DatId::new(0x1400_0040),
                name: "Cruiser".into(),
                troop_capacity: 1,
                ..CapitalShipClass::default()
            });
            let wing = world.fighter_classes.insert(FighterClass {
                dat_id: DatId::new(0x1c00_0001),
                name: "Wing".into(),
                ..FighterClass::default()
            });
            let mut first = ShipInstance::new(cruiser, 10, alliance);
            first.tag = 1;
            let mut second = ShipInstance::new(cruiser, 10, alliance);
            second.tag = 2;
            second.name = Some("Cruiser 2".into());
            let mut dead = ShipInstance::new(cruiser, 0, alliance);
            dead.alive = false;
            let officer = world.characters.insert(Character {
                dat_id: DatId::new(0x3000_0240),
                name: "Officer".into(),
                is_major: true,
                ..Character::default()
            });
            let prisoner = world.characters.insert(Character {
                dat_id: DatId::new(0x3800_0340),
                name: "Prisoner".into(),
                is_captive: true,
                ..Character::default()
            });
            let fleet = world.fleets.insert(Fleet {
                location: system,
                capital_ships: vec![first, dead, second],
                fighters: vec![FighterEntry {
                    class: wing,
                    count: 3,
                    carrier: 2,
                }],
                characters: vec![officer, prisoner],
                is_alliance: alliance,
                ..Fleet::default()
            });
            // The regiment boards the first ship, the earlier on a tie
            // (carriage::ship_for_regiment).
            let class = named(&mut world, 0x1000_0005, "Regiment");
            let troop = world.troops.insert(TroopUnit {
                class_dat_id: class,
                is_alliance: alliance,
                ..TroopUnit::default()
            });
            world.systems[system].ground_units.push(troop);
            let mut transport = TroopTransportState::default();
            transport
                .load(&mut world, &ProductRoom::default(), fleet, &[troop])
                .unwrap();
            let mut report = OperationReport::new(
                system,
                false,
                Faction::Neutral,
                Faction::of_alliance(alliance),
            );
            report.fill_fleets(&world, &transport, &[fleet]);
            let names: Vec<_> = report
                .fleet_operational
                .iter()
                .map(|unit| unit.name.as_str())
                .collect();
            assert_eq!(
                names,
                ["Cruiser", "Regiment", "Officer", "Cruiser 2", "Wing", "Wing", "Wing"]
            );
            assert!(report.fleet_operational[2].major);
            // Each entry names the object its status marks read
            // (FUN_0044edb0 -> FUN_0042c3b0): a ship by its tag, what it
            // carries by the fleet, a character itself.
            let objects: Vec<_> = report
                .fleet_operational
                .iter()
                .map(|unit| unit.object)
                .collect();
            let ship = |tag| Some(ReportObject::Ship { fleet, tag });
            let aboard = Some(ReportObject::Aboard(fleet));
            assert_eq!(
                objects,
                [
                    ship(1),
                    aboard,
                    Some(ReportObject::Character(officer)),
                    ship(2),
                    aboard,
                    aboard,
                    aboard
                ]
            );
            assert_eq!(
                report.fleet_captured,
                [ReportUnit {
                    object: Some(ReportObject::Character(prisoner)),
                    ..unit(0x3800_0340, "Prisoner")
                }]
            );
            assert_eq!(report.fleets, [fleet]);
        }
    }

    #[test]
    fn a_fleet_lists_no_ship_on_its_own_leg_nor_what_it_carries_for_either_side() {
        // FUN_00496770.c:56,157 walk the fleet's ships in mode 1
        // (FUN_00502db0(fleet, 1)): a ship on its own leg from its yard
        // (bit 5, FUN_00556430) is not usable (0x004f7b80), so it and its
        // children are left out; the characters still list.
        for alliance in [true, false] {
            let mut world = GameWorld::default();
            let system = world.systems.insert(System::default());
            let cruiser = world.capital_ship_classes.insert(CapitalShipClass {
                dat_id: DatId::new(0x1400_0040),
                name: "Cruiser".into(),
                ..CapitalShipClass::default()
            });
            let wing = world.fighter_classes.insert(FighterClass {
                dat_id: DatId::new(0x1c00_0001),
                name: "Wing".into(),
                ..FighterClass::default()
            });
            let mut travelling = ShipInstance::new(cruiser, 10, alliance);
            travelling.tag = 1;
            travelling.transit = Some(crate::world::ShipTransit {
                from: system,
                to: system,
                arrival_tick: 9,
            });
            let mut standing = ShipInstance::new(cruiser, 10, alliance);
            standing.tag = 2;
            standing.name = Some("Cruiser 2".into());
            let officer = world.characters.insert(Character {
                dat_id: DatId::new(0x3000_0240),
                name: "Officer".into(),
                ..Character::default()
            });
            let fleet = world.fleets.insert(Fleet {
                location: system,
                capital_ships: vec![travelling, standing],
                fighters: vec![FighterEntry {
                    class: wing,
                    count: 1,
                    carrier: 1,
                }],
                characters: vec![officer],
                is_alliance: alliance,
                ..Fleet::default()
            });
            let mut report = OperationReport::new(
                system,
                false,
                Faction::Neutral,
                Faction::of_alliance(alliance),
            );
            report.fill_fleets(&world, &TroopTransportState::default(), &[fleet]);
            let names: Vec<_> = report
                .fleet_operational
                .iter()
                .map(|unit| unit.name.as_str())
                .collect();
            assert_eq!(names, ["Cruiser 2", "Officer"], "{alliance}");
        }
    }

    #[test]
    fn a_system_lists_both_sides_children_and_its_captives_apart() {
        // FUN_00496f50 on event 0x14f value 0: every child of the system to
        // +0x70, captive Personnel to +0x98.
        let mut world = GameWorld::default();
        let system = world.systems.insert(System::default());
        let rebels = named(&mut world, 0x1000_0005, "Mon Calamari Regiment");
        let troopers = named(&mut world, 0x1000_0006, "Stormtrooper Regiment");
        for (class, is_alliance) in [(rebels, true), (troopers, false)] {
            let troop = world.troops.insert(TroopUnit {
                class_dat_id: class,
                is_alliance,
                ..TroopUnit::default()
            });
            world.systems[system].ground_units.push(troop);
        }
        let held = world.characters.insert(Character {
            dat_id: DatId::new(0x3800_0341),
            name: "Held".into(),
            current_system: Some(system),
            is_captive: true,
            ..Character::default()
        });
        world.characters.insert(Character {
            dat_id: DatId::new(0x3800_0342),
            name: "Away".into(),
            current_system: Some(system),
            on_mission: true,
            ..Character::default()
        });
        let guerrillas = named(&mut world, 0x3c00_0001, "Guerrillas");
        let force = world.special_forces.insert(SpecialForceUnit {
            class_dat_id: guerrillas,
            ..SpecialForceUnit::default()
        });
        world.systems[system].special_forces.push(force);
        let mut report = OperationReport::new(system, false, Faction::Alliance, Faction::Empire);
        report.fill_system(&world);
        assert_eq!(
            report.system_operational,
            [
                unit(0x1000_0005, "Mon Calamari Regiment"),
                unit(0x1000_0006, "Stormtrooper Regiment"),
                ReportUnit {
                    object: Some(ReportObject::SpecialForce(force)),
                    ..unit(0x3c00_0001, "Guerrillas")
                }
            ]
        );
        assert_eq!(
            report.system_captured,
            [ReportUnit {
                object: Some(ReportObject::Character(held)),
                ..unit(0x3800_0341, "Held")
            }]
        );
    }

    #[test]
    fn an_assault_lists_the_holders_regiments_at_the_system_and_the_others_with_the_fleet() {
        // FUN_00496f50 on event 0x150 value 0: regiments owned by +0x68 to
        // +0x70, other regiments to +0x78; a captive of +0x68's side to
        // +0x90, any other captive to +0x98; the rest to +0x70.
        for holder in [Faction::Alliance, Faction::Empire] {
            let mut world = GameWorld::default();
            let system = world.systems.insert(System::default());
            let rebels = named(&mut world, 0x1000_0005, "Mon Calamari Regiment");
            let troopers = named(&mut world, 0x1000_0006, "Stormtrooper Regiment");
            for (class, is_alliance) in [(rebels, true), (troopers, false)] {
                let troop = world.troops.insert(TroopUnit {
                    class_dat_id: class,
                    is_alliance,
                    ..TroopUnit::default()
                });
                world.systems[system].ground_units.push(troop);
            }
            let person = |key, raw, name| ReportUnit {
                object: Some(ReportObject::Character(key)),
                ..unit(raw, name)
            };
            let mut captives = Vec::new();
            for (raw, name, is_alliance) in [(0x3800_0341, "Rebel", true), (0x3800_0342, "Imperial", false)] {
                let key = world.characters.insert(Character {
                    dat_id: DatId::new(raw),
                    name: name.into(),
                    is_alliance,
                    current_system: Some(system),
                    is_captive: true,
                    ..Character::default()
                });
                captives.push(person(key, raw, name));
            }
            let free = world.characters.insert(Character {
                dat_id: DatId::new(0x3800_0343),
                name: "Free".into(),
                current_system: Some(system),
                ..Character::default()
            });
            let attacker = if holder == Faction::Alliance {
                Faction::Empire
            } else {
                Faction::Alliance
            };
            let mut report = OperationReport::new(system, true, holder, attacker);
            report.fill_assaulted_system(&world);
            let (own, other) = if holder == Faction::Alliance {
                (unit(0x1000_0005, "Mon Calamari Regiment"), unit(0x1000_0006, "Stormtrooper Regiment"))
            } else {
                (unit(0x1000_0006, "Stormtrooper Regiment"), unit(0x1000_0005, "Mon Calamari Regiment"))
            };
            let (own_captive, other_captive) = if holder == Faction::Alliance {
                (captives[0].clone(), captives[1].clone())
            } else {
                (captives[1].clone(), captives[0].clone())
            };
            assert_eq!(
                report.system_operational,
                [own, person(free, 0x3800_0343, "Free")],
                "{holder:?}"
            );
            assert_eq!(report.fleet_operational, [other]);
            assert_eq!(report.fleet_captured, [own_captive]);
            assert_eq!(report.system_captured, [other_captive]);
        }
    }

    #[test]
    fn a_loaded_character_reports_under_its_files_personnel_family() {
        // FUN_0044e500.c:178 sorts the tabs by id >> 24, Personnel taking
        // 0x30..0x3f; a loaded character's id is its MJCHARSD or MNCHARSD
        // index alone (load_game_data), so its entry takes the file's
        // family (MJCHARSD 0x30, MNCHARSD 0x38) and keeps the index.
        for is_alliance in [true, false] {
            for (raw, is_major, family) in [(0x242, true, 0x30), (0x340, false, 0x38)] {
                let mut world = GameWorld::default();
                let key = world.characters.insert(Character {
                    dat_id: DatId::new(raw),
                    is_alliance,
                    is_empire: !is_alliance,
                    is_major,
                    ..Character::default()
                });
                let unit = character(&world, key).expect("the character exists");
                assert_eq!(
                    (unit.family(), unit.dat_id.index(), unit.major),
                    (family, raw, is_major),
                    "{is_alliance} {raw:#x}"
                );
            }
        }
    }

    #[test]
    fn loaded_ships_and_squadrons_report_under_their_files_families() {
        // FUN_0044e500.c:178 sorts the tabs by id >> 24 (Capital Ships
        // 0x14..0x1b, Squadrons 0x1c..0x1f); a loaded CAPSHPSD or FIGHTSD
        // id is the bare record id (load_game_data), so the entry takes the
        // record's family (CAPSHPSD family_id, the Death Star 0x18) or
        // FIGHTSD's 0x1c and keeps the index.
        for (alliance, cruiser_id, wing_id) in [(true, 0x40, 0x1), (false, 0x80, 0x5)] {
            let mut world = GameWorld::default();
            let cruiser = world.capital_ship_classes.insert(CapitalShipClass {
                dat_id: DatId::new(cruiser_id),
                family: 0x14,
                ..CapitalShipClass::default()
            });
            let station = world.capital_ship_classes.insert(CapitalShipClass {
                dat_id: DatId::new(0x8f),
                family: 0x18,
                ..CapitalShipClass::default()
            });
            let wing = world.fighter_classes.insert(FighterClass {
                dat_id: DatId::new(wing_id),
                ..FighterClass::default()
            });
            let ship = |class| capital_ship(&world, &ShipInstance::new(class, 10, alliance));

            let unit = ship(cruiser).expect("the class exists");
            assert_eq!((unit.family(), unit.dat_id.index()), (0x14, cruiser_id));
            let unit = ship(station).expect("the class exists");
            assert_eq!((unit.family(), unit.dat_id.index()), (0x18, 0x8f));
            let unit = squadron(&world, wing).expect("the class exists");
            assert_eq!((unit.family(), unit.dat_id.index()), (0x1c, wing_id));
        }
    }
}
