<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { fade, scale } from "svelte/transition";
  import { cubicOut } from "svelte/easing";

  import type { AppPreferences } from "$lib/appPreferences";
  import { DEFAULT_APP_PREFERENCES } from "$lib/appPreferences";
  import type { Translations } from "$lib/i18n";
  import { X } from "lucide-svelte";

  import SettingsSidebar from "./SettingsSidebar.svelte";
  import AppearanceTab from "./AppearanceTab.svelte";
  import AutomationTab from "./AutomationTab.svelte";
  import BehaviorTab from "./BehaviorTab.svelte";
  import DataTab from "./DataTab.svelte";
  import SystemTab from "./SystemTab.svelte";
  import AboutTab from "./AboutTab.svelte";

  type Props = {
    open: boolean;
    prefs: AppPreferences;
    tx: Translations;
    appVersion: string;
    onclose: () => void;
    onpatch: (patch: Partial<AppPreferences>) => void;
    onerror: (msg: string) => void;
  };

  let {
    open: modalOpen,
    prefs,
    tx,
    appVersion,
    onclose,
    onpatch,
    onerror,
  }: Props = $props();

  let activeTab = $state<
    "appearance" | "automation" | "behavior" | "data" | "system" | "about"
  >("appearance");

  async function pickExportFolder() {
    try {
      const dir = await open({
        directory: true,
        multiple: false,
        title: tx.pref.defaultExportFolder,
      });
      const path =
        typeof dir === "string" ? dir : Array.isArray(dir) ? dir[0] : null;
      if (path) onpatch({ lastExportDir: path });
    } catch (e) {
      onerror(String(e));
    }
  }

  async function exportPrefsFile() {
    try {
      const path = await save({
        filters: [{ name: "Srappy preferences", extensions: ["json"] }],
        defaultPath: "srappy-preferences.json",
      });
      if (path)
        await invoke("save_text", {
          text: JSON.stringify(prefs, null, 2),
          path,
        });
    } catch (e) {
      onerror(String(e));
    }
  }

  async function importPrefsFile() {
    try {
      const path = await open({
        filters: [{ name: "JSON", extensions: ["json"] }],
        multiple: false,
      });
      if (!path || Array.isArray(path)) return;
      const text = await invoke<string>("read_text", { path });
      const parsed = JSON.parse(text) as Partial<AppPreferences>;
      onpatch({ ...DEFAULT_APP_PREFERENCES, ...parsed });
    } catch {
      onerror(tx.pref.prefsInvalidFile);
    }
  }

  function backdropKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      e.preventDefault();
      onclose();
    }
  }
</script>

{#if modalOpen}
  <div class="modal-root" onkeydown={backdropKeydown} role="presentation">
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="modal-backdrop"
      transition:fade={{ duration: 250 }}
      onclick={onclose}
      role="presentation"
    ></div>

    <div
      class="modal-window"
      transition:scale={{ duration: 400, start: 0.96, easing: cubicOut }}
      role="dialog"
      aria-modal="true"
    >
      <SettingsSidebar
        {activeTab}
        {tx}
        {appVersion}
        onchange={(t) => (activeTab = t)}
      />

      <main class="content-panel scroll-themed">
        <button type="button" class="close-btn" onclick={onclose}>
          <X size={20} />
        </button>

        <div class="tab-container">
          {#if activeTab === "appearance"}
            <AppearanceTab {prefs} {tx} {onpatch} />
          {:else if activeTab === "automation"}
            <AutomationTab {prefs} {tx} {onpatch} />
          {:else if activeTab === "behavior"}
            <BehaviorTab {prefs} {tx} {onpatch} />
          {:else if activeTab === "data"}
            <DataTab
              {prefs}
              {tx}
              {pickExportFolder}
              {exportPrefsFile}
              {importPrefsFile}
            />
          {:else if activeTab === "system"}
            <SystemTab {prefs} {tx} {onpatch} />
          {:else if activeTab === "about"}
            <AboutTab {tx} {appVersion} />
          {/if}
        </div>
      </main>
    </div>
  </div>
{/if}

<style>
  .modal-root {
    position: fixed;
    inset: 0;
    z-index: 5000;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    pointer-events: auto;
  }
  .modal-backdrop {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.45);
  }

  .modal-window {
    position: relative;
    width: 100%;
    max-width: 840px;
    height: 600px;
    background: var(--surface);
    backdrop-filter: blur(16px) saturate(180%);
    -webkit-backdrop-filter: blur(16px) saturate(180%);
    border: 1px solid var(--border-color);
    border-radius: 16px;
    display: flex;
    overflow: hidden;
    box-shadow: 0 32px 64px rgba(0, 0, 0, 0.5);
  }

  .content-panel {
    flex: 1;
    position: relative;
    overflow-y: auto;
    background: transparent;
  }
  .close-btn {
    position: absolute;
    top: 1.25rem;
    right: 1.25rem;
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 0.5rem;
    border-radius: 8px;
    transition: all 0.2s;
    z-index: 10;
  }
  .close-btn:hover {
    background: rgba(128, 128, 128, 0.1);
    color: var(--text-primary);
    transform: rotate(90deg);
  }

  .tab-container {
    padding: 2.5rem 3.5rem 3.5rem;
  }
</style>
