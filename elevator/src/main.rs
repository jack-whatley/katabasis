use clap::Parser;
use crate::opt::Opt;

mod opt;
mod utils;

fn main() {
    if let Err(error) = Opt::run(Opt::parse()) {
        println!("Error: {}", error);
        std::process::exit(1);
    }

    std::process::exit(0);
}
