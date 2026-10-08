//! The original Missions window (window type 11, `FUN_0049f130`): the
//! missions under way at a system, the selected mission's agents or decoys,
//! and its target. Recovery notes: `ghidra/notes/sector-quadrants.md`,
//! "Type 11".
//!
//! It opens from the icon a sector window shows at the bottom right of a
//! planet (`FUN_0045d090`, `FUN_0045aac0` kind `0x40`), and a move released
//! over the window targets its system (`+0x70`, `FUN_004aa470`).

use std::collections::BTreeMap;

use egui_macroquad::egui;
use rebellion_core::dat::Faction;
use rebellion_core::fog::FogState;
use rebellion_core::ids::{CharacterKey, SystemKey};
use rebellion_core::missions::{
    ActiveMission, MissionFaction, MissionMember, MissionState, MissionTarget,
};
use rebellion_core::world::GameWorld;

use crate::bmp_cache::{BmpCache, DllSource};
use crate::cockpit::{CockpitFaction, CockpitLayout};
use crate::defenses_window::title_resource;
use crate::fleet_window::{en_route_mark, faction_side, paint_native, MiniObject};
use crate::mission_dialog::{kind_icon, kind_name};
use crate::panels::fleets::capital_ship_mini_id;
use crate::quadrant_icons::{mission_keys, missions_side, visible_members};
use crate::sector_window::planet_resource_id;
use crate::system_window::{
    character_mini_resource_id, clamp_window_to_galaxy, defense_facility_mini, exact_clicked,
    logical_rect, manufacturing_facility_mini, production_facility_mini, rect_contains,
    special_force_mini, troop_mini,
};

pub const MISSIONS_WINDOW_WIDTH: f32 = 235.0;
pub const MISSIONS_WINDOW_HEIGHT: f32 = 304.0;

const BACKGROUND: u32 = 11165;
const CLOSE_NORMAL: u32 = 10108;
const CLOSE_PRESSED: u32 = 10109;
const MINIMIZE_NORMAL: u32 = 10253;
const MINIMIZE_PRESSED: u32 = 10254;
const SECTOR_NORMAL: u32 = 10209;
const SECTOR_PRESSED: u32 = 10208;
/// The frame keyed over a selected mission's picture (`FUN_004a1590`):
/// 11127 while the player is side 1, 11128 otherwise.
const MISSION_FRAME: [u32; 2] = [11127, 11128];
/// A minimized Missions window's rail icon (`FUN_004a21c0`).
const RAIL_ICONS: [u32; 3] = [11539, 11540, 11541];
/// A mission's GOKRES mini is its class's TEXTSTRA id's low 12 bits plus
/// `0x4000` (`FUN_004a1590`); [`kind_icon`] adds `0x1000` for side 2.
const MISSION_MINI_BASE: u32 = 0x4000;

/// The title label `+0x17c` (`FUN_0049fef0`): after the strip's corner (2, 2)
/// and the sector button's width plus 5, 235 less three times 19 wide, 16
/// high.
const TITLE: (f32, f32, f32, f32) = (21.0, 2.0, 178.0, 16.0);
/// The mission list `+0x1b8` (`FUN_00607ea0`, id 10) and its 90 by 50 rows,
/// stacked downwards (`+0xf0` 1, no grid).
const MISSION_LIST: (f32, f32, f32, f32) = (5.0, 24.0, 94.0, 275.0);
const MISSION_ROW: (f32, f32) = (90.0, 50.0);
/// A list item's text rect: the cell moved by the list's (`+0xe8`, `+0xec`)
/// default (1, 0), trimmed by 3 on the right for a left-aligned format.
const TEXT_OFFSET: (f32, f32) = (1.0, 0.0);
const TEXT_RIGHT_TRIM: f32 = 3.0;
/// The tab strip (`FUN_0060d590`, id `0x16`) and its two 61 by 16 buttons.
const TAB_STRIP: (f32, f32) = (105.0, 127.0);
const TAB_SIZE: (f32, f32) = (61.0, 16.0);
/// The member list `+0x1c0` (id `0xb`): a grid of 115 by 43 cells, one to a
/// row at this width, each item a 122 by 43 image.
const MEMBER_LIST: (f32, f32, f32, f32) = (107.0, 145.0, 117.0, 147.0);
const MEMBER_ROW: (f32, f32) = (115.0, 43.0);
const MEMBER_IMAGE: (f32, f32) = (122.0, 43.0);
/// The target picture's box (`FUN_0049fef0`, `FUN_004a10a0`).
const PICTURE: (f32, f32, f32, f32) = (108.0, 37.0, 122.0, 50.0);
/// `+0x178`, TEXTSTRA 34081 "Target:", and `+0x174`, the target's name.
const TARGET_LABEL: (f32, f32, f32) = (109.0, 25.0, 113.0);
const TARGET_NAME: (f32, f32, f32) = (109.0, 91.0, 113.0);

/// The member list's two tabs (`FUN_004a1ba0`), by button id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MissionsTab {
    /// Id `0x17`: members whose role `+0x78` bit 0 is clear.
    Agents,
    /// Id `0x18`: the decoys, bit 0 set.
    Decoys,
}

impl MissionsTab {
    pub const ALL: [Self; 2] = [Self::Agents, Self::Decoys];

    /// The button's x in the strip (`FUN_0049f540`).
    const fn x(self) -> f32 {
        match self {
            Self::Agents => 0.0,
            Self::Decoys => 61.0,
        }
    }

    /// The help message: TEXTSTRA 34048 and 34049.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Agents => "Agents",
            Self::Decoys => "Decoys",
        }
    }
}

/// The tab's bitmap by the selected mission's side (`FUN_004a0ca0`): the
/// normal id, or the pressed id one higher while the tab is selected
/// (`FUN_0060d700` copies it into the unset selected state). A side other
/// than 1 or 2 sets no art.
#[must_use]
pub const fn tab_resource(tab: MissionsTab, side: u8, selected: bool) -> Option<u32> {
    let base = match (tab, side) {
        (MissionsTab::Agents, 1) => 11560,
        (MissionsTab::Decoys, 1) => 11562,
        (MissionsTab::Agents, 2) => 11565,
        (MissionsTab::Decoys, 2) => 11567,
        _ => return None,
    };
    Some(if selected { base + 1 } else { base })
}

/// One mission row (`FUN_004a1590`).
#[derive(Debug, Clone, PartialEq, Eq)]
struct MissionRow {
    mission: u64,
    /// The side bits of the first member seen carrying the mission.
    side: u8,
    picture: u32,
    label: &'static str,
}

/// One member row (`FUN_004a0e10`): the member, its GOKRES mini (`None`
/// where the port maps none) and its name.
#[derive(Debug, Clone, PartialEq, Eq)]
struct MemberRow {
    member: MissionMember,
    mini: Option<u32>,
    label: String,
    /// En route to the mission's target (`+0x50` bit 4): the mark drawn over
    /// its mini (`en_route_mark`).
    en_route: Option<(DllSource, u32)>,
}

/// What the selected mission's details name (`+0x1c8`, `FUN_004a10a0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    System(SystemKey),
    Character(CharacterKey),
    Object(MissionTarget),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OpenMissionsWindow {
    system: SystemKey,
    logical_position: (i16, i16),
    /// The selected mission row `+0x1c4`, by mission id.
    selected: Option<u64>,
    tab: MissionsTab,
}

/// The open Missions windows. The last is focused and paints on top.
#[derive(Debug)]
pub struct MissionsWindowState {
    faction: CockpitFaction,
    windows: Vec<OpenMissionsWindow>,
}

impl Default for MissionsWindowState {
    fn default() -> Self {
        Self {
            faction: CockpitFaction::Alliance,
            windows: Vec::new(),
        }
    }
}

