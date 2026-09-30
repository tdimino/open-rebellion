//! Native-only loading of a staged encyclopedia bundle for one selected installation.

use std::collections::BTreeMap;
use std::ffi::CString;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use rebellion_data::encyclopedia::{
    parse_manifest, EncyclopediaManifest, CATALOG_JSON_BYTES_LIMIT, MANIFEST_JSON_BYTES_LIMIT,
};
use rebellion_render::encyclopedia_assets::{
    MAX_ENCYCLOPEDIA_IMAGE_BYTES, MAX_ENCYCLOPEDIA_NON_IMAGE_BYTES,
};
use rebellion_render::inspect_encyclopedia_bytes;

use crate::encyclopedia_session::{
    prepare_encyclopedia_session, EncyclopediaAvailability, EncyclopediaBytes,
    MAX_ENCYCLOPEDIA_RETAINED_BYTES,
};

const CATALOG_PATH: &str = "catalog.json";
const MANIFEST_PATH: &str = "manifest.json";
const ENCYCLOPEDIA_DIRECTORY: &str = "encyclopedia";
const MAX_EFFECTIVE_IMAGE_BYTES: u64 = 128 * 1024 * 1024;
const MAX_BASE_DAT_AGGREGATE_BYTES: u64 = MAX_ENCYCLOPEDIA_RETAINED_BYTES;
const MAX_DIRECTORY_ENTRIES_PER_SCAN: usize = 16_384;
const MAX_DIRECTORY_NAME_BYTES_PER_SCAN: usize = 4 * 1024 * 1024;

#[derive(Clone, Copy)]
struct LoadLimits {
    catalog: usize,
    manifest: usize,
    image: usize,
    dat: usize,
    retained: u64,
    image_aggregate: u64,
    dat_aggregate: u64,
    directory_entries: usize,
    directory_name_bytes: usize,
}

impl LoadLimits {
    fn directory_limits(self) -> DirectoryLimits {
        DirectoryLimits {
            entries: self.directory_entries,
            name_bytes: self.directory_name_bytes,
        }
    }
}

impl Default for LoadLimits {
    fn default() -> Self {
        Self {
            catalog: CATALOG_JSON_BYTES_LIMIT,
            manifest: MANIFEST_JSON_BYTES_LIMIT,
            image: MAX_ENCYCLOPEDIA_IMAGE_BYTES,
            // E45's target-independent byte inspector supplies the one shared hash
            // implementation and therefore also owns this runtime DAT read ceiling.
            dat: MAX_ENCYCLOPEDIA_NON_IMAGE_BYTES,
            retained: MAX_ENCYCLOPEDIA_RETAINED_BYTES,
            image_aggregate: MAX_EFFECTIVE_IMAGE_BYTES,
            dat_aggregate: MAX_BASE_DAT_AGGREGATE_BYTES,
            directory_entries: MAX_DIRECTORY_ENTRIES_PER_SCAN,
            directory_name_bytes: MAX_DIRECTORY_NAME_BYTES_PER_SCAN,
        }
    }
}

#[derive(Clone, Copy)]
struct DirectoryLimits {
    entries: usize,
    name_bytes: usize,
}

impl Default for DirectoryLimits {
    fn default() -> Self {
        Self {
            entries: MAX_DIRECTORY_ENTRIES_PER_SCAN,
            name_bytes: MAX_DIRECTORY_NAME_BYTES_PER_SCAN,
        }
    }
}

#[derive(Debug)]
enum DirectoryScanError {
    Io(io::Error),
    EntryLimit { limit: usize },
    NameBytesLimit { limit: usize },
    IdentityChanged,
}

impl From<io::Error> for DirectoryScanError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

#[derive(Debug)]
struct NativeLoadError {
    code: &'static str,
    root: PathBuf,
    member: String,
    detail: String,
}

impl NativeLoadError {
    fn new(
        code: &'static str,
        root: impl Into<PathBuf>,
        member: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code,
            root: root.into(),
            member: member.into(),
            detail: detail.into(),
        }
    }
}

impl fmt::Display for NativeLoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: root {} member {:?}: {}",
            self.code,
            self.root.display(),
            self.member,
            self.detail
        )
    }
}

/// Loads the encyclopedia paired with `gdata`, or the explicitly supplied staging root.
#[must_use]
pub fn load_native_encyclopedia(
    gdata: &Path,
    override_root: Option<&Path>,
) -> EncyclopediaAvailability {
    match load_native_with(gdata, override_root, LoadLimits::default(), || {}) {
        Ok(session) => EncyclopediaAvailability::Ready(session),
        Err(error) => EncyclopediaAvailability::Unavailable(error.to_string()),
    }
}

fn load_native_with<F>(
    gdata: &Path,
    override_root: Option<&Path>,
    limits: LoadLimits,
    after_roots_open: F,
) -> Result<crate::encyclopedia_session::EncyclopediaSession, NativeLoadError>
where
    F: FnOnce(),
{
    platform::ensure_supported().map_err(|error| {
        NativeLoadError::new(
            "unsupported_platform",
            gdata,
            ".",
            format!("native encyclopedia filesystem adapter is unavailable: {error}"),
        )
    })?;
    let (gdata_dir, bundle_dir, bundle_root) =
        open_selected_roots(gdata, override_root, limits.directory_limits())?;

    // Tests use this seam to replace both pathnames. Reads below stay attached to
    // the already-open directory descriptions rather than following the new names.
    after_roots_open();

    let catalog_present = bundle_dir.exact_member_presence(CATALOG_PATH, &bundle_root)?;
    let manifest_present = bundle_dir.exact_member_presence(MANIFEST_PATH, &bundle_root)?;
    match (catalog_present, manifest_present) {
        (false, false) => {
            return Err(NativeLoadError::new(
                "bundle_absent",
                &bundle_root,
                ".",
                "catalog.json and manifest.json are absent",
            ));
        }
        (false, true) => {
            return Err(NativeLoadError::new(
                "partial_bundle",
                &bundle_root,
                CATALOG_PATH,
                "manifest is present but catalog is absent",
            ));
        }
        (true, false) => {
            return Err(NativeLoadError::new(
                "partial_bundle",
                &bundle_root,
                MANIFEST_PATH,
                "catalog is present but manifest is absent",
            ));
        }
        (true, true) => {}
    }

    let manifest_file = bundle_dir.open_exact_file(MANIFEST_PATH, &bundle_root)?;
    let manifest_len = manifest_file.preflight_len(limits.manifest, &bundle_root, MANIFEST_PATH)?;
    let manifest_retained = retained_add(
        0,
        MANIFEST_PATH,
        manifest_len,
        limits.retained,
        &bundle_root,
    )?;
    let manifest_bytes = manifest_file.read_stable(limits.manifest, &bundle_root, MANIFEST_PATH)?;
    let manifest = parse_manifest(&manifest_bytes).map_err(|error| {
        NativeLoadError::new(error.code(), &bundle_root, error.path(), error.to_string())
    })?;

    let base_dats = read_binding_sources(&gdata_dir, gdata, &manifest, limits)?;
    let bytes = read_bundle_files(
        &bundle_dir,
        &bundle_root,
        &manifest,
        manifest_bytes,
        manifest_retained,
        limits,
    )?;

    prepare_encyclopedia_session(bytes, &base_dats).map_err(|error| {
        NativeLoadError::new(
            error.code(),
            &bundle_root,
            error.path(),
            format!("{}; selected GData root {}", error, gdata.display()),
        )
    })
}

