# P58F8 tactical maneuver and movement

P58F8 connects the recovered maneuver-state producer to the live tactical
mobility path and applies the original velocity and millisecond position
integration contracts. Production ships retain the original stationary
constructor state until a desired direction is supplied.

## Implemented contract

| Surface | Mapping |
|---|---|
| Default direction state | Current direction `(0,0,+1)` and desired direction `(0,0,0)` |
| Maneuver bonus | Missing state record returns 1; active state returns `clamp(9 - value, 1, 9)` |
| Effective engine | Base plus maneuver bonus, minus 25% per engine hit and active tractor power, clamped at zero |
| Movement alignment | Nonnegative dot product of normalized current and desired directions |
| Velocity | Current forward vector multiplied by effective engine power and alignment |
| Position integration | `position += velocity * delta_milliseconds * 0.001` |
| Combat step | 250 milliseconds, matching four authoritative steps per second |
| Browser proof state | State value 4 gives bonus 5; one engine hit gives power 78.75; aligned velocity moves the selected ship 19.6875 source units in one step |

The exact executable paths and recovered constructor boundary are recorded in
the [Ghidra note](../../../../ghidra/notes/tactical-maneuver-movement.md). The
deterministic fixture supplies a desired direction and advances one production
movement step. It does not claim that interactive maneuver commands, turn
rates, collision handling, or formation routing are complete.

## Verification

| Gate | Result |
|---|---|
| Focused tactical render tests | 27 passed, 0 failed |
| Focused fixture test | 1 passed, 0 failed |
| Workspace tests | 701 passed, 0 failed, 21 ignored |
| Harness unit and static checks | 11 passed; script syntax, catalog, and production-exclusion checks pass |
| Focused maneuver/movement browser matrix | 4 of 4 passed |
| Complete tactical browser matrix | 72 of 72 passed |
| Browser isolation | 72 fresh muted launches, 72 four-request starts, 72 closed |
| Source-state proof | State 4, bonus 5, one engine hit, effective power 78.75, alignment 1 |
| Alliance movement | Z `-56 -> -36.3125` with velocity `+78.75` over 250 ms |
| Imperial movement | Z `56 -> 36.3125` with velocity `-78.75` over 250 ms |
| Render-state proof | Integrated selected position equals the rendered participant position in every focused case |
| Runtime diagnostics | Stable screenshots and no page, console, request, or missing-asset errors |
| Astra medium visual acceptance | Pass across both factions and both viewports; no visible acceptance defects |
| Production WASM SHA-256 | `2116bab3480909bd77ae4c664fb75f962a8754600b3b96a1f828edcd5aa36512` |
| Fixture WASM SHA-256 | `6f695e965fcbda34046e4470ecf1442f92865f89b238b313db5ab61c7c57159d` |
| Runtime pack SHA-256 | `7f0289265d9f85bfc971b8160edcab0da2425f1ed7246c35e570b6a07fdde631` |

The [artifact bundle](p58f8-tactical-maneuver-movement/) retains both factions
at 640 by 480 and 1280 by 800 letterboxed viewports, result records, focused
and complete summaries, a contact sheet, and the Astra verdict.

## Acceptance boundary

This is qualified A1 implementation evidence. Maneuver-state bonus production,
effective-power velocity, signed faction movement, 250-millisecond integration,
production render-state mutation, and browser presentation are proven. Exact
global RNG equivalence, interactive command delivery, current-to-desired turn
rates, collision and formation behavior, lossless A0 comparison, and all 106
strict tactical cells remain open.
