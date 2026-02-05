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
    fn test_create_and_find_by_date_range() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = MissedPromptRepository::new(conn);
        repo.create(1000, Some("idle")).unwrap();

        let prompts = repo.find_by_date_range(0, 2000).unwrap();
        assert_eq!(prompts.len(), 1);
        assert_eq!(prompts[0].timestamp, 1000);
        assert_eq!(prompts[0].reason.as_deref(), Some("idle"));
    }

    #[test]
    fn test_duplicate_timestamp_ignored() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = MissedPromptRepository::new(conn);
        repo.create(1000, Some("first")).unwrap();
        repo.create(1000, Some("second")).unwrap();

        let prompts = repo.find_by_date_range(0, 2000).unwrap();
        assert_eq!(prompts.len(), 1);
        assert_eq!(prompts[0].reason.as_deref(), Some("first"));
    }

    #[test]
    fn test_find_by_date_range_empty() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = MissedPromptRepository::new(conn);
        let prompts = repo.find_by_date_range(0, 10000).unwrap();
        assert!(prompts.is_empty());
    }

    #[test]
    fn test_delete_by_timestamp() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = MissedPromptRepository::new(conn);
        repo.create(1000, Some("test")).unwrap();

        repo.delete_by_timestamp(1000).unwrap();

        let prompts = repo.find_by_date_range(0, 2000).unwrap();
        assert!(prompts.is_empty());
    }

    #[test]
    fn test_ordered_by_timestamp() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = MissedPromptRepository::new(conn);
        repo.create(3000, None).unwrap();
        repo.create(1000, None).unwrap();
        repo.create(2000, None).unwrap();

        let prompts = repo.find_by_date_range(0, 5000).unwrap();
        assert_eq!(prompts.len(), 3);
        assert_eq!(prompts[0].timestamp, 1000);
        assert_eq!(prompts[1].timestamp, 2000);
        assert_eq!(prompts[2].timestamp, 3000);
    }
}
