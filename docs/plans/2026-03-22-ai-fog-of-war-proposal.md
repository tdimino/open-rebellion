---
title: "AI Fog of War — Research and Proposal"
description: "Research findings and implementation proposals for true fog of war on the AI in Open Rebellion"
category: "proposal"
created: 2026-03-22
status: draft
---

# AI Fog of War — Research and Proposal

## Executive Summary

The original Star Wars Rebellion (1998) AI is confirmed omniscient. Ghidra RE of `FUN_00519d00` shows the galaxy evaluation function iterates ALL systems via `FUN_0051cad0(0x90)` with no visibility filter—the AI sees every system, every fleet, every garrison in the galaxy. Our implementation faithfully replicates this: `evaluate_galaxy_state()` iterates `world.systems.iter()` without consulting `FogState`.

This document proposes three approaches for handling AI visibility, ranging from faithful omniscience (status quo with a config flag) to honest fog with memory decay. It draws on research into how other strategy games handle this problem.

---

## Part I: Industry Research — How Other Games Handle AI Fog of War

### The Universal Truth: Almost Everyone Cheats

The overwhelming finding is that **nearly every major 4X and strategy game gives the AI some form of map omniscience**. This is not laziness—it is a conscious design choice born from a fundamental asymmetry: humans have spatial intuition, pattern recognition, and strategic inference that rule-based AI cannot replicate.

#### Civilization Series (I through VII)

**Verdict: The AI cheats with information, always has.**

Civilization's AI has had full map knowledge since at least Civ III (2001). CivFanatics forum analysis and source code inspection confirm:

- The AI knows the locations of all cities, units, and wonders regardless of explored territory
- On higher difficulty levels, the AI receives explicit resource bonuses (production, gold, science) on top of its information advantages
- Civ VI introduced diplomatic visibility levels (delegation, embassy, spy) that give the *player* graduated information access, but the AI already has this information natively
- Sid Meier himself acknowledged the challenge: he omitted multiplayer alliances from Civ I because the AI was "almost as good as humans in using them, which caused players to think that the computer was cheating" (Computer Gaming World, 1993)

The Civ approach is instructive: **information cheating is invisible to most players, resource cheating is what they notice.** Players rarely catch the AI reacting to hidden information but immediately notice when it builds a Wonder faster than should be possible.

#### Stellaris (Paradox, 2016+)

**Verdict: Cheated originally, then introduced a formal intel system.**

Pre-Nemesis (2021), Stellaris AI had full map knowledge. The Nemesis expansion introduced a tiered intel system:

- **Intel levels** (0-100) determine what information is visible about an empire: 10 = borders only, 30 = basic demographics, 50 = fleet power, 70 = technology, 100 = full internal state
- **Espionage networks** allow active intelligence gathering, with spy operations that can raise or lower intel levels
- **Fog of war** covers unexplored systems with actual visual fog

However, community consensus is that the *tactical* AI still cheats with fleet movement:

> "The AI can see everything you do, they do not get fog of war. They are supposed to 'pretend' they don't know, but the tactical AI always knows where your ships are." —Paradox Forums, 2019

This is the **pretend-not-to-know** pattern: the AI has full information but is coded to respond only to information it "should" have. It is notoriously difficult to tune correctly.

#### Hearts of Iron IV (Paradox, 2016+)

**Verdict: Added simulated fog post-launch; still partially cheats.**

HOI4 is the most relevant Paradox title because it introduced a **shared fog simulation** among all enemies of a country:

> "AI now has a Fog of War simulation (shared among all enemies of a country), so they shouldn't chase hiding armies/fleets that haven't shown themselves as much." —Paradox developer post

Key design: all AI factions hostile to a player share a single "simulated fog" layer. If *any* enemy faction can see a player unit, all of them know about it. This is cheaper than per-faction fog while still preventing the most egregious omniscient behaviors (chasing hidden fleets, avoiding ambushes).

#### Total War Series (Creative Assembly, 2000+)

**Verdict: The AI absolutely cheats with vision.**

Community evidence is overwhelming:

