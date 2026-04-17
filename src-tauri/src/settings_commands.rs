use crate::config::{load_config, save_config, AppConfig};
use tauri::{AppHandle, Emitter};

/// 获取当前配置
#[tauri::command]
pub async fn get_config(app: AppHandle) -> Result<AppConfig, String> {
    load_config(&app)
}

/// 保存配置
#[tauri::command]
pub async fn save_config_handler(
    app: AppHandle,
    config: AppConfig,
) -> Result<(), String> {
    save_config(&app, &config)?;

    // 触发配置变更事件
    let _ = app.emit("config-changed", &config);

    Ok(())
}

/// 设置开机自启动
#[tauri::command]
pub async fn set_auto_start(app: AppHandle, enabled: bool) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        use tauri::utils::platform::current_exe;

        let exe_path = current_exe()
            .map_err(|e| format!("Failed to get executable path: {}", e))?;

        let exe_path_str = exe_path.to_string_lossy().to_string();

        // 使用 PowerShell 设置注册表启动项
        let ps_script = if enabled {
            format!(
                "New-ItemProperty -Path 'HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Run' -Name 'glm-token-monitor' -Value '{}' -PropertyType String -Force",
                exe_path_str.replace("\\", "\\\\")
            )
        } else {
            "Remove-ItemProperty -Path 'HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Run' -Name 'glm-token-monitor' -ErrorAction SilentlyContinue".to_string()
        };

        let result = std::process::Command::new("powershell")
            .args(["-Command", &ps_script])
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .output()
            .map_err(|e| format!("Failed to execute PowerShell: {}", e))?;

        if !result.status.success() {
            let error = String::from_utf8_lossy(&result.stderr);
            return Err(format!("PowerShell command failed: {}", error));
        }

        Ok(())
    }

    #[cfg(target_os = "macos")]
    {
        use std::fs;
        use tauri::utils::platform::current_exe;

        let exe_path = current_exe()
            .map_err(|e| format!("Failed to get executable path: {}", e))?;

        let plist_path = dirs::home_dir()
            .ok_or_else(|| "Failed to get home directory".to_string())?
            .join("Library/LaunchAgents/com.glm-token-monitor.app.plist");

        if enabled {
            // 创建 LaunchAgent plist 文件
            let plist_content = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.glm-token-monitor.app</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>
"#,
                exe_path.to_string_lossy()
            );

            fs::write(&plist_path, plist_content)
                .map_err(|e| format!("Failed to write LaunchAgent plist: {}", e))?;
        } else {
            // 删除 LaunchAgent plist 文件
            let _ = fs::remove_file(&plist_path);
        }

        Ok(())
    }

    #[cfg(target_os = "linux")]
    {
        use std::fs;
        use tauri::utils::platform::current_exe;

        let exe_path = current_exe()
            .map_err(|e| format!("Failed to get executable path: {}", e))?;

        let autostart_dir = dirs::home_dir()
            .ok_or_else(|| "Failed to get home directory".to_string())?
            .join(".config/autostart");

        if !autostart_dir.exists() {
            fs::create_dir_all(&autostart_dir)
                .map_err(|e| format!("Failed to create autostart directory: {}", e))?;
        }

        let desktop_file = autostart_dir.join("glm-token-monitor.desktop");

        if enabled {
            let desktop_content = format!(
                r#"[Desktop Entry]
Type=Application
Name=glm-token-monitor
Exec={}
Icon=glm-token-monitor
Terminal=false
X-MultipleArgs=false
Categories=Utility;
"#,
                exe_path.to_string_lossy()
            );

            fs::write(&desktop_file, desktop_content)
                .map_err(|e| format!("Failed to write desktop file: {}", e))?;
        } else {
            let _ = fs::remove_file(&desktop_file);
        }

        Ok(())
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err("Auto-start is not supported on this platform".to_string())
    }
}

/// 更新阈值配置
#[tauri::command]
pub async fn update_threshold_config(
    app: AppHandle,
    fresh_threshold: u32,
    flow_threshold: u32,
    warning_threshold: u32,
    panic_threshold: u32,
) -> Result<(), String> {
    let mut config = load_config(&app)?;

    // 验证阈值顺序
    if fresh_threshold >= flow_threshold
        || flow_threshold >= warning_threshold
        || warning_threshold >= panic_threshold
        || panic_threshold >= 100
    {
        return Err("阈值顺序错误: fresh < flow < warning < panic < 100".to_string());
    }

    config.threshold_config.fresh_threshold = fresh_threshold;
    config.threshold_config.flow_threshold = flow_threshold;
    config.threshold_config.warning_threshold = warning_threshold;
    config.threshold_config.panic_threshold = panic_threshold;

    save_config(&app, &config)?;

    // 触发配置变更事件
    let _ = app.emit("config-changed", &config);

    Ok(())
}

/// 更新颜色配置
#[tauri::command]
pub async fn update_color_config(
    app: AppHandle,
    fresh_color: Option<String>,
    flow_color: Option<String>,
    warning_color: Option<String>,
    panic_color: Option<String>,
    exhausted_color: Option<String>,
) -> Result<(), String> {
    let mut config = load_config(&app)?;

    // 如果传入空字符串，则清除自定义颜色（恢复默认）
    config.threshold_config.fresh_color = if fresh_color.as_deref() == Some("") { None } else { fresh_color };
    config.threshold_config.flow_color = if flow_color.as_deref() == Some("") { None } else { flow_color };
    config.threshold_config.warning_color = if warning_color.as_deref() == Some("") { None } else { warning_color };
    config.threshold_config.panic_color = if panic_color.as_deref() == Some("") { None } else { panic_color };
    config.threshold_config.exhausted_color = if exhausted_color.as_deref() == Some("") { None } else { exhausted_color };

    save_config(&app, &config)?;

    // 触发配置变更事件
    let _ = app.emit("config-changed", &config);

    Ok(())
}

/// 测试 API 连接
#[tauri::command]
pub async fn test_api_connection(
    api_url: String,
    api_key: String,
) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to build client: {}", e))?;

    let response = client
        .get(&api_url)
        .header("Authorization", api_key)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.status().is_success() {
        Ok("Connection successful".to_string())
    } else {
        Err(format!("API returned error: {}", response.status()))
    }
}

/// 领取每日成长奖励
#[tauri::command]
pub async fn claim_daily_growth_reward(
    app: AppHandle,
    current_percent: u32,
    date: String,
) -> Result<(u32, bool, crate::config::PetGrowthData), String> {
    let mut config = load_config(&app)?;

    let (xp, upgraded) = config.growth_data.claim_daily_reward(current_percent, date)?;

    save_config(&app, &config)?;

    // 如果升级了，触发特殊事件
    if upgraded {
        let _ = app.emit("level-up", config.growth_data.level);
    }

    // 触发配置变更事件
    let _ = app.emit("config-changed", &config);

    Ok((xp, upgraded, config.growth_data))
}

/// 获取成长数据
#[tauri::command]
pub async fn get_growth_data(
    app: AppHandle,
) -> Result<crate::config::PetGrowthData, String> {
    let config = load_config(&app)?;
    Ok(config.growth_data)
}
