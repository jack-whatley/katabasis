use crate::opt::{CollectionCommand, Command, Opt};

pub async fn run(opt: Opt) -> anyhow::Result<()> {
    match opt.command {
        Command::Collection(col) => match col.command {
            CollectionCommand::Create {
                name,
                game,
                game_version,
            } => {
                crate::collection::create(name, game, game_version).await?;
            }
            CollectionCommand::List {
                limit
            } => {
                let mods = manager::collections::get_all(limit).await?;

                for m in mods {
                    println!("{:#?}", m);
                }
            },
            CollectionCommand::Remove {
                id
            } => {
                manager::collections::remove(id).await?;
            },
            CollectionCommand::RemoveAll => { manager::collections::remove_all().await?; },
            CollectionCommand::InstallCollection { id } => {
                manager::collections::install(&id).await?;
            }
        },
        Command::Plugin(_) => {}
    }

    Ok(())
}
