# P58-B20 recovery and withdrawal feedback

P58-B20 restores three production tactical voice paths: the RGBG acknowledgement
after the last live fighter group member docks, the RGBG warning when Recover
cannot reserve another friendly carrier slot, and the faction warning when a
withdrawal leaves a hyperdrive-disabled capital ship behind. All browser runs
remain muted.

This checkpoint deepens existing tactical journeys without changing the
106-cell denominator. It does not accept any strict cell: lossless A0 coverage
and acceptance remain 0 of 106.

## Source contract

`FUN_005b8630` removes a recovered fighter-group object and queues the matching
faction/group acknowledgement after no live object remains for that group.
`FUN_005a0240`, called from the withdrawal path in `FUN_00596ac0`, iterates the
player fleet, applies the readiness predicate at `FUN_005b1b70`, and queues the
faction hyperdrive warning when one ship cannot withdraw.

| Route | Alliance | Empire | Production rule |
|---|---|---|---|
| Fighter recovery complete | events `0x87`–`0x8a`, WAVE `14104`–`14107` | events `0x105`–`0x108`, WAVE `15108`–`15111` | Emit once after the last live member of the RGBG group recovers |
| Fighter recovery capacity | events `0x8b`–`0x8e`, WAVE `14108`–`14111` | events `0x109`–`0x10c`, WAVE `15112`–`15115` | Emit once per selected RGBG group when no friendly carrier slot remains |
| Withdrawal hyperdrive warning | event `0x95`, WAVE `14118` | event `0x10e`, WAVE `15117` | Emit after withdrawal confirmation when any live player ship cannot enter hyperspace |

The recovery-capacity event/resource identity and wording are verified against
the executable voice table and owned audio transcription. Its exact original
caller remains open, so this checkpoint claims the production no-slot behavior,
not caller-complete recovery semantics.

## Implementation result

- Fighter docking queues one exact completion cue only when no live member of
  that player RGBG group remains.
- A Recover command reserves available carrier slots in stable order and queues
  one exact capacity warning for each selected group left without a slot.
- Withdrawal confirmation retains the normal start cue and adds the exact
  hyperdrive warning when at least one live player capital has zero hyperdrive
  capacity or condition. Operational ships continue through the existing
  withdrawal path; disabled ships remain behind.
- Deterministic browser fixtures exercise two selected fighter groups against
  one carrier slot, a final docking callback, and a fleet containing one
  operational plus one disabled-hyperdrive capital.
- The browser harness asserts exact faction event, WAVE, routing, loaded, and
  muted fields rather than inferring audio success from asset presence.

## Verification

| Gate | Result |
|---|---|
| Complete Rust workspace | 753 passed, 0 failed, 34 ignored |
| Focused renderer tests | 264 passed, 0 failed, 3 ignored |
| Fixture-feature app test | 1 passed, 0 failed |
| Harness tests | 26 passed, 0 failed |
| Fixture build validation | 38 scenarios and 152 executions; production fixture-token exclusion passed |
| Deterministic tactical matrix | 106 of 106 A1 cells mapped; denominator unchanged |
| Focused muted browser journeys | 12 of 12 passed across three scenarios, both factions, and both viewports |
| Browser startup and cleanup | 12 four-request starts, 12 muted launches, 12 closed browsers |
| Exact browser voice routes | Completion, capacity, withdrawal start, and hyperdrive warning events loaded and routed for both factions |
| Independent browser review | PASS; populated faction cockpits, coherent controls and panels, stable 4:3 and letterboxed presentation, no visible blocker |
| Strict 106-cell gate | Not run; A0 coverage and acceptance remain 0 of 106 |

The final runs are `2026-09-26T07-34-05-938Z-6406`,
`2026-09-26T07-34-57-421Z-8272`, and
`2026-09-26T07-36-08-167Z-9901`. Raw screenshots, console logs, request
ledgers, and per-case results remain under the ignored artifact tree. Durable
hashes, the inspected contact sheet, and the acceptance boundary are recorded
in the [artifact bundle](p58-b20-tactical-recovery-withdrawal-feedback/README.md).
