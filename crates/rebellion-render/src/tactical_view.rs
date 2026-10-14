//! Tactical combat view — 2D battlefield for player-involved space battles.
//!
//! The battle engine (`BattleSession` and everything it steps) lives in
//! `rebellion_core::tactical` and is re-exported here; this module draws it,
//! takes the player's input and plays its queued cues.
//!
//! A battle the player meets opens the Battle Alert and this view
//! (`GameMode::TacticalCombat`); its Simulate and every unwatched battle run
//! the same session headless (`BattleSession::simulate_to_end`).
//!
//! # Phases
//!
//! 1. **Placement**: Player positions capital ships in their deployment zone.
//!    Fighters auto-deploy around their carrier. AI side auto-places.
//! 2. **Combat**: Real-time phased combat with HUD (Task #8).
//! 3. **Results**: Summary screen showing losses (Task #10).
//!
//! # Integration
//!
//! ```ignore
//! // In main.rs combat resolution loop:
//! if battle_involves_player {
//!     tactical_state = TacticalState::new_battle(...);
//!     game_mode = GameMode::TacticalCombat;
//! } else {
//!     integrator::resolve_system_space_combat(...); // headless session
//! }
//! ```

use std::collections::{HashMap, HashSet};

use macroquad::prelude::*;
use rebellion_core::combat::CombatForce;
use rebellion_core::ids::{FleetKey, SystemKey};
use rebellion_core::troop_transport::TroopTransportState;
use rebellion_core::world::GameWorld;

use crate::bmp_cache::{resources, BmpCache, DllSource};
use crate::scroll_bar::{ScrollBar, ScrollPointer, ScrollRange, LIST_BAR_29FC};
#[cfg(feature = "interface-test-fixtures")]
use crate::tactical_assets::TacticalLodView;
use crate::tactical_assets::{
    OriginalTacticalCamera, TacticalAssetRenderer, TacticalEffectRenderObject,
    TacticalFighterRenderObject, TacticalProjectileRenderObject, TacticalRenderObject,
    TacticalScreenProjection,
};
use rebellion_core::tactical::resource_ids::TacticalCapitalShipResource;
pub use rebellion_core::tactical::*;

/// The original TACTICAL.DLL background is a complete 640×480 composition.
const TACTICAL_WIDTH: f32 = 640.0;
const TACTICAL_HEIGHT: f32 = 480.0;
/// Measured black aperture inside bitmap 1000. All battle primitives stay here.
const BATTLE_APERTURE: NativeRect = NativeRect::new(16.0, 28.0, 444.0, 439.0);

/// Default placement of the modeless 470x331 Battle Alert / Results window
/// on the original 640x480 command-center canvas. The original window is
/// draggable; the deterministic launcher keeps this centered position.
const BATTLE_RESULTS_WINDOW_X: f32 = 85.0;
const BATTLE_RESULTS_WINDOW_Y: f32 = 74.0;
const BATTLE_RESULTS_SCENE_X: f32 = 12.0;
const BATTLE_RESULTS_SCENE_Y: f32 = 13.0;
const BATTLE_RESULTS_RAIL_X: f32 = 412.0;
const BATTLE_ALERT_WINDOW_X: f32 = 65.0;
const BATTLE_ALERT_WINDOW_Y: f32 = 54.0;

/// Selected-capital-ship display panel.
const TACTICAL_SELECTED_SHIP_PANEL: u32 = 1302;

/// `FUN_005e4110` builds two `CoolwinStatusBar` meters (`FUN_0060e150`) at
/// panel-local (34, 81) and (103, 81), 37 by 10, each a blue fill
/// (0x02ff0000) over a red background (0x020000ff). The refresh at
/// 0x005e50ce sets the left one to hull (`FUN_005b1640`, +0x3cc / +0x3c8)
/// and the right one to shields (`FUN_005b1650`, +0x3c0 / +0x3bc, 0 for a
/// ship without shields), as truncated percentages.
const TACTICAL_SELECTED_HULL_METER: NativeRect = NativeRect::new(516.0, 109.0, 37.0, 10.0);
const TACTICAL_SELECTED_SHIELD_METER: NativeRect = NativeRect::new(585.0, 109.0, 37.0, 10.0);
/// `FUN_005e4b40` draws the selected ship's 96 by 39 art at panel-local
/// (0x1a, 0x11).
const TACTICAL_SELECTED_SHIP_ART_X: f32 = 508.0;
const TACTICAL_SELECTED_SHIP_ART_Y: f32 = 45.0;
/// `FUN_005e4880` labels the panel in font 8, white, left-aligned from the
/// top-left of each box: name (26, 7), task force (26, 52), order (15, 182)
/// and tactic (15, 193), panel-local.
/// hyp: measured, not traced. A0 TAC-03-C001's glyphs measure at a 9-pixel
/// em (cap height 7, "No Orders" 41 wide), each baseline 9 below its box top.
const TACTICAL_SELECTED_NAME_POINT: (f32, f32) = (508.0, 44.0);
const TACTICAL_SELECTED_TASK_FORCE_POINT: (f32, f32) = (508.0, 89.0);
const TACTICAL_SELECTED_ORDER_POINT: (f32, f32) = (497.0, 219.0);
const TACTICAL_SELECTED_TACTIC_POINT: (f32, f32) = (497.0, 230.0);
const TACTICAL_SELECTED_TEXT_EM: f32 = 9.0;
/// `FUN_005e1fd0` labels the fighter-group panel (id `0x17`) in font 8,
/// white: group name (26, 9), order (20, 20) and tactics (20, 31);
/// `FUN_005e16a0` moves the count label to (23, 146) at every refresh
/// (`FUN_00601b30(+0xb0, 0x17, 0x92)`). Panel-local. A0 TAC-03-C019 puts
/// each baseline 9 below its box top, left-aligned at the box x:
/// `FUN_00601b80` drops `DT_CENTER` when the text is narrower than the box.
const TACTICAL_FIGHTER_NAME_POINT: (f32, f32) = (508.0, 46.0);
const TACTICAL_FIGHTER_ORDER_POINT: (f32, f32) = (502.0, 57.0);
const TACTICAL_FIGHTER_TACTIC_POINT: (f32, f32) = (502.0, 68.0);
const TACTICAL_FIGHTER_COUNT_POINT: (f32, f32) = (505.0, 183.0);
/// `FUN_005fcc30` copies the 102 by 107 fighter art (TACTICAL 2030-2037)
/// to panel-local (23, 59).
const TACTICAL_FIGHTER_ART_X: f32 = 505.0;
const TACTICAL_FIGHTER_ART_Y: f32 = 87.0;
/// TEXTTACT `0xddc5`-`0xddcc`, the count label for fighter types
/// `0x1d`-`0x24` (`FUN_005e16a0`).
const TACTICAL_FIGHTER_TYPE_LABELS: [&str; 8] = [
    "X-wings:",
    "Y-wings:",
    "A-wings:",
    "B-wings:",
    "Fighters:",
    "Intrcpts:",
    "Bombers:",
    "Defenders:",
];

const TACTICAL_SUBSYSTEM_X: [f32; 5] = [492.0, 519.0, 546.0, 573.0, 600.0];
const TACTICAL_SUBSYSTEM_Y: f32 = 131.0;

#[derive(Debug, Clone, Copy, PartialEq)]
struct NativeRect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl NativeRect {
    const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn contains(self, x: f32, y: f32) -> bool {
        x >= self.x && y >= self.y && x < self.x + self.width && y < self.y + self.height
    }
}

/// One uniformly scaled, letterboxed copy of the original tactical canvas.
#[derive(Debug, Clone, Copy)]
struct TacticalCanvas {
    x: f32,
    y: f32,
    scale: f32,
}

impl TacticalCanvas {
    fn new(width: f32, height: f32) -> Self {
        let scale = (width / TACTICAL_WIDTH)
            .min(height / TACTICAL_HEIGHT)
            .max(0.01);
        Self {
            x: (width - TACTICAL_WIDTH * scale) * 0.5,
            y: (height - TACTICAL_HEIGHT * scale) * 0.5,
            scale,
        }
    }

    fn point(self, x: f32, y: f32) -> (f32, f32) {
        (self.x + x * self.scale, self.y + y * self.scale)
    }

    fn logical_pointer(self, x: f32, y: f32) -> (f32, f32) {
        ((x - self.x) / self.scale, (y - self.y) / self.scale)
    }

    fn aperture(self) -> NativeRect {
        let (x, y) = self.point(BATTLE_APERTURE.x, BATTLE_APERTURE.y);
        NativeRect::new(
            x,
            y,
            BATTLE_APERTURE.width * self.scale,
            BATTLE_APERTURE.height * self.scale,
        )
    }

    fn arena_transform(self, zoom: f32, camera_x: f32, camera_y: f32) -> (f32, f32, f32) {
        let aperture = self.aperture();
        let scale = (aperture.width / ARENA_WIDTH).min(aperture.height / ARENA_HEIGHT) * zoom;
        let x = aperture.x + aperture.width * 0.5 - (ARENA_WIDTH * 0.5 + camera_x) * scale;
        let y = aperture.y + aperture.height * 0.5 - (ARENA_HEIGHT * 0.5 + camera_y) * scale;
        (scale, x, y)
    }
}

/// Original right-rail destination inside the Battle Results window.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BattleResultTab {
    #[default]
    Summary,
    AllianceForces,
    ImperialForces,
    GoDirectlyTo,
}

/// Unit family selected inside an Alliance or Imperial force-detail tab.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BattleResultCategory {
    #[default]
    CapitalShips,
    Fighters,
    Troops,
    Personnel,
}

/// Original right-rail page inside the pre-battle Battle Alert window.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum BattleAlertTab {
    #[default]
    Summary,
    AllianceForces,
    ImperialForces,
    System,
}

impl BattleAlertTab {
    const ALL: [Self; 4] = [
        Self::Summary,
        Self::AllianceForces,
        Self::ImperialForces,
        Self::System,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Summary => "summary",
            Self::AllianceForces => "alliance-forces",
            Self::ImperialForces => "imperial-forces",
            Self::System => "system-summary",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BattleAlertControl {
    Retreat,
    Simulate,
    TakeCommand,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum TacticalCommandPanel {
    #[default]
    Display,
    Maneuvers {
        pending_order: TacticalOrder,
        pending_tactic: TacticalTactic,
    },
    Missions {
        pending_order: TacticalOrder,
    },
}

#[derive(Debug, Clone, Copy)]
struct TacticalCameraBookmark {
    source: OriginalTacticalCamera,
    camera_x: f32,
    camera_y: f32,
    zoom: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TacticalCameraFollow {
    TaskForce(u8),
    FighterGroup(u8),
}

/// Persistent state for the tactical combat view across frames.
pub struct TacticalState {
    /// The current battle session (None when not in combat).
    pub session: Option<BattleSession>,
    /// Ship being dragged during placement (index in session.ships).
    pub dragging_ship: Option<usize>,
    /// Mouse offset from ship center during drag.
    pub drag_offset: (f32, f32),
    /// Camera offset for panning the battlefield.
    pub camera_x: f32,
    pub camera_y: f32,
    /// Zoom level.
    pub zoom: f32,
    /// Independently visible navigation-point sets 1 through 4.
    navigation_sets_visible: [bool; 4],
    logged_navigation_scene: Option<[bool; 4]>,
    /// Default and player-memorized camera states for resources 1040–1043.
    default_camera: Option<TacticalCameraBookmark>,
    memorized_camera: Option<TacticalCameraBookmark>,
    /// Keyboard-driven task-force or fighter-group chase target.
    camera_follow: Option<TacticalCameraFollow>,
    /// Original Tactical Display faction-wireframe switches.
    pub highlight_alliance: bool,
    pub highlight_empire: bool,
    /// Active source tab in the post-battle Battle Results window.
    pub battle_result_tab: BattleResultTab,
    /// Active force family in the Alliance or Imperial result tab.
    pub battle_result_category: BattleResultCategory,
    /// The original strategic Battle Alert owns entry until the player takes
    /// command, simulates, or attempts to retreat.
    battle_alert_open: bool,
    battle_alert_tab: BattleAlertTab,
    /// The first row the Battle Alert's list shows; a tab change refills
    /// the list from the top (`FUN_00450660` → `FUN_0060a280`).
    battle_alert_scroll: usize,
    /// The scroll arrow held down, drawn pressed.
    battle_alert_scroll_drag: crate::scroll_bar::ScrollDrag,
    /// The Battle Alert's title and summary, formatted from TEXTSTRA by the
    /// caller (`FUN_0049c0d0`); `None` keeps the port's own wording.
    battle_alert_text: Option<(String, String)>,
    battle_alert_pressed: Option<BattleAlertControl>,
    /// The strategic world already consumed the auto-resolve result. Closing
    /// the result window must not apply the same losses a second time.
    strategic_results_applied: bool,
    /// Whether the authentic Battle Options panel owns the right-hand housing.
    battle_options_open: bool,
    /// Captured Battle Options control. Dispatch occurs only on a matching release.
    battle_options_pressed: Option<TacticalBattleOptionsControl>,
    /// The native withdrawal confirmation replaces the right-hand options panel.
    withdraw_confirmation_open: bool,
    /// Captured confirmation control. Dispatch occurs only on a matching release.
    withdraw_confirmation_pressed: Option<TacticalWithdrawConfirmationControl>,
    /// The player handed command of their force to the tactical simulation.
    player_observing: bool,
    /// Captured press on the source Death Star superlaser control.
    death_star_laser_pressed: bool,
    /// The next valid right-click assigns the superlaser target.
    death_star_targeting: bool,
    command_panel: TacticalCommandPanel,
    warmed_font_sizes: HashSet<u16>,
    render_original_participants: bool,
    render_original_starfield: bool,
    render_original_planet: bool,
    render_pyrotechnics: bool,
    display_holocube: bool,
    asset_renderer: TacticalAssetRenderer,
    #[cfg(feature = "interface-test-fixtures")]
    suppress_mapped_capital_fallback: bool,
    #[cfg(feature = "interface-test-fixtures")]
    suppress_mapped_fighter_fallback: bool,
    #[cfg(feature = "interface-test-fixtures")]
    proof_resource_2560: bool,
    #[cfg(feature = "interface-test-fixtures")]
    proof_lod_follows_zoom: bool,
}

impl Default for TacticalState {
    fn default() -> Self {
        Self {
            session: None,
            dragging_ship: None,
            drag_offset: (0.0, 0.0),
            camera_x: 0.0,
            camera_y: 0.0,
            zoom: 1.0,
            navigation_sets_visible: [false; 4],
            logged_navigation_scene: None,
            default_camera: None,
            memorized_camera: None,
            camera_follow: None,
            highlight_alliance: true,
            highlight_empire: true,
            battle_result_tab: BattleResultTab::Summary,
            battle_result_category: BattleResultCategory::CapitalShips,
            battle_alert_open: false,
            battle_alert_tab: BattleAlertTab::Summary,
            battle_alert_scroll: 0,
            battle_alert_scroll_drag: crate::scroll_bar::ScrollDrag::default(),
            battle_alert_text: None,
            battle_alert_pressed: None,
            strategic_results_applied: false,
            battle_options_open: false,
            battle_options_pressed: None,
            withdraw_confirmation_open: false,
            withdraw_confirmation_pressed: None,
            player_observing: false,
            death_star_laser_pressed: false,
            death_star_targeting: false,
            command_panel: TacticalCommandPanel::Display,
            warmed_font_sizes: HashSet::new(),
            render_original_participants: true,
            render_original_starfield: true,
            render_original_planet: true,
            render_pyrotechnics: true,
            display_holocube: true,
            asset_renderer: TacticalAssetRenderer::default(),
            #[cfg(feature = "interface-test-fixtures")]
            suppress_mapped_capital_fallback: false,
            #[cfg(feature = "interface-test-fixtures")]
            suppress_mapped_fighter_fallback: false,
            #[cfg(feature = "interface-test-fixtures")]
            proof_resource_2560: false,
            #[cfg(feature = "interface-test-fixtures")]
            proof_lod_follows_zoom: false,
        }
    }
}

impl TacticalState {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Begin a new battle — sets session and resets UI state.
    pub fn begin_battle(
        &mut self,
        world: &GameWorld,
        system: SystemKey,
        attacker: FleetKey,
        defender: FleetKey,
        player_is_attacker: bool,
        tick: u64,
        rng_seed: u32,
    ) {
        self.begin_battle_forces(
            world,
            system,
            CombatForce::of_fleet(world, attacker),
            CombatForce::of_fleet(world, defender),
            player_is_attacker,
            tick,
            rng_seed,
        );
    }

    /// Begin a battle between two sides ([`BattleSession::new_with_forces`]).
    #[expect(
        clippy::too_many_arguments,
        reason = "Keep explicit battle inputs at this boundary, as begin_battle does."
    )]
    pub fn begin_battle_forces(
        &mut self,
        world: &GameWorld,
        system: SystemKey,
        attacker: CombatForce,
        defender: CombatForce,
        player_is_attacker: bool,
        tick: u64,
        rng_seed: u32,
    ) {
        let session = BattleSession::new_with_forces(
            world,
            system,
            attacker,
            defender,
            player_is_attacker,
            tick,
            rng_seed,
        );
        let player_is_empire = player_is_attacker != session.attacker_is_alliance;
        self.asset_renderer
            .set_palette_selector(session.system_picture_id);
        self.asset_renderer
            .enable_original_camera(player_is_empire, session.source_layout);
        self.session = Some(session);
        self.dragging_ship = None;
        self.drag_offset = (0.0, 0.0);
        self.camera_x = 0.0;
        self.camera_y = 0.0;
        self.zoom = 1.0;
        self.navigation_sets_visible = [false; 4];
        self.logged_navigation_scene = None;
        self.default_camera =
            self.asset_renderer
                .camera_state()
                .map(|source| TacticalCameraBookmark {
                    source,
                    camera_x: self.camera_x,
                    camera_y: self.camera_y,
                    zoom: self.zoom,
                });
        self.memorized_camera = None;
        self.camera_follow = None;
        self.highlight_alliance = true;
        self.highlight_empire = true;
        self.battle_result_tab = BattleResultTab::Summary;
        self.battle_result_category = BattleResultCategory::CapitalShips;
        self.battle_alert_open = true;
        macroquad::logging::info!("[battle_alert] opened");
        self.battle_alert_tab = BattleAlertTab::Summary;
        self.battle_alert_scroll = 0;
        self.battle_alert_scroll_drag = crate::scroll_bar::ScrollDrag::default();
        self.battle_alert_text = None;
        self.battle_alert_pressed = None;
        self.strategic_results_applied = false;
        self.battle_options_open = false;
        self.battle_options_pressed = None;
        self.withdraw_confirmation_open = false;
        self.withdraw_confirmation_pressed = None;
        self.player_observing = false;
        self.death_star_laser_pressed = false;
        self.death_star_targeting = false;
        self.command_panel = TacticalCommandPanel::Display;
        self.render_original_participants = true;
        self.render_original_starfield = true;
        self.render_original_planet = true;
        self.render_pyrotechnics = true;
        self.display_holocube = true;
        #[cfg(feature = "interface-test-fixtures")]
        {
            self.suppress_mapped_capital_fallback = false;
            self.suppress_mapped_fighter_fallback = false;
            self.proof_resource_2560 = false;
            self.proof_lod_follows_zoom = false;
        }
    }

    /// Apply the five original Tactical Display options. The original Game
    /// Options screen disables these controls while a battle is in progress,
    /// so production callers update them only from outside tactical combat.
    pub fn set_display_options(&mut self, flags: [bool; 5]) {
        self.render_original_starfield = flags[0];
        self.render_original_planet = flags[1];
        self.render_pyrotechnics = flags[2];
        self.asset_renderer.set_high_detail(flags[3]);
        self.display_holocube = flags[4];
    }

    /// Test-only direct state for the source-backed empty-space presentation.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_empty_space_fixture(&mut self) {
        self.render_original_starfield = true;
        self.render_original_planet = false;
    }

    /// Enable the source-bound tactical LOD proof in isolated fixture builds.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn enable_resource_2560_proof(&mut self) {
        self.sync_proof_palette();
        self.proof_resource_2560 = true;
    }

