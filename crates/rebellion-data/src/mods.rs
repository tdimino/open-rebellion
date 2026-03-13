//! Mod loading system for Open Rebellion.
//!
//! # Overview
//!
//! Mods are directories under a `mods/` folder. Each mod directory contains:
//!   - `mod.toml`  — manifest: name, version, author, description, dependencies
//!   - `*.json`    — JSON overlay files keyed by entity type (RFC 7396 Merge Patch)
//!
//! # Manifest format (mod.toml)
//!
//! ```toml
//! name = "better-star-destroyers"
//! version = "1.2.0"
//! author = "ObiWanModder"
//! description = "Rebalances Imperial capital ships."
//!
//! [dependencies]
//! "rebel-units" = ">=1.0.0"
//! ```
//!
//! # Overlay files
//!
//! Each JSON file in the mod directory patches one entity category.
//! The filename maps to a world arena (e.g. `capital_ships.json`).
//! The file contains an array of patch objects. Each patch object must have
//! an `"id"` field matching the entity's `dat_id` numeric value.
//! All other fields are merged via RFC 7396 Merge Patch:
//!   - `null` values delete the field
//!   - present values overwrite
//!   - absent fields are preserved unchanged
//!
//! ```json
//! [
//!   { "id": 5, "hull": 2500, "shield_strength": 1800 },
//!   { "id": 12, "is_alliance": true }
//! ]
//! ```
//!
//! # Load order
//!
//! Mods are sorted topologically by their declared `[dependencies]`.
//! A mod may only override entities that were already loaded by the base game
//! or by a previously-loaded mod.
//!
//! # Hot reload (native only)
//!
//! On non-WASM targets, `ModWatcher` wraps a `notify::RecommendedWatcher` that
//! watches the mods directory for file-system events and signals when a reload
//! is needed. Call `ModWatcher::changed()` each frame to check.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// ─────────────────────────────────────────────────────────────────────────────
// Manifest
// ─────────────────────────────────────────────────────────────────────────────

/// Parsed `mod.toml` manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModManifest {
    /// Unique machine-readable name (kebab-case, e.g. `"better-star-destroyers"`).
    pub name: String,
    /// Semver version string, e.g. `"1.2.0"`.
    pub version: String,
    /// Author display name.
    #[serde(default)]
    pub author: String,
    /// Short human-readable description.
    #[serde(default)]
    pub description: String,
    /// Dependency map: mod name → semver requirement string (e.g. `">=1.0.0"`).
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    /// Absolute path to the mod's directory (set during discovery, not from TOML).
    #[serde(skip)]
    pub path: PathBuf,
}

