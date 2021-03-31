-- Your SQL goes here
CREATE TABLE bloodpressures (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    date TEXT NOT NULL,
    time TEXT NOT NULL,
    systole INTEGER NOT NULL,
    diastole INTEGER NOT NULL,
    pulse INTEGER NOT NULL
)
