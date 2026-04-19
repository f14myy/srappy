<script lang="ts">
  import { browser } from "$app/environment";
  import { invoke } from "@tauri-apps/api/core";
  import { save, open } from "@tauri-apps/plugin-dialog";
  import { listen } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";
  import { crossfade } from "svelte/transition";
  import { quintOut, expoOut } from "svelte/easing";
  import { flip } from "svelte/animate";
  import { slide } from "svelte/transition";

  import { loadAppPreferences, saveAppPreferences, loadWindowBounds, saveWindowBounds, type AppPreferences } from "$lib/appPreferences";
  import { loadScrapeOptions, saveScrapeOptions } from "$lib/scrapePreferences";
  import { i18n } from "$lib/i18n";
  import { applyTheme } from "$lib/themes";
  import type { ScrapeResult, ScrapeOptions, ProgressEvent, LogEvent, ScrapeSession } from "$lib/types";

  import SettingsModal from "$lib/components/settings/SettingsModal.svelte";
  import TopBar from "$lib/components/layout/TopBar.svelte";
  import HeroSection from "$lib/components/scraping/HeroSection.svelte";
  import ResultSection from "$lib/components/results/ResultSection.svelte";
  import SessionBar from "$lib/components/layout/SessionBar.svelte";
  import StatsScreen from "$lib/components/stats/StatsScreen.svelte";
  import { saveToHistory } from "$lib/historyStore";

  const APP_VERSION = "1.0";

  const [send, receive] = crossfade({
    duration: (d) => Math.sqrt(d * 200) + 150,
    fallback(node, params) {
      const style = getComputedStyle(node);
      const transform = style.transform === "none" ? "" : style.transform;

      return {
        duration: 450,
        easing: expoOut,
        css: (t) => `
          transform: ${transform} scale(${0.9 + t * 0.1});
          opacity: ${t}
        `
      };
    }
  });

  function parentDir(filePath: string): string {
    const i = Math.max(filePath.lastIndexOf("/"), filePath.lastIndexOf("\\"));
    return i > 0 ? filePath.slice(0, i) : "";
  }

  function patchPrefs(patch: Partial<AppPreferences>) {
    prefs = { ...prefs, ...patch };
  }

  async function notifyScrapeDone(pages: number) {
    if (!prefs.notifyOnComplete || pages < 1) return;
    try {
      const n = await import("@tauri-apps/plugin-notification");
      let ok = await n.isPermissionGranted();
      if (!ok) {
        const r = await n.requestPermission();
        ok = r === "granted";
      }
      if (ok) {
        n.sendNotification({
          title: "Srappy",
          body: tx.pref.scrapeCompleteNotify.replace("{n}", String(pages)),
        });
      }
    } catch {
      /* web / denied */
    }
  }

  function onGlobalKeydown(e: KeyboardEvent) {
    if ((e.metaKey || e.ctrlKey) && e.key === ",") {
      e.preventDefault();
      settingsOpen = true;
    }
    if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === "l") {
      e.preventDefault();
      document.getElementById("url-input")?.focus();
    }
    if (e.key === "Escape" && settingsOpen) {
      settingsOpen = false;
    }
  }

  function onMouseMove(e: MouseEvent) {
    if (!prefs.interactiveGrid) return;
    const x = (e.clientX / window.innerWidth) * 100;
    const y = (e.clientY / window.innerHeight) * 100;
    document.documentElement.style.setProperty("--m-x", `${x}%`);
    document.documentElement.style.setProperty("--m-y", `${y}%`);
  }

  // ─── State ───────────────────────────────────────────────────────────────────
  let prefs = $state(loadAppPreferences());
  let url = $state("");
  let isScraping = $state(false);
  let progress = $state(0);
  let progressLabel = $state("");
  let settingsOpen = $state(false);
  let statsOpen = $state(false);
  // SESSION STATE
  let activeSession = $state<ScrapeSession | null>(null);
  let minimizedSessions = $state<ScrapeSession[]>([]);
  let errorMsg = $state("");
  let liveStats = $state<ProgressEvent | null>(null);
  let scrapeStartTime = $state<number | null>(null);
  let speed = $state(0); // pages/sec
  let logs = $state<LogEvent[]>([]);
  let speedHistory = $state<number[]>([]);
  let speedHistoryTimer: ReturnType<typeof setInterval> | null = null;

  /** Snapshot for progress events (listen handler must read current crawl limits). */
  let progressRun = $state({ recursive: false, maxPages: 10 });

  let scrapeOptions = $state<ScrapeOptions>(loadScrapeOptions());

  // ─── Derived ─────────────────────────────────────────────────────────────────
  const tx = $derived(i18n[prefs.lang]);

  $effect(() => {
    if (!browser) return;
    saveAppPreferences(prefs);
    saveScrapeOptions(scrapeOptions);
    applyTheme(prefs.theme, prefs);
    document.documentElement.style.setProperty("--ui-scale", String(prefs.uiScale));
    document.documentElement.classList.toggle("reduce-motion", prefs.reduceMotion);
  });

  $effect(() => {
    const cap = prefs.logMaxLines;
    if (logs.length > cap) {
      logs = logs.slice(-cap);
    }
  });

  // ─── Speed calculation ───────────────────────────────────────────────────────
  $effect(() => {
    if (liveStats && scrapeStartTime && liveStats.pages_done > 0) {
      const elapsed = (Date.now() - scrapeStartTime) / 1000;
      speed = liveStats.pages_done / Math.max(0.1, elapsed);
    }
  });

  // ─── Progress events ─────────────────────────────────────────────────────────
  let progressTimer: ReturnType<typeof setInterval> | null = null;

  let unlistenProgress: (() => void) | null = null;
  listen<ProgressEvent>("scrape-progress", (e) => {
    liveStats = e.payload;
  }).then((fn) => { unlistenProgress = fn; });

  let unlistenLog: (() => void) | null = null;
  let logQueue: LogEvent[] = [];
  let logBatchTimer: ReturnType<typeof setInterval> | null = null;

  listen<LogEvent>("scrape-log", (e) => {
    logQueue.push(e.payload);
    if (!logBatchTimer) {
      logBatchTimer = setInterval(() => {
        if (logQueue.length > 0) {
          const cap = prefs.logMaxLines;
          logs = [...logs, ...logQueue].slice(-cap);
          logQueue = [];
        }
      }, 150);
    }
  }).then((fn) => { unlistenLog = fn; });

  onDestroy(() => { 
    unlistenProgress?.(); 
    unlistenLog?.(); 
    if (logBatchTimer) clearInterval(logBatchTimer);
  });

  onMount(() => {
    let unlistenMove: (() => void) | undefined;
    let unlistenResize: (() => void) | undefined;
    let debounceTimer: ReturnType<typeof setTimeout> | undefined;

    void (async () => {
      try {
        const { getCurrentWindow, PhysicalPosition, PhysicalSize } = await import("@tauri-apps/api/window");
        const win = getCurrentWindow();
        if (prefs.rememberWindow) {
          const b = loadWindowBounds();
          if (b) {
            await win.setPosition(new PhysicalPosition(b.x, b.y));
            await win.setSize(new PhysicalSize(b.width, b.height));
          }
        }

        const persistBounds = async () => {
          if (!loadAppPreferences().rememberWindow) return;
          clearTimeout(debounceTimer);
          debounceTimer = setTimeout(async () => {
            try {
              const p = await win.outerPosition();
              const s = await win.outerSize();
              saveWindowBounds({ x: p.x, y: p.y, width: s.width, height: s.height });
            } catch {
              /* ignore */
            }
          }, 450);
        };

        unlistenMove = await win.onMoved(persistBounds);
        unlistenResize = await win.onResized(persistBounds);
      } catch {
        /* browser / dev */
      }

      try {
        if (!prefs.lastExportDir) {
          const dir = await invoke<string>("get_app_dir");
          if (dir) {
            prefs = { ...prefs, lastExportDir: dir };
          }
        }
      } catch {
        /* ignore */
      }
    })();

    return () => {
      if (debounceTimer !== undefined) clearTimeout(debounceTimer);
      unlistenMove?.();
      unlistenResize?.();
    };
  });

  function startProgress() {
    progress = 0;
    logs = [];
    speedHistory = [];

    speedHistoryTimer = setInterval(() => {
      if (isScraping) {
        speedHistory.push(speed);
      }
    }, 1000);

    if (progressRun.recursive) {
      progressLabel = tx.status.crawling;
      return;
    }

    const steps = [
      { target: 20, label: tx.status.connecting },
      { target: 45, label: tx.status.downloading },
      { target: 75, label: tx.status.parsing },
      { target: 92, label: tx.status.cleaning },
    ];
    progressLabel = steps[0].label;

    if (scrapeOptions.recursive) {
      // In recursive mode, real events will drive the progress
      return;
    }

    let stepIndex = 0;
    progressTimer = setInterval(() => {
      const step = steps[stepIndex] ?? steps[steps.length - 1];
      progressLabel = step.label;
      if (progress < step.target) {
        progress = Math.min(progress + 1.2, step.target);
      } else if (stepIndex < steps.length - 1) {
        stepIndex++;
      }
    }, 60);
  }

  function finishProgress(success: boolean) {
    if (progressTimer) clearInterval(progressTimer);
    if (speedHistoryTimer) clearInterval(speedHistoryTimer);
    progressLabel = success ? tx.status.done : tx.status.error;
    if (success) progress = 100;
    setTimeout(() => { progress = 0; progressLabel = ""; }, 800);
  }

  // ─── Scrape ──────────────────────────────────────────────────────────────────
  async function scrape(e: Event) {
    e.preventDefault();
    if (!url) return;
    isScraping = true;
    errorMsg = "";
    
    // If there was an active session, minimize it first
    if (activeSession) {
      minimizeSession(activeSession);
    }
    
    activeSession = null;
    liveStats = null;
    progress = 0;
    speed = 0;
    progressRun = {
      recursive: scrapeOptions.recursive,
      maxPages: scrapeOptions.max_pages,
    };
    scrapeStartTime = Date.now();
    startProgress();
    try {
      const res = await invoke<ScrapeResult>("scrape_url", { url, options: scrapeOptions });
      let finalSpeed = 0;
      if (scrapeStartTime && res.pages_scraped > 1) {
        finalSpeed = res.pages_scraped / ((Date.now() - scrapeStartTime) / 1000);
      }
      
      activeSession = {
        id: Math.random().toString(36).substring(2, 9),
        result: res,
        url: url,
        options: { ...scrapeOptions },
        speed: finalSpeed,
        speedHistory: [...speedHistory],
        timestamp: Date.now()
      };
      saveToHistory(activeSession);

      finishProgress(true);      if (activeSession) await notifyScrapeDone(activeSession.result.pages_scraped);
    } catch (err) {
      errorMsg = String(err);
      finishProgress(false);
    } finally {
      isScraping = false;
    }
  }

  async function stopScraping() {
    try {
      await invoke("stop_scraping");
    } catch (err) {
      errorMsg = String(err);
    }
  }

  async function resumeFromSession() {
    try {
      const selected = await open({
        filters: [{
          name: "Srappy Session",
          extensions: ["srappy"]
        }]
      });

      if (!selected) return;

      const session = await invoke<any>("load_session", { path: selected });
      if (!session || !session.state) {
        throw new Error("Invalid session file: No state found.");
      }

      url = session.url;
      scrapeOptions = session.options;
      progressRun = {
        recursive: scrapeOptions.recursive,
        maxPages: scrapeOptions.max_pages,
      };
      isScraping = true;
      errorMsg = "";
      
      if (activeSession) minimizeSession(activeSession);
      activeSession = null;
      
      scrapeStartTime = Date.now();
      startProgress();

      const priorPages = Array.isArray(session.pages) ? session.pages : null;
      const res = await invoke<ScrapeResult>("resume_scraping", {
        options: scrapeOptions,
        crawl_state: session.state,
        prior_pages: priorPages,
      });

      let finalSpeed = 0;
      if (scrapeStartTime && res.pages_scraped > 1) {
        finalSpeed = res.pages_scraped / ((Date.now() - scrapeStartTime) / 1000);
      }
      
      activeSession = {
        id: Math.random().toString(36).substring(2, 9),
        result: res,
        url: url,
        options: { ...scrapeOptions },
        speed: finalSpeed,
        speedHistory: [...speedHistory],
        timestamp: Date.now()
      };
      saveToHistory(activeSession);

      finishProgress(true);
      if (activeSession) await notifyScrapeDone(activeSession.result.pages_scraped);
    } catch (err) {
      errorMsg = String(err);
      finishProgress(false);
    } finally {
      isScraping = false;
    }
  }

  function minimizeSession(session: ScrapeSession) {
    if (activeSession?.id === session.id) {
      activeSession = null;
    }
    if (!minimizedSessions.find(s => s.id === session.id)) {
      minimizedSessions = [session, ...minimizedSessions].slice(0, 10);
    }
  }

  function restoreSession(session: ScrapeSession) {
    if (activeSession) {
      minimizeSession(activeSession);
    }
    activeSession = session;
    minimizedSessions = minimizedSessions.filter(s => s.id !== session.id);
    url = session.url;
    scrapeOptions = { ...session.options };
  }

  function deleteSession(id: string) {
    minimizedSessions = minimizedSessions.filter(s => s.id !== id);
    if (activeSession?.id === id) activeSession = null;
  }

  // ─── Export ──────────────────────────────────────────────────────────────────
  type ExportFormat = "txt" | "json" | "csv" | "csv_meta" | "md" | "srappy";

  async function exportData(format: ExportFormat) {
    if (!activeSession) return;
    const result = activeSession.result;
    
    let content = "";
    let extName = "";
    let ext = "";

    if (format === "txt") {
      content = result.pages_scraped > 1 
        ? result.pages.map(p => `=== ${p.url} ===\n\n${p.text}`).join("\n\n\n")
        : result.pages[0]?.text ?? "";
      extName = "Text Document";
      ext = "txt";
    } else if (format === "json") {
      content = JSON.stringify(result.pages, null, 2);
      extName = "JSON File";
      ext = "json";
    } else if (format === "csv") {
      const escape = (s: string) => `"${s.replace(/"/g, '""')}"`;
      content = "URL,Target,LoadTimeMs,SizeBytes,Text\n";
      content += result.pages.map(p => {
        return `${escape(p.url)},${escape("Body")},${p.load_time_ms},${p.size_bytes},${escape(p.text)}`;
      }).join("\n");
      extName = "CSV File";
      ext = "csv";
    } else if (format === "csv_meta") {
      const escape = (s: string) => `"${s.replace(/"/g, '""')}"`;
      content = "URL,CharCount,LoadTimeMs,SizeBytes\n";
      content += result.pages.map(p => {
        return `${escape(p.url)},${p.char_count},${p.load_time_ms},${p.size_bytes}`;
      }).join("\n");
      extName = "CSV (metadata)";
      ext = "csv";
    } else if (format === "md") {
      content = result.pages.map(p => `## [${p.url}](${p.url})\n\n${p.text}`).join("\n\n---\n\n");
      extName = "Markdown File";
      ext = "md";
    } else if (format === "srappy") {
      content = JSON.stringify({
        url,
        options: scrapeOptions,
        state: result.crawl_state,
        pages: result.pages
      }, null, 2);
      extName = "Srappy Session";
      ext = "srappy";
    }

    try {
      const filePath = await save({
        filters: [{ name: extName, extensions: [ext] }],
        defaultPath: prefs.lastExportDir || undefined,
      });
      if (filePath) {
        await invoke("save_text", { text: content, path: filePath });
        const dir = parentDir(filePath);
        if (dir) prefs = { ...prefs, lastExportDir: dir };
      }
    } catch (err) {
      errorMsg = String(err);
    }
  }
