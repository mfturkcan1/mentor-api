-- Your SQL goes here
CREATE TABLE routine_parts
(
    id          SERIAL PRIMARY KEY,
    start_hour  TIMESTAMP NOT NULL,
    end_hour    TIMESTAMP NOT NULL,
    description TEXT      NOT NULL,
    routine_id  INTEGER   NOT NULL REFERENCES routines (id)
);