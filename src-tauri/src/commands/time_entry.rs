use crate::db::{Database, TimeEntry, MissedPrompt};
use tauri::State;

#[tauri::command]
pub fn create_time_entry(
    db: State<'_, Database>,
    timestamp: i64,
    category: String,
    duration_minutes: Option<i32>,
    is_away: Option<bool>,
    is_retroactive: Option<bool>,
    notes: Option<String>,
) -> Result<i64, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO time_entries (timestamp, category, duration_minutes, is_away, is_retroactive, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (
            timestamp,
            &category,
            duration_minutes.unwrap_or(15),
            is_away.unwrap_or(false) as i32,
            is_retroactive.unwrap_or(false) as i32,
            &notes,
        ),
    )
    .map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub fn get_entries_for_date(
    db: State<'_, Database>,
    start_timestamp: i64,
    end_timestamp: i64,
) -> Result<Vec<TimeEntry>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, timestamp, category, duration_minutes, is_away, is_retroactive, notes, created_at
             FROM time_entries
             WHERE timestamp >= ?1 AND timestamp < ?2
             ORDER BY timestamp ASC",
        )
        .map_err(|e| e.to_string())?;

    let entries = stmt
        .query_map([start_timestamp, end_timestamp], |row| {
            Ok(TimeEntry {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                category: row.get(2)?,
                duration_minutes: row.get(3)?,
                is_away: row.get::<_, i32>(4)? != 0,
                is_retroactive: row.get::<_, i32>(5)? != 0,
                notes: row.get(6)?,
                created_at: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(entries)
}

#[tauri::command]
pub fn update_time_entry(
    db: State<'_, Database>,
    id: i64,
    category: Option<String>,
    notes: Option<String>,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    if let Some(cat) = category {
        conn.execute(
            "UPDATE time_entries SET category = ?1 WHERE id = ?2",
            (&cat, id),
        )
        .map_err(|e| e.to_string())?;
    }

    if let Some(n) = notes {
        conn.execute(
            "UPDATE time_entries SET notes = ?1 WHERE id = ?2",
            (&n, id),
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub fn delete_time_entry(db: State<'_, Database>, id: i64) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM time_entries WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn create_missed_prompt(
    db: State<'_, Database>,
    timestamp: i64,
    reason: Option<String>,
) -> Result<i64, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT OR IGNORE INTO missed_prompts (timestamp, reason) VALUES (?1, ?2)",
        (timestamp, &reason),
    )
    .map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub fn get_missed_prompts(
    db: State<'_, Database>,
    start_timestamp: i64,
    end_timestamp: i64,
) -> Result<Vec<MissedPrompt>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, timestamp, reason, created_at
             FROM missed_prompts
             WHERE timestamp >= ?1 AND timestamp < ?2
             ORDER BY timestamp ASC",
        )
        .map_err(|e| e.to_string())?;

    let prompts = stmt
        .query_map([start_timestamp, end_timestamp], |row| {
            Ok(MissedPrompt {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                reason: row.get(2)?,
                created_at: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(prompts)
}

#[tauri::command]
pub fn delete_missed_prompt(db: State<'_, Database>, timestamp: i64) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM missed_prompts WHERE timestamp = ?1", [timestamp])
        .map_err(|e| e.to_string())?;

    Ok(())
}