    /// Set a deterministic source-coordinate LOD view for browser acceptance.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn set_tactical_lod_fixture(&mut self, view: TacticalLodView) {
        self.sync_proof_palette();
        self.proof_resource_2560 = true;
        self.proof_lod_follows_zoom = false;
        self.asset_renderer.set_view(view);
    }

    /// Drive all three source LOD slots through the authentic zoom controls.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn enable_tactical_lod_journey(&mut self) {
        self.sync_proof_palette();
        self.proof_resource_2560 = true;
        self.proof_lod_follows_zoom = true;
        self.zoom = 2.0;
        self.asset_renderer
            .set_view(TacticalLodView::from_fixture_zoom(self.zoom));
    }

    /// Enable the source-traced camera for isolated browser acceptance.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn enable_original_camera_proof(&mut self, player_is_empire: bool) {
        self.sync_proof_palette();
        self.proof_resource_2560 = true;
        self.proof_lod_follows_zoom = false;
        let source_layout = self.session.as_ref().map_or_else(
            || OriginalTacticalLayout::from_active_counts(0, 0),
            |session| session.source_layout,
        );
        self.asset_renderer
            .enable_original_camera(player_is_empire, source_layout);
    }

    #[cfg(feature = "interface-test-fixtures")]
    fn sync_proof_palette(&mut self) {
        let selector = self
            .session
            .as_ref()
            .map_or(1, |session| session.system_picture_id);
        self.asset_renderer.set_palette_selector(selector);
    }

    /// Preserve the historical proof and negative-control fixtures without
    /// submitting live participant families alongside the isolated object.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn disable_original_participant_rendering(&mut self) {
        self.render_original_participants = false;
        self.render_original_planet = false;
        self.asset_renderer.disable_original_camera();
    }

    /// Remove mapped participant fallbacks in the browser-only framebuffer control.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn suppress_mapped_capital_fallback(&mut self) {
        self.suppress_mapped_capital_fallback = true;
        self.suppress_mapped_fighter_fallback = true;
        // Keep the common system backdrop present so the paired framebuffer
        // remains a participant-only negative control.
        self.render_original_planet = true;
    }

    /// Center the source camera on the player's first production fighter group
    /// for deterministic close/far browser evidence. This never enters a
    /// production build; normal play must reach the same state through the
    /// original fighter-group controls.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn focus_player_fighter_for_fixture(&mut self) {
        let target = self.session.as_ref().and_then(|session| {
            session
                .fighters
                .iter()
                .enumerate()
                .find(|(_, fighter)| {
                    fighter.alive && fighter.is_attacker == session.player_is_attacker
                })
                .map(|(index, fighter)| {
                    (
                        u32::try_from(index)
                            .unwrap_or(u32::MAX)
                            .saturating_add(1001),
                        fighter.source_position.rendered(),
                    )
                })
        });
        if let Some((object_id, position)) = target {
            self.asset_renderer.focus_target(object_id, position);
        }
    }

    /// Populate every source group slot for deterministic browser coverage.
    /// Production battles retain the executable's proven initial states.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_group_presentation_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let mut task_force = 0_u8;
        for ship in &mut session.ships {
            if ship.alive && ship.is_attacker == session.player_is_attacker {
                ship.task_force = task_force.min(7);
                task_force = task_force.saturating_add(1);
            }
        }
        let mut fighter_group = 0_u8;
        for fighter in &mut session.fighters {
            if fighter.alive
                && fighter.squad_count > 0
                && fighter.is_attacker == session.player_is_attacker
            {
                fighter.fighter_group = fighter_group.min(3);
                fighter_group = fighter_group.saturating_add(1);
            }
        }
    }

    /// Put three live player capitals in one task force so the recovered
    /// previous/next controls can prove their exact cyclic selection contract.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_selected_navigation_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let mut selected = None;
        for (index, ship) in session.ships.iter_mut().enumerate() {
            ship.selected = false;
            if ship.alive && ship.is_attacker == session.player_is_attacker {
                ship.task_force = 0;
                selected.get_or_insert(index);
            }
        }
        for fighter in &mut session.fighters {
            fighter.selected = false;
        }
        if let Some(index) = selected {
            session.ships[index].selected = true;
            session.selected_ship = Some(index);
            session.selected_fighter_group = None;
        }
        session.paused = true;
        self.command_panel = TacticalCommandPanel::Display;
    }

    /// Select one player capital and make the first player fighter group
    /// addressable through the authentic command panels. Production play
    /// reaches the same state through the task-force and fighter controls.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_command_assignment_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let Some(ship_index) = session
            .ships
            .iter()
            .position(|ship| ship.alive && ship.is_attacker == session.player_is_attacker)
        else {
            return;
        };
        for ship in &mut session.ships {
            ship.selected = false;
            if ship.alive && ship.is_attacker == session.player_is_attacker {
                ship.fighter_capacity = 1;
            }
        }
        for fighter in &mut session.fighters {
            fighter.selected = false;
            if fighter.alive && fighter.is_attacker == session.player_is_attacker {
                fighter.fighter_group = 0;
            }
        }
        session.ships[ship_index].selected = true;
        session.selected_ship = Some(ship_index);
        session.selected_fighter_group = None;
        session.paused = true;
        self.command_panel = TacticalCommandPanel::Display;
    }

    /// Keep one player fighter group addressable while preserving the
    /// production Death Star manager object for deterministic browser proof.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_death_star_presentation_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        for fighter in &mut session.fighters {
            fighter.selected = false;
            if fighter.alive && fighter.is_attacker == session.player_is_attacker {
                fighter.fighter_group = 0;
            }
        }
        session.selected_fighter_group = None;
        session.paused = true;
        self.command_panel = TacticalCommandPanel::Display;

        // Exercise the other production branch of `FUN_005d0340` when the
        // local player owns the Imperial station. The ordinary Alliance
        // journey reaches the RGBG branch through the authentic mission
        // control; this fixture-only setup lets the browser prove the exact
        // station-under-attack caller without inventing an Imperial command.
        if session.player_voice_faction() == TacticalVoiceFaction::Empire {
            if let Some(index) = session.fighters.iter().position(|fighter| {
                fighter.alive
                    && fighter.squad_count > 0
                    && fighter.identity.is_alliance
                    && fighter.is_attacker != session.player_is_attacker
            }) {
                session.fighters[index].fighter_group = 0;
                session.fighters[index].order = TacticalOrder::AttackDeathStar;
                begin_original_attack_order(session, &[], &[index], TacticalOrder::AttackDeathStar);
            }
        }
    }

    /// Fully charge the production Death Star manager for the deterministic
    /// superlaser control and right-click target journey.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_death_star_laser_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        if let Some(death_star) = session.death_star.as_mut() {
            death_star.laser_charge = ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE;
            death_star.action_committed = false;
        }
        if player_death_star(session).is_some() {
            if let Some(target) = session
                .ships
                .iter_mut()
                .find(|ship| ship.alive && ship.is_attacker != session.player_is_attacker)
            {
                target.source_position.x = 0.0;
                target.source_position.y = 40.0;
                target.source_position.z = session.source_layout.outer_positive_z;
            }
        }
        session.death_star_beam = None;
        session.paused = true;
        self.death_star_laser_pressed = false;
        self.death_star_targeting = false;
    }

    /// Drive the recovered state-5 producer to a deterministic source result
    /// so the browser can prove both native resolution and app-level 201/202
    /// routing without waiting two wall-clock minutes.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_trench_run_outcome_fixture(&mut self, outcome: TacticalTrenchRunOutcome) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let Some(index) = session.fighters.iter().position(|fighter| {
            fighter.alive
                && fighter.squad_count > 0
                && fighter.identity.is_alliance
                && session
                    .death_star
                    .is_some_and(|death_star| fighter.is_attacker != death_star.is_attacker)
        }) else {
            return;
        };
        session.fighters[index].fighter_group = 0;
        session.fighters[index].order = TacticalOrder::AttackDeathStar;
        session.fighters[index].maneuverability = 9;
        match outcome {
            TacticalTrenchRunOutcome::Success => {
                // Seed 15 yields a maneuver draw of 8 and a success draw of 6.
                session.trench_run_success_rating = 9;
                session.tactical_rng = OriginalTacticalRng::new(15);
            }
            TacticalTrenchRunOutcome::Failure => {
                // Seed 1 yields a maneuver draw of 2 and a success draw of 68.
                session.trench_run_success_rating = 1;
                session.tactical_rng = OriginalTacticalRng::new(1);
            }
        };
        if session.trench_run_remaining_ms.is_none() {
            begin_original_attack_order(session, &[], &[index], TacticalOrder::AttackDeathStar);
        }
        session.advance_original_trench_run(ORIGINAL_TRENCH_RUN_DURATION_MILLISECONDS);
    }

    /// Populate each post-battle condition column and enter the production
    /// Battle Results phase without running an entire deterministic battle.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_battle_results_presentation_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        for alliance in [true, false] {
            let mut ships = session
                .ships
                .iter_mut()
                .filter(|ship| ship.identity.is_alliance == alliance);
            if let Some(ship) = ships.next() {
                ship.alive = true;
                ship.hull_current = ship.hull_max;
                ship.subsystem_damage = TacticalSubsystemDamage::default();
                ship.refresh_subsystem_condition();
            }
            if let Some(ship) = ships.next() {
                ship.alive = true;
                ship.hull_current = (ship.hull_max / 2).max(1);
                ship.subsystem_damage.engines = 1;
                ship.refresh_subsystem_condition();
            }
            if let Some(ship) = ships.next() {
                ship.alive = false;
                ship.hull_current = 0;
                ship.shield = 0;
                ship.refresh_subsystem_condition();
            }

            let mut fighters = session
                .fighters
                .iter_mut()
                .filter(|fighter| fighter.identity.is_alliance == alliance);
            if let Some(fighter) = fighters.next() {
                fighter.alive = true;
                fighter.hull_current = fighter.hull_max;
                fighter.squad_count = 1;
            }
            if let Some(fighter) = fighters.next() {
                fighter.alive = true;
                fighter.hull_current = (fighter.hull_max * 0.5).max(1.0);
                fighter.squad_count = 1;
            }
            if let Some(fighter) = fighters.next() {
                fighter.alive = false;
                fighter.hull_current = 0.0;
                fighter.squad_count = 0;
            }
        }
        session.phase = BattlePhase::Results;
        session.winner = Some(if session.player_is_attacker {
            CombatWinner::Attacker
        } else {
            CombatWinner::Defender
        });
        session.paused = true;
        session.selected_ship = None;
        session.selected_fighter_group = None;
        self.battle_result_tab = BattleResultTab::Summary;
        self.battle_result_category = BattleResultCategory::CapitalShips;
        self.command_panel = TacticalCommandPanel::Display;
    }

    /// Keep one player ship withdrawal-capable while disabling another one's
    /// hyperdrive so the production confirmation route emits the source
    /// faction warning and leaves only the operational ship eligible.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_withdrawal_warning_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let mut player_ships = session
            .ships
            .iter_mut()
            .filter(|ship| ship.alive && ship.is_attacker == session.player_is_attacker);
        let Some(operational) = player_ships.next() else {
            return;
        };
        operational.subsystem_damage.hyperdrive = 0;
        operational.refresh_subsystem_condition();
        let Some(stranded) = player_ships.next() else {
            return;
        };
        stranded.subsystem_damage.hyperdrive = stranded.subsystem_capacity.hyperdrive;
        stranded.refresh_subsystem_condition();
    }

    /// Commit one recovered capital maneuver and one fighter recovery order
    /// through the production assignment and executor path.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_command_execution_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let Some(ship_index) = session.ships.iter().position(|ship| {
            ship.alive
                && ship.is_attacker == session.player_is_attacker
                && ship.fighter_capacity > 0
        }) else {
            return;
        };
        for ship in &mut session.ships {
            ship.selected = false;
        }
        for fighter in &mut session.fighters {
            fighter.selected = false;
            if fighter.alive && fighter.is_attacker == session.player_is_attacker {
                fighter.fighter_group = 0;
            }
        }
        let forward_z = if session.ships[ship_index].identity.is_alliance {
            1.0
        } else {
            -1.0
        };
        session.ships[ship_index].source_forward = TacticalWorldVector {
            x: 0.0,
            y: 0.0,
            z: forward_z,
        };
        session.ships[ship_index].selected = true;
        session.selected_ship = Some(ship_index);
        session.selected_fighter_group = None;
        assign_selected_command(
            session,
            TacticalOrder::LeftHook,
            Some(TacticalTactic::Surround),
        );

        if let Some(hold_index) = session.ships.iter().enumerate().find_map(|(index, ship)| {
            (index != ship_index && ship.alive && ship.is_attacker == session.player_is_attacker)
                .then_some(index)
        }) {
            session.ships[ship_index].selected = false;
            let hold_ship = &mut session.ships[hold_index];
            hold_ship.selected = true;
            hold_ship.source_desired_forward = hold_ship.source_forward;
            hold_ship.source_velocity = hold_ship.source_forward;
            hold_ship.source_waypoint = Some(hold_ship.source_position);
            session.selected_ship = Some(hold_index);
            assign_selected_command(session, TacticalOrder::HoldPosition, None);
        }

        for ship in &mut session.ships {
            ship.selected = false;
        }
        for fighter in &mut session.fighters {
            fighter.selected = fighter.alive
                && fighter.is_attacker == session.player_is_attacker
                && fighter.fighter_group == 0;
        }
        session.selected_ship = None;
        session.selected_fighter_group = Some(0);
        assign_selected_command(session, TacticalOrder::Recover, None);

        for fighter in &mut session.fighters {
            fighter.selected = false;
        }
        session.ships[ship_index].selected = true;
        session.selected_ship = Some(ship_index);
        session.selected_fighter_group = None;
        session.paused = true;
        self.command_panel = TacticalCommandPanel::Display;
    }

    /// Advance one turning capital, one arriving capital, and both final
    /// fighter-recovery transitions through the production executors.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_command_progression_fixture(&mut self) {
        let focus = {
            let Some(session) = self.session.as_mut() else {
                return;
            };
            let player_ships = session
                .ships
                .iter()
                .enumerate()
                .filter(|(_, ship)| {
                    ship.alive
                        && ship.is_attacker == session.player_is_attacker
                        && ship.subsystem_capacity.engines
                })
                .map(|(index, _)| index)
                .take(2)
                .collect::<Vec<_>>();
            let player_fighters = session
                .fighters
                .iter()
                .enumerate()
                .filter(|(_, fighter)| {
                    fighter.alive && fighter.is_attacker == session.player_is_attacker
                })
                .map(|(index, _)| index)
                .take(2)
                .collect::<Vec<_>>();
            if player_ships.len() < 2 || player_fighters.len() < 2 {
                return;
            }
            let turning = player_ships[0];
            let arriving = player_ships[1];
            let forward_z = if session.ships[turning].identity.is_alliance {
                1.0
            } else {
                -1.0
            };
            for ship in &mut session.ships {
                ship.selected = false;
                ship.source_velocity = TacticalWorldVector::ZERO;
            }
            for fighter in &mut session.fighters {
                fighter.selected = false;
            }

            let turning_position = session.ships[turning].source_position;
            session.ships[turning].selected = true;
            session.ships[turning].maneuverability = 4;
            session.ships[turning].sublight_engine_power = 20.0;
            session.ships[turning].source_forward = TacticalWorldVector {
                x: 0.0,
                y: 0.0,
                z: forward_z,
            };
            session.ships[turning].source_waypoint = Some(TacticalWorldPosition {
                x: turning_position.x + 50.0,
                y: turning_position.y,
                z: turning_position.z + 50.0 * forward_z,
            });
            session.ships[turning].order = TacticalOrder::LeftHook;

            let arriving_position = session.ships[arriving].source_position;
            session.ships[arriving].sublight_engine_power = 100.0;
            session.ships[arriving].source_forward = TacticalWorldVector {
                x: 0.0,
                y: 0.0,
                z: forward_z,
            };
            session.ships[arriving].source_desired_forward = session.ships[arriving].source_forward;
            session.ships[arriving].source_waypoint = Some(TacticalWorldPosition {
                x: arriving_position.x,
                y: arriving_position.y,
                z: arriving_position.z + forward_z,
            });
            session.ships[arriving].order = TacticalOrder::Anvil;

            session.advance_original_tactical_movement(ORIGINAL_TACTICAL_STEP_MILLISECONDS);

            let carrier_position = session.ships[arriving].source_position;
            let returning = player_fighters[0];
            session.fighters[returning].fighter_group = 0;
            session.fighters[returning].source_position = TacticalWorldPosition {
                x: carrier_position.x,
                y: carrier_position.y,
                z: carrier_position.z + 2.5 * forward_z,
            };
            session.fighters[returning].sublight_engine_power = 4.0;
            session.fighters[returning].order = TacticalOrder::Recover;
            session.fighters[returning].recovery_state = TacticalFighterRecoveryState::Returning;
            session.fighters[returning].recovery_target = Some(arriving);

            let docking = player_fighters[1];
            session.fighters[docking].fighter_group = 1;
            session.fighters[docking].source_position = carrier_position;
            session.fighters[docking].order = TacticalOrder::Recover;
            session.fighters[docking].recovery_state = TacticalFighterRecoveryState::Docking;
            session.fighters[docking].recovery_target = Some(arriving);
            session.advance_original_fighter_recovery(ORIGINAL_TACTICAL_STEP_MILLISECONDS);

            session.selected_ship = Some(turning);
            session.selected_fighter_group = None;
            session.paused = true;
            self.command_panel = TacticalCommandPanel::Display;
            let object_id = u32::try_from(turning).unwrap_or(u32::MAX).saturating_add(1);
            (object_id, session.ships[turning].source_position.rendered())
        };
        self.asset_renderer.focus_target(focus.0, focus.1);
    }

    /// Exercise both recovered automatic attack-target executors with capital
    /// and fighter owners. Production play reaches these states by committing
    /// the authentic Missions-panel controls.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_attack_targeting_fixture(&mut self) {
        let focus = {
            let Some(session) = self.session.as_mut() else {
                return;
            };
            let player_ships = session
                .ships
                .iter()
                .enumerate()
                .filter(|(_, ship)| ship.alive && ship.is_attacker == session.player_is_attacker)
                .map(|(index, _)| index)
                .take(2)
                .collect::<Vec<_>>();
            let player_fighters = session
                .fighters
                .iter()
                .enumerate()
                .filter(|(_, fighter)| {
                    fighter.alive && fighter.is_attacker == session.player_is_attacker
                })
                .map(|(index, _)| index)
                .take(2)
                .collect::<Vec<_>>();
            if player_ships.len() < 2 || player_fighters.len() < 2 {
                return;
            }
            for ship in &mut session.ships {
                ship.selected = false;
                ship.attack_target = None;
            }
            for fighter in &mut session.fighters {
                fighter.selected = false;
                fighter.attack_target = None;
            }
            session.fighters[player_fighters[0]].fighter_group = 0;
            session.fighters[player_fighters[1]].fighter_group = 1;

            session.ships[player_ships[0]].selected = true;
            session.selected_ship = Some(player_ships[0]);
            session.selected_fighter_group = None;
            assign_selected_command(session, TacticalOrder::AttackCapitalShips, None);

            session.ships[player_ships[0]].selected = false;
            session.ships[player_ships[1]].selected = true;
            session.selected_ship = Some(player_ships[1]);
            assign_selected_command(session, TacticalOrder::AttackFighters, None);

            session.ships[player_ships[1]].selected = false;
            session.fighters[player_fighters[0]].selected = true;
            session.selected_ship = None;
            session.selected_fighter_group = Some(0);
            assign_selected_command(session, TacticalOrder::AttackCapitalShips, None);

            session.fighters[player_fighters[0]].selected = false;
            session.fighters[player_fighters[1]].selected = true;
            session.selected_fighter_group = Some(1);
            assign_selected_command(session, TacticalOrder::AttackFighters, None);

            session.fighters[player_fighters[1]].selected = false;
            session.ships[player_ships[1]].selected = true;
            session.selected_ship = Some(player_ships[1]);
            session.selected_fighter_group = None;
            session.paused = true;
            self.command_panel = TacticalCommandPanel::Display;

            let selected = player_ships[1];
            let object_id = u32::try_from(selected)
                .unwrap_or(u32::MAX)
                .saturating_add(1);
            (
                object_id,
                session.ships[selected].source_position.rendered(),
            )
        };
        self.asset_renderer.focus_target(focus.0, focus.1);
    }

    /// Invalidate each first-acquired attack target and exercise the recovered
    /// same-class replacement lifecycle. The retired objects remain in the
    /// fixture record so the browser harness can prove why they were skipped.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_attack_target_lifecycle_fixture(&mut self) {
        self.configure_attack_targeting_fixture();
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let first_enemy_ship = session
            .ships
            .iter()
            .position(|ship| ship.alive && ship.is_attacker != session.player_is_attacker);
        let first_enemy_fighter = session
            .fighters
            .iter()
            .position(|fighter| fighter.alive && fighter.is_attacker != session.player_is_attacker);
        let (Some(first_enemy_ship), Some(first_enemy_fighter)) =
            (first_enemy_ship, first_enemy_fighter)
        else {
            return;
        };

        session.ships[first_enemy_ship].retreating = true;
        session.fighters[first_enemy_fighter].recovery_state =
            TacticalFighterRecoveryState::Recovered;
        refresh_original_attack_targets(session);
    }

    /// Display one stable frame from every source-selected impact family.
    /// This bridge is compiled only into interface-fixture builds.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_effect_presentation_fixture(&mut self) {
        let focus = {
            let Some(session) = self.session.as_mut() else {
                return;
            };
            let targets = session
                .ships
                .iter()
                .enumerate()
                .filter(|(_, ship)| ship.is_attacker == session.player_is_attacker)
                .map(|(index, _)| index)
                .take(6)
                .collect::<Vec<_>>();
            let fixtures = [
                (WeaponKind::LaserCannon, OriginalTacticalImpactStage::Hit, 2),
                (WeaponKind::IonCannon, OriginalTacticalImpactStage::Hit, 2),
                (
                    WeaponKind::LaserCannon,
                    OriginalTacticalImpactStage::Damage,
                    3,
                ),
                (
                    WeaponKind::IonCannon,
                    OriginalTacticalImpactStage::Damage,
                    7,
                ),
                (
                    WeaponKind::Turbolaser,
                    OriginalTacticalImpactStage::Damage,
                    2,
                ),
                (
                    WeaponKind::LaserCannon,
                    OriginalTacticalImpactStage::Destroyed,
                    8,
                ),
            ];
            session.impact_effects.clear();
            for (target, (weapon, stage, frame)) in targets.iter().copied().zip(fixtures) {
                queue_original_tactical_impact(&mut session.impact_effects, target, weapon, stage);
                if let Some(effect) = session
                    .impact_effects
                    .iter_mut()
                    .find(|effect| effect.target == target)
                {
                    effect.frame = frame.min(effect.sequence.frame_count().saturating_sub(1));
                }
            }
            session.paused = true;
            targets.get(2).and_then(|&target| {
                session.ships.get(target).map(|ship| {
                    (
                        u32::try_from(target).unwrap_or(u32::MAX).saturating_add(1),
                        ship.source_position.rendered(),
                    )
                })
            })
        };
        if let Some((object_id, position)) = focus {
            self.asset_renderer.focus_target(object_id, position);
        }
    }

    /// Present the remaining source-backed tactical detail states together:
    /// one destroyed hostile hull, the selected command hull's personnel
    /// contents, and a second friendly hull that can receive direct Escort.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_detail_escort_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let player_ships = session
            .ships
            .iter()
            .enumerate()
            .filter(|(_, ship)| ship.alive && ship.is_attacker == session.player_is_attacker)
            .map(|(index, _)| index)
            .take(2)
            .collect::<Vec<_>>();
        let destroyed = session
            .ships
            .iter()
            .enumerate()
            .find(|(_, ship)| ship.alive && ship.is_attacker != session.player_is_attacker)
            .map(|(index, _)| index);
        let ([selected, _escort_target], Some(destroyed)) = (player_ships.as_slice(), destroyed)
        else {
            return;
        };

        for ship in &mut session.ships {
            ship.selected = false;
        }
        for fighter in &mut session.fighters {
            fighter.selected = false;
        }
        session.ships[*selected].selected = true;
        session.selected_ship = Some(*selected);
        session.selected_fighter_group = None;

        // Center the retired hull solely for a fully inspectable source frame
        // in both faction cameras. Production destruction keeps the live
        // object's final position.
        session.ships[destroyed].source_position.x = 0.0;
        session.ships[destroyed].source_position.y = 0.0;
        session.ships[destroyed].source_position.z = 0.0;
        session.ships[destroyed].hull_current = 0;
        session.ships[destroyed].shield = 0;
        session.ships[destroyed].alive = false;
        session.ships[destroyed].selected = false;
        session.impact_effects.clear();
        queue_original_tactical_impact(
            &mut session.impact_effects,
            destroyed,
            WeaponKind::LaserCannon,
            OriginalTacticalImpactStage::Destroyed,
        );
        if let Some(effect) = session
            .impact_effects
            .iter_mut()
            .find(|effect| effect.target == destroyed)
        {
            effect.frame = 8;
        }
        session.paused = true;
        self.command_panel = TacticalCommandPanel::Display;
    }

    /// Queue every recovered weapon-event variant so the muted browser journey
    /// can prove the complete TACTICAL.DLL 13033-13054 runtime bank.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_tactical_audio_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        session.pending_audio_cues.clear();
        for event in [
            TacticalAudioEvent::LaserFire,
            TacticalAudioEvent::TurbolaserFire,
            TacticalAudioEvent::IonFire,
            TacticalAudioEvent::TorpedoFire,
            TacticalAudioEvent::LaserImpact,
            TacticalAudioEvent::IonImpact,
            TacticalAudioEvent::TurbolaserImpact,
            TacticalAudioEvent::TorpedoImpact,
        ] {
            for variant in 0..event.variant_count() {
                session
                    .pending_audio_cues
                    .push(TacticalAudioCue::from_variant(event, variant));
            }
        }
        session.pending_voice_cues.clear();
        for source_event in (0x20..=0x99).chain(0x11e..=0x13c).chain(0x9a..=0x11d) {
            session
                .pending_voice_cues
                .push(TacticalVoiceCue::fixture_source_event(source_event));
        }
    }

    /// Freeze all three projectile shapes plus both shared field families on
    /// live production participants for deterministic browser inspection.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_projectile_field_fixture(&mut self) {
        {
            let Some(session) = self.session.as_mut() else {
                return;
            };
            let alliance = session
                .ships
                .iter()
                .enumerate()
                .filter(|(_, ship)| ship.identity.is_alliance)
                .map(|(index, _)| index)
                .collect::<Vec<_>>();
            let empire = session
                .ships
                .iter()
                .enumerate()
                .filter(|(_, ship)| !ship.identity.is_alliance)
                .map(|(index, _)| index)
                .collect::<Vec<_>>();
            if alliance.len() < 3 || empire.len() < 3 {
                return;
            }
            session.weapon_effects.clear();
            let projectile_fixtures = [
                (
                    alliance[0],
                    empire[0],
                    WeaponKind::LaserCannon,
                    ORIGINAL_LASER_PROJECTILE_THRESHOLD,
                ),
                (
                    empire[1],
                    alliance[1],
                    WeaponKind::Turbolaser,
                    ORIGINAL_TURBOLASER_PROJECTILE_THRESHOLD - 1.0,
                ),
                (
                    alliance[2],
                    empire[2],
                    WeaponKind::IonCannon,
                    ORIGINAL_ION_PROJECTILE_THRESHOLD,
                ),
            ];
            for (source, target, weapon, strength) in projectile_fixtures {
                queue_original_tactical_projectile(
                    &mut session.weapon_effects,
                    &session.ships,
                    source,
                    target,
                    weapon,
                    strength,
                );
            }
            for effect in &mut session.weapon_effects {
                effect.elapsed = 0.5;
            }

            session.field_effects.clear();
            session.set_tractor_field(empire[0], alliance[1], true);
            session.set_tractor_field(alliance[0], empire[1], true);
            session.set_gravity_field(alliance[2], empire[1], true);
            session.field_effects[0].frame = 3;
            session.field_effects[1].frame = 5;
            session.paused = true;
        }
        // Widen the source camera field without moving any participant. This
        // keeps both opposing field targets and the projectile paths inside
        // the 444x439 aperture for one deterministic inspection frame.
        for _ in 0..4 {
            self.asset_renderer.zoom_out();
        }
    }

    /// Select the player's first ship, undamaged. Battles open with nothing
    /// selected (`FUN_005c5800`); the participant and camera gates read the
    /// selected panel and the Center control from this staged selection.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn select_first_player_ship_for_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let Some(index) = session
            .ships
            .iter()
            .position(|ship| ship.is_attacker == session.player_is_attacker)
        else {
            return;
        };
        session.ships[index].selected = true;
        session.selected_ship = Some(index);
    }

    /// Select one production capital ship and install stable live damage for
    /// the selected-HUD browser gate. This bridge never enters production.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_selected_damage_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let Some(index) = session
            .ships
            .iter()
            .position(|ship| ship.alive && ship.is_attacker == session.player_is_attacker)
        else {
            return;
        };
        for ship in &mut session.ships {
            ship.selected = false;
        }
        let ship = &mut session.ships[index];
        ship.selected = true;
        ship.hull_current = (ship.hull_max * 2 / 5).max(1);
        ship.shield = (ship.shield_max / 4).max(0);
        session.selected_ship = Some(index);
        session.selected_fighter_group = None;
        session.paused = true;
    }

    /// Install all five selected-capital condition bands plus source-identified
    /// tractor and gravity locks for deterministic browser inspection.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_subsystem_field_command_fixture(&mut self) {
        self.configure_selected_damage_fixture();
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let alliance = session
            .ships
            .iter()
            .enumerate()
            .filter(|(_, ship)| ship.identity.is_alliance)
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        let empire = session
            .ships
            .iter()
            .enumerate()
            .filter(|(_, ship)| !ship.identity.is_alliance)
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        if alliance.len() < 3 || empire.len() < 3 {
            return;
        }
        session.field_effects.clear();
        assert!(session.set_tractor_field(empire[0], alliance[1], true));
        assert!(session.set_tractor_field(empire[2], alliance[1], true));
        assert!(session.set_tractor_field(alliance[0], empire[1], true));
        assert!(session.set_gravity_field(alliance[2], empire[1], true));
        // P58F5 is a resource-band presentation fixture. Field changes now
        // refresh production mobility, so install its deliberately chosen
        // five-band display state after those production mutations complete.
        if let Some(index) = session.selected_ship {
            session.ships[index].subsystem_condition = TacticalSubsystemCondition {
                shields: 0,
                weapons: 24,
                tractor: 25,
                engines: 50,
                hyperdrive: 75,
            };
        }
        session.advance_presentational_effects(0.4);
        session.paused = true;
        for _ in 0..4 {
            self.asset_renderer.zoom_out();
        }
    }

    /// Drive one selected production capital through the source combat-damage
    /// path with deterministic rolls for browser inspection. Direct setup is
    /// confined to the test bridge; every recorded hit uses production logic.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_live_subsystem_damage_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let Some(index) = session.ships.iter().position(|ship| {
            ship.alive
                && ship.is_attacker == session.player_is_attacker
                && ship
                    .subsystem_capacity
                    .hit_limits()
                    .iter()
                    .all(|limit| *limit > 0)
        }) else {
            return;
        };
        for ship in &mut session.ships {
            ship.selected = false;
        }
        let ship = &mut session.ships[index];
        ship.selected = true;
        ship.hull_current = ship.hull_max;
        ship.shield = ship.shield_max;
        ship.subsystem_damage = TacticalSubsystemDamage::default();
        ship.refresh_subsystem_condition();
        session.selected_ship = Some(index);
        session.selected_fighter_group = None;

        let shield_damage = (session.ships[index].shield_max / 5).max(1);
        apply_original_capital_damage(
            &mut session.ships,
            &mut session.field_effects,
            index,
            shield_damage,
            OriginalTacticalDamageRolls { shield: 0, hull: 0 },
        );
        session.ships[index].shield = 0;
        for (roll, hits) in [(66, 2), (81, 3), (91, 4), (96, 2)] {
            for _ in 0..hits {
                apply_original_capital_damage(
                    &mut session.ships,
                    &mut session.field_effects,
                    index,
                    1,
                    OriginalTacticalDamageRolls {
                        shield: 100,
                        hull: roll,
                    },
                );
            }
        }
        session.paused = true;
    }

    /// Present one source-selected repair while an enemy tractor source lowers
    /// the selected ship's effective sublight condition. Direct setup remains
    /// test-only; repair selection and mobility use production functions.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_subsystem_repair_mobility_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let Some(target) = session.ships.iter().position(|ship| {
            ship.alive
                && ship.is_attacker == session.player_is_attacker
                && ship
                    .subsystem_capacity
                    .hit_limits()
                    .iter()
                    .all(|limit| *limit > 0)
        }) else {
            return;
        };
        let Some(source) = session.ships.iter().enumerate().find_map(|(index, ship)| {
            (index != target && ship.alive && ship.is_attacker != session.player_is_attacker)
                .then_some(index)
        }) else {
            return;
        };

        for ship in &mut session.ships {
            ship.selected = false;
        }
        session.ships[target].selected = true;
        session.ships[target].hull_current = session.ships[target].hull_max;
        session.ships[target].sublight_engine_power = 100.0;
        session.ships[target].maneuver_state_value = Some(9.0);
        session.ships[target].damage_control = 100;
        session.ships[target].subsystem_damage = TacticalSubsystemDamage {
            shields: 1,
            weapons: 2,
            tractor: 1,
            engines: 2,
            hyperdrive: 1,
        };
        session.ships[source].hull_current = session.ships[source].hull_max;
        session.ships[source].subsystem_capacity.tractor = true;
        session.ships[source].tractor_beam_power = 25.0;
        session.ships[source].subsystem_damage.tractor = 0;
        session.field_effects.clear();
        session.subsystem_repairs.clear();
        session.selected_ship = Some(target);
        session.selected_fighter_group = None;
        let _ = session.set_tractor_field(source, target, true);

        if let Some(repair) =
            attempt_original_subsystem_repair(&mut session.ships[target], target, 1, 1)
        {
            session.subsystem_repairs.push(repair);
        }
        refresh_original_subsystem_conditions(&mut session.ships, &session.field_effects);
        let target_object_id = u32::try_from(target).unwrap_or(u32::MAX).saturating_add(1);
        let target_position = session.ships[target].source_position.rendered();
        let (camera_x, camera_y) =
            camera_offset_for_target(session.ships[target].x, session.ships[target].y);
        session.paused = true;
        (self.camera_x, self.camera_y) = (camera_x, camera_y);
        self.asset_renderer
            .focus_target(target_object_id, target_position);
    }

    /// Advance one selected production capital through the recovered
    /// maneuver producer, velocity contract, and 250 ms source integrator.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn configure_maneuver_movement_fixture(&mut self) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        let Some(target) = session.ships.iter().position(|ship| {
            ship.alive
                && ship.is_attacker == session.player_is_attacker
                && ship.subsystem_capacity.engines
        }) else {
            return;
        };
        for ship in &mut session.ships {
            ship.selected = false;
            ship.source_velocity = TacticalWorldVector::ZERO;
        }
        session.ships[target].selected = true;
        session.ships[target].hull_current = session.ships[target].hull_max;
        session.ships[target].sublight_engine_power = 100.0;
        session.ships[target].subsystem_damage.engines = 1;
        session.ships[target].source_forward = if session.ships[target].identity.is_alliance {
            TacticalWorldVector::SOURCE_FORWARD
        } else {
            TacticalWorldVector {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            }
        };
        session.ships[target].source_desired_forward = session.ships[target].source_forward;
        session.field_effects.clear();
        session.selected_ship = Some(target);
        session.selected_fighter_group = None;
        assert!(session.set_maneuver_state_value(target, Some(4.0)));
        session.advance_original_tactical_movement(ORIGINAL_TACTICAL_STEP_MILLISECONDS);
        refresh_original_subsystem_conditions(&mut session.ships, &session.field_effects);

        let target_object_id = u32::try_from(target).unwrap_or(u32::MAX).saturating_add(1);
        let target_position = session.ships[target].source_position.rendered();
        let (camera_x, camera_y) =
            camera_offset_for_target(session.ships[target].x, session.ships[target].y);
        session.paused = true;
        (self.camera_x, self.camera_y) = (camera_x, camera_y);
        self.asset_renderer
            .focus_target(target_object_id, target_position);
    }

    /// End the current battle — clears session. Returns the session for
    /// result processing by the caller.
    pub fn end_battle(&mut self) -> Option<BattleSession> {
        self.dragging_ship = None;
        self.command_panel = TacticalCommandPanel::Display;
        self.session.take()
    }

    /// Consume the one-shot trench-run presentation edge, if the source
    /// tactical result dispatcher resolved one during the latest combat step.
    pub fn take_pending_trench_run_cinematic(&mut self) -> Option<TacticalTrenchRunOutcome> {
        self.session
            .as_mut()
            .and_then(BattleSession::take_pending_trench_run_cinematic)
    }

    /// Drain source-mapped tactical WAVE cues for the application audio
    /// backend. Muted test runs still retain the event/resource diagnostics.
    pub fn take_pending_audio_cues(&mut self) -> Vec<TacticalAudioCue> {
        self.session
            .as_mut()
            .map_or_else(Vec::new, BattleSession::take_pending_audio_cues)
    }

    /// Drain exact VOICEFXA/VOICEFXE tactical acknowledgements for the
    /// application audio backend.
    pub fn take_pending_voice_cues(&mut self) -> Vec<TacticalVoiceCue> {
        self.session
            .as_mut()
            .map_or_else(Vec::new, BattleSession::take_pending_voice_cues)
    }

    /// The Battle Alert's Simulate: run the live session to its end without
    /// its window (`BattleSession::simulate_to_end`), then open the same
    /// Battle Results a played battle shows. The caller applies the session
    /// to the strategic world at once.
    pub fn simulate_battle(&mut self) -> bool {
        let Some(session) = self.session.as_mut() else {
            return false;
        };
        session.simulate_to_end();
        flush_engine_log(session);
        self.battle_result_tab = BattleResultTab::Summary;
        self.battle_result_category = BattleResultCategory::CapitalShips;
        self.strategic_results_applied = true;
        true
    }

    /// True when auto-resolve has already mutated the strategic world.
    #[must_use]
    pub const fn strategic_results_applied(&self) -> bool {
        self.strategic_results_applied
    }

    /// Returns true if a battle is currently active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.session.is_some()
    }

    /// Set the Battle Alert's title and summary paragraph for this battle.
    pub fn set_battle_alert_text(&mut self, title: String, summary: String) {
        self.battle_alert_text = Some((title, summary));
    }

    /// Whether the pre-battle source surface still owns tactical entry.
    #[must_use]
    pub const fn battle_alert_open(&self) -> bool {
        self.battle_alert_open
    }

    /// Whether the battle shows as a window over the command center: the
    /// Battle Alert before it, the results after it (the 470x331 window on
    /// the strategic canvas). The app draws the live command center beneath.
    #[must_use]
    pub fn over_command_center(&self) -> bool {
        self.battle_alert_open
            || self
                .session
                .as_ref()
                .is_some_and(|session| session.phase == BattlePhase::Results)
    }

    /// Existing fixture scenarios predate Battle Alert and intentionally
    /// enter the same production session after its Take Command transition.
    #[cfg(feature = "interface-test-fixtures")]
    pub fn dismiss_battle_alert_for_fixture(&mut self) {
        self.battle_alert_open = false;
        self.battle_alert_pressed = None;
    }
}

/// Default ship icon size when no sprite is available.
const DEFAULT_SHIP_SIZE: f32 = 40.0;

/// Fighter squadron icon size.
const FIGHTER_SIZE: f32 = 16.0;

/// Source positions in the renderer's handedness.
trait RenderedPosition {
    fn rendered(self) -> Vec3;
}

impl RenderedPosition for TacticalWorldPosition {
    fn rendered(self) -> Vec3 {
        // Direct3D retained mode is left-handed and the browser renderer is
        // right-handed. Preserve Y and reflect source Z.
        vec3(self.x, self.y, -self.z)
    }
}

/// HUD art for an order.
trait TacticalOrderArt {
    fn maneuver_graphic(self) -> Option<u32>;
    fn mission_graphic(self, player_is_alliance: bool) -> Option<u32>;
}

impl TacticalOrderArt for TacticalOrder {
    fn maneuver_graphic(self) -> Option<u32> {
        use resources::tactical as art;
        match self {
            Self::LeftHook => Some(art::MANEUVER_HUD_LEFT_HOOK),
            Self::RightHook => Some(art::MANEUVER_HUD_RIGHT_HOOK),
            Self::Hammer => Some(art::MANEUVER_HUD_HAMMER),
            Self::Anvil => Some(art::MANEUVER_HUD_ANVIL),
            Self::HoldPosition => Some(art::MANEUVER_HUD_HOLD_POSITION),
            _ => None,
        }
    }

    fn mission_graphic(self, player_is_alliance: bool) -> Option<u32> {
        use resources::tactical as art;
        match (self, player_is_alliance) {
            (Self::AttackCapitalShips, true) => {
                Some(art::MISSIONS_HUD_ATTACK_CAPITAL_SHIPS_ALLIANCE)
            }
            (Self::AttackFighters, true) => Some(art::MISSIONS_HUD_ATTACK_FIGHTERS_ALLIANCE),
            (Self::Recover, true) => Some(art::MISSIONS_HUD_RECOVER_ALLIANCE),
            (Self::AttackDeathStar, true) => Some(art::MISSIONS_HUD_ATTACK_DEATH_STAR_ALLIANCE),
            (Self::AttackCapitalShips, false) => {
                Some(art::MISSIONS_HUD_ATTACK_CAPITAL_SHIPS_EMPIRE)
            }
            (Self::AttackFighters, false) => Some(art::MISSIONS_HUD_ATTACK_FIGHTERS_EMPIRE),
            (Self::Recover, false) => Some(art::MISSIONS_HUD_RECOVER_EMPIRE),
            _ => None,
        }
    }
}

/// Log the engine's queued diagnostic lines (`BattleSession::pending_log_lines`).
fn flush_engine_log(session: &mut BattleSession) {
    for line in session.pending_log_lines.drain(..) {
        macroquad::logging::info!("{}", line);
    }
}

/// A weapon effect's material colour.
trait WeaponEffectColor {
    fn color(&self) -> Color;
}

impl WeaponEffectColor for WeaponEffect {
    fn color(&self) -> Color {
        projectile_selector_color(self.color_selector)
    }
}

/// The flat Direct3DRM material each projectile selector names.
fn projectile_selector_color(selector: u8) -> Color {
    match selector {
        1 => Color::new(0.0, 1.0, 0.0, 1.0),
        2 => Color::new(0.0, 0.0, 1.0, 1.0),
        _ => Color::new(1.0, 0.0, 0.0, 1.0),
    }
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

/// Result of drawing the tactical view — tells main.rs what action to take.
#[derive(Debug, Clone, PartialEq)]
pub enum TacticalAction {
    /// Player is still in the tactical view — no transition needed.
    None,
    /// Player clicked "Begin Battle" — advance from placement to combat phase.
    BeginCombat,
    /// Player clicked "Auto-Resolve" — skip tactical view, auto-resolve.
    AutoResolve,
    /// Battle is over or player clicked "Return to Galaxy" — transition back.
    ReturnToGalaxy,
    /// Leave the Battle Results window and open the battle system.
    OpenBattleSystem(SystemKey),
    /// Leave the Battle Results window and open the surviving victorious fleet.
    OpenBattleFleet(FleetKey),
    /// Toggle combat pause.
    TogglePause,
    /// Set combat speed multiplier.
    SetSpeed(u32),
    /// Retreat selected ships.
    RetreatSelected,
    /// Withdraw every eligible player hull after the Battle Options command.
    WithdrawFromBattle,
    /// Attempt an immediate retreat from the pre-battle Battle Alert.
    RetreatBeforeBattle,
    /// Route to the original unified Game Options surface.
    OpenGameOptions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TacticalHudControl {
    TaskForce(u8),
    FighterGroup(u8),
    NavigationSet(u8),
    Pause,
    ZoomIn,
    ZoomOut,
    CameraTarget,
    CameraLeft,
    CameraRight,
    CameraUp,
    CameraDown,
    CameraRecall,
    CameraMemorize,
    HighlightAlliance,
    HighlightEmpire,
    BattleOptions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TacticalBattleOptionsControl {
    Withdraw,
    SimulateRemainder,
    ObserveToggle,
    GameOptions,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TacticalWithdrawConfirmationControl {
    Confirm,
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TacticalCommandControl {
    PreviousCapital,
    NextCapital,
    OpenManeuvers,
    OpenMissions,
    LeftHook,
    RightHook,
    Hammer,
    Anvil,
    HoldPosition,
    ToggleTactic,
    AttackCapitalShips,
    AttackFighters,
    Recover,
    AttackDeathStar,
    Confirm,
    Cancel,
}

#[derive(Debug, Clone, Copy)]
struct TacticalHudControlSpec {
    control: TacticalHudControl,
    rect: NativeRect,
    hit_resource: u32,
}

const TACTICAL_HUD_CONTROLS: [TacticalHudControlSpec; 17] = [
    TacticalHudControlSpec {
        control: TacticalHudControl::NavigationSet(0),
        rect: NativeRect::new(485.0, 272.0, 27.0, 27.0),
        hit_resource: resources::tactical::BTN_NAVIGATION_SET_NORMAL[0],
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::NavigationSet(1),
        rect: NativeRect::new(521.0, 272.0, 27.0, 27.0),
        hit_resource: resources::tactical::BTN_NAVIGATION_SET_NORMAL[1],
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::NavigationSet(2),
        rect: NativeRect::new(560.0, 272.0, 27.0, 27.0),
        hit_resource: resources::tactical::BTN_NAVIGATION_SET_NORMAL[2],
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::NavigationSet(3),
        rect: NativeRect::new(601.0, 272.0, 27.0, 27.0),
        hit_resource: resources::tactical::BTN_NAVIGATION_SET_NORMAL[3],
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::Pause,
        rect: NativeRect::new(560.0, 307.0, 28.0, 21.0),
        hit_resource: resources::tactical::BTN_PAUSE_RUNNING,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::ZoomIn,
        rect: NativeRect::new(486.0, 343.0, 24.0, 24.0),
        hit_resource: resources::tactical::BTN_CAMERA_ZOOM_IN_NORMAL,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::ZoomOut,
        rect: NativeRect::new(603.0, 343.0, 24.0, 24.0),
        hit_resource: resources::tactical::BTN_CAMERA_ZOOM_OUT_NORMAL,
    },
    // The target button is painted above the four overlapping D-pad arms and
    // therefore owns opaque pixels in their shared bounding region.
    TacticalHudControlSpec {
        control: TacticalHudControl::CameraTarget,
        rect: NativeRect::new(538.0, 379.0, 23.0, 23.0),
        hit_resource: resources::tactical::BTN_CAMERA_TARGET_NORMAL,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::CameraLeft,
        rect: NativeRect::new(511.0, 376.0, 42.0, 43.0),
        hit_resource: resources::tactical::BTN_CAMERA_LEFT_NORMAL,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::CameraRight,
        rect: NativeRect::new(557.0, 376.0, 43.0, 43.0),
        hit_resource: resources::tactical::BTN_CAMERA_RIGHT_NORMAL,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::CameraUp,
        rect: NativeRect::new(537.0, 344.0, 43.0, 43.0),
        hit_resource: resources::tactical::BTN_CAMERA_UP_NORMAL,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::CameraDown,
        rect: NativeRect::new(537.0, 412.0, 43.0, 43.0),
        hit_resource: resources::tactical::BTN_CAMERA_DOWN_NORMAL,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::CameraRecall,
        rect: NativeRect::new(484.0, 430.0, 43.0, 25.0),
        hit_resource: resources::tactical::BTN_CAMERA_RECALL_NORMAL,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::CameraMemorize,
        rect: NativeRect::new(585.0, 430.0, 43.0, 25.0),
        hit_resource: resources::tactical::BTN_CAMERA_MEMORIZE_NORMAL,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::HighlightAlliance,
        rect: NativeRect::new(517.0, 304.0, 30.0, 26.0),
        hit_resource: resources::tactical::DIM_ALLIANCE_SHIPS,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::HighlightEmpire,
        rect: NativeRect::new(482.0, 304.0, 30.0, 26.0),
        hit_resource: resources::tactical::DIM_EMPIRE_SHIPS,
    },
    TacticalHudControlSpec {
        control: TacticalHudControl::BattleOptions,
        rect: NativeRect::new(606.0, 308.0, 20.0, 20.0),
        hit_resource: resources::tactical::BTN_BATTLE_OPTIONS_NORMAL,
    },
];

#[derive(Debug, Clone, Copy)]
struct TacticalBattleOptionsSpec {
    control: TacticalBattleOptionsControl,
    rect: NativeRect,
    normal_resource: u32,
    pressed_resource: u32,
}

// REBEXE FUN_005dab90 places these at panel-local (18, 19), (14, 93),
// (87, 93), (23, 184) and (97, 184) on panel 1306, whose container
// `FUN_0059cac0` places at (482, 28) (0x1e2, 0x1c).
const TACTICAL_BATTLE_OPTIONS_CONTROLS: [TacticalBattleOptionsSpec; 5] = [
    TacticalBattleOptionsSpec {
        control: TacticalBattleOptionsControl::Withdraw,
        rect: NativeRect::new(500.0, 47.0, 48.0, 34.0),
        normal_resource: resources::tactical::BTN_WITHDRAW_FROM_BATTLE_NORMAL,
        pressed_resource: resources::tactical::BTN_WITHDRAW_FROM_BATTLE_PRESSED,
    },
    TacticalBattleOptionsSpec {
        control: TacticalBattleOptionsControl::SimulateRemainder,
        rect: NativeRect::new(496.0, 121.0, 44.0, 25.0),
        normal_resource: resources::tactical::BTN_SIMULATE_REMAINDER_NORMAL,
        pressed_resource: resources::tactical::BTN_SIMULATE_REMAINDER_PRESSED,
    },
    TacticalBattleOptionsSpec {
        control: TacticalBattleOptionsControl::ObserveToggle,
        rect: NativeRect::new(569.0, 121.0, 45.0, 25.0),
        normal_resource: resources::tactical::BTN_OBSERVE_TOGGLE_NORMAL,
        pressed_resource: resources::tactical::BTN_OBSERVE_TOGGLE_PRESSED,
    },
    TacticalBattleOptionsSpec {
        control: TacticalBattleOptionsControl::GameOptions,
        rect: NativeRect::new(505.0, 212.0, 52.0, 33.0),
        normal_resource: resources::tactical::BTN_GAME_OPTIONS_NORMAL,
        pressed_resource: resources::tactical::BTN_GAME_OPTIONS_PRESSED,
    },
    TacticalBattleOptionsSpec {
        control: TacticalBattleOptionsControl::Close,
        rect: NativeRect::new(579.0, 212.0, 29.0, 33.0),
        normal_resource: resources::tactical::BTN_BATTLE_OPTIONS_CLOSE_NORMAL,
        pressed_resource: resources::tactical::BTN_BATTLE_OPTIONS_CLOSE_PRESSED,
    },
];

#[derive(Debug, Clone, Copy)]
struct TacticalWithdrawConfirmationSpec {
    control: TacticalWithdrawConfirmationControl,
    rect: NativeRect,
    normal_resource: u32,
    pressed_resource: u32,
}

// REBEXE FUN_005f0f40 constructs these controls at panel-local (66, 202)
// and (106, 202), each 27x25, with TACTICAL resources 1113 through 1116.
const TACTICAL_WITHDRAW_CONFIRMATION_CONTROLS: [TacticalWithdrawConfirmationSpec; 2] = [
    TacticalWithdrawConfirmationSpec {
        control: TacticalWithdrawConfirmationControl::Confirm,
        rect: NativeRect::new(548.0, 230.0, 27.0, 25.0),
        normal_resource: resources::tactical::BTN_ASSIGN_CONFIRM_NORMAL,
        pressed_resource: resources::tactical::BTN_ASSIGN_CONFIRM_PRESSED,
    },
    TacticalWithdrawConfirmationSpec {
        control: TacticalWithdrawConfirmationControl::Cancel,
        rect: NativeRect::new(588.0, 230.0, 27.0, 25.0),
        normal_resource: resources::tactical::BTN_ASSIGN_CANCEL_NORMAL,
        pressed_resource: resources::tactical::BTN_ASSIGN_CANCEL_PRESSED,
    },
];

const DEATH_STAR_LASER_GAUGE_RECT: NativeRect = NativeRect::new(474.0, 0.0, 166.0, 25.0);
const DEATH_STAR_LASER_BUTTON_RECT: NativeRect = NativeRect::new(613.0, 2.0, 21.0, 20.0);
const DEATH_STAR_LASER_FILL_RECT: NativeRect = NativeRect::new(508.0, 9.0, 95.0, 6.0);

// REBEXE FUN_0059be80 builds the task-force header 0xad01 at (9, 1) and the
// eight slots 0xad02-0xad09 at x 0x3c + 0x1d * i, y 2, each 0x19 x 0x11.
const TASK_FORCE_HEADER_POINT: (f32, f32) = (9.0, 1.0);
const TASK_FORCE_BUTTON_X: f32 = 60.0;
const GROUP_BUTTON_PITCH: f32 = 29.0;

const TASK_FORCE_BUTTONS: [NativeRect; 8] = [
    NativeRect::new(60.0, 2.0, 25.0, 17.0),
    NativeRect::new(89.0, 2.0, 25.0, 17.0),
    NativeRect::new(118.0, 2.0, 25.0, 17.0),
    NativeRect::new(147.0, 2.0, 25.0, 17.0),
    NativeRect::new(176.0, 2.0, 25.0, 17.0),
    NativeRect::new(205.0, 2.0, 25.0, 17.0),
    NativeRect::new(234.0, 2.0, 25.0, 17.0),
    NativeRect::new(263.0, 2.0, 25.0, 17.0),
];

// REBEXE FUN_0059c030 builds the fighter-group header 0xad0a at (0x128, 1)
// and the four group buttons 0xad0b-0xad0e at x 0x15b + 0x1d * i, y 2, each
// 0x19 x 0xf: the 25x17 squadron bitmaps show only their top 15 rows (A0
// TAC-03-C019 shows the strip background on rows 17 and 18).
const FIGHTER_GROUP_HEADER_POINT: (f32, f32) = (296.0, 1.0);
const FIGHTER_GROUP_BUTTON_X: f32 = 347.0;
const FIGHTER_GROUP_BUTTON_HEIGHT: f32 = 15.0;

const FIGHTER_GROUP_BUTTONS: [NativeRect; 4] = [
    NativeRect::new(347.0, 2.0, 25.0, 15.0),
    NativeRect::new(376.0, 2.0, 25.0, 15.0),
    NativeRect::new(405.0, 2.0, 25.0, 15.0),
    NativeRect::new(434.0, 2.0, 25.0, 15.0),
];

const FIGHTER_GROUP_NORMAL_ART: [u32; 4] = [
    resources::tactical::BTN_RED_SQUADRON_NORMAL,
    resources::tactical::BTN_BLUE_SQUADRON_NORMAL,
    resources::tactical::BTN_GREEN_SQUADRON_NORMAL,
    resources::tactical::BTN_GOLD_SQUADRON_NORMAL,
];

const FIGHTER_GROUP_PRESSED_ART: [u32; 4] = [
    resources::tactical::BTN_RED_SQUADRON_PRESSED,
    resources::tactical::BTN_BLUE_SQUADRON_PRESSED,
    resources::tactical::BTN_GREEN_SQUADRON_PRESSED,
    resources::tactical::BTN_GOLD_SQUADRON_PRESSED,
];

const DISPLAY_COMMAND_CONTROLS: [TacticalCommandControl; 4] = [
    TacticalCommandControl::PreviousCapital,
    TacticalCommandControl::NextCapital,
    TacticalCommandControl::OpenMissions,
    TacticalCommandControl::OpenManeuvers,
];

const MANEUVER_COMMAND_CONTROLS: [TacticalCommandControl; 8] = [
    TacticalCommandControl::Hammer,
    TacticalCommandControl::Anvil,
    TacticalCommandControl::LeftHook,
    TacticalCommandControl::RightHook,
    TacticalCommandControl::HoldPosition,
    TacticalCommandControl::ToggleTactic,
    TacticalCommandControl::Confirm,
    TacticalCommandControl::Cancel,
];

const MISSION_COMMAND_CONTROLS: [TacticalCommandControl; 6] = [
    TacticalCommandControl::AttackCapitalShips,
    TacticalCommandControl::AttackFighters,
    TacticalCommandControl::Recover,
    TacticalCommandControl::AttackDeathStar,
    TacticalCommandControl::Confirm,
    TacticalCommandControl::Cancel,
];

fn tactical_command_spec(
    control: TacticalCommandControl,
    player_is_alliance: bool,
) -> (NativeRect, u32) {
    use resources::tactical as art;
    match control {
        TacticalCommandControl::PreviousCapital => (
            NativeRect::new(489.0, 38.0, 11.0, 53.0),
            art::BTN_PREVIOUS_CAPITAL_NORMAL,
        ),
        TacticalCommandControl::NextCapital => (
            NativeRect::new(614.0, 38.0, 11.0, 53.0),
            art::BTN_NEXT_CAPITAL_NORMAL,
        ),
        // FUN_005e4110: 1107 at panel x 12 and 1105 at x 80. The original
        // opens Maneuvers and Tactics from the left and Missions from the
        // right (A0 TAC-05-C006, TAC-05-C012).
        TacticalCommandControl::OpenMissions => (
            NativeRect::new(562.0, 238.0, 58.0, 22.0),
            art::BTN_MISSIONS_NORMAL,
        ),
        TacticalCommandControl::OpenManeuvers => (
            NativeRect::new(494.0, 238.0, 58.0, 22.0),
            art::BTN_MANEUVERS_TACTICS_NORMAL,
        ),
        TacticalCommandControl::LeftHook => (
            NativeRect::new(495.0, 187.0, 28.0, 28.0),
            art::BTN_LEFT_HOOK_NORMAL,
        ),
        TacticalCommandControl::RightHook => (
            NativeRect::new(525.0, 187.0, 28.0, 28.0),
            art::BTN_RIGHT_HOOK_NORMAL,
        ),
        TacticalCommandControl::Hammer => (
            NativeRect::new(495.0, 157.0, 28.0, 28.0),
            art::BTN_HAMMER_NORMAL,
        ),
        TacticalCommandControl::Anvil => (
            NativeRect::new(525.0, 157.0, 28.0, 28.0),
            art::BTN_ANVIL_NORMAL,
        ),
        TacticalCommandControl::HoldPosition => (
            NativeRect::new(557.0, 172.0, 28.0, 28.0),
            art::BTN_HOLD_POSITION_NORMAL,
        ),
        TacticalCommandControl::ToggleTactic => (
            NativeRect::new(593.0, 160.0, 24.0, 52.0),
            art::BTN_TACTIC_STAND_OFF,
        ),
        TacticalCommandControl::AttackCapitalShips => (
            NativeRect::new(500.0, 157.0, 46.0, 26.0),
            if player_is_alliance {
                art::BTN_ATTACK_CAPITAL_SHIPS_ALLIANCE_NORMAL
            } else {
                art::BTN_ATTACK_CAPITAL_SHIPS_EMPIRE_NORMAL
            },
        ),
        TacticalCommandControl::AttackFighters => (
            NativeRect::new(500.0, 187.0, 46.0, 26.0),
            if player_is_alliance {
                art::BTN_ATTACK_FIGHTERS_ALLIANCE_NORMAL
            } else {
                art::BTN_ATTACK_FIGHTERS_EMPIRE_NORMAL
            },
        ),
        TacticalCommandControl::Recover => (
            NativeRect::new(568.0, 157.0, 46.0, 26.0),
            if player_is_alliance {
                art::BTN_RECOVER_ALLIANCE_NORMAL
            } else {
                art::BTN_RECOVER_EMPIRE_NORMAL
            },
        ),
        TacticalCommandControl::AttackDeathStar => (
            NativeRect::new(568.0, 187.0, 46.0, 26.0),
            art::BTN_ATTACK_DEATH_STAR_NORMAL,
        ),
        TacticalCommandControl::Confirm => (
            NativeRect::new(548.0, 230.0, 27.0, 25.0),
            art::BTN_ASSIGN_CONFIRM_NORMAL,
        ),
        TacticalCommandControl::Cancel => (
            NativeRect::new(588.0, 230.0, 27.0, 25.0),
            art::BTN_ASSIGN_CANCEL_NORMAL,
        ),
    }
}

fn tactical_command_controls(panel: TacticalCommandPanel) -> &'static [TacticalCommandControl] {
    match panel {
        TacticalCommandPanel::Display => &DISPLAY_COMMAND_CONTROLS,
        TacticalCommandPanel::Maneuvers { .. } => &MANEUVER_COMMAND_CONTROLS,
        TacticalCommandPanel::Missions { .. } => &MISSION_COMMAND_CONTROLS,
    }
}

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "A containing native bitmap rectangle guarantees finite nonnegative local pixels."
)]
#[expect(
    clippy::too_many_arguments,
    reason = "The source control predicate keeps panel, selection, faction, resource mask, and pointer coordinates explicit."
)]
fn tactical_command_control_at(
    cache: &mut BmpCache,
    panel: TacticalCommandPanel,
    has_selection: bool,
    selected_fighter: bool,
    attack_death_star_enabled: bool,
    player_is_alliance: bool,
    x: f32,
    y: f32,
) -> Option<TacticalCommandControl> {
    if !has_selection {
        return None;
    }
    tactical_command_controls(panel)
        .iter()
        .copied()
        .find(|control| {
            if (matches!(
                *control,
                TacticalCommandControl::PreviousCapital | TacticalCommandControl::NextCapital
            ) && selected_fighter)
                || (*control == TacticalCommandControl::Recover && !selected_fighter)
                || (*control == TacticalCommandControl::AttackDeathStar
                    && !attack_death_star_enabled)
            {
                return false;
            }
            let (rect, resource) = tactical_command_spec(*control, player_is_alliance);
            if !rect.contains(x, y) {
                return false;
            }
            cache.is_resource_hit(
                DllSource::Tactical,
                resource,
                (x - rect.x).floor() as usize,
                (y - rect.y).floor() as usize,
            )
        })
}

