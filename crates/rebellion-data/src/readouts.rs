//! State readouts for the developer commands and the playtest REPL: plain
//! text for `List characters`, `List fleets`, `List missions`, `Show research`,
//! `List production`, `Show side` and `Show system`.
//!
//! Each function reads the world and its states and changes nothing. They
//! see through fog of war: they are dev truth, not the player's view. Rows
//! are one per line, so the result of a command carries them joined by `\n`.

use rebellion_core::dat::Faction;
use rebellion_core::manufacturing::{
    BuildableKind, Destination, ManufacturingState, ProductionArea, QueueItem,
};
use rebellion_core::missions::{
    member_skill, ActiveMission, MissionFaction, MissionKind, MissionMember, MissionState,
    PHASE_END, PHASE_OUTCOME, PHASE_TIMER, PHASE_TRANSIT,
};
use rebellion_core::movement::MovementState;
use rebellion_core::research::{ResearchState, ResearchSystem, TechType};
use rebellion_core::resources::{maintenance_capacity, maintenance_used};
use rebellion_core::stockpiles::StockpileState;
use rebellion_core::world::{Character, GameWorld, Skill};

/// The side a `[alliance|empire]` argument names; `None` for no argument.
///
/// # Errors
/// Returns the usage text for any other word.
pub fn parse_side(text: &str) -> Result<Option<Faction>, String> {
    match text.trim().to_ascii_lowercase().as_str() {
        "" => Ok(None),
        "alliance" => Ok(Some(Faction::Alliance)),
        "empire" => Ok(Some(Faction::Empire)),
        other => Err(format!("{other:?} is not a side: use alliance or empire")),
    }
}

fn side_name(side: Faction) -> &'static str {
    match side {
        Faction::Alliance => "Alliance",
        Faction::Empire => "Empire",
        Faction::Neutral => "Neutral",
    }
}

/// A list's first row: its count, the side it is limited to, and what it lists.
fn heading(count: usize, side: Option<Faction>, noun: &str) -> String {
    side.map_or_else(
        || format!("{count} {noun}"),
        |side| format!("{count} {} {noun}", side_name(side)),
    )
}

/// A kind's name; `MissionKind::name` is empty for Autoscrap.
fn kind_label(kind: MissionKind) -> &'static str {
    match kind.name() {
        "" => "Autoscrap",
        name => name,
    }
}

fn system_name(world: &GameWorld, system: rebellion_core::ids::SystemKey) -> &str {
    world.systems.get(system).map_or("?", |s| s.name.as_str())
}

const fn phase_label(phase: u8) -> &'static str {
    match phase {
        PHASE_TRANSIT => "transit",
        PHASE_TIMER => "timer",
        PHASE_OUTCOME => "outcome",
        PHASE_END => "end",
        _ => "step",
    }
}

fn member_name(world: &GameWorld, member: MissionMember) -> String {
    match member {
        MissionMember::Character(key) => world
            .characters
            .get(key)
            .map_or_else(|| "?".to_owned(), |c| c.display_name().to_owned()),
        MissionMember::SpecialForce(key) => world
            .special_forces
            .get(key)
            .and_then(|unit| world.buildable_classes.get(&unit.class_dat_id))
            .map_or_else(|| "special force".to_owned(), |class| class.name.clone()),
    }
}

