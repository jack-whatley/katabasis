use std::path::Path;
use eyre::ensure;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use crate::collection::Collection;
use crate::thunderstore::version::VersionIdent;
use crate::utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportCollection {
    pub name: String,
    pub slug: String,
    pub plugins: Vec<VersionIdent>,
}

impl ExportCollection {
    pub fn from_collection(collection: &Collection) -> Self {
        Self {
            name: collection.name.clone(),
            slug: collection.game.slug.clone().to_string(),
            plugins: collection.plugins
                .iter()
                .map(|x| x.ident().clone())
                .collect(),
        }
    }

    pub async fn from_file(path: impl AsRef<Path>) -> eyre::Result<Self> {
        let path = path.as_ref();

        ensure!(
            path.exists(),
            "failed to import collection as file does not exist '{}'",
            path.display()
        );

        let file_content = tokio::fs::read(path).await?;

        Ok(serde_json::from_slice(file_content.as_slice())?)
    }

    pub async fn export(&self) -> eyre::Result<()> {
        let export_path = utils::paths::collection_export_path(&self.name);

        let mut file = tokio::fs::File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(export_path)
            .await?;

        let json = serde_json::to_vec(self)?;
        file.write_all(json.as_slice()).await?;

        Ok(())
    }
}
