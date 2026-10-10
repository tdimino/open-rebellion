//! The original scroll bar ("Scroll Bar", `FUN_0060aa70`), which
//! `FUN_0060a490` attaches to a list or a text field from an RCDATA of
//! STRATEGY bitmaps. Recovery notes: `ghidra/notes/battle-alert-lists.md`,
//! "The scroll bar".
//!
//! The bar is a child of its owner, laid out by `FUN_0060fa80`: as wide as
//! the RCDATA's first word, flush with the owner's right edge and as tall as
//! the owner. The 13-pixel arrows, track tiles and thumb pieces centre on
//! it. The arrows step one unit (`FUN_0060f6d0` ids 100 and 101: a list
//! row, or a text field's line), the track pages, and the thumb
//! (`FUN_0060fb80`) is as long as the visible share of the track, raised to
//! 12 pixels when under 13.
//!
//! The module is only geometry and input; each window paints the
//! [`ScrollBlit`]s with its own painter.

/// One RCDATA's art: the bar's width, then the track tile, the arrows
/// (normal, pressed) and the thumb's top, tiled middle and bottom.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ScrollBarArt {
    pub width: u16,
    pub track: u32,
    pub up: [u32; 2],
    pub down: [u32; 2],
    pub thumb: [u32; 3],
}

/// RCDATA `0x29fc`: the Battle Alert's list (`FUN_0044f860`).
pub(crate) const LIST_BAR_29FC: ScrollBarArt = ScrollBarArt {
    width: 0x0a,
    track: 0x29fd,
    up: [0x29fe, 0x29ff],
    down: [0x2a00, 0x2a01],
    thumb: [0x2a02, 0x2a03, 0x2a04],
};

/// RCDATA `0x299d`: the Message Index's list and its message text field
/// (`FUN_004665f0`).
pub(crate) const TEXT_BAR_299D: ScrollBarArt = ScrollBarArt {
    width: 0x0c,
    track: 0x29a0,
    up: [0x29bd, 0x29be],
    down: [0x29bf, 0x29c0],
    thumb: [0x29c1, 0x29c2, 0x29c3],
};

/// Every piece is 13 pixels wide; the arrows are 9 high, the track tile 13,
/// the thumb's caps 6 and its middle tile 12.
const PIECE_WIDTH: f32 = 13.0;
const ARROW_HEIGHT: f32 = 9.0;
const TRACK_TILE: f32 = 13.0;
const THUMB_CAP: f32 = 6.0;
const THUMB_TILE: f32 = 12.0;

/// A part of the bar under the pointer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ScrollPart {
    Up,
    Down,
    PageUp,
    PageDown,
    Thumb,
}

/// The bar's press state across frames: the part pressed, and for the thumb
/// where it was grabbed.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct ScrollDrag {
    pressed: Option<ScrollPart>,
    grab: f32,
}

/// One bitmap to draw, with its height clipped (tiles cut at the end).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ScrollBlit {
    pub resource: u32,
    pub x: f32,
    pub y: f32,
    pub height: f32,
}

/// The pointer this frame, in the bar's logical coordinates.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct ScrollPointer {
    pub at: Option<(f32, f32)>,
    pub pressed: bool,
    pub down: bool,
    pub released: bool,
}

/// What the bar shows: the first visible unit, how many units fit and how
/// many there are.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ScrollRange {
    pub first: usize,
    pub visible: f32,
    pub total: usize,
}

impl ScrollRange {
    /// The furthest the owner scrolls: until its last unit shows whole.
    pub fn max_first(self) -> usize {
        self.total
            .saturating_sub(self.visible.floor().max(1.0) as usize)
    }

    /// The bar shows only while the units overflow (`FUN_0060b5d0` hides it
    /// when everything is in view).
    pub fn overflows(self) -> bool {
        self.max_first() > 0
    }
}

/// A bar laid out against its owner.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ScrollBar {
    art: ScrollBarArt,
    piece_x: f32,
    top: f32,
    height: f32,
}

impl ScrollBar {
    /// `FUN_0060fa80`: the bar `art.width` wide at the owner's right edge;
    /// the pieces centre on it (`width / 2 - 13 / 2`, in integers).
    pub fn new(
        art: ScrollBarArt,
        owner_x: f32,
        owner_y: f32,
        owner_width: f32,
        owner_height: f32,
    ) -> Self {
        let bar_x = owner_x + owner_width - f32::from(art.width);
        Self {
            art,
            piece_x: bar_x + f32::from(art.width / 2) - (PIECE_WIDTH as u16 / 2) as f32,
            top: owner_y,
            height: owner_height,
        }
    }

    /// The bar's own width, which `FUN_0041fd00` takes off a text field's
    /// wrap width while the bar shows.
    pub fn width(self) -> f32 {
        f32::from(self.art.width)
    }

    fn track(self) -> (f32, f32) {
        (
            self.top + ARROW_HEIGHT,
            self.top + self.height - ARROW_HEIGHT,
        )
    }

