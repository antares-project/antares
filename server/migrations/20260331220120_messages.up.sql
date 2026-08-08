-- Add up migration script here

CREATE TABLE messages (
    id BLOB PRIMARY KEY NOT NULL,
    channel_id BLOB NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (channel_id) REFERENCES channels (id) ON DELETE CASCADE
) STRICT;