-- Add up migration script here

CREATE TABLE files (
    id BLOB PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    hash BLOB UNIQUE NOT NULL,
    size INTEGER NOT NULL,
    mime_type TEXT NOT NULL,
    counter INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
) STRICT;