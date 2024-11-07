CREATE TABLE kind (
    kind_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name    TEXT NOT NULL UNIQUE
);

CREATE TABLE producer (
    producer_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE beverage (
    beverage_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name        TEXT NOT NULL,
    kind_id     INTEGER NOT NULL,
    producer_id INTEGER NOT NULL,
    rating      REAL NOT NULL,
    description TEXT NOT NULL,
    FOREIGN KEY (kind_id) REFERENCES kind (kind_id),
    FOREIGN KEY (producer_id) REFERENCES producer (producer_id)
);
