<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { AppPreferences } from "$lib/appPreferences";
  import type { Translations } from "$lib/i18n";
  import Toggle from "../ui/Toggle.svelte";

  type Props = {
    prefs: AppPreferences;
    tx: Translations;
    onpatch: (patch: Partial<AppPreferences>) => void;
  };

  let { prefs, tx, onpatch }: Props = $props();

  async function toggleStartOnBoot() {
    const newVal = !prefs.startOnBoot;
    try {
      const { enable, disable, isEnabled } = await import("@tauri-apps/plugin-autostart");
      if (newVal) {
        await enable();
      } else {
        await disable();
      }
      onpatch({ startOnBoot: await isEnabled() });
    } catch (e) {
      console.error("Autostart plugin error", e);
      onpatch({ startOnBoot: newVal }); // fallback
    }
  }

  async function toggleCloseToTray() {
    const newVal = !prefs.closeToTray;
    try {
      await invoke("set_close_to_tray", { enabled: newVal });
      onpatch({ closeToTray: newVal });
    } catch (e) {
      console.error("Set close to tray error", e);
    }
  }
</script>

<div class="tab">
  <h2>{tx.system.title}</h2>
  
  <div class="card-group">
    <div class="card-row">
      <div class="info"><span class="lbl">{tx.system.startOnBoot}</span></div>
      <div class="action">
        <Toggle on={prefs.startOnBoot} onclick={toggleStartOnBoot} />
      </div>
    </div>

    <div class="card-row">
      <div class="info">
        <span class="lbl">{tx.system.closeToTray}</span>
        <span class="hnt">{tx.system.trayHint}</span>
      </div>
      <div class="action">
        <Toggle on={prefs.closeToTray} onclick={toggleCloseToTray} />
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
</style>
