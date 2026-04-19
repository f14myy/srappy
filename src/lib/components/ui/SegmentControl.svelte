<script lang="ts" generics="T extends string">
  import type { Component } from 'svelte';

  type Props = {
    options: { value: T, label?: string, icon?: any }[];
    active: T;
    onchange: (val: T) => void;
  };

  let { options, active, onchange }: Props = $props();
</script>

<div class="segment-control">
  {#each options as opt}
    <button 
      type="button" 
      class="segment-btn" 
      class:active={active === opt.value} 
      onclick={() => onchange(opt.value)}
    >
      {#if opt.icon}
        <opt.icon size={12} />
      {/if}
      {opt.label ?? opt.value}
    </button>
  {/each}
</div>

<style>
  .segment-control {
    display: flex;
    align-items: center;
    gap: 2px;
    background: rgba(0, 0, 0, 0.2);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    padding: 2px;
  }

  .segment-btn {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    background: transparent;
    border: none;
    border-radius: 4px;
    color: var(--text-muted);
    font-size: 0.68rem;
    font-weight: 600;
    padding: 0.15rem 0.5rem;
    cursor: pointer;
    transition: color 0.25s cubic-bezier(0.16, 1, 0.3, 1), background 0.25s, transform 0.25s cubic-bezier(0.34, 1.56, 0.64, 1);
    font-family: inherit;
    letter-spacing: 0.02em;
    white-space: nowrap;
  }

  .segment-btn:hover:not(.active) {
    color: var(--text-primary);
    transform: translateY(-1px);
  }

  .segment-btn.active {
    background: var(--accent);
    color: var(--accent-text);
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
  }
</style>
