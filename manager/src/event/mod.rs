use dashmap::DashMap;
use eyre::{eyre, OptionExt, Result};
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::OnceCell;
use uuid::Uuid;

static EVENT_STATE: OnceCell<Arc<EventState>> = OnceCell::const_new();

pub struct EventState {
    progress_bars: DashMap<Uuid, ProgressBar>,
}

impl EventState {
    pub async fn init() -> Result<Arc<Self>> {
        EVENT_STATE
            .get_or_try_init(|| async {
                Ok(Arc::new(Self {
                    progress_bars: DashMap::new(),
                }))
            })
            .await
            .cloned()
    }

    pub fn get() -> Result<Arc<Self>> {
        Ok(EVENT_STATE
            .get()
            .ok_or_eyre("event state is uninitialised")?
            .clone())
    }
}

#[cfg(feature = "cli")]
const CLI_PROGRESS_TOTAL: u64 = 1000;

#[derive(Debug, Clone, Serialize)]
pub struct ProgressBar {
    pub id: Uuid,
    pub label: String,
    pub total: f64,
    pub current: f64,
    pub last_update: f64,
    #[cfg(feature = "cli")]
    #[serde(skip)]
    pub cli_progress_bar: indicatif::ProgressBar,
}

#[derive(Debug, Clone)]
pub struct ProgressBarId(Uuid);

impl Drop for ProgressBarId {
    fn drop(&mut self) {
        let progress_uuid = self.0;

        tokio::spawn(async move {
            if let Ok(state) = EventState::get() {
                if let Some((_, bar)) = state.progress_bars.remove(&progress_uuid) {
                    #[cfg(feature = "cli")]
                    bar.cli_progress_bar.finish();
                }
            }
        });
    }
}

pub fn init_loading(
    label: &str,
    total: f64,
) -> Result<ProgressBarId> {
    let state = EventState::get()?;
    let id = ProgressBarId(Uuid::new_v4());

    state.progress_bars.insert(
        id.0,
        ProgressBar {
            id: id.0,
            label: label.to_owned(),
            total,
            current: 0.0,
            last_update: 0.0,
            #[cfg(feature = "cli")]
            cli_progress_bar: {
                let bar = indicatif::ProgressBar::new(CLI_PROGRESS_TOTAL);

                bar.set_position(0);
                bar.set_style(
                    indicatif::ProgressStyle::default_bar()
                        .template("{spinner:.green} [{elapsed_precise}] [{bar:.lime/green}] {pos}/{len} {msg}")?
                        .progress_chars("=>-"),
                );

                bar.enable_steady_tick(Duration::from_millis(100));

                bar
            }
        }
    );

    emit_loading(&id, 0.0, None)?;

    Ok(id)
}

pub fn emit_loading(
    id: &ProgressBarId,
    increment_frac: f64,
    message: Option<&str>,
) -> Result<()> {
    let state = EventState::get()?;

    let Some(mut progress) = state.progress_bars.get_mut(&id.0) else {
        return Err(eyre!("failed to fetch target progress bar"));
    };

    progress.current += increment_frac;
    let display_frac = progress.current / progress.total;

    if f64::abs(display_frac - progress.last_update) > 0.005 {
        #[cfg(feature = "cli")]
        {
            progress.cli_progress_bar.set_message(
                message.map(|x| x.to_owned())
                    .unwrap_or(progress.label.clone())
            );

            progress.cli_progress_bar.set_position(
                (display_frac * CLI_PROGRESS_TOTAL as f64).round() as u64,
            );
        }

        progress.last_update = display_frac;
    }

    Ok(())
}
