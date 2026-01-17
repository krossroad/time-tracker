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
