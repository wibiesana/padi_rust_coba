-- Migration: create_posts_table
CREATE TABLE IF NOT EXISTS posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL DEFAULT 1,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255),
    content TEXT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'published',
    created_at VARCHAR(19),
    updated_at VARCHAR(19)
);

-- +down
DROP TABLE IF EXISTS posts;
