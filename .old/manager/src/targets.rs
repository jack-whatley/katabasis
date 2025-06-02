use std::borrow::Cow;
use std::sync::LazyLock;
use serde::{Deserialize, Serialize};

const TARGETS: &str = include_str!("../targets.json");

pub type Target = TargetData<'static>;

static ALL_TARGETS: LazyLock<Vec<TargetData<'static>>> =
    LazyLock::new(|| serde_json::from_str(TARGETS).unwrap());

pub fn from_slug(slug: &str) -> Option<Target> {
    ALL_TARGETS.iter().find(|target| target.slug == slug)
}

#[derive(Deserialize, Debug)]
struct JsonTarget<'a> {
    name: &'a str,
    slug: &'a str,
    #[serde(borrow, rename = "loader")]
    mod_loader: PluginLoader<'a>,
    platforms: Platforms,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PluginLoader<'a> {
    #[serde(flatten)]
    pub kind: PluginLoaderKind,
    #[serde(default)]
    pub package_override: Option<&'a str>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "name")]
pub enum PluginLoaderKind {
    BepInEx,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Platforms {
    pub steam: Option<Steam>,
}

impl Platforms {
    pub fn has(&self, platform: Platform) -> bool {
        match platform {
            Platform::Steam => self.steam.is_some(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum Platform {
    #[default]
    Steam,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Steam {
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", from = "JsonTarget")]
pub struct TargetData<'a> {
    pub name: &'a str,
    pub slug: Cow<'a, str>,
    pub mod_loader: PluginLoader<'a>,
    pub platforms: Platforms,
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

impl<'a> PartialEq for TargetData<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.slug == other.slug
    }
}
