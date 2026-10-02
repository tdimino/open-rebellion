#!/usr/bin/env node

import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import zlib from "node:zlib";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(here, "../..");
const defaultScenario = path.join(here, "scenarios/encyclopedia-native-mods.json");
const defaultSourceRoot = path.resolve(
  repositoryRoot,
  "../agent-24-encyclopedia-base-parity-session-14-open-rebellion/.artifacts/e21/owned-data/base",
);
const defaultWorkspace = path.join(repositoryRoot, ".artifacts/encyclopedia/E26/native-acceptance");
const ownershipFile = ".e26-owned.json";
const operationKinds = new Set([
  "reset",
  "install",
  "replace_asset_atomically",
  "replace_overlay_atomically",
  "replace_overlay",
  "set_enabled",
]);

function parseArguments(argv) {
  const parsed = {
    action: null,
    scenario: defaultScenario,
    sourceRoot: defaultSourceRoot,
    workspace: defaultWorkspace,
    output: null,
  };
  const actions = new Map([
    ["--self-test", "self-test"],
    ["--verify-inputs", "verify-inputs"],
    ["--prepare", "prepare"],
    ["--apply-step", "apply-step"],
    ["--cleanup", "cleanup"],
    ["--print-runbook", "print-runbook"],
  ]);
  const paths = new Map([
    ["--scenario", "scenario"],
    ["--source-root", "sourceRoot"],
    ["--workspace", "workspace"],
    ["--output", "output"],
  ]);
  const values = new Map([
    ["--journey", "journey"],
    ["--step", "step"],
  ]);

  for (let index = 0; index < argv.length; index += 1) {
    const argument = argv[index];
    if (actions.has(argument)) {
      if (parsed.action) throw new Error(`choose exactly one action; saw ${parsed.action} and ${argument}`);
      parsed.action = actions.get(argument);
      continue;
    }
    const field = paths.get(argument);
    const valueField = values.get(argument);
    if (!field && !valueField) throw new Error(`unknown argument ${argument}`);
    const value = argv[index + 1];
    if (!value) throw new Error(`${argument} requires a value`);
    if (field) parsed[field] = path.resolve(value);
    else parsed[valueField] = value;
    index += 1;
  }
  if (!parsed.action) throw new Error("choose one of --self-test, --verify-inputs, --prepare, --apply-step, --cleanup, or --print-runbook");
  if (parsed.action === "apply-step" && (!parsed.journey || !parsed.step)) {
    throw new Error("--apply-step requires --journey and --step");
  }
  return parsed;
}

function readJson(file) {
  return JSON.parse(fs.readFileSync(file, "utf8"));
}

function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

function stableJson(value) {
  return `${JSON.stringify(value, null, 2)}\n`;
}

function isInside(parent, child) {
  const relative = path.relative(path.resolve(parent), path.resolve(child));
  return relative !== "" && !relative.startsWith(`..${path.sep}`) && relative !== ".." && !path.isAbsolute(relative);
}

function canonicalRoot(root, label, mustExist) {
  const absolute = path.resolve(root);
  if (fs.existsSync(absolute)) {
    const stat = fs.lstatSync(absolute);
    assert.ok(!stat.isSymbolicLink(), `${label} root must not be a symbolic link: ${absolute}`);
    assert.ok(stat.isDirectory(), `${label} root must be a directory: ${absolute}`);
    return fs.realpathSync(absolute);
  }
  assert.equal(mustExist, false, `${label} root does not exist: ${absolute}`);

  let ancestor = path.dirname(absolute);
  while (!fs.existsSync(ancestor)) {
    const parent = path.dirname(ancestor);
    assert.notEqual(parent, ancestor, `${label} root has no existing ancestor: ${absolute}`);
    ancestor = parent;
  }
  const stat = fs.lstatSync(ancestor);
  assert.ok(!stat.isSymbolicLink(), `${label} ancestor must not be a symbolic link: ${ancestor}`);
  assert.ok(stat.isDirectory(), `${label} ancestor must be a directory: ${ancestor}`);
  return path.resolve(fs.realpathSync(ancestor), path.relative(ancestor, absolute));
}

function assertSeparateRoots(sourceRoot, workspace) {
  const source = canonicalRoot(sourceRoot, "immutable source", true);
  const destination = canonicalRoot(workspace, "acceptance workspace", false);
  assert.notEqual(source, destination, "acceptance workspace cannot equal the immutable base root");
  assert.ok(!isInside(source, destination), "acceptance workspace cannot be inside the immutable base root");
  assert.ok(!isInside(destination, source), "immutable base root cannot be inside the acceptance workspace");
  return { source, destination };
}

