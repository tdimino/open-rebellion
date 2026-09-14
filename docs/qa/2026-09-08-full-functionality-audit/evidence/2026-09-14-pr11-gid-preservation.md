# PR #11: GID mapping preservation

Step 4 verifies the existing mappings at commit `a49be0d` on
`fix/alliance-cockpit-routing`. No Rust or browser-harness code changed.

| Faction | Command | Normal / pressed | Logical rectangle (x, y, width, height) |
|---|---|---|---|
| Alliance | `0x132` | `10012 / 10011` | `(446,406,27,16)` |
| Empire | `0x132` | `10026 / 10025` | `(519,434,37,25)` |

Both controls produce `CockpitButton::GalacticInformationDisplay`.
`strategic_gid_control` returns index 5 of each six-control bottom row;
these are its rightmost controls. Each row has exactly one GID control.
The separate side globes remain `GameOptions` (`0x133`). The app's GID
handler toggles the menu, clears its submenu selection, and gets the logged
command ID from `strategic_gid_control` for the active faction.

## Verification on 2026-09-14

```sh
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/will/.cargo/bin make test
```

Exit 0: 667 workspace tests plus four owned-DAT integration tests passed
(671 total), zero failed; the workspace pass reported 20 ignored.
The existing tests for both primary rows, both separate globes, and the
exact GID control records all passed. Those tests cover command identity,
resource states, rectangles, row position, and uniqueness. No redundant
regression tests were added.

Local command log: `/tmp/pr11-step4-test.log`.

This verifies preservation of the native mapping contract. The stale browser
harness coordinates remain step 6, the complete muted browser gate remains
step 7, and the broad historical-ledger correction remains step 8. No original
visual or browser acceptance is claimed, and no strict interface cell is closed.
Formatting and Clippy were not rerun for this documentation-only checkpoint;
step 3 retains the current passing format gate and 12 existing Clippy diagnostics.
