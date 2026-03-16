# Open Rebellion

Rust + macroquad + egui reimplementation of Star Wars Rebellion (1998, LucasArts). Runs native (macOS/Metal) and browser (WebAssembly/WebGL2). v0.6.0 (~92% complete). Living Galaxy + War Room + War Machine + Full Parity + Mod Workshop (in progress). 15 simulation systems, 4 scripted story chains, betrayal/decoy/escape mechanics, Han Solo speed bonus, mod manager panel. 259 tests passing.

## Build

**Critical**: `~/.local/bin/cc` shadows Apple clang. Prefix all cargo commands with `PATH="/usr/bin:$PATH"`.

```bash
# Native (interactive galaxy map)
PATH="/usr/bin:$PATH" cargo run -p rebellion-app -- data/base

# WASM (browser build, then serve web/)
bash scripts/build-wasm.sh

# dat-dumper (DAT binary files to JSON)
PATH="/usr/bin:$PATH" cargo run -p dat-dumper -- --gdata data/base --output data/base/json

# Type check
PATH="/usr/bin:$PATH" cargo check
```

## Workspace

| Crate | Role | Key Deps |
|-------|------|----------|
| `rebellion-core` | Pure types, zero IO. `ids.rs`, `dat/`, `world/` | slotmap, serde |
| `rebellion-data` | DAT to GameWorld loader. Uses dat-dumper as library | rebellion-core, dat-dumper |
| `rebellion-render` | macroquad 0.4 + egui-macroquad 0.17 galaxy map | rebellion-core, macroquad |
| `rebellion-app` | Entry point and main loop | all crates above |
| `dat-dumper` (tools/) | 51/51 DAT parsers with round-trip byte validation. CLI + library | clap, serde |

## Conventions

- **Two-layer types**: `dat/` structs mirror binary layout (import/export only); `world/` structs are the runtime model used by game logic and rendering
- **Entity identity**: `DatId(u32)` preserves original binary IDs; slotmap `*Key` types are runtime-only arena handles -- never serialize slotmap keys directly
- **DAT parsers**: Implement `DatRecord` trait (`parse` + `write_bytes`). Every parser must pass round-trip byte validation -- if reserialized bytes differ from original, the parser is wrong
- **Entity table pattern**: 16-byte header (`field1`, `count`, `family_id`, `field4`) then `count` fixed-size records
- **Never add rendering deps to rebellion-core** -- it must remain headless-testable
- **Never use `fieldN` for a field whose meaning is known** -- name it semantically

## Known Limitations

- WASM cfg guards added (v0.3.0) but browser data loading still returns error stub
- dat-dumper lives in `tools/` but is a library dependency of rebellion-data
- MISSNSD.DAT flag_col6..flag_col21 semantics partially resolved via Ghidra RE -- see `ghidra/notes/annotated-functions.md`
- CapitalShipClass/FighterClass world models carry only ~10 of 50+ DAT fields -- 15 more needed for combat (see `ghidra/notes/rust-implementation-guide.md`)
- Save v3 files rejected (bincode layout incompatible) -- migration from v3 not possible without SaveStateV3 struct
- Mod Manager panel shows empty list until ModRuntime is fully wired with mod data population
- `web/gl.js` still fetched from external URL on first WASM build -- should vendor

## Agent Docs

@agent_docs/architecture.md -- Crate graph, type system layers, entity identity, data flow, render architecture. Read when adding entity types or crates.
@agent_docs/roadmap.md -- Phase breakdown with status, what's next, what's blocked. Read when planning work.
agent_docs/simulation.md -- 15 simulation systems index, advance() pattern, integration order, "how to add" guides. Read when touching game logic.
agent_docs/systems/*.md -- Per-system detail docs (combat, blockade, uprising, death-star, research, jedi, victory, betrayal). Read when modifying a specific system.
agent_docs/save-load.md -- Save format (v4), migration framework, mod metadata hash, WASM stubs. Read when touching save/load.
agent_docs/mod-runtime.md -- ModRuntime, ModConfig, enable/disable, hot reload, structured errors. Read when wiring mod features.
agent_docs/dat-formats.md -- DAT binary format reference, all 3 structural patterns, file inventory, codec API. Read when parsing new DAT files.
agent_docs/game-domain.md -- Game mechanics glossary, entity relationships, implemented vs unimplemented systems. Read when implementing simulation logic.
agent_docs/modding.md -- Mod loader: TOML manifest, RFC 7396 merge patch, semver, load order, hot reload. Read when working on mod features.
agent_docs/ghidra-re.md -- REBEXE.EXE reverse engineering COMPLETE: 5,127 decompiled functions, combat formulas, 111 GNPRTB params, C++ class hierarchy, modder taxonomy. Read when implementing combat or designing mods.
agent_docs/assets.md -- Asset pipeline: 5 pipelines (HD upscaling, 3D models, sprite sheets, reference image generation, audio). 11 reference collections, 4 3D providers, 6 audio tools. Read when working on visual or audio assets.

## Reports

- [Knesset Athirat Swarm Report (2026-03-13)](docs/reports/2026-03-13-knesset-athirat-swarm-report.md) — Living Galaxy (COMPLETE) + War Room (COMPLETE). 4 daborot, ~45 min, ~11.7K LOC, 105+ tests.

## External References

- Metasharp editor (C# DAT parsers): `~/Desktop/Programming/StarWarsRebellionEditor.NET/`
- rebellion2 (C# game models): `~/Desktop/Programming/rebellion2/`
- Extracted game data: `~/Desktop/Programming/star-wars-rebellion/GData/`
- Project plan: `~/.claude/plans/2026-03-12-claude-md-agent-docs-for-open-rebellion.md`
