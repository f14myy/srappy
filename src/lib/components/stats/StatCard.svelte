<script lang="ts">
  import type { Component } from "svelte";

  type Props = {
    title: string;
    value: string | number;
    icon: any;
    trend?: number;
    color?: string;
  };

  let { title, value, icon: Icon, trend, color = "var(--accent)" }: Props = $props();
</script>

<div class="stat-card" style="--card-color: {color}">
  <div class="card-icon" style="color: {color}; background: color-mix(in srgb, {color} 12%, transparent);">
    <Icon size={18} />
  </div>
  <div class="card-content">
    <span class="card-title">{title}</span>
    <span class="card-value">{value}</span>
    {#if trend !== undefined}
      <span class="card-trend" class:up={trend > 0} class:down={trend < 0}>
        {trend > 0 ? "+" : ""}{trend}%
      </span>
    {/if}
  </div>
</div>

<style>
  .stat-card {
    background: rgba(255, 255, 255, 0.02);
    border: 1px solid var(--border-color);
    border-radius: 16px;
    padding: 1.25rem 1.5rem;
    display: flex;
    align-items: center;
    gap: 1.25rem;
    transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
    box-shadow: 0 4px 20px rgba(0,0,0,0.1);
    min-height: 90px;
  }

  .stat-card:hover {
    transform: translateY(-3px);
    border-color: var(--card-color);
    background: rgba(255, 255, 255, 0.04);
    box-shadow: 0 12px 32px rgba(0,0,0,0.25);
  }

  .card-icon {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    transition: transform 0.3s var(--ease-out-expo);
  }
  
  .stat-card:hover .card-icon {
    transform: scale(1.1) rotate(-5deg);
  }

  .card-content {
    display: flex;
    flex-direction: column;
    gap: 0.15rem;
    min-width: 0;
  }

  .card-title {
    font-size: 0.72rem;
    font-weight: 700;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.08em;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .card-value {
    font-size: 1.4rem;
    font-weight: 800;
    color: var(--text-primary);
    font-variant-numeric: tabular-nums;
    letter-spacing: -0.02em;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .card-trend {
    font-size: 0.7rem;
    font-weight: 700;
  }
  .card-trend.up { color: #4ade80; }
  .card-trend.down { color: #f87171; }
</style>
