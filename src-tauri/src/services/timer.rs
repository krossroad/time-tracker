use crate::db::{Database, SettingsRepository, TimeEntryRepository};
use crate::services::idle_detector;
use chrono::{Local, Timelike};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_notification::NotificationExt;
use tokio::sync::mpsc;
use tokio::time::{interval_at, Duration as TokioDuration, Instant};

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

    // Create timer aligned to clock boundaries using interval_at
    let seconds_until_next = seconds_until_next_boundary(interval_minutes);
    let start = Instant::now() + TokioDuration::from_secs(seconds_until_next);
    let mut timer = interval_at(start, TokioDuration::from_secs(interval_minutes * 60));

    let mut idle_start: Option<i64> = None;

    loop {
        tokio::select! {
            _ = timer.tick() => {
                let db = app_handle.state::<Database>();

                // Get settings using repository
                let (idle_threshold, notification_enabled, notification_sound) = {
                    let conn = db.conn.lock().unwrap();
                    let settings_repo = SettingsRepository::new(conn);
                    (
                        settings_repo.get_idle_threshold_minutes(),
                        settings_repo.is_notification_enabled(),
                        settings_repo.get_notification_sound(),
                    )
                };

                let is_idle = idle_detector::is_user_idle(idle_threshold);
                let now = Local::now().timestamp();
                // Prompt at END of interval: use previous interval's timestamp
                let aligned_timestamp = align_timestamp(now, interval_minutes as i64) - (interval_minutes as i64 * 60);

                if is_idle {
                    if idle_start.is_none() {
                        idle_start = Some(aligned_timestamp);
                    }
                    // Auto-create away entry using repository
                    let conn = db.conn.lock().unwrap();
                    let time_entry_repo = TimeEntryRepository::new(conn);
                    let _ = time_entry_repo.create_away_entry(aligned_timestamp, interval_minutes as i32);
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
                        let interval_start = chrono::DateTime::from_timestamp(aligned_timestamp, 0)
                            .map(|dt| dt.with_timezone(&Local).format("%-I:%M %p").to_string())
                            .unwrap_or_default();
                        let interval_end = chrono::DateTime::from_timestamp(aligned_timestamp + (interval_minutes as i64 * 60), 0)
                            .map(|dt| dt.with_timezone(&Local).format("%-I:%M %p").to_string())
                            .unwrap_or_default();
                        let notification_body = format!("What did you work on {} - {}?", interval_start, interval_end);

                        let _ = app_handle
                            .notification()
                            .builder()
                            .title("Time Tracker")
                            .body(&notification_body)
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
                        // Realign timer to new interval boundaries
                        let seconds_until_next = seconds_until_next_boundary(interval_minutes);
                        let start = Instant::now() + TokioDuration::from_secs(seconds_until_next);
                        timer = interval_at(start, TokioDuration::from_secs(interval_minutes * 60));
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

/// Calculate seconds until the next aligned interval boundary
fn seconds_until_next_boundary(interval_minutes: u64) -> u64 {
    let now = Local::now();
    let minutes = now.minute();
    let seconds = now.second();
    let minutes_until_next =
        (interval_minutes as u32 - (minutes % interval_minutes as u32)) % interval_minutes as u32;

    if minutes_until_next > 0 {
        (minutes_until_next * 60) as u64 - seconds as u64
    } else if seconds > 0 {
        // At the minute mark but seconds > 0, wait until next full interval
        (interval_minutes * 60) - seconds as u64
    } else {
        0
    }
}
