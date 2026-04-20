<script lang="ts">
  import {
    Clock, FileText, Hash, Database, Zap, Layers,
    Download, Copy, Check, CheckCircle, X, ChevronDown
  } from "lucide-svelte";
  import type { ScrapeResult } from "$lib/types";
  import type { Translations } from "$lib/i18n";
  import type { AppPreferences } from "$lib/appPreferences";

  type Props = {
    result: ScrapeResult;
    recursive: boolean;
    maxDepth: number;
    speed: number;
    speedHistory: number[];
    tx: Translations;
    viewMode: "text" | "table";
    copied: boolean;
    exportMenuOpen: boolean;
    id: string;
    send: any;
    receive: any;
    defaultFormat: AppPreferences["defaultExportFormat"];
    oncopy: () => void;
    onminimize: () => void;
    ontoggleview: (mode: "text" | "table") => void;
    ontoggleexport: () => void;
    onsave: (format: any) => void;
  };

  let { 
    result, recursive, maxDepth, speed, speedHistory, tx, viewMode, 
    copied, exportMenuOpen, id, send, receive, defaultFormat,
    oncopy, onminimize, ontoggleview, ontoggleexport, onsave
  }: Props = $props();

  function fmtTime(ms: number) {
    if (ms < 1000) return `${ms}ms`;
    const s = Math.floor(ms / 100) / 10;
    if (s < 60) return `${s}s`;
    const m = Math.floor(s / 60);
    const sec = Math.floor(s % 60);
    if (m < 60) return `${m}m ${sec}s`;
    const h = Math.floor(m / 60);
    const min = m % 60;
    return `${h}h ${min}m`;
  }

  function fmtSize(bytes: number) {
    if (bytes < 1024) return `${bytes} B`;
    const kb = bytes / 1024;
    if (kb < 1024) return `${kb.toFixed(1)} KB`;
    return `${(kb / 1024).toFixed(2)} MB`;
  }

  function fmtNum(n: number) {
    return n >= 1000 ? (n / 1000).toFixed(1) + "K" : n.toString();
  }

  const avgChars = $derived(
    result.pages_scraped > 0 ? Math.round(result.total_chars / result.pages_scraped) : 0
  );

  const avgTime = $derived(
    result.pages_scraped > 0 ? Math.round(result.load_time_ms / result.pages_scraped) : 0
  );

  const sparklineData = $derived(() => {
    if (!speedHistory || speedHistory.length < 2) return "";
    const max = Math.max(...speedHistory, 1);
    const w = 60;
    const h = 18;
    const step = w / (speedHistory.length - 1);
    let path = `M 0 ${h - (speedHistory[0] / max) * h}`;
    for (let i = 1; i < speedHistory.length; i++) {
        path += ` L ${i * step} ${h - (speedHistory[i] / max) * h}`;
    }
    return path;
  });
</script>

<div 
  class="summary-card"
