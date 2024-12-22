use clap::Parser;
use types::{Constraints, ModAndReleases};

use crate::types::ModType;
use std::error::Error;
use std::fs;

mod cli;
mod logging;
pub mod logic;
mod modrinth;
pub mod types;

fn parse_mods(path: &str) -> anyhow::Result<Vec<types::Mod>> {
    let data = fs::read_to_string(path)?;
    let mut mods = Vec::new();
    for line in data.lines() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("https://modrinth.com/mod/") {
            let name = line
                .trim_start_matches("https://modrinth.com/mod/")
                .to_string();
            mods.push(types::Mod {
                name,
                mod_type: ModType::ModrinthModType,
            });
        } else {
            log::warn!("Unsupported mod type for: {}", line);
        }
    }
    Ok(mods)
}

fn main() -> Result<(), Box<dyn Error>> {
    logging::setup();

    let cli = cli::Cli::parse();

    let mods = parse_mods(&cli.src)?;

    let mods_releases: Vec<ModAndReleases> = mods
        .iter()
        .filter_map(|m| match modrinth::check_releases(m) {
            Ok(releases) => Some(ModAndReleases {
                mod_item: m.clone(),
                releases,
            }),
            Err(e) => {
                log::error!("Error checking versions for {}: {}", m.name, e);
                None
            }
        })
        .collect();

    let c = Constraints {
        min_mc_version: cli.min_version,
        exact_mc_version: cli.exact_version,
        loader: cli.loader,
    };

    let best_mc_version = logic::find_best_mc_version(mods_releases, c);
    if let Some(mc_version) = best_mc_version {
        println!("Best MC version: {}", mc_version.str);
    } else {
        println!("No matching MC version found");
    }

    Ok(())
}
