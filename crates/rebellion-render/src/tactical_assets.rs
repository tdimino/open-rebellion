//! Decoding and rendering for the original tactical type-301/type-303 assets.
//!
//! P56 established one source-bound render proof. P57 installs its original
//! three-resource LOD family and reproduces the executable's selection rule.
//! No DAT identity is inferred from the resource ordinal.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

#[cfg(not(target_arch = "wasm32"))]
use std::{io::Read, path::Path};

use macroquad::prelude::*;
use macroquad::window::miniquad::{Backend, Comparison, PipelineParams};

use crate::bmp_cache::{resources, BmpCache, DllSource};

pub const PROOF_MESH_KEYS: [&str; 3] = ["2560/1033", "2561/1033", "2562/1033"];
pub const PROOF_TEXTURE_KEYS: [&str; 2] = ["SDESTI52.BMP/1033", "SDESTI_M.BMP/1033"];

const ORIGINAL_CLOSE_THRESHOLD: f32 = 15.0;
const ORIGINAL_MEDIUM_THRESHOLD: f32 = 40.0;
const ORIGINAL_REDUCED_DETAIL_THRESHOLD: f32 = 20.0;

const ORIGINAL_CAMERA_DISTANCE_SCALE: f32 = 1.7;
const ORIGINAL_CAMERA_FAR_SCALE: f32 = 2.5;
const ORIGINAL_CAMERA_NEAR: f32 = 1.0;
const ORIGINAL_CAMERA_FIELD: f32 = 0.2;
const ORIGINAL_CAMERA_FIELD_MIN: f32 = 0.005;
const ORIGINAL_CAMERA_FIELD_MAX: f32 = 1.5;
const ORIGINAL_CAMERA_ZOOM_IN_SCALE: f32 = 0.9;
const ORIGINAL_CAMERA_ZOOM_OUT_SCALE: f32 = 1.1;
const ORIGINAL_CAMERA_PITCH: i32 = 30;
const ORIGINAL_CAMERA_ALLIANCE_YAW: i32 = -30;
const ORIGINAL_CAMERA_EMPIRE_YAW: i32 = 150;
const ORIGINAL_CAMERA_INITIAL_STEP: i32 = 5;

/// Source-traced tactical camera state from `FUN_005d9490`, `FUN_005d9620`,
/// `FUN_005d9640`, and the command switch at `0x005d97c0`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OriginalTacticalCamera {
    pitch_degrees: i32,
    yaw_degrees: i32,
    zoom_step: i32,
    orbit_step: i32,
    field: f32,
    distance: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct OriginalCameraPose {
    position: Vec3,
    up: Vec3,
    field: f32,
    fovy_radians: f32,
    near: f32,
    far: f32,
    pitch_degrees: i32,
    yaw_degrees: i32,
    zoom_step: i32,
    orbit_step: i32,
}

impl OriginalTacticalCamera {
    /// Construct the original initial view for a battle extent and player side.
    #[must_use]
    pub fn new(player_is_empire: bool, battle_extent: f32) -> Self {
        Self {
            pitch_degrees: ORIGINAL_CAMERA_PITCH,
            yaw_degrees: if player_is_empire {
                ORIGINAL_CAMERA_EMPIRE_YAW
            } else {
                ORIGINAL_CAMERA_ALLIANCE_YAW
            },
            zoom_step: ORIGINAL_CAMERA_INITIAL_STEP,
            orbit_step: ORIGINAL_CAMERA_INITIAL_STEP,
            field: ORIGINAL_CAMERA_FIELD,
            distance: battle_extent * ORIGINAL_CAMERA_DISTANCE_SCALE,
        }
    }

    pub(crate) fn zoom_in(&mut self) {
        self.field = (self.field * ORIGINAL_CAMERA_ZOOM_IN_SCALE).max(ORIGINAL_CAMERA_FIELD_MIN);
        self.zoom_step -= 1;
        self.update_orbit_step();
    }

    pub(crate) fn zoom_out(&mut self) {
        self.field = (self.field * ORIGINAL_CAMERA_ZOOM_OUT_SCALE).min(ORIGINAL_CAMERA_FIELD_MAX);
        self.zoom_step += 1;
        self.update_orbit_step();
    }

    pub(crate) fn turn_left(&mut self) {
        self.yaw_degrees -= self.orbit_step;
    }

    pub(crate) fn turn_right(&mut self) {
        self.yaw_degrees += self.orbit_step;
    }

    pub(crate) fn pitch_up(&mut self) {
        if self.pitch_degrees < 90 {
            self.pitch_degrees += self.orbit_step;
        }
    }

    pub(crate) fn pitch_down(&mut self) {
        if self.pitch_degrees > -90 {
            self.pitch_degrees -= self.orbit_step;
        }
    }

    fn update_orbit_step(&mut self) {
        self.orbit_step = self.zoom_step.clamp(1, 5);
    }

