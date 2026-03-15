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

## Living Galaxy -- COMPLETE
*Automated galaxy simulation — shipped 2026-03-13 by Knesset Athirat*

All 11 deliverables shipped. ~8.7K LOC, 105+ unit tests. See `docs/reports/2026-03-13-knesset-athirat-swarm-report.md`.

Delivered:
- TEXTSTRA.DLL string extraction via pelite (real entity names)
- GameClock with pause/1x/2x/4x speed controls
- 9 seed table loaders + fleet instantiation
- All 51/51 DAT parsers with round-trip validation (29 new)
- Manufacturing system with production queues
- Mission system (diplomacy, recruitment) -- ported from rebellion2's Mission.cs
- Event system (conditional triggers, chaining, deterministic rng)
- AI manager (officer assignment, production priority, fleet deployment)
- Message log UI (egui bottom panel, 6 color-coded categories)
- Mod loader: TOML manifests, RFC 7396 merge patch, semver, hot reload
- Main loop integration (all systems wired)

## War Room -- COMPLETE
*Full strategy game without combat*

Committed: `6d47a10` (integration wiring — fog/fleet/panels/encyclopedia/audio into main loop)

Delivered:
- Player faction selection + 5 UI panels (officers, fleets, manufacturing, missions)
- Fleet movement on galaxy map (diamond icons, dashed routes, ETA labels)
- Fog of war with monotonic reveal + advance intel at 50% transit
- Encyclopedia viewer with 4 tabs + BMP texture cache from EData/
- Audio system via quad-snd (CoreAudio/ALSA/WebAudio)
- Main loop integration: all draw calls + event hooks wired

## War Machine -- UNBLOCKED (RE complete, ready to implement)
*Complete strategy game*

1. Space combat: auto-resolve first, then tactical 2D view
2. Ground combat: regiment engagement, orbital bombardment
3. Blockade mechanics, Death Star construction/destruction
4. Victory conditions (capture enemy HQ, destroy Death Star / find Rebel base)
5. 3D tactical models: nano-banana-pro concept art → Hunyuan3D Pro/Meshy → Blender sprite sheets. Pipeline proven in World War Watcher (14 models). See `@agent_docs/assets.md`.

Ghidra RE of REBEXE.EXE complete: 109 functions decompiled, combat call chain fully traced, bombardment formula decoded, 71 combat GNPRTB parameters mapped, game object layout documented. See `ghidra/notes/` for modular docs and 1,662-line annotated function reference.

## Full Parity
*Feature-complete*

1. All espionage missions (sabotage, assassination, rescue, abduction, incite uprising)
2. All scripted events (Luke/Vader, Yoda training, Han Solo speed bonus)
3. Special character abilities (Jedi training, betrayal, decoys)
4. Video playback (Smacker to WebM), sound effects, music
5. Save/load with mod compatibility metadata (bincode + mod version hash)
6. HD asset pack: all original BMPs upscaled 4x via waifu2x-ncnn-vulkan (4x SGI model via chaiNNer for comparison). PNG format, `data/hd/` with fallback to originals. See `@agent_docs/assets.md`.

## Mod Workshop
*Release candidate*

1. Mod manager UI (egui)
2. AI asset generation toolkit: nano-banana-pro (concept art) → Hunyuan3D Pro/Meshy (3D models) → Blender (sprite sheets) → waifu2x/chaiNNer (texture upscaling) → image-forge (compositing) + Qwen3-TTS (voice). Modders regenerate any asset via the same pipeline. See `@agent_docs/assets.md`.
3. Modder documentation + example mods
4. Hot reload for mod development (notify crate on native)
5. Distribution: itch.io (web), Homebrew (macOS), GitHub Releases

## Immediate Next Steps

1. **Launch Knesset Tiamat** -- War Machine implementation swarm (combat, missions, victory, save/load)
2. **Play-test session** -- tick speed feel, AI behavior, mission success rates, panel usability
3. **WASM build verification** -- confirmed 3.4MB artifact at v0.3.0
4. **Ghidra RE** -- COMPLETE. 109 functions, bombardment formula decoded, GNPRTB mapped. Continue decompiling for modder documentation.

## Known Technical Debt

- dat-dumper in `tools/` is also a library dep of rebellion-data -- works but unconventional
- WASM cfg guards added but browser data loading returns error stub (no fetch API yet)
- `web/gl.js` fetched from external URL on first build -- should vendor or pin version
- `CapitalShipClass`/`FighterClass` world models carry only ~10 of 50+ DAT fields -- 15 more needed for combat (see `ghidra/notes/rust-implementation-guide.md`)
- `SectorGroup` enum lives in `dat/` module but is a world-layer concept -- move to `world/`
- `GameWorld` needs `gnprtb: GnprtbParams` and `mission_tables: HashMap<String, MstbTable>` for combat
- Need `ShipInstance` type (per-hull combat state) to replace count-only `ShipEntry` in fleets
