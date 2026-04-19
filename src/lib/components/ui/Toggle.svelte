<script lang="ts">
  let { on, onclick, disabled = false }: { on: boolean, onclick: () => void, disabled?: boolean } = $props();
</script>

<button 
  type="button" 
  class="toggle-switch" 
  class:on 
  class:disabled
  {disabled}
  {onclick}
  aria-pressed={on}
  aria-label="Toggle"
></button>

<style>
  .toggle-switch {
    width: 36px;
    height: 20px;
    background: var(--border-color);
    border: none;
    border-radius: 999px;
    position: relative;
    cursor: pointer;
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: inset 0 2px 4px rgba(0,0,0,0.1);
    flex-shrink: 0;
  }

  .toggle-switch::after {
    content: "";
    position: absolute;
    top: 2px;
    left: 2px;
    width: 16px;
    height: 16px;
    background: #fff;
    border-radius: 50%;
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 1px 2px rgba(0,0,0,0.2);
  }

  .toggle-switch.on { background: var(--accent); }
  .toggle-switch.on::after { 
    transform: translateX(16px); 
    background: var(--accent-text);
  }

  .toggle-switch.disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
