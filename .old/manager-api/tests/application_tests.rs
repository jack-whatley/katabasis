use manager_api::collection;
use manager_api::plugin;

const BEPINEX_DIRS: &[&str] = &[
    "BepInEx",
    ".doorstop_version",
    "changelog.txt",
    "doorstop_config.ini",
    "winhttp.dll"
];

#[tokio::test]
async fn initialise_bepinex_collection() {
    let collection = collection::create(
        "TEST COLLECTION".to_owned(),
        "lethalcompany".to_owned(),
        "Any".to_owned(),
    ).await.unwrap();

    let path = collection::directory(&collection).await.unwrap();

    assert!(path.exists());

    for dir in BEPINEX_DIRS {
        assert!(path.join(dir).exists());
    }

    // Removing directory cleans up database
    tokio::fs::remove_dir_all(path).await.unwrap();
}

#[tokio::test]
async fn install_thunderstore_plugins() {
    let collection = collection::create(
        "TEST COLLECTION".to_owned(),
        "lethalcompany".to_owned(),
        "Any".to_owned(),
    ).await.unwrap();

    let path = collection::directory(&collection).await.unwrap();

    assert!(path.exists());

    collection::add_plugin(
        &collection,
        "https://thunderstore.io/c/lethal-company/p/RugbugRedfern/Skinwalkers/"
    ).await.unwrap();

    assert!(
        path.join("BepInEx")
            .join("plugins")
            .join("SkinwalkerMod.dll")
            .exists()
    );

    // Removing directory cleans up database
    tokio::fs::remove_dir_all(path).await.unwrap();
}

#[tokio::test]
async fn switch_plugin_state() {
    let collection = collection::create(
        "TEST COLLECTION".to_owned(),
        "lethalcompany".to_owned(),
        "Any".to_owned(),
    ).await.unwrap();

    let path = collection::directory(&collection).await.unwrap();

    assert!(path.exists());

    let plugin = collection::add_plugin(
        &collection,
        "https://thunderstore.io/c/lethal-company/p/RugbugRedfern/Skinwalkers/"
    ).await.unwrap();

    assert!(
        path.join("BepInEx")
            .join("plugins")
            .join("SkinwalkerMod.dll")
            .exists()
    );

    plugin::switch_plugin_state(
        &plugin.id,
        false
    ).await.unwrap();

    match plugin::state(&plugin.id).await {
        Ok(state) => assert!(!state),
        Err(e) => {
            println!("{:#?}", e);
            assert!(false);
        }
    }

    tokio::fs::remove_dir_all(path).await.unwrap();
}
