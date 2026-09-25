# P58-B19 mixed-task-force target rejection

P58-B19 restores the original refusal path used when a player tries to assign
one hostile focus target to capital ships selected from different task forces.
The command now leaves manual targets, active targets, and Escort targets
unchanged and queues the exact faction response: Alliance event `0x84`, WAVE
`14101`, or Imperial event `0x102`, WAVE `15105`.

This is production behavior inside the shared campaign and standalone tactical
renderer. It does not add a test-only production route or change the 106-cell
denominator. Lossless original captures and strict acceptance remain open.

## Source contract

`FUN_005a24d0` resolves the selected capital ships to one task-force ordinal
before assigning an enemy target. A mixed selection takes the rejection branch
and queues the faction event instead of mutating the order state. This corrects
the provisional `task_force_capacity` label in the tactical voice map to
`target_task_force_rejected`.

| Faction | Source event | Resource | Required result |
|---|---:|---:|---|
| Alliance | `0x84` | VOICEFXA WAVE `14101` | Reject and preserve every selected ship's orders |
| Empire | `0x102` | VOICEFXE WAVE `15105` | Reject and preserve every selected ship's orders |

## Implementation result

- Enemy focus assignment now resolves selected live player capitals through
  one source-backed helper.
- A mixed-task-force selection is rejected before manual target, active target,
  or Escort state changes.
- A same-task-force selection retains replace, append, and append-toggle
  behavior without emitting the rejection voice.
- The existing production group-presentation journey selects two projected
  capitals from different task forces, right-clicks a hostile capital, proves
  no focus-assignment or assigned-order event fires, and verifies the exact
  muted faction voice route. Unit tests prove all three order fields remain
  unchanged.

## Verification

| Gate | Result |
|---|---|
| Behavior-first proof | New Alliance rejection test failed before the constructor, event variant, and assignment helper existed |
| Complete Rust workspace | 731 passed, 0 failed, 32 ignored |
| Focused renderer tests | 265 passed, 0 failed, 1 ignored |
| Fixture-feature renderer tests | 266 passed, 0 failed, 2 ignored |
| Harness tests | 26 passed, 0 failed |
| Tactical lookup validation | 29 ship ordinals, 8 fighter ordinals, and 285 voices passed |
| Scoped Clippy | Passed with the documented pre-existing argument-count and constant-assertion warnings |
| Scoped mutation gate | 11 total: 8 caught, 2 unviable, 1 browser-bounded input survivor |
| Focused muted browser journey | 4 of 4 passed on the rebuilt combined fixture; both factions and both viewports |
| Browser startup and cleanup | 4 four-request starts, 4 muted launches, 4 closed browsers |
| Independent browser review | PASS; populated faction shells, two selected capitals, stable 4:3/letterbox presentation, no in-scope blocker |
| Strict 106-cell gate | Not run; A0 coverage and acceptance remain 0 of 106 |

The surviving mutation replaces the top-level GUI input handler with a no-op.
It is covered by the live pointer journey, which must observe both selection
logs and the exact rejection and audio logs. Unit-testing that wrapper would
duplicate the browser input contract rather than exercise the real event path.

The final combined-source browser run is
`2026-09-25T17-11-51-529Z-58971`. Raw screenshots, console logs, request
ledgers, and per-case results remain outside Git under the ignored artifact
tree. Durable hashes and the acceptance boundary are recorded in the
[artifact bundle](p58-b19-tactical-mixed-task-force-target/README.md).
