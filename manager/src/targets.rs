use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;
use std::sync::LazyLock;
use strum::{EnumIter, IntoEnumIterator};
use crate::collection::install::handler::bepinex::BepInExHandler;
use crate::collection::install::handler::dir_map::{DirectoryMap, MappedInstaller};
use crate::collection::install::handler::PluginHandler;

const TARGET_JSON: &str = include_str!("../targets.json");

static TARGETS: LazyLock<Vec<TargetData<'static>>> =
    LazyLock::new(|| serde_json::from_str(TARGET_JSON).unwrap());

pub type Target = &'static TargetData<'static>;

/// Iterate through all supported targets.
pub fn all() -> impl Iterator<Item = Target> {
    TARGETS.iter()
}

/// Find game that matches the provided slug.
pub fn from_slug(slug: &str) -> Option<Target> {
    all().find(|target| target.slug == slug)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonTarget<'a> {
    name: &'a str,
    slug: &'a str,
    #[serde(borrow)]
    mod_loader: ModLoader<'a>,
    #[serde(borrow)]
    platforms: Platforms<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", from = "JsonTarget")]
pub struct TargetData<'a> {
    pub name: &'a str,
    pub slug: Cow<'a, str>,
    pub mod_loader: ModLoader<'a>,
    pub platforms: Platforms<'a>,
}

impl<'a> From<JsonTarget<'a>> for TargetData<'a> {
    fn from(target: JsonTarget<'a>) -> Self {
        let JsonTarget {
            name,
            slug,
            mod_loader,
            platforms,
        } = target;

        let slug = Cow::Borrowed(slug);

        Self {
            name,
            slug,
            mod_loader,
            platforms,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize, EnumIter)]
#[serde(rename_all = "camelCase")]
pub enum Platform {
    #[default]
    Steam,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SteamPlatform<'a> {
    pub id: u32,
    #[serde(default)]
    pub dir_name: Option<&'a str>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Platforms<'a> {
    #[serde(borrow)]
    pub steam: Option<SteamPlatform<'a>>,
}

impl Platforms<'_> {
    /// Returns if the target's platforms support the provided
    /// platform.
    pub fn has(&self, platform: Platform) -> bool {
        match platform {
            Platform::Steam => self.steam.is_some(),
        }
    }

    /// Iterate through all supported platforms.
    pub fn iter(&self) -> impl Iterator<Item = Platform> {
        Platform::iter().filter(|x| self.has(*x))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModLoader<'a> {
    #[serde(flatten)]
    pub kind: ModLoaderKind,
    #[serde(default)]
    pub package_override: Option<&'a str>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum ModLoaderKind {
    BepInEx,
}

impl fmt::Display for ModLoader<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str_type = match self.kind {
            ModLoaderKind::BepInEx => "BepInEx",
        };

        write!(f, "{}", str_type)
    }
}

impl ModLoader<'_> {
    fn is_loader_plugin(&self, name: &str) -> bool {
        if let Some(plugin_name) = self.package_override {
            plugin_name == name
        }
        else {
            match &self.kind {
                ModLoaderKind::BepInEx => name.starts_with("BepInEx-BepInExPack")
            }
        }
    }
}

impl ModLoader<'static> {
    pub fn installer_for_plugin(&'static self, plugin_name: &str) -> Box<dyn PluginHandler> {
        match (self.is_loader_plugin(plugin_name), &self.kind) {
            (true, ModLoaderKind::BepInEx) => Box::new(BepInExHandler),
            (false, ModLoaderKind::BepInEx) => {
                const MAPPED_DIRS: &[DirectoryMap] = &[
                    DirectoryMap::flattened("plugins", "BepInEx/plugins"),
                    DirectoryMap::flattened("core", "BepInEx/core"),
                    DirectoryMap::none("config", "BepInEx/config").files_mutable(),
                ];

                Box::new(MappedInstaller::new(MAPPED_DIRS, 0))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_valheim_data() {
        let valheim_data = from_slug("valheim").unwrap();

        assert_eq!(valheim_data.name, "Valheim");
        assert_eq!(valheim_data.slug, "valheim");
        assert_eq!(valheim_data.mod_loader.kind, ModLoaderKind::BepInEx);

        assert!(valheim_data.platforms.has(Platform::Steam));
    }
}
