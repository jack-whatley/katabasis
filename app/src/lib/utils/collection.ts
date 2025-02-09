interface ICollectionCardModel {
    id: string;
    name: string;
    game: string;
    game_version: string;
}

interface IPlugin {
    id: string;
    name: string;
    source: string;
    api_url: string;
}

//     pub created: DateTime<Utc>,
//     pub modified: DateTime<Utc>,
//     pub last_played: Option<DateTime<Utc>>,

class CollectionCardModel implements ICollectionCardModel {
    id: string;
    name: string;
    game: string;
    game_version: string;
    /* TODO: Add icon logic here... */

    constructor(id: string, name: string, game: string, game_version: string) {
        this.id = id;
        this.name = name;
        this.game = game;
        this.game_version = game_version;
    }
}

export { type ICollectionCardModel, type IPlugin, CollectionCardModel }
