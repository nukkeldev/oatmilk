CREATE TABLE IF NOT EXISTS Songs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    parent_type INTEGER NOT NULL DEFAULT 0,
    parent_id INTEGER,
    author INTEGER NOT NULL DEFAULT -1,
    duration FLOAT,
    tags JSON DEFAULT "[]",
    description TEXT
);