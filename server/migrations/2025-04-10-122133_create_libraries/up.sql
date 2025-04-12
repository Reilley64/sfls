-- Your SQL goes here
CREATE TABLE libraries
(
    id         BIGINT PRIMARY KEY NOT NULL,
    created_at TEXT               NOT NULL,
    updated_at TEXT               NOT NULL,
    name       TEXT               NOT NULL,
    path       TEXT               NOT NULL,
    media_type TEXT               NOT NULL
)