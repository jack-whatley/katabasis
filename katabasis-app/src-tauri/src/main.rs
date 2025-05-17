#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use simplelog::{ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger};

fn main() {
    CombinedLogger::init(vec![
        #[cfg(debug_assertions)]
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Stdout,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("application.log").expect("log file 'application.log' creation failed"),
        )
    ]).expect("logger failed to initialise'");

    app_lib::run()
}
