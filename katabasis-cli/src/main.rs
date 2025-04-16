use clap::{Parser, Subcommand};
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};
use opt::Opt;

mod collection;
mod opt;
mod runner;
mod plugin;

// TODO: Super Important: https://github.com/JetBrains/intellij-community/blob/master/native/WinElevator/README.txt
// Should create something similar to this for handling symlinks (dont need to bother with piping or general commands
// though, probably can just use windows api for symlinks directly)

#[derive(Subcommand)]
enum CollectionCreate {
    AddPlugin {
        #[arg(short, long)]
        collection: String,

        #[arg(short, long)]
        source: String,

        #[arg(short, long)]
        url: String,
    },
    RemovePlugin {
        #[arg(short, long)]
        cid: String,

        #[arg(short, long)]
        pid: String,
    },
    InstallCollection {
        #[arg(long)]
        id: String,
    },
    Export {
        #[arg(long)]
        id: String,
    },
    Import {
        #[arg(long)]
        path: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // if let Err(error) = runner::run(Opt::parse()).await {
    //     println!("ERROR: {}", error);
    //     std::process::exit(1);
    // }

    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto
    ).expect("Could not setup logging");

    let name = "Test Collection".to_owned();
    let game = "lethalcompany".to_owned();
    let version = "Any".to_owned();

    // let collection = manager_api::collection::get("eedfd873-7649-4623-80a9-ee121da20ece").await?;

    // match manager_api::collection::remove(&collection).await {
    //     Ok(_) => {
    //         println!("Collection removed successfully!");
    //     },
    //     Err(e) => {
    //         println!("Error:\n{:#?}", e);
    //     }
    // }

    match manager_api::collection::create(name, game, version).await {
        Ok(collection) => {
            println!("Collection created successfully!\n{:#?}", collection);
        }
        Err(e) => {
            println!("Error creating Collection\n{:#?}", e);
        }
    }

    Ok(())
}
