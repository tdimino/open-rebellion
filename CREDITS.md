---
title: "Credits & Acknowledgments"
description: "Attribution for open-source projects, AI services, and community efforts used in Open Rebellion"
category: "reference"
created: 2026-03-14
updated: 2026-03-16
tags: [credits, attribution, tools]
---

# Credits & Acknowledgments

Open Rebellion is built on the work of many open-source projects, AI services, and community efforts.

## Game Heritage

- **Star Wars: Rebellion** (1998) — Coolhand Interactive / LucasArts. The original game this project reimplements.
- **swrebellion.net** — The fan community that kept the game alive for 25+ years with patches, mods, and documentation.
- **MetasharpNet / StarWarsRebellionEditor.NET** — Metasharp's open-source C# editor ([GitHub](https://github.com/MetasharpNet/StarWarsRebellionEditor.NET)). Source of DAT field semantics, asset organization knowledge, and DLL resource extraction (2,441+ BMPs + 3,223 data files from 11 game DLLs; see agent_docs/dll-resource-catalog.md).
- **rebellion2** — C# Unity port reference. Source of mission probability formulas (Mission.cs).

## AI Image Generation & Upscaling

- **Gemini 3 Pro Image** (nano-banana-pro) — Google DeepMind. Reference-based asset generation for encyclopedia images, concept art, and style-consistent new graphics.
- **Vertex AI Imagen 4.0** — Google Cloud. Non-generative super-resolution upscaler (`imagen-4.0-upscale-preview`). Primary upscaler for portraits, fighters, sprites, and planets where fidelity is critical.
- **waifu2x-ncnn-vulkan** — nihui ([GitHub](https://github.com/nihui/waifu2x-ncnn-vulkan)). Batch image upscaler for original 1998 BMPs. MIT license.
- **PBRify_UpscalerV4** — Kim2091 ([OpenModelDB](https://openmodeldb.info/models/4x-PBRify-UpscalerV4)). ESRGAN model purpose-built for 2000s game texture upscaling.
- **UltraSharpV2** — Kim2091 ([OpenModelDB](https://openmodeldb.info/models/4x-UltraSharpV2)). General multi-domain ESRGAN model for comparison.
- **Upscayl** — Free GUI upscaler with custom model import ([upscayl.org](https://upscayl.org)).
- **chaiNNer** — Node-based image processing with ESRGAN support ([chainner.app](https://chainner.app)).

## 3D Model Generation

- **Hunyuan3D Pro** — Tencent, via fal.ai. Primary 3D model generator for tactical combat assets.
- **WaveSpeedAI** — Hunyuan3D v3.1 Rapid endpoint ([wavespeed.ai](https://wavespeed.ai)). Budget batch generation at $0.0225/model.
- **Meshy** — Text-to-3D with Low Poly Mode ([meshy.ai](https://meshy.ai)). Comparison/fallback provider.
- **Trellis 2** — Microsoft, via 3D AI Studio ([3daistudio.com](https://www.3daistudio.com)). Rapid iteration with polygon control.

## 3D Pipeline Tools

- **gltf-transform** — Don McCurdy ([GitHub](https://github.com/donmccurdy/glTF-Transform)). GLB optimization, simplification, and DRACO compression.
- **Blender** — Blender Foundation ([blender.org](https://www.blender.org)). Sprite sheet rendering from GLB models.
- **BlenderSpriteGenerator** — RubielGames ([GitHub](https://github.com/RubielGames/BlenderSpriteGenerator)). MIT-licensed multi-angle sprite renderer addon.

## Audio Generation & Processing

- **Qwen3-TTS** — Alibaba Cloud ([GitHub](https://github.com/QwenLM/Qwen3-TTS)). Voice cloning from 3-second reference audio. Apache 2.0.
- **Voicebox** — jamiepine ([GitHub](https://github.com/jamiepine/voicebox)). MLX-accelerated voice synthesis desktop app wrapping Qwen3-TTS. MIT.
- **F5-TTS** — SWivid ([GitHub](https://github.com/SWivid/F5-TTS)). Flow matching voice cloning for A/B comparison. MIT.
- **LavaSR v2** — Yatharth Sharma ([GitHub](https://github.com/ysharma3501/LavaSR)). Audio super-resolution, 50x realtime on CPU. Apache 2.0.
- **ElevenLabs** — Sound effects API v2 ([elevenlabs.io](https://elevenlabs.io)). Text-to-SFX generation.
- **ACE-Step 1.5** — ACE Studio + StepFun. Local music generation, <4GB VRAM.
- **AudioLDM2** — CVSSP ([HuggingFace](https://huggingface.co/cvssp/audioldm2)). Local ambient SFX generation fallback.

## Game Engine

- **macroquad** — not-fl3 ([GitHub](https://github.com/not-fl3/macroquad)). Rust game framework for native + WASM.
- **egui-macroquad** — optozorax ([GitHub](https://github.com/optozorax/egui-macroquad)). egui integration for macroquad.
- **quad-snd** — not-fl3. Cross-platform audio (CoreAudio/ALSA/WebAudio).
- **slotmap** — Orson Peters. Arena-based entity storage.
- **pelite** — Caspar Krieger ([GitHub](https://github.com/CasualX/pelite)). PE file parser for TEXTSTRA.DLL string extraction.

## Reverse Engineering

- **Ghidra** — NSA ([GitHub](https://github.com/NationalSecurityAgency/ghidra)). Decompilation of REBEXE.EXE (22,741 functions).
- **GhidraMCP** — LaurieWired. REST API bridge for Ghidra.

## Development Tools

- **Claude Code** — Anthropic. AI-assisted development throughout the project.
- **nano-banana-pro** — Gemini 3 Pro Image integration skill.
- **gemini-claude-resonance** — Cross-model visual dialogue for style analysis.

---

Star Wars is a registered trademark of Lucasfilm Ltd. This is a fan project, not affiliated with or endorsed by Lucasfilm, Disney, or any rights holder.
