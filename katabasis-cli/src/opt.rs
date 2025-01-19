use clap::Parser;

#[derive(Parser, Debug)]
pub struct Opt {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    #[clap(alias = "c")]
    Collection(CollectionOpt),

    #[clap(alias = "p")]
    Plugin(PluginOpt),
}

#[derive(Parser, Debug)]
pub struct CollectionOpt {
    #[clap(subcommand)]
    pub command: CollectionCommand,
}

#[derive(Parser, Debug)]
pub enum CollectionCommand {
    Create {
        name: String,

        game: String,

        #[arg(short, long)]
        game_version: Option<String>,
    },
    List {
        #[arg(short, long)]
        limit: Option<u32>,
    },
    Remove {
        id: String,
    },
    RemoveAll,
}

#[derive(Parser, Debug)]
pub struct PluginOpt {
    #[clap(subcommand)]
    pub command: PluginCommand,
}

#[derive(Parser, Debug)]
pub enum PluginCommand {

}
