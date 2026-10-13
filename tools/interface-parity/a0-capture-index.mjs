// Render the ignored A0 manifest as a tracked Markdown index.
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const ROOT = fileURLToPath(new URL("../..", import.meta.url)).replace(/\/$/, "");
const OUT = `${ROOT}/docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-13-tactical-a0-capture-index.md`;
const manifest = JSON.parse(readFileSync(`${ROOT}/.artifacts/interface-parity/a0/manifest.json`, "utf8"));
const ledger = JSON.parse(readFileSync(`${ROOT}/docs/qa/2026-09-10-interface-parity-audit/2026-10-06-surface-ledger.json`, "utf8"));

const surfaces = [];
(function walk(o) {
  if (Array.isArray(o)) return o.forEach(walk);
  if (o && typeof o === "object") {
    if (typeof o.id === "string" && /^TAC-0[1-8]$/.test(o.id) && Array.isArray(o.acceptance_cells)) surfaces.push(o);
    else Object.values(o).forEach(walk);
  }
})(ledger);
surfaces.sort((a, b) => a.id.localeCompare(b.id));

const byCell = new Map(manifest.captures.map((c) => [c.cell_id, c]));
const esc = (s) => String(s ?? "").replace(/\|/g, "\\|").replace(/\s+/g, " ").trim();

const total = surfaces.reduce((n, s) => n + s.acceptance_cells.length, 0);
const open = [];
let body = "";
for (const s of surfaces) {
  const have = s.acceptance_cells.filter((c) => byCell.has(c.id)).length;
  body += `\n## ${s.id}: ${s.title} (${have} of ${s.acceptance_cells.length})\n\n`;
  body += "| Cell | Requirement | Setup | What the frame shows | SHA-256 |\n|---|---|---|---|---|\n";
  for (const cell of s.acceptance_cells) {
    const c = byCell.get(cell.id);
    if (!c) {
      open.push(cell);
      body += `| ${cell.id} | ${esc(cell.requirement)} | — | **Open**: no A0 frame yet | — |\n`;
      continue;
    }
    const png = c.artifacts.find((a) => a.kind === "visual_png");
    const extra = c.artifacts.filter((a) => a.kind !== "visual_png").map((a) => a.kind.replace("_", " "));
    const shown = esc(c.state_setup.notes) || `${esc(c.input_trace.at(-1))} (last input step)`;
    const notes = shown +(extra.length ? ` (also ${extra.join(", ")})` : "");
    body += `| ${cell.id} | ${esc(c.requirement)} | ${c.state_setup.kind} \`${esc(c.state_setup.identifier)}\` | ${notes} | \`${png.sha256.slice(0, 12)}…\` |\n`;
  }
}

const captured = manifest.captures.length;
const head = `# Tactical A0 capture index

Every A0 frame of the original game held in the ignored manifest
(\`.artifacts/interface-parity/a0/manifest.json\`), one row per acceptance cell.
Each frame is \`.artifacts/interface-parity/a0/<cell>/original.png\`, a 640×480
guest-side capture of the unmodified \`REBEXE.EXE\` (README of
\`tools/original-capture\`, "A0 captures"). The setup names a save in
\`tools/original-capture/saves/INDEX.md\` or the journey that reached the state;
the full input trace stays in the manifest.

Coverage: ${captured} of ${total}. TAC-01 to TAC-07 are the 106-cell tactical
matrix; TAC-08, the ground assault (\`UIP-F-010\`), is an A0-only family outside
that denominator (\`A0_ONLY_FAMILY_COUNTS\` in
\`tools/interface-parity/tactical-matrix.mjs\`). A frame here is a reference,
not an acceptance: comparison against the port and the strict gate have not
run. Regenerate this file with \`node tools/interface-parity/a0-capture-index.mjs\`.

Open: ${open.map((c) => `${c.id} (${c.requirement})`).join(", ")}. All four need
a battle the Alliance owns against an Imperial Death Star; the side-swap save
edits crash the original, so they wait on a native Alliance game that reaches
one (\`docs/plans/2026-09-22-tooling-tactical-106-cell-acceptance.md\`, session 5).
`;

writeFileSync(OUT, head + body);
console.log(`${captured}/${total}, open: ${open.map((c) => c.id).join(" ")}`);
