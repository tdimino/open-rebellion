#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(scriptDir, "..");
const auditDir = path.join(
  root,
  "docs/qa/2026-09-10-interface-parity-audit",
);
const surfacePath = path.join(auditDir, "surface-ledger.json");
const reversePath = path.join(auditDir, "reverse-engineering-ledger.json");
const auditPath = path.join(auditDir, "audit-report.json");
const catalogPath = path.join(root, "tools/interface-parity/scenarios/gid.catalog.json");
const generatedDocuments = [
  path.join(auditDir, "README.md"),
  path.join(auditDir, "index.md"),
  path.join(root, "docs/plans/2026-09-11-feat-batched-interface-parity-plan.md"),
  path.join(root, "agent_docs/roadmap.md"),
];
const generatedStart = "<!-- interface-parity-status:start -->";
const generatedEnd = "<!-- interface-parity-status:end -->";

const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const fail = (message) => {
  throw new Error(message);
};
const assert = (condition, message) => {
  if (!condition) fail(message);
};

function generatedStatus(surfaceLedger, reverseLedger, auditReport, catalog) {
  const required = surfaceLedger.surfaces.filter((surface) => surface.required);
  const accepted = required.flatMap((surface) => surface.acceptance_cells)
    .filter((cell) => cell.status === "passed").length;
  const counts = Object.fromEntries(
    ["complete", "partial", "fail", "blocked"].map((status) => [
      status,
      required.filter((surface) => surface.status === status).length,
    ]),
  );
  const gid = surfaceLedger.surfaces.find((surface) => surface.id === catalog.family);
  assert(gid, `missing scenario family ${catalog.family}`);
  const matches = gid.required_states.every(
    (requirement, index) => catalog.scenarios[index]?.requirement === requirement,
  );
  assert(matches, "GID catalog requirements do not match canonical CMD-02 cells");
  assert(
    auditReport.summary.strictly_accepted_required_cells ===
      `${accepted}_of_${surfaceLedger.acceptance_summary.required_baseline_cells}`,
    "strictly accepted cell count disagrees with canonical surface statuses",
  );
  assert(
    reverseLedger.work_packages.length > 0,
    "reverse-engineering ledger has no packages",
  );

  const total = surfaceLedger.acceptance_summary.required_baseline_cells;
  const executions = catalog.scenarios.length * catalog.factions.length * catalog.viewports.length;
  return [
    generatedStart,
    `Required interface families: ${required.length}. Complete: ${counts.complete}. ` +
      `Partial: ${counts.partial}. Failing: ${counts.fail}. Blocked: ${counts.blocked}. ` +
      `Strictly accepted cells: ${accepted}/${total}.`,
    `The current CMD-02 GID catalog has ${catalog.scenarios.length} scenarios (29 baseline cells and nine additional native filter variants) ` +
      `and ${executions} faction/viewport executions. Its strict original-evidence ` +
      `and cross-browser gate remains ${gid.status === "complete" ? "complete" : "open"}.`,
    generatedEnd,
  ].join("\n");
}

function synchronizeDocuments(block, mode) {
  for (const file of generatedDocuments) {
    const current = fs.readFileSync(file, "utf8");
    const start = current.indexOf(generatedStart);
    const end = current.indexOf(generatedEnd);
    assert(start >= 0 && end > start, `missing generated block in ${path.relative(root, file)}`);
    assert(
      current.indexOf(generatedStart, start + 1) < 0 &&
        current.indexOf(generatedEnd, end + 1) < 0,
      `duplicate generated markers in ${path.relative(root, file)}`,
    );
    const next = `${current.slice(0, start)}${block}${current.slice(end + generatedEnd.length)}`;
    if (mode === "--write") {
      if (next !== current) fs.writeFileSync(file, next);
    } else {
      assert(next === current, `generated status is stale in ${path.relative(root, file)}`);
    }
  }
}

function packageIdsForSurface(reverseLedger, surfaceId) {
  return reverseLedger.work_packages
    .filter((workPackage) => workPackage.surface_ids.includes(surfaceId))
    .map((workPackage) => workPackage.id)
    .sort();
}

function nextCellNumber(existingCells, surfaceId) {
  const pattern = new RegExp(`^${surfaceId}-C(\\d{3})$`);
  return (
    existingCells.reduce((maximum, cell) => {
      const match = pattern.exec(cell.id ?? "");
      return match ? Math.max(maximum, Number(match[1])) : maximum;
    }, 0) + 1
  );
}

function instantiateCells(surface) {
  const existingCells = surface.acceptance_cells ?? [];
  const existingByRequirement = new Map(
    existingCells.map((cell) => [cell.requirement, cell]),
  );
  let nextNumber = nextCellNumber(existingCells, surface.id);

  return surface.required_states.map((requirement) => {
    const existing = existingByRequirement.get(requirement);
    if (existing) return existing;

    const id = `${surface.id}-C${String(nextNumber).padStart(3, "0")}`;
    nextNumber += 1;
    return {
      id,
      requirement,
      status: surface.required ? "pending" : "excluded",
      counts_toward_parity: surface.required,
    };
  });
}

