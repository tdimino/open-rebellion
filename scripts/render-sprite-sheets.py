"""
Render 3D models to directional sprite sheets for Open Rebellion tactical combat.

Imports each GLB from data/models/optimized/ (or scripts/models-staging/ as fallback),
renders 8 orthographic directions at 30° isometric elevation, outputs PNG sprite
frames to data/models/sprites/.

Usage:
    blender --background --python scripts/render-sprite-sheets.py
    blender --background --python scripts/render-sprite-sheets.py -- --model isd-wavespeed
    blender --background --python scripts/render-sprite-sheets.py -- --resolution 128
    blender --background --python scripts/render-sprite-sheets.py -- --angles 16
    blender --background --python scripts/render-sprite-sheets.py -- --input scripts/models-staging

Requires: Blender 4.0+ (tested on 5.0.1)
"""

import bpy
import math
import os
import sys
from mathutils import Vector
from pathlib import Path


def parse_args():
    """Parse args after '--' separator (Blender consumes everything before it)."""
    argv = sys.argv
    if "--" in argv:
        argv = argv[argv.index("--") + 1:]
    else:
        argv = []

    import argparse
    parser = argparse.ArgumentParser(description="Render GLBs to sprite sheets")
    parser.add_argument("--model", help="Specific model filename (without .glb) to render")
    parser.add_argument("--input", default=None, help="Input directory for GLBs")
    parser.add_argument("--output", default=None, help="Output directory for sprites")
    parser.add_argument("--resolution", type=int, default=256, help="Render resolution per frame (default: 256)")
    parser.add_argument("--angles", type=int, default=8, help="Number of directional angles (default: 8)")
    parser.add_argument("--elevation", type=float, default=30.0, help="Camera elevation in degrees (default: 30)")
    return parser.parse_args(argv)


def find_project_root():
    """Walk up from script location to find project root (contains Cargo.toml)."""
    path = Path(__file__).resolve().parent.parent
    if (path / "Cargo.toml").exists():
        return path
    return Path.cwd()


def clear_scene():
    """Remove all objects from the scene."""
    bpy.ops.object.select_all(action='SELECT')
    bpy.ops.object.delete()
    # Also purge orphan data
    bpy.ops.outliner.orphans_purge(do_local_ids=True, do_linked_ids=True, do_recursive=True)


def import_glb(filepath):
    """Import a GLB file and return the imported mesh objects."""
    clear_scene()
    bpy.ops.import_scene.gltf(filepath=str(filepath))
    return [o for o in bpy.context.scene.objects if o.type == 'MESH']


def get_model_bounds(objects):
    """Calculate world-space bounding box of all mesh objects."""
    min_co = [float('inf')] * 3
    max_co = [float('-inf')] * 3
    for obj in objects:
        for v in obj.bound_box:
            world_v = obj.matrix_world @ Vector(v)
            for i in range(3):
                min_co[i] = min(min_co[i], world_v[i])
                max_co[i] = max(max_co[i], world_v[i])
    center = Vector([(min_co[i] + max_co[i]) / 2 for i in range(3)])
    size = max(max_co[i] - min_co[i] for i in range(3))
    return center, size


def setup_render(resolution):
    """Configure render settings for transparent sprite output."""
    scene = bpy.context.scene
    scene.render.resolution_x = resolution
    scene.render.resolution_y = resolution
    scene.render.film_transparent = True
    scene.render.image_settings.file_format = 'PNG'
    scene.render.image_settings.color_mode = 'RGBA'
    # Use Eevee for speed (sufficient for sprite rendering)
    # Blender 5.x: 'BLENDER_EEVEE', 4.x: 'BLENDER_EEVEE_NEXT'
    eevee_set = False
    for engine in ('BLENDER_EEVEE', 'BLENDER_EEVEE_NEXT'):
        try:
            scene.render.engine = engine
            eevee_set = True
            break
        except TypeError:
            continue
    if not eevee_set:
        print(f"WARNING: Could not set Eevee — using {scene.render.engine}. Renders may be slow.")


def setup_camera(size):
    """Create an orthographic camera sized to the model."""
    bpy.ops.object.camera_add()
    cam = bpy.context.active_object
    cam.data.type = 'ORTHO'
    cam.data.ortho_scale = size * 1.3  # slight padding
    bpy.context.scene.camera = cam
    return cam


def setup_lighting():
    """Add basic directional lighting."""
    bpy.ops.object.light_add(type='SUN', location=(5, 5, 10))
    light = bpy.context.active_object
    light.data.energy = 3.0


def render_directions(cam, center, size, num_angles, elevation_deg, output_dir, model_name):
    """Render the model from multiple directions."""
    distance = size * 2.5
    elevation = math.radians(elevation_deg)

    for i in range(num_angles):
        angle = (2 * math.pi * i) / num_angles
        deg = int(math.degrees(angle))

        x = center.x + distance * math.cos(angle) * math.cos(elevation)
        y = center.y + distance * math.sin(angle) * math.cos(elevation)
        z = center.z + distance * math.sin(elevation)

        cam.location = (x, y, z)

        # Point camera at model center
        direction = center - cam.location
        cam.rotation_euler = direction.to_track_quat('-Z', 'Y').to_euler()

        output_path = os.path.join(output_dir, f"{model_name}_{i:02d}_{deg:03d}deg.png")
        bpy.context.scene.render.filepath = output_path
        bpy.ops.render.render(write_still=True)
        print(f"  [{model_name}] Rendered {i+1}/{num_angles}: {deg}°")


def main():
    args = parse_args()
    root = find_project_root()

    # Determine input directory
    if args.input:
        input_dir = Path(args.input)
    else:
        # Prefer optimized models, fall back to staging
        optimized = root / "data" / "models" / "optimized"
        staging = root / "scripts" / "models-staging"
        input_dir = optimized if optimized.exists() and any(optimized.glob("*.glb")) else staging

    # Determine output directory
    output_dir = Path(args.output) if args.output else root / "data" / "models" / "sprites"
    output_dir.mkdir(parents=True, exist_ok=True)

    # Find GLBs to process
    if args.model:
        glb_file = input_dir / f"{args.model}.glb"
        if not glb_file.exists():
            # Try with suffix patterns
            matches = list(input_dir.glob(f"{args.model}*.glb"))
            if matches:
                glb_files = [matches[0]]
            else:
                print(f"ERROR: Model not found: {args.model} in {input_dir}")
                sys.exit(1)
        else:
            glb_files = [glb_file]
    else:
        glb_files = sorted(input_dir.glob("*.glb"))

    if not glb_files:
        print(f"No GLB files found in {input_dir}")
        sys.exit(1)

    print(f"Rendering {len(glb_files)} model(s) from {input_dir}")
    print(f"Output: {output_dir}")
    print(f"Resolution: {args.resolution}x{args.resolution}, {args.angles} angles, {args.elevation}° elevation")
    print()

    setup_render(args.resolution)

    for glb_path in glb_files:
        model_name = glb_path.stem
        print(f"[{model_name}] Importing {glb_path.name}...")

        meshes = import_glb(glb_path)
        if not meshes:
            print(f"  [{model_name}] ERROR: No mesh objects in GLB, skipping")
            continue

        center, size = get_model_bounds(meshes)
        print(f"  [{model_name}] {len(meshes)} meshes, size={size:.3f}")

        cam = setup_camera(size)
        setup_lighting()
        render_directions(cam, center, size, args.angles, args.elevation, str(output_dir), model_name)
        print()

    total = len(list(output_dir.glob("*.png")))
    print(f"Done. {total} sprite(s) in {output_dir}")


if __name__ == "__main__":
    main()
