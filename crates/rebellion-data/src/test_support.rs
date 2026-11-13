//! Shared, feature-gated encyclopedia fixtures for data and app tests.

use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use rebellion_core::ai::AiFaction;
use rebellion_core::dat::Faction;
use rebellion_core::fog::FogState;
use rebellion_core::ids::SystemKey;
use rebellion_core::tuning::GameConfig;
use rebellion_core::victory::VictoryState;
use rebellion_core::world::GameWorld;
use std::sync::Arc;

use crate::encyclopedia_catalog::{
    EncyclopediaCatalog, EncyclopediaCatalogEntry, EncyclopediaCategory,
};
use crate::encyclopedia_session::EncyclopediaResourceBytes;
use crate::save::SaveState;
use crate::simulation::SimulationStates;
use crate::text_templates::TextTemplates;

/// Build a complete save snapshot around an already-populated test world.
#[must_use]
pub fn save_state(
    world: GameWorld,
    alliance_hq: SystemKey,
    empire_hq: SystemKey,
    seed: u64,
    player_is_alliance: bool,
) -> SaveState {
    let player_faction = Faction::of_alliance(player_is_alliance);
    let states = SimulationStates::new(
        AiFaction::Empire,
        player_faction,
        VictoryState::new(alliance_hq, empire_hq),
    );
    SaveState::from_simulation(
        world,
        states,
        FogState::new(Faction::of_alliance(!player_is_alliance)),
        player_is_alliance,
        Xoshiro256PlusPlus::seed_from_u64(seed),
        GameConfig::default(),
        rebellion_core::agent_automation::PlayerAgent::default(),
    )
}

/// Encode a readable TEXTSTRA test template into its RCDATA byte-string form.
///
/// Placeholders use `{argument:kind}`, matching
/// [`crate::text_templates::TextTemplates`].
#[must_use]
pub fn textstra_rcdata(text: &str) -> String {
    let mut encoded = String::new();
    let mut remaining = text;
    while let Some(open) = remaining.find('{') {
        encoded.push_str(&remaining[..open]);
        let close = remaining[open..].find('}').unwrap() + open;
        let (argument, kind) = remaining[open + 1..close].split_once(':').unwrap();
        encoded.push('|');
        encoded.push(char::from(argument.parse::<u8>().unwrap()));
        encoded.push(char::from(kind.parse::<u8>().unwrap()));
        encoded.push_str("\0\0\0");
        remaining = &remaining[close + 1..];
    }
    encoded.push_str(remaining);
    encoded.push('\u{1}');
    encoded
}

/// Build a test template table from readable TEXTSTRA placeholder strings.
pub fn text_templates<'a>(entries: impl IntoIterator<Item = (u16, &'a str)>) -> TextTemplates {
    let entries = entries
        .into_iter()
        .map(|(id, text)| (id, textstra_rcdata(text)))
        .collect::<std::collections::HashMap<_, _>>();
    TextTemplates::from_json(&serde_json::to_vec(&entries).unwrap()).unwrap()
}

