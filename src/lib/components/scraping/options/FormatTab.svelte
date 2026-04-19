<script lang="ts">
  import type { ScrapeOptions } from "$lib/types";
  import type { Translations } from "$lib/i18n";
  import Toggle from "../../ui/Toggle.svelte";
  import Input from "../../ui/Input.svelte";

  let { options, tx, onoptions }: { options: ScrapeOptions, tx: Translations, onoptions: (o: ScrapeOptions) => void } = $props();

  function toggle(key: keyof ScrapeOptions) {
    onoptions({ ...options, [key]: !options[key as keyof ScrapeOptions] });
  }
</script>

<div class="pop-grid">
  <label class="pop-row has-tooltip">
    <span>{tx.options.collapseWhitespace}</span>
    <Toggle on={options.remove_whitespace} onclick={() => toggle("remove_whitespace")} />
    <div class="custom-tooltip">{tx.tooltips.ttWhitespace}</div>
  </label>
  <label class="pop-row has-tooltip">
    <span>{tx.options.removeLineBreaks}</span>
    <Toggle on={options.remove_newlines} onclick={() => toggle("remove_newlines")} />
    <div class="custom-tooltip">{tx.tooltips.ttLineBreaks}</div>
  </label>
  <label class="pop-row has-tooltip">
    <span>{tx.options.removeNumbers}</span>
    <Toggle on={options.remove_numbers} onclick={() => toggle("remove_numbers")} />
    <div class="custom-tooltip">{tx.tooltips.ttNumbers}</div>
  </label>
  <label class="pop-row has-tooltip">
    <span>{tx.options.removeSpecialChars}</span>
    <Toggle on={options.remove_special_chars} onclick={() => toggle("remove_special_chars")} />
    <div class="custom-tooltip">{tx.tooltips.ttSpecialChars}</div>
  </label>
</div>

<div class="pop-divider" style="margin: 0.6rem 0;"></div>

<div class="pop-slider-row has-tooltip" style="margin-top: 0;">
  <span style="font-size:0.75rem; margin-bottom:0.2rem;">{tx.options.targetSelector}</span>
  <Input 
    value={options.target_selector} 
    placeholder=".article-body, #main-content" 
    oninput={(v) => onoptions({...options, target_selector: v})} 
  />
  <div class="custom-tooltip">{tx.tooltips.ttTargetSelector}</div>
</div>

<style>
  .pop-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    column-gap: 2rem;
    row-gap: 0.8rem;
    margin-top: 0.4rem;
  }

  .pop-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 0.85rem;
    color: var(--text-primary);
    cursor: pointer;
  }

  .pop-divider {
    height: 1px;
    background: var(--border-color);
  }

  .pop-slider-row {
    display: flex;
    flex-direction: column;
    width: 100%;
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
