CREATE TABLE IF NOT EXISTS charts (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    project INTEGER NOT NULL,
    kind INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (project) REFERENCES projects(id),
    FOREIGN KEY (kind) REFERENCES chart_kind(id)
);