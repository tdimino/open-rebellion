# Game Options implementation checkpoint — 2026-09-24

Scope: PRE-03 / RE-OPT-01, P31, local bead `orlocal-ddb`.
Base: `e42776366d7e45aab90612bf05930dd6aa134801`.
This is an implementation checkpoint, **not strict original-game acceptance**.
PRE-03 remains failing and no acceptance cell is promoted.

## Implementation

Main-menu command 0x68, campaign globe/F1 (0x133), and tactical Options now
open the same COMMON 20002 bitmap surface. The caller remains in memory;
returning does not recreate the campaign or battle. Options rendering does
not run the campaign or tactical simulation. Battle save/load/restart and
pre-battle display controls are disabled. Music and SFX reuse the audio state.
Starfield, planet, pyrotechnics and high-detail preferences affect rendering.

Six original rows use the existing save services. Overwrite, current-campaign
load, restart, exit and deletion require confirmation. Escape cancels a
confirmation first, then returns to the caller. REBDLOG 10623–10627 supplies
the original confirmation background and buttons. Unreadable slots remain
occupied, including browser payload-only or metadata-only saves, so a corrupt
save cannot bypass overwrite confirmation. Load errors remain on the screen.
No save schema or game-data format changes are introduced.

Compatibility shortcuts while options is open:

- **F8 / Ctrl+L:** save-management load panel (six rows), outside battle.
- **F9 / Ctrl+S:** save-management save/delete panel (six rows), campaign only.
- Escape closes the compatibility panel before returning from options.

These routes share confirmation guards. Both interfaces offer slots 1–6, as
requested. Existing backend data in slots 7–10 remains untouched; there is no
migration or deletion. The constructor/manual has no delete button; a new
bitmap deletion control is not invented. Holocube remains visibly disabled
because upstream has no implementation. Preferences last for the app session;
settings persistence is not implemented in this checkpoint.

## Source evidence

[Game Options source contract](../../../../ghidra/notes/game-options.md) records
constructor/handler addresses, commands, geometry, resource IDs and native
context restrictions against the owned executable's SHA-256. The main-menu
retrieval hint is corrected: 0x68 is Save/Load; 0x67 is multiplayer. REBDLOG is
now staged as a seventh UI DLL. Its single named corner resource gets the
explicit staging-only alias 4294967295; native numeric identities are preserved.

## Verification

Native Cargo commands used sanitized `PATH=/usr/bin:/bin:/home/will/.cargo/bin`,
`CARGO_TARGET_DIR=/data/tmp/game-options-target`, and the existing local ALSA
link shim `LIBRARY_PATH=/data/tmp/orlocal-l7g-1-29-alsa.sjSyRO`.

- `cargo check --workspace`: pass, existing manifest/future-incompatibility warnings.
- `cargo test --workspace`: **759 passed, zero failures**, with contributor-owned
  DAT, text and UI files linked into ignored `data/base/`. The initial unstaged
  run stopped at `troop_classes` with a missing original-data file; staging the
  owned fixtures also enables both bitmap-dependent tactical tests.
- Renderer-only run without source fixtures: 250 passed, one ignored, two
  source-bitmap tests filtered out. Seven Game Options tests cover confirmation,
  cancellation, six-slot save management, invalid intents, and real egui pointer
  input while modal or in battle. The overwrite/legacy/corruption regression
  tests were observed failing before their fixes.
- `OPEN_REBELLION_TEST_SOURCE=<owned-install> go test ./tools/stage-ui-assets`:
  pass, including extraction of all 24 REBDLOG resources and confirmation BMP
  dimensions. No original assets or generated packs are committed.
- Sol high read-only review identified inaccessible deletion and corrupt-save
  overwrite bypass; both were fixed and re-reviewed without further findings.
  The review used the available subagent tool because codex-orchestrator was
  unavailable; it does not count as the prescribed orchestrator acceptance.

## Browser implementation smoke

Release WASM build passed. The production build was served locally with an
owned-data runtime pack, using Chromium 1234, a 1280×960 viewport and
`--mute-audio`; in-game music was also switched off. Both Alliance and Empire
passed `tools/interface-parity/game-options-smoke.mjs`: save, overwrite cancel,
load cancel/accept with verified fingerprints, deletion cancel/accept,
return/re-entry, corrupt-load preservation, corrupt-overwrite cancellation,
and retention of an older hidden slot. Both had zero page errors and four
HTTP 200 startup requests. Screenshots of the window, six-row management panel,
confirmations, errors and returned campaigns were inspected. Browser and server
were closed after the run.

Reproduce after serving a build and owned runtime pack:

```sh
CHROMIUM_PATH=<chromium-executable> OPTIONS_TEST_URL=http://127.0.0.1:8769 \
  OPTIONS_TEST_OUTPUT=<private-evidence-directory> \
  node tools/interface-parity/game-options-smoke.mjs
```

Private local evidence: `/data/projects/open-rebellion/agent-work/game-options-2026-09-24/`.
Screenshots and source assets are not committed. `sha256.json` lists every
retained screenshot, log and test result. SHA-256:

- Tested WASM: `6e57346a1b5791ce0d7cc5e5055a639fe404b457c957be79581c27bbcfee855c`.
- Evidence manifest: `26d30048141d68ed171214219e35944fdf4bbfab2f89b19d125ff964b7f1bbc5`.

The smoke proves the listed implementation actions, not exact original
pixels, slider/audio behavior, or live tactical return. The test pack contains
no audio or tactical 3D resources. Earlier exploratory runs exposed a
browser-owned Ctrl+L shortcut and were superseded by the F8/F9 smoke.

## Remaining acceptance boundary

Exact original text/font metrics, slider transfer/capture semantics,
context-specific return images, saved-slot faction badges, native deletion
input, settings persistence and Holocube remain incomplete or unverified.
Browser audio audibility and tactical rendering with each switch disabled
remain unverified. Original-executable A0 captures and live Astra acceptance
through codex-orchestrator are still required. That orchestrator/browser tool
is unavailable in this environment. Implementation screenshots and automated
smoke checks must not be treated as a substitute.
