# Open Rebellion plans

## Frontmatter convention

Every plan opens with YAML frontmatter carrying at least these keys:

| Key | Meaning |
|-----|---------|
| `title` | Plan name |
| `status` | `draft`, `planned`, `active`, `partial`, `superseded`, `completed`, or `archived` |
| `progress` | `"<N>% — <one-line state>"`, N being the share of the plan's committed phases that have landed; a completed plan reads `"100% — <what landed, commit hashes>"`. A beaded-out plan reads `"<closed> of <total> beads — <state>"`, from its epic on `tasks/INDEX.md` |
| `epic` | Once the plan is beaded out: its epic's ID (`agent_docs/task-graph.md`, "Plans and epics"). The epic's closed beads are its progress |
| `superseded_by` | Only for `superseded` plans: the plan or audit that replaces it |
| `created`, `updated` | ISO dates |

A `partial` plan has landed some phases and carries the rest in its progress
line. Completed plans move to `completed/` and keep their `progress` line.
An `archived` plan no longer drives work: its targets were removed, its
direction was abandoned, or its remaining items moved to another plan, a
roadmap milestone, an audit finding or a br node. It moves to `archive/`,
keeps its `progress` line, and adds `archived` (date) and `archive_reason`.
When its open items moved to one plan, `merged_into` names that plan.

## Current plans

- The [Batched Interface Parity Execution Plan](2026-09-11-feat-batched-interface-parity-plan.md)
  is the umbrella for original interface parity: the UIP-B01 to B07 bundle
  queue, the cadence, and the 627-cell denominator. Its harness tooling is
  documented in [`tools/interface-parity/README.md`](../../tools/interface-parity/README.md).
- The [Tactical 106-Cell Acceptance Train](2026-09-22-tooling-tactical-106-cell-acceptance.md)
  is the space-battle completion plan and evidence record (UIP-B06). A0
  references from the original cover all 106 cells plus the eight
  ground-assault cells
  ([capture index](../qa/2026-09-10-interface-parity-audit/evidence/2026-10-13-tactical-a0-capture-index.md)).
  The C6 comparison passes 11 of 106; the fixes are br nodes under
  `or-ep-tac-x15` ([task graph](../../agent_docs/task-graph.md)). Browser
  evidence alone cannot accept a cell that lacks an original baseline. The
  launcher and 3D asset plans it absorbed are archived; their references live
  in [`docs/reference/space-battle-launcher/`](../reference/space-battle-launcher/README.md)
  and the [asset library](../reference/asset-library/README.md).
- The [modality and focus](2026-10-10-feat-strategic-window-modality-and-focus.md)
  and [windowing toolkit](2026-10-10-strategic-windowing-toolkit-implementation.md)
  plans carry the strategic window work.
- The [Faithful HD Pipeline](2026-09-10-faithful-hd-pipeline/) is parked
  until original parity closes; modern replacement models stay in a separate
  experimental profile.

See the [plans index](INDEX.md) for active, planned, and completed work. The
[interface parity audit](../qa/2026-09-10-interface-parity-audit/) remains the
strict acceptance authority.

The
[Will Forster Encyclopedia Handoff Adaptation](completed/2026-10-02-feat-will-forster-encyclopedia-handoff-adaptation.md)
is complete (W0 through W7). It credits and selectively adapts the strongest
ideas from Will's PR #16 while preserving the current source, binding,
packed-transport, and visual-acceptance authorities.

After original space-battle parity is implemented, a separate planning pass
will define an optional X-Wing Alliance-style skirmish setup surface for ship
distribution, planet, and space-background selection. That feature must reuse
the production battle scene and remain outside the 106-cell original-interface
parity denominator. Its design plan is reserved for a Fable and `/ce:plan`
workflow when those project tools are available.
