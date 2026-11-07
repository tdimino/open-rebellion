# Strategic Windowing Toolkit: r29 Reconciliation and PR #46 Review Repair

Accepted-candidate date: 2026-10-25
Accepted-candidate source: a912b6a95e44ee1b6a00861b68bdf74ce9eb197a
Review-repair date: 2026-11-05
Review-repair source: 971f23cb171785e2fe21f17335b178ef09ebd3ba
Final review-candidate verification date: 2026-11-07
Final review-candidate code and fixture source: 1d7ea50f6a8dc84d76946f4c4124a53aebd812c4

This reconciliation joins three distinct accepted evidence lanes against the
same immutable r29 candidate: SW-12B fixture-browser, SW-12P
production-package/browser, and SW-13A Alliance plus SW-13E Empire Linux-native
evidence. No source or candidate changes occurred between their runs. The
sealed source revision and artifacts agree:

| Evidence object | Stable identity |
|---|---|
| Candidate source | a912b6a95e44ee1b6a00861b68bdf74ce9eb197a |
| Candidate manifest | SHA-256 28ae5cc5e9b798d02e30a3bb6b0785bcb40781fd0e0e27d8f185808eb09b8221 |
| Candidate evidence ledger | 89 entries; SHA-256 14d5215f61e476e9f1a10fb7ccae93a726dd88b6c8479120326fea7060ce6b66 |
| Fixture WASM | 6b543f68c27e58ddabd5ad40637ac7909be7f28b8e9117af3de44e3e46daacaa |
| Production WASM | 06b97750812ff1f8429ca4ee34a1d5b730a3dc9f802823d10d158ce638e46a77 |
| Runtime pack | 4ad37135a91b930db616ff5fd7c9403554939efd0461faebc4df105e260c9ea2 |
| Frozen native executable, both faction copies | 4a49ddef6db2d48efbe035b3295e36e544aa67db8ef37393bd4c833d5ba95edb |

## Accepted backend evidence

| Bead / lane | Accepted result and stable values | Recursive evidence seal |
|---|---|---|
| SW-12B .22, fixture-browser | Ten accepted fixture-browser gates with both-faction coverage; report SHA-256 7d0ac290fe4b4c3ecdc891be92831ef20ec3267134022720d574a1ff32f74cc5; case manifest SHA-256 66780335b320d6de27ea1c2cb3ade6463b369b8f84c56abe0993a6d2af53adba | 1,114 entries, SHA-256 bf3870e91a0431c4b87e789dc4fcae74e4d37d16517156ae4b817c2929d757dc |
| SW-12P .23, production-package/browser | Both factions passed all 11 production journey checks; report SHA-256 c25bd606caba8bc679ce0d934b297e4dc1e6f5194421d1bde778756e4cfe0226; case manifest SHA-256 6c84440b7dfbf16bd83eaebeb2b7a12a414a95547fcde9a90a2fc49b08b39cdd | 44 entries, SHA-256 2169a89bb44be30d990de2485e9c0c7dbee2145251a472fce4cce8aa422f50a4 |
| SW-13A .24, Alliance Linux-native | Seed 1337; day 0 focus-loss observation at WindowId(4); tactical day 1 returned through Alliance Victory Results to Galaxy day 1; report SHA-256 043dabc49821343b21dea0ab5818160fc70c2636ffee67a6db32f529ead1414e; tactical continuation SHA-256 315be575c8ad1c07bffc9bbaf73c1c294fac3842be79f9bee872755d89653260 | Original 70-entry ledger SHA-256 12cfdb43a9b1450b0143a941c9dc5f67445a7f1bb73baaabd952631f404affc1; interaction continuation 107-entry ledger SHA-256 4f0084076699b897bd395e5c3d802cebdfd48d270081d8136a1368411f3daa26; tactical continuation 135-entry ledger SHA-256 9e36d59b1fda641f04bb77caaa5419f54f6b2d875044ff63dbbce3b127e5e1a0 |
| SW-13E .25, Empire Linux-native | Seed 1337; day 0 focus-loss observation at WindowId(2); tactical return passed through Empire Victory Results to Galaxy day 3; outside-menu press retained day 0 Paused; report SHA-256 50df969657f9292ed2eb6c75f3994cde77675acfe692508e535e13c758e0d9d7; menu continuation SHA-256 b3d853d8fe52283803dca50e3940a53e9a654430da0098c193c5d888f0142998 | Main 94-entry ledger SHA-256 f79e8cf1101dae0f809ccd8071f812bf7f4afad8bb425a35253b7abf63b07ca8; menu 12-entry ledger SHA-256 cd2ab290efa4edc53ebf25fbedccb44a99c54add257a64e7a8583cd88dc1ae47 |