fn open_selected_roots(
    gdata: &Path,
    override_root: Option<&Path>,
    directory_limits: DirectoryLimits,
) -> Result<(ConfinedDir, ConfinedDir, PathBuf), NativeLoadError> {
    let gdata_dir =
        ConfinedDir::open_root_with_limits(gdata, gdata, "selected GData root", directory_limits)?;

    if let Some(root) = override_root {
        let bundle = ConfinedDir::open_root_with_limits(
            root,
            root,
            "explicit encyclopedia root",
            directory_limits,
        )?;
        return Ok((gdata_dir, bundle, root.to_path_buf()));
    }

    let common_layout = gdata
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.eq_ignore_ascii_case("GData"));
    if common_layout {
        let parent_path = gdata.parent().unwrap_or_else(|| Path::new("."));
        let install = ConfinedDir::open_root_with_limits(
            parent_path,
            parent_path,
            "selected install root",
            directory_limits,
        )?;
        let bundle_root = parent_path.join(ENCYCLOPEDIA_DIRECTORY);
        let bundle = install.open_exact_dir(
            ENCYCLOPEDIA_DIRECTORY,
            &bundle_root,
            "sibling encyclopedia root",
        )?;
        Ok((gdata_dir, bundle, bundle_root))
    } else {
        let bundle_root = gdata.join(ENCYCLOPEDIA_DIRECTORY);
        let bundle = gdata_dir.open_exact_dir(
            ENCYCLOPEDIA_DIRECTORY,
            &bundle_root,
            "flattened encyclopedia root",
        )?;
        Ok((gdata_dir, bundle, bundle_root))
    }
}

fn read_binding_sources(
    gdata_dir: &ConfinedDir,
    gdata_root: &Path,
    manifest: &EncyclopediaManifest,
    limits: LoadLimits,
) -> Result<BTreeMap<String, String>, NativeLoadError> {
    let mut opened = Vec::with_capacity(manifest.binding_sources.len());
    let mut aggregate = 0_u64;
    for source in &manifest.binding_sources {
        let (observed_name, file) =
            gdata_dir.open_case_insensitive_file(&source.basename, gdata_root)?;
        let len = file.preflight_len(limits.dat, gdata_root, &observed_name)?;
        aggregate = aggregate.checked_add(len).ok_or_else(|| {
            NativeLoadError::new(
                "resource_limit:base_dat_bytes",
                gdata_root,
                &observed_name,
                "base DAT aggregate length overflowed",
            )
        })?;
        if aggregate > limits.dat_aggregate {
            return Err(NativeLoadError::new(
                "resource_limit:base_dat_bytes",
                gdata_root,
                &observed_name,
                format!(
                    "base DAT aggregate exceeds {} bytes before allocation",
                    limits.dat_aggregate
                ),
            ));
        }
        opened.push((observed_name, file));
    }

    let mut facts = BTreeMap::new();
    for (observed_name, file) in opened {
        let bytes = file.read_stable(limits.dat, gdata_root, &observed_name)?;
        let digest = inspect_encyclopedia_bytes(&bytes, None).map_err(|detail| {
            NativeLoadError::new("base_dat_hash", gdata_root, &observed_name, detail)
        })?;
        facts.insert(observed_name, digest.sha256);
    }
    Ok(facts)
}

fn read_bundle_files(
    bundle_dir: &ConfinedDir,
    bundle_root: &Path,
    manifest: &EncyclopediaManifest,
    manifest_bytes: Vec<u8>,
    mut retained_total: u64,
    limits: LoadLimits,
) -> Result<EncyclopediaBytes, NativeLoadError> {
    let mut image_total = 0_u64;
    let mut opened = BTreeMap::new();

    // Pin every declared member and check all per-file/aggregate sizes before
    // allocating a single catalog or image buffer.
    for path in manifest.files.keys() {
        let file = bundle_dir.open_exact_file(path, bundle_root)?;
        let limit = if path == CATALOG_PATH {
            limits.catalog
        } else {
            limits.image
        };
        let len = file.preflight_len(limit, bundle_root, path)?;
        retained_total = retained_add(retained_total, path, len, limits.retained, bundle_root)?;
        if path != CATALOG_PATH {
            image_total = image_total.checked_add(len).ok_or_else(|| {
                NativeLoadError::new(
                    "resource_limit:effective_image_bytes",
                    bundle_root,
                    path,
                    "image aggregate length overflowed",
                )
            })?;
            if image_total > limits.image_aggregate {
                return Err(NativeLoadError::new(
                    "resource_limit:effective_image_bytes",
                    bundle_root,
                    path,
                    format!(
                        "declared image files exceed {} bytes before allocation",
                        limits.image_aggregate
                    ),
                ));
            }
        }
        opened.insert(path.clone(), (file, limit));
    }

    let mut bytes = BTreeMap::new();
    bytes.insert(MANIFEST_PATH.to_owned(), Arc::from(manifest_bytes));
    for (path, (file, limit)) in opened {
        let retained = file.read_stable(limit, bundle_root, &path)?;
        bytes.insert(path, Arc::from(retained));
    }
    Ok(bytes)
}

fn retained_add(
    total: u64,
    path: &str,
    byte_len: u64,
    limit: u64,
    root: &Path,
) -> Result<u64, NativeLoadError> {
    let path_len = u64::try_from(path.len()).map_err(|_| {
        NativeLoadError::new(
            "resource_limit:retained_bytes",
            root,
            path,
            "path length does not fit u64",
        )
    })?;
    let next = total
        .checked_add(path_len)
        .and_then(|value| value.checked_add(byte_len))
        .ok_or_else(|| {
            NativeLoadError::new(
                "resource_limit:retained_bytes",
                root,
                path,
                "retained-byte accounting overflowed",
            )
        })?;
    if next > limit {
        return Err(NativeLoadError::new(
            "resource_limit:retained_bytes",
            root,
            path,
            format!("candidate exceeds {limit} bytes before allocation"),
        ));
    }
    Ok(next)
}

struct ConfinedDir {
    file: File,
    display_path: PathBuf,
    directory_limits: DirectoryLimits,
}

impl ConfinedDir {
    #[cfg(test)]
    fn open_root(path: &Path, diagnostic_root: &Path, role: &str) -> Result<Self, NativeLoadError> {
        Self::open_root_with_limits(path, diagnostic_root, role, DirectoryLimits::default())
    }

