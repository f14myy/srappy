<script lang="ts">
  import { ChevronLeft, ChevronRight, Globe, ChevronsRight } from "lucide-svelte";
  import type { ScrapeResult, PageResult } from "$lib/types";
  import type { Translations } from "$lib/i18n";

  type Props = {
    result: ScrapeResult;
    tx: Translations;
    selectedPage: number;
    viewMode: "text" | "table";
    currentPage: PageResult | null;
    onprev: () => void;
    onnext: () => void;
    onselect: (i: number) => void;
  };

  let { result, tx, selectedPage, viewMode, currentPage, onprev, onnext, onselect }: Props = $props();

  function fmtTime(ms: number) {
    if (ms < 1000) return `${ms}ms`;
    const s = Math.floor(ms / 100) / 10;
    return `${s}s`;
  }

  function fmtSize(bytes: number) {
    if (bytes < 1024) return `${bytes} B`;
    const kb = bytes / 1024;
    return `${kb.toFixed(1)} KB`;
  }

  function fmtNum(n: number) {
    return n >= 1000 ? (n / 1000).toFixed(1) + "K" : n.toString();
  }

  function fmtUrl(u: string, max = 60) {
    const s = u.replace(/^https?:\/\//, "");
    return s.length > max ? s.slice(0, max) + "…" : s;
  }
</script>

{#if result.pages_scraped > 1}
  <div class="page-header">
    {#if viewMode === 'text'}
      <div class="page-nav scroll-themed">
        <button class="nav-arrow" onclick={onprev} disabled={selectedPage <= -1} aria-label="Previous">
          <ChevronLeft size={13} />
        </button>

        <button
          class="page-pill"
          class:active={selectedPage === -1}
          onclick={() => onselect(-1)}
        >
          {tx.results.allPages}
        </button>

        {#each result.pages as _, i}
          <button
            class="page-pill page-num"
            class:active={selectedPage === i}
            onclick={() => onselect(i)}
            title={result.pages[i].url}
          >{i + 1}</button>
        {/each}

        <button class="nav-arrow" onclick={onnext} disabled={selectedPage >= result.pages.length - 1} aria-label="Next">
          <ChevronRight size={13} />
        </button>
      </div>
    {/if}

    {#if currentPage}
      <div class="page-meta">
        <Globe size={11} />
        <span class="page-url">{fmtUrl(currentPage.url)}</span>
        <span class="page-stats">
          {fmtNum(currentPage.char_count)} {tx.results.chars}
          &nbsp;·&nbsp;
          {fmtSize(currentPage.size_bytes)}
          &nbsp;·&nbsp;
          {fmtTime(currentPage.load_time_ms)}
        </span>
      </div>
    {:else}
      <div class="page-meta muted">
        <ChevronsRight size={11} />
        <span>All {result.pages_scraped} pages combined</span>
      </div>
    {/if}
  </div>
{/if}

<style>
  .page-header {
    display: flex;
    flex-direction: column;
    gap: 0;
    border: 1px solid var(--border-color);
    border-top: none;
    border-bottom: none;
    background: var(--surface);
    backdrop-filter: blur(12px);
  }

  .page-nav {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.45rem 0.75rem;
    border-bottom: 1px solid var(--border-color);
    overflow-x: auto;
    flex-wrap: nowrap;
  }

  .nav-arrow {
    background: transparent;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--text-muted);
    padding: 0.15rem 0.25rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    transition: all 0.15s;
    flex-shrink: 0;
  }

  .nav-arrow:hover:not(:disabled) {
    color: var(--text-primary);
    border-color: var(--text-muted);
  }

  .nav-arrow:disabled { opacity: 0.3; cursor: default; }

  .page-pill {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.18rem 0.55rem;
    border-radius: 4px;
    border: 1px solid var(--border-color);
    background: transparent;
    color: var(--text-muted);
    font-size: 0.72rem;
    font-weight: 500;
    font-family: inherit;
    cursor: pointer;
    transition: all 0.12s;
    white-space: nowrap;
    flex-shrink: 0;
  }

  .page-pill.page-num { min-width: 26px; }

  .page-pill.active {
    background: var(--accent);
    color: var(--accent-text);
    border-color: var(--accent);
  }

  .page-pill:not(.active):hover {
    color: var(--text-primary);
    border-color: var(--text-muted);
  }

  .page-meta {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    padding: 0.35rem 0.75rem;
    font-size: 0.71rem;
    color: var(--text-muted);
    overflow: hidden;
  }

  .page-meta.muted { opacity: 0.6; }

  .page-url {
    font-family: ui-monospace, monospace;
    color: var(--accent);
    opacity: 0.8;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex: 1;
  }

  .page-stats {
    color: var(--text-muted);
    white-space: nowrap;
    font-variant-numeric: tabular-nums;
    flex-shrink: 0;
  }
</style>
