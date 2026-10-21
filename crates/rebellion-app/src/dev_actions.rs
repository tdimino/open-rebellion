//! The world-changing developer commands: `Post message`, `Battle at`,
//! `Blockade`, `Give Death Star`, `Damage ship`, the strike orders for the
//! AI's side, `Mission:`, `Recruit:` and `Jail:`. Each one goes through the production
//! code that the event it stands for would run (the message builders,
//! `apply_fleet_arrival` as the
//! app's own arrival step calls it, the tick's own combat and blockade
//! checks), so what it shows is the game's
//! behaviour. A command whose precondition fails is refused with the reason
//! and is never faked. `Maintenance capacity`, `Strip defenders`,
//! `Embark` and `Queue build` stand for no event: they are staging alone.
//! `agent_docs/dev-commands.md` lists them.

use rebellion_core::ai::AIAction;
use rebellion_core::ai_regions::Strike;
use rebellion_core::bombardment::BombardKind;
use rebellion_core::dat::Faction;
use rebellion_core::ids::{CharacterKey, FleetKey, SystemKey};
use rebellion_core::missions::{
    available_kinds, recruit_character, recruit_pool, MissionFaction, MissionKind, MissionMember,
    MissionRequest, MissionState, MissionTarget,
};
use rebellion_core::movement::{apply_fleet_arrival, ArrivalEvent, MovementState};
use rebellion_core::troop_transport::TroopTransportState;
use rebellion_core::uprising::UprisingState;
use rebellion_core::world::GameWorld;
use rebellion_data::integrator::{resolve_ai_strike, AiOutcome, StrikeEffects, StrikeStates};
use rebellion_data::text_templates::TextTemplates;
use rebellion_render::message_log::{GameMessage, MessageLog};

use crate::dev_commands::MessageClass;

/// The system named `name`, ignoring case.
pub fn find_system(world: &GameWorld, name: &str) -> Result<SystemKey, String> {
    world
        .systems
        .iter()
        .find(|(_, system)| system.name.eq_ignore_ascii_case(name.trim()))
        .map(|(key, _)| key)
        .ok_or_else(|| format!("no system named {name:?}"))
}

/// Every system by name: its holder (`A`, `E`, `-` for neither) and, after
/// a slash, the sides with fleets there; for choosing a `Battle at` or
/// `Blockade` target. A system with squadrons adds the counts the system
/// holds and its fleets carry, Alliance then Empire; a populated one adds
/// each side's support in points.
pub fn list_systems(world: &GameWorld) -> String {
    let mut rows: Vec<String> = world
        .systems
        .iter()
        .map(|(key, system)| {
            let holder = match holder_is_alliance(world, key) {
                Some(true) => "A",
                Some(false) => "E",
                None => "-",
            };
            let fleets = match (
                side_has_fleet(world, key, true),
                side_has_fleet(world, key, false),
            ) {
                (true, true) => "AE",
                (true, false) => "A",
                (false, true) => "E",
                (false, false) => "",
            };
            let aboard = |is_alliance: bool| -> u32 {
                system
                    .fleets
                    .iter()
                    .filter_map(|&fleet| world.fleets.get(fleet))
                    .filter(|fleet| fleet.is_alliance == is_alliance)
                    .flat_map(|fleet| &fleet.fighters)
                    .map(|entry| entry.count)
                    .sum()
            };
            let counts = [
                system.squadron_count(true),
                system.squadron_count(false),
                aboard(true),
                aboard(false),
            ];
            let mut row = format!("{} {holder}/{fleets}", system.name);
            if counts.iter().any(|&count| count > 0) {
                row.push_str(&format!(
                    " squadrons {}/{} fleets {}/{}",
                    counts[0], counts[1], counts[2], counts[3]
                ));
            }
            let yards = system
                .manufacturing_facilities
                .iter()
                .filter_map(|&facility| world.manufacturing_facilities.get(facility))
                .filter(|facility| facility.is_shipyard)
                .count();
            if yards > 0 {
                row.push_str(&format!(" yards {yards}"));
            }
            if system.is_populated {
                row.push_str(&format!(
                    " support {}/{}",
                    rebellion_core::uprising::support_points(system, Faction::Alliance),
                    rebellion_core::uprising::support_points(system, Faction::Empire)
                ));
            }
            row
        })
        .collect();
    rows.sort();
    rows.join(", ")
}

/// The side holding `system`, or `None` when no single side does.
fn holder_is_alliance(world: &GameWorld, system: SystemKey) -> Option<bool> {
    match world.systems.get(system)?.control.faction() {
        Some(Faction::Alliance) => Some(true),
        Some(Faction::Empire) => Some(false),
        _ => None,
    }
}

/// File a message class at `system` through its builder. Refused when the
/// builder would file nothing on a rail (a neutral system, no blockading
/// fleet, no fleet to arrive).
pub fn post_message(
    world: &GameWorld,
    templates: &TextTemplates,
    log: &mut MessageLog,
    class: MessageClass,
    system: SystemKey,
    tick: u64,
    player_is_alliance: bool,
) -> Result<String, String> {
    let name = world.systems[system].name.clone();
    let holder = holder_is_alliance(world, system);
    let messages: Vec<GameMessage> = match class {
        MessageClass::UprisingBegan | MessageClass::UprisingEnded => crate::uprising_messages(
            world,
            templates,
            system,
            tick,
            class == MessageClass::UprisingBegan,
            String::new(),
        ),
        MessageClass::Blockade => crate::blockade_messages(world, templates, system, tick),
        MessageClass::FleetArrival => {
            let (fleet, alliance) = fleet_at(world, system, player_is_alliance)
                .ok_or_else(|| format!("no fleet orbits {name} to arrive"))?;
            let fleet_name = world.fleet_name(fleet).unwrap_or_default().to_owned();
            vec![crate::fleet_arrival_message(
                templates,
                fleet,
                &fleet_name,
                system,
                &name,
                tick,
                alliance,
            )]
        }
        MessageClass::LoyaltyJoins => {
            let holder = holder.ok_or_else(|| format!("{name} has no holder to join"))?;
            crate::loyalty_messages(world, templates, system, tick, Some(holder), None)
        }
        MessageClass::LoyaltyNeutral => {
            let holder = holder.ok_or_else(|| format!("{name} has no holder to leave"))?;
            crate::loyalty_messages(world, templates, system, tick, None, Some(holder))
        }
    };
    let filed: Vec<GameMessage> = messages
        .into_iter()
        .filter(|message| message.rail.is_some() && message.display.is_some())
        .collect();
    if filed.is_empty() {
        return Err(format!(
            "{class:?} at {name} files nothing in the original (neutral system or no fleet)"
        ));
    }
    let titles: Vec<String> = filed
        .iter()
        .filter_map(|message| message.display.as_ref().map(|d| d.title.clone()))
        .collect();
    for message in filed {
        log.push(message);
    }
    Ok(format!("filed {}", titles.join(" | ")))
}

/// A fleet orbiting `system` and its side: the player's first, since only
/// the arriving side files the message.
fn fleet_at(
    world: &GameWorld,
    system: SystemKey,
    player_is_alliance: bool,
) -> Option<(FleetKey, bool)> {
    let fleets: Vec<(FleetKey, bool)> = world
        .systems
        .get(system)?
        .fleets
        .iter()
        .filter_map(|&key| world.fleets.get(key).map(|fleet| (key, fleet.is_alliance)))
        .collect();
    fleets
        .iter()
        .find(|(_, alliance)| *alliance == player_is_alliance)
        .or_else(|| fleets.first())
        .copied()
}

