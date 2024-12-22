/// Where the mod comes from
#[derive(Debug, Clone)]
pub enum ModType {
    ForgeModType,
    ModrinthModType,
}

/// Represents metadata about a mod
#[derive(Debug, Clone)]
pub struct Mod {
    pub name: String,
    pub mod_type: ModType,
}

/// Represents a Minecraft version, e.g. 1.16.5
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub struct MCVersion {
    pub str: String,
}

impl MCVersion {
    pub fn new(str: &str) -> Self {
        MCVersion {
            str: str.to_string(),
        }
    }
}

/// Represents a Release on the project's pagg
#[derive(Debug, Clone)]
pub struct ModRelease {
    pub mc_versions: Vec<MCVersion>,
    /// release number
    pub release: String,
    pub loaders: Vec<String>,
}

pub struct ModAndReleases {
    pub mod_item: Mod,
    pub releases: Vec<ModRelease>,
}

pub struct Constraints {
    pub min_mc_version: Option<String>,
    pub exact_mc_version: Option<String>,
    pub loader: Option<String>,
}
