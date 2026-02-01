-- Your SQL goes here
CREATE TYPE goal_period_type AS ENUM ('MONTH', 'YEAR', 'DEADLINE');

CREATE TYPE goal_status AS ENUM ('PLANNED', 'ACTIVE', 'DONE', 'CANCELED');

CREATE TYPE goal_priority AS ENUM ('LOW', 'MEDIUM', 'HIGH', 'CRITICAL');

CREATE TABLE IF NOT EXISTS goals (
    id          SERIAL PRIMARY KEY,

    title           TEXT NOT NULL,
    description     TEXT NULL,

    period_type     goal_period_type NOT NULL,

    deadline_at     TIMESTAMPTZ NULL,
    period_start    TIMESTAMPTZ NULL,
    period_end      TIMESTAMPTZ NULL,

    status          goal_status NOT NULL DEFAULT 'PLANNED',
    priority        goal_priority NOT NULL DEFAULT 'MEDIUM',

    target_value    INTEGER NULL, -- örn: 20 (kitap), 200 (kg squat), 30 (saat)
    current_value   INTEGER NULL DEFAULT 0,
    unit            TEXT NULL,  -- "kg", "hours", "pages", "sessions"...

    parent_goal_id  BIGINT NULL REFERENCES goals(id) ON DELETE SET NULL,

    create_date      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    update_date      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    delete_date      TIMESTAMPTZ
);