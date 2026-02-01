-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS goals;

DROP TYPE IF EXISTS goal_period_type;

DROP TYPE IF EXISTS goal_status;

DROP TYPE IF EXISTS goal_priority;

DROP TABLE IF NOT EXISTS goals;