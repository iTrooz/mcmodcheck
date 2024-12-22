use crate::types::ModType;
use std::error::Error;
use std::fs;

mod logging;
mod modrinth;
pub mod types;
pub mod logic;

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

    let mods = parse_mods("mods.txt")?;
    for mod_item in mods {
        match modrinth::check_versions(&mod_item) {
            Ok(versions) => log::info!("Versions for mod '{}': {:?}", mod_item.name, versions),
            Err(e) => log::error!("Error checking versions for {}: {}", mod_item.name, e),
        }
    }
    Ok(())
}
