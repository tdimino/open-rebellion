//! Decoding and rendering for the original tactical type-301/type-303 assets.
//!
//! P56 established one source-bound render proof. P57 installs its original
//! three-resource LOD family and reproduces the executable's selection rule.
//! No DAT identity is inferred from the resource ordinal.

use std::collections::{HashMap, HashSet};

#[cfg(not(target_arch = "wasm32"))]
use std::{io::Read, path::Path};

use macroquad::camera::Camera;
use macroquad::prelude::*;
use macroquad::window::miniquad::{
    Backend, Comparison, CullFace, FrontFaceOrder, PipelineParams, UniformDesc, UniformType,
};

#[cfg(not(target_arch = "wasm32"))]
use crate::tactical_asset_cache::set_tactical_asset_cache;
use crate::tactical_asset_cache::TACTICAL_OBJECT_CACHE;
use crate::tactical_view::OriginalTacticalLayout;

#[cfg(feature = "interface-test-fixtures")]
pub const PROOF_MESH_KEYS: [&str; 3] = ["2560/1033", "2561/1033", "2562/1033"];
#[cfg(feature = "interface-test-fixtures")]
pub const PROOF_TEXTURE_KEYS: [&str; 2] = ["SDESTI52.BMP/1033", "SDESTI_M.BMP/1033"];
#[cfg(all(feature = "interface-test-fixtures", not(target_arch = "wasm32")))]
pub const TACTICAL_PALETTE_FIRST: u32 = 5531;
#[cfg(all(feature = "interface-test-fixtures", not(target_arch = "wasm32")))]
pub const TACTICAL_PALETTE_LAST: u32 = 5557;

const ORIGINAL_CLOSE_THRESHOLD: f32 = 15.0;
const ORIGINAL_MEDIUM_THRESHOLD: f32 = 40.0;
const ORIGINAL_REDUCED_DETAIL_THRESHOLD: f32 = 20.0;
const ORIGINAL_FIGHTER_CLOSE_THRESHOLD: f32 = 5.0;
const ORIGINAL_FIGHTER_INDICATOR_THRESHOLD: f32 = 10.0;

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

// `FUN_005d4d10` creates a D3DRMLIGHT_DIRECTIONAL at RGB 0.8 on a frame at
// (5, 5, -1), aims that frame at the scene origin with D3DRMCONSTRAIN_Z, and
// adds a separate RGB 0.5 ambient light to the scene. Direct3D's lighting
// equation negates the directional-light ray vector to obtain the direction
// from the surface to the light. Reflecting source Z at our handedness
// boundary therefore produces the vector (5, 5, 1).
const ORIGINAL_LIGHT_FRAME_SOURCE_POSITION: [f32; 3] = [5.0, 5.0, -1.0];
const ORIGINAL_AMBIENT_LIGHT_RGB: f32 = 0.5;
const ORIGINAL_DIRECTIONAL_LIGHT_RGB: f32 = 0.8;
#[cfg(any(feature = "interface-test-fixtures", test))]
const ORIGINAL_DEVICE_DITHER: bool = false;
#[cfg(any(feature = "interface-test-fixtures", test))]
const ORIGINAL_SPECULAR_ENABLED: bool = false;

fn original_tactical_texture_filter() -> FilterMode {
    // `FUN_005c1c10` initializes the retained-mode device without calling
    // SetTextureQuality. The Direct3D no-filter path is nearest-point sampling.
    FilterMode::Nearest
}

fn original_tactical_pipeline_params() -> PipelineParams {
    PipelineParams {
        // Direct3D's default D3DCULL_CCW treats clockwise facets as front
        // faces. The handedness reflection and index reversal in
        // `decode_mesh_object` preserve that source winding.
        cull_face: CullFace::Back,
        front_face_order: FrontFaceOrder::Clockwise,
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    }
}

fn original_surface_to_light_direction() -> Vec3 {
    authored_position(vec3(
        ORIGINAL_LIGHT_FRAME_SOURCE_POSITION[0],
        ORIGINAL_LIGHT_FRAME_SOURCE_POSITION[1],
        ORIGINAL_LIGHT_FRAME_SOURCE_POSITION[2],
    ))
    .normalize()
}

#[cfg(test)]
fn original_tactical_light_factor(normal: Vec3) -> f32 {
    let diffuse = normal
        .normalize()
        .dot(original_surface_to_light_direction())
        .max(0.0);
    (ORIGINAL_AMBIENT_LIGHT_RGB + ORIGINAL_DIRECTIONAL_LIGHT_RGB * diffuse).min(1.0)
}

/// Source-traced tactical camera state from `FUN_005d9490`, `FUN_005d9620`,
/// `FUN_005d9640`, `FUN_00595be0`, `FUN_005c1080`, and the command switch at
/// `0x005d97c0`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OriginalTacticalCamera {
    pitch_degrees: i32,
    yaw_degrees: i32,
    zoom_step: i32,
    orbit_step: i32,
    field: f32,
    distance: f32,
    target_object_id: Option<u32>,
    target: Vec3,
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
    target_object_id: Option<u32>,
    target: Vec3,
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
            target_object_id: None,
            target: Vec3::ZERO,
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

    /// Reproduce camera command 9 after object/frame resolution: cache the
    /// current tactical object identity and look at its resolved world point.
    pub(crate) fn focus_target(&mut self, target_object_id: u32, target: Vec3) {
        self.target_object_id = Some(target_object_id);
        self.target = target;
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
            target_object_id: self.target_object_id,
            target: self.target,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OriginalTacticalLod {
    Close = 0,
    Medium = 1,
    Far = 2,
}

/// Original retained-mode fighter sprite state selected by `0x005d4af0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OriginalFighterDetail {
    Close = 0,
    Far = 1,
    Indicator = 2,
}

/// Reproduce the original fighter sprite selection at apparent view spans 5
/// and 10. The constructor starts every group in the far state.
#[must_use]
pub fn select_original_fighter_detail(
    current: OriginalFighterDetail,
    view_span: f32,
    high_detail: bool,
) -> OriginalFighterDetail {
    if !view_span.is_finite() || view_span < 0.0 {
        return current;
    }
    if view_span >= ORIGINAL_FIGHTER_INDICATOR_THRESHOLD {
        OriginalFighterDetail::Indicator
    } else if !high_detail || view_span >= ORIGINAL_FIGHTER_CLOSE_THRESHOLD {
        OriginalFighterDetail::Far
    } else {
        OriginalFighterDetail::Close
    }
}

impl OriginalTacticalLod {
    #[cfg(feature = "interface-test-fixtures")]
    const fn resource_id(self) -> u32 {
        2560 + self as u32
    }

    #[cfg(feature = "interface-test-fixtures")]
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
    #[cfg(any(feature = "interface-test-fixtures", test))]
    pub const CLOSE_FIXTURE: Self = Self {
        view_depth: 10.0,
        projection_scale: 1.0,
        high_detail: true,
    };
    #[cfg(any(feature = "interface-test-fixtures", test))]
    pub const MEDIUM_FIXTURE: Self = Self {
        view_depth: 25.0,
        projection_scale: 1.0,
        high_detail: true,
    };
    #[cfg(any(feature = "interface-test-fixtures", test))]
    pub const FAR_FIXTURE: Self = Self {
        view_depth: 50.0,
        projection_scale: 1.0,
        high_detail: true,
    };

    /// Test-only bridge from the working tactical zoom control to a source
    /// view-depth range that crosses all three original LOD thresholds.
    #[cfg(any(feature = "interface-test-fixtures", test))]
    pub(crate) fn from_fixture_zoom(zoom: f32) -> Self {
        Self {
            view_depth: 25.0 / zoom,
            projection_scale: 1.0,
            high_detail: true,
        }
    }
}

#[cfg(any(feature = "interface-test-fixtures", test))]
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
const PALETTE_MAGIC: &[u8; 8] = b"ORTPAL00";
#[cfg(not(target_arch = "wasm32"))]
const MAX_PROOF_OBJECT_BYTES: usize = 8 << 20;

#[cfg(not(target_arch = "wasm32"))]
const EXPECTED_MESH_FAMILY_BASES: [u32; 29] = [
    2010, 2020, 2030, 2040, 2050, 2060, 2070, 2080, 2090, 2100, 2110, 2120, 2130, 2140, 2150, 2510,
    2520, 2530, 2540, 2550, 2560, 2570, 2580, 2590, 2600, 2610, 2620, 2630, 2640,
];

#[cfg(not(target_arch = "wasm32"))]
const EXPECTED_NUMERIC_TEXTURE_RANGES: [(u32, u32); 25] = [
    (3020, 3025),
    (3040, 3045),
    (3060, 3065),
    (3080, 3086),
    (3100, 3106),
    (3120, 3126),
    (3140, 3145),
    (3160, 3165),
    (3180, 3185),
    (3200, 3215),
    (3220, 3235),
    (3240, 3255),
    (3260, 3266),
    (3280, 3286),
    (3300, 3306),
    (3320, 3335),
    (3340, 3355),
    (3360, 3375),
    (3520, 3527),
    (3620, 3627),
    (4000, 4039),
    (4100, 4139),
    (4200, 4204),
    (5501, 5527),
    (5531, 5557),
];

#[cfg(not(target_arch = "wasm32"))]
const EXPECTED_NUMERIC_TEXTURE_SINGLETONS: [u32; 9] =
    [3500, 3510, 3600, 3610, 4044, 4049, 5010, 5020, 5030];

