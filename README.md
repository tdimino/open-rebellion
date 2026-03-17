<p align="center">
  <img src="assets/tanit.svg" alt="Symbol of Tanit" width="80"/>
</p>

<h1 align="center">Open Rebellion</h1>

<p align="center">
  <em>An open-source reimplementation of Star Wars Rebellion (1998)</em>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-macroquad-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/Platform-macOS%20%7C%20Browser-blue.svg" alt="Platform">
  <img src="https://img.shields.io/badge/Parity-95%25-brightgreen.svg" alt="Parity">
  <img src="https://img.shields.io/badge/Tests-266%20passing-green.svg" alt="Tests">
  <img src="https://img.shields.io/badge/License-MIT-lightgrey.svg" alt="License">
</p>

---

<p align="center">
  <img src="assets/rebellion-menu.jpg" alt="Star Wars Rebellion main menu" width="720"/>
  <br/>
  <sub>Screenshot from <a href="https://x.com/oldyzach/status/2023783273046933925">@oldyzach</a></sub>
</p>

If you ever stayed up past midnight watching your fleet crawl across the galaxy map toward Coruscant, wondering if your three Mon Cal cruisers could take the Super Star Destroyer parked there—this project is for you.

If you spent hours on TheForce.Net debating whether Vergere was secretly Sith or whether the Force has no dark side as Jacen learned in *Traitor*, or ran play-by-post RPGs on private InvisionFree boards set in the Outer Rim—this project is for you.

If you modded Star Wars Galaxies until the servers shut down, dueled and roleplayed with sabers down on Jedi Academy honor servers, ran Rebellion tournaments on mIRC, or still have your GOG copy installed just to hear that Imperial March fanfare one more time—this project is *especially* for you.