fn member_list(world: &GameWorld, members: &[MissionMember]) -> String {
    members
        .iter()
        .map(|&member| member_name(world, member))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Whether `character` belongs to `side` by allegiance (a captive keeps its own).
fn character_is_side(character: &Character, side: Faction) -> bool {
    match side {
        Faction::Alliance => character.is_alliance,
        Faction::Empire => character.is_empire,
        Faction::Neutral => !character.is_alliance && !character.is_empire,
    }
}

/// Where a fleet is: `at <system>`, or `in transit <from> to <to> (n days)`.
fn fleet_location(
    world: &GameWorld,
    movement: &MovementState,
    fleet: rebellion_core::ids::FleetKey,
) -> String {
    if let Some(order) = movement.orders().get(&fleet) {
        return format!(
            "in transit {} to {} ({} days)",
            system_name(world, order.origin),
            system_name(world, order.destination),
            order.ticks_remaining()
        );
    }
    let at = world
        .fleets
        .get(fleet)
        .map_or("?", |f| system_name(world, f.location));
    format!("at {at}")
}

/// Every fleet, or one side's, one row each: its name and side, where it is
/// (`at <system>`, `in transit <from> to <to> (n days)`), its live capital
/// ships, squadrons and characters, whether it carries the Death Star and
/// whether it is flagged to bombard. First row: the count.
#[must_use]
pub fn list_fleets(world: &GameWorld, movement: &MovementState, side: Option<Faction>) -> String {
    let mut rows: Vec<(String, String)> = world
        .fleets
        .iter()
        .filter(|(_, fleet)| {
            side.is_none_or(|side| Faction::of_alliance(fleet.is_alliance) == side)
        })
        .map(|(key, fleet)| {
            let side = side_name(Faction::of_alliance(fleet.is_alliance));
            let name = world.fleet_name(key).unwrap_or("?");
            let row = format!(
                "{name} {side} {} ships {} squadrons {} characters {} death star {} bombarding {}",
                fleet_location(world, movement, key),
                fleet.capital_ships.iter().filter(|ship| ship.alive).count(),
                fleet.fighters.iter().map(|entry| entry.count).sum::<u32>(),
                fleet.characters.len(),
                fleet.has_death_star(&world.capital_ship_classes),
                fleet.bombarding,
            );
            (format!("{side}{name}"), row)
        })
        .collect();
    rows.sort();
    let mut out = vec![heading(rows.len(), side, "fleets")];
    out.extend(rows.into_iter().map(|(_, row)| row));
    out.join("\n")
}

/// Where a character stands: a fleet and its system, a system, in transit,
/// captive, or not placed.
fn character_location(
    world: &GameWorld,
    movement: &MovementState,
    character: &Character,
) -> String {
    if character.is_killed {
        return "dead".to_owned();
    }
    if let Some(fleet) = character.current_fleet {
        return format!(
            "{} {}",
            world.fleet_name(fleet).unwrap_or("?"),
            fleet_location(world, movement, fleet)
        );
    }
    match character.current_system {
        Some(system) if character.en_route => {
            format!("en route to {}", system_name(world, system))
        }
        Some(system) => format!("at {}", system_name(world, system)),
        None if character.recruited => "not placed".to_owned(),
        None => "not placed (recruit pool)".to_owned(),
    }
}

fn character_status(character: &Character) -> String {
    if character.is_killed {
        "killed".to_owned()
    } else if character.is_captive {
        format!(
            "captive of {}",
            character.captured_by.map_or("?", side_name)
        )
    } else {
        "free".to_owned()
    }
}

/// Every character, or one side's: name and `#<dat id>`, side, location,
/// current mission, the skills that gate missions (Diplomacy, Espionage,
/// Combat, Leadership for the kinds, ship, troop and facility design for
/// Research), officer rank and status. The port keeps no health or injury
/// for a character: status is free, captive or killed.
#[must_use]
pub fn list_characters(
    world: &GameWorld,
    missions: &MissionState,
    movement: &MovementState,
    side: Option<Faction>,
) -> String {
    let mut rows: Vec<(String, String)> = world
        .characters
        .iter()
        .filter(|(_, c)| side.is_none_or(|side| character_is_side(c, side)))
        .map(|(key, c)| {
            let allegiance = if c.is_alliance {
                "Alliance"
            } else if c.is_empire {
                "Empire"
            } else {
                "Neutral"
            };
            let mission = missions
                .missions()
                .iter()
                .find(|m| {
                    m.members()
                        .any(|member| member == MissionMember::Character(key))
                })
                .map_or_else(
                    || "none".to_owned(),
                    |m| {
                        format!(
                            "{} at {} (#{} {})",
                            kind_label(m.kind),
                            system_name(world, m.target_system),
                            m.id,
                            phase_label(m.phase)
                        )
                    },
                );
            let skill = |label: &str, skill: Skill| {
                format!(
                    "{label} {}",
                    member_skill(world, MissionMember::Character(key), skill).unwrap_or(0)
                )
            };
            let skills = [
                skill("dip", Skill::Diplomacy),
                skill("esp", Skill::Espionage),
                skill("com", Skill::Combat),
                skill("lead", Skill::Leadership),
                skill("ship", Skill::ShipDesign),
                skill("troop", Skill::TroopTraining),
                skill("fac", Skill::FacilityDesign),
            ]
            .join(" ");
            let row = format!(
                "{} #{} {}{} | {} | mission {} | {} | rank {:?} | {}",
                c.display_name(),
                c.dat_id.raw(),
                allegiance,
                if c.is_major { " major" } else { "" },
                character_location(world, movement, c),
                mission,
                skills,
                c.command_kind,
                character_status(c)
            );
            (format!("{allegiance}{}", c.name), row)
        })
        .collect();
    rows.sort();
    let mut out = vec![heading(rows.len(), side, "characters")];
    out.extend(rows.into_iter().map(|(_, row)| row));
    out.join("\n")
}

fn mission_row(world: &GameWorld, missions: &MissionState, tick: u64, m: &ActiveMission) -> String {
    let days = m
        .days_left(tick, missions.last_arrival(m.id))
        .map_or_else(|| "-".to_owned(), |days| days.to_string());
    let decoys = if m.decoys.is_empty() {
        String::new()
    } else {
        format!(" decoys [{}]", member_list(world, &m.decoys))
    };
    format!(
        "#{} {:?} {} team [{}]{decoys} target {} phase {} ({}) days left {days}",
        m.id,
        m.faction,
        kind_label(m.kind),
        member_list(world, &m.team),
        system_name(world, m.target_system),
        m.phase,
        phase_label(m.phase),
    )
}

/// The active missions, or one side's: kind, team, decoys, target, phase and
/// days left in the transit or timer wait.
#[must_use]
pub fn list_missions(
    world: &GameWorld,
    missions: &MissionState,
    tick: u64,
    side: Option<Faction>,
) -> String {
    let rows: Vec<String> = missions
        .missions()
        .iter()
        .filter(|m| {
            side.is_none_or(|side| {
                (m.faction == MissionFaction::Alliance) == (side == Faction::Alliance)
            })
        })
        .map(|m| mission_row(world, missions, tick, m))
        .collect();
    let mut out = vec![heading(rows.len(), side, "missions")];
    out.extend(rows);
    out.join("\n")
}

/// Each side's three research trees: level, points held, and the next class's
/// order and cost in points (`ResearchSystem::next_order`, `scaled_cost`).
#[must_use]
pub fn show_research(world: &GameWorld, research: &ResearchState) -> String {
    let mut rows = Vec::new();
    for side in [Faction::Alliance, Faction::Empire] {
        let alliance = side == Faction::Alliance;
        let levels = research.side(alliance);
        for tech in TechType::ALL {
            let tree = match tech {
                TechType::Ship => "ship",
                TechType::Troop => "troop",
                TechType::Facility => "facility",
            };
            let level = levels.level(tech);
            let next = ResearchSystem::next_order(world, alliance, tech, level).map_or_else(
                || "none (tree exhausted)".to_owned(),
                |(order, difficulty)| {
                    format!(
                        "order {order} cost {}",
                        ResearchSystem::scaled_cost(world, difficulty)
                    )
                },
            );
            rows.push(format!(
                "{} {tree} level {level} points {} next {next}",
                side_name(side),
                levels.points(tech)
            ));
        }
    }
    rows.join("\n")
}

const fn area_label(area: ProductionArea) -> &'static str {
    match area {
        ProductionArea::Shipyard => "shipyard",
        ProductionArea::TrainingFacility => "training",
        ProductionArea::ConstructionYard => "construction",
    }
}

