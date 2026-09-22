# Interface parity evidence

This directory contains the durable reports and review artifacts for each
verified interface checkpoint. The parent [audit index](../README.md) remains
the source of truth for scope, acceptance, and current work.

## Current tactical sequence

| Checkpoint | Report | Artifact bundle |
|---|---|---|
| P57A | [Three-LOD family](2026-09-13-tactical-3d-lod-family.md) | [`p57-tactical-3d-lod-family/`](p57-tactical-3d-lod-family/) |
| P57B1 | [Live LOD journey](2026-09-13-tactical-live-lod-journey.md) | [`p57b1-tactical-live-lod/`](p57b1-tactical-live-lod/) |
| P57B2A | [Camera contract](2026-09-13-tactical-camera-contract.md) | [`p57b2-tactical-camera/`](p57b2-tactical-camera/) |
| P57B2B1 | [Target control](2026-09-13-tactical-target-control.md) | [`p57b2b1-tactical-target/`](p57b2b1-tactical-target/) |
| P57B2B2 | [Battle layout](2026-09-13-tactical-battle-layout.md) | [`p57b2b2-tactical-layout/`](p57b2b2-tactical-layout/) |
| P57B2B3 | [Participant placement](2026-09-13-tactical-participant-placement.md) | [`p57b2b3-tactical-participants/`](p57b2b3-tactical-participants/) |
| P57B2C1 | [Authored transform and system palette](2026-09-13-tactical-transform-palette.md) | [`p57b2c1-tactical-transform-palette/`](p57b2c1-tactical-transform-palette/) |
| P57B2C2A | [Retained-mode light rig](2026-09-14-tactical-light-rig.md) | [`p57b2c2a-tactical-light-rig/`](p57b2c2a-tactical-light-rig/) |
| P57B2C2B | [Retained-mode render state](2026-09-14-tactical-render-state.md) | [`p57b2c2b-tactical-render-state/`](p57b2c2b-tactical-render-state/) |
| P58A | [Tactical resource identity join](2026-09-14-tactical-resource-join.md) | [`p58a-tactical-resource-join/`](p58a-tactical-resource-join/) |
| P58B | [Production tactical capital ships](2026-09-14-tactical-production-participants.md) | [`p58b-tactical-production-participants/`](p58b-tactical-production-participants/) |
| P58C | [Tactical fighters and projected selection](2026-09-14-tactical-fighters-selection.md) | [`p58c-tactical-fighters-selection/`](p58c-tactical-fighters-selection/) |
| P58D | [Production fighter-detail journey](2026-09-14-tactical-fighter-detail-journey.md) | [`p58d-tactical-fighter-detail-journey/`](p58d-tactical-fighter-detail-journey/) |
| P58E | [Task-force and fighter-group presentation](2026-09-14-tactical-group-presentation.md) | [`p58e-tactical-group-presentation/`](p58e-tactical-group-presentation/) |
| P58F1 | [System-selected tactical planets](2026-09-14-tactical-planets.md) | [`p58f1-tactical-planets/`](p58f1-tactical-planets/) |
| P58F2 | [Tactical impact effects](2026-09-14-tactical-impact-effects.md) | [`p58f2-tactical-impact-effects/`](p58f2-tactical-impact-effects/) |
| P58F3 | [Tactical projectiles and fields](2026-09-14-tactical-projectile-fields.md) | [`p58f3-tactical-projectile-fields/`](p58f3-tactical-projectile-fields/) |
| P58F4 | [Selected-capital damage presentation](2026-09-15-tactical-selected-damage.md) | [`p58f4-tactical-selected-damage/`](p58f4-tactical-selected-damage/) |
| P58F5 | [Subsystem and field-command presentation](2026-09-15-tactical-subsystem-field-commands.md) | [`p58f5-tactical-subsystem-field-commands/`](p58f5-tactical-subsystem-field-commands/) |
| P58F6 | [Live subsystem damage](2026-09-15-tactical-live-subsystem-damage.md) | [`p58f6-tactical-live-subsystem-damage/`](p58f6-tactical-live-subsystem-damage/) |
| P58F7 | [Subsystem repair and mobility](2026-09-15-tactical-subsystem-repair-mobility.md) | [`p58f7-tactical-subsystem-repair-mobility/`](p58f7-tactical-subsystem-repair-mobility/) |
| P58F8 | [Maneuver and movement](2026-09-16-tactical-maneuver-movement.md) | [`p58f8-tactical-maneuver-movement/`](p58f8-tactical-maneuver-movement/) |
| P58F9 | [Tactical command assignment](2026-09-17-tactical-command-assignment.md) | [`p58f9-tactical-command-assignment/`](p58f9-tactical-command-assignment/) |
| P58F10 | [Tactical order execution](2026-09-17-tactical-order-execution.md) | [`p58f10-tactical-order-execution/`](p58f10-tactical-order-execution/) |
| P58F11 | [Tactical command progression](2026-09-18-tactical-command-progression.md) | [`p58f11-tactical-command-progression/`](p58f11-tactical-command-progression/) |
| P58F12 | [Tactical attack target acquisition](2026-09-18-tactical-attack-targeting.md) | [`p58f12-tactical-attack-targeting/`](p58f12-tactical-attack-targeting/) |
| P58F13 | [Tactical attack target lifecycle](2026-09-19-tactical-attack-target-lifecycle.md) | [`p58f13-tactical-attack-target-lifecycle/`](p58f13-tactical-attack-target-lifecycle/) |
| P58-B06 | [Tactical completion checkpoint](2026-09-22-tactical-completion-bundle.md) | [`p58-b06-tactical-completion/`](p58-b06-tactical-completion/) |
| P58-B07 | [Tactical 106-cell matrix contract](2026-09-22-tactical-106-matrix-contract.md) | [`p58-b07-tactical-matrix-contract/`](p58-b07-tactical-matrix-contract/) |

## Current strategic correction

| Checkpoint | Report | Artifact bundle |
|---|---|---|
| PR #11 | [Cockpit routing correction](2026-09-14-cockpit-routing-correction.md) | [`pr11-cockpit-routing/`](pr11-cockpit-routing/) |

Earlier strategic, GID, shell, control, staging, and render reports are indexed
in the parent [audit overview](../index.md). Every new artifact directory must
include its own `README.md` inventory and be linked from this index and the
parent audit indexes.
