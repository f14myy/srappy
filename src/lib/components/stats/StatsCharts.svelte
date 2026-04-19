<script lang="ts">
  import type { Translations } from "$lib/i18n";
  import StatChart from "./StatChart.svelte";

  type Props = {
    pagesData: number[];
    speedData: number[];
    charsData: number[];
    tx: Translations;
  };

  let { pagesData, speedData, charsData, tx }: Props = $props();

  const numFormatter = (v: number) => v >= 1000 ? (v / 1000).toFixed(1) + "K" : Math.round(v).toString();
  const speedFormatter = (v: number) => v.toFixed(1) + " p/s";
</script>

<section class="charts-row">
  <StatChart 
    data={pagesData} 
    labels={[]} 
    title={tx.stats.pagesPerScraping} 
    color="var(--accent)" 
    formatter={(v) => Math.round(v).toString()}
  />
  <StatChart 
    data={speedData} 
    labels={[]} 
    title={tx.stats.performance} 
    color="var(--accent)" 
    formatter={speedFormatter}
  />
  <StatChart 
    data={charsData} 
    labels={[]} 
    title={tx.stats.charsExtracted} 
    color="var(--accent)" 
    formatter={numFormatter}
  />
</section>

<style>
  .charts-row {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
    gap: 1.25rem;
  }
</style>
