//! BMP texture cache for DLL-extracted UI assets.
//!
//! Provides `BmpCache`, a lazy-loading egui texture registry keyed by
//! `(DllSource, resource_id)`.  Callers request a texture by DLL source and
//! numeric resource ID; on first access the cache locates the BMP on disk,
//! decodes it via the `image` crate, and registers it as an egui texture.
//! Subsequent calls return the cached `TextureHandle` immediately.
//!
//! # Path convention
//!
//! Original BMPs are staged as:
//! ```text
//! {base_path}/{dll-name}-dll/BMP/{resource_id}.bmp
//! ```
//! e.g. `data/base/ui/strategy-dll/BMP/10553.bmp`
//!
//! HD override PNGs (optional) live at:
//! ```text
//! {hd_path}/{dll-name}/{resource_id}.png
//! ```
//! HD PNGs take priority over original BMPs when present.
//!
//! # WASM
//!
//! On `wasm32` targets filesystem access is unavailable; all loads return
//! `None` (the caller should render a placeholder).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use egui_macroquad::egui::{self, TextureHandle, TextureOptions};

// ---------------------------------------------------------------------------
// DllSource
// ---------------------------------------------------------------------------

/// Which DLL a UI resource comes from.
///
/// Maps directly to the `{dll-name}-dll/` staging directory name used by
/// `scripts/stage-ui-assets.py`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DllSource {
    /// `STRATEGY.DLL` — galaxy map chrome, character panels, event screens
    Strategy,
    /// `COMMON.DLL` — global buttons, sliders, main-menu backgrounds
    Common,
    /// `TACTICAL.DLL` — combat HUD, ship sprites, squadron controls
    Tactical,
    /// `GOKRES.DLL` — entity status sprites, character portraits, ship icons
    Gokres,
}

impl DllSource {
    /// Lowercase DLL name used as the staging directory prefix.
    ///
    /// Staging layout: `{base_path}/{dll_dir_name}/BMP/{id}.bmp`
    pub fn dll_dir_name(self) -> &'static str {
        match self {
            DllSource::Strategy => "strategy-dll",
            DllSource::Common   => "common-dll",
            DllSource::Tactical => "tactical-dll",
            DllSource::Gokres   => "gokres-dll",
        }
    }

    /// Egui texture name prefix (for debug labels).
    pub fn texture_prefix(self) -> &'static str {
        match self {
            DllSource::Strategy => "strategy",
            DllSource::Common   => "common",
            DllSource::Tactical => "tactical",
            DllSource::Gokres   => "gokres",
        }
    }
}

// ---------------------------------------------------------------------------
// BmpCache
// ---------------------------------------------------------------------------

/// Lazy-loading texture cache for DLL-extracted BMP assets.
pub struct BmpCache {
    /// Root directory containing staged `{dll-name}-dll/BMP/` trees.
    base_path: Option<PathBuf>,
    /// Optional HD PNG override directory.  If `Some`, checked before `base_path`.
    hd_path: Option<PathBuf>,
    /// Cached textures.  `None` value means "attempted load, file not found".
    textures: HashMap<(DllSource, u32), Option<TextureHandle>>,
}

impl BmpCache {
    /// Create an empty cache with no path configured.
    pub fn new() -> Self {
        Self {
            base_path: None,
            hd_path: None,
            textures: HashMap::new(),
        }
    }

    /// Set the root directory that contains `{dll-name}-dll/BMP/` trees.
    ///
    /// Call before any `get()` or `preload_range()` invocations.
    pub fn set_base_path(&mut self, path: impl Into<PathBuf>) {
        self.base_path = Some(path.into());
    }

    /// Set an optional HD PNG override directory.
    ///
    /// Expected layout: `{hd_path}/{dll-name}/{resource_id}.png`
    pub fn set_hd_path(&mut self, path: impl Into<PathBuf>) {
        self.hd_path = Some(path.into());
    }

    /// Retrieve a texture by source DLL and resource ID.
    ///
    /// On first call for a given `(source, id)` the BMP is loaded from disk
    /// and cached.  Returns `None` if the file is absent, unreadable, or this
    /// is a WASM build.
    pub fn get(
        &mut self,
        ctx: &egui::Context,
        source: DllSource,
        resource_id: u32,
    ) -> Option<&TextureHandle> {
        let key = (source, resource_id);

        if !self.textures.contains_key(&key) {
            let handle = self.load_texture(ctx, source, resource_id);
            self.textures.insert(key, handle);
        }

        self.textures.get(&key)?.as_ref()
    }

    /// Bulk-load all resources in `[start, end]` (inclusive) for one DLL.
    ///
    /// Useful for pre-warming the cache before the first frame that needs
    /// those textures, avoiding hitches.  Missing files are silently skipped.
    pub fn preload_range(
        &mut self,
        ctx: &egui::Context,
        source: DllSource,
        start: u32,
        end: u32,
    ) {
        for id in start..=end {
            let key = (source, id);
            if !self.textures.contains_key(&key) {
                let handle = self.load_texture(ctx, source, id);
                self.textures.insert(key, handle);
            }
        }
    }

    // ── Internal ────────────────────────────────────────────────────────────

    #[cfg(not(target_arch = "wasm32"))]
    fn load_texture(
        &self,
        ctx: &egui::Context,
        source: DllSource,
        resource_id: u32,
    ) -> Option<TextureHandle> {
        // 1. Check HD PNG override first.
        if let Some(hd_dir) = &self.hd_path {
            let hd_file = hd_dir
                .join(source.dll_dir_name())
                .join(format!("{}.png", resource_id));
            if hd_file.exists() {
                if let Some(handle) = load_image_as_texture(ctx, source, resource_id, &hd_file) {
                    return Some(handle);
                }
            }
        }

        // 2. Fall back to original staged BMP.
        let base = self.base_path.as_deref()?;
        let bmp_file = base
            .join(source.dll_dir_name())
            .join("BMP")
            .join(format!("{}.bmp", resource_id));

        if bmp_file.exists() {
            load_image_as_texture(ctx, source, resource_id, &bmp_file)
        } else {
            None
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn load_texture(
        &self,
        _ctx: &egui::Context,
        _source: DllSource,
        _resource_id: u32,
    ) -> Option<TextureHandle> {
        None
    }
}

impl Default for BmpCache {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// File loader (native only)
// ---------------------------------------------------------------------------

/// Decode an image file (BMP or PNG) and register it as an egui texture.
#[cfg(not(target_arch = "wasm32"))]
fn load_image_as_texture(
    ctx: &egui::Context,
    source: DllSource,
    resource_id: u32,
    path: &Path,
) -> Option<TextureHandle> {
    let bytes = std::fs::read(path).ok()?;

    // `image` crate auto-detects format from magic bytes — handles both BMP
    // (which may be palette-indexed) and PNG.
    let img = image::load_from_memory(&bytes).ok()?;
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();

    let color_image = egui::ColorImage::from_rgba_unmultiplied(
        [w as usize, h as usize],
        rgba.as_raw(),
    );

    let handle = ctx.load_texture(
        &format!("{}_{}", source.texture_prefix(), resource_id),
        color_image,
        TextureOptions::default(),
    );

    Some(handle)
}
