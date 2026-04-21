// основные типы данных приложения
export type PageResult = {
  url: string;
  text: string;
  char_count: number;
  size_bytes: number;
  load_time_ms: number;
};

export type CrawlState = {
  current_level: [string, number][];
  visited: string[];
  results?: PageResult[];
};

export type ScrapeResult = {
  pages: PageResult[];
  total_chars: number;
  total_size_bytes: number;
  load_time_ms: number;
  pages_scraped: number;
  pages_failed?: number;
  crawl_state?: CrawlState;
};

export type ScrapeOptions = {
  remove_whitespace: boolean;
  remove_newlines: boolean;
  remove_numbers: boolean;
  remove_special_chars: boolean;
  recursive: boolean;
  max_depth: number;
  max_pages: number;
  delay_ms: number;
  same_domain_only: boolean;
  enable_whitelist: boolean;
  enable_blacklist: boolean;
  domain_whitelist: string;
  domain_blacklist: string;
  target_selector: string;
  proxies: string;
  user_agents: string;
  use_headless_chrome: boolean;
  unique_domains_only: boolean;
  concurrency: number;
  excluded_tags: string[];
};

export type LogEvent = {
  url: string;
  status: "ok" | "error";
  time_ms: number;
  message?: string;
};

export type ProgressEvent = {
  current_url: string;
  pages_done: number;
  pages_found: number;
  total_pages: number;
  pages_failed: number;
  depth: number;
};

export interface ScrapeSession {
  id: string;
  result: ScrapeResult;
  url: string;
  options: ScrapeOptions;
  speed: number;
  speedHistory: number[];
  timestamp: number;
}

export type ScrapeHistoryItem = {
  id: string;
  timestamp: number;
  url: string;
  pages_scraped: number;
  pages_failed?: number;
  total_chars: number;
  total_size_bytes: number;
  duration_ms: number;
  recursive: boolean;
  max_depth?: number;
  avg_speed?: number;
};
