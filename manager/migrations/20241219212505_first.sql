-- Main Migration Script
CREATE TABLE collections (
    id TEXT NOT NULL,
    name TEXT NOT NULL,
    game TEXT NOT NULL,
    game_version TEXT NOT NULL,

    created INTEGER NOT NULL,
    modified INTEGER NOT NULL,
    last_played INTEGER,

    PRIMARY KEY (id)
);
