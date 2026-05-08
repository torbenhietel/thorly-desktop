// doneby-desktop Tauri-App.
//
// Strategie: Thin-Wrapper. Die App laedt direkt https://app.doneby.io/ —
// das gesamte UI + Logik kommt aus doneby-web (SvelteKit). Adaptive-Layout
// (Sidebar rechts statt TabNav oben) wird in doneby-web ueber den
// window.__TAURI__-Check aktiviert.
//
// Was diese Rust-Seite kann:
//  - Window-Verwaltung (Size, Title, Icon)
//  - Auto-Updater
//  - Native Shell (Open URLs in System-Browser, falls noetig)
//  - Native Dialogs (File-Picker etc., spaeter)

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    doneby_desktop_lib::run()
}