/// A product's class name, as `List production` prints it.
#[must_use]
pub fn product_name(world: &GameWorld, kind: BuildableKind) -> String {
    let named = match kind {
        BuildableKind::CapitalShip(key) => world.capital_ship_classes.get(key).map(|c| &c.name),
        BuildableKind::Fighter(key) => world.fighter_classes.get(key).map(|c| &c.name),
        _ => kind
            .class_dat_id()
            .and_then(|class| world.buildable_classes.get(&class))
            .map(|c| &c.name),
    };
    named
        .filter(|name| !name.is_empty())
        .cloned()
        .or_else(|| {
            kind.class_dat_id()
                .map(|class| format!("class #{}", class.raw()))
        })
        .unwrap_or_else(|| "?".to_owned())
}

fn destination_label(world: &GameWorld, destination: Destination) -> String {
    match destination {
        Destination::System(system) => system_name(world, system).to_owned(),
        Destination::Fleet(fleet) => {
            let at = world
                .fleets
                .get(fleet)
                .map_or("?", |f| system_name(world, f.location));
            format!("{} at {at}", world.fleet_name(fleet).unwrap_or("?"))
        }
    }
}

fn queue_row(
    world: &GameWorld,
    index: usize,
    item: &QueueItem,
    eta: Option<u64>,
    area_destination: Option<Destination>,
) -> String {
    let destination = item
        .destination
        .or(area_destination)
        .map_or_else(|| "stays".to_owned(), |d| destination_label(world, d));
    let eta = eta.map_or_else(|| "no yard".to_owned(), |day| format!("day {day}"));
    format!(
        "  [{}] {} {}/{} eta {eta} -> {destination}",
        index + 1,
        product_name(world, item.kind),
        item.work_done,
        item.total_cost
    )
}

