import path from "node:path";

export const TACTICAL_FAMILY_COUNTS = Object.freeze({
  "TAC-01": 13,
  "TAC-02": 13,
  "TAC-03": 20,
  "TAC-04": 14,
  "TAC-05": 23,
  "TAC-06": 9,
  "TAC-07": 14,
});

export const TACTICAL_CELL_TOTAL = 106;
export const EXPECTED_ORIGINAL_EXE_SHA256 =
  "b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab";

const fail = (message) => {
  throw new Error(message);
};

const assert = (condition, message) => {
  if (!condition) fail(message);
};

export function collectTacticalCells(surfaceLedger) {
  assert(Array.isArray(surfaceLedger?.surfaces), "surface ledger has no surfaces");
  const seenIds = new Set();
  const cells = [];

  for (const [family, expectedCount] of Object.entries(TACTICAL_FAMILY_COUNTS)) {
    const matches = surfaceLedger.surfaces.filter((surface) => surface.id === family);
    assert(matches.length === 1, `expected exactly one ${family} surface`);
    const surface = matches[0];
    assert(surface.required === true, `${family} must remain required`);
    assert(
      Array.isArray(surface.acceptance_cells),
      `${family} has no acceptance_cells`,
    );
    assert(
      surface.acceptance_cells.length === expectedCount,
      `${family} denominator drift: expected ${expectedCount}, got ${surface.acceptance_cells.length}`,
    );

    for (const cell of surface.acceptance_cells) {
      assert(
        new RegExp(`^${family}-C\\d{3}$`).test(cell.id),
        `invalid tactical cell ID ${cell.id}`,
      );
      assert(!seenIds.has(cell.id), `duplicate tactical cell ${cell.id}`);
      assert(cell.counts_toward_parity === true, `${cell.id} must count toward parity`);
      assert(
        typeof cell.requirement === "string" && cell.requirement.length > 0,
        `${cell.id} has no requirement`,
      );
      seenIds.add(cell.id);
      cells.push({
        id: cell.id,
        family,
        requirement: cell.requirement,
        ledger_status: cell.status,
      });
    }
  }

  assert(
    cells.length === TACTICAL_CELL_TOTAL,
    `tactical denominator drift: expected ${TACTICAL_CELL_TOTAL}, got ${cells.length}`,
  );
  return cells;
}

export function catalogCoverage(catalog, cells, { strict = false } = {}) {
  assert(Array.isArray(catalog?.scenarios), "tactical catalog has no scenarios");
  const known = new Set(cells.map(({ id }) => id));
  const mapped = new Map();

  for (const scenario of catalog.scenarios) {
    assert(
      typeof scenario.slug === "string" && scenario.slug.length > 0,
      "tactical catalog contains a scenario without a slug",
    );
    const auditCells = scenario.audit_cells ?? [];
    assert(Array.isArray(auditCells), `${scenario.slug} audit_cells must be an array`);
    for (const cellId of auditCells) {
      assert(known.has(cellId), `${scenario.slug} maps unknown tactical cell ${cellId}`);
      assert(
        !mapped.has(cellId),
        `${cellId} is mapped by both ${mapped.get(cellId)} and ${scenario.slug}`,
      );
      mapped.set(cellId, scenario.slug);
    }
  }

  const missing = cells.map(({ id }) => id).filter((id) => !mapped.has(id));
  if (strict && missing.length > 0) {
    fail(`tactical catalog is missing ${missing.length} cells: ${missing.join(", ")}`);
  }
  return { mapped, missing };
}

function validateArtifactPath(artifactPath, cellId) {
  assert(typeof artifactPath === "string", `${cellId} has an artifact without a path`);
  assert(!path.isAbsolute(artifactPath), `${cellId} artifact path must be relative`);
  assert(
    !artifactPath.split(/[\\/]/).includes(".."),
    `${cellId} artifact path may not traverse outside the A0 store`,
  );
  assert(
    artifactPath.startsWith(`.artifacts/interface-parity/a0/${cellId}/`),
    `${cellId} artifact path must stay in its ignored A0 directory`,
  );
}

