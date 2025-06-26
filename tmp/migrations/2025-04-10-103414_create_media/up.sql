-- Your SQL goes here
CREATE TABLE media
(
    id              BIGINT PRIMARY KEY NOT NULL DEFAULT snowflake.nextval(),
    created_at      TIMESTAMP          NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMP          NOT NULL DEFAULT NOW(),
    type            VARCHAR(255)       NOT NULL,
    path            TEXT,
    title           TEXT               NOT NULL,
    season          INTEGER,
    episode         INTEGER,
    files JSONB NOT NULL,
    attributes      JSONB              NOT NULL
);

ALTER TABLE media
    ADD COLUMN parent_id BIGINT REFERENCES media;

-- CREATE UNIQUE INDEX media_path_parent_id ON media (path, parent_id) WHERE path NOT NULL;
