use chartcharm_database::get_connection;
use chartcharm_shared::theme::ThemeError;

pub fn populate_theme_table() -> Result<(), ThemeError> {
    let mut db = get_connection()
        .map_err(|e| ThemeError::ConnectionError("N/A".to_string(), e.to_string()))?;
    let mut stmt = db
        .prepare("INSERT INTO theme (name) VALUES ('auto'), ('light'), ('dark');")
        .map_err(|e| ThemeError::InsertError(e.to_string()))?;

    stmt.execute()
        .map_err(|e| ThemeError::InsertError(e.to_string()))?;
    Ok(())
}

pub fn create_theme_table() -> Result<(), ThemeError> {
    let mut db = get_connection()
        .map_err(|e| ThemeError::ConnectionError("N/A".to_string(), e.to_string()))?;
    let create_table_sql =
        "CREATE TABLE IF NOT EXISTS theme (id INTEGER PRIMARY KEY, name TEXT NOT NULL);";
    let mut stmt = db
        .prepare(create_table_sql)
        .map_err(|e| ThemeError::CreateTableError(e.to_string()))?;

    stmt.execute()
        .map_err(|e| ThemeError::CreateTableError(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn insert_theme(name: &str) -> Result<(), ThemeError> {
    let mut db = get_connection()
        .map_err(|e| ThemeError::ConnectionError("N/A".to_string(), e.to_string()))?;
    let mut stmt = db
        .prepare(&format!("INSERT INTO theme (name) VALUES ({});", name))
        .map_err(|e| ThemeError::InsertError(e.to_string()))?;

    stmt.execute()
        .map_err(|e| ThemeError::InsertError(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn query_theme(id: u8) -> Result<String, ThemeError> {
    let mut db = match get_connection() {
        Ok(db) => db,
        Err(e) => {
            println!("Failed to get database connection: {:?}", e);
            return Err(ThemeError::ConnectionError(id.to_string(), e.to_string()));
        }
    };

    let mut stmt = match db.prepare(&format!("SELECT name FROM theme WHERE id = {};", id)) {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("Failed to create theme query statement: {:?}", e);
            return Err(ThemeError::RetrieveError(e.to_string()));
        }
    };

    let mut rows = match stmt.execute() {
        Ok(rows) => rows,
        Err(e) => {
            println!("Failed to execute theme query statement: {:?}", e);
            return Err(ThemeError::RetrieveError(e.to_string()));
        }
    };

    let row = rows.next_row().unwrap().unwrap();

    let columns = match row.parse() {
        Ok(columns) => columns,
        Err(e) => {
            println!("Failed to get theme query statement columns: {:?}", e);
            return Err(ThemeError::RetrieveError(e.to_string()));
        }
    };

    let value_option = columns.get(0); // This is of type Option<&Value>

    match value_option.as_string() {
        Some(s) => Ok(s),
        None => Err(ThemeError::DecodeError),
    }
}
