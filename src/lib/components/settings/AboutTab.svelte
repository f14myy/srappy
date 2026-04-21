<script lang="ts">
  import type { Translations } from "$lib/i18n";

  type Props = {
    tx: Translations;
    appVersion: string;
  };

  let { tx, appVersion }: Props = $props();
</script>

<div class="tab">
  <h2>{tx.pref.about}</h2>
  <div class="brand">
    <div class="logo">
      <svg viewBox="0 0 250 250" fill="var(--accent)">
        <rect
          x="60.9329"
          y="35"
          width="126.805"
          height="25.361"
          transform="rotate(45 60.9329 35)"
        />
        <rect
          width="126.805"
          height="25.361"
          transform="matrix(-0.707107 0.707107 0.707107 0.707107 132.665 107.279)"
        />
        <rect
          x="124.335"
          y="35"
          width="126.805"
          height="25.361"
          transform="rotate(45 124.335 35)"
        />
        <rect
          width="126.805"
          height="25.361"
          transform="matrix(-0.707107 0.707107 0.707107 0.707107 196.067 107.279)"
        />
      </svg>
    </div>
    <h3>Srappy</h3>
    <p>{tx.about.version} {appVersion}</p>
  </div>
  <div class="card">
    <p>{tx.about.license}</p>
    <div class="links">
      <a href="https://github.com/f14my/srappy" target="_blank">GitHub Repo</a>
      <a href="javascript:void(0)">Report Issue</a>
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

  .brand {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 2rem 0;
  }
  .logo {
    width: 64px;
    height: 64px;
    margin-bottom: 1rem;
  }
  .brand h3 {
    font-size: 2rem;
    font-weight: 800;
    margin: 0;
    color: var(--text-primary);
  }
  .brand p {
    font-size: 0.9rem;
    color: var(--text-muted);
    margin: 0.5rem 0 0;
  }

  .card {
    background: rgba(128, 128, 128, 0.05);
    border: 1px solid var(--border-color);
    border-radius: 12px;
    padding: 1.5rem;
    text-align: center;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .card p {
    margin: 0;
    font-size: 0.85rem;
    color: var(--text-primary);
  }

  .links {
    display: flex;
    justify-content: center;
    gap: 1.5rem;
  }
  .links a {
    color: var(--accent);
    font-size: 0.85rem;
    font-weight: 600;
    text-decoration: none;
    transition: opacity 0.2s;
  }
  .links a:hover {
    opacity: 0.8;
    text-decoration: underline;
  }
</style>
