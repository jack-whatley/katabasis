use eyre::OptionExt;
use manager::FrontendCollection;
use crate::logger;

#[tauri::command]
pub async fn list_collections() -> logger::Result<Vec<FrontendCollection>> {
    Ok(manager::list_collections().await?.into_iter().map(Into::into).collect())
}

#[tauri::command]
pub async fn list_collection(name: &str) -> logger::Result<FrontendCollection> {
    Ok(
        manager::list_collections()
            .await?
            .into_iter()
            .map(|x| x.into())
            .find(|x: &FrontendCollection| x.name == name)
            .ok_or_eyre("failed to find collection matching name")?)
}

#[tauri::command]
pub async fn launch_collection(name: &str) -> logger::Result<()> {
    Ok(
        manager::launch_collection_detached(name).await?)
}

#[tauri::command]
pub async fn shortcut_collection(name: &str) -> logger::Result<()> {
    Ok(
        manager::create_shortcut(name).await?)
}
