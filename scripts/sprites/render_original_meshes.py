#!/usr/bin/env python3
"""Render the staged TACTICAL.DLL type-301 meshes to 8-direction sprite sheets (Track A).

Classification: experimental-remaster review artifact, runtime_eligible: false. The 1998
tactical view is Direct3D Retained Mode geometry; these sheets are a comparison aid and a
remaster input, never a parity deliverable and never a WASM fallback.

The rig is the recovered one, not a guess:
  camera  P57B2A  pitch 30 deg, Alliance yaw -30 (Empire 150), field 0.2 -> fovy = 2*atan(0.2),
                  near 1.0, far = distance * 2.5, position/up from OriginalTacticalCamera::pose
  light   P57B2C2A directional RGB 0.8 from surface-to-light (5,5,1) normalised (source frame
                  (5,5,-1) reflected), ambient 0.5, no specular, Gouraud per vertex:
                  light = min(1, 0.5 + 0.8 * max(0, n.L)); material = min(1, diffuse*light + emissive)
                  colour = texel * material            (crates/rebellion-render/src/tactical_assets.rs)
  sampling nearest, no mip, back-face cull, CW front after the z-reflection
Lighting is evaluated here per vertex in Python and baked into a colour attribute, so Blender is
only a rasteriser: Eevee, one sample, pixel filter 0, view transform Raw, textures Non-Color.
Camera distance cannot follow the battle rule (extent * 1.7 frames the whole battle); each LOD
family gets one shared fit distance so close/medium/far share scale. That and any --elevation /
--yaw override are recorded in provenance as non-authentic.

Usage (outer process; it spawns Blender itself):
  uv run scripts/sprites/render_original_meshes.py --ids 2560 2561 2562            # proof family
  uv run scripts/sprites/render_original_meshes.py --all --cell 128
  uv run scripts/sprites/render_original_meshes.py --ids 2560 --unlit --check-palette
     (white-tinted textured chunks must hit a palette entry exactly; tinted textured chunks and
      untextured chunks, the far LOD of every family, must hit texel*tint or the tint itself
      within +/-1 per channel; provenance records off_palette (must be 0) and tinted_pixels)

Outputs under data/models/sprites/ (gitignored, derived from original game data):
  frames/<id>/frame_00..07.png, <id>.png + <id>.json (sprite-forge atlas-v2, directions 8),
  provenance.json, review/<base>.png (+ index.html) at black / white / palette-0 / checkerboard.
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import math
import os
import shutil
import subprocess
import sys
from pathlib import Path

SCRIPT_PATH = Path(__file__).resolve()
SPRITES_DIR = SCRIPT_PATH.parent
PROJECT_ROOT = SPRITES_DIR.parents[1]
sys.path.insert(0, str(SPRITES_DIR))
import tactical3d  # noqa: E402

DEFAULT_OUTPUT = PROJECT_ROOT / "data" / "models" / "sprites"
DEFAULT_SPRITE_FORGE = Path(os.environ.get("SPRITE_FORGE_DIR", Path.home() / ".claude" / "skills" / "sprite-forge"))
BLENDER = os.environ.get("BLENDER", "/opt/homebrew/bin/blender")
DIRECTIONS = ["S", "SE", "E", "NE", "N", "NW", "W", "SW"]

# Recovered constants (tactical_assets.rs). Names mirror the Rust so a diff is a grep away.
ORIGINAL_CAMERA_PITCH = 30
ORIGINAL_CAMERA_ALLIANCE_YAW = -30
ORIGINAL_CAMERA_EMPIRE_YAW = 150
ORIGINAL_CAMERA_FIELD = 0.2
ORIGINAL_CAMERA_NEAR = 1.0
ORIGINAL_CAMERA_FAR_SCALE = 2.5
ORIGINAL_CAMERA_DISTANCE_SCALE = 1.7   # battle rule, recorded but not applied per model
ORIGINAL_LIGHT_FRAME_SOURCE_POSITION = (5.0, 5.0, -1.0)
ORIGINAL_AMBIENT_LIGHT_RGB = 0.5
ORIGINAL_DIRECTIONAL_LIGHT_RGB = 0.8
FIT_MARGIN = 1.15
EVIDENCE = {"camera": "P57B2A (docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-camera-contract.md)",
            "light": "P57B2C2A (docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-light-rig.md)"}


# ------------------------------------------------------------------ maths shared by both halves

def normalise(v):
    n = math.sqrt(sum(c * c for c in v))
    return tuple(c / n for c in v) if n else v


def surface_to_light():
    x, y, z = ORIGINAL_LIGHT_FRAME_SOURCE_POSITION
    return normalise((x, y, -z))   # authored_position: reflect z


def rotate_y(p, degrees):
    """Right-handed rotation about +Y (the meshes are Y-up; z was reflected by the decoder)."""
    t = math.radians(degrees)
    c, s = math.cos(t), math.sin(t)
    x, y, z = p
    return (x * c + z * s, y, -x * s + z * c)


def camera_pose(pitch_deg, yaw_deg, distance):
    """OriginalTacticalCamera::pose, verbatim."""
    if yaw_deg > 180:
        yaw_deg -= 360
    if yaw_deg < -180:
        yaw_deg += 360
    p, y = math.radians(pitch_deg), math.radians(yaw_deg)
    sp, cp, sy, cy = math.sin(p), math.cos(p), math.sin(y), math.cos(y)
    position = (-distance * sp * sy, distance * sp * cy, distance * cp)
    up = (-cp * sy, cp * cy, -sp)
    return position, up


def vertex_material(normal, diffuse, emissive, light_dir):
    """Per-vertex Gouraud term of the runtime shader: min(1, diffuse.rgb*light + emissive)."""
    n = normalise(normal)
    d = max(0.0, n[0] * light_dir[0] + n[1] * light_dir[1] + n[2] * light_dir[2])
    light = min(1.0, ORIGINAL_AMBIENT_LIGHT_RGB + ORIGINAL_DIRECTIONAL_LIGHT_RGB * d)
    return tuple(min(1.0, diffuse[i] * light + emissive[i]) for i in range(3)) + (diffuse[3],)


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


# ------------------------------------------------------------------ planning (outer process)

def family_frame(manifest: tactical3d.Manifest, ids: list[int]) -> tuple[tuple[float, float, float], float]:
    """Shared target and bounding radius for a LOD family so all three LODs render at one scale."""
    mins = [manifest.meshes_by_id[i]["bounds_min"] for i in ids]
    maxs = [manifest.meshes_by_id[i]["bounds_max"] for i in ids]
    lo = [min(m[k] for m in mins) for k in range(3)]
    hi = [max(m[k] for m in maxs) for k in range(3)]
    # decoder reflects z: bounds z flips sign
    centre = ((lo[0] + hi[0]) / 2, (lo[1] + hi[1]) / 2, -(lo[2] + hi[2]) / 2)
    radius = 0.0
    for i in ids:
        for chunk in manifest.mesh(i).chunks:
            for p in chunk.positions:
                radius = max(radius, math.dist(p, centre))
    return centre, radius


def build_jobs(manifest, ids, args, out_root: Path) -> list[dict]:
    fams = manifest.families()
    by_id = {mid: base for base, members in fams.items() for mid in members}
    wanted = sorted(set(ids))
    unknown = [i for i in wanted if i not in by_id]
    if unknown:
        sys.exit(f"not in runtime manifest: {unknown}")
    fovy = 2.0 * math.atan(args.field)
    jobs = []
    cache: dict[int, tuple] = {}
    for mid in wanted:
        base = by_id[mid]
        if base not in cache:
            cache[base] = family_frame(manifest, fams[base])
        centre, radius = cache[base]
        distance = radius / math.tan(fovy / 2.0) * FIT_MARGIN if args.projection == "perspective" else radius * 4.0
        near = ORIGINAL_CAMERA_NEAR
        near_override = distance - radius * 1.5 < near
        if near_override:
            near = max(0.01, distance - radius * 1.5)
        position, up = camera_pose(args.pitch, args.yaw, distance)
        position = tuple(position[k] + centre[k] for k in range(3))
        jobs.append({
            "id": mid, "family": base, "lod": tactical3d.LOD_LABELS[mid - base] if mid - base < 3 else str(mid - base),
            "frames_dir": str(out_root / "frames" / str(mid)),
            "cell": args.cell, "unlit": args.unlit, "palette": args.palette,
            "target": centre, "radius": radius, "distance": distance, "near": near, "far": distance * ORIGINAL_CAMERA_FAR_SCALE,
            "fovy": fovy, "projection": args.projection, "ortho_scale": 2 * radius * FIT_MARGIN,
            "camera_position": position, "camera_up": up,
            "headings": [k * 360.0 / len(DIRECTIONS) for k in range(len(DIRECTIONS))],
            "near_override": near_override,
        })
    return jobs


# ------------------------------------------------------------------ Blender worker

def worker(spec_path: Path) -> None:
    import bpy  # type: ignore
    from mathutils import Matrix, Vector  # type: ignore

    spec = json.loads(spec_path.read_text())
    manifest = tactical3d.Manifest.load(Path(spec["runtime_root"]))
    light_dir = surface_to_light()
    report = {"jobs": [], "blender": bpy.app.version_string}

    scene = bpy.context.scene
    scene.render.engine = "BLENDER_EEVEE"
    scene.render.film_transparent = True
    scene.render.image_settings.file_format = "PNG"
    scene.render.image_settings.color_mode = "RGBA"
    scene.render.image_settings.color_depth = "8"
    scene.render.filter_size = 0.0
    scene.render.dither_intensity = 0.0
    scene.eevee.taa_render_samples = 1
    scene.view_settings.view_transform = "Raw"
    scene.view_settings.look = "None"
    scene.view_settings.exposure = 0.0
    scene.view_settings.gamma = 1.0
    scene.display_settings.display_device = "sRGB"
    scene.render.resolution_percentage = 100

    def clear():
        bpy.ops.object.select_all(action="SELECT")
        bpy.ops.object.delete()
        for coll in (bpy.data.meshes, bpy.data.materials, bpy.data.images, bpy.data.cameras):
            for block in list(coll):
                if block.users == 0:
                    coll.remove(block)

    def make_texture_image(name: str, tex, palette):
        rgba = tactical3d.texture_to_rgba(tex, palette)
        img = bpy.data.images.new(name, tex.width, tex.height, alpha=True)
        img.colorspace_settings.name = "Non-Color"
        # Blender stores rows bottom-up; write the last source row first so UV v maps as 1 - v.
        rows = [rgba[r * tex.width * 4:(r + 1) * tex.width * 4] for r in range(tex.height)]
        flat = b"".join(reversed(rows))
        img.pixels.foreach_set([b / 255.0 for b in flat])
        img.pack()
        return img

    def make_material(name: str, image):
        mat = bpy.data.materials.new(name)
        mat.use_nodes = True
        mat.use_backface_culling = True
        if hasattr(mat, "blend_method"):   # removed for Eevee in newer Blender; alpha < 1 never occurs in the corpus
            mat.blend_method = "OPAQUE"
        nodes, links = mat.node_tree.nodes, mat.node_tree.links
        nodes.clear()
        out = nodes.new("ShaderNodeOutputMaterial")
        emit = nodes.new("ShaderNodeEmission")
        emit.inputs["Strength"].default_value = 1.0
        attr = nodes.new("ShaderNodeVertexColor")
        attr.layer_name = "material"
        mul = nodes.new("ShaderNodeMix")
        mul.data_type = "RGBA"
        mul.blend_type = "MULTIPLY"
        mul.inputs["Factor"].default_value = 1.0
        if image is not None:
            tex = nodes.new("ShaderNodeTexImage")
            tex.image = image
            tex.interpolation = "Closest"
            tex.extension = "REPEAT"
            links.new(tex.outputs["Color"], mul.inputs[6])
        else:
            mul.inputs[6].default_value = (1.0, 1.0, 1.0, 1.0)   # untextured: texel is white
        links.new(attr.outputs["Color"], mul.inputs[7])
        links.new(mul.outputs[2], emit.inputs["Color"])
        # alpha = texel.a * tint.a like the runtime fragment shader (every corpus material has a = 1).
        transparent = nodes.new("ShaderNodeBsdfTransparent")
        mix = nodes.new("ShaderNodeMixShader")
        links.new(attr.outputs["Alpha"], mix.inputs["Fac"])
        links.new(transparent.outputs["BSDF"], mix.inputs[1])
        links.new(emit.outputs["Emission"], mix.inputs[2])
        links.new(mix.outputs["Shader"], out.inputs["Surface"])
        return mat

    for job in spec["jobs"]:
        clear()
        mesh = manifest.mesh(job["id"])
        palette = manifest.palette(job["palette"])
        frames_dir = Path(job["frames_dir"])
        frames_dir.mkdir(parents=True, exist_ok=True)
        # One Blender mesh per chunk (material), rebuilt per heading with baked vertex material.
        images = {}
        for mi, mat in enumerate(mesh.materials):
            if mat.texture_name and mat.texture_name not in images:
                tex = manifest.texture_by_name(mat.texture_name)
                images[mat.texture_name] = make_texture_image(mat.texture_name, tex, manifest.palette_for(tex, job["palette"]))
        materials = [make_material(f"m{job['id']}_{mi}", images.get(mat.texture_name)) for mi, mat in enumerate(mesh.materials)]

        cam_data = bpy.data.cameras.new("cam")
        if job["projection"] == "perspective":
            cam_data.type = "PERSP"
            cam_data.sensor_fit = "VERTICAL"
            cam_data.lens_unit = "FOV"
            cam_data.angle_y = job["fovy"]
        else:
            cam_data.type = "ORTHO"
            cam_data.ortho_scale = job["ortho_scale"]
        cam_data.clip_start = job["near"]
        cam_data.clip_end = job["far"]
        cam = bpy.data.objects.new("cam", cam_data)
        scene.collection.objects.link(cam)
        scene.camera = cam
        pos, up = Vector(job["camera_position"]), Vector(job["camera_up"])
        target = Vector(job["target"])
        fwd = (target - pos).normalized()
        right = fwd.cross(up).normalized()
        true_up = right.cross(fwd).normalized()
        rot = Matrix((right, true_up, -fwd)).transposed()   # columns: cam X, Y, -Z(view)
        cam.matrix_world = Matrix.Translation(pos) @ rot.to_4x4()
        scene.render.resolution_x = scene.render.resolution_y = job["cell"]

        winding_votes = [0, 0]
        for k, heading in enumerate(job["headings"]):
            objs = []
            for ci, chunk in enumerate(mesh.chunks):
                mat = mesh.materials[chunk.material]
                verts = [rotate_y(tuple(p[j] - job["target"][j] for j in range(3)), heading) for p in chunk.positions]
                verts = [tuple(v[j] + job["target"][j] for j in range(3)) for v in verts]
                norms = [rotate_y(n, heading) for n in chunk.normals]
                # tactical3d's (i0, i2, i1) order is CCW-front in the reflected space: every proof-family
                # triangle's geometric normal agrees with its source normals in that order (0 disagree).
                faces = list(chunk.triangles)
                if k == 0:
                    for a, b, c in faces:
                        e1 = Vector(verts[b]) - Vector(verts[a])
                        e2 = Vector(verts[c]) - Vector(verts[a])
                        geo = e1.cross(e2)
                        avg = Vector(norms[a]) + Vector(norms[b]) + Vector(norms[c])
                        winding_votes[0 if geo.dot(avg) >= 0 else 1] += 1
                me = bpy.data.meshes.new(f"chunk{ci}")
                me.from_pydata(verts, [], faces)
                me.update()
                uv = me.uv_layers.new(name="uv")
                col = me.color_attributes.new(name="material", type="FLOAT_COLOR", domain="POINT")
                for li, loop in enumerate(me.loops):
                    u, v = chunk.uvs[loop.vertex_index]
                    uv.data[li].uv = (u, 1.0 - v)
                for vi in range(len(verts)):
                    if job["unlit"]:
                        # skip only the light term: tint = min(1, diffuse + emissive), so a textured
                        # chunk passes its texel through and an untextured one shows its real tint
                        col.data[vi].color = tuple(min(1.0, mat.diffuse[i] + mat.emissive[i]) for i in range(3)) + (mat.diffuse[3],)
                    else:
                        col.data[vi].color = vertex_material(norms[vi], mat.diffuse, mat.emissive, light_dir)
                me.materials.append(materials[chunk.material])
                ob = bpy.data.objects.new(f"chunk{ci}", me)
                scene.collection.objects.link(ob)
                objs.append(ob)
            out_path = frames_dir / f"frame_{k:02d}.png"
            scene.render.filepath = str(out_path)
            bpy.ops.render.render(write_still=True)
            for ob in objs:
                me = ob.data
                bpy.data.objects.remove(ob)
                bpy.data.meshes.remove(me)
        report["jobs"].append({"id": job["id"], "frames": len(job["headings"]),
                               "winding_agree": winding_votes[0], "winding_disagree": winding_votes[1],
                               "textures": sorted(images)})
        print(f"[render] {job['id']} ({job['lod']}) {len(job['headings'])} frames -> {frames_dir}")
    Path(spec["report"]).write_text(json.dumps(report, indent=2))


# ------------------------------------------------------------------ post-processing (outer process)

def load_module(path: Path, name: str):
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    sys.modules[name] = mod
    spec.loader.exec_module(mod)
    return mod


def material_tint(m) -> tuple[float, float, float]:
    """What --unlit bakes per vertex: min(1, diffuse + emissive), the runtime material term without light."""
    return tuple(min(1.0, m.diffuse[i] + m.emissive[i]) for i in range(3))


def allowed_colours(mesh, palette) -> tuple[set[tuple[int, int, int]], set[tuple[int, int, int]]]:
    """(exact palette entries, tinted colours) an --unlit render of `mesh` may contain.

    The runtime fragment shader outputs texel * material, so a textured chunk whose material tint is
    not pure white (e.g. carrak52 at 0.976, 0.976, 1.0) legitimately leaves the palette: its colours
    are palette entries scaled by the tint. An untextured chunk (the far LOD of every family) has no
    texel and renders the tint itself.
    """
    exact, tinted = set(), set()
    for m in mesh.materials:
        tint = material_tint(m)
        if m.texture_name:
            if all(abs(c - 1.0) < 1e-6 for c in tint):
                exact.update(tuple(c) for c in palette)
            else:
                tinted.update(tuple(int(round(c * t)) for c, t in zip(rgb, tint)) for rgb in palette)
        else:
            tinted.add(tuple(int(round(t * 255)) for t in tint))
    return exact, tinted


def check_frames(frames_dir: Path, exact, tinted, unlit: bool, check_palette: bool) -> dict:
    """Frame count, binary alpha and, under --unlit --check-palette, colour pass-through.

    Pixels of white-tinted textured chunks must be palette entries exactly; pixels of tinted textured
    chunks and of untextured chunks must be the tint-scaled colour within +/-1 per channel (float ->
    8-bit rounding). `tinted_pixels` counts the latter, `off_palette` (must be 0) everything else.
    """
    from PIL import Image
    result = {"frames": 0, "alpha_binary": True, "opaque_pixels": 0, "off_palette": 0, "tinted_pixels": 0,
              "tinted_materials": bool(tinted)}

    def near_tinted(rgb):
        return any((rgb[0] + dx, rgb[1] + dy, rgb[2] + dz) in tinted
                   for dx in (-1, 0, 1) for dy in (-1, 0, 1) for dz in (-1, 0, 1))

    for f in sorted(frames_dir.glob("frame_*.png")):
        with Image.open(f) as im:
            raw = im.convert("RGBA").tobytes()
        result["frames"] += 1
        for i in range(0, len(raw), 4):
            r, g, b, a = raw[i], raw[i + 1], raw[i + 2], raw[i + 3]
            if a not in (0, 255):
                result["alpha_binary"] = False
            if a == 255:
                result["opaque_pixels"] += 1
                if unlit and check_palette and (r, g, b) not in exact:
                    if near_tinted((r, g, b)):
                        result["tinted_pixels"] += 1
                    else:
                        result["off_palette"] += 1
    return result


def stitch(job: dict, out_root: Path, sprite_forge: Path, palette_id: int, provenance_extra: dict) -> Path:
    stitch_py = sprite_forge / "scripts" / "stitch_spritesheet.py"
    if not stitch_py.exists():
        sys.exit(f"sprite-forge not found at {sprite_forge} (set SPRITE_FORGE_DIR or --sprite-forge)")
    sheet = out_root / f"{job['id']}.png"
    cmd = ["python3", str(stitch_py), "--input-dir", job["frames_dir"], "--cols", str(len(DIRECTIONS)),
           "--cell-size", f"{job['cell']}x{job['cell']}", "--fit", "origin", "--atlas", "v2",
           "--directions", ",".join(DIRECTIONS), "--state", "idle:1:once",
           "--generator", "open-rebellion/render_original_meshes.py", "-o", str(sheet)]
    for k, v in {"resource_id": job["id"], "family": job["family"], "lod": job["lod"], "palette": palette_id,
                 "classification": "experimental-remaster", "runtime_eligible": "false", **provenance_extra}.items():
        cmd += ["--provenance", f"{k}={v}"]
    r = subprocess.run(cmd, capture_output=True, text=True)
    if r.returncode != 0:
        sys.exit(f"stitch failed for {job['id']}:\n{r.stdout}\n{r.stderr}")
    return sheet


def review_sheet(base: int, jobs: list[dict], out_root: Path, palette0) -> Path | None:
    from PIL import Image, ImageDraw
    review_dir = out_root / "review"
    review_dir.mkdir(parents=True, exist_ok=True)
    cell = jobs[0]["cell"]
    backgrounds = [("black", (0, 0, 0)), ("white", (255, 255, 255)), ("palette-0", tuple(palette0)), ("checker", None)]
    label_h = 14
    block_w = len(DIRECTIONS) * cell
    block_h = len(backgrounds) * (cell + label_h)
    sheet = Image.new("RGB", (block_w, block_h * len(jobs)), (40, 40, 40))
    draw = ImageDraw.Draw(sheet)
    for ji, job in enumerate(sorted(jobs, key=lambda j: j["id"])):
        frames = sorted(Path(job["frames_dir"]).glob("frame_*.png"))
        for bi, (bname, colour) in enumerate(backgrounds):
            y0 = ji * block_h + bi * (cell + label_h)
            draw.text((2, y0), f"{job['id']} {job['lod']} on {bname}", fill=(230, 230, 230))
            for k, f in enumerate(frames):
                x0 = k * cell
                if colour is None:
                    bg = Image.new("RGB", (cell, cell), (200, 200, 200))
                    d2 = ImageDraw.Draw(bg)
                    step = max(4, cell // 8)
                    for yy in range(0, cell, step):
                        for xx in range(0, cell, step):
                            if ((xx // step) + (yy // step)) % 2:
                                d2.rectangle([xx, yy, xx + step - 1, yy + step - 1], fill=(120, 120, 120))
                else:
                    bg = Image.new("RGB", (cell, cell), colour)
                with Image.open(f) as fr:
                    fr = fr.convert("RGBA")
                    bg.paste(fr, (0, 0), fr)
                sheet.paste(bg, (x0, y0 + label_h))
    path = review_dir / f"{base}.png"
    sheet.save(path)
    return path


def write_review_index(out_root: Path, review_paths: list[Path]) -> None:
    helper = PROJECT_ROOT / "scripts" / "build-comparison-html.py"
    if not helper.exists() or not review_paths:
        return
    mod = load_module(helper, "build_comparison_html")
    rows = "\n".join(f"<h2>Family {p.stem}</h2><img src=\"{mod.img_to_data_uri(p)}\" style=\"image-rendering:pixelated;max-width:100%\">"
                     for p in review_paths)
    (out_root / "review" / "index.html").write_text(
        "<!doctype html><meta charset=utf-8><title>Original mesh direction sheets (review)</title>"
        "<body style='background:#111;color:#ddd;font-family:system-ui;padding:16px'>"
        "<p>experimental-remaster review artifact; runtime_eligible: false. Rows per LOD: black, white, palette-0, checkerboard.</p>"
        + rows)


# ------------------------------------------------------------------ main

def main() -> int:
    argv = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else sys.argv[1:]
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--ids", type=int, nargs="*", help="mesh resource ids (any LOD; framing is per family)")
    p.add_argument("--all", action="store_true", help="every mesh in the runtime manifest")
    p.add_argument("--cell", type=int, default=128, help="frame size in px (128 standard, 256 hero)")
    p.add_argument("--palette", type=int, default=tactical3d.DEFAULT_BATTLE_PALETTE, help="battle palette id for rule-1 textures")
    p.add_argument("--faction", choices=["alliance", "empire"], default="alliance", help="camera yaw: -30 or 150")
    p.add_argument("--elevation", type=float, help="override pitch degrees (recorded as non-authentic)")
    p.add_argument("--yaw", type=float, help="override camera yaw degrees (recorded as non-authentic)")
    p.add_argument("--projection", choices=["perspective", "ortho"], default="perspective")
    p.add_argument("--unlit", action="store_true", help="skip the light term (material = diffuse, emissive) for palette checks")
    p.add_argument("--check-palette", action="store_true", help="with --unlit: every opaque pixel must be a palette colour")
    p.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    p.add_argument("--runtime-root", type=Path, default=tactical3d.RUNTIME_ROOT)
    p.add_argument("--sprite-forge", type=Path, default=DEFAULT_SPRITE_FORGE)
    p.add_argument("--no-review", action="store_true")
    p.add_argument("--worker", type=Path, help=argparse.SUPPRESS)
    a = p.parse_args(argv)

    if a.worker:
        worker(a.worker)
        return 0

    manifest = tactical3d.Manifest.load(a.runtime_root)
    ids = sorted(manifest.meshes_by_id) if a.all else (a.ids or [])
    if not ids:
        sys.exit("give --ids or --all")
    a.field = ORIGINAL_CAMERA_FIELD
    overrides = {k: v for k, v in {"elevation": a.elevation, "yaw": a.yaw, "unlit": a.unlit or None,
                                    "projection": None if a.projection == "perspective" else a.projection}.items() if v is not None}
    a.pitch = ORIGINAL_CAMERA_PITCH if a.elevation is None else a.elevation
    a.yaw = (ORIGINAL_CAMERA_EMPIRE_YAW if a.faction == "empire" else ORIGINAL_CAMERA_ALLIANCE_YAW) if a.yaw is None else a.yaw
    out_root = a.output.resolve()
    out_root.mkdir(parents=True, exist_ok=True)
    jobs = build_jobs(manifest, ids, a, out_root)
    for job in jobs:
        shutil.rmtree(job["frames_dir"], ignore_errors=True)

    work = out_root / "work"
    work.mkdir(exist_ok=True)
    spec_path, report_path = work / "spec.json", work / "render-report.json"
    spec_path.write_text(json.dumps({"runtime_root": str(a.runtime_root), "jobs": jobs, "report": str(report_path)}, indent=2))
    if not Path(BLENDER).exists() and shutil.which(BLENDER) is None:
        sys.exit(f"Blender not found at {BLENDER} (set BLENDER)")
    cmd = [BLENDER, "--background", "--python", str(SCRIPT_PATH), "--", "--worker", str(spec_path)]
    print(f"$ {' '.join(cmd)}")
    r = subprocess.run(cmd, capture_output=True, text=True)
    for line in r.stdout.splitlines():
        if line.startswith("[render]") or "Error" in line or "Traceback" in line:
            print(line)
    if r.returncode != 0 or not report_path.exists():
        print(r.stdout[-3000:], r.stderr[-3000:], sep="\n", file=sys.stderr)
        sys.exit("Blender worker failed")
    report = json.loads(report_path.read_text())
    winding = {j["id"]: j for j in report["jobs"]}

    rig = {
        "evidence": EVIDENCE, "blender": report["blender"], "script_sha256": sha256_file(SCRIPT_PATH),
        "tactical3d_sha256": sha256_file(SPRITES_DIR / "tactical3d.py"),
        "pitch_degrees": a.pitch, "yaw_degrees": a.yaw, "field": a.field, "fovy_radians": 2 * math.atan(a.field),
        "near": ORIGINAL_CAMERA_NEAR, "far_scale": ORIGINAL_CAMERA_FAR_SCALE, "projection": a.projection,
        "distance_rule": {"original": f"battle_extent * {ORIGINAL_CAMERA_DISTANCE_SCALE}",
                          "applied": f"family_radius / tan(fovy/2) * {FIT_MARGIN} (per LOD family, non-authentic framing)"},
        "light": {"surface_to_light": surface_to_light(), "directional_rgb": ORIGINAL_DIRECTIONAL_LIGHT_RGB,
                  "ambient_rgb": ORIGINAL_AMBIENT_LIGHT_RGB, "shading": "gouraud (per vertex, baked)", "specular": False,
                  "unlit": a.unlit},
        "sampling": {"filter": "nearest", "pixel_filter": 0.0, "samples": 1, "view_transform": "Raw", "texture_colorspace": "Non-Color"},
        "rotation": "ship yaw about +Y per direction, camera fixed",
        "rig": "override" if overrides else "authentic",
        "overrides": overrides,
    }
    palette = manifest.palette(a.palette)
    prov_path = out_root / "provenance.json"
    provenance = json.loads(prov_path.read_text()) if prov_path.exists() else {"schema": "open-rebellion/mesh-sprites-v1", "sheets": {}}
    provenance.update({"classification": "experimental-remaster", "runtime_eligible": False, "rig": rig,
                       "palette": {"id": a.palette, "sha256": hashlib.sha256(bytes(c for rgb in palette for c in rgb)).hexdigest()},
                       "directions": DIRECTIONS})
    failures = []
    for job in jobs:
        frames_dir = Path(job["frames_dir"])
        exact, tinted = allowed_colours(manifest.mesh(job["id"]), palette)
        check = check_frames(frames_dir, exact, tinted, a.unlit, a.check_palette)
        w = winding[job["id"]]
        if check["frames"] != len(DIRECTIONS):
            failures.append(f"{job['id']}: {check['frames']} frames")
        if not check["alpha_binary"]:
            failures.append(f"{job['id']}: anti-aliased alpha")
        if a.unlit and a.check_palette and check["off_palette"]:
            failures.append(f"{job['id']}: {check['off_palette']} opaque pixels off palette and off the tinted colours")
        if w["winding_disagree"] > w["winding_agree"]:
            failures.append(f"{job['id']}: winding disagrees with source normals ({w['winding_disagree']} vs {w['winding_agree']})")
        rec = manifest.meshes_by_id[job["id"]]
        sheet = stitch(job, out_root, a.sprite_forge, a.palette, {"rig": rig["rig"], "mesh_object_sha256": rec["object_sha256"]})
        textures = {}
        for name in w["textures"]:
            trec = manifest.textures_by_name[name.upper()]
            textures[name] = {"object_sha256": trec.get("object_sha256"), "source_sha256": trec.get("source_sha256")}
        provenance["sheets"][str(job["id"])] = {
            "family": job["family"], "lod": job["lod"], "sheet": sheet.name, "manifest": sheet.with_suffix(".json").name,
            "sheet_sha256": sha256_file(sheet), "cell": job["cell"], "headings": job["headings"],
            "mesh": {"object": rec["object"], "object_sha256": rec["object_sha256"], "source_sha256": rec["source_sha256"]},
            "textures": textures, "framing": {"target": job["target"], "radius": job["radius"], "distance": job["distance"],
                                              "near": job["near"], "near_override": job["near_override"], "far": job["far"]},
            "checks": {**check, "winding_agree": w["winding_agree"], "winding_disagree": w["winding_disagree"]},
        }
        print(f"[sheet] {sheet.name} ({job['lod']}) opaque={check['opaque_pixels']} winding={w['winding_agree']}/{w['winding_agree'] + w['winding_disagree']}")
    prov_path.write_text(json.dumps(provenance, indent=2))

    if not a.no_review:
        review_paths = []
        for base in sorted({j["family"] for j in jobs}):
            fam_jobs = [j for j in jobs if j["family"] == base]
            rp = review_sheet(base, fam_jobs, out_root, palette[0])
            if rp:
                review_paths.append(rp)
        write_review_index(out_root, review_paths)
        print(f"[review] {len(review_paths)} family sheet(s) -> {out_root / 'review'}")
    print(f"rendered {len(jobs)} sheet(s); provenance {prov_path}")
    if failures:
        for f in failures:
            print(f"FAIL {f}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
