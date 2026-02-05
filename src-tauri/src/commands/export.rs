use crate::db::{Database, TimeEntryRepository};
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
    let repo = TimeEntryRepository::new(conn);

    let entries = repo
        .find_raw_by_date_range(start_timestamp, end_timestamp)
        .map_err(|e| e.to_string())?;

    let mut csv = String::from("Date,Time,Category,Duration (minutes),Is Away,Is Retroactive,Notes\n");

    for (timestamp, category, duration_minutes, is_away, is_retroactive, notes) in entries {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_csv_field_plain() {
        assert_eq!(escape_csv_field("hello"), "hello");
    }

    #[test]
    fn test_escape_csv_field_with_comma() {
        assert_eq!(escape_csv_field("hello,world"), "\"hello,world\"");
    }

    #[test]
    fn test_escape_csv_field_with_quotes() {
        assert_eq!(escape_csv_field("say \"hi\""), "\"say \"\"hi\"\"\"");
    }

    #[test]
    fn test_escape_csv_field_with_newline() {
        assert_eq!(escape_csv_field("line1\nline2"), "\"line1\nline2\"");
    }

    #[test]
    fn test_escape_csv_field_empty() {
        assert_eq!(escape_csv_field(""), "");
    }
}