</script>

<svelte:head>
  <title>Srappy — Web Scraper</title>
</svelte:head>

<svelte:window onkeydown={onGlobalKeydown} onmousemove={onMouseMove} />

<SettingsModal
  open={settingsOpen}
  {prefs}
  {tx}
  appVersion={APP_VERSION}
  onclose={() => (settingsOpen = false)}
  onpatch={patchPrefs}
  onerror={(m) => (errorMsg = m)}
/>

{#if statsOpen}
  <StatsScreen {tx} onclose={() => (statsOpen = false)} />
{/if}

<div class="app" class:modal-open={settingsOpen}>
  <div class="grid-bg" class:interactive={prefs.interactiveGrid}></div>
  <TopBar {settingsOpen} {tx} onopensettings={() => (settingsOpen = !settingsOpen)} />

  <div class="main-area scroll-themed" class:has-result={activeSession !== null} class:has-tabs={minimizedSessions.length > 0}>
    <div class="flex-spacer" class:shrunk={isScraping || activeSession !== null || minimizedSessions.length > 0}></div>
    <HeroSection
      {url}
      {isScraping}
      {progress}
      {progressLabel}
      {errorMsg}
      compact={isScraping || activeSession !== null || minimizedSessions.length > 0}
      {liveStats}
      {logs}
      recursive={scrapeOptions.recursive}
      maxDepth={scrapeOptions.max_depth}
      maxPages={scrapeOptions.max_pages}
      {speed}
      options={scrapeOptions}
      {tx}
      onscrape={scrape}
      onurlchange={(v: string) => (url = v)}
      onoptions={(o: ScrapeOptions) => (scrapeOptions = o)}
      onstop={stopScraping}
      onresume={resumeFromSession}
      onopenstats={() => (statsOpen = true)}
    />

    {#if activeSession}
      {@const sid = activeSession.id}
      <ResultSection
        id={sid}
        {send}
        {receive}
        result={activeSession.result}
        recursive={activeSession.options.recursive}
        maxDepth={activeSession.options.max_depth}
        speed={activeSession.speed}
        speedHistory={activeSession.speedHistory}
        {tx}
        onsave={(format) => exportData(format)}
        onminimize={() => minimizeSession(activeSession!)}
      />
    {/if}
    <div class="flex-spacer" class:shrunk={isScraping || activeSession !== null || minimizedSessions.length > 0}></div>
  </div>

  <!-- Minimized sessions stack -->
  {#if minimizedSessions.length > 0}
    <div class="sessions-stack">
      {#each minimizedSessions as session, i (session.id)}
        <div 
          animate:flip={{ duration: 400, easing: expoOut }} 
          in:slide={{ duration: 400, easing: expoOut }}
          out:slide={{ duration: 400, easing: expoOut }}
          style="width: 100%; pointer-events: none;"
        >
          <SessionBar 
            {session} 
            {tx} 
            index={i} 
            {send}
            {receive}
            onrestore={() => restoreSession(session)} 
            ondelete={() => deleteSession(session.id)} 
          />
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  :root {
    --bg-color: #080808;
    --surface: rgba(12, 12, 12, 0.9);
    --border-color: #1e1e1e;
    --text-primary: #e4e4e7;
    --text-muted: #71717a;
    --accent: #fafafa;
    --accent-text: #080808;
    --grid-color: #111111;
    --scrollbar-track: rgba(255, 255, 255, 0.06);
    --scrollbar-thumb: rgba(255, 255, 255, 0.22);
    --ui-scale: 1;
    --ease-out-expo: cubic-bezier(0.16, 1, 0.3, 1);
    font-family: "Inter", -apple-system, sans-serif;
    font-size: calc(14px * var(--ui-scale));
  }

  @media (prefers-reduced-motion: reduce) {
    :root {
      --motion-duration: 0.01ms;
    }
  }

  :global(*),
  :global(*::before),
  :global(*::after) {
    box-sizing: border-box;
  }

  :global(.scroll-themed) {
    scrollbar-width: thin;
    scrollbar-color: var(--scrollbar-thumb) var(--scrollbar-track);
    -ms-overflow-style: auto;
  }
  :global(.scroll-themed::-webkit-scrollbar) {
    width: 8px;
    height: 8px;
  }
  :global(.scroll-themed::-webkit-scrollbar-track) {
    background: var(--scrollbar-track);
    border-radius: 4px;
  }
  :global(.scroll-themed::-webkit-scrollbar-thumb) {
    background: var(--scrollbar-thumb);
    border-radius: 4px;
  }
  :global(.scroll-themed::-webkit-scrollbar-thumb:hover) {
    background: rgba(255, 255, 255, 0.32);
  }

  :global(body) {
    margin: 0;
    padding: 0;
    background-color: var(--bg-color);
    color: var(--text-primary);
    transition:
      background-color 0.45s var(--ease-out-expo),
      color 0.45s var(--ease-out-expo);
    overflow: hidden;
    user-select: none;
    -webkit-user-select: none;
  }

  :global(input), :global(textarea), :global(pre), :global(code), :global([contenteditable="true"]) {
    user-select: text !important;
    -webkit-user-select: text !important;
  }

  /* ─── Outer shell ───────────────────────────────────────────────────────── */
  .app {
    height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    position: relative;
    overflow: hidden;
    transition:
      filter 0.45s var(--ease-out-expo),
      transform 0.45s var(--ease-out-expo);
  }

  .app.modal-open {
    pointer-events: none;
    filter: brightness(0.7) saturate(0.9);
    transform: scale(0.98);
  }

  /* ─── Grid background ───────────────────────────────────────────────────── */
  .grid-bg {
    position: fixed;
    inset: 0;
    background-image:
      linear-gradient(to right, var(--grid-color) 1px, transparent 1px),
      linear-gradient(to bottom, var(--grid-color) 1px, transparent 1px);
    background-size: 52px 52px;
    z-index: 0;
    mask-image: radial-gradient(ellipse at 50% 25%, black 20%, transparent 72%);
    -webkit-mask-image: radial-gradient(ellipse at 50% 25%, black 20%, transparent 72%);
    pointer-events: none;
    animation: gridDrift 28s linear infinite;
  }

  .grid-bg::after {
    content: "";
    position: absolute;
    inset: 0;
    background-image: inherit;
    background-size: inherit;
    background-position: calc(0px + 2px) calc(0px + 2px); /* Subtle shift for 'bend' effect */
    opacity: 0;
    transition: opacity 1s var(--ease-out-expo);
    mask-image: radial-gradient(circle at var(--m-x, 50%) var(--m-y, 50%), black 0%, transparent 25%);
    -webkit-mask-image: radial-gradient(circle at var(--m-x, 50%) var(--m-y, 50%), black 0%, transparent 25%);
    pointer-events: none;
  }

  /* Grid drift animation needs to be synced between layers */
  .grid-bg.interactive::after {
    animation: gridDrift 28s linear infinite;
    animation-delay: -0.1s; /* Slight offset in animation time also creates movement */
  }

  .app:not(.modal-open) .grid-bg.interactive::after {
    opacity: 0.8; /* Higher opacity since it's just shifting lines, not glowing */
  }

  @keyframes gridDrift {
    0% {
      background-position: 0 0, 0 0;
    }
    100% {
      background-position: 52px 52px, 52px 52px;
    }
  }

  :global(html.reduce-motion) .grid-bg {
    animation: none;
  }

  /* ─── TopBar sits outside main-area so it stays at top ─────────────────── */
  :global(.topbar) {
    flex-shrink: 0;
    z-index: 100; /* Ensure on top */
  }

  /* ─── Sessions Stack ─────────────────────────────────────────────────────── */
  .sessions-stack {
    position: fixed;
    bottom: 1rem;
    left: 50%;
    transform: translateX(-50%);
    width: 100%;
    max-width: 900px;
    display: flex;
    flex-direction: column-reverse; /* Newest at the bottom */
    gap: 0.5rem;
    z-index: 2000;
    pointer-events: none;
    padding: 0 1rem;
  }

  /* Each bar is individual, so we don't need much here, but let's add spacing */
  .sessions-stack :global(.session-bar) {
    pointer-events: auto;
    /* Stack effect for bars further up the list? */
    /* opacity: calc(1 - var(--index) * 0.1); */
  }

  /* ─── Main area ─────────────────────────────────────────────────────────── */
  .main-area {
    flex: 1;
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    overflow: hidden;
    position: relative;
    z-index: 10;
  }

  .flex-spacer {
    flex-grow: 1;
    transition: flex-grow 0.7s cubic-bezier(0.16, 1, 0.3, 1);
    min-height: 0;
  }

  .flex-spacer.shrunk {
    flex-grow: 0.0001;
  }

  /* When result is shown: anchor to top but stay within flex bounds */
  .main-area.has-result {
    overflow-x: hidden;
    overflow-y: auto;
    min-height: 0;
  }

  .main-area.has-tabs {
    padding-bottom: 5rem; /* Space for bars only if they exist */
  }
</style>
