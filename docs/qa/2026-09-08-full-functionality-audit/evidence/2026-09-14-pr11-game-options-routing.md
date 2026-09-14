# PR #11: Game Options destination checkpoint

The user narrowed step 2 on 2026-09-14 to opening the Game Options screen;
fixing that screen is a separate task. This checkpoint does not complete the
maintainer's original request for every original options control.

Both command-center side globes now dispatch `GameOptions` for `0x133`.
Alliance resources remain `10013/10014`, rectangle `(3,355,27,41)`; Empire
resources remain `10027/10028`, rectangle `(79,192,35,57)`. GID remains on
`0x132`; no keyboard mapping is changed in this step.

The app enters a distinct `GameMode::GameOptions`, dismisses GID, refreshes
save metadata, and displays the existing slot-picker and audio UI with a Game
Options heading and Save/Load selectors. Return to Game, slot-picker Cancel,
and Escape return to the existing campaign. Main-menu Load Game retains its
separate return path. Options mode does not advance galaxy ticks. No new
save format, tactical renderer, or audio behavior is introduced.

## Validation

Commands ran serially with system tools first:

```sh
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/will/.cargo/bin make test
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/will/.cargo/bin make fmt-check
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/will/.cargo/bin make clippy
```

- Routing red gate: exit 2; both globe tests failed with `SaveLoad` instead
  of `GameOptions`.
- Navigation red gate: exit 2; the options entry test remained in Galaxy mode.
- Final test gate: exit 0; 665 workspace tests plus four owned-DAT integration
  tests passed (669 total), zero failed; workspace pass reported 20 ignored.
- Formatting: four changed Rust files formatted with `rustfmt --edition 2021
  --config skip_children=true`; `make fmt-check` passed, exit 0.
- Strict Clippy: exit 2, with the same 12 pre-existing renderer diagnostics as
  [step 1](2026-09-14-pr11-encyclopedia-routing.md). The renderer failure still
  prevents establishing a clean downstream app lint result.

Local logs: `/tmp/pr11-step2-{red,app-red,test,fmt,clippy}.log`.
Browser/WASM and original visual acceptance were not run for this checkpoint.
No strict interface cell is accepted, and P31 remains pending.

## Separate task: complete the original Game Options screen

Recover and implement the original layout and full saved-game control behavior,
sound controls, all five tactical display options (Starfield, Planet,
Pyrotechnics, High Detail Models, Holocube), and restart/exit behavior. Establish
restart semantics from original evidence before choosing new-campaign versus
reinitialization behavior. The current tactical production renderer has no
planet, holocube, or production 3D detail selection; those dependencies need
explicitly scoped work. This checkpoint adds no nonfunctional switches or
unavailable-setting placeholders.

Reference: retained `steam-guide/guide-015.png` in the interface audit shows
the original options composition and five tactical switches. It is discovery
reference, not an accepted lossless baseline. The complete original/native/
packaged-browser state and interaction matrix remains required for acceptance.