/// Bring one idle fleet of each side missing from `system` there.
pub fn battle_at(
    world: &mut GameWorld,
    movement: &MovementState,
    troop_transport: &mut TroopTransportState,
    system: SystemKey,
    tick: u64,
) -> Result<String, String> {
    let mut moved = Vec::new();
    for alliance in [true, false] {
        if let Some(note) = bring_side(world, movement, troop_transport, system, alliance, tick)? {
            moved.push(note);
        }
    }
    let held = match holder_is_alliance(world, system) {
        Some(true) => "held by the Alliance",
        Some(false) => "held by the Empire",
        None => "held by neither side",
    };
    Ok(if moved.is_empty() {
        format!("{held}; both sides already orbit it")
    } else {
        format!("{held}; {}", moved.join("; "))
    })
}

/// Bring one idle fleet of the side not holding `system` there, so the
/// simulation forms a blockade. Refused for a neutral system, or where the
/// holder has a fleet (that is a battle, not a blockade).
pub fn blockade(
    world: &mut GameWorld,
    movement: &MovementState,
    troop_transport: &mut TroopTransportState,
    system: SystemKey,
    tick: u64,
) -> Result<String, String> {
    let name = world.systems[system].name.clone();
    let holder = holder_is_alliance(world, system)
        .ok_or_else(|| format!("{name} has no holder to blockade"))?;
    if side_has_fleet(world, system, holder) {
        return Err(format!(
            "the holder has a fleet at {name}; that is a battle"
        ));
    }
    Ok(
        bring_side(world, movement, troop_transport, system, !holder, tick)?
            .unwrap_or_else(|| "the blockader already orbits it".into()),
    )
}

fn side_has_fleet(world: &GameWorld, system: SystemKey, alliance: bool) -> bool {
    world.systems[system].fleets.iter().any(|&key| {
        world
            .fleets
            .get(key)
            .is_some_and(|f| f.is_alliance == alliance)
    })
}

/// Move one idle fleet of a side to `system` through `apply_fleet_arrival`:
/// one with capital ships, not in transit, carrying no Death Star (its
/// battle is a separate path). `None` when the side is already there.
fn bring_side(
    world: &mut GameWorld,
    movement: &MovementState,
    troop_transport: &mut TroopTransportState,
    system: SystemKey,
    alliance: bool,
    tick: u64,
) -> Result<Option<String>, String> {
    if side_has_fleet(world, system, alliance) {
        return Ok(None);
    }
    let side = if alliance { "Alliance" } else { "Imperial" };
    let mut idle: Vec<(FleetKey, String)> = world
        .fleets
        .iter()
        .filter(|(key, fleet)| {
            fleet.is_alliance == alliance
                && !fleet.capital_ships.is_empty()
                && !fleet.has_death_star(&world.capital_ship_classes)
                && !movement.is_in_transit(*key)
        })
        // A fleet whose system is missing is skipped, not indexed.
        .filter_map(|(key, fleet)| {
            Some((key, world.systems.get(fleet.location)?.name.clone()))
        })
        .collect();
    // Order by the fleet's name for a choice that repeats run to run.
    idle.sort_by_key(|(key, _)| world.fleet_name(*key).unwrap_or_default().to_owned());
    let (fleet, origin_name) = idle
        .into_iter()
        .next()
        .ok_or_else(|| format!("no idle {side} fleet with capital ships"))?;
    let origin = world.fleets[fleet].location;
    apply_fleet_arrival(
        world,
        movement,
        troop_transport,
        &ArrivalEvent {
            fleet,
            tick,
            origin,
            system,
            join: None,
        },
    )
    .ok_or("the arrival did not apply")?;
    let fleet_name = world.fleet_name(fleet).unwrap_or_default().to_owned();
    eprintln!(
        "[dev-command] moved {side} {fleet_name} from {origin_name} to {}",
        world.systems[system].name
    );
    Ok(Some(format!(
        "moved {side} {fleet_name} from {origin_name}"
    )))
}

/// Put a Death Star ship (the CAPSHPSD family-0x18 class) in the named
/// Imperial fleet, as a completed build leaves it
/// (`integrator::apply_boarding_inner`).
/// Refused for a name no Imperial fleet has, or while another fleet carries
/// it. The sector's Death Star bit is refreshed at once, as the container
/// change reruns it (`death_star::refresh_sectors`).
pub fn give_death_star(world: &mut GameWorld, name: &str) -> Result<String, String> {
    if let Some(holder) = world
        .fleets
        .iter()
        .find(|(_, fleet)| fleet.has_death_star(&world.capital_ship_classes))
        .map(|(key, _)| key)
    {
        return Err(format!(
            "{} already carries the Death Star",
            world.fleet_name(holder).unwrap_or_default()
        ));
    }
    // Both sides number their fleets, so only Imperial names are searched.
    let fleet = world
        .fleets
        .iter()
        .filter(|(_, fleet)| !fleet.is_alliance)
        .map(|(key, _)| key)
        .find(|&key| {
            world
                .fleet_name(key)
                .is_some_and(|fleet_name| fleet_name.eq_ignore_ascii_case(name.trim()))
        })
        .ok_or_else(|| format!("no Imperial fleet named {name:?}"))?;
    let (class, hull) = world
        .capital_ship_classes
        .iter()
        .find(|(_, class)| class.is_death_star())
        .map(|(key, class)| (key, class.hull.cast_signed()))
        .ok_or("no Death Star class is loaded")?;
    world.fleets[fleet]
        .capital_ships
        .push(rebellion_core::world::ShipInstance::new(class, hull, false));
    world.number_capital_ships();
    rebellion_core::death_star::refresh_sectors(world);
    let system = world
        .systems
        .get(world.fleets[fleet].location)
        .map_or("", |s| s.name.as_str());
    Ok(format!(
        "{} at {system} carries the Death Star",
        world.fleet_name(fleet).unwrap_or_default()
    ))
}

/// Halve the hull of the first live capital ship in the player's fleet
/// named `name`, as a battle's damage import leaves a hit ship
/// (`tactical/apply.rs`). Staging only: the repair timer arms on the next
/// tick through the repair system's own rules, and the damage counters are
/// left as they are.
pub fn damage_ship(world: &mut GameWorld, alliance: bool, name: &str) -> Result<String, String> {
    let fleet = world
        .fleets
        .iter()
        .filter(|(_, fleet)| fleet.is_alliance == alliance)
        .map(|(key, _)| key)
        .find(|&key| {
            world
                .fleet_name(key)
                .is_some_and(|fleet_name| fleet_name.eq_ignore_ascii_case(name.trim()))
        })
        .ok_or_else(|| format!("no fleet of yours named {name:?}"))?;
    let fleet_name = world.fleet_name(fleet).unwrap_or_default().to_owned();
    let system = world
        .systems
        .get(world.fleets[fleet].location)
        .map_or_else(String::new, |s| s.name.clone());
    let ship = world.fleets[fleet]
        .capital_ships
        .iter_mut()
        .find(|ship| ship.alive)
        .ok_or_else(|| format!("{fleet_name} has no capital ship"))?;
    let before = ship.hull_current;
    if before < 2 {
        return Err(format!("{fleet_name}'s first ship has no hull to lose"));
    }
    ship.hull_current = before / 2;
    Ok(format!(
        "{fleet_name} at {system}: first capital ship hull {before} -> {}",
        ship.hull_current
    ))
}

