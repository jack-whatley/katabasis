mod commands;
mod logger;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::is_first_time,
            commands::collection::list_collections,
            commands::collection::list_collection,
            commands::collection::launch_collection,
            commands::collection::shortcut_collection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
