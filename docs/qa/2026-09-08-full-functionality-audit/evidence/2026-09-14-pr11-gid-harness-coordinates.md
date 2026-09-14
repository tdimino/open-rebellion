# PR #11: GID browser-harness coordinates

Step 6 updates `probeGid` in `tools/interface-parity/run.mjs` from the
side-globe rectangles to the renderer's command `0x132` bottom controls:

| Faction | Rectangle (x, y, width, height) | Logical click center |
|---|---|---|
| Alliance | `(446, 406, 27, 16)` | `(459.5, 414)` |
| Empire | `(519, 434, 37, 25)` | `(537.5, 446.5)` |

The existing uniform scaling and letterbox offsets apply to both the center
and outside-edge probes. Hover and held-pressed captures use the corrected
center. The opening assertion now requires the logged command `0x132` and
`destination=gid_menu status=opened_original`; the outside probe continues to
reject any GID opening, regardless of command ID.

## Scoped verification on 2026-09-14

- `node --check tools/interface-parity/run.mjs`: exit 0.
- A dependency-free `node --input-type=module` geometry check evaluated the
  actual probe setup against both command `0x132` records parsed from
  `crates/rebellion-render/src/cockpit.rs`. All four faction/catalog-viewport
  combinations matched; each center was inside and each outside point was
  outside the respective rectangle.
- `node tools/interface-parity/validate-catalog.mjs`: exit 0; 38 scenarios,
  152 catalog executions validated (not browser executions). Initially blocked
  by missing dependencies; `npm ci --ignore-scripts --no-audit --no-fund`
  installed the existing locked packages after the offline cache proved incomplete.
  No dependency manifests or lockfiles changed.
- Audit JSON parsing and checkpoint links: pass.
- `git diff --check`: exit 0.

No Make target covers this JavaScript harness. No Rust changed, so Cargo
formatting, tests, and Clippy were not rerun. The full muted browser gate and
retained runtime evidence remain step 7; this checkpoint does not claim
browser or visual acceptance. Historical interface-ledger corrections remain
step 8.
