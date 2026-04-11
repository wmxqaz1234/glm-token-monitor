use tauri::{AppHandle, Manager};
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};

/// 创建系统托盘（Windows 和 macOS）
pub fn create_system_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // 创建托盘菜单
    let show_item = MenuItem::with_id(app, "show", "显示宠物", true, None::<&str>)?;
    let hide_item = MenuItem::with_id(app, "hide", "隐藏宠物", true, None::<&str>)?;
    let refresh_item = MenuItem::with_id(app, "refresh", "刷新数据", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let settings_item = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
    let separator2 = PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[
        &show_item,
        &hide_item,
        &refresh_item,
        &separator,
        &settings_item,
        &separator2,
        &quit_item,
    ])?;

    // 使用 Tauri 默认图标作为托盘图标
    let icon = app.default_window_icon().ok_or("Failed to get default icon")?.clone();

    let _tray = TrayIconBuilder::with_id("main-tray")
        .icon(icon)
        .menu(&menu)
        .tooltip("glm-token-monitor - Token Monitor")
        .build(app)?;

    // 处理托盘菜单事件
    app.on_menu_event(|app, event| {
        match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "hide" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }
            "refresh" => {
                // 触发数据刷新
                let app_handle = app.clone();
                tauri::async_runtime::spawn(async move {
                    crate::polling::emit_usage(&app_handle).await;
                });
            }
            "settings" => {
                // 打开设置窗口
                if let Some(window) = app.get_webview_window("settings") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        }
    });

    // 处理托盘图标点击事件
    let app_handle = app.clone();
    app.on_tray_icon_event(move |_app, event| {
        match event {
            TrayIconEvent::Click {
                id: _,
                position: _,
                rect: _,
                button,
                button_state: _,
            } => {
                if button == MouseButton::Left {
                    // 左键点击切换主窗口显示/隐藏
                    if let Some(window) = app_handle.get_webview_window("main") {
                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                }
            }
            _ => {}
        }
    });

    Ok(())
}
