fn order_mc_versions(versions: Vec<String>) -> Vec<String> {
    let mut versions = versions;
    versions.sort_by(|a, b| {
        let a = a.split('.').collect::<Vec<&str>>();
        let b = b.split('.').collect::<Vec<&str>>();
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
    });
    versions
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_mc_versions() {
        let versions = vec![
            "1.12.2".to_string(),
            "1.7.10".to_string(),
            "1.16.5".to_string(),
            "1.8.9".to_string(),
            "1.12.1".to_string(),
        ];
        let expected = vec![
            "1.7.10".to_string(),
            "1.8.9".to_string(),
            "1.12.1".to_string(),
            "1.12.2".to_string(),
            "1.16.5".to_string(),
        ];
        assert_eq!(order_mc_versions(versions), expected);
    }

   
    #[test]
    fn test_order_mc_versions_same_major_minor() {
        let versions = vec![
            "1.12.2".to_string(),
            "1.12.1".to_string(),
            "1.12.0".to_string(),
        ];
        let expected = vec![
            "1.12.0".to_string(),
            "1.12.1".to_string(),
            "1.12.2".to_string(),
        ];
        assert_eq!(order_mc_versions(versions), expected);
    }

    // Uhhh this should never happen, but let's test it anyway
    #[test]
    fn test_order_mc_versions_different_length() {
        let versions = vec![
            "1.12".to_string(),
            "1.12.1".to_string(),
            "1.12.0.1".to_string(),
        ];
        let expected = vec![
            "1.12".to_string(),
            "1.12.0.1".to_string(),
            "1.12.1".to_string(),
        ];
        assert_eq!(order_mc_versions(versions), expected);
    }
}
