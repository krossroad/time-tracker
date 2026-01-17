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