function hydrate(surfaceLedger, reverseLedger) {
  surfaceLedger.cell_id_format = "<surface-id>-C<three-digit-stable-ordinal>";
  surfaceLedger.execution_id_format =
    "<cell-id>::<faction>::<content>::<interaction>::<window>::<intelligence>::<simulation>::<runtime>::<browser>::<viewport>::<dpr>";
  surfaceLedger.cell_instantiation_gate =
    "Every listed state has a stable baseline cell. Before implementation begins for a surface, split compound requirements into explicit child cells and instantiate every applicable contextual and execution dimension. A surface passes only when every resulting required execution passes.";
  surfaceLedger.cell_dimensions = [
    "faction",
    "content_population",
    "interaction_state",
    "window_state",
    "intelligence_state",
    "simulation_state",
    "runtime",
    "browser",
    "viewport",
    "device_pixel_ratio",
  ];
  surfaceLedger.execution_matrix = {
    original_reference: {
      runtime: "owned-original-en-US",
      viewport: "640x480",
      dpr: [1],
      purpose: "A0 visual, input, audio, and transition authority",
    },
    native: {
      runtimes: ["macOS-native"],
      viewport: "640x480-logical",
      dpr: [1, 2],
    },
    browser: {
      runtime: "wasm",
      browsers: ["Chrome", "Firefox", "Safari"],
      viewports_ref: "audit-report.json#/viewports",
    },
  };

  surfaceLedger.surfaces = surfaceLedger.surfaces.map((surface) => ({
    ...surface,
    reverse_engineering_package_ids: packageIdsForSurface(
      reverseLedger,
      surface.id,
    ),
    acceptance_cells: instantiateCells(surface),
  }));

  const cells = surfaceLedger.surfaces.flatMap(
    (surface) => surface.acceptance_cells,
  );
  surfaceLedger.acceptance_summary = {
    surface_families: surfaceLedger.surfaces.length,
    required_surface_families: surfaceLedger.surfaces.filter(
      (surface) => surface.required,
    ).length,
    baseline_cells: cells.length,
    required_baseline_cells: cells.filter((cell) => cell.counts_toward_parity)
      .length,
    excluded_baseline_cells: cells.filter(
      (cell) => !cell.counts_toward_parity,
    ).length,
    required_surfaces_with_retrieval_packages: surfaceLedger.surfaces.filter(
      (surface) =>
        surface.required && surface.reverse_engineering_package_ids.length > 0,
    ).length,
  };
}

