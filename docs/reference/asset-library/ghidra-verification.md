# Proving original entity-to-graphic mappings

The [entity catalog](entity-catalog.json) lists all 331 decoded base-game entities and every graphic role still unresolved. Its 262 class/miniature relations are derived from the native selector and validated over all 131 non-system records, but the DAT-to-runtime-class initialization edge remains open. The [DLL inventory](resource-inventory.json) proves resource existence and shape, not final display behavior.

## Evidence ladder

| Grade | Required proof |
|---|---|
| `implementation-only` | The port loads a resource for a named entity. This says nothing by itself about the original game. |
| `source-derived` | A native selector and a decoded DAT field produce a whole-family, resource-validated ID mapping, but one original initialization or state edge remains untraced. Record the missing edge. |
| `source-proven` | A saved original-executable function, table, or data structure joins the entity's DAT family and ID to the exact module/type/resource ID, with branch conditions for faction and visual state. Record the address and relevant operands, not just a plausible offset. |
| `runtime-corroborated` | The original executable loads or displays that exact resource for that entity and state, or a lossless original capture and resource pixels corroborate it. |
| `accepted` | The corresponding interface-audit cell passes source, interaction, audio, native, and packaged-browser gates. This is not a mapping grade. |

Promotion requires a resource-existence check against the owned installation, a cross-check over *every* member of the family, and explicit treatment of exceptions. A regular sequence of image IDs is a hypothesis until the original lookup proves the sequence and its bounds. The `source-derived` class relation still needs the original DAT-field-to-runtime-class write traced before promotion. A Ghidra trace cannot alone prove palette composition, timing, occlusion, or browser parity.

## Reproducible local route

1. Use the saved Ghidra project documented in [ghidra-re.md](../../../agent_docs/ghidra-re.md), not the incomplete text export. `REBEXE.EXE` has thousands of zero-byte exported-function placeholders.
2. Start from original UI constructors and loaders in the [reverse-engineering ledger](../../qa/2026-09-10-interface-parity-audit/reverse-engineering-ledger.md): `FUN_0044f670`, `FUN_00442d70`, and `FUN_0044c410` for object windows; `RE-ENC-01` for ENCYBMAP/EData; `RE-TAC-01` and `RE-TAC-02` for battle resources. Trace callers, data references, entity-family tests, ID arithmetic, `FindResource`/`LoadResource` and custom type-301/303 loaders.
3. Use `ghidra/scripts/DumpInterfaceFunctions.py` with `analyzeHeadless` `-readOnly -noanalysis -postScript`, then inspect the generated `/tmp/open-rebellion-interface-functions.txt`. Run one target batch at a time because the script uses a fixed temporary output path. For a missing target, query the saved project directly rather than interpreting an empty exported `.c` as absent behavior.
4. For every proposed relation, record `family`, `dat_id`, `resource_key`, `role`, `faction`, `state`, `native_function`, `table_or_operand`, `branch_predicate`, and `source_binary`. Verify the resource ID and type against a contributor-owned DLL or EData file and the [metadata inventory](resource-inventory.json). Keep extracted pixels and game binaries out of Git.
5. Generate a whole-family report that marks mapped, exception, and unresolved entities. Test a low, middle, and high DAT ID plus every non-linear exception in the original runtime. Compare to a native 640×480 lossless capture when available, then run the packaged-browser audit matrix before accepting the visible surface.

## Priority lookup families

| Family | DAT source and first question | Additional joins to prove |
|---|---|
| Capital ships and fighters | `CAPSHPSD` and `FIGHTSD`; do original mini-icon and status-view selectors use the current record-order mapping? | GOKRES hull/status, damage and weapon diagrams; EData; TACTICAL BMP/type-301 mesh/type-303 texture; orientation, fighter groups, faction, battle effect, sound. |
| Troops and special forces | `TROOPSD` and `SPECFCSD`; which GOKRES miniature does each class use? | Status/encyclopedia art, mission and ground-report variants, research unlock state. |
| Facilities | `PROFACSD`, `MANFACSD`, `DEFFACSD`; separate the shared icon from class-specific assets. | Construction, damaged/disabled, defense, production, and encyclopedia states. |
| Characters | `MJCHARSD` and `MNCHARSD`; confirm the current major/minor portrait arithmetic and each exception. | Mini-icons, injury/capture/Jedi variants, authored events, STRATEGY and EData art. |
| Research | `research_order` and `research_difficulty` live on buildable entity records. Determine whether the original research list loads entity art or separate UI resources. | Per-faction unlocked, available, in-progress, completed, and unavailable states. Do not invent a standalone research-entity table. |
| Systems and planets | `SYSTEMSD.picture_id` is data, not yet a proven resource ID. Trace its native consumer. | Galaxy marker, system window, EData, tactical planet/backdrop, battle and event variants. |
| Faction chrome, banners, effects | These are UI/event resource families, not DAT entities. Trace state-driven resource selection directly. | Exact normal/hover/pressed/disabled bitmap, frame sequence, sound cue, and render geometry. |

The [space-battle inventory](space-battle.md) and [standalone launcher plan](../../plans/2026-09-12-tooling-standalone-space-battle-launcher.md) turn confirmed tactical joins into reproducible functional and visual tests. The [interface audit](../../qa/2026-09-10-interface-parity-audit/README.md) remains the acceptance authority.
