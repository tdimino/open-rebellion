//! The interface a save carries beside the simulation: the galaxy view's
//! GID display mode and its open child windows.
//!
//! The original writes it after the world and the controllers
//! (`FUN_004095b0` → `FUN_00401250` → `FUN_00415f60`, the strategic view's
//! slot `+0x3c`, `FUN_00440ea0`) and reads it back after rebuilding the view
//! (`FUN_00409480` → `FUN_00401270` → `FUN_00415f80`, slot `+0x40`,
//! `FUN_00440f80`). These are plain records: the renderer builds them from
//! its window states and restores the windows from them, and the save keeps
//! them outside the state fingerprint.

use serde::{Deserialize, Serialize};

use crate::ids::DatId;

/// The galaxy view's interface record (`FUN_00440ea0`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedInterface {
    /// The GID display mode's command byte, the low byte of the record's
    /// first word (view `+0xc8`, which `FUN_00425d00` sets,
    /// `FUN_00425d00.c:126`); `None` for a save made with no galaxy view.
    pub gid_mode: Option<u8>,
    /// The view's child windows, the bottom of the z-order first: the
    /// writer starts at the bottom window (`FUN_00440f20`) and walks up
    /// (`GetWindow` `GW_HWNDPREV`), and the loader creates them in that
    /// order, each above the last.
    pub windows: Vec<SavedWindow>,
}

/// One child window: its descriptor (`FUN_0049ecf0`) and its class payload
/// (slot `+0x24`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedWindow {
    pub kind: SavedWindowKind,
    /// The window's top-left on the 640x480 canvas (descriptor `+0x28`,
    /// `+0x2c`). Under Wine a dragged System window reopens where it was
    /// dragged (Empire day-2 start, played on to day 6).
    pub position: (i16, i16),
    /// Shown, or hidden on the minimized rail (`+0x110`: `FUN_00607d00(5)`
    /// sets it, `FUN_00607d30` clears it).
    pub shown: bool,
}

/// The window types `FUN_00441190` rebuilds, each with its class payload.
/// The "Battle in Progress..." overlay (type 7, `FUN_0049ee20`) is left
/// out: the port has no such window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SavedWindowKind {
    /// Type 1: the sector (`FUN_0045d2b0`, its subject `+0x144`).
    Sector { sector: DatId },
    /// Type 9: the system (`FUN_00458ea0`). The page is not saved.
    System { system: DatId },
    /// Type 4: the system and the tab's control id, `0x66..0x69`
    /// (`FUN_004a7300`, `+0x194`).
    Fleet { system: DatId, tab: u32 },
    /// Type 10: the system (`FUN_004aa6d0`). The page is not saved.
    Defenses { system: DatId },
    /// Type 11: the system (`FUN_004a0b00`). The tab is not saved.
    Missions { system: DatId },
}
