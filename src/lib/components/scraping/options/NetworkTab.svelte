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

<div style="display: flex; flex-direction: column; gap: 0.8rem;">
  <label class="pop-row has-tooltip" style="margin-bottom: 0;">
    <span style="font-weight: 600; color: #fbbf24;">{tx.options.useHeadlessChrome}</span>
    <Toggle on={options.use_headless_chrome} onclick={() => toggle("use_headless_chrome")} />
    <div class="custom-tooltip">{tx.tooltips.ttHeadlessChrome}</div>
  </label>

  <div class="pop-divider" style="margin: 0.2rem 0;"></div>

  <div class="pop-slider-row has-tooltip" style="margin-top: 0; gap: 0.3rem;">
    <span style="font-size:0.75rem;">{tx.options.proxies}</span>
    <Input 
      type="textarea" 
      style="height: 60px;" 
      placeholder="http://123.45.67.89:8080" 
      value={options.proxies} 
      oninput={(v) => onoptions({...options, proxies: v})} 
    />
    <div class="custom-tooltip">{tx.tooltips.ttProxies}</div>
  </div>

  <div class="pop-slider-row has-tooltip" style="margin-top: 0; gap: 0.3rem;">
    <span style="font-size:0.75rem;">{tx.options.userAgents}</span>
    <Input 
      type="textarea" 
      style="height: 60px;" 
      placeholder="Mozilla/5.0..." 
      value={options.user_agents} 
      oninput={(v) => onoptions({...options, user_agents: v})} 
    />
    <div class="custom-tooltip">{tx.tooltips.ttUserAgents}</div>
  </div>
</div>

<style>
  .pop-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 0.85rem;
    color: var(--text-primary);
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
