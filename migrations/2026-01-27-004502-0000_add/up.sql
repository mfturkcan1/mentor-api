-- Your SQL goes here
ALTER TABLE routines
    ADD COLUMN IF NOT EXISTS update_date TIMESTAMP;

ALTER TABLE routines
    ADD COLUMN IF NOT EXISTS delete_date TIMESTAMP;