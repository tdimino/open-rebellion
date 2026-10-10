//! The agent's advice topics (`ghidra/notes/droid-advisor-triggers.md`,
//! "Advice topics"): TEXTSTRA's topic table, held until the game releases
//! each topic into the Message Index's Advice category.

use crate::advisor::AdvisorFaction;
use crate::message_log::MessageDisplay;

/// A topic the agent holds (`FUN_0048b460`): its table index, its kind (the
/// low nibble of the entry's first byte) and the order it sorts by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Topic {
    index: u16,
    kind: u8,
    order: u16,
}

/// Kind 7 topics file when the briefing ends (`FUN_00439f20`).
const OPENING_KIND: u8 = 7;

/// `FUN_00439bc0`: the periodic release waits this many steps after the last.
const RELEASE_STEPS: u32 = 300;

/// A window the player opened that releases topics (`FUN_0043a0b0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopicWindow {
    /// A new sector window (`FUN_00429ce0`, bit 1).
    Sector,
    /// The System window, type 9 (`FUN_0045aac0`, bit 2).
    System,
    /// The Fleet window, type 4 (bit 4).
    Fleet,
    /// System Defenses, type 10 (bit 8).
    SystemDefenses,
    /// The Missions window, type 11 (bit `0x10`).
    Missions,
}

impl TopicWindow {
    /// The agent's `+0x150` bit and the topic kind it releases.
    const fn bit_and_kind(self) -> (u8, u8) {
        match self {
            Self::Sector => (1, 6),
            Self::System => (2, 2),
            Self::Fleet => (4, 3),
            Self::SystemDefenses => (8, 5),
            Self::Missions => (0x10, 4),
        }
    }
}

/// The agent's held topics (`+0x154`), released-kind bits (`+0x150`) and
/// last release step (`+0x148`). None of it is saved (`FUN_004397a0`), so a
/// loaded game builds the list again on its first release.
#[derive(Debug, Clone, Default)]
pub struct AdviceTopics {
    faction: Option<AdvisorFaction>,
    table: Vec<Topic>,
    /// `+0x150` bit `0x10000000`: the list was built (`FUN_00439f20`).
    built: bool,
    held: Vec<Topic>,
    opened: u8,
    last_release: u32,
}

/// A topic to file: its display, from TEXTSTRA RCDATA (`FUN_0048b2e0`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopicMessage {
    pub display: MessageDisplay,
}

