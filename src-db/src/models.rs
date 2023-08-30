// src-db/src/models.rs

use dirs::config_dir;
use prsqlite::*;
use std::fs;

pub async fn populate(db: &mut Connection) {
    println!("Populating database");
}

pub fn check_empty_sync(db: &mut Connection) -> Result<bool, String> {
    let sql_script = fs::read_to_string("sql/count_themes.sql").expect("Unable to read SQL file");

    let (theme_count, err_msg) = {
        // Nested function to limit the scope of stmt and rows
        fn inner(db: &mut Connection, sql_script: &str) -> (i64, String) {
            let mut theme_count = 0;
            let mut err_msg = String::new();
            let mut stmt = db.prepare(sql_script).unwrap();
            let mut rows = stmt.execute().unwrap();
            if let Some(row) = rows.next_row().unwrap() {
                let columns = row.parse().unwrap();
                let theme_count_value = columns.get(0).to_owned();
                if let Value::Integer(i) = theme_count_value {
                    theme_count = i;
                } else {
                    err_msg = "Unexpected value type, expected an Integer".to_string();
                }
            } else {
                err_msg = "No rows returned by the query".to_string();
            }
            (theme_count, err_msg)
        }

        inner(db, &sql_script)
    }; // stmt and rows are dropped here because inner function scope ends

    if !err_msg.is_empty() {
        return Err(err_msg);
    }

    println!("Theme count: {}", theme_count);
    Ok(theme_count == 0)
}

pub async fn check_empty(db: &mut Connection) -> bool {
    match check_empty_sync(db) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("{}", e);
            false
        }
    }
}

pub async fn db_string() -> String {
    let base_dir = config_dir().expect("Could not find config directory");
    println!("Base dir: {:?}", base_dir);

    let mut db_dir = base_dir.clone();
    db_dir.push("ChartCharm");
    if !db_dir.exists() {
        fs::create_dir_all(&db_dir).expect("Failed to create directory");
    }
    println!("DB dir: {:?}", db_dir);

    let mut db_path = db_dir;
    db_path.push("database.sqlite3");
    let db_string = format!("{}", db_path.to_str().unwrap());
    println!("DB string: {}", db_string);

    if !db_path.exists() {
        fs::File::create(&db_path).expect("Failed to create file");
        println!("DB file created");
    }

    db_string
}