/// The fleet of `side` named `name`, ignoring case. Both sides number their
/// fleets, so the side tells the two `Fleet 1`s apart.
pub fn find_fleet(world: &GameWorld, side: Faction, name: &str) -> Result<FleetKey, String> {
    world
        .fleets
        .iter()
        .filter(|(_, fleet)| Faction::of_alliance(fleet.is_alliance) == side)
        .map(|(key, _)| key)
        .find(|&key| {
            world
                .fleet_name(key)
                .is_some_and(|fleet_name| fleet_name.eq_ignore_ascii_case(name.trim()))
        })
        .ok_or_else(|| {
            let side = if side == Faction::Alliance {
                "Alliance"
            } else {
                "Imperial"
            };
            format!("no {side} fleet named {name:?}")
        })
}

/// Order a strike of `kind` for `fleet` as the AI orders one: its action at
/// the fleet's system (`AIAction::Bombard`, `AIAction::DestroySystem`)
/// through `integrator::resolve_ai_strike`, which runs
/// `integrator::order_strike` without the menu row's enable test
/// (`FUN_00581e80`, which only the menu runs). Refused for Target Civilian
/// Facilities, an order the AI never gives (`ai_regions::Strike`), and with
/// the resolver's refusal.
pub fn ai_strike(
    world: &mut GameWorld,
    fleet: FleetKey,
    kind: BombardKind,
    states: &mut StrikeStates<'_>,
    roll: &mut dyn FnMut() -> f64,
) -> Result<StrikeEffects, String> {
    let system = world
        .fleets
        .get(fleet)
        .map(|value| value.location)
        .ok_or("no such fleet")?;
    let action = match kind {
        BombardKind::Military => AIAction::Bombard {
            fleet,
            system,
            kind: Strike::Military,
        },
        BombardKind::General => AIAction::Bombard {
            fleet,
            system,
            kind: Strike::General,
        },
        BombardKind::DestroySystem => AIAction::DestroySystem { fleet, system },
        BombardKind::Civilian => {
            return Err(format!("{:#x}: the AI never orders it", kind.order()));
        }
    };
    let side = rebellion_core::dat::Faction::of_alliance(world.fleets[fleet].is_alliance);
    match resolve_ai_strike(&action, side, world, states, roll) {
        (AiOutcome::Applied { .. }, strike) => Ok(strike),
        (outcome, _) => Err(format!("{outcome:?}")),
    }
}

/// A strike order's result: `0x<order>: ` and, per run, its counts, or
/// `idle` for a run that did nothing (both sides flagged there) and
/// `in transit, not flagged` for no run. `Err` when no run did anything,
/// since then nothing changed that a script could check.
pub fn strike_detail(kind: BombardKind, runs: &[StrikeEffects]) -> Result<String, String> {
    let detail = if runs.is_empty() {
        "in transit, not flagged".to_owned()
    } else {
        runs.iter()
            .map(|strike| {
                strike.report.as_ref().map_or_else(
                    || "idle".to_owned(),
                    |report| {
                        format!(
                            "hits {} regiments {} facilities {} hq {} energy {} raw {} defence {} \
                             ships lost {} characters {} destroyed {}{}",
                            report.hits,
                            report.regiments.len(),
                            report.facilities.len(),
                            report.headquarters,
                            report.energy,
                            report.raw_material,
                            report.defence_damage,
                            report.ships_destroyed,
                            report.characters.len(),
                            report.planet_destroyed,
                            strike.cascade.as_ref().map_or_else(String::new, |cascade| {
                                format!(
                                    " cascade regiments {} squadrons {} facilities {} special \
                                     forces {} hq {} adrift {} autoscrapped {}",
                                    cascade.regiments.len(),
                                    cascade.squadrons.len(),
                                    cascade.facilities.len(),
                                    cascade.special_forces.len(),
                                    cascade.headquarters,
                                    cascade.adrift.len(),
                                    cascade.autoscrapped.len(),
                                )
                            }),
                        )
                    },
                )
            })
            .collect::<Vec<_>>()
            .join("; ")
    };
    let detail = format!("{:#x}: {detail}", kind.order());
    if runs.iter().any(|strike| strike.report.is_some()) {
        Ok(detail)
    } else {
        Err(detail)
    }
}

/// Destroy every defender the holder of `system` keeps there: its
/// regiments, its own squadrons, its defence facilities and its fleets,
/// through the paths a battle or a strike takes. port: staging only, with
/// no original counterpart; it writes no control, and the regiment kills
/// queue the control rerun (event `0x301`) as any kill does.
pub fn strip_defenders(
    world: &mut GameWorld,
    transport: &mut TroopTransportState,
    system: SystemKey,
    tick: u64,
) -> Result<String, String> {
    let holder = rebellion_core::bombardment::holder(world, system);
    let sys = world.systems.get(system).ok_or("no such system")?;
    let name = sys.name.clone();
    if holder == Faction::Neutral {
        return Err(format!("{name} has no holder"));
    }
    let ours = holder == Faction::Alliance;
    let regiments: Vec<_> = sys
        .ground_units
        .iter()
        .copied()
        .filter(|&key| world.troops.get(key).is_some_and(|unit| unit.is_alliance == ours))
        .collect();
    let facilities: Vec<_> = sys
        .defense_facilities
        .iter()
        .copied()
        .filter(|&key| world.defense_facilities.get(key).is_some_and(|f| f.side == holder))
        .collect();
    let fleets: Vec<_> = sys
        .fleets
        .iter()
        .copied()
        .filter(|&key| world.fleets.get(key).is_some_and(|f| f.is_alliance == ours))
        .collect();
    let squadrons: u32 = sys
        .fighters
        .iter()
        .filter(|entry| entry.is_alliance == ours)
        .map(|entry| entry.count)
        .sum();
    // A regiment's kill queues its system's control rerun (event 0x301).
    for &troop in &regiments {
        rebellion_core::missions::destroy_target(world, MissionTarget::Troop(troop));
    }
    for &facility in &facilities {
        rebellion_core::missions::destroy_target(world, MissionTarget::DefenseFacility(facility));
    }
    world.systems[system]
        .fighters
        .retain(|entry| entry.is_alliance != ours);
    for &fleet in &fleets {
        rebellion_core::post_battle::destroy_fleet(world, transport, fleet, tick);
    }
    rebellion_core::support_shift::regiments_changed(world, system);
    Ok(format!(
        "{name}: {holder:?} lost {} regiments, {squadrons} squadrons, {} defence facilities, {} fleets",
        regiments.len(),
        facilities.len(),
        fleets.len()
    ))
}

