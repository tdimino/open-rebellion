//! Shared fixtures for app-layer unit tests.

use rebellion_core::dat::Faction;
use rebellion_core::ids::SystemKey;
use rebellion_core::world::{ControlKind, GameWorld, System};
use rebellion_data::encyclopedia_session::EncyclopediaResourceBytes;

/// A one-system world for order-message tests, with the original fixture name.
pub(crate) fn order_world(holder: Option<Faction>, is_populated: bool) -> (GameWorld, SystemKey) {
    let mut world = GameWorld::default();
    let system = world.systems.insert(System {
        name: "Selonia".into(),
        is_populated,
        control: holder.map_or(ControlKind::Uncontrolled, ControlKind::Controlled),
        ..System::default()
    });
    (world, system)
}

/// The four canonical Encyclopedia resource names filled with one indexed
/// 400x200 fixture image.
pub(crate) fn uniform_encyclopedia_artwork(pixel: [u8; 3]) -> EncyclopediaResourceBytes {
    let image: std::sync::Arc<[u8]> =
        rebellion_data::test_support::indexed_encyclopedia_bmp(pixel).into();
    ["EDATA.001", "EDATA.014", "EDATA.015", "EDATA.115"]
        .into_iter()
        .map(|name| (name.into(), image.clone()))
        .collect()
}
