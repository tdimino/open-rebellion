# PR #11: command-center keyboard routing checkpoint

Step 3 maps F1 to Game Options (`0x133`) and F7 to Encyclopedia (`0x131`)
in both the egui event path and macroquad fallback. The shared handler serves
both faction cockpits and dispatches the destinations implemented in steps 1
and 2. F2–F5 retain their finder destinations; F6 remains unbound.

The production keyboard routine delegates to a small input-boundary helper.
Tests feed actual egui key events or a macroquad key-state predicate into that
same routine. The fallback test checks the polled key list as well as the
mapping, so adding F1 to a mapper without polling it is caught. No GPU context
or new dependency is needed. Existing direct accelerator and bitmap tests
remain in place.

## Verification on 2026-09-14

Commands ran serially with system tools first:

```sh
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/will/.cargo/bin make test
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/will/.cargo/bin make fmt-check
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/will/.cargo/bin make clippy
```

- Red gate: `make test` exited 2; the egui, fallback, and direct accelerator
  assertions all failed because F1 returned no destination.
- Green gate: `make test` exited 0; 667 workspace tests plus four owned-DAT
  integration tests passed (671 total), zero failed; workspace pass reported
  20 ignored.
- Targeted `rustfmt --edition 2021 --config skip_children=true` completed;
  `make fmt-check` exited 0.
- `make clippy` exited 2 with the same 12 existing renderer diagnostics as
  [step 2](2026-09-14-pr11-game-options-routing.md). Strict workspace Clippy
  remains failing, and downstream app lint cleanliness is not established.

Local logs: `/tmp/pr11-step3-{red,test,fmt,clippy}.log`.

## Boundary

This is native automated keyboard-routing evidence. Physical keyboard behavior
in native and packaged browser runtimes remains part of the later acceptance
gate. No strict original-interface cell is promoted. The user-deferred Game
Options screen repair, GID harness update, broader historical ledger correction,
full browser evidence, and final PR update remain separate work.
