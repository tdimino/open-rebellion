---
title: "Agent Tooling"
description: "Project-specific routing for the Claude Code Minoan and Codex skill toolbelt"
category: "agent-docs"
created: 2026-09-08
updated: 2026-09-24
tags: [agents, skills, codex, fable, ghidra, qa, research, gemini, audio]
---

# Agent Tooling for Open Rebellion

Use the smallest skill set that fits the task. Read the selected skill's
`SKILL.md` before acting, keep generated evidence out of Git unless the audit
explicitly calls for it, and record material conclusions in the audit JSON and
Markdown rather than leaving them only in agent output.

Claude Code Minoan skills link to their canonical definitions in
[`tdimino/claude-code-minoan`](https://github.com/tdimino/claude-code-minoan).

## Required Review Lanes

| Tool | Use it for | Project rule |
|------|------------|--------------|
| [`codex-orchestrator`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/integration-automation/codex-orchestrator) | Risk-based independent review, architecture checks, debugging, and real-browser acceptance | Use Sol high or extra-high when consequential architecture, reverse-engineering ambiguity, persistence/network formats, security, or unusual diff risk warrants it; routine, well-tested slices need no separate Sol review. Use Astra only for live browser/computer-use acceptance: low for routine checks and medium for complex or release-significant journeys. Retain inspected screenshots, console/network logs, and artifact hashes. Run evidence agents from a dedicated `/tmp` workspace so the repository stays read-only. |
| [`fable`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/core-development/fable) | Deep cross-cutting parity audits, browser/multiplayer optimization, and synthesis across prior findings | Fable 5.1 is the default; `--naos` selects 5.0. Verify the reported `MODEL:` line and verdict trailer before trusting results, then weave accepted findings into the audit and roadmaps. Keep `.subdaimon-output/` local and untracked. |

## Reverse Engineering

Open Rebellion's Ghidra workflow is project-native rather than a general skill:

- Read `agent_docs/ghidra-re.md` before reverse-engineering game behavior.
- Use the existing Open Rebellion Ghidra project, Jython scripts, decompilation
  corpus, and `ghidra/notes/` sources described there.
- Record addresses, call chains, field offsets, and confidence, then connect the
  finding to a Rust test or acceptance fixture.
- Do not use [`bg3se-macos-ghidra`](https://github.com/tdimino/bg3se-macos/tree/main/tools/skills/bg3se-macos-ghidra); that skill encodes Baldur's Gate 3-specific
  binaries, layouts, launch flows, and terminology.

## Supporting Skills

| Skill | Use when |
|-------|----------|
| [`test-harness-auditor`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/core-development/test-harness-auditor) | Auditing build, test, lint, browser, and debug gates before M5 CI restoration. |
| [`cloudflare`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/integration-automation/cloudflare) | Implementing the password-protected Pages/Functions v1.0 deployment, secrets, headers, environments, and rollback. |
| [`architecture-md-builder`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/core-development/architecture-md-builder) | Updating the architectural map after an approved cross-crate or multiplayer boundary change. |
| [`claude-md-manager`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/core-development/claude-md-manager) | Keeping `CLAUDE.md` concise, accurate, and progressively disclosed. |
| [`agents-md-manager`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/core-development/agents-md-manager) | Maintaining the cross-agent `AGENTS.md` contract and validating its limits. |
| [`design-audit`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/design-media/design-audit) | Scoring accessibility, responsiveness, rendering quality, and browser performance without silently changing the UI. |
| [`keenable`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/research/keenable) | Default or high-volume web search, point-in-time queries, and prompt-guided extraction from known URLs. |
| [`exa-search`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/research/exa-search) | Semantic discovery, academic research, and finding sources related to a strong seed page. |
| [`firecrawl`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/research/firecrawl) | Extracting clean content from selected or JavaScript-heavy pages and mapping or crawling sites. |
| [`nano-banana-pro`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/design-media/nano-banana-pro) | Generating or editing Gemini 3 Pro images, including reference-guided and batch workflows. |
| [`gemini-claude-resonance`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/design-media/gemini-claude-resonance) | Iterating through shared-memory Claude–Gemini visual dialogue, analysis, and faithful transformation. |
| [`image-forge`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/design-media/image-forge) / [`sprite-forge`](https://github.com/tdimino/claude-code-minoan/tree/main/skills/design-media/sprite-forge) | Preparing owned replacement art or sprite assets; never repackage copyrighted source data. |
| `parakeet` (local, `~/.claude/skills/parakeet`) | Identifying original WAVE resources by content (see below); two MLX engines, no NeMo. |

## Test Quality Gates

`cargo-mutants` is a local CLI (`cargo install --locked cargo-mutants`), not a
workspace dependency. It changes small pieces of code and reports each change
the tests fail to notice. A surviving mutant is either a missing test or code
no test can reach; close it or record why it stays.

```bash
env PATH=/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/Users/tomdimino/.cargo/bin \
  cargo mutants -p rebellion-core --file crates/rebellion-core/src/tick.rs
```

Scope every run to the files a change touches; whole-workspace runs are slow.
Tests that need original DATs are `#[ignore]`d and the mutant copy omits the
ignored `data/base`, so mutants cannot see them. Check seeding and replay code
with `make test-assets` instead.
Mutants in rendering code often survive because the unit tests draw nothing.
Browser acceptance covers that code, so record those survivors instead of
writing tests that only mirror the drawing calls.

`proptest` is deferred until economy or combat math has edge cases the replay
goldens do not pin. Adding it needs approval as a dev dependency. Formal
verification (`kani`) is out of scope for now.

## Voice Resource Identification

Use this to map an original `WAVE` resource ID (TACTICAL, VOICEFXA/E, COMMON DLLs)
to what it says, or to confirm that it is SFX, before wiring it into an audio path:

1. Extract the DLL's `WAVE` resources to a directory outside the repository. Most
   are 8-bit unsigned PCM at 11,025 Hz, and ffmpeg decodes them directly.
2. Run `~/.claude/skills/parakeet/scripts/batch_transcribe.py <dir> --engine both
   --out .artifacts/voice-id/<dll>.jsonl` without hotwords first.
3. Read `status`. `agreed` is corroborated speech and `sfx_likely` is non-speech.
   `needs_listen` means a person must listen before the evidence cites it. You
   may re-run only those clips with `--context-file` and a small proper-noun
   primer as a third opinion. Dumping every TEXTSTRA string made Qwen3 recite
   the list. Never commit a derived vocabulary file.
4. Evidence cites the resource ID, the WAVE `sha256`, the engine models and
   revisions, and the verified line. Commit IDs and hashes, never the audio.

ASR is evidence, not authority. A transcript alone never closes a parity cell.

Baseline, 2026-09-24: 61 of TACTICAL.DLL's 66 `WAVE` resources (13000–13065)
are `sfx_likely` under both engines. Five are `needs_listen`: 13034, 13046, and
13052, plus 13019 and 13059, where Qwen3 hears a single plausible word ("Yeah.",
"Okay.") and Parakeet hears nothing. All 285 VOICEFXA/VOICEFXE resources are
speech: 270 agreed and 15 need a listen.

## Guardrails

- Skills assist implementation and verification; they do not lower a feature's
  acceptance criteria or turn an inferred result into a pass.
- Keep passwords, API tokens, session secrets, and proprietary game assets out
  of prompts, reports, screenshots, and commits.
- Preserve unrelated work. Stage explicit paths, update the evidence ledger,
  and commit and push each verified feature before advancing.
