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
    Target {
        #[arg(short, long)]
        slug: Option<String>,
    },
    Create {
        name: String,
        game: String,
    },
    Launch {
        name: String,
    },
    List,
    AddPlugin {
        id: String,
        url: String,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    logger::setup()?;

    let cli = CLI::parse();

    match &cli.command {
        Commands::Target { slug } => {
            if let Some(slug) = slug {
                println!("{:#?}", manager::specific_target(&slug));
            } else {
                println!("{:#?}", manager::all_targets().collect::<Vec<_>>());
            }
        }
        Commands::Create { name, game } => {
            manager::create_collection(&name, &game).await?;
        }
        Commands::Launch { name } => {
            manager::launch_collection_detached(&name).await?;
        }
        Commands::List => {
            for collection in manager::list_collections().await? {
                println!("{}", collection.name);
            }
        }
        Commands::AddPlugin { id, url } => {
            manager::add_plugin(&id, &url).await?;
        }
    }

    Ok(())
}
