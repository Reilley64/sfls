-- This file should undo anything in `up.sql`
ALTER TABLE media
    DROP COLUMN library_id;
