use crate::models::ScrapeOptions;
use scraper::{Html, Selector};
use url::Url;

const BLOCK_TAGS: &[&str] = &[
    "p",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "blockquote",
    "pre",
    "br",
    "hr",
    "div",
    "section",
    "article",
    "header",
    "footer",
    "aside",
    "nav",
    "main",
    "li",
    "tr",
    "td",
    "dt",
    "dd",
    "address",
    "figure",
    "figcaption",
    "table",
    "thead",
    "tbody",
    "tfoot",
    "ul",
    "ol",
    "dl",
    "details",
    "summary",
    "form",
    "fieldset",
    "legend",
    "label",
    "caption",
    "center",
];

const IGNORE_TAGS: &[&str] = &["script", "style", "noscript", "svg", "head", "meta", "link"];

pub fn extract_clean_text(node: scraper::ElementRef<'_>, user_excluded: &[String]) -> String {
    let mut text = String::new();

    for child in node.children() {
        if let Some(el) = child.value().as_element() {
            let tag = el.name();

            // пропускаем мусорные теги и то, что юзер попросил исключить
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
                        // для строчных элементов просто следим за пробелами
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
                // добавляем пробел, чтобы слова не слипались
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
        // оставляем только буквы и пробелы
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
            // убираем якоря (#foo), они нам не нужны для краулинга
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
