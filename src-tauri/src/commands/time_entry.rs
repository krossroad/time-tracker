use crate::db::{Database, MissedPrompt, MissedPromptRepository, TimeEntry, TimeEntryRepository};
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
    let repo = TimeEntryRepository::new(conn);

    repo.create(
        timestamp,
        &category,
        duration_minutes.unwrap_or(15),
        is_away.unwrap_or(false),
        is_retroactive.unwrap_or(false),
        notes.as_deref(),
    )
    .map_err(Into::into)
}

#[tauri::command]
pub fn get_entries_for_date(
    db: State<'_, Database>,
    start_timestamp: i64,
    end_timestamp: i64,
) -> Result<Vec<TimeEntry>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let repo = TimeEntryRepository::new(conn);

    repo.find_by_date_range(start_timestamp, end_timestamp)
        .map_err(Into::into)
}

#[tauri::command]
pub fn update_time_entry(
    db: State<'_, Database>,
    id: i64,
    category: Option<String>,
    notes: Option<String>,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let repo = TimeEntryRepository::new(conn);

    if let Some(cat) = category {
        repo.update_category(id, &cat).map_err(|e| e.to_string())?;
    }

    if let Some(n) = notes {
        repo.update_notes(id, &n).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub fn delete_time_entry(db: State<'_, Database>, id: i64) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let repo = TimeEntryRepository::new(conn);

    repo.delete(id).map_err(Into::into)
}

#[tauri::command]
pub fn create_missed_prompt(
    db: State<'_, Database>,
    timestamp: i64,
    reason: Option<String>,
) -> Result<i64, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let repo = MissedPromptRepository::new(conn);

    repo.create(timestamp, reason.as_deref()).map_err(Into::into)
}

#[tauri::command]
pub fn get_missed_prompts(
    db: State<'_, Database>,
    start_timestamp: i64,
    end_timestamp: i64,
) -> Result<Vec<MissedPrompt>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let repo = MissedPromptRepository::new(conn);

    repo.find_by_date_range(start_timestamp, end_timestamp)
        .map_err(Into::into)
}

#[tauri::command]
pub fn delete_missed_prompt(db: State<'_, Database>, timestamp: i64) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let repo = MissedPromptRepository::new(conn);

    repo.delete_by_timestamp(timestamp).map_err(Into::into)
}
