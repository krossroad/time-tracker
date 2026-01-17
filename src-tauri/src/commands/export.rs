use crate::db::Database;
use chrono::{DateTime, Utc};
use tauri::State;

/// Escapes a string for CSV format (handles quotes and commas)
fn escape_csv_field(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

#[tauri::command]
pub fn export_entries_to_csv(
    db: State<'_, Database>,
    start_timestamp: i64,
    end_timestamp: i64,
) -> Result<String, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT timestamp, category, duration_minutes, is_away, is_retroactive, notes
             FROM time_entries
             WHERE timestamp >= ?1 AND timestamp < ?2
             ORDER BY timestamp ASC",
        )
        .map_err(|e| e.to_string())?;

    let mut csv = String::from("Date,Time,Category,Duration (minutes),Is Away,Is Retroactive,Notes\n");

    let entries = stmt
        .query_map([start_timestamp, end_timestamp], |row| {
            let timestamp: i64 = row.get(0)?;
            let category: String = row.get(1)?;
            let duration_minutes: i32 = row.get(2)?;
            let is_away: bool = row.get::<_, i32>(3)? != 0;
            let is_retroactive: bool = row.get::<_, i32>(4)? != 0;
            let notes: Option<String> = row.get(5)?;

            Ok((timestamp, category, duration_minutes, is_away, is_retroactive, notes))
        })
        .map_err(|e| e.to_string())?;

    for entry in entries {
        let (timestamp, category, duration_minutes, is_away, is_retroactive, notes) =
            entry.map_err(|e| e.to_string())?;

        // Convert timestamp to date and time strings
        let datetime = DateTime::<Utc>::from_timestamp(timestamp, 0)
            .ok_or("Invalid timestamp")?;
        let date = datetime.format("%Y-%m-%d").to_string();
        let time = datetime.format("%H:%M").to_string();

        let notes_escaped = escape_csv_field(&notes.unwrap_or_default());

        csv.push_str(&format!(
            "{},{},{},{},{},{},{}\n",
            date,
            time,
            category,
            duration_minutes,
            is_away,
            is_retroactive,
            notes_escaped
        ));
    }

    Ok(csv)
}
