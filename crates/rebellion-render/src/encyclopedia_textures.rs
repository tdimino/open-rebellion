//! Lazy upload and single-selection ownership for verified encyclopedia art.

use egui_macroquad::egui::{self, TextureHandle, TextureOptions};
use image::ImageFormat;

use crate::encyclopedia_assets::inspect_encyclopedia_bytes;
use crate::encyclopedia_view::{TopicImageRenderProfile, TopicImageView};

/// Sampling applied by the upload backend after an upstream selector has
/// already chosen and validated the exact bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaTextureSampling {
    Nearest,
    Linear,
}

/// One decoded upload request. Pixel ownership remains with the cache for the
/// duration of the call; backends own only the returned texture handle.
pub struct EncyclopediaTextureUpload<'a> {
    pub asset_id: &'a str,
    pub digest: &'a str,
    pub width: u32,
    pub height: u32,
    pub rgba: &'a [u8],
    pub sampling: EncyclopediaTextureSampling,
}

/// Narrow renderer boundary used by the real egui path and ownership-counting
/// tests. `release` must relinquish the backend resource represented by a
/// handle exactly once.
pub trait EncyclopediaTextureBackend {
    type Texture;

    fn upload(&mut self, upload: EncyclopediaTextureUpload<'_>) -> Result<Self::Texture, String>;

    fn release(&mut self, texture: Self::Texture);
}

/// Real renderer backend. Cloning an egui context preserves access to the same
/// texture manager; dropping its `TextureHandle` releases that ownership.
#[derive(Clone)]
pub struct EguiEncyclopediaTextureBackend {
    context: egui::Context,
}

impl EguiEncyclopediaTextureBackend {
    #[must_use]
    pub fn new(context: &egui::Context) -> Self {
        Self {
            context: context.clone(),
        }
    }
}

impl EncyclopediaTextureBackend for EguiEncyclopediaTextureBackend {
    type Texture = TextureHandle;

    fn upload(&mut self, upload: EncyclopediaTextureUpload<'_>) -> Result<Self::Texture, String> {
        let width = usize::try_from(upload.width)
            .map_err(|_| "encyclopedia texture width does not fit usize".to_owned())?;
        let height = usize::try_from(upload.height)
            .map_err(|_| "encyclopedia texture height does not fit usize".to_owned())?;
        if width == 0 || height == 0 {
            return Err(format!(
                "encyclopedia texture dimensions must be nonzero, got {}x{}",
                upload.width, upload.height
            ));
        }
        let expected_rgba_len = width
            .checked_mul(height)
            .and_then(|pixels| pixels.checked_mul(4))
            .ok_or_else(|| {
                format!(
                    "encyclopedia RGBA shape {}x{} overflows addressable byte length",
                    upload.width, upload.height
                )
            })?;
        if upload.rgba.len() != expected_rgba_len {
            return Err(format!(
                "encyclopedia RGBA length mismatch for {}x{}: expected {}, got {}",
                upload.width,
                upload.height,
                expected_rgba_len,
                upload.rgba.len()
            ));
        }

        let max_texture_side = self.context.input(|input| input.max_texture_side);
        if width > max_texture_side || height > max_texture_side {
            return Err(format!(
                "encyclopedia texture dimensions {}x{} exceed current backend max texture side {}",
                upload.width, upload.height, max_texture_side
            ));
        }

        let image = egui::ColorImage::from_rgba_unmultiplied([width, height], upload.rgba);
        let options = match upload.sampling {
            EncyclopediaTextureSampling::Nearest => TextureOptions::NEAREST,
            EncyclopediaTextureSampling::Linear => TextureOptions::LINEAR,
        };
        Ok(self.context.load_texture(
            format!("encyclopedia:{}:{}", upload.asset_id, upload.digest),
            image,
            options,
        ))
    }

    fn release(&mut self, texture: Self::Texture) {
        drop(texture);
    }
}

/// Transition-only telemetry. A stable selection emits one initial selection
/// and at most one cache-hit event, rather than logging every frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncyclopediaTextureEvent {
    Selected {
        asset_id: String,
        digest: String,
        profile: TopicImageRenderProfile,
        cache_hit: bool,
    },
    Released {
        asset_id: String,
        digest: String,
        profile: TopicImageRenderProfile,
    },
    Failed {
        asset_id: String,
        diagnostic: String,
    },
}

