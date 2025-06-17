use std::str::FromStr;

use eyre::OptionExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, Clone, Serialize, Deserialize)]
#[serde(into = "String", try_from = "String")]
pub struct VersionIdent {
    str: String,
    name_index: u16,
    version_index: u16,
}

impl VersionIdent {
    pub fn new(namespace: &str, name: &str, version: &str) -> Self {
        let str = format!("{}-{}-{}", namespace, name, version);

        let name_index = namespace.len() as u16 + 1;
        let version_index = name_index + name.len() as u16 + 1;

        Self {
            str,
            name_index,
            version_index,
        }
    }

    pub fn namespace(&self) -> &str {
        &self.str[..self.name_index as usize - 1]
    }

    pub fn name(&self) -> &str {
        &self.str[self.name_index as usize..self.version_index as usize - 1]
    }

    pub fn version(&self) -> &str {
        &self.str[self.version_index as usize..]
    }

    pub fn as_str(&self) -> &str {
        &self.str
    }
}

impl PartialEq for VersionIdent {
    fn eq(&self, other: &Self) -> bool {
        self.str == other.str
    }
}

impl TryFrom<String> for VersionIdent {
    type Error = eyre::Report;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut indices = value.match_indices('-').map(|(x, _)| x);

        let version_index = indices
            .next_back()
            .ok_or_eyre("failed to fetch version index")? as u16
            + 1;
        let name_index = indices
            .next_back()
            .ok_or_eyre("failed to fetch name index")? as u16
            + 1;

        Ok(Self {
            str: value,
            name_index,
            version_index,
        })
    }
}

impl FromStr for VersionIdent {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.to_string().try_into()
    }
}

impl From<VersionIdent> for String {
    fn from(value: VersionIdent) -> Self {
        value.str
    }
}

#[derive(Debug, Eq, Clone, Serialize, Deserialize)]
#[serde(into = "String", try_from = "String")]
pub struct PackageIdent {
    str: String,
    name_index: u16,
}

impl PackageIdent {
    pub fn new(namespace: &str, name: &str) -> Self {
        let str = format!("{}-{}", namespace, name);

        let name_index = namespace.len() as u16 + 1;

        Self { str, name_index }
    }

    pub fn namespace(&self) -> &str {
        &self.str[..self.name_index as usize - 1]
    }

    pub fn name(&self) -> &str {
        &self.str[self.name_index as usize..]
    }

    pub fn as_str(&self) -> &str {
        &self.str
    }
}

impl PartialEq for PackageIdent {
    fn eq(&self, other: &Self) -> bool {
        self.str == other.str
    }
}

impl TryFrom<String> for PackageIdent {
    type Error = eyre::Report;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut indices = value.match_indices('-').map(|(x, _)| x);

        let name_index = indices
            .next_back()
            .ok_or_eyre("failed to fetch name index")? as u16
            + 1;

        Ok(Self {
            str: value,
            name_index,
        })
    }
}

impl FromStr for PackageIdent {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.to_string().try_into()
    }
}

impl From<PackageIdent> for String {
    fn from(value: PackageIdent) -> Self {
        value.str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_version_ident() {
        let valid_versions = vec![
            "denikson-BepInExPack_Valheim-5.4.2202",
            "ValheimModding-Jotunn-2.25.0",
        ];

        for version in valid_versions {
            VersionIdent::try_from(version.to_string()).unwrap();
        }
    }

    #[test]
    fn test_parse_valid_package_ident() {
        let valid_versions = vec!["denikson-BepInExPack_Valheim", "ValheimModding-Jotunn"];

        for version in valid_versions {
            PackageIdent::try_from(version.to_string()).unwrap();
        }
    }
}
