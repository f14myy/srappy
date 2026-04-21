<script lang="ts">
  import type { ScrapeSession } from "$lib/types";
  import type { Translations } from "$lib/i18n";
  import {
    ChevronUp,
    X,
    Globe,
    FileText,
    GripVertical,
    Clock,
    CheckCircle2,
    AlertCircle,
  } from "lucide-svelte";

  type Props = {
    session: ScrapeSession;
    tx: Translations;
    index: number;
    onrestore: () => void;
    ondelete: () => void;
    onreorder: (from: number, to: number) => void;
    send: any;
    receive: any;
  };

  let { session, tx, onrestore, ondelete, onreorder, index }: Props = $props();

  let isDragging = $state(false);
  let isOver = $state(false);

  function fmtUrl(u: string, max = 40) {
    const s = u.replace(/^https?:\/\//, "");
    return s.length > max ? s.slice(0, max) + "…" : s;
  }

  function formatSize(bytes: number) {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + " " + sizes[i];
  }

  function formatTime(ms: number) {
    if (ms < 1000) return `${ms}ms`;
    return `${(ms / 1000).toFixed(1)}s`;
  }

  // обработчики для перетаскивания (drag-and-drop)
  function handleDragStart(e: DragEvent) {
    isDragging = true;
    if (e.dataTransfer) {
      e.dataTransfer.effectAllowed = "move";
      e.dataTransfer.setData("text/plain", index.toString());
    }
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    if (e.dataTransfer) e.dataTransfer.dropEffect = "move";
    isOver = true;
  }

  function handleDragLeave() {
    isOver = false;
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    isOver = false;
    const fromIndex = parseInt(e.dataTransfer?.getData("text/plain") || "-1");
    if (fromIndex !== -1 && fromIndex !== index) {
      onreorder(fromIndex, index);
    }
  }

  function handleDragEnd() {
    isDragging = false;
    isOver = false;
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="session-bar"
  class:is-dragging={isDragging}
  class:is-over={isOver}
  style="--index: {index};"
  draggable="true"
  ondragstart={handleDragStart}
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
  ondragend={handleDragEnd}
>
  <div class="drag-handle">
    <GripVertical size={14} />
  </div>

  <div class="session-info">
    <div class="url-group">
      <Globe size={13} class="info-icon" />
      <span class="url">{fmtUrl(session.url)}</span>
    </div>

    <div class="sep"></div>

    <div class="metrics">
      <div class="metric" title={tx.results.pages}>
        <FileText size={12} class="info-icon muted" />
        <span class="stats">{session.result.pages_scraped}</span>
      </div>

      {#if session.result.pages_failed && session.result.pages_failed > 0}
        <div class="metric error" title="Failed pages">
          <AlertCircle size={12} class="info-icon" />
          <span class="stats">{session.result.pages_failed}</span>
        </div>
      {/if}

      <div class="metric" title="Total size">
        <div class="dot size"></div>
        <span class="stats">{formatSize(session.result.total_size_bytes)}</span>
      </div>

      <div class="metric" title="Execution time">
        <Clock size={12} class="info-icon muted" />
        <span class="stats">{formatTime(session.result.load_time_ms)}</span>
      </div>
    </div>
  </div>

  <div class="actions">
    <button class="bar-btn restore" onclick={onrestore} title="Restore session">
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
    width: 100%;
    height: 48px;
    padding: 0 0.6rem;
    background: var(--surface);
    backdrop-filter: blur(20px) saturate(180%);
    -webkit-backdrop-filter: blur(20px) saturate(180%);
    border: 1px solid var(--border-color);
    border-radius: 12px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
    pointer-events: auto;
    transition:
      all 0.4s cubic-bezier(0.16, 1, 0.3, 1),
      border-color 0.2s,
      background 0.2s;
    user-select: none;
    cursor: grab;
    position: relative;
    /* динамический z-index, чтобы верхние карточки были поверх нижних */
    z-index: calc(2000 - var(--index));
  }

  .session-bar:active {
    cursor: grabbing;
  }

  .session-bar.is-dragging {
    opacity: 0.4;
    transform: scale(0.98);
    border-style: dashed;
    border-color: var(--text-muted);
  }

  .session-bar.is-over {
    border-color: var(--accent);
    background: rgba(255, 255, 255, 0.05);
    transform: translateY(-2px);
  }

  .session-bar:hover:not(.is-dragging) {
    border-color: rgba(255, 255, 255, 0.25);
    background: rgba(24, 24, 24, 0.98);
    transform: translateY(-4px) scale(1.005);
    box-shadow:
      0 12px 40px rgba(0, 0, 0, 0.6),
      0 0 20px rgba(255, 255, 255, 0.04);
  }

  .drag-handle {
    display: flex;
    align-items: center;
    padding: 0 0.4rem;
    color: var(--text-muted);
    opacity: 0;
    transition: opacity 0.2s;
  }

  .session-bar:hover .drag-handle {
    opacity: 0.6;
  }

  .session-info {
    display: flex;
    align-items: center;
    gap: 0.8rem;
    min-width: 0;
    flex: 1;
  }

  .url-group {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
    flex-shrink: 1;
  }

  .session-bar :global(.info-icon) {
    flex-shrink: 0;
    color: var(--accent);
  }
  .session-bar :global(.info-icon.muted) {
    opacity: 0.4;
    color: var(--text-muted);
  }

  .url {
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    letter-spacing: -0.01em;
  }

  .sep {
    width: 1px;
    height: 16px;
    background: rgba(255, 255, 255, 0.08);
    flex-shrink: 0;
  }

  .metrics {
    display: flex;
    align-items: center;
    gap: 1.1rem;
    flex-shrink: 0;
  }

  .metric {
    display: flex;
    align-items: center;
    gap: 0.4rem;
  }

  .metric.error {
    color: #f87171;
  }
  .metric.error :global(.info-icon) {
    color: currentColor;
  }

  .dot {
    width: 5px;
    height: 5px;
    border-radius: 50%;
    background: var(--text-muted);
    opacity: 0.5;
  }
  .dot.size {
    background: var(--accent);
    opacity: 0.8;
  }

  .stats {
    font-size: 0.74rem;
    font-weight: 500;
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
    white-space: nowrap;
  }

  .metric.error .stats {
    color: #fca5a5;
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    margin-left: auto;
  }

  .bar-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 30px;
    height: 30px;
    background: transparent;
    border: none;
    border-radius: 8px;
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
