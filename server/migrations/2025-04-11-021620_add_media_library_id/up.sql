-- Your SQL goes here
ALTER TABLE media
    ADD COLUMN library_id BIGINT REFERENCES libraries NOT NULL;