function assertSafeRelative(relative) {
  assert.equal(typeof relative, "string", "owned path must be a string");
  assert.ok(relative.length > 0, "owned path must not be empty");
  assert.equal(path.posix.normalize(relative), relative, `owned path is not normalized: ${relative}`);
  assert.ok(!relative.startsWith("/"), `owned path is absolute: ${relative}`);
  assert.ok(!relative.split("/").includes(".."), `owned path traverses a parent: ${relative}`);
  assert.ok(!relative.includes("\\"), `owned path uses a backslash: ${relative}`);
  return relative;
}

function checkedOwnedPath(root, relative) {
  assertSafeRelative(relative);
  const resolved = path.resolve(root, ...relative.split("/"));
  assert.ok(isInside(root, resolved), `owned path escaped workspace: ${relative}`);
  return resolved;
}

const crcTable = (() => {
  const table = [];
  for (let value = 0; value < 256; value += 1) {
    let current = value;
    for (let bit = 0; bit < 8; bit += 1) {
      current = current & 1 ? 0xedb88320 ^ (current >>> 1) : current >>> 1;
    }
    table.push(current >>> 0);
  }
  return table;
})();

function crc32(bytes) {
  let current = 0xffffffff;
  for (const byte of bytes) current = crcTable[(current ^ byte) & 0xff] ^ (current >>> 8);
  return (current ^ 0xffffffff) >>> 0;
}

function pngChunk(type, data) {
  const name = Buffer.from(type, "ascii");
  const result = Buffer.alloc(12 + data.length);
  result.writeUInt32BE(data.length, 0);
  name.copy(result, 4);
  data.copy(result, 8);
  result.writeUInt32BE(crc32(Buffer.concat([name, data])), 8 + data.length);
  return result;
}

function solidPng(specification) {
  const { width, height, rgba } = specification;
  assert.ok(Number.isInteger(width) && width > 0, "PNG width must be positive");
  assert.ok(Number.isInteger(height) && height > 0, "PNG height must be positive");
  assert.deepEqual(rgba.length, 4, "PNG RGBA must have four channels");
  assert.ok(rgba.every((channel) => Number.isInteger(channel) && channel >= 0 && channel <= 255));
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(width, 0);
  ihdr.writeUInt32BE(height, 4);
  ihdr[8] = 8;
  ihdr[9] = 6;
  const stride = 1 + width * 4;
  const raw = Buffer.alloc(height * stride);
  const pixel = Buffer.from(rgba);
  for (let row = 0; row < height; row += 1) {
    raw[row * stride] = 0;
    for (let column = 0; column < width; column += 1) {
      pixel.copy(raw, row * stride + 1 + column * 4);
    }
  }
  return Buffer.concat([
    Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]),
    pngChunk("IHDR", ihdr),
    pngChunk("IDAT", zlib.deflateSync(raw, { level: 9 })),
    pngChunk("IEND", Buffer.alloc(0)),
  ]);
}

