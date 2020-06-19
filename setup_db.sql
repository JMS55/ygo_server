CREATE TABLE IF NOT EXISTS users (
    id SERIAL NOT NULL PRIMARY KEY,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    duel_points INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS tournaments (
    id SERIAL NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    rounds INTEGER NOT NULL,
    duel_points_per_win INTEGER NOT NULL,
    duel_points_per_loss INTEGER NOT NULL,
    duel_points_jackpot INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS matches (
    id SERIAL NOT NULL PRIMARY KEY,
    tournament INTEGER NOT NULL REFERENCES tournaments(id),
    round INTEGER NOT NULL,
    duelist1 INTEGER REFERENCES users(id),
    duelist2 INTEGER REFERENCES users(id),
    duelist1_reported_winning BOOLEAN,
    duelist2_reported_winning BOOLEAN
);
