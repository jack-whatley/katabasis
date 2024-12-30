use std::str::FromStr;
use clap::{Parser, Subcommand};
use manager::{collections, SupportedGames};

#[derive(Parser)]
#[command(version, about = "A CLI for managing and installing mod collections using the katabasis API.", long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Collection {
        #[command(subcommand)]
        command: Option<CollectionCreate>
    }
}

#[derive(Subcommand)]
enum CollectionCreate {
    Create {
        #[arg(short, long)]
        name: String,

        #[arg(short, long)]
        game: String,

        #[arg(long)]
        game_version: Option<String>,
    },
    List {
        #[arg(short, long)]
        number: Option<u32>,
    },
    Remove {
        #[arg(short, long)]
        id: String,
    }
}

async fn create_collection(name: String, game: String, game_version: Option<String>) -> manager::Result<String> {
    let parsed_game = SupportedGames::from_str(&game)?;

    Ok(collections::create::create(name, parsed_game, game_version.unwrap_or("Any".to_string())).await?)
}

#[tokio::main]
async fn main() -> manager::Result<()> {
    let cli = CLI::parse();

    match &cli.command {
        Some(Commands::Collection { command }) => {
            match &command {
                Some(CollectionCreate::Create { name, game, game_version }) => {
                    println!("Creating collection {}", name);

                    let collection_id = create_collection(name.clone(), game.clone(), game_version.clone()).await?;

                    println!("Created Collection: {}", collection_id);
                }
                Some(CollectionCreate::List { number }) => {
                    let collections = collections::get_all(number.clone()).await?;

                    println!("Found {} collections:", collections.len());

                    for collection in collections {
                        println!("- {}", collection);
                    }
                },
                Some(CollectionCreate::Remove { id }) => {
                    collections::remove(id.clone()).await?;
                }
                None => {
                    println!("Please provide the required arguments, run the command katabasis-cli.exe collection create -h / --help for help.");
                }
            }
        }
        None => {
            println!("No command provided, run the command katabasis-cli.exe -h / --help for help.");
        }
    }

    Ok(())
}
