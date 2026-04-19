<script lang="ts">
  import type { AppPreferences } from "$lib/appPreferences";
  import type { Translations } from "$lib/i18n";
  import Toggle from "../ui/Toggle.svelte";

  type Props = {
    prefs: AppPreferences;
    tx: Translations;
    onpatch: (patch: Partial<AppPreferences>) => void;
  };

  let { prefs, tx, onpatch }: Props = $props();

  const logSizes = [200, 500, 2000];
</script>

<div class="tab">
  <h2>{tx.pref.behavior}</h2>
  
  <div class="card-group">
    <div class="card-row">
      <div class="info">
        <span class="lbl">{tx.pref.notifyOnComplete}</span>
        <span class="hnt">{tx.pref.notifyOnCompleteHint}</span>
      </div>
      <div class="action">
        <Toggle on={prefs.notifyOnComplete} onclick={() => onpatch({ notifyOnComplete: !prefs.notifyOnComplete })} />
      </div>
    </div>

    <div class="card-row">
      <div class="info">
        <span class="lbl">{tx.pref.rememberWindow}</span>
        <span class="hnt">{tx.pref.rememberWindowHint}</span>
      </div>
      <div class="action">
        <Toggle on={prefs.rememberWindow} onclick={() => onpatch({ rememberWindow: !prefs.rememberWindow })} />
      </div>
    </div>

    <div class="card-row">
      <div class="info">
        <span class="lbl">{tx.pref.interactiveGrid}</span>
        <span class="hnt">{tx.pref.interactiveGridHint}</span>
      </div>
      <div class="action">
        <Toggle on={prefs.interactiveGrid} onclick={() => onpatch({ interactiveGrid: !prefs.interactiveGrid })} />
      </div>
    </div>

    <div class="card-row">
      <div class="info">
        <span class="lbl">{tx.pref.reduceMotion}</span>
        <span class="hnt">{tx.pref.reduceMotionHint}</span>
      </div>
      <div class="action">
        <Toggle on={prefs.reduceMotion} onclick={() => onpatch({ reduceMotion: !prefs.reduceMotion })} />
      </div>
    </div>
  </div>

  <div class="card-group">
    <div class="card-row">
      <div class="info">
        <span class="lbl">{tx.pref.logBufferSize}</span>
      </div>
      <div class="action">
        <div class="chips">
          {#each logSizes as n}
            <button class="chip" class:active={prefs.logMaxLines === n} onclick={() => onpatch({ logMaxLines: n })}>{n}</button>
          {/each}
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .tab { display: flex; flex-direction: column; gap: 2rem; }
  .tab h2 { font-size: 1.5rem; font-weight: 800; margin: 0 0 0.5rem 0; color: var(--text-primary); letter-spacing: -0.02em; }
  
  .card-group { background: rgba(128, 128, 128, 0.02); border: 1px solid var(--border-color); border-radius: 12px; display: flex; flex-direction: column; overflow: hidden; }
  .card-row { display: flex; align-items: center; justify-content: space-between; padding: 1rem 1.25rem; border-bottom: 1px solid var(--border-color); gap: 2rem; }
  .card-row:last-child { border-bottom: none; }

  .info { display: flex; flex-direction: column; gap: 0.25rem; }
  .lbl { font-weight: 600; font-size: 0.9rem; color: var(--text-primary); }
  .hnt { font-size: 0.75rem; color: var(--text-muted); line-height: 1.4; }
  .action { display: flex; align-items: center; justify-content: flex-end; }

  .chips { display: flex; flex-wrap: wrap; gap: 0.4rem; }
  .chip { background: rgba(128, 128, 128, 0.05); border: 1px solid transparent; border-radius: 8px; color: var(--text-muted); font-size: 0.8rem; font-weight: 600; padding: 0.4rem 0.8rem; cursor: pointer; transition: all 0.2s; font-family: inherit; }
  .chip:hover { background: rgba(128, 128, 128, 0.1); color: var(--text-primary); }
  .chip.active { background: var(--accent); color: var(--accent-text); border-color: var(--accent); }
</style>
