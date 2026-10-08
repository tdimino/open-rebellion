//! Native acquisition and atomic installation for Encyclopedia mod overlays.

#[cfg(not(target_arch = "wasm32"))]
use std::collections::BTreeMap;
#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::io::{Read, Take};
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
use rebellion_data::encyclopedia_overlay::{
    parse_encyclopedia_overlay, referenced_artwork_paths, EncyclopediaOverlayLayer,
};
#[cfg(not(target_arch = "wasm32"))]
use rebellion_data::encyclopedia_session::{
    EncyclopediaInstallDisposition, EncyclopediaSessionInput, EncyclopediaSessionStore,
    MAX_ENCYCLOPEDIA_IMAGE_BYTES,
};
#[cfg(not(target_arch = "wasm32"))]
use rebellion_data::mods::ModManifest;
#[cfg(not(target_arch = "wasm32"))]
use rebellion_data::mods::{ModContent, ModContentTarget, ModRuntime};

#[cfg(not(target_arch = "wasm32"))]
const MAX_MOD_ARTWORK_BYTES: usize = 128 * 1024 * 1024;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EncyclopediaModDiagnostic {
    pub mod_name: String,
    pub code: &'static str,
    pub path: String,
    pub detail: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EncyclopediaModInstallReport {
    pub disposition: Option<EncyclopediaInstallDisposition>,
    pub active_mods: Vec<String>,
    pub diagnostics: Vec<EncyclopediaModDiagnostic>,
}

/// Acquire every enabled native contribution in the shared resolved order.
/// No session state changes during this phase.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn prepare_native_encyclopedia_overlays(
    runtime: &ModRuntime,
    ordered: &[&ModManifest],
) -> Result<Vec<EncyclopediaOverlayLayer>, Vec<EncyclopediaModDiagnostic>> {
    if !runtime.errors.is_empty() {
        return Err(runtime
            .errors
            .iter()
            .map(|error| EncyclopediaModDiagnostic {
                mod_name: error.mod_name().to_owned(),
                code: "dependency_error",
                path: "mod.toml".to_owned(),
                detail: error.to_string(),
            })
            .collect());
    }

    let mut layers = Vec::new();
    for manifest in ordered {
        if let Err(error) = validate_mod_root(&manifest.path) {
            return Err(vec![diagnostic(
                &manifest.name,
                "mod_root_unavailable",
                &manifest.path.display().to_string(),
                error,
            )]);
        }
        let target = ModContent::encyclopedia_target_from_dir(&manifest.path);
        let bytes = match target {
            ModContentTarget::Missing => continue,
            ModContentTarget::Bytes(bytes) => bytes,
            ModContentTarget::ReadError { path, message, .. } => {
                return Err(vec![diagnostic(
                    &manifest.name,
                    "overlay_unavailable",
                    &path.display().to_string(),
                    message,
                )]);
            }
        };
        let patches = match parse_encyclopedia_overlay(&bytes) {
            Ok(patches) => patches,
            Err(error) => {
                return Err(vec![diagnostic(
                    &manifest.name,
                    "overlay_invalid",
                    "encyclopedia.json",
                    format!("{error:#}"),
                )]);
            }
        };
        let mut artwork = BTreeMap::new();
        let mut retained = 0_usize;
        for path in referenced_artwork_paths(&patches) {
            let bytes = match read_confined_artwork(&manifest.path, path) {
                Ok(bytes) => bytes,
                Err(error) => {
                    return Err(vec![diagnostic(
                        &manifest.name,
                        "artwork_unavailable",
                        path,
                        error,
                    )]);
                }
            };
            retained = match retained.checked_add(bytes.len()) {
                Some(total) if total <= MAX_MOD_ARTWORK_BYTES => total,
                _ => {
                    return Err(vec![diagnostic(
                        &manifest.name,
                        "artwork_resource_limit",
                        path,
                        "mod artwork aggregate byte limit exceeded",
                    )]);
                }
            };
            artwork.insert(path.to_owned(), Arc::from(bytes));
        }
        match EncyclopediaOverlayLayer::new(manifest.name.clone(), patches, artwork) {
            Ok(layer) => layers.push(layer),
            Err(error) => {
                return Err(vec![diagnostic(
                    &manifest.name,
                    "overlay_invalid",
                    "encyclopedia.json",
                    format!("{error:#}"),
                )]);
            }
        }
    }
    Ok(layers)
}