function validateScenario(scenario) {
  assert.equal(scenario.schema_version, 1);
  assert.equal(scenario.family, "encyclopedia-native-live-mod-acceptance");
  assert.equal(scenario.fixture.synthetic_test_only, true);
  assert.equal(scenario.fixture.browser_policy, "unmodified-base-only");
  assert.equal(scenario.fixture.stable_topic_id, scenario.source.topic.topic_id);
  assert.ok(!JSON.stringify(scenario).includes("EDATA.192"), "deferred EDATA.192 must not enter E26");

  const generated = new Map();
  for (const [name, specification] of Object.entries(scenario.synthetic_images)) {
    const bytes = solidPng(specification);
    assert.equal(bytes.length, specification.byte_length, `${name} PNG byte length`);
    assert.equal(sha256(bytes), specification.sha256, `${name} PNG digest`);
    generated.set(name, bytes);
  }

  for (const [key, mod] of Object.entries(scenario.mods)) {
    assertSafeRelative(mod.directory);
    assert.equal(mod.manifest.name, mod.directory, `${key} manifest/directory identity`);
    assert.ok(/^\d+\.\d+\.\d+$/.test(mod.manifest.version), `${key} version must be explicit semver`);
    assert.equal(mod.manifest.author, "Open Rebellion synthetic acceptance fixture");
    assert.ok(Array.isArray(mod.overlay), `${key} overlay must be a root array`);
    for (const [relative, image] of Object.entries(mod.assets)) {
      assertSafeRelative(relative);
      assert.ok(relative.startsWith("encyclopedia/assets/"), `${key} asset must use the confined encyclopedia namespace`);
      assert.ok(generated.has(image), `${key} references unknown synthetic image ${image}`);
    }
  }

  assert.deepEqual(
    scenario.journeys.map(({ id }) => id),
    ["original-live-edits", "faithful-hd-mod-null-precedence"],
  );
  const requiredSteps = new Set([
    "base-ready",
    "install-alpha",
    "install-beta",
    "image-only-atomic-save",
    "malformed-beta-keeps-last-good",
    "fix-beta-recovers",
    "disable-alpha-does-not-resurrect",
    "missing-dependency-restores-base",
    "disable-all-restores-base",
    "approved-hd-ready",
    "mod-overrides-hd",
    "explicit-null-overrides-mod-and-hd",
    "remove-null-restores-mod",
    "remove-mod-restores-hd",
  ]);
  const seenSteps = new Set();
  for (const journey of scenario.journeys) {
    assert.ok(["original-parity", "faithful-hd"].includes(journey.render_profile));
    for (const step of journey.steps) {
      assert.ok(!seenSteps.has(step.id), `duplicate step ID ${step.id}`);
      seenSteps.add(step.id);
      assert.ok(operationKinds.has(step.operation.kind), `unsupported operation ${step.operation.kind}`);
      assert.equal(step.expect.topic_id ?? scenario.source.topic.topic_id, scenario.source.topic.topic_id);
      assert.ok(Object.hasOwn(step.expect, "published_changed"), `${step.id} must state publication semantics`);
      if (step.expect.image_source === "null") {
        assert.equal(step.expect.asset_id, null);
        assert.equal(step.expect.digest, null);
      } else {
        assert.match(step.expect.digest, /^[0-9a-f]{64}$/);
      }
    }
  }
  assert.deepEqual(seenSteps, requiredSteps, "scenario step matrix changed without review");
  assert.equal(scenario.invariants.selection_topic_id_stays, scenario.source.topic.topic_id);
  assert.equal(scenario.invariants.world_fingerprint_unchanged_for_fixed_enabled_list, true);
  assert.equal(scenario.invariants.save_fingerprint_unchanged_for_fixed_enabled_list, true);
  assert.equal(scenario.invariants.rng_fingerprint_unchanged_for_all_content_refreshes, true);
  assert.equal(scenario.invariants.maximum_live_texture_entries, 1);
  assert.equal(scenario.invariants.maximum_retained_candidate_generations, 1);
  assert.equal(
    scenario.invariants.dependency_failure_effective_policy,
    "empty-resolved-order-restores-base",
  );
  const dependencyStep = scenario.journeys
    .flatMap(({ steps }) => steps)
    .find(({ id }) => id === "missing-dependency-restores-base");
  assert.ok(dependencyStep, "dependency failure step is required");
  assert.equal(dependencyStep.expect.image_source, "base");
  assert.equal(dependencyStep.expect.asset_id, scenario.source.topic.image.asset_id);
  assert.equal(dependencyStep.expect.title, scenario.source.topic.title);
  return generated;
}

function verifySource(scenario, sourceRoot) {
  const catalogPath = path.join(sourceRoot, "encyclopedia/catalog.json");
  const manifestPath = path.join(sourceRoot, "encyclopedia/manifest.json");
  const catalogBytes = fs.readFileSync(catalogPath);
  const manifestBytes = fs.readFileSync(manifestPath);
  assert.equal(sha256(catalogBytes), scenario.source.catalog_sha256, "owned catalog identity");
  assert.equal(sha256(manifestBytes), scenario.source.manifest_sha256, "owned manifest identity");
  const catalog = JSON.parse(catalogBytes);
  const manifest = JSON.parse(manifestBytes);
  assert.equal(manifest.source_profile, scenario.source.source_profile);
  assert.equal(manifest.catalog_sha256, scenario.source.catalog_sha256);
  assert.equal(manifest.files["catalog.json"], scenario.source.catalog_sha256);

  const expected = scenario.source.topic;
  const topic = catalog.topics[expected.topic_id];
  assert.ok(topic, `owned catalog lacks ${expected.topic_id}`);
  const localized = topic.localized[expected.language];
  assert.equal(localized.title, expected.title);
  assert.equal(sha256(Buffer.from(localized.title)), expected.title_sha256);
  assert.equal(sha256(Buffer.from(localized.body)), expected.body_sha256);
  assert.equal(localized.image_id, expected.image.asset_id);
  const binding = catalog.bindings.find(({ topic_id: topicId }) => topicId === expected.topic_id);
  assert.deepEqual(
    { family: binding.family, dat_id: binding.dat_id, variant: binding.variant },
    expected.binding,
  );
  const descriptor = catalog.images[expected.image.asset_id];
  assert.deepEqual(
    {
      path: descriptor.path,
      format: descriptor.format,
      width: descriptor.width,
      height: descriptor.height,
      byte_length: descriptor.byte_length,
      sha256: descriptor.sha256,
    },
    {
      path: expected.image.relative_path,
      format: expected.image.format,
      width: expected.image.width,
      height: expected.image.height,
      byte_length: expected.image.byte_length,
      sha256: expected.image.sha256,
    },
  );
  const imageBytes = fs.readFileSync(path.join(sourceRoot, "encyclopedia", descriptor.path));
  assert.equal(imageBytes.length, descriptor.byte_length);
  assert.equal(sha256(imageBytes), descriptor.sha256);
  for (const source of manifest.binding_sources) {
    assert.equal(sha256(fs.readFileSync(path.join(sourceRoot, source.basename))), source.sha256);
  }

  return {
    source_profile: manifest.source_profile,
    catalog_sha256: sha256(catalogBytes),
    manifest_sha256: sha256(manifestBytes),
    binding_sources_verified: manifest.binding_sources.length,
    topic_id: expected.topic_id,
    base_asset_id: expected.image.asset_id,
    base_asset_sha256: sha256(imageBytes),
  };
}

