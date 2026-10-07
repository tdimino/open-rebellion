//! What window type 9, the Manufacturing and Production window, shows: its
//! overview's three producer bands and its five facility pages. The window
//! itself (chrome, tabs, rail) is `system_window.rs`.
//!
//! Recovery: `ghidra/notes/manufacturing-build-selection.md` ("Window
//! composition"). Manual pp. 82–86, Figs. 2.10, 3.24, 3.27.

use rebellion_core::delivery::DeliveryState;
use rebellion_core::fog::FogState;
use rebellion_core::ids::{DatId, ManufacturingFacilityKey, ProductionFacilityKey, SystemKey};
use rebellion_core::manufacturing::{BuildableKind, ManufacturingState, ProductionArea};
use rebellion_core::world::GameWorld;

use crate::fleet_window::{control_side, fleet_side};
use crate::panels::fleets::{capital_ship_mini_id, fighter_mini_id};
use crate::system_window::{
    defense_facility_mini, manufacturing_facility_mini, opposing_contents_visible,
    production_facility_mini, special_force_mini, troop_mini,
};

/// The overview's bands, top to bottom: ships, troops, facilities
/// (`FUN_00455060`: managers `FUN_00509670(system, 0)`, `(.., 2)`, `(.., 1)`).
pub const BAND_AREAS: [ProductionArea; 3] = [
    ProductionArea::Shipyard,
    ProductionArea::TrainingFacility,
    ProductionArea::ConstructionYard,
];

/// Each band's rect (`FUN_00458480`), 166 by 79.
pub const BAND_RECTS: [(f32, f32, f32, f32); 3] = [
    (55.0, 57.0, 166.0, 79.0),
    (55.0, 138.0, 166.0, 79.0),
    (55.0, 219.0, 166.0, 79.0),
];

/// A band's frame (10290), keyed over the background under it.
pub const BAND_FRAME: u32 = 10_290;

/// The yard column at (6, 71) (`+0x198`, 46 by 226).
pub const YARD_COLUMN: (u32, f32, f32) = (10_298, 6.0, 71.0);

/// Each band's "N:M" yard count origin, font 10 (`FUN_00457690`).
pub const COUNT_ORIGINS: [(f32, f32); 3] = [(6.0, 119.0), (6.0, 200.0), (6.0, 280.0)];

/// Each band's progress bar (`FUN_004acec0`, ids `0x6d..0x6f`): 160 by 4,
/// light gray `0x200f0f0` over black `0x2000000`.
pub const PROGRESS_RECTS: [(f32, f32, f32, f32); 3] = [
    (56.0, 127.0, 160.0, 4.0),
    (56.0, 208.0, 160.0, 4.0),
    (56.0, 289.0, 160.0, 4.0),
];

/// Inside a band (`FUN_00458080`): the title, the status line, the units
/// line and the destination line, font 10; the product's mini.
pub const BAND_TITLE: (f32, f32) = (5.0, 1.0);
pub const BAND_STATUS: (f32, f32) = (5.0, 16.0);
pub const BAND_UNITS: (f32, f32) = (5.0, 47.0);
pub const BAND_DESTINATION: (f32, f32) = (5.0, 57.0);
pub const BAND_MINI: (f32, f32) = (40.0, 15.0);

/// The facility pages' list (`FUN_00607ea0`, id 2) and its 69 by 40 cells.
pub const LIST: (f32, f32, f32, f32) = (8.0, 77.0, 222.0, 225.0);
pub const CELL: (f32, f32) = (69.0, 40.0);
pub const COLUMNS: usize = 3;

/// The mines page's empty deposit picture (`FUN_004568a0`, `0x232d`).
pub const DEPOSIT_PICTURE: u32 = 9_005;

/// A band's title strip, selected or not, by the shown side
/// (`FUN_00456230`: `+0x18c`, `+0x190`).
#[must_use]
pub const fn band_strip(side: u8, selected: bool) -> u32 {
    let base = match side {
        1 => 10_291,
        2 => 10_293,
        _ => 10_295,
    };
    if selected {
        base
    } else {
        base + 1
    }
}

/// The selected item's frame, keyed over its picture (`+0x15c`).
#[must_use]
pub const fn item_frame(side: u8) -> u32 {
    match side {
        1 => 10_262,
        2 => 10_263,
        _ => 10_264,
    }
}

