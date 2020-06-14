CREATE TABLE IF NOT EXISTS users (
    username TEXT PRIMARY KEY NOT NULL,
    password TEXT NOT NULL,
    is_admin INTEGER NOT NULL,
    duel_points INTEGE NOT NULL
);
