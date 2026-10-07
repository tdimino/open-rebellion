//! The Agent menu (`ghidra/notes/agent-menu.md`): STRATEGY `RT_RCDATA` list
//! `0xdead`, items `0x110..0x11e`. A right click on the advisor panel opens
//! it (`FUN_00422ce0` → `FUN_004420b0` → `FUN_0042d050`), and
//! `FUN_00487900` enables and checks its items before it shows.

use egui_macroquad::egui;

use crate::bmp_cache::BmpCache;
use crate::cockpit::{CockpitFaction, CockpitLayout};
use crate::game_menu::{draw_game_menu, GameMenuEntry, GameMenuPlacement, GameMenuResponse};

const AGENT_MENU_ID: &str = "original-agent-menu";

/// The bitmap each record names in word 10, shown for a checked item.
/// hyp: word 10 is the checked item's bitmap, as the Command submenu's
/// ranks carry it (`object-popup-menu.md`).
const CHECK_BITMAP: u32 = 11902;

/// An Agent menu command the port carries out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentCommand {
    /// 0x115, Alt+G: toggle module 0x15 (`FUN_00439d60`).
    ManageGarrisons,
    /// 0x116, Alt+U: toggle module 0x14 (`FUN_00439d60`).
    ManageProduction,
    /// 0x11e, Alt+A: flip bit `0x8000` (`FUN_00439e80`).
    AgentAdvice,
}

/// One record of list `0xdead`: its command, sort key (word 2) and TEXTSTRA
/// label (word 5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgentMenuRecord {
    pub command: u16,
    pub sort_key: u16,
    pub label: &'static str,
}

/// STRATEGY `RT_RCDATA` `0x110..0x11e` (no `0x11d` in the list), in sort
/// order; labels are TEXTSTRA `0x3000 + command`.
pub const AGENT_MENU_RECORDS: [AgentMenuRecord; 14] = [
    AgentMenuRecord {
        command: 0x110,
        sort_key: 100,
        label: "Build Ships",
    },
    AgentMenuRecord {
        command: 0x111,
        sort_key: 110,
        label: "Build Troops",
    },
    AgentMenuRecord {
        command: 0x112,
        sort_key: 120,
        label: "Build Facilities",
    },
    AgentMenuRecord {
        command: 0x113,
        sort_key: 200,
        label: "Galaxy Overview",
    },
    AgentMenuRecord {
        command: 0x114,
        sort_key: 500,
        label: "Objectives",
    },
    AgentMenuRecord {
        command: 0x115,
        sort_key: 510,
        label: "Manage Garrisons",
    },
    AgentMenuRecord {
        command: 0x116,
        sort_key: 520,
        label: "Manage Production",
    },
    AgentMenuRecord {
        command: 0x117,
        sort_key: 530,
        label: "Manage Maintenance",
    },
    AgentMenuRecord {
        command: 0x118,
        sort_key: 540,
        label: "Not An Operation",
    },
    AgentMenuRecord {
        command: 0x119,
        sort_key: 600,
        label: "Deactivate",
    },
    AgentMenuRecord {
        command: 0x11a,
        sort_key: 601,
        label: "Reactivate",
    },
    AgentMenuRecord {
        command: 0x11b,
        sort_key: 610,
        label: "Translate Counterpart",
    },
    AgentMenuRecord {
        command: 0x11e,
        sort_key: 615,
        label: "Agent Advice",
    },
    AgentMenuRecord {
        command: 0x11c,
        sort_key: 620,
        label: "Messages",
    },
];

/// The agent state the menu shows.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AgentMenuView {
    pub garrisons: bool,
    pub production: bool,
    pub advice: bool,
}

/// One item as `FUN_00487900` leaves it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgentMenuItem {
    pub record: AgentMenuRecord,
    pub enabled: bool,
    pub checked: bool,
    pub command: Option<AgentCommand>,
}

