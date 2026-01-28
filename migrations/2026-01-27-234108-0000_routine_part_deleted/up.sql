-- Your SQL goes here
ALTER TABLE routine_parts
    ADD COLUMN IF NOT EXISTS delete_date TIMESTAMP;