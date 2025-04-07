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

CREATE TABLE plugins (
    id TEXT NOT NULL,
    name TEXT NOT NULL,
    source TEXT NOT NULL,
    api_url TEXT NOT NULL,
    is_enabled INTEGER NOT NULL,

    PRIMARY KEY (id)
);

CREATE TABLE collections_plugins_link (
    collection_id TEXT NOT NULL,
    plugin_id TEXT NOT NULL,

    -- Using compound primary key to prevent duplicate plugins appearing in collections
    PRIMARY KEY (collection_id, plugin_id)
);
