//! Encyclopedia presentation surfaces.
//!
//! [`draw_encyclopedia_surface`] is the source-backed DTO renderer. It consumes
//! the presentation, navigation, and selected-byte contracts without looking
//! up world state or filesystem content. The older [`draw_encyclopedia`] entry
//! remains available until the production lifecycle migration is complete.
//!
//! # EDATA mapping
//!
//! The original game stores encyclopedia images in sequentially numbered BMP
//! files (`EData/EDATA.NNN`).  The C# editor (`SwRebellionEditor`) reveals the
//! direct index mapping for entity types that don't go through `ENCYBMAP.DLL`:
//!
//! | Entity type          | First EDATA index |
//! |----------------------|-------------------|
//! | Production facilities | 1               |
//! | Manufacturing facs   | 3                 |
//! | Troops               | 15                |
//! | Special forces       | 25                |
//! | Fighters             | 34                |
//! | Capital ships        | 42                |
//! | Major characters     | 72                |
//! | Minor characters     | 78                |
//!
//! Star systems use a two-level lookup via `ENCYBMAP.DLL` (not yet implemented
//! here — systems fall back to a placeholder image).
//!
//! # Integration
//!
//! ```ignore
//! egui_macroquad::ui(|ctx| {
//!     if let Some(action) = draw_encyclopedia(ctx, world, &mut enc_state) {
//!         // handle action (currently none, may add focus-system in future)
//!     }
//! });
//! ```

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use egui_macroquad::egui::TextureOptions;
use egui_macroquad::egui::{self, Color32, RichText, ScrollArea, TextureHandle, Vec2};
use rebellion_core::ids::{CapitalShipKey, CharacterKey, FighterKey, SystemKey};
use rebellion_core::world::GameWorld;

#[cfg(not(target_arch = "wasm32"))]
use crate::bmp_cache::{load_approved_hd_assets, validated_hd_bytes};
use crate::bmp_cache::{ApprovedHdAsset, AssetRenderProfile, BmpCache, DllSource};
use crate::cockpit::CockpitFaction;
use crate::encyclopedia_navigation::{
    BodyScrollIntent, EncyclopediaAction, EncyclopediaMode,
    EncyclopediaState as EncyclopediaNavigationState, SelectionForce, SourceKeyIntent,
};
use crate::encyclopedia_textures::{
    EguiEncyclopediaTextureBackend, EncyclopediaTextureCache, EncyclopediaTextureEvent,
};
use crate::encyclopedia_view::EncyclopediaView;

#[cfg(target_arch = "wasm32")]
static WASM_EDATA_CACHE: std::sync::LazyLock<std::sync::Mutex<HashMap<String, Vec<u8>>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));

/// Install original encyclopedia artwork unpacked from the browser runtime pack.
///
/// Keys are original filenames such as `EDATA.042`. GPU textures remain lazy:
/// opening a topic decodes only that topic's source bitmap, then retains the
/// resulting egui texture in [`EncyclopediaState`].
#[cfg(target_arch = "wasm32")]
pub fn set_encyclopedia_asset_cache(cache: HashMap<String, Vec<u8>>) {
    *WASM_EDATA_CACHE.lock().unwrap() = cache;
}

fn edata_filename(edata_n: u16) -> String {
    format!("EDATA.{edata_n:03}")
}

#[cfg(target_arch = "wasm32")]
fn wasm_edata_bytes(edata_n: u16) -> Option<Vec<u8>> {
    WASM_EDATA_CACHE
        .lock()
        .unwrap()
        .get(&edata_filename(edata_n))
        .cloned()
}

// ---------------------------------------------------------------------------
// Source-backed DTO surface
// ---------------------------------------------------------------------------

/// Source client width passed through `FUN_00429f30` to `FUN_0045d400`.
pub const ENCYCLOPEDIA_WIDTH: f32 = 470.0;
/// Source client height. The shell bitmap is 331 pixels high, but the original
/// constructor requests a 470 by 330 client and clips to that client.
pub const ENCYCLOPEDIA_HEIGHT: f32 = 330.0;

const TOPIC_OVERLAY: u32 = 0x2861;
const INDEX_OVERLAY: u32 = 0x2862;
const ALLIANCE_SHELL: u32 = 0x285f;
const EMPIRE_SHELL: u32 = 0x2860;
const ALLIANCE_RAIL: u32 = 0x2959;
const EMPIRE_RAIL: u32 = 0x295d;

const TOPIC_BODY_RECT: (f32, f32, f32, f32) = (17.0, 231.0, 395.0, 80.0);
const INDEX_LIST_RECT: (f32, f32, f32, f32) = (36.0, 137.0, 350.0, 160.0);
const INDEX_ROW_HEIGHT: f32 = 20.0;
const INDEX_HEADER_TEXTSTRA_ID: u16 = 0x1842;
const INDEX_STATIC_TEXTSTRA_ID: u16 = 0x1843;

/// Localized source-resource strings installed once with the surface.
///
/// These labels are UI chrome, rather than catalog topic content. Keeping the
/// retained strings here avoids copying a catalog or body on each draw while
/// making absent source resources explicit.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EncyclopediaSurfaceLabels {
    pub index_header: Option<Arc<str>>,
    pub index_static: Option<Arc<str>>,
}

