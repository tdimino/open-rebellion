//! The Death Star's strategic rules: the sector it holds (system bit 11),
//! the destroy cascade and the adrift sweep.
//!
//! The Death Star is the capital ship of family `0x18`-`0x1b` (CAPSHPSD
//! record 136), built as any shipyard product; a fleet holds it while one of
//! its live ships is of that family (`Fleet::has_death_star`, `FUN_005003d0`).
//! Destroy System is the bombardment order `0x223` (`crate::bombardment`;
//! `ghidra/notes/death-star-strategic.md`); the destroyed system's children
//! go through the destroy cascade ([`cleanup_destroyed_system`]) and its
//! adrift characters through the sweep the same day ([`sweep_adrift`]).
//!
//! # Source
//! - `ghidra/notes/death-star-strategic.md`, `ghidra/notes/bombardment.md`
//! - `entity-system.md §4.2` — `alive_flag` inverted semantics for systems

use crate::dat::Faction;
use crate::ids::SystemKey;
use crate::world::GameWorld;

// ---------------------------------------------------------------------------
// System bit 11: a Death Star in the sector
// ---------------------------------------------------------------------------

use crate::manufacturing::ManufacturingState;
use crate::movement::MovementState;

/// `FUN_005178b0`: mark the sectors holding a usable Death Star
/// (`GameWorld::death_star_sectors`, system `+0x88` bit 11 through
/// `FUN_0050a890`). A Death Star is usable (`+0x50` bit 0, written only by
/// `FUN_004f7b80.c:8-12`) once complete and not en route; the port's ship
/// joins its fleet on completion, and a fleet's ships take its en-route bit
/// (`FUN_004f8240.c:17-31`), so a usable one is in a fleet its system holds
/// (`System::fleets` never holds a fleet in transit,
/// `movement::reconcile_fleet_orbits`). The sector is that system's
/// (`FUN_00518690.c:19-37`).
///
/// It reruns where the usable bit of a family `0x18`-`0x1b` object changes,
/// on event `0x301` (`FUN_00514510.c:149-158`): a Death Star's completion,
/// departure, arrival and loss call it at once. The bit is not saved, so a
/// load or a new game rebuilds it.
pub fn refresh_sectors(world: &mut GameWorld) {
    let sectors = world
        .systems
        .values()
        .filter(|sys| {
            sys.fleets
                .iter()
                .filter_map(|&fleet| world.fleets.get(fleet))
                .any(|fleet| fleet.has_usable_death_star(&world.capital_ship_classes))
        })
        .map(|sys| sys.sector)
        .collect();
    world.death_star_sectors = sectors;
}

// ---------------------------------------------------------------------------
// The destroy cascade
// ---------------------------------------------------------------------------

use crate::delivery::DeliveryState;
use crate::ids::{CharacterKey, DatId, FighterKey, SpecialForceKey, TroopKey};
use crate::manufacturing::{BuildableKind, Destination};
use crate::missions::{
    MissionFaction, MissionMember, MissionState, MissionTarget, MoveDestination,
};
use crate::post_battle::{request_place, request_squadron_place, Mover, Place, Relocation};
use crate::troop_transport::{RegimentLeg, TroopTransportState};
use crate::world::SystemSquadrons;

/// One object the cascade's autoscrap request (`+0x50` bit 12,
/// `FUN_004f78e0(1)`) handled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Autoscrapped {
    /// An uncompleted product of the system's queues.
    Product(BuildableKind),
    /// A completed product travelling to the system.
    Delivery(BuildableKind),
    /// A regiment travelling to its surface.
    Regiment(TroopKey),
    /// A squadron travelling to it.
    Squadron {
        class: FighterKey,
        is_alliance: bool,
    },
}

/// What a system's destruction did to its children. The support shifts,
/// the HQ's among them (`FUN_00517cd0` on its event `0x300`), are the
/// caller's.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CascadeReport {
    /// Regiments the system held, destroyed, with their side.
    pub regiments: Vec<(TroopKey, Faction)>,
    /// Squadrons the system held, destroyed.
    pub squadrons: Vec<SystemSquadrons>,
    /// Facilities destroyed: class and side.
    pub facilities: Vec<(DatId, Faction)>,
    /// Special forces destroyed.
    pub special_forces: Vec<SpecialForceKey>,
    /// Uncompleted products destroyed: every product of the system's own
    /// production managers (reason `0x16`, `FUN_0052b7d0` ->
    /// `FUN_0052a2c0`), and special-force products of other managers
    /// bound for it (`FUN_00503e20`). None is refunded (`FUN_00530270`).
    pub products: Vec<BuildableKind>,
    /// The Alliance HQ died with the system.
    pub headquarters: bool,
    /// Characters left adrift.
    pub adrift: Vec<CharacterKey>,
    /// What took the autoscrap request, and where it went.
    pub autoscrapped: Vec<(Autoscrapped, Relocation)>,
}

/// A system's destruction (slot `+0xac`, `FUN_005080d0` → `FUN_004f8660` →
/// `FUN_004f71d0`; hook `+0x11c`, `FUN_00510950` → `FUN_004fb520`), once
/// the caller has set `is_destroyed`:
///
/// 1. The existing bit's clear reaches `FUN_005109a0` → `FUN_0050b760`,
///    which clears a destroyed system's populated bit (`FUN_0050a3c0(0)`).
/// 2. Slot `+0xb4(1)` on every child (`FUN_004fcd00`, mode 3). Fleets
///    survive with everything aboard (`FUN_00524fb0` returns 1), the Death
///    Star's own too, so bit 11 keeps its sector. The system's completed
///    regiments, squadrons, facilities and HQ not en route die
///    (`FUN_004f8710` → `FUN_004f7ed0`); the uncompleted or en route ones
///    take the autoscrap request. Special forces not en route die; en
///    route ones are left to arrive (`FUN_00503e20` with argument 1).
///    Characters off a mission go adrift (`FUN_004ef2e0` →
///    `FUN_00534790`), with no message (`FUN_00536ac0` only logs).
///
/// Owner, support and resources are not written, no move is cancelled and
/// the blockade bit is untouched (`ghidra/notes/death-star-strategic.md`,
/// "What destroying a system does"). port: the original walks the children
/// in container order; the port takes its per-kind lists in turn.
pub fn cleanup_destroyed_system(
    world: &mut GameWorld,
    system: SystemKey,
    movement: &MovementState,
    transport: &mut TroopTransportState,
    manufacturing: &mut ManufacturingState,
    deliveries: &mut DeliveryState,
) -> CascadeReport {
    let mut report = CascadeReport::default();
    // Coruscant's flag is the port's Empire seat, not an ALLFACSD object, so
    // the cascade leaves it.
    let headquarters = crate::support_shift::alliance_headquarters(world, system);
    let Some(sys) = world.systems.get_mut(system) else {
        return report;
    };
    sys.is_populated = false;
    // The HQ (ALLFACSD 0x20000001) is completed; the port never moves it.
    if headquarters {
        sys.is_headquarters = false;
        report.headquarters = true;
    }
    report.squadrons = std::mem::take(&mut sys.fighters);
    let regiments = sys.ground_units.clone();
    let special_forces = sys.special_forces.clone();
    for troop in regiments {
        if let Some(unit) = world.troops.get(troop) {
            report.regiments.push((
                troop,
                crate::mission_detection::faction_of(unit.is_alliance),
            ));
        }
        crate::missions::destroy_target(world, MissionTarget::Troop(troop));
    }
    for facility in world.facilities_at(system) {
        if let Some(entry) = world.facility(facility) {
            report.facilities.push(entry);
        }
        world.remove_facility(system, facility);
    }
    for unit in special_forces {
        if world
            .special_forces
            .get(unit)
            .is_some_and(|value| !value.en_route)
        {
            crate::missions::destroy_special_force(world, unit);
            report.special_forces.push(unit);
        }
    }
    // FUN_004ef2e0: a character directly held by the system, not on a
    // mission (`+0x78` bit 7) and not destroyed. The container's reason is
    // 0xf or 0x10, never 0x14, so none takes the autoscrap branch. hyp:
    // the setter's guard `FUN_0053a000` (`+0x24 & 0x30`) holds for every
    // live character.
    for (key, character) in &mut world.characters {
        if character.current_system == Some(system)
            && character.current_fleet.is_none()
            && !character.on_mission
            && !character.is_killed
        {
            character.adrift = true;
            report.adrift.push(key);
        }
    }
    autoscrap(
        world,
        system,
        movement,
        transport,
        manufacturing,
        deliveries,
        &mut report,
    );
    report
}

/// The chain's mover for a product of `kind`.
const fn product_mover(kind: BuildableKind) -> Mover {
    match kind {
        BuildableKind::Troop(_) => Mover::Regiment,
        BuildableKind::Fighter(_) => Mover::Squadron,
        _ => Mover::Other,
    }
}

/// Where an autoscrap request sends a product of `kind` held for
/// `system`: the chain with `hint`, its producer's system or its transit
/// origin, as stage 2. A squadron that boards a fleet takes the ship there
/// with the most room, as the move's leg does (`FUN_00556390` ->
/// `FUN_00552300`), and holds that room from then (`products`); the
/// result's tag is that ship's, 0 for a system.
#[expect(
    clippy::too_many_arguments,
    reason = "The request reads the world, the transport, movement and product states and the product."
)]
fn place_product(
    world: &mut GameWorld,
    transport: &TroopTransportState,
    movement: &MovementState,
    products: &mut crate::carriage::ProductRoom,
    system: SystemKey,
    hint: SystemKey,
    is_alliance: bool,
    kind: BuildableKind,
) -> Option<(Place, u32)> {
    let found = match product_mover(kind) {
        Mover::Squadron => request_squadron_place(
            world,
            transport,
            movement,
            products,
            system,
            Some(hint),
            is_alliance,
        ),
        mover => request_place(world, transport, movement, system, Some(hint), is_alliance, mover),
    }?;
    let Place::Fleet(fleet) = found else {
        return Some((found, 0));
    };
    let index = crate::carriage::ship_for_squadron(world, products, fleet)?;
    let carrier = crate::carriage::tag_ship(world, fleet, index)?;
    products.add(kind, carrier);
    Some((found, carrier))
}

/// The side a product of `kind` is built for: its class's, or the
/// facility order's. port: a class missing from the catalog takes the
/// building system's holder.
fn product_is_alliance(world: &GameWorld, kind: BuildableKind, holder: bool) -> bool {
    match kind {
        BuildableKind::CapitalShip(class) => world
            .capital_ship_classes
            .get(class)
            .map(|value| value.is_alliance),
        BuildableKind::Fighter(class) => world
            .fighter_classes
            .get(class)
            .map(|value| value.is_alliance),
        BuildableKind::Troop(class) | BuildableKind::SpecialForce(class) => world
            .buildable_classes
            .get(&class)
            .map(|value| value.is_alliance),
        BuildableKind::DefenseFacility(build)
        | BuildableKind::ManufacturingFacility(build)
        | BuildableKind::ProductionFacility(build) => Some(build.is_alliance),
    }
    .unwrap_or(holder)
}

/// The destroyed system's own products die with its managers, then each
/// autoscrap request runs at once ([`request_bound_for`]): the chain from
/// the destroyed system refuses the system itself (`FUN_00553410`,
/// `0x22`).
fn autoscrap(
    world: &mut GameWorld,
    system: SystemKey,
    movement: &MovementState,
    transport: &mut TroopTransportState,
    manufacturing: &mut ManufacturingState,
    deliveries: &mut DeliveryState,
    report: &mut CascadeReport,
) {
    // The system's three production managers are completed children
    // (FUN_0050e200), destroyed in the child walk (FUN_004f8710); each
    // one's existing-bit clear (hook +0x128, FUN_0052b7d0 -> FUN_0052a2c0)
    // destroys every product it queued with reason 0x16, whatever its
    // destination.
    report.products.extend(
        manufacturing
            .take_queued(system)
            .into_iter()
            .map(|item| item.kind),
    );
    request_bound_for(
        world,
        system,
        movement,
        transport,
        manufacturing,
        deliveries,
        Walk::Destroyed,
        report,
    );
}

/// The child walk whose autoscrap requests [`request_bound_for`] serves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Walk {
    /// Destroy System's cascade, slot `+0xb4`: a special-force product
    /// dies (`FUN_00503e20(1)`), and the system refuses everything
    /// (`FUN_00553410`, `0x22`).
    Destroyed,
    /// A holder change, slot `+0xb0`: a special-force product takes the
    /// new side when its class serves it (slot `+0x50`,
    /// `FUN_004f27d0`), else dies with reason 3 (`FUN_00503d90`); the
    /// system keeps what still suits it (chain stage 1).
    HolderChange,
}

/// The autoscrap requests (`+0x50` bit `0x1000`, event `0x307` →
/// `FUN_00515480`) of what is bound for `system` without standing in it:
/// another manager's unfinished products (`FUN_004f8680.c:14-17`, a
/// squadron's `FUN_00503510`), and completed products, regiments and
/// squadrons on their way (`+0x50` `0x20`). Each takes the chain from
/// `system` with its hint (`FUN_00515480.c:82-153`): the producer's system,
/// or the transit origin. One `system` still accepts stays as it is
/// (stage 1); one no place accepts dies with reason 2, which refunds
/// nothing (`FUN_00530270`). A capital ship sits in a fleet, which neither
/// walk visits (`FUN_00524fb0`, `FUN_0051ebb0`). A product's unfinished
/// move is a reparent with no transit, and it keeps its queue slot
/// (`FUN_00528890` reads `+0x38`, which the move leaves): on completion it
/// ships from the yard to its new place. port: a queued special force
/// whose class serves both sides keeps the side the port reads from its
/// class.
#[expect(
    clippy::too_many_arguments,
    reason = "The requests read and move the world, the transport, movement, production and delivery states."
)]
fn request_bound_for(
    world: &mut GameWorld,
    system: SystemKey,
    movement: &MovementState,
    transport: &mut TroopTransportState,
    manufacturing: &mut ManufacturingState,
    deliveries: &mut DeliveryState,
    walk: Walk,
    report: &mut CascadeReport,
) {
    let Some(control) = world.systems.get(system).map(|sys| sys.control) else {
        return;
    };
    let holder = control.is_controlled_by(Faction::Alliance);
    let side = GameWorld::facility_holder(control);
    let serves = |world: &GameWorld, class: DatId| {
        walk == Walk::HolderChange
            && world
                .buildable_classes
                .get(&class)
                .is_none_or(|value| value.serves_side(side))
    };
    let relocation = |found: Option<Place>| match found {
        Some(Place::System(to)) if to == system => Relocation::Stayed,
        Some(Place::System(to)) => Relocation::Moved(to),
        Some(Place::Fleet(fleet)) => Relocation::Boarded(fleet),
        None => Relocation::Destroyed(2),
    };
    let mut products = crate::carriage::ProductRoom::new(manufacturing, deliveries);
    manufacturing.retain_bound_for(system, |producer, item| match item.kind {
        BuildableKind::CapitalShip(_) => true,
        BuildableKind::SpecialForce(class) => {
            let kept = serves(world, class);
            if !kept {
                report.products.push(item.kind);
            }
            kept
        }
        kind => {
            let is_alliance = product_is_alliance(world, kind, holder);
            let found = place_product(
                world,
                transport,
                movement,
                &mut products,
                system,
                producer,
                is_alliance,
                kind,
            );
            report.autoscrapped.push((
                Autoscrapped::Product(kind),
                relocation(found.map(|(place, _)| place)),
            ));
            let (destination, carrier) = match found {
                Some((Place::System(to), carrier)) => (Destination::System(to), carrier),
                Some((Place::Fleet(fleet), carrier)) => (Destination::Fleet(fleet), carrier),
                None => return false,
            };
            item.destination = (destination != Destination::System(producer)).then_some(destination);
            item.carrier = carrier;
            true
        }
    });
    // Completed products travelling here; their hint is their transit
    // origin.
    let mut placed = Vec::new();
    let taken = deliveries.take_bound_for(system, |delivery| {
        let is_alliance = product_is_alliance(world, delivery.kind, holder);
        let found = place_product(
            world,
            transport,
            movement,
            &mut products,
            system,
            delivery.origin,
            is_alliance,
            delivery.kind,
        );
        report.autoscrapped.push((
            Autoscrapped::Delivery(delivery.kind),
            relocation(found.map(|(place, _)| place)),
        ));
        let stays = matches!(found, Some((Place::System(to), _)) if to == system);
        if !stays {
            placed.push(found);
        }
        !stays
    });
    for (mut delivery, found) in taken.into_iter().zip(placed) {
        if let Some((place, carrier)) = found {
            let destination = match place {
                Place::System(to) => Destination::System(to),
                Place::Fleet(fleet) => Destination::Fleet(fleet),
            };
            delivery.carrier = carrier;
            deliveries.relocate(world, delivery, destination);
        }
    }
    // Regiments travelling to its surface.
    let regiments: Vec<(TroopKey, SystemKey)> = transport
        .transits()
        .iter()
        .filter(|transit| transit.leg == RegimentLeg::Surface(system))
        .map(|transit| (transit.troop, transit.origin))
        .collect();
    for (troop, origin) in regiments {
        let is_alliance = world.troops.get(troop).is_some_and(|unit| unit.is_alliance);
        let found = request_place(
            world,
            transport,
            movement,
            system,
            Some(origin),
            is_alliance,
            Mover::Regiment,
        );
        let outcome = match found {
            Some(Place::System(to)) if to == system => Relocation::Stayed,
            Some(Place::System(to)) => {
                transport.reroute_regiment(troop, to);
                Relocation::Moved(to)
            }
            // A regiment's chain offers no fleet (stage 4 is a squadron's).
            Some(Place::Fleet(_)) | None => {
                transport.forget_transit(troop);
                world.troops.remove(troop);
                Relocation::Destroyed(2)
            }
        };
        report
            .autoscrapped
            .push((Autoscrapped::Regiment(troop), outcome));
    }
    // Squadrons travelling to it, in order; one that stays keeps its leg,
    // so the walk passes over it.
    let mut stayed = 0;
    while let Some(index) = world
        .squadron_transits
        .iter()
        .enumerate()
        .filter(|(_, transit)| transit.leg == RegimentLeg::Surface(system))
        .nth(stayed)
        .map(|(index, _)| index)
    {
        let transit = world.squadron_transits[index].clone();
        let found = request_squadron_place(
            world,
            transport,
            movement,
            &products,
            system,
            Some(transit.origin),
            transit.is_alliance,
        );
        let leg = match found {
            Some(Place::System(to)) if to == system => {
                stayed += 1;
                report.autoscrapped.push((
                    Autoscrapped::Squadron {
                        class: transit.class,
                        is_alliance: transit.is_alliance,
                    },
                    Relocation::Stayed,
                ));
                continue;
            }
            Some(Place::System(to)) => Some(RegimentLeg::Surface(to)),
            Some(Place::Fleet(fleet)) => Some(RegimentLeg::Fleet(fleet)),
            None => None,
        };
        // A leg with no ship left to carry it loses the squadron too.
        let moved = leg.and_then(|leg| {
            crate::troop_transport::reroute_squadron(world, &products, index, leg)
        });
        if leg.is_none() {
            world.squadron_transits.remove(index);
        }
        report.autoscrapped.push((
            Autoscrapped::Squadron {
                class: transit.class,
                is_alliance: transit.is_alliance,
            },
            relocation(moved.and(found)),
        ));
    }
}

