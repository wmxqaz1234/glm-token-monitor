#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod events;
mod polling;
mod commands;

use events::UsageData;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_current_usage
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
