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

  const formats: AppPreferences["defaultExportFormat"][] = ["txt", "json", "csv", "csv_meta", "md", "srappy"];
</script>

<div class="tab">
  <h2>{tx.automation.title}</h2>
  
  <div class="card-group">
    <div class="card-row">
      <div class="info">
        <span class="lbl">{tx.automation.defaultFormat}</span>
      </div>
      <div class="action">
        <div class="chips">
          {#each formats as f}
            <button class="chip" class:active={prefs.defaultExportFormat === f} onclick={() => onpatch({ defaultExportFormat: f })}>.{f.toUpperCase()}</button>
          {/each}
        </div>
      </div>
    </div>

    <div class="card-row vertical">
      <div class="info">
        <span class="lbl">{tx.automation.filenamePattern}</span>
        <span class="hnt">{tx.automation.patternHint}</span>
      </div>
      <div class="action full">
        <input type="text" class="text-in full" value={prefs.filenamePattern} oninput={(e) => onpatch({ filenamePattern: (e.target as HTMLInputElement).value })} />
      </div>
    </div>

    <div class="card-row">
      <div class="info">
        <span class="lbl">{tx.automation.autoSave}</span>
        <span class="hnt">{tx.automation.autoSaveHint}</span>
      </div>
      <div class="action">
        <Toggle on={prefs.autoSaveSessions} onclick={() => onpatch({ autoSaveSessions: !prefs.autoSaveSessions })} />
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
  .card-row.vertical { flex-direction: column; align-items: flex-start; gap: 0.75rem; }

  .info { display: flex; flex-direction: column; gap: 0.25rem; }
  .lbl { font-weight: 600; font-size: 0.9rem; color: var(--text-primary); }
  .hnt { font-size: 0.75rem; color: var(--text-muted); line-height: 1.4; }
  .action { display: flex; align-items: center; justify-content: flex-end; }
  .action.full { width: 100%; }

  .chips { display: flex; flex-wrap: wrap; gap: 0.4rem; }
  .chip { background: rgba(128, 128, 128, 0.05); border: 1px solid transparent; border-radius: 8px; color: var(--text-muted); font-size: 0.8rem; font-weight: 600; padding: 0.4rem 0.8rem; cursor: pointer; transition: all 0.2s; font-family: inherit; }
  .chip:hover { background: rgba(128, 128, 128, 0.1); color: var(--text-primary); }
  .chip.active { background: var(--accent); color: var(--accent-text); border-color: var(--accent); }

  .text-in { background: rgba(128, 128, 128, 0.05); border: 1px solid var(--border-color); border-radius: 8px; color: var(--text-primary); font-size: 0.85rem; padding: 0.5rem 0.75rem; font-family: inherit; outline: none; transition: border-color 0.2s; }
  .text-in:focus { border-color: var(--accent); }
  .text-in.full { width: 100%; }
</style>
