use crate::logger;

pub mod collection;

#[tauri::command]
pub async fn is_first_time() -> logger::Result<bool> {
    Ok(manager::first_usage().await?)
}
