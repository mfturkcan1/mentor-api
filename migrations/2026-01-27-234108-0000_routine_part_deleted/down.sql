-- This file should undo anything in `up.sql`
ALTER TABLE routine_parts
DROP COLUMN IF EXISTS delete_date;