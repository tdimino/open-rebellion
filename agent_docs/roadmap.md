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

## War Machine -- BLOCKED on Ghidra RE
*Complete strategy game*

1. Space combat: auto-resolve first, then tactical 2D view
2. Ground combat: regiment engagement, orbital bombardment
3. Blockade mechanics, Death Star construction/destruction
4. Victory conditions (capture enemy HQ, destroy Death Star / find Rebel base)
5. 3D tactical models: nano-banana-pro concept art → Hunyuan3D Pro/Meshy → Blender sprite sheets. Pipeline proven in World War Watcher (14 models). See `@agent_docs/assets.md`.

Blocked: Combat formulas live in REBEXE.EXE (2.8MB, 22,741 functions), NOT STRATEGY.DLL (resource-only, 29MB sprites, 9KB of CRT boilerplate). swrebellion.net community has no RE work — empirical data editing only. We're on our own with Ghidra. See `@agent_docs/ghidra-re.md`.

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

1. **WASM build verification** -- `bash scripts/build-wasm.sh` with all War Room systems (in progress)
2. **Play-test session** -- tick speed feel, AI behavior, mission success rates, panel usability
3. **Ghidra RE of REBEXE.EXE** -- Combat formulas (STRATEGY.DLL is resource-only; project created, GhidraMCP bridge active on :8080)
4. **Victory conditions implementation** -- capture enemy HQ, destroy Death Star / find Rebel base

## Known Technical Debt

- dat-dumper in `tools/` is also a library dep of rebellion-data -- works but unconventional
- WASM uses `std::process::exit` and `std::fs::read` -- need `#[cfg]` guards for browser target
- `web/gl.js` fetched from external URL on first build -- should vendor or pin version
- `CapitalShipClass`/`FighterClass` world models carry only ~10 of 50+ DAT fields -- expand when combat lands
- `SectorGroup` enum lives in `dat/` module but is a world-layer concept -- move to `world/`
