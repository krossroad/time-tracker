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
