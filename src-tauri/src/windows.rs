use tauri::{WebviewWindow, Manager, Result};

/// 打开信息面板窗口
#[tauri::command]
pub async fn open_info_panel(
    main_window: WebviewWindow,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    // 获取或创建 info-panel 窗口
    let info_panel = match app_handle.get_webview_window("info-panel") {
        Some(window) => window,
        None => return Err("Info panel window not found".to_string()),
    };

    // 获取主窗口位置
    let main_pos = main_window
        .current_position()
        .map_err(|e| format!("Failed to get main window position: {}", e))?;

    // 计算副窗口位置（主窗口右侧，间隔 8px）
    let panel_width = 280;
    let main_width = 96;
    let gap = 8;
    let mut info_x = main_pos.x + main_width as i32 + gap;
    let info_y = main_pos.y;

    // 获取主显示器信息
    if let Some(monitor) = main_window.current_monitor().map_err(|e| format!("Failed to get monitor: {}", e))? {
        let screen_size = monitor.size();
        // 如果超出屏幕右侧，则显示在左侧
        if info_x + panel_width > screen_size.width as i32 {
            info_x = main_pos.x - panel_width - gap;
        }
    }

    // 设置窗口位置
    info_panel
        .set_position(tauri::Position::Physical(tauri::PhysicalPosition {
            x: info_x.max(0),
            y: info_y.max(0),
        }))
        .map_err(|e| format!("Failed to set info panel position: {}", e))?;

    // 显示窗口并设置焦点
    info_panel.show().map_err(|e| format!("Failed to show info panel: {}", e))?;
    info_panel.set_focus().map_err(|e| format!("Failed to focus info panel: {}", e))?;

    Ok(())
}

/// 关闭信息面板窗口
#[tauri::command]
pub async fn close_info_panel(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(info_panel) = app_handle.get_webview_window("info-panel") {
        info_panel.hide().map_err(|e| format!("Failed to hide info panel: {}", e))?;
    }
    Ok(())
}
