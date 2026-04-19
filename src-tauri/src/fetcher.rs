use reqwest::Client;
use scraper::Html;
use std::cell::RefCell;
use std::sync::Arc;
use std::time::Instant;
use url::Url;

use crate::error::ScrapeError;
use crate::models::{PageResult, ScrapeOptions};
use crate::extractor::{extract_clean_text, post_process, extract_links, normalized_registrable_host, host_matches_domain_rule};

thread_local! {
    pub static HEADLESS_BROWSER: RefCell<Option<headless_chrome::Browser>> = const { RefCell::new(None) };
}

pub async fn fetch_page(
    client: &Client,
    url: &str,
    opts: &ScrapeOptions,
) -> Result<(PageResult, Vec<String>), ScrapeError> {
    if opts.use_headless_chrome.unwrap_or(false) {
        return fetch_page_headless(url, opts).await;
    }

    let start = Instant::now();
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() || e.status() == Some(reqwest::StatusCode::TOO_MANY_REQUESTS) {
                ScrapeError::BlockedOrTimeout(e.to_string())
            } else {
                ScrapeError::Network(e.to_string())
            }
        })?;

    if !response.status().is_success() {
        return Err(if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS || response.status() == reqwest::StatusCode::FORBIDDEN {
            ScrapeError::BlockedOrTimeout(format!("HTTP {}", response.status()))
        } else {
            ScrapeError::Network(format!("HTTP {}", response.status()))
        });
    }

    let size_hint = response.content_length().unwrap_or(0) as usize;
    let html = response.text().await.map_err(|e| ScrapeError::Parse(e.to_string()))?;
    let actual_size = if size_hint == 0 { html.len() } else { size_hint };

    process_html_to_result(&html, url, opts, start.elapsed().as_millis(), actual_size)
}

pub async fn fetch_page_headless(
    url: &str,
    opts: &ScrapeOptions,
) -> Result<(PageResult, Vec<String>), ScrapeError> {
    let start = Instant::now();
    let url_string = url.to_string();

    let html = tokio::task::spawn_blocking(move || -> Result<String, ScrapeError> {
        HEADLESS_BROWSER.with(|cell| -> Result<String, ScrapeError> {
            let mut slot = cell.borrow_mut();
            if slot.is_none() {
                let browser = headless_chrome::Browser::new(headless_chrome::LaunchOptions {
                    headless: true,
                    sandbox: false,
                    ..Default::default()
                })
                .map_err(|e| ScrapeError::Parse(e.to_string()))?;
                *slot = Some(browser);
            }
            let browser = slot
                .as_ref()
                .ok_or_else(|| ScrapeError::Parse("headless browser not initialized".into()))?;

            let tab = browser.new_tab().map_err(|e| ScrapeError::Parse(e.to_string()))?;
            tab.navigate_to(&url_string)
                .map_err(|e| ScrapeError::Parse(e.to_string()))?;
            tab.wait_until_navigated()
                .map_err(|e| ScrapeError::Parse(e.to_string()))?;

            for _ in 0..40 {
                let state = tab
                    .evaluate("document.readyState", false)
                    .map_err(|e| ScrapeError::Parse(e.to_string()))?;
                let ready = state
                    .value
                    .as_ref()
                    .and_then(|v| v.as_str())
                    .map(|s| s == "complete")
                    .unwrap_or(false);
                if ready {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(50));
            }

            let html_val = tab
                .evaluate("document.documentElement.outerHTML", false)
                .map_err(|e| ScrapeError::Parse(e.to_string()))?;
            drop(tab);
            if let Some(val) = html_val.value {
                if let Some(s) = val.as_str() {
                    return Ok(s.to_string());
                }
            }
            Err(ScrapeError::Parse(
                "Could not extract HTML from headless chrome".to_string(),
            ))
        })
    })
    .await
    .map_err(|e| ScrapeError::Other(e.to_string()))??;

    process_html_to_result(&html, url, opts, start.elapsed().as_millis(), html.len())
}