function tomlString(value) {
  return JSON.stringify(value);
}

function manifestToml(manifest) {
  const lines = [
    `name = ${tomlString(manifest.name)}`,
    `version = ${tomlString(manifest.version)}`,
    `author = ${tomlString(manifest.author)}`,
    `description = ${tomlString(manifest.description)}`,
  ];
  const dependencies = Object.entries(manifest.dependencies).sort(([left], [right]) => left.localeCompare(right));
  if (dependencies.length) {
    lines.push("", "[dependencies]");
    for (const [name, requirement] of dependencies) lines.push(`${tomlString(name)} = ${tomlString(requirement)}`);
  }
  return `${lines.join("\n")}\n`;
}

class OwnedWorkspace {
  constructor(root, sourceRoot) {
    const { source, destination } = assertSeparateRoots(sourceRoot, root);
    this.root = destination;
    this.sourceRoot = source;
    this.files = new Set();
    this.directories = new Set(["."]);
  }

  initialize() {
    assertSeparateRoots(this.sourceRoot, this.root);
    if (fs.existsSync(this.root)) {
      const entries = fs.readdirSync(this.root);
      assert.deepEqual(entries, [], `fresh acceptance workspace is not empty: ${this.root}`);
    } else {
      fs.mkdirSync(this.root, { recursive: true });
    }
    this.write(ownershipFile, Buffer.from("{}\n"));
  }

  static openExisting(root, sourceRoot) {
    const { source, destination } = assertSeparateRoots(sourceRoot, root);
    const ownership = readOwnership(destination);
    assert.equal(canonicalRoot(ownership.source_root, "recorded immutable source", true), source);
    const observed = inventory(destination);
    const expectedFiles = new Set(ownership.files);
    expectedFiles.add(ownershipFile);
    assert.deepEqual(
      observed.files.filter((relative) => !expectedFiles.has(relative)),
      [],
      "refusing to mutate an acceptance workspace with unowned files",
    );
    const expectedDirectories = new Set(ownership.directories);
    assert.deepEqual(
      observed.directories.filter((relative) => !expectedDirectories.has(relative)),
      [],
      "refusing to mutate an acceptance workspace with unowned directories",
    );
    const workspace = new OwnedWorkspace(destination, source);
    workspace.files = new Set(ownership.files);
    workspace.files.add(ownershipFile);
    workspace.directories = new Set([".", ...ownership.directories]);
    return workspace;
  }

  rememberParents(relative) {
    let parent = path.posix.dirname(relative);
    while (parent !== ".") {
      this.directories.add(parent);
      parent = path.posix.dirname(parent);
    }
  }

  write(relative, bytes) {
    const target = checkedOwnedPath(this.root, relative);
    fs.mkdirSync(path.dirname(target), { recursive: true });
    fs.writeFileSync(target, bytes);
    this.files.add(relative);
    this.rememberParents(relative);
  }

