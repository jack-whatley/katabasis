class CollectionCardModel {
    id: string;
    name: string;
    game: string;
    /* TODO: Add icon logic here... */

    constructor(id: string, name: string, game: string) {
        this.id = id;
        this.name = name;
        this.game = game;
    }
}

export { CollectionCardModel }