/// Load `count` of the player's regiments standing at the system of the
/// player's fleet named `name` onto that fleet, in surface order, each by
/// the order the Defenses window's drag onto a fleet gives: the regiment's
/// Move onto the fleet (`PanelAction::MoveRegiment` ->
/// `TroopTransportState::move_regiment`, `FUN_00578f30` -> `FUN_00556390`,
/// `ghidra/notes/regiment-unload.md`), which boards it on the ship the leg
/// chooses. port: staging only, with no original counterpart: one line for
/// the drags. Refused, with nothing moved, when fewer than `count` of the
/// player's regiments stand there; a regiment the Move refuses stops the
/// line, naming how many boarded.
#[expect(
    clippy::too_many_arguments,
    reason = "The Move reads the world, the transport, movement, the blockades and the products, and the order's fleet, count and day."
)]
pub fn embark(
    world: &mut GameWorld,
    transport: &mut TroopTransportState,
    movement: &MovementState,
    blockaded: &std::collections::HashSet<SystemKey>,
    products: &rebellion_core::carriage::ProductRoom,
    alliance: bool,
    name: &str,
    count: usize,
    tick: u64,
) -> Result<String, String> {
    let fleet = find_fleet(world, Faction::of_alliance(alliance), name)?;
    let fleet_name = world.fleet_name(fleet).unwrap_or_default().to_owned();
    let system = world.fleets[fleet].location;
    let sys = world.systems.get(system).ok_or("the fleet stands at no system")?;
    let system_name = sys.name.clone();
    let regiments: Vec<_> = sys
        .ground_units
        .iter()
        .copied()
        .filter(|&key| world.troops.get(key).is_some_and(|unit| unit.is_alliance == alliance))
        .collect();
    if regiments.len() < count {
        return Err(format!(
            "{fleet_name} at {system_name}: {} of your regiments stand there, {count} asked",
            regiments.len()
        ));
    }
    for (boarded, &troop) in regiments.iter().take(count).enumerate() {
        transport
            .move_regiment(
                world,
                movement,
                blockaded,
                products,
                troop,
                rebellion_core::troop_transport::RegimentTarget::Fleet(fleet),
                tick,
            )
            .map_err(|error| format!("{fleet_name}: boarded {boarded} of {count}: {error:?}"))?;
    }
    Ok(format!(
        "{fleet_name} at {system_name}: boarded {count} regiments, {} aboard",
        transport.cargo(fleet).len()
    ))
}

/// Queue one unit of the class named `class` at `system` for a side,
/// through Build Selection's own checks and enqueue
/// (`build_selection::order_refusal`, `order_items`,
/// `ManufacturingState::build_units`). The class must be one the side's
/// managers list (`listed_classes`). port: staging only, with no original
/// counterpart: it builds for either side, the AI's included.
#[expect(
    clippy::too_many_arguments,
    reason = "The order reads the research, the queues, the cargo, the products and the fleets on their way."
)]
pub fn queue_build(
    world: &mut GameWorld,
    research: &rebellion_core::research::ResearchState,
    manufacturing: &mut rebellion_core::manufacturing::ManufacturingState,
    transport: &rebellion_core::troop_transport::TroopTransportState,
    deliveries: &rebellion_core::delivery::DeliveryState,
    movement: &rebellion_core::movement::MovementState,
    alliance: bool,
    class: &str,
    system: SystemKey,
) -> Result<String, String> {
    use rebellion_core::build_selection::{listed_classes, order_items, order_refusal};
    use rebellion_core::manufacturing::ProductionArea;
    let side = Faction::of_alliance(alliance);
    let name = world.systems.get(system).ok_or("no such system")?.name.clone();
    if rebellion_core::bombardment::holder(world, system) != side {
        return Err(format!("{name} is not held by the {side:?}"));
    }
    let (area, kind) = [
        ProductionArea::Shipyard,
        ProductionArea::TrainingFacility,
        ProductionArea::ConstructionYard,
    ]
    .into_iter()
    .flat_map(|area| {
        listed_classes(world, research, area, alliance)
            .into_iter()
            .map(move |kind| (area, kind))
    })
    .find(|&(_, kind)| {
        rebellion_data::readouts::product_name(world, kind).eq_ignore_ascii_case(class.trim())
    })
    .ok_or_else(|| format!("the {side:?} lists no class named {class:?}"))?;
    if let Some(refusal) = order_refusal(
        world,
        manufacturing,
        transport,
        deliveries,
        system,
        area,
        kind,
        1,
        alliance,
        movement,
    ) {
        return Err(format!("{refusal:?} ({:?})", refusal.status()));
    }
    let items = order_items(world, system, area, kind, 1, alliance)
        .ok_or_else(|| format!("{name} has no {area:?} to build it"))?;
    manufacturing.build_units(system, items);
    rebellion_core::carriage::seat_products(world, transport, deliveries, manufacturing);
    rebellion_core::manufacturing::seat_ordered_ships(world, manufacturing);
    Ok(format!(
        "{name} {area:?}: queued 1 {}{}",
        rebellion_data::readouts::product_name(world, kind),
        ordered_ship(manufacturing, system, area)
    ))
}

/// The tag of the unfinished capital ship the last order at `system`'s
/// `area` made, as `Queue build` reports it, or nothing for another product:
/// the ship exists from the order (`QueueItem::ship`).
fn ordered_ship(
    manufacturing: &rebellion_core::manufacturing::ManufacturingState,
    system: SystemKey,
    area: rebellion_core::manufacturing::ProductionArea,
) -> String {
    let ship = manufacturing
        .queue(system, area)
        .and_then(|queue| queue.items().back())
        .map_or(0, |item| item.ship);
    if ship == 0 {
        String::new()
    } else {
        format!(" (unfinished ship {ship})")
    }
}

/// Raise a side's maintenance capacity by at least `amount`, as
/// `resources::maintenance_capacity` counts it: 50 for each mine matched
/// by a refinery. port: staging only, with no original counterpart; the
/// pairs belong to no system, so no system's energy or raw cap sees them,
/// but they cycle and deliver like any facility of the side.
pub fn raise_maintenance_capacity(
    world: &mut GameWorld,
    alliance: bool,
    amount: u32,
) -> Result<String, String> {
    use rebellion_core::resources::{maintenance_capacity, FACILITY_MAINTENANCE_CAPACITY};
    use rebellion_core::world::ProductionFacilityInstance;

    let side = Faction::of_alliance(alliance);
    let class = |mine: bool| {
        world
            .production_facilities
            .values()
            .find(|facility| facility.is_mine == mine)
            .map(|facility| facility.class_dat_id)
    };
    let (Some(mine), Some(refinery)) = (class(true), class(false)) else {
        return Err("no mine and refinery class is loaded".into());
    };
    let before = maintenance_capacity(world, side);
    let pairs = u64::from(amount).div_ceil(FACILITY_MAINTENANCE_CAPACITY.unsigned_abs());
    for _ in 0..pairs {
        for (class_dat_id, is_mine) in [(mine, true), (refinery, false)] {
            world.production_facilities.insert(ProductionFacilityInstance {
                class_dat_id,
                side,
                is_mine,
            });
        }
    }
    Ok(format!(
        "{side:?} maintenance capacity {before} -> {}",
        maintenance_capacity(world, side)
    ))
}

/// The character called `name` (`<name>` or `<name> #<id>`, the id as
/// `List characters` prints it), ignoring case.
pub fn find_character(world: &GameWorld, name: &str) -> Result<CharacterKey, String> {
    let name = name.trim();
    let (name, id) = match name.rsplit_once('#') {
        Some((head, id)) => (
            head.trim(),
            Some(
                id.trim()
                    .parse::<u32>()
                    .map_err(|_| format!("{id:?} is not a character id"))?,
            ),
        ),
        None => (name, None),
    };
    let found: Vec<(CharacterKey, u32)> = world
        .characters
        .iter()
        .filter(|(_, c)| {
            !c.is_killed
                && (c.name.eq_ignore_ascii_case(name)
                    || c.display_name().eq_ignore_ascii_case(name))
        })
        .map(|(key, c)| (key, c.dat_id.raw()))
        .filter(|(_, raw)| id.is_none_or(|id| id == *raw))
        .collect();
    match found.as_slice() {
        [(key, _)] => Ok(*key),
        [] => Err(format!("no living character named {name:?}")),
        many => Err(format!(
            "{} characters are named {name:?}: add `#<id>`, one of {:?}",
            many.len(),
            many.iter().map(|(_, raw)| raw).collect::<Vec<_>>()
        )),
    }
}

