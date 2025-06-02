CREATE TABLE collections (
    name TEXT NOT NULL,
    game TEXT NOT NULL,
    game_version TEXT NOT NULL,
    install_type TEXT NOT NULL,

    created INTEGER NOT NULL,
    modified INTEGER NOT NULL,
    last_played INTEGER,

    PRIMARY KEY (name)
);

CREATE TABLE plugins (
    name TEXT NOT NULL,
    source TEXT NOT NULL,
    api_url TEXT NOT NULL,
    version TEXT NOT NULL,
    is_enabled INTEGER NOT NULL,
    icon_url TEXT,

    PRIMARY KEY (name)
);

CREATE TABLE collections_plugins_link (
    collection_name TEXT NOT NULL,
    plugin_name TEXT NOT NULL,

    PRIMARY KEY (collection_name, plugin_name)
);