  atomicWrite(relative, bytes) {
    const target = checkedOwnedPath(this.root, relative);
    const candidateRelative = `${path.posix.dirname(relative)}/.${path.posix.basename(relative)}.e26-candidate`
      .replace(/^\.\//, "");
    const candidate = checkedOwnedPath(this.root, candidateRelative);
    fs.mkdirSync(path.dirname(target), { recursive: true });
    fs.writeFileSync(candidate, bytes, { flag: "wx" });
    fs.renameSync(candidate, target);
    this.files.add(relative);
    this.rememberParents(relative);
  }

  saveOwnership() {
    const payload = {
      schema_version: 1,
      owner: "E26-native-acceptance",
      source_root: this.sourceRoot,
      files: [...this.files].sort(),
      directories: [...this.directories].filter((entry) => entry !== ".").sort(),
    };
    this.write(ownershipFile, Buffer.from(stableJson(payload)));
  }
}

function writeConfig(workspace, enabled) {
  assert.equal(new Set(enabled).size, enabled.length, "enabled mod list contains duplicates");
  workspace.atomicWrite("mods/config.toml", Buffer.from(`enabled = [${enabled.map(tomlString).join(", ")}]\n`));
}

function materializeMod(workspace, scenario, generated, key) {
  const mod = scenario.mods[key];
  assert.ok(mod, `unknown mod key ${key}`);
  const prefix = `mods/${mod.directory}`;
  workspace.write(`${prefix}/mod.toml`, Buffer.from(manifestToml(mod.manifest)));
  workspace.write(`${prefix}/encyclopedia.json`, Buffer.from(stableJson(mod.overlay)));
  for (const [relative, imageName] of Object.entries(mod.assets)) {
    workspace.write(`${prefix}/${relative}`, generated.get(imageName));
  }
}

function materializeHd(workspace, scenario, generated) {
  const configuration = scenario.faithful_hd;
  const image = scenario.synthetic_images[configuration.image];
  const output = `hd/${configuration.output_relative_path}`;
  workspace.write(output, generated.get(configuration.image));
  workspace.write(
    "hd/manifest.json",
    Buffer.from(stableJson({
      schema_version: 1,
      profile: configuration.profile,
      assets: {
        [configuration.approval_key]: {
          approved: true,
          review: {
            reviewer: "synthetic-e26-acceptance",
            evidence: "feature-only fixture; not original-game evidence",
          },
          gates: { human_review: "pass" },
          source: { sha256: configuration.source_sha256 },
          output: { sha256: image.sha256 },
        },
      },
    })),
  );
}

function applyOperation(workspace, scenario, generated, operation) {
  switch (operation.kind) {
    case "reset":
      writeConfig(workspace, operation.enabled);
      return;
    case "install":
      for (const key of operation.mods) materializeMod(workspace, scenario, generated, key);
      writeConfig(workspace, operation.enabled);
      return;
    case "replace_asset_atomically": {
      const mod = scenario.mods[operation.mod];
      assert.ok(mod, `unknown mod ${operation.mod}`);
      assertSafeRelative(operation.path);
      workspace.atomicWrite(`mods/${mod.directory}/${operation.path}`, generated.get(operation.image));
      return;
    }
    case "replace_overlay_atomically": {
      const mod = scenario.mods[operation.mod];
      assert.ok(mod, `unknown mod ${operation.mod}`);
      workspace.atomicWrite(`mods/${mod.directory}/encyclopedia.json`, Buffer.from(operation.raw_utf8));
      return;
    }
    case "replace_overlay": {
      const mod = scenario.mods[operation.mod];
      assert.ok(mod, `unknown mod ${operation.mod}`);
      workspace.atomicWrite(`mods/${mod.directory}/encyclopedia.json`, Buffer.from(stableJson(operation.overlay)));
      return;
    }
    case "set_enabled":
      writeConfig(workspace, operation.enabled);
      return;
    default:
      throw new Error(`unsupported operation ${operation.kind}`);
  }
}

function findStep(scenario, journeyId, stepId) {
  const journey = scenario.journeys.find(({ id }) => id === journeyId);
  assert.ok(journey, `unknown journey ${journeyId}`);
  const step = journey.steps.find(({ id }) => id === stepId);
  assert.ok(step, `unknown step ${journeyId}/${stepId}`);
  return { journey, step };
}

function applyNamedStep(scenario, sourceRoot, workspaceRoot, journeyId, stepId) {
  const generated = validateScenario(scenario);
  const workspace = OwnedWorkspace.openExisting(workspaceRoot, sourceRoot);
  const { journey, step } = findStep(scenario, journeyId, stepId);
  applyOperation(workspace, scenario, generated, step.operation);
  workspace.saveOwnership();
  return {
    schema_version: 1,
    status: "step-applied",
    journey: journey.id,
    render_profile: journey.render_profile,
    step: step.id,
    operation: step.operation.kind,
    expect: step.expect,
  };
}

function prepareWorkspace(scenario, sourceRoot, workspaceRoot) {
  const generated = validateScenario(scenario);
  const source = verifySource(scenario, sourceRoot);
  const workspace = new OwnedWorkspace(workspaceRoot, sourceRoot);
  workspace.initialize();
  workspace.write("mods/config.toml", Buffer.from("enabled = []\n"));
  materializeHd(workspace, scenario, generated);
  workspace.write(
    "scenario.json",
    Buffer.from(stableJson({
      schema_version: scenario.schema_version,
      family: scenario.family,
      scenario_sha256: sha256(Buffer.from(stableJson(scenario))),
      source,
      browser_policy: scenario.fixture.browser_policy,
      synthetic_test_only: true,
    })),
  );
  workspace.saveOwnership();
  return { workspace, generated, source };
}

function readOwnership(workspaceRoot) {
  const marker = checkedOwnedPath(workspaceRoot, ownershipFile);
  const stat = fs.lstatSync(marker);
  assert.ok(stat.isFile() && !stat.isSymbolicLink(), "E26 ownership marker must be a regular file");
  const ownership = readJson(marker);
  assert.equal(ownership.schema_version, 1);
  assert.equal(ownership.owner, "E26-native-acceptance");
  return ownership;
}

function inventory(root) {
  const files = [];
  const directories = [];
  function walk(relative) {
    const absolute = relative === "." ? root : checkedOwnedPath(root, relative);
    for (const entry of fs.readdirSync(absolute, { withFileTypes: true })) {
      const child = relative === "." ? entry.name : `${relative}/${entry.name}`;
      const stat = fs.lstatSync(checkedOwnedPath(root, child));
      assert.ok(!stat.isSymbolicLink(), `refusing symlink in owned workspace: ${child}`);
      if (stat.isDirectory()) {
        directories.push(child);
        walk(child);
      } else {
        assert.ok(stat.isFile(), `refusing non-regular workspace member: ${child}`);
        files.push(child);
      }
    }
  }
  walk(".");
  return { files: files.sort(), directories: directories.sort() };
}

function cleanOwnedWorkspace(workspaceRoot, sourceRoot) {
  const { source, destination } = assertSeparateRoots(sourceRoot, workspaceRoot);
  const ownership = readOwnership(destination);
  assert.equal(canonicalRoot(ownership.source_root, "recorded immutable source", true), source);
  const observed = inventory(destination);
  const expectedFiles = new Set(ownership.files);
  expectedFiles.add(ownershipFile);
  const unknownFiles = observed.files.filter((relative) => !expectedFiles.has(relative));
  const expectedDirectories = new Set(ownership.directories);
  const unknownDirectories = observed.directories.filter((relative) => !expectedDirectories.has(relative));
  assert.deepEqual({ unknownFiles, unknownDirectories }, { unknownFiles: [], unknownDirectories: [] },
    "refusing cleanup because the workspace contains unowned entries");
  for (const relative of observed.files) fs.unlinkSync(checkedOwnedPath(destination, relative));
  for (const relative of observed.directories.sort((left, right) => right.length - left.length)) {
    fs.rmdirSync(checkedOwnedPath(destination, relative));
  }
  fs.rmdirSync(destination);
}

function runbook(scenario, scenarioPath, sourceRoot, workspaceRoot) {
  const scenarioHash = sha256(fs.readFileSync(scenarioPath));
  const parsedScenarioHash = sha256(Buffer.from(stableJson(scenario)));
  const artifactManifest = path.join(
    repositoryRoot,
    ".artifacts/encyclopedia/E26/r4/artifact-manifest.json",
  );
  const lines = [
    "# E26 coordinator native live-acceptance runbook",
    "",
    `Exact scenario-file SHA-256: \`${scenarioHash}\``,
    `Parsed/pretty scenario SHA-256: \`${parsedScenarioHash}\``,
    `Immutable owned base: \`${path.resolve(sourceRoot)}\``,
    `Synthetic acceptance workspace: \`${path.resolve(workspaceRoot)}\``,
    `Immutable build manifest: \`${artifactManifest}\``,
    "",
    "The worker did not execute computer-use. Coordinator Astra must launch the exact feature binary with a fresh native display and inspect actual pixels. The fixture must consume the accepted E24/E25/E46/E52 implementation; modeled telemetry is not acceptance.",
    "Before launch, verify every binary/module SHA-256 against the immutable build manifest. Use the feature native artifact for the author-edit journeys. The feature WASM artifact is compile evidence only; browser v1 must remain the unmodified base and must never receive this mod workspace or HD root.",
    "",
    "Environment contract:",
    "",
    `- \`${scenario.fixture.request_environment}=${scenario.fixture.request_value}\``,
    `- \`${scenario.fixture.mods_root_environment}=${path.resolve(workspaceRoot, "mods")}\``,
    `- \`REBELLION_ENCYCLOPEDIA_HD_ROOT=${path.resolve(workspaceRoot, "hd")}\` (feature fixture only)`,
    "- `OPEN_REBELLION_ASSET_PROFILE=original-parity` for the first journey; `faithful-hd` for the second.",
    "- Pass the immutable owned base directory as the native data argument. Do not copy or modify it.",
    "",
    "Exact native launch (set the render profile to the journey named below):",
    "",
    "```sh",
    `env REBELLION_ENCYCLOPEDIA_INSPECTOR=1 REBELLION_ENCYCLOPEDIA_MODS_DIR=${path.resolve(workspaceRoot, "mods")} REBELLION_ENCYCLOPEDIA_HD_ROOT=${path.resolve(workspaceRoot, "hd")} OPEN_REBELLION_ASSET_PROFILE=original-parity ${path.join(repositoryRoot, ".artifacts/encyclopedia/E26/r4/builds/open-rebellion-e26-r4-feature")} ${path.resolve(sourceRoot)}`,
    "```",
    "",
    "For every step: apply the declared operation, wait at least for the accepted 100 ms quiet boundary (never use sleep alone as proof), wait for matching fixture telemetry, inspect a screenshot, and record the selected stable topic, visible title/body/art, asset ID/digest, cache release/upload, generation, retained byte/texture counts, diagnostics, and actual world/save/RNG fingerprints.",
    "",
  ];
  for (const journey of scenario.journeys) {
    lines.push(`## ${journey.id} (${journey.render_profile})`, "");
    for (const [index, step] of journey.steps.entries()) {
      const expected = step.expect;
      lines.push(
        `${index + 1}. **${step.id}** — operation \`${step.operation.kind}\`; expect title \`${expected.title}\`, image source \`${expected.image_source}\`, asset \`${expected.asset_id ?? "none"}\`, digest \`${expected.digest ?? "none"}\`, published_changed \`${expected.published_changed}\`${expected.diagnostic_code ? `, diagnostic \`${expected.diagnostic_code}\`` : ""}.`,
        "",
        "   ```sh",
        `   node tools/interface-parity/encyclopedia-native-acceptance.mjs --apply-step --journey ${journey.id} --step ${step.id} --source-root ${path.resolve(sourceRoot)} --workspace ${path.resolve(workspaceRoot)}`,
        "   ```",
        "",
      );
    }
    lines.push("");
  }
  lines.push(
    "Required closeout:",
    "",
    "- Fixed enabled-list content edits preserve actual world/save/RNG fingerprints; toggles separately record the existing active-mod metadata change.",
    "- `original:5696` stays selected across every publication.",
    "- Replaced/removed/null images release the superseded texture and never retain more than one live selected entry.",
    "- Browser evidence remains the immutable base from E21; no mod directory or HD candidate is exposed to browser v1.",
    "- Hash the immutable base catalog/manifest/art before and after; hash all synthetic inputs and screenshots/logs.",
    "- Stop the app/display cleanly. Run `--cleanup` only after preserving evidence; cleanup refuses unknown files, directories, symlinks, and non-regular members.",
    "",
  );
  return `${lines.join("\n")}\n`;
}

