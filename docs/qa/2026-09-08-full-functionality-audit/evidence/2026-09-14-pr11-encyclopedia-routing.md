# PR #11: Encyclopedia command routing checkpoint

The first review item is implemented on `fix/alliance-cockpit-routing`.
The maintainer's [review](https://github.com/tdimino/open-rebellion/pull/11#issuecomment-5659077378)
identifies command `0x131` as Encyclopedia through `FUN_00429f30` and
`FUN_0045d400`. This corrects the earlier Game Options label for that command.
The source attribution is the maintainer's trace, not a new independent Ghidra run.

Both faction control tables now dispatch `CockpitButton::Encyclopedia`.
Alliance normal/pressed resources remain `10010/10009`; Imperial resources
remain `10024/10023`. Their rectangles and input masks are unchanged.
The app opens its existing Encyclopedia state, closes the GID menu and submenu,
and preserves the selected tab and entity on repeated activation. Its runtime
log identifies `command=0x131 destination=encyclopedia status=opened`.

## Verification on 2026-09-14

Commands ran serially from the repository root with this environment prefix:

```sh
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/will/.cargo/bin make test
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/will/.cargo/bin make fmt-check
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/will/.cargo/bin make clippy
```

- Initial documented PATH attempt: Make exit 2, Cargo absent at the other
  user's path. The prefix above uses this machine's installation, with system
  tools first.
- Routing red gate: Make exit 2; both expected routing assertions failed because
  `0x131` produced `GameOptions`.
- Application red gate: Make exit 2; both open/reopen tests failed because the
  viewer remained closed. The unrelated-controls test passed.
- Final `make test`: exit 0; 663 workspace tests plus four owned-DAT integration
  tests passed (667 total), zero failed, 20 ignored in the workspace pass.
- `make fmt-check`: exit 0.
- `make clippy`: exit 2; 12 existing renderer diagnostics, comprising missing
  `must_use`, `chunks_exact_to_as_chunks`, `copy_iterator`, excessive booleans,
  items after statements, unfulfilled expectations, and float comparisons.
  `tactical_view.rs` is byte-identical to HEAD. The diagnosed `bmp_cache.rs`
  code at lines 1308 and 1574 is unchanged; only resource names/comments changed
  elsewhere in that file. Strict workspace Clippy is not green, and renderer
  failure prevents establishing a clean downstream app lint result.

Local command logs: `/tmp/pr11-step1-{red,app-red,test,fmt,clippy}.log`.

## Acceptance boundary

This is a native-tested routing checkpoint, not original Encyclopedia visual
acceptance. The existing viewer has incomplete original layout and browser
artwork. P35 remains `fail`; no strict interface cell is accepted. No browser or
WASM acceptance was run for this isolated step. Unified Game Options, F1/F7,
the GID harness, complete muted browser evidence, and the broader historical
ledger correction remain later PR #11 checklist items. The user subsequently requested a commit of this isolated step after formatting
the four changed Rust files. Targeted `rustfmt --edition 2021 --config
skip_children=true` and `git diff --check` passed. Push and PR editing remain
pending.
