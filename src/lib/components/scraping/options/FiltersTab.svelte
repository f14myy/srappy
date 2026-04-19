<script lang="ts">
  import type { ScrapeOptions } from "$lib/types";
  import type { Translations } from "$lib/i18n";
  import Toggle from "../../ui/Toggle.svelte";
  import TagFilters from "./TagFilters.svelte";
  import Input from "../../ui/Input.svelte";

  let { options, tx, onoptions }: { options: ScrapeOptions, tx: Translations, onoptions: (o: ScrapeOptions) => void } = $props();

  function toggle(key: keyof ScrapeOptions) {
    onoptions({ ...options, [key]: !options[key as keyof ScrapeOptions] });
  }
</script>

<div class="pop-filters-tab" class:disabled={!options.recursive}>
  {#if !options.recursive}
    <div class="warn-msg">{tx.options.filtersDisabledWarn}</div>
  {/if}
  
  <label class="pop-row has-tooltip" style="margin-bottom: 0.4rem;">
    <span>{tx.options.sameDomainOnly}</span>
    <Toggle on={options.same_domain_only} onclick={() => {
      let nextState = !options.same_domain_only;
      onoptions({
        ...options, 
        same_domain_only: nextState,
        enable_whitelist: nextState ? false : options.enable_whitelist,
        enable_blacklist: nextState ? false : options.enable_blacklist
      });
    }} disabled={!options.recursive} />
    <div class="custom-tooltip">{tx.tooltips.ttSameDomain}</div>
  </label>

  <label class="pop-row has-tooltip" style="margin-bottom: 0.4rem;">
    <span>{tx.options.uniqueDomainsOnly}</span>
    <Toggle on={options.unique_domains_only} onclick={() => toggle("unique_domains_only")} disabled={!options.recursive} />
    <div class="custom-tooltip">{tx.tooltips.ttUniqueDomains}</div>
  </label>
  
  <div class="pop-divider" style="margin: 0.4rem 0;"></div>

  <div class="filter-group" class:disabled={options.same_domain_only}>
     <label class="pop-row">
       <span>{tx.options.enableWhitelist}</span>
       <Toggle on={options.enable_whitelist} onclick={() => {
         onoptions({...options, enable_whitelist: !options.enable_whitelist, enable_blacklist: false})
       }} disabled={!options.recursive || options.same_domain_only} />
     </label>
     
     <div class="pop-slider-row" class:disabled={!options.enable_whitelist}>
       <span class="field-label">{tx.options.whitelistDomains} <span class="muted">({tx.options.commaSeparated})</span></span>
       <Input 
        value={options.domain_whitelist} 
        placeholder="domain.com, api.site.com" 
        oninput={(v) => onoptions({...options, domain_whitelist: v})} 
        disabled={!options.recursive || options.same_domain_only || !options.enable_whitelist}
       />
       <div class="hint">{tx.options.whitelistHint}</div>
     </div>

     <div class="pop-divider" style="margin: 0.2rem 0;"></div>

     <label class="pop-row">
       <span>{tx.options.enableBlacklist}</span>
       <Toggle on={options.enable_blacklist} onclick={() => {
         onoptions({...options, enable_blacklist: !options.enable_blacklist, enable_whitelist: false})
       }} disabled={!options.recursive || options.same_domain_only} />
     </label>
     
     <div class="pop-slider-row" class:disabled={!options.enable_blacklist}>
       <span class="field-label">{tx.options.blacklistDomains} <span class="muted">({tx.options.commaSeparated})</span></span>
       <Input 
        value={options.domain_blacklist} 
        placeholder="youtube.com, facebook.com" 
        oninput={(v) => onoptions({...options, domain_blacklist: v})} 
        disabled={!options.recursive || options.same_domain_only || !options.enable_blacklist}
       />
       <div class="hint">{tx.options.blacklistHint}</div>
     </div>
  </div>

  <div class="pop-divider" style="margin: 0.8rem 0 0.4rem;"></div>
  
  <TagFilters {options} {tx} {onoptions} />
</div>

<style>
  .pop-filters-tab {
    display: flex;
    flex-direction: column;
    max-height: 380px;
    overflow-y: auto;
    padding-right: 0.5rem;
  }

  .pop-filters-tab.disabled {
    opacity: 0.5;
    pointer-events: none;
  }

  .warn-msg {
    font-size: 0.75rem; 
    color: #f87171; 
    margin-bottom: 0.6rem; 
    text-align: center;
  }

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

  .filter-group {
    display:flex; 
    flex-direction:column; 
    gap:0.5rem;
  }

  .filter-group.disabled {
    opacity: 0.5;
    pointer-events: none;
  }

  .pop-slider-row {
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
  }

  .pop-slider-row.disabled {
    opacity: 0.5;
    pointer-events: none;
  }

  .field-label {
    font-size: 0.75rem;
  }

  .muted {
    opacity: 0.5; 
    font-size: 0.6rem;
  }

  .hint {
    font-size: 0.65rem; 
    color: var(--text-muted); 
    opacity: 0.8;
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
