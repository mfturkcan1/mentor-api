-- This file should undo anything in `up.sql`
ALTER TABLE goals ALTER COLUMN parent_goal_id TYPE BIGINT;
