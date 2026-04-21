<script lang="ts">
  import type { Translations } from "$lib/i18n";
  import {
    Palette,
    Zap,
    Bell,
    Database,
    Cpu,
    Info,
    Settings2,
    AlertCircle,
    History,
  } from "lucide-svelte";

  type Tab = "appearance" | "automation" | "behavior" | "data" | "system" | "logs" | "about";

  type Props = {
    activeTab: Tab;
    tx: Translations;
    appVersion: string;
    onchange: (tab: Tab) => void;
  };

  let { activeTab, tx, appVersion, onchange }: Props = $props();
</script>

<aside class="sidebar">
  <div class="sidebar-header">
    <Settings2 size={18} class="accent-icon" />
    <span>{tx.global.settings}</span>
  </div>

  <nav class="nav-menu">
    <button
      class="nav-btn"
      class:active={activeTab === "appearance"}
      onclick={() => onchange("appearance")}
    >
      <Palette size={16} />
      {tx.pref.appearance}
    </button>
    <button
      class="nav-btn"
      class:active={activeTab === "automation"}
      onclick={() => onchange("automation")}
    >
      <Zap size={16} />
      {tx.automation.title}
    </button>
    <button
      class="nav-btn"
      class:active={activeTab === "behavior"}
      onclick={() => onchange("behavior")}
    >
      <Bell size={16} />
      {tx.pref.behavior}
    </button>
    <button class="nav-btn" class:active={activeTab === "data"} onclick={() => onchange("data")}>
      <Database size={16} />
      {tx.pref.data}
    </button>
    <button
      class="nav-btn"
      class:active={activeTab === "system"}
      onclick={() => onchange("system")}
    >
      <Cpu size={16} />
      {tx.system.title}
    </button>
    <button class="nav-btn" class:active={activeTab === "logs"} onclick={() => onchange("logs")}>
      <History size={16} />
      {tx.stats.history || "Logs"}
    </button>
    <button class="nav-btn" class:active={activeTab === "about"} onclick={() => onchange("about")}>
      <Info size={16} />
      {tx.pref.about}
    </button>
  </nav>

  <div class="sidebar-footer">
    <span class="v-pill">v{appVersion}</span>
  </div>
</aside>

<style>
  .sidebar {
    width: 240px;
    background: rgba(128, 128, 128, 0.05);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    padding: 1.5rem 0.8rem;
    flex-shrink: 0;
  }
  .sidebar-header {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    padding: 0 0.8rem 1.5rem;
    font-weight: 700;
    font-size: 0.95rem;
    color: var(--text-primary);
  }
  .sidebar-header :global(.accent-icon) {
    color: var(--accent);
  }
  .nav-menu {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
    flex: 1;
  }
  .nav-btn {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.6rem 0.8rem;
    border: none;
    border-radius: 8px;
    background: transparent;
    color: var(--text-muted);
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
    text-align: left;
    font-family: inherit;
  }
  .nav-btn:hover {
    background: rgba(128, 128, 128, 0.1);
    color: var(--text-primary);
    transition: 0.3s;
  }
  .nav-btn.active {
    background: var(--accent);
    color: var(--accent-text);
    transition: 0.3s;
  }
  .sidebar-footer {
    padding-top: 1rem;
    text-align: center;
  }
  .v-pill {
    font-size: 0.65rem;
    font-weight: 800;
    color: var(--text-muted);
    opacity: 0.6;
    background: rgba(128, 128, 128, 0.1);
    padding: 0.25rem 0.6rem;
    border-radius: 99px;
    letter-spacing: 0.05em;
  }
</style>