- AI armies avoid ambushes they should not know about
- AI reacts to player army movements behind fog of war
- The fog of war "appears to benefit specific AI factions"—some perform dramatically worse when given honest fog via modding

The Total War case demonstrates the **performance cliff**: when modders force honest fog on the AI, it becomes dramatically worse because its decision-making was never designed for incomplete information.

#### StarCraft / StarCraft II (Blizzard, 1998/2010)

**Verdict: Standard AI cheats; competitive AI explicitly labeled.**

StarCraft II is notable for being transparent about it:

- Difficulty levels below "Cheater 1" use some fog (contested—likely partial)
- "Cheater 1 (Vision)" explicitly gives the AI full map visibility
- "Cheater 2" and "Cheater 3" add resource bonuses on top

The BWAPI community (StarCraft: Brood War bot competition) has produced the most sophisticated academic work on AI under honest fog. Key approaches from the literature:

1. **Particle-model position prediction** (Weber, Mateas, Jhala 2011): Track last-known positions of enemy units, predict movement using per-unit-type models learned from replays. Confidence decreases over time.

2. **Influence maps** (Hagelbäck and Johansson 2008): Propagate threat/opportunity fields from known positions. Gives the AI a "sense" of where danger is without exact knowledge.

3. **Plan recognition under incomplete information** (Kabanza et al. 2010): Match observed actions against known strategy templates. Predict enemy intent from partial observations.

#### Master of Orion (1993-2003)

**Verdict: Cheats, but the game's simpler decision space makes it less noticeable.**

The original MoO AI had full galaxy knowledge. MoO2 added espionage as a gameplay mechanic, but the AI's internal decisions were still based on complete information.

### The Kohan II Approach (Game AI Pro)

The most directly applicable industry case study comes from Kevin Dill's chapter in *Game AI Pro 2*, describing Kohan II: Kings of War (2004). This is the only published account I found of a shipped strategy game that successfully used influence maps to create the **illusion of honest fog** while technically cheating:

- The AI uses influence maps to evaluate threats and opportunities
- Enemy units beyond fog contribute *reduced* influence (not zero, not full)
- The result: the AI responds proportionally to threats without obviously cheating
- Reviewers described it as having a "noncheating AI" even though it was not strictly honest
- Players could replay games from any viewpoint and never detected the cheat

Dill's key insight: **"Is it to build an AI that doesn't cheat or is it to provide a compelling experience for the player?"** The influence map approach produced emergent flanking, diversionary attacks, and adaptive defense—not because it was programmed to do those things, but because responding to spatial pressure gradients naturally produces them.

### The Clemens Winter Approach (CodeCraft / Deep RL)

A modern academic approach from Clemens Winter's CodeCraft RL agent (2021) demonstrates a clean architectural split:

- **Policy (what the agent does)**: Sees only objects within sensor radius. Uses "last known values" for objects that were previously visible but are now in fog. Essentially: snapshot at last observation, then act on stale data.
- **Value function (how it trains)**: Sees the full game state (omniscient). This asymmetry—honest policy, omniscient value function—dramatically improves training stability without giving the agent unfair runtime behavior.

The "last known position" pattern is directly relevant: the agent "will often move to and stay at the last known position of an enemy drone. When playing against itself during training, this strategy works quite well."

---

## Part II: What This Means for Open Rebellion

### The Original Was Omniscient By Design

The Ghidra RE is unambiguous. In `FUN_00519d00`, the galaxy evaluation iterates ALL systems—no visibility check, no fog parameter, no sensor range calculation. The AI sees:

- Every enemy fleet and its exact composition
- Every enemy system's garrison strength (ships + troops + facilities)
- The enemy HQ location
- All neutral/unoccupied systems

This was deliberate. In 1998, with ~200 star systems and rule-based AI on a Pentium 133, LucasArts chose omniscience because:

1. The AI was already struggling with its limited decision space
2. The sensor/probe droid mechanics existed for the *player* as a gameplay feature, not as an AI constraint
3. Strategy games of that era universally gave AI full map knowledge

### The Consequence of Honest Fog

If we naively restrict the AI to only seeing systems within its `FogState.visible` set, the AI would:

