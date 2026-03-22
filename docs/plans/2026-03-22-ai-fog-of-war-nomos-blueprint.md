---
title: "Nomos Blueprint: AI Fog-of-War Cognitive Architecture (Skotia)"
type: design
status: planned
date: 2026-03-22
designer: Nomos (soul architect subdaimon)
tags: [ai, fog-of-war, cognitive-architecture, design]
---

# AI Fog-of-War Cognitive Architecture — Skotia

## Name and Etymology
**Skotia** (Greek *skotia*, "darkness, obscurity") — the AI's model of what it does not know. Fog-of-war is not about limiting *what the AI sees* but about giving the AI an **epistemology of uncertainty** — a structured model of degrading belief about unseen regions of the galaxy.

## 1. Memory Architecture: `AiFogMemory`

### Core Insight

The existing `FogState` (fog.rs, 373 LOC) answers a binary question: "Has this faction *ever seen* this system?" The AI needs a richer question: **"What does this faction *believe* about this system, and how confident is that belief?"**

### The Belief Record

```rust
struct SystemBelief {
    system: SystemKey,
    last_observed_tick: u64,
    observed_control: ControlKind,
    observed_fleet_strength: u32,
    observed_has_enemy_fleet: bool,
    observed_has_manufacturing: bool,
    observed_popularity: f32,
    confidence: f64,  // 0.0..1.0, decays over time
    source: IntelSource,
}

enum IntelSource {
    DirectObservation,  // fleet physically present
    SensorContact,      // sensor radius — high reliability, no garrison detail
    Espionage,          // mission success — full system reveal
    AlliedReport,       // allied system — control status only
    Stale,              // inference from last known — lowest reliability
}
```

### Confidence Decay Model

**Step function with exponential tail:**

```
confidence(t) = {
    1.0                                           if t < FRESH_WINDOW
    1.0 * e^(-decay_rate * (t - FRESH_WINDOW))    if t >= FRESH_WINDOW
}

where:
    t = current_tick - last_observed_tick
    FRESH_WINDOW = fresh_window_ticks (default: 50)
    decay_rate = ln(2) / memory_decay_ticks (default: 200)
```

Intelligence does not degrade instantly. A fleet you saw 10 days ago is probably still there (the step). After the fresh window, confidence decays with a half-life.

**Source modifiers:**

| Source | Initial Confidence | Decay Multiplier | Rationale |
|---|---|---|---|
| DirectObservation | 1.0 | 1.0x | You were there |
| SensorContact | 0.7 | 1.5x | Detected fleet, not composition |
| Espionage | 0.95 | 1.0x | Mission may have been detected |
| AlliedReport | 0.8 | 1.2x | Ally sees fleet, not full intel |
| Stale | 0.3 | 0.0 (floor) | The "we have no idea" baseline |

### The Full Struct

```rust
struct AiFogMemory {
    faction: AiFaction,
    beliefs: HashMap<SystemKey, SystemBelief>,
    unknown_systems: HashSet<SystemKey>,
    pending_rumors: Vec<IntelRumor>,
    pending_scouts: HashSet<SystemKey>,
    last_sensor_sweep_tick: u64,
}
```

## 2. Decision-Making Under Uncertainty

### Modified Pipeline

```
if fog_enabled:
    decay_all() → update confidence values
    refresh_from_sensors() → direct observation + sensor contacts
    apply_pending_rumors() → espionage intel
    evaluate_galaxy_with_uncertainty() → UncertainGalaxyState
        → mentalQuery gate: "Do I have enough intel to attack?"
        → if NO: prioritize_scouting() → espionage dispatch
        → if YES: score_attack_target_uncertain() → weighted by confidence
        → evaluate_fleet_deployment() (unchanged two-pass)
        → emit AIActions
else:
    existing pipeline unchanged
```

### Four Decision Patterns

1. **Scout-Before-Attack**: Best target has low confidence → dispatch espionage first
2. **Conservative Garrison Sizing**: `estimate = base * (1.0 + (1.0 - confidence) * uncertainty_factor)`
3. **Patrol Routes**: Fleet stations to maximize sensor coverage of border regions
4. **Risk Tolerance**: `risk_tolerance: f64` (0.0=paranoid, 1.0=reckless) — Empire 0.6, Alliance 0.3

### Scoring: 5th Factor

Adds `intel_quality = belief.confidence` as a 5th scoring factor alongside weakness, proximity, deconfliction, and freshness. The AI naturally prefers targets with good intel.

## 3. Sensor Model

| Source | Trigger | Reveals | Confidence |
|---|---|---|---|
| Fleet presence | Own fleet at system | Everything | 1.0 |
| Sensor radius | Fleet within detection * 15.0 | Fleet presence, rough strength | 0.7 |
| Espionage | SystemIntelligenceGathered | Full snapshot | 0.95 |
| Allied system | Own faction controls | Control, own forces | 1.0 |
| No intel | Never observed | Nothing | 0.0 |

**Separation from rendering:** `FogState` → monotonic rendering. `AiFogMemory` → temporal, decaying AI cognition.

## 4. Config Integration

| Field | Type | Default | Description |
|---|---|---|---|
| `fog_enabled` | bool | false | Master toggle (false = omniscient parity) |
| `fresh_window_ticks` | u64 | 50 | Ticks before decay starts |
| `memory_decay_ticks` | u64 | 200 | Half-life of confidence decay |
| `attack_confidence_threshold` | f64 | 0.5 | Min confidence to attack without scouting |
| `garrison_uncertainty_factor` | f64 | 1.5 | Strength inflation at zero confidence |
| `risk_tolerance` | f64 | 0.4 | 0.0=paranoid, 1.0=reckless |
| `scout_priority` | f64 | 0.5 | Weight for scouting vs attacking |
| `confidence_floor` | f64 | 0.05 | Beliefs never decay below this |

## 5. Implementation Phases

| Phase | Description | LOC |
|---|---|---|
| 1 | Data structures: `ai_fog.rs`, `AiFogConfig` in tuning.rs | ~150 |
| 2 | Sensor refresh + decay + tests | ~100 |
| 3 | Uncertain evaluation + modified scoring + strength inflation | ~120 |
| 4 | Scout-or-attack gate + espionage dispatch + patrol routing | ~80 |
| 5 | Autoresearch parameters + eval metrics | ~40 |
| **Total** | | **~490 + tests** |

## 6. Architecture Diagram

```
                    GAME WORLD (ground truth)
                           |
              +────────────+────────────+
              |                         |
         FogSystem                 AiFogMemory
     (rendering, monotonic)    (cognitive, temporal)
              |                         |
              v                         v
      FogState.visible           beliefs HashMap
    "ever seen? yes/no"     "what do I believe? how sure?"
              |                         |
              v                         v
      dim/bright render         UncertainGalaxyState
                                        |
                           +────────────+────────────+
                           |            |            |
                      confident     uncertain     unknown
                      (attack)    (scout first)   (patrol)
```

## 7. Recommendation

Ship omniscient mode now (zero cost, parity with original). Implement Skotia as a v0.16.0 feature — the sweet spot between authenticity and competence. Leave full honest fog as a modding surface or optional hard mode.

*Designed by Nomos, 2026-03-22.*
