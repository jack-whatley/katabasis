use std::fmt::Display;

use eyre::ensure;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionIdent {
    namespace: String,
    name: String,
    version: Option<String>,
}

impl VersionIdent {
    pub fn new(namespace: &str, name: &str, version: &str) -> Self {
        Self {
            namespace: namespace.to_string(),
            name: name.to_string(),
            version: Some(version.to_string()),
        }
    }

    pub fn split(&self) -> (&str, &str, Option<&str>) {
        let namespace = &self.namespace;
        let name = &self.name;
        let version = self.version.as_deref();

        (namespace, name, version)
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }
}

impl TryFrom<String> for VersionIdent {
    type Error = eyre::Report;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts = value.split('-').collect::<Vec<&str>>();

        ensure!(
            parts.len() >= 2 && parts.len() <= 3,
            "Version identifier must have between 2 and 3 parts"
        );

        let version = parts.get(2).and_then(|x| {
            if x.is_empty() {
                None
            } else {
                Some(x.to_string())
            }
        });

        Ok(Self {
            namespace: parts[0].to_string(),
            name: parts[1].to_string(),
            version,
        })
    }
}

impl Display for VersionIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.namespace, self.name)?;

        if let Some(version) = &self.version {
            write!(f, "-{}", version)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid() {
        let valid_versions = vec![
            "denikson-BepInExPack_Valheim-5.4.2202",
            "denikson-BepInExPack_Valheim",
            "ValheimModding-Jotunn-2.25.0",
            "ValheimModding-Jotunn",
        ];

        for version in valid_versions {
            VersionIdent::try_from(version.to_string()).unwrap();
        }
    }

    #[test]
    fn test_parse_valid_specific() {
        let valid_version = "ValheimModding-Jotunn-2.25.0";
        let version_ident = VersionIdent::try_from(valid_version.to_string()).unwrap();

        assert_eq!(version_ident.namespace, "ValheimModding");
        assert_eq!(version_ident.name, "Jotunn");
        assert_eq!(version_ident.version, Some("2.25.0".to_string()));
    }

    #[test]
    fn test_parse_valid_specific_without_version() {
        let valid_version = "ValheimModding-Jotunn";
        let version_ident = VersionIdent::try_from(valid_version.to_string()).unwrap();

        assert_eq!(version_ident.namespace, "ValheimModding");
        assert_eq!(version_ident.name, "Jotunn");
        assert_eq!(version_ident.version, None);
    }

    #[test]
    fn test_parse_valid_with_extra_chars() {
        let valid_version = "ValheimModding-Jotunn-";
        let version_ident = VersionIdent::try_from(valid_version.to_string()).unwrap();

        assert_eq!(version_ident.namespace, "ValheimModding");
        assert_eq!(version_ident.name, "Jotunn");
        assert_eq!(version_ident.version, None);
    }
}
