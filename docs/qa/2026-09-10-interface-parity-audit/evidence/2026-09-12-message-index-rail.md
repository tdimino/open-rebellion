# Message Index rail resting-art checkpoint

Status: source-mapped visual checkpoint, not `CMD-08` acceptance.

The original `REBEXE.EXE` constructor `FUN_00427270` creates nine Message
Index category controls on each command-center side rail. Their positions,
command IDs, and bitmap pairs are now represented directly in
`crates/rebellion-render/src/cockpit.rs`. This restores the icons that were
previously black slots in the Imperial cockpit and absent from the Alliance
rail. The controls use STRATEGY resources rather than replacement drawings.

| Faction | Rectangles in the 640x480 canvas | Resting BMPs | Illuminated BMPs | Commands |
|---|---|---|---|---|
| Alliance | `x=3`, `y=109+25n`, `27x22` | `10050..10058` | `10060..10068` | `0x136..0x13e` |
| Empire | `x=611`, `y=110+25n`, `27x22` | `10030..10038` | `10040..10048` | `0x136..0x13e` |

At startup, the original constructor explicitly assigns the darker resting
resource to each control. The local browser harness now decodes the original
8-bit BMPs and compares every pixel against the fresh muted 640x480 capture.
Both faction rails matched their nine source bitmaps exactly, 5,346 pixels per
rail. The four-case smoke also passed stable frames, four-request startup,
clean browser diagnostics, and closed all four isolated browser processes.
The 1280x800 geometry is covered by a source-rectangle scaling test; it is not
claimed as pixel-identical to the 640x480 source.

The full workspace test, formatting, strict Clippy, and native-build gate
passed with the clean-repository Clippy configuration. This worktree also has
an ignored local `clippy.toml` that disallows historical `fieldN` bindings in
`dat-dumper`; running `make all` without setting `CLIPPY_CONF_DIR` to a clean
directory fails on those pre-existing bindings. No lint rule or extractor
source was changed to conceal that local-only conflict.
An Astra medium read-only review independently inspected the faction captures,
source mappings, pixel checks, muted launch and request logs, and audit claims.
Its sole documentation-schema finding was corrected; the follow-up review
found no remaining P0 through P2 issue and approved this resting-art checkpoint.
The packaged production WASM SHA-256 is
`c8702f756ff5aa3bce2f2ca5aad2914962b4cb5a695f3a73100d63319568cdb1`.
The unchanged runtime pack SHA-256 is
`1ce8e2370d037423ad20683ff747245548b3eca453305596a45ef3083899a862`.

The original `FUN_00422ce0` dispatches commands `0x136..0x13e` through
`FUN_0042a240`, using category values `0x7a..0x82` with the last two in
reverse command order. Its exported pseudocode is empty, so this checkpoint
does not assign unproven labels or make the controls open an invented panel.
The illuminated/unread predicate, pointer and disabled states, exact Message
Index window, category filtering, report navigation, clear/delete behavior,
and original-executable screenshot matrix remain open in `RE-MSG-01` and
`CMD-08`. All 16 strict `CMD-08` cells remain pending.
