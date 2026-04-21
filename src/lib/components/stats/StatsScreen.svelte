<script lang="ts">
  import { getHistory, clearHistory, deleteHistoryItem } from "$lib/historyStore";
  import type { ScrapeHistoryItem } from "$lib/types";
  import type { Translations } from "$lib/i18n";
  import { Trash2, BarChart3, X } from "lucide-svelte";
  import { fade, scale } from "svelte/transition";
  import { cubicOut } from "svelte/easing";

  import StatsOverview from "./StatsOverview.svelte";
  import StatsCharts from "./StatsCharts.svelte";
  import HistoryTable from "./HistoryTable.svelte";
  import HistoryDetailView from "./HistoryDetailView.svelte";
  import { onMount } from "svelte";

  type Props = {
    tx: Translations;
    onclose: () => void;
  };

  let { tx, onclose }: Props = $props();

  let history = $state<ScrapeHistoryItem[]>([]);
  let selectedItem = $state<ScrapeHistoryItem | null>(null);

  onMount(async () => {
    history = await getHistory();
  });

  function backdropKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      e.preventDefault();
      if (selectedItem) {
        selectedItem = null;
      } else {
        onclose();
      }
    }
  }

  const stats = $derived(() => {
    const total = history.length;
    const chars = history.reduce((acc, curr) => acc + curr.total_chars, 0);
    const pages = history.reduce((acc, curr) => acc + curr.pages_scraped, 0);
    const failed = history.reduce((acc, curr) => acc + (curr.pages_failed || 0), 0);
    const data = history.reduce((acc, curr) => acc + curr.total_size_bytes, 0);
    const duration = history.reduce((acc, curr) => acc + curr.duration_ms, 0);

    const successRate = pages + failed > 0 ? (pages / (pages + failed)) * 100 : 100;
    const avgSpeed = history.reduce((acc, curr) => acc + (curr.avg_speed || 0), 0) / (total || 1);

    const domains: Record<string, number> = {};
    history.forEach((h) => {
      try {
        const url = new URL(h.url);
        const domain = url.hostname.replace("www.", "");
        domains[domain] = (domains[domain] || 0) + 1;
      } catch {
        /* ignore */
      }
    });
    const topDomain = Object.entries(domains).sort((a, b) => b[1] - a[1])[0]?.[0] || "—";

    return {
      total,
      chars,
      pages,
      failed,
      data,
      duration,
      successRate,
      avgSpeed,
      topDomain,
    };
  });

  const pagesData = $derived(
    history
      .slice(0, 15)
      .reverse()
      .map((h) => h.pages_scraped),
  );
  const speedData = $derived(
    history
      .slice(0, 15)
      .reverse()
      .map((h) => h.avg_speed || 0),
  );
  const charsData = $derived(
    history
      .slice(0, 15)
      .reverse()
      .map((h) => h.total_chars),
  );

  async function handleDelete(id: string) {
    if (confirm(tx.stats.deleteConfirm)) {
      await deleteHistoryItem(id);
      history = await getHistory();
    }
  }

  async function handleClear() {
    if (confirm(tx.stats.clearHistory)) {
      await clearHistory();
      history = [];
    }
  }
</script>

<div class="modal-root" onkeydown={backdropKeydown} role="presentation">
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="modal-backdrop"
    transition:fade={{ duration: 250 }}
    onclick={onclose}
    role="presentation"
  ></div>

  <div
    class="modal-window scroll-themed"
    transition:scale={{ duration: 400, start: 0.96, easing: cubicOut }}
    role="dialog"
    aria-modal="true"
  >
    <header class="stats-header">
      <div class="title-group">
        <BarChart3 size={24} class="accent-icon" />
        <h1>{selectedItem ? tx.stats.sessionDetail : tx.stats.title}</h1>
      </div>
      <div class="header-actions">
        {#if !selectedItem}
          <button class="btn-clear" onclick={handleClear} disabled={history.length === 0}>
            <Trash2 size={14} />
            {tx.stats.clearBtn}
          </button>
        {/if}
        <button class="btn-close" onclick={onclose}>
          <X size={20} />
        </button>
      </div>
    </header>

    <div class="stats-content">
      {#if selectedItem}
        <HistoryDetailView item={selectedItem} {tx} onback={() => (selectedItem = null)} />
      {:else}
        <StatsOverview stats={stats()} {tx} />
        <StatsCharts {pagesData} {speedData} {charsData} {tx} />
        <HistoryTable
          {history}
          {tx}
          ondelete={handleDelete}
          onselect={(item) => (selectedItem = item)}
        />
      {/if}
    </div>
  </div>
</div>

<style>
  .modal-root {
    position: fixed;
    inset: 0;
    z-index: 5000;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    pointer-events: auto;
  }
  .modal-backdrop {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.45);
    backdrop-filter: blur(4px);
  }

  .modal-window {
    position: relative;
    width: 100%;
    max-width: 1100px;
    max-height: 90vh;
    height: auto;
    background: var(--surface);
    backdrop-filter: blur(24px) saturate(180%);
    -webkit-backdrop-filter: blur(24px) saturate(180%);
    border: 1px solid var(--border-color);
    border-radius: 20px;
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    box-shadow: 0 32px 64px rgba(0, 0, 0, 0.5);
    padding: 2.5rem;
  }

  .stats-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2.5rem;
    width: 100%;
  }
  .title-group {
    display: flex;
    align-items: center;
    gap: 1rem;
  }
  .title-group h1 {
    font-size: 2rem;
    font-weight: 800;
    margin: 0;
    letter-spacing: -0.02em;
    color: var(--text-primary);
  }
  .accent-icon {
    color: var(--accent);
  }
  .header-actions {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .btn-clear {
    background: rgba(239, 68, 68, 0.1);
    color: #f87171;
    border: 1px solid rgba(239, 68, 68, 0.2);
    padding: 0.5rem 1rem;
    border-radius: 8px;
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    transition: all 0.2s;
  }
  .btn-clear:hover:not(:disabled) {
    background: rgba(239, 68, 68, 0.2);
    border-color: rgba(239, 68, 68, 0.4);
  }
  .btn-clear:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-close {
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 0.5rem;
    border-radius: 8px;
    transition: all 0.2s;
  }
  .btn-close:hover {
    background: rgba(128, 128, 128, 0.1);
    color: var(--text-primary);
  }

  .stats-content {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  @media (max-width: 1024px) {
    .modal-root {
      padding: 1rem;
    }
    .modal-window {
      padding: 1.5rem;
    }
  }
</style>