function validate(surfaceLedger, reverseLedger, auditReport) {
  const surfaceIds = new Set();
  const cellIds = new Set();
  const workPackageIds = new Set();
  const validStatuses = new Set(reverseLedger.status_vocabulary);

  for (const workPackage of reverseLedger.work_packages) {
    assert(!workPackageIds.has(workPackage.id), `duplicate package ${workPackage.id}`);
    workPackageIds.add(workPackage.id);
    assert(validStatuses.has(workPackage.status), `invalid status on ${workPackage.id}`);
    assert(workPackage.retrieval?.sources?.length > 0, `missing sources on ${workPackage.id}`);
    assert(workPackage.retrieval?.method, `missing method on ${workPackage.id}`);
    assert(workPackage.retrieval?.next_proof, `missing next proof on ${workPackage.id}`);
  }

  for (const surface of surfaceLedger.surfaces) {
    assert(!surfaceIds.has(surface.id), `duplicate surface ${surface.id}`);
    surfaceIds.add(surface.id);
    assert(
      surface.acceptance_cells.length === surface.required_states.length,
      `${surface.id} does not have one baseline cell per required state`,
    );
    if (surface.required) {
      assert(
        surface.reverse_engineering_package_ids.length > 0,
        `${surface.id} has no retrieval package`,
      );
    }
    for (const packageId of surface.reverse_engineering_package_ids) {
      assert(workPackageIds.has(packageId), `${surface.id} links unknown ${packageId}`);
    }

    const requirements = new Set();
    for (const cell of surface.acceptance_cells) {
      assert(
        new RegExp(`^${surface.id}-C\\d{3}$`).test(cell.id),
        `invalid cell ID ${cell.id}`,
      );
      assert(!cellIds.has(cell.id), `duplicate cell ${cell.id}`);
      cellIds.add(cell.id);
      assert(!requirements.has(cell.requirement), `duplicate requirement on ${surface.id}`);
      requirements.add(cell.requirement);
      assert(
        cell.counts_toward_parity === surface.required,
        `parity scope mismatch on ${cell.id}`,
      );
    }
    assert(
      surface.required_states.every((state) => requirements.has(state)),
      `${surface.id} has an uninstantiated state`,
    );
  }

  for (const workPackage of reverseLedger.work_packages) {
    for (const surfaceId of workPackage.surface_ids) {
      assert(surfaceIds.has(surfaceId), `${workPackage.id} links unknown ${surfaceId}`);
    }
  }

  const cells = surfaceLedger.surfaces.flatMap(
    (surface) => surface.acceptance_cells,
  );
  const requiredSurfaces = surfaceLedger.surfaces.filter(
    (surface) => surface.required,
  );
  const requiredCells = cells.filter((cell) => cell.counts_toward_parity);
  const spaceBattleCells = surfaceLedger.surfaces
    .filter((surface) => /^TAC-0[1-7]$/.test(surface.id))
    .flatMap((surface) => surface.acceptance_cells)
    .filter((cell) => cell.counts_toward_parity);
  const summary = surfaceLedger.acceptance_summary;
  assert(summary.surface_families === surfaceLedger.surfaces.length, "bad surface total");
  assert(
    summary.required_surface_families === requiredSurfaces.length,
    "bad required surface total",
  );
  assert(summary.baseline_cells === cells.length, "bad baseline cell total");
  assert(
    summary.required_baseline_cells === requiredCells.length,
    "bad required cell total",
  );
  assert(
    summary.required_surfaces_with_retrieval_packages === requiredSurfaces.length,
    "not every required surface has a retrieval package",
  );
  const reverseSummary = reverseLedger.coverage_summary;
  assert(
    reverseSummary.work_packages === reverseLedger.work_packages.length,
    "bad reverse-engineering package total",
  );
  assert(
    reverseSummary.required_surface_families === requiredSurfaces.length,
    "bad reverse-engineering surface total",
  );
  assert(
    reverseSummary.required_surface_families_with_packages ===
      requiredSurfaces.length,
    "bad reverse-engineering mapping total",
  );
  assert(
    reverseSummary.required_baseline_cells === requiredCells.length,
    "bad reverse-engineering cell total",
  );
  assert(
    reverseSummary.space_battle_baseline_cells === spaceBattleCells.length,
    "bad space-battle cell total",
  );
  assert(
    auditReport.scope.surface_families === surfaceLedger.surfaces.length,
    "audit report surface total disagrees with surface ledger",
  );
  assert(
    auditReport.summary.required_baseline_cells.startsWith(
      `${requiredCells.length}_`,
    ),
    "audit report required-cell total disagrees with surface ledger",
  );
  assert(
    auditReport.summary.retrieval_package_coverage ===
      `${requiredSurfaces.length}_of_${requiredSurfaces.length}_required_surface_families`,
    "audit report retrieval coverage disagrees with ledgers",
  );
  assert(
    auditReport.summary.space_battle_scope.startsWith(
      `${spaceBattleCells.length}_`,
    ),
    "audit report space-battle total disagrees with surface ledger",
  );
  const tacticalTranche = auditReport.tranches.find(
    (tranche) => tranche.id === "UIP-T05",
  );
  assert(tacticalTranche, "missing UIP-T05 tactical tranche");
  assert(
    ["TAC-01", "TAC-02", "TAC-03", "TAC-04", "TAC-05", "TAC-06", "TAC-07", "TAC-08"].every(
      (surfaceId) => tacticalTranche.surface_ids.includes(surfaceId),
    ),
    "UIP-T05 does not cover every tactical and ground-assault family",
  );

  return {
    surface_families: surfaceLedger.surfaces.length,
    required_surface_families: requiredSurfaces.length,
    baseline_cells: cells.length,
    required_baseline_cells: requiredCells.length,
    reverse_engineering_packages: reverseLedger.work_packages.length,
    space_battle_baseline_cells: spaceBattleCells.length,
    required_surfaces_with_retrieval_packages:
      summary.required_surfaces_with_retrieval_packages,
  };
}

const mode = process.argv[2] ?? "--check";
const surfaceLedger = readJson(surfacePath);
const reverseLedger = readJson(reversePath);
const auditReport = readJson(auditPath);
const catalog = readJson(catalogPath);

if (mode === "--write") {
  hydrate(surfaceLedger, reverseLedger);
  fs.writeFileSync(surfacePath, `${JSON.stringify(surfaceLedger, null, 2)}\n`);
} else if (mode !== "--check") {
  fail("usage: validate-interface-parity-ledgers.mjs [--check|--write]");
}

const result = validate(surfaceLedger, reverseLedger, auditReport);
assert(catalog.scenarios.length === 38, "GID catalog must cover 29 baseline cells and nine native filter variants");
synchronizeDocuments(generatedStatus(surfaceLedger, reverseLedger, auditReport, catalog), mode);
process.stdout.write(`${JSON.stringify({ status: "pass", ...result }, null, 2)}\n`);