    /// `FUN_0060fb80`: the thumb's top and bottom.
    fn thumb(self, range: ScrollRange) -> (f32, f32) {
        let (track_top, track_bottom) = self.track();
        let track = track_bottom - track_top;
        let total = range.total.max(1) as f32;
        let mut length = (range.visible.min(total) * track / total).floor();
        if length < 13.0 {
            length = 12.0;
        }
        let travel = range.total as f32 - range.visible;
        let top = if travel > 0.0 {
            track_top + ((track - length) * range.first as f32 / travel).floor()
        } else {
            track_top
        };
        let top = top.clamp(track_top, (track_bottom - length).max(track_top));
        (top, (top + length).min(track_bottom))
    }

    fn part_at(self, range: ScrollRange, (x, y): (f32, f32)) -> Option<ScrollPart> {
        if x < self.piece_x || x >= self.piece_x + PIECE_WIDTH || y < self.top {
            return None;
        }
        let (track_top, track_bottom) = self.track();
        let (thumb_top, thumb_bottom) = self.thumb(range);
        Some(if y < track_top {
            ScrollPart::Up
        } else if y < thumb_top {
            ScrollPart::PageUp
        } else if y < thumb_bottom {
            ScrollPart::Thumb
        } else if y < track_bottom {
            ScrollPart::PageDown
        } else if y < self.top + self.height {
            ScrollPart::Down
        } else {
            return None;
        })
    }

    /// Apply this frame's pointer to `range.first` and return the new first
    /// unit. The arrows step on release over them, as buttons do; the track
    /// pages by `page` units on the press; the thumb follows the drag.
    pub fn update(
        self,
        drag: &mut ScrollDrag,
        range: ScrollRange,
        page: usize,
        pointer: ScrollPointer,
    ) -> usize {
        let max = range.max_first();
        let mut first = range.first.min(max);
        if !range.overflows() {
            *drag = ScrollDrag::default();
            return 0;
        }
        let range = ScrollRange { first, ..range };
        let over = pointer.at.and_then(|at| self.part_at(range, at));
        if pointer.pressed {
            drag.pressed = over;
            match over {
                Some(ScrollPart::PageUp) => first = first.saturating_sub(page.max(1)),
                Some(ScrollPart::PageDown) => first = (first + page.max(1)).min(max),
                Some(ScrollPart::Thumb) => {
                    let (thumb_top, _) = self.thumb(range);
                    drag.grab = pointer.at.map_or(0.0, |(_, y)| y - thumb_top);
                }
                _ => {}
            }
        } else if pointer.down && drag.pressed == Some(ScrollPart::Thumb) {
            if let Some((_, y)) = pointer.at {
                let (track_top, track_bottom) = self.track();
                let (thumb_top, thumb_bottom) = self.thumb(range);
                let travel = track_bottom - track_top - (thumb_bottom - thumb_top);
                if travel > 0.0 {
                    let share = ((y - drag.grab - track_top) / travel).clamp(0.0, 1.0);
                    first = (share * max as f32).round() as usize;
                }
            }
        }
        if pointer.released {
            match (drag.pressed.take(), over) {
                (Some(ScrollPart::Up), Some(ScrollPart::Up)) => first = first.saturating_sub(1),
                (Some(ScrollPart::Down), Some(ScrollPart::Down)) => first = (first + 1).min(max),
                _ => {}
            }
        }
        first
    }

    /// The bitmaps to draw for `range`, top to bottom: track tiles, thumb,
    /// then the arrows (pressed while held over them).
    pub fn blits(
        self,
        range: ScrollRange,
        drag: &ScrollDrag,
        pointer: ScrollPointer,
    ) -> Vec<ScrollBlit> {
        let mut blits = Vec::new();
        if !range.overflows() {
            return blits;
        }
        let range = ScrollRange {
            first: range.first.min(range.max_first()),
            ..range
        };
        let (track_top, track_bottom) = self.track();
        let mut y = track_top;
        while y < track_bottom {
            blits.push(ScrollBlit {
                resource: self.art.track,
                x: self.piece_x,
                y,
                height: (track_bottom - y).min(TRACK_TILE),
            });
            y += TRACK_TILE;
        }
        let (thumb_top, thumb_bottom) = self.thumb(range);
        let [cap_top, middle, cap_bottom] = self.art.thumb;
        blits.push(ScrollBlit {
            resource: cap_top,
            x: self.piece_x,
            y: thumb_top,
            height: THUMB_CAP,
        });
        let mut y = thumb_top + THUMB_CAP;
        let middle_end = thumb_bottom - THUMB_CAP;
        while y < middle_end {
            blits.push(ScrollBlit {
                resource: middle,
                x: self.piece_x,
                y,
                height: (middle_end - y).min(THUMB_TILE),
            });
            y += THUMB_TILE;
        }
        blits.push(ScrollBlit {
            resource: cap_bottom,
            x: self.piece_x,
            y: middle_end,
            height: THUMB_CAP,
        });
        let over = pointer.at.and_then(|at| self.part_at(range, at));
        let held = |part| pointer.down && drag.pressed == Some(part) && over == Some(part);
        blits.push(ScrollBlit {
            resource: self.art.up[usize::from(held(ScrollPart::Up))],
            x: self.piece_x,
            y: self.top,
            height: ARROW_HEIGHT,
        });
        blits.push(ScrollBlit {
            resource: self.art.down[usize::from(held(ScrollPart::Down))],
            x: self.piece_x,
            y: track_bottom,
            height: ARROW_HEIGHT,
        });
        blits
    }
}
