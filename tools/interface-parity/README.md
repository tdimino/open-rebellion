# Interface parity browser harness

This test-only harness exercises the original Galactic Information Display (GID)
menu and 38 deterministic states, for both factions at 640×480 and a letterboxed
1280×800 viewport. It launches a new pinned Chrome for Testing process for
each case with audio muted. The fixture bridge exists only in the separate
`interface-test-fixtures` WASM build. Production HTML and WASM are checked for
fixture imports and markers.

From this directory, install the pinned Node packages with `npm ci`, then run:

```sh
node run.mjs --smoke        # build both WASM variants; four quick cases
node run.mjs --all          # rebuild; all 152 faction/viewport cases
node run.mjs --all --no-build  # reuse a verified fixture build
node run.mjs --all --scenario=pan --no-build  # four focused faction/viewport cases
```

The runner expects the exact Chrome for Testing version in `browser.json`.
Set `OPEN_REBELLION_CHROME_FOR_TESTING` to its executable if it is installed
elsewhere. The build needs the local owned game data, the wasm32 Rust target,
and the tools used by `scripts/build-wasm.sh`. The runner serves only on
127.0.0.1. Screenshots, complete browser console/cache diagnostics, resource hashes, interaction probes,
four-request ledgers, and results go to the ignored
`.artifacts/interface-parity/<run-id>/` directory. Failed cases exit nonzero.

The generated test site also contains `battle.html`, a test-only launcher for the
real tactical scene. Its two links use the versioned codes in
[`tactical.catalog.json`](scenarios/tactical.catalog.json) and start muted. The
fixture proves deterministic production-path battle entry and the current
shell/control checkpoint only. Run `node run.mjs --battle --all --no-build` to
check both factions at both viewports. The tactical probes cover pause stability,
faction highlights, zoom round trips, source-shaped control misses, held pressed
art, and aperture isolation. Tactical controls and images still have no lossless
original-game baselines. See the [standalone battle plan](../../docs/plans/2026-09-12-tooling-standalone-space-battle-launcher.md).

The PNG comparison directory contains *implementation regression* baselines,
not original-game truth. `--update-goldens` creates only missing implementation
baselines, never overwrites a mismatch, and requires an accepted, SHA-verified
original source in `baselines/accepted-original.json`. A passing browser run without
original lossless captures does not complete a strict interface-parity cell.
At native 640x480, every run also checks all nine Message Index rail icons
pixel-for-pixel against the corresponding original STRATEGY BMPs. This proves
resting-art identity only, not the unread states or Message Index interactions.
For native-size GID menu captures, the runner checks all 634 root-border pixels
against STRATEGY 10100 through 10107 when unobscured. With a foreground system
window, it first confirms the overlap, then checks every still-visible border
pixel and records the covered remainder separately. A root-row hover probe
also guards against reintroducing the unsupported synthetic color wash.
Menu interior, typography, and original geometry remain separate acceptance work.
The runner compares captured menu, hover, legend, and pan/zoom states as well as
the initial frame once each state has its own reviewed original reference and
implementation regression baseline. Until then their comparison is recorded as
`unbaselined`, not an assertion of visual fidelity.
See the [interface audit](../../docs/qa/2026-09-10-interface-parity-audit/README.md)
and the [execution sidecar](../../docs/plans/2026-09-11-tooling-interface-parity-acceleration-sidecar.md).