/// TEXTSTRA entries shared by assault reports and strike-derived outcomes.
#[must_use]
pub const fn assault_report_template_entries() -> [(u16, &'static str); 5] {
    [
        (0x7090, "Assault on {1:1}"),
        (
            0x7091,
            "{1:4} troops have taken control of the {2:4} system {3:1}",
        ),
        (
            0x7092,
            "{2:4} Troops have defended {3:1} from an {1:4} assault.",
        ),
        (
            0x7093,
            "{1:4} troops have seized control of the neutral system {2:1}.",
        ),
        (
            0x7094,
            "The neutral system {2:1} has repulsed an attack by {1:4} troops.",
        ),
    ]
}

pub fn encyclopedia_entry(
    object_id: u32,
    text_resource_id: u16,
    name: impl Into<String>,
) -> EncyclopediaCatalogEntry {
    EncyclopediaCatalogEntry {
        object_id,
        text_resource_id,
        name: name.into(),
    }
}

#[must_use]
pub fn encyclopedia_category(
    command_id: u16,
    label_resource_id: u16,
    family_range: Option<std::ops::Range<u8>>,
) -> EncyclopediaCategory {
    EncyclopediaCategory::new(
        command_id,
        label_resource_id,
        format!("Category {command_id}"),
        family_range,
    )
}

/// Canonical seven-category catalog shell with caller-selected entries.
#[must_use]
pub fn encyclopedia_catalog(entries: Vec<EncyclopediaCatalogEntry>) -> EncyclopediaCatalog {
    EncyclopediaCatalog {
        title: "Galactic Encyclopedia".into(),
        topic_label: "Topic".into(),
        categories: [
            encyclopedia_category(0x6f, 0x1850, None),
            encyclopedia_category(0x70, 0x1855, Some(0x90..0x98)),
            encyclopedia_category(0x71, 0x1854, Some(0x14..0x20)),
            encyclopedia_category(0x72, 0x1852, Some(0x20..0x30)),
            encyclopedia_category(0x73, 0x1851, Some(0x40..0x80)),
            encyclopedia_category(0x74, 0x1856, Some(0x10..0x14)),
            encyclopedia_category(0x75, 0x1853, Some(0x30..0x40)),
        ],
        entries,
    }
}

/// Representative system, concept, and capital-ship topics shared by
/// Encyclopedia presenter and session tests.
#[must_use]
pub fn representative_encyclopedia_catalog() -> EncyclopediaCatalog {
    encyclopedia_catalog(vec![
        encyclopedia_entry(0x9200_0064, 0x2e00, "Abregado-rae"),
        encyclopedia_entry(0x5100_0010, 0x2c50, "Diplomacy"),
        encyclopedia_entry(0x1400_0040, 0x2740, "Mon Calamari Cruiser"),
    ])
}

/// A 400x200, 8-bit indexed BMP whose palette index 1 has `pixel` RGB and
/// fills the image. This matches the encyclopedia's native artwork geometry.
#[must_use]
pub fn indexed_encyclopedia_bmp(pixel: [u8; 3]) -> Vec<u8> {
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 200;
    const PIXEL_OFFSET: u32 = 14 + 40 + 1024;
    let pixel_bytes = WIDTH * HEIGHT;
    let file_bytes = PIXEL_OFFSET + pixel_bytes;
    let mut bytes = encyclopedia_bmp_header(PIXEL_OFFSET, pixel_bytes, 8, 256);
    bytes.resize(PIXEL_OFFSET as usize, 0);
    bytes[58..62].copy_from_slice(&[pixel[2], pixel[1], pixel[0], 0]);
    bytes.resize(file_bytes as usize, 1);
    bytes
}

/// The four indexed Encyclopedia resources shared by session/presenter tests.
#[must_use]
pub fn indexed_encyclopedia_artwork() -> EncyclopediaResourceBytes {
    [
        ("EDATA.001", [1, 2, 3]),
        ("EDATA.014", [14, 15, 16]),
        ("EDATA.015", [15, 16, 17]),
        ("EDATA.115", [115, 116, 117]),
    ]
    .into_iter()
    .map(|(name, pixel)| (name.into(), Arc::from(indexed_encyclopedia_bmp(pixel))))
    .collect()
}

/// A 400x200, 24-bit RGB BMP filled with `pixel`.
#[must_use]
pub fn rgb_encyclopedia_bmp(pixel: [u8; 3]) -> Vec<u8> {
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 200;
    const PIXEL_BYTES: u32 = WIDTH * HEIGHT * 3;
    let mut bytes = encyclopedia_bmp_header(54, PIXEL_BYTES, 24, 0);
    for _ in 0..WIDTH * HEIGHT {
        bytes.extend_from_slice(&[pixel[2], pixel[1], pixel[0]]);
    }
    bytes
}

fn encyclopedia_bmp_header(pixel_offset: u32, pixel_bytes: u32, bits: u16, colors: u32) -> Vec<u8> {
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 200;
    let file_bytes = pixel_offset + pixel_bytes;
    let mut bytes = Vec::with_capacity(file_bytes as usize);
    bytes.extend_from_slice(b"BM");
    bytes.extend_from_slice(&file_bytes.to_le_bytes());
    bytes.extend_from_slice(&[0; 4]);
    bytes.extend_from_slice(&pixel_offset.to_le_bytes());
    bytes.extend_from_slice(&40_u32.to_le_bytes());
    bytes.extend_from_slice(&(WIDTH as i32).to_le_bytes());
    bytes.extend_from_slice(&(HEIGHT as i32).to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&bits.to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(&pixel_bytes.to_le_bytes());
    bytes.extend_from_slice(&[0; 8]);
    bytes.extend_from_slice(&colors.to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes
}
