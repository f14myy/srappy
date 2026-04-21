<script lang="ts">
  // выводит список ошибок, если что-то пошло не так
  import type { LogEvent } from "$lib/types";
  import type { Translations } from "$lib/i18n";
  import { Trash2, AlertCircle, Clock, Globe, ShieldAlert } from "lucide-svelte";
  import { fade, slide } from "svelte/transition";
  import { cubicOut } from "svelte/easing";

  type Props = {
    tx: Translations;
    errorLogs: LogEvent[];
    onclear?: () => void;
  };

  let { tx, errorLogs, onclear }: Props = $props();

  function fmtUrl(u: string, max = 50) {
    const s = u.replace(/^https?:\/\//, "");
    return s.length > max ? s.slice(0, max) + "…" : s;
  }
</script>

<div class="tab">
  <div class="header-row">
    <div class="title-group">
      <h2>{tx.stats.history || "Logs"}</h2>
      <span class="count-pill">{errorLogs.length}</span>
    </div>
    {#if errorLogs.length > 0}
      <button class="btn-clear" onclick={onclear} transition:fade>
        <Trash2 size={14} />
        {tx.stats.clearBtn || "Clear"}
      </button>
    {/if}
  </div>

  <div class="logs-container scroll-themed">
    {#if errorLogs.length === 0}
      <div class="empty-state" transition:fade>
        <ShieldAlert size={48} class="empty-icon" />
        <p>{tx.stats.noData || "No error logs yet."}</p>
        <span class="hnt">Scraping errors and system crashes will appear here.</span>
      </div>
    {:else}
      <div class="logs-list">
        {#each [...errorLogs].reverse() as log, i (log.url + log.message + i)}
          <div class="log-item" transition:slide={{ duration: 300, easing: cubicOut }}>
            <div class="log-header">
              <div class="log-tag error">ERROR</div>
              <div class="log-url">
                <Globe size={12} />
                <span>{fmtUrl(log.url)}</span>
              </div>
            </div>

            <div class="log-body">
              <AlertCircle size={14} class="err-icon" />
              <p class="message">{log.message || "Unknown error occurred"}</p>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .tab {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    height: 100%;
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
  .title-group {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .count-pill {
    background: rgba(239, 68, 68, 0.1);
    color: #f87171;
    font-size: 0.7rem;
    font-weight: 800;
    padding: 0.2rem 0.6rem;
    border-radius: 99px;
    border: 1px solid rgba(239, 68, 68, 0.2);
  }

  .btn-clear {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: rgba(128, 128, 128, 0.05);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    color: var(--text-muted);
    font-size: 0.75rem;
    font-weight: 600;
    padding: 0.4rem 0.8rem;
    cursor: pointer;
    transition: all 0.2s;
  }
  .btn-clear:hover {
    background: rgba(239, 68, 68, 0.1);
    color: #f87171;
    border-color: rgba(239, 68, 68, 0.3);
  }

  .logs-container {
    flex: 1;
    background: rgba(128, 128, 128, 0.02);
    border: 1px solid var(--border-color);
    border-radius: 12px;
    overflow-y: auto;
    min-height: 300px;
    padding: 1rem;
  }

  .empty-state {
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem;
    text-align: center;
    color: var(--text-muted);
  }
  .empty-state :global(.empty-icon) {
    opacity: 0.1;
    margin-bottom: 1rem;
  }
  .empty-state p {
    font-weight: 600;
    margin: 0 0 0.5rem 0;
    color: var(--text-primary);
  }
  .empty-state .hnt {
    font-size: 0.75rem;
    opacity: 0.6;
  }

  .logs-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .log-item {
    background: rgba(128, 128, 128, 0.03);
    border: 1px solid var(--border-color);
    border-radius: 10px;
    padding: 0.85rem;
    display: flex;
    flex-direction: column;
    gap: 0.6rem;
    transition: all 0.2s;
  }
  .log-item:hover {
    background: rgba(128, 128, 128, 0.05);
    border-color: rgba(255, 255, 255, 0.1);
  }

  .log-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-variant-numeric: tabular-nums;
  }

  .log-tag {
    font-size: 0.6rem;
    font-weight: 900;
    padding: 0.1rem 0.4rem;
    border-radius: 4px;
    letter-spacing: 0.05em;
  }
  .log-tag.error {
    background: #ef4444;
    color: white;
  }

  .log-url {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--text-muted);
    overflow: hidden;
  }
  .log-url span {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .log-body {
    display: flex;
    align-items: flex-start;
    gap: 0.6rem;
    padding: 0.4rem 0.6rem;
    background: rgba(0, 0, 0, 0.2);
    border-radius: 6px;
    border-left: 2px solid #ef4444;
  }
  .log-body :global(.err-icon) {
    color: #f87171;
    flex-shrink: 0;
    margin-top: 0.1rem;
  }
  .message {
    margin: 0;
    font-size: 0.8rem;
    color: var(--text-primary);
    line-height: 1.5;
    word-break: break-word;
    font-family: "JetBrains Mono", "Consolas", monospace;
  }
</style>