/// The autoscrap requests each holder change raised and has not served
/// (`GameWorld::autoscrap_systems`): the change's child walk
/// (`FUN_00510820` → `FUN_004fae40`, slot `+0xb0`) flags what is bound for
/// the system, and event `0x307` serves each request after phase 5
/// ([`request_bound_for`], [`Walk::HolderChange`]). The system's own
/// managers' products are gone already
/// ([`crate::manufacturing::serve_manager_sides`]). Returns what the
/// requests did: `autoscrapped` and the products that died outright.
pub fn serve_holder_requests(
    world: &mut GameWorld,
    movement: &MovementState,
    transport: &mut TroopTransportState,
    manufacturing: &mut ManufacturingState,
    deliveries: &mut DeliveryState,
) -> CascadeReport {
    let mut report = CascadeReport::default();
    for system in std::mem::take(&mut world.autoscrap_systems) {
        request_bound_for(
            world,
            system,
            movement,
            transport,
            manufacturing,
            deliveries,
            Walk::HolderChange,
            &mut report,
        );
    }
    report
}

/// Event `0x304` for each change of a system's battle bit (hook `+0x23c`,
/// `FUN_00511780`, from `FUN_0050b8e0`): reruns the bit at every system
/// ([`crate::blockade::BlockadeState::battle_changes`]) and sweeps each
/// system whose bit changed with its new value. The headless and native
/// steps both call it after the blockade rule. Returns each swept system
/// with the trips its sweep started.
pub fn sweep_battle_changes(
    world: &mut GameWorld,
    missions: &mut MissionState,
    blockade: &mut crate::blockade::BlockadeState,
    transport: &TroopTransportState,
    movement: &MovementState,
    tick: u64,
) -> Vec<(SystemKey, Vec<u64>)> {
    blockade
        .battle_changes(world)
        .into_iter()
        .map(|(system, bit)| {
            (
                system,
                sweep_adrift(world, missions, transport, movement, system, bit, tick),
            )
        })
        .collect()
}

/// Event `0x307` for each character a holder change flagged
/// (`support_shift`, `FUN_004ef260` -> `FUN_004f78e0(1)`): `FUN_00515480`
/// -> `FUN_00545df0` -> `FUN_00548da0` sweeps the character's location
/// ([`sweep_adrift`]) with that system's battle bit. Sweeps each system
/// holding a flagged character off a mission, in system order, and returns
/// each with the trips its sweep started.
///
/// port: the request is the port's adrift flag, which the sweep takes alike
/// (`FUN_0054c7d0.c:55-56` tests `+0x50` bit 12 or `+0x78` bit 6). hyp: the
/// event runs before the operation that raised it ends, so the port sweeps
/// once the operation's holder changes are served.
pub fn sweep_requests(
    world: &mut GameWorld,
    missions: &mut MissionState,
    transport: &TroopTransportState,
    movement: &MovementState,
    blockade: &crate::blockade::BlockadeState,
    tick: u64,
) -> Vec<(SystemKey, Vec<u64>)> {
    let systems: Vec<SystemKey> = world
        .systems
        .keys()
        .filter(|&system| {
            world.characters.values().any(|c| {
                c.adrift
                    && !c.on_mission
                    && !c.is_killed
                    && c.current_fleet.is_none()
                    && c.current_system == Some(system)
            })
        })
        .collect();
    systems
        .into_iter()
        .map(|system| {
            let battle = blockade.in_battle(system);
            let started = sweep_adrift(world, missions, transport, movement, system, battle, tick);
            (system, started)
        })
        .filter(|(_, started)| !started.is_empty())
        .collect()
}

/// `FUN_00548e90` at `system`: `battle_bit` is the system's battle bit
/// (`+0x88` `0x1000`: side 1 and side 2 both present at a system with a
/// side, not the blockade bit `0x20`) when it runs. It runs on each change
/// of the bit (event `0x304` with flag 0, raised by hook `+0x23c`,
/// `FUN_00511780`, from the rule `FUN_0050b8e0` → `FUN_0050a900`;
/// [`sweep_battle_changes`]) and on each adrift flag the destroy cascade
/// sets (event `0x30e`, `FUN_00534790` → `FUN_00536460`;
/// `integrator::resolve_strike`).
///
/// Adrift characters aboard the system's fleets move into the system
/// (slot `+0xa8`). Unless the bit is set, `FUN_00549230` then takes each
/// adrift character held by the system, off a mission, and groups them
/// ([`sweep_groups`]). Returns the missions started.
#[must_use]
pub fn sweep_adrift(
    world: &mut GameWorld,
    missions: &mut MissionState,
    transport: &TroopTransportState,
    movement: &MovementState,
    system: SystemKey,
    battle_bit: bool,
    tick: u64,
) -> Vec<u64> {
    let Some(sys) = world.systems.get(system) else {
        return Vec::new();
    };
    for fleet in sys.fleets.clone() {
        let aboard: Vec<CharacterKey> = world.fleets.get(fleet).map_or_else(Vec::new, |value| {
            value
                .characters
                .iter()
                .copied()
                .filter(|&key| world.characters.get(key).is_some_and(|c| c.adrift))
                .collect()
        });
        for key in aboard {
            if let Some(value) = world.fleets.get_mut(fleet) {
                value.characters.retain(|&other| other != key);
            }
            if let Some(character) = world.characters.get_mut(key) {
                character.current_fleet = None;
                character.current_system = Some(system);
            }
        }
    }
    if battle_bit {
        return Vec::new();
    }
    let mut flagged: Vec<CharacterKey> = world
        .characters
        .iter()
        .filter(|(_, c)| {
            c.adrift
                && !c.on_mission
                && !c.is_killed
                && c.current_system == Some(system)
                && c.current_fleet.is_none()
        })
        .map(|(key, _)| key)
        .collect();
    let mut started = Vec::new();
    // Prisoners freed by FUN_00549850 hold the autoscrap request again and
    // go round once more (event 0x307 → FUN_00548da0). hyp: the request's
    // sweep runs after FUN_00549850 returns, so the freed form one group.
    while !flagged.is_empty() {
        flagged = sweep_groups(
            world,
            missions,
            transport,
            movement,
            system,
            &flagged,
            tick,
            &mut started,
        );
    }
    started
}

/// One pass of `FUN_00549230` over the flagged characters at `system`.
///
/// `FUN_0054c7d0` skips a character holding a captor id (`FUN_004ed270`),
/// which stays flagged, and clears the flags of every other one. Those
/// whose system is destroyed or of another side group by side bits (a
/// prisoner's are its captor's) and destination, the retreat chain (`FUN_00543040` → `FUN_005545e0`):
///
/// - a group with a free member starts its hidden trip, prisoners aboard
///   (`FUN_00547740` / `FUN_00547880`; [`MissionState::send_adrift`]);
/// - a group of prisoners is freed (`FUN_00549850`, slot `+0x218`); the
///   freed are returned for the next pass;
/// - each member of a group with no destination is captured or freed and
///   tries again on its new side, else goes to a jail (`FUN_00549580`,
///   [`strand`]).
///
/// port: the port has no Bounty mission (MISSNSD family `0x65`), so the
/// Jabba's Palace destination of `FUN_00543040` never applies.
#[expect(
    clippy::too_many_arguments,
    reason = "The pass reads the chain's states and writes the missions and the started list."
)]
fn sweep_groups(
    world: &mut GameWorld,
    missions: &mut MissionState,
    transport: &TroopTransportState,
    movement: &MovementState,
    system: SystemKey,
    flagged: &[CharacterKey],
    tick: u64,
    started: &mut Vec<u64>,
) -> Vec<CharacterKey> {
    let Some(sys) = world.systems.get(system) else {
        return Vec::new();
    };
    // The side bits FUN_0054c7d0 compares: a stalemate keeps the earlier
    // holder's (FUN_0050b610).
    let (destroyed, holder) = (sys.is_destroyed, crate::bombardment::holder(world, system));
    let mut groups: Vec<(bool, Option<SystemKey>, Vec<CharacterKey>)> = Vec::new();
    for &key in flagged {
        let Some(character) = world.characters.get_mut(key) else {
            continue;
        };
        if character.captured_by.is_some() {
            continue;
        }
        character.adrift = false;
        // The side bits: a prisoner carries its captor's (FUN_004f1a80 ->
        // FUN_004eec60).
        let is_alliance = character.is_alliance != character.is_captive;
        if !destroyed && holder == crate::mission_detection::faction_of(is_alliance) {
            continue;
        }
        let destination = match request_place(
            world,
            transport,
            movement,
            system,
            None,
            is_alliance,
            Mover::Other,
        ) {
            Some(Place::System(to)) => Some(to),
            Some(Place::Fleet(_)) | None => None,
        };
        match groups
            .iter_mut()
            .find(|(side, to, _)| *side == is_alliance && *to == destination)
        {
            Some((_, _, members)) => members.push(key),
            None => groups.push((is_alliance, destination, vec![key])),
        }
    }
    let mut freed = Vec::new();
    for (is_alliance, destination, members) in groups {
        let Some(destination) = destination else {
            for key in members {
                strand(world, missions, transport, movement, system, key, tick, started);
            }
            continue;
        };
        let captive = |key: &CharacterKey| world.characters.get(*key).is_some_and(|c| c.is_captive);
        if members.iter().all(captive) {
            for key in members {
                if let Some(character) = world.characters.get_mut(key) {
                    character.is_captive = false;
                    character.capture_tick = None;
                }
                freed.push(key);
            }
            continue;
        }
        let faction = if is_alliance {
            MissionFaction::Alliance
        } else {
            MissionFaction::Empire
        };
        let team = members.into_iter().map(MissionMember::Character).collect();
        started.push(missions.send_adrift(
            world,
            faction,
            team,
            MoveDestination::System(destination),
            tick,
        ));
    }
    freed
}

/// `FUN_00548120` at a mission's end (end code `+0x64` set;
/// `ghidra/notes/mission-return.md`): `FUN_00521220` routes a member when
/// the record's return policy (`+0x68`) is 2, or 1 and the side bits of the
/// object holding it differ from its own; the others stay where they are.
/// `FUN_00542ee0` gives a routed member its destination: under policy 1 the
/// container it set out from (`+0x70`) when that is a legal Move
/// destination for its side, else the retreat chain (`FUN_00543040`). `FUN_00548910` then groups the members by destination:
///
/// - a group with a team or decoy member starts the hidden Return
///   `0x42000002` (`FUN_005475f0`), captives aboard, when all stand at one
///   place (`FUN_0054bf00`);
/// - a group of captives alone is freed (`FUN_00549850`), and the request
///   it raises sweeps their location at once;
/// - a group with no destination goes through `FUN_00549580`: a character
///   is stranded ([`strand`]), a special force destroyed with reason 9
///   (slot `+0x20c`, `FUN_00503eb0`).
///
/// Returns the missions started.
///
/// `+0x70` is [`crate::missions::MissionEnded::set_out_from`], the fleet
/// or system the team joined from; a fleet passes the Move test
/// (`FUN_005543c0` -> `FUN_00553410`) while it is of the member's side and
/// not en route, and a Return to it boards it.
///
/// The return runs after the outcome's effects (the phase-10 handler
/// applies them before `FUN_00546ea0` runs `FUN_00548120`), so a Diplomacy
/// that turned its system keeps its envoy there.
///
/// port: the Return is the port's hidden Move, as Autorouting is; a port
/// character rides its fleet, not a ship, so a `+0x70` ship is its fleet.
#[expect(
    clippy::too_many_arguments,
    reason = "The return reads the chain's states and the battle bits and writes the missions."
)]
pub fn return_members(
    world: &mut GameWorld,
    missions: &mut MissionState,
    transport: &TroopTransportState,
    movement: &MovementState,
    blockade: &crate::blockade::BlockadeState,
    ended: &crate::missions::MissionEnded,
    tick: u64,
) -> Vec<u64> {
    let policy = ended
        .kind
        .record_id()
        .and_then(|id| world.mission_record(id))
        .map_or(0, |record| record.return_policy);
    let mut started = Vec::new();
    if policy == 0 {
        return started;
    }
    let side_of = |world: &GameWorld, member: MissionMember| match member {
        MissionMember::Character(key) => world
            .characters
            .get(key)
            .filter(|c| !c.is_killed)
            .map(|c| c.is_alliance != c.is_captive),
        MissionMember::SpecialForce(key) => world.special_forces.get(key).map(|u| u.is_alliance),
    };
    let mut groups: Vec<(Option<MoveDestination>, bool, Vec<MissionMember>)> = Vec::new();
    let members = ended
        .team
        .iter()
        .map(|&member| (member, true))
        .chain(ended.captured.iter().map(|&member| (member, false)));
    for (member, on_team) in members {
        let Some(side) = side_of(world, member) else {
            continue;
        };
        let faction = crate::mission_detection::faction_of(side);
        let same = crate::missions::container_side(world, member) == Some(faction);
        if !(policy == 2 || (policy == 1 && !same)) {
            continue;
        }
        let at = crate::missions::member_location(world, member);
        let accepts = |system: SystemKey| {
            world.systems.get(system).is_some_and(|sys| {
                !sys.is_destroyed && crate::bombardment::holder(world, system) == faction
            })
        };
        // FUN_00553410 on a fleet: of the member's side (1/0x24) and not
        // en route (1/0x21); a character needs no room.
        let legal = |to: MoveDestination| match to {
            MoveDestination::System(system) => accepts(system),
            MoveDestination::Fleet(fleet) => world.fleets.get(fleet).is_some_and(|value| {
                crate::dat::Faction::of_alliance(value.is_alliance) == faction
                    && !movement.is_in_transit(fleet)
            }),
        };
        // Under policy 1 the container it stands in is another side's,
        // which the Move test refuses (1/0x24, FUN_00553410).
        let destination = if policy == 1 && ended.set_out_from.is_some_and(legal) {
            ended.set_out_from
        } else {
            at.or(ended.origin).and_then(|from| {
                match request_place(world, transport, movement, from, ended.origin, side, Mover::Other)
                {
                    Some(Place::System(to)) => Some(MoveDestination::System(to)),
                    Some(Place::Fleet(_)) | None => None,
                }
            })
        };
        match groups.iter_mut().find(|(to, _, _)| *to == destination) {
            Some((_, team, list)) => {
                *team |= on_team;
                list.push(member);
            }
            None => groups.push((destination, on_team, vec![member])),
        }
    }
    for (destination, team, members) in groups {
        let Some(destination) = destination else {
            for member in members {
                match member {
                    MissionMember::Character(key) => {
                        if let Some(from) = crate::missions::member_location(world, member) {
                            strand(
                                world,
                                missions,
                                transport,
                                movement,
                                from,
                                key,
                                tick,
                                &mut started,
                            );
                        }
                    }
                    // FUN_00549580 -> slot +0x20c (FUN_00503eb0): +0xac(9).
                    MissionMember::SpecialForce(key) => {
                        crate::missions::destroy_special_force(world, key);
                    }
                }
            }
            continue;
        };
        if !team {
            for member in members {
                let MissionMember::Character(key) = member else {
                    continue;
                };
                let Some(character) = world.characters.get_mut(key) else {
                    continue;
                };
                character.is_captive = false;
                character.captured_by = None;
                character.capture_tick = None;
                character.adrift = true;
                if let Some(at) = character.current_system {
                    started.extend(sweep_adrift(
                        world,
                        missions,
                        transport,
                        movement,
                        at,
                        blockade.in_battle(at),
                        tick,
                    ));
                }
            }
            continue;
        }
        let places: Vec<Option<SystemKey>> = members
            .iter()
            .map(|&member| crate::missions::member_location(world, member))
            .collect();
        let arrived = match destination {
            MoveDestination::System(system) => places[0] == Some(system),
            MoveDestination::Fleet(fleet) => members.iter().all(|&member| {
                member
                    .character()
                    .and_then(|key| world.characters.get(key))
                    .is_some_and(|c| c.current_fleet == Some(fleet))
            }),
        };
        if places.windows(2).any(|pair| pair[0] != pair[1]) || arrived {
            continue;
        }
        started.push(missions.send_adrift(world, ended.faction, members, destination, tick));
    }
    started
}

