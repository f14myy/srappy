use std::fs;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Instant;
use tauri::Emitter;

use crate::error::ScrapeError;
use crate::models::{PageResult, ScrapeOptions, ScrapeResult, CrawlState, ProgressEvent, LogEvent, AppState};
use crate::fetcher::{fetch_page, build_clients};
use crate::crawler::crawl;

#[tauri::command]
pub async fn scrape_url(
    app: tauri::AppHandle,
    state: tauri::State<'_, Arc<AppState>>,
    url: String,
    options: ScrapeOptions,
) -> Result<ScrapeResult, ScrapeError> {
    state.stop_flag.store(false, Ordering::SeqCst);
    let timer = Instant::now();
    let clients = build_clients(&options)?;

    if options.recursive {
        crawl(
            app,
            state.stop_flag.clone(),
            state.active_browsers.clone(),
            state.browser_id_counter.clone(),
            clients,
            url,
            options,
            timer,
            None,
        )
        .await
    } else {
        let _ = app.emit(
            "scrape-progress",
            ProgressEvent {
                current_url: url.clone(),
                pages_done: 0,
                pages_found: 1,
                total_pages: if options.recursive { options.max_pages } else { 1 },
                pages_failed: 0,
                depth: 0,
            },
        );

        let t_start = Instant::now();
        let client = {
            let mut rng = rand::thread_rng();
            use rand::Rng;
            let idx = rng.gen_range(0..clients.len());
            clients[idx].clone()
        };
        let res = fetch_page(
            &client,
            &url,
            &options,
            state.stop_flag.clone(),
            state.active_browsers.clone(),
            state.browser_id_counter.clone(),
        )
        .await;
        let elapsed = t_start.elapsed().as_millis();
        
        let (page, _) = match res {
            Ok(r) => {
                let _ = app.emit("scrape-log", LogEvent { url: url.clone(), status: "ok".into(), time_ms: elapsed, message: None });
                r
            }
            Err(e) => {
                let _ = app.emit("scrape-log", LogEvent { url: url.clone(), status: "error".into(), time_ms: elapsed, message: Some(e.to_string()) });
                return Err(e);
            }
        };

        let _ = app.emit(
            "scrape-progress",
            ProgressEvent {
                current_url: String::new(),
                pages_done: 1,
                pages_found: 1,
                total_pages: 1,
                pages_failed: 0,
                depth: 0,
            },
        );

        let total_chars = page.char_count;
        let total_size = page.size_bytes;
        Ok(ScrapeResult {
            load_time_ms: timer.elapsed().as_millis(),
            pages_scraped: 1,
            pages: vec![page],
            total_chars,
            total_size_bytes: total_size,
            pages_failed: 0,
            crawl_state: None,
        })
    }
}

#[tauri::command]
pub fn stop_scraping(state: tauri::State<'_, Arc<AppState>>) {
    state.stop_flag.store(true, Ordering::SeqCst);
    if let Ok(mut browsers) = state.active_browsers.lock() {
        browsers.clear();
    }
}

#[tauri::command]
pub async fn resume_scraping(
    app: tauri::AppHandle,
    state: tauri::State<'_, Arc<AppState>>,
    options: ScrapeOptions,
    mut crawl_state: CrawlState,
    prior_pages: Option<Vec<PageResult>>,
) -> Result<ScrapeResult, ScrapeError> {
    if crawl_state.results.is_empty() {
        if let Some(pages) = prior_pages.filter(|p| !p.is_empty()) {
            crawl_state.results = pages.clone();
            for p in pages {
                crawl_state.visited.insert(p.url.clone());
            }
        }
    }
    state.stop_flag.store(false, Ordering::SeqCst);
    let timer = Instant::now();
    let clients = build_clients(&options)?;
    let start_url = crawl_state
        .current_level
        .first()
        .map(|(u, _)| u.clone())
        .unwrap_or_default();

    crawl(
        app,
        state.stop_flag.clone(),
        state.active_browsers.clone(),
        state.browser_id_counter.clone(),
        clients,
        start_url,
        options,
        timer,
        Some(crawl_state),
    )
    .await
}

#[tauri::command]
pub async fn load_session(path: String) -> Result<serde_json::Value, ScrapeError> {
    let content = fs::read_to_string(path).map_err(|e| ScrapeError::File(e.to_string()))?;
    serde_json::from_str(&content).map_err(|e| ScrapeError::Parse(e.to_string()))
}

#[tauri::command]
pub async fn save_text(text: String, path: String) -> Result<(), ScrapeError> {
    fs::write(path, text).map_err(|e| ScrapeError::File(e.to_string()))
}

#[tauri::command]
pub fn read_text(path: String) -> Result<String, ScrapeError> {
    fs::read_to_string(path).map_err(|e| ScrapeError::File(e.to_string()))
}

#[tauri::command]
pub fn get_app_dir(app: tauri::AppHandle) -> Result<String, ScrapeError> {
    use tauri::Manager;
    let mut path = app.path().document_dir().map_err(|e| ScrapeError::File(e.to_string()))?;
    path.push("Srappy");
    if !path.exists() {
        fs::create_dir_all(&path).map_err(|e| ScrapeError::File(e.to_string()))?;
    }
    Ok(path.to_string_lossy().into_owned())
}

#[tauri::command]
pub fn set_close_to_tray(state: tauri::State<'_, Arc<AppState>>, enabled: bool) {
    state.close_to_tray.store(enabled, Ordering::SeqCst);
}