1. **Never attack systems it hasn't scouted** — reducing galactic conquest to a handful of border skirmishes
2. **Not know where the player's HQ is** — the Empire's core strategy (find and destroy the Rebel base) would be crippled
3. **Ignore threats it can't see** — allowing the player to mass fleets out of sensor range and alpha-strike with impunity
4. **Make no strategic decisions about most of the galaxy** — with ~200 systems and sensor radius covering maybe 20-30, the AI would be blind to 85% of the map

The original game's Empire AI is explicitly designed around the narrative of *hunting the hidden Rebel base*. The AI already knows where it is—the gameplay mechanic of probes and espionage exists to make the *player* feel like they're being hunted.

---

## Part III: Three Proposed Approaches

### Approach A: Faithful Omniscience with Config Flag

**Philosophy**: Ship what the original shipped. The AI is omniscient. Add a config flag for future use.

**Changes**:
- Add `ai.fog_mode: "omniscient"` to `AiConfig` / `GameConfig`
- Document in the AI parity tracker that this is `FAITHFUL` to the original
- No changes to `AISystem::advance()` or `evaluate_galaxy_state()`

**Pros**:
- Zero risk. The original game was fun despite (because of?) omniscient AI
- No performance cost
- Maintains parity with the original
- The simplest approach by far

**Cons**:
- Not a differentiator. Any fan remake could do this.
- Misses an opportunity to make the AI feel more "alive" and human-like
- Players who discover the omniscience (via JSONL telemetry) may feel cheated

**Estimated effort**: 5 LOC (config field + doc comment)

---

### Approach B: Influence-Weighted Fog (The Kohan II Pattern)

**Philosophy**: The AI sees everything but *trusts* what it sees proportionally to its intelligence quality. Systems within sensor range are evaluated at full fidelity. Systems outside sensor range are evaluated with degraded accuracy—the AI knows *something* is there but not exactly what.

**How it works**:

1. **AI Fog Layer**: Create an `AiFogState` per AI faction. Unlike player `FogState` (binary visible/not-visible), this stores a continuous **intel quality** per system (0.0 = unknown, 1.0 = perfect knowledge).

2. **Intel quality sources**:
   - Fleet sensor radius: 1.0 at fleet location, decaying with distance (same formula as player fog: `detection * 15.0` coordinate units)
   - Espionage missions: Successful espionage against a system raises its intel quality for N ticks
   - Probe droids (future): Expendable scouts that raise intel quality in a region
   - Decay: Intel quality for a system decays by some rate per tick when no active source covers it