/// A band's title (TEXTSTRA 6185, 6193, 6195).
#[must_use]
pub const fn band_title(area: ProductionArea) -> &'static str {
    match area {
        ProductionArea::Shipyard => "Ship Construction",
        ProductionArea::TrainingFacility => "Troops in Training",
        ProductionArea::ConstructionYard => "Facilities Under Construction",
    }
}

/// A band's status line with nothing building (TEXTSTRA 6198..6200).
#[must_use]
pub const fn band_idle_text(area: ProductionArea) -> &'static str {
    match area {
        ProductionArea::Shipyard => "No Ships are being built",
        ProductionArea::TrainingFacility => "No Troops in training",
        ProductionArea::ConstructionYard => "No Facilities are being built",
    }
}

/// What one band shows.
#[derive(Debug, Clone, PartialEq)]
pub struct Band {
    pub area: ProductionArea,
    /// The product's name, or the band's idle text.
    pub status: String,
    /// The product's GOKRES mini (class `+0x30 & 0xfff`).
    pub mini: Option<u32>,
    /// "Building: N" or "Training: N", the units left, with a product.
    pub units: Option<String>,
    /// "Destination: " and the area's destination system.
    pub destination: String,
    /// The product's progress (`+0x5c` of `+0x68`), with a product.
    pub progress: Option<f32>,
}

/// The band for `system`'s `area` (`FUN_00458080`, `FUN_00457c90`,
/// `FUN_00457b40`, `FUN_00457f30`, `FUN_00458040`).
///
/// port: the area's queue at the system stands in for its manager; the units
/// left are the queued units. hyp: the other side's production shows only
/// where the player sees that side's objects there
/// (`opposing_contents_visible`), as the pages do.
#[must_use]
pub fn band(
    world: &GameWorld,
    manufacturing: &ManufacturingState,
    visible: bool,
    system: SystemKey,
    area: ProductionArea,
) -> Band {
    let destination_system = manufacturing.destination(system, area).unwrap_or(system);
    let destination = format!(
        "Destination: {}",
        world
            .systems
            .get(destination_system)
            .map_or("", |value| value.name.as_str())
    );
    let queue = manufacturing
        .queue(system, area)
        .filter(|_| visible)
        .filter(|queue| !queue.is_empty());
    let Some(queue) = queue else {
        return Band {
            area,
            status: band_idle_text(area).into(),
            mini: None,
            units: None,
            destination,
            progress: None,
        };
    };
    let active = queue
        .active()
        .expect("a non-empty queue has an active unit");
    let (name, mini) = product(world, active.kind);
    let verb = if area == ProductionArea::TrainingFacility {
        "Training"
    } else {
        "Building"
    };
    Band {
        area,
        status: name,
        mini,
        units: Some(format!("{verb}: {}", queue.len())),
        destination,
        progress: Some(active.progress_fraction()),
    }
}

/// A product's name and GOKRES mini: its class's (`FUN_00437880`, class
/// `+0x30 & 0xfff`).
#[must_use]
pub fn product(world: &GameWorld, kind: BuildableKind) -> (String, Option<u32>) {
    match kind {
        BuildableKind::CapitalShip(key) => world
            .capital_ship_classes
            .get(key)
            .map_or((String::new(), None), |class| {
                (class.name.clone(), capital_ship_mini_id(class.dat_id))
            }),
        BuildableKind::Fighter(key) => world
            .fighter_classes
            .get(key)
            .map_or((String::new(), None), |class| {
                (class.name.clone(), fighter_mini_id(class.dat_id))
            }),
        _ => kind.class_dat_id().map_or((String::new(), None), |class| {
            let mini = class_mini(class);
            let name = world
                .buildable_classes
                .get(&class)
                .map(|value| value.name.clone())
                .filter(|name| !name.is_empty())
                .or_else(|| mini.map(|(_, label)| label.to_owned()))
                .unwrap_or_default();
            (name, mini.map(|(id, _)| id))
        }),
    }
}

/// A regiment, special-force or facility class's GOKRES mini and label.
pub(crate) fn class_mini(class: DatId) -> Option<(u32, &'static str)> {
    troop_mini(class)
        .or_else(|| special_force_mini(class))
        .or_else(|| defense_facility_mini(class))
        .or_else(|| manufacturing_facility_mini(class))
        .or_else(|| production_facility_mini(class))
}

