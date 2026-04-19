<script lang="ts">
  type Props = {
    label: string;
    value: number;
    min: number;
    max: number;
    step?: number;
    unit?: string;
    disabled?: boolean;
    onchange: (val: number) => void;
    tooltip?: string;
  };

  let {
    label,
    value,
    min,
    max,
    step = 1,
    unit = "",
    disabled = false,
    onchange,
    tooltip,
  }: Props = $props();

  const progress = $derived(((value - min) / (max - min)) * 100);
</script>

<div class="slider-row" class:disabled class:has-tooltip={!!tooltip}>
  <div class="slider-header">
    <span>{label}</span>
    <div class="slider-value-box">
      <input
        type="number"
        class="num-in-minimal"
        {min}
        {max}
        {step}
        {value}
        oninput={(e) => onchange(Number((e.target as HTMLInputElement).value))}
        {disabled}
      />
      {#if unit}<span class="unit">{unit}</span>{/if}
    </div>
  </div>
  <input
    type="range"
    class="range-in"
    style:--val="{progress}%"
    {min}
    {max}
    {step}
    {value}
    oninput={(e) => onchange(Number((e.target as HTMLInputElement).value))}
    {disabled}
  />
  {#if tooltip}
    <div class="custom-tooltip">{tooltip}</div>
  {/if}
</div>

<style>
  .slider-row {
    display: flex;
    flex-direction: column;
    width: 100%;
    position: relative;
  }

  .slider-row.disabled {
    opacity: 0.5;
    pointer-events: none;
  }

  .slider-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    font-size: 0.8rem;
    margin-bottom: 0.4rem;
  }

  .slider-value-box {
    display: flex;
    align-items: center;
    gap: 2px;
  }

  .num-in-minimal {
    width: 48px;
    background: transparent;
    border: none;
    color: var(--accent);
    text-align: right;
    font-size: 0.8rem;
    font-weight: 600;
    padding: 0;
    outline: none;
    font-family: inherit;
    -moz-appearance: textfield;
  }

  .num-in-minimal::-webkit-inner-spin-button,
  .num-in-minimal::-webkit-outer-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  .unit {
    color: var(--text-muted);
    font-size: 0.75rem;
  }

  .range-in {
    width: 100%;
    appearance: none;
    -webkit-appearance: none;
    background: linear-gradient(
      to right,
      var(--accent) var(--val, 0%),
      var(--border-color) var(--val, 0%)
    );
    border-radius: 999px;
    outline: none;
    cursor: pointer;
  }

  .range-in::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: var(--text-primary);
    cursor: pointer;
    border: none;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.4);
  }

  /* TOOLTIPS */
  .has-tooltip {
    position: relative;
  }
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
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.6);
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
