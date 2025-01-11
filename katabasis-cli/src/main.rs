use std::str::FromStr;
use clap::{Parser, Subcommand};
use manager::{collections, SupportedGames, SupportedPluginSources};
use anyhow::Result;

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

        #[arg(short, long)]
        plugin: Option<String>,
    },
    Remove {
        #[arg(short, long)]
        id: String,
    },
    RemoveAll,
    AddPlugin {
        #[arg(short, long)]
        collection: String,

        #[arg(short, long)]
        source: String,

        #[arg(short, long)]
        url: String
    },
    RemovePlugin {
        #[arg(short, long)]
        cid: String,

        #[arg(short, long)]
        pid: String,
    },
    InstallCollection {
        #[arg(long)]
        id: String
    },
    Export {
        #[arg(long)]
        id: String
    },
    Import {
        #[arg(long)]
        path: String
    }
}

async fn create_collection(name: String, game: String, game_version: Option<String>) -> manager::Result<String> {
    let parsed_game = SupportedGames::from_str(&game)?;

    Ok(collections::create::create(name, parsed_game, game_version.unwrap_or("Any".to_string())).await?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = CLI::parse();

    match &cli.command {
        Some(Commands::Collection { command }) => {
            match &command {
                Some(CollectionCreate::Create { name, game, game_version }) => {
                    println!("Creating collection {}", name);

                    let collection_id = create_collection(name.clone(), game.clone(), game_version.clone()).await?;

                    println!("Created Collection: {}", collection_id);
                }
                Some(CollectionCreate::List { number, plugin }) => {
                    if plugin.is_none() {
                        let collections = collections::get_all(number.clone()).await?;

                        println!("Found {} collections:", collections.len());

                        for collection in collections {
                            println!("- {}", collection);
                        }
                    }
                    else {
                        let plugin_id = plugin.clone().unwrap();

                        let plugins = collections::fetch_all_plugins(&plugin_id).await?;

                        for plugin in plugins {
                            println!("- {:#?}", plugin);
                        }
                    }
                },
                Some(CollectionCreate::Remove { id }) => {
                    collections::remove(id.clone()).await?;
                },
                Some(CollectionCreate::RemoveAll) => {
                    collections::remove_all().await?;
                },
                None => {
                    println!("Please provide the required arguments, run the command katabasis-cli.exe collection create -h / --help for help.");
                }
                Some(CollectionCreate::AddPlugin { collection, source, url }) => {
                    collections::add_plugin(collection.as_str(), SupportedPluginSources::from(source.clone()), url.as_str()).await?;
                }
                Some(CollectionCreate::RemovePlugin { cid, pid }) => {
                    collections::remove_plugin(cid.as_str(), pid.as_str()).await?;
                }
                Some(CollectionCreate::InstallCollection { id }) => {
                    collections::install(&id).await?;
                }
                Some(CollectionCreate::Export { id }) => {
                    let file_path = collections::export(&id).await?;

                    println!("Exported to: {}", file_path);
                }
                Some(CollectionCreate::Import { path }) => {
                    let name = collections::import(&path).await?;

                    println!("Imported Collection: '{}'", name);
                }
            }
        }
        None => {
            println!("No command provided, run the command katabasis-cli.exe -h / --help for help.");
        }
    }

    Ok(())
}
