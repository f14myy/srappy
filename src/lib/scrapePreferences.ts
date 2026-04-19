import type { ScrapeOptions } from "./types";

const SCRAPE_OPTIONS_KEY = "srappy-scrape-options-v1";

export const DEFAULT_SCRAPE_OPTIONS: ScrapeOptions = {
  remove_whitespace: false,
  remove_newlines: false,
  remove_numbers: false,
  remove_special_chars: false,
  recursive: false,
  max_depth: 2,
  max_pages: 10,
  delay_ms: 0,
  same_domain_only: false,
  enable_whitelist: false,
  enable_blacklist: false,
  domain_whitelist: "",
  domain_blacklist: "",
  target_selector: "",
  proxies: "",
  user_agents: "",
  use_headless_chrome: false,
  unique_domains_only: false,
  excluded_tags: ["button", "input", "select", "textarea"],
};

export function loadScrapeOptions(): ScrapeOptions {
  if (typeof localStorage === "undefined") return { ...DEFAULT_SCRAPE_OPTIONS };
  try {
    const raw = localStorage.getItem(SCRAPE_OPTIONS_KEY);
    if (!raw) return { ...DEFAULT_SCRAPE_OPTIONS };
    const parsed = JSON.parse(raw) as Partial<ScrapeOptions>;
    // Merge with defaults to ensure all fields (including new ones) exist
    return { ...DEFAULT_SCRAPE_OPTIONS, ...parsed };
  } catch {
    return { ...DEFAULT_SCRAPE_OPTIONS };
  }
}

export function saveScrapeOptions(options: ScrapeOptions): void {
  if (typeof localStorage === "undefined") return;
  try {
    // We don't want to save some transient state if there was any, 
    // but ScrapeOptions is purely configuration here.
    localStorage.setItem(SCRAPE_OPTIONS_KEY, JSON.stringify(options));
  } catch {
    /* ignore */
  }
}
