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
//! Mods are sorted topologically by their declared `[dependencies]`, with a
//! lexicographic name tie-break whenever multiple unrelated mods are ready.
//! A mod may only override entities that were already loaded by the base game
//! or by a previously-loaded mod.
//!
//! # Hot reload (native only)
//!
//! On non-WASM targets, `ModWatcher` wraps a `notify::RecommendedWatcher` that
//! watches the mods directory for file-system events and signals when a reload
//! is needed. Call `ModWatcher::changed()` each frame to check.

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

#[cfg(test)]
use std::cell::Cell;

#[cfg(any(target_os = "linux", target_os = "android"))]
use std::fs::{File, OpenOptions};
#[cfg(any(target_os = "linux", target_os = "android"))]
use std::io::Read;
#[cfg(any(target_os = "linux", target_os = "android"))]
use std::os::fd::AsRawFd;
#[cfg(any(target_os = "linux", target_os = "android"))]
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};

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
    /// Whether this mod is currently enabled. Not stored in mod.toml —
    /// managed by `ModConfig`.
    #[serde(skip)]
    pub enabled: bool,
}

impl ModManifest {
    /// Parse a manifest from a `mod.toml` file.
    #[cfg(not(target_arch = "wasm32"))]
    ///
    /// # Errors
    /// Returns an error if the manifest cannot be read or parsed.
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let text =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let mut manifest: ModManifest =
            toml::from_str(&text).with_context(|| format!("parsing TOML in {}", path.display()))?;
        // Store the containing directory (not the manifest file path itself).
        manifest.path = path.parent().unwrap_or(Path::new(".")).to_path_buf();
        Ok(manifest)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn from_file(_path: &Path) -> anyhow::Result<Self> {
        anyhow::bail!("mod loading from filesystem not supported on WASM")
    }

    /// Parse the version field as a `semver::Version`.
    ///
    /// # Errors
    /// Returns an error if the manifest version is not valid semantic version syntax.
    pub fn semver_version(&self) -> anyhow::Result<semver::Version> {
        semver::Version::parse(&self.version)
            .with_context(|| format!("mod '{}' has invalid version '{}'", self.name, self.version))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Mod config (persisted enable/disable state)
// ─────────────────────────────────────────────────────────────────────────────

/// Persisted mod enable/disable state. Stored in `mods/config.toml`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModConfig {
    /// Mod names that are currently enabled.
    pub enabled: Vec<String>,
}

impl ModConfig {
    #[must_use]
    pub fn load(mods_dir: &Path) -> Self {
        let path = mods_dir.join("config.toml");
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default()
    }

