<script lang="ts">
  import type { ScrapeResult } from "$lib/types";
  import type { Translations } from "$lib/i18n";
  import type { AppPreferences } from "$lib/appPreferences";
  import { FolderOutput, Search, X } from "lucide-svelte";
  
  import SummaryCard from "./SummaryCard.svelte";
  import PageHeader from "./PageHeader.svelte";
  import ResultContent from "./ResultContent.svelte";

  type Props = {
    result: ScrapeResult;
    recursive: boolean;
    maxDepth: number;
    speed: number;
    speedHistory: number[];
    tx: Translations;
    onsave: (format: "txt" | "json" | "csv" | "csv_meta" | "md" | "srappy") => void;
    onminimize: () => void;
    id: string;
    send: any;
    receive: any;
    defaultFormat: AppPreferences["defaultExportFormat"];
  };

  let { result, recursive, maxDepth, speed, speedHistory, tx, onsave, onminimize, id, send, receive, defaultFormat }: Props = $props();

  let selectedPage = $state(0);
  let viewMode = $state<"text" | "table">("text");
  let searchQuery = $state("");
  let copied = $state(false);
  let exportMenuOpen = $state(false);

  // Close menu if clicked outside
  function handleDocumentClick(e: MouseEvent) {
    if (exportMenuOpen && !(e.target as HTMLElement).closest('.export-dropdown-wrapper')) {
      exportMenuOpen = false;
    }
  }

  const currentPage = $derived(
    selectedPage >= 0 && selectedPage < result.pages.length
      ? result.pages[selectedPage]
      : null
  );

  const displayText = $derived(
    currentPage
      ? currentPage.text
      : result.pages.map((p) => p.text).join("\n")
  );

  async function copyText() {
    await navigator.clipboard.writeText(displayText);
    copied = true;
    setTimeout(() => (copied = false), 1800);
  }

  function prev() {
    if (selectedPage > (result.pages_scraped > 1 ? -1 : 0)) selectedPage--;
  }
  function next() {
    if (selectedPage < result.pages.length - 1) selectedPage++;
  }
</script>

<svelte:window onclick={handleDocumentClick} />

<section class="result-section">
  {#if result.crawl_state}
    <div class="partial-crawl-banner" role="status">
      <FolderOutput size={14} class="banner-icon" />
      <span class="banner-text">
        {tx.session.partialCrawlBanner.replace(/\{n\}/g, String(result.pages_scraped))}
      </span>
    </div>
  {/if}

  <SummaryCard 
    {result} {recursive} {maxDepth} {speed} {speedHistory} {tx} {viewMode} 
    {copied} {exportMenuOpen} {id} {send} {receive} {defaultFormat}
    oncopy={copyText}
    onminimize={onminimize}
    ontoggleview={(m) => viewMode = m}
    ontoggleexport={() => exportMenuOpen = !exportMenuOpen}
    onsave={(f) => { onsave(f); exportMenuOpen = false; }}
  />

  <div class="result-body">
    <div class="search-bar">
      <div class="search-input-wrapper">
        <Search size={14} class="search-icon" />
        <input 
          type="text" 
          placeholder={tx.hero.placeholder || "Search words..."} 
          bind:value={searchQuery}
          spellcheck={false}
        />
        {#if searchQuery}
          <button class="clear-search" onclick={() => searchQuery = ""}>
            <X size={14} />
          </button>
        {/if}
      </div>
    </div>

    <PageHeader 
      {result} {tx} {selectedPage} {viewMode} {currentPage}
      onprev={prev} onnext={next} onselect={(i) => selectedPage = i}
    />

    <ResultContent 
      {result} {viewMode} {displayText} {searchQuery}
      onselect={(i) => { viewMode = 'text'; selectedPage = i; }}
    />
  </div>
</section>

<style>
  .result-section {
    width: 100%;
    padding: 0 2rem 1.5rem;
    box-sizing: border-box;
    z-index: 10;
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
    gap: 0;
  }

  .result-body {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
    overflow: hidden;
  }

  .search-bar {
    background: var(--surface);
    border: 1px solid var(--border-color);
    border-top: none;
    padding: 0.4rem 0.75rem;
    display: flex;
    align-items: center;
  }

  .search-input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    flex: 1;
    background: rgba(0, 0, 0, 0.2);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    padding: 0 0.5rem;
    height: 30px;
  }

  .search-icon {
    color: var(--text-muted);
    margin-right: 0.4rem;
  }

  .search-input-wrapper input {
    background: transparent;
    border: none;
    color: var(--text-primary);
    font-size: 0.78rem;
    width: 100%;
    outline: none;
    font-family: inherit;
  }

  .clear-search {
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 0.2rem;
    display: flex;
    align-items: center;
    border-radius: 4px;
  }

  .clear-search:hover {
    color: var(--text-primary);
    background: rgba(255, 255, 255, 0.05);
  }

  .partial-crawl-banner {
    display: flex;
    align-items: flex-start;
    gap: 0.5rem;
    width: 100%;
    max-width: calc(100vw - 4rem);
    padding: 0.55rem 0.85rem;
    margin-bottom: 0.5rem;
    background: rgba(74, 222, 128, 0.08);
    border: 1px solid rgba(74, 222, 128, 0.35);
    border-radius: 8px;
    font-size: 0.78rem;
    line-height: 1.45;
    color: #86efac;
  }

  .partial-crawl-banner :global(.banner-icon) {
    flex-shrink: 0;
    margin-top: 0.12rem;
    opacity: 0.9;
  }

  .banner-text {
    flex: 1;
    min-width: 0;
  }
</style>
