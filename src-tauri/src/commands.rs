use crate::events::UsageData;
use crate::polling::fetch_usage;
use tauri::AppHandle;

/// 手动获取当前使用量（前端触发，用于按需刷新）
#[tauri::command]
pub async fn get_current_usage(app: AppHandle) -> Result<UsageData, String> {
    fetch_usage(&app).await
}