    fn open_root_with_limits(
        path: &Path,
        diagnostic_root: &Path,
        role: &str,
        directory_limits: DirectoryLimits,
    ) -> Result<Self, NativeLoadError> {
        platform::ensure_supported().map_err(|error| {
            NativeLoadError::new(
                "unsupported_platform",
                diagnostic_root,
                ".",
                format!("native encyclopedia filesystem adapter is unavailable: {error}"),
            )
        })?;
        let metadata = fs::symlink_metadata(path).map_err(|error| {
            NativeLoadError::new(
                if error.kind() == io::ErrorKind::NotFound {
                    "missing_root"
                } else {
                    "root_open_failed"
                },
                diagnostic_root,
                ".",
                format!("{role}: {error}"),
            )
        })?;
        if metadata.file_type().is_symlink() {
            return Err(NativeLoadError::new(
                "unsafe_symlink",
                diagnostic_root,
                ".",
                format!("{role} is a symlink"),
            ));
        }
        if !metadata.is_dir() {
            return Err(NativeLoadError::new(
                "not_directory",
                diagnostic_root,
                ".",
                format!("{role} is not a directory"),
            ));
        }
        let file = platform::open_root_directory(path).map_err(|error| {
            NativeLoadError::new(
                "root_open_failed",
                diagnostic_root,
                ".",
                format!("cannot open {role}: {error}"),
            )
        })?;
        Ok(Self {
            file,
            display_path: path.to_path_buf(),
            directory_limits,
        })
    }

    fn exact_member_presence(
        &self,
        name: &str,
        diagnostic_root: &Path,
    ) -> Result<bool, NativeLoadError> {
        Ok(self
            .resolve_name(name, true, diagnostic_root, name)?
            .is_some())
    }

    fn open_exact_dir(
        &self,
        name: &str,
        diagnostic_root: &Path,
        role: &str,
    ) -> Result<Self, NativeLoadError> {
        let Some(observed) = self.resolve_name(name, true, diagnostic_root, name)? else {
            return Err(NativeLoadError::new(
                "missing_root",
                diagnostic_root,
                ".",
                format!("{role} is absent"),
            ));
        };
        let file = platform::open_child(&self.file, &observed, true)
            .map_err(|error| self.open_error(error, diagnostic_root, name, role))?;
        Ok(Self {
            file,
            display_path: self.display_path.join(observed),
            directory_limits: self.directory_limits,
        })
    }

    fn open_exact_file(
        &self,
        relative: &str,
        diagnostic_root: &Path,
    ) -> Result<OpenedFile, NativeLoadError> {
        let mut directory = Self {
            file: self.file.try_clone().map_err(|error| {
                NativeLoadError::new(
                    "file_open_failed",
                    diagnostic_root,
                    relative,
                    format!("cannot duplicate root handle: {error}"),
                )
            })?,
            display_path: self.display_path.clone(),
            directory_limits: self.directory_limits,
        };
        let mut components = relative.split('/').peekable();
        while let Some(component) = components.next() {
            let last = components.peek().is_none();
            let Some(observed) =
                directory.resolve_name(component, true, diagnostic_root, relative)?
            else {
                return Err(NativeLoadError::new(
                    "missing_file",
                    diagnostic_root,
                    relative,
                    format!("required component {component:?} is absent"),
                ));
            };
            let file =
                platform::open_child(&directory.file, &observed, !last).map_err(|error| {
                    directory.open_error(error, diagnostic_root, relative, "required bundle member")
                })?;
            if last {
                return OpenedFile::new(file, diagnostic_root, relative);
            }
            directory = Self {
                file,
                display_path: directory.display_path.join(observed),
                directory_limits: directory.directory_limits,
            };
        }
        Err(NativeLoadError::new(
            "unsafe_path",
            diagnostic_root,
            relative,
            "empty relative path",
        ))
    }

    fn open_case_insensitive_file(
        &self,
        basename: &str,
        diagnostic_root: &Path,
    ) -> Result<(String, OpenedFile), NativeLoadError> {
        let Some(observed) = self.resolve_name(basename, false, diagnostic_root, basename)? else {
            return Err(NativeLoadError::new(
                "missing_binding_source",
                diagnostic_root,
                basename,
                format!("required DAT {basename:?} is absent"),
            ));
        };
        let file = platform::open_child(&self.file, &observed, false).map_err(|error| {
            self.open_error(error, diagnostic_root, basename, "required base DAT")
        })?;
        Ok((observed, OpenedFile::new(file, diagnostic_root, basename)?))
    }

    fn resolve_name(
        &self,
        requested: &str,
        require_exact_case: bool,
        diagnostic_root: &Path,
        member: &str,
    ) -> Result<Option<String>, NativeLoadError> {
        let mut match_count = 0_usize;
        let mut matches = Vec::with_capacity(2);
        platform::visit_directory_entries(&self.file, self.directory_limits, |name, symlink| {
            let Some(name) = name.to_str() else {
                return;
            };
            if !name.eq_ignore_ascii_case(requested) {
                return;
            }
            match_count = match_count.saturating_add(1);
            if matches.len() < 2 {
                matches.push(DirectoryEntry {
                    name: name.to_owned(),
                    symlink,
                });
            }
        })
        .map_err(|error| match error {
            DirectoryScanError::Io(error) => NativeLoadError::new(
                "directory_read_failed",
                diagnostic_root,
                member,
                error.to_string(),
            ),
            DirectoryScanError::EntryLimit { limit } => NativeLoadError::new(
                "resource_limit:directory_entries",
                diagnostic_root,
                member,
                format!("directory scan exceeds {limit} entries"),
            ),
            DirectoryScanError::NameBytesLimit { limit } => NativeLoadError::new(
                "resource_limit:directory_name_bytes",
                diagnostic_root,
                member,
                format!("directory scan exceeds {limit} total name bytes"),
            ),
            DirectoryScanError::IdentityChanged => NativeLoadError::new(
                "directory_changed_during_scan",
                diagnostic_root,
                member,
                "the descriptor-backed directory identity changed during enumeration",
            ),
        })?;
        if match_count > 1 {
            matches.sort_by(|left, right| left.name.cmp(&right.name));
            return Err(NativeLoadError::new(
                "ambiguous_case",
                diagnostic_root,
                member,
                format!(
                    "case-folded name {requested:?} has candidates {:?}",
                    matches
                        .iter()
                        .map(|entry| entry.name.as_str())
                        .collect::<Vec<_>>(),
                ),
            ));
        }
        let Some(entry) = matches.pop() else {
            return Ok(None);
        };
        if require_exact_case && entry.name != requested {
            return Err(NativeLoadError::new(
                "case_mismatch",
                diagnostic_root,
                member,
                format!("expected exact name {requested:?}, found {:?}", entry.name),
            ));
        }
        if entry.symlink {
            return Err(NativeLoadError::new(
                "unsafe_symlink",
                diagnostic_root,
                member,
                format!("component {:?} is a symlink", entry.name),
            ));
        }
        Ok(Some(entry.name))
    }

