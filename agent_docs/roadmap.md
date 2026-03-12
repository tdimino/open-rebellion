# Roadmap

## Galaxy Viewer -- COMPLETE
*Interactive galaxy data viewer -- native + WASM*

Commits: `4e258bb` (scaffold) > `5905497` (dat-dumper + types) > `fc968ec` (renderer + WASM) > `22b61a6` (review fixes)

Delivered:
- Cargo workspace with 5 crates
- dat-dumper: 22/22 documented DAT files parsed, 3 structural patterns, round-trip byte validated
- rebellion-core: DatId newtype, 11 slotmap keys, dat/ enums, world/ structs, GameWorld root
- rebellion-data: DAT to GameWorld loader (sectors > systems > ships > fighters > characters)
- rebellion-render: macroquad galaxy map with pan/zoom/click-to-select, egui info panel + status bar
- WASM build: 2.9MB browser artifact via scripts/build-wasm.sh

## Living Galaxy -- NEXT
*Automated galaxy simulation*

1. Parse TEXTSTRA.DLL string resources -- replace placeholder entity names
2. Game tick system with time controls (pause/1x/2x/4x)
3. Mission system (diplomacy, recruitment) -- port from rebellion2's Mission.cs
4. Manufacturing system (production queues) -- port from rebellion2
5. Event system (conditionals + actions)
6. AI manager (officer assignment, production priority)
7. Message log UI (egui panel)
8. Mod loader: TOML manifests, JSON overlay, hot reload (native only)

Dependencies: None -- all source material available.

## War Room
*Full strategy game without combat*

1. Player faction selection (Empire/Alliance)
2. Full egui UI: officers, fleets, manufacturing, research, missions
3. Fleet movement on galaxy map (animated hyperspace routes)
4. Fog of war and intel system
5. Encyclopedia viewer (loaded from extracted BMPs in EData/)
6. Audio via kira crate (CoreAudio on macOS, WebAudio in browser)

Dependencies: Living Galaxy tick system and simulation.

## War Machine -- BLOCKED on Ghidra RE
*Complete strategy game*

1. Space combat: auto-resolve first, then tactical 2D view
2. Ground combat: regiment engagement, orbital bombardment
3. Blockade mechanics, Death Star construction/destruction
4. Victory conditions (capture enemy HQ, destroy Death Star / find Rebel base)

Blocked: Combat formulas live in STRATEGY.DLL (29MB) and TACTICAL.DLL (7.5MB). Need Ghidra RE or community documentation (swrebellion.net forums).

## Full Parity
*Feature-complete*

1. All espionage missions (sabotage, assassination, rescue, abduction, incite uprising)
2. All scripted events (Luke/Vader, Yoda training, Han Solo speed bonus)
3. Special character abilities (Jedi training, betrayal, decoys)
4. Video playback (Smacker to WebM), sound effects, music
5. Save/load with mod compatibility metadata (bincode + mod version hash)

## Mod Workshop
*Release candidate*

1. Mod manager UI (egui)
2. AI asset generation toolkit (nano-banana-pro + image-forge + meshy + Qwen3-TTS)
3. Modder documentation + example mods
4. Hot reload for mod development (notify crate on native)
5. Distribution: itch.io (web), Homebrew (macOS), GitHub Releases

## Immediate Next Steps

1. **TEXTSTRA.DLL parsing** -- Win32 PE string resource extraction. Unlocks real entity names ("Coruscant" instead of "System 11648").
2. **Tick system** -- Frame-independent game clock with pause/speed controls. Foundation for all simulation.
3. **MISSNSD.DAT parsing** -- Mission definitions (2,816B). Likely Pattern 1 or 2. Needed for Living Galaxy mission system.

## Known Technical Debt

- dat-dumper in `tools/` is also a library dep of rebellion-data -- works but unconventional
- WASM uses `std::process::exit` and `std::fs::read` -- need `#[cfg]` guards for browser target
- `web/gl.js` fetched from external URL on first build -- should vendor or pin version
- `CapitalShipClass`/`FighterClass` world models carry only ~10 of 50+ DAT fields -- expand when combat lands
- `SectorGroup` enum lives in `dat/` module but is a world-layer concept -- move to `world/`
