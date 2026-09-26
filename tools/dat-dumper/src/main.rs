#![allow(dead_code)]

mod codec;
mod dat_record;
mod registry;
mod types;
mod validate;

use clap::Parser;
use std::path::PathBuf;

type TacticalVoiceBank<'a> = (&'a str, &'a str, &'a str, &'a [(u32, u32)]);

#[derive(Parser)]
#[command(
    name = "dat-dumper",
    about = "Parse Star Wars Rebellion .DAT files to JSON"
)]
struct Cli {
    /// Path to `GData` directory containing .DAT files
    #[arg(short, long)]
    gdata: PathBuf,

    /// Specific .DAT file to parse (e.g. CAPSHPSD.DAT). If omitted, parse all known files.
    #[arg(short, long)]
    file: Option<String>,

    /// Output directory for JSON files. If omitted: stdout for a single file, summary only for all.
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Extract TEXTSTRA.DLL string table to JSON. Outputs to --output dir as textstra.json,
    /// or stdout if --output is not set. Requires TEXTSTRA.DLL in the --gdata directory.
    #[arg(long)]
    extract_strings: bool,

    /// Extract the four main-menu sound effects from COMMON.DLL. Outputs
    /// named WAV files beneath --output, which is required for this mode.
    #[arg(long)]
    extract_menu_sfx: bool,

    /// Extract recovered tactical event sounds from TACTICAL.DLL. Outputs
    /// named WAV files beneath --output, which is required for this mode.
    #[arg(long)]
    extract_tactical_sfx: bool,

    /// Extract the source-proven tactical voice banks from VOICEFXA.DLL and
    /// VOICEFXE.DLL. Outputs faction directories beneath --output.
    #[arg(long)]
    extract_tactical_voice: bool,
}

