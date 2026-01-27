-- This file should undo anything in `up.sql`
ALTER TABLE routines
DROP COLUMN IF EXISTS update_date;

ALTER TABLE routines
DROP COLUMN IF EXISTS delete_date;