<script lang="ts">
  import type { LogEvent } from "$lib/types";
  import { Terminal } from "lucide-svelte";

  let { logs }: { logs: LogEvent[] } = $props();

  let termBody = $state<HTMLElement>();
  $effect(() => {
    if (logs && logs.length >= 0 && termBody) {
      termBody.scrollTop = termBody.scrollHeight;
    }
  });

  function trimUrl(u: string, max = 65) {
    const s = u.replace(/^https?:\/\//, "");
    return s.length > max ? s.slice(0, max) + "…" : s;
  }
</script>

{#if logs.length > 0}
  <div class="terminal-card">
    <div class="term-header">
      <Terminal size={12} />
      <span>Live Logs</span>
    </div>
    <div class="term-body scroll-themed" bind:this={termBody}>
      {#each logs as log (log.url)}
        <div class="term-line">
          <span class="t-status" class:err={log.status === "error"}>
            {log.status === "ok" ? "[200 OK]" : "[ERROR] "}
          </span>
          <span class="t-url" title={log.url}>{trimUrl(log.url)}</span>
          <span class="t-time">({log.time_ms}ms)</span>
        </div>
      {/each}
    </div>
  </div>
{/if}

<style>
  .terminal-card {
    width: 100%;
    max-width: 800px;
    background: #080808;
    border: 1px solid var(--border-color);
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: slideDown 0.2s ease-out;
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-6px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .term-header {
    background: var(--surface);
    border-bottom: 1px solid var(--border-color);
    padding: 0.4rem 0.75rem;
    display: flex;
    align-items: center;
    gap: 0.4rem;
    font-size: 0.7rem;
    color: var(--text-muted);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .term-body {
    padding: 0.5rem 0.75rem;
    height: 140px;
    overflow-y: auto;
    font-family: ui-monospace, "Cascadia Code", "Fira Code", monospace;
    font-size: 0.72rem;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .term-line {
    display: flex;
    gap: 0.5rem;
    white-space: nowrap;
  }

  .t-status {
    color: #4ade80;
    font-weight: 600;
  }
  .t-status.err {
    color: #f87171;
  }

  .t-url {
    color: var(--text-primary);
    opacity: 0.85;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
  }

  .t-time {
    color: var(--text-muted);
    font-variant-numeric: tabular-nums;
  }
</style>
