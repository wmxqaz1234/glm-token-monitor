#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod events;
mod polling;
mod commands;
mod tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_current_usage
        ])
        .setup(|app| {
            // Windows 平台：设置完全透明无边框窗口
            #[cfg(target_os = "windows")]
            {
                use tauri::Manager;
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.set_ignore_cursor_events(true);
                    let _ = window.set_decorations(false);
                }
            }

            // 创建系统托盘（macOS）
            #[cfg(target_os = "macos")]
            if let Err(e) = tray::create_system_tray(app) {
                eprintln!("System tray error: {}", e);
            }

            // 启动轮询服务
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = polling::start_polling_loop(app_handle).await {
                    eprintln!("Polling loop error: {}", e);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