/// The facility families each page lists (`FUN_004568a0`): shipyards
/// `0x28`, training facilities `0x29`, construction yards `0x2a`,
/// refineries `0x2d`, mines `0x2c`.
#[must_use]
pub const fn page_family(page: FacilityPage) -> u8 {
    match page {
        FacilityPage::Shipyards => 0x28,
        FacilityPage::TrainingFacilities => 0x29,
        FacilityPage::ConstructionYards => 0x2a,
        FacilityPage::Refineries => 0x2d,
        FacilityPage::Mines => 0x2c,
    }
}

/// The five facility pages (`0x68..0x6c`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FacilityPage {
    Shipyards,
    TrainingFacilities,
    ConstructionYards,
    Refineries,
    Mines,
}

/// The yard page each band's count reads (`FUN_0052c8c0`, `FUN_0052c5a0`,
/// `FUN_0052c270`).
#[must_use]
pub const fn yard_page(area: ProductionArea) -> FacilityPage {
    match area {
        ProductionArea::Shipyard => FacilityPage::Shipyards,
        ProductionArea::TrainingFacility => FacilityPage::TrainingFacilities,
        ProductionArea::ConstructionYard => FacilityPage::ConstructionYards,
    }
}

/// Where a facility is in its making (`FUN_00458fe0`'s state offset).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FacilityState {
    /// `+0x50` bit 2 set: built.
    Built,
    /// Bit 2 clear: queued or building.
    UnderConstruction,
    /// Bits 2 and 4 set: completed and travelling to its system.
    EnRoute,
}