/// A mission the Create Mission dialog would begin: what
/// `MissionDialogAction::Begin` carries.
pub struct MissionOrder {
    pub kind: MissionKind,
    pub team: Vec<MissionMember>,
    pub decoys: Vec<MissionMember>,
    pub target: SystemKey,
    pub object: Option<MissionTarget>,
}

/// The objects at `system` a mission may name, in the order a dev order
/// tries them: its fleets' Death Stars, then its defence, manufacturing and
/// production facilities, regiments and special forces.
fn objects_at(world: &GameWorld, system: SystemKey) -> Vec<MissionTarget> {
    let Some(value) = world.systems.get(system) else {
        return Vec::new();
    };
    let death_stars = value
        .fleets
        .iter()
        .filter_map(|&fleet| world.fleets.get(fleet))
        .flat_map(|fleet| &fleet.capital_ships)
        .map(|ship| MissionTarget::CapitalShip(ship.tag))
        .filter(|target| target.is_death_star(world));
    death_stars
        .chain(value.defense_facilities.iter().copied().map(MissionTarget::DefenseFacility))
        .chain(
            value
                .manufacturing_facilities
                .iter()
                .copied()
                .map(MissionTarget::ManufacturingFacility),
        )
        .chain(
            value
                .production_facilities
                .iter()
                .copied()
                .map(MissionTarget::ProductionFacility),
        )
        .chain(value.ground_units.iter().copied().map(MissionTarget::Troop))
        .chain(value.special_forces.iter().copied().map(MissionTarget::SpecialForce))
        .collect()
}

/// Resolve a `Mission:` line into the order the dialog would submit for
/// `player`, refusing what the dialog would not offer: a character of the
/// other side, a kind outside `available_kinds` (the refusal names the kinds
/// it does offer), or an order `dispatch_validated` would refuse. The last
/// check runs on copies, so a refusal leaves the campaign as it was. A kind
/// that names an object takes the first one at the system the dialog
/// offers it for ([`objects_at`]), as a release on that object would.
///
/// `ship` names the target, a capital ship at the system, by its tag. Such
/// an order skips the dialog's list and the check on copies, so the real
/// `dispatch_validated` gives the validator's own answer: an unfinished ship
/// is refused `0x40`/`0x34` (`FUN_00593500.c:49-58`), which the dialog only
/// shows as a list without the kind. port: staging only; the player's
/// dialog never begins an order it does not list.
#[expect(
    clippy::too_many_arguments,
    reason = "The dialog's order is these inputs; a parameter struct would only restate them."
)]
pub fn mission_order(
    world: &GameWorld,
    uprisings: &UprisingState,
    missions: &MissionState,
    player: MissionFaction,
    team: &[String],
    decoys: &[String],
    kind: MissionKind,
    system: &str,
    ship: Option<u32>,
    tick: u64,
) -> Result<MissionOrder, String> {
    let target = find_system(world, system)?;
    let members = |names: &[String]| -> Result<Vec<MissionMember>, String> {
        names
            .iter()
            .map(|name| {
                let key = find_character(world, name)?;
                let own = world.characters[key].is_alliance == (player == MissionFaction::Alliance);
                if own {
                    Ok(MissionMember::Character(key))
                } else {
                    Err(format!("{name} is not on the player's side"))
                }
            })
            .collect()
    };
    let (team, decoys) = (members(team)?, members(decoys)?);
    if let Some(tag) = ship {
        if !matches!(kind, MissionKind::Sabotage | MissionKind::DeathStarSabotage) {
            return Err(format!("{} names no ship", kind.name()));
        }
        let (fleet, _) = rebellion_core::carriage::ship_of_tag(world, tag)
            .ok_or_else(|| format!("no capital ship has tag {tag}"))?;
        let at = world.fleets[fleet].location;
        if at != target {
            return Err(format!(
                "ship {tag} is at {}, not {}",
                world
                    .systems
                    .get(at)
                    .map_or("no system", |sys| sys.name.as_str()),
                world.systems[target].name
            ));
        }
        return Ok(MissionOrder {
            kind,
            team,
            decoys,
            target,
            object: Some(MissionTarget::CapitalShip(tag)),
        });
    }
    let object = objects_at(world, target).into_iter().find(|&object| {
        available_kinds(world, uprisings, player, &team, &decoys, target, Some(object))
            .contains(&kind)
    });
    let offered = available_kinds(world, uprisings, player, &team, &decoys, target, object);
    if !offered.contains(&kind) {
        let names: Vec<&str> = offered.iter().map(|kind| kind.name()).collect();
        return Err(format!(
            "the Create Mission dialog does not offer {} for this team at {}; it offers [{}]",
            kind.name(),
            world.systems[target].name,
            names.join(", ")
        ));
    }
    let request = MissionRequest {
        kind,
        faction: player,
        team: team.clone(),
        decoys: decoys.clone(),
        target_system: target,
        target_character: None,
        target_object: object,
        tick,
    };
    if let Err(refusal) =
        missions
            .clone()
            .dispatch_validated(request, &mut world.clone(), uprisings)
    {
        return Err(format!("the mission validator refuses it: {refusal:?}"));
    }
    Ok(MissionOrder {
        kind,
        team,
        decoys,
        target,
        object,
    })
}

/// Apply Recruitment's success effect to `name` for `side`, as the mission's
/// resolution does (`recruit_character`, `MissionEffect::CharacterRecruited`):
/// the character joins at `system`, or at the side's headquarters. Refused
/// for a character outside the side's recruit pool (`recruit_pool`), naming
/// why.
pub fn recruit(
    world: &mut GameWorld,
    name: &str,
    side: Faction,
    system: Option<&str>,
) -> Result<String, String> {
    let key = find_character(world, name)?;
    let faction = match side {
        Faction::Alliance => MissionFaction::Alliance,
        Faction::Empire => MissionFaction::Empire,
        Faction::Neutral => return Err("Neutral has no recruits".into()),
    };
    let character = &world.characters[key];
    let label = character.display_name().to_owned();
    if character.recruited {
        return Err(format!("{label} is already recruited"));
    }
    if character.is_major {
        return Err(format!(
            "{label} is a major character: Recruitment takes minor ones"
        ));
    }
    let pool = recruit_pool(world, side);
    if !pool.contains(&key) {
        return Err(format!(
            "{label} is not in the {side:?} recruit pool (wrong side or captive)"
        ));
    }
    let target = match system {
        Some(system) => {
            let target = find_system(world, system)?;
            if !world.systems[target].control.is_controlled_by(side) {
                return Err(format!(
                    "{} is not held by the {side:?}; Recruitment places a recruit in one of its systems",
                    world.systems[target].name
                ));
            }
            target
        }
        None => world
            .systems
            .iter()
            .find(|(_, s)| s.is_headquarters && s.control.is_controlled_by(side))
            .or_else(|| {
                world
                    .systems
                    .iter()
                    .find(|(_, s)| s.control.is_controlled_by(side))
            })
            .map(|(key, _)| key)
            .ok_or_else(|| format!("the {side:?} holds no system to recruit at"))?,
    };
    recruit_character(world, key, target, faction, pool.len() == 1);
    Ok(format!(
        "{label} recruited at {}",
        world.systems[target].name
    ))
}