impl AdviceTopics {
    /// Read `faction`'s table: RCDATA `0x6000` (Alliance) or `0x6800`
    /// (Empire) holds the count, and entry `i` sits at `base + 3i`
    /// (`FUN_0048b460`). `rcdata` reads one RCDATA entry.
    pub fn new<'a>(faction: AdvisorFaction, rcdata: impl Fn(u16) -> Option<&'a [u8]>) -> Self {
        let base = table_base(faction);
        let count = rcdata(base)
            .and_then(|bytes| bytes.get(..2))
            .map_or(0, |word| u16::from_le_bytes([word[0], word[1]]));
        let mut held: Vec<Topic> = (1..=count)
            .filter_map(|index| {
                let entry = rcdata(base.wrapping_add(index * 3))?;
                Some(Topic {
                    index,
                    kind: entry.first()? & 0xf,
                    order: u16::from_le_bytes([*entry.get(2)?, *entry.get(3)?]),
                })
            })
            .collect();
        // FUN_005f5440 keeps the list ordered by the entry's id.
        held.sort_by_key(|topic| topic.order);
        Self {
            faction: Some(faction),
            table: held,
            built: false,
            held: Vec::new(),
            opened: 0,
            last_release: 0,
        }
    }

    /// Build the held list once (`FUN_00439f20`) and file every kind 7
    /// topic: the briefing's step 13 calls it with advice on, and the other
    /// releases call it first.
    pub fn release_opening<'a>(
        &mut self,
        rcdata: impl Fn(u16) -> Option<&'a [u8]>,
    ) -> Vec<TopicMessage> {
        if self.built {
            return Vec::new();
        }
        self.built = true;
        let (opening, rest): (Vec<Topic>, Vec<Topic>) = self
            .table
            .iter()
            .partition(|topic| topic.kind == OPENING_KIND);
        self.held = rest;
        opening
            .into_iter()
            .filter_map(|topic| self.message(topic, &rcdata))
            .collect()
    }

    /// The player opened `window` with advice on (`FUN_0043a0b0`): the first
    /// time, file the first held topic of its kind.
    pub fn window_opened<'a>(
        &mut self,
        window: TopicWindow,
        now: u32,
        rcdata: impl Fn(u16) -> Option<&'a [u8]>,
    ) -> Vec<TopicMessage> {
        let mut filed = self.release_opening(&rcdata);
        let (bit, kind) = window.bit_and_kind();
        if self.opened & bit != 0 {
            return filed;
        }
        self.opened |= bit;
        if let Some(position) = self.held.iter().position(|topic| topic.kind == kind) {
            let topic = self.held.remove(position);
            self.last_release = now;
            filed.extend(self.message(topic, &rcdata));
        }
        filed
    }

    /// The side's update with advice on (`FUN_00439bc0`): 300 steps after the
    /// last release, file the first held topic whose kind is open
    /// (`FUN_00439fb0`).
    /// The held topics' indices, in release order: for the developer
    /// command `List advice topics`.
    #[must_use]
    pub fn held_topics(&self) -> Vec<u16> {
        self.held.iter().map(|topic| topic.index).collect()
    }

    /// Release held topic `index` now, out of turn: for the developer
    /// command `Release advice topic`. Refused before the opening topics are
    /// filed, and for a topic not held (unknown, or already filed).
    pub fn release_topic<'a>(
        &mut self,
        index: u16,
        now: u32,
        rcdata: impl Fn(u16) -> Option<&'a [u8]>,
    ) -> Result<TopicMessage, String> {
        if !self.built {
            return Err("the opening topics are not filed yet".into());
        }
        let position = self
            .held
            .iter()
            .position(|topic| topic.index == index)
            .ok_or_else(|| format!("topic {index} is not held"))?;
        let topic = self.held.remove(position);
        self.last_release = now;
        self.message(topic, &rcdata)
            .ok_or_else(|| format!("topic {index} has no text"))
    }

    pub fn tick<'a>(
        &mut self,
        now: u32,
        rcdata: impl Fn(u16) -> Option<&'a [u8]>,
    ) -> Vec<TopicMessage> {
        if self.last_release.saturating_add(RELEASE_STEPS) > now {
            return Vec::new();
        }
        let mut filed = self.release_opening(&rcdata);
        self.last_release = now;
        let opened = self.opened;
        if let Some(position) = self
            .held
            .iter()
            .position(|topic| kind_open(topic.kind, opened))
        {
            let topic = self.held.remove(position);
            filed.extend(self.message(topic, &rcdata));
        }
        filed
    }

    /// `FUN_0048b2e0`: title RCDATA `0x6001 + 3i` and body `0x6002 + 3i`
    /// (`0x6801`/`0x6802` for the Empire), the advice background 0x42f /
    /// 0x430 and sound 0x461 / 0x462.
    fn message<'a>(
        &self,
        topic: Topic,
        rcdata: &impl Fn(u16) -> Option<&'a [u8]>,
    ) -> Option<TopicMessage> {
        let faction = self.faction?;
        let base = table_base(faction);
        let title = rcdata(base.wrapping_add(topic.index * 3 + 1))?;
        let body = rcdata(base.wrapping_add(topic.index * 3 + 2))?;
        let (background, sound) = match faction {
            AdvisorFaction::Alliance => (0x42f, 0x461),
            AdvisorFaction::Empire => (0x430, 0x462),
        };
        Some(TopicMessage {
            display: MessageDisplay {
                title: plain_text(title),
                body: plain_text(body),
                background,
                overlay: 0,
                sound,
            },
        })
    }
}

/// The table's RCDATA base: `0x6000`, `0x6800` for the Empire.
const fn table_base(faction: AdvisorFaction) -> u16 {
    match faction {
        AdvisorFaction::Alliance => 0x6000,
        AdvisorFaction::Empire => 0x6800,
    }
}

/// `FUN_00439fb0`: kinds 2..6 wait for their window's bit; the rest are
/// always open.
const fn kind_open(kind: u8, opened: u8) -> bool {
    match kind {
        2 => opened & 2 != 0,
        3 => opened & 4 != 0,
        4 => opened & 0x10 != 0,
        5 => opened & 8 != 0,
        6 => opened & 1 != 0,
        _ => true,
    }
}

/// A topic's text without its end mark (`0x01`); the topics carry no
/// argument markers.
fn plain_text(bytes: &[u8]) -> String {
    bytes
        .iter()
        .take_while(|&&byte| byte != 1)
        .map(|&byte| char::from(byte))
        .collect()
}
