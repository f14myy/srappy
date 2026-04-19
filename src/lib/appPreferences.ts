import type { Lang } from "$lib/i18n";
import type { ThemeConfig } from "$lib/themes";

const STORAGE_KEY = "srappy-app-preferences-v2"; // Bumped version for schema change
const WINDOW_KEY = "srappy-window-bounds-v1";

export type CustomTheme = {
  id: string;
  name: string;
  config: ThemeConfig;
  mode: "dark" | "light";
};

export type AppPreferences = {
  theme: string;
  lang: Lang;
  /** Multiplier for base UI (1 = 100%) */
  uiScale: number;
  reduceMotion: boolean;
  logMaxLines: number;
  notifyOnComplete: boolean;
  rememberWindow: boolean;
  lastExportDir: string;
  interactiveGrid: boolean;
  // NEW AUTOMATION & UI
  defaultExportFormat: "txt" | "json" | "csv" | "csv_meta" | "md" | "srappy";
  filenamePattern: string;
  autoSaveSessions: boolean;
  gridIntensity: number;
  startOnBoot: boolean;
  closeToTray: boolean;
  customThemes: CustomTheme[];
};

export const DEFAULT_APP_PREFERENCES: AppPreferences = {
  theme: "nord",
  lang: "en",
  uiScale: 1,
  reduceMotion: false,
  logMaxLines: 500,
  notifyOnComplete: true,
  rememberWindow: true,
  lastExportDir: "",
  interactiveGrid: true,
  // DEFAULTS
  defaultExportFormat: "txt",
  filenamePattern: "{timestamp}_{host}",
  autoSaveSessions: false,
  gridIntensity: 0.8,
  startOnBoot: false,
  closeToTray: false,
  customThemes: [],
};

export function loadAppPreferences(): AppPreferences {
  if (typeof localStorage === "undefined") return { ...DEFAULT_APP_PREFERENCES };
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return { ...DEFAULT_APP_PREFERENCES };
    const parsed = JSON.parse(raw) as Partial<AppPreferences>;
    return { ...DEFAULT_APP_PREFERENCES, ...parsed };
  } catch {
    return { ...DEFAULT_APP_PREFERENCES };
  }
}

export function saveAppPreferences(p: AppPreferences): void {
  if (typeof localStorage === "undefined") return;
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(p));
  } catch {
    /* ignore quota */
  }
}

export type WindowBounds = { x: number; y: number; width: number; height: number };

export function loadWindowBounds(): WindowBounds | null {
  if (typeof localStorage === "undefined") return null;
  try {
    const raw = localStorage.getItem(WINDOW_KEY);
    if (!raw) return null;
    return JSON.parse(raw) as WindowBounds;
  } catch {
    return null;
  }
}

export function saveWindowBounds(b: WindowBounds): void {
  if (typeof localStorage === "undefined") return;
  try {
    localStorage.setItem(WINDOW_KEY, JSON.stringify(b));
  } catch {
    /* ignore */
  }
}
