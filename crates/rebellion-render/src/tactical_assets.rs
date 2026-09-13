//! Decoding and rendering for the original tactical type-301/type-303 assets.
//!
//! P56 intentionally installs one source-bound proof pair. The cache and
//! decoder are reusable by later fleet integration, but no DAT identity is
//! inferred from the proof resource.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

#[cfg(not(target_arch = "wasm32"))]
use std::{io::Read, path::Path};

use macroquad::prelude::*;
use macroquad::window::miniquad::{Backend, Comparison, PipelineParams};

use crate::bmp_cache::{resources, BmpCache, DllSource};

pub const PROOF_MESH_KEY: &str = "2560/1033";
pub const PROOF_TEXTURE_KEY: &str = "SDESTI52.BMP/1033";

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

/// Install the same P56 proof pair from an ignored native runtime store.
#[cfg(not(target_arch = "wasm32"))]
pub fn install_native_tactical_proof(runtime_root: &Path) -> Result<(), String> {
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
    let mut meshes = manifest
        .meshes
        .iter()
        .filter(|record| record.id == 2560 && record.language == 1033);
    let mesh = meshes
        .next()
        .filter(|_| meshes.next().is_none())
        .ok_or("tactical runtime lacks unique mesh 2560/1033")?;
    let texture_bindings = mesh.texture_bindings.as_deref().unwrap_or_default();
    if texture_bindings.len() != 1
        || !texture_bindings[0]
            .resource_name
            .eq_ignore_ascii_case("SDESTI52.BMP")
        || texture_bindings[0].resource_language != 1033
    {
        return Err("mesh 2560 does not bind SDESTI52.BMP/1033".to_string());
    }
    let mut textures = manifest.textures.iter().filter(|record| {
        record.identifier_kind == "name"
            && record
                .name
                .as_deref()
                .is_some_and(|name| name.eq_ignore_ascii_case("SDESTI52.BMP"))
            && record.language == 1033
    });
    let texture = textures
        .next()
        .filter(|_| textures.next().is_none())
        .ok_or("tactical runtime lacks unique SDESTI52.BMP/1033")?;
    if texture.kind != "indexed_rle" || texture.palette_rule.as_deref() != Some("battle_active") {
        return Err("SDESTI52.BMP does not retain the active battle-palette rule".to_string());
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

    let mesh_bytes = load_object(runtime_root, &mesh.object, &mesh.object_sha256, ".mesh")?;
    let texture_bytes = load_object(
        runtime_root,
        &texture.object,
        &texture.object_sha256,
        ".texture",
    )?;
    set_tactical_asset_cache(
        HashMap::from([(PROOF_MESH_KEY.to_string(), mesh_bytes)]),
        HashMap::from([(PROOF_TEXTURE_KEY.to_string(), texture_bytes)]),
    );
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

/// Lazily allocated GPU state for the single-resource P56 proof.
#[derive(Default)]
pub(crate) struct TacticalProofRenderer {
    attempted: bool,
    meshes: Vec<Mesh>,
    material: Option<Material>,
}

impl TacticalProofRenderer {
    pub(crate) fn draw(&mut self, bmp_cache: &mut BmpCache, aperture: (f32, f32, f32, f32)) {
        if !self.attempted {
            self.attempted = true;
            if let Err(error) = self.load(bmp_cache) {
                macroquad::logging::warn!("[tactical_3d] resource 2560 unavailable: {}", error);
            }
        }
        if self.meshes.is_empty() || self.material.is_none() {
            return;
        }

        let (x, y, width, height) = aperture;
        let viewport = (
            x.round() as i32,
            (screen_height() - y - height).round() as i32,
            width.round() as i32,
            height.round() as i32,
        );
        set_camera(&Camera3D {
            position: vec3(2.8, -4.2, 2.2),
            target: Vec3::ZERO,
            up: Vec3::Z,
            fovy: 35.0_f32.to_radians(),
            aspect: Some(width / height),
            viewport: Some(viewport),
            z_near: 0.1,
            z_far: 100.0,
            ..Default::default()
        });
        gl_use_material(self.material.as_ref().unwrap());
        for mesh in &self.meshes {
            draw_mesh(mesh);
        }
        gl_use_default_material();
        set_default_camera();
    }

    fn load(&mut self, bmp_cache: &mut BmpCache) -> Result<(), String> {
        let (mesh_bytes, texture_bytes) = {
            let cache = TACTICAL_OBJECT_CACHE.lock().unwrap();
            (
                cache.meshes.get(PROOF_MESH_KEY).cloned(),
                cache.textures.get(PROOF_TEXTURE_KEY).cloned(),
            )
        };
        let mesh_bytes = mesh_bytes.ok_or("typed mesh entry is missing")?;
        let texture_bytes = texture_bytes.ok_or("typed texture entry is missing")?;
        let palette = bmp_cache
            .original_palette_rgba(DllSource::Tactical, resources::tactical::BACKGROUND)
            .ok_or("TACTICAL 1000 active palette is unavailable")?;
        let texture = decode_indexed_texture(&texture_bytes, &palette)?;
        let (mut meshes, source_vertices, source_faces) = decode_mesh_object(&mesh_bytes)?;
        for mesh in &mut meshes {
            mesh.texture = Some(texture.clone());
        }
        self.material = Some(load_tactical_material()?);
        let triangles: usize = meshes.iter().map(|mesh| mesh.indices.len() / 3).sum();
        let render_vertices: usize = meshes.iter().map(|mesh| mesh.vertices.len()).sum();
        self.meshes = meshes;
        macroquad::logging::info!(
            "[tactical_3d] rendered resource_id=2560 language=1033 texture=SDESTI52.BMP palette=tactical-dll/1000 source_vertices={} source_faces={} render_vertices={} triangles={}",
            source_vertices,
            source_faces,
            render_vertices,
            triangles
        );
        Ok(())
    }
}

fn decode_mesh_object(bytes: &[u8]) -> Result<(Vec<Mesh>, usize, usize), String> {
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
    let center = vec3(
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
    let scale = 2.0 / extent;
    for _ in 0..materials {
        for _ in 0..11 {
            reader.f32()?;
        }
        let name_len = reader.bounded_count(512, "texture name")?;
        let name = reader.bytes(name_len)?;
        std::str::from_utf8(name).map_err(|_| "invalid tactical texture name")?;
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
        let mut vertices = Vec::with_capacity(vertex_count);
        for _ in 0..vertex_count {
            let position = vec3(reader.f32()?, reader.f32()?, reader.f32()?);
            let normal = vec3(reader.f32()?, reader.f32()?, reader.f32()?);
            let uv = vec2(reader.f32()?, reader.f32()?);
            vertices.push(Vertex {
                position: (position - center) * scale,
                uv,
                color: WHITE.into(),
                normal: normal.extend(0.0),
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
        output.push(Mesh {
            vertices,
            indices,
            texture: None,
        });
    }
    if !reader.finished() {
        return Err("tactical mesh object has trailing bytes".to_string());
    }
    Ok((output, source_vertices, source_faces))
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
attribute vec4 normal;
varying lowp vec2 uv;
varying lowp float light;
uniform mat4 Model;
uniform mat4 Projection;
void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
    light = 0.28 + 0.72 * max(dot(normalize(normal.xyz), normalize(vec3(-0.35, -0.5, 0.8))), 0.0);
}"#;

const TACTICAL_FRAGMENT_GLSL: &str = r#"#version 100
varying lowp vec2 uv;
varying lowp float light;
uniform sampler2D Texture;
void main() {
    lowp vec4 texel = texture2D(Texture, uv);
    gl_FragColor = vec4(texel.rgb * light, texel.a);
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
};
vertex RasterizerData vertexShader(Vertex v [[stage_in]], constant Uniforms& u [[buffer(0)]]) {
    RasterizerData out;
    out.position = u.Projection * u.Model * float4(v.position, 1.0);
    out.uv = v.texcoord;
    out.light = 0.28 + 0.72 * max(dot(normalize(v.normal.xyz), normalize(float3(-0.35, -0.5, 0.8))), 0.0);
    return out;
}
fragment float4 fragmentShader(RasterizerData in [[stage_in]], texture2d<float> Texture [[texture(0)]], sampler TextureSmplr [[sampler(0)]]) {
    float4 texel = Texture.sample(TextureSmplr, in.uv);
    return float4(texel.rgb * in.light, texel.a);
}"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reader_rejects_non_finite_and_truncated_values() {
        let encoded_nan = f32::NAN.to_bits().to_le_bytes();
        let mut non_finite = Reader::new(&encoded_nan);
        assert!(non_finite.f32().is_err());
        let mut truncated = Reader::new(&[1, 2, 3]);
        assert!(truncated.u32().is_err());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "requires the ignored runtime store generated from an owned TACTICAL.DLL"]
    fn owned_native_proof_pair_installs_with_exact_typed_keys() {
        let runtime = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../data/base/ui/tactical-dll/TACTICAL3D/runtime");
        install_native_tactical_proof(&runtime).expect("install owned P56 tactical proof pair");
        let cache = TACTICAL_OBJECT_CACHE.lock().unwrap();
        assert_eq!(cache.meshes.len(), 1);
        assert_eq!(cache.textures.len(), 1);
        assert!(cache.meshes.contains_key(PROOF_MESH_KEY));
        assert!(cache.textures.contains_key(PROOF_TEXTURE_KEY));
    }
}