/// The items `FUN_00487900` enables and checks. Manage Garrisons and Manage
/// Production are checked while they run (`FUN_00439e30`), Agent Advice
/// while bit `0x8000` is clear. port: the Build orders (module 0x17), the
/// encyclopedia views, Translate Counterpart and Messages are not ported, so
/// they stay disabled, as are the items `FUN_00487900` leaves alone.
#[must_use]
pub fn agent_menu_items(view: AgentMenuView) -> Vec<AgentMenuItem> {
    AGENT_MENU_RECORDS
        .iter()
        .map(|&record| {
            let (command, checked) = match record.command {
                0x115 => (Some(AgentCommand::ManageGarrisons), view.garrisons),
                0x116 => (Some(AgentCommand::ManageProduction), view.production),
                0x11e => (Some(AgentCommand::AgentAdvice), view.advice),
                _ => (None, false),
            };
            AgentMenuItem {
                record,
                enabled: command.is_some(),
                checked,
                command,
            }
        })
        .collect()
}

/// Whether a right click at canvas `point` lands on the agent's panel.
/// hyp: the agent is the larger droid aperture (`FUN_0042adb0`, panel
/// `param_1[0x4a]`); the other panel's list is `0xbeef`, the Message Alerts.
#[must_use]
pub fn agent_panel_contains(faction: CockpitFaction, point: (f32, f32)) -> bool {
    let (x, y, width, height) = crate::advisor::agent_aperture(faction);
    point.0 >= x && point.0 < x + width && point.1 >= y && point.1 < y + height
}

/// The open menu's anchor, a 640 by 480 canvas point.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct AgentMenuState {
    pub anchor: Option<(f32, f32)>,
}

/// Open the menu on a right-button release over the agent's panel
/// (`FUN_00422ce0` `WM_RBUTTONUP` → `FUN_004420b0`), anchored at the
/// release. A release over a window or menu does not reach the panel.
pub fn open_agent_menu_on_right_click(
    ctx: &egui::Context,
    state: &mut AgentMenuState,
    layout: CockpitLayout,
    faction: CockpitFaction,
) -> bool {
    if layout.scale <= 0.0 || ctx.is_pointer_over_area() {
        return false;
    }
    let (released, pointer) = ctx.input(|input| {
        (
            input
                .pointer
                .button_released(egui::PointerButton::Secondary),
            input.pointer.interact_pos(),
        )
    });
    let Some(pointer) = pointer.filter(|_| released) else {
        return false;
    };
    let point = (
        (pointer.x - layout.canvas.x) / layout.scale,
        (pointer.y - layout.canvas.y) / layout.scale,
    );
    if !agent_panel_contains(faction, point) {
        return false;
    }
    state.anchor = Some(point);
    true
}

