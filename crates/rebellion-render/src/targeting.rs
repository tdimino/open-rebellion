//! The galaxy view's targeting mode for a Mission, Move or Confirmed Move
//! order. Recovery notes: `ghidra/notes/object-popup-menu.md`, "Targeting",
//! and `ghidra/notes/move-order.md`.
//!
//! Choosing one of those orders from the object pop-up menu hands it to the
//! galaxy view (`FUN_00429320`): mode `+0xc0` becomes 2, the order waits at `+0xc4`,
//! the mouse is captured and the cursor is REBEXE.EXE cursor 1002. The next
//! left-button release (`FUN_00422ce0`) targets the object under the point,
//! which only a child window can supply.

use egui_macroquad::egui;
use rebellion_core::fog::FogState;
use rebellion_core::ids::{FleetKey, SystemKey, TroopKey};
use rebellion_core::missions::MissionMember;
use rebellion_core::world::GameWorld;

use crate::bmp_cache::{resources::rebexe, BmpCache, DllSource};
use crate::cockpit::CockpitLayout;
use crate::defenses_window::DefensesWindowState;
use crate::fleet_window::FleetWindowState;
use crate::missions_window::MissionsWindowState;
use crate::sector_window::SectorWindowState;
use crate::system_window::SystemWindowState;

const CAPTURE_ID: &str = "original_targeting_capture";

/// The order that waits for a target, built by `FUN_00487c50` with the
/// menu's selection as its team.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetOrder {
    /// Mission: `FUN_0042a320` opens the mission dialog for the target.
    Mission(Vec<MissionMember>),
    /// Move (`0x201`), or Confirmed Move (`0x202`) when `confirmed`, for a
    /// team of fleets: one fleet's own, or a sector window's fleet icon's
    /// (`FUN_00512700`, kind `0x10`). port: a character's or special force's
    /// move is not ported.
    FleetMove {
        fleets: Vec<FleetKey>,
        confirmed: bool,
    },
    /// A regiment's Move (`0x201`). Released on a Fleet window it boards the
    /// fleet (`ghidra/notes/fleet-window.md`, "Loading a regiment onto a
    /// fleet").
    TroopMove { troop: TroopKey },
    /// A capital ship's Move (`0x201`). Released on a Fleet window it joins
    /// that fleet; on a system it forms a fleet of its own there
    /// (`ghidra/notes/fleet-join-split.md`).
    ShipMove {
        fleet: FleetKey,
        ships: Vec<usize>,
        roster: u64,
    },
    /// Destination (`0x214`): the target system becomes where production
    /// delivers. A facility icon's team is every production area of the
    /// system (`FUN_00512700` kind 4 → `FUN_0052c170`); a Manufacturing
    /// window band's is its own manager's `area`.
    Destination {
        system: SystemKey,
        area: Option<rebellion_core::manufacturing::ProductionArea>,
    },
}

/// What a release lands on: a system, or a fleet a Fleet window gives
/// (`+0x70`, `FUN_004a3130`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseTarget {
    System(SystemKey),
    Fleet { fleet: FleetKey, system: SystemKey },
}

impl ReleaseTarget {
    /// The system the target lies in.
    #[must_use]
    pub const fn system(self) -> SystemKey {
        match self {
            Self::System(system) | Self::Fleet { system, .. } => system,
        }
    }
}

/// An order waiting for its target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Targeting {
    order: TargetOrder,
}

/// How a release ends targeting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetingEnd {
    /// The order's target is set (vtable `+0x2c`).
    Target {
        order: TargetOrder,
        target: ReleaseTarget,
    },
    /// No object was under the point, so the order is destroyed.
    Dropped,
}

impl Targeting {
    #[must_use]
    pub fn new(order: TargetOrder) -> Self {
        Self { order }
    }

    #[must_use]
    pub fn order(&self) -> &TargetOrder {
        &self.order
    }

