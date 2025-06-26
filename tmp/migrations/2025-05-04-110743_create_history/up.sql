-- Your SQL goes here
CREATE TABLE history
(
    id         BIGINT PRIMARY KEY           NOT NULL DEFAULT snowflake.nextval(),
    created_at TIMESTAMP                    NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP                    NOT NULL DEFAULT NOW(),
    media_id   BIGINT REFERENCES media (id) NOT NULL,
    user_id    BIGINT REFERENCES users (id) NOT NULL,
    position   BIGINT                       NOT NULL
)