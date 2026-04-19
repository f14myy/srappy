<div align="center">
  <svg viewBox="0 0 24 24" width="64" height="64" fill="currentColor">
    <path d="M6 4l8 8-8 8h4l8-8-8-8zM13 4l8 8-8 8h4l8-8-8-8z" />
  </svg>
  <h1>Srappy</h1>
  <p>High-performance desktop web scraping utility built with Rust and Svelte</p>
  
  <p>
    <a href="README.ru.md">🇷🇺 Перейти к русской версии</a>
  </p>

  <img src="https://img.shields.io/badge/Version-1.0.0-black?style=flat-square" alt="Version">
  <img src="https://img.shields.io/badge/License-GPL--3.0-black?style=flat-square" alt="License">
  <img src="https://img.shields.io/badge/Build-Rust-orange?style=flat-square&logo=rust" alt="Rust">
</div>

---

## Overview

Srappy is a specialized tool designed for efficient, clean text extraction from web resources. Unlike generic browser-based scrapers that consume significant system resources, Srappy leverages the speed of Rust and the lightweight nature of Tauri to provide a streamlined experience without the overhead of a full browser environment.

The application is primarily focused on creating high-quality datasets for machine learning, research, and content archival where text purity and structural integrity are paramount.

## Core Capabilities

### Clean Extraction Engine
Srappy doesn't just "grab" HTML. It uses a recursive tree-walking algorithm to identify and isolate content blocks while stripping away interactive elements, advertisements, and navigation noise. You get the text as it was meant to be read, not as it was coded.

### Recursive Crawling
Built-in support for deep-level scraping. Define your starting point and max depth; the engine will automatically map and follow internal links, respecting domain boundaries and user-defined filters.

### Modern Web Support
While the core engine is extremely fast via raw HTTP requests, some modern sites require JavaScript execution to reveal their content. Srappy includes an optional Headless Chrome integration to handle React, Vue, and Angular applications seamlessly.

### Defensive Scraping
To prevent IP blocks and maintain access, Srappy supports:
- Staggered request delays (cascading timers).
- Automatic User-Agent rotation.
- Proxy pool management with random selection.

## Analytical Dashboard

Every scraping session is more than just a download; it's a data event. Srappy provides an advanced analytics suite to monitor your performance:

- **Live Metrics:** Real-time tracking of scraping speed (pages per second) and data volume.
- **Visual Trends:** Integrated SVG charts using Bézier curves to visualize character counts and efficiency across your history.
- **Session Intelligence:** Deep-dive reports for every historical entry, calculating data density (characters per byte), estimated human reading time, and success/failure ratios.

## Technical Architecture

- **Backend:** Rust, utilizing `tokio` for asynchronous task management and `reqwest` for networking.
- **Frontend:** Svelte 5, providing a reactive, low-latency interface that remains responsive even when handling thousands of log entries.
- **Interop:** Tauri bridge for secure, type-safe communication between the UI and the system level.

## Installation

1. Navigate to the [Releases](https://github.com/f14my/srappy/releases) page.
2. Download the installer appropriate for your operating system (Windows, macOS, or Linux).
3. Run the installer and launch Srappy.

## License

Srappy is released under the **GNU General Public License v3.0**. See the LICENSE file for details.

---
Developed by **f14my**
