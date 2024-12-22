#[derive(Debug)]
pub enum ModType {
    ForgeModType,
    ModrinthModType,
}

#[derive(Debug)]
pub struct Mod {
    pub name: String,
    pub mod_type: ModType,
}

#[derive(Debug)]
pub struct ModVersion {
    pub mc_versions: Vec<String>,
    pub mod_version: String,
    pub loaders: Vec<String>,
}
