#![cfg(windows)]

use clap::Parser;
use crate::opt::Opt;

mod opt;
mod utils;

#[tokio::main]
async fn main() {
    if let Err(error) = Opt::run(Opt::parse()).await {
        eprintln!("{}", error);

        std::process::exit(1);
    }

    std::process::exit(0);
}
