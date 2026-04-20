export type Lang = "en" | "ru";

export type Translations = {
  global: {
    settings: string;
    theme: string;
    language: string;
    lightThemes: string;
    darkThemes: string;
    shortcutHint: string;
    copy: string;
    copied: string;
    save: string;
  };
  hero: {
    title: string;
    subtitle: string;
    subtitleStrong: string;
    subtitleEnd: string;
    placeholder: string;
    extract: string;
    scraping: string;
  };
  status: {
    connecting: string;
    downloading: string;
    parsing: string;
    cleaning: string;
    done: string;
    error: string;
    stop: string;
    resuming: string;
    crawling: string;
  };
  options: {
    scrapingOptions: string;
    collapseWhitespace: string;
    removeLineBreaks: string;
    removeNumbers: string;
    removeSpecialChars: string;
    recursiveMode: string;
    maxDepth: string;
    maxPages: string;
    levels: string;
    sameDomainOnly: string;
    delayRateLimit: string;
    whitelistDomains: string;
    blacklistDomains: string;
    commaSeparated: string;
    enableWhitelist: string;
    enableBlacklist: string;
    whitelistHint: string;
    blacklistHint: string;
    tabFormat: string;
    tabCrawler: string;
    tabFilters: string;
    filtersDisabledWarn: string;
    targetSelector: string;
    proxies: string;
    userAgents: string;
    useHeadlessChrome: string;
    tabNetwork: string;
    uniqueDomainsOnly: string;
    concurrency: string;
    tagFilters: string;
    resetDefaults: string;
  };
  automation: {
    title: string;
    defaultFormat: string;
    filenamePattern: string;
    autoSave: string;
    autoSaveHint: string;
    patternHint: string;
  };
  system: {
    title: string;
    startOnBoot: string;
    closeToTray: string;
    trayHint: string;
  };
  ui: {
    gridIntensity: string;
  };
  results: {
    pageOf: string;
    of: string;
    allPages: string;
    pagesScraped: string;
    pagesFailed: string;
    speed: string;
    completed: string;
    depth: string;
    avgPage: string;
    chars: string;
    pages: string;
    exportAll: string;
  };
  session: {
    saveSession: string;
    sessionSaved: string;
    resumeSession: string;
    partialCrawlBanner: string;
    exportCsvMeta: string;
  };
  pref: {
    appearance: string;
    behavior: string;
    data: string;
    about: string;
    uiScale: string;
    reduceMotion: string;
    reduceMotionHint: string;
    notifyOnComplete: string;
    notifyOnCompleteHint: string;
    rememberWindow: string;
    rememberWindowHint: string;
    logBufferSize: string;
    defaultExportFolder: string;
    chooseFolder: string;
    clearFolderHint: string;
    exportPrefs: string;
    importPrefs: string;
    importTheme: string;
    prefsInvalidFile: string;
    scrapeCompleteNotify: string;
    interactiveGrid: string;
    interactiveGridHint: string;
  };
  stats: {
    title: string;
    overview: string;
    totalScrapings: string;
    totalChars: string;
    totalBytes: string;
    totalPages: string;
    avgSpeed: string;
    history: string;
    searchPlaceholder: string;
    noData: string;
    clearHistory: string;
    deleteConfirm: string;
    date: string;
    performance: string;
    scrapingsOverTime: string;
    pagesPerScraping: string;
    clearBtn: string;
    successRate: string;
    timeSpent: string;
    topDomain: string;
    sessionDetail: string;
    charsExtracted: string;
    duration: string;
    average: string;
    perPage: string;
    dataDensity: string;
    recursiveStatus: string;
    recursionOn: string;
    recursionOff: string;
    depthReached: string;
    backToHistory: string;
  };
  tooltips: {
    ttWhitespace: string;
    ttLineBreaks: string;
    ttNumbers: string;
    ttSpecialChars: string;
    ttRecursive: string;
    ttSameDomain: string;
    ttDelay: string;
    ttMaxDepth: string;
    ttMaxPages: string;
    ttTargetSelector: string;
    ttProxies: string;
    ttUserAgents: string;
    ttHeadlessChrome: string;
    ttUniqueDomains: string;
    ttConcurrency: string;
    };  about: {
    version: string;
    license: string;
  };
};

