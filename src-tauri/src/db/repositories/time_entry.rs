use crate::db::models::TimeEntry;
use rusqlite::Connection;
use std::sync::MutexGuard;

use super::{int_to_bool, bool_to_int, Result};

pub struct TimeEntryRepository<'a> {
    conn: MutexGuard<'a, Connection>,
}

impl<'a> TimeEntryRepository<'a> {
    pub fn new(conn: MutexGuard<'a, Connection>) -> Self {
        Self { conn }
    }

    pub fn create(
        &self,
        timestamp: i64,
        category: &str,
        duration_minutes: i32,
        is_away: bool,
        is_retroactive: bool,
        notes: Option<&str>,
    ) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO time_entries (timestamp, category, duration_minutes, is_away, is_retroactive, notes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                timestamp,
                category,
                duration_minutes,
                bool_to_int(is_away),
                bool_to_int(is_retroactive),
                notes,
            ),
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn create_away_entry(&self, timestamp: i64, duration_minutes: i32) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO time_entries (timestamp, category, duration_minutes, is_away) VALUES (?1, 'away', ?2, 1)",
            (timestamp, duration_minutes),
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn find_by_date_range(&self, start: i64, end: i64) -> Result<Vec<TimeEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, timestamp, category, duration_minutes, is_away, is_retroactive, notes, created_at
             FROM time_entries
             WHERE timestamp >= ?1 AND timestamp < ?2
             ORDER BY timestamp ASC",
        )?;

        let entries = stmt
            .query_map([start, end], |row| {
                Ok(TimeEntry {
                    id: row.get(0)?,
                    timestamp: row.get(1)?,
                    category: row.get(2)?,
                    duration_minutes: row.get(3)?,
                    is_away: int_to_bool(row.get(4)?),
                    is_retroactive: int_to_bool(row.get(5)?),
                    notes: row.get(6)?,
                    created_at: row.get(7)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(entries)
    }

    pub fn find_raw_by_date_range(
        &self,
        start: i64,
        end: i64,
    ) -> Result<Vec<(i64, String, i32, bool, bool, Option<String>)>> {
        let mut stmt = self.conn.prepare(
            "SELECT timestamp, category, duration_minutes, is_away, is_retroactive, notes
             FROM time_entries
             WHERE timestamp >= ?1 AND timestamp < ?2
             ORDER BY timestamp ASC",
        )?;

        let entries = stmt
            .query_map([start, end], |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    int_to_bool(row.get(3)?),
                    int_to_bool(row.get(4)?),
                    row.get(5)?,
                ))
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(entries)
    }

    pub fn update_category(&self, id: i64, category: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE time_entries SET category = ?1 WHERE id = ?2",
            (category, id),
        )?;
        Ok(())
    }

    pub fn update_notes(&self, id: i64, notes: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE time_entries SET notes = ?1 WHERE id = ?2",
            (notes, id),
        )?;
        Ok(())
    }

    pub fn delete(&self, id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM time_entries WHERE id = ?1", [id])?;
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
    fn test_create_returns_id() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = TimeEntryRepository::new(conn);
        let id = repo.create(1000, "deep_work", 15, false, false, Some("test")).unwrap();
        assert!(id > 0);
    }

    #[test]
    fn test_create_and_find_by_date_range() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = TimeEntryRepository::new(conn);
        repo.create(1000, "deep_work", 15, false, true, Some("coding")).unwrap();

        let entries = repo.find_by_date_range(0, 2000).unwrap();
        assert_eq!(entries.len(), 1);
        let e = &entries[0];
        assert_eq!(e.timestamp, 1000);
        assert_eq!(e.category, "deep_work");
        assert_eq!(e.duration_minutes, 15);
        assert!(!e.is_away);
        assert!(e.is_retroactive);
        assert_eq!(e.notes.as_deref(), Some("coding"));
    }

    #[test]
    fn test_find_by_date_range_excludes_out_of_range() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = TimeEntryRepository::new(conn);
        repo.create(500, "admin", 15, false, false, None).unwrap();
        repo.create(1500, "meetings", 15, false, false, None).unwrap();
        repo.create(2500, "break", 15, false, false, None).unwrap();

        let entries = repo.find_by_date_range(1000, 2000).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].timestamp, 1500);
    }

    #[test]
    fn test_find_by_date_range_empty() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = TimeEntryRepository::new(conn);
        let entries = repo.find_by_date_range(0, 10000).unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn test_create_away_entry() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = TimeEntryRepository::new(conn);
        repo.create_away_entry(1000, 15).unwrap();

        let entries = repo.find_by_date_range(0, 2000).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].category, "away");
        assert!(entries[0].is_away);
        assert_eq!(entries[0].duration_minutes, 15);
    }

    #[test]
    fn test_find_raw_by_date_range() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = TimeEntryRepository::new(conn);
        repo.create(1000, "deep_work", 15, false, true, Some("raw test")).unwrap();

        let entries = repo.find_raw_by_date_range(0, 2000).unwrap();
        assert_eq!(entries.len(), 1);
        let (ts, cat, dur, away, retro, notes) = &entries[0];
        assert_eq!(*ts, 1000);
        assert_eq!(cat, "deep_work");
        assert_eq!(*dur, 15);
        assert!(!away);
        assert!(retro);
        assert_eq!(notes.as_deref(), Some("raw test"));
    }

    #[test]
    fn test_update_category() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = TimeEntryRepository::new(conn);
        let id = repo.create(1000, "deep_work", 15, false, false, None).unwrap();

        repo.update_category(id, "meetings").unwrap();

        let entries = repo.find_by_date_range(0, 2000).unwrap();
        assert_eq!(entries[0].category, "meetings");
    }

    #[test]
    fn test_update_notes() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = TimeEntryRepository::new(conn);
        let id = repo.create(1000, "deep_work", 15, false, false, None).unwrap();

        repo.update_notes(id, "updated notes").unwrap();

        let entries = repo.find_by_date_range(0, 2000).unwrap();
        assert_eq!(entries[0].notes.as_deref(), Some("updated notes"));
    }

    #[test]
    fn test_delete() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = TimeEntryRepository::new(conn);
        let id = repo.create(1000, "deep_work", 15, false, false, None).unwrap();

        repo.delete(id).unwrap();

        let entries = repo.find_by_date_range(0, 2000).unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn test_multiple_entries_ordered_by_timestamp() {
        let db = setup_db();
        let conn = db.conn.lock().unwrap();
        let repo = TimeEntryRepository::new(conn);
        repo.create(3000, "break", 15, false, false, None).unwrap();
        repo.create(1000, "deep_work", 15, false, false, None).unwrap();
        repo.create(2000, "meetings", 15, false, false, None).unwrap();

        let entries = repo.find_by_date_range(0, 5000).unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].timestamp, 1000);
        assert_eq!(entries[1].timestamp, 2000);
        assert_eq!(entries[2].timestamp, 3000);
    }
}
