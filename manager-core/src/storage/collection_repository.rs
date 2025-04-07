use crate::data::{Collection, IntermediateCollection};
use crate::error;

/// Inserts or updates a collection.
pub async fn upsert(
    collection: &Collection,
    db: impl sqlx::Executor<'_, Database = sqlx::Sqlite>
) -> error::KatabasisResult<()> {
    let created_timestamp = collection.created.timestamp();
    let modified_timestamp = collection.modified.timestamp();
    let last_played_timestamp = collection.last_played
        .map(|x| x.timestamp());

    sqlx::query!(
        r#"
            INSERT INTO collections (
                id, name, game, game_version, created, modified, last_played
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7
            )
            ON CONFLICT (id) DO UPDATE SET
                name = $2,
                game = $3,
                game_version = $4,
                created = $5,
                modified = $6,
                last_played = $7
        "#,
        collection.id,
        collection.name,
        collection.game,
        collection.game_version,
        created_timestamp,
        modified_timestamp,
        last_played_timestamp
    ).execute(db).await?;

    Ok(())
}

/// Fetches a single collection.
pub async fn get(
    id: &str,
    db: impl sqlx::Executor<'_, Database = sqlx::Sqlite>
) -> error::KatabasisResult<Collection> {
    let single_result = sqlx::query_as!(
        IntermediateCollection,
        r#"
            SELECT id, name, game, game_version, created, modified, last_played
            FROM collections
            WHERE id = $1
        "#,
        id
    ).fetch_one(db).await?;

    Ok(single_result.try_into()?)
}

/// Fetches all collections.
pub async fn get_all(
    limit: Option<i32>,
    db: impl sqlx::Executor<'_, Database = sqlx::Sqlite>
) -> error::KatabasisResult<Vec<Collection>> {
    let limit = limit.unwrap_or(1000);

    let all_collections = sqlx::query_as!(
        IntermediateCollection,
        r#"
            SELECT id, name, game, game_version, created, modified, last_played
            FROM collections
            LIMIT $1
        "#,
        limit
    ).fetch_all(db).await?;

    Ok(
        all_collections
            .into_iter()
            .map(|x| x.try_into())
            .collect::<error::KatabasisResult<Vec<Collection>>>()?
    )
}

/// Removes a single collection.
pub async fn remove(
    collection: &Collection,
    db: impl sqlx::Executor<'_, Database = sqlx::Sqlite>
) -> error::KatabasisResult<()> {
    sqlx::query!(
        r#"
            DELETE FROM collections WHERE id = $1
        "#,
        collection.id
    ).execute(db).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::time::Duration;
    use chrono::Utc;
    use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
    use sqlx::SqlitePool;
    use crate::data::Collection;
    use crate::data::support::Game;
    use super::{upsert, get, get_all};

    fn test_collection(id: Option<String>) -> Collection {
        Collection {
            id: id.map_or("1".to_owned(), |x| x),
            name: "test collection".to_owned(),
            game: Game::LethalCompany,
            game_version: "".to_owned(),
            created: Utc::now(),
            modified: Utc::now(),
            last_played: None,
        }
    }

    async fn initialise_database() -> SqlitePool {
        let sql_options = SqliteConnectOptions::from_str("sqlite::memory:")
            .unwrap()
            .busy_timeout(Duration::from_secs(30))
            .journal_mode(SqliteJournalMode::Wal)
            .optimize_on_close(true, None);

        let pool = SqlitePoolOptions::new()
            .max_connections(100)
            .connect_with(sql_options)
            .await
            .unwrap();

        sqlx::migrate!().run(&pool).await.unwrap();

        pool
    }

    #[tokio::test]
    async fn test_insert_collection() {
        let pool = initialise_database().await;
        let collection = test_collection(None);

        match upsert(&collection, &pool).await {
            Ok(_) => assert!(true),
            Err(err) => {
                println!("{:#?}", err);
                assert!(false)
            },
        }

        match get("1", &pool).await {
            Ok(x) => assert_eq!(x.name, "test collection".to_owned()),
            Err(err) => {
                println!("{:#?}", err);
                assert!(false)
            }
        }
    }

    #[tokio::test]
    async fn test_get_collection() {
        let pool = initialise_database().await;
        let collection = test_collection(Some("TEST ID".to_owned()));

        match upsert(&collection, &pool).await {
            Ok(_) => assert!(true),
            Err(err) => {
                println!("{:#?}", err);
                assert!(false)
            },
        }

        match get("TEST ID", &pool).await {
            Ok(x) => assert_eq!(x.name, "test collection".to_owned()),
            Err(err) => {
                println!("{:#?}", err);
                assert!(false)
            }
        }
    }

    #[tokio::test]
    async fn test_get_all_collections() {
        let pool = initialise_database().await;
        let collection_one = test_collection(Some("COLLECTION ONE".to_owned()));
        let collection_two = test_collection(Some("COLLECTION TWO".to_owned()));

        match upsert(&collection_one, &pool).await {
            Ok(_) => assert!(true),
            Err(err) => {
                println!("{:#?}", err);
                assert!(false)
            },
        }

        match upsert(&collection_two, &pool).await {
            Ok(_) => assert!(true),
            Err(err) => {
                println!("{:#?}", err);
                assert!(false)
            },
        }

        match get_all(None, &pool).await {
            Ok(x) => {
                assert_eq!(x, vec![collection_one, collection_two]);
            }
            Err(err) => {
                println!("{:#?}", err);
                assert!(false)
            }
        }
    }
}
