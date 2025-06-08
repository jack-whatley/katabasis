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
    Target {
        #[arg(short, long)]
        slug: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    logger::setup()?;

    let cli = CLI::parse();

    match &cli.command {
        Commands::Directory => {
            println!("Application Directory: '{}'", manager::app_dir().display());
        }
        Commands::Target { slug } => {
            if let Some(slug) = slug {
                println!("{:#?}", manager::specific_target(&slug));
            }
            else {
                println!("{:#?}", manager::all_targets().collect::<Vec<_>>());
            }
        }
    }

    Ok(())
}
