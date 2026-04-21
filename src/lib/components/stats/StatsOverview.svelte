<script lang="ts">
  import type { Translations } from "$lib/i18n";
  import {
    History,
    FileText,
    Hash,
    Database,
    Zap,
    Globe,
    CheckCircle2,
    Clock,
  } from "lucide-svelte";
  import StatCard from "./StatCard.svelte";

  type Props = {
    stats: {
      total: number;
      chars: number;
      pages: number;
      failed: number;
      data: number;
      duration: number;
      successRate: number;
      avgSpeed: number;
      topDomain: string;
    };
    tx: Translations;
  };

  let { stats, tx }: Props = $props();

  function fmtSize(bytes: number) {
    if (bytes < 1024) return `${bytes} B`;
    const kb = bytes / 1024;
    if (kb < 1024) return `${kb.toFixed(1)} KB`;
    return `${(kb / 1024).toFixed(2)} MB`;
  }

  function fmtNum(n: number) {
    return n >= 1000 ? (n / 1000).toFixed(1) + "K" : n.toString();
  }

  function fmtDuration(ms: number) {
    const s = Math.floor(ms / 1000);
    if (s < 60) return `${s}s`;
    const m = Math.floor(s / 60);
    if (m < 60) return `${m}m ${s % 60}s`;
    return `${Math.floor(m / 60)}h ${m % 60}m`;
  }
</script>

<section class="overview-grid">
  <StatCard title={tx.stats.totalScrapings} value={stats.total} icon={History} />
  <StatCard title={tx.stats.totalPages} value={fmtNum(stats.pages)} icon={FileText} />
  <StatCard title={tx.stats.totalChars} value={fmtNum(stats.chars)} icon={Hash} />
  <StatCard title={tx.stats.totalBytes} value={fmtSize(stats.data)} icon={Database} />

  <StatCard title={tx.stats.avgSpeed} value={`${stats.avgSpeed.toFixed(1)} p/s`} icon={Zap} />
  <StatCard
    title={tx.stats.successRate}
    value={`${stats.successRate.toFixed(0)}%`}
    icon={CheckCircle2}
  />
  <StatCard title={tx.stats.timeSpent} value={fmtDuration(stats.duration)} icon={Clock} />
  <StatCard title={tx.stats.topDomain} value={stats.topDomain} icon={Globe} />
</section>

<style>
  .overview-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 1.25rem;
  }
</style>
