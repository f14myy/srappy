pub mod error;
pub mod models;
pub mod extractor;
pub mod fetcher;
pub mod crawler;
pub mod commands;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use models::AppState;
use commands::*;
use tauri::{Manager, menu::{Menu, MenuItem}, tray::{TrayIconBuilder, TrayIconEvent}};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = Arc::new(AppState {
        stop_flag: Arc::new(AtomicBool::new(false)),
        close_to_tray: Arc::new(AtomicBool::new(false)),
        active_browsers: Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
        browser_id_counter: Arc::new(std::sync::atomic::AtomicU64::new(0)),
    });

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, Some(vec!["--minimized"])))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
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
                    if let TrayIconEvent::Click { .. } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let state = window.state::<Arc<AppState>>();
                if state.close_to_tray.load(Ordering::SeqCst) {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            scrape_url,
            save_text,
            read_text,
            stop_scraping,
            resume_scraping,
            load_session,
            get_app_dir,
            set_close_to_tray
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