/// Hold `name` in the other side's jail as `FUN_00549580` holds a stranded
/// mission member (`death_star::hold_in_jail`): captured with no captor, out
/// of every system and fleet, in the Empire Jail for an Alliance character
/// and the Alliance Jail for an Imperial one. Refused for a prisoner, and
/// for a character on a mission or travelling, whose mission or trip the
/// jail would strand.
pub fn jail(world: &mut GameWorld, name: &str, tick: u64) -> Result<String, String> {
    let key = find_character(world, name)?;
    let character = &world.characters[key];
    let label = character.display_name().to_owned();
    if character.is_captive {
        return Err(format!("{label} is already a prisoner"));
    }
    if character.on_mission {
        return Err(format!("{label} is on a mission"));
    }
    if character.en_route {
        return Err(format!("{label} is travelling"));
    }
    rebellion_core::death_star::hold_in_jail(world, key, tick);
    let jail = world.characters[key].abode.map_or("", |abode| abode.name());
    Ok(format!("{label} held in the {jail}"))
}

/// The refusal the real `dispatch_validated` last gave a `PanelAction::DispatchMission`,
/// as `<reason> (0x<a>/0x<b>)`, for the `Mission:` result that is waiting on it.
pub static LAST_DISPATCH_REFUSAL: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

