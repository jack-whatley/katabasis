use crate::error;

pub struct ApplicationSettings {
    pub concurrent_downloads: ManagedSetting<i64>,
    pub concurrent_operations: ManagedSetting<i64>,
    pub retry_limit: ManagedSetting<i64>,
}

pub struct ManagedSetting<T> {
    setting: T,
    validator: Box<dyn Fn(T) -> bool>,
}

impl<T> ManagedSetting<T> where T: Clone + 'static {
    pub fn init(default: T, validator: fn(T) -> bool) -> ManagedSetting<T> {
        ManagedSetting {
            setting: default,
            validator: Box::new(validator),
        }
    }

    pub fn get(&self) -> T {
        self.setting.clone()
    }

    pub fn set(&mut self, value: T) -> bool {
        if (self.validator)(value.clone()) {
            self.setting = value;

            return true
        }

        false
    }
}

impl Default for ApplicationSettings {
    fn default() -> Self {
        Self {
            concurrent_downloads: ManagedSetting::init(
                10, |x| x > 0 && x < 26),
            concurrent_operations: ManagedSetting::init(
                25, |x| x > 0 && x < 51),
            retry_limit: ManagedSetting::init(
                5, |x| x > 0 && x < 6),
        }
    }
}

struct IntermediateSettings {
    pub download_limit: i64,
    pub file_limit: i64,
    pub retry_limit: i64,
}

impl TryFrom<IntermediateSettings> for ApplicationSettings {
    type Error = error::KatabasisError;

    fn try_from(value: IntermediateSettings) -> Result<Self, Self::Error> {
        Ok(
            Self {
                concurrent_downloads: ManagedSetting::init(
                    value.download_limit, |x| x > 0 && x < 26
                ),
                concurrent_operations: ManagedSetting::init(
                    value.file_limit, |x| x > 0 && x < 51
                ),
                retry_limit: ManagedSetting::init(
                    value.retry_limit, |x| x > 0 && x < 6
                )
            }
        )
    }
}

/// Fetches the [`ApplicationSettings`] settings struct stored in
/// the database.
pub async fn get_settings(
    db: impl sqlx::Executor<'_, Database = sqlx::Sqlite>
) -> error::KatabasisResult<ApplicationSettings> {
    let query_result = sqlx::query_as!(
        IntermediateSettings,
        r#"
            SELECT download_limit, file_limit, retry_limit FROM settings WHERE id = $1
        "#,
        1i64
    ).fetch_one(db).await?;

    Ok(query_result.try_into()?)
}

/// Sets the [`ApplicationSettings`] settings struct stored in
/// the database.
pub async fn set_settings(
    settings: ApplicationSettings,
    db: impl sqlx::Executor<'_, Database = sqlx::Sqlite>
) -> error::KatabasisResult<()> {
    let downloads = settings.concurrent_downloads.get();
    let operations = settings.concurrent_operations.get();
    let retries = settings.retry_limit.get();

    sqlx::query!(
        r#"
            UPDATE settings
            SET download_limit = $1, file_limit = $2, retry_limit = $3
            WHERE id = $4
        "#,
        downloads,
        operations,
        retries,
        1u8
    ).execute(db).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::storage::initialise_database;
    use super::{ManagedSetting, get_settings, set_settings, ApplicationSettings};

    #[test]
    fn test_get_managed_setting() {
        let managed_setting = ManagedSetting::init(
            10, |x| x > 0 && x < 26);

        assert_eq!(managed_setting.get(), 10);
    }

    #[test]
    fn test_set_valid_managed_setting() {
        let mut managed_setting = ManagedSetting::init(
            10, |x| x > 0 && x < 26);

        assert!(managed_setting.set(25));
        assert_eq!(managed_setting.get(), 25);
    }

    #[test]
    fn test_set_invalid_managed_setting() {
        let mut managed_setting = ManagedSetting::init(
            10, |x| x > 0 && x < 26);

        assert!(!managed_setting.set(100));
        assert_eq!(managed_setting.get(), 10);

        assert!(!managed_setting.set(-1));
        assert_eq!(managed_setting.get(), 10);
    }

    #[tokio::test]
    async fn test_default_application_settings() {
        let pool = initialise_database().await;

        match get_settings(&pool).await {
            Ok(settings) => {
                assert_eq!(settings.concurrent_downloads.get(), 10);
                assert_eq!(settings.concurrent_operations.get(), 25);
                assert_eq!(settings.retry_limit.get(), 5);
            },
            Err(error) => {
                println!("{:#?}", error);
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_set_application_settings() {
        let pool = initialise_database().await;
        let mut settings = ApplicationSettings::default();

        assert!(settings.concurrent_downloads.set(25));
        assert!(settings.concurrent_operations.set(50));
        assert!(settings.retry_limit.set(1));

        match set_settings(settings, &pool).await {
            Ok(_) => assert!(true),
            Err(error) => {
                println!("{:#?}", error);
                assert!(false);
            }
        }

        match get_settings(&pool).await {
            Ok(settings) => {
                assert_eq!(settings.concurrent_downloads.get(), 25);
                assert_eq!(settings.concurrent_operations.get(), 50);
                assert_eq!(settings.retry_limit.get(), 1);
            },
            Err(error) => {
                println!("{:#?}", error);
                assert!(false);
            }
        }
    }
}
