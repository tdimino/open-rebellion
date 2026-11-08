# Strategic Windowing Toolkit: Evidence Reconciliation

Date: 2026-11-08

This record joins the window manager's evidence: the earlier fixture-browser,
production-package and Linux-native lanes, each bound to the candidate
artifacts below, and the workspace and browser gates on the reviewed source,
bound to the artifacts in "Review verification". The earlier lanes ran
against one candidate with no source change between their runs:

| Evidence object | Stable identity |
|---|---|
| Candidate manifest | SHA-256 28ae5cc5e9b798d02e30a3bb6b0785bcb40781fd0e0e27d8f185808eb09b8221 |
| Candidate evidence ledger | 89 entries; SHA-256 14d5215f61e476e9f1a10fb7ccae93a726dd88b6c8479120326fea7060ce6b66 |
| Fixture WASM | 6b543f68c27e58ddabd5ad40637ac7909be7f28b8e9117af3de44e3e46daacaa |
| Production WASM | 06b97750812ff1f8429ca4ee34a1d5b730a3dc9f802823d10d158ce638e46a77 |
| Runtime pack | 4ad37135a91b930db616ff5fd7c9403554939efd0461faebc4df105e260c9ea2 |
| Frozen native executable, both faction copies | 4a49ddef6db2d48efbe035b3295e36e544aa67db8ef37393bd4c833d5ba95edb |

## Accepted backend evidence

| Lane | Accepted result and stable values | Recursive evidence seal |
|---|---|---|
| Fixture-browser | Ten accepted fixture-browser gates with both-faction coverage; report SHA-256 7d0ac290fe4b4c3ecdc891be92831ef20ec3267134022720d574a1ff32f74cc5; case manifest SHA-256 66780335b320d6de27ea1c2cb3ade6463b369b8f84c56abe0993a6d2af53adba | 1,114 entries, SHA-256 bf3870e91a0431c4b87e789dc4fcae74e4d37d16517156ae4b817c2929d757dc |
| Production-package/browser | Both factions passed all 11 production journey checks; report SHA-256 c25bd606caba8bc679ce0d934b297e4dc1e6f5194421d1bde778756e4cfe0226; case manifest SHA-256 6c84440b7dfbf16bd83eaebeb2b7a12a414a95547fcde9a90a2fc49b08b39cdd | 44 entries, SHA-256 2169a89bb44be30d990de2485e9c0c7dbee2145251a472fce4cce8aa422f50a4 |
| Alliance Linux-native | Seed 1337; day 0 focus-loss observation at WindowId(4); tactical day 1 returned through Alliance Victory Results to Galaxy day 1; report SHA-256 043dabc49821343b21dea0ab5818160fc70c2636ffee67a6db32f529ead1414e; tactical continuation SHA-256 315be575c8ad1c07bffc9bbaf73c1c294fac3842be79f9bee872755d89653260 | Original 70-entry ledger SHA-256 12cfdb43a9b1450b0143a941c9dc5f67445a7f1bb73baaabd952631f404affc1; interaction continuation 107-entry ledger SHA-256 4f0084076699b897bd395e5c3d802cebdfd48d270081d8136a1368411f3daa26; tactical continuation 135-entry ledger SHA-256 9e36d59b1fda641f04bb77caaa5419f54f6b2d875044ff63dbbce3b127e5e1a0 |
| Empire Linux-native | Seed 1337; day 0 focus-loss observation at WindowId(2); tactical return passed through Empire Victory Results to Galaxy day 3; outside-menu press retained day 0 Paused; report SHA-256 50df969657f9292ed2eb6c75f3994cde77675acfe692508e535e13c758e0d9d7; menu continuation SHA-256 b3d853d8fe52283803dca50e3940a53e9a654430da0098c193c5d888f0142998 | Main 94-entry ledger SHA-256 f79e8cf1101dae0f809ccd8071f812bf7f4afad8bb425a35253b7abf63b07ca8; menu 12-entry ledger SHA-256 cd2ab290efa4edc53ebf25fbedccb44a99c54add257a64e7a8583cd88dc1ae47 |

The coordinator independently accepted each backend after checking its report,
case/result manifest, ledger, and applicable served or executable identity.
All cited ledgers were rechecked from the repository root while preparing this
reconciliation. The Empire Linux-native row-click observations remain classified as
unverified, not as a product failure; the coordinator clarified that selecting
the Status row is not part of the required native outside-press criterion. The
focused menu continuation instead proves dismissal on the outside press,
suppression of the next-frame orphan release, and response to a later fresh
press.

