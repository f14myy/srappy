<script lang="ts">
  import type { ScrapeOptions } from "$lib/types";
  import type { Translations } from "$lib/i18n";
  import Toggle from "../../ui/Toggle.svelte";
  import Slider from "../../ui/Slider.svelte";

  let { options, tx, onoptions }: { options: ScrapeOptions, tx: Translations, onoptions: (o: ScrapeOptions) => void } = $props();

  function toggle(key: keyof ScrapeOptions) {
    onoptions({ ...options, [key]: !options[key as keyof ScrapeOptions] });
  }

  function setNum(key: keyof ScrapeOptions, val: number) {
    onoptions({ ...options, [key]: val });
  }
</script>

<label class="pop-row has-tooltip" style="margin-bottom: 0.5rem;">
  <span style="font-weight: 600; color: #4ade80;">{tx.options.recursiveMode}</span>
  <Toggle on={options.recursive} onclick={() => toggle("recursive")} />
  <div class="custom-tooltip">{tx.tooltips.ttRecursive}</div>
</label>

<div style="margin-top: 0.6rem;">
  <Slider 
    label={tx.options.delayRateLimit}
    value={options.delay_ms}
    min={0}
    max={3000}
    step={100}
    unit="ms"
    disabled={!options.recursive}
    onchange={(v) => setNum("delay_ms", v)}
    tooltip={tx.tooltips.ttDelay}
  />
</div>

<div class="pop-slider-group" class:disabled={!options.recursive}>
  <Slider 
    label={tx.options.maxDepth}
    value={options.max_depth}
    min={1}
    max={10}
    disabled={!options.recursive}
    onchange={(v) => setNum("max_depth", v)}
    tooltip={tx.tooltips.ttMaxDepth}
  />

  <Slider 
    label={tx.options.maxPages}
    value={options.max_pages}
    min={1}
    max={50}
    disabled={!options.recursive}
    onchange={(v) => setNum("max_pages", v)}
    tooltip={tx.tooltips.ttMaxPages}
  />
</div>

<style>
  .pop-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 0.85rem;
    color: var(--text-primary);
    cursor: pointer;
  }

  .pop-slider-group {
    display: flex;
    flex-direction: column;
    gap: 1.2rem;
    padding: 0.8rem 0;
    margin-top: 0.25rem;
  }

  .pop-slider-group.disabled {
    opacity: 0.5;
    pointer-events: none;
  }

  .has-tooltip { position: relative; }
  .custom-tooltip {
    display: block;
    position: absolute;
    bottom: calc(100% + 6px);
    left: 50%;
    transform: translateX(-50%) translateY(4px);
    background: var(--surface);
    border: 1px solid var(--border-color);
    padding: 6px 10px;
    border-radius: 6px;
    font-size: 0.7rem;
    color: var(--text-primary);
    z-index: 100;
    box-shadow: 0 4px 16px rgba(0,0,0,0.6);
    pointer-events: none;
    white-space: nowrap;
    opacity: 0;
    visibility: hidden;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  }
  .has-tooltip:hover .custom-tooltip {
    opacity: 1;
    visibility: visible;
    transform: translateX(-50%) translateY(0);
  }
</style>
