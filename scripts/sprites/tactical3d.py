#!/usr/bin/env python3
"""Reader for the staged TACTICAL.DLL runtime objects (P55) used by the sprite tracks.

Follows the Go writer (``tools/stage-ui-assets/tactical3d_runtime.go``) and the Rust proof
decoder (``crates/rebellion-render/src/tactical_assets.rs``):

* ``ORTMESH\\0`` v1: u32 version, materials, chunks, source_vertices, source_faces, normal_mode,
  6 x f32 bounds; materials = 4 f32 diffuse, 1 f32 specular exponent, 3 f32 specular,
  3 f32 emissive, u32 name_len + name; chunks = u32 material, u32 vertex_count, u32 index_count,
  vertices (pos xyz, normal xyz, uv) f32, u16 indices.
  Handedness (``authored_position``): negate position z and normal z, swap triangle indices 1,2.
* ``ORTINDEX`` v1: 7 x u32 = version, width, height, palette_id, palette_rule (1 battle_active,
  2 planet_pair), pixels, trailing; then pixel indices; then ``trailing`` tail bytes (4 allowed).
* ``ORTPAL00``: u32 version, u32 resource id, 256 x 3 RGB.

Nothing here touches the Rust crates. Textures resolve by name (mesh material bindings) or by
id (fighters 4000-4134, palettes 5531-5557).
"""

from __future__ import annotations

import json
import math
import struct
from dataclasses import dataclass, field
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parents[2]
RUNTIME_ROOT = PROJECT_ROOT / "data" / "base" / "ui" / "tactical-dll" / "TACTICAL3D" / "runtime"
MESH_MAGIC = b"ORTMESH\0"
TEXTURE_MAGIC = b"ORTINDEX"
PALETTE_MAGIC = b"ORTPAL00"
PALETTE_RULES = {1: "battle_active", 2: "planet_pair"}
DEFAULT_BATTLE_PALETTE = 5531  # TACTICAL_PALETTE_FIRST; per-system palettes run 5531-5557
LOD_LABELS = ("close", "medium", "far")


class TacticalObjectError(ValueError):
    pass


class Reader:
    def __init__(self, data: bytes):
        self.data, self.pos = data, 0

    def expect(self, magic: bytes) -> None:
        if self.data[self.pos:self.pos + len(magic)] != magic:
            raise TacticalObjectError(f"bad magic, expected {magic!r}")
        self.pos += len(magic)

    def u32(self) -> int:
        v = struct.unpack_from("<I", self.data, self.pos)[0]
        self.pos += 4
        return v

    def u16(self) -> int:
        v = struct.unpack_from("<H", self.data, self.pos)[0]
        self.pos += 2
        return v

    def f32(self) -> float:
        v = struct.unpack_from("<f", self.data, self.pos)[0]
        self.pos += 4
        return v

    def f32s(self, n: int) -> tuple[float, ...]:
        v = struct.unpack_from(f"<{n}f", self.data, self.pos)
        self.pos += 4 * n
        return v

    def bytes(self, n: int) -> bytes:
        if self.pos + n > len(self.data):
            raise TacticalObjectError("truncated object")
        v = self.data[self.pos:self.pos + n]
        self.pos += n
        return v

    def finished(self) -> bool:
        return self.pos == len(self.data)


@dataclass
class Material:
    diffuse: tuple[float, float, float, float]
    specular_exponent: float
    specular: tuple[float, float, float]
    emissive: tuple[float, float, float]
    texture_name: str | None


@dataclass
class Chunk:
    material: int
    positions: list[tuple[float, float, float]]   # handedness applied (z negated)
    normals: list[tuple[float, float, float]]
    uvs: list[tuple[float, float]]
    triangles: list[tuple[int, int, int]]         # winding swapped (1,2) like the Rust decoder


@dataclass
class Mesh:
    materials: list[Material]
    chunks: list[Chunk]
    source_vertices: int
    source_faces: int
    normal_mode: int
    bounds_min: tuple[float, float, float]
    bounds_max: tuple[float, float, float]


@dataclass
class IndexedTexture:
    width: int
    height: int
    palette_id: int
    palette_rule: int
    indices: bytes
    trailing: bytes = b""

    @property
    def rule_name(self) -> str:
        return PALETTE_RULES.get(self.palette_rule, f"rule_{self.palette_rule}")


