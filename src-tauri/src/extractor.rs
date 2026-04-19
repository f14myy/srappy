use scraper::{Html, Selector};
use url::Url;
use crate::models::ScrapeOptions;

const BLOCK_TAGS: &[&str] = &[
    "p", "h1", "h2", "h3", "h4", "h5", "h6", "blockquote", "pre", "br", "hr",
    "div", "section", "article", "header", "footer", "aside", "nav", "main",
    "li", "tr", "td", "dt", "dd", "address", "figure", "figcaption",
    "table", "thead", "tbody", "tfoot", "ul", "ol", "dl", "details", "summary",
    "form", "fieldset", "legend", "label", "caption", "center",
];

const IGNORE_TAGS: &[&str] = &[
    "script", "style", "noscript", "svg", "head", "meta", "link",
];

pub fn extract_clean_text(node: scraper::ElementRef<'_>, user_excluded: &[String]) -> String {
    let mut text = String::new();
    
    for child in node.children() {
        if let Some(el) = child.value().as_element() {
            let tag = el.name();
            
            // Skip hidden, interactive or user-excluded tags
            if IGNORE_TAGS.contains(&tag) || user_excluded.iter().any(|t| t == tag) {
                continue;
            }
            
            if let Some(el_ref) = scraper::ElementRef::wrap(child) {
                let inner = extract_clean_text(el_ref, user_excluded);
                if !inner.is_empty() {
                    if BLOCK_TAGS.contains(&tag) {
                        text.push('\n');
                        text.push_str(inner.trim());
                        text.push('\n');
                    } else {
                        // For inline elements (span, b, a), just ensure a space
                        if !text.ends_with(' ') && !text.ends_with('\n') {
                            text.push(' ');
                        }
                        text.push_str(inner.trim());
                    }
                }
            }
        } else if let Some(t) = child.value().as_text() {
            let content = t.trim();
            if !content.is_empty() {
                // Check if we need a leading space to avoid merging words
                if !text.is_empty() && !text.ends_with(' ') && !text.ends_with('\n') {
                    text.push(' ');
                }
                text.push_str(content);
            }
        }
    }
    text
}

pub fn post_process(raw: String, opts: &ScrapeOptions) -> String {
    // Collapse blank lines
    let mut text: String = raw
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    if opts.remove_newlines {
        text = text.replace('\n', " ");
    }
    if opts.remove_numbers {
        text = text.chars().filter(|c| !c.is_ascii_digit()).collect();
    }
    if opts.remove_special_chars {
        // Keep letters (all scripts), digits (unless removed above), and whitespace
        text = text
            .chars()
            .filter(|c| c.is_alphabetic() || c.is_whitespace())
            .collect();
    }
    if opts.remove_whitespace {
        text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    }
    text
}

pub fn extract_links(document: &Html, base: &Url) -> Vec<String> {
    let Ok(selector) = Selector::parse("a[href]") else {
        return vec![];
    };
    let mut links: Vec<String> = document
        .select(&selector)
        .filter_map(|el| el.value().attr("href"))
        .filter_map(|href| base.join(href).ok())
        .filter(|u| u.scheme() == "http" || u.scheme() == "https")
        .map(|mut u| {
            u.set_fragment(None);
            u.to_string()
        })
        .collect();
    links.sort();
    links.dedup();
    links
}

/// Lowercase host and strip one leading `www.` so apex and www match for crawl policy.
pub fn normalized_registrable_host(host: &str) -> String {
    let h = host.trim().to_ascii_lowercase();
    if let Some(rest) = h.strip_prefix("www.") {
        if rest.is_empty() {
            return h;
        }
        return rest.to_string();
    }
    h
}

/// `rule` matches `host` if equal or if `host` is a subdomain of `rule` (dot suffix).
pub fn host_matches_domain_rule(host: &str, rule: &str) -> bool {
    let h = normalized_registrable_host(host);
    let r = normalized_registrable_host(rule);
    if r.is_empty() {
        return false;
    }
    h == r || h.ends_with(&format!(".{}", r))
}
