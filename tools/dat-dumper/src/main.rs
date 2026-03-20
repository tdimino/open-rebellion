#![allow(dead_code)]

mod codec;
mod dat_record;
mod validate;
mod types;
mod registry;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "dat-dumper", about = "Parse Star Wars Rebellion .DAT files to JSON")]
struct Cli {
    /// Path to GData directory containing .DAT files
    #[arg(short, long)]
    gdata: PathBuf,

    /// Specific .DAT file to parse (e.g. CAPSHPSD.DAT). If omitted, parse all known files.
    #[arg(short, long)]
    file: Option<String>,

    /// Output directory for JSON files. If omitted: stdout for a single file, summary only for all.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
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
                anyhow::anyhow!(
                    "Unknown DAT file: {}. Known files: {:?}",
                    name,
                    known
                )
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
            eprintln!("SKIP {}: file not found", name);
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
                            println!("{}", json);
                        } else {
                            eprintln!("OK   {}", name);
                        }
                    }
                }
                success += 1;
            }
            Err(e) => {
                eprintln!("FAIL {}: {}", name, e);
                failed += 1;
            }
        }
    }

    if total > 1 {
        eprintln!(
            "\n{} succeeded, {} failed out of {} files",
            success,
            failed,
            total
        );
    }

    Ok(())
}
