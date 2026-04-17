/// 桌面通知模块
/// 使用 Tauri 的通知插件发送系统通知

use tauri::{AppHandle, Manager};
use crate::config::{load_config, save_config};

/// 检查并发送预警通知
/// percent: 当前 Token 使用百分比
/// 返回是否发送了通知
pub fn check_and_notify(app: &AppHandle, percent: u32) -> bool {
    // 加载配置
    let mut config = match load_config(app) {
        Ok(cfg) => cfg,
        Err(_) => return false,
    };

    // 检查是否启用通知
    if !config.notification_config.enabled {
        return false;
    }

    // 检查是否超过阈值
    if percent < config.notification_config.threshold {
        return false;
    }

    // 检查冷却时间
    if let Some(last_time) = config.notification_config.last_notification_time {
        let now = chrono::Utc::now().timestamp();
        let cooldown_secs = config.notification_config.cooldown_minutes as i64 * 60;
        if now - last_time < cooldown_secs {
            return false; // 还在冷却期内
        }
    }

    // 发送通知
    let title = format!("⚠️ Token 使用率预警");
    let body = format!("当前使用率已达 {}%，请注意控制使用量", percent);

    // 使用 Tauri 发送通知（通过事件到前端）
    if let Err(e) = app.emit("show-notification", serde_json::json!({
        "title": title,
        "body": body,
        "sound": config.notification_config.sound_enabled
    })) {
        eprintln!("Failed to emit notification event: {}", e);
        return false;
    }

    // 更新最后通知时间
    config.notification_config.last_notification_time = Some(chrono::Utc::now().timestamp());
    let _ = save_config(app, &config);

    true
}

/// 发送自定义通知
pub fn send_notification(app: &AppHandle, title: String, body: String, sound: bool) -> Result<(), String> {
    app.emit("show-notification", serde_json::json!({
        "title": title,
        "body": body,
        "sound": sound
    })).map_err(|e| e.to_string())
}

/// 发送配额耗尽通知
pub fn send_quota_exhausted(app: &AppHandle) -> Result<(), String> {
    send_notification(
        app,
        "🚫 配额已耗尽".to_string(),
        "您的 Token 配额已用完，请等待重置或充值".to_string(),
        true
    )
}

/// 发送配额即将耗尽通知
pub fn send_quota_warning(app: &AppHandle, percent: u32) -> Result<(), String> {
    send_notification(
        app,
        "⚠️ 配额即将耗尽".to_string(),
        format!("Token 使用率已达 {}%，建议减少使用", percent),
        true
    )
}
