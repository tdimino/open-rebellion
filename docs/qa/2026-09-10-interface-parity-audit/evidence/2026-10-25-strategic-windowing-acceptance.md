# Strategic Windowing Toolkit: r29 Reconciliation and PR #46 Review Repair

Accepted-candidate date: 2026-10-25
Accepted-candidate source: a912b6a95e44ee1b6a00861b68bdf74ce9eb197a
Review-repair date: 2026-11-05
Review-repair source: 971f23cb171785e2fe21f17335b178ef09ebd3ba

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

The additive repair at
`971f23cb171785e2fe21f17335b178ef09ebd3ba` addresses the maintainer review
of head `44704a7e139389379354fb442cde7b6da0c89c80`. Its original-game focus
anchors are named by save, day, and visible result:

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

Review-repair verification on 2026-11-05:

- `git diff --check`: pass.
- `rustfmt --check --edition 2021` on every changed Rust source, with
  `skip_children=true` for the application root: pass.
- `node --check tools/interface-parity/sector-quadrants.mjs`: pass.
- Direct standard-library-only `rustc --test` coverage for
  `rebellion-windowing`: 154 passed (18 unit, 4 contract, 45 lifecycle,
  31 open-order, 13 reconciliation, 27 routing, 16 scene tests).
- The required `scripts/mini-cargo.sh` full-workspace fmt/check path was
  attempted repeatedly, but the configured `mac-mini-ts` host failed DNS
  resolution before sync or compilation. No full-workspace success is claimed
  by this repair record.

## Audit boundary and open work

No strict interface-parity cell changes in this reconciliation. The audit
remains at 1 complete, 15 partial, 27 failing required families and 13/627
strictly accepted required cells. The following remain explicit:

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
sequence is complete and reconciled at r29; this is not a 100% parity or
release claim.
