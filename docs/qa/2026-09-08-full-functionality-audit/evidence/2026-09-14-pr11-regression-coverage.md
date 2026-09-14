# PR #11: regression coverage checkpoint

Step 5 reviews the committed tests at `c9a1ad4` on
`fix/alliance-cockpit-routing`. Steps 1–3 already supplied the required
coverage; no additional Rust changes or duplicate tests are needed.

| Scope | Existing coverage |
|---|---|
| Both factions | Primary control command IDs, rectangles and resource states; side globes map to Game Options; GID remains the rightmost bottom control. |
| Encyclopedia | Actual `0x131` control opens the viewer for either faction and dismisses GID; repeated activation preserves selection; unrelated controls leave state unchanged. |
| Game Options | Entry from either faction opens the save panel in the distinct Options mode and dismisses GID; return restores Galaxy mode; other navigation modes are unchanged. |
| Keyboard | Injected egui events and macroquad fallback polling both verify F1 → Game Options and F7 → Encyclopedia, with F2–F5 finder mappings and F6 unmapped. |

Tests are in `crates/rebellion-render/src/cockpit.rs` and
`crates/rebellion-app/src/cockpit_routing.rs`. The shared keyboard mapping
is faction-independent; faction-specific control and destination tests
cover both Alliance and Empire.

## Verification

The unchanged Rust source was tested during step 4 on 2026-09-14:

```sh
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/will/.cargo/bin make test
```

That run exited 0: 671 passed, zero failed, with 20 ignored in the workspace
pass (four owned-DAT tests are run separately and included in the total).
See [the step 4 evidence](2026-09-14-pr11-gid-preservation.md) and local log
`/tmp/pr11-step4-test.log`. Tests were not rerun for this documentation-only
checkpoint. JSON parsing, coverage-test name checks, and `git diff --check`
validate this change. There are no changed Rust files requiring rustfmt.

This completes the native regression-test checklist item only. Physical
keyboard and browser journeys remain step 7; no release acceptance or
original visual parity is claimed. Full Game Options repair remains separate.