/// Where `Save: <name>` and `Load: <name>` keep a save: the evidence
/// folder's `saves` directory, under the slot files' own extension.
pub fn save_path(evidence: &std::path::Path, name: &str) -> std::path::PathBuf {
    let slot_file = rebellion_data::save::slot_path(std::path::Path::new(""), 0);
    let extension = slot_file.extension().unwrap_or_default().to_string_lossy();
    evidence.join("saves").join(format!("{name}.{extension}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::world::{Abode, CapitalShipClass, Character, Fleet, Sector, System};

    #[test]
    fn a_jailed_character_sits_in_the_other_sides_jail_out_of_its_fleet() {
        // FUN_00549580: captured with no captor, into the jail of the
        // captor's side bits (ghidra/notes/jail.md, section 4); the abode
        // names are TEXTSTRA 11600 and 11616.
        for (is_alliance, name) in [(true, "Empire Jail"), (false, "Alliance Jail")] {
            let mut world = GameWorld::default();
            let system = world.systems.insert(System::default());
            let fleet = world.fleets.insert(Fleet {
                location: system,
                is_alliance,
                ..Fleet::default()
            });
            let key = world.characters.insert(Character {
                name: "Han Solo".into(),
                is_alliance,
                is_empire: !is_alliance,
                current_fleet: Some(fleet),
                ..Character::default()
            });
            world.fleets[fleet].characters.push(key);

            assert_eq!(
                jail(&mut world, "han solo", 12),
                Ok(format!("Han Solo held in the {name}"))
            );
            let character = &world.characters[key];
            assert_eq!(character.held_abode().map(Abode::name), Some(name));
            assert!(world.fleets[fleet].characters.is_empty(), "{is_alliance}");
        }
    }

    #[test]
    fn the_jail_command_refuses_a_prisoner_a_mission_member_and_a_traveller() {
        for (character, why) in [
            (
                Character {
                    is_captive: true,
                    ..Character::default()
                },
                "is already a prisoner",
            ),
            (
                Character {
                    on_mission: true,
                    ..Character::default()
                },
                "is on a mission",
            ),
            (
                Character {
                    en_route: true,
                    ..Character::default()
                },
                "is travelling",
            ),
        ] {
            let mut world = GameWorld::default();
            let key = world.characters.insert(Character {
                name: "Leia Organa".into(),
                is_alliance: true,
                ..character
            });
            let before = world.characters[key].clone();

            assert_eq!(
                jail(&mut world, "Leia Organa", 3),
                Err(format!("Leia Organa {why}"))
            );
            let after = &world.characters[key];
            assert_eq!(
                (after.is_captive, after.abode, after.capture_tick),
                (before.is_captive, before.abode, before.capture_tick)
            );
        }
    }

    #[test]
    fn a_sabotage_order_takes_the_first_object_the_dialog_offers_it_for() {
        // FUN_005422f0 keeps the kinds whose validator (FUN_0054c590) takes
        // the released object: a yard gives Sabotage; the system alone
        // offers none. port: the dev line names no object, so the order
        // takes the first one at the system that the dialog offers.
        let mut world = GameWorld::default();
        let target_rules = rebellion_core::world::MissionTargetRules {
            target_loss_ends: true,
            other_side_target: true,
            opponent_target: true,
            ..rebellion_core::world::MissionTargetRules::default()
        };
        world.mission_records = vec![rebellion_core::world::MissionRecord {
            dat_id: rebellion_core::ids::DatId::new(0x6900_0012),
            timer_min_days: 5,
            timer_spread_days: 10,
            repeats: true,
            hidden: false,
            detection_phases: true,
            can_resign: true,
            return_policy: 1,
            rules: target_rules,
            members: rebellion_core::world::MissionMemberRules {
                alliance: true,
                empire: true,
                special_force_mask: 0x402,
                character_mask: 0x1_0000,
            },
        }];
        let system = world.systems.insert(System {
            name: "Kuat".into(),
            is_populated: true,
            exploration_status: rebellion_core::dat::ExplorationStatus::Explored,
            control: rebellion_core::world::ControlKind::Controlled(Faction::Empire),
            ..System::default()
        });
        let yard = world.manufacturing_facilities.insert(
            rebellion_core::world::ManufacturingFacilityInstance {
                class_dat_id: rebellion_core::ids::DatId::new(0x2800_0001),
                side: Faction::Empire,
                is_shipyard: true,
            },
        );
        world.systems[system].manufacturing_facilities.push(yard);
        world.characters.insert(Character {
            name: "Han Solo".into(),
            is_alliance: true,
            is_major: true,
            recruited: true,
            loyalty: rebellion_core::world::SkillPair {
                base: 100,
                variance: 0,
            },
            current_system: Some(system),
            ..Character::default()
        });
        let order = mission_order(
            &world,
            &UprisingState::default(),
            &MissionState::new(),
            MissionFaction::Alliance,
            &["Han Solo".into()],
            &[],
            MissionKind::Sabotage,
            "Kuat",
            None,
            1,
        )
        .expect("the dialog offers Sabotage on the yard");
        assert_eq!(
            order.object,
            Some(rebellion_core::missions::MissionTarget::ManufacturingFacility(yard))
        );
    }

    #[test]
    fn queue_build_names_the_unfinished_ship_its_order_made_and_nothing_for_other_products() {
        use rebellion_core::manufacturing::{
            BuildableKind, ManufacturingState, ProductionArea, QueueItem,
        };
        let mut world = GameWorld::default();
        let system = world.systems.insert(System::default());
        let class = world
            .capital_ship_classes
            .insert(CapitalShipClass::default());
        let item = |ship| QueueItem {
            kind: BuildableKind::CapitalShip(class),
            work_done: 0,
            total_cost: 10,
            destination: None,
            carrier: 0,
            ship,
        };
        let mut manufacturing = ManufacturingState::default();
        assert_eq!(
            ordered_ship(&manufacturing, system, ProductionArea::Shipyard),
            ""
        );
        manufacturing.build_units(system, vec![item(41), item(42)]);
        assert_eq!(
            ordered_ship(&manufacturing, system, ProductionArea::Shipyard),
            " (unfinished ship 42)"
        );
        manufacturing.build_units(system, vec![item(0)]);
        assert_eq!(
            ordered_ship(&manufacturing, system, ProductionArea::Shipyard),
            ""
        );
    }

    #[test]
    fn a_named_ship_is_the_orders_target_finished_or_not_and_must_stand_at_the_system() {
        // The mission's +0x38 holds the ship itself (death-star-product.md,
        // section 4); an unfinished one goes on to the validator, whose
        // FUN_00593500.c:49-58 refuses it 0x40/0x34 at the real dispatch.
        use rebellion_core::missions::MissionTarget;
        for player_is_alliance in [true, false] {
            let mut world = GameWorld::default();
            let corellia = world.systems.insert(System {
                name: "Corellia".into(),
                ..System::default()
            });
            world.systems.insert(System {
                name: "Kuat".into(),
                ..System::default()
            });
            let class = world
                .capital_ship_classes
                .insert(CapitalShipClass::default());
            let mut unfinished = rebellion_core::world::ShipInstance::new(class, 100, false);
            unfinished.completed = false;
            let finished = rebellion_core::world::ShipInstance::new(class, 100, false);
            let fleet = world.fleets.insert(Fleet {
                location: corellia,
                is_alliance: !player_is_alliance,
                capital_ships: vec![unfinished, finished],
                ..Fleet::default()
            });
            world.systems[corellia].fleets.push(fleet);
            rebellion_core::carriage::tag_all(&mut world);
            world.characters.insert(Character {
                name: "Han Solo".into(),
                is_alliance: player_is_alliance,
                is_empire: !player_is_alliance,
                ..Character::default()
            });
            let player = if player_is_alliance {
                MissionFaction::Alliance
            } else {
                MissionFaction::Empire
            };
            let order = |kind, system: &str, ship| {
                mission_order(
                    &world,
                    &UprisingState::default(),
                    &MissionState::new(),
                    player,
                    &["Han Solo".into()],
                    &[],
                    kind,
                    system,
                    ship,
                    1,
                )
                .map(|order| (order.kind, order.target, order.object))
            };
            for ship in &world.fleets[fleet].capital_ships {
                assert_eq!(
                    order(MissionKind::Sabotage, "Corellia", Some(ship.tag)),
                    Ok((
                        MissionKind::Sabotage,
                        corellia,
                        Some(MissionTarget::CapitalShip(ship.tag))
                    )),
                    "{player_is_alliance} {}",
                    ship.completed
                );
            }
            let tag = world.fleets[fleet].capital_ships[0].tag;
            assert_eq!(
                order(MissionKind::Sabotage, "Kuat", Some(tag)),
                Err(format!("ship {tag} is at Corellia, not Kuat"))
            );
            assert_eq!(
                order(MissionKind::Sabotage, "Corellia", Some(999)),
                Err("no capital ship has tag 999".to_owned())
            );
            assert_eq!(
                order(MissionKind::Espionage, "Corellia", Some(tag)),
                Err("Espionage names no ship".to_owned())
            );
        }
    }

    #[test]
    fn stripping_a_system_destroys_only_its_holders_defenders_and_keeps_its_control() {
        use rebellion_core::world::{
            ControlKind, DefenseFacilityInstance, ShipInstance, SystemSquadrons, TroopUnit,
        };
        for holder in [Faction::Alliance, Faction::Empire] {
            let ours = holder == Faction::Alliance;
            let mut world = GameWorld::default();
            let system = world.systems.insert(System {
                name: "Coruscant".into(),
                control: ControlKind::Controlled(holder),
                ..System::default()
            });
            let troop = |world: &mut GameWorld, is_alliance| {
                let key = world.troops.insert(TroopUnit {
                    is_alliance,
                    ..TroopUnit::default()
                });
                world.systems[system].ground_units.push(key);
                key
            };
            let (first, second, visitor) = (
                troop(&mut world, ours),
                troop(&mut world, ours),
                troop(&mut world, !ours),
            );
            let battery = world.defense_facilities.insert(DefenseFacilityInstance {
                side: holder,
                ..DefenseFacilityInstance::default()
            });
            world.systems[system].defense_facilities.push(battery);
            let class = world.fighter_classes.insert(Default::default());
            world.systems[system].fighters = vec![
                SystemSquadrons {
                    class,
                    count: 3,
                    is_alliance: ours,
                },
                SystemSquadrons {
                    class,
                    count: 1,
                    is_alliance: !ours,
                },
            ];
            let ship = world.capital_ship_classes.insert(CapitalShipClass::default());
            let fleet = |world: &mut GameWorld, is_alliance| {
                let key = world.fleets.insert(Fleet {
                    location: system,
                    is_alliance,
                    capital_ships: vec![ShipInstance::new(ship, 100, is_alliance)],
                    ..Fleet::default()
                });
                world.systems[system].fleets.push(key);
                key
            };
            let (guard, raider) = (fleet(&mut world, ours), fleet(&mut world, !ours));

            strip_defenders(&mut world, &mut TroopTransportState::default(), system, 9)
                .expect("stripped");

            let sys = &world.systems[system];
            assert_eq!(sys.ground_units, [visitor], "{holder:?}");
            assert!(!world.troops.contains_key(first) && !world.troops.contains_key(second));
            assert!(sys.defense_facilities.is_empty());
            assert!(!world.defense_facilities.contains_key(battery));
            assert_eq!(sys.fighters.len(), 1);
            assert_eq!(sys.fighters[0].is_alliance, !ours);
            assert_eq!(sys.fleets, [raider]);
            assert!(!world.fleets.contains_key(guard));
            assert_eq!(sys.control, ControlKind::Controlled(holder));
            assert!(world.support_events.iter().any(|event| matches!(
                event,
                rebellion_core::support_shift::QueuedEvent::Regiments(at) if *at == system
            )));
        }
    }

    // The Defenses window's drag onto a fleet is the regiment's Move onto
    // it (FUN_00578f30 -> FUN_00556390): in the fleet's own system it boards
    // at once, on the ship with room (regiment-unload.md).
    #[test]
    fn embark_boards_the_players_regiments_and_refuses_too_many_with_nothing_moved() {
        use rebellion_core::world::{ControlKind, ShipInstance, TroopUnit};
        for holder in [Faction::Alliance, Faction::Empire] {
            let ours = holder == Faction::Alliance;
            let mut world = GameWorld::default();
            let system = world.systems.insert(System {
                name: "Coruscant".into(),
                control: ControlKind::Controlled(holder),
                ..System::default()
            });
            let troops: Vec<_> = [ours, ours, !ours]
                .into_iter()
                .map(|is_alliance| {
                    let key = world.troops.insert(TroopUnit {
                        is_alliance,
                        ..TroopUnit::default()
                    });
                    world.systems[system].ground_units.push(key);
                    key
                })
                .collect();
            let ship = world.capital_ship_classes.insert(CapitalShipClass {
                troop_capacity: 4,
                hull: 100,
                ..CapitalShipClass::default()
            });
            let fleet = world.insert_fleet(Fleet {
                location: system,
                is_alliance: ours,
                capital_ships: vec![ShipInstance::new(ship, 100, ours)],
                ..Fleet::default()
            });
            world.systems[system].fleets.push(fleet);
            let name = world.fleet_name(fleet).unwrap().to_owned();
            let mut transport = TroopTransportState::default();
            let mut run = |world: &mut GameWorld, count| {
                embark(
                    world,
                    &mut transport,
                    &MovementState::new(),
                    &std::collections::HashSet::new(),
                    &rebellion_core::carriage::ProductRoom::default(),
                    ours,
                    &name,
                    count,
                    9,
                )
            };

            assert!(run(&mut world, 3).is_err(), "{holder:?}: two of ours stand there");
            assert_eq!(world.systems[system].ground_units, troops, "{holder:?}");
            assert!(run(&mut world, 2).is_ok(), "{holder:?}");
            assert_eq!(world.systems[system].ground_units, [troops[2]], "{holder:?}");
            assert_eq!(transport.cargo(fleet), &troops[..2], "{holder:?}");
        }
    }

    #[test]
    fn a_given_death_star_marks_its_sector_at_once() {
        // FUN_00514a60 reruns FUN_005178b0 when a [0x18,0x1c) object
        // changes container (event 0x301), so the bit is set before the
        // day's next shift.
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector::default());
        let system = world.systems.insert(System {
            sector,
            ..System::default()
        });
        world.capital_ship_classes.insert(CapitalShipClass {
            family: 0x18,
            hull: 25_000,
            ..CapitalShipClass::default()
        });
        let fleet = world.fleets.insert(Fleet {
            location: system,
            ..Fleet::default()
        });
        world.systems[system].fleets.push(fleet);
        give_death_star(&mut world, "Fleet").expect("given");
        assert!(world.death_star_in_sector(&world.systems[system]));
    }

    #[test]
    fn the_other_sides_fleet_destroys_a_system_through_the_ais_strike_and_keeps_the_orbit() {
        // FUN_004b1070 issues the AI's 0x223 for its Death Star fleet;
        // integrator::resolve_ai_strike runs it through order_strike: the
        // cascade kills the system's own regiment and leaves every fleet in
        // orbit (FUN_00524fb0, FUN_004f8710). The AI orders no 0x221.
        use rebellion_core::world::{ControlKind, ShipInstance, TroopUnit};
        for striker in [Faction::Empire, Faction::Alliance] {
            let victim = if striker == Faction::Empire {
                Faction::Alliance
            } else {
                Faction::Empire
            };
            let mut world = GameWorld::default();
            let system = world.systems.insert(System {
                control: ControlKind::Controlled(victim),
                ..System::default()
            });
            // CAPSHPSD record 136: bombardment modifier 15, the gate's
            // strength (FUN_00582440).
            let death_star = world.capital_ship_classes.insert(CapitalShipClass {
                family: 0x18,
                hull: 100,
                bombardment_modifier: 15,
                ..CapitalShipClass::default()
            });
            let cruiser = world.capital_ship_classes.insert(CapitalShipClass {
                family: 0x14,
                hull: 100,
                ..CapitalShipClass::default()
            });
            let mut fleet = |side: Faction, class| {
                let alliance = side == Faction::Alliance;
                let key = world.fleets.insert(Fleet {
                    location: system,
                    is_alliance: alliance,
                    capital_ships: vec![ShipInstance::new(class, 100, alliance)],
                    ..Fleet::default()
                });
                world.systems[system].fleets.push(key);
                key
            };
            let orbit = fleet(victim, cruiser);
            let strike = fleet(striker, death_star);
            let troop = world.troops.insert(TroopUnit {
                class_dat_id: rebellion_core::ids::DatId::new(0x1000_0001),
                is_alliance: victim == Faction::Alliance,
            });
            world.systems[system].ground_units.push(troop);
            assert_eq!(
                find_fleet(&world, striker, "fleet"),
                Ok(strike),
                "{striker:?}"
            );
            assert_eq!(
                find_fleet(&world, victim, "FLEET"),
                Ok(orbit),
                "{striker:?}"
            );
            let adjective = if striker == Faction::Alliance {
                "Alliance"
            } else {
                "Imperial"
            };
            assert_eq!(
                find_fleet(&world, striker, "Fleet 9"),
                Err(format!("no {adjective} fleet named \"Fleet 9\""))
            );

            let mut movement = MovementState::new();
            let mut manufacturing = rebellion_core::manufacturing::ManufacturingState::new();
            let mut transport = TroopTransportState::default();
            let mut deliveries = rebellion_core::delivery::DeliveryState::new();
            let mut missions = MissionState::new();
            let uprisings = UprisingState::new();
            let mut blockade = rebellion_core::blockade::BlockadeState::new();
            let mut states = StrikeStates {
                movement: &mut movement,
                manufacturing: &mut manufacturing,
                transport: &mut transport,
                deliveries: &mut deliveries,
                missions: &mut missions,
                uprisings: &uprisings,
                blockade: &mut blockade,
                tick: 1,
                step: 0,
            };
            assert!(ai_strike(
                &mut world,
                strike,
                BombardKind::Civilian,
                &mut states,
                &mut || 0.5
            )
            .is_err());
            let effects = ai_strike(
                &mut world,
                strike,
                BombardKind::DestroySystem,
                &mut states,
                &mut || 0.5,
            )
            .expect("the strike runs");
            assert_eq!(effects.destroyed(), Some(system), "{striker:?}");
            assert!(world.systems[system].is_destroyed, "{striker:?}");
            assert!(!world.troops.contains_key(troop), "{striker:?}");
            assert_eq!(world.systems[system].fleets, [orbit, strike], "{striker:?}");
            assert_eq!(world.fleets[orbit].capital_ships.len(), 1, "{striker:?}");
            let detail = strike_detail(BombardKind::DestroySystem, &[effects]);
            assert!(
                detail.as_ref().is_ok_and(
                    |line| line.starts_with("0x223: ") && line.contains("destroyed true")
                ),
                "{detail:?}"
            );
        }
        assert_eq!(
            strike_detail(BombardKind::General, &[]),
            Err("0x222: in transit, not flagged".to_owned())
        );
    }

    #[test]
    fn a_battle_brings_the_sides_first_idle_fleet_of_capital_ships_but_not_its_death_star() {
        // port: the developer command's choice of fleet; the Death Star is
        // its family-0x18 ship (FUN_005003d0).
        for alliance in [false, true] {
            let mut world = GameWorld::default();
            let mut at = |x| {
                world.systems.insert(System {
                    x,
                    ..System::default()
                })
            };
            let (home, battle) = (at(0), at(40));
            let death_star = world.capital_ship_classes.insert(CapitalShipClass {
                family: 0x18,
                ..CapitalShipClass::default()
            });
            let cruiser = world.capital_ship_classes.insert(CapitalShipClass {
                family: 0x14,
                ..CapitalShipClass::default()
            });
            let mut fleet = |is_alliance, ships: Vec<_>| {
                let key = world.fleets.insert(Fleet {
                    location: home,
                    is_alliance,
                    capital_ships: ships
                        .into_iter()
                        .map(|class| rebellion_core::world::ShipInstance::new(class, 100, is_alliance))
                        .collect(),
                    ..Fleet::default()
                });
                world.systems[home].fleets.push(key);
                key
            };
            fleet(!alliance, vec![cruiser]);
            fleet(alliance, vec![]);
            fleet(alliance, vec![death_star]);
            let chosen = fleet(alliance, vec![cruiser]);
            let note = bring_side(
                &mut world,
                &MovementState::new(),
                &mut TroopTransportState::default(),
                battle,
                alliance,
                0,
            )
            .expect("a fleet comes");
            assert!(note.is_some());
            assert_eq!(world.systems[battle].fleets, [chosen], "{alliance}");
        }
    }
}