#[cfg(not(target_arch = "wasm32"))]
const EXPECTED_NAMED_TEXTURES: [&str; 59] = [
    "ASSFRG52.BMP",
    "ASSFRG_M.BMP",
    "ATMAP.BMP",
    "ATMAP_M.BMP",
    "A_ESCR.BMP",
    "A_ESCR_M.BMP",
    "BCRSER52.BMP",
    "BCRSER_M.BMP",
    "BULK52.BMP",
    "BULK_M.BMP",
    "CARRAK52.BMP",
    "CARRAK_M.BMP",
    "CORVET52.BMP",
    "CORVET_M.BMP",
    "ESCARRIM.BMP",
    "ESCARR_I.BMP",
    "E_DRED52.BMP",
    "E_DRED_M.BMP",
    "GLMAP128.BMP",
    "GLMAP_M.BMP",
    "GUNSHIP.BMP",
    "GUNSHP_M.BMP",
    "INTERD.BMP",
    "INTERD_M.BMP",
    "LANCER52.BMP",
    "LANCER_M.BMP",
    "MONCAL52.BMP",
    "MONCAL_M.BMP",
    "NEBUL52.BMP",
    "NEBUL_M.BMP",
    "R1MAP.BMP",
    "R1MAP_M.BMP",
    "R2MAP.BMP",
    "R2MAP_M.BMP",
    "R3MAP2.BMP",
    "R3MAP2_M.BMP",
    "R4MAP.BMP",
    "R4MAP_M.BMP",
    "R5MAP.BMP",
    "R5MAP_M.BMP",
    "R_DRED52.BMP",
    "R_DRED_M.BMP",
    "SDESTI52.BMP",
    "SDESTI_M.BMP",
    "SDESTV52.BMP",
    "SDESTV_M.BMP",
    "SDSTI252.BMP",
    "SDSTI2_M.BMP",
    "SDSTV252.BMP",
    "SDSTV2_M.BMP",
    "SPDEST24.BMP",
    "SPDEST52.BMP",
    "SPDEST_S.BMP",
    "STRGAL.BMP",
    "STRGAL_M.BMP",
    "STRIKE52.BMP",
    "STRIKE_M.BMP",
    "TUNA52.BMP",
    "TUNA_M.BMP",
];

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, serde::Deserialize)]
struct NativeManifest {
    schema_version: u32,
    meshes: Vec<NativeMeshRecord>,
    textures: Vec<NativeTextureRecord>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, serde::Deserialize)]
struct NativeMeshRecord {
    id: u32,
    language: u32,
    source_sha256: String,
    object_sha256: String,
    object: String,
    vertices: usize,
    faces: usize,
    triangles: usize,
    chunks: usize,
    materials: usize,
    texture_bindings: Option<Vec<NativeTextureBinding>>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, serde::Deserialize)]
struct NativeTextureBinding {
    x_filename: String,
    resource_name: String,
    resource_language: u32,
    resource_source_sha256: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, serde::Deserialize)]
struct NativeTextureRecord {
    identifier_kind: String,
    #[serde(default)]
    id: u32,
    name: Option<String>,
    language: u32,
    kind: String,
    #[serde(default)]
    width: u32,
    #[serde(default)]
    height: u32,
    palette_rule: Option<String>,
    source_sha256: String,
    object_sha256: String,
    object: String,
}

#[cfg(not(target_arch = "wasm32"))]
fn valid_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

