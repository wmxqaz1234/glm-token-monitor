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

    // 获取主窗口位置（使用逻辑坐标）
    let main_pos = main_window
        .current_position()
        .map_err(|e| format!("Failed to get main window position: {}", e))?;
    let main_pos_logical = main_pos.to_logical(main_window.current_monitor().ok_or("Failed to get monitor")?.scale_factor());

    // 获取主窗口实际大小（使用逻辑坐标）
    let main_size = main_window
        .inner_size()
        .map_err(|e| format!("Failed to get main window size: {}", e))?;
    let main_size_logical = main_size.to_logical(main_window.current_monitor().ok_or("Failed to get monitor")?.scale_factor());

    // 计算副窗口位置（主窗口右侧，间隔 8px）
    let panel_width = 280.0;
    let gap = 8.0;
    let mut info_x = main_pos_logical.x + main_size_logical.width + gap;
    let info_y = main_pos_logical.y;

    // 获取主显示器信息
    if let Some(monitor) = main_window.current_monitor().map_err(|e| format!("Failed to get monitor: {}", e))? {
        let screen_size = monitor.size();
        let screen_size_logical = screen_size.to_logical(monitor.scale_factor());
        // 如果超出屏幕右侧，则显示在左侧
        if info_x + panel_width > screen_size_logical.width {
            info_x = main_pos_logical.x - panel_width - gap;
        }
    }

    // 设置窗口位置（使用逻辑坐标）
    info_panel
        .set_position(tauri::Position::Logical(tauri::LogicalPosition {
            x: info_x.max(0.0),
            y: info_y.max(0.0),
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
