# Open Rebellion

Open Rebellion is a Rust reimplementation of Star Wars Rebellion (1998), with
native macroquad/egui and browser WebAssembly/WebGL targets.

The September 2026 audit is authoritative. The project is not yet 100%
functional; historical parity percentages are estimates, not acceptance proof.

## Active Work

- Start with `progress.json`, the ignored live recovery record.
- Use `docs/qa/2026-09-08-full-functionality-audit/README.md` for the audit index,
  feature ledger, evidence contract, and definition of done.
- Use `agent_docs/roadmap.md` for milestone order, including the protected
  Cloudflare Pages v1.0 release.
- Use `agent_docs/agent-tooling.md` to select Codex, Fable, Ghidra, QA, asset,
  and deployment workflows without loading unrelated skills.
- Complete one feature at a time. Update progress JSON, audit JSON, audit
  Markdown, and roadmap evidence together.
- Once a feature passes its stated gates, commit and push it before starting the
  next feature. Never check off a partial or inferred result.
- Work directly on `main`; do not create or retain side branches unless the
  user explicitly changes the main-only policy.
- Browser acceptance must use `codex-orchestrator` with Astra at medium effort.
  Retain inspected screenshots, console/network logs, and artifact hashes.
- GitHub Actions workflow definitions are intentionally local and untracked as
  of 2026-09-08. Verification is manual until M5 restores a reviewed CI provider.

## Commands

`~/.local/bin/cc` shadows Apple clang. Use this sanitized PATH for Cargo:

```bash
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/tomdimino/.cargo/bin cargo check --workspace
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/tomdimino/.cargo/bin cargo test --workspace
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/tomdimino/.cargo/bin cargo run -p rebellion-app -- data/base
./scripts/build-wasm.sh
./scripts/package-web.sh dev
```

Formatting and strict Clippy have known audited baseline failures. Run scoped
checks for touched code; do not report the workspace gates as green until their
dedicated findings close.

## Structure

- `crates/rebellion-core` — headless runtime types and simulation.
- `crates/rebellion-data` — DAT loading, seeding, and persistence.
- `crates/rebellion-render` — macroquad/egui rendering and interaction.
- `crates/rebellion-app` — native/WASM entry point and interactive loop.
- `crates/rebellion-playtest` — headless campaigns and JSONL telemetry.
- `tools/dat-dumper` — DAT codecs and TEXTSTRA extraction.
- `web` and `scripts` — browser shell, WASM builds, packaging, asset staging.
- `agent_docs` and `docs` — architecture, plans, mechanics, and QA evidence.

## Conventions

- Keep binary-layout `dat` types separate from runtime `world` types.
- Preserve original `DatId` identity; never persist slotmap keys directly.
- Require every DAT parser to round-trip original bytes through `DatRecord`.
- Keep `rebellion-core` free of rendering and platform I/O dependencies.
- Express simulation mutations as ordered `GameEffect` values applied by the
  authoritative integrator; economy runs before manufacturing.
- Prove bitmap display with a resource mapping, cache hit, and inspected runtime
  screenshot. Decode or HTTP success alone is insufficient.
- Keep password/session material in encrypted Cloudflare secrets. Never commit
  credentials, original game data, or generated proprietary asset packs.

## Boundaries

- Always: Preserve unrelated user changes and stage explicit paths only.
- Always: Verify in proportion to risk and record exact commands/results.
- Always: Use conventional, atomic commit messages and push every verified item.
- Ask: Before adding production dependencies, changing architecture, or breaking
  save, DAT, or network formats.
- Never: Run `gh auth setup-git` or switch the active `gh` account to repair a
  push; use the macOS keychain and repository-scoped credential username.
- Never: Claim 100% functionality until every supported P00–P40 pass is green
  and all P0/P1 findings are closed with release-artifact evidence.

## Troubleshooting

- Linker invokes a Claude/tmux launcher: rerun Cargo with the sanitized PATH.
- Browser boots black: wait for the bitmap manifest and all staged assets, then
  inspect console/network logs before assigning a visual result.
- Push omits `main`: run `git push origin main` and compare `HEAD` with upstream.
- Pages/Jekyll reports a missing local path: inspect tracked symlinks; local
  scratch links must remain ignored.