    ///
    /// # Errors
    /// Returns an error if the mod directory or load-order file cannot be written.
    pub fn save(&self, mods_dir: &Path) -> anyhow::Result<()> {
        let path = mods_dir.join("config.toml");
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    #[must_use]
    pub fn is_enabled(&self, name: &str) -> bool {
        self.enabled.iter().any(|n| n == name)
    }

    pub fn toggle(&mut self, name: &str) {
        if self.is_enabled(name) {
            self.enabled.retain(|n| n != name);
        } else {
            self.enabled.push(name.to_string());
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Content (overlay data from JSON files)
// ─────────────────────────────────────────────────────────────────────────────

/// Reserved root filename for the encyclopedia content overlay.
pub const ENCYCLOPEDIA_MOD_FILENAME: &str = "encyclopedia.json";

/// Maximum retained diagnostic message bytes emitted by the confined target
/// reader. The separately stored target path is not repeated in the message.
pub const ENCYCLOPEDIA_READ_ERROR_MESSAGE_BYTES_LIMIT: usize = 256;

#[cfg(test)]
thread_local! {
    static ENCYCLOPEDIA_TARGET_READ_CALLS: Cell<u64> = const { Cell::new(0) };
}

#[cfg(test)]
fn reset_encyclopedia_target_read_calls() {
    ENCYCLOPEDIA_TARGET_READ_CALLS.set(0);
}

#[cfg(test)]
fn encyclopedia_target_read_calls() -> u64 {
    ENCYCLOPEDIA_TARGET_READ_CALLS.get()
}

/// Raw encyclopedia input discovered beside a mod's world overlays.
///
/// This layer deliberately does not parse the bytes. The presence-aware
/// encyclopedia parser owns that later step, including malformed-content
/// diagnostics. A read failure is retained independently so it cannot discard
/// otherwise valid world patches from the same mod.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum ModContentTarget {
    /// The reserved root file is absent.
    #[default]
    Missing,
    /// Exact bytes read from the reserved root file.
    Bytes(Vec<u8>),
    /// The reserved root path exists but could not be read.
    ReadError {
        /// Exact path which failed.
        path: PathBuf,
        /// Platform-independent I/O error category.
        kind: std::io::ErrorKind,
        /// Contextual diagnostic suitable for the future mod diagnostics path.
        message: String,
    },
}

/// Reads only the reserved encyclopedia target through a pinned, no-follow
/// root/file handle. The caller's admission callback runs after the bounded
/// regular-file length is known and before the byte buffer is allocated.
///
/// I/O and confinement failures remain a present [`ModContentTarget::ReadError`]
/// so they cannot be confused with an absent target. A candidate-budget
/// failure is returned directly in the caller's error type.
#[cfg(not(target_arch = "wasm32"))]
pub fn read_encyclopedia_target_with_admission<E>(
    dir: &Path,
    admit: impl FnMut(u64) -> Result<(), E>,
) -> Result<ModContentTarget, E> {
    #[cfg(test)]
    ENCYCLOPEDIA_TARGET_READ_CALLS.set(
        ENCYCLOPEDIA_TARGET_READ_CALLS
            .get()
            .checked_add(1)
            .expect("a finite test cannot overflow the target-read counter"),
    );
    read_encyclopedia_target_platform(dir, admit)
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn read_encyclopedia_target_platform<E>(
    dir: &Path,
    mut admit: impl FnMut(u64) -> Result<(), E>,
) -> Result<ModContentTarget, E> {
    let target = dir.join(ENCYCLOPEDIA_MOD_FILENAME);
    let root = match open_mod_directory_nofollow(dir) {
        Ok(root) => root,
        Err(error) => return Ok(target_read_error(&target, error, "pinning mod root")),
    };
    let pinned_path = proc_fd_child(&root, std::ffi::OsStr::new(ENCYCLOPEDIA_MOD_FILENAME));
    let pinned = match open_mod_path_nofollow(&pinned_path) {
        Ok(pinned) => pinned,
        Err(error) => return Ok(classify_target_open_error(&target, error)),
    };
    let metadata = match pinned.metadata() {
        Ok(metadata) => metadata,
        Err(error) => {
            return Ok(target_read_error(
                &target,
                error,
                "inspecting content target",
            ))
        }
    };
    if !metadata.is_file() {
        return Ok(content_target_read_error(
            target,
            if metadata.is_dir() {
                std::io::ErrorKind::IsADirectory
            } else {
                std::io::ErrorKind::InvalidInput
            },
            "reading mod content target: reserved target is not a regular file".to_owned(),
        ));
    }
    let length = metadata.len();
    if length > crate::encyclopedia::OVERLAY_JSON_BYTES_LIMIT as u64 {
        return Ok(content_target_read_error(
            target,
            std::io::ErrorKind::InvalidData,
            format!(
                "resource_limit:json_bytes: reserved encyclopedia target length {length} exceeds {}",
                crate::encyclopedia::OVERLAY_JSON_BYTES_LIMIT
            ),
        ));
    }
    let length_usize = match usize::try_from(length) {
        Ok(length) => length,
        Err(_) => {
            return Ok(content_target_read_error(
                target,
                std::io::ErrorKind::InvalidData,
                "resource_limit:json_bytes: target length does not fit address space".to_owned(),
            ));
        }
    };
    admit(length)?;

    let readable_path = proc_fd_path(&pinned);
    let mut readable = match OpenOptions::new().read(true).open(&readable_path) {
        Ok(readable) => readable,
        Err(error) => {
            return Ok(target_read_error(
                &target,
                error,
                "opening pinned content target",
            ))
        }
    };
    let opened_metadata = match readable.metadata() {
        Ok(metadata) => metadata,
        Err(error) => {
            return Ok(target_read_error(
                &target,
                error,
                "verifying content target",
            ))
        }
    };
    if !same_file_identity(&metadata, &opened_metadata) {
        return Ok(content_target_read_error(
            target,
            std::io::ErrorKind::InvalidData,
            "reserved encyclopedia target identity changed before read".to_owned(),
        ));
    }

    let mut bytes = vec![0_u8; length_usize];
    if let Err(error) = readable.read_exact(&mut bytes) {
        return Ok(target_read_error(
            &target,
            error,
            "reading pinned content target",
        ));
    }
    let mut extra = [0_u8; 1];
    match readable.read(&mut extra) {
        Ok(0) => {}
        Ok(_) => {
            return Ok(content_target_read_error(
                target,
                std::io::ErrorKind::InvalidData,
                "reserved encyclopedia target grew during bounded read".to_owned(),
            ));
        }
        Err(error) => {
            return Ok(target_read_error(
                &target,
                error,
                "finishing bounded content read",
            ))
        }
    }
    let final_metadata = match readable.metadata() {
        Ok(metadata) => metadata,
        Err(error) => {
            return Ok(target_read_error(
                &target,
                error,
                "rechecking content target",
            ))
        }
    };
    if !same_file_identity(&metadata, &final_metadata) {
        return Ok(content_target_read_error(
            target,
            std::io::ErrorKind::InvalidData,
            "reserved encyclopedia target identity changed during read".to_owned(),
        ));
    }
    Ok(ModContentTarget::Bytes(bytes))
}

#[cfg(all(
    not(target_arch = "wasm32"),
    not(any(target_os = "linux", target_os = "android"))
))]
fn read_encyclopedia_target_platform<E>(
    dir: &Path,
    _admit: impl FnMut(u64) -> Result<(), E>,
) -> Result<ModContentTarget, E> {
    let target = dir.join(ENCYCLOPEDIA_MOD_FILENAME);
    match std::fs::symlink_metadata(&target) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Ok(ModContentTarget::Missing)
        }
        Err(error) => Ok(target_read_error(&target, error, "inspecting content target")),
        Ok(_) => Ok(content_target_read_error(
            target,
            std::io::ErrorKind::Unsupported,
            "secure_confinement_unavailable: pinned no-follow content reads require Linux or Android"
                .to_owned(),
        )),
    }
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn proc_fd_path(file: &File) -> PathBuf {
    PathBuf::from(format!("/proc/self/fd/{}", file.as_raw_fd()))
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn proc_fd_child(file: &File, component: &std::ffi::OsStr) -> PathBuf {
    let mut path = proc_fd_path(file);
    path.push(component);
    path
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn open_mod_directory_nofollow(path: &Path) -> std::io::Result<File> {
    const O_NOFOLLOW: i32 = 0o400000;
    const O_DIRECTORY: i32 = 0o200000;
    const O_CLOEXEC: i32 = 0o2000000;
    OpenOptions::new()
        .read(true)
        .custom_flags(O_NOFOLLOW | O_DIRECTORY | O_CLOEXEC)
        .open(path)
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn open_mod_path_nofollow(path: &Path) -> std::io::Result<File> {
    const O_NOFOLLOW: i32 = 0o400000;
    const O_CLOEXEC: i32 = 0o2000000;
    const O_PATH: i32 = 0o10000000;
    OpenOptions::new()
        .read(true)
        .custom_flags(O_NOFOLLOW | O_CLOEXEC | O_PATH)
        .open(path)
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn same_file_identity(before: &std::fs::Metadata, after: &std::fs::Metadata) -> bool {
    before.dev() == after.dev() && before.ino() == after.ino() && before.len() == after.len()
}

#[cfg(not(target_arch = "wasm32"))]
fn target_read_error(path: &Path, error: std::io::Error, action: &str) -> ModContentTarget {
    let kind = error.kind();
    let message = error.raw_os_error().map_or_else(
        || format!("{action}: {kind:?}"),
        |code| format!("{action}: {kind:?} (os error {code})"),
    );
    content_target_read_error(path.to_path_buf(), kind, message)
}

#[cfg(not(target_arch = "wasm32"))]
fn content_target_read_error(
    path: PathBuf,
    kind: std::io::ErrorKind,
    message: String,
) -> ModContentTarget {
    let message = if message.len() <= ENCYCLOPEDIA_READ_ERROR_MESSAGE_BYTES_LIMIT {
        message
    } else {
        "confined target diagnostic exceeded its admitted byte envelope".to_owned()
    };
    ModContentTarget::ReadError {
        path,
        kind,
        message,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn classify_target_open_error(path: &Path, error: std::io::Error) -> ModContentTarget {
    if error.kind() == std::io::ErrorKind::NotFound {
        ModContentTarget::Missing
    } else {
        target_read_error(path, error, "pinning content target")
    }
}

/// The parsed world overlay content and separate encyclopedia target from one
/// mod's root files.
///
/// `patches` maps entity type name (the JSON filename stem) to a vec of
/// patch objects. Each patch object is a JSON `Value::Object` that must
/// contain an `"id"` field identifying the target entity. The exact root
/// [`ENCYCLOPEDIA_MOD_FILENAME`] is never inserted into this map.
#[derive(Debug, Default)]
pub struct ModContent {
    pub patches: HashMap<String, Vec<Value>>,
    /// Raw reserved content target, kept outside world patch parsing.
    pub encyclopedia: ModContentTarget,
}

impl ModContent {
    /// Load all `*.json` overlay files from the mod directory.
    #[cfg(not(target_arch = "wasm32"))]
    ///
    /// # Errors
    /// Returns an error if a present content file cannot be read or parsed.
    pub fn from_dir(dir: &Path) -> anyhow::Result<Self> {
        let encyclopedia =
            read_encyclopedia_target_with_admission(
                dir,
                |_| Ok::<(), std::convert::Infallible>(()),
            )
            .expect("the public content-loader admission callback is infallible");
        Self::from_dir_with_encyclopedia(dir, encyclopedia)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn from_dir_world_only(dir: &Path) -> anyhow::Result<Self> {
        Self::from_dir_with_encyclopedia(dir, ModContentTarget::Missing)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn from_dir_with_encyclopedia(
        dir: &Path,
        encyclopedia: ModContentTarget,
    ) -> anyhow::Result<Self> {
        let mut content = ModContent {
            encyclopedia,
            ..Self::default()
        };
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(e) => bail!("cannot read mod directory {}: {}", dir.display(), e),
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.file_name().and_then(|name| name.to_str()) == Some(ENCYCLOPEDIA_MOD_FILENAME) {
                continue;
            }
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

    #[cfg(target_arch = "wasm32")]
    pub fn from_dir(_dir: &Path) -> anyhow::Result<Self> {
        anyhow::bail!("mod content loading from filesystem not supported on WASM")
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
    #[cfg(not(target_arch = "wasm32"))]
    ///
    /// # Errors
    /// Returns an error if the mod directory cannot be read or an installed manifest
    /// cannot be loaded.
    pub fn discover(mods_dir: &Path) -> anyhow::Result<Vec<ModManifest>> {
        let mut manifests = Vec::new();
        if !mods_dir.exists() {
            return Ok(manifests);
        }
        let entries = std::fs::read_dir(mods_dir)
            .with_context(|| format!("scanning mods directory {}", mods_dir.display()))?;
        for entry in entries.flatten() {
            let meta = entry.metadata();
            if meta.is_ok_and(|m| m.is_dir()) {
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

    /// On WASM, mod discovery from the filesystem is not supported.
    /// Returns an empty list — mods must be loaded via other means (not yet implemented).
    #[cfg(target_arch = "wasm32")]
    pub fn discover(_mods_dir: &Path) -> anyhow::Result<Vec<ModManifest>> {
        Ok(Vec::new())
    }

    /// Resolve a topological load order for the given manifests.
    ///
    /// Validates that:
    /// - All declared dependencies are present in the input list.
    /// - Installed versions satisfy declared semver requirements.
    /// - There are no dependency cycles.
    ///
    /// Returns manifests in dependency-first order (a mod's dependencies always
    /// appear before the mod itself in the returned vec). Ready mods that are
    /// unrelated are ordered lexicographically by name, independent of
    /// filesystem discovery order.
    ///
    /// # Errors
    /// Returns an error for invalid version requirements, missing or incompatible
    /// dependencies, or a dependency cycle.
    ///
    /// # Panics
    /// Panics if the topological order contains a duplicate index, violating its traversal invariant.
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
                let Some(&dep_idx) = name_to_idx.get(dep_name.as_str()) else {
                    bail!(
                        "mod '{}' depends on '{}' which is not installed",
                        manifest.name,
                        dep_name
                    )
                };
                let dep_version = manifests[dep_idx].semver_version()?;
                if !req.matches(&dep_version) {
                    bail!(
                        "mod '{}' requires '{}@{}' but installed version is '{}'",
                        manifest.name,
                        dep_name,
                        req_str,
                        dep_version
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

        let mut ready: BTreeSet<(&str, usize)> = (0..n)
            .filter(|&i| in_degree[i] == 0)
            .map(|i| (manifests[i].name.as_str(), i))
            .collect();
        let mut order: Vec<usize> = Vec::with_capacity(n);

        while let Some((_, node)) = ready.pop_first() {
            order.push(node);
            for &dependent in &rev_adj[node] {
                in_degree[dependent] -= 1;
                if in_degree[dependent] == 0 {
                    ready.insert((manifests[dependent].name.as_str(), dependent));
                }
            }
        }

        if order.len() != n {
            bail!("mod dependency cycle detected — cannot resolve load order");
        }

        // Convert indices back to manifests (consume the vec).
        let mut indexed: Vec<Option<ModManifest>> = manifests.into_iter().map(Some).collect();
        Ok(order
            .into_iter()
            .map(|i| indexed[i].take().unwrap())
            .collect())
    }

    /// Apply mod overlays to a serialized world represented as a `serde_json::Value`.
    ///
    /// The `world_json` must be a JSON object whose top-level keys match the
    /// entity type names used in mod overlay filenames (e.g. `"capital_ship_classes"`
    /// maps to the `capital_ship_classes` field of the serialized `GameWorld`).
    ///
    /// Each patch array targets entities by matching the `"id"` selector. For
    /// entity arenas this corresponds to `dat_id.raw()`; for the wrapped
    /// GNPRTB/SDPRTB parameter tables it corresponds to `parameter_id`.
    /// Fields are merged via RFC 7396 Merge Patch: null values remove keys,
    /// present values overwrite, absent fields are preserved.
    ///
    /// # Errors
    /// Returns an error if the world or mod content cannot be patched in the
    /// expected JSON arena structure.
    ///
    /// # Panics
    /// Panics if a parameter-table entries array changes type after it has been validated.
    pub fn apply(world_json: &mut Value, content: &ModContent) -> anyhow::Result<()> {
        for (entity_type, patches) in &content.patches {
            let Some(arena) = world_json.get_mut(entity_type) else {
                eprintln!("[mod-loader] overlay '{entity_type}' targets unknown arena — skipping");
                continue;
            };

            for patch in patches {
                // Patches must be objects with an "id" field.
                let Some(patch_obj) = patch.as_object() else {
                    eprintln!(
                        "[mod-loader] patch in '{entity_type}' is not a JSON object — skipping"
                    );
                    continue;
                };
                let Some(target_id) = patch_obj.get("id").and_then(serde_json::Value::as_u64)
                else {
                    eprintln!(
                        "[mod-loader] patch in '{entity_type}' missing numeric 'id' field — skipping"
                    );
                    continue;
                };

                // Slotmap and HashMap arenas serialize as objects whose values are
                // entities. GNPRTB/SDPRTB serialize as {"entries": [...]}; their
                // entries use parameter_id rather than dat_id.
                let matched = if arena.get("entries").and_then(Value::as_array).is_some() {
                    let entries = arena
                        .get_mut("entries")
                        .and_then(Value::as_array_mut)
                        .expect("entries was verified as an array");
                    patch_matching_entity(entries.iter_mut(), target_id, patch)
                } else if let Some(arena_obj) = arena.as_object_mut() {
                    patch_matching_entity(arena_obj.values_mut(), target_id, patch)
                } else {
                    eprintln!("[mod-loader] arena '{entity_type}' is not a JSON object — skipping");
                    continue;
                };

                if !matched {
                    eprintln!(
                        "[mod-loader] patch in '{entity_type}' targets id={target_id} which was not found — skipping"
                    );
                }
            }
        }
        Ok(())
    }
}

fn patch_matching_entity<'a>(
    entities: impl Iterator<Item = &'a mut Value>,
    target_id: u64,
    patch: &Value,
) -> bool {
    for entity in entities {
        if entity_selector_id(entity) == Some(target_id) {
            // `id` selects the target and is not itself part of the serialized
            // GameWorld entity. Removing it also keeps parameter entries valid.
            let mut fields = patch.clone();
            if let Some(fields_obj) = fields.as_object_mut() {
                fields_obj.remove("id");
            }
            merge_patch(entity, &fields);
            return true;
        }
    }
    false
}

fn entity_selector_id(entity: &Value) -> Option<u64> {
    // DatId is a newtype tuple struct and normally serializes as a bare number.
    // Retain the object fallback for hand-crafted or legacy serialized worlds.
    entity
        .get("dat_id")
        .and_then(Value::as_u64)
        .or_else(|| {
            entity
                .get("dat_id")
                .and_then(Value::as_object)
                .and_then(|dat_id| dat_id.get("id"))
                .and_then(Value::as_u64)
        })
        .or_else(|| entity.get("id").and_then(Value::as_u64))
        .or_else(|| entity.get("parameter_id").and_then(Value::as_u64))
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
///
/// # Panics
/// Panics if the target is not an object after object initialization.
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
                    let entry = target_map.entry(key.clone()).or_insert(Value::Null);
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
    ///
    /// # Errors
    /// Returns an error if mod discovery, dependency resolution, or content loading fails.
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

        Ok(Self {
            _watcher: watcher,
            receiver: rx,
        })
    }

    /// Returns `true` if any relevant file-system event has occurred since the
    /// last call to `changed()`. Drains all pending events.
    #[must_use]
    pub fn changed(&self) -> bool {
        let mut any = false;
        // Drain all pending messages without blocking.
        while let Ok(event) = self.receiver.try_recv() {
            match event {
                Ok(e) => {
                    // Only react to modifications and creations — not access events.
                    use notify::EventKind::{Create, Modify, Remove};
                    match e.kind {
                        Modify(_) | Create(_) | Remove(_) => any = true,
                        _ => {}
                    }
                }
                Err(e) => eprintln!("[mod-watcher] watch error: {e}"),
            }
        }
        any
    }
}

/// WASM stub for `ModWatcher` — hot reload is not available in the browser.
#[cfg(target_arch = "wasm32")]
pub struct ModWatcher;

#[cfg(target_arch = "wasm32")]
impl ModWatcher {
    /// No-op on WASM — always returns `Ok`.
    pub fn new(_mods_dir: &Path) -> anyhow::Result<Self> {
        Ok(Self)
    }

    /// Always returns `false` on WASM — no filesystem events.
    pub fn changed(&self) -> bool {
        false
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Runtime (discovery + validation + application orchestrator)
// ─────────────────────────────────────────────────────────────────────────────

/// Structured errors from mod validation.
#[derive(Debug, Clone)]
pub enum ModError {
    MissingDependency {
        mod_name: String,
        dep_name: String,
    },
    VersionMismatch {
        mod_name: String,
        dep_name: String,
        required: String,
        found: String,
    },
    ParseError {
        mod_name: String,
        message: String,
    },
    /// The enabled set has no load order (a dependency cycle or a duplicate
    /// name), so no enabled mod loads.
    LoadOrder {
        mod_name: String,
        message: String,
    },
}

impl ModError {
    /// The mod this error belongs to; empty when no single mod is at fault.
    #[must_use]
    pub fn mod_name(&self) -> &str {
        match self {
            Self::MissingDependency { mod_name, .. }
            | Self::VersionMismatch { mod_name, .. }
            | Self::ParseError { mod_name, .. }
            | Self::LoadOrder { mod_name, .. } => mod_name,
        }
    }
}

impl std::fmt::Display for ModError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingDependency { dep_name, .. } => {
                write!(
                    f,
                    "requires '{dep_name}', which is not installed and enabled"
                )
            }
            Self::VersionMismatch {
                dep_name,
                required,
                found,
                ..
            } => write!(
                f,
                "requires '{dep_name}' {required}, but {found} is enabled"
            ),
            Self::ParseError { message, .. } => f.write_str(message),
            Self::LoadOrder { message, .. } => write!(f, "not loaded: {message}"),
        }
    }
}

/// Dependency problems among the enabled mods. A dependency that is installed
/// but disabled counts as missing, because only enabled mods load; while any
/// remain, `enabled_sorted` loads nothing.
fn dependency_errors(discovered: &[ModManifest]) -> Vec<ModError> {
    let mut errors = Vec::new();
    for manifest in discovered.iter().filter(|m| m.enabled) {
        for (dep_name, required) in &manifest.dependencies {
            let Some(dep) = discovered.iter().find(|m| m.enabled && &m.name == dep_name) else {
                errors.push(ModError::MissingDependency {
                    mod_name: manifest.name.clone(),
                    dep_name: dep_name.clone(),
                });
                continue;
            };
            // An unparseable requirement or version can never be satisfied.
            let matches = match (semver::VersionReq::parse(required), dep.semver_version()) {
                (Ok(req), Ok(version)) => req.matches(&version),
                _ => false,
            };
            if !matches {
                errors.push(ModError::VersionMismatch {
                    mod_name: manifest.name.clone(),
                    dep_name: dep_name.clone(),
                    required: required.clone(),
                    found: dep.version.clone(),
                });
            }
        }
    }
    if errors.is_empty() {
        let enabled: Vec<ModManifest> = discovered.iter().filter(|m| m.enabled).cloned().collect();
        let names: Vec<String> = enabled.iter().map(|m| m.name.clone()).collect();
        if let Err(e) = ModLoader::resolve_load_order(enabled) {
            errors.extend(names.into_iter().map(|mod_name| ModError::LoadOrder {
                mod_name,
                message: e.to_string(),
            }));
        }
    }
    errors
}

/// Runtime mod management: discovery, validation, and application.
///
/// Created once at startup, queried by the mod manager UI, and used
/// to apply enabled mods to the game world.
pub struct ModRuntime {
    /// All discovered mods (from scanning `mods_dir`).
    pub discovered: Vec<ModManifest>,
    /// Persisted enable/disable config.
    pub config: ModConfig,
    /// Structured errors from last validation pass.
    pub errors: Vec<ModError>,
    /// The mods directory path.
    pub mods_dir: PathBuf,
}

impl ModRuntime {
    /// Discover all mods in `mods_dir` and load config.
    #[cfg(not(target_arch = "wasm32"))]
    #[must_use]
    pub fn discover(mods_dir: &Path) -> Self {
        let config = ModConfig::load(mods_dir);
        let mut errors = Vec::new();
        let mut discovered = match ModLoader::discover(mods_dir) {
            Ok(manifests) => manifests,
            Err(e) => {
                errors.push(ModError::ParseError {
                    mod_name: String::new(),
                    message: e.to_string(),
                });
                Vec::new()
            }
        };

        // Set enabled flag from config.
        for manifest in &mut discovered {
            manifest.enabled = config.is_enabled(&manifest.name);
        }
        errors.extend(dependency_errors(&discovered));

        Self {
            discovered,
            config,
            errors,
            mods_dir: mods_dir.to_path_buf(),
        }
    }

    /// On WASM, mod discovery is not supported.
    #[cfg(target_arch = "wasm32")]
    pub fn discover(mods_dir: &Path) -> Self {
        Self {
            discovered: Vec::new(),
            config: ModConfig::default(),
            errors: Vec::new(),
            mods_dir: mods_dir.to_path_buf(),
        }
    }

    /// Return only enabled mods in dependency-sorted order.
    #[must_use]
    pub fn enabled_sorted(&self) -> Vec<&ModManifest> {
        let enabled: Vec<ModManifest> = self
            .discovered
            .iter()
            .filter(|m| m.enabled)
            .cloned()
            .collect();

        match ModLoader::resolve_load_order(enabled) {
            Ok(sorted) => {
                // Map sorted names back to references into `discovered`.
                let names: Vec<String> = sorted.iter().map(|m| m.name.clone()).collect();
                let mut refs: Vec<&ModManifest> = Vec::with_capacity(names.len());
                for name in &names {
                    if let Some(m) = self.discovered.iter().find(|m| &m.name == name) {
                        refs.push(m);
                    }
                }
                refs
            }
            Err(e) => {
                eprintln!("[mod-runtime] load order resolution failed: {e}");
                Vec::new()
            }
        }
    }

    /// Apply all enabled mods to the world (RFC 7396 merge patch).
    ///
    /// This is the existing-call-site wrapper: it resolves the enabled set
    /// once, then delegates that exact order to [`Self::apply_ordered`].
    #[cfg(not(target_arch = "wasm32"))]
    pub fn apply_enabled(&self, world: &mut rebellion_core::world::GameWorld) -> Vec<ModError> {
        let sorted = self.enabled_sorted();
        self.apply_ordered(world, &sorted)
    }

    /// Apply world overlays in the caller-supplied resolved order.
    ///
    /// Later overlays win. This method deliberately does not sort again: a
    /// caller that also updates non-world content can pass the same resolved
    /// order to both paths. Lexicographic ordering is only the deterministic
    /// tie-break for unrelated ready mods; authors who require override
    /// precedence must declare that relationship as a dependency instead of
    /// relying on historical filesystem discovery order.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn apply_ordered(
        &self,
        world: &mut rebellion_core::world::GameWorld,
        ordered: &[&ModManifest],
    ) -> Vec<ModError> {
        if ordered.is_empty() {
            return Vec::new();
        }

        // Serialize world to JSON for patching.
        let mut world_json = match serde_json::to_value(&*world) {
            Ok(v) => v,
            Err(e) => {
                return vec![ModError::ParseError {
                    mod_name: String::new(),
                    message: format!("failed to serialize world: {e}"),
                }];
            }
        };

        let mut errors = Vec::new();
        for manifest in ordered {
            let content = match ModContent::from_dir_world_only(&manifest.path) {
                Ok(c) => c,
                Err(e) => {
                    errors.push(ModError::ParseError {
                        mod_name: manifest.name.clone(),
                        message: e.to_string(),
                    });
                    continue;
                }
            };
            if let Err(e) = ModLoader::apply(&mut world_json, &content) {
                errors.push(ModError::ParseError {
                    mod_name: manifest.name.clone(),
                    message: e.to_string(),
                });
            }
        }

        // Deserialize patched JSON back into the world.
        match serde_json::from_value(world_json) {
            Ok(patched) => *world = patched,
            Err(e) => {
                errors.push(ModError::ParseError {
                    mod_name: String::new(),
                    message: format!("failed to deserialize patched world: {e}"),
                });
            }
        }

        errors
    }

    /// Replace dependency errors with ones computed from the current enabled set.
    fn refresh_dependency_errors(&mut self) {
        self.errors
            .retain(|e| matches!(e, ModError::ParseError { .. }));
        self.errors.extend(dependency_errors(&self.discovered));
    }

    /// Toggle a mod's enabled state and persist config.
    pub fn toggle_mod(&mut self, name: &str) {
        self.config.toggle(name);
        for m in &mut self.discovered {
            if m.name == name {
                m.enabled = self.config.is_enabled(name);
            }
        }
        self.refresh_dependency_errors();
        if let Err(e) = self.config.save(&self.mods_dir) {
            eprintln!("[mod-runtime] failed to save config: {e}");
        }
    }

    /// Check for filesystem changes and return true if mods need reloading.
    #[must_use]
    pub fn check_reload(&self, watcher: &ModWatcher) -> bool {
        watcher.changed()
    }

    /// Re-discover mods (after file change or toggle).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn refresh(&mut self) {
        let refreshed = Self::discover(&self.mods_dir);
        self.discovered = refreshed.discovered;
        self.errors = refreshed.errors;
        // Preserve the live config (it may have been toggled since last discover).
        for m in &mut self.discovered {
            m.enabled = self.config.is_enabled(&m.name);
        }
        self.refresh_dependency_errors();
    }

    /// WASM stub: apply_enabled (no filesystem access).
    #[cfg(target_arch = "wasm32")]
    pub fn apply_enabled(&self, _world: &mut rebellion_core::world::GameWorld) -> Vec<ModError> {
        Vec::new()
    }

    /// WASM stub: ordered filesystem mod application is unavailable.
    #[cfg(target_arch = "wasm32")]
    pub fn apply_ordered(
        &self,
        _world: &mut rebellion_core::world::GameWorld,
        _ordered: &[&ModManifest],
    ) -> Vec<ModError> {
        Vec::new()
    }

    /// WASM stub: refresh (no filesystem access).
    #[cfg(target_arch = "wasm32")]
    pub fn refresh(&mut self) {}

    /// Return (name, version) pairs for all enabled mods (for save metadata).
    #[must_use]
    pub fn enabled_mod_list(&self) -> Vec<(String, String)> {
        self.enabled_sorted()
            .iter()
            .map(|m| (m.name.clone(), m.version.clone()))
            .collect()
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
            enabled: false,
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
    fn unrelated_mods_resolve_lexicographically_regardless_of_discovery_order() {
        let forward = vec![
            make_manifest("alpha", "1.0.0", &[]),
            make_manifest("middle", "1.0.0", &[]),
            make_manifest("zulu", "1.0.0", &[]),
        ];
        let reverse = forward.iter().cloned().rev().collect();

        let names = |manifests| {
            ModLoader::resolve_load_order(manifests)
                .unwrap()
                .into_iter()
                .map(|manifest| manifest.name)
                .collect::<Vec<_>>()
        };

        assert_eq!(names(forward), ["alpha", "middle", "zulu"]);
        assert_eq!(names(reverse), ["alpha", "middle", "zulu"]);
    }

    #[test]
    fn declared_dependencies_load_before_lexically_earlier_dependents() {
        let mods = vec![
            make_manifest("alpha-dependent", "1.0.0", &[("zulu-base", "^1")]),
            make_manifest("zulu-base", "1.0.0", &[]),
        ];

        let names: Vec<_> = ModLoader::resolve_load_order(mods)
            .unwrap()
            .into_iter()
            .map(|manifest| manifest.name)
            .collect();

        assert_eq!(names, ["zulu-base", "alpha-dependent"]);
    }

    #[test]
    fn load_order_missing_dep_errors() {
        let mods = vec![make_manifest(
            "mod-b",
            "1.0.0",
            &[("mod-missing", ">=1.0.0")],
        )];
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

    #[test]
    fn apply_patches_wrapped_parameter_entries() {
        let mut world = json!({
            "gnprtb": {
                "entries": [
                    { "parameter_id": 3588, "alliance_sp_medium": 75 },
                    { "parameter_id": 3589, "alliance_sp_medium": 50 }
                ]
            },
            "sdprtb": {
                "entries": [
                    { "parameter_id": 7680, "multiplayer_alliance": 40 }
                ]
            }
        });
        let mut content = ModContent::default();
        content.patches.insert(
            "gnprtb".to_string(),
            vec![json!({ "id": 3588, "alliance_sp_medium": 90 })],
        );
        content.patches.insert(
            "sdprtb".to_string(),
            vec![json!({ "id": 7680, "multiplayer_alliance": 55 })],
        );

        ModLoader::apply(&mut world, &content).unwrap();

        assert_eq!(world["gnprtb"]["entries"][0]["alliance_sp_medium"], 90);
        assert_eq!(world["gnprtb"]["entries"][1]["alliance_sp_medium"], 50);
        assert_eq!(world["sdprtb"]["entries"][0]["multiplayer_alliance"], 55);
        assert!(world["gnprtb"]["entries"][0].get("id").is_none());
        assert_eq!(world["gnprtb"]["entries"][0]["parameter_id"], 3588);
    }

    #[test]
    fn malformed_encyclopedia_bytes_are_retained_while_world_patches_apply_once() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join("gnprtb.json"),
            r#"[{"id":77,"development":9}]"#,
        )
        .unwrap();
        let raw = b"\xff{malformed encyclopedia";
        std::fs::write(tmp.path().join("encyclopedia.json"), raw).unwrap();

        let content = ModContent::from_dir(tmp.path()).unwrap();
        assert!(matches!(
            &content.encyclopedia,
            ModContentTarget::Bytes(bytes) if bytes == raw
        ));
        assert_eq!(content.patches.len(), 1);
        assert!(content.patches.contains_key("gnprtb"));
        assert!(!content.patches.contains_key("encyclopedia"));

        let mut manifest = make_manifest("combined", "1.0.0", &[]);
        manifest.path = tmp.path().to_path_buf();
        manifest.enabled = true;
        let runtime = ModRuntime {
            discovered: vec![manifest.clone()],
            config: ModConfig::default(),
            errors: Vec::new(),
            mods_dir: tmp.path().to_path_buf(),
        };
        let mut world = parameter_world();

        let errors = runtime.apply_ordered(&mut world, &[&manifest]);

        assert!(errors.is_empty());
        assert_eq!(world.gnprtb.value(77, 0), 9);
        assert_eq!(world.gnprtb.value(78, 0), 314);
    }

    #[test]
    fn world_application_skips_a_large_valid_reserved_target_while_public_loading_reads_it_once() {
        use std::io::Write;

        const LARGE_VALID_TARGET_BYTES: u64 = 1024 * 1024;

        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join("gnprtb.json"),
            r#"[{"id":77,"development":23}]"#,
        )
        .unwrap();
        let mut target = std::fs::File::create(tmp.path().join(ENCYCLOPEDIA_MOD_FILENAME)).unwrap();
        target.write_all(b"[").unwrap();
        std::io::copy(
            &mut std::io::repeat(b' ').take(LARGE_VALID_TARGET_BYTES - 2),
            &mut target,
        )
        .unwrap();
        target.write_all(b"]").unwrap();
        drop(target);

        let mut manifest = make_manifest("large-content", "1.0.0", &[]);
        manifest.path = tmp.path().to_path_buf();
        manifest.enabled = true;
        let runtime = ModRuntime {
            discovered: vec![manifest.clone()],
            config: ModConfig::default(),
            errors: Vec::new(),
            mods_dir: tmp.path().to_path_buf(),
        };
        let mut world = parameter_world();

        reset_encyclopedia_target_read_calls();
        let errors = runtime.apply_ordered(&mut world, &[&manifest]);

        assert!(errors.is_empty());
        assert_eq!(world.gnprtb.value(77, 0), 23);
        assert_eq!(encyclopedia_target_read_calls(), 0);

        let content = ModContent::from_dir(tmp.path()).unwrap();
        assert!(matches!(
            content.encyclopedia,
            ModContentTarget::Bytes(bytes)
                if bytes.len() == usize::try_from(LARGE_VALID_TARGET_BYTES).unwrap()
        ));
        assert_eq!(encyclopedia_target_read_calls(), 1);
    }

    #[test]
    fn a_missing_encyclopedia_target_is_allowed() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("gnprtb.json"), "[]").unwrap();

        let content = ModContent::from_dir(tmp.path()).unwrap();

        assert!(matches!(content.encyclopedia, ModContentTarget::Missing));
        assert!(content.patches.contains_key("gnprtb"));
    }

    #[test]
    fn an_unreadable_encyclopedia_target_is_separate_from_valid_world_patches() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join("gnprtb.json"),
            r#"[{"id":77,"development":11}]"#,
        )
        .unwrap();
        let reserved_path = tmp.path().join("encyclopedia.json");
        std::fs::create_dir(&reserved_path).unwrap();

        let content = ModContent::from_dir(tmp.path()).unwrap();

        assert!(matches!(
            &content.encyclopedia,
            ModContentTarget::ReadError { path, kind, message }
                if path == &reserved_path
                    && *kind == std::io::ErrorKind::IsADirectory
                    && message.contains("reading mod content target")
        ));
        assert_eq!(content.patches["gnprtb"].len(), 1);

        let mut world = json!({
            "gnprtb": {"entries": [{"parameter_id": 77, "development": 1}]}
        });
        ModLoader::apply(&mut world, &content).unwrap();
        assert_eq!(world["gnprtb"]["entries"][0]["development"], 11);
    }

    #[test]
    fn malformed_world_json_remains_an_error_when_encyclopedia_bytes_are_present() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("encyclopedia.json"), b"not parsed here").unwrap();
        std::fs::write(tmp.path().join("sdprtb.json"), b"[]").unwrap();
        std::fs::write(tmp.path().join("gnprtb.json"), b"not valid JSON").unwrap();

        let error = ModContent::from_dir(tmp.path()).unwrap_err();

        let message = error.to_string();
        assert!(message.contains("parsing JSON in"));
        assert!(message.contains("gnprtb.json"));
    }

    #[test]
    fn only_the_exact_root_encyclopedia_filename_is_reserved() {
        let tmp = tempfile::tempdir().unwrap();
        let root_bytes = b"root target bytes";
        std::fs::write(tmp.path().join("encyclopedia.json"), root_bytes).unwrap();
        std::fs::write(
            tmp.path().join("encyclopedia-copy.json"),
            r#"[{"id":1,"value":"world target"}]"#,
        )
        .unwrap();
        let nested = tmp.path().join("nested");
        std::fs::create_dir(&nested).unwrap();
        std::fs::write(nested.join("encyclopedia.json"), b"nested target bytes").unwrap();

        let content = ModContent::from_dir(tmp.path()).unwrap();

        assert!(matches!(
            &content.encyclopedia,
            ModContentTarget::Bytes(bytes) if bytes == root_bytes
        ));
        assert_eq!(content.patches.len(), 1);
        assert!(content.patches.contains_key("encyclopedia-copy"));
    }

    #[test]
    fn an_encyclopedia_target_above_the_parser_limit_is_rejected_before_retention() {
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join(ENCYCLOPEDIA_MOD_FILENAME);
        let file = std::fs::File::create(&target).unwrap();
        file.set_len((crate::encyclopedia::OVERLAY_JSON_BYTES_LIMIT as u64) + 1)
            .unwrap();

        let content = ModContent::from_dir(tmp.path()).unwrap();

        assert!(matches!(
            content.encyclopedia,
            ModContentTarget::ReadError { kind, message, .. }
                if kind == std::io::ErrorKind::InvalidData
                    && message.contains("resource_limit:json_bytes")
        ));
    }

    #[test]
    fn an_encyclopedia_target_at_the_parser_limit_is_retained_exactly() {
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join(ENCYCLOPEDIA_MOD_FILENAME);
        let file = std::fs::File::create(&target).unwrap();
        file.set_len(crate::encyclopedia::OVERLAY_JSON_BYTES_LIMIT as u64)
            .unwrap();

        let content = ModContent::from_dir(tmp.path()).unwrap();

        assert!(matches!(
            content.encyclopedia,
            ModContentTarget::Bytes(bytes)
                if bytes.len() == crate::encyclopedia::OVERLAY_JSON_BYTES_LIMIT
        ));
    }

    #[test]
    fn target_admission_runs_before_the_bounded_read() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join(ENCYCLOPEDIA_MOD_FILENAME),
            b"contributor bytes",
        )
        .unwrap();
        let mut observed_length = None;

        let result = read_encyclopedia_target_with_admission(tmp.path(), |length| {
            observed_length = Some(length);
            Err("synthetic admission rejection")
        });

        assert_eq!(result, Err("synthetic admission rejection"));
        assert_eq!(observed_length, Some(17));
    }

    #[test]
    fn empty_target_bytes_are_retained_for_the_parser_to_diagnose() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join(ENCYCLOPEDIA_MOD_FILENAME), b"").unwrap();