pub fn process_html_to_result(
    html: &str,
    url: &str,
    opts: &ScrapeOptions,
    load_time_ms: u128,
    actual_size: usize,
) -> Result<(PageResult, Vec<String>), ScrapeError> {
    let base_url = Url::parse(url).unwrap_or_else(|_| Url::parse("http://localhost").unwrap());
    
    let document = Html::parse_document(html);
    let mut raw = String::new();
    let mut selector_matched = false;
    let empty_vec = vec![];
    let excluded = opts.excluded_tags.as_ref().unwrap_or(&empty_vec);

    if let Some(target) = opts.target_selector.as_deref().filter(|s| !s.is_empty()) {
        if let Ok(sel) = scraper::Selector::parse(target) {
            for el in document.select(&sel) {
                raw.push_str(&extract_clean_text(el, excluded));
                raw.push('\n');
                selector_matched = true;
            }
        }
    }

    if !selector_matched {
        raw = extract_clean_text(document.root_element(), excluded);
    }

    let text = post_process(raw, opts);
    let mut links = extract_links(&document, &base_url);

    let whitelist: Vec<&str> = if opts.enable_whitelist.unwrap_or(false) {
        opts.domain_whitelist.as_deref().unwrap_or("").split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
    } else { vec![] };
    
    let blacklist: Vec<&str> = if opts.enable_blacklist.unwrap_or(false) {
        opts.domain_blacklist.as_deref().unwrap_or("").split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
    } else { vec![] };
    let base_host_opt = Url::parse(url).ok().and_then(|u| u.host_str().map(|s| s.to_string()));
    let base_norm = base_host_opt.as_deref().map(normalized_registrable_host);

    links.retain(|l| {
        if let Ok(l_parsed) = Url::parse(l) {
            if let Some(host) = l_parsed.host_str() {
                if opts.same_domain_only.unwrap_or(false) && !opts.unique_domains_only.unwrap_or(false) {
                    let link_norm = normalized_registrable_host(host);
                    if base_norm.as_deref() != Some(&link_norm) {
                        return false;
                    }
                }
                if !whitelist.is_empty()
                    && !whitelist
                        .iter()
                        .any(|&w| host_matches_domain_rule(host, w))
                {
                    return false;
                }
                if !blacklist.is_empty()
                    && blacklist
                        .iter()
                        .any(|&b| host_matches_domain_rule(host, b))
                {
                    return false;
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    });

    Ok((
        PageResult {
            url: url.to_string(),
            char_count: text.chars().count(),
            size_bytes: actual_size,
            load_time_ms,
            text,
        },
        links,
    ))
}

pub fn build_clients(opts: &ScrapeOptions) -> Result<Arc<Vec<Client>>, ScrapeError> {
    let mut clients = Vec::new();
    
    let default_ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
    let uas: Vec<&str> = if let Some(ua_list) = &opts.user_agents {
        let list: Vec<&str> = ua_list.split('\n').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        if list.is_empty() { vec![default_ua] } else { list }
    } else {
        vec![default_ua]
    };

    let proxies: Vec<&str> = if let Some(p_list) = &opts.proxies {
        p_list.split('\n').map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
    } else {
        vec![]
    };

    if proxies.is_empty() {
        for ua in uas {
            let client = Client::builder()
                .user_agent(ua)
                .timeout(std::time::Duration::from_secs(15))
                .build()
                .map_err(|e| ScrapeError::Network(e.to_string()))?;
            clients.push(client);
        }
    } else {
        for p in proxies {
            let proxy = reqwest::Proxy::all(p).map_err(|e| {
                ScrapeError::Network(format!("Invalid proxy '{}': {}", p, e))
            })?;
            for ua in &uas {
                let client = Client::builder()
                    .user_agent(*ua)
                    .timeout(std::time::Duration::from_secs(15))
                    .proxy(proxy.clone())
                    .build()
                    .map_err(|e| ScrapeError::Network(e.to_string()))?;
                clients.push(client);
            }
        }
    }

    if clients.is_empty() {
        let client = Client::builder()
            .user_agent(default_ua)
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .map_err(|e| ScrapeError::Network(e.to_string()))?;
        clients.push(client);
    }

    Ok(Arc::new(clients))
}
