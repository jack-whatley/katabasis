use manager::{Collection, Plugin};

#[tauri::command]
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

#[tauri::command]
fn get_manager_version() -> String {
    manager::get_version()
}

#[tauri::command]
async fn get_collections() -> Result<Vec<Collection>, String> {
    match manager::collections::get_all(None).await {
        Ok(collections) => Ok(collections),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn get_collection(id: String) -> Result<Collection, String> {
    match manager::collections::get_one(&id).await {
        Ok(collection) => Ok(collection),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn get_plugins(collection_id: String) -> Result<Vec<Plugin>, String> {
    match manager::collections::fetch_all_plugins(&collection_id).await {
        Ok(plugins) => Ok(plugins),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn remove_plugins(collection_id: String, plugin_id: String) -> Result<(), String> {
    match manager::collections::remove_plugin(&collection_id, &plugin_id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_version,
            get_manager_version,
            get_collections,
            get_collection,
            get_plugins,
            remove_plugins
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