/// Rebuild from the immutable base and publish only a fully valid candidate.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn install_native_encyclopedia_mods(
    base: &EncyclopediaSessionInput,
    store: &mut EncyclopediaSessionStore,
    runtime: &ModRuntime,
    ordered: &[&ModManifest],
) -> EncyclopediaModInstallReport {
    let layers = match prepare_native_encyclopedia_overlays(runtime, ordered) {
        Ok(layers) => layers,
        Err(diagnostics) => {
            return EncyclopediaModInstallReport {
                disposition: None,
                active_mods: Vec::new(),
                diagnostics,
            };
        }
    };
    let active_mods = layers
        .iter()
        .map(|layer| layer.mod_name().to_owned())
        .collect();
    match store.replace_with_overlays(base.clone(), layers) {
        Ok(disposition) => EncyclopediaModInstallReport {
            disposition: Some(disposition),
            active_mods,
            diagnostics: Vec::new(),
        },
        Err(error) => EncyclopediaModInstallReport {
            disposition: None,
            active_mods: Vec::new(),
            diagnostics: vec![diagnostic(
                "effective-session",
                "candidate_invalid",
                "encyclopedia.json",
                format!("{error:#}"),
            )],
        },
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn log_native_encyclopedia_mod_report(report: &EncyclopediaModInstallReport) {
    for diagnostic in &report.diagnostics {
        macroquad::logging::warn!(
            "[encyclopedia] mod_fallback mod={} code={} path={} detail={}",
            diagnostic.mod_name,
            diagnostic.code,
            diagnostic.path,
            diagnostic.detail
        );
    }
    macroquad::logging::info!(
        "[encyclopedia] mod_candidate disposition={:?} active_mods={} diagnostics={}",
        report.disposition,
        report.active_mods.join(","),
        report.diagnostics.len()
    );
}

#[cfg(not(target_arch = "wasm32"))]
fn diagnostic(
    mod_name: &str,
    code: &'static str,
    path: &str,
    detail: impl Into<String>,
) -> EncyclopediaModDiagnostic {
    EncyclopediaModDiagnostic {
        mod_name: mod_name.to_owned(),
        code,
        path: path.to_owned(),
        detail: detail.into(),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn read_confined_artwork(root: &Path, relative: &str) -> Result<Vec<u8>, String> {
    validate_mod_root(root)?;
    let mut path = root.to_path_buf();
    for component in Path::new(relative).components() {
        let std::path::Component::Normal(component) = component else {
            return Err("artwork path is not confined to the mod root".to_owned());
        };
        path.push(component);
        let metadata = std::fs::symlink_metadata(&path)
            .map_err(|error| format!("cannot inspect {}: {error}", path.display()))?;
        if metadata.file_type().is_symlink() {
            return Err(format!("{} traverses a symlink", path.display()));
        }
    }
    let metadata = std::fs::symlink_metadata(&path)
        .map_err(|error| format!("cannot inspect {}: {error}", path.display()))?;
    if !metadata.is_file() {
        return Err(format!("{} is not a regular file", path.display()));
    }
    if metadata.len() > MAX_ENCYCLOPEDIA_IMAGE_BYTES as u64 {
        return Err(format!(
            "{} exceeds the {}-byte image limit",
            path.display(),
            MAX_ENCYCLOPEDIA_IMAGE_BYTES
        ));
    }
    let file =
        File::open(&path).map_err(|error| format!("cannot open {}: {error}", path.display()))?;
    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    let mut bounded: Take<File> = file.take(MAX_ENCYCLOPEDIA_IMAGE_BYTES as u64 + 1);
    bounded
        .read_to_end(&mut bytes)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    if bytes.len() > MAX_ENCYCLOPEDIA_IMAGE_BYTES {
        return Err(format!(
            "{} grew beyond its image byte limit",
            path.display()
        ));
    }
    Ok(bytes)
}

#[cfg(not(target_arch = "wasm32"))]
fn validate_mod_root(root: &Path) -> Result<(), String> {
    let root_metadata = std::fs::symlink_metadata(root)
        .map_err(|error| format!("cannot inspect mod root {}: {error}", root.display()))?;
    if root_metadata.file_type().is_symlink() || !root_metadata.is_dir() {
        return Err("mod root must be a regular non-symlink directory".to_owned());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use rebellion_data::mods::{ModConfig, ModManifest, ModRuntime};

    use super::prepare_native_encyclopedia_overlays;

    fn manifest(name: &str, path: PathBuf, dependencies: &[(&str, &str)]) -> ModManifest {
        ModManifest {
            name: name.to_owned(),
            version: "1.0.0".to_owned(),
            author: String::new(),
            description: String::new(),
            dependencies: dependencies
                .iter()
                .map(|(name, requirement)| ((*name).to_owned(), (*requirement).to_owned()))
                .collect(),
            path,
            enabled: true,
        }
    }

    fn runtime(root: &Path, discovered: Vec<ModManifest>) -> ModRuntime {
        ModRuntime {
            config: ModConfig {
                enabled: discovered.iter().map(|mod_| mod_.name.clone()).collect(),
            },
            discovered,
            errors: Vec::new(),
            mods_dir: root.to_path_buf(),
        }
    }

    use std::path::Path;

    #[test]
    fn missing_artwork_rejects_the_complete_overlay_candidate() {
        let root = tempfile::tempdir().unwrap();
        let mod_root = root.path().join("missing-art");
        std::fs::create_dir(&mod_root).unwrap();
        std::fs::write(
            mod_root.join("encyclopedia.json"),
            br#"[{"id":335544384,"image":{"path":"encyclopedia/assets/missing.bmp"}}]"#,
        )
        .unwrap();
        let runtime = runtime(root.path(), vec![manifest("missing-art", mod_root, &[])]);

        let diagnostics =
            prepare_native_encyclopedia_overlays(&runtime, &runtime.enabled_sorted()).unwrap_err();

        assert_eq!(diagnostics[0].mod_name, "missing-art");
        assert_eq!(diagnostics[0].code, "artwork_unavailable");
        assert!(diagnostics[0].path.ends_with("missing.bmp"));
    }

    #[test]
    fn shared_dependency_order_is_reused_for_encyclopedia_layers() {
        let root = tempfile::tempdir().unwrap();
        let mut manifests = Vec::new();
        for (name, dependencies) in [
            ("alpha-dependent", vec![("zulu-base", ">=1.0.0")]),
            ("zulu-base", Vec::new()),
        ] {
            let path = root.path().join(name);
            std::fs::create_dir(&path).unwrap();
            std::fs::write(path.join("encyclopedia.json"), b"[]").unwrap();
            manifests.push(manifest(name, path, &dependencies));
        }
        let runtime = runtime(root.path(), manifests);

        let layers =
            prepare_native_encyclopedia_overlays(&runtime, &runtime.enabled_sorted()).unwrap();

        assert_eq!(
            layers
                .iter()
                .map(|layer| layer.mod_name())
                .collect::<Vec<_>>(),
            ["zulu-base", "alpha-dependent"]
        );
    }

    #[test]
    fn symlinked_artwork_is_rejected_before_session_preparation() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;

            let root = tempfile::tempdir().unwrap();
            let mod_root = root.path().join("linked-art");
            std::fs::create_dir_all(mod_root.join("encyclopedia/assets")).unwrap();
            std::fs::write(root.path().join("outside.bmp"), b"outside").unwrap();
            symlink(
                root.path().join("outside.bmp"),
                mod_root.join("encyclopedia/assets/linked.bmp"),
            )
            .unwrap();
            std::fs::write(
                mod_root.join("encyclopedia.json"),
                br#"[{"id":335544384,"image":{"path":"encyclopedia/assets/linked.bmp"}}]"#,
            )
            .unwrap();
            let runtime = runtime(root.path(), vec![manifest("linked-art", mod_root, &[])]);

            let diagnostics =
                prepare_native_encyclopedia_overlays(&runtime, &runtime.enabled_sorted())
                    .unwrap_err();

            assert_eq!(diagnostics[0].code, "artwork_unavailable");
            assert!(diagnostics[0].detail.contains("symlink"));
        }
    }

    #[test]
    fn symlinked_mod_root_is_rejected_even_without_artwork() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;

            let root = tempfile::tempdir().unwrap();
            let real_root = root.path().join("real-root");
            let linked_root = root.path().join("linked-root");
            std::fs::create_dir(&real_root).unwrap();
            std::fs::write(real_root.join("encyclopedia.json"), b"[]").unwrap();
            symlink(&real_root, &linked_root).unwrap();
            let runtime = runtime(root.path(), vec![manifest("linked-root", linked_root, &[])]);

            let diagnostics =
                prepare_native_encyclopedia_overlays(&runtime, &runtime.enabled_sorted())
                    .unwrap_err();

            assert_eq!(diagnostics[0].code, "mod_root_unavailable");
            assert!(diagnostics[0].detail.contains("non-symlink directory"));
        }
    }
}
