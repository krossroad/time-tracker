use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new() -> Result<Self, rusqlite::Error> {
        let db_path = get_db_path();

        // Ensure the directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }

        let conn = Connection::open(&db_path)?;
        Ok(Database {
            conn: Mutex::new(conn),
        })
    }
}

fn get_db_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join("Library")
        .join("Application Support")
        .join("com.timetracker.app")
        .join("time_tracker.db")
}

#[cfg(test)]
impl Database {
    pub fn new_in_memory() -> Result<Self, rusqlite::Error> {
        let conn = Connection::open_in_memory()?;
        Ok(Database {
            conn: Mutex::new(conn),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::migrations;

    #[test]
    fn test_new_in_memory_creates_database() {
        let db = Database::new_in_memory().unwrap();
        let conn = db.conn.lock().unwrap();
        conn.execute_batch("SELECT 1").unwrap();
    }

    #[test]
    fn test_in_memory_database_with_migrations() {
        let db = Database::new_in_memory().unwrap();
        let conn = db.conn.lock().unwrap();
        migrations::run_migrations(&conn).unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM time_entries", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }
}