impl MissionsWindowState {
    /// Open `system`'s Missions window at a logical point, clamped into the
    /// galaxy view, with its first mission selected (`FUN_0049f540`,
    /// `FUN_00609500`), or bring its open window to the front: one per
    /// system (`FUN_0045aac0`).
    #[expect(
        clippy::too_many_arguments,
        reason = "Opening selects the first row, which reads the mission and fog state."
    )]
    pub fn open(
        &mut self,
        world: &GameWorld,
        fog: &FogState,
        missions: &MissionState,
        system: SystemKey,
        logical_position: (i16, i16),
        faction: CockpitFaction,
        layout: CockpitLayout,
    ) -> bool {
        self.prepare_faction(faction);
        if !world.systems.contains_key(system) {
            return false;
        }
        if self.focus(system) {
            return true;
        }
        let player = cockpit_faction(faction);
        let selected = mission_rows(world, fog, missions, player, system)
            .first()
            .map(|row| row.mission);
        self.windows.push(OpenMissionsWindow {
            system,
            logical_position: clamp_window_to_galaxy(
                logical_position,
                layout,
                MISSIONS_WINDOW_WIDTH,
                MISSIONS_WINDOW_HEIGHT,
            ),
            selected,
            tab: MissionsTab::Agents,
        });
        true
    }

    #[must_use]
    pub fn window_count(&self) -> usize {
        self.windows.len()
    }

    #[must_use]
    pub fn is_open(&self, system: SystemKey) -> bool {
        self.windows.iter().any(|window| window.system == system)
    }

    #[must_use]
    pub fn contains_screen_point(&self, layout: CockpitLayout, point: (f32, f32)) -> bool {
        let point = egui::pos2(point.0, point.1);
        self.windows
            .iter()
            .any(|window| rect_contains(window_screen_rect(window, layout), point))
    }

    /// The screen rect of the `index`th mission row of `system`'s window.
    #[must_use]
    pub fn mission_row_screen_rect(
        &self,
        layout: CockpitLayout,
        system: SystemKey,
        index: usize,
    ) -> Option<egui::Rect> {
        let window = self.windows.iter().find(|window| window.system == system)?;
        Some(mission_row_rect(
            window_screen_rect(window, layout),
            layout.scale,
            index,
        ))
    }

    /// The screen rect of `tab`'s button in `system`'s window.
    #[must_use]
    pub fn tab_screen_rect(
        &self,
        layout: CockpitLayout,
        system: SystemKey,
        tab: MissionsTab,
    ) -> Option<egui::Rect> {
        let window = self.windows.iter().find(|window| window.system == system)?;
        Some(tab_rect(
            window_screen_rect(window, layout),
            layout.scale,
            tab,
        ))
    }

    /// The screen rect of the `index`th member row of `system`'s window.
    #[must_use]
    pub fn member_row_screen_rect(
        &self,
        layout: CockpitLayout,
        system: SystemKey,
        index: usize,
    ) -> Option<egui::Rect> {
        let window = self.windows.iter().find(|window| window.system == system)?;
        Some(member_row_rect(
            window_screen_rect(window, layout),
            layout.scale,
            index,
        ))
    }

    /// What `system`'s window shows, for the interface fixture.
    #[must_use]
    pub fn report(
        &self,
        world: &GameWorld,
        fog: &FogState,
        missions: &MissionState,
        system: SystemKey,
    ) -> Option<MissionsWindowReport> {
        let window = self.windows.iter().find(|window| window.system == system)?;
        let player = cockpit_faction(self.faction);
        let rows = mission_rows(world, fog, missions, player, system);
        let selected = window
            .selected
            .and_then(|id| rows.iter().position(|row| row.mission == id));
        let selected_row = selected.map(|index| &rows[index]);
        let members = selected_row.map_or_else(Vec::new, |row| {
            member_rows(
                world,
                fog,
                missions,
                player,
                system,
                row.mission,
                window.tab,
            )
        });
        Some(MissionsWindowReport {
            origin: window.logical_position,
            side: missions_side(world, fog, missions, player, system),
            rows: rows
                .iter()
                .map(|row| (row.label.to_string(), row.side, row.picture))
                .collect(),
            selected,
            tab: window.tab,
            tab_side: selected_row.map(|row| row.side),
            members: members.into_iter().map(|row| row.label).collect(),
            target: selected_row
                .map(|row| target_name(world, target(missions, player, system, row))),
        })
    }

    /// The system a move released over the Missions window egui draws as
    /// `layer` takes (`+0x70`, `FUN_004aa470`: the subject wherever the point
    /// is), or `None` when `layer` is none of them.
    #[must_use]
    pub fn release_target(&self, layer: egui::LayerId) -> Option<SystemKey> {
        self.windows
            .iter()
            .find(|window| area_id(window.system) == layer.id)
            .map(|window| window.system)
    }

    pub fn clear(&mut self) {
        self.windows.clear();
    }

    fn prepare_faction(&mut self, faction: CockpitFaction) {
        if self.faction != faction {
            self.faction = faction;
            self.clear();
        }
    }

    /// Show `member` in `system`'s open window (slot 27, `FUN_004a1e10`):
    /// its mission's row is selected and its role bit 0 picks the tab, the
    /// decoys' when set. port: the member row itself is not highlighted.
    pub fn show_member(
        &mut self,
        missions: &MissionState,
        system: SystemKey,
        member: MissionMember,
    ) -> bool {
        let Some(window) = self
            .windows
            .iter_mut()
            .find(|window| window.system == system)
        else {
            return false;
        };
        let Some(mission) = missions
            .missions()
            .iter()
            .find(|mission| mission.team.contains(&member) || mission.decoys.contains(&member))
        else {
            return false;
        };
        window.selected = Some(mission.id);
        window.tab = if mission.decoys.contains(&member) {
            MissionsTab::Decoys
        } else {
            MissionsTab::Agents
        };
        true
    }

    fn focus(&mut self, system: SystemKey) -> bool {
        let Some(index) = self
            .windows
            .iter()
            .position(|window| window.system == system)
        else {
            return false;
        };
        let window = self.windows.remove(index);
        self.windows.push(window);
        true
    }

    fn close(&mut self, system: SystemKey) -> Option<OpenMissionsWindow> {
        let index = self
            .windows
            .iter()
            .position(|window| window.system == system)?;
        Some(self.windows.remove(index))
    }

    fn window_mut(&mut self, system: SystemKey) -> Option<&mut OpenMissionsWindow> {
        self.windows
            .iter_mut()
            .find(|window| window.system == system)
    }
}

/// What one Missions window shows ([`MissionsWindowState::report`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissionsWindowReport {
    pub origin: (i16, i16),
    /// `FUN_004a1f60`'s side: the title strip and rail icon.
    pub side: u8,
    /// Each mission row's name, side and GOKRES mini.
    pub rows: Vec<(String, u8, u32)>,
    pub selected: Option<usize>,
    pub tab: MissionsTab,
    /// The selected mission's side, which picks the tab art.
    pub tab_side: Option<u8>,
    pub members: Vec<String>,
    /// The selected mission's target name, or "Target Unknown".
    pub target: Option<String>,
}

/// Actions that leave the Missions window manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissionsWindowAction {
    /// The sector button (`0xca`) opens the subject's sector window
    /// (`FUN_00429ce0`).
    OpenSector(SystemKey),
    SelectSystem(SystemKey),
    /// The minimize button (`0x15`) posts `0x466`: the window goes to the
    /// galaxy view's rail.
    Minimize {
        system: SystemKey,
        logical_position: (i16, i16),
    },
}

/// A minimized Missions window's rail icon (`FUN_004a21c0`): 11539 for side
/// 1, 11540 for side 2, 11541 otherwise, by `FUN_004a1f60`'s side.
#[must_use]
pub fn rail_icon(
    world: &GameWorld,
    fog: &FogState,
    missions: &MissionState,
    faction: CockpitFaction,
    system: SystemKey,
) -> u32 {
    match missions_side(world, fog, missions, cockpit_faction(faction), system) {
        1 => RAIL_ICONS[0],
        2 => RAIL_ICONS[1],
        _ => RAIL_ICONS[2],
    }
}

fn mission(missions: &MissionState, id: u64) -> Option<&ActiveMission> {
    missions.missions().iter().find(|mission| mission.id == id)
}

const fn side_faction(side: u8) -> MissionFaction {
    if side == 1 {
        MissionFaction::Alliance
    } else {
        MissionFaction::Empire
    }
}

/// `FUN_004a1590`'s rows: each distinct mission of the system's members on a
/// visible mission (a mission key, role `+0x78` bit 8 clear), with the side
/// of the first member carrying it. A mission without a MISSNSD class
/// (`FUN_0051cab0`) gets no row.
///
/// port: the player sees the other side's members only where the System
/// window shows them, as for the sector window's icons. hyp: the rows follow
/// the mission keys' order, the map `FUN_004a1590` collects them in, which
/// for the port's sequential ids is the order the missions began.
/// The members the window lists at `system`: those there, then those on
/// their way to it. A member's transit puts it in its target's container
/// at once (`FUN_00556430` to `+0x74`; `mission-lifecycle.md`), so it is
/// listed there, en route (`+0x50` bit 4), as a fleet is
/// (`move-order.md`). Each carries its side, whether it is on a hidden
/// mission, and whether it is en route.
fn listed_members(
    world: &GameWorld,
    fog: &FogState,
    transits: &[rebellion_core::missions::MemberTransit],
    player: Faction,
    system: SystemKey,
) -> Vec<(MissionMember, u8, bool, bool)> {
    let mut members: Vec<_> = visible_members(world, fog, player, system)
        .into_iter()
        .map(|(member, side, hidden)| (member, side, hidden, false))
        .collect();
    if world.systems.get(system).is_none_or(|value| {
        value.exploration_status == rebellion_core::dat::ExplorationStatus::Unexplored
    }) {
        return members;
    }
    let opposing_visible =
        crate::system_window::opposing_contents_visible(world, fog, player, system);
    let own = crate::fleet_window::faction_side(player);
    for transit in transits.iter().filter(|transit| transit.to == system) {
        let (side, hidden) = match transit.member {
            MissionMember::Character(key) => match world.characters.get(key) {
                Some(character) if !character.is_killed => (
                    if character.is_alliance {
                        1
                    } else if character.is_empire {
                        2
                    } else {
                        0
                    },
                    character.on_hidden_mission,
                ),
                _ => continue,
            },
            MissionMember::SpecialForce(key) => match world.special_forces.get(key) {
                Some(force) => (crate::fleet_window::fleet_side(force.is_alliance), false),
                None => continue,
            },
        };
        if (opposing_visible || side == own) && !members.iter().any(|(m, ..)| *m == transit.member)
        {
            members.push((transit.member, side, hidden, true));
        }
    }
    members
}

