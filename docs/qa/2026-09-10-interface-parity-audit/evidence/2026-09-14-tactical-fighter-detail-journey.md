# P58D tactical fighter-detail journey

P58D proves that live production fighter groups switch among their original
type-303 indicator, far, and close resources through the authentic bitmap zoom
controls. The proof uses one retained renderer and keeps both loaded fighter
families resident throughout the journey.

## Implemented contract

- The source camera begins centered on the player's production fighter only in
  the test fixture. Production play does not receive this shortcut.
- Each zoom click changes the same production camera field used by the battle
  renderer.
- Each fighter independently selects close below view span `5`, far from `5`
  through less than `10`, and its two-pixel indicator at `10` or above.
- Alliance A-wing resources are `4024`, `4029`, and `4204`. Imperial TIE
  Fighter resources are `4104`, `4109`, and `4204`.
- Scene diagnostics emit only when a fighter crosses a detail boundary. The
  nine-state sequence preserves the real mixed states caused by the two source
  depth lanes.

The observed detail sequence in both faction journeys is:

`indicator/indicator`, `far/indicator`, `far/far`, `close/far`,
`close/close`, `close/far`, `far/far`, `far/indicator`,
`indicator/indicator`.

## Verification

| Gate | Result |
|---|---|
| Focused renderer and fixture tests | 12 passed, 1 owned-data test ignored |
| Workspace tests | 671 passed, 0 failed, 21 ignored |
| Tactical browser matrix | 40 of 40 passed |
| Browser isolation | 40 fresh muted launches, 40 four-request starts, 40 closed |
| Runtime diagnostics | Stable screenshots, no page, console, request, or missing-asset errors |
| Astra medium | Qualified A1 acceptance after inspecting all 24 PNGs, four bundled results, the summary, and 40 referenced raw records |
| Production WASM SHA-256 | `df6b489048e6b9c59d46031e5905c065627a1a91e1b428d115a57586f00bc562` |
| Fixture WASM SHA-256 | `87be63a58afb488eb4db2f36862864dec18814bd0e254d5914a647cc4552ab05` |
| Runtime pack SHA-256 | `7f0289265d9f85bfc971b8160edcab0da2425f1ed7246c35e570b6a07fdde631` |

The [artifact bundle](p58d-tactical-fighter-detail-journey/) retains initial,
far, close, return-far, and return-indicator screenshots plus the complete
result records for both factions and both viewports. The close screenshots
visibly retain the centered source fighter sprite.

The [Astra acceptance record](p58d-tactical-fighter-detail-journey/astra-browser-acceptance.json)
confirms both faction journeys and both viewports without accepting an
unbaselined tactical cell.

## Acceptance boundary

This is qualified A1 implementation evidence. It does not supply a lossless
owned-original A0 baseline, prove the original automatic task-force or fighter
group arrangement, or accept any of the 106 strict tactical cells. Task-force
and fighter-group controls, damage, planets, effects, Death Star states,
results, audio, and strategic return remain open.