    /// `FUN_00422ce0`'s `WM_LBUTTONUP` in mode 2, given the target
    /// [`release_destination`] found under the point.
    #[must_use]
    pub fn release(self, target: Option<ReleaseTarget>) -> TargetingEnd {
        match target {
            Some(target) => TargetingEnd::Target {
                order: self.order,
                target,
            },
            None => TargetingEnd::Dropped,
        }
    }
}

/// The windows a release can land on.
#[derive(Clone, Copy)]
pub struct ReleaseWindows<'a> {
    pub sector: &'a SectorWindowState,
    pub system: &'a SystemWindowState,
    pub fleet: &'a FleetWindowState,
    pub defenses: &'a DefensesWindowState,
    pub missions: &'a MissionsWindowState,
}

/// The target a targeting release at `point` lands on. `FUN_00422ce0` asks
/// the child window under the point (`ChildWindowFromPointEx`); here the
/// topmost egui layer stands for it. A system, Defenses or Missions window
/// gives its own system, a sector window the planet under the point, a Fleet
/// window the fleet or system its `+0x70` gives; the galaxy map is drawn by
/// the view itself, so a release over it, or between planets, gives none and
/// the order is destroyed (`ghidra/notes/move-order.md`, "Hit tests").
///
/// Move and Confirmed Move ask `+0x70`, the container under the point.
/// port: Mission asks `+0x68`, which may answer a character or a fleet;
/// every object reduces to its system for a mission, so both hit tests give
/// the same answer. The walk up from a team member and Shift's pass-through
/// click are not ported.
#[must_use]
pub fn release_destination(
    ctx: &egui::Context,
    world: &GameWorld,
    movement: &rebellion_core::movement::MovementState,
    fog: &FogState,
    layout: CockpitLayout,
    windows: ReleaseWindows<'_>,
    point: egui::Pos2,
) -> Option<ReleaseTarget> {
    let layer = window_layer_at(ctx, point)?;
    if let Some(system) = windows.system.release_target(layer) {
        return Some(ReleaseTarget::System(system));
    }
    if let Some(system) = windows.defenses.release_target(layer) {
        return Some(ReleaseTarget::System(system));
    }
    if let Some(system) = windows.missions.release_target(layer) {
        return Some(ReleaseTarget::System(system));
    }
    if let Some(target) = windows
        .fleet
        .release_target(world, movement, fog, layout, layer, point)
    {
        return target;
    }
    windows
        .sector
        .release_target(world, layout, layer, point)
        .flatten()
        .map(ReleaseTarget::System)
}

/// The topmost visible area holding `point`, passing over the capture,
/// which covers the screen while targeting.
fn window_layer_at(ctx: &egui::Context, point: egui::Pos2) -> Option<egui::LayerId> {
    let capture = egui::Id::new(CAPTURE_ID);
    ctx.memory(|memory| {
        let layers: Vec<egui::LayerId> = memory.layer_ids().collect();
        layers.into_iter().rev().find(|layer| {
            layer.id != capture
                && memory.areas().is_visible(layer)
                && memory
                    .area_rect(layer.id)
                    .is_some_and(|rect| rect.contains(point))
        })
    })
}

/// The screen rectangle of a cursor of `size` canvas pixels whose hotspot is
/// at `pointer`. The original's 640 by 480 screen scales with the canvas, so
/// the cursor does too.
#[must_use]
pub fn targeting_cursor_rect(
    layout: CockpitLayout,
    pointer: egui::Pos2,
    size: egui::Vec2,
) -> egui::Rect {
    let (hot_x, hot_y) = rebexe::TARGETING_CURSOR_HOTSPOT;
    let scale = layout.scale;
    egui::Rect::from_min_size(pointer - egui::vec2(hot_x, hot_y) * scale, size * scale)
}

