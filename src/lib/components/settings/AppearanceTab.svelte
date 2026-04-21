<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import type { AppPreferences, CustomTheme } from "$lib/appPreferences";
  import type { Translations } from "$lib/i18n";
  import { builtinThemes, darkThemes, lightThemes } from "$lib/themes";
  import { Check, Upload, Trash2 } from "lucide-svelte";
  import SegmentControl from "../ui/SegmentControl.svelte";

  type Props = {
    prefs: AppPreferences;
    tx: Translations;
    onpatch: (patch: Partial<AppPreferences>) => void;
    onerror?: (msg: string) => void;
  };

  let { prefs, tx, onpatch, onerror }: Props = $props();

  let themeMode = $state<"dark" | "light">("dark");

  $effect(() => {
    themeMode = lightThemes.includes(prefs.theme) ? "light" : "dark";
  });

  const scales = [
    { v: 0.9, label: "90%" },
    { v: 1, label: "100%" },
    { v: 1.1, label: "110%" },
    { v: 1.25, label: "125%" },
  ];

  async function importTheme() {
    try {
      const path = await open({
        filters: [{ name: "Srappy Theme", extensions: ["srappy-theme", "json"] }],
        multiple: false,
      });
      if (!path || Array.isArray(path)) return;

      const text = await invoke<string>("read_text", { path });
      const theme = JSON.parse(text) as CustomTheme;

      if (!theme.id || !theme.name || !theme.config) {
        throw new Error("Invalid theme file format");
      }

      const existing = (prefs.customThemes || []).find((t) => t.id === theme.id);
      if (existing) {
        onpatch({
          customThemes: prefs.customThemes.map((t) => (t.id === theme.id ? theme : t)),
        });
      } else {
        onpatch({
          customThemes: [...(prefs.customThemes || []), theme],
        });
      }
    } catch (e) {
      onerror?.(String(e));
    }
  }

  function deleteTheme(id: string, e: MouseEvent) {
    e.stopPropagation();
    onpatch({
      customThemes: prefs.customThemes.filter((t) => t.id !== id),
      theme: prefs.theme === id ? "nord" : prefs.theme,
    });
  }

  const allThemes = $derived({
    dark: [
      ...darkThemes,
      ...(prefs.customThemes || []).filter((t) => t.mode === "dark").map((t) => t.id),
    ],
    light: [
      ...lightThemes,
      ...(prefs.customThemes || []).filter((t) => t.mode === "light").map((t) => t.id),
    ],
  });

  function getThemeConfig(key: string) {
    return builtinThemes[key] || (prefs.customThemes || []).find((t) => t.id === key)?.config;
  }
</script>