/// One cell of a facility page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FacilityItem {
    Manufacturing(ManufacturingFacilityKey),
    Production(ProductionFacilityKey),
    /// The `index`th unit queued or travelling for this system.
    Pending(usize),
    /// The mines page's `index`th empty deposit.
    Deposit(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FacilityCell {
    pub item: FacilityItem,
    /// The STRATEGY picture.
    pub picture: u32,
}

/// A facility's picture (`FUN_00458fe0`): its type's base plus its state,
/// 0 built, 1 under construction (2 when its side is 1), 3 en route.
#[must_use]
pub fn facility_picture(class: DatId, side: u8, state: FacilityState) -> Option<u32> {
    let base = match (class.family(), class.index()) {
        (0x28..=0x2a, 1) => 9_006,
        (0x28..=0x2a, 2) => 9_014,
        (0x28..=0x2a, 3) => 9_022,
        (0x28..=0x2a, 4) => 9_010,
        (0x28..=0x2a, 5) => 9_018,
        (0x28..=0x2a, 6) => 9_026,
        (0x2c..=0x2f, 1) => 9_001,
        (0x2c..=0x2f, 2) => 9_030,
        _ => return None,
    };
    Some(match state {
        FacilityState::Built => base,
        FacilityState::UnderConstruction if side == 1 => base + 2,
        FacilityState::UnderConstruction => base + 1,
        FacilityState::EnRoute => base + 3,
    })
}

/// A queued or delivered facility unit's class and side.
fn pending_class(kind: BuildableKind) -> Option<(DatId, u8)> {
    match kind {
        BuildableKind::ManufacturingFacility(build) | BuildableKind::ProductionFacility(build) => {
            Some((build.class, fleet_side(build.is_alliance)))
        }
        _ => None,
    }
}

/// The cells of `page` at `system`, in walk order: its built facilities of
/// the shown side and family, then those queued for it, then those en route
/// to it; on the mines page, one empty deposit per raw-material deposit
/// beyond its mines.
///
/// port: a facility the original creates at the order sits in the port's
/// construction queues until done, and in its deliveries while travelling;
/// they show here as the original's under-construction and en-route
/// objects. The other side's facilities show only where the player sees
/// that side's objects (`opposing_contents_visible`).
#[must_use]
pub fn page_cells(
    world: &GameWorld,
    manufacturing: &ManufacturingState,
    deliveries: &DeliveryState,
    visible: bool,
    system: SystemKey,
    page: FacilityPage,
) -> Vec<FacilityCell> {
    let Some(value) = world.systems.get(system) else {
        return Vec::new();
    };
    if !visible {
        return Vec::new();
    }
    let side = control_side(value.control);
    let family = page_family(page);
    let wanted = |class: DatId, owner: u8| class.family() == family && owner == side;
    let mut cells: Vec<FacilityCell> = Vec::new();
    for &key in &value.manufacturing_facilities {
        let Some(facility) = world.manufacturing_facilities.get(key) else {
            continue;
        };
        let owner = fleet_side(facility.is_alliance);
        if wanted(facility.class_dat_id, owner) {
            if let Some(picture) =
                facility_picture(facility.class_dat_id, owner, FacilityState::Built)
            {
                cells.push(FacilityCell {
                    item: FacilityItem::Manufacturing(key),
                    picture,
                });
            }
        }
    }
    for &key in &value.production_facilities {
        let Some(facility) = world.production_facilities.get(key) else {
            continue;
        };
        let owner = fleet_side(facility.is_alliance);
        if wanted(facility.class_dat_id, owner) {
            if let Some(picture) =
                facility_picture(facility.class_dat_id, owner, FacilityState::Built)
            {
                cells.push(FacilityCell {
                    item: FacilityItem::Production(key),
                    picture,
                });
            }
        }
    }
    let queued = pending_units(manufacturing, system)
        .map(|kind| (kind, FacilityState::UnderConstruction))
        .chain(
            deliveries
                .en_route()
                .iter()
                .filter(|delivery| delivery.destination == system)
                .map(|delivery| (delivery.kind, FacilityState::EnRoute)),
        );
    for (index, (kind, state)) in queued.enumerate() {
        let Some((class, owner)) = pending_class(kind) else {
            continue;
        };
        if wanted(class, owner) {
            if let Some(picture) = facility_picture(class, owner, state) {
                cells.push(FacilityCell {
                    item: FacilityItem::Pending(index),
                    picture,
                });
            }
        }
    }
    if page == FacilityPage::Mines {
        let mines = cells.len();
        cells.extend(
            (mines..usize::from(value.raw_materials)).map(|index| FacilityCell {
                item: FacilityItem::Deposit(index),
                picture: DEPOSIT_PICTURE,
            }),
        );
    }
    cells
}

/// Every construction unit queued anywhere that will stand at `system`, in
/// system then queue order.
fn pending_units(
    manufacturing: &ManufacturingState,
    system: SystemKey,
) -> impl Iterator<Item = BuildableKind> + '_ {
    let mut queues: Vec<_> = manufacturing
        .queues()
        .iter()
        .filter(|((_, area), _)| *area == ProductionArea::ConstructionYard)
        .collect();
    queues.sort_unstable_by_key(|(key, _)| **key);
    queues.into_iter().flat_map(move |((origin, _), queue)| {
        queue
            .items()
            .iter()
            .filter(move |item| item.destination.unwrap_or(*origin) == system)
            .map(|item| item.kind)
    })
}

/// A page's tab is empty when it lists no facility (`FUN_00455060`: the
/// yard counts with mode 3, the refinery and mine walks).
#[must_use]
pub fn page_is_empty(cells: &[FacilityCell]) -> bool {
    !cells
        .iter()
        .any(|cell| !matches!(cell.item, FacilityItem::Deposit(_)))
}

/// A band's "N:M" (`FUN_00457690`): the area's yards built at the system,
/// then those and the ones being built or deployed there (manual Fig. 2.10),
/// joined by the executable's ":" (`DAT_006a872c`, as Fig. 3.24 prints).
#[must_use]
pub fn yard_count(cells: &[FacilityCell]) -> String {
    let built = cells
        .iter()
        .filter(|cell| {
            matches!(
                cell.item,
                FacilityItem::Manufacturing(_) | FacilityItem::Production(_)
            )
        })
        .count();
    format!("{built}:{}", cells.len())
}