@dataclass
class Manifest:
    root: Path
    data: dict
    meshes_by_id: dict[int, dict] = field(default_factory=dict)
    textures_by_name: dict[str, dict] = field(default_factory=dict)
    textures_by_id: dict[int, dict] = field(default_factory=dict)

    @classmethod
    def load(cls, root: Path = RUNTIME_ROOT) -> "Manifest":
        data = json.loads((root / "manifest.json").read_text())
        m = cls(root, data)
        for rec in data["meshes"]:
            m.meshes_by_id[rec["id"]] = rec
        for rec in data["textures"]:
            if rec.get("identifier_kind") == "name":
                m.textures_by_name[rec["name"].upper()] = rec
            else:
                m.textures_by_id[rec["id"]] = rec
        return m

    def object_bytes(self, rec: dict) -> bytes:
        return (self.root / rec["object"]).read_bytes()

    def mesh(self, resource_id: int) -> Mesh:
        rec = self.meshes_by_id.get(resource_id)
        if rec is None:
            raise TacticalObjectError(f"mesh {resource_id} not in manifest")
        return decode_mesh(self.object_bytes(rec))

    def palette(self, resource_id: int = DEFAULT_BATTLE_PALETTE) -> list[tuple[int, int, int]]:
        rec = self.textures_by_id.get(resource_id)
        if rec is None or rec.get("kind") != "palette_rgb24":
            raise TacticalObjectError(f"palette {resource_id} not in manifest")
        return decode_palette(self.object_bytes(rec), resource_id)

    def texture_by_name(self, name: str) -> IndexedTexture:
        rec = self.textures_by_name.get(name.upper())
        if rec is None:
            raise TacticalObjectError(f"texture {name!r} not in manifest")
        return decode_indexed_texture(self.object_bytes(rec))

    def texture_by_id(self, resource_id: int) -> IndexedTexture:
        rec = self.textures_by_id.get(resource_id)
        if rec is None or rec.get("kind") != "indexed_rle":
            raise TacticalObjectError(f"indexed texture {resource_id} not in manifest")
        return decode_indexed_texture(self.object_bytes(rec))

    def palette_for(self, tex: IndexedTexture, battle_palette: int = DEFAULT_BATTLE_PALETTE):
        """Rule 1 uses the selected battle palette; rule 2 (planet_pair) binds palette id+30."""
        if tex.palette_rule == 2:
            return self.palette(tex.palette_id)
        return self.palette(battle_palette)

    def families(self) -> dict[int, list[int]]:
        """Mesh family base -> [close, medium, far] ids (resource_base + 0/1/2)."""
        fams: dict[int, list[int]] = {}
        for mid in sorted(self.meshes_by_id):
            fams.setdefault(mid - mid % 10, []).append(mid)
        return fams


# ------------------------------------------------------------------ decoders

def decode_mesh(data: bytes) -> Mesh:
    r = Reader(data)
    r.expect(MESH_MAGIC)
    if r.u32() != 1:
        raise TacticalObjectError("unsupported mesh version")
    n_materials, n_chunks, src_v, src_f, normal_mode = r.u32(), r.u32(), r.u32(), r.u32(), r.u32()
    if normal_mode > 1 or n_materials > 256 or n_chunks > 4096 or src_v > 1_000_000 or src_f > 1_000_000:
        raise TacticalObjectError("invalid mesh header")
    bounds = r.f32s(6)
    extent = max(bounds[3] - bounds[0], bounds[4] - bounds[1], bounds[5] - bounds[2])
    if not math.isfinite(extent) or extent <= 0.0:
        raise TacticalObjectError("invalid mesh bounds")
    materials = []
    for _ in range(n_materials):
        diffuse = r.f32s(4)
        spec_exp = r.f32()
        specular = r.f32s(3)
        emissive = r.f32s(3)
        name_len = r.u32()
        if name_len > 512:
            raise TacticalObjectError("texture name too long")
        try:
            name = r.bytes(name_len).decode("utf-8")
        except UnicodeDecodeError as exc:
            raise TacticalObjectError("invalid tactical texture name") from exc
        materials.append(Material(diffuse, spec_exp, specular, emissive, name or None))
    chunks = []
    for _ in range(n_chunks):
        material = r.u32()
        if material >= n_materials:
            raise TacticalObjectError("chunk references invalid material")
        vc, ic = r.u32(), r.u32()
        if vc == 0 or ic == 0 or ic % 3 or vc > 65536 or ic > 3_000_000:
            raise TacticalObjectError("invalid chunk size")
        positions, normals, uvs = [], [], []
        for _ in range(vc):
            px, py, pz, nx, ny, nz, u, v = r.f32s(8)
            positions.append((px, py, -pz))
            normals.append((nx, ny, -nz))
            uvs.append((u, v))
        idx = struct.unpack_from(f"<{ic}H", data, r.pos)
        r.pos += 2 * ic
        if max(idx) >= vc:
            raise TacticalObjectError("chunk index out of range")
        triangles = [(idx[i], idx[i + 2], idx[i + 1]) for i in range(0, ic, 3)]
        chunks.append(Chunk(material, positions, normals, uvs, triangles))
    if not r.finished():
        raise TacticalObjectError("mesh object has trailing bytes")
    return Mesh(materials, chunks, src_v, src_f, normal_mode, tuple(bounds[:3]), tuple(bounds[3:]))


