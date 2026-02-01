-- Your SQL goes here
DO $$
BEGIN
CREATE TYPE goal_life_cycle AS ENUM ('SHORT_TERM', 'MEDIUM_TERM', 'LONG_TERM', 'LIFE_TIME');
EXCEPTION
  WHEN duplicate_object THEN
    NULL;
END$$;

ALTER TABLE goals ADD COLUMN IF NOT EXISTS goal_cycle goal_life_cycle NOT NULL DEFAULT 'MEDIUM_TERM';