/// Hold the pointer for the galaxy view while targeting. `FUN_00429320`
/// captures the mouse, so no other window receives a press or release until
/// targeting ends; the map reads its release through macroquad, beneath egui.
pub fn capture_pointer(ctx: &egui::Context) {
    let screen = ctx.screen_rect();
    egui::Area::new(egui::Id::new(CAPTURE_ID))
        .order(egui::Order::Foreground)
        .fixed_pos(screen.min)
        .show(ctx, |ui| {
            ui.allocate_response(screen.size(), egui::Sense::click_and_drag());
        });
}

/// Draw the targeting cursor over every window at the pointer. egui paints a
/// painter-only layer after the areas of its order, so it also covers the
/// mission dialog, which shares Tooltip. Returns
/// false when cursor 1002 is not staged, so the caller keeps the system
/// cursor visible.
#[must_use]
pub fn draw_targeting_cursor(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    layout: CockpitLayout,
    pointer: egui::Pos2,
) -> bool {
    let Some(texture) = cache.get(ctx, DllSource::Rebexe, rebexe::TARGETING_CURSOR) else {
        return false;
    };
    let rect = targeting_cursor_rect(layout, pointer, texture.size_vec2());
    let painter = ctx.layer_painter(egui::LayerId::new(
        egui::Order::Tooltip,
        egui::Id::new("original_targeting_cursor"),
    ));
    painter.image(
        texture.id(),
        rect,
        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
        egui::Color32::WHITE,
    );
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cockpit::CockpitViewport;
    use rebellion_core::ids::CharacterKey;

    fn layout(scale: f32) -> CockpitLayout {
        let viewport = CockpitViewport {
            x: 10.0,
            y: 20.0,
            width: 640.0 * scale,
            height: 480.0 * scale,
        };
        CockpitLayout {
            canvas: viewport,
            galaxy: viewport,
            scale,
        }
    }

    fn mission() -> TargetOrder {
        TargetOrder::Mission(vec![MissionMember::Character(CharacterKey::default())])
    }

    #[test]
    fn a_release_on_a_system_targets_it_with_the_team() {
        // FUN_00422ce0 WM_LBUTTONUP, mode 2: the target is set (+0x2c).
        let system = SystemKey::default();
        let targeting = Targeting::new(mission());
        assert_eq!(targeting.order(), &mission());

        assert_eq!(
            targeting.release(Some(ReleaseTarget::System(system))),
            TargetingEnd::Target {
                order: mission(),
                target: ReleaseTarget::System(system),
            }
        );
    }

    #[test]
    fn a_move_keeps_its_fleet_and_kind_through_the_release() {
        // FUN_00422ce0 sets the target on the order it holds at +0xc4,
        // whatever its kind; FUN_00487cc0 later reads 0x201 or 0x202.
        let system = SystemKey::default();
        for confirmed in [false, true] {
            let order = TargetOrder::FleetMove {
                fleets: vec![FleetKey::default()],
                confirmed,
            };
            let target = ReleaseTarget::System(system);
            assert_eq!(
                Targeting::new(order.clone()).release(Some(target)),
                TargetingEnd::Target { order, target }
            );
        }
    }

    #[test]
    fn a_release_on_no_object_drops_the_order() {
        // FUN_00422ce0: with no object under the point the order is destroyed.
        assert_eq!(
            Targeting::new(mission()).release(None),
            TargetingEnd::Dropped
        );
        let order = TargetOrder::FleetMove {
            fleets: vec![FleetKey::default()],
            confirmed: true,
        };
        assert_eq!(Targeting::new(order).release(None), TargetingEnd::Dropped);
    }

    #[test]
    fn the_cursor_hotspot_sits_on_the_pointer_at_every_scale() {
        // REBEXE.EXE cursor 1002: 32 by 32, hotspot (12, 12).
        let size = egui::vec2(32.0, 32.0);
        let pointer = egui::pos2(300.0, 200.0);

        let at_1 = targeting_cursor_rect(layout(1.0), pointer, size);
        assert_eq!(at_1.min, egui::pos2(288.0, 188.0));
        assert_eq!(at_1.size(), size);

        let at_2 = targeting_cursor_rect(layout(2.0), pointer, size);
        assert_eq!(at_2.min, egui::pos2(276.0, 176.0));
        assert_eq!(at_2.size(), egui::vec2(64.0, 64.0));
    }

    #[test]
    fn the_staged_cursor_is_drawn_on_top_with_its_hotspot_on_the_pointer() {
        // REBEXE.EXE cursor 1002, staged by extract-dll-resources.py --cursors.
        let root = std::env::temp_dir().join(format!("targeting-cursor-{}", std::process::id()));
        let dir = root.join("rebexe-exe/BMP");
        std::fs::create_dir_all(&dir).unwrap();
        let (side, stride, offset) = (32usize, 96usize, 54usize);
        let mut bmp = vec![0xff_u8; offset + stride * side];
        bmp[..2].copy_from_slice(b"BM");
        bmp[10..14].copy_from_slice(&(offset as u32).to_le_bytes());
        bmp[14..18].copy_from_slice(&40u32.to_le_bytes());
        bmp[18..22].copy_from_slice(&(side as i32).to_le_bytes());
        bmp[22..26].copy_from_slice(&(side as i32).to_le_bytes());
        bmp[26..28].copy_from_slice(&1u16.to_le_bytes());
        bmp[28..30].copy_from_slice(&24u16.to_le_bytes());
        bmp[30..54].fill(0);
        std::fs::write(dir.join("1002.bmp"), bmp).unwrap();
        let mut cache = BmpCache::new();
        cache.set_base_path(&root);

        let ctx = egui::Context::default();
        // A new Area is laid out unseen on its first frame; draw two.
        let mut output = None;
        let mut drawn = false;
        for _ in 0..2 {
            output = Some(ctx.run(egui::RawInput::default(), |ctx| {
                drawn =
                    draw_targeting_cursor(ctx, &mut cache, layout(2.0), egui::pos2(100.0, 100.0));
                // The mission dialog's order, drawn after the cursor.
                egui::Area::new(egui::Id::new("window_below"))
                    .order(egui::Order::Tooltip)
                    .show(ctx, |ui| {
                        ui.label("a window");
                    });
            }));
        }
        let output = output.unwrap();
        assert!(drawn);
        assert!(
            output
                .shapes
                .iter()
                .any(|clipped| matches!(clipped.shape, egui::Shape::Text(_))),
            "the window below is painted"
        );
        let texture = cache
            .get(&ctx, DllSource::Rebexe, rebexe::TARGETING_CURSOR)
            .unwrap()
            .id();
        std::fs::remove_dir_all(root).unwrap();

        let last = output.shapes.last().expect("the cursor is drawn");
        let egui::Shape::Mesh(mesh) = &last.shape else {
            panic!("the topmost shape is not the cursor: {:?}", last.shape);
        };
        assert_eq!(mesh.texture_id, texture);
        assert_eq!(
            mesh.calc_bounds(),
            egui::Rect::from_min_size(egui::pos2(76.0, 76.0), egui::vec2(64.0, 64.0))
        );
    }

    #[test]
    fn the_capture_keeps_a_click_from_the_window_below() {
        // FUN_00429320 captures the mouse for the galaxy view.
        let click = |capture: bool| {
            let ctx = egui::Context::default();
            let mut clicked = false;
            for frame in 0..4 {
                let pointer = egui::pos2(20.0, 20.0);
                let mut events = vec![egui::Event::PointerMoved(pointer)];
                if frame >= 2 {
                    events.push(egui::Event::PointerButton {
                        pos: pointer,
                        button: egui::PointerButton::Primary,
                        pressed: frame == 2,
                        modifiers: egui::Modifiers::NONE,
                    });
                }
                let input = egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(640.0, 480.0),
                    )),
                    events,
                    ..Default::default()
                };
                let _ = ctx.run(input, |ctx| {
                    egui::Area::new(egui::Id::new("window_below"))
                        .fixed_pos(egui::Pos2::ZERO)
                        .show(ctx, |ui| {
                            let response = ui
                                .allocate_response(egui::vec2(100.0, 100.0), egui::Sense::click());
                            clicked |= response.clicked();
                        });
                    if capture {
                        capture_pointer(ctx);
                    }
                });
            }
            clicked
        };

        assert!(click(false), "the window takes the click without a capture");
        assert!(!click(true));
    }

    #[test]
    fn an_unstaged_cursor_draws_nothing_and_says_so() {
        let mut cache = BmpCache::new();
        let ctx = egui::Context::default();
        let mut drawn = true;
        let output = ctx.run(egui::RawInput::default(), |ctx| {
            drawn = draw_targeting_cursor(ctx, &mut cache, layout(1.0), egui::pos2(10.0, 10.0));
        });

        assert!(!drawn);
        assert!(output.shapes.is_empty());
    }

    mod release {
        use super::*;
        use crate::cockpit::CockpitFaction;
        use crate::defenses_window::draw_defenses_windows;
        use crate::fleet_window::draw_fleet_windows;
        use crate::missions_window::draw_missions_windows;
        use crate::sector_window::draw_sector_windows;
        use crate::system_window::draw_system_windows;
        use rebellion_core::dat::{ExplorationStatus, Faction, SectorGroup};
        use rebellion_core::ids::DatId;
        use rebellion_core::missions::MissionState;
        use rebellion_core::troop_transport::TroopTransportState;
        use rebellion_core::world::{ControlKind, Fleet, Sector, System};

        /// A sector at (317, 248) holding two systems whose planets sit at
        /// (14, 44) and (159, 89) in its window.
        fn world() -> (GameWorld, SystemKey, SystemKey) {
            let mut world = GameWorld::default();
            let sector = world.sectors.insert(Sector {
                dat_id: DatId::new(36),
                name: "Sesswenna".into(),
                group: SectorGroup::Core,
                x: 317,
                y: 248,
                systems: Vec::new(),
            });
            let mut system = |x, y| {
                let key = world.systems.insert(System {
                    dat_id: DatId::new(100),
                    name: "System".into(),
                    sector,
                    x,
                    y,
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
                    control: ControlKind::Uncontrolled,
                });
                world.sectors[sector].systems.push(key);
                key
            };
            let first = system(322, 260);
            let second = system(373, 272);
            (world, first, second)
        }

        /// Draw the windows and the capture twice, so every area is laid
        /// out and visible, then release at the canvas point `at`.
        fn release_at(
            world: &GameWorld,
            sectors: &mut SectorWindowState,
            systems: &mut SystemWindowState,
            at: (f32, f32),
        ) -> Option<SystemKey> {
            release_on(
                world,
                sectors,
                systems,
                &mut FleetWindowState::default(),
                &mut DefensesWindowState::default(),
                at,
            )
            .map(|target| match target {
                ReleaseTarget::System(system) => system,
                other @ ReleaseTarget::Fleet { .. } => panic!("no fleet window: {other:?}"),
            })
        }

        fn release_on(
            world: &GameWorld,
            sectors: &mut SectorWindowState,
            systems: &mut SystemWindowState,
            fleets: &mut FleetWindowState,
            defenses: &mut DefensesWindowState,
            at: (f32, f32),
        ) -> Option<ReleaseTarget> {
            release_on_all(
                world,
                sectors,
                systems,
                fleets,
                defenses,
                (&mut MissionsWindowState::default(), &MissionState::new()),
                at,
            )
        }

        fn release_on_all(
            world: &GameWorld,
            sectors: &mut SectorWindowState,
            systems: &mut SystemWindowState,
            fleets: &mut FleetWindowState,
            defenses: &mut DefensesWindowState,
            (missions_windows, missions): (&mut MissionsWindowState, &MissionState),
            at: (f32, f32),
        ) -> Option<ReleaseTarget> {
            let layout = layout(1.0);
            let ctx = egui::Context::default();
            let mut cache = BmpCache::new();
            let fog = FogState::new(Faction::Alliance);
            let uprisings = rebellion_core::uprising::UprisingState::default();
            for _ in 0..2 {
                let input = egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(700.0, 520.0),
                    )),
                    ..Default::default()
                };
                let _ = ctx.run(input, |ctx| {
                    let faction = CockpitFaction::Alliance;
                    let _ = draw_sector_windows(
                        ctx,
                        world,
                        &rebellion_core::movement::MovementState::default(),
                        &fog,
                        sectors,
                        faction,
                        layout,
                        &mut cache,
                        &uprisings,
                        missions,
                    );
                    let _ = draw_system_windows(
                        ctx,
                        world,
                        &rebellion_core::movement::MovementState::default(),
                        &fog,
                        missions,
                        &rebellion_core::manufacturing::ManufacturingState::new(),
                        &rebellion_core::delivery::DeliveryState::new(),
                        systems,
                        faction,
                        layout,
                        &mut cache,
                    );
                    let _ = draw_fleet_windows(
                        ctx,
                        world,
                        &rebellion_core::movement::MovementState::default(),
                        &fog,
                        &TroopTransportState::default(),
                        fleets,
                        faction,
                        layout,
                        &mut cache,
                    );
                    let _ = draw_defenses_windows(
                        ctx,
                        world,
                        &fog,
                        &rebellion_core::missions::MissionState::new(),
                        &rebellion_core::economy::EconomyState::default(),
                        defenses,
                        faction,
                        layout,
                        &mut cache,
                    );
                    let _ = draw_missions_windows(
                        ctx,
                        world,
                        &fog,
                        missions,
                        missions_windows,
                        faction,
                        layout,
                        &mut cache,
                    );
                    capture_pointer(ctx);
                });
            }
            let point = egui::pos2(layout.canvas.x + at.0, layout.canvas.y + at.1);
            let windows = ReleaseWindows {
                sector: sectors,
                system: systems,
                fleet: fleets,
                defenses,
                missions: missions_windows,
            };
            release_destination(
                &ctx,
                world,
                &rebellion_core::movement::MovementState::default(),
                &fog,
                layout,
                windows,
                point,
            )
        }

        /// The Alliance's first sector window sits at (60, 35).
        fn planet_center(planet: (f32, f32)) -> (f32, f32) {
            (60.0 + planet.0 + 18.5, 35.0 + planet.1 + 18.5)
        }

        #[test]
        fn a_release_on_a_sector_windows_planet_targets_that_system() {
            // FUN_0045c830 → FUN_0045c660: the item whose 37 by 37 rectangle
            // holds the point (FUN_00459e30).
            let (world, first, second) = world();
            let mut sectors = SectorWindowState::default();
            sectors.open_for_system(&world, first, CockpitFaction::Alliance);
            let mut systems = SystemWindowState::default();

            let at = planet_center((14.0, 44.0));
            assert_eq!(
                release_at(&world, &mut sectors, &mut systems, at),
                Some(first)
            );
            let at = planet_center((159.0, 89.0));
            assert_eq!(
                release_at(&world, &mut sectors, &mut systems, at),
                Some(second)
            );
            // Past the first planet's right edge, between the two.
            let at = (60.0 + 14.0 + 37.0, 35.0 + 44.0 + 18.5);
            assert_eq!(release_at(&world, &mut sectors, &mut systems, at), None);
        }

        #[test]
        fn a_planets_screen_rect_is_where_a_release_targets_it() {
            let (world, first, second) = world();
            let layout = layout(1.0);
            let mut sectors = SectorWindowState::default();
            assert_eq!(sectors.planet_screen_rect(&world, layout, second), None);

            sectors.open_for_system(&world, first, CockpitFaction::Alliance);
            let rect = sectors
                .planet_screen_rect(&world, layout, second)
                .expect("the sector's window is open");
            let min = egui::pos2(
                layout.canvas.x + 60.0 + 159.0,
                layout.canvas.y + 35.0 + 89.0,
            );
            assert_eq!(rect, egui::Rect::from_min_size(min, egui::vec2(37.0, 37.0)));
            let center = rect.center();
            let at = (center.x - layout.canvas.x, center.y - layout.canvas.y);
            let mut systems = SystemWindowState::default();
            assert_eq!(
                release_at(&world, &mut sectors, &mut systems, at),
                Some(second)
            );
        }

        #[test]
        fn a_release_on_the_bare_galaxy_map_drops_the_order() {
            // FUN_00422ce0 finds no child window over the map the view draws
            // itself, so the order is destroyed.
            let (world, first, _) = world();
            let mut sectors = SectorWindowState::default();
            sectors.open_for_system(&world, first, CockpitFaction::Alliance);
            let mut systems = SystemWindowState::default();

            assert_eq!(
                release_at(&world, &mut sectors, &mut systems, (600.0, 450.0)),
                None
            );
        }

        #[test]
        fn a_release_anywhere_in_a_system_window_targets_its_system() {
            // FUN_004aa470: the window's system wherever the point lies.
            let (world, first, second) = world();
            let mut sectors = SectorWindowState::default();
            sectors.open_for_system(&world, first, CockpitFaction::Alliance);
            let mut systems = SystemWindowState::default();
            systems.open(
                &world,
                second,
                (70, 90),
                CockpitFaction::Alliance,
                layout(1.0),
            );

            // Over the first planet, which the system window covers.
            let at = planet_center((14.0, 44.0));
            assert_eq!(
                release_at(&world, &mut sectors, &mut systems, at),
                Some(second)
            );
        }

        fn add_fleet(world: &mut GameWorld, system: SystemKey, is_alliance: bool) -> FleetKey {
            let fleet = world.fleets.insert(Fleet {
                location: system,
                capital_ships: Vec::new(),
                fighters: Vec::new(),
                characters: Vec::new(),
                is_alliance,
                has_death_star: false,
            });
            world.systems[system].fleets.push(fleet);
            fleet
        }

        #[test]
        fn a_release_on_a_fleet_windows_fleet_targets_that_fleet() {
            // FUN_004a3130: in the left list, the entry under the point.
            let (mut world, first, _) = world();
            world.systems[first].control = ControlKind::Controlled(Faction::Alliance);
            let fleet = add_fleet(&mut world, first, true);
            let mut fleets = FleetWindowState::default();
            fleets.open(
                &world,
                first,
                (70, 90),
                CockpitFaction::Alliance,
                layout(1.0),
            );

            // The first entry spans (4, 29) to (95, 79) in the window.
            let at = (70.0 + 40.0, 90.0 + 50.0);
            assert_eq!(
                release_on(
                    &world,
                    &mut SectorWindowState::default(),
                    &mut SystemWindowState::default(),
                    &mut fleets,
                    &mut DefensesWindowState::default(),
                    at,
                ),
                Some(ReleaseTarget::Fleet {
                    fleet,
                    system: first
                })
            );
            // Below the last entry, the list gives the subject.
            let at = (70.0 + 40.0, 90.0 + 150.0);
            assert_eq!(
                release_on(
                    &world,
                    &mut SectorWindowState::default(),
                    &mut SystemWindowState::default(),
                    &mut fleets,
                    &mut DefensesWindowState::default(),
                    at,
                ),
                Some(ReleaseTarget::System(first))
            );
        }

        #[test]
        fn a_fleet_window_over_a_sector_window_answers_the_release() {
            // ChildWindowFromPointEx finds the topmost child.
            let (mut world, first, _) = world();
            world.systems[first].control = ControlKind::Controlled(Faction::Alliance);
            let fleet = add_fleet(&mut world, first, true);
            let mut sectors = SectorWindowState::default();
            sectors.open_for_system(&world, first, CockpitFaction::Alliance);
            let mut fleets = FleetWindowState::default();
            // Over the first planet at (74, 79) in the canvas.
            fleets.open(
                &world,
                first,
                (40, 50),
                CockpitFaction::Alliance,
                layout(1.0),
            );

            let at = (40.0 + 40.0, 50.0 + 50.0);
            assert_eq!(
                release_on(
                    &world,
                    &mut sectors,
                    &mut SystemWindowState::default(),
                    &mut fleets,
                    &mut DefensesWindowState::default(),
                    at,
                ),
                Some(ReleaseTarget::Fleet {
                    fleet,
                    system: first
                })
            );
        }
        #[test]
        fn a_release_on_a_defenses_window_targets_its_system_even_over_a_row() {
            // FUN_004aa470 (+0x70): the window's subject wherever the point
            // is, so a move never targets a listed object.
            let (mut world, first, _) = world();
            world.systems[first].control = ControlKind::Controlled(Faction::Alliance);
            let troop = world.troops.insert(rebellion_core::world::TroopUnit {
                class_dat_id: rebellion_core::ids::DatId::new(0x1000_0001),
                is_alliance: true,
                regiment_strength: 100,
            });
            world.systems[first].ground_units.push(troop);
            let mut sectors = SectorWindowState::default();
            sectors.open_for_system(&world, first, CockpitFaction::Alliance);
            let mut defenses = DefensesWindowState::default();
            defenses.open(
                &world,
                first,
                (40, 50),
                CockpitFaction::Alliance,
                layout(1.0),
            );

            // The first cell spans (7, 81) to (77, 151) in the window; the
            // window covers the first planet at (74, 79).
            for at in [(40.0 + 30.0, 50.0 + 100.0), (40.0 + 150.0, 50.0 + 40.0)] {
                assert_eq!(
                    release_on(
                        &world,
                        &mut sectors,
                        &mut SystemWindowState::default(),
                        &mut FleetWindowState::default(),
                        &mut defenses,
                        at,
                    ),
                    Some(ReleaseTarget::System(first)),
                    "{at:?}"
                );
            }
        }

        #[test]
        fn a_release_on_a_missions_window_targets_its_system_even_over_a_row() {
            // FUN_004aa470 (+0x70) answers for type 11 too: the subject
            // wherever the point is (move-order.md, "Hit tests").
            let (mut world, first, _) = world();
            let agent = world.characters.insert(rebellion_core::world::Character {
                dat_id: rebellion_core::ids::DatId::new(832),
                name: "Agent".into(),
                is_alliance: true,
                current_system: Some(first),
                recruited: true,
                ..Default::default()
            });
            let mut missions = MissionState::new();
            missions.dispatch(rebellion_core::missions::MissionRequest::single(
                rebellion_core::missions::MissionKind::Diplomacy,
                rebellion_core::missions::MissionFaction::Alliance,
                agent,
                first,
                None,
                0,
            ));
            let fog = FogState::new(Faction::Alliance);
            let mut windows = MissionsWindowState::default();
            assert!(windows.open(
                &world,
                &fog,
                &missions,
                first,
                (40, 50),
                CockpitFaction::Alliance,
                layout(1.0),
            ));

            // The first mission row spans (5, 24) to (95, 74) in the window.
            for at in [(40.0 + 30.0, 50.0 + 40.0), (40.0 + 150.0, 50.0 + 200.0)] {
                assert_eq!(
                    release_on_all(
                        &world,
                        &mut SectorWindowState::default(),
                        &mut SystemWindowState::default(),
                        &mut FleetWindowState::default(),
                        &mut DefensesWindowState::default(),
                        (&mut windows, &missions),
                        at,
                    ),
                    Some(ReleaseTarget::System(first)),
                    "{at:?}"
                );
            }
        }
    }
}
