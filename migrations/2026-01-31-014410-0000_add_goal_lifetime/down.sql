-- This file should undo anything in `up.sql`

ALTER TABLE goals
DROP COLUMN IF EXISTS goal_cycle;