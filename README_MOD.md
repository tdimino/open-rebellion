# Modding Open Rebellion

Open Rebellion loads its simulation data from the original game's `.DAT`
files, then lets you patch the resulting in-memory world with JSON overlay
files — no Rust required. This document covers how to write, install, and
test a mod. For the runtime implementation details, see
`agent_docs/modding.md` and `agent_docs/mod-runtime.md`.

## 1. Generate your reference data first

Before you can write a patch you need two things: the numeric `dat_id` of
the entity you want to change, and the exact field names/current values on
it. Both come from dumping the original `.DAT` files to JSON.

**Via Docker** (see the main `README.md` Docker section for the base setup):

```bash
# In .env:
PREPARE_MODDING=1

docker compose up
```

This runs an extra step in the `builder` service that writes:

- `data/base/json/<TABLE>.json` — one file per original `.DAT` table
  (e.g. `CAPSHPSD.json` for capital ships), with every field the game reads.
- `data/base/json/textstra.json` — `{ "<string_id>": "Display Name" }`,
  looked up via each entity's `text_stra_dll_id` field.

`PREPARE_MODDING` defaults to `0` (skipped) since most people don't need it;
turn it on when you're about to write a mod. It's idempotent to rerun.

**Natively**, the equivalent is:

```bash
cargo build -p dat-dumper --release
./target/release/dat-dumper --gdata data/base --output data/base/json
./target/release/dat-dumper --gdata data/base --extract-strings --output data/base/json
```

Both `data/base/json/` and the game data itself are gitignored — this is
local reference material, not something to commit.

### Finding an entity's `dat_id`

Open the relevant JSON file and find your entity by name, cross-referencing
each record's `text_stra_dll_id` against `textstra.json`. The `id` field in
that record **is** the exact number to use as `"id"` in your overlay patch —
it's written into `DatId` unmodified at load time (`dat_id: DatId::new(dat.id)`
in `crates/rebellion-data/src/lib.rs`), with no re-encoding. These are small,
plain, per-table sequential numbers, not a bit-packed scheme — don't guess one.

Example, from a real `CAPSHPSD.json` entry:

```json
{ "id": 133, "text_stra_dll_id": 10117, "hull": 2750, "shield_strength": 300, ... }
```

`textstra.json["10117"]` resolves to `"Imperial Star Destroyer"`, confirming
which record this is.

## 2. Which `.DAT` table feeds which overlay filename

Your overlay file's name must match a `GameWorld` field name **exactly** —
there is no aliasing or fuzzy matching. Get it wrong and the mod loader
silently skips the file with an `unknown arena` warning.

| Overlay filename | Source `.DAT` table (for `dat_id` + field lookup) | Notes |
|---|---|---|
| `capital_ship_classes.json` | `CAPSHPSD.DAT` | Star Destroyers, cruisers, etc. |
| `fighter_classes.json` | `FIGHTSD.DAT` | X-wings, TIEs, etc. |
| `characters.json` | `MJCHARSD.DAT` (named) + `MNCHARSD.DAT` (generic) | Both feed the same `characters` arena |
| `troop_classes.json` | `TROOPSD.DAT` | Ground unit stats |
| `defense_facility_classes.json` | `DEFFACSD.DAT` | Planetary shields/defenses |
| `gnprtb.json` | `GNPRTB.DAT` | Global balance constants; overlay `id` selects the matching `parameter_id` entry |
| `sdprtb.json` | `SDPRTB.DAT` | Per-side startup parameters; overlay `id` selects the matching `parameter_id` entry |

The full, authoritative field list for `GameWorld` (and thus every other
possible overlay target) is `crates/rebellion-core/src/world/mod.rs` —
search for `pub struct GameWorld`.

## 3. Directory layout

```
mods/
└── my-mod/
    ├── mod.toml                  # manifest (required)
    └── capital_ship_classes.json # one file per entity category you patch
```

### `mod.toml`

```toml
name = "my-mod"
version = "1.2.0"
author = "you"
description = "What this mod does."

[dependencies]
"some-other-mod" = ">=1.0.0"
```

| Field | Required | Notes |
|---|---|---|
| `name` | Yes | kebab-case, unique across all installed mods |
| `version` | Yes | Semver, e.g. `"1.2.0"` |
| `author` | No | Display only |
| `description` | No | Display only |
| `dependencies` | No | mod name → semver requirement; validated and topologically sorted (Kahn's algorithm) at load time |

## 4. Writing the overlay JSON

Each file is an array of patch objects, merged via **RFC 7396 JSON Merge
Patch**:

```json
[
  { "id": 133, "hull": 3000, "shield_strength": 2000 }
]
```

Rules:
- `"id"` is required and must match the target entity's `dat_id`, or the
  parameter entry's `parameter_id` for `gnprtb.json` and `sdprtb.json`.
- A field you include **overwrites** that field.
- A field set to `null` **deletes** it.
- Any field you omit is **left untouched**.

See the worked example at `mods/examples/star-destroyer-rebalance/` — it
raises the Imperial Star Destroyer's hull to 3000 and shields to 2000.

## 5. Installing and testing

1. Drop your mod directory under `mods/`.
2. Launch the game — mods are auto-discovered on startup.
3. Press **Tab** to open the Mod Manager panel and enable your mod (or edit
   `mods/config.toml` directly: `enabled = ["my-mod"]`).
4. **Native builds only**: editing an enabled mod's files hot-reloads it
   immediately, no restart needed (`ModWatcher`, backed by `notify`). This
   doesn't work in the browser/WASM build yet.

Load order is dependency-first: if mod B depends on mod A, A's patches apply
before B's, and B can override anything A set for the same entity/field.

## 6. Save compatibility

Save files record which mods (name + version) were active when they were
written, as an FNV-1a hash. Loading a save with a different active mod set
prints a mismatch warning — it doesn't block loading, but expect surprises
if the entities the save references have since changed shape.

## Reference

- `agent_docs/modding.md` — manifest/overlay spec
- `agent_docs/mod-runtime.md` — `ModRuntime`/`ModLoader`/`ModWatcher` internals
- `crates/rebellion-data/src/mods.rs` — implementation
- `mods/examples/star-destroyer-rebalance/` — minimal working example
