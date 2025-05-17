use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
use tauri_plugin_updater::UpdaterExt;
use manager_api::{collection, plugin, Collection, KatabasisResult, Plugin};

// TODO: Implement logging and notification system in tauri::command's

#[tauri::command]
async fn get_collections() -> KatabasisResult<Vec<Collection>> {
    collection::get_all().await
}

#[tauri::command]
async fn get_collection(id: String) -> KatabasisResult<Collection> {
    collection::get(&id).await
}

#[tauri::command]
async fn get_plugins(collection_id: String) -> KatabasisResult<Vec<Plugin>> {
    plugin::get_all(&collection_id).await
}

#[tauri::command]
async fn remove_plugins(plugin_id: String) -> KatabasisResult<()> {
    plugin::remove(&plugin_id).await
}

#[tauri::command]
async fn create_collection(name: String, game: String) -> KatabasisResult<()> {
    let _ = collection::create(
        name,
        game,
        "Any".to_owned()
    ).await?;

    Ok(())
}

#[tauri::command]
async fn import_plugin(
    collection_id: String,
    plugin_url: String,
) -> KatabasisResult<()> {
    let collection = collection::get(&collection_id).await?;

    let _ = collection::add_plugin(
        &collection,
        &plugin_url,
    ).await?;

    Ok(())
}

#[tauri::command]
async fn switch_plugin(plugin_id: String, is_enabled: bool) -> KatabasisResult<()> {
    plugin::switch_plugin_state(&plugin_id, is_enabled).await
}

#[tauri::command]
async fn install_collection(collection_id: String) -> KatabasisResult<()> {
    let collection = collection::get(&collection_id).await?;

    collection::install(&collection).await
}

// #[tauri::command]
// async fn import_collection(file_path: String) -> Result<(), String> {
//     match manager::collections::import(file_path).await {
//         Ok(_) => Ok(()),
//         Err(e) => Err(e.to_string()),
//     }
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(
            |app| {
                app.handle().plugin(tauri_plugin_dialog::init())
                    .expect("Failed to initialise dialog");

                app.handle().plugin(tauri_plugin_updater::Builder::new().build())
                    .expect("Failed to initialise updater");

                let handle = app.handle().clone();

                tauri::async_runtime::spawn(async move {
                    update(handle).await
                });

                Ok(())
            }
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_collections,
            get_collection,
            get_plugins,
            remove_plugins,
            create_collection,
            import_plugin,
            switch_plugin,
            install_collection,
            // import_collection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;

        let do_update: bool = app.dialog()
            .message(format!("There is a new release, version: '{:?}'. Do you wish to update now?", update.version))
            .buttons(MessageDialogButtons::OkCancelCustom("Yes".to_string(), "No".to_string()))
            .title("Application Update Detected")
            .blocking_show();

        if do_update {
            update.download_and_install(
                |chunk_length, _content_length| {
                    downloaded += chunk_length;
                },
                || {
                    println!("download finished");
                }
            ).await?;

            let _message = app.dialog()
                .message("The application has successfully updated. It will now restart...")
                .kind(MessageDialogKind::Warning)
                .title("Application Update Complete")
                .blocking_show();

            app.restart();
        }
    }

    Ok(())
}
