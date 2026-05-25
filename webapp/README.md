# Open Rebellion — Modern Web UI

A React + TypeScript + WASM frontend for Open Rebellion, demonstrating the
**augment** architecture: keep `rebellion-core` (game logic) in Rust, build
the UI in modern web tech.

## Featured Feature: Character Group Profiles

A QoL feature the original Star Wars Rebellion never had — define persistent
group profiles like *"When Han Solo and Chewbacca are selected together,
Han→Sabotage, Chewie→Espionage"* and they auto-apply in the mission planner.

Profiles live entirely in the UI layer (localStorage). The Rust engine
receives normal mission-dispatch commands at the moment you confirm.

## Architecture

```
┌──────────────────────────────────────────────────────────┐
│  React + TypeScript UI (webapp/)                          │
│  - CharacterList (Ctrl+A, shift-select, etc.)             │
│  - ProfileManager (CRUD, import/export)                   │
│  - MissionPlanner (auto-applies profiles, allows override)│
│  - localStorage for profiles                              │
└─────────────┬────────────────────────────────────────────┘
              │ wasm-bindgen
┌─────────────▼────────────────────────────────────────────┐
│  rebellion-web crate (crates/rebellion-web/)              │
│  - WASM bindings, JSON-serializable DTOs                  │
│  - Thread-local Engine singleton                          │
│  - Wraps rebellion-core, rebellion-data                   │
└─────────────┬────────────────────────────────────────────┘
              │
┌─────────────▼────────────────────────────────────────────┐
│  rebellion-core, rebellion-data (existing crates)         │
│  - 100% game logic parity, deterministic sim              │
│  - GameWorld, Character, MissionSystem, etc.              │
└──────────────────────────────────────────────────────────┘
```

## Build & Run

### Prerequisites

- **Rust** (rustup, with wasm32-unknown-unknown target)
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- **wasm-pack**
  ```bash
  cargo install wasm-pack
  ```
- **Node.js 18+** and npm

### Quick start

```bash
# From repo root: build WASM + webapp bundle
bash scripts/build-webapp.sh

# Dev mode (Vite hot reload, falls back to JS mock if WASM missing)
bash scripts/dev-webapp.sh
```

Then open <http://localhost:5173>.

### Mock mode

If the WASM build hasn't run yet, the webapp automatically falls back to a
TypeScript mock engine (in `src/wasm/engine.ts`) that simulates a small set
of canonical characters/systems. **Great for UI iteration without rebuilding
Rust** — you can develop the entire UI in pure JS/TS first.

## Hotkeys

| Hotkey | Action |
|--------|--------|
| `Ctrl + A` | Select all visible characters |
| `Shift + click` | Range select |
| `Ctrl + click` | Toggle individual selection |
| `Esc` | Clear selection |
| `Enter` | Open mission planner with selection |
| `Delete` | Clear selection |

## Project Layout

```
webapp/
├── src/
│   ├── App.tsx                 — Top-level shell + tabs
│   ├── main.tsx                — React entry
│   ├── styles.css              — Star Wars dark theme
│   ├── types/
│   │   ├── game.ts             — Engine DTOs (Character, System, etc.)
│   │   └── profiles.ts         — GroupProfile + matching logic
│   ├── wasm/
│   │   ├── engine.ts           — TS wrapper around WASM, with JS mock fallback
│   │   └── pkg/                — wasm-pack output (generated)
│   ├── hooks/
│   │   ├── useEngine.ts        — Game state + refresh
│   │   ├── useGroupProfiles.ts — Profile CRUD + localStorage
│   │   ├── useSelection.ts     — Multi-select with shift/ctrl modifiers
│   │   └── useHotkeys.ts       — Global keyboard shortcuts
│   └── components/
│       ├── CharacterList.tsx   — Sortable, filterable, multi-select table
│       ├── ProfileManager.tsx  — Profile CRUD UI + import/export
│       ├── MissionPlanner.tsx  — Mission planner with profile auto-apply
│       └── StatusBar.tsx       — Day/missions/advance time controls
├── vite.config.ts              — Vite + WASM plugin config
├── tsconfig.json
├── package.json
└── index.html
```

## Why This Architecture?

The Rust game engine (`rebellion-core`) gives us deterministic simulation
with 100% combat parity to the original 1998 game. But macroquad (their
current renderer) is a hobbyist engine — fine for solo dev, limiting for
community-driven feature work.

Modern web tooling gives us:

- 10x faster UI iteration (hot reload, instant feedback)
- 100x larger contributor pool (web devs)
- Browser-native deploy (no install)
- Easy QoL features like Ctrl+A select, drag-rectangle multi-select,
  customizable hotkeys, dark mode, accessibility, etc.

The **augment** approach keeps the best of both: Rust for correctness,
TypeScript for UX velocity.

See also: `crates/rebellion-web/src/lib.rs` for the WASM API surface.