/// Result for the current visible selection. Diagnostics remain available on
/// cached failures while events are emitted only on state transitions.
pub struct EncyclopediaTextureResolution<'a, Texture> {
    pub texture: Option<&'a Texture>,
    pub diagnostic: Option<&'a str>,
    pub events: Vec<EncyclopediaTextureEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextureKey {
    asset_id: String,
    digest: String,
    profile: TopicImageRenderProfile,
}

impl TextureKey {
    fn from_image(image: &TopicImageView) -> Self {
        Self {
            asset_id: image.asset_id.clone(),
            digest: image.digest.clone(),
            profile: image.render_profile,
        }
    }

    fn matches(&self, image: &TopicImageView) -> bool {
        self.asset_id == image.asset_id
            && self.digest == image.digest
            && self.profile == image.render_profile
    }

    fn selected_event(&self, cache_hit: bool) -> EncyclopediaTextureEvent {
        EncyclopediaTextureEvent::Selected {
            asset_id: self.asset_id.clone(),
            digest: self.digest.clone(),
            profile: self.profile,
            cache_hit,
        }
    }

    fn released_event(&self) -> EncyclopediaTextureEvent {
        EncyclopediaTextureEvent::Released {
            asset_id: self.asset_id.clone(),
            digest: self.digest.clone(),
            profile: self.profile,
        }
    }
}

enum CacheEntry<Texture> {
    Ready {
        key: TextureKey,
        texture: Texture,
        hit_reported: bool,
    },
    Failed {
        key: TextureKey,
        diagnostic: String,
    },
}

impl<Texture> CacheEntry<Texture> {
    fn key(&self) -> &TextureKey {
        match self {
            Self::Ready { key, .. } | Self::Failed { key, .. } => key,
        }
    }
}

/// Cache for the one image selected by the active topic. Superseded handles
/// are released before a new decode/upload begins, so an error can never
/// expose stale art and retained GPU ownership remains bounded to one handle.
pub struct EncyclopediaTextureCache<Backend: EncyclopediaTextureBackend> {
    backend: Backend,
    entry: Option<CacheEntry<Backend::Texture>>,
}

impl<Backend: EncyclopediaTextureBackend> EncyclopediaTextureCache<Backend> {
    #[must_use]
    pub fn new(backend: Backend) -> Self {
        Self {
            backend,
            entry: None,
        }
    }

    pub fn resolve<'a>(
        &'a mut self,
        image: Option<&TopicImageView>,
    ) -> EncyclopediaTextureResolution<'a, Backend::Texture> {
        let Some(image) = image else {
            let events = self.release_entry();
            return self.resolution(events);
        };

        if self
            .entry
            .as_ref()
            .is_some_and(|entry| entry.key().matches(image))
        {
            let events = match self.entry.as_mut() {
                Some(CacheEntry::Ready {
                    key, hit_reported, ..
                }) if !*hit_reported => {
                    *hit_reported = true;
                    vec![key.selected_event(true)]
                }
                _ => Vec::new(),
            };
            return self.resolution(events);
        }

        let mut events = self.release_entry();
        let key = TextureKey::from_image(image);
        match decode_selected_image(image).and_then(|rgba| {
            self.backend.upload(EncyclopediaTextureUpload {
                asset_id: &image.asset_id,
                digest: &image.digest,
                width: image.width,
                height: image.height,
                rgba: &rgba,
                sampling: sampling_for(image.render_profile),
            })
        }) {
            Ok(texture) => {
                events.push(key.selected_event(false));
                self.entry = Some(CacheEntry::Ready {
                    key,
                    texture,
                    hit_reported: false,
                });
            }
            Err(error) => {
                let diagnostic = format!(
                    "encyclopedia image {:?} is unavailable: {error}",
                    image.asset_id
                );
                events.push(EncyclopediaTextureEvent::Failed {
                    asset_id: image.asset_id.clone(),
                    diagnostic: diagnostic.clone(),
                });
                self.entry = Some(CacheEntry::Failed { key, diagnostic });
            }
        }

        self.resolution(events)
    }

    fn release_entry(&mut self) -> Vec<EncyclopediaTextureEvent> {
        match self.entry.take() {
            Some(CacheEntry::Ready { key, texture, .. }) => {
                self.backend.release(texture);
                vec![key.released_event()]
            }
            Some(CacheEntry::Failed { .. }) | None => Vec::new(),
        }
    }

    fn resolution(
        &self,
        events: Vec<EncyclopediaTextureEvent>,
    ) -> EncyclopediaTextureResolution<'_, Backend::Texture> {
        match self.entry.as_ref() {
            Some(CacheEntry::Ready { texture, .. }) => EncyclopediaTextureResolution {
                texture: Some(texture),
                diagnostic: None,
                events,
            },
            Some(CacheEntry::Failed { diagnostic, .. }) => EncyclopediaTextureResolution {
                texture: None,
                diagnostic: Some(diagnostic),
                events,
            },
            None => EncyclopediaTextureResolution {
                texture: None,
                diagnostic: None,
                events,
            },
        }
    }
}

