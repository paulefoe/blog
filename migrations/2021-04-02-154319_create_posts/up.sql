CREATE TABLE posts (
    id INTEGER PRIMARY KEY,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 'f',
    created_at timestamp default current_timestamp,
    views_count INTEGER default 0,
    description TEXT NOT NULL
);