export const i18n: Record<Lang, Translations> = {
  en: {
    global: {
      settings: "Settings",
      theme: "Theme",
      language: "Language",
      lightThemes: "Light",
      darkThemes: "Dark",
      shortcutHint: "Tip: Ctrl+, or ⌘, opens settings · Esc closes",
      copy: "Copy",
      copied: "Copied!",
      save: "Save",
    },
    hero: {
      title: "The Blazing Fast Web Scraper",
      subtitle: "Powered by Rust. Extract",
      subtitleStrong: "clean, readable text",
      subtitleEnd: "from any URL with zero browser overhead.",
      placeholder: "https://example.com",
      extract: "Go go go!",
      scraping: "Scraping…",
    },
    status: {
      connecting: "Connecting...",
      downloading: "Downloading HTML...",
      parsing: "Parsing content...",
      cleaning: "Cleaning text...",
      done: "Done!",
      error: "Error",
      stop: "Stop",
      resuming: "Resuming...",
      crawling: "Crawling",
    },
    options: {
      scrapingOptions: "Scraping Options",
      collapseWhitespace: "Collapse whitespace",
      removeLineBreaks: "Remove line breaks",
      removeNumbers: "Remove numbers",
      removeSpecialChars: "Remove special chars",
      recursiveMode: "Recursive Mode",
      maxDepth: "Max depth",
      maxPages: "Max pages",
      levels: "levels",
      sameDomainOnly: "Same Domain Only",
      delayRateLimit: "Delay (Rate Limit)",
      whitelistDomains: "Whitelist (domains)",
      blacklistDomains: "Blacklist (domains)",
      commaSeparated: "comma separated",
      enableWhitelist: "Enable Whitelist",
      enableBlacklist: "Enable Blacklist",
      whitelistHint: "Only visit recursive links containing these domains.",
      blacklistHint: "Skip recursive links containing these domains.",
      tabFormat: "Format",
      tabCrawler: "Crawler",
      tabFilters: "Filters",
      filtersDisabledWarn: "Filters only work when Recursive Mode is ON.",
      targetSelector: "CSS Selector Target",
      proxies: "Proxies (IP:Port)",
      userAgents: "User-Agents",
      useHeadlessChrome: "Use Headless Chrome (JS Render)",
      tabNetwork: "Network",
      uniqueDomainsOnly: "Only Unique Domains",
      concurrency: "Concurrency",
      tagFilters: "Filter HTML Tags",
      resetDefaults: "Reset to Default",
    },
    automation: {
      title: "Automation",
      defaultFormat: "Default Export Format",
      filenamePattern: "Filename Pattern",
      autoSave: "Auto-save every session",
      autoSaveHint: "Saves results automatically to default folder.",
      patternHint: "Use {timestamp}, {host}, {pages} as placeholders.",
    },
    system: {
      title: "System",
      startOnBoot: "Launch on system startup",
      closeToTray: "Minimize to tray on close",
      trayHint: "App stays active in the background.",
    },
    ui: {
      gridIntensity: "Grid Glow Intensity",
    },
    results: {
      pageOf: "Page",
      of: "of",
      allPages: "All pages",
      pagesScraped: "pages",
      pagesFailed: "failed",
      speed: "pg/s",
      completed: "Completed",
      depth: "Depth",
      avgPage: "avg/page",
      chars: "chars",
      pages: "pages",
      exportAll: "Export All",
    },
    session: {
      saveSession: "Save Session",
      sessionSaved: "Session saved to disk",
      resumeSession: "Resume Session",
      partialCrawlBanner:
        "Partial crawl: {n} page(s) saved. Export a .srappy session to resume later.",
      exportCsvMeta: "CSV (metadata only, Excel-safe)",
    },
    pref: {
      appearance: "Appearance",
      behavior: "Behavior",
      data: "Data & files",
      about: "About",
      uiScale: "Interface scale",
      reduceMotion: "Reduce motion",
      reduceMotionHint: "Less animation throughout the app.",
      notifyOnComplete: "Notify when scrape finishes",
      notifyOnCompleteHint: "Desktop notification (OS permission may be required).",
      rememberWindow: "Remember window position & size",
      rememberWindowHint: "Restores the main window on next launch.",
      logBufferSize: "Live log buffer (lines)",
      defaultExportFolder: "Default folder for Save / Export",
      chooseFolder: "Choose…",
      clearFolderHint: "Clear",
      exportPrefs: "Export app settings",
      importPrefs: "Import app settings",
      importTheme: "Import Theme (.srappy-theme)",
      prefsInvalidFile: "Could not import settings (invalid file).",
      scrapeCompleteNotify: "Scrape finished · {n} page(s)",
      interactiveGrid: "Interactive background grid",
      interactiveGridHint: "Grid reacts to mouse movement (glow effect).",
    },
    stats: {
      title: "Statistics",
      overview: "Overview",
      totalScrapings: "Total Scrapings",
      totalChars: "Total Characters",
      totalBytes: "Total Data",
      totalPages: "Total Pages",
      avgSpeed: "Average Speed",
      history: "History",
      searchPlaceholder: "Search by URL...",
      noData: "No history found yet.",
      clearHistory: "Clear all history?",
      deleteConfirm: "Delete this item?",
      date: "Date",
      performance: "Performance",
      scrapingsOverTime: "Activity",
      pagesPerScraping: "Efficiency",
      clearBtn: "Clear History",
      successRate: "Success Rate",
      timeSpent: "Time Spent",
      topDomain: "Top Domain",
      sessionDetail: "Session Detail",
      charsExtracted: "Characters extracted",
      duration: "Duration",
      average: "Average",
      perPage: "per page",
      dataDensity: "Data Density",
      recursiveStatus: "Crawling",
      recursionOn: "Recursive",
      recursionOff: "Single Page",
      depthReached: "Depth Reached",
      backToHistory: "Back to History",
    },
    tooltips: {
      ttWhitespace: "Replaces multiple spaces and tabs with a single space.",
      ttLineBreaks: "Removes all newlines, turning the text into one continuous block.",
      ttNumbers: "Removes all digits from the extracted text.",
      ttSpecialChars: "Keeps only letters and spaces, removing punctuation and symbols.",
      ttRecursive: "Crawler will automatically follow links found on the page.",
      ttSameDomain: "Crawler will not leave the original website.",
      ttDelay: "Wait time between sending requests to avoid getting IP blocked.",
      ttMaxDepth: "How many clicks away from the starting URL the crawler is allowed to go.",
      ttMaxPages: "Absolute limit of pages to scrape before stopping.",
      ttTargetSelector:
        "Only extract text from elements matching this CSS selector (e.g. .article or #content). Leave empty to extract everything.",
      ttProxies:
        "Enter proxies separated by newline (e.g. http://127.0.0.1:8080). Crawler will pick randomly.",
      ttUserAgents: "Enter User-Agents separated by newline. Crawler will randomly rotate them.",
      ttHeadlessChrome:
        "Spins up a real Chrome instance to render JavaScript pages (like React/Vue). Slower, but powerful.",
      ttUniqueDomains:
        "Only visits one page per domain. Useful for broad indexing of multiple sites.",
      ttConcurrency:
        "How many pages to scrape at the same time. Reduce this if using Headless Chrome to save CPU/RAM.",
    },
    about: {
      version: "Version",
      license: "GPLv3 License · Made by f14my",
    },
  },
  ru: {
    global: {
      settings: "Настройки",
      theme: "Тема",
      language: "Язык",
      lightThemes: "Светлые",
      darkThemes: "Темные",
      shortcutHint: "Ctrl+, или ⌘, — настройки · Esc — закрыть",
      copy: "Копир.",
      copied: "Скопировано!",
      save: "Сохранить",
    },
    hero: {
      title: "Ну прям очень быстрый веб-скрапер",
      subtitle: "На Rust ( а еще продвинутый ). Извлекает",
      subtitleStrong: "чистый текст,",
      subtitleEnd: "да давай бездарь используй его для дата-сетов.",
      placeholder: "https://example.com",
      extract: "Понеслась!",
      scraping: "Скрапинг…",
    },
    status: {
      connecting: "Подключение...",
      downloading: "Загрузка HTML...",
      parsing: "Парсинг...",
      cleaning: "Очистка текста...",
      done: "Готово!",
      error: "Ошибка",
      stop: "Стоп",
      resuming: "Возобновление...",
      crawling: "Обходим",
    },
    options: {
      scrapingOptions: "Параметры скрапинга",
      collapseWhitespace: "Убрать пробелы",
      removeLineBreaks: "Убрать переносы строк",
      removeNumbers: "Убрать числа",
      removeSpecialChars: "Убрать спецсимволы",
      recursiveMode: "Рекурсивный режим",
      maxDepth: "Макс. глубина",
      maxPages: "Макс. страниц",
      levels: "уровней",
      sameDomainOnly: "Только этот домен",
      delayRateLimit: "Задержка (анти-бан)",
      whitelistDomains: "Белый список доменов",
      blacklistDomains: "Черный список доменов",
      commaSeparated: "через запятую",
      enableWhitelist: "Включить Белый список",
      enableBlacklist: "Включить Черный список",
      whitelistHint: "Скрапер пойдет только по совпавшим доменам.",
      blacklistHint: "Скрапер будет игнорировать совпавшие домены.",
      tabFormat: "Форматирование",
      tabCrawler: "Рекурсивный режим",
      tabFilters: "Фильтры",
      filtersDisabledWarn: "Фильтры работают только при включенной рекурсии.",
      targetSelector: "CSS Селектор (Таргет)",
      proxies: "Прокси-серверы",
      userAgents: "User-Agents",
      useHeadlessChrome: "Использовать Хром (JS Рендер)",
      tabNetwork: "Сеть",
      uniqueDomainsOnly: "Только уникальные домены",
      concurrency: "Параллельные задачи",
      tagFilters: "Фильтр HTML тегов",
      resetDefaults: "Сбросить по умолчанию",
    },
    automation: {
      title: "Автоматизация",
      defaultFormat: "Формат экспорта по умолчанию",
      filenamePattern: "Шаблон имени файла",
      autoSave: "Авто-сохранение сессий",
      autoSaveHint: "Результаты сохраняются автоматически в выбранную папку.",
      patternHint: "Используйте {timestamp}, {host}, {pages}.",
    },
    system: {
      title: "Система",
      startOnBoot: "Запуск при старте системы",
      closeToTray: "Сворачивать в трей при закрытии",
      trayHint: "Приложение останется активным в фоне.",
    },
    ui: {
      gridIntensity: "Яркость свечения сетки",
    },
    results: {
      pageOf: "Страница",
      of: "из",
      allPages: "Все страницы",
      pagesScraped: "стр.",
      pagesFailed: "ошибок",
      speed: "стр/с",
      completed: "Завершено",
      depth: "Глубина",
      avgPage: "в среднем/стр",
      chars: "симв.",
      pages: "стр.",
      exportAll: "Экспорт всех",
    },
    session: {
      saveSession: "Сохранить сессию",
      sessionSaved: "Сессия сохранена",
      resumeSession: "Возобновить сессию",
      partialCrawlBanner:
        "Частичный обход: сохранено {n} стр. Экспортируйте .srappy, чтобы продолжить.",
      exportCsvMeta: "CSV (только метаданные, для Excel)",
    },
    pref: {
      appearance: "Внешний вид",
      behavior: "Поведение",
      data: "Данные и файлы",
      about: "О программе",
      uiScale: "Масштаб интерфейса",
      reduceMotion: "Меньше анимации",
      reduceMotionHint: "Сокращает анимации в интерфейсе.",
      notifyOnComplete: "Уведомление после скрапинга",
      notifyOnCompleteHint: "Системное уведомление (может понадобиться разрешение ОС).",
      rememberWindow: "Запоминать окно",
      rememberWindowHint: "Позиция и размер окна при следующем запуске.",
      logBufferSize: "Буфер логов (строк)",
      defaultExportFolder: "Папка по умолчанию для экспорта",
      chooseFolder: "Выбрать…",
      clearFolderHint: "Сбросить",
      exportPrefs: "Экспорт настроек",
      importPrefs: "Импорт настроек",
      importTheme: "Импорт темы (.srappy-theme)",
      prefsInvalidFile: "Не удалось импортировать настройки (неверный файл).",
      scrapeCompleteNotify: "Скрапинг готов · {n} стр.",
      interactiveGrid: "Интерактивная сетка фона",
      interactiveGridHint: "Сетка реагирует на движение мыши (смещение и свечение).",
    },
    stats: {
      title: "Статистика",
      overview: "Обзор",
      totalScrapings: "Всего скрапингов",
      totalChars: "Всего символов",
      totalBytes: "Всего данных",
      totalPages: "Всего страниц",
      avgSpeed: "Средняя скорость",
      history: "История",
      searchPlaceholder: "Поиск по URL...",
      noData: "История пока пуста.",
      clearHistory: "Очистить всю историю?",
      deleteConfirm: "Удалить эту запись?",
      date: "Дата",
      performance: "Производительность",
      scrapingsOverTime: "Активность",
      pagesPerScraping: "Эффективность",
      clearBtn: "Очистить историю",
      successRate: "Успешность",
      timeSpent: "Время в работе",
      topDomain: "Топ домен",
      sessionDetail: "Детали сессии",
      charsExtracted: "Извлечено символов",
      duration: "Длительность",
      average: "В среднем",
      perPage: "на страницу",
      dataDensity: "Плотность данных",
      recursiveStatus: "Тип обхода",
      recursionOn: "Рекурсивный",
      recursionOff: "Одна страница",
      depthReached: "Глубина",
      backToHistory: "Вернуться к истории",
    },
    tooltips: {
      ttWhitespace: "Заменяет множественные пробелы и табы на один пробел.",
      ttLineBreaks: "Удаляет все переносы строк, сбивая текст в один непрерывный блок.",
      ttNumbers: "Удаляет все цифры и числа из результата.",
      ttSpecialChars: "Оставляет только буквы и пробелы, убирая пунктуацию и символы.",
      ttRecursive: "Парсер будет автоматически переходить по ссылкам внутри страницы.",
      ttSameDomain: "Парсер не будет переходить на сторонние сайты.",
      ttDelay: "Время ожидания между запросами. Помогает избежать блокировок от защиты сайта.",
      ttMaxDepth: "На сколько 'кликов' вглубь от начальной страницы может уйти краулер.",
      ttMaxPages: "Жесткий лимит общего количества скачанных страниц.",
      ttTargetSelector:
        "Доставать текст только из элементов, подходящих под этот CSS-селектор (например .article или #content). Оставьте пустым, чтобы парсить всё.",
      ttProxies:
        "Вводите прокси с новой строки (напр. http://ip:port). Скрапер будет брать их рандомно.",
      ttUserAgents: "Списки User-Agent с новой строки. Скрапер будет их ротировать.",
      ttHeadlessChrome:
        "Запускает реальный Chrome для страниц на React/Vue, которые не отдают текст напрямую. Сильно медленнее.",
      ttUniqueDomains:
        "Посещать только одну страницу для каждого домена. Полезно для быстрого обхода множества сайтов.",
      ttConcurrency:
        "Сколько страниц парсить одновременно. Уменьшите это значение при использовании Хрома, чтобы не вешать систему.",
    },
    about: {
      version: "Версия",
      license: "Лицензия GPLv3 · Разработал f14my",
    },
  },
};