#[cfg(test)]
fn tactical_command_rect_control_at(
    panel: TacticalCommandPanel,
    has_selection: bool,
    selected_fighter: bool,
    attack_death_star_enabled: bool,
    x: f32,
    y: f32,
) -> Option<TacticalCommandControl> {
    if !has_selection {
        return None;
    }
    tactical_command_controls(panel)
        .iter()
        .copied()
        .find(|control| {
            (!matches!(
                *control,
                TacticalCommandControl::PreviousCapital | TacticalCommandControl::NextCapital
            ) || !selected_fighter)
                && (*control != TacticalCommandControl::Recover || selected_fighter)
                && (*control != TacticalCommandControl::AttackDeathStar
                    || attack_death_star_enabled)
                && tactical_command_spec(*control, true).0.contains(x, y)
        })
}

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "A containing native bitmap rectangle guarantees finite nonnegative local pixels."
)]
fn tactical_hud_control_at(cache: &mut BmpCache, x: f32, y: f32) -> Option<TacticalHudControl> {
    for (index, rect) in TASK_FORCE_BUTTONS.iter().enumerate() {
        if rect.contains(x, y) {
            let pixel_x = (x - rect.x).floor() as usize;
            let pixel_y = (y - rect.y).floor() as usize;
            if cache.is_resource_hit(
                DllSource::Tactical,
                resources::tactical::BTN_TASK_FORCE_NORMAL,
                pixel_x,
                pixel_y,
            ) {
                return Some(TacticalHudControl::TaskForce(
                    u8::try_from(index).unwrap_or(7),
                ));
            }
        }
    }
    for (index, rect) in FIGHTER_GROUP_BUTTONS.iter().enumerate() {
        if rect.contains(x, y) {
            let pixel_x = (x - rect.x).floor() as usize;
            let pixel_y = (y - rect.y).floor() as usize;
            if cache.is_resource_hit(
                DllSource::Tactical,
                FIGHTER_GROUP_NORMAL_ART[index],
                pixel_x,
                pixel_y,
            ) {
                return Some(TacticalHudControl::FighterGroup(
                    u8::try_from(index).unwrap_or(3),
                ));
            }
        }
    }
    TACTICAL_HUD_CONTROLS.iter().find_map(|spec| {
        if !spec.rect.contains(x, y) {
            return None;
        }
        let pixel_x = (x - spec.rect.x).floor() as usize;
        let pixel_y = (y - spec.rect.y).floor() as usize;
        cache
            .is_resource_hit(DllSource::Tactical, spec.hit_resource, pixel_x, pixel_y)
            .then_some(spec.control)
    })
}

fn tactical_battle_options_control_at(
    cache: &mut BmpCache,
    withdraw_enabled: bool,
    x: f32,
    y: f32,
) -> Option<TacticalBattleOptionsControl> {
    TACTICAL_BATTLE_OPTIONS_CONTROLS.iter().find_map(|spec| {
        if spec.control == TacticalBattleOptionsControl::Withdraw && !withdraw_enabled {
            return None;
        }
        if !spec.rect.contains(x, y) {
            return None;
        }
        cache
            .is_resource_hit(
                DllSource::Tactical,
                spec.normal_resource,
                (x - spec.rect.x).floor() as usize,
                (y - spec.rect.y).floor() as usize,
            )
            .then_some(spec.control)
    })
}

fn tactical_withdraw_confirmation_control_at(
    cache: &mut BmpCache,
    x: f32,
    y: f32,
) -> Option<TacticalWithdrawConfirmationControl> {
    TACTICAL_WITHDRAW_CONFIRMATION_CONTROLS
        .iter()
        .find_map(|spec| {
            if !spec.rect.contains(x, y) {
                return None;
            }
            cache
                .is_resource_hit(
                    DllSource::Tactical,
                    spec.normal_resource,
                    (x - spec.rect.x).floor() as usize,
                    (y - spec.rect.y).floor() as usize,
                )
                .then_some(spec.control)
        })
}

fn player_death_star(session: &BattleSession) -> Option<TacticalDeathStar> {
    session
        .death_star
        .filter(|death_star| death_star.is_attacker == session.player_is_attacker)
}

fn tactical_death_star_laser_ready(session: &BattleSession) -> bool {
    player_death_star(session).is_some_and(|death_star| {
        death_star.operational()
            && death_star.laser_charge >= ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE
            && session.death_star_beam.is_none()
    })
}

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "The containing original bitmap rectangle guarantees finite nonnegative pixels."
)]
fn tactical_death_star_laser_control_at(cache: &mut BmpCache, x: f32, y: f32) -> bool {
    DEATH_STAR_LASER_BUTTON_RECT.contains(x, y)
        && cache.is_resource_hit(
            DllSource::Tactical,
            resources::tactical::DEATH_STAR_LASER_READY,
            (x - DEATH_STAR_LASER_BUTTON_RECT.x).floor() as usize,
            (y - DEATH_STAR_LASER_BUTTON_RECT.y).floor() as usize,
        )
}

fn tactical_withdraw_enabled(session: &BattleSession) -> bool {
    session.ships.iter().any(|ship| {
        ship.alive
            && !ship.retreating
            && ship.is_attacker == session.player_is_attacker
            && ship.subsystem_capacity.hyperdrive > 0
            && ship.subsystem_condition.hyperdrive > 0
    })
}

/// What `FUN_005e16a0` puts on the fighter-group panel below the three
/// top labels: the art and the count line `<type label> N a/b`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TacticalFighterPanel {
    /// TACTICAL 2030-2037 for the first type present.
    art: u32,
    type_label: &'static str,
    /// N: the group's squadrons of that type (vtable slot `0x3c(type)`).
    /// hyp: a squadron count; A0 TAC-03-C019 shows six for 72/72.
    squadrons_of_type: u32,
    /// a: `FUN_005c6530`, over every squadron in the group,
    /// `max(1, trunc(hull / 8))` (`FUN_005ba330` through `FUN_005b1770`).
    craft: u32,
    /// b: `FUN_005c6570`, the same with the maximum hull (`FUN_005ba380`).
    capacity: u32,
}

