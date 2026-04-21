<script lang="ts">
  import type { ScrapeHistoryItem } from "$lib/types";
  import type { Translations } from "$lib/i18n";
  import {
    ArrowLeft,
    Globe,
    Clock,
    FileText,
    Hash,
    Database,
    Zap,
    Layers,
    CheckCircle2,
    AlertCircle,
    Calendar,
    Info,
  } from "lucide-svelte";
  import { fade, fly } from "svelte/transition";
  import StatCard from "./StatCard.svelte";

  type Props = {
    item: ScrapeHistoryItem;
    tx: Translations;
    onback: () => void;
  };

  let { item, tx, onback }: Props = $props();

  function fmtSize(bytes: number) {
    if (bytes < 1024) return `${bytes} B`;
    const kb = bytes / 1024;
    if (kb < 1024) return `${kb.toFixed(1)} KB`;
    return `${(kb / 1024).toFixed(2)} MB`;
  }

  function fmtNum(n: number) {
    return n >= 1000 ? (n / 1000).toFixed(1) + "K" : n.toString();
  }

  function fmtTime(ms: number) {
    if (ms < 1000) return `${ms}ms`;
    const s = Math.floor(ms / 100) / 10;
    return `${s}s`;
  }

  const avgChars = $derived(
    item.pages_scraped > 0 ? Math.round(item.total_chars / item.pages_scraped) : 0,
  );
  const avgSize = $derived(
    item.pages_scraped > 0 ? Math.round(item.total_size_bytes / item.pages_scraped) : 0,
  );
  const dataDensity = $derived(
    item.total_size_bytes > 0 ? (item.total_chars / item.total_size_bytes).toFixed(2) : "0",
  );
  const successRate = $derived(
    (item.pages_scraped / (item.pages_scraped + (item.pages_failed || 0))) * 100,
  );

  // Estimates
  const readTimeMin = $derived(Math.round(item.total_chars / 1500)); // ~250 words/min * 6 chars/word
</script>

<div class="detail-view" in:fly={{ x: 20, duration: 400 }} out:fade={{ duration: 200 }}>
  <button class="back-btn" onclick={onback}>
    <ArrowLeft size={16} />
    {tx.stats.backToHistory}
  </button>

  <header class="detail-header">
    <div class="url-badge">
      <Globe size={14} />
      <span class="url-text">{item.url}</span>
    </div>
    <div class="date-badge">
      <Calendar size={14} />
      <span>{new Date(item.timestamp).toLocaleString()}</span>
    </div>
  </header>

  <div class="detail-grid">
    <div class="main-stats">
      <StatCard title={tx.results.pages} value={item.pages_scraped} icon={FileText} />
      <StatCard title={tx.results.chars} value={fmtNum(item.total_chars)} icon={Hash} />
      <StatCard title="Total Size" value={fmtSize(item.total_size_bytes)} icon={Database} />
      <StatCard
        title={tx.results.speed}
        value={`${(item.avg_speed || 0).toFixed(1)} p/s`}
        icon={Zap}
      />
    </div>

    <div class="detailed-metrics">
      <div class="metrics-column">
        <h3 class="section-title"><Info size={14} /> {tx.stats.overview}</h3>
        <div class="metric-row">
          <span>{tx.stats.duration}:</span>
          <strong>{fmtTime(item.duration_ms)}</strong>
        </div>
        <div class="metric-row">
          <span>{tx.stats.successRate}:</span>
          <strong class:warn={successRate < 90}>{successRate.toFixed(1)}%</strong>
        </div>
        <div class="metric-row">
          <span>{tx.stats.dataDensity} (char/byte):</span>
          <strong>{dataDensity}</strong>
        </div>
        <div class="metric-row">
          <span>Est. Reading Time:</span>
          <strong>~{readTimeMin} min</strong>
        </div>
      </div>

      <div class="metrics-column">
        <h3 class="section-title"><Layers size={14} /> Crawler Info</h3>
        <div class="metric-row">
          <span>{tx.stats.recursiveStatus}:</span>
          <strong class="mode-tag" class:active={item.recursive}>
            {item.recursive ? tx.stats.recursionOn : tx.stats.recursionOff}
          </strong>
        </div>
        {#if item.recursive}
          <div class="metric-row">
            <span>{tx.stats.depthReached}:</span>
            <strong>{item.max_depth || 1}</strong>
          </div>
        {/if}
        <div class="metric-row">
          <span>{tx.stats.average} {tx.stats.perPage}:</span>
          <strong>{fmtNum(avgChars)} chars</strong>
        </div>
        <div class="metric-row">
          <span>{tx.stats.average} Size:</span>
          <strong>{fmtSize(avgSize)}</strong>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .detail-view {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .back-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: transparent;
    border: none;
    color: var(--accent);
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    padding: 0.5rem 0;
    width: fit-content;
    transition: opacity 0.2s;
  }
  .back-btn:hover {
    opacity: 0.8;
  }

  .detail-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .url-badge,
  .date-badge {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.4rem 0.75rem;
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    font-size: 0.8rem;
    color: var(--text-muted);
  }
  .url-text {
    color: var(--accent);
    font-family: ui-monospace, monospace;
    font-weight: 600;
  }

  .detail-grid {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .main-stats {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }

  .detailed-metrics {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
    background: rgba(255, 255, 255, 0.01);
    border: 1px solid var(--border-color);
    padding: 1.5rem;
    border-radius: 16px;
  }

  .metrics-column {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  .section-title {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin: 0 0 0.5rem;
    font-size: 0.85rem;
    color: var(--text-primary);
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .metric-row {
    display: flex;
    justify-content: space-between;
    font-size: 0.85rem;
    color: var(--text-muted);
  }
  .metric-row strong {
    color: var(--text-primary);
  }
  .metric-row strong.warn {
    color: #f87171;
  }

  .mode-tag {
    padding: 0.1rem 0.4rem;
    border-radius: 4px;
    background: rgba(255, 255, 255, 0.05);
    font-size: 0.75rem;
  }
  .mode-tag.active {
    background: rgba(74, 222, 128, 0.1);
    color: #4ade80;
  }
</style>
