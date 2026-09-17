# Open Rebellion

Open Rebellion is a Rust reimplementation of Star Wars Rebellion (1998), with
native macroquad/egui and browser WebAssembly/WebGL targets.

The September 2026 audit is authoritative. The project is not yet 100%
functional; historical parity percentages are estimates, not acceptance proof.

## Contribution Model

- This repository is an upstream project that we contribute to; we do not own
  its remote. Every contribution must be submitted for review through a pull
  request.
- Treat `origin` (`tdimino/open-rebellion`) as read-only upstream and
  `will-forster` (`will-forster/open-rebellion`) as the writable fork.
- Never push directly to `origin` or to either remote's `main` branch. Never
  force-push a published branch unless the user explicitly authorizes it.
- Use one focused topic branch and one pull request per feature or fix. Do not
  stack unrelated work into the same branch or PR.
- Keep `.beads/` local. Use its database for issue tracking, but never stage or
  commit beads data to this repository.

## Starting Work

1. Run `git status --short --branch` and `git remote -v`. Preserve all existing
   changes; never reset, discard, or absorb another contributor's work.
2. Read `progress.json`, the current audit README, `agent_docs/roadmap.md`, and
   the relevant system documentation before choosing a task.
3. Run `git fetch origin --prune` and base new work on the current
   `origin/main`, not a stale local `main`.
4. Create a focused branch such as `feat/<short-description>`,
   `fix/<short-description>`, or `docs/<short-description>`. If the primary
   checkout is dirty, use an isolated worktree rather than carrying unrelated
   changes onto the branch.
5. Confirm that the branch contains only the intended work before editing.

## Active Work

- Use `docs/qa/2026-09-08-full-functionality-audit/README.md` for the audit index,
  feature ledger, evidence contract, and definition of done.
- Use `agent_docs/roadmap.md` for milestone order, including the protected
  Cloudflare Pages v1.0 release.
- Use `agent_docs/agent-tooling.md` to select Codex, Fable, Ghidra, QA, asset,
  and deployment workflows without loading unrelated skills.
- Complete one feature at a time. Update progress JSON, audit JSON, audit
  Markdown, and roadmap evidence together.
- Once a feature passes its stated gates, commit it, push its topic branch to
  `will-forster`, and open an upstream PR before starting the next feature.
  Never check off a partial or inferred result.
- Use Astra only for live browser or computer-use acceptance through
  `codex-orchestrator`: low effort for routine checks, medium for complex or
  release-significant journeys. Do not assign Astra source, implementation,
  reverse-engineering, or documentation reviews. Retain its inspected
  screenshots, console/network logs, and artifact hashes.
- Use Codex Sol through `codex-orchestrator`, at high or extra-high effort,
  when risk warrants independent code, reverse-engineering, architecture,
  persistence/network, security, or documentation review. Routine, well-tested
  feature slices do not require a Sol review.
- Launch every Open Rebellion browser-test session with Chromium
  `--mute-audio`, and keep the in-game music control muted unless the test
  explicitly verifies audio. Close the test browser and local server when the
  run finishes.
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

## Pull Request Workflow

1. Verify the final diff and run the checks required by the affected audit or
   subsystem. Record exact commands and results.
2. Stage explicit paths only. Confirm that `.beads/`, credentials, original
   game data, generated proprietary assets, and unrelated changes are absent.
3. Create an atomic conventional commit, then push the current branch to the
   fork with `git push -u will-forster HEAD`.
4. Open a PR against upstream `main`, for example:

   ```bash
   gh pr create \
     --repo tdimino/open-rebellion \
     --base main \
     --head will-forster:<branch>
   ```

5. Include the scope, audit or issue identifiers, verification commands,
   results, known baseline failures, and retained evidence in the PR body.
6. Address review on the same topic branch. Do not merge the PR or bypass
   upstream approval unless the repository owner explicitly asks us to.
7. After upstream merges the PR, fetch `origin` and start the next branch from
   the updated `origin/main`.

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
- Always: Use conventional, atomic commits, push only to the fork, and submit
  every contribution through an upstream PR.
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
- Push targets the wrong repository: stop and inspect `git remote -v`; the topic
  branch must go to `will-forster`, never `origin`.
- PR cannot find the head branch: run `git push -u will-forster HEAD`, then use
  `--head will-forster:<branch>` when creating the PR.
- Upstream advanced before the first push: fetch `origin` and rebase the local
  topic branch on `origin/main`. Do not rewrite a published branch without
  explicit approval.
- Pages/Jekyll reports a missing local path: inspect tracked symlinks; local
  scratch links must remain ignored.