    fn pose(mut self) -> OriginalCameraPose {
        if self.yaw_degrees > 180 {
            self.yaw_degrees -= 360;
        }
        if self.yaw_degrees < -180 {
            self.yaw_degrees += 360;
        }
        let pitch = (self.pitch_degrees as f32).to_radians();
        let yaw = (self.yaw_degrees as f32).to_radians();
        let (sin_pitch, cos_pitch) = pitch.sin_cos();
        let (sin_yaw, cos_yaw) = yaw.sin_cos();

        // Direct3D retained mode is left-handed and the original meshes use
        // Y-up. Reflecting source Z produces Macroquad's right-handed space.
        let position = vec3(
            -self.distance * sin_pitch * sin_yaw,
            self.distance * sin_pitch * cos_yaw,
            self.distance * cos_pitch,
        );
        let up = vec3(-cos_pitch * sin_yaw, cos_pitch * cos_yaw, -sin_pitch);
        OriginalCameraPose {
            position,
            up,
            field: self.field,
            // Wine's retained-mode compatibility implementation constructs
            // unit-front clip planes at +/-field. This is the corresponding
            // vertical angle for Macroquad's perspective camera.
            fovy_radians: 2.0 * self.field.atan(),
            near: ORIGINAL_CAMERA_NEAR,
            far: self.distance * ORIGINAL_CAMERA_FAR_SCALE,
            pitch_degrees: self.pitch_degrees,
            yaw_degrees: self.yaw_degrees,
            zoom_step: self.zoom_step,
            orbit_step: self.orbit_step,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OriginalTacticalLod {
    Close = 0,
    Medium = 1,
    Far = 2,
}

impl OriginalTacticalLod {
    const fn resource_id(self) -> u32 {
        2560 + self as u32
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Close => "close",
            Self::Medium => "medium",
            Self::Far => "far",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TacticalLodView {
    pub view_depth: f32,
    pub projection_scale: f32,
    pub high_detail: bool,
}

impl TacticalLodView {
    pub const CLOSE_FIXTURE: Self = Self {
        view_depth: 10.0,
        projection_scale: 1.0,
        high_detail: true,
    };
    pub const MEDIUM_FIXTURE: Self = Self {
        view_depth: 25.0,
        projection_scale: 1.0,
        high_detail: true,
    };
    pub const FAR_FIXTURE: Self = Self {
        view_depth: 50.0,
        projection_scale: 1.0,
        high_detail: true,
    };

    /// Test-only bridge from the working tactical zoom control to a source
    /// view-depth range that crosses all three original LOD thresholds.
    pub(crate) fn from_fixture_zoom(zoom: f32) -> Self {
        Self {
            view_depth: 25.0 / zoom,
            projection_scale: 1.0,
            high_detail: true,
        }
    }
}

impl Default for TacticalLodView {
    fn default() -> Self {
        Self::CLOSE_FIXTURE
    }
}

/// Reproduce `FUN_005d3770`: state 0/1/2 maps to base/base+1/base+2.
/// The original divides each threshold by projection scale before comparing
/// view depth. Exact high-detail threshold equality retains the current LOD.
#[must_use]
pub fn select_original_tactical_lod(
    current: OriginalTacticalLod,
    view: TacticalLodView,
) -> OriginalTacticalLod {
    if !view.view_depth.is_finite()
        || !view.projection_scale.is_finite()
        || view.view_depth < 0.0
        || view.projection_scale <= 0.0
    {
        return current;
    }
    if !view.high_detail {
        let medium_depth = ORIGINAL_REDUCED_DETAIL_THRESHOLD / view.projection_scale;
        return if view.view_depth <= medium_depth {
            OriginalTacticalLod::Medium
        } else {
            OriginalTacticalLod::Far
        };
    }
    // Preserve the executable's f32-rounded quotient boundaries. Multiplying
    // depth by scale is algebraically equivalent over real numbers but can
    // choose a different LOD after floating-point rounding.
    let close_depth = ORIGINAL_CLOSE_THRESHOLD / view.projection_scale;
    let medium_depth = ORIGINAL_MEDIUM_THRESHOLD / view.projection_scale;
    if view.view_depth > medium_depth {
        OriginalTacticalLod::Far
    } else if view.view_depth > close_depth && view.view_depth < medium_depth {
        OriginalTacticalLod::Medium
    } else if view.view_depth < close_depth {
        OriginalTacticalLod::Close
    } else {
        current
    }
}

const MESH_MAGIC: &[u8; 8] = b"ORTMESH\0";
const TEXTURE_MAGIC: &[u8; 8] = b"ORTINDEX";
#[cfg(not(target_arch = "wasm32"))]
const MAX_PROOF_OBJECT_BYTES: usize = 8 << 20;

#[derive(Default)]
struct TacticalObjectCache {
    meshes: HashMap<String, Vec<u8>>,
    textures: HashMap<String, Vec<u8>>,
}

static TACTICAL_OBJECT_CACHE: LazyLock<Mutex<TacticalObjectCache>> =
    LazyLock::new(|| Mutex::new(TacticalObjectCache::default()));

/// Install typed tactical runtime objects unpacked from the browser pack.
pub fn set_tactical_asset_cache(
    meshes: HashMap<String, Vec<u8>>,
    textures: HashMap<String, Vec<u8>>,
) {
    *TACTICAL_OBJECT_CACHE.lock().unwrap() = TacticalObjectCache { meshes, textures };
}

/// Install the P57 proof family from an ignored native runtime store.
#[cfg(not(target_arch = "wasm32"))]
pub fn install_native_tactical_lod_family(runtime_root: &Path) -> Result<(), String> {
    use serde::Deserialize;
    use sha2::{Digest, Sha256};

    #[derive(Deserialize)]
    struct Manifest {
        schema_version: u32,
        meshes: Vec<MeshRecord>,
        textures: Vec<TextureRecord>,
    }
    #[derive(Deserialize)]
    struct MeshRecord {
        id: u32,
        language: u32,
        object_sha256: String,
        object: String,
        texture_bindings: Option<Vec<TextureBinding>>,
    }
    #[derive(Deserialize)]
    struct TextureBinding {
        resource_name: String,
        resource_language: u32,
    }
    #[derive(Deserialize)]
    struct TextureRecord {
        identifier_kind: String,
        name: Option<String>,
        language: u32,
        kind: String,
        palette_rule: Option<String>,
        object_sha256: String,
        object: String,
    }

    let manifest_bytes = read_bounded(&runtime_root.join("manifest.json"))?;
    let manifest: Manifest = serde_json::from_slice(&manifest_bytes)
        .map_err(|error| format!("decode tactical runtime manifest: {error}"))?;
    if manifest.schema_version != 1 {
        return Err("unsupported tactical runtime manifest version".to_string());
    }
    fn load_object(
        root: &Path,
        relative: &str,
        digest: &str,
        suffix: &str,
    ) -> Result<Vec<u8>, String> {
        if digest.len() != 64
            || !digest
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
            || relative != format!("objects/{digest}{suffix}")
        {
            return Err("invalid content-addressed tactical object identity".to_string());
        }
        let bytes = read_bounded(&root.join(relative))?;
        let actual = format!("{:x}", Sha256::digest(&bytes));
        if actual != digest {
            return Err("tactical runtime object failed SHA-256 verification".to_string());
        }
        Ok(bytes)
    }

    let expected_meshes = [
        (2560, Some("SDESTI52.BMP")),
        (2561, Some("SDESTI_M.BMP")),
        (2562, None),
    ];
    let mut installed_meshes = HashMap::new();
    for (mesh_id, expected_texture) in expected_meshes {
        let matches: Vec<_> = manifest
            .meshes
            .iter()
            .filter(|record| record.id == mesh_id && record.language == 1033)
            .collect();
        if matches.len() != 1 {
            return Err(format!("tactical runtime lacks unique mesh {mesh_id}/1033"));
        }
        let mesh = matches[0];
        let bindings = mesh.texture_bindings.as_deref().unwrap_or_default();
        match expected_texture {
            Some(texture_name)
                if bindings.len() == 1
                    && bindings[0].resource_name.eq_ignore_ascii_case(texture_name)
                    && bindings[0].resource_language == 1033 => {}
            None if bindings.is_empty() => {}
            Some(texture_name) => {
                return Err(format!("mesh {mesh_id} does not bind {texture_name}/1033"));
            }
            None => return Err(format!("mesh {mesh_id} unexpectedly binds a texture")),
        }
        installed_meshes.insert(
            format!("{mesh_id}/1033"),
            load_object(runtime_root, &mesh.object, &mesh.object_sha256, ".mesh")?,
        );
    }

    let mut installed_textures = HashMap::new();
    for texture_name in ["SDESTI52.BMP", "SDESTI_M.BMP"] {
        let matches: Vec<_> = manifest
            .textures
            .iter()
            .filter(|record| {
                record.identifier_kind == "name"
                    && record
                        .name
                        .as_deref()
                        .is_some_and(|name| name.eq_ignore_ascii_case(texture_name))
                    && record.language == 1033
            })
            .collect();
        if matches.len() != 1 {
            return Err(format!("tactical runtime lacks unique {texture_name}/1033"));
        }
        let texture = matches[0];
        if texture.kind != "indexed_rle" || texture.palette_rule.as_deref() != Some("battle_active")
        {
            return Err(format!(
                "{texture_name} does not retain the active battle-palette rule"
            ));
        }
        installed_textures.insert(
            format!("{texture_name}/1033"),
            load_object(
                runtime_root,
                &texture.object,
                &texture.object_sha256,
                ".texture",
            )?,
        );
    }
    set_tactical_asset_cache(installed_meshes, installed_textures);
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn read_bounded(path: &Path) -> Result<Vec<u8>, String> {
    let file = std::fs::File::open(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let metadata = file
        .metadata()
        .map_err(|error| format!("{}: {error}", path.display()))?;
    if !metadata.is_file() || metadata.len() > MAX_PROOF_OBJECT_BYTES as u64 {
        return Err(format!("{} is not a bounded regular file", path.display()));
    }
    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    file.take(MAX_PROOF_OBJECT_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("{}: {error}", path.display()))?;
    if bytes.len() as u64 != metadata.len() {
        return Err(format!("{} changed while being read", path.display()));
    }
    Ok(bytes)
}

struct TacticalLodAsset {
    resource_id: u32,
    meshes: Vec<Mesh>,
}

/// Lazily allocated GPU state for the source-bound P57 LOD family.
pub(crate) struct TacticalProofRenderer {
    attempted: bool,
    family_loads: usize,
    assets: Vec<TacticalLodAsset>,
    material: Option<Material>,
    current_lod: OriginalTacticalLod,
    view: TacticalLodView,
    logged_lod: Option<OriginalTacticalLod>,
    source_camera: Option<OriginalTacticalCamera>,
    logged_camera: Option<[u32; 8]>,
}

impl Default for TacticalProofRenderer {
    fn default() -> Self {
        Self {
            attempted: false,
            family_loads: 0,
            assets: Vec::new(),
            material: None,
            current_lod: OriginalTacticalLod::Medium,
            view: TacticalLodView::default(),
            logged_lod: None,
            source_camera: None,
            logged_camera: None,
        }
    }
}

impl TacticalProofRenderer {
    pub(crate) fn set_view(&mut self, view: TacticalLodView) {
        self.view = view;
    }

    pub(crate) fn enable_original_camera(&mut self, player_is_empire: bool, battle_extent: f32) {
        self.source_camera = Some(OriginalTacticalCamera::new(player_is_empire, battle_extent));
        self.logged_camera = None;
    }

    pub(crate) fn zoom_in(&mut self) {
        if let Some(camera) = &mut self.source_camera {
            camera.zoom_in();
        }
    }

    pub(crate) fn zoom_out(&mut self) {
        if let Some(camera) = &mut self.source_camera {
            camera.zoom_out();
        }
    }

    pub(crate) fn turn_left(&mut self) {
        if let Some(camera) = &mut self.source_camera {
            camera.turn_left();
        }
    }

    pub(crate) fn turn_right(&mut self) {
        if let Some(camera) = &mut self.source_camera {
            camera.turn_right();
        }
    }

    pub(crate) fn pitch_up(&mut self) {
        if let Some(camera) = &mut self.source_camera {
            camera.pitch_up();
        }
    }

    pub(crate) fn pitch_down(&mut self) {
        if let Some(camera) = &mut self.source_camera {
            camera.pitch_down();
        }
    }

    pub(crate) fn draw(&mut self, bmp_cache: &mut BmpCache, aperture: (f32, f32, f32, f32)) {
        if !self.attempted {
            self.attempted = true;
            if let Err(error) = self.load(bmp_cache) {
                macroquad::logging::warn!("[tactical_3d] LOD family unavailable: {}", error);
            }
        }
        if self.assets.len() != 3 || self.material.is_none() {
            return;
        }

        let prior_lod = self.current_lod;
        self.current_lod = select_original_tactical_lod(self.current_lod, self.view);
        if self.logged_lod != Some(self.current_lod) {
            macroquad::logging::info!(
                "[tactical_3d] lod_selection from={} to={} resource_id={} view_depth={} projection_scale={} high_detail={} family_loads={}",
                prior_lod.label(),
                self.current_lod.label(),
                self.current_lod.resource_id(),
                self.view.view_depth,
                self.view.projection_scale,
                self.view.high_detail,
                self.family_loads,
            );
            self.logged_lod = Some(self.current_lod);
        }

        let (x, y, width, height) = aperture;
        let viewport = (
            x.round() as i32,
            (screen_height() - y - height).round() as i32,
            width.round() as i32,
            height.round() as i32,
        );
        if let Some(source_camera) = self.source_camera {
            let pose = source_camera.pose();
            let camera_key = [
                pose.position.x.to_bits(),
                pose.position.y.to_bits(),
                pose.position.z.to_bits(),
                pose.field.to_bits(),
                pose.pitch_degrees as u32,
                pose.yaw_degrees as u32,
                pose.zoom_step as u32,
                pose.orbit_step as u32,
            ];
            if self.logged_camera != Some(camera_key) {
                macroquad::logging::info!(
                    "[tactical_3d] camera_source pitch={} yaw={} field={} zoom_step={} orbit_step={} distance={} near={} far={} fovy_radians={} handedness=lh_y_up_to_rh_y_up",
                    pose.pitch_degrees,
                    pose.yaw_degrees,
                    pose.field,
                    pose.zoom_step,
                    pose.orbit_step,
                    source_camera.distance,
                    pose.near,
                    pose.far,
                    pose.fovy_radians,
                );
                self.logged_camera = Some(camera_key);
            }
            set_camera(&Camera3D {
                position: pose.position,
                target: Vec3::ZERO,
                up: pose.up,
                fovy: pose.fovy_radians,
                aspect: Some(width / height),
                viewport: Some(viewport),
                z_near: pose.near,
                z_far: pose.far,
                ..Default::default()
            });
        } else {
            let camera_direction = vec3(2.8, -4.2, 2.2).normalize();
            let camera_distance = self.view.view_depth.mul_add(0.24, 2.0);
            set_camera(&Camera3D {
                position: camera_direction * camera_distance,
                target: Vec3::ZERO,
                up: Vec3::Z,
                fovy: 35.0_f32.to_radians(),
                aspect: Some(width / height),
                viewport: Some(viewport),
                z_near: 0.1,
                z_far: 100.0,
                ..Default::default()
            });
        }
        gl_use_material(self.material.as_ref().unwrap());
        let asset = &self.assets[self.current_lod as usize];
        debug_assert_eq!(asset.resource_id, self.current_lod.resource_id());
        for mesh in &asset.meshes {
            draw_mesh(mesh);
        }
        gl_use_default_material();
        set_default_camera();
    }

    fn load(&mut self, bmp_cache: &mut BmpCache) -> Result<(), String> {
        let (mesh_payloads, texture_payloads) = {
            let cache = TACTICAL_OBJECT_CACHE.lock().unwrap();
            let meshes = PROOF_MESH_KEYS
                .iter()
                .map(|key| {
                    cache
                        .meshes
                        .get(*key)
                        .cloned()
                        .ok_or_else(|| format!("typed mesh entry {key} is missing"))
                })
                .collect::<Result<Vec<_>, _>>()?;
            let textures = PROOF_TEXTURE_KEYS
                .iter()
                .map(|key| {
                    cache
                        .textures
                        .get(*key)
                        .cloned()
                        .map(|bytes| ((*key).to_string(), bytes))
                        .ok_or_else(|| format!("typed texture entry {key} is missing"))
                })
                .collect::<Result<Vec<_>, _>>()?;
            (meshes, textures)
        };
        let palette = bmp_cache
            .original_palette_rgba(DllSource::Tactical, resources::tactical::BACKGROUND)
            .ok_or("TACTICAL 1000 active palette is unavailable")?;

        let mut textures = HashMap::new();
        for (key, bytes) in texture_payloads {
            textures.insert(key, decode_indexed_texture(&bytes, &palette)?);
        }
        let white = Texture2D::from_rgba8(1, 1, &[255, 255, 255, 255]);
        white.set_filter(FilterMode::Nearest);

        let mut family_transform = None;
        let mut assets = Vec::with_capacity(3);
        let mut diagnostics = Vec::with_capacity(3);
        for (index, mesh_bytes) in mesh_payloads.iter().enumerate() {
            let resource_id = 2560 + index as u32;
            let mut decoded = decode_mesh_object(mesh_bytes, family_transform)?;
            family_transform = Some(decoded.transform);
            for chunk in &mut decoded.chunks {
                let texture = chunk
                    .texture_name
                    .as_ref()
                    .and_then(|name| textures.get(&format!("{}/1033", name.to_ascii_uppercase())))
                    .cloned()
                    .unwrap_or_else(|| white.clone());
                chunk.mesh.texture = Some(texture);
            }
            let triangles: usize = decoded
                .chunks
                .iter()
                .map(|chunk| chunk.mesh.indices.len() / 3)
                .sum();
            let render_vertices: usize = decoded
                .chunks
                .iter()
                .map(|chunk| chunk.mesh.vertices.len())
                .sum();
            diagnostics.push(format!(
                "{}:{}/{}/{}/{}",
                resource_id,
                decoded.source_vertices,
                decoded.source_faces,
                render_vertices,
                triangles,
            ));
            assets.push(TacticalLodAsset {
                resource_id,
                meshes: decoded.chunks.into_iter().map(|chunk| chunk.mesh).collect(),
            });
        }

        self.material = Some(load_tactical_material()?);
        self.assets = assets;
        self.family_loads = self.family_loads.saturating_add(1);
        macroquad::logging::info!(
            "[tactical_3d] family_loaded base=2560 resources=2560,2561,2562 textures=SDESTI52.BMP,SDESTI_M.BMP palette=tactical-dll/1000 diagnostics={} family_loads={}",
            diagnostics.join(","),
            self.family_loads,
        );
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct MeshTransform {
    center: Vec3,
    scale: f32,
}

struct DecodedMeshChunk {
    mesh: Mesh,
    texture_name: Option<String>,
}

struct DecodedMeshObject {
    chunks: Vec<DecodedMeshChunk>,
    source_vertices: usize,
    source_faces: usize,
    transform: MeshTransform,
}

struct DecodedMaterial {
    diffuse: Color,
    texture_name: Option<String>,
}

fn decode_mesh_object(
    bytes: &[u8],
    family_transform: Option<MeshTransform>,
) -> Result<DecodedMeshObject, String> {
    let mut reader = Reader::new(bytes);
    reader.expect(MESH_MAGIC)?;
    if reader.u32()? != 1 {
        return Err("unsupported tactical mesh object version".to_string());
    }
    let materials = reader.bounded_count(256, "materials")?;
    let chunks = reader.bounded_count(4096, "chunks")?;
    let source_vertices = reader.bounded_count(1_000_000, "source vertices")?;
    let source_faces = reader.bounded_count(1_000_000, "source faces")?;
    if reader.u32()? > 1 {
        return Err("invalid tactical mesh normal mode".to_string());
    }
    let mut bounds = [0.0_f32; 6];
    for value in &mut bounds {
        *value = reader.f32()?;
    }
    let source_center = vec3(
        (bounds[0] + bounds[3]) * 0.5,
        (bounds[1] + bounds[4]) * 0.5,
        (bounds[2] + bounds[5]) * 0.5,
    );
    let extent = (bounds[3] - bounds[0])
        .max(bounds[4] - bounds[1])
        .max(bounds[5] - bounds[2]);
    if !extent.is_finite() || extent <= 0.0 {
        return Err("invalid tactical mesh bounds".to_string());
    }
    let transform = family_transform.unwrap_or(MeshTransform {
        center: source_center,
        scale: 2.0 / extent,
    });
    let mut decoded_materials = Vec::with_capacity(materials);
    for _ in 0..materials {
        let diffuse = [reader.f32()?, reader.f32()?, reader.f32()?, reader.f32()?];
        for _ in 0..7 {
            reader.f32()?;
        }
        let name_len = reader.bounded_count(512, "texture name")?;
        let name = reader.bytes(name_len)?;
        let name = std::str::from_utf8(name).map_err(|_| "invalid tactical texture name")?;
        decoded_materials.push(DecodedMaterial {
            diffuse: Color::new(
                diffuse[0].clamp(0.0, 1.0),
                diffuse[1].clamp(0.0, 1.0),
                diffuse[2].clamp(0.0, 1.0),
                diffuse[3].clamp(0.0, 1.0),
            ),
            texture_name: (!name.is_empty()).then(|| name.to_string()),
        });
    }

    let mut output = Vec::with_capacity(chunks);
    for _ in 0..chunks {
        let material = reader.u32()? as usize;
        if material >= materials {
            return Err("tactical mesh chunk references an invalid material".to_string());
        }
        let vertex_count = reader.bounded_count(u16::MAX as usize + 1, "chunk vertices")?;
        let index_count = reader.bounded_count(3_000_000, "chunk indices")?;
        if vertex_count == 0 || index_count == 0 || index_count % 3 != 0 {
            return Err("invalid tactical mesh chunk size".to_string());
        }
        let decoded_material = &decoded_materials[material];
        let mut vertices = Vec::with_capacity(vertex_count);
        for _ in 0..vertex_count {
            let position = vec3(reader.f32()?, reader.f32()?, reader.f32()?);
            let normal = vec3(reader.f32()?, reader.f32()?, reader.f32()?);
            let uv = vec2(reader.f32()?, reader.f32()?);
            let relative = position - transform.center;
            vertices.push(Vertex {
                position: vec3(relative.x, relative.y, -relative.z) * transform.scale,
                uv,
                color: decoded_material.diffuse.into(),
                normal: vec3(normal.x, normal.y, -normal.z).extend(0.0),
            });
        }
        let mut indices = Vec::with_capacity(index_count);
        for _ in 0..index_count {
            let index = reader.u16()?;
            if usize::from(index) >= vertex_count {
                return Err("tactical mesh chunk index is out of range".to_string());
            }
            indices.push(index);
        }
        for triangle in indices.chunks_exact_mut(3) {
            triangle.swap(1, 2);
        }
        output.push(DecodedMeshChunk {
            mesh: Mesh {
                vertices,
                indices,
                texture: None,
            },
            texture_name: decoded_material.texture_name.clone(),
        });
    }
    if !reader.finished() {
        return Err("tactical mesh object has trailing bytes".to_string());
    }
    Ok(DecodedMeshObject {
        chunks: output,
        source_vertices,
        source_faces,
        transform,
    })
}

fn decode_indexed_texture(bytes: &[u8], palette: &[[u8; 4]; 256]) -> Result<Texture2D, String> {
    let mut reader = Reader::new(bytes);
    reader.expect(TEXTURE_MAGIC)?;
    if reader.u32()? != 1 {
        return Err("unsupported tactical texture object version".to_string());
    }
    let width = reader.u32()?;
    let height = reader.u32()?;
    let palette_id = reader.u32()?;
    let palette_rule = reader.u32()?;
    let pixels = reader.u32()? as usize;
    let trailing = reader.u32()? as usize;
    let expected = (width as usize)
        .checked_mul(height as usize)
        .ok_or("tactical texture dimensions overflow")?;
    if width == 0
        || height == 0
        || width > u16::MAX.into()
        || height > u16::MAX.into()
        || pixels != expected
        || pixels > 16_777_216
        || palette_id != 0
        || palette_rule != 1
        || trailing != 0
    {
        return Err("invalid P56 indexed tactical texture header".to_string());
    }
    let indices = reader.bytes(pixels)?;
    if !reader.finished() {
        return Err("tactical texture object has trailing bytes".to_string());
    }
    let mut rgba = Vec::with_capacity(pixels * 4);
    for index in indices {
        rgba.extend_from_slice(&palette[usize::from(*index)]);
    }
    let texture = Texture2D::from_rgba8(width as u16, height as u16, &rgba);
    texture.set_filter(FilterMode::Nearest);
    Ok(texture)
}

fn load_tactical_material() -> Result<Material, String> {
    let backend = unsafe { get_internal_gl().quad_context.info().backend };
    let shader = match backend {
        Backend::OpenGl => ShaderSource::Glsl {
            vertex: TACTICAL_VERTEX_GLSL,
            fragment: TACTICAL_FRAGMENT_GLSL,
        },
        Backend::Metal => ShaderSource::Msl {
            program: TACTICAL_METAL,
        },
    };
    load_material(
        shader,
        MaterialParams {
            pipeline_params: PipelineParams {
                depth_write: true,
                depth_test: Comparison::LessOrEqual,
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .map_err(|error| format!("compile tactical material: {error}"))
}

struct Reader<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> Reader<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    fn expect(&mut self, magic: &[u8]) -> Result<(), String> {
        if self.bytes(magic.len())? != magic {
            return Err("invalid tactical runtime object magic".to_string());
        }
        Ok(())
    }

    fn bytes(&mut self, count: usize) -> Result<&'a [u8], String> {
        let end = self
            .cursor
            .checked_add(count)
            .ok_or("tactical runtime object offset overflow")?;
        let value = self
            .bytes
            .get(self.cursor..end)
            .ok_or("truncated tactical runtime object")?;
        self.cursor = end;
        Ok(value)
    }

    fn u16(&mut self) -> Result<u16, String> {
        Ok(u16::from_le_bytes(
            self.bytes(2)?.try_into().map_err(|_| "invalid u16")?,
        ))
    }

    fn u32(&mut self) -> Result<u32, String> {
        Ok(u32::from_le_bytes(
            self.bytes(4)?.try_into().map_err(|_| "invalid u32")?,
        ))
    }

    fn f32(&mut self) -> Result<f32, String> {
        let value = f32::from_bits(self.u32()?);
        value
            .is_finite()
            .then_some(value)
            .ok_or("non-finite tactical runtime value".to_string())
    }

    fn bounded_count(&mut self, maximum: usize, label: &str) -> Result<usize, String> {
        let count = self.u32()? as usize;
        if count > maximum {
            return Err(format!("tactical {label} count exceeds {maximum}"));
        }
        Ok(count)
    }

    fn finished(&self) -> bool {
        self.cursor == self.bytes.len()
    }
}

const TACTICAL_VERTEX_GLSL: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
attribute vec4 normal;
varying lowp vec2 uv;
varying lowp float light;
varying lowp vec4 tint;
uniform mat4 Model;
uniform mat4 Projection;
void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
    tint = color0 / 255.0;
    light = 0.28 + 0.72 * max(dot(normalize(normal.xyz), normalize(vec3(-0.35, -0.5, 0.8))), 0.0);
}"#;

const TACTICAL_FRAGMENT_GLSL: &str = r#"#version 100
varying lowp vec2 uv;
varying lowp float light;
varying lowp vec4 tint;
uniform sampler2D Texture;
void main() {
    lowp vec4 texel = texture2D(Texture, uv);
    gl_FragColor = vec4(texel.rgb * tint.rgb * light, texel.a * tint.a);
}"#;

const TACTICAL_METAL: &str = r#"
#include <metal_stdlib>
using namespace metal;
struct Uniforms { float4x4 Model; float4x4 Projection; float4 _Time; };
struct Vertex {
    float3 position [[attribute(0)]];
    float2 texcoord [[attribute(1)]];
    float4 color0 [[attribute(2)]];
    float4 normal [[attribute(3)]];
};
struct RasterizerData {
    float4 position [[position]];
    float2 uv [[user(locn0)]];
    float light [[user(locn1)]];
    float4 tint [[user(locn2)]];
};
vertex RasterizerData vertexShader(Vertex v [[stage_in]], constant Uniforms& u [[buffer(0)]]) {
    RasterizerData out;
    out.position = u.Projection * u.Model * float4(v.position, 1.0);
    out.uv = v.texcoord;
    out.tint = v.color0 / 255.0;
    out.light = 0.28 + 0.72 * max(dot(normalize(v.normal.xyz), normalize(float3(-0.35, -0.5, 0.8))), 0.0);
    return out;
}
fragment float4 fragmentShader(RasterizerData in [[stage_in]], texture2d<float> Texture [[texture(0)]], sampler TextureSmplr [[sampler(0)]]) {
    float4 texel = Texture.sample(TextureSmplr, in.uv);
    return float4(texel.rgb * in.tint.rgb * in.light, texel.a * in.tint.a);
}"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_near(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 1.0e-5,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn reader_rejects_non_finite_and_truncated_values() {
        let encoded_nan = f32::NAN.to_bits().to_le_bytes();
        let mut non_finite = Reader::new(&encoded_nan);
        assert!(non_finite.f32().is_err());
        let mut truncated = Reader::new(&[1, 2, 3]);
        assert!(truncated.u32().is_err());
    }

    #[test]
    fn original_camera_preserves_faction_pose_clip_and_field_contract() {
        let alliance = OriginalTacticalCamera::new(false, 100.0).pose();
        let empire = OriginalTacticalCamera::new(true, 100.0).pose();
        assert_eq!(alliance.pitch_degrees, 30);
        assert_eq!(alliance.yaw_degrees, -30);
        assert_eq!(empire.yaw_degrees, 150);
        assert_near(alliance.position.x, -empire.position.x);
        assert_near(alliance.position.y, -empire.position.y);
        assert_near(alliance.position.z, empire.position.z);
        assert_near(alliance.near, 1.0);
        assert_near(alliance.far, 425.0);
        assert_near(alliance.field, 0.2);
        assert_near(alliance.fovy_radians, 2.0 * 0.2_f32.atan());
        assert_near(alliance.up.length(), 1.0);
    }

    #[test]
    fn original_camera_commands_use_field_zoom_and_adaptive_orbit_steps() {
        let mut camera = OriginalTacticalCamera::new(false, 100.0);
        camera.zoom_in();
        assert_near(camera.field, 0.18);
        assert_eq!(camera.zoom_step, 4);
        assert_eq!(camera.orbit_step, 4);
        camera.turn_left();
        assert_eq!(camera.yaw_degrees, -34);
        camera.turn_right();
        assert_eq!(camera.yaw_degrees, -30);
        camera.zoom_out();
        assert_near(camera.field, 0.198);
        assert_eq!(camera.zoom_step, 5);
        assert_eq!(camera.orbit_step, 5);
        camera.pitch_up();
        assert_eq!(camera.pitch_degrees, 35);
        camera.pitch_down();
        assert_eq!(camera.pitch_degrees, 30);

        for _ in 0..100 {
            camera.zoom_in();
        }
        assert_near(camera.field, ORIGINAL_CAMERA_FIELD_MIN);
        assert_eq!(camera.orbit_step, 1);
        for _ in 0..200 {
            camera.zoom_out();
        }
        assert_near(camera.field, ORIGINAL_CAMERA_FIELD_MAX);
        assert_eq!(camera.orbit_step, 5);
    }

    #[test]
    fn original_lod_rule_preserves_threshold_edges_and_reduced_detail_mode() {
        let close = TacticalLodView::CLOSE_FIXTURE;
        let medium = TacticalLodView::MEDIUM_FIXTURE;
        let far = TacticalLodView::FAR_FIXTURE;
        assert_eq!(
            select_original_tactical_lod(OriginalTacticalLod::Medium, close),
            OriginalTacticalLod::Close
        );
        assert_eq!(
            select_original_tactical_lod(OriginalTacticalLod::Close, medium),
            OriginalTacticalLod::Medium
        );
        assert_eq!(
            select_original_tactical_lod(OriginalTacticalLod::Medium, far),
            OriginalTacticalLod::Far
        );
        for threshold in [ORIGINAL_CLOSE_THRESHOLD, ORIGINAL_MEDIUM_THRESHOLD] {
            let view = TacticalLodView {
                view_depth: threshold,
                projection_scale: 1.0,
                high_detail: true,
            };
            for current in [
                OriginalTacticalLod::Close,
                OriginalTacticalLod::Medium,
                OriginalTacticalLod::Far,
            ] {
                assert_eq!(select_original_tactical_lod(current, view), current);
            }
        }
        fn immediately_below(value: f32) -> f32 {
            f32::from_bits(value.to_bits() - 1)
        }
        fn immediately_above(value: f32) -> f32 {
            f32::from_bits(value.to_bits() + 1)
        }

        // Non-unit scales guard the executable's divide-then-round sequence.
        // A multiply-first implementation fails at least the 0.3 close edge.
        for projection_scale in [0.3_f32, 2.3_f32] {
            let close_edge = ORIGINAL_CLOSE_THRESHOLD / projection_scale;
            let medium_edge = ORIGINAL_MEDIUM_THRESHOLD / projection_scale;
            for (edge, below, above) in [
                (
                    close_edge,
                    OriginalTacticalLod::Close,
                    OriginalTacticalLod::Medium,
                ),
                (
                    medium_edge,
                    OriginalTacticalLod::Medium,
                    OriginalTacticalLod::Far,
                ),
            ] {
                let at_edge = TacticalLodView {
                    view_depth: edge,
                    projection_scale,
                    high_detail: true,
                };
                assert_eq!(
                    select_original_tactical_lod(OriginalTacticalLod::Far, at_edge),
                    OriginalTacticalLod::Far
                );
                assert_eq!(
                    select_original_tactical_lod(
                        OriginalTacticalLod::Far,
                        TacticalLodView {
                            view_depth: immediately_below(edge),
                            ..at_edge
                        },
                    ),
                    below
                );
                assert_eq!(
                    select_original_tactical_lod(
                        OriginalTacticalLod::Close,
                        TacticalLodView {
                            view_depth: immediately_above(edge),
                            ..at_edge
                        },
                    ),
                    above
                );
            }

            let reduced_edge = ORIGINAL_REDUCED_DETAIL_THRESHOLD / projection_scale;
            for (view_depth, expected) in [
                (immediately_below(reduced_edge), OriginalTacticalLod::Medium),
                (reduced_edge, OriginalTacticalLod::Medium),
                (immediately_above(reduced_edge), OriginalTacticalLod::Far),
            ] {
                assert_eq!(
                    select_original_tactical_lod(
                        OriginalTacticalLod::Close,
                        TacticalLodView {
                            view_depth,
                            projection_scale,
                            high_detail: false,
                        },
                    ),
                    expected
                );
            }
        }

        let mut current = OriginalTacticalLod::Close;
        for (zoom, expected) in [
            (2.0, OriginalTacticalLod::Close),
            (1.6, OriginalTacticalLod::Medium),
            (0.524_288, OriginalTacticalLod::Far),
            (0.655_36, OriginalTacticalLod::Medium),
            (2.0, OriginalTacticalLod::Close),
        ] {
            current =
                select_original_tactical_lod(current, TacticalLodView::from_fixture_zoom(zoom));
            assert_eq!(current, expected);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "requires the ignored runtime store generated from an owned TACTICAL.DLL"]
    fn owned_native_lod_family_installs_with_exact_typed_keys() {
        let runtime = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../data/base/ui/tactical-dll/TACTICAL3D/runtime");
        install_native_tactical_lod_family(&runtime)
            .expect("install owned P57 tactical LOD family");
        let cache = TACTICAL_OBJECT_CACHE.lock().unwrap();
        assert_eq!(cache.meshes.len(), PROOF_MESH_KEYS.len());
        assert_eq!(cache.textures.len(), PROOF_TEXTURE_KEYS.len());
        for key in PROOF_MESH_KEYS {
            assert!(cache.meshes.contains_key(key));
        }
        for key in PROOF_TEXTURE_KEYS {
            assert!(cache.textures.contains_key(key));
        }
    }
}
