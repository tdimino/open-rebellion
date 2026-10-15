#!/usr/bin/env node

import { createHash } from "node:crypto";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const defaultRepoRoot = path.resolve(here, "../..");
const defaultSiteRoot = path.join(defaultRepoRoot, ".artifacts/interface-parity/site");

function sourceTree(relativeRoot, include) {
  const absoluteRoot = path.join(defaultRepoRoot, relativeRoot);
  const files = [];
  const visit = (directory) => {
    for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
      const absolute = path.join(directory, entry.name);
      if (entry.isDirectory() && entry.name !== "node_modules") visit(absolute);
      else if (entry.isFile() && include(absolute)) {
        files.push(path.relative(defaultRepoRoot, absolute).split(path.sep).join("/"));
      }
    }
  };
  visit(absoluteRoot);
  return files;
}

export const WINDOWING_BUILD_MANIFEST_FILENAME = ".windowing-contract-build.json";
export const WINDOWING_BUILD_SOURCE_FILES = Object.freeze([...new Set([
  "Cargo.toml",
  "Cargo.lock",
  ".cargo/config.toml",
  "assets/fonts/LiberationSans-Regular.ttf",
  "assets/fonts/LiberationSans-Bold.ttf",
  "docs/plans/2026-10-10-strategic-windowing-toolkit-implementation.md",
  "scripts/build-interface-test-wasm.sh",
  "scripts/build-wasm.sh",
  "scripts/build-runtime-pack.py",
  "web/index.html",
  "web/gl.js",
  ...sourceTree("crates", (filename) => filename.endsWith(".rs") || path.basename(filename) === "Cargo.toml"),
  ...sourceTree("tools/interface-parity", (filename) => {
    const relative = path.relative(defaultRepoRoot, filename).split(path.sep).join("/");
    return (filename.endsWith(".mjs") && !filename.endsWith(".test.mjs"))
      || relative.startsWith("tools/interface-parity/schemas/") && filename.endsWith(".json")
      || [
        "tools/interface-parity/browser.json",
        "tools/interface-parity/package.json",
        "tools/interface-parity/package-lock.json",
        "tools/interface-parity/windowing-contract-manifest.json",
      ].includes(relative);
  }),
])].sort());
export const WINDOWING_BUILD_ARTIFACT_FILES = Object.freeze([
  "index.html",
  "gl.js",
  "open-rebellion-test.wasm",
  "data/runtime.orpk"
]);

function fail(message) {
  throw new Error(`windowing build manifest: ${message}`);
}

function sha256File(filename) {
  return createHash("sha256").update(fs.readFileSync(filename)).digest("hex");
}

function normalizeFiles(files, label) {
  if (!Array.isArray(files) || files.length === 0
      || files.some((file) => typeof file !== "string" || file.length === 0 || path.isAbsolute(file))) {
    fail(`${label} files must be nonempty relative paths`);
  }
  const normalized = [...new Set(files)].sort();
  if (normalized.length !== files.length) fail(`${label} files contain duplicates`);
  return normalized;
}

function records(base, files, label) {
  return normalizeFiles(files, label).map((relative) => {
    const absolute = path.resolve(base, relative);
    if (!absolute.startsWith(`${path.resolve(base)}${path.sep}`) || !fs.existsSync(absolute)
        || !fs.statSync(absolute).isFile()) {
      fail(`${label} file is missing or escapes its root: ${relative}`);
    }
    return { path: relative, sha256: sha256File(absolute) };
  });
}

function options(value = {}) {
  return {
    repoRoot: path.resolve(value.repoRoot ?? defaultRepoRoot),
    siteRoot: path.resolve(value.siteRoot ?? defaultSiteRoot),
    sourceFiles: value.sourceFiles ?? WINDOWING_BUILD_SOURCE_FILES,
    artifactFiles: value.artifactFiles ?? WINDOWING_BUILD_ARTIFACT_FILES,
  };
}

export function writeWindowingBuildManifest(value = {}) {
  const configured = options(value);
  const manifest = {
    schema_version: 1,
    kind: "open-rebellion-windowing-contract-build",
    sources: records(configured.repoRoot, configured.sourceFiles, "source"),
    artifacts: records(configured.siteRoot, configured.artifactFiles, "artifact"),
  };
  fs.writeFileSync(
    path.join(configured.siteRoot, WINDOWING_BUILD_MANIFEST_FILENAME),
    `${JSON.stringify(manifest, null, 2)}\n`,
  );
  return manifest;
}

function verifyRecords(actual, expectedFiles, base, label) {
  if (!Array.isArray(actual)) fail(`${label} records are missing`);
  const expected = normalizeFiles(expectedFiles, label);
  const actualPaths = actual.map((record) => record?.path).sort();
  if (JSON.stringify(actualPaths) !== JSON.stringify(expected)) {
    fail(`${label} inventory mismatch`);
  }
  for (const record of actual) {
    if (typeof record?.path !== "string" || !/^[a-f0-9]{64}$/.test(record.sha256 ?? "")) {
      fail(`malformed ${label} record`);
    }
    const absolute = path.resolve(base, record.path);
    if (!absolute.startsWith(`${path.resolve(base)}${path.sep}`) || !fs.existsSync(absolute)
        || !fs.statSync(absolute).isFile()) {
      fail(`${label} file is missing or escapes its root: ${record.path}`);
    }
    if (sha256File(absolute) !== record.sha256) {
      fail(`${label} hash mismatch for ${record.path}`);
    }
  }
}

export function verifyWindowingBuildManifest(value = {}) {
  const configured = options(value);
  const filename = path.join(configured.siteRoot, WINDOWING_BUILD_MANIFEST_FILENAME);
  if (!fs.existsSync(filename)) fail(`missing ${WINDOWING_BUILD_MANIFEST_FILENAME}; rebuild the fixture site`);
  const manifest = JSON.parse(fs.readFileSync(filename, "utf8"));
  if (manifest?.schema_version !== 1
      || manifest?.kind !== "open-rebellion-windowing-contract-build"
      || Object.keys(manifest).sort().join(",") !== "artifacts,kind,schema_version,sources") {
    fail("manifest structure is invalid");
  }
  verifyRecords(manifest.sources, configured.sourceFiles, configured.repoRoot, "source");
  verifyRecords(manifest.artifacts, configured.artifactFiles, configured.siteRoot, "artifact");
  return { status: "verified", manifest };
}

if (process.argv[1] && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href) {
  try {
    const manifest = writeWindowingBuildManifest();
    process.stdout.write(`${JSON.stringify({
      status: "written",
      sources: manifest.sources.length,
      artifacts: manifest.artifacts.length,
    })}\n`);
  } catch (error) {
    process.stderr.write(`${error.stack || error}\n`);
    process.exitCode = 1;
  }
}
