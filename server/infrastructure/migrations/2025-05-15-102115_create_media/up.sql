-- Your SQL goes here
CREATE TABLE media
(
    id         BIGINT PRIMARY KEY          NOT NULL DEFAULT snowflake.nextval(),
    created_at TIMESTAMP                   NOT NULL DEFAULT now(),
    updated_at TIMESTAMP                   NOT NULL DEFAULT now(),
    library_id BIGINT REFERENCES libraries NOT NULL,
    type       VARCHAR(255)                NOT NULL,
    path       TEXT,
    title      TEXT                        NOT NULL,
    season     INTEGER,
    episode    INTEGER,
    files      JSONB                       NOT NULL,
    attributes JSONB                       NOT NULL,
    parent_id  BIGINT REFERENCES media
);

-- CREATE UNIQUE INDEX media_path_parent_id ON media (path, parent_id) WHERE path NOT NULL;
