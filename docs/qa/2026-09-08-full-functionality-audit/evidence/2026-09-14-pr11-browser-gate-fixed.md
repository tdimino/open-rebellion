# PR #11: browser-harness failures resolved

The complete muted GID gate now passes: **152/152 cases, zero failures,
zero launch retries; exit 0.** This supersedes the automated result in the
[initial failed gate](2026-09-14-pr11-browser-gate.md), whose raw evidence is
preserved. Independent Astra live acceptance remains pending.

## Changes

- Register a listener before each GID action and require a fresh matching
  command event. Prior openings cannot satisfy a subsequent click.
- Wait through the following paint before capturing or interacting with UI.
  Event delivery and paint share a bounded two-second deadline; listeners and
  timers are removed on success, timeout, or action failure.
- Keep Chrome's 30-second startup deadline. Retry a launch timeout once after
  a one-second pause, before any page or application exists. Retain duration
  and error details for every attempt. Other launch errors, page failures,
  and test assertions are never retried.
- Add eleven dependency-free Node regressions, available through
  `make test-interface-harness` and `npm run test:unit`.

No application source, acceptance bitmap, dependency version, or fixture
scenario changed. The complete rerun used the original tested WASM and runtime
pack; all 152 result records match the initial gate's artifact hashes.

## Investigation and validation

The original run had seven assertions checked before their expected console
events arrived and six browser launch timeouts. A delayed-event unit test
reproduced the fixed-80-ms race. The first targeted event-only browser runs
then exposed that the menu event precedes next-frame drawing: native-size
bitmap checks failed. A paint regression test reproduced that gap. Both were
fixed with event-and-paint synchronization; the bitmap assertions were kept.

Eight isolated muted Chrome launches all passed, taking 1.142–9.951 seconds.
The original intermittent OS/browser startup cause was not established.
Bounded startup recovery is covered by tests, including second-timeout failure
and non-timeout fail-fast behavior. **It was not needed in the passing full
run.** This is resilience to a startup failure, not proof that Chrome can
never time out.

| Check | Result |
|---|---|
| `make test-interface-harness` | 11 passed, 0 failed |
| `node --check tools/interface-parity/run.mjs` | Exit 0 |
| `node tools/interface-parity/validate-catalog.mjs` | 38 scenarios / 152 planned cases valid |
| Fleet targeted rerun | 4/4 passed, no launch retries |
| Idle Shipyards targeted rerun | 4/4 passed, no launch retries |
| Full `--all --no-build` rerun | 152/152 passed, no launch retries |
| Audit JSON, evidence references, hashes, `git diff --check` | Pass |

Full command:

```sh
env OPEN_REBELLION_CHROME_FOR_TESTING='/Users/will/Library/Caches/ms-playwright/chromium-1234/chrome-mac-arm64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing' node tools/interface-parity/run.mjs --all --no-build
```

The [machine-readable report](2026-09-14-pr11-browser-gate-fixed.json) records
source hashes for the uncommitted harness changes on base commit `ce0b17a`,
all targeted run IDs, the intermediate failed run IDs, and validation details.
Rust tests and Clippy were not rerun for these JavaScript/Make/documentation
changes. No JavaScript or documentation formatter is configured; existing
style and whitespace were checked.

## Retained evidence and limits

The full run is retained locally in
`.artifacts/interface-parity/2026-09-14T17-39-35-437Z-15870/`, including results,
screenshots, console/network logs, tested artifacts, harness source snapshots,
red/green regression logs, and the launch diagnostic. `SHA256SUMS` covers 1,407
files; its hash is in the JSON report. Generated binaries and licensed assets
remain outside Git.

Every case had exactly four HTTP-200 startup requests, no recorded browser
errors, and successful browser cleanup. The process check after exit found no
pinned Chrome process or harness PID 15870; the local server had exited.
All launches used `--mute-audio`, and the fixture kept in-game music muted.

The primary agent inspected the retained Alliance fleet menu-root and Empire
Idle Construction selection captures from the full run, plus the Alliance
Idle Shipyards selection capture from its targeted run. The JSON report
records their hashes. This was not independent live acceptance. All 152
passing cases are unbaselined; no original visual parity is claimed.

The required `codex-orchestrator` capability remains unavailable, so Astra's
live review is still pending. The user will follow up on that external tooling
separately. The automated failures are resolved, but the
broader step-7 acceptance checkpoint stays open for that required review.
