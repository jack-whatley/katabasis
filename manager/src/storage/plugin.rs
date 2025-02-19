use std::ffi::OsStr;
use std::fs::File;
use std::path::PathBuf;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use walkdir::WalkDir;
use crate::api::thunderstore;
use crate::collection::Collection;
use crate::storage::dir::Directories;
use crate::storage::KbApp;
use crate::utils::download;

/// Module containing logic for handling plugins that make up collections

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum SupportedPluginSources {
    Thunderstore
}

impl From<String> for SupportedPluginSources {
    fn from(s: String) -> Self {
        match s.as_str() {
            "thunderstore" => SupportedPluginSources::Thunderstore,
            "th" => SupportedPluginSources::Thunderstore,
            _ => SupportedPluginSources::Thunderstore
        }
    }
}

pub struct ThunderstoreHandler;

impl SupportedPluginSources {
    pub fn get_handler(&self) -> Box<impl SourceHandler + Send + Sync> {
        match self {
            SupportedPluginSources::Thunderstore => Box::from(ThunderstoreHandler)
        }
    }
}

/// Trait used for handling different mod source APIs
#[async_trait]
pub trait SourceHandler {
    async fn parse_share_url(&self, url: &str) -> crate::Result<Plugin>;

    async fn download_plugin(&self, collection_id: &str, plugin: &Plugin) -> crate::Result<()>;

    async fn get_plugin_file_dir(&self, plugin: &Plugin) -> crate::Result<PathBuf>;
}

/// Implementation for parsing the Thunderstore API
#[async_trait]
impl SourceHandler for ThunderstoreHandler {
    async fn parse_share_url(&self, url: &str) -> crate::Result<Plugin> {
        let state = KbApp::get().await?;
        let plugin_uuid = Uuid::new_v4();

        let thunderstore_details = thunderstore::extract_thunderstore_url(url).ok_or(
            crate::error::Error::ParseError(
                format!("Failed to extract correct information from provided thunderstore url: {}", url)
            )
        )?;

        let req_url = format!(
            "https://thunderstore.io/api/experimental/package/{}/{}",
            thunderstore_details.0,
            thunderstore_details.1
        );

        let package = download::fetch_json::<thunderstore::Package>(&req_url, &state.net_semaphore).await?;
        
        Ok(Plugin {
            id: format!("{}", plugin_uuid.as_hyphenated()),
            name: package.name,
            source: SupportedPluginSources::Thunderstore,
            api_url: req_url,
            is_enabled: true,
        })
    }

    async fn download_plugin(&self, collection_id: &str, plugin: &Plugin) -> crate::Result<()> {
        let state = KbApp::get().await?;

        let download_path = state.directories
            .collection_plugin_dir(collection_id)
            .join(format!("{}.zip", &plugin.name));

        // If download path somehow already exists there is no point downloading the same file again
        // likely will want to prefer to remove the file in the future once versioning is implemented.
        if !download_path.exists() {
            let package = download::fetch_json::<thunderstore::Package>(
                &plugin.api_url,
                &state.net_semaphore
            ).await?;

            let package_bytes = download::fetch_url(
                reqwest::Method::GET,
                &package.latest.download_url,
                &state.net_semaphore
            ).await?;

            let mut download_file: fs::File = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(&download_path)
                .await?;

            download_file.write_all(&package_bytes).await?;
        }

        // TODO: Find a better way to pass data to move closure
        let clone_download_path = download_path.clone();
        let clone_name = plugin.name.clone();
        let clone_id = collection_id.to_string();

        tokio::task::spawn_blocking(move || -> crate::Result<()> {
            let archive = File::open(&clone_download_path)?;
            let mut archive = zip::ZipArchive::new(archive)?;

            archive.extract(
                state.directories
                    .collection_plugin_dir(&clone_id)
                    .join(&clone_name)
            )?;

            Ok(())
        }).await??;

        fs::remove_file(&download_path).await?;

        Ok(())
    }

