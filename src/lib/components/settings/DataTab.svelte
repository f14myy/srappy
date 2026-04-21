<script lang="ts">
  import type { AppPreferences } from "$lib/appPreferences";
  import type { Translations } from "$lib/i18n";
  import { FolderOpen, Download, Upload } from "lucide-svelte";

  type Props = {
    prefs: AppPreferences;
    tx: Translations;
    pickExportFolder: () => void;
    exportPrefsFile: () => void;
    importPrefsFile: () => void;
  };

  let { prefs, tx, pickExportFolder, exportPrefsFile, importPrefsFile }: Props = $props();
</script>

<div class="tab">
  <h2>{tx.pref.data}</h2>

  <div class="card-group">
    <div class="card-row vertical">
      <div class="info">
        <span class="lbl">{tx.pref.defaultExportFolder}</span>
      </div>
      <div class="action full">
        <div class="path-row">
          <div class="p-text">{prefs.lastExportDir || tx.pref.clearFolderHint}</div>
          <button class="btn-primary small" onclick={pickExportFolder}>
            <FolderOpen size={14} />
            {tx.pref.chooseFolder}
          </button>
        </div>
      </div>
    </div>
  </div>

  <div class="card-group">
    <div class="card-row">
      <div class="info">
        <span class="lbl">{tx.pref.exportPrefs}</span>
      </div>
      <div class="action">
        <button class="btn-secondary" onclick={exportPrefsFile}
          ><Download size={14} /> {tx.global.save}</button
        >
      </div>
    </div>
    <div class="card-row">
      <div class="info">
        <span class="lbl">{tx.pref.importPrefs}</span>
      </div>
      <div class="action">
        <button class="btn-secondary" onclick={importPrefsFile}
          ><Upload size={14} /> {tx.pref.chooseFolder}</button
        >
      </div>
    </div>
  </div>
</div>

<style>
  .tab {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }
  .tab h2 {
    font-size: 1.5rem;
    font-weight: 800;
    margin: 0 0 0.5rem 0;
    color: var(--text-primary);
    letter-spacing: -0.02em;
  }

  .card-group {
    background: rgba(128, 128, 128, 0.02);
    border: 1px solid var(--border-color);
    border-radius: 12px;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  .card-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem 1.25rem;
    border-bottom: 1px solid var(--border-color);
    gap: 2rem;
  }
  .card-row:last-child {
    border-bottom: none;
  }
  .card-row.vertical {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.75rem;
  }

  .info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
  .lbl {
    font-weight: 600;
    font-size: 0.9rem;
    color: var(--text-primary);
  }
  .action {
    display: flex;
    align-items: center;
    justify-content: flex-end;
  }
  .action.full {
    width: 100%;
  }

  .btn-primary {
    background: var(--accent);
    color: var(--accent-text);
    border: none;
    border-radius: 8px;
    font-size: 0.8rem;
    font-weight: 600;
    padding: 0.5rem 1rem;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    transition: opacity 0.2s;
    font-family: inherit;
  }
  .btn-primary:hover {
    opacity: 0.9;
  }
  .btn-primary.small {
    padding: 0.35rem 0.75rem;
    font-size: 0.75rem;
  }

  .btn-secondary {
    background: rgba(128, 128, 128, 0.05);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    color: var(--text-primary);
    font-size: 0.8rem;
    font-weight: 600;
    padding: 0.5rem 1rem;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    transition: all 0.2s;
    font-family: inherit;
  }
  .btn-secondary:hover {
    background: rgba(128, 128, 128, 0.1);
    border-color: var(--text-muted);
  }

  .path-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: 100%;
    background: rgba(128, 128, 128, 0.05);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 0.3rem 0.3rem 0.3rem 0.75rem;
  }
  .p-text {
    flex: 1;
    font-size: 0.8rem;
    color: var(--text-muted);
    font-family: ui-monospace, monospace;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
