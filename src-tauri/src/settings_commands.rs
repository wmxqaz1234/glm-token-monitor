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
