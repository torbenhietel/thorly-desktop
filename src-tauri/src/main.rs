// thorly-desktop Tauri-App.
//
// Strategie: Thin-Wrapper. Die App laedt direkt https://app.thorly.io/ —
// das gesamte UI + Logik kommt aus thorly-web (SvelteKit). Adaptive-Layout
// (Sidebar rechts statt TabNav oben) wird in thorly-web ueber den
// window.__TAURI__-Check aktiviert.
//
// Was diese Rust-Seite kann:
//  - Window-Verwaltung (Size, Title, Icon)
//  - Auto-Updater
//  - Native Shell (Open URLs in System-Browser, falls noetig)
//  - Native Dialogs (File-Picker etc., spaeter)

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    thorly_desktop_lib::run()
}
