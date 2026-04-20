<script lang="ts">
  import type { ScrapeSession } from "$lib/types";
  import type { Translations } from "$lib/i18n";
  import { ChevronUp, X, Globe, FileText } from "lucide-svelte";
  import { fly } from "svelte/transition";
  import { quintOut } from "svelte/easing";
  import type { TransitionConfig } from "svelte/transition";

  type Props = {
    session: ScrapeSession;
    tx: Translations;
    index: number;
    onrestore: () => void;
    ondelete: () => void;
    send: any;
    receive: any;
  };

  let { session, tx, onrestore, ondelete, index, send, receive }: Props = $props();

  function fmtUrl(u: string, max = 55) {
    const s = u.replace(/^https?:\/\//, "");
    return s.length > max ? s.slice(0, max) + "…" : s;
  }
</script>

<div 
  class="session-bar" 
  style="--index: {index};"
>
  <div class="session-info">
    <Globe size={14} class="info-icon" />
    <span class="url">{fmtUrl(session.url)}</span>
    <div class="sep"></div>
    <FileText size={12} class="info-icon muted" />
    <span class="stats">{session.result.pages_scraped} {tx.results.pages}</span>
  </div>

  <div class="actions">
    <button class="bar-btn restore" onclick={onrestore} title="Restore">
      <ChevronUp size={16} />
    </button>
    <button class="bar-btn delete" onclick={ondelete} title="Close permanently">
      <X size={16} />
    </button>
  </div>
</div>

<style>
  .session-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    height: 44px;
    padding: 0 0.75rem 0 1rem;
    background: var(--surface);
    backdrop-filter: blur(16px) saturate(180%);
    -webkit-backdrop-filter: blur(16px) saturate(180%);
    border: 1px solid var(--border-color);
    border-radius: 12px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
    pointer-events: auto;
    transition: 
      transform 0.4s cubic-bezier(0.16, 1, 0.3, 1), 
      border-color 0.2s, 
      background 0.2s, 
      box-shadow 0.4s cubic-bezier(0.16, 1, 0.3, 1);
    user-select: none;
    z-index: calc(2000 - var(--index));
  }

  .session-bar:hover {
    border-color: rgba(255, 255, 255, 0.25);
    background: rgba(24, 24, 24, 0.95);
    transform: translateY(-4px) scale(1.01);
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.6), 0 0 20px rgba(255, 255, 255, 0.04);
  }

  .session-info {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    min-width: 0;
    flex: 1;
  }

  .info-icon {
    flex-shrink: 0;
    opacity: 0.8;
    color: var(--accent);
  }

  .info-icon.muted {
    opacity: 0.4;
    color: var(--text-muted);
  }

  .url {
    font-size: 0.85rem;
    font-weight: 500;
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    opacity: 0.9;
    letter-spacing: -0.01em;
  }

  .sep {
    width: 1px;
    height: 14px;
    background: rgba(255, 255, 255, 0.1);
  }

  .stats {
    font-size: 0.75rem;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
    white-space: nowrap;
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    margin-left: 0.75rem;
  }

  .bar-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    background: transparent;
    border: none;
    border-radius: 6px;
    color: var(--text-muted);
    cursor: pointer;
    transition: all 0.15s;
  }

  .bar-btn:hover {
    background: rgba(255, 255, 255, 0.08);
    color: var(--text-primary);
  }

  .bar-btn.delete:hover {
    background: rgba(239, 68, 68, 0.15);
    color: #f87171;
  }

  .bar-btn.restore:hover {
    color: var(--accent);
    background: rgba(255, 255, 255, 0.05);
  }

  .bar-btn:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: -2px;
  }
</style>
