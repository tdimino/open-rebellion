# Open Rebellion

Rust + macroquad + egui reimplementation of Star Wars Rebellion (1998, LucasArts). Runs native (macOS/Metal) and browser (WebAssembly/WebGL2). Phase 0 complete: interactive galaxy data viewer.

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
| `dat-dumper` (tools/) | 22/22 DAT parsers with round-trip byte validation. CLI + library | clap, serde |

## Conventions

- **Two-layer types**: `dat/` structs mirror binary layout (import/export only); `world/` structs are the runtime model used by game logic and rendering
- **Entity identity**: `DatId(u32)` preserves original binary IDs; slotmap `*Key` types are runtime-only arena handles -- never serialize slotmap keys directly
- **DAT parsers**: Implement `DatRecord` trait (`parse` + `write_bytes`). Every parser must pass round-trip byte validation -- if reserialized bytes differ from original, the parser is wrong
- **Entity table pattern**: 16-byte header (`field1`, `count`, `family_id`, `field4`) then `count` fixed-size records
- **Never add rendering deps to rebellion-core** -- it must remain headless-testable
- **Never use `fieldN` for a field whose meaning is known** -- name it semantically

## Known Limitations

- Entity names are placeholders ("System 11648") -- real names live in TEXTSTRA.DLL (Win32 string resources), not yet parsed
- 22 of ~51 DAT files have parsers; remaining ~29 are undocumented
- WASM compiles but `std::process::exit` and `std::fs::read` need `#[cfg(not(target_arch = "wasm32"))]` guards
- dat-dumper lives in `tools/` but is a library dependency of rebellion-data

## Agent Docs

@agent_docs/architecture.md -- Crate graph, type system layers, entity identity, data flow, render architecture. Read when adding entity types or crates.
@agent_docs/dat-formats.md -- DAT binary format reference, all 3 structural patterns, file inventory, codec API. Read when parsing new DAT files.
@agent_docs/game-domain.md -- Game mechanics glossary, entity relationships, unimplemented systems. Read when implementing simulation logic.
@agent_docs/roadmap.md -- Phase breakdown with status, what's next, what's blocked. Read when planning work.

## External References

- Metasharp editor (C# DAT parsers): `~/Desktop/Programming/StarWarsRebellionEditor.NET/`
- rebellion2 (C# game models): `~/Desktop/Programming/rebellion2/`
- Extracted game data: `~/Desktop/Programming/star-wars-rebellion/GData/`
- Project plan: `~/.claude/plans/2026-03-11-open-rebellion-star-wars-rebellion-reimplementation.md`