    fn open_error(
        &self,
        error: io::Error,
        diagnostic_root: &Path,
        member: &str,
        role: &str,
    ) -> NativeLoadError {
        NativeLoadError::new(
            if platform::is_no_follow_error(&error) {
                "unsafe_symlink"
            } else if error.kind() == io::ErrorKind::NotFound {
                "missing_file"
            } else {
                "file_open_failed"
            },
            diagnostic_root,
            member,
            format!("cannot open {role}: {error}"),
        )
    }
}

struct OpenedFile {
    file: File,
    before: fs::Metadata,
}

impl OpenedFile {
    fn new(file: File, diagnostic_root: &Path, member: &str) -> Result<Self, NativeLoadError> {
        let before = file.metadata().map_err(|error| {
            NativeLoadError::new(
                "file_metadata_failed",
                diagnostic_root,
                member,
                error.to_string(),
            )
        })?;
        if !before.is_file() {
            return Err(NativeLoadError::new(
                "not_regular_file",
                diagnostic_root,
                member,
                "required member is not a regular file",
            ));
        }
        Ok(Self { file, before })
    }

    fn preflight_len(
        &self,
        limit: usize,
        diagnostic_root: &Path,
        member: &str,
    ) -> Result<u64, NativeLoadError> {
        let limit = u64::try_from(limit).map_err(|_| {
            NativeLoadError::new(
                "resource_limit:file_bytes",
                diagnostic_root,
                member,
                "configured byte limit does not fit u64",
            )
        })?;
        let len = self.before.len();
        if len > limit {
            return Err(NativeLoadError::new(
                "resource_limit:file_bytes",
                diagnostic_root,
                member,
                format!("observed {len} bytes exceeds {limit} before allocation"),
            ));
        }
        Ok(len)
    }

    fn read_stable(
        mut self,
        limit: usize,
        diagnostic_root: &Path,
        member: &str,
    ) -> Result<Vec<u8>, NativeLoadError> {
        let expected = self.preflight_len(limit, diagnostic_root, member)?;
        let expected_usize = usize::try_from(expected).map_err(|_| {
            NativeLoadError::new(
                "resource_limit:file_bytes",
                diagnostic_root,
                member,
                "observed byte length does not fit usize",
            )
        })?;
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(expected_usize).map_err(|error| {
            NativeLoadError::new(
                "resource_limit:file_bytes",
                diagnostic_root,
                member,
                format!("cannot reserve bounded read buffer: {error}"),
            )
        })?;
        let mut bounded = self.file.by_ref().take(expected);
        bounded.read_to_end(&mut bytes).map_err(|error| {
            NativeLoadError::new(
                "file_read_failed",
                diagnostic_root,
                member,
                error.to_string(),
            )
        })?;
        if bytes.len() != expected_usize {
            return Err(NativeLoadError::new(
                "file_changed_during_read",
                diagnostic_root,
                member,
                format!(
                    "opened length was {expected_usize}, but exactly {} bytes were read",
                    bytes.len()
                ),
            ));
        }
        let mut extra = [0_u8; 1];
        if self.file.read(&mut extra).map_err(|error| {
            NativeLoadError::new(
                "file_read_failed",
                diagnostic_root,
                member,
                error.to_string(),
            )
        })? != 0
        {
            return Err(NativeLoadError::new(
                "file_changed_during_read",
                diagnostic_root,
                member,
                "file grew after its bounded preflight",
            ));
        }
        let after = self.file.metadata().map_err(|error| {
            NativeLoadError::new(
                "file_metadata_failed",
                diagnostic_root,
                member,
                error.to_string(),
            )
        })?;
        if !platform::same_file_snapshot(&self.before, &after) {
            return Err(NativeLoadError::new(
                "file_changed_during_read",
                diagnostic_root,
                member,
                "file identity or metadata changed during the bounded read",
            ));
        }
        Ok(bytes)
    }
}

#[derive(Debug)]
struct DirectoryEntry {
    name: String,
    symlink: bool,
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
mod platform {
    use std::ffi::{c_char, c_int, OsStr};
    use std::fs::{self, File, OpenOptions};
    use std::io;
    use std::os::fd::{AsRawFd, FromRawFd};
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
    use std::path::Path;

    use super::{CString, DirectoryLimits, DirectoryScanError};

    #[cfg(target_os = "linux")]
    const O_CLOEXEC: c_int = 0o2_000_000;
    #[cfg(target_os = "linux")]
    const O_DIRECTORY: c_int = 0o200_000;
    #[cfg(target_os = "linux")]
    const O_NOFOLLOW: c_int = 0o400_000;
    #[cfg(target_os = "linux")]
    const O_NONBLOCK: c_int = 0o4_000;
    #[cfg(target_os = "linux")]
    const DESCRIPTOR_DIRECTORY: &str = "/proc/self/fd";

    #[cfg(target_os = "macos")]
    const O_CLOEXEC: c_int = 0x0100_0000;
    #[cfg(target_os = "macos")]
    const O_DIRECTORY: c_int = 0x0010_0000;
    #[cfg(target_os = "macos")]
    const O_NOFOLLOW: c_int = 0x0000_0100;
    #[cfg(target_os = "macos")]
    const O_NONBLOCK: c_int = 0x0000_0004;
    #[cfg(target_os = "macos")]
    const DESCRIPTOR_DIRECTORY: &str = "/dev/fd";

    unsafe extern "C" {
        fn openat(directory: c_int, path: *const c_char, flags: c_int, ...) -> c_int;
    }

    pub fn ensure_supported() -> io::Result<()> {
        Ok(())
    }

    pub fn open_root_directory(path: &Path) -> io::Result<File> {
        OpenOptions::new()
            .read(true)
            .custom_flags(O_CLOEXEC | O_DIRECTORY | O_NOFOLLOW)
            .open(path)
    }

