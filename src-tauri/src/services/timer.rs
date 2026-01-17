use crate::db::Database;
use crate::services::idle_detector;
use chrono::{Local, Timelike};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_notification::NotificationExt;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration as TokioDuration};

pub enum TimerCommand {
    UpdateInterval(u64),
    Stop,
}

pub async fn start_timer(
    app_handle: AppHandle,
    initial_interval_minutes: u64,
    mut rx: mpsc::Receiver<TimerCommand>,
) {
    let mut interval_minutes = initial_interval_minutes;
    let mut timer = interval(TokioDuration::from_secs(interval_minutes * 60));

    // Calculate time until next aligned interval
    let now = Local::now();
    let minutes = now.minute();
    let minutes_until_next =
        (interval_minutes as u32 - (minutes % interval_minutes as u32)) % interval_minutes as u32;
    if minutes_until_next > 0 {
        tokio::time::sleep(TokioDuration::from_secs((minutes_until_next * 60) as u64)).await;
    }

    let mut idle_start: Option<i64> = None;

    loop {
        tokio::select! {
            _ = timer.tick() => {
                let db = app_handle.state::<Database>();
                let conn = db.conn.lock().unwrap();

                // Get settings
                let idle_threshold: u32 = conn
                    .query_row(
                        "SELECT value FROM settings WHERE key = 'idle_threshold_minutes'",
                        [],
                        |row| row.get::<_, String>(0),
                    )
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()
                    .unwrap_or(5);

                let notification_enabled = conn
                    .query_row(
                        "SELECT value FROM settings WHERE key = 'notification_enabled'",
                        [],
                        |row| row.get::<_, String>(0),
                    )
                    .unwrap_or_else(|_| "true".to_string())
                    == "true";

                let notification_sound = conn
                    .query_row(
                        "SELECT value FROM settings WHERE key = 'notification_sound'",
                        [],
                        |row| row.get::<_, String>(0),
                    )
                    .unwrap_or_else(|_| "default".to_string());

                drop(conn);

                let is_idle = idle_detector::is_user_idle(idle_threshold);
                let now = Local::now().timestamp();
                // Prompt at END of interval: use previous interval's timestamp
                let aligned_timestamp = align_timestamp(now, interval_minutes as i64) - (interval_minutes as i64 * 60);

                if is_idle {
                    if idle_start.is_none() {
                        idle_start = Some(aligned_timestamp);
                    }
                    // Auto-create away entry
                    let conn = db.conn.lock().unwrap();
                    let _ = conn.execute(
                        "INSERT INTO time_entries (timestamp, category, duration_minutes, is_away) VALUES (?1, 'away', ?2, 1)",
                        (aligned_timestamp, interval_minutes as i32),
                    );
                } else {
                    // User is active
                    if let Some(away_start) = idle_start {
                        // User returned from being away
                        idle_start = None;

                        // Emit event to prompt for missed time
                        let _ = app_handle.emit("return-from-away", serde_json::json!({
                            "away_start": away_start,
                            "away_end": aligned_timestamp,
                        }));
                    }

                    // Send notification if enabled
                    if notification_enabled {
                        let _ = app_handle
                            .notification()
                            .builder()
                            .title("Time Tracker")
                            .body("What you worked in last session?")
                            .show();

                        // Play sound using afplay on macOS (notify_rust sound support is limited)
                        #[cfg(target_os = "macos")]
                        {
                            let sound_name = if notification_sound == "default" {
                                "Ping".to_string()
                            } else {
                                // Capitalize first letter to match macOS sound file names
                                let mut chars = notification_sound.chars();
                                match chars.next() {
                                    Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                                    None => notification_sound.clone(),
                                }
                            };
                            let sound_path = format!("/System/Library/Sounds/{}.aiff", sound_name);
                            let _ = std::process::Command::new("afplay")
                                .arg(&sound_path)
                                .spawn();
                        }
                    }

                    // Emit prompt event
                    let _ = app_handle.emit("prompt-time-entry", serde_json::json!({
                        "timestamp": aligned_timestamp,
                    }));

                    // Show the window
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
            Some(cmd) = rx.recv() => {
                match cmd {
                    TimerCommand::UpdateInterval(new_interval) => {
                        interval_minutes = new_interval;
                        timer = interval(TokioDuration::from_secs(interval_minutes * 60));
                    }
                    TimerCommand::Stop => {
                        break;
                    }
                }
            }
        }
    }
}

fn align_timestamp(timestamp: i64, interval_minutes: i64) -> i64 {
    let interval_seconds = interval_minutes * 60;
    (timestamp / interval_seconds) * interval_seconds
}