/// A special force deployed into `system`, if it is destroyed: the product
/// has no leg (`FUN_005150c0.c:76` -> `FUN_00545180`), and its presence
/// check `FUN_00556710` gives it the autoscrap request (`FUN_00503e20(0)`),
/// which event `0x307` serves through the sweep: the chain's place of its
/// side takes it, else it is destroyed with reason 9 (`FUN_00549580.c:56-60`;
/// `death-star-strategic.md`, "Arrival at a destroyed system").
pub fn deploy_special_forces(
    world: &mut GameWorld,
    transport: &TroopTransportState,
    movement: &MovementState,
    system: SystemKey,
) {
    if !world.systems.get(system).is_some_and(|sys| sys.is_destroyed) {
        return;
    }
    let units: Vec<SpecialForceKey> = world.systems[system]
        .special_forces
        .iter()
        .copied()
        .filter(|&unit| {
            world
                .special_forces
                .get(unit)
                .is_some_and(|value| !value.on_mission && !value.en_route)
        })
        .collect();
    for unit in units {
        let is_alliance = world.special_forces[unit].is_alliance;
        match request_place(world, transport, movement, system, None, is_alliance, Mover::Other) {
            Some(Place::System(to)) => {
                world.systems[system].special_forces.retain(|&key| key != unit);
                world.systems[to].special_forces.push(unit);
            }
            Some(Place::Fleet(_)) | None => crate::missions::destroy_special_force(world, unit),
        }
    }
}

/// Yavin, `FUN_005431e0`'s origin for side 1 away from any system.
const YAVIN: u32 = 0x9200_0121;

/// An escape's release (`FUN_004ef0c0` → slot `+0x214`, status 1): the
/// prisoner bit clears, the native side returns and the captor clears
/// (`FUN_004f18e0`), and the autoscrap request it raises (`FUN_004f78e0(1)`)
/// posts event `0x307`, which sweeps the character's own location at once
/// (`FUN_00515480` → `FUN_00545df0` → `FUN_00548da0`, [`sweep_adrift`]): it
/// stays at a system of its side and leaves any other. Returns the
/// missions started.
///
/// hyp: the sweep of an abode was not traced (`jail.md`, "Escape"); a
/// jailed escapee is swept from `FUN_005431e0`'s origin away from any
/// system, Yavin for the Alliance and Coruscant for the Empire.
#[expect(
    clippy::too_many_arguments,
    reason = "The release reads the chain's states and the battle bit and writes the missions."
)]
pub fn free_escapee(
    world: &mut GameWorld,
    missions: &mut MissionState,
    transport: &TroopTransportState,
    movement: &MovementState,
    blockade: &crate::blockade::BlockadeState,
    key: CharacterKey,
    tick: u64,
) -> Vec<u64> {
    let Some(character) = world.characters.get(key) else {
        return Vec::new();
    };
    let jailed = character.held_abode().is_some();
    let origin = if jailed {
        let id = if character.is_alliance {
            YAVIN
        } else {
            crate::victory::CORUSCANT
        };
        world
            .systems
            .iter()
            .find(|(_, sys)| sys.dat_id.is_record(id))
            .map(|(system, _)| system)
    } else {
        character
            .current_fleet
            .and_then(|fleet| world.fleets.get(fleet))
            .map(|fleet| fleet.location)
            .or(character.current_system)
    };
    let character = &mut world.characters[key];
    character.is_captive = false;
    character.captured_by = None;
    character.capture_tick = None;
    character.escape_due = None;
    character.adrift = true;
    if jailed {
        character.leave_abode();
        character.current_system = origin;
    }
    let Some(system) = origin else {
        return Vec::new();
    };
    sweep_adrift(
        world,
        missions,
        transport,
        movement,
        system,
        blockade.in_battle(system),
        tick,
    )
}

/// `FUN_00549580` for one member of a group with no destination. A free
/// member is captured with no captor (slot `+0x20c`, status 0) and a
/// prisoner freed (slot `+0x218`, status 2); the prisoner-bit hook gives
/// the member its captor's side bits or its own back (`FUN_004f1a80` →
/// `FUN_004eec60`). The retreat chain (`FUN_00543040`) runs again on that
/// side: a destination starts the member's hidden trip there, a prisoner
/// aboard as one (`FUN_00547740`). With none again, a free member is
/// captured again and goes to the jail of its side bits (side 1:
/// `0x98000440`, otherwise `0x98000480`), the jail of the side opposite its
/// own, out of every system and fleet. The status change posts event
/// `0x30a` (`FUN_0057c8b0`), whose victory check reads the prisoner bit
/// (`victory-conditions.md`), and its autoscrap request clears again in the
/// same pass (`FUN_004f78e0(0)`).
///
/// port: the trip to the abode takes 1 day (`FUN_00555b30` sets 1 tick
/// between two keys not both systems), during which the member already sits
/// in the abode (`FUN_00556390`); the port holds it there from the sweep on
/// with its en route bit clear, which no reader of an abode occupant sees
/// (the Personnel Finder row of a family-`0x98` container shows no state,
/// `jail.md`).
#[expect(
    clippy::too_many_arguments,
    reason = "The member's retry reads the chain's states and writes the missions and the started list."
)]
fn strand(
    world: &mut GameWorld,
    missions: &mut MissionState,
    transport: &TroopTransportState,
    movement: &MovementState,
    system: SystemKey,
    key: CharacterKey,
    tick: u64,
    started: &mut Vec<u64>,
) {
    let Some(character) = world.characters.get_mut(key) else {
        return;
    };
    let captive = !character.is_captive;
    character.is_captive = captive;
    character.captured_by = None;
    character.capture_tick = captive.then_some(tick);
    let side = character.is_alliance != captive;
    if let Some(Place::System(to)) =
        request_place(world, transport, movement, system, None, side, Mover::Other)
    {
        let faction = if side {
            MissionFaction::Alliance
        } else {
            MissionFaction::Empire
        };
        let team = vec![MissionMember::Character(key)];
        let to = MoveDestination::System(to);
        started.push(missions.send_adrift(world, faction, team, to, tick));
        return;
    }
    hold_in_jail(world, key, tick);
}