## Architecture and proven contracts

The implementation separates four responsibilities:

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
behaviors were checked on the candidate above.
This is evidence for window identity, ordering, event ownership, focus/capture
cancellation, scene boundaries, content receipts, and the declared journeys;
it is not a claim that every source-level API described in historical design
text exists or that every original window surface is complete.

## Original-game focus anchors

The manager's focus rules are settled against these original runs, named by
save, day and visible result:

- `empire-fleet-to-yaga-minor`, days 38-45: load restores Sesswenna below
  Coruscant. Without a GID press, Escape closes Coruscant and the next Escape
  is inert. With the GID menu open, Escape 1 reveals Coruscant without closing
  the menu, Escape 2 closes Coruscant and reveals Sesswenna, and Escape 3 is
  inert.
- `alliance-day54-advice`, day 61: load restores no window; a galaxy
  double-click opens Corellian Sector, the first GID press opens the menu, and
  Escape leaves that Sector and the menu state unchanged. A rail-light press
  opens Message Index above the menu; closing the index leaves the menu open
  and its galaxy children hidden.

## Review verification

The reviewed source was built and tested on the Mac Mini through
`scripts/mini-cargo.sh`; the browser gates ran on Chrome for Testing
153.0.8010.12, muted, against the fixture site built from that source:

| Object | SHA-256 |
|---|---|
| Fixture WASM used by the browser gates | `1562146ffb9ef8376b625b641fdc1fa54b51df7433a37fd6c65e35ab84f9921a` |
| Runtime ORPK | `40eb0b3ed914c719179fc3225f9d3f7ea8edab64a785c7e1f9804e375ee02236` |

- `scripts/gates.sh`: workspace tests with and without the fixture feature (6,438 passed, 0 failed, 93 ignored in all), the replay golden, telemetry coverage, provenance, WASM and both leak greps pass; the beads gate fails on this branch because its `tasks/INDEX.md` lacks the review plan's epic row, which `main` regenerates.
- `cargo mutants` on the review fixes' diff: every mutant in the new helpers is caught; the nine missed sit in `main` and `apply_panel_action`, the frame loop no unit test reaches.

| Gate | Result |
|---|---|
| `node --test *.test.mjs` | 109 passed, 0 failed |
| `windowing-contract.mjs` (journeys A-E) | pass; 4,600 assertions |
| `windowing-contract.mjs --case adapter-smoke` | pass; 20 assertions |
| `windowing-contract.mjs --case identity-history` | pass; 576 assertions |
| `windowing-contract.mjs --case input-scenes` | pass; 504 assertions |
| `windowing-contract.mjs --case modal-boundaries` | pass; 3,520 assertions |
| `run.mjs --all` (GID) | 152 of 152 |
| `fleet-window.mjs` | pass |
| `fleet-finder.mjs` | pass |
| `fleet-move.mjs` | pass |
| `orbital-strike.mjs` | pass |
| `status-window.mjs` | pass |
| `mission-dialog.mjs` | pass |
| `build-selection.mjs` | pass |
| `sector-quadrants.mjs` | pass |
| `audit-batch.mjs` | pass |
| `message-index-shell.mjs` | pass |
| `encyclopedia-index-shell.mjs` | fail, both factions: about 3,000 differing pixels per selected or held category against its reference; `main` fails it the same way |

Not run: `encyclopedia-publication.mjs`, `encyclopedia-surface.mjs`, `encyclopedia-canonical-surface.mjs`, `encyclopedia-art.mjs`, `window-restore.mjs`, `endings.mjs`, `game-options-smoke.mjs`, `fleet-registry.mjs` and `replay-browser.mjs`. None of these rows is original-game or native-Windows
acceptance.

## Audit boundary and open work

No strict interface-parity cell changes in this reconciliation. The current
audit remains at 2 complete, 15 partial, 26 failing required families and
28/627 strictly accepted required cells. The following remain explicit:

- Original-runtime/W1 gaps and unsupported original behavior are not closed by
  the port-side browser and native runs.
- Native-Windows parity is not established by Linux/X11 native evidence.
- Original bitmap, typography, layout, complete state matrices, and A0
  comparisons remain governed by their existing cells and evidence.
- System Finder content remains missing.
- The known-deviations register is unchanged: no deviation was added, reused,
  approved, retired, or silently used to close an original cell.

The shared initiative plan remains active for those external acceptance and
original-interface gaps. Neither the earlier lanes nor the review verification
is a 100% parity or release claim.