**Open Rebellion** is a from-scratch Rust reimplementation of [Star Wars Rebellion](https://www.gog.com/en/game/star_wars_rebellion) (Coolhand/LucasArts, 1998), the 4X galactic strategy game that never got a sequel, never got a Mac port, and never got the love it deserved. We're fixing all three.

## What It Does

Rebellion was a game about grand strategy in the Star Wars universe—not the lightsaber duels or the trench runs, but the *logistics*. Who controls the shipyards at Fondor? Can you flip Sullust before the Empire garrisons it? Is Luke ready to go to Dagobah? You managed officers, research trees, espionage networks, fleet deployments, and planetary economies across 200 star systems.

Open Rebellion reads the original game data files, converts them to clean JSON, and reimplements the simulation from the ground up in Rust. It runs natively on macOS and in the browser via WebAssembly.

### Current State: v0.7.0 — Release READY (~95% parity)

Seven phases complete. 15 simulation systems:

- **Galaxy Viewer** — 200 star systems, pan/zoom/click, 51/51 DAT parsers with byte-level round-trip validation
- **Living Galaxy** — Game clock, manufacturing, 9 mission types, AI manager, event system, mod loader
- **War Room** — Player faction selection, 5 UI panels, fleet movement, fog of war, encyclopedia, audio
- **War Machine** — Space combat (7-phase pipeline), ground combat, orbital bombardment, blockade, uprising, Death Star, research, Jedi training, victory conditions, save/load
- **Full Parity** — 4 scripted story chains (Dagobah, Final Battle, Bounty Hunters, Jabba's Palace), Han Solo speed bonus, betrayal mechanics, decoy system, character escape (ESCAPETB), mission state flags
- **Mod Workshop** — Sensor-radius fog, captivity tracking, save format v4 with mod metadata + migration framework, ModRuntime (discover/enable/disable/reload), Mod Manager panel, pure research contract
- **Ghidra RE Complete** — 5,127 functions decompiled from REBEXE.EXE, 111 GNPRTB parameters mapped, C++ class hierarchy reconstructed

## You Will Need

- **Rust** (stable toolchain)
- **A legal copy of Star Wars Rebellion**—[GOG](https://www.gog.com/en/game/star_wars_rebellion) ($5.99), Steam, or original CD. Extract the `GData/` directory.

We don't distribute any game data. Same model as [DevilutionX](https://github.com/diasurgical/devilutionX), [OpenMW](https://openmw.org), and [The Force Engine](https://theforceengine.github.io)—you bring the data, we bring the engine.

## Quick Start

```bash
# Clone
git clone https://github.com/tdimino/open-rebellion.git
cd open-rebellion

# Copy your GData files
cp -r /path/to/star-wars-rebellion/GData/* data/base/

# Run native (macOS)
cargo run -p rebellion-app -- data/base

# Or build for browser
bash scripts/build-wasm.sh
# Then serve web/ with any HTTP server
python3 -m http.server 8080 -d web/
```

**Controls**: scroll to zoom, right-drag to pan, left-click to select a system, `R` to reset view, `Esc` to quit.

## Architecture

Five Rust crates in a Cargo workspace:

| Crate | Purpose |
|-------|---------|
| `rebellion-core` | Pure game types—no rendering, no IO. Entity IDs, world model, simulation structs. |
| `rebellion-data` | Loads original `.DAT` binary files into the game world. |
| `rebellion-render` | macroquad 0.4 galaxy map + egui-macroquad UI panels. |
| `rebellion-app` | Entry point—runs the main loop on desktop and WASM. |
| `dat-dumper` | CLI tool that exports all `.DAT` files to human-readable JSON. |

The original game's binary data files (51 DAT files) and executable (REBEXE.EXE, 22,741 functions) have been fully reverse-engineered using [Metasharp's editor](https://github.com/MetasharpNet/StarWarsRebellionEditor.NET) and [Ghidra](https://ghidra-sre.org/). Every DAT parser passes round-trip byte validation. Combat formulas, AI decision trees, and game balance parameters are documented in `ghidra/notes/`.

## Roadmap

| Milestone | Status | What You Get |
|-----------|--------|-------------|
| **Galaxy Viewer** | Complete | Interactive star map, 51/51 DAT parsers, WASM build |
| **Living Galaxy** | Complete | Game clock, 9 missions, manufacturing, AI, events, mod loader |
| **War Room** | Complete | Player UI, fleet movement, fog of war, encyclopedia, audio |
| **War Machine** | Complete | Combat (space/ground/bombardment), blockade, uprising, Death Star, victory, save/load |
| **Full Parity** | Complete | Story events, betrayal, decoys, escape, Han speed bonus, mission flags |
| **Mod Workshop** | Complete | Sensor fog, captivity, save migration, mod manager panel, ModRuntime |
| **Release** | Ready | Release packaging, CI, example mod, vendored WASM deps |

**Ghidra RE complete.** 5,127 functions decompiled from `REBEXE.EXE`, combat formulas decoded, 111 GNPRTB parameters mapped, C++ class hierarchy reconstructed. See `ghidra/notes/` for the full corpus (7 scholar documents, 4,179 lines).

## The Data Pipeline

```
Your GOG copy          dat-dumper           Open Rebellion
─────────────          ──────────           ─────────────
GData/SYSTEMSD.DAT  →  JSON  →  200 star systems
GData/CAPSHPSD.DAT  →  JSON  →  30 capital ship classes
GData/MJCHARSD.DAT  →  JSON  →  6 major characters (Luke, Vader, ...)
GData/GNPRTB.DAT    →  JSON  →  213 game balance parameters
TEXTSTRA.DLL        →  pelite → Real entity names ("Coruscant", "Luke Skywalker")
```

Run `cargo run -p dat-dumper -- --gdata data/base --output data/base/json` to export everything.

## Asset Pipeline

Every resource in the original game has been fully extracted to open formats—**nothing remains locked in the 1998 binaries**:

| Resource | Count | Format | Source |
|----------|-------|--------|--------|
| Game data | 51 files | DAT → JSON | dat-dumper (round-trip validated) |
| UI images | 2,441 BMPs | BMP → PNG | 11 DLLs via pefile extraction |
| Animation data | 3,223 files | BIN | ALSPRITE/EMSPRITE/ALBRIEF/EMBRIEF.DLL |
| Voice lines | 285 WAVs | WAV | VOICEFXA/VOICEFXE.DLL |
| Cutscene videos | 15 files | SMK → WebM | MDATA/ via ffmpeg |
| Soundtrack | 16 WAVs | WAV | MDATA/300-315 (John Williams excerpts) |
| Entity names | 511+ strings | UTF-16 → text | TEXTSTRA.DLL via pelite |
| Encyclopedia text | 348 entries | RT_RCDATA | ENCYTEXT.DLL |

Six AI-powered pipelines handle upscaling and new asset generation:

1. **HD Upscaling**—waifu2x (batch), Vertex AI Imagen 4.0 (faithful), Gemini edit (generative detail)
2. **3D Models**—Hunyuan3D Pro, WaveSpeedAI ($0.02/model), Meshy, Trellis 2 → Blender sprite sheets
3. **Reference Image Generation**—nano-banana-pro with 11 curated reference collections (91 images)
4. **Audio**—Voicebox/Qwen3-TTS (voice cloning), LavaSR v2 (upscaling), ACE-Step 1.5 (music), ElevenLabs (SFX)
5. **UI Upscaling**—per-category strategy: Vertex AI for buttons/sprites, Gemini edit for backgrounds
6. **Encyclopedia Content**—TEXTSTRA.DLL names + Wookieepedia-sourced descriptions

See [CREDITS.md](CREDITS.md) for full tool attribution.

## Modding

The entire point of rebuilding from scratch is to make Rebellion *moddable*. The original game hardcoded everything. We won't.

The mod system supports:
- **Add anything**—new systems, characters, ships, fighters, missions, events
- **Patch anything**—field-level JSON overlays using RFC 7396 Merge Patch
- **In-game mod manager**—discover, enable/disable, reload from the Mod Manager panel (Tab key)
- **Hot reload**—edit a mod, see changes instantly (native only)
- **Dependency management**—`mod.toml` manifests with semver constraints, topological load order
- **Save compatibility**—save files track which mods were active (FNV-1a hash, warns on mismatch)

If you've ever wanted to add the *Executor*-class as a buildable ship, give Mara Jade a recruitment mission, or create a Clone Wars total conversion—that's what this is for.

## Community Roots

This project exists because a small, stubborn community kept Rebellion alive for 25+ years:

- **[swrebellion.net](https://swrebellion.net)**—The hub. Forums, mods, the [Mechanics Inside Rebellion](https://swrebellion.net/forums/topic/9639-mechanics-inside-rebellion-part-ii/) thread that documented game internals.
- **[RebED](https://swrebellion.net/files/)**—240+ mod cards created by the community over two decades.
- **[Metasharp's Editor](https://github.com/MetasharpNet/StarWarsRebellionEditor.NET)**—686 commits of .NET code reverse-engineering every binary format. Without this, Open Rebellion wouldn't exist.
- **[Prima Strategy Guide](https://archive.org/details/star-wars-rebellion-guide/mode/2up)**—276 pages, free on archive.org.

We stand on their shoulders.

## Contributing

We're one developer and one AI collaborator, building in public. If you want to help:

- **Play-testing**: Run it with your GOG copy, report what looks wrong—tick speed, AI behavior, combat balance, mission success rates
- **Game data expertise**: If you know what GNPRTB parameter #147 does, or how the original handled sensor range, open an issue
- **Modding**: Create mods, test the mod manager, report issues with the JSON overlay system
- **Story events**: 15+ scripted story beats are documented in `ghidra/notes/` but only 4 major chains are implemented—help add the rest
- **Art and audio**: All 2,441 original BMPs + 285 voice WAVs + 15 videos are extracted. Help upscale, generate HD replacements, or create total conversion assets (see `agent_docs/assets.md`)
- **Distribution**: Homebrew formula, itch.io packaging, WASM optimization

## License

MIT. The engine is free. The game data is yours—bring your own copy.

---

<p align="center">
  <em>"Many are the wand-bearers, but few are the Bacchoi."</em><br/>
  <sub>— Plato, <i>Phaedo</i> 69c</sub>
</p>

<p align="center">
  <sub>Built by <a href="https://github.com/tdimino">Tom di Mino</a> and <a href="https://claude.com/claude-code">Claudius, Artifex Maximus</a> · Minoan Mystery LLC</sub>
</p>