The coordinator independently accepted each backend after checking its report,
case/result manifest, ledger, and applicable served or executable identity.
All cited ledgers were rechecked from the repository root while preparing this
reconciliation. The earlier .25 row-click observations remain classified as
unverified, not as a product failure; the coordinator clarified that selecting
the Status row is not part of the required native outside-press criterion. The
focused menu continuation instead proves dismissal on the outside press,
suppression of the next-frame orphan release, and response to a later fresh
press.

## Architecture and proven contracts

The r29 implementation separates four responsibilities:

1. rebellion-windowing is a renderer-independent, standard-library-only
   authority for semantic window identities, lifecycle and order, focus,
   bounded history, modality, capture, and event-local keyboard routing
   projections derived from immutable frame state.
2. rebellion-render translates runtime system/sector identities to semantic
   subjects and declared policies. Its adapter paints in snapshot order,
   realizes exact manager z-order through stable egui IDs, records frame-local
   geometry and paint receipts, and admits normalized physical events through
   a recipient-aware router. A physical press has one owner across frames;
   cancellation quarantines a later orphan release.
3. rebellion-app::StrategicWindowing owns the manager, scene membership,
   prepared content, queued commands, and the post-dispatch commit boundary.
   Galaxy, Tactical, and Suspended membership affects command eligibility and
   snapshot routing together. Feature content is attached/released only through
   matching identity receipts; content readiness/absence/suspension is distinct
   from whether a surface painted in a particular frame.
4. Fixture, production-package, and native reports remain separate evidence
   products tied to the same frozen candidate. Their pass does not itself
   promote a manual-facing checklist row or a strict audit cell.

The live adapter, fixture-browser, production-package/browser, and Linux-native
behaviors were checked at candidate r29.
This is evidence for window identity, ordering, event ownership, focus/capture
cancellation, scene boundaries, content receipts, and the declared journeys;
it is not a claim that every source-level API described in historical design
text exists or that every original window surface is complete.

## PR #46 review repair evidence

The review-repair sequence began at
`971f23cb171785e2fe21f17335b178ef09ebd3ba` in response to the maintainer
review of head `44704a7e139389379354fb442cde7b6da0c89c80`. Its retained original-game
focus anchors are named by save, day, and visible result:

- `empire-fleet-to-yaga-minor`, days 38--45: load restores Sesswenna below
  Coruscant. Without a GID press, Escape closes Coruscant and the next Escape
  is inert. With the GID menu open, Escape 1 reveals Coruscant without closing
  the menu, Escape 2 closes Coruscant and reveals Sesswenna, and Escape 3 is
  inert.
- `alliance-day54-advice`, day 61: load restores no window; a galaxy
  double-click opens Corellian Sector, the first GID press opens the menu, and
  Escape leaves that Sector and the menu state unchanged. A rail-light press
  opens Message Index above the menu; closing the index leaves the menu open
  and its galaxy children hidden.

The final source-changing repair is
`1d7ea50f6a8dc84d76946f4c4124a53aebd812c4`. The documentation-only
reconciliation that contains this section does not change the code, fixture,
package inputs, or the following artifact identities:

| Final review-candidate object | SHA-256 |
|---|---|
| Production WASM | `d0e6b434b3a74314cd3391a1fdf98ebd65e96d67066df97a64b62d14dbd88baa` |
| Fixture WASM used by the browser gates | `be50230174ca151dbc4bd6f367f140c6fb7f88d02ee2449b40a40c331f34b703` |
| Runtime ORPK | `d804e130f915deb1bb65036d38e28ed0dd4f606faec25243134bd615e16607f8` |
| Build manifest | `d2ba62744d8f1ad536bb322d3497ce59c1a05c44d2ba21719db5de14c8619d90` |

Final review-candidate verification on the Linux VPS:

- `cargo check --workspace --all-targets` and the same command with
  `--features interface-test-fixtures`: pass with zero warnings.
- `rustfmt --check --edition 2021` on the 45 existing Rust files changed from
  the pull-request merge base, using `skip_children=true`: pass.
