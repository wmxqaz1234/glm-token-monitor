use crate::events::UsageData;
use crate::polling::fetch_usage;
use crate::database::{self, HistoryEntry, HistoryStats, CumulativeStats, ChartData};
use crate::config::{load_config, save_config, NotificationConfig};
use crate::notifications;
use tauri::AppHandle;

/// 手动获取当前使用量（前端触发，用于按需刷新）
#[tauri::command]
pub async fn get_current_usage(app: AppHandle) -> Result<UsageData, String> {
    fetch_usage(&app).await
}

/// 手动刷新（强制立即查询并记录到数据库）
#[tauri::command]
pub async fn manual_refresh(app: AppHandle) -> Result<UsageData, String> {
    let data = fetch_usage(&app).await?;

    // 记录到数据库
    let log_entry = crate::database::UsageLogEntry {
        tokens_percent: data.tokens_percent,
        tokens_used: data.tokens_usage,
        tokens_total: data.total,
        time_percent: data.time_percent,
        time_remaining: data.time_remaining,
        weekly_tokens_percent: data.weekly_tokens_percent,
        weekly_tokens_used: data.weekly_tokens_usage,
        level: Some(data.level.clone()),
    };

    crate::database::insert_usage_log(&log_entry).map_err(|e| e.to_string())?;

    Ok(data)
}

/// 获取历史数据
/// hours: 查询最近 N 小时的数据，0 表示全部
#[tauri::command]
pub fn get_history(hours: u32) -> Result<Vec<HistoryEntry>, String> {
    database::query_history(hours).map_err(|e| e.to_string())
}

/// 获取历史统计摘要
#[tauri::command]
pub fn get_history_stats() -> Result<HistoryStats, String> {
    database::get_history_stats().map_err(|e| e.to_string())
}

/// 获取累积 Token 统计（用于宠物成长系统）
#[tauri::command]
pub fn get_cumulative_stats() -> Result<CumulativeStats, String> {
    database::get_cumulative_stats().map_err(|e| e.to_string())
}

/// 获取指定日期范围内的累积使用量
/// days: 查询最近 N 天的累积量
#[tauri::command]
pub fn get_cumulative_in_range(days: u32) -> Result<u64, String> {
    database::get_cumulative_in_range(days).map_err(|e| e.to_string())
}

/// 导出历史数据为 CSV
/// hours: 查询最近 N 小时的数据，0 表示全部
#[tauri::command]
pub fn export_history_csv(hours: u32) -> Result<String, String> {
    database::export_to_csv(hours).map_err(|e| e.to_string())
}

/// 清理旧数据
/// days: 保留最近 N 天的数据
#[tauri::command]
pub fn cleanup_history(days: u32) -> Result<String, String> {
    let deleted = database::cleanup_old_data(days).map_err(|e| e.to_string())?;
    Ok(format!("已删除 {} 条旧记录", deleted))
}

/// 获取图表历史数据（按天聚合）
/// days: 查询最近 N 天的数据（7, 30, 90）
#[tauri::command]
pub fn get_history_data(days: i32) -> Result<Vec<ChartData>, String> {
    database::get_history_data(days).map_err(|e| e.to_string())
}

/// 更新通知配置
#[tauri::command]
pub fn update_notification_config(
    app: AppHandle,
    enabled: bool,
    threshold: u32,
    sound_enabled: bool,
    cooldown_minutes: u32,
) -> Result<(), String> {
    let mut config = load_config(&app)?;
    config.notification_config = NotificationConfig {
        enabled,
        threshold,
        sound_enabled,
        cooldown_minutes,
        last_notification_time: config.notification_config.last_notification_time,
    };
    save_config(&app, &config).map_err(|e| e.to_string())
}

/// 发送测试通知
#[tauri::command]
pub fn test_notification(app: AppHandle) -> Result<(), String> {
    notifications::send_notification(
        &app,
        "🔔 测试通知".to_string(),
        "这是一条测试通知，如果您看到这条消息，说明通知功能正常".to_string(),
        true
    )
}
