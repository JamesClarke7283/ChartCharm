CREATE TABLE settings IF NOT EXISTS(
    id INTEGER PRIMARY KEY,
    theme_selected INTEGER NOT NULL,
    FOREIGN KEY (theme_selected) REFERENCES theme (id)
);
