<script lang="ts">
  type Props = {
    value: string;
    placeholder?: string;
    oninput: (val: string) => void;
    type?: "text" | "textarea";
    disabled?: boolean;
    style?: string;
  };

  let { value, placeholder, oninput, type = "text", disabled = false, style }: Props = $props();
</script>

{#if type === "text"}
  <input 
    {type} 
    class="list-in" 
    {placeholder} 
    {value} 
    oninput={(e) => oninput((e.target as HTMLInputElement).value)} 
    {disabled}
    {style}
  />
{:else}
  <textarea 
    class="list-in" 
    {placeholder} 
    {value} 
    oninput={(e) => oninput((e.target as HTMLTextAreaElement).value)} 
    {disabled}
    {style}
  ></textarea>
{/if}

<style>
  .list-in {
    width: 100%;
    background: rgba(0,0,0,0.2);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    color: var(--text-primary);
    font-size: 0.75rem;
    padding: 0.4rem 0.5rem;
    outline: none;
    font-family: inherit;
    transition: border-color 0.15s;
  }

  .list-in:focus {
    border-color: var(--accent);
  }

  textarea.list-in {
    resize: none;
  }
</style>