/// Every production queue, or those at one side's systems: per system and
/// area, each item with its progress, the day it completes with the yards
/// working and materials on hand (the Status window's best case,
/// `ProductionQueue::completion_days`) and its destination.
#[must_use]
pub fn list_production(
    world: &GameWorld,
    manufacturing: &ManufacturingState,
    tick: u64,
    side: Option<Faction>,
) -> String {
    let mut systems: Vec<_> = world
        .systems
        .iter()
        .filter(|(_, s)| side.is_none_or(|side| s.control.is_controlled_by(side)))
        .map(|(key, s)| (s.name.clone(), key))
        .collect();
    systems.sort();
    let mut out = Vec::new();
    for (name, system) in systems {
        let holder = world.systems[system].control.faction();
        for area in ProductionArea::ALL {
            let Some(queue) = manufacturing.queue(system, area).filter(|q| !q.is_empty()) else {
                continue;
            };
            let periods = holder.map_or_else(Vec::new, |holder| {
                rebellion_core::build_selection::yard_periods(
                    world,
                    system,
                    area,
                    holder == Faction::Alliance,
                )
            });
            let etas = queue.completion_days(tick, &periods);
            let side = holder.map_or("-", |h| if h == Faction::Alliance { "A" } else { "E" });
            out.push(format!(
                "{name} ({side}) {}: {} items",
                area_label(area),
                queue.len()
            ));
            for (index, item) in queue.items().iter().enumerate() {
                out.push(queue_row(
                    world,
                    index,
                    item,
                    etas.get(index).copied().flatten(),
                    manufacturing.destination(system, area),
                ));
            }
        }
    }
    if out.is_empty() {
        out.push("no production queues".to_owned());
    }
    out.join("\n")
}

/// One system as a bombardment reads it, for checking a strike's deltas:
/// holder, support points, energy and raw material, the HQ and destroyed
/// flags, then per side the regiments, squadrons and facilities the system
/// holds (defence facilities by DAT family), then each fleet with its
/// bombard strength (`+0x1dc`), each ship's hull and its flag.
///
/// # Errors
/// Returns `no system named …` for an unknown name.
pub fn show_system(world: &GameWorld, name: &str) -> Result<String, String> {
    let (key, system) = world
        .systems
        .iter()
        .find(|(_, system)| system.name.eq_ignore_ascii_case(name.trim()))
        .ok_or_else(|| format!("no system named {name:?}"))?;
    let mut rows = vec![format!(
        "{} holder {} support {}/{} energy {} raw {} hq {} destroyed {}",
        system.name,
        side_name(rebellion_core::bombardment::holder(world, key)),
        rebellion_core::uprising::support_points(system, Faction::Alliance),
        rebellion_core::uprising::support_points(system, Faction::Empire),
        system.total_energy,
        system.raw_materials,
        system.is_headquarters,
        system.is_destroyed,
    )];
    for side in [Faction::Alliance, Faction::Empire] {
        let alliance = side == Faction::Alliance;
        let regiments = system
            .ground_units
            .iter()
            .filter_map(|&troop| world.troops.get(troop))
            .filter(|troop| troop.is_alliance == alliance)
            .count();
        // Each defence facility by DAT family: 0x22 KDY, 0x23 LNR, 0x24
        // shield generator (DEFFACSD).
        let families: Vec<String> = system
            .defense_facilities
            .iter()
            .filter_map(|&facility| world.defense_facilities.get(facility))
            .filter(|facility| facility.side == side)
            .map(|facility| format!("{:#x}", facility.class_dat_id.family()))
            .collect();
        let defense = families.len();
        let manufacturing = system
            .manufacturing_facilities
            .iter()
            .filter_map(|&facility| world.manufacturing_facilities.get(facility))
            .filter(|facility| facility.side == side)
            .count();
        let production = system
            .production_facilities
            .iter()
            .filter_map(|&facility| world.production_facilities.get(facility))
            .filter(|facility| facility.side == side)
            .count();
        rows.push(format!(
            "{} regiments {regiments} squadrons {} defense {defense} [{}] manufacturing \
             {manufacturing} production {production}",
            side_name(side),
            system.squadron_count(alliance),
            families.join(" "),
        ));
    }
    for &fleet in &system.fleets {
        let Some(value) = world.fleets.get(fleet) else {
            continue;
        };
        let hull: Vec<String> = value
            .capital_ships
            .iter()
            .map(|ship| {
                let max = world
                    .capital_ship_classes
                    .get(ship.class)
                    .map_or(0, |class| class.hull);
                format!("{}/{max}", ship.hull_current)
            })
            .collect();
        rows.push(format!(
            "fleet {} {} strength {} ships {} hull [{}] bombarding {}",
            world.fleet_name(fleet).unwrap_or_default(),
            if value.is_alliance {
                "Alliance"
            } else {
                "Empire"
            },
            rebellion_core::bombardment::fleet_strength(world, fleet),
            value.capital_ships.len(),
            hull.join(" "),
            value.bombarding,
        ));
    }
    Ok(rows.join("\n"))
}

