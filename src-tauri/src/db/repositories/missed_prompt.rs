use crate::db::models::MissedPrompt;
use rusqlite::Connection;
use std::sync::MutexGuard;

use super::Result;

pub struct MissedPromptRepository<'a> {
    conn: MutexGuard<'a, Connection>,
}

impl<'a> MissedPromptRepository<'a> {
    pub fn new(conn: MutexGuard<'a, Connection>) -> Self {
        Self { conn }
    }

    pub fn create(&self, timestamp: i64, reason: Option<&str>) -> Result<i64> {
        self.conn.execute(
            "INSERT OR IGNORE INTO missed_prompts (timestamp, reason) VALUES (?1, ?2)",
            (timestamp, reason),
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn find_by_date_range(&self, start: i64, end: i64) -> Result<Vec<MissedPrompt>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, timestamp, reason, created_at
             FROM missed_prompts
             WHERE timestamp >= ?1 AND timestamp < ?2
             ORDER BY timestamp ASC",
        )?;

        let prompts = stmt
            .query_map([start, end], |row| {
                Ok(MissedPrompt {
                    id: row.get(0)?,
                    timestamp: row.get(1)?,
                    reason: row.get(2)?,
                    created_at: row.get(3)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(prompts)
    }

    pub fn delete_by_timestamp(&self, timestamp: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM missed_prompts WHERE timestamp = ?1", [timestamp])?;
        Ok(())
    }
}
