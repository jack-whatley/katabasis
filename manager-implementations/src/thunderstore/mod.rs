use std::path::{Path, PathBuf};
use async_trait::async_trait;
use log::{info, warn};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use manager_core::data::{Collection, Plugin};
use manager_core::data::support::PluginSource;
use manager_core::error;
use uuid::Uuid;
use manager_core::state::KatabasisApp;
use manager_core::utils::{fs, net};
use crate::PluginHandler;

pub struct ThunderstorePluginHandler;

#[async_trait]
impl PluginHandler for ThunderstorePluginHandler {
    async fn initialise_plugin(
        &self,
        state: &KatabasisApp,
        url: &str,
    ) -> error::KatabasisResult<Plugin> {
        let id = Uuid::new_v4();
        let thunderstore_url = get_thunderstore_url(url)?;
        let package = fetch_package(state, &thunderstore_url).await?;

        let plugin = Plugin {
            id: format!("{}", id.hyphenated()),
            name: package.name,
            source: PluginSource::Thunderstore,
            api_url: thunderstore_url,
            version: package.latest.version_number,
            is_enabled: true,
            icon_url: Some(package.latest.icon),
        };

        Ok(plugin)
    }

    async fn download_latest(
        &self,
        state: &KatabasisApp,
        collection: &Collection,
        plugin: &Plugin
    ) -> error::KatabasisResult<()> {
        let package = fetch_package(state, &plugin.api_url).await?;
        let mut file_stream = net::fetch_stream(
            &package.latest.download_url,
            &state.net_semaphore,
            &state.http_client,
            state.settings.retry_limit.get()
        ).await?;

        let target_path = state.directories
            .download_dir()
            .join(format!("{}.zip", plugin.name));

        fs::write_stream_to_file(
            &target_path,
            &mut file_stream
        ).await?;

        let unzip_path = target_path.with_extension("");

        let _io_permit = state.io_semaphore.0.acquire().await;

        fs::unzip_file_to_dir(
            &target_path,
            &unzip_path,
        ).await?;

        install_mod_files(
            &unzip_path,
            state.directories.collection_dir(&collection.id)
        ).await?;

        Ok(())
    }

    async fn has_update(
        &self,
        state: &KatabasisApp,
        plugin: &Plugin,
    ) -> error::KatabasisResult<bool> {
        let package = fetch_package(state, &plugin.api_url).await?;

        Ok(package.latest.version_number != plugin.version)
    }
}

fn get_thunderstore_url(url: &str) -> error::KatabasisResult<String> {
    static RE: Lazy<Regex> = Lazy::new(
        || {
            Regex::new(r"https://([A-Za-z0-9.]{4})?thunderstore\.io/c/([A-Za-z0-9-])+/p/(?<namespace>[A-Za-z0-9]+)/(?<name>[A-Za-z0-9]+)/")
                .unwrap()
        }
    );

    if let Some(captures) = RE.captures(&url) {
        Ok(
            format!(
                "https://thunderstore.io/api/experimental/package/{}/{}",
                &captures["namespace"],
                &captures["name"]
            )
        )
    }
    else {
        Err(
            error::KatabasisErrorKind::InvalidPluginUrl(
                format!("Failed to parse Thunderstore URL: {}", url)
            ).into()
        )
    }
}

/// Fetch Thunderstore package details.
async fn fetch_package(state: &KatabasisApp, url: &str) -> error::KatabasisResult<Package> {
    Ok(
        net::fetch_json(
            url,
            &state.net_semaphore,
            &state.http_client,
            state.settings.retry_limit.get()
        ).await?
    )
}

const IGNORE_FILES: &[&str] = &["icon.png", "LICENSE.md", "manifest.json", "README.md", "CHANGELOG.md"];

async fn install_mod_files(
    plugin_dir: impl AsRef<Path>,
    collection_dir: impl AsRef<Path>,
) -> error::KatabasisResult<()> {
    let plugin_dir = plugin_dir.as_ref();
    let collection_dir = collection_dir.as_ref();

    let paths = fs::iterate_directory(plugin_dir, true).await?;
    let mut move_targets: Vec<PathBuf> = vec![];
    let mut config_dir: Option<PathBuf> = None;

    for path in paths {
        if IGNORE_FILES.iter().any(|x| path.ends_with(x)) {
            continue;
        }

        if path.ends_with("config") {
            config_dir = Some(path);
            continue;
        }

        if let Some(ext) = path.extension() {
            if ext == "dll" {
                move_targets.push(path);
            }
        }
    }

    if let Some(config_path) = config_dir {
        fs::copy_contents_to(
            config_path,
            collection_dir.join("BepInEx").join("config")
        ).await?;
    }

    let target_install_dir = collection_dir
        .join("BepInEx")
        .join("plugins");

    for path in move_targets {
        let file_name = path.file_name().unwrap();

        match tokio::fs::copy(path.clone(), &target_install_dir.join(file_name)).await {
            Ok(_) => {},
            Err(e) => {
                warn!("Failed to copy file, plugin may not be installed correctly:\n{:#?}", e);
            }
        }
    }

    fs::remove_dir_all(plugin_dir).await?;

    Ok(())
}

/// Serializable representation of the complete Thunderstore package model.
#[derive(Serialize, Deserialize)]
struct Package {
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
struct PackageVersion {
    pub namespace: String,
    pub name: String,
    pub version_number: String,
    pub full_name: String,
    pub description: String,
    pub icon: String,
    pub dependencies: Vec<String>,
    pub download_url: String,
    pub downloads: usize,
    pub date_created: String,
    pub website_url: String,
    pub is_active: bool
}

#[cfg(test)]
mod tests {
    use super::get_thunderstore_url;

    #[test]
    fn test_parse_correct_thunderstore_url() {
        let parsed_url = get_thunderstore_url("https://thunderstore.io/c/lethal-company/p/Evaisa/LethalThings/")
            .unwrap();

        assert_eq!(
            parsed_url,
            "https://thunderstore.io/api/experimental/package/Evaisa/LethalThings".to_owned()
        )
    }

    #[test]
    fn test_parse_new_url() {
        let parsed_url = get_thunderstore_url("https://new.thunderstore.io/c/lethal-company/p/RugbugRedfern/Skinwalkers/")
            .unwrap();

        assert_eq!(
            parsed_url,
            "https://thunderstore.io/api/experimental/package/RugbugRedfern/Skinwalkers".to_owned()
        )
    }
}
