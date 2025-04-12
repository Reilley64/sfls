-- Your SQL goes here
CREATE TABLE libraries
(
    id         BIGINT PRIMARY KEY NOT NULL DEFAULT snowflake.nextval(),
    created_at TIMESTAMP          NOT NULL DEFAULT NOW(),
    created_by TEXT               NOT NULL,
    updated_at TIMESTAMP          NOT NULL DEFAULT NOW(),
    updated_by TEXT               NOT NULL,
    name       TEXT               NOT NULL,
    path       TEXT               NOT NULL,
    media_type TEXT               NOT NULL
)