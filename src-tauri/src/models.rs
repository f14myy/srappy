use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone)]
pub struct PageResult {
    pub url: String,
    pub text: String,
    pub char_count: usize,
    pub size_bytes: usize,
    pub load_time_ms: u128,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ScrapeResult {
    pub pages: Vec<PageResult>,
    pub total_chars: usize,
    pub total_size_bytes: usize,
    pub load_time_ms: u128,
    pub pages_scraped: usize,
    #[serde(default)]
    pub pages_failed: usize,
    pub crawl_state: Option<CrawlState>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CrawlState {
    pub current_level: Vec<(String, usize)>,
    pub visited: HashSet<String>,
    pub results: Vec<PageResult>,
}

pub struct AppState {
    pub stop_flag: Arc<AtomicBool>,
    pub close_to_tray: Arc<AtomicBool>,
    pub active_browsers: Arc<std::sync::Mutex<std::collections::HashMap<u64, headless_chrome::Browser>>>,
    pub browser_id_counter: Arc<std::sync::atomic::AtomicU64>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ScrapeOptions {
    pub remove_whitespace: bool,
    pub remove_newlines: bool,
    pub remove_numbers: bool,
    pub remove_special_chars: bool,
    pub recursive: bool,
    pub max_depth: usize,
    pub max_pages: usize,
    pub delay_ms: Option<u64>,
    pub same_domain_only: Option<bool>,
    pub enable_whitelist: Option<bool>,
    pub enable_blacklist: Option<bool>,
    pub domain_whitelist: Option<String>,
    pub domain_blacklist: Option<String>,
    pub target_selector: Option<String>,
    pub proxies: Option<String>,
    pub user_agents: Option<String>,
    pub use_headless_chrome: Option<bool>,
    pub unique_domains_only: Option<bool>,
    pub excluded_tags: Option<Vec<String>>,
}

#[derive(Serialize, Clone)]
pub struct LogEvent {
    pub url: String,
    pub status: String,
    pub time_ms: u128,
    pub message: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct ProgressEvent {
    pub current_url: String,
    pub pages_done: usize,
    pub pages_found: usize,
    pub total_pages: usize,
    pub pages_failed: usize,
    pub depth: usize,
}
