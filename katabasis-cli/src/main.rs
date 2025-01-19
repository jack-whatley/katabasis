use clap::{Parser, Subcommand};
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
async fn main() {
    if let Err(error) = runner::run(Opt::parse()).await {
        println!("ERROR: {}", error);
        std::process::exit(1);
    }
}
