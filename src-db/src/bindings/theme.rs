use crate::get_connection;
use chartcharm_shared::theme::{Theme, ThemeError};
use prsqlite::{Buffer, Connection, Value};

pub async fn create_table() -> Result<(), ThemeError> {
    let mut db = get_connection()
        .await
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

pub async fn insert_theme(name: &str) -> Result<(), ThemeError> {
    let mut db = get_connection()
        .await
        .map_err(|e| ThemeError::ConnectionError("N/A".to_string(), e.to_string()))?;
    let mut stmt = db
        .prepare("INSERT INTO theme (name) VALUES (?);")
        .map_err(|e| ThemeError::InsertError(e.to_string()))?;

    stmt.execute()
        .map_err(|e| ThemeError::InsertError(e.to_string()))?;
    Ok(())
}

pub async fn query_theme<'a>(id: u8) -> Result<String, ThemeError> {
    let mut db = match get_connection().await {
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

    let rows = match stmt.execute() {
        Ok(rows) => rows,
        Err(e) => {
            println!("Failed to execute theme query statement: {:?}", e);
            return Err(ThemeError::RetrieveError(e.to_string()));
        }
    };

    let row = rows.next_row().unwrap().unwrap();

    let mut columns = match row.parse() {
        Ok(columns) => columns,
        Err(e) => {
            println!("Failed to get theme query statement columns: {:?}", e);
            return Err(ThemeError::RetrieveError(e.to_string()));
        }
    };

    let value_option = columns.get(0); // This is of type Option<&Value>

    if let Some(value) = value_option {
        let buffer: Buffer<'a> = value.force_text_buffer();
        let slice: &[u8] = &buffer;
        match std::str::from_utf8(slice) {
            Ok(s) => Ok(s.to_string()),
            Err(_) => Err(ThemeError::DecodeError),
        }
    } else {
        Err(ThemeError::RetrieveError("No theme found".to_string()))
    }
}
