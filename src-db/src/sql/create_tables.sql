-- Create Theme table
CREATE TABLE IF NOT EXISTS Theme (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL
);

-- Create Settings table
CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    theme_selected INTEGER,
    FOREIGN KEY (theme_selected) REFERENCES Theme(id)
);

-- Create Project table
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create DataPoint table
CREATE TABLE IF NOT EXISTS data_points (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project INTEGER,
    data REAL NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project) REFERENCES projects(id)
);
