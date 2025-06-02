use std::str::FromStr;

use manager::{collections, SupportedGames};

pub async fn create(
    name: String,
    game: String,
    game_version: Option<String>,
) -> anyhow::Result<()> {
    let parsed_game = SupportedGames::from_str(&game)?;
    let game_version = game_version.unwrap_or("Any".to_string());

    let name = collections::create::create(name, parsed_game, game_version).await?;

    println!("Created Collection: {}", name);

    Ok(())
}
