CREATE TABLE settings (
    id INT NOT NULL,
    download_limit INT NOT NULL,
    file_limit INT NOT NULL,
    retry_limit INT NOT NULL,

    PRIMARY KEY (id)
);

INSERT INTO settings (
    id, download_limit, file_limit, retry_limit
) VALUES (
    1, 10, 25, 5
)
