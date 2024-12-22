use crate::types::{MCVersion, Mod, ModRelease};

fn parse_version(value: serde_json::Value) -> anyhow::Result<ModRelease> {
    let mc_versions: Vec<String> = serde_json::from_value(value["game_versions"].clone())?;
    Ok(ModRelease {
        mc_versions: mc_versions.into_iter().map(|s| MCVersion::new(&s)).collect(),
        release: serde_json::from_value(value["name"].clone())?,
        loaders: serde_json::from_value(value["loaders"].clone())?,
    })
}

pub fn check_versions(m: &Mod) -> anyhow::Result<Vec<ModRelease>> {
    let url = format!("https://api.modrinth.com/v2/project/{}/version", m.name);
    let body = reqwest::blocking::get(&url)?.text()?;
    let versions: Vec<serde_json::Value> = serde_json::from_str(&body)?;

    let mut parsed_versions = vec![];
    log::debug!("Parsing {} versions for mod '{}'..", versions.len(), m.name);
    for version in versions {
        match parse_version(version.clone()) {
            Ok(v) => parsed_versions.push(v),
            Err(e) => log::warn!("Error parsing version: {}. Object: {}", e, version),
        }
    }

    Ok(parsed_versions)
}
