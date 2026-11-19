//! Shared fixtures for app-layer unit tests.

use rebellion_core::dat::Faction;
use rebellion_core::ids::SystemKey;
use rebellion_core::world::{ControlKind, GameWorld, System};
use rebellion_data::encyclopedia_session::EncyclopediaResourceBytes;
use rebellion_render::message_log::{GameMessage, MessageRail, MessageTarget, RailAudience};

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

/// Assert the shared two-copy filing contract for a conflict initiated by
/// one side against the other side's system.
pub(crate) fn assert_two_side_conflict_filing(
    messages: &[GameMessage],
    actor: Faction,
    system: SystemKey,
    other_side_advice: u8,
) {
    assert_eq!(messages.len(), 2, "{actor:?}");
    for message in messages {
        assert_eq!(message.rail, Some(MessageRail::Conflict));
        assert_eq!(message.target, Some(MessageTarget::System(system)));
    }
    let actor_is_alliance = actor == Faction::Alliance;
    assert_eq!(messages[0].audience, RailAudience::side(actor_is_alliance));
    assert_eq!(messages[0].advice, None);
    assert_eq!(messages[1].audience, RailAudience::side(!actor_is_alliance));
    assert_eq!(messages[1].advice, Some(other_side_advice));
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