/// Whether the player sees what `system`'s window lists: an explored system
/// the player holds, or one where the player sees the other side's objects
/// (`opposing_contents_visible` covers both).
#[must_use]
pub fn contents_visible(
    world: &GameWorld,
    fog: &FogState,
    player: rebellion_core::dat::Faction,
    system: SystemKey,
) -> bool {
    world.systems.get(system).is_some_and(|value| {
        value.exploration_status != rebellion_core::dat::ExplorationStatus::Unexplored
    }) && opposing_contents_visible(world, fog, player, system)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::dat::{ExplorationStatus, Faction};
    use rebellion_core::manufacturing::{FacilityBuild, QueueItem};
    use rebellion_core::world::{
        CapitalShipClass, ControlKind, ManufacturingFacilityInstance, ProductionFacilityInstance,
        System,
    };

    fn add_system(world: &mut GameWorld, name: &str, control: ControlKind) -> SystemKey {
        world.systems.insert(System {
            dat_id: DatId::new(0x9000_0001),
            name: name.into(),
            sector: rebellion_core::ids::SectorKey::default(),
            x: 0,
            y: 0,
            exploration_status: ExplorationStatus::Explored,
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

    fn yard(
        world: &mut GameWorld,
        system: SystemKey,
        class: u32,
        alliance: bool,
    ) -> ManufacturingFacilityKey {
        let key = world
            .manufacturing_facilities
            .insert(ManufacturingFacilityInstance {
                class_dat_id: DatId::new(class),
                is_alliance: alliance,
                is_shipyard: class >> 24 == 0x28,
            });
        world.systems[system].manufacturing_facilities.push(key);
        key
    }

    fn production(world: &mut GameWorld, system: SystemKey, class: u32) -> ProductionFacilityKey {
        let key = world
            .production_facilities
            .insert(ProductionFacilityInstance {
                class_dat_id: DatId::new(class),
                is_alliance: true,
                is_mine: class == 0x2c00_0001,
            });
        world.systems[system].production_facilities.push(key);
        key
    }

    const ALLIANCE: ControlKind = ControlKind::Controlled(Faction::Alliance);

    fn alliance(class: u32) -> FacilityBuild {
        FacilityBuild {
            class: DatId::new(class),
            is_alliance: true,
        }
    }

    // FUN_00458fe0: each type's base, then 0 built, 1 under construction (2
    // for side 1), 3 en route; mines and refineries 9001 and 9030.
    #[test]
    fn a_facility_picture_is_its_types_base_plus_its_state() {
        let shipyard = DatId::new(0x2800_0001);
        assert_eq!(
            facility_picture(shipyard, 2, FacilityState::Built),
            Some(9_006)
        );
        assert_eq!(
            facility_picture(shipyard, 2, FacilityState::UnderConstruction),
            Some(9_007)
        );
        assert_eq!(
            facility_picture(shipyard, 1, FacilityState::UnderConstruction),
            Some(9_008)
        );
        assert_eq!(
            facility_picture(shipyard, 1, FacilityState::EnRoute),
            Some(9_009)
        );
        let bases: Vec<_> = (1..=6)
            .map(|index| facility_picture(DatId::new(0x2a00_0000 | index), 1, FacilityState::Built))
            .collect();
        assert_eq!(bases, [9_006, 9_014, 9_022, 9_010, 9_018, 9_026].map(Some));
        assert_eq!(
            facility_picture(DatId::new(0x2c00_0001), 1, FacilityState::Built),
            Some(9_001)
        );
        assert_eq!(
            facility_picture(DatId::new(0x2d00_0002), 1, FacilityState::Built),
            Some(9_030)
        );
        assert_eq!(
            facility_picture(DatId::new(0x2800_0007), 1, FacilityState::Built),
            None
        );
    }

    // FUN_00456230: the bands' title strips by side, selected first.
    #[test]
    fn band_strips_and_item_frames_follow_the_shown_side() {
        assert_eq!(
            [band_strip(1, true), band_strip(1, false)],
            [10_291, 10_292]
        );
        assert_eq!(
            [band_strip(2, true), band_strip(2, false)],
            [10_293, 10_294]
        );
        assert_eq!(
            [band_strip(3, true), band_strip(0, false)],
            [10_295, 10_296]
        );
        assert_eq!(
            [item_frame(1), item_frame(2), item_frame(0)],
            [10_262, 10_263, 10_264]
        );
    }

    // FUN_00457b40, FUN_00457f30: an idle band says so and names its own
    // system as its destination.
    #[test]
    fn an_idle_band_shows_its_idle_text_and_its_own_system() {
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Bortras", ALLIANCE);
        let manufacturing = ManufacturingState::new();
        let bands: Vec<_> = BAND_AREAS
            .iter()
            .map(|&area| band(&world, &manufacturing, true, system, area))
            .collect();
        assert_eq!(
            bands
                .iter()
                .map(|band| band.status.as_str())
                .collect::<Vec<_>>(),
            [
                "No Ships are being built",
                "No Troops in training",
                "No Facilities are being built"
            ]
        );
        assert!(bands
            .iter()
            .all(|band| band.destination == "Destination: Bortras"
                && band.units.is_none()
                && band.mini.is_none()
                && band.progress.is_none()));
    }

    // FUN_00457c90: a product's name, mini and units left, "Building: " for
    // ships, and the area's destination.
    #[test]
    fn a_building_band_shows_its_product_units_progress_and_destination() {
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Bortras", ALLIANCE);
        let away = add_system(&mut world, "Denab", ALLIANCE);
        let class = world.capital_ship_classes.insert(CapitalShipClass {
            dat_id: DatId::new(0x1400_0040),
            name: "Corellian Corvette".into(),
            ..CapitalShipClass::default()
        });
        let mut manufacturing = ManufacturingState::new();
        manufacturing.set_destination(system, ProductionArea::Shipyard, away);
        manufacturing.build(
            system,
            &QueueItem::new(BuildableKind::CapitalShip(class), 10, 10),
            3,
        );

        let shown = band(
            &world,
            &manufacturing,
            true,
            system,
            ProductionArea::Shipyard,
        );
        assert_eq!(shown.status, "Corellian Corvette");
        assert_eq!(shown.mini, capital_ship_mini_id(DatId::new(0x1400_0040)));
        assert!(shown.mini.is_some());
        assert_eq!(shown.units.as_deref(), Some("Building: 3"));
        assert_eq!(shown.destination, "Destination: Denab");
        assert_eq!(shown.progress, Some(0.0));

        // Hidden from the player, the band shows nothing building.
        let hidden = band(
            &world,
            &manufacturing,
            false,
            system,
            ProductionArea::Shipyard,
        );
        assert_eq!(hidden.status, "No Ships are being built");
    }

    // FUN_00457c90: troops say "Training: ".
    #[test]
    fn a_training_band_counts_its_units_as_training() {
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Bortras", ALLIANCE);
        let mut manufacturing = ManufacturingState::new();
        manufacturing.build(
            system,
            &QueueItem::new(BuildableKind::Troop(DatId::new(0x1000_0002)), 4, 4),
            2,
        );
        let shown = band(
            &world,
            &manufacturing,
            true,
            system,
            ProductionArea::TrainingFacility,
        );
        assert_eq!(shown.status, "Alliance Army Regiment");
        assert_eq!(shown.units.as_deref(), Some("Training: 2"));
    }

    // FUN_004568a0: a page lists the shown side's facilities of its family
    // in walk order, then the ones being built for the system and the ones
    // travelling to it.
    #[test]
    fn a_page_lists_built_then_building_then_travelling_facilities() {
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Bortras", ALLIANCE);
        let elsewhere = add_system(&mut world, "Denab", ALLIANCE);
        let built = yard(&mut world, system, 0x2800_0001, true);
        yard(&mut world, system, 0x2800_0004, false);
        yard(&mut world, system, 0x2900_0002, true);
        let template = alliance(0x2800_0004);
        let mut manufacturing = ManufacturingState::new();
        manufacturing.enqueue(
            elsewhere,
            QueueItem::new(BuildableKind::ManufacturingFacility(template), 5, 5)
                .delivered_to(system),
        );
        // Built for its own system, this one is not listed here.
        manufacturing.enqueue(
            elsewhere,
            QueueItem::new(
                BuildableKind::ManufacturingFacility(alliance(0x2800_0001)),
                5,
                5,
            ),
        );
        let mut deliveries = DeliveryState::new();
        deliveries.depart(
            &world,
            &[rebellion_core::manufacturing::Departure {
                origin: elsewhere,
                destination: system,
                tick: 0,
                kind: BuildableKind::ManufacturingFacility(template),
            }],
        );

        let cells = page_cells(
            &world,
            &manufacturing,
            &deliveries,
            true,
            system,
            FacilityPage::Shipyards,
        );
        assert_eq!(
            cells,
            [
                FacilityCell {
                    item: FacilityItem::Manufacturing(built),
                    picture: 9_006
                },
                FacilityCell {
                    item: FacilityItem::Pending(0),
                    picture: 9_012
                },
                FacilityCell {
                    item: FacilityItem::Pending(1),
                    picture: 9_013
                },
            ]
        );
        assert_eq!(yard_count(&cells), "1:3");
        assert!(!page_is_empty(&cells));
        let none = page_cells(
            &world,
            &manufacturing,
            &deliveries,
            false,
            system,
            FacilityPage::Shipyards,
        );
        assert!(none.is_empty());
    }

    // FUN_004568a0 case 0x6c: the mines page fills its system's deposits
    // with empty-slot pictures; a page of deposits alone is empty.
    #[test]
    fn the_mines_page_fills_the_systems_deposits() {
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Bortras", ALLIANCE);
        world.systems[system].raw_materials = 3;
        let mine = production(&mut world, system, 0x2c00_0001);
        production(&mut world, system, 0x2d00_0002);
        let manufacturing = ManufacturingState::new();
        let deliveries = DeliveryState::new();

        let mines = page_cells(
            &world,
            &manufacturing,
            &deliveries,
            true,
            system,
            FacilityPage::Mines,
        );
        assert_eq!(
            mines.iter().map(|cell| cell.item).collect::<Vec<_>>(),
            [
                FacilityItem::Production(mine),
                FacilityItem::Deposit(1),
                FacilityItem::Deposit(2)
            ]
        );
        assert_eq!(mines[1].picture, DEPOSIT_PICTURE);
        let refineries = page_cells(
            &world,
            &manufacturing,
            &deliveries,
            true,
            system,
            FacilityPage::Refineries,
        );
        assert_eq!(refineries.len(), 1);
        assert_eq!(refineries[0].picture, 9_030);

        world.systems[system].production_facilities.clear();
        let empty = page_cells(
            &world,
            &manufacturing,
            &deliveries,
            true,
            system,
            FacilityPage::Mines,
        );
        assert_eq!(empty.len(), 3);
        assert!(page_is_empty(&empty));
    }

    // FUN_00458fe0: a mine being built shows its under-construction picture
    // (9001 + 2 for side 1) on the mines page, ahead of the empty deposits.
    #[test]
    fn a_mine_being_built_shows_as_under_construction() {
        let mut world = GameWorld::default();
        let system = add_system(&mut world, "Bortras", ALLIANCE);
        world.systems[system].raw_materials = 2;
        let mut manufacturing = ManufacturingState::new();
        manufacturing.enqueue(
            system,
            QueueItem::new(
                BuildableKind::ProductionFacility(alliance(0x2c00_0001)),
                5,
                5,
            ),
        );

        let mines = page_cells(
            &world,
            &manufacturing,
            &DeliveryState::new(),
            true,
            system,
            FacilityPage::Mines,
        );
        assert_eq!(
            mines,
            [
                FacilityCell {
                    item: FacilityItem::Pending(0),
                    picture: 9_003
                },
                FacilityCell {
                    item: FacilityItem::Deposit(1),
                    picture: DEPOSIT_PICTURE
                },
            ]
        );
    }

    // FUN_00452fc0: the window lists its contents for the player's own
    // systems, and for the other side's only where the player sees them.
    #[test]
    fn contents_show_at_the_players_explored_systems() {
        let mut world = GameWorld::default();
        let own = add_system(&mut world, "Bortras", ALLIANCE);
        let enemy = add_system(
            &mut world,
            "Denab",
            ControlKind::Controlled(Faction::Empire),
        );
        let unexplored = add_system(&mut world, "Kessel", ALLIANCE);
        world.systems[unexplored].exploration_status = ExplorationStatus::Unexplored;
        let fog = FogState::new(Faction::Alliance);

        assert!(contents_visible(&world, &fog, Faction::Alliance, own));
        assert!(!contents_visible(&world, &fog, Faction::Alliance, enemy));
        assert!(!contents_visible(
            &world,
            &fog,
            Faction::Alliance,
            unexplored
        ));
        assert!(contents_visible(&world, &fog, Faction::Empire, enemy));
    }
}
