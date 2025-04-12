-- Your SQL goes here
CREATE TABLE media
(
    id         BIGINT PRIMARY KEY NOT NULL,
    created_at TEXT               NOT NULL,
    updated_at TEXT               NOT NULL,
    type       TEXT               NOT NULL,
    nfo_id     BIGINT,
    path       TEXT               NOT NULL,
    video_file TEXT,
    title      TEXT               NOT NULL,
    season     INTEGER,
    episode    INTEGER,
    attributes TEXT               NOT NULL
);
