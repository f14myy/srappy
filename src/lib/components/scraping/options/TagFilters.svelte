<script lang="ts">
  import type { ScrapeOptions } from "$lib/types";
  import type { Translations } from "$lib/i18n";

  let {
    options,
    tx,
    onoptions,
  }: { options: ScrapeOptions; tx: Translations; onoptions: (o: ScrapeOptions) => void } = $props();

  const COMMON_TAGS = [
    "div",
    "p",
    "span",
    "section",
    "article",
    "header",
    "footer",
    "aside",
    "nav",
    "main",
    "h1",
    "h2",
    "h3",
    "li",
    "ul",
    "table",
    "a",
    "pre",
    "blockquote",
    "form",
    "input",
    "button",
    "details",
    "summary",
    "fieldset",
    "label",
  ];

  function toggleTag(tag: string) {
    let next = [...(options.excluded_tags || [])];
    if (next.includes(tag)) {
      next = next.filter((t) => t !== tag);
    } else {
      next.push(tag);
    }
    onoptions({ ...options, excluded_tags: next });
  }

  function resetTags() {
    onoptions({ ...options, excluded_tags: ["button", "input", "select", "textarea"] });
  }
</script>

<div class="tags-filter-section">
  <div class="tags-header">
    <span class="tags-title">{tx.options.tagFilters || "Excluded Tags"}</span>
    <button type="button" class="reset-link" onclick={resetTags}
      >{tx.options.resetDefaults || "Reset"}</button
    >
  </div>
  <div class="tags-grid">
    {#each COMMON_TAGS as tag}
      <button
        type="button"
        class="tag-btn"
        class:excluded={options.excluded_tags.includes(tag)}
        onclick={() => toggleTag(tag)}
      >
        &lt;{tag}&gt;
      </button>
    {/each}
  </div>
</div>

<style>
  .tags-filter-section {
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
    padding-bottom: 0.5rem;
  }

  .tags-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.4rem;
  }

  .tags-title {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--text-muted);
  }

  .tags-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 0.35rem;
  }

  .tag-btn {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--text-muted);
    font-size: 0.65rem;
    font-family: ui-monospace, monospace;
    padding: 0.2rem 0.45rem;
    cursor: pointer;
    transition: all 0.15s;
  }

  .tag-btn:hover {
    border-color: var(--text-muted);
    color: var(--text-primary);
    background: rgba(255, 255, 255, 0.06);
  }

  .tag-btn.excluded {
    background: rgba(239, 68, 68, 0.1);
    border-color: rgba(239, 68, 68, 0.4);
    color: #f87171;
    text-decoration: line-through;
  }

  .reset-link {
    background: transparent;
    border: none;
    color: var(--accent);
    font-size: 0.65rem;
    cursor: pointer;
    padding: 0;
    opacity: 0.7;
    transition: opacity 0.15s;
  }

  .reset-link:hover {
    opacity: 1;
    text-decoration: underline;
  }
</style>
