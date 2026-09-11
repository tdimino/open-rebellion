# Faithful HD Pipeline

Research cutoff: 2026-09-10. This plan supersedes the assumption that one
neural model should process every original bitmap. It preserves the 1998 assets
as the sole parity baseline and treats higher-resolution graphics as an optional
enhancement.

The machine-readable companion is [plan.json](plan.json).

## Decision

Open Rebellion has three distinct visual profiles:

| Profile | Pixel source | Role |
|---|---|---|
| `original-parity` | Original extracted resources with nearest sampling | Default and the only profile allowed to close interface-parity cells |
| `faithful-hd` | Reviewed, deterministic or constrained 4x assets | Optional enhancement with original-family fallback |
| `experimental-remaster` | Generative or semantic restoration | Research only; never parity evidence or a release default |

Newer diffusion super-resolution methods can produce plausible high-frequency
detail, but plausibility is not identity. Their fidelity and realism controls
also demonstrate that several visually reasonable outputs can exist for one
input. That is useful for a remaster and disqualifying for strict parity.

## Current implementation

P45 begins with fail-closed foundations:

- `BmpCache` and the encyclopedia now default to `original-parity` and use
  explicit nearest-neighbor sampling for original resources.
- `data/hd` is never consulted unless native startup explicitly sets
  `OPEN_REBELLION_ASSET_PROFILE=faithful-hd`.
- Faithful-HD lookup requires `data/hd/manifest.json` and only accepts assets
  with complete review metadata plus matching source-BMP and output-PNG hashes.
  The renderer decodes the exact output bytes it verified.
- `scripts/faithful_hd_pipeline.py` preserves indexed palettes, performs exact
  4x nearest scaling, records source, palette, route, configuration, generator,
  and output hashes, verifies logical reconstruction, and requires an explicit
  reviewer plus evidence before approval.
- The previous UltraSharp batch script remains available for comparisons, but
  its raw outputs are runtime-ineligible until imported into this contract.
- Browser builds remain on `original-parity` until a reviewed ORPK revision can
  carry both manifest approvals and optional HD chunks.

The profile can be tested natively with:

```bash
OPEN_REBELLION_ASSET_PROFILE=faithful-hd cargo run -p rebellion-app -- data/base
```

No HD file is selected unless its manifest record is approved.

## Method routing

| Asset family | Faithful-HD route | Reason |
|---|---|---|
| Text, glyphs, cursors, controls, chrome, masks | Nearest 4x; evaluate Scale2x/4x only as a family-wide challenger | Preserves counts, palette indices, silhouettes, and interaction-state identity |
| Advisor and other indexed animation | One deterministic transform for the whole decoded sequence | Prevents per-frame shimmer and preserves authored delta regions |
| Small decorative sprites | Nearest or reviewed Scale2x/4x; xBRZ only after license review | Edge smoothing can change silhouettes |
| Portraits, planets, large ships, backgrounds | Compare SwinIR and HAT first; retain Real-ESRGAN and UltraSharp as challengers | Conventional reconstruction is safer than semantic regeneration, but still requires masks and review |
| Text-bearing painted images | Protect original glyph regions; do not ask a general SR model to reconstruct them | Current scene-text research reports character-structure distortion from general SR |
| Generative diffusion methods | `experimental-remaster` only | Detail is regenerated rather than recovered |

UltraSharp V2 is no longer the universal winner. The earlier twenty-image
shootout remains useful preference evidence, but it lacks family completeness,
provenance, repeated-output hashes, alpha tests, and parity metrics.

## Learned-model experiment

P45B will compare SwinIR, HAT, Real-ESRGAN, and UltraSharp on complete asset
families rather than isolated samples. Each candidate must use one pinned CPU or
CUDA environment, a checkpoint digest, three identical output runs, and the
same preprocessing and postprocessing. MPS remains a preview path because
PyTorch does not guarantee identical results across releases and platforms.

Model residuals may affect only reviewed painted-region masks:

```text
output = nearest_4x(source) + painted_mask * bounded_model_residual
```

The residual is zero on text, alpha boundaries, state indicators, controls,
hotspot-bearing chrome, and protected silhouettes. Protected flat regions are
quantized back to source palette colors.

## Acceptance gates

Every faithful-HD asset must pass:

- exact source DLL/resource, source-byte, palette, configuration, generator,
  model when applicable, and output hashes;
- exact 4x dimensions and registration;
- source reconstruction after reduction with SSIM at least 0.995, normalized
  mean absolute error at most 1/255, and edge IoU at least 0.98;
