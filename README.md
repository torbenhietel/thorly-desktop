# thorly-desktop

Desktop-Wrapper für [thorly.io](https://thorly.io) — gebaut mit [Tauri 2](https://v2.tauri.app/).

Thin-Wrapper-Strategie: Die App startet ein natives Window und lädt
direkt `https://app.thorly.io/`. Das gesamte UI + Logik kommt aus
[`thorly-web`](https://github.com/torbenhietel/thorly-web) (SvelteKit).

Adaptive-Layout: Im Tauri-Wrapper wird automatisch das Desktop-Layout
aktiviert (Sidebar rechts + Status-Bar unten) statt der Browser-TabNav.
Erkennung über `window.__TAURI__`.

## Was die App liefert

- Native Fenster (Win/Mac/Linux), 10-30 MB Binary
- Auto-Update über GitHub-Releases
- Magic-Link-Auth (gleicher Account wie Web)
- 1:1-Sync mit der SaaS-Variante
- Status-Bar unten mit Live-KPIs (Pending Approvals, Konversationen, Cost)

## Local Build (für Entwicklung)

Voraussetzungen: [Rust](https://rustup.rs/) + [Bun](https://bun.sh/) installiert.

```bash
bun install
bun run dev    # startet Tauri-App lokal mit Live-Reload
bun run build  # produziert plattformspezifische Builds in src-tauri/target
```

## CI / Distribution

GitHub-Actions baut auf Push to `main` für alle 3 Plattformen
(macOS arm64+x64, Windows x64, Linux x64) und veröffentlicht die
Binaries als GitHub-Release. Tauri-Updater pullt von dort.

Aktuell **unsigned** (Beta-Phase) — Endnutzer sehen beim ersten Start
einmal SmartScreen/Gatekeeper-Warning, dann läuft alles normal.

Code-Signing kommt mit dem Public Release (~$400/Jahr für Win+Mac).

## Beta-Install (für Tech-Tester)

```bash
# macOS / Linux:
curl -fsSL https://thorly.io/install.sh | bash

# Windows (PowerShell):
irm https://thorly.io/install.ps1 | iex
```

Direkt-Download alternativ über die [Releases-Page](https://github.com/torbenhietel/thorly-desktop/releases).

## Architektur

```
   ┌──────────────────────────────────┐
   │  Tauri (Rust-Kern)               │
   │  - Window-Verwaltung             │
   │  - Auto-Updater                  │
   │  - Native Shell                  │
   └───────────────┬──────────────────┘
                   │ WebView (System-Webkit/WebView2/WKWebView)
                   ▼
   ┌──────────────────────────────────┐
   │  app.thorly.io  (SvelteKit)      │
   │  - Detected window.__TAURI__     │
   │  - Adaptive-Layout (Sidebar)     │
   │  - Status-Bar unten              │
   └───────────────┬──────────────────┘
                   │ REST + WebSocket
                   ▼
   ┌──────────────────────────────────┐
   │  app.thorly.io/api/...           │
   │  thorly Backend (Bun + Hono)     │
   │  Postgres, Multi-Tenant          │
   └──────────────────────────────────┘
```

## Lizenz

Proprietär. Kontakt: kontakt@thconsulting.dev