<div class="tab">
  <div class="header-row">
    <h2>{tx.pref.appearance}</h2>
  </div>

  <div class="card-group">
    <div class="card-row">
      <label>{tx.global.theme}</label>
      <SegmentControl
        options={[
          { value: "dark", label: tx.global.darkThemes },
          { value: "light", label: tx.global.lightThemes },
        ]}
        active={themeMode}
        onchange={(v) => (themeMode = v as any)}
      />
    </div>
    <div class="card-content">
      <div class="theme-grid">
        {#each themeMode === "dark" ? allThemes.dark : allThemes.light as key}
          {@const typedTh = getThemeConfig(key)}
          {#if typedTh}
            <button
              type="button"
              class="theme-card"
              class:active={prefs.theme === key}
              style="--bg: {typedTh.bg}; --acc: {typedTh.accent}; --brd: {typedTh.border}; --txt: {typedTh.text};"
              onclick={() => onpatch({ theme: key })}
            >
              <div class="preview">
                <div class="dot" style="background: {typedTh.accent}"></div>
              </div>
              <span class="theme-name">{key.replace("dark-", "").replace("light-", "")}</span>
              {#if prefs.theme === key}<div class="check"><Check size={10} /></div>{/if}
              {#if (prefs.customThemes || []).some((t) => t.id === key)}
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <div class="delete-theme" onclick={(e) => deleteTheme(key, e)} title="Delete Theme">
                  <Trash2 size={10} />
                </div>
              {/if}
            </button>
          {/if}
        {/each}

        <button type="button" class="theme-card import-card" onclick={importTheme}>
          <div class="preview"><Upload size={16} /></div>
          <span class="theme-name">{tx.pref.importTheme}</span>
        </button>
      </div>
    </div>
  </div>

  <div class="card-group">
    <div class="card-row">
      <div class="info">
        <span class="lbl">{tx.ui.gridIntensity}</span>
      </div>
      <div class="action">
        <div class="slider-row">
          <input
            type="range"
            class="input-range"
            min="0"
            max="1"
            step="0.1"
            value={prefs.gridIntensity}
            oninput={(e) =>
              onpatch({ gridIntensity: Number((e.target as HTMLInputElement).value) })}
          />
          <span class="val">{Math.round(prefs.gridIntensity * 100)}%</span>
        </div>
      </div>
    </div>

    <div class="card-row">
      <div class="info">
        <span class="lbl">{tx.pref.uiScale}</span>
      </div>
      <div class="action">
        <div class="chips">
          {#each scales as s}
            <button
              class="chip"
              class:active={prefs.uiScale === s.v}
              onclick={() => onpatch({ uiScale: s.v })}>{s.label}</button
            >
          {/each}
        </div>
      </div>
    </div>

    <div class="card-row">
      <div class="info">
        <span class="lbl">{tx.global.language}</span>
      </div>
      <div class="action">
        <SegmentControl
          options={[
            { value: "en", label: "EN" },
            { value: "ru", label: "RU" },
            { value: "es", label: "ES" },
            { value: "nl", label: "NL" },
          ]}
          active={prefs.lang}
          onchange={(v) => onpatch({ lang: v as any })}
        />
      </div>
    </div>
  </div>
</div>

<style>
  .tab {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }
  .tab h2 {
    font-size: 1.5rem;
    font-weight: 800;
    margin: 0;
    color: var(--text-primary);
    letter-spacing: -0.02em;
  }

  .header-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.5rem;
  }

  .btn-secondary {
    background: rgba(128, 128, 128, 0.05);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    color: var(--text-primary);
    font-size: 0.8rem;
    font-weight: 600;
    padding: 0.5rem 1rem;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    transition: all 0.2s;
    font-family: inherit;
  }
  .btn-secondary:hover {
    background: rgba(128, 128, 128, 0.1);
    border-color: var(--text-muted);
  }
  .btn-secondary.small {
    padding: 0.35rem 0.75rem;
    font-size: 0.75rem;
  }

  .card-group {
    background: rgba(128, 128, 128, 0.02);
    border: 1px solid var(--border-color);
    border-radius: 12px;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  .card-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem 1.25rem;
    border-bottom: 1px solid var(--border-color);
    gap: 2rem;
  }
  .card-row:last-child {
    border-bottom: none;
  }
  .card-row label {
    font-size: 0.75rem;
    font-weight: 700;
    text-transform: uppercase;
    color: var(--text-muted);
    letter-spacing: 0.1em;
  }

  .card-content {
    padding: 1rem 1.25rem;
    background: rgba(128, 128, 128, 0.04);
    border-bottom: 1px solid var(--border-color);
  }
  .card-content:last-child {
    border-bottom: none;
  }

  .info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
  .lbl {
    font-weight: 600;
    font-size: 0.9rem;
    color: var(--text-primary);
  }
  .action {
    display: flex;
    align-items: center;
    justify-content: flex-end;
  }

  .chips {
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
  }
  .chip {
    background: rgba(128, 128, 128, 0.05);
    border: 1px solid transparent;
    border-radius: 8px;
    color: var(--text-muted);
    font-size: 0.8rem;
    font-weight: 600;
    padding: 0.4rem 0.8rem;
    cursor: pointer;
    transition: all 0.2s;
    font-family: inherit;
  }
  .chip:hover {
    background: rgba(128, 128, 128, 0.1);
    color: var(--text-primary);
  }
  .chip.active {
    background: var(--accent);
    color: var(--accent-text);
    border-color: var(--accent);
  }

  .slider-row {
    display: flex;
    align-items: center;
    gap: 1rem;
    width: 200px;
  }
  .input-range {
    flex: 1;
    height: 4px;
    border-radius: 99px;
    appearance: none;
    background: rgba(128, 128, 128, 0.15);
    outline: none;
  }
  .input-range::-webkit-slider-thumb {
    appearance: none;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: var(--text-primary);
    cursor: pointer;
  }
  .val {
    font-size: 0.8rem;
    font-weight: 600;
    color: var(--accent);
    font-variant-numeric: tabular-nums;
    width: 36px;
    text-align: right;
  }

  .theme-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
    gap: 0.75rem;
  }
  .theme-card {
    background: var(--bg);
    border: 1px solid var(--brd);
    border-radius: 10px;
    padding: 0.6rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    align-items: center;
    cursor: pointer;
    position: relative;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  }
  .theme-card:hover {
    transform: translateY(-2px);
    border-color: var(--text-muted);
  }
  .theme-card.active {
    border-color: var(--acc);
    box-shadow: 0 0 0 1px var(--acc);
  }
  .preview {
    width: 100%;
    height: 36px;
    background: rgba(128, 128, 128, 0.1);
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .dot {
    width: 16px;
    height: 16px;
    border-radius: 50%;
  }
  .theme-card span.theme-name {
    font-size: 0.65rem;
    font-weight: 700;
    text-transform: uppercase;
    color: var(--txt);
    max-width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .check {
    position: absolute;
    top: -6px;
    right: -6px;
    background: var(--acc);
    color: var(--accent-text);
    width: 18px;
    height: 18px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 2px 4px rgba(128, 128, 128, 0.2);
  }

  .delete-theme {
    position: absolute;
    top: -6px;
    left: -6px;
    background: #ef4444;
    color: white;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0;
    transition: opacity 0.2s;
    border: none;
    cursor: pointer;
  }
  .theme-card:hover .delete-theme {
    opacity: 1;
  }

  .import-card {
    background: rgba(128, 128, 128, 0.05);
    border: 1px dashed var(--border-color);
    color: var(--text-muted);
  }
  .import-card:hover {
    background: rgba(128, 128, 128, 0.08);
    border-color: var(--text-muted);
    color: var(--text-primary);
  }
  .import-card .preview {
    background: rgba(128, 128, 128, 0.05);
    color: var(--text-muted);
  }
  .import-card:hover .preview {
    color: var(--accent);
  }
</style>