    pub fn open_child(parent: &File, name: &str, directory: bool) -> io::Result<File> {
        let name = CString::new(name).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidInput, "path component contains NUL")
        })?;
        let mut flags = O_CLOEXEC | O_NOFOLLOW | O_NONBLOCK;
        if directory {
            flags |= O_DIRECTORY;
        }
        // SAFETY: `parent` stays live for the call, `name` is NUL terminated, and
        // no creation flag is supplied, so `openat` consumes no variadic mode.
        let fd = unsafe { openat(parent.as_raw_fd(), name.as_ptr(), flags) };
        if fd == -1 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: a successful `openat` returns one newly owned descriptor.
        Ok(unsafe { File::from_raw_fd(fd) })
    }

    pub fn visit_directory_entries<F>(
        directory: &File,
        limits: DirectoryLimits,
        mut visit: F,
    ) -> Result<(), DirectoryScanError>
    where
        F: FnMut(&OsStr, bool),
    {
        let pinned_before = directory.metadata()?;
        let scan_directory = open_child(directory, ".", true)?;
        let scan_before = scan_directory.metadata()?;
        if !same_directory_identity(&pinned_before, &scan_before) {
            return Err(DirectoryScanError::IdentityChanged);
        }
        let scan_path =
            Path::new(DESCRIPTOR_DIRECTORY).join(scan_directory.as_raw_fd().to_string());

        // On Linux `/proc/self/fd/N` and on macOS `/dev/fd/N` reopen the live
        // descriptor, not its original pathname. `openat(parent, ".")` first
        // creates an independent directory description at offset zero, so a
        // `/dev/fd` duplicate on macOS cannot leak iteration offsets across scans.
        let entries = fs::read_dir(&scan_path)?;

        let mut entry_count = 0_usize;
        let mut name_bytes = 0_usize;
        for entry in entries {
            let entry = entry?;
            entry_count = entry_count
                .checked_add(1)
                .ok_or(DirectoryScanError::EntryLimit {
                    limit: limits.entries,
                })?;
            if entry_count > limits.entries {
                return Err(DirectoryScanError::EntryLimit {
                    limit: limits.entries,
                });
            }

            // Only this one transient OS string is materialized. Every name,
            // including unrelated and non-UTF-8 names, is charged before it can
            // be retained as a resolver candidate or diagnostic.
            let name = entry.file_name();
            name_bytes = name_bytes
                .checked_add(name.as_os_str().as_bytes().len())
                .ok_or(DirectoryScanError::NameBytesLimit {
                    limit: limits.name_bytes,
                })?;
            if name_bytes > limits.name_bytes {
                return Err(DirectoryScanError::NameBytesLimit {
                    limit: limits.name_bytes,
                });
            }
            visit(name.as_os_str(), entry.file_type()?.is_symlink());
        }

        let pinned_after = directory.metadata()?;
        let scan_after = scan_directory.metadata()?;
        if !same_directory_scan_identity(&pinned_before, &pinned_after, &scan_after) {
            return Err(DirectoryScanError::IdentityChanged);
        }
        Ok(())
    }

    pub fn is_no_follow_error(error: &io::Error) -> bool {
        #[cfg(target_os = "linux")]
        const ELOOP: i32 = 40;
        #[cfg(target_os = "macos")]
        const ELOOP: i32 = 62;
        error.raw_os_error() == Some(ELOOP)
    }

    fn same_directory_identity(before: &fs::Metadata, after: &fs::Metadata) -> bool {
        before.dev() == after.dev() && before.ino() == after.ino()
    }

    pub(super) fn same_directory_scan_identity(
        before: &fs::Metadata,
        pinned_after: &fs::Metadata,
        scan_after: &fs::Metadata,
    ) -> bool {
        same_directory_identity(before, pinned_after) && same_directory_identity(before, scan_after)
    }

    pub fn same_file_snapshot(before: &fs::Metadata, after: &fs::Metadata) -> bool {
        before.dev() == after.dev()
            && before.ino() == after.ino()
            && before.len() == after.len()
            && before.mtime() == after.mtime()
            && before.mtime_nsec() == after.mtime_nsec()
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
mod platform {
    use std::ffi::OsStr;
    use std::fs::{self, File};
    use std::io;
    use std::path::Path;

    use super::{DirectoryLimits, DirectoryScanError};

    fn unsupported() -> io::Error {
        io::Error::new(
            io::ErrorKind::Unsupported,
            "secure descriptor-relative encyclopedia reads support Linux and macOS only",
        )
    }

    pub fn ensure_supported() -> io::Result<()> {
        Err(unsupported())
    }

    pub fn open_root_directory(_path: &Path) -> io::Result<File> {
        Err(unsupported())
    }

    pub fn open_child(_parent: &File, _name: &str, _directory: bool) -> io::Result<File> {
        Err(unsupported())
    }

    pub fn visit_directory_entries<F>(
        _directory: &File,
        _limits: DirectoryLimits,
        _visit: F,
    ) -> Result<(), DirectoryScanError>
    where
        F: FnMut(&OsStr, bool),
    {
        Err(DirectoryScanError::Io(unsupported()))
    }

    pub fn is_no_follow_error(_error: &io::Error) -> bool {
        false
    }

    pub fn same_file_snapshot(_before: &fs::Metadata, _after: &fs::Metadata) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use serde_json::Value;

    use super::*;

    const VALID_CATALOG: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/catalog.json");
    const VALID_MANIFEST: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/manifest.json");
    const VALID_IMAGE_1: &[u8] = include_bytes!(
        "../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.001"
    );
    const VALID_IMAGE_2: &[u8] = include_bytes!(
        "../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.002"
    );
    const VALID_IMAGE_3: &[u8] = include_bytes!(
        "../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.003"
    );
    const VALID_DAT: &[u8] = include_bytes!(
        "../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/sources/SYNTHETIC.DAT"
    );

    static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

    struct TempTree(PathBuf);

    impl TempTree {
        fn new(label: &str) -> Self {
            let serial = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "open-rebellion-e13-{label}-{}-{serial}",
                std::process::id()
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempTree {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn write_valid_bundle(root: &Path) {
        fs::create_dir_all(root.join("assets")).unwrap();
        fs::write(root.join("catalog.json"), VALID_CATALOG).unwrap();
        fs::write(root.join("manifest.json"), VALID_MANIFEST).unwrap();
        fs::write(root.join("assets/EDATA.001"), VALID_IMAGE_1).unwrap();
        fs::write(root.join("assets/EDATA.002"), VALID_IMAGE_2).unwrap();
        fs::write(root.join("assets/EDATA.003"), VALID_IMAGE_3).unwrap();
    }

    fn write_valid_dat(gdata: &Path) {
        fs::create_dir_all(gdata).unwrap();
        fs::write(gdata.join("SYNTHETIC.DAT"), VALID_DAT).unwrap();
    }

    fn unavailable(availability: EncyclopediaAvailability) -> String {
        match availability {
            EncyclopediaAvailability::Unavailable(diagnostic) => diagnostic,
            EncyclopediaAvailability::Ready(_) => panic!("invalid input must not publish Ready"),
        }
    }

    fn native_error(
        result: Result<crate::encyclopedia_session::EncyclopediaSession, NativeLoadError>,
    ) -> NativeLoadError {
        match result {
            Ok(_) => panic!("invalid input must not produce a session"),
            Err(error) => error,
        }
    }

    #[cfg(unix)]
    fn directory_usage(path: &Path) -> (usize, usize) {
        use std::os::unix::ffi::OsStrExt;

        fs::read_dir(path)
            .unwrap()
            .map(|entry| {
                let name = entry.unwrap().file_name();
                (1_usize, name.as_os_str().as_bytes().len())
            })
            .fold((0, 0), |(entries, bytes), (entry, name_bytes)| {
                (entries + entry, bytes + name_bytes)
            })
    }

    #[test]
    fn alternate_install_uses_the_encyclopedia_sibling_of_selected_gdata() {
        let tree = TempTree::new("alternate");
        let install = tree.path().join("alternate-install");
        let gdata = install.join("GData");
        let bundle = install.join("encyclopedia");
        write_valid_dat(&gdata);
        write_valid_bundle(&bundle);

        let EncyclopediaAvailability::Ready(session) = load_native_encyclopedia(&gdata, None)
        else {
            panic!("valid sibling bundle should be ready")
        };

        assert_eq!(session.base_bytes()["catalog.json"].as_ref(), VALID_CATALOG);
        assert_eq!(
            session.base_bytes()["assets/EDATA.002"].as_ref(),
            VALID_IMAGE_2
        );
    }

    #[test]
    fn explicit_flattened_layout_uses_the_selected_directory_child_only() {
        let tree = TempTree::new("flattened");
        let flattened = tree.path().join("staged-data");
        write_valid_dat(&flattened);
        write_valid_bundle(&flattened.join("encyclopedia"));

        assert!(matches!(
            load_native_encyclopedia(&flattened, None),
            EncyclopediaAvailability::Ready(_)
        ));
    }

    #[test]
    fn explicit_override_is_used_without_falling_back_to_the_install_sibling() {
        let tree = TempTree::new("override");
        let install = tree.path().join("install");
        let gdata = install.join("GData");
        let sibling = install.join("encyclopedia");
        let override_root = tree.path().join("separate-staging");
        write_valid_dat(&gdata);
        fs::create_dir_all(&sibling).unwrap();
        fs::write(sibling.join("catalog.json"), b"not the selected bundle").unwrap();
        write_valid_bundle(&override_root);

        assert!(matches!(
            load_native_encyclopedia(&gdata, Some(&override_root)),
            EncyclopediaAvailability::Ready(_)
        ));
    }

    #[test]
    fn missing_and_wrong_selected_dats_are_named_unavailable_outcomes() {
        let tree = TempTree::new("dat-failures");
        let missing = tree.path().join("missing/GData");
        fs::create_dir_all(&missing).unwrap();
        write_valid_bundle(&tree.path().join("missing/encyclopedia"));
        let diagnostic = unavailable(load_native_encyclopedia(&missing, None));
        assert!(diagnostic.contains("SYNTHETIC.DAT"), "{diagnostic}");
        assert!(diagnostic.contains(missing.to_string_lossy().as_ref()));

        let wrong = tree.path().join("wrong/GData");
        fs::create_dir_all(&wrong).unwrap();
        fs::write(wrong.join("SYNTHETIC.DAT"), b"wrong installation").unwrap();
        write_valid_bundle(&tree.path().join("wrong/encyclopedia"));
        let diagnostic = unavailable(load_native_encyclopedia(&wrong, None));
        assert!(
            diagnostic.contains("binding_source_mismatch"),
            "{diagnostic}"
        );
        assert!(diagnostic.contains(wrong.to_string_lossy().as_ref()));
    }

    #[test]
    fn absent_partial_and_corrupt_bundles_never_publish_partial_ready_state() {
        let tree = TempTree::new("bundle-failures");
        let gdata = tree.path().join("GData");
        write_valid_dat(&gdata);

        let diagnostic = unavailable(load_native_encyclopedia(&gdata, None));
        assert!(diagnostic.contains("encyclopedia"), "{diagnostic}");

        let root = tree.path().join("encyclopedia");
        fs::create_dir(&root).unwrap();
        fs::write(root.join("catalog.json"), VALID_CATALOG).unwrap();
        let diagnostic = unavailable(load_native_encyclopedia(&gdata, None));
        assert!(diagnostic.contains("manifest.json"), "{diagnostic}");
        assert!(diagnostic.contains(root.to_string_lossy().as_ref()));

        fs::remove_dir_all(&root).unwrap();
        write_valid_bundle(&root);
        let mut corrupt = VALID_IMAGE_1.to_vec();
        corrupt[0] ^= 0xff;
        fs::write(root.join("assets/EDATA.001"), corrupt).unwrap();
        let diagnostic = unavailable(load_native_encyclopedia(&gdata, None));
        assert!(diagnostic.contains("EDATA.001"), "{diagnostic}");
        assert!(diagnostic.contains(root.to_string_lossy().as_ref()));
    }

    #[test]
    fn traversal_in_a_present_manifest_is_rejected_before_any_escape_read() {
        let tree = TempTree::new("traversal");
        let gdata = tree.path().join("GData");
        let root = tree.path().join("encyclopedia");
        write_valid_dat(&gdata);
        write_valid_bundle(&root);
        fs::write(tree.path().join("escape.bmp"), VALID_IMAGE_1).unwrap();
        let mut manifest: Value = serde_json::from_slice(VALID_MANIFEST).unwrap();
        manifest["files"]["assets/../escape.bmp"] = Value::String("0".repeat(64));
        fs::write(
            root.join("manifest.json"),
            serde_json::to_vec(&manifest).unwrap(),
        )
        .unwrap();

        let diagnostic = unavailable(load_native_encyclopedia(&gdata, None));
        assert!(diagnostic.contains("unsafe_asset_path"), "{diagnostic}");
    }

    #[cfg(unix)]
    #[test]
    fn internal_asset_symlink_is_rejected_even_when_it_targets_valid_bytes() {
        use std::os::unix::fs::symlink;

        let tree = TempTree::new("symlink");
        let gdata = tree.path().join("GData");
        let root = tree.path().join("encyclopedia");
        write_valid_dat(&gdata);
        write_valid_bundle(&root);
        let outside = tree.path().join("outside-image");
        fs::write(&outside, VALID_IMAGE_1).unwrap();
        fs::remove_file(root.join("assets/EDATA.001")).unwrap();
        symlink(&outside, root.join("assets/EDATA.001")).unwrap();

        let diagnostic = unavailable(load_native_encyclopedia(&gdata, None));
        assert!(diagnostic.contains("symlink"), "{diagnostic}");
        assert!(diagnostic.contains("assets/EDATA.001"), "{diagnostic}");
    }

    #[cfg(unix)]
    #[test]
    fn case_ambiguous_bundle_and_dat_names_are_rejected() {
        let tree = TempTree::new("case-ambiguity");
        let gdata = tree.path().join("GData");
        let root = tree.path().join("encyclopedia");
        write_valid_dat(&gdata);
        write_valid_bundle(&root);
        for index in 0..64 {
            fs::write(root.join(format!("unrelated-{index:03}")), b"ignored").unwrap();
        }
        fs::write(root.join("CATALOG.JSON"), VALID_CATALOG).unwrap();
        fs::write(root.join("Catalog.Json"), VALID_CATALOG).unwrap();
        let diagnostic = unavailable(load_native_encyclopedia(&gdata, None));
        assert!(diagnostic.contains("ambiguous_case"), "{diagnostic}");
        let reported_candidates = ["catalog.json", "CATALOG.JSON", "Catalog.Json"]
            .into_iter()
            .filter(|candidate| diagnostic.contains(candidate))
            .count();
        assert_eq!(
            reported_candidates, 2,
            "ambiguity diagnostics retain exactly two bounded candidates: {diagnostic}"
        );

        fs::remove_file(root.join("CATALOG.JSON")).unwrap();
        fs::remove_file(root.join("Catalog.Json")).unwrap();
        fs::write(gdata.join("synthetic.dat"), VALID_DAT).unwrap();
        let diagnostic = unavailable(load_native_encyclopedia(&gdata, None));
        assert!(diagnostic.contains("ambiguous_case"), "{diagnostic}");
        assert!(diagnostic.contains("SYNTHETIC.DAT"), "{diagnostic}");
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[test]
    fn directory_scan_limits_accept_exact_boundaries_and_reject_excess_work() {
        let tree = TempTree::new("directory-limits");
        let gdata = tree.path().join("GData");
        let root = tree.path().join("encyclopedia");
        write_valid_dat(&gdata);
        write_valid_bundle(&root);
        for index in 0..64 {
            fs::write(root.join(format!("unrelated-{index:03}")), b"ignored").unwrap();
        }

        let (entry_count, name_bytes) = directory_usage(&root);
        let exact = LoadLimits {
            directory_entries: entry_count,
            directory_name_bytes: name_bytes,
            ..LoadLimits::default()
        };
        load_native_with(&gdata, None, exact, || {})
            .expect("the exact directory entry and name-byte boundaries must be accepted");

        let entry_error = native_error(load_native_with(
            &gdata,
            None,
            LoadLimits {
                directory_entries: entry_count - 1,
                directory_name_bytes: name_bytes,
                ..LoadLimits::default()
            },
            || {},
        ));
        assert_eq!(entry_error.code, "resource_limit:directory_entries");

        let name_error = native_error(load_native_with(
            &gdata,
            None,
            LoadLimits {
                directory_entries: entry_count,
                directory_name_bytes: name_bytes - 1,
                ..LoadLimits::default()
            },
            || {},
        ));
        assert_eq!(name_error.code, "resource_limit:directory_name_bytes");
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[test]
    fn non_utf8_unrelated_names_are_charged_to_the_directory_work_budget() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;

        let tree = TempTree::new("directory-non-utf8");
        let gdata = tree.path().join("GData");
        let root = tree.path().join("encyclopedia");
        write_valid_dat(&gdata);
        write_valid_bundle(&root);
        fs::write(
            root.join(OsString::from_vec(vec![b'x', 0xff, b'y'])),
            b"ignored",
        )
        .unwrap();

        let (entry_count, name_bytes) = directory_usage(&root);
        let error = native_error(load_native_with(
            &gdata,
            None,
            LoadLimits {
                directory_entries: entry_count,
                directory_name_bytes: name_bytes - 1,
                ..LoadLimits::default()
            },
            || {},
        ));
        assert_eq!(error.code, "resource_limit:directory_name_bytes");
    }

    #[cfg(unix)]
    #[test]
    fn replacing_selected_paths_after_open_cannot_mix_another_installation() {
        let tree = TempTree::new("replace-race");
        let install = tree.path().join("install");
        let gdata = install.join("GData");
        let bundle = install.join("encyclopedia");
        write_valid_dat(&gdata);
        write_valid_bundle(&bundle);

        let old_gdata = install.join("opened-GData");
        let old_bundle = install.join("opened-encyclopedia");
        let session = load_native_with(&gdata, None, LoadLimits::default(), || {
            fs::rename(&gdata, &old_gdata).unwrap();
            fs::create_dir(&gdata).unwrap();
            fs::write(gdata.join("SYNTHETIC.DAT"), b"replacement DAT").unwrap();

            fs::rename(&bundle, &old_bundle).unwrap();
            fs::create_dir(&bundle).unwrap();
            fs::write(bundle.join("catalog.json"), b"replacement catalog").unwrap();
        })
        .expect("open directory descriptions must stay attached to one source snapshot");

        assert_eq!(
            session.base_bytes()["manifest.json"].as_ref(),
            VALID_MANIFEST
        );
        assert_eq!(
            session.base_bytes()["assets/EDATA.003"].as_ref(),
            VALID_IMAGE_3
        );
    }

    #[test]
    fn per_file_and_aggregate_limits_fail_before_publishing_a_candidate() {
        let tree = TempTree::new("read-limits");
        let gdata = tree.path().join("GData");
        let bundle = tree.path().join("encyclopedia");
        write_valid_dat(&gdata);
        write_valid_bundle(&bundle);

        let image_limit = LoadLimits {
            image: VALID_IMAGE_1.len() - 1,
            ..LoadLimits::default()
        };
        let error = load_native_with(&gdata, None, image_limit, || {}).unwrap_err();
        assert_eq!(error.code, "resource_limit:file_bytes");
        assert!(error.member.contains("EDATA.001"), "{error}");

        let dat_limit = LoadLimits {
            dat: VALID_DAT.len() - 1,
            ..LoadLimits::default()
        };
        let error = load_native_with(&gdata, None, dat_limit, || {}).unwrap_err();
        assert_eq!(error.code, "resource_limit:file_bytes");
        assert_eq!(error.member, "SYNTHETIC.DAT");

        let exact_dat_limit = LoadLimits {
            dat: VALID_DAT.len(),
            dat_aggregate: u64::try_from(VALID_DAT.len()).unwrap(),
            ..LoadLimits::default()
        };
        load_native_with(&gdata, None, exact_dat_limit, || {})
            .expect("the exact DAT byte limit and aggregate boundary must be accepted");

        let exact_image_total =
            u64::try_from(VALID_IMAGE_1.len() + VALID_IMAGE_2.len() + VALID_IMAGE_3.len()).unwrap();
        let exact_image_aggregate = LoadLimits {
            image_aggregate: exact_image_total,
            ..LoadLimits::default()
        };
        load_native_with(&gdata, None, exact_image_aggregate, || {})
            .expect("the exact image aggregate boundary must be accepted");

        let image_aggregate = LoadLimits {
            image_aggregate: exact_image_total - 1,
            ..LoadLimits::default()
        };
        let error = load_native_with(&gdata, None, image_aggregate, || {}).unwrap_err();
        assert_eq!(error.code, "resource_limit:effective_image_bytes");
    }

    #[test]
    fn approved_default_limits_and_retained_accounting_keep_exact_boundaries() {
        let limits = LoadLimits::default();
        assert_eq!(limits.image_aggregate, 128 * 1024 * 1024);
        assert_eq!(limits.dat_aggregate, 512 * 1024 * 1024);
        assert_eq!(limits.directory_entries, 16_384);
        assert_eq!(limits.directory_name_bytes, 4 * 1024 * 1024);

        let tree = TempTree::new("retained-boundary");
        assert_eq!(
            retained_add(2, "abc", 5, 10, tree.path()).unwrap(),
            10,
            "path and byte storage both count toward the retained ledger"
        );
        let error = retained_add(2, "abc", 5, 9, tree.path()).unwrap_err();
        assert_eq!(error.code, "resource_limit:retained_bytes");
    }

    #[test]
    fn absent_and_empty_roots_keep_distinct_stable_diagnostics() {
        let tree = TempTree::new("absence-diagnostics");
        let missing = tree.path().join("missing-GData");
        let error = match ConfinedDir::open_root(&missing, &missing, "selected GData root") {
            Ok(_) => panic!("an absent selected root must not open"),
            Err(error) => error,
        };
        assert_eq!(error.code, "missing_root");

        let install = tree.path().join("install");
        let gdata = install.join("GData");
        let bundle = install.join("encyclopedia");
        write_valid_dat(&gdata);
        fs::create_dir(&bundle).unwrap();
        let diagnostic = unavailable(load_native_encyclopedia(&gdata, None));
        assert!(diagnostic.starts_with("bundle_absent:"), "{diagnostic}");

        fs::write(bundle.join("catalog.json"), VALID_CATALOG).unwrap();
        let diagnostic = unavailable(load_native_encyclopedia(&gdata, None));
        assert!(diagnostic.starts_with("partial_bundle:"), "{diagnostic}");
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[test]
    fn a_regular_file_selected_as_a_root_is_rejected() {
        let tree = TempTree::new("regular-root");
        let gdata = tree.path().join("not-a-directory");
        fs::write(&gdata, b"not a directory").unwrap();

        let diagnostic = unavailable(load_native_encyclopedia(&gdata, None));
        assert!(diagnostic.starts_with("not_directory:"), "{diagnostic}");
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    #[test]
    fn unsupported_native_targets_fail_before_examining_the_selected_path() {
        let missing = Path::new("path-that-must-not-be-examined");
        let diagnostic = unavailable(load_native_encyclopedia(missing, None));
        assert!(
            diagnostic.starts_with("unsupported_platform:"),
            "{diagnostic}"
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn unix_descriptor_primitives_fail_closed_and_compare_complete_snapshots() {
        use std::fs::FileTimes;
        use std::os::unix::fs::symlink;
        use std::time::SystemTime;

        let tree = TempTree::new("descriptor-primitives");
        let root = tree.path().join("root");
        fs::create_dir(&root).unwrap();
        let root_handle = platform::open_root_directory(&root).unwrap();

        fs::write(root.join("regular"), b"same bytes").unwrap();
        assert!(platform::open_root_directory(&root.join("regular")).is_err());
        symlink(root.join("regular"), root.join("file-link")).unwrap();
        assert!(platform::open_child(&root_handle, "file-link", false).is_err());
        assert!(platform::open_child(&root_handle, "regular", true).is_err());

        let linked_root = tree.path().join("root-link");
        symlink(&root, &linked_root).unwrap();
        assert!(platform::open_root_directory(&linked_root).is_err());
        assert!(platform::is_no_follow_error(&io::Error::from_raw_os_error(
            40
        )));
        assert!(!platform::is_no_follow_error(
            &io::Error::from_raw_os_error(2)
        ));

        let first = root.join("first");
        let second = root.join("second");
        fs::write(&first, b"equal").unwrap();
        fs::write(&second, b"equal").unwrap();
        File::options()
            .write(true)
            .open(&first)
            .unwrap()
            .set_times(FileTimes::new().set_modified(SystemTime::UNIX_EPOCH))
            .unwrap();
        File::options()
            .write(true)
            .open(&second)
            .unwrap()
            .set_times(FileTimes::new().set_modified(SystemTime::UNIX_EPOCH))
            .unwrap();
        let first_metadata = fs::metadata(&first).unwrap();
        let second_metadata = fs::metadata(&second).unwrap();
        assert!(platform::same_file_snapshot(
            &first_metadata,
            &first_metadata
        ));
        assert!(!platform::same_file_snapshot(
            &first_metadata,
            &second_metadata
        ));

        let other_root = tree.path().join("other-root");
        fs::create_dir(&other_root).unwrap();
        let root_metadata = root_handle.metadata().unwrap();
        let other_metadata = fs::metadata(&other_root).unwrap();
        assert!(platform::same_directory_scan_identity(
            &root_metadata,
            &root_metadata,
            &root_metadata,
        ));
        assert!(!platform::same_directory_scan_identity(
            &root_metadata,
            &other_metadata,
            &root_metadata,
        ));
        assert!(!platform::same_directory_scan_identity(
            &root_metadata,
            &root_metadata,
            &other_metadata,
        ));

        let directory = ConfinedDir {
            file: root_handle,
            display_path: root,
            directory_limits: DirectoryLimits::default(),
        };
        let not_found = directory.open_error(
            io::Error::from(io::ErrorKind::NotFound),
            tree.path(),
            "missing",
            "test member",
        );
        assert_eq!(not_found.code, "missing_file");
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[test]
    fn resolved_members_and_intermediate_directories_cannot_be_swapped_for_symlinks() {
        use std::os::unix::fs::symlink;

        let tree = TempTree::new("component-replacement");
        let root = tree.path().join("root");
        let outside = tree.path().join("outside");
        fs::create_dir(&root).unwrap();
        fs::create_dir(&outside).unwrap();
        fs::write(root.join("member"), b"original").unwrap();
        fs::write(outside.join("member"), b"replacement").unwrap();
        fs::create_dir(root.join("assets")).unwrap();
        fs::create_dir(outside.join("assets")).unwrap();

        let directory = ConfinedDir::open_root(&root, &root, "test root").unwrap();
        let observed_member = directory
            .resolve_name("member", true, &root, "member")
            .unwrap()
            .unwrap();
        fs::rename(root.join("member"), root.join("opened-member")).unwrap();
        symlink(outside.join("member"), root.join("member")).unwrap();
        assert!(
            platform::open_child(&directory.file, &observed_member, false).is_err(),
            "a member replacement symlink must not be followed"
        );

        let observed_directory = directory
            .resolve_name("assets", true, &root, "assets")
            .unwrap()
            .unwrap();
        fs::rename(root.join("assets"), root.join("opened-assets")).unwrap();
        symlink(outside.join("assets"), root.join("assets")).unwrap();
        assert!(
            platform::open_child(&directory.file, &observed_directory, true).is_err(),
            "an intermediate-directory replacement symlink must not be followed"
        );
    }

    #[cfg(unix)]
    #[test]
    fn a_symlinked_bundle_directory_is_not_followed() {
        use std::os::unix::fs::symlink;

        let tree = TempTree::new("root-symlink");
        let gdata = tree.path().join("GData");
        let actual = tree.path().join("actual-bundle");
        write_valid_dat(&gdata);
        write_valid_bundle(&actual);
        symlink(&actual, tree.path().join("encyclopedia")).unwrap();

        let diagnostic = unavailable(load_native_encyclopedia(&gdata, None));
        assert!(diagnostic.contains("unsafe_symlink"), "{diagnostic}");
        assert!(diagnostic.contains("encyclopedia"), "{diagnostic}");
    }
}