function writeOutput(file, value) {
  if (!file) {
    process.stdout.write(typeof value === "string" ? value : stableJson(value));
    return;
  }
  fs.mkdirSync(path.dirname(file), { recursive: true });
  fs.writeFileSync(file, typeof value === "string" ? value : stableJson(value));
}

function selfTest(scenario) {
  const generated = validateScenario(scenario);
  const temporary = fs.mkdtempSync(path.join(os.tmpdir(), "open-rebellion-e26-harness-"));
  const source = path.join(temporary, "immutable-source");
  const workspaceRoot = path.join(temporary, "acceptance");
  fs.mkdirSync(source);
  const workspace = new OwnedWorkspace(workspaceRoot, source);
  workspace.initialize();
  workspace.write("mods/config.toml", Buffer.from("enabled = []\n"));
  materializeHd(workspace, scenario, generated);

  const originalJourney = scenario.journeys[0];
  for (const step of originalJourney.steps) applyOperation(workspace, scenario, generated, step.operation);
  const betaRoot = checkedOwnedPath(workspaceRoot, "mods/e26-beta");
  assert.equal(
    sha256(fs.readFileSync(path.join(betaRoot, "encyclopedia/assets/beta.png"))),
    scenario.synthetic_images.beta_replacement.sha256,
  );
  assert.equal(fs.readFileSync(path.join(betaRoot, "encyclopedia.json"), "utf8").includes("E26 Beta Recovered"), true);
  assert.equal(fs.readFileSync(path.join(workspaceRoot, "mods/config.toml"), "utf8"), "enabled = []\n");
  workspace.saveOwnership();

  const injected = path.join(workspaceRoot, "user-note.txt");
  fs.writeFileSync(injected, "must survive");
  assert.throws(
    () => cleanOwnedWorkspace(workspaceRoot, source),
    /unowned entries/,
    "cleanup must refuse unknown user content",
  );
  assert.equal(fs.readFileSync(injected, "utf8"), "must survive");
  fs.unlinkSync(injected);
  cleanOwnedWorkspace(workspaceRoot, source);
  assert.equal(fs.existsSync(workspaceRoot), false);

  if (process.platform !== "win32") {
    const protectedRoot = path.join(temporary, "protected-user-directory");
    const alias = path.join(temporary, "rebound-acceptance");
    fs.mkdirSync(protectedRoot);
    fs.writeFileSync(path.join(protectedRoot, "sentinel.txt"), "must survive root rebound");
    fs.writeFileSync(
      path.join(protectedRoot, ownershipFile),
      stableJson({
        schema_version: 1,
        owner: "E26-native-acceptance",
        source_root: source,
        files: ["sentinel.txt"],
        directories: [],
      }),
    );
    fs.symlinkSync(protectedRoot, alias, "dir");
    assert.throws(
      () => cleanOwnedWorkspace(alias, source),
      /symbolic link/,
      "cleanup must reject a workspace root rebound to a symlink",
    );
    assert.equal(
      fs.readFileSync(path.join(protectedRoot, "sentinel.txt"), "utf8"),
      "must survive root rebound",
      "cleanup refusal must happen before deleting target contents",
    );
    fs.unlinkSync(alias);
    fs.unlinkSync(path.join(protectedRoot, ownershipFile));
    fs.unlinkSync(path.join(protectedRoot, "sentinel.txt"));
    fs.rmdirSync(protectedRoot);
  }

  fs.rmdirSync(source);
  fs.rmdirSync(temporary);

  return {
    schema_version: 1,
    status: "pass",
    scope: "structural_file_materialization_only",
    checks: [
      "strict scenario matrix",
      "deterministic PNG identities",
      "confined mod and HD materialization",
      "atomic overlay and image replacement",
      "exact final enabled state",
      "unknown-content cleanup refusal",
      "workspace-root symlink cleanup refusal before mutation",
      "owned cleanup after refusal",
    ],
    journeys: scenario.journeys.length,
    steps: scenario.journeys.reduce((total, journey) => total + journey.steps.length, 0),
  };
}