export function a0Coverage(manifest, cells, { strict = false } = {}) {
  if (!manifest) {
    if (strict) fail("strict tactical validation requires an A0 manifest");
    return { captures: new Map(), missing: cells.map(({ id }) => id) };
  }

  assert(manifest.source?.unmodified === true, "A0 source must be unmodified");
  assert(manifest.source?.language === "en-US", "A0 source must be English en-US");
  assert(
    manifest.source?.executable_sha256 === EXPECTED_ORIGINAL_EXE_SHA256,
    "A0 executable hash does not match the audited original",
  );
  assert(manifest.source?.capture?.width === 640, "A0 capture width must be 640");
  assert(manifest.source?.capture?.height === 480, "A0 capture height must be 480");
  assert(manifest.source?.capture?.device_scale_factor === 1, "A0 capture DPR must be 1");
  assert(manifest.source?.capture?.lossless === true, "A0 capture must be lossless");
  assert(
    manifest.source?.capture?.client_area_only === true,
    "A0 capture must contain only the game client area",
  );
  assert(
    manifest.source?.capture?.host_scaling === false,
    "A0 capture may not use host scaling",
  );

  const known = new Map(cells.map((cell) => [cell.id, cell]));
  const captures = new Map();
  for (const capture of manifest.captures ?? []) {
    const cell = known.get(capture.cell_id);
    assert(cell, `A0 manifest maps unknown tactical cell ${capture.cell_id}`);
    assert(!captures.has(capture.cell_id), `duplicate A0 capture for ${capture.cell_id}`);
    assert(
      capture.requirement === cell.requirement,
      `${capture.cell_id} requirement does not match the surface ledger`,
    );
    assert(
      Array.isArray(capture.input_trace) && capture.input_trace.length > 0,
      `${capture.cell_id} has no ordered input trace`,
    );
    assert(
      capture.input_trace.every((step) => typeof step === "string" && step.length > 0),
      `${capture.cell_id} has an invalid input trace step`,
    );
    assert(
      typeof capture.captured_at === "string" &&
        Number.isFinite(Date.parse(capture.captured_at)),
      `${capture.cell_id} has an invalid capture timestamp`,
    );
    assert(
      Array.isArray(capture.artifacts) && capture.artifacts.length > 0,
      `${capture.cell_id} has no artifacts`,
    );
    assert(
      capture.artifacts.some((artifact) => artifact.kind === "visual_png"),
      `${capture.cell_id} has no lossless visual PNG`,
    );
    for (const artifact of capture.artifacts) {
      validateArtifactPath(artifact.path, capture.cell_id);
      assert(
        typeof artifact.sha256 === "string" && /^[a-f0-9]{64}$/.test(artifact.sha256),
        `${capture.cell_id} has an invalid artifact hash`,
      );
      if (artifact.kind === "visual_png") {
        assert(artifact.mime_type === "image/png", `${capture.cell_id} visual must be PNG`);
        assert(artifact.width === 640, `${capture.cell_id} visual width must be 640`);
        assert(artifact.height === 480, `${capture.cell_id} visual height must be 480`);
        assert(artifact.path.endsWith(".png"), `${capture.cell_id} visual path must end in .png`);
      }
    }
    captures.set(capture.cell_id, capture);
  }

  const missing = cells.map(({ id }) => id).filter((id) => !captures.has(id));
  if (strict && missing.length > 0) {
    fail(`A0 manifest is missing ${missing.length} cells: ${missing.join(", ")}`);
  }
  return { captures, missing };
}

export function buildTacticalMatrix({
  surfaceLedger,
  catalog,
  a0Manifest = null,
  strict = false,
  requireAccepted = false,
}) {
  const cells = collectTacticalCells(surfaceLedger);
  const strictEvidence = strict || requireAccepted;
  const catalogResult = catalogCoverage(catalog, cells, { strict: strictEvidence });
  const a0Result = a0Coverage(a0Manifest, cells, { strict: strictEvidence });
  const rows = cells.map((cell) => ({
    ...cell,
    scenario_slug: catalogResult.mapped.get(cell.id) ?? null,
    a0_status: a0Result.captures.has(cell.id) ? "available" : "reference-needed",
    accepted: cell.ledger_status === "passed",
  }));
  const accepted = rows.filter((row) => row.accepted).length;
  if (requireAccepted && accepted !== TACTICAL_CELL_TOTAL) {
    fail(`strict parity acceptance is ${accepted}/${TACTICAL_CELL_TOTAL}`);
  }

  return {
    schema_version: 1,
    family: "TAC-01..TAC-07",
    expected_distribution: TACTICAL_FAMILY_COUNTS,
    summary: {
      denominator: TACTICAL_CELL_TOTAL,
      matrix_coverage: catalogResult.mapped.size,
      a0_coverage: a0Result.captures.size,
      parity_acceptance: accepted,
      catalog_cells_missing: catalogResult.missing.length,
      a0_cells_missing: a0Result.missing.length,
      status: accepted === TACTICAL_CELL_TOTAL &&
          catalogResult.mapped.size === TACTICAL_CELL_TOTAL &&
          a0Result.captures.size === TACTICAL_CELL_TOTAL
        ? "accepted"
        : catalogResult.mapped.size === TACTICAL_CELL_TOTAL &&
            a0Result.captures.size === TACTICAL_CELL_TOTAL
          ? "runnable-not-accepted"
          : "incomplete",
    },
    rows,
  };
}

export function forbiddenTrackedA0Paths(trackedPaths) {
  return trackedPaths.filter((file) =>
    file.startsWith(".artifacts/interface-parity/a0/") ||
    file === "tools/interface-parity/tactical-a0-manifest.local.json" ||
    file.startsWith("tools/interface-parity/baselines/original/tactical/"));
}