fn mission_rows(
    world: &GameWorld,
    fog: &FogState,
    missions: &MissionState,
    player: Faction,
    system: SystemKey,
) -> Vec<MissionRow> {
    let keys = mission_keys(missions);
    let mut sides = BTreeMap::new();
    for (member, side, hidden, _) in listed_members(world, fog, missions.en_route(), player, system)
    {
        if hidden {
            continue;
        }
        if let Some(&key) = keys.get(&member) {
            sides.entry(key).or_insert(side);
        }
    }
    sides
        .into_iter()
        .filter_map(|(id, side)| {
            let kind = mission(missions, id)?.kind;
            kind.record_id()?;
            Some(MissionRow {
                mission: id,
                side,
                picture: MISSION_MINI_BASE + kind_icon(kind, side_faction(side)),
                label: kind_name(kind),
            })
        })
        .collect()
}

/// `FUN_004a1ba0`'s rows for `tab`: the members at the system carrying the
/// mission, in the walk's order; the decoys on the Decoys tab, the rest on
/// the Agents tab.
fn member_rows(
    world: &GameWorld,
    fog: &FogState,
    missions: &MissionState,
    player: Faction,
    system: SystemKey,
    id: u64,
    tab: MissionsTab,
) -> Vec<MemberRow> {
    let Some(value) = mission(missions, id) else {
        return Vec::new();
    };
    let keys = mission_keys(missions);
    listed_members(world, fog, missions.en_route(), player, system)
        .into_iter()
        .filter(|(member, ..)| keys.get(member) == Some(&id))
        .filter(|(member, ..)| value.decoys.contains(member) == (tab == MissionsTab::Decoys))
        .filter_map(|(member, _, _, en_route)| {
            let (mini, label) = member_mini(world, member)?;
            Some(MemberRow {
                member,
                mini,
                label,
                en_route: mini
                    .filter(|_| en_route)
                    .and_then(|mini| Some(en_route_mark(mini_object(world, member)?, mini))),
            })
        })
        .collect()
}

/// What `member`'s mini stands for, as `FUN_0042c3b0` picks its en route
/// mark.
fn mini_object(world: &GameWorld, member: MissionMember) -> Option<MiniObject> {
    match member {
        MissionMember::Character(_) => Some(MiniObject::Character),
        MissionMember::SpecialForce(key) => world
            .special_forces
            .get(key)
            .map(|force| MiniObject::SpecialForce(force.class_dat_id)),
    }
}

/// A member's GOKRES mini and name (`FUN_0042c3b0(.., 0, 1)`,
/// `FUN_004f62d0`). port: of the mini's status overlays only the en route
/// mark is drawn (`MemberRow::en_route`); the others are not, as in the
/// Mission dialog.
fn member_mini(world: &GameWorld, member: MissionMember) -> Option<(Option<u32>, String)> {
    match member {
        MissionMember::Character(key) => {
            let character = world.characters.get(key)?;
            Some((
                character_mini_resource_id(character.dat_id, character.is_major),
                character.name.clone(),
            ))
        }
        MissionMember::SpecialForce(key) => {
            let force = world.special_forces.get(key)?;
            let (mini, label) = special_force_mini(force.class_dat_id)?;
            Some((Some(mini), label.into()))
        }
    }
}

/// `FUN_004a10a0`'s target `+0x1c8`. The player's own mission names its
/// order's target (`FUN_004f3000`, `+0x70`); another side's mission in
/// families `0x50..0x5f` names the system; any other names nothing.
fn target(
    missions: &MissionState,
    player: Faction,
    system: SystemKey,
    row: &MissionRow,
) -> Option<Target> {
    let value = mission(missions, row.mission)?;
    if row.side == faction_side(player) {
        return Some(if let Some(character) = value.target_character {
            Target::Character(character)
        } else if let Some(object) = value.target_object {
            Target::Object(object)
        } else {
            Target::System(value.target_system)
        });
    }
    let family = value.kind.record_id()?.family();
    (0x50..=0x5f)
        .contains(&family)
        .then_some(Target::System(system))
}

/// The target's picture and name, when it still exists: a system's planet
/// (STRATEGY, `FUN_0045c970`), a character's GOKRES mini (`FUN_0042c3b0(..,
/// 0, 1)`), or another object's GOKRES portrait, its mini less `0x4000`
/// (`FUN_0042c3b0(.., 1, 1)`).
fn target_view(world: &GameWorld, target: Target) -> Option<(DllSource, Option<u32>, String)> {
    let portrait = |found: Option<(u32, &str)>| {
        found.map(|(mini, label)| {
            (
                DllSource::Gokres,
                Some(mini - MISSION_MINI_BASE),
                label.to_string(),
            )
        })
    };
    match target {
        Target::System(key) => {
            let system = world.systems.get(key)?;
            Some((
                DllSource::Strategy,
                Some(planet_resource_id(system.dat_id)),
                system.name.clone(),
            ))
        }
        Target::Character(key) => {
            let character = world.characters.get(key)?;
            Some((
                DllSource::Gokres,
                character_mini_resource_id(character.dat_id, character.is_major),
                character.name.clone(),
            ))
        }
        Target::Object(MissionTarget::DefenseFacility(key)) => portrait(defense_facility_mini(
            world.defense_facilities.get(key)?.class_dat_id,
        )),
        Target::Object(MissionTarget::ManufacturingFacility(key)) => portrait(
            manufacturing_facility_mini(world.manufacturing_facilities.get(key)?.class_dat_id),
        ),
        Target::Object(MissionTarget::ProductionFacility(key)) => portrait(
            production_facility_mini(world.production_facilities.get(key)?.class_dat_id),
        ),
        Target::Object(MissionTarget::Troop(key)) => {
            portrait(troop_mini(world.troops.get(key)?.class_dat_id))
        }
        Target::Object(MissionTarget::SpecialForce(key)) => portrait(special_force_mini(
            world.special_forces.get(key)?.class_dat_id,
        )),
        Target::Object(MissionTarget::DeathStar(key)) => {
            let class = world
                .fleets
                .get(key)?
                .capital_ships
                .iter()
                .find_map(|ship| {
                    world
                        .capital_ship_classes
                        .get(ship.class)
                        .filter(|class| ship.alive && class.is_death_star())
                })?;
            Some((
                DllSource::Gokres,
                capital_ship_mini_id(class.dat_id).map(|mini| mini - MISSION_MINI_BASE),
                class.name.clone(),
            ))
        }
    }
}

/// `+0x174`: the target's name, or TEXTSTRA 34087 "Target Unknown".
fn target_name(world: &GameWorld, target: Option<Target>) -> String {
    target
        .and_then(|target| target_view(world, target))
        .map_or_else(|| "Target Unknown".to_string(), |(_, _, name)| name)
}

/// Where a bitmap lands centred in a box: the original's integer
/// `(box - size) / 2` offsets (`FUN_0049fef0`, `FUN_004a0e10`).
#[expect(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    reason = "Boxes and bitmap sizes are small integers."
)]
fn centered(box_size: (f32, f32), size: [usize; 2]) -> (f32, f32) {
    let offset = |outer: f32, inner: usize| ((outer as i32 - inner as i32) / 2) as f32;
    (offset(box_size.0, size[0]), offset(box_size.1, size[1]))
}

/// Where the target picture lands: centred in its box (`FUN_0049fef0`), or
/// at the box's corner when its size is unknown.
fn picture_at(size: Option<[usize; 2]>) -> (f32, f32) {
    let (x, y) = size.map_or((0.0, 0.0), |size| centered((PICTURE.2, PICTURE.3), size));
    (PICTURE.0 + x, PICTURE.1 + y)
}

/// A member row's text top (`FUN_004a0e10`, item `+0x34`): the mini's height
/// plus its centring offset, so the name sits under the mini.
#[expect(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    reason = "Bitmap heights are small integers."
)]
fn member_text_top(mini_height: usize) -> f32 {
    let height = mini_height as i32;
    (height + (MEMBER_IMAGE.1 as i32 - height) / 2) as f32
}

fn cockpit_faction(faction: CockpitFaction) -> Faction {
    match faction {
        CockpitFaction::Alliance => Faction::Alliance,
        CockpitFaction::Empire => Faction::Empire,
    }
}

fn area_id(system: SystemKey) -> egui::Id {
    egui::Id::new(("original-missions-window", system))
}

fn window_screen_rect(window: &OpenMissionsWindow, layout: CockpitLayout) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(
            layout.canvas.x + f32::from(window.logical_position.0) * layout.scale,
            layout.canvas.y + f32::from(window.logical_position.1) * layout.scale,
        ),
        egui::vec2(
            MISSIONS_WINDOW_WIDTH * layout.scale,
            MISSIONS_WINDOW_HEIGHT * layout.scale,
        ),
    )
}

fn tab_rect(window: egui::Rect, scale: f32, tab: MissionsTab) -> egui::Rect {
    logical_rect(
        window,
        scale,
        TAB_STRIP.0 + tab.x(),
        TAB_STRIP.1,
        TAB_SIZE.0,
        TAB_SIZE.1,
    )
}