#[cfg(not(target_arch = "wasm32"))]
fn expected_mesh_ids() -> HashSet<u32> {
    EXPECTED_MESH_FAMILY_BASES
        .iter()
        .flat_map(|base| [*base, *base + 1, *base + 2])
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
fn expected_numeric_texture_ids() -> HashSet<u32> {
    let mut ids: HashSet<_> = EXPECTED_NUMERIC_TEXTURE_RANGES
        .iter()
        .flat_map(|(first, last)| *first..=*last)
        .collect();
    ids.extend(EXPECTED_NUMERIC_TEXTURE_SINGLETONS);
    ids
}

#[cfg(not(target_arch = "wasm32"))]
fn validate_native_manifest(manifest: &NativeManifest) -> Result<(), String> {
    if manifest.schema_version != 1 {
        return Err("unsupported tactical runtime manifest version".to_string());
    }
    let expected_meshes = expected_mesh_ids();
    let actual_meshes: HashSet<_> = manifest.meshes.iter().map(|record| record.id).collect();
    if manifest.meshes.len() != expected_meshes.len() || actual_meshes != expected_meshes {
        return Err(
            "tactical runtime mesh identities do not match the original corpus".to_string(),
        );
    }
    let expected_numeric = expected_numeric_texture_ids();
    let expected_named: HashSet<_> = EXPECTED_NAMED_TEXTURES.iter().copied().collect();
    let actual_numeric: HashSet<_> = manifest
        .textures
        .iter()
        .filter(|record| record.identifier_kind == "id")
        .map(|record| record.id)
        .collect();
    let actual_named: HashSet<_> = manifest
        .textures
        .iter()
        .filter(|record| record.identifier_kind == "name")
        .filter_map(|record| record.name.as_deref())
        .collect();
    if manifest.textures.len() != expected_numeric.len() + expected_named.len()
        || actual_numeric != expected_numeric
        || actual_named != expected_named
    {
        return Err(
            "tactical runtime texture identities do not match the original corpus".to_string(),
        );
    }

    let named_sources: HashMap<_, _> = manifest
        .textures
        .iter()
        .filter_map(|record| {
            record
                .name
                .as_deref()
                .map(|name| (name, record.source_sha256.as_str()))
        })
        .collect();
    for mesh in &manifest.meshes {
        if mesh.language != 1033
            || !valid_sha256(&mesh.source_sha256)
            || mesh.vertices == 0
            || mesh.faces == 0
            || mesh.faces != mesh.triangles
            || mesh.chunks == 0
            || mesh.materials == 0
        {
            return Err(format!("invalid tactical mesh metadata {}/1033", mesh.id));
        }
        for binding in mesh.texture_bindings.as_deref().unwrap_or_default() {
            if binding.resource_language != 1033
                || !binding
                    .x_filename
                    .eq_ignore_ascii_case(&binding.resource_name)
                || named_sources.get(binding.resource_name.as_str()).copied()
                    != Some(binding.resource_source_sha256.as_str())
            {
                return Err(format!(
                    "invalid tactical mesh texture binding {}/1033",
                    mesh.id
                ));
            }
        }
    }
    for texture in &manifest.textures {
        if texture.language != 1033 || !valid_sha256(&texture.source_sha256) {
            return Err("invalid tactical texture metadata".to_string());
        }
        let palette = texture.identifier_kind == "id" && (5531..=5557).contains(&texture.id);
        let expected_rule = if palette {
            None
        } else if texture.identifier_kind == "id" && (5501..=5527).contains(&texture.id) {
            Some("planet_pair")
        } else {
            Some("battle_active")
        };
        let expected_kind = if palette {
            "palette_rgb24"
        } else {
            "indexed_rle"
        };
        if texture.kind != expected_kind
            || texture.palette_rule.as_deref() != expected_rule
            || (!palette && (texture.width == 0 || texture.height == 0))
            || (palette && (texture.width != 0 || texture.height != 0))
        {
            return Err("invalid tactical texture kind or palette metadata".to_string());
        }
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn read_native_manifest(runtime_root: &Path) -> Result<NativeManifest, String> {
    let manifest_bytes = read_bounded(&runtime_root.join("manifest.json"))?;
    let manifest: NativeManifest = serde_json::from_slice(&manifest_bytes)
        .map_err(|error| format!("decode tactical runtime manifest: {error}"))?;
    validate_native_manifest(&manifest)?;
    Ok(manifest)
}

#[cfg(not(target_arch = "wasm32"))]
fn load_native_object(
    root: &Path,
    relative: &str,
    digest: &str,
    suffix: &str,
) -> Result<Vec<u8>, String> {
    use sha2::{Digest, Sha256};

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

/// Install the complete verified tactical runtime corpus for native play.
#[cfg(not(target_arch = "wasm32"))]
pub fn install_native_tactical_assets(runtime_root: &Path) -> Result<(), String> {
    let manifest = read_native_manifest(runtime_root)?;
    let mut installed_meshes = HashMap::with_capacity(manifest.meshes.len());
    for mesh in &manifest.meshes {
        let key = format!("{}/{}", mesh.id, mesh.language);
        if installed_meshes.contains_key(&key) {
            return Err(format!("duplicate tactical mesh {key}"));
        }
        let bytes = load_native_object(runtime_root, &mesh.object, &mesh.object_sha256, ".mesh")?;
        let decoded = decode_mesh_object(&bytes)?;
        let triangles: usize = decoded
            .chunks
            .iter()
            .map(|chunk| chunk.mesh.indices.len() / 3)
            .sum();
        let texture_names: HashSet<_> = decoded
            .chunks
            .iter()
            .filter_map(|chunk| chunk.texture_name.as_deref())
            .map(str::to_ascii_uppercase)
            .collect();
        let manifest_texture_names: HashSet<_> = mesh
            .texture_bindings
            .as_deref()
            .unwrap_or_default()
            .iter()
            .map(|binding| binding.resource_name.clone())
            .collect();
        if decoded.source_vertices != mesh.vertices
            || decoded.source_faces != mesh.faces
            || triangles != mesh.triangles
            || decoded.chunks.len() != mesh.chunks
            || decoded.materials != mesh.materials
            || texture_names != manifest_texture_names
        {
            return Err(format!("tactical mesh object metadata mismatch {key}"));
        }
        installed_meshes.insert(key, bytes);
    }

    let mut installed_textures = HashMap::with_capacity(manifest.textures.len());
    for texture in &manifest.textures {
        let identifier = match texture.identifier_kind.as_str() {
            "id" if texture.id > 0 => texture.id.to_string(),
            "name" => texture
                .name
                .as_deref()
                .filter(|name| !name.is_empty())
                .map(str::to_ascii_uppercase)
                .ok_or("named tactical texture lacks a name")?,
            _ => return Err("invalid tactical texture identity".to_string()),
        };
        let key = format!("{identifier}/{}", texture.language);
        if installed_textures.contains_key(&key) {
            return Err(format!("duplicate tactical texture {key}"));
        }
        let bytes = load_native_object(
            runtime_root,
            &texture.object,
            &texture.object_sha256,
            ".texture",
        )?;
        if texture.kind == "palette_rgb24" {
            decode_palette_object(&bytes, texture.id)?;
        } else {
            validate_indexed_texture_object(&bytes, texture)?;
        }
        installed_textures.insert(key, bytes);
    }
    set_tactical_asset_cache(installed_meshes, installed_textures);
    Ok(())
}

/// Install only the historical P57 proof subset in fixture tests.
#[cfg(all(feature = "interface-test-fixtures", not(target_arch = "wasm32")))]
pub fn install_native_tactical_lod_family(runtime_root: &Path) -> Result<(), String> {
    let manifest = read_native_manifest(runtime_root)?;
    let mut installed_meshes = HashMap::new();
    for mesh_id in [2560, 2561, 2562] {
        let mesh = manifest
            .meshes
            .iter()
            .find(|record| record.id == mesh_id && record.language == 1033)
            .ok_or_else(|| format!("tactical runtime lacks mesh {mesh_id}/1033"))?;
        installed_meshes.insert(
            format!("{mesh_id}/1033"),
            load_native_object(runtime_root, &mesh.object, &mesh.object_sha256, ".mesh")?,
        );
    }

    let mut installed_textures = HashMap::new();
    for texture in &manifest.textures {
        let key = match (texture.identifier_kind.as_str(), texture.name.as_deref()) {
            ("name", Some(name))
                if ["SDESTI52.BMP", "SDESTI_M.BMP"]
                    .iter()
                    .any(|expected| name.eq_ignore_ascii_case(expected)) =>
            {
                format!("{}/{}", name.to_ascii_uppercase(), texture.language)
            }
            ("id", _) if (TACTICAL_PALETTE_FIRST..=TACTICAL_PALETTE_LAST).contains(&texture.id) => {
                format!("{}/{}", texture.id, texture.language)
            }
            _ => continue,
        };
        installed_textures.insert(
            key,
            load_native_object(
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
    meshes: Vec<TacticalMeshChunk>,
}

impl TacticalLodAsset {
    fn has_drawable_geometry(&self) -> bool {
        !self.meshes.is_empty()
            && self
                .meshes
                .iter()
                .all(|chunk| chunk.mesh.vertices.len() >= 3 && chunk.mesh.indices.len() >= 3)
    }
}

struct TacticalMeshChunk {
    mesh: Mesh,
    emissive: [f32; 3],
}

struct TacticalFighterAsset {
    close_resource_id: u32,
    close: Texture2D,
    far_resource_id: u32,
    far: Texture2D,
    indicator_resource_id: u32,
    indicator: Texture2D,
}

struct TacticalPlanetAsset {
    resource_id: u32,
    palette_resource_id: u32,
    texture: Texture2D,
}

impl TacticalFighterAsset {
    fn selected(&self, detail: OriginalFighterDetail) -> (u32, &Texture2D) {
        match detail {
            OriginalFighterDetail::Close => (self.close_resource_id, &self.close),
            OriginalFighterDetail::Far => (self.far_resource_id, &self.far),
            OriginalFighterDetail::Indicator => (self.indicator_resource_id, &self.indicator),
        }
    }
}

/// One source-bound capital-ship instance submitted by the live battle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TacticalRenderObject {
    pub object_id: u32,
    pub resource_base: u32,
    pub position: Vec3,
}

/// One source-bound fighter group submitted by the live battle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TacticalFighterRenderObject {
    pub object_id: u32,
    pub close_resource_id: u32,
    pub far_resource_id: u32,
    pub indicator_resource_id: u32,
    pub position: Vec3,
}

/// Screen-space footprint produced by the source tactical camera.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TacticalScreenProjection {
    pub object_id: u32,
    pub center: Vec2,
    pub min: Vec2,
    pub max: Vec2,
}

impl TacticalScreenProjection {
    #[must_use]
    pub(crate) fn contains(self, point: Vec2, minimum_radius: f32) -> bool {
        let min = self
            .min
            .min(self.center - vec2(minimum_radius, minimum_radius));
        let max = self
            .max
            .max(self.center + vec2(minimum_radius, minimum_radius));
        point.x >= min.x && point.x <= max.x && point.y >= min.y && point.y <= max.y
    }
}

/// Capital ships successfully submitted through the authentic 3D path.
#[derive(Debug, Default)]
pub(crate) struct TacticalDrawReport {
    pub rendered_object_ids: Vec<u32>,
    pub screen_positions: Vec<(u32, Vec2)>,
    pub projections: Vec<TacticalScreenProjection>,
}

/// Lazily allocated GPU state shared by production participants and fixture proofs.
pub(crate) struct TacticalAssetRenderer {
    #[cfg(feature = "interface-test-fixtures")]
    attempted: bool,
    family_loads: usize,
    #[cfg(feature = "interface-test-fixtures")]
    assets: Vec<TacticalLodAsset>,
    participant_families: HashMap<u32, Vec<TacticalLodAsset>>,
    participant_lods: HashMap<u32, OriginalTacticalLod>,
    unavailable_families: HashSet<u32>,
    fighter_families: HashMap<u32, TacticalFighterAsset>,
    fighter_details: HashMap<u32, OriginalFighterDetail>,
    unavailable_fighter_families: HashSet<u32>,
    planet: Option<TacticalPlanetAsset>,
    unavailable_planet: Option<u32>,
    material: Option<Material>,
    #[cfg(feature = "interface-test-fixtures")]
    current_lod: OriginalTacticalLod,
    #[cfg(feature = "interface-test-fixtures")]
    view: TacticalLodView,
    #[cfg(feature = "interface-test-fixtures")]
    logged_lod: Option<OriginalTacticalLod>,
    source_camera: Option<OriginalTacticalCamera>,
    logged_camera: Option<[u32; 12]>,
    source_layout: Option<OriginalTacticalLayout>,
    logged_layout: Option<OriginalTacticalLayout>,
    palette_selector: u8,
    logged_participant_scene: bool,
    logged_fighter_scene: Option<String>,
    logged_planet_scene: Option<u32>,
}

impl Default for TacticalAssetRenderer {
    fn default() -> Self {
        Self {
            #[cfg(feature = "interface-test-fixtures")]
            attempted: false,
            family_loads: 0,
            #[cfg(feature = "interface-test-fixtures")]
            assets: Vec::new(),
            participant_families: HashMap::new(),
            participant_lods: HashMap::new(),
            unavailable_families: HashSet::new(),
            fighter_families: HashMap::new(),
            fighter_details: HashMap::new(),
            unavailable_fighter_families: HashSet::new(),
            planet: None,
            unavailable_planet: None,
            material: None,
            #[cfg(feature = "interface-test-fixtures")]
            current_lod: OriginalTacticalLod::Medium,
            #[cfg(feature = "interface-test-fixtures")]
            view: TacticalLodView::default(),
            #[cfg(feature = "interface-test-fixtures")]
            logged_lod: None,
            source_camera: None,
            logged_camera: None,
            source_layout: None,
            logged_layout: None,
            palette_selector: 1,
            logged_participant_scene: false,
            logged_fighter_scene: None,
            logged_planet_scene: None,
        }
    }
}

impl TacticalAssetRenderer {
    pub(crate) fn set_palette_selector(&mut self, selector: u8) {
        let selector = selector.clamp(1, 27);
        if self.palette_selector != selector {
            self.palette_selector = selector;
            #[cfg(feature = "interface-test-fixtures")]
            {
                self.attempted = false;
                self.assets.clear();
                self.logged_lod = None;
            }
            self.participant_families.clear();
            self.participant_lods.clear();
            self.unavailable_families.clear();
            self.fighter_families.clear();
            self.fighter_details.clear();
            self.unavailable_fighter_families.clear();
            self.planet = None;
            self.unavailable_planet = None;
            self.material = None;
            self.logged_participant_scene = false;
            self.logged_fighter_scene = None;
            self.logged_planet_scene = None;
        }
    }

    /// Draw the system-selected original tactical planet behind the retained
    /// participant scene. The resource and palette identities are recovered
    /// from `FUN_0059a850`, `FUN_00596ad0`, and `FUN_005c2e60`. Its provisional
    /// left-edge placement follows the best available native screenshots and
    /// remains outside strict A0 acceptance until the retained-frame transform
    /// is recovered.
    pub(crate) fn draw_planet(&mut self, aperture: (f32, f32, f32, f32)) -> bool {
        let resource_id = 5500 + u32::from(self.palette_selector);
        if self.planet.as_ref().map(|planet| planet.resource_id) != Some(resource_id)
            && self.unavailable_planet != Some(resource_id)
        {
            if let Err(error) = self.load_planet(resource_id) {
                self.unavailable_planet = Some(resource_id);
                macroquad::logging::warn!(
                    "[tactical_3d] planet resource={} unavailable: {}",
                    resource_id,
                    error
                );
            }
        }
        let Some(planet) = self
            .planet
            .as_ref()
            .filter(|planet| planet.resource_id == resource_id)
        else {
            return false;
        };
        let source_scale = aperture.3 / 439.0;
        let size = vec2(planet.texture.width(), planet.texture.height()) * source_scale;
        let x = aperture.0 - size.x * 0.5;
        let y = aperture.1;
        draw_texture_ex(
            &planet.texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(size),
                ..Default::default()
            },
        );
        if self.logged_planet_scene != Some(resource_id) {
            macroquad::logging::info!(
                "[tactical_3d] planet_scene resource={} palette_resource_id={} dimensions={}x{} source_position=left_edge_provisional source=FUN_0059a850,FUN_00596ad0,FUN_005c2e60,FUN_00509610,FUN_0040b0e0",
                resource_id,
                planet.palette_resource_id,
                planet.texture.width(),
                planet.texture.height(),
            );
            self.logged_planet_scene = Some(resource_id);
        }
        true
    }

    #[cfg(feature = "interface-test-fixtures")]
    pub(crate) fn set_view(&mut self, view: TacticalLodView) {
        self.view = view;
    }

    pub(crate) fn enable_original_camera(
        &mut self,
        player_is_empire: bool,
        layout: OriginalTacticalLayout,
    ) {
        self.source_camera = Some(OriginalTacticalCamera::new(
            player_is_empire,
            layout.battle_extent,
        ));
        self.participant_lods.clear();
        self.fighter_details.clear();
        self.logged_camera = None;
        self.source_layout = Some(layout);
        self.logged_layout = None;
        self.logged_participant_scene = false;
        self.logged_fighter_scene = None;
    }

    #[cfg(feature = "interface-test-fixtures")]
    pub(crate) fn disable_original_camera(&mut self) {
        self.source_camera = None;
        self.logged_camera = None;
        self.source_layout = None;
        self.logged_layout = None;
        self.logged_participant_scene = false;
        self.logged_fighter_scene = None;
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

    pub(crate) fn focus_target(&mut self, target_object_id: u32, target: Vec3) {
        if let Some(camera) = &mut self.source_camera {
            camera.focus_target(target_object_id, target);
        }
    }

    #[cfg(feature = "interface-test-fixtures")]
    pub(crate) fn draw_proof(&mut self, aperture: (f32, f32, f32, f32)) {
        if !self.attempted {
            self.attempted = true;
            if let Err(error) = self.load_proof() {
                macroquad::logging::warn!("[tactical_3d] LOD family unavailable: {}", error);
            }
        }
        if self.assets.len() != 3 || self.material.is_none() {
            return;
        }

        if let Some(layout) = self.source_layout {
            if self.logged_layout != Some(layout) {
                macroquad::logging::info!(
                    "[tactical_3d] layout_source first_active_objects={} second_active_objects={} battle_extent={} outer_positive_z={} outer_negative_z={} inner_negative_z={} inner_positive_z={} source=FUN_005ab650",
                    layout.first_active_objects,
                    layout.second_active_objects,
                    layout.battle_extent,
                    layout.outer_positive_z,
                    layout.outer_negative_z,
                    layout.inner_negative_z,
                    layout.inner_positive_z,
                );
                self.logged_layout = Some(layout);
            }
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
                pose.target_object_id.unwrap_or(0),
                pose.target.x.to_bits(),
                pose.target.y.to_bits(),
                pose.target.z.to_bits(),
            ];
            if self.logged_camera != Some(camera_key) {
                macroquad::logging::info!(
                    "[tactical_3d] camera_source pitch={} yaw={} field={} zoom_step={} orbit_step={} distance={} near={} far={} fovy_radians={} target_object_id={} target_x={} target_y={} target_z={} handedness=lh_y_up_to_rh_y_up",
                    pose.pitch_degrees,
                    pose.yaw_degrees,
                    pose.field,
                    pose.zoom_step,
                    pose.orbit_step,
                    source_camera.distance,
                    pose.near,
                    pose.far,
                    pose.fovy_radians,
                    pose.target_object_id.unwrap_or(0),
                    pose.target.x,
                    pose.target.y,
                    pose.target.z,
                );
                self.logged_camera = Some(camera_key);
            }
            set_camera(&Camera3D {
                position: pose.position,
                target: pose.target,
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
        let material = self.material.as_ref().unwrap();
        gl_use_material(material);
        material.set_uniform("ObjectTranslation", [0.0_f32; 4]);
        let asset = &self.assets[self.current_lod as usize];
        debug_assert_eq!(asset.resource_id, self.current_lod.resource_id());
        for chunk in &asset.meshes {
            material.set_uniform(
                "MaterialEmissive",
                [chunk.emissive[0], chunk.emissive[1], chunk.emissive[2], 0.0],
            );
            draw_mesh(&chunk.mesh);
        }
        gl_use_default_material();
        set_default_camera();
    }

    /// Draw every mapped live capital ship through its joined three-LOD family.
    pub(crate) fn draw_participants(
        &mut self,
        aperture: (f32, f32, f32, f32),
        objects: &[TacticalRenderObject],
    ) -> TacticalDrawReport {
        let mut report = TacticalDrawReport::default();
        if objects.is_empty() || self.source_camera.is_none() {
            return report;
        }

        let mut resource_bases = objects
            .iter()
            .map(|object| object.resource_base)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        resource_bases.sort_unstable();
        for resource_base in resource_bases {
            if self.participant_families.contains_key(&resource_base)
                || self.unavailable_families.contains(&resource_base)
            {
                continue;
            }
            if let Err(error) = self.load_participant_family(resource_base) {
                self.unavailable_families.insert(resource_base);
                macroquad::logging::warn!(
                    "[tactical_3d] participant family base={} unavailable: {}",
                    resource_base,
                    error
                );
            }
        }
        if self.material.is_none() {
            return report;
        }

        if let Some(layout) = self.source_layout {
            if self.logged_layout != Some(layout) {
                macroquad::logging::info!(
                    "[tactical_3d] layout_source first_active_objects={} second_active_objects={} battle_extent={} outer_positive_z={} outer_negative_z={} inner_negative_z={} inner_positive_z={} source=FUN_005ab650",
                    layout.first_active_objects,
                    layout.second_active_objects,
                    layout.battle_extent,
                    layout.outer_positive_z,
                    layout.outer_negative_z,
                    layout.inner_negative_z,
                    layout.inner_positive_z,
                );
                self.logged_layout = Some(layout);
            }
        }

        let (x, y, width, height) = aperture;
        let viewport = (
            x.round() as i32,
            (screen_height() - y - height).round() as i32,
            width.round() as i32,
            height.round() as i32,
        );
        let source_camera = self.source_camera.unwrap();
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
            pose.target_object_id.unwrap_or(0),
            pose.target.x.to_bits(),
            pose.target.y.to_bits(),
            pose.target.z.to_bits(),
        ];
        if self.logged_camera != Some(camera_key) {
            macroquad::logging::info!(
                "[tactical_3d] camera_source pitch={} yaw={} field={} zoom_step={} orbit_step={} distance={} near={} far={} fovy_radians={} target_object_id={} target_x={} target_y={} target_z={} handedness=lh_y_up_to_rh_y_up",
                pose.pitch_degrees,
                pose.yaw_degrees,
                pose.field,
                pose.zoom_step,
                pose.orbit_step,
                source_camera.distance,
                pose.near,
                pose.far,
                pose.fovy_radians,
                pose.target_object_id.unwrap_or(0),
                pose.target.x,
                pose.target.y,
                pose.target.z,
            );
            self.logged_camera = Some(camera_key);
        }
        let camera = Camera3D {
            position: pose.position,
            target: pose.target,
            up: pose.up,
            fovy: pose.fovy_radians,
            aspect: Some(width / height),
            viewport: Some(viewport),
            z_near: pose.near,
            z_far: pose.far,
            ..Default::default()
        };
        let camera_matrix = camera.matrix();
        set_camera(&camera);

        let material = self.material.as_ref().unwrap();
        gl_use_material(material);
        let mut selected_resources = Vec::new();
        let mut screen_positions = Vec::new();
        let mut screen_bounds = Vec::new();
        for object in objects {
            let Some(family) = self.participant_families.get(&object.resource_base) else {
                continue;
            };
            let prior_lod = self
                .participant_lods
                .get(&object.object_id)
                .copied()
                .unwrap_or(OriginalTacticalLod::Medium);
            let lod = select_original_tactical_lod(
                prior_lod,
                TacticalLodView {
                    view_depth: pose.position.distance(object.position),
                    projection_scale: 1.0,
                    high_detail: true,
                },
            );
            let asset = &family[lod as usize];
            if !asset.has_drawable_geometry() {
                continue;
            }
            self.participant_lods.insert(object.object_id, lod);
            material.set_uniform(
                "ObjectTranslation",
                [object.position.x, object.position.y, object.position.z, 0.0],
            );
            for chunk in &asset.meshes {
                material.set_uniform(
                    "MaterialEmissive",
                    [chunk.emissive[0], chunk.emissive[1], chunk.emissive[2], 0.0],
                );
                draw_mesh(&chunk.mesh);
            }
            report.rendered_object_ids.push(object.object_id);
            if let Some(screen) = project_world_position(camera_matrix, object.position, aperture) {
                report.screen_positions.push((object.object_id, screen));
                screen_positions.push(format!(
                    "{}:{:.3},{:.3}",
                    object.object_id, screen.x, screen.y
                ));
                let mut min = screen;
                let mut max = screen;
                for vertex in asset.meshes.iter().flat_map(|chunk| &chunk.mesh.vertices) {
                    if let Some(projected) = project_world_position(
                        camera_matrix,
                        vertex.position + object.position,
                        aperture,
                    ) {
                        min = min.min(projected);
                        max = max.max(projected);
                    }
                }
                report.projections.push(TacticalScreenProjection {
                    object_id: object.object_id,
                    center: screen,
                    min,
                    max,
                });
                screen_bounds.push(format!(
                    "{}:{:.3},{:.3},{:.3},{:.3}",
                    object.object_id, min.x, min.y, max.x, max.y
                ));
            }
            selected_resources.push(format!("{}:{}", object.object_id, asset.resource_id));
        }
        gl_use_default_material();
        set_default_camera();

        if !self.logged_participant_scene {
            let mut families: Vec<_> = self.participant_families.keys().copied().collect();
            families.sort_unstable();
            macroquad::logging::info!(
                "[tactical_3d] participant_scene requested={} rendered={} families={} resources={} screen_positions={} source_positions=true",
                objects.len(),
                report.rendered_object_ids.len(),
                families
                    .iter()
                    .map(u32::to_string)
                    .collect::<Vec<_>>()
                    .join(","),
                selected_resources.join(","),
                screen_positions.join(";"),
            );
            macroquad::logging::info!(
                "[tactical_3d] participant_bounds screen_bounds={} source_projection=true",
                screen_bounds.join(";")
            );
            self.logged_participant_scene = true;
        }
        report
    }

    /// Draw every mapped live fighter group through its original type-303
    /// close, far, and two-by-two indicator resources.
    pub(crate) fn draw_fighters(
        &mut self,
        aperture: (f32, f32, f32, f32),
        objects: &[TacticalFighterRenderObject],
    ) -> TacticalDrawReport {
        let mut report = TacticalDrawReport::default();
        let Some(source_camera) = self.source_camera else {
            return report;
        };
        if objects.is_empty() {
            return report;
        }

        let mut close_resources = objects
            .iter()
            .map(|object| object.close_resource_id)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        close_resources.sort_unstable();
        for close_resource_id in close_resources {
            if self.fighter_families.contains_key(&close_resource_id)
                || self
                    .unavailable_fighter_families
                    .contains(&close_resource_id)
            {
                continue;
            }
            let Some(object) = objects
                .iter()
                .find(|object| object.close_resource_id == close_resource_id)
            else {
                continue;
            };
            if let Err(error) = self.load_fighter_family(*object) {
                self.unavailable_fighter_families.insert(close_resource_id);
                macroquad::logging::warn!(
                    "[tactical_3d] fighter family close={} unavailable: {}",
                    close_resource_id,
                    error
                );
            }
        }

        let pose = source_camera.pose();
        let camera = Camera3D {
            position: pose.position,
            target: pose.target,
            up: pose.up,
            fovy: pose.fovy_radians,
            aspect: Some(aperture.2 / aperture.3),
            viewport: None,
            z_near: pose.near,
            z_far: pose.far,
            ..Default::default()
        };
        let camera_matrix = camera.matrix();
        let camera_forward = (pose.target - pose.position).normalize();
        let source_scale = aperture.2 / 444.0;
        let mut selected_resources = Vec::new();
        let mut screen_positions = Vec::new();
        let mut screen_bounds = Vec::new();
        for object in objects {
            let Some(center) = project_world_position(camera_matrix, object.position, aperture)
            else {
                continue;
            };
            let axial_depth = (object.position - pose.position).dot(camera_forward).abs();
            let view_span = axial_depth * pose.field;
            let prior = self
                .fighter_details
                .get(&object.object_id)
                .copied()
                .unwrap_or(OriginalFighterDetail::Far);
            let detail = select_original_fighter_detail(prior, view_span, true);
            let Some(family) = self.fighter_families.get(&object.close_resource_id) else {
                continue;
            };
            let (resource_id, texture) = family.selected(detail);
            let size = vec2(texture.width(), texture.height()) * source_scale;
            let min = center - size * 0.5;
            let max = min + size;
            draw_texture_ex(
                texture,
                min.x,
                min.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(size),
                    ..Default::default()
                },
            );
            self.fighter_details.insert(object.object_id, detail);
            report.rendered_object_ids.push(object.object_id);
            report.screen_positions.push((object.object_id, center));
            report.projections.push(TacticalScreenProjection {
                object_id: object.object_id,
                center,
                min,
                max,
            });
            selected_resources.push(format!(
                "{}:{}:{:?}:{:.3}",
                object.object_id, resource_id, detail, view_span
            ));
            screen_positions.push(format!(
                "{}:{:.3},{:.3}",
                object.object_id, center.x, center.y
            ));
            screen_bounds.push(format!(
                "{}:{:.3},{:.3},{:.3},{:.3}",
                object.object_id, min.x, min.y, max.x, max.y
            ));
        }

        let scene_signature = selected_resources
            .iter()
            .map(|entry| {
                entry
                    .rsplit_once(':')
                    .map_or(entry.as_str(), |(prefix, _)| prefix)
            })
            .collect::<Vec<_>>()
            .join(",");
        if self.logged_fighter_scene.as_deref() != Some(scene_signature.as_str()) {
            let mut families = self.fighter_families.keys().copied().collect::<Vec<_>>();
            families.sort_unstable();
            macroquad::logging::info!(
                "[tactical_3d] fighter_scene requested={} rendered={} families={} resources={} screen_positions={} screen_bounds={} source_positions=true source=FUN_005ab650,FUN_005c63f0,0x005d4af0",
                objects.len(),
                report.rendered_object_ids.len(),
                families.iter().map(u32::to_string).collect::<Vec<_>>().join(","),
                selected_resources.join(","),
                screen_positions.join(";"),
                screen_bounds.join(";"),
            );
            self.logged_fighter_scene = Some(scene_signature);
        }
        report
    }

    #[cfg(feature = "interface-test-fixtures")]
    fn load_proof(&mut self) -> Result<(), String> {
        let palette_resource_id = 5530 + u32::from(self.palette_selector);
        let (mesh_payloads, texture_payloads, palette_payload) = {
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
            let palette_key = format!("{palette_resource_id}/1033");
            let palette = cache
                .textures
                .get(&palette_key)
                .cloned()
                .ok_or_else(|| format!("typed palette entry {palette_key} is missing"))?;
            (meshes, textures, palette)
        };
        let palette = decode_palette_object(&palette_payload, palette_resource_id)?;

        let mut textures = HashMap::new();
        for (key, bytes) in texture_payloads {
            textures.insert(key, decode_indexed_texture(&bytes, &palette)?);
        }
        let white = Texture2D::from_rgba8(1, 1, &[255, 255, 255, 255]);
        white.set_filter(original_tactical_texture_filter());

        let mut assets = Vec::with_capacity(3);
        let mut diagnostics = Vec::with_capacity(3);
        for (index, mesh_bytes) in mesh_payloads.iter().enumerate() {
            let resource_id = 2560 + index as u32;
            let mut decoded = decode_mesh_object(mesh_bytes)?;
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
                meshes: decoded
                    .chunks
                    .into_iter()
                    .map(|chunk| TacticalMeshChunk {
                        mesh: chunk.mesh,
                        emissive: chunk.emissive,
                    })
                    .collect(),
            });
        }

        self.material = Some(load_tactical_material()?);
        self.assets = assets;
        self.family_loads = self.family_loads.saturating_add(1);
        macroquad::logging::info!(
            "[tactical_3d] family_loaded base=2560 resources=2560,2561,2562 textures=SDESTI52.BMP,SDESTI_M.BMP palette_selector={} palette_resource_id={} palette_flags=68 transform=authored_xyz_z_reflection light_directional_rgb={} light_ambient_rgb={} light_frame_source=5,5,-1 surface_to_light_rh={},{},{} light_constraint=z render_quality=gouraud device_dither={} texture_filter=nearest mip_filter=none source_cull=d3dcull_ccw target_cull=back_cw depth_test=less_equal depth_write=true specular={} material=diffuse_plus_emissive diagnostics={} family_loads={}",
            self.palette_selector,
            palette_resource_id,
            ORIGINAL_DIRECTIONAL_LIGHT_RGB,
            ORIGINAL_AMBIENT_LIGHT_RGB,
            original_surface_to_light_direction().x,
            original_surface_to_light_direction().y,
            original_surface_to_light_direction().z,
            ORIGINAL_DEVICE_DITHER,
            ORIGINAL_SPECULAR_ENABLED,
            diagnostics.join(","),
            self.family_loads,
        );
        Ok(())
    }

    fn load_participant_family(&mut self, resource_base: u32) -> Result<(), String> {
        let palette_resource_id = 5530 + u32::from(self.palette_selector);
        let (mesh_payloads, palette_payload) = {
            let cache = TACTICAL_OBJECT_CACHE.lock().unwrap();
            let meshes = (0..3_u32)
                .map(|offset| {
                    let key = format!("{}/1033", resource_base + offset);
                    cache
                        .meshes
                        .get(&key)
                        .cloned()
                        .map(|bytes| (resource_base + offset, bytes))
                        .ok_or_else(|| format!("typed mesh entry {key} is missing"))
                })
                .collect::<Result<Vec<_>, _>>()?;
            let palette_key = format!("{palette_resource_id}/1033");
            let palette = cache
                .textures
                .get(&palette_key)
                .cloned()
                .ok_or_else(|| format!("typed palette entry {palette_key} is missing"))?;
            (meshes, palette)
        };
        let palette = decode_palette_object(&palette_payload, palette_resource_id)?;

        let mut decoded_meshes = Vec::with_capacity(3);
        let mut texture_names = HashSet::new();
        for (resource_id, bytes) in mesh_payloads {
            let decoded = decode_mesh_object(&bytes)?;
            for chunk in &decoded.chunks {
                if let Some(name) = &chunk.texture_name {
                    texture_names.insert(name.to_ascii_uppercase());
                }
            }
            decoded_meshes.push((resource_id, decoded));
        }
        let texture_payloads = {
            let cache = TACTICAL_OBJECT_CACHE.lock().unwrap();
            texture_names
                .iter()
                .map(|name| {
                    let key = format!("{name}/1033");
                    cache
                        .textures
                        .get(&key)
                        .cloned()
                        .map(|bytes| (name.clone(), bytes))
                        .ok_or_else(|| format!("typed texture entry {key} is missing"))
                })
                .collect::<Result<Vec<_>, _>>()?
        };
        let mut textures = HashMap::with_capacity(texture_payloads.len());
        for (name, bytes) in texture_payloads {
            textures.insert(name, decode_indexed_texture(&bytes, &palette)?);
        }
        let white = Texture2D::from_rgba8(1, 1, &[255, 255, 255, 255]);
        white.set_filter(original_tactical_texture_filter());

        let mut assets = Vec::with_capacity(3);
        let mut diagnostics = Vec::with_capacity(3);
        for (resource_id, mut decoded) in decoded_meshes {
            for chunk in &mut decoded.chunks {
                let texture = chunk
                    .texture_name
                    .as_ref()
                    .and_then(|name| textures.get(&name.to_ascii_uppercase()))
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
                meshes: decoded
                    .chunks
                    .into_iter()
                    .map(|chunk| TacticalMeshChunk {
                        mesh: chunk.mesh,
                        emissive: chunk.emissive,
                    })
                    .collect(),
            });
        }

        if self.material.is_none() {
            self.material = Some(load_tactical_material()?);
        }
        self.participant_families.insert(resource_base, assets);
        self.family_loads = self.family_loads.saturating_add(1);
        let mut texture_names: Vec<_> = texture_names.into_iter().collect();
        texture_names.sort();
        macroquad::logging::info!(
            "[tactical_3d] participant_family_loaded base={} resources={},{},{} textures={} palette_selector={} palette_resource_id={} diagnostics={} family_loads={}",
            resource_base,
            resource_base,
            resource_base + 1,
            resource_base + 2,
            texture_names.join(","),
            self.palette_selector,
            palette_resource_id,
            diagnostics.join(","),
            self.family_loads,
        );
        Ok(())
    }

    fn load_fighter_family(&mut self, object: TacticalFighterRenderObject) -> Result<(), String> {
        let palette_resource_id = 5530 + u32::from(self.palette_selector);
        let resource_ids = [
            object.close_resource_id,
            object.far_resource_id,
            object.indicator_resource_id,
        ];
        let (payloads, palette_payload) = {
            let cache = TACTICAL_OBJECT_CACHE.lock().unwrap();
            let payloads = resource_ids
                .iter()
                .map(|resource_id| {
                    let key = format!("{resource_id}/1033");
                    cache
                        .textures
                        .get(&key)
                        .cloned()
                        .map(|bytes| (*resource_id, bytes))
                        .ok_or_else(|| format!("typed texture entry {key} is missing"))
                })
                .collect::<Result<Vec<_>, _>>()?;
            let palette_key = format!("{palette_resource_id}/1033");
            let palette = cache
                .textures
                .get(&palette_key)
                .cloned()
                .ok_or_else(|| format!("typed palette entry {palette_key} is missing"))?;
            (payloads, palette)
        };
        let palette = decode_palette_object(&palette_payload, palette_resource_id)?;
        let mut textures = payloads
            .into_iter()
            .map(|(resource_id, bytes)| {
                decode_indexed_texture(&bytes, &palette).map(|texture| (resource_id, texture))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;
        let close = textures
            .remove(&object.close_resource_id)
            .ok_or("decoded fighter close texture is missing")?;
        let far = textures
            .remove(&object.far_resource_id)
            .ok_or("decoded fighter far texture is missing")?;
        let indicator = textures
            .remove(&object.indicator_resource_id)
            .ok_or("decoded fighter indicator texture is missing")?;
        macroquad::logging::info!(
            "[tactical_3d] fighter_family_loaded close={} far={} indicator={} dimensions={}x{},{}x{},{}x{} palette_selector={} palette_resource_id={} texture_filter=nearest alpha=opaque source=FUN_005c63f0",
            object.close_resource_id,
            object.far_resource_id,
            object.indicator_resource_id,
            close.width(),
            close.height(),
            far.width(),
            far.height(),
            indicator.width(),
            indicator.height(),
            self.palette_selector,
            palette_resource_id,
        );
        self.fighter_families.insert(
            object.close_resource_id,
            TacticalFighterAsset {
                close_resource_id: object.close_resource_id,
                close,
                far_resource_id: object.far_resource_id,
                far,
                indicator_resource_id: object.indicator_resource_id,
                indicator,
            },
        );
        self.family_loads = self.family_loads.saturating_add(1);
        Ok(())
    }

    fn load_planet(&mut self, resource_id: u32) -> Result<(), String> {
        if !(5501..=5527).contains(&resource_id) {
            return Err(format!("invalid tactical planet resource {resource_id}"));
        }
        let palette_resource_id = resource_id + 30;
        let (texture_payload, palette_payload) = {
            let cache = TACTICAL_OBJECT_CACHE.lock().unwrap();
            let texture_key = format!("{resource_id}/1033");
            let palette_key = format!("{palette_resource_id}/1033");
            let texture = cache
                .textures
                .get(&texture_key)
                .cloned()
                .ok_or_else(|| format!("typed texture entry {texture_key} is missing"))?;
            let palette = cache
                .textures
                .get(&palette_key)
                .cloned()
                .ok_or_else(|| format!("typed palette entry {palette_key} is missing"))?;
            (texture, palette)
        };
        let palette = decode_palette_object(&palette_payload, palette_resource_id)?;
        let texture =
            decode_indexed_texture_with_rule(&texture_payload, &palette, palette_resource_id, 2)?;
        if texture.width() != 256.0 || texture.height() != 256.0 {
            return Err(format!(
                "tactical planet {resource_id} has invalid dimensions {}x{}",
                texture.width(),
                texture.height()
            ));
        }
        macroquad::logging::info!(
            "[tactical_3d] planet_loaded resource={} palette_resource_id={} dimensions={}x{} texture_filter=nearest alpha=opaque source=FUN_0059a850,FUN_00596ad0,FUN_005c2e60",
            resource_id,
            palette_resource_id,
            texture.width(),
            texture.height(),
        );
        self.planet = Some(TacticalPlanetAsset {
            resource_id,
            palette_resource_id,
            texture,
        });
        self.family_loads = self.family_loads.saturating_add(1);
        Ok(())
    }
}

fn project_world_position(
    camera_matrix: Mat4,
    position: Vec3,
    aperture: (f32, f32, f32, f32),
) -> Option<Vec2> {
    let clip = camera_matrix * position.extend(1.0);
    if !clip.is_finite() || clip.w <= 0.0 {
        return None;
    }
    let ndc = clip.truncate() / clip.w;
    let (x, y, width, height) = aperture;
    Some(vec2(
        x + (ndc.x + 1.0) * 0.5 * width,
        y + (1.0 - ndc.y) * 0.5 * height,
    ))
}

struct DecodedMeshChunk {
    mesh: Mesh,
    texture_name: Option<String>,
    emissive: [f32; 3],
}

struct DecodedMeshObject {
    chunks: Vec<DecodedMeshChunk>,
    #[cfg(not(target_arch = "wasm32"))]
    materials: usize,
    source_vertices: usize,
    source_faces: usize,
}

struct DecodedMaterial {
    diffuse: Color,
    emissive: [f32; 3],
    texture_name: Option<String>,
}

fn decode_mesh_object(bytes: &[u8]) -> Result<DecodedMeshObject, String> {
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
    let extent = (bounds[3] - bounds[0])
        .max(bounds[4] - bounds[1])
        .max(bounds[5] - bounds[2]);
    if !extent.is_finite() || extent <= 0.0 {
        return Err("invalid tactical mesh bounds".to_string());
    }
    let mut decoded_materials = Vec::with_capacity(materials);
    for _ in 0..materials {
        let diffuse = [reader.f32()?, reader.f32()?, reader.f32()?, reader.f32()?];
        let _specular_exponent = reader.f32()?;
        let _specular = [reader.f32()?, reader.f32()?, reader.f32()?];
        let emissive = [reader.f32()?, reader.f32()?, reader.f32()?];
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
            emissive,
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
            vertices.push(Vertex {
                position: authored_position(position),
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
            emissive: decoded_material.emissive,
        });
    }
    if !reader.finished() {
        return Err("tactical mesh object has trailing bytes".to_string());
    }
    Ok(DecodedMeshObject {
        chunks: output,
        #[cfg(not(target_arch = "wasm32"))]
        materials,
        source_vertices,
        source_faces,
    })
}

fn authored_position(position: Vec3) -> Vec3 {
    vec3(position.x, position.y, -position.z)
}

fn decode_palette_object(
    bytes: &[u8],
    expected_resource_id: u32,
) -> Result<[[u8; 4]; 256], String> {
    let mut reader = Reader::new(bytes);
    reader.expect(PALETTE_MAGIC)?;
    if reader.u32()? != 1 || reader.u32()? != expected_resource_id {
        return Err("invalid tactical palette object identity".to_string());
    }
    let rgb = reader.bytes(256 * 3)?;
    if !reader.finished() {
        return Err("tactical palette object has trailing bytes".to_string());
    }
    let mut palette = [[0_u8; 4]; 256];
    for (entry, source) in palette.iter_mut().zip(rgb.chunks_exact(3)) {
        *entry = [source[0], source[1], source[2], 255];
    }
    Ok(palette)
}

#[cfg(not(target_arch = "wasm32"))]
fn validate_indexed_texture_object(
    bytes: &[u8],
    record: &NativeTextureRecord,
) -> Result<(), String> {
    let mut reader = Reader::new(bytes);
    reader.expect(TEXTURE_MAGIC)?;
    let version = reader.u32()?;
    let width = reader.u32()?;
    let height = reader.u32()?;
    let palette_id = reader.u32()?;
    let palette_rule = reader.u32()?;
    let pixels = reader.u32()? as usize;
    let trailing = reader.u32()? as usize;
    let expected_pixels = (width as usize)
        .checked_mul(height as usize)
        .ok_or("tactical texture dimensions overflow")?;
    let planet_pair = record.identifier_kind == "id" && (5501..=5527).contains(&record.id);
    let expected_palette_id = if planet_pair { record.id + 30 } else { 0 };
    let expected_palette_rule = if planet_pair { 2 } else { 1 };
    let expected_trailing = if record.identifier_kind == "id" && (4200..=4204).contains(&record.id)
    {
        4
    } else {
        0
    };
    if version != 1
        || width != record.width
        || height != record.height
        || width == 0
        || height == 0
        || width > u16::MAX.into()
        || height > u16::MAX.into()
        || palette_id != expected_palette_id
        || palette_rule != expected_palette_rule
        || pixels != expected_pixels
        || pixels > 16_777_216
        || trailing != expected_trailing
    {
        return Err("indexed tactical texture metadata mismatch".to_string());
    }
    reader.bytes(pixels)?;
    reader.bytes(trailing)?;
    if !reader.finished() {
        return Err("indexed tactical texture has unexpected trailing bytes".to_string());
    }
    Ok(())
}

fn decode_indexed_texture(bytes: &[u8], palette: &[[u8; 4]; 256]) -> Result<Texture2D, String> {
    decode_indexed_texture_with_rule(bytes, palette, 0, 1)
}

fn decode_indexed_texture_with_rule(
    bytes: &[u8],
    palette: &[[u8; 4]; 256],
    expected_palette_id: u32,
    expected_palette_rule: u32,
) -> Result<Texture2D, String> {
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
        || palette_id != expected_palette_id
        || palette_rule != expected_palette_rule
        || !matches!(trailing, 0 | 4)
    {
        return Err("invalid indexed tactical texture header".to_string());
    }
    let indices = reader.bytes(pixels)?;
    reader.bytes(trailing)?;
    if !reader.finished() {
        return Err("tactical texture object has trailing bytes".to_string());
    }
    let mut rgba = Vec::with_capacity(pixels * 4);
    for index in indices {
        rgba.extend_from_slice(&palette[usize::from(*index)]);
    }
    let texture = Texture2D::from_rgba8(width as u16, height as u16, &rgba);
    texture.set_filter(original_tactical_texture_filter());
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
    let material = load_material(
        shader,
        MaterialParams {
            pipeline_params: original_tactical_pipeline_params(),
            uniforms: vec![
                UniformDesc::new("SourceLightDirectionAmbient", UniformType::Float4),
                UniformDesc::new("SourceLightColorDirectional", UniformType::Float4),
                UniformDesc::new("MaterialEmissive", UniformType::Float4),
                UniformDesc::new("ObjectTranslation", UniformType::Float4),
            ],
            ..Default::default()
        },
    )
    .map_err(|error| format!("compile tactical material: {error}"))?;
    let direction = original_surface_to_light_direction();
    material.set_uniform(
        "SourceLightDirectionAmbient",
        [
            direction.x,
            direction.y,
            direction.z,
            ORIGINAL_AMBIENT_LIGHT_RGB,
        ],
    );
    material.set_uniform(
        "SourceLightColorDirectional",
        [1.0, 1.0, 1.0, ORIGINAL_DIRECTIONAL_LIGHT_RGB],
    );
    material.set_uniform("MaterialEmissive", [0.0_f32; 4]);
    material.set_uniform("ObjectTranslation", [0.0_f32; 4]);
    Ok(material)
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
varying lowp vec3 light;
varying lowp vec4 tint;
uniform mat4 Model;
uniform mat4 Projection;
uniform vec4 SourceLightDirectionAmbient;
uniform vec4 SourceLightColorDirectional;
uniform lowp vec4 MaterialEmissive;
uniform vec4 ObjectTranslation;
void main() {
    gl_Position = Projection * Model * vec4(position + ObjectTranslation.xyz, 1.0);
    uv = texcoord;
    tint = color0 / 255.0;
    lowp float diffuse = max(dot(normalize(normal.xyz), normalize(SourceLightDirectionAmbient.xyz)), 0.0);
    light = min(vec3(1.0), vec3(SourceLightDirectionAmbient.w) + SourceLightColorDirectional.rgb * SourceLightColorDirectional.w * diffuse);
}"#;

const TACTICAL_FRAGMENT_GLSL: &str = r#"#version 100
precision lowp float;
varying lowp vec2 uv;
varying lowp vec3 light;
varying lowp vec4 tint;
uniform sampler2D Texture;
uniform lowp vec4 MaterialEmissive;
void main() {
    lowp vec4 texel = texture2D(Texture, uv);
    lowp vec3 material = min(vec3(1.0), tint.rgb * light + MaterialEmissive.rgb);
    gl_FragColor = vec4(texel.rgb * material, texel.a * tint.a);
}"#;

const TACTICAL_METAL: &str = r#"
#include <metal_stdlib>
using namespace metal;
struct Uniforms {
    float4x4 Model;
    float4x4 Projection;
    float4 _Time;
    float4 SourceLightDirectionAmbient;
    float4 SourceLightColorDirectional;
    float4 MaterialEmissive;
    float4 ObjectTranslation;
};
struct Vertex {
    float3 position [[attribute(0)]];
    float2 texcoord [[attribute(1)]];
    float4 color0 [[attribute(2)]];
    float4 normal [[attribute(3)]];
};
struct RasterizerData {
    float4 position [[position]];
    float2 uv [[user(locn0)]];
    float3 light [[user(locn1)]];
    float4 tint [[user(locn2)]];
};
vertex RasterizerData vertexShader(Vertex v [[stage_in]], constant Uniforms& u [[buffer(0)]]) {
    RasterizerData out;
    out.position = u.Projection * u.Model * float4(v.position + u.ObjectTranslation.xyz, 1.0);
    out.uv = v.texcoord;
    out.tint = v.color0 / 255.0;
    float diffuse = max(dot(normalize(v.normal.xyz), normalize(u.SourceLightDirectionAmbient.xyz)), 0.0);
    out.light = min(float3(1.0), float3(u.SourceLightDirectionAmbient.w) + u.SourceLightColorDirectional.rgb * u.SourceLightColorDirectional.w * diffuse);
    return out;
}
fragment float4 fragmentShader(RasterizerData in [[stage_in]], constant Uniforms& u [[buffer(0)]], texture2d<float> Texture [[texture(0)]], sampler TextureSmplr [[sampler(0)]]) {
    float4 texel = Texture.sample(TextureSmplr, in.uv);
    float3 material = min(float3(1.0), in.tint.rgb * in.light + u.MaterialEmissive.rgb);
    return float4(texel.rgb * material, texel.a * in.tint.a);
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
    fn authored_mesh_positions_keep_source_scale_and_reflect_only_z() {
        assert_eq!(
            authored_position(vec3(12.5, -3.25, 8.0)),
            vec3(12.5, -3.25, -8.0)
        );
    }

    #[test]
    fn source_light_rig_preserves_retained_mode_direction_and_intensity() {
        let direction = original_surface_to_light_direction();
        let inverse_length = 51.0_f32.sqrt().recip();
        assert_near(direction.x, 5.0 * inverse_length);
        assert_near(direction.y, 5.0 * inverse_length);
        assert_near(direction.z, inverse_length);
        assert_near(
            original_tactical_light_factor(-direction),
            ORIGINAL_AMBIENT_LIGHT_RGB,
        );
        assert_near(original_tactical_light_factor(direction), 1.0);
        assert_near(
            original_tactical_light_factor(vec3(-1.0, 1.0, 0.0)),
            ORIGINAL_AMBIENT_LIGHT_RGB,
        );
    }

    #[test]
    fn source_device_state_uses_nearest_gouraud_backface_depth_contract() {
        assert!(!ORIGINAL_DEVICE_DITHER);
        assert!(!ORIGINAL_SPECULAR_ENABLED);
        assert_eq!(original_tactical_texture_filter(), FilterMode::Nearest);
        let pipeline = original_tactical_pipeline_params();
        assert_eq!(pipeline.cull_face, CullFace::Back);
        assert_eq!(pipeline.front_face_order, FrontFaceOrder::Clockwise);
        assert_eq!(pipeline.depth_test, Comparison::LessOrEqual);
        assert!(pipeline.depth_write);
        assert!(pipeline.color_blend.is_none());
        assert!(pipeline.alpha_blend.is_none());
    }

    #[test]
    fn tactical_palette_object_preserves_rgb_and_uses_opaque_texture_alpha() {
        let resource_id = 5537_u32;
        let mut bytes = Vec::from(PALETTE_MAGIC.as_slice());
        bytes.extend_from_slice(&1_u32.to_le_bytes());
        bytes.extend_from_slice(&resource_id.to_le_bytes());
        for index in 0_u16..256 {
            let component = u8::try_from(index).unwrap();
            bytes.extend_from_slice(&[component, 255 - component, 17]);
        }
        let palette = decode_palette_object(&bytes, resource_id).unwrap();
        assert_eq!(palette[0], [0, 255, 17, 255]);
        assert_eq!(palette[255], [255, 0, 17, 255]);
        assert!(decode_palette_object(&bytes, resource_id + 1).is_err());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn tactical_planet_object_requires_its_exact_paired_palette() {
        let resource_id = 5507_u32;
        let palette_resource_id = resource_id + 30;
        let width = 256_u32;
        let height = 256_u32;
        let pixels = width * height;
        let mut bytes = Vec::from(TEXTURE_MAGIC.as_slice());
        for value in [1_u32, width, height, palette_resource_id, 2, pixels, 0] {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        bytes.resize(bytes.len() + pixels as usize, 0);
        let record = NativeTextureRecord {
            identifier_kind: "id".to_string(),
            id: resource_id,
            name: None,
            language: 1033,
            kind: "indexed_rle".to_string(),
            width,
            height,
            palette_rule: Some("planet_pair".to_string()),
            source_sha256: "0".repeat(64),
            object_sha256: "0".repeat(64),
            object: "objects/unused.texture".to_string(),
        };
        validate_indexed_texture_object(&bytes, &record).unwrap();

        let palette_offset = TEXTURE_MAGIC.len() + 3 * std::mem::size_of::<u32>();
        bytes[palette_offset..palette_offset + 4]
            .copy_from_slice(&(palette_resource_id + 1).to_le_bytes());
        assert!(validate_indexed_texture_object(&bytes, &record).is_err());
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
    fn original_layout_expands_extent_and_derives_four_source_lanes() {
        let balanced = OriginalTacticalLayout::from_active_counts(2, 2);
        assert_eq!(balanced.first_active_objects, 2);
        assert_eq!(balanced.second_active_objects, 2);
        assert_near(balanced.battle_extent, 106.0);
        assert_near(balanced.outer_positive_z, 53.0);
        assert_near(balanced.outer_negative_z, -53.0);
        assert_near(balanced.inner_negative_z, -33.0);
        assert_near(balanced.inner_positive_z, 33.0);

        let uneven = OriginalTacticalLayout::from_active_counts(7, 3);
        assert_near(uneven.battle_extent, 121.0);
        assert_near(uneven.outer_positive_z, 60.5);
        assert_near(uneven.inner_positive_z, 40.5);
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
        camera.focus_target(17, vec3(4.0, 5.0, 6.0));
        let targeted = camera.pose();
        assert_eq!(targeted.target_object_id, Some(17));
        assert_eq!(targeted.target, vec3(4.0, 5.0, 6.0));

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

    #[test]
    fn original_fighter_detail_uses_exact_source_thresholds() {
        for (view_span, expected) in [
            (0.0, OriginalFighterDetail::Close),
            (4.999, OriginalFighterDetail::Close),
            (5.0, OriginalFighterDetail::Far),
            (9.999, OriginalFighterDetail::Far),
            (10.0, OriginalFighterDetail::Indicator),
            (100.0, OriginalFighterDetail::Indicator),
        ] {
            assert_eq!(
                select_original_fighter_detail(OriginalFighterDetail::Far, view_span, true),
                expected
            );
        }
        assert_eq!(
            select_original_fighter_detail(OriginalFighterDetail::Close, 1.0, false),
            OriginalFighterDetail::Far
        );
        assert_eq!(
            select_original_fighter_detail(OriginalFighterDetail::Indicator, f32::NAN, true),
            OriginalFighterDetail::Indicator
        );
        assert_eq!(
            select_original_fighter_detail(OriginalFighterDetail::Close, -1.0, true),
            OriginalFighterDetail::Close
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn complete_tactical_corpus_identity_sets_are_exact() {
        let meshes = expected_mesh_ids();
        let numeric = expected_numeric_texture_ids();
        let named: HashSet<_> = EXPECTED_NAMED_TEXTURES.iter().copied().collect();
        assert_eq!(meshes.len(), 87);
        assert_eq!(numeric.len(), 338);
        assert_eq!(named.len(), 59);
        assert!(meshes.contains(&2010));
        assert!(meshes.contains(&2642));
        assert!(!meshes.contains(&2009));
        assert!(!numeric.contains(&4040));
        assert!(numeric.contains(&4044));
        assert!(numeric.contains(&5557));
        assert!(named.contains("MONCAL52.BMP"));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "requires the ignored runtime store generated from an owned TACTICAL.DLL"]
    fn owned_native_complete_corpus_installs_with_exact_counts() {
        let runtime = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../data/base/ui/tactical-dll/TACTICAL3D/runtime");
        let manifest = read_native_manifest(&runtime).expect("validate original corpus manifest");

        let mut replaced_mesh = manifest.clone();
        replaced_mesh.meshes[0].id = 9999;
        assert!(validate_native_manifest(&replaced_mesh).is_err());

        let mut wrong_language = manifest.clone();
        wrong_language.meshes[0].language = 0;
        assert!(validate_native_manifest(&wrong_language).is_err());

        let mut wrong_kind = manifest.clone();
        wrong_kind.textures[0].kind = "palette_rgb24".to_string();
        assert!(validate_native_manifest(&wrong_kind).is_err());

        let mut broken_binding = manifest;
        let binding = broken_binding
            .meshes
            .iter_mut()
            .find_map(|record| record.texture_bindings.as_mut()?.first_mut())
            .expect("original corpus contains a named texture binding");
        binding.resource_name = "NOT-A-TACTICAL-TEXTURE.BMP".to_string();
        assert!(validate_native_manifest(&broken_binding).is_err());

        install_native_tactical_assets(&runtime).expect("install complete native tactical corpus");
        let cache = TACTICAL_OBJECT_CACHE.lock().unwrap();
        assert_eq!(cache.meshes.len(), 87);
        assert_eq!(cache.textures.len(), 397);
        for key in ["2010/1033", "2510/1033", "2642/1033"] {
            assert!(cache.meshes.contains_key(key), "missing mesh {key}");
        }
        for key in ["MONCAL52.BMP/1033", "STRIKE52.BMP/1033", "5531/1033"] {
            assert!(cache.textures.contains_key(key), "missing texture {key}");
        }
    }

    #[cfg(all(feature = "interface-test-fixtures", not(target_arch = "wasm32")))]
    #[test]
    #[ignore = "requires the ignored runtime store generated from an owned TACTICAL.DLL"]
    fn owned_native_lod_family_installs_with_exact_typed_keys() {
        let runtime = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../data/base/ui/tactical-dll/TACTICAL3D/runtime");
        install_native_tactical_lod_family(&runtime)
            .expect("install owned P57 tactical LOD family");
        let cache = TACTICAL_OBJECT_CACHE.lock().unwrap();
        assert_eq!(cache.meshes.len(), PROOF_MESH_KEYS.len());
        assert_eq!(
            cache.textures.len(),
            PROOF_TEXTURE_KEYS.len()
                + usize::try_from(TACTICAL_PALETTE_LAST - TACTICAL_PALETTE_FIRST + 1).unwrap()
        );
        for key in PROOF_MESH_KEYS {
            assert!(cache.meshes.contains_key(key));
        }
        for key in PROOF_TEXTURE_KEYS {
            assert!(cache.textures.contains_key(key));
        }
        for palette_id in TACTICAL_PALETTE_FIRST..=TACTICAL_PALETTE_LAST {
            assert!(cache.textures.contains_key(&format!("{palette_id}/1033")));
        }
    }
}