- bit-identical protected text, control, alpha, hotspot, and state masks;
- no new connected components, duplicated subjects, or pixels outside an
  authorized silhouette;
- no halo pixels over black, white, source-blue, or checkerboard backgrounds;
- three identical hashes for learned-model runs in the canonical environment;
- human inspection at logical scale, 4x, difference overlay, and four alpha
  backgrounds;
- matching native and browser captures from the same approved output digest.

Animations additionally require exact frame count, order, identity, cadence,
zero static-region shimmer, and atomic family fallback if any frame fails.

## Browser delivery

Do not run super-resolution models in the game client. Generate immutable
assets offline. A future ORPK version should split the original startup pack
from optional, manifest-bound surface packs, then lazy-load those packs by
surface. Initial budgets are no HD bytes on the parity startup path, no runtime
ML allocation, chunks no larger than 8 MiB, and measured texture LRUs near
128 MiB on desktop and 64 MiB on mobile. A 4x image consumes sixteen times the
source RGBA texture memory before compression, so full eager loading is not
acceptable.

KTX2 may be evaluated for large opaque painted images. Lossy block compression
is not permitted for text, controls, alpha boundaries, or strict UI art.

## Milestones

1. P45A is complete: explicit profiles, source-bound manifests, verified-byte
   decoding, deterministic indexed scaling, fallback probes, and Astra browser
   regression review pass. See the [evidence](../../qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-faithful-hd-foundation.md).
2. P45B: add authoritative family classifications and run the complete-family
   SwinIR/HAT/Real-ESRGAN/UltraSharp comparison.
3. P45C: add protected masks, residual bounds, alpha composites, logical reconstruction, component,
   and temporal gates to the generator.
4. P45D: design optional manifest-bound ORPK chunks with bounded decode and texture
   caches.
5. P45E: verify `faithful-hd` in native, Chromium, Firefox, and Safari without using
   its pixels as original-parity evidence.

## Primary sources

- [Scale2x algorithm](https://www.scale2x.it/algorithm)
- [Real-ESRGAN, ICCV Workshops 2021](https://openaccess.thecvf.com/content/ICCV2021W/AIM/html/Wang_Real-ESRGAN_Training_Real-World_Blind_Super-Resolution_With_Pure_Synthetic_Data_ICCVW_2021_paper.html)
- [SwinIR](https://arxiv.org/abs/2108.10257)
- [HAT, CVPR 2023](https://openaccess.thecvf.com/content/CVPR2023/html/Chen_Activating_More_Pixels_in_Image_Super-Resolution_Transformer_CVPR_2023_paper.html)
- [SeeSR, CVPR 2024](https://openaccess.thecvf.com/content/CVPR2024/html/Wu_SeeSR_Towards_Semantics-Aware_Real-World_Image_Super-Resolution_CVPR_2024_paper.html)
- [DiffBIR](https://arxiv.org/abs/2308.15070)
- [IFCSR, CVPR 2026](https://openaccess.thecvf.com/content/CVPR2026/html/Back_IFCSR_Inference-Free_Fidelity-Realism_Control_for_One-Step_Diffusion-based_Real-World_Image_Super-Resolution_CVPR_2026_paper.html)
- [FiDeSR, CVPR 2026](https://openaccess.thecvf.com/content/CVPR2026/html/Kim_FiDeSR_High-Fidelity_and_Detail-Preserving_One-Step_Diffusion_Super-Resolution_CVPR_2026_paper.html)
- [TIGER scene-text restoration, CVPR 2026](https://openaccess.thecvf.com/content/CVPR2026/html/Luo_Restore_Text_First_Enhance_Image_Later_Two-Stage_Scene_Text_Image_CVPR_2026_paper.html)
- [BasicVSR++](https://openaccess.thecvf.com/content/CVPR2022/html/Chan_BasicVSR_Improving_Video_Super-Resolution_With_Enhanced_Propagation_and_Alignment_CVPR_2022_paper.html)
- [PyTorch reproducibility](https://docs.pytorch.org/docs/2.14/notes/randomness.html)
- [ONNX Runtime Web](https://onnxruntime.ai/docs/tutorials/web/)
- [WebGPU specification](https://www.w3.org/TR/webgpu/)
- [KTX 2.0 specification](https://registry.khronos.org/KTX/specs/2.0/ktxspec.v2.html)

Model-code licenses and checkpoint terms must be recorded separately. The
engine license does not grant redistribution rights for original or derivative
Star Wars artwork. Generate local packs from an owned installation unless those
rights are established.
