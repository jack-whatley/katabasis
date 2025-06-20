use eyre::{Context, Result};
use tracing::{Level, level_filters::LevelFilter};
use tracing_subscriber::{Layer, Registry, layer::SubscriberExt};

/// Initialises the tracing logger. Only to the console
/// for this application.
pub fn setup() -> Result<()> {
    let subscriber = Registry::default().with(
        tracing_subscriber::fmt::layer()
            .with_ansi(true)
            .with_filter(LevelFilter::from_level(Level::INFO)),
    );

    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to initialise the application logger")?;

    Ok(())
}
