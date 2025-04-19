use crate::data::{Collection, CollectionPluginLink, Plugin};
use crate::error;
use log::warn;

/// Inserts or updates a plugin.
pub async fn upsert(
    collection: &Collection,
    plugin: &Plugin,
    db: impl sqlx::Executor<'_, Database = sqlx::Sqlite> + Copy
) -> error::KatabasisResult<()> {
    let link = CollectionPluginLink {
        collection_id: collection.id.clone(),
        plugin_id: plugin.id.clone(),
    };

    let plugin_result = sqlx::query!(
        r#"
            INSERT INTO plugins (
                id, name, source, api_url, version, is_enabled, icon_url
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7
            )
            ON CONFLICT (id) DO UPDATE SET
                name = $2,
                source = $3,
                api_url = $4,
                version = $5,
                is_enabled = $6,
                icon_url = $7
        "#,
        plugin.id,
        plugin.name,
        plugin.source,
        plugin.api_url,
        plugin.version,
        plugin.is_enabled,
        plugin.icon_url
    ).execute(db).await;

    let link_result = sqlx::query!(
        r#"
            INSERT INTO collections_plugins_link (
                collection_id, plugin_id
            ) VALUES (
                $1, $2
            ) ON CONFLICT DO NOTHING
        "#,
        link.collection_id,
        link.plugin_id,
    ).execute(db).await;

    if plugin_result.is_err() || link_result.is_err() {
        remove(plugin, db).await?;
    }

    Ok(())
}

/// Fetches a single plugin from an ID.
pub async fn get(
    id: &str,
    db: impl sqlx::Executor<'_, Database = sqlx::Sqlite>
) -> error::KatabasisResult<Plugin> {
    Ok(
        sqlx::query_as!(
            Plugin,
            r#"
                SELECT id, name, source, api_url, version, is_enabled as `is_enabled: bool`, icon_url
                FROM plugins
                WHERE id = $1
            "#,
            id
        ).fetch_one(db).await?
    )
}

/// Fetches all plugins for a single collection.
pub async fn get_all(
    collection: &Collection,
    db: impl sqlx::Executor<'_, Database = sqlx::Sqlite> + Copy
) -> error::KatabasisResult<Vec<Plugin>> {
    let links = sqlx::query_as!(
        CollectionPluginLink,
        r#"
            SELECT collection_id, plugin_id FROM collections_plugins_link WHERE collection_id = $1
        "#,
        collection.id
    ).fetch_all(db).await?;

    let mut plugins = Vec::with_capacity(links.len());

    for link in links {
        plugins.push(get(&link.plugin_id, db).await?);
    }

    Ok(plugins)
}

/// Removes a plugin from the database.
pub async fn remove(
    plugin: &Plugin,
    db: impl sqlx::Executor<'_, Database = sqlx::Sqlite> + Copy
) -> error::KatabasisResult<()> {
    let remove_plugin = sqlx::query!(
        r#"
            DELETE FROM plugins WHERE id = $1
        "#,
        plugin.id
    ).execute(db).await;

    match remove_plugin {
        Ok(_) => {}
        Err(err) => {
            warn!("Failed to remove a plugin:\n{:#?}", err);
        }
    }

    let remove_link = sqlx::query!(
        r#"
            DELETE FROM collections_plugins_link WHERE plugin_id = $1
        "#,
        plugin.id
    ).execute(db).await;

    match remove_link {
        Ok(_) => {}
        Err(err) => {
            warn!("Failed to remove a plugin link:\n{:#?}", err);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::support::{InstallType, PluginSource, PluginTarget};
    use crate::storage::{collection_repository, initialise_database};
    use chrono::Utc;

    async fn create_test_data(
        db: impl sqlx::Executor<'_, Database = sqlx::Sqlite> + Copy
    ) -> error::KatabasisResult<(Collection, Plugin, Plugin)> {
        let collection = Collection {
            id: "1".to_owned(),
            name: "EXAMPLE COLLECTION".to_owned(),
            game: PluginTarget::LethalCompany,
            game_version: "Any".to_owned(),
            install_type: InstallType::Copy,
            created: Utc::now(),
            modified: Utc::now(),
            last_played: None,
        };

        collection_repository::upsert(&collection, db).await?;

        let plugin_one = Plugin {
            id: "PLUGIN ONE".to_owned(),
            name: "EXAMPLE PLUGIN".to_owned(),
            source: PluginSource::Thunderstore,
            api_url: "".to_owned(),
            version: "0.0.1".to_owned(),
            is_enabled: true,
            icon_url: None
        };

        let plugin_two = Plugin {
            id: "PLUGIN TWO".to_owned(),
            name: "EXAMPLE PLUGIN TWO".to_owned(),
            source: PluginSource::Thunderstore,
            api_url: "".to_owned(),
            version: "0.1.5".to_owned(),
            is_enabled: false,
            icon_url: None
        };

        upsert(&collection, &plugin_one, db).await?;
        upsert(&collection, &plugin_two, db).await?;

        Ok((collection, plugin_one, plugin_two))
    }

    #[tokio::test]
    async fn test_upsert() {
        let pool = initialise_database().await;

        let (collection, p1, p2) = match create_test_data(&pool).await {
            Ok(a) => a,
            Err(err) => panic!("Failed to create test data:\n{:#?}", err),
        };

        match get_all(&collection, &pool).await {
            Ok(plugins) => {
                assert_eq!(plugins.len(), 2);
                assert_eq!(plugins, vec![p1, p2]);
            }
            Err(err) => {
                println!("Failed to get all plugins:\n{:#?}", err);
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_remove() {
        let pool = initialise_database().await;

        let (collection, p1, p2) = match create_test_data(&pool).await {
            Ok(a) => a,
            Err(err) => panic!("Failed to create test data:\n{:#?}", err),
        };

        match remove(&p1, &pool).await {
            Ok(_) => {},
            Err(err) => {
                println!("Failed to remove a plugin:\n{:#?}", err);
                assert!(false);
            }
        }

        match get(&p1.id, &pool).await {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }

        match get_all(&collection, &pool).await {
            Ok(plugins) => {
                assert_eq!(plugins.len(), 1);
                assert_eq!(plugins, vec![p2]);
            }
            Err(err) => {
                println!("Failed to get a plugin:\n{:#?}", err);
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_update() {
        let pool = initialise_database().await;

        let (collection, mut p1, _) = match create_test_data(&pool).await {
            Ok(a) => a,
            Err(err) => panic!("Failed to create test data:\n{:#?}", err),
        };

        p1.name = "MODIFIED PLUGIN".to_owned();

        match upsert(&collection, &p1, &pool).await {
            Ok(_) => {},
            Err(err) => {
                println!("Failed to update plugin:\n{:#?}", err);
                assert!(false);
            }
        }

        match get(&p1.id, &pool).await {
            Ok(plugin) => {
                assert_eq!(plugin.name, "MODIFIED PLUGIN");
            },
            Err(err) => {
                println!("Failed to get updated plugin:\n{:#?}", err);
                assert!(false);
            }
        }
    }
}