/// Draw the open Agent menu and return the command chosen.
pub fn draw_agent_menu(
    ctx: &egui::Context,
    state: &mut AgentMenuState,
    cache: &mut BmpCache,
    layout: CockpitLayout,
    faction: CockpitFaction,
    view: AgentMenuView,
    input_enabled: bool,
) -> Option<AgentCommand> {
    let anchor = state.anchor?;
    let items = agent_menu_items(view);
    let entries: Vec<GameMenuEntry<'_>> = items
        .iter()
        .map(|item| GameMenuEntry {
            label: item.record.label,
            icon: item.checked.then_some(CHECK_BITMAP),
            enabled: item.enabled,
            submenu: false,
        })
        .collect();
    match draw_game_menu(
        ctx,
        egui::Id::new(AGENT_MENU_ID),
        cache,
        layout,
        faction,
        GameMenuPlacement::in_frame(anchor),
        &entries,
        input_enabled,
    ) {
        GameMenuResponse::Open => None,
        GameMenuResponse::Dismissed => {
            state.anchor = None;
            None
        }
        GameMenuResponse::Chosen(index) => {
            state.anchor = None;
            items[index].command
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_menu_lists_list_0xdeads_records_in_sort_order() {
        // STRATEGY RT_RCDATA 0x110..0x11e, word 2 the sort key, word 5 the
        // TEXTSTRA label; 0x11d (Message Alerts) is not in the list.
        let keys: Vec<u16> = AGENT_MENU_RECORDS
            .iter()
            .map(|record| record.sort_key)
            .collect();
        let mut sorted = keys.clone();
        sorted.sort_unstable();
        assert_eq!(keys, sorted);
        assert!(!AGENT_MENU_RECORDS
            .iter()
            .any(|record| record.command == 0x11d));
        assert_eq!(AGENT_MENU_RECORDS[12].label, "Agent Advice");
    }

    #[test]
    fn running_automations_and_advice_are_checked_and_enabled() {
        // FUN_00487900: 0x115/0x116 checked when FUN_00439e30 says the module
        // runs; 0x11e checked when bit 0x8000 is clear.
        let item = |items: &[AgentMenuItem], command| {
            *items
                .iter()
                .find(|item| item.record.command == command)
                .unwrap()
        };
        let off = agent_menu_items(AgentMenuView::default());
        for command in [0x115, 0x116, 0x11e] {
            assert!(item(&off, command).enabled && !item(&off, command).checked);
        }
        let on = agent_menu_items(AgentMenuView {
            garrisons: true,
            production: false,
            advice: true,
        });
        assert!(item(&on, 0x115).checked);
        assert!(!item(&on, 0x116).checked);
        assert!(item(&on, 0x11e).checked);
        assert_eq!(item(&on, 0x11e).command, Some(AgentCommand::AgentAdvice));
        assert!(!item(&on, 0x110).enabled, "port: Build Ships is not ported");
    }

    #[test]
    fn a_right_click_on_the_agents_droid_opens_the_menu() {
        // FUN_004420b0 hit-tests the panel at the click; the larger droid's
        // aperture (FUN_0042adb0).
        assert!(agent_panel_contains(
            CockpitFaction::Alliance,
            (560.0, 400.0)
        ));
        assert!(!agent_panel_contains(
            CockpitFaction::Alliance,
            (320.0, 420.0)
        ));
        assert!(agent_panel_contains(CockpitFaction::Empire, (50.0, 400.0)));
        // (541, 337, 67, 116): right and bottom edges are outside.
        assert!(agent_panel_contains(
            CockpitFaction::Alliance,
            (541.0, 337.0)
        ));
        assert!(!agent_panel_contains(
            CockpitFaction::Alliance,
            (608.0, 400.0)
        ));
        assert!(!agent_panel_contains(
            CockpitFaction::Alliance,
            (560.0, 453.0)
        ));
        assert!(!agent_panel_contains(
            CockpitFaction::Alliance,
            (540.0, 400.0)
        ));
    }

    /// A right click's release at `screen` on a canvas at (10, 20), scale 2.
    fn right_click(state: &mut AgentMenuState, screen: egui::Pos2, scale: f32) -> bool {
        use crate::cockpit::CockpitViewport;
        let layout = CockpitLayout {
            canvas: CockpitViewport {
                x: 10.0,
                y: 20.0,
                width: 640.0 * scale,
                height: 480.0 * scale,
            },
            galaxy: CockpitViewport {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
            },
            scale,
        };
        let ctx = egui::Context::default();
        let mut opened = false;
        for pressed in [true, false] {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1400.0, 1000.0),
                )),
                events: vec![
                    egui::Event::PointerMoved(screen),
                    egui::Event::PointerButton {
                        pos: screen,
                        button: egui::PointerButton::Secondary,
                        pressed,
                        modifiers: egui::Modifiers::NONE,
                    },
                ],
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                opened |=
                    open_agent_menu_on_right_click(ctx, state, layout, CockpitFaction::Alliance);
            });
        }
        opened
    }

    #[test]
    fn a_right_button_release_on_the_droid_anchors_the_menu_at_its_canvas_point() {
        // FUN_00422ce0 WM_RBUTTONUP -> FUN_004420b0 at the release point.
        let mut state = AgentMenuState::default();
        assert!(right_click(
            &mut state,
            egui::pos2(10.0 + 560.0 * 2.0, 20.0 + 400.0 * 2.0),
            2.0
        ));
        assert_eq!(state.anchor, Some((560.0, 400.0)));

        let mut state = AgentMenuState::default();
        assert!(!right_click(
            &mut state,
            egui::pos2(10.0 + 320.0 * 2.0, 20.0 + 420.0 * 2.0),
            2.0
        ));
        assert_eq!(state.anchor, None);

        let mut state = AgentMenuState::default();
        assert!(!right_click(&mut state, egui::pos2(570.0, 420.0), 0.0));
        assert_eq!(state.anchor, None);
    }
}