def decode_palette(data: bytes, expected_id: int | None = None) -> list[tuple[int, int, int]]:
    r = Reader(data)
    r.expect(PALETTE_MAGIC)
    version, rid = r.u32(), r.u32()
    if version != 1 or (expected_id is not None and rid != expected_id):
        raise TacticalObjectError("invalid palette identity")
    rgb = r.bytes(256 * 3)
    if not r.finished():
        raise TacticalObjectError("palette object has trailing bytes")
    return [(rgb[i], rgb[i + 1], rgb[i + 2]) for i in range(0, 768, 3)]


def decode_indexed_texture(data: bytes) -> IndexedTexture:
    r = Reader(data)
    r.expect(TEXTURE_MAGIC)
    version, width, height, palette_id, rule, pixels, trailing = (r.u32() for _ in range(7))
    if version != 1 or width == 0 or height == 0 or width > 65535 or height > 65535 \
            or pixels != width * height or pixels > 16_777_216 or rule not in PALETTE_RULES:
        raise TacticalObjectError("invalid indexed texture header")
    indices = r.bytes(pixels)
    tail = r.bytes(trailing)
    if not r.finished():
        raise TacticalObjectError("indexed texture has unexpected extra bytes")
    return IndexedTexture(width, height, palette_id, rule, indices, tail)


def texture_to_image(tex: IndexedTexture, palette: list[tuple[int, int, int]]):
    """Mode-P image carrying the palette block (no transparency: textures are opaque)."""
    from PIL import Image
    img = Image.frombytes("P", (tex.width, tex.height), tex.indices)
    img.putpalette([c for rgb in palette for c in rgb])
    return img


def texture_to_rgba(tex: IndexedTexture, palette: list[tuple[int, int, int]]) -> bytes:
    lut = [bytes((r, g, b, 255)) for r, g, b in palette]
    return b"".join(lut[i] for i in tex.indices)


# ------------------------------------------------------------------- self-test

def main() -> int:
    """Decode every manifest object and report; exit 1 on any failure."""
    m = Manifest.load()
    errors = []
    for mid in sorted(m.meshes_by_id):
        try:
            mesh = m.mesh(mid)
            rec = m.meshes_by_id[mid]
            tri = sum(len(c.triangles) for c in mesh.chunks)
            if tri != rec["triangles"] or len(mesh.chunks) != rec["chunks"]:
                errors.append(f"mesh {mid}: counts differ from manifest")
        except (TacticalObjectError, struct.error) as exc:
            errors.append(f"mesh {mid}: {exc}")
    rules: dict[str, int] = {}
    tails = 0
    for rec in m.data["textures"]:
        try:
            if rec["kind"] == "palette_rgb24":
                m.palette(rec["id"])
                rules["palette"] = rules.get("palette", 0) + 1
            else:
                tex = decode_indexed_texture(m.object_bytes(rec))
                rules[tex.rule_name] = rules.get(tex.rule_name, 0) + 1
                tails += bool(tex.trailing)
                if (tex.width, tex.height) != (rec["width"], rec["height"]):
                    errors.append(f"texture {rec.get('name', rec.get('id'))}: size differs from manifest")
        except (TacticalObjectError, struct.error) as exc:
            errors.append(f"texture {rec.get('name', rec.get('id'))}: {exc}")
    print(f"meshes={len(m.meshes_by_id)} families={len(m.families())} textures={rules} trailing_tail={tails} errors={len(errors)}")
    for e in errors[:20]:
        print("  " + e)
    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