        let content = ModContent::from_dir(tmp.path()).unwrap();

        assert!(matches!(content.encyclopedia, ModContentTarget::Bytes(bytes) if bytes.is_empty()));
    }

    #[test]
    fn only_not_found_is_classified_as_an_absent_target() {
        let target = Path::new("synthetic/encyclopedia.json");
        assert!(matches!(
            classify_target_open_error(
                target,
                std::io::Error::new(std::io::ErrorKind::NotFound, "synthetic missing")
            ),
            ModContentTarget::Missing
        ));
        assert!(matches!(
            classify_target_open_error(
                target,
                std::io::Error::new(std::io::ErrorKind::PermissionDenied, "synthetic denied")
            ),
            ModContentTarget::ReadError { kind, .. }
                if kind == std::io::ErrorKind::PermissionDenied
        ));
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    fn symlinked_mod_root_is_rejected_without_reading_the_outside_target() {
        use std::os::unix::fs::symlink;

        let parent = tempfile::tempdir().unwrap();
        let outside = parent.path().join("outside");
        let linked_root = parent.path().join("linked-root");
        std::fs::create_dir(&outside).unwrap();
        std::fs::write(outside.join(ENCYCLOPEDIA_MOD_FILENAME), b"outside sentinel").unwrap();
        symlink(&outside, &linked_root).unwrap();

        let target = read_encyclopedia_target_with_admission(&linked_root, |_| {
            Ok::<(), std::convert::Infallible>(())
        })
        .unwrap();

        assert!(matches!(target, ModContentTarget::ReadError { .. }));
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    fn symlinked_target_is_rejected_without_reading_the_outside_bytes() {
        use std::os::unix::fs::symlink;

        let tmp = tempfile::tempdir().unwrap();
        let outside = tmp.path().join("outside.json");
        std::fs::write(&outside, b"outside sentinel").unwrap();
        symlink(&outside, tmp.path().join(ENCYCLOPEDIA_MOD_FILENAME)).unwrap();

        let target = read_encyclopedia_target_with_admission(tmp.path(), |_| {
            Ok::<(), std::convert::Infallible>(())
        })
        .unwrap();

        assert!(matches!(target, ModContentTarget::ReadError { .. }));
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    fn target_and_root_replacement_after_admission_cannot_redirect_the_read() {
        let parent = tempfile::tempdir().unwrap();
        let root = parent.path().join("mod-root");
        let pinned_root = parent.path().join("pinned-root");
        std::fs::create_dir(&root).unwrap();
        let original = b"original retained";
        let replacement = b"outside sentinel!";
        assert_eq!(original.len(), replacement.len());
        std::fs::write(root.join(ENCYCLOPEDIA_MOD_FILENAME), original).unwrap();

        let target = read_encyclopedia_target_with_admission(&root, |_| {
            std::fs::rename(&root, &pinned_root).unwrap();
            std::fs::create_dir(&root).unwrap();
            std::fs::write(root.join(ENCYCLOPEDIA_MOD_FILENAME), replacement).unwrap();
            Ok::<(), std::convert::Infallible>(())
        })
        .unwrap();

        assert!(matches!(
            target,
            ModContentTarget::Bytes(bytes) if bytes == original
        ));
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    fn growth_after_admission_is_rejected_before_the_exact_read() {
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join(ENCYCLOPEDIA_MOD_FILENAME);
        std::fs::write(&target, b"[]").unwrap();

        let result = read_encyclopedia_target_with_admission(tmp.path(), |_| {
            use std::io::Write;
            std::fs::OpenOptions::new()
                .append(true)
                .open(&target)
                .unwrap()
                .write_all(b"x")
                .unwrap();
            Ok::<(), std::convert::Infallible>(())
        })
        .unwrap();

        assert!(matches!(
            result,
            ModContentTarget::ReadError { kind, message, .. }
                if kind == std::io::ErrorKind::InvalidData
                    && message.contains("identity changed before read")
        ));
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    #[test]
    fn a_fifo_encyclopedia_target_is_rejected_without_blocking_for_a_writer() {
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join(ENCYCLOPEDIA_MOD_FILENAME);
        assert!(std::process::Command::new("mkfifo")
            .arg(&target)
            .status()
            .unwrap()
            .success());
        let root = tmp.path().to_path_buf();
        let (sent, received) = std::sync::mpsc::channel();
        let reader = std::thread::spawn(move || {
            let result = ModContent::from_dir(&root);
            let _ = sent.send(result);
        });

        let result = received.recv_timeout(std::time::Duration::from_millis(250));
        if result.is_err() {
            // Unblock the legacy implementation so RED evidence never strands
            // a test thread after proving that it attempted to read the FIFO.
            drop(std::fs::OpenOptions::new().write(true).open(&target));
        }
        let content = result
            .expect("the reserved target reader must reject a FIFO before opening it")
            .unwrap();
        reader.join().unwrap();

        assert!(matches!(
            content.encyclopedia,
            ModContentTarget::ReadError { kind, message, .. }
                if kind == std::io::ErrorKind::InvalidInput
                    && message.contains("not a regular file")
        ));
    }

    // ── ModRuntime tests ────────────────────────────────────────────────────

    #[test]
    fn discover_on_empty_directory_finds_no_mods() {
        let tmp = tempfile::tempdir().unwrap();
        let runtime = ModRuntime::discover(tmp.path());
        assert_eq!(runtime.discovered.len(), 0);
        assert!(runtime.errors.is_empty());
    }

    #[test]
    fn toggled_config_persists_after_save_and_reload() {
        let tmp = tempfile::tempdir().unwrap();
        let mut config = ModConfig::default();

        // Toggle on
        config.toggle("test-mod");
        assert!(config.is_enabled("test-mod"));
        config.save(tmp.path()).unwrap();

        // Reload and verify
        let reloaded = ModConfig::load(tmp.path());
        assert!(reloaded.is_enabled("test-mod"));

        // Toggle off
        let mut config2 = reloaded;
        config2.toggle("test-mod");
        assert!(!config2.is_enabled("test-mod"));
        config2.save(tmp.path()).unwrap();

        let reloaded2 = ModConfig::load(tmp.path());
        assert!(!reloaded2.is_enabled("test-mod"));
    }

    #[test]
    fn apply_patches_skips_disabled_mods() {
        let tmp = tempfile::tempdir().unwrap();

        // Create mod-a (will be enabled)
        let mod_a_dir = tmp.path().join("mod-a");
        std::fs::create_dir(&mod_a_dir).unwrap();
        std::fs::write(
            mod_a_dir.join("mod.toml"),
            r#"
name = "mod-a"
version = "1.0.0"
"#,
        )
        .unwrap();
        std::fs::write(
            mod_a_dir.join("capital_ships.json"),
            r#"[{"id": 1, "hull": 9999}]"#,
        )
        .unwrap();

        // Create mod-b (will NOT be enabled)
        let dependent_dir = tmp.path().join("mod-b");
        std::fs::create_dir(&dependent_dir).unwrap();
        std::fs::write(
            dependent_dir.join("mod.toml"),
            r#"
name = "mod-b"
version = "1.0.0"
"#,
        )
        .unwrap();
        std::fs::write(
            dependent_dir.join("capital_ships.json"),
            r#"[{"id": 1, "hull": 1}]"#,
        )
        .unwrap();

        // Enable only mod-a
        let mut config = ModConfig::default();
        config.toggle("mod-a");
        config.save(tmp.path()).unwrap();

        let runtime = ModRuntime::discover(tmp.path());
        assert_eq!(runtime.enabled_sorted().len(), 1);
        assert_eq!(runtime.enabled_sorted()[0].name, "mod-a");
    }

    #[test]
    fn an_enabled_mod_with_a_missing_dependency_loads_nothing_and_reports_why() {
        let tmp = tempfile::tempdir().unwrap();

        // Create a mod with a missing dependency
        let mod_dir = tmp.path().join("mod-bad");
        std::fs::create_dir(&mod_dir).unwrap();
        std::fs::write(
            mod_dir.join("mod.toml"),
            r#"
name = "mod-bad"
version = "1.0.0"

[dependencies]
"nonexistent" = ">=1.0.0"
"#,
        )
        .unwrap();

        let mut config = ModConfig::default();
        config.toggle("mod-bad");
        config.save(tmp.path()).unwrap();

        let runtime = ModRuntime::discover(tmp.path());
        // enabled_sorted() should return empty (load order resolution fails)
        let sorted = runtime.enabled_sorted();
        assert!(sorted.is_empty());
        assert!(matches!(
            runtime.errors.as_slice(),
            [ModError::MissingDependency { mod_name, dep_name }]
                if mod_name == "mod-bad" && dep_name == "nonexistent"
        ));
        assert_eq!(
            runtime.errors[0].to_string(),
            "requires 'nonexistent', which is not installed and enabled"
        );
    }

    fn write_manifest(root: &Path, name: &str, body: &str) {
        let dir = root.join(name);
        std::fs::create_dir(&dir).unwrap();
        std::fs::write(dir.join("mod.toml"), format!("name = \"{name}\"\n{body}")).unwrap();
    }

    fn write_parameter_patch(root: &Path, directory: &str, name: &str, value: i32) -> ModManifest {
        let path = root.join(directory);
        std::fs::create_dir(&path).unwrap();
        std::fs::write(
            path.join("gnprtb.json"),
            format!(r#"[{{"id":77,"development":{value}}}]"#),
        )
        .unwrap();
        let mut manifest = make_manifest(name, "1.0.0", &[]);
        manifest.path = path;
        manifest.enabled = true;
        manifest
    }

    fn parameter_world() -> rebellion_core::world::GameWorld {
        use rebellion_core::world::{GameWorld, GnprtbEntry, GnprtbParams};

        let entry = |parameter_id, development| GnprtbEntry {
            parameter_id,
            development,
            alliance_sp_easy: development,
            alliance_sp_medium: development,
            alliance_sp_hard: development,
            empire_sp_easy: development,
            empire_sp_medium: development,
            empire_sp_hard: development,
            multiplayer: development,
        };
        GameWorld {
            gnprtb: GnprtbParams::new(vec![entry(77, 1), entry(78, 314)]),
            difficulty_index: 6,
            ..GameWorld::default()
        }
    }

    #[test]
    fn apply_ordered_uses_the_supplied_order_and_preserves_unrelated_world_state() {
        let tmp = tempfile::tempdir().unwrap();
        let alpha = write_parameter_patch(tmp.path(), "alpha-dir", "alpha", 10);
        let zulu = write_parameter_patch(tmp.path(), "zulu-dir", "zulu", 20);
        let runtime = ModRuntime {
            discovered: vec![alpha.clone(), zulu.clone()],
            config: ModConfig::default(),
            errors: Vec::new(),
            mods_dir: tmp.path().to_path_buf(),
        };
        let mut world = parameter_world();
        let mut expected = serde_json::to_value(&world).unwrap();
        let target = expected["gnprtb"]["entries"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|entry| entry["parameter_id"] == 77)
            .unwrap();
        target["development"] = json!(10);

        let errors = runtime.apply_ordered(&mut world, &[&zulu, &alpha]);

        assert!(errors.is_empty());
        assert_eq!(serde_json::to_value(&world).unwrap(), expected);
        assert_eq!(world.gnprtb.value(78, 0), 314);
        assert_eq!(world.difficulty_index, 6);
    }

    #[test]
    fn apply_enabled_has_identical_world_effects_for_reversed_discovery_order() {
        let tmp = tempfile::tempdir().unwrap();
        let alpha = write_parameter_patch(tmp.path(), "alpha-dir", "alpha", 10);
        let zulu = write_parameter_patch(tmp.path(), "zulu-dir", "zulu", 20);
        let runtime = |discovered| ModRuntime {
            discovered,
            config: ModConfig {
                enabled: vec!["alpha".to_string(), "zulu".to_string()],
            },
            errors: Vec::new(),
            mods_dir: tmp.path().to_path_buf(),
        };
        let forward = runtime(vec![alpha.clone(), zulu.clone()]);
        let reverse = runtime(vec![zulu, alpha]);
        let mut forward_world = parameter_world();
        let mut reverse_world = parameter_world();

        assert!(forward.apply_enabled(&mut forward_world).is_empty());
        assert!(reverse.apply_enabled(&mut reverse_world).is_empty());

        assert_eq!(forward_world.gnprtb.value(77, 0), 20);
        assert_eq!(
            serde_json::to_value(forward_world).unwrap(),
            serde_json::to_value(reverse_world).unwrap()
        );
    }

    #[test]
    fn duplicate_enabled_names_still_block_loading_with_diagnostics() {
        let tmp = tempfile::tempdir().unwrap();
        let first = write_parameter_patch(tmp.path(), "first-dir", "duplicate", 10);
        let second = write_parameter_patch(tmp.path(), "second-dir", "duplicate", 20);
        let discovered = vec![first, second];

        let errors = dependency_errors(&discovered);

        assert_eq!(errors.len(), 2);
        assert!(errors.iter().all(|error| {
            error.mod_name() == "duplicate"
                && matches!(error, ModError::LoadOrder { message, .. }
                    if message.contains("duplicate mod name 'duplicate'"))
        }));
    }

    #[test]
    fn a_dependency_below_the_required_version_is_reported_as_a_mismatch() {
        let tmp = tempfile::tempdir().unwrap();
        write_manifest(tmp.path(), "mod-base", "version = \"1.0.0\"\n");
        write_manifest(
            tmp.path(),
            "mod-top",
            "version = \"1.0.0\"\n[dependencies]\n\"mod-base\" = \">=2.0.0\"\n",
        );
        let mut config = ModConfig::default();
        config.toggle("mod-base");
        config.toggle("mod-top");
        config.save(tmp.path()).unwrap();

        let runtime = ModRuntime::discover(tmp.path());

        assert!(runtime.enabled_sorted().is_empty());
        assert!(matches!(
            runtime.errors.as_slice(),
            [ModError::VersionMismatch { mod_name, dep_name, required, found }]
                if mod_name == "mod-top" && dep_name == "mod-base"
                    && required == ">=2.0.0" && found == "1.0.0"
        ));
    }

    #[test]
    fn refresh_clears_the_missing_error_once_the_dependency_is_installed() {
        let tmp = tempfile::tempdir().unwrap();
        write_manifest(
            tmp.path(),
            "mod-top",
            "version = \"1.0.0\"\n[dependencies]\n\"mod-base\" = \">=1.0.0\"\n",
        );
        let mut config = ModConfig::default();
        config.toggle("mod-top");
        config.toggle("mod-base");
        config.save(tmp.path()).unwrap();

        let mut runtime = ModRuntime::discover(tmp.path());
        assert_eq!(runtime.errors.len(), 1);

        write_manifest(tmp.path(), "mod-base", "version = \"1.0.0\"\n");
        runtime.refresh();

        assert!(runtime.errors.is_empty());
        assert_eq!(runtime.enabled_sorted().len(), 2);
    }

    #[test]
    fn a_dependency_cycle_names_every_enabled_mod_it_blocks() {
        let tmp = tempfile::tempdir().unwrap();
        for (name, dep) in [("mod-a", "mod-b"), ("mod-b", "mod-a")] {
            write_manifest(
                tmp.path(),
                name,
                &format!("version = \"1.0.0\"\n[dependencies]\n\"{dep}\" = \">=1.0.0\"\n"),
            );
        }
        let mut config = ModConfig::default();
        config.toggle("mod-a");
        config.toggle("mod-b");
        config.save(tmp.path()).unwrap();

        let mut runtime = ModRuntime::discover(tmp.path());
        assert!(runtime.enabled_sorted().is_empty());
        let mut named: Vec<_> = runtime
            .errors
            .iter()
            .filter(|e| matches!(e, ModError::LoadOrder { .. }))
            .map(ModError::mod_name)
            .collect();
        named.sort_unstable();
        assert_eq!(named, ["mod-a", "mod-b"]);

        runtime.toggle_mod("mod-b");
        assert!(
            runtime.errors.iter().all(|e| e.mod_name() == "mod-a"
                && matches!(e, ModError::MissingDependency { .. })),
            "breaking the cycle replaces the load-order errors"
        );
    }

    #[test]
    fn enabling_a_disabled_dependency_clears_its_missing_error() {
        let tmp = tempfile::tempdir().unwrap();
        write_manifest(tmp.path(), "mod-base", "version = \"1.0.0\"\n");
        write_manifest(
            tmp.path(),
            "mod-top",
            "version = \"1.0.0\"\n[dependencies]\n\"mod-base\" = \">=1.0.0\"\n",
        );
        let mut config = ModConfig::default();
        config.toggle("mod-top");
        config.save(tmp.path()).unwrap();

        let mut runtime = ModRuntime::discover(tmp.path());
        assert!(matches!(
            runtime.errors.as_slice(),
            [ModError::MissingDependency { mod_name, dep_name }]
                if mod_name == "mod-top" && dep_name == "mod-base"
        ));

        runtime.toggle_mod("mod-base");

        assert!(runtime.errors.is_empty());
        assert_eq!(runtime.enabled_sorted().len(), 2);
    }

    #[test]
    fn enabled_mod_list_sorted() {
        let tmp = tempfile::tempdir().unwrap();

        // mod-base: no deps
        let base_dir = tmp.path().join("mod-base");
        std::fs::create_dir(&base_dir).unwrap();
        std::fs::write(
            base_dir.join("mod.toml"),
            r#"
name = "mod-base"
version = "2.0.0"
"#,
        )
        .unwrap();

        // mod-ext: depends on mod-base
        let ext_dir = tmp.path().join("mod-ext");
        std::fs::create_dir(&ext_dir).unwrap();
        std::fs::write(
            ext_dir.join("mod.toml"),
            r#"
name = "mod-ext"
version = "1.5.0"

[dependencies]
"mod-base" = ">=1.0.0"
"#,
        )
        .unwrap();

        // Enable both
        let mut config = ModConfig::default();
        config.toggle("mod-base");
        config.toggle("mod-ext");
        config.save(tmp.path()).unwrap();

        let runtime = ModRuntime::discover(tmp.path());
        let list = runtime.enabled_mod_list();
        assert_eq!(list.len(), 2);
        // mod-base must come before mod-ext (dependency order)
        assert_eq!(list[0], ("mod-base".to_string(), "2.0.0".to_string()));
        assert_eq!(list[1], ("mod-ext".to_string(), "1.5.0".to_string()));
    }
}