/// Source-resource diagnostics surfaced without substituting invented prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaSurfaceDiagnostic {
    MissingLocalizedResourceText { resource_id: u16 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SurfaceFocusSignature {
    Index { category_id: Option<String> },
    Topic { topic_id: Option<String> },
}

/// Pixel-scroll state owned by the drawing adapter. E53 retains only ordered
/// source intents and stable IDs; this state contains no world or catalog data.
#[derive(Debug, Clone, PartialEq)]
pub struct EncyclopediaSurfaceState {
    body_scroll_offset: f32,
    labels: EncyclopediaSurfaceLabels,
    focus_signature: Option<SurfaceFocusSignature>,
    reclaim_after_consumed_key: bool,
}

impl EncyclopediaSurfaceState {
    #[must_use]
    pub fn body_scroll_offset(&self) -> f32 {
        self.body_scroll_offset
    }

    /// Install the already-localized source chrome strings for subsequent
    /// frames. Missing strings remain diagnostics rather than placeholders.
    pub fn set_source_labels(&mut self, labels: EncyclopediaSurfaceLabels) {
        self.labels = labels;
    }
}

impl Default for EncyclopediaSurfaceState {
    fn default() -> Self {
        Self {
            body_scroll_offset: 0.0,
            labels: EncyclopediaSurfaceLabels::default(),
            focus_signature: None,
            reclaim_after_consumed_key: false,
        }
    }
}

/// One shared native/browser DTO-surface frame. Actions remain unapplied so
/// the E53 controller is the sole transition authority. Keyboard entries keep
/// event order within their egui batch and precede response-derived pointer
/// actions; egui widget responses do not expose a total cross-device event
/// order. Texture events expose E46 telemetry without logging every frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncyclopediaSurfaceFrame {
    pub actions: Vec<EncyclopediaAction>,
    pub consumed_scroll_intents: Vec<BodyScrollIntent>,
    pub surface_diagnostics: Vec<EncyclopediaSurfaceDiagnostic>,
    pub texture_events: Vec<EncyclopediaTextureEvent>,
    pub texture_diagnostic: Option<String>,
    pub active_asset_id: Option<String>,
    pub active_digest: Option<String>,
    /// Read-only pointer attempts for source controls whose disabled state
    /// intentionally suppresses controller actions. This lets acceptance
    /// evidence distinguish an inspected disabled click from a fabricated
    /// `NoChange` navigation action.
    pub input_attempts: Vec<EncyclopediaSurfaceInputAttempt>,
    /// Read-only identity of the renderer-owned keyboard target and egui's
    /// actually focused widget after this input batch. Acceptance fixtures use
    /// this to distinguish surface focus from an unrelated global widget.
    pub keyboard_focus: Option<EncyclopediaSurfaceFocus>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaSurfaceInputControl {
    PreviousTopic,
    NextTopic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncyclopediaSurfaceInputAttempt {
    pub control: EncyclopediaSurfaceInputControl,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaSurfaceFocusKind {
    IndexList,
    TopicBody,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncyclopediaSurfaceFocus {
    pub target_kind: EncyclopediaSurfaceFocusKind,
    pub target_id: u64,
    pub focused_id: Option<u64>,
    pub owns_focus: bool,
}

impl EncyclopediaSurfaceFrame {
    fn empty() -> Self {
        Self {
            actions: Vec::new(),
            consumed_scroll_intents: Vec::new(),
            surface_diagnostics: Vec::new(),
            texture_events: Vec::new(),
            texture_diagnostic: None,
            active_asset_id: None,
            active_digest: None,
            input_attempts: Vec::new(),
            keyboard_focus: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BitmapControlResponse {
    activated: bool,
    attempted: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EncyclopediaControlSpec {
    command_id: u16,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    normal_resource: u32,
    pressed_resource: u32,
    disabled_resource: Option<u32>,
}

const fn encyclopedia_control_specs(faction: CockpitFaction) -> [EncyclopediaControlSpec; 7] {
    let (slot_2, slot_3, slot_4, slot_5, slot_6) = match faction {
        CockpitFaction::Alliance => (
            (0x286c, 0x286b),
            (0x2868, 0x2867),
            (0x2d60, 0x2d5f),
            (0x2870, 0x286f),
            (0x286a, 0x2869),
        ),
        CockpitFaction::Empire => (
            (0x2878, 0x2877),
            (0x2874, 0x2873),
            (0x2d62, 0x2d61),
            (0x287a, 0x2879),
            (0x2876, 0x2875),
        ),
    };
    [
        EncyclopediaControlSpec {
            command_id: 0x6f,
            x: 36,
            y: 78,
            width: 49,
            height: 41,
            normal_resource: 0x2864,
            pressed_resource: 0x2863,
            disabled_resource: None,
        },
        EncyclopediaControlSpec {
            command_id: 0x70,
            x: 88,
            y: 78,
            width: 49,
            height: 41,
            normal_resource: 0x286e,
            pressed_resource: 0x286d,
            disabled_resource: None,
        },
        EncyclopediaControlSpec {
            command_id: 0x71,
            x: 140,
            y: 78,
            width: 49,
            height: 41,
            normal_resource: slot_2.0,
            pressed_resource: slot_2.1,
            disabled_resource: None,
        },
        EncyclopediaControlSpec {
            command_id: 0x72,
            x: 192,
            y: 78,
            width: 49,
            height: 41,
            normal_resource: slot_3.0,
            pressed_resource: slot_3.1,
            disabled_resource: None,
        },
        EncyclopediaControlSpec {
            command_id: 0x73,
            x: 244,
            y: 78,
            width: 49,
            height: 41,
            normal_resource: slot_4.0,
            pressed_resource: slot_4.1,
            disabled_resource: None,
        },
        EncyclopediaControlSpec {
            command_id: 0x74,
            x: 296,
            y: 78,
            width: 49,
            height: 41,
            normal_resource: slot_5.0,
            pressed_resource: slot_5.1,
            disabled_resource: None,
        },
        EncyclopediaControlSpec {
            command_id: 0x75,
            x: 348,
            y: 78,
            width: 49,
            height: 41,
            normal_resource: slot_6.0,
            pressed_resource: slot_6.1,
            disabled_resource: None,
        },
    ]
}

fn rail_control_specs(faction: CockpitFaction) -> [EncyclopediaControlSpec; 3] {
    match faction {
        CockpitFaction::Alliance => [
            EncyclopediaControlSpec {
                command_id: 0xfb,
                x: 423,
                y: 25,
                width: 32,
                height: 31,
                normal_resource: 0x2882,
                pressed_resource: 0x2883,
                disabled_resource: None,
            },
            EncyclopediaControlSpec {
                command_id: 0x67,
                x: 423,
                y: 93,
                width: 32,
                height: 31,
                normal_resource: 0x2886,
                pressed_resource: 0x2887,
                disabled_resource: Some(0x2887),
            },
            EncyclopediaControlSpec {
                command_id: 0x68,
                x: 423,
                y: 147,
                width: 32,
                height: 31,
                normal_resource: 0x2884,
                pressed_resource: 0x2885,
                disabled_resource: Some(0x2885),
            },
        ],
        CockpitFaction::Empire => [
            EncyclopediaControlSpec {
                command_id: 0xfb,
                x: 426,
                y: 21,
                width: 44,
                height: 41,
                normal_resource: 0x2888,
                pressed_resource: 0x2889,
                disabled_resource: None,
            },
            EncyclopediaControlSpec {
                command_id: 0x67,
                x: 426,
                y: 89,
                width: 44,
                height: 41,
                normal_resource: 0x288c,
                pressed_resource: 0x288d,
                disabled_resource: Some(0x288d),
            },
            EncyclopediaControlSpec {
                command_id: 0x68,
                x: 426,
                y: 143,
                width: 44,
                height: 41,
                normal_resource: 0x288a,
                pressed_resource: 0x288b,
                disabled_resource: Some(0x288b),
            },
        ],
    }
}

const fn topic_direction_specs() -> [EncyclopediaControlSpec; 2] {
    [
        EncyclopediaControlSpec {
            command_id: 0x83,
            x: 28,
            y: 14,
            width: 21,
            height: 17,
            normal_resource: 0x2891,
            pressed_resource: 0x2892,
            disabled_resource: Some(0x2893),
        },
        EncyclopediaControlSpec {
            command_id: 0x84,
            x: 380,
            y: 14,
            width: 21,
            height: 17,
            normal_resource: 0x288e,
            pressed_resource: 0x288f,
            disabled_resource: Some(0x2890),
        },
    ]
}

fn draw_bitmap_control(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    cache: &mut BmpCache,
    window_rect: egui::Rect,
    scale: f32,
    spec: EncyclopediaControlSpec,
    enabled: bool,
    selected: bool,
) -> bool {
    draw_bitmap_control_response(ui, ctx, cache, window_rect, scale, spec, enabled, selected)
        .activated
}

#[expect(
    clippy::too_many_arguments,
    reason = "fixed source control inputs stay explicit"
)]
fn draw_bitmap_control_response(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    cache: &mut BmpCache,
    window_rect: egui::Rect,
    scale: f32,
    spec: EncyclopediaControlSpec,
    enabled: bool,
    selected: bool,
) -> BitmapControlResponse {
    let rect = encyclopedia_rect(
        window_rect,
        scale,
        f32::from(spec.x),
        f32::from(spec.y),
        f32::from(spec.width),
        f32::from(spec.height),
    );
    let response = ui.interact(
        rect,
        bitmap_control_id(ui, spec),
        if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    let (pointer, primary_down, primary_released) = ctx.input(|input| {
        (
            input.pointer.interact_pos(),
            input.pointer.button_down(egui::PointerButton::Primary),
            input.pointer.button_released(egui::PointerButton::Primary),
        )
    });
    let pointer_inside = pointer.is_some_and(|point| encyclopedia_rect_contains(rect, point));
    let captured_press = enabled && primary_down && pointer_inside;
    let resource = bitmap_control_resource(spec, enabled, selected, captured_press);
    paint_strategy_resource(ui.painter(), ctx, cache, resource, rect);

    BitmapControlResponse {
        activated: enabled
            && response.clicked()
            && response
                .interact_pointer_pos()
                .is_some_and(|point| encyclopedia_rect_contains(rect, point)),
        attempted: primary_released && pointer_inside,
    }
}

fn bitmap_control_id(ui: &egui::Ui, spec: EncyclopediaControlSpec) -> egui::Id {
    ui.id().with(("encyclopedia-control", spec.command_id))
}

fn bitmap_control_resource(
    spec: EncyclopediaControlSpec,
    enabled: bool,
    selected: bool,
    captured_press: bool,
) -> u32 {
    if !enabled {
        spec.disabled_resource.unwrap_or(spec.normal_resource)
    } else if selected || captured_press {
        spec.pressed_resource
    } else {
        spec.normal_resource
    }
}

fn keyboard_actions(
    ctx: &egui::Context,
    focus_id: egui::Id,
    mode: EncyclopediaMode,
    visible_rows: usize,
    reclaim_after_consumed_key: bool,
    owned_focus_ids: &[egui::Id],
) -> (Vec<EncyclopediaAction>, bool) {
    let (has_focus, had_focus, focused) = ctx.memory(|memory| {
        (
            memory.has_focus(focus_id),
            memory.had_focus_last_frame(focus_id),
            memory.focused(),
        )
    });
    // `None` with prior ownership is egui's pending Tab handoff. A concrete
    // different focus ID belongs to another widget and must not be reclaimed.
    let pointer_claimed_focus = ctx.input(|input| input.pointer.any_pressed());
    let reclaimable_focus = focused
        .map(|focused| owned_focus_ids.contains(&focused))
        .unwrap_or(true);
    if !(has_focus
        || (had_focus && focused.is_none())
        || (reclaim_after_consumed_key && reclaimable_focus && !pointer_claimed_focus))
    {
        return (Vec::new(), false);
    }

    ctx.memory_mut(|memory| {
        // On the first Tab/arrows frame after a transition, egui may have
        // applied its default focus motion before the source child gets a
        // chance to install its filter. Restore that child, then consume the
        // source-owned event below.
        if !has_focus {
            memory.request_focus(focus_id);
        }
        memory.set_focus_lock_filter(
            focus_id,
            egui::EventFilter {
                tab: true,
                horizontal_arrows: true,
                vertical_arrows: true,
                escape: true,
            },
        );
    });

    let mut actions = Vec::new();
    let mut consumed = false;
    ctx.input_mut(|input| {
        input.events.retain(|event| {
            let egui::Event::Key {
                key, pressed: true, ..
            } = event
            else {
                return true;
            };
            let intent = match *key {
                egui::Key::Escape => SourceKeyIntent::Escape,
                egui::Key::Enter => SourceKeyIntent::Enter,
                egui::Key::ArrowLeft => SourceKeyIntent::Left,
                egui::Key::ArrowRight => SourceKeyIntent::Right,
                egui::Key::ArrowUp => SourceKeyIntent::Up,
                egui::Key::ArrowDown => SourceKeyIntent::Down,
                egui::Key::PageUp => SourceKeyIntent::PageUp { visible_rows },
                egui::Key::PageDown => SourceKeyIntent::PageDown { visible_rows },
                egui::Key::Home if mode == EncyclopediaMode::Index => SourceKeyIntent::Home,
                egui::Key::End if mode == EncyclopediaMode::Index => SourceKeyIntent::End,
                // Source consumes Tab locally without a navigation command.
                egui::Key::Tab => {
                    consumed = true;
                    return false;
                }
                _ => return true,
            };
            actions.push(EncyclopediaAction::SourceKey(intent));
            consumed = true;
            false
        });
    });
    (actions, consumed)
}

fn apply_body_scroll_intents(
    state: &mut EncyclopediaSurfaceState,
    intents: &[BodyScrollIntent],
    line_height: f32,
    page_height: f32,
) {
    for intent in intents {
        match intent {
            BodyScrollIntent::LineUp => {
                state.body_scroll_offset = (state.body_scroll_offset - line_height).max(0.0);
            }
            BodyScrollIntent::LineDown => state.body_scroll_offset += line_height,
            BodyScrollIntent::PageUp => {
                state.body_scroll_offset = (state.body_scroll_offset - page_height).max(0.0);
            }
            BodyScrollIntent::PageDown => state.body_scroll_offset += page_height,
            BodyScrollIntent::ResetToTop => state.body_scroll_offset = 0.0,
        }
    }
}

const fn rail_selection_states(mode: EncyclopediaMode) -> (bool, bool) {
    (
        matches!(mode, EncyclopediaMode::Topic),
        matches!(mode, EncyclopediaMode::Index),
    )
}

#[derive(Debug, Clone, Copy)]
struct SurfaceKeyboardTarget {
    focus_id: egui::Id,
    kind: EncyclopediaSurfaceFocusKind,
    visible_rows: usize,
}

/// Draw the source-backed encyclopedia surface over an already-resolved DTO.
///
/// This function performs no world lookup, language fallback, image selection,
/// filesystem discovery, or navigation transition. Physical controls emit E53
/// actions, while E46 receives only the already-selected verified image bytes.
#[expect(
    clippy::too_many_arguments,
    reason = "the DTO surface keeps each owner explicit"
)]
pub fn draw_encyclopedia_surface(
    ctx: &egui::Context,
    view: &EncyclopediaView,
    navigation: &mut EncyclopediaNavigationState,
    surface: &mut EncyclopediaSurfaceState,
    chrome: &mut BmpCache,
    textures: &mut EncyclopediaTextureCache<EguiEncyclopediaTextureBackend>,
    faction: CockpitFaction,
    origin: egui::Pos2,
    scale: f32,
) -> EncyclopediaSurfaceFrame {
    if !(scale.is_finite() && scale > 0.0) {
        return EncyclopediaSurfaceFrame::empty();
    }

    let consumed_scroll_intents = navigation.take_body_scroll_intents();
    let style = ctx.style();
    let body_font = egui::TextStyle::Body.resolve(&style);
    let line_height = ctx.fonts(|fonts| fonts.row_height(&body_font));
    apply_body_scroll_intents(
        surface,
        &consumed_scroll_intents,
        line_height,
        TOPIC_BODY_RECT.3 * scale,
    );

    let active_image = if navigation.mode() == EncyclopediaMode::Topic {
        view.active_topic
            .as_ref()
            .and_then(|topic| topic.image.as_ref())
    } else {
        None
    };
    let active_asset_id = active_image.map(|image| image.asset_id.clone());
    let active_digest = active_image.map(|image| image.digest.clone());
    let resolution = textures.resolve(active_image);
    let texture_id = resolution.texture.map(TextureHandle::id);
    let texture_events = resolution.events;
    let texture_diagnostic = resolution.diagnostic.map(str::to_owned);

    let focus_signature = match navigation.mode() {
        EncyclopediaMode::Index => SurfaceFocusSignature::Index {
            category_id: navigation.selected_category_id().map(str::to_owned),
        },
        EncyclopediaMode::Topic => SurfaceFocusSignature::Topic {
            topic_id: navigation.selection.topic_id.clone(),
        },
    };
    let transfer_focus = surface.focus_signature.as_ref() != Some(&focus_signature);
    surface.focus_signature = Some(focus_signature);

    let surface_diagnostics = if navigation.mode() == EncyclopediaMode::Index {
        [
            (
                INDEX_HEADER_TEXTSTRA_ID,
                surface.labels.index_header.is_none(),
            ),
            (
                INDEX_STATIC_TEXTSTRA_ID,
                surface.labels.index_static.is_none(),
            ),
        ]
        .into_iter()
        .filter_map(|(resource_id, missing)| {
            missing.then_some(
                EncyclopediaSurfaceDiagnostic::MissingLocalizedResourceText { resource_id },
            )
        })
        .collect()
    } else {
        Vec::new()
    };

    let mut mouse_actions = Vec::new();
    let mut input_attempts = Vec::new();
    let mut keyboard_target = None;
    let mut owned_focus_ids = Vec::new();
    egui::Area::new(egui::Id::new("source-encyclopedia-surface"))
        .fixed_pos(origin)
        .movable(false)
        .sense(egui::Sense::empty())
        .order(egui::Order::Middle)
        .show(ctx, |ui| {
            let size = egui::vec2(ENCYCLOPEDIA_WIDTH * scale, ENCYCLOPEDIA_HEIGHT * scale);
            let (window_rect, _) = ui.allocate_exact_size(size, egui::Sense::empty());
            ui.set_clip_rect(ui.clip_rect().intersect(window_rect));
            let (shell, rail) = match faction {
                CockpitFaction::Alliance => (ALLIANCE_SHELL, ALLIANCE_RAIL),
                CockpitFaction::Empire => (EMPIRE_SHELL, EMPIRE_RAIL),
            };
            paint_strategy_resource_native_clipped(
                ui.painter(),
                ctx,
                chrome,
                shell,
                window_rect.min,
                scale,
                window_rect,
            );
            paint_strategy_resource(
                ui.painter(),
                ctx,
                chrome,
                rail,
                encyclopedia_rect(window_rect, scale, 412.0, 0.0, 58.0, 330.0),
            );
            let (overlay, overlay_y) = match navigation.mode() {
                EncyclopediaMode::Index => (INDEX_OVERLAY, 13.0),
                EncyclopediaMode::Topic => (TOPIC_OVERLAY, 14.0),
            };
            paint_strategy_resource(
                ui.painter(),
                ctx,
                chrome,
                overlay,
                encyclopedia_rect(window_rect, scale, 12.0, overlay_y, 400.0, 306.0),
            );

            let rail_controls = rail_control_specs(faction);
            owned_focus_ids.push(bitmap_control_id(ui, rail_controls[0]));
            if draw_bitmap_control(
                ui,
                ctx,
                chrome,
                window_rect,
                scale,
                rail_controls[0],
                true,
                false,
            ) {
                mouse_actions.push(EncyclopediaAction::Close);
            }
            let topic_enabled = navigation.selection.topic_id.is_some();
            let (topic_selected, index_selected) = rail_selection_states(navigation.mode());
            owned_focus_ids.push(bitmap_control_id(ui, rail_controls[1]));
            if draw_bitmap_control(
                ui,
                ctx,
                chrome,
                window_rect,
                scale,
                rail_controls[1],
                topic_enabled,
                topic_selected,
            ) {
                mouse_actions.push(EncyclopediaAction::SetMode(EncyclopediaMode::Topic));
            }
            owned_focus_ids.push(bitmap_control_id(ui, rail_controls[2]));
            if draw_bitmap_control(
                ui,
                ctx,
                chrome,
                window_rect,
                scale,
                rail_controls[2],
                true,
                index_selected,
            ) {
                mouse_actions.push(EncyclopediaAction::SetMode(EncyclopediaMode::Index));
            }

            keyboard_target = match navigation.mode() {
                EncyclopediaMode::Index => Some(draw_index_surface(
                    ui,
                    ctx,
                    view,
                    navigation,
                    surface,
                    chrome,
                    faction,
                    window_rect,
                    scale,
                    transfer_focus,
                    &mut mouse_actions,
                    &mut owned_focus_ids,
                )),
                EncyclopediaMode::Topic => draw_topic_surface(
                    ui,
                    ctx,
                    view,
                    surface,
                    chrome,
                    texture_id,
                    texture_diagnostic.as_deref(),
                    window_rect,
                    scale,
                    transfer_focus,
                    &mut mouse_actions,
                    &mut input_attempts,
                    &mut owned_focus_ids,
                ),
            };
        });

    let (mut actions, consumed_key) = keyboard_target.map_or_else(
        || (Vec::new(), false),
        |target| {
            keyboard_actions(
                ctx,
                target.focus_id,
                navigation.mode(),
                target.visible_rows,
                surface.reclaim_after_consumed_key,
                &owned_focus_ids,
            )
        },
    );
    surface.reclaim_after_consumed_key = consumed_key;
    actions.append(&mut mouse_actions);
    let keyboard_focus = keyboard_target.map(|target| {
        let focused_id = ctx.memory(|memory| memory.focused());
        EncyclopediaSurfaceFocus {
            target_kind: target.kind,
            target_id: target.focus_id.value(),
            focused_id: focused_id.map(|id| id.value()),
            owns_focus: focused_id == Some(target.focus_id),
        }
    });

    EncyclopediaSurfaceFrame {
        actions,
        consumed_scroll_intents,
        surface_diagnostics,
        texture_events,
        texture_diagnostic,
        active_asset_id,
        active_digest,
        input_attempts,
        keyboard_focus,
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "fixed source surface inputs stay explicit"
)]
fn draw_index_surface(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    view: &EncyclopediaView,
    navigation: &EncyclopediaNavigationState,
    surface: &EncyclopediaSurfaceState,
    chrome: &mut BmpCache,
    faction: CockpitFaction,
    window_rect: egui::Rect,
    scale: f32,
    transfer_focus: bool,
    actions: &mut Vec<EncyclopediaAction>,
    owned_focus_ids: &mut Vec<egui::Id>,
) -> SurfaceKeyboardTarget {
    if let Some(header) = surface.labels.index_header.as_deref() {
        ui.painter().text(
            encyclopedia_rect(window_rect, scale, 36.0, 14.0, 0.0, 0.0).left_top(),
            egui::Align2::LEFT_TOP,
            header,
            egui::FontId::proportional(14.0 * scale),
            Color32::WHITE,
        );
    }
    if let Some(label) = surface.labels.index_static.as_deref() {
        ui.painter().text(
            encyclopedia_rect(window_rect, scale, 36.0, 48.0, 0.0, 0.0).left_top(),
            egui::Align2::LEFT_TOP,
            label,
            egui::FontId::proportional(14.0 * scale),
            Color32::WHITE,
        );
    }

    for (slot, spec) in encyclopedia_control_specs(faction).into_iter().enumerate() {
        owned_focus_ids.push(bitmap_control_id(ui, spec));
        let (category_id, enabled, selected) = if slot == 0 {
            (
                None,
                view.index_enabled,
                navigation.selected_category_id().is_none(),
            )
        } else if let Some(category) = view.categories.get(slot - 1) {
            (
                Some(category.category_id.as_str()),
                category.enabled,
                navigation.selected_category_id() == Some(category.category_id.as_str()),
            )
        } else {
            (None, false, false)
        };
        if draw_bitmap_control(ui, ctx, chrome, window_rect, scale, spec, enabled, selected) {
            actions.push(EncyclopediaAction::SelectCategory {
                category_id: category_id.map(str::to_owned),
                force: SelectionForce::Normal,
            });
        }
    }

    let selected_category_label = navigation
        .selected_category_id()
        .and_then(|selected| {
            view.categories
                .iter()
                .find(|category| category.category_id == selected)
        })
        .and_then(|category| category.label.as_deref())
        .or(view.index_label.as_deref())
        .unwrap_or("");
    ui.painter().text(
        encyclopedia_rect(window_rect, scale, 40.0, 119.0, 346.0, 18.0).left_top(),
        egui::Align2::LEFT_TOP,
        selected_category_label,
        egui::FontId::proportional(14.0 * scale),
        Color32::WHITE,
    );

    if let Some(selected) = navigation.selection.topic_id.as_deref() {
        if let Some(topic) = view.topics.iter().find(|topic| topic.topic_id == selected) {
            ui.painter().text(
                encyclopedia_rect(window_rect, scale, 143.0, 45.0, 245.0, 18.0).left_top(),
                egui::Align2::LEFT_TOP,
                topic.title.as_ref(),
                egui::FontId::proportional(14.0 * scale),
                Color32::WHITE,
            );
        }
    }

    let list_rect = encyclopedia_rect(
        window_rect,
        scale,
        INDEX_LIST_RECT.0,
        INDEX_LIST_RECT.1,
        INDEX_LIST_RECT.2,
        INDEX_LIST_RECT.3,
    );
    let list_focus = ui.interact(
        list_rect,
        ui.id().with("encyclopedia-index-list-focus"),
        egui::Sense::focusable_noninteractive(),
    );
    owned_focus_ids.push(list_focus.id);
    if transfer_focus {
        list_focus.request_focus();
    }
    ui.scope_builder(egui::UiBuilder::new().max_rect(list_rect), |ui| {
        ui.set_clip_rect(list_rect);
        ScrollArea::vertical()
            .id_salt("source-encyclopedia-index-list")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_width(list_rect.width());
                for topic in &view.topics {
                    let (row, response) = ui.allocate_exact_size(
                        egui::vec2(list_rect.width(), INDEX_ROW_HEIGHT * scale),
                        egui::Sense::click(),
                    );
                    owned_focus_ids.push(response.id);
                    let selected =
                        navigation.selection.topic_id.as_deref() == Some(topic.topic_id.as_str());
                    if selected {
                        ui.painter().rect_filled(
                            row,
                            0.0,
                            Color32::from_rgba_unmultiplied(64, 96, 128, 160),
                        );
                    }
                    ui.painter().text(
                        row.left_center(),
                        egui::Align2::LEFT_CENTER,
                        topic.title.as_ref(),
                        egui::FontId::proportional(14.0 * scale),
                        Color32::WHITE,
                    );
                    if response.clicked() {
                        actions.push(EncyclopediaAction::SelectTopic(topic.topic_id.clone()));
                    }
                    if response.double_clicked() {
                        actions.push(EncyclopediaAction::SetMode(EncyclopediaMode::Topic));
                    }
                }
            });
    });
    SurfaceKeyboardTarget {
        focus_id: list_focus.id,
        kind: EncyclopediaSurfaceFocusKind::IndexList,
        visible_rows: (list_rect.height() / (INDEX_ROW_HEIGHT * scale))
            .floor()
            .max(1.0) as usize,
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "fixed source surface inputs stay explicit"
)]
fn draw_topic_surface(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    view: &EncyclopediaView,
    surface: &mut EncyclopediaSurfaceState,
    chrome: &mut BmpCache,
    texture_id: Option<egui::TextureId>,
    texture_diagnostic: Option<&str>,
    window_rect: egui::Rect,
    scale: f32,
    transfer_focus: bool,
    actions: &mut Vec<EncyclopediaAction>,
    input_attempts: &mut Vec<EncyclopediaSurfaceInputAttempt>,
    owned_focus_ids: &mut Vec<egui::Id>,
) -> Option<SurfaceKeyboardTarget> {
    let directions = topic_direction_specs();
    owned_focus_ids.push(bitmap_control_id(ui, directions[0]));
    let previous_enabled = view.navigation.previous_topic_id.is_some();
    let previous = draw_bitmap_control_response(
        ui,
        ctx,
        chrome,
        window_rect,
        scale,
        directions[0],
        previous_enabled,
        false,
    );
    if previous.attempted {
        input_attempts.push(EncyclopediaSurfaceInputAttempt {
            control: EncyclopediaSurfaceInputControl::PreviousTopic,
            enabled: previous_enabled,
        });
    }
    if previous.activated {
        actions.push(EncyclopediaAction::PreviousTopic);
    }
    owned_focus_ids.push(bitmap_control_id(ui, directions[1]));
    let next_enabled = view.navigation.next_topic_id.is_some();
    let next = draw_bitmap_control_response(
        ui,
        ctx,
        chrome,
        window_rect,
        scale,
        directions[1],
        next_enabled,
        false,
    );
    if next.attempted {
        input_attempts.push(EncyclopediaSurfaceInputAttempt {
            control: EncyclopediaSurfaceInputControl::NextTopic,
            enabled: next_enabled,
        });
    }
    if next.activated {
        actions.push(EncyclopediaAction::NextTopic);
    }

    let Some(active) = view.active_topic.as_ref() else {
        return None;
    };
    ui.painter().text(
        encyclopedia_rect(window_rect, scale, 36.0, 14.0, 340.0, 17.0).left_top(),
        egui::Align2::LEFT_TOP,
        active.title.as_ref(),
        egui::FontId::proportional(14.0 * scale),
        Color32::WHITE,
    );

    if let Some(texture_id) = texture_id {
        ui.painter().image(
            texture_id,
            encyclopedia_rect(
                window_rect,
                scale,
                12.0,
                31.0,
                active
                    .image
                    .as_ref()
                    .map_or(400.0, |image| image.width as f32),
                active
                    .image
                    .as_ref()
                    .map_or(200.0, |image| image.height as f32),
            ),
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );
    } else if let Some(diagnostic) = texture_diagnostic {
        ui.painter().text(
            encyclopedia_rect(window_rect, scale, 20.0, 40.0, 380.0, 20.0).left_top(),
            egui::Align2::LEFT_TOP,
            diagnostic,
            egui::FontId::proportional(12.0 * scale),
            Color32::LIGHT_RED,
        );
    }

    let body_rect = encyclopedia_rect(
        window_rect,
        scale,
        TOPIC_BODY_RECT.0,
        TOPIC_BODY_RECT.1,
        TOPIC_BODY_RECT.2,
        TOPIC_BODY_RECT.3,
    );
    let body_focus = ui.interact(
        body_rect,
        ui.id().with("encyclopedia-topic-body-focus"),
        egui::Sense::focusable_noninteractive(),
    );
    owned_focus_ids.push(body_focus.id);
    if transfer_focus {
        body_focus.request_focus();
    }
    let scroll_output = ui
        .scope_builder(egui::UiBuilder::new().max_rect(body_rect), |ui| {
            ui.set_clip_rect(body_rect);
            ScrollArea::vertical()
                .id_salt("source-encyclopedia-topic-body")
                .auto_shrink([false, false])
                .vertical_scroll_offset(surface.body_scroll_offset)
                .show(ui, |ui| {
                    ui.set_width(body_rect.width());
                    ui.label(active.body.as_ref());
                    for stat in &active.stats {
                        ui.horizontal(|ui| {
                            ui.label(stat.label.as_ref());
                            ui.label(stat.value.as_ref());
                        });
                    }
                })
        })
        .inner;
    surface.body_scroll_offset = scroll_output.state.offset.y;
    Some(SurfaceKeyboardTarget {
        focus_id: body_focus.id,
        kind: EncyclopediaSurfaceFocusKind::TopicBody,
        visible_rows: 0,
    })
}

fn encyclopedia_rect_contains(rect: egui::Rect, point: egui::Pos2) -> bool {
    point.x >= rect.min.x && point.x < rect.max.x && point.y >= rect.min.y && point.y < rect.max.y
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct NativeBitmapGeometry {
    destination: egui::Rect,
    uv: egui::Rect,
    clip_rect: egui::Rect,
}

fn native_bitmap_geometry(
    origin: egui::Pos2,
    native_size: egui::Vec2,
    scale: f32,
    clip_rect: egui::Rect,
) -> NativeBitmapGeometry {
    NativeBitmapGeometry {
        destination: egui::Rect::from_min_size(origin, native_size * scale),
        uv: egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
        clip_rect,
    }
}

fn paint_strategy_resource_native_clipped(
    painter: &egui::Painter,
    ctx: &egui::Context,
    cache: &mut BmpCache,
    resource_id: u32,
    origin: egui::Pos2,
    scale: f32,
    clip_rect: egui::Rect,
) {
    let Some(texture) = cache.get(ctx, DllSource::Strategy, resource_id) else {
        return;
    };
    let geometry = native_bitmap_geometry(origin, texture.size_vec2(), scale, clip_rect);
    painter.with_clip_rect(geometry.clip_rect).image(
        texture.id(),
        geometry.destination,
        geometry.uv,
        Color32::WHITE,
    );
}

fn paint_strategy_resource(
    painter: &egui::Painter,
    ctx: &egui::Context,
    cache: &mut BmpCache,
    resource_id: u32,
    rect: egui::Rect,
) {
    let Some(texture) = cache.get(ctx, DllSource::Strategy, resource_id) else {
        return;
    };
    painter.image(
        texture.id(),
        rect,
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
        Color32::WHITE,
    );
}

// ---------------------------------------------------------------------------
// Tab selection
// ---------------------------------------------------------------------------

/// Which entity category is currently displayed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EncyclopediaTab {
    #[default]
    CapitalShips,
    Fighters,
    Characters,
    Systems,
}

// ---------------------------------------------------------------------------
// EncyclopediaState
// ---------------------------------------------------------------------------

/// All mutable state for the encyclopedia panel.
pub struct EncyclopediaState {
    /// Whether the panel is open.
    pub open: bool,
    /// Active tab.
    pub tab: EncyclopediaTab,
    /// Selected entity within the current tab (list index).
    pub selected_index: usize,
    /// Path to the `EData`/ directory (original BMPs).
    pub edata_path: Option<PathBuf>,
    /// Path to HD upscaled PNGs directory, used only by the faithful-HD profile.
    pub hd_path: Option<PathBuf>,
    /// Explicit asset profile. Original parity is the default.
    pub asset_profile: AssetRenderProfile,
    /// Selected original index category command (`0x6f..=0x75`).
    ///
    /// The replacement panel does not consume this field. It is retained for
    /// the source-exact index renderer that will replace that panel.
    pub original_category_command: u16,
    /// Stable compound object identity selected in the original index.
    pub original_selected_object_id: Option<u32>,
    /// First catalog row shown by the original index list.
    pub original_scroll_row: usize,
    /// EDATA keys explicitly approved by the faithful-HD manifest.
    approved_hd_assets: HashMap<String, ApprovedHdAsset>,
    /// Cached textures keyed by EDATA file number (1-based).
    textures: HashMap<u16, Option<TextureHandle>>,
}

impl EncyclopediaState {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Configure the `EData` directory.  Call before opening the encyclopedia.
    pub fn set_edata_path(&mut self, path: impl Into<PathBuf>) {
        self.edata_path = Some(path.into());
        self.textures.clear();
    }

    /// Configure the HD upscaled PNG directory. Expected naming:
    /// `EDATA_NNN.png`. Setting a path does not enable HD substitution.
    pub fn set_hd_path(&mut self, path: impl Into<PathBuf>) {
        let path = path.into();
        #[cfg(not(target_arch = "wasm32"))]
        {
            let manifest_root = path.parent().unwrap_or(&path);
            self.approved_hd_assets = load_approved_hd_assets(manifest_root);
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.approved_hd_assets.clear();
        }
        self.hd_path = Some(path);
        self.textures.clear();
    }

    /// Change the explicit render profile and invalidate prior textures.
    pub fn set_asset_profile(&mut self, profile: AssetRenderProfile) {
        if self.asset_profile != profile {
            self.asset_profile = profile;
            self.textures.clear();
        }
    }
}

impl Default for EncyclopediaState {
    fn default() -> Self {
        EncyclopediaState {
            open: false,
            tab: EncyclopediaTab::default(),
            selected_index: 0,
            edata_path: None,
            hd_path: None,
            asset_profile: AssetRenderProfile::OriginalParity,
            original_category_command: 0x6f,
            original_selected_object_id: None,
            original_scroll_row: 0,
            approved_hd_assets: HashMap::new(),
            textures: HashMap::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Public draw function
// ---------------------------------------------------------------------------

/// Render the encyclopedia window inside an `egui_macroquad::ui` closure.
///
/// Returns `Some(SystemKey)` when the user clicks "Zoom to system" on a
/// star system entry (caller should pan the galaxy map to that system).
/// Returns `None` otherwise.
#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
#[expect(
    clippy::cast_possible_truncation,
    reason = "Rendering uses floating pixel coordinates and fixed-width resource IDs; retain existing rounding and narrowing."
)]
pub fn draw_encyclopedia(
    ctx: &egui::Context,
    world: &GameWorld,
    state: &mut EncyclopediaState,
    bmp_cache: &mut BmpCache,
) -> Option<SystemKey> {
    if !state.open {
        return None;
    }

    let mut focus_system: Option<SystemKey> = None;

    // Extract fields that the Window::open() needs to borrow independently
    // of the closure's mutable borrow of `state`.
    let mut window_open = state.open;

    egui::Window::new("Encyclopedia")
        .default_size([700.0, 520.0])
        .min_width(480.0)
        .min_height(360.0)
        .collapsible(true)
        .open(&mut window_open)
        .show(ctx, |ui| {
            // ── Tab bar ───────────────────────────────────────────────────
            ui.horizontal(|ui| {
                for (label, tab) in [
                    ("Capital Ships", EncyclopediaTab::CapitalShips),
                    ("Fighters", EncyclopediaTab::Fighters),
                    ("Characters", EncyclopediaTab::Characters),
                    ("Systems", EncyclopediaTab::Systems),
                ] {
                    if ui.selectable_label(state.tab == tab, label).clicked() && state.tab != tab {
                        state.tab = tab;
                        state.selected_index = 0;
                    }
                }
            });
            ui.separator();

            // ── Two-column layout: list (left) | detail (right) ───────────
            ui.columns(2, |cols| {
                // Left: scrollable entity list
                let list_ui = &mut cols[0];
                ScrollArea::vertical()
                    .id_salt("enc_list")
                    .show(list_ui, |ui| match state.tab {
                        EncyclopediaTab::CapitalShips => {
                            let keys: Vec<(CapitalShipKey, &str)> = world
                                .capital_ship_classes
                                .iter()
                                .map(|(k, c)| (k, c.name.as_str()))
                                .collect();
                            for (i, (_, name)) in keys.iter().enumerate() {
                                let sel = state.selected_index == i;
                                if ui.selectable_label(sel, *name).clicked() {
                                    state.selected_index = i;
                                }
                            }
                        }
                        EncyclopediaTab::Fighters => {
                            let keys: Vec<(FighterKey, &str)> = world
                                .fighter_classes
                                .iter()
                                .map(|(k, c)| (k, c.name.as_str()))
                                .collect();
                            for (i, (_, name)) in keys.iter().enumerate() {
                                let sel = state.selected_index == i;
                                if ui.selectable_label(sel, *name).clicked() {
                                    state.selected_index = i;
                                }
                            }
                        }
                        EncyclopediaTab::Characters => {
                            let chars: Vec<(CharacterKey, &str)> = world
                                .characters
                                .iter()
                                .map(|(k, c)| (k, c.name.as_str()))
                                .collect();
                            for (i, (_, name)) in chars.iter().enumerate() {
                                let sel = state.selected_index == i;
                                if ui.selectable_label(sel, *name).clicked() {
                                    state.selected_index = i;
                                }
                            }
                        }
                        EncyclopediaTab::Systems => {
                            let systems: Vec<(SystemKey, &str)> = world
                                .systems
                                .iter()
                                .map(|(k, s)| (k, s.name.as_str()))
                                .collect();
                            for (i, (_, name)) in systems.iter().enumerate() {
                                let sel = state.selected_index == i;
                                if ui.selectable_label(sel, *name).clicked() {
                                    state.selected_index = i;
                                }
                            }
                        }
                    });

                // Right: detail pane
                let detail_ui = &mut cols[1];
                ScrollArea::vertical()
                    .id_salt("enc_detail")
                    .show(detail_ui, |ui| {
                        match state.tab {
                            EncyclopediaTab::CapitalShips => {
                                let ships: Vec<CapitalShipKey> =
                                    world.capital_ship_classes.keys().collect();
                                if let Some(&key) = ships.get(state.selected_index) {
                                    if let Some(ship) = world.capital_ship_classes.get(key) {
                                        // GOKRES.DLL 122×50 ship status sprite.
                                        // Formula: resource_id = dat_id.raw() + 1024.
                                        // Ships without a sprite fall through to the EDATA image.
                                        let gokres_id = ship.dat_id.raw() + 1024;
                                        if let Some(tex) =
                                            bmp_cache.get(ctx, DllSource::Gokres, gokres_id)
                                        {
                                            ui.add(
                                                egui::Image::new(tex)
                                                    .fit_to_exact_size(Vec2::new(122.0, 50.0)),
                                            );
                                        }

                                        // EDATA offset for capital ships: 42 + 0-based index
                                        let edata_n = 42u16 + state.selected_index as u16;
                                        show_edata_image(ui, ctx, edata_n, state);
                                        ui.add_space(4.0);
                                        ui.heading(&ship.name);
                                        ui.separator();
                                        let faction = match (ship.is_alliance, ship.is_empire) {
                                            (true, false) => "Alliance",
                                            (false, true) => "Empire",
                                            _ => "Both",
                                        };
                                        stat_row(ui, "Faction", faction);
                                        stat_row(ui, "Hull", &ship.hull.to_string());
                                        stat_row(ui, "Shields", &ship.shield_strength.to_string());
                                        stat_row(
                                            ui,
                                            "Sublight",
                                            &ship.sub_light_engine.to_string(),
                                        );
                                        stat_row(ui, "Hyperdrive", &ship.hyperdrive.to_string());
                                        stat_row(ui, "Maneuver", &ship.maneuverability.to_string());
                                        stat_row(
                                            ui,
                                            "Fighters",
                                            &ship.fighter_capacity.to_string(),
                                        );
                                        stat_row(ui, "Troops", &ship.troop_capacity.to_string());
                                        stat_row(
                                            ui,
                                            "Build cost",
                                            &ship.refined_material_cost.to_string(),
                                        );
                                        stat_row(
                                            ui,
                                            "Maintenance",
                                            &ship.maintenance_cost.to_string(),
                                        );
                                        stat_row(
                                            ui,
                                            "Research order",
                                            &ship.research_order.to_string(),
                                        );
                                        stat_row(
                                            ui,
                                            "Build time",
                                            &ship.research_difficulty.to_string(),
                                        );
                                    }
                                }
                            }
                            EncyclopediaTab::Fighters => {
                                let fighters: Vec<FighterKey> =
                                    world.fighter_classes.keys().collect();
                                if let Some(&key) = fighters.get(state.selected_index) {
                                    if let Some(ftr) = world.fighter_classes.get(key) {
                                        // EDATA offset for fighters: 34 + 0-based index
                                        let edata_n = 34u16 + state.selected_index as u16;
                                        show_edata_image(ui, ctx, edata_n, state);
                                        ui.add_space(4.0);
                                        ui.heading(&ftr.name);
                                        ui.separator();
                                        let faction = match (ftr.is_alliance, ftr.is_empire) {
                                            (true, false) => "Alliance",
                                            (false, true) => "Empire",
                                            _ => "Both",
                                        };
                                        stat_row(ui, "Faction", faction);
                                        stat_row(
                                            ui,
                                            "Squadron size",
                                            &ftr.squadron_size.to_string(),
                                        );
                                        stat_row(ui, "Torpedoes", &ftr.torpedoes.to_string());
                                        stat_row(
                                            ui,
                                            "Build cost",
                                            &ftr.refined_material_cost.to_string(),
                                        );
                                        stat_row(
                                            ui,
                                            "Maintenance",
                                            &ftr.maintenance_cost.to_string(),
                                        );
                                    }
                                }
                            }
                            EncyclopediaTab::Characters => {
                                let chars: Vec<CharacterKey> = world.characters.keys().collect();
                                if let Some(&key) = chars.get(state.selected_index) {
                                    if let Some(chr) = world.characters.get(key) {
                                        // Major characters (0..5) → EDATA 72+; minor (6+) → 78+
                                        // We don't have a major/minor flag split by index here,
                                        // but world.characters stores major first (load order).
                                        // Use 72 for first 6, 78 for the rest.
                                        let edata_n = if state.selected_index < 6 {
                                            72u16 + state.selected_index as u16
                                        } else {
                                            78u16 + (state.selected_index - 6) as u16
                                        };
                                        show_edata_image(ui, ctx, edata_n, state);
                                        ui.add_space(4.0);
                                        ui.heading(&chr.name);
                                        ui.separator();
                                        let kind = if chr.is_major {
                                            "Major character"
                                        } else {
                                            "Minor character"
                                        };
                                        stat_row(ui, "Type", kind);
                                        stat_row_pair(
                                            ui,
                                            "Diplomacy",
                                            chr.diplomacy.base,
                                            chr.diplomacy.variance,
                                        );
                                        stat_row_pair(
                                            ui,
                                            "Espionage",
                                            chr.espionage.base,
                                            chr.espionage.variance,
                                        );
                                        stat_row_pair(
                                            ui,
                                            "Ship Design",
                                            chr.ship_design.base,
                                            chr.ship_design.variance,
                                        );
                                        stat_row_pair(
                                            ui,
                                            "Troop Training",
                                            chr.troop_training.base,
                                            chr.troop_training.variance,
                                        );
                                        stat_row_pair(
                                            ui,
                                            "Facility Design",
                                            chr.facility_design.base,
                                            chr.facility_design.variance,
                                        );
                                        stat_row_pair(
                                            ui,
                                            "Combat",
                                            chr.combat.base,
                                            chr.combat.variance,
                                        );
                                        stat_row_pair(
                                            ui,
                                            "Leadership",
                                            chr.leadership.base,
                                            chr.leadership.variance,
                                        );
                                        stat_row_pair(
                                            ui,
                                            "Loyalty",
                                            chr.loyalty.base,
                                            chr.loyalty.variance,
                                        );
                                        if chr.jedi_probability > 0 {
                                            stat_row(
                                                ui,
                                                "Jedi probability",
                                                &format!("{}%", chr.jedi_probability),
                                            );
                                        }
                                        let mut roles = Vec::new();
                                        if chr.can_be_admiral {
                                            roles.push("Admiral");
                                        }
                                        if chr.can_be_general {
                                            roles.push("General");
                                        }
                                        if chr.can_be_commander {
                                            roles.push("Commander");
                                        }
                                        if !roles.is_empty() {
                                            stat_row(ui, "Roles", &roles.join(", "));
                                        }
                                    }
                                }
                            }
                            EncyclopediaTab::Systems => {
                                let systems: Vec<SystemKey> = world.systems.keys().collect();
                                if let Some(&key) = systems.get(state.selected_index) {
                                    if let Some(system) = world.systems.get(key) {
                                        // Systems use ENCYBMAP.DLL for their image index.
                                        // Until ENCYBMAP is parsed, show a placeholder.
                                        show_placeholder_image(ui);
                                        ui.add_space(4.0);
                                        ui.heading(&system.name);
                                        ui.separator();
                                        if let Some(sector) = world.sectors.get(system.sector) {
                                            stat_row(ui, "Sector", &sector.name);
                                            let region = match sector.group {
                                                rebellion_core::dat::SectorGroup::Core => "Core",
                                                rebellion_core::dat::SectorGroup::RimInner => {
                                                    "Inner Rim"
                                                }
                                                rebellion_core::dat::SectorGroup::RimOuter => {
                                                    "Outer Rim"
                                                }
                                            };
                                            stat_row(ui, "Region", region);
                                        }
                                        stat_row(
                                            ui,
                                            "Position",
                                            &format!("({}, {})", system.x, system.y),
                                        );
                                        stat_row(
                                            ui,
                                            "Alliance",
                                            &format!("{:.0}%", system.popularity_alliance * 100.0),
                                        );
                                        stat_row(
                                            ui,
                                            "Empire",
                                            &format!("{:.0}%", system.popularity_empire * 100.0),
                                        );
                                        stat_row(ui, "Fleets", &system.fleets.len().to_string());
                                        stat_row(
                                            ui,
                                            "Defenses",
                                            &system.defense_facilities.len().to_string(),
                                        );
                                        stat_row(
                                            ui,
                                            "Shipyards",
                                            &system.manufacturing_facilities.len().to_string(),
                                        );

                                        ui.add_space(6.0);
                                        if ui.small_button("Zoom to system on map").clicked() {
                                            focus_system = Some(key);
                                        }
                                    }
                                }
                            }
                        }
                    });
            });
        });

    // Write back the open flag (egui sets it to false when the X button is clicked).
    state.open = window_open;

    focus_system
}

// ---------------------------------------------------------------------------
// Original index shell
// ---------------------------------------------------------------------------

/// Native Galactic Encyclopedia window dimensions from `FUN_00429f30`.
pub const ENCYCLOPEDIA_INDEX_WIDTH: f32 = 470.0;
pub const ENCYCLOPEDIA_INDEX_HEIGHT: f32 = 330.0;

const ORIGINAL_INDEX_VISIBLE_ROWS: usize = 9;
const ORIGINAL_INDEX_ROW_HEIGHT: f32 = 18.0;

/// One immutable source object shown by the authentic index list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OriginalEncyclopediaEntry {
    pub object_id: u32,
    pub name: String,
}

impl OriginalEncyclopediaEntry {
    #[must_use]
    pub const fn family(&self) -> u8 {
        (self.object_id >> 24) as u8
    }
}

/// Localized source catalog transported from the original DAT/TEXTSTRA data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OriginalEncyclopediaCatalog {
    pub title: String,
    pub topic_label: String,
    category_labels: [String; 7],
    entries: Vec<OriginalEncyclopediaEntry>,
}

impl OriginalEncyclopediaCatalog {
    #[must_use]
    pub fn new(
        title: String,
        topic_label: String,
        category_labels: [String; 7],
        mut entries: Vec<OriginalEncyclopediaEntry>,
    ) -> Self {
        entries.sort_by(|left, right| {
            left.name
                .to_lowercase()
                .cmp(&right.name.to_lowercase())
                .then_with(|| left.object_id.cmp(&right.object_id))
        });
        entries.dedup_by_key(|entry| entry.object_id);
        Self {
            title,
            topic_label,
            category_labels,
            entries,
        }
    }

    #[must_use]
    pub fn category_label(&self, command_id: u16) -> Option<&str> {
        let index = usize::from(command_id.checked_sub(0x6f)?);
        self.category_labels.get(index).map(String::as_str)
    }

    #[must_use]
    pub fn entries_for(&self, command_id: u16) -> Vec<&OriginalEncyclopediaEntry> {
        if !(0x6f..=0x75).contains(&command_id) {
            return Vec::new();
        }
        self.entries
            .iter()
            .filter(|entry| original_category_contains(command_id, entry.family()))
            .collect()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

const fn original_category_contains(command_id: u16, family: u8) -> bool {
    match command_id {
        0x6f => true,
        0x70 => family >= 0x90 && family < 0x98,
        0x71 => family >= 0x14 && family < 0x20,
        0x72 => family >= 0x20 && family < 0x30,
        0x73 => family >= 0x40 && family < 0x80,
        0x74 => family >= 0x10 && family < 0x14,
        0x75 => family >= 0x30 && family < 0x40,
        _ => false,
    }
}

const INDEX_CONTENT: u32 = 10_338;
const INDEX_CONTENT_X: f32 = 12.0;
const INDEX_CONTENT_Y: f32 = 13.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OriginalControlSpec {
    command_id: u16,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    normal_resource: u32,
    pressed_resource: u32,
    selected_in_index: bool,
}

const fn encyclopedia_category_controls(
    faction: crate::cockpit::CockpitFaction,
) -> [OriginalControlSpec; 7] {
    let (ships_normal, ships_pressed, facilities_normal, facilities_pressed) = match faction {
        crate::cockpit::CockpitFaction::Alliance => (10_348, 10_347, 10_344, 10_343),
        crate::cockpit::CockpitFaction::Empire => (10_360, 10_359, 10_356, 10_355),
    };
    let (personnel_normal, personnel_pressed, missions_normal, missions_pressed) = match faction {
        crate::cockpit::CockpitFaction::Alliance => (10_346, 10_345, 11_616, 11_615),
        crate::cockpit::CockpitFaction::Empire => (10_358, 10_357, 11_618, 11_617),
    };
    let (troops_normal, troops_pressed) = match faction {
        crate::cockpit::CockpitFaction::Alliance => (10_352, 10_351),
        crate::cockpit::CockpitFaction::Empire => (10_362, 10_361),
    };
    [
        OriginalControlSpec {
            command_id: 0x6f,
            x: 36,
            y: 78,
            width: 49,
            height: 41,
            normal_resource: 10_340,
            pressed_resource: 10_339,
            selected_in_index: false,
        },
        OriginalControlSpec {
            command_id: 0x70,
            x: 88,
            y: 78,
            width: 49,
            height: 41,
            normal_resource: 10_350,
            pressed_resource: 10_349,
            selected_in_index: false,
        },
        OriginalControlSpec {
            command_id: 0x71,
            x: 140,
            y: 78,
            width: 49,
            height: 41,
            normal_resource: ships_normal,
            pressed_resource: ships_pressed,
            selected_in_index: false,
        },
        OriginalControlSpec {
            command_id: 0x72,
            x: 192,
            y: 78,
            width: 49,
            height: 41,
            normal_resource: facilities_normal,
            pressed_resource: facilities_pressed,
            selected_in_index: false,
        },
        OriginalControlSpec {
            command_id: 0x73,
            x: 244,
            y: 78,
            width: 49,
            height: 41,
            normal_resource: missions_normal,
            pressed_resource: missions_pressed,
            selected_in_index: false,
        },
        OriginalControlSpec {
            command_id: 0x74,
            x: 296,
            y: 78,
            width: 49,
            height: 41,
            normal_resource: troops_normal,
            pressed_resource: troops_pressed,
            selected_in_index: false,
        },
        OriginalControlSpec {
            command_id: 0x75,
            x: 348,
            y: 78,
            width: 49,
            height: 41,
            normal_resource: personnel_normal,
            pressed_resource: personnel_pressed,
            selected_in_index: false,
        },
    ]
}

const fn encyclopedia_rail_controls(
    faction: crate::cockpit::CockpitFaction,
) -> [OriginalControlSpec; 3] {
    match faction {
        crate::cockpit::CockpitFaction::Alliance => [
            OriginalControlSpec {
                command_id: 0xfb,
                x: 423,
                y: 25,
                width: 32,
                height: 31,
                normal_resource: 10_370,
                pressed_resource: 10_371,
                selected_in_index: false,
            },
            OriginalControlSpec {
                command_id: 0x67,
                x: 423,
                y: 93,
                width: 32,
                height: 31,
                normal_resource: 10_374,
                pressed_resource: 10_375,
                selected_in_index: false,
            },
            OriginalControlSpec {
                command_id: 0x68,
                x: 423,
                y: 147,
                width: 32,
                height: 31,
                normal_resource: 10_372,
                pressed_resource: 10_373,
                selected_in_index: true,
            },
        ],
        crate::cockpit::CockpitFaction::Empire => [
            OriginalControlSpec {
                command_id: 0xfb,
                x: 426,
                y: 21,
                width: 44,
                height: 41,
                normal_resource: 10_376,
                pressed_resource: 10_377,
                selected_in_index: false,
            },
            OriginalControlSpec {
                command_id: 0x67,
                x: 426,
                y: 89,
                width: 44,
                height: 41,
                normal_resource: 10_380,
                pressed_resource: 10_381,
                selected_in_index: false,
            },
            OriginalControlSpec {
                command_id: 0x68,
                x: 426,
                y: 143,
                width: 44,
                height: 41,
                normal_resource: 10_378,
                pressed_resource: 10_379,
                selected_in_index: true,
            },
        ],
    }
}

/// Paint the source-exact Galactic Encyclopedia index bitmap layers.
///
/// Text labels and the object list are intentionally absent until their
/// `TEXTSTRA` and native-list contracts are transported. Category resource
/// `0x75` is painted at native size and clipped to its 49-by-41 control, as
/// the original constructor does; it is never scaled from 57 pixels high.
pub fn draw_encyclopedia_index_shell(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    faction: crate::cockpit::CockpitFaction,
    origin: egui::Pos2,
    scale: f32,
    selected_category: u16,
) -> Option<u16> {
    draw_encyclopedia_index_layers(ctx, cache, faction, origin, scale, selected_category, None)
        .activated_command
}

/// Paint the authentic index shell together with localized source rows.
///
/// The catalog is immutable reference data and selection is stored by compound
/// object id, so category changes and source reordering cannot silently select
/// a different object. Topic composition remains a later parity checkpoint.
pub fn draw_encyclopedia_index_catalog(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    faction: crate::cockpit::CockpitFaction,
    origin: egui::Pos2,
    scale: f32,
    state: &mut EncyclopediaState,
    catalog: &OriginalEncyclopediaCatalog,
) -> Option<u16> {
    if scale <= 0.0 || catalog.is_empty() {
        return None;
    }
    reconcile_original_index_state(state, catalog, false);
    let entries = catalog.entries_for(state.original_category_command);
    let selected_name = entries
        .iter()
        .find(|entry| Some(entry.object_id) == state.original_selected_object_id)
        .map(|entry| entry.name.as_str())
        .unwrap_or("");
    let category_label = catalog
        .category_label(state.original_category_command)
        .unwrap_or("");
    let content = OriginalIndexContent {
        title: &catalog.title,
        topic_label: &catalog.topic_label,
        category_label,
        selected_name,
        entries: &entries,
        selected_object_id: state.original_selected_object_id,
        scroll_row: state.original_scroll_row,
    };
    let outcome = draw_encyclopedia_index_layers(
        ctx,
        cache,
        faction,
        origin,
        scale,
        state.original_category_command,
        Some(content),
    );
    if let Some(command) = outcome.activated_command {
        if (0x6f..=0x75).contains(&command) && command != state.original_category_command {
            state.original_category_command = command;
            reconcile_original_index_state(state, catalog, true);
        }
    }
    if let Some(scroll_row) = outcome.scroll_row {
        state.original_scroll_row = scroll_row;
        reconcile_original_index_state(state, catalog, false);
    }
    if let Some(object_id) = outcome.selected_object_id {
        state.original_selected_object_id = Some(object_id);
        reconcile_original_index_state(state, catalog, true);
    }
    outcome.activated_command
}

fn reconcile_original_index_state(
    state: &mut EncyclopediaState,
    catalog: &OriginalEncyclopediaCatalog,
    reveal_selection: bool,
) {
    if !(0x6f..=0x75).contains(&state.original_category_command) {
        state.original_category_command = 0x6f;
    }
    let entries = catalog.entries_for(state.original_category_command);
    if !entries
        .iter()
        .any(|entry| Some(entry.object_id) == state.original_selected_object_id)
    {
        state.original_selected_object_id = entries.first().map(|entry| entry.object_id);
        state.original_scroll_row = 0;
    }
    let selected_index = entries
        .iter()
        .position(|entry| Some(entry.object_id) == state.original_selected_object_id)
        .unwrap_or(0);
    let max_scroll = entries.len().saturating_sub(ORIGINAL_INDEX_VISIBLE_ROWS);
    state.original_scroll_row = state.original_scroll_row.min(max_scroll);
    if reveal_selection {
        if selected_index < state.original_scroll_row {
            state.original_scroll_row = selected_index;
        } else if selected_index >= state.original_scroll_row + ORIGINAL_INDEX_VISIBLE_ROWS {
            state.original_scroll_row = selected_index + 1 - ORIGINAL_INDEX_VISIBLE_ROWS;
        }
    }
}

#[derive(Default)]
struct OriginalIndexOutcome {
    activated_command: Option<u16>,
    selected_object_id: Option<u32>,
    scroll_row: Option<usize>,
}

struct OriginalIndexContent<'a> {
    title: &'a str,
    topic_label: &'a str,
    category_label: &'a str,
    selected_name: &'a str,
    entries: &'a [&'a OriginalEncyclopediaEntry],
    selected_object_id: Option<u32>,
    scroll_row: usize,
}

fn draw_encyclopedia_index_layers(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    faction: crate::cockpit::CockpitFaction,
    origin: egui::Pos2,
    scale: f32,
    selected_category: u16,
    content: Option<OriginalIndexContent<'_>>,
) -> OriginalIndexOutcome {
    if scale <= 0.0 || !(0x6f..=0x75).contains(&selected_category) {
        return OriginalIndexOutcome::default();
    }

    let mut outcome = OriginalIndexOutcome::default();
    egui::Area::new(egui::Id::new("original-encyclopedia-index-shell"))
        .fixed_pos(origin)
        .order(egui::Order::Middle)
        .show(ctx, |ui| {
            let size = egui::vec2(
                ENCYCLOPEDIA_INDEX_WIDTH * scale,
                ENCYCLOPEDIA_INDEX_HEIGHT * scale,
            );
            let (window_rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
            let (base, rail) = match faction {
                crate::cockpit::CockpitFaction::Alliance => (10_335, 10_585),
                crate::cockpit::CockpitFaction::Empire => (10_336, 10_589),
            };
            paint_original_resource_native(
                ui.painter(),
                ctx,
                cache,
                base,
                window_rect.min,
                scale,
                window_rect,
            );
            paint_original_resource_native(
                ui.painter(),
                ctx,
                cache,
                rail,
                encyclopedia_point(window_rect, scale, 412.0, 0.0),
                scale,
                window_rect,
            );
            let content_rect = encyclopedia_rect(
                window_rect,
                scale,
                INDEX_CONTENT_X,
                INDEX_CONTENT_Y,
                400.0,
                306.0,
            );
            paint_original_resource_native(
                ui.painter(),
                ctx,
                cache,
                INDEX_CONTENT,
                content_rect.min,
                scale,
                content_rect,
            );

            for control in encyclopedia_category_controls(faction) {
                if draw_original_control(
                    ui,
                    ctx,
                    cache,
                    window_rect,
                    scale,
                    control,
                    control.command_id == selected_category,
                ) {
                    outcome.activated_command = Some(control.command_id);
                }
            }
            for control in encyclopedia_rail_controls(faction) {
                if draw_original_control(
                    ui,
                    ctx,
                    cache,
                    window_rect,
                    scale,
                    control,
                    control.selected_in_index,
                ) {
                    outcome.activated_command = Some(control.command_id);
                }
            }
            if let Some(content) = content {
                draw_original_index_content(ui, window_rect, scale, content, &mut outcome);
            }
        });
    outcome
}

fn draw_original_index_content(
    ui: &mut egui::Ui,
    window_rect: egui::Rect,
    scale: f32,
    content: OriginalIndexContent<'_>,
    outcome: &mut OriginalIndexOutcome,
) {
    let painter = ui.painter();
    let title_font = egui::FontId::proportional(15.0 * scale);
    let list_font = egui::FontId::proportional(14.0 * scale);
    painter.text(
        encyclopedia_point(window_rect, scale, 211.0, 14.0),
        egui::Align2::CENTER_TOP,
        content.title,
        title_font,
        Color32::WHITE,
    );
    painter.text(
        encyclopedia_point(window_rect, scale, 36.0, 48.0),
        egui::Align2::LEFT_TOP,
        content.topic_label,
        list_font.clone(),
        Color32::WHITE,
    );
    painter.text(
        encyclopedia_point(window_rect, scale, 143.0, 47.0),
        egui::Align2::LEFT_TOP,
        content.selected_name,
        list_font.clone(),
        Color32::WHITE,
    );
    painter.text(
        encyclopedia_point(window_rect, scale, 40.0, 120.0),
        egui::Align2::LEFT_TOP,
        content.category_label,
        list_font.clone(),
        Color32::WHITE,
    );

    let max_scroll = content
        .entries
        .len()
        .saturating_sub(ORIGINAL_INDEX_VISIBLE_ROWS);
    let list_rect = original_index_list_rect(window_rect, scale);
    let list_painter = painter.with_clip_rect(list_rect);
    if ui.rect_contains_pointer(list_rect) {
        let wheel = ui.input(|input| input.raw_scroll_delta.y);
        if wheel.abs() > f32::EPSILON {
            let rows = ((wheel.abs() / 24.0).ceil() as usize).clamp(1, 3);
            outcome.scroll_row = Some(if wheel < 0.0 {
                content.scroll_row.saturating_add(rows).min(max_scroll)
            } else {
                content.scroll_row.saturating_sub(rows)
            });
        }
    }
    let up_rect = encyclopedia_rect(window_rect, scale, 374.0, 137.0, 12.0, 13.0);
    let down_rect = encyclopedia_rect(window_rect, scale, 374.0, 284.0, 12.0, 13.0);
    if ui
        .interact(
            up_rect,
            ui.id().with("encyclopedia-index-scroll-up"),
            egui::Sense::click(),
        )
        .clicked()
    {
        outcome.scroll_row = Some(content.scroll_row.saturating_sub(1));
    }
    if ui
        .interact(
            down_rect,
            ui.id().with("encyclopedia-index-scroll-down"),
            egui::Sense::click(),
        )
        .clicked()
    {
        outcome.scroll_row = Some(content.scroll_row.saturating_add(1).min(max_scroll));
    }

    for (visible_row, entry) in content
        .entries
        .iter()
        .skip(content.scroll_row)
        .take(ORIGINAL_INDEX_VISIBLE_ROWS)
        .enumerate()
    {
        let y = 137.0 + visible_row as f32 * ORIGINAL_INDEX_ROW_HEIGHT;
        let row_rect = original_index_row_rect(window_rect, scale, visible_row);
        let selected = Some(entry.object_id) == content.selected_object_id;
        if selected {
            list_painter.rect_filled(row_rect, 0.0, Color32::from_rgb(0, 0, 96));
        }
        let response = ui.interact(
            row_rect,
            ui.id().with(("encyclopedia-index-row", entry.object_id)),
            egui::Sense::click(),
        );
        list_painter.text(
            encyclopedia_point(window_rect, scale, 40.0, y + 1.0),
            egui::Align2::LEFT_TOP,
            &entry.name,
            list_font.clone(),
            Color32::WHITE,
        );
        if response.clicked() {
            outcome.selected_object_id = Some(entry.object_id);
        }
    }
}

fn draw_original_control(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    cache: &mut BmpCache,
    window_rect: egui::Rect,
    scale: f32,
    control: OriginalControlSpec,
    selected: bool,
) -> bool {
    let rect = encyclopedia_rect(
        window_rect,
        scale,
        f32::from(control.x),
        f32::from(control.y),
        f32::from(control.width),
        f32::from(control.height),
    );
    let control_id = ui
        .id()
        .with(("encyclopedia-original-control", control.command_id));
    let response = ui.interact(rect, control_id, egui::Sense::click());
    let capture_id = control_id.with("opaque-press-origin");
    if ctx.input(|input| input.pointer.primary_pressed()) {
        let captured = ctx.pointer_latest_pos().is_some_and(|point| {
            original_control_contains(cache, control.normal_resource, rect, scale, point)
        });
        ui.data_mut(|data| data.insert_temp(capture_id, captured));
    }
    let captured = ui.data(|data| data.get_temp::<bool>(capture_id).unwrap_or(false));
    let (pointer, primary_down) = ctx.input(|input| {
        (
            input.pointer.interact_pos(),
            input.pointer.button_down(egui::PointerButton::Primary),
        )
    });
    let pointer_hits = pointer.is_some_and(|point| {
        original_control_contains(cache, control.normal_resource, rect, scale, point)
    });
    let pressed = selected
        || (primary_down && response.is_pointer_button_down_on() && captured && pointer_hits);
    paint_original_resource_native(
        ui.painter(),
        ctx,
        cache,
        if pressed {
            control.pressed_resource
        } else {
            control.normal_resource
        },
        rect.min,
        scale,
        rect,
    );
    captured
        && response.clicked()
        && response.interact_pointer_pos().is_some_and(|point| {
            original_control_contains(cache, control.normal_resource, rect, scale, point)
        })
}

fn original_control_contains(
    cache: &mut BmpCache,
    resource_id: u32,
    rect: egui::Rect,
    scale: f32,
    point: egui::Pos2,
) -> bool {
    if point.x < rect.min.x
        || point.x >= rect.max.x
        || point.y < rect.min.y
        || point.y >= rect.max.y
    {
        return false;
    }
    let x = ((point.x - rect.min.x) / scale).floor() as usize;
    let y = ((point.y - rect.min.y) / scale).floor() as usize;
    cache.is_resource_hit(DllSource::Strategy, resource_id, x, y)
}

fn encyclopedia_point(parent: egui::Rect, scale: f32, x: f32, y: f32) -> egui::Pos2 {
    egui::pos2(parent.min.x + x * scale, parent.min.y + y * scale)
}

fn encyclopedia_rect(
    parent: egui::Rect,
    scale: f32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> egui::Rect {
    egui::Rect::from_min_size(
        encyclopedia_point(parent, scale, x, y),
        egui::vec2(width * scale, height * scale),
    )
}

fn original_index_list_rect(parent: egui::Rect, scale: f32) -> egui::Rect {
    encyclopedia_rect(parent, scale, 36.0, 137.0, 350.0, 160.0)
}

fn original_index_row_rect(parent: egui::Rect, scale: f32, visible_row: usize) -> egui::Rect {
    let y = 137.0 + visible_row as f32 * ORIGINAL_INDEX_ROW_HEIGHT;
    encyclopedia_rect(parent, scale, 36.0, y, 338.0, ORIGINAL_INDEX_ROW_HEIGHT)
        .intersect(original_index_list_rect(parent, scale))
}

fn paint_original_resource_native(
    painter: &egui::Painter,
    ctx: &egui::Context,
    cache: &mut BmpCache,
    resource_id: u32,
    origin: egui::Pos2,
    scale: f32,
    clip_rect: egui::Rect,
) {
    let Some(texture) = cache.get(ctx, DllSource::Strategy, resource_id) else {
        return;
    };
    let rect = egui::Rect::from_min_size(origin, texture.size_vec2() * scale);
    painter.with_clip_rect(clip_rect).image(
        texture.id(),
        rect,
        egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
        Color32::WHITE,
    );
}

/// Fixed-position test adapter for deterministic browser inspection.
#[cfg(feature = "interface-test-fixtures")]
pub fn draw_encyclopedia_index_fixture(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    faction: crate::cockpit::CockpitFaction,
    selected_category: u16,
) -> Option<u16> {
    draw_encyclopedia_index_shell(
        ctx,
        cache,
        faction,
        egui::pos2(85.0, 55.0),
        1.0,
        selected_category,
    )
}

/// Fixed-position catalog adapter for deterministic browser inspection.
#[cfg(feature = "interface-test-fixtures")]
pub fn draw_encyclopedia_index_catalog_fixture(
    ctx: &egui::Context,
    cache: &mut BmpCache,
    faction: crate::cockpit::CockpitFaction,
    state: &mut EncyclopediaState,
    catalog: &OriginalEncyclopediaCatalog,
) -> Option<u16> {
    draw_encyclopedia_index_catalog(
        ctx,
        cache,
        faction,
        egui::pos2(85.0, 55.0),
        1.0,
        state,
        catalog,
    )
}

// ---------------------------------------------------------------------------
// Image helpers
// ---------------------------------------------------------------------------

/// Load and display an EDATA BMP image, caching the texture by EDATA number.
///
/// On first call for a given `edata_n`, reads the BMP file, decodes it via
/// the `image` crate, and registers it as an egui texture.  Subsequent calls
/// use the cached handle.  If the file is missing or fails to decode, shows
/// a gray placeholder rectangle.
fn show_edata_image(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    edata_n: u16,
    state: &mut EncyclopediaState,
) {
    // Lazy-load the texture if not yet cached. HD is an explicit enhancement
    // profile; original parity never probes the HD tree.
    if !state.textures.contains_key(&edata_n) {
        let handle = load_edata_texture(
            ctx,
            edata_n,
            state.hd_path.as_deref(),
            state.edata_path.as_deref(),
            state.asset_profile,
            state
                .approved_hd_assets
                .get(&format!("edata/EDATA_{edata_n:03}")),
        );
        if handle.is_none() {
            eprintln!(
                "[encyclopedia] original artwork unavailable asset={}",
                edata_filename(edata_n)
            );
        }
        state.textures.insert(edata_n, handle);
    }

    if let Some(Some(handle)) = state.textures.get(&edata_n) {
        let size = egui::vec2(400.0, 200.0);
        ui.add(egui::Image::from_texture((handle.id(), size)));
    } else {
        show_placeholder_image(ui);
    }
}

/// Paint one original EDATA image at a fixed native-size location for the
/// test-only browser transport gate.
///
/// This is not an encyclopedia UI and is absent from production builds. It
/// isolates the runtime-pack/cache/decode/render path so exact source pixels
/// can be compared without promoting the current replacement window.
#[cfg(feature = "interface-test-fixtures")]
pub fn draw_encyclopedia_artwork_fixture(
    ctx: &egui::Context,
    edata_n: u16,
    state: &mut EncyclopediaState,
) {
    egui::Area::new(egui::Id::new("encyclopedia_artwork_transport_fixture"))
        .fixed_pos(egui::pos2(120.0, 120.0))
        .show(ctx, |ui| show_edata_image(ui, ctx, edata_n, state));
}

/// Draw a gray placeholder rectangle when no image is available.
fn show_placeholder_image(ui: &mut egui::Ui) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(400.0, 200.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 4.0, Color32::from_gray(40));
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        "No image",
        egui::FontId::default(),
        Color32::from_gray(100),
    );
}

/// Load an EDATA image and register it as an egui texture.
///
/// In faithful-HD mode, checks for an approved PNG first (`EDATA_NNN.png` in
/// `hd_path`), then falls back to the original BMP (`EDATA.NNN` in
/// `edata_path`). Original-parity mode never probes the HD path.
/// Returns `None` if neither exists or decoding fails.
/// On WASM targets, always returns `None` (filesystem access not available).
#[cfg(not(target_arch = "wasm32"))]
fn load_edata_texture(
    ctx: &egui::Context,
    edata_n: u16,
    hd_path: Option<&Path>,
    edata_path: Option<&Path>,
    profile: AssetRenderProfile,
    approved_hd: Option<&ApprovedHdAsset>,
) -> Option<TextureHandle> {
    let dir = edata_path?;
    let bmp_file = dir.join(edata_filename(edata_n));

    if profile == AssetRenderProfile::FaithfulHd {
        if let Some(hd_dir) = hd_path {
            let hd_file = hd_dir.join(format!("EDATA_{edata_n:03}.png"));
            if let Some(bytes) =
                approved_hd.and_then(|approval| validated_hd_bytes(&bmp_file, &hd_file, approval))
            {
                if let Some(handle) = load_image_bytes(ctx, edata_n, &bytes, TextureOptions::LINEAR)
                {
                    return Some(handle);
                }
            } else if approved_hd.is_some() && hd_file.exists() {
                eprintln!(
                    "[encyclopedia] HD source/output digest mismatch for EDATA_{edata_n:03}; falling back to original"
                );
            }
        }
    }

    // Original data is authoritative and uses exact nearest sampling.
    if bmp_file.exists() {
        return load_image_file(ctx, edata_n, &bmp_file, TextureOptions::NEAREST);
    }

    None
}

/// Decode an image file (BMP or PNG) and register it as an egui texture.
#[cfg(not(target_arch = "wasm32"))]
fn load_image_file(
    ctx: &egui::Context,
    edata_n: u16,
    path: &Path,
    texture_options: TextureOptions,
) -> Option<TextureHandle> {
    let bytes = std::fs::read(path).ok()?;

    load_image_bytes(ctx, edata_n, &bytes, texture_options)
}

fn load_image_bytes(
    ctx: &egui::Context,
    edata_n: u16,
    bytes: &[u8],
    texture_options: TextureOptions,
) -> Option<TextureHandle> {
    // image crate auto-detects format from magic bytes.
    let img = image::load_from_memory(bytes).ok()?;
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();

    let color_image =
        egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], rgba.as_raw());

    let handle = ctx.load_texture(format!("edata_{edata_n}"), color_image, texture_options);

    Some(handle)
}

