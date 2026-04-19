<script lang="ts">
  import type { ProgressEvent } from "$lib/types";
  import type { Translations } from "$lib/i18n";
  import { Zap, Globe, Layers, Loader, Clock } from "lucide-svelte";
  import { fly } from "svelte/transition";
  import { expoOut } from "svelte/easing";

  let { 
    isScraping, progress, progressLabel, recursive, liveStats, maxDepth, maxPages, speed, tx 
  }: {
    isScraping: boolean;
    progress: number;
    progressLabel: string;
    recursive: boolean;
    liveStats: ProgressEvent | null;
    maxDepth: number;
    maxPages: number;
    speed: number;
    tx: Translations;
  } = $props();

  const realProgress = $derived.by(() => {
    if (!liveStats) return progress;
    if (liveStats.total_pages > 0) {
      return (liveStats.pages_done / liveStats.total_pages) * 100;
    }
    return progress;
  });

  const eta = $derived.by(() => {
    if (!isScraping || speed <= 0 || !liveStats) return null;
    const remaining = liveStats.total_pages - liveStats.pages_done;
    if (remaining <= 0) return null;
    const seconds = remaining / speed;
    if (seconds < 60) return `${Math.round(seconds)}s`;
    const mins = Math.floor(seconds / 60);
    const secs = Math.round(seconds % 60);
    return `${mins}m ${secs}s`;
  });

  function trimUrl(u: string, max = 52) {
    const s = u.replace(/^https?:\/\//, "");
    return s.length > max ? s.slice(0, max) + "…" : s;
  }
</script>

{#if isScraping || (progress > 0 && progress < 100)}
  <div 
    class="progress-card" 
    class:recursive-mode={recursive}
    transition:fly={{ y: -15, duration: 400, easing: expoOut }}
  >
    <!-- Header row -->
    <div class="progress-head">
      <div class="prog-status">
        <span class="pulse-dot"></span>
        <span class="prog-label">
          {recursive ? tx.status.crawling : progressLabel}
        </span>
      </div>
      {#if recursive && liveStats}
        <div class="pages-badge">
          <Zap size={11} />
          {liveStats.pages_done} / {liveStats.total_pages}
          {#if liveStats.pages_failed > 0}
            <span class="fail-count" title="Failed pages">({liveStats.pages_failed} {tx.results.pagesFailed || 'failed'})</span>
          {/if}
          <span class="badge-muted">· {liveStats.pages_found} {tx.results.pages}</span>
        </div>
      {/if}
    </div>

    <!-- Bar -->
    <div class="progress-track">
      <div class="progress-fill" style="width:{realProgress}%"></div>
    </div>
    <div class="progress-pct-row">
      <div class="eta">
        {#if eta}
          <Clock size={10} /> ETA: {eta}
        {/if}
      </div>
      <div class="progress-pct">{Math.round(realProgress)}%</div>
    </div>

    <!-- Single-page phase -->
    {#if !recursive}
      <div class="phase-row">
        <Loader size={11} />
        <span>{progressLabel}</span>
      </div>
    {:else if liveStats}
      <!-- Recursive detail -->
      {#if liveStats.current_url}
        <div class="current-url-row">
          <Globe size={11} />
          <span class="curr-url-text">{trimUrl(liveStats.current_url)}</span>
        </div>
      {/if}
      <div class="meta-row">
        <span class="meta-chip">
          <Layers size={10} />
          {tx.results.depth} {liveStats.depth}/{maxDepth}
        </span>
        {#if speed > 0}
          <span class="meta-chip">
            <Zap size={10} />
            {speed.toFixed(1)} {tx.results.speed}
          </span>
        {/if}
        <span class="meta-chip">
          {progressLabel}
        </span>
      </div>
    {/if}
  </div>
{/if}

<style>
  .progress-card {
    width: 100%;
    max-width: 800px;
    background: var(--surface);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 0.75rem 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    backdrop-filter: blur(12px);
    animation: slideDown 0.42s cubic-bezier(0.34, 1.56, 0.64, 1) both;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.22);
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-12px) scale(0.98);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  .progress-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
  }

  .prog-status {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .pulse-dot {
    width: 7px; height: 7px;
    background: var(--accent);
    border-radius: 50%;
    box-shadow: 0 0 0 0 var(--accent);
    animation: pulse 1.4s ease-in-out infinite;
    flex-shrink: 0;
  }

  @keyframes pulse {
    0%   { box-shadow: 0 0 0 0 rgba(255,255,255,0.4); }
    70%  { box-shadow: 0 0 0 5px rgba(255,255,255,0); }
    100% { box-shadow: 0 0 0 0 rgba(255,255,255,0); }
  }

  .prog-label {
    font-size: 0.78rem;
    font-weight: 600;
    color: var(--text-primary);
    letter-spacing: 0.02em;
  }

  .pages-badge {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    font-size: 0.72rem;
    font-weight: 600;
    color: var(--accent);
    font-variant-numeric: tabular-nums;
    letter-spacing: 0.02em;
  }

  .badge-muted {
    font-weight: 500;
    color: var(--text-muted);
    font-size: 0.68rem;
  }

  .fail-count {
    color: #f87171;
    font-size: 0.68rem;
    margin: 0 0.2rem;
  }

  /* Bar */
  .progress-track {
    width: 100%;
    height: 2px;
    background: var(--border-color);
    border-radius: 999px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(
      90deg,
      color-mix(in srgb, var(--accent) 85%, #fff),
      var(--accent),
      color-mix(in srgb, var(--accent) 75%, #000)
    );
    background-size: 200% 100%;
    border-radius: 999px;
    transition: width 0.45s cubic-bezier(0.16, 1, 0.3, 1);
    box-shadow: 0 0 12px color-mix(in srgb, var(--accent) 55%, transparent);
    animation: barShimmer 2.2s ease-in-out infinite;
  }

  @keyframes barShimmer {
    0%,
    100% {
      background-position: 0% 50%;
    }
    50% {
      background-position: 100% 50%;
    }
  }

  .progress-pct-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: -0.25rem;
  }

  .eta {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    font-size: 0.68rem;
    color: var(--accent);
    opacity: 0.8;
    font-weight: 600;
  }

  .progress-pct {
    font-size: 0.68rem;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
    text-align: right;
  }

  /* Phase row (single page) */
  .phase-row {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    font-size: 0.76rem;
    color: var(--text-muted);
  }

  /* Recursive detail rows */
  .current-url-row {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    font-size: 0.75rem;
    color: var(--text-muted);
    font-family: ui-monospace, monospace;
    overflow: hidden;
  }

  .curr-url-text {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex: 1;
    color: var(--accent);
    opacity: 0.8;
  }

  .meta-row {
    display: flex;
    gap: 0.4rem;
    flex-wrap: wrap;
  }

  .meta-chip {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    font-size: 0.7rem;
    color: var(--text-muted);
    background: rgba(255,255,255,0.04);
    border: 1px solid var(--border-color);
    padding: 0.15rem 0.45rem;
    border-radius: 4px;
    font-variant-numeric: tabular-nums;
    transition:
      border-color 0.25s,
      background 0.25s,
      transform 0.25s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .meta-chip:hover {
    border-color: color-mix(in srgb, var(--accent) 40%, var(--border-color));
    background: rgba(255, 255, 255, 0.06);
    transform: translateY(-1px);
  }

  :global(html.reduce-motion) .progress-card {
    animation: none;
    box-shadow: none;
  }

  :global(html.reduce-motion) .progress-fill {
    animation: none;
    background: var(--accent);
    background-size: auto;
    transition: width 0.15s linear;
  }

  :global(html.reduce-motion) .pulse-dot {
    animation: none;
    opacity: 0.85;
  }

  :global(html.reduce-motion) .meta-chip {
    transition: none;
  }

  :global(html.reduce-motion) .meta-chip:hover {
    transform: none;
  }
</style>
