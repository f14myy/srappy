<script lang="ts">
  import { fade, slide } from "svelte/transition";
  import { quintOut } from "svelte/easing";
  import type { Translations } from "$lib/i18n";
  import type { ProgressEvent, ScrapeOptions, LogEvent } from "$lib/types";
  import { Globe, Settings2, CheckCircle, AlertCircle, X, FolderOpen, BarChart3 } from "lucide-svelte";
  import OptionsPopover from "./options/OptionsPopover.svelte";
  import ProgressCard from "../results/ProgressCard.svelte";
  import TerminalLogs from "../results/TerminalLogs.svelte";

  type Props = {
    url: string;
    isScraping: boolean;
    progress: number;
    progressLabel: string;
    errorMsg: string;
    compact: boolean;
    liveStats: ProgressEvent | null;
    logs: LogEvent[];
    recursive: boolean;
    maxDepth: number;
    maxPages: number;
    speed: number;
    options: ScrapeOptions;
    tx: Translations;
    onscrape: (e: Event) => void;
    onurlchange: (v: string) => void;
    onoptions: (o: ScrapeOptions) => void;
    onstop: () => void;
    onresume: () => void;
    onopenstats: () => void;
  };

  let {
    url, isScraping, progress, progressLabel, errorMsg,
    compact, liveStats, logs, recursive, maxDepth, maxPages, speed, options, tx,
    onscrape, onurlchange, onoptions, onstop, onresume, onopenstats
  }: Props = $props();

  function trimUrl(u: string, max = 52) {
    const s = u.replace(/^https?:\/\//, "");
    return s.length > max ? s.slice(0, max) + "…" : s;
  }

  const isDone = $derived(!isScraping && progress === 0 && progressLabel === "");

  let optionsOpen = $state(false);
  function closeOptions() { optionsOpen = false; }
  

</script>

<section class="hero" class:compact class:options-open={optionsOpen && !isScraping}>
  <div class="hero-header-wrap">
    {#if !compact}
      <div 
        class="hero-titles" 
        transition:slide={{ duration: 500, easing: quintOut }}
      >
        <div class="brand-mark">
          <svg viewBox="0 0 24 24" width="36" height="36" fill="var(--accent)"><path d="M6 4l8 8-8 8h4l8-8-8-8zM13 4l8 8-8 8h4l8-8-8-8z" /></svg>
        </div>
        <h1>{tx.hero.title}</h1>
        <p class="subtitle">
          {tx.hero.subtitle} <strong>{tx.hero.subtitleStrong}</strong> {tx.hero.subtitleEnd}
        </p>
      </div>
    {/if}
  </div>

  <!-- URL form -->
  <form class="action-bar" onsubmit={onscrape}>
    <div class="input-wrapper">
      <Globe size={13} class="input-globe" color="var(--text-muted)" />
      <input
        type="url"
        id="url-input"
        placeholder={tx.hero.placeholder}
        value={url}
        oninput={(e) => onurlchange((e.target as HTMLInputElement).value)}
        required
        disabled={isScraping}
        autocomplete="off"
        spellcheck={false}
      />
      <button 
        type="button" 
        class="opt-btn" 
        onclick={onopenstats} 
        disabled={isScraping}
        title={tx.stats.title}
      >
        <BarChart3 size={15} />
      </button>
      <button 
        type="button" 
        class="opt-btn" 
        class:active={optionsOpen}
        onclick={() => optionsOpen = !optionsOpen} 
        title={tx.options.scrapingOptions}
      >
        <Settings2 size={15} />
      </button>
    </div>
    <button id="scrape-btn" type="submit" class="btn-primary" disabled={isScraping || !url}>
      {#if isScraping}
        <span class="spin-dot"></span>
        {tx.hero.scraping}
      {:else}
        {tx.hero.extract}
      {/if}
    </button>

  <!-- Scraping Options Popover -->
  {#if optionsOpen && !isScraping}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="popover-backdrop"
      transition:fade={{ duration: 200 }}
      onclick={closeOptions}
    ></div>
    <div class="popover-anim" transition:fade={{ duration: 220 }}>
      <OptionsPopover {options} {tx} {onoptions} />
    </div>
  {/if}
  </form>

  <!-- Progress card (shown while scraping) -->
  {#if isScraping || (progress > 0 && progress < 100)}
    <ProgressCard {isScraping} {progress} {progressLabel} {recursive} {liveStats} {maxDepth} {maxPages} {speed} {tx} />
    {#if isScraping && recursive}
      <button type="button" class="stop-btn" onclick={onstop} title={tx.status.stop}>
        <X size={14} /> {tx.status.stop}
      </button>
    {/if}
    <TerminalLogs {logs} />
  {:else if progress === 100}
    <div class="done-wrap">
      <div class="done-flash" aria-hidden="true"></div>
      <div class="done-msg">
        <CheckCircle size={14} />
        {tx.status.done}
      </div>
    </div>
  {/if}

  {#if errorMsg}
    <div class="error-msg">
      <AlertCircle size={14} />
      {errorMsg}
    </div>
  {/if}
</section>

<style>
  /* ─── Layout ─────────────────────────────────────────────────────────────── */
  .hero {
    width: 100%;
    padding: 0 2rem;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1.1rem;
    position: relative;
    z-index: 1;
    transition: 
      padding 0.6s cubic-bezier(0.16, 1, 0.3, 1),
      gap 0.6s cubic-bezier(0.16, 1, 0.3, 1),
      transform 0.55s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .hero.options-open {
    transform: translateY(-150px);
  }

  .hero-header-wrap {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .hero-titles {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    overflow: hidden;
  }

  /* Welcome mode — spacing handled by centering parent */
  .hero:not(.compact) {
    text-align: center;
    gap: 1rem;
    padding-bottom: 2rem;
  }

  .hero.compact {
    padding-top: 1rem;
    padding-bottom: 0.75rem;
    gap: 0.65rem;
  }

  .hero:not(.compact) .brand-mark {
    color: var(--accent);
    opacity: 0.9;
    margin-bottom: 0.25rem;
    animation: heroReveal 0.85s cubic-bezier(0.16, 1, 0.3, 1) backwards;
  }

  .hero:not(.compact) h1 {
    font-size: 3rem;
    line-height: 1.08;
    margin: 0;
    letter-spacing: -0.04em;
    font-weight: 800;
    color: var(--text-primary);
    animation: heroReveal 0.85s 0.08s cubic-bezier(0.16, 1, 0.3, 1) backwards;
  }

  .hero:not(.compact) .subtitle {
    color: var(--text-muted);
    font-size: 0.95rem;
    max-width: 480px;
    line-height: 1.6;
    margin: 0;
    animation: heroReveal 0.85s 0.16s cubic-bezier(0.16, 1, 0.3, 1) backwards;
  }

  .hero.compact .brand-mark {
    color: var(--accent);
    opacity: 0.9;
    margin-bottom: 0.25rem;
  }

  .hero.compact h1 {
    font-size: 3rem;
    line-height: 1.08;
    margin: 0;
    letter-spacing: -0.04em;
    font-weight: 800;
    color: var(--text-primary);
  }

  .hero.compact .subtitle {
    color: var(--text-muted);
    font-size: 0.95rem;
    max-width: 480px;
    line-height: 1.6;
    margin: 0;
  }

  @keyframes heroReveal {
    from {
      opacity: 0;
      transform: translateY(18px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .subtitle strong {
    color: var(--text-primary);
    font-weight: 600;
  }

  /* ─── Action bar ────────────────────────────────────────────────────────── */
  .action-bar {
    display: flex;
    gap: 0.5rem;
    width: 100%;
    max-width: 800px;
    position: relative;
    animation: heroReveal 0.85s 0.22s cubic-bezier(0.16, 1, 0.3, 1) backwards;
  }

  .hero.compact .action-bar {
    animation: none;
  }

  .popover-anim {
    position: absolute;
    left: 0;
    right: 0;
    top: 0;
    bottom: 0;
    pointer-events: none;
    z-index: 45;
  }

  .popover-anim :global(.options-popover) {
    pointer-events: auto;
  }

  .input-wrapper {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: var(--surface);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 0 0.5rem 0 0.9rem;
    height: 42px;
    backdrop-filter: blur(12px);
    transition: border-color 0.2s, box-shadow 0.2s;
  }

  .input-wrapper:focus-within {
    border-color: var(--text-muted);
    box-shadow: 0 0 0 2px rgba(255,255,255,0.04);
  }

  .opt-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    height: 32px;
    width: 32px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s;
  }

  .opt-btn:hover, .opt-btn.active {
    background: rgba(255,255,255,0.08);
    color: var(--text-primary);
  }

  input[type="url"] {
    background: transparent;
    border: none;
    color: var(--text-primary);
    font-size: 0.88rem;
    width: 100%;
    outline: none;
    font-family: "Inter", sans-serif;
  }

  input::placeholder {
    color: var(--border-color);
    opacity: 1;
  }

  .btn-primary {
    background: var(--accent);
    color: var(--accent-text);
    border: none;
    padding: 0 1.25rem;
    height: 42px;
    white-space: nowrap;
    font-family: "Inter", sans-serif;
    font-size: 0.85rem;
    font-weight: 600;
    border-radius: 8px;
    cursor: pointer;
    transition:
      opacity 0.25s cubic-bezier(0.16, 1, 0.3, 1),
      transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1),
      box-shadow 0.25s;
    outline: none;
    display: flex;
    align-items: center;
    gap: 0.4rem;
    box-shadow: 0 2px 12px color-mix(in srgb, var(--accent) 35%, transparent);
  }

  .btn-primary:hover:not(:disabled) {
    opacity: 0.92;
    transform: translateY(-2px);
    box-shadow: 0 6px 24px color-mix(in srgb, var(--accent) 45%, transparent);
  }

  .btn-primary:active:not(:disabled) {
    transform: translateY(0);
    transition-duration: 0.1s;
  }
  .btn-primary:disabled { opacity: 0.3; cursor: not-allowed; }

  .spin-dot {
    width: 7px; height: 7px;
    border-radius: 50%;
    border: 1.5px solid currentColor;
    border-top-color: transparent;
    animation: spin 0.7s linear infinite;
    flex-shrink: 0;
  }

  @keyframes spin { to { transform: rotate(360deg); } }

  .popover-backdrop {
    position: fixed;
    inset: 0;
    z-index: 40;
    background: transparent;
  }

  .stop-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    padding: 0.35rem 0.85rem;
    margin-top: 0.15rem;
    background: rgba(239, 68, 68, 0.12);
    border: 1px solid rgba(239, 68, 68, 0.45);
    border-radius: 8px;
    color: #f87171;
    font-size: 0.78rem;
    font-weight: 600;
    font-family: inherit;
    cursor: pointer;
    transition: background 0.15s, border-color 0.15s, color 0.15s;
  }

  .stop-btn:hover {
    background: rgba(239, 68, 68, 0.2);
    border-color: rgba(248, 113, 113, 0.7);
    color: #fca5a5;
  }

  .error-msg {
    display: flex;
    align-items: flex-start;
    gap: 0.45rem;
    max-width: 800px;
    padding: 0.65rem 0.85rem;
    background: rgba(239, 68, 68, 0.08);
    border: 1px solid rgba(239, 68, 68, 0.35);
    border-radius: 8px;
    color: #fca5a5;
    font-size: 0.8rem;
    line-height: 1.45;
  }

  .error-msg :global(svg) {
    flex-shrink: 0;
    margin-top: 0.1rem;
  }

  .done-wrap {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    margin-top: 0.25rem;
  }

  .done-flash {
    position: absolute;
    inset: -8px -12px;
    border-radius: 12px;
    pointer-events: none;
    background: radial-gradient(
      circle at 50% 30%,
      rgba(34, 197, 94, 0.28),
      transparent 58%
    );
    animation: doneFlash 1.15s cubic-bezier(0.16, 1, 0.3, 1) forwards;
  }

  .done-msg {
    position: relative;
    z-index: 1;
    display: inline-flex;
    align-items: center;
    gap: 0.45rem;
    padding: 0.55rem 1rem;
    border-radius: 8px;
    border: 1px solid rgba(34, 197, 94, 0.35);
    background: rgba(34, 197, 94, 0.08);
    color: #86efac;
    font-size: 0.85rem;
    font-weight: 500;
    animation: doneMsgIn 0.55s cubic-bezier(0.34, 1.56, 0.64, 1) both;
  }

  @keyframes doneFlash {
    0% {
      opacity: 0;
      transform: scale(0.92);
    }
    40% {
      opacity: 1;
      transform: scale(1);
    }
    100% {
      opacity: 0;
      transform: scale(1.06);
    }
  }

  @keyframes doneMsgIn {
    from {
      opacity: 0;
      transform: translateY(8px) scale(0.96);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  :global(html.reduce-motion) .done-flash {
    animation: none;
    opacity: 0;
  }

  :global(html.reduce-motion) .done-msg {
    animation: none;
  }

  :global(html.reduce-motion) .btn-primary {
    transition: opacity 0.15s;
    box-shadow: none;
  }

  :global(html.reduce-motion) .btn-primary:hover:not(:disabled) {
    transform: none;
    box-shadow: none;
  }

  :global(html.reduce-motion) .btn-primary:active:not(:disabled) {
    transform: none;
  }

</style>
