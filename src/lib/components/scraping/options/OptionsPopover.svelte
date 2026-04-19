<script lang="ts">
  import type { Translations } from "$lib/i18n";
  import type { ScrapeOptions } from "$lib/types";
  import { FileText, Network, Filter, Zap } from "lucide-svelte";
  import SegmentControl from "../../ui/SegmentControl.svelte";
  import FormatTab from "./FormatTab.svelte";
  import CrawlerTab from "./CrawlerTab.svelte";
  import FiltersTab from "./FiltersTab.svelte";
  import NetworkTab from "./NetworkTab.svelte";

  let { options, tx, onoptions }: { options: ScrapeOptions, tx: Translations, onoptions: (o: ScrapeOptions) => void } = $props();

  let popTab = $state<"format" | "crawler" | "filters" | "network">("format");

  const tabOptions = $derived([
    { value: "format", label: tx.options.tabFormat, icon: FileText },
    { value: "crawler", label: tx.options.tabCrawler, icon: Network },
    { value: "filters", label: tx.options.tabFilters, icon: Filter },
    { value: "network", label: tx.options.tabNetwork, icon: Zap }
  ]);
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="options-popover">
  <div class="pop-header-row">
    <div class="pop-header">{tx.options.scrapingOptions}</div>
    <SegmentControl 
      options={tabOptions.map(t => ({ value: t.value as any, label: t.label, icon: t.icon }))} 
      active={popTab} 
      onchange={(v) => popTab = v as any} 
    />
  </div>

  {#if popTab === 'format'}
    <FormatTab {options} {tx} {onoptions} />
  {:else if popTab === 'crawler'}
    <CrawlerTab {options} {tx} {onoptions} />
  {:else if popTab === 'filters'}
    <FiltersTab {options} {tx} {onoptions} />
  {:else if popTab === 'network'}
    <NetworkTab {options} {tx} {onoptions} />
  {/if}
</div>

<style>
  .options-popover {
    position: absolute;
    top: calc(100% + 8px);
    left: 0;
    width: 100%;
    background: var(--surface);
    border: 1px solid var(--border-color);
    border-radius: 12px;
    padding: 1rem 1.25rem;
    z-index: 91;
    backdrop-filter: blur(24px);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
    display: flex;
    flex-direction: column;
    gap: 0.6rem;
    animation: popIn 0.38s cubic-bezier(0.34, 1.56, 0.64, 1) both;
    transform-origin: top center;
  }

  @keyframes popIn {
    from {
      opacity: 0;
      transform: translateY(-14px) scale(0.94);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  .pop-header-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.25rem;
  }

  .pop-header {
    font-size: 0.75rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
  }

  :global(html.reduce-motion) .options-popover {
    animation: none;
  }
</style>
