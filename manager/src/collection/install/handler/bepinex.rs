use crate::collection::install::handler::PluginHandler;
use crate::collection::{Collection, Plugin};
use crate::utils::fs::{CopyFileOpts, PluginZip};
use crate::utils::{fs, paths};
use async_trait::async_trait;
use std::borrow::Cow;
use std::path::{Path, PathBuf};

pub struct BepInExHandler;

#[async_trait]
impl PluginHandler for BepInExHandler {
    async fn extract(&self, zip: PluginZip, dir: PathBuf, _plugin_name: &str) -> eyre::Result<()> {
        fs::extract_archive(zip, dir, |rel_path| {
            let mut components = rel_path.components();

            if components.clone().count() == 1 {
                return Ok(None);
            }

            components.next();

            Ok(Some(Cow::Borrowed(components.as_path())))
        }).await?;

        Ok(())
    }

    async fn install(&self, src: &Path, _plugin_name: &str, collection: &Collection) -> eyre::Result<()> {
        let dest = paths::collection_dir(&collection.name);

        if !dest.exists() {
            tokio::fs::create_dir_all(&dest).await?;
        }

        fs::copy_dir_contents_to(src, dest, |rel_path, _| -> eyre::Result<CopyFileOpts> {
            if rel_path.extension().is_some_and(|ext| ext == "cfg") {
                Ok(CopyFileOpts { should_copy_file: true, should_overwrite_file: false })
            }
            else {
                Ok(CopyFileOpts { should_copy_file: false, should_overwrite_file: true })
            }
        }).await?;

        Ok(())
    }

    async fn uninstall(&self, _plugin: &Plugin, collection: &Collection) -> eyre::Result<()> {
        for file in walk_bepinex_core_dir(collection).await? {
            tokio::fs::remove_file(file).await?;
        }

        Ok(())
    }

    async fn switch(&self, enabled: bool, _plugin: &Plugin, collection: &Collection) -> eyre::Result<()> {
        for file in walk_bepinex_core_dir(collection).await? {
            fs::switch_file(&file, enabled).await?;
        }

        Ok(())
    }
}

async fn walk_bepinex_core_dir(collection: &Collection) -> eyre::Result<Vec<PathBuf>> {
    let dest = paths::collection_dir(&collection.name)
        .join("BepInEx")
        .join("core");

    let mut read_dir = tokio::fs::read_dir(&dest).await?;
    let mut core_files: Vec<PathBuf> = Vec::new(); 

    while let Some(entry) = read_dir.next_entry().await? {
        if entry.file_type().await.is_ok_and(|x| x.is_file()) {
            core_files.push(entry.path());
        }
    }

    Ok(core_files)
}
