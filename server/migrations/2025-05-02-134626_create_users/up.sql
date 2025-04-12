-- Your SQL goes here
CREATE TABLE users
(
    id         BIGINT PRIMARY KEY NOT NULL DEFAULT snowflake.nextval(),
    created_at TIMESTAMP          NOT NULL DEFAULT NOW(),
    created_by TEXT               NOT NULL,
    updated_at TIMESTAMP          NOT NULL DEFAULT NOW(),
    updated_by TEXT               NOT NULL,
    email      VARCHAR(255)       NOT NULL UNIQUE,
    password   VARCHAR(255)       NOT NULL,
    name       VARCHAR(255)       NOT NULL,
    is_admin   BOOLEAN            NOT NULL
);
