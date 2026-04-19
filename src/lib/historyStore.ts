import { invoke } from "@tauri-apps/api/core";
import type { ScrapeHistoryItem, ScrapeSession } from "./types";

const MAX_HISTORY = 100;
let cachedHistory: ScrapeHistoryItem[] | null = null;

async function getHistoryPath(): Promise<string> {
  const base = await invoke<string>("get_app_dir");
  return `${base}/history.json`;
}

export async function saveToHistory(session: ScrapeSession): Promise<void> {
  const history = await getHistory();
  const newItem: ScrapeHistoryItem = {
    id: session.id,
    timestamp: session.timestamp,
    url: session.url,
    pages_scraped: session.result.pages_scraped,
    pages_failed: session.result.pages_failed || 0,
    total_chars: session.result.total_chars,
    total_size_bytes: session.result.total_size_bytes,
    duration_ms: session.result.load_time_ms,
    recursive: session.options.recursive,
    max_depth: session.options.max_depth,
    avg_speed: session.speed
  };

  const updated = [newItem, ...history].slice(0, MAX_HISTORY);
  cachedHistory = updated;
  
  try {
    const path = await getHistoryPath();
    await invoke("save_text", { text: JSON.stringify(updated, null, 2), path });
  } catch (e) {
    console.error("Failed to save history to disk", e);
  }
}

export async function getHistory(): Promise<ScrapeHistoryItem[]> {
  if (cachedHistory) return cachedHistory;
  
  try {
    const path = await getHistoryPath();
    const raw = await invoke<string>("read_text", { path });
    cachedHistory = raw ? JSON.parse(raw) : [];
    return cachedHistory || [];
  } catch {
    return [];
  }
}

export async function clearHistory(): Promise<void> {
  cachedHistory = [];
  try {
    const path = await getHistoryPath();
    await invoke("save_text", { text: "[]", path });
  } catch (e) {
    console.error("Failed to clear history", e);
  }
}

export async function deleteHistoryItem(id: string): Promise<void> {
  const history = await getHistory();
  const updated = history.filter(item => item.id !== id);
  cachedHistory = updated;
  
  try {
    const path = await getHistoryPath();
    await invoke("save_text", { text: JSON.stringify(updated, null, 2), path });
  } catch (e) {
    console.error("Failed to delete history item", e);
  }
}
