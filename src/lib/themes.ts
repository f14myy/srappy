import type { AppPreferences } from "./appPreferences";

export type ThemeKey = string;

export type ThemeConfig = {
  bg: string;
  surface: string;
  border: string;
  text: string;
  textMuted: string;
  accent: string;
  accentText: string;
  grid: string;
};

export const darkThemes: string[] = [
  "nord",
  "dark",
  "deepdark",
  "dracula",
  "anthropic",
  "gruvbox",
  "tokyonight",
  "catppuccin",
  "everforest_dark",
  "material_ocean",
  "onedark",
  "aura",
];

export const lightThemes: string[] = [
  "light",
  "nord_light",
  "gruvbox_light",
  "github_light",
  "rosepine_dawn",
  "catppuccin_latte",
  "tokyonight_day",
  "solarized_light",
  "everforest_light",
  "soft_warm",
  "high_contrast_light",
];

export const builtinThemes: Record<string, ThemeConfig> = {
  // темные темы
  anthropic: {
    bg: "#1d1b1a",
    surface: "rgba(35,33,31,0.85)",
    border: "#3c3836",
    text: "#fdfbf7",
    textMuted: "#a09d98",
    accent: "#d97757",
    accentText: "#1d1b1a",
    grid: "#262322",
  },
  dark: {
    bg: "#121212",
    surface: "rgba(30,30,30,0.85)",
    border: "#333333",
    text: "#ededed",
    textMuted: "#888888",
    accent: "#ffffff",
    accentText: "#000000",
    grid: "#222222",
  },
  deepdark: {
    bg: "#000000",
    surface: "rgba(10,10,10,0.8)",
    border: "#222222",
    text: "#d4d4d4",
    textMuted: "#71717a",
    accent: "#ffffff",
    accentText: "#000000",
    grid: "#111111",
  },
  nord: {
    bg: "#2e3440",
    surface: "rgba(40,44,52,0.85)",
    border: "#434c5e",
    text: "#d8dee9",
    textMuted: "#81a1c1",
    accent: "#88c0d0",
    accentText: "#2e3440",
    grid: "#3b4252",
  },
  catppuccin: {
    bg: "#1e1e2e",
    surface: "rgba(24,24,37,0.85)",
    border: "#313244",
    text: "#cdd6f4",
    textMuted: "#a6adc8",
    accent: "#cba6f7",
    accentText: "#11111b",
    grid: "#181825",
  },
  dracula: {
    bg: "#282a36",
    surface: "rgba(33,34,44,0.85)",
    border: "#44475a",
    text: "#f8f8f2",
    textMuted: "#6272a4",
    accent: "#ff79c6",
    accentText: "#282a36",
    grid: "#21222c",
  },
  gruvbox: {
    bg: "#282828",
    surface: "rgba(40,40,40,0.85)",
    border: "#504945",
    text: "#ebdbb2",
    textMuted: "#a89984",
    accent: "#fabd2f",
    accentText: "#282828",
    grid: "#32302f",
  },
  tokyonight: {
    bg: "#1a1b26",
    surface: "rgba(22,22,30,0.85)",
    border: "#292e42",
    text: "#c0caf5",
    textMuted: "#565f89",
    accent: "#7aa2f7",
    accentText: "#15161e",
    grid: "#16161e",
  },
  everforest_dark: {
    bg: "#2b3339",
    surface: "rgba(40,48,54,0.85)",
    border: "#323c41",
    text: "#d3c6aa",
    textMuted: "#859289",
    accent: "#a7c080",
    accentText: "#2b3339",
    grid: "#3a454d",
  },
  material_ocean: {
    bg: "#0f111a",
    surface: "rgba(15,17,26,0.85)",
    border: "#1c2030",
    text: "#8f93a2",
    textMuted: "#4b526d",
    accent: "#80cbc4",
    accentText: "#0f111a",
    grid: "#191d2b",
  },
  onedark: {
    bg: "#282c34",
    surface: "rgba(33,37,43,0.85)",
    border: "#3e4451",
    text: "#abb2bf",
    textMuted: "#5c6370",
    accent: "#61afef",
    accentText: "#282c34",
    grid: "#2c313a",
  },
  aura: {
    bg: "#15141b",
    surface: "rgba(21,20,27,0.85)",
    border: "#2d2b38",
    text: "#edecee",
    textMuted: "#6d6d6d",
    accent: "#a277ff",
    accentText: "#15141b",
    grid: "#1c1b22",
  },

  // светлые темы
  light: {
    bg: "#f8f8f8",
    surface: "rgba(255,255,255,0.85)",
    border: "#e0e0e0",
    text: "#111111",
    textMuted: "#777777",
    accent: "#111111",
    accentText: "#ffffff",
    grid: "#e8e8e8",
  },
  nord_light: {
    bg: "#eceff4",
    surface: "rgba(229,233,240,0.85)",
    border: "#d8dee9",
    text: "#2e3440",
    textMuted: "#4c566a",
    accent: "#5e81ac",
    accentText: "#eceff4",
    grid: "#e5e9f0",
  },
  rosepine_dawn: {
    bg: "#faf4ed",
    surface: "rgba(255,250,243,0.85)",
    border: "#dfdad9",
    text: "#575279",
    textMuted: "#797593",
    accent: "#907aa9",
    accentText: "#faf4ed",
    grid: "#f2e9e1",
  },
  catppuccin_latte: {
    bg: "#eff1f5",
    surface: "rgba(230,233,239,0.85)",
    border: "#ccd0da",
    text: "#4c4f69",
    textMuted: "#6c6f85",
    accent: "#8839ef",
    accentText: "#eff1f5",
    grid: "#e6e9ef",
  },
  gruvbox_light: {
    bg: "#fbf1c7",
    surface: "rgba(242,229,188,0.85)",
    border: "#d5c4a1",
    text: "#3c3836",
    textMuted: "#7c6f64",
    accent: "#b57614",
    accentText: "#fbf1c7",
    grid: "#ebdbb2",
  },
  tokyonight_day: {
    bg: "#e1e2e7",
    surface: "rgba(213,214,219,0.85)",
    border: "#c4c8da",
    text: "#3760bf",
    textMuted: "#6172b0",
    accent: "#2e7de9",
    accentText: "#e1e2e7",
    grid: "#d5d6db",
  },
  solarized_light: {
    bg: "#fdf6e3",
    surface: "rgba(238,232,213,0.85)",
    border: "#ccc2a6",
    text: "#657b83",
    textMuted: "#93a1a1",
    accent: "#268bd2",
    accentText: "#fdf6e3",
    grid: "#eee8d5",
  },
  github_light: {
    bg: "#ffffff",
    surface: "rgba(246,248,250,0.85)",
    border: "#d0d7de",
    text: "#24292f",
    textMuted: "#57606a",
    accent: "#0969da",
    accentText: "#ffffff",
    grid: "#eaeff5",
  },
  everforest_light: {
    bg: "#fdf6e3",
    surface: "rgba(248,242,223,0.85)",
    border: "#e5dfc5",
    text: "#5c6a72",
    textMuted: "#939f91",
    accent: "#8da101",
    accentText: "#fdf6e3",
    grid: "#f0ebce",
  },
  soft_warm: {
    bg: "#faf9f6",
    surface: "rgba(255,255,255,0.85)",
    border: "#e8e6e1",
    text: "#4a4a4a",
    textMuted: "#a0a0a0",
    accent: "#d4a373",
    accentText: "#ffffff",
    grid: "#f0eee9",
  },
  high_contrast_light: {
    bg: "#ffffff",
    surface: "rgba(255,255,255,1)",
    border: "#000000",
    text: "#000000",
    textMuted: "#666666",
    accent: "#000000",
    accentText: "#ffffff",
    grid: "#eeeeee",
  },
};

export function applyTheme(key: string, prefs?: AppPreferences) {
  let t = builtinThemes[key];
  if (!t && prefs) {
    const custom = prefs.customThemes.find((ct) => ct.id === key);
    if (custom) t = custom.config;
  }

  if (!t) t = builtinThemes["nord"]; // если ничего не нашли, берем норд как базу

  const root = document.documentElement;
  root.style.setProperty("--bg-color", t.bg);
  root.style.setProperty("--surface", t.surface);
  root.style.setProperty("--border-color", t.border);
  root.style.setProperty("--text-primary", t.text);
  root.style.setProperty("--text-muted", t.textMuted);
  root.style.setProperty("--accent", t.accent);
  root.style.setProperty("--accent-text", t.accentText);
  root.style.setProperty("--grid-color", t.grid);
}