impl TacticalFighterPanel {
    /// `FUN_005e16a0` tests the types of the local player's side
    /// (`FUN_00595c40`: Alliance `0x1d`-`0x20`, Empire `0x21`-`0x24`) in
    /// order and labels the first one the group holds. With none present it
    /// leaves the count label and art untouched; the port draws neither.
    fn for_group(session: &BattleSession, group: u8) -> Option<Self> {
        let squadrons = || {
            session.fighters.iter().filter(move |fighter| {
                fighter.alive
                    && fighter.squad_count > 0
                    && fighter.is_attacker == session.player_is_attacker
                    && fighter.fighter_group == group
            })
        };
        let first_type: u8 = if player_is_alliance(session) {
            0x1d
        } else {
            0x21
        };
        let (type_id, squadrons_of_type) = (first_type..first_type + 4).find_map(|type_id| {
            let count = squadrons()
                .filter(|fighter| {
                    fighter
                        .tactical_resource
                        .is_some_and(|resource| resource.tactical_ordinal == type_id)
                })
                .count() as u32;
            (count > 0).then_some((type_id, count))
        })?;
        let craft_of = |hull: f32| ((hull / ORIGINAL_FIGHTER_HULL_PER_CRAFT) as u32).max(1);
        let index = usize::from(type_id - 0x1d);
        Some(Self {
            art: 2030 + index as u32,
            type_label: TACTICAL_FIGHTER_TYPE_LABELS[index],
            squadrons_of_type,
            craft: squadrons()
                .map(|fighter| craft_of(fighter.hull_current))
                .sum(),
            capacity: squadrons().map(|fighter| craft_of(fighter.hull_max)).sum(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct TacticalGroupHud<'a> {
    task_forces: [bool; 8],
    fighter_groups: [bool; 4],
    selected_task_force: Option<u8>,
    selected_fighter_group: Option<u8>,
    selected_ship_name: Option<&'a str>,
    selected_ship_hud: Option<u32>,
    selected_ship_hull: Option<(i32, i32)>,
    selected_ship_shield: Option<(i32, i32)>,
    selected_ship_subsystems: Option<TacticalSubsystemCondition>,
    selected_ship_contents: &'a [TacticalShipContent],
    selected_fighter_panel: Option<TacticalFighterPanel>,
    selected_order: TacticalOrder,
    selected_tactic: TacticalTactic,
}

impl<'a> TacticalGroupHud<'a> {
    fn from_session(session: &'a BattleSession) -> Self {
        let mut task_forces = [false; 8];
        let mut fighter_groups = [false; 4];
        for ship in &session.ships {
            if ship.alive && ship.is_attacker == session.player_is_attacker {
                task_forces[usize::from(ship.task_force.min(7))] = true;
            }
        }
        for fighter in &session.fighters {
            if fighter.alive
                && fighter.squad_count > 0
                && fighter.is_attacker == session.player_is_attacker
                && fighter.fighter_group < 4
            {
                fighter_groups[usize::from(fighter.fighter_group)] = true;
            }
        }
        let selected_task_force = selected_player_task_force(session);
        let selected_ship_name = session
            .ships
            .iter()
            .find(|ship| {
                ship.alive && ship.selected && ship.is_attacker == session.player_is_attacker
            })
            .map(|ship| ship.name.as_str());
        let selected_ship = session.ships.iter().find(|ship| {
            ship.alive && ship.selected && ship.is_attacker == session.player_is_attacker
        });
        let selected_fighter = session.selected_fighter_group.and_then(|group| {
            session.fighters.iter().find(|fighter| {
                fighter.alive
                    && fighter.selected
                    && fighter.is_attacker == session.player_is_attacker
                    && fighter.fighter_group == group
            })
        });
        Self {
            task_forces,
            fighter_groups,
            selected_task_force,
            selected_fighter_group: session.selected_fighter_group,
            selected_ship_name,
            selected_ship_hud: selected_ship
                .and_then(|ship| ship.tactical_resource)
                .map(TacticalCapitalShipResource::hud_resource),
            selected_ship_hull: selected_ship.map(|ship| (ship.hull_current, ship.hull_max)),
            selected_ship_shield: selected_ship.map(|ship| (ship.shield, ship.shield_max)),
            selected_ship_subsystems: selected_ship.map(|ship| ship.subsystem_condition),
            selected_ship_contents: selected_ship.map_or(&[], |ship| ship.contents.as_slice()),
            selected_fighter_panel: session
                .selected_fighter_group
                .and_then(|group| TacticalFighterPanel::for_group(session, group)),
            selected_order: selected_fighter.map_or_else(
                || selected_ship.map_or(TacticalOrder::None, |ship| ship.order),
                |fighter| fighter.order,
            ),
            selected_tactic: selected_fighter.map_or_else(
                || selected_ship.map_or(TacticalTactic::StandOff, |ship| ship.tactic),
                |fighter| fighter.tactic,
            ),
        }
    }
}

fn tactical_meter_percent(current: i32, maximum: i32) -> u32 {
    if maximum <= 0 {
        0
    } else {
        ((current as f32 / maximum as f32).clamp(0.0, 1.0) * 100.0) as u32
    }
}

/// `FUN_0060e460` paints the red background, then `width * percent / 100`
/// whole pixels of blue fill.
fn draw_tactical_selected_meter(canvas: TacticalCanvas, rect: NativeRect, percent: u32) {
    let (x, y) = canvas.point(rect.x, rect.y);
    let height = rect.height * canvas.scale;
    draw_rectangle(
        x,
        y,
        rect.width * canvas.scale,
        height,
        Color::new(1.0, 0.0, 0.0, 1.0),
    );
    let fill = (rect.width as u32 * percent.min(100) / 100) as f32;
    if fill > 0.0 {
        draw_rectangle(
            x,
            y,
            fill * canvas.scale,
            height,
            Color::new(0.0, 0.0, 1.0, 1.0),
        );
    }
}

fn selected_player_task_force(session: &BattleSession) -> Option<u8> {
    if session.selected_fighter_group.is_some() {
        return None;
    }
    let mut selected = session.ships.iter().filter(|ship| {
        ship.alive && ship.selected && ship.is_attacker == session.player_is_attacker
    });
    let group = selected.next()?.task_force;
    selected
        .all(|ship| ship.task_force == group)
        .then_some(group)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TacticalFocusAssignment {
    Assigned {
        object_ids: Vec<usize>,
        maximum_targets: usize,
    },
    RejectedMixedTaskForces {
        member_count: usize,
    },
}

/// Apply the original enemy-focus constraint recovered from `FUN_005a24d0`.
/// A capital selection spanning more than one task force is rejected before
/// any target or escort state changes, and the faction-specific source event
/// is queued for the production audio path.
fn assign_selected_capital_focus_target(
    session: &mut BattleSession,
    target_idx: usize,
    append: bool,
) -> TacticalFocusAssignment {
    let selected = session
        .ships
        .iter()
        .enumerate()
        .filter_map(|(index, ship)| {
            (ship.selected && ship.is_attacker == session.player_is_attacker && ship.alive)
                .then_some(index)
        })
        .collect::<Vec<_>>();
    let Some(first) = selected.first().copied() else {
        return TacticalFocusAssignment::Assigned {
            object_ids: Vec::new(),
            maximum_targets: 0,
        };
    };
    let task_force = session.ships[first].task_force;
    if selected
        .iter()
        .any(|index| session.ships[*index].task_force != task_force)
    {
        session
            .pending_voice_cues
            .push(TacticalVoiceCue::target_task_force_rejected(
                session.player_voice_faction(),
            ));
        return TacticalFocusAssignment::RejectedMixedTaskForces {
            member_count: selected.len(),
        };
    }

    let target = TacticalAttackTarget::CapitalShip(target_idx);
    for index in selected.iter().copied() {
        let ship = &mut session.ships[index];
        ship.escort_target = None;
        if append {
            if let Some(position) = ship
                .manual_targets
                .iter()
                .position(|existing| *existing == target)
            {
                ship.manual_targets.remove(position);
            } else {
                ship.manual_targets.push(target);
            }
        } else {
            ship.manual_targets.clear();
            ship.manual_targets.push(target);
        }
        ship.attack_target = ship.manual_targets.first().copied();
    }
    let object_ids = selected
        .iter()
        .filter_map(|index| {
            (session.ships[*index].attack_target == Some(target)).then_some(*index + 1)
        })
        .collect::<Vec<_>>();
    let maximum_targets = selected
        .iter()
        .map(|index| session.ships[*index].manual_targets.len())
        .max()
        .unwrap_or(0);
    TacticalFocusAssignment::Assigned {
        object_ids,
        maximum_targets,
    }
}

fn select_task_force(session: &mut BattleSession, group: u8) -> usize {
    let first = session.ships.iter().position(|ship| {
        ship.alive
            && !ship.retreating
            && ship.is_attacker == session.player_is_attacker
            && ship.task_force == group
    });
    let Some(first) = first else {
        return 0;
    };
    for ship in &mut session.ships {
        ship.selected = ship.alive
            && !ship.retreating
            && ship.is_attacker == session.player_is_attacker
            && ship.task_force == group;
    }
    for fighter in &mut session.fighters {
        fighter.selected = false;
    }
    session.selected_ship = Some(first);
    session.selected_fighter_group = None;
    session.ships.iter().filter(|ship| ship.selected).count()
}

fn assign_selected_to_task_force(session: &mut BattleSession, group: u8) -> usize {
    let mut assigned_groups = [false; 8];
    for ship in &session.ships {
        if ship.alive && ship.is_attacker == session.player_is_attacker {
            assigned_groups[usize::from(ship.task_force.min(7))] = true;
        }
    }
    let next_blank = assigned_groups
        .iter()
        .position(|assigned| !assigned)
        .and_then(|index| u8::try_from(index).ok());
    if !assigned_groups[usize::from(group.min(7))] && next_blank != Some(group.min(7)) {
        return 0;
    }
    let mut assigned = 0;
    for ship in &mut session.ships {
        if ship.alive && ship.selected && ship.is_attacker == session.player_is_attacker {
            ship.task_force = group.min(7);
            assigned += 1;
        }
    }
    if assigned > 0 {
        BattleSession::refresh_original_task_force_geometry(&mut session.ships);
    }
    assigned
}

fn select_fighter_group(session: &mut BattleSession, group: u8) -> bool {
    let assigned = session.fighters.iter().any(|fighter| {
        fighter.alive
            && fighter.squad_count > 0
            && fighter.is_attacker == session.player_is_attacker
            && fighter.fighter_group == group
    });
    if !assigned {
        return false;
    }
    for ship in &mut session.ships {
        ship.selected = false;
    }
    for fighter in &mut session.fighters {
        fighter.selected = fighter.alive
            && fighter.squad_count > 0
            && fighter.is_attacker == session.player_is_attacker
            && fighter.fighter_group == group;
    }
    session.selected_ship = None;
    session.selected_fighter_group = Some(group);
    true
}

fn cycle_selected_capital(session: &mut BattleSession, step: isize) -> Option<(usize, usize, u8)> {
    let current = session.selected_ship.or_else(|| {
        session.ships.iter().position(|ship| {
            ship.alive && ship.selected && ship.is_attacker == session.player_is_attacker
        })
    })?;
    let group = session.ships.get(current)?.task_force;
    let eligible = session
        .ships
        .iter()
        .enumerate()
        .filter(|(_, ship)| {
            ship.alive
                && !ship.retreating
                && ship.is_attacker == session.player_is_attacker
                && ship.task_force == group
        })
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    let current_position = eligible.iter().position(|index| *index == current)?;
    let count = isize::try_from(eligible.len()).ok()?;
    let next_position = (isize::try_from(current_position).ok()? + step).rem_euclid(count);
    let next = eligible[usize::try_from(next_position).ok()?];
    for (index, ship) in session.ships.iter_mut().enumerate() {
        ship.selected = index == next;
    }
    for fighter in &mut session.fighters {
        fighter.selected = false;
    }
    session.selected_ship = Some(next);
    session.selected_fighter_group = None;
    Some((current, next, group))
}

fn selected_command_values(session: &BattleSession) -> (TacticalOrder, TacticalTactic) {
    if let Some(group) = session.selected_fighter_group {
        if let Some(fighter) = session.fighters.iter().find(|fighter| {
            fighter.alive
                && fighter.selected
                && fighter.is_attacker == session.player_is_attacker
                && fighter.fighter_group == group
        }) {
            return (fighter.order, fighter.tactic);
        }
    }
    session
        .ships
        .iter()
        .find(|ship| ship.alive && ship.selected && ship.is_attacker == session.player_is_attacker)
        .map_or((TacticalOrder::None, TacticalTactic::StandOff), |ship| {
            (ship.order, ship.tactic)
        })
}

fn assign_selected_command(
    session: &mut BattleSession,
    order: TacticalOrder,
    tactic: Option<TacticalTactic>,
) -> (usize, usize) {
    let mut capital_members = 0;
    let mut fighter_members = 0;
    let mut selected_capitals = Vec::new();
    let mut selected_fighters = Vec::new();
    if session.selected_fighter_group.is_some() {
        for (index, fighter) in session.fighters.iter_mut().enumerate() {
            if fighter.alive
                && fighter.selected
                && fighter.is_attacker == session.player_is_attacker
            {
                fighter.order = order;
                fighter.escort_target = None;
                if let Some(tactic) = tactic {
                    fighter.tactic = tactic;
                }
                if order != TacticalOrder::Recover {
                    fighter.recovery_state = TacticalFighterRecoveryState::AwaitingCarrier;
                    fighter.recovery_target = None;
                }
                selected_fighters.push(index);
                fighter_members += 1;
            }
        }
    } else {
        for (index, ship) in session.ships.iter_mut().enumerate() {
            if ship.alive && ship.selected && ship.is_attacker == session.player_is_attacker {
                ship.order = order;
                ship.escort_target = None;
                if let Some(tactic) = tactic {
                    ship.tactic = tactic;
                }
                selected_capitals.push(index);
                capital_members += 1;
            }
        }
    }
    begin_original_capital_order(session, &selected_capitals, order);
    begin_original_attack_order(session, &selected_capitals, &selected_fighters, order);
    if order == TacticalOrder::Recover {
        begin_original_fighter_recovery(session, &selected_fighters);
    }
    (capital_members, fighter_members)
}

fn queue_selected_command_voice(
    session: &mut BattleSession,
    panel: TacticalCommandPanel,
    order: TacticalOrder,
    formation_changed: bool,
) {
    let faction = if player_is_alliance(session) {
        TacticalVoiceFaction::Alliance
    } else {
        TacticalVoiceFaction::Empire
    };
    let cue = if let Some(group) = session.selected_fighter_group {
        match (panel, order) {
            // `FUN_005d0340` dispatches Attack Death Star entry from the
            // order executor because the local listener may be either the
            // Alliance attackers or the Imperial station owner. The shared
            // executor above has already queued that perspective-correct cue.
            (TacticalCommandPanel::Missions { .. }, TacticalOrder::AttackDeathStar) => return,
            (TacticalCommandPanel::Maneuvers { .. }, _) => {
                TacticalVoiceCue::fighter_maneuver(faction, group)
            }
            (TacticalCommandPanel::Missions { .. }, _)
                if matches!(
                    order,
                    TacticalOrder::AttackCapitalShips | TacticalOrder::AttackFighters
                ) =>
            {
                TacticalVoiceCue::fighter_attack(faction, group)
            }
            (TacticalCommandPanel::Missions { .. }, _) => {
                TacticalVoiceCue::fighter_mission(faction, group)
            }
            (TacticalCommandPanel::Display, _) => return,
        }
    } else {
        let Some(group) = selected_player_task_force(session) else {
            return;
        };
        match panel {
            TacticalCommandPanel::Maneuvers { .. } if formation_changed => {
                TacticalVoiceCue::capital_formation(faction, group)
            }
            TacticalCommandPanel::Maneuvers { .. } => {
                TacticalVoiceCue::capital_maneuver(faction, group)
            }
            TacticalCommandPanel::Missions { .. }
                if matches!(
                    order,
                    TacticalOrder::AttackCapitalShips | TacticalOrder::AttackFighters
                ) =>
            {
                TacticalVoiceCue::capital_attack(faction, group)
            }
            TacticalCommandPanel::Missions { .. } => {
                TacticalVoiceCue::capital_mission(faction, group)
            }
            TacticalCommandPanel::Display => return,
        }
    };
    session.pending_voice_cues.push(cue);
}

fn pressed_tactical_hud_control(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
) -> Option<TacticalHudControl> {
    if !is_mouse_button_down(MouseButton::Left) {
        return None;
    }
    let (mouse_x, mouse_y) = mouse_position();
    let (x, y) = canvas.logical_pointer(mouse_x, mouse_y);
    tactical_hud_control_at(cache, x, y)
}

fn pressed_tactical_battle_options_control(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
    withdraw_enabled: bool,
) -> Option<TacticalBattleOptionsControl> {
    if !is_mouse_button_down(MouseButton::Left) {
        return None;
    }
    let (mouse_x, mouse_y) = mouse_position();
    let (x, y) = canvas.logical_pointer(mouse_x, mouse_y);
    tactical_battle_options_control_at(cache, withdraw_enabled, x, y)
}

fn pressed_tactical_withdraw_confirmation_control(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
) -> Option<TacticalWithdrawConfirmationControl> {
    if !is_mouse_button_down(MouseButton::Left) {
        return None;
    }
    let (mouse_x, mouse_y) = mouse_position();
    let (x, y) = canvas.logical_pointer(mouse_x, mouse_y);
    tactical_withdraw_confirmation_control_at(cache, x, y)
}

fn pressed_tactical_command_control(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
    panel: TacticalCommandPanel,
    has_selection: bool,
    selected_fighter: bool,
    attack_death_star_enabled: bool,
    player_is_alliance: bool,
) -> Option<TacticalCommandControl> {
    if !is_mouse_button_down(MouseButton::Left) {
        return None;
    }
    let (mouse_x, mouse_y) = mouse_position();
    let (x, y) = canvas.logical_pointer(mouse_x, mouse_y);
    tactical_command_control_at(
        cache,
        panel,
        has_selection,
        selected_fighter,
        attack_death_star_enabled,
        player_is_alliance,
        x,
        y,
    )
}

#[cfg(test)]
fn tactical_hud_rect_control_at(x: f32, y: f32) -> Option<TacticalHudControl> {
    TACTICAL_HUD_CONTROLS
        .iter()
        .find_map(|spec| spec.rect.contains(x, y).then_some(spec.control))
}

fn draw_tactical_bitmap(cache: &mut BmpCache, id: u32, canvas: TacticalCanvas, x: f32, y: f32) {
    draw_original_bitmap(cache, DllSource::Tactical, id, canvas, x, y, None);
}

fn draw_strategy_bitmap(cache: &mut BmpCache, id: u32, canvas: TacticalCanvas, x: f32, y: f32) {
    draw_original_bitmap(cache, DllSource::Strategy, id, canvas, x, y, None);
}

fn draw_original_bitmap(
    cache: &mut BmpCache,
    source: DllSource,
    id: u32,
    canvas: TacticalCanvas,
    x: f32,
    y: f32,
    source_rect: Option<Rect>,
) {
    let Some(texture) = cache.get_macroquad_original(source, id) else {
        return;
    };
    let (screen_x, screen_y) = canvas.point(x, y);
    let source_width = source_rect.map_or_else(|| texture.width(), |rect| rect.w);
    let source_height = source_rect.map_or_else(|| texture.height(), |rect| rect.h);
    draw_texture_ex(
        texture,
        screen_x,
        screen_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(
                source_width * canvas.scale,
                source_height * canvas.scale,
            )),
            source: source_rect,
            ..Default::default()
        },
    );
}

/// Cache every glyph and size that the macroquad tactical pass can emit before
/// any texture-backed draw command is queued. Macroquad 0.4.14 replaces its
/// shared font-atlas texture when an uncached glyph expands the atlas, which
/// invalidates earlier commands in the same WebGL frame.
fn prewarm_tactical_font_atlas(
    session: &BattleSession,
    canvas: TacticalCanvas,
    warmed_sizes: &mut HashSet<u16>,
) {
    // The projected unit labels clamp to this complete integer range as the
    // camera zoom changes. Prewarm every value up front: adding only the
    // current zoom's sizes can repack Macroquad's shared glyph atlas during a
    // later zoom and perturb otherwise-static HUD text at fractional scales.
    let mut sizes = vec![7_u16, 8, 9, 10, 11, 12, 13, 14];
    sizes.extend([
        (10.0 * canvas.scale) as u16,
        (11.0 * canvas.scale) as u16,
        (12.0 * canvas.scale) as u16,
        (13.0 * canvas.scale) as u16,
        (14.0 * canvas.scale) as u16,
        (16.0 * canvas.scale) as u16,
        (18.0 * canvas.scale) as u16,
    ]);
    sizes.sort_unstable();
    // The selected panel's labels draw in the result face; its sizes are
    // keyed apart from the default face's.
    let result_size = (TACTICAL_SELECTED_TEXT_EM * canvas.scale).max(1.0) as u16;
    let result_key = result_size | 0x8000;
    if sizes.iter().all(|size| warmed_sizes.contains(size)) && warmed_sizes.contains(&result_key) {
        return;
    }

    let mut characters: std::collections::BTreeSet<_> =
        Font::latin_character_list().into_iter().collect();
    for text in [
        "0123456789x#",
        "Task Force No Orders RETREAT Battle Paused.",
        "Red Group Blue Group Green Group Gold Group",
        "Battle at Alliance Imperial Forces Capital Ships Fighters Troops Personnel",
        "Operational Damaged Destroyed Go Directly To System Fleet victorious withdrawn",
        "Withdraw Confirmation Are you sure you wish to order a fleet withdraw?",
    ] {
        characters.extend(text.chars());
    }
    characters.extend(session.system_name.chars());
    for ship in &session.ships {
        characters.extend(ship.name.chars());
    }
    for fighter in &session.fighters {
        characters.extend(fighter.name.chars());
    }
    let sample: String = characters.into_iter().collect();

    let mut previous = None;
    for size in sizes {
        if size > 0 && previous != Some(size) && warmed_sizes.insert(size) {
            measure_text(&sample, None, size, 1.0);
            previous = Some(size);
        }
    }
    if warmed_sizes.insert(result_key) {
        measure_text(&sample, result_font().as_ref(), result_size, 1.0);
    }
}

#[expect(
    clippy::cast_possible_truncation,
    reason = "The 640x480 tactical source canvas uses integer device-pixel clipping."
)]
fn set_tactical_aperture_clip(aperture: Option<NativeRect>) {
    let clip = aperture.map(|rect| {
        (
            rect.x.round() as i32,
            rect.y.round() as i32,
            rect.width.round() as i32,
            rect.height.round() as i32,
        )
    });
    // SAFETY: macroquad exposes its immediate drawing state through this API.
    // The clip is always reset before original HUD controls are painted.
    unsafe {
        get_internal_gl().quad_gl.scissor(clip);
    }
}

fn draw_tactical_assignment_button(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
    control: TacticalCommandControl,
    resource: u32,
) {
    let rect = tactical_command_spec(control, true).0;
    draw_tactical_bitmap(cache, resource, canvas, rect.x, rect.y);
}

fn draw_tactical_assignment_panel(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
    player_is_alliance: bool,
    panel: TacticalCommandPanel,
    selected_fighter: bool,
    attack_death_star_enabled: bool,
    pressed: Option<TacticalCommandControl>,
) {
    use resources::tactical as art;
    match panel {
        TacticalCommandPanel::Display => {}
        TacticalCommandPanel::Maneuvers {
            pending_order,
            pending_tactic,
        } => {
            draw_tactical_bitmap(cache, art::MANEUVERS_PANEL, canvas, 482.0, 28.0);
            if let Some(graphic) = pending_order.maneuver_graphic() {
                draw_tactical_bitmap(cache, graphic, canvas, 496.0, 42.0);
            }
            for (control, normal, selected) in [
                (
                    TacticalCommandControl::LeftHook,
                    art::BTN_LEFT_HOOK_NORMAL,
                    pending_order == TacticalOrder::LeftHook,
                ),
                (
                    TacticalCommandControl::RightHook,
                    art::BTN_RIGHT_HOOK_NORMAL,
                    pending_order == TacticalOrder::RightHook,
                ),
                (
                    TacticalCommandControl::Hammer,
                    art::BTN_HAMMER_NORMAL,
                    pending_order == TacticalOrder::Hammer,
                ),
                (
                    TacticalCommandControl::Anvil,
                    art::BTN_ANVIL_NORMAL,
                    pending_order == TacticalOrder::Anvil,
                ),
                (
                    TacticalCommandControl::HoldPosition,
                    art::BTN_HOLD_POSITION_NORMAL,
                    pending_order == TacticalOrder::HoldPosition,
                ),
            ] {
                let active = selected || pressed == Some(control);
                draw_tactical_assignment_button(
                    cache,
                    canvas,
                    control,
                    if active { normal + 1 } else { normal },
                );
            }
            draw_tactical_assignment_button(
                cache,
                canvas,
                TacticalCommandControl::ToggleTactic,
                if pending_tactic == TacticalTactic::Surround {
                    art::BTN_TACTIC_SURROUND
                } else {
                    art::BTN_TACTIC_STAND_OFF
                },
            );
            for (control, normal, pressed_art) in [
                (
                    TacticalCommandControl::Confirm,
                    art::BTN_ASSIGN_CONFIRM_NORMAL,
                    art::BTN_ASSIGN_CONFIRM_PRESSED,
                ),
                (
                    TacticalCommandControl::Cancel,
                    art::BTN_ASSIGN_CANCEL_NORMAL,
                    art::BTN_ASSIGN_CANCEL_PRESSED,
                ),
            ] {
                draw_tactical_assignment_button(
                    cache,
                    canvas,
                    control,
                    if pressed == Some(control) {
                        pressed_art
                    } else {
                        normal
                    },
                );
            }
        }
        TacticalCommandPanel::Missions { pending_order } => {
            draw_tactical_bitmap(cache, art::MISSIONS_PANEL, canvas, 482.0, 28.0);
            draw_tactical_bitmap(
                cache,
                pending_order
                    .mission_graphic(player_is_alliance)
                    .unwrap_or(art::MISSIONS_HUD_EMPTY),
                canvas,
                496.0,
                42.0,
            );
            for (control, normal, pressed_art, selected) in [
                (
                    TacticalCommandControl::AttackCapitalShips,
                    if player_is_alliance {
                        art::BTN_ATTACK_CAPITAL_SHIPS_ALLIANCE_NORMAL
                    } else {
                        art::BTN_ATTACK_CAPITAL_SHIPS_EMPIRE_NORMAL
                    },
                    if player_is_alliance {
                        art::BTN_ATTACK_CAPITAL_SHIPS_ALLIANCE_PRESSED
                    } else {
                        art::BTN_ATTACK_CAPITAL_SHIPS_EMPIRE_PRESSED
                    },
                    pending_order == TacticalOrder::AttackCapitalShips,
                ),
                (
                    TacticalCommandControl::AttackFighters,
                    if player_is_alliance {
                        art::BTN_ATTACK_FIGHTERS_ALLIANCE_NORMAL
                    } else {
                        art::BTN_ATTACK_FIGHTERS_EMPIRE_NORMAL
                    },
                    if player_is_alliance {
                        art::BTN_ATTACK_FIGHTERS_ALLIANCE_PRESSED
                    } else {
                        art::BTN_ATTACK_FIGHTERS_EMPIRE_PRESSED
                    },
                    pending_order == TacticalOrder::AttackFighters,
                ),
                (
                    TacticalCommandControl::Recover,
                    if player_is_alliance {
                        art::BTN_RECOVER_ALLIANCE_NORMAL
                    } else {
                        art::BTN_RECOVER_EMPIRE_NORMAL
                    },
                    if player_is_alliance {
                        art::BTN_RECOVER_ALLIANCE_PRESSED
                    } else {
                        art::BTN_RECOVER_EMPIRE_PRESSED
                    },
                    pending_order == TacticalOrder::Recover,
                ),
            ] {
                let resource = if control == TacticalCommandControl::Recover && !selected_fighter {
                    if player_is_alliance {
                        art::BTN_RECOVER_ALLIANCE_DISABLED
                    } else {
                        art::BTN_RECOVER_EMPIRE_DISABLED
                    }
                } else if selected || pressed == Some(control) {
                    pressed_art
                } else {
                    normal
                };
                draw_tactical_assignment_button(cache, canvas, control, resource);
            }
            draw_tactical_assignment_button(
                cache,
                canvas,
                TacticalCommandControl::AttackDeathStar,
                if !attack_death_star_enabled {
                    art::BTN_ATTACK_DEATH_STAR_DISABLED
                } else if pending_order == TacticalOrder::AttackDeathStar
                    || pressed == Some(TacticalCommandControl::AttackDeathStar)
                {
                    art::BTN_ATTACK_DEATH_STAR_PRESSED
                } else {
                    art::BTN_ATTACK_DEATH_STAR_NORMAL
                },
            );
            for (control, normal, pressed_art) in [
                (
                    TacticalCommandControl::Confirm,
                    art::BTN_ASSIGN_CONFIRM_NORMAL,
                    art::BTN_ASSIGN_CONFIRM_PRESSED,
                ),
                (
                    TacticalCommandControl::Cancel,
                    art::BTN_ASSIGN_CANCEL_NORMAL,
                    art::BTN_ASSIGN_CANCEL_PRESSED,
                ),
            ] {
                draw_tactical_assignment_button(
                    cache,
                    canvas,
                    control,
                    if pressed == Some(control) {
                        pressed_art
                    } else {
                        normal
                    },
                );
            }
        }
    }
}

fn draw_tactical_withdraw_confirmation(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
    pressed: Option<TacticalWithdrawConfirmationControl>,
) {
    use resources::tactical as art;

    draw_tactical_bitmap(cache, art::WITHDRAW_CONFIRMATION_PANEL, canvas, 482.0, 28.0);
    for spec in TACTICAL_WITHDRAW_CONFIRMATION_CONTROLS {
        draw_tactical_bitmap(
            cache,
            if pressed == Some(spec.control) {
                spec.pressed_resource
            } else {
                spec.normal_resource
            },
            canvas,
            spec.rect.x,
            spec.rect.y,
        );
    }

    // FUN_005f1050 places TEXTTACT 56772 at (20, 28) and TEXTTACT 56771
    // at (14, 92) in the 149x236 panel. Macroquad takes a text baseline, so
    // add the recovered ten-pixel font height while preserving those origins.
    let font_size = 10.0 * canvas.scale;
    let text_color = Color::new(0.84, 0.92, 0.84, 1.0);
    for (text, x, top) in [
        ("Withdraw Confirmation", 502.0, 56.0),
        ("Are you sure you wish to", 496.0, 120.0),
        ("order a fleet withdraw?", 496.0, 132.0),
    ] {
        let (screen_x, screen_y) = canvas.point(x, top + 10.0);
        draw_text(text, screen_x, screen_y, font_size, text_color);
    }
}

/// Paint only resource-backed battle chrome. Unmapped command groups remain
/// visible but are not passed off as implemented interactions.
#[expect(
    clippy::too_many_arguments,
    reason = "The original HUD paints one fixed canvas from explicit battle state."
)]
#[expect(
    clippy::too_many_lines,
    reason = "The authored tactical chrome is one fixed 640×480 source composition."
)]
fn draw_original_tactical_hud(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
    player_is_alliance: bool,
    groups: TacticalGroupHud<'_>,
    attack_death_star_enabled: bool,
    paused: bool,
    highlight_alliance: bool,
    highlight_empire: bool,
    navigation_sets_visible: [bool; 4],
    pressed_control: Option<TacticalHudControl>,
    command_panel: TacticalCommandPanel,
    pressed_command: Option<TacticalCommandControl>,
    battle_options_open: bool,
    withdraw_confirmation_open: bool,
    player_observing: bool,
    withdraw_enabled: bool,
    pressed_options: Option<TacticalBattleOptionsControl>,
    pressed_withdraw_confirmation: Option<TacticalWithdrawConfirmationControl>,
    death_star: Option<TacticalDeathStar>,
    death_star_laser_pressed: bool,
    death_star_targeting: bool,
) {
    use resources::tactical as art;

    if let Some(death_star) = death_star {
        draw_tactical_bitmap(
            cache,
            art::DEATH_STAR_LASER_GAUGE,
            canvas,
            DEATH_STAR_LASER_GAUGE_RECT.x,
            DEATH_STAR_LASER_GAUGE_RECT.y,
        );
        let fraction =
            (death_star.laser_charge / ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE).clamp(0.0, 1.0);
        if fraction > 0.0 {
            let (x, y) = canvas.point(DEATH_STAR_LASER_FILL_RECT.x, DEATH_STAR_LASER_FILL_RECT.y);
            draw_rectangle(
                x,
                y,
                DEATH_STAR_LASER_FILL_RECT.width * canvas.scale * fraction,
                DEATH_STAR_LASER_FILL_RECT.height * canvas.scale,
                Color::new(0.0, 0.82, 0.0, 1.0),
            );
        }
        let ready = death_star.operational()
            && death_star.laser_charge >= ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE;
        let active =
            death_star_laser_pressed || death_star_targeting || death_star.action_committed;
        draw_tactical_bitmap(
            cache,
            if active {
                art::DEATH_STAR_LASER_FIRED
            } else if ready {
                art::DEATH_STAR_LASER_READY
            } else {
                art::DEATH_STAR_LASER_LOADING
            },
            canvas,
            DEATH_STAR_LASER_BUTTON_RECT.x,
            DEATH_STAR_LASER_BUTTON_RECT.y,
        );
    }

    // The top task-force and fighter-group strips occupy slots in bitmap 1000.
    draw_tactical_bitmap(
        cache,
        if player_is_alliance {
            art::TASK_FORCES_ALLIANCE
        } else {
            art::TASK_FORCES_EMPIRE
        },
        canvas,
        TASK_FORCE_HEADER_POINT.0,
        TASK_FORCE_HEADER_POINT.1,
    );
    for index in 0..8_u8 {
        let assigned = groups.task_forces[usize::from(index)];
        let active = assigned
            && (groups.selected_task_force == Some(index)
                || pressed_control == Some(TacticalHudControl::TaskForce(index)));
        draw_tactical_bitmap(
            cache,
            if active {
                art::BTN_TASK_FORCE_PRESSED
            } else if assigned {
                art::BTN_TASK_FORCE_NORMAL
            } else {
                art::BTN_TASK_FORCE_UNASSIGNED
            },
            canvas,
            TASK_FORCE_BUTTON_X + f32::from(index) * GROUP_BUTTON_PITCH,
            2.0,
        );
        if assigned {
            let label = (index + 1).to_string();
            let font_size = 13.0 * canvas.scale;
            let width = measure_text(&label, None, font_size as u16, 1.0).width;
            let (button_x, y) = canvas.point(
                TASK_FORCE_BUTTON_X + f32::from(index) * GROUP_BUTTON_PITCH,
                16.0,
            );
            draw_text(
                &label,
                button_x + (25.0 * canvas.scale - width) * 0.5,
                y,
                font_size,
                BLACK,
            );
        }
    }
    draw_tactical_bitmap(
        cache,
        if player_is_alliance {
            art::FIGHTER_SQUADRONS_ALLIANCE
        } else {
            art::FIGHTER_SQUADRONS_EMPIRE
        },
        canvas,
        FIGHTER_GROUP_HEADER_POINT.0,
        FIGHTER_GROUP_HEADER_POINT.1,
    );
    for index in 0..4_u8 {
        let assigned = groups.fighter_groups[usize::from(index)];
        let active = assigned
            && (groups.selected_fighter_group == Some(index)
                || pressed_control == Some(TacticalHudControl::FighterGroup(index)));
        draw_original_bitmap(
            cache,
            DllSource::Tactical,
            if active {
                FIGHTER_GROUP_PRESSED_ART[usize::from(index)]
            } else if assigned {
                FIGHTER_GROUP_NORMAL_ART[usize::from(index)]
            } else {
                art::SQUADRON_UNASSIGNED
            },
            canvas,
            FIGHTER_GROUP_BUTTON_X + f32::from(index) * GROUP_BUTTON_PITCH,
            2.0,
            Some(Rect::new(0.0, 0.0, 25.0, FIGHTER_GROUP_BUTTON_HEIGHT)),
        );
    }

    let has_selection =
        groups.selected_fighter_group.is_some() || groups.selected_ship_name.is_some();
    if withdraw_confirmation_open {
        draw_tactical_withdraw_confirmation(cache, canvas, pressed_withdraw_confirmation);
    } else if battle_options_open {
        draw_tactical_bitmap(cache, art::BATTLE_OPTIONS_PANEL, canvas, 482.0, 28.0);
        for spec in TACTICAL_BATTLE_OPTIONS_CONTROLS {
            if spec.control == TacticalBattleOptionsControl::Withdraw && !withdraw_enabled {
                continue;
            }
            let selected =
                spec.control == TacticalBattleOptionsControl::ObserveToggle && player_observing;
            let pressed = pressed_options == Some(spec.control);
            let resource = if selected || pressed {
                spec.pressed_resource
            } else {
                spec.normal_resource
            };
            draw_tactical_bitmap(cache, resource, canvas, spec.rect.x, spec.rect.y);
        }
    } else if command_panel != TacticalCommandPanel::Display {
        draw_tactical_assignment_panel(
            cache,
            canvas,
            player_is_alliance,
            command_panel,
            groups.selected_fighter_group.is_some(),
            attack_death_star_enabled,
            pressed_command,
        );
    } else {
        let panel_resource = if groups.selected_fighter_group.is_some() {
            1307
        } else if groups.selected_ship_name.is_some() {
            TACTICAL_SELECTED_SHIP_PANEL
        } else if player_is_alliance {
            art::IDLE_PANEL_ALLIANCE
        } else {
            art::IDLE_PANEL_EMPIRE
        };
        draw_tactical_bitmap(cache, panel_resource, canvas, 482.0, 28.0);
    }
    if !withdraw_confirmation_open
        && !battle_options_open
        && command_panel == TacticalCommandPanel::Display
    {
        if let (Some(name), Some(group)) = (groups.selected_ship_name, groups.selected_task_force) {
            if let Some(resource) = groups.selected_ship_hud {
                draw_tactical_bitmap(
                    cache,
                    resource,
                    canvas,
                    TACTICAL_SELECTED_SHIP_ART_X,
                    TACTICAL_SELECTED_SHIP_ART_Y,
                );
            }
            if let Some((hull, maximum)) = groups.selected_ship_hull {
                draw_tactical_selected_meter(
                    canvas,
                    TACTICAL_SELECTED_HULL_METER,
                    tactical_meter_percent(hull, maximum),
                );
            }
            if let Some((shield, maximum)) = groups.selected_ship_shield {
                draw_tactical_selected_meter(
                    canvas,
                    TACTICAL_SELECTED_SHIELD_METER,
                    tactical_meter_percent(shield, maximum),
                );
            }
            if let Some(subsystems) = groups.selected_ship_subsystems {
                for (index, resource_id) in subsystems.resource_ids().into_iter().enumerate() {
                    draw_tactical_bitmap(
                        cache,
                        resource_id,
                        canvas,
                        TACTICAL_SUBSYSTEM_X[index],
                        TACTICAL_SUBSYSTEM_Y,
                    );
                }
            }
            // TACTICAL 1302 reserves the two blue apertures below the hull
            // portrait for the selected capital ship's personnel and troops.
            // GOKRES supplies the original compact 61x25 assignment images.
            for (index, content) in groups.selected_ship_contents.iter().take(2).enumerate() {
                draw_original_bitmap(
                    cache,
                    DllSource::Gokres,
                    content.resource_id,
                    canvas,
                    492.0 + index as f32 * 63.0,
                    156.0,
                    None,
                );
            }
            for ((x, y), text) in [
                (TACTICAL_SELECTED_NAME_POINT, name.to_owned()),
                (
                    TACTICAL_SELECTED_TASK_FORCE_POINT,
                    format!("Task Force # {}", group + 1),
                ),
                (
                    TACTICAL_SELECTED_ORDER_POINT,
                    groups.selected_order.label().to_owned(),
                ),
                (
                    TACTICAL_SELECTED_TACTIC_POINT,
                    format!("Tactics: {}", groups.selected_tactic.label()),
                ),
            ] {
                draw_result_text(canvas, &text, x, y, TACTICAL_SELECTED_TEXT_EM, WHITE);
            }
        } else if let Some(group) = groups.selected_fighter_group {
            // `FUN_005e16a0`: art first, then the labels (`FUN_005e22a0`
            // paints them over it), so the count line overlays the art.
            if let Some(panel) = groups.selected_fighter_panel {
                draw_tactical_bitmap(
                    cache,
                    panel.art,
                    canvas,
                    TACTICAL_FIGHTER_ART_X,
                    TACTICAL_FIGHTER_ART_Y,
                );
            }
            let labels = ["Red Group", "Blue Group", "Green Group", "Gold Group"];
            let mut lines = vec![
                (
                    TACTICAL_FIGHTER_NAME_POINT,
                    labels[usize::from(group.min(3))].to_owned(),
                ),
                (
                    TACTICAL_FIGHTER_ORDER_POINT,
                    groups.selected_order.label().to_owned(),
                ),
                (
                    TACTICAL_FIGHTER_TACTIC_POINT,
                    format!("Tactics: {}", groups.selected_tactic.label()),
                ),
            ];
            if let Some(panel) = groups.selected_fighter_panel {
                lines.push((
                    TACTICAL_FIGHTER_COUNT_POINT,
                    format!(
                        "{} {} {}/{}",
                        panel.type_label, panel.squadrons_of_type, panel.craft, panel.capacity
                    ),
                ));
            }
            for ((x, y), text) in lines {
                draw_result_text(canvas, &text, x, y, TACTICAL_SELECTED_TEXT_EM, WHITE);
            }
        }
    }
    if !withdraw_confirmation_open
        && !battle_options_open
        && command_panel == TacticalCommandPanel::Display
        && has_selection
    {
        if groups.selected_ship_name.is_some() {
            for (control, normal, pressed_art) in [
                (
                    TacticalCommandControl::PreviousCapital,
                    art::BTN_PREVIOUS_CAPITAL_NORMAL,
                    art::BTN_PREVIOUS_CAPITAL_PRESSED,
                ),
                (
                    TacticalCommandControl::NextCapital,
                    art::BTN_NEXT_CAPITAL_NORMAL,
                    art::BTN_NEXT_CAPITAL_PRESSED,
                ),
            ] {
                draw_tactical_assignment_button(
                    cache,
                    canvas,
                    control,
                    if pressed_command == Some(control) {
                        pressed_art
                    } else {
                        normal
                    },
                );
            }
        }
        for (control, normal, pressed_art) in [
            (
                TacticalCommandControl::OpenMissions,
                art::BTN_MISSIONS_NORMAL,
                art::BTN_MISSIONS_PRESSED,
            ),
            (
                TacticalCommandControl::OpenManeuvers,
                art::BTN_MANEUVERS_TACTICS_NORMAL,
                art::BTN_MANEUVERS_TACTICS_PRESSED,
            ),
        ] {
            draw_tactical_assignment_button(
                cache,
                canvas,
                control,
                if pressed_command == Some(control) {
                    pressed_art
                } else {
                    normal
                },
            );
        }
    }

    for (index, x) in [485.0, 521.0, 560.0, 601.0].into_iter().enumerate() {
        let selected = navigation_sets_visible[index]
            || pressed_control == Some(TacticalHudControl::NavigationSet(index as u8));
        draw_tactical_bitmap(
            cache,
            if selected {
                art::BTN_NAVIGATION_SET_SELECTED[index]
            } else {
                art::BTN_NAVIGATION_SET_NORMAL[index]
            },
            canvas,
            x,
            272.0,
        );
    }
    draw_tactical_bitmap(
        cache,
        if highlight_empire {
            art::HIGHLIGHT_EMPIRE_SHIPS
        } else {
            art::DIM_EMPIRE_SHIPS
        },
        canvas,
        482.0,
        304.0,
    );
    draw_tactical_bitmap(
        cache,
        if highlight_alliance {
            art::HIGHLIGHT_ALLIANCE_SHIPS
        } else {
            art::DIM_ALLIANCE_SHIPS
        },
        canvas,
        517.0,
        304.0,
    );
    draw_tactical_bitmap(
        cache,
        if paused {
            art::BTN_PAUSE_PAUSED
        } else {
            art::BTN_PAUSE_RUNNING
        },
        canvas,
        560.0,
        307.0,
    );
    draw_tactical_bitmap(
        cache,
        if pressed_control == Some(TacticalHudControl::BattleOptions) {
            art::BTN_BATTLE_OPTIONS_PRESSED
        } else {
            art::BTN_BATTLE_OPTIONS_NORMAL
        },
        canvas,
        606.0,
        308.0,
    );

    draw_tactical_bitmap(
        cache,
        if pressed_control == Some(TacticalHudControl::ZoomIn) {
            art::BTN_CAMERA_ZOOM_IN_PRESSED
        } else {
            art::BTN_CAMERA_ZOOM_IN_NORMAL
        },
        canvas,
        486.0,
        343.0,
    );
    draw_tactical_bitmap(
        cache,
        if pressed_control == Some(TacticalHudControl::ZoomOut) {
            art::BTN_CAMERA_ZOOM_OUT_PRESSED
        } else {
            art::BTN_CAMERA_ZOOM_OUT_NORMAL
        },
        canvas,
        603.0,
        343.0,
    );
    for (id, x, y) in [
        (
            if pressed_control == Some(TacticalHudControl::CameraLeft) {
                art::BTN_CAMERA_LEFT_PRESSED
            } else {
                art::BTN_CAMERA_LEFT_NORMAL
            },
            511.0,
            376.0,
        ),
        (
            if pressed_control == Some(TacticalHudControl::CameraRight) {
                art::BTN_CAMERA_RIGHT_PRESSED
            } else {
                art::BTN_CAMERA_RIGHT_NORMAL
            },
            557.0,
            376.0,
        ),
        (
            if pressed_control == Some(TacticalHudControl::CameraUp) {
                art::BTN_CAMERA_UP_PRESSED
            } else {
                art::BTN_CAMERA_UP_NORMAL
            },
            537.0,
            344.0,
        ),
        (
            if pressed_control == Some(TacticalHudControl::CameraDown) {
                art::BTN_CAMERA_DOWN_PRESSED
            } else {
                art::BTN_CAMERA_DOWN_NORMAL
            },
            537.0,
            412.0,
        ),
        (
            if pressed_control == Some(TacticalHudControl::CameraTarget) {
                art::BTN_CAMERA_TARGET_PRESSED
            } else {
                art::BTN_CAMERA_TARGET_NORMAL
            },
            538.0,
            379.0,
        ),
    ] {
        draw_tactical_bitmap(cache, id, canvas, x, y);
    }
    draw_tactical_bitmap(
        cache,
        if pressed_control == Some(TacticalHudControl::CameraRecall) {
            art::BTN_CAMERA_RECALL_PRESSED
        } else {
            art::BTN_CAMERA_RECALL_NORMAL
        },
        canvas,
        484.0,
        430.0,
    );
    draw_tactical_bitmap(
        cache,
        if pressed_control == Some(TacticalHudControl::CameraMemorize) {
            art::BTN_CAMERA_MEMORIZE_PRESSED
        } else {
            art::BTN_CAMERA_MEMORIZE_NORMAL
        },
        canvas,
        585.0,
        430.0,
    );
}

fn activate_tactical_command(state: &mut TacticalState, control: TacticalCommandControl) {
    match control {
        TacticalCommandControl::PreviousCapital | TacticalCommandControl::NextCapital => {
            let step = if control == TacticalCommandControl::PreviousCapital {
                -1
            } else {
                1
            };
            if let Some((from, to, group)) = state
                .session
                .as_mut()
                .and_then(|session| cycle_selected_capital(session, step))
            {
                macroquad::logging::info!(
                    "[tactical_selection] direction={} task_force={} from={} to={}",
                    if step < 0 { "previous" } else { "next" },
                    group + 1,
                    from,
                    to,
                );
            }
        }
        TacticalCommandControl::OpenManeuvers => {
            if let Some(session) = state.session.as_ref() {
                let (pending_order, pending_tactic) = selected_command_values(session);
                state.command_panel = TacticalCommandPanel::Maneuvers {
                    pending_order,
                    pending_tactic,
                };
            }
        }
        TacticalCommandControl::OpenMissions => {
            if let Some(session) = state.session.as_ref() {
                let (pending_order, _) = selected_command_values(session);
                state.command_panel = TacticalCommandPanel::Missions { pending_order };
            }
        }
        TacticalCommandControl::LeftHook => {
            if let TacticalCommandPanel::Maneuvers { pending_order, .. } = &mut state.command_panel
            {
                *pending_order = TacticalOrder::LeftHook;
            }
        }
        TacticalCommandControl::RightHook => {
            if let TacticalCommandPanel::Maneuvers { pending_order, .. } = &mut state.command_panel
            {
                *pending_order = TacticalOrder::RightHook;
            }
        }
        TacticalCommandControl::Hammer => {
            if let TacticalCommandPanel::Maneuvers { pending_order, .. } = &mut state.command_panel
            {
                *pending_order = TacticalOrder::Hammer;
            }
        }
        TacticalCommandControl::Anvil => {
            if let TacticalCommandPanel::Maneuvers { pending_order, .. } = &mut state.command_panel
            {
                *pending_order = TacticalOrder::Anvil;
            }
        }
        TacticalCommandControl::HoldPosition => {
            if let TacticalCommandPanel::Maneuvers { pending_order, .. } = &mut state.command_panel
            {
                *pending_order = TacticalOrder::HoldPosition;
            }
        }
        TacticalCommandControl::ToggleTactic => {
            if let TacticalCommandPanel::Maneuvers { pending_tactic, .. } = &mut state.command_panel
            {
                *pending_tactic = if *pending_tactic == TacticalTactic::StandOff {
                    TacticalTactic::Surround
                } else {
                    TacticalTactic::StandOff
                };
            }
        }
        TacticalCommandControl::AttackCapitalShips => {
            if let TacticalCommandPanel::Missions { pending_order } = &mut state.command_panel {
                *pending_order = TacticalOrder::AttackCapitalShips;
            }
        }
        TacticalCommandControl::AttackFighters => {
            if let TacticalCommandPanel::Missions { pending_order } = &mut state.command_panel {
                *pending_order = TacticalOrder::AttackFighters;
            }
        }
        TacticalCommandControl::Recover => {
            if let TacticalCommandPanel::Missions { pending_order } = &mut state.command_panel {
                *pending_order = TacticalOrder::Recover;
            }
        }
        TacticalCommandControl::AttackDeathStar => {
            if let TacticalCommandPanel::Missions { pending_order } = &mut state.command_panel {
                *pending_order = TacticalOrder::AttackDeathStar;
            }
        }
        TacticalCommandControl::Confirm => {
            let panel = state.command_panel;
            if let Some(session) = state.session.as_mut() {
                let (_, previous_tactic) = selected_command_values(session);
                let (kind, order, tactic) = match panel {
                    TacticalCommandPanel::Maneuvers {
                        pending_order,
                        pending_tactic,
                    } => ("maneuvers", pending_order, Some(pending_tactic)),
                    TacticalCommandPanel::Missions { pending_order } => {
                        ("missions", pending_order, None)
                    }
                    TacticalCommandPanel::Display => return,
                };
                let (capital_members, fighter_members) =
                    assign_selected_command(session, order, tactic);
                if capital_members + fighter_members > 0 {
                    queue_selected_command_voice(
                        session,
                        panel,
                        order,
                        tactic.is_some_and(|value| value != previous_tactic),
                    );
                }
                if order == TacticalOrder::AttackDeathStar {
                    macroquad::logging::info!(
                        "[tactical_death_star] trench_run_launch status={} fighter_members={} group={} remaining_ms={} rng_state={}",
                        if session.trench_run_pending_outcome.is_some() {
                            "launched"
                        } else {
                            "rejected"
                        },
                        fighter_members,
                        session.trench_run_group.map_or(4, |group| group),
                        session.trench_run_remaining_ms.unwrap_or(0.0),
                        session.tactical_rng.state(),
                    );
                }
                macroquad::logging::info!(
                    "[tactical_orders] panel={} event=commit order={} tactic={} capital_members={} fighter_members={}",
                    kind,
                    order.source_code(),
                    tactic.map_or(0, TacticalTactic::source_code),
                    capital_members,
                    fighter_members,
                );
            }
            state.command_panel = TacticalCommandPanel::Display;
        }
        TacticalCommandControl::Cancel => {
            macroquad::logging::info!("[tactical_orders] event=cancel");
            state.command_panel = TacticalCommandPanel::Display;
        }
    }
}

fn activate_tactical_battle_options(
    state: &mut TacticalState,
    control: TacticalBattleOptionsControl,
) -> TacticalAction {
    match control {
        TacticalBattleOptionsControl::Withdraw => {
            state.battle_options_open = false;
            state.battle_options_pressed = None;
            state.withdraw_confirmation_open = true;
            state.withdraw_confirmation_pressed = None;
            macroquad::logging::info!(
                "[tactical_options] command=withdraw status=confirmation_open panel=1310 controls=1113:1114:1115:1116 title=56772 body=56771"
            );
            TacticalAction::None
        }
        TacticalBattleOptionsControl::SimulateRemainder => {
            macroquad::logging::info!("[tactical_options] command=simulate_remainder");
            TacticalAction::AutoResolve
        }
        TacticalBattleOptionsControl::ObserveToggle => {
            state.player_observing = !state.player_observing;
            macroquad::logging::info!(
                "[tactical_options] command={} observing={}",
                if state.player_observing {
                    "observe"
                } else {
                    "take_command"
                },
                state.player_observing
            );
            TacticalAction::None
        }
        TacticalBattleOptionsControl::GameOptions => {
            macroquad::logging::info!("[tactical_options] command=game_options status=routed");
            TacticalAction::OpenGameOptions
        }
        TacticalBattleOptionsControl::Close => {
            state.battle_options_open = false;
            macroquad::logging::info!("[tactical_options] command=close");
            TacticalAction::None
        }
    }
}

fn activate_tactical_withdraw_confirmation(
    state: &mut TacticalState,
    control: TacticalWithdrawConfirmationControl,
) -> TacticalAction {
    state.withdraw_confirmation_open = false;
    state.withdraw_confirmation_pressed = None;
    match control {
        TacticalWithdrawConfirmationControl::Confirm => {
            let hyperdrive_warning = if let Some(session) = state.session.as_mut() {
                session.queue_withdrawal_started_voice();
                session.queue_withdrawal_hyperdrive_warning()
            } else {
                false
            };
            macroquad::logging::info!(
                "[tactical_options] command=withdraw status=withdrawal_started confirmation=accepted hyperdrive_warning={}",
                hyperdrive_warning,
            );
            TacticalAction::WithdrawFromBattle
        }
        TacticalWithdrawConfirmationControl::Cancel => {
            macroquad::logging::info!(
                "[tactical_options] command=withdraw status=confirmation_cancelled"
            );
            TacticalAction::None
        }
    }
}

fn update_tactical_camera_follow(state: &mut TacticalState) -> bool {
    let Some(follow) = state.camera_follow else {
        return false;
    };
    let Some(session) = state.session.as_ref() else {
        state.camera_follow = None;
        return false;
    };
    let (source_positions, fallback_positions, object_id): (Vec<_>, Vec<_>, u32) = match follow {
        TacticalCameraFollow::TaskForce(group) => {
            let members = session
                .ships
                .iter()
                .filter(|ship| {
                    ship.alive
                        && !ship.retreating
                        && ship.is_attacker == session.player_is_attacker
                        && ship.task_force == group
                })
                .collect::<Vec<_>>();
            (
                members.iter().map(|ship| ship.source_position).collect(),
                members.iter().map(|ship| (ship.x, ship.y)).collect(),
                10_001 + u32::from(group),
            )
        }
        TacticalCameraFollow::FighterGroup(group) => {
            let members = session
                .fighters
                .iter()
                .filter(|fighter| {
                    fighter.alive
                        && fighter.squad_count > 0
                        && fighter.is_attacker == session.player_is_attacker
                        && fighter.fighter_group == group
                })
                .collect::<Vec<_>>();
            (
                members
                    .iter()
                    .map(|fighter| fighter.source_position)
                    .collect(),
                members
                    .iter()
                    .map(|fighter| (fighter.x, fighter.y))
                    .collect(),
                10_101 + u32::from(group),
            )
        }
    };
    if source_positions.is_empty() {
        state.camera_follow = None;
        return false;
    }
    let count = source_positions.len() as f32;
    let source = TacticalWorldPosition {
        x: source_positions
            .iter()
            .map(|position| position.x)
            .sum::<f32>()
            / count,
        y: source_positions
            .iter()
            .map(|position| position.y)
            .sum::<f32>()
            / count,
        z: source_positions
            .iter()
            .map(|position| position.z)
            .sum::<f32>()
            / count,
    };
    let fallback_count = fallback_positions.len() as f32;
    let fallback_x = fallback_positions
        .iter()
        .map(|position| position.0)
        .sum::<f32>()
        / fallback_count;
    let fallback_y = fallback_positions
        .iter()
        .map(|position| position.1)
        .sum::<f32>()
        / fallback_count;
    state.camera_x = fallback_x - ARENA_WIDTH * 0.5;
    state.camera_y = fallback_y - ARENA_HEIGHT * 0.5;
    state
        .asset_renderer
        .focus_target(object_id, source.rendered());
    true
}

