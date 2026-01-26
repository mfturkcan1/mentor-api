-- Your SQL goes here
CREATE TABLE routines
(
    id          SERIAL PRIMARY KEY,
    title       TEXT      NOT NULL,
    create_date TIMESTAMP NOT NULL
);