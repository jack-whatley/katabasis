use manager::{Collection, Plugin, SupportedGames, SupportedPluginSources};

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

#[tauri::command]
async fn create_collection(name: String, game: String) -> Result<(), String> {
    let parsed_game = SupportedGames::from(game);
    let game_version = "Any".to_string();

    match manager::collections::create::create(name, parsed_game, game_version).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn import_plugin(
    collection_id: String,
    plugin_source: String,
    plugin_url: String
) -> Result<(), String> {
    let parsed_source = SupportedPluginSources::from(plugin_source);

    match manager::collections::add_plugin(&collection_id, parsed_source, &plugin_url).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn switch_plugin(plugin_id: String, is_enabled: bool) -> Result<(), String> {
    match manager::collections::switch_plugin(&plugin_id, is_enabled).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn install_collection(collection_id: String) -> Result<(), String> {
    match manager::collections::install(&collection_id).await {
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
            remove_plugins,
            create_collection,
            import_plugin,
            switch_plugin,
            install_collection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