fn handle_original_tactical_controls(
    state: &mut TacticalState,
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
) -> TacticalAction {
    let (mouse_x, mouse_y) = mouse_position();
    let (x, y) = canvas.logical_pointer(mouse_x, mouse_y);
    if state.withdraw_confirmation_open {
        let hovered = tactical_withdraw_confirmation_control_at(cache, x, y);
        if is_mouse_button_pressed(MouseButton::Left) {
            state.withdraw_confirmation_pressed = hovered;
        }
        if is_mouse_button_released(MouseButton::Left) {
            let captured = state.withdraw_confirmation_pressed.take();
            if let Some(captured) = captured.filter(|captured| Some(*captured) == hovered) {
                return activate_tactical_withdraw_confirmation(state, captured);
            }
        } else if !is_mouse_button_down(MouseButton::Left) {
            state.withdraw_confirmation_pressed = None;
        }
        return TacticalAction::None;
    }
    let death_star_ready = state
        .session
        .as_ref()
        .is_some_and(tactical_death_star_laser_ready);
    if state.death_star_targeting && !death_star_ready {
        state.death_star_targeting = false;
    }
    let death_star_hovered = death_star_ready && tactical_death_star_laser_control_at(cache, x, y);
    if is_mouse_button_pressed(MouseButton::Left) && death_star_hovered {
        state.death_star_laser_pressed = true;
        return TacticalAction::None;
    }
    if is_mouse_button_released(MouseButton::Left) && state.death_star_laser_pressed {
        state.death_star_laser_pressed = false;
        if death_star_hovered {
            state.death_star_targeting = true;
            macroquad::logging::info!(
                "[tactical_death_star] superlaser_targeting status=armed input=right_click"
            );
        }
        return TacticalAction::None;
    }
    if !is_mouse_button_down(MouseButton::Left) {
        state.death_star_laser_pressed = false;
    }
    if state.battle_options_open {
        let withdraw_enabled = state
            .session
            .as_ref()
            .is_some_and(tactical_withdraw_enabled);
        let hovered = tactical_battle_options_control_at(cache, withdraw_enabled, x, y);
        if is_mouse_button_pressed(MouseButton::Left) {
            state.battle_options_pressed = hovered;
        }
        if is_mouse_button_released(MouseButton::Left) {
            let captured = state.battle_options_pressed.take();
            if let Some(captured) = captured.filter(|captured| Some(*captured) == hovered) {
                return activate_tactical_battle_options(state, captured);
            }
        } else if !is_mouse_button_down(MouseButton::Left) {
            state.battle_options_pressed = None;
        }
        return TacticalAction::None;
    }
    if is_mouse_button_pressed(MouseButton::Left) {
        let (has_selection, selected_fighter, attack_death_star_enabled, player_is_alliance) =
            state
                .session
                .as_ref()
                .map_or((false, false, false, true), |session| {
                    let selected_fighter = session.selected_fighter_group.is_some();
                    let attack_death_star_enabled = selected_fighter
                        && session.death_star.is_some_and(|death_star| {
                            death_star.operational()
                                && death_star.is_attacker != session.player_is_attacker
                        });
                    (
                        selected_fighter
                            || session.ships.iter().any(|ship| {
                                ship.alive
                                    && ship.selected
                                    && ship.is_attacker == session.player_is_attacker
                            }),
                        selected_fighter,
                        attack_death_star_enabled,
                        session.player_is_attacker == session.attacker_is_alliance,
                    )
                });
        let command_control = if state.player_observing {
            None
        } else {
            tactical_command_control_at(
                cache,
                state.command_panel,
                has_selection,
                selected_fighter,
                attack_death_star_enabled,
                player_is_alliance,
                x,
                y,
            )
        };
        if let Some(control) = command_control {
            activate_tactical_command(state, control);
        } else {
            match tactical_hud_control_at(cache, x, y) {
                Some(TacticalHudControl::NavigationSet(set)) => {
                    if let Some(visible) = state.navigation_sets_visible.get_mut(usize::from(set)) {
                        *visible = !*visible;
                        state.logged_navigation_scene = None;
                        macroquad::logging::info!(
                            "[tactical_navigation] event=set_visibility set={} visible={}",
                            set + 1,
                            *visible,
                        );
                    }
                }
                Some(TacticalHudControl::TaskForce(group)) => {
                    if let Some(session) = state.session.as_mut() {
                        let control =
                            is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);
                        let changed = if control {
                            assign_selected_to_task_force(session, group)
                        } else {
                            select_task_force(session, group)
                        };
                        if changed > 0 {
                            state.command_panel = TacticalCommandPanel::Display;
                            macroquad::logging::info!(
                                "[tactical_groups] task_force_{} slot={} members={}",
                                if control { "assigned" } else { "selected" },
                                group + 1,
                                changed,
                            );
                        }
                    }
                }
                Some(TacticalHudControl::FighterGroup(group)) => {
                    if let Some(session) = state.session.as_mut() {
                        if select_fighter_group(session, group) {
                            state.command_panel = TacticalCommandPanel::Display;
                            let members = session
                                .fighters
                                .iter()
                                .filter(|fighter| fighter.selected)
                                .count();
                            macroquad::logging::info!(
                                "[tactical_groups] fighter_group_selected slot={} members={}",
                                group + 1,
                                members,
                            );
                        }
                    }
                }
                Some(TacticalHudControl::Pause) => return TacticalAction::TogglePause,
                Some(TacticalHudControl::ZoomIn) => {
                    state.camera_follow = None;
                    state.zoom = (state.zoom * 1.25).min(2.0);
                    state.asset_renderer.zoom_in();
                }
                Some(TacticalHudControl::ZoomOut) => {
                    state.camera_follow = None;
                    state.zoom = (state.zoom / 1.25).max(0.5);
                    state.asset_renderer.zoom_out();
                }
                Some(TacticalHudControl::CameraTarget) => {
                    state.camera_follow = None;
                    let selected = state.session.as_ref().and_then(|session| {
                        session.selected_ship.and_then(|index| {
                            session.ships.get(index).map(|ship| {
                                (index, ship.x, ship.y, ship.source_position.rendered())
                            })
                        })
                    });
                    if let Some((_index, x, y, _source_position)) = selected {
                        (state.camera_x, state.camera_y) = camera_offset_for_target(x, y);
                        state.asset_renderer.focus_target(
                            u32::try_from(_index).unwrap_or(u32::MAX).saturating_add(1),
                            _source_position,
                        );
                    }
                }
                Some(TacticalHudControl::CameraLeft) => {
                    if state.camera_follow.take().is_some() {
                        macroquad::logging::info!(
                            "[tactical_camera] command=follow_exit reason=manual"
                        );
                    }
                    state.asset_renderer.turn_left();
                }
                Some(TacticalHudControl::CameraRight) => {
                    state.camera_follow = None;
                    state.asset_renderer.turn_right();
                }
                Some(TacticalHudControl::CameraUp) => {
                    state.camera_follow = None;
                    state.asset_renderer.pitch_up();
                }
                Some(TacticalHudControl::CameraDown) => {
                    state.camera_follow = None;
                    state.asset_renderer.pitch_down();
                }
                Some(TacticalHudControl::CameraMemorize) => {
                    state.memorized_camera =
                        state
                            .asset_renderer
                            .camera_state()
                            .map(|source| TacticalCameraBookmark {
                                source,
                                camera_x: state.camera_x,
                                camera_y: state.camera_y,
                                zoom: state.zoom,
                            });
                    macroquad::logging::info!(
                        "[tactical_camera] command=memorize stored={}",
                        state.memorized_camera.is_some(),
                    );
                }
                Some(TacticalHudControl::CameraRecall) => {
                    if let Some(bookmark) = state.memorized_camera.or(state.default_camera) {
                        state.asset_renderer.restore_camera_state(bookmark.source);
                        state.camera_x = bookmark.camera_x;
                        state.camera_y = bookmark.camera_y;
                        state.zoom = bookmark.zoom;
                        state.camera_follow = None;
                        macroquad::logging::info!(
                            "[tactical_camera] command=recall source={}",
                            if state.memorized_camera.is_some() {
                                "memorized"
                            } else {
                                "default"
                            },
                        );
                    }
                }
                Some(TacticalHudControl::HighlightAlliance) => {
                    state.highlight_alliance = !state.highlight_alliance;
                }
                Some(TacticalHudControl::HighlightEmpire) => {
                    state.highlight_empire = !state.highlight_empire;
                }
                Some(TacticalHudControl::BattleOptions) => {
                    state.battle_options_open = true;
                    state.withdraw_confirmation_open = false;
                    state.withdraw_confirmation_pressed = None;
                    state.command_panel = TacticalCommandPanel::Display;
                    macroquad::logging::info!("[tactical_options] command=open");
                }
                None => {}
            }
        }
    }
    let task_force_keys = [
        KeyCode::F1,
        KeyCode::F2,
        KeyCode::F3,
        KeyCode::F4,
        KeyCode::F5,
        KeyCode::F6,
        KeyCode::F7,
        KeyCode::F8,
    ];
    for (index, key) in task_force_keys.iter().enumerate() {
        if is_key_pressed(*key) {
            if let Some(session) = state.session.as_mut() {
                if select_task_force(session, u8::try_from(index).unwrap_or(7)) > 0 {
                    state.command_panel = TacticalCommandPanel::Display;
                    let group = u8::try_from(index).unwrap_or(7);
                    state.camera_follow = Some(TacticalCameraFollow::TaskForce(group));
                    macroquad::logging::info!(
                        "[tactical_camera] command=follow kind=task_force group={}",
                        group + 1,
                    );
                }
            }
        }
    }
    let fighter_group_keys = [KeyCode::F9, KeyCode::F10, KeyCode::F11, KeyCode::F12];
    for (index, key) in fighter_group_keys.iter().enumerate() {
        if is_key_pressed(*key) {
            if let Some(session) = state.session.as_mut() {
                if select_fighter_group(session, u8::try_from(index).unwrap_or(3)) {
                    state.command_panel = TacticalCommandPanel::Display;
                    let group = u8::try_from(index).unwrap_or(3);
                    state.camera_follow = Some(TacticalCameraFollow::FighterGroup(group));
                    macroquad::logging::info!(
                        "[tactical_camera] command=follow kind=fighter_group group={}",
                        group + 1,
                    );
                }
            }
        }
    }
    if is_key_pressed(KeyCode::Equal) {
        state.camera_follow = None;
        state.zoom = (state.zoom * 1.25).min(2.0);
        state.asset_renderer.zoom_in();
    }
    if is_key_pressed(KeyCode::Minus) {
        state.camera_follow = None;
        state.zoom = (state.zoom / 1.25).max(0.5);
        state.asset_renderer.zoom_out();
    }
    if is_key_pressed(KeyCode::Enter) {
        state.camera_follow = None;
        let selected = state.session.as_ref().and_then(|session| {
            session.selected_ship.and_then(|index| {
                session.ships.get(index).map(|ship| {
                    (
                        u32::try_from(index).unwrap_or(u32::MAX).saturating_add(1),
                        ship.x,
                        ship.y,
                        ship.source_position.rendered(),
                    )
                })
            })
        });
        if let Some((object_id, x, y, source)) = selected {
            (state.camera_x, state.camera_y) = camera_offset_for_target(x, y);
            state.asset_renderer.focus_target(object_id, source);
        }
    }
    TacticalAction::None
}

fn camera_offset_for_target(x: f32, y: f32) -> (f32, f32) {
    (x - ARENA_WIDTH * 0.5, y - ARENA_HEIGHT * 0.5)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BattleResultCondition {
    Operational,
    Damaged,
    Destroyed,
}

#[derive(Debug, Clone)]
struct BattleResultEntry {
    name: String,
    condition: BattleResultCondition,
}

#[derive(Debug, Clone, Copy)]
struct BattleResultSkin {
    frame: u32,
    rail: u32,
    close: [u32; 2],
    tabs: [[u32; 2]; 4],
    close_rect: NativeRect,
    tab_rects: [NativeRect; 4],
}

#[derive(Debug, Clone, Copy)]
struct BattleAlertSkin {
    frame: u32,
    scene: u32,
    /// The list pages' picture (layer 0x33, `FUN_0044f860`).
    list_scene: u32,
    rail: u32,
    tabs: [[u32; 2]; 4],
    tab_rects: [NativeRect; 4],
    buttons: [[u32; 3]; 3],
}

fn battle_alert_skin(player_is_alliance: bool) -> BattleAlertSkin {
    use resources::strategy as art;
    if player_is_alliance {
        BattleAlertSkin {
            frame: art::BATTLE_ALERT_WINDOW_ALLIANCE,
            scene: art::BATTLE_ALERT_SCENE_ALLIANCE,
            list_scene: art::BATTLE_ALERT_SCENE_ALLIANCE_DISABLED,
            rail: art::BATTLE_RAIL_ALLIANCE,
            tabs: [
                [
                    art::BATTLE_SUMMARY_ALLIANCE_NORMAL,
                    art::BATTLE_SUMMARY_ALLIANCE_PRESSED,
                ],
                [
                    art::BATTLE_ALLIANCE_FORCES_ALLIANCE_NORMAL,
                    art::BATTLE_ALLIANCE_FORCES_ALLIANCE_PRESSED,
                ],
                [
                    art::BATTLE_EMPIRE_FORCES_ALLIANCE_NORMAL,
                    art::BATTLE_EMPIRE_FORCES_ALLIANCE_PRESSED,
                ],
                [
                    art::BATTLE_SYSTEM_ALLIANCE_NORMAL,
                    art::BATTLE_SYSTEM_ALLIANCE_PRESSED,
                ],
            ],
            tab_rects: [
                NativeRect::new(418.0, 21.0, 41.0, 41.0),
                NativeRect::new(418.0, 81.0, 41.0, 41.0),
                NativeRect::new(418.0, 143.0, 41.0, 41.0),
                NativeRect::new(418.0, 205.0, 41.0, 41.0),
            ],
            buttons: [
                [
                    art::BATTLE_ALERT_RETREAT_ALLIANCE_NORMAL,
                    art::BATTLE_ALERT_RETREAT_ALLIANCE_PRESSED,
                    art::BATTLE_ALERT_RETREAT_ALLIANCE_DISABLED,
                ],
                [
                    art::BATTLE_ALERT_SIMULATE_ALLIANCE_NORMAL,
                    art::BATTLE_ALERT_SIMULATE_ALLIANCE_PRESSED,
                    art::BATTLE_ALERT_SIMULATE_ALLIANCE_DISABLED,
                ],
                [
                    art::BATTLE_ALERT_COMMAND_ALLIANCE_NORMAL,
                    art::BATTLE_ALERT_COMMAND_ALLIANCE_PRESSED,
                    art::BATTLE_ALERT_COMMAND_ALLIANCE_DISABLED,
                ],
            ],
        }
    } else {
        BattleAlertSkin {
            frame: art::BATTLE_ALERT_WINDOW_EMPIRE,
            scene: art::BATTLE_ALERT_SCENE_EMPIRE,
            list_scene: art::BATTLE_ALERT_SCENE_EMPIRE_DISABLED,
            rail: art::BATTLE_RAIL_EMPIRE,
            tabs: [
                [
                    art::BATTLE_SUMMARY_EMPIRE_NORMAL,
                    art::BATTLE_SUMMARY_EMPIRE_PRESSED,
                ],
                [
                    art::BATTLE_ALLIANCE_FORCES_EMPIRE_NORMAL,
                    art::BATTLE_ALLIANCE_FORCES_EMPIRE_PRESSED,
                ],
                [
                    art::BATTLE_EMPIRE_FORCES_EMPIRE_NORMAL,
                    art::BATTLE_EMPIRE_FORCES_EMPIRE_PRESSED,
                ],
                [
                    art::BATTLE_SYSTEM_EMPIRE_NORMAL,
                    art::BATTLE_SYSTEM_EMPIRE_PRESSED,
                ],
            ],
            tab_rects: [
                NativeRect::new(426.0, 17.0, 44.0, 41.0),
                NativeRect::new(426.0, 80.0, 44.0, 41.0),
                NativeRect::new(426.0, 143.0, 44.0, 41.0),
                NativeRect::new(426.0, 206.0, 44.0, 41.0),
            ],
            buttons: [
                [
                    art::BATTLE_ALERT_RETREAT_EMPIRE_NORMAL,
                    art::BATTLE_ALERT_RETREAT_EMPIRE_PRESSED,
                    art::BATTLE_ALERT_RETREAT_EMPIRE_DISABLED,
                ],
                [
                    art::BATTLE_ALERT_SIMULATE_EMPIRE_NORMAL,
                    art::BATTLE_ALERT_SIMULATE_EMPIRE_PRESSED,
                    art::BATTLE_ALERT_SIMULATE_EMPIRE_DISABLED,
                ],
                [
                    art::BATTLE_ALERT_COMMAND_EMPIRE_NORMAL,
                    art::BATTLE_ALERT_COMMAND_EMPIRE_PRESSED,
                    art::BATTLE_ALERT_COMMAND_EMPIRE_DISABLED,
                ],
            ],
        }
    }
}

fn battle_result_skin(player_is_alliance: bool) -> BattleResultSkin {
    use resources::strategy as art;
    if player_is_alliance {
        BattleResultSkin {
            frame: art::BATTLE_WINDOW_ALLIANCE,
            rail: art::BATTLE_RAIL_ALLIANCE,
            close: [
                art::BATTLE_CLOSE_ALLIANCE_NORMAL,
                art::BATTLE_CLOSE_ALLIANCE_PRESSED,
            ],
            tabs: [
                [
                    art::BATTLE_SUMMARY_ALLIANCE_NORMAL,
                    art::BATTLE_SUMMARY_ALLIANCE_PRESSED,
                ],
                [
                    art::BATTLE_ALLIANCE_FORCES_ALLIANCE_NORMAL,
                    art::BATTLE_ALLIANCE_FORCES_ALLIANCE_PRESSED,
                ],
                [
                    art::BATTLE_EMPIRE_FORCES_ALLIANCE_NORMAL,
                    art::BATTLE_EMPIRE_FORCES_ALLIANCE_PRESSED,
                ],
                [
                    art::BATTLE_DESTINATION_ALLIANCE_NORMAL,
                    art::BATTLE_DESTINATION_ALLIANCE_PRESSED,
                ],
            ],
            close_rect: NativeRect::new(423.0, 25.0, 32.0, 31.0),
            tab_rects: [
                NativeRect::new(418.0, 88.0, 41.0, 41.0),
                NativeRect::new(418.0, 142.0, 41.0, 41.0),
                NativeRect::new(418.0, 196.0, 41.0, 41.0),
                NativeRect::new(418.0, 250.0, 41.0, 41.0),
            ],
        }
    } else {
        BattleResultSkin {
            frame: art::BATTLE_WINDOW_EMPIRE,
            rail: art::BATTLE_RAIL_EMPIRE,
            close: [
                art::BATTLE_CLOSE_EMPIRE_NORMAL,
                art::BATTLE_CLOSE_EMPIRE_PRESSED,
            ],
            tabs: [
                [
                    art::BATTLE_SUMMARY_EMPIRE_NORMAL,
                    art::BATTLE_SUMMARY_EMPIRE_PRESSED,
                ],
                [
                    art::BATTLE_ALLIANCE_FORCES_EMPIRE_NORMAL,
                    art::BATTLE_ALLIANCE_FORCES_EMPIRE_PRESSED,
                ],
                [
                    art::BATTLE_EMPIRE_FORCES_EMPIRE_NORMAL,
                    art::BATTLE_EMPIRE_FORCES_EMPIRE_PRESSED,
                ],
                [
                    art::BATTLE_DESTINATION_EMPIRE_NORMAL,
                    art::BATTLE_DESTINATION_EMPIRE_PRESSED,
                ],
            ],
            close_rect: NativeRect::new(426.0, 17.0, 44.0, 41.0),
            tab_rects: [
                NativeRect::new(426.0, 89.0, 44.0, 41.0),
                NativeRect::new(426.0, 148.0, 44.0, 41.0),
                NativeRect::new(426.0, 207.0, 44.0, 41.0),
                NativeRect::new(426.0, 266.0, 44.0, 41.0),
            ],
        }
    }
}

impl BattleResultTab {
    const fn index(self) -> usize {
        match self {
            Self::Summary => 0,
            Self::AllianceForces => 1,
            Self::ImperialForces => 2,
            Self::GoDirectlyTo => 3,
        }
    }

    const fn from_index(index: usize) -> Self {
        match index {
            1 => Self::AllianceForces,
            2 => Self::ImperialForces,
            3 => Self::GoDirectlyTo,
            _ => Self::Summary,
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Summary => "summary",
            Self::AllianceForces => "alliance_forces",
            Self::ImperialForces => "imperial_forces",
            Self::GoDirectlyTo => "go_directly_to",
        }
    }
}

impl BattleResultCategory {
    const ALL: [Self; 4] = [
        Self::CapitalShips,
        Self::Fighters,
        Self::Troops,
        Self::Personnel,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::CapitalShips => "Capital Ships",
            Self::Fighters => "Fighters",
            Self::Troops => "Troops",
            Self::Personnel => "Personnel",
        }
    }
}

fn player_is_alliance(session: &BattleSession) -> bool {
    session.player_is_attacker == session.attacker_is_alliance
}

fn winner_is_alliance(session: &BattleSession) -> Option<bool> {
    match session.winner {
        Some(CombatWinner::Attacker) => Some(session.attacker_is_alliance),
        Some(CombatWinner::Defender) => Some(!session.attacker_is_alliance),
        Some(CombatWinner::Draw) | None => None,
    }
}

fn winner_fleet(session: &BattleSession) -> Option<FleetKey> {
    match session.winner {
        Some(CombatWinner::Attacker) => session.attacker_fleet,
        Some(CombatWinner::Defender) => session.defender_fleet,
        Some(CombatWinner::Draw) | None => None,
    }
}

fn result_fleet_for_faction(session: &BattleSession, alliance: bool) -> Option<FleetKey> {
    if session.attacker_is_alliance == alliance {
        session.attacker_fleet
    } else {
        session.defender_fleet
    }
}

fn battle_result_entries(
    session: &BattleSession,
    world: &GameWorld,
    troop_transport: &TroopTransportState,
    alliance: bool,
    category: BattleResultCategory,
) -> Vec<BattleResultEntry> {
    match category {
        BattleResultCategory::CapitalShips => {
            let mut entries = session
                .ships
                .iter()
                .filter(|ship| ship.identity.is_alliance == alliance)
                .map(|ship| BattleResultEntry {
                    name: ship.name.clone(),
                    condition: if !ship.alive || ship.hull_current <= 0 {
                        BattleResultCondition::Destroyed
                    } else if ship.hull_current < ship.hull_max
                        || ship.subsystem_damage.total_hits() > 0
                    {
                        BattleResultCondition::Damaged
                    } else {
                        BattleResultCondition::Operational
                    },
                })
                .collect::<Vec<_>>();
            if let Some(death_star) = session
                .death_star
                .filter(|value| value.is_alliance == alliance)
            {
                entries.push(BattleResultEntry {
                    name: "Death Star".into(),
                    condition: if death_star.destroyed || death_star.hull <= 0.0 {
                        BattleResultCondition::Destroyed
                    } else if death_star.hull < ORIGINAL_DEATH_STAR_LASER_FULL_CHARGE {
                        BattleResultCondition::Damaged
                    } else {
                        BattleResultCondition::Operational
                    },
                });
            }
            entries
        }
        BattleResultCategory::Fighters => session
            .fighters
            .iter()
            .filter(|fighter| fighter.identity.is_alliance == alliance)
            .map(|fighter| BattleResultEntry {
                name: fighter.name.clone(),
                condition: if !fighter.alive || fighter.squad_count == 0 {
                    BattleResultCondition::Destroyed
                } else if fighter.hull_current < fighter.hull_max {
                    BattleResultCondition::Damaged
                } else {
                    BattleResultCondition::Operational
                },
            })
            .collect(),
        BattleResultCategory::Troops => {
            let fleet_key = result_fleet_for_faction(session, alliance);
            let surviving_capacity = session
                .ships
                .iter()
                .filter(|ship| ship.identity.is_alliance == alliance && ship.alive)
                .filter_map(|ship| world.capital_ship_classes.get(ship.class_key))
                .map(|class| class.troop_capacity)
                .fold(0_u32, u32::saturating_add) as usize;
            fleet_key
                .map_or(&[][..], |fleet| troop_transport.cargo(fleet))
                .iter()
                .enumerate()
                .map(|(index, troop)| {
                    let strength = world
                        .troops
                        .get(*troop)
                        .map_or(0, |unit| unit.regiment_strength);
                    BattleResultEntry {
                        name: format!("Regiment {}", index + 1),
                        condition: if index >= surviving_capacity || strength <= 0 {
                            BattleResultCondition::Destroyed
                        } else if strength < 100 {
                            BattleResultCondition::Damaged
                        } else {
                            BattleResultCondition::Operational
                        },
                    }
                })
                .collect()
        }
        BattleResultCategory::Personnel => {
            let fleet_key = result_fleet_for_faction(session, alliance);
            fleet_key
                .and_then(|fleet| world.fleets.get(fleet))
                .into_iter()
                .flat_map(|fleet| fleet.characters.iter())
                .filter_map(|character| world.characters.get(*character))
                .map(|character| BattleResultEntry {
                    name: character.name.clone(),
                    condition: BattleResultCondition::Operational,
                })
                .collect()
        }
    }
}

thread_local! {
    /// The bold face of the Battle Alert's fleet and ship rows (font 5).
    static RESULT_BOLD_FONT: Option<Font> = load_ttf_font_from_bytes(include_bytes!(
        "../../../assets/fonts/LiberationSans-Bold.ttf"
    ))
    .ok();
}

thread_local! {
    /// The face the Battle Alert and result screens draw their text in.
    /// port: the original's Windows font is Arial (Wine capture of "Battle
    /// at Coruscant"); Liberation Sans shares its metrics.
    static RESULT_FONT: Option<Font> = load_ttf_font_from_bytes(include_bytes!(
        "../../../assets/fonts/LiberationSans-Regular.ttf"
    ))
    .ok();
}

/// The result text face, if it loaded.
fn result_font() -> Option<Font> {
    RESULT_FONT.with(Clone::clone)
}

/// The width of `text` in the result face at `font_size` screen pixels.
fn result_text_width(text: &str, font: Option<&Font>, font_size: u16) -> f32 {
    measure_text(text, font, font_size, 1.0).width
}

fn draw_result_text(
    canvas: TacticalCanvas,
    text: &str,
    x: f32,
    baseline_y: f32,
    size: f32,
    color: Color,
) {
    let (screen_x, screen_y) = canvas.point(x, baseline_y);
    let font = result_font();
    draw_text_ex(
        text,
        screen_x,
        screen_y,
        TextParams {
            font: font.as_ref(),
            font_size: (size * canvas.scale).max(1.0) as u16,
            color,
            ..TextParams::default()
        },
    );
}

fn draw_result_text_centered(
    canvas: TacticalCanvas,
    text: &str,
    center_x: f32,
    baseline_y: f32,
    size: f32,
    color: Color,
) {
    let font = result_font();
    let font_size = (size * canvas.scale).max(1.0) as u16;
    let width = result_text_width(text, font.as_ref(), font_size);
    let (screen_x, screen_y) = canvas.point(center_x, baseline_y);
    draw_text_ex(
        text,
        screen_x - width * 0.5,
        screen_y,
        TextParams {
            font: font.as_ref(),
            font_size,
            color,
            ..TextParams::default()
        },
    );
}

/// Lines wrap at `max_width` and advance 1.35 sizes, the spacing of the
/// Wine capture's body text.
fn draw_result_wrapped_lines(
    canvas: TacticalCanvas,
    lines: &[String],
    x: f32,
    mut baseline_y: f32,
    max_width: f32,
    size: f32,
    color: Color,
) {
    let font = result_font();
    let font_size = (size * canvas.scale).max(1.0) as u16;
    let advance = size * 1.35;
    for line in lines {
        let mut current = String::new();
        for word in line.split_whitespace() {
            let candidate = if current.is_empty() {
                word.to_owned()
            } else {
                format!("{current} {word}")
            };
            if !current.is_empty()
                && result_text_width(&candidate, font.as_ref(), font_size)
                    > max_width * canvas.scale
            {
                draw_result_text(canvas, &current, x, baseline_y, size, color);
                baseline_y += advance;
                current = word.to_owned();
            } else {
                current = candidate;
            }
        }
        if !current.is_empty() {
            draw_result_text(canvas, &current, x, baseline_y, size, color);
            baseline_y += advance;
        }
    }
}

/// The Battle Results text, as `FUN_0049b0f0`'s three builders write it:
/// `FUN_0049b330` when the player won, `FUN_0049b4d0` when the player lost,
/// `FUN_0049b150` for no victor (TEXTSTRA `0x7002`…`0x7016`,
/// `ghidra/notes/battle-result-text-and-cadence.md`). The system line
/// compares the holder with the player's side (`+0x58`) and with the
/// holder of a blockade already in place (`+0x5c`).
/// port: a played battle's results show before the losing fleet's
/// withdrawal is sent, so its destination (`0x700d`) is named only after
/// Simulate; otherwise `0x700c` stands in.
fn battle_result_summary_lines(session: &BattleSession) -> Vec<String> {
    let adjective = |alliance: bool| if alliance { "Alliance" } else { "Imperial" };
    let system = session.system_name.as_str();
    let player_alliance = session.player_is_attacker == session.attacker_is_alliance;
    let withdrew = |alliance: bool| {
        let side_is_attacker = session.attacker_is_alliance == alliance;
        session
            .ships
            .iter()
            .any(|ship| ship.is_attacker == side_is_attacker && ship.retreated)
    };
    let fate = |alliance: bool| {
        if !withdrew(alliance) {
            format!(
                "The {} fleet has been completely destroyed.",
                adjective(alliance)
            )
        } else if let Some(destination) = session
            .withdrawal_destination
            .as_ref()
            .filter(|_| alliance == player_alliance)
        {
            format!(
                "The {} fleet has withdrawn to {destination}.",
                adjective(alliance)
            )
        } else {
            format!("The {} fleet has withdrawn.", adjective(alliance))
        }
    };
    let blockaded_holder = session
        .holder_is_alliance
        .filter(|_| session.blockaded_at_start);
    let Some(winner_alliance) = winner_is_alliance(session) else {
        let (player, enemy) = (player_alliance, !player_alliance);
        let mut lines = vec![
            format!("The battle at {system} is indecisive."),
            "There has been no victor.".to_owned(),
        ];
        if !withdrew(player) && !withdrew(enemy) {
            lines.push(format!(
                "All {} and {} ships have been destroyed.",
                adjective(true),
                adjective(false)
            ));
        } else {
            lines.push(fate(player));
            lines.push(fate(enemy));
        }
        return lines;
    };
    let loser_alliance = !winner_alliance;
    let (winner, loser) = (adjective(winner_alliance), adjective(loser_alliance));
    if winner_alliance == player_alliance {
        let status = match session.holder_is_alliance {
            None => format!("{system} has been cleared of {loser} forces."),
            Some(holder) if blockaded_holder == Some(holder) => {
                if holder == player_alliance {
                    format!("{winner} forces have broken the blockade of {system}.")
                } else {
                    format!("{winner} forces have maintained the blockade of {system}.")
                }
            }
            Some(holder) if holder == player_alliance => {
                format!("{system} has been successfully defended from {loser} forces.")
            }
            Some(_) => format!("{system} is now under blockade by {winner} forces."),
        };
        vec![
            format!("The {winner} fleet is victorious."),
            status,
            fate(loser_alliance),
        ]
    } else {
        let player = adjective(player_alliance);
        let status = match session.holder_is_alliance {
            None => format!("The {winner} fleet was victorious at the neutral world {system}."),
            Some(holder) if blockaded_holder == Some(holder) => {
                if holder == player_alliance {
                    format!("{winner} forces have maintained the blockade of {system}.")
                } else {
                    format!("{winner} forces have broken the blockade of {system}.")
                }
            }
            Some(holder) if holder == player_alliance => {
                format!("{system} is now under blockade by {winner} forces.")
            }
            Some(_) => format!("The {player} attack on {system} has failed."),
        };
        vec![
            format!("The {player} fleet is defeated."),
            status,
            fate(player_alliance),
        ]
    }
}

fn draw_battle_result_summary(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
    session: &BattleSession,
    text_color: Color,
) {
    use resources::strategy as art;
    let scene = match winner_is_alliance(session) {
        Some(true) => art::EVENT_BATTLE_ALLIANCE_VICTORY,
        Some(false) => art::EVENT_BATTLE_EMPIRE_VICTORY,
        None => art::EVENT_BATTLE_MUTUAL_DESTRUCTION,
    };
    draw_strategy_bitmap(
        cache,
        scene,
        canvas,
        BATTLE_RESULTS_WINDOW_X + BATTLE_RESULTS_SCENE_X,
        BATTLE_RESULTS_WINDOW_Y + BATTLE_RESULTS_SCENE_Y,
    );
    draw_result_text_centered(
        canvas,
        &format!("Battle at {}", session.system_name),
        BATTLE_RESULTS_WINDOW_X + 212.0,
        BATTLE_RESULTS_WINDOW_Y + 34.0,
        18.0,
        text_color,
    );
    draw_result_wrapped_lines(
        canvas,
        &battle_result_summary_lines(session),
        BATTLE_RESULTS_WINDOW_X + 25.0,
        BATTLE_RESULTS_WINDOW_Y + 230.0,
        350.0,
        14.0,
        text_color,
    );
}

fn draw_battle_result_force_details(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
    session: &BattleSession,
    world: &GameWorld,
    troop_transport: &TroopTransportState,
    alliance: bool,
    category: BattleResultCategory,
    text_color: Color,
) {
    use resources::strategy as art;
    draw_strategy_bitmap(
        cache,
        art::BATTLE_FORCE_DETAIL_THREE_COLUMN,
        canvas,
        BATTLE_RESULTS_WINDOW_X + BATTLE_RESULTS_SCENE_X,
        BATTLE_RESULTS_WINDOW_Y + BATTLE_RESULTS_SCENE_Y,
    );
    let faction = if alliance { "Alliance" } else { "Imperial" };
    draw_result_text_centered(
        canvas,
        &format!("Battle at {}", session.system_name),
        BATTLE_RESULTS_WINDOW_X + 212.0,
        BATTLE_RESULTS_WINDOW_Y + 32.0,
        18.0,
        text_color,
    );
    draw_result_text_centered(
        canvas,
        &format!("{faction} Forces"),
        BATTLE_RESULTS_WINDOW_X + 212.0,
        BATTLE_RESULTS_WINDOW_Y + 52.0,
        16.0,
        text_color,
    );

    for (index, item) in BattleResultCategory::ALL.into_iter().enumerate() {
        let center = BATTLE_RESULTS_WINDOW_X + 62.0 + index as f32 * 96.0;
        let color = if item == category { WHITE } else { text_color };
        draw_result_text_centered(
            canvas,
            item.label(),
            center,
            BATTLE_RESULTS_WINDOW_Y + 78.0,
            10.0,
            color,
        );
    }

    for (index, label) in ["Operational", "Damaged", "Destroyed"]
        .into_iter()
        .enumerate()
    {
        draw_result_text_centered(
            canvas,
            label,
            BATTLE_RESULTS_WINDOW_X + 80.0 + index as f32 * 111.0,
            BATTLE_RESULTS_WINDOW_Y + 99.0,
            11.0,
            text_color,
        );
    }

    let entries = battle_result_entries(session, world, troop_transport, alliance, category);
    for (column, condition) in [
        BattleResultCondition::Operational,
        BattleResultCondition::Damaged,
        BattleResultCondition::Destroyed,
    ]
    .into_iter()
    .enumerate()
    {
        for (row, entry) in entries
            .iter()
            .filter(|entry| entry.condition == condition)
            .take(9)
            .enumerate()
        {
            draw_result_text_centered(
                canvas,
                &entry.name,
                BATTLE_RESULTS_WINDOW_X + 80.0 + column as f32 * 111.0,
                BATTLE_RESULTS_WINDOW_Y + 122.0 + row as f32 * 20.0,
                11.0,
                WHITE,
            );
        }
    }
}

fn draw_battle_result_destinations(
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
    session: &BattleSession,
    player_alliance: bool,
    mouse: (f32, f32),
    mouse_down: bool,
    mouse_released: bool,
    text_color: Color,
) -> TacticalAction {
    use resources::strategy as art;
    draw_strategy_bitmap(
        cache,
        art::BATTLE_DESTINATION_BACKGROUND,
        canvas,
        BATTLE_RESULTS_WINDOW_X + BATTLE_RESULTS_SCENE_X,
        BATTLE_RESULTS_WINDOW_Y + BATTLE_RESULTS_SCENE_Y,
    );
    draw_result_text_centered(
        canvas,
        "Go Directly To...",
        BATTLE_RESULTS_WINDOW_X + 212.0,
        BATTLE_RESULTS_WINDOW_Y + 56.0,
        18.0,
        text_color,
    );

    let system_rect = NativeRect::new(
        BATTLE_RESULTS_WINDOW_X + 29.0,
        BATTLE_RESULTS_WINDOW_Y + 182.0,
        169.0,
        96.0,
    );
    let fleet_rect = NativeRect::new(
        BATTLE_RESULTS_WINDOW_X + 229.0,
        BATTLE_RESULTS_WINDOW_Y + 182.0,
        169.0,
        96.0,
    );
    draw_strategy_bitmap(
        cache,
        art::BATTLE_DESTINATION_SYSTEM,
        canvas,
        system_rect.x,
        system_rect.y,
    );
    let winning_alliance = winner_is_alliance(session);
    if let Some(winning_alliance) = winning_alliance {
        draw_strategy_bitmap(
            cache,
            if winning_alliance {
                art::BATTLE_DESTINATION_ALLIANCE_FLEET
            } else {
                art::BATTLE_DESTINATION_EMPIRE_FLEET
            },
            canvas,
            fleet_rect.x,
            fleet_rect.y,
        );
    }
    let selected = if player_alliance {
        art::BATTLE_DESTINATION_ALLIANCE_SELECTED
    } else {
        art::BATTLE_DESTINATION_EMPIRE_SELECTED
    };
    if mouse_down && system_rect.contains(mouse.0, mouse.1) {
        draw_strategy_bitmap(cache, selected, canvas, system_rect.x, system_rect.y);
    }
    if winning_alliance.is_some() && mouse_down && fleet_rect.contains(mouse.0, mouse.1) {
        draw_strategy_bitmap(cache, selected, canvas, fleet_rect.x, fleet_rect.y);
    }
    draw_result_text_centered(
        canvas,
        &session.system_name,
        system_rect.x + system_rect.width * 0.5,
        BATTLE_RESULTS_WINDOW_Y + 174.0,
        14.0,
        WHITE,
    );
    if let Some(winning_alliance) = winning_alliance {
        draw_result_text_centered(
            canvas,
            if winning_alliance {
                "Alliance Fleet"
            } else {
                "Imperial Fleet"
            },
            fleet_rect.x + fleet_rect.width * 0.5,
            BATTLE_RESULTS_WINDOW_Y + 174.0,
            14.0,
            WHITE,
        );
    }

    if mouse_released && system_rect.contains(mouse.0, mouse.1) {
        macroquad::logging::info!("[tactical-results] destination=system");
        return TacticalAction::OpenBattleSystem(session.system);
    }
    if mouse_released && fleet_rect.contains(mouse.0, mouse.1) {
        if let Some(fleet) = winner_fleet(session) {
            macroquad::logging::info!("[tactical-results] destination=fleet");
            return TacticalAction::OpenBattleFleet(fleet);
        }
    }
    TacticalAction::None
}