    async fn get_plugin_file_dir(&self, plugin: &Plugin) -> crate::Result<PathBuf> {
        // As Thunderstore doesn't use a standard layout for BepInEx files it will be instead better
        // to just search for the directory where the .dll files are located and provide that

        let state = KbApp::get().await?;

        let collection = plugin.get_collection(&state.db_pool).await?;

        let plugin_dir = state.directories
            .collection_plugin_dir(&collection.id)
            .join(&plugin.name);

        // TODO: Should default back to BepInEx/plugins for mods without a dll (sound ones)
        // TODO: Or if there are no folders inside do the whole folder
        // TODO: Should also move config files to correct place

        for entry in WalkDir::new(&plugin_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            if path.is_file()
                && (path.extension() == Some(OsStr::new("dll")) || path.extension() == Some(OsStr::new("lethalbundle"))) {
                return Ok(path.parent().ok_or(
                    crate::Error::FileSystemError(
                        format!("Failed to extract parent directory of {}", plugin_dir.display())
                    )
                )?.to_path_buf());
            }
        }

        Err(crate::Error::FileSystemError(
            format!("Failed to find plugin dll files in plugin directory: '{}'", plugin_dir.display())
        ))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub source: SupportedPluginSources,
    pub api_url: String,
    pub is_enabled: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ExportPlugin {
    pub name: String,
    pub source: SupportedPluginSources,
    pub api_url: String
}

impl Plugin {
    pub(crate) fn to_export(&self) -> ExportPlugin {
        ExportPlugin {
            name: self.name.clone(),
            source: self.source.clone(),
            api_url: self.api_url.clone()
        }
    }

    pub async fn from_collection(
        collection_id: &str,
        db: impl sqlx::Executor<'_, Database = sqlx::Sqlite> + Copy
    ) -> crate::Result<Vec<Self>> {
        let plugin_ids: Vec<_> = sqlx::query!(
            "
                SELECT plugin_id FROM collections_plugins_link
                WHERE collection_id = $1
            ",
            collection_id
        ).fetch_all(db).await?;

        let collected_ids: Vec<&str> = plugin_ids
            .iter()
            .map(|plugin_id| { plugin_id.plugin_id.as_str() })
            .collect();

        Ok(Self::get_many(collected_ids.as_slice(), db).await?)
    }

    pub async fn get_collection(
        &self,
        db: impl sqlx::Executor<'_, Database = sqlx::Sqlite> + Copy
    ) -> crate::Result<Collection> {
        let collection_id = sqlx::query!(
            "
                SELECT collection_id FROM collections_plugins_link
                WHERE plugin_id = $1
            ",
            self.id
        ).fetch_one(db).await?;

        Collection::get(&collection_id.collection_id, db).await?.ok_or(
            crate::error::Error::SQLiteStringError(
                format!("Failed to find collection with id {}", collection_id.collection_id)
            )
        )
    }

    pub async fn get(
        id: &str,
        db: impl sqlx::Executor<'_, Database = sqlx::Sqlite>
    ) -> crate::Result<Option<Self>> {
        Ok(Self::get_many(&[id], db).await?.into_iter().next())
    }

    pub async fn get_many(
        ids: &[&str],
        db: impl sqlx::Executor<'_, Database = sqlx::Sqlite>
    ) -> crate::Result<Vec<Self>> {
        let collected_ids = serde_json::to_string(&ids).map_err(|err| {
            crate::error::Error::ParseError(
                format!("Failed to convert ids {:?} to JSON: '{:#?}'", ids, err)
            )
        })?;

        let query_results = sqlx::query_as!(
            Self,
            "
                SELECT id, name, source, api_url, is_enabled as `is_enabled: bool`
                FROM plugins
                WHERE id IN (SELECT value FROM json_each($1))
            ",
            collected_ids
        ).fetch_all(db).await?;

        Ok(query_results)
    }

    /// Creates or updates a [`Plugin`] into the katabasis backend database.
    pub async fn update(
        &self,
        collection_id: String,
        db: impl sqlx::Executor<'_, Database = sqlx::Sqlite> + Copy
    ) -> crate::Result<()> {
        Collection::update_modified(&collection_id, db).await?;

        let is_enabled = self.is_enabled as i64;

        sqlx::query!(
            "
                INSERT INTO plugins (
                    id, name, source, api_url, is_enabled
                )
                VALUES (
                    $1, $2, $3, $4, $5
                )
                ON CONFLICT (id) DO UPDATE SET
                    name = $2,
                    source = $3,
                    api_url = $4,
                    is_enabled = $5
            ",
            self.id,
            self.name,
            self.source,
            self.api_url,
            is_enabled,
        ).execute(db).await?;

        let insert_result = sqlx::query!(
            "
                INSERT INTO collections_plugins_link (
                    collection_id, plugin_id
                )
                VALUES (
                    $1, $2
                )
                ON CONFLICT (collection_id, plugin_id) DO NOTHING
            ",
            collection_id,
            self.id
        ).execute(db).await;

        match insert_result {
            Ok(_) => {},
            Err(e) => {
                sqlx::query!(
                    "
                        DELETE FROM plugins WHERE id = $1
                    ",
                    self.id
                ).execute(db).await?;

                return Err(crate::error::Error::SQLiteStringError(
                    format!("Failed to insert link between collection and plugin: {:?}", e)
                ))
            }
        }

        Ok(())
    }

    pub async fn remove(
        &self,
        collection_id: &str,
        directories: &Directories,
        db: impl sqlx::Executor<'_, Database = sqlx::Sqlite> + Copy
    ) -> crate::Result<()> {
        let download_path = directories.collection_plugin_dir(collection_id).join(&self.name);

        if download_path.exists() {
            fs::remove_dir_all(download_path).await?;
        }

        sqlx::query!(
            "
                DELETE FROM collections_plugins_link WHERE plugin_id = $1
            ",
            self.id
        ).execute(db).await?;

        sqlx::query!(
            "
                DELETE FROM plugins WHERE id = $1
            ",
            self.id
        ).execute(db).await?;

        Ok(())
    }
}
