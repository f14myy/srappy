<script lang="ts">
  import type { Translations } from "$lib/i18n";
  import { Settings, BarChart3 } from "lucide-svelte";

  type Props = {
    settingsOpen: boolean;
    tx: Translations;
    onopensettings: () => void;
  };

  let { settingsOpen, tx, onopensettings }: Props = $props();
</script>

<header class="topbar">
  <div class="logo">
    <svg
      viewBox="0 0 24 24"
      width="16"
      height="16"
      fill="currentColor"
    >
      <path d="M6 4l8 8-8 8h4l8-8-8-8zM13 4l8 8-8 8h4l8-8-8-8z" />
    </svg>
    <span>SRAPPY</span>
    <span class="logo-badge">v1.1</span>
  </div>

  <nav class="nav-links">
    <button
      class="settings-trigger"
      class:active={settingsOpen}
      onclick={onopensettings}
      aria-label="Open settings"
    >
      <div class="trigger-inner">
        <Settings size={15} />
        <span>{tx.global.settings}</span>
      </div>
    </button>
  </nav>
</header>

<style>
  .topbar {
    width: 100%;
    max-width: 900px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1.25rem 2rem;
    box-sizing: border-box;
    position: relative;
    z-index: 100;
    animation: topbarIn 0.8s cubic-bezier(0.16, 1, 0.3, 1) backwards;
  }

  @keyframes topbarIn {
    from { opacity: 0; transform: translateY(-10px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 0.45rem;
    font-weight: 700;
    letter-spacing: 0.08em;
    font-size: 0.88rem;
    color: var(--text-primary);
  }

  .logo svg {
    color: currentColor;
    filter: none;
  }

  .logo span {
    background: none;
    -webkit-background-clip: initial;
    -webkit-text-fill-color: initial;
    color: inherit;
  }

  .logo-badge {
    font-size: 0.6rem;
    font-weight: 600;
    padding: 0.1rem 0.3rem;
    border-radius: 3px;
    background: var(--border-color);
    color: var(--text-muted);
    letter-spacing: 0.04em;
    margin-top: -2px;
    -webkit-text-fill-color: initial;
  }

  .nav-links {
    display: flex;
    align-items: center;
  }

  .settings-trigger {
    background: transparent;
    border: none;
    padding: 0;
    cursor: pointer;
    position: relative;
    color: var(--text-muted);
    transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .trigger-inner {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.45rem 0.9rem;
    border-radius: 10px;
    background: rgba(255, 255, 255, 0);
    border: 1px solid transparent;
    font-size: 0.82rem;
    font-weight: 600;
    transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .settings-trigger:hover .trigger-inner,
  .settings-trigger.active .trigger-inner {
    background: rgba(255, 255, 255, 0.05);
    border-color: rgba(255, 255, 255, 0.1);
    color: var(--text-primary);
    box-shadow: 
      0 4px 12px rgba(0, 0, 0, 0.2),
      inset 0 0 0 1px rgba(255, 255, 255, 0.05);
  }

  .settings-trigger:active .trigger-inner {
    transform: scale(0.96);
    background: rgba(255, 255, 255, 0.08);
  }

  .settings-trigger.active .trigger-inner {
    background: rgba(255, 255, 255, 0.08);
    border-color: rgba(255, 255, 255, 0.2);
    color: var(--accent);
  }

  .settings-trigger :global(svg) {
    transition: none;
  }

  .settings-trigger:hover :global(svg) {
    transform: none;
  }
</style>
