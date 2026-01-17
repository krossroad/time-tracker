use crate::db::{Database, Setting};
use tauri::State;
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

#[tauri::command]
pub fn get_setting(db: State<'_, Database>, key: String) -> Result<Option<String>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT value FROM settings WHERE key = ?1")
        .map_err(|e| e.to_string())?;

    let result = stmt
        .query_row([&key], |row| row.get(0))
        .ok();

    Ok(result)
}

#[tauri::command]
pub fn set_setting(db: State<'_, Database>, key: String, value: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
        (&key, &value),
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_all_settings(db: State<'_, Database>) -> Result<Vec<Setting>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT key, value FROM settings")
        .map_err(|e| e.to_string())?;

    let settings = stmt
        .query_map([], |row| {
            Ok(Setting {
                key: row.get(0)?,
                value: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(settings)
}

#[tauri::command]
pub fn test_notification(app: AppHandle, sound: String) -> Result<(), String> {
    // Show notification
    app.notification()
        .builder()
        .title("Test Notification")
        .body("This is a test notification sound")
        .show()
        .map_err(|e| e.to_string())?;

    // Play sound using afplay on macOS (notify_rust sound support is limited on macOS)
    #[cfg(target_os = "macos")]
    {
        let sound_name = if sound == "default" {
            "Ping".to_string()
        } else {
            // Capitalize first letter to match macOS sound file names
            let mut chars = sound.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                None => sound.clone(),
            }
        };
        let sound_path = format!("/System/Library/Sounds/{}.aiff", sound_name);
        std::process::Command::new("afplay")
            .arg(&sound_path)
            .spawn()
            .ok();
    }

    Ok(())
}