>
  <div class="summary-left">
    <CheckCircle size={14} class="summary-check" color="#4ade80" />
    <span class="summary-title">{tx.results.completed}</span>
  </div>

  <div class="summary-stats">
    <div class="stat-item">
      <Clock size={12} />
      <span title="{result.load_time_ms}ms">{fmtTime(result.load_time_ms)}</span>
    </div>
    <div class="stat-sep">·</div>

    {#if result.pages_scraped > 1}
      <div class="stat-item accent">
        <FileText size={12} />
        <span>{result.pages_scraped} {tx.results.pagesScraped}</span>
      </div>
      <div class="stat-sep">·</div>
    {/if}

    <div class="stat-item">
      <Hash size={12} />
      <span>{fmtNum(result.total_chars)} {tx.results.chars}</span>
    </div>
    <div class="stat-sep">·</div>

    <div class="stat-item">
      <Database size={12} />
      <span>{fmtSize(result.total_size_bytes)}</span>
    </div>

    {#if (result.pages_failed ?? 0) > 0}
      <div class="stat-sep">·</div>
      <div class="stat-item warn">
        <span>{result.pages_failed} {tx.results.pagesFailed}</span>
      </div>
    {/if}

    {#if recursive && result.pages_scraped > 1}
      <div class="stat-sep">·</div>
      <div class="stat-item">
        <Layers size={12} />
        <span>{tx.results.depth}: {maxDepth}</span>
      </div>
      {#if speed > 0}
        <div class="stat-sep">·</div>
        <div class="stat-item">
          <Zap size={12} />
          <span>{speed.toFixed(1)} {tx.results.speed}</span>
        </div>
      {/if}
      {#if speedHistory && speedHistory.length > 1}
        <div class="stat-sep">·</div>
        <div class="stat-item" title="Speed history">
          <svg width="60" height="18" viewBox="0 0 60 18" preserveAspectRatio="none">
             <path d={sparklineData()} fill="none" stroke="var(--accent)" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </div>
      {/if}
      <div class="stat-sep">·</div>
      <div class="stat-item muted">
        <span>⌀ {fmtNum(avgChars)} {tx.results.chars}/pg</span>
      </div>
      <div class="stat-sep">·</div>
      <div class="stat-item muted">
        <span>⌀ {fmtTime(avgTime)}/pg</span>
      </div>
    {/if}
  </div>

  <div class="summary-actions">
    {#if result.pages_scraped > 1}
      <div class="view-toggles">
         <button class="view-btn" class:active={viewMode === 'text'} onclick={() => ontoggleview('text')}>
           <FileText size={12}/> Text
         </button>
         <button class="view-btn" class:active={viewMode === 'table'} onclick={() => ontoggleview('table')}>
           <Database size={12}/> Table
         </button>
      </div>
      <div style="width: 2px;"></div>
    {/if}

    <button class="icon-action" onclick={oncopy} title={tx.global.copy}>
      {#if copied}
        <Check size={13} />
        {tx.global.copied}
      {:else}
        <Copy size={13} />
        {tx.global.copy}
      {/if}
    </button>

    <div class="export-dropdown-wrapper" style="position: relative;">
      <div class="split-btn">
        <button id="save-btn" class="icon-action primary split-left" onclick={() => onsave(defaultFormat)} title="Quick Save (.{defaultFormat.toUpperCase()})">
          <Download size={13} />
          {result.pages_scraped > 1 ? tx.results.exportAll : tx.global.save}
        </button>
        <button class="icon-action primary split-right" onclick={ontoggleexport} aria-label="Export options">
          <span class="save-chev" class:open={exportMenuOpen}>
            <ChevronDown size={13} />
          </span>
        </button>
      </div>
      
      {#if exportMenuOpen}
        <div class="export-menu">
          <button class="export-item" class:is-default={defaultFormat === 'txt'} onclick={() => onsave("txt")}>
            <span class="ext">.TXT</span> Plain Text {#if defaultFormat === 'txt'}<span class="def-badge">DEF</span>{/if}
          </button>
          <button class="export-item" class:is-default={defaultFormat === 'json'} onclick={() => onsave("json")}>
            <span class="ext">.JSON</span> Structured Array {#if defaultFormat === 'json'}<span class="def-badge">DEF</span>{/if}
          </button>
          <button class="export-item" class:is-default={defaultFormat === 'csv'} onclick={() => onsave("csv")}>
            <span class="ext">.CSV</span> Spreadsheet {#if defaultFormat === 'csv'}<span class="def-badge">DEF</span>{/if}
          </button>
          <button class="export-item" class:is-default={defaultFormat === 'csv_meta'} onclick={() => onsave("csv_meta")}>
            <span class="ext">.CSV</span> {tx.session.exportCsvMeta} {#if defaultFormat === 'csv_meta'}<span class="def-badge">DEF</span>{/if}
          </button>
          <button class="export-item" class:is-default={defaultFormat === 'md'} onclick={() => onsave("md")}>
            <span class="ext">.MD</span> Markdown {#if defaultFormat === 'md'}<span class="def-badge">DEF</span>{/if}
          </button>
          {#if result.crawl_state}
            <button class="export-item" style="color: #4ade80;" class:is-default={defaultFormat === 'srappy'} onclick={() => onsave("srappy")}>
              <span class="ext" style="color: #4ade80; background: rgba(74, 222, 128, 0.1);">.SESS</span> {tx.session.saveSession} {#if defaultFormat === 'srappy'}<span class="def-badge" style="background: #4ade80; color: #000;">DEF</span>{/if}
            </button>
          {/if}
        </div>
      {/if}
    </div>
    
    <button class="icon-action" onclick={onminimize} title="Minimize to bar" aria-label="Minimize results">
      <X size={14} />
    </button>
  </div>
</div>

<style>
  .summary-card {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.65rem 1rem;
    background: var(--surface);
    border: 1px solid var(--border-color);
    border-bottom: none;
    border-radius: 10px 10px 0 0;
    backdrop-filter: blur(12px);
    flex-wrap: wrap;
    position: relative;
    z-index: 100;
  }

  .summary-left {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    flex-shrink: 0;
  }

  .summary-title {
    font-size: 0.78rem;
    font-weight: 700;
    color: #4ade80;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .summary-stats {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    flex: 1;
    flex-wrap: wrap;
  }

  .stat-item {
    display: flex;
    align-items: center;
    gap: 0.28rem;
    font-size: 0.76rem;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
    white-space: nowrap;
  }

  .stat-item.accent { color: var(--accent); }
  .stat-item.muted  { color: var(--text-muted); opacity: 0.7; }
  .stat-item.warn { color: #fbbf24; }

  .stat-sep {
    font-size: 0.72rem;
    color: var(--border-color);
    padding: 0 0.1rem;
  }

  .summary-actions {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    margin-left: auto;
    flex-shrink: 0;
  }

  .icon-action {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    padding: 0.28rem 0.65rem;
    background: transparent;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    color: var(--text-muted);
    font-size: 0.75rem;
    font-weight: 500;
    font-family: inherit;
    cursor: pointer;
    transition: all 0.15s;
    white-space: nowrap;
  }

  .icon-action:hover {
    border-color: var(--text-muted);
    color: var(--text-primary);
    background: rgba(255,255,255,0.04);
  }

  .icon-action.primary {
    background: var(--accent);
    color: var(--accent-text);
    border-color: transparent;
  }

  .icon-action.primary:hover {
    opacity: 0.85;
  }

  .split-btn {
    display: flex;
    align-items: stretch;
  }

  .split-left {
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
    border-right: 1px solid rgba(0,0,0,0.1) !important;
  }

  .split-right {
    border-top-left-radius: 0;
    border-bottom-left-radius: 0;
    padding: 0.28rem 0.35rem !important;
  }

  .save-chev {
    display: inline-flex;
    transition: transform 0.35s cubic-bezier(0.34, 1.56, 0.64, 1), opacity 0.2s;
  }

  .save-chev.open {
    transform: rotate(180deg);
  }

  .view-toggles {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    background: rgba(0,0,0,0.2);
    border-radius: 6px;
    padding: 2px;
  }
  
  .view-btn {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    padding: 0.2rem 0.6rem;
    background: transparent;
    border: none;
    color: var(--text-muted);
    border-radius: 4px;
    font-size: 0.72rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s;
    font-family: inherit;
  }
  
  .view-btn.active {
    background: rgba(255,255,255,0.1);
    color: var(--text-primary);
  }
  
  .view-btn:hover:not(.active) {
    color: var(--text-primary);
  }

  .export-menu {
    position: absolute;
    top: calc(100% + 4px);
    right: 0;
    min-width: 200px;
    width: max-content;
    max-width: min(280px, 92vw);
    background: var(--surface);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 0.35rem;
    box-shadow: 0 4px 20px rgba(0,0,0,0.6);
    z-index: 9999;
    display: flex;
    flex-direction: column;
    gap: 2px;
    animation: menuFadeIn 0.28s cubic-bezier(0.34, 1.56, 0.64, 1) both;
    transform-origin: top right;
  }

  @keyframes menuFadeIn {
    from {
      opacity: 0;
      transform: translateY(-6px) scale(0.94);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  .export-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: 100%;
    padding: 0.4rem 0.5rem;
    background: transparent;
    border: none;
    border-radius: 4px;
    color: var(--text-primary);
    font-size: 0.75rem;
    font-family: inherit;
    text-align: left;
    cursor: pointer;
    transition: background 0.1s;
  }

  .export-item:hover {
    background: rgba(255,255,255,0.06);
  }

  .export-item .ext {
    font-size: 0.65rem;
    font-family: monospace;
    font-weight: 700;
    color: var(--accent);
    background: rgba(0,0,0,0.2);
    padding: 0.1rem 0.3rem;
    border-radius: 3px;
    width: 40px;
    text-align: center;
  }

  .def-badge {
    margin-left: auto;
    font-size: 0.6rem;
    font-weight: 800;
    background: var(--accent);
    color: var(--accent-text);
    padding: 1px 4px;
    border-radius: 3px;
    opacity: 0.8;
  }
</style>
