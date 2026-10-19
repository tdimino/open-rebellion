---
title: "Rust Implementation Guide: Combat & Mission Systems"
description: "Maps decompiled C++ from REBEXE.EXE to rebellion-core Rust implementation"
category: "ghidra"
created: 2026-03-15
updated: 2026-10-14
---

# Rust Implementation Guide: Combat & Mission Systems

*Maps decompiled C++ from REBEXE.EXE to `rebellion-core` Rust. Written for the Knesset Tiamat swarm.*

**Source notes**: `simulate-runs-tactical-engine.md`, `bombardment.md`, `death-star-strategic.md`, `story-missions.md`, `build-delivery.md`, `record-registry.md`, `character-roster.md`
**Target crate**: `crates/rebellion-core/src/` — zero IO, no rendering deps, all advance() pattern

---

## Index

1. [Game Object Layout → Rust Structs](#1-game-object-layout--rust-structs)
2. [Space Battles](#2-space-battles)
3. [Ground Combat](#3-ground-combat)
4. [Bombardment](#4-bombardment)
5. [Mission System Expansion](#5-mission-system-expansion)
6. [Integration Order](#6-integration-order)

---

## 1. Game Object Layout → Rust Structs

### 1.1 The C++ Entity Base Object

Every game object in REBEXE.EXE shares one heap-allocated C++ object layout. The fields below are read across multiple functions:

```
+0x00   void*   vtable
+0x24   uint    owner_bits           bits 6-7 = side (1 Alliance, 2 Empire)
+0x50   uint    status_flags         bit0=active, bit2=completed, bit3=destroyed, bit4=en route,
                                     bit5=en route active, bit6=existing, bit11=autorouting,
                                     bit12=autoscrap request (build-delivery.md)
+0x60   int     hull_current         current hull (ships); squad size (squadrons)
+0x64   uint    shield_weapon_packed bits 0-3 = shield_recharge_allocated, bits 4-7 = weapon_recharge_allocated
+0x66   short   loyalty_current      characters: base loyalty 0-100
+0x78   uint    role_flags           characters: bit0=decoy, bit4=may resign its mission, bit6=adrift, bit7=on a mission
+0x8a   short   enhanced_loyalty     mission bonus, raw 0-0x7fff, clamped 0-100
+0x96   short   command_kind         characters: officer rank 0 none, 1 Commander, 2 Admiral, 3 General
+0x9a   short   hyperdrive_modifier  mission speed bonus (Han Solo: +hyperdrive_mod)
+0xac   byte    character_flags      bit0=prisoner, bit4=Force aware, bit5=Force sensitive, bit7=discoverer
```

`FUN_00549910`'s `this` is the MissionMgr; its `+0x58` is the story-flag word (`story-missions.md`). On a fleet, `+0x58` bit 6 marks the fleet's bombard order (`bombardment.md`). Officer rank is `character-roster.md`.

### 1.2 What This Means for `CapitalShipClass` and Instance Records

The current `CapitalShipClass` in `world/mod.rs` carries only ~10 of ~50 DAT fields. Combat needs these additions promoted to the world model:

**Add to `CapitalShipClass`** (class template — from CAPSHPSD.DAT):
```rust
// Weapons (4 arcs × 3 types = 12 values; DAT fields already parsed by dat-dumper)
pub weapons_fore_turbolaser: u32,
pub weapons_fore_ion: u32,
pub weapons_fore_laser: u32,
pub weapons_aft_turbolaser: u32,
pub weapons_aft_ion: u32,
pub weapons_aft_laser: u32,
pub weapons_port_turbolaser: u32,
pub weapons_port_ion: u32,
pub weapons_port_laser: u32,
pub weapons_starboard_turbolaser: u32,
pub weapons_starboard_ion: u32,
pub weapons_starboard_laser: u32,

// Defense
pub shield_recharge: u32,       // max shield recharge per round
pub damage_control: u32,        // hull repair rate
pub bombardment: u32,           // CAPSHPSD bombardment strength (see §4)
```

**Add a new `ShipInstance` world type** (runtime per-hull state — replaces the count-only `ShipEntry`):
```rust
/// A single hull of a capital ship class — the runtime instance.
/// Distinct from ShipEntry which is a (class, count) summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipInstance {
    pub class: CapitalShipKey,
    /// Current hull. Starts at CapitalShipClass::hull, reduced by combat.
    pub hull_current: i32,
    /// Packed nibbles: bits 0-3 = shield_recharge_allocated, bits 4-7 = weapon_recharge_allocated.
    /// Mirrors C++ offset +0x64. Read via helpers shield_nibble() / weapon_nibble().
    /// Note: C++ writes via XOR-mask pattern `(new ^ old) & 0xf ^ old` — functionally equivalent
    /// to simple mask-OR for in-range values, but the XOR is what the original binary does.
    pub shield_weapon_packed: u8,
    /// `+0x50` bit 3: destroyed.
    pub destroyed: bool,
    pub faction_is_alliance: bool,
}

impl ShipInstance {
    pub fn shield_nibble(&self) -> u8 { self.shield_weapon_packed & 0x0f }
    pub fn weapon_nibble(&self) -> u8 { (self.shield_weapon_packed >> 4) & 0x0f }
}
```

**Add to `FighterClass`** (template — from FIGHTSD.DAT):
```rust
pub attack: u32,
pub defense: u32,
pub bombardment: u32,
```

### 1.3 GNPRTB Fields Needed in the World Model

Load these GNPRTB values into `GameWorld` at startup rather than hard-coding them:

| GNPRTB | Global / reader | Use | Shipped |
|---|---|---|---|
| `0x1400` (5120) | `DAT_006bb6e8`, `FUN_0055d8c0` | system-distance divisor in transit ticks | 5 in every column |
| `0x600`, `0x601` | `FUN_0055e930`, `FUN_0055e950` | leadership divisors for bombardment defence fire and hits | 40 |
| `0x602`, `0x603` | `FUN_0055e970` | bombardment hit draw | 1..10 |
| `0x604`, `0x605` | `FUN_0058cf80` | energy and raw-material target values | 9 |
| `0x606` | `FUN_0055e9e0` | character hit chance under Destroy System | 50 |
| 7705 | `FUN_0055a0e0` | support shift for the bombarding side | −20 |

`DAT_00661a88` is the percentage base 100 of `FUN_0053e170` / `FUN_0053e190` (`s * x / 100`); no difficulty modifier enters the transit or bombardment arithmetic (`build-delivery.md`, `bombardment.md`).

---

## 2. Space Battles

Every space battle runs the tactical engine, `crates/rebellion-core/src/tactical.rs`:
played in its window, simulated from the Battle Alert, or headless when no
one watches (`FUN_0040a050` state 8). The headless step (`FUN_005ae9c0`)
ranks opponents, assigns targets and applies each group's damage rate once a
second; the 1200 s stalemate rule ends every battle. See
`simulate-runs-tactical-engine.md`, `tactical-engagement-ai.md` and
`tactical-weapon-loop.md`.

`FUN_00549910` and the flag word at `+0x58` are the MissionMgr story checks
(`story-missions.md`) and apply no combat damage.

| Step | Port |
|---|---|
| Both sides present (`FUN_00509710`, `FUN_0050b8e0`) | `combat::side_present` |
| One session per contested system (`FUN_00516ef0` seats every fleet and the system's squadrons) | `BattleSession::new_with_forces` |
| Headless run | `BattleSession::simulate_to_end` |
| Write-back (`FUN_0040cfc0`, `FUN_005032c0`) | `tactical::apply::apply_results` |
| A battle nobody watches | `rebellion-data` `integrator::resolve_system_space_combat` |

### 2.1 Family Bytes

The DAT headers give these DatId family bytes (`record-registry.md`, Appendix below):

- capital ships `0x14`-`0x1b` (CAPSHPSD), the Death Star by its record's family `0x18`;
- fighters `0x1c`-`0x1f`, troops `0x10`-`0x13`, special forces `0x3c`-`0x3f`;
- major characters `0x30`-`0x37` (`0x34` Palpatine), minor characters `0x38`-`0x3b`;
- fleets `0x08`-`0x0f`; missions `0x40`-`0x7f` (`0x71` Dagobah, `0x73` Pickup).

The original identifies a Death Star by a ship in `[0x18,0x1c)` (`FUN_005003d0`, `FUN_00582440`, `FUN_0058d440`; `death-star-strategic.md`). Task-graph node `or-port-death-star-family-qtde` moves the port's `is_death_star` onto it.

`extract_difficulty` reads `+0x24 >> 4 & 3`, which `FUN_0054a1d0` (a story-flag broadcaster) passes to `FUN_004fd600` to pick the Alliance (1) or Empire (2) view; it is a side selector.

---

## 3. Ground Combat

The original resolves a planetary assault as order `0x234` in `0x0058c120`-`0x0058c8b0` (`rebexe-map.md`). Where regiment-versus-regiment combat is resolved, and which class fields it reads (attack `+0x64`, defense `+0x68`, bombardment defense `+0x60`), is open: task-graph nodes `or-q-ground-resolution-nms` and `or-q-regiment-combat-eaxn`. Epic `or-ep-gnd-mmf` and node `or-p-gnd-ykv` port the original's report-only assault and remove the port's `resolve_ground`, whose troop loop and `regiment_strength` rest on `FUN_00560d50`.

`FUN_00560d50` is the character manager's move hook, run by the object-move dispatcher `FUN_00514a60` (event `0x320`). A moved capital ship (`0x14`-`0x1b`) clears the CommandKind (`+0x96`) of every character aboard through `FUN_004ee350`; a moved character (`0x30`-`0x3b`) keeps its CommandKind only between two capital ships of the same fleet. For the Emperor (family `0x34`) it also calls `FUN_005617b0`, which sets the Emperor-at-Coruscant flag (`FUN_0055f650`, event `0x230` `SeatOfPower`) when Palpatine is not a prisoner, is active and sits at Empire-held Coruscant `0x90000109` (`ground-unit-assets.md`, `character-roster.md`, `story-encounters.md` row 7a).

---

## 4. Bombardment

### 4.1 The Original's Bombardment

Orders `0x220`-`0x223` pass the gate `FUN_00582440` and run
`FUN_0058d800`: a Death Star phase for Destroy System, KDY/LNR defence fire
at the bombarding ships, per-hit target rolls against each target's value,
and support shifts (`bombardment.md`). The chain
`FUN_00556430` → `FUN_00555d30` → `FUN_00555b30` → `FUN_0055d8c0` is the
transit-time distance (`build-delivery.md`), and `FUN_00509620` returns a
system's coordinates. The port's `bombardment.rs` still uses that distance;
task-graph node `or-port-bombardment-8usw` replaces it.

## 5. Mission System Expansion

### 5.1 All 9 Mission Types as an Enum

The current `MissionKind` in `missions.rs` only covers `Diplomacy` and `Recruitment`. MISSNSD.DAT defines 25 mission records; COMBAT-SUMMARY.md and the DAT format notes confirm at minimum these mission type codes:

| Code | Mission | *MSTB File | Skill Used |
|------|---------|------------|------------|
| — | Diplomacy | DIPLMSTB.DAT | `diplomacy` |
| — | Recruitment | RCRTMSTB.DAT | `leadership` |
| 6 | Sabotage | SBTGMSTB.DAT | `espionage` |
| 7 | Assassination | ASSNMSTB.DAT | `combat` |
| — | Espionage | ESPIMSTB.DAT | `espionage` |
| — | Rescue | RESCMSTB.DAT | `combat` |
| — | Abduction | ABDCMSTB.DAT | `espionage` |
| — | Incite Uprising | INCTMSTB.DAT | `diplomacy` |
| 21 | Autoscrap | — | — (auto, no character) |

```rust
// Extend MissionKind in crates/rebellion-core/src/missions.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissionKind {
    // --- Implemented ---
    Diplomacy,
    Recruitment,

    // --- War Machine phase ---
    /// Destroy an enemy facility. SBTGMSTB.DAT. Code 6.
    Sabotage,
    /// Kill an enemy character. ASSNMSTB.DAT. Code 7.
    Assassination,
    /// Gather intelligence on an enemy system. ESPIMSTB.DAT.
    Espionage,
    /// Free a captured character. RESCMSTB.DAT.
    Rescue,
    /// Capture an enemy character. ABDCMSTB.DAT.
    Abduction,
    /// Trigger a planetary uprising. INCTMSTB.DAT.
    InciteUprising,
    /// Automatic scrapping of obsolete units. Code 21. No character required.
    Autoscrap,
}
```

### 5.2 Extending `coefficients()` for New Mission Types

Each `*MSTB.DAT` file is an `IntTableEntry` table (16B per entry): `id`, `field2`, `threshold` (i32 skill delta), `value` (probability 0-100). The quadratic formula from rebellion2 still applies — the coefficients are fit from these tables.

Until the *MSTB tables are fit, use placeholders that match the structural pattern:

```rust
impl MissionKind {
    pub fn coefficients(self) -> (f64, f64, f64) {
        match self {
            // Existing (from rebellion2 Mission.cs):
            MissionKind::Diplomacy    => (0.005558,  0.7656,  20.15),
            MissionKind::Recruitment  => (-0.001748, 0.8657,  11.923),

            // Placeholders — fit from *MSTB.DAT IntTableEntry tables:
            MissionKind::Sabotage     => (-0.002,    0.75,    15.0),  // SBTGMSTB.DAT
            MissionKind::Assassination=> (-0.003,    0.80,    10.0),  // ASSNMSTB.DAT
            MissionKind::Espionage    => (-0.002,    0.78,    12.0),  // ESPIMSTB.DAT
            MissionKind::Rescue       => (-0.002,    0.72,    10.0),  // RESCMSTB.DAT
            MissionKind::Abduction    => (-0.002,    0.70,     8.0),  // ABDCMSTB.DAT
            MissionKind::InciteUprising=>(-0.003,    0.65,    18.0),  // INCTMSTB.DAT
            MissionKind::Autoscrap    => (0.0,       0.0,   100.0),  // always succeeds
        }
    }

    /// Which character skill score determines success probability.
    pub fn skill(self) -> fn(&crate::world::Character) -> u32 {
        match self {
            MissionKind::Diplomacy      => |c| c.diplomacy.base,
            MissionKind::Recruitment    => |c| c.leadership.base,
            MissionKind::Sabotage       => |c| c.espionage.base,
            MissionKind::Assassination  => |c| c.combat.base,
            MissionKind::Espionage      => |c| c.espionage.base,
            MissionKind::Rescue         => |c| c.combat.base,
            MissionKind::Abduction      => |c| c.espionage.base,
            MissionKind::InciteUprising => |c| c.diplomacy.base,
            MissionKind::Autoscrap      => |_| 100, // always max
        }
    }
}
```

### 5.3 *MSTB Probability Tables as Rust Lookup Tables

The `IntTableEntry` format: `threshold` is a signed skill delta; `value` is probability at that threshold. These should be loaded from DAT at startup and used for linear interpolation, replacing the fitted quadratic coefficients eventually.

```rust
/// One entry in a *MSTB probability table.
/// Mirrors the IntTableEntry layout (16B): id, field2, threshold(i32), value(u32).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MstbEntry {
    pub threshold: i32,  // signed skill delta (negative = below average)
    pub value: u32,      // probability 0-100 at this threshold
}

/// A complete *MSTB table for one mission type.
/// Loaded from DIPLMSTB.DAT, SBTGMSTB.DAT, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MstbTable {
    pub entries: Vec<MstbEntry>,
}

impl MstbTable {
    /// Interpolate probability for a given skill score.
    /// Skill score is mapped to a threshold delta before lookup.
    pub fn lookup(&self, skill_score: u32) -> f64 {
        // Convert skill score (0-100) to threshold delta relative to midpoint (50).
        let threshold = skill_score as i32 - 50;

        // Find bracketing entries and linearly interpolate.
        let entries = &self.entries;
        if entries.is_empty() { return 50.0; }

        // Clamp to table bounds.
        if threshold <= entries.first().unwrap().threshold as i32 {
            return entries.first().unwrap().value as f64;
        }
        if threshold >= entries.last().unwrap().threshold as i32 {
            return entries.last().unwrap().value as f64;
        }

        // Linear interpolation between bracketing entries.
        for window in entries.windows(2) {
            let lo = &window[0];
            let hi = &window[1];
            if threshold >= lo.threshold && threshold <= hi.threshold {
                let t = (threshold - lo.threshold) as f64
                    / (hi.threshold - lo.threshold) as f64;
                return lo.value as f64 + t * (hi.value as f64 - lo.value as f64);
            }
        }

        50.0 // fallback
    }
}
```

Add to `GameWorld`:
```rust
/// Mission probability tables, one per MissionKind.
/// Loaded from *MSTB.DAT files by rebellion-data.
pub mission_tables: std::collections::HashMap<String, MstbTable>,
```

### 5.4 Extending `MissionEffect` for Combat Mission Outcomes

Current `MissionEffect` only covers `PopularityShifted` and `CharacterRecruited`. Add:

```rust
// In missions.rs MissionEffect enum:

/// A facility was sabotaged — reduce its remaining production ticks.
FacilitySabotaged {
    system: SystemKey,
    /// Index of the facility in system.manufacturing_facilities (or defense_facilities).
    facility_index: usize,
    /// Ticks of production lost.
    ticks_lost: u32,
},

/// A character was assassinated — remove from game.
CharacterKilled {
    character: CharacterKey,
    faction: MissionFaction,
},

/// A character was captured by the opposing faction.
CharacterCaptured {
    character: CharacterKey,
    captured_by: MissionFaction,
    at_system: SystemKey,
},

/// Intelligence gathered on a system — reveal fog of war.
SystemIntelligenceGathered {
    system: SystemKey,
    faction: MissionFaction,
},

/// Uprising started on a system — shift popularity.
UprisingStarted {
    system: SystemKey,
    /// Popularity shift applied immediately.
    popularity_delta: f32,
},
```

---

## 6. Integration Order

`rebellion-data/src/simulation.rs` runs combat after movement each tick
(`agent_docs/simulation.md`), and `rebellion-app` `main.rs` does the same for
battles no window opens:

1. Space: for each system where both sides are present,
   `integrator::resolve_system_space_combat(world, troop_transport,
   movement, system, rng_seed, tick)` runs one headless tactical session
   seeded from one roll, then the caller reconciles a destroyed Death Star
   (`tactical::apply::reconcile_death_star_result`).
2. Ground: `CombatSystem::resolve_ground(&world, system, alliance_attacks,
   &rolls, tick)`, applied by `apply_ground_combat_result_inner`; node
   `or-p-gnd-ykv` replaces it with the original's assault (§3).
3. Bombardment: `BombardmentSystem::resolve_bombardment`, the port formula;
   node `or-port-bombardment-8usw` replaces it with `FUN_0058d800` (§4).

---

## Appendix: Entity Family Byte Quick Reference

DAT header ranges (`record-registry.md`):

| Family Range | Entity Type | Families in use |
|---|---|---|
| `0x08-0x0f` | Fleets | executable-made |
| `0x10-0x13` | Troop regiments (TROOPSD) | `0x10` |
| `0x14-0x1b` | Capital ships (CAPSHPSD) | `0x14`; `0x18` Death Star (record 136) |
| `0x1c-0x1f` | Fighter squadrons (FIGHTSD) | `0x1c` |
| `0x20-0x21` | Alliance HQ (ALLFACSD) | `0x20` |
| `0x22-0x27` | Defence facilities (DEFFACSD) | `0x22`-`0x25` |
| `0x28-0x2b` | Manufacturing facilities (MANFACSD) | `0x28`-`0x2a` |
| `0x2c-0x2f` | Production facilities (PROFACSD) | `0x2c`, `0x2d` |
| `0x30-0x37` | Major characters (MJCHARSD) | `0x30`-`0x35` (`0x34` Palpatine) |
| `0x38-0x3b` | Minor characters (MNCHARSD) | `0x38` |
| `0x3c-0x3f` | Special forces (SPECFCSD) | `0x3c` |
| `0x40-0x7f` | Missions (MISSNSD) | 22 kinds (`0x71` Dagobah, `0x73` Pickup) |
| `0x80-0x8f` | Sectors (SECTORSD) | `0x80` |
| `0x90-0x97` | Star systems (SYSTEMSD) | `0x90`, `0x92` |
