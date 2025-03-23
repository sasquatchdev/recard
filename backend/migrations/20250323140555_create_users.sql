-- Add migration script here
CREATE TABLE users (
    uuid TEXT PRIMARY KEY,
    username TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    decks TEXT[] NOT NULL DEFAULT '{}'
);
