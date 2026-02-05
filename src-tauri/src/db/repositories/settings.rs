use crate::db::models::Setting;
use rusqlite::Connection;
use std::sync::MutexGuard;

use super::Result;

pub struct SettingsRepository<'a> {
    conn: MutexGuard<'a, Connection>,
}

impl<'a> SettingsRepository<'a> {
    pub fn new(conn: MutexGuard<'a, Connection>) -> Self {
        Self { conn }
    }

    pub fn get(&self, key: &str) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
        let result = stmt.query_row([key], |row| row.get(0)).ok();
        Ok(result)
    }

    pub fn set(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            (key, value),
        )?;
        Ok(())
    }

    pub fn get_all(&self) -> Result<Vec<Setting>> {
        let mut stmt = self.conn.prepare("SELECT key, value FROM settings")?;

        let settings = stmt
            .query_map([], |row| {
                Ok(Setting {
                    key: row.get(0)?,
                    value: row.get(1)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(settings)
    }

    /// Get interval_minutes setting, defaults to 15
    pub fn get_interval_minutes(&self) -> u64 {
        self.get("interval_minutes")
            .ok()
            .flatten()
            .and_then(|v| v.parse().ok())
            .unwrap_or(15)
    }

    /// Get idle_threshold_minutes setting, defaults to 5
    pub fn get_idle_threshold_minutes(&self) -> u32 {
        self.get("idle_threshold_minutes")
            .ok()
            .flatten()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5)
    }

    /// Get notification_enabled setting, defaults to true
    pub fn is_notification_enabled(&self) -> bool {
        self.get("notification_enabled")
            .ok()
            .flatten()
            .map(|v| v == "true")
            .unwrap_or(true)
    }

    /// Get notification_sound setting, defaults to "default"
    pub fn get_notification_sound(&self) -> String {
        self.get("notification_sound")
            .ok()
            .flatten()
            .unwrap_or_else(|| "default".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{connection::Database, migrations};

    fn setup_db() -> Database {
        let db = Database::new_in_memory().unwrap();
        {
            let conn = db.conn.lock().unwrap();
            migrations::run_migrations(&conn).unwrap();
        }
        db
    }

    #[test]
    fn test_default_interval_minutes() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = SettingsRepository::new(conn);
        assert_eq!(repo.get_interval_minutes(), 15);
    }

    #[test]
    fn test_default_idle_threshold() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = SettingsRepository::new(conn);
        assert_eq!(repo.get_idle_threshold_minutes(), 5);
    }

    #[test]
    fn test_default_notification_enabled() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = SettingsRepository::new(conn);
        assert!(repo.is_notification_enabled());
    }

    #[test]
    fn test_default_notification_sound() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = SettingsRepository::new(conn);
        assert_eq!(repo.get_notification_sound(), "default");
    }

    #[test]
    fn test_set_and_get() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = SettingsRepository::new(conn);
        repo.set("interval_minutes", "30").unwrap();
        assert_eq!(repo.get("interval_minutes").unwrap(), Some("30".to_string()));
    }

    #[test]
    fn test_get_nonexistent_key() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = SettingsRepository::new(conn);
        assert_eq!(repo.get("nonexistent").unwrap(), None);
    }

    #[test]
    fn test_get_all_returns_defaults() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = SettingsRepository::new(conn);
        let all = repo.get_all().unwrap();
        assert_eq!(all.len(), 4);
        let keys: Vec<&str> = all.iter().map(|s| s.key.as_str()).collect();
        assert!(keys.contains(&"interval_minutes"));
        assert!(keys.contains(&"idle_threshold_minutes"));
        assert!(keys.contains(&"notification_enabled"));
        assert!(keys.contains(&"notification_sound"));
    }
}
