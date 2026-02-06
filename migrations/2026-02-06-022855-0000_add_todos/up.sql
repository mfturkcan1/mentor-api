-- Your SQL goes here

CREATE TABLE IF NOT EXISTS todos(
    id uuid DEFAULT uuidv7() PRIMARY KEY,

    title TEXT NOT NULL,
    description TEXT,

    parent_goal_id  INTEGER,

    completed BOOLEAN DEFAULT FALSE,

    completed_date TIMESTAMPTZ,
    create_date      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    update_date      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    delete_date      TIMESTAMPTZ,

    CONSTRAINT todos_parent_goal_fk
    FOREIGN KEY (parent_goal_id) REFERENCES goals(id) ON DELETE SET NULL
)