#[cfg(target_arch = "wasm32")]
fn load_edata_texture(
    ctx: &egui::Context,
    edata_n: u16,
    _hd_path: Option<&Path>,
    _edata_path: Option<&Path>,
    _profile: AssetRenderProfile,
    _approved_hd: Option<&ApprovedHdAsset>,
) -> Option<TextureHandle> {
    let bytes = wasm_edata_bytes(edata_n)?;
    load_image_bytes(ctx, edata_n, &bytes, TextureOptions::NEAREST)
}

// ---------------------------------------------------------------------------
// Stat display helpers
// ---------------------------------------------------------------------------

/// Two-column stat row: label on left, value on right.
fn stat_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(format!("{label}:"))
                .small()
                .color(Color32::from_gray(160)),
        );
        ui.label(RichText::new(value).small());
    });
}

/// Two-column stat row for `SkillPair` values (base ± variance).
fn stat_row_pair(ui: &mut egui::Ui, label: &str, base: u32, variance: u32) {
    let value = if variance > 0 {
        format!("{base} ± {variance}")
    } else {
        base.to_string()
    };
    stat_row(ui, label, &value);
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::encyclopedia_navigation::{
        apply_encyclopedia_action, BodyScrollIntent, EncyclopediaAction, EncyclopediaMode,
        EncyclopediaState as EncyclopediaNavigationState, NavigationOutcome, SourceKeyIntent,
    };
    use crate::encyclopedia_textures::{EguiEncyclopediaTextureBackend, EncyclopediaTextureCache};
    use crate::encyclopedia_view::{
        ActiveTopicView, CategoryViewItem, EncyclopediaSelection, EncyclopediaView,
        NavigationState, StatRowView, TopicImageRenderProfile, TopicImageView, TopicViewItem,
    };

    fn synthetic_indexed_edata() -> Vec<u8> {
        const WIDTH: usize = 400;
        const HEIGHT: usize = 200;
        const PIXEL_OFFSET: usize = 1_078;
        let mut bytes = vec![0_u8; PIXEL_OFFSET + WIDTH * HEIGHT];
        let file_size = bytes.len() as u32;
        bytes[0..2].copy_from_slice(b"BM");
        bytes[2..6].copy_from_slice(&file_size.to_le_bytes());
        bytes[10..14].copy_from_slice(&(PIXEL_OFFSET as u32).to_le_bytes());
        bytes[14..18].copy_from_slice(&40_u32.to_le_bytes());
        bytes[18..22].copy_from_slice(&(WIDTH as i32).to_le_bytes());
        bytes[22..26].copy_from_slice(&(HEIGHT as i32).to_le_bytes());
        bytes[26..28].copy_from_slice(&1_u16.to_le_bytes());
        bytes[28..30].copy_from_slice(&8_u16.to_le_bytes());
        bytes[34..38].copy_from_slice(&((WIDTH * HEIGHT) as u32).to_le_bytes());
        bytes[46..50].copy_from_slice(&256_u32.to_le_bytes());
        bytes[54 + 4..54 + 8].copy_from_slice(&[0x20, 0x80, 0xe0, 0]);
        bytes[PIXEL_OFFSET..].fill(1);
        bytes
    }

    #[test]
    fn encyclopedia_defaults_to_original_parity() {
        let state = EncyclopediaState::new();
        assert_eq!(state.asset_profile, AssetRenderProfile::OriginalParity);
    }

    #[test]
    fn changing_asset_profile_invalidates_cached_images() {
        let mut state = EncyclopediaState::new();
        state.textures.insert(42, None);

        state.set_asset_profile(AssetRenderProfile::FaithfulHd);

        assert!(state.textures.is_empty());
        assert_eq!(state.asset_profile, AssetRenderProfile::FaithfulHd);
    }

    #[test]
    fn edata_filenames_preserve_the_original_three_digit_identity() {
        assert_eq!(edata_filename(1), "EDATA.001");
        assert_eq!(edata_filename(42), "EDATA.042");
        assert_eq!(edata_filename(192), "EDATA.192");
    }

    #[test]
    fn original_edata_bytes_decode_at_source_size() {
        let ctx = egui::Context::default();
        let texture = load_image_bytes(
            &ctx,
            42,
            &synthetic_indexed_edata(),
            TextureOptions::NEAREST,
        )
        .expect("synthetic original EDATA should decode");

        assert_eq!(texture.size(), [400, 200]);
    }

    #[test]
    fn encyclopedia_category_geometry_matches_the_recovered_native_controls() {
        let controls = encyclopedia_control_specs(CockpitFaction::Alliance);

        assert_eq!(
            controls.map(|control| {
                (
                    control.command_id,
                    control.x,
                    control.y,
                    control.width,
                    control.height,
                )
            }),
            [
                (0x6f, 36, 78, 49, 41),
                (0x70, 88, 78, 49, 41),
                (0x71, 140, 78, 49, 41),
                (0x72, 192, 78, 49, 41),
                (0x73, 244, 78, 49, 41),
                (0x74, 296, 78, 49, 41),
                (0x75, 348, 78, 49, 41),
            ]
        );
        assert_eq!(
            controls.last().unwrap().x + controls.last().unwrap().width,
            397
        );
    }

    #[test]
    fn encyclopedia_category_resources_preserve_faction_variants() {
        let alliance = encyclopedia_control_specs(CockpitFaction::Alliance);
        let empire = encyclopedia_control_specs(CockpitFaction::Empire);

        assert_eq!(
            alliance.map(|control| (control.normal_resource, control.pressed_resource)),
            [
                (0x2864, 0x2863),
                (0x286e, 0x286d),
                (0x286c, 0x286b),
                (0x2868, 0x2867),
                (0x2d60, 0x2d5f),
                (0x2870, 0x286f),
                (0x286a, 0x2869),
            ]
        );
        assert_eq!(
            empire.map(|control| (control.normal_resource, control.pressed_resource)),
            [
                (0x2864, 0x2863),
                (0x286e, 0x286d),
                (0x2878, 0x2877),
                (0x2874, 0x2873),
                (0x2d62, 0x2d61),
                (0x287a, 0x2879),
                (0x2876, 0x2875),
            ]
        );
    }

    #[test]
    fn original_index_categories_match_the_recovered_command_geometry() {
        let controls = encyclopedia_category_controls(CockpitFaction::Alliance);

        assert_eq!(
            controls.map(|control| (
                control.command_id,
                control.x,
                control.y,
                control.width,
                control.height,
            )),
            [
                (0x6f, 36, 78, 49, 41),
                (0x70, 88, 78, 49, 41),
                (0x71, 140, 78, 49, 41),
                (0x72, 192, 78, 49, 41),
                (0x73, 244, 78, 49, 41),
                (0x74, 296, 78, 49, 41),
                (0x75, 348, 78, 49, 41),
            ]
        );
    }

    #[test]
    fn original_index_category_resources_preserve_faction_variants() {
        let alliance = encyclopedia_category_controls(CockpitFaction::Alliance);
        let empire = encyclopedia_category_controls(CockpitFaction::Empire);

        assert_eq!(
            alliance.map(|control| (control.normal_resource, control.pressed_resource)),
            [
                (10_340, 10_339),
                (10_350, 10_349),
                (10_348, 10_347),
                (10_344, 10_343),
                (11_616, 11_615),
                (10_352, 10_351),
                (10_346, 10_345),
            ]
        );
        assert_eq!(
            empire.map(|control| (control.normal_resource, control.pressed_resource)),
            [
                (10_340, 10_339),
                (10_350, 10_349),
                (10_360, 10_359),
                (10_356, 10_355),
                (11_618, 11_617),
                (10_362, 10_361),
                (10_358, 10_357),
            ]
        );
    }

    #[test]
    fn original_index_rail_preserves_faction_geometry_and_selected_mode() {
        let alliance = encyclopedia_rail_controls(CockpitFaction::Alliance);
        let empire = encyclopedia_rail_controls(CockpitFaction::Empire);

        assert_eq!(
            alliance.map(|control| (
                control.command_id,
                control.x,
                control.y,
                control.width,
                control.height,
                control.normal_resource,
                control.pressed_resource,
                control.selected_in_index,
            )),
            [
                (0xfb, 423, 25, 32, 31, 10_370, 10_371, false),
                (0x67, 423, 93, 32, 31, 10_374, 10_375, false),
                (0x68, 423, 147, 32, 31, 10_372, 10_373, true),
            ]
        );
        assert_eq!(
            empire.map(|control| (
                control.command_id,
                control.x,
                control.y,
                control.width,
                control.height,
                control.normal_resource,
                control.pressed_resource,
                control.selected_in_index,
            )),
            [
                (0xfb, 426, 21, 44, 41, 10_376, 10_377, false),
                (0x67, 426, 89, 44, 41, 10_380, 10_381, false),
                (0x68, 426, 143, 44, 41, 10_378, 10_379, true),
            ]
        );
    }

    fn original_catalog_fixture() -> OriginalEncyclopediaCatalog {
        OriginalEncyclopediaCatalog::new(
            "Galactic Encyclopedia".into(),
            "Topic".into(),
            [
                "All Databases".into(),
                "System Database".into(),
                "Ship Database".into(),
                "Facilities Database".into(),
                "Missions Database".into(),
                "Troop Database".into(),
                "Personnel Database".into(),
            ],
            vec![
                OriginalEncyclopediaEntry {
                    object_id: 0x9000_0001,
                    name: "Allyuen".into(),
                },
                OriginalEncyclopediaEntry {
                    object_id: 0x1400_0001,
                    name: "Alliance Dreadnaught".into(),
                },
                OriginalEncyclopediaEntry {
                    object_id: 0x1000_0001,
                    name: "Alliance Army Regiment".into(),
                },
                OriginalEncyclopediaEntry {
                    object_id: 0x3000_0001,
                    name: "Ackbar".into(),
                },
            ],
        )
    }

    #[test]
    fn original_catalog_uses_source_labels_and_family_filters() {
        let catalog = original_catalog_fixture();

        assert_eq!(catalog.len(), 4);
        assert!(!catalog.is_empty());
        assert_eq!(catalog.category_label(0x6f), Some("All Databases"));
        assert_eq!(catalog.category_label(0x75), Some("Personnel Database"));
        assert_eq!(catalog.entries_for(0x6f).len(), 4);
        assert_eq!(catalog.entries_for(0x70)[0].name, "Allyuen");
        assert_eq!(catalog.entries_for(0x71)[0].name, "Alliance Dreadnaught");
        assert_eq!(catalog.entries_for(0x74)[0].name, "Alliance Army Regiment");
        assert_eq!(catalog.entries_for(0x75)[0].name, "Ackbar");
    }

    #[test]
    fn original_catalog_selection_preserves_identity_or_resets_to_first_match() {
        let catalog = original_catalog_fixture();
        let mut state = EncyclopediaState {
            original_selected_object_id: Some(0x1400_0001),
            ..EncyclopediaState::default()
        };

        reconcile_original_index_state(&mut state, &catalog, false);
        assert_eq!(state.original_selected_object_id, Some(0x1400_0001));

        state.original_category_command = 0x71;
        reconcile_original_index_state(&mut state, &catalog, true);
        assert_eq!(state.original_selected_object_id, Some(0x1400_0001));

        state.original_category_command = 0x70;
        reconcile_original_index_state(&mut state, &catalog, true);
        assert_eq!(state.original_selected_object_id, Some(0x9000_0001));
        assert_eq!(state.original_scroll_row, 0);
    }

    #[test]
    fn original_category_ranges_preserve_every_exclusive_boundary() {
        assert!(original_category_contains(0x6f, 0));
        for (command, start, end) in [
            (0x70, 0x90, 0x98),
            (0x71, 0x14, 0x20),
            (0x72, 0x20, 0x30),
            (0x73, 0x40, 0x80),
            (0x74, 0x10, 0x14),
            (0x75, 0x30, 0x40),
        ] {
            assert!(!original_category_contains(command, start - 1));
            assert!(original_category_contains(command, start));
            assert!(original_category_contains(command, end - 1));
            assert!(!original_category_contains(command, end));
        }
        assert!(!original_category_contains(0x76, 0x30));
    }

    #[test]
    fn original_catalog_reveal_and_clamp_keep_selection_visible() {
        let entries = (0..12)
            .map(|index| OriginalEncyclopediaEntry {
                object_id: 0x9000_0000 | index,
                name: format!("System {index:02}"),
            })
            .collect();
        let catalog = OriginalEncyclopediaCatalog::new(
            "Galactic Encyclopedia".into(),
            "Topic".into(),
            std::array::from_fn(|index| format!("Category {index}")),
            entries,
        );
        let mut state = EncyclopediaState {
            original_category_command: 0x70,
            original_selected_object_id: Some(0x9000_000b),
            original_scroll_row: 0,
            ..EncyclopediaState::default()
        };

        reconcile_original_index_state(&mut state, &catalog, true);
        assert_eq!(state.original_scroll_row, 3);

        state.original_selected_object_id = Some(0x9000_0000);
        reconcile_original_index_state(&mut state, &catalog, true);
        assert_eq!(state.original_scroll_row, 0);

        state.original_scroll_row = usize::MAX;
        reconcile_original_index_state(&mut state, &catalog, false);
        assert_eq!(state.original_scroll_row, 3);

        state.original_category_command = 0xffff;
        state.original_selected_object_id = Some(0xffff_ffff);
        reconcile_original_index_state(&mut state, &catalog, false);
        assert_eq!(state.original_category_command, 0x6f);
        assert_eq!(state.original_selected_object_id, Some(0x9000_0000));
        assert_eq!(state.original_scroll_row, 0);
    }

    #[test]
    fn original_index_ninth_row_is_clipped_to_the_native_list_boundary() {
        let parent = egui::Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(470.0, 330.0));
        let list = original_index_list_rect(parent, 1.0);

        for row in 0..ORIGINAL_INDEX_VISIBLE_ROWS {
            let rect = original_index_row_rect(parent, 1.0, row);
            assert!(rect.min.y >= list.min.y);
            assert!(rect.max.y <= list.max.y);
            assert!(rect.height() > 0.0);
        }
        assert_eq!(original_index_row_rect(parent, 1.0, 8).height(), 16.0);
    }

    #[test]
    fn encyclopedia_hit_testing_excludes_right_and_bottom_edges() {
        let rect = egui::Rect::from_min_max(egui::pos2(22.0, 46.0), egui::pos2(58.0, 87.0));

        assert!(encyclopedia_rect_contains(rect, egui::pos2(22.0, 46.0)));
        assert!(encyclopedia_rect_contains(rect, egui::pos2(57.999, 86.999)));
        assert!(!encyclopedia_rect_contains(rect, egui::pos2(58.0, 46.0)));
        assert!(!encyclopedia_rect_contains(rect, egui::pos2(22.0, 87.0)));
    }

    #[test]
    fn bitmap_controls_choose_exact_normal_pressed_selected_and_disabled_resources() {
        let spec = EncyclopediaControlSpec {
            command_id: 1,
            x: 0,
            y: 0,
            width: 1,
            height: 1,
            normal_resource: 10,
            pressed_resource: 11,
            disabled_resource: Some(12),
        };

        assert_eq!(bitmap_control_resource(spec, true, false, false), 10);
        assert_eq!(bitmap_control_resource(spec, true, false, true), 11);
        assert_eq!(bitmap_control_resource(spec, true, true, false), 11);
        assert_eq!(bitmap_control_resource(spec, false, true, true), 12);

        let no_disabled = EncyclopediaControlSpec {
            disabled_resource: None,
            ..spec
        };
        assert_eq!(
            bitmap_control_resource(no_disabled, false, false, false),
            10
        );
    }

    #[test]
    fn topic_and_index_rail_selection_states_are_mutually_exclusive() {
        assert_eq!(
            rail_selection_states(EncyclopediaMode::Topic),
            (true, false)
        );
        assert_eq!(
            rail_selection_states(EncyclopediaMode::Index),
            (false, true)
        );
    }

    fn surface_view(selected_topic: Option<&str>, disabled_category: bool) -> EncyclopediaView {
        let topic_ids = ["alpha", "beta", "gamma"];
        let selected_index = selected_topic
            .and_then(|selected| topic_ids.iter().position(|topic_id| *topic_id == selected));
        EncyclopediaView {
            index_label: Some(Arc::from("All topics")),
            index_enabled: true,
            categories: (0..6)
                .map(|index| CategoryViewItem {
                    category_id: format!("category-{index}"),
                    command: format!("0x{:x}", 0x70 + index),
                    label: (!(disabled_category && index == 1))
                        .then(|| Arc::from(format!("Category {index}"))),
                    enabled: !(disabled_category && index == 1),
                })
                .collect(),
            topics: topic_ids
                .iter()
                .map(|topic_id| TopicViewItem {
                    topic_id: (*topic_id).to_owned(),
                    title: Arc::from(*topic_id),
                })
                .collect(),
            active_topic: selected_index.map(|index| ActiveTopicView {
                topic_id: topic_ids[index].to_owned(),
                title: Arc::from("Σynthetic <b>title</b>"),
                body: Arc::from("Literal <script>markup</script> stays text.\nUnicode: Σ/ς/σ."),
                image: None,
                stats: vec![StatRowView {
                    label: Arc::from("Source row"),
                    value: Arc::from("supplied value"),
                }],
            }),
            navigation: NavigationState {
                selected_category_id: None,
                selected_topic_id: selected_topic.map(str::to_owned),
                previous_topic_id: selected_index
                    .and_then(|index| index.checked_sub(1))
                    .map(|index| topic_ids[index].to_owned()),
                next_topic_id: selected_index
                    .and_then(|index| index.checked_add(1))
                    .and_then(|index| topic_ids.get(index))
                    .map(|topic_id| (*topic_id).to_owned()),
                world_epoch: 7,
            },
            diagnostics: Vec::new(),
        }
    }

    fn surface_navigation(
        topic_id: Option<&str>,
        mode: EncyclopediaMode,
    ) -> EncyclopediaNavigationState {
        EncyclopediaNavigationState::new(
            EncyclopediaSelection {
                category_id: None,
                topic_id: topic_id.map(str::to_owned),
            },
            mode,
        )
    }

    fn run_surface_frame(
        ctx: &egui::Context,
        raw_input: egui::RawInput,
        view: &EncyclopediaView,
        navigation: &mut EncyclopediaNavigationState,
        surface: &mut EncyclopediaSurfaceState,
        chrome: &mut BmpCache,
        textures: &mut EncyclopediaTextureCache<EguiEncyclopediaTextureBackend>,
        faction: CockpitFaction,
    ) -> (egui::FullOutput, EncyclopediaSurfaceFrame) {
        let mut frame = None;
        let output = ctx.run(raw_input, |ctx| {
            frame = Some(draw_encyclopedia_surface(
                ctx,
                view,
                navigation,
                surface,
                chrome,
                textures,
                faction,
                egui::Pos2::ZERO,
                1.0,
            ));
        });
        (
            output,
            frame.expect("surface should produce one frame result"),
        )
    }

    fn pointer_input(position: egui::Pos2, pressed: Option<bool>) -> egui::RawInput {
        let mut events = vec![egui::Event::PointerMoved(position)];
        if let Some(pressed) = pressed {
            events.push(egui::Event::PointerButton {
                pos: position,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::default(),
            });
        }
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            events,
            ..Default::default()
        }
    }

    fn key_input(keys: impl IntoIterator<Item = egui::Key>) -> egui::RawInput {
        egui::RawInput {
            events: keys
                .into_iter()
                .map(|key| egui::Event::Key {
                    key,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::default(),
                })
                .collect(),
            ..Default::default()
        }
    }

    fn click_surface(
        ctx: &egui::Context,
        position: egui::Pos2,
        view: &EncyclopediaView,
        navigation: &mut EncyclopediaNavigationState,
        surface: &mut EncyclopediaSurfaceState,
        chrome: &mut BmpCache,
        textures: &mut EncyclopediaTextureCache<EguiEncyclopediaTextureBackend>,
        faction: CockpitFaction,
    ) -> EncyclopediaSurfaceFrame {
        // egui areas use an initial sizing pass before their controls become
        // hit-testable. The second hover frame represents the settled surface.
        for pressed in [None, None, Some(true)] {
            let _ = run_surface_frame(
                ctx,
                pointer_input(position, pressed),
                view,
                navigation,
                surface,
                chrome,
                textures,
                faction,
            );
        }
        run_surface_frame(
            ctx,
            pointer_input(position, Some(false)),
            view,
            navigation,
            surface,
            chrome,
            textures,
            faction,
        )
        .1
    }

    // Source: encyclopedia-ui-contract.md, "Client composition" and
    // "Category controls" (`0x6f..=0x75`, fixed 49x41 rectangles).
    #[test]
    fn original_surface_mouse_controls_emit_stable_actions_for_both_factions() {
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let ctx = egui::Context::default();
            let view = surface_view(None, false);
            let mut navigation = surface_navigation(None, EncyclopediaMode::Index);
            let mut surface = EncyclopediaSurfaceState::default();
            let mut chrome = BmpCache::new();
            let mut textures =
                EncyclopediaTextureCache::new(EguiEncyclopediaTextureBackend::new(&ctx));

            let category = click_surface(
                &ctx,
                egui::pos2(88.0 + 24.0, 78.0 + 20.0),
                &view,
                &mut navigation,
                &mut surface,
                &mut chrome,
                &mut textures,
                faction,
            );
            assert_eq!(
                category.actions,
                vec![EncyclopediaAction::SelectCategory {
                    category_id: Some("category-0".to_owned()),
                    force: SelectionForce::Normal,
                }]
            );

            let close_center = match faction {
                CockpitFaction::Alliance => egui::pos2(423.0 + 16.0, 25.0 + 15.0),
                CockpitFaction::Empire => egui::pos2(426.0 + 22.0, 21.0 + 20.0),
            };
            let close = click_surface(
                &ctx,
                close_center,
                &view,
                &mut navigation,
                &mut surface,
                &mut chrome,
                &mut textures,
                faction,
            );
            assert_eq!(close.actions, vec![EncyclopediaAction::Close]);
        }
    }

    #[test]
    fn disabled_category_and_topic_endpoint_controls_cannot_activate() {
        let ctx = egui::Context::default();
        let disabled_view = surface_view(None, true);
        let mut navigation = surface_navigation(None, EncyclopediaMode::Index);
        let mut surface = EncyclopediaSurfaceState::default();
        let mut chrome = BmpCache::new();
        let mut textures = EncyclopediaTextureCache::new(EguiEncyclopediaTextureBackend::new(&ctx));

        let disabled = click_surface(
            &ctx,
            egui::pos2(140.0 + 24.0, 78.0 + 20.0),
            &disabled_view,
            &mut navigation,
            &mut surface,
            &mut chrome,
            &mut textures,
            CockpitFaction::Alliance,
        );
        assert!(disabled.actions.is_empty());

        let exact_right_edge = click_surface(
            &ctx,
            egui::pos2(137.0, 98.0),
            &disabled_view,
            &mut navigation,
            &mut surface,
            &mut chrome,
            &mut textures,
            CockpitFaction::Alliance,
        );
        assert!(exact_right_edge.actions.is_empty());

        let first_view = surface_view(Some("alpha"), false);
        let mut navigation = surface_navigation(Some("alpha"), EncyclopediaMode::Topic);
        let backward = click_surface(
            &ctx,
            egui::pos2(28.0 + 10.0, 14.0 + 8.0),
            &first_view,
            &mut navigation,
            &mut surface,
            &mut chrome,
            &mut textures,
            CockpitFaction::Alliance,
        );
        assert!(backward.actions.is_empty());
        assert_eq!(
            backward.input_attempts,
            vec![EncyclopediaSurfaceInputAttempt {
                control: EncyclopediaSurfaceInputControl::PreviousTopic,
                enabled: false,
            }]
        );

        let last_view = surface_view(Some("gamma"), false);
        let mut navigation = surface_navigation(Some("gamma"), EncyclopediaMode::Topic);
        let forward = click_surface(
            &ctx,
            egui::pos2(380.0 + 10.0, 14.0 + 8.0),
            &last_view,
            &mut navigation,
            &mut surface,
            &mut chrome,
            &mut textures,
            CockpitFaction::Alliance,
        );
        assert!(forward.actions.is_empty());
        assert_eq!(
            forward.input_attempts,
            vec![EncyclopediaSurfaceInputAttempt {
                control: EncyclopediaSurfaceInputControl::NextTopic,
                enabled: false,
            }]
        );
    }

    // Source: encyclopedia-ui-contract.md, "Mode, focus, keyboard, and scrolling".
    #[test]
    fn topic_keyboard_events_preserve_input_order_without_rederiving_endpoints() {
        let ctx = egui::Context::default();
        let view = surface_view(Some("beta"), false);
        let mut navigation = surface_navigation(Some("beta"), EncyclopediaMode::Topic);
        let mut surface = EncyclopediaSurfaceState::default();
        let mut chrome = BmpCache::new();
        let mut textures = EncyclopediaTextureCache::new(EguiEncyclopediaTextureBackend::new(&ctx));
        let events = [
            egui::Key::ArrowLeft,
            egui::Key::ArrowRight,
            egui::Key::ArrowUp,
            egui::Key::ArrowDown,
            egui::Key::ArrowDown,
            egui::Key::PageUp,
            egui::Key::PageDown,
            egui::Key::Enter,
            egui::Key::Tab,
            egui::Key::Escape,
        ]
        .into_iter()
        .map(|key| egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        })
        .collect();

        let frame = run_surface_frame(
            &ctx,
            egui::RawInput {
                events,
                ..Default::default()
            },
            &view,
            &mut navigation,
            &mut surface,
            &mut chrome,
            &mut textures,
            CockpitFaction::Empire,
        )
        .1;

        assert_eq!(
            frame.actions,
            vec![
                EncyclopediaAction::SourceKey(SourceKeyIntent::Left),
                EncyclopediaAction::SourceKey(SourceKeyIntent::Right),
                EncyclopediaAction::SourceKey(SourceKeyIntent::Up),
                EncyclopediaAction::SourceKey(SourceKeyIntent::Down),
                EncyclopediaAction::SourceKey(SourceKeyIntent::Down),
                EncyclopediaAction::SourceKey(SourceKeyIntent::PageUp { visible_rows: 0 }),
                EncyclopediaAction::SourceKey(SourceKeyIntent::PageDown { visible_rows: 0 }),
                EncyclopediaAction::SourceKey(SourceKeyIntent::Enter),
                EncyclopediaAction::SourceKey(SourceKeyIntent::Escape),
            ]
        );
    }

    #[test]
    fn index_enter_and_escape_translate_without_inventing_directional_selection() {
        let ctx = egui::Context::default();
        let view = surface_view(Some("beta"), false);
        let mut navigation = surface_navigation(Some("beta"), EncyclopediaMode::Index);
        let mut surface = EncyclopediaSurfaceState::default();
        let mut chrome = BmpCache::new();
        let mut textures = EncyclopediaTextureCache::new(EguiEncyclopediaTextureBackend::new(&ctx));
        let events = [egui::Key::Enter, egui::Key::Tab, egui::Key::Escape]
            .into_iter()
            .map(|key| egui::Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::default(),
            })
            .collect();

        let frame = run_surface_frame(
            &ctx,
            egui::RawInput {
                events,
                ..Default::default()
            },
            &view,
            &mut navigation,
            &mut surface,
            &mut chrome,
            &mut textures,
            CockpitFaction::Alliance,
        )
        .1;

        assert_eq!(
            frame.actions,
            vec![
                EncyclopediaAction::SourceKey(SourceKeyIntent::Enter),
                EncyclopediaAction::SourceKey(SourceKeyIntent::Escape),
            ]
        );
    }

    #[test]
    fn surface_drains_the_ordered_scroll_batch_once_before_drawing() {
        let ctx = egui::Context::default();
        let mut view = surface_view(Some("beta"), false);
        view.active_topic.as_mut().unwrap().body = Arc::from("long body line\n".repeat(128));
        let mut navigation = surface_navigation(Some("beta"), EncyclopediaMode::Topic);
        assert_eq!(
            apply_encyclopedia_action(&mut navigation, &view, EncyclopediaAction::NextTopic,),
            NavigationOutcome::Applied
        );
        assert_eq!(
            apply_encyclopedia_action(
                &mut navigation,
                &view,
                EncyclopediaAction::Scroll(BodyScrollIntent::LineDown),
            ),
            NavigationOutcome::ScrollRequested(BodyScrollIntent::LineDown)
        );
        let mut surface = EncyclopediaSurfaceState::default();
        let mut chrome = BmpCache::new();
        let mut textures = EncyclopediaTextureCache::new(EguiEncyclopediaTextureBackend::new(&ctx));

        let first = run_surface_frame(
            &ctx,
            egui::RawInput::default(),
            &view,
            &mut navigation,
            &mut surface,
            &mut chrome,
            &mut textures,
            CockpitFaction::Alliance,
        )
        .1;
        assert_eq!(
            first.consumed_scroll_intents,
            vec![BodyScrollIntent::ResetToTop, BodyScrollIntent::LineDown]
        );
        assert!(navigation.pending_body_scroll_intents().is_empty());
        assert!(surface.body_scroll_offset() > 0.0);

        let second = run_surface_frame(
            &ctx,
            egui::RawInput::default(),
            &view,
            &mut navigation,
            &mut surface,
            &mut chrome,
            &mut textures,
            CockpitFaction::Alliance,
        )
        .1;
        assert!(second.consumed_scroll_intents.is_empty());
    }

    #[test]
    fn line_and_page_scroll_arithmetic_moves_in_both_directions_and_clamps() {
        let mut state = EncyclopediaSurfaceState {
            body_scroll_offset: 100.0,
            ..Default::default()
        };
        apply_body_scroll_intents(&mut state, &[BodyScrollIntent::LineUp], 10.0, 40.0);
        assert_eq!(state.body_scroll_offset(), 90.0);

        apply_body_scroll_intents(&mut state, &[BodyScrollIntent::PageUp], 10.0, 40.0);
        assert_eq!(state.body_scroll_offset(), 50.0);

        apply_body_scroll_intents(&mut state, &[BodyScrollIntent::PageDown], 10.0, 40.0);
        assert_eq!(state.body_scroll_offset(), 90.0);

        state.body_scroll_offset = 5.0;
        apply_body_scroll_intents(&mut state, &[BodyScrollIntent::LineUp], 10.0, 40.0);
        assert_eq!(state.body_scroll_offset(), 0.0);

        state.body_scroll_offset = 20.0;
        apply_body_scroll_intents(&mut state, &[BodyScrollIntent::PageUp], 10.0, 40.0);
        assert_eq!(state.body_scroll_offset(), 0.0);
    }

    #[test]
    fn invalid_scales_fail_without_consuming_navigation_intents_or_allocating_art() {
        for scale in [0.0, -1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let ctx = egui::Context::default();
            let view = surface_view(Some("beta"), false);
            let mut navigation = surface_navigation(Some("beta"), EncyclopediaMode::Topic);
            assert_eq!(
                apply_encyclopedia_action(
                    &mut navigation,
                    &view,
                    EncyclopediaAction::Scroll(BodyScrollIntent::LineDown),
                ),
                NavigationOutcome::ScrollRequested(BodyScrollIntent::LineDown)
            );
            let mut surface = EncyclopediaSurfaceState::default();
            let mut chrome = BmpCache::new();
            let mut textures =
                EncyclopediaTextureCache::new(EguiEncyclopediaTextureBackend::new(&ctx));
            let mut frame = None;

            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                frame = Some(draw_encyclopedia_surface(
                    ctx,
                    &view,
                    &mut navigation,
                    &mut surface,
                    &mut chrome,
                    &mut textures,
                    CockpitFaction::Alliance,
                    egui::Pos2::ZERO,
                    scale,
                ));
            });

            assert_eq!(frame.unwrap(), EncyclopediaSurfaceFrame::empty());
            assert_eq!(
                navigation.pending_body_scroll_intents(),
                &[BodyScrollIntent::LineDown]
            );
        }
    }

    #[test]
    fn selected_art_is_resolved_only_in_topic_mode_and_released_in_index_mode() {
        let ctx = egui::Context::default();
        let mut view = surface_view(Some("beta"), false);
        view.active_topic.as_mut().unwrap().image = Some(TopicImageView {
            asset_id: "synthetic-art".to_owned(),
            digest: "7c7c81733a86716fed19d36619f7accda8e8175aadb1ac60ef2ac390f75a401a".to_owned(),
            format: "bmp".to_owned(),
            width: 400,
            height: 200,
            bytes: Arc::from(synthetic_indexed_edata()),
            render_profile: TopicImageRenderProfile::OriginalNearest,
        });
        let mut navigation = surface_navigation(Some("beta"), EncyclopediaMode::Topic);
        let mut surface = EncyclopediaSurfaceState::default();
        let mut chrome = BmpCache::new();
        let mut textures = EncyclopediaTextureCache::new(EguiEncyclopediaTextureBackend::new(&ctx));

        let topic = run_surface_frame(
            &ctx,
            egui::RawInput::default(),
            &view,
            &mut navigation,
            &mut surface,
            &mut chrome,
            &mut textures,
            CockpitFaction::Alliance,
        )
        .1;
        assert_eq!(topic.active_asset_id.as_deref(), Some("synthetic-art"));
        assert_eq!(
            topic.active_digest.as_deref(),
            Some("7c7c81733a86716fed19d36619f7accda8e8175aadb1ac60ef2ac390f75a401a")
        );
        assert!(
            matches!(
                topic.texture_events.as_slice(),
                [EncyclopediaTextureEvent::Selected {
                    cache_hit: false,
                    ..
                }]
            ),
            "events={:?} diagnostic={:?}",
            topic.texture_events,
            topic.texture_diagnostic
        );

        let mut index_navigation = surface_navigation(Some("beta"), EncyclopediaMode::Index);
        let index = run_surface_frame(
            &ctx,
            egui::RawInput::default(),
            &view,
            &mut index_navigation,
            &mut surface,
            &mut chrome,
            &mut textures,
            CockpitFaction::Alliance,
        )
        .1;
        assert_eq!(index.active_asset_id, None);
        assert_eq!(index.active_digest, None);
        assert!(matches!(
            index.texture_events.as_slice(),
            [EncyclopediaTextureEvent::Released { .. }]
        ));
    }

    #[test]
    fn literal_unicode_markup_and_supplied_stat_rows_reach_egui_unchanged() {
        fn collect_text(shape: &egui::epaint::Shape, output: &mut String) {
            match shape {
                egui::epaint::Shape::Text(text) => output.push_str(&text.galley.job.text),
                egui::epaint::Shape::Vec(shapes) => {
                    for shape in shapes {
                        collect_text(shape, output);
                    }
                }
                _ => {}
            }
        }

        let ctx = egui::Context::default();
        let view = surface_view(Some("beta"), false);
        let mut navigation = surface_navigation(Some("beta"), EncyclopediaMode::Topic);
        let mut surface = EncyclopediaSurfaceState::default();
        let mut chrome = BmpCache::new();
        let mut textures = EncyclopediaTextureCache::new(EguiEncyclopediaTextureBackend::new(&ctx));

        let _ = run_surface_frame(
            &ctx,
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(640.0, 480.0),
                )),
                ..Default::default()
            },
            &view,
            &mut navigation,
            &mut surface,
            &mut chrome,
            &mut textures,
            CockpitFaction::Alliance,
        );
        let (output, frame) = run_surface_frame(
            &ctx,
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(640.0, 480.0),
                )),
                ..Default::default()
            },
            &view,
            &mut navigation,
            &mut surface,
            &mut chrome,
            &mut textures,
            CockpitFaction::Alliance,
        );
        assert_eq!(frame.active_asset_id, None);
        let mut text = String::new();
        for clipped in &output.shapes {
            collect_text(&clipped.shape, &mut text);
        }
        assert!(text.contains("Σynthetic <b>title</b>"), "text={text:?}");
        assert!(text.contains("Literal <script>markup</script> stays text."));
        assert!(text.contains("Unicode: Σ/ς/σ."));
        assert!(text.contains("Source row"));
        assert!(text.contains("supplied value"));
    }

    #[test]
    fn scaled_surface_hit_regions_follow_the_supplied_origin_and_scale() {
        let ctx = egui::Context::default();
        let view = surface_view(None, false);
        let mut navigation = surface_navigation(None, EncyclopediaMode::Index);
        let mut surface = EncyclopediaSurfaceState::default();
        let mut chrome = BmpCache::new();
        let mut textures = EncyclopediaTextureCache::new(EguiEncyclopediaTextureBackend::new(&ctx));
        let origin = egui::pos2(23.0, 31.0);
        let scale = 0.75;
        let position = egui::pos2(
            origin.x + (192.0 + 24.0) * scale,
            origin.y + (78.0 + 20.0) * scale,
        );
        let mut frame = None;

        for pressed in [None, None, Some(true), Some(false)] {
            let _ = ctx.run(pointer_input(position, pressed), |ctx| {
                frame = Some(draw_encyclopedia_surface(
                    ctx,
                    &view,
                    &mut navigation,
                    &mut surface,
                    &mut chrome,
                    &mut textures,
                    CockpitFaction::Empire,
                    origin,
                    scale,
                ));
            });
        }

        assert_eq!(
            frame.unwrap().actions,
            vec![EncyclopediaAction::SelectCategory {
                category_id: Some("category-2".to_owned()),
                force: SelectionForce::Normal,
            }]
        );
    }

    // Source: encyclopedia-ui-contract.md lines 203-258. Keyboard routing is
    // owned by the focused source child and Tab is consumed locally.
    #[test]
    fn surface_routes_and_consumes_keys_only_for_transition_focused_children() {
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let ctx = egui::Context::default();
            let view = surface_view(Some("beta"), false);
            let mut navigation = surface_navigation(Some("beta"), EncyclopediaMode::Index);
            let mut surface = EncyclopediaSurfaceState::default();
            let mut chrome = BmpCache::new();
            let mut textures =
                EncyclopediaTextureCache::new(EguiEncyclopediaTextureBackend::new(&ctx));

            let initial = run_surface_frame(
                &ctx,
                egui::RawInput::default(),
                &view,
                &mut navigation,
                &mut surface,
                &mut chrome,
                &mut textures,
                faction,
            )
            .1;
            let index_focus = ctx
                .memory(|memory| memory.focused())
                .expect("initial index transition should focus the list child");
            assert_eq!(
                initial.keyboard_focus,
                Some(EncyclopediaSurfaceFocus {
                    target_kind: EncyclopediaSurfaceFocusKind::IndexList,
                    target_id: index_focus.value(),
                    focused_id: Some(index_focus.value()),
                    owns_focus: true,
                })
            );

            let unrelated = egui::Id::new((
                "unrelated-widget",
                matches!(faction, CockpitFaction::Empire),
            ));
            let mut frame = None;
            let mut right_remained = false;
            let _ = ctx.run(key_input([egui::Key::ArrowRight]), |ctx| {
                ctx.memory_mut(|memory| memory.request_focus(unrelated));
                frame = Some(draw_encyclopedia_surface(
                    ctx,
                    &view,
                    &mut navigation,
                    &mut surface,
                    &mut chrome,
                    &mut textures,
                    faction,
                    egui::Pos2::ZERO,
                    1.0,
                ));
                right_remained = ctx.input(|input| input.key_pressed(egui::Key::ArrowRight));
            });
            let unfocused = frame.unwrap();
            assert!(unfocused.actions.is_empty());
            assert_eq!(
                unfocused.keyboard_focus,
                Some(EncyclopediaSurfaceFocus {
                    target_kind: EncyclopediaSurfaceFocusKind::IndexList,
                    target_id: index_focus.value(),
                    focused_id: Some(unrelated.value()),
                    owns_focus: false,
                })
            );
            assert!(
                right_remained,
                "unfocused surface must not consume ArrowRight"
            );

            assert_eq!(
                apply_encyclopedia_action(
                    &mut navigation,
                    &view,
                    EncyclopediaAction::SelectCategory {
                        category_id: Some("category-0".to_owned()),
                        force: SelectionForce::Normal,
                    },
                ),
                NavigationOutcome::Applied
            );
            let mut down_remained = true;
            let mut frame = None;
            let _ = ctx.run(key_input([egui::Key::ArrowDown]), |ctx| {
                frame = Some(draw_encyclopedia_surface(
                    ctx,
                    &view,
                    &mut navigation,
                    &mut surface,
                    &mut chrome,
                    &mut textures,
                    faction,
                    egui::Pos2::ZERO,
                    1.0,
                ));
                down_remained = ctx.input(|input| input.key_pressed(egui::Key::ArrowDown));
            });
            assert_eq!(
                frame.unwrap().actions,
                vec![EncyclopediaAction::SourceKey(SourceKeyIntent::Down)]
            );
            assert!(!down_remained, "focused list must consume ArrowDown");

            let mut tab_remained = true;
            let mut frame = None;
            let _ = ctx.run(key_input([egui::Key::Tab]), |ctx| {
                frame = Some(draw_encyclopedia_surface(
                    ctx,
                    &view,
                    &mut navigation,
                    &mut surface,
                    &mut chrome,
                    &mut textures,
                    faction,
                    egui::Pos2::ZERO,
                    1.0,
                ));
                tab_remained = ctx.input(|input| input.key_pressed(egui::Key::Tab));
            });
            assert!(frame.unwrap().actions.is_empty());
            assert!(
                !tab_remained,
                "focused source child must consume Tab locally"
            );
            assert_eq!(
                ctx.memory(|memory| memory.focused()),
                Some(index_focus),
                "local Tab handling must restore the index list focus"
            );

            let mut frame = None;
            let mut left_remained = false;
            let _ = ctx.run(key_input([egui::Key::ArrowLeft]), |ctx| {
                ctx.memory_mut(|memory| memory.request_focus(unrelated));
                frame = Some(draw_encyclopedia_surface(
                    ctx,
                    &view,
                    &mut navigation,
                    &mut surface,
                    &mut chrome,
                    &mut textures,
                    faction,
                    egui::Pos2::ZERO,
                    1.0,
                ));
                left_remained = ctx.input(|input| input.key_pressed(egui::Key::ArrowLeft));
            });
            assert!(frame.unwrap().actions.is_empty());
            assert!(
                left_remained,
                "a prior locally consumed key must not reclaim unrelated focus"
            );
            assert_eq!(ctx.memory(|memory| memory.focused()), Some(unrelated));

            assert_eq!(
                apply_encyclopedia_action(
                    &mut navigation,
                    &view,
                    EncyclopediaAction::SetMode(EncyclopediaMode::Topic),
                ),
                NavigationOutcome::Applied
            );
            let topic = run_surface_frame(
                &ctx,
                key_input([egui::Key::ArrowRight]),
                &view,
                &mut navigation,
                &mut surface,
                &mut chrome,
                &mut textures,
                faction,
            )
            .1;
            assert_eq!(
                topic.actions,
                vec![EncyclopediaAction::SourceKey(SourceKeyIntent::Right)]
            );
            assert_eq!(
                topic.keyboard_focus.map(|focus| focus.target_kind),
                Some(EncyclopediaSurfaceFocusKind::TopicBody)
            );
            assert!(topic.keyboard_focus.unwrap().owns_focus);
        }
    }

    #[test]
    fn focused_index_keys_emit_typed_category_and_layout_sized_row_intents() {
        let ctx = egui::Context::default();
        let view = surface_view(Some("beta"), false);
        let mut navigation = surface_navigation(Some("beta"), EncyclopediaMode::Index);
        let mut surface = EncyclopediaSurfaceState::default();
        let mut chrome = BmpCache::new();
        let mut textures = EncyclopediaTextureCache::new(EguiEncyclopediaTextureBackend::new(&ctx));

        let frame = run_surface_frame(
            &ctx,
            key_input([
                egui::Key::ArrowLeft,
                egui::Key::ArrowRight,
                egui::Key::ArrowUp,
                egui::Key::ArrowDown,
                egui::Key::PageUp,
                egui::Key::PageDown,
                egui::Key::Home,
                egui::Key::End,
            ]),
            &view,
            &mut navigation,
            &mut surface,
            &mut chrome,
            &mut textures,
            CockpitFaction::Alliance,
        )
        .1;

        assert_eq!(
            frame.actions,
            vec![
                EncyclopediaAction::SourceKey(SourceKeyIntent::Left),
                EncyclopediaAction::SourceKey(SourceKeyIntent::Right),
                EncyclopediaAction::SourceKey(SourceKeyIntent::Up),
                EncyclopediaAction::SourceKey(SourceKeyIntent::Down),
                EncyclopediaAction::SourceKey(SourceKeyIntent::PageUp { visible_rows: 8 }),
                EncyclopediaAction::SourceKey(SourceKeyIntent::PageDown { visible_rows: 8 }),
                EncyclopediaAction::SourceKey(SourceKeyIntent::Home),
                EncyclopediaAction::SourceKey(SourceKeyIntent::End),
            ]
        );
    }

    #[test]
    fn index_source_labels_render_at_proven_origins_and_missing_labels_are_diagnostic() {
        fn text_shapes(output: &egui::FullOutput) -> Vec<(String, egui::Pos2)> {
            fn collect(shape: &egui::epaint::Shape, text: &mut Vec<(String, egui::Pos2)>) {
                match shape {
                    egui::epaint::Shape::Text(shape) => {
                        text.push((shape.galley.job.text.clone(), shape.pos));
                    }
                    egui::epaint::Shape::Vec(shapes) => {
                        for shape in shapes {
                            collect(shape, text);
                        }
                    }
                    _ => {}
                }
            }

            let mut text = Vec::new();
            for clipped in &output.shapes {
                collect(&clipped.shape, &mut text);
            }
            text
        }

        let ctx = egui::Context::default();
        let view = surface_view(None, false);
        let mut navigation = surface_navigation(None, EncyclopediaMode::Index);
        let mut surface = EncyclopediaSurfaceState::default();
        surface.set_source_labels(EncyclopediaSurfaceLabels {
            index_header: Some(Arc::from("Synthetic source header")),
            index_static: Some(Arc::from("Synthetic source static text")),
        });
        let mut chrome = BmpCache::new();
        let mut textures = EncyclopediaTextureCache::new(EguiEncyclopediaTextureBackend::new(&ctx));
        let _ = run_surface_frame(
            &ctx,
            egui::RawInput::default(),
            &view,
            &mut navigation,
            &mut surface,
            &mut chrome,
            &mut textures,
            CockpitFaction::Alliance,
        );
        let (output, frame) = run_surface_frame(
            &ctx,
            egui::RawInput::default(),
            &view,
            &mut navigation,
            &mut surface,
            &mut chrome,
            &mut textures,
            CockpitFaction::Alliance,
        );
        assert!(frame.surface_diagnostics.is_empty());
        let text = text_shapes(&output);
        assert!(
            text.iter().any(|(value, pos)| {
                value == "Synthetic source header" && *pos == egui::pos2(36.0, 14.0)
            }),
            "text shapes: {text:?}"
        );
        assert!(
            text.iter().any(|(value, pos)| {
                value == "Synthetic source static text" && *pos == egui::pos2(36.0, 48.0)
            }),
            "text shapes: {text:?}"
        );

        let ctx = egui::Context::default();
        let mut navigation = surface_navigation(None, EncyclopediaMode::Index);
        let mut surface = EncyclopediaSurfaceState::default();
        let mut chrome = BmpCache::new();
        let mut textures = EncyclopediaTextureCache::new(EguiEncyclopediaTextureBackend::new(&ctx));
        let (_, frame) = run_surface_frame(
            &ctx,
            egui::RawInput::default(),
            &view,
            &mut navigation,
            &mut surface,
            &mut chrome,
            &mut textures,
            CockpitFaction::Empire,
        );
        assert_eq!(
            frame.surface_diagnostics,
            vec![
                EncyclopediaSurfaceDiagnostic::MissingLocalizedResourceText {
                    resource_id: 0x1842,
                },
                EncyclopediaSurfaceDiagnostic::MissingLocalizedResourceText {
                    resource_id: 0x1843,
                },
            ]
        );
    }

    #[test]
    fn native_shell_geometry_clips_row_331_without_resampling_the_texture() {
        let client = egui::Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(470.0, 330.0));
        let geometry = native_bitmap_geometry(client.min, egui::vec2(470.0, 331.0), 1.0, client);

        assert_eq!(geometry.destination.size(), egui::vec2(470.0, 331.0));
        assert_eq!(
            geometry.uv,
            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0))
        );
        assert_eq!(geometry.clip_rect, client);
        assert_eq!(
            geometry.destination.intersect(geometry.clip_rect).size(),
            egui::vec2(470.0, 330.0)
        );
        assert_eq!(geometry.destination.max.y, client.max.y + 1.0);

        let scaled = native_bitmap_geometry(client.min, egui::vec2(470.0, 331.0), 0.75, client);
        assert_eq!(scaled.destination.size(), egui::vec2(352.5, 248.25));
        assert_eq!(scaled.uv, geometry.uv);
    }
}
