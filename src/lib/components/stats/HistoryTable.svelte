<script lang="ts">
  import type { ScrapeHistoryItem } from "$lib/types";
  import type { Translations } from "$lib/i18n";
  import { Search, Globe, Trash2, ArrowRight } from "lucide-svelte";
  import { slide } from "svelte/transition";

  type Props = {
    history: ScrapeHistoryItem[];
    tx: Translations;
    ondelete: (id: string) => void;
    onselect: (item: ScrapeHistoryItem) => void;
  };

  let { history, tx, ondelete, onselect }: Props = $props();
  let searchQuery = $state("");

  const filteredHistory = $derived(
    history.filter(item => item.url.toLowerCase().includes(searchQuery.toLowerCase()))
  );

  function fmtSize(bytes: number) {
    if (bytes < 1024) return `${bytes} B`;
    const kb = bytes / 1024;
    return `${kb.toFixed(1)} KB`;
  }

  function fmtNum(n: number) {
    return n >= 1000 ? (n / 1000).toFixed(1) + "K" : n.toString();
  }
</script>

<section class="history-section">
  <div class="section-header">
    <div class="search-box">
      <Search size={16} />
      <input 
        type="text" 
        placeholder={tx.stats.searchPlaceholder} 
        bind:value={searchQuery}
      />
    </div>
  </div>

  <div class="history-table-wrapper">
    <table class="history-table">
      <thead>
        <tr>
          <th>{tx.stats.date}</th>
          <th>URL</th>
          <th class="r-align">{tx.results.pages}</th>
          <th class="r-align">{tx.results.chars}</th>
          <th class="r-align">Size</th>
          <th class="r-align">{tx.results.speed}</th>
          <th class="c-align"></th>
        </tr>
      </thead>
      <tbody>
        {#each filteredHistory as item (item.id)}
          <tr transition:slide={{ duration: 200 }} onclick={() => onselect(item)}>
            <td class="date-cell">{new Date(item.timestamp).toLocaleString()}</td>
            <td class="url-cell" title={item.url}>
              <div class="url-wrap">
                <Globe size={12} />
                <span>{item.url}</span>
              </div>
            </td>
            <td class="r-align">{item.pages_scraped}</td>
            <td class="r-align">{fmtNum(item.total_chars)}</td>
            <td class="r-align">{fmtSize(item.total_size_bytes)}</td>
            <td class="r-align">{(item.avg_speed || 0).toFixed(1)}</td>
            <td class="c-align">
              <div class="row-actions">
                <button class="btn-detail" onclick={(e) => { e.stopPropagation(); onselect(item); }}>
                    <ArrowRight size={14} />
                </button>
                <button class="btn-delete-row" onclick={(e) => { e.stopPropagation(); ondelete(item.id); }}>
                    <Trash2 size={14} />
                </button>
              </div>
            </td>
          </tr>
        {:else}
          <tr>
            <td colspan="7" class="no-data">{tx.stats.noData}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</section>

<style>
  .history-section { background: var(--surface); border: 1px solid var(--border-color); border-radius: 12px; overflow: hidden; margin-bottom: 2rem; }
  .section-header { padding: 1rem 1.5rem; border-bottom: 1px solid var(--border-color); display: flex; align-items: center; }
  .search-box { display: flex; align-items: center; gap: 0.75rem; background: rgba(0,0,0,0.2); border: 1px solid var(--border-color); border-radius: 8px; padding: 0.4rem 0.8rem; width: 320px; transition: all 0.2s; }
  .search-box:focus-within { border-color: var(--accent); width: 400px; }
  .search-box input { background: transparent; border: none; color: var(--text-primary); font-size: 0.85rem; width: 100%; outline: none; }
  .history-table-wrapper { width: 100%; overflow-x: auto; }
  .history-table { width: 100%; border-collapse: collapse; font-size: 0.85rem; }
  .history-table th { text-align: left; padding: 0.75rem 1.5rem; color: var(--text-muted); font-weight: 700; font-size: 0.75rem; text-transform: uppercase; letter-spacing: 0.05em; border-bottom: 1px solid var(--border-color); background: rgba(0,0,0,0.1); }
  .history-table td { padding: 0.75rem 1.5rem; border-bottom: 1px solid rgba(255,255,255,0.03); color: var(--text-primary); }
  .history-table tr { cursor: pointer; transition: background 0.15s; }
  .history-table tr:hover td { background: rgba(255,255,255,0.02); }
  .url-wrap { display: flex; align-items: center; gap: 0.5rem; color: var(--accent); font-family: ui-monospace, monospace; font-size: 0.8rem; }
  .url-wrap span { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .date-cell { color: var(--text-muted); font-size: 0.75rem; white-space: nowrap; }
  .r-align { text-align: right; font-variant-numeric: tabular-nums; }
  .c-align { text-align: center; width: 90px; }
  
  .row-actions { display: flex; align-items: center; justify-content: center; gap: 0.5rem; }
  
  .btn-delete-row, .btn-detail { background: transparent; border: none; color: var(--text-muted); cursor: pointer; padding: 0.4rem; border-radius: 6px; transition: all 0.15s; display: flex; align-items: center; }
  .btn-delete-row:hover { background: rgba(239, 68, 68, 0.15); color: #f87171; }
  .btn-detail:hover { background: rgba(255, 255, 255, 0.08); color: var(--accent); }
  
  .no-data { text-align: center; padding: 4rem 0 !important; color: var(--text-muted); font-style: italic; }
</style>
