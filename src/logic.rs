use crate::types::{Constraints, MCVersion, Mod, ModAndReleases, ModRelease};
use itertools::Itertools;

/// Flattens a list of mods and their releases into a list of (Mod, ModRelease)
fn get_flat_releases(mods: &Vec<ModAndReleases>) -> Vec<(Mod, ModRelease)> {
    mods.iter()
        .flat_map(|mod_and_versions| {
            mod_and_versions
                .releases
                .iter()
                .map(move |version| (mod_and_versions.mod_item.clone(), version.clone()))
        })
        .collect()
}

/// Extracts all unique MC versions from a list of releases
fn extract_sorted_unique_mc_versions(releases: &Vec<(Mod, ModRelease)>) -> Vec<MCVersion> {
    releases
        .iter()
        .flat_map(|(_, release)| release.mc_versions.iter().cloned())
        .unique()
        .sorted()
        .collect()
}

/// Find the highest MC version that satisfies the constraints for all mods
pub fn find_best_mc_version(mods: Vec<ModAndReleases>, c: Constraints) -> Option<MCVersion> {
    let matching_releases: Vec<_> = get_flat_releases(&mods)
        .into_iter()
        .filter(|(_, version)| version.match_constraints(&c))
        .collect();

    let candidate_mc_versions = extract_sorted_unique_mc_versions(&matching_releases);
    for mc_version in candidate_mc_versions.into_iter().rev() {
        let matching_mods = matching_releases
            .iter()
            .filter(|(_, release)| release.mc_versions.contains(&mc_version))
            .map(|(m, _)| m)
            .unique()
            .count();
        if matching_mods == mods.len() {
            return Some(mc_version.clone());
        }
    }
    return None;
}

impl ModRelease {
    /// Check if this release matches the given constraints
    fn match_constraints(&self, c: &Constraints) -> bool {
        if let Some(loader) = &c.loader {
            if !self.loaders.contains(loader) {
                return false;
            }
        }

        if let Some(exact_version) = &c.exact_mc_version {
            if self.release != *exact_version {
                return false;
            }
        }

        if let Some(min_version) = &c.min_mc_version {
            let min_version = MCVersion::new(min_version);
            let version = MCVersion::new(&self.release);
            if version < min_version {
                return false;
            }
        }

        true
    }
}

impl Ord for MCVersion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.str.split('.').collect::<Vec<&str>>();
        let b = other.str.split('.').collect::<Vec<&str>>();
        for i in 0..a.len() {
            if i >= b.len() {
                return std::cmp::Ordering::Greater;
            }

            // Compare this part of the version
            let a = a[i].parse::<u32>().unwrap();
            let b = b[i].parse::<u32>().unwrap();
            if a < b {
                return std::cmp::Ordering::Less;
            } else if a > b {
                return std::cmp::Ordering::Greater;
            }
        }
        if a.len() < b.len() {
            return std::cmp::Ordering::Less;
        }
        std::cmp::Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    #[test]
    fn test_find_best_mc_version() {
        let mod1 = Mod {
            name: "Waystones".to_string(),
            mod_type: ModType::ModrinthModType,
        };
        let mod2 = Mod {
            name: "IceAndFire".to_string(),
            mod_type: ModType::ModrinthModType,
        };

        let release1 = ModRelease {
            release: "".to_string(),
            mc_versions: vec![MCVersion::new("1.16.4")],
            loaders: vec!["forge".to_string()],
        };

        let release2 = ModRelease {
            release: "".to_string(),
            mc_versions: vec![MCVersion::new("1.16.5")],
            loaders: vec!["forge".to_string()],
        };

        let release3 = ModRelease {
            release: "".to_string(),
            mc_versions: vec![MCVersion::new("1.17.1")],
            loaders: vec!["fabric".to_string()],
        };

        let release4 = ModRelease {
            release: "".to_string(),
            mc_versions: vec![MCVersion::new("1.12.2")],
            loaders: vec!["fabric".to_string()],
        };

        let release5 = ModRelease {
            release: "".to_string(),
            mc_versions: vec![MCVersion::new("1.16.5")],
            loaders: vec!["forge".to_string()],
        };

        let mods: Vec<ModAndReleases> = vec![
            ModAndReleases {
                mod_item: mod1,
                releases: vec![release1, release2],
            },
            ModAndReleases {
                mod_item: mod2,
                releases: vec![release3, release4, release5],
            },
        ];

        let constraints = Constraints {
            loader: Some("forge".to_string()),
            exact_mc_version: None,
            min_mc_version: None,
        };

        let best_version = find_best_mc_version(mods, constraints);
        assert_eq!(best_version, Some(MCVersion::new("1.16.5")));
    }

    #[test]
    fn test_mc_version_ordering() {
        let v1 = MCVersion::new("1.16.5");
        let v2 = MCVersion::new("1.16.4");
        let v3 = MCVersion::new("1.17.2");

        assert!(v1 > v2);
        assert!(v2 < v3);
        assert!(v1 < v3);
    }

    #[test]
    fn test_mc_version_equality() {
        let v1 = MCVersion::new("1.16.5");
        let v2 = MCVersion::new("1.16.5");

        assert_eq!(v1, v2);
    }
}
