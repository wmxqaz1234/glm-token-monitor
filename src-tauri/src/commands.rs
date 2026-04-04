use crate::events::UsageData;
use crate::polling::fetch_usage;

/// 手动获取当前使用量
#[tauri::command]
pub async fn get_current_usage() -> Result<UsageData, String> {
    fetch_usage().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_current_usage() {
        let result = get_current_usage().await;
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.used, 65);
    }
}
