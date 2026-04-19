<script lang="ts">
  type Props = {
    data: number[];
    labels: string[];
    title: string;
    color?: string;
    formatter?: (v: number) => string;
  };

  let { data, labels, title, color = "var(--accent)", formatter = (v) => v.toString() }: Props = $props();

  const width = 500;
  const height = 220;
  const padding = 30;

  const maxValue = $derived(Math.max(...data, 1));
  const points = $derived(
    data.map((v, i) => ({
      x: padding + (i * (width - padding * 2)) / Math.max(data.length - 1, 1),
      y: height - padding - (v * (height - padding * 2)) / maxValue,
      value: v
    }))
  );

  // Smoothing logic (simple Bézier)
  const pathData = $derived(() => {
    if (points.length === 0) return "";
    if (points.length === 1) return `M ${points[0].x} ${points[0].y}`;
    
    let d = `M ${points[0].x} ${points[0].y}`;
    for (let i = 0; i < points.length - 1; i++) {
      const curr = points[i];
      const next = points[i + 1];
      const cp1x = curr.x + (next.x - curr.x) / 2;
      d += ` C ${cp1x} ${curr.y}, ${cp1x} ${next.y}, ${next.x} ${next.y}`;
    }
    return d;
  });

  const areaData = $derived(
    points.length > 0 
      ? `${pathData()} L ${points[points.length-1].x} ${height - padding} L ${points[0].x} ${height - padding} Z`
      : ""
  );

  let hoveredPoint = $state<{x: number, y: number, value: number} | null>(null);
</script>

<div class="chart-container">
  <div class="chart-header">
    <span class="chart-title">{title}</span>
    {#if hoveredPoint}
      <span class="chart-value" style="color: {color}">{formatter(hoveredPoint.value)}</span>
    {/if}
  </div>
  <div class="chart-wrapper">
    <svg viewBox="0 0 {width} {height}" preserveAspectRatio="xMidYMid meet">
      <defs>
        <linearGradient id="grad-{title.replace(/\s+/g, '')}" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stop-color={color} stop-opacity="0.25" />
          <stop offset="100%" stop-color={color} stop-opacity="0" />
        </linearGradient>
      </defs>
      
      <!-- Grid lines -->
      {#each [0, 0.25, 0.5, 0.75, 1] as tick}
        {@const y = padding + tick * (height - padding * 2)}
        <line x1={padding} y1={y} x2={width - padding} y2={y} stroke="var(--border-color)" stroke-width="1" opacity="0.3" stroke-dasharray="4 4" />
        <text x={padding - 5} y={y + 4} text-anchor="end" font-size="10" fill="var(--text-muted)" opacity="0.5">
          {formatter(maxValue * (1 - tick))}
        </text>
      {/each}

      <!-- Area -->
      <path d={areaData} fill="url(#grad-{title.replace(/\s+/g, '')})" />
      
      <!-- Path -->
      <path d={pathData()} fill="none" stroke={color} stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" />

      <!-- Interactive layer -->
      {#each points as p, i}
        <!-- Invisible hover target -->
        <circle 
          cx={p.x} cy={p.y} r="12" 
          fill="transparent" 
          class="hover-target"
          onmouseenter={() => hoveredPoint = p}
          onmouseleave={() => hoveredPoint = null}
        />
        
        <!-- Visible dot -->
        <circle 
          cx={p.x} cy={p.y} r={hoveredPoint === p ? "5" : "3.5"} 
          fill="var(--bg-color)" 
          stroke={color} 
          stroke-width="2" 
          style="pointer-events: none; transition: all 0.2s cubic-bezier(0.175, 0.885, 0.32, 1.275);"
          opacity={hoveredPoint && hoveredPoint !== p ? "0.4" : "1"}
        />
      {/each}
    </svg>
  </div>
</div>

<style>
  .chart-container {
    background: rgba(255, 255, 255, 0.02);
    border: 1px solid var(--border-color);
    border-radius: 16px;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
    flex: 1;
    min-width: 340px;
    transition: transform 0.2s, border-color 0.2s;
  }

  .chart-container:hover {
    border-color: var(--text-muted);
  }

  .chart-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    min-height: 1.5rem;
  }

  .chart-title {
    font-size: 0.85rem;
    font-weight: 700;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }

  .chart-value {
    font-family: ui-monospace, monospace;
    font-weight: 700;
    font-size: 1.1rem;
    animation: valueIn 0.2s cubic-bezier(0.16, 1, 0.3, 1) both;
  }

  @keyframes valueIn {
    from { opacity: 0; transform: translateY(-4px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .chart-wrapper {
    width: 100%;
    height: 100%;
    position: relative;
  }

  svg {
    width: 100%;
    height: 100%;
    overflow: visible;
  }

  .hover-target { cursor: pointer; }
</style>
