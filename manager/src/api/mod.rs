pub mod thunderstore {
    use serde::{Deserialize, Serialize};
    use url::Url;

    // This function needs a lot of improvement to make it more robust (including more unit tests!)

    /// Extracts from a thunderstore URL the mod namespace and mod name.
    pub fn extract_thunderstore_url(url: &str) -> Option<(String, String)> {
        let p_url = Url::parse(url).ok()?;

        if p_url.host_str() != Some("thunderstore.io") { return None; }

        let path_segments = p_url.path_segments().map(|c| c.collect::<Vec<_>>())?;
        let path_len = path_segments.len();

        Some((path_segments[path_len - 3].to_string(), path_segments[path_len - 2].to_string()))
    }

    /// Serializable representation of the complete Thunderstore package model.
    #[derive(Serialize, Deserialize)]
    pub struct Package {
        pub namespace: String,
        pub name: String,
        pub full_name: String,
        pub owner: String,
        pub package_url: String,
        pub date_created: String,
        pub date_updated: String,
        pub rating_score: i64,
        pub is_pinned: bool,
        pub is_deprecated: bool,
        pub total_downloads: i64,
        pub latest: PackageVersion
    }

    /// Serializable representation of the Thunderstore package model for a specific package version.
    #[derive(Serialize, Deserialize)]
    pub struct PackageVersion {
        pub namespace: String,
        pub name: String,
        pub version_number: String,
        pub full_name: String,
        pub description: String,
        pub dependencies: Vec<String>,
        pub download_url: String,
        pub downloads: usize,
        pub date_created: String,
        pub website_url: String,
        pub is_active: bool
    }

    #[cfg(test)]
    mod tests {
        use super::extract_thunderstore_url;

        #[test]
        fn test_extract_url() {
            let th_url_one = "https://thunderstore.io/c/lethal-company/p/Evaisa/LethalThings/";
            let th_url_two = "https://thunderstore.io/c/lethal-company/p/notnotnotswipez/MoreCompany/";

            let expected_one = Some(("Evaisa".to_string(), "LethalThings".to_string()));
            let expected_two = Some(("notnotnotswipez".to_string(), "MoreCompany".to_string()));

            assert_eq!(extract_thunderstore_url(th_url_one), expected_one);
            assert_eq!(extract_thunderstore_url(th_url_two), expected_two);
        }
    }
}