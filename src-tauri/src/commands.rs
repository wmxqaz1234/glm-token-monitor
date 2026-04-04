use crate::events::UsageData;
use crate::polling::fetch_usage;

/// 手动获取当前使用量（前端触发，用于按需刷新）
#[tauri::command]
pub async fn get_current_usage() -> Result<UsageData, String> {
    fetch_usage().await
}
