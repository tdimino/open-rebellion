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

### Stage the Encyclopedia source profile

The production Encyclopedia uses the supported owned English `ENCYTEXT.DLL`
and `ENCYBMAP.DLL` profile plus the selected DAT files. Keep all generated
source data and original artwork ignored:

```bash
OWNED_INSTALL="/path/to/owned-install"
SOURCE_JSON="$PWD/data/base/encyclopedia/source.json"

go run ./tools/stage-ui-assets --encyclopedia-only \
  --source "$OWNED_INSTALL" \
  --encyclopedia-output "$SOURCE_JSON"

# Read-only: rechecks the JSON and sidecar without reopening the DLLs.
go run ./tools/stage-ui-assets --encyclopedia-only --verify \
  --encyclopedia-output "$SOURCE_JSON"

# data/base must already contain the selected DAT/TEXTSTRA/UI inputs.
REBELLION_EDATA_DIR="$OWNED_INSTALL/EData" bash scripts/build-wasm.sh
REBELLION_EDATA_DIR="$OWNED_INSTALL/EData" bash scripts/package-web.sh dev
```

The stage writes `source.json` and `source.json.manifest.json`. The sidecar is
generated provenance: do not edit hashes to repair a mismatch. Restage from
the same owned DLL/DAT/EData profile. A differing generated output requires
the explicit `--force` flag; verification is read-only. Browser production
builds require the complete canonical source and exact referenced EData,
whereas browser mods remain deliberately unsupported.

An Encyclopedia overlay selects the compound catalog object ID, not a slotmap
key. For ordinary DAT records the ID is `(family_id << 24) | id`; if the DAT
ID already has a high byte, retain it unchanged. After finding `family_id` and
`id` in the corresponding JSON reference dump, this prints the selector:

```bash
FAMILY_ID=0x14
DAT_ID=64
printf '0x%08x\n' "$(( (FAMILY_ID << 24) | DAT_ID ))"
```

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

### Encyclopedia text and artwork overlays

Native mods reserve one presentation-only root file, `encyclopedia.json`; it
never enters `GameWorld`, saves, replay, or simulation fingerprints:

```text
mods/my-encyclopedia-mod/
├── mod.toml
├── encyclopedia.json
└── encyclopedia/
    └── assets/
        └── cruiser.bmp
```

```json
[
  {
    "id": 335544384,
    "title": "Contributor cruiser",
    "body": "Contributor-authored replacement text.",
    "image": {"path": "encyclopedia/assets/cruiser.bmp"}
  }
]
```

The default action is `patch`: absent fields inherit the lower layer, values
replace it, and `null` requests removal where the complete effective topic
remains valid. Empty title/body strings are invalid. `replace` requires title,
body, and image; `add` also requires a new supported non-system object ID and a
unique nonzero `text_resource_id`; `remove` accepts only `id`. Existing text
resource IDs are immutable.

Artwork paths must stay below `encyclopedia/assets/`, contain only safe ASCII
segments, and end in lowercase `.bmp`. Files must be regular, non-symlinked,
400-by-200 uncompressed 8-bit indexed BMPs. The runtime derives hashes and the
`mod:v1:...` identity from the retained bytes; authors do not provide either.
Encyclopedia mod names must be 1–128 ASCII letters, digits, `.`, `_`, or `-`.

The base source manifest remains immutable. Enabled overlays apply in the
shared dependency-first order; unrelated ready mods use lexical name order and
later layers win per field. A malformed JSON file, missing artwork, unsafe
path, dependency error, or invalid final session rejects the whole candidate
and keeps the last-known-good publication. Fix the files and choose **Reload
Mods** to retry. Disabling the mod rebuilds from the immutable base plus the
remaining enabled layers, restoring original content when none remain.

The supported base profile is English language 1033. This overlay format does
not invent per-language records or aliases. Browser builds consume the
unmodified staged base and do not discover native mod directories. The lower
level loader has an explicit catalog-root adapter, but the application does
not read a `REBELLION_ENCYCLOPEDIA_DIR` environment override; use the selected
DAT root's implemented `encyclopedia/source.json` layout.

## 5. Installing and testing

1. Drop your mod directory under `mods/`.
2. Launch the game — mods are auto-discovered on startup.
3. Press **Tab** to open the Mod Manager panel and enable your mod (or edit
   `mods/config.toml` directly: `enabled = ["my-mod"]`).
4. **Native builds only**: world overlays can hot-reload through `ModWatcher`.
   After editing `encyclopedia.json` or its artwork, choose **Reload Mods** so
   the complete candidate is reacquired and validated atomically. Browser/WASM
   builds do not load filesystem mods.

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
