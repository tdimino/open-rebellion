//! Generation-bounded, selected-topic-only Encyclopedia texture ownership.

use crate::encyclopedia_surface::EncyclopediaArtworkView;

/// Sampling policy attached to a decoded upload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncyclopediaTextureSampling {
    Nearest,
}

/// Decoded pixels passed to a platform texture backend.
#[derive(Debug, Clone, Copy)]
pub struct EncyclopediaTextureUpload<'a> {
    pub filename: &'a str,
    pub digest: &'a str,
    pub width: u32,
    pub height: u32,
    pub rgba: &'a [u8],
    pub sampling: EncyclopediaTextureSampling,
}

/// Minimal ownership boundary for native and browser GPU handles.
pub trait EncyclopediaTextureBackend {
    type Texture;

    fn upload(&mut self, upload: EncyclopediaTextureUpload<'_>) -> Result<Self::Texture, String>;

    fn release(&mut self, texture: Self::Texture);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EncyclopediaTextureKey {
    generation: u64,
    resource_id: u16,
    filename: String,
    digest: String,
}

/// Result of resolving one active topic's optional artwork.
pub struct EncyclopediaTextureResolution<'a, Texture> {
    pub texture: Option<&'a Texture>,
    pub cache_hit: bool,
}

/// Retains at most one active topic texture and releases it deterministically.
pub struct EncyclopediaTopicTextureCache<Backend: EncyclopediaTextureBackend> {
    backend: Backend,
    retained: Option<(EncyclopediaTextureKey, Backend::Texture)>,
}

impl<Backend: EncyclopediaTextureBackend> EncyclopediaTopicTextureCache<Backend> {
    #[must_use]
    pub const fn new(backend: Backend) -> Self {
        Self {
            backend,
            retained: None,
        }
    }

    /// Resolve exact validated bytes for the selected topic.
    ///
    /// A session generation or resource identity change invalidates the prior
    /// handle. `None` also releases a retained handle, preventing stale art on
    /// source-unavailable topics and index frames.
    pub fn resolve(
        &mut self,
        generation: u64,
        artwork: Option<&EncyclopediaArtworkView<'_>>,
    ) -> Result<EncyclopediaTextureResolution<'_, Backend::Texture>, String> {
        let requested_key = artwork.map(|artwork| EncyclopediaTextureKey {
            generation,
            resource_id: artwork.resource_id,
            filename: artwork.filename.to_owned(),
            digest: artwork.digest.to_owned(),
        });
        if self
            .retained
            .as_ref()
            .is_some_and(|(key, _)| Some(key) == requested_key.as_ref())
        {
            return Ok(EncyclopediaTextureResolution {
                texture: self.retained.as_ref().map(|(_, texture)| texture),
                cache_hit: true,
            });
        }

        self.release_retained();
        let Some(artwork) = artwork else {
            return Ok(EncyclopediaTextureResolution {
                texture: None,
                cache_hit: false,
            });
        };

        let decoded = image::load_from_memory(artwork.bytes)
            .map_err(|error| format!("decoding {}: {error}", artwork.filename))?;
        if decoded.width() != artwork.width || decoded.height() != artwork.height {
            return Err(format!(
                "decoded {} dimensions {}x{} do not match validated {}x{}",
                artwork.filename,
                decoded.width(),
                decoded.height(),
                artwork.width,
                artwork.height
            ));
        }
        let rgba = decoded.to_rgba8();
        let texture = self.backend.upload(EncyclopediaTextureUpload {
            filename: artwork.filename,
            digest: artwork.digest,
            width: artwork.width,
            height: artwork.height,
            rgba: rgba.as_raw(),
            sampling: EncyclopediaTextureSampling::Nearest,
        })?;
        self.retained = requested_key.map(|key| (key, texture));
        Ok(EncyclopediaTextureResolution {
            texture: self.retained.as_ref().map(|(_, texture)| texture),
            cache_hit: false,
        })
    }

    fn release_retained(&mut self) {
        if let Some((_, texture)) = self.retained.take() {
            self.backend.release(texture);
        }
    }
}

impl<Backend: EncyclopediaTextureBackend> Drop for EncyclopediaTopicTextureCache<Backend> {
    fn drop(&mut self) {
        self.release_retained();
    }
}
