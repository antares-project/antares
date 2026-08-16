-- Add up migration script here

CREATE TABLE profiles (
    id BLOB PRIMARY KEY NOT NULL,
    public_key BLOB UNIQUE NOT NULL,
    name TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_at TEXT NOT NULL
) STRICT;