/// One side's top strip and counts: raw and refined stock, maintenance
/// capacity and use, systems held, fleets, characters and active missions.
#[must_use]
pub fn show_side(
    world: &GameWorld,
    stockpiles: &StockpileState,
    manufacturing: &ManufacturingState,
    missions: &MissionState,
    side: Faction,
) -> String {
    let alliance = side == Faction::Alliance;
    let stock = stockpiles.side(side);
    let systems = world
        .systems
        .values()
        .filter(|s| s.control.is_controlled_by(side))
        .count();
    let fleets = world
        .fleets
        .values()
        .filter(|f| f.is_alliance == alliance)
        .count();
    let characters = world
        .characters
        .values()
        .filter(|c| !c.is_killed && character_is_side(c, side))
        .count();
    let active = missions
        .missions()
        .iter()
        .filter(|m| (m.faction == MissionFaction::Alliance) == alliance)
        .count();
    format!(
        "{} raw {} refined {} maintenance used {} of {} systems {systems} fleets {fleets} \
         characters {characters} missions {active}",
        side_name(side),
        stock.raw,
        stock.refined,
        maintenance_used(world, manufacturing, side),
        maintenance_capacity(world, side),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::world::{
        ControlKind, DefenseFacilityInstance, Fleet, Sector, System, TroopUnit,
    };

    #[test]
    fn show_system_counts_each_sides_units_and_names_its_fleets() {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector::default());
        let system = world.systems.insert(System {
            name: "Target".into(),
            sector,
            control: ControlKind::Controlled(Faction::Empire),
            total_energy: 4,
            raw_materials: 3,
            ..System::default()
        });
        for is_alliance in [false, false, true] {
            let troop = world.troops.insert(TroopUnit {
                class_dat_id: rebellion_core::ids::DatId::new(0x1000_0001),
                is_alliance,
            });
            world.systems[system].ground_units.push(troop);
        }
        let battery = world.defense_facilities.insert(DefenseFacilityInstance {
            class_dat_id: rebellion_core::ids::DatId::new(0x2200_0001),
            side: Faction::Empire,
        });
        world.systems[system].defense_facilities.push(battery);
        let yard = world.manufacturing_facilities.insert(
            rebellion_core::world::ManufacturingFacilityInstance {
                side: Faction::Empire,
                ..Default::default()
            },
        );
        world.systems[system].manufacturing_facilities.push(yard);
        let mine =
            world
                .production_facilities
                .insert(rebellion_core::world::ProductionFacilityInstance {
                    side: Faction::Alliance,
                    ..Default::default()
                });
        world.systems[system].production_facilities.push(mine);
        let fleet = world.fleets.insert(Fleet {
            location: system,
            is_alliance: true,
            bombarding: true,
            ..Fleet::default()
        });
        world.systems[system].fleets.push(fleet);

        let text = show_system(&world, "target").unwrap();
        let rows: Vec<&str> = text.lines().collect();

        assert!(rows[0].starts_with("Target holder Empire "), "{text}");
        assert!(rows[0].contains(" energy 4 raw 3 "), "{text}");
        assert!(rows[1].starts_with("Alliance regiments 1 "), "{text}");
        assert!(rows[2].starts_with("Empire regiments 2 "), "{text}");
        assert!(rows[2].contains(" defense 1 [0x22] "), "{text}");
        assert!(rows[1].contains(" defense 0 [] "), "{text}");
        assert!(rows[1].ends_with(" manufacturing 0 production 1"), "{text}");
        assert!(rows[2].ends_with(" manufacturing 1 production 0"), "{text}");
        assert!(
            rows[3].contains(" Alliance strength 0 ships 0 hull [] bombarding true"),
            "{text}"
        );
        assert_eq!(
            show_system(&world, "Nowhere"),
            Err("no system named \"Nowhere\"".into())
        );
    }

    #[test]
    fn list_characters_says_where_each_character_stands() {
        let mut world = GameWorld::default();
        let mut at = |name: &str| {
            world.systems.insert(System {
                name: name.into(),
                ..System::default()
            })
        };
        let (home, away) = (at("Home"), at("Away"));
        let parked = world.fleets.insert(Fleet {
            location: home,
            ..Fleet::default()
        });
        let moving = world.fleets.insert(Fleet {
            location: home,
            ..Fleet::default()
        });
        let mut movement = MovementState::new();
        assert!(movement.order(moving, home, away, 4));
        for (name, character) in [
            (
                "Aboard",
                Character {
                    current_fleet: Some(parked),
                    ..Character::default()
                },
            ),
            (
                "Flying",
                Character {
                    current_fleet: Some(moving),
                    ..Character::default()
                },
            ),
            (
                "Walking",
                Character {
                    current_system: Some(away),
                    en_route: true,
                    ..Character::default()
                },
            ),
            (
                "Standing",
                Character {
                    current_system: Some(home),
                    ..Character::default()
                },
            ),
            (
                "Hired",
                Character {
                    recruited: true,
                    ..Character::default()
                },
            ),
            ("Pooled", Character::default()),
            (
                "Fallen",
                Character {
                    current_system: Some(home),
                    is_killed: true,
                    ..Character::default()
                },
            ),
        ] {
            world.characters.insert(Character {
                name: name.into(),
                is_empire: true,
                ..character
            });
        }
        let text = list_characters(&world, &MissionState::new(), &movement, None);
        let location = |name: &str| {
            text.lines()
                .find(|row| row.starts_with(&format!("{name} #")))
                .and_then(|row| row.split(" | ").nth(1))
                .unwrap_or_else(|| panic!("no {name} in {text}"))
                .to_owned()
        };
        assert_eq!(location("Aboard"), "Fleet at Home");
        assert_eq!(location("Flying"), "Fleet in transit Home to Away (4 days)");
        assert_eq!(location("Walking"), "en route to Away");
        assert_eq!(location("Standing"), "at Home");
        assert_eq!(location("Hired"), "not placed");
        assert_eq!(location("Pooled"), "not placed (recruit pool)");
        assert_eq!(location("Fallen"), "dead");
    }

    #[test]
    fn list_fleets_names_each_fleets_side_and_where_it_is() {
        let mut world = GameWorld::default();
        let mut at = |name: &str| {
            world.systems.insert(System {
                name: name.into(),
                ..System::default()
            })
        };
        let (home, away) = (at("Home"), at("Away"));
        let class = world
            .capital_ship_classes
            .insert(rebellion_core::world::CapitalShipClass::default());
        let mut dead = rebellion_core::world::ShipInstance::new(class, 0, false);
        dead.alive = false;
        world.fleets.insert(Fleet {
            location: home,
            capital_ships: vec![
                rebellion_core::world::ShipInstance::new(class, 10, false),
                dead,
            ],
            bombarding: true,
            ..Fleet::default()
        });
        let rebel = world.fleets.insert(Fleet {
            location: home,
            is_alliance: true,
            ..Fleet::default()
        });
        let mut movement = MovementState::new();
        assert!(movement.order(rebel, home, away, 4));

        let text = list_fleets(&world, &movement, None);
        let rows: Vec<&str> = text.lines().collect();
        assert_eq!(rows[0], "2 fleets", "{text}");
        assert_eq!(
            rows[1],
            "Fleet Alliance in transit Home to Away (4 days) ships 0 squadrons 0 characters 0 \
             death star false bombarding false"
        );
        assert_eq!(
            rows[2],
            "Fleet Empire at Home ships 1 squadrons 0 characters 0 death star false \
             bombarding true"
        );
        let empire = list_fleets(&world, &movement, Some(Faction::Empire));
        assert_eq!(
            empire.lines().collect::<Vec<_>>(),
            ["1 Empire fleets", rows[2]]
        );
    }
}
