# Contributing to Open Rebellion

Open Rebellion is restoring both the behavior and the original bitmap-driven
interface of *Star Wars: Rebellion* (1998). Contributions should close one
bounded, auditable unit at a time.

## Choose a task

| Work | Start here |
|------|------------|
| Gameplay, simulation, persistence, browser, or multiplayer | [Full-functionality audit](docs/qa/2026-09-08-full-functionality-audit/) and its `P00` through `P40` acceptance passes |
| Original interface implementation | [Interface parity guide](docs/qa/2026-09-10-interface-parity-audit/) and a required family or cell in `surface-ledger.json` |
| Ghidra or resource recovery | An open `RE-*` package in the [reverse-engineering ledger](docs/qa/2026-09-10-interface-parity-audit/reverse-engineering-ledger.md) |
| Reference screenshot research | A missing state in the [screenshot ledger](docs/qa/2026-09-10-interface-parity-audit/screenshot-ledger.md) |
| Milestone planning | [Roadmap](agent_docs/roadmap.md) |
| Architecture and subsystem context | [Agent documentation index](agent_docs/INDEX.md) and [documentation index](docs/INDEX.md) |

Before coding, identify the relevant `P##`, `RE-*`, surface-family, or cell ID.
Keep the pull request focused on that unit and cite the ID in its description.

## Development workflow

1. Fork the repository and create a focused branch.
2. Read the applicable audit entry, reference evidence, and subsystem docs.
3. Add or update tests with the implementation.
4. Run the narrow checks first, followed by the relevant workspace and browser
   gates.
5. Update the corresponding Markdown and JSON records with verified evidence.
6. Open a pull request describing the behavior, tests, visual evidence, and any
   remaining limitations.

Do not include broad formatting changes or unrelated cleanup in a feature pull
request. Do not mark a partial, inferred, or static-only result complete.

## Core checks

```sh
cargo check --workspace
cargo test --workspace --all-targets
go test -count=1 ./tools/stage-ui-assets
go vet ./tools/stage-ui-assets
python3 -m unittest scripts/test_build_runtime_pack.py
node scripts/validate-interface-parity-ledgers.mjs --check
./scripts/build-wasm.sh
```

Formatting and strict Clippy have known repository-wide audit findings. Run
scoped checks on touched code and avoid reformatting unrelated files. Platform
notes and the macOS compiler-path workaround are documented in [AGENTS.md](AGENTS.md).

## Interface acceptance

The original game is the authority. Do not invent visible panels, controls,
icons, layouts, or interaction behavior in parity mode. A required interface
cell passes only with:

- authoritative original evidence and exact resource identity;
- matching composition, geometry, hotspots, states, and audio where applicable;
- native and packaged-browser captures;
- clean console, network, and missing-asset diagnostics;
- updated audit Markdown and JSON in the same focused change.

The space-battle interface is explicitly covered by `TAC-01` through `TAC-07`
in the interface surface ledger.

## Original game assets

Contributors need a legally owned copy of the original game for complete local
testing. Do not commit original DLLs, DAT files, audio, video, extracted runtime
packs, or other copyrighted game assets. Use the repository tooling to stage
them locally. See [the asset guide](agent_docs/assets.md) and
[`stage-ui-assets`](tools/stage-ui-assets/README.md).

## Pull-request checklist

- [ ] The PR names its audit or ledger ID.
- [ ] The change is one reviewable unit.
- [ ] Tests and builds relevant to the change pass.
- [ ] UI changes include native and browser evidence where required.
- [ ] Audit, roadmap, and evidence records reflect only verified results.
- [ ] No credentials or copyrighted game assets are included.

By contributing code, you agree that it is provided under the repository's
[MIT License](LICENSE).
