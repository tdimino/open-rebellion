# PR #11: initial full muted GID browser gate

The later [fixed harness rerun](2026-09-14-pr11-browser-gate-fixed.md) passes all 152 cases. The initial result below is preserved for provenance.

**This initial run did not pass.** At source commit `ce0b17a`, the complete
38-scenario × two-faction × two-viewport run attempted all 152 cases:
**139 passed, 13 failed; exit 1.** No case was skipped or retried into a pass.
The [machine-readable report](2026-09-14-pr11-browser-gate.json) lists every
failure, expected events found in retained logs, build hashes, and inspected
screenshot hashes.

## Execution

```sh
env OPEN_REBELLION_CHROME_FOR_TESTING='/Users/will/Library/Caches/ms-playwright/chromium-1234/chrome-mac-arm64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing' node tools/interface-parity/run.mjs --all
```

Chrome for Testing `151.0.7922.34` and Playwright Core `1.63.0` were used.
Each attempted case launched with `--mute-audio`; the fixture also mutes
in-game music. Production and isolated fixture WASM builds passed, as did
catalog validation and production fixture-token exclusion. The missing Rust
`wasm32-unknown-unknown` target was installed before the build.

The build emitted 11 fixture-app warnings. `wasm-opt` was unavailable, so
artifacts are unoptimized. MDATA.300 and COMMON.DLL audio were absent from the
build script's default external installation path; this muted run does not
verify audio. Asset staging found 51 DATs, 2,303 bitmaps, and 3,988 advisor frames.

## Failures

| Count | Failure | Evidence |
|---|---|---|
| 3 | Opening assertion after a fixed 80 ms wait | Alliance blockade at 1280×800 and Alliance fleet at both sizes. All final logs contain the expected `0x132` opening event. |
| 4 | Filter-selection assertion after a fixed 80 ms wait | Alliance Idle Shipyards at 640×480, Empire Idle Construction at 640×480, Alliance Troopers at 1280×800, Empire Fighter Squadrons at 1280×800. All final logs contain the expected selection event. |
| 6 | Chrome launch exceeded 30 seconds | All four Fleets Enroute cases and both Alliance Active Personnel cases. These failed before application requests. Subsequent browser launches recovered. |

The seven assertion failures show that checking the accumulated console log
after an arbitrary delay can run before the expected event is delivered.
Their later log entries do not turn the failed cases into passes: subsequent
probes did not run. The harness should wait for the expected event with a
bounded timeout before asserting. The intermittent Chrome launch failures
also need investigation. No application or harness source was changed during
this gate, and no acceptance threshold was relaxed.

## Evidence and cleanup

Raw results, screenshots, console/network diagnostics, build/run log,
production and fixture WASM, runtime pack, and shell files are retained locally
in `.artifacts/interface-parity/2026-09-14T16-07-36-387Z-10031/`.
`SHA256SUMS` covers 1,323 files; its SHA-256 is recorded in the JSON report.
Generated binaries and licensed assets are not included in this documentation
change. The tracked production WASM overwritten by the build was restored
after preserving the tested artifact locally.

146 cases reached the application and recorded exactly four startup requests;
the other six failed during browser launch. All case records report cleanup
closed. After process exit, `ps -axo pid,ppid,stat,rss,comm` confirmed no pinned
Chrome processes or harness PID remained; the local server exited with its
owning harness process.

The primary agent inspected four retained screenshots, listed with hashes in
the JSON report. The two menu-root captures show GID open for Alliance at
640×480 and Empire at 1280×800. All 139 passing cases are **unbaselined**;
zero screenshots matched reviewed golden baselines. No original visual parity
or strict interface-cell closure is claimed.

## Remaining acceptance

`AGENTS.md` and `agent_docs/agent-tooling.md` require Astra live browser
acceptance through `codex-orchestrator`. That capability was not found in
available tools, PATH, or local skill directories; the user has been asked
for its location. No independent Astra acceptance was performed.

Step 7 remains unchecked. Resolve the harness/event timing and launch failures,
rerun the complete gate, and complete the required live review before advancing.
