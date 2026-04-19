pub mod error;
pub mod models;
pub mod extractor;
pub mod fetcher;
pub mod crawler;
pub mod commands;

use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use models::AppState;
use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = Arc::new(AppState {
        stop_flag: Arc::new(AtomicBool::new(false)),
    });

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            scrape_url,
            save_text,
            read_text,
            stop_scraping,
            resume_scraping,
            load_session,
            get_app_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