/// The `index`th mission row (`FUN_00609ae0`, `+0xf0` 1): stacked from the
/// list's corner.
#[expect(clippy::cast_precision_loss, reason = "Row indexes are small.")]
fn mission_row_rect(window: egui::Rect, scale: f32, index: usize) -> egui::Rect {
    logical_rect(
        window,
        scale,
        MISSION_LIST.0,
        MISSION_LIST.1 + index as f32 * MISSION_ROW.1,
        MISSION_ROW.0,
        MISSION_ROW.1,
    )
}

/// The `index`th member row (`FUN_00609ae0`'s grid): the next cell would
/// pass the list's width, so each row holds one.
#[expect(clippy::cast_precision_loss, reason = "Row indexes are small.")]
fn member_row_rect(window: egui::Rect, scale: f32, index: usize) -> egui::Rect {
    logical_rect(
        window,
        scale,
        MEMBER_LIST.0,
        MEMBER_LIST.1 + index as f32 * MEMBER_ROW.1,
        MEMBER_ROW.0,
        MEMBER_ROW.1,
    )
}

#[derive(Default)]
struct WindowDrawResult {
    focus: bool,
    close: bool,
    minimize: bool,
    open_sector: bool,
    select: Option<u64>,
    tab: Option<MissionsTab>,
}

