use clap::{Parser, Subcommand};
use eyre::Result;

mod logger;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Directory,
}

#[tokio::main]
async fn main() -> Result<()> {
    logger::setup()?;

    let cli = CLI::parse();

    match &cli.command {
        Commands::Directory => {
            println!("Application Directory: '{}'", manager::app_dir().display());
        }
    }

    Ok(())
}