- `cargo test --workspace --no-fail-fast`: 3,202 passed, 0 failed, 46 ignored.
  The fixture-feature variant: 3,228 passed, 0 failed, 47 ignored.
- `cargo build --workspace --release`: pass.
- `cargo clippy -p rebellion-windowing --all-targets --no-deps -- -D warnings`:
  pass. The wider changed-package strict command is not green: it exits 101 on
  29 reported source locations already present in `origin/main`, with zero
  branch-introduced locations.
- `node --test tools/interface-parity/*.test.mjs`: 109 passed, 0 failed.
- `npm run build` in `tools/interface-parity`: production and isolated fixture
  builds pass, including production-fixture exclusion. `wasm-opt` was not
  installed, so the build retained its size-only warning.

The no-build browser command set followed that package build. Result reports
bind the fixture-driven rows to the fixture WASM hash above and, where used,
the runtime ORPK hash; Encyclopedia Publication binds to the production WASM.
The GID summary does not retain a WASM hash, so its row is deliberately only a
same-run regression observation rather than exact-artifact acceptance. None of
these rows is original-game or native-Windows acceptance:

| Command / gate | Result | Artifact boundary |
|---|---|---|
| `node windowing-contract.mjs --case=journey-a,journey-b,journey-c,journey-d,journey-e --no-build` | 20/20 executions; 4,600 assertions; zero diagnostics | fixture WASM |
| `node fleet-window.mjs --no-build` | 40/40 | fixture WASM and runtime ORPK |
| `node run.mjs --all --no-build` (GID) | 152/152; zero launch, execution, or recovered timeouts; 152/152 browsers closed | same-run observation; summary omits artifact hashes |
| `node fleet-move.mjs --no-build` | 12/12 | fixture WASM and runtime ORPK |
| `node orbital-strike.mjs --no-build` | 22/22 | fixture WASM |
| `node windowing-contract.mjs --case=adapter-smoke --no-build` | 4/4; 20 assertions; exact recipients and deltas; zero underlay actions | fixture WASM |
| `node fleet-finder.mjs --no-build` | 18/18 | fixture WASM and runtime ORPK |
| `node status-window.mjs --no-build` | 10/10 | fixture WASM |
| `node message-index-shell.mjs --no-build` | pass | fixture WASM and runtime ORPK |
| `node audit-batch.mjs --no-build` | pass | fixture WASM and runtime ORPK |
| `node sector-quadrants.mjs --no-build` | 12/12 | fixture WASM and runtime ORPK |
| `node encyclopedia-publication.mjs` with canonical local EData | pass | production WASM and runtime ORPK |
| `node scripts/validate-interface-parity-ledgers.mjs --check` | pass | tracked ledger inputs; not a browser artifact |
| `npm run check` in `tools/interface-parity` | pass; the tactical A0 matrix remains incomplete and has 0 strict parity acceptance | tracked catalog/matrix inputs; not a browser artifact |

All commands above ran directly on the VPS. No Mac Mini, RCH worker, private
or local network, native-Windows environment, or original-game runtime was
contacted or used. Generated WASM, browser run directories, proprietary data,
and local beads state remain outside the contribution commit. The local
`scripts/task-index.py --check` failure is also not claimed green: the ignored
working beads graph omits an epic present in the committed JSONL, so that
local-store drift is not used as pull-request evidence.

## Audit boundary and open work

No strict interface-parity cell changes in this reconciliation. The current
audit remains at 2 complete, 15 partial, 26 failing required families and
28/627 strictly accepted required cells. The following remain explicit:

- Original-runtime/W1 gaps and unsupported original behavior are not closed by
  the r29 port-side browser/native runs.
- Native-Windows parity is not established by Linux/X11 native evidence.
- Original bitmap, typography, layout, complete state matrices, and A0
  comparisons remain governed by their existing cells and evidence.
- System Finder content remains missing.
- The known-deviations register is unchanged: no deviation was added, reused,
  approved, retired, or silently used to close an original cell.
- progress.json was absent and was not created.

The shared initiative plan remains active for those external acceptance and
original-interface gaps. The bounded SW-12B/SW-12P/SW-13A/SW-13E evidence
sequence is complete and reconciled at r29, and the final PR #46 repair has a
separate exact-source VPS regression checkpoint. Neither is a 100% parity or
release claim.
