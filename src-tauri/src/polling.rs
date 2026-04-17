use crate::events::{ApiResponse, UsageData, EVENT_USAGE_UPDATE};
use crate::config::get_active_model_config;
use crate::database::UsageLogEntry;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter};
use tokio::time::interval;

/// 格式化时间为 API 需要的格式：YYYY-MM-DD HH:MM:SS
fn format_time(secs: i64) -> String {
    if let Some(datetime) = chrono::DateTime::from_timestamp(secs, 0) {
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    } else {
        String::new()
    }
}

/// 请求真实 API 获取用量数据
pub async fn fetch_usage(app: &AppHandle) -> Result<UsageData, String> {
    // 从配置读取 API 参数
    let model_config = get_active_model_config(app)?;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    // 计算时间：当前时间和昨天当前时间
    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Failed to get current time: {}", e))?
        .as_secs() as i64;
    let yesterday_secs = now_secs - 86400; // 24 小时前

    let end_time = format_time(now_secs);
    let start_time = format_time(yesterday_secs);

    let response = client
        .get(&model_config.api_url())
        .query(&[("startTime", start_time), ("endTime", end_time)])
        .header("Authorization", &model_config.api_key)
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    let api_resp: ApiResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if !api_resp.success {
        return Err(format!("API returned error code: {}", api_resp.code));
    }

    let data = api_resp.data.ok_or("API response missing data field")?;

    let mut time_percent: u32 = 0;
    let mut time_remaining: Option<u32> = None;
    let mut time_reset_time: Option<i64> = None;
    let mut tokens_percent: u32 = 0;
    let mut tokens_reset_time: Option<i64> = None;
    let mut tokens_usage: Option<u64> = None;
    let mut weekly_tokens_percent: u32 = 0;
    let mut weekly_tokens_reset_time: Option<i64> = None;
    let mut weekly_tokens_usage: Option<u64> = None;
    let mut usage_details: Vec<crate::events::UsageDetailData> = Vec::new();

    for item in &data.limits {
        let unit = item.unit.unwrap_or(0);
        match item.limit_type.as_str() {
            "TIME_LIMIT" => {
                time_percent = item.percentage.unwrap_or(0);
                time_remaining = item.remaining;
                time_reset_time = item.next_reset_time;
                // 提取工具使用详情
                if let Some(details) = &item.usage_details {
                    for detail in details {
                        usage_details.push(crate::events::UsageDetailData {
                            model_code: detail.model_code.clone(),
                            usage: detail.usage,
                        });
                    }
                }
            }
            "TOKENS_LIMIT" => {
                match unit {
                    6 => {
                        // 周 Token 限制
                        weekly_tokens_percent = item.percentage.unwrap_or(0);
                        weekly_tokens_reset_time = item.next_reset_time;
                        weekly_tokens_usage = item.usage.map(|u| u as u64);
                    }
                    _ => {
                        // 5h Token 限制（unit=3 或其他）
                        tokens_percent = item.percentage.unwrap_or(0);
                        tokens_reset_time = item.next_reset_time;
                        tokens_usage = item.usage.map(|u| u as u64);
                    }
                }
            }
            _ => {}
        }
    }

    Ok(UsageData {
        // 宠物状态由 5h Token 额度（tokens_percent）驱动
        used: tokens_percent,
        total: 100,
        time_percent,
        tokens_percent,
        weekly_tokens_percent,
        time_remaining,
        tokens_reset_time,
        weekly_tokens_reset_time,
        time_reset_time,
        level: data.level,
        usage_details,
        tokens_usage,
        weekly_tokens_usage,
    })
}

/// 启动轮询循环（支持自适应轮询）
pub async fn start_polling_loop(app: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // 启动时立即拉取一次
    let mut current_usage = emit_usage(&app).await;

    // 从配置读取轮询设置
    let config = crate::config::load_config(&app)?;
    let adaptive_enabled = config.polling_config.adaptive_polling;
    let default_interval = config.polling_config.interval_minutes * 60;
    let adaptive_config = config.polling_config.adaptive_config;

    loop {
        // 根据使用率决定下次轮询间隔
        let interval_secs = if adaptive_enabled {
            calculate_adaptive_interval(
                current_usage,
                &adaptive_config,
                default_interval
            )
        } else {
            default_interval
        };

        // 等待指定间隔
        tokio::time::sleep(Duration::from_secs(interval_secs)).await;

        // 拉取新数据
        current_usage = emit_usage(&app).await;
    }
}

/// 计算自适应轮询间隔
fn calculate_adaptive_interval(
    usage_percent: u32,
    config: &crate::config::AdaptivePollingConfig,
    default_interval: u64,
) -> u64 {
    // 配额耗尽且启用暂停时，使用较长间隔
    if usage_percent >= 100 && config.pause_when_exhausted {
        return 30 * 60; // 30 分钟
    }

    // 高使用率：更频繁轮询
    if usage_percent >= config.high_usage_threshold {
        config.high_usage_interval * 60
    }
    // 低使用率：降低频率
    else if usage_percent <= config.low_usage_threshold {
        config.low_usage_interval * 60
    }
    // 中等使用率：使用默认间隔
    else {
        default_interval
    }
}

/// 发送使用量更新，返回使用率百分比
pub async fn emit_usage(app: &AppHandle) -> u32 {
    match fetch_usage(app).await {
        Ok(data) => {
            let usage_percent = data.tokens_percent;

            // 记录到数据库
            let log_entry = UsageLogEntry {
                tokens_percent: data.tokens_percent,
                tokens_used: data.tokens_usage,
                tokens_total: data.total,
                time_percent: data.time_percent,
                time_remaining: data.time_remaining,
                weekly_tokens_percent: data.weekly_tokens_percent,
                weekly_tokens_used: data.weekly_tokens_usage,
                level: Some(data.level.clone()),
            };

            if let Err(e) = crate::database::insert_usage_log(&log_entry) {
                eprintln!("Failed to save usage log: {}", e);
            }

            // 检查并发送预警通知
            crate::notifications::check_and_notify(app, data.tokens_percent);

            // 配额耗尽时发送特殊通知
            if data.tokens_percent >= 100 {
                let _ = crate::notifications::send_quota_exhausted(app);
            }

            // 发送事件到前端
            if let Err(e) = app.emit(EVENT_USAGE_UPDATE, data) {
                eprintln!("Failed to emit usage update: {}", e);
            }

            usage_percent
        }
        Err(e) => {
            eprintln!("Failed to fetch usage: {}", e);
            let _ = app.emit("usage-error", format!("Usage fetch failed: {}", e));
            0 // 返回 0 表示获取失败
        }
    }
}
