-- Your SQL goes here
CREATE TABLE label (
    id serial8 NOT NULL PRIMARY KEY,
    name text NOT NULL UNIQUE,
    color text NOT NULL
);