/// `FUN_00549580`'s jail: the character is captured with no captor (slot
/// `+0x20c`), which gives it its captor's side bits (`FUN_004f1a80` →
/// `FUN_004eec60`), and its trip reparents it into the jail of those side
/// bits at once (`FUN_00556390`; side 1: `0x98000440`, otherwise
/// `0x98000480`), the jail of the side opposite its own, out of every system
/// and fleet ([`strand`]).
pub fn hold_in_jail(world: &mut GameWorld, key: CharacterKey, tick: u64) {
    for (_, fleet) in &mut world.fleets {
        fleet.characters.retain(|&other| other != key);
    }
    let Some(character) = world.characters.get_mut(key) else {
        return;
    };
    character.is_captive = true;
    character.captured_by = None;
    character.capture_tick = Some(tick);
    character.current_system = None;
    character.current_fleet = None;
    character.abode = Some(crate::world::Abode::jail_of(!character.is_alliance));
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::carriage::ProductRoom;
    use super::*;
    use crate::dat::{ExplorationStatus, Faction, SectorGroup};
    use crate::ids::FleetKey;
    use crate::world::ControlKind;
    use crate::ids::DatId;
    use crate::world::{Fleet, GameWorld, Sector, System};

    fn make_world() -> (GameWorld, SystemKey) {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(0x9200_0000),
            name: "Outer Rim".into(),
            group: SectorGroup::RimOuter,
            x: 0,
            y: 0,
            systems: vec![],
        });
        let sys = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0000),
            name: "Alderaan".into(),
            sector,
            x: 100,
            y: 100,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.9,
            popularity_empire: 0.1,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: vec![],
            ground_units: vec![],
            special_forces: vec![],
            fighters: Vec::new(),
            defense_facilities: vec![],
            manufacturing_facilities: vec![],
            production_facilities: vec![],
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Controlled(Faction::Alliance),
        });
        (world, sys)
    }

    /// An Empire fleet at `sys` holding the Death Star ship (family `0x18`).
    fn add_ds_fleet(world: &mut GameWorld, sys: SystemKey) -> FleetKey {
        let class = world
            .capital_ship_classes
            .insert(crate::world::CapitalShipClass {
                dat_id: DatId::new(crate::world::DEATH_STAR_CLASS_ID),
                family: 0x18,
                is_empire: true,
                hull: 100,
                ..Default::default()
            });
        let fk = world.fleets.insert(Fleet {
            location: sys,
            capital_ships: vec![crate::world::ShipInstance::new(class, 100, false)],
            fighters: vec![],
            characters: vec![],
            is_alliance: false,
            bombarding: false,
        });
        world.systems.get_mut(sys).unwrap().fleets.push(fk);
        fk
    }

    // ── Construction tests ───────────────────────────────────────────────────

    #[test]
    fn bit_11_covers_the_sector_of_a_death_star_that_is_not_travelling() {
        let (mut world, sys) = make_world();
        let home = world.systems[sys].sector;
        let neighbour = world.systems.insert(System {
            name: "Delaya".into(),
            ..world.systems[sys].clone()
        });
        let far_sector = world.sectors.insert(Sector {
            dat_id: DatId::new(0x9200_0001),
            name: "Core".into(),
            group: SectorGroup::Core,
            x: 0,
            y: 0,
            systems: vec![],
        });
        let far = world.systems.insert(System {
            name: "Coruscant".into(),
            sector: far_sector,
            ..world.systems[sys].clone()
        });
        let fleet = add_ds_fleet(&mut world, sys);
        let mut movement = MovementState::new();

        refresh_sectors(&mut world);
        assert!(world.death_star_in_sector(&world.systems[sys]));
        assert!(world.death_star_in_sector(&world.systems[neighbour]));
        assert!(!world.death_star_in_sector(&world.systems[far]));
        assert_eq!(world.death_star_sectors.len(), 1);
        assert!(world.death_star_sectors.contains(&home));

        assert!(crate::movement::begin_fleet_transit(
            &mut movement,
            &mut world,
            fleet,
            far,
            10
        ));
        refresh_sectors(&mut world);
        assert!(world.death_star_sectors.is_empty());
    }

    // ── The destroy cascade ──────────────────────────────────────────────────

    use crate::manufacturing::{Departure, FacilityBuild, ProductionArea, QueueItem};
    use crate::world::{
        CapitalShipClass, Character, DefenseFacilityInstance, FighterClass, FighterEntry,
        ManufacturingFacilityInstance, ProductionFacilityInstance, ShipInstance, SpecialForceUnit,
        TroopUnit,
    };

    /// Alderaan (Alliance, at (100, 100)) and a second system at `(x, 100)`
    /// held by `refuge`.
    fn cascade_world(x: u16, refuge: Faction) -> (GameWorld, SystemKey, SystemKey) {
        let (mut world, target) = make_world();
        let other = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0002),
            name: "Delaya".into(),
            x,
            control: ControlKind::Controlled(refuge),
            ..world.systems[target].clone()
        });
        let sector = world.systems[target].sector;
        world.sectors[sector].systems = vec![target, other];
        (world, target, other)
    }

    fn troop(world: &mut GameWorld, system: SystemKey, is_alliance: bool) -> TroopKey {
        let key = world.troops.insert(TroopUnit {
            class_dat_id: DatId::new(0x1000_0001),
            is_alliance,
        });
        world.systems[system].ground_units.push(key);
        key
    }

    /// One completed capital ship of a side: the fleet override
    /// `FUN_004fe540` makes a fleet usable, and so present, only through
    /// such a ship.
    fn completed_ship(world: &mut GameWorld, is_alliance: bool) -> crate::world::ShipInstance {
        let class = world
            .capital_ship_classes
            .insert(crate::world::CapitalShipClass {
                is_alliance,
                is_empire: !is_alliance,
                hull: 100,
                ..crate::world::CapitalShipClass::default()
            });
        crate::world::ShipInstance::new(class, 100, is_alliance)
    }

    fn destroy(
        world: &mut GameWorld,
        system: SystemKey,
        transport: &mut TroopTransportState,
        manufacturing: &mut ManufacturingState,
        deliveries: &mut DeliveryState,
    ) -> CascadeReport {
        world.systems[system].is_destroyed = true;
        cleanup_destroyed_system(
            world,
            system,
            &MovementState::new(),
            transport,
            manufacturing,
            deliveries,
        )
    }

    fn destroy_plain(world: &mut GameWorld, system: SystemKey) -> CascadeReport {
        destroy(
            world,
            system,
            &mut TroopTransportState::default(),
            &mut ManufacturingState::new(),
            &mut DeliveryState::new(),
        )
    }

    /// `FUN_00524fb0`, the fleet's slot `+0xb4`, returns 1 and does nothing:
    /// every fleet at the system survives with its ships, squadrons,
    /// regiments and characters, the Death Star's own included, and bit 11
    /// keeps its sector (`death-star-strategic.md`, "What destroying a
    /// system does"; the Yaga Minor VM run).
    #[test]
    fn both_sides_fleets_survive_the_destruction_with_everything_aboard() {
        let (mut world, sys) = make_world();
        let death_star = add_ds_fleet(&mut world, sys);
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            is_alliance: true,
            hull: 100,
            troop_capacity: 2,
            ..CapitalShipClass::default()
        });
        let fighter = world.fighter_classes.insert(FighterClass::default());
        let rebels = world.fleets.insert(Fleet {
            location: sys,
            capital_ships: vec![ShipInstance::new(class, 100, true)],
            fighters: vec![FighterEntry {
                class: fighter,
                count: 2,
                carrier: 0,
            }],
            is_alliance: true,
            ..Fleet::default()
        });
        world.systems[sys].fleets.push(rebels);
        let pilot = world.characters.insert(Character {
            is_alliance: true,
            current_system: Some(sys),
            current_fleet: Some(rebels),
            ..Character::default()
        });
        world.fleets[rebels].characters.push(pilot);
        let regiment = troop(&mut world, sys, true);
        let mut transport = TroopTransportState::default();
        transport
            .embark(&mut world, &ProductRoom::default(), rebels, &[regiment])
            .unwrap();

        let report = destroy(
            &mut world,
            sys,
            &mut transport,
            &mut ManufacturingState::new(),
            &mut DeliveryState::new(),
        );

        assert_eq!(world.systems[sys].fleets, vec![death_star, rebels]);
        assert!(world.fleet_has_death_star(death_star));
        assert_eq!(world.fleets[rebels].fighters[0].count, 2);
        assert_eq!(world.fleets[rebels].characters, vec![pilot]);
        assert_eq!(transport.cargo(rebels), &[regiment]);
        assert!(world.troops.contains_key(regiment));
        assert!(!world.characters[pilot].adrift);
        assert!(report.regiments.is_empty());
        refresh_sectors(&mut world);
        assert!(world.death_star_in_sector(&world.systems[sys]));
    }

    /// `FUN_004f8710`: the system's completed children not en route die,
    /// whichever side holds them, the HQ among them; `FUN_0050b760` clears
    /// the populated bit; the chain writes no owner, support or resources.
    #[test]
    fn the_systems_own_completed_units_of_both_sides_die_with_it() {
        let (mut world, sys) = make_world();
        world.systems[sys].is_headquarters = true;
        world.systems[sys].total_energy = 4;
        world.systems[sys].raw_materials = 3;
        let rebels = troop(&mut world, sys, true);
        let imperials = troop(&mut world, sys, false);
        let fighter = world.fighter_classes.insert(FighterClass::default());
        world.systems[sys].add_squadrons(fighter, 2, true);
        world.systems[sys].add_squadrons(fighter, 1, false);
        let shield = world.defense_facilities.insert(DefenseFacilityInstance {
            class_dat_id: DatId::new(0x2400_0001),
            side: Faction::Alliance,
        });
        let yard = world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                class_dat_id: DatId::new(0x2800_0001),
                side: Faction::Alliance,
                is_shipyard: true,
            });
        let mine = world
            .production_facilities
            .insert(ProductionFacilityInstance {
                class_dat_id: DatId::new(0x2c00_0001),
                side: Faction::Alliance,
                is_mine: true,
            });
        world.systems[sys].defense_facilities.push(shield);
        world.systems[sys].manufacturing_facilities.push(yard);
        world.systems[sys].production_facilities.push(mine);

        let report = destroy_plain(&mut world, sys);

        let value = &world.systems[sys];
        assert!(value.ground_units.is_empty() && world.troops.is_empty());
        assert!(value.fighters.is_empty());
        assert!(value.defense_facilities.is_empty() && world.defense_facilities.is_empty());
        assert!(value.manufacturing_facilities.is_empty());
        assert!(value.production_facilities.is_empty());
        assert!(!value.is_headquarters && report.headquarters);
        assert!(!value.is_populated);
        assert_eq!(value.control, ControlKind::Controlled(Faction::Alliance));
        assert_eq!((value.total_energy, value.raw_materials), (4, 3));
        assert!((value.popularity_alliance - 0.9).abs() < f32::EPSILON);
        assert_eq!(
            report.regiments,
            vec![(rebels, Faction::Alliance), (imperials, Faction::Empire)]
        );
        assert_eq!(
            report
                .squadrons
                .iter()
                .map(|entry| entry.count)
                .sum::<u32>(),
            3
        );
        assert_eq!(report.facilities.len(), 3);
    }

    /// Coruscant's HQ flag is the port's Empire seat; the ALLFACSD HQ is the
    /// only HQ object (`FUN_00506110`, `game-end-path.md`).
    #[test]
    fn coruscant_keeps_its_seat_flag_when_destroyed() {
        let (mut world, sys) = make_world();
        world.systems[sys].dat_id = DatId::new(0x9000_0109);
        world.systems[sys].is_headquarters = true;

        let report = destroy_plain(&mut world, sys);

        assert!(world.systems[sys].is_headquarters);
        assert!(!report.headquarters);
    }

    /// `FUN_00503e20` with argument 1: a special force not en route dies; one
    /// en route is left to arrive.
    #[test]
    fn special_forces_die_unless_en_route() {
        let (mut world, sys) = make_world();
        let stationed = world.special_forces.insert(SpecialForceUnit {
            is_alliance: true,
            on_mission: true,
            ..SpecialForceUnit::default()
        });
        let arriving = world.special_forces.insert(SpecialForceUnit {
            en_route: true,
            ..SpecialForceUnit::default()
        });
        world.systems[sys].special_forces = vec![stationed, arriving];

        let report = destroy_plain(&mut world, sys);

        assert_eq!(report.special_forces, vec![stationed]);
        assert!(!world.special_forces.contains_key(stationed));
        assert_eq!(world.systems[sys].special_forces, vec![arriving]);
    }

    /// `FUN_004ef2e0`: a character directly at the system, off a mission and
    /// not destroyed, goes adrift, major or minor, either side; one on a
    /// mission, aboard a fleet or elsewhere is untouched.
    #[test]
    fn characters_left_at_the_system_off_a_mission_go_adrift() {
        let (mut world, sys, elsewhere) = cascade_world(400, Faction::Alliance);
        let fleet = add_ds_fleet(&mut world, sys);
        let add = |world: &mut GameWorld, is_alliance, set: &dyn Fn(&mut Character)| {
            let mut character = Character {
                is_alliance,
                is_empire: !is_alliance,
                current_system: Some(sys),
                ..Character::default()
            };
            set(&mut character);
            world.characters.insert(character)
        };
        let minor = add(&mut world, true, &|_| {});
        let major = add(&mut world, false, &|c| c.is_major = true);
        let busy = add(&mut world, true, &|c| c.on_mission = true);
        let aboard = add(&mut world, false, &|c| c.current_fleet = Some(fleet));
        let away = add(&mut world, true, &|c| c.current_system = Some(elsewhere));

        let report = destroy_plain(&mut world, sys);

        assert_eq!(report.adrift, vec![minor, major]);
        for (key, adrift) in [
            (minor, true),
            (major, true),
            (busy, false),
            (aboard, false),
            (away, false),
        ] {
            assert_eq!(world.characters[key].adrift, adrift, "{key:?}");
            assert!(!world.characters[key].is_killed);
        }
    }

    /// `FUN_004f8710` destroys the system's production managers, and each
    /// one's existing-bit clear (`FUN_0052b7d0` -> `FUN_0052a2c0`) destroys
    /// every product it queued with reason `0x16`, whatever its destination:
    /// none is relocated or refunded.
    #[test]
    fn the_systems_own_queues_die_with_their_managers() {
        let (mut world, sys, refuge) = cascade_world(400, Faction::Alliance);
        let rebel = world.capital_ship_classes.insert(CapitalShipClass {
            is_alliance: true,
            ..CapitalShipClass::default()
        });
        let imperial = world
            .capital_ship_classes
            .insert(CapitalShipClass::default());
        let mut manufacturing = ManufacturingState::new();
        manufacturing.enqueue(sys, QueueItem::new(BuildableKind::CapitalShip(rebel), 10));
        manufacturing.enqueue(
            sys,
            QueueItem::new(BuildableKind::CapitalShip(imperial), 10)
                .delivered_to(Destination::System(refuge)),
        );
        let shield = BuildableKind::DefenseFacility(FacilityBuild {
            class: DatId::new(0x2400_0001),
            is_alliance: true,
        });
        manufacturing.enqueue(sys, QueueItem::new(shield, 10));

        let report = destroy(
            &mut world,
            sys,
            &mut TroopTransportState::default(),
            &mut manufacturing,
            &mut DeliveryState::new(),
        );

        assert_eq!(
            report.products,
            vec![
                BuildableKind::CapitalShip(rebel),
                BuildableKind::CapitalShip(imperial),
                shield,
            ]
        );
        assert!(report.autoscrapped.is_empty());
        assert_eq!(manufacturing.queued_at(sys), 0);
        assert!(world.systems[refuge].fleets.is_empty());
    }

    /// Another yard's products bound for the destroyed system
    /// (`death-star-strategic.md`, "Builds", case 2): a capital ship in the
    /// spare fleet is not walked and stays queued for it; a special force
    /// dies (`FUN_00503e20`); a regiment takes the request with its
    /// producer as the hint and keeps its queue slot, now bound for its new
    /// place (`FUN_00528890` keeps the manager's key).
    #[test]
    fn another_yards_products_bound_for_the_system_keep_their_slot_or_die_by_kind() {
        let (mut world, sys, refuge) = cascade_world(400, Faction::Alliance);
        let yard = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0003),
            name: "Kuat".into(),
            y: 300,
            control: ControlKind::Controlled(Faction::Empire),
            ..world.systems[sys].clone()
        });
        let ship = world.capital_ship_classes.insert(CapitalShipClass {
            is_alliance: true,
            ..CapitalShipClass::default()
        });
        let regiment = DatId::new(0x1000_0001);
        let commando = DatId::new(0x3c00_0001);
        for class in [regiment, commando] {
            world.buildable_classes.insert(
                class,
                crate::world::BuildableClass {
                    is_alliance: true,
                    ..Default::default()
                },
            );
        }
        let mut manufacturing = ManufacturingState::new();
        let bound = |kind| QueueItem::new(kind, 10).delivered_to(Destination::System(sys));
        manufacturing.enqueue(yard, bound(BuildableKind::CapitalShip(ship)));
        manufacturing.enqueue(yard, bound(BuildableKind::SpecialForce(commando)));
        manufacturing.enqueue(yard, bound(BuildableKind::Troop(regiment)));
        manufacturing.enqueue(yard, QueueItem::new(BuildableKind::Troop(regiment), 10));

        let report = destroy(
            &mut world,
            sys,
            &mut TroopTransportState::default(),
            &mut manufacturing,
            &mut DeliveryState::new(),
        );

        assert_eq!(report.products, vec![BuildableKind::SpecialForce(commando)]);
        assert_eq!(
            report.autoscrapped,
            vec![(
                Autoscrapped::Product(BuildableKind::Troop(regiment)),
                Relocation::Moved(refuge)
            )]
        );
        let queued = |area| {
            manufacturing
                .queue(yard, area)
                .map(|queue| {
                    queue
                        .items()
                        .iter()
                        .map(|item| (item.kind, item.destination))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default()
        };
        assert_eq!(
            queued(ProductionArea::Shipyard),
            vec![(BuildableKind::CapitalShip(ship), Some(Destination::System(sys)))]
        );
        assert_eq!(
            queued(ProductionArea::TrainingFacility),
            vec![
                (BuildableKind::Troop(regiment), Some(Destination::System(refuge))),
                (BuildableKind::Troop(regiment), None),
            ]
        );
    }

    /// An en route object is the destination's child, so it takes the
    /// request too (`FUN_004f8710`): stage 2 offers its transit origin
    /// (`death-star-strategic.md`, "The autoscrap request"). A regiment, a
    /// squadron and a delivery bound for the planet all go back.
    #[test]
    fn travellers_bound_for_the_destroyed_planet_go_back_to_their_origin() {
        let (mut world, sys, refuge) = cascade_world(400, Faction::Alliance);
        let mut transport = TroopTransportState::default();
        let regiment = troop(&mut world, refuge, true);
        transport
            .move_regiment(
                &mut world,
                &MovementState::new(),
                &std::collections::HashSet::new(),
                &ProductRoom::default(),
                regiment,
                crate::troop_transport::RegimentTarget::System(sys),
                1,
            )
            .unwrap();
        let fighter = world.fighter_classes.insert(FighterClass::default());
        world
            .squadron_transits
            .push(crate::troop_transport::SquadronTransit {
                class: fighter,
                is_alliance: true,
                origin: refuge,
                leg: RegimentLeg::Surface(sys),
                carrier: 0,
                arrival_tick: 30,
                via: None,
            });
        // A capital ship travels in the spare fleet, which the walk skips;
        // a facility product is the system's child.
        let shield = BuildableKind::DefenseFacility(FacilityBuild {
            class: DatId::new(0x2400_0001),
            is_alliance: true,
        });
        let mut deliveries = DeliveryState::new();
        deliveries.depart(
            &world,
            &[Departure {
                origin: refuge,
                destination: Destination::System(sys),
                tick: 1,
                kind: shield,
                carrier: 0,
            }],
        );

        let report = destroy(
            &mut world,
            sys,
            &mut transport,
            &mut ManufacturingState::new(),
            &mut deliveries,
        );

        assert_eq!(report.autoscrapped.len(), 3);
        assert!(report
            .autoscrapped
            .iter()
            .all(|(_, outcome)| *outcome == Relocation::Moved(refuge)));
        // Each is the refuge's child now, still on its way.
        let regiment_transit = &transport.transits()[0];
        assert_eq!(regiment_transit.leg, RegimentLeg::Surface(refuge));
        assert_eq!(world.squadron_transits[0].leg, RegimentLeg::Surface(refuge));
        assert_eq!(deliveries.en_route()[0].destination, refuge);
        assert!(world.systems[refuge].ground_units.is_empty());
    }

    /// A regiment or squadron relocated on its way keeps its arrival day T
    /// (`FUN_00556390.c:17-21`, `FUN_00556430.c:30`); on T `FUN_00556620`
    /// starts its next leg from the destroyed system, at the default speed
    /// for a regiment and the class speed for a squadron (`FUN_0055d8c0`,
    /// `FUN_00502f80`), and it lands at the end of that leg.
    #[test]
    fn a_relocated_regiment_or_squadron_keeps_its_arrival_day_then_travels_on() {
        let (mut world, sys, refuge) = cascade_world(400, Faction::Alliance);
        let mut transport = TroopTransportState::default();
        let regiment = troop(&mut world, refuge, true);
        transport
            .move_regiment(
                &mut world,
                &MovementState::new(),
                &std::collections::HashSet::new(),
                &ProductRoom::default(),
                regiment,
                crate::troop_transport::RegimentTarget::System(sys),
                1,
            )
            .unwrap();
        let due = transport.transits()[0].arrival_tick;
        assert!(due > 7);
        let class = FighterClass {
            is_alliance: true,
            ..FighterClass::default()
        };
        let speed = crate::movement::fighter_speed(&world, &class);
        let fighter = world.fighter_classes.insert(class);
        world
            .squadron_transits
            .push(crate::troop_transport::SquadronTransit {
                class: fighter,
                is_alliance: true,
                origin: refuge,
                leg: RegimentLeg::Surface(sys),
                carrier: 0,
                arrival_tick: 30,
                via: None,
            });

        destroy(
            &mut world,
            sys,
            &mut transport,
            &mut ManufacturingState::new(),
            &mut DeliveryState::new(),
        );
        assert_eq!(transport.transits()[0].arrival_tick, due);
        assert_eq!(world.squadron_transits[0].arrival_tick, 30);

        let position = |key: SystemKey| (world.systems[key].x, world.systems[key].y);
        let regiment_leg = crate::movement::transit_ticks_between(
            &world,
            position(sys),
            position(refuge),
            crate::movement::default_speed(&world),
        );
        let squadron_leg =
            crate::movement::transit_ticks_between(&world, position(sys), position(refuge), speed);
        transport.advance_transit(&mut world, due);
        assert_eq!(
            transport.transits()[0].arrival_tick,
            due + u64::from(regiment_leg)
        );
        transport.advance_transit(&mut world, 30);
        assert_eq!(
            world.squadron_transits[0].arrival_tick,
            30 + u64::from(squadron_leg)
        );

        let last = (due + u64::from(regiment_leg)).max(30 + u64::from(squadron_leg));
        let arrivals = transport.advance_transit(&mut world, last);
        assert_eq!(arrivals.arrived.len(), 1);
        assert_eq!(arrivals.squadrons_arrived.len(), 1);
        assert_eq!(world.systems[refuge].ground_units, vec![regiment]);
        assert_eq!(world.systems[refuge].squadron_count(true), 1);
    }

    /// The relocation of a delivery on its way leaves its timer, `+0x3c`
    /// and `+0x44` alone (`FUN_00556390.c:17-21`, `FUN_00556430.c:30`): it
    /// keeps its arrival day T, and on T `FUN_00556620.c:27-31` starts a
    /// second leg from the destroyed system to its new place, `FUN_0055d8c0`
    /// ticks at its speed (`death-star-strategic.md`, "A relocated unit's
    /// pending arrival").
    #[test]
    fn a_relocated_delivery_keeps_its_arrival_day_then_travels_on_from_the_destroyed_system() {
        let (mut world, sys, refuge) = cascade_world(400, Faction::Alliance);
        let shield = BuildableKind::DefenseFacility(FacilityBuild {
            class: DatId::new(0x2400_0001),
            is_alliance: true,
        });
        let mut deliveries = DeliveryState::new();
        deliveries.depart(
            &world,
            &[Departure {
                origin: refuge,
                destination: Destination::System(sys),
                tick: 1,
                kind: shield,
                carrier: 0,
            }],
        );
        let leg = u64::from(crate::delivery::transit_days(&world, shield, refuge, sys));
        let due = 1 + leg;
        assert!(due > 7);

        destroy(
            &mut world,
            sys,
            &mut TroopTransportState::default(),
            &mut ManufacturingState::new(),
            &mut deliveries,
        );
        assert_eq!(deliveries.en_route()[0].arrival_tick, due);

        let tick = |n| [crate::tick::TickEvent { tick: n }];
        let on_due = deliveries.advance(&world, &tick(due));
        assert!(on_due.arrivals.is_empty() && on_due.lost.is_empty());
        let second = u64::from(crate::delivery::transit_days(&world, shield, sys, refuge));
        assert_eq!(deliveries.en_route()[0].arrival_tick, due + second);

        let arrived = deliveries.advance(&world, &tick(due + second));
        assert_eq!(arrived.arrivals.len(), 1);
        assert_eq!(arrived.arrivals[0].system, refuge);
        assert!(deliveries.en_route().is_empty());
    }

    /// With no system of its side left, the request destroys a travelling
    /// regiment with reason 2 (`FUN_00515480`), refunding nothing.
    #[test]
    fn a_traveller_with_no_place_of_its_side_is_destroyed() {
        let (mut world, sys, origin) = cascade_world(400, Faction::Alliance);
        let mut transport = TroopTransportState::default();
        let regiment = troop(&mut world, origin, true);
        transport
            .move_regiment(
                &mut world,
                &MovementState::new(),
                &std::collections::HashSet::new(),
                &ProductRoom::default(),
                regiment,
                crate::troop_transport::RegimentTarget::System(sys),
                1,
            )
            .unwrap();
        world.systems[origin].control = ControlKind::Controlled(Faction::Empire);

        let report = destroy(
            &mut world,
            sys,
            &mut transport,
            &mut ManufacturingState::new(),
            &mut DeliveryState::new(),
        );

        assert!(transport.transits().is_empty());
        assert!(!world.troops.contains_key(regiment));
        assert_eq!(
            report.autoscrapped,
            vec![(Autoscrapped::Regiment(regiment), Relocation::Destroyed(2))]
        );
    }

    /// `FUN_00548e90` with the battle bit clear: each adrift character of
    /// either side leaves on the hidden Adrift mission (`FUN_00547880`) for
    /// its side's nearest system (`FUN_00543040`), its flag cleared.
    #[test]
    fn a_cleared_battle_bit_sends_each_sides_adrift_characters_on_a_hidden_trip() {
        let (mut world, sys, rebel_refuge) = cascade_world(400, Faction::Alliance);
        let imperial_refuge = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0003),
            name: "Kuat".into(),
            y: 300,
            control: ControlKind::Controlled(Faction::Empire),
            ..world.systems[sys].clone()
        });
        let rebel = world.characters.insert(Character {
            is_alliance: true,
            current_system: Some(sys),
            ..Character::default()
        });
        let imperial = world.characters.insert(Character {
            is_empire: true,
            current_system: Some(sys),
            ..Character::default()
        });
        destroy_plain(&mut world, sys);
        let mut missions = MissionState::new();

        let started = sweep_adrift(
            &mut world,
            &mut missions,
            &TroopTransportState::default(),
            &MovementState::new(),
            sys,
            false,
            9,
        );

        assert_eq!(started.len(), 2);
        let trips: Vec<_> = missions
            .missions()
            .iter()
            .map(|mission| (mission.kind, mission.target_system, mission.team.clone()))
            .collect();
        assert_eq!(
            trips,
            vec![
                (
                    crate::missions::MissionKind::Move,
                    rebel_refuge,
                    vec![MissionMember::Character(rebel)]
                ),
                (
                    crate::missions::MissionKind::Move,
                    imperial_refuge,
                    vec![MissionMember::Character(imperial)]
                ),
            ]
        );
        for key in [rebel, imperial] {
            let character = &world.characters[key];
            assert!(!character.adrift && character.on_mission && character.on_hidden_mission);
        }
    }

    /// With the battle bit set, the sweep only gathers adrift characters
    /// aboard the system's fleets into the system (slot `+0xa8`).
    #[test]
    fn a_set_battle_bit_only_gathers_adrift_characters_into_the_system() {
        for is_alliance in [false, true] {
            let (mut world, sys, _) = cascade_world(400, Faction::Alliance);
            let fleet = world.fleets.insert(crate::world::Fleet {
                location: sys,
                is_alliance,
                ..crate::world::Fleet::default()
            });
            world.systems[sys].fleets.push(fleet);
            let aboard = world.characters.insert(Character {
                is_alliance,
                is_empire: !is_alliance,
                current_system: Some(sys),
                current_fleet: Some(fleet),
                adrift: true,
                ..Character::default()
            });
            world.fleets[fleet].characters.push(aboard);
            world.systems[sys].is_destroyed = true;
            let mut missions = MissionState::new();

            let started = sweep_adrift(
                &mut world,
                &mut missions,
                &TroopTransportState::default(),
                &MovementState::new(),
                sys,
                true,
                9,
            );

            assert!(started.is_empty() && missions.is_empty());
            assert!(world.fleets[fleet].characters.is_empty());
            assert_eq!(world.characters[aboard].current_fleet, None);
            assert!(world.characters[aboard].adrift);
        }
    }

    fn sweep(world: &mut GameWorld, missions: &mut MissionState, sys: SystemKey) -> Vec<u64> {
        sweep_adrift(
            world,
            missions,
            &TroopTransportState::default(),
            &MovementState::new(),
            sys,
            false,
            9,
        )
    }

    fn stranded_at(world: &mut GameWorld, sys: SystemKey, is_alliance: bool, captive: bool) -> CharacterKey {
        world.characters.insert(Character {
            is_alliance,
            is_empire: !is_alliance,
            current_system: Some(sys),
            adrift: true,
            is_captive: captive,
            ..Character::default()
        })
    }

    /// `FUN_00549580`: a member the retreat chain gives no destination is
    /// captured with no captor (slot `+0x20c`, status 0), which gives it its
    /// captor's side bits (`FUN_004f1a80` → `FUN_004eec60`), and the chain
    /// runs again on that side: it leaves as a prisoner for the other
    /// side's system (`jail.md`, "The second way into jail").
    #[test]
    fn a_character_with_no_place_of_its_side_leaves_captive_for_the_other_sides() {
        for is_alliance in [true, false] {
            let other = crate::mission_detection::faction_of(!is_alliance);
            let (mut world, sys, refuge) = cascade_world(400, other);
            world.systems[sys].is_destroyed = true;
            let stranded = stranded_at(&mut world, sys, is_alliance, false);
            let mut missions = MissionState::new();

            let started = sweep(&mut world, &mut missions, sys);

            let character = &world.characters[stranded];
            assert!(character.is_captive && character.captured_by.is_none(), "{is_alliance}");
            assert_eq!((character.abode, started.len()), (None, 1), "{is_alliance}");
            let trip = &missions.missions()[0];
            assert_eq!(
                (trip.target_system, trip.team.clone()),
                (refuge, vec![MissionMember::Character(stranded)])
            );
        }
    }

    /// `FUN_00549580` with no destination on either side: the member goes
    /// to the jail of its side bits, its captor's, `0x98000440` for side 1
    /// and `0x98000480` otherwise, out of every system (`jail.md`).
    #[test]
    fn a_character_with_no_place_of_either_side_is_held_in_the_other_sides_jail() {
        for (is_alliance, jail) in [
            (true, crate::world::Abode::EmpireJail),
            (false, crate::world::Abode::AllianceJail),
        ] {
            let (mut world, sys, _) = cascade_world(400, Faction::Neutral);
            let stranded = stranded_at(&mut world, sys, is_alliance, false);
            world.characters[stranded].adrift = false;
            destroy_plain(&mut world, sys);
            let mut missions = MissionState::new();

            let started = sweep(&mut world, &mut missions, sys);

            assert!(started.is_empty() && missions.is_empty());
            let character = &world.characters[stranded];
            assert_eq!(character.abode, Some(jail), "{is_alliance}");
            assert!(character.is_captive && character.captured_by.is_none());
            assert_eq!(character.current_system, None);
            assert!(!character.adrift && !character.on_mission);
        }
    }

    /// `FUN_00556390` reparents the jailed character into the abode, out of
    /// the fleet that held it, and the capture leaves no captor
    /// (`FUN_00549580`, slot `+0x20c`).
    #[test]
    fn a_jailed_character_leaves_its_fleets_roster_for_the_other_sides_jail() {
        for is_alliance in [true, false] {
            let (mut world, sys) = make_world();
            let fleet = world.fleets.insert(Fleet {
                location: sys,
                is_alliance,
                ..Fleet::default()
            });
            let key = world.characters.insert(Character {
                is_alliance,
                is_empire: !is_alliance,
                current_fleet: Some(fleet),
                ..Character::default()
            });
            world.fleets[fleet].characters.push(key);

            hold_in_jail(&mut world, key, 7);

            assert!(world.fleets[fleet].characters.is_empty(), "{is_alliance}");
            let character = &world.characters[key];
            assert_eq!(
                character.held_abode(),
                Some(crate::world::Abode::jail_of(!is_alliance)),
                "{is_alliance}"
            );
            assert_eq!(character.capture_tick, Some(7));
        }
    }

    /// A prisoner groups on its captor's side bits; with no place there,
    /// `FUN_00549580` frees it (slot `+0x218`, status 2, its own side back)
    /// and the chain finds its own side's system.
    #[test]
    fn a_prisoner_with_no_place_of_its_captors_side_is_freed_for_its_own() {
        for is_alliance in [true, false] {
            let side = crate::mission_detection::faction_of(is_alliance);
            let (mut world, sys, refuge) = cascade_world(400, side);
            world.systems[sys].is_destroyed = true;
            let captive = stranded_at(&mut world, sys, is_alliance, true);
            let mut missions = MissionState::new();

            let started = sweep(&mut world, &mut missions, sys);

            let character = &world.characters[captive];
            assert!(!character.is_captive, "{is_alliance}");
            assert!(!character.adrift && character.on_hidden_mission);
            assert_eq!(started.len(), 1);
            let trip = &missions.missions()[0];
            assert_eq!(
                (trip.target_system, trip.team.clone()),
                (refuge, vec![MissionMember::Character(captive)])
            );
        }
    }

    /// `FUN_00549230` → `FUN_00549850`: a group of prisoners given a place
    /// on their captor's side is freed (slot `+0x218`); the next pass finds
    /// no place of their own side, so `FUN_00549580` captures each again
    /// and it leaves as a prisoner for its captor's system.
    #[test]
    fn a_group_of_prisoners_with_a_place_of_their_captors_side_returns_there_captive() {
        for is_alliance in [true, false] {
            let other = crate::mission_detection::faction_of(!is_alliance);
            let (mut world, sys, refuge) = cascade_world(400, other);
            world.systems[sys].is_destroyed = true;
            let captive = stranded_at(&mut world, sys, is_alliance, true);
            let mut missions = MissionState::new();

            let started = sweep(&mut world, &mut missions, sys);

            let character = &world.characters[captive];
            assert!(character.is_captive && !character.adrift, "{is_alliance}");
            assert_eq!(started.len(), 1);
            assert_eq!(missions.missions()[0].target_system, refuge);
        }
    }

    /// An escape's release raises the autoscrap request, whose event
    /// `0x307` sweeps the escapee's location at once (`FUN_00515480` →
    /// `FUN_00548da0`): freed at its captor's system, it leaves for its own
    /// side's.
    #[test]
    fn an_escapee_at_its_captors_system_leaves_for_its_own_sides() {
        for is_alliance in [true, false] {
            let own = crate::mission_detection::faction_of(is_alliance);
            let (mut world, sys, refuge) = cascade_world(400, own);
            world.systems[sys].control =
                ControlKind::Controlled(crate::mission_detection::faction_of(!is_alliance));
            let escapee = stranded_at(&mut world, sys, is_alliance, true);
            world.characters[escapee].adrift = false;
            world.characters[escapee].escape_due = Some(9);
            let mut missions = MissionState::new();

            let started = free_escapee(
                &mut world,
                &mut missions,
                &TroopTransportState::default(),
                &MovementState::new(),
                &crate::blockade::BlockadeState::new(),
                escapee,
                9,
            );

            let character = &world.characters[escapee];
            assert!(!character.is_captive && character.escape_due.is_none(), "{is_alliance}");
            assert_eq!(started.len(), 1, "{is_alliance}");
            assert_eq!(missions.missions()[0].target_system, refuge);
        }
    }

    // ── A holder change moves the old holder's characters off ───────────────

    /// `sys` (Alderaan) held by `old`, with a refuge of `old`'s at
    /// (400, 100); the holder then changes to `new` (`FUN_0050a300` ->
    /// `FUN_00510d70`).
    fn capture(world: &mut GameWorld, sys: SystemKey, new: ControlKind) {
        let old = world.systems[sys].control;
        world.systems[sys].control = new;
        crate::support_shift::control_changed(world, sys, old, (false, false), 1);
    }

    fn sweep_all(
        world: &mut GameWorld,
        missions: &mut MissionState,
        blockade: &crate::blockade::BlockadeState,
    ) -> Vec<u64> {
        sweep_requests(
            world,
            missions,
            &TroopTransportState::default(),
            &MovementState::new(),
            blockade,
            9,
        )
        .into_iter()
        .flat_map(|(_, started)| started)
        .collect()
    }

    /// The owner change walks the system's children (`FUN_00510820` ->
    /// `FUN_004fae40`, slot `+0xb0`): a character off a mission whose side
    /// differs from the system's new one takes the autoscrap request
    /// (`FUN_004ef260` -> `FUN_004f78e0(1)`), whose event `0x307` sweeps its
    /// location (`FUN_00515480` -> `FUN_00545df0` -> `FUN_00548da0`): it
    /// leaves for the nearest system of its side (`FUN_00543040`). The
    /// original: three Alliance officers at Talus on day 220 were at
    /// Alliance-held Corfai on day 277, after Imperial Fleet 1 took Talus
    /// on day 272.
    #[test]
    fn a_holder_change_sends_the_old_holders_characters_to_their_nearest_system() {
        for is_alliance in [true, false] {
            let old = crate::mission_detection::faction_of(is_alliance);
            let new = crate::mission_detection::faction_of(!is_alliance);
            let (mut world, sys, refuge) = cascade_world(400, old);
            world.systems[sys].control = ControlKind::Controlled(old);
            let officer = world.characters.insert(Character {
                is_alliance,
                is_empire: !is_alliance,
                current_system: Some(sys),
                ..Character::default()
            });
            let away = world.characters.insert(Character {
                is_alliance,
                is_empire: !is_alliance,
                current_system: Some(sys),
                on_mission: true,
                ..Character::default()
            });
            let captor = world.characters.insert(Character {
                is_alliance: !is_alliance,
                is_empire: is_alliance,
                current_system: Some(sys),
                ..Character::default()
            });
            capture(&mut world, sys, ControlKind::Controlled(new));
            let mut missions = MissionState::new();
            let started = sweep_all(
                &mut world,
                &mut missions,
                &crate::blockade::BlockadeState::new(),
            );
            assert_eq!(started.len(), 1, "{is_alliance}");
            assert_eq!(missions.missions()[0].target_system, refuge);
            assert_eq!(
                missions.missions()[0].team,
                vec![MissionMember::Character(officer)]
            );
            assert!(!world.characters[away].adrift, "{is_alliance}");
            assert!(!world.characters[captor].adrift, "{is_alliance}");
        }
    }

    /// `FUN_004ef260.c:13-21`: a prisoner, whose side bits are its
    /// captor's, is first released (slot `+0x218`, `FUN_004ef5b0`); one now
    /// of the system's side stays there, free and unflagged.
    #[test]
    fn a_holder_change_frees_the_old_holders_prisoners_of_the_new_side() {
        for is_alliance in [true, false] {
            let old = crate::mission_detection::faction_of(!is_alliance);
            let new = crate::mission_detection::faction_of(is_alliance);
            let (mut world, sys, _) = cascade_world(400, old);
            world.systems[sys].control = ControlKind::Controlled(old);
            let prisoner = stranded_at(&mut world, sys, is_alliance, true);
            world.characters[prisoner].adrift = false;
            world.characters[prisoner].capture_tick = Some(3);
            capture(&mut world, sys, ControlKind::Controlled(new));
            let character = &world.characters[prisoner];
            assert!(
                !character.is_captive && character.capture_tick.is_none(),
                "{is_alliance}"
            );
            assert!(!character.adrift, "{is_alliance}");
            assert_eq!(character.current_system, Some(sys));
        }
    }

    /// The sweep skips its grouping while the system's battle bit is set
    /// (`FUN_00548e90`): the flagged character waits there for the bit's
    /// change (event `0x304`).
    #[test]
    fn a_holder_change_in_a_battle_leaves_the_flagged_characters_waiting() {
        for is_alliance in [true, false] {
            let old = crate::mission_detection::faction_of(is_alliance);
            let new = crate::mission_detection::faction_of(!is_alliance);
            let (mut world, sys, _) = cascade_world(400, old);
            world.systems[sys].control = ControlKind::Controlled(old);
            let officer = world.characters.insert(Character {
                is_alliance,
                is_empire: !is_alliance,
                current_system: Some(sys),
                ..Character::default()
            });
            capture(&mut world, sys, ControlKind::Controlled(new));
            let mut blockade = crate::blockade::BlockadeState::new();
            for fleet_alliance in [true, false] {
                let class = world.capital_ship_classes.insert(CapitalShipClass {
                    hull: 100,
                    ..CapitalShipClass::default()
                });
                let fleet = world.fleets.insert(crate::world::Fleet {
                    location: sys,
                    capital_ships: vec![ShipInstance::new(class, 100, fleet_alliance)],
                    is_alliance: fleet_alliance,
                    ..crate::world::Fleet::default()
                });
                world.systems[sys].fleets.push(fleet);
            }
            blockade.refresh_battle(&world, sys);
            assert!(blockade.in_battle(sys));
            let mut missions = MissionState::new();
            assert!(sweep_all(&mut world, &mut missions, &blockade).is_empty());
            assert!(world.characters[officer].adrift, "{is_alliance}");
        }
    }

    /// Slot `+0xb0` of a special force (`FUN_00503d90`): it takes the
    /// system's new side when its class serves that side (slot `+0x50`,
    /// `FUN_004f27d0`: class `+0x40`/`+0x44`), and is removed otherwise
    /// (`+0xac(3)`).
    #[test]
    fn a_holder_change_turns_or_removes_the_special_forces_there() {
        for is_alliance in [true, false] {
            let old = crate::mission_detection::faction_of(is_alliance);
            let new = crate::mission_detection::faction_of(!is_alliance);
            let (mut world, sys, _) = cascade_world(400, old);
            world.systems[sys].control = ControlKind::Controlled(old);
            let unit = |world: &mut GameWorld, raw: u32, both: bool| {
                let class = DatId::new(raw);
                world.buildable_classes.insert(
                    class,
                    crate::world::BuildableClass {
                        is_alliance: is_alliance || both,
                        is_empire: !is_alliance || both,
                        ..Default::default()
                    },
                );
                let key = world.special_forces.insert(SpecialForceUnit {
                    class_dat_id: class,
                    is_alliance,
                    ..SpecialForceUnit::default()
                });
                world.systems[sys].special_forces.push(key);
                key
            };
            let own = unit(&mut world, 0x3c00_0001, false);
            let both = unit(&mut world, 0x3c00_0002, true);
            let theirs = unit(&mut world, 0x3c00_0003, false);
            world.buildable_classes.insert(
                DatId::new(0x3c00_0003),
                crate::world::BuildableClass {
                    is_alliance: !is_alliance,
                    is_empire: is_alliance,
                    ..Default::default()
                },
            );
            let away = unit(&mut world, 0x3c00_0004, false);
            world.special_forces[away].on_mission = true;
            capture(&mut world, sys, ControlKind::Controlled(new));
            assert!(!world.special_forces.contains_key(own), "{is_alliance}");
            assert_eq!(world.systems[sys].special_forces, vec![both, theirs, away]);
            assert_eq!(world.special_forces[both].is_alliance, !is_alliance);
            assert_eq!(world.special_forces[theirs].is_alliance, !is_alliance);
            assert_eq!(world.special_forces[away].is_alliance, is_alliance);
            // A neutral holder keeps only a class that serves both sides
            // (FUN_004f27d0 with side 3).
            capture(&mut world, sys, ControlKind::Uncontrolled);
            assert_eq!(world.systems[sys].special_forces, vec![both, away]);
        }
    }

    /// Only a flagged character the system holds itself, off a mission and
    /// alive, has its location swept (`FUN_00548da0`); an adrift one aboard
    /// a fleet there waits for a sweep of that system.
    #[test]
    fn the_requests_sweep_only_systems_holding_a_flagged_character() {
        let (mut world, sys, _) = cascade_world(400, Faction::Alliance);
        let fleet = world.fleets.insert(crate::world::Fleet {
            location: sys,
            is_alliance: true,
            ..crate::world::Fleet::default()
        });
        world.systems[sys].fleets.push(fleet);
        let aboard = world.characters.insert(Character {
            is_alliance: true,
            current_system: Some(sys),
            current_fleet: Some(fleet),
            adrift: true,
            ..Character::default()
        });
        world.fleets[fleet].characters.push(aboard);
        for (adrift, on_mission, is_killed) in [
            (false, false, false),
            (true, true, false),
            (true, false, true),
        ] {
            world.characters.insert(Character {
                is_alliance: true,
                current_system: Some(sys),
                adrift,
                on_mission,
                is_killed,
                ..Character::default()
            });
        }
        let mut missions = MissionState::new();
        let blockade = crate::blockade::BlockadeState::new();
        assert!(sweep_all(&mut world, &mut missions, &blockade).is_empty());
        assert_eq!(world.characters[aboard].current_fleet, Some(fleet));
    }

    /// hyp: a jailed escapee is swept from `FUN_005431e0`'s origin away
    /// from any system, Yavin (`0x92000121`) for side 1 and Coruscant
    /// (`0x90000109`) otherwise; at its own side's intact system it stays
    /// there unflagged (`FUN_0054c7d0`). The seat carries its id as
    /// `load_game_data` stores it, the SYSTEMSD index alone.
    #[test]
    fn a_jailed_escapee_is_swept_from_its_sides_seat() {
        for (is_alliance, seat) in [(true, YAVIN), (false, crate::victory::CORUSCANT)] {
            let own = crate::mission_detection::faction_of(is_alliance);
            let (mut world, sys, _) = cascade_world(400, own);
            world.systems[sys].dat_id = DatId::new(DatId::new(seat).index());
            world.systems[sys].control = ControlKind::Controlled(own);
            let escapee = world.characters.insert(Character {
                is_alliance,
                is_empire: !is_alliance,
                is_captive: true,
                abode: Some(crate::world::Abode::jail_of(!is_alliance)),
                ..Character::default()
            });
            let mut missions = MissionState::new();

            let started = free_escapee(
                &mut world,
                &mut missions,
                &TroopTransportState::default(),
                &MovementState::new(),
                &crate::blockade::BlockadeState::new(),
                escapee,
                9,
            );

            let character = &world.characters[escapee];
            assert!(started.is_empty(), "{is_alliance}");
            assert_eq!((character.abode, character.current_system), (None, Some(sys)));
            assert!(!character.is_captive && !character.adrift);
        }
    }

    fn record(raw: u32, return_policy: u8) -> crate::world::MissionRecord {
        crate::world::MissionRecord {
            dat_id: DatId::new(raw),
            timer_min_days: 0,
            timer_spread_days: 0,
            repeats: false,
            hidden: false,
            detection_phases: false,
            can_resign: false,
            return_policy,
            rules: crate::world::MissionTargetRules::default(),
            members: crate::world::MissionMemberRules::default(),
        }
    }

    fn ended(
        kind: crate::missions::MissionKind,
        is_alliance: bool,
        team: Vec<MissionMember>,
        captured: Vec<MissionMember>,
        origin: SystemKey,
    ) -> crate::missions::MissionEnded {
        crate::missions::MissionEnded {
            mission_id: 1,
            kind,
            faction: if is_alliance {
                MissionFaction::Alliance
            } else {
                MissionFaction::Empire
            },
            end_code: 1,
            tick: 9,
            ordered_tick: 0,
            rolls: 1,
            team,
            captured,
            origin: Some(origin),
            set_out_from: Some(MoveDestination::System(origin)),
            target_system: origin,
            lead: None,
            result: 0,
        }
    }

    fn return_after(world: &mut GameWorld, missions: &mut MissionState, ended: &crate::missions::MissionEnded) -> Vec<u64> {
        return_members(
            world,
            missions,
            &TroopTransportState::default(),
            &MovementState::new(),
            &crate::blockade::BlockadeState::new(),
            ended,
            9,
        )
    }

    /// `FUN_00521220` under policy 1 (MISSNSD `+0x68`, Diplomacy): a member
    /// standing in another side's system goes home by the hidden Return to
    /// the container it set out from (`FUN_00542ee0`, `+0x70`), its captive
    /// with it (`FUN_00548910`; `mission-return.md`).
    #[test]
    fn a_member_at_another_sides_system_goes_home_with_its_captive() {
        use crate::missions::MissionKind;
        for is_alliance in [true, false] {
            let own = crate::mission_detection::faction_of(is_alliance);
            let (mut world, target, home) = cascade_world(400, own);
            world.systems[target].control = ControlKind::Uncontrolled;
            world.mission_records = vec![record(0x5100_0010, 1)];
            let envoy = stranded_at(&mut world, target, is_alliance, false);
            world.characters[envoy].adrift = false;
            let captive = stranded_at(&mut world, target, !is_alliance, true);
            world.characters[captive].adrift = false;
            let mut missions = MissionState::new();

            let started = return_after(
                &mut world,
                &mut missions,
                &ended(
                    MissionKind::Diplomacy,
                    is_alliance,
                    vec![MissionMember::Character(envoy)],
                    vec![MissionMember::Character(captive)],
                    home,
                ),
            );

            assert_eq!(started.len(), 1, "{is_alliance}");
            let trip = &missions.missions()[0];
            assert_eq!(trip.target_system, home);
            assert_eq!(
                trip.team,
                vec![MissionMember::Character(envoy), MissionMember::Character(captive)]
            );
        }
    }

    /// Policy 1: `+0x70` is the container the member joined from
    /// (`FUN_00536490`), a fleet too. `FUN_00542ee0` sends it back there
    /// when the container passes the Move test for its side (`FUN_005543c0`
    /// -> `FUN_00553410`: its side, not en route), else down the retreat
    /// chain (`FUN_00543040`; `mission-return.md` section 3).
    /// A special-force product deployed into a destroyed system has no leg
    /// (`FUN_00545180`); its presence check `FUN_00556710` calls
    /// `FUN_00503e20(0)`, whose autoscrap request reaches the sweep through
    /// event `0x307`: the nearest system of its side takes it, else it dies
    /// with reason 9 (`FUN_00549580.c:56-60`; `death-star-strategic.md`,
    /// "Arrival at a destroyed system").
    #[test]
    fn a_special_force_deployed_into_a_destroyed_system_moves_on_or_dies() {
        for is_alliance in [true, false] {
            let own = crate::mission_detection::faction_of(is_alliance);
            let other = crate::mission_detection::faction_of(!is_alliance);
            for (refuge_side, survives) in [(own, true), (other, false)] {
                let (mut world, sys, refuge) = cascade_world(400, refuge_side);
                world.systems[sys].is_destroyed = true;
                let unit = world.special_forces.insert(SpecialForceUnit {
                    is_alliance,
                    ..SpecialForceUnit::default()
                });
                world.systems[sys].special_forces.push(unit);
                // One on a mission is not the presence check's (FUN_00556710
                // runs only for a deployment); it stays.
                let busy = world.special_forces.insert(SpecialForceUnit {
                    is_alliance,
                    on_mission: true,
                    ..SpecialForceUnit::default()
                });
                world.systems[sys].special_forces.push(busy);

                deploy_special_forces(
                    &mut world,
                    &TroopTransportState::default(),
                    &MovementState::new(),
                    sys,
                );

                assert_eq!(world.systems[sys].special_forces, vec![busy]);
                assert_eq!(world.special_forces.contains_key(unit), survives);
                assert_eq!(
                    world.systems[refuge].special_forces.contains(&unit),
                    survives,
                    "{is_alliance} {refuge_side:?}"
                );
            }
        }
    }

    /// `FUN_00549580` on a special force the chain gives no destination:
    /// slot `+0x20c` is `FUN_00503eb0`, which destroys it with reason 9
    /// (`+0xac(9)`, special-force vtable `0x0065e160`); it is never jailed.
    #[test]
    fn a_special_force_with_no_place_to_go_home_to_is_destroyed() {
        use crate::missions::MissionKind;
        for is_alliance in [true, false] {
            let other = crate::mission_detection::faction_of(!is_alliance);
            let (mut world, target, home) = cascade_world(400, other);
            world.systems[target].control = ControlKind::Uncontrolled;
            world.mission_records = vec![record(0x5100_0010, 1)];
            let unit = world.special_forces.insert(SpecialForceUnit {
                is_alliance,
                ..SpecialForceUnit::default()
            });
            world.systems[target].special_forces.push(unit);
            let mut missions = MissionState::new();

            let started = return_after(
                &mut world,
                &mut missions,
                &ended(
                    MissionKind::Diplomacy,
                    is_alliance,
                    vec![MissionMember::SpecialForce(unit)],
                    Vec::new(),
                    home,
                ),
            );

            assert!(started.is_empty(), "{is_alliance}");
            assert!(!world.special_forces.contains_key(unit), "{is_alliance}");
            assert!(world.systems[target].special_forces.is_empty(), "{is_alliance}");
        }
    }

    /// `FUN_005543c0` -> `FUN_00553410` refuses a `+0x70` fleet of the other
    /// side (`1/0x24`) or en route (`1/0x21`) and a `+0x70` system that is
    /// destroyed or held by the other side; the chain then decides.
    #[test]
    fn a_container_the_move_test_refuses_sends_the_member_down_the_chain() {
        use crate::missions::MissionKind;
        for is_alliance in [true, false] {
            let own = crate::mission_detection::faction_of(is_alliance);
            let (mut world, target, home) = cascade_world(400, own);
            world.systems[target].control = ControlKind::Uncontrolled;
            world.mission_records = vec![record(0x5100_0010, 1)];
            let far = world.systems.insert(System {
                x: 900,
                control: ControlKind::Controlled(own),
                ..world.systems[home].clone()
            });
            let envoy = stranded_at(&mut world, target, is_alliance, false);
            world.characters[envoy].adrift = false;
            let fleet = world.fleets.insert(Fleet {
                location: far,
                is_alliance,
                ..Fleet::default()
            });
            world.systems[far].fleets.push(fleet);
            let mut sent = ended(
                MissionKind::Diplomacy,
                is_alliance,
                vec![MissionMember::Character(envoy)],
                Vec::new(),
                home,
            );
            let goes_to = |world: &mut GameWorld,
                           sent: &crate::missions::MissionEnded,
                           movement: &MovementState| {
                let mut missions = MissionState::new();
                return_members(
                    world,
                    &mut missions,
                    &TroopTransportState::default(),
                    movement,
                    &crate::blockade::BlockadeState::new(),
                    sent,
                    9,
                );
                missions
                    .missions()
                    .front()
                    .map(|trip| (trip.target_system, trip.target_object))
            };
            let still = MovementState::new();
            let mut away = MovementState::new();
            assert!(away.order(fleet, far, home, 5));

            sent.set_out_from = Some(MoveDestination::Fleet(fleet));
            assert_eq!(
                goes_to(&mut world, &sent, &still),
                Some((far, Some(MissionTarget::Fleet(fleet)))),
                "{is_alliance}"
            );
            // En route: the chain's nearest own system, home.
            assert_eq!(goes_to(&mut world, &sent, &away), Some((home, None)), "{is_alliance}");
            world.fleets[fleet].is_alliance = !is_alliance;
            assert_eq!(goes_to(&mut world, &sent, &still), Some((home, None)), "{is_alliance}");

            sent.set_out_from = Some(MoveDestination::System(far));
            assert_eq!(goes_to(&mut world, &sent, &still), Some((far, None)), "{is_alliance}");
            world.systems[far].is_destroyed = true;
            assert_eq!(goes_to(&mut world, &sent, &still), Some((home, None)), "{is_alliance}");
            world.systems[far].is_destroyed = false;
            let enemy = crate::mission_detection::faction_of(!is_alliance);
            world.systems[far].control = ControlKind::Controlled(enemy);
            assert_eq!(goes_to(&mut world, &sent, &still), Some((home, None)), "{is_alliance}");
        }
    }

    /// Policy 2 routes every member (`FUN_00521220.c:26-33`); one already
    /// standing where the chain sends it starts no Return.
    #[test]
    fn a_member_already_where_it_would_return_starts_no_trip() {
        use crate::missions::MissionKind;
        for is_alliance in [true, false] {
            let own = crate::mission_detection::faction_of(is_alliance);
            let (mut world, _, home) = cascade_world(400, own);
            world.mission_records = vec![record(0x5100_0010, 2)];
            let envoy = stranded_at(&mut world, home, is_alliance, false);
            world.characters[envoy].adrift = false;
            let mut missions = MissionState::new();

            let started = return_after(
                &mut world,
                &mut missions,
                &ended(
                    MissionKind::Diplomacy,
                    is_alliance,
                    vec![MissionMember::Character(envoy)],
                    Vec::new(),
                    home,
                ),
            );

            assert!(started.is_empty(), "{is_alliance}");
        }
    }

    #[test]
    fn a_member_that_set_out_from_a_fleet_goes_home_to_that_fleet() {
        use crate::missions::MissionKind;
        for is_alliance in [true, false] {
            let own = crate::mission_detection::faction_of(is_alliance);
            let (mut world, target, home) = cascade_world(400, own);
            world.systems[target].control = ControlKind::Uncontrolled;
            world.mission_records = vec![record(0x5100_0010, 1)];
            let envoy = stranded_at(&mut world, target, is_alliance, false);
            world.characters[envoy].adrift = false;
            let fleet = world.fleets.insert(Fleet {
                location: home,
                is_alliance,
                ..Fleet::default()
            });
            world.systems[home].fleets.push(fleet);
            let mut sent = ended(
                MissionKind::Diplomacy,
                is_alliance,
                vec![MissionMember::Character(envoy)],
                Vec::new(),
                home,
            );
            sent.set_out_from = Some(MoveDestination::Fleet(fleet));
            let mut missions = MissionState::new();

            assert_eq!(return_after(&mut world, &mut missions, &sent).len(), 1);
            let trip = &missions.missions()[0];
            assert_eq!(
                (trip.target_system, trip.target_object),
                (home, Some(MissionTarget::Fleet(fleet))),
                "{is_alliance}"
            );

            // A fleet that is gone no longer resolves (FUN_005543c0): the
            // retreat chain picks the member's own system instead.
            world.fleets.remove(fleet);
            world.systems[home].fleets.retain(|&key| key != fleet);
            let mut missions = MissionState::new();
            assert_eq!(return_after(&mut world, &mut missions, &sent).len(), 1);
            let trip = &missions.missions()[0];
            assert_eq!((trip.target_system, trip.target_object), (home, None), "{is_alliance}");
        }
    }

    /// Policy 1 with the member's own side holding its system: `route` is
    /// 0 and it stays (`FUN_00548120`, slot `+0x1d0` with flag 0); policy 0
    /// (Move) never routes.
    #[test]
    fn a_member_at_its_own_sides_system_or_off_a_move_stays() {
        use crate::missions::MissionKind;
        for is_alliance in [true, false] {
            let own = crate::mission_detection::faction_of(is_alliance);
            let (mut world, target, home) = cascade_world(400, own);
            world.systems[target].control = ControlKind::Controlled(own);
            world.mission_records = vec![record(0x5500_0016, 1), record(0x4100_0001, 0)];
            let recruiter = stranded_at(&mut world, target, is_alliance, false);
            let team = vec![MissionMember::Character(recruiter)];
            let mut missions = MissionState::new();

            let stayed = return_after(
                &mut world,
                &mut missions,
                &ended(MissionKind::Recruitment, is_alliance, team.clone(), Vec::new(), home),
            );
            world.systems[target].control = ControlKind::Uncontrolled;
            let moved = return_after(
                &mut world,
                &mut missions,
                &ended(MissionKind::Move, is_alliance, team, Vec::new(), home),
            );

            assert!(stayed.is_empty() && moved.is_empty() && missions.is_empty(), "{is_alliance}");
        }
    }

    /// `FUN_0054c7d0`: a character holding a captor id (`FUN_004ed270`) is
    /// skipped and stays flagged; it neither leaves nor is freed.
    #[test]
    fn a_character_held_by_a_captor_is_skipped_and_stays_adrift() {
        for is_alliance in [true, false] {
            let side = crate::mission_detection::faction_of(is_alliance);
            let (mut world, sys, _) = cascade_world(400, side);
            world.systems[sys].is_destroyed = true;
            let held = world.characters.insert(Character {
                is_alliance,
                is_empire: !is_alliance,
                current_system: Some(sys),
                adrift: true,
                is_captive: true,
                captured_by: Some(crate::mission_detection::faction_of(!is_alliance)),
                ..Character::default()
            });
            let mut missions = MissionState::new();

            let started = sweep_adrift(
                &mut world,
                &mut missions,
                &TroopTransportState::default(),
                &MovementState::new(),
                sys,
                false,
                9,
            );

            assert!(started.is_empty() && missions.is_empty());
            let character = &world.characters[held];
            assert!(character.adrift && character.is_captive, "{is_alliance}");
            assert_eq!(character.current_system, Some(sys));
        }
    }

    /// `FUN_00549230`: a group with a free member starts the hidden trip
    /// for every member, prisoners among them (slot `+0x1d4` only chooses
    /// the trip over `FUN_00549850`). A prisoner groups on its captor's side
    /// bits (`FUN_004f1a80` → `FUN_004eec60`), so it travels with its
    /// captor's characters and stays a prisoner.
    #[test]
    fn a_captive_travels_in_its_captors_groups_trip() {
        for is_alliance in [true, false] {
            let captor = crate::mission_detection::faction_of(!is_alliance);
            let (mut world, sys, refuge) = cascade_world(400, captor);
            world.systems[sys].is_destroyed = true;
            let mut add = |is_alliance, is_captive| {
                world.characters.insert(Character {
                    is_alliance,
                    is_empire: !is_alliance,
                    current_system: Some(sys),
                    adrift: true,
                    is_captive,
                    ..Character::default()
                })
            };
            let captive = add(is_alliance, true);
            let free = add(!is_alliance, false);
            let mut missions = MissionState::new();

            let started = sweep_adrift(
                &mut world,
                &mut missions,
                &TroopTransportState::default(),
                &MovementState::new(),
                sys,
                false,
                9,
            );

            assert_eq!(started.len(), 1, "{is_alliance}");
            let trip = &missions.missions()[0];
            assert_eq!(trip.target_system, refuge);
            let mut members: Vec<MissionMember> = trip.members().collect();
            members.sort_by_key(|member| *member != MissionMember::Character(captive));
            assert_eq!(
                members,
                vec![
                    MissionMember::Character(captive),
                    MissionMember::Character(free)
                ]
            );
            assert!(world.characters[captive].is_captive);
            assert!(!world.characters[captive].adrift);
        }
    }

    /// `FUN_0050b8e0` sets the battle bit (`+0x88` `0x1000`, `FUN_0050a900`)
    /// when side 1 and side 2 are both present at a system whose side bits
    /// are not 0, and writes it clear at side 0; its change posts event
    /// `0x304` (`FUN_00511780`), whose sweep only gathers while the bit is
    /// set (`FUN_00548e90`; `blockade-bit.md`, the rule).
    #[test]
    fn both_sides_arriving_sets_the_battle_bit_and_its_sweep_only_gathers() {
        for is_alliance in [true, false] {
            let side = crate::mission_detection::faction_of(is_alliance);
            let enemy = crate::mission_detection::faction_of(!is_alliance);
            for control in [ControlKind::Controlled(enemy), ControlKind::Uncontrolled] {
                let (mut world, sys, _) = cascade_world(400, side);
                world.systems[sys].control = control;
                let own_ship = completed_ship(&mut world, is_alliance);
                let their_ship = completed_ship(&mut world, !is_alliance);
                let own = world.fleets.insert(Fleet {
                    location: sys,
                    capital_ships: vec![own_ship],
                    is_alliance,
                    ..Fleet::default()
                });
                let theirs = world.fleets.insert(Fleet {
                    location: sys,
                    capital_ships: vec![their_ship],
                    is_alliance: !is_alliance,
                    ..Fleet::default()
                });
                world.systems[sys].fleets.extend([own, theirs]);
                let aboard = world.characters.insert(Character {
                    is_alliance,
                    is_empire: !is_alliance,
                    current_system: Some(sys),
                    current_fleet: Some(own),
                    adrift: true,
                    ..Character::default()
                });
                world.fleets[own].characters.push(aboard);
                let mut missions = MissionState::new();
                let mut blockade = crate::blockade::BlockadeState::new();

                let swept = sweep_battle_changes(
                    &mut world,
                    &mut missions,
                    &mut blockade,
                    &TroopTransportState::default(),
                    &MovementState::new(),
                    9,
                );

                assert!(missions.is_empty(), "{is_alliance} {control:?}");
                let character = &world.characters[aboard];
                assert!(character.adrift);
                // A neutral system is side 3 (FUN_0055a080, FUN_0050b610),
                // whose battle bit FUN_0050b8e0 sets as a held one's.
                assert_eq!(swept, vec![(sys, Vec::new())], "{is_alliance} {control:?}");
                assert_eq!(character.current_fleet, None, "{control:?}");
            }
        }
    }

    /// The battle bit clearing (one side leaves) posts event `0x304` again,
    /// and that sweep dispatches (`FUN_00548e90` → `FUN_00549230`).
    #[test]
    fn the_battle_bit_clearing_sends_the_adrift_characters_away() {
        for is_alliance in [true, false] {
            let side = crate::mission_detection::faction_of(is_alliance);
            let enemy = crate::mission_detection::faction_of(!is_alliance);
            let (mut world, sys, refuge) = cascade_world(400, side);
            world.systems[sys].control = ControlKind::Controlled(enemy);
            let own_ship = completed_ship(&mut world, is_alliance);
            let their_ship = completed_ship(&mut world, !is_alliance);
            let own = world.fleets.insert(Fleet {
                location: sys,
                capital_ships: vec![own_ship],
                is_alliance,
                ..Fleet::default()
            });
            let theirs = world.fleets.insert(Fleet {
                location: sys,
                capital_ships: vec![their_ship],
                is_alliance: !is_alliance,
                ..Fleet::default()
            });
            world.systems[sys].fleets.extend([own, theirs]);
            let stranded = world.characters.insert(Character {
                is_alliance,
                is_empire: !is_alliance,
                current_system: Some(sys),
                adrift: true,
                ..Character::default()
            });
            let mut missions = MissionState::new();
            let mut blockade = crate::blockade::BlockadeState::new();
            let mut sweep = |world: &mut GameWorld, missions: &mut MissionState| {
                sweep_battle_changes(
                    world,
                    missions,
                    &mut blockade,
                    &TroopTransportState::default(),
                    &MovementState::new(),
                    9,
                )
            };
            let _ = sweep(&mut world, &mut missions);
            assert!(missions.is_empty());
            world.systems[sys].fleets.retain(|&fleet| fleet != theirs);

            let swept = sweep(&mut world, &mut missions);

            assert_eq!(swept.len(), 1, "{is_alliance}");
            let trip = &missions.missions()[0];
            assert_eq!(
                (trip.target_system, trip.team.clone()),
                (refuge, vec![MissionMember::Character(stranded)])
            );
        }
    }

    /// `FUN_0054c7d0` compares the character's side with the container's
    /// side bits, which both sides' regiments keep as the earlier troop
    /// holder (`FUN_0050b610.c:45-70`): at its own side's stalemated system
    /// an adrift character only loses its flag.
    #[test]
    fn a_stalemate_keeps_the_earlier_holders_side_for_the_sweep() {
        for is_alliance in [true, false] {
            let side = crate::mission_detection::faction_of(is_alliance);
            let (mut world, sys, _) = cascade_world(400, side);
            world.systems[sys].control = ControlKind::Controlled(side);
            troop(&mut world, sys, is_alliance);
            crate::support_shift::refresh_troop_bits(&mut world, sys);
            troop(&mut world, sys, !is_alliance);
            crate::support_shift::rebuild_control(&mut world, sys);
            let home = world.characters.insert(Character {
                is_alliance,
                is_empire: !is_alliance,
                current_system: Some(sys),
                adrift: true,
                ..Character::default()
            });
            let mut missions = MissionState::new();

            let started = sweep_adrift(
                &mut world,
                &mut missions,
                &TroopTransportState::default(),
                &MovementState::new(),
                sys,
                false,
                9,
            );

            assert!(started.is_empty() && missions.is_empty(), "{is_alliance}");
            let character = &world.characters[home];
            assert!(!character.adrift);
            assert_eq!(character.current_system, Some(sys));
        }
    }

    /// `FUN_0054c7d0` clears the flags of a free character it takes (the
    /// autoscrap request, `FUN_004f78e0(0)`, and the adrift bit,
    /// `FUN_00534790(0)`) whether or not its container passes the group
    /// test, so an adrift character at its own side's intact system stays
    /// there with its flag cleared.
    #[test]
    fn an_adrift_character_at_its_own_sides_intact_system_only_loses_its_flag() {
        for is_alliance in [true, false] {
            let side = crate::mission_detection::faction_of(is_alliance);
            let (mut world, sys, _) = cascade_world(400, side);
            world.systems[sys].control = ControlKind::Controlled(side);
            let home = world.characters.insert(Character {
                is_alliance,
                is_empire: !is_alliance,
                current_system: Some(sys),
                adrift: true,
                ..Character::default()
            });
            let mut missions = MissionState::new();

            let started = sweep_adrift(
                &mut world,
                &mut missions,
                &TroopTransportState::default(),
                &MovementState::new(),
                sys,
                false,
                9,
            );

            assert!(started.is_empty() && missions.is_empty());
            let character = &world.characters[home];
            assert!(!character.adrift, "{is_alliance}");
            assert_eq!(character.current_system, Some(sys));
            assert!(!character.on_mission);
        }
    }

    /// The chain by kind (`FUN_005073d0`, `FUN_00554f80`) for another
    /// yard's products bound for the destroyed system: a queued regiment's
    /// `0x204` lifts the blockade refusal, so it takes its side's blockaded
    /// system (stage 5); a queued squadron boards its side's carrier with
    /// room (stage 4); a ship sits in the spare fleet and is not walked.
    #[test]
    fn a_queued_regiment_takes_a_blockaded_system_and_a_queued_squadron_a_carrier() {
        let (mut world, sys, refuge) = cascade_world(400, Faction::Alliance);
        add_ds_fleet(&mut world, refuge);
        let kuat = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0003),
            name: "Kuat".into(),
            y: 300,
            control: ControlKind::Controlled(Faction::Empire),
            ..world.systems[sys].clone()
        });
        let carrier_class = world.capital_ship_classes.insert(CapitalShipClass {
            is_alliance: true,
            hull: 100,
            fighter_capacity: 2,
            ..CapitalShipClass::default()
        });
        let carrier = world.fleets.insert(Fleet {
            location: kuat,
            capital_ships: vec![ShipInstance::new(carrier_class, 100, true)],
            is_alliance: true,
            ..Fleet::default()
        });
        world.systems[kuat].fleets.push(carrier);
        let regiment = DatId::new(0x1000_0001);
        world.buildable_classes.insert(
            regiment,
            crate::world::BuildableClass {
                is_alliance: true,
                ..Default::default()
            },
        );
        let fighter = world.fighter_classes.insert(FighterClass {
            is_alliance: true,
            ..FighterClass::default()
        });
        let mut manufacturing = ManufacturingState::new();
        for kind in [
            BuildableKind::CapitalShip(carrier_class),
            BuildableKind::Fighter(fighter),
            BuildableKind::Troop(regiment),
        ] {
            let item = QueueItem::new(kind, 10).delivered_to(Destination::System(sys));
            manufacturing.enqueue(kuat, item);
        }

        let report = destroy(
            &mut world,
            sys,
            &mut TroopTransportState::default(),
            &mut manufacturing,
            &mut DeliveryState::new(),
        );

        assert_eq!(
            report.autoscrapped,
            vec![
                (
                    Autoscrapped::Product(BuildableKind::Fighter(fighter)),
                    Relocation::Boarded(carrier)
                ),
                (
                    Autoscrapped::Product(BuildableKind::Troop(regiment)),
                    Relocation::Moved(refuge)
                ),
            ]
        );
        assert_eq!(manufacturing.queued_at(kuat), 3);
    }

    /// A queued squadron the chain sends to a fleet makes the Move's leg
    /// there (`FUN_00556390` -> `FUN_00552300`): it rides the ship with the
    /// most room and holds that room (`FUN_00500b40`, mode 3), so the next
    /// finds none and is destroyed with reason 2.
    #[test]
    fn a_queued_squadron_the_cascade_sends_to_a_carrier_rides_its_ship_and_holds_its_room() {
        for side in [Faction::Alliance, Faction::Empire] {
            let is_alliance = side == Faction::Alliance;
            let enemy = if is_alliance { Faction::Empire } else { Faction::Alliance };
            let (mut world, sys, refuge) = cascade_world(400, enemy);
            let kuat = world.systems.insert(System {
                dat_id: DatId::new(0x9000_0003),
                name: "Kuat".into(),
                y: 300,
                control: ControlKind::Controlled(enemy),
                ..world.systems[sys].clone()
            });
            let carrier_class = world.capital_ship_classes.insert(CapitalShipClass {
                is_alliance,
                hull: 100,
                fighter_capacity: 1,
                ..CapitalShipClass::default()
            });
            let carrier = world.fleets.insert(Fleet {
                location: refuge,
                capital_ships: vec![ShipInstance::new(carrier_class, 100, is_alliance)],
                is_alliance,
                ..Fleet::default()
            });
            world.systems[refuge].fleets.push(carrier);
            let fighter = world.fighter_classes.insert(FighterClass {
                is_alliance,
                ..FighterClass::default()
            });
            let squadron = BuildableKind::Fighter(fighter);
            let mut manufacturing = ManufacturingState::new();
            for _ in 0..2 {
                manufacturing.enqueue(kuat, QueueItem::new(squadron, 10).delivered_to(Destination::System(sys)));
            }

            let report = destroy(
                &mut world,
                sys,
                &mut TroopTransportState::default(),
                &mut manufacturing,
                &mut DeliveryState::new(),
            );

            assert_eq!(
                report.autoscrapped,
                vec![
                    (
                        Autoscrapped::Product(squadron),
                        Relocation::Boarded(carrier)
                    ),
                    (Autoscrapped::Product(squadron), Relocation::Destroyed(2)),
                ],
                "{side:?}"
            );
            let tag = world.fleets[carrier].capital_ships[0].tag;
            let kept: Vec<_> = manufacturing
                .items()
                .map(|(_, _, item)| (item.destination, item.carrier))
                .collect();
            assert_eq!(kept, [(Some(Destination::Fleet(carrier)), tag)], "{side:?}");
            assert_ne!(tag, 0, "{side:?}");
        }
    }

    /// A completed capital ship travels as a child of the spare fleet, which
    /// the walk skips, and a special force en route is left alone
    /// (`FUN_00503e20`): both stay on their way to the destroyed system.
    #[test]
    fn a_ship_or_special_force_travelling_to_the_destroyed_system_is_left_alone() {
        let (mut world, sys, refuge) = cascade_world(400, Faction::Alliance);
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            is_alliance: true,
            ..CapitalShipClass::default()
        });
        let commando = DatId::new(0x3c00_0001);
        let mut deliveries = DeliveryState::new();
        let departures: Vec<Departure> = [
            BuildableKind::CapitalShip(class),
            BuildableKind::SpecialForce(commando),
        ]
        .into_iter()
        .map(|kind| Departure {
            origin: refuge,
            destination: Destination::System(sys),
            tick: 1,
            kind,
            carrier: 0,
        })
        .collect();
        deliveries.depart(&world, &departures);
        let before = deliveries.en_route().to_vec();

        let report = destroy(
            &mut world,
            sys,
            &mut TroopTransportState::default(),
            &mut ManufacturingState::new(),
            &mut deliveries,
        );

        assert!(report.autoscrapped.is_empty());
        assert_eq!(deliveries.en_route(), before.as_slice());
    }

    /// Only objects in the destroyed system's own container take the
    /// request: a delivery bound for a fleet there is the fleet's child, and
    /// one bound elsewhere is untouched.
    #[test]
    fn deliveries_bound_for_a_fleet_there_or_elsewhere_are_left_alone() {
        let (mut world, sys, refuge) = cascade_world(400, Faction::Alliance);
        let fleet = add_ds_fleet(&mut world, sys);
        let class = world
            .capital_ship_classes
            .insert(CapitalShipClass::default());
        let mut deliveries = DeliveryState::new();
        let kind = BuildableKind::CapitalShip(class);
        deliveries.depart(
            &world,
            &[
                Departure {
                    origin: refuge,
                    destination: Destination::Fleet(fleet),
                    tick: 1,
                    kind,
                    carrier: 0,
                },
                Departure {
                    origin: sys,
                    destination: Destination::System(refuge),
                    tick: 1,
                    kind,
                    carrier: 0,
                },
            ],
        );
        let before = deliveries.en_route().to_vec();

        let report = destroy(
            &mut world,
            sys,
            &mut TroopTransportState::default(),
            &mut ManufacturingState::new(),
            &mut deliveries,
        );

        assert!(report.autoscrapped.is_empty());
        assert_eq!(deliveries.en_route(), before.as_slice());
    }

    /// A squadron whose transit origin no longer accepts it goes to its
    /// side's nearest system (stage 3). It keeps its arrival day, and the
    /// leg from the destroyed system starts then (`FUN_00556620`).
    #[test]
    fn a_squadron_refused_by_its_origin_travels_to_the_nearest_system_of_its_side() {
        let (mut world, sys, refuge) = cascade_world(400, Faction::Alliance);
        let origin = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0004),
            name: "Ord Mantell".into(),
            x: 900,
            control: ControlKind::Controlled(Faction::Empire),
            ..world.systems[sys].clone()
        });
        let class = FighterClass {
            is_alliance: true,
            ..FighterClass::default()
        };
        let speed = crate::movement::fighter_speed(&world, &class);
        let fighter = world.fighter_classes.insert(class);
        world
            .squadron_transits
            .push(crate::troop_transport::SquadronTransit {
                class: fighter,
                is_alliance: true,
                origin,
                leg: RegimentLeg::Surface(sys),
                carrier: 0,
                arrival_tick: 30,
                via: None,
            });
        let days = crate::movement::transit_ticks_between(&world, (100, 100), (400, 100), speed);

        let report = destroy_plain(&mut world, sys);

        assert!(days > 1);
        assert_eq!(world.squadron_transits.len(), 1);
        let transit = &world.squadron_transits[0];
        assert_eq!(transit.leg, RegimentLeg::Surface(refuge));
        assert_eq!(transit.arrival_tick, 30);
        TroopTransportState::default().advance_transit(&mut world, 30);
        assert_eq!(world.squadron_transits[0].arrival_tick, 30 + u64::from(days));
        assert_eq!(report.autoscrapped[0].1, Relocation::Moved(refuge));
    }

    /// `FUN_00548e90` → `FUN_0054c7d0`: the sweep takes adrift characters
    /// off a mission, alive and directly at the system, and groups those
    /// of one side bound for one place in a single Adrift mission.
    #[test]
    fn the_sweep_takes_only_adrift_characters_directly_at_the_system_in_one_group() {
        let (mut world, sys, refuge) = cascade_world(400, Faction::Alliance);
        world.systems[sys].is_destroyed = true;
        let fleet = world.fleets.insert(Fleet {
            location: refuge,
            is_alliance: true,
            ..Fleet::default()
        });
        world.systems[refuge].fleets.push(fleet);
        let mut add = |set: &dyn Fn(&mut Character)| {
            let mut character = Character {
                is_alliance: true,
                current_system: Some(sys),
                adrift: true,
                ..Character::default()
            };
            set(&mut character);
            world.characters.insert(character)
        };
        let first = add(&|_| {});
        let second = add(&|_| {});
        let calm = add(&|c| c.adrift = false);
        let busy = add(&|c| c.on_mission = true);
        let dead = add(&|c| c.is_killed = true);
        let away = add(&|c| c.current_system = Some(refuge));
        let aboard = add(&|c| c.current_fleet = Some(fleet));
        let mut missions = MissionState::new();

        let started = sweep_adrift(
            &mut world,
            &mut missions,
            &TroopTransportState::default(),
            &MovementState::new(),
            sys,
            false,
            9,
        );

        let ids: Vec<u64> = missions.missions().iter().map(|m| m.id).collect();
        assert_eq!(started, ids);
        let teams: Vec<_> = missions.missions().iter().map(|m| m.team.clone()).collect();
        assert_eq!(
            teams,
            vec![vec![
                MissionMember::Character(first),
                MissionMember::Character(second)
            ]]
        );
        for (key, adrift) in [
            (calm, false),
            (busy, true),
            (dead, true),
            (away, true),
            (aboard, true),
        ] {
            assert_eq!(world.characters[key].adrift, adrift, "{key:?}");
        }
    }

    // ── A holder change's autoscrap requests (event 0x307) ────────────────

    /// `sys` held by `own` with a yard system of `own` at (400, 100) and one
    /// of the other side at (100, 300); `sys` then passes to the other side.
    fn holder_world(own: Faction) -> (GameWorld, SystemKey, SystemKey, SystemKey, Faction) {
        let enemy = if own == Faction::Alliance {
            Faction::Empire
        } else {
            Faction::Alliance
        };
        let (mut world, sys, yard) = cascade_world(400, own);
        world.systems[sys].control = ControlKind::Controlled(own);
        let enemy_yard = world.systems.insert(System {
            dat_id: DatId::new(0x9000_0003),
            name: "Kuat".into(),
            y: 300,
            control: ControlKind::Controlled(enemy),
            ..world.systems[sys].clone()
        });
        let sector = world.systems[sys].sector;
        world.sectors[sector].systems.push(enemy_yard);
        (world, sys, yard, enemy_yard, enemy)
    }

    fn serve(
        world: &mut GameWorld,
        transport: &mut TroopTransportState,
        manufacturing: &mut ManufacturingState,
        deliveries: &mut DeliveryState,
    ) -> CascadeReport {
        serve_holder_requests(
            world,
            &MovementState::new(),
            transport,
            manufacturing,
            deliveries,
        )
    }

    /// The holder change's walk flags another manager's unfinished product
    /// in the system (FUN_004f8680.c:14-17), and event 0x307 runs the chain
    /// (FUN_00515480): the new holder's own product stays (stage 1), the old
    /// holder's goes to its producer's system, the hint (FUN_00515480.c:
    /// 82-124), keeping its queue slot; a capital ship sits in a fleet, which
    /// the walk leaves (FUN_0051ebb0).
    #[test]
    fn a_holder_change_sends_the_old_holders_products_to_their_yards_and_keeps_the_new_holders_for_either_side(
    ) {
        for own in [Faction::Alliance, Faction::Empire] {
            let (mut world, sys, yard, enemy_yard, enemy) = holder_world(own);
            let facility = |side: Faction| {
                BuildableKind::ManufacturingFacility(FacilityBuild {
                    class: DatId::new(0x2800_0001),
                    is_alliance: side == Faction::Alliance,
                })
            };
            let ship = world.capital_ship_classes.insert(CapitalShipClass {
                is_alliance: own == Faction::Alliance,
                is_empire: own == Faction::Empire,
                ..CapitalShipClass::default()
            });
            let mut manufacturing = ManufacturingState::new();
            for (at, kind) in [
                (yard, facility(own)),
                (yard, BuildableKind::CapitalShip(ship)),
                (enemy_yard, facility(enemy)),
            ] {
                manufacturing.enqueue(
                    at,
                    QueueItem::new(kind, 10).delivered_to(Destination::System(sys)),
                );
            }

            capture(&mut world, sys, ControlKind::Controlled(enemy));
            let report = serve(
                &mut world,
                &mut TroopTransportState::default(),
                &mut manufacturing,
                &mut DeliveryState::new(),
            );

            assert_eq!(
                report.autoscrapped,
                vec![
                    (Autoscrapped::Product(facility(own)), Relocation::Moved(yard)),
                    (Autoscrapped::Product(facility(enemy)), Relocation::Stayed),
                ],
                "{own:?}"
            );
            let destinations = |at: SystemKey, area: ProductionArea| -> Vec<Option<Destination>> {
                manufacturing
                    .queue(at, area)
                    .map(|queue| queue.items().iter().map(|item| item.destination).collect())
                    .unwrap_or_default()
            };
            let none: Vec<Option<Destination>> = vec![None];
            let here = vec![Some(Destination::System(sys))];
            assert_eq!(destinations(yard, ProductionArea::ConstructionYard), none, "{own:?}");
            assert_eq!(destinations(yard, ProductionArea::Shipyard), here, "{own:?}");
            assert_eq!(
                destinations(enemy_yard, ProductionArea::ConstructionYard),
                here,
                "{own:?}"
            );
            assert!(world.autoscrap_systems.is_empty(), "{own:?}");
        }
    }

    /// A special force's slot +0xb0 is FUN_00503d90, not the request: it
    /// takes the system's new side when its class serves it (FUN_004f27d0),
    /// else dies with reason 3.
    #[test]
    fn a_holder_change_ends_a_special_force_product_its_class_cannot_serve_for_either_side() {
        for own in [Faction::Alliance, Faction::Empire] {
            let (mut world, sys, yard, _, enemy) = holder_world(own);
            let one_side = DatId::new(0x3c00_0001);
            let both = DatId::new(0x3c00_0002);
            for (class, is_alliance, is_empire) in [
                (one_side, own == Faction::Alliance, own == Faction::Empire),
                (both, true, true),
            ] {
                world.buildable_classes.insert(
                    class,
                    crate::world::BuildableClass {
                        is_alliance,
                        is_empire,
                        ..Default::default()
                    },
                );
            }
            let mut manufacturing = ManufacturingState::new();
            for class in [one_side, both] {
                manufacturing.enqueue(
                    yard,
                    QueueItem::new(BuildableKind::SpecialForce(class), 10)
                        .delivered_to(Destination::System(sys)),
                );
            }

            capture(&mut world, sys, ControlKind::Controlled(enemy));
            let report = serve(
                &mut world,
                &mut TroopTransportState::default(),
                &mut manufacturing,
                &mut DeliveryState::new(),
            );

            assert_eq!(report.products, [BuildableKind::SpecialForce(one_side)], "{own:?}");
            let kept: Vec<BuildableKind> = manufacturing
                .queue(yard, ProductionArea::TrainingFacility)
                .map(|queue| queue.items().iter().map(|item| item.kind).collect())
                .unwrap_or_default();
            assert_eq!(kept, [BuildableKind::SpecialForce(both)], "{own:?}");
        }
    }

    /// A completed product or regiment on its way to the system is its
    /// child, en route (+0x50 0x20), and takes the request as well
    /// (FUN_004f8680.c:14-17); the hint is its transit origin
    /// (FUN_00515480.c:125-153), and the relocation keeps its arrival day
    /// (FUN_00556390.c:17-21).
    #[test]
    fn a_holder_change_sends_the_old_holders_travellers_back_to_their_origins_for_either_side() {
        for own in [Faction::Alliance, Faction::Empire] {
            let (mut world, sys, yard, _, enemy) = holder_world(own);
            let is_alliance = own == Faction::Alliance;
            let shield = BuildableKind::DefenseFacility(FacilityBuild {
                class: DatId::new(0x2400_0001),
                is_alliance,
            });
            let mut deliveries = DeliveryState::new();
            deliveries.depart(
                &world,
                &[Departure {
                    origin: yard,
                    destination: Destination::System(sys),
                    tick: 1,
                    kind: shield,
                    carrier: 0,
                }],
            );
            let due = deliveries.en_route()[0].arrival_tick;
            let mut transport = TroopTransportState::default();
            let regiment = troop(&mut world, yard, is_alliance);
            transport
                .move_regiment(
                    &mut world,
                    &MovementState::new(),
                    &std::collections::HashSet::new(),
                    &ProductRoom::default(),
                    regiment,
                    crate::troop_transport::RegimentTarget::System(sys),
                    1,
                )
                .unwrap();

            capture(&mut world, sys, ControlKind::Controlled(enemy));
            let report = serve(
                &mut world,
                &mut transport,
                &mut ManufacturingState::new(),
                &mut deliveries,
            );

            assert_eq!(
                report.autoscrapped,
                vec![
                    (Autoscrapped::Delivery(shield), Relocation::Moved(yard)),
                    (Autoscrapped::Regiment(regiment), Relocation::Moved(yard)),
                ],
                "{own:?}"
            );
            assert_eq!(deliveries.en_route()[0].destination, yard, "{own:?}");
            assert_eq!(deliveries.en_route()[0].arrival_tick, due, "{own:?}");
            assert_eq!(
                transport.transits()[0].leg,
                RegimentLeg::Surface(yard),
                "{own:?}"
            );
        }
    }

    /// The new holder's own regiment and squadron on their way stay bound
    /// for the system: its stage 1 accepts them (FUN_005073d0,
    /// FUN_00553410), so nothing moves and their legs stand.
    #[test]
    fn a_holder_change_keeps_the_new_holders_travellers_bound_for_the_system_for_either_side() {
        for own in [Faction::Alliance, Faction::Empire] {
            let (mut world, sys, _, enemy_yard, enemy) = holder_world(own);
            let is_alliance = enemy == Faction::Alliance;
            capture(&mut world, sys, ControlKind::Controlled(enemy));
            let mut transport = TroopTransportState::default();
            let regiment = troop(&mut world, enemy_yard, is_alliance);
            transport
                .move_regiment(
                    &mut world,
                    &MovementState::new(),
                    &std::collections::HashSet::new(),
                    &ProductRoom::default(),
                    regiment,
                    crate::troop_transport::RegimentTarget::System(sys),
                    1,
                )
                .unwrap();
            let fighter = world.fighter_classes.insert(FighterClass {
                is_alliance,
                ..FighterClass::default()
            });
            for arrival_tick in [30, 40] {
                world
                    .squadron_transits
                    .push(crate::troop_transport::SquadronTransit {
                        class: fighter,
                        is_alliance,
                        origin: enemy_yard,
                        leg: RegimentLeg::Surface(sys),
                        carrier: 0,
                        arrival_tick,
                        via: None,
                    });
            }
            let legs = |world: &GameWorld, transport: &TroopTransportState| {
                (
                    transport
                        .transits()
                        .iter()
                        .map(|transit| transit.leg)
                        .collect::<Vec<_>>(),
                    world
                        .squadron_transits
                        .iter()
                        .map(|transit| (transit.leg, transit.via))
                        .collect::<Vec<_>>(),
                )
            };
            let before = legs(&world, &transport);

            let report = serve(
                &mut world,
                &mut transport,
                &mut ManufacturingState::new(),
                &mut DeliveryState::new(),
            );

            let squadron = Autoscrapped::Squadron {
                class: fighter,
                is_alliance,
            };
            assert_eq!(
                report.autoscrapped,
                vec![
                    (Autoscrapped::Regiment(regiment), Relocation::Stayed),
                    (squadron.clone(), Relocation::Stayed),
                    (squadron, Relocation::Stayed),
                ],
                "{own:?}"
            );
            assert_eq!(legs(&world, &transport), before, "{own:?}");
        }
    }
}