#[expect(
    clippy::too_many_lines,
    reason = "Keep this existing ordered routine together; splitting its phases is a separate refactor."
)]
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // ── TEXTSTRA.DLL string extraction (separate path) ───────────────────────
    if cli.extract_strings {
        let dll_path = cli.gdata.join("TEXTSTRA.DLL");
        if !dll_path.exists() {
            anyhow::bail!("TEXTSTRA.DLL not found in {}", cli.gdata.display());
        }

        let strings = types::textstra::load_strings(&dll_path)?;
        let json = serde_json::to_string_pretty(&strings)?;

        match &cli.output {
            Some(out_dir) => {
                std::fs::create_dir_all(out_dir)?;
                let out_path = out_dir.join("textstra.json");
                std::fs::write(&out_path, &json)?;
                eprintln!(
                    "OK   TEXTSTRA.DLL -> {} ({} strings)",
                    out_path.display(),
                    strings.len()
                );
            }
            None => {
                println!("{json}");
            }
        }
        return Ok(());
    }

    // ── COMMON.DLL menu SFX extraction (separate path) ───────────────────────
    if cli.extract_menu_sfx {
        let dll_path = cli.gdata.join("COMMON.DLL");
        if !dll_path.exists() {
            anyhow::bail!("COMMON.DLL not found in {}", cli.gdata.display());
        }
        let out_dir = cli
            .output
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("--output is required with --extract-menu-sfx"))?;
        std::fs::create_dir_all(out_dir)?;

        let mapping = [
            (8000, "menu_galaxy_size.wav"),
            (8001, "menu_load_options.wav"),
            (8002, "menu_quit.wav"),
            (8004, "menu_select.wav"),
        ];
        let resource_ids: Vec<u32> = mapping.iter().map(|(id, _)| *id).collect();
        let waves = types::wave_resources::load_waves(&dll_path, &resource_ids)?;
        for (resource_id, filename) in mapping {
            let bytes = waves
                .get(&resource_id)
                .ok_or_else(|| anyhow::anyhow!("missing extracted resource {resource_id}"))?;
            let out_path = out_dir.join(filename);
            std::fs::write(&out_path, bytes)?;
            eprintln!(
                "OK   COMMON.DLL WAVE {} -> {} ({} bytes)",
                resource_id,
                out_path.display(),
                bytes.len()
            );
        }
        return Ok(());
    }

    // ── TACTICAL.DLL event SFX extraction (separate path) ───────────────────
    if cli.extract_tactical_sfx {
        let dll_path = cli.gdata.join("TACTICAL.DLL");
        if !dll_path.exists() {
            anyhow::bail!("TACTICAL.DLL not found in {}", cli.gdata.display());
        }
        let out_dir = cli
            .output
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("--output is required with --extract-tactical-sfx"))?;
        std::fs::create_dir_all(out_dir)?;

        let mapping = [
            (13_033, "tactical_event_0d_0.wav"),
            (13_034, "tactical_event_0d_1.wav"),
            (13_035, "tactical_event_0d_2.wav"),
            (13_036, "tactical_event_0e_0.wav"),
            (13_037, "tactical_event_0e_1.wav"),
            (13_038, "tactical_event_0e_2.wav"),
            (13_039, "tactical_event_0f_0.wav"),
            (13_040, "tactical_event_0f_1.wav"),
            (13_041, "tactical_event_0f_2.wav"),
            (13_042, "tactical_event_10_0.wav"),
            (13_043, "tactical_event_10_1.wav"),
            (13_044, "tactical_event_10_2.wav"),
            (13_045, "tactical_event_11_0.wav"),
            (13_046, "tactical_event_11_1.wav"),
            (13_047, "tactical_event_11_2.wav"),
            (13_048, "tactical_event_12_0.wav"),
            (13_049, "tactical_event_12_1.wav"),
            (13_050, "tactical_event_12_2.wav"),
            (13_051, "tactical_event_13_0.wav"),
            (13_052, "tactical_event_13_1.wav"),
            (13_053, "tactical_event_13_2.wav"),
            (13_054, "tactical_event_14_0.wav"),
        ];
        let resource_ids: Vec<u32> = mapping.iter().map(|(id, _)| *id).collect();
        let waves = types::wave_resources::load_waves(&dll_path, &resource_ids)?;
        for (resource_id, filename) in mapping {
            let bytes = waves
                .get(&resource_id)
                .ok_or_else(|| anyhow::anyhow!("missing extracted resource {resource_id}"))?;
            let out_path = out_dir.join(filename);
            std::fs::write(&out_path, bytes)?;
            eprintln!(
                "OK   TACTICAL.DLL WAVE {} -> {} ({} bytes)",
                resource_id,
                out_path.display(),
                bytes.len()
            );
        }
        return Ok(());
    }

    // ── Complete VOICEFX tactical-bank extraction (separate path) ──────────
    if cli.extract_tactical_voice {
        let out_dir = cli
            .output
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("--output is required with --extract-tactical-voice"))?;
        let banks: [TacticalVoiceBank<'_>; 2] = [
            (
                "VOICEFXA.DLL",
                "alliance",
                "voicefxa",
                &[(14_001, 14_122), (15_133, 15_163)],
            ),
            ("VOICEFXE.DLL", "empire", "voicefxe", &[(15_001, 15_132)]),
        ];
        for (dll_name, faction_dir, suffix, ranges) in banks {
            let dll_path = cli.gdata.join(dll_name);
            if !dll_path.exists() {
                anyhow::bail!("{dll_name} not found in {}", cli.gdata.display());
            }
            let resource_ids = ranges
                .iter()
                .copied()
                .flat_map(|(first, last)| first..=last)
                .collect::<Vec<_>>();
            let waves = types::wave_resources::load_waves(&dll_path, &resource_ids)?;
            let destination = out_dir.join(faction_dir);
            std::fs::create_dir_all(&destination)?;
            for resource_id in resource_ids {
                let bytes = waves
                    .get(&resource_id)
                    .ok_or_else(|| anyhow::anyhow!("missing extracted resource {resource_id}"))?;
                let out_path = destination.join(format!("{resource_id}-{suffix}.wav"));
                std::fs::write(&out_path, bytes)?;
            }
            eprintln!(
                "OK   {dll_name} tactical voice bank -> {} ({} WAVs)",
                destination.display(),
                waves.len()
            );
        }
        return Ok(());
    }

    // ── DAT file parsing ─────────────────────────────────────────────────────
    let registry = registry::build_registry();

    // Build the list of (filename, path) pairs to process.
    let files_to_parse: Vec<(&str, PathBuf)> = if let Some(ref name) = cli.file {
        let upper = name.to_uppercase();
        let key = registry
            .keys()
            .find(|k| k.to_uppercase() == upper)
            .copied()
            .ok_or_else(|| {
                let mut known: Vec<_> = registry.keys().copied().collect();
                known.sort_unstable();
                anyhow::anyhow!("Unknown DAT file: {name}. Known files: {known:?}")
            })?;
        vec![(key, cli.gdata.join(key))]
    } else {
        let mut v: Vec<(&str, PathBuf)> = registry
            .keys()
            .map(|&name| (name, cli.gdata.join(name)))
            .collect();
        v.sort_by_key(|(name, _)| *name);
        v
    };

    let total = files_to_parse.len();
    let mut success = 0usize;
    let mut failed = 0usize;

    for (name, path) in &files_to_parse {
        if !path.exists() {
            eprintln!("SKIP {name}: file not found");
            continue;
        }

        let data = std::fs::read(path)?;
        let parse_fn = registry[name];

        match parse_fn(&data, name) {
            Ok(json) => {
                match &cli.output {
                    Some(out_dir) => {
                        std::fs::create_dir_all(out_dir)?;
                        let json_name = name.replace(".DAT", ".json");
                        let out_path = out_dir.join(&json_name);
                        std::fs::write(&out_path, &json)?;
                        eprintln!("OK   {} -> {}", name, out_path.display());
                    }
                    None => {
                        if total == 1 {
                            println!("{json}");
                        } else {
                            eprintln!("OK   {name}");
                        }
                    }
                }
                success += 1;
            }
            Err(e) => {
                eprintln!("FAIL {name}: {e}");
                failed += 1;
            }
        }
    }

    if total > 1 {
        eprintln!("\n{success} succeeded, {failed} failed out of {total} files");
    }

    Ok(())
}
