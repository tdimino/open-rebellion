# Tactical capital-ship weapon loop

This note records the capital-ship weapon contract recovered from the owned
English `REBEXE.EXE` with SHA-256
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.
It covers battery construction, directional classification, target scoring,
fire order, energy consumption, and weapon and shield recharge.

## Recovered functions

| Function | Role |
|---|---|
| `FUN_005b05c0` | Capital tactical-object constructor and DAT-field transport |
| `FUN_005b0e50` | Initializes one 0x74-byte weapon-arc record |
| `FUN_005b18d0` | Returns the maximum installed weapon range |
| `FUN_005b1770` | Scales a value by current hull divided by maximum hull |
| `FUN_005b3a40` | Autonomous per-family target collection and scoring |
| `FUN_005b3f10` | Strongest-arc selection, event emission, energy use, and recharge queuing |
| `FUN_005b6530` | Direct-target range, arc, and candidate calculation |
| `FUN_005b64e0` | Adds one unique arc to the four-entry recharge queue |
| `FUN_005b6320` | Shield recharge and FIFO weapon-energy distribution |
| `FUN_005b0400` | Hull and subsystem-adjusted weapon recharge |
| `FUN_005b0460` | Hull and subsystem-adjusted shield recharge |
| `FUN_005b6980` | Disables and empties one weapon arc |
| `FUN_005d2360` | Initializes the shared weapon-strength record |
| `FUN_005d2490` | Turbolaser target modifier |
| `FUN_005d24e0` | Laser cannon target modifier |
| `FUN_005d2530` | Ion cannon target modifier |

## Arc memory and order

Each arc occupies 0x74 bytes. The first begins at object offset `+0x17c`.
The constructor stores the four records in this order:

1. fore at `+0x17c`;
2. starboard at `+0x1f0`;
3. aft at `+0x264`;
4. port at `+0x2d8`.

Within each record:

| Arc offset | Meaning |
|---|---|
| `+0x00` | laser cannon count |
| `+0x04` | ion cannon count |
| `+0x08` | turbolaser count |
| `+0x0c` | current laser candidate strength |
| `+0x10` | current turbolaser candidate strength |
| `+0x14` | current ion candidate strength |
| `+0x64` | ready flag |
| `+0x68` | current energy reserve |
| `+0x6c` | full energy reserve |

The full reserve is the sum of all three weapon counts. Firing subtracts the
count of each family that emitted an event. A fired arc becomes unavailable and
is queued once until its reserve is full again.

## Direction and range

`FUN_005b6530` rotates the source forward vector by plus and minus 45 degrees
and tests the target delta in the XZ plane. Its result codes are fore 0,
starboard 1, aft 2, and port 3. Equality at either forward boundary belongs to
the fore arc. Equality at either rear boundary remains in the lateral arc.

Distance is three-dimensional. The range getters map to the CAPSHPSD values as
follows:

| Vtable getter | Object field | DAT value |
|---|---|---|
| `+0x50` | `+0x394` | ion cannon range |
| `+0x54` | `+0x398` | laser cannon range |
| `+0x58` | `+0x3a0` | turbolaser range |

## Candidate strength and fire order

The shared family coefficient is the exact float immediate `0x3e888889`.
Against capital ships, ion cannons and turbolasers retain a 1.0 target
modifier. Laser cannons use the exact immediate `0x3e2b020c`. Autonomous
target collection also multiplies by current hull divided by maximum hull.
The direct-target callback does not apply that hull multiplier.

Autonomous collection keeps the first target on an equal score. The firing
routine sums all three candidate strengths per arc, selects the first strict
maximum, and emits every populated family in that arc. Its event order is ion,
laser, then turbolaser. It repeats until no arc has a positive candidate.

Recovered event IDs from `FUN_005a7500` are:

| Event | Meaning |
|---|---|
| `0x0d` | fire laser cannon |
| `0x0e` | fire turbolaser |
| `0x0f` | fire ion cannon |
| `0x10` | fire torpedo |
| `0x11` | take laser hit |
| `0x12` | take ion hit |
| `0x13` | take turbolaser hit |
| `0x14` | take torpedo hit |

## Recharge

The constructor stores `weapon_recharge_rate * 3.75`. Effective weapon
recharge is:

```text
max(0, hull_ratio * base_weapon_recharge
       - base_weapon_recharge * 0.25 * weapon_subsystem_hits)
```

Shield recharge uses the same formula with the shield rate and shield damage
counter. `FUN_005b6320` applies the elapsed interval, then distributes weapon
energy through the queued arcs in FIFO order. Surplus energy proceeds to the
next arc. A full arc becomes ready and leaves the queue.

## Implementation boundary

`crates/rebellion-render/src/tactical_view.rs` now implements this capital
battery and recharge contract. The source transports floating-point damage
events, while the current Rust hull and shield stores remain integral. The
implementation preserves the recovered float strength through projectile
selection and rounds only at that existing storage boundary. Joining the
original global RNG sequence and converting combat energy stores to floats
remain separate parity work.