fn battle_alert_retreat_enabled(session: &BattleSession) -> bool {
    session.ships.iter().any(|ship| {
        ship.alive
            && ship.is_attacker == session.player_is_attacker
            && ship.subsystem_capacity.hyperdrive > 0
            && ship.subsystem_condition.hyperdrive > 0
    })
}

/// The summary's text box (`FUN_0044f860`): font 6 at (0x24, 0xd8), 0x148
/// wide, over a black copy one pixel down and right.
const BATTLE_ALERT_SUMMARY_X: f32 = 36.0;
const BATTLE_ALERT_SUMMARY_Y: f32 = 216.0;
const BATTLE_ALERT_SUMMARY_WIDTH: f32 = 328.0;
/// Font 6's cell (`FUN_0060eed0`: lfHeight 18), the summary's line pitch.
const BATTLE_ALERT_SUMMARY_LINE: f32 = 18.0;
/// hyp: measured, not traced. Steam-guide 055's lines ("The Imperial fleet
/// has entered the Deyer" 275 pixels, "system. Alliance forces have been
/// detected on" 318) measure at an em of 15.5, not the 18-pixel cell's
/// 16.1, as the alert's other fonts do; its title, font 0xb, measures at 21.
const BATTLE_ALERT_SUMMARY_EM: f32 = 15.5;
const BATTLE_ALERT_TITLE_EM: f32 = 21.0;
/// Arial's ascent, 1854 of its 2048 units: GDI puts a cell's top there.
const ARIAL_ASCENT: f32 = 1854.0 / 2048.0;

/// The summary, wrapped in its box, each line over its shadow.
fn draw_battle_alert_summary(canvas: TacticalCanvas, lines: &[String], color: Color) {
    let font = result_font();
    let font_size = (BATTLE_ALERT_SUMMARY_EM * canvas.scale).round().max(1.0) as u16;
    let fits = |text: &str| {
        result_text_width(text, font.as_ref(), font_size)
            <= BATTLE_ALERT_SUMMARY_WIDTH * canvas.scale
    };
    let mut wrapped = Vec::new();
    for line in lines {
        let mut current = String::new();
        for word in line.split_whitespace() {
            let candidate = if current.is_empty() {
                word.to_owned()
            } else {
                format!("{current} {word}")
            };
            if current.is_empty() || fits(&candidate) {
                current = candidate;
            } else {
                wrapped.push(std::mem::replace(&mut current, word.to_owned()));
            }
        }
        if !current.is_empty() {
            wrapped.push(current);
        }
    }
    let x = BATTLE_ALERT_WINDOW_X + BATTLE_ALERT_SUMMARY_X;
    let top = BATTLE_ALERT_WINDOW_Y + BATTLE_ALERT_SUMMARY_Y;
    for (index, line) in wrapped.iter().enumerate() {
        let baseline =
            top + index as f32 * BATTLE_ALERT_SUMMARY_LINE + BATTLE_ALERT_SUMMARY_EM * ARIAL_ASCENT;
        for (dx, tint) in [(1.0, BLACK), (0.0, color)] {
            let (screen_x, screen_y) = canvas.point(x + dx, baseline + dx);
            draw_text_ex(
                line,
                screen_x,
                screen_y,
                TextParams {
                    font: font.as_ref(),
                    font_size,
                    color: tint,
                    ..TextParams::default()
                },
            );
        }
    }
}

fn draw_battle_alert_text(state: &TacticalState, canvas: TacticalCanvas, text_color: Color) {
    let session = state.session.as_ref().expect("active Battle Alert session");
    let title = state.battle_alert_text.as_ref().map_or_else(
        || format!("Battle at {}", session.system_name),
        |(title, _)| title.clone(),
    );
    // FUN_0044f860: the title in font 0xb, centred in 400 from the window's
    // left with its cell at y 16 (steam-guide 055, window at (4, 12):
    // "Battle at Deyer" centres on x 200 and starts at y 20).
    draw_result_text_centered(
        canvas,
        &title,
        BATTLE_ALERT_WINDOW_X + 200.0,
        BATTLE_ALERT_WINDOW_Y + 16.0 + BATTLE_ALERT_TITLE_EM * ARIAL_ASCENT,
        BATTLE_ALERT_TITLE_EM,
        text_color,
    );
    let player_alliance = player_is_alliance(session);
    let lines = match state.battle_alert_tab {
        BattleAlertTab::Summary if state.battle_alert_text.is_some() => {
            vec![state
                .battle_alert_text
                .as_ref()
                .map(|(_, summary)| summary.clone())
                .unwrap_or_default()]
        }
        BattleAlertTab::Summary => vec![
            format!(
                "The {} fleet has entered the {} system.",
                if player_alliance {
                    "Imperial"
                } else {
                    "Alliance"
                },
                session.system_name
            ),
            format!(
                "{} forces have been detected on an intercept course.",
                if player_alliance {
                    "Alliance"
                } else {
                    "Imperial"
                }
            ),
        ],
        // The other tabs show the list instead (FUN_00450660).
        BattleAlertTab::AllianceForces
        | BattleAlertTab::ImperialForces
        | BattleAlertTab::System => Vec::new(),
    };
    draw_battle_alert_summary(canvas, &lines, text_color);
}

/// The Battle Alert's list (`FUN_0044f860`): `FUN_00607ea0` at (0x53, 0x28),
/// 0x100 by 0xea, rows 0x28 high. A row's picture and text cell start at
/// its top left: the GOKRES minis of the System Assets capture (steam-guide
/// 056, whose window sits at (1, 5)) are exactly at (83, 80) and (83, 120).
const BATTLE_ALERT_LIST_X: f32 = 83.0;
const BATTLE_ALERT_LIST_Y: f32 = 40.0;
const BATTLE_ALERT_LIST_WIDTH: f32 = 256.0;
const BATTLE_ALERT_LIST_HEIGHT: f32 = 234.0;
const BATTLE_ALERT_ROW_HEIGHT: f32 = 40.0;
/// `FUN_00450e10`: the text starts 0x46 in, 0x5a when indented, the
/// indented picture 0x14 in.
const BATTLE_ALERT_TEXT_X: f32 = 70.0;
const BATTLE_ALERT_INDENT: f32 = 20.0;
/// The centred rows (`+0x50 = 1`) centre on x 203.5 in captures 056 and
/// 057.
const BATTLE_ALERT_CENTRE_WIDTH: f32 = 241.0;
/// hyp: measured, not traced. Font 4's rows measure at a 14-pixel cell
/// ("GenCore Level II" 91 pixels wide, "A-wing" 36), font 5's in bold
/// ("Deyer" 34), and the 0xb heading at an em of 20.5 ("Alliance Forces"
/// 142 wide; a 24-pixel cell would give 148), as the Message Index's text
/// does (`ghidra/notes/message-index-rows.md`).
const BATTLE_ALERT_ROW_EM: f32 = 14.0 * 2048.0 / 2288.0;
const BATTLE_ALERT_HEADING_EM: f32 = 20.5;

/// How a Battle Alert row draws (`FUN_00450e10`'s font and `+0x50`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BattleAlertRowStyle {
    /// The page's heading, 0x1923..0x1925, font 0xb, centred.
    Heading,
    /// A fleet, font 5 (bold) without a picture, centred.
    Fleet,
    /// A capital ship, font 5 with its mini.
    Ship,
    /// What a ship carries or the system holds, font 4 with its mini.
    Unit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BattleAlertRow {
    label: String,
    style: BattleAlertRowStyle,
    /// The GOKRES mini (`FUN_0042c3b0(.., 0, 1)`).
    mini: Option<u32>,
    /// Carried by a ship: picture and text 0x14 further in.
    indented: bool,
}

impl BattleAlertRow {
    fn heading(label: &str) -> Self {
        Self {
            label: label.to_owned(),
            style: BattleAlertRowStyle::Heading,
            mini: None,
            indented: false,
        }
    }

    fn unit(mini: Option<u32>, label: String, indented: bool) -> Self {
        Self {
            label,
            style: BattleAlertRowStyle::Unit,
            mini,
            indented,
        }
    }
}

/// The rows `FUN_00450770` fills the list with for `tab`. The force pages
/// list each of that side's fleets at the system (`FUN_004ffef0`), then each
/// ship (`FUN_00502db0`) and what it carries: squadrons, regiments and
/// personnel (`FUN_00450ce0`). System Assets lists the system's facilities
/// (families 0x20..0x2f, `FUN_00539d70`), regiments (`FUN_00504c40`) and
/// personnel (`FUN_00536da0`).
///
/// Last come the side's squadrons at the system (`FUN_00503a50`), under the
/// system's name. port: the facilities follow the port's lists: defenses, then manufacturing,
/// then production.
fn battle_alert_rows(
    session: &BattleSession,
    world: &GameWorld,
    transport: &TroopTransportState,
    tab: BattleAlertTab,
) -> Vec<BattleAlertRow> {
    use crate::fleet_window::{listed_units, FleetWindowEntry, FleetWindowTab};
    let Some(system) = world.systems.get(session.system) else {
        return Vec::new();
    };
    let alliance = match tab {
        BattleAlertTab::Summary => return Vec::new(),
        BattleAlertTab::AllianceForces => true,
        BattleAlertTab::ImperialForces => false,
        BattleAlertTab::System => {
            use crate::system_window::{
                character_mini_resource_id, defense_facility_mini, manufacturing_facility_mini,
                production_facility_mini, special_force_mini, troop_mini,
            };
            let unit = |found: Option<(u32, &str)>| {
                found.map(|(mini, label)| BattleAlertRow::unit(Some(mini), label.to_owned(), false))
            };
            let mut rows = vec![BattleAlertRow::heading("System Assets")];
            rows.extend(system.defense_facilities.iter().filter_map(|&key| {
                unit(defense_facility_mini(
                    world.defense_facilities.get(key)?.class_dat_id,
                ))
            }));
            rows.extend(system.manufacturing_facilities.iter().filter_map(|&key| {
                unit(manufacturing_facility_mini(
                    world.manufacturing_facilities.get(key)?.class_dat_id,
                ))
            }));
            rows.extend(system.production_facilities.iter().filter_map(|&key| {
                unit(production_facility_mini(
                    world.production_facilities.get(key)?.class_dat_id,
                ))
            }));
            rows.extend(
                system
                    .ground_units
                    .iter()
                    .filter_map(|&key| unit(troop_mini(world.troops.get(key)?.class_dat_id))),
            );
            rows.extend(
                crate::quadrant_icons::system_members(world, session.system).filter_map(
                    |(member, _, _)| match member {
                        rebellion_core::missions::MissionMember::Character(key) => {
                            let character = world.characters.get(key)?;
                            Some(BattleAlertRow::unit(
                                character_mini_resource_id(character.dat_id, character.is_major),
                                character.name.clone(),
                                false,
                            ))
                        }
                        rebellion_core::missions::MissionMember::SpecialForce(key) => unit(
                            special_force_mini(world.special_forces.get(key)?.class_dat_id),
                        ),
                    },
                ),
            );
            return rows;
        }
    };
    let mut rows = vec![BattleAlertRow::heading(if alliance {
        "Alliance Forces"
    } else {
        "Imperial Forces"
    })];
    for &fleet in &system.fleets {
        if world
            .fleets
            .get(fleet)
            .is_none_or(|value| value.is_alliance != alliance)
        {
            continue;
        }
        rows.push(BattleAlertRow {
            label: world.fleet_name(fleet).unwrap_or_default().to_owned(),
            style: BattleAlertRowStyle::Fleet,
            mini: None,
            indented: false,
        });
        let ships = listed_units(
            world,
            transport,
            FleetWindowEntry::Fleet(fleet),
            FleetWindowTab::CapitalShips,
        );
        for (mini, label, index) in ships {
            rows.push(BattleAlertRow {
                label,
                style: BattleAlertRowStyle::Ship,
                mini,
                indented: false,
            });
            let Some(index) = index else {
                continue;
            };
            for tab in [
                FleetWindowTab::Fighters,
                FleetWindowTab::Troops,
                FleetWindowTab::Personnel,
            ] {
                rows.extend(
                    listed_units(
                        world,
                        transport,
                        FleetWindowEntry::Ship { fleet, index },
                        tab,
                    )
                    .into_iter()
                    .map(|(mini, label, _)| BattleAlertRow::unit(mini, label, true)),
                );
            }
        }
    }
    // Last, the side's squadrons the system holds (`FUN_00503a50`) under the
    // system's name. hyp: they are not indented, as the guide's "A-wing"
    // under "Deyer" reads.
    let squadrons: Vec<BattleAlertRow> = system
        .fighters
        .iter()
        .filter(|entry| entry.is_alliance == alliance)
        .filter_map(|entry| {
            let class = world.fighter_classes.get(entry.class)?;
            let mini = crate::panels::fleets::fighter_mini_id(class.dat_id);
            Some(
                (0..entry.count)
                    .map(move |_| BattleAlertRow::unit(mini, class.name.clone(), false)),
            )
        })
        .flatten()
        .collect();
    if !squadrons.is_empty() {
        rows.push(BattleAlertRow {
            label: system.name.clone(),
            style: BattleAlertRowStyle::Fleet,
            mini: None,
            indented: false,
        });
        rows.extend(squadrons);
    }
    rows
}

/// The rows the list shows at once, a partial last row included.
fn battle_alert_visible_rows() -> usize {
    (BATTLE_ALERT_LIST_HEIGHT / BATTLE_ALERT_ROW_HEIGHT).ceil() as usize
}

/// Draw the list page and its scroll bar, and scroll on the arrows, the
/// track and the wheel.
fn draw_battle_alert_list(
    state: &mut TacticalState,
    cache: &mut BmpCache,
    canvas: TacticalCanvas,
    rows: &[BattleAlertRow],
    mouse: (f32, f32),
) {
    state.battle_alert_scroll = state
        .battle_alert_scroll
        .min(battle_alert_scroll_range(0, rows.len()).max_first());
    let origin_x = BATTLE_ALERT_WINDOW_X + BATTLE_ALERT_LIST_X;
    let origin_y = BATTLE_ALERT_WINDOW_Y + BATTLE_ALERT_LIST_Y;
    let (clip_x, clip_y) = canvas.point(origin_x, origin_y);
    set_tactical_aperture_clip(Some(NativeRect::new(
        clip_x,
        clip_y,
        BATTLE_ALERT_LIST_WIDTH * canvas.scale,
        BATTLE_ALERT_LIST_HEIGHT * canvas.scale,
    )));
    let regular = result_font();
    let bold = RESULT_BOLD_FONT.with(Clone::clone);
    let first = state.battle_alert_scroll;
    for (line, row) in rows
        .iter()
        .skip(first)
        .take(battle_alert_visible_rows())
        .enumerate()
    {
        let top = origin_y + line as f32 * BATTLE_ALERT_ROW_HEIGHT;
        let indent = if row.indented {
            BATTLE_ALERT_INDENT
        } else {
            0.0
        };
        if let Some(mini) = row.mini {
            draw_original_bitmap(
                cache,
                DllSource::Gokres,
                mini,
                canvas,
                origin_x + indent,
                top,
                None,
            );
        }
        let (em, font) = match row.style {
            BattleAlertRowStyle::Heading => (BATTLE_ALERT_HEADING_EM, regular.as_ref()),
            BattleAlertRowStyle::Fleet | BattleAlertRowStyle::Ship => {
                (BATTLE_ALERT_ROW_EM, bold.as_ref().or(regular.as_ref()))
            }
            BattleAlertRowStyle::Unit => (BATTLE_ALERT_ROW_EM, regular.as_ref()),
        };
        // GDI places the cell's top at `top`; Arial's ascent is 1854 of
        // its 2048 units.
        let baseline = top + em * 1854.0 / 2048.0;
        let font_size = (em * canvas.scale).max(1.0) as u16;
        let centred = matches!(
            row.style,
            BattleAlertRowStyle::Heading | BattleAlertRowStyle::Fleet
        );
        let (x, y) = if centred {
            let width = result_text_width(&row.label, font, font_size) / canvas.scale;
            canvas.point(
                origin_x + (BATTLE_ALERT_CENTRE_WIDTH - width) / 2.0,
                baseline,
            )
        } else {
            canvas.point(origin_x + indent + BATTLE_ALERT_TEXT_X, baseline)
        };
        draw_text_ex(
            &row.label,
            x,
            y,
            TextParams {
                font,
                font_size,
                color: WHITE,
                ..TextParams::default()
            },
        );
    }
    set_tactical_aperture_clip(None);

    // The scroll bar shows while the rows overflow the list (captures 056
    // and 057).
    let bar = battle_alert_scroll_bar();
    let range = battle_alert_scroll_range(first, rows.len());
    let pointer = ScrollPointer {
        at: Some((
            mouse.0 - BATTLE_ALERT_WINDOW_X,
            mouse.1 - BATTLE_ALERT_WINDOW_Y,
        )),
        pressed: is_mouse_button_pressed(MouseButton::Left),
        down: is_mouse_button_down(MouseButton::Left),
        released: is_mouse_button_released(MouseButton::Left),
    };
    state.battle_alert_scroll = bar.update(
        &mut state.battle_alert_scroll_drag,
        range,
        range.visible.floor() as usize,
        pointer,
    );
    let range = ScrollRange {
        first: state.battle_alert_scroll,
        ..range
    };
    for blit in bar.blits(range, &state.battle_alert_scroll_drag, pointer) {
        draw_original_bitmap(
            cache,
            DllSource::Strategy,
            blit.resource,
            canvas,
            BATTLE_ALERT_WINDOW_X + blit.x,
            BATTLE_ALERT_WINDOW_Y + blit.y,
            Some(Rect::new(0.0, 0.0, 13.0, blit.height)),
        );
    }
    let in_list = mouse.0 >= origin_x
        && mouse.0 < origin_x + BATTLE_ALERT_LIST_WIDTH
        && mouse.1 >= origin_y
        && mouse.1 < origin_y + BATTLE_ALERT_LIST_HEIGHT;
    let wheel = mouse_wheel().1;
    if in_list && wheel != 0.0 && range.overflows() {
        state.battle_alert_scroll = if wheel > 0.0 {
            state.battle_alert_scroll.saturating_sub(1)
        } else {
            (state.battle_alert_scroll + 1).min(range.max_first())
        };
    }
}

/// The list's bar, RCDATA `0x29fc` (`FUN_0044f860`), in window coordinates.
fn battle_alert_scroll_bar() -> ScrollBar {
    ScrollBar::new(
        LIST_BAR_29FC,
        BATTLE_ALERT_LIST_X,
        BATTLE_ALERT_LIST_Y,
        BATTLE_ALERT_LIST_WIDTH,
        BATTLE_ALERT_LIST_HEIGHT,
    )
}

fn battle_alert_scroll_range(first: usize, rows: usize) -> ScrollRange {
    ScrollRange {
        first,
        visible: BATTLE_ALERT_LIST_HEIGHT / BATTLE_ALERT_ROW_HEIGHT,
        total: rows,
    }
}

fn draw_original_battle_alert(
    state: &mut TacticalState,
    cache: &mut BmpCache,
    world: &GameWorld,
    transport: &TroopTransportState,
) -> TacticalAction {
    let canvas = TacticalCanvas::new(screen_width(), screen_height());
    let session = state.session.as_ref().expect("active Battle Alert session");
    prewarm_tactical_font_atlas(session, canvas, &mut state.warmed_font_sizes);
    let player_alliance = player_is_alliance(session);
    let skin = battle_alert_skin(player_alliance);
    let text_color = if player_alliance {
        Color::from_rgba(255, 45, 40, 255)
    } else {
        Color::from_rgba(40, 255, 65, 255)
    };

    // The app has drawn the live command center beneath this window.
    draw_strategy_bitmap(
        cache,
        skin.frame,
        canvas,
        BATTLE_ALERT_WINDOW_X,
        BATTLE_ALERT_WINDOW_Y,
    );
    // FUN_00450660: Summary shows layer 0x32, the scene; the list pages
    // layer 0x33, their own picture under the same title.
    draw_strategy_bitmap(
        cache,
        if state.battle_alert_tab == BattleAlertTab::Summary {
            skin.scene
        } else {
            skin.list_scene
        },
        canvas,
        BATTLE_ALERT_WINDOW_X + 12.0,
        BATTLE_ALERT_WINDOW_Y + 13.0,
    );
    draw_battle_alert_text(state, canvas, text_color);
    if state.battle_alert_tab != BattleAlertTab::Summary {
        let rows = battle_alert_rows(session, world, transport, state.battle_alert_tab);
        let (pointer_x, pointer_y) = mouse_position();
        let pointer = canvas.logical_pointer(pointer_x, pointer_y);
        draw_battle_alert_list(state, cache, canvas, &rows, pointer);
    }
    let session = state.session.as_ref().expect("active Battle Alert session");
    draw_strategy_bitmap(
        cache,
        skin.rail,
        canvas,
        BATTLE_ALERT_WINDOW_X + BATTLE_RESULTS_RAIL_X,
        BATTLE_ALERT_WINDOW_Y,
    );

    let (screen_mouse_x, screen_mouse_y) = mouse_position();
    let mouse = canvas.logical_pointer(screen_mouse_x, screen_mouse_y);
    let local_mouse = (
        mouse.0 - BATTLE_ALERT_WINDOW_X,
        mouse.1 - BATTLE_ALERT_WINDOW_Y,
    );
    let mouse_down = is_mouse_button_down(MouseButton::Left);
    let mouse_pressed = is_mouse_button_pressed(MouseButton::Left);
    let mouse_released = is_mouse_button_released(MouseButton::Left);

    for (index, tab) in BattleAlertTab::ALL.into_iter().enumerate() {
        let rect = skin.tab_rects[index];
        let active = state.battle_alert_tab == tab
            || (mouse_down && rect.contains(local_mouse.0, local_mouse.1));
        draw_strategy_bitmap(
            cache,
            skin.tabs[index][usize::from(active)],
            canvas,
            BATTLE_ALERT_WINDOW_X + rect.x,
            BATTLE_ALERT_WINDOW_Y + rect.y,
        );
        if mouse_released && rect.contains(local_mouse.0, local_mouse.1) {
            if state.battle_alert_tab != tab {
                state.battle_alert_scroll = 0;
            }
            state.battle_alert_tab = tab;
            macroquad::logging::info!("[battle_alert] tab={}", tab.label());
        }
    }

    let button_controls = [
        BattleAlertControl::Retreat,
        BattleAlertControl::Simulate,
        BattleAlertControl::TakeCommand,
    ];
    let button_rects = [
        NativeRect::new(12.0, 296.0, 134.0, 27.0),
        NativeRect::new(146.0, 296.0, 134.0, 27.0),
        NativeRect::new(280.0, 296.0, 134.0, 27.0),
    ];
    let retreat_enabled = battle_alert_retreat_enabled(session);
    let hovered = button_rects
        .iter()
        .enumerate()
        .find(|(index, rect)| {
            (*index != 0 || retreat_enabled) && rect.contains(local_mouse.0, local_mouse.1)
        })
        .map(|(index, _)| button_controls[index]);
    if mouse_pressed {
        state.battle_alert_pressed = hovered;
    }

    for (index, rect) in button_rects.into_iter().enumerate() {
        let control = button_controls[index];
        let enabled = control != BattleAlertControl::Retreat || retreat_enabled;
        let resource_state = if !enabled {
            2
        } else if mouse_down
            && state.battle_alert_pressed == Some(control)
            && hovered == Some(control)
        {
            1
        } else {
            0
        };
        draw_strategy_bitmap(
            cache,
            skin.buttons[index][resource_state],
            canvas,
            BATTLE_ALERT_WINDOW_X + rect.x,
            BATTLE_ALERT_WINDOW_Y + rect.y,
        );
    }

    if mouse_released {
        let captured = state.battle_alert_pressed.take();
        if let Some(control) = captured.filter(|control| Some(*control) == hovered) {
            state.battle_alert_open = false;
            return match control {
                BattleAlertControl::Retreat => {
                    if let Some(session) = state.session.as_mut() {
                        session.queue_withdrawal_started_voice();
                    }
                    macroquad::logging::info!(
                        "[battle_alert] command=retreat resource={} status=started",
                        skin.buttons[0][0]
                    );
                    TacticalAction::RetreatBeforeBattle
                }
                BattleAlertControl::Simulate => {
                    macroquad::logging::info!(
                        "[battle_alert] command=simulate resource={} status=routed",
                        skin.buttons[1][0]
                    );
                    TacticalAction::AutoResolve
                }
                BattleAlertControl::TakeCommand => {
                    macroquad::logging::info!(
                        "[battle_alert] command=take_command resource={} status=paused_tactical",
                        skin.buttons[2][0]
                    );
                    TacticalAction::BeginCombat
                }
            };
        }
    } else if !mouse_down {
        state.battle_alert_pressed = None;
    }
    TacticalAction::None
}

fn draw_original_battle_results(
    state: &mut TacticalState,
    cache: &mut BmpCache,
    world: &GameWorld,
    troop_transport: &TroopTransportState,
) -> TacticalAction {
    let canvas = TacticalCanvas::new(screen_width(), screen_height());
    let session = state.session.as_ref().expect("active result session");
    prewarm_tactical_font_atlas(session, canvas, &mut state.warmed_font_sizes);
    let player_alliance = player_is_alliance(session);
    let skin = battle_result_skin(player_alliance);
    let text_color = if player_alliance {
        Color::from_rgba(255, 45, 40, 255)
    } else {
        Color::from_rgba(40, 255, 65, 255)
    };

    // The app has drawn the live command center beneath this window.
    draw_strategy_bitmap(
        cache,
        skin.frame,
        canvas,
        BATTLE_RESULTS_WINDOW_X,
        BATTLE_RESULTS_WINDOW_Y,
    );

    let (screen_mouse_x, screen_mouse_y) = mouse_position();
    let mouse = canvas.logical_pointer(screen_mouse_x, screen_mouse_y);
    let mouse_down = is_mouse_button_down(MouseButton::Left);
    let mouse_released = is_mouse_button_released(MouseButton::Left);

    let mut action = match state.battle_result_tab {
        BattleResultTab::Summary => {
            draw_battle_result_summary(cache, canvas, session, text_color);
            TacticalAction::None
        }
        BattleResultTab::AllianceForces => {
            draw_battle_result_force_details(
                cache,
                canvas,
                session,
                world,
                troop_transport,
                true,
                state.battle_result_category,
                text_color,
            );
            TacticalAction::None
        }
        BattleResultTab::ImperialForces => {
            draw_battle_result_force_details(
                cache,
                canvas,
                session,
                world,
                troop_transport,
                false,
                state.battle_result_category,
                text_color,
            );
            TacticalAction::None
        }
        BattleResultTab::GoDirectlyTo => draw_battle_result_destinations(
            cache,
            canvas,
            session,
            player_alliance,
            mouse,
            mouse_down,
            mouse_released,
            text_color,
        ),
    };

    draw_strategy_bitmap(
        cache,
        skin.rail,
        canvas,
        BATTLE_RESULTS_WINDOW_X + BATTLE_RESULTS_RAIL_X,
        BATTLE_RESULTS_WINDOW_Y,
    );
    let close_active = mouse_down
        && skin.close_rect.contains(
            mouse.0 - BATTLE_RESULTS_WINDOW_X,
            mouse.1 - BATTLE_RESULTS_WINDOW_Y,
        );
    draw_strategy_bitmap(
        cache,
        skin.close[usize::from(close_active)],
        canvas,
        BATTLE_RESULTS_WINDOW_X + skin.close_rect.x,
        BATTLE_RESULTS_WINDOW_Y + skin.close_rect.y,
    );
    if mouse_released
        && skin.close_rect.contains(
            mouse.0 - BATTLE_RESULTS_WINDOW_X,
            mouse.1 - BATTLE_RESULTS_WINDOW_Y,
        )
    {
        action = TacticalAction::ReturnToGalaxy;
    }

    for (index, rect) in skin.tab_rects.into_iter().enumerate() {
        let local_mouse = (
            mouse.0 - BATTLE_RESULTS_WINDOW_X,
            mouse.1 - BATTLE_RESULTS_WINDOW_Y,
        );
        let selected = state.battle_result_tab.index() == index;
        let active = selected || (mouse_down && rect.contains(local_mouse.0, local_mouse.1));
        draw_strategy_bitmap(
            cache,
            skin.tabs[index][usize::from(active)],
            canvas,
            BATTLE_RESULTS_WINDOW_X + rect.x,
            BATTLE_RESULTS_WINDOW_Y + rect.y,
        );
        if mouse_released && rect.contains(local_mouse.0, local_mouse.1) {
            state.battle_result_tab = BattleResultTab::from_index(index);
            macroquad::logging::info!(
                "[tactical-results] tab={} category={}",
                state.battle_result_tab.label(),
                state.battle_result_category.label()
            );
        }
    }

    if matches!(
        state.battle_result_tab,
        BattleResultTab::AllianceForces | BattleResultTab::ImperialForces
    ) && mouse_released
    {
        for (index, category) in BattleResultCategory::ALL.into_iter().enumerate() {
            let rect = NativeRect::new(
                BATTLE_RESULTS_WINDOW_X + 14.0 + index as f32 * 96.0,
                BATTLE_RESULTS_WINDOW_Y + 58.0,
                96.0,
                25.0,
            );
            if rect.contains(mouse.0, mouse.1) {
                state.battle_result_category = category;
                macroquad::logging::info!(
                    "[tactical-results] tab={} category={}",
                    state.battle_result_tab.label(),
                    category.label()
                );
            }
        }
    }
    action
}

