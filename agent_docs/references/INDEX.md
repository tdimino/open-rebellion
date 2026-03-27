# Reference Image Catalogs

5 catalog files listing all 1,097 vision-LLM-named reference images by upscaling category. Load the relevant catalog when selecting Gemini reference images for a specific DLL asset type.

| Catalog | Files | DLL Assets | Load When |
|---------|-------|-----------|-----------|
| [catalog-portraits.md](catalog-portraits.md) | 330 | GOKRES portraits (2000-2999), mini-icons (19000-19999) | Upscaling character images |
| [catalog-ships.md](catalog-ships.md) | 271 | GOKRES ship views (1-1999), TACTICAL sprites (2000-2999) | Upscaling ship/vehicle images |
| [catalog-weapons.md](catalog-weapons.md) | 198 | TACTICAL weapon sprites, COMMON UI tech | Upscaling weapons/tech diagrams |
| [catalog-planets.md](catalog-planets.md) | 226 | STRATEGY planet panels, system backgrounds | Upscaling planet/location images |
| [catalog-scenes.md](catalog-scenes.md) | 217 | STRATEGY event screens (6000-6999), briefings | Upscaling narrative scene art |

## Usage

1. Identify which DLL asset category you're upscaling
2. Load the matching catalog file
3. Search for a filename matching the subject (e.g., "ackbar" for an Ackbar portrait)
4. Feed that reference image alongside the source BMP to Gemini's multi-image input
