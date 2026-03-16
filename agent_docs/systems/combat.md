# Combat System

`combat.rs` + `bombardment.rs` — Space and ground combat resolution.

## Trigger

Opposing fleets coexist at a system (checked each tick in `main.rs`). Per-system 5-tick cooldown prevents infinite re-trigger on draws.

## Types

| Type | Purpose |
|------|---------|
| `CombatSystem` | Stateless resolver — `resolve_space()` and `resolve_ground()` |
| `SpaceCombatResult` | `{ winner, attacker_fleet, defender_fleet, ship_damage, fighter_losses }` |
| `GroundCombatResult` | `{ winner, system, troop_damage }` |
| `ShipDamageEvent` | `{ fleet, ship_index, hull_after }` — hull=0 means destroyed |
| `FighterLossEvent` | `{ fleet, fighter_index, squads_before, squads_after }` |
| `TroopDamageEvent` | `{ troop, strength_after }` — strength ≤ 0 means destroyed |
| `CombatPhaseFlags` | Custom bitfield (NOT `bitflags!`) — uses `.set()` not `.insert()` |
| `CombatSide` | Attacker / Defender / Draw |
| `BombardmentSystem` | `resolve_bombardment(world, fleet, system, difficulty, tick)` |

## API

```rust
let result = CombatSystem::resolve_space(&world, atk, def, sys, difficulty, &rolls, tick);
let ground = CombatSystem::resolve_ground(&world, sys, alliance_attacks, &rolls, tick);
let brd = BombardmentSystem::resolve_bombardment(&world, fleet, sys, difficulty, tick);
```

## Space Combat: 7-Phase Pipeline

From Ghidra RE of REBEXE.EXE:

1. **Weapon fire** — turbolaser/ion/laser per arc, damage rolls
2. **Shield absorption** — shield_strength absorbs damage
3. **Hull damage** — remaining damage reduces hull_current
4. **Fighter engagement** — squadron vs squadron, torpedo bombing
5. **Result** — winner determined by surviving fleet strength

Key gotchas from RE:
- Phase gate: `ACTIVE && !PHASES_ENABLED` (inverted from initial expectation)
- Alt-shield path: exact `== 0x71` family byte (not range `0x71..=0x72`)
- Fighter losses must be computed from initial vs surviving counts

## Bombardment

Formula from `FUN_005617b0`: `damage = sqrt((atk-def)²) / GNPRTB[0x1400]` with difficulty modifier, minimum 1 damage (only after division, not on zero raw_power).

## Effect Application (main.rs)

- `apply_space_combat_result`: Maps destroyed hulls (hull_after == 0) to fleet `capital_ships` count decrements. Removes empty fleets from system fleet lists.
- `apply_ground_combat_result`: Updates `TroopUnit.regiment_strength`. Removes destroyed troops from system and arena.

## Source

- `ghidra/notes/space-combat.md` — full 7-phase pipeline
- `ghidra/notes/ground-combat.md` — troop iteration
- `ghidra/notes/bombardment.md` — Euclidean distance formula
- `ghidra/notes/combat-formulas.md` — GNPRTB parameter mapping
