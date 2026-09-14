//! State transitions for command-center destinations.

use rebellion_render::{CockpitButton, CockpitState, EncyclopediaState, SaveLoadPanelState};

use crate::GameMode;

/// Open the campaign options destination using the existing slot picker.
pub(crate) fn open_game_options(
    cockpit: &mut CockpitState,
    saves: &mut SaveLoadPanelState,
    mode: &mut GameMode,
) {
    cockpit.gid_ui.menu_open = false;
    cockpit.gid_ui.category = None;
    saves.open_save();
    *mode = GameMode::GameOptions;
}

/// Return from campaign options without taking the main-menu load route.
pub(crate) fn return_from_game_options(mode: &mut GameMode) -> bool {
    if *mode != GameMode::GameOptions {
        return false;
    }
    *mode = GameMode::Galaxy;
    true
}

/// Handle the Encyclopedia destination without resetting its browsing state.
pub(crate) fn open_encyclopedia(
    button: CockpitButton,
    cockpit: &mut CockpitState,
    encyclopedia: &mut EncyclopediaState,
) -> bool {
    if button != CockpitButton::Encyclopedia {
        return false;
    }
    cockpit.gid_ui.menu_open = false;
    cockpit.gid_ui.category = None;
    encyclopedia.open = true;
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebellion_render::cockpit::GidCategory;
    use rebellion_render::encyclopedia::EncyclopediaTab;
    use rebellion_render::{strategic_primary_controls, CockpitFaction};

    #[test]
    fn game_options_opens_from_either_campaign_and_dismisses_gid() {
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let mut cockpit = CockpitState::new(faction);
            cockpit.gid_ui.menu_open = true;
            cockpit.gid_ui.category = Some(GidCategory::Fleets);
            let mut saves = SaveLoadPanelState::default();
            let mut mode = GameMode::Galaxy;

            open_game_options(&mut cockpit, &mut saves, &mut mode);

            assert_eq!(mode, GameMode::GameOptions);
            assert!(saves.open);
            assert!(saves.save_mode);
            assert!(!cockpit.gid_ui.menu_open);
            assert_eq!(cockpit.gid_ui.category, None);
            assert!(return_from_game_options(&mut mode));
            assert_eq!(mode, GameMode::Galaxy);
        }
    }

    #[test]
    fn options_return_does_not_change_other_navigation_routes() {
        for mut mode in [GameMode::MainMenu, GameMode::LoadGame, GameMode::Galaxy] {
            let before = mode.clone();
            assert!(!return_from_game_options(&mut mode));
            assert_eq!(mode, before);
        }
    }

    #[test]
    fn command_131_opens_encyclopedia_and_dismisses_gid_for_both_factions() {
        for faction in [CockpitFaction::Alliance, CockpitFaction::Empire] {
            let mut cockpit = CockpitState::new(faction);
            cockpit.gid_ui.menu_open = true;
            cockpit.gid_ui.category = Some(GidCategory::Fleets);
            let mut encyclopedia = EncyclopediaState::new();
            let control = strategic_primary_controls(faction)
                .iter()
                .find(|control| control.command_id == 0x131)
                .expect("Encyclopedia command");

            assert!(open_encyclopedia(
                control.button,
                &mut cockpit,
                &mut encyclopedia,
            ));
            assert!(encyclopedia.open);
            assert!(!cockpit.gid_ui.menu_open);
            assert_eq!(cockpit.gid_ui.category, None);
        }
    }

    #[test]
    fn repeated_activation_preserves_encyclopedia_selection() {
        let mut cockpit = CockpitState::new(CockpitFaction::Alliance);
        let mut encyclopedia = EncyclopediaState::new();
        encyclopedia.tab = EncyclopediaTab::Characters;
        encyclopedia.selected_index = 3;

        for _ in 0..2 {
            assert!(open_encyclopedia(
                CockpitButton::Encyclopedia,
                &mut cockpit,
                &mut encyclopedia,
            ));
            assert!(encyclopedia.open);
            assert_eq!(encyclopedia.tab, EncyclopediaTab::Characters);
            assert_eq!(encyclopedia.selected_index, 3);
        }
    }

    #[test]
    fn unrelated_controls_leave_encyclopedia_and_gid_untouched() {
        for button in [
            CockpitButton::SystemFinder,
            CockpitButton::FleetFinder,
            CockpitButton::TroopFinder,
            CockpitButton::PersonnelFinder,
            CockpitButton::GameOptions,
            CockpitButton::SaveLoad,
            CockpitButton::GalacticInformationDisplay,
        ] {
            let mut cockpit = CockpitState::new(CockpitFaction::Empire);
            cockpit.gid_ui.menu_open = true;
            cockpit.gid_ui.category = Some(GidCategory::Fleets);
            let mut encyclopedia = EncyclopediaState::new();

            assert!(!open_encyclopedia(button, &mut cockpit, &mut encyclopedia));
            assert!(!encyclopedia.open);
            assert!(cockpit.gid_ui.menu_open);
            assert_eq!(cockpit.gid_ui.category, Some(GidCategory::Fleets));
        }
    }
}