impl<Backend: EncyclopediaTextureBackend> Drop for EncyclopediaTextureCache<Backend> {
    fn drop(&mut self) {
        if let Some(CacheEntry::Ready { texture, .. }) = self.entry.take() {
            self.backend.release(texture);
        }
    }
}

fn sampling_for(profile: TopicImageRenderProfile) -> EncyclopediaTextureSampling {
    match profile {
        TopicImageRenderProfile::OriginalNearest => EncyclopediaTextureSampling::Nearest,
        TopicImageRenderProfile::FaithfulHdLinear => EncyclopediaTextureSampling::Linear,
    }
}

fn decode_selected_image(image: &TopicImageView) -> Result<Vec<u8>, String> {
    let inspected = inspect_encyclopedia_bytes(&image.bytes, Some(&image.format))?;
    if inspected.sha256 != image.digest {
        return Err(format!(
            "selected-byte digest mismatch: view declares {}, bytes are {}",
            image.digest, inspected.sha256
        ));
    }
    if (inspected.width, inspected.height) != (Some(image.width), Some(image.height)) {
        return Err(format!(
            "selected-byte dimensions mismatch: view declares {}x{}, bytes are {}x{}",
            image.width,
            image.height,
            inspected.width.unwrap_or_default(),
            inspected.height.unwrap_or_default()
        ));
    }

    let format = match image.format.as_str() {
        "bmp" => ImageFormat::Bmp,
        "png" => ImageFormat::Png,
        _ => unreachable!("inspection rejects unsupported formats"),
    };
    let decoded = image::load_from_memory_with_format(&image.bytes, format)
        .map_err(|error| format!("selected-byte decode failed: {error}"))?;
    let rgba = decoded.to_rgba8();
    if rgba.dimensions() != (image.width, image.height) {
        return Err(format!(
            "upload dimensions mismatch: view declares {}x{}, decode produced {}x{}",
            image.width,
            image.height,
            rgba.width(),
            rgba.height()
        ));
    }
    Ok(rgba.into_raw())
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::io::Cursor;
    use std::rc::Rc;
    use std::sync::Arc;

    use egui_macroquad::egui;
    use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};
    use sha2::{Digest, Sha256};

    use crate::{TopicImageRenderProfile, TopicImageView};

    use super::{
        EguiEncyclopediaTextureBackend, EncyclopediaTextureBackend, EncyclopediaTextureCache,
        EncyclopediaTextureEvent, EncyclopediaTextureSampling, EncyclopediaTextureUpload,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct FakeTexture(u64);

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct UploadRecord {
        asset_id: String,
        digest: String,
        width: u32,
        height: u32,
        rgba_len: usize,
        sampling: EncyclopediaTextureSampling,
    }

    #[derive(Debug, Default)]
    struct BackendState {
        upload_attempts: usize,
        created: usize,
        released: usize,
        live: usize,
        max_live: usize,
        fail_uploads: bool,
        uploads: Vec<UploadRecord>,
    }

    #[derive(Clone)]
    struct FakeBackend {
        state: Rc<RefCell<BackendState>>,
    }

    impl EncyclopediaTextureBackend for FakeBackend {
        type Texture = FakeTexture;

        fn upload(
            &mut self,
            upload: EncyclopediaTextureUpload<'_>,
        ) -> Result<Self::Texture, String> {
            let mut state = self.state.borrow_mut();
            state.upload_attempts += 1;
            if state.fail_uploads {
                return Err("synthetic upload refusal".to_owned());
            }

            state.created += 1;
            state.live += 1;
            state.max_live = state.max_live.max(state.live);
            state.uploads.push(UploadRecord {
                asset_id: upload.asset_id.to_owned(),
                digest: upload.digest.to_owned(),
                width: upload.width,
                height: upload.height,
                rgba_len: upload.rgba.len(),
                sampling: upload.sampling,
            });
            Ok(FakeTexture(state.created as u64))
        }

        fn release(&mut self, _texture: Self::Texture) {
            let mut state = self.state.borrow_mut();
            state.released += 1;
            state.live -= 1;
        }
    }

    fn fake_backend() -> (FakeBackend, Rc<RefCell<BackendState>>) {
        let state = Rc::new(RefCell::new(BackendState::default()));
        (
            FakeBackend {
                state: state.clone(),
            },
            state,
        )
    }

    fn encoded_image(width: u32, height: u32, pixel: [u8; 4], format: ImageFormat) -> Arc<[u8]> {
        let mut bytes = Cursor::new(Vec::new());
        DynamicImage::ImageRgba8(RgbaImage::from_pixel(width, height, Rgba(pixel)))
            .write_to(&mut bytes, format)
            .expect("synthetic image must encode");
        Arc::from(bytes.into_inner())
    }

    fn encoded_png(pixel: [u8; 4]) -> Arc<[u8]> {
        encoded_image(1, 1, pixel, ImageFormat::Png)
    }

    fn encoded_bmp(pixel: [u8; 4]) -> Arc<[u8]> {
        encoded_image(1, 1, pixel, ImageFormat::Bmp)
    }

    fn image(asset_id: &str, bytes: Arc<[u8]>, profile: TopicImageRenderProfile) -> TopicImageView {
        image_with_format(asset_id, bytes, "png", profile)
    }

    fn image_with_format(
        asset_id: &str,
        bytes: Arc<[u8]>,
        format: &str,
        profile: TopicImageRenderProfile,
    ) -> TopicImageView {
        image_with_dimensions(asset_id, bytes, format, 1, 1, profile)
    }

    fn image_with_dimensions(
        asset_id: &str,
        bytes: Arc<[u8]>,
        format: &str,
        width: u32,
        height: u32,
        profile: TopicImageRenderProfile,
    ) -> TopicImageView {
        TopicImageView {
            asset_id: asset_id.to_owned(),
            digest: format!("{:x}", Sha256::digest(&bytes)),
            format: format.to_owned(),
            width,
            height,
            bytes,
            render_profile: profile,
        }
    }

    #[test]
    fn unchanged_identity_digest_and_profile_reuse_one_uploaded_handle() {
        let (backend, state) = fake_backend();
        let mut cache = EncyclopediaTextureCache::new(backend);
        let selected = image(
            "edata:7",
            encoded_png([1, 2, 3, 255]),
            TopicImageRenderProfile::OriginalNearest,
        );

        let first = cache.resolve(Some(&selected));
        assert_eq!(first.texture.map(|texture| texture.0), Some(1));
        assert_eq!(first.diagnostic, None);
        assert_eq!(
            first.events,
            vec![EncyclopediaTextureEvent::Selected {
                asset_id: "edata:7".to_owned(),
                digest: selected.digest.clone(),
                profile: TopicImageRenderProfile::OriginalNearest,
                cache_hit: false,
            }]
        );

        let first_hit = cache.resolve(Some(&selected));
        assert_eq!(first_hit.texture.map(|texture| texture.0), Some(1));
        assert_eq!(
            first_hit.events,
            vec![EncyclopediaTextureEvent::Selected {
                asset_id: "edata:7".to_owned(),
                digest: selected.digest.clone(),
                profile: TopicImageRenderProfile::OriginalNearest,
                cache_hit: true,
            }]
        );

        let later_frame = cache.resolve(Some(&selected));
        assert_eq!(later_frame.texture.map(|texture| texture.0), Some(1));
        assert!(later_frame.events.is_empty());

        let state = state.borrow();
        assert_eq!(state.upload_attempts, 1);
        assert_eq!(state.created, 1);
        assert_eq!(state.released, 0);
        assert_eq!(state.live, 1);
    }

    #[test]
    fn asset_digest_and_profile_changes_each_replace_the_owned_handle() {
        let (backend, state) = fake_backend();
        let mut cache = EncyclopediaTextureCache::new(backend);
        let shared = encoded_png([4, 5, 6, 255]);
        let first = image(
            "edata:7",
            shared.clone(),
            TopicImageRenderProfile::OriginalNearest,
        );
        let changed_asset = image("edata:8", shared, TopicImageRenderProfile::OriginalNearest);
        let changed_digest = image(
            "edata:8",
            encoded_png([7, 8, 9, 255]),
            TopicImageRenderProfile::OriginalNearest,
        );
        let changed_profile = image(
            "edata:8",
            changed_digest.bytes.clone(),
            TopicImageRenderProfile::FaithfulHdLinear,
        );

        assert_eq!(
            cache.resolve(Some(&first)).texture.map(|texture| texture.0),
            Some(1)
        );
        assert_eq!(
            cache
                .resolve(Some(&changed_asset))
                .texture
                .map(|texture| texture.0),
            Some(2)
        );
        assert_eq!(
            cache
                .resolve(Some(&changed_digest))
                .texture
                .map(|texture| texture.0),
            Some(3)
        );
        assert_eq!(
            cache
                .resolve(Some(&changed_profile))
                .texture
                .map(|texture| texture.0),
            Some(4)
        );

        let state = state.borrow();
        assert_eq!(state.created, 4);
        assert_eq!(state.released, 3);
        assert_eq!(state.live, 1);
        assert_eq!(state.max_live, 1);
        assert_eq!(
            state
                .uploads
                .iter()
                .map(|upload| upload.sampling)
                .collect::<Vec<_>>(),
            vec![
                EncyclopediaTextureSampling::Nearest,
                EncyclopediaTextureSampling::Nearest,
                EncyclopediaTextureSampling::Nearest,
                EncyclopediaTextureSampling::Linear,
            ]
        );
        assert_eq!(state.uploads[0].rgba_len, 4);
        assert_eq!((state.uploads[0].width, state.uploads[0].height), (1, 1));
    }

    #[test]
    fn null_art_allocates_nothing_and_releases_the_prior_selection() {
        let (backend, state) = fake_backend();
        let mut cache = EncyclopediaTextureCache::new(backend);
        let selected = image(
            "edata:7",
            encoded_png([10, 11, 12, 255]),
            TopicImageRenderProfile::OriginalNearest,
        );
        cache.resolve(Some(&selected));

        let cleared = cache.resolve(None);
        assert!(cleared.texture.is_none());
        assert_eq!(cleared.diagnostic, None);
        assert_eq!(
            cleared.events,
            vec![EncyclopediaTextureEvent::Released {
                asset_id: "edata:7".to_owned(),
                digest: selected.digest.clone(),
                profile: TopicImageRenderProfile::OriginalNearest,
            }]
        );
        assert!(cache.resolve(None).events.is_empty());

        let state = state.borrow();
        assert_eq!(state.created, 1);
        assert_eq!(state.released, 1);
        assert_eq!(state.live, 0);
    }

    #[test]
    fn failed_replacement_names_the_asset_and_never_exposes_stale_art() {
        let (backend, state) = fake_backend();
        let mut cache = EncyclopediaTextureCache::new(backend);
        let first = image(
            "edata:7",
            encoded_png([13, 14, 15, 255]),
            TopicImageRenderProfile::OriginalNearest,
        );
        let failed = image(
            "mod:v1:test:broken.png",
            encoded_png([16, 17, 18, 255]),
            TopicImageRenderProfile::FaithfulHdLinear,
        );
        cache.resolve(Some(&first));
        state.borrow_mut().fail_uploads = true;

        let failure = cache.resolve(Some(&failed));
        assert!(failure.texture.is_none());
        assert!(failure
            .diagnostic
            .expect("failure diagnostic")
            .contains("mod:v1:test:broken.png"));
        assert!(failure.events.iter().any(|event| matches!(
            event,
            EncyclopediaTextureEvent::Failed { asset_id, .. }
                if asset_id == "mod:v1:test:broken.png"
        )));

        state.borrow_mut().fail_uploads = false;
        let repeated = cache.resolve(Some(&failed));
        assert!(repeated.texture.is_none());
        assert!(repeated.events.is_empty());
        assert!(repeated
            .diagnostic
            .expect("cached failure diagnostic")
            .contains("mod:v1:test:broken.png"));

        let state = state.borrow();
        assert_eq!(state.upload_attempts, 2);
        assert_eq!(state.created, 1);
        assert_eq!(state.released, 1);
        assert_eq!(state.live, 0);
    }

    #[test]
    fn decode_failures_are_cached_without_per_frame_retry_or_event_spam() {
        let (backend, state) = fake_backend();
        let mut cache = EncyclopediaTextureCache::new(backend);
        let bytes: Arc<[u8]> = Arc::from(b"not a png".as_slice());
        let broken = image("edata:99", bytes, TopicImageRenderProfile::OriginalNearest);

        let first = cache.resolve(Some(&broken));
        assert!(first.texture.is_none());
        assert!(first
            .diagnostic
            .expect("decode diagnostic")
            .contains("edata:99"));
        assert_eq!(first.events.len(), 1);

        let repeated = cache.resolve(Some(&broken));
        assert!(repeated.texture.is_none());
        assert!(repeated.events.is_empty());
        assert_eq!(state.borrow().upload_attempts, 0);
    }

    #[test]
    fn repeated_image_edits_keep_one_live_resource_and_drop_every_handle() {
        let (backend, state) = fake_backend();
        {
            let mut cache = EncyclopediaTextureCache::new(backend);
            for value in 0_u8..50 {
                let selected = image(
                    "mod:v1:test:art.png",
                    encoded_png([value, 0, 0, 255]),
                    TopicImageRenderProfile::OriginalNearest,
                );
                assert!(cache.resolve(Some(&selected)).texture.is_some());
            }
        }

        let state = state.borrow();
        assert_eq!(state.created, 50);
        assert_eq!(state.released, 50);
        assert_eq!(state.live, 0);
        assert_eq!(state.max_live, 1);
    }

    #[test]
    fn egui_backend_accepts_verified_bytes_through_the_real_context_path() {
        let context = egui::Context::default();
        let backend = EguiEncyclopediaTextureBackend::new(&context);
        let mut cache = EncyclopediaTextureCache::new(backend);
        let selected = image(
            "edata:7",
            encoded_png([19, 20, 21, 255]),
            TopicImageRenderProfile::OriginalNearest,
        );

        let resolved = cache.resolve(Some(&selected));
        assert!(resolved.texture.is_some());
        assert_eq!(resolved.diagnostic, None);
        assert_eq!(resolved.events.len(), 1);
        assert!(cache.resolve(None).texture.is_none());
    }

    #[test]
    fn egui_backend_rejects_over_limit_selected_art_without_stale_or_new_resource() {
        let context = egui::Context::default();
        context.input_mut(|input| input.max_texture_side = 2);
        let initially_allocated = context.tex_manager().read().num_allocated();
        let backend = EguiEncyclopediaTextureBackend::new(&context);
        let mut cache = EncyclopediaTextureCache::new(backend);
        let wide_boundary = image_with_dimensions(
            "edata:wide-boundary",
            encoded_image(2, 1, [22, 23, 24, 255], ImageFormat::Bmp),
            "bmp",
            2,
            1,
            TopicImageRenderProfile::OriginalNearest,
        );
        let boundary = image_with_dimensions(
            "edata:boundary",
            encoded_image(1, 2, [25, 26, 27, 255], ImageFormat::Png),
            "png",
            1,
            2,
            TopicImageRenderProfile::OriginalNearest,
        );
        let oversized = image_with_dimensions(
            "edata:too-tall",
            encoded_image(1, 3, [28, 29, 30, 255], ImageFormat::Bmp),
            "bmp",
            1,
            3,
            TopicImageRenderProfile::OriginalNearest,
        );

        let accepted_wide = cache.resolve(Some(&wide_boundary));
        assert!(accepted_wide.texture.is_some());
        assert_eq!(accepted_wide.diagnostic, None);
        assert_eq!(
            context.tex_manager().read().num_allocated(),
            initially_allocated + 1
        );

        let accepted = cache.resolve(Some(&boundary));
        assert!(accepted.texture.is_some());
        assert_eq!(accepted.diagnostic, None);
        assert_eq!(
            context.tex_manager().read().num_allocated(),
            initially_allocated + 1
        );

        let rejected = cache.resolve(Some(&oversized));
        assert!(rejected.texture.is_none());
        let diagnostic = rejected
            .diagnostic
            .expect("asset-named limit diagnostic")
            .to_owned();
        assert!(diagnostic.contains("edata:too-tall"));
        assert!(diagnostic.contains("1x3"));
        assert!(diagnostic.contains("exceed current backend max texture side 2"));
        assert!(rejected.events.iter().any(|event| matches!(
            event,
            EncyclopediaTextureEvent::Failed { asset_id, .. }
                if asset_id == "edata:too-tall"
        )));
        assert_eq!(
            context.tex_manager().read().num_allocated(),
            initially_allocated
        );

        let repeated = cache.resolve(Some(&oversized));
        assert!(repeated.texture.is_none());
        assert!(repeated.events.is_empty());
        assert_eq!(repeated.diagnostic, Some(diagnostic.as_str()));
        assert_eq!(
            context.tex_manager().read().num_allocated(),
            initially_allocated
        );
    }

    #[test]
    fn public_egui_upload_rejects_invalid_rgba_shapes_without_panicking() {
        let context = egui::Context::default();
        let initially_allocated = context.tex_manager().read().num_allocated();
        let mut backend = EguiEncyclopediaTextureBackend::new(&context);

        for (width, height) in [(0, 1), (1, 0)] {
            let zero_side = backend.upload(EncyclopediaTextureUpload {
                asset_id: "synthetic:zero-side",
                digest: "unused",
                width,
                height,
                rgba: &[],
                sampling: EncyclopediaTextureSampling::Nearest,
            });
            let zero_side_error = match zero_side {
                Ok(_) => panic!("zero-sided RGBA dimensions must not allocate a texture"),
                Err(error) => error,
            };
            assert!(zero_side_error.contains("nonzero"));
        }

        let wrong_length = backend.upload(EncyclopediaTextureUpload {
            asset_id: "synthetic:wrong-length",
            digest: "unused",
            width: 2,
            height: 2,
            rgba: &[0; 15],
            sampling: EncyclopediaTextureSampling::Nearest,
        });
        let wrong_length_error = match wrong_length {
            Ok(_) => panic!("invalid RGBA length must not allocate a texture"),
            Err(error) => error,
        };
        assert!(wrong_length_error.contains("16"));
        assert!(wrong_length_error.contains("15"));

        let overflow = backend.upload(EncyclopediaTextureUpload {
            asset_id: "synthetic:overflow",
            digest: "unused",
            width: u32::MAX,
            height: u32::MAX,
            rgba: &[],
            sampling: EncyclopediaTextureSampling::Nearest,
        });
        let overflow_error = match overflow {
            Ok(_) => panic!("overflowing RGBA dimensions must not allocate a texture"),
            Err(error) => error,
        };
        assert!(overflow_error.contains("overflows"));
        assert_eq!(
            context.tex_manager().read().num_allocated(),
            initially_allocated
        );
    }

    #[test]
    fn bmp_selected_bytes_reach_the_backend_with_original_sampling() {
        let (backend, state) = fake_backend();
        let mut cache = EncyclopediaTextureCache::new(backend);
        let selected = image_with_format(
            "edata:7",
            encoded_bmp([22, 23, 24, 255]),
            "bmp",
            TopicImageRenderProfile::OriginalNearest,
        );

        let resolved = cache.resolve(Some(&selected));
        assert!(resolved.texture.is_some());
        assert_eq!(resolved.diagnostic, None);
        assert_eq!(state.borrow().upload_attempts, 1);
        assert_eq!(
            state.borrow().uploads[0].sampling,
            EncyclopediaTextureSampling::Nearest
        );
    }
}
