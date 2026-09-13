---
title: "Agent Tooling"
description: "Project-specific routing for the Claude Code Minoan and Codex skill toolbelt"
category: "agent-docs"
created: 2026-09-08
updated: 2026-09-08
tags: [agents, skills, codex, fable, ghidra, qa]
---

# Agent Tooling for Open Rebellion

Use the smallest skill set that fits the task. Read the selected skill's
`SKILL.md` before acting, keep generated evidence out of Git unless the audit
explicitly calls for it, and record material conclusions in the audit JSON and
Markdown rather than leaving them only in agent output.

## Required Review Lanes

| Tool | Use it for | Project rule |
|------|------------|--------------|
| `codex-orchestrator` | Risk-based independent review, architecture checks, debugging, and real-browser acceptance | Use Sol high or extra-high when consequential architecture, reverse-engineering ambiguity, persistence/network formats, security, or unusual diff risk warrants it; routine, well-tested slices need no separate Sol review. Use Astra only for live browser/computer-use acceptance: low for routine checks and medium for complex or release-significant journeys. Retain inspected screenshots, console/network logs, and artifact hashes. Run evidence agents from a dedicated `/tmp` workspace so the repository stays read-only. |
| `fable` | Deep cross-cutting parity audits, browser/multiplayer optimization, and synthesis across prior findings | Fable 5.1 is the default; `--naos` selects 5.0. Verify the reported `MODEL:` line and verdict trailer before trusting results, then weave accepted findings into the audit and roadmaps. Keep `.subdaimon-output/` local and untracked. |

## Reverse Engineering

Open Rebellion's Ghidra workflow is project-native rather than a general skill:

- Read `agent_docs/ghidra-re.md` before reverse-engineering game behavior.
- Use the existing Open Rebellion Ghidra project, Jython scripts, decompilation
  corpus, and `ghidra/notes/` sources described there.
- Record addresses, call chains, field offsets, and confidence, then connect the
  finding to a Rust test or acceptance fixture.
- Do not use `bg3se-macos-ghidra`; that skill encodes Baldur's Gate 3-specific
  binaries, layouts, launch flows, and terminology.

## Supporting Skills

| Skill | Use when |
|-------|----------|
| `test-harness-auditor` | Auditing build, test, lint, browser, and debug gates before M5 CI restoration. |
| `cloudflare` | Implementing the password-protected Pages/Functions v1.0 deployment, secrets, headers, environments, and rollback. |
| `architecture-md-builder` | Updating the architectural map after an approved cross-crate or multiplayer boundary change. |
| `claude-md-manager` | Keeping `CLAUDE.md` concise, accurate, and progressively disclosed. |
| `agents-md-manager` | Maintaining the cross-agent `AGENTS.md` contract and validating its limits. |
| `design-audit` | Scoring accessibility, responsiveness, rendering quality, and browser performance without silently changing the UI. |
| `image-forge` / `sprite-forge` | Preparing owned replacement art or sprite assets; never repackage copyrighted source data. |
| `mycelium` | Reading or recording file-level context only when the task explicitly adopts the repository's git-notes workflow. |

## Guardrails

- Skills assist implementation and verification; they do not lower a feature's
  acceptance criteria or turn an inferred result into a pass.
- Keep passwords, API tokens, session secrets, and proprietary game assets out
  of prompts, reports, screenshots, and commits.
- Preserve unrelated work. Stage explicit paths, update the evidence ledger,
  and commit and push each verified feature before advancing.