/// Draw the tactical combat view (macroquad).
///
/// Call this when `GameMode::TacticalCombat`. Returns a `TacticalAction`
/// indicating whether the main loop should transition modes.
#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
#[expect(
    clippy::cast_precision_loss,
    reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
)]
///
/// # Panics
/// Panics if the active battle session disappears during drawing.
pub fn draw_tactical_view(
    state: &mut TacticalState,
    bmp_cache: &mut BmpCache,
    world: &GameWorld,
    troop_transport: &TroopTransportState,
) -> TacticalAction {
    if state.session.is_none() {
        return TacticalAction::ReturnToGalaxy;
    }

    if state.battle_alert_open {
        return draw_original_battle_alert(state, bmp_cache, world, troop_transport);
    }

    let mut action = TacticalAction::None;

    // 0. Advance combat if in Combat phase (mutable borrow).
    {
        let session = state.session.as_mut().unwrap();
        if session.phase == BattlePhase::Combat && !session.paused {
            // Step at ~4 ticks per second (at 60fps), scaled by combat_speed.
            let dt = get_frame_time();
            session.advance_presentational_effects(dt);
            let ticks_per_sec = 4.0 * session.combat_speed as f32;
            session.step_accumulator += dt * ticks_per_sec;
            while session.step_accumulator >= 1.0 {
                session.step_accumulator -= 1.0;
                session.step();
                if session.phase == BattlePhase::Results {
                    break;
                }
            }
        }
        flush_engine_log(session);
    }

    if state
        .session
        .as_ref()
        .is_some_and(|session| session.phase == BattlePhase::Results)
    {
        return draw_original_battle_results(state, bmp_cache, world, troop_transport);
    }

    // 1. Paint the unscaled-source tactical shell first, then put the battle
    // only inside its original black aperture. Bitmap 1000 includes every
    // frame edge and the empty right-hand panel housing.
    let canvas = TacticalCanvas::new(screen_width(), screen_height());
    prewarm_tactical_font_atlas(
        state.session.as_ref().unwrap(),
        canvas,
        &mut state.warmed_font_sizes,
    );
    clear_background(BLACK);
    draw_tactical_bitmap(bmp_cache, resources::tactical::BACKGROUND, canvas, 0.0, 0.0);
    set_tactical_aperture_clip(Some(canvas.aperture()));
    let aperture = canvas.aperture();
    let aperture_tuple = (aperture.x, aperture.y, aperture.width, aperture.height);
    update_tactical_camera_follow(state);
    if state.render_original_starfield {
        state.asset_renderer.draw_backdrop(aperture_tuple);
    }
    if state.render_original_planet {
        state.asset_renderer.draw_planet(aperture_tuple);
    }

    // 2. Submit mapped production participants through their original 3D and
    // type-303 resource families. Unmapped or unavailable objects retain the
    // bounded 2D fallback below.
    let (scale, offset_x, offset_y) =
        canvas.arena_transform(state.zoom, state.camera_x, state.camera_y);
    let render_pyrotechnics = state.render_pyrotechnics;
    let (
        production_objects,
        production_fighters,
        production_fields,
        production_projectiles,
        production_effects,
    ) = if state.render_original_participants {
        state
            .session
            .as_ref()
            .map(|session| {
                let ships = session
                    .ships
                    .iter()
                    .enumerate()
                    .filter(|(_, ship)| ship.alive)
                    .filter_map(|(index, ship)| {
                        ship.tactical_resource.map(|resource| TacticalRenderObject {
                            object_id: u32::try_from(index).unwrap_or(u32::MAX).saturating_add(1),
                            resource_base: resource.mesh_resource_base,
                            position: ship.source_position.rendered(),
                        })
                    })
                    .collect::<Vec<_>>();
                let fighters = session
                    .fighters
                    .iter()
                    .enumerate()
                    .filter(|(_, fighter)| fighter.alive)
                    .filter_map(|(index, fighter)| {
                        fighter.tactical_resource.map(|resource| {
                            let player_side = fighter.is_attacker == session.player_is_attacker;
                            TacticalFighterRenderObject {
                                object_id: u32::try_from(index)
                                    .unwrap_or(u32::MAX)
                                    .saturating_add(1001),
                                close_resource_id: resource
                                    .grouped_close_resource(player_side, fighter.fighter_group),
                                far_resource_id: resource
                                    .grouped_far_resource(player_side, fighter.fighter_group),
                                indicator_resource_id: resource
                                    .grouped_indicator_resource(player_side, fighter.fighter_group),
                                position: fighter.source_position.rendered(),
                            }
                        })
                    })
                    .collect::<Vec<_>>();
                let mut effects = if render_pyrotechnics {
                    session
                        .impact_effects
                        .iter()
                        .filter_map(|effect| {
                            let ship = session.ships.get(effect.target)?;
                            let (source_width, source_height) = effect.sequence.source_size();
                            Some(TacticalEffectRenderObject {
                                object_id: u32::try_from(effect.target)
                                    .unwrap_or(u32::MAX)
                                    .saturating_add(1),
                                resource_id: effect.sequence.resource_base()
                                    + u32::from(effect.frame),
                                source_width,
                                source_height,
                                position: ship.source_position.rendered(),
                            })
                        })
                        .collect::<Vec<_>>()
                } else {
                    Vec::new()
                };
                if let Some(death_star) = session.death_star {
                    effects.push(TacticalEffectRenderObject {
                        object_id: 3001,
                        resource_id: death_star.resource_id(),
                        source_width: 128,
                        source_height: 128,
                        position: death_star.source_position.rendered(),
                    });
                }
                let fields = session
                    .field_effects
                    .iter()
                    .filter_map(|effect| {
                        let ship = session.ships.get(effect.target)?;
                        Some(TacticalEffectRenderObject {
                            object_id: u32::try_from(effect.target)
                                .unwrap_or(u32::MAX)
                                .saturating_add(1),
                            resource_id: effect.resource_id()?,
                            source_width: 128,
                            source_height: 128,
                            position: ship.source_position.rendered(),
                        })
                    })
                    .collect::<Vec<_>>();
                let mut projectiles = session
                    .weapon_effects
                    .iter()
                    .enumerate()
                    .map(|(index, effect)| TacticalProjectileRenderObject {
                        object_id: u32::try_from(index)
                            .unwrap_or(u32::MAX)
                            .saturating_add(2001),
                        origin: effect.source_position.rendered(),
                        target: effect.target_position.rendered(),
                        progress: effect.progress(),
                        longitudinal_scale: effect.longitudinal_scale,
                        shape_variant: effect.shape_variant,
                        color_selector: effect.color_selector,
                        color: effect.color(),
                    })
                    .collect::<Vec<_>>();
                if let Some(beam) = session.death_star_beam {
                    projectiles.push(TacticalProjectileRenderObject {
                        object_id: 3999,
                        origin: beam.source_position.rendered(),
                        target: beam.target_position.rendered(),
                        progress: (beam.elapsed / beam.duration).clamp(0.0, 1.0),
                        longitudinal_scale: 1.0,
                        shape_variant: 1,
                        color_selector: beam.color_selector,
                        color: projectile_selector_color(beam.color_selector),
                    });
                }
                (ships, fighters, fields, projectiles, effects)
            })
            .unwrap_or_default()
    } else {
        (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new())
    };
    let ship_report = state
        .asset_renderer
        .draw_participants(aperture_tuple, &production_objects);
    let fighter_report = state
        .asset_renderer
        .draw_fighters(aperture_tuple, &production_fighters);
    let projectile_report = state
        .asset_renderer
        .draw_projectiles(aperture_tuple, &production_projectiles);
    let _field_report = state
        .asset_renderer
        .draw_fields(aperture_tuple, &production_fields);
    let effect_report = state
        .asset_renderer
        .draw_effects(aperture_tuple, &production_effects);
    if let Some(session) = state.session.as_mut() {
        let mut formation_footprint_changed = false;
        for (object_id, envelope) in &ship_report.collision_envelopes {
            let Some(index) = object_id
                .checked_sub(1)
                .and_then(|index| usize::try_from(index).ok())
            else {
                continue;
            };
            if let Some(ship) = session.ships.get_mut(index) {
                let recovered = OriginalTacticalCollisionEnvelope {
                    vertical_radius: envelope.vertical_radius,
                    planar_diameter: envelope.planar_diameter,
                };
                if ship.source_collision_envelope != Some(recovered) {
                    ship.source_collision_envelope = Some(recovered);
                    formation_footprint_changed = true;
                }
            }
        }
        if formation_footprint_changed && session.combat_tick == 0 && session.paused {
            BattleSession::refresh_original_task_force_geometry(&mut session.ships);
        }
    }
    let rendered_ship_indexes = ship_report
        .rendered_object_ids
        .into_iter()
        .filter_map(|object_id| usize::try_from(object_id.saturating_sub(1)).ok())
        .collect::<Vec<_>>();
    let rendered_fighter_indexes = fighter_report
        .rendered_object_ids
        .into_iter()
        .filter_map(|object_id| usize::try_from(object_id.saturating_sub(1001)).ok())
        .collect::<Vec<_>>();
    let ship_projections = ship_report
        .projections
        .into_iter()
        .filter_map(|projection| {
            usize::try_from(projection.object_id.saturating_sub(1))
                .ok()
                .map(|index| (index, projection))
        })
        .collect::<HashMap<_, _>>();
    let fighter_projections = fighter_report
        .projections
        .into_iter()
        .filter_map(|projection| {
            usize::try_from(projection.object_id.saturating_sub(1001))
                .ok()
                .map(|index| (index, projection))
        })
        .collect::<HashMap<_, _>>();
    let rendered_effect_targets = effect_report
        .rendered_object_ids
        .into_iter()
        .filter_map(|object_id| usize::try_from(object_id.saturating_sub(1)).ok())
        .collect::<HashSet<_>>();
    let rendered_projectiles = projectile_report
        .rendered_object_ids
        .into_iter()
        .filter_map(|object_id| usize::try_from(object_id.saturating_sub(2001)).ok())
        .collect::<HashSet<_>>();
    let navigation_projections = tactical_navigation_projections(state, aperture);
    if state.logged_navigation_scene != Some(state.navigation_sets_visible) {
        let points = navigation_projections
            .iter()
            .map(|projection| {
                format!(
                    "{}:{}:{:.3},{:.3}",
                    projection.set + 1,
                    projection.point + 1,
                    projection.screen.x,
                    projection.screen.y,
                )
            })
            .collect::<Vec<_>>()
            .join(";");
        macroquad::logging::info!(
            "[tactical_navigation] event=scene visible={:?} screen_positions={}",
            state.navigation_sets_visible,
            points,
        );
        state.logged_navigation_scene = Some(state.navigation_sets_visible);
    }

    #[cfg(feature = "interface-test-fixtures")]
    if state.proof_resource_2560 {
        if state.proof_lod_follows_zoom {
            state
                .asset_renderer
                .set_view(TacticalLodView::from_fixture_zoom(state.zoom));
        }
        let aperture = canvas.aperture();
        state
            .asset_renderer
            .draw_proof((aperture.x, aperture.y, aperture.width, aperture.height));
    }

    // Borrow session for rendering (immutable reads).
    let session = state.session.as_ref().unwrap();
    let phase = session.phase;
    let player_is_attacker = session.player_is_attacker;
    let combat_tick = session.combat_tick;
    let paused = session.paused;
    draw_tactical_navigation_points(session, &navigation_projections, canvas.scale);

    // 3. Draw deployment zones (placement phase only).
    if phase == BattlePhase::Placement {
        let zone_w = ARENA_WIDTH * DEPLOY_ZONE_FRACTION * scale;
        draw_rectangle(
            offset_x,
            offset_y,
            zone_w,
            ARENA_HEIGHT * scale,
            Color::new(0.0, 0.15, 0.3, 0.15),
        );
        draw_rectangle_lines(
            offset_x,
            offset_y,
            zone_w,
            ARENA_HEIGHT * scale,
            2.0,
            Color::new(0.0, 0.4, 0.8, 0.5),
        );

        let def_x = offset_x + ARENA_WIDTH * (1.0 - DEPLOY_ZONE_FRACTION) * scale;
        draw_rectangle(
            def_x,
            offset_y,
            zone_w,
            ARENA_HEIGHT * scale,
            Color::new(0.3, 0.05, 0.0, 0.15),
        );
        draw_rectangle_lines(
            def_x,
            offset_y,
            zone_w,
            ARENA_HEIGHT * scale,
            2.0,
            Color::new(0.8, 0.2, 0.0, 0.5),
        );
    }

    // 4. Draw ships (macroquad primitives).
    let aperture = canvas.aperture();
    for (ship_index, ship) in session.ships.iter().enumerate() {
        #[cfg(feature = "interface-test-fixtures")]
        let suppress_fixture_fallback =
            state.suppress_mapped_capital_fallback && ship.tactical_resource.is_some();
        #[cfg(not(feature = "interface-test-fixtures"))]
        let suppress_fixture_fallback = false;
        if !ship.alive || rendered_ship_indexes.contains(&ship_index) || suppress_fixture_fallback {
            continue;
        }
        let sx = offset_x + ship.x * scale;
        let sy = offset_y + ship.y * scale;
        if !aperture.contains(sx, sy) {
            continue;
        }
        let size = DEFAULT_SHIP_SIZE * scale;

        let color = if ship.is_attacker {
            if ship.selected {
                Color::new(0.3, 1.0, 0.3, 0.9)
            } else {
                Color::new(0.2, 0.6, 1.0, 0.8)
            }
        } else {
            Color::new(1.0, 0.3, 0.2, 0.8)
        };

        let half = size / 2.0;
        draw_triangle(
            macroquad::math::Vec2::new(sx, sy - half),
            macroquad::math::Vec2::new(sx - half * 0.6, sy),
            macroquad::math::Vec2::new(sx + half * 0.6, sy),
            color,
        );
        draw_triangle(
            macroquad::math::Vec2::new(sx - half * 0.6, sy),
            macroquad::math::Vec2::new(sx + half * 0.6, sy),
            macroquad::math::Vec2::new(sx, sy + half),
            color,
        );

        let is_alliance = ship.is_attacker == session.attacker_is_alliance;
        let highlighted = if is_alliance {
            state.highlight_alliance
        } else {
            state.highlight_empire
        };
        if highlighted {
            let wire_color = if is_alliance {
                Color::new(0.9, 0.15, 0.12, 0.85)
            } else {
                Color::new(0.1, 0.9, 0.2, 0.85)
            };
            draw_rectangle_lines(
                sx - half * 0.85,
                sy - half * 0.85,
                size * 0.85,
                size * 0.85,
                1.0,
                wire_color,
            );
        }

        let bar_w = size * 0.8;
        let bar_h = 4.0;
        let bar_x = sx - bar_w / 2.0;
        let bar_y = sy + half + 4.0;
        let health_frac = if ship.hull_max > 0 {
            ship.hull_current as f32 / ship.hull_max as f32
        } else {
            0.0
        };
        draw_rectangle(bar_x, bar_y, bar_w, bar_h, Color::new(0.3, 0.0, 0.0, 0.8));
        let health_color = if health_frac > 0.5 {
            Color::new(0.0, 0.8, 0.0, 0.9)
        } else if health_frac > 0.25 {
            Color::new(0.9, 0.7, 0.0, 0.9)
        } else {
            Color::new(0.9, 0.1, 0.0, 0.9)
        };
        draw_rectangle(bar_x, bar_y, bar_w * health_frac, bar_h, health_color);

        if ship.shield_max > 0 {
            let shield_frac = ship.shield as f32 / ship.shield_max as f32;
            let sbar_y = sy - half - 8.0;
            draw_rectangle(bar_x, sbar_y, bar_w, bar_h, Color::new(0.0, 0.0, 0.3, 0.6));
            draw_rectangle(
                bar_x,
                sbar_y,
                bar_w * shield_frac,
                bar_h,
                Color::new(0.3, 0.5, 1.0, 0.8),
            );
        }

        #[expect(
            clippy::manual_clamp,
            reason = "min/max map NaN to the lower bound; clamp would propagate NaN."
        )]
        let font_size = (12.0 * scale).max(8.0).min(14.0) as u16;
        let label = &ship.name;
        let dims = measure_text(label, None, font_size, 1.0);
        draw_text(
            label,
            sx - dims.width / 2.0,
            bar_y + bar_h + dims.height + 2.0,
            f32::from(font_size),
            WHITE,
        );

        // Focus-fire reticle: draw targeting brackets if this ship is being targeted.
        if ship.attack_target.is_some() && ship.is_attacker == player_is_attacker {
            // Small arrow/marker near ship indicating it has orders.
            draw_circle_lines(sx, sy, half + 4.0, 1.5, Color::new(1.0, 0.6, 0.0, 0.6));
        }

        // Retreat indicator: pulsing "R" above ship.
        if ship.retreating {
            let retreat_alpha = 0.5 + 0.5 * (combat_tick as f32 * 0.3).sin().abs();
            draw_text(
                "RETREAT",
                sx - 20.0,
                sy - half - 14.0,
                10.0,
                Color::new(1.0, 0.5, 0.0, retreat_alpha),
            );
        }
    }

    // The original identifies every protected Escort target with a white box.
    // Use the same retained projections as hit testing so the marker remains
    // attached to the source object at every camera angle and LOD.
    let escort_targets = session
        .ships
        .iter()
        .filter(|ship| ship.alive && ship.order == TacticalOrder::Escort)
        .filter_map(|ship| ship.escort_target)
        .chain(
            session
                .fighters
                .iter()
                .filter(|fighter| fighter.alive && fighter.order == TacticalOrder::Escort)
                .filter_map(|fighter| fighter.escort_target),
        )
        .collect::<HashSet<_>>();
    for target in escort_targets {
        if let Some(projection) = ship_projections.get(&target) {
            draw_projected_frame(*projection, canvas.scale, WHITE);
        }
    }

    // The retained 3D submission and the interaction layer share these exact
    // projected bounds. This replaces the obsolete arena-coordinate frame.
    for (ship_index, projection) in &ship_projections {
        let Some(ship) = session.ships.get(*ship_index) else {
            continue;
        };
        if ship.alive && ship.selected {
            draw_projected_selection_frame(*projection, canvas.scale);
        }
    }

    // 5. Draw fighter squadrons.
    for (fighter_index, fighter) in session.fighters.iter().enumerate() {
        #[cfg(feature = "interface-test-fixtures")]
        let suppress_fixture_fallback =
            state.suppress_mapped_fighter_fallback && fighter.tactical_resource.is_some();
        #[cfg(not(feature = "interface-test-fixtures"))]
        let suppress_fixture_fallback = false;
        if !fighter.alive
            || rendered_fighter_indexes.contains(&fighter_index)
            || suppress_fixture_fallback
        {
            continue;
        }
        let fx = offset_x + fighter.x * scale;
        let fy = offset_y + fighter.y * scale;
        if !aperture.contains(fx, fy) {
            continue;
        }
        let size = FIGHTER_SIZE * scale;

        let color = if fighter.is_attacker {
            Color::new(0.4, 0.8, 1.0, 0.7)
        } else {
            Color::new(1.0, 0.5, 0.3, 0.7)
        };

        let half = size / 2.0;
        draw_triangle(
            macroquad::math::Vec2::new(fx, fy - half),
            macroquad::math::Vec2::new(fx - half, fy + half),
            macroquad::math::Vec2::new(fx + half, fy + half),
            color,
        );

        let label = format!("x{}", fighter.squad_count);
        #[expect(
            clippy::manual_clamp,
            reason = "min/max map NaN to the lower bound; clamp would propagate NaN."
        )]
        let font_size = (10.0 * scale).max(7.0).min(12.0) as u16;
        draw_text(
            &label,
            fx + half + 2.0,
            fy + 4.0,
            f32::from(font_size),
            color,
        );
    }

    for (fighter_index, projection) in &fighter_projections {
        let Some(fighter) = session.fighters.get(*fighter_index) else {
            continue;
        };
        if fighter.alive && fighter.selected {
            draw_projected_selection_frame(*projection, canvas.scale);
        }
    }

    // 6. The normal path above submits the executable's retained projectile
    // meshes. Keep this bounded 2D fallback only when the source camera cannot
    // render a projectile.
    for (effect_index, effect) in session.weapon_effects.iter().enumerate() {
        let source = macroquad::math::Vec2::new(
            offset_x + effect.fallback_source[0] * scale,
            offset_y + effect.fallback_source[1] * scale,
        );
        let target = session.ships.get(effect.target).map_or_else(
            || {
                macroquad::math::Vec2::new(
                    offset_x + effect.fallback_target[0] * scale,
                    offset_y + effect.fallback_target[1] * scale,
                )
            },
            |ship| {
                tactical_ship_screen_position(
                    effect.target,
                    ship,
                    &ship_projections,
                    scale,
                    offset_x,
                    offset_y,
                )
            },
        );

        let alpha = (1.0 - effect.progress()).clamp(0.0, 1.0);
        let base_color = effect.color();
        let color = Color::new(base_color.r, base_color.g, base_color.b, alpha);
        if !rendered_projectiles.contains(&effect_index) {
            let current = source.lerp(target, effect.progress());
            let travel = target - source;
            let direction = if travel.length_squared() > f32::EPSILON {
                travel.normalize()
            } else {
                macroquad::math::Vec2::X
            };
            let tail = current - direction * (2.0 * canvas.scale).max(1.0);
            draw_line(
                tail.x,
                tail.y,
                current.x,
                current.y,
                canvas.scale.max(1.0),
                color,
            );
        }

        // Retain the bounded fallback only when an authentic target frame
        // was unavailable for this event.
        if effect.progress() < 0.375 && !rendered_effect_targets.contains(&effect.target) {
            let flash_r = 4.0 + effect.progress() * 16.0;
            draw_circle(
                target.x,
                target.y,
                flash_r,
                Color::new(1.0, 1.0, 0.8, alpha * 0.6),
            );
        }
    }

    // 6b. Draw targeting lines for both typed source attack targets.
    if phase == BattlePhase::Combat {
        for (ship_index, ship) in session.ships.iter().enumerate() {
            if !ship.alive || !ship.selected || ship.is_attacker != player_is_attacker {
                continue;
            }
            let source = tactical_ship_screen_position(
                ship_index,
                ship,
                &ship_projections,
                scale,
                offset_x,
                offset_y,
            );
            let targets = if ship.manual_targets.is_empty() {
                ship.attack_target.into_iter().collect::<Vec<_>>()
            } else {
                ship.manual_targets.clone()
            };
            for target_spec in targets {
                match target_spec {
                    TacticalAttackTarget::CapitalShip(target_index) => {
                        let Some(target_ship) = session.ships.get(target_index) else {
                            continue;
                        };
                        if !target_ship.alive {
                            continue;
                        }
                        let target = tactical_ship_screen_position(
                            target_index,
                            target_ship,
                            &ship_projections,
                            scale,
                            offset_x,
                            offset_y,
                        );
                        let radius = ship_projections.get(&target_index).map_or(
                            DEFAULT_SHIP_SIZE * scale * 0.4,
                            |projection| {
                                (projection.max - projection.min).max_element() * 0.6 + 3.0
                            },
                        );
                        draw_tactical_attack_target(source, target, radius);
                    }
                    TacticalAttackTarget::FighterGroup(target_index) => {
                        let Some(target_fighter) = session.fighters.get(target_index) else {
                            continue;
                        };
                        if !target_fighter.alive {
                            continue;
                        }
                        let target = tactical_fighter_screen_position(
                            target_index,
                            target_fighter,
                            &fighter_projections,
                            scale,
                            offset_x,
                            offset_y,
                        );
                        let radius = fighter_projections.get(&target_index).map_or(
                            FIGHTER_SIZE * scale * 0.75,
                            |projection| {
                                (projection.max - projection.min).max_element() * 0.6 + 3.0
                            },
                        );
                        draw_tactical_attack_target(source, target, radius);
                    }
                }
            }
        }

        for (fighter_index, fighter) in session.fighters.iter().enumerate() {
            if !fighter.alive || !fighter.selected || fighter.is_attacker != player_is_attacker {
                continue;
            }
            let source = tactical_fighter_screen_position(
                fighter_index,
                fighter,
                &fighter_projections,
                scale,
                offset_x,
                offset_y,
            );
            match fighter.attack_target {
                Some(TacticalAttackTarget::CapitalShip(target_index)) => {
                    let Some(target_ship) = session.ships.get(target_index) else {
                        continue;
                    };
                    if !target_ship.alive {
                        continue;
                    }
                    let target = tactical_ship_screen_position(
                        target_index,
                        target_ship,
                        &ship_projections,
                        scale,
                        offset_x,
                        offset_y,
                    );
                    let radius = ship_projections
                        .get(&target_index)
                        .map_or(DEFAULT_SHIP_SIZE * scale * 0.4, |projection| {
                            (projection.max - projection.min).max_element() * 0.6 + 3.0
                        });
                    draw_tactical_attack_target(source, target, radius);
                }
                Some(TacticalAttackTarget::FighterGroup(target_index)) => {
                    let Some(target_fighter) = session.fighters.get(target_index) else {
                        continue;
                    };
                    if !target_fighter.alive {
                        continue;
                    }
                    let target = tactical_fighter_screen_position(
                        target_index,
                        target_fighter,
                        &fighter_projections,
                        scale,
                        offset_x,
                        offset_y,
                    );
                    let radius = fighter_projections
                        .get(&target_index)
                        .map_or(FIGHTER_SIZE * scale * 0.75, |projection| {
                            (projection.max - projection.min).max_element() * 0.6 + 3.0
                        });
                    draw_tactical_attack_target(source, target, radius);
                }
                None => {}
            }
        }
    }

    // End immutable borrow before mutable operations.
    let _ = session;

    // Restore full-canvas painting before HUD controls and any later egui pass.
    set_tactical_aperture_clip(None);

    // 6. Handle input per phase.
    if phase == BattlePhase::Placement {
        let session = state.session.as_mut().unwrap();
        handle_placement_input(
            session,
            &mut state.dragging_ship,
            &mut state.drag_offset,
            scale,
            offset_x,
            offset_y,
        );
    } else if phase == BattlePhase::Combat {
        let session = state.session.as_mut().unwrap();
        handle_combat_input(
            session,
            &mut state.death_star_targeting,
            scale,
            offset_x,
            offset_y,
            canvas.aperture(),
            &ship_projections,
            &navigation_projections,
        );
    }

    if phase == BattlePhase::Combat {
        action = handle_original_tactical_controls(state, bmp_cache, canvas);
        let pressed_control = pressed_tactical_hud_control(bmp_cache, canvas);
        let player_is_alliance = state
            .session
            .as_ref()
            .is_some_and(|session| session.player_is_attacker == session.attacker_is_alliance);
        let groups = TacticalGroupHud::from_session(state.session.as_ref().unwrap());
        let death_star = state.session.as_ref().and_then(player_death_star);
        let has_selection =
            groups.selected_fighter_group.is_some() || groups.selected_ship_name.is_some();
        let attack_death_star_enabled = groups.selected_fighter_group.is_some()
            && state.session.as_ref().is_some_and(|session| {
                session.death_star.is_some_and(|death_star| {
                    death_star.operational() && death_star.is_attacker != session.player_is_attacker
                })
            });
        let withdraw_enabled = state
            .session
            .as_ref()
            .is_some_and(tactical_withdraw_enabled);
        let pressed_options = state
            .battle_options_open
            .then(|| pressed_tactical_battle_options_control(bmp_cache, canvas, withdraw_enabled))
            .flatten()
            .filter(|control| state.battle_options_pressed == Some(*control));
        let pressed_withdraw_confirmation = state
            .withdraw_confirmation_open
            .then(|| pressed_tactical_withdraw_confirmation_control(bmp_cache, canvas));
        let pressed_command = (!state.battle_options_open
            && !state.withdraw_confirmation_open
            && !state.player_observing)
            .then(|| {
                pressed_tactical_command_control(
                    bmp_cache,
                    canvas,
                    state.command_panel,
                    has_selection,
                    groups.selected_fighter_group.is_some(),
                    attack_death_star_enabled,
                    player_is_alliance,
                )
            });
        draw_original_tactical_hud(
            bmp_cache,
            canvas,
            player_is_alliance,
            groups,
            attack_death_star_enabled,
            paused,
            state.highlight_alliance,
            state.highlight_empire,
            state.navigation_sets_visible,
            pressed_control,
            state.command_panel,
            pressed_command.flatten(),
            state.battle_options_open,
            state.withdraw_confirmation_open,
            state.player_observing,
            withdraw_enabled,
            pressed_options,
            pressed_withdraw_confirmation
                .flatten()
                .filter(|control| state.withdraw_confirmation_pressed == Some(*control)),
            death_star,
            state.death_star_laser_pressed,
            state.death_star_targeting,
        );
        if paused {
            let (x, y) = canvas.point(25.0, 48.0);
            draw_text("Battle Paused.", x, y, 16.0 * canvas.scale, RED);
        }
    }

    action
}

// ---------------------------------------------------------------------------
// Combat input handling
// ---------------------------------------------------------------------------

fn tactical_ship_screen_position(
    ship_index: usize,
    ship: &TacticalShip,
    projections: &HashMap<usize, TacticalScreenProjection>,
    scale: f32,
    offset_x: f32,
    offset_y: f32,
) -> macroquad::math::Vec2 {
    projections.get(&ship_index).map_or_else(
        || macroquad::math::Vec2::new(offset_x + ship.x * scale, offset_y + ship.y * scale),
        |projection| projection.center,
    )
}

fn tactical_fighter_screen_position(
    fighter_index: usize,
    fighter: &TacticalFighter,
    projections: &HashMap<usize, TacticalScreenProjection>,
    scale: f32,
    offset_x: f32,
    offset_y: f32,
) -> macroquad::math::Vec2 {
    projections.get(&fighter_index).map_or_else(
        || macroquad::math::Vec2::new(offset_x + fighter.x * scale, offset_y + fighter.y * scale),
        |projection| projection.center,
    )
}

#[derive(Debug, Clone, Copy)]
struct TacticalNavigationProjection {
    set: u8,
    point: u8,
    position: TacticalWorldPosition,
    screen: macroquad::math::Vec2,
}

fn tactical_navigation_projections(
    state: &TacticalState,
    aperture: NativeRect,
) -> Vec<TacticalNavigationProjection> {
    let Some(session) = state.session.as_ref() else {
        return Vec::new();
    };
    let aperture = (aperture.x, aperture.y, aperture.width, aperture.height);
    session
        .navigation_sets
        .iter()
        .enumerate()
        .filter(|(set, _)| state.navigation_sets_visible[*set])
        .flat_map(|(set, points)| {
            points
                .iter()
                .enumerate()
                .filter_map(move |(point, position)| {
                    state
                        .asset_renderer
                        .project_source_position(aperture, position.rendered())
                        .map(|screen| TacticalNavigationProjection {
                            set: u8::try_from(set).unwrap_or(3),
                            point: u8::try_from(point).unwrap_or(u8::MAX),
                            position: *position,
                            screen,
                        })
                })
        })
        .collect()
}

fn draw_tactical_navigation_points(
    session: &BattleSession,
    projections: &[TacticalNavigationProjection],
    canvas_scale: f32,
) {
    let colors = [RED, GREEN, BLUE, YELLOW];
    let selected_routes = session
        .ships
        .iter()
        .filter(|ship| ship.selected && ship.is_attacker == session.player_is_attacker)
        .flat_map(|ship| ship.navigation_route.iter().copied())
        .collect::<Vec<_>>();
    for projection in projections {
        let assigned = selected_routes.contains(&projection.position);
        let color = colors[usize::from(projection.set.min(3))];
        let radius = if assigned { 7.0 } else { 5.0 } * canvas_scale;
        let top = vec2(projection.screen.x, projection.screen.y - radius);
        let left = vec2(
            projection.screen.x - radius,
            projection.screen.y + radius * 0.75,
        );
        let right = vec2(
            projection.screen.x + radius,
            projection.screen.y + radius * 0.75,
        );
        draw_triangle(top, left, right, color);
        draw_triangle_lines(top, left, right, canvas_scale.max(1.0), WHITE);
        if assigned {
            draw_circle_lines(
                projection.screen.x,
                projection.screen.y,
                radius + 2.0 * canvas_scale,
                canvas_scale.max(1.0),
                WHITE,
            );
        }
    }
}

fn draw_tactical_attack_target(
    source: macroquad::math::Vec2,
    target: macroquad::math::Vec2,
    radius: f32,
) {
    draw_line(
        source.x,
        source.y,
        target.x,
        target.y,
        1.0,
        Color::new(1.0, 0.5, 0.0, 0.4),
    );
    draw_circle_lines(
        target.x,
        target.y,
        radius,
        1.5,
        Color::new(1.0, 0.3, 0.0, 0.7),
    );
}

fn draw_projected_selection_frame(projection: TacticalScreenProjection, canvas_scale: f32) {
    draw_projected_frame(projection, canvas_scale, Color::new(1.0, 0.82, 0.12, 0.95));
}

fn draw_projected_frame(projection: TacticalScreenProjection, canvas_scale: f32, color: Color) {
    let padding = 3.0 * canvas_scale;
    let minimum_size = 10.0 * canvas_scale;
    let minimum_half = macroquad::math::Vec2::splat(minimum_size * 0.5);
    let origin = (projection.min - macroquad::math::Vec2::splat(padding))
        .min(projection.center - minimum_half);
    let max = (projection.max + macroquad::math::Vec2::splat(padding))
        .max(projection.center + minimum_half);
    let size = max - origin;
    let corner = (size.min_element() * 0.28).clamp(3.0 * canvas_scale, 8.0 * canvas_scale);
    let thickness = canvas_scale.max(1.0);
    for (x, x_direction) in [(origin.x, 1.0), (origin.x + size.x, -1.0)] {
        for (y, y_direction) in [(origin.y, 1.0), (origin.y + size.y, -1.0)] {
            draw_line(x, y, x + corner * x_direction, y, thickness, color);
            draw_line(x, y, x, y + corner * y_direction, thickness, color);
        }
    }
}

/// Handle mouse input during the combat phase.
///
/// - Left-click: select a player ship.
/// - Right-click on friendly: issue the original order-1 Escort command.
/// - Right-click on enemy: issue focus-fire order to all selected player ships.
/// - R key: retreat selected ships.
fn handle_combat_input(
    session: &mut BattleSession,
    death_star_targeting: &mut bool,
    scale: f32,
    offset_x: f32,
    offset_y: f32,
    aperture: NativeRect,
    projections: &HashMap<usize, TacticalScreenProjection>,
    navigation_projections: &[TacticalNavigationProjection],
) {
    let (mx, my) = mouse_position();
    let inside_battle = aperture.contains(mx, my);
    let arena_pointer_x = (mx - offset_x) / scale;
    let arena_pointer_y = (my - offset_y) / scale;
    let hit_radius = DEFAULT_SHIP_SIZE * 0.6;
    let screen_pointer = macroquad::math::Vec2::new(mx, my);
    let projected_hit_radius = 7.0 * (aperture.width / BATTLE_APERTURE.width);
    let navigation_hit = navigation_projections.iter().find(|projection| {
        projection.screen.distance(screen_pointer) <= 8.0 * (aperture.width / BATTLE_APERTURE.width)
    });
    let control = is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);

    if inside_battle && is_mouse_button_pressed(MouseButton::Left) && control {
        if let Some(navigation) = navigation_hit {
            let mut changed = 0;
            for ship in &mut session.ships {
                if !ship.selected || !ship.alive || ship.is_attacker != session.player_is_attacker {
                    continue;
                }
                let before = ship.navigation_route.len();
                ship.navigation_route
                    .retain(|point| *point != navigation.position);
                if before != ship.navigation_route.len() {
                    changed += 1;
                    ship.source_waypoint = ship.navigation_route.first().copied();
                    ship.source_desired_forward = ship
                        .source_waypoint
                        .map_or(TacticalWorldVector::ZERO, |waypoint| {
                            source_delta(waypoint, ship.source_position).normalized()
                        });
                }
            }
            macroquad::logging::info!(
                "[tactical_navigation] event=route_remove set={} point={} members={}",
                navigation.set + 1,
                navigation.point + 1,
                changed,
            );
            return;
        }
    }

    // Left-click: select player's ship.
    if inside_battle && is_mouse_button_pressed(MouseButton::Left) {
        let mut hit = None;
        for (i, ship) in session.ships.iter().enumerate() {
            if !ship.alive || ship.retreating {
                continue;
            }
            if ship.is_attacker != session.player_is_attacker {
                continue;
            }
            let projected_hit = projections.get(&i).is_some_and(|projection| {
                projection.contains(screen_pointer, projected_hit_radius)
            });
            let dx = arena_pointer_x - ship.x;
            let dy = arena_pointer_y - ship.y;
            if projected_hit
                || (!projections.contains_key(&i) && dx * dx + dy * dy < hit_radius * hit_radius)
            {
                hit = Some(i);
                break;
            }
        }

        if let Some(idx) = hit {
            // The original manual uses Ctrl to build a multi-ship selection
            // before assigning those ships to a task force.
            if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
                session.ships[idx].selected = !session.ships[idx].selected;
            } else {
                for s in &mut session.ships {
                    s.selected = false;
                }
                session.ships[idx].selected = true;
            }
            session.selected_ship = Some(idx);
            session.selected_fighter_group = None;
            for fighter in &mut session.fighters {
                fighter.selected = false;
            }
            macroquad::logging::info!(
                "[tactical_3d] selection object_id={} source_projection={}",
                idx + 1,
                projections.contains_key(&idx)
            );
        } else {
            // Clicked empty space — deselect all.
            for s in &mut session.ships {
                s.selected = false;
            }
            session.selected_ship = None;
            session.selected_fighter_group = None;
            for fighter in &mut session.fighters {
                fighter.selected = false;
            }
        }
    }

    // Right-click: assign an armed Death Star superlaser or issue focus fire.
    if inside_battle && is_mouse_button_pressed(MouseButton::Right) {
        if let Some(navigation) = navigation_hit {
            let mut assigned = 0;
            let mut longest_route = 0;
            for ship in &mut session.ships {
                if !ship.selected || !ship.alive || ship.is_attacker != session.player_is_attacker {
                    continue;
                }
                ship.order = TacticalOrder::None;
                ship.escort_target = None;
                if control {
                    if !ship.navigation_route.contains(&navigation.position) {
                        ship.navigation_route.push(navigation.position);
                    }
                } else {
                    ship.navigation_route.clear();
                    ship.navigation_route.push(navigation.position);
                }
                ship.source_waypoint = ship.navigation_route.first().copied();
                ship.source_desired_forward = ship
                    .source_waypoint
                    .map_or(TacticalWorldVector::ZERO, |waypoint| {
                        source_delta(waypoint, ship.source_position).normalized()
                    });
                assigned += 1;
                longest_route = longest_route.max(ship.navigation_route.len());
            }
            macroquad::logging::info!(
                "[tactical_navigation] event=route_assign set={} point={} append={} members={} route_len={}",
                navigation.set + 1,
                navigation.point + 1,
                control,
                assigned,
                longest_route,
            );
            return;
        }
        let mut target_hit = None;
        for (i, ship) in session.ships.iter().enumerate() {
            if !ship.alive {
                continue;
            }
            // Can only target enemy ships.
            if ship.is_attacker == session.player_is_attacker {
                continue;
            }
            let projected_hit = projections.get(&i).is_some_and(|projection| {
                projection.contains(screen_pointer, projected_hit_radius)
            });
            let dx = arena_pointer_x - ship.x;
            let dy = arena_pointer_y - ship.y;
            if projected_hit
                || (!projections.contains_key(&i) && dx * dx + dy * dy < hit_radius * hit_radius)
            {
                target_hit = Some(i);
                break;
            }
        }

        if *death_star_targeting {
            if let Some(target_idx) = target_hit {
                if session.commit_death_star_shot(target_idx) {
                    *death_star_targeting = false;
                }
            }
            return;
        }

        let friendly_hit = session.ships.iter().enumerate().find_map(|(index, ship)| {
            if !ship.alive || ship.retreating || ship.is_attacker != session.player_is_attacker {
                return None;
            }
            let projected_hit = projections.get(&index).is_some_and(|projection| {
                projection.contains(screen_pointer, projected_hit_radius)
            });
            let dx = arena_pointer_x - ship.x;
            let dy = arena_pointer_y - ship.y;
            (projected_hit
                || (!projections.contains_key(&index)
                    && dx * dx + dy * dy < hit_radius * hit_radius))
                .then_some(index)
        });
        if let Some(target_idx) = friendly_hit {
            let (capital_members, fighter_members) = assign_selected_escort(session, target_idx);
            if capital_members + fighter_members > 0 {
                macroquad::logging::info!(
                    "[tactical_escort] event=assigned order=1 target_object_id={} capital_members={} fighter_members={} source_projection={}",
                    target_idx + 1,
                    capital_members,
                    fighter_members,
                    projections.contains_key(&target_idx),
                );
            }
            return;
        }

        if let Some(target_idx) = target_hit {
            // Assign one or several ordered focus targets to selected ships.
            match assign_selected_capital_focus_target(session, target_idx, control) {
                TacticalFocusAssignment::Assigned {
                    object_ids,
                    maximum_targets,
                } => {
                    macroquad::logging::info!(
                        "[tactical_3d] focus source_object_ids={} target_object_id={} source_projection={}",
                        object_ids
                            .iter()
                            .map(usize::to_string)
                            .collect::<Vec<_>>()
                            .join(","),
                        target_idx + 1,
                        projections.contains_key(&target_idx)
                            && object_ids
                                .iter()
                                .all(|object_id| projections.contains_key(&(*object_id - 1)))
                    );
                    macroquad::logging::info!(
                        "[tactical_navigation] event=target_assign status=assigned append={} target_object_id={} members={} target_count={}",
                        control,
                        target_idx + 1,
                        object_ids.len(),
                        maximum_targets,
                    );
                }
                TacticalFocusAssignment::RejectedMixedTaskForces { member_count } => {
                    macroquad::logging::info!(
                        "[tactical_navigation] event=target_assign status=rejected_mixed_task_forces append={} target_object_id={} members={}",
                        control,
                        target_idx + 1,
                        member_count,
                    );
                }
            }
        } else {
            // Right-clicked empty space — clear focus targets for selected ships.
            for ship in &mut session.ships {
                if ship.selected && ship.is_attacker == session.player_is_attacker {
                    ship.attack_target = None;
                    ship.manual_targets.clear();
                    ship.escort_target = None;
                    if ship.order == TacticalOrder::Escort {
                        ship.order = TacticalOrder::None;
                    }
                }
            }
        }
    }

    // R key: retreat selected ships.
    if is_key_pressed(KeyCode::R) {
        for ship in &mut session.ships {
            if ship.selected && ship.is_attacker == session.player_is_attacker && ship.alive {
                ship.retreating = true;
                ship.selected = false;
            }
        }
        session.selected_ship = None;
        session.selected_fighter_group = None;
    }
}

// ---------------------------------------------------------------------------
// Placement input handling
// ---------------------------------------------------------------------------

