-- Your SQL goes here
ALTER TABLE media ADD COLUMN parent_id BIGINT REFERENCES media;
