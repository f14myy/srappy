<script lang="ts">
  import type { ScrapeResult } from "$lib/types";

  type Props = {
    result: ScrapeResult;
    viewMode: "text" | "table";
    displayText: string;
    searchQuery: string;
    onselect: (i: number) => void;
  };

  let { result, viewMode, displayText, searchQuery, onselect }: Props = $props();

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

  function fmtUrl(u: string, max = 80) {
    const s = u.replace(/^https?:\/\//, "");
    return s.length > max ? s.slice(0, max) + "…" : s;
  }

  function escapeHtml(unsafe: string) {
    return unsafe
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;")
      .replace(/'/g, "&#039;");
  }

  const highlightedText = $derived(() => {
    const escaped = escapeHtml(displayText);
    if (!searchQuery.trim()) return escaped;

    try {
      const regex = new RegExp(`(${searchQuery.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")})`, "gi");
      return escaped.replace(regex, "<mark>$1</mark>");
    } catch {
      return escaped;
    }
  });

  const filteredPages = $derived(
    searchQuery.trim()
      ? result.pages
          .map((p, i) => ({ ...p, originalIndex: i }))
          .filter(
            (p) =>
              p.text.toLowerCase().includes(searchQuery.toLowerCase()) ||
              p.url.toLowerCase().includes(searchQuery.toLowerCase()),
          )
      : result.pages.map((p, i) => ({ ...p, originalIndex: i })),
  );
</script>

{#if viewMode === "text"}
  <div class="text-area scroll-themed">
    <pre><code>{@html highlightedText()}</code></pre>
  </div>
{:else}
  <div class="table-area scroll-themed">
    <table class="data-table">
      <thead>
        <tr>
          <th style="width: 50px;">#</th>
          <th>URL</th>
          <th class="r-align">Chars</th>
          <th class="r-align">Size</th>
          <th class="r-align">Time</th>
        </tr>
      </thead>
      <tbody>
        {#each filteredPages as p}
          <tr tabindex="0" onclick={() => onselect(p.originalIndex)}>
            <td class="muted">{p.originalIndex + 1}</td>
            <td class="td-url" title={p.url}>{fmtUrl(p.url)}</td>
            <td class="r-align">{fmtNum(p.char_count)}</td>
            <td class="r-align">{fmtSize(p.size_bytes)}</td>
            <td class="r-align">{fmtTime(p.load_time_ms)}</td>
          </tr>
        {/each}
        {#if filteredPages.length === 0}
          <tr>
            <td colspan="5" style="text-align: center; padding: 2rem; color: var(--text-muted);">
              No results matching "{searchQuery}"
            </td>
          </tr>
        {/if}
      </tbody>
    </table>
  </div>
{/if}

<style>
  .text-area {
    border: 1px solid var(--border-color);
    border-top: none;
    border-radius: 0 0 10px 10px;
    background: var(--surface);
    backdrop-filter: blur(12px);
    overflow-y: auto;
    padding: 1rem 1.25rem;
    flex: 1;
    min-height: 0;
    animation: bodyReveal 0.45s cubic-bezier(0.16, 1, 0.3, 1) 0.08s both;
  }

  pre {
    margin: 0;
    white-space: pre-wrap;
    word-break: break-word;
    font-family: ui-monospace, "Cascadia Code", "Fira Code", monospace;
    font-size: 0.8rem;
    line-height: 1.65;
    color: var(--text-primary);
  }

  :global(mark) {
    background: rgba(250, 250, 250, 0.25);
    color: #fff;
    padding: 0 1px;
    border-radius: 2px;
    box-shadow: 0 0 4px rgba(255, 255, 255, 0.1);
  }

  .table-area {
    border: 1px solid var(--border-color);
    border-top: none;
    border-radius: 0 0 10px 10px;
    background: var(--surface);
    backdrop-filter: blur(12px);
    overflow-y: auto;
    flex: 1;
    min-height: 0;
    animation: bodyReveal 0.45s cubic-bezier(0.16, 1, 0.3, 1) 0.08s both;
  }

  @keyframes bodyReveal {
    from {
      opacity: 0;
      transform: translateY(6px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .data-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.76rem;
  }

  .data-table th {
    text-align: left;
    padding: 0.5rem 1rem;
    color: var(--text-muted);
    font-weight: 600;
    border-bottom: 1px solid var(--border-color);
    position: sticky;
    top: 0;
    background: rgba(8, 8, 8, 0.95);
    backdrop-filter: blur(12px);
    z-index: 2;
  }

  .data-table td {
    padding: 0.5rem 1rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    color: var(--text-primary);
  }

  .data-table tr {
    cursor: pointer;
    transition: background 0.15s;
  }

  .data-table tr:hover {
    background: rgba(255, 255, 255, 0.04);
  }

  .data-table .td-url {
    font-family: ui-monospace, monospace;
    color: var(--accent);
    opacity: 0.85;
    max-width: 300px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .r-align {
    text-align: right;
    font-variant-numeric: tabular-nums;
  }
  .muted {
    color: var(--text-muted);
  }

  :global(html.reduce-motion) .text-area,
  :global(html.reduce-motion) .table-area {
    animation: none;
  }
</style>
