-- Migration: create_comments_table
CREATE TABLE IF NOT EXISTS comments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    post_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL DEFAULT 1,
    comment TEXT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'approved',
    created_at VARCHAR(19),
    updated_at VARCHAR(19)
);

-- +down
DROP TABLE IF EXISTS comments;