fn handle_placement_input(
    session: &mut BattleSession,
    dragging_ship: &mut Option<usize>,
    drag_offset: &mut (f32, f32),
    scale: f32,
    offset_x: f32,
    offset_y: f32,
) {
    let (mx, my) = mouse_position();

    // Convert mouse to arena coordinates.
    let arena_pointer_x = (mx - offset_x) / scale;
    let arena_pointer_y = (my - offset_y) / scale;

    // Deployment zone bounds for the player's side.
    let (zone_min_x, zone_max_x) = if session.player_is_attacker {
        (0.0, ARENA_WIDTH * DEPLOY_ZONE_FRACTION)
    } else {
        (ARENA_WIDTH * (1.0 - DEPLOY_ZONE_FRACTION), ARENA_WIDTH)
    };

    if is_mouse_button_pressed(MouseButton::Left) {
        // Check if clicking on a player's ship.
        let mut hit = None;
        for (i, ship) in session.ships.iter().enumerate() {
            if !ship.alive {
                continue;
            }
            if ship.is_attacker != session.player_is_attacker {
                continue;
            }
            let dx = arena_pointer_x - ship.x;
            let dy = arena_pointer_y - ship.y;
            if dx * dx + dy * dy < (DEFAULT_SHIP_SIZE * 0.6).powi(2) {
                hit = Some(i);
                break;
            }
        }

        if let Some(idx) = hit {
            *dragging_ship = Some(idx);
            *drag_offset = (
                session.ships[idx].x - arena_pointer_x,
                session.ships[idx].y - arena_pointer_y,
            );
            // Select this ship.
            for s in &mut session.ships {
                s.selected = false;
            }
            session.ships[idx].selected = true;
            session.selected_ship = Some(idx);
        } else {
            // Deselect.
            for s in &mut session.ships {
                s.selected = false;
            }
            session.selected_ship = None;
        }
    }

    if is_mouse_button_down(MouseButton::Left) {
        if let Some(idx) = *dragging_ship {
            let new_x = (arena_pointer_x + drag_offset.0).clamp(
                zone_min_x + DEFAULT_SHIP_SIZE * 0.5,
                zone_max_x - DEFAULT_SHIP_SIZE * 0.5,
            );
            let new_y = (arena_pointer_y + drag_offset.1).clamp(
                DEFAULT_SHIP_SIZE * 0.5,
                ARENA_HEIGHT - DEFAULT_SHIP_SIZE * 0.5,
            );
            session.ships[idx].x = new_x;
            session.ships[idx].y = new_y;
        }
    }

    if is_mouse_button_released(MouseButton::Left) {
        *dragging_ship = None;
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_core::tactical::test_support::{test_fighter, test_session, test_ship};

    // Source: FUN_005a8f70 waypoint; FUN_005cf980 recovery executor.
    #[test]
    fn committed_maneuver_hold_and_recover_enter_original_executors() {
        let mut player = test_ship(64, 0, true, true);
        player.selected = true;
        player.source_position.z = -50.0;
        player.fighter_capacity = 2;
        let mut enemy = test_ship(128, 0, false, true);
        enemy.source_position = TacticalWorldPosition {
            x: 10.0,
            y: 4.0,
            z: 50.0,
        };
        let mut fighter = test_fighter(1, true);
        fighter.selected = true;
        let mut session = test_session(vec![player, enemy], vec![fighter], true);
        session.selected_ship = Some(0);

        assert_eq!(
            assign_selected_command(
                &mut session,
                TacticalOrder::LeftHook,
                Some(TacticalTactic::Surround),
            ),
            (1, 0)
        );
        assert_eq!(session.ships[0].order, TacticalOrder::LeftHook);
        assert_eq!(session.ships[0].tactic, TacticalTactic::Surround);
        assert!(session.ships[0].source_waypoint.is_some());
        assert_ne!(
            session.ships[0].source_desired_forward,
            TacticalWorldVector::ZERO
        );

        assert_eq!(
            assign_selected_command(&mut session, TacticalOrder::HoldPosition, None),
            (1, 0)
        );
        assert_eq!(session.ships[0].source_waypoint, None);
        assert_eq!(
            session.ships[0].source_desired_forward,
            TacticalWorldVector::ZERO
        );
        assert_eq!(session.ships[0].source_velocity, TacticalWorldVector::ZERO);

        session.ships[0].selected = false;
        session.selected_ship = None;
        session.selected_fighter_group = Some(0);
        assert_eq!(
            assign_selected_command(&mut session, TacticalOrder::Recover, None),
            (0, 1)
        );
        assert_eq!(session.fighters[0].order, TacticalOrder::Recover);
        assert_eq!(
            session.fighters[0].recovery_state,
            TacticalFighterRecoveryState::Returning
        );
        assert_eq!(session.fighters[0].recovery_target, Some(0));
    }

    // Source: FUN_005d0bb0 / FUN_005a8fc0 attack target acquisition.
    #[test]
    fn attack_orders_acquire_first_eligible_typed_target_for_both_unit_classes() {
        let mut player_ship = test_ship(64, 0, true, true);
        player_ship.selected = true;
        let dead_enemy_ship = test_ship(128, 0, false, false);
        let live_enemy_ship = test_ship(129, 1, false, true);

        let mut player_fighter = test_fighter(1, true);
        player_fighter.fighter_group = 0;
        let mut recovered_enemy_fighter = test_fighter(5, false);
        recovered_enemy_fighter.recovery_state = TacticalFighterRecoveryState::Recovered;
        let live_enemy_fighter = test_fighter(6, false);
        let mut session = test_session(
            vec![player_ship, dead_enemy_ship, live_enemy_ship],
            vec![player_fighter, recovered_enemy_fighter, live_enemy_fighter],
            true,
        );
        session.selected_ship = Some(0);

        assert_eq!(
            assign_selected_command(&mut session, TacticalOrder::AttackCapitalShips, None,),
            (1, 0)
        );
        assert_eq!(
            session.ships[0].attack_target,
            Some(TacticalAttackTarget::CapitalShip(2))
        );

        assert_eq!(
            assign_selected_command(&mut session, TacticalOrder::AttackFighters, None),
            (1, 0)
        );
        assert_eq!(
            session.ships[0].attack_target,
            Some(TacticalAttackTarget::FighterGroup(2))
        );

        session.ships[0].selected = false;
        session.selected_ship = None;
        session.fighters[0].selected = true;
        session.selected_fighter_group = Some(0);
        assert_eq!(
            assign_selected_command(&mut session, TacticalOrder::AttackFighters, None),
            (0, 1)
        );
        assert_eq!(
            session.fighters[0].attack_target,
            Some(TacticalAttackTarget::FighterGroup(2))
        );
    }

    // Source: FUN_005d0bb0 attack target replacement.
    #[test]
    fn attack_executor_preserves_live_engagement_and_replaces_invalid_target() {
        let mut player = test_ship(64, 0, true, true);
        player.selected = true;
        player.attack_target = Some(TacticalAttackTarget::CapitalShip(2));
        let first_enemy = test_ship(128, 0, false, true);
        let second_enemy = test_ship(129, 1, false, true);
        let mut session = test_session(vec![player, first_enemy, second_enemy], Vec::new(), true);
        session.selected_ship = Some(0);

        assign_selected_command(&mut session, TacticalOrder::AttackCapitalShips, None);
        assert_eq!(
            session.ships[0].attack_target,
            Some(TacticalAttackTarget::CapitalShip(2))
        );

        session.ships[2].alive = false;
        assign_selected_command(&mut session, TacticalOrder::AttackCapitalShips, None);
        assert_eq!(
            session.ships[0].attack_target,
            Some(TacticalAttackTarget::CapitalShip(1))
        );
    }

    #[cfg(feature = "interface-test-fixtures")]
    #[test]
    fn tactical_audio_fixture_queues_every_weapon_and_voice_resource_once() {
        // The fixture mirrors FUN_005bae60's complete source-event mapping,
        // grouped by the two owned resource DLLs for deterministic loading.
        let mut state = TacticalState {
            session: Some(test_session(Vec::new(), Vec::new(), true)),
            ..TacticalState::default()
        };

        state.configure_tactical_audio_fixture();

        let session = state.session.as_mut().unwrap();
        assert_eq!(
            session
                .take_pending_audio_cues()
                .into_iter()
                .map(|cue| cue.resource_id)
                .collect::<Vec<_>>(),
            (13_033..=13_054).collect::<Vec<_>>()
        );
        let voice_resources = session
            .take_pending_voice_cues()
            .into_iter()
            .map(|cue| cue.resource_id)
            .collect::<Vec<_>>();
        let expected = (14_001..=14_122)
            .chain(15_133..=15_163)
            .chain(15_001..=15_132)
            .collect::<Vec<_>>();
        assert_eq!(voice_resources, expected);
        assert_eq!(voice_resources.len(), 285);
    }

    #[test]
    fn original_tactical_canvas_letterboxes_without_distorting_controls() {
        let native = TacticalCanvas::new(640.0, 480.0);
        assert_eq!((native.x, native.y, native.scale), (0.0, 0.0, 1.0));
        let wide = TacticalCanvas::new(1280.0, 800.0);
        assert_eq!(wide.scale, 5.0 / 3.0);
        assert_eq!(wide.x, (1280.0 - 640.0 * wide.scale) * 0.5);
        let (screen_x, screen_y) = wide.point(574.0, 317.0);
        let (logical_x, logical_y) = wide.logical_pointer(screen_x, screen_y);
        assert!((logical_x - 574.0).abs() < 0.001);
        assert!((logical_y - 317.0).abs() < 0.001);
    }

    // Source: FUN_005f0f40 control rect boundaries.
    #[test]
    fn original_tactical_control_rects_stop_at_bitmap_edges() {
        assert_eq!(
            tactical_hud_rect_control_at(560.0, 307.0),
            Some(TacticalHudControl::Pause)
        );
        assert_eq!(
            tactical_hud_rect_control_at(587.9, 327.9),
            Some(TacticalHudControl::Pause)
        );
        assert_eq!(tactical_hud_rect_control_at(588.0, 317.0), None);
        assert_eq!(tactical_hud_rect_control_at(559.9, 317.0), None);
        assert_eq!(
            tactical_hud_rect_control_at(486.0, 343.0),
            Some(TacticalHudControl::ZoomIn)
        );
        assert_eq!(tactical_hud_rect_control_at(510.0, 355.0), None);
        assert_eq!(
            tactical_hud_rect_control_at(603.0, 343.0),
            Some(TacticalHudControl::ZoomOut)
        );
        assert_eq!(
            tactical_hud_rect_control_at(517.0, 304.0),
            Some(TacticalHudControl::HighlightAlliance)
        );
        assert_eq!(
            tactical_hud_rect_control_at(482.0, 304.0),
            Some(TacticalHudControl::HighlightEmpire)
        );
        assert_eq!(
            tactical_hud_rect_control_at(485.0, 272.0),
            Some(TacticalHudControl::NavigationSet(0))
        );
        assert_eq!(
            tactical_hud_rect_control_at(627.9, 298.9),
            Some(TacticalHudControl::NavigationSet(3))
        );
        assert_eq!(tactical_hud_rect_control_at(628.0, 285.0), None);
        assert_eq!(
            tactical_hud_rect_control_at(484.0, 430.0),
            Some(TacticalHudControl::CameraRecall)
        );
        assert_eq!(
            tactical_hud_rect_control_at(627.9, 454.9),
            Some(TacticalHudControl::CameraMemorize)
        );
        assert_eq!(tactical_hud_rect_control_at(628.0, 442.0), None);
    }

    // Source: FUN_005f0f40 / FUN_005f1050 bitmap hit masks.
    #[test]
    #[ignore = "requires extracted original bitmaps in data/base/ui"]
    fn original_tactical_controls_use_source_bitmap_hit_masks() {
        let mut cache = BmpCache::new();
        cache.set_base_path(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/base/ui"),
        );

        // Original control hit testing excludes every outer edge, even when
        // the containing rectangle starts there.
        assert_eq!(tactical_hud_control_at(&mut cache, 560.0, 307.0), None);
        assert_eq!(
            tactical_hud_control_at(&mut cache, 562.1, 308.1),
            Some(TacticalHudControl::Pause)
        );

        // Both zoom controls contain transparent matte pixels inside their
        // bounding rectangles. Those pixels must pass through untouched.
        assert_eq!(tactical_hud_control_at(&mut cache, 487.1, 344.1), None);
        assert_eq!(
            tactical_hud_control_at(&mut cache, 491.1, 344.1),
            Some(TacticalHudControl::ZoomIn)
        );
        assert_eq!(tactical_hud_control_at(&mut cache, 604.1, 344.1), None);
        assert_eq!(
            tactical_hud_control_at(&mut cache, 609.1, 344.1),
            Some(TacticalHudControl::ZoomOut)
        );
        assert_eq!(
            tactical_hud_control_at(&mut cache, 520.1, 397.1),
            Some(TacticalHudControl::CameraLeft)
        );
        assert_eq!(
            tactical_hud_control_at(&mut cache, 590.1, 397.1),
            Some(TacticalHudControl::CameraRight)
        );
        assert_eq!(
            tactical_hud_control_at(&mut cache, 558.1, 354.1),
            Some(TacticalHudControl::CameraUp)
        );
        assert_eq!(
            tactical_hud_control_at(&mut cache, 558.1, 444.1),
            Some(TacticalHudControl::CameraDown)
        );

        assert_eq!(
            tactical_hud_control_at(&mut cache, 518.1, 305.1),
            Some(TacticalHudControl::HighlightAlliance)
        );
        assert_eq!(
            tactical_hud_control_at(&mut cache, 483.1, 305.1),
            Some(TacticalHudControl::HighlightEmpire)
        );
        assert_eq!(tactical_hud_control_at(&mut cache, 588.0, 317.0), None);
        assert_eq!(
            tactical_hud_control_at(&mut cache, 616.0, 318.0),
            Some(TacticalHudControl::BattleOptions)
        );

        assert_eq!(
            tactical_battle_options_control_at(&mut cache, true, 521.0, 64.0),
            Some(TacticalBattleOptionsControl::Withdraw)
        );
        assert_eq!(
            tactical_battle_options_control_at(&mut cache, false, 521.0, 64.0),
            None
        );
        assert_eq!(
            tactical_battle_options_control_at(&mut cache, true, 519.0, 126.0),
            Some(TacticalBattleOptionsControl::SimulateRemainder)
        );
        assert_eq!(
            tactical_battle_options_control_at(&mut cache, true, 583.0, 126.0),
            Some(TacticalBattleOptionsControl::ObserveToggle)
        );
        assert_eq!(
            tactical_battle_options_control_at(&mut cache, true, 520.0, 219.0),
            Some(TacticalBattleOptionsControl::GameOptions)
        );
        assert_eq!(
            tactical_battle_options_control_at(&mut cache, true, 579.0, 219.0),
            Some(TacticalBattleOptionsControl::Close)
        );
        assert_eq!(
            tactical_withdraw_confirmation_control_at(&mut cache, 560.0, 242.0),
            Some(TacticalWithdrawConfirmationControl::Confirm)
        );
        assert_eq!(
            tactical_withdraw_confirmation_control_at(&mut cache, 600.0, 242.0),
            Some(TacticalWithdrawConfirmationControl::Cancel)
        );
        assert_eq!(
            tactical_withdraw_confirmation_control_at(&mut cache, 547.0, 229.0),
            None
        );
        assert_eq!(
            tactical_withdraw_confirmation_control_at(&mut cache, 614.0, 242.0),
            None
        );

        assert_eq!(
            tactical_hud_control_at(&mut cache, 72.1, 10.1),
            Some(TacticalHudControl::TaskForce(0))
        );
        assert_eq!(
            tactical_hud_control_at(&mut cache, 254.1, 10.1),
            Some(TacticalHudControl::TaskForce(7))
        );
        assert_eq!(tactical_hud_control_at(&mut cache, 60.1, 2.1), None);
        assert_eq!(
            tactical_hud_control_at(&mut cache, 342.1, 10.1),
            Some(TacticalHudControl::FighterGroup(0))
        );
        assert_eq!(
            tactical_hud_control_at(&mut cache, 420.1, 10.1),
            Some(TacticalHudControl::FighterGroup(3))
        );
    }

    // Source: FUN_005f0c20 builds TACTICAL 1310, FUN_005f0f40 places confirm and
    // cancel, and FUN_005f1170 dispatches the withdrawal on confirm (P58-B13).
    #[test]
    fn withdrawal_requires_native_confirmation_before_dispatch() {
        let mut state = TacticalState {
            session: Some(test_session(Vec::new(), Vec::new(), true)),
            ..TacticalState::default()
        };
        state.battle_options_open = true;

        assert_eq!(
            activate_tactical_battle_options(&mut state, TacticalBattleOptionsControl::Withdraw,),
            TacticalAction::None
        );
        assert!(!state.battle_options_open);
        assert!(state.withdraw_confirmation_open);

        assert_eq!(
            activate_tactical_withdraw_confirmation(
                &mut state,
                TacticalWithdrawConfirmationControl::Cancel,
            ),
            TacticalAction::None
        );
        assert!(!state.withdraw_confirmation_open);
        assert!(state
            .session
            .as_ref()
            .unwrap()
            .pending_voice_cues
            .is_empty());

        state.battle_options_open = true;
        assert_eq!(
            activate_tactical_battle_options(&mut state, TacticalBattleOptionsControl::Withdraw,),
            TacticalAction::None
        );
        assert_eq!(
            activate_tactical_withdraw_confirmation(
                &mut state,
                TacticalWithdrawConfirmationControl::Confirm,
            ),
            TacticalAction::WithdrawFromBattle
        );
        assert!(!state.withdraw_confirmation_open);
        assert_eq!(
            state.session.as_mut().unwrap().take_pending_voice_cues(),
            vec![TacticalVoiceCue::withdrawal_started(
                TacticalVoiceFaction::Alliance
            )]
        );

        let mut operational = test_ship(64, 0, false, true);
        operational.is_attacker = false;
        operational.subsystem_capacity.hyperdrive = 1;
        operational.refresh_subsystem_condition();
        let mut stranded = test_ship(64, 1, false, true);
        stranded.is_attacker = false;
        stranded.subsystem_capacity.hyperdrive = 1;
        stranded.subsystem_damage.hyperdrive = 1;
        stranded.refresh_subsystem_condition();
        let mut empire_state = TacticalState {
            session: Some(test_session(vec![operational, stranded], Vec::new(), false)),
            withdraw_confirmation_open: true,
            ..TacticalState::default()
        };
        assert_eq!(
            activate_tactical_withdraw_confirmation(
                &mut empire_state,
                TacticalWithdrawConfirmationControl::Confirm,
            ),
            TacticalAction::WithdrawFromBattle
        );
        assert_eq!(
            empire_state
                .session
                .as_mut()
                .unwrap()
                .take_pending_voice_cues(),
            vec![
                TacticalVoiceCue::withdrawal_started(TacticalVoiceFaction::Empire),
                TacticalVoiceCue::withdrawal_hyperdrive_warning(TacticalVoiceFaction::Empire),
            ]
        );
    }

    // Source: REBDLG.DLL resource IDs 10710-10973 battle alert frames.
    #[test]
    fn battle_alert_skins_match_recovered_faction_resources_and_geometry() {
        let alliance = battle_alert_skin(true);
        assert_eq!(alliance.frame, 10_710);
        assert_eq!(alliance.scene, 10_712);
        assert_eq!(alliance.rail, 10_820);
        assert_eq!(alliance.tabs[0], [10_728, 10_729]);
        assert_eq!(alliance.tabs[3], [10_734, 10_735]);
        assert_eq!(alliance.buttons[0], [10_971, 10_972, 10_973]);
        assert_eq!(alliance.buttons[2], [10_719, 10_720, 10_721]);
        assert_eq!(
            alliance.tab_rects,
            [
                NativeRect::new(418.0, 21.0, 41.0, 41.0),
                NativeRect::new(418.0, 81.0, 41.0, 41.0),
                NativeRect::new(418.0, 143.0, 41.0, 41.0),
                NativeRect::new(418.0, 205.0, 41.0, 41.0),
            ]
        );

        let empire = battle_alert_skin(false);
        assert_eq!(empire.frame, 10_711);
        assert_eq!(empire.scene, 10_713);
        assert_eq!(empire.rail, 10_821);
        assert_eq!(empire.tabs[0], [10_738, 10_739]);
        assert_eq!(empire.tabs[3], [10_744, 10_745]);
        assert_eq!(empire.buttons[0], [10_974, 10_975, 10_976]);
        assert_eq!(empire.buttons[2], [10_725, 10_726, 10_727]);
        assert_eq!(
            empire.tab_rects,
            [
                NativeRect::new(426.0, 17.0, 44.0, 41.0),
                NativeRect::new(426.0, 80.0, 44.0, 41.0),
                NativeRect::new(426.0, 143.0, 44.0, 41.0),
                NativeRect::new(426.0, 206.0, 44.0, 41.0),
            ]
        );
    }

    // Source: manual pages 143 and 148, task-force and fighter-group selection (P58E).
    #[test]
    fn task_force_and_fighter_group_selection_are_mutually_exclusive() {
        let mut ships = (0..8)
            .map(|index| test_ship(64, index, true, true))
            .collect::<Vec<_>>();
        ships[0].selected = true;
        let mut fighters = (0..4)
            .map(|index| {
                let mut fighter = test_fighter(1, true);
                fighter.fighter_group = index;
                fighter.identity.fleet_roster_index = usize::from(index);
                fighter.fleet_fighter_index = usize::from(index);
                fighter
            })
            .collect::<Vec<_>>();
        fighters.push(test_fighter(5, false));
        let mut session = test_session(ships, fighters, true);
        session.selected_ship = Some(0);

        assert_eq!(select_task_force(&mut session, 7), 1);
        assert_eq!(selected_player_task_force(&session), Some(7));
        assert_eq!(session.selected_ship, Some(7));

        assert!(select_fighter_group(&mut session, 3));
        assert_eq!(session.selected_ship, None);
        assert_eq!(session.selected_fighter_group, Some(3));
        assert!(session.ships.iter().all(|ship| !ship.selected));
        assert_eq!(
            session
                .fighters
                .iter()
                .filter(|fighter| fighter.selected)
                .count(),
            1
        );

        assert_eq!(select_task_force(&mut session, 0), 1);
        assert_eq!(session.selected_fighter_group, None);
        assert!(session.fighters.iter().all(|fighter| !fighter.selected));
    }

    // Source: manual pages 143 and 148, the Ctrl task-force assignment rule (P58E).
    #[test]
    fn task_force_assignment_accepts_only_an_existing_or_next_blank_slot() {
        let mut ships = vec![
            test_ship(64, 0, true, true),
            test_ship(65, 1, true, true),
            test_ship(128, 0, false, true),
        ];
        ships[0].selected = true;
        ships[1].selected = true;
        let mut session = test_session(ships, Vec::new(), true);

        assert_eq!(assign_selected_to_task_force(&mut session, 4), 0);
        assert_eq!(session.ships[0].task_force, 0);
        assert_eq!(session.ships[1].task_force, 1);
        assert_eq!(assign_selected_to_task_force(&mut session, 2), 2);
        assert_eq!(session.ships[0].task_force, 2);
        assert_eq!(session.ships[1].task_force, 2);
    }

    #[test]
    fn selected_capital_navigation_wraps_within_the_current_task_force() {
        let mut ships = vec![
            test_ship(64, 0, true, true),
            test_ship(65, 1, true, true),
            test_ship(66, 2, true, true),
            test_ship(128, 0, false, true),
        ];
        for ship in &mut ships[..3] {
            ship.task_force = 2;
            ship.selected = true;
        }
        let mut session = test_session(ships, vec![test_fighter(1, true)], true);
        session.selected_ship = Some(0);

        assert_eq!(cycle_selected_capital(&mut session, -1), Some((0, 2, 2)));
        assert_eq!(session.selected_ship, Some(2));
        assert_eq!(
            session
                .ships
                .iter()
                .enumerate()
                .filter_map(|(index, ship)| ship.selected.then_some(index))
                .collect::<Vec<_>>(),
            vec![2]
        );
        assert_eq!(cycle_selected_capital(&mut session, 1), Some((2, 0, 2)));
        assert_eq!(cycle_selected_capital(&mut session, 1), Some((0, 1, 2)));
        assert!(session.fighters.iter().all(|fighter| !fighter.selected));
    }

    #[test]
    fn alliance_mixed_task_forces_reject_focus_target_without_mutating_orders() {
        // FUN_005a24d0 queues source event 0x84 when the selected Alliance
        // capitals cannot resolve to one task-force ordinal.
        let mut first = test_ship(64, 0, true, true);
        first.selected = true;
        first.task_force = 1;
        first.manual_targets = vec![TacticalAttackTarget::CapitalShip(3)];
        first.attack_target = first.manual_targets.first().copied();
        first.escort_target = Some(3);
        let mut second = test_ship(65, 1, true, true);
        second.selected = true;
        second.task_force = 2;
        second.manual_targets = vec![TacticalAttackTarget::CapitalShip(4)];
        second.attack_target = second.manual_targets.first().copied();
        second.escort_target = Some(4);
        let hostile = test_ship(128, 2, false, true);
        let mut session = test_session(vec![first, second, hostile], Vec::new(), true);
        let before = session.ships[..2]
            .iter()
            .map(|ship| {
                (
                    ship.manual_targets.clone(),
                    ship.attack_target,
                    ship.escort_target,
                )
            })
            .collect::<Vec<_>>();

        let result = assign_selected_capital_focus_target(&mut session, 2, false);

        assert_eq!(
            result,
            TacticalFocusAssignment::RejectedMixedTaskForces { member_count: 2 }
        );
        assert_eq!(
            session.ships[..2]
                .iter()
                .map(|ship| (
                    ship.manual_targets.clone(),
                    ship.attack_target,
                    ship.escort_target
                ))
                .collect::<Vec<_>>(),
            before
        );
        assert_eq!(
            session.take_pending_voice_cues(),
            vec![TacticalVoiceCue::target_task_force_rejected(
                TacticalVoiceFaction::Alliance
            )]
        );
    }

    #[test]
    fn imperial_mixed_task_forces_reject_focus_target_without_mutating_orders() {
        // FUN_005a24d0 queues source event 0x102 for the same rejected command
        // when the player-side tactical manager is Imperial.
        let hostile = test_ship(64, 0, true, true);
        let mut first = test_ship(128, 1, false, true);
        first.selected = true;
        first.task_force = 3;
        first.manual_targets = vec![TacticalAttackTarget::CapitalShip(0)];
        first.attack_target = first.manual_targets.first().copied();
        let mut second = test_ship(129, 2, false, true);
        second.selected = true;
        second.task_force = 4;
        second.escort_target = Some(0);
        let mut session = test_session(vec![hostile, first, second], Vec::new(), false);
        let before = session.ships[1..]
            .iter()
            .map(|ship| {
                (
                    ship.manual_targets.clone(),
                    ship.attack_target,
                    ship.escort_target,
                )
            })
            .collect::<Vec<_>>();

        let result = assign_selected_capital_focus_target(&mut session, 0, true);

        assert_eq!(
            result,
            TacticalFocusAssignment::RejectedMixedTaskForces { member_count: 2 }
        );
        assert_eq!(
            session.ships[1..]
                .iter()
                .map(|ship| (
                    ship.manual_targets.clone(),
                    ship.attack_target,
                    ship.escort_target
                ))
                .collect::<Vec<_>>(),
            before
        );
        assert_eq!(
            session.take_pending_voice_cues(),
            vec![TacticalVoiceCue::target_task_force_rejected(
                TacticalVoiceFaction::Empire
            )]
        );
    }

    // Source: FUN_005a24d0 focus target constraint.
    #[test]
    fn one_task_force_assigns_focus_target_without_rejection_voice() {
        let mut first = test_ship(64, 0, true, true);
        first.selected = true;
        first.task_force = 5;
        let mut second = test_ship(65, 1, true, true);
        second.selected = true;
        second.task_force = 5;
        let hostile = test_ship(128, 2, false, true);
        let mut session = test_session(vec![first, second, hostile], Vec::new(), true);

        let result = assign_selected_capital_focus_target(&mut session, 2, false);

        assert_eq!(
            result,
            TacticalFocusAssignment::Assigned {
                object_ids: vec![1, 2],
                maximum_targets: 1,
            }
        );
        assert!(session.ships[..2].iter().all(|ship| {
            ship.manual_targets == vec![TacticalAttackTarget::CapitalShip(2)]
                && ship.attack_target == Some(TacticalAttackTarget::CapitalShip(2))
        }));
        assert!(session.take_pending_voice_cues().is_empty());
    }

    // Source: FUN_005a24d0 focus target toggle.
    #[test]
    fn appending_an_existing_focus_target_toggles_it_off() {
        let mut friendly = test_ship(64, 0, true, true);
        friendly.selected = true;
        friendly.task_force = 5;
        friendly.manual_targets = vec![TacticalAttackTarget::CapitalShip(1)];
        friendly.attack_target = friendly.manual_targets.first().copied();
        let hostile = test_ship(128, 1, false, true);
        let mut session = test_session(vec![friendly, hostile], Vec::new(), true);

        let result = assign_selected_capital_focus_target(&mut session, 1, true);

        assert_eq!(
            result,
            TacticalFocusAssignment::Assigned {
                object_ids: Vec::new(),
                maximum_targets: 0,
            }
        );
        assert!(session.ships[0].manual_targets.is_empty());
        assert_eq!(session.ships[0].attack_target, None);
        assert!(session.take_pending_voice_cues().is_empty());
    }

    #[test]
    fn focus_target_with_no_selected_capitals_is_a_quiet_noop() {
        let friendly = test_ship(64, 0, true, true);
        let hostile = test_ship(128, 1, false, true);
        let mut session = test_session(vec![friendly, hostile], Vec::new(), true);
        let before = session.ships[0].manual_targets.clone();

        let result = assign_selected_capital_focus_target(&mut session, 1, false);

        assert_eq!(
            result,
            TacticalFocusAssignment::Assigned {
                object_ids: Vec::new(),
                maximum_targets: 0,
            }
        );
        assert_eq!(session.ships[0].manual_targets, before);
        assert!(session.take_pending_voice_cues().is_empty());
    }

    #[test]
    fn tactical_assignment_hit_regions_follow_the_source_panels() {
        assert_eq!(
            tactical_command_rect_control_at(
                TacticalCommandPanel::Display,
                true,
                false,
                false,
                493.0,
                60.0,
            ),
            Some(TacticalCommandControl::PreviousCapital)
        );
        assert_eq!(
            tactical_command_rect_control_at(
                TacticalCommandPanel::Display,
                true,
                false,
                false,
                618.0,
                60.0,
            ),
            Some(TacticalCommandControl::NextCapital)
        );
        assert_eq!(
            tactical_command_rect_control_at(
                TacticalCommandPanel::Display,
                true,
                true,
                false,
                493.0,
                60.0,
            ),
            None
        );
        assert_eq!(
            tactical_command_rect_control_at(
                TacticalCommandPanel::Display,
                true,
                false,
                false,
                500.0,
                245.0,
            ),
            // The left button opens Maneuvers and Tactics, the right one
            // Missions (A0 TAC-05-C006, TAC-05-C012; FUN_005e4110).
            Some(TacticalCommandControl::OpenManeuvers)
        );
        assert_eq!(
            tactical_command_rect_control_at(
                TacticalCommandPanel::Display,
                true,
                false,
                false,
                570.0,
                245.0,
            ),
            Some(TacticalCommandControl::OpenMissions)
        );
        let maneuvers = TacticalCommandPanel::Maneuvers {
            pending_order: TacticalOrder::None,
            pending_tactic: TacticalTactic::StandOff,
        };
        assert_eq!(
            tactical_command_rect_control_at(maneuvers, true, false, false, 500.0, 160.0),
            Some(TacticalCommandControl::Hammer)
        );
        assert_eq!(
            tactical_command_rect_control_at(maneuvers, true, false, false, 600.0, 170.0),
            Some(TacticalCommandControl::ToggleTactic)
        );
        let missions = TacticalCommandPanel::Missions {
            pending_order: TacticalOrder::None,
        };
        assert_eq!(
            tactical_command_rect_control_at(missions, true, false, false, 575.0, 165.0),
            None
        );
        assert_eq!(
            tactical_command_rect_control_at(missions, true, true, false, 575.0, 165.0),
            Some(TacticalCommandControl::Recover)
        );
        assert_eq!(
            tactical_command_rect_control_at(missions, true, true, false, 575.0, 195.0),
            None
        );
        assert_eq!(
            tactical_command_rect_control_at(missions, true, true, true, 575.0, 195.0),
            Some(TacticalCommandControl::AttackDeathStar)
        );
        assert_eq!(
            tactical_command_rect_control_at(missions, false, true, false, 505.0, 165.0),
            None
        );
    }

    // Source: FUN_005dcc70's maneuver panel commits order +0x68 and tactic +0x38 on
    // check and returns unchanged on cancel (P58F9).
    #[test]
    fn maneuver_confirm_commits_and_cancel_preserves_selected_capital_orders() {
        let mut ship = test_ship(64, 0, true, true);
        ship.selected = true;
        let mut enemy = test_ship(128, 0, false, true);
        enemy.source_position.z = 50.0;
        let mut session = test_session(vec![ship, enemy], Vec::new(), true);
        session.selected_ship = Some(0);
        let mut state = TacticalState {
            session: Some(session),
            ..TacticalState::default()
        };

        activate_tactical_command(&mut state, TacticalCommandControl::OpenManeuvers);
        activate_tactical_command(&mut state, TacticalCommandControl::Hammer);
        activate_tactical_command(&mut state, TacticalCommandControl::ToggleTactic);
        activate_tactical_command(&mut state, TacticalCommandControl::Cancel);
        let ship = &state.session.as_ref().unwrap().ships[0];
        assert_eq!(ship.order, TacticalOrder::None);
        assert_eq!(ship.tactic, TacticalTactic::StandOff);
        assert!(state
            .session
            .as_ref()
            .unwrap()
            .pending_voice_cues
            .is_empty());

        activate_tactical_command(&mut state, TacticalCommandControl::OpenManeuvers);
        activate_tactical_command(&mut state, TacticalCommandControl::Hammer);
        activate_tactical_command(&mut state, TacticalCommandControl::ToggleTactic);
        activate_tactical_command(&mut state, TacticalCommandControl::Confirm);
        let ship = &state.session.as_ref().unwrap().ships[0];
        assert_eq!(ship.order, TacticalOrder::Hammer);
        assert_eq!(ship.tactic, TacticalTactic::Surround);
        assert_eq!(state.command_panel, TacticalCommandPanel::Display);
        assert_eq!(
            state.session.as_ref().unwrap().pending_voice_cues,
            vec![TacticalVoiceCue::capital_formation(
                TacticalVoiceFaction::Alliance,
                0
            )]
        );
    }

    // Source: FUN_005def00's mission panel enables Recover (order code 2) for a
    // selected fighter group and commits it to every member (P58F9).
    #[test]
    fn mission_confirm_assigns_recover_to_the_whole_selected_fighter_group() {
        let mut fighters = vec![test_fighter(1, true), test_fighter(1, true)];
        for (index, fighter) in fighters.iter_mut().enumerate() {
            fighter.selected = true;
            fighter.identity.fleet_roster_index = index;
            fighter.fleet_fighter_index = index;
        }
        let mut session = test_session(Vec::new(), fighters, true);
        session.selected_fighter_group = Some(0);
        let mut state = TacticalState {
            session: Some(session),
            ..TacticalState::default()
        };

        activate_tactical_command(&mut state, TacticalCommandControl::OpenMissions);
        activate_tactical_command(&mut state, TacticalCommandControl::Recover);
        activate_tactical_command(&mut state, TacticalCommandControl::Confirm);
        assert!(state
            .session
            .as_ref()
            .unwrap()
            .fighters
            .iter()
            .all(|fighter| fighter.order == TacticalOrder::Recover));
        assert_eq!(
            state.session.as_ref().unwrap().pending_voice_cues,
            vec![
                TacticalVoiceCue::fighter_recovery_capacity(TacticalVoiceFaction::Alliance, 0,),
                TacticalVoiceCue::fighter_mission(TacticalVoiceFaction::Alliance, 0),
            ]
        );
    }

    // Source: TACTICAL.DLL resources 1058/1059 paint the 23 by 23 target control
    // at (538,379) after the D-pad (tactical target control evidence,
    // 2026-09-13). Centering on the 1200 by 800 arena is the port's 2D fallback.
    #[test]
    #[ignore = "requires extracted original bitmaps in data/base/ui"]
    fn target_control_owns_its_topmost_mask_and_centers_the_selected_position() {
        let mut cache = BmpCache::new();
        cache.set_base_path(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/base/ui"),
        );
        assert_eq!(
            tactical_hud_control_at(&mut cache, 549.1, 390.1),
            Some(TacticalHudControl::CameraTarget)
        );
        assert_eq!(camera_offset_for_target(120.0, 190.0), (-480.0, -210.0));
        assert_eq!(camera_offset_for_target(600.0, 400.0), (0.0, 0.0));
    }
}
