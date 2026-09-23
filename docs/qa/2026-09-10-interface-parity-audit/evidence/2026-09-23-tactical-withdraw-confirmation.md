# P58-B13 tactical withdrawal confirmation

P58-B13 restores the original tactical withdrawal confirmation between the
Battle Options command and fleet retreat. The implementation is derived from
the owned executable and its extracted TACTICAL and TEXTTACT resources rather
than a replacement dialog.

## Source contract

| Source | Recovered behavior |
|---|---|
| `FUN_005f0c20` / TACTICAL `1310` | 149×236 confirmation panel in the existing right-hand tactical housing |
| `FUN_005f1050` / TEXTTACT `56772` | `Withdraw Confirmation` at panel-local `(20, 28)` |
| `FUN_005f1050` / TEXTTACT `56771` | Two-line withdrawal prompt at panel-local `(14, 92)` |
| `FUN_005f0f40` | Confirm at `(66, 202)` and cancel at `(106, 202)`, both 27×25 |
| TACTICAL `1113`–`1116` | Confirm/cancel normal and captured-press bitmaps |
| `FUN_005f1170` | Confirm dispatches withdrawal; cancel closes without dispatch |

The dialog replaces the Battle Options panel in the same `(481, 27)` housing.
Pointer capture dispatches only when release occurs over the captured opaque
bitmap region. Exact outer edges and transparent pixels do not activate a
control.

## Result

| Measure | Result |
|---|---:|
| Canonical tactical cells | 106 |
| Cells with a deterministic A1 scenario | 101 |
| Journey-mapped cells | 82 |
| Snapshot-mapped cells | 19 |
| Cells needing a new scenario | 5 |
| A0 captures | 0 |
| Strictly accepted cells | 0 |

The focused browser journey opens the dialog, proves its source pixels and both
normal controls, cancels once, reopens it, proves both captured-press states,
confirms withdrawal once, and rejects a second withdrawal after retreat begins.
Both faction cases use four-request startup, mute audio, report no browser
errors, and close their browser processes.

## Verification

| Gate | Result |
|---|---|
| Renderer tests | 252 passed, 1 ignored |
| Harness unit tests | 26 passed |
| Tactical catalog and matrix checks | Passed; 101 mapped, 5 missing |
| Withdrawal browser journey | 2 of 2 passed |
| Startup request count | Four in both cases |
| Muted launches and browser cleanup | 2 of 2 |
| Independent visual review | Pass; no P0–P3 visual finding |
| A0 acceptance | 0 of 106 |

Raw screenshots, network records, console logs, and per-case JSON remain in
the ignored `.artifacts/interface-parity/` tree. Durable hashes and summaries
are indexed in the
[artifact bundle](p58-b13-tactical-withdraw-confirmation/README.md).
