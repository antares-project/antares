-- Add up migration script here

CREATE TABLE attachments (
    id BLOB PRIMARY KEY NOT NULL,
    message_id BLOB NOT NULL,
    file_id BLOB NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (message_id) REFERENCES messages (id) ON DELETE CASCADE,
    FOREIGN KEY (file_id) REFERENCES files (id) ON DELETE CASCADE
) STRICT;