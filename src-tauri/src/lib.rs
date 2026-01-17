/// The `tx` variable is a `tokio::sync::mpsc::Sender<TimerCommand>` channel sender,
/// created to allow sending timer commands to a background timer task.
/// It is managed by the Tauri application state via `app.manage(tx);`,
/// making it accessible to other parts of the application (such as command handlers)
/// that may need to communicate with the timer service.
/// The corresponding receiver (`rx`) is moved into the background timer task,
/// which listens for incoming commands and acts accordingly.
///
/// In summary, `tx` is used to send commands to the background timer task from elsewhere in the app.
mod commands;
mod db;
mod services;

use db::{migrations, Database, SettingsRepository};
use services::TimerCommand;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};
use tokio::sync::mpsc;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // Set as menu bar app (no dock icon) on macOS
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            // Initialize database
            let database = Database::new().expect("Failed to initialize database");
            {
                let conn = database.conn.lock().unwrap();
                migrations::run_migrations(&conn).expect("Failed to run migrations");
            }
            app.manage(database);

            // Create tray menu
            let open_item =
                MenuItem::with_id(app, "open", "Open Time Tracker", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open_item, &quit_item])?;

            // Build tray icon
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "open" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            // Handle window close event - hide instead of quit
            if let Some(window) = app.get_webview_window("main") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        // Prevent the window from closing (quitting the app)
                        api.prevent_close();
                        // Hide the window instead
                        let _ = window_clone.hide();
                    }
                });
            }

            // Start background timer
            let app_handle = app.handle().clone();
            let (tx, rx) = mpsc::channel::<TimerCommand>(10);
            app.manage(tx);

            // Get interval from settings using repository
            let db = app.state::<Database>();
            let interval: u64 = {
                let conn = db.conn.lock().unwrap();
                let settings_repo = SettingsRepository::new(conn);
                settings_repo.get_interval_minutes()
            };

            tauri::async_runtime::spawn(async move {
                services::timer::start_timer(app_handle, interval, rx).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::create_time_entry,
            commands::get_entries_for_date,
            commands::update_time_entry,
            commands::delete_time_entry,
            commands::create_missed_prompt,
            commands::get_missed_prompts,
            commands::delete_missed_prompt,
            commands::get_setting,
            commands::set_setting,
            commands::get_all_settings,
            commands::test_notification,
            commands::export_entries_to_csv,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