impl ModManifest {
    /// Parse a manifest from a `mod.toml` file.
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("reading {}", path.display()))?;
        let mut manifest: ModManifest = toml::from_str(&text)
            .with_context(|| format!("parsing TOML in {}", path.display()))?;
        // Store the containing directory (not the manifest file path itself).
        manifest.path = path
            .parent()
            .unwrap_or(Path::new("."))
            .to_path_buf();
        Ok(manifest)
    }

    /// Parse the version field as a `semver::Version`.
    pub fn semver_version(&self) -> anyhow::Result<semver::Version> {
        semver::Version::parse(&self.version)
            .with_context(|| format!("mod '{}' has invalid version '{}'", self.name, self.version))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Content (overlay data from JSON files)
// ─────────────────────────────────────────────────────────────────────────────

/// The parsed overlay content from one mod's JSON files.
///
/// `patches` maps entity type name (the JSON filename stem) to a vec of
/// patch objects. Each patch object is a JSON `Value::Object` that must
/// contain an `"id"` field identifying the target entity.
#[derive(Debug, Default)]
pub struct ModContent {
    pub patches: HashMap<String, Vec<Value>>,
}

impl ModContent {
    /// Load all `*.json` overlay files from the mod directory.
    pub fn from_dir(dir: &Path) -> anyhow::Result<Self> {
        let mut content = ModContent::default();
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(e) => bail!("cannot read mod directory {}: {}", dir.display(), e),
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            if stem.is_empty() {
                continue;
            }
            let text = std::fs::read_to_string(&path)
                .with_context(|| format!("reading mod overlay {}", path.display()))?;
            let array: Vec<Value> = serde_json::from_str(&text)
                .with_context(|| format!("parsing JSON in {}", path.display()))?;
            content.patches.insert(stem, array);
        }
        Ok(content)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Loader
// ─────────────────────────────────────────────────────────────────────────────

/// The top-level mod loader.
pub struct ModLoader;

impl ModLoader {
    /// Scan `mods_dir` for subdirectories containing `mod.toml`.
    ///
    /// Returns an unsorted list of discovered manifests.
    pub fn discover(mods_dir: &Path) -> anyhow::Result<Vec<ModManifest>> {
        let mut manifests = Vec::new();
        if !mods_dir.exists() {
            return Ok(manifests);
        }
        let entries = std::fs::read_dir(mods_dir)
            .with_context(|| format!("scanning mods directory {}", mods_dir.display()))?;
        for entry in entries.flatten() {
            let meta = entry.metadata();
            if meta.map(|m| m.is_dir()).unwrap_or(false) {
                let manifest_path = entry.path().join("mod.toml");
                if manifest_path.exists() {
                    match ModManifest::from_file(&manifest_path) {
                        Ok(m) => manifests.push(m),
                        Err(e) => {
                            eprintln!("[mod-loader] skipping {}: {}", manifest_path.display(), e);
                        }
                    }
                }
            }
        }
        Ok(manifests)
    }

    /// Resolve a topological load order for the given manifests.
    ///
    /// Validates that:
    /// - All declared dependencies are present in the input list.
    /// - Installed versions satisfy declared semver requirements.
    /// - There are no dependency cycles.
    ///
    /// Returns manifests in dependency-first order (a mod's dependencies always
    /// appear before the mod itself in the returned vec).
    pub fn resolve_load_order(manifests: Vec<ModManifest>) -> anyhow::Result<Vec<ModManifest>> {
        // Build name → index and name → version maps for O(1) lookup.
        let mut name_to_idx: HashMap<&str, usize> = HashMap::new();
        for (i, m) in manifests.iter().enumerate() {
            if name_to_idx.insert(m.name.as_str(), i).is_some() {
                bail!("duplicate mod name '{}'", m.name);
            }
        }

        // Validate dependencies and collect edge list.
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); manifests.len()]; // adj[i] = indices that i depends on
        for (i, manifest) in manifests.iter().enumerate() {
            for (dep_name, req_str) in &manifest.dependencies {
                let req = semver::VersionReq::parse(req_str).with_context(|| {
                    format!(
                        "mod '{}' has invalid semver requirement '{}' for dependency '{}'",
                        manifest.name, req_str, dep_name
                    )
                })?;
                let dep_idx = match name_to_idx.get(dep_name.as_str()) {
                    Some(&idx) => idx,
                    None => bail!(
                        "mod '{}' depends on '{}' which is not installed",
                        manifest.name, dep_name
                    ),
                };
                let dep_version = manifests[dep_idx].semver_version()?;
                if !req.matches(&dep_version) {
                    bail!(
                        "mod '{}' requires '{}@{}' but installed version is '{}'",
                        manifest.name, dep_name, req_str, dep_version
                    );
                }
                adj[i].push(dep_idx);
            }
        }

        // Kahn's algorithm for topological sort (cycle detection included).
        let n = manifests.len();
        let mut in_degree = vec![0usize; n];
        // Build reverse edges: dep → dependents
        let mut rev_adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (i, deps) in adj.iter().enumerate() {
            for &dep in deps {
                rev_adj[dep].push(i);
                in_degree[i] += 1;
            }
        }

        let mut queue: Vec<usize> = (0..n).filter(|&i| in_degree[i] == 0).collect();
        let mut order: Vec<usize> = Vec::with_capacity(n);

        while let Some(node) = queue.pop() {
            order.push(node);
            for &dependent in &rev_adj[node] {
                in_degree[dependent] -= 1;
                if in_degree[dependent] == 0 {
                    queue.push(dependent);
                }
            }
        }

        if order.len() != n {
            bail!("mod dependency cycle detected — cannot resolve load order");
        }

        // Convert indices back to manifests (consume the vec).
        let mut indexed: Vec<Option<ModManifest>> = manifests.into_iter().map(Some).collect();
        Ok(order.into_iter().map(|i| indexed[i].take().unwrap()).collect())
    }

    /// Apply mod overlays to a serialized world represented as a `serde_json::Value`.
    ///
    /// The `world_json` must be a JSON object whose top-level keys match the
    /// entity type names used in mod overlay filenames (e.g. `"capital_ships"`
    /// maps to the `capital_ships` field of the serialized `GameWorld`).
    ///
    /// Each patch array targets entities by matching the `"id"` field (which
    /// corresponds to `dat_id.raw()` in world types). Fields are merged via
    /// RFC 7396 Merge Patch: null values remove keys, present values overwrite,
    /// absent fields are preserved.
    pub fn apply(world_json: &mut Value, content: &ModContent) -> anyhow::Result<()> {
        for (entity_type, patches) in &content.patches {
            let arena = match world_json.get_mut(entity_type) {
                Some(v) => v,
                None => {
                    eprintln!("[mod-loader] overlay '{}' targets unknown arena — skipping", entity_type);
                    continue;
                }
            };

            // The arena serializes as an object (slotmap serializes as {"slot_key": entity_obj}).
            // We need to find values whose "id" / "dat_id" matches the patch target.
            let arena_obj = match arena.as_object_mut() {
                Some(o) => o,
                None => {
                    eprintln!("[mod-loader] arena '{}' is not a JSON object — skipping", entity_type);
                    continue;
                }
            };

            for patch in patches {
                // Patches must be objects with an "id" field.
                let patch_obj = match patch.as_object() {
                    Some(o) => o,
                    None => {
                        eprintln!("[mod-loader] patch in '{}' is not a JSON object — skipping", entity_type);
                        continue;
                    }
                };
                let target_id = match patch_obj.get("id").and_then(|v| v.as_u64()) {
                    Some(id) => id,
                    None => {
                        eprintln!("[mod-loader] patch in '{}' missing numeric 'id' field — skipping", entity_type);
                        continue;
                    }
                };

                // Find the matching entity in the arena by dat_id.
                let mut matched = false;
                for entity_val in arena_obj.values_mut() {
                    let entity_id = entity_val
                        .get("dat_id")
                        .and_then(|v| v.as_object())
                        .and_then(|o| o.get("id"))
                        .and_then(|v| v.as_u64())
                        .or_else(|| {
                            // Fallback: direct "id" field at top level.
                            entity_val.get("id").and_then(|v| v.as_u64())
                        });

                    if entity_id == Some(target_id) {
                        merge_patch(entity_val, patch);
                        matched = true;
                        break;
                    }
                }

                if !matched {
                    eprintln!(
                        "[mod-loader] patch in '{}' targets id={} which was not found — skipping",
                        entity_type, target_id
                    );
                }
            }
        }
        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// RFC 7396 Merge Patch
// ─────────────────────────────────────────────────────────────────────────────

/// Apply an RFC 7396 JSON Merge Patch to `target`.
///
/// Rules:
/// - If `patch` is not an object, replace `target` with `patch` entirely.
/// - For each key in `patch`:
///   - If the value is `null`, remove that key from `target`.
///   - Otherwise, recursively merge into `target[key]`.
pub fn merge_patch(target: &mut Value, patch: &Value) {
    match patch {
        Value::Object(patch_map) => {
            // Ensure target is an object so we can merge into it.
            if !target.is_object() {
                *target = Value::Object(serde_json::Map::new());
            }
            let target_map = target.as_object_mut().unwrap();
            for (key, patch_val) in patch_map {
                if patch_val.is_null() {
                    target_map.remove(key);
                } else {
                    let entry = target_map
                        .entry(key.clone())
                        .or_insert(Value::Null);
                    merge_patch(entry, patch_val);
                }
            }
        }
        _ => {
            // Non-object patch replaces target wholesale.
            *target = patch.clone();
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Hot reload (native only)
// ─────────────────────────────────────────────────────────────────────────────

/// A file-system watcher for the mods directory.
///
/// Only available on non-WASM targets. Use `changed()` to poll for events.
#[cfg(not(target_arch = "wasm32"))]
pub struct ModWatcher {
    _watcher: notify::RecommendedWatcher,
    receiver: std::sync::mpsc::Receiver<Result<notify::Event, notify::Error>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl ModWatcher {
    /// Begin watching `mods_dir` for file-system changes.
    pub fn new(mods_dir: &Path) -> anyhow::Result<Self> {
        use notify::Watcher;
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = notify::RecommendedWatcher::new(
            move |event| {
                let _ = tx.send(event);
            },
            notify::Config::default(),
        )
        .context("creating file watcher")?;

        watcher
            .watch(mods_dir, notify::RecursiveMode::Recursive)
            .with_context(|| format!("watching mods directory {}", mods_dir.display()))?;

        Ok(Self { _watcher: watcher, receiver: rx })
    }

    /// Returns `true` if any relevant file-system event has occurred since the
    /// last call to `changed()`. Drains all pending events.
    pub fn changed(&self) -> bool {
        let mut any = false;
        // Drain all pending messages without blocking.
        while let Ok(event) = self.receiver.try_recv() {
            match event {
                Ok(e) => {
                    // Only react to modifications and creations — not access events.
                    use notify::EventKind::*;
                    match e.kind {
                        Modify(_) | Create(_) | Remove(_) => any = true,
                        _ => {}
                    }
                }
                Err(e) => eprintln!("[mod-watcher] watch error: {}", e),
            }
        }
        any
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // ── merge_patch ──────────────────────────────────────────────────────────

    #[test]
    fn merge_patch_overwrites_field() {
        let mut target = json!({ "hull": 1000, "name": "Star Destroyer" });
        let patch = json!({ "hull": 2500 });
        merge_patch(&mut target, &patch);
        assert_eq!(target["hull"], 2500);
        assert_eq!(target["name"], "Star Destroyer");
    }

    #[test]
    fn merge_patch_null_deletes_field() {
        let mut target = json!({ "hull": 1000, "shield_strength": 500 });
        let patch = json!({ "shield_strength": null });
        merge_patch(&mut target, &patch);
        assert!(target.get("shield_strength").is_none());
        assert_eq!(target["hull"], 1000);
    }

    #[test]
    fn merge_patch_adds_new_field() {
        let mut target = json!({ "name": "X-Wing" });
        let patch = json!({ "squadron_size": 12 });
        merge_patch(&mut target, &patch);
        assert_eq!(target["squadron_size"], 12);
    }

    #[test]
    fn merge_patch_nested_object() {
        let mut target = json!({ "skills": { "diplomacy": 5, "combat": 3 } });
        let patch = json!({ "skills": { "combat": 8 } });
        merge_patch(&mut target, &patch);
        assert_eq!(target["skills"]["diplomacy"], 5);
        assert_eq!(target["skills"]["combat"], 8);
    }

    #[test]
    fn merge_patch_non_object_replaces() {
        let mut target = json!(42);
        let patch = json!(99);
        merge_patch(&mut target, &patch);
        assert_eq!(target, json!(99));
    }

    // ── ModManifest TOML parsing ─────────────────────────────────────────────

    #[test]
    fn manifest_parses_basic_toml() {
        let toml_str = r#"
name = "test-mod"
version = "1.0.0"
author = "Tester"
description = "A test mod"

[dependencies]
"base-game" = ">=0.1.0"
"#;
        let manifest: ModManifest = toml::from_str(toml_str).unwrap();
        assert_eq!(manifest.name, "test-mod");
        assert_eq!(manifest.version, "1.0.0");
        assert_eq!(manifest.dependencies["base-game"], ">=0.1.0");
    }

    #[test]
    fn manifest_parses_minimal_toml() {
        let toml_str = r#"
name = "minimal"
version = "0.1.0"
"#;
        let manifest: ModManifest = toml::from_str(toml_str).unwrap();
        assert_eq!(manifest.name, "minimal");
        assert!(manifest.dependencies.is_empty());
    }

    // ── resolve_load_order ───────────────────────────────────────────────────

    fn make_manifest(name: &str, version: &str, deps: &[(&str, &str)]) -> ModManifest {
        ModManifest {
            name: name.to_string(),
            version: version.to_string(),
            author: String::new(),
            description: String::new(),
            dependencies: deps
                .iter()
                .map(|(n, r)| (n.to_string(), r.to_string()))
                .collect(),
            path: PathBuf::new(),
        }
    }

    #[test]
    fn load_order_no_deps() {
        let mods = vec![
            make_manifest("mod-a", "1.0.0", &[]),
            make_manifest("mod-b", "1.0.0", &[]),
        ];
        let order = ModLoader::resolve_load_order(mods).unwrap();
        assert_eq!(order.len(), 2);
    }

    #[test]
    fn load_order_simple_chain() {
        // b depends on a — a must come first.
        let mods = vec![
            make_manifest("mod-b", "1.0.0", &[("mod-a", ">=1.0.0")]),
            make_manifest("mod-a", "1.0.0", &[]),
        ];
        let order = ModLoader::resolve_load_order(mods).unwrap();
        assert_eq!(order.len(), 2);
        let names: Vec<&str> = order.iter().map(|m| m.name.as_str()).collect();
        let a_pos = names.iter().position(|&n| n == "mod-a").unwrap();
        let b_pos = names.iter().position(|&n| n == "mod-b").unwrap();
        assert!(a_pos < b_pos, "mod-a must load before mod-b");
    }

    #[test]
    fn load_order_missing_dep_errors() {
        let mods = vec![
            make_manifest("mod-b", "1.0.0", &[("mod-missing", ">=1.0.0")]),
        ];
        assert!(ModLoader::resolve_load_order(mods).is_err());
    }

    #[test]
    fn load_order_version_mismatch_errors() {
        let mods = vec![
            make_manifest("mod-a", "0.5.0", &[]),
            make_manifest("mod-b", "1.0.0", &[("mod-a", ">=1.0.0")]),
        ];
        assert!(ModLoader::resolve_load_order(mods).is_err());
    }

    #[test]
    fn load_order_cycle_errors() {
        let mods = vec![
            make_manifest("mod-a", "1.0.0", &[("mod-b", ">=1.0.0")]),
            make_manifest("mod-b", "1.0.0", &[("mod-a", ">=1.0.0")]),
        ];
        assert!(ModLoader::resolve_load_order(mods).is_err());
    }

    // ── ModLoader::apply ─────────────────────────────────────────────────────

    #[test]
    fn apply_patches_matching_entity() {
        // Minimal world JSON mimicking a slotmap arena serialization.
        let mut world = json!({
            "capital_ships": {
                "1v1": { "dat_id": { "id": 1 }, "hull": 1000, "name": "Star Destroyer" },
                "2v1": { "dat_id": { "id": 2 }, "hull": 500, "name": "Corellian Corvette" }
            }
        });
        let mut content = ModContent::default();
        content.patches.insert(
            "capital_ships".to_string(),
            vec![json!({ "id": 1, "hull": 2000 })],
        );
        ModLoader::apply(&mut world, &content).unwrap();
        assert_eq!(world["capital_ships"]["1v1"]["hull"], 2000);
        assert_eq!(world["capital_ships"]["2v1"]["hull"], 500); // untouched
    }

    #[test]
    fn apply_skips_unknown_arena() {
        let mut world = json!({ "systems": {} });
        let mut content = ModContent::default();
        content.patches.insert(
            "nonexistent_arena".to_string(),
            vec![json!({ "id": 1, "name": "test" })],
        );
        // Should not error — unknown arenas are logged and skipped.
        ModLoader::apply(&mut world, &content).unwrap();
    }

    #[test]
    fn apply_null_field_removes_it() {
        let mut world = json!({
            "characters": {
                "1v1": { "dat_id": { "id": 5 }, "name": "Luke", "jedi_probability": 80 }
            }
        });
        let mut content = ModContent::default();
        content.patches.insert(
            "characters".to_string(),
            vec![json!({ "id": 5, "jedi_probability": null })],
        );
        ModLoader::apply(&mut world, &content).unwrap();
        assert!(world["characters"]["1v1"].get("jedi_probability").is_none());
        assert_eq!(world["characters"]["1v1"]["name"], "Luke");
    }
}
