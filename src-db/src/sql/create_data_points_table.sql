CREATE TABLE IF NOT EXISTS data_points (
    id INTEGER PRIMARY KEY,
    project INTEGER NOT NULL,
    data REAL NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (project) REFERENCES projects(id)
);
