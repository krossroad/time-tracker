use rusqlite::Connection;

pub fn run_migrations(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS time_entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp INTEGER NOT NULL,
            category TEXT NOT NULL,
            duration_minutes INTEGER DEFAULT 15,
            is_away INTEGER DEFAULT 0,
            is_retroactive INTEGER DEFAULT 0,
            notes TEXT,
            created_at INTEGER DEFAULT (strftime('%s', 'now'))
        );

        CREATE INDEX IF NOT EXISTS idx_time_entries_timestamp ON time_entries(timestamp);

        CREATE TABLE IF NOT EXISTS missed_prompts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp INTEGER NOT NULL UNIQUE,
            reason TEXT,
            created_at INTEGER DEFAULT (strftime('%s', 'now'))
        );

        CREATE INDEX IF NOT EXISTS idx_missed_prompts_timestamp ON missed_prompts(timestamp);

        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        -- Insert default settings if they don't exist
        INSERT OR IGNORE INTO settings (key, value) VALUES ('interval_minutes', '15');
        INSERT OR IGNORE INTO settings (key, value) VALUES ('idle_threshold_minutes', '5');
        INSERT OR IGNORE INTO settings (key, value) VALUES ('notification_enabled', 'true');
        INSERT OR IGNORE INTO settings (key, value) VALUES ('notification_sound', 'default');
        ",
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_conn() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();
        conn
    }

    #[test]
    fn test_creates_all_tables() {
        let conn = setup_conn();
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert!(tables.contains(&"time_entries".to_string()));
        assert!(tables.contains(&"missed_prompts".to_string()));
        assert!(tables.contains(&"settings".to_string()));
    }

    #[test]
    fn test_idempotent() {
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();
        run_migrations(&conn).unwrap();
    }

    #[test]
    fn test_default_settings() {
        let conn = setup_conn();
        let get = |key: &str| -> String {
            conn.query_row(
                "SELECT value FROM settings WHERE key = ?1",
                [key],
                |row| row.get(0),
            )
            .unwrap()
        };
        assert_eq!(get("interval_minutes"), "15");
        assert_eq!(get("idle_threshold_minutes"), "5");
        assert_eq!(get("notification_enabled"), "true");
        assert_eq!(get("notification_sound"), "default");
    }

    #[test]
    fn test_creates_indexes() {
        let conn = setup_conn();
        let indexes: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='index'")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert!(indexes.contains(&"idx_time_entries_timestamp".to_string()));
        assert!(indexes.contains(&"idx_missed_prompts_timestamp".to_string()));
    }
}
