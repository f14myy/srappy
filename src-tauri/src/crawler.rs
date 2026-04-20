use reqwest::Client;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tauri::Emitter;
use url::Url;

use crate::error::ScrapeError;
use crate::models::{PageResult, ScrapeOptions, ScrapeResult, CrawlState, ProgressEvent, LogEvent};
use crate::fetcher::fetch_page;
use crate::extractor::normalized_registrable_host;

pub fn dedup_queue_by_min_depth(items: Vec<(String, usize)>) -> Vec<(String, usize)> {
    let mut best: HashMap<String, usize> = HashMap::new();
    for (url, d) in items {
        best
            .entry(url)
            .and_modify(|dd| *dd = (*dd).min(d))
            .or_insert(d);
    }
    let mut out: Vec<_> = best.into_iter().collect();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

pub async fn crawl(
    app: tauri::AppHandle,
    stop_flag: Arc<AtomicBool>,
    active_browsers: Arc<std::sync::Mutex<std::collections::HashMap<u64, headless_chrome::Browser>>>,
    browser_id_counter: Arc<std::sync::atomic::AtomicU64>,
    clients: Arc<Vec<Client>>,
    start_url: String,
    opts: ScrapeOptions,
    timer: Instant,
    initial_state: Option<CrawlState>,
) -> Result<ScrapeResult, ScrapeError> {
    let max_depth = opts.max_depth;
    let max_pages = opts.max_pages;

    let mut visited: HashSet<String> = initial_state.as_ref().map(|s| s.visited.clone()).unwrap_or_default();
    let mut visited_domains: HashSet<String> = HashSet::new();
    let mut results: Vec<PageResult> = initial_state.as_ref().map(|s| s.results.clone()).unwrap_or_default();
    
    if opts.unique_domains_only.unwrap_or(false) {
        for res in &results {
            if let Ok(u) = Url::parse(&res.url) {
                if let Some(host) = u.host_str() {
                    visited_domains.insert(normalized_registrable_host(host));
                }
            }
        }
    }

    let pages_found = Arc::new(AtomicUsize::new(if results.is_empty() { 1 } else { results.len() }));
    let last_emit = Arc::new(std::sync::Mutex::new(Instant::now()));
    let emit_throttle = std::time::Duration::from_millis(50);

    let mut current_level: Vec<(String, usize)> = initial_state.map(|s| s.current_level).unwrap_or_else(|| vec![(start_url, 0)]);

    let mut is_stopped = false;
    let mut pages_failed = 0usize;

    'outer: while !current_level.is_empty() {
        if stop_flag.load(Ordering::SeqCst) {
            is_stopped = true;
            break 'outer;
        }
        let mut to_scrape: Vec<(String, usize)> = Vec::new();
        for (url, depth) in &current_level {
            if to_scrape.len() >= max_pages.saturating_sub(results.len()) {
                break;
            }
            if visited.contains(url) {
                continue;
            }
            if opts.unique_domains_only.unwrap_or(false) {
                if let Ok(u) = Url::parse(url) {
                    if let Some(host) = u.host_str() {
                        let norm = normalized_registrable_host(host);
                        if visited_domains.contains(&norm) {
                            continue;
                        }
                    }
                }
            }
            to_scrape.push((url.clone(), *depth));
        }

        if to_scrape.is_empty() {
            break;
        }

        for (url, _) in &to_scrape {
            visited.insert(url.clone());
            if opts.unique_domains_only.unwrap_or(false) {
                if let Ok(u) = Url::parse(url) {
                    if let Some(host) = u.host_str() {
                        visited_domains.insert(normalized_registrable_host(host));
                    }
                }
            }
        }

        let depth = to_scrape.first().map(|(_, d)| *d).unwrap_or(0);

        {
            let mut last = last_emit.lock().unwrap();
            *last = Instant::now();
            let _ = app.emit(
                "scrape-progress",
                ProgressEvent {
                    current_url: to_scrape[0].0.clone(),
                    pages_done: results.len(),
                    pages_found: pages_found.load(Ordering::Relaxed),
                    total_pages: max_pages,
                    pages_failed,
                    depth,
                },
            );
        }

        let mut join_set = tokio::task::JoinSet::new();
        for (idx, (url, d)) in to_scrape.into_iter().enumerate() {
            let clients = clients.clone();
            let client = {
                let mut rng = rand::thread_rng();
                use rand::Rng;
                let client_idx = rng.gen_range(0..clients.len());
                clients[client_idx].clone()
            };

            let opts = opts.clone();
            let app_clone = app.clone();
            let stop_flag_clone = stop_flag.clone();
            let active_browsers_clone = active_browsers.clone();
            let browser_id_counter_clone = browser_id_counter.clone();

            join_set.spawn(async move {
                if let Some(delay) = opts.delay_ms {
                    if delay > 0 {
                        // Check stop flag before delay
                        if stop_flag_clone.load(Ordering::SeqCst) {
                            return Err(ScrapeError::Other("Stopped".into()));
                        }
                        tokio::time::sleep(std::time::Duration::from_millis(delay * (idx as u64))).await;
                    }
                }

                if stop_flag_clone.load(Ordering::SeqCst) {
                    return Err(ScrapeError::Other("Stopped".into()));
                }

                let timer = std::time::Instant::now();
                let res = fetch_page(&client, &url, &opts, stop_flag_clone, active_browsers_clone, browser_id_counter_clone).await;
                let elapsed = timer.elapsed().as_millis();
                
                match res {
                    Ok(r) => {
                        let _ = app_clone.emit("scrape-log", LogEvent { url: url.clone(), status: "ok".into(), time_ms: elapsed, message: None });
                        Ok((r, d, url))
                    }
                    Err(e) => {
                        let _ = app_clone.emit("scrape-log", LogEvent { url: url.clone(), status: "error".into(), time_ms: elapsed, message: Some(e.to_string()) });
                        Err(e)
                    }
                }
            });
        }

        let mut next_level: Vec<(String, usize)> = Vec::new();

        loop {
            if stop_flag.load(Ordering::SeqCst) {
                join_set.abort_all();
                is_stopped = true;
                break 'outer;
            }

            tokio::select! {
                task_result = join_set.join_next() => {
                    let task_result = match task_result {
                        Some(res) => res,
                        None => break, // All tasks done in this level
                    };

                    match task_result {
                        Ok(Ok(((page, links), d, url))) => {
                            if d < max_depth {
                                let mut new: Vec<(String, usize)> = Vec::new();
                                for u_str in links {
                                    if visited.contains(&u_str) {
                                        continue;
                                    }
                                    if opts.unique_domains_only.unwrap_or(false) {
                                        if let Ok(u_parsed) = Url::parse(&u_str) {
                                            if let Some(host) = u_parsed.host_str() {
                                                let norm = normalized_registrable_host(host);
                                                if visited_domains.contains(&norm) {
                                                    continue;
                                                }
                                                visited_domains.insert(norm);
                                            }
                                        }
                                    }
                                    new.push((u_str, d + 1));
                                }
                                pages_found.fetch_add(new.len(), Ordering::Relaxed);
                                next_level.extend(new);
                            }
                            results.push(page);

                            {
                                let mut last = last_emit.lock().unwrap();
                                if last.elapsed() > emit_throttle {
                                    *last = Instant::now();
                                    let _ = app.emit(
                                        "scrape-progress",
                                        ProgressEvent {
                                            current_url: url,
                                            pages_done: results.len(),
                                            pages_found: pages_found.load(Ordering::Relaxed),
                                            total_pages: max_pages,
                                            pages_failed,
                                            depth,
                                        },
                                    );
                                }
                            }

                            if results.len() >= max_pages {
                                join_set.abort_all();
                                break 'outer;
                            }
                        }
                        Ok(Err(_)) => {
                            pages_failed += 1;
                        }
                        Err(e) => {
                            if e.is_cancelled() {
                                continue;
                            }
                            pages_failed += 1;
                        }
                    }
                }
                _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {
                    // Just to loop back and check stop_flag
                }
            }
        }

        current_level = dedup_queue_by_min_depth(next_level);
        if depth >= max_depth {
            break;
        }
    }

    let _ = app.emit(
        "scrape-progress",
        ProgressEvent {
            current_url: String::new(),
            pages_done: results.len(),
            pages_found: pages_found.load(Ordering::Relaxed),
            total_pages: max_pages,
            pages_failed,
            depth: 0,
        },
    );

    let total_chars = results.iter().map(|p| p.char_count).sum();
    let total_size = results.iter().map(|p| p.size_bytes).sum();
    let pages_scraped = results.len();

    let crawl_state = if is_stopped {
        Some(CrawlState {
            current_level: current_level.clone(),
            visited: visited.clone(),
            results: results.clone(),
        })
    } else {
        None
    };

    Ok(ScrapeResult {
        pages: results,
        total_chars,
        total_size_bytes: total_size,
        load_time_ms: timer.elapsed().as_millis(),
        pages_scraped,
        pages_failed,
        crawl_state,
    })
}
