#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod application;
mod utils;
mod notifications;

use eframe::egui;
use crate::application::App;

fn main() -> eframe::Result<()> {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Katabasis::ModManager",
        opts,
        Box::new(|_ctx| {
            Ok(Box::new(App::new()))
        })
    )
}