3. **Modified evaluation**: `evaluate_galaxy_state()` still iterates all systems but filters the information it trusts:
   - `intel >= 0.8`: Full knowledge (fleet composition, garrison strength, control status)
   - `intel >= 0.4`: Partial knowledge (knows control status and "strong"/"weak"/"empty" garrison estimate)
   - `intel >= 0.1`: Vague knowledge (knows it's enemy-controlled but not strength)
   - `intel < 0.1`: AI treats system as "unknown"—will not target it for attack but also won't assume it's safe

4. **Modified attack scoring**: `score_attack_target()` applies a confidence multiplier based on intel quality. Low-intel targets get a large uncertainty penalty, making the AI prefer to attack systems it knows about.

5. **Scout behavior**: If the AI has many "unknown" systems (early game, or after losing sensor coverage), it dispatches probe missions or moves small fleets to establish sensor coverage before committing to major attacks.

**Changes to architecture**:

```
New type: AiFogState {
    faction: AiFaction,
    intel: HashMap<SystemKey, f32>,       // 0.0-1.0 intel quality
    last_seen: HashMap<SystemKey, u64>,   // tick of last observation
    last_known: HashMap<SystemKey, SystemSnapshot>, // cached state at last observation
}

New type: SystemSnapshot {
    control: ControlKind,
    garrison_estimate: GarrisonTier,  // Unknown, Empty, Light, Medium, Heavy
    fleet_count: u32,
    tick: u64,
}

Modified: AISystem::advance() takes &mut AiFogState
Modified: evaluate_galaxy_state() filters by intel quality
Modified: score_attack_target() includes intel confidence factor
New: evaluate_scouting() — dispatch probes/scouts to low-intel regions
New config: ai.fog_mode: "influence" | "omniscient" | "honest"
New config: ai.intel_decay_rate: f32 (per-tick decay when not covered)
New config: ai.min_attack_intel: f32 (minimum intel to consider attacking)
```

**Pros**:
- Produces emergent "smart" behavior: AI scouts before attacking, prefers known targets, responds proportionally to threats
- Not exploitable by the player (the AI still has *some* awareness of the whole galaxy, just degraded)
- The Kohan II precedent proves this works in a shipped game and produces positive player reviews
- Config-driven: can be tuned to anywhere between full omniscience and near-honest fog
- Compatible with the existing FogState infrastructure (reuses sensor radius formula)

**Cons**:
- Moderate implementation effort (~200-300 LOC)
- Requires tuning: if intel decays too fast, AI becomes passive; too slow, it's effectively omniscient
- `SystemSnapshot` introduces stale data the AI must reason about, adding complexity
- The AI may need a dedicated scouting phase, which adds another heuristic module

**Estimated effort**: ~300 LOC in ai.rs + ~100 LOC new fog types + config additions

---

### Approach C: Honest Fog with Memory and Inference

**Philosophy**: The AI operates under the same fog rules as the player. It can only see systems within its sensor radius and fleets in transit. What it cannot see, it must remember, infer, or scout.

**How it works**:

1. **Per-faction AI FogState**: The AI faction gets its own `FogState` (or shares the player's if the AI controls the same faction in dual-AI mode). Only systems in `visible` are used for evaluation.

2. **Memory layer**: When a system leaves sensor coverage, the AI retains a `SystemSnapshot` of what it last saw there. This snapshot decays:
   - **Fresh** (0-50 ticks since last seen): AI trusts the data fully
   - **Stale** (50-200 ticks): AI assumes garrison could be ±30% different from last observed
   - **Ancient** (200+ ticks): AI only remembers control status, not garrison strength

3. **Inference engine**: The AI uses heuristics to fill gaps:
   - If the player has been producing ships for 100 ticks without any new fleets appearing in sensor range, infer that those ships are massing somewhere unseen
   - If a system was enemy-controlled 200 ticks ago and no battles occurred nearby, assume it still is
   - If the AI loses sensor coverage of a region, increase threat estimate for that region over time (the "fog of uncertainty" grows)

4. **Scouting doctrine**: The AI *must* actively scout:
   - Small fast fleets (single corvette with high detection) dispatched to low-intel regions
   - Espionage missions used for strategic intelligence (where is the enemy HQ?)
   - The Empire's narrative-canonical use of probe droids to find the Rebel base becomes a real gameplay mechanic, not theater

5. **Modified galaxy evaluation**: `evaluate_galaxy_state()` only iterates systems in the AI's visible set plus memory. The `GalaxyState` buckets now have a confidence flag per entry.

**Changes to architecture**:

```
New type: AiMemory {
    snapshots: HashMap<SystemKey, SystemSnapshot>,
    inferred_threats: HashMap<SystemKey, f32>,
    total_enemy_production_estimate: u32,
    last_scout_tick: HashMap<SystemKey, u64>,
}

Modified: AISystem::advance() takes &FogState, &mut AiMemory
Modified: evaluate_galaxy_state() uses fog.visible + memory.snapshots
New: evaluate_scouting() — scout dispatch with priority scoring
New: update_inference() — inference engine runs each AI tick
New: decay_memory() — stale snapshot degradation
New config: ai.fog_mode: "honest"
New config: ai.memory_fresh_ticks: u64
New config: ai.memory_stale_ticks: u64
New config: ai.inferred_threat_growth_rate: f32
```

**Pros**:
- Genuinely novel. No other Rebellion fan project or 1998-era strategy game does this.
- Creates emergent gameplay: the AI can be ambushed, deceived, surprised
- The Empire "hunting the Rebel base" narrative becomes mechanically real
- Scouting becomes a real AI activity, making the galaxy feel alive
- Dual-AI mode becomes genuinely interesting: two imperfect-information AIs fighting each other
- Aligns with the academic state of the art (particle models, influence maps, plan recognition)

**Cons**:
- Hard. Probably the single hardest AI feature we could implement.
- The AI will be weaker—possibly much weaker—than the omniscient version without extensive tuning
- The inference engine is a research problem, not an engineering problem. Getting it "right" requires iteration.
- Scouting AI is itself a nontrivial subproblem (where to scout, how often, with what)
- Memory management adds state complexity and potential save/load issues
- Risk of the Total War failure mode: an AI that seems stupid because it doesn't react to things the player expects it to know about

**Estimated effort**: ~500-800 LOC, significant tuning iteration

---

## Part IV: Recommendation

### Ship Approach A Now, Build Toward B

**Phase 1 (v0.15.0)**: Add the `ai.fog_mode` config field with `"omniscient"` as default. Document the design decision. This costs nothing and establishes the extension point.

**Phase 2 (v0.16.0 or later)**: Implement Approach B (influence-weighted fog). This is the sweet spot:

- It is implementable in ~300-400 LOC
- It produces noticeably better AI behavior without the risk of catastrophic weakness
- It is tunable via config, so autoresearch can optimize the parameters
- It reuses the existing sensor-radius infrastructure from `FogSystem`
- It does not require solving the scouting AI problem (though it benefits from it)
- The Kohan II precedent gives confidence that this produces a good player experience

**Phase 3 (future/modding)**: Expose Approach C as an optional "hard mode" or leave it as a modding extensibility surface. The honest fog approach is fascinating but risky for the default experience. It could be a great feature for a "Total Conversion" or difficulty mode.

### The Design Principle

The original game was omniscient because LucasArts needed the AI to be competent, and omniscience was the only practical way to achieve that in 1998. We should respect that constraint while recognizing that we have tools they did not: config-driven tuning, autoresearch optimization, influence maps, and the benefit of 28 years of game AI research.

The right default is an AI that *appears* to respect fog of war but is not crippled by it—the Kohan II illusion. Players should feel that the AI earned its knowledge through sensors and scouts, even if the underlying system is more forgiving than pure honest fog.

---

## Part V: Implementation Details for Approach B

### Changes to `AISystem::advance()`

The current signature:

```rust
pub fn advance(
    state: &mut AIState,
    world: &GameWorld,
    mfg_state: &ManufacturingState,
    _mission_state: &MissionState,
    movement: &crate::movement::MovementState,
    tick_events: &[TickEvent],
    config: &GameConfig,
    research_state: &ResearchState,
) -> Vec<AIAction>
```

Would become:

```rust
pub fn advance(
    state: &mut AIState,
    world: &GameWorld,
    mfg_state: &ManufacturingState,
    _mission_state: &MissionState,
    movement: &crate::movement::MovementState,
    tick_events: &[TickEvent],
    config: &GameConfig,
    research_state: &ResearchState,
    ai_fog: Option<&mut AiFogState>,  // None = omniscient mode
) -> Vec<AIAction>
```

When `ai_fog` is `None` (or `config.ai.fog_mode == "omniscient"`), behavior is unchanged. When present, `evaluate_galaxy_state()` filters through the intel quality map.

### New State: `AiFogState`

Lives alongside `AIState` in the simulation state bundle. Persisted in save files (save format v5). Seeded at game start from the AI faction's initial fleet positions.

### Intel Quality Calculation

Per tick, for each system:

```
base_intel = max(sensor_coverage, espionage_intel, memory_residual)
sensor_coverage = max(0, 1.0 - distance_to_nearest_fleet / sensor_radius) for AI fleets
espionage_intel = if espionage_active_at_system { 0.8 } else { 0.0 }
memory_residual = last_direct_intel * decay_factor^(ticks_since_last_seen)
```

This naturally produces a gradient: high intel near AI fleets, medium intel in recently-scouted areas, low intel in distant or long-unseen regions.

### Modified `evaluate_galaxy_state()`

The core change is a filter on the system iteration:

```rust
for (key, sys) in world.systems.iter() {
    let intel = match ai_fog {
        Some(fog) => fog.intel_quality(key),
        None => 1.0, // omniscient
    };

    if intel < config.ai.min_intel_threshold {
        continue; // AI doesn't know about this system
    }

    // existing bucketing logic, but garrison strength is
    // multiplied by intel to reflect uncertainty
}
```

### Modified `score_attack_target()`

Add an intel confidence factor to the existing 4-factor scoring:

```
score = weakness * w_weakness
      + proximity * w_proximity
      + deconfliction * w_deconfliction
      + freshness * w_freshness
      + intel_confidence * w_intel  // NEW: prefer known targets
```

Where `intel_confidence` is the intel quality for the target system. This naturally makes the AI prefer to attack systems it has good intelligence about.

### New Config Fields

```rust
pub struct AiConfig {
    // ... existing fields ...

    /// Fog mode: "omniscient" (default, parity), "influence" (weighted), "honest" (future)
    pub fog_mode: String,

    /// Per-tick intel decay rate when no active source covers a system. **Augmentation.**
    pub intel_decay_rate: f32,

    /// Minimum intel quality to include a system in galaxy evaluation. **Augmentation.**
    pub min_intel_threshold: f32,

    /// Weight for intel confidence in attack scoring. **Augmentation.**
    pub weight_intel: f64,

    /// Sensor multiplier (coordinate units per detection point). **Parity** with FogSystem.
    pub sensor_multiplier: f32,
}
```

---

## Appendix: Sources

### Academic Papers
- Robertson, G. & Watson, I. "A Review of Real-Time Strategy Game AI." *AI Magazine*, AAAI, 2014. — Comprehensive survey of fog-of-war handling in RTS AI: influence maps, particle models, plan recognition.
- Hagelbäck, J. & Johansson, S. "Dealing with fog of war in a real-time strategy game environment." *IEEE Symposium on Computational Intelligence and Games*, 2008. — Foundational paper on potential fields for RTS AI under fog.
- Weber, B., Mateas, M. & Jhala, A. "A Particle Model for State Estimation in Real-Time Strategy Games." *AAAI Conference on AI and Interactive Digital Entertainment*, 2011. — Particle-based enemy position prediction with confidence decay.
- Kabanza, F. et al. "Opponent Behaviour Recognition for Real-Time Strategy Games." *Plan, Activity, and Intent Recognition*, 2010. — Plan recognition under incomplete information using decision trees.
- Synnaeve, G. & Bessière, P. "A Bayesian Model for Plan Recognition in RTS Games Applied to StarCraft." *AIIDE*, 2011. — Bayesian state estimation robust to 80% missing observations.

### Industry Sources
- Dill, K. "Spatial Reasoning for Strategic Decision Making." *Game AI Pro 2*, Chapter 31, 2015. — Kohan II influence map architecture, "illusion of intelligence" pattern, region-based reasoning.
- Winter, C. "Mastering Real-Time Strategy Games with Deep Reinforcement Learning: Mere Mortal Edition." Blog post, 2021. — Omniscient value function + honest policy architecture, last-known-position behavior.
- Wikipedia. "Artificial intelligence in video games: Cheating AI." — Survey of AI cheating patterns across game history.

### Community Analysis
- CivFanatics Forums. "Does the AI cheat with information?" Thread, 2017. — Player analysis of Civ VI information cheating.
- Paradox Interactive Forums. "The AI is CLEARLY cheating with Fog of War." Thread, 2019. — Community evidence of Stellaris AI vision cheating.
- Paradox Interactive Forums. "Does the AI cheat fog of war?" Thread (HOI4), 2018. — Developer confirmation of HOI4's shared fog simulation.
- Steam Community. "AI cheating?" (Stellaris), 2016. — Discussion of tactical AI vision omniscience.
- Reddit r/4Xgaming. "4X games where the AI is competitive without excessive cheating." Thread, 2024.

### Original Game RE
- Open Rebellion Ghidra notes: `ghidra/notes/ai-behavior-analysis.md` — FUN_00519d00 confirmed omniscient.
- Open Rebellion AI parity tracker: `agent_docs/systems/ai-parity-tracker.md` — "Map visibility: Omniscient (no fog check) — FAITHFUL."
