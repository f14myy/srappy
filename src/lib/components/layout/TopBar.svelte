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
    <span class="logo-badge">v1.0</span>
  </div>

  <nav class="nav-links">
    <button
      class="nav-btn"
      class:active={settingsOpen}
      onclick={onopensettings}
      aria-label="Open settings"
    >
      <Settings size={14} strokeWidth={2} />
      {tx.global.settings}
    </button>
  </nav>
</header>

<style>
  .topbar {
    width: 100%;
    max-width: 860px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.85rem 1.5rem;
    box-sizing: border-box;
    position: relative;
    z-index: 1;
    border-bottom: 1px solid var(--border-color);
    animation: topbarIn 0.65s cubic-bezier(0.16, 1, 0.3, 1) backwards;
  }

  @keyframes topbarIn {
    from {
      opacity: 0;
      transform: translateY(-14px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
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

  .logo-badge {
    font-size: 0.6rem;
    font-weight: 600;
    padding: 0.1rem 0.3rem;
    border-radius: 3px;
    background: var(--border-color);
    color: var(--text-muted);
    letter-spacing: 0.04em;
    margin-top: -2px;
  }

  .nav-links {
    display: flex;
    gap: 0.4rem;
  }

  .nav-btn {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    background: transparent;
    border: 1px solid var(--border-color);
    color: var(--text-muted);
    padding: 0.35rem 0.75rem;
    border-radius: 6px;
    font-size: 0.8rem;
    font-family: inherit;
    cursor: pointer;
    transition:
      color 0.25s cubic-bezier(0.16, 1, 0.3, 1),
      border-color 0.25s,
      background 0.25s,
      transform 0.25s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .nav-btn:hover,
  .nav-btn.active {
    color: var(--text-primary);
    border-color: var(--text-muted);
    background: rgba(255, 255, 255, 0.04);
    transform: translateY(-1px);
  }
</style>
