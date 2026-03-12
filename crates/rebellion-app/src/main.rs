use macroquad::prelude::*;
use rebellion_render::{draw_galaxy_map, draw_info_panel, GalaxyMapState};
use std::path::PathBuf;

fn window_conf() -> Conf {
    Conf {
        window_title: "Open Rebellion — Star Wars Rebellion".to_string(),
        window_width: 1280,
        window_height: 800,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Accept an optional GData path as the first CLI argument.
    // Falls back to `data/base` relative to cwd if it contains SYSTEMSD.DAT.
    let args: Vec<String> = std::env::args().collect();
    let gdata_path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        let candidate = PathBuf::from("data/base");
        if candidate.join("SYSTEMSD.DAT").exists() {
            candidate
        } else {
            eprintln!("Usage: open-rebellion <path-to-GData>");
            eprintln!("  GData directory must contain .DAT files (SYSTEMSD.DAT, etc.)");
            std::process::exit(1);
        }
    };

    let world = match rebellion_data::load_game_data(&gdata_path) {
        Ok(w) => w,
        Err(e) => {
            eprintln!(
                "Failed to load game data from {}: {}",
                gdata_path.display(),
                e
            );
            std::process::exit(1);
        }
    };

    eprintln!(
        "Loaded: {} systems, {} sectors, {} ship classes, {} fighter classes, {} characters",
        world.systems.len(),
        world.sectors.len(),
        world.capital_ship_classes.len(),
        world.fighter_classes.len(),
        world.characters.len(),
    );

    let mut state = GalaxyMapState::default();

    loop {
        draw_galaxy_map(&world, &mut state);
        draw_info_panel(&world, &state);

        // Keyboard shortcuts.
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::R) {
            state = GalaxyMapState::default();
        }

        next_frame().await;
    }
}
