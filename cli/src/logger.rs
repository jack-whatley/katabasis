use eyre::{Context, Result};
use tracing::{Level, level_filters::LevelFilter};
use tracing_subscriber::{Layer, Registry, layer::SubscriberExt};

/// Initialises the tracing logger. Only to the console
/// for this application.
pub async fn setup() -> Result<()> {
    manager::list_collections().await?;

    let log_path = manager::log_path();

    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(log_path)?;

    let subscriber = Registry::default().with(
        tracing_subscriber::fmt::layer()
            .with_writer(log_file)
            .with_ansi(false)
            .with_filter(LevelFilter::from_level(Level::INFO)),
    );

    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to initialise the application logger")?;

    Ok(())
}