/// Draw every open Missions window.
///
/// port: the scroll bars, the tab help text, the list drags (`FUN_004a0400`)
/// and Escape (`FUN_004a0e00`) are not ported; the galaxy view's windows keep
/// no shared keyboard focus to route a key by.
#[expect(
    clippy::too_many_arguments,
    reason = "Keep explicit state and rendering inputs at this UI boundary, as the Fleet window does."
)]
pub fn draw_missions_windows(
    ctx: &egui::Context,
    world: &GameWorld,
    fog: &FogState,
    missions: &MissionState,
    state: &mut MissionsWindowState,
    faction: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> Vec<MissionsWindowAction> {
    state.prepare_faction(faction);
    let player = cockpit_faction(faction);
    // FUN_004a0c20: a selected mission that leaves the list clears the
    // selection.
    for window in &mut state.windows {
        let rows = mission_rows(world, fog, missions, player, window.system);
        if window
            .selected
            .is_some_and(|id| !rows.iter().any(|row| row.mission == id))
        {
            window.selected = None;
        }
    }
    let windows = state.windows.clone();
    let focused_system = windows.last().map(|window| window.system);
    let mut actions = Vec::new();
    for window in &windows {
        let result = draw_missions_window(
            ctx,
            world,
            fog,
            missions,
            window,
            focused_system == Some(window.system),
            faction,
            layout,
            cache,
        );
        let system = window.system;
        if result.close || !world.systems.contains_key(system) {
            state.close(system);
            continue;
        }
        if result.minimize {
            if let Some(closed) = state.close(system) {
                actions.push(MissionsWindowAction::Minimize {
                    system,
                    logical_position: closed.logical_position,
                });
            }
            continue;
        }
        if result.open_sector {
            actions.push(MissionsWindowAction::OpenSector(system));
        }
        if let Some(open) = state.window_mut(system) {
            // FUN_004a0c60: a different mission refreshes the details and
            // reselects the Agents tab (FUN_004a0ca0).
            if let Some(id) = result.select.filter(|&id| open.selected != Some(id)) {
                open.selected = Some(id);
                open.tab = MissionsTab::Agents;
            }
            if let Some(tab) = result.tab {
                open.tab = tab;
            }
        }
        if result.focus {
            state.focus(system);
            actions.push(MissionsWindowAction::SelectSystem(system));
        }
    }
    actions
}

#[expect(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    reason = "Keep the window's ordered paint and input pass together, as the Defenses window does."
)]
fn draw_missions_window(
    ctx: &egui::Context,
    world: &GameWorld,
    fog: &FogState,
    missions: &MissionState,
    window: &OpenMissionsWindow,
    focused: bool,
    faction: CockpitFaction,
    layout: CockpitLayout,
    cache: &mut BmpCache,
) -> WindowDrawResult {
    let mut result = WindowDrawResult::default();
    let Some(system) = world.systems.get(window.system) else {
        result.close = true;
        return result;
    };
    let player = cockpit_faction(faction);
    let side = missions_side(world, fog, missions, player, window.system);
    let rows = mission_rows(world, fog, missions, player, window.system);
    let selected = window
        .selected
        .and_then(|id| rows.iter().find(|row| row.mission == id));
    let scale = layout.scale;
    let screen_rect = window_screen_rect(window, layout);
    let id = area_id(window.system);
    if focused {
        ctx.move_to_top(egui::LayerId::new(egui::Order::Foreground, id));
    }
    let area = egui::Area::new(id)
        .fixed_pos(screen_rect.min)
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            let (pointer, primary_down) = ctx.input(|input| {
                (
                    input.pointer.interact_pos(),
                    input.pointer.button_down(egui::PointerButton::Primary),
                )
            });
            let (local, window_response) =
                ui.allocate_exact_size(screen_rect.size(), egui::Sense::click());
            let painter = ui.painter().clone();
            let white = egui::Color32::WHITE;
            let label_font = egui::FontId::proportional((11.0 * scale).max(7.0));
            let list_font = egui::FontId::proportional((9.0 * scale).max(6.0));

            painter.rect_filled(local, 0.0, egui::Color32::BLACK);
            paint_native(
                &painter,
                ctx,
                cache,
                DllSource::Strategy,
                BACKGROUND,
                local,
                scale,
                0.0,
                0.0,
            );

            // The paint slot (FUN_0049fef0): the target picture centred in
            // its box, `+0x174` and `+0x178`, then the strip and the title.
            if let Some(row) = selected {
                let target = target(missions, player, window.system, row);
                let view = target.and_then(|target| target_view(world, target));
                if let Some((source, Some(picture), _)) = &view {
                    let (x, y) = picture_at(cache.original_resource_size(*source, *picture));
                    paint_native(&painter, ctx, cache, *source, *picture, local, scale, x, y);
                }
                // `+0x174`: font 5, format 0x11 (centred, word-wrapped).
                let name = view.map_or_else(|| "Target Unknown".to_string(), |(_, _, name)| name);
                let mut job = egui::text::LayoutJob::simple(
                    name,
                    label_font.clone(),
                    white,
                    TARGET_NAME.2 * scale,
                );
                job.halign = egui::Align::Center;
                painter.galley(
                    logical_rect(
                        local,
                        scale,
                        TARGET_NAME.0 + TARGET_NAME.2 / 2.0,
                        TARGET_NAME.1,
                        0.0,
                        0.0,
                    )
                    .min,
                    painter.layout_job(job),
                    white,
                );
                // `+0x178`: font 5, format 1 (centred), TEXTSTRA 34081.
                painter.text(
                    logical_rect(
                        local,
                        scale,
                        TARGET_LABEL.0 + TARGET_LABEL.2 / 2.0,
                        TARGET_LABEL.1,
                        0.0,
                        0.0,
                    )
                    .min,
                    egui::Align2::CENTER_TOP,
                    "Target:",
                    label_font.clone(),
                    white,
                );
            }
            paint_native(
                &painter,
                ctx,
                cache,
                DllSource::Strategy,
                title_resource(side, focused),
                local,
                scale,
                2.0,
                2.0,
            );
            // `+0x17c`: TEXTSTRA 34085 and the system name, font 5, format
            // 0x24 (one line, left-aligned, vertically centred).
            painter.text(
                logical_rect(local, scale, TITLE.0, TITLE.1 + TITLE.3 / 2.0, 0.0, 0.0).min,
                egui::Align2::LEFT_CENTER,
                format!("Mission at {}", system.name),
                label_font,
                egui::Color32::BLACK,
            );

            let buttons = [
                (SECTOR_NORMAL, SECTOR_PRESSED, 3.0, "sector"),
                (MINIMIZE_NORMAL, MINIMIZE_PRESSED, 204.0, "minimize"),
                (CLOSE_NORMAL, CLOSE_PRESSED, 218.0, "close"),
            ];
            let mut clicked = [false; 3];
            for (index, (normal, pressed, x, name)) in buttons.into_iter().enumerate() {
                let rect = logical_rect(local, scale, x, 3.0, 14.0, 14.0);
                let response = ui.interact(
                    rect,
                    ui.id().with((window.system, name)),
                    egui::Sense::click(),
                );
                let down = primary_down && pointer.is_some_and(|point| rect_contains(rect, point));
                paint_native(
                    &painter,
                    ctx,
                    cache,
                    DllSource::Strategy,
                    if down { pressed } else { normal },
                    local,
                    scale,
                    x,
                    3.0,
                );
                clicked[index] = exact_clicked(&response, rect);
            }
            result.open_sector = clicked[0];
            result.minimize = clicked[1];
            result.close = clicked[2];

            // The mission list: each picture keyed at its row's corner, the
            // frame keyed over it while selected, and the name word-wrapped
            // from (1, 0), white either way (`FUN_00607ea0`'s `+0xdc`).
            // port: the scroll bar is not drawn, so the rows past the
            // list's foot are cut off.
            let list = logical_rect(
                local,
                scale,
                MISSION_LIST.0,
                MISSION_LIST.1,
                MISSION_LIST.2,
                MISSION_LIST.3,
            );
            let list_painter = painter.with_clip_rect(list);
            let frame = if faction_side(player) == 1 {
                MISSION_FRAME[0]
            } else {
                MISSION_FRAME[1]
            };
            for (index, row) in rows.iter().enumerate() {
                let cell = mission_row_rect(local, scale, index);
                if !cell.intersects(list) {
                    break;
                }
                paint_native(
                    &list_painter,
                    ctx,
                    cache,
                    DllSource::Gokres,
                    row.picture,
                    cell,
                    scale,
                    0.0,
                    0.0,
                );
                if window.selected == Some(row.mission) {
                    paint_native(
                        &list_painter,
                        ctx,
                        cache,
                        DllSource::Strategy,
                        frame,
                        cell,
                        scale,
                        0.0,
                        0.0,
                    );
                }
                let galley = list_painter.layout(
                    row.label.to_string(),
                    list_font.clone(),
                    white,
                    (MISSION_ROW.0 - TEXT_RIGHT_TRIM) * scale,
                );
                list_painter.galley(
                    logical_rect(cell, scale, TEXT_OFFSET.0, TEXT_OFFSET.1, 0.0, 0.0).min,
                    galley,
                    white,
                );
                let response = ui.interact(
                    cell.intersect(list),
                    ui.id().with((window.system, "mission", index)),
                    egui::Sense::click(),
                );
                if exact_clicked(&response, cell) {
                    result.select = Some(row.mission);
                    result.focus = true;
                }
            }

            // With a mission selected, its side's tabs and the current
            // tab's members (FUN_004a0ca0); with none, both hide.
            if let Some(row) = selected {
                for tab in MissionsTab::ALL {
                    let rect = tab_rect(local, scale, tab);
                    if let Some(art) = tab_resource(tab, row.side, tab == window.tab) {
                        paint_native(
                            &painter,
                            ctx,
                            cache,
                            DllSource::Strategy,
                            art,
                            rect,
                            scale,
                            0.0,
                            0.0,
                        );
                    }
                    let response = ui.interact(
                        rect,
                        ui.id().with((window.system, tab)),
                        egui::Sense::click(),
                    );
                    if exact_clicked(&response, rect) {
                        result.tab = Some(tab);
                        result.focus = true;
                    }
                }
                let members = logical_rect(
                    local,
                    scale,
                    MEMBER_LIST.0,
                    MEMBER_LIST.1,
                    MEMBER_LIST.2,
                    MEMBER_LIST.3,
                );
                let member_painter = painter.with_clip_rect(members);
                let rows = member_rows(
                    world,
                    fog,
                    missions,
                    player,
                    window.system,
                    row.mission,
                    window.tab,
                );
                for (index, member) in rows.iter().enumerate() {
                    let cell = member_row_rect(local, scale, index);
                    if !cell.intersects(members) {
                        break;
                    }
                    let size = member
                        .mini
                        .and_then(|mini| cache.original_resource_size(DllSource::Gokres, mini));
                    if let Some(mini) = member.mini {
                        let (x, y) = size.map_or((0.0, 0.0), |size| centered(MEMBER_IMAGE, size));
                        paint_native(
                            &member_painter,
                            ctx,
                            cache,
                            DllSource::Gokres,
                            mini,
                            cell,
                            scale,
                            x,
                            y,
                        );
                        if let Some((source, mark)) = member.en_route {
                            paint_native(
                                &member_painter,
                                ctx,
                                cache,
                                source,
                                mark,
                                cell,
                                scale,
                                x,
                                y,
                            );
                        }
                    }
                    // Format 0x21: one line, centred in the cell less 2 on
                    // each side, under the mini (item `+0x34`), font 10.
                    member_painter.text(
                        logical_rect(
                            cell,
                            scale,
                            TEXT_OFFSET.0 + MEMBER_ROW.0 / 2.0,
                            member_text_top(size.map_or(0, |size| size[1])),
                            0.0,
                            0.0,
                        )
                        .min,
                        egui::Align2::CENTER_TOP,
                        &member.label,
                        list_font.clone(),
                        white,
                    );
                    let response = ui.interact(
                        cell.intersect(members),
                        ui.id().with((window.system, "member", index)),
                        egui::Sense::click(),
                    );
                    if exact_clicked(&response, cell) {
                        result.focus = true;
                    }
                }
            }

            if window_response.clicked() || clicked.iter().any(|&value| value) {
                result.focus = true;
            }
        });
    if area.response.clicked() {
        result.focus = true;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cockpit::CockpitViewport;
    use crate::fleet_window::tests::PAINTED;
    use rebellion_core::dat::{ExplorationStatus, SectorGroup};
    use rebellion_core::ids::DatId;
    use rebellion_core::missions::{MissionKind, MissionRequest};
    use rebellion_core::world::{Character, ControlKind, Sector, System};

    fn world(control: ControlKind) -> (GameWorld, SystemKey) {
        let mut world = GameWorld::default();
        let sector = world.sectors.insert(Sector {
            dat_id: DatId::new(36),
            name: "Sesswenna".into(),
            group: SectorGroup::Core,
            x: 317,
            y: 248,
            systems: Vec::new(),
        });
        let system = world.systems.insert(System {
            dat_id: DatId::new(100),
            name: "Sluis Van".into(),
            sector,
            x: 320,
            y: 250,
            exploration_status: ExplorationStatus::Explored,
            popularity_alliance: 0.5,
            popularity_empire: 0.5,
            is_populated: true,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: Vec::new(),
            ground_units: Vec::new(),
            special_forces: Vec::new(),
            defense_facilities: Vec::new(),
            manufacturing_facilities: Vec::new(),
            production_facilities: Vec::new(),
            is_headquarters: false,
            is_destroyed: false,
            control,
        });
        (world, system)
    }

    fn character(
        world: &mut GameWorld,
        system: SystemKey,
        name: &str,
        alliance: bool,
    ) -> CharacterKey {
        world.characters.insert(Character {
            dat_id: DatId::new(832),
            name: name.into(),
            is_alliance: alliance,
            is_empire: !alliance,
            current_system: Some(system),
            recruited: true,
            ..Default::default()
        })
    }

    fn faction(alliance: bool) -> MissionFaction {
        if alliance {
            MissionFaction::Alliance
        } else {
            MissionFaction::Empire
        }
    }

    /// Send `team` and `decoys` on a `kind` mission at `system`.
    fn send(
        missions: &mut MissionState,
        kind: MissionKind,
        alliance: bool,
        team: &[CharacterKey],
        decoys: &[CharacterKey],
        system: SystemKey,
        target_character: Option<CharacterKey>,
    ) -> u64 {
        missions.dispatch(MissionRequest {
            kind,
            faction: faction(alliance),
            team: team.iter().copied().map(MissionMember::Character).collect(),
            decoys: decoys
                .iter()
                .copied()
                .map(MissionMember::Character)
                .collect(),
            target_system: system,
            target_character,
            target_object: None,
            tick: 0,
        })
    }

    fn unseen() -> FogState {
        FogState::new(Faction::Alliance)
    }

    /// An Alliance system with an Alliance Diplomacy mission (two agents and
    /// a decoy) and an Empire Espionage mission (one agent).
    fn busy() -> (GameWorld, SystemKey, MissionState, [CharacterKey; 4]) {
        let (mut world, system) = world(ControlKind::Controlled(Faction::Alliance));
        let first = character(&mut world, system, "Leia", true);
        let second = character(&mut world, system, "Mon", true);
        let decoy = character(&mut world, system, "Decoy", true);
        let spy = character(&mut world, system, "Spy", false);
        let mut missions = MissionState::new();
        send(
            &mut missions,
            MissionKind::Diplomacy,
            true,
            &[first, second],
            &[decoy],
            system,
            None,
        );
        send(
            &mut missions,
            MissionKind::Espionage,
            false,
            &[spy],
            &[],
            system,
            None,
        );
        (world, system, missions, [first, second, decoy, spy])
    }

    #[test]
    fn a_member_on_its_way_is_listed_at_its_target_with_the_en_route_mark() {
        // FUN_00556430 puts a travelling member in its target's container at
        // once (mission-lifecycle.md); FUN_0042c3b0 draws 11501 over the
        // mini of an en route (+0x50 bit 4) character.
        let (mut world, system, _, _) = busy();
        let traveller = world.characters.insert(Character {
            dat_id: DatId::new(832),
            name: "Traveller".into(),
            is_alliance: true,
            current_system: None,
            recruited: true,
            ..Default::default()
        });
        let member = MissionMember::Character(traveller);
        let transit = rebellion_core::missions::MemberTransit {
            member,
            mission_id: 0,
            to: system,
            arrival: 9,
        };
        let listed = |transits: &[rebellion_core::missions::MemberTransit]| {
            listed_members(&world, &unseen(), transits, Faction::Alliance, system)
        };
        assert!(!listed(&[]).iter().any(|(m, ..)| *m == member));
        let with = listed(&[transit]);
        assert!(with.contains(&(member, 1, false, true)), "{with:?}");
        assert_eq!(
            mini_object(&world, member).map(|object| en_route_mark(object, 0)),
            Some((DllSource::Strategy, 11_501))
        );
    }

    #[test]
    fn a_killed_or_unseen_traveller_is_not_listed_and_two_force_classes_carry_their_own_mark() {
        // port: a member bound for a system shows where the System window
        // shows its side's members, and a killed one not at all.
        // FUN_0042c3b0 keeps the GOKRES default (class & 0xfff) + 0x5000 for
        // classes 0x3c000003/0x3c000005 instead of 11501.
        let (mut world, system, _, _) = busy();
        world.systems[system].control = ControlKind::Controlled(Faction::Empire);
        let mut traveller = |name: &str, is_alliance: bool, is_killed: bool| {
            MissionMember::Character(world.characters.insert(Character {
                dat_id: DatId::new(832),
                name: name.into(),
                is_alliance,
                is_empire: !is_alliance,
                is_killed,
                recruited: true,
                ..Default::default()
            }))
        };
        let own = traveller("Own", true, false);
        let killed = traveller("Killed", true, true);
        let enemy = traveller("Enemy", false, false);
        let transits: Vec<_> = [own, killed, enemy]
            .into_iter()
            .map(|member| rebellion_core::missions::MemberTransit {
                member,
                mission_id: 0,
                to: system,
                arrival: 9,
            })
            .collect();
        let listed = |fog: &FogState| {
            listed_members(&world, fog, &transits, Faction::Alliance, system)
                .into_iter()
                .filter(|(.., en_route)| *en_route)
                .map(|(member, ..)| member)
                .collect::<Vec<_>>()
        };
        assert_eq!(listed(&unseen()), [own]);
        let mut seen = unseen();
        seen.reveal(system);
        assert_eq!(listed(&seen), [own, enemy]);

        let mut force = |class: u32| {
            MissionMember::SpecialForce(world.special_forces.insert(
                rebellion_core::world::SpecialForceUnit {
                    class_dat_id: DatId::new(class),
                    is_alliance: true,
                    skills: [0; 8],
                    on_mission: false,
                },
            ))
        };
        let forces: Vec<_> = [
            (0x3c00_0001, (DllSource::Strategy, 11_501)),
            (0x3c00_0003, (DllSource::Gokres, 21_826)),
            (0x3c00_0005, (DllSource::Gokres, 21_888)),
        ]
        .into_iter()
        .map(|(class, mark)| (class, force(class), mark))
        .collect();
        for (class, member, mark) in forces {
            let mini = member_mini(&world, member).and_then(|(mini, _)| mini);
            assert_eq!(
                mini.zip(mini_object(&world, member))
                    .map(|(mini, object)| en_route_mark(object, mini)),
                Some(mark),
                "0x{class:x}"
            );
        }
    }

    fn labels(rows: &[MissionRow]) -> Vec<(&'static str, u8, u32)> {
        rows.iter()
            .map(|row| (row.label, row.side, row.picture))
            .collect()
    }

    #[test]
    fn each_visible_mission_gets_one_row_with_its_sides_mini_and_name() {
        // FUN_004a1590: one row per mission key of the members on a visible
        // mission; GOKRES 0x4000 + (class +0x30 & 0xfff), 0x5000 + .. for
        // side 2: Diplomacy (TEXTSTRA 0x2c10) 0x4c10, Espionage (0x2c13)
        // 0x5c13.
        let (world, system, missions, _) = busy();
        assert_eq!(
            labels(&mission_rows(
                &world,
                &unseen(),
                &missions,
                Faction::Alliance,
                system
            )),
            [("Diplomacy", 1, 0x4c10), ("Espionage", 2, 0x5c13)]
        );
    }

    #[test]
    fn hidden_missions_and_unseen_enemies_get_no_row() {
        // FUN_004a1590 skips role +0x78 bit 8 (a hidden mission). port: the
        // other side's members show only where the System window shows
        // them (an own or revealed system).
        let (mut world, system, missions, [first, second, decoy, _]) = busy();
        for member in [first, second, decoy] {
            world.characters[member].on_hidden_mission = true;
        }
        world.systems[system].control = ControlKind::Controlled(Faction::Empire);
        assert!(mission_rows(&world, &unseen(), &missions, Faction::Alliance, system).is_empty());

        let mut seen = unseen();
        seen.reveal(system);
        assert_eq!(
            labels(&mission_rows(
                &world,
                &seen,
                &missions,
                Faction::Alliance,
                system
            )),
            [("Espionage", 2, 0x5c13)]
        );
    }

    #[test]
    fn the_rail_icon_and_title_strip_prefer_the_other_sides_missions() {
        // FUN_004a21c0 and FUN_004a2200 read FUN_004a1f60: the other side's
        // missions win, then the player's, else side 3.
        let (mut world, system, mut missions, [.., spy]) = busy();
        let icon = |world: &GameWorld, missions: &MissionState| {
            rail_icon(world, &unseen(), missions, CockpitFaction::Alliance, system)
        };
        assert_eq!(icon(&world, &missions), 11540);
        world.characters[spy].current_system = None;
        assert_eq!(icon(&world, &missions), 11539);
        missions = MissionState::new();
        assert_eq!(icon(&world, &missions), 11541);
        assert_eq!(
            [1, 2, 3].map(|side| [title_resource(side, true), title_resource(side, false)]),
            [[10299, 10200], [10201, 10302], [10303, 10304]]
        );
    }

    #[test]
    fn each_tab_shows_the_selected_missions_side_art() {
        // FUN_004a0ca0: 0x2d28/0x2d29 and 0x2d2a/0x2d2b for side 1,
        // 0x2d2d/0x2d2e and 0x2d2f/0x2d30 for side 2, none otherwise; the
        // selected tab shows its pressed bitmap (FUN_0060d700).
        let cases = [
            (MissionsTab::Agents, 1, Some([11560, 11561])),
            (MissionsTab::Decoys, 1, Some([11562, 11563])),
            (MissionsTab::Agents, 2, Some([11565, 11566])),
            (MissionsTab::Decoys, 2, Some([11567, 11568])),
            (MissionsTab::Agents, 3, None),
        ];
        for (tab, side, art) in cases {
            assert_eq!(
                tab_resource(tab, side, false).zip(tab_resource(tab, side, true)),
                art.map(|[normal, selected]| (normal, selected)),
                "{tab:?} side {side}"
            );
        }
        // TEXTSTRA 34048 and 34049.
        assert_eq!(
            MissionsTab::ALL.map(MissionsTab::name),
            ["Agents", "Decoys"]
        );
    }

    #[test]
    fn opening_selects_the_first_mission_and_its_agents_then_its_decoys_on_their_tab() {
        // FUN_0049f540 selects the first row (FUN_00609500); FUN_004a0ca0
        // fills the Agents tab (FUN_004a1ba0(0x17)): members carrying the
        // key with +0x78 bit 0 clear, the decoys on tab 0x18.
        let (world, system, missions, _) = busy();
        let fog = unseen();
        let mut state = MissionsWindowState::default();
        assert!(state.open(
            &world,
            &fog,
            &missions,
            system,
            (20, 30),
            CockpitFaction::Alliance,
            scaled()
        ));
        let report = state.report(&world, &fog, &missions, system).unwrap();
        assert_eq!(report.selected, Some(0));
        assert_eq!(report.tab, MissionsTab::Agents);
        assert_eq!(report.tab_side, Some(1));
        assert_eq!(report.members, ["Leia", "Mon"]);
        assert_eq!(report.side, 2);

        state.windows[0].tab = MissionsTab::Decoys;
        let report = state.report(&world, &fog, &missions, system).unwrap();
        assert_eq!(report.members, ["Decoy"]);
    }

    #[test]
    fn showing_a_member_selects_its_mission_and_the_tab_its_role_picks() {
        // FUN_004a1e10 (slot 27): the member's mission key selects its row;
        // role bit 0 picks the decoys' tab. FUN_00429440 sends a character
        // on a visible mission here (kind 11).
        let (world, system, missions, [_, _, decoy, spy]) = busy();
        let fog = unseen();
        let mut state = MissionsWindowState::default();
        assert!(!state.show_member(&missions, system, MissionMember::Character(spy)));
        assert!(state.open(
            &world,
            &fog,
            &missions,
            system,
            (20, 30),
            CockpitFaction::Alliance,
            scaled()
        ));

        assert!(state.show_member(&missions, system, MissionMember::Character(spy)));
        assert_eq!(state.windows[0].selected, Some(1));
        assert_eq!(state.windows[0].tab, MissionsTab::Agents);

        assert!(state.show_member(&missions, system, MissionMember::Character(decoy)));
        assert_eq!(state.windows[0].selected, Some(0));
        assert_eq!(state.windows[0].tab, MissionsTab::Decoys);

        let idle = CharacterKey::default();
        assert!(!state.show_member(&missions, system, MissionMember::Character(idle)));
        assert_eq!(state.windows[0].selected, Some(0));
    }

    #[test]
    fn the_details_name_the_own_targets_and_only_the_other_sides_system_missions() {
        // FUN_004a10a0: the player's own mission names its order's target
        // (+0x70); another side's mission names the system for families
        // 0x50..0x5f (Espionage 0x52), else nothing: TEXTSTRA 34087
        // "Target Unknown" (Assassination 0x63).
        let (mut world, system, mut missions, [first, ..]) = busy();
        let other = character(&mut world, system, "Killer", false);
        send(
            &mut missions,
            MissionKind::Assassination,
            false,
            &[other],
            &[],
            system,
            Some(first),
        );
        let fog = unseen();
        let rows = mission_rows(&world, &fog, &missions, Faction::Alliance, system);
        let name = |row: &MissionRow| {
            target_name(&world, target(&missions, Faction::Alliance, system, row))
        };
        assert_eq!(
            rows.iter().map(name).collect::<Vec<_>>(),
            ["Sluis Van", "Sluis Van", "Target Unknown"]
        );

        // The player's own Assassination names its target character.
        let own = character(&mut world, system, "Agent", true);
        let enemy = character(&mut world, system, "Vader", false);
        let mut missions = MissionState::new();
        send(
            &mut missions,
            MissionKind::Assassination,
            true,
            &[own],
            &[],
            system,
            Some(enemy),
        );
        let rows = mission_rows(&world, &fog, &missions, Faction::Alliance, system);
        assert_eq!(
            target_name(
                &world,
                target(&missions, Faction::Alliance, system, &rows[0])
            ),
            "Vader"
        );
        assert_eq!(
            target_view(&world, Target::Character(enemy)).map(|(source, id, _)| (source, id)),
            Some((DllSource::Gokres, Some(18_176 + 832)))
        );
    }

    #[test]
    fn an_objects_target_picture_is_its_portrait_a_mini_less_0x4000() {
        // FUN_0042c3b0(.., 1, 1): class +0x30 & 0xfff, against the mini's
        // + 0x4000; GOKRES 1600 is the KDY-150's 122 by 50 portrait.
        let (mut world, _) = world(ControlKind::Uncontrolled);
        let battery =
            world
                .defense_facilities
                .insert(rebellion_core::world::DefenseFacilityInstance {
                    class_dat_id: DatId::new(0x2200_0001),
                    side: rebellion_core::dat::Faction::Empire,
                });
        assert_eq!(
            target_view(
                &world,
                Target::Object(MissionTarget::DefenseFacility(battery))
            ),
            Some((DllSource::Gokres, Some(16_896 - 0x4000), "KDY-150".into()))
        );
    }

    #[test]
    fn a_death_star_target_shows_the_living_hulls_portrait() {
        // FUN_0042c3b0(.., 1, 1) on the hull MissionTarget::DeathStar
        // names: GOKRES 18312 (its mini) less 0x4000; a destroyed hull is
        // no target.
        let (mut world, _) = world(ControlKind::Uncontrolled);
        let mut class = |dat_id: u32, name: &str| {
            world
                .capital_ship_classes
                .insert(rebellion_core::world::CapitalShipClass {
                    dat_id: DatId::new(dat_id),
                    name: name.into(),
                    ..Default::default()
                })
        };
        let destroyer = class(133, "Imperial I Star Destroyer");
        let death_star = class(136, "Death Star");
        let ship = rebellion_core::world::ShipInstance::new;
        let fleet = world.fleets.insert(rebellion_core::world::Fleet {
            location: SystemKey::default(),
            capital_ships: vec![ship(destroyer, 10, false), ship(death_star, 10, false)],
            fighters: Vec::new(),
            characters: Vec::new(),
            is_alliance: false,
            has_death_star: true,
        });
        assert_eq!(
            target_view(&world, Target::Object(MissionTarget::DeathStar(fleet))),
            Some((
                DllSource::Gokres,
                Some(18_312 - 0x4000),
                "Death Star".into()
            ))
        );
        world.fleets[fleet].capital_ships[1].alive = false;
        assert_eq!(
            target_view(&world, Target::Object(MissionTarget::DeathStar(fleet))),
            None
        );
    }

    #[test]
    fn a_member_without_a_mapped_mini_still_lists_its_name() {
        // FUN_004a0e10 adds a row for every member; port: a character the
        // port maps no mini for keeps its name.
        let (mut world, system, missions, [first, ..]) = busy();
        world.characters[first].dat_id = DatId::new(1);
        let rows = member_rows(
            &world,
            &unseen(),
            &missions,
            Faction::Alliance,
            system,
            missions.missions()[0].id,
            MissionsTab::Agents,
        );
        assert_eq!(
            rows.iter()
                .map(|row| (row.mini, row.label.as_str()))
                .collect::<Vec<_>>(),
            [(None, "Leia"), (Some(18_176 + 832), "Mon")]
        );
        assert!(
            rows.iter().all(|row| row.en_route.is_none()),
            "a member at the system is not en route"
        );
    }

    #[test]
    fn a_bitmap_centres_with_integer_offsets_and_a_members_name_sits_under_its_mini() {
        // FUN_0049fef0 and FUN_004a0e10: (box - size) / 2 in integers; the
        // name's top is the mini's height plus its offset (item +0x34).
        assert_eq!(centered((122.0, 50.0), [61, 25]), (30.0, 12.0));
        assert_eq!(centered((122.0, 43.0), [66, 25]), (28.0, 9.0));
        assert_eq!(centered((122.0, 50.0), [122, 50]), (0.0, 0.0));
        assert_eq!(picture_at(Some([61, 25])), (138.0, 49.0));
        assert_eq!(picture_at(None), (108.0, 37.0));
        assert_eq!(member_text_top(25), 34.0);
        assert_eq!(member_text_top(0), 21.0);
    }

    /// The canvas 10 by 20 pixels in and twice the original size.
    fn scaled() -> CockpitLayout {
        let viewport = CockpitViewport {
            x: 10.0,
            y: 20.0,
            width: 1280.0,
            height: 960.0,
        };
        CockpitLayout {
            canvas: viewport,
            galaxy: viewport,
            scale: 2.0,
        }
    }

    const ORIGIN: (i16, i16) = (20, 30);

    /// A logical point in the window opened at `ORIGIN` on [`scaled`].
    fn at(x: f32, y: f32) -> egui::Pos2 {
        egui::pos2(
            10.0 + (f32::from(ORIGIN.0) + x) * 2.0,
            20.0 + (f32::from(ORIGIN.1) + y) * 2.0,
        )
    }

    fn opened(
        world: &GameWorld,
        missions: &MissionState,
        system: SystemKey,
    ) -> MissionsWindowState {
        let mut state = MissionsWindowState::default();
        assert!(state.open(
            world,
            &unseen(),
            missions,
            system,
            ORIGIN,
            CockpitFaction::Alliance,
            scaled()
        ));
        state
    }

    #[test]
    fn the_windows_rects_and_release_target_follow_the_canvas_offset_and_scale() {
        // The mission rows at (5, 24 + 50 n), the tabs at (105, 127) and
        // (166, 127), the member rows at (107, 145 + 43 n) (FUN_0049f540,
        // FUN_00609ae0); +0x70 gives the subject (FUN_004aa470).
        let (world, system, missions, _) = busy();
        let mut state = opened(&world, &missions, system);
        assert!(state.open(
            &world,
            &unseen(),
            &missions,
            system,
            (300, 100),
            CockpitFaction::Alliance,
            scaled()
        ));
        assert_eq!(state.window_count(), 1);
        assert!(state.is_open(system));
        assert_eq!(
            state.mission_row_screen_rect(scaled(), system, 1),
            Some(egui::Rect::from_min_size(
                at(5.0, 74.0),
                egui::vec2(180.0, 100.0)
            ))
        );
        assert_eq!(
            state.tab_screen_rect(scaled(), system, MissionsTab::Decoys),
            Some(egui::Rect::from_min_size(
                at(166.0, 127.0),
                egui::vec2(122.0, 32.0)
            ))
        );
        assert_eq!(
            state.member_row_screen_rect(scaled(), system, 2),
            Some(egui::Rect::from_min_size(
                at(107.0, 231.0),
                egui::vec2(230.0, 86.0)
            ))
        );
        let point = |p: egui::Pos2| (p.x, p.y);
        assert!(state.contains_screen_point(scaled(), point(at(234.0, 303.0))));
        assert!(!state.contains_screen_point(scaled(), point(at(235.5, 150.0))));

        let layer = egui::LayerId::new(egui::Order::Foreground, area_id(system));
        assert_eq!(state.release_target(layer), Some(system));
        let elsewhere = egui::LayerId::new(egui::Order::Foreground, egui::Id::new("elsewhere"));
        assert_eq!(state.release_target(elsewhere), None);

        assert!(state.open(
            &world,
            &unseen(),
            &missions,
            system,
            ORIGIN,
            CockpitFaction::Empire,
            scaled()
        ));
        assert_eq!(state.faction, CockpitFaction::Empire);
        state.clear();
        assert_eq!(state.window_count(), 0);
        assert!(!state.is_open(system));
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Text {
        text: String,
        pos: egui::Pos2,
        size: egui::Vec2,
        rect: egui::Rect,
        font: f32,
        wrap: f32,
    }

    #[derive(Default)]
    struct Run {
        actions: Vec<MissionsWindowAction>,
        texts: Vec<Text>,
        painted: Vec<(u32, egui::Pos2)>,
    }

    /// Draw the windows once per frame of events, on [`scaled`] for the
    /// Alliance; the last frame's text and bitmaps are kept.
    fn run(
        world: &GameWorld,
        missions: &MissionState,
        state: &mut MissionsWindowState,
        frames: Vec<Vec<egui::Event>>,
    ) -> Run {
        let ctx = egui::Context::default();
        let mut cache = BmpCache::new();
        let fog = unseen();
        let mut result = Run::default();
        for (index, events) in frames.into_iter().enumerate() {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1400.0, 1000.0),
                )),
                time: Some(index as f64 * 0.05),
                events,
                ..Default::default()
            };
            PAINTED.with(|painted| painted.borrow_mut().clear());
            let output = ctx.run(input, |ctx| {
                result.actions.extend(draw_missions_windows(
                    ctx,
                    world,
                    &fog,
                    missions,
                    state,
                    CockpitFaction::Alliance,
                    scaled(),
                    &mut cache,
                ));
            });
            result.painted = PAINTED.with(|painted| painted.take());
            result.texts.clear();
            for clipped in output.shapes {
                if let egui::Shape::Text(text) = clipped.shape {
                    let format = &text.galley.job.sections[0].format;
                    result.texts.push(Text {
                        text: text.galley.text().to_owned(),
                        pos: text.pos,
                        size: text.galley.size(),
                        rect: text.galley.rect,
                        font: format.font_id.size,
                        wrap: text.galley.job.wrap.max_width,
                    });
                }
            }
        }
        result
    }

    fn hover(point: egui::Pos2) -> Vec<Vec<egui::Event>> {
        vec![
            vec![egui::Event::PointerMoved(point)],
            vec![egui::Event::PointerMoved(point)],
        ]
    }

    fn press(point: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos: point,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::default(),
        }
    }

    fn click(point: egui::Pos2) -> Vec<Vec<egui::Event>> {
        let mut frames = hover(point);
        frames.extend([vec![press(point, true)], vec![press(point, false)], vec![]]);
        frames
    }

    fn text<'a>(run: &'a Run, text: &str) -> &'a Text {
        run.texts
            .iter()
            .find(|value| value.text == text)
            .unwrap_or_else(|| panic!("no {text:?} in {:?}", run.texts))
    }

    #[test]
    fn the_window_paints_its_chrome_rows_frame_tabs_members_and_target() {
        // FUN_0049fef0: 11165, the focused strip of FUN_004a1f60's side (2:
        // 10201) at (2, 2), the title at (21, 2) 16 high; the buttons;
        // each mission's mini at its row's corner with the side 1 frame
        // 11127 over the selected one, its name from (1, 0) wrapped at 87;
        // the selected mission's tabs (Agents pressed); its agents' minis
        // and centred names under them; "Target:" and the target's name
        // centred over the 113-wide rects at (109, 25) and (109, 91).
        let (world, system, missions, _) = busy();
        let mut state = opened(&world, &missions, system);
        let run = run(&world, &missions, &mut state, hover(at(150.0, 10.0)));

        for (id, point) in [
            (BACKGROUND, at(0.0, 0.0)),
            (10201, at(2.0, 2.0)),
            (SECTOR_NORMAL, at(3.0, 3.0)),
            (MINIMIZE_NORMAL, at(204.0, 3.0)),
            (CLOSE_NORMAL, at(218.0, 3.0)),
            (0x4c10, at(5.0, 24.0)),
            (MISSION_FRAME[0], at(5.0, 24.0)),
            (0x5c13, at(5.0, 74.0)),
            (11561, at(105.0, 127.0)),
            (11562, at(166.0, 127.0)),
            (18_176 + 832, at(107.0, 145.0)),
            (18_176 + 832, at(107.0, 188.0)),
        ] {
            assert!(
                run.painted.contains(&(id, point)),
                "{id} at {point:?} in {:?}",
                run.painted
            );
        }
        assert_eq!(
            run.painted
                .iter()
                .filter(|(id, _)| MISSION_FRAME.contains(id))
                .count(),
            1
        );
        // The target system's planet (FUN_0045c970), whose size the test
        // cache lacks, so it lands at the box's corner.
        assert!(run
            .painted
            .contains(&(planet_resource_id(DatId::new(100)), at(108.0, 37.0))));

        let title = text(&run, "Mission at Sluis Van");
        assert_eq!((title.pos.x, title.font), (at(21.0, 0.0).x, 22.0));
        assert_eq!(title.pos.y + title.size.y / 2.0, at(0.0, 10.0).y);
        let row = text(&run, "Diplomacy");
        assert_eq!((row.pos, row.font, row.wrap), (at(6.0, 24.0), 18.0, 174.0));
        let member = text(&run, "Leia");
        assert_eq!(member.pos.x + member.size.x / 2.0, at(165.5, 0.0).x);
        assert_eq!(member.pos.y, at(0.0, 166.0).y);
        let label = text(&run, "Target:");
        assert_eq!(label.pos.x + label.size.x / 2.0, at(165.5, 0.0).x);
        assert_eq!(label.pos.y, at(0.0, 25.0).y);
        let name = text(&run, "Sluis Van");
        assert_eq!(name.pos.x + name.rect.center().x, at(165.5, 0.0).x);
        assert_eq!(name.pos.y, at(0.0, 91.0).y);
        assert_eq!(name.wrap, 226.0);
    }

    #[test]
    fn a_click_on_another_mission_selects_it_and_a_tab_click_switches_the_members() {
        // FUN_004a0c60 refreshes for a different mission and FUN_004a0ca0
        // reselects the Agents tab; id 0x16 refills for the clicked tab.
        let (world, system, missions, _) = busy();
        let mut state = opened(&world, &missions, system);

        let decoys = run(&world, &missions, &mut state, click(at(190.0, 135.0)));
        assert_eq!(state.windows[0].tab, MissionsTab::Decoys);
        assert!(decoys.painted.contains(&(11563, at(166.0, 127.0))));
        assert_eq!(decoys.actions, [MissionsWindowAction::SelectSystem(system)]);

        let other = run(&world, &missions, &mut state, click(at(40.0, 90.0)));
        let espionage = missions.missions()[1].id;
        assert_eq!(state.windows[0].selected, Some(espionage));
        assert_eq!(state.windows[0].tab, MissionsTab::Agents);
        assert!(other.painted.contains(&(MISSION_FRAME[0], at(5.0, 74.0))));
        assert!(other.painted.contains(&(11566, at(105.0, 127.0))));
        assert!(other.texts.iter().any(|value| value.text == "Spy"));

        // The same mission keeps the current tab.
        state.windows[0].tab = MissionsTab::Decoys;
        let _ = run(&world, &missions, &mut state, click(at(40.0, 90.0)));
        assert_eq!(state.windows[0].tab, MissionsTab::Decoys);
    }

    #[test]
    fn a_selected_mission_that_ends_clears_the_selection_and_hides_the_details() {
        // FUN_004a0c20 clears +0x1c4 when its row goes; FUN_004a0ca0 then
        // hides the strip and the member list, FUN_004a10a0 the texts.
        let (world, system, mut missions, _) = busy();
        let mut state = opened(&world, &missions, system);
        let first = missions.missions()[0].id;
        assert!(missions.cancel(first).is_some());

        let run = run(&world, &missions, &mut state, hover(at(150.0, 10.0)));
        assert_eq!(state.windows[0].selected, None);
        assert!(!run
            .painted
            .iter()
            .any(|(id, _)| (11560..=11569).contains(id) || MISSION_FRAME.contains(id)));
        assert!(!run.texts.iter().any(|value| value.text == "Target:"));
        assert!(run.painted.contains(&(0x5c13, at(5.0, 24.0))));
    }

    #[test]
    fn the_title_buttons_open_the_sector_minimize_and_close() {
        // 0xca opens the sector window (FUN_00429ce0); 0x15 posts 0x466;
        // 0x14 closes.
        let (world, system, missions, _) = busy();
        let mut state = opened(&world, &missions, system);
        let sector = run(&world, &missions, &mut state, click(at(9.0, 9.0)));
        assert_eq!(
            sector.actions,
            [
                MissionsWindowAction::OpenSector(system),
                MissionsWindowAction::SelectSystem(system)
            ]
        );

        let minimized = run(&world, &missions, &mut state, click(at(210.0, 9.0)));
        assert_eq!(
            minimized.actions,
            [MissionsWindowAction::Minimize {
                system,
                logical_position: ORIGIN
            }]
        );
        assert_eq!(state.window_count(), 0);

        let mut state = opened(&world, &missions, system);
        let closed = run(&world, &missions, &mut state, click(at(224.0, 9.0)));
        assert!(closed.actions.is_empty());
        assert_eq!(state.window_count(), 0);
    }

    #[test]
    fn a_title_button_shows_its_pressed_art_only_while_the_button_is_held_over_it() {
        let (world, system, missions, _) = busy();
        let mut state = opened(&world, &missions, system);
        let hovered = run(&world, &missions, &mut state, hover(at(210.0, 9.0)));
        assert!(hovered.painted.contains(&(MINIMIZE_NORMAL, at(204.0, 3.0))));

        let mut frames = hover(at(224.0, 9.0));
        frames.push(vec![press(at(224.0, 9.0), true)]);
        let held = run(&world, &missions, &mut state, frames);
        assert!(held.painted.contains(&(CLOSE_PRESSED, at(218.0, 3.0))));
        assert!(held.painted.contains(&(MINIMIZE_NORMAL, at(204.0, 3.0))));
        assert_eq!(state.window_count(), 1);
    }
}
