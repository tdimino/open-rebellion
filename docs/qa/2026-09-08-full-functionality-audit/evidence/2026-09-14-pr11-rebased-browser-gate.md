# PR #11: browser gate after rebase

The full muted GID gate passes on **`e7533c6`**, rebased onto upstream
**`741bcfb`**: **152/152 cases passed, zero failures, zero launch retries;
exit 0.** Both production and fixture WASM were rebuilt from the rebased
source. This is fresh artifact evidence, not reuse of the pre-rebase result.

## Command and checks

```sh
env OPEN_REBELLION_CHROME_FOR_TESTING='/Users/will/Library/Caches/ms-playwright/chromium-1234/chrome-mac-arm64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing' node tools/interface-parity/run.mjs --all
```

- Production and isolated fixture WASM builds passed.
- Catalog validation passed: 38 scenarios × two factions × two viewports.
- Production fixture-token exclusion passed.
- All 152 expected case IDs appear exactly once and pass.
- Each case used a fresh muted Chrome launch, loaded exactly four successful
  HTTP requests, recorded no browser errors, and completed cleanup.
- No launch timeout or retry occurred; no unstable screenshots were reported.
- Every case's WASM and runtime-pack hashes match the retained fresh build.

Chrome for Testing `151.0.7922.34` and Playwright Core `1.63.0` were used.
The browser launched with `--mute-audio`; the fixture also mutes in-game music.

## Retained evidence

The [JSON report](2026-09-14-pr11-rebased-browser-gate.json) records artifact
and harness hashes, the full result summary, and inspected screenshot hashes.
Raw screenshots, case results, console/network diagnostics, the build/run log,
and copies of the tested artifacts are retained locally in:

`.artifacts/interface-parity/2026-09-14T19-01-30-472Z-20165/`

`SHA256SUMS` covers 1,394 files; its hash is included in the JSON report.
Generated binaries and licensed assets remain outside Git. The tracked WASM
file overwritten during the build was restored after preserving the tested
production artifact. No application or harness source changed for this rerun.

The primary agent inspected the Alliance fleet menu-root and Empire Idle
Construction selection captures at 640×480. Their menu and selected-mode
states are visible and their hashes are retained. All 152 cases remain
**unbaselined**; this is automated regression evidence, not original visual
parity or independent live acceptance.

After exit, `ps -axo pid,ppid,stat,rss,comm` confirmed no pinned Chromium
process or harness PID 20165 remained. The local server exited with the
harness process.

## Limits and provenance

Both WASM builds emitted three renderer and eleven app compiler warnings.
`wasm-opt` is unavailable, so the retained artifacts are unoptimized.
MDATA.300 and COMMON.DLL audio are missing at the script's default external
installation path; this muted gate does not verify audio. No strict Clippy
or workspace-format success is inferred from these results.

The user is handling the external `codex-orchestrator` follow-up separately;
required independent Astra acceptance remains pending. The earlier
[pre-rebase passing gate](2026-09-14-pr11-browser-gate-fixed.md) and
[initial failed gate](2026-09-14-pr11-browser-gate.md) remain preserved with
their original source and artifact provenance.