function main() {
  const options = parseArguments(process.argv.slice(2));
  const scenario = readJson(options.scenario);
  switch (options.action) {
    case "self-test":
      writeOutput(options.output, selfTest(scenario));
      return;
    case "verify-inputs": {
      const generated = validateScenario(scenario);
      const source = verifySource(scenario, options.sourceRoot);
      writeOutput(options.output, {
        schema_version: 1,
        status: "pass",
        scenario_sha256: sha256(fs.readFileSync(options.scenario)),
        source,
        synthetic_images: Object.fromEntries(
          [...generated].map(([name, bytes]) => [name, { byte_length: bytes.length, sha256: sha256(bytes) }]),
        ),
        browser_policy: scenario.fixture.browser_policy,
      });
      return;
    }
    case "prepare": {
      const prepared = prepareWorkspace(scenario, options.sourceRoot, options.workspace);
      const book = runbook(scenario, options.scenario, options.sourceRoot, options.workspace);
      prepared.workspace.write("RUNBOOK.md", Buffer.from(book));
      prepared.workspace.saveOwnership();
      writeOutput(options.output, {
        schema_version: 1,
        status: "prepared",
        workspace: path.resolve(options.workspace),
        source: prepared.source,
        scenario_sha256: sha256(fs.readFileSync(options.scenario)),
        runbook: path.join(path.resolve(options.workspace), "RUNBOOK.md"),
        live_status: "awaiting coordinator handoff and Astra execution",
      });
      return;
    }
    case "apply-step":
      writeOutput(
        options.output,
        applyNamedStep(scenario, options.sourceRoot, options.workspace, options.journey, options.step),
      );
      return;
    case "cleanup":
      validateScenario(scenario);
      cleanOwnedWorkspace(options.workspace, options.sourceRoot);
      writeOutput(options.output, { schema_version: 1, status: "cleaned", workspace: path.resolve(options.workspace) });
      return;
    case "print-runbook":
      validateScenario(scenario);
      writeOutput(
        options.output,
        runbook(scenario, options.scenario, options.sourceRoot, options.workspace),
      );
      return;
    default:
      throw new Error(`unhandled action ${options.action}`);
  }
}

try {
  main();
} catch (error) {
  console.error(`E26 native acceptance harness failed: ${error.stack || error}`);
  process.exitCode = 1;
}
